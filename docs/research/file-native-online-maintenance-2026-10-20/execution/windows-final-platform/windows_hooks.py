"""Reviewed source-bound hooks and history-native-v1. Pending maps refuse."""
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
             "hook-source-schema-review-final-binding")
        path=Path(spec["path"]);self.closure()
        need(path in self.paths,"hook-in-full-transitive-source-closure")
        need(file_hash(path)==spec["sha256"],"hook-source-bytes")
        source=regular_read(path,2*1024**2).decode("utf-8")
        tree=ast.parse(source)
        found=[node for node in tree.body if isinstance(node,(ast.FunctionDef,ast.AsyncFunctionDef))
               and node.name==spec["function"]]
        need(len(found)==1 and not isinstance(found[0],ast.AsyncFunctionDef),"one-ordinary-reviewed-hook")
        need(digest(ast.get_source_segment(source,found[0]).encode())==spec["function_sha256"],
             "exact-hook-function-method-hash")
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
        need(spec.get("kind")=="in-process-source","closed-source-hook-kind")
        key=(spec["path"],spec["sha256"])
        if key not in self.loaded:
            module_spec=importlib.util.spec_from_file_location("bound_hook_"+spec["sha256"],spec["path"])
            module=importlib.util.module_from_spec(module_spec)
            module_spec.loader.exec_module(module)
            self.loaded[key]=module
        function=getattr(self.loaded[key],spec["function"])
        need(list(inspect.signature(function).parameters)==["host","request"],"reviewed-hook-call-signature")
        result=function(host,copy.deepcopy(request))
        need(type(result) is dict and result.get("source_sha256")==spec["sha256"],
             "source-bound-hook-receipt")
        result=dict(result);result.pop("source_sha256")
        raw=canonical(result)
        need(type(spec.get("result_max_bytes")) is int and 0<=len(raw)<=spec["result_max_bytes"],
             "source-bound-hook-result-ceiling")
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
            capture_schema="single-raw-ref-v1")
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
        return self.host.record(path,raw,category,8192)
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
    def leave_phase(self,phase_id):
        self._go();p=self.active[phase_id]
        need(p["entered"],"entered-phase");p["entered"]=False
        self.host.driver.phase_limits.pop(phase_id,None)
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
        need(row["resources_closed"] and row["raw_retained"] and not row["violations"],"uncensored-owned-call")
        return dict(token=state["token"],input=copy.deepcopy(row["input"]),env=copy.deepcopy(row["env"]),
            exit=row["exit"],violations=copy.deepcopy(row["violations"]),
            binary_sha256=row["executable_sha256"],source_pin=row["source_pin"],
            identity=dict(pid=identity["pid"],creation_id=str(identity["creation_time"]),
                image_sha256=row["executable_sha256"],contained=identity["same_job"]),
            start_ns=row["process_started_ns"],end_ns=row["process_done_ns"],spawn_ns=row["spawn_total_ns"],
            value=response,raw_ref=copy.deepcopy(state["raw_reference"]),
            stdout_eof=row["stdout_eof"],stderr_eof=row["stderr_eof"],process_reaped=row["popen_reaped"],
            io_closed=row["resources_closed"],native_closed=row["resources_closed"],
            callbacks_joined=row["python_callback_joined"],peak_rss=row["observed_peak_rss"])
    def current_call_observation(self):
        self._go();d=self.host.driver;token=d.context.completed_token
        return self._observation(d.registry[token],d.context.completed_row,d.context.completed_response)
    def case_call_pairs(self,phase_id):
        self._go();p=self.active[phase_id];result=[]
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
                native_identity=dict(pid=n["pid"],creation_time=n["creation"],same_job=True),
                process_started_ns=m["process_started_ns"],process_done_ns=m["process_done_ns"],
                spawn_total_ns=m["spawn_total_ns"],stdout_eof=e["out_eof"],stderr_eof=e["err_eof"],
                popen_reaped=e["native_closed"],resources_closed=e["native_closed"] and e["io_closed"],
                raw_retained=True,python_callback_joined=e["callbacks_joined"],observed_peak_rss=e["peak"])
            rows.append(self._observation(state,row,v["response"]))
        return rows
    def abort_phase(self,phase_id,reason):
        self._go();self.host.driver.close_owned("history-abort:"+reason,self.active[phase_id]["phase"]["root"])
    def guard_prefix(self,phase_id,start_ns,end_ns):
        import zlib
        self._go();path=Path(self.host.owner["receipt"]).with_suffix(".rss.jsonl.gz")
        maximum=self.active[phase_id]["profile"]["guard_prefix_max_bytes"]
        self.host.ledger.root_name(path)
        with path.open("rb") as stream:
            n=stream.seek(0,2);need(n<=maximum,"bounded-actual-guard-prefix")
            stream.seek(0);raw=stream.read(n)
        decoder=zlib.decompressobj(31);plain=decoder.decompress(raw,maximum)
        need(not decoder.unconsumed_tail,"guard-prefix-expanded-bound")
        complete=plain[:plain.rfind(b"\n")+1];samples=[];rolling=__import__("hashlib").sha256();observed=None;last_ns=None
        for line in complete.splitlines(keepends=True):
            rolling.update(line);row=strict(line,262144)
            need(row["run_nonce"]==self.host.owner["nonce"],"actual-outer-run-prefix")
            if row.get("event")=="rss-sample" and row.get("sample_complete") is True and start_ns<=row["completed_perf_counter_ns"]<=end_ns:
                samples.append(row);observed=rolling.hexdigest();last_ns=row["completed_perf_counter_ns"]
        need(samples and 0<=end_ns-last_ns<=40_000_000,"actual-completed-prefix-within40ms")
        ref=self.host.record("guard-prefix-"+phase_id,dict(path=str(path),prefix_bytes=n,
            prefix_sha256=digest(raw),first=samples[0],last=samples[-1],samples=len(samples)),
            category="evidence",max_bytes=32768)
        return dict(status="PROVISIONAL_UNQUALIFIED",guard_sha256=self.b["guard_source"]["sha256"],
            final_receipt=None,outer_run_id=self.host.owner["nonce"],host_pid=self.host.owner["job_root_pid"],
            host_creation_id=str(self.host.owner["job_root_creation_time"]),clock_domain="Windows-QPC",
            prefix_ref=ref,observed_prefix_sha256=observed,window_start_ns=start_ns,window_end_ns=end_ns,
            last_completed_rss_ns=last_ns,owned_drain_sha256=None)
    def donor_authority(self,donor):
        self._go();spec=self.host.grant["history_donors"][donor["id"]]
        need(spec["root"]==donor["root"],"exact-owned-donor-root")
        for key in ("accepted_inputs","request_provenance","seed"):
            self.read_reference(spec[key])
        value=strict(self.read_reference(spec["accepted_inputs"]),16*1024**2)
        records=value["records"] if type(value) is dict else value
        need(type(records) is list and len(records)==donor["actual_H"],"complete-durable-donor-records")
        return dict(records=records,accepted_inputs=spec["accepted_inputs"],
            request_provenance=spec["request_provenance"],seed=spec["seed"],chain_refs=spec["chain_refs"])
    def seal_context(self,phase_id):
        self._go();p=self.active[phase_id]
        cases=[x for x in self.host.adapter.cases.values() if x["root"]==p["phase"]["root"]]
        need(cases and cases[-1]["closed"] and cases[-1].get("closure_ref"),"actual-closed-case-before-seal")
        checkpoint=self.host.checkpoint("history-seal:"+phase_id,{})
        drain=self.host.drain("history-seal:"+phase_id)
        need(drain["children_empty"] and not drain["failures"],"actual-owned-drain-before-seal")
        return dict(case_closed=True,case_closure_ref=cases[-1]["closure_ref"],
            checkpoint_ref=checkpoint["raw_ref"],owned_drain_ref=drain["raw_ref"],
            outer_run_id=self.host.owner["nonce"],host_pid=self.host.owner["job_root_pid"],
            host_creation_id=str(self.host.owner["job_root_creation_time"]),clock_domain="Windows-QPC")
    def append_phase_index(self,index):
        self._go()
        need(index["phase"] not in [x["phase"] for x in self.index],"one-phase-index")
        ref=self.host.record("history-seal-index",index,category="evidence",max_bytes=16384)
        self.index.append(copy.deepcopy(index));return ref


def HistoryNative(host):
    import importlib
    b=host.grant["runtime_bindings"]
    module=importlib.import_module("runtime_contracts")
    need(host.source_hash(module.__file__)==b["runtime_source"]["sha256"],"actual-runtime-source")
    host.history_ports=HistoryPorts(host,b)
    return module.make_runner(host,b)


class NativeCase:

    def __init__(self,host,case_id,root):
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
        path=Path(spec["path"]);need(self.host.source_hash(path)==spec["sha256"],"template-file-source")
        self.host.verify_closure();source=regular_read(path,131072).decode("utf-8")
        tree=ast.parse(source);functions=[n for n in tree.body if isinstance(n,ast.FunctionDef)]
        selected=next((n for n in functions if n.name==spec["function"]),None)
        need(selected is not None and digest(ast.get_source_segment(source,selected).encode())
             ==spec["function_sha256"] and spec["reviewed"] is True,"exact-reviewed-template-function")
        bad={f.name for f in functions if any(isinstance(n,(ast.Import,ast.ImportFrom)) for n in ast.walk(f))}
        for _ in functions:
            bad|={f.name for f in functions if any(isinstance(n,ast.Call) and isinstance(n.func,ast.Name)
                  and n.func.id in bad for n in ast.walk(f))}
        need(selected.name not in bad,"no-hidden-provider-in-constructor-closure")
        functions=[f for f in functions if f.name not in bad]
        key=spec["sha256"]
        if key not in self.loaded:
            import json,hashlib,struct,os,uuid
            namespace=dict(need=need,strict=strict,encoded=canonical,
                digest=lambda p:file_hash(p),copy=copy,json=json,hashlib=hashlib,
                struct=struct,os=os,uuid=uuid,time=time,Path=Path,
                Case=lambda a,i,r:NativeCase(self.host,i,r))
            namespace.update(extra)
            for node in tree.body:
                if isinstance(node,ast.Assign) and all(isinstance(t,ast.Name) for t in node.targets):
                    try:value=ast.literal_eval(node.value)
                    except (ValueError,TypeError):
                        if isinstance(node.value,ast.BinOp) and isinstance(node.value.op,ast.Add) and isinstance(
                            node.value.left,ast.Name) and isinstance(node.value.right,ast.Constant):
                            value=namespace[node.value.left.id]+node.value.right.value
                        else:continue
                    for target in node.targets:namespace[target.id]=value
            exec(compile(ast.Module(body=functions,type_ignores=[]),str(path),"exec"),namespace)
            self.loaded[key]=namespace
        return self.loaded[key][spec["function"]]
    def dispatch(self,case_id,root):
        spec=self.host.manifest["cases"][case_id]
        need(spec["constructor_reviewed"] is True and spec["final_pin"]==self.host.final_pin,
             "exact-final-constructor-review")
        import importlib
        paged=importlib.import_module("paged_index");schedules=importlib.import_module("focused_schedules")
        self.host.source_hash(paged.__file__);self.host.source_hash(schedules.__file__)
        function=self.load(spec["template"],template_globals(self.host,dict(paged_index=paged,schedules=schedules)))
        return function(NativeCase(self.host,case_id,root),copy.deepcopy(spec["constructor_arguments"]))

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
        case_id,_=self.host.adapter.case_for(root)
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
