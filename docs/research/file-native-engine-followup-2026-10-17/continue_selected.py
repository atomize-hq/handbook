"""Bounded post-cap A recovery/continuation; never initialize or promote survivors.

Run after the measured scheduler. Original initialization censored events and
execution failures remain untouched. New native events carry post-cap sessions;
the view recovery timer is separate from initialization and later commit clocks.
"""
import argparse,json,pathlib,time,traceback
from experiment import Driver,cmd
from workloads import allocation_check
import baseline_harness as b
p=argparse.ArgumentParser();p.add_argument('--base',type=pathlib.Path,required=True);p.add_argument('--exe',type=pathlib.Path,required=True);a=p.parse_args()
assert (a.base/'execution-completed.json').exists(),'wait for measured scheduler'
helper_sha256=b.sha(pathlib.Path(__file__).read_bytes());rows=[]
for backend in ['manifest','cow']:
    base=a.base/backend;root=base/'growth/100000-A-project'
    if not (root/'SELECTED.json').exists():
        rows.append(dict(backend=backend,status='no selected A authority; continuation not attempted'));continue
    d=Driver(base,a.exe,'growth',backend);info=d.inspect(root)
    status=b.load(root/'VIEW-STATUS.json') if (root/'VIEW-STATUS.json').exists() else {'status':'not-started'}
    view_complete=status['status']=='complete' and status.get('target_publication')==info['selected']['publication']
    if len(info['history'])==3 and view_complete:continue
    assert 1<=len(info['history'])<=3
    start=time.perf_counter_ns();before=info['selected'];row=dict(root=str(root),backend=backend,session=d.session,helper_sha256=helper_sha256,before=before,selected_publications_before=len(info['history']),caps_seconds=dict(resume=900,publish=900,packet=600),qualification='post-cap continuation of positive selected authority; original censored init preserved; no unselected candidate promoted')
    try:
        # Measure missing/divergent view bytes rather than assuming how far the
        # interrupted refresh got. Reserve two *full* metadata rewrites/packets,
        # although COW updates usually share pages. The fixed fixture operations
        # change at most 64 bounded fixture bodies; 128 MiB covers originals, navigation,
        # transient replacement files, metadata expansion and diagnostic logs.
        selected,publication,state=b.selected_state(root);assert selected==before
        assert len(state['docs'])==100000
        missing=divergent=authored_existing=0
        for doc in state['docs'].values():
            source=root/doc['original'];assert source.stat().st_size<=131072
            dest=root/'view'/doc['path']
            if dest.is_file():authored_existing+=((dest.stat().st_size+4095)//4096)*4096
            if not dest.is_file() or b.sha(dest.read_bytes())!=doc['digest']:
                missing+=((source.stat().st_size+4095)//4096)*4096
                if dest.is_file():divergent+=((dest.stat().st_size+4095)//4096)*4096
        metadata=(((root/publication['candidate']['state']).stat().st_size+4095)//4096)*4096
        if backend=='cow':metadata+=b.inventory(root/'objects')['allocated_file_bytes']
        packet=b.inventory(root/publication['candidate']['closure'])['allocated_file_bytes']
        other_view=max(0,b.inventory(root/'view')['allocated_file_bytes']-authored_existing)
        reserve=missing+divergent+other_view+2*metadata+2*packet+128*1024**2
        row['additional_file_reserve']=dict(bytes=reserve,missing_view_payload=missing,divergent_backup=divergent,other_view_backup_bound=other_view,two_full_metadata=2*metadata,two_full_packets=2*packet,other_margin=128*1024**2)
        allocation_check(reserve);d.budget(reserve)
        if not view_complete:
            resumed=d.call(cmd(root,'resume'),timeout=900)
            assert resumed['result']['status']=='view-resumed'
            row['view_recovery_engine_ms']=resumed['wall_ns']/1e6
            assert b.load(root/'SELECTED.json')==before
        done={v['request'] for v in info['history']}
        for count in [1,64]:
            name=f'commit-{count}-packet-True'
            if name in done:continue
            request={**b.request(name,b.batch_ops(count,name)),'packet':True}
            out=d.call(cmd(root,'publish',mode='A',policy='slot',request=request),timeout=900)
            assert out['result']['status']=='published' and out['result']['result']['outcome']=='accepted'
        final=d.inspect(root);assert len(final['history'])==3
        final_view=b.load(root/'VIEW-STATUS.json');assert final_view['status']=='complete' and final_view['target_publication']==final['selected']['publication']
        row.update(status='selected continuation complete',selected_publications_after=3,after=final['selected'],view_status=final_view)
        allocation_check()
    except Exception as error:
        row.update(status='continuation stopped',error=str(error),traceback=traceback.format_exc())
    row['continuation_controller_ns']=time.perf_counter_ns()-start;rows.append(row)
    b.write(base/'continuation-observations.json',[row])
    print(json.dumps({k:v for k,v in row.items() if k not in ['traceback','view_status']}),flush=True)
b.write(a.base/'continuation-observations.json',rows)
