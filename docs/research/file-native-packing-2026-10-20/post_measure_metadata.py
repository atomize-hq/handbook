"""Read-only allocation/table closeout and concrete prospective next-stage census."""
import hashlib,json,runpy,time
from pathlib import Path
from allocation import allocation

HERE=Path(__file__).resolve().parent
R=Path('C:/file-native-packing-20261020');ROOT=R/'history-restored/disposable/store'
runpy.run_path(str(HERE/'final_tables.py'),run_name='__main__')
store=(ROOT/'STORE.json').read_bytes();selected=(ROOT/'SELECTED.json').read_bytes()
catalog_path=ROOT/('pack-catalogs/'+json.loads(store)['catalog']+'.json')
catalog=json.loads(catalog_path.read_bytes());paths=[catalog_path]
for pack in catalog['packs']:paths.extend([ROOT/('packs/'+pack['pack']+'.pack'),ROOT/('packs/'+pack['index']+'.idx')])
rows=[]
for path in paths:
    allocated,eof=allocation(path);rows.append(dict(path=path.relative_to(ROOT).as_posix(),bytes=eof,allocation_bytes=allocated,sha256=hashlib.sha256(path.read_bytes()).hexdigest()))
result=dict(files=len(rows),logical_bytes=sum(x['bytes'] for x in rows),allocation_bytes=sum(x['allocation_bytes'] for x in rows),rows=rows,api='GetFileInformationByHandleEx(FileStandardInfo=1).AllocationSize',directory_MFT_metadata=None,physical_device_writes=None)
(R/'long-matrix-fixed2/PZ-active-representation-allocation.json').write_text(json.dumps(result,indent=2))
assert store==(ROOT/'STORE.json').read_bytes() and selected==(ROOT/'SELECTED.json').read_bytes()
whole=json.loads((R/'long-matrix-fixed2/PZ-allocation.json').read_bytes())
proposal_path=R/'receipts/next-large-stage-proposal.json';p=json.loads(proposal_path.read_bytes())
closed=json.loads((R/'receipts/final-PZ-allocation.json').read_bytes())
assert closed['exit']==0 and closed['process_closure_verified'] and closed['censor'] is None
p['last_closed_ownership_bytes']=closed['final_ownership']['total_new_bytes'];p['last_closed_ownership_files']=closed['final_ownership']['total_new_files']
p['whole_store_before_cleanup']=whole['total'];p['active_PZ_representation']=result
p['cleanup']['prospective_before_any_credit']=p['last_closed_ownership_bytes']+p['cleanup']['new_write_return_reservation']
release=p['cleanup']['potential_release_bytes'];p['expected_store_after_cleanup_bytes']=whole['total']['logical_bytes']-release
p['expected_store_after_cleanup_files']=whole['total']['files']-504833-32
p['packed_capture_restore']['conservative_formula_using_expected_release']=p['last_closed_ownership_bytes']-release+620000000+600000000+33554432
p['packed_capture_restore']['conditional_fits_current_cap']=p['packed_capture_restore']['conservative_formula_using_expected_release']<=p['current_total_cap']
assert p['expected_store_after_cleanup_bytes']<600000000 and p['expected_store_after_cleanup_files']<12000
proposal_path.write_text(json.dumps(p,indent=2));print(json.dumps({k:p[k] for k in ['last_closed_ownership_bytes','whole_store_before_cleanup','expected_store_after_cleanup_bytes','expected_store_after_cleanup_files']},indent=2));print('PZactive',result['files'],result['logical_bytes'],result['allocation_bytes'])
