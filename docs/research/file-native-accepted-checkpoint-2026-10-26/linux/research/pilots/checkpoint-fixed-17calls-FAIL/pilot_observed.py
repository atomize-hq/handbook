"""Observation-only wrapper; original worker, recipe and oracle bytes preserved."""
import sys,json,time,hashlib,uuid
from pathlib import Path
sys.dont_write_bytecode=True
import pilot_worker as w
ROOT=Path(__file__).resolve().parent
original=w.Host.call

def snapshot(root):
    row={'monotonic_ns':time.monotonic_ns(),'wall_ns':time.time_ns(),'documents':{},'errors':[]}
    def read(name,bound=4*1024**2):
        try:
            p=root/name
            if not p.exists():return None
            if p.stat().st_size>bound:raise ValueError('snapshot-bound')
            raw=p.read_bytes();v=json.loads(raw);row['documents'][name]={'sha256':hashlib.sha256(raw).hexdigest(),'bytes':len(raw),'value':v};return v
        except Exception as e:row['errors'].append({'path':name,'error':repr(e)});return None
    store=read('STORE.json',65536);read('SELECTED.json',65536)
    physical=(store or {}).get('physical') or {}
    if physical.get('plan'):read('physical-plans/'+physical['plan']+'.json')
    pd=root/'physical-progress'
    if pd.exists():
        for f in sorted(pd.iterdir())[:512]:
            if f.is_file():
                v=read('physical-progress/'+f.name)
                pending=(v or {}).get('physical') or {} if isinstance(v,dict) else {}
                if pending.get('plan'):read('physical-plans/'+pending['plan']+'.json')
    try:
        raw=(root/'STORE.json').read_bytes();row['store_stable_during_snapshot']=hashlib.sha256(raw).hexdigest()==row['documents'].get('STORE.json',{}).get('sha256')
    except Exception as e:row['errors'].append({'path':'STORE.json second read','error':repr(e)});row['store_stable_during_snapshot']=False
    return row

def call(self,root,action,*,timeout=120,code=0,env=None,on_started=None,**fields):
    observe=action=='packing' and fields.get('operation')=='cleanup'
    row={'token':uuid.uuid4().hex,'action':action,'fields':fields,'status':'STARTED'} if observe else None
    if observe:row['before']=snapshot(root)
    try:
        answer=original(self,root,action,timeout=timeout,code=code,env=env,on_started=on_started,**fields)
        if observe:row['response']=answer;row['status']='RETURNED'
        return answer
    except BaseException as e:
        if observe:row['error']=repr(e);row['status']='RAISED'
        raise
    finally:
        if observe:
            row['after']=snapshot(root)
            w.write('observations/cleanup-'+row['token']+'.json',row)
w.Host.call=call
w.main()
