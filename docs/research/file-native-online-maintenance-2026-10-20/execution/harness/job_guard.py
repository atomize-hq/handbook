"""Windows stage Job, shared heavy token, fixed clock and admitted-root watchdog.

Only this stage's Job is terminated. Imported legacy guards are never invoked.
Receipts are experiment-owned JSON; they do not confer deletion authority.
"""
import argparse
import contextlib
import ctypes
from ctypes import wintypes as w
import datetime as dt
import hashlib
import gzip
import json
import msvcrt
import os
from pathlib import Path
import shutil
import subprocess
import sys
import threading
import time
import uuid

FUNCTIONAL = dt.datetime.fromisoformat('2026-10-21T11:10:14+00:00').timestamp()
TOKEN = Path('C:/file-native-online-control-20261020/windows-heavy.lock')
REGISTRY = TOKEN.with_name('supervisors.json')
OWNER = TOKEN.with_name('current-owner.json')
REGISTRY_LOCK = TOKEN.with_name('supervisors.lock')
GiB = 1024**3
K = ctypes.WinDLL('kernel32', use_last_error=True)
PS = ctypes.WinDLL('psapi', use_last_error=True)


class Basic(ctypes.Structure):
    _fields_ = [('User', ctypes.c_int64), ('Kernel', ctypes.c_int64),
                ('Flags', w.DWORD), ('Min', ctypes.c_size_t), ('Max', ctypes.c_size_t),
                ('Active', w.DWORD), ('Affinity', ctypes.c_size_t),
                ('Priority', w.DWORD), ('Scheduling', w.DWORD)]


class IO(ctypes.Structure):
    _fields_ = [(x, ctypes.c_uint64) for x in ['ro', 'wo', 'oo', 'rb', 'wb', 'ob']]


class Limits(ctypes.Structure):
    _fields_ = [('Basic', Basic), ('IO', IO), ('Process', ctypes.c_size_t),
                ('Job', ctypes.c_size_t), ('PeakProcess', ctypes.c_size_t),
                ('PeakJob', ctypes.c_size_t)]


class Memory(ctypes.Structure):
    _fields_ = [('cb', w.DWORD), ('faults', w.DWORD)] + [
        (x, ctypes.c_size_t) for x in
        ['peak', 'rss', 'peakpage', 'page', 'peaknon', 'non', 'pagefile', 'peakfile']]


K.CreateJobObjectW.restype = w.HANDLE
K.CreateJobObjectW.argtypes = [ctypes.c_void_p, w.LPCWSTR]
K.SetInformationJobObject.argtypes = [w.HANDLE, ctypes.c_int, ctypes.c_void_p, w.DWORD]
K.AssignProcessToJobObject.argtypes = [w.HANDLE, w.HANDLE]
K.TerminateJobObject.argtypes = [w.HANDLE, w.UINT]
K.QueryInformationJobObject.argtypes = [w.HANDLE, ctypes.c_int, ctypes.c_void_p,
                                       w.DWORD, ctypes.c_void_p]
K.CloseHandle.argtypes = [w.HANDLE]
K.OpenProcess.restype = w.HANDLE
K.OpenProcess.argtypes = [w.DWORD, w.BOOL, w.DWORD]
K.GetProcessTimes.argtypes = [w.HANDLE, ctypes.POINTER(w.FILETIME), ctypes.POINTER(w.FILETIME),
                             ctypes.POINTER(w.FILETIME), ctypes.POINTER(w.FILETIME)]
PS.GetProcessMemoryInfo.argtypes = [w.HANDLE, ctypes.POINTER(Memory), w.DWORD]
NT = ctypes.WinDLL('ntdll')
NT.NtResumeProcess.argtypes = [w.HANDLE]


def sha_file(path):
    digest = hashlib.sha256()
    with Path(path).open('rb') as source:
        for block in iter(lambda: source.read(1024 * 1024), b''):
            digest.update(block)
    return digest.hexdigest()


def write_json(path, row):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open('x', encoding='utf-8') as output:
        json.dump(row, output, indent=2)
        output.write('\n')


def census(root, stop=None):
    root = Path(root)
    count = total = 0
    if not root.exists():
        return dict(bytes=0, files=0)
    if root.is_symlink():
        raise ValueError('root-symlink')
    for directory, dirs, files in os.walk(root, followlinks=False):
        for name in dirs + files:
            path = Path(directory) / name
            if path.is_symlink() or (hasattr(path, 'is_junction') and path.is_junction()):
                raise ValueError('census-link:' + str(path))
        for name in files:
            if stop is not None and stop.is_set():
                raise InterruptedError('census-stopped')
            try:
                total += (Path(directory) / name).stat().st_size
                count += 1
            except FileNotFoundError:
                pass  # Concurrent successful removal receives no prospective credit.
    return dict(bytes=total, files=count)


class Budget:
    def __init__(self, path):
        self.path = Path(path)
        self.raw_hash = sha_file(path)
        self.config = json.loads(self.path.read_bytes())
        assert not self.config.get('template_not_execution_admission'), 'unfilled-budget-template'
        assert self.config['admission_ledger_sha256'] == '2ff4e277a44c3dea99f53d5b098250e78a50c40d63539022bf365f4cc048c82c'
        self.categories = self.config['categories']
        assert all(Path(v['root']).is_absolute() for v in self.categories.values())
        roots = {name: Path(v['root']).resolve() for name, v in self.categories.items()}
        for name, path in roots.items():
            for other, parent in roots.items():
                if name != other and path.is_relative_to(parent):
                    assert self.categories[name].get('nonadditive_subset_of') == other, 'undeclared-overlapping-roots'
        for name, rule in self.categories.items():
            assert rule.get('charge_mode', 'absolute') in ['absolute', 'positive_delta']
            if rule.get('charge_mode') == 'positive_delta':
                assert type(rule['baseline_bytes']) is int and rule['baseline_bytes'] >= 0
                assert type(rule['baseline_files']) is int and rule['baseline_files'] >= 0

    def check(self, reserves=None, stop=None):
        reserves = reserves or {}
        assert set(reserves) <= set(self.categories)
        rows = {}
        for name, rule in self.categories.items():
            actual = census(rule['root'], stop)
            reserve = reserves.get(name, {})
            b, f = reserve.get('bytes', 0), reserve.get('files', 0)
            assert type(b) is int and b >= 0 and type(f) is int and f >= 0
            charged = dict(actual)
            if rule.get('charge_mode') == 'positive_delta':
                charged = dict(bytes=max(0, actual['bytes']-rule['baseline_bytes']),
                               files=max(0, actual['files']-rule['baseline_files']))
            rows[name] = dict(**charged, absolute_actual=actual, reserve_bytes=b, reserve_files=f,
                              charge_mode=rule.get('charge_mode', 'absolute'),
                              additive_charge=not bool(rule.get('nonadditive_subset_of')),
                              cap_bytes=rule['bytes'], cap_files=rule['files'])
            if charged['bytes'] + b > rule['bytes'] or charged['files'] + f > rule['files']:
                raise ValueError('prospective-category-cap:' + name + ':' + json.dumps(rows[name]))
        group_rows = {}
        for name, group in self.config.get('groups', {}).items():
            assert set(group['categories']) <= set(rows)
            assert all(rows[k]['additive_charge'] for k in group['categories']), 'subset-counted-twice'
            total_b = sum(rows[k]['bytes']+rows[k]['reserve_bytes'] for k in group['categories'])
            total_f = sum(rows[k]['files']+rows[k]['reserve_files'] for k in group['categories'])
            group_rows[name] = dict(bytes=total_b, files=total_f, cap_bytes=group['bytes'], cap_files=group['files'])
            if total_b > group['bytes'] or total_f > group['files']:
                raise ValueError('prospective-shared-group-cap:'+name+':'+json.dumps(group_rows[name]))
        free = shutil.disk_usage('C:/').free
        if free < 140 * GiB:
            raise ValueError('windows-free-floor:' + str(free))
        return dict(categories=rows, shared_groups=group_rows, free_bytes=free, epoch=time.time(),
                    qualification='logical bytes; no planned deletion credit; parent owns cross-host ledger')


def rss(pid):
    handle = K.OpenProcess(0x410, False, pid)
    if not handle:
        return None
    try:
        memory = Memory()
        memory.cb = ctypes.sizeof(memory)
        return (memory.rss, memory.peak) if PS.GetProcessMemoryInfo(
            handle, ctypes.byref(memory), memory.cb) else None
    finally:
        K.CloseHandle(handle)


def creation_time(pid):
    handle = K.OpenProcess(0x1000, False, pid)
    if not handle:
        error = ctypes.get_last_error()
        if error in [87, 1168]:
            return None
        raise ctypes.WinError(error)
    try:
        values = [w.FILETIME() for _ in range(4)]
        if not K.GetProcessTimes(handle, *(ctypes.byref(v) for v in values)):
            raise ctypes.WinError(ctypes.get_last_error())
        return values[0].dwHighDateTime * 2**32 + values[0].dwLowDateTime
    finally:
        K.CloseHandle(handle)


def control_json(path, row):
    """Token holder updates bounded advisory records; token itself never moves."""
    data = (json.dumps(row, separators=(',', ':'))+'\n').encode()
    assert len(data) <= 8192, 'control-record-cap'
    temp = path.with_suffix('.tmp')
    with temp.open('wb') as output:
        output.write(data)
        output.flush()
        os.fsync(output.fileno())
    for attempt in range(20):
        try:
            os.replace(temp, path)
            break
        except PermissionError:
            if attempt == 19:
                raise
            time.sleep(.01)


@contextlib.contextmanager
def registry_lock():
    try:
        with REGISTRY_LOCK.open('xb') as initial:
            initial.write(b'1')
            initial.flush()
            os.fsync(initial.fileno())
    except FileExistsError:
        pass
    handle = REGISTRY_LOCK.open('r+b')
    locked = False
    try:
        while time.time() < FUNCTIONAL:
            if os.fstat(handle.fileno()).st_size != 1:
                time.sleep(.01)
                continue
            try:
                handle.seek(0)
                msvcrt.locking(handle.fileno(), msvcrt.LK_NBLCK, 1)
                locked = True
                break
            except OSError:
                time.sleep(.01)
        assert locked, 'registry-lock-cutoff'
        yield
    finally:
        if locked:
            handle.seek(0)
            msvcrt.locking(handle.fileno(), msvcrt.LK_UNLCK, 1)
        handle.close()


def registry_rows():
    if not REGISTRY.exists():
        return []
    with REGISTRY.open('rb') as source:
        raw = source.read(8193)
    assert len(raw) <= 8192, 'supervisor-registry-byte-cap'
    rows = json.loads(raw)
    assert isinstance(rows, list) and len(rows) <= 16, 'supervisor-registry-cap'
    for row in rows:
        assert set(row) == {'pid', 'creation_time', 'registered_by', 'role'}
    return rows


def supervisors(extra_pids=()):
    with registry_lock():
        live = {}
        for row in registry_rows():
            actual = creation_time(row['pid'])
            if actual is not None and actual == row['creation_time']:
                live[row['pid']] = row
        for pid in {os.getpid(), os.getppid(), *extra_pids}:
            started = creation_time(pid)
            assert started is not None, 'supervisor-owner-vanished'
            live[pid] = dict(pid=pid, creation_time=started, registered_by=os.getpid(),
                             role='guard-or-explicit-new-work-supervisor')
        assert len(live) <= 16, 'supervisor-registry-cap'
        control_json(REGISTRY, list(live.values()))
        return list(live.values())


def supervisor_rss(rows=None):
    total = 0
    samples = []
    for row in registry_rows() if rows is None else rows:
        started = creation_time(row['pid'])
        if started is None or started != row['creation_time']:
            samples.append(dict(pid=row['pid'], state='ended-or-reused'))
            continue
        value = rss(row['pid'])
        if value is None:
            # A second identity query distinguishes an exit race from invisibility.
            after = creation_time(row['pid'])
            if after is None or after != started:
                samples.append(dict(pid=row['pid'], state='exited-during-rss'))
                continue
            raise RuntimeError('unreadable-live-supervisor-rss:'+str(row['pid']))
        total += value[0]
        samples.append(dict(pid=row['pid'], creation_time=started, rss=value[0], peak=value[1]))
    if total > 768 * 1024**2:
        raise RuntimeError('combined-supervisor-rss-censor:'+str(total))
    return total, samples


def members(job):
    # Fixed bounded table. Overflow fails closed rather than omitting descendants.
    buffer = ctypes.create_string_buffer(8 + ctypes.sizeof(ctypes.c_size_t) * 2048)
    if not K.QueryInformationJobObject(job, 3, buffer, len(buffer), None):
        raise ctypes.WinError(ctypes.get_last_error())
    assigned = int.from_bytes(buffer.raw[:4], 'little')
    count = int.from_bytes(buffer.raw[4:8], 'little')
    if assigned > 2048 or count > 2048:
        raise ValueError('job-member-table-overflow')
    size = ctypes.sizeof(ctypes.c_size_t)
    return [int.from_bytes(buffer.raw[8 + i * size:8 + (i + 1) * size], 'little')
            for i in range(count)]


def run(argv, receipt, budget_path, seconds, reserves=None, stdin=None, cwd=None):
    """Execute one heavy stage; its engine children inherit this single Job."""
    begin = time.time()
    receipt = Path(receipt)
    assert not receipt.exists()
    receipt.parent.mkdir(parents=True, exist_ok=True)
    budget = Budget(budget_path)
    preflight = budget.check(reserves)
    supervisors(budget.config.get('supervisor_pids', []))
    supervisor_rss()  # Register and sample waiting guards before the heavy token.
    token = TOKEN.open('r+b')  # Parent establishes this path; never replace it.
    assert os.fstat(token.fileno()).st_size >= 1, 'uninitialized-heavy-token'
    locked = False
    token_wait_begin = time.time()
    token_contentions = 0
    while time.time() < FUNCTIONAL:
        try:
            token.seek(0)
            msvcrt.locking(token.fileno(), msvcrt.LK_NBLCK, 1)
            locked = True
            break
        except OSError:
            token_contentions += 1
            supervisor_rss()
            time.sleep(.1)
    if not locked:
        token.close()
        row = dict(argv=argv, censor='functional-cutoff-waiting-token',
                   token_wait_seconds=time.time() - token_wait_begin, no_child_started=True)
        write_json(receipt, row)
        return row
    acquired = time.time()
    absolute_deadline = min(acquired + float(seconds), FUNCTIONAL)
    job = None
    process = None
    stdout = stderr = None
    timer = sampler = disk_thread = feeder = None
    stop = threading.Event()
    reason = []
    errors = []
    peaks = {}
    aggregate = [0]
    missing = [0]
    exit_races = []
    supervisor_peak = [0]
    supervisor_samples = []
    samples = []
    terminations = []
    lock = threading.Lock()
    remaining = None
    owner = None
    initial_rss = None
    closure_checks = []
    memory_timing = dict(samples=0, max_interval_seconds=0.0, intervals_over_40ms=0)
    rss_stream = None
    def current_members():
        with lock:
            return members(job) if job is not None else []
    def terminate(why):
        with lock:
            if not reason and why != 'root-completed-descendant-cleanup':
                reason.append(why)
            if job:
                terminations.append(dict(epoch=time.time(), reason=why))
                K.TerminateJobObject(job, 137)
    try:
        if OWNER.exists():
            previous = json.loads(OWNER.read_bytes())
            assert isinstance(previous.get('pid'), int) and previous.get('nonce'), 'unknown-owner-record'
            assert previous.get('state') != 'closure-unknown', 'previous-closure-unknown-requires-reconciliation'
            if previous.get('state') != 'closed':
                old_started = creation_time(previous['pid'])
                assert old_started is None or old_started != previous['creation_time'], 'live-owner-without-token'
        supervisor_rows = supervisors(budget.config.get('supervisor_pids', []))
        super_total, super_rows = supervisor_rss(supervisor_rows)
        supervisor_peak[0] = super_total
        supervisor_samples.append(dict(epoch=time.time(), total=super_total, processes=super_rows))
        owner = dict(pid=os.getpid(), creation_time=creation_time(os.getpid()),
                     nonce=uuid.uuid4().hex, acquired_epoch=acquired, job_root_pid=None,
                     categories=list(budget.categories), budget_sha256=budget.raw_hash,
                     receipt=str(receipt.resolve()), state='preparing',
                     authority='advisory ownership only; OS byte lock controls exclusion')
        control_json(OWNER, owner)
        budget.check(reserves)  # Token wait cannot make the old census sufficient.
        effective = absolute_deadline - time.time()
        assert effective > 0, 'functional-cutoff'
        job = K.CreateJobObjectW(None, None)
        if not job:
            raise ctypes.WinError(ctypes.get_last_error())
        limits = Limits()
        limits.Basic.Flags = 0x2000 | 0x100 | 0x200  # kill-on-close/process+Job commit
        limits.Process = 2 * GiB
        limits.Job = 3 * GiB
        if not K.SetInformationJobObject(job, 9, ctypes.byref(limits), ctypes.sizeof(limits)):
            raise ctypes.WinError(ctypes.get_last_error())
        stdout = receipt.with_suffix('.stdout').open('xb')
        stderr = receipt.with_suffix('.stderr').open('xb')
        env = dict(os.environ, PYTHONDONTWRITEBYTECODE='1', FILE_NATIVE_STAGE_JOB='1',
                   FILE_NATIVE_BUDGET=str(Path(budget_path).resolve()))
        process = subprocess.Popen(argv, cwd=cwd, env=env, close_fds=True,
                                   stdin=subprocess.PIPE if stdin is not None else subprocess.DEVNULL,
                                   stdout=stdout, stderr=stderr, creationflags=0x4)
        if not K.AssignProcessToJobObject(job, w.HANDLE(process._handle)):
            process.kill()
            process.wait(timeout=5)
            raise ctypes.WinError(ctypes.get_last_error())
        assert process.pid in current_members(), 'suspended-root-not-in-job'
        initial_rss = rss(process.pid)
        assert initial_rss is not None and initial_rss[0] > 0, 'suspended-root-rss-unreadable'
        peaks[str(process.pid)] = initial_rss[1]
        owner.update(job_root_pid=process.pid, job_root_creation_time=creation_time(process.pid),
                     state='assigned-suspended', initial_root_rss=initial_rss[0])
        control_json(OWNER, owner)
        # Arm before resume, stdin or any further disk work.
        timer = threading.Timer(max(0, absolute_deadline-time.time()), terminate, args=('time-censor',))
        timer.daemon = True
        timer.start()
        rss_stream = gzip.open(receipt.with_suffix('.rss.jsonl.gz'), 'xb', compresslevel=1)
        def memory_watch():
            last_tick = None
            next_tick = time.monotonic()
            while not stop.is_set():
                try:
                    tick = time.monotonic()
                    interval = tick-last_tick if last_tick is not None else None
                    if interval is not None:
                        memory_timing['max_interval_seconds'] = max(memory_timing['max_interval_seconds'], interval)
                        if interval > .04:
                            memory_timing['intervals_over_40ms'] += 1
                    last_tick = tick
                    memory_timing['samples'] += 1
                    pids = current_members()
                    current = 0
                    process_samples = []
                    for pid in pids:
                        value = rss(pid)
                        if value is None:
                            missing[0] += 1
                            if pid not in current_members():
                                exit_races.append(dict(pid=pid, epoch=time.time(), state='departed-before-rss'))
                                continue
                            raise RuntimeError('unreadable-owned-pid-rss:'+str(pid))
                        peaks[str(pid)] = max(peaks.get(str(pid), 0), value[1])
                        process_samples.append(dict(pid=pid, rss=value[0], peak=value[1]))
                        current += value[0]
                        if value[0] > 2 * GiB:
                            terminate('process-rss-censor')
                            return
                    aggregate[0] = max(aggregate[0], current)
                    sup_total, sup_rows = supervisor_rss()
                    supervisor_peak[0] = max(supervisor_peak[0], sup_total)
                    rss_stream.write((json.dumps(dict(epoch=time.time(), monotonic=tick,
                        interval_seconds=interval, job_rss=current, processes=process_samples,
                        supervisor_rss=sup_total, supervisors=sup_rows), separators=(',', ':'))+'\n').encode())
                    if not supervisor_samples or time.time()-supervisor_samples[-1]['epoch'] >= 1:
                        supervisor_samples.append(dict(epoch=time.time(), total=sup_total, processes=sup_rows))
                    if current > 3 * GiB:
                        terminate('stage-aggregate-rss-censor')
                        return
                except BaseException as error:
                    if not stop.is_set():
                        errors.append(repr(error))
                        terminate('memory-monitor-error')
                    return
                next_tick = max(next_tick+.02, time.monotonic())
                stop.wait(max(0, next_tick-time.monotonic()))
        def disk_watch():
            while not stop.is_set():
                try:
                    observation = budget.check(stop=stop)
                    if not stop.is_set():
                        samples.append(observation)
                except BaseException as error:
                    if not stop.is_set():
                        errors.append(repr(error))
                        terminate('disk-or-free-censor')
                    return
                stop.wait(5)
        sampler = threading.Thread(target=memory_watch, daemon=True)
        disk_thread = threading.Thread(target=disk_watch, daemon=True)
        sampler.start()
        disk_thread.start()
        if time.time() >= absolute_deadline or NT.NtResumeProcess(w.HANDLE(process._handle)) != 0:
            raise RuntimeError('resume-failed')
        owner.update(state='running', resumed_epoch=time.time())
        control_json(OWNER, owner)
        if stdin is not None:
            def feed():
                try:
                    process.stdin.write(stdin)
                    process.stdin.close()
                except (OSError, ValueError):
                    pass
            feeder = threading.Thread(target=feed, daemon=True)
            feeder.start()
        process.wait(timeout=max(.001, absolute_deadline-time.time()) + 10)
        if current_members():
            terminate('root-completed-descendant-cleanup')
        until = time.monotonic() + 5
        while current_members() and time.monotonic() < until:
            time.sleep(.01)
        remaining = current_members()
    except BaseException as error:
        errors.append(repr(error))
        terminate('stage-deadline-before-launch' if process is None and time.time() >= absolute_deadline else 'guard-error')
        if process:
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                errors.append('termination-wait-failed')
    finally:
        if timer:
            timer.cancel()
            timer.join(timeout=1)
        stop.set()
        for thread in [sampler, disk_thread, feeder]:
            if thread:
                thread.join(timeout=5)
                if thread.is_alive():
                    errors.append('unjoined-monitor:'+thread.name)
        if job:
            try:
                remaining = current_members()
                if remaining:
                    terminate('guard-finally')
                    until = time.monotonic() + 5
                    while current_members() and time.monotonic() < until:
                        time.sleep(.01)
                    remaining = current_members()
                for _ in range(2):
                    remaining = current_members()
                    closure_checks.append(dict(epoch=time.time(), members=remaining))
                    time.sleep(.02)
            except BaseException as error:
                errors.append(repr(error))
                remaining = None
            with lock:
                closing_job = job
                job = None  # All callbacks see an inert handle before native close.
                K.CloseHandle(closing_job)
        elif process is None:
            # A preflight deadline can expire before CreateJobObject/Popen.
            # There is no native group to drain, and no ambiguous process lifetime.
            closure_checks = [dict(epoch=time.time(), members=[], proof='no-Job-or-child-created')
                              for _ in range(2)]
            remaining = []
        if rss_stream and (sampler is None or not sampler.is_alive()):
            rss_stream.close()
        if stdout:
            stdout.close()
        if stderr:
            stderr.close()
        if owner:
            proven_empty = len(closure_checks) == 2 and all(x['members'] == [] for x in closure_checks)
            owner.update(state='closed' if proven_empty else 'closure-unknown',
                         closed_epoch=time.time(), remaining_job_pids=remaining,
                         errors=errors)
        try:
            if owner:
                control_json(OWNER, owner)
        finally:
            token.seek(0)
            msvcrt.locking(token.fileno(), msvcrt.LK_UNLCK, 1)
            token.close()
    row = dict(argv=argv, cwd=str(cwd) if cwd else None, started_epoch=begin,
               token_acquired_epoch=acquired, token_wait_seconds=acquired-token_wait_begin,
               token_wait_started_epoch=token_wait_begin, token_contentions=token_contentions,
               before_token_setup_seconds=token_wait_begin-begin,
               total_seconds=time.time()-begin, stage_seconds=time.time()-acquired,
               requested_seconds=seconds, functional_cutoff_epoch=FUNCTIONAL,
               absolute_deadline_epoch=absolute_deadline, memory_sample_timing=memory_timing,
               raw_rss=str(receipt.with_suffix('.rss.jsonl.gz')), closure_checks=closure_checks,
               exit=process.returncode if process else None,
               censor=reason[0] if reason else None, errors=errors,
               job_commit_limit=3*GiB, process_commit_limit=2*GiB,
               job_aggregate_sampled_peak_rss=aggregate[0] or None,
               per_process_observed_peak_rss=peaks, missing_memory_samples=missing[0],
               pid_exit_races=exit_races, initial_suspended_root_rss=initial_rss[0] if initial_rss else None,
               supervisor_rss_limit=768*1024**2, supervisor_peak_rss=supervisor_peak[0],
               supervisor_samples=supervisor_samples, owner=owner,
               remaining_job_pids=remaining, process_closure_verified=remaining == [] and not errors,
               terminations=terminations, disk_samples=samples, prospective=preflight,
               guard_sha256=sha_file(__file__), budget_sha256=budget.raw_hash,
               qualification='one owned Job; sampled RSS; commit hard caps; shared token waits excluded from engine latency')
    write_json(receipt, row)
    return row


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--budget', required=True)
    parser.add_argument('--receipt', required=True)
    parser.add_argument('--seconds', required=True, type=float)
    parser.add_argument('--reserve-json', default='{}')
    parser.add_argument('--cwd')
    parser.add_argument('argv', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    command = args.argv[1:] if args.argv[:1] == ['--'] else args.argv
    outcome = run(command, args.receipt, args.budget, args.seconds,
                  json.loads(args.reserve_json), cwd=args.cwd)
    print(json.dumps({k: v for k, v in outcome.items() if k != 'disk_samples'}))
    sys.exit(0 if outcome.get('exit') == 0 and not outcome.get('censor')
             and outcome.get('process_closure_verified') else 1)
