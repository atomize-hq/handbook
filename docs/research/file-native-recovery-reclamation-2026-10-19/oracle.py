"""Independent profile-2 ordinary-file semantics, no engine during extraction."""
import hashlib,json,sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'file-native-incremental-engine-experiment-2026-10-18'))
from verify_store import Tree,checked

def navigation(tree):
    lines=(Path(tree)/'collection.yaml').read_text(encoding='utf-8').splitlines()
    assert lines[:2]==['version: 2','documents:'];docs={};doc=edge=None
    for line in lines[2:]:
        if line.startswith('  - id: '):
            key=json.loads(line[8:]);assert key not in docs;doc=dict(id=key,relationships=[]);docs[key]=doc
        elif line.startswith('      - kind: '):
            edge=dict(kind=json.loads(line[14:]));doc['relationships'].append(edge)
        elif line.startswith('        '):
            key,value=line.strip().split(': ',1);edge[key]=json.loads(value)
        elif line.startswith('    relationships:'): pass
        else:
            key,value=line.strip().split(': ',1);doc[key]=json.loads(value)
    return docs

def verify(root,reference=None,readable=None):
    root=Path(root);s=json.loads((root/'SELECTED.json').read_bytes());p=json.loads(checked(root/f"publications/{s['publication']}.json",s['digest']))
    graph=Tree(root).entries(p['graph']['tree']);projection=Tree(root).entries(p['projection'])
    docs={key[2:]:value['doc'] for key,value in graph.items() if key.startswith('D/')}
    expected={}
    for id,d in docs.items():
        edges=[(key,value) for key,value in graph.items() if key.startswith('E/'+id+'/')]
        expected[id]=dict(id=id,title=d['title'],kind=d['kind'],source=d['path'],relationships=[v for _,v in sorted(edges)])
        checked(root/d['original'],d['digest'])
    if reference is not None:
        assert docs=={k:{**v,'edges':[]} for k,v in reference['docs'].items()} or all({k:v for k,v in d.items() if k!='edges'}=={k:v for k,v in reference['docs'][id].items() if k!='edges'} for id,d in docs.items())
        assert set(docs)==set(reference['docs'])
        for id in docs:assert expected[id]['relationships']==reference['docs'][id]['edges']
    config=json.loads((root/'STORE.json').read_bytes())
    if readable is None:readable=root/('view' if config['mode']=='A' else f"generations/{p['attempt']}")
    readable=Path(readable);assert navigation(readable)==expected
    assert set(projection)=={'collection.yaml','packet/context.md','packet/provenance.json'}|{d['path'] for d in docs.values()}
    actual={x.relative_to(readable).as_posix() for x in readable.rglob('*') if x.is_file()};actual.discard('MANIFEST.json')
    assert actual==set(projection)
    for path,spec in projection.items():checked(readable/path,spec['digest'])
    for d in docs.values():assert (readable/d['path']).read_bytes()==checked(root/d['original'],d['digest'])
    return dict(documents=len(docs),projection_files=len(projection),discovery_relationships_and_extraction=True)
