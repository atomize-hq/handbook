"""Serial native prerequisite phases; preserve every attempt and initial censor."""
import json
import os
import pathlib
import signal
import subprocess
import sys
import time

HERE = pathlib.Path(__file__).resolve().parent
allocation = json.loads((HERE/'allocation.json').read_bytes())
runtime = pathlib.Path(allocation['runtime'])
plan = [('correctness','cow',None),
        ('regressions','manifest',None),('regressions','cow',None),
        ('restoration','manifest',None),('restoration','cow',None),
        ('indirect-gc','cow',None),('backend-checks','cow',None),
        ('navigation','manifest',None),('navigation','cow',None),
        ('contention','manifest',None),('contention','cow',None),
        ('growth','manifest','A'),('growth','cow','A'),
        ('growth','manifest','B'),('growth','cow','B')]
(HERE/'execution-plan.json').write_text(json.dumps(plan,indent=2)+'\n')
manifest = runtime/'data/v1/manifest/correctness'
while not (manifest/'native-phase-completed.json').exists():
    if (manifest/'native-phase-stopped.json').exists():
        raise RuntimeError('initial manifest correctness failed; inspect retained stop')
    if time.time() >= allocation['launch_deadline_epoch']:
        raise RuntimeError('report-reserve-deadline')
    time.sleep(1)

for phase,backend,mode in plan:
    if time.time() >= allocation['launch_deadline_epoch']:
        break
    command=[sys.executable,str(HERE/'native_runner.py'),phase,'--backend',backend]
    if mode: command += ['--mode',mode]
    label=phase+'-'+backend+(('-'+mode) if mode else '')
    started=time.time()
    with (HERE/(label+'.log')).open('xb') as log:
        process=subprocess.Popen(command,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
        timed_out=False
        try:
            process.wait(timeout=min(1200 if phase=='correctness' else 600,
                max(1,allocation['launch_deadline_epoch']-time.time())))
        except subprocess.TimeoutExpired:
            timed_out=True
            os.killpg(process.pid,signal.SIGKILL)
            process.wait()
        if process.returncode:
            # Only this newly owned process group; paused helper children may remain.
            try: os.killpg(process.pid,signal.SIGKILL)
            except ProcessLookupError: pass
    event=dict(command=command,label=label,start_epoch=started,end_epoch=time.time(),
        exit=process.returncode,censored=timed_out,pid=process.pid)
    with (HERE/'schedule-raw.jsonl').open('a') as f:f.write(json.dumps(event)+'\n')
    print(json.dumps(event),flush=True)
    # Independent phases continue after recorded failure; no failed root is reused.

(HERE/'schedule-completed.json').write_text(json.dumps(dict(epoch=time.time(),
    scheduled=len(plan),reached=sum(1 for _ in (HERE/'schedule-raw.jsonl').open())))+'\n')
