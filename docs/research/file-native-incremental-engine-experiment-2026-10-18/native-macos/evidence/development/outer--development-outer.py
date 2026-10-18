"""Whole-owned-root allocation and process-group watchdog for native capsule."""
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
A=json.loads((ROOT/'allocation.json').read_bytes())
SOURCE=ROOT/'source/docs/research/file-native-incremental-engine-experiment-2026-10-18'
E=ROOT/'evidence'

def inventory():
    logical=allocated=files=0
    seen=set()
    for directory,_,names in os.walk(ROOT):
        for name in names:
            try:s=(Path(directory)/name).stat()
            except FileNotFoundError:continue
            files+=1;logical+=s.st_size
            key=(s.st_dev,s.st_ino)
            if key not in seen:allocated+=s.st_blocks*512;seen.add(key)
    return dict(epoch=time.time(),files=files,logical_bytes=logical,
        allocated_file_bytes=allocated,free_bytes=shutil.disk_usage(ROOT).free)

def reason(snapshot):
    if time.time()>=A['launch_deadline_epoch']-30:return 'native-execution-cutoff-report-reserve'
    if snapshot['free_bytes']<A['reserve_bytes']:return 'native-free-space-floor'
    # Reserve 512 MiB for bounded 1k writes during the monitor/cooperative interval.
    if max(snapshot['logical_bytes'],snapshot['allocated_file_bytes'])>A['cap_bytes']-512*1024**2:return 'native-whole-root-cap-margin'
    return None

def run(label,argv,cap):
    before=inventory()
    if reason(before):raise RuntimeError(reason(before))
    start=time.time();deadline=min(start+cap,A['launch_deadline_epoch'])
    env={**os.environ,'CARGO_TARGET_DIR':str(ROOT/'target'),'PYTHONDONTWRITEBYTECODE':'1'}
    with (E/(label+'.stdout')).open('xb') as out,(E/(label+'.stderr')).open('xb') as err:
        p=subprocess.Popen(argv,cwd=SOURCE,stdout=out,stderr=err,env=env,start_new_session=True)
        stopped=None;requested=None
        while p.poll() is None:
            snap=inventory()
            with (E/'resource-events.jsonl').open('a') as f:f.write(json.dumps(dict(label=label,**snap))+'\n')
            why=reason(snap)
            if time.time()>=deadline:why=why or 'outer-invocation-cap'
            if why and not stopped:
                stopped=why;requested=time.time()
                (ROOT/'stop.json').write_text(json.dumps(dict(reason=why,epoch=requested)))
                if label!='native':os.killpg(p.pid,signal.SIGKILL)
            if stopped and p.poll() is None and (time.time()-requested>=30 or time.time()>=A['launch_deadline_epoch']):
                os.killpg(p.pid,signal.SIGKILL)
            if p.poll() is None:time.sleep(min(5,max(.05,deadline-time.time())))
        p.wait()
        if p.returncode:
            try:os.killpg(p.pid,signal.SIGKILL)
            except ProcessLookupError:pass
    streams={}
    for kind in ['stdout','stderr']:
        path=E/(label+'.'+kind);data=path.read_bytes()
        streams[kind]=dict(file=path.name,bytes=len(data),sha256=hashlib.sha256(data).hexdigest())
    row=dict(label=label,argv=argv,cwd=str(SOURCE),start_epoch=start,end_epoch=time.time(),
        exit=p.returncode,censored=stopped is not None,reason=stopped,streams=streams,
        after=inventory(),environment=dict(CARGO_TARGET_DIR=env['CARGO_TARGET_DIR'],PYTHONDONTWRITEBYTECODE='1'))
    with (E/'outer-events.jsonl').open('a') as f:f.write(json.dumps(row)+'\n')
    print(json.dumps(row),flush=True)
    return p.returncode==0 and not stopped

if __name__=='__main__':
    ok=run('build',['cargo','build','--release','--locked','--workspace'],900)
    if ok:ok=run('test',['cargo','test','--locked','--workspace'],900)
    if ok:
        binaries={}
        for name in ['file-native-incremental','project-consumer','research-consumer']:
            p=ROOT/'target/release'/name
            binaries[name]=dict(path=str(p),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),bytes=p.stat().st_size)
        (E/'binary-pins.json').write_text(json.dumps(binaries,indent=2)+'\n')
        ok=run('native',[sys.executable,str(ROOT/'capsule_adapter.py'),
            '--exe',str(ROOT/'target/release/file-native-incremental'),
            '--base',str(ROOT/'runtime'),'--pin',A['pin']],max(1,A['launch_deadline_epoch']-time.time()))
    (E/'outer-completed.json').write_text(json.dumps(dict(status='passed' if ok else 'stopped',epoch=time.time(),resources=inventory()),indent=2)+'\n')
    raise SystemExit(0 if ok else 1)
