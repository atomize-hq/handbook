"""Terminal fences and accepted intent bindings remain effective after restore."""
import argparse,copy,json,subprocess
from pathlib import Path
a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--capsule',required=True);a.add_argument('--output',required=True);ns=a.parse_args();rows=[]
for mode in ['A','B']:
 for n in [16,128]:
  root=Path(ns.capsule)/f'restored-{mode}-{n}/disposable/store';selected=json.loads((root/'SELECTED.json').read_bytes())
  candidate=next(json.loads(p.read_bytes()) for p in (root/'attempts').glob('*/candidate.json') if json.loads(p.read_bytes())['request']['request']=='eligible')
  changed=copy.deepcopy(candidate['request']);changed['ops']=[dict(kind='label',id='d000003',text='different-restored-intent')]
  for request,error in [(dict(request='terminal-fence',base=selected,ops=[],packet=False),'refused:cancelled-fenced'),(changed,'refused:request-id-intent-mismatch')]:
   payload=dict(root=str(root),action='publish',request=request);p=subprocess.run([ns.exe],input=json.dumps(payload).encode(),capture_output=True,timeout=180);response=json.loads(p.stdout);assert p.returncode==2 and response['error']==error
   assert json.loads((root/'SELECTED.json').read_bytes())==selected;rows.append(dict(mode=mode,n=n,input=payload,expected_refusal=error,response=response))
Path(ns.output).write_text(json.dumps(rows,indent=2),encoding='utf-8')
