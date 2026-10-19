"""Bounded independent selected-history/control/source/index verification.

All accepted membership/outcome entries are compared. Reuse is only for exactly
equal immutable root identities; Tree still verifies every parent/child binding.
"""
import argparse,hashlib,json,sys,time,os
from concurrent.futures import ThreadPoolExecutor
from collections import deque
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'file-native-incremental-engine-experiment-2026-10-18'))
from verify_store import Tree

def sha(b):return hashlib.sha256(b).hexdigest()
def checked(path,h):
    with Path(path).open('rb') as f:
        assert os.fstat(f.fileno()).st_size<=128*1024*1024
        b=f.read()
    assert sha(b)==h,(str(path),h);return b

def verify(root,baseline=None):
    root=Path(root);start=time.monotonic();config=json.loads((root/'STORE.json').read_bytes());selected=json.loads((root/'SELECTED.json').read_bytes());s=selected;chain={};outcomes={};content=set();results={};sources=set();roots=0;permanent=hashlib.sha256();retired=[];packets=set();generations=[]
    base=json.loads(Path(baseline).read_bytes()) if baseline else None;tail=hashlib.sha256();tail_count=0;tail_started=False
    expected_sequence=None
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
    reservations=json.loads((root/'RESERVATIONS.json').read_bytes()) if (root/'RESERVATIONS.json').exists() else {}
    for attempt,r in reservations.items():
        if r['state']=='abandoned':continue
        if r['state']!='sealed':continue # partial bytes preserved, recovery needs fencing
        b=checked(root/f'attempts/{attempt}/candidate.json',r['candidate_digest']);c=json.loads(b)
        assert c['attempt']==attempt and c['intent']==r['intent'] and c['request']['request']==r['request'] and c['expected']==r['base']
        if c['expected']:assert chain[c['expected']['publication']]==c['expected']
        content.add((c['graph']['tree'],c['projection']))
        for h in c['replay_objects']:checked(root/'objects'/h[:2]/h,h)
        for path,h in c['replay_files'].items():checked(root/path,h)
        Tree(root).entries(c['changes'])
        if c['checkpoint']:
            Tree(root).entries(c['checkpoint']['membership']);Tree(root).entries(c['checkpoint']['outcomes'])
        if c['packet_ref']:packets.add((c['packet_ref']['attempt'],c['packet_ref']['manifest']))
    # Full independent graph indexes from exact header + ordered-edge identities.
    for graph,projection in sorted(content,key=str):
        entries=Tree(root).entries(graph);expected={};docs={k[2:]:v for k,v in entries.items() if k.startswith('D/')}
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
        for path,f in manifest.items():checked(base/path,f['digest'])
    return dict(profile='all-accepted-membership-outcomes-retained-graph-projection-sources-controls',selected=selected,publications=len(chain),ledger_rows=ledger_rows,permanent_digest=permanent_digest,retired_publications=len(retired),retired_content='unavailable-by-contract' if retired else 'none',unique_content_roots=roots,root_visits=len(chain),sources=len(sources),pins=len(pins),reservations=len(reservations),packets=len(packets),generations=len(generations),seconds=time.monotonic()-start)

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--root',required=True);a.add_argument('--baseline');a.add_argument('--output',required=True);ns=a.parse_args();result=verify(ns.root,ns.baseline);Path(ns.output).write_text(json.dumps(result,indent=2),encoding='utf-8');print(json.dumps(result,indent=2))
