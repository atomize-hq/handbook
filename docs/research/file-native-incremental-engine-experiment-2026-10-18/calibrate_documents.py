"""Final-core fresh 1k/10k calibration matching the achieved 100k H=2/3/4 trace."""
import argparse,json
from pathlib import Path
from controller import Driver
from checks import result,req,op
from finish_matrix import bounded_source

def run(d):
    rows=[]
    for n in [1000,10000]:
        root=d.base/f'n{n}/disposable/store';r=str(root)
        initial=result(d.call(dict(action='init',root=r,mode='A',consumer='project',count=n),cap=600));assert initial['status']=='published'
        assert not (root/'PENDING.json').exists() and json.loads((root/'VIEW.json').read_bytes())['selected']==initial['selected']
        for number,k in enumerate([1,8,64],1):
            before=result(d.call(dict(action='inspect',root=r)))
            assert before['publication']['sequence']==number-1
            ids=[f'd{(i*n//k+17)%n:06}' for i in range(k)]
            publication=d.call(dict(action='publish',root=r,request=req(f'genuine-{n}-{k}',[op('import',id=id,text=bounded_source(id,k)) for id in ids],base=before['selected'])),cap=180,env={'EXP_DEFER_REFRESH':'1'})
            out=result(publication);assert out['status']=='published' and out['result']['outcome']=='accepted'
            refresh=d.call(dict(action='refresh',root=r),cap=180)
            assert not (root/'PENDING.json').exists() and json.loads((root/'VIEW.json').read_bytes())['selected']==out['selected']
            after=result(d.call(dict(action='inspect',root=r)));assert after['publication']['sequence']==number
            rows.append({'n':n,'k':k,'selected_publications':number+1,'publication':publication,'refresh':refresh})
            (d.base/'calibration.json').write_text(json.dumps(rows,indent=2))
        d.call(dict(action='audit',root=r),cap=180)
        audited=result(d.call(dict(action='view-audit',root=r)));assert not audited['missing'] and not audited['edited']
    return rows

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);ns=a.parse_args()
    assert not Path(ns.base).exists(),'fresh observation and store roots required'
    d=Driver(ns.exe,ns.base,900);(d.base/'calibration.json').write_text(json.dumps(run(d),indent=2))
