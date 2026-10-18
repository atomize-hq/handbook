"""Profile preserved whole-manifest preparation before changing metadata backend."""
import pathlib, subprocess, json, os, time, hashlib, sys, shutil
HERE=pathlib.Path(__file__).resolve().parent
BASE=pathlib.Path(sys.argv[1])
EXE=pathlib.Path(sys.argv[2])
BASE.mkdir(parents=True,exist_ok=True)
out=BASE/'profile.jsonl'
with out.open('x',encoding='utf8') as log:
    for layout in ['baseline','sharded']:
        for n in [1000,10000]:
            assert shutil.disk_usage(BASE).free>160*1024**3
            root=BASE/f'profile-{layout}-{n}'
            c=dict(action='init',root=str(root),mode='A',consumer='project',count=n)
            start=time.perf_counter_ns()
            p=subprocess.Popen([str(EXE)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,env={**os.environ,'EXP_LAYOUT':layout})
            try: a,b=p.communicate(json.dumps(c).encode(),timeout=600); censored=False
            except subprocess.TimeoutExpired:
                p.kill();a,b=p.communicate();censored=True
            event=dict(command=c,layout=layout,binary_sha256=hashlib.sha256(EXE.read_bytes()).hexdigest(),external_ns=time.perf_counter_ns()-start,censored=censored,exit=p.returncode,output=json.loads(a) if a else None,progress=b.decode(errors='replace'))
            log.write(json.dumps(event)+'\n');log.flush()
            print(layout,n,event['external_ns']/1e9,p.returncode,flush=True)
