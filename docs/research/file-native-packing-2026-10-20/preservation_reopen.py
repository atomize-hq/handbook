import hashlib,json
from pathlib import Path
import driver
from driver import Driver
root=Path('C:/file-native-packing-20261020/history-restored/disposable/store')
base=Path('C:/file-native-packing-20261020/preservation-reopen');d=Driver(base)
driver.EXE=Path('C:/file-native-packing-20261020/bin/slice1.exe')
assert hashlib.sha256(driver.EXE.read_bytes()).hexdigest()=='369af68c46543d48f64f9d6ef79775a941873d8cc1404e1370870e7f808fd9a7'
verified=json.loads(Path('C:/file-native-packing-20261020/receipts/preservation-verify-result.json').read_bytes())
assert verified['publications']==101006 and verified['ledger_rows']==100000 and verified['retired_publications']==101
assert d.call(root,'inspect')['result']['selected']==verified['selected']
d.call(root,'audit');d.call(root,'lifecycle',operation='setup')
retired=[]
for marker in sorted((root/'retired').glob('*.json')):
    selected=json.loads(marker.read_bytes());r=d.call(root,'point',code=2,selected=selected,id='d000000');assert r['error']=='retired:snapshot';retired.append(selected)
assert len(retired)==101
for request in ['initial','history-000001','history-000101','history-050000','history-100000','deep-lost-reply']:
    assert d.call(root,'resolve',request_id=request)['result']['outcome']=='accepted'
assert json.loads((root/'DISPOSABLE.json').read_bytes())==str(root.resolve())
result=dict(status='verified-reopened-and-freshly-authorized',selected=verified['selected'],publications=101006,ledger_rows=100000,retired_content_refusals=len(retired),frozen_binary_sha256=hashlib.sha256(driver.EXE.read_bytes()).hexdigest(),no_new_publication=True,disposable_authority_imported=False)
(base/'summary.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
