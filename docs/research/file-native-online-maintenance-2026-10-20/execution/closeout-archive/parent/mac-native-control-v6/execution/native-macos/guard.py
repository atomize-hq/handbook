"""Experimental macOS own-session guard. CONTROL REVIEW CANDIDATE ONLY.
No global process enumeration. No automatic old-PID recovery or token unlink.
Foreground cooperative subprocesses must remain in the owned session.
"""
import argparse
import datetime as dt
import errno
import fcntl
import gzip
import hashlib
import json
import os
from pathlib import Path
import select
import secrets
import shutil
import signal
import socket
import subprocess
import sys
import threading
import time
from native_probe import identity, usage, group

ROOT = Path('/private/tmp/file-native-online-macos-20261020')
CUTOFF = dt.datetime.fromisoformat('2026-10-21T11:10:14+00:00').timestamp()
PERIOD = .03


def same(expected):
    actual = identity(expected['pid'])
    return actual, all(actual.get(k) == expected[k] for k in ('pid', 'uid', 'pgid', 'start')) and actual['returned_bytes'] == 136 and actual['errno'] == 0


def send(sock, row):
    raw = json.dumps(row, separators=(',', ':')).encode()
    if len(raw) > 4096:
        raise ValueError('control-message-cap')
    sock.send(raw)


def receive(sock, timeout):
    if not select.select([sock], [], [], timeout)[0]:
        return None
    raw = sock.recv(4097)
    if not raw:
        raise EOFError('control-peer-closed')
    if len(raw) > 4096:
        raise ValueError('control-message-cap')
    return json.loads(raw)


def kill_owned(anchor):
    observed, matches = same(anchor)
    row = {'identity': observed, 'matches': matches, 'signal_errno': None, 'steps': []}
    if not matches:
        row['signal'] = 'REFUSED_UNKNOWN_IDENTITY'
        return row
    try:
        # Freeze the nonce-owned anchor before enumeration so it cannot finish
        # a concurrent spawn after a one-shot group kill. Keep its PGID reserved
        # while draining all other members, including newly visible children.
        os.kill(anchor['pid'], signal.SIGSTOP)
        until = time.monotonic()+1
        while time.monotonic() < until:
            observed, matches = same(anchor)
            if not matches:
                row['signal'] = 'UNKNOWN_ANCHOR_WHILE_STOPPING'
                return row
            if observed.get('state') == 4:
                break
            time.sleep(.005)
        else:
            row['signal'] = 'ANCHOR_STOP_UNPROVEN'
            return row
        empty = 0
        while time.monotonic() < until:
            members = group(anchor['pgid'])
            if not members['valid']:
                row['signal'] = 'UNKNOWN_OWNED_MEMBERSHIP'
                return row
            others = [pid for pid in members['members'] if pid != anchor['pid']]
            empty = empty+1 if not others else 0
            row['steps'].append(members)
            if empty >= 2:
                break
            for pid in others:
                before = identity(pid)
                after = identity(pid)
                valid = before['returned_bytes'] == after['returned_bytes'] == 136 and before['errno'] == after['errno'] == 0
                valid = valid and before.get('uid') == anchor['uid'] and before.get('pgid') == anchor['pgid']
                valid = valid and all(before.get(k) == after.get(k) for k in ('pid','uid','pgid','start'))
                if valid:
                    try:
                        os.kill(pid, signal.SIGKILL)
                    except OSError as error:
                        row['steps'].append({'pid': pid, 'signal_errno': error.errno})
            time.sleep(.01)
        if empty < 2:
            row['signal'] = 'DRAIN_INCOMPLETE_ANCHOR_RETAINED'
            return row
        observed, matches = same(anchor)
        if not matches:
            row['signal'] = 'UNKNOWN_ANCHOR_BEFORE_FINAL_KILL'
            return row
        os.kill(anchor['pid'], signal.SIGKILL)
        row['signal'] = 'OWNED_MEMBERS_DRAINED_ANCHOR_KILLED'
    except OSError as e:
        row.update(signal='FAILED', signal_errno=e.errno)
    return row


def watchdog(sock, parent_identity, deadline, nonce, receipt):
    """Separate process; armed before token waits and all workload setup."""
    anchor = None
    events = []
    reason = None
    try:
        send(sock, {'ready': nonce, 'pid': os.getpid()})
        while True:
            observed_parent, parent_matches = same(parent_identity)
            if not parent_matches or observed_parent.get('state') == 5:
                reason = 'supervisor-dead-or-identity-unknown'
                break
            if time.time() >= CUTOFF or time.monotonic() >= deadline:
                reason = 'absolute-deadline'
                break
            try:
                row = receive(sock, min(.02, max(0, deadline-time.monotonic())))
            except EOFError:
                reason = 'supervisor-pipe-closed'
                break
            if row is None:
                continue
            if row.get('nonce') != nonce:
                reason = 'watchdog-nonce-mismatch'
                break
            if row.get('stop'):
                return
            if 'anchor' in row:
                anchor = row['anchor']
                observed, matches = same(anchor)
                if not matches:
                    events.append(observed)
                    reason = 'watchdog-anchor-identity'
                    break
                send(sock, {'anchor_ack': nonce})
    except BaseException as e:
        reason = 'watchdog-error:' + repr(e)
    finally:
        if reason:
            if anchor:
                events.append(kill_owned(anchor))
            receipt.write_text(json.dumps({'reason': reason, 'events': events,
                                           'utc': dt.datetime.now(dt.timezone.utc).isoformat()})+'\n')
            # Alert only our exact supervisor. Never resume/reuse a stale PID.
            observed, matches = same(parent_identity)
            if matches:
                try:
                    os.kill(parent_identity['pid'], signal.SIGUSR1)
                except OSError:
                    pass
        sock.close()


def anchor_main(fd, nonce, command, cwd, input_path):
    sock = socket.socket(fileno=fd)
    send(sock, {'anchor_ready': nonce, 'pid': os.getpid()})
    row = receive(sock, 5)
    if not row or row != {'go': nonce}:
        return 90
    inp = open(input_path, 'rb') if input_path else subprocess.DEVNULL
    try:
        p = subprocess.Popen(command, cwd=cwd, stdin=inp, close_fds=True)
        code = p.wait()
        send(sock, {'done': nonce, 'exit': code})
        row = receive(sock, max(0, CUTOFF-time.time()))
        return 0 if row == {'release': nonce} else 91
    finally:
        if input_path:
            inp.close()
        sock.close()


def disk_census(path):
    logical = allocated = files = 0
    for directory, dirs, names in os.walk(path, followlinks=False):
        for name in names:
            p = Path(directory)/name
            if p.is_symlink():
                raise ValueError('census-symlink:'+str(p))
            s = p.stat()
            logical += s.st_size
            allocated += s.st_blocks*512
            files += 1
    return {'logical_bytes': logical, 'allocated_bytes': allocated, 'files': files}


def run(ns):
    started = time.time()
    deadline = min(time.monotonic()+ns.seconds, time.monotonic()+max(0, CUTOFF-started))
    if started >= CUTOFF:
        raise RuntimeError('functional-cutoff-before-setup')
    outroot = ROOT/'evidence/control'/ns.name
    outroot.mkdir(exist_ok=False)
    nonce = secrets.token_hex(16)
    parent = identity(os.getpid())
    assert parent['returned_bytes'] == 136 and parent['errno'] == 0
    record = {'kind': 'native-guard-control-candidate', 'nonce': nonce,
              'source_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
              'supervisor': parent, 'started_epoch': started,
              'functional_cutoff': '2026-10-21T11:10:14Z', 'seconds_cap': ns.seconds,
              'global_enumeration': False, 'censor': None, 'closure': 'UNKNOWN',
              'samples': 0, 'peak_group_rss': 0, 'peak_supervisor_rss': 0,
              'missing_live_rss': [], 'transient_exited_members': [],
              'termination': [], 'sample_interval_max_s': 0, 'late_intervals': 0,
              'per_process_cap': ns.limit, 'group_cap': min(ns.limit, 2*1024**3),
              'supervisor_cap': 768*1024**2, 'aggregate_cap': 3*1024**3,
              'command': ns.command, 'source_pin': 'PENDING_NO_ENGINE_RELEASE'}
    triggered = []
    old = signal.signal(signal.SIGUSR1, lambda *_: triggered.append('independent-watchdog'))
    left, right = socket.socketpair(socket.AF_UNIX, socket.SOCK_DGRAM)
    watch = os.fork()
    if watch == 0:
        left.close()
        try:
            watchdog(right, parent, deadline, nonce, outroot/'watchdog.json')
        finally:
            os._exit(0)
    right.close()
    watch_identity = identity(watch)
    token = None
    process = None
    anchor = None
    conn = None
    work_exit = None
    watch_reaped = False
    sample_started = None
    samples_ended = None
    disk_stop = threading.Event()
    disk_thread = None
    disk_state = {'last': None, 'failure': None}
    stdout = stderr = samples = None
    try:
        ack = receive(left, 2)
        assert ack and ack.get('ready') == nonce
        token = (ROOT/'mac-heavy.lock').open('a+b')
        while True:
            if triggered or time.monotonic() >= deadline or time.time() >= CUTOFF:
                raise RuntimeError('deadline-during-token-wait')
            try:
                fcntl.flock(token.fileno(), fcntl.LOCK_EX|fcntl.LOCK_NB)
                break
            except BlockingIOError:
                time.sleep(.02)
        record['token_acquired'] = time.time()
        token.seek(0)
        token.truncate()
        token.write(json.dumps({'nonce': nonce, 'supervisor': parent, 'categories': ['F01','F08','F12'],
                                'fixed_cutoff': CUTOFF}).encode())
        token.flush()
        os.fsync(token.fileno())
        before = disk_census(ROOT)
        record['prelaunch_census'] = before
        free = shutil.disk_usage(ROOT).free
        record['prelaunch_free_bytes'] = free
        if free < ns.free_floor:
            raise RuntimeError('free-space-refusal')
        if before['logical_bytes']+ns.prospective > ns.runtime_cap:
            raise RuntimeError('prospective-runtime-cap-refusal')
        if triggered or time.monotonic() >= deadline:
            raise RuntimeError('deadline-during-setup')
        conn, other = socket.socketpair(socket.AF_UNIX, socket.SOCK_DGRAM)
        stdout = (outroot/'worker.stdout').open('xb')
        stderr = (outroot/'worker.stderr').open('xb')
        process = subprocess.Popen([sys.executable, '-B', str(Path(__file__).resolve()),
                     '--anchor', str(other.fileno()), nonce, ns.cwd or '', ns.input or '', *ns.command],
                     pass_fds=(other.fileno(),), start_new_session=True, stdout=stdout, stderr=stderr)
        other.close()
        hello = receive(conn, 2)
        assert hello and hello.get('anchor_ready') == nonce and hello.get('pid') == process.pid
        anchor = identity(process.pid)
        assert anchor['returned_bytes'] == 136 and anchor['errno'] == 0
        assert anchor['ppid'] == os.getpid() and anchor['uid'] == os.getuid() and anchor['pgid'] == process.pid
        record['anchor'] = anchor
        send(left, {'nonce': nonce, 'anchor': anchor})
        assert receive(left, 2) == {'anchor_ack': nonce}
        assert group(process.pid)['members'] == [process.pid]
        samples = gzip.open(outroot/'rss-samples.jsonl.gz', 'xt')
        def monitor_disk():
            # No disk traversal occurs in the memory sampler or watchdog.
            while not disk_stop.is_set():
                try:
                    snapshot = {'epoch': time.time(), 'free_bytes': shutil.disk_usage(ROOT).free,
                                'runtime': disk_census(ROOT)}
                    if snapshot['free_bytes'] < ns.free_floor:
                        raise RuntimeError('live-free-space-cap')
                    if snapshot['runtime']['logical_bytes'] > ns.runtime_cap:
                        raise RuntimeError('live-runtime-cap')
                    categories = {}
                    for category, path, byte_cap, file_cap in [
                        ('F04', ROOT/'target', 128*1024**2, 5000),
                        ('F08', ROOT/'fixtures', 96*1024**2, 30000),
                        ('F12', ROOT/'evidence', 24*1024**2, 10000)]:
                        count = disk_census(path)
                        categories[category] = count
                        if count['logical_bytes'] > byte_cap or count['files'] > file_cap:
                            raise RuntimeError('live-category-cap:'+category)
                    snapshot['categories'] = categories
                    disk_state['last'] = snapshot
                except BaseException as error:
                    disk_state['failure'] = repr(error)
                    return
                disk_stop.wait(.25)
        disk_thread = threading.Thread(target=monitor_disk, daemon=True)
        disk_thread.start()
        send(conn, {'go': nonce})
        sample_started = time.monotonic()
        previous = None
        next_sample = sample_started
        while True:
            now = time.monotonic()
            if triggered or now >= deadline or time.time() >= CUTOFF:
                raise RuntimeError('absolute-deadline')
            w, wc = os.waitpid(watch, os.WNOHANG)
            if w:
                watch_reaped = True
                raise RuntimeError('watchdog-exited-before-closure')
            if disk_state['failure']:
                raise RuntimeError(disk_state['failure'])
            members = group(process.pid)
            if not members['valid'] or not members['members']:
                raise RuntimeError('membership-unknown-or-anchor-missing')
            heavy = helper = 0
            rows = []
            for pid in sorted(set(members['members']+[os.getpid(), watch])):
                ident = identity(pid)
                u = usage(pid)
                rows.append({'identity': ident, 'usage': u})
                valid = ident['returned_bytes'] == 136 and ident['errno'] == 0 and ident.get('uid') == os.getuid()
                if pid in (os.getpid(), watch):
                    expected = parent if pid == os.getpid() else watch_identity
                    valid = valid and all(ident.get(k) == expected.get(k) for k in ('pid','start','uid','pgid'))
                else:
                    valid = valid and ident.get('pgid') == process.pid
                if not valid or u['return'] != 0 or u['rss_bytes'] is None:
                    again = group(process.pid)
                    if pid not in (os.getpid(), watch) and again['valid'] and pid not in again['members']:
                        record['transient_exited_members'].append(rows[-1])
                        continue
                    record['missing_live_rss'].append(rows[-1])
                    raise RuntimeError('missing-live-rss-or-identity')
                if pid in (os.getpid(), watch):
                    helper += u['rss_bytes']
                else:
                    heavy += u['rss_bytes']
                    if u['rss_bytes'] > ns.limit:
                        record['violating_rss_observation'] = {'monotonic_s': now, 'process': rows[-1], 'group_partial_rss': heavy, 'members': members}
                        record['peak_group_rss'] = max(record['peak_group_rss'], heavy)
                        samples.write(json.dumps(record['violating_rss_observation'])+'\n')
                        samples.flush()
                        raise RuntimeError('per-process-rss-cap')
            if previous is not None:
                gap = now-previous
                record['sample_interval_max_s'] = max(record['sample_interval_max_s'], gap)
                record['late_intervals'] += int(gap > .04)
            previous = now
            record['samples'] += 1
            record['peak_group_rss'] = max(record['peak_group_rss'], heavy)
            record['peak_supervisor_rss'] = max(record['peak_supervisor_rss'], helper)
            samples.write(json.dumps({'monotonic_s': now, 'members': members, 'processes': rows,
                                     'group_rss': heavy, 'helpers_rss': helper})+'\n')
            samples.flush()
            if heavy > min(ns.limit, 2*1024**3) or helper > 768*1024**2 or heavy+helper > 3*1024**3:
                raise RuntimeError('aggregate-rss-cap')
            row = receive(conn, 0)
            if row:
                assert row.get('done') == nonce
                work_exit = row['exit']
                current = group(process.pid)
                if not current['valid']:
                    raise RuntimeError('completion-membership-unknown')
                if current['members'] != [process.pid]:
                    raise RuntimeError('root-completed-orphan-group')
                send(conn, {'release': nonce})
                process.wait(timeout=2)
                break
            # Raw worker output has a small control-only ceiling.
            if stdout.tell()+stderr.tell() > 1024*1024:
                raise RuntimeError('control-output-cap')
            next_sample += PERIOD
            time.sleep(max(0, next_sample-time.monotonic()))
        samples_ended = time.monotonic()
    except BaseException as e:
        record['censor'] = str(e) or type(e).__name__
    finally:
        samples_ended = samples_ended or time.monotonic()
        disk_stop.set()
        if disk_thread:
            disk_thread.join(timeout=1)
            record['disk_monitor_closed'] = not disk_thread.is_alive()
            if disk_thread.is_alive() and record['censor'] is None:
                record['censor'] = 'disk-monitor-not-closed'
        record['last_disk_observation'] = disk_state
        if process is not None:
            if anchor and process.poll() is None:
                record['termination'].append(kill_owned(anchor))
            try:
                process.wait(timeout=2)
            except subprocess.TimeoutExpired:
                record['wait_failure'] = True
            observations = []
            closure_until = time.monotonic()+2
            while time.monotonic() < closure_until:
                observation = group(process.pid)
                if observation['valid'] and observation['members']:
                    observation['remaining_identities'] = [{'identity': identity(pid), 'usage': usage(pid)} for pid in observation['members']]
                observations.append(observation)
                if len(observations) >= 2 and all(r['valid'] and r['members'] == [] for r in observations[-2:]):
                    break
                time.sleep(.05)
            record['closure_history'] = observations
            observations = observations[-2:]
            record['closure_observations'] = observations
            record['closure'] = 'PROVEN_EMPTY' if all(r['valid'] and r['members'] == [] for r in observations) else 'UNKNOWN_OR_REMAINING'
            record['anchor_exit'] = process.returncode
        else:
            record['closure'] = 'NO_WORKLOAD_CREATED'
        try:
            send(left, {'nonce': nonce, 'stop': True})
        except OSError:
            pass
        left.close()
        if conn:
            conn.close()
        if not watch_reaped:
            os.waitpid(watch, 0)
        record['watchdog_exit_reaped'] = True
        if token:
            token.close()
        for h in (stdout, stderr, samples):
            if h:
                h.close()
        signal.signal(signal.SIGUSR1, old)
        record['worker_exit'] = work_exit
        record['finished_epoch'] = time.time()
        if sample_started:
            span = (samples_ended or time.monotonic())-sample_started
            record['sample_span_s'] = span
            record['effective_samples_hz'] = record['samples']/span if span else None
        record['final_census'] = disk_census(ROOT)
        record['qualification'] = 'CONTROL_CANDIDATE_ONLY_NO_ENGINE_RELEASE'
        (outroot/'result.json').write_text(json.dumps(record, indent=2)+'\n')
    return record

if __name__ == '__main__':
    if len(sys.argv)>1 and sys.argv[1] == '--anchor':
        raise SystemExit(anchor_main(int(sys.argv[2]), sys.argv[3], sys.argv[6:], sys.argv[4] or None, sys.argv[5] or None))
    p = argparse.ArgumentParser()
    p.add_argument('--name', required=True)
    p.add_argument('--seconds', type=float, default=5)
    p.add_argument('--cwd')
    p.add_argument('--input')
    p.add_argument('--limit', type=int, default=2*1024**3)
    p.add_argument('--free-floor', type=int, default=20*1024**3)
    p.add_argument('--runtime-cap', type=int, default=4*1024**2)
    p.add_argument('--prospective', type=int, default=32768)
    p.add_argument('command', nargs=argparse.REMAINDER)
    ns = p.parse_args()
    if ns.command[:1] == ['--']:
        ns.command = ns.command[1:]
    if not ns.command or not ns.name.replace('-','').isalnum():
        p.error('command and bounded alphanumeric name required')
    r = run(ns)
    print(json.dumps({k:r[k] for k in ('censor','closure','samples','peak_group_rss','peak_supervisor_rss','worker_exit')}))
    raise SystemExit(0 if r['censor'] is None and r['closure']=='PROVEN_EMPTY' and r['worker_exit']==0 else 1)
