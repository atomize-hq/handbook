"""Boundary controls for sibling source reservations and legacy budget fallback."""
import copy
import json
import os
from pathlib import Path
import sys
import matrix_budget as module
from job_guard import write_json, sha_file

assert os.environ.get('FILE_NATIVE_STAGE_JOB') == '1' and sys.dont_write_bytecode
real = module.check_matrix({'receipts':dict(bytes=32768,files=2)})
assert real['matrix_source_excluding_reserved_focus']['reservation_categories'] == ['final_protocol_source','windows_cadence_source']
config = dict(source_reservation_categories=['final_protocol_source','windows_cadence_source'])
categories = dict(source=dict(files=200),final_protocol_source=dict(files=12,nonadditive_subset_of='source'),
    windows_cadence_source=dict(files=8,nonadditive_subset_of='source'))
rows = dict(source=dict(absolute_actual=dict(bytes=601,files=76),cap_bytes=1000),
    final_protocol_source=dict(absolute_actual=dict(bytes=100,files=7),cap_bytes=192),
    windows_cadence_source=dict(absolute_actual=dict(bytes=10,files=1),cap_bytes=64))

class FakeBudget:
    def __init__(self,*unused):
        self.config = copy.deepcopy(config)
        self.categories = copy.deepcopy(categories)
    def check(self,*unused):
        return dict(categories=copy.deepcopy(rows))

module.Budget = FakeBudget
positive = module.check_matrix({'source':dict(bytes=253,files=112)})
assert positive['matrix_source_excluding_reserved_focus'] == dict(actual_bytes=491,cap_bytes=744,
    actual_files=68,cap_files=180,reservation_categories=['final_protocol_source','windows_cadence_source'])
negatives = []
for name,reserves in [('other-owner-byte-reserve',dict(bytes=254)),('other-owner-file-reserve',dict(files=113))]:
    try:
        module.check_matrix({'source':reserves})
    except AssertionError as error:
        negatives.append(dict(case=name,refused=True,reason=str(error)))
    else:
        raise AssertionError('source-reservation-overdraw-accepted:'+name)
config.pop('source_reservation_categories')
legacy = module.check_matrix({'source':dict(bytes=307,files=119)})
assert legacy['matrix_source_excluding_reserved_focus']['reservation_categories'] == ['final_protocol_source']
assert legacy['matrix_source_excluding_reserved_focus']['cap_bytes'] == 808
try:
    module.check_matrix({'source':dict(bytes=308)})
except AssertionError as error:
    negatives.append(dict(case='legacy-focus-byte-reserve',refused=True,reason=str(error)))
else:
    raise AssertionError('legacy-source-reservation-overdraw-accepted')
write_json(Path(sys.argv[1]),dict(status='passed',real_preflight=real,
    budget_helper_sha256=sha_file(Path(module.__file__)),negative_cases=negatives,
    new_both_siblings_and_old_focus_fallback=True,no_engine=True))
print(json.dumps(dict(status='passed',negative_cases=len(negatives))))
