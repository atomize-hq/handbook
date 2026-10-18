"""Read-only final census of the owned native Linux experiment paths."""
import json,os,shutil,time
from pathlib import Path

roots=sorted(Path.home().glob('file-native-incremental-*'))
roots+=sorted(Path.home().glob('native-linux-*-evidence.tar.gz'))
roots+=sorted(Path('/tmp').glob('i1-bounds-*'))
rows=[]
for root in roots:
 files=[root] if root.is_file() else [Path(d)/n for d,_,names in os.walk(root) for n in names]
 logical=allocated=count=0
 for path in files:
  try:st=path.stat()
  except FileNotFoundError:continue
  count+=1;logical+=st.st_size;allocated+=st.st_blocks*512
 rows.append(dict(path=str(root),files=count,logical_bytes=logical,allocated_file_bytes=allocated))
processes=[]
for process in Path('/proc').iterdir():
 if not process.name.isdigit():continue
 try:argv=[v.decode(errors='replace') for v in (process/'cmdline').read_bytes().split(b'\0') if v]
 except (FileNotFoundError,PermissionError,ProcessLookupError):continue
 if not argv:continue
 engine=Path(argv[0]).name in {'file-native-incremental','project-consumer','research-consumer'}
 owned_python=Path(argv[0]).name.startswith('python') and any(v.startswith('/home/spenser/file-native-incremental-') and v.endswith('.py') for v in argv[1:])
 if engine or owned_python:processes.append(dict(pid=int(process.name),argv=argv))
total=sum(v['logical_bytes'] for v in rows);free=shutil.disk_usage(Path.home()).free
result=dict(epoch=time.time(),roots=rows,total_logical_bytes=total,total_allocated_file_bytes=sum(v['allocated_file_bytes'] for v in rows),logical_cap_bytes=6*1024**3,free_bytes=free,required_free_bytes=20*1024**3,matching_owned_processes=processes,qualification='Read-only post-work snapshot. Includes all incremental-prefixed home paths, preserved native evidence transports and all bounds fixtures conservatively. Allocated file bytes sum st_blocks per pathname, may repeat hardlinks, exclude directory/filesystem metadata, and are not unique physical/device usage. Process matches cover the three named engine basenames and Python argv with absolute incremental-prefixed .py paths; Cargo/Bash/relative scripts/python -c or stdin supervisors are outside this filter. Completed controller records establish their own children finished. Current Git report/export copies on the Windows host are separate.')
assert total<=6*1024**3 and free>=20*1024**3 and not processes,result
print(json.dumps(result,indent=2))
