"""Independent checker bridge."""
import copy
import base64
import hashlib
import threading
from pathlib import Path
from windows_deadlines import Unsupported, canonical, digest, integer, need, strict
from windows_growth import confined, file_hash, inventory, regular_read

CHECKS=frozenset(("complete_physical","all_accepted_outcomes","exact_request_intents",
    "selected_membership","graph","source_bytes","navigation","provenance",
    "retired_refusals","b_loose_controls","ordinary_selected_once_b","read_only_inventory"))

class OracleBridge:
    def __init__(self,host):
        self.host=host
        self.lock=threading.RLock()

    def _invoke(self,mode,root,kwargs):
        from windows_capsule import checker_capture
        with self.lock:
            return consume_checker(checker_capture(self.host,mode,root,kwargs))

    def __call__(self,*,root,checkpoint,final_pin,known_requests,required_mapped_keys):
        need(final_pin==self.host.final_pin,"oracle-final-pin")
        self.host.adapter.quiesce(root)
        proof=self._invoke("full",root,dict(checkpoint=checkpoint,quiescent=True,
            known_requests=known_requests,required_mapped_keys=required_mapped_keys))
        proof=normalize_full(proof,len(known_requests))
        need(proof.get("status")=="passed" and CHECKS<=set(proof.get("checks",[]))
             and proof.get("reverse_selected_cohort") is True
             and proof.get("all_files_directories_hashed") is True,"full-complete-oracle-only")
        need(proof.get("actual_H")==len(known_requests),"all-accepted-exact-H")
        need(set(required_mapped_keys)<=set(proof.get("mapped_keys",[])),"oracle-required-mapped-keys")
        ref=self.host.events.append(dict(kind="independent-oracle",proof=proof))
        proof["receipt_sha256"]=ref["sha256"]
        proof["history_oracle"]["receipt_sha256"]=ref["sha256"]
        return proof

    def legacy_full(self,root,checkpoint,known_requests_ref,request_provenance_ref,
                    required_mapped_keys,transport_profile,output_prefix,limits):
        from windows_capsule import checker_capture
        from windows_adapter import checker_native
        with self.lock:
            c=checker_capture(self.host,"legacy_full",root,
                dict(checkpoint=checkpoint,known_requests_ref=known_requests_ref,
                    request_provenance_ref=request_provenance_ref,required_mapped_keys=required_mapped_keys),
                transport_profile=transport_profile,output_prefix=output_prefix,limits=limits)
            consume_checker(c)
            return dict(index_ref=c["index_ref"],request_ref=c["request_ref"],
                native=checker_native(self.host,c),artifacts=c["index"]["artifacts"])

    def decoder(self,root):return GuardedDecoder(self,root)

    def refuse(self,root,checkpoint,require_before_index=False):
        proof=self._invoke("refuse",root,dict(checkpoint=checkpoint,require_before_index=require_before_index))
        need(proof["status"]=="expected-independent-refusal" and proof["complete_expected_refusal"] is True,
            "actual-full-independent-refusal")
        return proof

    def query(self,mode,root,**kwargs):
        need(mode in ("mapping","backlog"),"closed-independent-query-mode")
        proof=self._invoke(mode,root,kwargs)
        proof["independent"]=proof["engine_code_imported"] is False and proof["engine_audit_used"] is False
        need(proof["independent"],"independent-mapping-or-backlog")
        return proof

from windows_growth import read_ref

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
            at=stream.tell();line=stream.readline(min(left,4259840)+1)
            need(line and len(line)<=min(left,4259840) and line.endswith(b"\n"),"whole-bounded-frame")
            left-=len(line);row=strict(line,4259840);need(canonical(row)+b"\n"==line,"canonical-call-frame")
            if row.get("call")!=reference["call"] or row.get("kind") not in ("begin","launch","start","data","rss","end"):continue
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
        "closure_ref","case_id","scope_id","case_scope","action_profile_id","action_profile_sha256"}
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

def ordinary(*args,**kwargs):
    from windows_capsule import ordinary as source_ordinary
    return source_ordinary(*args,**kwargs)

class GuardedDecoder:
    def __init__(self,bridge,root,physical_only=False):
        self.bridge,self.root=bridge,Path(root);self.physical_only=physical_only
        self.entries=GuardedEntries(self)
    def raw(self,key):
        need(type(key) is str,"guarded-decoder-key")
        proof=self.bridge._invoke("raw",self.root,dict(key=key,physical_only=self.physical_only))
        raw=base64.b64decode(proof["raw_base64"],validate=True)
        need(len(raw)<=8388608 and len(raw)==proof["bytes"] and digest(raw)==proof["sha256"],
            "actual-guarded-decoder-bytes")
        return raw

class GuardedEntries:
    def __init__(self,decoder):self.d=decoder
    def __getitem__(self,key):
        proof=self.d.bridge._invoke("entry",self.d.root,dict(key=key,physical_only=self.d.physical_only))
        row=proof["entry"]
        need(type(row) is list and len(row)==5 and type(row[0]) is dict
            and all(integer(row[i]) for i in (1,2,3)) and type(row[4]) is str
            and len(row[4])==64,"actual-guarded-v5-entry")
        return tuple(row)

def consume_checker(capture):
    index=capture["index"];contract=capture["contract"];limits=capture["limits"]
    prefix=Path(capture["index_ref"]["path"]);roles={}
    total=capture["index_ref"]["bytes"]
    for artifact in index["artifacts"]:
        role=artifact["role"]
        need(role in contract["artifacts"] and role!="partial_suffix" and role not in roles
            and artifact["complete"] is True
            and Path(artifact["path"])==prefix.with_suffix(contract["artifacts"][role]),
            "closed-complete-checker-artifact")
        ref=dict(path=artifact["path"],offset=0,bytes=artifact["bytes"],sha256=artifact["sha256"])
        # Hash every companion, including large gzip manifests, without expanding it.
        from windows_growth import file_hash
        need(file_hash(ref["path"],ref["bytes"])==ref["sha256"]
            and Path(ref["path"]).stat().st_size==ref["bytes"],"actual-complete-companion-hash")
        roles[role]=ref;total+=ref["bytes"]
    need(total<=limits["total_output_bytes"]
        and {"before","after","payload"}<=set(roles),"whole-checker-output-and-required-artifacts")
    evidence=index["proof"];audit=index["index_open_audit"]
    need(evidence["source_unchanged"] is True and evidence["before"]["complete"] is True
        and evidence["after"]["complete"] is True
        and evidence["before"]["state_sha256"]==evidence["after"]["state_sha256"]
        and audit["spawn_events"]==0 and not audit["unexpected_modules"],"actual-readonly-independent-checker-proof")
    payload=strict(read_ref(roles["payload"],limits["normalized_reserve_bytes"]),limits["normalized_reserve_bytes"])
    need(payload["status"]==index["status"] and payload["final_pin"]==index["engine_pin"]
        and payload["engine_code_imported"] is False and payload["engine_audit_used"] is False
        and payload["source_unchanged"] is True,"actual-full-checker-payload")
    capture["verified_full"]=payload["all_files_directories_hashed"] is True
    capture["artifact_refs"]=roles
    checker_settlement(capture["_host"],capture)
    payload["artifact_refs"]=roles;payload["index_ref"]=capture["index_ref"]
    payload["native_raw"]=dict(raw=capture["native_raw"],call_token=capture["call_token"],
        closure_ref=capture["closure_ref"],output=roles["payload"],request_ref=capture["request_ref"])
    return payload

def normalize_full(proof,actual_H):
    full=proof["history_oracle"];r1=full["mapped_cohort_closure"];intents=full["request_intents"]
    semantic=full["semantic"];navigation=full["navigation"]
    need(proof["status"]=="passed" and proof["quiescent"] is True
        and proof["all_files_directories_hashed"] is True
        and proof["checks"]==dict(semantic=semantic,navigation=navigation,request_intents=intents,
            cohort=r1,quota=full.get("quota"))
        and r1["profile"]=="every-mapped-key-to-authenticated-accepted-role-owner-v1"
        and r1["accepted_publications"]==actual_H and r1["captured_selected_authority"] is True
        and r1["candidate_context"] is False
        and semantic["publications"]==intents["publications"]==intents["actual_H"]==actual_H
        and intents["engine_code_imported"] is False
        and navigation["immutable_discovery_semantics_verified"] is True,
        "actual-full-verifier-semantic-intent-navigation-and-R1-closure")
    summary=dict(passed=True,actual_H=actual_H,all_accepted_outcomes=True,all_request_intents=True,
        source_graph_navigation=True,reverse_selected_cohort=True,full_file_directory_hash_proof=True,
        quiescent=True,source_unchanged=True,engine_audit_used=False)
    return dict(status="passed",final_pin=proof["final_pin"],engine_code_imported=False,
        engine_audit_used=False,source_unchanged=True,quiescent=True,all_files_directories_hashed=True,
        checks=sorted(CHECKS),actual_H=actual_H,mapped_keys=proof["mapped_keys"],
        reverse_selected_cohort=True,history_oracle=summary,artifact_refs=proof["artifact_refs"],
        index_ref=proof["index_ref"],native_raw=proof["native_raw"])

def checker_settlement(host,c):
    state=host.driver.registry[c["call_token"]];row=host.driver.context.completed_row
    need(state["done"].is_set() and state["owner_finished"] and state["resources_closed"]
        and state["stdout_eof"] and state["stderr_eof"] and state["callback_joined"]
        and not state["violations"] and state["raw_reference"]==c["native_raw"],
        "same-call-positive-writer-severance")
    prefix=Path(c["index_ref"]["path"]);contract=c["contract"]
    expected={prefix}
    for role,suffix in contract["artifacts"].items():
        if role!="partial_suffix":
            final=prefix.with_suffix(suffix);expected.update((final,final.with_name(final.name+".partial")))
    retained={p for p in expected if p.exists()}
    indexed={Path(a["path"]) for a in c["index"]["artifacts"]}|{prefix}
    need(retained==indexed,"all-output-final-and-partial-members-accounted")
    read_ref(c["request_ref"],c["limits"]["max_request_bytes"])
    read_ref(c["index_ref"],16384)
    proof=dict(schema="closed-writer-settlement-v1",source_pin_sha256=host.source_manifest_sha256,
        profile_sha256=host.grant["profile_sha256"],budget_sha256=host.owner["budget_sha256"],
        whole_phase_forecast_sha256=host.grant["whole_phase_forecast_sha256"],
        process_reaped=True,io_closed=True,callbacks_joined=True,raw_writer_released=True,
        scope="this-exact-native-checker-call",call_token=state["token"])
    category=host.ledger.root_name(prefix)
    state["checker_writes_retired"]=True
    host.ledger.settle("checker-output:"+str(prefix),{category:dict(
        bytes=sum(p.stat().st_size for p in retained),files=len(retained),directories=0)},proof)
    gate=row["input"]["script_spec"]["startup_gate"]
    paths=[Path(gate[k]) for k in ("ready","release")]
    blobs=[regular_read(p,8192) for p in paths if p.exists()]
    need(len(blobs)==2,"both-actual-gate-files-retained")
    category=host.ledger.root_name(paths[0])
    host.ledger.settle("script-gate:"+gate["nonce"],{category:dict(
        bytes=sum(map(len,blobs)),files=2,directories=0)},proof)
    if not hasattr(host,"checker_results"):host.checker_results={}
    host.checker_results[state["root"]]=dict(artifacts=c["artifact_refs"],index_ref=c["index_ref"],
        request_ref=c["request_ref"],limits=c["limits"],call_token=c["call_token"],closure_ref=c["closure_ref"],mode=c["index"]["mode"],full=c["verified_full"])
    return proof

def long_inventory_digest(host,c):
    import gzip,json
    files=[];dirs=[];excluded=[];total=0
    exclusions={"writer.lock","preparation.lock","pack-readers.lock","pack-maintenance.lock","DISPOSABLE.json"}
    before=c["artifacts"]["before"]
    need(file_hash(before["path"],before["bytes"])==before["sha256"],"before-proof-hash")
    with gzip.open(before["path"],"rb") as stream:
        for line in stream:
            need(len(line)<=6*c["limits"]["max_path_utf8"]+192,"bounded-inventory-row")
            row=strict(line,len(line))
            if "directory" in row:
                if row["directory"]!=".":dirs.append(row["directory"])
            else:
                total+=row["bytes"];(excluded if row["path"] in exclusions else files).append(row)
            need(len(files)+len(excluded)<=c["limits"]["max_files"]
                and len(dirs)<=c["limits"]["max_directories"] and total<=c["limits"]["max_logical_bytes"],
                "bounded-reconstructed-inventory")
    facts=dict(files=sorted(files,key=lambda r:r["path"]),directories=sorted(dirs),
        excluded=sorted(excluded,key=lambda r:r["path"]),total_source_bytes=total)
    sha=digest((json.dumps(facts,ensure_ascii=True,sort_keys=True,separators=(",",":"))+"\n").encode())
    return sha,len(files),len(dirs)
