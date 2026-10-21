"""Bounded append-only lossless records. No runtime files on import."""
import base64
import copy
import gzip
import hashlib
import uuid
import threading
import time
from pathlib import Path
from windows_deadlines import canonical, digest, integer, need, Refusal, strict, project_response
from windows_growth import confined, file_hash, inventory


class Evidence:
    def __init__(self, path, ledger, category, *, max_bytes, max_files=1, binding=None):
        self.path = confined(Path(ledger.roots[category]["root"]), Path(path))
        need(self.path.parent.is_dir() and not self.path.exists(), "fresh-evidence-path")
        need(integer(max_bytes, 1) and max_files == 1, "bounded-one-evidence-file")
        self.ledger, self.category = ledger, category
        self.max_bytes, self.written, self.count = max_bytes, 0, 0
        self.lock, self.binding = threading.RLock(), copy.deepcopy(binding or {})
        need(len(canonical(self.binding))<=512,"bounded-evidence-binding")
        self.pending={};self.calls={}
        self.v2=self.binding.get("capture_schema")=="single-raw-ref-v1"
        self.run=uuid.uuid4().hex
        self.context=None
        self.closed=False
        self.key = "evidence:"+str(self.path)
        ledger.reserve(self.key, {category:dict(bytes=max_bytes, files=1, directories=0)})
        try:
            with self.path.open("xb"):
                pass
        except BaseException:
            ledger.settle(self.key)
            raise
        if self.v2:
            self.append(dict(v=1,kind="stream",run=self.run,binding=self.binding))

    def append(self, value):
        if self.v2:
            value=copy.deepcopy(value);token=value.get("call_token",value.get("call"))
            if value.get("kind")=="raw-complete":
                return dict(covered_by="native-end-v2")
            if value.get("kind")=="native-RSS":
                from windows_native import counter
                value=dict(v=1,kind="rss",call=token,pid=value["pid"],creation=value["creation_time"],
                    qpc=counter(),rss=value["rss"],peak=value["peak"],
                    role="reader" if value["rss_limit"]==268435456 else "checker",
                    phase="sample" if self.calls[token].get("started") else "suspended")
            if value.get("kind")=="positive-native-start":
                from windows_native import counter
                value=dict(v=1,kind="start",call=token,pid=value["pid"],
                    creation=value["creation_time"],qpc=counter(),rss=value["positive_rss"])
            if value.get("kind")=="native-call":
                need(token in self.calls,"begun-single-capture-call")
                original=value
                value={k:v for k,v in value.items() if k not in
                    ("stdout","stderr","input","env","historical_invocation")}
                value.update(v=1,kind="end",call=token,out_n=original["stdout_bytes"],err_n=original["stderr_bytes"],
                    in_n=original["input_bytes_sent"],in_sha=original["input_written_prefix_sha256"],
                    input_sha256=self.calls[token]["input_sha256"])
                need(len(canonical(value))+64<=self.calls[token]["end_max"],"bounded-native-end-frame")
            elif self.context is not None:
                completed=getattr(self.context,"completed_token",None)
                if completed in self.calls and self.calls[completed].get("ended"):
                    value=project_response(value,self.call_reference(completed),
                        getattr(self.context,"completed_response",None),max_bytes=16*1024**2)
            if value.get("kind") not in ("begin","start","data","rss","end"):token=None
            if token in self.calls:
                value.setdefault("call",token)
                value["seq"]=self.calls[token]["frames"]
            raw=canonical(value)+b"\n"
        else:
            raw=canonical(dict(binding=self.binding, observation=value))+b"\n"
        with self.lock:
            need(not self.closed,"closed-evidence-stream")
            token=(value.get("call",value.get("call_token")) if type(value) is dict else None)
            if token in self.pending:
                need(len(raw)<=self.pending[token],"per-call-raw-reservation")
            held=sum(self.pending.values())-(len(raw) if token in self.pending else 0)
            need(self.written+len(raw)+held<=self.max_bytes,
                 "raw-evidence-plus-pending-before-write")
            if type(value) is dict and value.get("kind")=="native-RSS":
                need(len(raw)<=1024,"exact-RSS-row-envelope")
            if type(value) is dict and value.get("kind")=="raw-chunk":
                need(len(raw)<=len(value["bytes_b64"])+1024,"exact-raw-chunk-envelope")
            with self.path.open("ab") as output:
                output.write(raw)
                output.flush()
            if token in self.pending:
                self.pending[token]-=len(raw)
            self.written += len(raw)
            self.count += 1
            if self.v2 and token in self.calls:
                c=self.calls[token];c["hash"].update(raw);c["bytes"]+=len(raw);c["frames"]+=1
                c["end"]=self.written
                if value.get("kind")=="end":c["ended"]=True
                if value.get("kind")=="start":c["started"]=True
        if self.v2 and token in self.calls and value.get("kind")=="end":
            return self.call_reference(token)
        return dict(path=str(self.path), offset=self.written-len(raw), bytes=len(raw),
                    rows=self.count, stream_bytes=self.written, sha256=digest(raw))

    def reserve_call(self,token,maximum):
        with self.lock:
            need(integer(maximum,1) and token not in self.pending
                 and self.written+sum(self.pending.values())+maximum<=self.max_bytes,
                 "whole-call-raw-envelope-before-pipes")
            self.pending[token]=maximum

    def finish_call(self,token):
        with self.lock:
            need(token in self.pending,"known-call-evidence-envelope")
            del self.pending[token]  # Unwritten reservation only, never deletion credit.

    def begin_call(self,token,payload,encoded,env,profile):
        need(self.v2 and token in self.pending and token not in self.calls,"single-capture-begin")
        self.calls[token]=dict(hash=hashlib.sha256(),begin=self.written,end=self.written,
            bytes=0,frames=0,ended=False,input_sha256=digest(canonical(payload)),end_max=profile["end_max_bytes"])
        self.append(dict(v=1,kind="begin",call=token,run=self.run,input_n=len(encoded),
            input_sha=digest(encoded),input_scope="planned",env=env,profile_sha=profile["profile_sha256"],
            pid=None,creation=None,request=payload if not encoded else None))
        self.raw(token,"stdin",0,encoded)

    def call_reference(self,token):
        with self.lock:
            c=self.calls[token]
            need(c["ended"],"closed-call-frames-before-reference")
            return dict(schema="single-raw-ref-v1",path=str(self.path),run=self.run,
                call=token,begin=c["begin"],end=c["end"],frames=c["frames"],
                bytes=c["bytes"],sha256=c["hash"].hexdigest())

    def raw(self, call, stream, offset, block):
        need(type(block) is bytes and len(block) <= 4*1024**2, "bounded-raw-chunk")
        if self.v2:
            need(call in self.calls and not self.calls[call]["ended"],"live-single-capture")
            result=None
            for start in range(0,len(block),16384):
                part=block[start:start+16384]
                result=self.append(dict(v=1,kind="data",call=call,
                    stream={"stdin":"in","stdout":"out","stderr":"err"}[stream],
                    offset=offset+start,n=len(part),sha=digest(part),
                    b64=base64.b64encode(part).decode("ascii")))
            return result
        return self.append(dict(kind="raw-chunk", call_token=call, stream=stream,
            offset=offset, bytes=len(block), sha256=digest(block),
            bytes_b64=base64.b64encode(block).decode("ascii")))

    def reference(self):
        return dict(path=str(self.path), bytes=self.written, rows=self.count,
                    sha256=file_hash(self.path, self.max_bytes), lossless=True,
                    schema="single-raw-stream-v1" if self.v2 else "legacy-raw-v1",run=self.run)

    def close(self):
        need(not self.pending,"all-call-evidence-reservations-closed")
        self.closed=True
        if self.v2:
            need(callable(getattr(self,"settlement_proof",None)),"source-bound-closed-stream-settlement")
            self.ledger.settle(self.key,{self.category:dict(bytes=self.written,files=1,directories=0)},
                self.settlement_proof())
        else:self.ledger.settle(self.key)
        return self.reference()


def gzip_copy(source, destination, ledger, category, max_bytes, poll):
    """Retain source; reserve worst case gzip+trailer BEFORE opening output."""
    source, destination = Path(source), Path(destination)
    confined(Path(ledger.roots[category]["root"]), destination)
    need(destination.parent.is_dir() and not destination.exists(), "fresh-gzip-output")
    size = source.stat().st_size
    need(max_bytes >= size+(size//16384+1)*32+4096, "gzip-incompressible-and-footer-bound")
    token = "gzip:"+str(destination)
    ledger.reserve(token, {category:dict(bytes=max_bytes, files=1, directories=0)})
    with source.open("rb") as inp, destination.open("xb") as raw:
        with gzip.GzipFile(filename="", fileobj=raw, mode="wb", compresslevel=1, mtime=0) as out:
            while True:
                poll()
                block = inp.read(65536)
                if not block:
                    break
                out.write(block)
                need(raw.tell() <= max_bytes-4096, "gzip-runtime-bound")
        raw.flush()
    need(destination.stat().st_size <= max_bytes, "closed-gzip-final-bound")
    ledger.settle(token)
    # Caller must checkpoint after its final JSON as well; this is not that census.
    return dict(source_sha256=file_hash(source), gzip_sha256=file_hash(destination),
                bytes=destination.stat().st_size, source_retained=True)


def record_call(driver,state,payload,encoded,env,argv,profile,historical,script_gate,
                 output_mode,actual_sha,started,peak,samples,deadline,out,err,offsets,code,limit):
    token=state["token"];p=state["process"];response=None
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
                driver.evidence.raw(token,name,offsets[name],bytes(target[offsets[name]:end]))
                offsets[name]=end
        except BaseException as error:
            state["violations"].append("raw-retry-"+name+":"+repr(error))
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
        process_done_ns=time.perf_counter_ns(),spawn_total_ns=time.perf_counter_ns()-started,
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
        row["raw_retained"]=offsets["stdout"]==len(out) and offsets["stderr"]==len(err)
        state["raw_reference"]=driver.evidence.append(dict(kind="native-call",**row))
        state["raw_row_written"]=True
    except BaseException as error:
        state["violations"].append("native-row-sink:"+repr(error))
        row["raw_retained"]=False
        row["violations"]=list(state["violations"])
    return response,row

