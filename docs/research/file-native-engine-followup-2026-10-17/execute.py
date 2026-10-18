"""Serial lane: paired backends at each tier, continuing independent bounded cases."""
import argparse,json,pathlib,subprocess,sys,time,threading
from monitor import watch
p=argparse.ArgumentParser();p.add_argument('--base',required=True,type=pathlib.Path);p.add_argument('--exe',required=True,type=pathlib.Path);p.add_argument('--skip-correctness',action='store_true');p.add_argument('--tiers',nargs='+',default=['1000','10000','100000']);a=p.parse_args()
here=pathlib.Path(__file__).resolve().parent;a.base.mkdir(parents=True,exist_ok=True)
plans=[]
if not a.skip_correctness:
    plans.extend((backend,'correctness',[]) for backend in ['manifest','cow'])
plans.extend((backend,phase,[]) for backend in ['manifest','cow'] for phase in ['contention','navigation'])
plans.extend((backend,'growth',['--tiers',n,'--modes',mode]) for n in a.tiers for backend in ['manifest','cow'] for mode in ['A','B'])
failures=[]
for backend,phase,extra in plans:
    command=[sys.executable,str(here/'experiment.py'),phase,'--base',str(a.base/backend),'--exe',str(a.exe),'--backend',backend,*extra]
    start=time.time();print('START',backend,phase,extra,time.ctime(),flush=True)
    stop=threading.Event();observer=None
    if phase=='growth' and int(extra[1])>=10000:
        root=a.base/backend/'growth'/f'{extra[1]}-{extra[3]}-project'
        progress=a.base/f'namespace-progress-{backend}-{extra[1]}-{extra[3]}-{time.time_ns()}.jsonl'
        observer=threading.Thread(target=watch,args=(root,stop,progress),daemon=True);observer.start()
    try:result=subprocess.run(command)
    finally:
        stop.set()
        if observer:observer.join()
    with (a.base/'execution.jsonl').open('a') as log:log.write(json.dumps(dict(command=command,start_epoch=start,end_epoch=time.time(),exit=result.returncode))+'\n')
    if result.returncode:failures.append(dict(phase=phase,backend=backend,extra=extra,exit=result.returncode))
    print('DONE',backend,phase,extra,flush=True)
(a.base/'execution-completed.json').write_text(json.dumps(dict(epoch=time.time(),phases=len(plans),failures=failures),indent=2)+'\n')
if failures:
    (a.base/'failed-phases.json').write_text(json.dumps(failures,indent=2)+'\n');raise SystemExit(2)
