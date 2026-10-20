"""Actual CP1 A/B full oracles and isolated required-metadata negative probes."""
import json
import os
from pathlib import Path
import shutil
import sys
from job_guard import Budget, census, sha_file, write_json
from physical_decoder import Decoder, verify, SCHEMA_PIN

base = Path('C:/file-native-online-matrix-20261020')
source_a = Path('C:/file-native-online-cp1-20261020/fixtures/smoke1-A/disposable/store')
source_b = Path('C:/file-native-online-cp1-20261020/fixtures/smoke1-B/disposable/store')
size = census(source_a)
Budget(os.environ['FILE_NATIVE_BUDGET']).check({'protocol': dict(bytes=3*size['bytes']+1024**2, files=3*size['files']+100)})
out = base/'receipts/physical-decoder-v1'
out.mkdir(exist_ok=False)
before = {str(root): {name: sha_file(root/name) for name in ['STORE.json', 'SELECTED.json']}
          for root in [source_a, source_b]}
cases = []
for mode, root in [('A', source_a), ('B', source_b)]:
    answer = verify(root, out/(mode+'-full.json'), quiescent=True)
    assert answer['navigation']['readable_declared_current']
    if mode == 'B':
        assert answer['request_intents']['exact_loose_B_publication_controls'] == answer['semantic']['publications']
    cases.append(dict(case=mode+'-actual-full', actual_H=answer['semantic']['publications'], raw_records=answer['raw_records']))
config = json.loads((source_a/'STORE.json').read_bytes())
pin = 'physical-pins/'+config['physical']['id']+'.lock'
plan = 'physical-plans/'+config['physical']['plan']+'.json'
for name in ['pin-bool-version', 'missing-pin', 'damaged-selected-plan']:
    destination = base/'protocol/decoder-negatives'/name/'store'
    def omit(directory, names):
        return [n for n in names if n == 'DISPOSABLE.json' or
                name == 'missing-pin' and Path(directory) == source_a/'physical-pins' and n == Path(pin).name]
    shutil.copytree(source_a, destination, ignore=omit)
    if name == 'pin-bool-version':
        p = destination/pin
        identity = json.loads(p.read_bytes())
        assert identity['version'] == 1
        identity['version'] = True
        p.write_text(json.dumps(identity), encoding='utf-8')
    elif name == 'damaged-selected-plan':
        p = destination/plan
        with p.open('ab') as output:
            output.write(b' ')
    try:
        Decoder(destination, quiescent=True)
    except (AssertionError, FileNotFoundError) as error:
        cases.append(dict(case=name, refused=True, error=type(error).__name__, fixture=str(destination)))
    else:
        raise AssertionError('required-physical-metadata-accepted:'+name)
after = {str(root): {name: sha_file(root/name) for name in ['STORE.json', 'SELECTED.json']}
         for root in [source_a, source_b]}
assert before == after
write_json(out/'summary.json', dict(status='passed', schema_pin=SCHEMA_PIN, cases=cases,
           decoder_sha256=sha_file(Path(__file__).with_name('physical_decoder.py')),
           outcome_oracle_sha256=sha_file(Path(__file__).with_name('outcome_oracle.py')),
           sources_read_only=True, selected_and_store_unchanged=True,
           qualification='actual CP1 fixtures; negative derivatives are labeled structural integrity probes; not a final-pin matrix'))
print(json.dumps(dict(status='passed', cases=len(cases))))
