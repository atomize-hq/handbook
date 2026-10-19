"""Final namespace-recreation fix, within the original native deadline."""
import hashlib,json,os,shutil,sys
from pathlib import Path
from bounded import run
source=Path(__file__).resolve().parent;engine=source.parent/'file-native-incremental-engine-experiment-2026-10-18';root=Path(sys.argv[1]);pin=sys.argv[2];prior=json.loads((root/'finished.json').read_bytes());deadline=prior['rows'][0]['start_epoch']+1800;os.environ['PATH']=str(Path.home()/'.cargo/bin')+os.pathsep+os.environ.get('PATH','');exe=root/'build/release/file-native-incremental';shutil.copyfile(exe,root/'previous-core-binary');rows=[]
for name,args,cap,cwd in [
    ('build',['cargo','build','--locked','--offline','--release','--target-dir',root/'build'],120,engine),
    ('tests',['cargo','test','--locked','--offline','--target-dir',root/'build'],120,engine),
    ('capsule',[sys.executable,source/'capsule.py','--exe',exe,'--scratch',root/'final-core-small'],180,None),
    ('extra',[sys.executable,source/'focused_extra.py','--exe',exe,'--base',root/'final-core-extra'],120,None)]:
    row=run(args,root/('final-core-'+name),cap,deadline,cwd=cwd);rows.append(dict(name=name,**row))
    if row['status']!='success':break
result=dict(source_pin=pin,rows=rows,binary_sha256=hashlib.sha256(exe.read_bytes()).hexdigest(),source_sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in (engine/'src').glob('*.rs')},harness_sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [source/'capsule.py',source/'oracle.py',source/'operational_backup.py',source/'final_native.py']})
(root/'final-core-binding.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
