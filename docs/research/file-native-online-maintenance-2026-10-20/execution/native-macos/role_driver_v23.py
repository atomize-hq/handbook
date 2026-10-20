"""Foreground reader/checker invocations inside one admitted native guard.
Explicit role selects memory bound; executable names never select policy.
Engine use requires the parent-reviewed source/build binding. Control mode only
launches the exact current Python executable and requires CONTROL_ONLY token.
"""
import fcntl
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import threading
import time
import admission_v20 as admission
import monitor_v20 as monitor
import keeper_v20 as keeper
from native_probe import identity, usage
from guard_v20 import Events

ROLE_LIMITS = {'reader': 256*1024**2, 'checker': 2*1024**3}


def sha256(path):
    value = hashlib.sha256()
    with Path(path).open('rb') as stream:
        for block in iter(lambda: stream.read(1024*1024), b''):
            value.update(block)
    return value.hexdigest()


def ancestor_token():
    token_path = admission.ROOT/'mac-heavy.lock'
    if token_path.is_symlink() or token_path.stat().st_size > 16384:
        raise RuntimeError('role-token-path-or-size')
    with token_path.open('rb') as token:
        record = admission.strict_json(token.read())
        try:
            fcntl.flock(token.fileno(), fcntl.LOCK_EX|fcntl.LOCK_NB)
        except BlockingIOError:
            pass
        else:
            raise RuntimeError('role-heavy-token-not-held')
    if not monitor.same_identity(record['parent'], identity(record['parent']['pid'])):
        raise RuntimeError('role-supervisor-identity')
    chain = []
    row = identity(os.getpid())
    for _ in range(8):
        if not monitor.valid_identity(row):
            raise RuntimeError('role-ancestor-identity-unknown')
        chain.append(row)
        if monitor.same_identity(record['keeper'], row):
            break
        row = identity(row['ppid'])
    else:
        raise RuntimeError('role-not-descendant-of-token-keeper')
    if len(chain) < 3 or chain[-2]['pgid'] != chain[-2]['pid']:
        raise RuntimeError('role-anchor-ancestor-shape')
    if identity(os.getpid())['pgid'] != chain[-2]['pgid']:
        raise RuntimeError('role-not-in-anchor-process-group')
    if time.time() >= record['fixed_cutoff'] or keeper.FENCE.exists():
        raise RuntimeError('role-fixed-cutoff-or-host-fence')
    return record, chain


def invoke(role, argv, *, name, timeout, identity_budget, binding=None,
           binding_sha256=None, input_path=None, evidence_cap=131072,
           control_only=False):
    if role not in ROLE_LIMITS or not argv or type(timeout) not in (int, float) or timeout <= 0:
        raise ValueError('explicit-role-command-timeout-required')
    if type(identity_budget) is not int or not 1 <= identity_budget <= 4096:
        raise ValueError('role-identity-budget')
    if type(evidence_cap) is not int or not 4096 <= evidence_cap <= 1024*1024:
        raise ValueError('role-evidence-cap')
    if not name.replace('-', '').isalnum():
        raise ValueError('role-name')
    token, ancestors = ancestor_token()
    binary = Path(argv[0]).resolve(strict=True)
    binary_hash = sha256(binary)
    if control_only:
        if token['source_pin'] != 'CONTROL_ONLY' or binary != Path(sys.executable).resolve():
            raise RuntimeError('role-control-only-exact-python')
        binding_record = {'source_pin': 'CONTROL_ONLY', 'binary_sha256': binary_hash}
    else:
        if binding is None or binding_sha256 is None or sha256(binding) != binding_sha256:
            raise RuntimeError('role-build-binding-hash')
        path = Path(binding)
        if path.is_symlink() or path.stat().st_size > 65536:
            raise RuntimeError('role-build-binding-path-size')
        binding_record = admission.strict_json(path.read_bytes())
        fields = {'source_pin', 'source_binding', 'source_binding_sha256',
                  'guard_sha256', 'binary_path', 'binary_sha256', 'engine_release'}
        if set(binding_record) != fields or binding_record['engine_release'] is not True:
            raise RuntimeError('role-build-binding-shape-release')
        if binding_record['source_pin'] != token['source_pin'] or token['source_pin'] == 'CONTROL_ONLY':
            raise RuntimeError('role-build-source-pin')
        if binding_record['binary_path'] != str(binary) or binding_record['binary_sha256'] != binary_hash:
            raise RuntimeError('role-exact-binary-binding')
        if binding_record['guard_sha256'] != sha256(Path(__file__).with_name('guard_v20.py')):
            raise RuntimeError('role-reviewed-guard-binding')
        source = binding_record['source_binding']
        if sha256(source) != binding_record['source_binding_sha256']:
            raise RuntimeError('role-source-binding-bytes')
        source_row = admission.source_binding(source, token['source_pin'])
        if source_row['guard_sha256'] != binding_record['guard_sha256']:
            raise RuntimeError('role-source-guard-binding')
    request = {k: {'bytes': 0, 'files': 0} for k in admission.LIMITS}
    request['F12'] = {'bytes': evidence_cap+524288, 'files': 2}
    admission.admit(admission.snapshot(shared=False), request, admission.limits())
    folder = admission.ROOT/'evidence/roles'/name
    folder.mkdir(parents=True, exist_ok=False)
    events = Events(folder/'samples.jsonl.gz', evidence_cap)
    known = {}; failure = []; samples = []; stop = threading.Event()
    nonce = token['nonce']+'-role-'+name
    registry = monitor.Registry(nonce)
    myself = identity(os.getpid())
    deadline = min(time.monotonic()+timeout,
                   time.monotonic()+max(0, token['fixed_cutoff']-time.time()))
    inp = subprocess.DEVNULL
    if input_path is not None:
        path = Path(input_path)
        if not path.resolve().is_relative_to(admission.ROOT) or path.is_symlink():
            raise RuntimeError('role-input-path')
        inp = path.open('rb')
    child = None
    def sample():
        previous = None
        while not stop.is_set():
            try:
                now = time.monotonic()
                if previous is not None and now-previous > .04:
                    raise RuntimeError('role-RSS-gap-over-40ms')
                previous = now
                own_i = identity(os.getpid()); own_u = usage(os.getpid())
                if not monitor.same_identity(myself, own_i) or own_u['return'] or not own_u['rss_bytes']:
                    raise RuntimeError('role-driver-RSS-unavailable')
                active = []; pending = list(known.values()); visited = set()
                while pending:
                    expected = pending.pop()
                    if expected['pid'] in visited:
                        continue
                    visited.add(expected['pid'])
                    current = identity(expected['pid'])
                    if current['returned_bytes'] == 0 and current['errno'] in (0, 3):
                        continue
                    if not monitor.same_identity(expected, current):
                        raise RuntimeError('role-owned-identity-unknown-or-escaped')
                    memory = usage(expected['pid'])
                    if memory['return'] or memory['rss_bytes'] is None:
                        raise RuntimeError('role-owned-RSS-unavailable')
                    active.append({'identity': current, 'usage': memory})
                    query = monitor.children(current['pid'])
                    if not query['valid']:
                        raise RuntimeError('role-scoped-descendants-unknown')
                    for pid in query['children']:
                        row = identity(pid)
                        if not monitor.valid_identity(row) or row['ppid'] != current['pid']:
                            raise RuntimeError('role-descendant-identity-unknown')
                        if pid in known and not monitor.same_identity(known[pid], row):
                            raise RuntimeError('role-descendant-identity-reuse')
                        known[pid] = row; pending.append(row)
                total = sum(x['usage']['rss_bytes'] for x in active)
                row = {'kind': 'role-RSS', 'monotonic': now, 'role': role,
                       'role_limit_bytes': ROLE_LIMITS[role], 'owned_RSS_bytes': total,
                       'driver_RSS_bytes': own_u['rss_bytes'], 'processes': active,
                       'retained_identities': len(known)}
                events.emit(row); samples.append((now, total))
                if total > ROLE_LIMITS[role]:
                    raise RuntimeError('explicit-'+role+'-RSS-cap')
                if len(active) > 64 or len(known) > identity_budget:
                    raise RuntimeError('role-live-or-cumulative-process-cap')
                if time.monotonic() >= deadline:
                    raise RuntimeError('role-timeout')
            except BaseException as error:
                failure.append(repr(error)); return
            stop.wait(.01)
    helper_stop = threading.Event()
    def helper_loop():
        while not helper_stop.is_set():
            try:
                own_i = identity(os.getpid()); own_u = usage(os.getpid())
                if not monitor.same_identity(myself, own_i) or own_u['return'] or not own_u['rss_bytes']:
                    raise RuntimeError('role-helper-RSS-unavailable')
                registry.publish([{'identity': own_i, 'usage': own_u}], 'role-driver-'+role)
            except BaseException as error:
                failure.append(repr(error)); return
            helper_stop.wait(.01)
    helper_thread = threading.Thread(target=helper_loop, daemon=True)
    try:
        thread = None
        try:
            # Positive helper sample and ancestor binding precede child launch.
            own_u = usage(os.getpid())
            if own_u['return'] or not own_u['rss_bytes']:
                raise RuntimeError('role-positive-driver-RSS-before-GO')
            registry.publish([{'identity': myself, 'usage': own_u}], 'role-before-GO')
            ancestor_token()
            helper_thread.start()
            child = subprocess.Popen([str(binary), *argv[1:]], stdin=inp, close_fds=True)
            expected = identity(child.pid)
            if not monitor.valid_identity(expected) or expected['ppid'] != os.getpid() or expected['pgid'] != myself['pgid']:
                raise RuntimeError('role-created-child-identity')
            known[child.pid] = expected
            events.emit({'kind': 'role-GO', 'role': role, 'limit': ROLE_LIMITS[role],
                         'binary': str(binary), 'binary_sha256': binary_hash,
                         'binding': binding_record, 'ancestor_nonce': token['nonce'],
                         'ancestors': ancestors, 'child': expected, 'timeout_seconds': timeout})
            thread = threading.Thread(target=sample, daemon=True); thread.start()
            while child.poll() is None and not failure:
                time.sleep(.002)
            if failure:
                raise RuntimeError(failure[0])
            if child.returncode != 0:
                raise RuntimeError('role-child-exit:'+str(child.returncode))
            if not samples or not any(total > 0 for _, total in samples):
                raise RuntimeError('role-positive-owned-RSS-not-observed')
        except BaseException as error:
            failure.append(repr(error))
        finally:
            stop.set()
            if thread is not None: thread.join(.2)
            if thread is not None and thread.is_alive():
                failure.append('role-sampler-shutdown-unknown')
            if failure:
                for expected in reversed(list(known.values())):
                    keeper.exact_signal(expected, signal.SIGKILL)
            if child is not None:
                try: child.wait(timeout=.3)
                except subprocess.TimeoutExpired: failure.append('role-child-reap-unknown')
            closure = []
            for _ in range(2):
                rows = [identity(x['pid']) for x in known.values()]
                closure.append({'monotonic': time.monotonic(), 'identities': rows,
                                'absent': all(x['returned_bytes'] == 0 and x['errno'] in (0, 3) for x in rows)})
                time.sleep(.02)
            if not known or not all(x['absent'] for x in closure):
                failure.append('role-closure-not-proven')
            events.emit({'kind': 'role-close', 'failures': failure, 'closure': closure})
            events.close()
            if inp is not subprocess.DEVNULL: inp.close()
        result = {'role': role, 'limit_bytes': ROLE_LIMITS[role], 'failures': failure,
                  'samples': len(samples), 'max_start_gap_s': max((b[0]-a[0] for a,b in zip(samples,samples[1:])), default=0),
                  'peak_RSS_bytes': max((x[1] for x in samples), default=None),
                  'closure': closure, 'binary_sha256': binary_hash, 'source_pin': token['source_pin']}
        admission.bounded_receipt(folder/'result.json', result)
        if failure: raise RuntimeError('role-invocation-censored:'+repr(failure))
        return result
    finally:
        helper_stop.set()
        if helper_thread.ident is not None: helper_thread.join(.2)
        registry.update(remove=True)
