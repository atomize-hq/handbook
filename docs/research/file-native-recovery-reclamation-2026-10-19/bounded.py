"""Finite owned-process runner. Receipts retain censors and exact output files."""
import argparse,ctypes,json,os,subprocess,time
from pathlib import Path

def rss(pid):
    if os.name=='nt':
        class Counters(ctypes.Structure):
            _fields_=[('cb',ctypes.c_ulong),('faults',ctypes.c_ulong)]+[(n,ctypes.c_size_t) for n in ['peak','working','peak_paged','paged','peak_nonpaged','nonpaged','pagefile','peak_pagefile']]
        kernel=ctypes.WinDLL('kernel32',use_last_error=True);kernel.OpenProcess.restype=ctypes.c_void_p;kernel.CloseHandle.argtypes=[ctypes.c_void_p]
        h=kernel.OpenProcess(0x410,False,pid)
        if not h:return 0
        c=Counters();c.cb=ctypes.sizeof(c);ps=ctypes.WinDLL('psapi');ps.GetProcessMemoryInfo.argtypes=[ctypes.c_void_p,ctypes.c_void_p,ctypes.c_ulong]
        ok=ps.GetProcessMemoryInfo(h,ctypes.byref(c),c.cb);kernel.CloseHandle(h);return c.peak if ok else 0
    try:
        for line in Path(f'/proc/{pid}/status').read_text().splitlines():
            if line.startswith('VmHWM:'):return int(line.split()[1])*1024
    except FileNotFoundError:pass
    return 0

def run(args,output,cap,deadline=None,memory=2*1024**3,cwd=None):
    output=Path(output);output.parent.mkdir(parents=True,exist_ok=True);start=time.time()
    if deadline is not None and start+cap>deadline:raise RuntimeError('full-cap-does-not-fit')
    peak=0;censor=None
    with Path(str(output)+'.stdout').open('wb') as out,Path(str(output)+'.stderr').open('wb') as err:
        p=subprocess.Popen([str(a) for a in args],stdout=out,stderr=err,cwd=cwd,start_new_session=os.name!='nt')
        while p.poll() is None:
            peak=max(peak,rss(p.pid))
            if time.time()-start>=cap:censor='time-cap'
            if peak>memory:censor='memory-cap'
            if censor:
                if os.name=='nt':subprocess.run(['taskkill','/PID',str(p.pid),'/T','/F'],capture_output=True,timeout=30)
                else:
                    import signal
                    os.killpg(p.pid,signal.SIGKILL)
                p.wait(timeout=30);break
            time.sleep(.25)
    row=dict(args=[str(a) for a in args],start_epoch=start,seconds=time.time()-start,exit=p.returncode,status='censored' if censor else ('success' if p.returncode==0 else 'failure'),censor=censor,peak_memory_bytes=peak)
    Path(str(output)+'.json').write_text(json.dumps(row,indent=2),encoding='utf-8');return row

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--output',required=True);a.add_argument('--cap',type=float,required=True);a.add_argument('--deadline',type=float);a.add_argument('command',nargs=argparse.REMAINDER);ns=a.parse_args();row=run(ns.command,ns.output,ns.cap,ns.deadline);print(json.dumps(row,indent=2));raise SystemExit(0 if row['status']=='success' else 1)
