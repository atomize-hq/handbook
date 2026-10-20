"""PROPOSAL ONLY. Default is a read-only forecast; --execute requires a later bound GO.
No shared Cargo mutations, cleanup, engine CLI, source edits, or address-space limit.
Disk thresholds are conservative sampled stops, NOT an aggregate filesystem quota.
"""
import os, sys, json, time, signal, subprocess, hashlib, stat, platform, datetime, select, resource
from pathlib import Path
M = 1048576
O = Path('/private/tmp/file-native-p-macos-20261020')
C = O / 'cargo-private-proposed'
R = C / '.p-build-receipts'
S = O / 'source-final/docs/research/file-native-incremental-engine-experiment-2026-10-18'
T = O / 'target-private-proposed'
F = O / 'frozen-bin-private-proposed'
U = S / 'target'  # Existing cache test uses CARGO_MANIFEST_DIR, not CARGO_TARGET_DIR.
CONFIG = R / 'proposal.json'
ORIGINAL_CUT = datetime.datetime.fromisoformat('2026-10-20T07:55:16+00:00').timestamp()
TOOLBIN = Path('/Users/spensermcconnell/.rustup/toolchains/1.89.0-aarch64-apple-darwin/bin')
CARGO = TOOLBIN / 'cargo'
RUSTC = TOOLBIN / 'rustc'
CLANG = Path('/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang')

def utc():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()

def digest(p):
    h = hashlib.sha256()
    with p.open('rb') as f:
        for b in iter(lambda: f.read(M), b''):
            h.update(b)
    return h.hexdigest()

def regular(root):
    if not root.exists():
        return []
    if root.is_file():
        assert not root.is_symlink()
        return [root]
    out = []
    for parent, dirs, names in os.walk(root, followlinks=False):
        for n in dirs:
            assert not (Path(parent) / n).is_symlink(), 'symlink directory'
        for n in names:
            p = Path(parent) / n
            assert stat.S_ISREG(p.lstat().st_mode), str(p)
            out.append(p)
    return sorted(out)

def census(root):
    fs = regular(root)
    return {'bytes': sum(p.stat().st_size for p in fs), 'files': len(fs)}

def save(name, value):
    p = R / name
    assert p.parent == R and not p.exists(), 'never overwrite a prior receipt'
    raw = (json.dumps(value, indent=2) + '\n').encode()
    assert len(raw) <= 512 * 1024
    with p.open('xb') as f:
        f.write(raw)

def kill_group(pgid):
    try:
        os.killpg(pgid, signal.SIGTERM)
    except ProcessLookupError:
        return
    time.sleep(0.8)
    try:
        os.killpg(pgid, signal.SIGKILL)
    except ProcessLookupError:
        pass

def timer(pgid, deadline, fd):
    # Separate OS process, outside worker group. No ps/census/hash/input reads.
    # Cancellation byte closes normally. EOF (supervisor death) fails closed.
    while True:
        remaining = deadline - time.time()
        if remaining <= 0:
            kill_group(pgid)
            return 124
        ready, _, _ = select.select([fd], [], [], min(remaining, 0.2))
        if ready:
            if os.read(fd, 1) == b'X':
                return 0
            kill_group(pgid)
            return 125

def arm(pgid, deadline):
    readfd, writefd = os.pipe()
    proc = subprocess.Popen([sys.executable, '-B', __file__, '--timer', str(pgid), str(deadline), str(readfd)],
                            pass_fds=(readfd,), start_new_session=True, close_fds=True)
    os.close(readfd)
    return proc, writefd

def cancel(pair):
    proc, fd = pair
    try:
        os.write(fd, b'X')
    except BrokenPipeError:
        pass
    finally:
        os.close(fd)
    proc.wait(timeout=3)
    assert proc.returncode == 0, ('watchdog fired', proc.returncode)

def ps_group(pgid):
    p = subprocess.run(['/bin/ps', '-axo', 'pid,ppid,pgid,rss'], capture_output=True, text=True, timeout=2)
    assert p.returncode == 0, 'RSS unavailable; fail closed, value remains NULL'
    rows = []
    for line in p.stdout.splitlines()[1:]:
        try:
            pid, ppid, group, kib = map(int, line.split())
        except ValueError:
            continue
        if group == pgid:
            rows.append({'pid': pid, 'ppid': ppid, 'rss_bytes': kib * 1024})
    return rows

def ps_tree(root_pid):
    p = subprocess.run(['/bin/ps', '-axo', 'pid,ppid,pgid,rss'], capture_output=True, text=True, timeout=2)
    assert p.returncode == 0, 'RSS unavailable; fail closed with NULL'
    all_rows = []
    for line in p.stdout.splitlines()[1:]:
        try:
            pid, ppid, pgid, kib = map(int, line.split())
        except ValueError:
            continue
        all_rows.append({'pid': pid, 'ppid': ppid, 'pgid': pgid, 'rss_bytes': kib * 1024})
    included = {root_pid}
    while True:
        more = {r['pid'] for r in all_rows if r['ppid'] in included}
        if more <= included:
            break
        included |= more
    rows = [r for r in all_rows if r['pid'] in included]
    assert any(r['pid'] == root_pid for r in rows), 'supervisor RSS missing'
    return rows

def account(unit=False):
    whole, cache, target, frozen, fixture = [census(p) for p in [O, C, T, F, U]]
    other = whole['bytes'] - cache['bytes'] - target['bytes'] - frozen['bytes'] - fixture['bytes']
    reserve = 1572864
    assert cache['bytes'] <= 36*M and other + reserve <= 4*M
    assert target['bytes'] <= 164*M and frozen['bytes'] <= 8*M
    assert fixture['bytes'] <= (84*M if unit else 0)
    assert whole['bytes'] + reserve <= 256*M and whole['files'] <= 5000
    assert os.statvfs(O).f_bavail * os.statvfs(O).f_frsize > 20*1024**3
    return dict(whole=whole, private_cache=cache, target=target, frozen=frozen,
                unit_fixture=fixture, other_bytes=other, unmeasured_Git_OS_reserve_bytes=reserve)

def environment():
    # Whitelist prevents inherited CARGO_*, RUSTFLAGS/encoded flags, wrappers,
    # RUSTUP_TOOLCHAIN, CC/CXX/AR options, DYLD_*, foreign target config, etc.
    return {'PATH': str(TOOLBIN) + ':/usr/bin:/bin:/usr/sbin:/sbin',
            'HOME': str(C/'run-home'), 'CARGO_HOME': str(C), 'TMPDIR': str(T/'tmp'),
            'TMP': str(T/'tmp'), 'TEMP': str(T/'tmp'),
            'CLANG_MODULE_CACHE_PATH': str(T/'clang-module-cache'),
            'CARGO_TARGET_DIR': str(T), 'RUSTC': str(RUSTC), 'RUSTDOC': str(TOOLBIN/'rustdoc'),
            'CC': str(CLANG), 'CXX': str(CLANG),
            'CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER': str(CLANG),
            'CARGO_NET_OFFLINE': 'true', 'CARGO_CACHE_AUTO_CLEAN_FREQUENCY': 'never',
            'CARGO_INCREMENTAL': '0', 'CARGO_BUILD_JOBS': '1',
            'CARGO_PROFILE_RELEASE_DEBUG': '0', 'CARGO_PROFILE_RELEASE_STRIP': 'symbols',
            'CARGO_PROFILE_RELEASE_CODEGEN_UNITS': '1', 'PYTHONDONTWRITEBYTECODE': '1',
            'LANG': 'C', 'LC_ALL': 'C', 'TZ': 'UTC'}

def file_limit():
    # Kernel limit applies only to tool/compiler/test descendants. No RLIMIT_AS.
    resource.setrlimit(resource.RLIMIT_FSIZE, (16*M, 16*M))

def old_snapshots():
    # Preserve old Q/N payloads with complete path/size/content streaming digest.
    # These pre-existing roots are observations, never destinations or cleanup targets.
    rows = json.loads((O/'preflight/root-census.json').read_bytes())
    roots = [Path(z['root']) for z in rows if any(k in z['root'] for k in ['file-native-q-', 'file-native-n-'])]
    out = {}
    for root in roots:
        h = hashlib.sha256(); count = total = 0
        for p in regular(root):
            n = p.stat().st_size
            relative = str(p.relative_to(root)) if root.is_dir() else p.name
            h.update(relative.encode() + b'\0' + str(n).encode() + b'\0' + digest(p).encode() + b'\n')
            count += 1; total += n
        out[str(root)] = {'files': count, 'bytes': total, 'path_size_content_sha256': h.hexdigest()}
    return out

def preservation():
    before = json.loads((C/'.p-preparation-receipts/source-compiler-before.json').read_bytes())
    source_count = 0
    for path, expected in before['source'].items():
        q = Path(path)
        assert q.stat().st_size == expected['bytes'] and digest(q) == expected['sha256'], ('source drift', path)
        source_count += 1
    assert source_count == 145 and len(before['core18']) == 18
    for z in before['tools']:
        assert Path(z['path']).stat().st_size == z['bytes'] and digest(Path(z['path'])) == z['sha256']
    private = json.loads((C/'.p-preparation-receipts/private-immutable-inventory.json').read_bytes())
    for path, z in private.items():
        q = C/path
        assert q.stat().st_size == z['bytes'] and digest(q) == z['sha256'], ('private registry drift', path)
    shared = json.loads((C/'.p-preparation-receipts/shared-before-hashes.json').read_bytes())
    matched = 0; observations = {}
    for path, z in shared.items():
        q = Path(path)
        if q.name.startswith('.global-cache'):
            observations[path] = {'bytes': q.stat().st_size, 'sha256': digest(q), 'utc': utc(), 'attribution': 'UNKNOWN'}
        else:
            assert q.stat().st_size == z['bytes'] and digest(q) == z['sha256'], ('shared payload/index/lock drift', path)
            matched += 1
    return {'utc': utc(), 'source_files_matched': source_count, 'core_files': 18,
            'tool_bindings': before['tools'], 'private_registry_files_matched': len(private),
            'shared_non_DB_files_matched': matched, 'shared_DB_observations_only': observations,
            'shared_DB_preservation_through_closeout': None, 'writer_attribution': 'UNKNOWN'}

def worker(auth):
    # Outer dedicated watchdog is already armed before this release handshake.
    assert sys.stdin.buffer.read(1) == b'G'
    end = min(auth['go_epoch'] + auth['whole_stage_seconds'], ORIGINAL_CUT)
    assert time.time() < end and platform.system() == 'Darwin' and platform.machine() == 'arm64'
    assert not T.exists() and not F.exists() and not U.exists(), 'fresh target and retained fixture required'
    assert C.resolve() == C and O.resolve() == O and S.resolve() == S
    for ancestor in [S, *S.parents]:
        assert not (ancestor/'.cargo/config').exists() and not (ancestor/'.cargo/config.toml').exists(), ('unexpected ancestor config', str(ancestor))
    assert (C/'config.toml').read_bytes() == b'[net]\noffline = true\n[cache]\nauto-clean-frequency = "never"\n'
    initial = account(); baseline = preservation(); old = old_snapshots()
    save('execution-before.json', {'utc': utc(), 'accounting': initial, 'preservation': baseline, 'old_QN': old})
    assert time.time() < end
    # These are created ONLY in future explicitly admitted execution.
    for d in [T, T/'tmp', T/'clang-module-cache', C/'run-home']:
        d.mkdir()
    env = environment(); commands = []
    def command(label, argv, seconds, unit=False):
        cutoff = min(end, time.time() + seconds)
        watch = arm(os.getpgrp(), cutoff)  # Independent per-child whole-group timer.
        try:
            with (R/(label+'.stdout')).open('xb') as out, (R/(label+'.stderr')).open('xb') as err:
                p = subprocess.Popen(argv, cwd=S, env=env, stdout=out, stderr=err, preexec_fn=file_limit)
                rc = p.wait(timeout=max(0.01, cutoff-time.time()) + 1)
            assert not [r for r in ps_group(os.getpgrp()) if r['pid'] != os.getpid()], 'descendants remain after command'
            commands.append({'label': label, 'argv': argv, 'exit': rc, 'finished_utc': utc()})
            save(label+'-receipt.json', commands[-1])
            assert rc == 0, (label, rc)
        finally:
            cancel(watch)
        account(unit)
    command('rustc-native-host', [str(RUSTC), '-vV'], 120)
    host = (R/'rustc-native-host.stdout').read_text()
    assert 'host: aarch64-apple-darwin' in host and 'release: 1.89.0' in host
    common = ['--release', '--target', 'aarch64-apple-darwin', '--locked', '--offline', '-j1']
    command('native-build', [str(CARGO), 'build', *common, '-p', 'file-native-incremental', '--bin', 'file-native-incremental'], auth['build_seconds'])
    exe = T/'aarch64-apple-darwin/release/file-native-incremental'
    assert exe.is_file() and not exe.is_symlink() and exe.stat().st_size <= 8*M
    assert exe.read_bytes()[:4] == b'\xcf\xfa\xed\xfe', 'expected 64-bit little endian Mach-O'
    import struct
    with exe.open('rb') as f:
        header = f.read(12)
    assert struct.unpack('<I', header[4:8])[0] == 0x0100000c, 'expected ARM64 Mach-O CPU'
    F.mkdir()
    raw = exe.read_bytes()
    with (F/'file-native-incremental').open('xb') as f:
        f.write(raw)
    os.chmod(F/'file-native-incremental', 0o500)
    assert digest(F/'file-native-incremental') == digest(exe)
    save('frozen-binary.json', {'path': str(F/'file-native-incremental'), 'bytes': len(raw), 'sha256': digest(exe), 'architecture': 'aarch64-apple-darwin', 'executed': False})
    fresh = account(); unit_admitted = auth['conditional_units'] and fresh['whole']['bytes'] + 1572864 + 16*M + 84*M + 2*M <= 256*M
    save('unit-admission.json', {'utc': utc(), 'fresh_accounting': fresh, 'admitted': unit_admitted,
                               'max_pre_actual_with_reserve_bytes': 161480704, 'additional_target_bytes': 16*M,
                               'retained_fixture_bytes': 84*M, 'returns_bytes': 2*M})
    if unit_admitted:
        command('workspace-five-units', [str(CARGO), 'test', *common, '--workspace', '--lib', '--', '--test-threads=1', '--nocapture'], 120, True)
        stdout = (R/'workspace-five-units.stdout').read_text()
        assert 'test result: ok. 5 passed; 0 failed;' in stdout
    after = preservation(); old_after = old_snapshots()
    save('execution-after.json', {'utc': utc(), 'accounting': account(unit_admitted), 'preservation': after,
                                 'old_QN': old_after, 'old_QN_matched': old_after == old,
                                 'units': 'passed5' if unit_admitted else 'skipped-before-test-writes',
                                 'engine_CLI_invoked': False, 'qualification': None})
    assert old_after == old, 'old Q/N observation drift; never restore'

def execute(auth_path):
    # Activation record is the only input read before arming the outer timer.
    # Parent supplies a future GO bound to this immutable reviewed code/config.
    auth = json.loads(Path(auth_path).read_bytes())
    assert auth['status'] == 'PARENT_EXPLICIT_GO' and auth['allocated_bytes'] == 256*M
    assert 0 < auth['whole_stage_seconds'] <= 600 and 0 < auth['build_seconds'] <= 600
    assert isinstance(auth['conditional_units'], bool)
    auth['go_epoch'] = datetime.datetime.fromisoformat(auth['go_utc'].replace('Z', '+00:00')).timestamp()
    assert auth['go_epoch'] <= time.time() < min(auth['go_epoch'] + auth['whole_stage_seconds'], ORIGINAL_CUT)
    p = subprocess.Popen([sys.executable, '-B', __file__, '--worker', json.dumps(auth)], stdin=subprocess.PIPE, start_new_session=True)
    watch = arm(p.pid, min(auth['go_epoch'] + auth['whole_stage_seconds'], ORIGINAL_CUT))
    reason = None; peak = None; per_process_peak = None; samples = 0; unit = False; missing_samples = 0
    try:
        assert digest(Path(__file__)) == auth['code_sha256'] and digest(CONFIG) == auth['config_sha256']
        assert json.loads(CONFIG.read_bytes())['status'] == 'PROPOSAL_ONLY'
        p.stdin.write(b'G'); p.stdin.flush(); p.stdin.close()
        while p.poll() is None:
            try:
                rows = ps_tree(os.getpid())  # Includes independent watchdog processes.
            except BaseException:
                missing_samples += 1
                raise
            rss = sum(r['rss_bytes'] for r in rows)
            largest = max(r['rss_bytes'] for r in rows)
            peak = max(peak or 0, rss); per_process_peak = max(per_process_peak or 0, largest); samples += 1
            assert rss <= 3*1024**3 and largest <= 2*1024**3, 'RSS stop'
            if (R/'unit-admission.json').exists():
                unit = json.loads((R/'unit-admission.json').read_bytes())['admitted']
            a = account(unit)
            if unit:
                admission = json.loads((R/'unit-admission.json').read_bytes())
                assert a['target']['bytes'] <= admission['fresh_accounting']['target']['bytes'] + 16*M, 'unit target growth stop'
            if not unit:
                assert a['target']['bytes'] < 130*M and a['whole']['bytes'] + 1572864 < 220*M, 'build soft stop'
            time.sleep(0.1)
        p.wait(timeout=2)
        assert p.returncode == 0, ('worker failed', p.returncode)
    except BaseException as e:
        reason = type(e).__name__ + ': ' + str(e)
        kill_group(p.pid)
        p.wait(timeout=3)
    finally:
        # Close any orphan descendants even after a nominal worker exit.
        try:
            os.killpg(p.pid, 0)
            kill_group(p.pid)
        except ProcessLookupError:
            pass
        timer_error = None
        try:
            cancel(watch)
        except BaseException as e:
            timer_error = str(e)
        try:
            os.killpg(p.pid, 0); remaining = True
        except ProcessLookupError:
            remaining = False
        save('supervisor-final.json', {'utc': utc(), 'worker_exit': p.returncode, 'reason': reason,
             'watchdog_error': timer_error, 'remaining_group': remaining, 'worker_pgid': p.pid,
             'sampled_aggregate_peak_RSS_bytes': peak, 'sampled_per_process_peak_RSS_bytes': per_process_peak,
             'samples': samples, 'missing_samples': missing_samples, 'failed_or_timed_out_after_bindings': 'UNKNOWN unless execution-after.json exists', 'missing_RSS_policy': 'NULL and stop; never interpret missing as zero',
             'disk_enforcement': 'sampled conservative thresholds with inflight reserve, not hard quota',
             'RLIMIT_FSIZE_bytes': 16*M, 'RLIMIT_AS': None, 'checker_started': False,
             'checker_RSS_bytes': None, 'engine_CLI_invoked': False, 'qualification': None,
             'original_cutoff_utc': '2026-10-20T07:55:16Z', 'original_closeout_utc': '2026-10-20T08:25:16Z'})
    if reason or timer_error or remaining:
        raise SystemExit(1)

if __name__ == '__main__':
    if len(sys.argv) > 1 and sys.argv[1] == '--timer':
        raise SystemExit(timer(int(sys.argv[2]), float(sys.argv[3]), int(sys.argv[4])))
    elif len(sys.argv) > 1 and sys.argv[1] == '--worker':
        worker(json.loads(sys.argv[2]))
    elif len(sys.argv) == 3 and sys.argv[1] == '--execute':
        execute(sys.argv[2])
    else:
        print(json.dumps({'status': 'PROPOSAL_ONLY; NO EXECUTION', 'current40MiB': census(O),
                          'private': census(C), 'target_exists': T.exists(), 'fixture_exists': U.exists(),
                          'frozen_exists': F.exists(), 'future_allocation': None, 'config': json.loads(CONFIG.read_bytes())}, indent=2))
