"""Finish bounded supplemental checks after serial native lanes have exited."""
import json
import pathlib
import subprocess
import sys
import time

HERE=pathlib.Path(__file__).resolve().parent
allocation=json.loads((HERE/'allocation.json').read_bytes())
while not (HERE/'schedule-completed.json').exists():
    if time.time() >= allocation['report_deadline_epoch']-480:
        raise RuntimeError('postprocess-wait-report-reserve')
    time.sleep(1)

commands=[('supplement-manifest',[sys.executable,str(HERE/'supplement.py'),'manifest']),
          ('supplement-cow',[sys.executable,str(HERE/'supplement.py'),'cow']),
          ('terminal-extraction',[sys.executable,str(HERE/'extract_native.py')])]
for label,command in commands:
    started=time.time()
    with (HERE/(label+'.log')).open('xb') as log:
        try:
            completed=subprocess.run(command,stdout=log,stderr=subprocess.STDOUT,timeout=240)
            code=completed.returncode;censored=False
        except subprocess.TimeoutExpired:
            code=None;censored=True
    event=dict(command=command,label=label,start_epoch=started,end_epoch=time.time(),exit=code,censored=censored)
    with (HERE/'postprocess-raw.jsonl').open('a') as f:f.write(json.dumps(event)+'\n')
    print(json.dumps(event),flush=True)
(HERE/'postprocess-completed.json').write_text(json.dumps(dict(epoch=time.time()))+'\n')
