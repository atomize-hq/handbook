"""Synthetic pessimistic byte-ceiling boundary on genuine A/B preparations."""
import argparse,json
from controller import Driver
from checks import result,req,expect_error,cleanup

def run(d):
    rows=[]
    for mode in ['A','B']:
        root=d.base/mode/'disposable/store';r=str(root)
        def call(action,allow=False,**kw):return d.call(dict(action=action,root=r,**kw),allow=allow)
        call('init',mode=mode,consumer='project',count=16);call('lifecycle',operation='setup')
        candidate=result(call('prepare',request=req('one-real-preparation',[])))['candidate']
        path=root/'RESERVATIONS.json';saved=path.read_bytes();registry=json.loads(saved);assert len(registry)==1
        reservation=next(iter(registry.values()));reservation['bytes']=32*1024**3
        path.write_text(json.dumps(registry));before=path.read_bytes()
        # This models pessimistically retained quota, not allocated 32 GiB data.
        refusal=call('prepare',allow=True,request=req('blocked-by-bytes',[]));expect_error(refusal,'quota:admission-full')
        assert path.read_bytes()==before
        observation=result(call('lifecycle',operation='quota'))['reservations'];assert len(observation)==1
        path.write_bytes(saved);cleanup(d,r);call('audit')
        rows.append({'mode':mode,'genuine_prepared_candidate':candidate,'synthetic_reserved_bytes':32*1024**3,'reservation_count_at_refusal':1,'byte_ceiling_refused_before_count_ceiling':True,'registry_unchanged_on_refusal':True,'real_32GiB_allocation':False,'restored_original_registry_and_reclaimed':True})
    return rows

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);ns=a.parse_args()
    d=Driver(ns.exe,ns.base,900);(d.base/'quota-bytes.json').write_text(json.dumps(run(d),indent=2))
