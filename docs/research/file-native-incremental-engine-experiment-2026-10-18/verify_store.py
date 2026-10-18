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
    for row in receipts:
        i=row['index'];s,p=chronological[i]
        assert p['request']==f'history-{i:06}' and row['selected']==s
        actual=json.loads(checked(root/f"results/{p['result_hash']}.json",p['result_hash']))
        assert actual==row['result'] and actual['outcome']=='accepted'
        assert p['graph']['count']==16 and p['graph']['cost']==0
        assert row['category']==['maintenance','content','metadata','equal-content','restore'][i%5]
        c=json.loads((root/f"attempts/{p['attempt']}/candidate.json").read_bytes())
        assert sha(json.dumps(c['request'],indent=2,ensure_ascii=False).encode())==p['intent']
        assert c['graph']==p['graph'] and c['projection']==p['projection']
        assert c['request']['request']==f'history-{i:06}' and c['request']['base']==chronological[i-1][0]
        ops=c['request']['ops']
        if i%5==0:assert ops==[]
        elif i%5 in [1,4]:assert len(ops)==1 and ops[0]['kind']=='import' and ops[0]['id']=='d000000' and ops[0]['text'].encode()==(variant if i%5==1 else original)
        elif i%5==2:assert len(ops)==1 and ops[0]['kind']=='label' and ops[0]['id']==f'd{i%16:06}' and ops[0]['text']==f'history-{i%16}-{(i//80)%2}'
        else:assert len(ops)==1 and ops[0]['kind']=='cost' and ops[0]['value']==0
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
