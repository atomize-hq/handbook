import json,sys
from pathlib import Path
import capsule_adapter_v30 as a
b=a.Bridge({'control_only':True,'control_argv':[sys.executable,'-B',str(Path(__file__).with_name('fake_capsule_child_v28.py'))],'phase':'tiny','role_identity_budget':16,'run_name':'output-cap-tiny-v30','stdout_cap':4096})
try:
 try:b.call(a.ROOT/'fixtures/output-cap-tiny-v30/disposable/store','overflow')
 except RuntimeError as e:assert 'role-output-cap:stdout' in str(e),str(e)
 else:raise AssertionError('cap did not refuse')
finally:b.close()
p=a.ROOT/'evidence/roles/output-cap-tiny-v30-1/result.json';row=json.loads(p.read_bytes());assert row['failures'] and not row['output_complete'] and row['outputs']['stdout']['bytes']<=4096 and all(x['absent'] for x in row['closure'])
a.report(a.ROOT/'evidence/control/output-cap-tiny-v30-proof.json',{'expected_censor':True,'raw_output_complete':False,'no_partial_output_promoted':True,'positive_two_observation_closure':True,'bytes_retained':row['outputs']['stdout']['bytes']})
print('EXPECTED_OUTPUT_CAP_CENSOR_PASS')
