"""Native fact-dirtiness control and historical profile2 packet checkpoint gap."""
import json,sys,hashlib
import supervise as s
sys.path.insert(0,str(s.H))
from focused import Driver
from oracle import Tree,verify

s.STAGES=json.loads((s.BASE/'stages.json').read_bytes())
exe=s.BASE/'build/release/file-native-incremental'
assert json.loads((s.BASE/'stage1-data/summary.json').read_bytes()),'separate fact-dirtiness control required'
for profile in [2,3]:verify(s.BASE/f'stage1-data/p{profile}/disposable/store')
d=Driver(exe,s.BASE/'historical2-data');rows=[]
for mode in ['A','B']:
    root=d.init(mode,mode,n=16,profile=2);old=d.head(root);path=root/f"publications/{old['publication']}.json";original=path.read_bytes()
    c=d.call(root,'prepare',profile=2,request=dict(request='sealed2-packet',base=old,ops=[],packet=True))['result']['candidate']
    saved=(root/c).read_bytes();d.call(root,'select',profile=4,candidate=c);assert (root/c).read_bytes()==saved
    d.call(root,'checkpoint',profile=4,request_id='checkpoint2-packet')
    selected=json.loads((root/f"publications/{d.head(root)['publication']}.json").read_bytes());projection=Tree(root).entries(selected['projection'])
    assert projection['packet/provenance.json'].get('projection_profile',2)==2
    assert path.read_bytes()==original;verify(root)
    d.publish(root,'upgrade4-after-checkpoint2',[],profile=4);verify(root)
    rows.append(dict(mode=mode,sealed2_packet_replayed=True,checkpoint_carried_profile2=True,old_publication_unchanged=True,candidate_sha256=hashlib.sha256(saved).hexdigest(),status='success'))
(d.base/'summary.json').write_text(json.dumps(rows,indent=2))
print(json.dumps(rows,indent=2))
sys.path.insert(0,str(s.BASE/'source/docs/research/file-native-recovery-reclamation-2026-10-19'))
from verify_operational import verify as operational
for mode in ['A','B']:
    for n in [16,128]:
        root=s.BASE/f'focused-data/recovery-capsule/restored-{mode}-{n}/disposable/store'
        full=operational(root);verify(root)
        (s.BASE/f'restored-operational-{mode}-{n}.json').write_text(json.dumps(full,indent=2))
