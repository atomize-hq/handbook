import sys,pathlib,json,subprocess,hashlib
r=pathlib.Path(__file__).parent;f=pathlib.Path('/Users/spensermcconnell/.codex/worktrees/file-native-q-macos-20261019/handbook/docs/research/file-native-recovery-reclamation-2026-10-19');sys.path.insert(0,str(f));from oracle import verify;from verify_operational import verify as operational
root=r/'external-empty/disposable/store';events=[]
def call(action,**fields):
 p=subprocess.run([str(r/'engine')],input=json.dumps(dict(root=str(root),action=action,**fields)).encode(),capture_output=True,timeout=120);events.append(dict(action=action,fields=fields,exit=p.returncode,stdout=p.stdout.decode(),stderr=p.stderr.decode()));(r/'evidence/delta-events.json').write_text(json.dumps(events,indent=2));assert p.returncode==0,events[-1];return json.loads(p.stdout)
call('init',mode='A',consumer='project',count=16);external=root/'view/user-empty/child';external.mkdir(parents=True);source=root/'view/docs/item-d000001.txt'
# Choose an actual authored file from navigation rather than an assumed fixture name.
from oracle import navigation
nav=navigation(root/'view');source=root/'view'/nav['d000001']['source'];source.write_bytes(b'external divergent bytes');call('resume');assert external.is_dir();assert any(p.read_bytes()==b'external divergent bytes' for p in (root/'divergence').rglob('*') if p.is_file());call('audit');verify(root)
rows=[dict(case='external-empty-directory-and-divergent-bytes',status='success')]
for mode in ['A','B']:
 for n in [16,128]:
  clone=r/f'capsule/restored-{mode}-{n}/disposable/store';rows.append(dict(case='independent-operational-restore',mode=mode,n=n,result=operational(clone)))
(r/'evidence/delta.json').write_text(json.dumps(rows,indent=2));print(json.dumps(rows))
