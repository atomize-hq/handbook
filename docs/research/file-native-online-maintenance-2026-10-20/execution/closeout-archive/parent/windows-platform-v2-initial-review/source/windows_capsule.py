"""One reviewed LaneHost for LONG, legacy restore and HISTORY."""
import copy
import importlib
import os
from pathlib import Path
import threading
import time
import uuid
from windows_adapter import NativeAdapter
from windows_deadlines import (CHECKER_BYTES, FUNCTIONAL, GUARD_SHA, Deadline,
                               Unsupported, canonical, digest, integer, need, strict)
from windows_driver import WindowsDriver
from windows_evidence import Evidence
from windows_growth import confined, file_hash, GrowthLedger, inventory, regular_read
from windows_hooks import BoundHooks, CaseTemplates, HistoryNative
from windows_native import Memory, Native, _bind_clock
from windows_oracle_bridge import OracleBridge

OWNER=Path("C:/file-native-online-control-20261020/current-owner.json")


class LaneHost:
    """Construction is allowed only behind the verified bootstrap startup gate."""
    def __init__(self,manifest,manifest_sha256,grant,gate_receipt):
        self.native=None;self.driver=None;self.events=None
        try:
            self._initialize(manifest,manifest_sha256,grant,gate_receipt)
        except BaseException as primary:
            errors=[]
            if self.driver is not None:
                try:self.driver.close_owned("LaneHost-constructor")
                except BaseException as error:errors.append("driver:"+repr(error))
            if self.events is not None:
                try:self.events.close()
                except BaseException as error:errors.append("evidence:"+repr(error))
            if self.native is not None:
                try:self.native.close()
                except BaseException as error:
                    errors.append("native:"+repr(error))
                    if self.native not in Native.FAILED_CONSTRUCTORS:
                        Native.FAILED_CONSTRUCTORS.append(self.native)
            if errors:
                raise RuntimeError("LaneHost-constructor:"+repr(primary)+";cleanup:"+repr(errors)) from primary
            raise

    def _initialize(self,manifest,manifest_sha256,grant,gate_receipt):
        self.manifest=copy.deepcopy(manifest);self.manifest_sha256=manifest_sha256
        self.grant=copy.deepcopy(grant);self.final_pin=manifest["final_pin"]
        self.guard_sha256=manifest["guard"]["sha256"]
        need(gate_receipt["pid"]==os.getpid() and grant["runtime_go"] is True
             and grant["transport_reviewed"] is True,"verified-bootstrap-gate-and-runtime-release")
        self.deadline=Deadline(manifest["absolute_deadline"],FUNCTIONAL-time.time())
        self.source_manifest_sha256=digest(canonical(manifest["source_files"]))
        self.verify_closure()
        need(os.environ.get("FILE_NATIVE_STAGE_JOB")=="1","accepted-outer-QPC-required")
        self.owner=strict(regular_read(OWNER,8192),8192)
        budget=os.environ.get("FILE_NATIVE_BUDGET")
        need(budget and file_hash(budget)==self.owner["budget_sha256"]==grant["budget_sha256"]
             and self.owner["state"]=="running","exact-current-stage-budget-owner")
        self.native=Native(self.owner)
        self.events=None
        self._poll_last_ns=0;self._poll_receipt=None
        self.ledger=GrowthLedger(manifest["roots"],self.poll)
        self.events=Evidence(grant["events_path"],self.ledger,grant["events_category"],
            max_bytes=grant["events_max_bytes"],binding=dict(final_pin=self.final_pin,
            guard_sha256=self.guard_sha256,manifest_sha256=manifest_sha256,claim_id=grant["claim_id"],
            capture_schema="single-raw-ref-v1"))
        self.hooks=BoundHooks(manifest,self.verify_closure)
        engine=manifest["engine"]
        need(engine.get("reviewed") is True and engine["source_pin"]==self.final_pin
             and engine.get("core_reviewed") is True,"final-engine-core-reviewed")
        build=self.hooks.call("build-binding-proof",self,dict(engine=engine))
        need(build.get("binary_sha256")==engine["sha256"] and build.get("final_pin")==self.final_pin,
             "exact-final-build-source-manifest-proof")
        self.driver=WindowsDriver(self.native,engine["path"],engine["sha256"],self.final_pin,
            self.events,self.admit_engine,manifest["absolute_deadline"])
        _bind_clock(self.native,self.driver.registry)
        self.events.context=self.driver.context
        self.oracle=OracleBridge(self);self.adapter=NativeAdapter(self)
        core=importlib.import_module("focused_protocol")
        need(self.source_hash(core.__file__)==engine["core_sha256"],"exact-shared-Harness-source")
        self.harness=core.Harness(self.driver,self.adapter,self.oracle,self.events.append,
                                  final_pin=self.final_pin)
        self.history_authorization=None
        self.history=HistoryNative(self)
        self.case_templates=CaseTemplates(self)
        self.record_counts={};self.record_lock=threading.RLock()
        self.events.settlement_proof=self.writer_closure_proof
        self.events.append(dict(kind="LaneHost-start",gate_receipt=gate_receipt,
            source_manifest_sha256=self.source_manifest_sha256,owner=self.owner))

    def source_hash(self,path):
        key=str(Path(path).resolve())
        rows=[r for p,r in self.manifest["source_files"].items() if Path(p)==Path(key)]
        row=rows[0] if len(rows)==1 else None
        need(row is not None,"source-in-transitive-manifest:"+key)
        need(file_hash(key,row["bytes"])==row["sha256"],"source-bytes-unchanged")
        return row["sha256"]

    def verify_closure(self):
        for path,row in self.manifest["source_files"].items():
            need(file_hash(path,row["bytes"])==row["sha256"],"transitive-source-changed:"+path)
        return self.source_manifest_sha256

    def poll(self):
        """FAST native identity/deadline/RSS only; no directory walk/full hash."""
        self.deadline.poll()
        need(self.native.stage_live(),"live-creation-bound-owned-stage")
        row=Memory();row.cb=__import__("ctypes").sizeof(row)
        need(self.native.PS.GetProcessMemoryInfo(self.native.stage,__import__("ctypes").byref(row),row.cb)
             and 0<row.rss<=CHECKER_BYTES,"positive-stage-RSS")
        now=time.perf_counter_ns()
        receipt=dict(live=True,observed_ns=now,pid=self.owner["job_root_pid"],
            creation_time=self.owner["job_root_creation_time"],rss=row.rss,
            guard_nonce=self.owner["nonce"],guard_sha256=self.guard_sha256,
            budget_sha256=self.owner["budget_sha256"],source_manifest_sha256=self.source_manifest_sha256)
        receipt["receipt_sha256"]=digest(canonical(receipt))
        # This content-addressed fast observation is not a final guard verdict.
        if self.events is not None and now-self._poll_last_ns>=250_000_000:
            self.events.append(dict(kind="fast-native-poll",**receipt))
            self._poll_last_ns=now
        return receipt

    def authorize(self,config,lane):
        self.poll();self.verify_closure()
        spec=self.grant["lanes"].get(lane)
        need(spec is not None and spec["config_sha256"]==digest(canonical(config))
             and spec["go"] is True,"parent-exact-lane-config-GO")
        proof=dict(go=True,lane=lane,final_pin=self.final_pin,
            config_sha256=spec["config_sha256"],claim_id=self.grant["claim_id"],
            budget_sha256=self.owner["budget_sha256"],source_manifest_sha256=self.source_manifest_sha256)
        proof["receipt_sha256"]=self.events.append(dict(kind="lane-authorization",**proof))["sha256"]
        if lane=="history":self.history_authorization=copy.deepcopy(proof)
        return proof

    def _reserves(self,label,value):
        proof=self.hooks.call("forecast-reserves",self,dict(label=label,reserves=value))
        need(proof.get("incremental") is True and proof.get("planned_deletion_credit")==0
             and proof.get("existing_root_counted_once") is True,"closed-incremental-reserves")
        return proof["growth"]

    def checkpoint(self,label,reserves):
        self.poll()
        active=[s for s in self.driver.registry.values() if not s["resources_closed"]]
        need(not active,"checkpoint-no-active-owned-I/O")
        growth=self._reserves(label,reserves)
        proof=self.ledger.checkpoint(label,growth,quiescent=True)
        need(self.grant.get("no_other_semantic_writer") is True,"parent-exclusive-root-writer-release")
        proof.update(go=True,no_other_semantic_writer=True,claim_id=self.grant["claim_id"],
            forecast_sha256=self.grant["whole_phase_forecast_sha256"])
        ref=self.events.append(dict(kind="budget-checkpoint",**proof))
        proof["receipt_sha256"]=ref["sha256"];proof["raw_ref"]=ref
        return proof

    def admit_engine(self,payload,env,code):
        self.poll()
        spec=self.hooks.call("engine-input-policy",self,dict(input=payload,env=env,expected_exit=code))
        need(spec.get("admitted") is True and spec.get("source_pin")==self.final_pin,
             "parent-pinned-engine-JSON-schema")
        self.ledger.root_name(payload["root"])
        token="engine-growth:"+uuid.uuid4().hex
        self.ledger.reserve(token,spec["growth"])
        # Retained increment remains charged after settle, including failure output.
        self.ledger.settle(token)
        profile=copy.deepcopy(getattr(self,"invoke_profile",None) or spec["profile"])
        need(profile.get("api_version")==2,"runtime-v2-capture-only")
        return profile

    def writer_closure_proof(self):
        return dict(schema="closed-writer-settlement-v1",
            source_pin_sha256=self.source_manifest_sha256,profile_sha256=self.grant["profile_sha256"],
            budget_sha256=self.owner["budget_sha256"],
            whole_phase_forecast_sha256=self.grant["whole_phase_forecast_sha256"],
            process_reaped=True,io_closed=True,callbacks_joined=True,raw_writer_released=True)

    def record(self,label,value,category,max_bytes):
        raw=value if type(value) is bytes else canonical(value)+b"\n"
        need(integer(max_bytes,1) and len(raw)<=max_bytes,"record-prewrite-byte-bound")
        with self.record_lock:
            candidate=Path(label)
            if candidate.is_absolute():
                path=confined(Path(self.ledger.roots[category]["root"]),candidate)
            else:
                need(label and all(c.isalnum() or c in "-_:" for c in label),"closed-record-label")
                number=self.record_counts.get(label,0);self.record_counts[label]=number+1
                base=Path(self.ledger.roots[category]["root"])
                path=base/(label.replace(":","-")+"-"+str(number)+".json")
            need(path.parent.is_dir() and not path.exists(),"exclusive-retained-record")
            token="record:"+str(path)
            self.ledger.reserve(token,{category:dict(bytes=max_bytes,files=1,directories=0)})
            with path.open("xb") as stream:
                stream.write(raw);stream.flush()
            self.ledger.settle(token,{category:dict(bytes=len(raw),files=1,directories=0)},
                self.writer_closure_proof())
        return dict(path=str(path),bytes=len(raw),sha256=digest(raw),source_pin=self.final_pin)

    def invoke(self,spec):
        """Closed v2; a decoded response and one raw reference, no stdout copy."""
        need(type(spec) is dict and spec.get("api")=="lane-invoke-v2","explicit-lane-v2")
        common={"kind","api","label","executable","role","timeout_seconds","max_stdout_bytes",
            "max_stderr_bytes","growth","allowed_exit_codes","profile","transport_profile","max_receipt_bytes"}
        extra=({"final_pin","root","action","fields","stdin_release","max_input_bytes","quarantine_release"}
            if spec["kind"]=="engine-json" else {"script","argv","transitive_bindings",
                "startup_gate","required_gate","max_stdin_bytes","output"})
        need(set(spec)==common|extra and len(spec["allowed_exit_codes"])==1,"closed-v2-invocation")
        self.poll();self.verify_closure()
        clock=Deadline(self.deadline.end-12,spec["timeout_seconds"])
        policy=self.hooks.call("lane-invoke-policy",self,dict(spec=spec))
        need(policy.get("admitted") is True and policy.get("exact_argv_and_transitive_closure") is True
             and canonical(policy["profile"])==canonical(spec["transport_profile"]),"reviewed-v2-source-profile")
        if spec["kind"]=="engine-json":
            need(spec["final_pin"]==self.final_pin and spec["executable"]["sha256"]==self.driver.binary_sha256
                 and spec["executable"]["path"]==str(self.driver.executable)
                 and spec["stdin_release"]=="after-owned-containment-and-budget-gate"
                 and "root" not in spec["fields"] and "action" not in spec["fields"],"candidate-v2-fields")
            release=spec["quarantine_release"]
            need(release.get("engine_open_allowed") is True and release.get("root")==spec["root"],
                 "oracle-qualified-quarantine-before-engine")
            self.invoke_profile=copy.deepcopy(spec["transport_profile"])
            try:
                response=self.driver.call(spec["root"],spec["action"],timeout=clock.remaining(),
                    code=spec["allowed_exit_codes"][0],**spec["fields"])
            finally:self.invoke_profile=None
        else:
            need(spec["kind"]=="python-argv" and spec["max_stdin_bytes"]==0
                 and policy.get("startup_gate_before_corpus_IO") is True,"v2-script-gate")
            transport=copy.deepcopy(policy["transport"])
            transport["startup_gate"]["manifest_sha256"]=self.manifest_sha256
            transport["timeout"]=clock.remaining()
            response=self.driver.script(transport,spec["transport_profile"])
        token=getattr(self.driver.context,"completed_token",None)
        row=getattr(self.driver.context,"completed_row",None)
        need(token in self.driver.registry and row and row["resources_closed"] and not row["violations"],
             "v2-actual-owned-call-closure")
        raw=self.driver.registry[token]["raw_reference"]
        need(raw.get("schema")=="single-raw-ref-v1","one-v2-raw-capture")
        known={self.owner["job_root_pid"],os.getpid()}|{s["public_identity"]["pid"]
            for s in self.driver.registry.values() if not s["resources_closed"] and s.get("public_identity")}
        need(self.native.members()<=known,"no-unregistered-owned-descendant")
        closure=self.events.append(dict(kind="invocation-closure",call=token,raw_ref=raw,
            resources_closed=True,callbacks_joined=row["python_callback_joined"]))
        ns=response.get("wall_ns") if spec["kind"]=="engine-json" else None
        need(ns is None and spec["kind"]=="python-argv" or type(ns) is int and 0<=ns<=row["spawn_total_ns"],
             "actual-engine-and-spawn-clocks")
        result=dict(exit_code=row["exit"],source_pin=self.final_pin,
            binary_sha256=row["executable_sha256"],contained_before_input=True,
            children_empty=True,closure_scope="owned-call-process-IO-handles",
            budget_receipt_sha256=policy["budget_receipt_sha256"],closure_receipt_sha256=closure["sha256"],
            raw=raw,response=response,engine_ns=ns,spawn_total_ns=row["spawn_total_ns"],
            observed_peak_rss=row["observed_peak_rss"])
        if spec["kind"]=="python-argv":
            output=spec["output"];path=Path(output["path"])
            need(path.is_file() and path.stat().st_size<=output["max_bytes"],"bound-actual-script-output")
            result.update(startup_gate_before_corpus_IO=True,oracle_sha256=policy["oracle_sha256"],
                output=dict(path=str(path),bytes=path.stat().st_size,sha256=file_hash(path)))
        return result


    def run_case(self,case_id,config):
        from windows_driver import windows_sharing_case
        spec=self.manifest["cases"][case_id];root=Path(spec["root"])
        with self.harness.case(case_id,root,spec["forecast"]):
            if case_id in ("P/windows-loose-no-delete-sharing-A","P/windows-loose-no-delete-sharing-B",
                "F/B-control-no-delete-sharing-remains-retained","Windows-pack-index-no-delete-sharing"):
                return windows_sharing_case(self,case_id,root)
            if case_id=="X-Windows1000":
                return self.harness.mapping_stress(root,spec["forecast"],count=1000,native_focused=False)
            if case_id in ("H1k-H10k-three-cycles-1and4","L-H1k-H10k-exact-long"):
                module=importlib.import_module("history_lane" if case_id.startswith("H") else "long_lane")
                self.source_hash(module.__file__)
                return getattr(module,"run_history" if case_id.startswith("H") else "run_long")(self,config)
            if case_id in ("D-N64-N1k-AandB-payload-title","R-long-stream-restore"):
                return self.invoke(spec["invocation"])
            return self.case_templates.dispatch(case_id,root)
    def drain(self,label):
        if self.events is None and getattr(self,"last_drain",None) is not None:
            return copy.deepcopy(self.last_drain)
        failures=[]
        try:
            self.driver.close_owned("LaneHost-drain:"+label)
        except BaseException as error:
            failures.append(repr(error))
        for control in self.adapter.controls.values():
            try:
                self.adapter._write_signal(control["paths"],"release")
            except BaseException as error:
                failures.append(repr(error))
        for handle in list(self.adapter.handles.values()):
            thread=handle["thread"]
            try:
                if thread.ident is not None:
                    thread.join(min(5,self.deadline.remaining()))
                handle["joined"]=not thread.is_alive()
                if handle.get("error") is not None:
                    failures.append("callback-error:"+handle["id"]+":"+repr(handle["error"]))
                if not handle["joined"]:
                    failures.append("callback-live:"+handle["id"])
            except BaseException as error:
                handle["joined"]=False
                failures.append("callback-join:"+handle["id"]+":"+repr(error))
        rows=list(self.driver.registry.values())
        empty=all(s["done"].is_set() and s["resources_closed"] for s in rows)
        callbacks=all(h["joined"] for h in self.adapter.handles.values())
        proof=dict(label=label,children_empty=empty and callbacks,processes_closed=empty,
            handles_closed=empty,callbacks_joined=callbacks,scope="owned-children-IO-handles",
            failures=failures,outer_QPC_two_empty_Job_polls="required-after-host-exit")
        proof["raw_ref"]=self.events.append(dict(kind="LaneHost-drain",**proof))
        proof["receipt_sha256"]=proof["raw_ref"]["sha256"]
        self.last_drain=copy.deepcopy(proof)
        return proof

    def finalize(self):
        if getattr(self,"finalized",None) is not None:
            return copy.deepcopy(self.finalized)
        errors=[];closure=None;raw=None;actual={}
        try:closure=self.drain("post-final-JSON")
        except BaseException as error:errors.append("drain:"+repr(error))
        stream=self.events;self.events=None
        try:
            if stream is not None:raw=stream.close()
        except BaseException as error:errors.append("evidence:"+repr(error))
        try:
            for name,spec in self.ledger.roots.items():
                row=inventory(spec["root"],self.poll)
                need(row["bytes"]<=spec["bytes"] and row["file_count"]<=spec["files"]
                     and row["directory_count"]<=spec["directories"],"post-final-file-census")
                actual[name]=dict(bytes=row["bytes"],files=row["file_count"],
                    directories=row["directory_count"],inventory_sha256=digest(canonical(row)))
        except BaseException as error:errors.append("census:"+repr(error))
        try:self.native.close()
        except BaseException as error:errors.append("native:"+repr(error))
        self.finalized=dict(raw=raw,actual=actual,owned_closure=closure,failures=errors,
            own_closed=not errors and closure is not None and closure["children_empty"] and not closure["failures"],
            outer_final_QPC_receipt_pending=True)
        return copy.deepcopy(self.finalized)
