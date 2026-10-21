import builtins,contextlib,fcntl,hashlib,json,math,os,threading,time
from pathlib import Path
import admission_v20 as admission
from guard_v20 import Events
import role_driver_v34 as roles
ROOT=admission.ROOT
READERS=frozenset(('resolve','point','inspect','lookup-bench'))
@contextlib.contextmanager
def reserve(category,size,files):
  if _PHASE is None:raise RuntimeError('phase-lease-required')
  if category=='F08':_PHASE.grow(category,size,files)
  yield
def owned_fixture(path):
  p=Path(path)
  if not p.is_absolute() or '..' in p.parts or not p.resolve().is_relative_to(ROOT/'fixtures'):raise ValueError('native-owned-fixture')
  for q in [p,*p.parents]:
    if q.is_relative_to(ROOT) and q.is_symlink():raise ValueError('native-fixture-symlink')
  return p
def write_input(path,data,maximum):
  if len(data)>maximum:raise RuntimeError('native-request-prewrite-cap')
  _PHASE.file(path,len(data))
  with Path(path).open('xb') as out:out.write(data)
class Call:
  def __init__(self,adapter,owner,request,expected,env,producer,forecast,reservations,input_row):
    self.call_id=owner.call_id;self._adapter=adapter;self._owner=owner
    self._request=request;self._expected=expected;self._env=None if env is None else dict(env);self._producer=producer
    self._forecast=forecast;self._reservations=reservations;self._input_row=input_row
    self._row=None;self._value=None;self._error=None;self._done=False;self._lock=threading.Lock()
  def wait_started(self,timeout=None):return self._owner.wait_started(timeout)
  @property
  def started_proof_ref(self):return self._owner.started_proof_ref
  @property
  def input_released(self):return self._owner.input_released
  def interrupt(self,operation):return self._owner.interrupt(operation)
  def wait(self,timeout=None):
    with self._lock:
      if not self._done:
        try:
          result=self._owner.wait(timeout)
          stdout=Path(result['outputs']['stdout']['path']).read_bytes();stderr=Path(result['outputs']['stderr']['path']).read_bytes()
          if not result['output_complete']:raise RuntimeError('native-output-incomplete')
          value=admission.strict_json(stdout) if stdout else None
          binding=result['producer_binding']
          row={'call_id':self.call_id,'input':self._request,'env':self._env,'exit':result['exit'],'stdout':stdout.decode('utf-8'),'stderr':stderr.decode('utf-8'),'seconds':result['seconds'],'guarded_seconds':result['guarded_seconds'],'violations':[],'rss_limit':result['limit_bytes'],'sampled_peak_rss':result['peak_RSS_bytes'],'producer_kind':binding['producer_kind'],'producer_source_pin':binding['producer_source_pin'],'active_candidate_source_pin':result['active_candidate_source_pin'],'exe_sha256':result['binary_sha256'],'executable_sha256':result['binary_sha256'],'source_binding':binding,'forecast_id':self._forecast['id'],'native_role_result':str(Path(result['outputs']['stdout']['path']).parent/'result.json')}
          self._row=row;self._value=value
          with self._adapter._lock:
            self._adapter.events.append(row)
            self._adapter._log.emit({'kind':'native-call','call_id':self.call_id,'input':self._input_row,'env':self._env,'exit':row['exit'],'seconds':row['seconds'],'producer_kind':row['producer_kind'],'producer_source_pin':row['producer_source_pin'],'active_candidate_source_pin':row['active_candidate_source_pin'],'binary_sha256':row['exe_sha256'],'outputs':result['outputs'],'role_result':row['native_role_result'],'forecast_id':row['forecast_id'],'violations':[]})
          if row['exit']!=self._expected:
            conflict=(self._request['action']=='publish' and self._expected==0 and row['exit']==2 and isinstance(value,dict) and value.get('error')=='conflict:expected-publication' and value.get('result') is None)
            observed_exit=(row['exit']==86 and self._expected==0 and isinstance(self._env,dict) and self._env.get('EXP_EXIT') in self._forecast.get('exit_phases',[]))
            if conflict or observed_exit:raise AssertionError(row)
            raise RuntimeError('unexpected-native-semantic-response:'+repr(row))
        except BaseException as exc:
          self._error=exc
          semantic=(isinstance(exc,AssertionError) and self._row is not None and exc.args==(self._row,))
          if not semantic:
            try:
              with self._adapter._lock:self._adapter._log.emit({'kind':'native-call-censored','call_id':self.call_id,'input':self._input_row,'env':self._env,'error':repr(exc),'owner_closed':self._owner.closed})
            except BaseException as log_error:self._error=RuntimeError('native-call-and-censor-log-failure:'+repr((exc,log_error)))
        finally:
          if self._owner.closed:
            self._done=True;self._reservations.close()
          else:
            self._adapter._ownership_HOLD=True
      if self._error is not None:raise self._error
      return self._value
  @property
  def row(self):
    if not self._done:raise RuntimeError('native-call-result-not-complete')
    return self._row
class NativeAdapter:
  def __init__(self,config,*,phase=None):
    self.phase=phase or _PHASE
    if self.phase is None or self.phase is not _PHASE:raise RuntimeError('phase-lease-before-adapter-required')
    self.phase.auth()
    fields={'run_name','control_only','engine_argv','candidate_binding','candidate_binding_sha256','forecasts','forecasts_sha256','source_pin','role_identity_budget','producers'}
    if not isinstance(config,dict) or set(config)!=fields:raise ValueError('native-config-closed-shape')
    name=config['run_name']
    if not isinstance(name,str) or not name.replace('-','').isalnum() or len(name)>64:raise ValueError('native-run-name')
    if type(config['control_only']) is not bool or type(config['role_identity_budget']) is not int or not 1<=config['role_identity_budget']<=4096:raise ValueError('native-control-identity-budget')
    self.config=config;self._lock=threading.RLock();self._serial=0;self.events=[];self._calls=[];self._ownership_HOLD=False
    self._forecasts=roles.producer.document(config['forecasts'],config['forecasts_sha256'],262144)
    if not isinstance(self._forecasts,dict) or set(self._forecasts)!={'source_pin','rows'} or self._forecasts['source_pin']!=config['source_pin'] or not isinstance(self._forecasts['rows'],list):raise ValueError('native-forecast-manifest')
    self._forecast_index={}
    for row in self._forecasts['rows']:
      fields={'id','case_id','action','F08_bytes','F08_files','stdout_cap','stderr_cap','input_cap','RSS_cap','receipt_cap','role'}
      if not isinstance(row,dict) or not fields<=set(row) or set(row)-fields-{'exit_phases'} or row['id'] in self._forecast_index:raise ValueError('native-forecast-closed-row')
      if any(type(row[k]) is not int or row[k]<0 for k in fields-{'id','case_id','action','role'}):raise ValueError('native-forecast-numeric-bound')
      if row['role'] not in ('reader','checker'):raise ValueError('native-forecast-explicit-role')
      if not isinstance(row.get('exit_phases',[]),list) or any(not isinstance(x,str) or not x for x in row.get('exit_phases',[])):raise ValueError('native-forecast-exit-phases')
      self._forecast_index[row['id']]=row
    if config['control_only']:
      if config['source_pin']!='CONTROL_ONLY' or Path(config['engine_argv'][0]).resolve()!=Path(roles.sys.executable).resolve():raise ValueError('native-control-Python-source')
      self.binary_sha256=roles.sha256(Path(config['engine_argv'][0]).resolve());self.source_binding={'source_pin':'CONTROL_ONLY','producer_kind':'control'}
    else:
      bound=roles.producer.document(config['candidate_binding'],config['candidate_binding_sha256'],65536)
      if bound['source_pin']!=config['source_pin'] or Path(config['engine_argv'][0]).resolve()!=Path(bound['binary_path']):raise ValueError('native-candidate-source-binary')
      self.binary_sha256=bound['binary_sha256'];self.source_binding=bound
    self._folder=ROOT/'evidence/capsules'/name
    self.phase.file(self._folder/'calls.jsonl.gz',262144)
    self._folder.mkdir(parents=True,exist_ok=False)
    self._log=Events(self._folder/'calls.jsonl.gz',262144)
  def start(self,root,action,*,case_id,forecast_id,code=0,env=None,on_started=None,timeout=115,producer='candidate',interrupt_grant=None,control_interrupts=(),**fields):
    if self._ownership_HOLD:raise RuntimeError('native-adapter-owned-closeout-HOLD')
    root=owned_fixture(root)
    if type(timeout) not in (int,float) or not math.isfinite(timeout) or not 0<timeout<=115:raise ValueError('native-call-timeout')
    reserved={'env','on_started','forecast_id','case_id','producer','code','root','action','exe_override'}
    if set(fields)&reserved:raise ValueError('native-request-options-in-payload')
    if env is not None and (not isinstance(env,dict) or any(not isinstance(k,str) or not k.startswith('EXP_') or not isinstance(v,str) for k,v in env.items())):raise ValueError('native-env-options')
    for key in ('EXP_READY','EXP_RELEASE','EXP_PACK_CANCEL'):
      if env is not None and key in env:owned_fixture(env[key])
    forecast=self._forecast_index.get(forecast_id)
    if forecast is None or (forecast['case_id'],forecast['action'])!=(case_id,action):raise RuntimeError('native-source-complete-case-forecast-required')
    if forecast['role']!=('reader' if action in READERS else 'checker'):raise ValueError('native-action-role-forecast')
    request={'root':str(root),'action':action,**fields};data=(json.dumps(request,separators=(',',':'),allow_nan=False)+'\n').encode()
    if len(data)>forecast['input_cap']:raise RuntimeError('native-request-cap')
    with self._lock:self._serial+=1;name=self.config['run_name']+'-'+str(self._serial)
    inputs=self._folder/(name+'-input.json');write_input(inputs,data,forecast['input_cap'])
    produced={'argv':list(self.config['engine_argv']),'historical_grant':None}
    if producer!='candidate':
      produced=self.config['producers'].get(producer)
      if not isinstance(produced,dict) or set(produced)!={'argv','historical_grant'} or produced['historical_grant'] is None:raise RuntimeError('native-parent-released-producer-required')
    allowed=[code]
    if action=='publish' and code==0:allowed=[0,2]
    if isinstance(env,dict) and 'EXP_EXIT' in env:
      if env['EXP_EXIT'] not in forecast.get('exit_phases',[]):raise RuntimeError('source-mapped-exit-phase-required')
      if code==0:allowed.append(86)
    stack=contextlib.ExitStack();owner=None
    try:
      stack.enter_context(reserve('F08',forecast['F08_bytes'],forecast['F08_files']))
      options={'name':name,'timeout':timeout,'identity_budget':self.config['role_identity_budget'],'binding':self.config['candidate_binding'],'binding_sha256':self.config['candidate_binding_sha256'],'input_path':inputs,'evidence_cap':forecast['RSS_cap'],'control_only':self.config['control_only'],'expected_exit':tuple(dict.fromkeys(allowed)),'env':env,'stdout_cap':forecast['stdout_cap'],'stderr_cap':forecast['stderr_cap'],'input_cap':forecast['input_cap'],'receipt_cap':forecast['receipt_cap'],'case_context':{'parent_run_id':self.config['run_name'],'case_id':case_id},'historical_grant':produced['historical_grant'],'interrupt_grant':interrupt_grant,'control_interrupts':control_interrupts,'on_started':on_started}
      owner=roles.OwnedCall(forecast['role'],produced['argv'],phase=self.phase,**options)
      call=Call(self,owner,request,code,env,produced,forecast,stack,{'path':str(inputs),'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()})
      with self._lock:self._calls.append(call)
      owner.activate()
      return call
    except BaseException:
      if owner is not None and not owner._activated:owner.cancel_unstarted()
      if owner is None or owner.closed:stack.close()
      else:self._ownership_HOLD=True
      raise
  def call(self,root,action,**options):return self.start(root,action,**options).wait()
  def close(self):
    errors=[]
    for call in list(self._calls):
      if call._done:continue
      try:call.wait()
      except BaseException as exc:errors.append(repr(exc))
    self._log.close()
    return errors
class Driver:
  def __init__(self,adapter):
    self.adapter=adapter;self.events=adapter.events
    self.binary_sha256=adapter.binary_sha256;self.source_binding=adapter.source_binding
  def call(self,root,action,timeout=120,code=0,env=None,*,exe_override=None,producer='candidate',on_started=None,case_id=None,forecast_id=None,**fields):
    if type(timeout) not in (int,float) or not math.isfinite(timeout) or timeout<=0:raise ValueError('driver-finite-positive-timeout')
    if exe_override is not None:
      target=Path(exe_override).resolve(strict=True);candidate=Path(self.adapter.config['engine_argv'][0]).resolve(strict=True)
      if target!=candidate:
        matches=[key for key,row in self.adapter.config['producers'].items() if isinstance(row,dict) and Path(row['argv'][0]).resolve(strict=True)==target and row.get('historical_grant') is not None]
        if len(matches)!=1:raise RuntimeError('unbound-historical-executable-override')
        if producer not in ('candidate',matches[0]):raise RuntimeError('historical-producer-override-conflict')
        producer=matches[0]
    if case_id is None or forecast_id is None:
      context=self.adapter.current_case(root) if hasattr(self.adapter,'current_case') else None
      if context is None:raise RuntimeError('shared-case-preflight-context-required')
      case_id=context['case_id'];forecast_id=context['forecast_for_call'](action,fields,env)
    return self.adapter.call(root,action,producer=producer,case_id=case_id,forecast_id=forecast_id,code=code,env=env,on_started=on_started,timeout=min(timeout,115),**fields)
def require_shared_protocol(release,expected_sha256):
  row=roles.producer.document(release,expected_sha256,262144)
  fields={'source_pin','protocol_core','shared_v5_decoder','case_families','workspace_units','ordinary_cases','focused_cases','version_map','parent_released'}
  if not isinstance(row,dict) or set(row)!=fields or row['parent_released'] is not True:raise RuntimeError('frozen-shared-protocol-release-required')
  for key in ('protocol_core','shared_v5_decoder'):
    item=row[key]
    if not isinstance(item,dict) or set(item)!={'path','bytes','sha256'}:raise ValueError('shared-protocol-member-shape')
    p=Path(item['path'])
    if p.is_symlink() or not p.is_file() or not p.resolve().is_relative_to(ROOT/'source') or p.stat().st_size!=item['bytes'] or roles.sha256(p)!=item['sha256']:raise RuntimeError('shared-protocol-member-exact-source')
  for family,count in [('packing',52),('strict',24),('recovery',24),('focus',8)]:
    cases=row['case_families'].get(family)
    if not isinstance(cases,list) or len(cases)!=count or len(set(cases))!=count:raise RuntimeError('shared-case-ID-coverage:'+family)
  if not row['workspace_units'] or not row['ordinary_cases'] or not row['focused_cases'] or not row['version_map']:raise RuntimeError('shared-full-unit-ordinary-focused-version-map-required')
  return row
if __name__=='__main__':raise SystemExit('SOURCE_ONLY_FROZEN_SHARED_PROTOCOL_AND_PARENT_CONTROL_REVIEW_REQUIRED')
_PHASE=None
class _Output:
  def __init__(self,lease,raw,path):self.p=lease;self.raw=raw;self.path=path;lease.handles.add(self)
  def __getattr__(self,key):return getattr(self.raw,key)
  def __enter__(self):return self
  def __exit__(self,*exc):self.close()
  def write(self,data):
    with self.p.lock:
      self.p.auth()
      size=len(data.encode(self.raw.encoding or 'utf-8')) if isinstance(data,str) else len(data)
      if self.raw.tell()+size>self.p.files[self.path]:raise RuntimeError('phase-file-cap')
      return self.raw.write(data)
  def writelines(self,lines):
    for line in lines:self.write(line)
  def truncate(self,size=None):
    with self.p.lock:
      self.p.auth();size=self.raw.tell() if size is None else size
      if type(size) is not int or not 0<=size<=self.p.files[self.path]:raise RuntimeError('phase-truncate-cap')
      return self.raw.truncate(size)
  def close(self):
    try:self.raw.close()
    finally:
      if self.raw.closed:self.p.handles.discard(self)
      else:self.p.hold=True
class PhaseLease:
  """One authenticated monotonic pool; no refund of censored artifacts."""
  def __init__(self,release):
    required={'kind','parent_released','phase','source_pin','guard_sha256','claims','baseline','budget','runtime_sources','python','fixed_cutoff','issued_monotonic','deadline_monotonic','max_calls','directory_cap','guard_prospective'}
    if set(release)!=required or release['kind']!='native-control-release-v34' or release['parent_released'] is not True:raise RuntimeError('phase-parent-release-required')
    self.r=release;self.root=ROOT;self.lock=threading.RLock();self.hold=False
    self.files={};self.dirs=set();self.handles=set();self.owners={};self.identities=set();self.pythreads=[];self.used={k:{'bytes':0,'files':0} for k in ('F08','F12')}
    self.token,self.chain=roles.ancestor_token()
    if release['source_pin']!='CONTROL_ONLY' or self.token['source_pin']!=release['source_pin'] or release['claims']!=self.token['claims'] or release['claims']!=admission.CLAIMS or release['guard_sha256']!=roles.sha256(Path(__file__).with_name('guard_v20.py')) or release['fixed_cutoff']!=self.token['fixed_cutoff']:raise RuntimeError('phase-guard-source-claim-binding')
    issued=release['issued_monotonic'];self.deadline=release['deadline_monotonic']
    if any(type(n) not in (int,float) or not math.isfinite(n) for n in (issued,self.deadline)) or not 0<self.deadline-issued<=45 or not issued<=time.monotonic()<self.deadline:raise RuntimeError('phase-fixed-local-clock')
    if release['budget']!={'F12':{'bytes':3145728,'files':96},'F08':{'bytes':1048576,'files':32}} or release['max_calls']!=10 or release['directory_cap']!=128:raise RuntimeError('phase-reviewed-bounds')
    for k in self.used:
      if set(release['baseline'][k])!={'bytes','files'} or any(type(n) is not int or n<0 for n in release['baseline'][k].values()) or release['guard_prospective'][k]!=release['budget'][k]:raise RuntimeError('phase-baseline-budget')
    gp=release['guard_prospective']
    if set(gp)!=set(admission.LIMITS) or gp['F01']!={'bytes':131072,'files':8} or any(gp[k]!={'bytes':0,'files':0} for k in ('F04','F13','C1')):raise RuntimeError('phase-other-growth')
    admission.admit(admission.snapshot(shared=False),gp,admission.limits())
    self.grow('F12',1253376,8) # admitted outer/keeper/RSS/stdout/finalization
    self.grow('F12',4096,1) # authenticated release document itself
    self.check()
  def auth(self):
    if self.hold or time.monotonic()>=self.deadline or time.time()>=self.token['fixed_cutoff']:raise RuntimeError('phase-HOLD-or-deadline')
    p=self.root/'mac-heavy.lock'
    if p.is_symlink() or p.stat().st_size>16384:raise RuntimeError('phase-token-path')
    with p.open('rb') as f:
      row=admission.strict_json(f.read())
      try:fcntl.flock(f.fileno(),fcntl.LOCK_EX|fcntl.LOCK_NB)
      except BlockingIOError:pass
      else:raise RuntimeError('phase-heavy-token-not-held')
    if any(row[k]!=self.token[k] for k in ('nonce','parent','keeper','source_pin','claims','fixed_cutoff')):raise RuntimeError('phase-actual-guard-nonce-changed')
    if any(not roles.monitor.same_identity(v,roles.identity(v['pid'])) for v in (row['parent'],row['keeper'],self.chain[0])):raise RuntimeError('phase-native-owner-birth')
  def category(self,path):
    p=Path(path).absolute()
    if '..' in p.parts or p.is_symlink():raise RuntimeError('phase-output-path')
    for q in p.parents:
      if q.is_relative_to(self.root) and q.is_symlink():raise RuntimeError('phase-output-parent')
    rel=p.relative_to(self.root)
    if rel.parts[0]=='fixtures' and len(rel.parts)>1 and rel.parts[1]=='native-controls-v34':return p,'F08'
    if rel.parts[0]=='evidence' and ('v34' in str(rel) or rel in (Path('evidence'),Path('evidence/control'),Path('evidence/roles'),Path('evidence/capsules'))):return p,'F12'
    raise RuntimeError('phase-unowned-output')
  def grow(self,category,size,files):
    with self.lock:
      self.auth()
      if category not in self.used or any(type(n) is not int or n<0 for n in (size,files)):raise ValueError('phase-amount')
      cap=self.r['budget'][category];u=self.used[category]
      if u['bytes']+size>cap['bytes'] or u['files']+files>cap['files']:raise RuntimeError('phase-pool-cap')
      u['bytes']+=size;u['files']+=files
  def file(self,path,cap):
    p,k=self.category(path)
    with self.lock:
      if p in self.files:
        if cap>self.files[p]:raise RuntimeError('phase-file-cap-cannot-expand')
        return
      if p.exists():raise RuntimeError('phase-prefix-reuse')
      self.grow(k,cap,1);self.files[p]=cap
  def directory(self,path):
    p,_=self.category(path)
    with self.lock:
      self.auth()
      if not p.exists() and p not in self.dirs:
        if len(self.dirs)>=self.r['directory_cap']:raise RuntimeError('phase-directory-cap')
        self.dirs.add(p)
  def role_files(self,folder,out,err,rss,receipt):
    for name,cap in [('stdout.bin',out),('stderr.bin',err),('samples.jsonl.gz',rss),('result.json',receipt)]:self.file(folder/name,cap)
  def register_owner(self,owner):
    with self.lock:
      self.auth()
      if _PHASE is not self or len(self.owners)>=10 or owner.call_id in self.owners:raise RuntimeError('phase-owned-call-cap')
      opt=owner._options
      self.role_files(self.root/'evidence/roles'/owner.call_id,opt.get('stdout_cap',65536),opt.get('stderr_cap',16384),opt.get('evidence_cap',131072),opt.get('receipt_cap',65536))
      self.owners[owner.call_id]=owner
  def register_python(self,thread):
    self.auth();self.pythreads.append(thread)
  def observe_identity(self,row):
    with self.lock:
      key=(row['pid'],*row['start'])
      self.identities.add(key)
      if len(self.identities)>64:self.hold=True;raise RuntimeError('phase-cumulative-identity-cap')
  def open(self,path,mode='r',*args,**kw):
    if not any(v in mode for v in 'wax+'):return self.old_open(path,mode,*args,**kw)
    p=Path(path)
    if p in [self.root/n for n in ('mac-supervisors.json','mac-supervisors.lock','mac-heavy.lock')]:self.auth();return self.old_open(path,mode,*args,**kw)
    p,k=self.category(path);self.auth()
    if p not in self.files:self.file(p,32768 if k=='F12' else 4096)
    return _Output(self,self.old_open(p,mode,*args,**kw),p)
  def check(self):
    for k,name in [('F12','evidence'),('F08','fixtures')]:
      v=admission.census(self.root/name);base=self.r['baseline'][k];cap=self.r['budget'][k]
      if any(v['logical_bytes' if u=='bytes' else 'files']<base[u] or v['logical_bytes' if u=='bytes' else 'files']>base[u]+cap[u] for u in ('bytes','files')):raise RuntimeError('phase-actual-growth-or-baseline-shrink')
  def __enter__(self):
    global _PHASE
    if _PHASE is not None:raise RuntimeError('phase-lease-already-active')
    self.old_open=builtins.open;self.old_path_open=Path.open;self.old_mkdir=Path.mkdir
    def mkdir(p,*args,**kw):self.directory(p);return self.old_mkdir(p,*args,**kw)
    builtins.open=self.open;Path.open=lambda p,*a,**k:self.open(p,*a,**k);Path.mkdir=mkdir;_PHASE=self
    return self
  def __exit__(self,*exc):
    global _PHASE
    for o in self.owners.values():
      if not o.closed:
        if not o._activated:o.cancel_unstarted()
        else:
          o._abort.set()
          try:o.wait(1)
          except BaseException:pass
    if any(not o.closed for o in self.owners.values()):self.hold=True;raise RuntimeError('phase-owned-closeout-HOLD')
    if any(t.is_alive() for t in self.pythreads):self.hold=True;raise RuntimeError('phase-Python-HOLD')
    for f in list(self.handles):f.close()
    if self.handles:self.hold=True;raise RuntimeError('phase-writer-HOLD')
    self.check()
    for name,digest in self.r['runtime_sources'].items():
      if roles.sha256(Path(__file__).with_name(name))!=digest:raise RuntimeError('phase-source-drift')
    admission.bounded_receipt(self.root/'evidence/control/native-phase-lease-v34.json',{'guard_nonce':self.token['nonce'],'claims':self.token['claims'],'source_pin':self.token['source_pin'],'sources':self.r['runtime_sources'],'promised':self.used,'files':{str(p):n for p,n in self.files.items()},'all_owners_closed':True,'failed_prefixes_retained':True,'control_error':repr(exc[1]) if exc[1] else None},byte_cap=32768)
    self.check()
    builtins.open=self.old_open;Path.open=self.old_path_open;Path.mkdir=self.old_mkdir;_PHASE=None
