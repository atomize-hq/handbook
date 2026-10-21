"""Source candidate for the complete native Harness boundary.
No CLI, engine command invention, source loading, or qualification on import.
Construction requires an independently authenticated final runtime release.
"""
import base64,builtins,fcntl,hashlib,json,math,threading,time
from pathlib import Path
ROOT=Path('/private/tmp/file-native-online-macos-20261020')
FUNCTIONAL_CUTOFF=1792581014
READERS=frozenset(('resolve','point','inspect','lookup-bench'))
_ACTIVE=None
def need(ok,message):
 if not ok:raise RuntimeError(message)
def strict(raw):
 def pairs(items):
  row={}
  for key,value in items:
   if key in row:raise ValueError('duplicate-key:'+key)
   row[key]=value
  return row
 return json.loads(raw,object_pairs_hook=pairs,parse_constant=lambda v:(_ for _ in ()).throw(ValueError('nonfinite:'+v)))
def sha(raw):return hashlib.sha256(raw).hexdigest()
def file_sha(path):
 h=hashlib.sha256()
 with Path(path).open('rb') as f:
  for block in iter(lambda:f.read(65536),b''):h.update(block)
 return h.hexdigest()
def finite(value):
 need(type(value) in (int,float) and math.isfinite(value) and value>0,'finite-positive-timeout');return float(value)
def checked_path(path,base):
 p=Path(path);need(p.is_absolute() and '..' not in p.parts and p.is_relative_to(base),'owned-path')
 for q in [p,*p.parents]:
  if q.is_relative_to(base):need(not q.is_symlink(),'symlink-refused')
 return p
class Output:
 def __init__(self,lease,raw,path):self.lease=lease;self.raw=raw;self.path=path;lease.handles.add(self)
 def __getattr__(self,name):return getattr(self.raw,name)
 def __enter__(self):return self
 def __exit__(self,*exc):self.close()
 def write(self,data):
  with self.lease.lock:
   self.lease.auth();size=len(data.encode(self.raw.encoding)) if isinstance(data,str) else len(data)
   need(self.raw.tell()+size<=self.lease.files[self.path],'owned-file-byte-cap');return self.raw.write(data)
 def writelines(self,rows):
  for row in rows:self.write(row)
 def truncate(self,size=None):
  self.lease.auth();size=self.raw.tell() if size is None else size
  need(type(size) is int and 0<=size<=self.lease.files[self.path],'owned-truncate-cap');return self.raw.truncate(size)
 def close(self):
  try:self.raw.close()
  finally:
   if self.raw.closed:
    self.lease.handles.discard(self)
    if self.lease.facade is not None:self.lease.facade.ext.settle_write(self.path)
   else:self.lease.hold=True
class Lease:
 def __init__(self,release,runtime):
  self.r=release;self.runtime=runtime;self.lock=threading.RLock();self.hold=False
  self.files={};self.dirs=set();self.handles=set();self.owners={};self.identities=set();self.used={k:{'bytes':0,'files':0} for k in ('F08','F12')}
  self.token,self.chain=runtime.roles.ancestor_token()
  need(release['parent_released'] is True and release['source_pin']==self.token['source_pin'] and release['claims']==self.token['claims']==runtime.admission.CLAIMS,'actual-token-source-claims')
  need(release['fixed_cutoff']==self.token['fixed_cutoff']==FUNCTIONAL_CUTOFF,'fixed-cutoff')
  issued=release['issued_monotonic'];self.deadline=release['deadline_monotonic']
  finite(release['stage_seconds']);need(all(type(v) in (int,float) and math.isfinite(v) for v in (issued,self.deadline)) and 0<self.deadline-issued<=release['stage_seconds'] and issued<=time.monotonic()<self.deadline,'fixed-local-deadline')
  need(release['identity_budget']<=4096 and type(release['identity_budget']) is int,'identity-cap')
  need(set(release['budget'])=={'F08','F12'},'phase-pools')
  for k,v in release['budget'].items():need(set(v)=={'bytes','files'} and all(type(n) is int and n>=0 for n in v.values()),'pool-shape')
  for key in ('fixture_prefix','evidence_prefix'):checked_path(Path(release[key]),ROOT/('fixtures' if key=='fixture_prefix' else 'evidence'))
  runtime.admission.admit(runtime.admission.snapshot(),release['prospective'],runtime.admission.limits())
  self.verify_sources()
 def verify_sources(self):
  need(bool(self.r['source_files']),'complete-runtime-source-closure')
  for path,v in self.r['source_files'].items():
   p=checked_path(Path(path),ROOT);need(p.is_file() and p.stat().st_size==v['bytes'] and file_sha(p)==v['sha256'],'runtime-source-drift:'+path)
  for module in (self.runtime.roles,self.runtime.admission,self.runtime.monitor):
   p=Path(module.__file__).resolve();need(str(p) in self.r['source_files'] and file_sha(p)==self.r['source_files'][str(p)]['sha256'],'imported-runtime-binding')
  need(file_sha(Path(self.r['guard_path']))==self.r['guard_sha256'],'guard-source-drift')
  need(file_sha(Path(self.r['python']['path']))==self.r['python']['sha256'],'Python-source-binding')
 def auth(self):
  need(not self.hold and time.monotonic()<self.deadline and time.time()<FUNCTIONAL_CUTOFF,'phase-HOLD-or-deadline')
  p=ROOT/'mac-heavy.lock';need(not p.is_symlink() and p.stat().st_size<=16384,'actual-token-path')
  with p.open('rb') as f:
   token=strict(f.read())
   try:fcntl.flock(f.fileno(),fcntl.LOCK_EX|fcntl.LOCK_NB)
   except BlockingIOError:pass
   else:raise RuntimeError('heavy-token-not-held')
  need(all(token[k]==self.token[k] for k in ('nonce','parent','keeper','source_pin','claims','fixed_cutoff')),'actual-heavy-nonce-changed')
  for expected in (token['parent'],token['keeper'],self.chain[0]):
   need(self.runtime.monitor.same_identity(expected,self.runtime.roles.identity(expected['pid'])),'native-owner-birth-drift')
  need(not (ROOT/'HOST-FENCE.json').exists(),'host-fence')
 def path(self,path,category=None):
  p=checked_path(path,ROOT)
  choices={'F08':Path(self.r['fixture_prefix']),'F12':Path(self.r['evidence_prefix'])}
  for k,base in choices.items():
   if p.is_relative_to(base) and (category is None or category==k):return p
  raise RuntimeError('unowned-phase-output')
 def grow(self,category,size,files):
  with self.lock:
   self.auth();need(category in self.used and all(type(n) is int and n>=0 for n in (size,files)),'growth-shape')
   u=self.used[category];cap=self.r['budget'][category]
   need(u['bytes']+size<=cap['bytes'] and u['files']+files<=cap['files'],'whole-phase-pool-cap')
   u['bytes']+=size;u['files']+=files
 def file(self,path,cap):
  p=self.path(path);need(type(cap) is int and cap>=0,'file-cap')
  with self.lock:
   if p in self.files:need(cap<=self.files[p],'no-expanding-file-slot');return
   need(not p.exists(),'retain-existing-prefix');k='F08' if p.is_relative_to(Path(self.r['fixture_prefix'])) else 'F12'
   self.grow(k,cap,1);self.files[p]=cap
 def directory(self,path):
  candidate=Path(path)
  if candidate.exists() and any(candidate in Path(self.r[k]).parents for k in ('fixture_prefix','evidence_prefix')):
   self.auth();need(not candidate.is_symlink(),'existing-parent-alias');return
  p=self.path(path)
  with self.lock:
   self.auth()
   if not p.exists() and p not in self.dirs:need(len(self.dirs)<self.r['directory_cap'],'directory-cap');self.dirs.add(p)
 def rewrite(self,path,cap):
  p=self.path(path,'F08');need(p.is_file() and not p.is_symlink() and type(cap) is int and cap>=p.stat().st_size,'reviewed-rewrite-cap')
  with self.lock:
   if p not in self.files:self.grow('F08',cap,1);self.files[p]=cap
   else:need(cap<=self.files[p],'rewrite-slot-expansion')
 def role_files(self,folder,out,err,rss,receipt):
  for name,cap in [('stdout.bin',out),('stderr.bin',err),('samples.jsonl.gz',rss),('result.json',receipt)]:self.file(folder/name,cap)
 def register_owner(self,owner):
  with self.lock:
   self.auth();need(_ACTIVE is self and owner.call_id not in self.owners and len(self.owners)<self.r['max_calls'],'registered-owner-cap')
   o=owner._options;self.role_files(ROOT/'evidence/roles'/owner.call_id,o['stdout_cap'],o['stderr_cap'],o['evidence_cap'],o['receipt_cap'])
   self.owners[owner.call_id]=owner
 def authorize_owned(self,role,argv,o):
  self.auth();f=self.facade;pending=f.pending.get(o['name'])
  need(pending is not None and pending['argv']==list(argv) and pending['env']==o.get('env') and pending['input_path']==str(o.get('input_path')),'exact-registered-call-before-input')
  ctx=pending['case'];need(o.get('case_context')=={'parent_run_id':ctx['run_name'],'case_id':ctx['case_id']},'owned-exact-case-context')
  if pending['mode']=='json-stdin':
   need(role==('reader' if pending['request']['action'] in READERS else 'checker'),'owned-independent-role')
   f.validate_control_env(ctx,o.get('env'),pending['request']['action'])
  else:need(o.get('argv_grant')==pending['grant_id'],'owned-exact-argv-grant')
 def bind_argv(self,grant_id,argv,binary_sha256,environment,payload,case_context,mode):
  self.auth();f=self.facade;g=f.contract['argv_grants'][grant_id]
  need(g['source_pin']==f.final_pin and g['case_id']==case_context['case_id'] and g['parent_run_id']==case_context['parent_run_id'],'argv-source-case')
  need(g['argv']==list(argv) and g['environment']==environment and g['binary_sha256']==binary_sha256 and g['stdin_sha256']==sha(payload) and g['mode']==mode,'actual-argv-environment-wire-binding')
  need(g['kind'] in ('workspace','ordinary','build','oracle'),'explicit-native-argv-kind')
  need(g['source_sha256'] in {v['sha256'] for v in f.contract['source_files'].values()},'argv-source-implementation')
  return {'producer_kind':g['kind'],'producer_source_pin':f.final_pin,'active_candidate_source_pin':f.final_pin,'binary_sha256':binary_sha256,'guard_nonce':self.token['nonce'],'argv':list(argv),'stdin_sha256':sha(payload),'argv_grant':grant_id,'case_context':case_context}
 def observe_identity(self,row):
  with self.lock:
   self.identities.add((row['pid'],tuple(row['start'])))
   if len(self.identities)>self.r['identity_budget']:self.hold=True;raise RuntimeError('phase-cumulative-identity-cap')
 def open(self,path,mode='r',*args,**kw):
  if not any(x in mode for x in 'wax+'):return self.old_open(path,mode,*args,**kw)
  p=Path(path)
  if p in [ROOT/n for n in ('mac-supervisors.json','mac-supervisors.lock','mac-heavy.lock')]:self.auth();return self.old_open(p,mode,*args,**kw)
  p=self.path(p);self.auth()
  if p not in self.files:self.facade.ext.authorize_write(p,mode)
  need(p in self.files,'declare-output-before-open')
  return Output(self,self.old_open(p,mode,*args,**kw),p)
 def install(self):
  global _ACTIVE
  need(_ACTIVE is None,'phase-already-owned');self.auth()
  self.old_open=builtins.open;self.old_path_open=Path.open;self.old_mkdir=Path.mkdir
  def mkdir(path,*args,**kw):self.directory(path);return self.old_mkdir(path,*args,**kw)
  builtins.open=self.open;Path.open=lambda path,*args,**kw:self.open(path,*args,**kw);Path.mkdir=mkdir;_ACTIVE=self
 def census(self):
  snap=self.runtime.admission.snapshot()
  for k in ('F08','F12'):
   v=snap['categories'][k];b=self.r['baseline'][k];cap=self.r['budget'][k]
   need(0<=v['logical_bytes']-b['bytes']<=cap['bytes'] and 0<=v['files']-b['files']<=cap['files'],'actual-phase-growth')
  return snap
 def close(self):
  global _ACTIVE
  need(all(v.closed for v in self.owners.values()),'native-owner-closure-unknown')
  for h in list(self.handles):h.close()
  need(not self.handles,'owned-handle-closure-unknown')
  self.verify_sources();snap=self.census()
  builtins.open=self.old_open;Path.open=self.old_path_open;Path.mkdir=self.old_mkdir;_ACTIVE=None
  return snap
class Call:
 def __init__(self,facade,case,root,argv,forecast,request,env,code,call_id):
  self.f=facade;self.case=case;self.root=Path(root);self.argv=argv;self.forecast=forecast;self.request=request;self.env=env;self.code=code;self.call_id=call_id
  self.owner=None;self.done=False;self.row=None;self.error=None;self.value=None;self.lock=threading.Lock();self.raw_start_ns=None;self.raw_end_ns=None;self.native_identity=None
 def started(self,owner):
  self.raw_start_ns=self.f.life_clock();proof=self.f.life.active_proof(self);self.native_identity=proof['identity_after']
 def wait(self,timeout=None):
  with self.lock:
   if self.done:
    if self.error is not None:raise self.error
    return self.value
   try:
    result=self.owner.wait(timeout);self.raw_end_ns=self.f.life_clock()
    need(result['output_complete'],'incomplete-native-output')
    raw=self.f.read_evidence(Path(result['outputs']['stdout']['path']),self.forecast['stdout_cap'])
    stderr=self.f.read_evidence(Path(result['outputs']['stderr']['path']),self.forecast['stderr_cap'])
    self.value=strict(raw) if raw else None
    self.row={'call_id':self.call_id,'case_id':self.case['case_id'],'input':self.request,'env':self.env,'exit':result['exit'],'stdout':raw.decode('utf-8'),'stderr':stderr.decode('utf-8'),'violations':[],'executable_sha256':result['binary_sha256'],'source_binding':result['producer_binding'],'native_result':str(Path(result['outputs']['stdout']['path']).parent/'result.json'),'raw_start_ns':self.raw_start_ns,'raw_end_ns':self.raw_end_ns}
    self.f.emit({'kind':'native-call','row':self.row})
    if result['exit']!=self.code:
     conflict=self.request['action']=='publish' and self.code==0 and result['exit']==2 and type(self.value) is dict and self.value.get('error')=='conflict:expected-publication'
     observed=result['exit']==86 and self.code==0 and (self.env or {}).get('EXP_EXIT') in self.forecast['exit_phases']
     if conflict or observed or result['exit'] in (-2,-15,-9) and self.case.get('interrupt_grant') is not None:raise AssertionError(self.row)
     raise RuntimeError('unexpected-native-exit:'+str(result['exit']))
   except BaseException as exc:
    self.error=exc
    self.f.emit({'kind':'native-call-exception','case_id':self.case['case_id'],'call_id':self.call_id,'error':repr(exc),'owner_closed':self.owner.closed})
   finally:
    if self.owner.closed:self.done=True
    else:self.f.lease.hold=True
   if self.error is not None:raise self.error
   return self.value
class Driver:
 def __init__(self,facade):
  self.f=facade;self.binary_sha256=facade.contract['binary_binding']['binary_sha256'];self.source_binding=facade.contract['binary_binding'];self.events=facade.events
 def call(self,root,action,timeout=120,code=0,env=None,*,exe_override=None,producer='candidate',on_started=None,**fields):
  finite(timeout)
  if exe_override is not None:
   target=Path(exe_override).resolve(strict=True)
   if target!=Path(self.f.contract['engine_argv'][0]).resolve():
    choices=[k for k,v in self.f.context(root)['producers'].items() if Path(v['argv'][0]).resolve()==target and v['grant'] is not None]
    need(len(choices)==1 and producer in ('candidate',choices[0]),'exact-parent-bound-historical-override');producer=choices[0]
  return self.f.start(root,action,timeout=min(timeout,115),code=code,env=env,producer=producer,on_started=on_started,**fields).wait()
class NativeFacade:
 reader_actions=READERS
 sha_file=staticmethod(file_sha)
 def __init__(self,release_bytes,release_sha256,runtime,decoder,case_map,lifecycle_type,extensions_type):
  need(sha(release_bytes)==release_sha256 and len(release_bytes)<=1024**2,'parent-runtime-release-byte-binding')
  r=strict(release_bytes)
  required={'kind','parent_released','source_pin','claims','fixed_cutoff','issued_monotonic','deadline_monotonic','stage_seconds','budget','baseline','prospective','identity_budget','directory_cap','max_calls','fixture_prefix','evidence_prefix','source_files','guard_path','guard_sha256','python','binary_binding','binding_path','binding_sha256','engine_argv','cases','checkpoints','work_source_map','workspace','ordinary','adapter_sha256','source_manifest_sha256','host_guard','event_cap','decoder_sha256','argv_grants','sdk'}
  need(set(r)==required and r['kind']=='native-final-facade-release-v36','closed-final-native-release')
  need(r['source_pin']=='0628322e4b48e0efad19cdc32238a622d4aaa7a8','exact-final-pin')
  self.contract=r;self.runtime=runtime;self.decoder=decoder;self.case_map=case_map;self.final_pin=r['source_pin'];self.events=[];self.calls=[];self.contexts={};self.lock=threading.RLock();self.serial=0;self.dispatches={};self.pending={}
  need(decoder.source_sha256==r['decoder_sha256'] and decoder.engine_code_imported is False,'independent-final-decoder-binding')
  case_map.bind_providers(r['cases'],r['sdk']);need(set(r['cases'])==set(case_map.plans),'all-158-concrete-providers')
  need(set(r['workspace'])=={str(i) for i in range(1,6)},'five-original-workspace-slots')
  unit_names=[u['name'] for slot in r['workspace'].values() for u in slot['units']]
  need(len(unit_names)==31 and len(set(unit_names))==31,'exhaustive-actual31-unit-identities')
  need(len(r['ordinary'])==17 and len({v['id'] for v in r['ordinary']})==17,'all-17-ordinary-routes')
  need(r['binary_binding']['source_pin']==self.final_pin and file_sha(Path(r['engine_argv'][0]).resolve())==r['binary_binding']['binary_sha256'],'final-binary-binding')
  self.lease=Lease(r,runtime);self.lease.facade=self;self.life=lifecycle_type(self);self.ext=extensions_type(self);self.driver=Driver(self)
  self.capabilities=frozenset(r['checkpoints'])|frozenset(v['capability'] for v in r['checkpoints'].values())|frozenset(v['capability'] for v in r['cases'].values())
  self.lease.install()
  folder=Path(r['evidence_prefix'])/'facade';self.lease.directory(folder);self.lease.file(folder/'events.jsonl.gz',r['event_cap']);folder.mkdir(parents=True,exist_ok=False)
  self.log=runtime.Events(folder/'events.jsonl.gz',r['event_cap']);self.ext.initialize()
 def life_clock(self):
  need(time.CLOCK_MONOTONIC==6,'native-raw-clock-ABI');return time.clock_gettime_ns(time.CLOCK_MONOTONIC)
 strict=staticmethod(strict)
 sha=staticmethod(sha)
 def clip(self,seconds):
  left=min(finite(seconds),self.lease.deadline-time.monotonic(),FUNCTIONAL_CUTOFF-time.time());need(left>0,'phase-deadline');return left
 def wall_time(self):return time.time()
 def monotonic(self):return time.monotonic()
 def sleep(self,seconds):
  need(type(seconds) in (int,float) and math.isfinite(seconds) and seconds>=0,'finite-nonnegative-sleep')
  if seconds:time.sleep(self.clip(seconds))
 def emit(self,row):
  with self.lock:self.log.emit(dict(row,source_pin=self.final_pin,guard_nonce=self.lease.token['nonce']));self.events.append(row)
 def context(self,root):
  key=str(Path(root));need(key in self.contexts,'unadmitted-native-root');return self.contexts[key]
 def sole_case(self):
  need(len({v['case_id'] for v in self.contexts.values()})==1,'explicit-single-case-callback-context');return next(iter(self.contexts.values()))
 def preflight(self,case_id,root,forecast,final_pin):
  self.lease.auth();need(final_pin==self.final_pin,'preflight-final-pin')
  plan=self.case_map[case_id];cfg=self.contract['cases'][case_id];p=self.lease.path(Path(root),'F08')
  need(str(p)==cfg['root'] and 'disposable' in p.parts and str(p) not in self.contexts,'exact-case-root')
  need(forecast==cfg['forecast'] and cfg['parent_released'] is True,'exact-case-forecast-GO')
  need(cfg['source_pin']==self.final_pin and cfg['handler']==plan['handler'],'case-source-handler')
  self.lease.verify_sources();self.lease.census()
  self.lease.grow('F08',forecast['F08_bytes'],forecast['F08_files'])
  with self.lock:self.contexts[str(p)]=cfg;self.dispatches[case_id]=0
  return {'go':True,'case_id':case_id,'root':str(p),'final_pin':self.final_pin,'reader_limit':268435456,'checker_limit':2147483648,'default_payloads':True,'heavy_token':self.lease.token['nonce'],'host_guard':self.contract['host_guard'],'binary_sha256':self.driver.binary_sha256,'binary_source_pin':self.final_pin,'source_manifest_sha256':self.contract['source_manifest_sha256'],'adapter_sha256':self.contract['adapter_sha256'],'guard_sha256':self.contract['guard_sha256']}
 def current_case(self,root):return self.context(root)
 def before_call(self,root,action,role,timeout):
  self.clip(timeout);self.lease.auth();ctx=self.context(root)
  need(role==('reader' if action in READERS else 'checker'),'independent-action-role')
  need(action in ctx['calls'] and ctx['calls'][action]['role']==role,'source-complete-action-forecast')
  self.lease.census();return ctx['calls'][action]
 def validate_control_env(self,ctx,env,action):
  if env is None:return
  need(type(env) is dict and all(type(k) is str and k.startswith('EXP_') and type(v) is str for k,v in env.items()),'closed-engine-environment')
  need('EXP_PAUSE' not in env,'actual-Rust-uses-EXP_BARRIER')
  phase=env.get('EXP_BARRIER');controls=[v for v in self.life.controls if v['case_id']==ctx['case_id'] and v['root']==ctx['root']]
  if phase is not None:
   need(phase in self.contract['checkpoints'] and phase in ctx['calls'][action]['barrier_phases'],'exact-case-source-barrier')
   spec=self.contract['checkpoints'][phase]
   need(spec['source_pin']==self.final_pin and spec['capability']=='barrier:'+phase and base64.b64decode(spec['ready_b64'],validate=True)==phase.encode('utf-8'),'actual-Rust-ready-bytes')
   pairs=[v for v in controls if env.get('EXP_READY')==v['ready'] and env.get('EXP_RELEASE')==v['release']]
   need(len(pairs)==1,'exact-same-case-ready-release-pair')
   if 'EXP_PACK_CANCEL' in env:need(env['EXP_PACK_CANCEL']==pairs[0]['cancel'],'exact-same-control-cancel-path')
  else:need('EXP_READY' not in env and 'EXP_RELEASE' not in env,'ready-release-without-source-barrier')
  if 'EXP_PACK_CANCEL' in env:need(any(env['EXP_PACK_CANCEL']==v['cancel'] for v in controls),'exact-case-cancel-path')
  for key in ('EXP_READY','EXP_RELEASE','EXP_PACK_CANCEL'):
   if key in env:
    p=checked_path(Path(env[key]),Path(ctx['control_root']))
    self.lease.path(p,'F08');need(p in self.lease.files,'control-slot-before-input')
  for key,field in (('EXP_EXIT','exit_phases'),('EXP_ERROR','error_phases')):
   if key in env:need(env[key] in ctx['calls'][action][field],'case-source-injection-phase')
  if 'EXP_NTH' in env:need(env['EXP_NTH'].isdigit() and int(env['EXP_NTH'])>0 and str(int(env['EXP_NTH']))==env['EXP_NTH'],'positive-canonical-checkpoint-occurrence')
 def start(self,root,action,*,timeout=115,code=0,env=None,producer='candidate',on_started=None,**fields):
  finite(timeout);ctx=self.context(root);role='reader' if action in READERS else 'checker';fc=self.before_call(root,action,role,timeout)
  self.validate_control_env(ctx,env,action)
  need(type(code) is int and code in (0,1,2,86,-2,-9,-15),'expected-exit')
  need(env is None or type(env) is dict and all(type(k) is str and k.startswith('EXP_') and type(v) is str for k,v in env.items()),'environment-overrides')
  need(not set(fields)&{'root','action','env','code','timeout','producer','on_started','exe_override'},'engine-payload-options')
  request={'root':str(root),'action':action,**fields};raw=(json.dumps(request,separators=(',',':'),allow_nan=False)+'\n').encode()
  need(len(raw)<=fc['input_cap'],'request-cap')
  with self.lock:
   need(self.dispatches[ctx['case_id']]<ctx['max_calls'],'case-call-bound')
   self.dispatches[ctx['case_id']]+=1;self.serial+=1;name=ctx['run_name']+'-'+str(self.serial)
  folder=Path(self.contract['evidence_prefix'])/'inputs';self.lease.directory(folder);self.lease.file(folder/(name+'.json'),len(raw));folder.mkdir(parents=True,exist_ok=True)
  with (folder/(name+'.json')).open('xb') as f:f.write(raw)
  produced={'argv':self.contract['engine_argv'],'grant':None} if producer=='candidate' else ctx['producers'][producer]
  call=Call(self,ctx,root,list(produced['argv']),fc,request,None if env is None else dict(env),code,name)
  allowed=[code]
  if action=='publish' and code==0:allowed.append(2)
  if (env or {}).get('EXP_EXIT') in fc['exit_phases'] and code==0:allowed.append(86)
  def started(owner):
   call.started(owner)
   if on_started is not None:on_started(owner)
  options={'name':name,'timeout':self.clip(timeout),'identity_budget':ctx['role_identity_budget'],'binding':self.contract['binding_path'],'binding_sha256':self.contract['binding_sha256'],'input_path':folder/(name+'.json'),'evidence_cap':fc['RSS_cap'],'control_only':False,'expected_exit':tuple(dict.fromkeys(allowed)),'env':env,'stdout_cap':fc['stdout_cap'],'stderr_cap':fc['stderr_cap'],'input_cap':fc['input_cap'],'receipt_cap':fc['receipt_cap'],'case_context':{'parent_run_id':ctx['run_name'],'case_id':ctx['case_id']},'historical_grant':produced['grant'],'interrupt_grant':ctx.get('interrupt_grant'),'on_started':started}
  self.pending[name]={'case':ctx,'argv':call.argv,'env':env,'input_path':str(folder/(name+'.json')),'request':request,'mode':'json-stdin'}
  owner=self.runtime.roles.OwnedCall(role,call.argv,phase=self.lease,**options);call.owner=owner
  with self.lock:self.calls.append(call);self.life.attach(call)
  try:owner.activate()
  except BaseException:
   if not owner._activated:owner.cancel_unstarted()
   if not owner.closed:self.lease.hold=True
   raise
  return call
 def read_evidence(self,path,limit):return self._read(checked_path(path,ROOT/'evidence'),limit)
 def read_fixture(self,path,limit):return self._read(checked_path(path,ROOT/'fixtures'),limit)
 def _read(self,path,limit):
  need(type(limit) is int and 0<=limit<=32*1024**2 and path.is_file() and path.stat().st_size<=limit,'bounded-regular-read')
  with path.open('rb') as f:data=f.read(limit+1)
  need(len(data)<=limit,'read-growth-cap');return data
 def read(self,root,relative,limit=4*1024**2):
  p=self.ext.relative(root,relative);return self._read(p,limit)
 def exists(self,root,relative):return self.ext.relative(root,relative).exists()
 def file_sha256(self,root,relative):return file_sha(self.ext.relative(root,relative))
 def write_fixture(self,path,data,*,fresh):
  p=self.lease.path(Path(path),'F08');need(type(data) is bytes,'exact-write-bytes')
  self.lease.file(p,len(data));self.lease.directory(p.parent);p.parent.mkdir(parents=True,exist_ok=True)
  with p.open('xb' if fresh else 'wb') as f:f.write(data)
 def inventory(self,root):return self.ext.inventory(root)
 def mapping(self,root,key):return self.ext.mapping(root,key)
 def backlog(self,root):return self.ext.backlog(root)
 def quiesce(self,root):return self.ext.quiesce(root)
 def removal_proof(self,root,before,phase):return self.ext.removal_proof(root,before,phase)
 def crash_source_binding(self,root,phase):return self.ext.crash_source_binding(root,phase)
 def extend(self,harness,root,spec):return self.ext.extend(harness,root,spec)
 def control(self,root,name):return self.life.control(root,name)
 def spawn(self,callback):return self.life.spawn(callback)
 def alive(self,handle):return self.life.alive(handle)
 def wait(self,handle,timeout):return self.life.wait(handle,timeout)
 def await_ready(self,control,handle,timeout):return self.life.await_ready(control,handle,timeout)
 def release(self,control):return self.life.release(control)
 def signal(self,control,kind):return self.life.signal(control,kind)
 def end_signal(self,control,kind):return self.life.end_signal(control,kind)
 def begin_phase(self,root,label):return self.life.begin_phase(root,label)
 def overlap(self,root,label,publishers,readers):return self.life.overlap(root,label,publishers,readers)
 def interrupt(self,handle,kind):return self.life.interrupt(handle,kind)
 def finish_interrupted(self,handle,kind):return self.life.finish_interrupted(handle,kind)
 def close_case(self,case_id,root):
  return self.ext.close_case(case_id,root)
 def finish(self):
  for root,ctx in list(self.contexts.items()):self.close_case(ctx['case_id'],root)
  need(all(not self.life.alive(h) for h in self.life.handles),'callback-closeout-HOLD')
  self.emit({'kind':'facade-finished','qualification':'outer-guard-final-closure-pending'})
  self.log.close();return self.lease.close()
if __name__=='__main__':raise SystemExit('SOURCE_ONLY_NO_NATIVE_RUNTIME_LAUNCHER')
