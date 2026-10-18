"""Independent selected-chain and old/final exact-byte check for scattered imports."""
import argparse,hashlib,json
from pathlib import Path
from verify_store import Tree,checked
from finish_matrix import bounded_source

def point(tree,h,key):
    while h:
        node=tree.node(h)
        if node[0]=='leaf':
            values=dict(node[1]);value=values.get(key)
            if not value:return None
            if value.startswith(b'@'):value=tree.obj(value[1:].decode())
            return json.loads(value)
        h=next((child for child,lo,hi in node[2] if lo<=key<=hi),None)
    return None

def run(matrix):
    rows=[]
    for n in [1000,10000,100000]:
        root=matrix/f'n{n}/disposable/store';selected=json.loads((root/'SELECTED.json').read_bytes());found={};chain=0
        while selected:
            chain+=1;assert chain<=1000
            pub=json.loads(checked(root/f"publications/{selected['publication']}.json",selected['digest']))
            if pub['request'].startswith(f'genuine-{n}-'):found[int(pub['request'].split('-')[-1])]=pub
            if pub['request']=='combined-packet':
                manifest=(root/f"attempts/{pub['packet_ref']['attempt']}/packet/MANIFEST.json").read_bytes()
                assert hashlib.sha256(manifest).hexdigest()==pub['packet_ref']['manifest']
                rows.append({'operation':'combined-packet','selected_membership_authenticated':selected,'packet_manifest':pub['packet_ref']['manifest']})
            selected=pub['predecessor']
        for k in [1,8,64]:
            pub=found.get(k)
            if not pub:rows.append({'n':n,'k':k,'unperformed_or_not_selected':True});continue
            prior=pub['predecessor'];before=json.loads(checked(root/f"publications/{prior['publication']}.json",prior['digest']))
            ids=[f'd{(i*n//k+17)%n:06}' for i in range(k)];tree=Tree(root)
            for id in ids:
                old=point(tree,before['graph']['tree'],'D/'+id)['doc'];new=point(tree,pub['graph']['tree'],'D/'+id)['doc']
                expected=bounded_source(id,k).encode();assert old['digest']!=new['digest'] and new['digest']==hashlib.sha256(expected).hexdigest()
                assert (root/new['original']).read_bytes()==expected
            rows.append({'n':n,'k':k,'selected':pub['id'],'distinct_source_digests_verified':k,'exact_4096_byte_sources_verified':k})
    return rows

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--matrix',required=True);a.add_argument('--output',required=True);ns=a.parse_args()
    Path(ns.output).write_text(json.dumps(run(Path(ns.matrix)),indent=2))
