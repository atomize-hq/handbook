"""Native argv providers and exact unfiltered unit qualification. Import is inert."""
import base64,hashlib,json,os,re,threading,time
from pathlib import Path
def need(ok,message):
 if not ok:raise RuntimeError(message)
def sha(data):return hashlib.sha256(data).hexdigest()
class Commands:
 def __init__(self,host):
  self.host=host;self.f=host.f;self.rows=[];self.calls=[];self.serial=0;self.lock=threading.RLock()
 def run(self,root,action,argv,stdin=b'',*,seconds,env=None,expected=0,code=None):
  if code is not None:expected=code
  f=self.f;ctx=f.context(root);f.clip(seconds)
  need(action in ('ordinary','workspace','oracle','build') and type(argv) is list and argv and all(type(v) is str for v in argv),'native-argv-shape')
  need(type(stdin) is bytes,'native-stdin-wire')
  grants=[(k,g) for k,g in f.contract['argv_grants'].items() if g['case_id']==ctx['case_id'] and g['kind']==action and g['argv']==argv and g['stdin_sha256']==sha(stdin)]
  need(len(grants)==1,'one-exact-reviewed-case-argv-grant');grant_id,g=grants[0]
  need(g['expected_exit']==expected and g['command_env']==(env or {}),'argv-exit-environment-binding')
  mode='bash-script' if action=='ordinary' else ('checker-stdin' if action=='oracle' and stdin else 'argv')
  need(g['mode']==mode,'native-command-mode')
  if action=='ordinary':need(Path(argv[0]).resolve()==Path('/bin/bash').resolve() and argv[1:5]==['--noprofile','--norc','-s','--'] and bool(stdin),'actual-Bash-withheld-source')
  if mode=='checker-stdin':
   need(argv==[f.contract['python']['path'],'-I','-S','-B','-'] and g['checker_source_sha256'] in {v['sha256'] for v in f.contract['source_files'].values()},'exact-reviewed-checker-stdin-source')
  if action=='workspace':
   need(not stdin and '--locked' in argv and '--offline' in argv and '--workspace' in argv and '--release' in argv,'workspace-full-offline-locked')
   need(argv[-2:] in (['--','--list'],['--','--test-threads=1']),'unfiltered-workspace-only')
   need((env or {}).get('CARGO_HOME')==str(f.runtime.admission.ROOT/'cargo-private') and (env or {}).get('CARGO_NET_OFFLINE')=='true','private-offline-cache-only')
  with self.lock:self.serial+=1;name=ctx['run_name']+'-argv-'+str(self.serial)
  input_path=None
  if stdin:
   folder=Path(f.contract['evidence_prefix'])/'argv-input';f.lease.directory(folder);folder.mkdir(parents=True,exist_ok=True)
   input_path=folder/(name+'.stdin');f.lease.file(input_path,len(stdin))
   with input_path.open('xb') as out:out.write(stdin)
  profile=g['outputs'];f.pending[name]={'case':ctx,'argv':argv,'env':None,'input_path':str(input_path),'mode':mode,'grant_id':grant_id}
  options={'name':name,'timeout':f.clip(seconds),'identity_budget':g['identity_budget'],'input_path':input_path,'input_cap':g['input_cap'],'evidence_cap':profile['RSS_cap'],'stdout_cap':profile['stdout_cap'],'stderr_cap':profile['stderr_cap'],'receipt_cap':profile['receipt_cap'],'control_only':False,'expected_exit':expected,'env':None,'command_env':env or {},'start_mode':mode,'argv_grant':grant_id,'case_context':{'parent_run_id':ctx['run_name'],'case_id':ctx['case_id']}}
  owner=f.runtime.roles.OwnedCall('reader' if action=='ordinary' else 'checker',argv,phase=f.lease,**options)
  # The wrapper and both registries precede worker/native activation.
  state={'owner':owner,'case_id':ctx['case_id'],'argv':list(argv),'grant_id':grant_id,'row':None}
  with self.lock:self.calls.append(state)
  owner.activate();failure=None
  try:result=owner.wait()
  except BaseException as exc:
   failure=exc;p=f.runtime.admission.ROOT/'evidence/roles'/name/'result.json'
   if not p.is_file():raise
   result=f.strict(f.read_evidence(p,profile['receipt_cap']))
  stdout=f.read_evidence(Path(result['outputs']['stdout']['path']),profile['stdout_cap'])
  stderr=f.read_evidence(Path(result['outputs']['stderr']['path']),profile['stderr_cap'])
  need(owner.closed,'argv-native-helper-Python-closeout')
  row={'token':name,'case_id':ctx['case_id'],'kind':action,'argv':list(argv),'stdin_sha256':sha(stdin),'command_env':env or {},'executable_sha256':result['binary_sha256'],'exit':result['exit'],'stdout':stdout.decode('utf-8'),'stderr':stderr.decode('utf-8'),'output_complete':result['output_complete'],'native_closure':result['closure'],'result_path':str(f.runtime.admission.ROOT/'evidence/roles'/name/'result.json'),'result_sha256':f.sha_file(f.runtime.admission.ROOT/'evidence/roles'/name/'result.json'),'stdout_sha256':sha(stdout),'stderr_sha256':sha(stderr),'source_binding':result['producer_binding'],'error':repr(failure) if failure else None}
  state['row']=row;self.rows.append(row);self.host._command_rows.append(row)
  f.emit({'kind':'native-argv-result','row':row})
  if failure is not None:raise failure
  need(row['exit']==expected and row['output_complete'],'exact-argv-exit-and-EOF')
  return row
 def close(self):
  errors=[]
  for state in self.calls:
   owner=state['owner']
   if owner.closed:continue
   if not owner._activated:owner.cancel_unstarted();continue
   owner._abort.set()
   try:owner.wait(self.f.clip(2))
   except BaseException as exc:errors.append(repr(exc))
  need(all(v['owner'].closed for v in self.calls),'argv-owner-closure-HOLD')
  return errors
def verify_unit_batch(listing,run,expected,list_argv,run_argv):
 need(listing['argv']==list_argv and run['argv']==run_argv,'exact-unit-argv')
 need(list_argv[-2:]==['--','--list'] and run_argv[-2:]==['--','--test-threads=1'],'unfiltered-unit-argv')
 for row in (listing,run):need(row['exit']==0 and row['output_complete'] and len(row['native_closure'])==2 and all(v['absent'] for v in row['native_closure']) and row['result_sha256'],'unit-native-raw-closure')
 names=re.findall(r'^(.+): test$',listing['stdout'],re.M);passed=re.findall(r'^test (.+) \.\.\. ok$',run['stdout'],re.M)
 simple=lambda name:name.rsplit('::',1)[-1]
 need(len(names)==31 and len(set(names))==31 and {simple(v) for v in names}==set(expected),'all31-catalog-functions')
 need(len(passed)==31 and len(set(passed))==31 and set(passed)==set(names),'all31-named-results')
 totals=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out;',run['stdout'])
 need(totals and sum(int(v[0]) for v in totals)==31 and not any(any(int(n) for n in v[1:]) for v in totals),'all31-zero-filter-ignore-failure')
 return {'tests':names,'actual_test_functions':31,'native_batches':2,'invented_per_test_receipts':False,'list_command':listing['token'],'test_command':run['token'],'list_result_sha256':listing['result_sha256'],'test_result_sha256':run['result_sha256']}
def workspace(c):
 a=c.a;f=a.f;spec=a.m['workspace'];old=getattr(a,'workspace_proof',None)
 if old is not None:
  need(old['final_pin']==f.final_pin and old['source_catalog_sha256']==spec['catalog_sha256'],'same-native-batch-not-heavy-rerun')
  return dict(old,reused_same_native_batch=True)
 need(spec['source_pin']==f.final_pin and spec['reviewed'] is True and spec['catalog_sha256']=='9306071c419670e5d50049d217978e67361a5167c3cdbace1116f908c7c08e6b','exact31-workspace-source')
 catalog=f.strict(Path(spec['catalog_path']).read_bytes());need(sha(Path(spec['catalog_path']).read_bytes())==spec['catalog_sha256'],'catalog-bytes')
 expected=[name for names in catalog['source_unit_catalog'].values() for name in names];need(len(expected)==31 and len(set(expected))==31,'exact31-source-catalog')
 fixture=Path(spec['fixture_root']);f.lease.path(fixture,'F08');need(not fixture.exists(),'one-retained-unit-fixture-lineage')
 f.lease.grow('F08',spec['fixture_max_bytes'],spec['fixture_max_files']);f.lease.directory(fixture);fixture.mkdir(parents=True,exist_ok=False)
 env={'EXP_PACK_TEST_ROOT':str(fixture),'TMPDIR':str(fixture),'CARGO_INCREMENTAL':'0','CARGO_NET_OFFLINE':'true','CARGO_HOME':str(f.runtime.admission.ROOT/'cargo-private')}
 common=[a.m['cargo'],'test','--manifest-path',spec['manifest_path'],'--locked','--offline','--workspace','--release','-j2','--target-dir',spec['target_dir']]
 listing=a.commands.run(c.root,'workspace',common+['--','--list'],seconds=spec['seconds'],env=env)
 result=a.commands.run(c.root,'workspace',common+['--','--test-threads=1'],seconds=spec['seconds'],env=env)
 proof=verify_unit_batch(listing,result,expected,common+['--','--list'],common+['--','--test-threads=1'])
 artifacts={}
 for p in fixture.rglob('*'):
  need(not p.is_symlink(),'unit-symlink')
  if p.is_file():artifacts[str(p.relative_to(fixture))]={'bytes':p.stat().st_size,'sha256':f.sha_file(p)}
 need(artifacts,'complete-unit-fixture-census')
 proof.update(final_pin=f.final_pin,source_catalog_sha256=spec['catalog_sha256'],artifacts=artifacts,artifact_manifest_sha256=sha(json.dumps(artifacts,sort_keys=True,separators=(',',':')).encode()),unit_fixture_prefix=str(fixture),native_host='Mac',status='UNIT_ASSERTIONS_COMPLETE_GUARD_FINALIZATION_PENDING')
 a.workspace_proof=proof;a.emit({'kind':'workspace-all31-unfiltered','proof':proof});return proof
def verify_source_closure(release_bytes,release_sha256):
 need(sha(release_bytes)==release_sha256 and len(release_bytes)<=2*1024**2,'source-release-bytes')
 def pairs(rows):
  obj={}
  for k,v in rows:
   need(k not in obj,'duplicate-release-key');obj[k]=v
  return obj
 r=json.loads(release_bytes,object_pairs_hook=pairs,parse_constant=lambda v:(_ for _ in ()).throw(ValueError(v)))
 need(r['source_pin']=='0628322e4b48e0efad19cdc32238a622d4aaa7a8' and r['parent_released'] is True,'exact-final-source-release')
 root=Path('/private/tmp/file-native-online-macos-20261020')
 for name,row in r['source_files'].items():
  p=Path(name);need(p.is_relative_to(root) and not p.is_symlink() and p.is_file(),'closed-source-member')
  raw=p.read_bytes();need(len(raw)==row['bytes'] and sha(raw)==row['sha256'],'complete-source-closure-before-imports')
 return r
run=workspace  # Explicit reviewed suite_units substitution; same native unfiltered batch.
if __name__=='__main__':raise SystemExit('SOURCE_ONLY_REVIEWED_BOOTSTRAP_AND_FINITE_PHASE_GO_REQUIRED')
