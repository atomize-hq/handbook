"""Independent complete graph/index and ordinary YAML discovery/extraction oracle.

No engine renderer/resolver is imported. The retained C1 tree decoder checks
hashes/parent bounds; this module independently parses the bounded YAML grammar.
"""
import hashlib,json,sys
from pathlib import Path
HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE.parent/'file-native-incremental-engine-experiment-2026-10-18'))
from verify_store import Tree,checked

def strict(raw):
    def pairs(rows):
        out={}
        for k,v in rows:
            assert k not in out;out[k]=v
        return out
    return json.loads(raw,object_pairs_hook=pairs)

def parse_group(raw):
    lines=raw.decode().splitlines();assert lines[0]=='version: 4'
    prefix=json.loads(lines[1].split(': ',1)[1]);assert set(prefix)<=set('01') and len(prefix)<=256
    assert lines[2]=='documents:';docs={};doc=rel=None
    for line in lines[3:]:
        if line.startswith('  - id: '):
            id=json.loads(line[8:]);assert id not in docs;doc=dict(id=id,relationships=[]);docs[id]=doc
        elif line.startswith('      - ordinal: '):
            rel=dict(ordinal=json.loads(line[17:]));doc['relationships'].append(rel)
        elif line.startswith('        '):
            k,v=line.strip().split(': ',1);assert k not in rel;rel[k]=json.loads(v)
        elif line in ['    relationships:','    relationships: []']:pass
        else:
            assert line.startswith('    ');k,v=line.strip().split(': ',1);assert k not in doc;doc[k]=json.loads(v)
    assert len(docs)<=512 and len(raw)<=262144
    for id in docs:
        bits=''.join(f'{b:08b}' for b in hashlib.sha256(id.encode()).digest());assert bits.startswith(prefix)
    return docs

def parse_edges(raw,id):
    lines=raw.decode().splitlines();assert lines[:3]==['version: 4','source_id: '+json.dumps(id),'relationships:'];rows=[];rel=None
    assert len(raw)<=1024*1024+1024
    for line in lines[3:]:
        if line.startswith('  - ordinal: '):rel=dict(ordinal=json.loads(line[13:]));rows.append(rel)
        else:
            assert line.startswith('    ');k,v=line.strip().split(': ',1);assert k not in rel;rel[k]=json.loads(v)
    assert len(rows)<=256 and (len(raw)<=262144 or len(rows)==1);return rows

def discovery(paths,read):
    docs={};used=set()
    for path in sorted(p for p in paths if p.startswith('navigation/groups/')):
        part=parse_group(read(path));assert not docs.keys()&part.keys();used.add(path)
        for id,d in part.items():
            if 'relationships_parts' in d:
                assert not d['relationships'];count=d.pop('relationships_parts');directory=d.pop('relationships_directory');assert type(count) is int and 0<count<=1000000
                refs=[f'{directory}{n:06}.yaml' for n in range(count)]
                for p in refs:
                    assert p in paths and p.startswith('navigation/relationships/'+id+'/') and p not in used
                    d['relationships'].extend(parse_edges(read(p),id));used.add(p)
            assert set(d)=={'id','title','kind','source','relationships'}
            ordinals=[]
            for e in d['relationships']:
                assert set(e)=={'ordinal','kind','target','external'} and type(e['external']) is bool and type(e['ordinal']) is int
                ordinals.append(e['ordinal'])
            assert ordinals==sorted(set(ordinals))
        docs.update(part)
    assert used=={p for p in paths if p.startswith('navigation/')}
    if docs:
        assert 'navigation.yaml' in paths
        assert read('navigation.yaml').startswith(b'version: 4\nlayout: stable-id-sha256-binary-prefix\n')
    else:assert not used and 'navigation.yaml' not in paths
    return docs

def navigation(tree):
    root=Path(tree);paths={p.relative_to(root).as_posix() for p in root.rglob('*') if p.is_file()}
    if 'collection.yaml' in paths:return parse_collection((root/'collection.yaml').read_bytes())
    return discovery(paths,lambda p:(root/p).read_bytes())

def parse_collection(raw):
    lines=raw.decode().splitlines();assert lines[:2]==['version: 2','documents:'];docs={};doc=rel=None
    for line in lines[2:]:
        if line.startswith('  - id: '):
            id=json.loads(line[8:]);assert id not in docs;doc=dict(id=id,relationships=[]);docs[id]=doc
        elif line.startswith('      - kind: '):rel=dict(kind=json.loads(line[14:]));doc['relationships'].append(rel)
        elif line.startswith('        '):k,v=line.strip().split(': ',1);assert k not in rel;rel[k]=json.loads(v)
        elif line.startswith('    relationships:'):pass
        else:k,v=line.strip().split(': ',1);assert k not in doc;doc[k]=json.loads(v)
    return docs

def verify(root,reference=None,readable=None):
    root=Path(root);sel=strict((root/'SELECTED.json').read_bytes());p=strict(checked(root/f"publications/{sel['publication']}.json",sel['digest']))
    graph=Tree(root).entries(p['graph']['tree']);projection=Tree(root).entries(p['projection']);g=p['graph']
    profile=projection['packet/provenance.json'].get('projection_profile',2);assert profile in [2,3,4]
    for f in projection.values():checked(root/f['source'],f['digest'])
    docs={k[2:]:v for k,v in graph.items() if k.startswith('D/')};expected_index={};expected={};all_expected={};edge_map={}
    for key,value in graph.items():
        if key.startswith('E/'):edge_map.setdefault(key.split('/')[1],[]).append((key,value))
    for id,h in docs.items():
        d=h['doc'];assert d['id']==id and not d['edges'];expected_index['D/'+id]=h;expected_index['P/'+d['path'].lower()]=id
        if d['label'] is not None:expected_index['L/'+d['label']]=id
        rows=[]
        for k,e in sorted(edge_map.get(id,[])):
            n=int(k.rsplit('/',1)[1],16);assert n<h['next'];expected_index[k]=e
            if not e['external']:
                assert e['target'] in docs;expected_index[f"R/{e['target']}/{id}/{n:016x}"]=e
            else:assert e['target'].startswith('https://')
            rows.append(dict(ordinal=n,**e))
        raw=checked(root/d['original'],d['digest']);wanted=dict(id=id,title=d['title'],kind=d['kind'],source=d['path'],relationships=rows);all_expected[id]=wanted
        complete=False
        if d['kind']=='self-describing-json':
            try:
                s=strict(raw);complete=json.dumps(s,sort_keys=True)==json.dumps(dict(id=id,title=d['title'],kind=d['kind'],relationships=[{k:v for k,v in e.items() if k!='ordinal'} for e in rows]),sort_keys=True)
            except (ValueError,AssertionError):pass
        if profile!=4:
            wanted={**wanted,'relationships':[{k:v for k,v in e.items() if k!='ordinal'} for e in rows]};expected[id]=wanted
        elif not complete:expected[id]=wanted
    assert graph==expected_index and len(docs)==g['count'] and sum(h['doc']['cost'] for h in docs.values())==g['cost']<=g['budget']
    actual=discovery(set(projection),lambda path:checked(root/projection[path]['source'],projection[path]['digest'])) if profile==4 else parse_collection(checked(root/projection['collection.yaml']['source'],projection['collection.yaml']['digest']))
    assert actual==expected,(actual.keys(),expected.keys())
    expected_paths={h['doc']['path'] for h in docs.values()}|{'packet/context.md','packet/provenance.json'}|{x for x in projection if x.startswith('navigation/') or x=='navigation.yaml' or x=='collection.yaml'}
    assert set(projection)==expected_paths
    if reference is not None:
        assert set(reference['docs'])==set(docs)
        for id,h in docs.items():
            assert {k:v for k,v in h['doc'].items() if k!='edges'}=={k:v for k,v in reference['docs'][id].items() if k!='edges'}
            assert [{k:v for k,v in e.items() if k!='ordinal'} for e in all_expected[id]['relationships']]==reference['docs'][id]['edges']
    config=strict((root/'STORE.json').read_bytes());current=True
    if config['mode']=='A':
        mark=strict((root/'VIEW.json').read_bytes()) if (root/'VIEW.json').exists() else None
        current=bool(mark and mark['selected']==sel and not (root/'PENDING.json').exists())
    readable=Path(readable) if readable else root/('view' if config['mode']=='A' else f"generations/{p['attempt']}")
    if current:
        names={x.relative_to(readable).as_posix() for x in readable.rglob('*') if x.is_file()};names.discard('MANIFEST.json');assert names==set(projection)
        if config['mode']=='B':assert strict(checked(readable/'MANIFEST.json',p['generation_manifest']))==projection
        assert navigation(readable)==expected
        for path,f in projection.items():
            checked(readable/path,f['digest'])
            if config['mode']=='B':assert (readable/path).stat().st_nlink==1
        for h in docs.values():assert (readable/h['doc']['path']).read_bytes()==checked(root/h['doc']['original'],h['doc']['digest'])
    return dict(documents=len(docs),supplemented_documents=len(expected),projection_files=len(projection),immutable_discovery_semantics_verified=True,readable_declared_current=current,discovery_relationships_and_extraction=current)
