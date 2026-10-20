"""Clamp the existing worker group to the parent's corrected dispatch cutoff."""
import subprocess,time,signal,os,json,datetime
from pathlib import Path
O=Path(__file__).resolve().parent;worker=str(O/'followup.py')+' worker';cutoff=datetime.datetime.fromisoformat('2026-10-20T02:19:38+00:00').timestamp();start=time.time();samples=[];killed=[]
while True:
 p=subprocess.run(['ps','-axo','pid=,pgid=,args='],capture_output=True,text=True,timeout=2);groups=[]
 for line in p.stdout.splitlines():
  row=line.split(None,2)
  if len(row)==3 and row[2].endswith(worker):groups.append(int(row[1]))
 samples.append(dict(time=time.time(),groups=groups))
 if not groups:break
 if time.time()>=cutoff:
  for g in groups:
   try:os.killpg(g,signal.SIGKILL);killed.append(g)
   except ProcessLookupError:pass
  break
 time.sleep(.1)
(O/'evidence/dispatch-cutoff.json').write_text(json.dumps(dict(correct_dispatch='2026-10-20T02:09:38Z',correct_functional_cutoff='2026-10-20T02:19:38Z',correct_closeout_deadline='2026-10-20T02:29:38Z',watchdog_start_epoch=start,ended_epoch=time.time(),samples=samples,killed_groups=killed),indent=2))
