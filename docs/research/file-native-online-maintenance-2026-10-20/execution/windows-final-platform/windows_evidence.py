import base64,copy,gzip,hashlib,threading,time,uuid
from pathlib import Path
from windows_deadlines import canonical,digest,integer,need,Refusal,strict,project_response
from windows_growth import confined,file_hash,inventory,regular_read,authenticate_action_profile
from windows_native import counter
from windows_hooks import adopt_capture_profile,SIDEBAND_KINDS

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

    def prepare_pending(self,token,root,action,profile,planned_launch=None):
        from windows_growth import profile_reservation_v2
        need(token not in self.calls,"profile-before-intent")
        authenticate_action_profile(profile,self.profiles,self.binding["profile_sha256"])
        maximum=profile_reservation_v2(profile)
        self._ensure_bridge()
        self.reserve_call(token,maximum)
        self.calls[token]=dict(hash=hashlib.sha256(),begin=self.written,end=self.written,bytes=0,
            frames=0,ended=False,start=None,start_ref=None,launch_ref=None,root=str(root),action=action,profile=copy.deepcopy(profile),
            encoded=None,env=None,intent=None,adopted=False,writer_closed=False,meta_ref=None,planned_launch=copy.deepcopy(planned_launch),
            streams={s:dict(accepted=0,written=0,buffer=bytearray(),final=False) for s in ("in","out","err")})
        return token

    def begin_call(self,token,payload,encoded,env,profile):
        with self.lock:
            if token not in self.calls:self.prepare_pending(token,payload.get("root",""),payload.get("action","script"),profile)
            c=self.calls[token]
            need(not c["adopted"],"unadopted-profile")
            adopt_capture_profile(self.profiles,self.binding["profile_sha256"],c["profile"],profile)
            c["profile"]=copy.deepcopy(profile)
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
            launch=self.calls[token]["planned_launch"]
            self.launch(token,**launch)
            from windows_growth import capture_sideband
            return capture_sideband(self,value,token)
        if kind=="raw-complete":return dict(covered_by="canonical-END")
        if kind=="native-RSS":
            c=self.calls[token]
            return self._write(dict(kind="rss",pid=value["pid"],creation=value["creation_time"],
                qpc=counter(),rss=value["rss"],peak=value["peak"],
                role="reader" if value["rss_limit"]==268435456 else "checker",
                phase="running" if c["start"] else "suspended"),token,True)
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
            launch=c.get("launch_value") or ([],value["executable_sha256"],"candidate",value["source_pin"],[],None);start=c["start"]
            metadata=dict(schema="native-call-meta-v2",call=token,run=self.run,raw_ref=ref,
                source_pin=self.binding["final_pin"],binary_sha256=launch[1],
                producer_kind=launch[2],producer_source_pin=launch[3],
                identity=dict(pid=start["pid"],creation=start["creation"],executable_sha256=start["executable_sha256"]) if start else None,
                process_started_ns=value["process_started_ns"],process_done_ns=value["process_done_ns"],
                spawn_total_ns=value["spawn_total_ns"],source_proof_ref=self.header_ref,
                image_proof_ref=c["start_ref"],
                qpc_frequency=self.qpc_frequency,clock_bridge_ref=self.bridge_ref,closure_ref=end_ref,
                case_id=context.get("case_id"),scope_id=context.get("scope_id"),
                case_scope=context.get("case_scope"),action_profile_id=c["profile"]["profile_id"],
                action_profile_sha256=c["profile"]["profile_sha256"])
            need(len(canonical(metadata))+1<=c["profile"]["native_metadata_max_bytes"],"typed-native-meta-width")
            c["meta_ref"]=self._write(metadata,token)
            c["writer_closed"]=value["resources_closed"] and value["python_callback_joined"]
            return ref
        if kind=="callback-cleanup":value=self._callback_projection(value)
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
        from windows_growth import capture_sideband
        need(kind in SIDEBAND_KINDS,"source-owned-sideband-kind")
        return capture_sideband(self,value)

    def state_reference(self,value):
        from windows_growth import state_reference
        return state_reference(self,value)


    def _callback_projection(self,value):
        responses=value["responses"];n=value["spawned"]
        need(type(n) is int and 0<=n<=128 and len(responses)==n==value["observed_waits"],
            "source-callback-wait-count")
        queued=getattr(self.context,"callback_waits",[])
        rows=queued[-n:] if n else []
        need(len(rows)==n,"actual-ordered-callback-waits")
        if n:del queued[-n:]
        scope=getattr(self.context,"value",{}).get("case_scope");refs=[]
        def walk(v,handle):
            if type(v) is dict:
                if set(v)=={"status","result","error","metrics","wall_ns","peak_memory_bytes"}:
                    wanted=digest(canonical(v));matches=[]
                    for token in handle["native_tokens"]:
                        s=self.driver_registry[token]
                        if s["context"].get("handle_id")==handle["id"] and s["context"].get("case_scope")==scope:
                            if s.get("response_sha256")==wanted and s["done"].is_set() and s["resources_closed"]:
                                matches.append(s["raw_reference"])
                    need(len(matches)==1,"one-actual-callback-native-response")
                    return dict(schema="native-response-ref-v1",raw_ref=copy.deepcopy(matches[0]))
                return {k:walk(x,handle) for k,x in v.items()}
            if type(v) is list:return [walk(x,handle) for x in v]
            return v
        out=[]
        for response,row in zip(responses,rows):
            handle=self.callback_handles[row["handle_id"]]
            need(handle["case_scope"]==scope
                and row["response_sha256"]==digest(canonical(response)),"same-wait-return-and-case")
            out.append(walk(response,handle));refs.append(row["proof_ref"])
        value["responses"]=out;value["callback_wait_refs"]=refs
        return value

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



WIRE_CONTRACT={'api':'windows-capture-v2-r3','status':'RAM_CONTRACT_NOT_RUNTIME_GO','encoding':'UTF8 sorted compact JSON allow_nan_false plus LF; RFC4648 base64 no LF','parameters':{'tape_path_bytes':[1,32767],'root_bytes':[1,32767],'env_json_bytes':[2,8192],'argv_json_bytes':[2,131072],'source_files_json_bytes':[2,131072],'sideband_json_bytes':[2,4194304],'case_id_bytes':[1,256],'scope_id_bytes':[1,256],'reason_bytes':[1,8192],'case_call_count':[1,100000],'case_page_count':[1,1024],'proof_json_bytes':[2,4194304],'action_profile_id_bytes':[1,256]},'types':{'range_ref':{'object':{'path':{'string':{'parameter':'tape_path_bytes'}},'offset':{'uint':64},'bytes':{'uint':64},'sha256':{'ascii':64}}},'raw_ref':{'object':{'schema':{'literal':'single-raw-ref-v1'},'path':{'string':{'parameter':'tape_path_bytes'}},'run':{'ascii':32},'call':{'ascii':32},'begin':{'uint':64},'end':{'uint':64},'frames':{'uint':64},'bytes':{'uint':64},'sha256':{'ascii':64}}},'planned_ref':{'object':{'schema':{'literal':'planned-input-ref-v1'},'path':{'string':{'parameter':'tape_path_bytes'}},'run':{'ascii':32},'call':{'ascii':32},'begin':{'uint':64},'input_n':{'uint':64},'input_sha':{'ascii':64}}},'identity':{'object':{'pid':{'uint':32},'creation':{'uint':64},'executable_sha256':{'ascii':64}}},'script_gate':{'object':{'ready':{'string':{'parameter':'tape_path_bytes'}},'release':{'string':{'parameter':'tape_path_bytes'}},'nonce':{'ascii':32},'manifest_sha256':{'ascii':64},'root':{'string':{'parameter':'root_bytes'}}}}},'record_schemas':{'stream':{'object':{'v':{'literal':2},'kind':{'literal':'stream'},'run':{'ascii':32},'source_pin':{'ascii':40},'binary_sha':{'ascii':64},'guard_sha':{'ascii':64},'driver_sha':{'ascii':64},'profile_sha':{'ascii':64}}},'begin':{'object':{'v':{'literal':2},'kind':{'literal':'begin'},'call':{'ascii':32},'seq':{'uint':64},'root':{'string':{'parameter':'root_bytes'}},'action':{'ascii':32},'role':{'ascii':16},'env':{'bytes':{'parameter':'env_json_bytes'}},'input_n':{'uint':64},'input_sha':{'ascii':64},'input_scope':{'literal':'planned'},'pid':{'oneof':[{'uint':32},{'literal':None}]},'creation':{'oneof':[{'uint':64},{'literal':None}]},'qpc_start':{'uint':64}}},'launch':{'object':{'v':{'literal':2},'kind':{'literal':'launch'},'call':{'ascii':32},'seq':{'uint':64},'argv':{'bytes':{'parameter':'argv_json_bytes'}},'executable_sha256':{'ascii':64},'producer_kind':{'oneof':[{'literal':'candidate'},{'literal':'historical'},{'literal':'python-argv'},{'literal':'native-argv'}]},'producer_source_pin':{'ascii':40},'source_files':{'bytes':{'parameter':'source_files_json_bytes'}},'startup_gate_sha256':{'oneof':[{'ascii':64},{'literal':None}]}}},'start':{'object':{'v':{'literal':2},'kind':{'literal':'start'},'call':{'ascii':32},'seq':{'uint':64},'pid':{'uint':32},'creation':{'uint':64},'qpc':{'uint':64},'rss':{'uint':32},'executable_sha256':{'ascii':64}}},'data':{'object':{'v':{'literal':2},'kind':{'literal':'data'},'call':{'ascii':32},'seq':{'uint':64},'stream':{'oneof':[{'literal':'in'},{'literal':'out'},{'literal':'err'}]},'offset':{'uint':64},'n':{'uint':64},'sha':{'ascii':64},'b64':{'literal':''}}},'rss':{'object':{'v':{'literal':2},'kind':{'literal':'rss'},'call':{'ascii':32},'seq':{'uint':64},'pid':{'uint':32},'creation':{'uint':64},'qpc':{'uint':64},'rss':{'uint':32},'peak':{'uint':32},'role':{'oneof':[{'literal':'reader'},{'literal':'checker'}]},'phase':{'oneof':[{'literal':'suspended'},{'literal':'running'}]}}},'end':{'object':{'v':{'literal':2},'kind':{'literal':'end'},'call':{'ascii':32},'seq':{'uint':64},'exit':{'oneof':[{'uint':32},{'literal':None}]},'in_n':{'uint':64},'out_n':{'uint':64},'err_n':{'uint':64},'in_sha':{'ascii':64},'out_sha':{'ascii':64},'err_sha':{'ascii':64},'input_complete':{'oneof':[{'literal':True},{'literal':False}]},'out_eof':{'oneof':[{'literal':True},{'literal':False}]},'err_eof':{'oneof':[{'literal':True},{'literal':False}]},'rss_n':{'uint':64},'peak':{'uint':32},'qpc_end':{'uint':64},'first_write_qpc':{'oneof':[{'uint':64},{'literal':None}]},'native_closed':{'oneof':[{'literal':True},{'literal':False}]},'io_closed':{'oneof':[{'literal':True},{'literal':False}]},'callbacks_joined':{'oneof':[{'literal':True},{'literal':False}]},'intent':{'oneof':[{'literal':None},{'literal':'cancel'},{'literal':'death'}]},'violations':{'array':{'count':16,'item':{'string':{'parameter':'reason_bytes'}}}}}},'bridge':{'object':{'schema':{'literal':'native-qpc-bridge-v1'},'run':{'ascii':32},'frequency':{'uint':64},'qpc_before':{'uint':64},'perf_ns':{'uint':64},'qpc_after':{'uint':64}}},'native_meta':{'object':{'schema':{'literal':'native-call-meta-v2'},'call':{'ascii':32},'run':{'ascii':32},'raw_ref':{'named':'raw_ref'},'process_started_ns':{'uint':64},'process_done_ns':{'uint':64},'spawn_total_ns':{'uint':64},'source_pin':{'ascii':40},'binary_sha256':{'ascii':64},'producer_kind':{'ascii':16},'producer_source_pin':{'ascii':40},'identity':{'oneof':[{'named':'identity'},{'literal':None}]},'source_proof_ref':{'named':'range_ref'},'image_proof_ref':{'oneof':[{'named':'range_ref'},{'literal':None}]},'qpc_frequency':{'uint':64},'clock_bridge_ref':{'named':'range_ref'},'closure_ref':{'named':'range_ref'},'case_id':{'string':{'parameter':'case_id_bytes'}},'scope_id':{'string':{'parameter':'scope_id_bytes'}},'case_scope':{'ascii':32},'action_profile_id':{'string':{'parameter':'action_profile_id_bytes'}},'action_profile_sha256':{'ascii':64}}},'sideband':{'object':{'schema':{'literal':'capture-sideband-v1'},'run':{'ascii':32},'kind':{'literal':'sideband'},'sideband_kind':{'ascii':96},'case_id':{'oneof':[{'string':{'parameter':'case_id_bytes'}},{'literal':None}]},'scope_id':{'oneof':[{'string':{'parameter':'scope_id_bytes'}},{'literal':None}]},'case_scope':{'oneof':[{'ascii':32},{'literal':None}]},'payload':{'bytes':{'parameter':'sideband_json_bytes'}},'payload_sha256':{'ascii':64}}},'case_proof':{'object':{'schema':{'literal':'native-case-proof-v1'},'kind':{'ascii':32},'run':{'ascii':32},'source_pin':{'ascii':40},'binary_sha256':{'ascii':64},'budget_sha256':{'ascii':64},'profile_catalogue_sha256':{'ascii':64},'case_id':{'string':{'parameter':'case_id_bytes'}},'scope_id':{'string':{'parameter':'scope_id_bytes'}},'case_scope':{'ascii':32},'root':{'string':{'parameter':'root_bytes'}},'start_ns':{'uint':64},'end_ns':{'uint':64},'clock_bridge_ref':{'named':'range_ref'},'payload':{'bytes':{'parameter':'proof_json_bytes'}}}},'case_index_page':{'object':{'schema':{'literal':'native-case-index-page-v1'},'run':{'ascii':32},'case_id':{'string':{'parameter':'case_id_bytes'}},'scope_id':{'string':{'parameter':'scope_id_bytes'}},'case_scope':{'ascii':32},'ordinal':{'uint':64},'first_call':{'uint':64},'calls':{'array':{'count':128,'item':{'object':{'raw_ref':{'named':'raw_ref'},'meta_ref':{'named':'range_ref'}}}}}}},'case_index':{'object':{'schema':{'literal':'native-case-index-v1'},'run':{'ascii':32},'source_pin':{'ascii':40},'binary_sha256':{'ascii':64},'budget_sha256':{'ascii':64},'profile_catalogue_sha256':{'ascii':64},'case_id':{'string':{'parameter':'case_id_bytes'}},'scope_id':{'string':{'parameter':'scope_id_bytes'}},'case_scope':{'ascii':32},'root':{'string':{'parameter':'root_bytes'}},'start_ns':{'uint':64},'end_ns':{'uint':64},'total_calls':{'uint':64},'pages':{'array':{'count':{'parameter':'case_page_count'},'item':{'named':'range_ref'}}},'ready_refs':{'array':{'count':4,'item':{'named':'range_ref'}}},'origin_ref':{'oneof':[{'named':'range_ref'},{'literal':None}]},'case_closure_ref':{'named':'range_ref'},'owned_drain_ref':{'named':'range_ref'},'checkpoint_ref':{'named':'range_ref'}}},'physical_state_ref':{'object':{'schema':{'literal':'physical-state-ref-v1'},'record_ref':{'named':'range_ref'},'source_proof_ref':{'named':'range_ref'},'run':{'ascii':32},'case_id':{'string':{'parameter':'case_id_bytes'}},'scope_id':{'string':{'parameter':'scope_id_bytes'}},'case_scope':{'ascii':32},'state_sha256':{'ascii':64}}}},'invariants':{'chunk_bytes':16384,'coalescing':'full16384 then one partial per stream only at EOF/failure; RSS never flushes','frame_members':['begin','launch','start','data','rss','end'],'call_hash':'exact canonical frame bytes in physical order for this run/call including END; exclude bridge/native_meta/sideband; seq0 strictly+1','range_hash':'exact SAME-handle verified range including LF','planned':'planned IN once; BEGIN may precede LAUNCH/START; no execution inferred','order':'BEGIN.qpc_start<=START.qpc<=runningRSS<=END.qpc_end; suspendedRSS identity-linked before START; first_write>=START and<=END; actual prefix digest','meta_refs':'source_proof_ref=exact stream header range; image_proof_ref=exact START range; closure_ref=exact END range; no richer proof invented','case':'case_id parent166; scope_id helperlabel; case_scope fresh32hex; complete case engine+checker through close','profile_global':'stream.profile_sha = SHA256 canonical profile catalogue {version:3,schedule_sha256,profiles:{id:action_payload}}','profile_action':'profile.profile_sha256 = SHA256 canonical action payload excluding ONLY profile_sha256 and profile_catalogue_sha256; catalogue stores same payload; no circular hash','widths':'parameters are bounded numeric data only; named resolves ONLY this types map; no caller schema nodes','b64_width':'DATA b64 literal empty is metadata baseline only; add 4*ceil(raw_chunk_bytes/3) per full coalesced chunk/one final partial','profile_adoption':'Prepare authenticated profile_type=group max lease. Adopt authenticated member whose group_id matches and profile_id is in group.members; each bound/component <=group; global catalogue hash equal. Persist actual action_profile_id/SHA in native_meta. Never rewrite global SHA as action SHA.','script_gate':'Source Driver injects FILE_NATIVE_SCRIPT_GATE canonical JSON exact five fields; LAUNCH binds its SHA. Child publishes readyJSON {pid,nonce,manifest_sha256}, waits parent releaseJSON {nonce,pid} before corpus I/O. Driver verifies actual live creation-bound PID/RSS before release; actual ready range retained in script-startup-gate-ready sideband. Driver additionally injects FILE_NATIVE_SCRIPT_DEADLINE_PERF_NS positive int64 absolute perf ns after actual phase/total clamp and before Popen; child polls it while waiting and before corpus access. Gate root means corpus; ready/release share owned artifact parent. Source preallocates 16384 bytes and 2 files before Popen, retains full upper hold on all paths. No caller env GO.'}}


def source_admission(ref):
    """Authenticate the parent's immutable physical source allocation."""
    need(type(ref) is dict and set(ref)=={"path","bytes","sha256"} and integer(ref["bytes"],1)
        and ref["bytes"]<=65536,"closed-source-admission-reference")
    raw=regular_read(ref["path"],ref["bytes"])
    need(len(raw)==ref["bytes"] and digest(raw)==ref["sha256"],"same-buffer-parent-source-admission")
    b=strict(raw,65536)
    kinds={
        "private-qualification-R3-source-budget-046-v2":(None,
            dict(LONG=49152,HISTORY=57344,CONTROL=32768,PROFILES=73728,FINALIZER=81920)),
        "private-qualification-R3-source-budget-052-v3":(
            "404d8959500bff1409ce8298279ca330be8f5d35b50914dacefd99872d43bccd",
            dict(LONG=47104,HISTORY=57344,CONTROL=16384,PROFILES=81920,FINALIZER=92160))}
    need(b.get("kind") in kinds,"known-approved-source-allocation-kind")
    claim,groups=kinds[b["kind"]]
    total=b["max_bytes"]
    if total in (303104,313344):
        need(b["kind"]=="private-qualification-R3-source-budget-052-v3"
            and b.get("claim059_sha256")=="96043645ad042150623ed0141d9ffe9ad8c90df67e9cce88daba3c0a4be139bd",
            "additional-source-physical-claim059")
        if total==313344:need(b.get("claim069_sha256")=="3ca712ec1c824741a5b83aad2723206de7a989b28a84c918bc0a9283baffdea7","source-claim069")
        groups={name:b["groups"][name]["bytes"] for name in groups}
        need(all(integer(n,1) for n in groups.values()),"parent-group-allocation-integers")
        if groups["PROFILES"]==83968 or "claim061_sha256" in b:
            need(b.get("claim061_sha256")=="5103300b9bb3a49eee35968d28347260109011419e49d7dc238be6f7611208b6",
                "source-claim061-refinement")
    else:need(total==294912,"approved-source-physical-total")
    need(b["claim046_sha256"]=="6bd67f146b344cdb8bd501a261df795e946a49b557d5a5d5227665c8e8e581ba"
        and (claim is None or b["claim052_sha256"]==claim)
        and b["engine_commit_fixed"]=="0628322e4b48e0efad19cdc32238a622d4aaa7a8"
        and b["owners"]==dict(LONG_PROFILES_FINALIZER="01a15b51-a50b-7930-8633-b42694240252",
            HISTORY="01a15bcc-6fb9-7e63-b935-a60d25d9390a")
        and b["source_only"] is True and b["runtime_go"] is False
        and b["max_bytes"]==b["bytes"]==total and b["max_files"]==b["files"]==10,
        "approved-claims-owners-pin-and-physical-total")
    names=dict(LONG={"long_lane.py","legacy_archive_restore.py"},HISTORY={"history_lane.py"},
        CONTROL={"fake_controls.py","fake_cases.json","lane_config.example.json","source-manifest.json","README.md"},
        PROFILES={"long_source_profiles.py"},FINALIZER={"phase_finalizer.py"})
    need(set(b["groups"])==set(names),"closed-source-groups")
    folder=Path(b["source_root"]);need(folder.is_absolute(),"absolute-source-allocation-root")
    actual={};total=0
    for group,paths in names.items():
        g=b["groups"][group]
        need(set(g)=={"bytes","files","paths"} and set(g["paths"])==paths
            and g["bytes"]==groups[group] and g["files"]==len(paths)
            and all(integer(cap,1) for cap in g["paths"].values()),"closed-source-group-caps")
        size=0
        for name,cap in g["paths"].items():
            p=folder/name;s=p.lstat()
            need(p.is_file() and not p.is_symlink() and s.st_nlink==1
                and not(s.st_file_attributes&1024) and s.st_size<=cap,"actual-source-file-cap")
            actual[name]=file_hash(p,cap);size+=s.st_size
        need(size<=g["bytes"],"actual-source-group-cap");total+=size
    need({p.name for p in folder.iterdir()}==set(actual)
        and total<=b["max_bytes"] and sum(groups.values())==b["max_bytes"],
        "closed-ten-source-files-and-whole-cap")
    return copy.deepcopy(ref)
