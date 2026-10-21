"""Mac host for the exact shared constructors. No import-time SDK or child launch.
Source projections keep shared function/class bodies intact while excluding
Linux process owners. Every projection and native substitution needs release.
A returned schedule result remains unqualified until independent final closure.
"""
import ast,builtins,fcntl,copy,hashlib,importlib.util,json,os,re,shutil,stat,sys,threading,time,types,uuid
from contextlib import contextmanager
from pathlib import Path
ROOT=Path('/private/tmp/file-native-online-macos-20261020')
PIN='0628322e4b48e0efad19cdc32238a622d4aaa7a8'
def need(ok,message):
 if not ok:raise RuntimeError(message)
def digest(path):
 h=hashlib.sha256()
 with Path(path).open('rb') as f:
  for block in iter(lambda:f.read(65536),b''):h.update(block)
 return h.hexdigest()
def encoded(v):return json.dumps(v,sort_keys=True,separators=(',',':'),allow_nan=False).encode()
def strict(raw,limit=4*1024**2):
 need(type(raw) in (bytes,str) and len(raw)<=limit,'bounded-JSON')
 def pairs(items):
  out={}
  for k,v in items:need(k not in out,'duplicate-JSON-key');out[k]=v
  return out
 def bad(v):raise ValueError('nonfinite:'+v)
 return json.loads(raw,object_pairs_hook=pairs,parse_constant=bad)
def owned(path,base):
 p=Path(path)
 need(p.is_absolute() and p.is_relative_to(Path(base)) and '..' not in p.parts,'owned-path')
 for q in [p,*p.parents]:need(not q.is_symlink(),'owned-path-alias')
 return p
def confined(root,path):return owned(path,Path(root))
class ClosedSDK:
 """No sys.path changes, Linux module imports, or hidden fallback imports.
 A projected node is the original AST node at its original source location.
 Parent manifests bind exact node source text and explicit injected globals.
 """
 def __init__(self,facade):
  self.f=facade;self.spec=facade.contract['sdk'];self.modules={};self.loading=set()
  need(self.spec['source_pin']==PIN and self.spec['parent_reviewed'] is True,'reviewed-SDK-source-pin')
  need(self.spec['projection_reviewed'] is True,'explicit-cross-platform-projection-review')
  total=0;paths=set()
  for name,row in self.spec['modules'].items():
   p=owned(row['path'],ROOT);need(p.is_file() and p.stat().st_nlink==1 and p.stat().st_size==row['bytes'] and digest(p)==row['sha256'],'exact-SDK-source:'+name)
   need(str(p) in facade.contract['source_files'] and facade.contract['source_files'][str(p)]=={'bytes':row['bytes'],'sha256':row['sha256']},'whole-release-SDK-source-binding')
   need(row['origin'] in ('sdk','native'),'declared-source-origin')
   if row['origin']=='sdk':paths.add(str(p));total+=row['bytes']
  need(total==self.spec['physical_bytes'] and len(paths)==self.spec['physical_files'],'actual-SDK-physical-closure')
  need(total<=self.spec['reserved_bytes'] and self.spec['reserved_bytes']>=577536,'no-old-SDK-undercount')
  self.builtin_import=builtins.__import__
 def import_(self,name,globals=None,locals=None,fromlist=(),level=0):
  need(level==0,'no-relative-SDK-imports')
  if name in self.spec['native_substitutions']:
   row=self.spec['native_substitutions'][name]
   need(row['reviewed'] is True and row['source_sha256'] in {v['sha256'] for v in self.f.contract['source_files'].values()},'reviewed-native-substitution')
   module=getattr(self.f.runtime,row['runtime_attribute'])
   need(digest(module.__file__)==row['source_sha256'],'actual-native-substitution-source')
   return module
  if name in self.spec['modules']:return self.load(name)
  base=name.split('.')[0]
  need(base in sys.stdlib_module_names and base not in {'subprocess','multiprocessing','pty','resource'},'no-unbound-SDK-runtime-import:'+name)
  return self.builtin_import(name,globals,locals,fromlist,level)
 def load(self,name):
  if name in self.modules:return self.modules[name]
  need(name in self.spec['modules'] and name not in self.loading,'closed-SDK-module-or-cycle:'+name)
  row=self.spec['modules'][name];p=Path(row['path']);raw=p.read_bytes()
  need(len(raw)==row['bytes'] and hashlib.sha256(raw).hexdigest()==row['sha256'],'SDK-before-load-drift')
  tree=ast.parse(raw,filename=str(p));projection=row.get('projection')
  if projection is not None:
   need(set(projection)=={'nodes','globals'} and projection['nodes'],'closed-source-projection')
   names={}
   for node in tree.body:
    if isinstance(node,(ast.FunctionDef,ast.AsyncFunctionDef,ast.ClassDef)):names[node.name]=node
   selected=[]
   for symbol,expected in projection['nodes'].items():
    need(symbol in names,'actual-projected-definition:'+name+'.'+symbol)
    node=names[symbol];text=ast.get_source_segment(raw.decode(),node).encode()
    need(hashlib.sha256(text).hexdigest()==expected,'exact-projected-node-bytes')
    selected.append(node)
   tree=ast.Module(body=selected,type_ignores=[]);ast.fix_missing_locations(tree)
  else:
   need(row['pure_import_reviewed'] is True and name not in ('native_guard','native_driver','native_driver_v3','native_adapter_v3','suite_main','suite_driver'),'no-Linux-owner-policy-execution')
  module=types.ModuleType(name);module.__file__=str(p);module.__source_sha256__=row['sha256']
  scope=module.__dict__;scope['__builtins__']=dict(vars(builtins),__import__=self.import_);self.loading.add(name)
  try:
   if projection is not None:
    scope.update({'fcntl':fcntl,'sys':sys,'copy':copy,'hashlib':hashlib,'json':json,'os':os,'re':re,'shutil':shutil,'threading':threading,'time':time,'uuid':uuid,'Path':Path,'contextmanager':contextmanager,'need':need,'encoded':encoded,'canonical':encoded,'digest':digest,'strict':strict,'owned':owned,'confined':confined})
    for key,value in projection['globals'].items():
     need(set(value)=={'module','symbol'},'closed-projected-global')
     target=self.load(value['module']);scope[key]=getattr(target,value['symbol'])
   exec(compile(tree,str(p),'exec'),scope)
   self.modules[name]=module
  finally:self.loading.remove(name)
  return module
 def bind_native(self,module):
  name=module.__name__;row=self.spec['modules'][name]
  need(digest(module.__file__)==row['sha256'],'bound-native-provider-source')
  module.__source_sha256__=row['sha256'];self.modules[name]=module

class Budget:
 """Monotone source-derived reservations; no failed-prefix or GC refunds."""
 def __init__(self,host):self.host=host;self.pending={};self.charged={};self.windows={};self.local=threading.local();self.lock=threading.RLock()
 def reserve_control(self,key,size,files=0,directories=0):
  need(type(key) is str and key and all(type(n) is int and n>=0 for n in (size,files,directories)),'finite-control-reservation')
  with self.lock:
   need(key not in self.pending and key not in self.charged,'unique-control-reservation')
   f=self.host.f;f.lease.auth();f.lease.census();f.lease.grow('F08',size,files)
   need(len(f.lease.dirs)+directories<=f.contract['directory_cap'],'prospective-directory-cap')
   self.pending[key]=(size,files,directories)
   self.windows[key]={'remaining':size,'reserved':0,'files':files,'open':{}}
   stack=getattr(self.local,'stack',[]);self.local.stack=[*stack,key]
 def reserve_evidence(self,key,size,files=0):
  with self.lock:
   need(key not in self.pending and key not in self.charged,'unique-evidence-reservation')
   self.host.f.lease.grow('F12',size,files);self.pending[key]=(size,files,0)
 def settle(self,key):
  with self.lock:
   need(key in self.pending,'known-reservation-settle')
   if key in self.windows:
    need(not self.windows[key]['open'],'all-window-writers-closed')
    stack=getattr(self.local,'stack',[]);need(stack and stack[-1]==key,'nested-writer-window-order');self.local.stack=stack[:-1]
   self.charged[key]=self.pending.pop(key)
   self.host.f.lease.census()

class Extensions:
 def __init__(self,facade):
  self.f=facade;self.sdk=None;self.h=None;self._current=None;self.case_roots={};self._aliases={};self._known_candidates={};self.poisoned_roots=set();self._command_rows=[];self._command_starts=[];self.drivers=[];self.workspace_proof=None
 def initialize(self):
  from native_suite_v36 import Commands
  f=self.f;self.sdk=ClosedSDK(f);self.m=self.sdk.spec['parameters']
  need(self.m['final_pin']==f.final_pin==PIN,'fixed-parameter-source-pin')
  need(set(self.m['case_specs'])==set(f.case_map.plans),'exact158-parameter-specs')
  need(set(self.m['case_versions'])==set(f.case_map.plans),'exact158-source-version-crosswalk')
  for case_id in f.case_map.plans:
   v=self.m['case_versions'][case_id];spec=self.m['case_specs'][case_id]
   need(v['status']=='reviewed-source-map' and v['final_pin']==PIN and v['version_mapping'] and v['required_checks'],'actual-source-version-map:'+case_id)
   need(spec['case_id']==case_id and spec['constructor_reviewed'] is True and spec['obligations'],'complete-constructor-obligations:'+case_id)
   provider=f.case_map.provider_bindings[case_id]
   need(v['source_sha256']==provider['source_sha256'],'same-actual-constructor-source:'+case_id)
  self.driver=f.driver;self.driver.owner=f.lease.token;self.drivers=[self.driver]
  self.output_root=owned(f.contract['evidence_prefix'],ROOT/'evidence')
  self.binding={'source_hashes':[v['sha256'] for v in f.contract['source_files'].values()],'final_pin':PIN,'checkpoints':f.contract['checkpoints'],'capabilities':f.capabilities,'adapter_sha256':f.contract['adapter_sha256'],'source_manifest_sha256':f.contract['source_manifest_sha256']}
  self.budget=Budget(self);self.commands=Commands(self)
  protocol=self.sdk.load('focused_protocol');adapter=self.sdk.load('suite_adapter')
  need(set(adapter.__dict__)>={'FinalHarness','Case'},'actual-shared-Harness-and-Case')
  self.Case=adapter.Case
  # These classes retain actual source bodies. No Linux Host constructor runs.
  class NativeHarness(adapter.FinalHarness):
   def initialize_legacy_inputs(h,root,records):
    need(root in h.active and not h.ledgers[root].accepted,'fresh-legacy-input-ledger')
    need(type(records) is list and records,'complete-source-bound-legacy-inputs')
    for row in records:
     need(set(row)=={'request','selected','request_receipt','request_sha256'},'actual-legacy-request-source-row')
     raw=f.read_evidence(Path(row['request_receipt']),4*1024**2)
     need(hashlib.sha256(raw).hexdigest()==row['request_sha256'] and strict(raw)==row['request'],'authentic-historical-request-bytes')
     request=row['request'];response=h.call(root,'resolve',request_id=request['request'])['result']
     need(response['outcome']=='accepted','actual-legacy-accepted-outcome')
     control=h.read(root,'publications/'+row['selected']['publication']+'.json',4*1024**2)
     need(hashlib.sha256(control).hexdigest()==row['selected']['digest'],'historical-selected-control-source')
     h.ledgers[root].accept(request,response,row['selected'])
  actual_oracle=self.sdk.load('suite_oracle')
  host=self
  class NativeOracle(actual_oracle.Oracle):
   def __call__(oracle,**fields):return host.independent_oracle(**fields)
  self.oracle=NativeOracle(self)
  self.h=NativeHarness(f.driver,self,self.oracle,self.emit,final_pin=PIN)
  native_suite=sys.modules['native_suite_v36'];self.sdk.bind_native(native_suite)
  providers={name:self.sdk.load(name) for name in ('suite_packing','suite_recovery','suite_ordinary','suite_extensions')}
  providers['native_suite_v36']=native_suite;f.case_map.bind_modules(providers)
  self.Inputs=protocol.Inputs
  return self
 def __getattr__(self,name):
  f=object.__getattribute__(self,'f')
  if name in {'wall_time','monotonic','sleep','before_call','control','spawn','alive','wait','await_ready','release','signal','end_signal','begin_phase','overlap','crash_source_binding','removal_proof'}:return getattr(f,name)
  if name=='capabilities':return f.capabilities
  raise AttributeError(name)
 @property
 def calls(self):
  return [dict(input=v.request,env=v.env,control_token=v.call_id,closed=v.owner.closed) for v in self.f.calls]
 def _native_driver(self,row):
  need(any(v.call_id==row['control_token'] for v in self.f.calls),'actual-owned-call-token')
  class Registry:
   def __init__(self,f):self.f=f
   def live(self,token):
    selected=[v for v in self.f.calls if v.call_id==token]
    need(len(selected)==1,'one-owned-native-call')
    call=selected[0]
    if call.owner.closed:return False
    self.f.life.active_proof(call);return True
  return Registry(self.f)
 def emit(self,row):return self.f.emit(row)
 def authorize_write(self,path,mode):
  p=self.f.lease.path(Path(path),'F08');stack=getattr(self.budget.local,'stack',[])
  need(stack and mode in ('wb','xb','w','x') and not p.exists(),'source-operation-window-before-fresh-write')
  cfg=self.f.contract['cases'][self._current]
  need(any(p.is_relative_to(owned(root,ROOT/'fixtures')) for root in cfg['write_roots']),'exact-reviewed-case-write-root')
  key=stack[-1];window=self.budget.windows[key]
  with self.budget.lock:
   need(window['files']>0 and p not in window['open'],'finite-window-files')
   cap=window['remaining']-window['reserved'];need(cap>=0,'finite-window-byte-cap')
   window['files']-=1;window['reserved']+=cap;window['open'][p]=cap
   self.f.lease.files[p]=cap
 def settle_write(self,path):
  if not hasattr(self,'budget'):return
  p=Path(path)
  with self.budget.lock:
   for window in self.budget.windows.values():
    if p not in window['open']:continue
    cap=window['open'].pop(p);actual=p.stat().st_size
    need(actual<=cap,'window-file-growth');window['remaining']-=actual;window['reserved']-=cap
    self.f.lease.files[p]=actual;return
 def relative(self,root,name):
  need(type(name) is str and name and '\\' not in name and ':' not in name and all(part not in ('','.','..') for part in name.split('/')),'owned-relative-name')
  return owned(Path(root)/name,Path(root))
 def read(self,root,name,limit=4*1024**2):return self.f._read(self.relative(root,name),limit)
 def exists(self,root,name):return self.relative(root,name).exists()
 def file_sha256(self,root,name):return digest(self.relative(root,name))
 def inventory(self,root):
  ctx=self.f.context(root);base=owned(root,ROOT/'fixtures');files={};dirs=[];size=0
  for directory,names,leaves in os.walk(base,followlinks=False):
   for name in names:
    p=Path(directory)/name;need(not p.is_symlink(),'inventory-directory-alias');dirs.append(p.relative_to(base).as_posix())
   for name in leaves:
    p=Path(directory)/name;s=p.lstat()
    need(stat.S_ISREG(s.st_mode) and s.st_nlink==1 and s.st_size<=ctx['forecast']['inventory_file_cap'],'regular-owned-corpus-file')
    key=p.relative_to(base).as_posix();files[key]={'bytes':s.st_size,'sha256':digest(p)};size+=s.st_size
    need(len(files)<=ctx['forecast']['inventory_files'] and size<=ctx['forecast']['inventory_bytes'],'complete-corpus-inventory-cap')
  return {'files':files,'directories':sorted(dirs),'bytes':size,'logical_bytes':size,'file_count':len(files),'directory_count':len(dirs),'durable_pins_included':True}
 def quiesce(self,root):
  ctx=self.f.context(root);held=[]
  for call in self.f.calls:
   if call.root!=Path(root) or call.owner.closed:continue
   phase=(call.env or {}).get('EXP_BARRIER');mapping=self.f.contract['checkpoints'].get(phase)
   need(call.request['action'] in self.f.reader_actions and mapping and mapping['dependent_io_paused'] is True,'live-mutator-prevents-quiescence')
   held.append({'call_id':call.call_id,'proof':self.f.life.active_proof(call),'phase':phase})
  return {'quiescent':True,'paused_readers_retained':held,'mutators_live':0}
 def independent_oracle(self,**fields):
  need(fields['final_pin']==PIN,'oracle-final-pin')
  self.quiesce(fields['root']);before=self.inventory(fields['root'])
  result=self.f.decoder.oracle(**fields)
  need(result['engine_code_imported'] is False and result['engine_audit_used'] is False and result['final_pin']==PIN,'independent-final-decoder')
  need(self.inventory(fields['root'])==before,'oracle-mutated-store')
  return result
 def mapping(self,root,key):
  self.quiesce(root);before=self.inventory(root);r=self.f.decoder.mapping(root,key)
  need(r['independent'] is True and r['source_pin']==PIN and r['receipt_sha256'],'source-bound-independent-mapping')
  need(self.inventory(root)==before,'mapping-mutated-store');return r
 def backlog(self,root):
  self.quiesce(root);before=self.inventory(root);r=self.f.decoder.backlog(root,inventory=before,source_pin=PIN)
  need(all(type(r[k]) is int and r[k]>=0 for k in ('unprotected','protected','excluded')) and r['receipt_sha256'],'full-independent-backlog')
  need(self.inventory(root)==before,'backlog-mutated-store');return r
 def quota_evidence(self,root):
  value=copy.deepcopy(self.m['quota_evidence'].get(str(root),{}))
  value.setdefault('owners',{});value.setdefault('stages',{});value.setdefault('qualified_schedule_sha256',[])
  return value
 def backup_restore(self,c,label):
  backup=self.sdk.load('operational_backup');self.quiesce(c.root);before=self.inventory(c.root)
  exclusions=set(self.m['backup_ephemeral_names'])
  need(exclusions and all('/' not in x and '*' not in x and not x.startswith('physical-pins') for x in exclusions),'finite-ephemeral-only-exclusions')
  archive=self.root(c.id,'archive-'+label);restored_root=self.root(c.id,label)
  need(not archive.exists() and not restored_root.exists(),'fresh-backup-prefixes')
  key='backup-'+uuid.uuid4().hex
  allowance=2*before['bytes']+sum(2*len(n.encode())+512 for n in before['files'])+65536
  self.budget.reserve_control(key,allowance,before['file_count']+8,before['directory_count']+4)
  previous=set(backup.EXCLUDED)
  try:
   backup.EXCLUDED=exclusions;archive.parent.mkdir(parents=True,exist_ok=True)
   backup.capture(c.root,archive,PIN,True);backup.restore(archive,restored_root)
  finally:backup.EXCLUDED=previous;self.budget.settle(key)
  for name,row in before['files'].items():
   if name.startswith('physical-pins/'):need((restored_root/name).is_file() and digest(restored_root/name)==row['sha256'],'durable-pins-retained-in-backup')
  self.register(c.id,restored_root);self.h.ledgers[restored_root]=self.Inputs()
  for row in self.h.ledgers[c.root].accepted.values():self.h.ledgers[restored_root].accept(row['request'],row['result'],row['selected'])
  for (root,path),request in list(self._known_candidates.items()):
   if root==str(c.root):self._known_candidates[(str(restored_root),path)]=copy.deepcopy(request)
  restored=self.Case(self,c.id,restored_root);restored.call('lifecycle',operation='setup')
  need(self.inventory(c.root)==before,'backup-mutated-original')
  return restored
 def profile(self,case_id):return copy.deepcopy(self.m['growth']['families'][self.m['case_families'][case_id]])
 def forecast(self,case_id):return dict(self.f.contract['cases'][case_id]['forecast'])
 def root(self,case_id,label='store'):
  need(case_id in self.f.case_map.plans and re.fullmatch(r'[A-Za-z0-9_-]+',label),'known-case-safe-root-label')
  row=self.f.contract['cases'][case_id];need(label in row['roots'],'finite-reviewed-root-template')
  return owned(row['roots'][label]['root'],ROOT/'fixtures')
 def preflight(self,case_id,root,forecast,final_pin):
  self._current=case_id;permit=self.f.preflight(case_id,root,forecast,final_pin);self.case_roots[str(root)]=case_id;return permit
 def register(self,case_id,root):
  cfg=self.f.contract['cases'][case_id];matches=[r for r in cfg['roots'].values() if r['root']==str(root)]
  need(case_id==self._current and len(matches)==1 and str(root) not in self.f.contexts,'exact-current-case-alias')
  alias=dict(cfg,**matches[0]);alias['case_id']=case_id
  need(alias['source_pin']==PIN and alias['parent_released'] is True,'released-alias-context')
  self.f.lease.grow('F08',alias['forecast']['F08_bytes'],alias['forecast']['F08_files'])
  self.f.contexts[str(root)]=alias;self.case_roots[str(root)]=case_id
  self._aliases.setdefault(case_id,[]).append(Path(root))
  self.h.active.add(Path(root));self.h.ledgers.setdefault(Path(root),self.Inputs())
  return alias['forecast']
 def mkdir(self,path):
  p=owned(path,ROOT/'fixtures');need(not p.exists(),'fresh-fixture-directory');self.f.lease.directory(p);p.mkdir(parents=True,exist_ok=False)
 def alias(self,case_id,label,source=None):
  root=self.root(case_id,label);need(not root.exists(),'fresh-alias')
  self.register(case_id,root)
  if source is not None:
   self.quiesce(source);facts=self.inventory(source)
   need(self.f.context(source)['source_pin']==PIN,'same-final-shallow-donor')
   key='clone-'+uuid.uuid4().hex;self.budget.reserve_control(key,facts['bytes'],facts['file_count'],facts['directory_count']+1)
   for name,row in facts['files'].items():self.f.lease.file(root/name,row['bytes'])
   root.parent.mkdir(parents=True,exist_ok=True);shutil.copytree(source,root,symlinks=False);self.budget.settle(key)
   for row in self.h.ledgers[source].accepted.values():self.h.ledgers[root].accept(row['request'],row['result'],row['selected'])
   self.h.call(root,'lifecycle',operation='setup')
  return root
 @contextmanager
 def case(self,case_id,label='store'):
  root=self.root(case_id,label)
  with self.h.case(case_id,root,self.forecast(case_id)):yield self.Case(self,case_id,root)
 def write(self,root,name,raw):
  need(type(raw) is bytes,'exact-fixture-bytes');path=self.relative(root,name);key='write-'+uuid.uuid4().hex
  self.budget.reserve_control(key,len(raw),0 if path.exists() else 1,1)
  if path.exists():self.f.lease.rewrite(path,max(len(raw),path.stat().st_size))
  else:self.f.lease.file(path,len(raw))
  path.parent.mkdir(parents=True,exist_ok=True)
  with path.open('wb' if path.exists() else 'xb') as out:out.write(raw);out.flush();os.fsync(out.fileno())
  self.budget.settle(key)
 def preserve_move(self,root,name,suffix='.withheld'):
  path=self.relative(root,name);target=path.with_name(path.name+suffix)
  need(suffix=='.withheld' and path.is_file() and not target.exists(),'fresh-preserved-prefix')
  self.f.lease.auth();path.rename(target)
 def prepared_candidates(self,root,requests):
  answer={}
  for p in (Path(root)/'attempts').glob('*/candidate.json'):
   raw=self.read(root,p.relative_to(root).as_posix());candidate=strict(raw)
   for request in requests:
    if candidate.get('request')==request:
     need(request['request'] not in answer,'unique-prepared-candidate');answer[request['request']]={'path':p.relative_to(root).as_posix(),'bytes':raw}
  need(set(answer)=={r['request'] for r in requests},'all-actual-prepared-candidates');return answer
 def await_ready(self,control,handle,timeout):
  result=self.f.await_ready(control,handle,timeout);call=control['call']
  handle['native']={'ready_ns':control['ready_raw_ns'],'input':copy.deepcopy(call.request),'env':copy.deepcopy(call.env),'identity':copy.deepcopy(call.native_identity),'control_token':call.call_id}
  return result
 def interrupt(self,handle,kind):
  operation={'death':'kill','cancel':'interrupt'}.get(kind,kind)
  need(len(handle['calls'])==1,'one-native-interruption-target')
  before=self.f.life.active_proof(handle['calls'][0]);ticket=self.f.interrupt(handle,operation)
  ack=ticket.wait(self.f.clip(2))
  proof={'kind':kind,'operation':operation,'identity_verified':True,'before':before,'ack':ack,'closed':False}
  handle['sdk_interruption']=proof;return proof
 def finish_interrupted(self,handle,kind):
  proof=handle.get('sdk_interruption');need(proof is not None and proof['kind']==kind,'same-sdk-interruption')
  result=self.f.finish_interrupted(handle,proof['operation'])
  expected={'interrupt':-2,'terminate':-15,'kill':-9}[proof['operation']]
  need(result['raw']['exit']==expected,'actual-native-signal-exit-required');proof['closed']=result['closed']
  proof['raw']=result['raw'];return proof
 def extend(self,harness,root,spec):
  need(harness is self.h and spec==self.m['case_specs'][spec['case_id']],'exact-source-reviewed-case-spec')
  return self.f.case_map.dispatch(self.Case(self,spec['case_id'],root),spec)
 def run_case(self,case_id):
  spec=self.m['case_specs'][case_id]
  with self.case(case_id) as case:return self.f.case_map.dispatch(case,spec)
 def close_case(self,case_id,root):
  self.commands.close();proof=self.f.life.close_case(case_id,root)
  aliases=self._aliases.pop(case_id,[])
  for p in [Path(root),*aliases]:
   self.f.contexts.pop(str(p),None);self.case_roots.pop(str(p),None)
   if p!=Path(root):self.h.active.discard(p)
  self._current=None;self.f.lease.census();return proof

class HistoricalFactory:
 """Native producer scope for the unchanged shared compatibility constructor."""
 def __init__(self,driver,host,grant):
  self.host=host;self.f=host.f;self.grant=grant
  need(driver is host.driver and type(grant) is dict and grant['parent_reviewed'] is True and grant['active_source_pin']==PIN,'reviewed-historical-producer-authority')
 @contextmanager
 def phase(self,root,label,binding):
  f=self.f;ctx=f.context(root)
  need(binding['parent_reviewed'] is True and binding['case_id']==ctx['case_id'] and binding['source_pin']==PIN,'exact-historical-phase-binding')
  key=binding['producer'];producer=ctx['producers'][key]
  need(producer['grant']==self.grant and binding['producer_source_pin']!=PIN and producer['source_pin']==binding['producer_source_pin'],'separate-historical-source-grant')
  need(binding['label']==label and binding['root']==str(root) and binding['actions'],'exact-old-producer-phase-scope')
  outer=self
  class Old:
   def call(self,actual_root,action,**fields):
    need(str(actual_root)==str(root) and action in binding['actions'],'source-reviewed-historical-action')
    f.lease.auth();result=f.driver.call(root,action,producer=key,**fields)
    need(all(v.owner.closed for v in f.calls if v.case['case_id']==ctx['case_id'] and v.root==Path(root)),'old-operation-must-close-before-transition')
    return result
  try:yield Old()
  finally:
   need(all(v.owner.closed for v in f.calls if v.case['case_id']==ctx['case_id'] and v.root==Path(root)),'historical-phase-closeout-HOLD')

if __name__=='__main__':raise SystemExit('SOURCE_ONLY_NO_NATIVE_LAUNCHER')
