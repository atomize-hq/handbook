"""Explicit 1800-second correction lane, separate from prior native windows."""
import argparse,json,os,shutil,signal,subprocess,sys,time
from pathlib import Path
a=argparse.ArgumentParser();a.add_argument('--root',required=True);a.add_argument('--source',required=True);a.add_argument('--pin',required=True);a.add_argument('--reserve-gib',type=int,default=20);a.add_argument('--deadline-epoch',type=float);ns=a.parse_args()
root=Path(ns.root);root.mkdir(exist_ok=False);source=Path(ns.source);start=time.monotonic();started=time.time();end=start+1800;rows=[]
if ns.deadline_epoch is not None:end=min(end,start+max(0,ns.deadline_epoch-started))
env=os.environ.copy();env['PATH']=str(Path.home()/'.cargo/bin')+os.pathsep+env.get('PATH','')
def logical():
    return sum((Path(d)/n).stat().st_size for p in Path.home().glob('file-native-incremental-*') if p.is_dir() for d,_,names in os.walk(p) for n in names if (Path(d)/n).exists())
def command(name,args,cap):
    begin=time.monotonic();reason=None
    if begin>=end:
        rows.append(dict(name=name,argv=args,unperformed='inherited deadline expired'));return False
    with (root/(name+'.stdout')).open('wb') as out,(root/(name+'.stderr')).open('wb') as err:
        p=subprocess.Popen(args,cwd=source,env=env,stdout=out,stderr=err,start_new_session=True)
        while p.poll() is None:
            if time.monotonic()>min(end,begin+cap):reason='deadline'
            elif shutil.disk_usage(root).free<ns.reserve_gib*1024**3:reason='free-reserve'
            elif logical()>6*1024**3:reason='combined-owned-cap'
            if reason:os.killpg(p.pid,signal.SIGKILL);p.wait();break
            time.sleep(2)
    rows.append(dict(name=name,argv=args,exit=p.returncode,censored=bool(reason),reason=reason,seconds=time.monotonic()-begin))
    (root/'outer.json').write_text(json.dumps(dict(pin=ns.pin,started=started,cap_seconds=1800,rows=rows),indent=2))
    return p.returncode==0 and not reason
target=str(root/'build')
if command('tests',['cargo','test','--locked','--workspace','--target-dir',target],600) and command('build',['cargo','build','--locked','--release','--workspace','--target-dir',target],600):
    command('audit-evidence',[sys.executable,str(source/'audit_evidence.py'),'--exe',str(root/'build/release/file-native-incremental'),'--base',str(root/'audit-evidence'),'--strict'],900)
(root/'finished.json').write_text(json.dumps(dict(pin=ns.pin,started=started,cap_seconds=1800,elapsed_seconds=time.monotonic()-start,logical_owned_bytes=logical(),rows=rows),indent=2))
