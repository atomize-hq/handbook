"""Missing OLD B publication control must refuse despite a valid packed record."""
import hashlib
import argparse
import json
import os
from pathlib import Path
import shutil
from job_guard import Budget, census, write_json
from physical_decoder import Decoder, verify, SCHEMA_PIN

base = Path('C:/file-native-online-matrix-20261020')
parser = argparse.ArgumentParser()
parser.add_argument('--attempt', type=int, default=1)
attempt = parser.parse_args().attempt
assert 1 <= attempt <= 10
suffix = '' if attempt == 1 else '-'+str(attempt)
source = Path('C:/file-native-online-cp1-20261020/fixtures/smoke1-B/disposable/store')
destination = base/('protocol/decoder-negatives/missing-old-B-control'+suffix+'/store')
output = base/('receipts/B-old-control-negative'+suffix+'.json')

def manifest(root):
    rows = []
    for directory, dirs, files in os.walk(root, followlinks=False):
        for name in sorted(files):
            p = Path(directory)/name
            assert not p.is_symlink()
            h = hashlib.sha256()
            with p.open('rb') as f:
                for block in iter(lambda: f.read(1024**2), b''):
                    h.update(block)
            rows.append(dict(path=p.relative_to(root).as_posix(), bytes=p.stat().st_size, sha256=h.hexdigest()))
    return sorted(rows, key=lambda row: row['path'])

size = census(source)
Budget(os.environ['FILE_NATIVE_BUDGET']).check({'protocol': dict(bytes=size['bytes']+1024**2, files=size['files']+100),
                                              'receipts': dict(bytes=1024**2, files=10)})
before = manifest(source)
write_json(output.with_name(output.stem+'-source-before.json'), dict(source=str(source), files=before))
d = Decoder(source, quiescent=True)
selected = json.loads((source/'SELECTED.json').read_bytes())
victim = selected
while True:
    p = json.loads(d.checked(source/f"publications/{victim['publication']}.json", victim['digest']))
    if p['predecessor'] is None:
        break
    victim = p['predecessor']
assert victim != selected
name = victim['publication']+'.json'
key = 'publications/'+name
assert key in d.entries, 'negative must retain a valid mapped publication record'

def omit(directory, names):
    return [n for n in names if n == 'DISPOSABLE.json' or Path(directory) == source/'publications' and n == name]

shutil.copytree(source, destination, ignore=omit)
reader = Decoder(destination, quiescent=True)
assert reader.checked(destination/key, victim['digest']) == d.checked(source/key, victim['digest'])
assert not (destination/key).exists()
try:
    verify(destination, base/'receipts/B-old-control-negative-unexpected.json', quiescent=True)
except FileNotFoundError as error:
    assert Path(error.filename).resolve() == (destination/key).resolve()
    refusal = str(error)
else:
    raise AssertionError('missing-old-B-loose-control-accepted')
after = manifest(source)
assert before == after
write_json(output, dict(status='passed', schema_pin=SCHEMA_PIN, victim=victim,
           current_selected=selected, valid_packed_record_preserved=True,
           old_loose_control_missing=True, independent_full_oracle_refused=refusal,
           source=str(source), negative_derivative=str(destination),
           source_files_before=before, source_files_after=after,
           all_source_paths_sizes_and_sha256_unchanged=True,
           qualification='labeled isolated negative derivative; ordinary B selected-once contract needs every accepted loose publication, not only current'))
print(json.dumps(dict(status='passed', source_files=len(before), missing_old_control=key)))
