"""Export exact committed bounded inputs, never working-tree substitutions."""
import hashlib,json,subprocess,tarfile,io
from pathlib import Path

ROOT=Path(__file__).resolve().parents[3]
OUT=Path(__file__).resolve().parent
PIN='6e66a7523612b80386cf9cc2b96d5341579b54fd'
E='docs/research/file-native-incremental-engine-experiment-2026-10-18'
H='docs/research/file-native-navigation-2026-10-19'
F='docs/research/file-native-recovery-reclamation-2026-10-19'
paths=subprocess.check_output(['git','ls-tree','-r','--name-only',PIN,E,H,F],cwd=ROOT).decode().splitlines()
chosen=[p for p in paths if (p.startswith(E+'/') and (p.endswith('.py') or '/src/' in p or '/consumers/' in p or p.endswith(('Cargo.lock','Cargo.toml','report.md','protocol.md','design-and-scope.md')))) or p.startswith(H+'/') or (p.startswith(F+'/') and p.endswith('.py'))]
manifest={}
with tarfile.open(OUT/'source.tar','w') as archive:
    for path in chosen:
        raw=subprocess.check_output(['git','show',PIN+':'+path],cwd=ROOT)
        manifest[path]=dict(bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest(),git_blob=subprocess.check_output(['git','rev-parse',PIN+':'+path],cwd=ROOT).decode().strip())
        item=tarfile.TarInfo(path);item.size=len(raw);item.mode=0o644;archive.addfile(item,io.BytesIO(raw))
(OUT/'evidence/source-manifest.json').write_text(json.dumps(dict(checkout=PIN,files=manifest,archive_sha256=hashlib.sha256((OUT/'source.tar').read_bytes()).hexdigest()),indent=2))
(OUT/'evidence/deadlines.json').write_text(json.dumps(dict(start='2026-10-19T22:54:45Z',execution_cutoff='2026-10-19T23:44:45Z',lane_deadline='2026-10-19T23:54:45Z',original_N_deadline='2026-10-20T00:38:51Z',effective_deadline='2026-10-19T23:54:45Z',seconds=3600,closeout_reserved_seconds=600,additional_owned_limit_bytes=768*1024**2,new_file_limit=100000,global_limit_bytes=40*1024**3,global_file_limit=5000000,reserve_bytes=1024**3,reserve_files=100000,parent_forecast_including_reserve_bytes=40828300920,parent_forecast_including_reserve_files=4282696),indent=2))
print(json.dumps(dict(files=len(chosen),source_bytes=sum(v['bytes'] for v in manifest.values()),archive_bytes=(OUT/'source.tar').stat().st_size)))
