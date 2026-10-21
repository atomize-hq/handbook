"""Prepared memory fixtures only; separate parent review/GO required."""
import copy
import inspect
import threading
import tracemalloc
import time
from unittest.mock import patch
import windows_driver as transport
import windows_native as native_module
import windows_evidence as evidence_module
from windows_adapter import NativeAdapter
from windows_native import Native, Pipe
from windows_deadlines import Refusal, Unsupported, canonical, digest, integer, need, strict
from windows_growth import GrowthLedger, native_call_bound, per_call_envelope
from windows_driver import WindowsDriver
from windows_hooks import BoundHooks

CONTROL_PLAN=dict(status="PREPARED_NOT_EXECUTED_PARENT_GO_REQUIRED",native_processes=0,
    engine_calls=0,filesystem_writes=0,max_checks=64,max_raw_bytes=65536,
    max_traced_python_bytes=8388608,real_case_qualification=False,
    optional_native_proposal=dict(status="UNALLOCATED_NOT_GO",bytes=2097152,files=128))

def components():
    return dict(input_bytes=1024,env_bytes=64,stdout_bytes=4096,stderr_bytes=1024,
        rss_samples=8,harness_bytes=65536,lane_record_bytes=8192,oracle_metadata_bytes=4096,
        outer_guard_bytes=4096,finalization_bytes=8192)

def ledger():
    value=GrowthLedger.__new__(GrowthLedger)
    value.lock=threading.RLock()
    value.poll=lambda:None
    value.roots={"body":dict(root="C:/owned",bytes=2000,files=100,directories=10)}
    value.anchors={"body":dict(bytes=1000,file_count=10,directory_count=2)}
    value.leases={}
    return value

class FakeNative:
    def __init__(self):
        self.terminated=[];self.lock=threading.RLock()

    def identity(self,state,live=True):
        need(not state["closed"] and state["creation_time"]==9 and state["same_job"],
             "fake-exact-identity")
        return dict(pid=12,creation_time=9,same_job=True)

    def terminate(self,state,code):
        self.identity(state)
        self.terminated.append((state["pid"],state["creation_time"],code))
        return True

def control_driver():
    d=WindowsDriver.__new__(WindowsDriver)
    d.native=FakeNative();d.lock=d.native.lock;d.registry={}
    state=dict(root="C:/owned/body",action="resolve",resources_closed=False,
        native=dict(pid=12,creation_time=9,same_job=True,closed=False),
        intentional_interrupt=None,termination_requests=[],done=threading.Event(),
        ownership="owned",termination_retired=False)
    d.registry["owned"]=state
    return d,state

def memory_controls():
    checks=[]
    def yes(name,fn):
        need(fn() is True,"control-positive:"+name);checks.append(name)
    def no(name,fn):
        try:
            fn()
        except (Refusal,Unsupported,ValueError,TypeError,KeyError):
            checks.append(name);return
        raise RuntimeError("control-negative-did-not-refuse:"+name)

    tracemalloc.start()
    yes("strict-complete",lambda:strict(b'{"a":1}',20)=={"a":1})
    no("strict-duplicate",lambda:strict(b'{"a":1,"a":2}',30))
    no("strict-tail",lambda:strict(b'{}{}',30))
    no("strict-nonfinite",lambda:strict(b'{"a":NaN}',30))
    no("strict-invalid-UTF8",lambda:strict(b'"\xff"',30))
    no("strict-byte-bound",lambda:strict(b'{}',1))
    yes("bool-not-integer",lambda:not integer(True))
    yes("canonical-exact-bytes",lambda:canonical({"z":1,"a":2})==b'{"a":2,"z":1}')

    a=ledger()
    a.reserve("one",{"body":dict(bytes=200,files=2,directories=1)})
    a.reserve("two",{"body":dict(bytes=300,files=2,directories=1)})
    yes("root-counted-once",lambda:sum(x["growth"]["body"]["bytes"] for x in a.leases.values())==500)
    no("concurrent-growth-overcap",lambda:a.reserve("three",{"body":dict(bytes=501,files=1,directories=0)}))
    yes("refused-lease-not-charged",lambda:"three" not in a.leases)
    no("duplicate-lease",lambda:a.reserve("one",{"body":dict(bytes=0,files=0,directories=0)}))
    no("unknown-root",lambda:a.reserve("unknown",{"other":dict(bytes=0,files=0,directories=0)}))
    no("bool-growth",lambda:a.reserve("bool",{"body":dict(bytes=True,files=0,directories=0)}))
    no("negative-growth",lambda:a.reserve("negative",{"body":dict(bytes=-1,files=0,directories=0)}))
    no("whole-root-twice",lambda:a.reserve("clone",{"body":dict(bytes=1001,files=0,directories=0)}))
    a.settle("one")
    yes("settled-failed-prefix-stays-charged",lambda:a.leases["one"]["settled"] and
        a.leases["one"]["growth"]["body"]["bytes"]==200)

    parts=components();envelope=per_call_envelope(parts)
    yes("all-sinks-positive",lambda:envelope["total_bytes"]>envelope["native_bytes"]>0)
    yes("no-gzip-credit",lambda:envelope["gzip_credit"]==0)
    missing=dict(parts);missing.pop("finalization_bytes")
    no("missing-finalization-component",lambda:per_call_envelope(missing))
    null=dict(parts,rss_samples=None)
    no("null-RSS-component",lambda:per_call_envelope(null))
    no("boolean-sink",lambda:per_call_envelope(dict(parts,outer_guard_bytes=True)))
    no("nonfinite-sink",lambda:per_call_envelope(dict(parts,harness_bytes=float("nan"))))
    no("negative-sink",lambda:per_call_envelope(dict(parts,lane_record_bytes=-1)))
    no("extra-sink",lambda:per_call_envelope(dict(parts,credit=-1)))
    no("oversize-output-RAM",lambda:per_call_envelope(dict(parts,stdout_bytes=16*1024**2+1)))
    profile=dict(stdout_bytes=4096,stderr_bytes=1024,rss_samples=8,
                 all_sink_components=parts)
    no("understated-input",lambda:native_call_bound(profile,{"body":"x"*1025},None))
    no("understated-RSS",lambda:native_call_bound(dict(profile,rss_samples=9),{},None))
    no("understated-output",lambda:native_call_bound(dict(profile,stdout_bytes=5000),{},None))

    d,s=control_driver()
    no("foreign-token",lambda:d.interrupt("foreign","death"))
    no("unknown-interrupt-kind",lambda:d.interrupt("owned","kill-by-name"))
    result=d.interrupt("owned","death")
    yes("owned-token-terminate",lambda:result["semantic_noncommit"] is False and len(d.native.terminated)==1)
    no("conflicting-interrupt",lambda:d.interrupt("owned","cancel"))
    d,s=control_driver();s["native"]["creation_time"]=8
    no("creation-mismatch",lambda:d.interrupt("owned","death"))
    d,s=control_driver();s["native"]["same_job"]=False
    no("foreign-Job",lambda:d.interrupt("owned","death"))
    d,s=control_driver();s["resources_closed"]=True;s["ownership"]="terminal_closed"
    no("closed-token",lambda:d.interrupt("owned","death"))
    d,s=control_driver();s["action"]="publish"
    no("writer-is-not-reader-death-control",lambda:d.interrupt("owned","death"))
    signature=inspect.signature(WindowsDriver.call)
    yes("env-explicit-keyword",lambda:signature.parameters["env"].kind==inspect.Parameter.KEYWORD_ONLY)
    yes("on-started-explicit-keyword",lambda:signature.parameters["on_started"].kind==inspect.Parameter.KEYWORD_ONLY)
    yes("no-shared-last",lambda:not hasattr(WindowsDriver,"last"))

    hooks=BoundHooks(dict(hooks={},final_pin="a"*40),lambda:None)
    no("unreleased-terminal-v2",lambda:hooks.binding("terminal-v2","a"*40))
    no("unreleased-ctor",lambda:hooks.binding("constructor:pack-old-delete","a"*40))
    no("unreleased-oracle",lambda:hooks.binding("oracle-process","a"*40))
    regressions=regression_controls()
    for name in regressions:
        yes(name,lambda:True)
    current,peak=tracemalloc.get_traced_memory();tracemalloc.stop()
    need(peak<=CONTROL_PLAN["max_traced_python_bytes"] and len(checks)<=CONTROL_PLAN["max_checks"],
         "bounded-memory-controls")
    return dict(status="MEMORY_CONTROLS_ONLY_NOT_NATIVE_QUALIFICATION",checks=checks,
        count=len(checks),traced_peak_bytes=peak,native_processes=0,engine_calls=0,
        filesystem_writes=0,bytecode_writes=0,all_engine_case_status="PENDING")

class MemoryEvidence:
    def __init__(self,fault):
        self.fault=fault;self.pending={};self.rows=[];self.chunks=[];self.failed=False
    def reserve_call(self,token,maximum):
        self.pending[token]=maximum
        if self.fault=="reserve":
            raise RuntimeError("injected-reserve-after-registration")
    def finish_call(self,token):
        del self.pending[token]
    def append(self,row):
        if self.fault=="native-row-sink" and row.get("kind")=="native-call":
            raise RuntimeError("injected-row-sink")
        self.rows.append(copy.deepcopy(row))
        return dict(path="MEMORY-ONLY",sha256=digest(canonical(row)),bytes=len(canonical(row)))
    def raw(self,token,name,offset,block):
        if self.fault=="raw-sink" and not self.failed:
            self.failed=True;raise RuntimeError("injected-raw-sink")
        self.chunks.append((name,offset,bytes(block)))

class MemoryHandle:
    def __init__(self,native,value):
        self.native=native;self.value=value;self.closed=False
    def __int__(self):
        return self.value
    def Close(self):
        need(not self.closed,"memory-double-Popen-close")
        self.closed=True;self.native.closed.add(self.value)
        self.native.trace.append("close-Popen")

class MemoryProcess:
    def __init__(self,native):
        self.native=native;self.pid=17;self.returncode=None;self.waits=0
        self._handle=MemoryHandle(native,100);self.input_closed=False
    def poll(self):
        if self.returncode is None and self.input_closed and all(p.eof for p in self.native.pipes if p.output):
            self.returncode=0
        return self.returncode
    def wait(self,timeout):
        self.waits+=1
        if self.native.fault=="reap" and self.waits==1:
            self.returncode=None
            raise transport.subprocess.TimeoutExpired("MEMORY",timeout)
        if self.returncode is None:
            self.returncode=137 if self.native.killed else 0
        return self.returncode

class MemoryPipe:
    def __init__(self,native,output):
        self.native=native;self.output=output;self.pending=False;self.eof=False;self.closed=False
        self.client_fd=1;self.client_closed=False;self.sent=False
        self.index=len(native.pipes)
    def close_client(self):
        self.client_closed=True
    def begin(self,block=None):
        self.pending=True
        if self.native.fault=="read" and self.output and not self.native.injected:
            self.native.injected=True;raise RuntimeError("injected-read")
        if self.native.fault=="write" and not self.output and not self.native.injected:
            self.native.injected=True;raise RuntimeError("injected-write")
        if not self.output:
            self.pending=False;return len(block)
        return self.complete()
    def complete(self):
        if not self.output:
            return None
        if self.native.fault=="tail" and self.index==1:
            raise RuntimeError("injected-tail-read")
        p=self.native.process
        if p is not None and (p.input_closed or p.returncode is not None):
            self.pending=False
            if not self.sent:
                self.sent=True
                return b'{"status":"ok","result":{},"wall_ns":1}' if self.index==1 else b"stderr-tail"
            self.eof=True;return b""
        return None
    def close(self):
        self.closed=True
        if not self.output and self.native.process is not None:
            self.native.process.input_closed=True

class MemoryK:
    def __init__(self,native):
        self.native=native
    @staticmethod
    def value(handle):
        return getattr(handle,"value",handle)
    def WaitForSingleObject(self,handle,timeout):
        need(self.value(handle) not in self.native.closed,"memory-use-after-close")
        return 0 if self.native.process.returncode is not None else 258
    def TerminateProcess(self,handle,code):
        need(self.value(handle) not in self.native.closed,"memory-terminate-closed-HANDLE")
        self.native.trace.append("terminate");self.native.killed=True
        if self.native.fault!="reap":
            self.native.process.returncode=code
        return True
    def CloseHandle(self,handle):
        value=self.value(handle)
        need(value not in self.native.closed,"memory-double-native-close")
        self.native.closed.add(value);self.native.trace.append("close-native")
        return True

class MemoryNative(Native):
    def __init__(self,fault):
        self.fault=fault;self.lock=threading.RLock();self.kept_pipes=[]
        self.pipes=[];self.closed=set();self.trace=[];self.process=None
        self.owner=dict(nonce="memory");self.injected=False;self.killed=False;self.samples=0
        self.K=MemoryK(self)
    def pipe(self,output_from_child,registry):
        p=MemoryPipe(self,output_from_child);self.pipes.append(p);registry.append(p)
        if self.fault=="pipe"+str(len(self.pipes)):
            raise RuntimeError("injected-pipe-constructor")
        return p
    def capture(self,process,executable,sha,owner_state):
        r=dict(handle=200,pid=17,creation_time=9,closed=False,verified=True,
               close_attempted=False,termination_retired=False)
        owner_state["native_handles"].append(r)
        if self.fault=="capture":
            raise RuntimeError("injected-capture")
        return r
    def identity(self,state,live=True):
        need(not state["closed"] and not state.get("close_attempted"),"memory-identity-closed")
        if live:
            need(not state.get("termination_retired"),"memory-retired-identity")
        return dict(pid=17,creation_time=9,same_job=True,handle_identity_verified=True)
    def memory(self,state):
        self.samples+=1
        if self.fault=="rss" and self.samples==2:
            raise RuntimeError("injected-RSS")
        return 4096,4096
    def resume(self,state):
        if self.fault=="resume":
            raise RuntimeError("injected-resume")

class MemoryTimer:
    ident=None
    def __init__(self,*args,**kwargs):
        self.daemon=False;self.cancelled=False
    def start(self):
        pass
    def cancel(self):
        self.cancelled=True
    def is_alive(self):
        return False

def transport_scenario(fault):
    native=MemoryNative(fault);sink=MemoryEvidence(fault)
    d=WindowsDriver.__new__(WindowsDriver)
    d.native=native;d.lock=native.lock;d.registry={};d.context=threading.local()
    d.evidence=sink;d.binary_sha256="a"*64;d.final_pin="b"*40
    d.absolute_deadline=time.time()+1000;d.observer=None
    profile=dict(stdout_bytes=4096,stderr_bytes=1024,rss_samples=32,callback_seconds=.001,
                 all_sink_components=dict(components(),rss_samples=32))
    def spawn(*args,**kwargs):
        if fault=="Popen":
            raise RuntimeError("injected-Popen")
        native.process=MemoryProcess(native)
        return native.process
    def image(*args,**kwargs):
        if fault=="hash":
            raise RuntimeError("injected-image-hash")
        return "a"*64
    def started(event):
        if fault=="callback-timeout":
            time.sleep(.03)
        elif fault in ("callback-error","reap","tail"):
            raise RuntimeError("injected-callback")
    class MissingGate:
        def exists(self):
            raise RuntimeError("injected-ready-read")
    gate=dict(root="C:/owned",ready="MEMORY",release="MEMORY",nonce="x",
              manifest_sha256="c"*64) if fault=="ready" else None
    caught=None
    with patch.object(transport.subprocess,"Popen",spawn),patch.object(transport,"file_hash",image), \
         patch.object(transport.threading,"Timer",MemoryTimer), \
         patch.object(transport,"Path",lambda value:MissingGate() if fault=="ready" else value):
        try:
            d._execute(["MEMORY"],dict(root="C:/owned",action="resolve"),b'{}',None,0,30,
                started if fault in ("callback-error","callback-timeout","reap","tail") else None,
                profile,script_gate=gate)
        except AssertionError as error:
            caught=error
    need(len(d.registry)==1,"one-memory-owner-state")
    s=next(iter(d.registry.values()))
    need(s["done"].is_set() and s["owner_finished"] and not sink.pending,
         "every-failed-owner-and-evidence-reservation-finished")
    need(all(p.closed and p.client_closed for p in native.pipes),"all-prior-pipes-closed")
    need(s["resources_closed"] is True,"all-memory-acquisitions-closed")
    need(caught is not None,"fault-remains-technical-failure")
    target={"hash":"image-hash","ready":"ready-read","native-row-sink":"row-sink",
        "callback-error":"callback","callback-timeout":"start-callback-timeout",
        "reap":"root-reap","tail":"raw-tail"}.get(fault,"pipe-constructor" if fault.startswith("pipe") else fault).lower()
    need(any(target in x.lower() for x in s["violations"]),"exact-fault-path-reached")
    if fault.startswith("pipe"):need(len(native.pipes)==int(fault[-1]),"exact-pipe-acquisition-fault")
    if fault=="native-row-sink":
        need(s["row"]["raw_retained"] is False,"sink-failure-not-success")
    if fault=="tail":
        need(s["row"]["stdout_eof"] is False,"failed-tail-never-fake-EOF")
    return dict(fault=fault,violations=list(s["violations"]),pipes=len(native.pipes),
                done=True,resources_closed=True,trace=native.trace,rows=len(sink.rows))

def native_close_interleaving():
    n=MemoryNative("normal");n.process=MemoryProcess(n)
    state=dict(handle=200,pid=17,creation_time=9,closed=False,verified=True,
               close_attempted=False,termination_retired=False)
    entered=threading.Event();release=threading.Event()
    def identity(record,live=True):
        if live:
            entered.set();need(release.wait(.5),"finite-memory-interleaving")
        return dict(pid=17,creation_time=9)
    n.identity=identity
    errors=[]
    def terminate():
        try:n.terminate(state)
        except BaseException as error:errors.append(repr(error))
    def close():
        try:n.close_identity(state)
        except BaseException as error:errors.append(repr(error))
    t=threading.Thread(target=terminate);u=threading.Thread(target=close)
    t.start();need(entered.wait(.5),"memory-terminator-entered");u.start()
    need("close-native" not in n.trace,"close-blocked-behind-identity-and-terminate")
    release.set();t.join(.5);u.join(.5)
    need(not t.is_alive() and not u.is_alive() and not errors,"memory-lock-threads-joined")
    need(n.trace.index("terminate")<n.trace.index("close-native"),"terminate-before-retirement")
    before=list(n.trace)
    n.closed.remove(200)  # Same numeric slot now represents a different object.
    need(n.terminate(state) is False and n.trace==before,"retired-or-reused-HANDLE-never-terminated")
    live=dict(handle=201,pid=17,creation_time=10,closed=False,verified=True,
              close_attempted=False,termination_retired=False)
    n.process.returncode=None
    def natural(record,live=True):
        if live:n.process.returncode=0
        return dict(pid=17,creation_time=10)
    n.identity=natural
    n.K.TerminateProcess=lambda handle,code:False
    need(n.terminate(live) is False,"natural-exit-between-identity-and-terminate")
    n.close_identity(live)
    return True

def capture_then_repeated_drain():
    native=MemoryNative("capture");sink=MemoryEvidence("capture")
    d=WindowsDriver.__new__(WindowsDriver);d.native=native;d.lock=native.lock;d.registry={}
    s=dict(root="C:/owned",resources_closed=True,done=threading.Event(),owner_finished=True,
           ownership="terminal_closed",termination_retired=True,process=None,
           termination_requests=[],violations=[])
    s["done"].set();d.registry["closed"]=s
    d.close_owned("one");d.close_owned("two")
    need(not native.trace,"repeated-drain-no-retired-Popen-fallback")
    return True

def constructor_lease_traces():
    from types import SimpleNamespace
    settled=[]
    ledger=SimpleNamespace(roots={"e":dict(root="C:/owned")},reserve=lambda *a:None,
                           settle=lambda key:settled.append(key))
    class PathOnly:
        parent=SimpleNamespace(is_dir=lambda:True)
        def exists(self):return False
        def open(self,*a):raise OSError("injected-open")
        def __str__(self):return "MEMORY"
    with patch.object(evidence_module,"confined",lambda *a:PathOnly()):
        try:evidence_module.Evidence("C:/owned/x",ledger,"e",max_bytes=100)
        except OSError:pass
        else:raise RuntimeError("constructor-must-refuse")
    need(len(settled)==1,"failed-evidence-constructor-lease-settled")
    for fault in ("creation","stage-live"):
        n=Native.__new__(Native);n.lock=threading.RLock();n.stage=None
        n.retired_stage=None;n.stage_close_uncertain=False;closed=[]
        n.K=SimpleNamespace(OpenProcess=lambda *a:77,CloseHandle=lambda h:closed.append(h) or True)
        n.creation=lambda handle: (_ for _ in ()).throw(RuntimeError("creation")) if fault=="creation" else 9
        n.stage_live=lambda:False
        try:n._open_stage(dict(job_root_pid=17,job_root_creation_time=9))
        except RuntimeError:pass
        else:raise RuntimeError("stage-constructor-must-refuse")
        need(closed==[77] and n.stage is None,"stage-constructor-retained-HANDLE-closed")
    return True

def pipe_component_traces():
    from types import SimpleNamespace
    for fault in ("event","client"):
        closed=[]
        k=SimpleNamespace(CreateNamedPipeW=lambda *a:11,
            CreateEventW=lambda *a:None if fault=="event" else 12,
            CreateFileW=lambda *a:None,CloseHandle=lambda h:closed.append(h) or True)
        n=SimpleNamespace(K=k,lock=threading.RLock(),kept_pipes=[])
        n.error=lambda:RuntimeError("memory-native-error")
        p=Pipe.__new__(Pipe)
        try:Pipe.__init__(p,n,True)
        except RuntimeError:pass
        else:raise RuntimeError("pipe-component-must-refuse")
        need(p.closed and 11 in closed and (fault=="event" or 12 in closed),
             "partial-event-client-constructor-closed")
    return True

def reader_end_trace(kind):
    from types import SimpleNamespace
    row=dict(input=dict(root="C:/owned",action="resolve",request_id="initial"),
        env=dict(EXP_BARRIER="pack-catalog-captured",EXP_ERROR="pack-catalog-captured"),
        executable_sha256="a"*64,call_token="reader",resources_closed=True,
        raw_retained=True,stdout_eof=True,stderr_eof=True,python_callback_joined=True,
        violations=[],native_identity=dict(pid=17,creation_time=9,same_job=True,handle_identity_verified=True),
        positive_start_event=dict(pid=17,creation_time=9,native_handle_identity=True,
            before_stdin_delivery=True,positive_rss=4096),
        intentional_interrupt=None if kind=="error" else kind,exit=2 if kind=="error" else 138 if kind=="cancel" else 137,
        stdout='{"status":"error","error":"injected-io:pack-catalog-captured"}')
    done=threading.Event();done.set()
    handle=dict(id="h",root="C:/owned",native_tokens=["reader"],joined=False,
                error=None if kind=="error" else AssertionError(copy.deepcopy(row)))
    class Joined:
        ident=None
        def is_alive(self):return False
    handle["thread"]=Joined()
    calls=[]
    driver=SimpleNamespace(binary_sha256="a"*64,
        registry={"reader":dict(token="reader",owner_finished=True,done=done,
             context=dict(handle_id="h"),raw_reference={"sha256":"x"})},
        wait_control=lambda *a:copy.deepcopy(row),
        interrupt=lambda *a:calls.append(a) or dict(termination_requested=True))
    class Hook:
        def call(self,*a):return dict(pin_proof="memory-only")
    host=SimpleNamespace(deadline=SimpleNamespace(remaining=lambda:10),
        hooks=Hook(),events=SimpleNamespace(append=lambda row:dict(sha256=digest(canonical(row)))))
    adapter=NativeAdapter.__new__(NativeAdapter)
    adapter.host=host;adapter.driver=driver;adapter.handles={"h":handle}
    first=adapter.finish_interrupted(handle,kind) if kind=="error" else adapter.interrupt(handle,kind)
    again=adapter.finish_interrupted(handle,kind)
    need(first==again and first["closed"] and first["identity_verified"]
         and first["semantic_noncommit"] is False,"unchanged-reader-end-contract-idempotent")
    need(len(calls)==(0 if kind=="error" else 1),"ordinary-error-never-signalled-repeat-finish-no-signal")
    for key,bad in (("violations",["RSS-censor"]),("raw_retained",False),
                    ("stdout_eof",False),("stderr_eof",False),("resources_closed",False),
                    ("python_callback_joined",False)):
        handle.pop("finish_proofs",None);row[key]=bad
        try:adapter.finish_interrupted(handle,kind)
        except RuntimeError:pass
        else:raise RuntimeError("technical-reader-error-hidden:"+key)
        row[key]=[] if key=="violations" else True
    handle.pop("finish_proofs",None);driver.registry["reader"]["owner_finished"]=False
    try:adapter.finish_interrupted(handle,kind)
    except RuntimeError:pass
    else:raise RuntimeError("owner-completion-fence-missing")
    return True

def regression_controls():
    """21 bounded groups; combined with43 original cases gives exactly64."""
    result=[]
    for label,faults in [
        ("callback-exception",("callback-error",)),("callback-timeout",("callback-timeout",)),
        ("successful-reap-unread-tails",("callback-error",)),("tail-and-sink-isolation",("tail","raw-sink","native-row-sink")),
        ("reap-timeout-isolation",("reap",)),("pipe1-ownership",("pipe1",)),
        ("pipe2-ownership",("pipe2",)),("pipe3-ownership",("pipe3",)),
        ("executable-hash-ownership",("hash",)),("Popen-failure-ownership",("Popen",)),
        ("capture-failure-ownership",("capture",)),("resume-failure-ownership",("resume",)),
        ("read-write-RSS-ready-failures",("read","write","rss","ready")),
        ("reservation-failure-ownership",("reserve",))]:
        for fault in faults:transport_scenario(fault)
        result.append(label)
    for label,fn in [("event-client-constructor-ownership",pipe_component_traces),
        ("stage-evidence-constructor-leases",constructor_lease_traces),
        ("close-vs-interrupt-and-natural-exit",native_close_interleaving),
        ("close-vs-close-owned-retired-fallback",capture_then_repeated_drain)]:
        need(fn() is True,label);result.append(label)
    for kind in ("error","cancel","death"):
        need(reader_end_trace(kind),"reader-end-"+kind)
        result.append("reader-end-"+kind+"-repeat-and-technical-errors")
    host_drain_traces()
    need(len(result)==21,"finite-21-regression-groups")
    return result

if __name__=="__main__":
    raise SystemExit("Prepared only: execute memory_controls through a separately approved source-bound runner.")

def host_drain_traces():
    """W5: retained unstarted/failing joins never skip later handles or finalize."""
    from types import SimpleNamespace
    from windows_capsule import LaneHost
    calls=[]
    class Thread:
        def __init__(self,name,ident,fail=False):
            self.name,self.ident,self.fail,self.live=name,ident,fail,ident is not None
        def join(self,timeout):
            calls.append(self.name)
            if self.fail:raise RuntimeError("join-injected")
            self.live=False
        def is_alive(self):return self.live
    class Events:
        def append(self,row):return dict(sha256=digest(canonical(row)))
        def close(self):calls.append("evidence");return dict(closed=True)
    h=LaneHost.__new__(LaneHost)
    h.deadline=SimpleNamespace(remaining=lambda:5)
    h.driver=SimpleNamespace(close_owned=lambda *a:calls.append("driver"),registry={})
    handles={}
    for name,ident,fail in (("unstarted",None,False),("joinfail",2,True),("later",3,False)):
        handles[name]=dict(id=name,thread=Thread(name,ident,fail),joined=False,
            error=RuntimeError("start-failed") if ident is None else None)
    h.adapter=SimpleNamespace(controls={},handles=handles)
    h.events=Events();h.ledger=SimpleNamespace(roots={})
    h.native=SimpleNamespace(close=lambda:calls.append("native"))
    p=h.drain("first")
    need("later" in calls and "unstarted" not in calls and handles["unstarted"]["joined"]
         and handles["later"]["joined"] and not p["children_empty"] and p["failures"],
         "W5-independent-ident-aware-joins-and-honest-failure")
    h.drain("second")
    need(calls.count("later")==2,"W5-repeat-drain-attempt")
    final=h.finalize();again=h.finalize()
    need(final==again and not final["own_closed"] and calls.count("native")==1
         and calls.count("evidence")==1,"W5-finalize-all-resources-once-no-false-closure")
    return True
