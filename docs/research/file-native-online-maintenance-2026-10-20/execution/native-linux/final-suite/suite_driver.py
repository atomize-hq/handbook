"""Final-suite native command transport; engine JSON uses immutable Driver v3."""
import base64, copy, fcntl, hashlib, json, os, selectors, signal, subprocess, sys, threading, time, uuid
from pathlib import Path
import native_guard as guard
from native_driver_v3 import Driver as EngineDriver
from suite_bootstrap import need, encoded, digest, owned, strict,retain_birth,observed_children,OwnedBirthUncertain
from native_resources_v3 import confined,regular_read

from suite_recovery import Commands,descendants


def retain_tree(starts,handles,snapshot,lock,stage):
    """Retain the exact ownership-observed rows, never a replacement birth."""
    with lock:
        owner=guard.process(os.getpid(),with_memory=False)
        need(owner is not None,'tree-owner-live')
        kept={}
        for pid,start in list(starts.items()):
            row=snapshot.get(pid)
            if row is None:continue
            if row['start']!=start:raise OwnedBirthUncertain(pid,start,stage,row['start'],'retained-PID-reused')
            retain_birth(starts,handles,row,lock,stage);kept[pid]=row
        parents=[owner,*kept.values()];seen=set()
        while parents:
            parent=parents.pop()
            if parent['pid'] in seen:continue
            seen.add(parent['pid'])
            for row in snapshot.values():
                if row['ppid']!=parent['pid'] or row['pid'] in kept:continue
                retain_birth(starts,handles,row,lock,stage,parent)
                kept[row['pid']]=row;parents.append(row)
        return list(kept.values())

READER_ACTIONS={'resolve','point','inspect','lookup-bench','ordinary'}

class JsonDriver(EngineDriver):
    """Preserve exact wire bytes before core3 stdin delivery, including key order."""
    def __init__(self,executable,output,*,expected_sha256,suite_pin,binary_source_pin,reservations,wire_sink,suite_manifest,historical_policy=None,start_observer=None):
        from suite_adapter import suite_owner
        self.wire_sink=wire_sink
        self.suite_manifest=suite_manifest
        need(__debug__ and sys.platform == 'linux', 'native-linux-unoptimized-python-required')
        self.owner = suite_owner(suite_manifest)
        need(guard.digest(Path(guard.__file__).with_name('native_driver.py')) == 'dfe8ef569b4981a76e199a2763de8a3cf9988bb24785b280fc3662bcd09427ce', 'accepted-owner-source-unchanged')
        need(self.owner['source_pin'] == suite_pin, 'suite-versus-guard')
        self.suite_pin, self.binary_source_pin = (suite_pin, binary_source_pin)
        self.executable = Path(executable)
        need(self.executable.is_absolute() and self.executable.resolve() == self.executable, 'exact-canonical-executable')
        if historical_policy is None:
            need(binary_source_pin == suite_pin, 'candidate-source-pin-equality')
            need(self.executable.is_relative_to(guard.ROOT / 'target') or self.executable.is_relative_to(guard.ROOT / 'source'), 'owned-candidate-binary')
            self.role = 'candidate'
        else:
            need(binary_source_pin != suite_pin, 'historical-source-must-remain-distinct')
            self.role = 'historical'
        self.policy = historical_policy
        self.binary_sha256 = expected_sha256
        self.verify_binary()
        need(regular_read(self.executable, self.executable.stat().st_size)[:4] == b'\x7fELF', 'native-ELF-only')
        self.output = confined(guard.ROOT / 'evidence', Path(output))
        need(self.output.parent.is_dir() and (not self.output.exists()), 'fresh-existing-owned-event-parent')
        self.reservations = reservations
        key = 'event-file-' + uuid.uuid4().hex
        reservations.reserve_evidence(key, 4096, 1)
        with self.output.open('xb'):
            pass
        reservations.settle(key)
        self.lock, self.registry = (threading.RLock(), {})
        self.events, self.last = ([], None)
        self.start_observer = start_observer
        self.guard_sha256=self.owner['guard_sha256']
    def append(self,row,state):
        row=self.source_row(row)
        super().append(row,state)
    def source_row(self,row):
        return dict(row,guard_version=5,guard_sha256=self.guard_sha256,
                 guard_utility_sha256='4ddf6c15c87a200530c2c5e15d6c5be26ff3251019d20f8dea6ca83948b634f9')
    def wait(self,token,timeout):return self.source_row(super().wait(token,timeout))
    def interrupt(self,token,kind):
        with self.lock:
            state=self._state(token)
            maintenance=state['action']=='packing'
            if maintenance:
                need(kind=='death' and not state['closed'] and state['intentional'] in (None,kind),
                     'exact-owned-maintainer-death-only')
                if state['intentional'] is None:
                    self._signal_locked(state,signal.SIGKILL,'registered-maintainer-death')
                    state['intentional']=kind;state['interrupt_ns']=time.perf_counter_ns()
        if maintenance:return self.wait(token,5)
        return super().interrupt(token,kind)
    def call(self,root,action,*,timeout=120,code=0,env=None,on_started=None,**fields):
        need(digest(__file__)==self.guard_sha256,'same-new-guard4-and-wire-source')
        wire=json.dumps(dict(root=str(root),action=action,**fields),separators=(',',':'),allow_nan=False).encode()
        need(len(wire)<=4*1024**2,'same-core-native-input-bound')
        lease='native-wire-'+uuid.uuid4().hex
        self.reservations.reserve_evidence(lease,(4*len(wire)+2)//3+8192,0)
        writing=threading.Event();finished=threading.Event()
        def observed(event):
            writing.set()
            try:
                self.wire_sink(dict(kind='suite-native-input-wire',call=event['control_token'],reservation=lease,
                    bytes_b64=base64.b64encode(wire).decode(),bytes=len(wire),
                    sha256=hashlib.sha256(wire).hexdigest(),before_stdin_delivery=True))
            finally:finished.set()
            if on_started is not None:on_started(event)
        try:
            return super().call(root,action,timeout=timeout,code=code,env=env,on_started=observed,**fields)
        except AssertionError as error:
            if len(error.args)==1 and type(error.args[0]) is dict and error.args[0].get('guard_nonce')==self.owner['nonce']:
                raise AssertionError(self.source_row(error.args[0])) from error
            raise
        finally:
            # If a timed-out spool callback is still writing, retain its lease.
            if not writing.is_set() or finished.is_set():self.reservations.settle(lease)



# SOURCE-PROPOSED guard4. Frozen4dd body/checks retained below; own authority.
# Changes: closed parent-selected96/512/640MiB fixture policy; explicit new source/manifest owner binding;
# old4dd remains only the byte-gated stdlib child exec utility. No global patch.
import argparse,ctypes,gzip,shutil
ROOT=guard.ROOT
CUTOFF=guard.CUTOFF
MIB=1024**2
SUITE_LIMITS={'source':(4*MIB,1500),'target':(256*MIB,5000),'fixtures':(640*MIB,30000),'evidence':(24*MIB,2000),'cargo-private':(64*MIB,4096)}
inventory=guard.inventory
process=guard.process
processes=guard.processes

def _guard_admit(ns):
    from suite_bootstrap import load,ClosedImports
    m=load(ns.policy_manifest_path,ns.policy_manifest_sha256)
    sys.meta_path.insert(0,ClosedImports(m['source_files']))
    for directory in reversed(m['import_directories']):
        if directory not in sys.path:sys.path.insert(0,directory)
    os.environ['FILE_NATIVE_ORACLE_RESEARCH']=m['oracle']['research_root']
    from suite_adapter import suite_limits
    if m.get('runtime_scope')=='interface-controls-v5':
        from suite_extensions import validate_controls
        validate_controls(m)
    elif m.get('runtime_scope') in {'native-build-v5','workspace-v5'}:
        from suite_oracle import native_stage_command
        template=m['native_guard_policy']['command_template']
        need(template==native_stage_command(m),'exact-reviewed-native-Cargo-provider')
    elif m.get('runtime_scope')=='private-cache-v5':
        need(m['private_cache']['enabled'] is True and m['private_cache']['copy_go'] is True,'C1-copy-authority')
    else:
        from suite_main import expand,validate
        m=expand(m);validate(m)
    p=m['native_guard_policy']
    need(all(v[0]<=SUITE_LIMITS[k][0] and v[1]<=SUITE_LIMITS[k][1] for k,v in suite_limits(m).items()),'closed-new-guard4-policy-ceilings')
    need(ns.pin==m['final_pin'] and ns.claim_id==m['claim']['claim_id'] and ns.ledger_sha256==m['claim']['ledger_sha256'],'new-guard-claim-release')
    need(0<ns.seconds<=p['seconds'] and ns.process_limit==2*1024**3 and ns.heavy_limit==2*1024**3 and
         ns.free_floor>=20*1024**3 and ns.cwd==str(ROOT) and ns.stdin is None,'same-memory-time-free-floor-root-policy')
    need(ns.reserve==p['reserve'] and ns.monitor_cargo,'exact-prospective-body-reserves-and-global-cache-monitor')
    need(Path(ns.receipt).parent==Path(m['evidence_root']),'pre-admitted-body-evidence-directory')
    command=ns.command[1:] if ns.command[:1]==['--'] else ns.command
    expected=[x.replace('@MANIFEST_PATH@',m['manifest_path']).replace('@MANIFEST_SHA256@',m['manifest_sha256']) for x in p['command_template']]
    need(command==expected and command,'exact-reviewed-child-launch')
    phase=strict(owned(m['phase']['owner_path'],ROOT/'evidence').read_bytes(),65536)
    parent=process(os.getppid())
    need(phase['kind']=='live-V5-phase-owner' and parent and phase['pid']==parent['pid'] and phase['start']==parent['start'] and
         phase['claim']==m['claim'] and phase['final_pin']==m['final_pin'] and phase['manifest_sha256']==m['manifest_sha256'] and
         phase['deadline_epoch']==m['phase']['deadline_epoch'] and digest(Path('/proc',str(parent['pid']),'cmdline'))==phase['cmdline_sha256'],'actual-live-outer-phase-watchdog')
    return m

def suite_guard_main(manifest_path,manifest_sha256,argv):
    parser=argparse.ArgumentParser()
    for flag in ['receipt','pin','claim-id','ledger-sha256']:parser.add_argument('--'+flag,required=True)
    parser.add_argument('--seconds',type=float,required=True)
    parser.add_argument('--reserve',action='append',default=[])
    parser.add_argument('--cwd');parser.add_argument('--stdin')
    parser.add_argument('--monitor-cargo',action='store_true')
    parser.add_argument('--process-limit',type=int,default=2*1024**3)
    parser.add_argument('--heavy-limit',type=int,default=2*1024**3)
    parser.add_argument('--free-floor',type=int,default=20*1024**3)
    parser.add_argument('command',nargs=argparse.REMAINDER)
    ns=parser.parse_args(argv);ns.policy_manifest_path=manifest_path;ns.policy_manifest_sha256=manifest_sha256
    return suite_guard_run(ns)

def suite_guard_run(ns):
    start = time.time()
    m = _guard_admit(ns)
    from suite_adapter import suite_limits
    limits = suite_limits(m)
    absolute_deadline = min(start + ns.seconds, m['phase']['deadline_epoch'], CUTOFF)
    if sys.platform != 'linux':
        raise RuntimeError('native-linux-required')
    if ROOT.resolve() != ROOT or ROOT.is_symlink():
        raise RuntimeError('runtime-containment')
    ROOT.mkdir(exist_ok=True)
    for name in limits:
        (ROOT / name).mkdir(exist_ok=True)
    receipt = Path(ns.receipt).resolve()
    if not receipt.is_relative_to(ROOT / 'evidence') or receipt.exists():
        raise RuntimeError('new-owned-receipt-required')
    receipt.parent.mkdir(parents=True, exist_ok=True)
    lock = (ROOT / 'heavy-session.lock').open('a+b')
    fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    libc = ctypes.CDLL(None, use_errno=True)
    if libc.prctl(36, 1, 0, 0, 0) != 0:
        raise OSError(ctypes.get_errno(), 'subreaper')
    effective = absolute_deadline - start
    if effective <= 0:
        raise RuntimeError('functional-cutoff')
    before = {k: inventory(ROOT / k) for k in limits}
    reserves = {k: dict(bytes=0, files=0) for k in limits}
    for entry in ns.reserve:
        category, amounts = entry.split('=', 1)
        byte_count, file_count = (int(x) for x in amounts.split(':'))
        if category not in limits or byte_count < 0 or file_count < 0:
            raise RuntimeError('invalid-category-reservation')
        reserves[category] = dict(bytes=byte_count, files=file_count)
    if reserves['evidence']['bytes'] < 65536 or reserves['source']['bytes'] < 1024:
        raise RuntimeError('receipt-and-owner-reservation-required')
    cargo = Path('/home/spenser/.cargo')
    from suite_adapter import cache_identity
    cargo_before = cache_identity(cargo,hashes=False) if ns.monitor_cargo else None
    for key, val in before.items():
        if val['bytes'] + reserves[key]['bytes'] > limits[key][0] or val['files'] + reserves[key]['files'] > limits[key][1]:
            raise RuntimeError('prospective-allocation:' + key)
    if shutil.disk_usage(ROOT).free < ns.free_floor:
        raise RuntimeError('preflight-free-floor')
    env = os.environ.copy()
    env['PATH'] = '/home/spenser/.cargo/bin:' + env.get('PATH', '')
    env['CARGO_INCREMENTAL'] = '0'
    env['CARGO_NET_OFFLINE'] = 'true'
    env['PYTHONDONTWRITEBYTECODE'] = '1'
    nonce = uuid.uuid4().hex
    identity = process(os.getpid())
    owner = dict(nonce=nonce, pid=os.getpid(), process_start=identity['start'], claim_id=ns.claim_id, ledger_sha256=ns.ledger_sha256, source_pin=ns.pin, categories=reserves, before=before, absolute_deadline=absolute_deadline, epoch=time.time(), guard_version=5, runtime_scope=m.get('runtime_scope','full-body'), guard_sha256=digest(__file__), category_limits={k: list(v) for k, v in limits.items()}, policy_manifest_path=ns.policy_manifest_path, policy_manifest_sha256=ns.policy_manifest_sha256, owner_cmdline_sha256=digest(Path('/proc', str(os.getpid()), 'cmdline')))
    owner_path = ROOT / 'source' / ('guard-owner-' + nonce + '.json')
    claim_path = ROOT / 'evidence' / ('claim-' + nonce + '.json')
    owner_path.write_text(json.dumps(owner, separators=(',', ':')) + '\n')
    claim_path.write_text(json.dumps(owner, separators=(',', ':')) + '\n')
    if time.time() >= absolute_deadline:
        raise RuntimeError('setup-exhausted-deadline-before-spawn')
    read_fd, write_fd = os.pipe()
    output = receipt.with_suffix('.stdout').open('wb')
    errors = receipt.with_suffix('.stderr').open('wb')
    stdin = open(ns.stdin, 'rb') if ns.stdin else subprocess.DEVNULL
    command = ns.command[1:] if ns.command[:1] == ['--'] else ns.command
    if not command:
        raise RuntimeError('command-required')
    state_lock=threading.RLock()
    p = subprocess.Popen([sys.executable, str(Path(guard.__file__).resolve()), '--child', str(read_fd), *command], cwd=ns.cwd, env=env, stdin=stdin, stdout=output, stderr=errors, start_new_session=True, pass_fds=(read_fd,))
    os.close(read_fd)
    initial = process(p.pid)
    if initial is None or initial['state'] == 'Z' or initial['rss'] is None or (initial['hwm'] is None) or (initial['rss'] <= 0):
        if initial is not None:
            starts={};handles={}
            retain_birth(starts,handles,initial,state_lock,'blocked-root-refusal')
            try:signal.pidfd_send_signal(handles[p.pid],signal.SIGKILL)
            finally:os.close(handles[p.pid])
        p.wait(timeout=5)
        raise RuntimeError('blocked-launch-positive-rss-required')
    tracked = {}
    active = {p.pid}
    registered_supervisors = {pid: row['start'] for pid, row in processes().items() if pid not in (os.getpid(), p.pid) and str(ROOT) in row['cmd']}
    pidfds = {}
    retain_birth(tracked,pidfds,initial,state_lock,'blocked-root')
    stop = threading.Event()
    reasons = []
    termination = []
    memory_path = receipt.with_suffix('.memory.jsonl.gz')
    memory_stream = gzip.open(memory_path, 'wt', encoding='utf-8')
    root_exec = None
    memory_count = 0
    aggregate_peak = supervisor_peak = 0
    disk_rows = []
    sample_errors = []
    process_peaks = {}
    intervals = []
    last_sample = None

    def terminate(reason):
        with state_lock:
            if reason not in reasons:
                reasons.append(reason)
            handles = list(pidfds.items())
            termination.append(dict(epoch=time.time(), reason=reason, tracked_pids=[pid for pid, _ in handles]))
            for _, fd in handles:
                try:
                    signal.pidfd_send_signal(fd, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                except BaseException as error:
                    sample_errors.append('retained-signal:'+repr(error))

    def members(rows):
        return retain_tree(tracked,pidfds,rows,state_lock,'tree-scan')

    def owned_snapshot():
        with state_lock:
            owner=process(os.getpid(),with_memory=False)
            need(owner is not None,'rss-owner-live')
            pending=[]
            for pid,start in list(tracked.items()):
                row=process(pid)
                if row is None:active.discard(pid);continue
                if row['start']!=start:raise OwnedBirthUncertain(pid,start,'rss-sampler',row['start'],'retained-PID-reused')
                pending.append((row,None))
            pending.extend((row,owner) for row in observed_children(owner))
            seen=set();rows=[]
            while pending:
                row,parent=pending.pop();pid=row['pid']
                if pid in seen:continue
                seen.add(pid)
                retain_birth(tracked,pidfds,row,state_lock,'rss-sampler',parent)
                active.add(pid);rows.append(row)
                if row['state']!='Z':pending.extend((child,row) for child in observed_children(row))
            return rows

    def sample_memory():
        nonlocal last_sample, memory_count, aggregate_peak, supervisor_peak, root_exec
        due = time.monotonic()
        while not stop.is_set():
            begin = time.monotonic()
            try:
                owned = owned_snapshot()
                live = [x for x in owned if x['state'] != 'Z']
                if any((x['rss'] is None or x['hwm'] is None for x in live)):
                    raise RuntimeError('missing-live-owned-memory')
                if last_sample is not None:
                    intervals.append(begin - last_sample)
                last_sample = begin
                supervisors = {os.getpid()}
                for pid, expected in registered_supervisors.items():
                    supervisor = process(pid)
                    if supervisor is not None and supervisor['start'] == expected:
                        supervisors.add(pid)
                parent = os.getppid()
                while parent > 1:
                    supervisors.add(parent)
                    ancestor = process(parent)
                    if ancestor is None:
                        break
                    parent = ancestor['ppid']
                overhead_rows = [detail for pid in supervisors if (detail := process(pid)) is not None and detail['state'] != 'Z']
                if any((x['rss'] is None or x['hwm'] is None for x in overhead_rows)):
                    raise RuntimeError('missing-live-supervisor-memory')
                if m['phase']['mode'] in {'build','workspace'} and root_exec is None:
                    root_row=next((x for x in live if x['pid']==p.pid),None)
                    if root_row:
                        ep=Path('/proc',str(p.pid),'exe')
                        try:
                            if str(ep.resolve())==m['cargo']:
                                st=ep.stat();need(digest(ep)==m['tools'][m['cargo']]['sha256'],'actual-Cargo-root-hash')
                                root_exec=dict(pid=p.pid,start=root_row['start'],rss=root_row['rss'],inode=st.st_ino,sha256=digest(ep),epoch=time.time())
                        except FileNotFoundError:pass
                heavy = sum((x['rss'] for x in live))
                overhead = sum((x['rss'] for x in overhead_rows))
                for row in owned:
                    if row['hwm'] is not None:
                        process_peaks[str(row['pid'])] = max(process_peaks.get(str(row['pid']), 0), row['hwm'])
                memory_stream.write(json.dumps(dict(epoch=time.time(), heavy_rss=heavy, supervisor_rss=overhead, zombie_pids=[x['pid'] for x in owned if x['state'] == 'Z'], absent_tracked_pids=[pid for pid in tracked if pid not in {x['pid'] for x in owned}], members=[dict(pid=x['pid'], start=x['start'], rss=x['rss'], hwm=x['hwm'], pgid=x['pgid']) for x in owned]), separators=(',', ':')) + '\n')
                memory_count += 1
                aggregate_peak = max(aggregate_peak, heavy)
                supervisor_peak = max(supervisor_peak, overhead)
                if heavy > ns.heavy_limit or any((x['rss'] > ns.process_limit for x in live)):
                    terminate('heavy-or-process-rss')
                    return
                if any((x['rss'] > 256 * MIB and x['cmd'].split(' ', 1)[0].endswith('/file-native-incremental') for x in live)):
                    terminate('engine-reader-conservative-256mib')
                    return
                if overhead > 768 * MIB or heavy + overhead >= 3 * 1024 ** 3:
                    terminate('supervisor-or-host-rss')
                    return
            except BaseException as e:
                sample_errors.append(repr(e))
                terminate('memory-observation-error')
                return
            due += 0.02
            stop.wait(max(0, due - time.monotonic()))

    def sample_disk():
        while not stop.is_set():
            try:
                now = {k: inventory(ROOT / k) for k in limits}
                free = shutil.disk_usage(ROOT).free
                row = dict(epoch=time.time(), scopes=now, free_bytes=free)
                if ns.monitor_cargo:
                    current_cargo = cache_identity(cargo,hashes=False)
                    row['cargo_identity_sha256'] = current_cargo
                    if current_cargo != cargo_before:
                        terminate('shared-cargo-growth')
                        return
                disk_rows.append(row)
                for key, value in now.items():
                    if value['bytes'] > limits[key][0] or value['files'] > limits[key][1]:
                        terminate('allocation:' + key)
                        return
                    if value['bytes'] > before[key]['bytes'] + reserves[key]['bytes'] or value['files'] > before[key]['files'] + reserves[key]['files']:
                        terminate('reservation-overrun:' + key)
                        return
                if free < ns.free_floor:
                    terminate('free-floor')
                    return
            except BaseException as e:
                sample_errors.append(repr(e))
                terminate('disk-observation-error')
                return
            stop.wait(0.25)
    timer = threading.Timer(max(0, absolute_deadline - time.time()), terminate, args=('time-cutoff',))
    timer.daemon = True
    sampler = threading.Thread(target=sample_memory, daemon=True)
    disk = threading.Thread(target=sample_disk, daemon=True)
    timer.start()
    sampler.start()
    disk.start()
    closure = []
    execute_started = None
    monitors_joined = False
    try:
        execute_started = time.time()
        if execute_started >= absolute_deadline:
            terminate('setup-exhausted-deadline-before-exec')
        elif not reasons:
            os.write(write_fd, b'G')
        os.close(write_fd)
        while p.poll() is None and (not reasons):
            time.sleep(0.01)
        if reasons:
            p.wait(timeout=10)
        else:
            p.wait(timeout=10)
        live = members(processes())
        if any((x['state'] != 'Z' for x in live)):
            terminate('root-exit-left-descendants')
        until = time.monotonic() + 10
        while time.monotonic() < until:
            for pid in list(tracked):
                if pid == p.pid:
                    continue
                try:
                    os.waitpid(pid, os.WNOHANG)
                except ChildProcessError:
                    pass
            remaining = members(processes())
            closure.append(dict(epoch=time.time(), pids=[x['pid'] for x in remaining], states=[x['state'] for x in remaining]))
            if len(closure) >= 2 and (not closure[-1]['pids']) and (not closure[-2]['pids']):
                break
            time.sleep(0.05)
    finally:
        abnormal=sys.exc_info()[0] is not None
        def cleanup(label,action):
            try:return action()
            except BaseException as error:
                sample_errors.append(label+':'+repr(error))
                return None
        cleanup('guard-final-retained-kill',lambda:terminate('supervisor-finally') if p.poll() is None or sample_errors or abnormal or len(closure)<2 or any(r['pids'] for r in closure[-2:]) else None)
        cleanup('guard-root-reap',lambda:p.wait(timeout=10))
        stop.set()
        cleanup('guard-timer-cancel',timer.cancel)
        for thread in (timer,sampler,disk):cleanup('guard-monitor-join',lambda thread=thread:thread.join(timeout=10))
        monitors_joined=not any(t.is_alive() for t in (timer,sampler,disk))
        cleanup('guard-memory-close',memory_stream.close)
        cleanup('guard-output-close',output.close)
        cleanup('guard-errors-close',errors.close)
        if ns.stdin:cleanup('guard-input-close',stdin.close)
        for fd in list(pidfds.values()):cleanup('guard-pidfd-close',lambda fd=fd:os.close(fd))
        if not monitors_joined:os._exit(2)
    positive = len(closure) >= 2 and (not closure[-1]['pids']) and (not closure[-2]['pids'])
    cadence_qualified = bool(intervals) and (not any((x > 0.04 for x in intervals)))
    row = dict(root_exec=root_exec,command=command, cwd=ns.cwd, source_pin=ns.pin, guard_sha256=digest(__file__), started_epoch=start, absolute_deadline_epoch=absolute_deadline, execute_started_epoch=execute_started, setup_seconds=execute_started - start if execute_started else None, execute_and_shutdown_seconds=time.time() - execute_started if execute_started else None, blocked_launch_positive_rss=initial['rss'], monitors_joined=monitors_joined, owner_nonce=nonce, owner_pid=os.getpid(), owner_process_start=identity['start'], registered_supervisor_identities=registered_supervisors, claim_id=ns.claim_id, ledger_sha256=ns.ledger_sha256, prospective_reservation=reserves, claim_path=str(claim_path), effective_cap_seconds=effective, seconds=time.time() - start, exit=p.returncode, censors=reasons, termination=termination, group_id=p.pid, tracked_identity=tracked, process_closure_verified=positive, closure_observations=closure, memory_interval_target_seconds=0.02, memory_max_interval_seconds=max(intervals, default=None), memory_intervals_over_40ms=sum((x > 0.04 for x in intervals)), rss_sampling_cadence_qualified=cadence_qualified, memory_samples_path=str(memory_path), memory_samples_sha256=digest(memory_path), memory_sample_count=memory_count, aggregate_heavy_peak_bytes=aggregate_peak, aggregate_supervisor_peak_bytes=supervisor_peak, process_high_water_bytes=process_peaks, heavy_rss_limit=ns.heavy_limit, process_rss_limit=ns.process_limit, supervisor_rss_limit=768 * MIB, native_host_rss_limit=3 * 1024 ** 3, free_floor_bytes=ns.free_floor, disk_samples=disk_rows, baseline=before, final={k: inventory(ROOT / k) for k in limits}, observation_errors=sample_errors, stdout_sha256=digest(receipt.with_suffix('.stdout')), stderr_sha256=digest(receipt.with_suffix('.stderr')), guard_version=5, runtime_scope=m.get('runtime_scope','full-body'), category_limits={k: list(v) for k, v in limits.items()}, policy_manifest_path=ns.policy_manifest_path, policy_manifest_sha256=ns.policy_manifest_sha256, guard_utility_sha256='4ddf6c15c87a200530c2c5e15d6c5be26ff3251019d20f8dea6ca83948b634f9')
    from suite_adapter import inclusive_receipt
    row=inclusive_receipt(m,receipt,row,before,reserves)
    print(json.dumps({k: row[k] for k in ('exit', 'censors', 'seconds', 'process_closure_verified', 'memory_max_interval_seconds', 'memory_intervals_over_40ms')}))
    return 0 if p.returncode == 0 and (not reasons) and positive and (not sample_errors) and cadence_qualified else 1

def verify_guard_policy(m,packet,g):
    policy=m['native_guard_policy']
    need(g['guard_version']==5 and g['guard_sha256']==policy['source_sha256'] and
         g['policy_manifest_path']==packet['manifest_path'] and g['policy_manifest_sha256']==packet['manifest_sha256'],
         'new-guard4-source-and-policy-manifest')
    from suite_adapter import suite_limits
    need(g['category_limits']=={k:list(v) for k,v in suite_limits(m).items()},'no-implicit-limit-override')
    claim=strict(owned(g['claim_path'],ROOT/'evidence').read_bytes(),8192)
    need(claim['guard_version']==5 and claim['guard_sha256']==g['guard_sha256'] and
         claim['nonce']==g['owner_nonce'] and claim['process_start']==g['owner_process_start'] and
         claim['categories']==g['prospective_reservation'],'new-guard4-retained-owner-claim')


def phase_terminal(receipt,manifest_sha,identity,covered,deadline):
    """No substantive file work follows the observer handoff."""
    import suite_bootstrap as early
    receipt_sha=digest(receipt)
    need(early.EARLY_THREAD.is_alive(),'caller-observer-live-through-final-receipt')
    early.EARLY_STOP.set();early.EARLY_THREAD.join(.04)
    need(not early.EARLY_THREAD.is_alive(),'caller-observer-terminal-join')
    rows=early.EARLY_ROWS
    i=next((i for i,r in enumerate(rows) if r['epoch']>=covered),len(rows))
    tail=rows[max(0,i-1):]
    endpoint=early.caller_sample();tail.append(endpoint)
    need(2<=len(tail)<=256 and tail[0]['epoch']<=covered and tail[-1]['epoch']>=covered,'bounded-caller-tail-overlap')
    for a,b in zip(tail,tail[1:]):need(0<=b['epoch']-a['epoch']<=.04,'caller-tail-cadence')
    need(all(r['caller_pid']==identity['pid'] and r['caller_start']==identity['start'] for r in tail),'same-caller-birth-through-tail')
    value=dict(kind='V5_PHASE_TERMINAL',receipt_sha256=receipt_sha,manifest_sha256=manifest_sha,
        caller_pid=identity['pid'],caller_start=identity['start'],deadline_epoch=deadline,
        substantive_end_epoch=endpoint['epoch'],covered_epoch=covered,observer_joined=True,
        samples=[[r['epoch'],r['caller_rss'],r['caller_hwm']] for r in tail])
    raw=encoded(value)+b'\n';need(len(raw)<=15360,'terminal-witness-byte-cap')
    def emit(block,prior):
        need(len(block)<=15360 and time.time()<deadline,'terminal-output-bounds')
        # Small writes, post-write synchronous RSS/HWM and cadence checks. A
        # blocked write or failed final check cannot finish with exit code zero.
        while block:
            part=block[:4096];n=os.write(1,part);need(n>0,'terminal-output-progress');block=block[n:]
            row=early.caller_sample();need(0<=row['epoch']-prior<=.04 and row['epoch']<deadline,'terminal-output-cadence')
            prior=row['epoch']
        return prior
    end=emit(raw,endpoint['epoch'])
    terminal=early.caller_sample()
    need(0<=terminal['epoch']-end<=.04,'terminal-final-sample-cadence')
    final=encoded(dict(kind='V5_PHASE_EXIT',witness_sha256=hashlib.sha256(raw).hexdigest(),
        epoch=terminal['epoch'],rss=terminal['caller_rss'],hwm=terminal['caller_hwm']))+b'\n'
    need(len(final)<=512,'constant-terminal-exit-record')
    emit(final,terminal['epoch'])
    os._exit(0)

def verify_phase_terminal(binding,receipt):
    """Parent-sealed actual exit plus exact two retained stdout records."""
    need(binding['reviewed'] is True and binding['returncode']==0,'parent-observed-successful-phase-exit')
    path=owned(binding['path'],guard.ROOT/'evidence')
    need(path.stat().st_size<=16384 and digest(path)==binding['sha256'],'terminal-witness-file-bound')
    raw=path.read_bytes();lines=raw.splitlines(keepends=True)
    need(len(lines)==2 and all(line.endswith(b'\n') for line in lines),'exact-two-terminal-records')
    a,b=[strict(line,16384) for line in lines]
    need(receipt['kind']=='V5_PHASE_FINAL' and receipt['status']=='PENDING_TERMINAL' and
        a['kind']=='V5_PHASE_TERMINAL' and b['kind']=='V5_PHASE_EXIT' and
        a['receipt_sha256']==binding['receipt_sha256'] and a['manifest_sha256']==receipt['manifest_sha256'] and
        b['witness_sha256']==hashlib.sha256(lines[0]).hexdigest(),'source-bound-terminal-receipt')
    need(a['caller_pid']==receipt['phase_caller']['pid'] and a['caller_start']==receipt['phase_caller']['start'] and
        a['deadline_epoch']==receipt['deadline_epoch'] and a['observer_joined'] is True and
        a['covered_epoch']==receipt['phase_memory_proof']['last_epoch'],'terminal-caller-memory-overlap')
    rows=a['samples'];need(2<=len(rows)<=256 and rows[0][0]<=a['covered_epoch']<=rows[-1][0] and
        rows[-1][0]==a['substantive_end_epoch'],'bounded-terminal-tail-endpoints')
    rows=rows+[[b['epoch'],b['rss'],b['hwm']]]
    for row in rows:need(len(row)==3 and all(type(x) in (int,float) for x in row) and
        0<row[1]<=row[2]<=768*1024**2 and row[0]<receipt['deadline_epoch'],'terminal-positive-caller-memory')
    for x,y in zip(rows,rows[1:]):need(0<=y[0]-x[0]<=.04,'terminal-memory-cadence')
    return dict(receipt, status='PASSED',terminal_witness_sha256=binding['sha256'])
