"""Byte-preserve a quiescent, owned development fixture before staged cleanup."""
import argparse,hashlib,json,os,tarfile,time
from pathlib import Path
from census import census
def digest(path):
    h=hashlib.sha256()
    with path.open('rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
    return h.hexdigest()
def run(root,out):
    root=Path(root).resolve();out=Path(out).resolve();owned=Path('C:/file-native-navigation-20261019').resolve()
    assert root.is_relative_to(owned) and root.name=='focused-first' and not out.is_relative_to(root)
    count=census(root);assert not count['links_excluded'] and count['logical_bytes']<128*1024**2
    inputs={p.relative_to(root.parent).as_posix():(p.stat().st_size,digest(p)) for p in root.rglob('*') if p.is_file()}
    with tarfile.open(out,'w:gz',compresslevel=1) as archive:archive.add(root,arcname=root.name)
    actual={}
    with tarfile.open(out) as archive:
        for member in archive:
            if member.isdir():continue
            assert member.isfile() and member.name not in actual
            h=hashlib.sha256();size=0
            with archive.extractfile(member) as stream:
                for b in iter(lambda:stream.read(1024*1024),b''):h.update(b);size+=len(b)
            actual[member.name]=(size,h.hexdigest())
    assert actual==inputs
    result=dict(status='byte-verified-development-archive',source=str(root),qualification='superseded development fixture; no operational-restore claim',files=count['files'],logical_bytes=count['logical_bytes'],archive=str(out),archive_bytes=out.stat().st_size,archive_sha256=digest(out),inventory_sha256=hashlib.sha256(json.dumps(inputs,sort_keys=True).encode()).hexdigest())
    out.with_suffix('.json').write_text(json.dumps(result,indent=2),encoding='utf-8');print(json.dumps(result,indent=2))
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--root',required=True);p.add_argument('--out',required=True);a=p.parse_args();run(a.root,a.out)
