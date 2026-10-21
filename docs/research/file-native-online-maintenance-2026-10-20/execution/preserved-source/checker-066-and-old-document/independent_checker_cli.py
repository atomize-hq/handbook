CONTRACT={"version":1,"cli":["--request","--output"],"engine_pin":"0628322e4b48e0efad19cdc32238a622d4aaa7a8","modes":["full","history_oracle","mapping","backlog","raw","entry","refuse","legacy_full"],"request_required":["version","mode","root","quiescent","limits","case_id","scope_id","case_scope"],"request_optional":["sources"],"limits_required":["max_files","max_directories","max_path_utf8","max_logical_bytes","proof_bytes","payload_bytes","error_bytes","total_output_bytes","v6_result_reserve_bytes","max_request_bytes","max_known_request_bytes","max_required_keys","normalized_reserve_bytes","progress_reserve_bytes"],"physical_only":{"modes":["raw","entry"],"route":"physical-orphan"},"entry_tuple":["descriptor","offset","encoded_size","raw_length","sha256"],"artifacts":{"before":".before.jsonl.gz","after":".after.jsonl.gz","result":".result.json","identities":".result.quota-identities.jsonl.gz","payload":".payload.json","error":".error.json","partial_suffix":".partial","progress":".progress.jsonl","backlog_paths":".backlog-paths.jsonl.gz"},"output_budget":"2*proof_bytes+payload_bytes+error_bytes+16384<=total_output_bytes","range_ref":{"path":"absolute path","offset":"u64","bytes":"u64","sha256":"lowerhex64"},"request_carrier_max_bytes":262144,"quiescence":{"full":True,"history_oracle":True,"refuse":True,"legacy_full":True,"raw":True,"entry":True,"mapping":"boolean","backlog":True},"startup_gate":{"carrier":"FILE_NATIVE_SCRIPT_GATE","keys":["ready","release","nonce","manifest_sha256","root"],"ready":["pid","nonce","manifest_sha256"],"release":["pid","nonce"],"root":"request.root","deadline":"FILE_NATIVE_SCRIPT_DEADLINE_PERF_NS; UTC1792581014"},"mode_fields":{"full":[["checkpoint","final_pin","known_requests_ref","request_provenance_ref","quota_evidence_ref","required_mapped_keys"],[]],"history_oracle":"full","legacy_full":[["checkpoint","final_pin","known_requests_ref","request_provenance_ref","required_mapped_keys"],[]],"refuse":[["checkpoint","final_pin","known_requests_ref","request_provenance_ref","quota_evidence_ref","require_before_index","expected_error_type","expected_message"],[]],"mapping":[["key"],[]],"backlog":[[],[]],"raw":[["key","physical_only"],["route","expected_sha256"]],"entry":[["key","physical_only"],["route"]]},"backlog_complete":True,"backlog_unit":"PATHS","proof_scope":"root:full/history_oracle/legacy_full/refuse/raw/entry;footprint:mapping/backlog","launch_flags":["-I","-S","-B","-u"]}

import argparse,base64,collections,contextlib,ctypes,errno,gzip,hashlib,importlib.machinery,importlib.util,json,os,re,sys,time,traceback
from pathlib import Path
INDEX_RESERVE=16384
CTX=None
DEADLINE=0
VERIFY_REFUSAL=False
HASH=re.compile(r'[0-9a-f]{64}')
def strict(raw):
 def pairs(rows):
  d={}
  for k,v in rows:
   if k in d:raise ValueError('duplicate-json-key')
   d[k]=v
  return d
 return json.loads(raw,object_pairs_hook=pairs,parse_constant=lambda x:(_ for _ in ()).throw(ValueError('nonfinite-json')))
def encoded(v):
 return (json.dumps(v,ensure_ascii=True,separators=(',',':'))+'\n').encode()
def closed(v,keys):
 if type(v) is not dict or set(v)!=set(keys):raise ValueError('closed-shape')
def integer(v):
 if type(v) is not int or not 0<=v<2**64:raise ValueError('u64')
 return v
def poll():
 if DEADLINE and (time.perf_counter_ns()>=DEADLINE or time.time()>=1792581014):raise Censor('absolute-deadline')
def safe_path(path,exists=True):
 p=Path(path)
 if not p.is_absolute() or '..' in p.parts:raise Censor('absolute-parent-free-path-required')
 for q in [*reversed(p.parents),p] if exists or p.exists() or p.is_symlink() else [*reversed(p.parents)]:
  st=q.lstat()
  if q.is_symlink() or getattr(st,'st_file_attributes',0)&1024:raise Censor('reparse-path')
 r=p.resolve(strict=exists)
 for q in [*reversed(r.parents),r] if r.exists() else [*reversed(r.parents)]:
  if getattr(q.lstat(),'st_file_attributes',0)&1024 or q.is_symlink():raise Censor('resolved-reparse')
 return r
def bounded(path,maximum):
 p=safe_path(path)
 with p.open('rb') as f:
  if os.fstat(f.fileno()).st_size>maximum:raise Censor('input-byte-censor')
  data=f.read(maximum+1)
 if len(data)>maximum:raise Censor('input-byte-censor')
 return data
def gate():
 global DEADLINE
 carrier=os.environ.get('FILE_NATIVE_SCRIPT_DEADLINE_PERF_NS','')
 if not re.fullmatch(r'[1-9][0-9]{0,18}',carrier) or not 0<int(carrier)<2**63:raise Censor('deadline-carrier')
 DEADLINE=int(carrier);poll()
 raw=os.environ.get('FILE_NATIVE_SCRIPT_GATE','')
 if len(raw.encode())>8192:raise Censor('gate-carrier')
 g=strict(raw);closed(g,('ready','release','nonce','manifest_sha256','root'))
 if not re.fullmatch(r'[0-9a-f]{32}',g['nonce']) or not HASH.fullmatch(g['manifest_sha256']):raise Censor('gate-identity')
 ready=safe_path(g['ready'],False);release=safe_path(g['release'],False)
 if ready.parent!=release.parent or ready==release or not Path(g['root']).is_absolute():raise Censor('gate-namespace')
 if ready.exists() or release.exists():raise Censor('gate-preexists')
 with ready.open('xb') as f:
  raw=encoded(dict(pid=os.getpid(),nonce=g['nonce'],manifest_sha256=g['manifest_sha256']))
  if f.write(raw)!=len(raw):raise OSError('gate-short-write')
  f.flush();os.fsync(f.fileno())
 while not release.exists():poll();time.sleep(.002)
 r=strict(bounded(release,8192));closed(r,('pid','nonce'))
 if type(r['pid']) is not int or r!={'pid':os.getpid(),'nonce':g['nonce']}:raise Censor('gate-release-identity')
 poll();return g
def digest(path):
 h=hashlib.sha256()
 with Path(path).open('rb') as f:
  for b in iter(lambda:f.read(1024*1024),b''):poll();h.update(b)
 return h.hexdigest()
def proof_ref(path):
 return dict(path=str(path),offset=0,bytes=path.stat().st_size,sha256=digest(path))
class Censor(RuntimeError):pass
class Sink:
 def __init__(self,ctx,file,part):
  self.ctx=ctx;self.file=file;self.part=part
 def write(self,data):
  poll()
  if type(data) is str:data=data.encode()
  if self.ctx.used[self.part]+len(data)>self.ctx.caps[self.part]:raise Censor('output-part-censor:'+self.part)
  if self.part in {'payload','normalized','result','progress'} and sum(self.ctx.used[k] for k in ['payload','normalized','result','progress'])+len(data)>self.ctx.limit['payload_bytes']:raise Censor('payload-group-censor')
  if sum(self.ctx.used.values())+len(data)>self.ctx.limit['total_output_bytes']:raise Censor('output-total-censor')
  n=self.file.write(data);self.ctx.used[self.part]+=n
  if n!=len(data):raise OSError('short-output-write')
  return n
 def flush(self):self.file.flush()
 def tell(self):return self.file.tell()
class Output:
 def __init__(self,index,limit):
  self.index=safe_path(index,False);self.limit=limit;self.used=collections.Counter();self.paths={}
  self.caps=dict(before=limit['proof_bytes'],after=limit['proof_bytes'],payload=limit['payload_bytes'],normalized=limit['normalized_reserve_bytes'],result=limit['payload_bytes']-limit['normalized_reserve_bytes']-limit['progress_reserve_bytes'],progress=limit['progress_reserve_bytes'],error=limit['error_bytes'],index=INDEX_RESERVE)
  for role,suffix in CONTRACT['artifacts'].items():
   if role=='partial_suffix':continue
   self.paths[role]=self.index.with_suffix(suffix)
  if any(p.exists() or p.with_name(p.name+'.partial').exists() for p in [self.index,*self.paths.values()]):raise Censor('output-prefix-preexists')
 def emit(self,role,value,part='payload',rows=False):
  path=self.paths[role];stage=path.with_name(path.name+'.partial')
  with stage.open('xb') as f:
   sink=Sink(self,f,part)
   if rows:
    with gzip.GzipFile(fileobj=sink,filename='',mode='wb',mtime=0,compresslevel=1) as z:
     for row in value:z.write(encoded(row))
   else:
    for chunk in json.JSONEncoder(ensure_ascii=True,indent=2).iterencode(value):sink.write(chunk.encode())
    sink.write(b'\n')
   sink.flush();os.fsync(f.fileno())
  if path.exists():raise Censor('final-artifact-race')
  stage.rename(path);return path
 def artifacts(self):
  answer=[]
  for role,path in self.paths.items():
   for p,complete in [(path,True),(path.with_name(path.name+'.partial'),False)]:
    if p.exists():
     valid=complete
     if complete:
      try:
       if p.name.endswith('.gz'):
        with gzip.open(p,'rb') as f:
         for line in f:strict(line)
       elif role!='progress':strict(p.read_bytes())
      except Exception:valid=False
     answer.append(dict(role=role,path=str(p),offset=0,bytes=p.stat().st_size,sha256=digest(p),complete=valid))
  return answer
class Progress:
 def __init__(self,ctx):self.ctx=ctx
 def __enter__(self):
  self.file=self.ctx.paths['progress'].with_name(self.ctx.paths['progress'].name+'.partial').open('xb');self.sink=Sink(self.ctx,self.file,'progress');return self
 def write(self,text):self.sink.write(text.encode());return len(text)
 def flush(self):self.sink.flush()
 def __exit__(self,kind,value,tb):
  try:self.flush();os.fsync(self.file.fileno())
  finally:self.file.close()
  if kind is None:self.file.name and Path(self.file.name).rename(self.ctx.paths['progress'])
def source_manifest(req):
 here=Path(__file__).resolve().parent
 prior=Path(os.environ.get('FILE_NATIVE_ORACLE_RESEARCH','C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook/docs/research' if os.name=='nt' else str(here.parents[2])))
 paths=[here/n for n in ['independent_checker_cli.py','independent_backlog.py','physical_decoder_v6.py','physical_decoder_v5.py','physical_decoder.py','paged_index.py','quota_v2_oracle.py','verify_operational_v2.py','outcome_oracle.py']]
 paths += [prior/n for n in ['file-native-packing-2026-10-20/independent.py','file-native-recovery-reclamation-2026-10-19/verify_operational.py','file-native-navigation-2026-10-19/oracle.py','file-native-incremental-engine-experiment-2026-10-18/verify_store.py']]
 if os.name=='nt':paths.append(Path('C:/Program Files/Git/mingw64/bin/libzstd.dll'))
 result={str(safe_path(p).resolve()):dict(bytes=p.stat().st_size,sha256=digest(p)) for p in paths}
 supplied=req.get('sources',{})
 if type(supplied) is not dict:raise Censor('source-bindings')
 for p,h in supplied.items():
  p=str(safe_path(p).resolve())
  if p not in result or not HASH.fullmatch(h) or result[p]['sha256']!=h:raise Censor('source-binding-changed')
 return result
class SourceLoader(importlib.machinery.SourceFileLoader):
 def get_code(self,name):
  raw=bounded(self.path,self.binding['bytes'])
  if len(raw)!=self.binding['bytes'] or hashlib.sha256(raw).hexdigest()!=self.binding['sha256']:raise Censor('loaded-source-binding')
  return compile(raw,self.path,'exec',dont_inherit=True)
class SourceOnly:
 def __init__(self,sources):self.paths={Path(p).stem:(p,v) for p,v in sources.items() if p.endswith('.py')}
 def find_spec(self,name,path=None,target=None):
  if name not in self.paths:return None
  p,v=self.paths[name];loader=SourceLoader(name,p);loader.binding=v
  return importlib.util.spec_from_file_location(name,p,loader=loader)
def read_ref(ref,maximum):
 if ref is None:return None
 closed(ref,CONTRACT['range_ref']);path=safe_path(ref['path'])
 offset=integer(ref['offset']);length=integer(ref['bytes'])
 if not 0<length<=maximum or not HASH.fullmatch(ref['sha256']):raise Censor('reference-bound')
 with path.open('rb') as f:
  before=os.fstat(f.fileno())
  if offset+length>before.st_size:raise Censor('reference-range')
  f.seek(offset);raw=f.read(length);after=os.fstat(f.fileno())
  if (before.st_dev,before.st_ino)!=(after.st_dev,after.st_ino) or after.st_size<offset+length:raise Censor('reference-race')
 if len(raw)!=length or not raw.endswith(b'\n') or hashlib.sha256(raw).hexdigest()!=ref['sha256']:raise Censor('reference-authentication-LF')
 return strict(raw)
def snapshot(ctx,root,role):
 limit=ctx.limit;counts=dict(files=0,directories=0,logical_bytes=0);state=hashlib.sha256()
 raw_bound=limit['max_files']*(6*limit['max_path_utf8']+192)+limit['max_directories']*(6*limit['max_path_utf8']+64)+4096
 def scan_error(error):raise Censor('snapshot-enumeration:'+str(error))
 def rows():
  for base,dirs,files in os.walk(root,followlinks=False,onerror=scan_error):
   poll();dirs.sort();files.sort();parent=Path(base)
   st=parent.lstat()
   if parent.is_symlink() or getattr(st,'st_file_attributes',0)&1024:raise Censor('directory-reparse')
   name=parent.relative_to(root).as_posix();counts['directories']+=1
   if len(name.encode())>limit['max_path_utf8'] or counts['directories']>limit['max_directories']:raise Censor('directory-geometry')
   row=dict(directory=name);line=encoded(row);state.update(line);yield row
   for filename in files:
    p=parent/filename;s=p.lstat();name=p.relative_to(root).as_posix()
    if p.is_symlink() or not p.is_file() or getattr(s,'st_file_attributes',0)&1024:raise Censor('file-reparse-or-type')
    counts['files']+=1;counts['logical_bytes']+=s.st_size
    if len(name.encode())>limit['max_path_utf8'] or counts['files']>limit['max_files'] or counts['logical_bytes']>limit['max_logical_bytes']:raise Censor('file-geometry')
    h=digest(p);after=p.stat()
    if (s.st_size,s.st_mtime_ns,s.st_ino,s.st_dev)!=(after.st_size,after.st_mtime_ns,after.st_ino,after.st_dev):raise Censor('snapshot-file-race')
    row=dict(path=name,bytes=s.st_size,sha256=h);state.update(encoded(row));yield row
 ctx.emit(role,rows(),role,True)
 return dict(complete=True,**counts,state_sha256=state.hexdigest(),source_raw_upper_bytes=raw_bound,enforced_physical_bytes=limit['proof_bytes'])
class NativePin:
 def __init__(self,path,exclusive):
  self.path=safe_path(path);self.initial=self.path.stat();self.exclusive=exclusive;self.locked=False
  self.fact=dict(path=str(self.path),native_probe=True,exclusive=exclusive,released=False)
 def __enter__(self):
  if os.name=='nt':
   from ctypes import wintypes as w
   k=self.k=ctypes.WinDLL('kernel32',use_last_error=True)
   class O(ctypes.Structure):
    _fields_=[('Internal',ctypes.c_void_p),('InternalHigh',ctypes.c_void_p),('Offset',w.DWORD),('OffsetHigh',w.DWORD),('hEvent',w.HANDLE)]
   self.o=O()
   k.CreateFileW.argtypes=[w.LPCWSTR,w.DWORD,w.DWORD,ctypes.c_void_p,w.DWORD,w.DWORD,w.HANDLE];k.CreateFileW.restype=w.HANDLE
   k.LockFileEx.argtypes=[w.HANDLE,w.DWORD,w.DWORD,w.DWORD,w.DWORD,ctypes.POINTER(O)];k.LockFileEx.restype=w.BOOL
   k.UnlockFileEx.argtypes=[w.HANDLE,w.DWORD,w.DWORD,w.DWORD,ctypes.POINTER(O)];k.UnlockFileEx.restype=w.BOOL
   k.CloseHandle.argtypes=[w.HANDLE];k.CloseHandle.restype=w.BOOL
   self.h=k.CreateFileW(str(self.path),0xC0000000,7,None,3,0x00200000,None)
   if self.h==ctypes.c_void_p(-1).value:raise ctypes.WinError(ctypes.get_last_error())
   try:
    class I(ctypes.Structure):
     _fields_=[('attributes',w.DWORD),('created',w.FILETIME),('accessed',w.FILETIME),('written',w.FILETIME),('volume',w.DWORD),('size_hi',w.DWORD),('size_lo',w.DWORD),('links',w.DWORD),('id_hi',w.DWORD),('id_lo',w.DWORD)]
    k.GetFileInformationByHandle.argtypes=[w.HANDLE,ctypes.POINTER(I)];k.GetFileInformationByHandle.restype=w.BOOL
    info=I()
    if not k.GetFileInformationByHandle(self.h,ctypes.byref(info)):raise ctypes.WinError(ctypes.get_last_error())
    index=info.id_hi*2**32+info.id_lo
    if info.attributes&1024 or index!=self.initial.st_ino:raise Censor('pin-handle-path-identity')
    self.fact['native_identity']=dict(volume=info.volume,file_index=index)
    self.locked=bool(k.LockFileEx(self.h,3 if self.exclusive else 1,0,1,0,ctypes.byref(self.o)))
    if not self.locked and ctypes.get_last_error()!=33:raise ctypes.WinError(ctypes.get_last_error())
   except BaseException:
    if not k.CloseHandle(self.h):raise ctypes.WinError(ctypes.get_last_error())
    raise
  else:
   import fcntl
   self.f=self.path.open('rb');self.fcntl=fcntl
   st=os.fstat(self.f.fileno());self.fact['native_identity']=dict(device=st.st_dev,inode=st.st_ino)
   if (st.st_dev,st.st_ino)!=(self.initial.st_dev,self.initial.st_ino):self.f.close();raise Censor('pin-handle-path-identity')
   try:fcntl.flock(self.f.fileno(),(fcntl.LOCK_EX if self.exclusive else fcntl.LOCK_SH)|fcntl.LOCK_NB);self.locked=True
   except OSError as e:
    if e.errno not in {errno.EAGAIN,errno.EACCES}:self.f.close();raise
  self.fact.update(lock_acquired=self.locked,pin_busy=not self.locked)
  if not self.exclusive and not self.locked:
   self.__exit__(None,None,None);raise Censor('shared-generation-pin-unavailable')
  return self
 def __exit__(self,kind,value,tb):
  if os.name=='nt':
   try:
    if self.locked and not self.k.UnlockFileEx(self.h,0,1,0,ctypes.byref(self.o)):raise ctypes.WinError(ctypes.get_last_error())
   finally:
    if not self.k.CloseHandle(self.h):raise ctypes.WinError(ctypes.get_last_error())
  else:
   try:
    if self.locked:self.fcntl.flock(self.f.fileno(),self.fcntl.LOCK_UN)
   finally:self.f.close()
  self.fact['released']=True
def pin_probe(path):
 with NativePin(path,True) as p:return p.fact
def physical_decoder(v5,root):
 # Fixed STORE3 constructor port: no selected_work, accepted chain or cohort pass.
 c=v5.prior.legacy.strict(v5.prior.legacy.bounded(root/'STORE.json',4096));v5.config3(c)
 d=v5.Decoder.__new__(v5.Decoder);d.root=root;d.config=c;d.entries={};d.descriptors=[];d.zstd=None;d.required={'STORE.json'};d.candidate_context=False;d.authority=d
 d.pin(c['physical'],c['catalog']);catalog=d.catalog(c['catalog']);d.descriptors=catalog['packs']
 for descriptor in d.descriptors:d.load_index(descriptor)
 d.physical_metadata=dict(physical_only=True,cohort_pass=False)
 return d
def metadata(d,v5):
 work=None;pins=[]
 if d.config['version']==3:
  b=d.config['physical'];work=d.read_metadata('physical-plans/'+b['plan']+'.json',v5.WORK_LIMIT,b['plan']);v5.work2(work)
  for g in [work['generation'],*work['older']]:
   pins.append(pin_probe(d.root/('physical-pins/'+g['id']+'.lock')))
 return dict(config=d.config,physical_metadata=d.physical_metadata,work=work,pin_probes=pins)
def live_query(req,root,v5):
 anchor=bounded(root/'STORE.json',4096);c=strict(anchor);v5.config3(c)
 pin=root/('physical-pins/'+c['physical']['id']+'.lock')
 with NativePin(pin,False) as held:
  if bounded(root/'STORE.json',4096)!=anchor:raise Censor('live-selected-anchor-race')
  d=v5.Decoder.__new__(v5.Decoder);d.root=root;d.config=c;d.entries={};d.descriptors=[];d.zstd=None;d.required={'STORE.json'};d.candidate_context=False;d.authority=d
  d.pin(c['physical'],c['catalog']);cat=d.catalog(c['catalog']);d.descriptors=cat['packs']
  work=d.read_metadata('physical-plans/'+c['physical']['plan']+'.json',v5.WORK_LIMIT,c['physical']['plan']);v5.work2(work)
  g=work['generation']
  if work['store_id']!=c['store_id'] or (g['id'],g['ordinal'],g['catalog'],g['packs'])!=(c['physical']['id'],c['physical']['ordinal'],c['catalog'],cat['packs']):raise AssertionError('live-work-anchor-binding')
  probes=[]
  for old in work['older']:
   d.pin(old,old['catalog']);probes.append(dict(generation=old,probe=pin_probe(root/('physical-pins/'+old['id']+'.lock'))))
  answer=dict(status='passed',independent=True,physical_only=False,cohort_pass=False,qualification='single selected immutable metadata/index footprint; no full state or cohort audit',config=c,work=work,pin_probes=probes,selected_pin_hold=held.fact)
  if req['mode']=='mapping':
   key=req['key']
   if type(key) is not str or v5.prior.legacy.role(key) is None:raise ValueError('typed-key')
   for descriptor in d.descriptors:d.load_index(descriptor)
   entry=d.entries.get(key)
   answer.update(key=key,mapped=entry is not None,entry=list(entry) if entry else None,selected_generation=c['physical'])
  if bounded(root/'STORE.json',4096)!=anchor:raise Censor('live-selected-anchor-race')
 return answer
def run_action(req,ctx,root):
 import physical_decoder_v5 as v5
 global VERIFY_REFUSAL
 mode=req['mode'];physical=req.get('physical_only',False)
 if mode=='backlog':
  from independent_backlog import verify
  d=v5.Decoder(root,quiescent=True)
  result,rows=verify(root,ctx.limit,metadata(d,v5)['pin_probes'])
  path=ctx.emit('backlog_paths',rows,'result',True)
  result['path_proof_ref']=proof_ref(path)
  return result
 if not req['quiescent']:return live_query(req,root,v5)
 if physical and (mode not in {'raw','entry'} or req.get('route')!='physical-orphan'):raise ValueError('physical-only-route')
 if mode in {'full','history_oracle','refuse','legacy_full'}:
  known=read_ref(req.get('known_requests_ref'),ctx.limit['max_known_request_bytes'])
  provenance=read_ref(req.get('request_provenance_ref'),ctx.limit['max_known_request_bytes'])
  quota=read_ref(req.get('quota_evidence_ref'),ctx.limit['max_known_request_bytes'])
  if type(known) is not dict or any(type(v) is not dict or k!=v.get('request') for k,v in known.items()):raise ValueError('known-request-map')
  path=ctx.paths['result']
  with Progress(ctx) as progress,contextlib.redirect_stdout(progress):
   if mode=='legacy_full':
    result=v5.verify(root,os.devnull,quiescent=True,known_requests=known,request_provenance=provenance)
    ctx.emit('result',result,'result')
   else:
    import physical_decoder_v6 as v6
    if req.get('final_pin')!=CONTRACT['engine_pin'] or v6.FINAL_SOURCE_PIN!=CONTRACT['engine_pin']:raise Censor('final-pin-binding')
    limits=dict(version=1,result_path=str(path),identity_path=str(path.with_suffix('.quota-identities.jsonl.gz')),max_bytes=ctx.caps['result'],result_reserve_bytes=ctx.limit['v6_result_reserve_bytes'])
    try:
     VERIFY_REFUSAL=True
     result=v6.verify(root,path,quiescent=True,known_requests=known,request_provenance=provenance,quota_evidence=quota,artifact_limits=limits)
     VERIFY_REFUSAL=False
    except v6.ArtifactCensor:
     VERIFY_REFUSAL=False;raise
    finally:
     ctx.used['result']+=sum(q.stat().st_size for p in [path,path.with_suffix('.quota-identities.jsonl.gz')] for q in [p,p.with_name(p.name+'.partial')] if q.exists())
  d=v5.Decoder(root,quiescent=True)
  required=req.get('required_mapped_keys',[])
  if type(required) is not list or len(required)>ctx.limit['max_required_keys'] or any(type(k) is not str for k in required):raise Censor('required-key-bound')
  found=[k for k in required if k in d.entries]
  if found!=required:raise AssertionError('required-mapped-key-missing')
  reverse=result['mapped_cohort_closure']
  checks=dict(semantic=result['semantic'],navigation=result['navigation'],request_intents=result['request_intents'],cohort=reverse,quota=result.get('quota'))
  payload=dict(status='passed',final_pin=req.get('final_pin'),checks=checks,actual_H=result['semantic']['publications'],mapped_keys=found,reverse_selected_cohort=reverse,history_oracle=result,fullproof_ref=proof_ref(path))
  return payload
 d=physical_decoder(v5,root) if physical else v5.Decoder(root,quiescent=True)
 if not physical:d.verify_packs()
 if mode in {'raw','entry','mapping'}:
  key=req['key']
  if type(key) is not str or v5.prior.legacy.role(key) is None:raise ValueError('typed-key')
  if mode=='entry':
   value=d.entries.get(key);variants=getattr(d,'variants',{}).get(key,[] if value is None else [value])
   return dict(status='passed',entry=list(value) if value else None,variants=[list(x) for x in variants],physical_only=physical,cohort_pass=not physical)
  if mode=='raw':
   raw=d.raw(key);expected=req['expected_sha256'] if 'expected_sha256' in req else (d.entries[key][4] if key in d.entries else None)
   h=hashlib.sha256(raw).hexdigest()
   if expected is None or h!=expected:raise AssertionError('raw-authentication-required')
   if 4*((len(raw)+2)//3)>ctx.limit['payload_bytes']:raise Censor('raw-output-geometry')
   return dict(status='passed',raw_base64=base64.b64encode(raw).decode(),bytes=len(raw),sha256=h,physical_only=physical,cohort_pass=not physical)
  entry=d.entries.get(key)
  return dict(status='passed',key=key,mapped=entry is not None,entry=list(entry) if entry else None,metadata=metadata(d,v5),reverse_selected_cohort=d.mapped_cohort_proof)
 raise ValueError('mode')
def main():
 global CTX
 g=gate()
 parser=argparse.ArgumentParser();parser.add_argument('--request',required=True);parser.add_argument('--output',required=True);args=parser.parse_args()
 if not sys.dont_write_bytecode or sys.flags.optimize or not(sys.flags.isolated and sys.flags.no_site) or os.environ.get('FILE_NATIVE_STAGE_JOB')!='1':raise Censor('owned-guard-required')
 request_path=safe_path(args.request);raw=bounded(request_path,CONTRACT['request_carrier_max_bytes']);req=strict(raw)
 required=set(CONTRACT['request_required']);allowed=required|set(CONTRACT['request_optional'])|set().union(*(set(x[0])|set(x[1]) for x in CONTRACT['mode_fields'].values() if type(x) is list))
 if type(req) is not dict or not required<=set(req)<=allowed or type(req['version']) is not int or req['version']!=1 or type(req['quiescent']) is not bool or req['mode'] not in CONTRACT['modes']:raise ValueError('request')
 spec=CONTRACT['mode_fields'][req['mode']]
 if type(spec) is str:spec=CONTRACT['mode_fields'][spec]
 needed=required|set(spec[0]);allowed=needed|set(spec[1])|{'sources'}
 if not needed<=set(req)<=allowed:raise ValueError('mode-request-fields')
 for k in ['physical_only','require_before_index']:
  if k in req and type(req[k]) is not bool:raise ValueError('mode-bool')
 if 'expected_sha256' in req and not(type(req['expected_sha256']) is str and HASH.fullmatch(req['expected_sha256'])):raise ValueError('expected-sha256')
 for k in ['case_id','scope_id']:
  if type(req[k]) is not str or not 1<=len(req[k].encode())<=256 or '\x00' in req[k]:raise ValueError('case-scope-id')
 if 'final_pin' in req and req['final_pin']!=CONTRACT['engine_pin']:raise Censor('final-pin-binding')
 root=Path(req['root'])
 if not root.is_absolute() or req['root']!=g['root'] or not safe_path(root).is_dir():raise ValueError('root')
 full_proof=req['mode'] not in {'mapping','backlog'}
 root=root.resolve();out=safe_path(args.output,False)
 if out.is_relative_to(root):raise Censor('output-inside-store')
 if not re.fullmatch(r'[0-9a-f]{32}',req['case_scope']):raise ValueError('case-scope')
 limit=req['limits'];closed(limit,CONTRACT['limits_required'])
 for value in limit.values():integer(value)
 if limit['normalized_reserve_bytes']+limit['progress_reserve_bytes']+limit['v6_result_reserve_bytes']>limit['payload_bytes']:raise Censor('payload-part-reservation')
 if len(raw)>limit['max_request_bytes']:raise Censor('request-byte-censor')
 if 2*limit['proof_bytes']+limit['payload_bytes']+limit['error_bytes']+INDEX_RESERVE>limit['total_output_bytes']:raise Censor('output-reservation')
 if not (0<limit['v6_result_reserve_bytes']<limit['payload_bytes'] and 0<limit['normalized_reserve_bytes']<limit['payload_bytes'] and 0<limit['progress_reserve_bytes']<limit['payload_bytes']):raise Censor('payload-reserves')
 if not req['quiescent'] and req['mode']!='mapping':raise Censor('mode-requires-quiescence')
 CTX=Output(out,limit);sources=source_manifest(req);audit_phase='control';footprint={};hashing=False
 audit=dict(index_opens=0,first_index_open=None,index_open_sha256=None,spawn_events=0,dlopen=[]);ah=hashlib.sha256()
 def hook(event,values):
  nonlocal hashing
  if event in {'subprocess.Popen','os.system','os.exec','os.posix_spawn'}:audit['spawn_events']+=1;raise Censor('engine-process-audit-forbidden')
  if event in {'os.remove','os.rmdir','os.mkdir','os.rename','os.link','os.symlink','os.chmod','os.truncate','os.utime'}:
   if any(isinstance(x,(str,bytes)) and safe_path(Path(os.fsdecode(x)).absolute(),False).is_relative_to(root) for x in values):raise Censor('store-mutation-forbidden')
  if event=='ctypes.dlopen':
   name=str(values[0])
   if name.lower() not in {'kernel32','kernel32.dll','ntdll','ntdll.dll'}:
    p=str(Path(name).resolve())
    if p not in sources or not p.lower().endswith('libzstd.dll'):raise Censor('unbound-native-library')
   audit['dlopen'].append(name)
  if event=='open' and isinstance(values[0],(str,bytes)):
   if os.fsdecode(values[0])==os.devnull and req['mode']=='legacy_full':return
   p=safe_path(Path(os.fsdecode(values[0])).absolute(),False);mode=values[1] or '';flags=values[2] or 0
   if p.is_relative_to(root) and (any(x in mode for x in 'wax+') or flags&(os.O_WRONLY|os.O_RDWR|os.O_CREAT|os.O_TRUNC)):raise Censor('store-write-forbidden')
   if p.is_relative_to(root) and audit_phase=='decoder' and not hashing and not full_proof:
    rel=p.relative_to(root).as_posix()
    if rel not in footprint:
     if len(footprint)>=limit['max_files']:raise Censor('footprint-file-censor')
     safe_path(p);st=p.stat()
     if sum(v['bytes'] for v in footprint.values())+st.st_size>limit['max_logical_bytes']:raise Censor('footprint-byte-censor')
     hashing=True
     try:h=digest(p)
     finally:hashing=False
     footprint[rel]=dict(bytes=st.st_size,device=st.st_dev,inode=st.st_ino,sha256=h)
   if p.is_relative_to(root) and p.suffix=='.idx' and audit_phase=='decoder' and not hashing:
    rel=p.relative_to(root).as_posix();audit['index_opens']+=1;ah.update(encoded(dict(path=rel,mode=mode,flags=flags)))
    if audit['first_index_open'] is None:audit['first_index_open']=rel
 sys.addaudithook(hook);sys.meta_path.insert(0,SourceOnly(sources))
 before=None;after=None;payload=None;error=None;status='technical_censor';stage='before-proof'
 try:
  before=snapshot(CTX,root,'before') if full_proof else None;stage='decoder';audit_phase='decoder'
  payload=run_action(req,CTX,root)
  if req['mode']=='refuse':raise Censor('expected-refusal-did-not-occur')
  status='passed'
 except Exception as e:
  error=dict(type=type(e).__name__,message=str(e),traceback=traceback.format_exc(),stage=stage)
  if VERIFY_REFUSAL and stage=='decoder' and req['mode']=='refuse' and type(e).__name__==req.get('expected_error_type') and str(e)==req.get('expected_message','') and type(e).__name__ in {'AssertionError','ValueError'} and not isinstance(e,(Censor,OSError)):
   status='expected-independent-refusal';payload=dict(status=status,reason=error,complete_expected_refusal=True,opened_paths=dict(count=audit['index_opens'],first=audit['first_index_open']))
  try:CTX.emit('error',error,'error')
  except Exception:status='technical_censor'
 finally:
  audit_phase='proof'
  try:after=snapshot(CTX,root,'after') if full_proof else None
  except Exception as e:
   status='technical_censor'
   if error is None:error=dict(type=type(e).__name__,message=str(e),traceback=traceback.format_exc(),stage='after-proof')
 unchanged=before is not None and after is not None and before['complete'] and after['complete'] and before['state_sha256']==after['state_sha256']
 if not full_proof:
  try:
   if not footprint:raise Censor('empty-live-footprint')
   for rel,fact in footprint.items():
    p=safe_path(root/rel)
    if (p.stat().st_size,p.stat().st_dev,p.stat().st_ino)!=(fact['bytes'],fact['device'],fact['inode']) or digest(p)!=fact['sha256']:raise Censor('live-footprint-race')
   before=dict(complete=True,kind='actual-open-footprint',files=len(footprint),logical_bytes=sum(x['bytes'] for x in footprint.values()),state_sha256=hashlib.sha256(encoded(footprint)).hexdigest())
   after=dict(complete=True,state_sha256=hashlib.sha256(encoded(footprint)).hexdigest())
   unchanged=True
   CTX.emit('before',[dict(path=k,**v) for k,v in sorted(footprint.items())],'before',True)
   CTX.emit('after',[after],'after',True)
  except Exception as e:
   status='technical_censor';error=error or dict(type=type(e).__name__,message=str(e),traceback=traceback.format_exc(),stage='footprint-after')
 if not unchanged:status='technical_censor'
 if req.get('require_before_index') is True and audit['index_opens']!=0:status='technical_censor'
 audit['index_open_sha256']=ah.hexdigest()
 loaded={str(Path(m.__file__).resolve()) for m in list(sys.modules.values()) if getattr(m,'__file__',None)}
 unexpected=[p for p in loaded if p not in sources and not Path(p).is_relative_to(Path(sys.base_prefix).resolve())]
 audit['checker_modules']={p:sources[p]['sha256'] for p in sorted(loaded&set(sources))}
 audit['unexpected_modules']=unexpected
 if unexpected or audit['spawn_events']:status='technical_censor'
 if payload is not None:
  payload['status']=status
  payload.update(engine_code_imported=bool(unexpected),engine_audit_used=bool(audit['spawn_events']))
  payload.update(source_unchanged=unchanged,all_files_directories_hashed=full_proof and unchanged,readonly_footprint_unchanged=unchanged,quiescent=req['quiescent'],final_pin=CONTRACT['engine_pin'])
  try:CTX.emit('payload',payload,'normalized')
  except Exception as e:status='technical_censor';error=dict(type=type(e).__name__,message=str(e),traceback=traceback.format_exc(),stage='payload')
 if error and not any(p.exists() for p in [CTX.paths['error'],CTX.paths['error'].with_name(CTX.paths['error'].name+'.partial')]):
  try:CTX.emit('error',error,'error')
  except Exception:status='technical_censor'
 actual=CTX.artifacts()
 if any(not x['complete'] for x in actual):status='technical_censor'
 for p,x in sources.items():
  if digest(p)!=x['sha256']:status='technical_censor'
 index=dict(version=1,mode=req['mode'],status=status,request_ref=dict(path=str(request_path),offset=0,bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest()),requested_root=req['root'],engine_pin=CONTRACT['engine_pin'],sources=sources,case_id=req['case_id'],scope_id=req['scope_id'],case_scope=req['case_scope'],proof=dict(before=before,after=after,source_unchanged=unchanged),artifacts=actual,index_open_audit=audit,result=proof_ref(CTX.paths['payload']) if CTX.paths['payload'].exists() else None,error=next((x for x in actual if x['role']=='error'),None))
 raw=encoded(index)
 if len(raw)>INDEX_RESERVE:raise Censor('index-byte-censor')
 with out.open('xb') as f:
  Sink(CTX,f,'index').write(raw);f.flush();os.fsync(f.fileno())
 print(json.dumps(dict(index=str(out),sha256=digest(out),status=status)))
 return 0 if status in {'passed','expected-independent-refusal'} else 2
if __name__=='__main__':sys.exit(main())
