"""Stdlib-only ordinary-file binding/verification; never calls an engine."""
import hashlib,json,sys
from pathlib import Path
def sha(b):return hashlib.sha256(b).hexdigest()
if sys.argv[1]=='bind':
    root=Path(sys.argv[2]);selector=json.loads((root/'SELECTED.json').read_bytes())
    b=(root/f"publications/{selector['publication']}.json").read_bytes();assert sha(b)==selector['digest'];p=json.loads(b)
    mode=json.loads((root/'STORE.json').read_bytes())['mode']
    if mode=='B':path=root/f"generations/{p['attempt']}";h=p['generation_manifest']
    else:
        packet=p['packet_ref'];assert packet is not None
        path=root/f"attempts/{packet['attempt']}/packet";h=packet['manifest']
    assert sha((path/'MANIFEST.json').read_bytes())==h
    print(path);print(h)
else:
    root=Path(sys.argv[2]);h=sys.argv[3];b=(root/'MANIFEST.json').read_bytes();assert sha(b)==h
    manifest=json.loads(b)
    for path,value in manifest.items():
        digest=value['digest'] if isinstance(value,dict) else value
        assert sha((root/path).read_bytes())==digest,path
    names={str(p.relative_to(root)).replace('\\','/') for p in root.rglob('*') if p.is_file()}
    assert names==set(manifest)|{'MANIFEST.json'}
    print(f'verified {len(manifest)} exact copied files; manifest {h}')
