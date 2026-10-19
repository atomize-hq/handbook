"""Authenticate separate final-pin raw evidence, core hashes and actual assertions."""
import json,hashlib,tarfile
from pathlib import Path
BASE=Path(__file__).resolve().parent
receipt=json.loads((BASE/'return.json').read_bytes());archive=BASE/'final-raw.tar.gz'
assert hashlib.sha256(archive.read_bytes()).hexdigest()==receipt['archive_sha256']
destination=BASE/'raw';destination.mkdir(exist_ok=False)
with tarfile.open(archive) as source:
    entries=source.getmembers();assert len(entries)<1000 and sum(p.size for p in entries)<16*1024**2
    for p in entries:assert p.isfile() and (destination/p.name).resolve().is_relative_to(destination.resolve())
    inventory=json.load(source.extractfile('return-inventory.json'))
    assert len(entries)==len(inventory)+1 and {p.name for p in entries}==set(inventory)|{'return-inventory.json'}
    for path,row in inventory.items():
        raw=source.extractfile(path).read();assert len(raw)==row['bytes'] and hashlib.sha256(raw).hexdigest()==row['sha256']
    source.extractall(destination,filter='data')
for p in BASE.glob('*.py'):
    if (destination/p.name).exists():assert p.read_bytes()==(destination/p.name).read_bytes(),p
load=lambda path:json.loads((destination/path).read_bytes())
manifest=load('source-manifest.json');assert len(manifest['core_Rust_Cargo_files'])==16
assert manifest['checkout']=='4bde156323812ef18c0dea6798cfc2108aaad933'
old=json.loads((BASE.parent/'evidence/source-manifest.json').read_bytes())
changed=[p for p,r in manifest['core_Rust_Cargo_files'].items() if r['sha256']!=old['files'][p]['sha256']]
assert changed==['docs/research/file-native-incremental-engine-experiment-2026-10-18/src/store.rs']
assert len(load('focused-data/summary.json')['cases'])==39 and len(load('focused-data/recovery-capsule/summary.json')['cases'])==24
smoke=load('readme-data/summary.json');assert len(smoke)==8 and all(r['status']=='success' for r in smoke)
assert all(r['exit']==0 and r['censor'] is None for r in load('stages.json'))
row=dict(candidate=manifest['checkout'],core_src_tree=manifest['core_src_tree'],core_files=16,only_changed_core_file=changed,archive_sha256=receipt['archive_sha256'],raw_files_verified=len(inventory),raw_bytes_verified=sum(r['bytes'] for r in inventory.values()),native_executable_sha256=load('executable.json')['sha256'],source_manifest_sha256=load('executable.json')['source_manifest_sha256'],native_harness_matches_local=True,focused_cases=39,recovery_cases=24,README_smoke_cases=8,all_completed_assertions_passed=True)
(BASE/'transfer-verification.json').write_text(json.dumps(row,indent=2))
print(json.dumps(row,indent=2))
