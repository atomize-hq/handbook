"""Declared serial growth/stage traces, high-degree work, real closure and contention."""
import argparse,concurrent.futures,json,time
from pathlib import Path
from controller import Driver
from checks import req,op,result,cleanup,verify_packet
from verify_store import Tree
def run(d,tiers,contention=False):
    rows=[]
    for n in tiers:
        r=str(d.base/f'n{n}'/'disposable'/'store')
        initial=d.call(dict(action='init',root=r,mode='A',consumer='project',count=n),cap=2400)
        d.call(dict(action='lifecycle',root=r,operation='setup'))
        for k in [1,8,64]:
            ids=[f'd{(i*n//k+17)%n:06}' for i in range(k)]
            ops=[op('label',id=id,text=f'trace-{k}-{i}') for i,id in enumerate(ids)]
            if n<=10000:
                trace=d.call(dict(action='validation-trace',root=r,ops=ops),cap=600)
                rows.append({'stage':'validation','n':n,'k':k,'observation':trace})
                cleanup(d,r)
            before=result(d.call(dict(action='inspect',root=r)))['selected']
            updates=[op('import',id=id,text=f'# Exact bounded source {id}\r\n'+('x'*4000)) for id in ids]
            publication=d.call(dict(action='publish',root=r,request=req(f'update-{k}',updates,base=before)),cap=1200,env={'EXP_DEFER_REFRESH':'1'})
            refresh=d.call(dict(action='refresh',root=r),cap=1200)
            rows.append({'stage':'publication-and-A','n':n,'k':k,'publication':publication,'refresh':refresh})
        d.call(dict(action='audit',root=r),cap=1200)
        head=result(d.call(dict(action='inspect',root=r)))['publication']
        stats={}
        for name in ['graph','projection','membership','outcomes','changes']:
            tree=Tree(r);tree.entries(head['graph']['tree'] if name=='graph' else head[name]);stats[name]=tree.stats()
        rows.append({'n':n,'storage_trees':stats})
        if n==10000:
            edge=d.call(dict(action='publish',root=r,request=req('one-in-5000',[op('replace-edge',id='d000000',value=10,text='constituent',target='d000100')])),cap=1200)
            rows.append({'stage':'outgoing-5000-one-edge','observation':edge})
            for start in range(5000,10000,256):
                ops=[op('edge',id=f'd{i:06}',text='requires',target='d009998') for i in range(start,min(start+256,10000))]
                d.call(dict(action='publish',root=r,request=req(f'incoming-build-{start}',ops)),cap=1200)
            incoming=d.call(dict(action='publish',root=r,request=req('incoming-5000-rename',[op('rename',id='d009998',text='renamed target')])),cap=1200)
            rows.append({'stage':'incoming-5000-affected','observation':incoming})
            packet=d.call(dict(action='packet',root=r),cap=1200);verify_packet(Path(r)/result(packet)['path'])
            rows.append({'stage':'selected-closure-5000','observation':packet});cleanup(d,r)
            combined=d.call(dict(action='publish',root=r,request=req('combined-packet',[op('cost',id='d000011',value=0)],packet=True)),cap=1200)
            rows.append({'stage':'combined-closure-5000','observation':combined})
            if contention:rows.extend(waves(d,r))
        # B complete independent generation control; actual full copy and preselection verification.
        br=str(d.base/f'B-n{n}'/'disposable'/'store')
        b=d.call(dict(action='init',root=br,mode='B',consumer='project',count=n),cap=2400,allow=True)
        rows.append({'stage':'complete-B','n':n,'observation':b})
        if b and b['status']=='ok' and result(b)['status']=='published':
            d.call(dict(action='audit',root=br),cap=1200)
            pub=d.call(dict(action='publish',root=br,request=req('B-one',[op('label',text='B-label')])),cap=1200,allow=True)
            rows.append({'stage':'complete-B-successor','n':n,'observation':pub})
        (d.base/'matrix.json').write_text(json.dumps(rows,indent=2))
    return rows
def waves(d,r):
    rows=[]
    for workers in [1,2,4,8]:
        cleanup(d,r);release=d.base/f'wave-{workers}-release'
        jobs=[];starts={};ends={};attempts={}
        def attempt(request,env):
            out=d.call(dict(action='publish',root=r,request=request),180,env,True)
            return out,time.perf_counter_ns()
        with concurrent.futures.ThreadPoolExecutor(max_workers=workers) as pool:
            for i in range(workers):
                name=f'wave-{workers}-{i}';ready=d.base/f'{name}-ready'
                operation=op('replace-edge',id='d000000',value=11,text='constituent',target='d000101') if i==workers-1 else op('label',id=f'd{8000+i*17:06}',text=name)
                request=req(name,[operation]);starts[name]=time.perf_counter_ns()
                env={'EXP_BARRIER':'candidate-ready','EXP_READY':str(ready),'EXP_RELEASE':str(release)}
                jobs.append((name,request,ready,pool.submit(attempt,request,env)))
            deadline=time.monotonic()+55
            while not all(ready.exists() for _,_,ready,_ in jobs):
                assert time.monotonic()<deadline;time.sleep(.01)
            release.write_text('release')
            for name,request,ready,future in jobs:
                out,finished=future.result();ends[name]=finished;attempts[name]=[out]
        cleanup(d,r)
        for name,request,ready,_ in jobs:
            while attempts[name][-1]['status']!='ok' and len(attempts[name])<4:
                attempts[name].append(d.call(dict(action='publish',root=r,request=request),180,allow=True));ends[name]=time.perf_counter_ns()
            rows.append({'stage':'contention','workers':workers,'request':name,'all_attempts':attempts[name],'request_all_attempt_ns':ends[name]-starts[name],
                'controlled_barrier':True,'pending_after':result(d.call(dict(action='lifecycle',root=r,operation='quota')))})
        cleanup(d,r)
    return rows
if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);a.add_argument('--tiers',nargs='+',type=int,default=[1000,10000,100000]);a.add_argument('--contention',action='store_true');ns=a.parse_args()
    d=Driver(ns.exe,ns.base,14400);rows=run(d,ns.tiers,ns.contention);(d.base/'matrix.json').write_text(json.dumps(rows,indent=2))
