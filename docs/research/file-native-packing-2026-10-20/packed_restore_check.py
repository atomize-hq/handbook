"""Prepared restore qualification stages; each requires its separate admission."""
import argparse,hashlib,json,os
from pathlib import Path
import driver
from driver import Driver
from independent import Decoder,verify
from packed_backup_stage import R,ROOT,RESTORED,CAPSULE,HERE,sha_file
from postcleanup_check import stable

OUT=R/'packed-restored-qualification';EXE=R/'bin/candidate2.exe'
driver.EXE=EXE
read=lambda p:json.loads(Path(p).read_bytes())
def closed(name):
    row=read(R/'receipts'/f'{name}.json');assert row['exit']==0 and row['censor'] is None and row['process_closure_verified'] and row['job_query_errors']==0
def exact():
    stable();assert sha_file(HERE/'independent.py')=='94987dbe1c992ef2491bb9819ad90185d2f29aef704b1bfaaa8f7e052a449e66'
    assert read(RESTORED/'STORE.json')==read(ROOT/'STORE.json')

def execute(stage):
    OUT.mkdir(exist_ok=True);exact();closed('packed-restore');manifest=read(CAPSULE/'manifest.json')
    if stage in ['verify','verify-successor']:
        if stage=='verify':assert read(RESTORED/'SELECTED.json')==manifest['selected'] and not (RESTORED/'DISPOSABLE.json').exists()
        else:closed('packed-successor')
        result=verify(RESTORED,OUT/(stage+'-independent.json'),R/'long-matrix/L-inventory.manifest.json',R/'preservation-inputs/post-gc-reference.json')
        assert result['raw_records']==504833 and result['raw_bytes']==579015428 and result['inventory_digest']=='058be9aea3ff6e4130f2a9f1af545c2f7ce895b6ebe6677d09b63e48d2f43e84'
        assert result['semantic']['ledger_rows']==100000 and result['semantic']['retired_publications']==101
        if stage=='verify':
            previous=read(R/'long-matrix-fixed2/postcleanup-independent.json');assert result['semantic']['publications']==101007
            for key in ['selected','permanent_digest','publications','ledger_rows','retired_publications']:assert result['semantic'][key]==previous['semantic'][key]
        else:
            successor=read(OUT/'successor.json');assert result['semantic']['publications']==101008 and result['semantic']['selected']==successor['selected']
            assert result['semantic']['permanent_digest']!=read(R/'long-matrix-fixed2/postcleanup-independent.json')['semantic']['permanent_digest']
        exact();return
    if stage=='reopen':
        closed('packed-restored-verify');assert read(RESTORED/'SELECTED.json')==manifest['selected'];before=(RESTORED/'SELECTED.json').read_bytes();d=Driver(OUT/'reopen-events')
        assert d.call(RESTORED,'inspect',exe_override=EXE)['result']['selected']==manifest['selected'];d.call(RESTORED,'audit',exe_override=EXE)
        assert not (RESTORED/'DISPOSABLE.json').exists()
        retired=0
        for marker in sorted((RESTORED/'retired').glob('*.json')):
            assert d.call(RESTORED,'point',exe_override=EXE,code=2,selected=read(marker),id='d000000')['error']=='retired:snapshot';retired+=1
        frozen=read(R/'long-matrix/frozen.json')
        for request in frozen['original_lookup_requests']:assert d.call(RESTORED,'resolve',exe_override=EXE,request_id=request['id'])['result']==frozen['expected_outcomes'][request['id']]
        intent=read(R/'long-matrix/P0-intent.json');assert d.call(RESTORED,'resolve',exe_override=EXE,request_id=intent['request'],intent=intent['original_intent'])['result']==frozen['expected_outcomes'][intent['request']]
        assert d.call(RESTORED,'resolve',exe_override=EXE,code=2,request_id=intent['request'],intent=hashlib.sha256(b'packed-restored-wrong-intent').hexdigest())['error']==intent['different_intent_refusal']
        assert retired==101 and before==(RESTORED/'SELECTED.json').read_bytes()
        result=dict(status='passed',retired_refusals=retired,events=len(d.events),original_outcomes_and_intent_checks=True,selected_unchanged=True,fresh_local_disposable_authority=False,imported_disposable_authority=False)
    else:
        closed('packed-restored-reopen');assert read(RESTORED/'SELECTED.json')==manifest['selected'];d=Driver(OUT/'successor-events')
        body=d.publish(RESTORED,'P-packed-restored-successor',ops=[dict(kind='label',id='d000004',text='packed-restored-successor')]);selected=d.head(RESTORED);assert selected!=manifest['selected']
        decoder=Decoder(RESTORED);publication=read(RESTORED/f"publications/{selected['publication']}.json");assert publication['predecessor']==manifest['selected'] and publication['sequence']==101007
        keys=[f"publications/{publication['id']}.json",f"results/{publication['result_hash']}.json",f"attempt-hints/{publication['attempt']}.json",f"attempts/{publication['attempt']}/candidate.json",f"attempts/{publication['attempt']}/allocations/{publication['id']}.json"]
        assert all(key not in decoder.entries and (RESTORED/key).is_file() for key in keys)
        frozen=read(R/'long-matrix/frozen.json')
        for request in frozen['original_lookup_requests']:assert d.call(RESTORED,'resolve',exe_override=EXE,request_id=request['id'])['result']==frozen['expected_outcomes'][request['id']]
        assert d.call(RESTORED,'resolve',exe_override=EXE,request_id='P-packed-restored-successor')['result']['outcome']=='accepted'
        result=dict(status='passed',response=body,selected=selected,predecessor=manifest['selected'],sequence=publication['sequence'],new_loose_tail_keys=keys,original_lookup_outcomes_unchanged=True,primary_root_unchanged=True,full_successor_independent_verification_pending=True)
    exact();(OUT/(stage+'.json')).write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('stage',choices=['verify','reopen','successor','verify-successor']);execute(parser.parse_args().stage)
