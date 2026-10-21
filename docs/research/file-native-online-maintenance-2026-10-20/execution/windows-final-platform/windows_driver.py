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


class NativeCallFailure(AssertionError):
    def __init__(self,row,reference):
        super().__init__(row);self.reference=reference
    def __repr__(self):
        return "NativeCallFailure("+repr(dict(call=self.args[0]["call_token"],raw_ref=self.reference))+")"
    __str__=__repr__


def maintainer_death_match(driver,state):
    grant=state.get("maintainer_death_grant");context=state.get("context",{})
    if not grant or state["action"]!="packing" or context.get("case_id")!="S-maintainer-death":
        return False
    c=driver.evidence.calls[state["token"]];env=c["env"] or {}
    proof=state.get("checkpoint_proof") or {}
    return (grant["case_scope"]==context.get("case_scope")
        and grant["phase"] in ("physical-before-selection","physical-after-selection")
        and grant["phase"]==env.get("EXP_BARRIER")==proof.get("phase")
        and grant["ready_ref"]==proof.get("ready_ref")
        and grant["root"]==state["root"]==strict(c["encoded"],4194304)["root"]
        and grant["input_sha256"]==digest(canonical(strict(c["encoded"],4194304)))
        and (grant["pid"],grant["creation_time"])==
            (state["public_identity"]["pid"],state["public_identity"]["creation_time"])
        and regular_read(grant["ready_ref"]["path"],8192)==grant["phase"].encode("ascii"))


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
            need(not state["resources_closed"] and state.get("ownership")=="owned" and (state["action"] in READERS or kind=="death" and maintainer_death_match(self,state))
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
            if profile.get("api_version")==2 and token is None:
                token=uuid.uuid4().hex
                self.evidence.prepare_pending(token,root,action,profile)
                self.context.pending_token=token
            return self._execute([str(self.executable)],payload,encoded,env,code,deadline.remaining(),
                on_started,profile,historical=historical,transport_started_ns=began)
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

    def script(self,spec,profile,*,external=None):
        need(spec["kind"]=="python" and spec.get("startup_gate"),"script-preaccess-gate-required")
        from windows_growth import confined
        gate=spec["startup_gate"];ready=Path(gate["ready"]);release=Path(gate["release"])
        ledger=self.evidence.ledger;category=ledger.root_name(ready.parent)
        need(ready.parent==release.parent and ready!=release and ready.parent.is_dir()
            and type(gate["nonce"]) is str and len(gate["nonce"])==32
            and all(c in "0123456789abcdef" for c in gate["nonce"]),"owned-fresh-script-gate")
        for path in (ready,release):
            confined(Path(ledger.roots[category]["root"]),path)
            need(not path.exists(),"script-gate-no-overwrite")
        ledger.reserve("script-gate:"+gate["nonce"],{category:dict(bytes=16384,files=2,directories=0)})
        # Both files and their full upper hold remain retained, including failures.
        return self._execute(spec["argv"],dict(script_spec=copy.deepcopy(spec)),b"",
            spec.get("env"),spec["expected_exit"],spec["timeout"],None,profile,
            script_gate=spec["startup_gate"],output_mode=spec["output_mode"],
            executable_sha256=spec["executable_sha256"],external=external)

    def _execute(self,argv,payload,encoded,env,code,timeout,on_started,profile,
                 *,historical=None,script_gate=None,output_mode="json",executable_sha256=None,transport_started_ns=None,external=None):
        need(not getattr(self.native,"stream_uncertain",False),"stream-resource-fence")
        need(set(profile)>={"stdout_bytes","stderr_bytes","rss_samples","callback_seconds"},
             "closed-invocation-ceilings")
        need(all(integer(profile[k],1) for k in ("stdout_bytes","stderr_bytes","rss_samples")),
             "positive-invocation-ceilings")
        need(profile["stdout_bytes"]+profile["stderr_bytes"]<=16*1024**2,"bounded-call-RAM")
        need(0 < profile["callback_seconds"] <= .5,"bounded-start-callback")
        environment={k:v for k,v in os.environ.items() if not k.startswith("EXP_")}
        environment["EXP_NAVIGATION"]="4"
        if env is not None:
            if payload.get("action")=="native-command":
                allowed={"EXP_PACK_TEST_ROOT","TMP","TEMP","CARGO_INCREMENTAL","CARGO_NET_OFFLINE",
                    "CARGO_HOME","FILE_NATIVE_ORDINARY"}
                need(type(env) is dict and set(env)<=allowed and all(type(v) is str
                    and "\0" not in k+v for k,v in env.items()),"source-owned-native-command-env")
            else:
                need(type(env) is dict and all(type(k) is str and type(v) is str
                     and (k.startswith("EXP_") or k=="FILE_NATIVE_PACED_ROOT" and v==profile.get("paced_root"))
                     and "\0" not in k+v for k,v in env.items()),
                     "engine-options-outside-INPUT")
            environment.update(env)
        if script_gate:
            from windows_evidence import WIRE_CONTRACT
            need(set(script_gate)==set(WIRE_CONTRACT["types"]["script_gate"]["object"])
                and len(canonical(script_gate))<=8192,"closed-source-script-gate")
            environment["FILE_NATIVE_SCRIPT_GATE"]=canonical(script_gate).decode("utf-8")
        action=payload.get("action","script")
        root=payload.get("root",script_gate.get("root") if script_gate else None)
        limit=READER_BYTES if action in READERS or script_gate and profile.get("role")=="reader" else CHECKER_BYTES
        timeout=min(timeout,profile["timeout_seconds"]) if profile.get("api_version")==2 else timeout
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
        started=transport_started_ns if transport_started_ns is not None else time.perf_counter_ns();peak=0;samples=0;rss_next=0;input_offset=0
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
                    "historical" if historical else "python-argv" if script_gate else profile.get("producer_kind","candidate"),
                    historical["binary_source_pin"] if historical else profile.get("producer_source_pin",self.final_pin),
                    profile.get("producer_source_files",[]),script_gate)
            need(file_hash(Path(argv[0]),poll=deadline.poll)==actual_sha,"actual-invoked-image-hash")
            timer=threading.Timer(deadline.remaining(),expire);timer.daemon=False;timer.start()
            for output in (False,True,True):
                self.native.pipe(output,registry=pipes)
            pin,pout,perr=pipes
            self._phase_clamp(deadline,state,payload);deadline.poll()
            if script_gate:
                script_deadline=int(deadline.mono_end*1_000_000_000)
                need(0<script_deadline<2**63,"actual-script-absolute-perf-deadline")
                environment["FILE_NATIVE_SCRIPT_DEADLINE_PERF_NS"]=str(script_deadline)
            state["creating_process"]=True
            p=subprocess.Popen(argv,stdin=pin.client_fd,stdout=pout.client_fd,stderr=perr.client_fd,
                env=environment,cwd=profile["cwd"] if action=="native-command" else None,
                close_fds=True,creationflags=0x4,startupinfo=external.startupinfo() if external else None)
            state["creating_process"]=False
            with self.lock:
                state["process"]=p;state["popen_handle_open"]=True;state["ownership"]="owned"
                state["popen_handle_value"]=int(p._handle)
                state["native"]=self.native.capture(p,Path(argv[0]),actual_sha,state)
                state["public_identity"]=self.native.identity(state["native"])
            if external:external.spawned()
            for pipe in pipes:
                pipe.close_client()
            sample(True);deadline.poll()
            rss,_=self.native.memory(state["native"])
            state["start_reference"]=self.evidence.append(dict(kind="positive-native-start",
                call_token=token,pid=p.pid,creation_time=state["native"]["creation_time"],
                positive_rss=rss,executable_sha256=actual_sha))
            self.native.resume(state["native"])
            resumed=time.perf_counter_ns();state["resumed_ns"]=resumed
            sample(True)
            if script_gate:
                gate=Path(script_gate["ready"])
                while not gate.exists():
                    deadline.poll();sample()
                    need(p.poll() is None,"script-ended-before-startup-gate")
                    time.sleep(min(.005,deadline.remaining()))
                ready_raw=regular_read(gate,8192);ready=strict(ready_raw,8192)
                need(ready.get("pid")==p.pid and ready.get("nonce")==script_gate["nonce"]
                     and ready.get("manifest_sha256")==script_gate["manifest_sha256"],
                     "authentic-arg-script-preaccess-gate")
            rss,_=self.native.memory(state["native"])
            need(0<rss<=limit and self.native.identity(state["native"])["pid"]==p.pid,
                "live-positive-native-RSS-before-script-release")
            if script_gate:
                self.evidence.append(dict(kind="script-startup-gate-ready",call_token=token,
                    context=state["context"],gate=copy.deepcopy(script_gate),
                    deadline_perf_ns=script_deadline,ready_ref=dict(path=str(gate),offset=0,bytes=len(ready_raw),sha256=digest(ready_raw))))
            event=dict(call_token=token,pid=p.pid,creation_time=state["native"]["creation_time"],
                same_job=True,native_handle_identity=True,guard_nonce=self.native.owner["nonce"],
                resumed_ns=resumed,observed_ns=time.perf_counter_ns(),positive_rss=rss,
                before_stdin_delivery=True,executable_sha256=actual_sha,final_pin=self.final_pin,
                context=copy.deepcopy(state["context"]))
            state["positive_start_event"]=event
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
                    release_raw=canonical(dict(nonce=script_gate["nonce"],pid=p.pid))+b"\n"
                    need(stream.write(release_raw)==len(release_raw),"complete-source-gate-release-write")
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
            if state.get("creating_process"):
                state["creation_unknown"]=True;self.native.stream_uncertain=True
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
            if state.get("creation_unknown"):state["resources_closed"]=False
            if external:
                try:external.spawned()
                except BaseException as error:
                    state["resources_closed"]=False
                    state["violations"].append("stream-endpoint:"+repr(error))
            try:
                response,row=record_call(self,state,payload,encoded,env,argv,profile,
                    historical,script_gate,output_mode,actual_sha,started,peak,samples,
                    deadline,out,err,offsets,code,limit)
            except BaseException as error:
                state["violations"].append("unexpected-row-finalizer:"+repr(error))
                row=dict(input=payload,env=env,call_token=token,exit=None,
                    executable_sha256=actual_sha,violations=list(state["violations"]),
                    resources_closed=state["resources_closed"],raw_retained=False,
                    row_incomplete=True,no_child_started=state["process"] is None and not state.get("creation_unknown"))
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
                    state["response_sha256"]=digest(canonical(response)) if type(response) is dict else None
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

def record_call(driver,state,payload,encoded,env,argv,profile,historical,script_gate,
                 output_mode,actual_sha,started,peak,samples,deadline,out,err,offsets,code,limit):
    token=state["token"];p=state["process"];response=None;flushed=True
    if not state["input_fully_delivered"] and state["intentional_interrupt"] is None and code!=86:
        state["violations"].append("incomplete-input-delivery")
    if p is not None:
        for name in ("stdout","stderr"):
            if not state[name+"_eof"]:
                state["violations"].append("raw-pipe-EOF-not-observed:"+name)
    for name,target in (("stdout",out),("stderr",err)):
        try:
            while offsets[name]<len(target):
                end=min(len(target),offsets[name]+65536)
                try:
                    driver.evidence.raw(token,name,offsets[name],bytes(target[offsets[name]:end]))
                    offsets[name]=end
                finally:
                    if profile.get("api_version")==2:offsets[name]=driver.evidence.accepted(token,name)
        except BaseException as error:
            state["violations"].append("raw-retry-"+name+":"+repr(error))
    if profile.get("api_version")==2:
        try:flushed=driver.evidence.flush_call(token)
        except BaseException as error:
            flushed=False;state["violations"].append("raw-coalescer-flush:"+repr(error))
    try:
        driver.evidence.append(dict(kind="raw-complete",call_token=token,
        stdout_bytes=len(out),stdout_sha256=digest(out),stderr_bytes=len(err),
        stderr_sha256=digest(err),stdout_eof=state["stdout_eof"],stderr_eof=state["stderr_eof"],
        process_reaped=p is not None and p.returncode is not None,no_child_started=p is None and not state.get("creation_unknown"),
        input_bytes_expected=len(encoded),input_bytes_sent=state["input_bytes_sent"],
        stdin_closed=state["stdin_closed"],input_fully_delivered=state["input_fully_delivered"]))
    except BaseException as error:
        state["violations"].append("raw-complete-sink:"+repr(error))
    try:
        out_text=bytes(out).decode("utf-8");err_text=bytes(err).decode("utf-8")
        if output_mode=="json" and out and not state["intentional_interrupt"]:
            response=strict(bytes(out),profile["stdout_bytes"])
        elif output_mode=="raw":
            response=dict(raw_stdout_sha256=digest(out),raw_stderr_sha256=digest(err))
        elif (not out and code!=86 and not state["intentional_interrupt"]
              and not(p is not None and p.returncode==86 and profile.get("observed_exit_codes")==[86])):
            raise Refusal("missing-complete-JSON-response")
    except BaseException as error:
        out_text=bytes(out).decode("utf-8",errors="replace")
        err_text=bytes(err).decode("utf-8",errors="replace")
        parse_error=repr(error);state["violations"].append("strict-output:"+parse_error)
    if deadline.remaining()<=0 and "time-censor" not in state["violations"]:
        state["violations"].append("time-censor")
    if historical and historical["expected_error"] is not None:
        if not(type(response) is dict and response.get("status")=="error" and response.get("error")==historical["expected_error"]):
            state["violations"].append("historical-exact-refusal-mismatch")
    if type(response) is dict and response.get("peak_memory_bytes") is not None:
        if not integer(response["peak_memory_bytes"]) or response["peak_memory_bytes"]>limit:
            state["violations"].append("reported-process-peak-RSS-censor")
    done=time.perf_counter_ns()
    row=dict(input=payload,env=copy.deepcopy(env),executable=argv[0],
        executable_sha256=actual_sha,source_pin=driver.final_pin,suite_pin=driver.final_pin,
        binary_source_pin=historical["binary_source_pin"] if historical else (None if script_gate else driver.final_pin),
        binary_role="historical" if historical else "python-argv" if script_gate else profile.get("producer_kind","candidate"),call_token=token,
        exit=None if p is None else p.returncode,stdout=out_text,stderr=err_text,
        stdout_bytes=len(out),stderr_bytes=len(err),
        stdout_sha256=digest(out),stderr_sha256=digest(err),violations=list(state["violations"]),
        positive_start_event=state.get("positive_start_event"),rss_limit=limit,
        observed_peak_rss=peak,memory_sample_count=samples,
        input_bytes_expected=len(encoded),input_bytes_sent=state["input_bytes_sent"],
        first_write_qpc=state.get("first_write_qpc"),
        input_wire_sha256=digest(encoded),input_written_prefix_sha256=digest(encoded[:state["input_bytes_sent"]]),
        input_transfer_accounting_complete=state["input_transfer_accounting_complete"],
        stdin_started_ns=state.get("stdin_started_ns"),input_complete_ns=state.get("input_complete_ns"),
        stdin_closed=state["stdin_closed"],input_fully_delivered=state["input_fully_delivered"],
        input_delivery_scope="completed native writes plus stdin closure; no child-consumption inference",
        process_started_ns=started,process_resumed_ns=state.get("resumed_ns"),
        process_done_ns=done,spawn_total_ns=done-started,
        intentional_interrupt=state["intentional_interrupt"],termination_requests=state["termination_requests"],
        native_identity=state.get("public_identity"),resources_closed=state["resources_closed"],
        ownership=state["ownership"],termination_retired=state["termination_retired"],
        no_child_started=p is None and not state.get("creation_unknown"),popen_reaped=state["popen_reaped"],
        popen_handle_closed=state["popen_handle_closed"],stdout_eof=state["stdout_eof"],
        stderr_eof=state["stderr_eof"],raw_retained=False,
        owner_completion_fence="wait_control-done-event-required",
        python_callback_joined=state["callback_joined"],historical_invocation=historical,
        raw_before_strict_parse=True,context=state["context"],
        outer_Job_closure="separate-accepted-QPC-final-receipt-required")
    try:
        row["raw_retained"]=flushed and offsets["stdout"]==len(out) and offsets["stderr"]==len(err)
        state["raw_reference"]=driver.evidence.append(dict(kind="native-call",**row))
        state["raw_row_written"]=True
    except BaseException as error:
        state["violations"].append("native-row-sink:"+repr(error))
        row["raw_retained"]=False
        row["violations"]=list(state["violations"])
    return response,row


class NativeCommands:
    """Two source-owned argv families; all bytes pass through WindowsDriver."""
    def __init__(self,adapter):self.a,self.host=adapter,adapter.host
    def run(self,root,label,argv,stdin=b"",*,expected=0,seconds=120,env=None):
        from windows_native import ORDINARY_PS
        import base64
        root=Path(root);_,case=self.a.case_for(root)
        need(label in ("workspace","ordinary") and type(argv) is list
            and type(stdin) is bytes and len(stdin)<=4194304,"closed-native-command-family")
        tools=self.host.grant["native_tools"];environment={}
        if label=="workspace":
            spec=self.a.m["workspace"];tool=tools["cargo"]
            common=[tool["path"],"test","--manifest-path",spec["manifest_path"],"--locked","--offline",
                "--workspace","--release","-j2","--target-dir",spec["target_dir"]]
            need(argv in (common+["--","--list"],common+["--","--test-threads=1"])
                and stdin==b"","all-workspace-unfiltered-command")
            operation="list" if argv[-1]=="--list" else "run"
            need(expected==0,"workspace-success-required")
            cwd=str(Path(spec["manifest_path"]).parent)
            fixture=str(root.parent/"unit-fixtures")
            expected_env=dict(EXP_PACK_TEST_ROOT=fixture,TMP=fixture,TEMP=fixture,
                CARGO_INCREMENTAL="0",CARGO_NET_OFFLINE="true",CARGO_HOME=spec["cargo_home"])
            need(env==expected_env,"source-owned-Windows-workspace-environment")
            environment=expected_env
        else:
            tool=tools["powershell"];operation="run";cwd=str(root.parent)
            need(expected in (0,23),"ordinary-source-owned-expected-exit")
            command=base64.b64encode(ORDINARY_PS.encode("utf-16le")).decode("ascii")
            need(argv==[tool["path"],"-NoProfile","-NonInteractive","-EncodedCommand",command]
                and stdin==b"" and set(env or {})=={"FILE_NATIVE_ORDINARY"},"fixed-native-PowerShell-program")
            data=strict(env["FILE_NATIVE_ORDINARY"].encode(),8192)
            need(set(data)=={"root","destination","kind","selected","label"}
                and data["root"]==str(root) and data["kind"] in ("payload","title")
                and type(data["label"]) is str and 1<=len(data["label"])<=120
                and all(c.isalnum() or c in "-_" for c in data["label"])
                and Path(data["destination"])==root.parent/("ordinary-"+data["label"]),
                "source-owned-PowerShell-data-and-output")
            self.host.ledger.root_name(data["destination"]);environment=env
        need(file_hash(tool["path"])==tool["sha256"],"actual-native-command-executable")
        profile=copy.deepcopy(self.host.grant["native_command_profiles"][case["case_id"]][label][operation])
        need(profile["producer_kind"]=="native-argv" and profile["role"]=="checker"
            and seconds<=profile["timeout_seconds"] and profile["cwd"]==cwd,
            "source-authenticated-command-profile")
        d=self.a.driver;d.context.value=dict(root=str(root),case_id=case["case_id"],
            scope_id=case["scope_id"],case_scope=case["case_scope"])
        payload=dict(root=str(root),action="native-command",family=label,operation=operation,
            argv_sha256=digest(canonical(argv)),stdin_sha256=digest(stdin),stdin_bytes=len(stdin))
        lease="native-command:"+uuid.uuid4().hex
        self.host.ledger.reserve(lease,profile["growth"])
        try:
            result=d._execute(argv,payload,stdin,environment,expected,seconds,None,profile,
                output_mode="raw",executable_sha256=tool["sha256"])
        finally:self.host.ledger.settle(lease)
        row=d.context.completed_row;token=d.context.completed_token
        need(not row["violations"] and row["resources_closed"] and row["raw_retained"],
            "actual-native-command-uncensored-closure")
        return dict(stdout=row["stdout"],stderr=row["stderr"],exit=row["exit"],token=token,
            raw_ref=copy.deepcopy(d.registry[token]["raw_reference"]),metadata_ref=self.host.events.calls[token]["meta_ref"])


class NativeBudget:
    def __init__(self,adapter):self.a=adapter
    def reserve_at(self,key,path,maximum,files,directories):
        category=self.a.host.ledger.root_name(path)
        return self.a.host.ledger.reserve(key,{category:dict(bytes=maximum,files=files,directories=directories)})
    def reserve_control(self,key,maximum,files=1,directories=0):
        context=getattr(self.a.driver.context,"value",{})
        root=context.get("root");need(root is not None,"owned-budget-context")
        return self.reserve_at(key,root,maximum,files,directories)
    def reserve_evidence(self,key,maximum,files=1):
        root=self.a.host.ledger.roots[self.a.host.grant["events_category"]]["root"]
        return self.reserve_at(key,root,maximum,files,0)
    def settle(self,key):return self.a.host.ledger.settle(key)


def mapping_stress_case(host,root,spec):
    return host.harness.mapping_stress(root,spec["forecast"],count=1000,native_focused=False)

def checker_native(host,c):
    state=host.driver.registry[c["call_token"]];row=host.driver.context.completed_row
    need(state.get("checker_writes_retired") is True,"checker-consumer-settled-output")
    oracle=[v["sha256"] for p,v in c["index"]["sources"].items() if Path(p).name=="physical_decoder_v5.py"]
    need(len(oracle)==1,"one-actual-legacy-oracle-source")
    return dict(exit_code=row["exit"],source_pin=host.final_pin,binary_sha256=row["executable_sha256"],
        contained_before_input=state["public_identity"]["same_job"],children_empty=state["resources_closed"],
        closure_scope="owned-call-process-IO-handles",budget_receipt_sha256=c["budget_ref"]["sha256"],
        closure_receipt_sha256=c["closure_ref"]["sha256"],raw=c["native_raw"],
        response=host.driver.context.completed_response,engine_ns=None,spawn_total_ns=row["spawn_total_ns"],
        observed_peak_rss=row["observed_peak_rss"],startup_gate_before_corpus_IO=True,
        oracle_sha256=oracle[0],output=c["index_ref"])
