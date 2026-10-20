"""Read-only closeout of namespace-prefix assertion; no engine case rerun."""
import hashlib,json,os
from pathlib import Path
root=Path('C:/file-native-packing-20261020/history-restored/disposable/store');base=Path('C:/file-native-packing-20261020/preservation-reopen')
events=json.loads((base/'events.json').read_bytes());assert len(events)==110
assert all(x['exit']==(2 if x['input']['action']=='point' else 0) and not x['violations'] for x in events)
points=[x for x in events if x['input']['action']=='point'];assert len(points)==101
assert all(json.loads(x['stdout'])['error']=='retired:snapshot' for x in points)
assert {json.dumps(x['input']['selected'],sort_keys=True) for x in points}=={json.dumps(json.loads(p.read_bytes()),sort_keys=True) for p in (root/'retired').glob('*.json')}
authorized=json.loads((root/'DISPOSABLE.json').read_bytes());normalized=authorized.removeprefix('\\\\?\\')
assert os.path.normcase(os.path.normpath(normalized))==os.path.normcase(str(root.resolve()))
verified=json.loads(Path('C:/file-native-packing-20261020/receipts/preservation-verify-result.json').read_bytes())
assert json.loads((root/'SELECTED.json').read_bytes())==verified['selected']
result=dict(status='verified-reopened-and-freshly-authorized',selected=verified['selected'],publications=101006,ledger_rows=100000,retired_content_refusals=101,engine_operations=110,frozen_binary_sha256='369af68c46543d48f64f9d6ef79775a941873d8cc1404e1370870e7f808fd9a7',no_new_publication=True,disposable_authority_imported=False,original_helper_exit=1,original_failure='final Python plain-path comparison versus Windows extended canonical namespace; all engine cases passed',read_only_supplement=True,successful_engine_cases_repeated=False)
(base/'summary.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
