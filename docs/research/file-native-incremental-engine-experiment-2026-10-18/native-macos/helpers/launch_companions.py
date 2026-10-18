"""Finite post-capsule companion runs on the unchanged common native binary."""
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import time

ROOT=Path(__file__).resolve().parent
sys.path.insert(0,str(ROOT))
from outer import inventory
A=json.loads((ROOT/'allocation-final.json').read_bytes())
SOURCE=Path(A['source']);E=ROOT/'evidence-final';HELPERS=ROOT/'companions-f2c45c1'
exe=ROOT/'final-lane/build/release/file-native-incremental'
commands=[('deep-cases',[sys.executable,str(HELPERS/'deep_cases.py'),'--exe',str(exe),'--base',str(ROOT/'final-lane/deep-cases')],900),
          ('walk-cases',[sys.executable,str(HELPERS/'walk_cases.py'),'--exe',str(exe),'--base',str(ROOT/'final-lane/walk-cases'),'--source',str(SOURCE),'--shell','bash'],600)]
env={**os.environ,'PYTHONPATH':str(SOURCE),'PYTHONDONTWRITEBYTECODE':'1'}
for label,argv,cap in commands:
    snap=inventory()
    assert time.time()<A['execution_deadline_epoch'] and snap['free_bytes']>=A['reserve_bytes']
    assert max(snap['logical_bytes'],snap['allocated_file_bytes'])<A['logical_cap_bytes']-512*1024**2
    start=time.time();deadline=min(time.monotonic()+cap,time.monotonic()+A['execution_deadline_epoch']-time.time());reason=None
    with (E/(label+'.stdout')).open('xb') as out,(E/(label+'.stderr')).open('xb') as err:
        p=subprocess.Popen(argv,cwd=SOURCE,env=env,stdout=out,stderr=err,start_new_session=True)
        while p.poll() is None:
            snap=inventory()
            with (E/'companion-resources.jsonl').open('a') as f:f.write(json.dumps(dict(label=label,**snap))+'\n')
            if time.monotonic()>=deadline:reason='companion-or-final-deadline'
            elif snap['free_bytes']<A['reserve_bytes']:reason='free-space-floor'
            elif max(snap['logical_bytes'],snap['allocated_file_bytes'])>A['logical_cap_bytes']-512*1024**2:reason='whole-root-cap-margin'
            if reason:os.killpg(p.pid,signal.SIGKILL);break
            time.sleep(2)
        p.wait()
        if p.returncode:
            try:os.killpg(p.pid,signal.SIGKILL)
            except ProcessLookupError:pass
    row=dict(label=label,argv=argv,start_epoch=start,end_epoch=time.time(),exit=p.returncode,censored=bool(reason),reason=reason,
        helper_sha256=hashlib.sha256(Path(argv[1]).read_bytes()).hexdigest(),core_pin=A['pin'],executable_sha256=hashlib.sha256(exe.read_bytes()).hexdigest())
    with (E/'companion-runs.jsonl').open('a') as f:f.write(json.dumps(row)+'\n')
    print(json.dumps(row),flush=True)
