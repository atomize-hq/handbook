"""Check committed evidence bytes and count native/local/Git additional scope."""
import json,hashlib,subprocess,sys,time
from pathlib import Path
BASE=Path(__file__).resolve().parent
ROOT=BASE.parents[2]
sys.path.insert(0,str(ROOT/'docs/research/file-native-navigation-2026-10-19'))
from census import census
inventory=json.loads((BASE/'evidence/native-raw/return-inventory.json').read_bytes())
for path,row in inventory.items():
    repo_path=(BASE/'evidence/native-raw'/path).relative_to(ROOT).as_posix()
    raw=subprocess.check_output(['git','cat-file','blob','HEAD:'+repo_path],cwd=ROOT)
    assert len(raw)==row['bytes'] and hashlib.sha256(raw).hexdigest()==row['sha256']
common=Path(subprocess.check_output(['git','rev-parse','--git-common-dir'],cwd=ROOT).decode().strip())
git=census(common);local=census(ROOT);native=json.loads((BASE/'evidence/final-native-snapshot.json').read_bytes())
assert native['harness_sha256']==hashlib.sha256((BASE/'final_native_snapshot.py').read_bytes()).hexdigest()
initial=json.loads((BASE/'evidence/local-before.json').read_bytes())['roots'][1]
growth=max(0,git['logical_bytes']-initial['logical_bytes']);files=max(0,git['files']-initial['files'])
total=native['native_owned']['logical_bytes']+native['new_test_temporary']['logical_bytes']+local['logical_bytes']+growth
count=native['native_owned']['files']+native['new_test_temporary']['files']+local['files']+files
assert total+16*1024**2<768*1024**2 and count+1000<100000
scope=subprocess.check_output(['git','diff','--name-only','6e66a7523612b80386cf9cc2b96d5341579b54fd','HEAD'],cwd=ROOT).decode().splitlines()
assert all(p.startswith('docs/research/file-native-n-linux-2026-10-19/') for p in scope)
row=dict(epoch=time.time(),report_commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT).decode().strip(),candidate='6e66a7523612b80386cf9cc2b96d5341579b54fd',committed_raw_members_byte_verified=len(inventory),scope_only_new_qualification_directory=True,local_worktree=local,shared_git=git,shared_git_initial=initial,conservative_entire_shared_git_growth_bytes=growth,conservative_entire_shared_git_growth_files=files,native=native,combined_additional_logical_bytes=total,combined_additional_files=count,lane_cap_bytes=768*1024**2,lane_cap_files=100000,extra_final_receipt_git_allowance_bytes=16*1024**2,extra_final_receipt_git_allowance_files=1000,continuous_full_lane_peak='missing; this is a closing census, prior stage samples/caps retained')
(BASE/'evidence/final-combined-accounting.json').write_text(json.dumps(row,indent=2))
print(json.dumps(row,indent=2))
