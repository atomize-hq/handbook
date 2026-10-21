"""Tiny journal descriptor controls, independent of real engine fixtures."""
import copy
import json
import os
from pathlib import Path
import sys
from job_guard import sha_file, write_json
from matrix_budget import check_matrix
from physical_decoder_v4 import Decoder

assert os.environ.get('FILE_NATIVE_STAGE_JOB') == '1' and sys.dont_write_bytecode
root = Path(sys.argv[1])
output = Path(sys.argv[2])
expected_sha = sys.argv[3]
decoder = Path(__file__).with_name('physical_decoder_v4.py')
assert sha_file(decoder) == expected_sha
check_matrix({'protocol':dict(bytes=4096,files=1),'receipts':dict(bytes=32768,files=2)})
assert not root.exists() and not output.exists()
root.mkdir()
(root/'PACKING.json').write_bytes(b'journal-presence-only')
descriptor = dict(pack='a'*64,index='b'*64,pack_bytes=1,index_bytes=53,records=1,codec=0,
    role='publication',first='publications/p.json',last='publications/p.json',
    instance='physical-static',segment=7)
journal = dict(version=2,id='physical-static',status='complete',expected_config='c'*64,
    catalog='d'*64,old=[descriptor],reserved_bytes=0,used_bytes=0,deleted_bytes=0,deferred_bytes=0,
    ownership_transferred=False)

class Fixture(Decoder):
    def __init__(self):
        self.root = root
        self.config = dict(physical=dict(id='physical-static'))
        self.required = set()
        self.value = copy.deepcopy(journal)
    def read_metadata(self,*unused):
        return self.value

fixture = Fixture()
assert fixture.journal()['selected_by_STORE']
fixture.value['old'][0].pop('segment')
assert fixture.journal()['selected_by_STORE']
rows = []
for label,changes in [('bool-segment',dict(segment=True)),('negative-segment',dict(segment=-1)),
        ('overflow-segment',dict(segment=2**64)),('unknown-descriptor-field',dict(unrecognized=0))]:
    fixture.value = copy.deepcopy(journal)
    fixture.value['old'][0].update(changes)
    try:
        fixture.journal()
    except (AssertionError,TypeError,ValueError) as error:
        rows.append(dict(case=label,refused=True,reason=str(error)))
    else:
        raise AssertionError('negative-was-accepted:'+label)
fixture.value = copy.deepcopy(journal)
fixture.value['old'] *= 65
try:
    fixture.journal()
except AssertionError as error:
    rows.append(dict(case='journal-old-descriptor-count65',refused=True,reason=str(error)))
else:
    raise AssertionError('unbounded-old-descriptor-list-accepted')
assert sha_file(decoder) == expected_sha
write_json(output,dict(status='passed',decoder_sha256=expected_sha,
    nonzero_segment_in_journal_old_accepted=True,legacy_segment_omission_accepted=True,
    negative_cases=rows,no_engine=True,no_real_fixture_reads=True))
print(json.dumps(dict(status='passed',negative_cases=len(rows))))
