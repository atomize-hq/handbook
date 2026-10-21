import base64,copy,gzip,hashlib,threading,time,uuid
from pathlib import Path
from windows_deadlines import canonical,digest,integer,need,Refusal,strict,project_response
from windows_growth import confined,file_hash,inventory
from windows_native import counter

class Evidence:
    def __init__(self,path,ledger,category,*,max_bytes,max_files=1,binding=None):
        self.path=confined(Path(ledger.roots[category]["root"]),Path(path))
        need(self.path.parent.is_dir() and not self.path.exists(),"fresh-evidence")
        need(integer(max_bytes,1) and max_files==1,"one-bounded-stream")
        self.ledger,self.category=ledger,category
        self.max_bytes,self.written,self.count=max_bytes,0,0
        self.lock,self.binding=threading.RLock(),copy.deepcopy(binding or {})
        need(len(canonical(self.binding))<=2048,"bounded-stream-binding")
        self.v2=self.binding.get("capture_schema")=="single-raw-ref-v1"
        self.run=self.binding.get("outer_run_id",uuid.uuid4().hex);self.context=None;self.profiles=None;self.bridge_ref=None
        self.pending={};self.calls={};self.closed=False;self.poisoned=False
        self.key="evidence:"+str(self.path)
        ledger.reserve(self.key,{category:dict(bytes=max_bytes,files=1,directories=0)})
        try:
            with self.path.open("xb"):pass
        except BaseException:
            ledger.settle(self.key);raise
        if self.v2:
            self.header_ref=self._write(dict(v=2,kind="stream",run=self.run,source_pin=self.binding["final_pin"],
                binary_sha=self.binding["binary_sha256"],guard_sha=self.binding["guard_sha256"],
                driver_sha=self.binding["driver_sha256"],profile_sha=self.binding["profile_sha256"]))

    def _write(self,value,token=None,frame=False):
        with self.lock:
            need(not self.closed and not self.poisoned,"writable-raw-stream")
            value=copy.deepcopy(value)
            c=self.calls.get(token)
            if frame:
                need(c is not None and not c["ended"],"live-call-frame")
                value.update(v=2,call=token,seq=c["frames"])
            raw=canonical(value)+b"\n"
            if token is not None:need(len(raw)<=self.pending[token],"reserved-call-frame")
            extra=0 if token is not None else len(raw)
            need(self.written+sum(self.pending.values())+extra<=self.max_bytes,"all-potential-before-append")
            begin=self.written
            try:
                with self.path.open("ab",buffering=0) as output:
                    at=0
                    while at<len(raw):
                        n=output.write(raw[at:]);need(type(n) is int and n>0,"positive-evidence-write")
                        at+=n;self.written+=n
                        if token is not None:self.pending[token]-=n
                    output.flush()
            except BaseException:
                self.poisoned=True;raise
            self.count+=1
            if frame:
                c["hash"].update(raw);c["bytes"]+=len(raw);c["frames"]+=1;c["end"]=self.written
                if value["kind"]=="end":c["ended"]=True
                if value["kind"]=="start":c["start"]=copy.deepcopy(value)
            return dict(path=str(self.path),offset=begin,bytes=len(raw),sha256=digest(raw))

    def reserve_call(self,token,maximum):
        with self.lock:
            need(integer(maximum,1) and token not in self.pending,"unique-bounded-call")
            need(self.written+sum(self.pending.values())+maximum<=self.max_bytes,"whole-call-before-dispatch")
            self.pending[token]=maximum

    def prepare_pending(self,token,root,action,profile):
        from windows_growth import profile_reservation_v2
        need(token not in self.calls and profile["profile_sha256"]==self.binding["profile_sha256"],
             "profile-before-intent")
        maximum=profile_reservation_v2(profile)
        self._ensure_bridge()
        self.reserve_call(token,maximum)
        self.calls[token]=dict(hash=hashlib.sha256(),begin=self.written,end=self.written,bytes=0,
            frames=0,ended=False,start=None,start_ref=None,launch_ref=None,root=str(root),action=action,profile=copy.deepcopy(profile),
            encoded=None,env=None,intent=None,adopted=False,writer_closed=False,meta_ref=None,
            streams={s:dict(accepted=0,written=0,buffer=bytearray(),final=False) for s in ("in","out","err")})
        return token

    def begin_call(self,token,payload,encoded,env,profile):
        with self.lock:
            if token not in self.calls:self.prepare_pending(token,payload.get("root",""),payload.get("action","script"),profile)
            c=self.calls[token]
            need(not c["adopted"] and c["profile"]["profile_sha256"]==profile["profile_sha256"],
                 "unadopted-profile")
            if c["encoded"] is None:self._intent(token,payload,encoded,env)
            need(c["encoded"]==encoded and c["env"]==env and c["input_sha"]==digest(encoded),
                 "adopt-exact-token-planned-bytes-env")
            c["adopted"]=True

    def _intent(self,token,payload,encoded,env):
        c=self.calls[token];need(c["encoded"] is None,"planned-input-once")
        need(type(encoded) is bytes and len(encoded)<=4*1024**2 and len(canonical(env))<=8192,"bounded-planned-input-env")
        c.update(encoded=encoded,env=copy.deepcopy(env),input_sha=digest(encoded),input_n=len(encoded))
        self._write(dict(kind="begin",root=str(payload.get("root",c["root"])),action=c["action"],
            role="reader" if c["action"] in ("resolve","point","inspect","lookup-bench") else "checker",
            env=copy.deepcopy(env),input_n=len(encoded),input_sha=c["input_sha"],input_scope="planned",
            pid=None,creation=None,qpc_start=counter()),token,True)
        self.raw(token,"stdin",0,encoded);self.flush_stream(token,"in",True)
        c["intent"]=dict(schema="planned-input-ref-v1",path=str(self.path),run=self.run,
            call=token,begin=c["begin"],input_n=len(encoded),input_sha=c["input_sha"])
        return copy.deepcopy(c["intent"])

    def accepted(self,token,stream):
        return self.calls[token]["streams"][{"stdin":"in","stdout":"out","stderr":"err"}[stream]]["accepted"]

    def raw(self,token,stream,offset,block):
        name={"stdin":"in","stdout":"out","stderr":"err"}[stream]
        with self.lock:
            c=self.calls[token];s=c["streams"][name]
            need(not c["ended"] and not s["final"] and offset==s["accepted"] and type(block) is bytes,
                 "continuous-live-raw-acceptance")
            for at in range(0,len(block),16384):
                self.flush_stream(token,name,False)
                piece=block[at:at+16384]
                room=16384-len(s["buffer"])
                first=piece[:room];s["buffer"].extend(first);s["accepted"]+=len(first)
                self.flush_stream(token,name,False)
                tail=piece[room:]
                if tail:s["buffer"].extend(tail);s["accepted"]+=len(tail)
            self.flush_stream(token,name,False)

    def flush_stream(self,token,name,final):
        with self.lock:
            s=self.calls[token]["streams"][name]
            while len(s["buffer"])>=16384 or final and s["buffer"]:
                part=bytes(s["buffer"][:16384])
                self._write(dict(kind="data",stream=name,offset=s["written"],n=len(part),
                    sha=digest(part),b64=base64.b64encode(part).decode("ascii")),token,True)
                del s["buffer"][:len(part)];s["written"]+=len(part)
            if final:s["final"]=True

    def flush_call(self,token):
        for name in ("in","out","err"):self.flush_stream(token,name,True)
        return all(s["accepted"]==s["written"] and not s["buffer"] for s in self.calls[token]["streams"].values())

    def call_reference(self,token):
        with self.lock:
            c=self.calls[token];need(c["ended"],"closed-frame-reference")
            return dict(schema="single-raw-ref-v1",path=str(self.path),run=self.run,call=token,
                begin=c["begin"],end=c["end"],frames=c["frames"],bytes=c["bytes"],sha256=c["hash"].hexdigest())

    def append(self,value):
        value=copy.deepcopy(value);kind=value.get("kind");token=value.get("call_token")
        if kind=="request-before-dispatch":
            token=getattr(self.context,"pending_token",None)
            need(token in self.calls,"owned-pending-intent")
            ref=self._intent(token,value["input"],canonical(value["input"]),value["env"])
            value={k:v for k,v in value.items() if k not in ("input","env")}
            value["input_ref"]=ref
            launch=self.calls[token]["profile"]["launch"]
            self.launch(token,**launch)
            return self._write(value,token)
        if kind=="raw-complete":return dict(covered_by="canonical-END")
        if kind=="native-RSS":
            c=self.calls[token]
            return self._write(dict(kind="rss",pid=value["pid"],creation=value["creation_time"],
                qpc=counter(),rss=value["rss"],peak=value["peak"],
                role="reader" if value["rss_limit"]==268435456 else "checker",
                phase="sample" if c["start"] else "suspended"),token,True)
        if kind=="positive-native-start":
            ref=self._write(dict(kind="start",pid=value["pid"],creation=value["creation_time"],
                qpc=counter(),rss=value["positive_rss"],executable_sha256=value["executable_sha256"]),token,True)
            self.calls[token]["start_ref"]=ref
            return ref
        if kind=="native-call":
            need(self.flush_call(token),"all-accepted-raw-flushed-before-END")
            end=dict(kind="end",exit=value["exit"],in_n=value["input_bytes_sent"],
                out_n=value["stdout_bytes"],err_n=value["stderr_bytes"],
                in_sha=value["input_written_prefix_sha256"],out_sha=value["stdout_sha256"],
                err_sha=value["stderr_sha256"],input_complete=value["input_fully_delivered"],
                out_eof=value["stdout_eof"],err_eof=value["stderr_eof"],
                rss_n=value["memory_sample_count"],peak=value["observed_peak_rss"],qpc_end=counter(),
                first_write_qpc=value["first_write_qpc"],native_closed=value["resources_closed"],
                io_closed=value["resources_closed"],callbacks_joined=value["python_callback_joined"],
                intent=value["intentional_interrupt"],violations=value["violations"])
            end_ref=self._write(end,token,True);ref=self.call_reference(token)
            c=self.calls[token];context=value.get("context") or {}
            launch=c["launch_value"];start=c["start"]
            metadata=dict(schema="native-call-meta-v2",call=token,run=self.run,raw_ref=ref,
                source_pin=self.binding["final_pin"],binary_sha256=launch[1],
                producer_kind=launch[2],producer_source_pin=launch[3],
                identity=dict(pid=start["pid"],creation=start["creation"],executable_sha256=start["executable_sha256"]) if start else None,
                process_started_ns=value["process_started_ns"],process_done_ns=value["process_done_ns"],
                spawn_total_ns=value["spawn_total_ns"],source_proof_ref=self.header_ref,
                image_proof_ref=c["start_ref"],
                qpc_frequency=self.qpc_frequency,clock_bridge_ref=self.bridge_ref,closure_ref=end_ref,
                case_id=context.get("case_id"),scope_id=context.get("scope_id"),
                case_scope=context.get("case_scope"))
            need(len(canonical(metadata))+1<=c["profile"]["native_metadata_max_bytes"],"typed-native-meta-width")
            c["meta_ref"]=self._write(metadata,token)
            c["writer_closed"]=value["resources_closed"] and value["python_callback_joined"]
            return ref
        if self.context is not None:
            completed=getattr(self.context,"completed_token",None)
            pending=getattr(self.context,"pending_token",None)
            selected=completed if completed in self.calls and self.calls[completed]["ended"] else pending
            if selected in self.calls:
                c=self.calls[selected]
                if "input" in value:
                    need(c["input_sha"]==digest(canonical(value["input"])),"same-native-intent-projection")
                    value.pop("input");value.pop("env",None);value["input_ref"]=copy.deepcopy(c["intent"])
                if c["ended"]:
                    ref=self.call_reference(selected)
                    if "raw_driver_row" in value:
                        need(value["raw_driver_row"]["call_token"]==selected,"same-driver-row")
                        value["raw_driver_row"]=dict(schema="native-call-row-ref-v1",raw_ref=ref,
                            metadata_ref=c["meta_ref"])
                    value=project_response(value,ref,getattr(self.context,"completed_response",None),
                        max_bytes=16*1024**2)
        return self._write(value)

    def _ensure_bridge(self):
        if self.bridge_ref is not None:return
        from windows_native import bridge,frequency
        body=bridge();self.qpc_frequency=frequency()
        self.bridge_ref=self._write(dict(schema="native-qpc-bridge-v1",run=self.run,
            frequency=self.qpc_frequency,**body))

    def launch(self,token,argv,executable_sha256,producer_kind,producer_source_pin,source_files,startup_gate):
        c=self.calls[token];p=c["profile"]
        if c["launch_ref"] is not None:
            need(c["launch_value"]==(argv,executable_sha256,producer_kind,producer_source_pin,source_files,startup_gate),
                 "exact-adopted-launch")
            return c["launch_ref"]
        need(type(source_files) is list,"authenticated-launch-sources")
        need(len(canonical(argv))<=p["argv_max_bytes"] and len(canonical(source_files))<=p["source_files_max_bytes"],
             "action-bound-launch-width")
        for item in source_files:
            need(set(item)=={"path","sha256"} and file_hash(item["path"])==item["sha256"],"actual-producer-source-file")
        c["launch_value"]=copy.deepcopy((argv,executable_sha256,producer_kind,producer_source_pin,source_files,startup_gate))
        c["launch_ref"]=self._write(dict(kind="launch",argv=copy.deepcopy(argv),
            executable_sha256=executable_sha256,producer_kind=producer_kind,
            producer_source_pin=producer_source_pin,source_files=copy.deepcopy(source_files),
            startup_gate_sha256=digest(canonical(startup_gate)) if startup_gate else None),token,True)
        return c["launch_ref"]

    def abort_pending(self,token,payload,env,binary_sha,source_pin,error):
        from windows_growth import abort_pending
        return abort_pending(self,token,payload,env,binary_sha,source_pin,error)

    def finish_call(self,token):
        with self.lock:
            c=self.calls[token]
            need(c["ended"] and c["writer_closed"] and all(not s["buffer"] for s in c["streams"].values()),
                 "closed-flushed-writer")
            del self.pending[token]
            c["encoded"]=None

    def reference(self):
        return dict(path=str(self.path),bytes=self.written,rows=self.count,
            sha256=file_hash(self.path,self.max_bytes),lossless=not self.poisoned,
            schema="single-raw-stream-v1",run=self.run)

    def close(self):
        with self.lock:
            self.closed=True
            need(not self.pending and not self.poisoned,"all-owned-writers-and-prefixes-closed")
            proof=self.settlement_proof()
            self.ledger.settle(self.key,{self.category:dict(bytes=self.written,files=1,directories=0)},proof)
            return self.reference()


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
        process_reaped=p is not None and p.returncode is not None,no_child_started=p is None,
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
        binary_role="historical" if historical else "python-argv" if script_gate else "candidate",call_token=token,
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
        no_child_started=p is None,popen_reaped=state["popen_reaped"],
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

