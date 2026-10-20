import os,sys,json,time,signal,subprocess,threading,hashlib,datetime,shutil
from pathlib import Path
OUT=Path(__file__).resolve().parent; ROOT=OUT.parents[3]; E=ROOT/'docs/research/file-native-incremental-engine-experiment-2026-10-18'; H=OUT.parent; R=OUT/'runtime'; EV=OUT/'evidence'
CUTOFF=datetime.datetime.fromisoformat('2026-10-19T23:44:38+00:00').timestamp()
env=dict(os.environ,TMPDIR=str(R),CARGO_TARGET_DIR=str(R/'build'),CARGO_PROFILE_DEV_DEBUG='0',CARGO_PROFILE_TEST_DEBUG='0',CARGO_INCREMENTAL='0')
rows=[]
def run(name,args,limit):
 remaining=CUTOFF-time.time()
 if remaining<3:
  rows.append(dict(name=name,status='not-started-original-cutoff',remaining=remaining));return False
 start=time.time();peak=None;why=None
 with (EV/(name+'.stdout')).open('wb') as stdout,(EV/(name+'.stderr')).open('wb') as stderr:
  p=subprocess.Popen(args,cwd=E,env=env,stdout=stdout,stderr=stderr,start_new_session=True)
  while p.poll() is None:
   if time.time()>=min(CUTOFF,start+limit):why='original-cutoff' if time.time()>=CUTOFF else 'outer-cap';os.killpg(p.pid,signal.SIGKILL);break
   snap=subprocess.run(['ps','-axo','pid=,ppid=,rss='],capture_output=True,text=True,timeout=2)
   processes=[tuple(map(int,x.split())) for x in snap.stdout.splitlines() if len(x.split())==3];ids={p.pid}
   for _ in range(12):ids|={a for a,b,c in processes if b in ids}
   values=[c*1024 for a,b,c in processes if a in ids]
   if values:peak=max(peak or 0,sum(values))
   if peak and peak>3*1024**3:why='aggregate-rss-censor';os.killpg(p.pid,signal.SIGKILL);break
   time.sleep(.08)
  code=p.wait()
 rows.append(dict(name=name,args=args,start_utc=datetime.datetime.fromtimestamp(start,datetime.timezone.utc).isoformat(),seconds=time.time()-start,exit=code,censor=why,aggregate_sampled_peak_rss_bytes=peak,missing_samples=peak is None,status='passed' if code==0 and why is None else 'failed-or-censored'))
 (EV/'commands.json').write_text(json.dumps(rows,indent=2));print(json.dumps(rows[-1]),flush=True)
 return code==0 and why is None
pins={str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in list((E/'src').glob('*'))+[E/'Cargo.toml',E/'Cargo.lock']+list(H.glob('*.py'))+[H/'legacy-fixtures.tar.gz']}
(EV/'source-pins.json').write_text(json.dumps(dict(source='4bde156323812ef18c0dea6798cfc2108aaad933',files=pins),indent=2))
if shutil.disk_usage(ROOT).free<20*1024**3:raise RuntimeError('free reserve')
if run('cargo-test',['cargo','test','--locked','--offline','--workspace','-j2'],120):
 if run('cargo-build',['cargo','build','--locked','--offline','-j2'],120):
  exe=R/'build/debug/file-native-incremental'
  (EV/'exe.json').write_text(json.dumps(dict(path=str(exe),sha256=hashlib.sha256(exe.read_bytes()).hexdigest(),build='native dev debug=0 incremental=0'),indent=2))
  run('focused',[sys.executable,str(H/'focused.py'),'--exe',str(exe),'--base',str(R/'focused'),'--capsule'],1800)
(EV/'commands.json').write_text(json.dumps(rows,indent=2))
