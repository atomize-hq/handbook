"""Read-only native authority/outcome diagnosis after an independent case cap."""
import argparse,json,pathlib
from experiment import Driver,cmd
import baseline_harness as b
p=argparse.ArgumentParser();p.add_argument('--base',type=pathlib.Path,required=True);p.add_argument('--root',type=pathlib.Path,required=True);p.add_argument('--exe',type=pathlib.Path,required=True);p.add_argument('--backend',choices=['manifest','cow'],required=True);a=p.parse_args()
d=Driver(a.base,a.exe,'cap-diagnosis',a.backend);inspection=d.call(cmd(a.root,'inspect',audit=False),allowed=(0,2))
outcome=d.call(cmd(a.root,'resolve',request_id='initial'),allowed=(0,2)) if inspection['status']=='ok' else None
view=b.load(a.root/'VIEW-STATUS.json') if (a.root/'VIEW-STATUS.json').exists() else None
record=dict(root=str(a.root),backend=a.backend,session=d.session,helper_sha256=b.sha(pathlib.Path(__file__).read_bytes()),inspection=inspection,view_status=view,initial_outcome=None if not outcome else {k:v for k,v in outcome['result'].items() if k!='changed'},qualification='native read-only inspection after capped case; original attempt remains censored; no candidate promoted or view resumed')
if outcome:record['initial_changed_count']=len(outcome['result'].get('changed',[]))
path=d.base/'cap-diagnosis-observations.json';rows=b.load(path) if path.exists() else [];rows.append(record);b.write(path,rows)
print(json.dumps(record,indent=2))
