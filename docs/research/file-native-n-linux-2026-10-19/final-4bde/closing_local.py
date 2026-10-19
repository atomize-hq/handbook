"""Combine all continuing-lane native, local, temporary and shared-Git lengths."""
import json,hashlib,subprocess,sys,time
from pathlib import Path
BASE=Path(__file__).resolve().parent;ROOT=BASE.parents[3]
sys.path.insert(0,str(ROOT/'docs/research/file-native-navigation-2026-10-19'))
from census import census
native=json.loads((BASE/'closing-native.json').read_bytes());assert native['harness_sha256']==hashlib.sha256((BASE/'closing_native.py').read_bytes()).hexdigest()
common=Path(subprocess.check_output(['git','rev-parse','--git-common-dir'],cwd=ROOT).decode().strip())
local=census(ROOT);git=census(common);initial=json.loads((BASE.parent/'evidence/local-before.json').read_bytes())['roots'][1]
growth=max(0,git['logical_bytes']-initial['logical_bytes']);files=max(0,git['files']-initial['files'])
total=native['native_owned']['logical_bytes']+native['new_test_temporary']['logical_bytes']+local['logical_bytes']+growth+native['shared_cargo_logical_growth_bytes']
count=native['native_owned']['files']+native['new_test_temporary']['files']+local['files']+files
assert total+16*1024**2<768*1024**2 and count+1000<100000
row=dict(epoch=time.time(),candidate=native['candidate'],original_lane_start='2026-10-19T22:54:45Z',original_execution_cutoff='2026-10-19T23:44:45Z',original_lane_deadline='2026-10-19T23:54:45Z',local_worktree=local,shared_git=git,shared_git_initial=initial,conservative_entire_shared_git_growth_bytes=growth,conservative_entire_shared_git_growth_files=files,native=native,combined_additional_logical_bytes=total,combined_additional_files=count,lane_cap_bytes=768*1024**2,lane_cap_files=100000,additional_closing_git_receipt_allowance_bytes=16*1024**2,additional_closing_git_receipt_allowance_files=1000)
(BASE/'closing-combined.json').write_text(json.dumps(row,indent=2));print(json.dumps({k:row[k] for k in ['candidate','combined_additional_logical_bytes','combined_additional_files','conservative_entire_shared_git_growth_bytes']},indent=2))
