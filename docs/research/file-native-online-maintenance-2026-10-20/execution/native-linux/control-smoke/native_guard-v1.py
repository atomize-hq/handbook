"""Bounded native Linux process lane; no engine-specific qualification logic."""
import argparse
import ctypes
import datetime as dt
import fcntl
import gzip
import hashlib
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import threading
import time

ROOT = Path('/home/spenser/file-native-online-linux-20261020')
CUTOFF = dt.datetime.fromisoformat('2026-10-21T11:10:14+00:00').timestamp()
MIB = 1024 ** 2
LIMITS = {'source': (4*MIB, 1500), 'target': (128*MIB, 5000),
          'fixtures': (32*MIB, 30000), 'evidence': (24*MIB, 2000)}


def digest(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as stream:
        for block in iter(lambda: stream.read(MIB), b''):
            h.update(block)
    return h.hexdigest()


def inventory(path, allow_links=False):
    total = allocated = files = directories = 0
    if not path.exists():
        return dict(bytes=0, allocated_bytes=0, files=0, directories=0)
    if path.is_symlink():
        raise RuntimeError('owned-root-symlink')
    for folder, dirs, names in os.walk(path, followlinks=False):
        directories += len(dirs)
        for name in dirs + names:
            p = Path(folder)/name
            try:
                s = p.lstat()
            except FileNotFoundError:
                continue
            if p.is_symlink() and not allow_links:
                raise RuntimeError('owned-symlink:'+str(p))
            if name in names:
                total += s.st_size
                allocated += s.st_blocks*512
                files += 1
    return dict(bytes=total, allocated_bytes=allocated, files=files,
                directories=directories)


def process(pid):
    """Read identity and current/high-water memory; disappearance is normal."""
    try:
        stat = Path(f'/proc/{pid}/stat').read_text()
        fields = stat[stat.rfind(')')+2:].split()
        values = {}
        for line in Path(f'/proc/{pid}/status').read_text().splitlines():
            if line.startswith(('VmRSS:', 'VmHWM:')):
                key, value = line.split(':', 1)
                values[key] = int(value.split()[0])*1024
        cmd = Path(f'/proc/{pid}/cmdline').read_bytes().replace(b'\0', b' ').decode(errors='replace')
        return dict(pid=pid, state=fields[0], ppid=int(fields[1]),
                    pgid=int(fields[2]), start=int(fields[19]),
                    rss=values.get('VmRSS', 0), hwm=values.get('VmHWM', 0), cmd=cmd)
    except (FileNotFoundError, ProcessLookupError):
        return None


def processes():
    return {
        int(p.name): row for p in Path('/proc').iterdir() if p.name.isdigit()
        if (row := process(int(p.name))) is not None}


def child(args):
    fd = int(args[0])
    if os.read(fd, 1) != b'G':
        raise SystemExit(125)
    os.close(fd)
    command = args[1:]
    os.execvpe(command[0], command, os.environ)


def run(ns):
    if sys.platform != 'linux':
        raise RuntimeError('native-linux-required')
    if ROOT.resolve() != ROOT or ROOT.is_symlink():
        raise RuntimeError('runtime-containment')
    ROOT.mkdir(exist_ok=True)
    for name in LIMITS:
        (ROOT/name).mkdir(exist_ok=True)
    receipt = Path(ns.receipt).resolve()
    if not receipt.is_relative_to(ROOT/'evidence') or receipt.exists():
        raise RuntimeError('new-owned-receipt-required')
    receipt.parent.mkdir(parents=True, exist_ok=True)
    lock = (ROOT/'heavy-session.lock').open('a+b')
    fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    libc = ctypes.CDLL(None, use_errno=True)
    if libc.prctl(36, 1, 0, 0, 0) != 0:
        raise OSError(ctypes.get_errno(), 'subreaper')
    start = time.time()
    effective = min(ns.seconds, CUTOFF-start)
    if effective <= 0:
        raise RuntimeError('functional-cutoff')
    before = {k: inventory(ROOT/k) for k in LIMITS}
    cargo = Path('/home/spenser/.cargo')
    cargo_before = inventory(cargo, allow_links=True) if ns.monitor_cargo else None
    for key, val in before.items():
        if val['bytes'] > LIMITS[key][0] or val['files'] > LIMITS[key][1]:
            raise RuntimeError('preflight-allocation:'+key)
    if shutil.disk_usage(ROOT).free < ns.free_floor:
        raise RuntimeError('preflight-free-floor')
    env = os.environ.copy()
    env['PATH'] = '/home/spenser/.cargo/bin:'+env.get('PATH', '')
    env['CARGO_INCREMENTAL'] = '0'
    env['CARGO_NET_OFFLINE'] = 'true'
    env['PYTHONDONTWRITEBYTECODE'] = '1'
    read_fd, write_fd = os.pipe()
    output = receipt.with_suffix('.stdout').open('wb')
    errors = receipt.with_suffix('.stderr').open('wb')
    stdin = open(ns.stdin, 'rb') if ns.stdin else subprocess.DEVNULL
    command = ns.command[1:] if ns.command[:1] == ['--'] else ns.command
    if not command:
        raise RuntimeError('command-required')
    p = subprocess.Popen([sys.executable, str(Path(__file__).resolve()),
                          '--child', str(read_fd), *command],
                         cwd=ns.cwd, env=env, stdin=stdin, stdout=output,
                         stderr=errors, start_new_session=True, pass_fds=(read_fd,))
    os.close(read_fd)
    initial = process(p.pid)
    if initial is None:
        raise RuntimeError('child-disappeared-before-launch')
    tracked = {p.pid: initial['start']}
    pidfds = {p.pid: os.pidfd_open(p.pid)}
    stop = threading.Event()
    reasons = []
    termination = []
    memory_path = receipt.with_suffix('.memory.jsonl.gz')
    memory_stream = gzip.open(memory_path, 'wt', encoding='utf-8')
    memory_count = 0
    aggregate_peak = supervisor_peak = 0
    disk_rows = []
    sample_errors = []
    process_peaks = {}
    intervals = []
    last_sample = None
    state_lock = threading.Lock()

    def terminate(reason):
        with state_lock:
            if reason not in reasons:
                reasons.append(reason)
            handles = list(pidfds.items())
        termination.append(dict(epoch=time.time(), reason=reason,
                                tracked_pids=[pid for pid, _ in handles]))
        try:
            os.killpg(p.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        for _, fd in handles:
            try:
                signal.pidfd_send_signal(fd, signal.SIGKILL)
            except ProcessLookupError:
                pass

    def members(rows):
        selected = {pid for pid, row in rows.items()
                    if row['pgid'] == p.pid or tracked.get(pid) == row['start']}
        while True:
            added = {pid for pid, row in rows.items()
                     if row['ppid'] in selected or row['ppid'] == os.getpid() and pid != p.pid}
            if added <= selected:
                break
            selected |= added
        with state_lock:
            for pid in selected:
                row = rows[pid]
                if pid not in tracked:
                    tracked[pid] = row['start']
                    try:
                        pidfds[pid] = os.pidfd_open(pid)
                    except ProcessLookupError:
                        pass
        return [rows[pid] for pid in selected]

    def sample_memory():
        nonlocal last_sample, memory_count, aggregate_peak, supervisor_peak
        due = time.monotonic()
        while not stop.is_set():
            begin = time.monotonic()
            try:
                rows = processes()
                owned = members(rows)
                if last_sample is not None:
                    intervals.append(begin-last_sample)
                last_sample = begin
                supervisors = {os.getpid()}
                parent = rows.get(os.getpid(), {}).get('ppid', 0)
                while parent > 1 and parent in rows:
                    supervisors.add(parent)
                    parent = rows[parent]['ppid']
                supervisors |= {pid for pid, row in rows.items()
                                if str(ROOT) in row['cmd'] and pid not in {x['pid'] for x in owned}}
                heavy = sum(x['rss'] for x in owned)
                overhead = sum(rows[pid]['rss'] for pid in supervisors if pid in rows)
                for row in owned:
                    process_peaks[str(row['pid'])] = max(process_peaks.get(str(row['pid']), 0), row['hwm'])
                memory_stream.write(json.dumps(dict(epoch=time.time(), heavy_rss=heavy,
                                          supervisor_rss=overhead,
                                          members=[dict(pid=x['pid'], start=x['start'],
                                                        rss=x['rss'], hwm=x['hwm'],
                                                        pgid=x['pgid']) for x in owned]),
                                               separators=(',', ':'))+'\n')
                memory_count += 1
                aggregate_peak = max(aggregate_peak, heavy)
                supervisor_peak = max(supervisor_peak, overhead)
                if heavy > ns.heavy_limit or any(x['rss'] > ns.process_limit for x in owned):
                    terminate('heavy-or-process-rss')
                    return
                if any(x['rss'] > 256*MIB and
                       x['cmd'].split(' ', 1)[0].endswith('/file-native-incremental')
                       for x in owned):
                    terminate('engine-reader-conservative-256mib')
                    return
                if overhead > 768*MIB or heavy+overhead >= 3*1024**3:
                    terminate('supervisor-or-host-rss')
                    return
            except BaseException as e:
                sample_errors.append(repr(e))
                terminate('memory-observation-error')
                return
            due += .02
            stop.wait(max(0, due-time.monotonic()))

    def sample_disk():
        while not stop.is_set():
            try:
                now = {k: inventory(ROOT/k) for k in LIMITS}
                free = shutil.disk_usage(ROOT).free
                row = dict(epoch=time.time(), scopes=now, free_bytes=free)
                if ns.monitor_cargo:
                    current_cargo = inventory(cargo, allow_links=True)
                    row['cargo_growth'] = max(0, current_cargo['bytes']-cargo_before['bytes'])
                    if row['cargo_growth'] > 4*MIB:
                        terminate('shared-cargo-growth')
                        return
                disk_rows.append(row)
                for key, value in now.items():
                    if value['bytes'] > LIMITS[key][0] or value['files'] > LIMITS[key][1]:
                        terminate('allocation:'+key)
                        return
                if free < ns.free_floor:
                    terminate('free-floor')
                    return
            except BaseException as e:
                sample_errors.append(repr(e))
                terminate('disk-observation-error')
                return
            stop.wait(.25)

    timer = threading.Timer(effective, terminate, args=('time-cutoff',))
    timer.daemon = True
    sampler = threading.Thread(target=sample_memory, daemon=True)
    disk = threading.Thread(target=sample_disk, daemon=True)
    timer.start()
    sampler.start()
    disk.start()
    closure = []
    try:
        os.write(write_fd, b'G')
        os.close(write_fd)
        while p.poll() is None and not reasons:
            time.sleep(.01)
        if reasons:
            p.wait(timeout=10)
        else:
            p.wait(timeout=10)
        # Subreaper ownership catches descendants that escaped the original PGID.
        live = members(processes())
        if any(x['state'] != 'Z' for x in live):
            terminate('root-exit-left-descendants')
        until = time.monotonic()+10
        while time.monotonic() < until:
            for pid in list(tracked):
                if pid == p.pid:
                    continue
                try:
                    os.waitpid(pid, os.WNOHANG)
                except ChildProcessError:
                    pass
            remaining = members(processes())
            closure.append(dict(epoch=time.time(), pids=[x['pid'] for x in remaining],
                                states=[x['state'] for x in remaining]))
            if len(closure) >= 2 and not closure[-1]['pids'] and not closure[-2]['pids']:
                break
            time.sleep(.05)
    finally:
        if p.poll() is None:
            terminate('supervisor-finally')
            p.wait(timeout=10)
        stop.set()
        timer.cancel()
        timer.join(timeout=2)
        sampler.join(timeout=3)
        disk.join(timeout=3)
        memory_stream.close()
        output.close()
        errors.close()
        if ns.stdin:
            stdin.close()
        for fd in pidfds.values():
            os.close(fd)
    positive = len(closure) >= 2 and not closure[-1]['pids'] and not closure[-2]['pids']
    row = dict(command=command, cwd=ns.cwd, source_pin=ns.pin,
               guard_sha256=digest(__file__), started_epoch=start,
               effective_cap_seconds=effective, seconds=time.time()-start,
               exit=p.returncode, censors=reasons, termination=termination,
               group_id=p.pid, tracked_identity=tracked,
               process_closure_verified=positive, closure_observations=closure,
               memory_interval_target_seconds=.02,
               memory_max_interval_seconds=max(intervals, default=None),
               memory_intervals_over_40ms=sum(x > .04 for x in intervals),
               memory_samples_path=str(memory_path), memory_samples_sha256=digest(memory_path),
               memory_sample_count=memory_count, aggregate_heavy_peak_bytes=aggregate_peak,
               aggregate_supervisor_peak_bytes=supervisor_peak,
               process_high_water_bytes=process_peaks,
               heavy_rss_limit=ns.heavy_limit, process_rss_limit=ns.process_limit,
               supervisor_rss_limit=768*MIB, native_host_rss_limit=3*1024**3,
               free_floor_bytes=ns.free_floor, disk_samples=disk_rows,
               baseline=before, final={k: inventory(ROOT/k) for k in LIMITS},
               observation_errors=sample_errors,
               stdout_sha256=digest(receipt.with_suffix('.stdout')),
               stderr_sha256=digest(receipt.with_suffix('.stderr')))
    receipt.write_text(json.dumps(row, separators=(',', ':'))+'\n')
    print(json.dumps({k: row[k] for k in ('exit', 'censors', 'seconds',
                                        'process_closure_verified',
                                        'memory_max_interval_seconds',
                                        'memory_intervals_over_40ms')}))
    return 0 if p.returncode == 0 and not reasons and positive and not sample_errors else 1


if __name__ == '__main__':
    if sys.argv[1:2] == ['--child']:
        child(sys.argv[2:])
    parser = argparse.ArgumentParser()
    parser.add_argument('--receipt', required=True)
    parser.add_argument('--seconds', type=float, required=True)
    parser.add_argument('--pin', required=True)
    parser.add_argument('--cwd')
    parser.add_argument('--stdin')
    parser.add_argument('--monitor-cargo', action='store_true')
    parser.add_argument('--process-limit', type=int, default=2*1024**3)
    parser.add_argument('--heavy-limit', type=int, default=2*1024**3)
    parser.add_argument('--free-floor', type=int, default=20*1024**3)
    parser.add_argument('command', nargs=argparse.REMAINDER)
    raise SystemExit(run(parser.parse_args()))
