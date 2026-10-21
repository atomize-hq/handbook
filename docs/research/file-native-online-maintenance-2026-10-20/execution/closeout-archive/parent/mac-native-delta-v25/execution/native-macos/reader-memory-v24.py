import sys,time
from role_driver_v23 import invoke
try:
 invoke('reader',[sys.executable,"-B","-c",'import time;x=bytearray(272*1024**2);time.sleep(2)'],name='reader-memory-v24',timeout=3,identity_budget=16,control_only=True)
 raise AssertionError("expected censor missing")
except RuntimeError as e:
 assert 'reader-RSS-cap' in str(e),str(e)
 print("EXPECTED_ROLE_CENSOR",str(e),flush=True)
time.sleep(.1)
