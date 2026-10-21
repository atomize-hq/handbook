"""NativeAdapter for the unchanged shared Harness. No import-time I/O/native calls."""
import copy
from contextlib import contextmanager
import os
from pathlib import Path
import threading
import time
import uuid
from windows_deadlines import (CHECKER_BYTES, READER_BYTES, READERS, Deadline,
GUARD_SHA, Unsupported, canonical, digest, integer, need, strict)
from windows_growth import confined, file_hash, inventory, regular_read


class NativeAdapter:
 def __init__(self,host):
  self.host=host
  self.driver=host.driver
  self.lock=threading.RLock()
  self.handles={};self.controls={};self.cases={};self.phases={}
  from windows_driver import NativeCommands,NativeBudget
  self.commands=NativeCommands(self);self.budget=NativeBudget(self)
  host.events.callback_handles=self.handles;host.events.driver_registry=self.driver.registry
# Merely listing a source checkpoint does not advertise its implementation.
  self.capabilities=set(host.hooks.capabilities())
  self.driver.observer=self.observe_start

 def wall_time(self):
  return time.time()

 def monotonic(self):
  return time.perf_counter()

 def sleep(self,seconds):
  end=time.perf_counter()+seconds
  need(0<=seconds<60,"bounded-Harness-sleep")
  while time.perf_counter()<end:
   self.host.poll()
   time.sleep(min(.005,max(0,end-time.perf_counter())))

 def preflight(self,case_id,root,forecast,final_pin):
  need(final_pin==self.host.final_pin,"case-final-pin")
  scope=self.host.grant.get("case_scopes",{}).get(case_id)
  required=scope["case_id"] if scope else case_id
  spec=self.host.manifest["cases"].get(required)
  need(spec and spec.get("reviewed") is True,"reviewed-required-case:"+required)
  expected=scope if scope else spec
  need(str(root)==expected["root"] and digest(canonical(forecast))==expected["forecast_sha256"],
  "exact-root-source-profile-case-scope")
  need(case_id not in self.cases or self.cases[case_id]["closed"],"case-scope-already-active")
  with self.lock:
   self.cases[case_id]=dict(root=str(root),closed=False,controls=[],handles=[],
   lease=None,case_id=required,scope_label=case_id,
   scope_id=case_id,case_scope=uuid.uuid4().hex,opened_ns=time.perf_counter_ns(),derivatives=[])
  current=self.cases[case_id]
  self.driver.context.value=dict(root=str(root),case_id=required,scope_id=case_id,
  case_scope=current["case_scope"])
  permit=self.host.checkpoint("case:"+case_id,forecast)
  current["lease"]=permit["receipt_sha256"]
  return dict(go=True,case_id=case_id,root=str(root),final_pin=final_pin,
  reader_limit=READER_BYTES,checker_limit=CHECKER_BYTES,default_payloads=True,
  heavy_token=self.host.owner["nonce"],host_guard=self.host.owner["receipt"],
  binary_sha256=self.driver.binary_sha256,binary_source_pin=final_pin,
  source_manifest_sha256=self.host.source_manifest_sha256,
  adapter_sha256=self.host.source_hash(__file__),guard_sha256=self.host.guard_sha256,
  budget_receipt_sha256=permit["receipt_sha256"])

 def case_for(self,root):
  matches=[(n,r) for n,r in self.cases.items() if r["root"]==str(root) and not r["closed"]]
  need(len(matches)==1,"one-admitted-native-case-for-root")
  return matches[0]

 def before_call(self,root,action,role,timeout):
  scope_id,scope=self.case_for(root)
  need(role==("reader" if action in READERS else "checker"),"independent-action-role")
  self.host.poll()
  context=copy.deepcopy(getattr(self.driver.context,"value",{}))
  context.update(root=str(root),phase=self.phases.get(str(root),{}).get("label"),
  case_id=scope["case_id"],scope_id=scope["scope_id"],case_scope=scope["case_scope"])
  self.driver.context.value=context
  profile=copy.deepcopy(self.host.grant["transport_profiles"][scope["case_id"]][action])
  launch=dict(argv=[str(self.driver.executable)],
  executable_sha256=self.driver.binary_sha256,producer_kind="candidate",
  producer_source_pin=self.host.final_pin,source_files=profile.get("producer_source_files",[]),startup_gate=None)
  token=uuid.uuid4().hex
  self.host.events.prepare_pending(token,root,action,profile,launch)
  self.driver.context.pending_token=token

 def read(self,root,relative,limit):
  need(type(relative) is str and "\\" not in relative and ":" not in relative
  and all(x not in ("",".","..") for x in relative.split("/")),"confined-relative-read")
  self.host.ledger.root_name(root)
  return regular_read(confined(Path(root),Path(root)/relative),limit)

 def exists(self,root,relative):
  self.host.ledger.root_name(root)
  return confined(Path(root),Path(root)/relative).exists()

 def file_sha256(self,root,relative):
  self.host.ledger.root_name(root)
  return file_hash(confined(Path(root),Path(root)/relative),poll=self.host.poll)

 def inventory(self,root):
  self.host.ledger.root_name(root)
  return inventory(root,self.host.poll)

 def control(self,root,name):
  case_id,case=self.case_for(root)
  need(type(name) is str and name and name.isascii() and len(name)<=120
  and all(c.isalnum() or c in "-_" for c in name),"closed-control-name")
  key=uuid.uuid4().hex
  folder=Path(self.host.manifest["release"]["controls_root"])
  category=self.host.ledger.root_name(folder)
  self.host.ledger.reserve("control:"+key,{category:dict(bytes=24576,files=3,directories=0)})
  values={kind:str(confined(folder,folder/(key+"-"+name+"."+kind)))
  for kind in ("ready","release","cancel")}
  need(folder.is_dir() and not any(Path(v).exists() for v in values.values()),"fresh-owned-controls")
  with self.lock:
   self.controls[key]=dict(paths=values,root=str(root),case_id=case_id,ended={},
   bytes_written=0)
   case["controls"].append(key)
  return copy.deepcopy(values)

 def _control(self,control):
  found=[(k,v) for k,v in self.controls.items() if v["paths"]==control]
  need(len(found)==1,"owned-exact-control")
  return found[0]

 def _write_signal(self,control,kind):
  key,row=self._control(control)
  path=Path(row["paths"][kind])
  if path.exists():
   need(regular_read(path,8192)==b"release\n","owned-signal-byte-binding")
   return
  need(not row.get("closed"),"retired-control-cannot-write")
  if kind=="cancel":need(kind not in row["ended"],"ended-cancel-cannot-be-recreated")
  with path.open("xb") as stream:
   need(stream.write(b"release\n")==8,"complete-control-signal-write");stream.flush()
  row["bytes_written"]+=8
  need(row["bytes_written"]<=24576,"control-reservation")
  self.host.events.append(dict(kind="control-signal",key=key,signal=kind,path=str(path),
  sha256=digest(b"release\n")))

 def release(self,control):
  self.host.poll()
  self._write_signal(control,"release")

 def signal(self,control,kind):
  need(kind=="cancel","closed-cancel-kind")
  self._write_signal(control,"cancel")

 def end_signal(self,control,kind):
  need(kind=="cancel","closed-end-cancel-kind")
  key,row=self._control(control)
  need(not row.get("closed"),"retired-control-cannot-rename")
  source=Path(row["paths"][kind])
  need(source.exists() and regular_read(source,8192)==b"release\n","retained-cancel-before-end")
  destination=source.with_suffix(".cancel-ended")
  need(not destination.exists(),"fresh-ended-cancel-evidence")
  source.rename(destination)
  row["ended"][kind]=str(destination)
  self.host.events.append(dict(kind="cancel-ended-by-owned-rename",key=key,
  path=str(destination),sha256=file_hash(destination),bytes_retained=True))

 def spawn(self,callback):
  need(callable(callback),"injected-thread-callback")
  inherited=copy.deepcopy(getattr(self.driver.context,"value",{}))
  root=inherited.get("root")
  if root is None:
   active=[r["root"] for r in self.cases.values() if not r["closed"]]
   need(len(active)==1,"unambiguous-spawn-root")
   root=active[0]
  _,case=self.case_for(root)
  identity=uuid.uuid4().hex
  handle=dict(id=identity,root=root,case_scope=case["case_scope"],response=None,error=None,native_tokens=[],
  started_ns=time.perf_counter_ns(),joined=False)
  def invoke():
   self.driver.context.value=dict(inherited,handle_id=identity,root=root,
   phase=self.phases.get(root,{}).get("label"))
   try:
    handle["response"]=callback()
   except BaseException as error:
    handle["error"]=error
   finally:
    handle["done_ns"]=time.perf_counter_ns()
  thread=threading.Thread(target=invoke,name="guarded-call-"+identity,daemon=False)
  handle["thread"]=thread
  with self.lock:
   self.handles[identity]=handle;case["handles"].append(identity)
  try:
   thread.start()
  except BaseException as error:
   handle["error"]=error;handle["joined"]=not thread.is_alive()
   handle["done_ns"]=time.perf_counter_ns()
   raise
  return handle


 def _retire_control(self,key):
  row=self.controls[key]
  with self.lock:
   if row.get("closed"):return
   proof=self.host.writer_closure_proof()
   paths=set(row["paths"].values())|set(row["ended"].values())
   actual=[]
   for name in paths:
    path=Path(name)
    if path.exists():
     raw=regular_read(path,8192)
     actual.append(dict(path=name,bytes=len(raw),sha256=digest(raw)))
   need(len(actual)<=3 and sum(r["bytes"] for r in actual)<=24576,"closed-control-reservation")
   row["closed"]=True
   category=self.host.ledger.root_name(Path(row["paths"]["ready"]).parent)
   self.host.ledger.settle("control:"+key,{category:dict(
   bytes=sum(r["bytes"] for r in actual),files=len(actual),directories=0)},proof)
   row["settlement"]=dict(actual=actual,proof=proof)

 def _handle(self,handle):
  need(type(handle) is dict and self.handles.get(handle.get("id")) is handle,
  "opaque-owned-callback-handle")
  return handle

 def observe_start(self,event):
  self.host.poll()
  key=event.get("context",{}).get("handle_id")
  with self.lock:
   if key is not None:
    need(key in self.handles,"start-associated-with-owned-callback")
    self.handles[key]["native_tokens"].append(event["call_token"])
   self.host.events.append(dict(kind="callback-native-association",event=event))

 def alive(self,handle):
  handle=self._handle(handle)
  return handle["thread"].is_alive() and any(self.driver.live(t) for t in handle["native_tokens"])

 def wait(self,handle,timeout):
  handle=self._handle(handle)
  end=min(timeout,self.host.deadline.remaining())
  if handle["thread"].ident is not None:
   handle["thread"].join(max(0,end))
  handle["joined"]=not handle["thread"].is_alive()
  returned=handle["response"] if handle["joined"] and handle["error"] is None else None
  sha=digest(canonical(returned))
  ref=self.host.events.append(dict(kind="callback-wait-result",handle_id=handle["id"],root=handle["root"],
  case_scope=handle["case_scope"],joined=handle["joined"],response_sha256=sha,
  native_tokens=len(handle["native_tokens"]),tokens_sha256=digest(canonical(handle["native_tokens"])),
  error_type=type(handle["error"]).__name__ if handle["error"] is not None else None))
  queued=getattr(self.driver.context,"callback_waits",[])
  need(len(queued)<100000,"bounded-callback-wait-registry")
  queued.append(dict(handle_id=handle["id"],response_sha256=sha,proof_ref=ref))
  self.driver.context.callback_waits=queued
  need(handle["joined"],"bounded-callback-wait-incomplete")
  if handle["error"] is not None:
   raise handle["error"]
  return handle["response"]

 def await_ready(self,control,handle,timeout):
  self._control(control);handle=self._handle(handle)
  deadline=Deadline(self.host.deadline.end,timeout)
  ready=Path(control["ready"])
  while not ready.exists():
   deadline.poll();self.host.poll()
   need(handle["thread"].is_alive(),"callback-ended-before-real-ready")
   time.sleep(.005)
  need(self.alive(handle),"ready-requires-live-native-identity")
  raw=regular_read(ready,8192)
  self.host.events.append(dict(kind="native-ready",control=control,handle_id=handle["id"],
  raw_sha256=digest(raw),observed_ns=time.perf_counter_ns()))
# Source-reviewed ready schema/checkpoint correspondence, not file presence alone.
  return self.host.hooks.call("ready-proof",self.host,dict(control=control,
  handle_id=handle["id"],tokens=list(handle["native_tokens"]),
  raw_sha256=digest(raw),root=handle["root"]))

 def begin_phase(self,root,label):
  self.case_for(root)
  self.phases[str(root)]=dict(label=label,start_ns=time.perf_counter_ns())
  self.driver.context.value=dict(root=str(root),phase=label)

 def overlap(self,root,label,publishers,readers):
  rows=[s["row"] for s in self.driver.registry.values() if s.get("row") is not None
  and s["root"]==str(root) and s["row"]["context"].get("phase")==label]
  proof=self.host.hooks.call("overlap-proof",self.host,dict(root=str(root),label=label,
  publishers=publishers,readers=readers,rows=rows))
  need(proof.get("independent_reader_processes")==4
  and proof.get("publisher_processes",0)>=publishers
  and proof.get("maintenance_overlap") is True,"actual-native-overlap-required")
  return proof

 def mapping(self,root,key):
  return self.host.oracle.query("mapping",root,key=key)

 def backlog(self,root):
  proof=self.host.oracle.query("backlog",root)
  need(all(type(proof.get(k)) is int and proof[k]>=0
  for k in ("unprotected","protected","excluded")),"complete-independent-backlog")
  return proof

 def quiesce(self,root):
  active=[s for s in self.driver.registry.values() if s["root"]==str(root)
  and not s["resources_closed"]]
  if active:
   proof=self.host.hooks.call("quiescence-proof",self.host,dict(root=str(root),
   calls=[s["token"] for s in active]))
   need(proof.get("only_authentic_paused_readers") is True
   and proof.get("no_active_writer") is True,"native-quiescent-or-authentic-paused-readers")
  return dict(root=str(root),no_active_writer=True)

 def _end_token(self,handle):
  handle=self._handle(handle)
  matches=set(handle["native_tokens"])|{s["token"] for s in self.driver.registry.values()
  if s["context"].get("handle_id")==handle["id"]}
  need(len(matches)==1,"one-exact-reader-call-for-owned-callback")
  return next(iter(matches))

 def interrupt(self,handle,kind):
  handle=self._handle(handle)
  need(kind in ("cancel","death"),"closed-reader-interrupt-kind")
  cached=handle.setdefault("finish_proofs",{})
  if kind in cached:
   return copy.deepcopy(cached[kind])
  token=self._end_token(handle)
  self.driver.interrupt(token,kind)
# The frozen caller requires CLOSED proof now, not a signal request.
  return self.finish_interrupted(handle,kind)

 def finish_interrupted(self,handle,kind):
  handle=self._handle(handle)
  need(kind in ("error","cancel","death"),"closed-reader-finish-kind")
  cached=handle.setdefault("finish_proofs",{})
  if kind in cached:
   return copy.deepcopy(cached[kind])
  need(not cached,"conflicting-reader-finish-kind")
  token=self._end_token(handle)
  try:
   row=self.driver.wait_control(token,5)
  finally:
   if handle["thread"].ident is not None:
    handle["thread"].join(min(5,max(0,self.host.deadline.remaining())))
   handle["joined"]=not handle["thread"].is_alive()
  state=self.driver.registry[token]
  need(handle["joined"] and state["owner_finished"] and state["done"].is_set()
  and row.get("resources_closed") is True and row.get("raw_retained") is True
  and row.get("stdout_eof") is True and row.get("stderr_eof") is True
  and row.get("python_callback_joined") is True and row["violations"]==[],
  "reader-owner-raw-EOF-resource-and-callback-closure")
  from windows_driver import maintainer_death_match
  maintainer=kind=="death" and maintainer_death_match(self.driver,state)
  need(row["executable_sha256"]==self.driver.binary_sha256
  and (row["input"]==dict(root=handle["root"],action="resolve",request_id="initial")
  or maintainer and row["input"]==strict(self.host.events.calls[token]["encoded"],4194304)),
  "exact-source-bound-interrupted-call")

  native=row.get("native_identity") or {}
  start=row.get("positive_start_event") or {}
  need(type(native.get("pid")) is int and native["pid"]>0
  and type(native.get("creation_time")) is int and native["creation_time"]>0
  and native.get("handle_identity_verified") is True and native.get("same_job") is True
  and native.get("pid")==start.get("pid") and native.get("creation_time")==start.get("creation_time")
  and start.get("native_handle_identity") is True and start.get("before_stdin_delivery") is True
  and type(start.get("positive_rss")) is int and 0<start["positive_rss"]<=(CHECKER_BYTES if maintainer else READER_BYTES),
  "raw-positive-native-reader-identity")
  error=handle["error"]
  if kind=="error":
   need(row["intentional_interrupt"] is None and row["exit"]==2
   and row.get("env",{}).get("EXP_ERROR")=="pack-catalog-captured"
   and row["env"].get("EXP_BARRIER")=="pack-catalog-captured"
   and error is None,"ordinary-injected-error-is-not-an-interrupt")
   response=strict(row["stdout"].encode("utf-8"),4*1024**2)
   need(response.get("status")=="error"
   and response.get("error")=="injected-io:pack-catalog-captured",
   "exact-ordinary-injected-error-row")
  else:
   need(row["intentional_interrupt"]==kind and row["exit"]==(138 if kind=="cancel" else 137),
   "exact-identity-owned-interrupt-exit")
   need(isinstance(error,AssertionError) and len(error.args)==1
   and type(error.args[0]) is dict and canonical(error.args[0])==canonical(row),
   "consume-only-this-call-intentional-AssertionError-row")
  proof=self.host.hooks.call("interruption-proof",self.host,
  dict(root=handle["root"],kind=kind,row=row,callback_joined=True,
  raw_reference=state.get("raw_reference")))
  proof.update(identity_verified=True,closed=True,call_token=token,
  kind=kind,semantic_noncommit=False,scope=("owned-maintainer-transport-closure" if maintainer
  else "owned-reader-transport-closure"),
  ordinary_injected_error=kind=="error",owner_finished=True,
  raw_reference=state.get("raw_reference"))
  proof["receipt_sha256"]=self.host.events.append(dict(kind="reader-end-closed",proof=proof))["sha256"]
  cached[kind]=copy.deepcopy(proof)
  if kind in ("cancel","death"):
   handle["consumed_error"]=dict(error_object=error,row_sha256=digest(canonical(row)),
   token=token,kind=kind,proof_sha256=proof["receipt_sha256"])
  return proof

 def removal_proof(self,root,before,phase):
  after=self.inventory(root)
  removed=[name for name in before["files"] if name not in after["files"]]
  need(removed,"actual-old-instance-removal-required")
  return self.host.hooks.call("removal-proof",self.host,
  dict(root=str(root),phase=phase,before=before,after=after,removed_paths=removed))

 def crash_source_binding(self,root,phase):
  return self.host.hooks.binding("constructor:"+phase,self.host.final_pin)

 def extend(self,harness,root,spec):
  need(harness is self.host.harness,"same-reviewed-Harness")
  return self.host.hooks.call(spec["capability"],self.host,dict(root=str(root),spec=spec))

 def prepared_candidates(self,root,requests):
  need(type(requests) is list and len(requests)==4
  and all(type(r) is dict and type(r.get("request")) is str for r in requests),
  "four-actual-prepared-requests")
  ids=[r["request"] for r in requests];need(len(set(ids))==4,"distinct-prepared-request-IDs")
  self.case_for(root)
  proof=self.host.hooks.call("prepared-candidates",self.host,dict(root=str(root),requests=ids))
  need(set(proof)=={"root","candidates"} and proof["root"]==str(root)
  and set(proof["candidates"])==set(ids),"exact-owned-prepared-candidate-map")
  actual={}
  for request in requests:
   key=request["request"];owner=proof["candidates"][key]
   need(set(owner)=={"path","sha256","bytes_length"} and owner["bytes_length"]<=4194304,
   "closed-prepared-candidate-descriptor")
   path=confined(Path(root),Path(root)/owner["path"])
   raw=regular_read(path,4194304)
   need(len(raw)==owner["bytes_length"] and digest(raw)==owner["sha256"],
   "same-buffer-sealed-candidate")
   value=strict(raw,4194304)
   need(value["request"]==request and value["expected"]==request["base"]
   and owner["path"]=="attempts/"+value["attempt"]+"/candidate.json",
   "exact-source-owned-request-and-attempt")
   actual[key]=dict(path=owner["path"],bytes=raw,sha256=digest(raw))
  return actual


 @property
 def h(self):return self.host.harness
 @property
 def m(self):return self.host.grant["suite_manifest"]
 @property
 def oracle(self):return self.host.oracle
 def forecast(self,case_id):return copy.deepcopy(self.host.grant["case_profiles"][case_id]["forecast"])
 def profile(self,case_id):return copy.deepcopy(self.host.grant["case_profiles"][case_id])
 def write(self,root,name,raw):
  need(type(raw) is bytes and "disposable" in Path(root).parts,"owned-fixture-byte-mutation")
  self.case_for(root);path=confined(Path(root),Path(root)/name)
  category=self.host.ledger.root_name(path);missing=[];folder=path.parent
  while not folder.exists():missing.append(folder);folder=folder.parent
  token="fixture-write:"+uuid.uuid4().hex
  self.host.ledger.reserve(token,{category:dict(bytes=len(raw),
  files=0 if path.exists() else 1,directories=len(missing))})
  path.parent.mkdir(parents=True,exist_ok=True)
  with path.open("wb") as stream:
   for at in range(0,len(raw),65536):
    self.host.poll();stream.write(raw[at:at+65536])
   stream.flush()
  self.host.ledger.settle(token)
  return dict(path=str(path),bytes=len(raw),sha256=digest(raw))
 def preserve_move(self,root,name,suffix=".withheld"):
  self.case_for(root);path=confined(Path(root),Path(root)/name)
  target=path.with_name(path.name+suffix)
  need(path.is_file() and not target.exists(),"fresh-retained-fixture-move")
  path.rename(target)
 def alias(self,case_id,label,source=None):
  import shutil
  spec=self.host.grant["case_derivatives"][case_id][label]
  root=Path(spec["root"]);category=self.host.ledger.root_name(root)
  if source is not None:parent=self.case_for(source)[1]
  else:
   matches=[r for r in self.cases.values() if not r["closed"] and r["case_id"]==case_id and "derivative_of" not in r]
   need(len(matches)==1,"one-admitted-base-case");parent=matches[0]
  phase=copy.deepcopy(self.phases.get(parent["root"]))
  if case_id=="D-N64-N1k-AandB-payload-title":need(phase is not None,"D-parent-phase-required")
  if phase is not None:self.phases[str(root)]=phase
  need(not root.exists() and root.parent.is_dir() and "disposable" in root.parts,
  "fresh-source-mapped-case-derivative")
  facts=inventory(source,self.host.poll) if source is not None else dict(
  bytes=0,file_count=0,directory_count=0)
  need(facts["bytes"]<=spec["max_bytes"] and facts["file_count"]<=spec["max_files"],
  "complete-clone-before-write")
  token="fixture-clone:"+uuid.uuid4().hex
  self.host.ledger.reserve(token,{category:dict(bytes=facts["bytes"],
  files=facts["file_count"],directories=facts["directory_count"]+1)})
  if source is None:root.mkdir()
  else:
   shutil.copytree(source,root,symlinks=False)
   need(inventory(root,self.host.poll)==facts,"exact-byte-directory-clone")
  self.host.ledger.settle(token)
  key=case_id+"@"+label
  self.cases[key]=dict(root=str(root),closed=False,controls=[],handles=[],lease=token,
  case_id=parent["case_id"],scope_id=parent["scope_id"],case_scope=parent["case_scope"],
  opened_ns=parent["opened_ns"],derivative_of=parent["root"],derivatives=[])
  anchor=next(row for row in self.cases.values() if row["scope_id"]==parent["scope_id"]
  and row["case_scope"]==parent["case_scope"] and "derivative_of" not in row)
  anchor["derivatives"].append(key)
  self.host.harness.active.add(root)
  self.host.harness.ledgers[root]=copy.deepcopy(self.host.harness.ledgers[source]) if source is not None else type(
  next(iter(self.host.harness.ledgers.values())))()
  return root
 def close_case(self,case_id,root):
  _,case=self.case_for(root)
  failures=[]
  derivative_keys=case.get("derivatives",[])
  family=[case]+[self.cases[key] for key in derivative_keys]
  roots={row["root"] for row in family}
  controls=[key for row in family for key in row["controls"]]
  handles=[key for row in family for key in row["handles"]]
  for key in controls:
   try:
    self._write_signal(self.controls[key]["paths"],"release")
   except BaseException as error:
    failures.append(repr(error))
  try:
   for owned_root in roots:self.driver.close_owned("case-finally",owned_root)
  except BaseException as error:
   failures.append(repr(error))
  for key in handles:
   h=self.handles[key]
   try:
    if h["thread"].ident is not None:
     h["thread"].join(min(5,max(0,self.host.deadline.remaining())))
    h["joined"]=not h["thread"].is_alive()
    if not h["joined"]:failures.append("callback-not-joined:"+key)
   except BaseException as error:
    h["joined"]=False;failures.append("callback-join:"+key+":"+repr(error))
  rows=[s for s in self.driver.registry.values() if s["root"] in roots]
  closed=all(s["resources_closed"] and s["done"].is_set() for s in rows)
  callbacks=all(self.handles[k]["joined"] for k in handles)
  case["closed"]=closed and callbacks
  if case["closed"] and not failures:
   for key in controls:
    try:self._retire_control(key)
    except BaseException as error:failures.append("control-settlement:"+repr(error))
  for key in derivative_keys:
   self.cases[key]["closed"]=case["closed"]
   self.h.active.discard(Path(self.cases[key]["root"]))
  registry={k:dict(joined=self.handles[k]["joined"],native_tokens=self.handles[k]["native_tokens"])
  for k in handles}
  summary=dict(callbacks=len(registry),joined=sum(r["joined"] for r in registry.values()),
  native_tokens=sum(len(r["native_tokens"]) for r in registry.values()),
  sha256=digest(canonical(registry)))
  proof=dict(case_id=case["case_id"],root=str(root),processes_closed=closed,handles_closed=closed,
  python_callbacks_joined=callbacks,failures=failures,
  callback_registry=summary,
  outer_Job_empty="separate-parent-QPC-final-receipt")
  proof["scope_id"]=case["scope_id"];proof["case_scope"]=case["case_scope"]
  proof["closed_ns"]=time.perf_counter_ns()
  reference=self.host.history_ports.case_proof("case-close",case,proof)
  proof["receipt_sha256"]=reference["sha256"]
  proof["raw_ref"]=reference;case["closure_ref"]=reference
  self.host.case_closures[case["scope_id"]]=dict(case=copy.deepcopy(case),proof=copy.deepcopy(proof))
  return proof


 def emit(self,value):
  need(type(value) is dict and type(value.get("kind")) is str,"source-owned-case-note")
  return self.host.events.append(value)

 def backup_restore(self,case,label):
  from windows_hooks import NativeCase
  need(case.a is self and type(label) is str and 1<=len(label)<=100
  and all(x.isalnum() or x in "-_" for x in label),"owned-operational-backup-label")
  self.quiesce(case.root)
  source=inventory(case.root,self.host.poll)
  capture=self.alias(case.id,label+"-capture",case.root)
  captured=inventory(capture,self.host.poll)
  need(captured==source and inventory(case.root,self.host.poll)==source,
  "full-read-only-operational-backup")
  restored=self.alias(case.id,label,capture)
  actual=inventory(restored,self.host.poll)
  need(actual==source and inventory(capture,self.host.poll)==captured,
  "complete-byte-for-byte-operational-restore")
  self.emit(dict(kind="operational-backup-restored",case_id=case.id,root=str(case.root),
  capture=str(capture),restored=str(restored),files=source["file_count"],
  directories=source["directory_count"],bytes=source["bytes"],
  inventory_sha256=digest(canonical(source)),durable_pin_files=[
  k for k in source["files"] if k.startswith("physical-pins/")],
  directory_backup=True,no_excluded_store_files=True))
  return NativeCase(self.host,case.id,restored)



def verify_capture_bounds(host):
 refs=host.grant["guard_budget_refs"];bounds=host.manifest["capture"]["guard_bounds"]
 need(set(refs)==set(bounds)=={"inner","outer"},"closed-two-guard-budgets")
 for name,ref in refs.items():
  need(set(ref)=={"path","offset","bytes","sha256"} and ref["offset"]==0
  and integer(ref["bytes"],1) and ref["bytes"]<=131072,"whole-sealed-guard-budget")
  raw=regular_read(ref["path"],ref["bytes"])
  need(len(raw)==ref["bytes"] and digest(raw)==ref["sha256"],"same-buffer-guard-budget")
  actual=strict(raw,131072)["evidence_bounds"]
  need(actual==bounds[name],"exact-actual-guard-numeric-bounds")
 need(Path(refs["inner"]["path"])==Path(os.environ["FILE_NATIVE_BUDGET"])
 and refs["inner"]["sha256"]==host.owner["budget_sha256"],"current-inner-budget-owner")
 outer=strict(regular_read("C:/file-native-online-control-20261020/outer-owner.json",8192),8192)
 need(outer["state"]=="running" and outer["nonce"]==host.grant["outer_run_nonce"]
 and outer["guard_sha256"]==host.grant["outer_capture_sha256"]
 and outer["budget_sha256"]==refs["outer"]["sha256"]
 and outer["job_root_pid"]==host.owner["pid"]
 and outer["job_root_creation_time"]==host.owner["creation_time"],
 "actual-outer-owner-budget-nonce-and-guard-hierarchy")
 return copy.deepcopy(bounds)

from windows_driver import checker_native
