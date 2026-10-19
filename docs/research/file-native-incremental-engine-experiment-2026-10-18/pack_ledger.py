"""Bounded ledger-only lossless transport if the full proof export is censored."""
import argparse,gzip,hashlib,json,time
from pathlib import Path

a=argparse.ArgumentParser();a.add_argument('--root',required=True);a.add_argument('--destination',required=True);a.add_argument('--deadline-epoch',type=float,required=True);ns=a.parse_args()
assert time.time()+300<ns.deadline_epoch,'Full300-second outer cap must fit original reserve'
began=time.monotonic();root=Path(ns.root);source=root/'expected-ledger.jsonl';destination=Path(ns.destination);destination.mkdir(exist_ok=False)
before=source.stat();digest=hashlib.sha256();chunks=[];offset=0
with source.open('rb') as f:
 while True:
  assert time.monotonic()-began<270
  raw=f.read(16*1024*1024)
  if not raw:break
  digest.update(raw);encoded=gzip.compress(raw,compresslevel=6,mtime=0);assert gzip.decompress(encoded)==raw
  path=destination/f'part-{len(chunks):03}.bin.gz';path.write_bytes(encoded)
  chunks.append(dict(path=path.name,offset=offset,raw_bytes=len(raw),raw_sha256=hashlib.sha256(raw).hexdigest(),bytes=len(encoded),sha256=hashlib.sha256(encoded).hexdigest()));offset+=len(raw)
count=0;second=hashlib.sha256()
with source.open('rb') as f:
 for line in f:
  assert time.monotonic()-began<285
  second.update(line);count+=1;assert json.loads(line)['index']==count
after=source.stat();assert (before.st_size,before.st_mtime_ns)==(after.st_size,after.st_mtime_ns)
assert count==100000 and offset==before.st_size and digest.hexdigest()==second.hexdigest()
restored=hashlib.sha256();length=0
for chunk in chunks:
 encoded=(destination/chunk['path']).read_bytes();assert hashlib.sha256(encoded).hexdigest()==chunk['sha256']
 raw=gzip.decompress(encoded);assert len(raw)==chunk['raw_bytes'] and hashlib.sha256(raw).hexdigest()==chunk['raw_sha256'];restored.update(raw);length+=len(raw)
assert length==offset and restored.hexdigest()==digest.hexdigest()
report=dict(kind='ledger-only-lossless-transport-not-selected-history-proof',source=str(source),selected=json.loads((root/'SELECTED.json').read_bytes()),raw_bytes=offset,raw_sha256=digest.hexdigest(),dense_rows=count,chunks=chunks,seconds=time.monotonic()-began,
 qualification='Exact dense expected-ledger bytes, verified through all compressed chunks and an independent reread. This does not substitute for the censored full proof export, selected-chain verification or a complete restartable store. Historical independent state checks remain separate raw observations. Caller enforces an actual300-second process-tree cap.')
(destination/'manifest.json').write_text(json.dumps(report,indent=2));print(json.dumps({k:report[k] for k in ['kind','raw_bytes','raw_sha256','dense_rows','seconds']}))
