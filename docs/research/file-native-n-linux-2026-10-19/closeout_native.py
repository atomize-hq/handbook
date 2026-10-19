"""Authenticate bounded raw receipts, census preserved scope, and bind exact sources."""
import json,sys,time,hashlib,tarfile,platform,subprocess,resource
from pathlib import Path
import supervise as s

base=s.BASE
assert (base/'host-after-post.json').exists(),'qualification still active'
manifest=json.loads((base/'source-manifest.json').read_bytes())
assert hashlib.sha256((base/'source.tar').read_bytes()).hexdigest()==manifest['archive_sha256']
for path,row in manifest['files'].items():assert hashlib.sha256((base/'source'/path).read_bytes()).hexdigest()==row['sha256']
qexe=Path('/home/spenser/file-native-q-linux-20261019-final/build/debug/file-native-incremental')
qhash=hashlib.sha256(qexe.read_bytes()).hexdigest();assert qhash=='e70d47a82e1c51a989395f0025f85859e74aed07c07e482e0a6b37913e76e854'
temporary=[s.census(p) for p in sorted(Path('/tmp').glob('i1-bounds-*')) if p.is_dir()]
current=s.host_scope()
(base/'closeout.json').write_text(json.dumps(dict(epoch=time.time(),platform=platform.platform(),host=current,shared_cargo_entire_current=s.census('/home/spenser/.cargo'),test_temporary_roots_including_preserved=temporary,actual_new_test_temporary_root='/tmp/i1-bounds-250183-1792450605137123489-0',q_binary_preserved_sha256=qhash,exact_transferred_source_reverified=True,native_exe=json.loads((base/'executable.json').read_bytes()),closeout_checker_peak_rss=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*1024),indent=2))

# No engine datasets or build outputs enter Git. Native originals remain intact.
selected=[]
for p in sorted(base.iterdir()):
    if p.is_file() and p.name!='matrix-failed.json' and p.suffix in ['.json','.jsonl','.stdout','.stderr','.py','.sh']:selected.append(p)
for relative in ['focused-data','focused-data/recovery-capsule','supplement-data','stage1-data','historical2-data']:
    selected.extend(p for p in (base/relative).glob('*.json') if p.is_file())
for cell in sorted((base/'matrix').iterdir()):
    selected.extend(p for p in cell.glob('*.json') if p.is_file())
for mode in ['A','B']:
    selected.extend(p for p in (base/('bash-'+mode)).glob('*') if p.is_file())
selected=sorted(set(selected));rows={p.relative_to(base).as_posix():dict(bytes=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in selected}
assert sum(v['bytes'] for v in rows.values())<32*1024**2
(base/'return-inventory.json').write_text(json.dumps(rows,indent=2))
archive=base/'native-raw.tar.gz'
with tarfile.open(archive,'w:gz',compresslevel=6) as out:
    for p in selected+[base/'return-inventory.json']:out.add(p,arcname=p.relative_to(base),recursive=False)
(base/'return.json').write_text(json.dumps(dict(archive=str(archive),bytes=archive.stat().st_size,sha256=hashlib.sha256(archive.read_bytes()).hexdigest(),raw_files=len(rows),raw_bytes=sum(v['bytes'] for v in rows.values()),owned_after_archive=s.census(base),new_test_temporary_census=next(x for x in temporary if x['root'].endswith('i1-bounds-250183-1792450605137123489-0'))),indent=2))
print((base/'return.json').read_text())
