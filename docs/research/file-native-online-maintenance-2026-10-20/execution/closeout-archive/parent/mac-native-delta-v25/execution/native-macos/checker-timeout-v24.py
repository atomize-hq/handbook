import sys,time
from role_driver_v23 import invoke
try:
 invoke('checker',[sys.executable,"-B","-c",'import time;time.sleep(2)'],name='checker-timeout-v24',timeout=0.15,identity_budget=16,control_only=True)
 raise AssertionError("expected censor missing")
except RuntimeError as e:
 assert 'role-timeout' in str(e),str(e)
 print("EXPECTED_ROLE_CENSOR",str(e),flush=True)
time.sleep(.1)
