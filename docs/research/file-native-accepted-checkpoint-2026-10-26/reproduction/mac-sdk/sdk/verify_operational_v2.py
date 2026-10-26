"""Explicit quota-v2 branch derived from protected verifier d86c7d54; V1 unchanged."""
import argparse,hashlib,json,sys,time,os,copy
from concurrent.futures import ThreadPoolExecutor
from collections import deque
from pathlib import Path
from outcome_oracle import PRIOR
sys.path.insert(0,str(PRIOR/'file-native-incremental-engine-experiment-2026-10-18'))
from verify_store import Tree

def sha(b):return hashlib.sha256(b).hexdigest()
def checked(path,h):
    with Path(path).open('rb') as f:
        assert os.fstat(f.fileno()).st_size<=128*1024*1024
        b=f.read()
    assert sha(b)==h,(str(path),h);return b

def verify(root,baseline=None,*,quota):
    assert quota['version']==2 and quota['semantic_owners'] is not None
    root=Path(root);start=time.monotonic();config=json.loads((root/'STORE.json').read_bytes());selected=json.loads((root/'SELECTED.json').read_bytes());s=selected;chain={};outcomes={};content=set();results={};sources=set();roots=0;permanent=hashlib.sha256();retired=[];packets=set();generations=[]
    base=json.loads(Path(baseline).read_bytes()) if baseline else None;tail=hashlib.sha256();tail_count=0;tail_started=False
    expected_sequence=None;owner_bytes=0;graph_contracts={}
    pool=ThreadPoolExecutor(max_workers=4);pending=deque()
    def result_body(s,p):
        body=json.loads(checked(root/f"results/{p['result_hash']}.json",p['result_hash']))
        assert body['outcome']=='accepted' and body['request']==p['request'] and body['publication']==p['id'] and body['state_digest']==sha(json.dumps(p['graph'],indent=2,ensure_ascii=False).encode())
        return p['request'],(s,body)
    def finish_one():
        request,value=pending.popleft().result();assert request not in results;results[request]=value
    while s:
        assert len(chain)<200001 and s['publication'] not in chain
        p=json.loads(checked(root/f"publications/{s['publication']}.json",s['digest']));assert p['store_id']==config['store_id'] and p['id']==s['publication'] and p['version']==1
        owner=p['allocation_owner'];assert owner['attempt']==p['attempt'] and owner['includes_intermediates_and_stages'];owner_bytes+=owner['conservative_bytes']
        g=p['graph'];assert g['version']==1 and g['consumer']==config['consumer'];graph_contracts.setdefault(g['tree'],set()).add((g['count'],g['cost'],g['budget']))
        if expected_sequence is not None:assert p['sequence']==expected_sequence
        pending.append(pool.submit(result_body,s,p))
        if len(pending)>=256:finish_one()
        evidence=json.dumps([s,p['result_hash']],sort_keys=True,separators=(',',':')).encode()+b'\n';permanent.update(evidence)
        if base and s==base['selected']:tail_started=True
        if tail_started:tail.update(evidence);tail_count+=1
        chain[p['id']]=s
        if s!=selected:
            key=sha(p['request'].encode());assert key not in outcomes
            outcomes[key]=dict(request=p['request'],intent=p['intent'],selected=s,result_hash=p['result_hash'],attempt=p['attempt'])
        elif len(chain)==1:head=p
        marker=root/f"retired/{p['id']}.json"
        if marker.exists():assert json.loads(marker.read_bytes())==s;retired.append(p['id'])
        else:
            content.add((p['graph']['tree'],p['projection']))
            if p['packet_ref']:packets.add((p['packet_ref']['attempt'],p['packet_ref']['manifest']))
            if config['mode']=='B':generations.append((p['attempt'],p['generation_manifest'],p['projection']))
        expected_sequence=p['sequence']-1
        s=p['predecessor']
        if len(chain)%10000==0:print(json.dumps(dict(phase='verify-chain',publications=len(chain),seconds=time.monotonic()-start)),flush=True)
    assert expected_sequence==-1
    while pending:finish_one()
    pool.shutdown()
    assert selected['publication'] not in retired
    members=Tree(root).entries(head['membership']);assert members=={k:v['digest'] for k,v in chain.items() if k!=selected['publication']}
    actual=Tree(root).entries(head['outcomes']);assert actual==outcomes
    Tree(root).entries(head['changes'])
    permanent_digest=permanent.hexdigest()
    if baseline:
        assert base['permanent_digest']==tail.hexdigest() and base['publications']==tail_count
    ledger=root/'expected-ledger.jsonl';ledger_rows=0
    if ledger.exists():
        with ledger.open('rb') as f:
            for line in f:
                row=json.loads(line);ledger_rows+=1;assert row['index']==ledger_rows
                assert results[row['result']['request']]==(row['selected'],row['result'])
    pins=json.loads((root/'PINS.json').read_bytes()) if (root/'PINS.json').exists() else {}
    for s in pins.values():assert chain[s['publication']]==s and s['publication'] not in retired
    for name in ['VIEW.json','PENDING.json']:
        path=root/name
        if not path.exists():continue
        value=json.loads(path.read_bytes());marks=[value] if name=='VIEW.json' else [value['target']]+([value['prior']] if value['prior'] else [])
        for mark in marks:
            s=mark['selected'];assert chain[s['publication']]==s and s['publication'] not in retired
            p=json.loads(checked(root/f"publications/{s['publication']}.json",s['digest']));assert p['graph']==mark['graph'] and p['projection']==mark['projection'] and p['sequence']==mark['sequence']
            content.add((p['graph']['tree'],p['projection']))
    reservations=quota['semantic_owners'] # validated typed V2 owners, including retained terminal owners
    for attempt,r in reservations.items():
        if r['state']=='abandoned':continue
        if r['state']!='sealed':continue # partial bytes preserved, recovery needs fencing
        b=checked(root/f'attempts/{attempt}/candidate.json',r['candidate_digest']);c=json.loads(b)
        assert c['attempt']==attempt and c['intent']==r['intent'] and c['request']['request']==r['request'] and c['expected']==r['base']
        if c['expected']:assert chain[c['expected']['publication']]==c['expected']
        content.add((c['graph']['tree'],c['projection']))
        g=c['graph'];graph_contracts.setdefault(g['tree'],set()).add((g['count'],g['cost'],g['budget']))
        for h in c['replay_objects']:checked(root/'objects'/h[:2]/h,h)
        for path,h in c['replay_files'].items():checked(root/path,h)
        Tree(root).entries(c['changes'])
        if c['checkpoint']:
            Tree(root).entries(c['checkpoint']['membership']);Tree(root).entries(c['checkpoint']['outcomes'])
        if c['packet_ref']:packets.add((c['packet_ref']['attempt'],c['packet_ref']['manifest']))
        if c['generation_manifest'] is not None:
            assert config['mode']=='B';generations.append((attempt,c['generation_manifest'],c['projection']))
    # Full independent graph indexes from exact header + ordered-edge identities.
    for graph,projection in sorted(content,key=str):
        entries=Tree(root).entries(graph);expected={};docs={k[2:]:v for k,v in entries.items() if k.startswith('D/')}
        assert len({h['doc']['path'].lower() for h in docs.values()})==len(docs)
        labels=[h['doc']['label'] for h in docs.values() if h['doc']['label'] is not None];assert len(set(labels))==len(labels)
        for count,cost,budget in graph_contracts.get(graph,[]):assert len(docs)==count and sum(h['doc']['cost'] for h in docs.values())==cost and cost<=budget
        for id,h in docs.items():
            d=h['doc'];assert d['id']==id and d['edges']==[];expected['D/'+id]=h;expected['P/'+d['path'].lower()]=id
            if d['label'] is not None:expected['L/'+d['label']]=id
            sources.add((d['original'],d['digest']))
        for key,e in entries.items():
            if not key.startswith('E/'):continue
            _,id,n=key.split('/');assert id in docs and int(n,16)<docs[id]['next'];expected[key]=e
            if not e['external']:
                assert e['target'] in docs;expected[f"R/{e['target']}/{id}/{n}"]=e
        assert entries==expected
        for path,f in Tree(root).entries(projection).items():sources.add((f['source'],f['digest']))
        roots+=1
    for path,h in sources:checked(root/path,h)
    for attempt,h in packets:
        base=root/f'attempts/{attempt}/packet';manifest=json.loads(checked(base/'MANIFEST.json',h))
        for path,h in manifest.items():checked(base/path,h)
    for attempt,h,projection in generations:
        base=root/f'generations/{attempt}';manifest=json.loads(checked(base/'MANIFEST.json',h));assert manifest==Tree(root).entries(projection)
        names={p.relative_to(base).as_posix() for p in base.rglob('*') if p.is_file()}-{'MANIFEST.json'}
        assert names==set(manifest),'generation-complete-files'
        for path,f in manifest.items():
            assert path and not path.startswith('/') and '\\' not in path and ':' not in path and all(x not in {'','.','..'} for x in path.split('/'))
            assert {'source','digest'}<=set(f)<={'source','digest','projection_profile'}
            checked(base/path,f['digest'])
    return dict(profile='V2-all-history-graphs-private-owners-generations',selected=selected,publications=len(chain),ledger_rows=ledger_rows,permanent_digest=permanent_digest,retired_publications=len(retired),retired_content='unavailable-by-contract' if retired else 'none',unique_content_roots=roots,root_visits=len(chain),sources=len(sources),pins=len(pins),reservations=len(reservations),packets=len(packets),generations=len(generations),conservative_cumulative_owner_allowance_bytes=owner_bytes,actual_physical_owner_bytes=False,seconds=time.monotonic()-start)

def typed_pretty(v):
    # Only the existing V2 registry/receipt contracts have typed selectors here.
    if type(v) is dict and (('reason' in v and 'expected' in v) or (v.get('version')==2 and 'active' in v)):
        v=copy.deepcopy(v)
        rows=[v] if 'reason' in v else v['active'].values()
        for r in rows:
            key='expected' if 'reason' in r else 'base'
            if r[key] is not None:r[key]={k:r[key][k] for k in ['publication','digest']}
    return json.dumps(v,indent=2,ensure_ascii=False).encode()
