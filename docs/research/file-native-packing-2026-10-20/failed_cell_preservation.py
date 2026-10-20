"""Losslessly preserve the failed input prefix; never calls it a completed cell."""
import json,subprocess,sys
from pathlib import Path
from guard import current
from packed_backup_stage import adapted,sha_file,R,HERE

case=R/'operational-matrix/n8000-A';root=case/'L-old/disposable/store';destination=case/'L-old-operational-backup';out=R/'operational-matrix/stage-evidence/n8000-A-L-old'
receipt=json.loads((R/'receipts/matrix-n8000-A-L-old.json').read_bytes());assert receipt['exit']==1 and receipt['process_closure_verified']
actual=current();assert actual['total_new_bytes']+67108864+33554432<=2684354560 and actual['free_bytes']>=140*1024**3
backup,quota,_=adapted('failed-A-L-old-preservation');quota.limit=67108864;backup.MAX_FILES=50000
before={name:(root/name).read_bytes() for name in ['SELECTED.json','STORE.json']}
manifest=backup.capture(root,destination,'4bde156323812ef18c0dea6798cfc2108aaad933',True)
checker=out/'independent-archive-checker.py';checked=subprocess.run([sys.executable,str(checker),str(destination),str(root),manifest['transport_sha256'],manifest['inventory_sha256'],str(manifest['files']),str(manifest['logical_bytes'])],capture_output=True,timeout=115)
(out/'failed-prefix-archive-check.stdout').write_bytes(checked.stdout);(out/'failed-prefix-archive-check.stderr').write_bytes(checked.stderr);assert checked.returncode==0
for name,raw in before.items():assert (root/name).read_bytes()==raw
result=dict(status='FAILED_INPUT_PREFIX_LOSSLESSLY_PRESERVED; not a passed seven-operation cell',manifest=manifest,archive_files={p.name:dict(bytes=p.stat().st_size,sha256=sha_file(p)) for p in destination.iterdir() if p.is_file()},worker_independent_parser=json.loads(checked.stdout),source_unchanged=True,failed_cell_root=str(root),new_archive=str(destination),accepted_measured_publications_before_refusal=5,no_rewind_no_further_publication=True,wrapper_sha256=sha_file(__file__),quota_bytes=quota.used)
(out/'failure-preservation.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
