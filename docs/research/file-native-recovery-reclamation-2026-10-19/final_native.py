"""Final namespace-recreation fix, within the original native deadline."""
import hashlib,json,os,shutil,sys
from pathlib import Path
from bounded import run
source=Path(__file__).resolve().parent;engine=source.parent/'file-native-incremental-engine-experiment-2026-10-18';root=Path(sys.argv[1]);pin=sys.argv[2];label=sys.argv[3] if len(sys.argv)>3 else 'final-core';prior=json.loads((root/'finished.json').read_bytes());deadline=prior['rows'][0]['start_epoch']+1800;os.environ['PATH']=str(Path.home()/'.cargo/bin')+os.pathsep+os.environ.get('PATH','');exe=root/'build/release/file-native-incremental';shutil.copyfile(exe,root/(label+'-previous-core-binary'));rows=[]
for name,args,cap,cwd in [
    ('build',['cargo','build','--locked','--offline','--release','--target-dir',root/'build'],30,engine),
    ('tests',['cargo','test','--locked','--offline','--target-dir',root/'build'],30,engine),
    ('capsule',[sys.executable,source/'capsule.py','--exe',exe,'--scratch',root/(label+'-small')],100,None),
    ('extra',[sys.executable,source/'focused_extra.py','--exe',exe,'--base',root/(label+'-extra')],30,None)]:
    row=run(args,root/(label+'-'+name),cap,deadline,cwd=cwd);rows.append(dict(name=name,**row))
    if row['status']!='success':break
if len(sys.argv)>4 and all(r['status']=='success' for r in rows):
    row=run([sys.executable,source/'legacy_compatibility.py','--exe',exe,'--fixtures',sys.argv[4],'--base',root/(label+'-legacy')],root/(label+'-legacy'),10,deadline);rows.append(dict(name='legacy',**row))
result=dict(source_pin=pin,rows=rows,binary_sha256=hashlib.sha256(exe.read_bytes()).hexdigest(),source_sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in (engine/'src').glob('*.rs')},harness_sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [source/'capsule.py',source/'oracle.py',source/'operational_backup.py',source/'final_native.py',source/'legacy_compatibility.py']})
(root/(label+'-binding.json')).write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
