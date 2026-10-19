"""Streaming role census with native allocated-file sizes where available."""
import argparse,ctypes,json,os,shutil,time
from pathlib import Path
from operational_backup import paths

def census(root):
    root=Path(root).resolve();groups={};start=time.monotonic()
    if os.name=='nt':
        size=ctypes.WinDLL('kernel32',use_last_error=True).GetCompressedFileSizeW
        size.argtypes=[ctypes.c_wchar_p,ctypes.POINTER(ctypes.c_ulong)];size.restype=ctypes.c_ulong
    for p in paths(root):
        name=p.relative_to(root).parts;role=name[0] if len(name)>1 else '(root-files)';s=p.stat()
        if os.name=='nt':
            hi=ctypes.c_ulong();ctypes.set_last_error(0);lo=size(str(p),ctypes.byref(hi))
            if lo==0xffffffff and ctypes.get_last_error():raise ctypes.WinError(ctypes.get_last_error())
            allocated=(hi.value<<32)|lo
        else:allocated=s.st_blocks*512
        r=groups.setdefault(role,dict(files=0,logical_bytes=0,compressed_size_api_bytes=0,allocated_file_bytes=None if os.name=='nt' else 0));r['files']+=1;r['logical_bytes']+=s.st_size
        if os.name=='nt':r['compressed_size_api_bytes']+=allocated
        else:r['allocated_file_bytes']+=allocated
    return dict(root=str(root),categories=groups,files=sum(v['files'] for v in groups.values()),logical_bytes=sum(v['logical_bytes'] for v in groups.values()),allocated_file_bytes=None if os.name=='nt' else sum(v['allocated_file_bytes'] for v in groups.values()),free_bytes=shutil.disk_usage(root).free,seconds=time.monotonic()-start,qualification='Windows compressed-size API does not measure cluster allocation; allocated file bytes unavailable on Windows in this census')

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--roots',nargs='+',required=True);a.add_argument('--output',required=True);ns=a.parse_args();rows=[]
    for root in ns.roots:
        if Path(root).exists():rows.append(census(root));Path(ns.output).write_text(json.dumps(rows,indent=2),encoding='utf-8')
    print(json.dumps([dict(root=r['root'],files=r['files'],logical_bytes=r['logical_bytes'],allocated_file_bytes=r['allocated_file_bytes'],seconds=r['seconds']) for r in rows],indent=2))
