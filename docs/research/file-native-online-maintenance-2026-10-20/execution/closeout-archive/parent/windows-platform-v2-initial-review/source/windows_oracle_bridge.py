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

def extract_call(reference,*,max_span,max_raw_bytes,source_pin,binary_sha256):
    """Bounded rehydration from exact canonical interleaved sequence/hash domain."""
    keys={"schema","path","run","call","begin","end","frames","bytes","sha256"}
    need(type(reference) is dict and set(reference)==keys
         and reference["schema"]=="single-raw-ref-v1","closed-single-raw-reference")
    need(all(integer(reference[k]) for k in ("begin","end","frames","bytes"))
         and reference["end"]>=reference["begin"]
         and reference["end"]-reference["begin"]<=max_span,"bounded-reference-span")
    data={s:bytearray() for s in ("in","out","err")};h=hashlib.sha256();seq=0;total=0;start=None;end=None;begin=None
    path=Path(reference["path"]);confined(Path(path.anchor),path)
    with path.open("rb") as stream:
        header=stream.readline(8193);head=strict(header,8192)
        need(head.get("kind")=="stream" and head.get("run")==reference["run"]
             and head.get("binding",{}).get("final_pin")==source_pin,"stream-run-source-binding")
        stream.seek(reference["begin"]);left=reference["end"]-reference["begin"]
        while left:
            line=stream.readline(min(left,262144)+1)
            need(line and len(line)<=left and len(line)<=262144 and line.endswith(b"\n"),"complete-bounded-frame")
            left-=len(line);row=strict(line,262144)
            need(canonical(row)+b"\n"==line,"canonical-physical-frame-bytes")
            if row.get("call",row.get("call_token"))!=reference["call"]:continue
            need(row.get("seq")==seq and integer(row["seq"]) and row["seq"]<2**64
                 and row.get("call_token",reference["call"])==reference["call"],"exact-call-sequence")
            seq+=1;total+=len(line);h.update(line)
            kind=row["kind"]
            if kind=="begin":
                need(begin is None and seq==1 and row["run"]==reference["run"],"one-first-call-begin")
                need(set(row)=={"v","kind","call","run","input_n","input_sha","input_scope","env",
                     "profile_sha","pid","creation","request","seq"} and row["input_scope"]=="planned",
                     "closed-planned-input-begin")
                begin=row
            elif kind=="start":
                need(start is None and set(row)=={"v","kind","call","seq","pid","creation","qpc","rss"},
                     "one-closed-positive-start");start=row
            elif kind=="data":
                need(set(row)=={"v","kind","call","stream","offset","n","sha","b64","seq"},"closed-data-frame")
                need(row["stream"]=="in" or start is not None,"positive-start-before-output")
                part=base64.b64decode(row["b64"],validate=True);target=data[row["stream"]]
                need(row["offset"]==len(target) and row["n"]==len(part)<=16384
                     and digest(part)==row["sha"],"continuous-authentic-raw-data")
                target.extend(part);need(sum(map(len,data.values()))<=max_raw_bytes,"expanded-raw-bound")
            elif kind=="end":
                need(end is None and not left,"one-final-call-end");end=row
            else:need(kind=="rss","closed-extracted-frame-kind")
    need(seq==reference["frames"] and total==reference["bytes"]
         and h.hexdigest()==reference["sha256"],"exact-call-frame-reference-digest")
    need(begin and start and end and not end["violations"]
         and end["executable_sha256"]==binary_sha256 and end["source_pin"]==source_pin
         and end["raw_retained"] and end["resources_closed"] and end["stdin_closed"]
         and end["input_fully_delivered"] and end["stdout_eof"] and end["stderr_eof"]
         and end["python_callback_joined"] and end["popen_reaped"]
         and start["rss"]>0 and end["input_bytes_sent"]==begin["input_n"]
         and start["pid"]==end["native_identity"]["pid"]
         and start["creation"]==end["native_identity"]["creation_time"],
         "positive-actual-call-native-input-EOF-closure")
    need(len(data["in"])==begin["input_n"] and digest(data["in"])==begin["input_sha"]
         and end["in_n"]==end["input_bytes_sent"]==len(data["in"])
         and digest(data["in"][:end["in_n"]])==end["in_sha"]
         and (end["first_write_qpc"] is None if end["in_n"]==0 else
              integer(end["first_write_qpc"],1) and end["first_write_qpc"]>=start["qpc"])
         and len(data["out"])==end["out_n"] and digest(data["out"])==end["stdout_sha256"]
         and len(data["err"])==end["err_n"] and digest(data["err"])==end["stderr_sha256"],
         "exact-complete-input-output-counts")
    return dict(response=strict(bytes(data["out"]),max_raw_bytes),begin=begin,start=start,end=end,
                planned_input=bytes(data["in"]),
                raw_ref=copy.deepcopy(reference))
