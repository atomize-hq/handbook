"""Independent bounded full walker/C1 decoder. Never used on timed normal paths."""
import hashlib,json,struct,copy
from pathlib import Path
def sha(b):return hashlib.sha256(b).hexdigest()
def checked(path,h):
    assert path.stat().st_size<=128*1024*1024
    b=path.read_bytes();assert sha(b)==h,(str(path),h);return b
class Tree:
    def __init__(self,root):self.root=Path(root);self.pages=0;self.leaves=0;self.branches=0;self.physical=0;self.tombstones=0;self.height=0;self.occupancy=[];self.seen=set()
    def obj(self,h):
        assert len(h)==64 and all(c in '0123456789abcdef' for c in h)
        p=self.root/'objects'/h[:2]/h;assert p.stat().st_size<=1024*1024
        return checked(p,h)
    def node(self,h):
        b=self.obj(h);assert b[:4]==b'FNC1';p=4
        def take(n):
            nonlocal p
            assert n>=0 and p+n<=len(b)
            v=b[p:p+n];p+=n;return v
        def integer(fmt):return struct.unpack(fmt,take(struct.calcsize(fmt)))[0]
        def key():
            n=integer('<H');assert n<=1024;return take(n).decode()
        tag=integer('<B')
        if tag==0:
            n=integer('<H');assert 0<n<=64;rows=[]
            for _ in range(n):
                k=key();size=integer('<I');assert size<=8192;rows.append((k,take(size)))
            assert all(rows[i][0]<rows[i+1][0] for i in range(n-1))
            node=('leaf',rows)
        else:
            assert tag==1;height=integer('<B');n=integer('<H');assert 0<height<=63 and 2<=n<=32
            children=[(take(32).hex(),key(),key()) for _ in range(n)]
            assert all(a<=z for _,a,z in children)
            assert all(children[i][2]<children[i+1][1] for i in range(n-1))
            node=('branch',height,children)
        assert p==len(b);return node
    def entries(self,h):
        out={}
        def walk(h,expected=None):
            assert h not in self.seen;self.seen.add(h);self.pages+=1;n=self.node(h)
            if n[0]=='leaf':
                self.leaves+=1;self.occupancy.append(len(n[1]));self.physical+=len(n[1]);height=0;lo=n[1][0][0];hi=n[1][-1][0]
                for k,v in n[1]:
                    assert k not in out
                    if not v:self.tombstones+=1;continue
                    if v.startswith(b'@'):v=self.obj(v[1:].decode())
                    out[k]=json.loads(v)
            else:
                self.branches+=1;height=n[1];self.height=max(self.height,height);lo=n[2][0][1];hi=n[2][-1][2]
                for child,a,z in n[2]:walk(child,(height-1,a,z))
            if expected is not None:assert (height,lo,hi)==expected
        if h:walk(h)
        return out
    def stats(self):return {k:getattr(self,k) for k in ['pages','leaves','branches','physical','tombstones','height','occupancy']}

class CachedPoints(Tree):
    """Post-run graph-header proof; immutable page hashes permit checked reuse."""
    def __init__(self,root):super().__init__(root);self.cache={}
    def point(self,h,key):
        expected=None
        while h:
            if h not in self.cache:self.cache[h]=self.node(h)
            node=self.cache[h]
            bounds=(0,node[1][0][0],node[1][-1][0]) if node[0]=='leaf' else (node[1],node[2][0][1],node[2][-1][2])
            if expected is not None:assert bounds==expected
            if node[0]=='leaf':
                value=dict(node[1]).get(key)
                if not value:return None
                if value.startswith(b'@'):value=self.obj(value[1:].decode())
                return json.loads(value)
            child=next(((child,lo,hi) for child,lo,hi in node[2] if lo<=key<=hi),None)
            if child is None:return None
            h,lo,hi=child;expected=(node[1]-1,lo,hi)
        return None
def verify_history(root,allow_censored_prefix=False):
    root=Path(root);selected=json.loads((root/'SELECTED.json').read_bytes());next=selected;chain=[];seen=set()
    while next:
        assert len(chain)<200001 and next['publication'] not in seen;seen.add(next['publication'])
        p=json.loads(checked(root/f"publications/{next['publication']}.json",next['digest']))
        assert p['id']==next['publication']
        body=json.loads(checked(root/f"results/{p['result_hash']}.json",p['result_hash']))
        assert body['outcome']=='accepted' and body['request']==p['request'] and body['publication']==p['id']
        chain.append((next,p));next=p['predecessor']
    for i,(_,p) in enumerate(reversed(chain)):assert p['sequence']==i
    head=chain[0][1];t=Tree(root);members=t.entries(head['membership']);membership_stats=t.stats()
    expected={s['publication']:s['digest'] for s,p in chain[1:]};assert members==expected,'missing/extra selected membership'
    t=Tree(root);outcomes=t.entries(head['outcomes']);outcome_stats=t.stats()
    expected={}
    for s,p in chain[1:]:
        key=sha(p['request'].encode());assert key not in expected,'duplicate accepted request'
        expected[key]={'request':p['request'],'intent':p['intent'],'selected':s,'result_hash':p['result_hash'],'attempt':p['attempt']}
    assert outcomes==expected,'missing/extra/original-outcome mismatch'
    ledger=root/'expected-ledger.jsonl';receipts=[json.loads(l) for l in ledger.read_bytes().splitlines()] if ledger.exists() else []
    chronological=list(reversed(chain))
    history_rows=[p for _,p in chronological if p['request'].startswith('history-') and p['request']!='history-checkpoint']
    assert [p['request'] for p in history_rows]==[f'history-{i:06}' for i in range(1,len(history_rows)+1)]
    assert [row['index'] for row in receipts]==list(range(1,len(receipts)+1))
    if not allow_censored_prefix:assert len(receipts)==len(history_rows),'ledger does not cover every claimed history selection'
    else:assert len(receipts)<=len(history_rows)<=len(receipts)+1
    if history_rows:
        origin=json.loads((root/'HISTORY-SOURCE.json').read_bytes())
        original=checked(root/origin['original'],origin['digest']);variant=original[:-1]+(b'\n' if original[-1:]==b' ' else b' ')
    points=CachedPoints(root);source_proofs=set()
    for row in receipts:
        i=row['index'];s,p=chronological[i]
        assert p['request']==f'history-{i:06}' and row['selected']==s
        actual=json.loads(checked(root/f"results/{p['result_hash']}.json",p['result_hash']))
        assert actual==row['result'] and actual['outcome']=='accepted'
        assert p['graph']['count']==16 and p['graph']['cost']==0
        assert row['category']==['maintenance','content','metadata','equal-content','restore'][i%5]
        # Accepted evidence outlives retireable attempt files. Reconstruct the
        # exact bounded recipe independently and bind it to the selected intent.
        ops=[]
        if i%5:
            kind={1:'import',2:'label',3:'cost',4:'import'}[i%5]
            text=(variant if i%5==1 else original).decode() if i%5 in [1,4] else f'history-{i%16}-{(i//80)%2}' if i%5==2 else ''
            ops=[dict(kind=kind,id='d000000' if i%5 in [1,4] else f'd{i%16:06}',text=text,target='',value=0)]
        base=chronological[i-1][0]
        request=dict(request=f'history-{i:06}',base=dict(publication=base['publication'],digest=base['digest']),ops=ops,packet=False)
        assert sha(json.dumps(request,indent=2,ensure_ascii=False).encode())==p['intent'],'independent recipe intent mismatch'
        candidate=root/f"attempts/{p['attempt']}/candidate.json"
        if candidate.exists():
            c=json.loads(candidate.read_bytes())
            assert c['request']==request and c['graph']==p['graph'] and c['projection']==p['projection']
        previous=chronological[i-1][1]
        if i%5 in [1,4]:
            data=variant if i%5==1 else original;digest=sha(data)
            before=points.point(previous['graph']['tree'],'D/d000000')['doc'];after=points.point(p['graph']['tree'],'D/d000000')['doc']
            assert before['digest']!=after['digest'] and after['digest']==digest and after['bytes']==len(data),'content/restore was not a genuine exact-byte mutation'
            if after['original'] not in source_proofs:
                assert checked(root/after['original'],digest)==data;source_proofs.add(after['original'])
        elif i%5==2:
            key=f'D/d{i%16:06}';before=points.point(previous['graph']['tree'],key)['doc'];after=points.point(p['graph']['tree'],key)['doc']
            assert before['label']!=after['label'] and after['label']==ops[0]['text'],'metadata label was not a genuine mutation'
        else:
            assert p['graph']==previous['graph'] and p['projection']==previous['projection'],'maintenance/equal-state changed semantic or projection state'
    recipe_verified=False
    initial=root/'INITIAL-ORACLE.json'
    if initial.exists():
        state=json.loads(initial.read_bytes());docs=copy.deepcopy(state['docs'])
        for i in range(1,len(history_rows)+1):
            if i%5 in [1,4]:
                data=variant if i%5==1 else original;h=sha(data);doc=docs['d000000']
                doc.update(digest=h,bytes=len(data),original=f"originals/{h[:2]}/{h}-{Path(doc['path']).name}")
            elif i%5==2:docs[f'd{i%16:06}']['label']=f'history-{i%16}-{(i//80)%2}'
        expected={}
        for id,doc in docs.items():
            edges=doc['edges'];header=copy.deepcopy(doc);header['edges']=[]
            expected['D/'+id]={'doc':header,'next':len(edges)}
            expected['P/'+doc['path'].lower()]=id
            if doc['label'] is not None:expected['L/'+doc['label']]=id
            for ordinal,edge in enumerate(edges):
                expected[f'E/{id}/{ordinal:016x}']=edge
                if not edge['external']:expected[f"R/{edge['target']}/{id}/{ordinal:016x}"]=edge
            assert sha((root/doc['original']).read_bytes())==doc['digest']
        actual=Tree(root).entries(head['graph']['tree']);assert actual==expected,'independent complete history recipe/index mismatch'
        recipe_verified=True
    return {'publications':len(chain),'ledger_boundaries':len(receipts),'membership':membership_stats,'outcomes':outcome_stats,'independent_full_walker':True,'independent_recipe_and_all_indexes':recipe_verified}
