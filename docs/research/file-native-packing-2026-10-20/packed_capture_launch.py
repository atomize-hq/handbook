"""Parent's capture-only admission with final exact identities and closed groups."""
import hashlib,json
from pathlib import Path
from packed_backup_stage import run,R,ROOT,HERE,sha_file
from postcleanup_check import stable

stable()
assert sha_file(HERE/'independent.py')=='94987dbe1c992ef2491bb9819ad90185d2f29aef704b1bfaaa8f7e052a449e66'
for name in ['main-cleanup','postcleanup-verify','postcleanup-reopen','postcleanup-metadata']:
    row=json.loads((R/'receipts'/f'{name}.json').read_bytes());assert row['exit']==0 and row['censor'] is None and row['process_closure_verified'] and row['job_query_errors']==0
for path,h in [('post-gc-operational-backup/files.jsonl','fd5fc7c9085e00b9a6a7af6980c431a4c36386bed59bff245b1d55ce413899ff'),('post-gc-operational-backup/operational.bin.gz','c052a458169768cd0b17e6b4ec9f0daec1f7aabab23d552288f9dde82587e452')]:assert sha_file(R/path)==h
before={name:(ROOT/name).read_bytes() for name in ['SELECTED.json','STORE.json','PACKING.json']}
binding=dict(cap_bytes=2415919104,parent_admission='capture only; no restore',source_core='d70f3864f2888d46d9112213b4a03f8d82745a8e',oracle_sha256=sha_file(HERE/'independent.py'),wrapper_sha256=sha_file(HERE/'packed_backup_stage.py'),launcher_sha256=sha_file(__file__),source_controls={name:hashlib.sha256(raw).hexdigest() for name,raw in before.items()},source_groups_closed=True,quiescent_capture=True,old_wrapper_source_preserved_in_commit='1543ccf44f7c741c39cb1f02679af6bdfeb807be')
(R/'receipts/packed-capture-launch-binding.json').write_text(json.dumps(binding,indent=2))
run('capture')
stable()
for name,raw in before.items():assert (ROOT/name).read_bytes()==raw
result=json.loads((R/'receipts/packed-capture-result.json').read_bytes());print('capture completed; controls unchanged; separate restore admission remains required')
