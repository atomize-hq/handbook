"""Hosted current-arm driver; journal reservations before starting a runner."""
import csv
import ctypes
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('bench', HERE / 'packet/bench.py')
bench = importlib.util.module_from_spec(spec)
spec.loader.exec_module(bench)

def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()

def verify_source():
    expected = json.loads((HERE / 'source-sha256.json').read_text())
    actual = {p.relative_to(HERE / 'source').as_posix(): sha(p) for p in (HERE / 'source').rglob('*') if p.is_file()}
    if actual != expected:
        raise RuntimeError('source_inventory_mismatch')
    for name, digest in [('bench.py', 'e8e3a833e23085edecf520789b1fe0af5fef7a5db23b6c4354ac5217260be857'), ('s06_work_compare.rs', '98c3864d5360214c0e0284736212e8aae9f79df8eb5b2e7ea3a96557388bcc23')]:
        if sha(HERE / 'packet' / name) != digest:
            raise RuntimeError('retained_runner_mismatch')

def identity():
    if os.name == 'nt':
        if ctypes.windll.shell32.IsUserAnAdmin():
            raise RuntimeError('administrative_token')
        ctypes.windll.kernel32.GetCurrentProcess.restype = ctypes.c_void_p
        ctypes.windll.advapi32.OpenProcessToken.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.POINTER(ctypes.c_void_p)]
        ctypes.windll.advapi32.GetTokenInformation.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_void_p, ctypes.c_uint32, ctypes.POINTER(ctypes.c_uint32)]
        ctypes.windll.kernel32.CloseHandle.argtypes = [ctypes.c_void_p]
        token = ctypes.c_void_p()
        if not ctypes.windll.advapi32.OpenProcessToken(ctypes.windll.kernel32.GetCurrentProcess(), 8, ctypes.byref(token)):
            raise ctypes.WinError()
        try:
            elevation, length = ctypes.c_uint32(), ctypes.c_uint32()
            if not ctypes.windll.advapi32.GetTokenInformation(token, 20, ctypes.byref(elevation), ctypes.sizeof(elevation), ctypes.byref(length)):
                raise ctypes.WinError()
            if elevation.value:
                raise RuntimeError('elevated_token')
        finally:
            ctypes.windll.kernel32.CloseHandle(token)
        name, length = ctypes.create_unicode_buffer(256), ctypes.c_uint32(256)
        if not ctypes.windll.advapi32.GetUserNameW(name, ctypes.byref(length)):
            raise ctypes.WinError()
        if name.value.casefold() != 'fnbench':
            raise RuntimeError('unexpected_account')
        return {'account': 'fnbench', 'effective_admin': False, 'elevated': False}
    if os.geteuid() == 0:
        raise RuntimeError('root_workload')
    status = {}
    for line in Path('/proc/self/status').read_text().splitlines():
        if line.startswith(('CapEff:', 'NoNewPrivs:', 'Seccomp:')):
            key, value = line.split(':', 1); status[key] = value.strip()
    return {'effective_root': False, 'security': status}

def windows_context(root, output, executable):
    if os.name != 'nt':
        return {'platform': 'linux'}
    tasklist = shutil.which('tasklist.exe')
    if not tasklist:
        raise RuntimeError('non_c_runtime_path')
    paths = {'workdir': Path.cwd(), 'source': HERE / 'source', 'runner': executable,
             'fixture_root': root, 'evidence': output, 'python': Path(sys.executable),
             'python_base': Path(sys.base_prefix), 'temp': Path(os.environ['TEMP']),
             'tmp': Path(os.environ['TMP']), 'system_root': Path(os.environ['SystemRoot']),
             'tasklist': Path(tasklist)}
    for key in ('HOME', 'USERPROFILE', 'APPDATA', 'LOCALAPPDATA', 'TMPDIR', 'CARGO_HOME', 'RUSTUP_HOME', 'HANDBOOK_PACKET2_VALIDATOR_EXE'):
        if os.environ.get(key):
            paths['env_' + key.lower()] = Path(os.environ[key])
    drives = {key: path.resolve().drive.upper() for key, path in paths.items()}
    search = [p for p in os.environ.get('PATH', '').split(os.pathsep) if p]
    python_search = [p for p in sys.path if p]
    if any(value != 'C:' for value in drives.values()) or any(Path(p).resolve().drive.upper() != 'C:' for p in search + python_search):
        raise RuntimeError('non_c_runtime_path')
    # Inspect the loaded Python process modules; no raw module paths leave the VM.
    psapi, kernel = ctypes.windll.psapi, ctypes.windll.kernel32
    kernel.GetCurrentProcess.restype = ctypes.c_void_p
    psapi.EnumProcessModules.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_void_p), ctypes.c_uint32, ctypes.POINTER(ctypes.c_uint32)]
    psapi.GetModuleFileNameExW.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_wchar_p, ctypes.c_uint32]
    modules = (ctypes.c_void_p * 1024)(); needed = ctypes.c_uint32()
    process = kernel.GetCurrentProcess()
    if not psapi.EnumProcessModules(process, modules, ctypes.sizeof(modules), ctypes.byref(needed)) or needed.value > ctypes.sizeof(modules):
        raise RuntimeError('non_c_runtime_path')
    count = needed.value // ctypes.sizeof(ctypes.c_void_p)
    for index in range(count):
        name = ctypes.create_unicode_buffer(32768)
        if not psapi.GetModuleFileNameExW(process, modules[index], name, len(name)) or Path(name.value).resolve().drive.upper() != 'C:':
            raise RuntimeError('non_c_runtime_path')
    return {'platform': 'windows', 'path_drives': drives, 'python_module_count': count, 'python_module_drives': ['C:'], 'search_path_drives': ['C:']}

def event(output, kind, **fields):
    record = {'event': kind, **fields}
    with (output / 'events.jsonl').open('a', encoding='utf-8') as f:
        f.write(json.dumps(record, separators=(',', ':')) + '\n'); f.flush(); os.fsync(f.fileno())

def safe_error(exc):
    flags = {'source_inventory_mismatch', 'retained_runner_mismatch', 'administrative_token', 'elevated_token', 'unexpected_account', 'root_workload', 'non_c_runtime_path', 'runner_changed', 'invalid_slots', 'fresh_root_required'}
    message = str(exc)
    return {'type': type(exc).__name__, 'reason': message if message in flags else 'other',
            'errno': getattr(exc, 'errno', None) if isinstance(getattr(exc, 'errno', None), int) else None,
            'winerror': getattr(exc, 'winerror', None) if isinstance(getattr(exc, 'winerror', None), int) else None}

def trace_summary(trace):
    result = {'stages': [], 'trace_sha256': None, 'returned': None, 'maintenance': []}
    if not trace.is_file():
        return result
    result['trace_sha256'] = sha(trace)
    allowed = {'allocated', 'grant_intent', 'grant_returned', 'prepared', 'start_intent', 'returned', 'maintenance', 'reopened'}
    with trace.open(encoding='utf-8', errors='replace') as f:
        for line in f:
            try:
                row = json.loads(line); stage = row.get('stage')
            except (ValueError, AttributeError):
                result['stages'].append('unparseable'); continue
            result['stages'].append(stage if isinstance(stage, str) and stage in allowed else 'other')
            if stage == 'returned':
                elapsed, execution = row.get('sdk_elapsed_ns'), row.get('execution')
                disposition = execution.get('disposition') if isinstance(execution, dict) else None
                allowed_dispositions = {'committed', 'replayed', 'refused', 'non_committed', 'uncertain', 'pending', 'unaccepted'}
                result['returned'] = {'sdk_elapsed_ns': elapsed if isinstance(elapsed, str) and elapsed.isdigit() and len(elapsed) <= 30 else None,
                                      'disposition': disposition if isinstance(disposition, str) and disposition in allowed_dispositions else 'unrecognized'}
            if stage == 'maintenance':
                progress = row.get('progress'); call = row.get('call')
                result['maintenance'].append({'call': call if isinstance(call, int) and 1 <= call <= 64 else None,
                                              'complete': progress.get('complete') if isinstance(progress, dict) and isinstance(progress.get('complete'), bool) else None})
    result['stages'] = result['stages'][:80]; result['maintenance'] = result['maintenance'][:64]
    return result

def main():
    root = Path(sys.argv[1]).resolve(); output = Path(sys.argv[2]).resolve()
    # The launcher creates this directory before invoking us, so even early failures can be reported.
    output.mkdir(parents=True, exist_ok=True)
    event(output, 'driver_entered')
    try:
        event(output, 'preflight_started')
        slots = json.loads(os.environ['FN_SLOTS'])
        run_kind = os.environ['FN_RUN_KIND']
        if run_kind not in ('initial', 'recovery') or not isinstance(slots, list) or not 1 <= len(slots) <= 6 or any(type(n) is not int or n < 1 for n in slots) or slots != sorted(set(slots)) or (run_kind == 'initial' and any(n > 6 for n in slots)) or (run_kind == 'recovery' and any(n <= 6 for n in slots)):
            raise RuntimeError('invalid_slots')
        verify_source(); token = identity()
        if root.exists():
            raise RuntimeError('fresh_root_required')
        binary_name = 's06_work_compare.exe' if os.name == 'nt' else 's06_work_compare'
        built = HERE / 'target/release/examples' / binary_name
        context = windows_context(root, output, built)
        root.mkdir(parents=True); runtime = root / 'runtime'; runtime.mkdir()
        executable = runtime / ('current-' + binary_name); shutil.copy2(built, executable)
        context = windows_context(root, output, executable)
        bench.RUNTIME = runtime
        summary = {'token': token, 'context': context, 'slots': slots, 'run_kind': run_kind, 'source_manifest_sha256': sha(HERE / 'source-sha256.json'), 'runner_sha256': sha(executable), 'attempts': []}
        event(output, 'preflight_passed', token=token, context=context, slots=slots, run_kind=run_kind, runner_sha256=summary['runner_sha256'], source_manifest_sha256=summary['source_manifest_sha256'])
    except Exception as exc:
        event(output, 'preflight_failed', error=safe_error(exc))
        raise SystemExit('preflight failed; no runner reserved')
    active_slot = None; samples = []
    def observe():
        names = {'cargo', 'rustc', 'cc', 'gcc', 'clang', 'cl', 'link', 'lld', 'ld', 'rust-lld'}
        if os.name == 'nt':
            out = subprocess.check_output(['tasklist.exe', '/FO', 'CSV', '/NH'], text=True, errors='replace', timeout=5)
            seen = [row[0] for row in csv.reader(out.splitlines()) if row]
        else:
            out = subprocess.check_output(['ps', '-axo', 'comm='], text=True, timeout=5)
            seen = [Path(line.strip()).name for line in out.splitlines()]
        return [name for name in seen if name.lower().removesuffix('.exe') in names]
    def observing():
        trace = active_slot / 'proof/trace.jsonl'; stage = 'before_start_intent'
        try:
            data = trace.read_bytes()
            if b'"stage":"returned"' in data: stage = 'after_returned_visible'
            elif b'"stage":"start_intent"' in data: stage = 'between_visible_markers'
        except OSError:
            pass
        found = observe(); samples.append({'monotonic_ns': time.monotonic_ns(), 'stage': stage, 'builds': found}); return found
    bench.observed_builds = observing
    for n in slots:
        active_slot = root / f'attempt-{n:02d}'; samples.clear()
        # A durable reservation is conservatively consumed even if execution is interrupted immediately afterward.
        event(output, 'attempt_reserved', slot=n)
        try:
            if sha(executable) != summary['runner_sha256']:
                raise RuntimeError('runner_changed')
            result = bench.run_arm(active_slot, 'current')
            if sha(executable) != summary['runner_sha256']:
                raise RuntimeError('runner_changed')
            item = {'slot': n, 'result': result, 'trace': trace_summary(active_slot / 'proof/trace.jsonl'), 'activity': list(samples)}
            summary['attempts'].append(item); event(output, 'attempt_completed', **item)
        except Exception as exc:
            stderr = active_slot / 'stderr.log'; text = stderr.read_text(errors='replace').casefold() if stderr.is_file() else ''
            flags = [f for f in ('panicked', 'permission denied', 'access is denied', 'not found', 'no such file', 'noncommitted', 'maintenance cap reached', 'mismatch') if f in text]
            match = re.search(r'exited (-?\d+)', str(exc))
            item = {'slot': n, 'error': safe_error(exc), 'process_exit': int(match.group(1)) if match else None,
                    'stderr_sha256': sha(stderr) if stderr.is_file() else None, 'stderr_flags': flags,
                    'trace': trace_summary(active_slot / 'proof/trace.jsonl'), 'activity': list(samples)}
            summary['attempts'].append(item); event(output, 'attempt_failed', **item)
            (output / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
            raise SystemExit('attempt failed; stopped without replacement')
        (output / 'summary.json').write_text(json.dumps(summary, indent=2) + '\n')
    verify_source(); event(output, 'driver_finished')

if __name__ == '__main__':
    main()
