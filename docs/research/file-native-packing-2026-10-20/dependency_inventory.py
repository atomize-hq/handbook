import hashlib,json,shutil,subprocess
from pathlib import Path
here=Path(__file__).resolve().parent;core=here.parent/'file-native-incremental-engine-experiment-2026-10-18'
p=subprocess.run(['cargo','metadata','--locked','--offline','--format-version','1'],cwd=core,capture_output=True,check=True,timeout=120);m=json.loads(p.stdout)
rows=[];notices=here/'notices';notices.mkdir(exist_ok=True)
for p in m['packages']:
    if p['source'] is None:continue
    root=Path(p['manifest_path']).parent
    licenses={f.name:dict(bytes=f.stat().st_size,sha256=hashlib.sha256(f.read_bytes()).hexdigest()) for f in root.iterdir() if f.is_file() and f.name.upper().startswith(('LICENSE','COPYING','NOTICE'))}
    rows.append(dict(name=p['name'],version=p['version'],license=p['license'],manifest_sha256=hashlib.sha256((root/'Cargo.toml').read_bytes()).hexdigest(),license_files=licenses))
    if p['name'] in ['zstd','zstd-safe','zstd-sys']:
        for f in root.iterdir():
            if f.is_file() and f.name.upper().startswith(('LICENSE','COPYING','NOTICE')):shutil.copyfile(f,notices/(p['name']+'-'+f.name))
        if p['name']=='zstd-sys':
            for f in [root/'zstd/LICENSE',root/'zstd/COPYING']:
                if f.exists():shutil.copyfile(f,notices/('zstd-C-'+f.name))
(here/'dependency-inventory.json').write_text(json.dumps(dict(packages=rows,lock_sha256=hashlib.sha256((core/'Cargo.lock').read_bytes()).hexdigest()),indent=2))
print(json.dumps(rows,indent=2))
