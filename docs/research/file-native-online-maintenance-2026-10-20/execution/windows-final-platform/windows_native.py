"""Windows HANDLE and overlapped-pipe provider. Constructor alone loads Win32.

No Jobs are created here. Each Popen child starts suspended inside the inherited
accepted QPC Job. Membership is checked against the caller's actual current Job
PID list AND retained creation-time-bound native handles; any-Job alone is unused.
"""
import ctypes as c
from ctypes import wintypes as w
import os
from pathlib import Path
import threading
import time
import uuid
from windows_deadlines import need, strict, Refusal
from windows_growth import file_hash, regular_read

PIPE_BYTES = 65536


class Overlapped(c.Structure):
    _fields_ = [("Internal",c.c_size_t),("InternalHigh",c.c_size_t),
                ("Offset",w.DWORD),("OffsetHigh",w.DWORD),("hEvent",w.HANDLE)]


class Security(c.Structure):
    _fields_ = [("nLength",w.DWORD),("lpSecurityDescriptor",c.c_void_p),
                ("bInheritHandle",w.BOOL)]


class Memory(c.Structure):
    _fields_ = [("cb",w.DWORD),("faults",w.DWORD)]+[
        (name,c.c_size_t) for name in ("peak","rss","peakpage","page","peaknon","non","pagefile","peakfile")]


class Native:
    FAILED_CONSTRUCTORS=[]
    def __init__(self, owner):
        need(os.name == "nt" and __debug__, "native-windows-unoptimized-required")
        self.K = c.WinDLL("kernel32",use_last_error=True)
        self.PS = c.WinDLL("psapi",use_last_error=True)
        self.NT = c.WinDLL("ntdll",use_last_error=True)
        K=self.K
        declarations = {
          "CloseHandle":([w.HANDLE],w.BOOL),
          "GetCurrentProcess":([],w.HANDLE),
          "GetProcessId":([w.HANDLE],w.DWORD),
          "OpenProcess":([w.DWORD,w.BOOL,w.DWORD],w.HANDLE),
          "GetProcessTimes":([w.HANDLE]+[c.POINTER(w.FILETIME)]*4,w.BOOL),
          "QueryFullProcessImageNameW":([w.HANDLE,w.DWORD,w.LPWSTR,c.POINTER(w.DWORD)],w.BOOL),
          "DuplicateHandle":([w.HANDLE,w.HANDLE,w.HANDLE,c.POINTER(w.HANDLE),w.DWORD,w.BOOL,w.DWORD],w.BOOL),
          "QueryInformationJobObject":([w.HANDLE,c.c_int,c.c_void_p,w.DWORD,c.POINTER(w.DWORD)],w.BOOL),
          "TerminateProcess":([w.HANDLE,w.UINT],w.BOOL),
          "WaitForSingleObject":([w.HANDLE,w.DWORD],w.DWORD),
          "GetExitCodeProcess":([w.HANDLE,c.POINTER(w.DWORD)],w.BOOL),
          "CreateNamedPipeW":([w.LPCWSTR,w.DWORD,w.DWORD,w.DWORD,w.DWORD,w.DWORD,w.DWORD,c.c_void_p],w.HANDLE),
          "CreateFileW":([w.LPCWSTR,w.DWORD,w.DWORD,c.POINTER(Security),w.DWORD,w.DWORD,w.HANDLE],w.HANDLE),
          "ConnectNamedPipe":([w.HANDLE,c.POINTER(Overlapped)],w.BOOL),
          "CreateEventW":([c.c_void_p,w.BOOL,w.BOOL,w.LPCWSTR],w.HANDLE),
          "ResetEvent":([w.HANDLE],w.BOOL),
          "ReadFile":([w.HANDLE,c.c_void_p,w.DWORD,c.POINTER(w.DWORD),c.POINTER(Overlapped)],w.BOOL),
          "WriteFile":([w.HANDLE,c.c_void_p,w.DWORD,c.POINTER(w.DWORD),c.POINTER(Overlapped)],w.BOOL),
          "GetOverlappedResult":([w.HANDLE,c.POINTER(Overlapped),c.POINTER(w.DWORD),w.BOOL],w.BOOL),
          "CancelIoEx":([w.HANDLE,c.POINTER(Overlapped)],w.BOOL),
        }
        for name,(args,result) in declarations.items():
            fn=getattr(K,name);fn.argtypes=args;fn.restype=result
        self.PS.GetProcessMemoryInfo.argtypes=[w.HANDLE,c.POINTER(Memory),w.DWORD]
        self.PS.GetProcessMemoryInfo.restype=w.BOOL
        self.NT.NtResumeProcess.argtypes=[w.HANDLE]
        self.NT.NtResumeProcess.restype=c.c_long
        self.owner = dict(owner)
        self.lock = threading.RLock()
        self.kept_pipes = []
        self.stage=None;self.retired_stage=None;self.stage_close_uncertain=False
        self._open_stage(owner)

    def _open_stage(self,owner):
        try:
            self.stage=self.K.OpenProcess(0x100000|0x1000,False,owner["job_root_pid"])
            need(self.stage and self.creation(self.stage)==owner["job_root_creation_time"],
                 "retained-exact-stage-native-handle")
            need(self.stage_live(),"live-owned-current-Job-stage")
        except BaseException as primary:
            try:
                self._close_stage()
            except BaseException as secondary:
                self.FAILED_CONSTRUCTORS.append(self)
                raise RuntimeError("stage-constructor:"+repr(primary)+";cleanup:"+repr(secondary)) from primary
            raise

    def _close_stage(self):
        with self.lock:
            need(not self.stage_close_uncertain,"retired-stage-handle-close-uncertain")
            if self.stage:
                handle=self.stage;self.stage=None;self.retired_stage=handle
                if not self.K.CloseHandle(handle):
                    self.stage_close_uncertain=True
                    raise self.error()

    def error(self):
        return c.WinError(c.get_last_error())

    def creation(self, handle):
        values=[w.FILETIME() for _ in range(4)]
        if not self.K.GetProcessTimes(handle,*(c.byref(x) for x in values)):
            raise self.error()
        return values[0].dwHighDateTime*2**32+values[0].dwLowDateTime

    def members(self):
        # Bounded allocation, failure/truncation never becomes an empty list.
        count=4096
        raw=c.create_string_buffer(8+count*c.sizeof(c.c_size_t))
        returned=w.DWORD()
        need(self.K.QueryInformationJobObject(None,3,raw,len(raw),c.byref(returned)),
             "current-Job-native-query")
        assigned,listed=c.cast(raw,c.POINTER(w.DWORD))[:2]
        need(0 < assigned <= listed <= count, "complete-current-Job-members")
        values=(c.c_size_t*listed).from_buffer(raw,8)
        return set(values)

    def stage_live(self):
        with self.lock:
            return bool(self.stage and self.K.WaitForSingleObject(self.stage,0)==258
                and self.creation(self.stage)==self.owner["job_root_creation_time"]
                and {os.getpid(),self.owner["job_root_pid"]}<=self.members())

    def capture(self,process,executable,expected_sha,owner_state):
        with self.lock:
            handle=w.HANDLE();current=self.K.GetCurrentProcess()
            need(self.K.DuplicateHandle(current,w.HANDLE(process._handle),current,
                 c.byref(handle),0,False,2),"duplicate-owned-child-handle")
            record=dict(handle=handle,pid=process.pid,creation_time=None,
                image=str(Path(executable).resolve()),executable_sha256=expected_sha,
                closed=False,close_attempted=False,termination_retired=False,verified=False)
            owner_state["native_handles"].append(record)
            try:
                need(self.K.GetProcessId(handle)==process.pid and self.stage_live(),
                     "Popen-child-native-identity")
                record["creation_time"]=self.creation(handle)
                self.identity(record)
                need(file_hash(executable)==expected_sha,"native-image-source-hash")
                record["verified"]=True
                return record
            except BaseException as primary:
                try:
                    self.close_identity(record,constructor_failure=True)
                except BaseException as secondary:
                    raise RuntimeError("capture:"+repr(primary)+";cleanup:"+repr(secondary)) from primary
                raise

    def identity(self,state,live=True):
        with self.lock:
            need(not state["closed"] and not state.get("close_attempted")
                 and self.K.GetProcessId(state["handle"])==state["pid"]
                 and self.creation(state["handle"])==state["creation_time"],
                 "stable-native-HANDLE-creation-identity")
            if live:
                need(not state.get("termination_retired")
                     and self.K.WaitForSingleObject(state["handle"],0)==258,"owned-child-not-live")
                need(self.stage_live() and state["pid"] in self.members(),"same-current-Job-required")
                size=w.DWORD(32768);text=c.create_unicode_buffer(size.value)
                need(self.K.QueryFullProcessImageNameW(state["handle"],0,text,c.byref(size)),
                     "native-image-path-query")
                need(os.path.normcase(text.value)==os.path.normcase(state["image"]),
                     "actual-native-image-path")
            return dict(pid=state["pid"],creation_time=state["creation_time"],
                        same_job=True if live else None,handle_identity_verified=True)

    def memory(self,state):
        with self.lock:
            self.identity(state)
            row=Memory();row.cb=c.sizeof(row)
            need(self.PS.GetProcessMemoryInfo(state["handle"],c.byref(row),row.cb)
                 and row.rss>0 and row.peak>0,"positive-owned-native-RSS")
            return row.rss,row.peak

    def resume(self,state):
        with self.lock:
            self.identity(state)
            need(self.NT.NtResumeProcess(state["handle"])==0,"native-resume-failed")

    def terminate(self,state,code=137):
        with self.lock:
            if state["closed"] or state.get("termination_retired") or state.get("close_attempted"):
                return False
            need(state.get("verified"),"verified-native-termination-capability")
            self.identity(state,live=False)
            if self.K.WaitForSingleObject(state["handle"],0)==0:
                return False
            try:
                self.identity(state)
            except Refusal:
                if self.K.WaitForSingleObject(state["handle"],0)==0:
                    return False
                raise
            if not self.K.TerminateProcess(state["handle"],code):
                if self.K.WaitForSingleObject(state["handle"],0)==0:
                    return False  # Natural exit while the HANDLE remained locked/open.
                raise self.error()
            return True

    def exited(self,state):
        with self.lock:
            return state["closed"] or self.K.WaitForSingleObject(state["handle"],0)==0

    def terminate_popen(self,state,code):
        with self.lock:
            p=state.get("process")
            if (state.get("ownership")!="owned" or state.get("termination_retired")
                or not state.get("popen_handle_open") or state.get("popen_reaped")
                or p is None or p.returncode is not None):
                return False
            need(int(p._handle)==state["popen_handle_value"],"retained-Popen-creation-handle")
            if self.K.WaitForSingleObject(w.HANDLE(state["popen_handle_value"]),0)==0:
                return False
            # No PID lookup: the still-open HANDLE came from this owner's Popen.
            if not self.K.TerminateProcess(w.HANDLE(state["popen_handle_value"]),code):
                if self.K.WaitForSingleObject(w.HANDLE(state["popen_handle_value"]),0)==0:
                    return False
                raise self.error()
            return True

    def close_popen(self,state):
        with self.lock:
            p=state.get("process")
            if p is None or state.get("popen_handle_closed"):
                return
            need(p.returncode is not None,"Popen-root-not-reaped")
            need(not state.get("popen_close_attempted"),"Popen-handle-closure-uncertain")
            state["popen_close_attempted"]=True
            state["popen_handle_open"]=False  # Retire capability BEFORE Close().
            p._handle.Close()
            state["popen_handle_closed"]=True

    def close_identity(self,state,constructor_failure=False):
        with self.lock:
            if state["closed"]:
                return
            need(not state.get("close_attempted"),"retired-native-handle-close-uncertain")
            state["termination_retired"]=True
            if state.get("verified") and not constructor_failure:
                self.identity(state,live=False)
                need(self.K.WaitForSingleObject(state["handle"],0)==0,"native-child-not-signaled")
            state["close_attempted"]=True  # Never retry an uncertain numeric HANDLE.
            need(self.K.CloseHandle(state["handle"]),"native-child-handle-close")
            state["closed"]=True

    def close(self):
        with self.lock:
            need(not self.kept_pipes,"pending-overlapped-resources-retained")
            self._close_stage()

    def pipe(self,output_from_child,registry):
        pipe=Pipe.__new__(Pipe)
        registry.append(pipe)  # Ownership precedes EVERY constructor acquisition.
        Pipe.__init__(pipe,self,output_from_child)
        return pipe


class Pipe:
    """Only the call owner performs I/O and cancellation; buffers survive uncertainty."""
    def __init__(self,native,output_from_child):
        self.native=native;self.output=output_from_child;self.pending=False
        self.closed=False;self.eof=False;self.buffer=None;self.ov=None
        self.client_fd=None;self.client_native=None;self.handle=None;self.event=None
        self.retired_parts={};self.close_uncertain=[]
        try:
            import msvcrt
            K=native.K;name=r"\\.\pipe\fn-online-"+uuid.uuid4().hex
            self.handle=K.CreateNamedPipeW(name,(1 if self.output else 2)|0x40000000|0x80000,
                0x8,1,PIPE_BYTES,PIPE_BYTES,0,None)
            need(self.handle and self.handle!=c.c_void_p(-1).value,"overlapped-server-pipe")
            self.event=K.CreateEventW(None,True,False,None)
            need(self.event,"overlapped-event")
            sa=Security(c.sizeof(Security),None,True)
            self.client_native=K.CreateFileW(name,0x40000000 if self.output else 0x80000000,
                0,c.byref(sa),3,0,None)
            need(self.client_native and self.client_native!=c.c_void_p(-1).value,"owned-client-pipe")
            self.client_fd=msvcrt.open_osfhandle(self.client_native,os.O_BINARY|
                (os.O_WRONLY if self.output else os.O_RDONLY))
            self.client_native=None  # fd now owns that HANDLE.
            self.ov=Overlapped();self.ov.hEvent=self.event
            ok=K.ConnectNamedPipe(self.handle,c.byref(self.ov))
            need(ok or c.get_last_error()==535,"connected-owned-pipe")
        except BaseException as primary:
            try:
                self.close()
            except BaseException as secondary:
                self.retain()
                raise RuntimeError("pipe-constructor:"+repr(primary)+";cleanup:"+repr(secondary)) from primary
            raise

    def retain(self):
        with self.native.lock:
            if self not in self.native.kept_pipes:
                self.native.kept_pipes.append(self)
            if self.native not in Native.FAILED_CONSTRUCTORS:
                Native.FAILED_CONSTRUCTORS.append(self.native)

    def close_client(self):
        if self.client_fd is not None:
            fd=self.client_fd;self.client_fd=None;self.retired_parts["fd"]=fd
            try:
                os.close(fd)
            except BaseException:
                self.close_uncertain.append("fd")
                raise
        if self.client_native and self.client_native!=c.c_void_p(-1).value:
            self._close_handle("client_native")

    def begin(self,block=None):
        need(not self.closed and not self.pending and not self.eof,"pipe-operation-state")
        K=self.native.K;need(K.ResetEvent(self.event),"reset-overlapped-event")
        self.ov=Overlapped();self.ov.hEvent=self.event
        size=16384 if self.output else len(block)
        need(0<size<=PIPE_BYTES and (self.output or type(block) is bytes),"bounded-pipe-block")
        self.buffer=c.create_string_buffer(size) if self.output else c.create_string_buffer(block,size)
        count=w.DWORD();call=K.ReadFile if self.output else K.WriteFile
        ok=call(self.handle,self.buffer,size,c.byref(count),c.byref(self.ov))
        if not ok and c.get_last_error() in (109,232):
            self.eof=True
            return b"" if self.output else 0
        need(ok or c.get_last_error()==997,"overlapped-submit")
        self.pending=True
        return self.complete()

    def complete(self):
        if not self.pending:
            return None
        count=w.DWORD()
        ok=self.native.K.GetOverlappedResult(self.handle,c.byref(self.ov),c.byref(count),False)
        if not ok:
            error=c.get_last_error()
            if error==996:
                return None
            if error in (109,232,995):
                self.pending=False;self.eof=True
                return b"" if self.output else 0
            raise self.native.error()
        self.pending=False
        return bytes(self.buffer[:count.value]) if self.output else count.value

    def _close_handle(self,name):
        handle=getattr(self,name)
        if not handle or handle==c.c_void_p(-1).value:
            setattr(self,name,None)
            return
        setattr(self,name,None);self.retired_parts[name]=handle
        if not self.native.K.CloseHandle(handle):
            self.close_uncertain.append(name)
            raise self.native.error()

    def close(self):
        if self.closed:
            return
        errors=[]
        try:
            self.close_client()
        except BaseException as error:
            errors.append(repr(error))
        if self.pending:
            try:
                self.native.K.CancelIoEx(self.handle,c.byref(self.ov))
                need(self.native.K.WaitForSingleObject(self.event,1000)==0,
                     "overlapped-cancel-not-complete-buffer-retained")
                self.complete()
            except BaseException as error:
                self.retain()
                raise RuntimeError("pipe-cancellation:"+repr(error)+";other:"+repr(errors)) from error
        for name in ("handle","event"):
            try:
                self._close_handle(name)
            except BaseException as error:
                errors.append(repr(error))
        if errors or self.close_uncertain:
            self.retain()
            raise RuntimeError("pipe-close-uncertain:"+repr(errors+self.close_uncertain))
        self.closed=True
        with self.native.lock:
            if self in self.native.kept_pipes:
                self.native.kept_pipes.remove(self)


class DriverSupport:
    @staticmethod
    def head(root):
        return strict(regular_read(Path(root)/"SELECTED.json",65536),65536)

    def publish(self,root,name,ops=None,**fields):
        return self.call(root,"publish",request=dict(request=name,base=self.head(root),
                         ops=ops or [],packet=False),**fields)

    def verify_binary(self,poll=None):
        need(file_hash(self.executable,poll=poll)==self.binary_sha256,"exact-executable-hash")

    def _state(self,token):
        need(type(token) is str and token in self.registry,"foreign-native-token")
        return self.registry[token]


    def _terminate(self,state,reason,code=137):
        with self.lock:  # SAME lock as Native identity/terminate/close and registry.
            if state.get("termination_retired") or state.get("ownership") in (
                    "closing","terminal_closed","terminal_uncertain"):
                return False
            if reason in state.setdefault("termination_reasons",set()):
                return False
            state["termination_reasons"].add(reason)
            state["termination_requests"].append(dict(reason=reason,code=code,
                                                       observed_ns=time.perf_counter_ns()))
            identity=state.get("native")
            if identity is not None:
                return self.native.terminate(identity,code)
            return self.native.terminate_popen(state,code)

    def _finish_resources(self,state,pipes,timer,callback,save_raw):
        """Each cleanup step is isolated; only this call owner reads/reaps."""
        def attempt(label,fn):
            try:
                return fn()
            except BaseException as error:
                state["violations"].append(label+":"+repr(error))
                return None
        p=state.get("process")
        if p is not None:
            attempt("final-owner-terminate",lambda:self._terminate(state,"final-owner-closure"))
            attempt("root-reap",lambda:p.wait(timeout=5))
            if p.returncode is None:
                attempt("reap-retry-terminate",lambda:self._terminate(state,"reap-retry"))
                attempt("root-reap-retry",lambda:p.wait(timeout=1))
            with self.lock:
                state["popen_reaped"]=p.returncode is not None
        for pipe in pipes:
            attempt("parent-client-close",pipe.close_client)
        until=time.perf_counter()+2  # UNCONDITIONAL, including successful reap.
        failed=set()
        while p is not None and time.perf_counter()<until:
            work=False
            for name,pipe in zip(("stdout","stderr"),pipes[1:]):
                if pipe.eof or pipe.closed or name in failed:
                    continue
                work=True
                try:
                    value=pipe.complete() if pipe.pending else pipe.begin()
                    if value:
                        save_raw(name,value)
                    elif value==b"":
                        pipe.eof=True;state[name+"_eof"]=True
                except BaseException as error:
                    state["violations"].append("raw-tail-"+name+":"+repr(error))
                    failed.add(name)  # NOT EOF; still close/cancel and retain uncertainty.
            if not work:
                break
            time.sleep(.001)
        if pipes and getattr(pipes[0],"pending",False):
            state["input_transfer_accounting_complete"]=False
        for pipe in pipes:
            attempt("pipe-close",pipe.close)
        state["stdin_closed"]=bool(pipes and pipes[0].closed)
        if timer is not None:
            attempt("timer-cancel",timer.cancel)
            if timer.ident is not None:
                attempt("timer-join",lambda:timer.join(1))
        state["timer_joined"]=timer is None or not timer.is_alive()
        if not state["timer_joined"]:
            state["violations"].append("deadline-callback-not-joined")
        if callback is not None and callback.ident is not None:
            attempt("callback-join",lambda:callback.join(.1))
        state["callback_joined"]=callback is None or not callback.is_alive()
        if not state["callback_joined"]:
            state["violations"].append("python-start-callback-not-joined")
        with self.lock:
            # Revoke every termination capability BEFORE releasing either HANDLE.
            state["ownership"]="closing";state["termination_retired"]=True
            for identity in state["native_handles"]:
                attempt("native-handle-close",lambda identity=identity:self.native.close_identity(identity))
            attempt("Popen-handle-close",lambda:self.native.close_popen(state))
            state["resources_closed"]=(all(x.closed for x in pipes)
                and not any(x in self.native.kept_pipes for x in pipes)
                and state["callback_joined"] and state["timer_joined"]
                and (p is None or state["popen_reaped"] and state["popen_handle_closed"])
                and all(x["closed"] for x in state["native_handles"]))
            state["ownership"]="terminal_closed" if state["resources_closed"] else "terminal_uncertain"
        return state["resources_closed"]


_CLOCK_KERNEL=None
_CLOCK_OWNER=None

def _clock_kernel():
    global _CLOCK_KERNEL
    if _CLOCK_KERNEL is None:
        need(os.name=="nt","native-Windows-QPC-only")
        k=c.WinDLL("kernel32",use_last_error=True)
        for name in ("QueryPerformanceCounter","QueryPerformanceFrequency"):
            fn=getattr(k,name);fn.argtypes=[c.POINTER(c.c_longlong)];fn.restype=w.BOOL
        _CLOCK_KERNEL=k
    return _CLOCK_KERNEL

def frequency():
    value=c.c_longlong()
    need(_clock_kernel().QueryPerformanceFrequency(c.byref(value)) and value.value>0,
         "actual-positive-QPC-frequency")
    return value.value

def counter():
    value=c.c_longlong()
    need(_clock_kernel().QueryPerformanceCounter(c.byref(value)) and value.value>=0,
         "actual-QPC-counter")
    return value.value

def bridge():
    before=counter();perf=time.perf_counter_ns();after=counter()
    need(after>=before,"ordered-QPC-perf-bracket")
    return dict(qpc_before=before,perf_ns=perf,qpc_after=after)

def _bind_clock(native,registry):
    global _CLOCK_OWNER
    need(isinstance(native,Native) and type(registry) is dict,"retained-native-clock-context")
    need(_CLOCK_OWNER is None or (_CLOCK_OWNER[0] is native and _CLOCK_OWNER[1] is registry),"one-owned-clock-context")
    _CLOCK_OWNER=(native,registry)

def process_proof(pid):
    need(type(pid) is int and pid>0 and _CLOCK_OWNER is not None,"owned-process-proof-context")
    native,registry=_CLOCK_OWNER
    with native.lock:
        if pid==native.owner["job_root_pid"]:
            need(native.stage_live(),"live-retained-stage-proof")
            handle=native.stage;created=native.creation(handle)
            size=w.DWORD(32768);text=c.create_unicode_buffer(size.value)
            need(native.K.QueryFullProcessImageNameW(handle,0,text,c.byref(size)),"stage-image-query")
            path=text.value;sha=file_hash(path)
            need(native.stage_live() and native.creation(handle)==created,"stable-retained-stage-proof")
        else:
            matches=[s["native"] for s in registry.values() if s.get("native")
                and s["native"]["pid"]==pid and s.get("ownership")=="owned"]
            need(len(matches)==1,"one-live-retained-owned-child")
            state=matches[0];native.identity(state)
            created=state["creation_time"];path=state["image"];sha=file_hash(path)
            need(sha==state["executable_sha256"],"owned-child-image-byte-binding")
            native.identity(state)
        return dict(pid=pid,creation_time=created,image_path=path,image_sha256=sha,alive=True)
