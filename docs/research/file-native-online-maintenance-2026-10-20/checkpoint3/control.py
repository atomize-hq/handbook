import importlib.util,sys,json,hashlib,os
from pathlib import Path
root=Path('C:/Users/spmcc/.codex/worktrees/file-native-online-cp2-20261020/handbook')
e=Path('C:/file-native-online-cp1-20261020/evidence')
g=Path('C:/Users/spmcc/.codex/worktrees/file-native-online-matrix-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/harness/job_guard.py')
assert hashlib.sha256(g.read_bytes()).hexdigest()=='1e4a965139cac5b48e6cbef57d0ac8cc3c3d0294f36a392383308abcee3affae'
budget=e/'cp3-budget.json'
assert hashlib.sha256(budget.read_bytes()).hexdigest()=='9c9c491a7c77c5dbd36a9d20afdc6bd421525c882b878511ded33de765a36e30'
spec=importlib.util.spec_from_file_location('guard',str(g));gu=importlib.util.module_from_spec(spec);spec.loader.exec_module(gu)
name=sys.argv[1]
os.environ['EXP_PACK_TEST_ROOT']='C:/file-native-online-cp1-20261020/fixtures/cp3/unit'
if name.startswith('build'):
 source={str(p.relative_to(root)).replace('\\','/'):{'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in (root/'docs/research/file-native-incremental-engine-experiment-2026-10-18').rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name in ['Cargo.toml','Cargo.lock'])}
 (e/('cp3-'+name+'-input.json')).write_text(json.dumps(dict(source_files=source,budget_sha256=hashlib.sha256(budget.read_bytes()).hexdigest(),binary_before=hashlib.sha256((Path('C:/file-native-online-cp1-20261020/build/release/file-native-incremental.exe')).read_bytes()).hexdigest()),indent=2)+'\n')
 common=['--locked','--offline','--workspace','--release','-j2','--target-dir','C:/file-native-online-cp1-20261020/build']
 skipped=['--skip','loading_indexes_evicts_before_transient_allocation']
 if name=='build9':skipped+=['--skip','paged_four_routes_negative_duplicates_and_conflict','--skip','paged_positive_negative_boundary_and_cache']
 code="import subprocess,sys;common="+repr(common)+";skipped="+repr(skipped)+";cargo='C:/Users/spmcc/.cargo/bin/cargo.exe';a=subprocess.run([cargo,'build',*common]);sys.exit(a.returncode) if a.returncode else None;b=subprocess.run([cargo,'test',*common,'--',*skipped]);sys.exit(b.returncode)"
 argv=[sys.executable,'-B','-c',code];cwd=root/'docs/research/file-native-incremental-engine-experiment-2026-10-18';reserves={'build':{'bytes':50331648,'files':150},'fixtures':{'bytes':4194304,'files':100},'evidence':{'bytes':1048576,'files':25}};seconds=240
 if name=='build9':reserves['fixtures']={'bytes':524288,'files':150};reserves['evidence']={'bytes':524288,'files':25}
else:
 script=e/('cp3-smoke.py' if name.startswith('smoke') else 'cp3-remediation.py' if name.startswith('remediation') else 'cp3-'+name+'.py')
 argv=[sys.executable,'-B',str(script),name];cwd=root;reserves={'fixtures':{'bytes':8388608,'files':2500},'evidence':{'bytes':2097152,'files':100}};seconds=600
 if name.startswith('remediation-green'):reserves={'fixtures':{'bytes':1048576,'files':500},'evidence':{'bytes':786432,'files':40}}
r=gu.run(argv,str(e/('cp3-'+name+'.json')),str(budget),seconds,reserves,cwd=str(cwd));print(json.dumps({k:r.get(k) for k in ['exit','errors','censor','process_closure_verified','job_aggregate_sampled_peak_rss']}))
