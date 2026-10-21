"""Native source-bound fixture operations and direct case assertions."""
import base64,hashlib,json,math,os,stat,zlib
from pathlib import Path
def need(ok,message):
 if not ok:raise RuntimeError(message)
class Extensions:
 def __init__(self,facade):
  self.f=facade;self.records={};self.serial=0
 def relative(self,root,name):
  need(type(name) is str and name and '\\' not in name and ':' not in name,'relative-file-name')
  rel=Path(name);need(not rel.is_absolute() and all(p not in ('','.','..') for p in name.split('/')),'owned-relative-components')
  p=Path(root)/rel
  need(p.is_relative_to(Path(root)),'root-confined-path')
  for q in [p,*p.parents]:
   if q.is_relative_to(Path(root)):need(not q.is_symlink(),'fixture-alias-refused')
  return p
 def inventory(self,root):
  ctx=self.f.context(root);root=Path(root);self.f.lease.path(root,'F08');files={};directories=[]
  cap=ctx['forecast']['inventory_file_cap'];max_files=ctx['forecast']['inventory_files'];max_bytes=ctx['forecast']['inventory_bytes'];total=0
  for directory,dirs,names in os.walk(root,followlinks=False,onerror=lambda e:(_ for _ in ()).throw(e)):
   for name in dirs:
    p=Path(directory)/name;need(not p.is_symlink(),'inventory-directory-alias');directories.append(str(p.relative_to(root)))
   for name in names:
    p=Path(directory)/name;s=p.lstat();need(stat.S_ISREG(s.st_mode) and s.st_nlink==1 and s.st_size<=cap,'inventory-regular-owned-file')
    key=str(p.relative_to(root));files[key]={'bytes':s.st_size,'sha256':self.f.file_sha256(root,key),'device':s.st_dev,'inode':s.st_ino,'mtime_ns':s.st_mtime_ns}
    total+=s.st_size;need(len(files)<=max_files and total<=max_bytes,'complete-inventory-cap')
  return {'files':files,'directories':sorted(directories),'logical_bytes':total,'durable_pins_included':True}
 def oracle(self,root,checkpoint,known_requests=None,required_mapped_keys=()):
  before=self.inventory(root);ctx=self.f.context(root)
  result=self.f.decoder.oracle(root=root,checkpoint=checkpoint,final_pin=self.f.final_pin,known_requests=known_requests or {},required_mapped_keys=required_mapped_keys)
  need(result['final_pin']==self.f.final_pin and result['engine_code_imported'] is False and result['engine_audit_used'] is False,'independent-oracle-source')
  need(result['status']==ctx['oracle_expected'][checkpoint],'exact-source-mapped-oracle-disposition')
  if result['status']=='passed':
   required={'complete_physical','all_accepted_outcomes','exact_request_intents','selected_membership','graph','source_bytes','navigation','provenance','retired_refusals','b_loose_controls','ordinary_selected_once_b','read_only_inventory'}
   need(set(result['checks'])>=required and all(result['checks'][k] is True for k in required),'complete-independent-oracle')
  else:need(result['status']=='refused' and result['refusal']==ctx['oracle_refusals'][checkpoint],'specific-independent-malformation-refusal')
  after=self.inventory(root);need(before==after and result['source_unchanged'] is True,'read-only-oracle-inventory')
  receipt=result['receipt'];raw=self.f.read_evidence(Path(receipt['path']),ctx['forecast']['oracle_receipt_cap'])
  need(self.f.sha(raw)==receipt['sha256'],'independent-oracle-receipt-bytes')
  self.f.emit({'kind':'independent-oracle','case_id':ctx['case_id'],'checkpoint':checkpoint,'result':result,'before':before,'after':after})
  return result
 def mapping(self,root,key):
  self.f.context(root);result=self.f.decoder.mapping(root,key)
  need(result['independent'] is True and result['source_pin']==self.f.final_pin and result['receipt_sha256'],'independent-final-mapping')
  return result
 def backlog(self,root):
  before=self.inventory(root);result=self.f.decoder.backlog(root,inventory=before,source_pin=self.f.final_pin)
  need(set(result)>={'unprotected','protected','excluded','receipt_sha256'} and all(type(result[k]) is int and result[k]>=0 for k in ('unprotected','protected','excluded')),'complete-independent-backlog')
  need(self.inventory(root)==before,'backlog-read-only')
  return result
 def quiesce(self,root):
  self.f.context(root);held=[]
  for call in self.f.calls:
   if call.root!=Path(root) or call.owner.closed:continue
   phase=(call.env or {}).get('EXP_PAUSE');mapping=self.f.contract['checkpoints'].get(phase)
   need(call.request['action'] in self.f.reader_actions and mapping and mapping['dependent_io_paused'] is True,'live-mutator-prevents-quiescence')
   proof=self.f.life.active_proof(call);held.append({'call_id':call.call_id,'proof':proof,'phase':phase})
  self.f.emit({'kind':'native-quiescence','case_id':self.f.context(root)['case_id'],'paused_readers_retained':held,'mutators_live':0})
  return {'quiescent':True,'paused_readers_retained':held}
 def crash_source_binding(self,root,phase):
  ctx=self.f.context(root);m=ctx['crash_maps'][phase]
  need(m['source_pin']==self.f.final_pin and m['phase']==phase and m['source_sha256'] in {v['sha256'] for v in self.f.contract['source_files'].values()},'reviewed-crash-source-map')
  return {'status':'reviewed-source-map','final_pin':self.f.final_pin,'checkpoint':phase,'source_sha256':m['source_sha256'],'capability':'constructor:'+phase}
 def removal_proof(self,root,before,phase):
  ctx=self.f.context(root);self.crash_source_binding(root,phase);after=self.inventory(root)
  removed=[{'path':name,'before':v} for name,v in before['files'].items() if name not in after['files']]
  required=ctx['crash_maps'][phase]['required_removals'];need(required and all(any(v['path']==name for v in removed) for name in required),'actual-required-removal-not-planned-credit')
  for v in removed:need(not self.relative(root,v['path']).exists(),'removed-path-reappeared')
  result={'actually_removed':True,'removed_paths':removed,'no_planned_deletion_credit':True,'after':after}
  self.f.emit({'kind':'actual-removal','case_id':ctx['case_id'],'phase':phase,'proof':result});return result
 def copy_file(self,source,destination,expected_sha256,cap):
  source=Path(source);dest=self.f.lease.path(Path(destination),'F08')
  need(source.is_file() and not source.is_symlink() and source.stat().st_size<=cap,'bounded-source-copy')
  identity=source.stat();need(self.f.sha_file(source)==expected_sha256,'copy-source-hash')
  self.f.lease.file(dest,identity.st_size);self.f.lease.directory(dest.parent);dest.parent.mkdir(parents=True,exist_ok=True)
  h=hashlib.sha256();count=0
  with source.open('rb') as inp:
   with dest.open('xb') as out:
    for block in iter(lambda:inp.read(65536),b''):
     self.f.lease.auth();count+=len(block);need(count<=identity.st_size,'copy-source-growth');h.update(block);out.write(block)
  final=source.stat();need((final.st_dev,final.st_ino,final.st_size,final.st_mtime_ns)==(identity.st_dev,identity.st_ino,identity.st_size,identity.st_mtime_ns) and count==identity.st_size and h.hexdigest()==expected_sha256,'copy-exact-source')
  return {'bytes':count,'sha256':h.hexdigest(),'retained_prefix':str(dest)}
 def backup(self,root,destination,exclusions):
  ctx=self.f.context(root);need(exclusions==ctx['backup_exclusions'] and all('*' not in x and '?' not in x for x in exclusions),'finite-reviewed-backup-exclusions')
  before=self.inventory(root);need(all(not name.startswith('physical-pins/') for name in exclusions),'durable-pin-exclusion-refused')
  dest=self.f.lease.path(Path(destination),'F08');need(not dest.exists(),'fresh-backup-prefix');self.f.lease.directory(dest);dest.mkdir(parents=True)
  for name,v in before['files'].items():
   if name not in exclusions:self.copy_file(self.relative(root,name),dest/name,v['sha256'],ctx['forecast']['inventory_file_cap'])
  expected={k:v['sha256'] for k,v in before['files'].items() if k not in exclusions}
  actual={str(p.relative_to(dest)):self.f.sha_file(p) for p in dest.rglob('*') if p.is_file()}
  need(actual==expected and self.inventory(root)==before,'exact-quiescent-backup')
  self.f.emit({'kind':'exact-backup','case_id':ctx['case_id'],'destination':str(dest),'files':actual,'exclusions':exclusions});return actual
 def patch(self,root,spec):
  ctx=self.f.context(root);need(spec['source_sha256']==ctx['source_sha256'] and spec['schema_pin']==ctx['schema_pin'],'source-bound-binary-constructor')
  p=self.relative(root,spec['path']);need(self.f.sha_file(p)==spec['before_sha256'],'constructor-before-byte-binding')
  self.serial+=1;backup=Path(ctx['backup_root'])/('mutation-'+str(self.serial))/spec['path']
  self.copy_file(p,backup,spec['before_sha256'],ctx['forecast']['inventory_file_cap'])
  edits=spec['edits'];need(edits and len(edits)<=128,'bounded-constructor-edits')
  original=p.stat().st_size;maximum=original
  for edit in edits:
   need(set(edit)=={'offset','before_b64','after_b64'} and type(edit['offset']) is int and edit['offset']>=0,'closed-source-mapped-edit')
   before=base64.b64decode(edit['before_b64'],validate=True);after=base64.b64decode(edit['after_b64'],validate=True)
   need(len(before)==len(after) and len(after)<=65536 and edit['offset']+len(after)<=original,'fixed-extent-source-edit')
   maximum=max(maximum,edit['offset']+len(after))
  self.f.lease.rewrite(p,maximum)
  with p.open('r+b') as out:
   for edit in edits:
    out.seek(edit['offset']);before=base64.b64decode(edit['before_b64'],validate=True);need(out.read(len(before))==before,'edit-preimage')
    out.seek(edit['offset']);out.write(base64.b64decode(edit['after_b64'],validate=True))
  need(self.f.sha_file(p)==spec['after_sha256'],'constructor-after-byte-binding')
  result={'path':spec['path'],'before_sha256':spec['before_sha256'],'after_sha256':spec['after_sha256'],'retained_original':str(backup)}
  self.f.emit({'kind':'source-mapped-constructor','case_id':ctx['case_id'],'result':result});return result
 def boundary_file(self,root,spec):
  ctx=self.f.context(root);need(spec['source_sha256']==ctx['source_sha256'] and spec['schema_pin']==ctx['schema_pin'],'boundary-constructor-source')
  pattern=base64.b64decode(spec['pattern_b64'],validate=True);tail=base64.b64decode(spec['tail_b64'],validate=True);count=spec['repeat']
  need(pattern and len(pattern)<=65536 and len(tail)<=65536 and type(count) is int and count>=0,'boundary-pattern-count')
  size=len(pattern)*count+len(tail);need(size==spec['bytes'] and size<=ctx['forecast']['inventory_file_cap'],'exact-boundary-size')
  p=self.relative(root,spec['path']);self.f.lease.file(p,size);self.f.lease.directory(p.parent);p.parent.mkdir(parents=True,exist_ok=True)
  h=hashlib.sha256()
  with p.open('xb') as out:
   for _ in range(count):self.f.lease.auth();out.write(pattern);h.update(pattern)
   out.write(tail);h.update(tail)
  need(h.hexdigest()==spec['sha256'],'exact-boundary-source-bytes')
  return {'path':spec['path'],'bytes':size,'sha256':h.hexdigest()}
 def native_provider_receipt(self,row,expected_binary):
  p=Path(row['result_path']);raw=self.f.read_evidence(p,524288)
  need(self.f.sha(raw)==row['closure_receipt_sha256'],'provider-raw-closure-bytes')
  result=self.f.strict(raw);need(result['binary_sha256']==expected_binary and result['exit']==row['exit'] and not result['failures'],'provider-native-result')
  need(len(result['closure'])==2 and all(v['absent'] and v['identities'] and all(i['returned_bytes']==0 and i['errno'] in (0,3) for i in v['identities']) for v in result['closure']),'provider-two-numeric-absences')
  samples=self.f.read_evidence(p.parent/'samples.jsonl.gz',1048576);d=zlib.decompressobj(31);decoded=d.decompress(samples,16*1024**2)
  need(d.eof and not d.unused_data and not d.unconsumed_tail,'provider-gzip-EOF')
  events=[self.f.strict(v) for v in decoded.splitlines()]
  created=[v for v in events if v['kind']=='role-created-input-withheld']
  need(len(created)==1 and created[0]['ancestor_nonce']==self.f.lease.token['nonce'] and result['active_candidate_source_pin']==self.f.final_pin,'provider-current-case-guard-binding')
  start=[v for v in events if v['kind']=='role-native-start-proof'];first=[v for v in events if v['kind']=='role-input-first-byte']
  need(len(start)==len(first)==1 and start[0]['input_bytes_delivered']==0 and start[0]['monotonic']<first[0]['monotonic'],'provider-positive-start-before-input')
  need(result['output_complete'] and row['raw_sha256']==self.f.sha_file(Path(result['outputs']['stdout']['path'])),'provider-complete-output')
  return result
 def provider_source(self,fn):
  need(self.f.sha_file(Path(fn.__code__.co_filename)) in {v['sha256'] for v in self.f.contract['source_files'].values()},'provider-complete-frozen-source-closure')
 def workspace_slot(self,ctx):
  slot=self.f.contract['workspace'][str(ctx['workspace_slot'])];expected={v['name']:v for v in slot['units']}
  self.provider_source(self.f.runtime.workspace_owned)
  rows=self.f.runtime.workspace_owned(slot,ctx,self.f.lease)
  need(len(rows)==len(expected) and {v['name'] for v in rows}==set(expected),'all-real-unit-identities-in-original-slot')
  for row in rows:
   unit=expected[row['name']]
   need(row['binary_sha256']==unit['binary_sha256'] and row['test_source_sha256']==unit['source_sha256'],'actual-unit-source-binary')
   self.native_provider_receipt(row,unit['binary_sha256'])
   need(row['test_count']==1 and row['exit']==unit['expected_exit'] and row['assertions_complete'] is True and row['native'] is True and row['closure_receipt_sha256'] and row['raw_sha256'],'direct-per-unit-result')
  return {'slot':ctx['workspace_slot'],'units':rows,'all_units_in_slot':True,'no_broad_workspace_label':True}
 def ordinary_routes(self,ctx):
  self.provider_source(self.f.runtime.ordinary_owned)
  specs=self.f.contract['ordinary'];rows=self.f.runtime.ordinary_owned(specs,ctx,self.f.lease)
  need(len(rows)==17 and {v['route_id'] for v in rows}=={v['id'] for v in specs},'all-original-17-ordinary-routes')
  for row in rows:
   self.native_provider_receipt(row,self.f.sha_file(Path('/bin/bash')))
   need(row['native_executable']=='/bin/bash' and row['proof_before_selection'] is True and row['selection_read_count']==1 and row['exact_copy'] is True and row['engine_or_pack_decoder_used'] is False,'actual-Bash-selected-once-extraction')
   need(row['source_sha256'] and row['raw_sha256'] and row['closure_receipt_sha256'] and row['exit']==0,'ordinary-native-source-closure')
  return {'routes':rows,'all_17_original_routes':True}
 def select(self,records,path):
  need(type(path) is list and path,'direct-evidence-selector')
  value=records
  for key in path:
   need(type(key) in (str,int),'selector-component');value=value[key]
  return value
 def assertion(self,records,spec):
  need(set(spec)=={'evidence','operator','expected'},'closed-direct-case-assertion')
  value=self.select(records,spec['evidence']);expected=spec['expected'];op=spec['operator']
  if op=='eq':ok=type(value) is type(expected) and value==expected
  elif op=='ne':ok=value!=expected
  elif op in ('ge','le'):
   need(type(value) in (int,float) and type(expected) in (int,float) and math.isfinite(value) and math.isfinite(expected),'finite-comparison')
   ok=value>=expected if op=='ge' else value<=expected
  elif op=='set_eq':ok=type(value) is list and type(expected) is list and sorted(value)==sorted(expected)
  elif op=='contains':ok=type(value) in (list,dict,str) and expected in value
  elif op=='all_true':ok=type(value) in (dict,list) and bool(value) and all(v is True for v in (value.values() if type(value) is dict else value))
  else:raise RuntimeError('unknown-direct-assertion-operator')
  need(ok,'case-assertion-failed:'+repr(spec['evidence']));return {'passed':True,'evidence':spec['evidence'],'operator':op,'observed':value}
 def execute_recipe(self,harness,root,ctx):
  recipe=ctx['recipe'];need(set(recipe)=={'source_sha256','steps','max_steps','case_args'} and recipe['source_sha256']==ctx['source_sha256'],'reviewed-constructor-recipe')
  steps=recipe['steps'];need(type(steps) is list and 0<len(steps)<=recipe['max_steps']<=4096,'bounded-case-constructor')
  records={};handles={};controls={}
  for step in steps:
   self.f.lease.auth();need(type(step) is dict and set(step)=={'id','kind','args'} and step['id'] not in records,'closed-unique-constructor-step')
   kind,args=step['kind'],step['args']
   if kind=='engine':
    need(args['action'] in ctx['calls'],'only-existing-source-mapped-engine-actions')
    value=harness.call(root,**args)
   elif kind=='patch':value=self.patch(root,args)
   elif kind=='boundary_file':value=self.boundary_file(root,args)
   elif kind=='inventory':value=self.inventory(root)
   elif kind=='oracle':value=self.oracle(root,**args)
   elif kind=='backlog':value=self.backlog(root)
   elif kind=='mapping':value=self.mapping(root,**args)
   elif kind=='quiesce':value=self.quiesce(root)
   elif kind=='backup':value=self.backup(root,**args)
   elif kind=='workspace':value=self.workspace_slot(ctx)
   elif kind=='ordinary':value=self.ordinary_routes(ctx)
   elif kind=='pause':
    control,handle=harness.paused(root,**args);controls[step['id']]=control;handles[step['id']]=handle;value={'authentic_ready':True,'control':{k:v for k,v in control.items() if k not in ('call','proof')}}
   elif kind=='release_join':
    self.f.release(controls[args['handle']]);value=self.f.wait(handles[args['handle']],args['timeout'])
   elif kind=='interrupt':
    h=handles[args['handle']];self.f.interrupt(h,args['operation']);value=self.f.finish_interrupted(h,args['operation'])
   elif kind=='removal':value=self.removal_proof(root,records[args['before']],args['phase'])
   elif kind=='shared_schedule':
    method=args['method'];binding=ctx['shared_schedules'][method];fn=self.f.runtime.shared_schedule(binding)
    need(self.f.sha_file(Path(fn.__code__.co_filename))==binding['sha256'],'latest-shared-schedule-source')
    value=fn(harness,root,**args['fields'])
   else:raise RuntimeError('unreviewed-constructor-step:'+kind)
   records[step['id']]=value
   self.f.emit({'kind':'case-constructor-step','case_id':ctx['case_id'],'step_id':step['id'],'operation':kind,'result':value})
  self.records[ctx['case_id']]=records;return records
 def assert_case(self,ctx):
  plan=self.f.case_map[ctx['case_id']];records=self.records[ctx['case_id']];proofs={}
  for name in plan['required_checks']:proofs[name]=self.assertion(records,ctx['case_assertions'][name])
  calls=[v for v in self.f.calls if v.case['case_id']==ctx['case_id']]
  need(all(v.owner.closed for v in calls),'case-native-closure-before-assertions')
  providers=[v for value in records.values() if type(value) is dict for v in value.get('units',value.get('routes',[]))]
  need(bool(calls or providers),'actual-native-case-evidence-required')
  need(all(v.native_identity and v.row and v.row['source_binding']['active_candidate_source_pin']==self.f.final_pin for v in calls),'actual-call-source-and-native-binding')
  self.f.lease.verify_sources()
  oracles=[v for v in records.values() if type(v) is dict and v.get('engine_code_imported') is False and v.get('engine_audit_used') is False]
  need(oracles and all(v['source_unchanged'] is True for v in oracles),'actual-read-only-independent-oracle-required')
  need(any('oracle'==s['kind'] for s in ctx['recipe']['steps']),'case-full-independent-oracle-required')
  result={'case_id':ctx['case_id'],'case_assertions':proofs,'required_checks_complete':True,'status':'NATIVE_CASE_ASSERTIONS_PRESENT_UNQUALIFIED','guard_final_receipt_pending':True}
  self.f.emit({'kind':'direct-case-assertions','result':result});return result
 def run_case(self,harness,root,case_id):
  cfg=self.f.contract['cases'][case_id]
  with harness.case(case_id,root,cfg['forecast']):self.execute_recipe(harness,root,cfg)
  return self.assert_case(cfg)
 def extend(self,harness,root,spec):
  ctx=self.f.context(root);plan=self.f.case_map[ctx['case_id']]
  need(spec['case_id']==ctx['case_id'] and spec['capability']==ctx['capability'] and spec['source_sha256']==ctx['source_sha256'] and spec['schema_pin']==ctx['schema_pin'],'exact-native-extension-binding')
  need(set(spec['required_checks'])>=set(plan['required_checks']),'no-skipped-extension-obligations')
  self.execute_recipe(harness,root,ctx);return self.assert_case(ctx)
if __name__=='__main__':raise SystemExit('SOURCE_ONLY_EXACT_FINAL_CONSTRUCTOR_BINDINGS_REQUIRED')
