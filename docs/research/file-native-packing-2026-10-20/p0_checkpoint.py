"""Package bounded raw receipts and source bytes; never packages whole stores."""
import gzip,hashlib,io,json,tarfile
from pathlib import Path

here=Path(__file__).resolve().parent;runtime=Path('C:/file-native-packing-20261020');output=here/'p0-checkpoint';output.mkdir(exist_ok=False)
selected={}
for path in (runtime/'long-matrix').rglob('*'):
    if path.is_file():selected['runtime/'+path.relative_to(runtime).as_posix()]=path
for prefix in ['long-L-old-measure','long-L-new-measure','long-L-old-intent','long-P0-build','long-P0-verify','long-P0-measure','long-P0-intent','long-P0-allocation','subset-H1000-corpus','subset-H1000-P0-build','subset-H1000-P0-verify','qualified-loose-export','qualified-loose-build','sample-tool-build']:
    for suffix in ['.json','.stdout','.stderr']:
        path=runtime/'receipts'/(prefix+suffix);assert path.is_file();selected['runtime/'+path.relative_to(runtime).as_posix()]=path
for name in ['p0-fresh-admission.json','long-core-freeze-recheck.json','qualified-loose-source.json','sample-tool-binding.json','P0-stage-result.json','next-stage-resource-proposal-v2.json']:
    path=runtime/'receipts'/name;assert path.is_file();selected['runtime/'+path.relative_to(runtime).as_posix()]=path
for name in ['H1000-corpus/manifest.json','H1000-corpus/inventory.jsonl','H1000-P0/sample.json','H1000-P0-independent.json']:
    path=runtime/'subsets'/name;assert path.is_file();selected['runtime/'+path.relative_to(runtime).as_posix()]=path
for name in ['guard.py','driver.py','independent.py','long_matrix.py','intent_probe.py','derive_tables.py','allocation.py','allocation_parallel.py','subset_corpus.py','verify_subset.py','measurement-protocol.md','p0_checkpoint.py']:
    path=here/name;selected['measurement-source/'+name]=path
for name in ['verify_operational.py','operational_backup.py']:
    path=here.parent/'file-native-recovery-reclamation-2026-10-19'/name;selected['dependent-source/recovery/'+name]=path
for name in ['oracle.py']:
    path=here.parent/'file-native-navigation-2026-10-19'/name;selected['dependent-source/navigation/'+name]=path
selected['dependent-source/incremental/verify_store.py']=here.parent/'file-native-incremental-engine-experiment-2026-10-18/verify_store.py'
selected['measurement-source/qualified_lookup.rs']=runtime/'qualified-loose-core/src/bin/qualified_lookup.rs'
for name in ['Cargo.toml','Cargo.lock','main.rs']:
    selected['measurement-source/sample-tool/'+name]=here/'sample-tool'/name
selected['bindings/frozen-core.json']=here/'candidate/source-binding.json'
rows=[];buffer=io.BytesIO();total=0
with tarfile.open(fileobj=buffer,mode='w') as archive:
    for name,path in sorted(selected.items()):
        assert not path.is_symlink();raw=path.read_bytes();total+=len(raw);assert len(raw)<=8*1024**2 and total<=16*1024**2
        row=dict(path=name,source=str(path),bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest());rows.append(row)
        member=tarfile.TarInfo(name);member.size=len(raw);member.mtime=0;member.mode=0o644;archive.addfile(member,io.BytesIO(raw))
compressed=gzip.compress(buffer.getvalue(),compresslevel=6,mtime=0);assert len(compressed)<=4*1024**2
(output/'raw-evidence.tar.gz').write_bytes(compressed)
with tarfile.open(fileobj=io.BytesIO(compressed),mode='r:gz') as archive:
    assert len(archive.getmembers())==len(rows)
    for member,row in zip(archive.getmembers(),rows):
        assert member.isfile() and member.name==row['path'] and member.size==row['bytes'];raw=archive.extractfile(member).read();assert hashlib.sha256(raw).hexdigest()==row['sha256']
index=dict(profile='P0-completed-stage-raw-receipts-and-exact-harness',core_pin='4cb6b9fba9ffabc294a6006ec54fe75366968db4',harness_base='5b49815e',files=rows,expanded_file_bytes=total,archive_bytes=len(compressed),archive_sha256=hashlib.sha256(compressed).hexdigest(),round_trip_verified=True,large_data='protected local original capture, working packed clone and H1k corpus/containers remain at their bound absolute paths; not duplicated into Git',PZ_qualified=False,cleanup_qualified=False)
(output/'raw-evidence-index.json').write_text(json.dumps(index,indent=2))
summary=json.loads((runtime/'receipts/P0-stage-result.json').read_bytes());summary['allocation']=json.loads((runtime/'long-matrix/storage-observations.json').read_bytes());summary['H1000_record_subset']=json.loads((runtime/'subsets/H1000-P0-independent.json').read_bytes());summary['next_resource_proposal']=json.loads((runtime/'receipts/next-stage-resource-proposal-v2.json').read_bytes());summary['current_actual']=summary['next_resource_proposal']['current'];(output/'summary.json').write_text(json.dumps(summary,indent=2))
print(json.dumps(dict(files=len(rows),expanded_file_bytes=total,archive_bytes=len(compressed),sha256=index['archive_sha256'],round_trip_verified=True)))
