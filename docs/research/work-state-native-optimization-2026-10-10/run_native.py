"""Run one native mode, retain raw stdout/stderr and Windows process resource peak."""
import ctypes, hashlib, json, pathlib, subprocess, sys, time
exe=pathlib.Path(sys.argv[1]);mode=sys.argv[2];out=pathlib.Path(sys.argv[3]);out.mkdir(parents=True,exist_ok=True)
class Memory(ctypes.Structure):
    _fields_=[('cb',ctypes.c_ulong),('faults',ctypes.c_ulong),('peak_working',ctypes.c_size_t),
              ('working',ctypes.c_size_t),('peak_paged',ctypes.c_size_t),('paged',ctypes.c_size_t),
              ('peak_nonpaged',ctypes.c_size_t),('nonpaged',ctypes.c_size_t),('pagefile',ctypes.c_size_t),
              ('peak_pagefile',ctypes.c_size_t),('private',ctypes.c_size_t)]
psapi=ctypes.WinDLL('psapi');psapi.GetProcessMemoryInfo.argtypes=[ctypes.c_void_p,ctypes.POINTER(Memory),ctypes.c_ulong]
start=time.time();max_working=max_private=0
with (out/f'{mode}.jsonl').open('w',encoding='utf-8') as stdout, (out/f'{mode}.stderr.txt').open('w',encoding='utf-8') as stderr:
    p=subprocess.Popen([str(exe),mode],stdout=stdout,stderr=stderr)
    while p.poll() is None:
        m=Memory();m.cb=ctypes.sizeof(m)
        if psapi.GetProcessMemoryInfo(int(p._handle),ctypes.byref(m),m.cb):
            max_working=max(max_working,m.peak_working);max_private=max(max_private,m.private)
        time.sleep(.2)
record={'command':[str(exe),mode],'exit':p.returncode,'wall_s':time.time()-start,
        'exe_sha256':hashlib.sha256(exe.read_bytes()).hexdigest(),
        'sampled_peak_private_bytes':max_private,'os_peak_working_set_bytes':max_working,
        'resource_sample_interval_ms':200,'includes_population':True}
(out/f'{mode}.run.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8')
print(json.dumps(record))
if p.returncode: sys.exit(p.returncode)
