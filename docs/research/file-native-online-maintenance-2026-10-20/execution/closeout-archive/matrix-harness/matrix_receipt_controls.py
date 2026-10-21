"""Tiny real directory exclusions plus fake boundary budgets; no engine."""
import copy
import json
import os
from pathlib import Path
import sys
import matrix_budget as module
from job_guard import census,sha_file,write_json

assert os.environ.get('FILE_NATIVE_STAGE_JOB') == '1' and sys.dont_write_bytecode
root,output = Path(sys.argv[1]).resolve(),Path(sys.argv[2]).resolve()
runtime=Path('C:/file-native-online-matrix-20261020').resolve()
assert root.is_relative_to(runtime/'protocol/paged-controls-v5') and not root.exists()
assert output.is_relative_to(runtime/'receipts') and not output.exists()
assert not any(output.is_relative_to(runtime/'receipts'/name) for name in ['stream-control-main','windows-cadence'])
real=module.check_matrix({'protocol':dict(bytes=4096,files=6),'receipts':dict(bytes=256*1024,files=10)})
assert real['matrix_nonstream_receipts']['reservation_categories'] == ['stream_receipts','windows_cadence_receipts']
receipts=root/'receipts'
for name,data in [('other',b'x'*11),('nested/other',b'x'*19),('stream/reserved',b'x'*40),('cadence/reserved',b'x'*50)]:
    p=receipts/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(data)
assert module.excluding(receipts,[receipts/'stream',receipts/'cadence']) == dict(bytes=30,files=2)
assert module.excluding(receipts,receipts/'stream') == dict(bytes=80,files=3)
categories=dict(receipts=dict(root=str(receipts),bytes=1000,files=20),
    stream_receipts=dict(root=str(receipts/'stream'),bytes=100,files=2,nonadditive_subset_of='receipts'),
    windows_cadence_receipts=dict(root=str(receipts/'cadence'),bytes=200,files=3,nonadditive_subset_of='receipts'),
    stream_landing_controls=dict(root=str(root/'landing'),bytes=50,files=1))
config=dict(receipt_reservation_categories=['stream_receipts','windows_cadence_receipts'],
    allocation_notes=dict(matrix_excluding_reserved_receipts_max_bytes=650),
    groups=dict(F12=dict(categories=['receipts','stream_landing_controls'],bytes=1000,files=20)))

class FakeBudget:
    def __init__(self,*unused):
        self.categories=copy.deepcopy(categories);self.config=copy.deepcopy(config)
    def check(self,*unused):return dict(categories={})

module.Budget=FakeBudget
positive=module.check_matrix({'receipts':dict(bytes=620,files=12)})['matrix_nonstream_receipts']
assert positive['actual'] == dict(bytes=30,files=2) and positive['cap_bytes'] == 650 and positive['cap_files'] == 14
negatives=[]
for name,reserve in [('reserved-sibling-bytes',dict(bytes=621)),('reserved-sibling-files',dict(files=13))]:
    try:module.check_matrix({'receipts':reserve})
    except AssertionError as error:negatives.append(dict(case=name,refused=True,reason=str(error)))
    else:raise AssertionError('other-receipt-overdraw-accepted:'+name)
config['receipt_reservation_categories'].append('stream_receipts')
try:module.check_matrix()
except AssertionError as error:negatives.append(dict(case='duplicate-reservation-category',refused=True,reason=str(error)))
else:raise AssertionError('duplicate-reservations-accepted')
config.pop('receipt_reservation_categories')
config['allocation_notes']=dict(matrix_nonstream_receipts_max_bytes=600)
legacy=module.check_matrix({'receipts':dict(bytes=520)})['matrix_nonstream_receipts']
assert legacy['actual']==dict(bytes=80,files=3) and legacy['cap_bytes']==600
try:module.check_matrix({'receipts':dict(bytes=521)})
except AssertionError as error:negatives.append(dict(case='legacy-stream-only-byte-bound',refused=True,reason=str(error)))
else:raise AssertionError('legacy-receipt-overdraw-accepted')
write_json(output,dict(status='passed',helper_sha256=sha_file(module.__file__),
    real_preflight=real,actual_synthetic_tree=census(root),positive=positive,negative_cases=negatives,
    one_traversal_multiple_reservations=True,external_landing_caps_reserved=True,
    prospective_bytes_and_files_enforced=True,old_note_fallback_preserved=True,no_engine=True))
print(json.dumps(dict(status='passed',negative_cases=len(negatives))))
