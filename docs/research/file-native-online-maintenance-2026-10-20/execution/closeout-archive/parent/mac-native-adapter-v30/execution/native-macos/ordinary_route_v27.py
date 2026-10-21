import sys,json,hashlib
from pathlib import Path
root=Path(sys.argv[1]);out=Path(sys.argv[2]);captured=sys.argv[3];save=sys.argv[4];selection=Path(captured) if captured else root/'SELECTED.json';raw=selection.read_bytes();s=json.loads(raw)
if save:Path(save).write_bytes(raw)
config=json.loads((root/'STORE.json').read_bytes())
if config['mode']=='B':
 p=root/'publications'/(s['publication']+'.json');b=p.read_bytes();assert hashlib.sha256(b).hexdigest()==s['digest'],'selected-publication-digest';pub=json.loads(b);tree=root/'generations'/pub['attempt'];assert tree.is_dir(),'selected-generation-missing';mb=(tree/'MANIFEST.json').read_bytes();assert hashlib.sha256(mb).hexdigest()==pub['generation_manifest'],'selected-generation-manifest-digest';attempt=pub['attempt']
else:
 assert not captured,'historical-A-view-not-admitted';tree=root/'view';assert tree.is_dir(),'ordinary-A-view-missing';attempt=None
out.mkdir(parents=True,exist_ok=False);(out/'route.json').write_text(json.dumps({'selected':s,'attempt':attempt,'generation':str(tree),'mode':config['mode'],'selection_source':str(selection),'selection_read_count':1,'engine_or_pack_decoder_used':False},indent=2));print(str(tree))
