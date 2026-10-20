"""Parent-conditional capture of the verified successor, with 320MB prewrite cap."""
import hashlib,json
from pathlib import Path
from guard import current
from packed_backup_stage import adapted,R,HERE,RESTORED,sha_file

read=lambda p:json.loads(Path(p).read_bytes())
EXE=R/'bin/candidate2.exe';DEST=R/'packed-successor-operational-backup'
for name in ['packed-successor','packed-successor-verify','primary-reclaim-exec-guard']:
    row=read(R/'receipts'/f'{name}.json');assert row['exit']==0 and row['censor'] is None and row['process_closure_verified'] and row['job_query_errors']==0
reclaimed=read(R/'receipts/primary-reclaim-exec.json');assert reclaimed['removed_files']==5807 and reclaimed['removed_bytes']==592458971
assert sha_file(EXE)=='568f631acf528c7de911bb88ec85d07ead9d7b4435ebe970c61053f06e31a93b' and sha_file(HERE/'independent.py')=='94987dbe1c992ef2491bb9819ad90185d2f29aef704b1bfaaa8f7e052a449e66'
source=read(R/'fix2/build-binding.json')['files']
for path,row in source.items():assert sha_file(path)==row['sha256']
verified=read(R/'packed-restored-qualification/verify-successor-independent.json');assert verified['semantic']['publications']==101008 and verified['raw_records']==504833 and verified['semantic']['ledger_rows']==100000 and verified['semantic']['retired_publications']==101
assert read(RESTORED/'SELECTED.json')==verified['semantic']['selected']
before={name:(RESTORED/name).read_bytes() for name in ['SELECTED.json','STORE.json','PACKING.json']}
actual=current();assert actual['total_new_bytes']+320000000+33554432<=2684354560 and actual['total_new_files']+12010<=560000 and actual['free_bytes']>=140*1024**3
(R/'receipts/successor-capture-fresh-admission.json').write_text(json.dumps(dict(actual=actual,transport_inventory_prewrite_cap=320000000,all_stage_margin=33554432,cap=2684354560),indent=2))
backup,quota,_=adapted('successor-capture');quota.limit=320000000
effective=dict(capture_source=str(RESTORED),new_destination=str(DEST),effective_prewrite_quota=quota.limit,source_file_bytes_limit=backup.MAX_BYTES,source_file_count_limit=backup.MAX_FILES,quota_override_before_any_archive_write=True,prior_generic_adapter_declared_limit=620000000,wrapper_sha256=sha_file(__file__),generic_wrapper_sha256=sha_file(HERE/'packed_backup_stage.py'),selected=verified['semantic']['selected'],source_core='d70f3864f2888d46d9112213b4a03f8d82745a8e',oracle_sha256=sha_file(HERE/'independent.py'))
(R/'packed-backup-inputs/successor-capture-effective-binding.json').write_text(json.dumps(effective,indent=2))
result=backup.capture(RESTORED,DEST,'d70f3864f2888d46d9112213b4a03f8d82745a8e',True);assert result['logical_bytes']<=600000000 and result['files']<=12000 and quota.used<=320000000
for path,row in source.items():assert sha_file(path)==row['sha256']
for name,raw in before.items():assert (RESTORED/name).read_bytes()==raw
result.update(transport_inventory_actual_bytes=quota.used,archive_files={p.name:dict(bytes=p.stat().st_size,sha256=sha_file(p)) for p in DEST.iterdir() if p.is_file()},source_controls_unchanged=True,parent_independent_transport_verification_pending=True)
(R/'receipts/successor-capture-result.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
