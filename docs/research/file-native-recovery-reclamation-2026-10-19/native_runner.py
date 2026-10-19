"""One finite changed-path lane on an already authorized native host."""
import argparse,hashlib,json,os,platform,shutil,sys,time
from pathlib import Path
from bounded import run

a=argparse.ArgumentParser();a.add_argument('--source',required=True);a.add_argument('--root',required=True);a.add_argument('--pin',required=True);a.add_argument('--reserve-gib',type=int,default=20);ns=a.parse_args()
source=Path(ns.source).resolve();root=Path(ns.root).resolve();root.mkdir(parents=True,exist_ok=False);start=time.time();deadline=start+1800;rows=[]
engine=source.parent/'file-native-incremental-engine-experiment-2026-10-18';os.environ['PATH']=str(Path.home()/'.cargo/bin')+os.pathsep+os.environ.get('PATH','');target=root/'build';exe=target/'release/file-native-incremental'
def command(name,args,cap,cwd=None):
    if shutil.disk_usage(root).free<ns.reserve_gib*1024**3:raise RuntimeError('free-reserve')
    row=run(args,root/name,cap,deadline,cwd=cwd);rows.append(dict(name=name,**row));return row['status']=='success'
try:
    if command('build',['cargo','build','--locked','--offline','--release','--target-dir',target],300,engine) and command('tests',['cargo','test','--locked','--offline','--target-dir',target],300,engine):
        for name,args,cap in [
            ('capsule',[sys.executable,source/'capsule.py','--exe',exe,'--scratch',root/'small'],300),
            ('extra',[sys.executable,source/'focused_extra.py','--exe',exe,'--base',root/'extra-data'],300),
            ('paired',[sys.executable,engine/'paired_intents.py','--exe',exe,'--base',root/'paired-data'],180),
            ('replay',[sys.executable,engine/'replay_lifetime.py','--exe',exe,'--base',root/'replay-data'],300),
            ('protocol',[sys.executable,engine/'checks.py','--exe',exe,'--base',root/'checks-data','--faults'],300)]:
            if not command(name,args,cap):break
finally:
    result=dict(pin=ns.pin,platform=platform.platform(),rows=rows,seconds=time.time()-start,free_bytes=shutil.disk_usage(root).free,binary_sha256=hashlib.sha256(exe.read_bytes()).hexdigest() if exe.exists() else None,source_sha256={str(p.relative_to(source.parent)):hashlib.sha256(p.read_bytes()).hexdigest() for base in [source,engine/'src'] for p in base.rglob('*') if p.is_file() and '__pycache__' not in p.parts and 'evidence' not in p.parts})
    (root/'finished.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
