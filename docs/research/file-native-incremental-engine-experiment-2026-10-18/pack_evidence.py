"""Lossless deterministic raw-observation export with independent byte/type checks."""
import argparse,base64,gzip,hashlib,json,zipfile
from pathlib import Path

def sha(b):return hashlib.sha256(b).hexdigest()
def typed(a,b):
    assert type(a) is type(b),(type(a).__name__,type(b).__name__)
    if isinstance(a,dict):
        assert a.keys()==b.keys()
        for k in a:typed(a[k],b[k])
    elif isinstance(a,list):
        assert len(a)==len(b)
        for x,y in zip(a,b):typed(x,y)
    else:assert a==b

def export(source,destination):
    source=Path(source);destination=Path(destination);destination.mkdir(parents=True,exist_ok=False);rows=[];events=0
    names={'events.jsonl','session.json','session-source.zip','completed.json','environment.json','outer.json','finished.json','checks.json','history.json','matrix.json','summary.json','stage1.json','differential.json','supplement.json','adversarial.json','deep-cases.json','walk-cases.json','adapter.py','deep_cases.py','walk_cases.py','namespace_cases.py','namespace-cases.json'}
    def put(path,relative):
        nonlocal events
        b=path.read_bytes();entry={'path':relative.as_posix(),'bytes':len(b),'sha256':sha(b)}
        if path.name=='events.jsonl':
            for line in b.splitlines():
                row=json.loads(line);raw=base64.b64decode(row['stdout_b64']);assert sha(raw)==row['stdout_sha256'] and len(raw)==row['stdout_bytes']
                if row['output'] is not None:typed(json.loads(raw),row['output'])
                assert type(row['censored']) is bool and type(row['exit']) is int
                events+=1
        encoded=gzip.compress(b,compresslevel=9,mtime=0);assert gzip.decompress(encoded)==b
        target=destination/(relative.as_posix()+'.gz');target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes(encoded)
        entry.update(archive=target.relative_to(destination).as_posix(),archive_bytes=len(encoded),archive_sha256=sha(encoded));rows.append(entry)
    # Prune data/build trees rather than recursively reading every store file.
    def visit(path):
        for child in sorted(path.iterdir()):
            if child.is_dir():
                if child.name in {'store','build','target','.git','__pycache__','disposable','fallback'} or child.name.startswith(('B-n','n100','json-','compact-')):continue
                visit(child)
            elif child.name in names or child.suffix in {'.stdout','.stderr'}:put(child,child.relative_to(source))
    visit(source)
    report={'source':str(source),'lossless_bytes_verified':True,'json_types_verified':True,'events':events,'files':rows}
    (destination/'manifest.json').write_text(json.dumps(report,indent=2));return report

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--source',required=True);a.add_argument('--destination',required=True);ns=a.parse_args()
    v=export(ns.source,ns.destination);print(json.dumps({'files':len(v['files']),'events':v['events']}))
