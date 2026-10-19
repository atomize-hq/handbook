"""Exact final-core genesis guidance delta, without repeating the runtime suite."""
import argparse,hashlib,json
from pathlib import Path
from focused import Driver
from oracle import Tree,verify
def run(exe,base):
    d=Driver(exe,base);checks=[];targets=[]
    for mode in ['A','B']:
        root=d.init('new-'+mode,mode,n=16)
        assert not (root/'README.md').exists();checks.append(dict(mode=mode,case='new-profile4-omits-root-markdown',status='success'))
        proof=verify(root);checks.append(dict(mode=mode,case='new-genesis-complete-source-projection',status='success',proof=proof))
        p=json.loads((root/f"publications/{d.head(root)['publication']}.json").read_bytes());g=Tree(root).entries(p['graph']['tree']);doc=g['D/d000006']['doc']
        targets.append(dict(mode=mode,selected=d.head(root),readable=str(root/('view' if mode=='A' else 'generations/'+p['attempt'])),id=doc['id'],source=doc['path'],expected_sha256=doc['digest']))
        root=d.init('old-'+mode,mode,n=16,profile=2);old=(root/'README.md').read_bytes();assert b'collection.yaml' in old
        checks.append(dict(mode=mode,case='legacy-profile2-genesis-keeps-original-guidance',status='success',sha256=hashlib.sha256(old).hexdigest()))
        d.publish(root,'profile4-target',[]);assert (root/'README.md').read_bytes()==old;verify(root)
        checks.append(dict(mode=mode,case='legacy-guidance-byte-exact-after-new-target',status='success'))
    result=dict(core_commit='4bde156323812ef18c0dea6798cfc2108aaad933',exe_sha256=hashlib.sha256(Path(exe).read_bytes()).hexdigest(),checks=checks)
    (d.base/'summary.json').write_text(json.dumps(result,indent=2),encoding='utf-8');(d.base/'shell-targets.json').write_text(json.dumps(targets,indent=2),encoding='utf-8');print(json.dumps(result,indent=2))
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--exe',required=True);p.add_argument('--base',required=True);a=p.parse_args();run(a.exe,a.base)
