"""Hash the standalone experiment files; this is a report inventory, not a store."""
import argparse,hashlib,json,os,time
from pathlib import Path

a=argparse.ArgumentParser();a.add_argument('--output',required=True);a.add_argument('--verify',action='store_true');ns=a.parse_args()
root=Path(__file__).resolve().parent;output=Path(ns.output).resolve()
assert output.is_relative_to(root) and output!=Path(__file__).resolve()
rows=[]
for directory,children,names in os.walk(root):
 children[:]=sorted(n for n in children if n not in {'.git','__pycache__','target'})
 for name in sorted(names):
  path=Path(directory)/name
  if path.resolve()==output:continue
  assert not path.is_symlink(),path
  raw=path.read_bytes();rows.append(dict(path=path.relative_to(root).as_posix(),bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest()))
rows.sort(key=lambda row:row['path'])
if ns.verify:
 saved=json.loads(output.read_bytes());assert saved['files']==rows,'Inventory membership, length or digest changed'
else:
 assert output.parent.is_dir()
 saved=dict(created_epoch=time.time(),files=rows,total_file_bytes=sum(row['bytes'] for row in rows),qualification='Snapshot of standalone source, documents and retained evidence. Excludes this inventory itself, Python caches, Git metadata and build targets. Hash equality authenticates these recorded bytes, not their semantic claims or executable reproducibility. Recreate after final document/artifact changes.')
 output.write_text(json.dumps(saved,indent=2))
print(json.dumps(dict(verified=ns.verify,files=len(rows),total_file_bytes=sum(row['bytes'] for row in rows))))
