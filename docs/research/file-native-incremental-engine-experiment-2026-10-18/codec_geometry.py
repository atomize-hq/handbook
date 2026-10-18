"""Independent bounded comparison of final control maps and page geometry."""
import argparse,json,hashlib
from pathlib import Path
from verify_store import Tree

class JsonTree(Tree):
    def node(self,h):
        n=json.loads(self.obj(h))
        if n['kind']=='leaf':
            assert set(n)=={'kind','pairs'} and 0<len(n['pairs'])<=64
            pairs=[]
            for k,v in n['pairs']:
                assert isinstance(k,str) and len(k.encode())<=1024 and len(v)<=8192 and all(type(x) is int and 0<=x<=255 for x in v)
                pairs.append((k,bytes(v)))
            assert all(pairs[i][0]<pairs[i+1][0] for i in range(len(pairs)-1));return ('leaf',pairs)
        assert set(n)=={'kind','height','children'} and n['kind']=='branch' and 0<n['height']<=63 and 2<=len(n['children'])<=32
        children=[]
        for child in n['children']:
            assert set(child)=={'id','min','max'} and child['min']<=child['max'];children.append((child['id'],child['min'],child['max']))
        assert all(children[i][2]<children[i+1][1] for i in range(len(children)-1));return ('branch',n['height'],children)

def run(base):
    rows=[]
    for n in [1000,10000]:
        maps=[]
        for codec,cls in [('json',JsonTree),('compact',Tree)]:
            root=base/f'{codec}-{n}';selected=json.loads((root/'SELECTED.json').read_bytes());pub=(root/f"publications/{selected['publication']}.json").read_bytes()
            assert hashlib.sha256(pub).hexdigest()==selected['digest'];candidate=json.loads(pub)['candidate'];state=(root/candidate['state']).read_bytes()
            assert hashlib.sha256(state).hexdigest()==candidate['state_digest'];descriptor=json.loads(state)
            tree=cls(root);entries=tree.entries(descriptor['root']);assert len(entries)==n;maps.append(entries)
            rows.append({'n':n,'codec':codec,'stats':tree.stats(),'reachable_page_bytes':sum((root/'objects'/h[:2]/h).stat().st_size for h in tree.seen),'descriptor':descriptor})
        assert maps[0]==maps[1]
    return {'document_maps_equal':True,'observations':rows}

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--base',required=True);a.add_argument('--output',required=True);ns=a.parse_args()
    Path(ns.output).write_text(json.dumps(run(Path(ns.base)),indent=2))
