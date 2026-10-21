import copy
import os
from pathlib import Path
import subprocess
import threading
import time
import uuid
from windows_deadlines import (CHECKER_BYTES, READER_BYTES, READERS, Deadline,
                               Refusal, canonical, digest, integer, need, strict)
from windows_growth import file_hash, regular_read, native_call_bound
from windows_native import PIPE_BYTES, DriverSupport, counter
from windows_evidence import record_call


class NativeCallFailure(AssertionError):
    def __init__(self,row,reference):
        super().__init__(row);self.reference=reference
    def __repr__(self):
        return "NativeCallFailure("+repr(dict(call=self.args[0]["call_token"],raw_ref=self.reference))+")"
    __str__=__repr__


class WindowsDriver(DriverSupport):
    def __init__(self, native, executable, expected_sha256, final_pin, evidence,
                 admit, absolute_deadline, *, observer=None, historical_policy=None):
        self.native,self.executable=native,Path(executable)
        need(self.executable.is_absolute() and self.executable.resolve()==self.executable,
             "canonical-owned-executable")
        self.binary_sha256,self.final_pin=expected_sha256,final_pin
        self.evidence,self.admit,self.absolute_deadline=evidence,admit,absolute_deadline
        self.observer,self.historical_policy=observer,historical_policy
        self.lock,self.registry=native.lock,{}
        self.context=threading.local();self.phase_limits={}
        self.verify_binary()

    def interrupt(self,token,kind):
        need(kind in ("cancel","death"),"closed-transport-interrupt-kind")
        with self.lock:
            state=self._state(token)
            if state["intentional_interrupt"]==kind:
                return copy.deepcopy(state["interrupt_request"])
            need(not state["resources_closed"] and state.get("ownership")=="owned" and state["action"] in READERS
                 and not state.get("termination_retired"),"owned-live-reader-only")
            identity=self.native.identity(state["native"])
            need(state["intentional_interrupt"] is None,"conflicting-transport-interrupt")
            need(self._terminate(state,"registered-"+kind,138 if kind=="cancel" else 137),
                 "interrupt-natural-exit-before-termination")
            state["intentional_interrupt"]=kind
            state["interrupt_request"]=dict(call_token=token,kind=kind,
                identity=identity,termination_requested=True,semantic_noncommit=False)
            return copy.deepcopy(state["interrupt_request"])

    def wait_control(self,token,timeout):
        with self.lock:
            state=self._state(token)
        need(state["done"].wait(min(max(0,timeout),5)),"call-owner-not-completed")
        with self.lock:
            need(state.get("owner_finished") and state["row"] is not None,
                 "missing-control-owner-fence-or-raw-row")
            return copy.deepcopy(state["row"])

    def live(self,token):
        with self.lock:
            state=self._state(token)
            if state["resources_closed"] or "native" not in state:
                return False
            try:
                self.native.identity(state["native"])
                return True
            except Refusal:
                return False

    def controls(self):
        with self.lock:
            return {key:dict(action=s["action"],root=s["root"],
                closed=s["resources_closed"],callback_joined=s["callback_joined"],
                done=s["done"].is_set(),native_identity=copy.deepcopy(s.get("public_identity")),
                context=copy.deepcopy(s["context"])) for key,s in self.registry.items()}

    def close_owned(self,reason,root=None):
        with self.lock:
            rows=[s for s in self.registry.values() if (root is None or s["root"]==str(root))
                  and (not s["resources_closed"] or not s["done"].is_set())]
        for state in rows:
            try:
                self._terminate(state,reason)
            except BaseException as error:
                state["violations"].append("close-owned:"+repr(error))
        for state in rows:
            need(state["done"].wait(5),"owned-call-not-joined")
            need(state["resources_closed"],"owned-call-closure-uncertain")

    def call(self,root,action,*,timeout=120,code=0,env=None,on_started=None,**fields):
        began=time.perf_counter_ns()
        payload=dict(root=str(root),action=action,**copy.deepcopy(fields));encoded=canonical(payload)
        token=getattr(self.context,"pending_token",None)
        try:
            if token in getattr(self.evidence,"calls",{}):
                c=self.evidence.calls[token];p=c["profile"]
                self.evidence.launch(token,[str(self.executable)],self.binary_sha256,
                    "historical" if self.historical_policy else "candidate",
                    self.historical_policy.grant["binary_source_pin"] if self.historical_policy else self.final_pin,
                    p.get("producer_source_files",[]),None)
            deadline=Deadline(self.absolute_deadline-12,timeout)
            self.verify_binary(deadline.poll)
            need(on_started is None or callable(on_started),"explicit-start-callback")
            need(len(encoded)<=4*1024**2,"engine-input-4MiB-bound")
            historical=self.historical_policy(payload,env,code) if self.historical_policy else None
            profile=self.admit(payload,env,code);deadline.poll()
            profile["_transport_started_ns"]=began
            if profile.get("api_version")==2 and token is None:
                token=uuid.uuid4().hex
                self.evidence.prepare_pending(token,root,action,profile)
                self.context.pending_token=token
            return self._execute([str(self.executable)],payload,encoded,env,code,deadline.remaining(),
                on_started,profile,historical=historical)
        except BaseException as error:
            if token in getattr(self.evidence,"calls",{}) and not self.evidence.calls[token]["ended"]:
                row=self.evidence.abort_pending(token,payload,env,self.binary_sha256,self.final_pin,error)
                self.context.completed_token=token;self.context.completed_row=row;self.context.completed_response=None
                ref=self.evidence.call_reference(token)
                self.evidence.finish_call(token)
                raise NativeCallFailure(row,ref) from error
            raise
        finally:
            self.context.pending_token=None

    def _phase_clamp(self,deadline,state,payload):
        pacing=payload.get("concurrent_pacing") or {}
        phase=pacing.get("phase_id") or state["context"].get("phase")
        with self.lock:limit=copy.deepcopy(self.phase_limits.get(phase))
        if limit is None:return
        seconds=(limit["old_pin_deadline_seconds"] if pacing.get("reader")==0 and pacing.get("batch")==0
            else limit["reader_deadline_seconds"] if state["action"] in READERS
            else limit["publisher_deadline_seconds"])
        end=(limit["origin_perf_ns"]+int(seconds*1_000_000_000))/1_000_000_000
        deadline.mono_end=min(deadline.mono_end,end)

    def script(self,spec,profile):
        need(spec["kind"]=="python" and spec.get("startup_gate"),"script-preaccess-gate-required")
        return self._execute(spec["argv"],dict(script_spec=copy.deepcopy(spec)),b"",
            spec.get("env"),spec["expected_exit"],spec["timeout"],None,profile,
            script_gate=spec["startup_gate"],output_mode=spec["output_mode"],
            executable_sha256=spec["executable_sha256"])

    def _execute(self,argv,payload,encoded,env,code,timeout,on_started,profile,
                 *,historical=None,script_gate=None,output_mode="json",executable_sha256=None):
        need(set(profile)>={"stdout_bytes","stderr_bytes","rss_samples","callback_seconds"},
             "closed-invocation-ceilings")
        need(all(integer(profile[k],1) for k in ("stdout_bytes","stderr_bytes","rss_samples")),
             "positive-invocation-ceilings")
        need(profile["stdout_bytes"]+profile["stderr_bytes"]<=16*1024**2,"bounded-call-RAM")
        need(0 < profile["callback_seconds"] <= .5,"bounded-start-callback")
        environment={k:v for k,v in os.environ.items() if not k.startswith("EXP_")}
        environment["EXP_NAVIGATION"]="4"
        if env is not None:
            need(type(env) is dict and all(type(k) is str and type(v) is str
                 and (k.startswith("EXP_") or k=="FILE_NATIVE_PACED_ROOT" and v==profile.get("paced_root"))
                 and "\0" not in k+v for k,v in env.items()),
                 "engine-options-outside-INPUT")
            environment.update(env)
        action=payload.get("action","script")
        root=payload.get("root",script_gate.get("root") if script_gate else None)
        limit=READER_BYTES if action in READERS or script_gate and profile.get("role")=="reader" else CHECKER_BYTES
        deadline=Deadline(self.absolute_deadline-12,timeout)
        token=getattr(self.context,"pending_token",None) or uuid.uuid4().hex
        need(len(canonical(env))<=8192,"env-evidence-bound")
        bound=native_call_bound(profile,payload,env)
        state=dict(token=token,root=str(root),action=action,violations=[],process=None,
            resources_closed=False,callback_joined=True,done=threading.Event(),row=None,
            intentional_interrupt=None,termination_requests=[],input_bytes_sent=0,
            stdin_closed=False,input_fully_delivered=False,input_transfer_accounting_complete=True,
            stdout_eof=False,stderr_eof=False,context=copy.deepcopy(getattr(self.context,"value",{})),
            ownership="preparing",termination_retired=False,native_handles=[],
            popen_handle_open=False,popen_handle_closed=False,popen_reaped=False,
            owner_finished=False,evidence_reserved=False,raw_row_written=False,first_write_qpc=None)
        out,err=bytearray(),bytearray()
        offsets={"stdout":0,"stderr":0}
        pipes=[];timer=None;callback=None;response=None;parse_error=None;p=None
        started=profile.get("_transport_started_ns",time.perf_counter_ns());peak=0;samples=0;rss_next=0;input_offset=0
        actual_sha=executable_sha256 or self.binary_sha256

        def terminate(reason):
            try:
                self._terminate(state,reason)
            except BaseException as error:
                state["violations"].append("native-terminate:"+repr(error))

        def expire():
            state["violations"].append("time-censor")
            terminate("absolute-total-deadline")

        def sample(force=False):
            nonlocal peak,samples,rss_next
            self._phase_clamp(deadline,state,payload)
            now=time.perf_counter_ns()
            if force or now>=rss_next:
                identity=state.get("native")
                if identity is not None and state["process"].poll() is None:
                    try:
                        rss,hwm=self.native.memory(identity)
                    except BaseException:
                        if self.native.exited(identity):
                            return
                        raise
                    peak=max(peak,hwm);samples+=1
                    need(samples<=profile["rss_samples"],"RSS-sample-evidence-ceiling")
                    self.evidence.append(dict(kind="native-RSS",call_token=token,
                        observed_ns=now,rss=rss,peak=hwm,pid=identity["pid"],
                        creation_time=identity["creation_time"],rss_limit=limit))
                    need(0<rss<=limit and hwm<=limit,"reader-or-checker-RSS-censor")
                rss_next=now+10_000_000

        def save_raw(name,block):
            target=out if name=="stdout" else err
            if name=="stdout" and block and not state["input_fully_delivered"] and not script_gate:
                if "response-before-complete-input" not in state["violations"]:
                    state["violations"].append("response-before-complete-input")
            target.extend(block)
            try:
                self.evidence.raw(token,name,offsets[name],block)
                offsets[name]+=len(block)
            finally:
                if profile.get("api_version")==2:offsets[name]=self.evidence.accepted(token,name)
            need(len(target)<=profile[name+"_bytes"]+PIPE_BYTES,"bounded-final-pipe-tail")
            if len(target)>profile[name+"_bytes"]:
                if "output-bound:"+name not in state["violations"]:
                    state["violations"].append("output-bound:"+name)
                terminate("output-ceiling")

        try:
            with self.lock:
                self.registry[token]=state
            if profile.get("api_version")==2:
                if token not in self.evidence.calls:self.evidence.prepare_pending(token,root,action,profile)
                self.evidence.begin_call(token,payload,encoded,env,profile)
            else:self.evidence.reserve_call(token,bound["native_bytes"])
            state["evidence_reserved"]=True
            if profile.get("api_version")==2:
                self.evidence.launch(token,argv,actual_sha,
                    "historical" if historical else "python-argv" if script_gate else "candidate",
                    historical["binary_source_pin"] if historical else profile.get("producer_source_pin",self.final_pin),
                    profile.get("producer_source_files",[]),script_gate)
            need(file_hash(Path(argv[0]),poll=deadline.poll)==actual_sha,"actual-invoked-image-hash")
            timer=threading.Timer(deadline.remaining(),expire);timer.daemon=False;timer.start()
            for output in (False,True,True):
                self.native.pipe(output,registry=pipes)
            pin,pout,perr=pipes
            deadline.poll()
            p=subprocess.Popen(argv,stdin=pin.client_fd,stdout=pout.client_fd,stderr=perr.client_fd,
                env=environment,close_fds=True,creationflags=0x4)
            with self.lock:
                state["process"]=p;state["popen_handle_open"]=True;state["ownership"]="owned"
                state["popen_handle_value"]=int(p._handle)
                state["native"]=self.native.capture(p,Path(argv[0]),actual_sha,state)
                state["public_identity"]=self.native.identity(state["native"])
            for pipe in pipes:
                pipe.close_client()
            sample(True);deadline.poll()
            self.native.resume(state["native"])
            resumed=time.perf_counter_ns();state["resumed_ns"]=resumed
            sample(True)
            if script_gate:
                gate=Path(script_gate["ready"])
                while not gate.exists():
                    deadline.poll();sample()
                    need(p.poll() is None,"script-ended-before-startup-gate")
                    time.sleep(min(.005,deadline.remaining()))
                ready=strict(regular_read(gate,8192),8192)
                need(ready.get("pid")==p.pid and ready.get("nonce")==script_gate["nonce"]
                     and ready.get("manifest_sha256")==script_gate["manifest_sha256"],
                     "authentic-arg-script-preaccess-gate")
            rss,_=self.native.memory(state["native"])
            event=dict(call_token=token,pid=p.pid,creation_time=state["native"]["creation_time"],
                same_job=True,native_handle_identity=True,guard_nonce=self.native.owner["nonce"],
                resumed_ns=resumed,observed_ns=time.perf_counter_ns(),positive_rss=rss,
                before_stdin_delivery=True,executable_sha256=actual_sha,final_pin=self.final_pin,
                context=copy.deepcopy(state["context"]))
            state["positive_start_event"]=event
            state["start_reference"]=self.evidence.append(dict(kind="positive-native-start",**event))
            callbacks=[x for x in (self.observer,on_started) if x is not None]
            if callbacks:
                errors=[]
                def notify():
                    try:
                        for fn in callbacks:
                            fn(copy.deepcopy(event))
                    except BaseException as error:
                        errors.append(repr(error))
                callback=threading.Thread(target=notify,name="native-start-"+token,daemon=False)
                state["callback_joined"]=False
                callback.start();callback.join(min(profile["callback_seconds"],deadline.remaining()))
                state["callback_joined"]=not callback.is_alive()
                need(state["callback_joined"] and not errors,"start-callback-timeout-or-error:"+repr(errors))
            deadline.poll();sample()
            if script_gate:
                release=Path(script_gate["release"])
                need(not release.exists(),"fresh-script-gate-release")
                with release.open("xb") as stream:
                    stream.write(canonical(dict(nonce=script_gate["nonce"],pid=p.pid))+b"\n")
                    stream.flush()
            if not encoded:
                pin.close();state["stdin_closed"]=True;state["input_fully_delivered"]=True
            transfer=None;tail_end=None;transfer_qpc=None
            while True:
                if deadline.remaining()<=0 and "time-censor" not in state["violations"]:
                    expire()
                sample()
                if state["violations"] or state["intentional_interrupt"]:
                    terminate("failure-or-registered-interrupt")
                    tail_end=tail_end or time.perf_counter()+5
                need(tail_end is None or time.perf_counter()<tail_end,
                     "post-termination-pipe-tail-deadline")
                for name,pipe in (("stdout",pout),("stderr",perr)):
                    if pipe.eof:
                        state[name+"_eof"]=True;continue
                    value=pipe.complete() if pipe.pending else pipe.begin()
                    if value is not None:
                        if value:
                            save_raw(name,value)
                        if pipe.eof or value==b"":
                            pipe.eof=True;state[name+"_eof"]=True
                if not pin.closed:
                    if state["violations"] or state["intentional_interrupt"]:
                        if pin.pending:
                            state["input_transfer_accounting_complete"]=False
                        pin.close();state["stdin_closed"]=True
                    else:
                        if not pin.pending and transfer is None:
                            transfer=encoded[input_offset:input_offset+16384]
                            transfer_qpc=counter() if profile.get("api_version")==2 else None
                            value=pin.begin(transfer)
                        else:
                            value=pin.complete()
                        if value is not None:
                            if value==0 and input_offset<len(encoded):
                                state["input_broken_pipe"]=True
                                pin.close();state["stdin_closed"]=True
                            else:
                                if value>0 and state["first_write_qpc"] is None:
                                    state["first_write_qpc"]=transfer_qpc
                                input_offset+=value
                                state["input_bytes_sent"]=input_offset
                                state.setdefault("stdin_started_ns",time.perf_counter_ns())
                                transfer=None
                                if input_offset==len(encoded):
                                    pin.close();state["stdin_closed"]=True
                                    state["input_fully_delivered"]=True
                                    state["input_complete_ns"]=time.perf_counter_ns()
                if state["stdout_eof"] and state["stderr_eof"] and p.poll() is not None:
                    break
                time.sleep(.001)
            p.wait(timeout=1)
        except BaseException as error:
            state["violations"].append("transport:"+repr(error))
            terminate("transport-error")
        finally:
            try:
                self._finish_resources(state,pipes,timer,callback,save_raw)
            except BaseException as error:
                state["violations"].append("unexpected-resource-finalizer:"+repr(error))
                with self.lock:
                    state["termination_retired"]=True
                    state["ownership"]="terminal_uncertain"
                    state["resources_closed"]=False
            try:
                response,row=record_call(self,state,payload,encoded,env,argv,profile,
                    historical,script_gate,output_mode,actual_sha,started,peak,samples,
                    deadline,out,err,offsets,code,limit)
            except BaseException as error:
                state["violations"].append("unexpected-row-finalizer:"+repr(error))
                row=dict(input=payload,env=env,call_token=token,exit=None,
                    executable_sha256=actual_sha,violations=list(state["violations"]),
                    resources_closed=state["resources_closed"],raw_retained=False,
                    row_incomplete=True,no_child_started=state["process"] is None)
            finally:
                try:
                    if token in self.evidence.pending:
                        self.evidence.finish_call(token)
                except BaseException as error:
                    state["violations"].append("call-evidence-settlement:"+repr(error))
                with self.lock:
                    row["violations"]=list(state["violations"])
                    self.context.completed_token=token;self.context.completed_row=row
                    self.context.completed_response=response
                    state["row"]=(row if state["intentional_interrupt"] or code in (2,86)
                        else {k:v for k,v in row.items() if k not in ("stdout","stderr","input")})
                    state["row"]["input_sha256"]=digest(canonical(payload))
                    state["owner_finished"]=True
                    state["done"].set()
        if row["exit"]!=code or row["violations"] or state["intentional_interrupt"]:
            raise NativeCallFailure(row,self.registry[token].get("raw_reference"))
        return response



def windows_sharing_case(host,case_id,root):
    h=host.harness;root=Path(root);mode="B" if case_id.endswith("-B") or case_id.startswith("F/") else "A"
    h.initialize(root,mode,32);h.publish(root,"sharing-tail")
    control=case_id.startswith("F/");containers=case_id=="Windows-pack-index-no-delete-sharing"
    if containers:
        h.pack(root,"raw",bootstrap=True)
        names=h.containers(h.state(root));need(names,"actual-pack-index-containers")
    else:
        role="publications" if mode=="A" or control else "results"
        names=[p.relative_to(root).as_posix() for p in (root/role).glob("*.json")]
        need(names and len(names)<=64,"bounded-loose-sharing");names=names[:1]
    held=[];before={n:file_hash(root/n) for n in names}
    try:
        for name in names:
            handle=host.native.K.CreateFileW(str(root/name),0x80000000,3,None,3,0,None)
            need(handle not in (0,None,-1,2**64-1),"no-delete-share-HANDLE")
            held.append(handle)
        h.publish(root,"while-share-held")
        h.pack(root,"zstd",bootstrap=not containers)
        h.cleanup(root,max_batches=2,require_complete=False)
        need(all((root/n).is_file() and file_hash(root/n)==before[n] for n in names),
             "exact-held-sharing-bytes")
        if not containers and not control:h.verify(root,"held-share-mapped",mapped_keys=names)
    finally:
        errors=[]
        for handle in held:
            try:
                with host.native.lock:need(host.native.K.CloseHandle(handle),"share-HANDLE-close")
            except BaseException as error:errors.append(repr(error))
        need(not errors,"all-share-HANDLE-closures:"+repr(errors))
    h.drain_two_passes(root)
    need(all((root/n).is_file() and file_hash(root/n)==before[n] for n in names) if control
         else all(not(root/n).exists() for n in names),"release-removal-or-permanent-control")
    return h.verify(root,"sharing-release-retry")
