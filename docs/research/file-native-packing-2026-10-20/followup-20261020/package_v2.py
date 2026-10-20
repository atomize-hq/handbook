"""Incremental follow-up evidence only; originals and sole images remain external."""
import gzip, hashlib, io, json, subprocess, tarfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
R = Path('C:/file-native-packing-20261020')
FU = R / 'followup-20261020'
OUT = HERE / 'evidence'
assert not OUT.exists()
sources = {}
external = {}
for path in FU.rglob('*'):
    if path.is_file():
        sources['runtime/followup-20261020/' + path.relative_to(FU).as_posix()] = path
for path in HERE.iterdir():
    if path.is_file():
        sources['followup-harness/' + path.name] = path
case = R / 'operational-matrix/n8000-B'
for label in ['L-old-path-v2', 'L-new-path-v2', 'P0-path-v2', 'PZ-path-v2']:
    for suffix in ['-measurement.json', '-independent.json', '-inventory.jsonl']:
        path = case / (label + suffix)
        if path.is_file():
            sources['runtime/n8000-B/' + path.name] = path
    path = case / (label + '-events/events.json')
    if path.is_file():
        sources['runtime/n8000-B/' + label + '-events/events.json'] = path
    folder = R / 'operational-matrix/stage-evidence' / ('n8000-B-' + label)
    for path in folder.rglob('*'):
        if path.is_file():
            sources['runtime/stage-evidence/' + folder.name + '/' + path.relative_to(folder).as_posix()] = path
path = case / 'matched-cohort-inventory.jsonl'
if path.is_file():
    sources['runtime/n8000-B/' + path.name] = path
for label in ['baseline', 'L-old-path-v2', 'L-new-path-v2', 'P0-path-v2', 'PZ-path-v2']:
    capsule = case / (label + '-operational-backup')
    for path in capsule.iterdir() if capsule.exists() else []:
        if not path.is_file():
            continue
        name = 'runtime/n8000-B/' + capsule.name + '/' + path.name
        if label != 'baseline' and path.name == 'manifest.json':
            sources[name] = path
        else:
            external[name] = path
for path in (R / 'receipts').glob('followup-B-*'):
    name = 'runtime/receipts/' + path.name
    if path.is_file():
        sources[name] = path
parent = Path('C:/file-native-qnp-20261019')
for path in list((parent / 'followup-20261020').glob('B-*')) + list(parent.glob('followup-B-*-parent-verify.txt')):
    if path.is_file():
        sources['parent/' + path.name] = path
original = HERE.parent / 'final-evidence/raw-evidence.tar.gz'
assert hashlib.sha256(original.read_bytes()).hexdigest() == '5176e47d4a7569ed7ac57b29fe0972e5d2e1d37c713f44c4d38c95311ded815d'
external['original-sealed-evidence'] = original
native_binding = json.loads((FU / 'native/binding.json').read_bytes())
native_archive = Path(native_binding['parent_archive_retained_in_place'])
assert hashlib.sha256(native_archive.read_bytes()).hexdigest() == native_binding['parent_archive_sha256']
external['parent-fresh-Mac-recovery'] = native_archive
external_index = {}
for name, path in external.items():
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(1048576), b''):
            h.update(chunk)
    external_index[name] = dict(path=str(path), bytes=path.stat().st_size, sha256=h.hexdigest(), retained=True)
index = {}
expanded = 0
buffer = io.BytesIO()
with tarfile.open(fileobj=buffer, mode='w') as tar:
    for name, path in sorted(sources.items()):
        assert path.stat().st_size <= 32*1024**2
        raw = path.read_bytes()
        expanded += len(raw)
        assert expanded <= 128*1024**2
        row = tarfile.TarInfo(name)
        row.size = len(raw)
        row.mtime = 0
        tar.addfile(row, io.BytesIO(raw))
        index[name] = dict(bytes=len(raw), sha256=hashlib.sha256(raw).hexdigest())
archive_raw = buffer.getvalue()
compressed = gzip.compress(archive_raw, compresslevel=6, mtime=0)
assert len(compressed) <= 14*1024**2
assert gzip.decompress(compressed) == archive_raw
seen = set()
with tarfile.open(fileobj=io.BytesIO(archive_raw), mode='r:') as tar:
    for member in tar:
        assert member.isfile() and member.name not in seen
        seen.add(member.name)
        raw = tar.extractfile(member).read()
        assert len(raw) == index[member.name]['bytes'] and hashlib.sha256(raw).hexdigest() == index[member.name]['sha256']
assert seen == set(index)
OUT.mkdir()
(OUT / 'raw-evidence.tar.gz').write_bytes(compressed)
(OUT / 'raw-index.json').write_text(json.dumps(index, indent=2))
(OUT / 'external-retained-index.json').write_text(json.dumps(external_index, indent=2))
summary = dict(scope='incremental followup only; original sealed evidence unchanged', core='d70f3864f2888d46d9112213b4a03f8d82745a8e', harness_head=subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(), archive_bytes=len(compressed), archive_sha256=hashlib.sha256(compressed).hexdigest(), members=len(index), expanded_bytes=expanded, external_retained_entries=len(external_index), whole_gzip_EOF_and_all_member_hashes_verified=True)
(OUT / 'summary.json').write_text(json.dumps(summary, indent=2))
assert sum(path.stat().st_size for path in OUT.iterdir()) <= 15*1024**2
print(json.dumps(summary, indent=2))
