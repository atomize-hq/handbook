"""Parent-admitted engine-only cleanup of the exact primary disposable clone."""
import hashlib,json,os
from pathlib import Path
from driver import Driver
from guard import current
from independent import Decoder

R=Path('C:/file-native-packing-20261020');ROOT=R/'history-restored/disposable/store';EXE=R/'bin/candidate2.exe';OUT=R/'long-matrix-fixed2'
read=lambda p:json.loads(Path(p).read_bytes())
def hash_file(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
    return h.hexdigest()
def identities():
    assert hash_file(EXE)=='568f631acf528c7de911bb88ec85d07ead9d7b4435ebe970c61053f06e31a93b'
    for path,row in read(R/'fix2/build-binding.json')['files'].items():assert hash_file(path)==row['sha256'],path
    assert read(ROOT/'SELECTED.json')==read(R/'long-matrix/frozen.json')['selected']
    assert read(ROOT/'STORE.json')['catalog']=='84b1331e8d66e26c1f02074a9aa39269dd16446f519399ac70857bef1228dcd5'
    assert read(ROOT/'STORE.json')['mode']=='A'
identities()
for name in ['final-PZ-verify-after-lock','final-PZ-measure','final-PZ-allocation','final-PZ-metadata']:
    receipt=read(R/'receipts'/f'{name}.json');assert receipt['exit']==0 and receipt['censor'] is None and receipt['process_closure_verified'] and receipt['job_query_errors']==0,name
for path,h in [('post-gc-operational-backup/files.jsonl','fd5fc7c9085e00b9a6a7af6980c431a4c36386bed59bff245b1d55ce413899ff'),('post-gc-operational-backup/operational.bin.gz','c052a458169768cd0b17e6b4ec9f0daec1f7aabab23d552288f9dde82587e452')]:assert hash_file(R/path)==h
for row in read(OUT/'PZ-active-representation-allocation.json')['rows']:
    path=ROOT/row['path'];assert path.stat().st_size==row['bytes'] and hash_file(path)==row['sha256']
before=current();before.update(new_reservation_bytes=8*1024**2,prospective_bytes=before['total_new_bytes']+8*1024**2,cap=2766562918)
(R/'receipts/main-cleanup-fresh-admission.json').write_text(json.dumps(before,indent=2))
assert before['prospective_bytes']<=2766562918 and before['total_new_files']+100<=560000 and before['free_bytes']>=140*1024**3
selected=(ROOT/'SELECTED.json').read_bytes();store=(ROOT/'STORE.json').read_bytes();driver=Driver(OUT/'cleanup-events')
body=driver.call(ROOT,'packing',exe_override=EXE,timeout=1800,operation='cleanup')
(OUT/'cleanup-response.json').write_text(json.dumps(body,indent=2))
identities();assert selected==(ROOT/'SELECTED.json').read_bytes() and store==(ROOT/'STORE.json').read_bytes()
d=Decoder(ROOT);remaining=[key for key in d.entries if (ROOT/key).exists()]
old=read(R/'long-matrix/P0-build.json')['catalog']['packs'];old_remaining=[]
for p in old:
    for key in ['packs/'+p['pack']+'.pack','packs/'+p['index']+'.idx']:
        if (ROOT/key).exists():old_remaining.append(key)
result=dict(response=body,mapped_records=len(d.entries),remaining_mapped_loose_files=remaining,remaining_old_P0_containers=old_remaining,active_representation_files=33,eligible_class_files_after=len(remaining)+33,eligible_loose_class_files_before=504833,eligible_file_reduction=1-(len(remaining)+33)/504833,source_binary_selected_store_unchanged=True,actual_successfully_deleted_bytes=body['result']['successfully_deleted_bytes_recorded'],deferred_bytes=body['result']['deferred_bytes'],protected_exact_post_GC_capture_hashes_checked=True,complete_independent_postcleanup_verification_pending=True)
(OUT/'cleanup-summary.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
if body['result']['status']=='complete':
    assert body['result']['deferred_bytes']==0 and not remaining and not old_remaining and result['actual_successfully_deleted_bytes']==1221001465
