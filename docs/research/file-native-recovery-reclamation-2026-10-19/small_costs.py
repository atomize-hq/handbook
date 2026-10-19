"""Matched small fixtures only; uncontrolled caches and concurrent read-only work."""
import argparse,hashlib,json,os,subprocess,time
from pathlib import Path
a=argparse.ArgumentParser();a.add_argument('--before',required=True);a.add_argument('--after',required=True);a.add_argument('--root',required=True);ns=a.parse_args();base=Path(ns.root);base.mkdir(exist_ok=False);rows=[]
for version,exe in [('legacy',ns.before),('profile2',ns.after)]:
 for mode in ['A','B']:
  for n in [16,128]:
   root=base/f'{version}-{mode}-{n}'/'disposable/store'
   def call(action,**fields):
    v=dict(root=str(root),action=action,**fields);env={k:v for k,v in os.environ.items() if not k.startswith('EXP_')};env['EXP_DEFER_REFRESH']='1';start=time.monotonic();p=subprocess.run([exe],input=json.dumps(v).encode(),capture_output=True,timeout=180,env=env);assert p.returncode==0,p.stderr
    out=json.loads(p.stdout);rows.append(dict(profile=version,mode=mode,n=n,input=v,response=out,seconds=time.monotonic()-start,binary_sha256=hashlib.sha256(Path(exe).read_bytes()).hexdigest()));return out['result']
   call('init',mode=mode,consumer='project',count=n)
   if mode=='A':call('refresh')
   for i in range(3):
    ops=[dict(kind='import',id='d000000',text='# matched ordinary edit\r\n'+str(i)+'\n')] if i!=1 else [dict(kind='rename',id='d000001',text='Matched renamed title')]
    s=json.loads((root/'SELECTED.json').read_bytes());call('publish',request=dict(request=f'matched-{i}',base=s,ops=ops,packet=False))
    if mode=='A':call('refresh')
Path(str(base)+'.json').write_text(json.dumps(dict(rows=rows,qualification='Three sparse small edits per cell; uncontrolled caches; overlaps read-only history verifier. Profile2 scans all N metadata headers; documents_visited counts point-header calls and omits this range scan. No isolated causal ratio or SLA.'),indent=2),encoding='utf-8')
