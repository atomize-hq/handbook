import gzip,json,sys
from pathlib import Path
import capsule_adapter_v30 as adapter
from role_driver_v29 import invoke
root=adapter.ROOT;here=Path(__file__).resolve().parent
cfg={'control_only':True,'control_argv':[sys.executable,'-B',str(here/'fake_capsule_child_v28.py')],'phase':'tiny','role_identity_budget':16,'run_name':'adapter-tiny-v30','stdout_cap':131072}
b=adapter.Bridge(cfg);adapter.Driver.bridge=b;d=adapter.Driver(root/'fixtures/adapter-tiny-v30/events');store=root/'fixtures/adapter-tiny-v30/disposable/store'
try:
 value=d.call(store,'resolve',env={'EXP_CONTROL':'explicit-value'});assert len(value['result']['body'])==50000 and value['result']['environment']=='explicit-value'
 refused=d.call(store,'refuse',code=2);assert refused['error']=='expected-refusal'
 assert d.call(store,'interrupt',code=86) is None
 p=adapter.recovery_call(b,{'root':str(store),'action':'refuse'},{'EXP_CONTROL':'transparent'});assert p.returncode==2 and json.loads(p.stdout)['error']=='expected-refusal'
 try:d.call(store,'duplicate')
 except ValueError as e:assert 'duplicate-key' in str(e)
 else:raise AssertionError('duplicate-key accepted')
 for event in b.calls:
  for name,data in event['outputs'].items():
   p=Path(data['path']);assert p.stat().st_size==data['bytes'] and adapter.roles.sha256(p)==data['sha256']
 assert len(b.calls)==5
finally:b.close()
rows=[json.loads(x) for x in gzip.open(b.folder/'calls.jsonl.gz','rt')];assert len(rows)==5
adapter.report(root/'evidence/control/adapter-tiny-v30-proof.json',{'kind':'tiny-fake-adapter-control-only','bridge_calls':len(b.calls),'donor_events':len(d.events),'raw_response_bytes':sum(x['outputs']['stdout']['bytes'] for x in b.calls),'roles':[x['role'] for x in b.calls],'expected_exits':[x['exit'] for x in b.calls],'duplicate_key_refusal':True,'all_raw_output_SHA_size_match':True,'full_capsule_executed':False})
print('TINY_ADAPTER_CONTROLS_PASS',flush=True)
