"""Bounded exact-candidate native growth cells; only successful disposable cells removed."""
import json,sys,time,shutil
from pathlib import Path
import supervise as s

s.STAGES=json.loads((s.BASE/'stages.json').read_bytes())
exe=s.BASE/'build/release/file-native-incremental'
rows=[]
for n in [128,1000,10000]:
    for mode in ['A','B']:
        label=f'n{n}-{mode}-p4';base=s.BASE/'matrix'/label
        current=s.census(s.BASE)
        sources=(n//10)*126976+(n%10)*65536
        forecast=sources*(3 if mode=='B' else 2)+64*1024**2
        admission=dict(cell=label,current=current,forecast_additional_bytes=forecast,additional_total_bound=current['logical_bytes']+forecast+32*1024**2,local_and_return_allowance_bytes=32*1024**2,new_file_forecast=current['files']+4*n+10000,global_parent_forecast_bytes=40828300920,global_parent_forecast_files=4282696)
        (s.BASE/(label+'-admission.json')).write_text(json.dumps(admission,indent=2))
        assert admission['additional_total_bound']<=768*1024**2,'combined-lane-capacity-censor'
        assert admission['new_file_forecast']<=100000,'new-files-censor'
        s.stage(label,[sys.executable,str(s.H/'measure.py'),'--exe',str(exe),'--base',str(base),'--n',str(n),'--mode',mode,'--profile','4','--deadline',str(s.CUTOFF)],1800,checker=True)
        summary=json.loads((base/'summary.json').read_bytes());assert summary['status']=='success'
        target=base/'cell/disposable/store';assert target.resolve().is_relative_to((s.BASE/'matrix').resolve())
        footprint=s.census(target);shutil.rmtree(target)
        rows.append(dict(cell=label,summary=summary,successful_disposable_fixture_removed=True,removed_census=footprint,owned_after=s.census(s.BASE)))
        (s.BASE/'matrix-cells.json').write_text(json.dumps(rows,indent=2))
        (s.BASE/'matrix-stages.json').write_text(json.dumps(s.STAGES,indent=2))
(s.BASE/'host-after-matrix.json').write_text(json.dumps(s.host_scope(),indent=2))
