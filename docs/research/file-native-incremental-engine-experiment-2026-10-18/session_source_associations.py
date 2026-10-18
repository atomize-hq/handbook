"""Distinguish captured working-source snapshots from associated compiler sources."""
import argparse,hashlib,json,os,zipfile
from pathlib import Path

a=argparse.ArgumentParser();a.add_argument('--scratch',required=True);a.add_argument('--output',required=True);ns=a.parse_args()
scratch=Path(ns.scratch);source=Path(__file__).resolve().parent;catalog={};references=[]
for name,pin,receipt_name in [
 ('checks-retention-v2','f3e942f6a2e84ff33fd6ea860caed49ad650f86f','windows-f3-source-binding.json'),
 ('final-v2-checks','7ac70f93cce14ecd3d935852801f034c122000d8','windows-final-source-binding.json'),
 ('audit-after','7e00d22b470c1eed476127d2f585789036e79cf3','windows-audit-source-binding.json'),
 ('accounting-after-v2/paired-intents','5a65a2155f9eb297d2cbb1b7c900fa749629605c','windows-accounting-source-binding.json')]:
 base=scratch/name;session=json.loads((base/'session.json').read_bytes());receipt=json.loads((source/'evidence'/receipt_name).read_bytes());archive=base/'session-source.zip';raw=archive.read_bytes();digest=hashlib.sha256(raw).hexdigest()
 expected=receipt.get('compiled_source_archive_sha256') or receipt.get('archive_sha256') or receipt['source_archive_sha256'];assert digest==expected
 with zipfile.ZipFile(archive) as z:files={n:hashlib.sha256(z.read(n)).hexdigest() for n in z.namelist() if (n.startswith('src/') or n.startswith('consumers/')) and (n.endswith('.rs') or Path(n).name in {'Cargo.toml','Cargo.lock'}) or n in {'Cargo.toml','Cargo.lock'}}
 catalog[session['sha256']]=dict(pin=pin,files=files,reference=receipt_name)
 references.append(dict(exe_sha256=session['sha256'],pin=pin,compiler_source_receipt=receipt_name,source_archive_sha256=digest))
rows=[]
for directory,dirs,names in os.walk(scratch):
 dirs[:]=[name for name in dirs if name not in {'store','disposable','target','__pycache__','.git'} and not name.startswith('build') and not (Path(directory)/name/'STORE.json').exists()]
 if 'session.json' not in names:continue
 path=Path(directory)/'session.json';session=json.loads(path.read_bytes())
 if 'sha256' not in session:continue
 ref=catalog.get(session['sha256']);snapshot=session.get('source_hashes');archive=Path(directory)/'session-source.zip'
 row=dict(session=path.relative_to(scratch).as_posix(),exe_sha256=session['sha256'],workspace_source_archive_present=archive.is_file(),associated_compiler_source_receipt=ref['reference'] if ref else None,associated_core_pin=ref['pin'] if ref else None)
 if ref and snapshot:
  mismatches=[name for name,digest in ref['files'].items() if snapshot.get(name)!=digest]
  row.update(workspace_engine_sources_match_associated_compiler_snapshot=not mismatches,workspace_engine_source_mismatches=mismatches)
 else:row['workspace_engine_sources_match_associated_compiler_snapshot']=None
 rows.append(row)
report=dict(references=references,sessions=rows,qualification='Session ZIPs capture working source/harness at Driver creation. Later old-binary audits can capture newer Rust source; that ZIP alone is not its compiler-input evidence. Named independent build/source receipts associate those binaries. Unknown development/control binaries retain their raw hashes/snapshots without an invented compiler-input association. No reproducible-build claim.')
Path(ns.output).write_text(json.dumps(report,indent=2));print(json.dumps(dict(sessions=len(rows),references=len(references),different_workspace_snapshots=sum(r.get('workspace_engine_sources_match_associated_compiler_snapshot') is False for r in rows))))
