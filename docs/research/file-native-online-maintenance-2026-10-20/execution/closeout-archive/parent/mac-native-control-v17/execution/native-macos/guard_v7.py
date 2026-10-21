"""Bounded native macOS guard replacement. No engine release is bundled.
All new executable work is foreground/cooperative; daemonizers are refused.
"""
import argparse
import datetime as dt
import fcntl
import gzip
import gc
import hashlib
import json
import os
from pathlib import Path
import select
import secrets
import signal
import socket
import subprocess
import sys
import threading
import time
import zlib
import admission_v7 as admission
import keeper_v7 as keeper
import monitor_v7 as monitor
from native_probe import identity, usage, group

ROOT = admission.ROOT
FIXED_CUTOFF = dt.datetime.fromisoformat('2026-10-21T11:10:14+00:00').timestamp()


class Events:
    def __init__(self, path, cap):
        self.file = path.open('xb')
        self.cap = cap
        self.rows = 0
        self.lock = threading.Lock()
        self.compressor = zlib.compressobj(level=6,wbits=31)

    def emit(self, row):
        raw = (json.dumps(row,separators=(',',':'))+'\n').encode()
        if len(raw) > 262144:
            raise RuntimeError('sample-row-byte-cap')
        with self.lock:
            candidate = self.compressor.copy()
            encoded = candidate.compress(raw)+candidate.flush(zlib.Z_SYNC_FLUSH)
            # Reserve footer space, and do not advance stream state on refusal.
            if self.file.tell()+len(encoded)+32 > self.cap:
                raise RuntimeError('RSS-evidence-prewrite-cap')
            self.file.write(encoded)
            self.file.flush()
            self.compressor = candidate
            self.rows += 1

    def close(self):
        with self.lock:
            tail = self.compressor.flush(zlib.Z_FINISH)
            if self.file.tell()+len(tail) > self.cap:
                raise RuntimeError('RSS-evidence-finalize-cap')
            self.file.write(tail)
            self.file.close()


def wait_reply(sock, key, nonce, timeout=.2):
    row, fds = keeper.receive(sock, timeout)
    if fds or row is None or row.get(key) != nonce:
        raise RuntimeError('keeper-handshake-failed:'+key)
    return row


def anchor_main(fd, nonce, command, cwd, input_path, token_fd, owner):
    control = socket.socket(fileno=fd)
    keeper.send(control, {'anchor_ready':nonce,'pid':os.getpid()})
    row, fds = keeper.receive(control, 5)
    if row != {'go':nonce} or fds:
        return 90
    inp = open(input_path,'rb') if input_path else subprocess.DEVNULL
    try:
        child = subprocess.Popen(command,cwd=cwd,stdin=inp,close_fds=True)
        keeper.send(control, {'started':nonce,'pid':child.pid})
        while child.poll() is None:
            observed_owner = identity(owner['pid'])
            if not monitor.same_identity(owner,observed_owner) or observed_owner.get('state') == 5 or time.time() >= FIXED_CUTOFF:
                # Independent keeper death: the trusted anchor remains an
                # ancestor of every workload child and retains the token FD.
                myself = identity(os.getpid())
                tree = monitor.OwnedTree(myself)
                until = time.monotonic()+1.5
                stable = 0
                previous = None
                while time.monotonic() < until:
                    observed_tree = tree.observe()
                    others = [x for x in observed_tree['identities'] if x['pid'] != os.getpid()]
                    signature = sorted((x['pid'],tuple(x['start'])) for x in others)
                    frozen = all(x['state'] in (4,5) for x in others)
                    stable = stable+1 if frozen and signature == previous else 0
                    previous = signature
                    if stable >= 2:
                        break
                    for expected in others:
                        keeper.exact_signal(expected,signal.SIGSTOP)
                    time.sleep(.01)
                if stable >= 2:
                    for expected in others:
                        keeper.exact_signal(expected,signal.SIGKILL)
                    try:
                        child.wait(timeout=.3)
                    except subprocess.TimeoutExpired:
                        pass
                observations = []
                for _ in range(2):
                    observations.append(group(os.getpid()))
                    time.sleep(.02)
                if stable >= 2 and len(observations) >= 2 and all(x['valid'] and x['members'] == [os.getpid()] for x in observations):
                    return 98
                keeper.durable_fence(nonce,owner,myself,myself,tree.known,'anchor-backup-UNKNOWN',ROOT/('anchor-'+nonce+'.release.json'))
                # No false closure or token release if backup cleanup is unknown.
                while True:
                    time.sleep(.02)
            time.sleep(.005)
        result = child.returncode
        os.close(1)
        os.close(2)
        keeper.send(control, {'done':nonce,'exit':result})
        while time.time() < FIXED_CUTOFF:
            row, fds = keeper.receive(control,.01)
            if row == {'release':nonce} and not fds:
                return 0
            observed_owner=identity(owner['pid'])
            if not monitor.same_identity(owner,observed_owner) or observed_owner.get('state')==5:
                return 98
        return 91
    finally:
        if input_path:
            inp.close()
        control.close()


class RemoteAnchor:
    def __init__(self, expected, output, error):
        self.expected = expected
        self.pid = expected['pid']
        self.stdout = os.fdopen(output,'rb',buffering=0)
        self.stderr = os.fdopen(error,'rb',buffering=0)
        self.returncode = None

    def poll(self):
        observed = identity(self.pid)
        if observed['returned_bytes'] == 0 and observed['errno'] in (0,3):
            self.returncode = 'REAPED_BY_KEEPER'
        return self.returncode

    def wait(self, timeout):
        until = time.monotonic()+timeout
        while self.poll() is None:
            if time.monotonic() >= until:
                raise subprocess.TimeoutExpired('remote-owned-anchor',timeout)
            time.sleep(.002)
        return self.returncode


def proof_after_reap(process, anchor, sampler):
    try:
        process.wait(timeout=2)
    except subprocess.TimeoutExpired:
        pass
    known = dict(sampler.tree.known) if sampler and sampler.tree else {anchor['pid']:anchor}
    proof = keeper.closure(anchor,known)
    return proof, known


def guardian_still_alive(pid, expected):
    # A child may disappear from libproc just before waitpid reports its exit.
    # Resolve that bounded kernel transition without inferring absence from RSS.
    until = time.monotonic()+.05
    while True:
        got,code = os.waitpid(pid,os.WNOHANG)
        if got:
            return False,code
        observed = identity(pid)
        if monitor.same_identity(expected,observed) and observed.get('state') != 5:
            return True,None
        if time.monotonic() >= until:
            return False,None
        time.sleep(.001)


def run(ns):
    sys.setswitchinterval(.001)
    gc.disable()  # bounded per-cell guard; avoid cyclic-GC pauses in the RSS sampler
    started_epoch = time.time()
    started_mono = time.monotonic()
    fixed = min(FIXED_CUTOFF, ns.test_fixed_epoch or FIXED_CUTOFF)
    local_deadline = started_mono+ns.seconds
    if started_epoch >= fixed:
        raise RuntimeError('fixed-cutoff-before-setup')
    if keeper.FENCE.exists():
        return {'status':'REFUSED_HOST_FENCE','fence':admission.strict_json(keeper.FENCE.read_bytes()),
                'censor':'durable-host-fence','closure':'NO_NEW_WORKLOAD','token_released':False}
    requested = admission.prospective(ns.prospective)
    cap = admission.limits(ns.reduced_limits)
    if requested['F01']['bytes'] < 131072 or requested['F01']['files'] < 8:
        raise ValueError('guard-source-control-reservation-too-small')
    required_evidence = ns.output_cap+ns.rss_evidence_cap+2*524288+65536
    if requested['F12']['bytes'] < required_evidence or requested['F12']['files'] < 8:
        raise ValueError('guard-evidence-reservation-too-small')
    # This quick per-category check precedes every new receipt/control file.
    light = admission.snapshot(shared=False)
    admission.admit(light,requested,cap,ns.free_floor)
    parent = identity(os.getpid())
    memory = usage(os.getpid())
    if not monitor.valid_identity(parent) or memory['return'] != 0 or not memory['rss_bytes']:
        raise RuntimeError('positive-supervisor-RSS-before-setup')
    nonce = secrets.token_hex(16)
    outroot = ROOT/'evidence/control'/ns.name
    outroot.mkdir(exist_ok=False)
    record = {'kind':'native-guard-v7','nonce':nonce,'status':'CONTROL_ONLY' if ns.control_only else 'PINNED_WORKLOAD',
              'source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
              'adapter_source_bindings':{p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in Path(__file__).parent.glob('*v7.py')},
              'source_pin':'CONTROL_ONLY' if ns.control_only else ns.source_pin,
              'claims':admission.CLAIMS,'limits':cap,'prospective':requested,
              'started_epoch':started_epoch,'functional_cutoff_epoch':FIXED_CUTOFF,
              'effective_fixed_cutoff_epoch':fixed,'local_deadline_monotonic':local_deadline,
              'global_enumeration':False,'censor':None,'closure':'NO_WORKLOAD_CREATED',
              'token_released':False,'worker_exit':None,'supervisor':parent,
              'before_setup_positive_RSS':memory,'observed_output_bytes':0,
              'workload_GO':False,'closure_proofs':[],'termination':[]}
    triggered = []
    previous_signal = signal.signal(signal.SIGUSR1,lambda *_:triggered.append('keeper-alert'))
    a,b = socket.socketpair(socket.AF_UNIX,socket.SOCK_DGRAM)
    watch = os.fork()
    if watch == 0:
        a.close()
        os.setsid()
        null = os.open('/dev/null',os.O_RDWR)
        for fd in (0,1,2):
            os.dup2(null,fd)
        if null > 2:
            os.close(null)
        try:
            keeper.run(b,nonce,parent,local_deadline,fixed,outroot)
        finally:
            os._exit(0)
    b.close()
    expected_keeper = identity(watch)
    record['keeper_identity'] = expected_keeper
    token = None
    acquired = False
    process = anchor = control = sampler = None
    stdout = stderr = None
    events = Events(outroot/'samples.jsonl.gz',ns.rss_evidence_cap)
    disk_stop = threading.Event()
    disk_thread = None
    shared_thread = None
    disk = {'latest':None,'failure':None}
    watch_reaped = False
    hold = False
    started_child = None
    completed_message = None
    eof_streams = set()
    source_row = None
    output_files = []
    try:
        hello = wait_reply(a,'ready',nonce)
        expected_keeper = hello['keeper']
        if expected_keeper['pid'] != watch or not monitor.valid_identity(expected_keeper):
            raise RuntimeError('keeper-session-handshake')
        record['keeper_identity'] = expected_keeper
        record['keeper_recovery_socket'] = hello['recovery_socket']
        def heartbeat(fields):
            keeper.send(a,{'nonce':nonce,**fields})
        sampler = monitor.Sampler(nonce,[parent,expected_keeper],events.emit,heartbeat,ns.limit)
        sampler.thread.start()
        sampler.before_go()  # positive helpers during setup and token waits
        token = (ROOT/'mac-heavy.lock').open('a+b')
        while True:
            if keeper.FENCE.exists():
                raise RuntimeError('host-fence-during-token-wait')
            if sampler.failure or triggered:
                raise RuntimeError(sampler.failure or 'keeper-alert-during-token-wait')
            if time.time() >= fixed or time.monotonic() >= local_deadline:
                raise RuntimeError('deadline-during-token-wait')
            try:
                fcntl.flock(token.fileno(),fcntl.LOCK_EX|fcntl.LOCK_NB)
                acquired = True
                break
            except BlockingIOError:
                time.sleep(.005)
        if keeper.FENCE.exists():
            raise RuntimeError('host-fence-after-token-acquisition')
        keeper.send(a,{'nonce':nonce,'token':True},token.fileno())
        wait_reply(a,'token_ack',nonce)
        record['token_acquired_epoch'] = time.time()
        token.seek(0)
        token.truncate()
        token.write(json.dumps({'nonce':nonce,'parent':parent,'keeper':expected_keeper,
                                'source_pin':record['source_pin'],'claims':admission.CLAIMS,
                                'fixed_cutoff':fixed},separators=(',',':')).encode())
        token.flush()
        os.fsync(token.fileno())
        if ns.setup_delay:
            until = time.monotonic()+ns.setup_delay
            while time.monotonic() < until:
                if sampler.failure or triggered or time.time() >= fixed or time.monotonic() >= local_deadline:
                    raise RuntimeError('deadline-or-sampler-during-setup-wait')
                time.sleep(.005)
        actual = admission.snapshot(shared=True)
        admission.admit(actual,requested,cap,ns.free_floor)
        record['admission_snapshot'] = actual
        ceiling = {k:{'bytes':v['logical_bytes']+requested[k]['bytes'],
                      'files':v['files']+requested[k]['files']} for k,v in actual['categories'].items()}
        record['admitted_growth_ceiling'] = ceiling
        if not ns.control_only:
            source_row = admission.source_binding(ns.source_binding,ns.source_pin)
            if source_row.get('guard_sha256') != record['source_sha256']:
                raise RuntimeError('guard-review-release-SHA-mismatch')
        else:
            executable = str(Path(ns.command[0]).resolve())
            if executable != str(Path(sys.executable).resolve()):
                raise RuntimeError('control-only-python-launcher-required')
        cwd = Path(ns.cwd or ROOT).resolve()
        if not cwd.is_relative_to(ROOT):
            raise RuntimeError('protected-cwd-refusal')
        record['launcher'] = {'path':str(Path(ns.command[0]).resolve()),
                              'sha256':hashlib.sha256(Path(ns.command[0]).resolve().read_bytes()).hexdigest(),
                              'argv':ns.command,'cwd':str(cwd),
                              'contract':'foreground, no daemonization or session escape; all subprocesses sampled'}
        def monitor_disk():
            last_shared = None
            while not disk_stop.is_set():
                try:
                    now = time.monotonic()
                    shared = False
                    observed = admission.snapshot(shared=shared,stop=disk_stop)
                    if shared:
                        last_shared = now
                    if observed['free_bytes'] < ns.free_floor:
                        raise RuntimeError('live-free-floor')
                    for k,v in observed['categories'].items():
                        if v['logical_bytes'] > ceiling[k]['bytes'] or v['files'] > ceiling[k]['files']:
                            raise RuntimeError('live-admitted-growth-cap:'+k)
                        if v['logical_bytes'] > cap[k]['bytes'] or v['files'] > cap[k]['files']:
                            raise RuntimeError('live-category-hard-cap:'+k)
                    disk['latest'] = {'monotonic':time.monotonic(),'snapshot':observed,'shared_checked':shared}
                    events.emit({'kind':'disk',**disk['latest']})
                except InterruptedError:
                    if disk_stop.is_set():
                        return
                    raise
                except BaseException as error:
                    disk['failure'] = repr(error)
                    try:
                        heartbeat({'failure':disk['failure']})
                    except BaseException:
                        pass
                    return
                disk_stop.wait(.05)
        def monitor_shared_growth():
            while not disk_stop.wait(2):
                try:
                    observed = admission.snapshot(shared=True,stop=disk_stop)
                    v = observed['categories']['F13']
                    if v['logical_bytes'] > ceiling['F13']['bytes'] or v['files'] > ceiling['F13']['files']:
                        raise RuntimeError('live-admitted-shared-growth-cap:F13')
                    if v['logical_bytes'] > cap['F13']['bytes'] or v['files'] > cap['F13']['files']:
                        raise RuntimeError('live-shared-hard-cap:F13')
                    disk['shared_latest'] = {'monotonic':time.monotonic(),'F13':v}
                    events.emit({'kind':'shared-growth',**disk['shared_latest']})
                except InterruptedError:
                    if disk_stop.is_set():
                        return
                    raise
                except BaseException as error:
                    disk['failure'] = repr(error)
                    try:
                        heartbeat({'failure':disk['failure']})
                    except BaseException:
                        pass
                    return
        disk_thread = threading.Thread(target=monitor_disk,daemon=True)
        shared_thread = threading.Thread(target=monitor_shared_growth,daemon=True)
        disk_thread.start()
        shared_thread.start()
        sampler.before_go()
        if triggered:
            raise RuntimeError('keeper-alert-before-anchor')
        # The initial full category census is admission. Live census runs in a
        # separate thread so it never replaces or blocks the RSS sampler.
        control,child_control = socket.socketpair(socket.AF_UNIX,socket.SOCK_DGRAM)
        environment = dict(os.environ,PYTHONDONTWRITEBYTECODE='1',
                           CARGO_HOME=str(ROOT/'cargo-private'),CARGO_TARGET_DIR=str(ROOT/'target'),
                           CARGO_BUILD_JOBS='2',TMPDIR=str(ROOT/'fixtures'))
        output_read,output_write = os.pipe()
        error_read,error_write = os.pipe()
        keeper.send(a,{'nonce':nonce,'launch':{'python':sys.executable,'guard':str(Path(__file__).resolve()),
                          'cwd':str(cwd),'input':ns.input or '', 'command':ns.command,'env_overrides':{k:environment[k] for k in ('PYTHONDONTWRITEBYTECODE','CARGO_HOME','CARGO_TARGET_DIR','CARGO_BUILD_JOBS','TMPDIR')}}},
                    [child_control.fileno(),output_write,error_write])
        launched = wait_reply(a,'launched',nonce,.3)
        os.close(output_write)
        os.close(error_write)
        process = RemoteAnchor(launched['anchor'],output_read,error_read)
        child_control.close()
        hello,_ = keeper.receive(control,.2)
        if not hello or hello.get('anchor_ready') != nonce or hello.get('pid') != process.pid:
            raise RuntimeError('anchor-nonce-handshake')
        anchor = identity(process.pid)
        if not monitor.valid_identity(anchor) or anchor['ppid'] != watch or anchor['pgid'] != process.pid:
            raise RuntimeError('anchor-identity-mismatch')
        record['anchor'] = anchor
        heartbeat({'anchor':anchor})
        wait_reply(a,'anchor_ack',nonce)
        sampler.attach(anchor)
        # Wait for a complete helper+anchor sample, not a stale helper-only row.
        until = time.monotonic()+.2
        while time.monotonic() < until:
            if sampler.failure:
                raise RuntimeError(sampler.failure)
            if sampler.last and sampler.last.get('tree') is not None:
                break
            time.sleep(.001)
        record['before_GO_positive_RSS'] = sampler.before_go()
        stdout = (outroot/'worker.stdout').open('xb')
        stderr = (outroot/'worker.stderr').open('xb')
        output_files = [stdout,stderr]
        for stream in (process.stdout,process.stderr):
            os.set_blocking(stream.fileno(),False)
        keeper.send(control,{'go':nonce})
        record['workload_GO'] = True
        while True:
            if sampler.failure:
                raise RuntimeError(sampler.failure)
            if disk['failure']:
                raise RuntimeError(disk['failure'])
            if triggered or time.time() >= fixed or time.monotonic() >= local_deadline:
                raise RuntimeError('independent-or-absolute-deadline')
            alive, exitcode = guardian_still_alive(watch,expected_keeper)
            if not alive:
                watch_reaped = exitcode is not None
                raise RuntimeError('independent-watchdog-died')
            if ns.test_fault == 'sample-gap' and sampler.count >= 8:
                # Explicit fault: pause only sampling, never reset any deadline.
                sampler.stop_event.set()
            if ns.test_fault == 'missing-RSS' and sampler.count >= 8:
                # A controlled unavailable owned-RSS observation; stored as a
                # declared injection, not presented as a native API failure.
                monitor.usage = lambda pid:{'return':-1,'errno':1,'rss_bytes':None,'start_abstime':None}
            for incoming,out in ((process.stdout,stdout),(process.stderr,stderr)):
                try:
                    data = os.read(incoming.fileno(),4096)
                except BlockingIOError:
                    data = None
                if data == b'':
                    eof_streams.add(incoming.fileno())
                if data:
                    if record['observed_output_bytes']+len(data) > ns.output_cap:
                        record['output_rejected_chunk_bytes'] = len(data)
                        raise RuntimeError('output-prewrite-cap')
                    out.write(data)
                    out.flush()
                    record['observed_output_bytes'] += len(data)
            row,_ = keeper.receive(control,0)
            if row and row.get('started') == nonce:
                started_child = row['pid']
                record['workload_child_pid'] = started_child
                if ns.test_fault == 'watchdog-death':
                    keeper.exact_signal(expected_keeper,signal.SIGKILL)
                    os.waitpid(watch,0)
                    watch_reaped = True
                    raise RuntimeError('injected-watchdog-death-after-GO')
            elif row and row.get('done') == nonce:
                completed_message = row
                record['worker_exit'] = row['exit']
                with sampler.tree_lock:
                    observed = sampler.tree.observe()
                if any(x['pid'] != anchor['pid'] for x in observed['identities']):
                    raise RuntimeError('root-completed-orphan-tree')
            if completed_message and len(eof_streams) == 2:
                keeper.send(control,{'release':nonce})
                process.wait(timeout=2)
                break
            time.sleep(.002)
    except BaseException as error:
        record['censor'] = repr(error)
        if sampler and sampler.failure:
            record['primary_sample_failure'] = sampler.failure
    finally:
        record['sample_count'] = sampler.count if sampler else 0
        record['sample_gap_max_s'] = sampler.max_gap if sampler else None
        record['sample_failure'] = sampler.failure if sampler else None
        record['peak_group_rss'] = sampler.peak_group if sampler else None
        record['peak_all_helpers_rss'] = sampler.peak_helpers if sampler else None
        if sampler and sampler.tree:
            record['last_tree_trace'] = sampler.tree.observations
        if anchor:
            known = dict(sampler.tree.known) if sampler and sampler.tree else {anchor['pid']:anchor}
            if process.poll() is None:
                try:
                    keeper.send(a,{'nonce':nonce,'failure':record['censor'] or 'facade-workload-cleanup'})
                except OSError as error:
                    record['cleanup_request_error'] = repr(error)
            proof,latest = proof_after_reap(process,anchor,sampler)
            known.update(latest)
            record['closure_proofs'].append(proof)
            hold = not proof['proven'] or ns.test_fault == 'force-closure-HOLD'
            record['closure'] = 'UNKNOWN_HOLD' if hold else 'PROVEN_EMPTY_TWICE'
            if hold:
                keeper.durable_fence(nonce,parent,expected_keeper,anchor,known,
                                      'parent-UNKNOWN-closure',Path(record['keeper_recovery_socket']))
        disk_stop.set()
        if disk_thread:
            disk_thread.join(timeout=.2)
            if disk_thread.is_alive():
                record['censor'] = record['censor'] or 'disk-monitor-not-closed'
                hold = bool(anchor)
        record['disk_monitor'] = disk
        if shared_thread:
            shared_thread.join(timeout=.2)
            if shared_thread.is_alive():
                record['censor'] = record['censor'] or 'shared-monitor-not-closed'
                hold = bool(anchor)
        if sampler:
            try:
                sampler.stop()
            except BaseException as error:
                record['sampler_shutdown_error'] = repr(error)
                hold = bool(anchor)
        if not watch_reaped:
            alive, code = guardian_still_alive(watch,expected_keeper)
            watch_reaped = code is not None
            if alive:
                try:
                    keeper.send(a,{'nonce':nonce,'hold':True} if hold else {'nonce':nonce,'stop':True})
                    if not hold:
                        wait_reply(a,'closed',nonce,.5)
                        os.waitpid(watch,0)
                        watch_reaped = True
                except BaseException as error:
                    record['keeper_shutdown_error'] = repr(error)
                    alive, code = guardian_still_alive(watch,expected_keeper)
                    watch_reaped = code is not None
                    if alive:
                        hold = bool(anchor)
            elif not watch_reaped:
                hold = bool(anchor)
        # If the independent deadline already closed the keeper, this parent's
        # two positive numeric observations still authorize safe token release.
        if hold and not watch_reaped:
            until = time.monotonic()+.5
            while time.monotonic() < until and not keeper.FENCE.exists():
                alive, code = guardian_still_alive(watch,expected_keeper)
                if not alive:
                    watch_reaped = code is not None
                    break
                time.sleep(.005)
        if hold:
            record['token_released'] = False
            record['ownership_HOLD'] = True
            record['keeper_recovery_socket'] = str(ROOT/('keeper-'+nonce+'.release.json'))
            # The keeper already owns the duplicate token FD and never closes
            # it on UNKNOWN. This parent's descriptor can close only after that
            # independently durable fence + retained owner have been recorded.
            if not keeper.FENCE.exists() and anchor:
                keeper.durable_fence(nonce,parent,expected_keeper,anchor,known,
                                      'parent-shutdown-UNKNOWN',Path(record['keeper_recovery_socket']))
            if (watch_reaped or not monitor.same_identity(expected_keeper,identity(watch))) and token:
                # Last-resort parent retention: a dead keeper cannot retain an
                # FD. Stay in this explicitly fenced owner until reviewed release.
                recovery_path = ROOT/('parent-keeper-'+nonce+'.release.json')
                keeper.durable_fence(nonce,parent,parent,anchor,known,'parent-fallback-HOLD',recovery_path)
                while True:
                    admission.bounded_receipt(outroot/'result.json',record)
                    time.sleep(.02)
                    row = admission.strict_json(recovery_path.read_bytes()) if recovery_path.exists() and not recovery_path.is_symlink() and recovery_path.stat().st_size <= 4096 else None
                    if row == {'nonce':nonce,'reviewed_release_after_positive_closure':True}:
                        proof = keeper.closure(anchor,known)
                        if proof['proven']:
                            keeper.FENCE.unlink()
                            break
                recovery_path.unlink(missing_ok=True)
        else:
            record['token_released'] = True
            if sampler:
                sampler.registry.update(remove=True)
        if token:
            token.close()
        a.close()
        if control:
            control.close()
        for f in output_files:
            f.close()
        if process:
            process.stdout.close()
            process.stderr.close()
        events.close()
        signal.signal(signal.SIGUSR1,previous_signal)
        record['keeper_reaped'] = watch_reaped
        record['raw_rows_written'] = events.rows
        record['finished_epoch'] = time.time()
        record['test_fault'] = ns.test_fault
        record['status'] = 'GUARD_CONTROL_CANDIDATE_NO_ENGINE_RELEASE'
        admission.bounded_receipt(outroot/'result.json',record)
    return record


def arguments():
    p = argparse.ArgumentParser()
    p.add_argument('--name',required=True)
    p.add_argument('--seconds',type=float,default=15)
    p.add_argument('--control-only',action='store_true')
    p.add_argument('--source-pin')
    p.add_argument('--source-binding')
    p.add_argument('--cwd')
    p.add_argument('--input')
    p.add_argument('--limit',type=int,default=2*1024**3)
    p.add_argument('--free-floor',type=int,default=20*1024**3)
    p.add_argument('--output-cap',type=int,default=8192)
    p.add_argument('--rss-evidence-cap',type=int,default=131072)
    p.add_argument('--prospective',type=admission.strict_json,
                   default={k:{'bytes':131072 if k=='F01' else 2*1024**2 if k=='F12' else 0,
                               'files':8 if k in ('F01','F12') else 0} for k in admission.LIMITS})
    p.add_argument('--reduced-limits',type=admission.strict_json,default={})
    p.add_argument('--test-fixed-epoch',type=float)
    p.add_argument('--setup-delay',type=float,default=0)
    p.add_argument('--test-fault',choices=['sample-gap','missing-RSS','watchdog-death','force-closure-HOLD'])
    p.add_argument('command',nargs=argparse.REMAINDER)
    ns = p.parse_args()
    if ns.command[:1] == ['--']:
        ns.command = ns.command[1:]
    if not ns.command or not ns.name.replace('-','').isalnum():
        p.error('name/command required')
    if (ns.test_fault or ns.test_fixed_epoch or ns.reduced_limits) and not ns.control_only:
        p.error('test injection requires control-only')
    return ns

if __name__ == '__main__':
    if len(sys.argv)>1 and sys.argv[1] == '--anchor':
        raise SystemExit(anchor_main(int(sys.argv[2]),sys.argv[3],sys.argv[8:],sys.argv[4],sys.argv[5],int(sys.argv[6]),admission.strict_json(sys.argv[7])))
    row = run(arguments())
    print(json.dumps({k:row.get(k) for k in ('censor','closure','token_released','sample_count','sample_gap_max_s','workload_GO','worker_exit','keeper_recovery_socket','nonce')}))
    raise SystemExit(0 if row.get('censor') is None and row.get('closure')=='PROVEN_EMPTY_TWICE' and row.get('worker_exit')==0 else 1)
