"""Admitted d70 full54 + strict24, one bounded serialized qualification stage."""
import hashlib,json,os,subprocess,sys,time,datetime
from pathlib import Path
from guard import census,current

here=Path(__file__).resolve().parent;work=here.parents[2];r=Path('C:/file-native-packing-20261020');out=r/'fix2/small-qualification';out.mkdir(exist_ok=False)
q=json.loads((r/'receipts/artifact-reclaim-guard.json').read_bytes());assert q['exit']==0 and q['process_closure_verified'];actual=current()
assert actual['total_new_bytes']+104*1024**2<=2791728742 and actual['total_new_files']+16000<=560000
(out/'admission.json').write_text(json.dumps(dict(actual=actual,new_fixtures=100*1024**2,returns=4*1024**2,effective_total_ceiling=actual['total_new_bytes']+104*1024**2,overall_cap=2791728742),indent=2))
inventory=[json.loads(line) for line in (r/'artifact-preservation-xz/files.jsonl').read_bytes().splitlines()];kept=[];absent=0
for row in inventory:
    path=Path(row['absolute'])
    if row['retain_in_place']:
        assert path.stat().st_size==row['bytes'] and hashlib.sha256(path.read_bytes()).hexdigest()==row['sha256'];kept.append(dict(path=str(path),sha256=row['sha256'],bytes=row['bytes']))
    else:assert not path.exists();absent+=1
assert absent==566 and len(kept)==2
(r/'receipts/artifact-reclaim-postcheck.json').write_text(json.dumps(dict(absent_inventoried_files=absent,retained=kept,successfully_deleted_target_bytes=438731626,post_reclaim_actual=actual,empty_directories_retained=True),indent=2))
binding=json.loads((r/'fix2/build-binding.json').read_bytes());exe=r/'bin/candidate2.exe';assert hashlib.sha256(exe.read_bytes()).hexdigest()==binding['exe_sha256']
for name,row in binding['files'].items():assert hashlib.sha256((work/name).read_bytes()).hexdigest()==row['sha256']
sources={}
for name in ['final_small_qualification.py','capsule.py','strict_supplement.py','driver.py','guard.py','independent.py']:
    raw=(here/name).read_bytes();(out/name).write_bytes(raw);sources[name]=hashlib.sha256(raw).hexdigest()
env=dict(os.environ,FILE_NATIVE_P_ROOT=str(r),FILE_NATIVE_P_EXE=str(exe));full=r/'fix2/full-capsule';strict=r/'fix2/strict-supplement'
for name,command,seconds in [('full54',[sys.executable,str(here/'capsule.py'),'--base',str(full)],1800),('strict24',[sys.executable,str(here/'strict_supplement.py'),'--capsule-base',str(full),'--base',str(strict)],900)]:
    with (out/(name+'.stdout')).open('xb') as stdout,(out/(name+'.stderr')).open('xb') as stderr:
        child=subprocess.run(command,env=env,stdout=stdout,stderr=stderr,timeout=seconds)
    assert child.returncode==0,(name,child.returncode)
f=json.loads((full/'summary.json').read_bytes());s=json.loads((strict/'summary.json').read_bytes());assert f['status']==s['status']=='passed' and len(f['cases'])==54 and f['events']==275 and len(s['cases'])==24 and f['exe_sha256']==binding['exe_sha256']
fixtures={str(path):census(path) for path in [full,strict]};assert sum(row['bytes'] for row in fixtures.values())<=100*1024**2
for name,row in binding['files'].items():assert hashlib.sha256((work/name).read_bytes()).hexdigest()==row['sha256']
result=dict(status='passed',core_pin='d70f3864f2888d46d9112213b4a03f8d82745a8e',exe=str(exe),exe_sha256=binding['exe_sha256'],full_cases=54,full_events=275,strict_cases=24,strict_engine_events=s['engine_events'],source_unchanged=True,harness_sources=sources,fixtures=fixtures,workspace_five_tests_passed_before_target_archive=True,old_4cb_results_unchanged=True,old_recovery_capsule_final_source='separate future stage, not claimed here',large_PZ_and_cleanup_admitted=False)
(out/'summary.json').write_text(json.dumps(result,indent=2));assert census(out)['bytes']<=4*1024**2;print(json.dumps(result,indent=2))
