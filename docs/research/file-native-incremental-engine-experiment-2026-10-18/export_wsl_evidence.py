"""Bounded post-work WSL raw export; no engine or measurement is launched."""
import argparse,hashlib,json,subprocess,sys,time
from pathlib import Path

a=argparse.ArgumentParser();a.add_argument('--scratch',required=True);ns=a.parse_args()
scratch=Path(ns.scratch);source=Path(__file__).resolve().parent
for lane in ['serial-continuation-v2','final-verification-v3','final-proofs-v3','accounting-wsl-final-v2']:
 assert (scratch/lane/'completed.json').is_file(),lane
end=json.loads((scratch/'final-proofs-v3/reserve-bound.json').read_bytes())['conservative_deadline_epoch']
assert time.time()+900<end,'Full post-work export cap does not fit original reporting reserve'
log=scratch/'wsl-final-export';log.mkdir(exist_ok=False);began=time.time();rows=[]
packer=(source/'pack_evidence.py').read_text();packer_sha=hashlib.sha256(packer.encode()).hexdigest()
for name,root in [('native-wsl-main','/home/spenser/file-native-incremental-wsl-final-20261018'),('native-wsl-audit','/home/spenser/file-native-incremental-wsl-audit-final')]:
 code="""import hashlib,json,os,shutil,signal,sys,tarfile,time
from pathlib import Path
signal.alarm(300)
root=Path(ROOT);destination=Path(DESTINATION);transport=Path(TRANSPORT)
if not root.exists():
 print(json.dumps(dict(unperformed='Guest lane root absent',root=str(root))));raise SystemExit(0)
matches=[]
for process in Path('/proc').iterdir():
 if not process.name.isdigit() or int(process.name)==os.getpid():continue
 try:argv=[v.decode(errors='replace') for v in (process/'cmdline').read_bytes().split(b'\\0') if v]
 except (FileNotFoundError,PermissionError,ProcessLookupError):continue
 if argv and (Path(argv[0]).name in {'file-native-incremental','project-consumer','research-consumer'} or any('/home/spenser/file-native-incremental-' in v or '/home/spenser/audit_lane.py' in v for v in argv)):
  matches.append(dict(pid=int(process.name),argv=argv))
assert not matches,matches
scope={'__name__':'post_work_export'};exec(PACKER,scope)
manifest=scope['export'](root,destination)
producer=dict(epoch=time.time(),source_root=str(root),packer_sha256=PACKER_SHA,matching_known_processes=matches,qualification='Post-work raw export after all host queue completion records. Process scan covers known engine basenames and argv containing absolute owned paths, including embedded python -c code; it is not universal supervisor detection. No engine is launched.')
(destination/'export-producer.json').write_text(json.dumps(producer,indent=2))
assert not transport.exists()
with tarfile.open(transport,'w:gz') as archive:archive.add(destination,arcname=NAME)
owned_paths=list(Path.home().glob('file-native-incremental-*'))+[Path('/home/spenser/audit_lane.py')]+list(Path('/tmp').glob('i1-bounds-*'))
owned=0
for path in owned_paths:
 if path.is_file():owned+=path.stat().st_size
 elif path.is_dir():
  for d,_,names in os.walk(path):
   for n in names:owned+=(Path(d)/n).stat().st_size
assert owned<=6*1024**3 and shutil.disk_usage(root).free>=40*1024**3 and shutil.disk_usage('/mnt/c').free>=140*1024**3 and transport.stat().st_size<=256*1024**2
print(json.dumps(dict(guest_completed=True,events=manifest['events'],files=len(manifest['files']),guest_owned_logical_bytes=owned,transport_bytes=transport.stat().st_size,transport_sha256=hashlib.sha256(transport.read_bytes()).hexdigest(),producer=producer,qualification='Guest alarm bounds export to300s; successful reply follows closed TAR and final footprint/free checks. Host timeout alone does not establish guest termination.')))
"""
 substitutions={'ROOT':repr(root),'DESTINATION':repr('/home/spenser/file-native-incremental-wsl-export-'+name),'TRANSPORT':repr('/mnt/c/file-native-incremental-20261018/'+name+'-evidence.tar.gz'),'PACKER_SHA':repr(packer_sha),'PACKER':repr(packer),'NAME':repr(name)}
 # Replace longer tokens first so PACKER_SHA is not partially replaced as PACKER.
 for token,value in sorted(substitutions.items(),key=lambda pair:-len(pair[0])):code=code.replace(token,value)
 assert time.time()+330<min(end,began+900)
 with (log/(name+'.stdout')).open('wb') as out,(log/(name+'.stderr')).open('wb') as err:
  process=subprocess.Popen(['wsl','-d','Ubuntu-24.04','--','python3','-c',code],stdout=out,stderr=err)
  censored=False
  try:process.wait(timeout=330)
  except subprocess.TimeoutExpired:
   censored=True;subprocess.run(['taskkill','/PID',str(process.pid),'/T','/F'],capture_output=True,timeout=30);process.wait(timeout=30)
 row=dict(name=name,exit=process.returncode,censored=censored,guest_completion='Inspect guest reply; host timeout leaves completion/termination unconfirmed',seconds=time.time()-began)
 rows.append(row);(log/'steps.json').write_text(json.dumps(dict(deadline_epoch=end,rows=rows),indent=2))
 if censored or process.returncode:raise RuntimeError(row)
(log/'completed.json').write_text(json.dumps(dict(deadline_epoch=end,rows=rows),indent=2))
