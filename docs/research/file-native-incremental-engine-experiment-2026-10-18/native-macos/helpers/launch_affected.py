"""Corrected-pin affected proof only, within the unchanged final lane budget."""
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
PIN='a1108c634cb8362aca50e1a6228766802b96f087'
SOURCE=ROOT/'source-corrected/docs/research/file-native-incremental-engine-experiment-2026-10-18'
LANE=ROOT/'corrected-lane';LANE.mkdir(exist_ok=False)
E=ROOT/'evidence-corrected';target=LANE/'build';exe=target/'release/file-native-incremental'
commands=[('test',['cargo','test','--locked','--workspace','--target-dir',str(target)],600),
          ('build',['cargo','build','--release','--locked','--workspace','--target-dir',str(target)],600),
          ('namespace',[sys.executable,str(SOURCE/'namespace_cases.py'),'--exe',str(exe),'--base',str(LANE/'namespace')],300),
          ('checks',[sys.executable,str(SOURCE/'checks.py'),'--exe',str(exe),'--base',str(LANE/'checks'),'--faults'],900),
          ('supplement',[sys.executable,str(SOURCE/'supplement.py'),'--exe',str(exe),'--base',str(LANE/'supplement')],900),
          ('adversarial',[sys.executable,str(SOURCE/'adversarial.py'),'--exe',str(exe),'--base',str(LANE/'adversarial')],600)]
env={**os.environ,'PYTHONPATH':str(SOURCE),'PYTHONDONTWRITEBYTECODE':'1'}
ok=True
for label,argv,cap in commands:
    snap=inventory()
    assert time.time()<A['execution_deadline_epoch'] and snap['free_bytes']>=A['reserve_bytes']
    assert max(snap['logical_bytes'],snap['allocated_file_bytes'])<A['logical_cap_bytes']-512*1024**2
    start=time.time();end=min(time.monotonic()+cap,time.monotonic()+A['execution_deadline_epoch']-time.time());reason=None
    with (E/(label+'.stdout')).open('xb') as out,(E/(label+'.stderr')).open('xb') as err:
        p=subprocess.Popen(argv,cwd=SOURCE,env=env,stdout=out,stderr=err,start_new_session=True)
        while p.poll() is None:
            snap=inventory()
            with (E/'resources.jsonl').open('a') as f:f.write(json.dumps(dict(label=label,**snap))+'\n')
            if time.monotonic()>=end:reason='affected-or-original-final-deadline'
            elif snap['free_bytes']<A['reserve_bytes']:reason='free-space-floor'
            elif max(snap['logical_bytes'],snap['allocated_file_bytes'])>A['logical_cap_bytes']-512*1024**2:reason='whole-root-cap-margin'
            if reason:os.killpg(p.pid,signal.SIGKILL);break
            time.sleep(2)
        p.wait()
        if p.returncode:
            try:os.killpg(p.pid,signal.SIGKILL)
            except ProcessLookupError:pass
    row=dict(label=label,argv=argv,cwd=str(SOURCE),start_epoch=start,end_epoch=time.time(),exit=p.returncode,
        censored=bool(reason),reason=reason,pin=PIN,launcher_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest())
    with (E/'runs.jsonl').open('a') as f:f.write(json.dumps(row)+'\n')
    print(json.dumps(row),flush=True)
    if label=='build' and p.returncode==0:
        binaries={}
        for name in ['file-native-incremental','project-consumer','research-consumer']:
            path=target/'release'/name;binaries[name]=dict(path=str(path),bytes=path.stat().st_size,sha256=hashlib.sha256(path.read_bytes()).hexdigest())
        (E/'binary-pins.json').write_text(json.dumps(binaries,indent=2)+'\n')
    if p.returncode or reason:ok=False;break
(E/'finished.json').write_text(json.dumps(dict(status='passed' if ok else 'stopped',pin=PIN,epoch=time.time(),resources=inventory(),original_final_execution_deadline=A['execution_deadline_epoch']),indent=2)+'\n')
raise SystemExit(0 if ok else 1)
