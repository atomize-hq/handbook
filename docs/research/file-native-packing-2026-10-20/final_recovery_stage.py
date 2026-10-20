"""Prepared final-source rerun of all original F cases, not a relabelled old pass."""
import hashlib,json
from pathlib import Path
from guard import census,current
from loose_regression import run

HERE=Path(__file__).resolve().parent;R=Path('C:/file-native-packing-20261020');OUT=R/'fix2/loose-regression-final';EXE=R/'bin/candidate2.exe'
assert not OUT.exists();actual=current();assert actual['total_new_bytes']+64*1024**2+32*1024**2<=2684354560 and actual['total_new_files']+12000<=560000 and actual['free_bytes']>=140*1024**3
OUT.mkdir();(OUT/'admission.json').write_text(json.dumps(dict(actual=actual,new_fixture_bound=64*1024**2,all_returns=32*1024**2,cap=2684354560),indent=2))
binding=json.loads((R/'fix2/build-binding.json').read_bytes());assert hashlib.sha256(EXE.read_bytes()).hexdigest()==binding['exe_sha256']
for path,row in binding['files'].items():assert hashlib.sha256(Path(path).read_bytes()).hexdigest()==row['sha256']
paths=[HERE/'final_recovery_stage.py',HERE/'loose_regression.py',HERE/'driver.py',HERE/'guard.py',HERE.parent/'file-native-recovery-reclamation-2026-10-19/capsule.py',HERE.parent/'file-native-recovery-reclamation-2026-10-19/operational_backup.py',HERE.parent/'file-native-navigation-2026-10-19/oracle.py'];sources={}
for index,path in enumerate(paths):
    raw=path.read_bytes();snapshot=OUT/(str(index)+'-'+path.name);snapshot.write_bytes(raw);sources[str(path)]=dict(sha256=hashlib.sha256(raw).hexdigest(),snapshot=str(snapshot))
result=run(EXE,OUT/'data');expected_cases=json.loads((R/'loose-regression/data/summary.json').read_bytes())['cases']
assert len(result['cases'])==24 and result['events']==282 and result['cases']==expected_cases and result['binary_sha256']==binding['exe_sha256']
assert [row['case'] for row in result['cases'] if row['status']=='expected refusal']==['impossible-final-namespace']
for path,row in binding['files'].items():assert hashlib.sha256(Path(path).read_bytes()).hexdigest()==row['sha256']
for path,row in sources.items():assert hashlib.sha256(Path(path).read_bytes()).hexdigest()==row['sha256']
size=census(OUT);assert size['bytes']<=64*1024**2
summary=dict(status='passed',core_pin='d70f3864f2888d46d9112213b4a03f8d82745a8e',binary_sha256=binding['exe_sha256'],cases=24,events=282,source_unchanged=True,sources=sources,fixture=size,old_4cb_run_preserved=True,actual=result)
(OUT/'final-summary.json').write_text(json.dumps(summary,indent=2));print(json.dumps(summary,indent=2))
