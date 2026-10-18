"""Focused WSL followup after proofs, wholly inside the original report reserve."""
import argparse,json,os,shutil,subprocess,time
from pathlib import Path

a=argparse.ArgumentParser();a.add_argument('--scratch',required=True);ns=a.parse_args()
scratch=Path(ns.scratch);source=Path(__file__).resolve().parent;log=scratch/'accounting-wsl-final-v2';log.mkdir(exist_ok=False);wait_end=time.monotonic()+6*3600;bound=scratch/'final-proofs-v3/reserve-bound.json';done=scratch/'final-proofs-v3/completed.json';end=None
while not done.exists():
 if bound.exists():end=json.loads(bound.read_bytes())['conservative_deadline_epoch']
 if (end and time.time()>=end) or time.monotonic()>wait_end:break
 time.sleep(15)
if end is None and bound.exists():end=json.loads(bound.read_bytes())['conservative_deadline_epoch']
owned=[scratch,*[Path(str(scratch)+s) for s in ['-build','-build-control','-build-final']]];local_bytes=0
for root in owned:
 for directory,_,names in os.walk(root):
  for name in names:
   try:local_bytes+=(Path(directory)/name).stat().st_size
   except FileNotFoundError:pass
if not done.exists() or end is None or time.time()+1800>end or local_bytes+6*1024**3+256*1024**2>32*1024**3 or shutil.disk_usage(scratch).free<140*1024**3:
 (log/'completed.json').write_text(json.dumps(dict(unperformed='full correction cap does not fit remaining original reserve, prior proof queue unfinished, or resource bound',deadline_epoch=end,local_logical_bytes=local_bytes),indent=2));raise SystemExit(0)
started=time.time();work_deadline=min(started+1500,end-300);pin='5a65a2155f9eb297d2cbb1b7c900fa749629605c'
guest_source='/home/spenser/file-native-incremental-source-5a65a21';guest_root='/home/spenser/file-native-incremental-wsl-accounting-final';guest_evidence='/home/spenser/file-native-incremental-wsl-accounting-evidence'
code="""import hashlib,json,os,shutil,subprocess,sys,tarfile,time
from pathlib import Path,PurePosixPath
started=time.time();source=Path(SOURCE);root=Path(ROOT);archive=Path('/mnt/c/file-native-incremental-20261018/capsule-5a65a21.tar');source.mkdir(exist_ok=False)
with tarfile.open(archive) as t:
 for m in t.getmembers():assert not PurePosixPath(m.name).is_absolute() and '..' not in PurePosixPath(m.name).parts and (m.isdir() or m.isfile())
 t.extractall(source)
p=subprocess.run([sys.executable,str(source/'accounting_lane.py'),'--source',str(source),'--root',str(root),'--pin',PIN,'--reserve-gib','40','--deadline-epoch',DEADLINE],timeout=max(1,float(DEADLINE)-time.time()+30))
sys.path.insert(0,str(source));from pack_evidence import export
destination=Path(EVIDENCE);manifest=export(root,destination);exe=root/'build/release/file-native-incremental';sha=lambda b:hashlib.sha256(b).hexdigest()
environment=dict(pin=PIN,source_tar_sha256=sha(archive.read_bytes()),exe_sha256=sha(exe.read_bytes()) if exe.exists() else None,outer_exit=p.returncode,work_deadline_epoch=float(DEADLINE),elapsed_including_export=time.time()-started,free_bytes=shutil.disk_usage(root).free)
(destination/'accounting-environment.json').write_text(json.dumps(environment,indent=2))
transport=Path('/mnt/c/file-native-incremental-20261018/native-wsl-accounting-evidence.tar.gz')
with tarfile.open(transport,'w:gz') as t:t.add(destination,arcname='native-wsl-accounting')
owned=sum((Path(d)/n).stat().st_size for p in Path.home().glob('file-native-incremental-*') if p.is_dir() for d,_,names in os.walk(p) for n in names if (Path(d)/n).exists())
assert owned<=6*1024**3 and shutil.disk_usage(root).free>=40*1024**3 and shutil.disk_usage('/mnt/c').free>=140*1024**3 and transport.stat().st_size<=256*1024**2
print(json.dumps(dict(events=manifest['events'],environment=environment,guest_completed=True,guest_owned_logical_bytes_after_export=owned,transport_bytes=transport.stat().st_size,qualification='Periodic resource watchdog covers engine work. Export has a final footprint/free-space check; whole-followup completion requires this successful guest reply. Host timeout alone does not prove Linux export termination.')))
""".replace('SOURCE',repr(guest_source)).replace('ROOT',repr(guest_root)).replace('EVIDENCE',repr(guest_evidence)).replace('PIN',repr(pin)).replace('DEADLINE',repr(str(work_deadline)))
argv=['wsl','-d','Ubuntu-24.04','--','python3','-c',code];censored=False
with (log/'launch.stdout').open('wb') as out,(log/'launch.stderr').open('wb') as err:
 p=subprocess.Popen(argv,stdout=out,stderr=err)
 try:p.wait(timeout=min(1800,end-time.time()))
 except subprocess.TimeoutExpired:
  censored=True;subprocess.run(['taskkill','/PID',str(p.pid),'/T','/F'],capture_output=True,timeout=30);p.wait(timeout=30)
(log/'completed.json').write_text(json.dumps(dict(pin=pin,argv=argv,started_epoch=started,deadline_epoch=end,cap_seconds=1800,work_deadline_epoch=work_deadline,exit=p.returncode,censored=censored,seconds=time.time()-started,guest_export_completion='confirmed by successful guest process completion' if p.returncode==0 and not censored else 'unconfirmed; export may be pending and must be inspected/terminated before final quiescence claims'),indent=2))
