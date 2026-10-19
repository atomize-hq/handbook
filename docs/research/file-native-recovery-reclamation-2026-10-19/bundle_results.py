"""Bounded review evidence; full store/transport payloads stay in owned runtime."""
import argparse,hashlib,json,shutil,tarfile,time
from pathlib import Path
a=argparse.ArgumentParser();a.add_argument('--runtime',required=True);a.add_argument('--destination',required=True);ns=a.parse_args();runtime=Path(ns.runtime);dest=Path(ns.destination);dest.mkdir(parents=True,exist_ok=True)
names=['original-roles.json','inventory-preflight-corrected.json','inventory-additional-corrected.json','admission.json','original-operational-verification-result.json','restored-operational-verification-result.json','post-gc-operational-verification-result.json','original-preservation-and-final-state.json','operational-restore-gate.json','final-build-binding.json','final-code-build-binding.json','native-return-verification.json','native-final-core-verification.json','native-compatible-core-verification.json','powershell-extraction.json','matched-small-final-summary.json','final-owned-roles.json','wsl-closeout-census.json','linux-closeout-census.txt','empty-external-directory-case.json','independent-oracle-whitespace-negative.json']
for name in names:shutil.copyfile(runtime/name,dest/name)
shutil.copyfile(runtime/'history-operational-backup/manifest.json',dest/'operational-backup-manifest.json')
for host in ['linux','wsl']:shutil.copyfile(runtime/(host+'-evidence-compatible.tar.gz'),dest/(host+'-raw.tar.gz'))
shutil.copyfile(runtime/'legacy-fixtures.tar.gz',dest/'legacy-fixtures.tar.gz')
files=[]
for p in runtime.iterdir():
 if p.is_file() and p.suffix in ['.json','.txt','.log','.stdout','.stderr']:files.append((p,'controller/'+p.name))
for name in ['windows-small-v1','windows-small-v2','windows-existing-checks-data','windows-final-capsule-data','windows-replay-data','windows-paired-data','windows-extra-data','windows-oracle-capsule-data','windows-parallel-backup-data','windows-final-core-data','windows-final-protocol-data','windows-compatible-core-data','windows-operational-final-data','windows-final-complete-data','windows-final-paired-data','windows-final-replay-data']:
 for filename in ['summary.json','events.json','events.jsonl','checks.json','paired-intents.json','replay-lifetime.json','extra.json','session.json','session-source.zip']:
  p=runtime/name/filename
  if p.exists():files.append((p,name+'/'+filename))
for p in (runtime/'reclamation').iterdir():
 if p.is_file():files.append((p,'reclamation/'+p.name))
with tarfile.open(dest/'windows-raw.tar.gz','w:gz',compresslevel=1) as archive:
 for p,name in files:archive.add(p,arcname=name)
if (dest/'windows-raw.tar.gz').stat().st_size>64*1024**2:raise ValueError('raw-evidence-cap')
index={p.name:dict(bytes=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in dest.iterdir() if p.is_file() and p.name!='index.json'}
index['qualification']='Full 2.47GB transport and 248MB per-file inventory stay at runtime; manifest above authenticates them. Raw failures/superseded attempts and unperformed native cases remain in bundles. Windows allocation labels in initial raw census are superseded by corrected JSON.'
(dest/'index.json').write_text(json.dumps(index,indent=2),encoding='utf-8');print(json.dumps({k:v['bytes'] for k,v in index.items() if isinstance(v,dict)},indent=2))
