"""Eligible preparation retains all deterministic replay dependencies across GC."""
import argparse,json
from pathlib import Path
from controller import Driver
from checks import req,op,result,expect_error,reseal,cleanup

def run(d):
    rows=[]
    for mode in ['A','B']:
        root=d.base/mode/'disposable/store';r=str(root)
        def call(action,allow=False,**kw):return d.call(dict(action=action,root=r,**kw),allow=allow)
        call('init',mode=mode,consumer='project',count=512);call('lifecycle',operation='setup')
        candidate=result(call('prepare',request=req('large-overlay',[op('label',id=f'd{i:06}',text=f'eligible-{i:03}') for i in range(256)])))['candidate']
        path=root/candidate;saved=path.read_bytes();c=json.loads(saved);assert c['version']==2 and c['replay_objects']
        c['replay_objects']=[];path.write_text(json.dumps(c));reseal(root,path)
        expect_error(call('select',allow=True,candidate=candidate),'replay-dependencies')
        path.write_bytes(saved);reseal(root,path)
        call('lifecycle',operation='collect');call('select',candidate=candidate);call('audit')
        selected=result(call('inspect'))['selected']
        call('publish',request=req('packet-base',[op('import',id='d000000',text='# base\r\n')],base=selected,packet=True))
        selected=result(call('inspect'))['selected']
        candidate=result(call('prepare',request=req('intermediate-imports',[op('import',id='d000000',text='# intermediate\r\n'),op('import',id='d000000',text='# final\r\n')],base=selected,packet=True)))['candidate']
        c=json.loads((root/candidate).read_bytes());private_sources=[p for p in c['replay_files'] if p.startswith('originals/')];assert len(private_sources)==2
        call('lifecycle',operation='collect');assert all((root/p).exists() for p in private_sources)
        call('select',candidate=candidate);call('audit')
        selected=result(call('inspect'))['selected']
        candidate=result(call('prepare',request=req('abandoned-intermediates',[op('import',id='d000000',text='# abandoned first\r\n'),op('import',id='d000000',text='# abandoned final\r\n')],base=selected)))['candidate']
        c=json.loads((root/candidate).read_bytes());private_sources=[p for p in c['replay_files'] if p.startswith('originals/')]
        cleanup(d,r);assert all(not (root/p).exists() for p in private_sources);call('audit')
        rows.append({'mode':mode,'large_eligible_reopen_after_gc':True,'intermediate_payload_and_packet_reopen_after_gc':True,'forged_dependency_list_refused':True,'abandoned_intermediate_sources_reclaimed':True})
    return rows

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);ns=a.parse_args()
    d=Driver(ns.exe,ns.base,1800);(d.base/'replay-lifetime.json').write_text(json.dumps(run(d),indent=2))
