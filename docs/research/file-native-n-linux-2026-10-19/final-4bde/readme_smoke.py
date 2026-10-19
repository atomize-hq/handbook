"""Explicit final-pin genesis omission and byte-exact historical README preservation."""
import sys,json,hashlib,shutil
from pathlib import Path
BASE=Path(__file__).resolve().parent
H=BASE/'source/docs/research/file-native-navigation-2026-10-19'
sys.path.insert(0,str(H))
from focused import Driver
from oracle import verify
d=Driver(BASE/'build/release/file-native-incremental',BASE/'readme-data');rows=[]
for mode in ['A','B']:
    root=d.init('new4-'+mode,mode,n=16,profile=4)
    assert not (root/'README.md').exists();verify(root)
    rows.append(dict(mode=mode,profile=4,case='genesis-root-README-absent',status='success'))
    for profile in [1,2,3]:
        if profile==1:
            root=d.base/('legacy1-'+mode)
            shutil.copytree(BASE/'focused-data/legacy-input/ordinary'/mode,root)
        else:root=d.init(f'old{profile}-'+mode,mode,n=16,profile=profile)
        path=root/'README.md';saved=path.read_bytes();assert saved.startswith(b'# File-native collection\n') and (b'NAVIGATION.md' if profile==1 else b'collection.yaml') in saved
        original_publication=d.head(root);oldpath=root/f"publications/{original_publication['publication']}.json";oldbytes=oldpath.read_bytes()
        if profile==1:d.call(root,'select',candidate=json.loads((root/'FIXTURE-CANDIDATE.json').read_bytes()))
        d.publish(root,f'upgrade{profile}-to4',[],profile=4)
        assert path.read_bytes()==saved and oldpath.read_bytes()==oldbytes;verify(root)
        d.call(root,'checkpoint',request_id=f'carry-upgraded-{profile}');assert path.read_bytes()==saved;verify(root)
        rows.append(dict(mode=mode,profile=profile,case='stored-README-and-old-publication-preserved-through-upgrade-checkpoint',README_sha256=hashlib.sha256(saved).hexdigest(),old_publication_sha256=hashlib.sha256(oldbytes).hexdigest(),status='success'))
    (d.base/'summary.json').write_text(json.dumps(rows,indent=2))
print(json.dumps(rows,indent=2))
