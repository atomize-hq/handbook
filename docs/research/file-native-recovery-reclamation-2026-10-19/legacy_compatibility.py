"""Frozen legacy roots/candidates stay exact; new preparations target profile 2."""
import argparse,hashlib,json,os,shutil,subprocess
from pathlib import Path
from oracle import verify
a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--fixtures',required=True);a.add_argument('--base',required=True);a.add_argument('--create',action='store_true');a.add_argument('--packet',action='store_true');ns=a.parse_args();base=Path(ns.base);base.mkdir(parents=True,exist_ok=False);rows=[]
for mode in ['A','B']:
 root=base/mode
 if not ns.create:shutil.copytree(Path(ns.fixtures)/mode,root)
 def call(action,code=0,**fields):
  env={k:v for k,v in os.environ.items() if not k.startswith('EXP_')}
  if ns.create:env['EXP_DEFER_REFRESH']='1'
  p=subprocess.run([ns.exe],input=json.dumps(dict(root=str(root),action=action,**fields)).encode(),capture_output=True,timeout=120,env=env);assert p.returncode==code,(p.returncode,p.stdout,p.stderr);return json.loads(p.stdout)['result']
 if ns.create:
  call('init',mode=mode,count=16,consumer='project')
  candidate=call('prepare',request=dict(request='legacy-eligible',base=None,ops=[],packet=ns.packet))['candidate'];(root/'FIXTURE-CANDIDATE.json').write_text(json.dumps(candidate));continue
 old=call('inspect')['selected'];old_bytes=(root/f"publications/{old['publication']}.json").read_bytes();call('audit')
 if mode=='A':call('resume')
 call('select',candidate=json.loads((root/'FIXTURE-CANDIDATE.json').read_bytes()))
 call('publish',request=dict(request='new-profile-target',base=call('inspect')['selected'],ops=[dict(kind='delete',id='d000000')],packet=False));call('audit');verify(root)
 assert (root/f"publications/{old['publication']}.json").read_bytes()==old_bytes
 assert 'd000000' not in call('dump')['state']['docs']
 rows.append(dict(mode=mode,legacy_root_exact=True,legacy_candidate_replayed=True,new_target_profile2=True,old_selected=old))
(base/'legacy-compatibility.json').write_text(json.dumps(rows,indent=2));print(json.dumps(rows,indent=2))
