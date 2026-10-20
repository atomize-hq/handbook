"""Final d70 main-cohort stages. Script availability is not a stage admission."""
import argparse,copy,hashlib,json,statistics,time
from pathlib import Path
from driver import Driver
from guard import current
from independent import verify

R=Path('C:/file-native-packing-20261020');ROOT=R/'history-restored/disposable/store';OLD=R/'long-matrix';BASE=R/'long-matrix-fixed2';EXE=R/'bin/candidate2.exe';PIN='d70f3864f2888d46d9112213b4a03f8d82745a8e';EXE_HASH='568f631acf528c7de911bb88ec85d07ead9d7b4435ebe970c61053f06e31a93b'
def read(path):return json.loads(Path(path).read_bytes())
def save(path,value):assert not Path(path).exists();Path(path).write_text(json.dumps(value,indent=2))
def bound():
    assert hashlib.sha256(EXE.read_bytes()).hexdigest()==EXE_HASH
    for name,row in read(R/'fix2/build-binding.json')['files'].items():assert hashlib.sha256(Path(name).read_bytes()).hexdigest()==row['sha256']
    assert read(ROOT/'SELECTED.json')==read(OLD/'frozen.json')['selected']
def build():
    bound();a=current();a.update(PZ_reservation=350000000,measurement_margin=33554432,cap=2791728742,prospective_bytes=a['total_new_bytes']+350000000+33554432,prospective_files=a['total_new_files']+2000);save(R/'receipts/final-PZ-fresh-admission.json',a)
    assert a['prospective_bytes']<=2791728742 and a['prospective_files']<=560000 and a['free_bytes']>=140*1024**3
    assert read(ROOT/'STORE.json')['catalog']==read(OLD/'P0-build.json')['response']['result']['catalog']
    d=Driver(BASE/'PZ-build-events');before=(ROOT/'SELECTED.json').read_bytes();save(BASE/'PZ-build-harness.json',dict(source=str(Path(__file__).resolve()),sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),core_pin=PIN,exe_sha256=EXE_HASH));body=d.call(ROOT,'packing',exe_override=EXE,timeout=1800,operation='build',codec='zstd',include_attempts=True,reserve_bytes=350000000,cleanup=False);bound();assert body['metrics']['pack_eligible_records']==504833 and body['metrics']['pack_eligible_raw_bytes']==579015428
    c=read(ROOT/'STORE.json');catalog=read(ROOT/f"pack-catalogs/{c['catalog']}.json");grouping=[{k:p[k] for k in ['role','first','last','records']} for p in catalog['packs']];assert grouping==read(OLD/'P0-build.json')['grouping'] and before==(ROOT/'SELECTED.json').read_bytes()
    result=dict(core_pin=PIN,exe_sha256=EXE_HASH,response=body,catalog=catalog,grouping=grouping,pack_plus_index_bytes=sum(p['pack_bytes']+p['index_bytes'] for p in catalog['packs']),P0_creation_core='4cb6b9fba9ffabc294a6006ec54fe75366968db4',P0_final_reader_core=PIN,physical_cleanup_complete=False,loose_and_old_raw_pack_duplicates_coexist=True)
    save(BASE/'PZ-build.json',result);print(json.dumps(dict(status='PZ-registered-no-cleanup',packs=len(catalog['packs']),records=sum(p['records'] for p in catalog['packs']),pack_plus_index_bytes=result['pack_plus_index_bytes'])))
def complete():
    bound();assert not (BASE/'PZ-independent.json').exists();result=verify(ROOT,BASE/'PZ-independent.json',OLD/'L-inventory.manifest.json',R/'preservation-inputs/post-gc-reference.json');bound();assert result['inventory_digest']==read(OLD/'P0-independent.json')['inventory_digest'] and result['semantic']['permanent_digest']==read(OLD/'P0-independent.json')['semantic']['permanent_digest'] and result['semantic']['publications']==101007 and result['semantic']['ledger_rows']==100000 and result['semantic']['retired_publications']==101
def measure():
    bound();d=Driver(BASE/'PZ-measure-events');f=read(OLD/'frozen.json');p0=read(BASE/'summary.json');baseline=read(OLD/'L-old-measurement.json');before=(ROOT/'SELECTED.json').read_bytes();warm=d.call(ROOT,'lookup-bench',exe_override=EXE,count=500,requests=f['original_lookup_requests'])
    assert [row['result_digest'] for row in warm['result']['rows']]==[row['result_digest'] for row in p0['warm']['result']['rows']]==[row['result_digest'] for row in baseline['warm']['result']['rows']]
    fresh=[]
    for request in f['original_lookup_requests']:
        for sample in range(20):
            start=time.perf_counter_ns();body=d.call(ROOT,'resolve',exe_override=EXE,request_id=request['id']);elapsed=time.perf_counter_ns()-start;assert body['result']==f['expected_outcomes'][request['id']];fresh.append(dict(category=request['category'],sample=sample,engine_ns=body['wall_ns'],controller_ns=elapsed,child_seconds=d.events[-1]['seconds'],rss=body.get('peak_memory_bytes'),metrics=body['metrics']))
    intent=read(OLD/'P0-intent.json');same=d.call(ROOT,'resolve',exe_override=EXE,request_id=intent['request'],intent=intent['original_intent']);assert same['result']==f['expected_outcomes'][intent['request']]
    wrong=hashlib.sha256(b'final-PZ-different-intent').hexdigest();different=d.call(ROOT,'resolve',exe_override=EXE,request_id=intent['request'],intent=wrong,code=2);assert different['error']==intent['different_intent_refusal']
    original=next(event['input']['request'] for event in read(OLD/'P0-intent-events/events.json') if event['input']['action']=='publish' and event['exit']==0)
    assert d.call(ROOT,'publish',exe_override=EXE,request=original)['result']==dict(status='deduplicated',result=f['expected_outcomes'][intent['request']]);changed=copy.deepcopy(original);changed['packet']=not changed['packet'];assert d.call(ROOT,'publish',exe_override=EXE,request=changed,code=2)['error']==intent['different_intent_refusal'];assert before==(ROOT/'SELECTED.json').read_bytes()
    ratios={row['category']:statistics.median(row['warm_ns'])/statistics.median(old['warm_ns']) for row,old in zip(warm['result']['rows'],p0['warm']['result']['rows'])};size=read(BASE/'PZ-build.json')['pack_plus_index_bytes']/read(OLD/'P0-build.json')['pack_plus_index_bytes']
    result=dict(status='passed',core_pin=PIN,exe_sha256=EXE_HASH,warm=warm,fresh=fresh,warm_over_final_P0=ratios,warm_2x_gate=all(v<=2 for v in ratios.values()),PZ_over_P0_pack_index_bytes=size,twenty_percent_smaller_gate=size<=.8,same_and_changed_original_intent_and_request_checked=True,selected_unchanged=True,process_starts=len(fresh),cache_qualification='same process warm in fixed original category order; fresh processes have uncontrolled OS/device caches',physical_cleanup_complete=False)
    bound();save(BASE/'PZ-measurement.json',result);print(json.dumps({k:v for k,v in result.items() if k not in ['warm','fresh']},indent=2))
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('stage',choices=['PZ-build','PZ-verify','PZ-measure']);a=p.parse_args();{'PZ-build':build,'PZ-verify':complete,'PZ-measure':measure}[a.stage]()
