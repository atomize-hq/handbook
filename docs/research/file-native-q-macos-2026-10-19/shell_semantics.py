import pathlib,subprocess,json,sys,hashlib
r=pathlib.Path(__file__).parent;sys.path.insert(0,'/Users/spensermcconnell/.codex/worktrees/file-native-q-macos-20261019/handbook/docs/research/file-native-recovery-reclamation-2026-10-19');from oracle import verify
rows=[]
for mode in ['A','B']:
 root=r/f'shell-{mode}';events=[]
 def call(action,**kw):
  q=subprocess.run([r/'final-engine'],input=json.dumps(dict(root=str(root),action=action,**kw)).encode(),capture_output=True,timeout=120);events.append(dict(action=action,fields=kw,exit=q.returncode,stdout=q.stdout.decode(),stderr=q.stderr.decode()));(r/f'evidence/shell-{mode}-events.json').write_text(json.dumps(events,indent=2));assert q.returncode==0;return json.loads(q.stdout)['result']
 call('init',mode=mode,consumer='project',count=16)
 call('publish',request=dict(request='ordered-multiple',base=json.loads((root/'SELECTED.json').read_bytes()),packet=False,ops=[dict(kind='edge',id='d000001',text='cites',target='d000003'),dict(kind='edge',id='d000001',text='cites',target='d000003'),dict(kind='edge',id='d000001',text='requires',target='d000004')]))
 verify(root);p=call('inspect')['publication'];tree=root/('view' if mode=='A' else 'generations/'+p['attempt'])
 script=r/f'semantic-{mode}.sh';script.write_text('set -euo pipefail\n'+'tree='+__import__('shlex').quote(str(tree))+'\n'+"find \"$tree\" -type f\ngrep -n -E 'id:|title:|source:|kind:|target:|external:' \"$tree/collection.yaml\"\n"+"awk '/  - id: \"d000001\"/{f=1;next} f && /  - id:/{exit} f{print}' \"$tree/collection.yaml\" > "+__import__('shlex').quote(str(r/f'edges-{mode}.txt'))+'\n')
 q=subprocess.run(['/bin/bash',script],capture_output=True,text=True,timeout=120);assert q.returncode==0;block=(r/f'edges-{mode}.txt').read_text();targets=[line.strip().split(': ',1)[1] for line in block.splitlines() if line.strip().startswith('target:')];assert targets==['"d000002"','"d000003"','"d000003"','"d000004"'];assert 'external: true' in q.stdout and 'https://unavailable.invalid/source' in q.stdout
 (r/f'evidence/shell-semantics-{mode}.stdout').write_text(q.stdout)
 rows.append(dict(mode=mode,ordered_targets=targets,multiple=True,directed_source='d000001',typed=True,external=True,independent_oracle=True))
(r/'evidence/shell-semantics.json').write_text(json.dumps(rows,indent=2))
