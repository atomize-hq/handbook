"""Fault only fresh copies; historical f6 Work2 decoder refusal controls."""
import copy
import hashlib
import json
from pathlib import Path
import shutil
from job_guard import Budget, sha_file, write_json
import os
from physical_decoder_v2 import Decoder, SCHEMA_PIN

runtime = Path('C:/file-native-online-matrix-20261020')
source = Path('C:/file-native-online-cp1-20261020/fixtures/rem8/disposable/store')
destination = runtime/'protocol/work2-f6-negative-copies'
assert not destination.exists()
before = {p.relative_to(source).as_posix(): sha_file(p) for p in source.rglob('*') if p.is_file()}
size = sum(p.stat().st_size for p in source.rglob('*') if p.is_file())
Budget(os.environ['FILE_NATIVE_BUDGET']).check({'protocol':dict(bytes=6*size+1024**2,files=6*len(before)+100)})
cases = []
for name in ['mutable-progress', 'bad-selected-plan-bytes', 'malformed-selector', 'wrong-selector-parent', 'duplicate-birth-claims', 'bool-pin-version']:
    root = destination/name/'disposable/store'
    shutil.copytree(source, root)
    c = json.loads((root/'STORE.json').read_bytes())
    b = c['physical']
    plan_path = root/('physical-plans/'+b['plan']+'.json')
    work = json.loads(plan_path.read_bytes())
    if name == 'mutable-progress':
        (root/'physical-progress').mkdir(exist_ok=True)
        (root/('physical-progress/'+b['id']+'.json')).write_bytes(b'{}')
    elif name == 'bad-selected-plan-bytes':
        plan_path.write_bytes(plan_path.read_bytes()+b' ')
    elif name == 'malformed-selector':
        (root/'physical-progress').mkdir(exist_ok=True)
        (root/('physical-progress/'+b['id']+'.selector-stage')).write_bytes(b'{')
    elif name in ['wrong-selector-parent', 'duplicate-birth-claims']:
        candidate = copy.deepcopy(work)
        if name == 'wrong-selector-parent':
            candidate['transition_parent'] = '0'*64
        else:
            candidate['birth_claims'] = ['duplicate', 'duplicate']
        raw = json.dumps(candidate, indent=2).encode()
        new_hash = hashlib.sha256(raw).hexdigest()
        (root/('physical-plans/'+new_hash+'.json')).write_bytes(raw)
        c['physical']['plan'] = new_hash
        if name == 'wrong-selector-parent':
            (root/'physical-progress').mkdir(exist_ok=True)
            (root/('physical-progress/'+b['id']+'.selector-stage')).write_text(json.dumps(c),encoding='utf-8')
        else:
            (root/'STORE.json').write_text(json.dumps(c),encoding='utf-8')
    else:
        p = root/('physical-pins/'+b['id']+'.lock')
        pin = json.loads(p.read_bytes());pin['version'] = True
        p.write_text(json.dumps(pin),encoding='utf-8')
    try:
        Decoder(root, quiescent=True)
    except (AssertionError, ValueError, FileNotFoundError, KeyError) as error:
        cases.append(dict(case=name, refused=True, exception=type(error).__name__, message=str(error)))
    else:
        raise AssertionError('negative-accepted:'+name)
after = {p.relative_to(source).as_posix(): sha_file(p) for p in source.rglob('*') if p.is_file()}
assert before == after
write_json(runtime/'receipts/work2-f6-decoder-negatives.json',dict(schema_pin=SCHEMA_PIN,cases=cases,
           source=str(source),source_files=len(before),complete_source_hash_manifest=before,
           source_all_file_hashes_unchanged=True,only_owned_fault_copies_mutated=True,
           qualification='historical f6 schema decoder controls only; not native engine fault or final-pin qualification'))
print(json.dumps(dict(status='passed',negative_cases=len(cases),source_files=len(before),source_unchanged=True)))
