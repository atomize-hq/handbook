import json,sys
from pathlib import Path
from role_driver_v29 import invoke
import admission_v20 as a
out=[]
for label,role,program,timeout,expected in [('memory','reader','import time;x=bytearray(272*1024**2);time.sleep(2)',3,'explicit-reader-RSS-cap'),('timeout','checker','import time;time.sleep(2)',.18,'role-timeout')]:
 name='role-failure-'+label+'-v29'
 try:invoke(role,[sys.executable,'-B','-c',program],name=name,timeout=timeout,identity_budget=16,control_only=True,stdout_cap=4096,stderr_cap=4096)
 except RuntimeError as e:assert expected in str(e),str(e)
 else:raise AssertionError('expected role censor missing')
 d=json.loads((a.ROOT/'evidence/roles'/name/'result.json').read_bytes());assert d['failures'] and all(x['absent'] for x in d['closure']);out.append({'case':label,'expected':expected,'result':d})
a.bounded_receipt(a.ROOT/'evidence/control/role-failure-v29-proof.json',out)
print('EXPECTED_MEMORY_TIMEOUT_CENSORS_PASS')
