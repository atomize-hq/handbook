"""Run the unchanged common lane with a stricter whole-root watchdog."""
import hashlib
import json
import os
from pathlib import Path
import signal
import shutil
import subprocess
import sys
import time

ROOT=Path(__file__).resolve().parent
A=json.loads((ROOT/'allocation-final.json').read_bytes())
E=ROOT/'evidence-final'
SOURCE=Path(A['source'])
deadline=time.monotonic()+max(0,A['execution_deadline_epoch']-time.time())

def inventory():
    logical=allocated=files=0;seen=set()
    for directory,_,names in os.walk(ROOT):
        for name in names:
            try:s=(Path(directory)/name).stat()
            except FileNotFoundError:continue
            files+=1;logical+=s.st_size;identity=(s.st_dev,s.st_ino)
            if identity not in seen:allocated+=s.st_blocks*512;seen.add(identity)
    return dict(epoch=time.time(),logical_bytes=logical,allocated_file_bytes=allocated,files=files,free_bytes=shutil.disk_usage(ROOT).free)

def descendants(pid):
    rows=[list(map(int,line.split())) for line in subprocess.check_output(['ps','-axo','pid=,ppid=,pgid='],text=True).splitlines()]
    owned={pid}
    while True:
        added={p for p,parent,group in rows if parent in owned}-owned
        if not added:break
        owned.update(added)
    return [(p,parent,g) for p,parent,g in rows if p in owned]

before=inventory();assert max(before['logical_bytes'],before['allocated_file_bytes'])<1024**3,before
assert before['free_bytes']>=A['reserve_bytes']
argv=[sys.executable,str(SOURCE/'run_lane.py'),'--root',A['lane_root'],'--source',str(SOURCE),'--pin',A['pin'],'--cap-gib','5','--reserve-gib','20']
(E/'launch-bindings.json').write_text(json.dumps(dict(argv=argv,allocation_sha256=hashlib.sha256((ROOT/'allocation-final.json').read_bytes()).hexdigest(),outer_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),common_runner_sha256=hashlib.sha256((SOURCE/'run_lane.py').read_bytes()).hexdigest(),before=before),indent=2)+'\n')
env={**os.environ,'PYTHONDONTWRITEBYTECODE':'1'}
with (E/'common-lane.stdout').open('xb') as out,(E/'common-lane.stderr').open('xb') as err:
    process=subprocess.Popen(argv,cwd=SOURCE,env=env,stdout=out,stderr=err,start_new_session=True)
    reason=None
    while process.poll() is None:
        snap=inventory()
        with (E/'global-resources.jsonl').open('a') as f:f.write(json.dumps(snap)+'\n')
        if time.monotonic()>=deadline:reason='global-final-work-deadline'
        elif snap['free_bytes']<A['reserve_bytes']:reason='global-free-space-floor'
        elif max(snap['logical_bytes'],snap['allocated_file_bytes'])>A['logical_cap_bytes']-512*1024**2:reason='global-file-cap-margin'
        if reason:
            # Stop leaf engine/compiler work first so its controller can capture bytes.
            owned=descendants(process.pid);parents={parent for _,parent,_ in owned}
            for pid,parent,group in owned:
                if pid!=process.pid and pid not in parents:
                    try:os.kill(pid,signal.SIGKILL)
                    except ProcessLookupError:pass
            time.sleep(2)
            for group in {g for _,_,g in descendants(process.pid)}:
                try:os.killpg(group,signal.SIGKILL)
                except ProcessLookupError:pass
            break
        time.sleep(10)
    process.wait()
(E/'global-finished.json').write_text(json.dumps(dict(argv=argv,exit=process.returncode,censored=reason is not None,reason=reason,after=inventory(),epoch=time.time()),indent=2)+'\n')
raise SystemExit(process.returncode if not reason else 1)
