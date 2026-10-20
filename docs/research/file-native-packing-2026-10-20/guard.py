"""Preemptive owned process-group watchdog and conservative ownership census.

Windows children start suspended, join a kill-on-close Job, then resume. Native
children start a new session. Never terminates processes outside that group.
"""
import argparse, ctypes, datetime, json, os, shutil, signal, subprocess, sys, time, threading
from pathlib import Path

ROOT=Path('C:/file-native-packing-20261020')
WORK=Path(__file__).resolve().parents[3]
CUTOFF=datetime.datetime.fromisoformat('2026-10-20T07:55:16+00:00').timestamp()
def census(path):
    total=count=0
    if not Path(path).exists():return dict(bytes=0,files=0)
    for directory,dirs,files in os.walk(path,followlinks=False):
        for name in files:
            p=Path(directory)/name
            if p.is_symlink():raise ValueError('census-symlink:'+str(p))
            try:total+=p.stat().st_size;count+=1
            except FileNotFoundError:pass
    return dict(bytes=total,files=count)
def scopes():
    return {'runtime':str(ROOT),'worktree':str(WORK),'cargo':str(Path.home()/'.cargo'),'shared_git':'C:/Users/spmcc/Documents/__Project_Code/handbook/.git'}
def ownership():return {k:census(v) for k,v in scopes().items()}
def current():
    base=json.loads((ROOT/'receipts/ownership-baseline.json').read_bytes())
    now=ownership();b=f=0
    for k,x in now.items():
        b+=x['bytes'] if k in ['runtime','worktree'] else max(0,x['bytes']-base['scopes'][k]['bytes'])
        f+=x['files'] if k in ['runtime','worktree'] else max(0,x['files']-base['scopes'][k]['files'])
    return dict(scopes=now,total_new_bytes=b,total_new_files=f,free_bytes=shutil.disk_usage(ROOT).free,time_utc=datetime.datetime.now(datetime.timezone.utc).isoformat())

if os.name=='nt':
    from ctypes import wintypes as w
    K=ctypes.WinDLL('kernel32',use_last_error=True)
    class Basic(ctypes.Structure):
        _fields_=[('User',ctypes.c_int64),('Kernel',ctypes.c_int64),('Flags',w.DWORD),('Min',ctypes.c_size_t),('Max',ctypes.c_size_t),('Active',w.DWORD),('Affinity',ctypes.c_size_t),('Priority',w.DWORD),('Scheduling',w.DWORD)]
    class IO(ctypes.Structure):_fields_=[(x,ctypes.c_uint64) for x in ['ro','wo','oo','rb','wb','ob']]
    class Limits(ctypes.Structure):_fields_=[('Basic',Basic),('IO',IO),('Process',ctypes.c_size_t),('Job',ctypes.c_size_t),('PeakProcess',ctypes.c_size_t),('PeakJob',ctypes.c_size_t)]
    class Memory(ctypes.Structure):_fields_=[('cb',w.DWORD),('faults',w.DWORD)]+[(x,ctypes.c_size_t) for x in ['peak','rss','peakpage','page','peaknon','non','pagefile','peakfile']]
    K.CreateJobObjectW.restype=w.HANDLE;K.CreateJobObjectW.argtypes=[ctypes.c_void_p,w.LPCWSTR]
    K.SetInformationJobObject.argtypes=[w.HANDLE,ctypes.c_int,ctypes.c_void_p,w.DWORD]
    K.AssignProcessToJobObject.argtypes=[w.HANDLE,w.HANDLE]
    K.TerminateJobObject.argtypes=[w.HANDLE,w.UINT];K.CloseHandle.argtypes=[w.HANDLE]
    K.QueryInformationJobObject.argtypes=[w.HANDLE,ctypes.c_int,ctypes.c_void_p,w.DWORD,ctypes.c_void_p]
    K.OpenProcess.restype=w.HANDLE;K.OpenProcess.argtypes=[w.DWORD,w.BOOL,w.DWORD]
    PS=ctypes.WinDLL('psapi');PS.GetProcessMemoryInfo.argtypes=[w.HANDLE,ctypes.POINTER(Memory),w.DWORD]
    def rss(pid):
        h=K.OpenProcess(0x410,False,pid)
        if not h:return None
        m=Memory();m.cb=ctypes.sizeof(m);ok=PS.GetProcessMemoryInfo(h,ctypes.byref(m),m.cb);K.CloseHandle(h)
        return (m.rss,m.peak) if ok else None
    def pids(job):
        buf=ctypes.create_string_buffer(8+8*512)
        if not K.QueryInformationJobObject(job,3,buf,len(buf),None):return None
        n=int.from_bytes(buf.raw[4:8],'little')
        return [int.from_bytes(buf.raw[8+i*ctypes.sizeof(ctypes.c_size_t):8+(i+1)*ctypes.sizeof(ctypes.c_size_t)],'little') for i in range(min(n,512))]

def run(argv,seconds,receipt,cwd=None,limit=2*1024**3,allowance=536870912,files=25000,stdin=None,env=None):
    begin=time.time();timeout=min(seconds,max(0,CUTOFF-begin));assert timeout>0,'functional-cutoff'
    receipt=Path(receipt);receipt.parent.mkdir(parents=True,exist_ok=True)
    out=receipt.with_suffix('.stdout').open('wb');err=receipt.with_suffix('.stderr').open('wb')
    data=Path(stdin).read_bytes() if stdin else None
    if os.name=='nt':
        job=K.CreateJobObjectW(None,None);assert job
        lim=Limits();lim.Basic.Flags=0x2000|0x100|0x200;lim.Process=limit;lim.Job=4*1024**3
        assert K.SetInformationJobObject(job,9,ctypes.byref(lim),ctypes.sizeof(lim))
        p=subprocess.Popen(argv,cwd=cwd,stdin=subprocess.PIPE if data is not None else subprocess.DEVNULL,stdout=out,stderr=err,env=env,creationflags=0x4)
        if not K.AssignProcessToJobObject(job,w.HANDLE(p._handle)):
            p.kill();raise ctypes.WinError(ctypes.get_last_error())
        nt=ctypes.WinDLL('ntdll');nt.NtResumeProcess.argtypes=[w.HANDLE];assert nt.NtResumeProcess(w.HANDLE(p._handle))==0
        kill=lambda:K.TerminateJobObject(job,137)
    else:
        p=subprocess.Popen(argv,cwd=cwd,stdin=subprocess.PIPE if data is not None else subprocess.DEVNULL,stdout=out,stderr=err,env=env,start_new_session=True)
        kill=lambda:os.killpg(p.pid,signal.SIGKILL)
    peaks={};missing=0;aggregate=0;why=None;samples=[];next_disk=0
    stop=threading.Event();termination={};termination_lock=threading.Lock();group_observations={};job_query_errors=0
    def group_members():
        nonlocal job_query_errors
        if os.name=='nt':
            members=pids(job)
            if members is None:job_query_errors+=1
        else:
            try:os.killpg(p.pid,0);members=[p.pid]
            except ProcessLookupError:members=[]
        if p.poll() is not None:group_observations.setdefault('root_exit_seen_epoch',time.time())
        if members==[]:group_observations.setdefault('group_empty_seen_epoch',time.time())
        return members
    def terminate(reason):
        nonlocal why
        with termination_lock:
            members=group_members()
            if members==[]:return
            if why is None:why=reason
            termination['requested_epoch']=time.time();termination['reason']=reason
            kill()
            termination['members_before_termination']=members
            try:p.wait(timeout=10);termination['child_exit_epoch']=time.time();termination['exit']=p.returncode
            except subprocess.TimeoutExpired:termination['termination_wait_failed']=True
            until=time.monotonic()+5
            while time.monotonic()<until:
                remaining_now=group_members()
                if remaining_now==[]:break
                time.sleep(.01)
            termination['group_after_termination']=remaining_now
            termination['group_termination_observed_epoch']=time.time()
    # Armed before either stdin or the first inventory. This thread does no disk
    # census and cannot be delayed by the monitor's enumeration or blocked write.
    deadline=threading.Timer(max(0,begin+timeout-time.time()),terminate,args=('time-censor',));deadline.daemon=True;deadline.start()
    def feed():
        try:p.stdin.write(data);p.stdin.close()
        except (OSError,ValueError):pass
    writer=None
    if data is not None:writer=threading.Thread(target=feed,daemon=True);writer.start()
    def sample_memory():
        nonlocal missing,aggregate
        while not stop.is_set():
            members=group_members()
            if members==[]:return
            if os.name=='nt':
                current_sum=0
                for pid in (members or [])+[os.getpid()]:
                    n=rss(pid)
                    if n is None:missing+=1;continue
                    peaks[str(pid)]=max(peaks.get(str(pid),0),n[1]);current_sum+=n[0]
                    if n[0]>limit:terminate('process-rss-censor');return
                aggregate=max(aggregate,current_sum)
                if current_sum>4*1024**3:terminate('aggregate-rss-censor');return
            stop.wait(.04)
    sampler=threading.Thread(target=sample_memory,daemon=True);sampler.start()
    try:
        while group_members()!=[]:
            if time.time()>=next_disk:
                sample=current();samples.append(sample);next_disk=time.time()+5
                if sample['total_new_bytes']>allowance or sample['total_new_files']>files or sample['free_bytes']<140*1024**3:
                    terminate('ownership-or-free-censor');break
            time.sleep(.04)
        p.wait(timeout=10)
    finally:
        if group_members()!=[]:terminate('supervisor-finally')
        deadline.cancel();deadline.join(timeout=11);stop.set();sampler.join(timeout=3)
        if writer:writer.join(timeout=1)
        if os.name=='nt':
            remaining=pids(job)
            if remaining!=[]:K.TerminateJobObject(job,137)
            K.CloseHandle(job)
        else:remaining=[]
        out.close();err.close()
    result=dict(argv=argv,cwd=str(cwd) if cwd else None,started_utc=datetime.datetime.fromtimestamp(begin,datetime.timezone.utc).isoformat(),seconds=time.time()-begin,cap_seconds=seconds,effective_seconds=timeout,exit=p.returncode,censor=why,independent_deadline=termination or None,group_observations=group_observations,job_query_errors=job_query_errors,per_process_rss_limit=limit,aggregate_rss_limit=4*1024**3,sampled_peak_rss_by_pid=peaks or None,missing_samples=missing,aggregate_sampled_peak_rss=aggregate or None,ownership_samples=samples,final_ownership=current(),group_remaining_before_closure=remaining)
    receipt.write_text(json.dumps(result,indent=2),encoding='utf-8');return result

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--baseline',action='store_true');a.add_argument('--census',action='store_true');a.add_argument('--seconds',type=int,default=120);a.add_argument('--receipt');a.add_argument('--cwd');a.add_argument('--stdin');a.add_argument('--allowance',type=int,default=536870912);a.add_argument('--files',type=int,default=25000);a.add_argument('--limit',type=int,default=2*1024**3);a.add_argument('argv',nargs=argparse.REMAINDER);n=a.parse_args()
    ROOT.mkdir(exist_ok=True)
    if n.baseline:
        p=ROOT/'receipts/ownership-baseline.json';assert not p.exists();r=dict(scopes=ownership(),time_utc=datetime.datetime.now(datetime.timezone.utc).isoformat());p.write_text(json.dumps(r,indent=2));print(json.dumps(r))
    elif n.census:print(json.dumps(current(),indent=2))
    else:
        argv=n.argv[1:] if n.argv[0]=='--' else n.argv;r=run(argv,n.seconds,n.receipt,n.cwd,n.limit,n.allowance,n.files,n.stdin);print(json.dumps({k:v for k,v in r.items() if k not in ['ownership_samples']},indent=2));sys.exit(0 if r['exit']==0 and not r['censor'] else 1)
