"""Calibrated FileStandardInfo AllocationSize, never GetCompressedFileSize.

Regular-file data allocation only; MFT/directory metadata and device writes are
unmeasured. Tiny NTFS resident files can have allocation smaller than EOF.
"""
import argparse,ctypes,hashlib,json,os,time
from collections import defaultdict
from pathlib import Path
from ctypes import wintypes as w
assert os.name=='nt'
K=ctypes.WinDLL('kernel32',use_last_error=True)
K.CreateFileW.argtypes=[w.LPCWSTR,w.DWORD,w.DWORD,ctypes.c_void_p,w.DWORD,w.DWORD,w.HANDLE];K.CreateFileW.restype=w.HANDLE
K.GetFileInformationByHandleEx.argtypes=[w.HANDLE,ctypes.c_int,ctypes.c_void_p,w.DWORD];K.CloseHandle.argtypes=[w.HANDLE]
class Standard(ctypes.Structure):_fields_=[('allocation',ctypes.c_int64),('eof',ctypes.c_int64),('links',w.DWORD),('delete_pending',ctypes.c_ubyte),('directory',ctypes.c_ubyte)]
def allocation(path):
    absolute=str(Path(path).resolve());native=absolute if absolute.startswith('\\\\?\\') else '\\\\?\\'+absolute
    h=K.CreateFileW(native,0x80,7,None,3,0,None)
    if h==w.HANDLE(-1).value:raise ctypes.WinError(ctypes.get_last_error())
    try:
        s=Standard()
        if not K.GetFileInformationByHandleEx(h,1,ctypes.byref(s),ctypes.sizeof(s)):raise ctypes.WinError(ctypes.get_last_error())
        assert not s.directory and not s.delete_pending and s.eof>=0 and s.allocation>=0
        return s.allocation,s.eof
    finally:K.CloseHandle(h)
def calibrate(root):
    root=Path(root);root.mkdir(parents=True,exist_ok=False);rows=[]
    for n in [0,1,4095,4096,4097,65536,65537,1048576]:
        p=root/str(n)
        with p.open('xb') as f:f.write(bytes((i%251 for i in range(n))));f.flush();os.fsync(f.fileno())
        allocated,eof=allocation(p);assert eof==n
        if n>=65536:assert allocated>=n and allocated%4096==0
        rows.append(dict(path=str(p),logical_bytes=n,allocation_bytes=allocated))
    return dict(api='GetFileInformationByHandleEx(FileStandardInfo=1).AllocationSize',regular_file_data_only=True,rows=rows,tiny_resident_file_allocation_may_be_below_eof=True,directory_metadata_and_device_writes=None)
def census(root,output):
    root=Path(root);start=time.monotonic();roles=defaultdict(lambda:dict(files=0,logical_bytes=0,allocation_bytes=0));total=dict(files=0,logical_bytes=0,allocation_bytes=0)
    for directory,dirs,files in os.walk(root):
        for name in files:
            p=Path(directory)/name;assert not p.is_symlink();relative=p.relative_to(root).as_posix();role=relative.split('/')[0]
            allocated,eof=allocation(p)
            for row in [roles[role],total]:row['files']+=1;row['logical_bytes']+=eof;row['allocation_bytes']+=allocated
            if total['files']%10000==0:
                Path(output).with_suffix('.prefix.json').write_text(json.dumps(dict(total=total,roles=roles,seconds=time.monotonic()-start),indent=2))
    ledger=roles.get('expected-ledger.jsonl',dict(files=0,logical_bytes=0,allocation_bytes=0));engine={k:total[k]-ledger[k] for k in total}
    result=dict(api='FileStandardInfo.AllocationSize',total=total,engine_excluding_test_ledger=engine,roles=roles,seconds=time.monotonic()-start,directory_filesystem_metadata=None,physical_device_writes=None)
    Path(output).write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2));return result
if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('root');a.add_argument('output');a.add_argument('--calibrate',action='store_true');n=a.parse_args()
    if n.calibrate:
        result=calibrate(n.root);Path(n.output).write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
    else:census(n.root,n.output)
