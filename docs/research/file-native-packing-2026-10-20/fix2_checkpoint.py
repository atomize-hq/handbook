"""Bounded exact raw receipts/source package; datasets remain at bound local paths."""
import gzip,hashlib,io,json,tarfile
from pathlib import Path

here=Path(__file__).resolve().parent;r=Path('C:/file-native-packing-20261020');out=here/'fix2-candidate';selected={}
for directory in [r/'fix2/unit-tests',r/'fix2/small-qualification']:
    for path in directory.iterdir():
        if path.is_file():selected['runtime/'+path.relative_to(r).as_posix()]=path
for directory in [r/'fix2/regression',r/'fix2/full-capsule',r/'fix2/strict-supplement',r/'long-matrix-fixed2']:
    for path in directory.iterdir():
        if path.is_file() and path.suffix in ['.json','.jsonl','.stdout','.stderr','.py','.ps1']:selected['runtime/'+path.relative_to(r).as_posix()]=path
    path=directory/'events/events.json'
    if path.exists():selected['runtime/'+path.relative_to(r).as_posix()]=path
for prefix in ['fix2-build','fix2-regression','B-shell-after-fix','B-shell-before-fix','B-shell-before-fix-v2','B-shell-before-fix-v3','fixed-P0-readonly','final-unit-tests','artifact-archive','artifact-archive-XZ','artifact-reclaim-plan-guard','artifact-reclaim-guard','final-small-qualification']:
    for suffix in ['.json','.stdout','.stderr']:
        path=r/'receipts'/(prefix+suffix);assert path.is_file();selected['runtime/'+path.relative_to(r).as_posix()]=path
for name in ['artifact-reclaim.json','artifact-reclaim.json.jsonl','artifact-reclaim-source-quiescence.json','artifact-reclaim-postcheck.json','service-stop-reconciliation.json','archive-unit-sequencing-proposal.json','final-source-small-return.json']:
    path=r/'receipts'/name;assert path.is_file();selected['runtime/'+path.relative_to(r).as_posix()]=path
for directory,names in [(r/'artifact-preservation',['archive-harness.py','fresh-admission.json','incomplete-prefix.json']),(r/'artifact-preservation-xz',['archive-harness.py','fresh-admission.json','files.jsonl','manifest.json'])]:
    for name in names:
        path=directory/name;selected['runtime/'+path.relative_to(r).as_posix()]=path
for directory in (r/'B-shell').iterdir():
    for path in directory.iterdir():
        if path.is_file():selected['runtime/'+path.relative_to(r).as_posix()]=path
for name in ['guard.py','driver.py','independent.py','capsule.py','strict_supplement.py','b_shell_reproducer.py','selected_once_copy.ps1','fix2_build.py','fix2_regression.py','final_unit_tests.py','final_small_qualification.py','fixed_P0_readonly.py','archive_artifacts.py','archive_artifacts_xz.py','reclaim_archived_targets.ps1','fix2_checkpoint.py']:
    selected['source/'+name]=here/name
for name in ['verify_operational.py','operational_backup.py']:
    selected['dependent-source/recovery/'+name]=here.parent/'file-native-recovery-reclamation-2026-10-19'/name
selected['dependent-source/incremental/verify_store.py']=here.parent/'file-native-incremental-engine-experiment-2026-10-18/verify_store.py';selected['dependent-source/navigation/oracle.py']=here.parent/'file-native-navigation-2026-10-19/oracle.py'
rows=[];buffer=io.BytesIO();total=0
with tarfile.open(fileobj=buffer,mode='w') as archive:
    for name,path in sorted(selected.items()):
        assert not path.is_symlink();raw=path.read_bytes();total+=len(raw);assert len(raw)<=8*1024**2 and total<=32*1024**2
        rows.append(dict(path=name,source=str(path),bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest()));member=tarfile.TarInfo(name);member.size=len(raw);member.mode=0o644;member.mtime=0;archive.addfile(member,io.BytesIO(raw))
compressed=gzip.compress(buffer.getvalue(),compresslevel=6,mtime=0);assert len(compressed)<=1536*1024;path=out/'raw-evidence.tar.gz';assert not path.exists();path.write_bytes(compressed)
with tarfile.open(fileobj=io.BytesIO(compressed),mode='r:gz') as archive:
    assert len(archive.getmembers())==len(rows)
    for member,row in zip(archive.getmembers(),rows):assert member.isfile() and member.name==row['path'] and member.size==row['bytes'] and hashlib.sha256(archive.extractfile(member).read()).hexdigest()==row['sha256']
index=dict(core_commit='d70f3864f2888d46d9112213b4a03f8d82745a8e',exe_sha256='568f631acf528c7de911bb88ec85d07ead9d7b4435ebe970c61053f06e31a93b',files=rows,expanded_file_bytes=total,archive_bytes=len(compressed),archive_sha256=hashlib.sha256(compressed).hexdigest(),round_trip_verified=True,large_fixture_and_artifact_archives='preserved local paths and complete SHA-bound manifests; not duplicated into Git',old_P0_evidence='separate immutable 4cb checkpoint; final-source reader qualification separately bound')
(out/'raw-evidence-index.json').write_text(json.dumps(index,indent=2));(out/'qualified-state.json').write_bytes((r/'receipts/final-source-small-return.json').read_bytes());print(json.dumps({k:v for k,v in index.items() if k not in ['files','large_fixture_and_artifact_archives','old_P0_evidence']},indent=2))
