"""Parent-reviewed restore-only stage; creates no local disposable authority."""
import json
from packed_backup_stage import run,R,HERE,ROOT,sha_file
from postcleanup_check import stable

stable();assert sha_file(HERE/'independent.py')=='94987dbe1c992ef2491bb9819ad90185d2f29aef704b1bfaaa8f7e052a449e66'
receipt=json.loads((R/'receipts/packed-capture.json').read_bytes());assert receipt['exit']==0 and receipt['censor'] is None and receipt['process_closure_verified'] and receipt['job_query_errors']==0
capsule=R/'packed-operational-backup'
for name,h in [('operational.bin.gz','2fabd1c2439fc95a37a0411c99025bfb5646dee855b6ed0e879fd190e35e324b'),('files.jsonl','0974f4a1dac1f13618105b9fca425b2a56c99528accecb5255f0d4ec6515b9b5'),('manifest.json','3b8f1b875a6b567cb386be34f4b3f7442f9e3538d8a58fbac524cf1473777281')]:assert sha_file(capsule/name)==h
before={name:(ROOT/name).read_bytes() for name in ['SELECTED.json','STORE.json','PACKING.json']}
binding=dict(cap=2415919104,parent_independent_transport_proof='C:/file-native-qnp-20261019/packed-capture-parent-verify.txt',wrapper_sha256=sha_file(HERE/'packed_backup_stage.py'),launcher_sha256=sha_file(__file__),core='d70f3864f2888d46d9112213b4a03f8d82745a8e',oracle_sha256=sha_file(HERE/'independent.py'),successor_admitted=False,disposable_authority_admitted=False)
(R/'receipts/packed-restore-launch-binding.json').write_text(json.dumps(binding,indent=2));run('restore');stable()
for name,raw in before.items():assert (ROOT/name).read_bytes()==raw
