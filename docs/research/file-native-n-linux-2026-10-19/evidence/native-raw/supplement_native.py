"""Separately bound coverage gaps: profile3, multipart adjacency, strict evidence, shell copies."""
import json,sys,hashlib,subprocess,time
from pathlib import Path
BASE=Path(__file__).resolve().parent
H=BASE/'source/docs/research/file-native-navigation-2026-10-19'
F=BASE/'source/docs/research/file-native-recovery-reclamation-2026-10-19'
sys.path.insert(0,str(H))
from focused import Driver
from oracle import verify,navigation,Tree
sys.path.insert(0,str(F))
from verify_operational import verify as operational

d=Driver(BASE/'build/release/file-native-incremental',BASE/'supplement-data');rows=[]
for mode in ['A','B']:
    root=d.init('profile3-'+mode,mode,n=16,profile=3)
    old=d.head(root);oldpath=root/f"publications/{old['publication']}.json";oldbytes=oldpath.read_bytes()
    request=dict(request='sealed3',base=old,ops=[dict(kind='rename',id='d000004',text='Sealed profile three')],packet=True)
    candidate=d.call(root,'prepare',profile=3,request=request)['result']['candidate']
    frozen=(root/candidate).read_bytes();c=json.loads(frozen)
    assert Tree(root).entries(c['projection'])['packet/provenance.json']['projection_profile']==3
    d.call(root,'select',profile=4,candidate=candidate);assert (root/candidate).read_bytes()==frozen
    assert oldpath.read_bytes()==oldbytes;verify(root)
    d.call(root,'checkpoint',profile=4,request_id='carry3')
    selected=json.loads((root/f"publications/{d.head(root)['publication']}.json").read_bytes())
    assert Tree(root).entries(selected['projection'])['packet/provenance.json']['projection_profile']==3
    verify(root);d.publish(root,'new4',[],profile=4);verify(root)
    rows.append(dict(case='sealed-profile3-packet-replay-checkpoint-upgrade',mode=mode,old_publication_sha256=hashlib.sha256(oldbytes).hexdigest(),candidate_sha256=hashlib.sha256(frozen).hexdigest(),status='success'))

    root=d.init('multipart-'+mode,mode,n=16);d.call(root,'lifecycle',operation='setup')
    for batch in range(3):
        d.publish(root,'grow-adjacency-'+str(batch),[dict(kind='edge',id='d000002',text='requires' if i%2 else 'cites',target='d000005') for i in range(200)])
    proof=verify(root);selected=json.loads((root/f"publications/{d.head(root)['publication']}.json").read_bytes());p=Tree(root).entries(selected['projection'])
    adj={k:v['digest'] for k,v in p.items() if k.startswith('navigation/relationships/d000002/')};assert len(adj)==3
    tree=root/('view' if mode=='A' else 'generations/'+selected['attempt']);nav=navigation(tree)
    assert len(nav['d000002']['relationships'])==601 and nav['d000002']['relationships'][0]['external']
    assert [x['ordinal'] for x in nav['d000002']['relationships']]==list(range(601))
    d.publish(root,'target-move',[dict(kind='path',id='d000005',text='docs/item-multipart-target-d000005/source.json')]);verify(root)
    selected=json.loads((root/f"publications/{d.head(root)['publication']}.json").read_bytes());p=Tree(root).entries(selected['projection'])
    assert adj=={k:v['digest'] for k,v in p.items() if k in adj}
    tree=root/('view' if mode=='A' else 'generations/'+selected['attempt']);newnav=navigation(tree)
    assert newnav['d000002']==nav['d000002'] and newnav['d000005']['source']=='docs/item-multipart-target-d000005/source.json'
    rows.append(dict(case='multipart-601-ordered-typed-duplicate-external-target-location',mode=mode,adjacency_parts=3,incoming_record_and_adjacency_bytes_unchanged=True,status='success'))

    d.publish(root,'packet-owner',[],packet=True);head=d.head(root);accepted=d.call(root,'resolve',request_id='packet-owner')['result']
    d.call(root,'checkpoint',request_id='carry4');d.call(root,'lifecycle',operation='retire',selected=head);d.call(root,'lifecycle',operation='collect')
    assert d.call(root,'resolve',request_id='packet-owner')['result']==accepted
    selected=json.loads((root/f"publications/{d.head(root)['publication']}.json").read_bytes());p=Tree(root).entries(selected['projection'])
    for name in [next(k for k in p if k.startswith('navigation/groups/')),'packet/context.md','packet/provenance.json']:
        path=root/p[name]['source'];saved=path.read_bytes()
        for damage in ['missing','corrupt']:
            if damage=='missing':path.unlink()
            else:path.write_bytes(b'independent native corrupt evidence')
            d.call(root,'audit',code=2);path.write_bytes(saved);d.call(root,'audit')
    node=root/'objects'/selected['projection'][:2]/selected['projection'];saved=node.read_bytes();node.unlink();d.call(root,'audit',code=2);node.write_bytes(saved);d.call(root,'audit')
    verify(root);rows.append(dict(case='profile4-strict-navigation-packet-tree-evidence-and-owner-retention',mode=mode,status='success'))

    # Capture B once, then publish a new head. Shell reads only that retained path.
    selected=json.loads((root/f"publications/{d.head(root)['publication']}.json").read_bytes());p=Tree(root).entries(selected['projection']);g=Tree(root).entries(selected['graph']['tree'])
    tree=root/('view' if mode=='A' else 'generations/'+selected['attempt'])
    captured=str(tree);expected_nav=navigation(tree)
    if mode=='B':d.publish(root,'after-capture',[])
    dest=BASE/('bash-'+mode)
    cp=subprocess.run(['bash',str(BASE/'ordinary.sh'),captured,str(dest)],capture_output=True,timeout=120)
    (BASE/('bash-'+mode+'.stdout')).write_bytes(cp.stdout);(BASE/('bash-'+mode+'.stderr')).write_bytes(cp.stderr);assert cp.returncode==0,(cp.returncode,cp.stderr)
    copied=dest/'tree';assert navigation(copied)==expected_nav
    for path,f in p.items():assert hashlib.sha256((copied/path).read_bytes()).hexdigest()==f['digest']
    for k,h in g.items():
        if k.startswith('D/'):assert (copied/h['doc']['path']).read_bytes()==(root/h['doc']['original']).read_bytes()
    extracted=(dest/'extracted-path.txt').read_text().strip();assert (dest/'extracted-source').read_bytes()==(copied/extracted).read_bytes()
    rows.append(dict(case='tool-free-bash-discovery-copy-extraction',mode=mode,captured_tree_once=captured,selected_at_capture=selected['id'],files_verified=len(p),authored_documents=16,external_locators_discovered=True,status='success'))

    start=time.monotonic();full=operational(root);assert time.monotonic()-start<=900
    (BASE/('operational-'+mode+'.json')).write_text(json.dumps(full,indent=2))
    rows.append(dict(case='independent-complete-operational-oracle',mode=mode,publications=full['publications'],status='success'))
    (d.base/'summary.json').write_text(json.dumps(rows,indent=2))
print(json.dumps(rows,indent=2))
