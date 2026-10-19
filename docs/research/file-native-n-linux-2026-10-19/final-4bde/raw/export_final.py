"""Authenticate final-pin receipts without changing the prior qualification."""
import json,hashlib,tarfile,sys,time,shutil
from pathlib import Path
BASE=Path(__file__).resolve().parent
OWNED=BASE.parent
sys.path.insert(0,str(BASE/'source/docs/research/file-native-navigation-2026-10-19'))
from census import census
done=json.loads((BASE/'completed.json').read_bytes());assert done['status']=='success' and done['completed_epoch']<1792453485
manifest=json.loads((BASE/'source-manifest.json').read_bytes())
for path,row in manifest['files'].items():assert hashlib.sha256((BASE/'source'/path).read_bytes()).hexdigest()==row['sha256']
assert hashlib.sha256((OWNED/'native-raw.tar.gz').read_bytes()).hexdigest()=='248c413ea513a1f310b8064f29319767930ca75fbb9576ee9dbd5f5060b79895'
old=json.loads((OWNED/'source-manifest.json').read_bytes())
for path,row in old['files'].items():assert hashlib.sha256((OWNED/'source'/path).read_bytes()).hexdigest()==row['sha256']
assert hashlib.sha256((OWNED/'build/release/file-native-incremental').read_bytes()).hexdigest()=='5295ce01770f9c4ba8b88bd4848f745c23c327ca78d6cd1a80770c2e1b69e89e'
assert hashlib.sha256(Path('/home/spenser/file-native-q-linux-20261019-final/build/debug/file-native-incremental').read_bytes()).hexdigest()=='e70d47a82e1c51a989395f0025f85859e74aed07c07e482e0a6b37913e76e854'
events=json.loads((BASE/'focused-data/events.json').read_bytes())+json.loads((BASE/'readme-data/events.json').read_bytes())
resources=json.loads((BASE/'focused-data/capsule-resources.json').read_bytes());assert all(not e['violations'] for e in events+resources)
stages=json.loads((BASE/'stages.json').read_bytes());assert len(stages)==4 and all(s['exit']==0 and s['censor'] is None for s in stages)
row=dict(epoch=time.time(),candidate=manifest['checkout'],core_files=len(manifest['core_Rust_Cargo_files']),source_reverified=True,prior_6e66_source_executable_and_archive_unchanged=True,prior_Q_binary_unchanged=True,focused_cases=len(done['full_focused']['cases']),focused_calls=done['full_focused']['events'],recovery=json.loads((BASE/'focused-data/recovery-capsule/summary.json').read_bytes()),README_cases=len(done['readme_smoke']),sampled_resource_rows=len(events+resources),missing_child_RSS_samples=sum(e['sampled_peak_rss'] is None for e in events+resources),maximum_sampled_engine_child_RSS=max((e['sampled_peak_rss'] for e in events+resources if e['sampled_peak_rss'] is not None),default=None),maximum_ordinary_child_seconds=max(e['seconds'] for e in events+resources),combined_native=census(OWNED),free_bytes=shutil.disk_usage(BASE).free)
row['shared_cargo_before']=json.loads((OWNED/'closeout.json').read_bytes())['shared_cargo_entire_current']
row['shared_cargo_after']=census('/home/spenser/.cargo')
row['shared_cargo_logical_growth_bytes']=max(0,row['shared_cargo_after']['logical_bytes']-row['shared_cargo_before']['logical_bytes'])
(BASE/'review-summary.json').write_text(json.dumps(row,indent=2))
paths=[p for p in BASE.iterdir() if p.is_file() and p.suffix in ['.py','.json','.stdout','.stderr']]
for directory in ['focused-data','focused-data/recovery-capsule','readme-data']:paths.extend((BASE/directory).glob('*.json'))
paths=sorted(set(paths));inventory={p.relative_to(BASE).as_posix():dict(bytes=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in paths}
assert sum(r['bytes'] for r in inventory.values())<16*1024**2
(BASE/'return-inventory.json').write_text(json.dumps(inventory,indent=2))
with tarfile.open(BASE/'final-raw.tar.gz','w:gz',compresslevel=6) as out:
    for p in paths+[BASE/'return-inventory.json']:out.add(p,arcname=p.relative_to(BASE),recursive=False)
archive=BASE/'final-raw.tar.gz'
(BASE/'return.json').write_text(json.dumps(dict(candidate=manifest['checkout'],archive_bytes=archive.stat().st_size,archive_sha256=hashlib.sha256(archive.read_bytes()).hexdigest(),raw_files=len(inventory),raw_bytes=sum(r['bytes'] for r in inventory.values()),combined_native_after_archive=census(OWNED),final_executable_sha256=done['final_executable_sha256']),indent=2))
print((BASE/'return.json').read_text())
