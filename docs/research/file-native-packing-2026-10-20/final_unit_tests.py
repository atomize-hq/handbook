"""Five locked workspace tests on d70, with explicit prospective admission."""
import datetime,hashlib,json,os,shutil,subprocess,time
from pathlib import Path

here=Path(__file__).resolve().parent;work=here.parents[2];r=Path('C:/file-native-packing-20261020');out=r/'fix2/unit-tests';out.mkdir(exist_ok=False);(out/'harness-source.py').write_bytes(Path(__file__).read_bytes())
receipt=json.loads((r/'receipts/fixed-P0-readonly.json').read_bytes());actual=receipt['final_ownership'];assert receipt['exit']==0 and receipt['process_closure_verified'] and time.time()-datetime.datetime.fromisoformat(actual['time_utc']).timestamp()<240,'fresh-admission-expired'
prospective=actual['total_new_bytes']+83886080+8388608+2097152+58720256;assert prospective<=2791728742 and actual['total_new_files']+2000<=560000 and actual['free_bytes']>=140*1024**3
(out/'prospective-admission.json').write_text(json.dumps(dict(actual=actual,prospective_bytes=prospective,cap=2791728742,unit_fixture=83886080,compile_and_testexe=8388608,reports_inventory=2097152,new_XZ=58720256,old_failed_gzip_retained=True),indent=2))
binding=json.loads((r/'fix2/build-binding.json').read_bytes())
for name,row in binding['files'].items():assert hashlib.sha256((work/name).read_bytes()).hexdigest()==row['sha256']
environ={k:v for k,v in os.environ.items() if not k.startswith('EXP_')};environ['RUSTUP_AUTO_INSTALL']='0'
versions={name:subprocess.check_output([name,'--version'],env=environ,timeout=10).decode().strip() for name in ['rustc','cargo']}
command=['cargo','test','--locked','--offline','--workspace','--release','-j2','--target-dir',str(r/'build'),'--','--nocapture'];child=subprocess.run(command,cwd=here.parent/'file-native-incremental-engine-experiment-2026-10-18',env=environ,capture_output=True,timeout=120)
(out/'cargo.stdout').write_bytes(child.stdout);(out/'cargo.stderr').write_bytes(child.stderr);assert child.returncode==0 and b'5 passed; 0 failed' in child.stdout
for name,row in binding['files'].items():assert hashlib.sha256((work/name).read_bytes()).hexdigest()==row['sha256']
tests=[]
for path in (r/'build/release/deps').glob('file_native_incremental-*.exe'):
    listing=subprocess.run([str(path),'--list'],capture_output=True,timeout=10);(out/(path.stem+'-list.stdout')).write_bytes(listing.stdout)
    if listing.returncode==0 and len([line for line in listing.stdout.splitlines() if line.endswith(b': test')])==5:tests.append(path)
assert len(tests)==1;frozen=r/'bin/fix2-unit-tests.exe';assert not frozen.exists();shutil.copyfile(tests[0],frozen)
fixtures=[]
for path in (here.parent/'file-native-incremental-engine-experiment-2026-10-18/target').glob('packing-cache-boundary-*'):
    files=[p for p in path.rglob('*') if p.is_file()];fixtures.append(dict(path=str(path),files=len(files),bytes=sum(p.stat().st_size for p in files)))
assert hashlib.sha256((r/'bin/candidate2.exe').read_bytes()).hexdigest()==binding['exe_sha256'] and hashlib.sha256((r/'bin/candidate1.exe').read_bytes()).hexdigest()=='478dd9ba9f687d4de81c60ce55f9ddf9e4e2fafcc493d581b3c63e12e42f7fe5'
result=dict(status='passed',core_pin='d70f3864f2888d46d9112213b4a03f8d82745a8e',command=command,versions=versions,meaningful_workspace_tests=5,source_unchanged=True,files=binding['files'],frozen_test_exe=str(frozen),frozen_test_exe_sha256=hashlib.sha256(frozen.read_bytes()).hexdigest(),all_cache_fixtures_preserved=fixtures,engine_binary_sha256=binding['exe_sha256']);(out/'summary.json').write_text(json.dumps(result,indent=2));print(json.dumps({k:v for k,v in result.items() if k!='files'},indent=2))
