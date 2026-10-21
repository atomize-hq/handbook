import sys,json
from role_driver_v23 import invoke
out=[]
for role in ('reader','checker'):
 out.append(invoke(role,[sys.executable,'-B','-c','import time;time.sleep(.2)'],name=role+'-normal-v25',timeout=2,identity_budget=16,control_only=True))
for kw,expected in [({'role':'undefined','argv':[sys.executable],'control_only':True},'explicit-role'),({'role':'reader','argv':['/bin/echo'],'control_only':True},'exact-python'),({'role':'reader','argv':[sys.executable],'control_only':False},'role-build-binding-hash')]:
 try:
  invoke(name='MUSTNOTCREATE',timeout=1,identity_budget=16,**kw)
  raise AssertionError('binding refusal missing')
 except (RuntimeError,ValueError) as e:
  assert expected in str(e),str(e)
  out.append({'expected_refusal':expected,'observed':str(e)})
print(json.dumps(out),flush=True)
