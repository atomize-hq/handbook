"""Low-frequency namespace-only progress; separate from engine application I/O."""
import json,os,pathlib,time
def counts(path):
    files=dirs=0
    if path.exists():
        for _,names,leaves in os.walk(path):dirs+=len(names);files+=len(leaves)
    return dict(files=files,directories=dirs)
def watch(root,stop,log):
    initial_attempts={p.name for p in (root/'attempts').iterdir()} if (root/'attempts').exists() else set()
    with log.open('x',encoding='utf8') as out:
        while not stop.is_set():
            start=time.perf_counter_ns();attempts=[]
            if (root/'attempts').exists():
                for path in (root/'attempts').iterdir():
                    if path.is_dir() and path.name not in initial_attempts:attempts.append(dict(attempt=path.name,namespace=counts(path)))
            row=dict(epoch=time.time(),originals=counts(root/'originals'),objects=counts(root/'objects'),view=counts(root/'view'),generations=counts(root/'generations'),preparations=attempts,selector_name_present=(root/'SELECTED.json').exists(),sampling_ns=time.perf_counter_ns()-start,qualification='name counts are progress, not authority or crash persistence')
            out.write(json.dumps(row)+'\n');out.flush();stop.wait(10)
