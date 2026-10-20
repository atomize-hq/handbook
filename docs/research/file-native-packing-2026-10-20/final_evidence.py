"""Bounded closeout archive of raw results; bulk images and progress stay indexed."""
import gzip,hashlib,io,json,subprocess,tarfile
from pathlib import Path
HERE=Path(__file__).resolve().parent;R=Path('C:/file-native-packing-20261020');OUT=HERE/'final-evidence'
OUT.mkdir(exist_ok=False);files={};external={}
def add(path,label=None):
    path=Path(path)
    if not path.is_file():return
    name=label or 'runtime/'+path.relative_to(R).as_posix()
    size=path.stat().st_size
    if size>8000000 or path.name.endswith('.json.jsonl'):
        h=hashlib.sha256()
        with path.open('rb') as f:
            for b in iter(lambda:f.read(1048576),b''):h.update(b)
        external[name]=dict(path=str(path),bytes=size,sha256=h.hexdigest(),retained=True)
    else:files[name]=path
for folder in ['packed-restored-qualification','packed-backup-inputs','fix2/corpus-controls']:
    for p in (R/folder).rglob('*'):add(p)
for folder in ['packed-operational-backup','packed-successor-operational-backup']:
    for name in ['manifest.json','files.jsonl']:add(R/folder/name)
    p=R/folder/'operational.bin.gz';manifest=json.loads((R/folder/'manifest.json').read_bytes());external['runtime/'+p.relative_to(R).as_posix()]=dict(path=str(p),bytes=p.stat().st_size,sha256=manifest['transport_sha256'],retained=True,kind='full operational image')
for p in (R/'receipts').iterdir():add(p)
for p in (R/'fix2/loose-regression-final').iterdir():add(p)
for name in ['summary.json','events.json']:add(R/'fix2/loose-regression-final/data'/name)
matrix=R/'operational-matrix'
for p in (matrix/'stage-evidence').rglob('*'):add(p)
for case in matrix.glob('n8000-*'):
    for p in case.iterdir():
        if p.is_file():add(p)
        elif p.name.endswith('-events'):add(p/'events.json')
        elif p.name.endswith('-operational-backup'):
            for name in ['manifest.json','files.jsonl']:add(p/name)
            manifest_path=p/'manifest.json'
            if manifest_path.exists():
                m=json.loads(manifest_path.read_bytes());transport=p/'operational.bin.gz';external['runtime/'+transport.relative_to(R).as_posix()]=dict(path=str(transport),bytes=transport.stat().st_size,sha256=m['transport_sha256'],retained=True)
for name in ['tables.json','tables.md']:add(matrix/name)
for p in HERE.glob('*.py'):add(p,'harness/'+p.name)
for p in HERE.glob('*.ps1'):add(p,'harness/'+p.name)
add(HERE/'report.md','report.md');index={};expanded=0
with (OUT/'raw-evidence.tar.gz').open('xb') as raw,gzip.GzipFile(filename='',mode='wb',fileobj=raw,compresslevel=6,mtime=0) as compressed,tarfile.open(fileobj=compressed,mode='w|') as tar:
    for name,path in sorted(files.items()):
        b=path.read_bytes();expanded+=len(b);assert expanded<=128*1024**2
        row=tarfile.TarInfo(name);row.size=len(b);row.mtime=0;tar.addfile(row,io.BytesIO(b));index[name]=dict(bytes=len(b),sha256=hashlib.sha256(b).hexdigest())
(OUT/'raw-index.json').write_text(json.dumps(index,indent=2));(OUT/'external-retained-index.json').write_text(json.dumps(external,indent=2))
archive=OUT/'raw-evidence.tar.gz';summary=dict(core='d70f3864f2888d46d9112213b4a03f8d82745a8e',harness_head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),archive_bytes=archive.stat().st_size,archive_sha256=hashlib.sha256(archive.read_bytes()).hexdigest(),members=len(index),expanded_bytes=expanded,external_retained_entries=len(external),large_progress_and_images_retained_at_exact_hashed_paths=True)
(OUT/'summary.json').write_text(json.dumps(summary,indent=2));print(json.dumps(summary,indent=2))
