"""Portable selected-history proof capsule, not a complete mutable-store backup."""
import argparse,base64,gzip,hashlib,json
from pathlib import Path,PurePosixPath
from verify_store import Tree,verify_history

def sha(b):return hashlib.sha256(b).hexdigest()
def export(root,destination):
    root=Path(root);destination=Path(destination);destination.mkdir(parents=True,exist_ok=False)
    verified=verify_history(root,allow_censored_prefix=True);selected=json.loads((root/'SELECTED.json').read_bytes());head=json.loads((root/f"publications/{selected['publication']}.json").read_bytes())
    paths=set();roots=set();s=selected;publications=0
    while s:
        path=root/f"publications/{s['publication']}.json";p=json.loads(path.read_bytes());paths.add(path);publications+=1
        paths.add(root/f"results/{p['result_hash']}.json")
        candidate=root/f"attempts/{p['attempt']}/candidate.json"
        if candidate.exists():paths.add(candidate)
        hint=root/f"attempt-hints/{p['attempt']}.json"
        if hint.exists():paths.add(hint)
        roots.update([p['graph']['tree'],p['projection']]);s=p['predecessor']
    roots.update([head['membership'],head['outcomes'],head['changes']]);used=set()
    class Captured(Tree):
        def obj(self,h):used.add(h);return super().obj(h)
    for h in roots:
        if not h:continue
        entries=Captured(root).entries(h)
        for key,value in entries.items():
            if key.startswith('D/') and isinstance(value,dict) and 'doc' in value:paths.add(root/value['doc']['original'])
            if isinstance(value,dict) and set(value)=={'source','digest'}:paths.add(root/value['source'])
    paths.update(root/'objects'/h[:2]/h for h in used)
    for name in ['STORE.json','SELECTED.json','HISTORY-SOURCE.json','INITIAL-ORACLE.json','expected-ledger.jsonl']:
        if (root/name).exists():paths.add(root/name)
    chunks=[];stream=None;file_count=0;raw_size=0;chunk_raw=0
    def write(row):
        nonlocal stream,chunk_raw
        b=json.dumps(row,separators=(',',':')).encode()+b'\n'
        if stream is None or chunk_raw+len(b)>64*1024*1024:
            if stream:stream.close()
            path=destination/f'part-{len(chunks):03}.jsonl.gz';chunks.append(path);stream=gzip.GzipFile(filename=str(path),mode='wb',mtime=0);chunk_raw=0
        stream.write(b);chunk_raw+=len(b)
    for path in sorted(paths):
        relative=path.relative_to(root).as_posix();offset=0;digest=hashlib.sha256()
        with path.open('rb') as f:
            while True:
                b=f.read(32*1024*1024)
                if not b:break
                digest.update(b);write({'path':relative,'offset':offset,'sha256':sha(b),'data':base64.b64encode(b).decode()});offset+=len(b)
        write({'path':relative,'end_sha256':digest.hexdigest(),'total_bytes':offset});file_count+=1;raw_size+=offset
    if stream:stream.close()
    manifest={'kind':'selected-history-proof-not-full-backup','selected':selected,'verification':verified,'publications':publications,'files':file_count,'raw_file_bytes':raw_size,
        'chunks':[{'path':p.name,'bytes':p.stat().st_size,'sha256':sha(p.read_bytes())} for p in chunks]}
    (destination/'manifest.json').write_text(json.dumps(manifest,indent=2));return manifest

def verify(capsule,restore=None):
    capsule=Path(capsule);manifest=json.loads((capsule/'manifest.json').read_bytes());current=None;digest=None;offset=0;files=0;out=None
    if restore:restore=Path(restore);restore.mkdir(parents=True,exist_ok=False)
    for part in manifest['chunks']:
        path=capsule/part['path'];assert path.stat().st_size==part['bytes'] and sha(path.read_bytes())==part['sha256']
        with gzip.open(path,'rb') as f:
            for line in f:
                row=json.loads(line);relative=PurePosixPath(row['path']);assert not relative.is_absolute() and '..' not in relative.parts and '\\' not in row['path']
                if current is None:
                    current=row['path'];digest=hashlib.sha256();offset=0
                    if restore:
                        target=restore/row['path'];target.parent.mkdir(parents=True,exist_ok=True);out=target.open('xb')
                assert current==row['path']
                if 'data' in row:
                    b=base64.b64decode(row['data'],validate=True);assert sha(b)==row['sha256'] and offset==row['offset'];digest.update(b);offset+=len(b)
                    if out:out.write(b)
                else:
                    assert offset==row['total_bytes'] and digest.hexdigest()==row['end_sha256'];files+=1;current=None
                    if out:out.close();out=None
    assert current is None and files==manifest['files']
    if restore:assert verify_history(restore,allow_censored_prefix=True)==manifest['verification']
    return {'files':files,'lossless_bytes_verified':True,'restored_independent_history_verified':bool(restore)}

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--root');a.add_argument('--destination');a.add_argument('--verify');a.add_argument('--restore');ns=a.parse_args()
    print(json.dumps(verify(ns.verify,ns.restore) if ns.verify else export(ns.root,ns.destination),indent=2))
