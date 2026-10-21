"""Exact ordinary source/destination comparison; never imports engine/pack code."""
import os,sys,json,hashlib,stat
from pathlib import Path
out=Path(sys.argv[1]);route=json.loads((out/'route.json').read_bytes());source=Path(route['generation'])
assert not (source/'route.json').exists(),'routing-sidecar-source-collision'
def manifest(root,sidecar=False):
 rows=[]
 for b,ds,ns in os.walk(root,followlinks=False):
  for n in ds:assert not (Path(b)/n).is_symlink(),'ordinary-directory-alias'
  for n in ns:
   p=Path(b)/n
   if sidecar and p==out/'route.json':continue
   s=p.lstat();assert stat.S_ISREG(s.st_mode),'ordinary-file-alias';h=hashlib.sha256()
   with p.open('rb') as f:
    for data in iter(lambda:f.read(1048576),b''):h.update(data)
   rows.append({'path':p.relative_to(root).as_posix(),'bytes':s.st_size,'sha256':h.hexdigest()})
 return sorted(rows,key=lambda r:r['path'])
a=manifest(source);b=manifest(out,True);assert a==b,'ordinary-copy-not-exact';route['files']=b;route['selection_reads']=1;route['engine_or_pack_decoder_used']=False;print(json.dumps(route,separators=(',',':')))
