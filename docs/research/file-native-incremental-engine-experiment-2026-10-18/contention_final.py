"""Bounded final-core 1/2/4/8 trace with an observed pending-artifact census."""
import argparse,concurrent.futures,hashlib,json,os,time,zipfile
from pathlib import Path
from controller import Driver
from checks import cleanup,op,req,result

def files(root):
    found={}
    for directory,_,names in os.walk(root):
        for name in names:
            p=Path(directory)/name;found[p.relative_to(root).as_posix()]=p.stat().st_size
    return found

def run(d,root):
    r=str(root);rows=[]
    assert json.loads((root/'STORE.json').read_bytes())['mode']=='A'
    assert result(d.call(dict(action='inspect',root=r)))['publication']['graph']['count']==10000
    assert result(d.call(dict(action='point',root=r,id='d000000')))['value']['next']>=5002
    for workers in [1,2,4,8]:
        d.guard();cleanup(d,r);before=files(root);release=d.base/f'final-wave-{workers}-release';jobs=[];attempts={};starts={};ends={}
        def attempt(request,env):
            out=d.call(dict(action='publish',root=r,request=request),180,env,True);return out,time.perf_counter_ns()
        with concurrent.futures.ThreadPoolExecutor(max_workers=workers) as pool:
            for i in range(workers):
                name=f'final-wave-{workers}-{i}';ready=d.base/(name+'-ready')
                operation=op('replace-edge',id='d000000',value=11,text='constituent',target=f'd{1000+workers:06}') if i==workers-1 else op('label',id=f'd{(i*10000//workers+701)%10000:06}',text=name)
                request=req(name,[operation]);starts[name]=time.perf_counter_ns()
                env={'EXP_BARRIER':'candidate-ready','EXP_READY':str(ready),'EXP_RELEASE':str(release)}
                jobs.append((name,request,ready,pool.submit(attempt,request,env)))
            deadline=min(d.end,time.monotonic()+55)
            while not all(ready.exists() for _,_,ready,_ in jobs):assert time.monotonic()<deadline;time.sleep(.01)
            census_start=time.monotonic();quota=result(d.call(dict(action='lifecycle',root=r,operation='quota')))['reservations'];assert len(quota)==workers
            after=files(root);new={p:n for p,n in after.items() if p not in before};growth=sum(max(n-before[p],0) for p,n in after.items() if p in before)
            archive=d.base/f'pending-{workers}.zip'
            with zipfile.ZipFile(archive,'w',compression=zipfile.ZIP_DEFLATED) as z:
                for path,size in sorted(new.items()):
                    b=(root/path).read_bytes();assert len(b)==size;z.writestr(zipfile.ZipInfo(path,(1980,1,1,0,0,0)),b,compress_type=zipfile.ZIP_DEFLATED)
                z.writestr('OBSERVED-RESERVATIONS.json',json.dumps(quota,indent=2))
            pending={'workers':workers,'observed_reservations':quota,'reserved_bytes':sum(q['bytes'] for q in quota.values()),'new_unique_files':len(new),'new_unique_file_bytes':sum(new.values()),'existing_file_growth_bytes':growth,'snapshot_archive':archive.name,'snapshot_sha256':hashlib.sha256(archive.read_bytes()).hexdigest(),'census_and_archive_pause_seconds':time.monotonic()-census_start}
            (d.base/f'pending-{workers}.json').write_text(json.dumps(pending,indent=2));release.write_text('release')
            for name,request,ready,future in jobs:
                out,finished=future.result();attempts[name]=[out];ends[name]=finished
        cleanup(d,r)
        for name,request,ready,_ in jobs:
            while attempts[name][-1]['status']!='ok' and len(attempts[name])<4:
                attempts[name].append(d.call(dict(action='publish',root=r,request=request),180,allow=True));ends[name]=time.perf_counter_ns()
            last=attempts[name][-1];assert last['status']=='ok' and result(last)['status']=='published' and result(last)['result']['outcome']=='accepted'
            rows.append({'workers':workers,'request':name,'all_attempts':attempts[name],'request_all_attempt_ns':ends[name]-starts[name],'pending':pending,'controlled_barrier_and_census_pause':True,'post_request_reservations':result(d.call(dict(action='lifecycle',root=r,operation='quota')))['reservations']})
            (d.base/'contention.json').write_text(json.dumps(rows,indent=2))
        cleanup(d,r)
    d.call(dict(action='audit',root=r),cap=min(180,max(1,d.end-time.monotonic())))
    return rows

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);a.add_argument('--root',required=True);ns=a.parse_args()
    assert not Path(ns.base).exists(),'fresh observation root required'
    d=Driver(ns.exe,ns.base,900);(d.base/'contention.json').write_text(json.dumps(run(d,Path(ns.root)),indent=2))
