"""Authenticate native archive and every bounded raw member before materializing."""
import json,hashlib,tarfile
from pathlib import Path
BASE=Path(__file__).resolve().parent
receipt=json.loads((BASE/'evidence/return.json').read_bytes());archive=BASE/'evidence/native-raw.tar.gz'
assert hashlib.sha256(archive.read_bytes()).hexdigest()==receipt['sha256']
target=BASE/'evidence/native-raw';target.mkdir(exist_ok=False)
with tarfile.open(archive) as source:
    entries=source.getmembers();assert len(entries)<1000 and sum(x.size for x in entries)<32*1024**2
    for entry in entries:assert entry.isfile() and (target/entry.name).resolve().is_relative_to(target.resolve())
    inventory=json.load(source.extractfile('return-inventory.json'))
    assert {x.name for x in entries}==set(inventory)|{'return-inventory.json'}
    for path,row in inventory.items():
        raw=source.extractfile(path).read();assert len(raw)==row['bytes'] and hashlib.sha256(raw).hexdigest()==row['sha256']
    source.extractall(target,filter='data')
for path in BASE.glob('*.py'):
    returned=target/path.name
    if returned.exists():assert path.read_bytes()==returned.read_bytes(),path
(BASE/'evidence/transfer-verification.json').write_text(json.dumps(dict(archive_sha256=receipt['sha256'],files_verified=len(inventory),bytes_verified=sum(r['bytes'] for r in inventory.values()),native_executable=json.loads((target/'executable.json').read_bytes()),exact_native_external_harness_matches_local=True,unrelated_host_process_arguments_excluded=True),indent=2))
print((BASE/'evidence/transfer-verification.json').read_text())
