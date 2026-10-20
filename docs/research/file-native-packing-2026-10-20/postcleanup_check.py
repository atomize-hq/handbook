"""Admitted complete postcleanup oracle, or bounded read-only reopen probes."""
import argparse,hashlib,json,os,runpy,sys
from pathlib import Path
from driver import Driver
from independent import verify

R=Path('C:/file-native-packing-20261020');ROOT=R/'history-restored/disposable/store';OUT=R/'long-matrix-fixed2';EXE=R/'bin/candidate2.exe'
read=lambda p:json.loads(Path(p).read_bytes())
def stable():
    assert hashlib.sha256(EXE.read_bytes()).hexdigest()=='568f631acf528c7de911bb88ec85d07ead9d7b4435ebe970c61053f06e31a93b'
    for path,row in read(R/'fix2/build-binding.json')['files'].items():assert hashlib.sha256(Path(path).read_bytes()).hexdigest()==row['sha256']
    assert read(ROOT/'SELECTED.json')==read(R/'long-matrix/frozen.json')['selected']
    assert read(ROOT/'STORE.json')['catalog']=='84b1331e8d66e26c1f02074a9aa39269dd16446f519399ac70857bef1228dcd5'

def run(stage):
    stable();before=(ROOT/'SELECTED.json').read_bytes();config=(ROOT/'STORE.json').read_bytes()
    receipt=read(R/'receipts/main-cleanup.json');assert receipt['process_closure_verified'] and receipt['exit']==0 and receipt['censor'] is None
    cleanup=read(OUT/'cleanup-summary.json');assert cleanup['deferred_bytes']==0 and not cleanup['remaining_mapped_loose_files'] and not cleanup['remaining_old_P0_containers']
    if stage=='verify':
        result=verify(ROOT,OUT/'postcleanup-independent.json',R/'long-matrix/L-inventory.manifest.json',R/'preservation-inputs/post-gc-reference.json')
        old=read(OUT/'PZ-independent.json')
        for key in ['raw_records','raw_bytes','inventory_digest']:assert result[key]==old[key]
        for key in ['publications','ledger_rows','permanent_digest','retired_publications','selected']:assert result['semantic'][key]==old['semantic'][key]
        assert result['raw_records']==504833 and result['semantic']['publications']==101007 and result['semantic']['retired_publications']==101
    else:
        full=read(OUT/'postcleanup-independent.json');d=Driver(OUT/'postcleanup-reopen-events')
        assert d.call(ROOT,'inspect',exe_override=EXE)['result']['selected']==full['semantic']['selected'];d.call(ROOT,'audit',exe_override=EXE)
        retired=0
        for marker in sorted((ROOT/'retired').glob('*.json')):
            body=d.call(ROOT,'point',exe_override=EXE,code=2,selected=read(marker),id='d000000');assert body['error']=='retired:snapshot';retired+=1
        f=read(R/'long-matrix/frozen.json')
        for request in f['original_lookup_requests']:assert d.call(ROOT,'resolve',exe_override=EXE,request_id=request['id'])['result']==f['expected_outcomes'][request['id']]
        intent=read(R/'long-matrix/P0-intent.json');assert d.call(ROOT,'resolve',exe_override=EXE,request_id=intent['request'],intent=intent['original_intent'])['result']==f['expected_outcomes'][intent['request']]
        wrong=hashlib.sha256(b'postcleanup-wrong-intent').hexdigest();assert d.call(ROOT,'resolve',exe_override=EXE,code=2,request_id=intent['request'],intent=wrong)['error']==intent['different_intent_refusal']
        assert retired==101
        sys.argv=['allocation_parallel.py',str(ROOT),str(OUT/'postcleanup-allocation.json')];runpy.run_path(str(Path(__file__).parent/'allocation_parallel.py'),run_name='__main__')
        result=dict(status='passed',retired_content_refusals=retired,engine_events=len(d.events),same_and_changed_intent_checked=True,all_original_lookup_outcomes_unchanged=True,selected_store_unchanged=True,no_publication=True,eligible_class_file_reduction=cleanup['eligible_file_reduction'],ninety_five_percent_gate=cleanup['eligible_file_reduction']>=.95)
        (OUT/'postcleanup-reopen.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
    stable();assert before==(ROOT/'SELECTED.json').read_bytes() and config==(ROOT/'STORE.json').read_bytes()

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('stage',choices=['verify','reopen']);run(p.parse_args().stage)
