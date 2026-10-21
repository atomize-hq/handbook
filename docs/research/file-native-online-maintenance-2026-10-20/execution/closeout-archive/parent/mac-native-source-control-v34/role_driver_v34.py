"""Foreground reader/checker invocations inside one admitted native guard.
Version 34 source candidate adds withheld stdin, native-exec proof, and owned handles.
Explicit role selects memory bound; executable names never select policy.
Engine use requires the parent-reviewed source/build binding. Control mode only
launches the exact current Python executable and requires CONTROL_ONLY token.
"""
import fcntl
import math
import hashlib
import json
import os
from pathlib import Path
import signal
import selectors
import subprocess
import sys
import threading
import time
import admission_v20 as admission
import monitor_v20 as monitor
import keeper_v20 as keeper
from native_probe import identity, usage
from guard_v20 import Events
from native_start_gate_v31 import prove_actual_exec
import producer_binding_v32 as producer

ROLE_LIMITS = {'reader': 256*1024**2, 'checker': 2*1024**3}


class InterruptTicket:
    def __init__(self,owner,command):self._owner=owner;self._command=command
    def wait(self,timeout=None):
        if threading.current_thread() is self._owner._thread:raise RuntimeError('interrupt-ticket-cannot-wait-in-start-callback')
        left=max(0,self._owner._deadline-time.monotonic())
        if timeout is not None:left=min(left,max(0,timeout))
        if not self._command['ack'].wait(left):raise TimeoutError('owned-interrupt-not-acknowledged')
        if self._command.get('error') is not None:raise self._command['error']
        return self._command['result']


class OwnedCall:
    """Opaque per-call owner. No public PID, Popen, or unscoped signal API.
    All interrupt work is consumed by the invocation's sampler/IO owner thread.
    The final shared harness facade is intentionally separate from this object.
    """
    def __init__(self,role,argv,*,phase=None,**options):
        if type(options.get('timeout')) not in (int,float) or not math.isfinite(options['timeout']) or options['timeout']<=0:raise ValueError('owned-finite-positive-timeout')
        if phase is None:raise ValueError('owned-phase-lease-required')
        phase.auth();self._phase=phase
        self.call_id=options['name'];self._role=role;self._argv=list(argv);self._options=dict(options,phase=phase)
        self._done=threading.Event();self._created=threading.Event();self._native_closed=threading.Event();self._helpers_closed=threading.Event();self._ready=threading.Event();self._abort=threading.Event();self._input_released=threading.Event()
        self._lock=threading.Lock();self._commands=[];self._command_count=0;self._operations=frozenset()
        self._deadline=time.monotonic()+float(options['timeout']);self._proof_ref=None;self._error=None;self._result=None
        self._activated=False;self._thread=threading.Thread(target=self._run,daemon=False)
        phase.register_owner(self)
    def activate(self):
        self._phase.auth()
        with self._lock:
            if self._activated or self._done.is_set():raise RuntimeError('owned-activation-state')
            self._activated=True
            try:self._thread.start()
            except BaseException:
                self._phase.hold=True;raise
        return self
    def cancel_unstarted(self):
        with self._lock:
            if self._activated:raise RuntimeError('activated-owner-needs-native-closeout')
            self._error=RuntimeError('owned-cancelled-before-activation');self._done.set();self._ready.set()
    def _run(self):
        try:self._result=invoke(self._role,self._argv,_controller=self,**self._options)
        except BaseException as exc:self._error=exc
        finally:
            with self._lock:
                for command in self._commands:command['error']=self._error or RuntimeError('call-closed-before-interrupt');command['ack'].set()
            self._done.set();self._ready.set()
    def _bind(self,expected,binary,parent,deadline,operations,proof_ref):
        self._deadline=min(self._deadline,deadline);self._operations=operations;self._proof_ref=proof_ref;self._ready.set()
    @property
    def closed(self):return self._done.is_set() and not self._thread.is_alive() and (not self._created.is_set() or self._native_closed.is_set() and self._helpers_closed.is_set())
    @property
    def started_proof_ref(self):return self._proof_ref
    @property
    def input_released(self):return self._input_released.is_set()
    def wait_started(self,timeout=None):
        left=max(0,self._deadline-time.monotonic())
        if timeout is not None:left=min(left,max(0,timeout))
        if not self._ready.wait(left):self._abort.set();raise TimeoutError('owned-start-not-proven')
        if self._proof_ref is None:raise self._error or RuntimeError('owned-start-not-proven')
        return self._proof_ref
    def interrupt(self,operation):
        if operation not in self._operations or self._done.is_set():raise RuntimeError('owned-interrupt-not-granted-or-call-closed')
        with self._lock:
            if self._command_count>=8:raise RuntimeError('owned-interrupt-command-cap')
            self._command_count+=1;command={'operation':operation,'ack':threading.Event(),'error':None};self._commands.append(command)
        return InterruptTicket(self,command)
    def _has_commands(self):
        with self._lock:return bool(self._commands)
    def _next_command(self):
        with self._lock:return self._commands.pop(0) if self._commands else None
    def wait(self,timeout=None):
        if threading.current_thread() is self._thread:raise RuntimeError('owned-call-cannot-wait-in-start-callback')
        left=max(0,self._deadline-time.monotonic())+1
        if timeout is not None:left=min(left,max(0,timeout))
        if not self._done.wait(left):
            self._abort.set()
            if not self._done.wait(1):raise RuntimeError('owned-call-closeout-unknown-outer-owner-retained')
            self._thread.join(.2)
            raise TimeoutError('owned-call-wait-censored')
        self._thread.join(.2)
        if self._thread.is_alive():raise RuntimeError('owned-Python-worker-not-joined')
        if self._error is not None:raise self._error
        if not self.closed:raise RuntimeError('owned-native-or-helper-closure-unknown')
        return self._result


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
           control_only=False, expected_exit=0, env=None,
           stdout_cap=65536, stderr_cap=16384, input_cap=1024*1024,
           start_mode="json-stdin", gate_sha256=None, receipt_cap=65536, case_context=None,
           historical_grant=None, interrupt_grant=None, control_interrupts=(),
           on_started=None, _controller=None, phase=None):
    if role not in ROLE_LIMITS or not argv or type(timeout) not in (int, float) or not math.isfinite(timeout) or timeout <= 0:
        raise ValueError('explicit-role-command-timeout-required')
    if type(identity_budget) is not int or not 1 <= identity_budget <= 4096:
        raise ValueError('role-identity-budget')
    if type(evidence_cap) is not int or not 4096 <= evidence_cap <= 1024*1024:
        raise ValueError('role-evidence-cap')
    if not name.replace('-', '').isalnum():
        raise ValueError('role-name')
    allowed_exit=(expected_exit,) if type(expected_exit) is int else expected_exit
    if not isinstance(allowed_exit,(tuple,list)) or not allowed_exit or any(type(x) is not int or x not in (0,1,2,86,-2,-15,-9) for x in allowed_exit) or len(set(allowed_exit))!=len(allowed_exit):
        raise ValueError('role-explicit-expected-exit')
    if any(type(n) is not int or not 1 <= n <= 8*1024**2 for n in (stdout_cap, stderr_cap)) or stdout_cap+stderr_cap > 8*1024**2:
        raise ValueError('role-output-cap')
    if env is not None and (not isinstance(env, dict) or any(not isinstance(k,str) or not k.startswith('EXP_') or not isinstance(v,str) for k,v in env.items())):
        raise ValueError('role-environment-overrides')
    environment = {k:v for k,v in os.environ.items() if not k.startswith('EXP_')}
    environment['EXP_NAVIGATION'] = '4'
    environment.update(env or {})
    if type(input_cap) is not int or not 0 <= input_cap <= 4*1024**2:raise ValueError('role-input-cap')
    if start_mode not in ('json-stdin','bash-line'):raise ValueError('role-start-mode')
    if on_started is not None and (not callable(on_started) or _controller is None):raise ValueError('role-start-callback-requires-owned-call')
    payload=b''
    if start_mode=='bash-line':
        gate=Path(__file__).with_name('bash_start_gate_v31.sh')
        if input_path is not None or len(argv)<3 or Path(argv[0]).resolve()!=Path('/bin/bash').resolve() or Path(argv[1]).resolve()!=gate or gate.is_symlink() or gate_sha256!=sha256(gate):raise ValueError('role-reviewed-Bash-gate-contract')
        payload=b'NATIVE-ROLE-GO-v31\n'
    elif input_path is not None:
        ip=Path(input_path)
        if ip.is_symlink() or not ip.resolve().is_relative_to(admission.ROOT) or not ip.is_file() or ip.stat().st_size>input_cap:raise ValueError('role-bounded-input-path')
        with ip.open('rb') as source:payload=source.read(input_cap+1)
        if len(payload)>input_cap:raise ValueError('role-input-prelaunch-cap')
    token, ancestors = ancestor_token()
    binary = Path(argv[0]).resolve(strict=True)
    binary_hash = sha256(binary)
    if control_only:
        allowed_binary=(binary==Path(sys.executable).resolve() or (start_mode=='bash-line' and binary==Path('/bin/bash').resolve()))
        if token['source_pin']!='CONTROL_ONLY' or not allowed_binary or historical_grant is not None or interrupt_grant is not None:raise RuntimeError('role-control-only-exact-native-binary')
        if not isinstance(control_interrupts,(tuple,list)) or any(x not in ('interrupt','terminate','kill') for x in control_interrupts):raise ValueError('control-interrupt-operations')
        operations=frozenset(control_interrupts)
        binding_record={'source_pin':'CONTROL_ONLY','producer_source_pin':'CONTROL_ONLY','active_candidate_source_pin':'CONTROL_ONLY','producer_kind':'control','binary_sha256':binary_hash}
    else:
        if control_interrupts:raise ValueError('control-interrupts-require-control-only')
        binding_record=producer.bind(binary,token,binding,binding_sha256,request_bytes=payload,env=env,case_context=case_context,historical_grant=historical_grant)
        operations=producer.interrupt_operations(interrupt_grant,token,payload,case_context)
    negative={-2:'interrupt',-15:'terminate',-9:'kill'}
    if any(x<0 and negative[x] not in operations for x in allowed_exit):raise ValueError('negative-exit-requires-exact-owned-interrupt-grant')
    if type(receipt_cap) is not int or not 4096<=receipt_cap<=524288:raise ValueError('role-receipt-cap')
    if phase is None:raise RuntimeError('role-phase-lease-required')
    phase.auth()
    if _controller is None or phase.owners.get(name) is not _controller or not _controller._activated:raise RuntimeError('role-registered-activated-owner-required')
    request = {k: {'bytes': 0, 'files': 0} for k in admission.LIMITS}
    request['F12'] = {'bytes': evidence_cap+receipt_cap+stdout_cap+stderr_cap, 'files': 4}
    admission.admit(admission.snapshot(shared=False), request, admission.limits())
    folder = admission.ROOT/'evidence/roles'/name
    phase.role_files(folder,stdout_cap,stderr_cap,evidence_cap,receipt_cap)
    folder.mkdir(parents=True, exist_ok=False)
    events = Events(folder/'samples.jsonl.gz', evidence_cap)
    known = {}; failure = []; samples = []; stop = threading.Event()
    nonce = token['nonce']+'-role-'+name
    registry = monitor.Registry(nonce)
    myself = identity(os.getpid())
    deadline = min(time.monotonic()+timeout,
                   time.monotonic()+max(0, token['fixed_cutoff']-time.time()))
    if _controller is not None:deadline=min(deadline,_controller._deadline)
    child = None
    outputs = {k:{'path':str(folder/(k+'.bin')), 'bytes':0, 'sha256':None} for k in ('stdout','stderr')}
    streams = {k:open(v['path'],'xb') for k,v in outputs.items()}
    selector = selectors.DefaultSelector()
    output_complete = False
    began = None
    child_seconds = None
    def positively_ended(expected):
        rows=[]
        for _ in range(2):
            current=identity(expected['pid']);rows.append(current)
            if not (current['returned_bytes']==0 and current['errno'] in (0,3)):return False
            time.sleep(.001)
        events.emit({'kind':'role-known-exit-native-absence','expected':expected,'observations':rows,'monotonic':time.monotonic()})
        return True
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
                        if positively_ended(expected):continue
                        raise RuntimeError('role-owned-RSS-unavailable')
                    active.append({'identity': current, 'usage': memory})
                    query = monitor.children(current['pid'])
                    if not query['valid']:
                        if positively_ended(expected):continue
                        raise RuntimeError('role-scoped-descendants-unknown')
                    for pid in query['children']:
                        row = identity(pid)
                        if not monitor.valid_identity(row):
                            until=time.monotonic()+.008;ended=False
                            while time.monotonic()<until:
                                again=monitor.children(current['pid'])
                                if again['valid'] and pid not in again['children']:
                                    ended=True;break
                                row=identity(pid)
                                if monitor.valid_identity(row):break
                                time.sleep(.001)
                            if ended:continue
                            if not monitor.valid_identity(row):raise RuntimeError('role-descendant-identity-unknown')
                        if row['ppid'] != current['pid']:
                            raise RuntimeError('role-descendant-parent-race-unknown')
                        if pid in known and not monitor.same_identity(known[pid], row):
                            raise RuntimeError('role-descendant-identity-reuse')
                        phase.observe_identity(row);known[pid] = row; pending.append(row)
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
            began = time.monotonic()
            child = subprocess.Popen([str(binary), *argv[1:]], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                     stderr=subprocess.PIPE, env=environment, close_fds=True)
            if _controller is not None:_controller._created.set()
            for key,pipe in (('stdout',child.stdout),('stderr',child.stderr)):
                os.set_blocking(pipe.fileno(), False)
                selector.register(pipe, selectors.EVENT_READ, key)
            expected = identity(child.pid)
            if not monitor.valid_identity(expected) or expected['ppid'] != os.getpid() or expected['pgid'] != myself['pgid']:
                raise RuntimeError('role-created-child-identity')
            phase.observe_identity(expected);known[child.pid] = expected
            events.emit({'kind':'role-created-input-withheld','role':role,'binary':str(binary),'binary_sha256':binary_hash,'binding':binding_record,'ancestor_nonce':token['nonce'],'ancestors':ancestors,'child':expected,'timeout_seconds':timeout,'expected_exit':expected_exit,'env_overrides':env,'input_bytes':len(payload),'input_sha256':hashlib.sha256(payload).hexdigest(),'stdout_cap':stdout_cap,'stderr_cap':stderr_cap,'start_mode':start_mode,'gate_sha256':gate_sha256})
            thread = threading.Thread(target=sample,daemon=True);thread.start()
            # A final native executable which reads stdin cannot finish its
            # request while this exclusive owned pipe writer withholds all data.
            while not samples and not failure:
                if time.monotonic()>=deadline or child.poll() is not None:raise RuntimeError('role-native-start-before-positive-sample-failed')
                time.sleep(.001)
            if failure or not any(n>0 for _,n in samples):raise RuntimeError('role-native-positive-sample-required')
            proof=prove_actual_exec(expected,binary,myself,deadline)
            if not samples or time.monotonic()-samples[-1][0]>.04:raise RuntimeError('role-start-sampler-stale')
            ancestor_token();events.emit(dict(proof,kind='role-native-start-proof',input_bytes_delivered=0,positive_sample_count=len(samples)))
            if _controller is not None:_controller._bind(expected,binary,myself,deadline,operations,str(folder/'samples.jsonl.gz'))
            if on_started is not None:on_started(_controller)
            # Callbacks cannot certify startup or override failed role telemetry.
            if failure or time.monotonic()-samples[-1][0]>.04:raise RuntimeError('role-post-callback-sampler-unhealthy')
            ancestor_token();proof=prove_actual_exec(expected,binary,myself,deadline)
            events.emit(dict(proof,kind='role-before-input-recheck',input_bytes_delivered=0))
            os.set_blocking(child.stdin.fileno(),False)
            delivered=0;first_byte=False;interrupted=False
            if payload:selector.register(child.stdin,selectors.EVENT_WRITE,'stdin')
            else:
                child.stdin.close()
                events.emit({'kind':'role-input-complete','bytes':0,'monotonic':time.monotonic(),'EOF_after_native_proof':True})
                if _controller is not None:_controller._input_released.set()
            while (child.poll() is None or selector.get_map()) and not failure:
                if time.monotonic()>=deadline:raise RuntimeError('role-timeout')
                if _controller is not None:
                    if _controller._abort.is_set():raise RuntimeError('owned-wait-aborted')
                    command=_controller._next_command()
                    if command is not None:
                        try:
                            if command['operation'] not in operations:raise RuntimeError('owned-interrupt-not-granted')
                            checked=prove_actual_exec(expected,binary,myself,deadline)
                            if failure or time.monotonic()-samples[-1][0]>.04:raise RuntimeError('owned-interrupt-sampler-unhealthy')
                            ancestor_token()
                            signum={'interrupt':signal.SIGINT,'terminate':signal.SIGTERM,'kill':signal.SIGKILL}[command['operation']]
                            keeper.exact_signal(expected,signum)
                            ack={'kind':'owned-native-interrupt','operation':command['operation'],'proof':checked,'monotonic':time.monotonic()}
                            events.emit(ack);command['result']=ack;interrupted=True
                            if not child.stdin.closed:
                                try:selector.unregister(child.stdin)
                                except KeyError:pass
                                child.stdin.close()
                        except BaseException as exc:
                            command['error']=exc;raise
                        finally:command['ack'].set()
                for selected,_ in selector.select(.002):
                    key=selected.data
                    if key=='stdin':
                        if _controller is not None and _controller._has_commands():continue
                        if failure or time.monotonic()-samples[-1][0]>.04:raise RuntimeError('role-input-sampler-unhealthy')
                        try:n=os.write(child.stdin.fileno(),payload[delivered:delivered+16384])
                        except BlockingIOError:continue
                        if n<=0:raise RuntimeError('role-input-write-unknown')
                        if not first_byte:
                            events.emit({'kind':'role-input-first-byte','monotonic':time.monotonic(),'start_proof_monotonic':proof['monotonic'],'positive_sample_count':len(samples),'start_mode':start_mode});first_byte=True
                        delivered+=n
                        if delivered==len(payload):
                            selector.unregister(child.stdin);child.stdin.close()
                            events.emit({'kind':'role-input-complete','bytes':delivered,'monotonic':time.monotonic()})
                            if _controller is not None:_controller._input_released.set()
                        continue
                    try:block=os.read(selected.fileobj.fileno(),16384)
                    except BlockingIOError:continue
                    if not block:selector.unregister(selected.fileobj);continue
                    maximum=stdout_cap if key=='stdout' else stderr_cap
                    if outputs[key]['bytes']+len(block)>maximum:raise RuntimeError('role-output-cap:'+key)
                    streams[key].write(block);outputs[key]['bytes']+=len(block)
            output_complete=not selector.get_map() and not failure
            if delivered!=len(payload) and not any(x<0 for x in allowed_exit):raise RuntimeError('role-input-delivery-incomplete')
            child_seconds = time.monotonic()-began
            output_complete = not selector.get_map() and not failure
            if failure:
                raise RuntimeError(failure[0])
            if child.returncode not in allowed_exit:
                raise RuntimeError('role-child-exit:'+str(child.returncode))
            if child.returncode<0 and not interrupted:raise RuntimeError('negative-exit-without-owned-interrupt-proof')
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
            elif _controller is not None:_controller._native_closed.set()
            events.emit({'kind': 'role-close', 'failures': failure, 'closure': closure})
            events.close()
            selector.close()
            if child is not None:
                if not child.stdin.closed:child.stdin.close()
                child.stdout.close();child.stderr.close()
            for key,stream in streams.items():
                stream.close();outputs[key]['sha256']=sha256(outputs[key]['path'])
        result = {'role': role, 'limit_bytes': ROLE_LIMITS[role], 'failures': failure,
                  'samples': len(samples), 'max_start_gap_s': max((b[0]-a[0] for a,b in zip(samples,samples[1:])), default=0),
                  'peak_RSS_bytes': max((x[1] for x in samples), default=None),
                  'closure': closure, 'binary_sha256': binary_hash, 'source_pin':binding_record['producer_source_pin'],'active_candidate_source_pin':token['source_pin'],'producer_binding':binding_record,
                  'exit':child.returncode if child is not None else None,
                  'seconds':child_seconds,
                  'guarded_seconds':time.monotonic()-began if began is not None else None,
                  'output_complete':output_complete,'outputs':outputs,'start_mode':start_mode,'input_bytes_delivered':delivered if began is not None and 'delivered' in locals() else 0}
        admission.bounded_receipt(folder/'result.json',result,byte_cap=receipt_cap)
        if failure: raise RuntimeError('role-invocation-censored:'+repr(failure))
        return result
    finally:
        helper_stop.set()
        if helper_thread.ident is not None: helper_thread.join(.2)
        joined=(not helper_thread.is_alive() and (thread is None or not thread.is_alive()))
        if joined:
            if _controller is not None:_controller._helpers_closed.set()
            registry.update(remove=True)
        else:raise RuntimeError('role-helper-or-sampler-shutdown-unknown-registry-retained')
