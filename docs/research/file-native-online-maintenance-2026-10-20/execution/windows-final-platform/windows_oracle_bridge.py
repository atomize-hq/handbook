"""Independent checker process bridge; never import the decoder or engine."""
import copy
import base64
import hashlib
import threading
from pathlib import Path
from windows_deadlines import Unsupported, canonical, digest, integer, need, strict
from windows_growth import confined, inventory, regular_read

CHECKS=frozenset(("complete_physical","all_accepted_outcomes","exact_request_intents",
    "selected_membership","graph","source_bytes","navigation","provenance",
    "retired_refusals","b_loose_controls","ordinary_selected_once_b","read_only_inventory"))


class OracleBridge:
    def __init__(self,host):
        self.host=host
        self.lock=threading.RLock()

    def _invoke(self,mode,root,kwargs):
        spec=self.host.manifest["hooks"].get("oracle-process")
        if spec is None or spec.get("reviewed") is not True:
            raise Unsupported("PENDING-reviewed-R1-plus-terminal-v2-full-verifier")
        need(spec["final_pin"]==self.host.final_pin and spec.get("reverse_cohort_closed") is True
             and spec.get("per_root_process") is True,"exact-final-independent-oracle-binding")
        # No in-process legacy global callback monkeypatching across roots.
        with self.lock:
            before=inventory(root,self.host.poll)
            invocation=self.host.hooks.call("oracle-invocation",self.host,
                dict(mode=mode,root=str(root),kwargs=copy.deepcopy(kwargs),
                     inventory=before,process_binding=copy.deepcopy(spec)))
            scope_id,scope=self.host.adapter.case_for(root)
            self.host.driver.context.value=dict(root=str(root),case_id=scope["case_id"],
                scope_id=scope["scope_id"],case_scope=scope["case_scope"])
            result=self.host.invoke(invocation)
            proof=strict(regular_read(result["output"]["path"],result["output"]["bytes"]),
                         result["output"]["bytes"])
            after=inventory(root,self.host.poll)
            need(before==after,"oracle-must-preserve-all-files-and-directories")
            need(proof.get("final_pin")==self.host.final_pin
                 and proof.get("engine_code_imported") is False
                 and proof.get("engine_audit_used") is False
                 and proof.get("source_unchanged") is True,"independent-source-unchanged-proof")
            proof["native_raw"]=result
            return proof

    def __call__(self,*,root,checkpoint,final_pin,known_requests,required_mapped_keys):
        need(final_pin==self.host.final_pin,"oracle-final-pin")
        self.host.adapter.quiesce(root)
        proof=self._invoke("full",root,dict(checkpoint=checkpoint,quiescent=True,
            known_requests=known_requests,required_mapped_keys=required_mapped_keys))
        need(proof.get("status")=="passed" and CHECKS<=set(proof.get("checks",[]))
             and proof.get("reverse_selected_cohort") is True
             and proof.get("all_files_directories_hashed") is True,"full-complete-oracle-only")
        need(proof.get("actual_H")==len(known_requests),"all-accepted-exact-H")
        need(set(required_mapped_keys)<=set(proof.get("mapped_keys",[])),"oracle-required-mapped-keys")
        ref=self.host.events.append(dict(kind="independent-oracle",proof=proof))
        proof["receipt_sha256"]=ref["sha256"]
        return proof

    def query(self,mode,root,**kwargs):
        need(mode in ("mapping","backlog"),"closed-independent-query-mode")
        proof=self._invoke(mode,root,kwargs)
        need(proof.get("independent") is True,"independent-mapping-or-backlog")
        return proof

def read_ref(ref,maximum):
    import os
    need(type(ref) is dict and type(ref.get("path")) is str,"bounded-reference")
    n=ref.get("bytes",ref.get("length"));at=ref.get("offset",0)
    need(integer(n) and n<=maximum and integer(at),"bounded-reference-range")
    p=Path(ref["path"]);confined(Path(p.anchor),p)
    with p.open("rb") as stream:
        before=os.fstat(stream.fileno());need(at+n<=before.st_size,"reference-within-file")
        stream.seek(at);raw=stream.read(n+1 if at+n==before.st_size else n)
        after=os.fstat(stream.fileno())
    need(len(raw)==n and (before.st_dev,before.st_ino)==(after.st_dev,after.st_ino)
         and after.st_size>=at+n and digest(raw)==ref["sha256"],"same-handle-reference-hash")
    return raw


def extract_call(reference,meta_ref,*,max_span,max_raw_bytes,source_pin,binary_sha256,
                 expected_scope=None,expected_exit=0,parse_stdout=True,expected_intent=None):
    keys={"schema","path","run","call","begin","end","frames","bytes","sha256"}
    need(type(reference) is dict and set(reference)==keys and reference["schema"]=="single-raw-ref-v1",
         "closed-single-raw-ref")
    need(all(integer(reference[k]) for k in ("begin","end","frames","bytes"))
         and 0<=reference["end"]-reference["begin"]<=max_span,"bounded-physical-span")
    data={s:bytearray() for s in ("in","out","err")};partial=set();h=hashlib.sha256()
    seq=0;total=0;frames={};rss=[];p=Path(reference["path"])
    confined(Path(p.anchor),p)
    with p.open("rb") as stream:
        header_line=stream.readline(8193);header=strict(header_line,8192)
        need(set(header)=={"v","kind","run","source_pin","binary_sha","guard_sha","driver_sha","profile_sha"}
             and header["v"]==2 and header["kind"]=="stream" and header["run"]==reference["run"]
             and header["source_pin"]==source_pin and canonical(header)+b"\n"==header_line,
             "canonical-run-source-header")
        header_ref=dict(path=str(p),offset=0,bytes=len(header_line),sha256=digest(header_line))
        stream.seek(reference["begin"]);left=reference["end"]-reference["begin"]
        while left:
            at=stream.tell();line=stream.readline(min(left,262144)+1)
            need(line and len(line)<=min(left,262144) and line.endswith(b"\n"),"whole-bounded-frame")
            left-=len(line);row=strict(line,262144);need(canonical(row)+b"\n"==line,"canonical-call-frame")
            if row.get("call")!=reference["call"]:continue
            need(row.get("v")==2 and row.get("seq")==seq and integer(row["seq"])
                 and row["seq"]<2**64,"strict-sequence")
            seq+=1;total+=len(line);h.update(line);kind=row["kind"]
            ref=dict(path=str(p),offset=at,bytes=len(line),sha256=digest(line))
            if kind=="data":
                need(set(row)=={"v","kind","call","seq","stream","offset","n","sha","b64"},"closed-data")
                name=row["stream"];need(name in data and name not in partial,"one-final-partial-per-stream")
                need(name=="in" or "start" in frames,"positive-start-before-output")
                block=base64.b64decode(row["b64"],validate=True)
                need(row["offset"]==len(data[name]) and row["n"]==len(block)
                     and 0<len(block)<=16384 and digest(block)==row["sha"],"continuous-coalesced-data")
                if len(block)<16384:partial.add(name)
                data[name].extend(block);need(sum(map(len,data.values()))<=max_raw_bytes,"expanded-raw-bound")
            elif kind=="rss":
                need(set(row)=={"v","kind","call","seq","pid","creation","qpc","rss","peak","role","phase"},
                     "closed-native-RSS");rss.append(row)
            else:
                need(kind in ("begin","launch","start","end") and kind not in frames,"one-closed-call-milestone")
                if kind=="begin":
                    need(seq==1 and set(row)=={"v","kind","call","seq","root","action","role","env",
                         "input_n","input_sha","input_scope","pid","creation","qpc_start"}
                         and row["input_scope"]=="planned","first-planned-BEGIN")
                elif kind=="launch":
                    need("begin" in frames and "start" not in frames and set(row)=={"v","kind","call","seq",
                         "argv","executable_sha256","producer_kind","producer_source_pin",
                         "source_files","startup_gate_sha256"},"closed-planned-LAUNCH")
                elif kind=="start":
                    need("launch" in frames and set(row)=={"v","kind","call","seq","pid","creation","qpc","rss",
                         "executable_sha256"},"closed-actual-START")
                else:
                    need(not left and set(row)=={"v","kind","call","seq","exit","in_n","out_n","err_n",
                        "in_sha","out_sha","err_sha","input_complete","out_eof","err_eof","rss_n","peak",
                        "qpc_end","first_write_qpc","native_closed","io_closed","callbacks_joined",
                        "intent","violations"},"one-final-closed-END")
                frames[kind]=(row,ref)
    need(seq==reference["frames"] and total==reference["bytes"] and h.hexdigest()==reference["sha256"],
         "exact-physical-call-frame-digest")
    need(set(frames)=={"begin","launch","start","end"},"actual-started-call-required")
    begin,launch,start,end=(frames[k][0] for k in ("begin","launch","start","end"))
    need(launch["executable_sha256"]==start["executable_sha256"]==binary_sha256 and start["rss"]>0
         and integer(start["pid"],1) and integer(start["creation"],1),"actual-authorized-image-start")
    need(end["exit"]==expected_exit and end["intent"]==expected_intent and end["violations"]==[]
         and all(end[k] is True for k in ("input_complete","out_eof","err_eof","native_closed","io_closed","callbacks_joined")),
         "uncensored-owned-exit-and-closure")
    need(len(data["in"])==begin["input_n"] and digest(data["in"])==begin["input_sha"]
         and end["in_n"]==len(data["in"]) and digest(data["in"][:end["in_n"]])==end["in_sha"]
         and (end["first_write_qpc"] is None if end["in_n"]==0 else
              integer(end["first_write_qpc"],1) and end["first_write_qpc"]>=start["qpc"]),
         "planned-vs-actual-written-prefix-and-order")
    need(len(data["out"])==end["out_n"] and digest(data["out"])==end["out_sha"]
         and len(data["err"])==end["err_n"] and digest(data["err"])==end["err_sha"]
         and len(rss)==end["rss_n"] and all(r["pid"]==start["pid"] and r["creation"]==start["creation"]
             and 0<r["rss"]<=r["peak"]<=end["peak"] for r in rss),"raw-output-RSS-counts")
    meta=strict(read_ref(meta_ref,65536),65536)
    meta_keys={"schema","call","run","raw_ref","source_pin","binary_sha256","producer_kind","producer_source_pin","identity","process_started_ns","process_done_ns","spawn_total_ns",
        "source_proof_ref","image_proof_ref","qpc_frequency","clock_bridge_ref",
        "closure_ref","case_id","scope_id","case_scope"}
    need(set(meta)==meta_keys and meta["schema"]=="native-call-meta-v2" and meta["raw_ref"]==reference
         and meta["source_proof_ref"]==header_ref and meta["image_proof_ref"]==frames["start"][1]
         and meta["closure_ref"]==frames["end"][1],
         "authenticated-native-metadata-links")
    if expected_scope is not None:
        need(all(meta[k]==expected_scope[k] for k in ("case_id","scope_id","case_scope")),"same-actual-case-scope")
    clock=strict(read_ref(meta["clock_bridge_ref"],8192),8192)
    need(clock["run"]==reference["run"] and clock["frequency"]==meta["qpc_frequency"]
         and integer(meta["qpc_frequency"],1) and clock["qpc_before"]<=clock["qpc_after"],
         "same-run-actual-clock-bridge")
    need(integer(meta["process_started_ns"],1) and meta["process_done_ns"]>=meta["process_started_ns"]
         and meta["spawn_total_ns"]==meta["process_done_ns"]-meta["process_started_ns"],"actual-perf-native-call-clocks")
    response=strict(bytes(data["out"]),max_raw_bytes) if parse_stdout else None
    return dict(response=response,planned_input=bytes(data["in"]),stderr=bytes(data["err"]),
        begin=begin,launch=launch,start=start,end=end,meta=meta,raw_ref=reference,meta_ref=meta_ref)
