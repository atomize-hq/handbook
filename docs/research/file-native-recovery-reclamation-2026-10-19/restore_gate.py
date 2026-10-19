"""Fresh operational reopening is a separate prerequisite from byte restore."""
import argparse,hashlib,json,os,subprocess,time
from pathlib import Path
a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--root',required=True);a.add_argument('--archive',required=True);a.add_argument('--verification',required=True);a.add_argument('--output',required=True);ns=a.parse_args();root=Path(ns.root).resolve();m=json.loads((Path(ns.archive)/'manifest.json').read_bytes());v=json.loads(Path(ns.verification).read_bytes());assert v['selected']==m['selected'];assert root!=Path(m['original']).resolve() and not (root/'DISPOSABLE.json').exists();events=[]
def call(action,**fields):
    payload=dict(action=action,root=str(root),**fields);env={k:v for k,v in os.environ.items() if not k.startswith('EXP_')};start=time.monotonic();p=subprocess.run([ns.exe],input=json.dumps(payload).encode(),capture_output=True,timeout=180,env=env);events.append(dict(input=payload,exit=p.returncode,seconds=time.monotonic()-start,stdout=p.stdout.decode(),stderr=p.stderr.decode()));Path(str(ns.output)+'.events.json').write_text(json.dumps(events,indent=2),encoding='utf-8');assert p.returncode==0,events[-1];return json.loads(p.stdout)['result']
head=call('inspect');assert head['selected']==m['selected']
lost=call('resolve',request_id='deep-lost-reply');assert lost['outcome']=='accepted'
if json.loads((root/'STORE.json').read_bytes())['mode']=='A':call('resume')
call('audit');call('lifecycle',operation='quota')
successor=call('publish',request=dict(request='restored-operational-successor',base=head['selected'],ops=[],packet=False));assert successor['status']=='published'
assert call('resolve',request_id='deep-lost-reply')==lost
result=dict(status='operational-restore-verified',destination=str(root),original=m['original'],archive_manifest_sha256=hashlib.sha256((Path(ns.archive)/'manifest.json').read_bytes()).hexdigest(),original_selected=m['selected'],selected=successor['selected'],successor=successor['result'],binary_sha256=hashlib.sha256(Path(ns.exe).read_bytes()).hexdigest(),verification=v,disposable_permission_imported=False)
Path(ns.output).write_text(json.dumps(result,indent=2),encoding='utf-8');print(json.dumps(result,indent=2))
