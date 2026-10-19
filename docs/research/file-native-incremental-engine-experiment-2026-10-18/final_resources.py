"""Read-only closeout census; no engine work or cleanup is performed."""
import argparse,json,os,shutil,subprocess,sys,tempfile,time
from pathlib import Path

a=argparse.ArgumentParser();a.add_argument('--scratch',required=True);a.add_argument('--output',required=True);ns=a.parse_args()
scratch=Path(ns.scratch).resolve();source=Path(__file__).resolve().parent
end=json.loads((scratch/'final-proofs-v3/reserve-bound.json').read_bytes())['conservative_deadline_epoch']
assert time.time()+600<end,'Full census cap must fit original reporting reserve'
for lane in ['serial-continuation-v2','final-verification-v3','final-proofs-v3','accounting-wsl-final-v2','wsl-final-export']:
 assert (scratch/lane/'completed.json').exists(),lane
began=time.time();local_end=time.monotonic()+590;work=source.parents[2]
# The caller also enforces an outer600-second descendant-process cap. Inner
# checks reserve75 seconds for subprocess caps and10 seconds for the receipt.
common=Path(subprocess.check_output(['git','rev-parse','--git-common-dir'],cwd=work,text=True,timeout=15).strip())
if not common.is_absolute():common=(work/common).resolve()
roots=[scratch,*[Path(str(scratch)+suffix) for suffix in ['-build','-build-control','-build-final']],work,common]
roots+=sorted(Path(tempfile.gettempdir()).glob('i1-bounds-*'))
rows=[]
for root in roots:
 logical=count=0
 if root.is_file():logical=root.stat().st_size;count=1
 elif root.is_dir():
  for directory,_,names in os.walk(root):
   assert time.monotonic()+85<local_end,'Local census exhausted its reserved share'
   for name in names:
    assert time.monotonic()+85<local_end,'Local census exhausted its reserved share'
    try:logical+=(Path(directory)/name).stat().st_size;count+=1
    except FileNotFoundError:pass
 rows.append(dict(path=str(root),exists=root.exists(),files=count,logical_bytes=logical))
 assert time.monotonic()+85<local_end,'Preserve time for guest snapshot and receipt'
guest="""import json,os,shutil,signal,time
from pathlib import Path
signal.alarm(40)
rows=[]
for root in sorted(Path.home().glob('file-native-incremental-*'))+[Path('/home/spenser/audit_lane.py')]+sorted(Path('/tmp').glob('i1-bounds-*')):
 logical=count=0
 if root.is_file():logical=root.stat().st_size;count=1
 elif root.is_dir():
  for directory,_,names in os.walk(root):
   for name in names:
    try:logical+=(Path(directory)/name).stat().st_size;count+=1
    except FileNotFoundError:pass
 rows.append(dict(path=str(root),files=count,logical_bytes=logical))
matches=[]
for p in Path('/proc').iterdir():
 if not p.name.isdigit() or int(p.name)==os.getpid():continue
 try:argv=[v.decode(errors='replace') for v in (p/'cmdline').read_bytes().split(b'\\0') if v]
 except (FileNotFoundError,PermissionError,ProcessLookupError):continue
 if argv and (Path(argv[0]).name in {'file-native-incremental','project-consumer','research-consumer'} or any('/home/spenser/file-native-incremental-' in v or '/home/spenser/audit_lane.py' in v for v in argv)):
  matches.append(dict(pid=int(p.name),argv=argv))
print(json.dumps(dict(epoch=time.time(),roots=rows,logical_bytes=sum(r['logical_bytes'] for r in rows),free_bytes=shutil.disk_usage(Path.home()).free,matching_known_processes=matches)))
"""
assert time.monotonic()+85<local_end
reply=subprocess.run(['wsl','-d','Ubuntu-24.04','--','python3','-c',guest],capture_output=True,timeout=45,check=True)
wsl=json.loads(reply.stdout.decode('utf-8'))
# Restrict inspection to Python/controllers and executables in our own runtime.
ps="Get-CimInstance Win32_Process | Where-Object { ($_.Name -eq 'python.exe' -or $_.Name -like '*consumer*' -or $_.Name -eq 'file-native-incremental.exe') -and ($_.CommandLine -like '*file-native-incremental-20261018*' -or $_.CommandLine -like '*file-native-incremental-engine-experiment-2026-10-18*' -or $_.CommandLine -match 'finish_verification.py|finish_proofs.py|accounting_wsl_final.py|serial_windows_v2.py|progress_watch.py') } | Select-Object ProcessId,Name,ExecutablePath,CommandLine | ConvertTo-Json -Depth 3 -Compress"
assert time.monotonic()+35<local_end
reply=subprocess.run(['pwsh','-NoProfile','-Command',ps],capture_output=True,text=True,timeout=30,check=True)
processes=json.loads(reply.stdout) if reply.stdout.strip() else []
if isinstance(processes,dict):processes=[processes]
processes=[p for p in processes if p['ProcessId'] not in {os.getpid(),os.getppid()}]
local=sum(r['logical_bytes'] for r in rows);free=shutil.disk_usage(scratch).free
report=dict(epoch=time.time(),seconds=time.time()-began,deadline_epoch=end,local_roots=rows,local_logical_bytes=local,guest=wsl,combined_logical_bytes=local+wsl['logical_bytes'],host_free_bytes=free,matching_known_windows_processes=processes,
 limits=dict(combined_logical_bytes=32*1024**3,guest_logical_bytes=6*1024**3,host_free_bytes=140*1024**3,guest_free_bytes=40*1024**3),
 qualification='Post-work file-length snapshot, including the full managed worktree, conservative entire shared Git database, transports, sibling builds and temporary i1-bounds fixtures. Per-path sums can repeat shared data and do not measure unique physical allocation. It precedes its own receipt and final small report/inventory/commit writes. Process filters cover known engine basenames and owned absolute argv, plus known Windows controller script names; they are not universal supervisor detection. Completed controller records separately cover their children. A guest timeout would leave guest termination unconfirmed; no destructive action is performed.')
report['within_resource_bounds']=local+wsl['logical_bytes']<=32*1024**3 and wsl['logical_bytes']<=6*1024**3 and free>=140*1024**3 and wsl['free_bytes']>=40*1024**3
Path(ns.output).write_text(json.dumps(report,indent=2));print(json.dumps({k:report[k] for k in ['combined_logical_bytes','host_free_bytes','within_resource_bounds','seconds']}))
assert report['within_resource_bounds']
assert not processes and not wsl['matching_known_processes'],'Inspect matching processes before a closeout claim'
