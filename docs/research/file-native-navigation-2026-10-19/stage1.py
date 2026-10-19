"""Separately measured complete renderer (2) versus fact-dirty renderer (3)."""
import argparse,hashlib,json,os,subprocess,sys,time
from pathlib import Path
HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE.parent/'file-native-recovery-reclamation-2026-10-19'))
from oracle import verify

def run(exe,base,n=128):
    base=Path(base).resolve();base.mkdir(parents=True,exist_ok=False);events=[];rows=[]
    def call(root,action,profile,**kw):
        env={k:v for k,v in os.environ.items() if not k.startswith('EXP_')};env['EXP_NAVIGATION']=str(profile);env['EXP_DEFER_REFRESH']='1'
        payload=dict(root=str(root),action=action,**kw);t=time.monotonic()
        p=subprocess.run([str(exe)],input=json.dumps(payload).encode(),capture_output=True,timeout=120,env=env)
        row=dict(input=payload,profile=profile,seconds=time.monotonic()-t,exit=p.returncode,stdout=p.stdout.decode(),stderr=p.stderr.decode());events.append(row)
        (base/'events.json').write_text(json.dumps(events,indent=2),encoding='utf-8');assert p.returncode==0,row
        return json.loads(p.stdout)
    for profile in [2,3]:
        root=base/f'p{profile}'/'disposable/store';call(root,'init',profile,mode='A',consumer='project',count=n)
        for label,ops in [('payload',[dict(kind='edit',id='d000004',text=' payload delta')]),('label',[dict(kind='label',id='d000005',text='not-navigation')]),('title',[dict(kind='rename',id='d000006',text='Navigation title')]),('relations',[dict(kind='edge',id='d000004',text='cites',target='d000005')]),('batch-payload',[dict(kind='edit',id=f'd{i:06}',text=' batch') for i in range(10,18)])]:
            before=call(root,'inspect',profile)['result'];request=dict(request=label,base=before['selected'],ops=ops,packet=False)
            prepared=call(root,'prepare',profile,request=request);candidate=prepared['result']['candidate'];c=json.loads((root/candidate).read_bytes())
            selected=call(root,'select',profile,candidate=candidate);refresh=call(root,'refresh',profile);audit=call(root,'audit',profile)
            independent=verify(root)
            pm=prepared['metrics'];sm=selected['metrics']
            if profile==3 and label in ['payload','label','batch-payload']:
                assert pm['navigation_parts']==sm['navigation_parts']==0
                assert pm['range_rows'].get('D',0)==sm['range_rows'].get('D',0)==0
            if profile==2:assert pm['range_rows']['D']==n
            rows.append(dict(profile=profile,n=n,case=label,prepare=prepared,select=selected,refresh=refresh,audit=audit,independent=independent,replay_objects=len(c['replay_objects']),replay_files=len(c['replay_files'])))
            (base/'summary.json').write_text(json.dumps(rows,indent=2),encoding='utf-8')
    return rows
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--exe',required=True);p.add_argument('--base',required=True);p.add_argument('--n',type=int,default=128);a=p.parse_args();run(Path(a.exe),a.base,a.n)
