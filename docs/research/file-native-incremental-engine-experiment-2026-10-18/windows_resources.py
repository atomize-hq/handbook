"""Bounded read-only CPU/peak-memory observer; short processes may be missed."""
import argparse,ctypes,hashlib,json,time
from ctypes import wintypes as w
from pathlib import Path

a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--output',required=True);a.add_argument('--seconds',type=int,default=21600);ns=a.parse_args()
assert 1<=ns.seconds<=21600
kernel=ctypes.WinDLL('kernel32',use_last_error=True);psapi=ctypes.WinDLL('psapi',use_last_error=True)
class Entry(ctypes.Structure):
    _fields_=[('size',w.DWORD),('usage',w.DWORD),('pid',w.DWORD),('heap',ctypes.c_size_t),('module',w.DWORD),('threads',w.DWORD),('parent',w.DWORD),('priority',w.LONG),('flags',w.DWORD),('name',w.WCHAR*260)]
class Memory(ctypes.Structure):
    _fields_=[('cb',w.DWORD),('faults',w.DWORD),('peak',ctypes.c_size_t),('working',ctypes.c_size_t),('peak_paged',ctypes.c_size_t),('paged',ctypes.c_size_t),('peak_nonpaged',ctypes.c_size_t),('nonpaged',ctypes.c_size_t),('pagefile',ctypes.c_size_t),('peak_pagefile',ctypes.c_size_t)]
kernel.CreateToolhelp32Snapshot.argtypes=[w.DWORD,w.DWORD];kernel.CreateToolhelp32Snapshot.restype=w.HANDLE
kernel.Process32FirstW.argtypes=[w.HANDLE,ctypes.POINTER(Entry)];kernel.Process32NextW.argtypes=kernel.Process32FirstW.argtypes
kernel.OpenProcess.argtypes=[w.DWORD,w.BOOL,w.DWORD];kernel.OpenProcess.restype=w.HANDLE
kernel.QueryFullProcessImageNameW.argtypes=[w.HANDLE,w.DWORD,w.LPWSTR,ctypes.POINTER(w.DWORD)]
kernel.GetProcessTimes.argtypes=[w.HANDLE,*([ctypes.POINTER(w.FILETIME)]*4)]
kernel.WaitForSingleObject.argtypes=[w.HANDLE,w.DWORD];kernel.CloseHandle.argtypes=[w.HANDLE]
psapi.GetProcessMemoryInfo.argtypes=[w.HANDLE,ctypes.POINTER(Memory),w.DWORD]
exe=Path(ns.exe).resolve();expected=str(exe).lower();sha=hashlib.sha256(exe.read_bytes()).hexdigest();active={};seen=set();end=time.monotonic()+ns.seconds
out=Path(ns.output);out.parent.mkdir(parents=True,exist_ok=True)
def ticks(v):return (v.dwHighDateTime<<32)|v.dwLowDateTime
def write(row):
    with out.open('a',encoding='utf-8') as f:f.write(json.dumps(row,separators=(',',':'))+'\n')
write({'kind':'allocation','started_epoch':time.time(),'seconds':ns.seconds,'poll_seconds':1,'exe':str(exe),'sha256':sha,'qualification':'Sampling may miss short processes; CPU covers the complete observed process, including startup. Added after the A100k recovery; no retrospective CPU claim.'})
while time.monotonic()<end and not out.with_suffix('.stop').exists():
    snap=kernel.CreateToolhelp32Snapshot(2,0);entry=Entry();entry.size=ctypes.sizeof(entry)
    if snap!=ctypes.c_void_p(-1).value:
        more=kernel.Process32FirstW(snap,ctypes.byref(entry))
        while more:
            pid=entry.pid
            if entry.name.lower()==exe.name.lower() and pid not in active:
                h=kernel.OpenProcess(0x1000|0x100000,False,pid)
                if h:
                    buf=ctypes.create_unicode_buffer(32768);size=w.DWORD(len(buf))
                    if kernel.QueryFullProcessImageNameW(h,0,buf,ctypes.byref(size)) and buf.value.lower()==expected:
                        created=w.FILETIME();exit=w.FILETIME();system=w.FILETIME();user=w.FILETIME()
                        if kernel.GetProcessTimes(h,*map(ctypes.byref,[created,exit,system,user])) and (pid,ticks(created)) not in seen:
                            active[pid]=(h,ticks(created),time.perf_counter_ns());h=None
                    if h:kernel.CloseHandle(h)
            more=kernel.Process32NextW(snap,ctypes.byref(entry))
        kernel.CloseHandle(snap)
    for pid,(h,created,first) in list(active.items()):
        if kernel.WaitForSingleObject(h,0)!=0:continue
        ct=w.FILETIME();et=w.FILETIME();st=w.FILETIME();ut=w.FILETIME();ok=kernel.GetProcessTimes(h,*map(ctypes.byref,[ct,et,st,ut]));memory=Memory();memory.cb=ctypes.sizeof(memory);mem=psapi.GetProcessMemoryInfo(h,ctypes.byref(memory),memory.cb)
        write({'kind':'exited-process','pid':pid,'created_filetime':created,'exit_filetime':ticks(et) if ok else None,'first_observed_ns':first,'observed_exit_ns':time.perf_counter_ns(),'user_cpu_ns':ticks(ut)*100 if ok else None,'system_cpu_ns':ticks(st)*100 if ok else None,'peak_working_set_bytes':memory.peak if mem else None,'exe_sha256':sha})
        seen.add((pid,created));kernel.CloseHandle(h);del active[pid]
    time.sleep(1)
for h,_,_ in active.values():kernel.CloseHandle(h)
write({'kind':'finished','epoch':time.time(),'still_active_at_stop':list(active),'sampled_exits':len(seen)})
