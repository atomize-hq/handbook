import hashlib,json,sys,time
from pathlib import Path
from guard import current,census
root=Path('C:/file-native-recovery-20261019/history-restored/disposable/store')
start=time.time();counts=census(root)
assert counts==dict(bytes=887923682,files=510583),counts
bindings={}
for name in ['SELECTED.json','STORE.json','HISTORY-SOURCE.json','expected-ledger.jsonl','RESERVATIONS.json','PINS.json','VIEW.json']:
    p=root/name
    if not p.exists():continue
    h=hashlib.sha256()
    with p.open('rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
    bindings[name]=dict(bytes=p.stat().st_size,sha256=h.hexdigest())
s=json.loads((root/'SELECTED.json').read_bytes())
assert s==dict(publication='publication-49204-1792429021747802100-16',digest='8b816596942b684e53a3f7366876976f73450280581ff4714472be26d9595169')
report=dict(request='parent admission for exact post-GC capture, one fresh restore, independent complete verification and reopen, fresh disposable setup; no long packing cleanup yet',source=str(root),source_census=counts,source_bindings=bindings,current_p_ownership=current(),proposed_total_p_cap_bytes=2415919104,proposed_total_p_cap_files=560000,
prospective=dict(existing_initial_allocation=536870912,capture_transport_plus_inventory_cap=750000000,fresh_restore_cap=887923682,stage_metadata_cap=16777216,total_bytes=536870912+750000000+887923682+16777216,files=25000+510583+2000),
destinations=['C:/file-native-packing-20261020/post-gc-operational-backup','C:/file-native-packing-20261020/history-restored/disposable/store'],
stage_caps=dict(capture_seconds=1800,restore_seconds=1800,complete_independent_verifier_seconds=900,ordinary_seconds=120,checker_rss_bytes=2147483648,aggregate_rss_bytes=4294967296),
functional_cutoff='2026-10-20T07:55:16Z',closeout_deadline='2026-10-20T08:25:16Z',no_scope_or_resource_grant_inferred=True,seconds=time.time()-start)
out=Path('C:/file-native-packing-20261020/receipts/preservation-stage-request.json');out.write_text(json.dumps(report,indent=2));print(json.dumps(report,indent=2))
