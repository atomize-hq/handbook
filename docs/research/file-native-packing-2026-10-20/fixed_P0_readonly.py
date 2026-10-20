"""Final-source reader on unchanged 4cb-produced raw packs; no publication/cleanup."""
import hashlib,json,time,statistics
from pathlib import Path
from driver import Driver

r=Path('C:/file-native-packing-20261020');root=r/'history-restored/disposable/store';old=r/'long-matrix';out=r/'long-matrix-fixed2';out.mkdir(exist_ok=False);exe=r/'bin/candidate2.exe';assert hashlib.sha256(exe.read_bytes()).hexdigest()=='568f631acf528c7de911bb88ec85d07ead9d7b4435ebe970c61053f06e31a93b'
(out/'measurement-source.py').write_bytes(Path(__file__).read_bytes());frozen=json.loads((old/'frozen.json').read_bytes());selected=(root/'SELECTED.json').read_bytes();store=(root/'STORE.json').read_bytes();assert json.loads(selected)==frozen['selected'];d=Driver(out/'events')
requests=frozen['original_lookup_requests'];baseline=json.loads((old/'L-old-measurement.json').read_bytes());previous=json.loads((old/'P0-measurement.json').read_bytes());warm=d.call(root,'lookup-bench',exe_override=exe,count=500,requests=requests)
assert [x['result_digest'] for x in warm['result']['rows']]==[x['result_digest'] for x in baseline['warm']['result']['rows']]==[x['result_digest'] for x in previous['warm']['result']['rows']]
fresh=[]
for request in requests:
    for sample in range(20):
        start=time.perf_counter_ns();body=d.call(root,'resolve',exe_override=exe,request_id=request['id']);elapsed=time.perf_counter_ns()-start;assert body['result']==frozen['expected_outcomes'][request['id']]
        fresh.append(dict(category=request['category'],sample=sample,engine_ns=body['wall_ns'],controller_ns=elapsed,child_seconds=d.events[-1]['seconds'],rss=body.get('peak_memory_bytes'),metrics=body['metrics']))
intent=json.loads((old/'P0-intent.json').read_bytes());same=d.call(root,'resolve',exe_override=exe,request_id=intent['request'],intent=intent['original_intent']);assert same['result']==frozen['expected_outcomes'][intent['request']]
changed=hashlib.sha256(b'fixed-final-source-different-intent').hexdigest();assert changed!=intent['original_intent'];different=d.call(root,'resolve',exe_override=exe,request_id=intent['request'],intent=changed,code=2);assert different['error']==intent['different_intent_refusal']
assert (root/'SELECTED.json').read_bytes()==selected and (root/'STORE.json').read_bytes()==store
ratios={row['category']:statistics.median(row['warm_ns'])/statistics.median(base['warm_ns']) for row,base in zip(warm['result']['rows'],baseline['warm']['result']['rows'])}
result=dict(status='passed',reader_core_pin='d70f3864f2888d46d9112213b4a03f8d82745a8e',pack_creation_core_pin='4cb6b9fba9ffabc294a6006ec54fe75366968db4',exe=str(exe),exe_sha256=hashlib.sha256(exe.read_bytes()).hexdigest(),original_cohort=frozen['original_lookup_requests'],warm=warm,fresh=fresh,warm_over_qualified_L=ratios,warm_2x_gate=all(value<=2 for value in ratios.values()),same_intent_exact_original_outcome=True,different_intent_refusal=different['error'],selected_and_store_unchanged=True,no_publish_or_cleanup=True,events=len(d.events),fresh_process_starts=len(fresh),cache_qualification='same process warm after first lookup in fixed category order; fresh process OS/device caches uncontrolled; per-category first lookup shares preceding categories')
(out/'summary.json').write_text(json.dumps(result,indent=2));print(json.dumps({k:v for k,v in result.items() if k not in ['warm','fresh','original_cohort']},indent=2))
