"""Independent complete navigation renderer and ordinary-file verification."""
import argparse,hashlib,json,time,sys
from pathlib import Path
from verify_store import Tree,checked

def sha(b):return hashlib.sha256(b).hexdigest()
def verify(root):
    started=time.monotonic();root=Path(root)
    if not (root/'SELECTED.json').exists():return {'root':str(root),'not_selected':True}
    if json.loads((root/'STORE.json').read_bytes()).get('navigation_version',1)==2:
        sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'file-native-recovery-reclamation-2026-10-19'))
        from oracle import verify as verify_v2
        return {'root':str(root),'navigation_version':2,**verify_v2(root),'seconds':time.monotonic()-started}
    selected=json.loads((root/'SELECTED.json').read_bytes());pub=json.loads(checked(root/f"publications/{selected['publication']}.json",selected['digest']))
    entries=Tree(root).entries(pub['graph']['tree']);projection=Tree(root).entries(pub['projection']);docs={k[2:]:v['doc'] for k,v in entries.items() if k.startswith('D/')};outgoing={id:[] for id in docs}
    for key,value in sorted(entries.items()):
        if key.startswith('E/'):
            _,id,ordinal=key.split('/');outgoing[id].append((int(ordinal,16),value))
    expected_paths={'NAVIGATION.md','packet/context.md','packet/provenance.json'};expected={}
    # Frozen profile-1 bytes remain valid; their actual root destinations must
    # exist too. A legacy dangling overview is a defect, never an oracle pass.
    root_text=(root/projection['NAVIGATION.md']['source']).read_text(encoding='utf-8')
    import re
    for destination in re.findall(r'\]\(([^)]+)\)',root_text):
        assert destination in projection,('legacy dangling root destination',destination)
    for id,doc in docs.items():
        source=checked(root/doc['original'],doc['digest']);assert len(source)==doc['bytes']
        expected[doc['path']]={'source':doc['original'],'digest':doc['digest']};expected_paths.add(doc['path'])
        title=doc['title'].replace('\r',' ').replace('\n',' ')
        nav=f"# {id}\n\n{title} — {doc['kind']}\n\n[Exact source](../{doc['path']}) · [Collection](../NAVIGATION.md)\n\n"
        for _,edge in sorted(outgoing[id]):
            if edge['external']:nav+=f"- {edge['kind']}: {edge['target']} (external; availability not promised)\n"
            else:nav+=f"- {edge['kind']}: [{edge['target']}](../{docs[edge['target']]['path']})\n"
        b=nav.encode();h=sha(b);name=f'navigation/{id}.md';expected_paths.add(name);expected[name]={'source':f'derived/{h[:2]}/{h}','digest':h}
        assert checked(root/expected[name]['source'],h)==b
    assert set(projection)==expected_paths,'complete projection namespace differs'
    assert all(projection[path]==spec for path,spec in expected.items()),'independent rendered navigation/source map differs'
    for path,spec in projection.items():checked(root/spec['source'],spec['digest'])
    if pub['packet_ref'] is None:
        assert (root/projection['packet/context.md']['source']).read_bytes()==b''
        p=json.loads((root/projection['packet/provenance.json']['source']).read_bytes())
        assert p['requested'] is False and p['inputs']==[] and p['output_sha256']==sha(b'')
    mode=json.loads((root/'STORE.json').read_bytes())['mode'];current=False
    if mode=='A':
        readable=root/'view';watermark=json.loads((root/'VIEW.json').read_bytes()) if (root/'VIEW.json').exists() else None
        current=bool(watermark and watermark['selected']==selected and not (root/'PENDING.json').exists())
    else:
        readable=root/f"generations/{pub['attempt']}";current=True
        assert json.loads(checked(readable/'MANIFEST.json',pub['generation_manifest']))==projection
    missing=[];edited=[]
    for path,spec in projection.items():
        p=readable/path
        if not p.exists():missing.append(path)
        else:
            if mode=='B':assert p.stat().st_nlink==1,'readable generation shares a hard link'
            if sha(p.read_bytes())!=spec['digest']:edited.append(path)
    actual={p.relative_to(readable).as_posix() for p in readable.rglob('*') if p.is_file()} if readable.exists() else set()
    if mode=='B':actual.discard('MANIFEST.json')
    extras=actual-set(projection)
    if current:assert not missing and not edited and not extras,'declared complete readable files differ'
    return {'root':str(root),'selected':selected,'documents':len(docs),'projection_files':len(projection),'independent_navigation_render_verified':True,'exact_sources_verified':True,'B_file_hardlink_independence_verified':mode=='B','readable_declared_current':current,'readable_missing':len(missing),'readable_edited':len(edited),'readable_extra':len(extras),'seconds':time.monotonic()-started}

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--roots',nargs='+',required=True);a.add_argument('--output',required=True);ns=a.parse_args();rows=[]
    for root in ns.roots:
        rows.append(verify(root));Path(ns.output).write_text(json.dumps(rows,indent=2))
