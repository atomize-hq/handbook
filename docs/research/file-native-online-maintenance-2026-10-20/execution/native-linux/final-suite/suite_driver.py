"""Final-suite native command transport; engine JSON uses immutable Driver v3."""
import base64, copy, fcntl, hashlib, json, os, selectors, signal, subprocess, sys, threading, time, uuid
from pathlib import Path
import native_guard as guard
from native_driver_v3 import Driver as EngineDriver
from suite_bootstrap import need, encoded, digest, owned, strict
from native_resources_v3 import confined,regular_read

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
        return dict(row,guard_version=4,guard_sha256=self.guard_sha256,
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

def descendants(pid):
    answer=set()
    try:
        for task in Path('/proc',str(pid),'task').iterdir():
            try: answer.update(map(int,(task/'children').read_text().split()))
            except FileNotFoundError: pass
    except FileNotFoundError: pass
    return answer

class Commands:
    def __init__(self,host):
        self.host=host;self.lock=threading.RLock();self.active={}
    def close(self):
        errors=[]
        with self.lock:
            for state in self.active.values():
                try:self._kill(state,'case-close')
                except BaseException as e:errors.append(repr(e))
        for state in list(self.active.values()):
            if not state['done'].wait(5):errors.append('command-owner-not-joined')
        need(not errors,'native-command-close:'+repr(errors))
    def _kill(self,state,reason):
        # Only our retained pidfds are authority; no reopened PID or foreign group.
        for pid,(start,fd) in list(state['owned'].items()):
            row=guard.process(pid,with_memory=False)
            if row is not None and row['start']==start and row['state']!='Z':
                try:signal.pidfd_send_signal(fd,signal.SIGKILL)
                except ProcessLookupError:pass
        state['kill_reasons'].append(reason)
    def run(self,root,action,argv,stdin=b'',*,expected=0,seconds=120,env=None):
        need(type(stdin) is bytes,'command-bytes-input')
        self.host.command_binding(action,argv,stdin,env)
        root=owned(root,guard.ROOT/'fixtures')
        deadline=min(time.time()+seconds,self.host.driver.owner['absolute_deadline'],guard.CUTOFF)
        need(deadline>time.time(),'native-command-fixed-deadline')
        limit=256*1024**2 if action in READER_ACTIONS else 2*1024**3
        executable=Path(argv[0]);binary_sha=digest(executable)
        token=uuid.uuid4().hex;gate_r,gate_w=os.pipe()
        state=dict(owned={},done=threading.Event(),kill_reasons=[],errors=[],pid=None)
        stop=threading.Event();watcher=None;p=None;rawout=bytearray();rawerr=bytearray()
        maximum=0;observations=[];sent=0;full=False;input_closed=False;selection=selectors.DefaultSelector()
        lease=None;wire_lease=None;failure=None;record=None;initial=None;exec_proof=None;pipe_fds=[]
        try:
            # Allocate exact output geometry and reserve before creating a child.
            out_r,out_w=os.pipe();pipe_fds.extend([out_r,out_w])
            err_r,err_w=os.pipe();pipe_fds.extend([err_r,err_w])
            tail=sum(fcntl.fcntl(fd,fcntl.F_GETPIPE_SZ) for fd in (out_w,err_w))
            payload=dict(root=str(root),action=action,operation=action,argv=argv,stdin_bytes=len(stdin),
                         stdin_sha256=hashlib.sha256(stdin).hexdigest())
            lease=self.host.budget.reserve_call(token,payload,env,tail)
            need(len(stdin)<=lease['input_bytes'],'complete-command-stdin-forecast')
            wire_bound=(4*(len(stdin)+lease['stdout_bytes']+lease['stderr_bytes']+tail)+2)//3+8192
            self.host.budget.reserve_evidence('command-wire-'+token,wire_bound,0)
            wire_lease='command-wire-'+token
            p=subprocess.Popen([sys.executable,str(Path(guard.__file__).resolve()),'--child',str(gate_r),*argv],
                               stdin=subprocess.PIPE,stdout=out_w,stderr=err_w,
                               pass_fds=(gate_r,),close_fds=True,env=self.host.environment(env))
            os.close(out_w);pipe_fds.remove(out_w);os.close(err_w);pipe_fds.remove(err_w)
            p.stdout=os.fdopen(out_r,'rb',buffering=0);pipe_fds.remove(out_r)
            p.stderr=os.fdopen(err_r,'rb',buffering=0);pipe_fds.remove(err_r)
            os.close(gate_r);gate_r=None
            initial=guard.process(p.pid)
            need(initial and initial['rss'] and initial['hwm'] and initial['pgid']==os.getpgrp(),'blocked-native-command-positive-rss')
            state['pid']=p.pid;state['owned'][p.pid]=(initial['start'],os.pidfd_open(p.pid))
            with self.lock:self.active[token]=state
            def memory():
                nonlocal maximum
                try:
                    while not stop.is_set():
                        with self.lock:
                            pending=list(state['owned']);seen=set()
                            while pending:
                                pid=pending.pop()
                                if pid in seen:continue
                                seen.add(pid);pending.extend(descendants(pid))
                                row=guard.process(pid)
                                if row is None or row['state']=='Z':continue
                                need(row['pgid']==os.getpgrp(),'command-child-left-owned-group')
                                if pid not in state['owned']:state['owned'][pid]=(row['start'],os.pidfd_open(pid))
                                need(state['owned'][pid][0]==row['start'] and row['rss'] is not None and row['hwm'] is not None,'command-live-rss-identity')
                                maximum=max(maximum,row['hwm'])
                                observations.append(dict(epoch=time.time(),pid=pid,start=row['start'],rss=row['rss'],hwm=row['hwm']))
                                if row['rss']>limit:state['errors'].append('command-role-rss-censor');self._kill(state,'rss')
                            if time.time()>=deadline:state['errors'].append('command-time-censor');self._kill(state,'deadline')
                        stop.wait(.01)
                except BaseException as e:
                    state['errors'].append('command-monitor:'+repr(e))
                    with self.lock:self._kill(state,'monitor-error')
            watcher=threading.Thread(target=memory,daemon=False);watcher.start()
            os.write(gate_w,b'G');os.close(gate_w);gate_w=None
            # Bash/Python stdin scripts cannot begin file I/O before this proof.
            while True:
                need(time.time()<deadline,'actual-command-exec-deadline')
                row=guard.process(p.pid);need(row and row['start']==initial['start'] and row['state']!='Z','command-exited-before-native-exec-proof')
                try:actual=Path(os.readlink('/proc/'+str(p.pid)+'/exe'))
                except FileNotFoundError:actual=None
                if actual==executable.resolve():
                    live_exe=Path('/proc',str(p.pid),'exe');live_stat=live_exe.stat();named_stat=executable.stat()
                    need(row['rss'] and row['rss']<=limit and digest(live_exe)==binary_sha==digest(executable) and
                         (live_stat.st_dev,live_stat.st_ino)==(named_stat.st_dev,named_stat.st_ino),'actual-command-exec-rss-sha-inode')
                    exec_proof=dict(path=str(actual),sha256=binary_sha,device=live_stat.st_dev,inode=live_stat.st_ino,
                                    rss=row['rss'],hwm=row['hwm'],before_stdin_delivery=True)
                    break
                time.sleep(.001)
            started_ns=time.perf_counter_ns();self.host.command_started(token,p.pid,initial['start'],action,started_ns)
            for name,f in [('stdin',p.stdin),('stdout',p.stdout),('stderr',p.stderr)]:
                os.set_blocking(f.fileno(),False)
                selection.register(f,selectors.EVENT_WRITE if name=='stdin' else selectors.EVENT_READ,name)
            kill_at=None
            while selection.get_map():
                if state['errors']:
                    kill_at=kill_at or time.monotonic()
                    need(time.monotonic()-kill_at<5,'command-pipe-closure-censor')
                ready=selection.select(.01);ready.sort(key=lambda x:x[0].data=='stdin')
                for item,_ in ready:
                    f,name=item.fileobj,item.data
                    if name=='stdin':
                        try:sent+=os.write(f.fileno(),stdin[sent:sent+65536])
                        except BrokenPipeError:
                            state['errors'].append('command-incomplete-input');selection.unregister(f);f.close();input_closed=True
                            continue
                        if sent==len(stdin):
                            selection.unregister(f);f.close();input_closed=True;full=True
                    else:
                        try:block=os.read(f.fileno(),16384)
                        except BlockingIOError:continue
                        if not block:selection.unregister(f);f.close();continue
                        target=rawout if name=='stdout' else rawerr;target.extend(block)
                        if len(target)>lease[name+'_bytes']:
                            state['errors'].append('command-output-censor:'+name)
                            with self.lock:self._kill(state,'output')
                        need(len(rawout)+len(rawerr)<=lease['stdout_bytes']+lease['stderr_bytes']+tail,'command-tail-reservation-exceeded')
            p.wait(timeout=5)
            need(full or not stdin,'command-full-input-required')
        except BaseException as e:
            failure=repr(e);state['errors'].append(failure)
        finally:
            if p is not None:
                with self.lock:self._kill(state,'owner-finally')
                try:p.wait(timeout=5)
                except subprocess.TimeoutExpired:state['errors'].append('command-root-not-reaped')
            stop.set()
            if watcher is not None:
                watcher.join(5)
                if watcher.is_alive():state['errors'].append('command-monitor-not-joined')
            selection.close()
            for fd in (gate_r,gate_w):
                if fd is not None:os.close(fd)
            for fd in pipe_fds:os.close(fd)
            if p is not None:
                # Drain setup-failure tails in this owner only. A missing EOF is
                # an explicit raw-capture failure, never a truncated success.
                for f,target in [(p.stdout,rawout),(p.stderr,rawerr)]:
                    if f.closed:continue
                    os.set_blocking(f.fileno(),False)
                    while True:
                        try:b=os.read(f.fileno(),16384)
                        except BlockingIOError:state['errors'].append('command-raw-eof-not-observed');break
                        if not b:break
                        target.extend(b)
                    f.close()
                if p.stdin and not p.stdin.closed:p.stdin.close()
            with self.lock:
                if watcher is None or not watcher.is_alive():
                    for start,fd in state['owned'].values():os.close(fd)
                record=dict(kind='native-command',token=token,root=str(root),action=action,argv=argv,
                    env=env,binary_sha256=binary_sha,pid=state['pid'],start=initial['start'] if initial else None,
                    initial_blocked_rss=initial['rss'] if initial else None,actual_exec=exec_proof,
                    stdin_bytes=len(stdin),input_bytes_sent=sent,input_fully_delivered=full,stdin_closed=input_closed,
                    stdin_sha256=hashlib.sha256(stdin).hexdigest(),stdin_bytes_b64=base64.b64encode(stdin).decode(),
                    stdout=rawout.decode(errors='replace'),stderr=rawerr.decode(errors='replace'),
                    stdout_bytes_b64=base64.b64encode(rawout).decode(),stderr_bytes_b64=base64.b64encode(rawerr).decode(),
                    stdout_sha256=hashlib.sha256(rawout).hexdigest(),stderr_sha256=hashlib.sha256(rawerr).hexdigest(),
                    exit=p.returncode if p else None,errors=state['errors'],memory=observations,peak_rss=maximum,
                    rss_limit=limit,started_ns=locals().get('started_ns'),done_ns=time.perf_counter_ns(),
                    root_reaped=p is not None and p.returncode is not None,
                    watcher_joined=watcher is None or not watcher.is_alive(),outer_guard_closure_still_required=True)
            try:
                self.host.emit(record)
                if lease is not None:self.host.budget.settle(token)
                if wire_lease is not None:self.host.budget.settle(wire_lease)
            finally:state['done'].set()
        need(record['exit']==expected and not state['errors'],'native-command-failed:'+repr(record))
        return record

# SOURCE-PROPOSED guard4. Frozen4dd body/checks retained below; own authority.
# Changes: closed parent-selected96/512/640MiB fixture policy; explicit new source/manifest owner binding;
# old4dd remains only the byte-gated stdlib child exec utility. No global patch.
import argparse,ctypes,gzip,shutil
ROOT=guard.ROOT
CUTOFF=guard.CUTOFF
MIB=1024**2
SUITE_LIMITS={'source':(4*MIB,1500),'target':(256*MIB,5000),'fixtures':(640*MIB,30000),'evidence':(24*MIB,2000)}
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
    if m.get('runtime_scope')=='interface-controls-v4':
        from suite_extensions import validate_controls
        validate_controls(m)
    else:
        from suite_main import expand,validate
        m=expand(m);validate(m)
    p=m['native_guard_policy']
    need(all(v[0]<=SUITE_LIMITS[k][0] and v[1]==SUITE_LIMITS[k][1] for k,v in suite_limits(m).items()),'closed-new-guard4-policy-ceilings')
    need(ns.pin==m['final_pin'] and ns.claim_id==m['claim']['claim_id'] and ns.ledger_sha256==m['claim']['ledger_sha256'],'new-guard-claim-release')
    need(0<ns.seconds<=p['seconds'] and ns.process_limit==2*1024**3 and ns.heavy_limit==2*1024**3 and
         ns.free_floor>=20*1024**3 and ns.cwd==str(ROOT) and ns.stdin is None,'same-memory-time-free-floor-root-policy')
    need(ns.reserve==p['reserve'] and ns.monitor_cargo,'exact-prospective-body-reserves-and-global-cache-monitor')
    need(Path(ns.receipt).parent==Path(m['evidence_root']),'pre-admitted-body-evidence-directory')
    command=ns.command[1:] if ns.command[:1]==['--'] else ns.command
    expected=[x.replace('@MANIFEST_PATH@',m['manifest_path']).replace('@MANIFEST_SHA256@',m['manifest_sha256']) for x in p['command_template']]
    need(command==expected and command,'exact-reviewed-child-launch')
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
    m = _guard_admit(ns)
    from suite_adapter import suite_limits
    limits = suite_limits(m)
    start = time.time()
    absolute_deadline = min(start + ns.seconds, CUTOFF)
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
    cargo_before = inventory(cargo, allow_links=True) if ns.monitor_cargo else None
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
    owner = dict(nonce=nonce, pid=os.getpid(), process_start=identity['start'], claim_id=ns.claim_id, ledger_sha256=ns.ledger_sha256, source_pin=ns.pin, categories=reserves, before=before, absolute_deadline=absolute_deadline, epoch=time.time(), guard_version=4, runtime_scope=m.get('runtime_scope','full-body'), guard_sha256=digest(__file__), category_limits={k: list(v) for k, v in limits.items()}, policy_manifest_path=ns.policy_manifest_path, policy_manifest_sha256=ns.policy_manifest_sha256, owner_cmdline_sha256=digest(Path('/proc', str(os.getpid()), 'cmdline')))
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
    p = subprocess.Popen([sys.executable, str(Path(guard.__file__).resolve()), '--child', str(read_fd), *command], cwd=ns.cwd, env=env, stdin=stdin, stdout=output, stderr=errors, start_new_session=True, pass_fds=(read_fd,))
    os.close(read_fd)
    initial = process(p.pid)
    if initial is None or initial['state'] == 'Z' or initial['rss'] is None or (initial['hwm'] is None) or (initial['rss'] <= 0):
        os.killpg(p.pid, signal.SIGKILL)
        p.wait(timeout=5)
        raise RuntimeError('blocked-launch-positive-rss-required')
    tracked = {p.pid: initial['start']}
    active = {p.pid}
    registered_supervisors = {pid: row['start'] for pid, row in processes().items() if pid not in (os.getpid(), p.pid) and str(ROOT) in row['cmd']}
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
            termination.append(dict(epoch=time.time(), reason=reason, tracked_pids=[pid for pid, _ in handles]))
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
        selected = {pid for pid, row in rows.items() if row['pgid'] == p.pid or tracked.get(pid) == row['start']}
        while True:
            added = {pid for pid, row in rows.items() if row['ppid'] in selected or (row['ppid'] == os.getpid() and pid != p.pid)}
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

    def children(pid):
        """All threads' children, including spawns from harness reader threads."""
        found = set()
        try:
            tasks = list(Path(f'/proc/{pid}/task').iterdir())
        except FileNotFoundError:
            return found
        for task in tasks:
            try:
                found.update((int(x) for x in (task / 'children').read_text().split()))
            except FileNotFoundError:
                pass
        return found

    def owned_snapshot():
        with state_lock:
            pending = set(active)
        pending |= children(os.getpid())
        seen = set()
        rows = []
        while pending:
            pid = pending.pop()
            if pid in seen or pid == os.getpid():
                continue
            seen.add(pid)
            row = process(pid)
            if row is None:
                with state_lock:
                    active.discard(pid)
                continue
            if pid in tracked and tracked[pid] != row['start']:
                raise RuntimeError('owned-pid-identity-reused')
            with state_lock:
                if pid not in tracked:
                    tracked[pid] = row['start']
                    try:
                        pidfds[pid] = os.pidfd_open(pid)
                    except ProcessLookupError:
                        pass
                active.add(pid)
            rows.append(row)
            pending |= children(pid) - seen
        return rows

    def sample_memory():
        nonlocal last_sample, memory_count, aggregate_peak, supervisor_peak
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
                    current_cargo = inventory(cargo, allow_links=True)
                    row['cargo_growth'] = max(0, current_cargo['bytes'] - cargo_before['bytes'])
                    if row['cargo_growth'] > 4 * MIB:
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
        if p.poll() is None:
            terminate('supervisor-finally')
            p.wait(timeout=10)
        stop.set()
        timer.cancel()
        timer.join(timeout=10)
        sampler.join(timeout=10)
        disk.join(timeout=10)
        monitors_joined = not any((t.is_alive() for t in (timer, sampler, disk)))
        if not monitors_joined:
            emergency = dict(status='failed-monitor-shutdown', source_pin=ns.pin, nonce=nonce, process_closure_observations=closure, timer_alive=timer.is_alive(), sampler_alive=sampler.is_alive(), disk_alive=disk.is_alive(), censors=reasons, process_closure_verified=False)
            receipt.write_text(json.dumps(emergency) + '\n')
            print(json.dumps(emergency), flush=True)
            os._exit(2)
        memory_stream.close()
        output.close()
        errors.close()
        if ns.stdin:
            stdin.close()
        for fd in pidfds.values():
            os.close(fd)
    positive = len(closure) >= 2 and (not closure[-1]['pids']) and (not closure[-2]['pids'])
    cadence_qualified = bool(intervals) and (not any((x > 0.04 for x in intervals)))
    row = dict(command=command, cwd=ns.cwd, source_pin=ns.pin, guard_sha256=digest(__file__), started_epoch=start, absolute_deadline_epoch=absolute_deadline, execute_started_epoch=execute_started, setup_seconds=execute_started - start if execute_started else None, execute_and_shutdown_seconds=time.time() - execute_started if execute_started else None, blocked_launch_positive_rss=initial['rss'], monitors_joined=monitors_joined, owner_nonce=nonce, owner_pid=os.getpid(), owner_process_start=identity['start'], registered_supervisor_identities=registered_supervisors, claim_id=ns.claim_id, ledger_sha256=ns.ledger_sha256, prospective_reservation=reserves, claim_path=str(claim_path), effective_cap_seconds=effective, seconds=time.time() - start, exit=p.returncode, censors=reasons, termination=termination, group_id=p.pid, tracked_identity=tracked, process_closure_verified=positive, closure_observations=closure, memory_interval_target_seconds=0.02, memory_max_interval_seconds=max(intervals, default=None), memory_intervals_over_40ms=sum((x > 0.04 for x in intervals)), rss_sampling_cadence_qualified=cadence_qualified, memory_samples_path=str(memory_path), memory_samples_sha256=digest(memory_path), memory_sample_count=memory_count, aggregate_heavy_peak_bytes=aggregate_peak, aggregate_supervisor_peak_bytes=supervisor_peak, process_high_water_bytes=process_peaks, heavy_rss_limit=ns.heavy_limit, process_rss_limit=ns.process_limit, supervisor_rss_limit=768 * MIB, native_host_rss_limit=3 * 1024 ** 3, free_floor_bytes=ns.free_floor, disk_samples=disk_rows, baseline=before, final={k: inventory(ROOT / k) for k in limits}, observation_errors=sample_errors, stdout_sha256=digest(receipt.with_suffix('.stdout')), stderr_sha256=digest(receipt.with_suffix('.stderr')), guard_version=4, runtime_scope=m.get('runtime_scope','full-body'), category_limits={k: list(v) for k, v in limits.items()}, policy_manifest_path=ns.policy_manifest_path, policy_manifest_sha256=ns.policy_manifest_sha256, guard_utility_sha256='4ddf6c15c87a200530c2c5e15d6c5be26ff3251019d20f8dea6ca83948b634f9')
    receipt.write_text(json.dumps(row, separators=(',', ':')) + '\n')
    print(json.dumps({k: row[k] for k in ('exit', 'censors', 'seconds', 'process_closure_verified', 'memory_max_interval_seconds', 'memory_intervals_over_40ms')}))
    return 0 if p.returncode == 0 and (not reasons) and positive and (not sample_errors) and cadence_qualified else 1

def verify_guard_policy(m,packet,g):
    policy=m['native_guard_policy']
    need(g['guard_version']==4 and g['guard_sha256']==policy['source_sha256'] and
         g['policy_manifest_path']==packet['manifest_path'] and g['policy_manifest_sha256']==packet['manifest_sha256'],
         'new-guard4-source-and-policy-manifest')
    from suite_adapter import suite_limits
    need(g['category_limits']=={k:list(v) for k,v in suite_limits(m).items()},'no-implicit-limit-override')
    claim=strict(owned(g['claim_path'],ROOT/'evidence').read_bytes(),8192)
    need(claim['guard_version']==4 and claim['guard_sha256']==g['guard_sha256'] and
         claim['nonce']==g['owner_nonce'] and claim['process_start']==g['owner_process_start'] and
         claim['categories']==g['prospective_reservation'],'new-guard4-retained-owner-claim')
