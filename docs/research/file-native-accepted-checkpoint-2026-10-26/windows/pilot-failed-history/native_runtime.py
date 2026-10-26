"""Small task-owned Win32 Job/pipe transport, reusing successful build interop."""
import ctypes as c, ctypes.wintypes as w
import hashlib,json,msvcrt,os,pathlib,shutil,subprocess,threading,time
import build_cli as n
O=pathlib.Path(__file__).resolve().parent
GIB=1024**3
def need(v,s):
    if not v: raise RuntimeError(s)
def sha(p):return hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def save(name,value):
    p=O/name;p.parent.mkdir(parents=True,exist_ok=True)
    p.write_text(json.dumps(value,indent=2)+'\n',encoding='utf-8')
pipe=n.api('CreatePipe',[c.POINTER(w.HANDLE),c.POINTER(w.HANDLE),c.POINTER(n.SA),w.DWORD])
inherit=n.api('SetHandleInformation',[w.HANDLE,w.DWORD,w.DWORD])
times=n.api('GetProcessTimes',[w.HANDLE,c.POINTER(w.FILETIME),c.POINTER(w.FILETIME),c.POINTER(w.FILETIME),c.POINTER(w.FILETIME)])
def identity(h,pid):
    ts=[w.FILETIME() for _ in range(4)];n.checked(times(h,*[c.byref(x) for x in ts]))
    buf=c.create_unicode_buffer(32768);size=w.DWORD(len(buf));n.checked(n.image_name(h,0,buf,c.byref(size)))
    return dict(pid=pid,creation_filetime=ts[0].dwLowDateTime+(ts[0].dwHighDateTime<<32),image=buf.value)
def rss(h):
    mem=n.MEM();mem.cb=c.sizeof(mem)
    return int(mem.WorkingSetSize) if n.papi.GetProcessMemoryInfo(h,c.byref(mem),c.sizeof(mem)) else None
class Runtime:
    def __init__(self):
        self.job=n.checked(n.create_job(None,None));limits=n.LIMIT()
        limits.BasicLimitInformation.LimitFlags=0x2000|0x200;limits.JobMemoryLimit=4*GIB
        n.checked(n.set_job(self.job,9,c.byref(limits),c.sizeof(limits)))
        self.launch_lock=threading.Lock();self.log_lock=threading.Lock();self.state_lock=threading.Lock()
        self.sequence=0;self.engine_calls=0;self.active={};self.records=[];self.violation=None
        self.stop=threading.Event();self.peak=0;self.deadline=time.monotonic()+360
        self.monitor=threading.Thread(target=self.watch,name='pilot-native-monitor',daemon=False);self.monitor.start()
    def emit(self,row):
        row=dict(perf_counter_ns=time.perf_counter_ns(),utc_epoch=time.time(),**row)
        with self.log_lock:
            with (O/'events.jsonl').open('a',encoding='utf-8') as f:f.write(json.dumps(row)+'\n');f.flush()
    def watch(self):
        with (O/'resource-samples.jsonl').open('x',encoding='utf-8') as f:
            while not self.stop.is_set():
                rows=[];total=0
                for pid in n.members(self.job):
                    h=n.open_process(0x1000|0x10,False,pid)
                    if h:
                        value=rss(h);n.close(h)
                        if value is not None:rows.append([pid,value]);total+=value
                h=n.open_process(0x1000|0x10,False,os.getpid())
                if h:total+=rss(h) or 0;n.close(h)
                self.peak=max(self.peak,total)
                row=dict(epoch=time.time(),perf_ns=time.perf_counter_ns(),rss_with_supervisor=total,process_rss=rows)
                f.write(json.dumps(row)+'\n');f.flush()
                if total>4*GIB:self.violation='aggregate-new-work-RSS-over4GiB'
                if shutil.disk_usage(O).free<140*GIB:self.violation='free-space-below140GiB'
                if time.monotonic()>self.deadline:self.violation='whole-pilot360s-timeout'
                if self.violation:n.terminate_job(self.job,124)
                self.stop.wait(.25)
    def run(self,argv,input_bytes,*,label,limit=2*GIB,timeout=60,env_extra=None,on_started=None,engine=False):
        with self.state_lock:
            self.sequence+=1;seq=self.sequence
            if engine:self.engine_calls+=1;need(self.engine_calls<=37,'engine-call-limit37')
        prefix=f'logs/{seq:02d}-{label}';start=time.monotonic();row=dict(sequence=seq,label=label,argv=argv,input_sha256=hashlib.sha256(input_bytes).hexdigest(),input_bytes=len(input_bytes),engine=engine,role_memory_limit=limit)
        save(prefix+'.input.json',json.loads(input_bytes) if input_bytes else {})
        env=os.environ.copy()
        for key in list(env):
            if key.startswith('EXP_') or key in ('RUSTFLAGS','RUSTDOCFLAGS','RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER'):env.pop(key)
        env.update(TEMP=str(O/'temp'),TMP=str(O/'temp'),TMPDIR=str(O/'temp'))
        env.update(env_extra or {})
        row['environment_overrides']=env_extra or {};save(prefix+'.command.json',row)
        pi=n.PI();own=None;parents=[];children=[];reader_threads=[];buffers={};eofs={};error=None;started=False;root_exit=None;stdout=b'';stderr=b''
        try:
            with self.launch_lock:
                own=n.checked(n.create_job(None,None));limits=n.LIMIT();limits.BasicLimitInformation.LimitFlags=0x2000|0x100;limits.ProcessMemoryLimit=limit
                n.checked(n.set_job(own,9,c.byref(limits),c.sizeof(limits)))
                sa=n.SA(c.sizeof(n.SA),None,True)
                pin_read=w.HANDLE();pin_write=w.HANDLE();n.checked(pipe(c.byref(pin_read),c.byref(pin_write),c.byref(sa),0));n.checked(inherit(pin_write,1,0))
                parents.append(pin_write.value);children.append(pin_read.value)
                channels={}
                for name in ('stdout','stderr'):
                    read=w.HANDLE();write=w.HANDLE();n.checked(pipe(c.byref(read),c.byref(write),c.byref(sa),0));n.checked(inherit(read,1,0))
                    parents.append(read.value);children.append(write.value);channels[name]=(read.value,write.value)
                si=n.START();si.cb=c.sizeof(si);si.dwFlags=0x100;si.hStdInput=pin_read;si.hStdOutput=channels['stdout'][1];si.hStdError=channels['stderr'][1]
                block=c.create_unicode_buffer('\0'.join(f'{k}={v}' for k,v in sorted(env.items(),key=lambda x:x[0].upper()))+'\0\0')
                cmd=c.create_unicode_buffer(subprocess.list2cmdline(argv))
                n.checked(n.create_process(argv[0],cmd,None,None,True,0x4|0x400|0x08000000,block,str(O),c.byref(si),c.byref(pi)));started=True
                n.checked(n.assign(self.job,pi.hProcess));n.checked(n.assign(own,pi.hProcess))
                row['native_identity']=identity(pi.hProcess,pi.dwProcessId)
                with self.state_lock:self.active[seq]=dict(pid=pi.dwProcessId,engine=engine)
                for h in children:n.close(h)
                children=[]
            def reader(name,handle):
                chunks=[];count=0
                try:
                    fd=msvcrt.open_osfhandle(handle,os.O_RDONLY|os.O_BINARY)
                    with os.fdopen(fd,'rb',buffering=0) as stream,(O/(prefix+'.'+name+'.log')).open('xb') as out:
                        while True:
                            chunk=stream.read(65536)
                            if not chunk:eofs[name]=dict(eof=True,perf_ns=time.perf_counter_ns());break
                            out.write(chunk);out.flush();chunks.append(chunk);count+=len(chunk)
                            if count>16*1024**2:self.violation='single-stream-over16MiB';n.terminate_job(self.job,124)
                except BaseException as exc:eofs[name]=dict(eof=False,error=repr(exc))
                buffers[name]=b''.join(chunks)
            for name,(handle,_) in channels.items():
                parents.remove(handle);thread=threading.Thread(target=reader,args=(name,handle),daemon=False);thread.start();reader_threads.append(thread)
            need(n.resume(pi.hThread)!=0xFFFFFFFF,'resume-failed');n.close(pi.hThread);pi.hThread=None
            value=None
            for _ in range(100):
                value=rss(pi.hProcess)
                if value and value>0:break
                time.sleep(.005)
            need(value is not None and 0<value<=limit,'positive-native-RSS-before-input')
            event=dict(**row['native_identity'],positive_rss=value,observed_ns=time.perf_counter_ns(),before_stdin_delivery=True)
            row['started_before_stdin']=event;self.emit(dict(kind='native-start-before-stdin',sequence=seq,label=label,**event))
            if on_started:on_started(event)
            handle=pin_write.value;parents.remove(handle)
            fd=msvcrt.open_osfhandle(handle,os.O_WRONLY|os.O_BINARY)
            with os.fdopen(fd,'wb',buffering=0) as stream:stream.write(input_bytes)
            row['stdin_closed_ns']=time.perf_counter_ns()
            while n.wait(pi.hProcess,100)==258:
                need(not self.violation,self.violation or 'monitor')
                value=rss(pi.hProcess)
                if value is not None:need(value<=limit,'role-RSS-limit')
                need(time.monotonic()-start<timeout,'invocation-timeout')
            code=w.DWORD();n.checked(n.exitcode(pi.hProcess,c.byref(code)));root_exit=code.value
            row['original_wait_returned_ns']=time.perf_counter_ns();row['exit']=root_exit
            need(not n.members(own),'unexpected-owned-descendant-after-root-exit')
            for thread in reader_threads:thread.join(5);need(not thread.is_alive(),'pipe-reader-EOF-timeout')
            need(all(eofs.get(name,{}).get('eof') is True for name in ('stdout','stderr')),'stdout-stderr-EOF-required')
            stdout=buffers['stdout'];stderr=buffers['stderr']
        except BaseException as exc:
            error=repr(exc)
            if started and own:
                n.terminate_job(own,125);n.wait(pi.hProcess,10000)
                value=w.DWORD()
                if n.exitcode(pi.hProcess,c.byref(value)):root_exit=value.value
        finally:
            for h in parents+children:n.close(h)
            for thread in reader_threads:thread.join(5)
            if own:
                row['job_empty']=not n.members(own);n.close(own)
            if pi.hThread:n.close(pi.hThread)
            if pi.hProcess:n.close(pi.hProcess)
            with self.state_lock:self.active.pop(seq,None)
            row.update(error=error,exit=root_exit,elapsed_seconds=time.monotonic()-start,eofs=eofs,handles_closed=all(not t.is_alive() for t in reader_threads),reader_threads_joined=all(not t.is_alive() for t in reader_threads))
            for name in ('stdout','stderr'):
                raw=buffers.get(name,b'');row[name+'_sha256']=hashlib.sha256(raw).hexdigest();row[name+'_bytes']=len(raw)
            save(prefix+'.result.json',row);self.records.append(row)
            self.emit(dict(kind='native-end',sequence=seq,label=label,result=row))
        need(error is None,'native-transport:'+str(error));need(not self.violation,self.violation or 'monitor')
        need(root_exit==0,'native-exit:'+str(root_exit))
        return stdout,stderr,row
    def close(self):
        self.stop.set();self.monitor.join(5)
        remaining=n.members(self.job)
        if remaining:n.terminate_job(self.job,125)
        for _ in range(40):
            if not n.members(self.job):break
            time.sleep(.05)
        closure=dict(job_empty=not n.members(self.job),engine_calls=self.engine_calls,resource_violation=self.violation,sampled_peak_new_work_rss=self.peak,callbacks_still_active=dict(self.active),all_transports_closed=all(r.get('handles_closed') and r.get('job_empty') for r in self.records),all_stdout_stderr_EOF=all(all(r.get('eofs',{}).get(x,{}).get('eof') for x in ('stdout','stderr')) for r in self.records))
        n.close(self.job);save('native-closure.json',closure);return closure
