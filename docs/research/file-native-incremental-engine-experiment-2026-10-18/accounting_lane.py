"""Finite affected-accounting regression lane; no prior host deadline is reset."""
import argparse,json,os,shutil,signal,subprocess,sys,time
from pathlib import Path

a=argparse.ArgumentParser();a.add_argument('--root',required=True);a.add_argument('--source',required=True);a.add_argument('--pin',required=True);a.add_argument('--reserve-gib',type=int,default=20);a.add_argument('--deadline-epoch',type=float);a.add_argument('--target');ns=a.parse_args()
root=Path(ns.root).resolve();root.mkdir(exist_ok=False);source=Path(ns.source).resolve();start=time.monotonic();started=time.time();end=start+1800
if ns.deadline_epoch is not None:end=min(end,start+max(0,ns.deadline_epoch-started))
env=os.environ.copy();env['PATH']=str(Path.home()/'.cargo/bin')+os.pathsep+env.get('PATH','');rows=[]

def logical():
 roots=[Path('C:/file-native-incremental-20261018'),*[Path('C:/file-native-incremental-20261018'+s) for s in ['-build','-build-control','-build-final']]] if os.name=='nt' else list(root.parent.glob('file-native-incremental-*'))
 total=0
 for p in roots:
  if not p.is_dir():continue
  for directory,_,names in os.walk(p):
   for name in names:
    try:total+=(Path(directory)/name).stat().st_size
    except FileNotFoundError:pass
 return total

def command(name,args,cap,phase_end=None):
 begin=time.monotonic();stop_at=min(end,begin+cap,phase_end if phase_end is not None else end);reason=None
 if begin>=stop_at:rows.append(dict(name=name,unperformed='inherited deadline expired'));return False
 with (root/(name+'.stdout')).open('wb') as out,(root/(name+'.stderr')).open('wb') as err:
  p=subprocess.Popen([str(x) for x in args],cwd=source,env=env,stdout=out,stderr=err,start_new_session=os.name!='nt')
  while p.poll() is None:
   if time.monotonic()>stop_at:reason='deadline'
   elif shutil.disk_usage(root).free<ns.reserve_gib*1024**3:reason='free-reserve'
   elif logical()>(32 if os.name=='nt' else 6)*1024**3:reason='owned-allocation-cap'
   if reason:
    if os.name=='nt':subprocess.run(['taskkill','/PID',str(p.pid),'/T','/F'],capture_output=True,timeout=30)
    else:os.killpg(p.pid,signal.SIGKILL)
    p.wait();break
   time.sleep(2)
 row=dict(name=name,argv=[str(x) for x in args],exit=p.returncode,censored=bool(reason),reason=reason,seconds=time.monotonic()-begin);rows.append(row)
 (root/'outer.json').write_text(json.dumps(dict(pin=ns.pin,started=started,cap_seconds=1800,rows=rows),indent=2));return p.returncode==0 and not reason

target=Path(ns.target).resolve() if ns.target else root/'build';exe=target/'release'/('file-native-incremental.exe' if os.name=='nt' else 'file-native-incremental')
if command('build',['cargo','build','--release','--locked','--workspace','--target-dir',target],600) and command('tests',['cargo','test','--locked','--workspace','--target-dir',target],600):
 phase_end=min(end,time.monotonic()+900)
 for script,name in [('paired_intents.py','paired-intents'),('supplement.py','supplement'),('replay_lifetime.py','replay-lifetime')]:
  if not command(name,[sys.executable,source/script,'--exe',exe,'--base',root/name],900,phase_end):break
(root/'finished.json').write_text(json.dumps(dict(pin=ns.pin,started=started,cap_seconds=1800,elapsed_seconds=time.monotonic()-start,logical_owned_bytes=logical(),rows=rows),indent=2))
