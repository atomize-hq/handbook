"""Archive bounded raw main-matrix evidence without bulk stores or binaries."""
import gzip,hashlib,io,json,subprocess,tarfile
from pathlib import Path

HERE=Path(__file__).resolve().parent;R=Path('C:/file-native-packing-20261020');OUT=HERE/'main-checkpoint'
OUT.mkdir(exist_ok=False);files={}
for folder in ['long-matrix','long-matrix-fixed2']:
    for p in (R/folder).rglob('*'):
        if p.is_file():files['runtime/'+p.relative_to(R).as_posix()]=p
for pattern in ['final-PZ*','main-cleanup*','postcleanup*','next-large-stage-proposal.json','remaining-matrix-proposal.json','measurement-stage-detect.txt','packed-backup-*-impact.txt']:
    for p in (R/'receipts').glob(pattern):
        if p.is_file():files['runtime/'+p.relative_to(R).as_posix()]=p
for name in ['final_long_matrix.py','final_tables.py','post_measure_metadata.py','main_cleanup.py','postcleanup_check.py','packed_backup_stage.py','independent.py','driver.py','guard.py','main_checkpoint.py']:
    files['harness/'+name]=HERE/name
for name in ['fix2/build-binding.json','post-gc-operational-backup/manifest.json','preservation-reopen/summary.json','receipts/preservation-verify-result.json']:
    files['runtime/'+name]=R/name
assert 'runtime/long-matrix-fixed2/postcleanup-independent.json' in files and 'runtime/long-matrix-fixed2/postcleanup-reopen.json' in files
index={};total=0
with (OUT/'raw-evidence.tar.gz').open('xb') as f,gzip.GzipFile(filename='',mode='wb',fileobj=f,compresslevel=6,mtime=0) as compressed,tarfile.open(fileobj=compressed,mode='w|') as tar:
    for name,p in sorted(files.items()):
        b=p.read_bytes();assert len(b)<=8000000;total+=len(b);assert total<=32000000
        index[name]=dict(bytes=len(b),sha256=hashlib.sha256(b).hexdigest());info=tarfile.TarInfo(name);info.size=len(b);info.mtime=0;tar.addfile(info,io.BytesIO(b))
archive=OUT/'raw-evidence.tar.gz';(OUT/'raw-evidence-index.json').write_text(json.dumps(index,indent=2))
binding=json.loads((R/'fix2/build-binding.json').read_bytes())['files'];core={}
for name,row in binding.items():
    b=subprocess.check_output(['git','show','d70f3864f2888d46d9112213b4a03f8d82745a8e:'+name]);assert hashlib.sha256(b).hexdigest()==row['sha256']==hashlib.sha256(Path(name).read_bytes()).hexdigest();core[name]=dict(sha256=row['sha256'],git_blob=subprocess.check_output(['git','rev-parse','d70f3864:'+name],text=True).strip())
summary=dict(core_commit='d70f3864f2888d46d9112213b4a03f8d82745a8e',core_files=core,executable_sha256='568f631acf528c7de911bb88ec85d07ead9d7b4435ebe970c61053f06e31a93b',harness_commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),oracle_commit='ba2ce1bbb3e963cff298ec270841c037e0a0a5df',oracle_sha256=hashlib.sha256((HERE/'independent.py').read_bytes()).hexdigest(),archive_bytes=archive.stat().st_size,archive_sha256=hashlib.sha256(archive.read_bytes()).hexdigest(),members=len(files),expanded_bytes=total,bulk_datasets_and_executables_excluded=True,main=json.loads((R/'long-matrix-fixed2/postcleanup-reopen.json').read_bytes()),packed_operational_restore_not_yet_qualified=True)
(OUT/'summary.json').write_text(json.dumps(summary,indent=2));print(json.dumps({k:v for k,v in summary.items() if k!='core_files'},indent=2))
