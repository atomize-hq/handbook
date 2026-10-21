"""One reviewed LaneHost for LONG, legacy restore and HISTORY."""
import copy
import importlib
import os
from pathlib import Path
import threading
import time
import uuid
from windows_adapter import NativeAdapter,verify_capture_bounds
from windows_deadlines import (CHECKER_BYTES, FUNCTIONAL, GUARD_SHA, Deadline,
Unsupported, Refusal, canonical, digest, integer, need, strict)
from windows_driver import WindowsDriver
from windows_evidence import Evidence,source_admission
from windows_growth import confined, file_hash, GrowthLedger, inventory, regular_read
from windows_hooks import BoundHooks, CaseTemplates, HistoryNative
from windows_native import Memory, Native, _bind_clock
from windows_oracle_bridge import OracleBridge

OWNER=Path("C:/file-native-online-control-20261020/current-owner.json")


class LaneHost:
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
  verify_capture_bounds(self)
  self.events=None
  self._poll_last_ns=0;self._poll_receipt=None
  self.ledger=GrowthLedger(manifest["roots"],self.poll)
  self.events=Evidence(grant["events_path"],self.ledger,grant["events_category"],
  max_bytes=grant["events_max_bytes"],binding=dict(final_pin=self.final_pin,
  guard_sha256=self.guard_sha256,manifest_sha256=manifest_sha256,claim_id=grant["claim_id"],
  capture_schema="single-raw-ref-v1",binary_sha256=manifest["engine"]["sha256"],
  driver_sha256=self.source_hash(__import__("windows_driver").__file__),
  profile_sha256=grant["profile_sha256"],outer_run_id=self.owner["nonce"]))
  self.events.profiles=copy.deepcopy(grant["profile_catalogue"])
  self.events.sideband_bounds=copy.deepcopy(grant["sideband_bounds"])
  from windows_growth import catalogue_digest
  from windows_hooks import SIDEBAND_KINDS
  need(catalogue_digest(self.events.profiles)==grant["profile_sha256"],
  "authenticated-global-profile-catalogue")
  auxiliary=self.events.profiles["profiles"]["capture/sidebands"]
  need(auxiliary==dict(profile_id="capture/sidebands",profile_type="auxiliary",
  sideband_bounds=self.events.sideband_bounds)
  and set(self.events.sideband_bounds)==set(SIDEBAND_KINDS)
  and all(integer(n,1) and n<=4194304 for n in self.events.sideband_bounds.values()),
  "complete-source-owned-catalogue-sideband-bounds")
  self.record_counts={};self.record_lock=threading.RLock();self.case_closures={}
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
  from source_controls import source_preflight
  source_preflight(self)
  self.record_counts={};self.record_lock=threading.RLock();self.case_closures={}
  self.events.settlement_proof=self.writer_closure_proof
  self.events.append(dict(kind="LaneHost-start",gate_receipt=gate_receipt,
  source_manifest_sha256=self.source_manifest_sha256,owner=self.owner,guard_budget_refs=copy.deepcopy(self.grant["guard_budget_refs"]),
  guard_bounds=copy.deepcopy(self.manifest["capture"]["guard_bounds"])))

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
  if self.events is not None and not getattr(self,"census_active",False) and now-self._poll_last_ns>=250_000_000:
   self.events.append(dict(kind="fast-native-poll",**receipt))
   self._poll_last_ns=now
  return receipt

 def authorize(self,config,lane):
  if lane=="long:verified-engine-release":return release_long_root(self,config)
  spec=self.grant["lanes"].get(lane)
  need(spec is not None and spec["config_sha256"]==digest(canonical(config))
  and spec["go"] is True,"parent-exact-original-lane-config-GO")
  self.poll();self.verify_closure()
  prior=getattr(self,"long_authorization",None) if lane=="long" else getattr(self,"history_authorization",None)
  if prior and prior["config_sha256"]==spec["config_sha256"]:return copy.deepcopy(prior)
  admission=None
  if lane in ("history","long"):
   original=config["profile_inputs"]["capture"] if lane=="long" else config["runtime_bindings"]["profile_config"]["data"]["capture"]
   need(original["guard_bounds"]==self.manifest["capture"]["guard_bounds"],"original-helper-guard-bounds")
   admission=source_admission(spec["source_admission"])
   supplied=config["runtime_bindings"]["source_admission"] if lane=="history" else config["source_admission"]
   need(supplied==admission,"config-source-admission-equals-parent-reference")
  proof=dict(go=True,lane=lane,final_pin=self.final_pin,
  config_sha256=spec["config_sha256"],claim_id=self.grant["claim_id"],
  budget_sha256=self.owner["budget_sha256"],source_manifest_sha256=self.source_manifest_sha256)
  if admission is not None:proof["source_admission"]=admission
  if lane=="history":
   freeze=accepted_source_freeze(self,spec["source_freeze"])
   need(config["runtime_bindings"]["history"]["source_freeze"]==freeze,"caller-matches-parent-source-freeze")
   proof["source_freeze"]=freeze
  proof["receipt_sha256"]=self.events.append(dict(kind="lane-authorization",**proof))["sha256"]
  if lane=="history":self.history_authorization=copy.deepcopy(proof)
  if lane=="long":
   self.long_authorization=copy.deepcopy(proof);self.long_config=copy.deepcopy(config)
   category=self.ledger.root_name(Path(config["roots"]["source"]))
   self.ledger.reserve("long-archive",{category:dict(bytes=592458971,files=5807,directories=311)})
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
  self.census_active=True
  try:proof=self.ledger.checkpoint(label,growth,quiescent=True)
  finally:self.census_active=False
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

 def writer_closure_proof(self,owned_file=None):
  if owned_file is not None:
   closed=owned_file.closed
  else:
   closed=all(s["done"].is_set() and s["owner_finished"] and s["resources_closed"]
   for s in self.driver.registry.values()) and all(
   h["joined"] and not h["thread"].is_alive() for h in self.adapter.handles.values())
  need(closed,"actual-scoped-writer-closure")
  return dict(schema="closed-writer-settlement-v1",
  source_pin_sha256=self.source_manifest_sha256,profile_sha256=self.grant["profile_sha256"],
  budget_sha256=self.owner["budget_sha256"],
  whole_phase_forecast_sha256=self.grant["whole_phase_forecast_sha256"],
  process_reaped=closed,io_closed=closed,callbacks_joined=closed,raw_writer_released=closed)

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
   self.writer_closure_proof(stream))
  return dict(path=str(path),offset=0,bytes=len(raw),sha256=digest(raw),source_pin=self.final_pin)

 def invoke(self,spec):
  need(type(spec) is dict and spec.get("api")=="lane-invoke-v2" and spec.get("kind")=="engine-json","engine-invoke-v2")
  common={"kind","api","label","executable","role","timeout_seconds","max_stdout_bytes","max_stderr_bytes",
   "growth","allowed_exit_codes","profile","transport_profile","max_receipt_bytes"}
  extra={"final_pin","root","action","fields","stdin_release","max_input_bytes","quarantine_release"}
  need(set(spec)==common|extra and len(spec["allowed_exit_codes"])==1,"closed-engine-invocation")
  self.poll();self.verify_closure()
  release=self.long_releases[spec["root"]]
  need(self.long_authorization and spec["quarantine_release"]==release
   and release["engine_open_allowed"] and spec["final_pin"]==self.final_pin
   and spec["executable"]["path"]==str(self.driver.executable)
   and spec["executable"]["sha256"]==self.driver.binary_sha256
   and spec["stdin_release"]=="after-owned-containment-and-budget-gate"
   and not {"root","action"}&set(spec["fields"]),"actual-LONG-engine-release")
  from windows_growth import authenticate_action_profile
  profile=copy.deepcopy(spec["transport_profile"])
  authenticate_action_profile(profile,self.events.profiles,self.grant["profile_sha256"])
  need(profile["role"]==spec["role"] and spec["max_stdout_bytes"]==profile["stdout_bytes"]
   and spec["max_stderr_bytes"]==profile["stderr_bytes"] and spec["max_input_bytes"]<=4194304
   and any(spec["growth"]==v for v in self.long_config["phase_reserves"].values()),"original-LONG-source-profile")
  clock=Deadline(self.deadline.end-12,spec["timeout_seconds"])
  token=uuid.uuid4().hex;payload=dict(root=spec["root"],action=spec["action"],**spec["fields"])
  self.events.prepare_pending(token,spec["root"],spec["action"],profile)
  self.events._intent(token,payload,canonical(payload),None)
  self.driver.context.pending_token=token;self.invoke_profile=profile
  try:response=self.driver.call(spec["root"],spec["action"],timeout=clock.remaining(),
   code=spec["allowed_exit_codes"][0],**spec["fields"])
  finally:self.invoke_profile=None
  row=self.driver.context.completed_row;state=self.driver.registry[token]
  need(state["done"].is_set() and state["owner_finished"] and row["resources_closed"] and not row["violations"],
   "engine-owned-call-closure")
  closure=self.events.append(dict(kind="invocation-closure",call=token,raw_ref=state["raw_reference"],
   resources_closed=True,callbacks_joined=row["python_callback_joined"]))
  ns=response.get("wall_ns");need(type(ns) is int and 0<=ns<=row["spawn_total_ns"],"engine-clock")
  return dict(exit_code=row["exit"],source_pin=self.final_pin,binary_sha256=row["executable_sha256"],
   contained_before_input=state["public_identity"]["same_job"],children_empty=row["resources_closed"],
   closure_scope="owned-call-process-IO-handles",budget_receipt_sha256=release["receipt_sha256"],
   closure_receipt_sha256=closure["sha256"],raw=state["raw_reference"],response=response,
   engine_ns=ns,spawn_total_ns=row["spawn_total_ns"],observed_peak_rss=row["observed_peak_rss"])



 def long_stream_capture(self,stream_config,verifier):
  self.ledger.reserve("long-stream",self.grant["long_stream_growth"])
  from windows_native import long_stream_capture
  return long_stream_capture(self,stream_config,verifier)
 def run_case(self,case_id,config):
  from windows_driver import windows_sharing_case,mapping_stress_case
  spec=self.manifest["cases"][case_id];root=Path(spec["root"])
  if case_id in ("H1k-H10k-three-cycles-1and4","L-H1k-H10k-exact-long"):
   return run_lane(self,config,"history" if case_id.startswith("H") else "long")
  if case_id=="R-long-stream-restore":return long_stream_dependency(self)
  if case_id=="X-Windows1000":
   return mapping_stress_case(self,root,spec)
  if spec.get("template",{}).get("function")=="shared":
   return self.case_templates.dispatch(case_id,root)
  with self.harness.case(case_id,root,spec["forecast"]):
   if case_id in ("P/windows-loose-no-delete-sharing-A","P/windows-loose-no-delete-sharing-B",
   "F/B-control-no-delete-sharing-remains-retained","Windows-pack-index-no-delete-sharing"):
    return windows_sharing_case(self,case_id,root)
   if case_id in ("H1k-H10k-three-cycles-1and4","L-H1k-H10k-exact-long"):
    module=importlib.import_module("history_lane" if case_id.startswith("H") else "long_lane")
    self.source_hash(module.__file__)
    return getattr(module,"run_history" if case_id.startswith("H") else "run_long")(self,config)
   if case_id=="D-N64-N1k-AandB-payload-title":return self.case_templates.dispatch(case_id,root)
   if case_id=="R-long-stream-restore":
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
     consumed=handle.get("consumed_error") or {};token=consumed.get("token")
     row=self.driver.registry.get(token,{})
     error=handle["error"];cached=handle.get("finish_proofs",{}).get(consumed.get("kind"),{})
     valid=(consumed.get("error_object") is error and isinstance(error,AssertionError)
     and len(error.args)==1 and type(error.args[0]) is dict
     and digest(canonical(error.args[0]))==consumed.get("row_sha256")
     and row.get("row") is not None
     and digest(canonical(row["row"]))==consumed.get("row_sha256")
     and row["done"].is_set() and row["owner_finished"] and row["resources_closed"]
     and cached.get("receipt_sha256")==consumed.get("proof_sha256")
     and cached.get("call_token")==token and cached.get("closed") is True)
     if not valid:failures.append("callback-error:"+handle["id"]+":"+repr(error))
    if not handle["joined"]:
     failures.append("callback-live:"+handle["id"])
   except BaseException as error:
    handle["joined"]=False
    failures.append("callback-join:"+handle["id"]+":"+repr(error))
  rows=list(self.driver.registry.values())
  empty=all(s["done"].is_set() and s["resources_closed"] for s in rows)
  callbacks=all(h["joined"] for h in self.adapter.handles.values())
  known={os.getpid(),self.owner["job_root_pid"]};polls=[]
  for _ in range(2):
   observed=self.native.members()
   polls.append(dict(observed_ns=time.perf_counter_ns(),members=sorted(observed),
   owned_children=sorted(observed-known)))
   if observed-known:failures.append("unregistered-or-live-owned-descendant")
   time.sleep(.005)
  actual={}
  for name,spec in self.ledger.roots.items():
   facts=inventory(spec["root"],self.poll)
   actual[name]=dict(bytes=facts["bytes"],files=facts["file_count"],
   directories=facts["directory_count"],inventory_sha256=digest(canonical(facts)))
  proof=dict(label=label,children_empty=empty and callbacks and not failures,processes_closed=empty,
  handles_closed=empty,callbacks_joined=callbacks,scope="owned-children-IO-handles",
  failures=failures,owned_members_polls=polls,census=actual,
  outer_QPC_two_empty_Job_polls="required-after-host-exit")
  proof["closure"]=dict(processes_closed=empty,handles_closed=empty,callbacks_joined=callbacks,
  empty_observations=sum(not row["owned_children"] for row in polls),
  post_close_census=len(actual)==len(self.ledger.roots),scope="owned-children-IO-handles")
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


def workspace_case(c):
 import re
 a,h=c.a,c.host
 old=getattr(a,"workspace_proof",None)
 if old is not None:
  need(old["final_pin"]==h.final_pin,"same-final-workspace-proof");return old
 s=a.m["workspace"];c.init("A",16);names=s["expected_test_functions"]
 need(s["reviewed"] is True and s["source_pin"]==h.final_pin and len(names)==31
 and len(set(names))==31,"final31-source-tests")
 for item in s["sources"]:
  need(file_hash(item["path"])==item["sha256"],"test-source")
 fixture=c.root.parent/"unit-fixtures";need(not fixture.exists(),"fresh-unit-prefix")
 key="workspace:"+str(fixture)
 a.budget.reserve_at(key,fixture,s["fixture_max_bytes"],s["fixture_max_files"],s["fixture_max_directories"])
 fixture.mkdir()
 env=dict(EXP_PACK_TEST_ROOT=str(fixture),TMP=str(fixture),TEMP=str(fixture),
 CARGO_INCREMENTAL="0",CARGO_NET_OFFLINE="true",CARGO_HOME=s["cargo_home"])
 common=[h.grant["native_tools"]["cargo"]["path"],"test","--manifest-path",s["manifest_path"],
 "--locked","--offline","--workspace","--release","-j2","--target-dir",s["target_dir"]]
 try:
  listed=a.commands.run(c.root,"workspace",common+["--","--list"],seconds=s["seconds"],env=env)
  tests=re.findall(r"^(.+): test$",listed["stdout"],re.M);need(set(tests)==set(names),"all31-list")
  run=a.commands.run(c.root,"workspace",common+["--","--test-threads=1"],seconds=s["seconds"],env=env)
  passed=re.findall(r"^test (.+) \.\.\. ok$",run["stdout"],re.M)
  need(len(passed)==31 and set(passed)==set(tests) and "FAILED" not in run["stdout"]+run["stderr"],
  "all31-tests")
 finally:a.budget.settle(key)
 facts=inventory(fixture,h.poll);need(facts["files"],"unit-census")
 proof=dict(final_pin=h.final_pin,tests=tests,actual_test_functions=31,
 list_command=listed["token"],test_command=run["token"],artifacts=facts["files"],
 artifact_manifest_sha256=digest(canonical(facts["files"])),
 oracle=c.verify("post-all31-tests"),unit_fixture_prefix=str(fixture))
 a.emit(dict(kind="workspace-all-actual-tests",**proof));a.workspace_proof=proof
 return proof





def accepted_source_freeze(host,freeze):
 """Validate machine acceptance; linked human reviews remain opaque evidence."""
 from datetime import datetime,timezone
 from windows_oracle_bridge import read_ref
 need(type(freeze) is dict and set(freeze)=={"engine","harness","manifest_ref","review_ref"}
 and freeze["engine"]=="2026-10-21T04:10:14Z","closed-R3-engine-and-harness-freeze")
 frozen=datetime.fromisoformat(freeze["harness"].replace("Z","+00:00"))
 need(frozen.tzinfo is not None and frozen.utcoffset().total_seconds()==0
 and datetime(2026,10,21,4,10,14,tzinfo=timezone.utc)<frozen
 and frozen.timestamp()<=time.time()<FUNCTIONAL,"actual-new-harness-freeze-time")
 def machine(ref,maximum):
  need(type(ref) is dict and set(ref)=={"path","offset","bytes","sha256"}
  and ref["offset"]==0 and 0<ref["bytes"]<=maximum,"closed-machine-acceptance-range")
  raw=read_ref(ref,maximum);value=strict(raw,maximum)
  need(raw==canonical(value)+b"\n","canonical-machine-acceptance-bytes")
  return value
 manifest=machine(freeze["manifest_ref"],65536)
 need(set(manifest)=={"kind","version","final_pin","frozen_utc","source_files"}
 and manifest["kind"]=="private-harness-source-manifest-v1" and type(manifest["version"]) is int and manifest["version"]==1
 and manifest["final_pin"]==host.final_pin
 and manifest["frozen_utc"]==freeze["harness"]
 and manifest["source_files"]==host.manifest["source_files"]
 and host.manifest["release"]["reviewed_source_manifest_sha256"]==freeze["manifest_ref"]["sha256"],
 "accepted-exact-native-source-manifest")
 review=machine(freeze["review_ref"],65536)
 need(set(review)=={"kind","version","verdict","final_pin","source_manifest_sha256",
 "source_files_sha256","review_refs"} and review["kind"]=="private-harness-review-acceptance-v1"
 and type(review["version"]) is int and review["version"]==1 and review["verdict"]=="SOURCE_ACCEPTED_FOR_BOUNDED_PHASE_ADMISSION"
 and review["final_pin"]==host.final_pin
 and review["source_manifest_sha256"]==freeze["manifest_ref"]["sha256"]
 and review["source_files_sha256"]==host.source_manifest_sha256
 and type(review["review_refs"]) is list and 0<len(review["review_refs"])<=64,
 "actual-parent-machine-review-association")
 total=0
 for ref in review["review_refs"]:
  need(set(ref)=={"path","offset","bytes","sha256"} and integer(ref["offset"])
  and integer(ref["bytes"],1),"actual-independent-review-reference")
  total+=ref["bytes"];need(total<=1048576,"bounded-linked-review-evidence")
  read_ref(ref,1048576)
 host.verify_closure()
 return copy.deepcopy(freeze)


def ordinary(c,label,selected=None,kind="payload",expected=0):
 import base64
 from windows_native import ORDINARY_PS
 need(kind in ("payload","title") and type(label) is str and 1<=len(label)<=120
 and all(x.isalnum() or x in "-_" for x in label),"owned-ordinary-route")
 destination=c.root.parent/("ordinary-"+label)
 need(not destination.exists(),"preserve-ordinary-prefix")
 tree=c.root/"view"
 if c.document("STORE.json")["mode"]=="B":
  captured=selected or c.selected();raw=c.read("publications/"+captured["publication"]+".json")
  need(digest(raw)==captured["digest"],"ordinary-selected-publication-binding")
  tree=confined(c.root,c.root/"generations"/strict(raw,131072)["attempt"])
 facts=inventory(tree,c.a.host.poll,absent=True)
 lease="ordinary:"+str(destination)
 c.a.budget.reserve_at(lease,destination,facts["bytes"],facts["file_count"],facts["directory_count"]+2)
 tool=c.a.host.grant["native_tools"]["powershell"]
 argv=[tool["path"],"-NoProfile","-NonInteractive","-EncodedCommand",
 base64.b64encode(ORDINARY_PS.encode("utf-16le")).decode("ascii")]
 data=dict(root=str(c.root),destination=str(destination),kind=kind,selected=selected,label=label)
 try:
  row=c.a.commands.run(c.root,"ordinary",argv,expected=expected,seconds=30,
  env=dict(FILE_NATIVE_ORDINARY=canonical(data).decode("utf-8")))
 finally:c.a.budget.settle(lease)
 if expected:
  need(expected==23 and not destination.exists(),"actual-retired-ordinary-refusal")
  return dict(refused=True,exit=row["exit"],raw_token=row["token"])
 answer=strict(row["stdout"].encode("utf-8"),1048576)
 need(set(answer)=={"selected","files","exact_copy","title_count","kind"}
 and answer["exact_copy"] is True and answer["kind"]==kind,"ordinary-native-answer")
 output=inventory(destination/"exact-copy",c.a.host.poll)
 source_files={k:v["sha256"] for k,v in facts["files"].items()}
 need(answer["files"]==source_files
 and output["files"]==facts["files"] and output["directories"]==facts["directories"],
 "ordinary-full-independent-source-and-copy-byte-census")
 return answer

def checker_capture(host,mode,root,fields,*,transport_profile=None,output_prefix=None,limits=None):
 import ast
 from windows_hooks import _catalogue_action
 from windows_oracle_bridge import read_ref
 host.poll();host.verify_closure();root=Path(root)
 policy=host.grant["checker_policy"]
 cli=policy["cli"];path=Path(cli["path"]);raw=regular_read(path,34816)
 need(len(raw)==cli["bytes"] and digest(raw)==cli["sha256"]==host.source_hash(path),
 "actual-reviewed-checker-CLI-source")
 nodes=[n for n in ast.parse(raw).body if isinstance(n,ast.Assign)
 and any(isinstance(t,ast.Name) and t.id=="CONTRACT" for t in n.targets)]
 need(len(nodes)==1,"one-actual-checker-contract");contract=ast.literal_eval(nodes[0].value)
 need(contract["engine_pin"]==host.final_pin and mode in contract["modes"],"checker-mode-and-pin")
 approved=policy["limits"][mode]
 need(limits is None or limits==approved,"exact-parent-checker-limits")
 limits=copy.deepcopy(approved)
 need(set(limits)==set(contract["limits_required"]) and all(integer(v,1) for v in limits.values())
 and limits["max_request_bytes"]<=contract["request_carrier_max_bytes"]
 and limits["max_known_request_bytes"]<=67108864 and limits["normalized_reserve_bytes"]<=67108864
 and 2*limits["proof_bytes"]+limits["payload_bytes"]+limits["error_bytes"]+16384
 <=limits["total_output_bytes"],"finite-checker-part-and-whole-limits")
 try:scope_id,scope=host.adapter.case_for(root)
 except Refusal:
  need(not any(v["root"]==str(root) and not v["closed"] for v in host.adapter.cases.values()),"no-ambiguous-root-scope")
  auth=getattr(host,"long_authorization",None)
  need(auth and str(root) in (host.long_config["roots"]["source"],host.long_config["roots"]["destination"])
  and mode=="legacy_full","authorized-quarantined-LONG-oracle-root")
  scope_id="LONG-"+("source" if str(root)==host.long_config["roots"]["source"] else "destination")
  scope=host.adapter.cases.get(scope_id)
  if scope is None or scope["closed"]:
   scope=dict(root=str(root),closed=False,controls=[],handles=[],lease=auth["receipt_sha256"],
   case_id="L-H1k-H10k-exact-long",scope_label=scope_id,scope_id=scope_id,case_scope=uuid.uuid4().hex,
   opened_ns=time.perf_counter_ns(),derivatives=[],quarantined=True)
   host.adapter.cases[scope_id]=scope
  need(scope["root"]==str(root),"same-quarantined-case-root")
 host.driver.context.value=dict(root=str(root),case_id=scope["case_id"],
 scope_id=scope["scope_id"],case_scope=scope["case_scope"],
 phase=host.adapter.phases.get(str(root),{}).get("label"))
 quiescent=mode!="mapping"
 if quiescent:host.adapter.quiesce(root)
 if mode=="backlog":
  need(not any(s["root"]==str(root) and not s["resources_closed"]
  for s in host.driver.registry.values()),"backlog-only-after-all-root-calls-close")
 maximum=policy["max_calls"][mode];need(integer(maximum,1),"finite-checker-count")
 with host.record_lock:
  key="checker:"+mode;number=host.record_counts.get(key,0)
  need(number<maximum,"checker-call-count");host.record_counts[key]=number+1
 profile=_catalogue_action(host,policy["profile_ids"][mode])
 need(transport_profile is None or canonical(profile)==canonical(transport_profile),
 "exact-source-checker-transport-profile")
 need(profile["role"]=="checker","checker-RSS-role")
 folder=Path(policy["output_roots"][mode])
 category=host.ledger.root_name(folder)
 need(category==policy["category_aliases"]["evidence"] and
 host.ledger.root_name(Path(host.manifest["release"]["controls_root"]))==policy["category_aliases"]["protocol"],
 "actual-checker-and-control-root-aliases")
 confined(Path(host.ledger.roots[category]["root"]),folder)
 need(folder.is_dir() and not folder.is_relative_to(root),"owned-existing-checker-artifact-root")
 prefix=Path(output_prefix) if output_prefix is not None else folder/("checker-"+mode+"-"+str(number)+".json")
 confined(folder,prefix)
 need(prefix.parent.is_dir() and not prefix.exists(),"fresh-checker-prefix")
 req=dict(version=contract["version"],mode=mode,root=str(root),quiescent=quiescent,
 limits=limits,case_id=scope["case_id"],scope_id=scope["scope_id"],case_scope=scope["case_scope"])
 fields=copy.deepcopy(fields)
 def reference(ref):
  if ref is None:return None
  result=dict(path=ref["path"],offset=ref.get("offset",0),bytes=ref["bytes"],sha256=ref["sha256"])
  allowed=list(policy["request_provenance"].values())+list(policy["quota_evidence"].values())
  if not any(r and all(result[k]==r.get(k,0) for k in result) for r in allowed):
   host.ledger.root_name(Path(result["path"]))
  read_ref(result,limits["max_known_request_bytes"])
  return result
 known=fields.pop("known_requests",None)
 if mode in ("full","history_oracle","legacy_full","refuse"):
  if known is not None:
   ref=host.record(str(prefix.with_suffix(".known.json")),known,category,limits["max_known_request_bytes"])
   fields["known_requests_ref"]=reference(ref)
  fields.setdefault("known_requests_ref",None)
  fields.setdefault("request_provenance_ref",policy["request_provenance"].get(str(root)))
  if mode!="legacy_full":fields.setdefault("quota_evidence_ref",policy["quota_evidence"].get(str(root)))
  for name in ("known_requests_ref","request_provenance_ref","quota_evidence_ref"):
   if name in fields:fields[name]=reference(fields[name])
  fields["final_pin"]=host.final_pin
  if mode=="refuse":fields.update(policy["refusals"][fields["checkpoint"]])
  else:fields.setdefault("required_mapped_keys",[])
 if fields.get("physical_only"):fields["route"]="physical-orphan"
 req.update(fields)
 req["sources"]=copy.deepcopy(policy["sources"])
 for source,sha in req["sources"].items():need(host.source_hash(source)==sha,"actual-checker-transitive-source")
 schema=contract["mode_fields"][mode]
 if type(schema) is str:schema=contract["mode_fields"][schema]
 need(set(contract["request_required"])|set(schema[0])<=set(req)
 <=set(contract["request_required"])|set(schema[0])|set(schema[1])|{"sources"},
 "actual-CLI-closed-request-fields")
 request=host.record(str(prefix.with_suffix(".request.json")),req,category,limits["max_request_bytes"])
 files=2*(len(contract["artifacts"])-1)+1
 host.ledger.reserve("checker-output:"+str(prefix),{category:dict(
 bytes=limits["total_output_bytes"],files=files,directories=0)})
 python=policy["python"];need(file_hash(python["path"])==python["sha256"],"actual-reviewed-Python-image")
 nonce=uuid.uuid4().hex
 control=Path(host.manifest["release"]["controls_root"])/(host.owner["nonce"]+"-"+mode+"-"+str(number))
 gate=dict(ready=str(control.with_suffix(".ready")),release=str(control.with_suffix(".release")),
 nonce=nonce,manifest_sha256=host.manifest_sha256,root=str(root))
 budget_ref=host.events.append(dict(kind="budget-checkpoint",scope="checker-output-prereservation",
 lease="checker-output:"+str(prefix),limits_sha256=digest(canonical(limits)),
 budget_sha256=host.owner["budget_sha256"],profile_sha256=host.grant["profile_sha256"],
 whole_phase_forecast_sha256=host.grant["whole_phase_forecast_sha256"]))
 host.driver.script(dict(kind="python",argv=[python["path"],"-I","-S","-B","-u",str(path),"--request",request["path"],
 "--output",str(prefix)],startup_gate=gate,expected_exit=0,timeout=profile["timeout_seconds"],
 output_mode="json",executable_sha256=python["sha256"]),profile)
 token=host.driver.context.completed_token;state=host.driver.registry[token];row=host.driver.context.completed_row
 need(state["done"].is_set() and state["owner_finished"] and row["resources_closed"]
 and row["python_callback_joined"] and not row["violations"],"actual-checker-native-call-closure")
 raw=regular_read(prefix,16384);index=strict(raw,16384)
 need(index["mode"]==mode and index["engine_pin"]==host.final_pin and set(index["sources"])==set(req["sources"])
 and all(index["sources"][p]==dict(bytes=Path(p).stat().st_size,sha256=h) for p,h in req["sources"].items())
 and all(index[k]==req[k] for k in ("case_id","scope_id","case_scope"))
 and index["status"] in ("passed","expected-independent-refusal"),"actual-CLI-index-binding")
 closure=host.events.append(dict(kind="invocation-closure",call=token,raw_ref=state["raw_reference"],
 resources_closed=True,callbacks_joined=row["python_callback_joined"],
 request_ref=reference(request),output_ref=dict(path=str(prefix),offset=0,bytes=len(raw),sha256=digest(raw))))
 return dict(_host=host,budget_ref=budget_ref,index=index,index_ref=dict(path=str(prefix),offset=0,bytes=len(raw),sha256=digest(raw)),
 request_ref=reference(request),native_raw=state["raw_reference"],call_token=token,
 closure_ref=closure,contract=contract,limits=limits)

CHECKER_WRITER_CONTRACT={'api': 'checker-writer-r3', 'max_live': 1, 'output_reserved_names': 17, 'gate_hold_bytes': 16384, 'gate_hold_files': 2, 'settles_outputs': True, 'settles_gates': True, 'failure': 'retain-full-unsettled-and-stop'}

def release_long_root(h,v):
 from windows_growth import read_ref
 need(getattr(h,"long_authorization",None) and set(v)=={
 "root","final_pin","byte_proof","oracle","inventory","forecast_sha256"}
 and v["final_pin"]==h.final_pin,"long-scope")
 g=h.long_config;root=v["root"]
 need(root in (g["roots"]["source"],g["roots"]["destination"]),"long-root")
 c=h.checker_results[root];state=h.driver.registry[c["call_token"]]
 need(state["root"]==root and state.get("checker_writes_retired") is True
 and state["resources_closed"] and state["owner_finished"],"oracle-closed")
 def ref(row):
  return dict(path=row["path"],offset=row.get("offset",0),bytes=row["bytes"],sha256=row["sha256"])
 r=c["artifacts"]["result"]
 need(ref(v["oracle"])==r,"oracle-ref")
 request=strict(read_ref(c["request_ref"],262144),262144)
 need(request["mode"]=="legacy_full" and request["root"]==root and request["quiescent"] is True,
 "full-request")
 need(c["full"] and c["mode"]=="legacy_full","legacy-full")
 from windows_oracle_bridge import long_inventory_digest
 sha,file_count,dir_count=long_inventory_digest(h,c)
 inv=strict(read_ref(ref(v["inventory"]),4096),4096)
 need(inv["before"]==inv["after"]==sha
 and inv["files"]==file_count and inv["directories"]==dir_count,
 "inventory")
 base=Path(g["roots"]["evidence"])
 f=base/"long-forecast.json"
 need(file_hash(f,g["limits"]["record_bytes"])==v["forecast_sha256"],
 "forecast")
 if root==g["roots"]["source"]:
  byte=strict(read_ref(ref(v["byte_proof"]),16384),16384)
  need(Path(v["byte_proof"]["path"])==base/"archive-byte-proof.json"
  and byte["status"]=="legacy-byte-restore-complete-quarantined"
  and byte["engine_open_allowed"] is False and byte["file_inventory_sha256"]==sha,
  "legacy-bytes")
 else:
  byte=v["byte_proof"]
  a=strict(regular_read(Path(g["stream"]["metadata_root"])/"result.json",2097152),2097152)
  need(byte==a and byte["source"]==g["roots"]["source"] and byte["destination"]==root
  and byte["both_helpers_closed"] is True and byte["destination_inventory_sha256"]==sha
  and byte["transport"]["gzip_crc_trailer_single_member_eof"] is True
  and byte["transport"]["FNOP1_exact_eof"] is True,"stream-bytes")
 p=dict(go=True,engine_open_allowed=True,root=root,final_pin=h.final_pin,
 oracle_sha256=r["sha256"],
 full_index_ref=c["index_ref"],inventory_sha256=sha,forecast_sha256=v["forecast_sha256"])
 p["receipt_sha256"]=h.events.append(dict(kind="lane-authorization",
 lane="long:verified-engine-release",**p))["sha256"]
 if not hasattr(h,"long_releases"):h.long_releases={}
 h.long_releases[root]=copy.deepcopy(p)
 return p

LONG_STREAM_CONTRACT={'api': 'long-stream-native-r3', 'helper_ids': ['long/stream-produce', 'long/stream-consume'], 'max_live': 2, 'control_stdio': {'stdin_bytes': 0, 'stdout_bytes': 4096, 'stderr_bytes': 8192}, 'external_payload': {'protocol': 'FNOP1-gzip-single-member', 'endpoint': 'one-dedicated-inherited-anonymous-pipe', 'retained_payload_bytes': 0, 'proof': 'independent-produce-consume-byte-counts-SHA256-CRC-EOF-plus-full-inventory'}, 'notes': {'script-startup-gate-ready': 2, 'invocation-closure': 2, 'callback-wait-result': 2}, 'failure_callback_waits_upper': 4, 'metadata_files_upper': 12, 'native_raw_refs': 2, 'gates': 2, 'callbacks': 2, 'proof_fields': ['provider_api', 'native_helpers'], 'native_helper_fields': ['role', 'raw_ref', 'metadata_ref', 'closure_ref', 'gate_ref']}

def run_lane(host,config,lane):
    import importlib.util
    host.authorize(config,lane)
    paths=[Path(p) for p in host.manifest["source_files"] if Path(p).name==lane+"_lane.py"]
    need(len(paths)==1,"one-authorized-lane-source")
    path=paths[0];raw=regular_read(path,131072)
    need(digest(raw)==host.source_hash(path),"same-lane-source-buffer")
    spec=importlib.util.spec_from_file_location("lane_"+lane+"_"+digest(raw),path)
    module=importlib.util.module_from_spec(spec)
    exec(compile(raw,str(path),"exec",dont_inherit=True),module.__dict__)
    value=getattr(module,"run_"+lane)(host,config)
    if lane=="long":
        from windows_growth import read_ref
        need(type(value) is dict and set(value)=={"path","offset","bytes","sha256"}
            and value["offset"]==0 and Path(value["path"])==Path(config["roots"]["evidence"])/"long-result.json",
            "actual-LONG-result-reference")
        raw=read_ref(value,config["limits"]["result_bytes"])
        proof=strict(raw,config["limits"]["result_bytes"])
        need(proof["source_pin"]==host.final_pin and
            proof["status"]=="PROVISIONAL_UNQUALIFIED-awaiting-postexit-QPC-join",
            "actual-LONG-result-body")
        host.long_result_ref=copy.deepcopy(value);host.long_result=proof
    return value


def long_stream_dependency(host):
    from windows_growth import read_ref
    result_ref=host.long_result_ref
    proof=strict(read_ref(result_ref,host.long_config["limits"]["result_bytes"]),
        host.long_config["limits"]["result_bytes"])
    need(proof==host.long_result and proof["source_pin"]==host.final_pin,
        "same-hash-bound-original-LONG-result")
    transport=proof["transport"]
    need(transport["provider_api"]=="long-stream-native-r3" and transport["both_helpers_closed"] is True
        and len(transport["native_helpers"])==2 and proof["successor_publish"] and proof["successor_resolve"],
        "actual-complete-LONG-stream-dependency")
    return dict(origin_case_id="L-H1k-H10k-exact-long",result_ref=copy.deepcopy(result_ref),transport=copy.deepcopy(transport),
        restored_oracle=copy.deepcopy(proof["restored_oracle"]),successor_oracle=copy.deepcopy(proof["successor_oracle"]),
        source_pin=host.final_pin,qualification="PROVISIONAL_UNQUALIFIED-awaiting-postexit-QPC-join")
