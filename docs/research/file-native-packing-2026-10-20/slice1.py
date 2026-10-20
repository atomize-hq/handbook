import hashlib,json,subprocess,sys,time
from pathlib import Path
from driver import Driver,EXE
from independent import Decoder,freeze,verify
HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE.parent/'file-native-recovery-reclamation-2026-10-19'))
from operational_backup import capture,restore

def run():
    base=Path('C:/file-native-packing-20261020/slice1');d=Driver(base);cases=[]
    for mode in ['A','B']:
        source=base/mode/'original/disposable/store'
        d.call(source,'init',mode=mode,consumer='project',count=128)
        d.call(source,'lifecycle',operation='setup')
        first=d.head(source);dec=Decoder(source);publication=json.loads(dec.raw(f"publications/{first['publication']}.json"));candidate=f"attempts/{publication['attempt']}/candidate.json"
        expected=json.loads(dec.raw(f"results/{publication['result_hash']}.json"))
        d.publish(source,'lost-reply',env={'EXP_EXIT':'reply'},code=86)
        lost=d.call(source,'resolve',request_id='lost-reply')['result']
        d.call(source,'cancel',request_id='cancelled')
        archive=base/mode/'small-operational-backup';capture(source,archive,'slice1-working',True)
        root=base/mode/'restored/disposable/store';restore(archive,root)
        verify(root,base/mode/'restored-before-packing.json')
        d.call(root,'inspect');d.call(root,'lifecycle',operation='setup')
        inventory=base/mode/'loose-inventory.jsonl';rows=freeze(root,inventory)
        selection=(root/'SELECTED.json').read_bytes();owners={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in (root/'publications').glob('*.json')}
        packed=d.call(root,'packing',operation='build',codec='raw',cleanup=True)
        assert packed['result']['status']=='complete'
        assert (root/'SELECTED.json').read_bytes()==selection
        verify(root,base/mode/'packed-independent.json',inventory)
        d.call(root,'audit')
        assert d.call(root,'resolve',request_id='initial')['result']==expected
        assert d.call(root,'resolve',request_id='lost-reply')['result']==lost
        assert d.call(root,'resolve',request_id='missing')['result']['outcome']=='absent-as-of'
        assert d.call(root,'resolve',request_id='cancelled')['result']['outcome']=='cancelled-fenced'
        for name,h in owners.items():assert hashlib.sha256(Decoder(root).raw('publications/'+name)).hexdigest()==h
        old=Path('C:/file-native-navigation-20261019/bin/final.exe')
        p=subprocess.run([str(old)],input=json.dumps(dict(root=str(root),action='inspect')).encode(),capture_output=True,timeout=120)
        (base/mode/'old-binary.stdout').write_bytes(p.stdout);assert p.returncode==2
        # Finalized old candidate can disappear after retirement; packed accepted
        # hint still resolves the exact original retry without recreating it.
        d.publish(root,'successor')
        d.call(root,'lifecycle',operation='retire',selected=first)
        d.call(root,'lifecycle',operation='collect')
        assert d.call(root,'select',candidate=candidate)['result']==expected
        assert d.call(root,'point',code=2,id='d000000',selected=first)['error']=='retired:snapshot'
        packed_archive=base/mode/'packed-operational-backup';capture(root,packed_archive,'slice1-working',True)
        restored=base/mode/'packed-restored/disposable/store';restore(packed_archive,restored)
        verify(restored,base/mode/'packed-restored-independent.json',inventory)
        d.call(restored,'inspect');d.call(restored,'lifecycle',operation='setup');d.publish(restored,'restored-valid-successor')
        cases.append(dict(mode=mode,records=len(rows),exact_outcomes=True,strict_old_binary_refusal=True,retirement_and_hint_retry=True,operational_restore_and_successor=True))
    result=dict(status='passed',cases=cases,events=len(d.events),exe_sha256=hashlib.sha256(EXE.read_bytes()).hexdigest())
    (base/'summary.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
if __name__=='__main__':run()
