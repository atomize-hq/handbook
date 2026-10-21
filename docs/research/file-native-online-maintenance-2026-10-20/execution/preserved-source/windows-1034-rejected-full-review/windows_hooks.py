"""Private hooks and history ports."""
import ast
import copy
from contextlib import contextmanager
import importlib.util
import inspect
from pathlib import Path
import threading
import time
from windows_deadlines import Unsupported, canonical, digest, need, strict
from windows_growth import file_hash, regular_read, template_globals


class BoundHooks:
 def __init__(self,manifest,closure):
  self.specs=manifest["hooks"];self.final_pin=manifest["final_pin"]
  self.closure=closure;self.loaded={}
  self.paths={Path(p) for p in manifest.get("source_files",{})}

 def binding(self,name,final_pin):
  spec=self.specs.get(name)
  if not spec or spec.get("reviewed") is not True:
   raise Unsupported("PENDING-exact-reviewed-hook:"+name)
  need(final_pin==self.final_pin==spec["final_pin"]
  and spec.get("schema_pin") and spec.get("review_sha256"),
  "hook-schema-review-final-binding")
  path=Path(spec["path"]);self.closure()
  need(path in self.paths,"hook-in-full-transitive-source-closure")
  need(file_hash(path)==spec["sha256"],"hook-bytes")
  source=regular_read(path,2*1024**2).decode("utf-8")
  tree=ast.parse(source)
  found=[node for node in tree.body if isinstance(node,(ast.FunctionDef,ast.AsyncFunctionDef))
  and node.name==spec["function"]]
  need(len(found)==1 and not isinstance(found[0],ast.AsyncFunctionDef),"ordinary-hook")
  need(digest(ast.get_source_segment(source,found[0]).encode())==spec["function_sha256"],
  "hook-function-hash")
  return dict(status="reviewed-source-map",final_pin=final_pin,
  checkpoint=spec.get("checkpoint"),source_sha256=spec["sha256"],
  schema_pin=spec["schema_pin"],review_sha256=spec["review_sha256"],
  function_sha256=spec["function_sha256"])

 def historical(self,host,grant_id):
  from windows_driver import WindowsDriver
  key=("historical",grant_id)
  if key not in self.loaded:
   grant=host.manifest["historical_grants"].get(grant_id)
   if not grant:
    raise Unsupported("PENDING-closed-historical-binary-grant:"+grant_id)
   policy=HistoricalGrant(host,grant)
   d=WindowsDriver(host.native,grant["path"],grant["binary_sha256"],
   host.final_pin,host.events,policy.admit,host.deadline.end,
   observer=host.adapter.observe_start,historical_policy=policy)
   d.registry=host.driver.registry
   d.context=host.driver.context
   self.loaded[key]=d
  return self.loaded[key]

 def capabilities(self):
  return [name for name,s in self.specs.items() if s and s.get("reviewed") is True
  and s.get("final_pin")==self.final_pin]

 def call(self,name,host,request):
  self.binding(name,host.final_pin)
  spec=self.specs[name]
  need(spec.get("kind")=="in-process-source","hook-kind")
  key=(spec["path"],spec["sha256"])
  if key not in self.loaded:
   module_spec=importlib.util.spec_from_file_location("bound_hook_"+spec["sha256"],spec["path"])
   module=importlib.util.module_from_spec(module_spec)
   raw=regular_read(spec["path"],2*1024**2)
   need(digest(raw)==spec["sha256"],"hook-source-changed")
   exec(compile(raw,spec["path"],"exec",dont_inherit=True),module.__dict__)
   self.loaded[key]=module
  function=getattr(self.loaded[key],spec["function"])
  need(list(inspect.signature(function).parameters)==["host","request"],"hook-signature")
  result=function(host,copy.deepcopy(request))
  need(type(result) is dict and result.get("source_sha256")==spec["sha256"],
  "hook-receipt")
  result=dict(result);result.pop("source_sha256")
  raw=canonical(result)
  need(type(spec.get("result_max_bytes")) is int and 0<=len(raw)<=spec["result_max_bytes"],
  "hook-result-ceiling")
  host.events.append(dict(kind="bound-hook-result",name=name,
  source_sha256=spec["sha256"],request_sha256=digest(canonical(request)),
  result_sha256=digest(raw),result_bytes=len(raw),
  receipt_reference=result.get("receipt_sha256")))
  return result



class HistoryPorts:

 def __init__(self,host,bindings):
  self.host,self.b=host,copy.deepcopy(bindings)
  self.active={};self.index=[];self.lock=threading.RLock()
 def bindings(self):
  b=self.b
  return dict(api="history-ports-v1",platform="Windows",source_pin=b["history"]["source_pin"],
  source_sha256=b["ports_source"]["sha256"],review_sha256=b["ports_review"]["sha256"],
  guard_sha256=b["guard_source"]["sha256"],guard_schema_sha256=b["guard_schema"]["sha256"],
  clock_sha256=b["clock_source"]["sha256"],paced_sha256=b["paced_source"]["sha256"],
  capture_schema="single-raw-ref-v1",native_meta_schema="native-call-meta-v2",
  case_observations_scope="complete-case-through-closure")
 def _go(self):
  p=self.host.history_authorization
  need(p and p["go"] is True and p["final_pin"]==self.host.final_pin
  and len(p["receipt_sha256"])==64,"actual-history-authorization-before-native-ports")
  self.host.verify_closure()
 def build_binding(self):
  self._go()
  return self.host.hooks.call("build-binding-proof",self.host,dict(engine=self.host.manifest["engine"]))
 def verify_case_catalog(self,catalog):
  from case_ledger import required_cases
  self._go();expected=required_cases("Windows")
  need(set(catalog["ids"])==set(expected) and len(expected)==166,"all-actual-Windows-cases")
  path=catalog["source"]["path"]
  need(self.host.source_hash(path)==catalog["source"]["sha256"],"actual-case-catalog-source")
  need(set(self.host.manifest["cases"])==set(expected),"all166-native-case-maps")
  for case,spec in self.host.manifest["cases"].items():
   self.host.hooks.binding("case:"+case,self.host.final_pin)
   need(spec.get("constructor_reviewed") is True,"actual-reviewed-case-constructor:"+case)
  return True
 def authorize_namespace(self,phase,profile):
  self._go()
  spec=self.host.grant["history_namespaces"][phase["id"]]
  path=Path(spec["path"]);category=self.host.ledger.root_name(path)
  need(not path.exists() and path.parent.is_dir() and spec["root"]==phase["root"],
  "fresh-owned-paced-namespace")
  self.host.ledger.reserve("pace-dir:"+phase["id"],
  {category:dict(bytes=spec["max_bytes"],files=spec["max_files"],directories=1)})
  path.mkdir()
  ref=self.host.record("pace-namespace-"+phase["id"],dict(path=str(path),phase=phase),
  category="evidence",max_bytes=8192)
  row=dict(path=str(path),canonical_root=str(Path(phase["root"]).resolve()),
  category=category,receipt_ref=ref)
  self.active[phase["id"]]=dict(phase=copy.deepcopy(phase),profile=copy.deepcopy(profile),
  namespace=row,origin=None,entered=False)
  return copy.deepcopy(row)
 def read_owned(self,path,maximum):
  self._go();path=Path(path);self.host.ledger.root_name(path)
  return regular_read(path,maximum) if path.exists() else None
 def write_once(self,path,raw,category):
  self._go()
  return self._range(self.host.record(path,raw,category,8192))
 def read_reference(self,ref):
  self._go()
  from windows_oracle_bridge import read_ref
  self.host.ledger.root_name(ref["path"])
  return read_ref(ref,64*1024**2)
 def verify_ready(self,marker,start,namespace):
  self._go();native=self.host.native
  with native.lock:
   state=self.host.driver.registry[start["token"]]
   identity=native.identity(state["native"])
   need(identity["pid"]==marker["pid"]==start["pid"]
   and str(identity["creation_time"])==start["creation_id"]
   and start["image_sha256"]==self.host.driver.binary_sha256
   and Path(namespace["canonical_root"])==Path(state["root"]).resolve(),
   "actual-retained-native-ready-identity")
  ref=self.host.record("ready-proof",dict(marker=marker,start=start),
  category="evidence",max_bytes=16384)
  return dict(same_opened_native_identity=True,receipt_sha256=ref["sha256"])
 def clock_sample(self):
  from windows_native import counter,frequency
  self._go();before=time.perf_counter_ns();tick=counter();after=time.perf_counter_ns()
  body=dict(kind="QueryPerformanceCounter",domain="Windows-QPC",frequency=frequency(),
  ticks=tick,perf_before_ns=before,perf_after_ns=after)
  body["receipt_ref"]=self.host.record("clock-bracket",body,category="evidence",max_bytes=8192)
  return body
 def enter_phase(self,phase,profile,capacity):
  self._go();p=self.active[phase["id"]]
  need(not p["entered"] and p["phase"]==phase,"one-owned-history-phase")
  scope_id,scope=self.host.adapter.case_for(phase["root"])
  p.update(entered=True,capacity=copy.deepcopy(capacity),scope_id=scope["scope_id"],case_scope=scope["case_scope"])
  self.host.adapter.phases[str(phase["root"])]=dict(label=phase["id"])
 def leave_phase(self,phase_id):
  self._go();p=self.active[phase_id]
  need(p["entered"],"entered-phase");p["entered"]=False
  self.host.driver.phase_limits.pop(phase_id,None)
  self.host.adapter.phases.pop(str(p["phase"]["root"]),None)
 def set_origin(self,phase_id,body,capacity):
  self._go();p=self.active[phase_id]
  need(p["origin"] is None and p["capacity"]==capacity,"one-fixed-no-reset-origin")
  p["origin"]=copy.deepcopy(body)
  with self.host.driver.lock:
   self.host.driver.phase_limits[phase_id]=dict(origin_perf_ns=body["origin_perf_ns"],
   publisher_deadline_seconds=44,reader_deadline_seconds=48,old_pin_deadline_seconds=23.6)
 def observe_started(self,phase_id,reader,batch,event):
  self._go();state=self.host.driver.registry[event["call_token"]]
  with self.host.driver.lock:
   identity=self.host.native.identity(state["native"])
   need(identity["pid"]==event["pid"] and identity["creation_time"]==event["creation_time"]
   and event["positive_rss"]>0 and event["before_stdin_delivery"],"native-reader-before-input")
   ref=copy.deepcopy(state["start_reference"])
  return dict(token=event["call_token"],pid=event["pid"],creation_id=str(event["creation_time"]),
  image_sha256=event["executable_sha256"],rss=event["positive_rss"],contained=True,raw_ref=ref)
 def _observation(self,state,row,response):
  identity=row["native_identity"]
  c=self.host.events.calls[state["token"]];scope=state["context"]
  need(row["resources_closed"] and row["raw_retained"] and not row["violations"],"uncensored-owned-call")
  return dict(token=state["token"],input=copy.deepcopy(row["input"]),env=copy.deepcopy(row["env"]),
  meta_ref=copy.deepcopy(c["meta_ref"]),case_id=scope["case_id"],scope_id=scope["scope_id"],
  case_scope=scope["case_scope"],start_observed=c["start"] is not None and c["start_ref"] is not None,
  input_complete=(row["input_fully_delivered"] and row["input_bytes_sent"]==row["input_bytes_expected"]
  and row["input_written_prefix_sha256"]==row["input_wire_sha256"]),
  exit=row["exit"],violations=copy.deepcopy(row["violations"]),
  binary_sha256=row["executable_sha256"],source_pin=row["source_pin"],
  identity=dict(pid=identity["pid"],creation_id=str(identity["creation_time"]),
  image_sha256=row["executable_sha256"],contained=identity["same_job"]),
  start_ns=row["process_started_ns"],end_ns=row["process_done_ns"],spawn_ns=row["spawn_total_ns"],
  value=response,raw_ref=copy.deepcopy(state["raw_reference"]),
  stdout_eof=row["stdout_eof"],stderr_eof=row["stderr_eof"],process_reaped=row["popen_reaped"],
  io_closed=row["resources_closed"],native_closed=row["resources_closed"],
  callbacks_joined=row["python_callback_joined"],peak_rss=row["observed_peak_rss"])
 def state_reference(self,actual):
  self._go()
  return self.host.events.state_reference(actual)
 def current_call_observation(self):
  self._go();d=self.host.driver;token=d.context.completed_token
  return self._observation(d.registry[token],d.context.completed_row,d.context.completed_response)
 def case_call_pairs(self,phase_id):
  self._go();p=self._case(phase_id);result=[]
  for token,s in list(self.host.driver.registry.items()):
   if s["context"].get("case_scope")!=p["case_scope"]:continue
   need(s["done"].is_set() and s["resources_closed"],"case-index-all-owned-calls-closed")
   c=self.host.events.calls[token]
   need(c["meta_ref"] is not None,"case-index-authenticated-metadata")
   result.append(dict(raw_ref=s["raw_reference"],meta_ref=c["meta_ref"]))
  result.sort(key=lambda x:x["raw_ref"]["begin"])
  return result

 def phase_call_observations(self,phase_id):
  self._go();p=self.active[phase_id];rows=[]
  from windows_oracle_bridge import extract_call
  for pair in self.case_call_pairs(phase_id):
   token=pair["raw_ref"]["call"];state=self.host.driver.registry[token]
   if state["action"]=="script":continue
   v=extract_call(pair["raw_ref"],pair["meta_ref"],max_span=self.host.grant["max_capture_span"],
   max_raw_bytes=16*1024**2,source_pin=self.host.final_pin,
   binary_sha256=self.host.driver.binary_sha256,expected_exit=state["row"]["exit"])
   b,n,e,m=v["begin"],v["start"],v["end"],v["meta"]
   row=dict(input=strict(v["planned_input"],4*1024**2),env=b["env"],exit=e["exit"],
   executable_sha256=n["executable_sha256"],source_pin=self.host.final_pin,violations=e["violations"],
   native_identity=dict(pid=n["pid"],creation_time=n["creation"],
   same_job=state["public_identity"]["same_job"]),
   input_fully_delivered=e["input_complete"],input_bytes_sent=e["in_n"],
   input_bytes_expected=b["input_n"],input_written_prefix_sha256=e["in_sha"],
   input_wire_sha256=b["input_sha"],
   process_started_ns=m["process_started_ns"],process_done_ns=m["process_done_ns"],
   spawn_total_ns=m["spawn_total_ns"],stdout_eof=e["out_eof"],stderr_eof=e["err_eof"],
   popen_reaped=e["native_closed"],resources_closed=e["native_closed"] and e["io_closed"],
   raw_retained=True,python_callback_joined=e["callbacks_joined"],observed_peak_rss=e["peak"])
   rows.append(self._observation(state,row,v["response"]))
  return rows
 def abort_phase(self,phase_id,reason):
  self._go();self.host.driver.close_owned("history-abort:"+reason,self.active[phase_id]["phase"]["root"])

 def guard_prefix(self,phase_id,start_ns,end_ns):
  import zlib,os,hashlib
  self._go();case=self._case(phase_id);source=self.b["guard_source"]
  self.host.source_hash(source["path"])
  schema=self.b["guard_schema"];self.host.source_hash(schema["path"])
  schema_raw=regular_read(schema["path"],73728)
  need(digest(schema_raw)==schema["sha256"],"same-buffer-guard-wire-source")
  tree=ast.parse(schema_raw.decode("utf-8"))
  nodes=[n for n in tree.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name)
  and t.id=="GUARD_WIRE_CONTRACT" for t in n.targets)]
  need(len(nodes)==1,"actual-source-guard-literal")
  contract=ast.literal_eval(nodes[0].value)
  need(contract["api"]=="bounded-windows-guard-wire-r3","published-guard-wire-api")
  literal_sha=digest(canonical(contract))
  bound_ref=self.host.grant["guard_bounds_ref"]
  need(bound_ref["bytes"]<=131072 and bound_ref.get("offset",0)==0
  and bound_ref["sha256"]==self.host.owner["budget_sha256"]
  and Path(bound_ref["path"])==Path(os.environ["FILE_NATIVE_BUDGET"]),
  "actual-original-guard-budget-reference")
  budget_raw=self.read_reference(bound_ref)
  need(digest(budget_raw)==self.host.owner["budget_sha256"],"same-buffer-original-guard-budget")
  bounds=strict(budget_raw,131072)["evidence_bounds"]
  path=Path(self.host.owner["receipt"]).with_suffix(".rss.jsonl.gz")
  maximum=bounds["rss"]["gzip_bytes"]
  row_cap=bounds["rss"]["row_bytes"];plain_cap=bounds["rss"]["raw_bytes"]
  need(type(row_cap) is int and 1<=row_cap<=262144 and type(plain_cap) is int
  and plain_cap>0,"source-bounded-guard-prefix")
  self.host.ledger.root_name(path);compressed=hashlib.sha256();rolling=hashlib.sha256()
  decoder=zlib.decompressobj(31);buffer=bytearray();expanded=0;observed=None
  samples=0;first=None;last=None;last_ns=None;previous=None;recorded=None
  def lines(piece):
   nonlocal expanded,observed,samples,first,last,last_ns,previous,recorded
   expanded+=len(piece);need(expanded<=plain_cap,"guard-prefix-expanded-bound")
   buffer.extend(piece)
   while b"\n" in buffer:
    at=buffer.index(10)+1;line=bytes(buffer[:at]);del buffer[:at]
    need(len(line)<=row_cap,"guard-source-row-bound")
    row=strict(line,row_cap);rolling.update(line)
    need(row["run_nonce"]==self.host.owner["nonce"],"actual-outer-run-prefix")
    event=row.get("event")
    if event=="rss-sample":
     need(set(row)==set(contract["rows"]["rss-sample"]["required"])
     and row["sample_complete"] is True,"actual-complete-guard-sample")
     now=row["completed_perf_counter_ns"]
     need(type(now) is int and now>=row["perf_counter_ns"]>0
     and (previous is None or 0<=now-previous<=40000000),"actual-guard-cadence")
     previous=now
     if start_ns<=now<=end_ns:
      samples+=1;first=first or row;last=row;last_ns=now;observed=rolling.hexdigest()
    elif event=="rss-write-complete":
     need(set(row)==set(contract["rows"][event]["required"]),"actual-write-completion-row")
     recorded=row["recorded_perf_counter_ns"]
    elif event in ("cadence-gap-before-rss","full-sample-cadence-late","stale-sample-before-resume"):
     raise Unsupported("actual-guard-prefix-censored:"+event)
   need(len(buffer)<=row_cap,"guard-partial-row-bound")
  with path.open("rb") as stream:
   info=os.fstat(stream.fileno());n=info.st_size
   need(n<=maximum,"bounded-actual-compressed-prefix")
   remaining=n
   while remaining:
    chunk=stream.read(min(65536,remaining));need(chunk,"complete-retained-prefix")
    remaining-=len(chunk);compressed.update(chunk);pending=chunk
    while pending:
     piece=decoder.decompress(pending,65536);pending=decoder.unconsumed_tail
     lines(piece)
    need(not decoder.unused_data,"one-original-guard-gzip-member")
   after=os.fstat(stream.fileno())
   need((info.st_dev,info.st_ino)==(after.st_dev,after.st_ino)
   and after.st_size>=n,"same-handle-guard-prefix")
  need(samples and first["completed_perf_counter_ns"]-start_ns<=40000000
  and last_ns is not None and 0<=end_ns-last_ns<=40000000,
  "actual-completed-prefix-within40ms")
  prefix=dict(path=str(path),offset=0,bytes=n,sha256=compressed.hexdigest())
  proof=self.case_proof("guard-prefix",case,dict(prefix_ref=prefix,
  observed_prefix_sha256=observed,first=first,last=last,samples=samples,
  last_completed_rss_ns=last_ns,last_recorded_perf_ns=recorded,
  window_start_ns=start_ns,window_end_ns=end_ns,bounds_ref=bound_ref,
  guard_wire_sha256=literal_sha))
  case["guard_prefix_ref"]=proof
  return dict(status="PROVISIONAL_UNQUALIFIED",guard_sha256=source["sha256"],
  final_receipt=None,outer_run_id=self.host.owner["nonce"],host_pid=self.host.owner["job_root_pid"],
  host_creation_id=str(self.host.owner["job_root_creation_time"]),clock_domain="Windows-QPC",
  observed_prefix_sha256=observed,phase_start_ns=start_ns,phase_end_ns=end_ns,
  owned_drain_sha256=case.get("seal_context",{}).get("owned_drain_ref",{}).get("sha256"),
  finalizer_sha256=self.b["finalizer_source"]["sha256"],
  source_pin=self.host.final_pin,binary_sha256=self.host.driver.binary_sha256,
  budget_sha256=self.host.owner["budget_sha256"])

 def donor_authority(self,donor):
  self._go();spec=self.host.grant["history_donors"][donor["id"]]
  need(spec["root"]==donor["root"],"exact-owned-donor-root")
  for key in ("accepted_inputs","request_provenance","seed"):
   self.read_reference(spec[key])
  value=strict(self.read_reference(spec["accepted_inputs"]),16*1024**2)
  records=value["records"] if type(value) is dict else value
  need(type(records) is list and len(records)==donor["actual_H"],"donor-records")
  return dict(records=records,accepted_inputs=spec["accepted_inputs"],
  request_provenance=spec["request_provenance"],seed=spec["seed"],chain_refs=spec["chain_refs"])
 def seal_context(self,phase_id):
  self._go();case=self._case(phase_id)
  need(case["closed"] and case.get("closure_ref"),"actual-closed-case-before-seal")
  if "seal_context" in case:return copy.deepcopy(case["seal_context"])
  checkpoint=self.host.checkpoint("history-seal:"+phase_id,{})
  checkpoint_ref=self.case_proof("checkpoint",case,checkpoint)
  drain=self.host.drain("history-seal:"+phase_id)
  need(drain["children_empty"] and not drain["failures"],"actual-owned-drain-before-seal")
  drain_ref=self.case_proof("owned-drain",case,drain)
  value=dict(case_closed=True,case_id=case["case_id"],scope_id=case["scope_id"],
  case_scope=case["case_scope"],root=case["root"],run=self.host.events.run,
  case_closure_ref=case["closure_ref"],checkpoint_ref=checkpoint_ref,owned_drain_ref=drain_ref,
  outer_run_id=self.host.owner["nonce"],host_pid=self.host.owner["job_root_pid"],
  host_creation_id=str(self.host.owner["job_root_creation_time"]),clock_domain="Windows-QPC")
  case["seal_context"]=copy.deepcopy(value)
  return value
 def append_phase_index(self,index):
  self._go()
  need(set(index)=={"id","body_ref"} and index["id"] not in [x["id"] for x in self.index],
  "one-owner-phase-index-entry")
  self.read_reference(index["body_ref"])
  raw=canonical(index)+b"\n";need(len(raw)<=8192,"bounded-external-phase-index")
  ref=self._range(self.host.record("history-phase-index-"+index["id"],raw,
  category=self.host.grant["events_category"],max_bytes=len(raw)))
  self.index.append(copy.deepcopy(index));return ref

 def _case(self,helperlabel):
  case=self.host.adapter.cases.get(helperlabel)
  need(case is not None and case["scope_id"]==helperlabel,"actual-helper-case-context")
  return case
 def case_proof(self,kind,case,payload):
  need(kind in ("case-close","owned-drain","checkpoint","oracle","guard-prefix"),"closed-case-proof-kind")
  e=self.host.events;e._ensure_bridge()
  end=time.perf_counter_ns()
  body=dict(schema="native-case-proof-v1",kind=kind,run=e.run,
  source_pin=self.host.final_pin,binary_sha256=self.host.driver.binary_sha256,
  budget_sha256=self.host.owner["budget_sha256"],
  profile_catalogue_sha256=self.host.grant["profile_sha256"],
  case_id=case["case_id"],scope_id=case["scope_id"],case_scope=case["case_scope"],
  root=case["root"],start_ns=case["opened_ns"],end_ns=end,
  clock_bridge_ref=copy.deepcopy(e.bridge_ref),payload=copy.deepcopy(payload))
  spec=PROOF_PAYLOAD_CONTRACT["kinds"][kind]
  need(len(canonical(payload))<=spec["max_bytes"],"source-kind-case-proof-bound")
  if "fields" in spec:need(set(payload)==set(spec["fields"]),"source-owned-proof-payload-fields")
  if "failures" in payload:
   need(len(payload["failures"])<=16 and len(canonical(payload["failures"]))<=131072,
   "bounded-failure-proof-tail")
  return e._write(body)
 def _range(self,actual):
  self.read_reference(actual)
  need(set(actual) in ({"path","bytes","sha256","source_pin"},{"path","offset","bytes","sha256"}),
  "actual-whole-file-or-range-reference")
  return dict(path=actual["path"],offset=actual.get("offset",0),bytes=actual["bytes"],sha256=actual["sha256"])
 def seal_case_index(self,helperlabel,ready_refs,origin_ref):
  self._go();case=self._case(helperlabel)
  need(case["closed"] and case.get("closure_ref"),"case-index-after-actual-case-close")
  context=self.seal_context(helperlabel);pairs=self.case_call_pairs(helperlabel)
  key=case["case_scope"];fingerprint=digest(canonical(pairs))
  old=getattr(self,"sealed_cases",{}).get(key)
  if old is not None:
   need(old["calls_sha256"]==fingerprint and old["ready_refs"]==ready_refs
   and old["origin_ref"]==origin_ref,"immutable-complete-case-index")
   return copy.deepcopy(old["ref"])
  need(len(pairs)<=100000 and len(ready_refs)<=4,"case-index-call-page-caps")
  tags={k:case[k] for k in ("case_id","scope_id","case_scope")};e=self.host.events;pages=[]
  for at in range(0,len(pairs),128):
   pages.append(e._write(dict(schema="native-case-index-page-v1",run=e.run,**tags,
   ordinal=len(pages),first_call=at,calls=pairs[at:at+128])))
  body=dict(schema="native-case-index-v1",run=e.run,source_pin=self.host.final_pin,
  binary_sha256=self.host.driver.binary_sha256,budget_sha256=self.host.owner["budget_sha256"],
  profile_catalogue_sha256=self.host.grant["profile_sha256"],**tags,root=case["root"],
  start_ns=case["opened_ns"],end_ns=time.perf_counter_ns(),total_calls=len(pairs),pages=pages,
  ready_refs=[self._range(r) for r in ready_refs],
  origin_ref=self._range(origin_ref) if origin_ref is not None else None,
  case_closure_ref=context["case_closure_ref"],owned_drain_ref=context["owned_drain_ref"],
  checkpoint_ref=context["checkpoint_ref"])
  ref=e._write(body)
  if not hasattr(self,"sealed_cases"):self.sealed_cases={}
  self.sealed_cases[key]=dict(calls_sha256=fingerprint,ready_refs=copy.deepcopy(ready_refs),
  origin_ref=copy.deepcopy(origin_ref),ref=ref)
  return copy.deepcopy(ref)


def HistoryNative(host):
 import importlib
 b=host.grant["runtime_bindings"]
 module=importlib.import_module("runtime_contracts")
 need(host.source_hash(module.__file__)==b["runtime_source"]["sha256"],"actual-runtime-source")
 host.history_ports=HistoryPorts(host,b)
 return module.make_runner(host,b)


class NativeCase:

 def __init__(self,host,case_id,root):
  host=getattr(host,"host",host)
  self.host,self.id,self.root=host,case_id,Path(root)
  self.h=host.harness;self.a=host.adapter
 def init(self,mode="A",n=16):
  self.h.initialize(self.root,mode,n);return self
 def selected(self):return self.h.selected(self.root)
 def read(self,name,limit=131072):return self.h.read(self.root,name,limit)
 def document(self,name,limit=131072):return strict(self.read(name,limit),limit)
 def write(self,name,raw):return self.a.write(self.root,name,raw)
 def publish(self,name,ops=None,**fields):return self.h.publish(self.root,name,ops,**fields)
 def pack(self,codec="raw",bootstrap=False,**fields):
  return self.h.pack(self.root,codec,bootstrap=bootstrap,**fields)
 def drain(self):return self.h.drain_two_passes(self.root)
 def verify(self,name="boundary"):return self.h.verify(self.root,self.id+"-"+name)
 def clone(self,label):
  return NativeCase(self.host,self.id,self.a.alias(self.id,label,self.root))
 def call(self,action,**fields):
  request=fields.get("request");ledger=self.h.ledgers[self.root]
  if request is not None and action in ("prepare","publish"):ledger.record(request)
  if action=="checkpoint":
   request=dict(request=fields["request_id"],base=None,ops=[
   dict(kind="checkpoint",id="",text="",target="",value=1)],packet=False)
   ledger.record(request)
  value=self.h.call(self.root,action,**fields)
  if value and value.get("status")=="ok" and action in ("publish","checkpoint","select"):
   result=value.get("result") or {};accepted=result.get("result",result)
   if type(accepted) is dict and accepted.get("outcome")=="accepted":
    if action=="select":request=self.document(fields["candidate"],4*1024**2)["request"]
    need(request is not None,"captured-exact-accepted-input")
    ledger.accept(request,accepted,self.selected())
  return value
 def lost(self,name,phase,ops=None):
  request=dict(request=name,base=self.selected(),ops=ops or [],packet=False)
  self.h.ledgers[self.root].record(request)
  self.h.call(self.root,"publish",request=request,code=86,env=dict(EXP_EXIT=phase))
  result=self.h.call(self.root,"resolve",request_id=name)["result"]
  if result.get("outcome")=="accepted":
   self.h.ledgers[self.root].accept(request,result,self.selected())
  return result


class CaseTemplates:

 def __init__(self,host):
  self.host=host;self.loaded={}
 def load(self,spec,extra):
  path=Path(spec["path"]);need(self.host.source_hash(path)==spec["sha256"],"template-source")
  self.host.verify_closure();raw=regular_read(path,131072)
  need(digest(raw)==spec["sha256"],"template-bytes")
  source=raw.decode("utf-8");tree=ast.parse(source);functions=[n for n in tree.body if isinstance(n,ast.FunctionDef)]
  selected=next((n for n in functions if n.name==spec["function"]),None)
  need(selected is not None and digest(ast.get_source_segment(source,selected).encode())
  ==spec["function_sha256"] and spec["reviewed"] is True,"reviewed-template")
  functions=self._functions(functions,selected.name,path.name)
  key=spec["sha256"]
  if key not in self.loaded:
   import json,hashlib,struct,os,uuid
   namespace=dict(need=need,strict=strict,encoded=canonical,
   digest=lambda p:file_hash(p),copy=copy,json=json,hashlib=hashlib,
   struct=struct,os=os,uuid=uuid,time=time,Path=Path,
   Case=lambda a,i,r:NativeCase(self.host,i,r))
   namespace.update(extra)
   from types import SimpleNamespace
   from windows_capsule import workspace_case
   from windows_native import historical_phase
   from windows_oracle_bridge import ordinary,GuardedDecoder
   def leaf(*args,**kwargs):
    return self.load(self.host.grant["leaf_template"],namespace)(*args,**kwargs)
   def physical(root,quiescent=False):
    need(quiescent is True,"explicit-quiescent-physical-constructor")
    return GuardedDecoder(self.host.oracle,root,physical_only=True)
   namespace.update(__native_workspace=workspace_case,__native_ordinary=ordinary,
   __native_leaf=leaf,__native_v5=SimpleNamespace(Decoder=physical),
   historical_phase=historical_phase,ordinary=ordinary)
   if path.name=="suite_units.py":namespace["run"]=workspace_case

   for node in tree.body:
    if isinstance(node,ast.Assign) and all(isinstance(t,ast.Name) for t in node.targets):
     try:value=ast.literal_eval(node.value)
     except (ValueError,TypeError):
      if isinstance(node.value,ast.BinOp) and isinstance(node.value.op,ast.Add) and isinstance(
      node.value.left,ast.Name) and isinstance(node.value.right,ast.Constant):
       value=namespace[node.value.left.id]+node.value.right.value
      else:continue
     for target in node.targets:namespace[target.id]=value
   exec(compile(ast.fix_missing_locations(ast.Module(body=functions,type_ignores=[])),str(path),"exec"),namespace)
   self.loaded[key]=namespace
  return self.loaded[key][spec["function"]]

 def _functions(self,functions,selected,source_name):
  routes={("suite_units","run"):"__native_workspace",
  ("suite_ordinary","ordinary"):"__native_ordinary",
  ("suite_extensions","leaf"):"__native_leaf",
  ("native_resources_v3","canonical"):"canonical",
  ("native_resources_v3","strict"):"strict"}
  stdlib={"base64","copy","hashlib","json","math","os","pathlib","re","struct","time","uuid"}
  class Rewrite(ast.NodeTransformer):
   def visit_ImportFrom(self,node):
    if node.level or any(a.name=="*" for a in node.names):
     raise ValueError("relative-or-star-provider-import")
    if node.module in stdlib:return node
    assignments=[]
    for alias in node.names:
     target=routes.get((node.module,alias.name))
     if target is None:raise ValueError("unmapped-native-provider:"+str(node.module))
     assignments.append(ast.copy_location(ast.Assign(
     targets=[ast.Name(id=alias.asname or alias.name,ctx=ast.Store())],
     value=ast.Name(id=target,ctx=ast.Load())),node))
    return assignments
   def visit_Import(self,node):
    if all(a.name in stdlib for a in node.names):return node
    if len(node.names)==1 and node.names[0].name=="physical_decoder_v5":
     alias=node.names[0]
     return ast.copy_location(ast.Assign(
     targets=[ast.Name(id=alias.asname or alias.name,ctx=ast.Store())],
     value=ast.Name(id="__native_v5",ctx=ast.Load())),node)
    raise ValueError("unmapped-native-module-import")
  rewritten=[];bad=set()
  for function in functions:
   if function.name in ("historical_phase","ordinary"):continue
   if source_name=="suite_units.py" and function.name=="run":continue
   try:rewritten.append(Rewrite().visit(copy.deepcopy(function)))
   except ValueError:bad.add(function.name)
  for _ in functions:
   bad|={f.name for f in functions if any(isinstance(n,ast.Call) and isinstance(n.func,ast.Name)
   and n.func.id in bad for n in ast.walk(f))}
  need(selected not in bad,"complete-source-native-constructor-closure")
  return [f for f in rewritten if f.name not in bad]

 def dispatch(self,case_id,root):
  spec=self.host.manifest["cases"][case_id]
  need(spec["constructor_reviewed"] is True and spec["final_pin"]==self.host.final_pin,
  "exact-final-constructor-review")
  import importlib
  paged=importlib.import_module("paged_index");schedules=importlib.import_module("focused_schedules")
  self.host.source_hash(paged.__file__);self.host.source_hash(schedules.__file__)
  function=self.load(spec["template"],template_globals(self.host,dict(paged_index=paged,schedules=schedules)))
  args=list(inspect.signature(function).parameters)
  need(len(args) in (1,2),"source-constructor-actual-arity")
  case=NativeCase(self.host,case_id,root)
  return function(case) if len(args)==1 else function(case,copy.deepcopy(spec["constructor_arguments"]))

class HistoricalGrant:
 """Old source remains old; closed case/action/input/env/role/count permission."""
 def __init__(self,host,grant):
  self.host,self.grant=host,copy.deepcopy(grant)
  need(set(grant)=={"path","binary_sha256","binary_source_pin","review_sha256",
  "case_id","fresh_or_owned_derivative","calls"}
  and grant["fresh_or_owned_derivative"] is True
  and grant["binary_source_pin"]!=host.final_pin and grant["review_sha256"],
  "closed-distinct-historical-source-grant")
  need(file_hash(grant["path"])==grant["binary_sha256"],"actual-old-executable-bytes")
  self.lock=threading.RLock();self.remaining=[r["count"] for r in grant["calls"]]
  need(all(type(n) is int and n>0 for n in self.remaining),"finite-historical-call-counts")

 def admit(self,payload,env,code):
  actual=digest(canonical(payload))
  rows=[r for r in self.grant["calls"] if r["input_sha256"]==actual and r["env"]==env
  and r["expected_exit"]==code]
  need(len(rows)==1,"historical-exact-growth-profile")
  row=rows[0];token="historical-growth:"+actual+":"+str(time.perf_counter_ns())
  self.host.ledger.reserve(token,row["growth"]);self.host.ledger.settle(token)
  return row["profile"]

 def __call__(self,payload,env,code):
  root=Path(payload["root"]);self.host.ledger.root_name(root)
  need("disposable" in root.parts,"historical-owned-derivative-only")
  banned=[Path(p) for p in self.host.grant["protected_roots"]]
  need(not any(root==p or root.is_relative_to(p) for p in banned),
  "never-open-protected-old-operational-root")
  _,case=self.host.adapter.case_for(root)
  case_id=case["case_id"]
  need(case_id==self.grant["case_id"],"historical-exact-case")
  actual=dict(root=str(root),action=payload["action"],input_sha256=digest(canonical(payload)),
  env=env,expected_exit=code,role="reader" if payload["action"] in
  ("resolve","point","inspect","lookup-bench") else "checker")
  with self.lock:
   found=[i for i,row in enumerate(self.grant["calls"])
   if all(row.get(k)==v for k,v in actual.items()) and self.remaining[i]>0]
   need(len(found)==1,"historical-action-input-env-role-count-grant")
   i=found[0];self.remaining[i]-=1
   return dict(binary_source_pin=self.grant["binary_source_pin"],binary_role="historical",
   case_id=case_id,grant_sha256=digest(canonical(self.grant)),call_index=i,
   remaining=self.remaining[i],expected_error=self.grant["calls"][i]["expected_error"])


SIDEBAND_KINDS = ['LaneHost-drain', 'LaneHost-start', 'authentic-semantic-refusal', 'barrier-completed', 'bound-hook-result', 'bounded-concurrency-attempts', 'budget-checkpoint', 'call-failure', 'call-result', 'callback-cleanup', 'callback-native-association', 'callback-wait-result', 'cancel-ended-by-owned-rename', 'case-closure', 'case-failure', 'case-start', 'cleanup-cooperative-target-miss', 'cleanup-crash-continuation', 'cleanup-crash-reached', 'concurrency-native-intervals', 'control-signal', 'donor-inputs-adopted', 'fast-native-poll', 'final-all-maintained-lineages-two-pass-drain', 'history-all1600-provisional', 'history-initial-lookups-complete', 'history-plan-admitted', 'history-retained-terminal-conflicts', 'independent-full-oracle', 'independent-oracle', 'invocation-closure', 'lane-authorization', 'mapping-stress-sample', 'native-boundary-assertions', 'native-ready', 'observed-injected-process-exit', 'old-reader-resumed', 'operational-backup-restored', 'ordinary-all17', 'post-reader-container-reachability', 'reader-end-closed', 'reader-finally-failure', 'reader-native-end', 'request-before-dispatch', 'schedule-complete-unqualified', 'script-startup-gate-ready', 'selected-state', 'semantic-conflict', 'semantic-publication-nonaccept', 'single-publisher-live-overlap', 'single-publisher-overlap-joined', 'single-publisher-overlap-observations', 'stress-summary-unqualified', 'two-drainage-passes', 'workspace-all-actual-tests']

def adopt_capture_profile(catalogue,expected,group,profile):
 from windows_growth import authenticate_action_profile
 authenticate_action_profile(profile,catalogue,expected)
 need(profile["profile_type"]=="action","actual-call-profile-is-action")
 if group["profile_sha256"]==profile["profile_sha256"]:return
 need(group["profile_type"]=="group" and profile["profile_type"]=="action"
 and profile["group_id"]==group["profile_id"] and profile["profile_id"] in group["members"],
 "authenticated-group-member")
 for key,value in profile.items():
  if type(value) is int and key in group:need(value<=group[key],"member-numeric-bound")
  if key=="all_sink_components":
   need(set(value)==set(group[key]) and all(v<=group[key][k] for k,v in value.items()),
   "member-components-bound")

PROOF_PAYLOAD_CONTRACT = {'api': 'windows-proof-payload-r3', 'limits': {'root_count': 64, 'root_name_ascii': 128, 'label_ascii': 256, 'counter_bits': 64, 'failures': 16, 'failure_json_bytes': 131072, 'guard_row_bytes': 262144}, 'kinds': {'case-close': {'max_bytes': 135168, 'fields': ['case_id', 'root', 'processes_closed', 'handles_closed', 'python_callbacks_joined', 'failures', 'callback_registry', 'outer_Job_empty', 'scope_id', 'case_scope', 'closed_ns'], 'registry_fields': ['callbacks', 'joined', 'native_tokens', 'sha256'], 'provenance': 'actual case family process/resources/done, joined callbacks; registry hash includes every handle and native token; physical associations remain on tape'}, 'owned-drain': {'max_bytes': 163840, 'fields': ['label', 'children_empty', 'processes_closed', 'handles_closed', 'callbacks_joined', 'scope', 'failures', 'owned_members_polls', 'census', 'outer_QPC_two_empty_Job_polls', 'closure', 'raw_ref', 'receipt_sha256'], 'poll_fields': ['observed_ns', 'members', 'owned_children'], 'census_fields': ['bytes', 'files', 'directories', 'inventory_sha256'], 'provenance': 'actual resources/done + callback checks; exactly two full QueryInformationJobObject tables; all category inventories'}, 'checkpoint': {'max_bytes': 32768, 'fields': ['label', 'actual', 'prospective', 'lease_count', 'lease_set_sha256', 'planned_reclaim_credit', 'go', 'no_other_semantic_writer', 'claim_id', 'forecast_sha256', 'receipt_sha256', 'raw_ref'], 'actual_fields': ['bytes', 'files', 'directories', 'inventory_sha256'], 'prospective_fields': ['bytes', 'files', 'directories'], 'provenance': 'complete in-memory inventory/lease validation, exact per-category projections and full-set hashes; source owner grant for no_other_semantic_writer'}, 'guard-prefix': {'max_bytes': 540672, 'fields': ['prefix_ref', 'observed_prefix_sha256', 'first', 'last', 'samples', 'last_completed_rss_ns', 'last_recorded_perf_ns', 'window_start_ns', 'window_end_ns', 'bounds_ref', 'guard_wire_sha256'], 'provenance': 'same-handle actual compressed prefix; two source-schema complete rows <=262144 each; digest of exact expanded physical lines through last sample; no final Job verdict'}, 'oracle': {'max_bytes': 4194304, 'active_producer': False, 'provenance': 'No case_proof oracle producer currently; OracleBridge emits independent-oracle sideband separately. This reserved API kind is not a successful call allocation.'}}}


def _hook_result(value):
 return dict(value,source_sha256=file_hash(__file__))

def ready_proof(host,request):
 a=host.adapter;handle=a._handle(a.handles[request["handle_id"]])
 a._control(request["control"])
 raw=regular_read(request["control"]["ready"],8192)
 need(digest(raw)==request["raw_sha256"],"same-actual-ready-bytes")
 live=[host.driver.registry[t] for t in request["tokens"] if host.driver.live(t)]
 need(len(live)==1,"one-actual-ready-process");state=live[0];token=state["token"]
 c=host.events.calls[token];env=c["env"] or {};phase=env.get("EXP_BARRIER")
 need(type(phase) is str and phase in host.manifest["engine"]["checkpoint_names"]
 and raw==phase.encode("ascii") and env.get("EXP_READY")==request["control"]["ready"]
 and env.get("EXP_RELEASE")==request["control"]["release"]
 and state["root"]==request["root"]==handle["root"]
 and state["context"].get("handle_id")==handle["id"],"native-source-checkpoint-correspondence")
 identity=host.native.identity(state["native"])
 ref=dict(path=request["control"]["ready"],offset=0,bytes=len(raw),sha256=digest(raw))
 proof=dict(schema="native-checkpoint-proof-v1",call_token=token,root=state["root"],phase=phase,
 pid=identity["pid"],creation_time=identity["creation_time"],identity_verified=identity["handle_identity_verified"],
 same_job=identity["same_job"],ready_ref=ref,observed_ns=time.perf_counter_ns())
 state["checkpoint_proof"]=copy.deepcopy(proof)
 if state["action"]=="packing" and state["context"].get("case_id")=="S-maintainer-death":
  need(phase in ("physical-before-selection","physical-after-selection"),"closed-maintainer-death-checkpoint")
  state["maintainer_death_grant"]=dict(root=state["root"],input_sha256=digest(canonical(strict(c["encoded"],4194304))),
  phase=phase,case_scope=state["context"]["case_scope"],
  pid=identity["pid"],creation_time=identity["creation_time"],ready_ref=ref)
 return _hook_result(proof)

def interruption_proof(host,request):
 row=request["row"];state=host.driver.registry[row["call_token"]]
 need(state["root"]==request["root"] and state["done"].is_set() and state["owner_finished"]
 and state["resources_closed"] and row["violations"]==[]
 and row["stdout_eof"] and row["stderr_eof"] and row["python_callback_joined"]
 and request["callback_joined"] is True and state["raw_reference"]==request["raw_reference"],
 "interruption-fence")
 return _hook_result(dict(schema="native-interruption-proof-v1",call_token=state["token"],
 kind=request["kind"],root=state["root"],raw_reference=state["raw_reference"],
 metadata_ref=host.events.calls[state["token"]]["meta_ref"],
 checkpoint_ref=(state.get("checkpoint_proof") or {}).get("ready_ref"),
 processes_closed=state["resources_closed"],owner_finished=state["owner_finished"],
 callbacks_joined=row["python_callback_joined"]))

def quiescence_proof(host,request):
 states=[s for s in host.driver.registry.values() if s["root"]==request["root"] and not s["resources_closed"]]
 need({s["token"] for s in states}==set(request["calls"]),"actual-active-root-call-set")
 facts=[]
 for state in states:
  proof=state.get("checkpoint_proof")
  need(state["action"] in ("resolve","point","inspect","lookup-bench") and proof is not None,
  "only-source-paused-readers-during-quiescent-oracle")
  identity=host.native.identity(state["native"])
  raw=regular_read(proof["ready_ref"]["path"],8192)
  need(digest(raw)==proof["ready_ref"]["sha256"] and identity["pid"]==proof["pid"]
  and identity["creation_time"]==proof["creation_time"],"actual-paused-reader-pin")
  facts.append(dict(token=state["token"],identity=identity,checkpoint=proof))
 return _hook_result(dict(only_authentic_paused_readers=all(s["action"] in
 ("resolve","point","inspect","lookup-bench") for s in states),
 no_active_writer=not any(s["action"] not in ("resolve","point","inspect","lookup-bench") for s in states),
 actual_paused_readers=facts))

def removal_proof(host,request):
 before,after=request["before"],request["after"]
 removed=sorted(name for name in before["files"] if name not in after["files"])
 need(removed==sorted(request["removed_paths"]) and all(
 not (Path(request["root"])/name).exists() for name in removed),"actual-removed-pathnames")
 return _hook_result(dict(root=request["root"],phase=request["phase"],removed_paths=removed,
 removed_files=len(removed),removed_bytes=sum(before["files"][p]["bytes"] for p in removed),
 before_sha256=digest(canonical(before)),after_sha256=digest(canonical(after)),
 planned_deletion_credit=0))

def _actual_payload(host,state):
 c=host.events.calls[state["token"]]
 if c["encoded"] is not None:return strict(c["encoded"],4194304)
 from windows_oracle_bridge import extract_call
 value=extract_call(state["raw_reference"],c["meta_ref"],
 max_span=host.grant["max_capture_span"],max_raw_bytes=20*1024**2,
 source_pin=host.final_pin,binary_sha256=state["row"]["executable_sha256"],
 expected_exit=state["row"]["exit"],parse_stdout=False,
 expected_intent=state["row"]["intentional_interrupt"])
 return strict(value["planned_input"],4194304)

def overlap_proof(host,request):
 states=[s for s in host.driver.registry.values() if s["root"]==request["root"]
 and s["context"].get("phase")==request["label"] and s.get("row") is not None]
 rows=[]
 for s in states:
  r=s["row"];event=r.get("positive_start_event")
  need(event is not None and event["positive_rss"]>0 and r["resources_closed"]
  and not r["violations"],"actual-positive-overlap-call")
  rows.append(dict(token=s["token"],action=s["action"],pid=event["pid"],
  creation=event["creation_time"],start=event["resumed_ns"],end=r["process_done_ns"],
  payload=_actual_payload(host,s)))
 readers=[r for r in rows if r["action"] in ("resolve","point","inspect","lookup-bench")]
 writers=[r for r in rows if r["action"]=="publish"]
 maintainers=[r for r in rows if r["action"]=="packing"
 and r["payload"].get("operation") in ("build","repack","cleanup")]
 def intersects(a,b):return max(a["start"],b["start"])<min(a["end"],b["end"])
 reader_count=len({(r["pid"],r["creation"]) for r in readers})
 writer_count=len({(r["pid"],r["creation"]) for r in writers})
 overlap=bool(maintainers and writers and all(
 any(intersects(r,m) for m in maintainers) and any(intersects(r,w) for w in writers) for r in readers))
 need(reader_count>=request["readers"] and writer_count>=request["publishers"] and overlap,
 "actual-reader-publisher-maintenance-overlap")
 return _hook_result(dict(independent_reader_processes=reader_count,publisher_processes=writer_count,
 maintenance_overlap=overlap,calls=rows))

def prepared_candidates(host,request):
 root=Path(request["root"]);wanted=set(request["requests"]);found={}
 need(all(type(k) is str for k in wanted),"prepared-request-ID-list")
 for path in (root/"attempts").glob("*/candidate.json"):
  raw=regular_read(path,4194304);value=strict(raw,4194304);key=value["request"]["request"]
  if key in wanted:
   need(key not in found,"one-actual-prepared-candidate")
   found[key]=dict(path=path.relative_to(root).as_posix(),sha256=digest(raw),bytes_length=len(raw))
 need(set(found)==wanted,"all-actual-prepared-candidates")
 return _hook_result(dict(root=str(root),candidates=found))


def _catalogue_action(host,identifier):
 from windows_growth import action_profile_digest,authenticate_action_profile
 profile=copy.deepcopy(host.events.profiles["profiles"][identifier])
 need(profile["profile_type"]=="action","actual-action-profile-only")
 profile["profile_sha256"]=action_profile_digest(profile)
 profile["profile_catalogue_sha256"]=host.grant["profile_sha256"]
 authenticate_action_profile(profile,host.events.profiles,host.grant["profile_sha256"])
 return profile

def engine_input_policy(host,request):
 from windows_deadlines import READERS,integer,Refusal
 payload=request["input"];root=Path(payload["root"]);action=payload["action"]
 need(root.is_absolute() and root.resolve()==root and type(action) is str
 and len(canonical(payload))<=4194304 and len(canonical(request["env"]))<=8192,
 "bounded-source-owned-engine-input")
 host.ledger.root_name(root)
 need(not any(root==Path(p) or root.is_relative_to(Path(p)) for p in host.grant["protected_roots"]),
 "never-engine-open-protected-root")
 context=getattr(host.driver.context,"value",{})
 scope=context.get("scope_id")
 active=[c for c in host.adapter.cases.values() if not c["closed"]
 and c["scope_id"]==scope and c["case_scope"]==context.get("case_scope")]
 need(active,"actual-admitted-case-context")
 rule=host.grant["engine_input_rules"][scope][action]
 need(set(rule)=={"source_pin","roots","profile_id","max_calls","expected_exits","growth"}
 and rule["source_pin"]==host.final_pin and str(root) in rule["roots"]
 and integer(rule["max_calls"],1) and type(rule["expected_exits"]) is list
 and request["expected_exit"] in rule["expected_exits"]
 and all(type(n) is int and n in (0,2,86) for n in rule["expected_exits"]),
 "closed-parent-engine-rule-and-count")
 if getattr(host,"long_authorization",None) is not None and str(root) in (
 host.long_config["roots"]["source"],host.long_config["roots"]["destination"]):
  need(str(root) in getattr(host,"long_releases",{}),"long-root-still-quarantined")
 profile=selected_engine_profile(host,rule["profile_id"])
 need(profile["role"]==("reader" if action in READERS else "checker"),"source-action-RSS-role")
 key="engine-policy:"+scope+":"+action
 with host.record_lock:
  count=host.record_counts.get(key,0)
  need(count<rule["max_calls"],"finite-source-engine-call-count")
  host.record_counts[key]=count+1
 return _hook_result(dict(admitted=True,source_pin=host.final_pin,growth=copy.deepcopy(rule["growth"]),
 profile=profile,rule_sha256=digest(canonical(rule)),call_number=count+1))

def forecast_reserves(host,request):
 if request["label"]=="long:legacy-restore":return legacy_forecast(host,request)
 label=request["label"];rule=host.grant["forecast_rules"][label]
 need(set(rule)=={"reserves_sha256","growth"} and rule["reserves_sha256"]==digest(canonical(request["reserves"])),
 "exact-parent-forecast-reserves")
 growth=copy.deepcopy(rule["growth"])
 for category,row in growth.items():
  need(category in host.ledger.roots and set(row)=={"bytes","files","directories"}
  and all(type(n) is int and n>=0 for n in row.values()),"incremental-forecast-growth")
 return _hook_result(dict(growth=growth,incremental=True,planned_deletion_credit=0,
 existing_root_counted_once=True,label=label,reserves_sha256=rule["reserves_sha256"]))

CONTROL_PAYLOAD_CONTRACT={'api': 'source-control-payload-v1', 'hold': {'bytes': 24576, 'files': 3, 'directories': 0}, 'signal_bytes': 8, 'ready_encoding': 'ASCII checkpoint name without newline', 'barriers': {'candidate-ready': 15, 'physical-before-selection': 25, 'physical-after-selection': 24}, 'core_io_sha256': '1e401ad489b78e10845dac65ccc1b36ee9f99753c0870aa5a41c4da6661aa02c', 'settlement': 'only actual closed-writer-settlement-v1 after all case processes and callbacks close; retain all actual files; failure retains full hold', 'category': 'ledger.root_name(manifest.release.controls_root); no inferred category'}

def build_binding_proof(host,request):
 from windows_growth import read_ref
 engine=request["engine"];ref=host.grant["engine_build_binding"]
 value=strict(read_ref(ref,2097152),2097152)
 need(value["source_pin"]==engine["source_pin"]==host.final_pin
 and value["binary"]["path"]==engine["path"]
 and value["binary"]["sha256"]==engine["sha256"]
 and Path(engine["path"]).stat().st_size==value["binary"]["bytes"]
 and file_hash(engine["path"])==engine["sha256"],"built-image")
 before=strict(read_ref(value["source_before_ref"],2097152),2097152)
 after=strict(read_ref(value["source_after_ref"],2097152),2097152)
 need(before==after and before==engine["source_files"],"build-sources")
 for path,row in before.items():
  need(Path(path).stat().st_size==row["bytes"] and file_hash(path,row["bytes"])==row["sha256"],
  "built-sources")
 for ref in value["guard_receipts"]:
  receipt=strict(read_ref(ref,2097152),2097152)
  need(receipt["argv"]==engine["build_argv"] and receipt["cwd"]==engine["crate_root"]
  and receipt["exit"]==0 and receipt["censor"] is None
  and receipt["process_closure_verified"] is True and receipt["postwrite_census_complete"] is True
  and len(receipt["closure_checks"])==2 and all(not r["members"] for r in receipt["closure_checks"]),
  "build-closure")
 need(value["guard_receipts"],"build-receipts")
 return _hook_result(dict(final_pin=host.final_pin,binary_sha256=engine["sha256"],
 build_binding_ref=host.grant["engine_build_binding"],source_files_sha256=digest(canonical(before))))

def selected_engine_profile(host,rule_id):
    current=getattr(host,"invoke_profile",None)
    if current is None:return _catalogue_action(host,rule_id)
    actual=_catalogue_action(host,current["profile_id"])
    need(canonical(actual)==canonical(current),"actual-invoke-profile")
    parent=host.events.profiles["profiles"][rule_id]
    need(actual["profile_id"]==rule_id or parent["profile_type"]=="group"
        and actual["group_id"]==rule_id and actual["profile_id"] in parent["members"],"parent-profile-member")
    return actual


def legacy_forecast(host,request):
    from windows_growth import inventory
    c=host.long_config
    need(host.long_authorization and "long-archive" in host.ledger.leases,"legacy-restore-lease")
    actual=inventory(c["roots"]["source"],host.poll,absent=True)
    remaining={c["categories"]["source"]:dict(bytes=c["stream"]["max_destination_bytes"]-actual["bytes"],
        files=c["stream"]["max_files"]-actual["file_count"])}
    need(request["reserves"]==remaining and all(n>=0 for r in remaining.values() for n in r.values()),
        "actual-legacy-remaining")
    return _hook_result(dict(growth={},incremental=True,planned_deletion_credit=0,existing_root_counted_once=True))
