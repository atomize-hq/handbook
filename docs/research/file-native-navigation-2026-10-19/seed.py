"""Authenticate a selected fixture's source corpus for bounded fixture setup."""
import argparse,hashlib,json,tarfile,time
from pathlib import Path
from oracle import Tree,verify
def run(root,out):
    root=Path(root).resolve();out=Path(out).resolve();start=time.monotonic();proof=verify(root)
    sel=json.loads((root/'SELECTED.json').read_bytes());pub=json.loads((root/f"publications/{sel['publication']}.json").read_bytes());g=Tree(root).entries(pub['graph']['tree'])
    sources={h['doc']['original']:dict(bytes=h['doc']['bytes'],sha256=h['doc']['digest']) for k,h in g.items() if k.startswith('D/')}
    assert len(sources)==10000 and sum(v['bytes'] for v in sources.values())==126976000
    with tarfile.open(out,'w:gz',compresslevel=1) as archive:
        for p in sorted(sources):archive.add(root/p,arcname=p,recursive=False)
    observed={}
    with tarfile.open(out) as archive:
        for member in archive:
            assert member.isfile() and member.name in sources and member.name not in observed
            raw=archive.extractfile(member).read();observed[member.name]=dict(bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest())
    assert observed==sources
    result=dict(source=str(root),selected=sel,graph=pub['graph'],proof=proof,files=len(sources),logical_bytes=sum(v['bytes'] for v in sources.values()),transport_bytes=out.stat().st_size,transport_sha256=hashlib.sha256(out.read_bytes()).hexdigest(),seconds=time.monotonic()-start,inventory=sources)
    out.with_suffix('.manifest.json').write_text(json.dumps(result,indent=2),encoding='utf-8');print(json.dumps({k:v for k,v in result.items() if k!='inventory'},indent=2))
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--root',required=True);p.add_argument('--out',required=True);a=p.parse_args();run(a.root,a.out)
