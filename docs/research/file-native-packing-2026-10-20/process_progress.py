"""Read-only Windows logical I/O/RSS samples; not physical device I/O."""
import ctypes,datetime,json,subprocess
from pathlib import Path
from ctypes import wintypes as w
from guard import K,rss
class Counters(ctypes.Structure):_fields_=[(name,ctypes.c_uint64) for name in ['read_operations','write_operations','other_operations','read_transfer_bytes','write_transfer_bytes','other_transfer_bytes']]
K.GetProcessIoCounters.argtypes=[w.HANDLE,ctypes.POINTER(Counters)]
p=subprocess.run(['powershell','-NoProfile','-Command',"Get-Process candidate1 -ErrorAction SilentlyContinue | Select-Object Id,CPU,WorkingSet64,PeakWorkingSet64,StartTime | ConvertTo-Json -Compress"],capture_output=True,timeout=10)
data=json.loads(p.stdout) if p.stdout.strip() else [];data=[data] if isinstance(data,dict) else data;rows=[]
for proc in data:
    pid=proc['Id'];handle=K.OpenProcess(0x410,False,pid);io=None
    if handle:
        c=Counters()
        if K.GetProcessIoCounters(handle,ctypes.byref(c)):io={name:getattr(c,name) for name,_ in c._fields_}
        K.CloseHandle(handle)
    rows.append(dict(process=proc,io_counters=io,rss=rss(pid)))
root=Path('C:/file-native-packing-20261020/history-restored/disposable/store');packs=list((root/'packs').glob('*')) if (root/'packs').exists() else []
result=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),processes=rows,pack_files=len(packs),pack_bytes=sum(p.stat().st_size for p in packs if p.is_file()),journal=json.loads((root/'PACKING.json').read_bytes()) if (root/'PACKING.json').exists() else None,qualification='Windows process logical I/O counters; no device write/amplification claim')
with Path('C:/file-native-packing-20261020/receipts/P0-progress-samples.jsonl').open('a') as f:f.write(json.dumps(result)+'\n')
print(json.dumps(result,indent=2))
