import os,ast,hashlib,importlib.util,json,sys,threading,time,zlib
from pathlib import Path
R=RuntimeError
V=ValueError
D=dict
L=len
I=isinstance
P=Path
CASES=('normal-nested','sequential-128','rss-rows','rss-row-bytes','rss-raw','rss-gzip','disk-rows','disk-bytes','supervisor-rows','supervisor-bytes','identities','final-preflight','stdout','stderr','timeout','missing-live-rss','slow-first','start-gap','root-end-gap','whole-job-gap','exit-race','error-storm','eof233','close-creation','close-transient','census-error','pipe-fence')
def need(ok,label):
 if not ok:
  raise R(label)
def read(path,limit):
 with P(path).open('rb')as f:
  b=f.read(limit+1)
 need(L(b)<=limit,'input-cap')
 return b
def sha(b):
 return hashlib.sha256(b).hexdigest()
def strict(b):
 def pairs(items):
  d={}
  for k,v in items:
   need(k not in d,'duplicate-key')
   d[k]=v
  return d
 def bad(v):
  raise V('nonfinite')
 return json.loads(b,object_pairs_hook=pairs,parse_constant=bad)
def census_view(value):
 if value is None:
  return None
 return dict(epoch=value['epoch'],free_bytes=value['free_bytes'],categories={k:v['absolute_actual']for k,v in value['categories'].items()},charged={k:dict(bytes=v['bytes'],files=v['files'])for k,v in value['categories'].items()if v['charge_mode']=='positive_delta'},groups={k:dict(bytes=v['bytes'],files=v['files'])for k,v in value['shared_groups'].items()})
class Summary:
 def __init__(self,g,m,cases):
  self.g=g
  self.path=P(m['summary_path'])
  self.limit=m['summary_limit']
  self.written=0
  need(type(self.limit)is int and self.limit==65536 and self.path.is_absolute()and(self.path.resolve()==self.path)and self.path.parent.is_dir()and(not self.path.exists()),'summary-admission')
  sizes=0
  for case in cases:
   budget=g.Budget(case['budget'])
   snap=census_view(budget.check(self.reserves(budget,1)))
   def worst(x):
    if type(x)is dict:
     return{k:worst(v)for k,v in x.items()}
    return 2**64-1 if type(x)in(int,float)else x
   sizes+=L(g._encode(dict(kind='case',case=case['name'],qualified=False,censor='x'*128,receipt_sha256='f'*64,budget_sha256='f'*64,bounds_sha256='f'*64,run_nonce='f'*32,lifetime_sha256='f'*64,process_closure_verified=False,postwrite_census_complete=False,postwrite_census=worst(snap)),8192))
  need(sizes+8192<=self.limit,'summary-forecast')
  self.file=self.path.open('xb',buffering=0)
 def reserves(self,budget,files=0):
  result={k:dict(bytes=self.limit-self.written,files=files)for k,v in budget.categories.items()if self.path.is_relative_to(P(v['root']).resolve())}
  need(result,'admitted-summary-root')
  return result
 def put(self,value):
  raw=self.g._encode(value,8192)
  need(self.written+L(raw)<=self.limit,'summary-cap')
  self.g._write_all(self.file,raw)
  self.file.flush()
  self.g.os.fsync(self.file.fileno())
  self.written+=L(raw)
  return dict(path=str(self.path),offset=self.written-L(raw),bytes=L(raw),sha256=sha(raw))
def done(row,case,g):
 return dict(kind='case',case=case['name'],qualified=row.get('qualified',False),censor=row.get('censor'),receipt_sha256=row.get('receipt_sha256'),budget_sha256=row.get('budget_sha256'),bounds_sha256=row.get('bounds_sha256'),run_nonce=row.get('run_nonce'),lifetime_sha256=sha(g._encode([row.get('creator_lifetime'),row.get('job_close'),row.get('closure_checks',[])],65536)),process_closure_verified=row.get('process_closure_verified',False),postwrite_census_complete=row.get('postwrite_census_complete',False),postwrite_census=census_view(row.get('postwrite_census')))
def reconcile(g,row,gate,sink,until):
 need(L(g._UNCLOSED)==1 and g._UNCLOSED[0]['kind']=='unjoined-io','retained-fence')
 ctx=g._UNCLOSED[0]
 need(ctx['pipes'].thread.is_alive(),'actual-unjoined')
 try:
  g.run([],row['owner']['receipt'],'not-opened',1)
 except RuntimeError as e:
  need('previous-controller' in str(e),'admission-fence')
 else:
  raise R('fence-bypass')
 gate.set()
 token=g.TOKEN.open('r+b')
 locked=False
 try:
  while True:
   need(time.time()<until,'fence-token-deadline')
   try:
    token.seek(0)
    g.msvcrt.locking(token.fileno(),g.msvcrt.LK_NBLCK,1)
    locked=True
    break
   except OSError:
    time.sleep(0.01)
  current=strict(read(g.OWNER,8192))
  need(current['nonce']==row['run_nonce'],'fence-owner')
  proof=g._finish_io_fence(row['run_nonce'],until)
  need(L(g._UNCLOSED)==1 and g._UNCLOSED[0]is ctx,'no-uncertain-close')
  ref=sink.put(dict(kind='fence-closed',case='pipe-fence',proof=proof,rss=ctx['raw'].reference(),stdio=ctx['pipes'].reference()))
  current.update(state='closed',closure_resolution='caller-native-reconciliation',reconciliation_sha256=ref['sha256'])
  g.control_json(g.OWNER,current)
  g._UNCLOSED.remove(ctx)
  return proof
 finally:
  try:
   if locked:
    token.seek(0)
    g.msvcrt.locking(token.fileno(),g.msvcrt.LK_UNLCK,1)
  finally:
   token.close()
def negative(name,src,gh):
 call='x.creation_time(os.getpid())' if name=='close-creation' else 'x.JobSampling(N(frozen=True,bounds=N(b={"collections":{"identity":{"bytes":1024}}}),charge=lambda *a:False),None,lambda:[]).memory(os.getpid())'
 return 'import os,time,hashlib,importlib.util;from types import SimpleNamespace as N;p='+repr(str(src))+";s=importlib.util.spec_from_file_location('negative',p);b=open(p,'rb').read(77825);assert len(b)<=77824 and hashlib.sha256(b).hexdigest()=="+repr(gh)+';x=importlib.util.module_from_spec(s);exec(compile(b,p,"exec"),x.__dict__);x.K.CloseHandle=lambda h:0\ntry:\n '+call+"\nexcept RuntimeError:pass\nelse:raise AssertionError('no close fence')\nassert x._UNCLOSED;before=len(x._UNCLOSED)\ntry:x.run([],'unused','unused',1)\nexcept RuntimeError as e:assert 'previous-controller' in str(e)\nelse:raise AssertionError('admission unfenced')\nassert len(x._UNCLOSED)==before;print('retained-close-fence',flush=True);time.sleep(.1)"
def main():
 need(L(sys.argv)==3,'manifest')
 raw=read(sys.argv[1],131072)
 need(sha(raw)==sys.argv[2],'manifest')
 m=strict(raw)
 need(set(m)=={'version','runtime_go','guard','cases','summary_limit','summary_path','absolute_deadline'}and m['version']==1 and(type(m['version'])is int)and(m['runtime_go']is True),'RUN-GO')
 end=m['absolute_deadline']
 need(type(end)in(int,float) and time.time()<end<=min(time.time()+900,1792581014),'finite-absolute-deadline')
 watchdog=threading.Timer(end-time.time(),os._exit,args=(124,))
 watchdog.daemon=False
 watchdog.start()
 try:
  src=P(m['guard']['path'])
  code=read(src,77824)
  need(sha(code)==m['guard']['sha256'],'guard-SHA')
  need([c['name']for c in m['cases']]==list(CASES),'27-cases')
  for c in m['cases']:
   need(set(c)=={'name','budget','budget_sha256','receipt','seconds','expected_censor'},'closed-case')
   need(type(c['seconds'])in(int,float)and 0<c['seconds']<=30,'stage-time')
   b=read(c['budget'],131072)
   need(sha(b)==c['budget_sha256'],'budget')
  gh=m['guard']['sha256']
  spec=importlib.util.spec_from_file_location('bounded_guard',src)
  g=importlib.util.module_from_spec(spec)
  exec(compile(code,str(src),'exec'),g.__dict__)
  nodes={n.name:n for n in ast.walk(ast.parse(code))if I(n,ast.FunctionDef)and n.name in('run','memory_watch')}
  lines=code.decode().splitlines()
  def loc(fn,text,after=0):
   n=nodes[fn]
   return next((i+1 for i,line in enumerate(lines)if n.lineno<=i+1<=n.end_lineno and i+1>after and(text in line)))
  tick,collect=(loc('memory_watch','tick = qpc()'),loc('memory_watch','pids = current_members()'))
  root=loc('run',"coverage('root-end-sampling-coverage')")
  tail=loc('run','if current_members():',root)
  wait=loc('run','process.wait(timeout=max(')
  results=[]
  halt=False
  sink=Summary(g,m,m['cases'])
  sink.put(dict(kind='header',manifest_sha256=sha(raw),guard_sha256=gh))
  rw=g.walk_tree
  rp=g.K.PeekNamedPipe
  rm,rc=(g._memory_handle,g.K.CreateJobObjectW)
  child='import os,time;print(os.getpid(),flush=True);time.sleep(.08)'
  for c in m['cases']:
   name=c['name']
   if halt or time.time()+46>end:
    results.append(D(case=name,status='NOT_STARTED',reason='held'))
    continue
   jobs=[]
   hits=[]
   block=threading.Event()
   gate=threading.Event()
   holder={}
   program="import time;print('OK',flush=True);time.sleep(.25)"
   if name in('normal-nested','exit-race','whole-job-gap'):
    program="import sys,time,subprocess;p=subprocess.Popen([sys.executable,'-I','-S','-B','-u','-c',"+repr(child.replace('.08','10' if name=='whole-job-gap' else '.6'))+"]);sum(i*i for i in range(100000));print('OK',flush=True);print('ERR',file=sys.stderr,flush=True);"+('time.sleep(.2)' if name=='whole-job-gap' else 'p.wait()')
   if name=='sequential-128':
    program="import sys,subprocess\nfor i in range(128):subprocess.run([sys.executable,'-I','-S','-B','-u','-c',"+repr(child)+"],check=True)\nprint('OK',flush=True)"
   if name in('stdout','stderr'):
    program='import os\nwhile True:os.write('+('1' if name=='stdout' else '2')+",b'x'*4096)"
   if name in('timeout','disk-rows','disk-bytes','supervisor-rows','supervisor-bytes'):
    program='import time;time.sleep(60)'
   if name=='identities':
    program="import sys,subprocess;subprocess.run([sys.executable,'-I','-S','-B','-c','import time;time.sleep(2)'])"
   def create(*a):
    value=rc(*a)
    jobs.append(bool(value))
    return value
   def memory(h):
    value=rm(h)
    pid=g.K.GetProcessId(h)
    if threading.current_thread().name!='owned-memory':
     holder.setdefault('root',pid)
    if not hits and threading.current_thread().name=='owned-memory':
     if name=='missing-live-rss':
      need(value and value[0]>0,'LIVE')
      hits.append(name)
      return None
     if name=='exit-race' and pid!=holder.get('root'):
      holder['terminate']('controlled-departure')
      need(g.K.WaitForSingleObject(h,1000)==0,'exit')
      hits.append(name)
      return None
    return value
   def trace(f,event,arg):
    if f.f_code.co_filename!=str(src)or event!='line':
     return trace
    fn=f.f_code.co_name
    n=f.f_lineno
    if fn=='memory_watch':
     holder['terminate']=f.f_locals['terminate']
     if block.is_set()and n==tick:
      gate.wait(2)
    if not hits:
     if fn=='memory_watch' and(name=='slow-first' and n==collect or(name=='start-gap' and n==tick and(f.f_locals.get('last_tick')is not None))):
      time.sleep(0.06)
      hits.append(name)
     elif name in('root-end-gap','whole-job-gap')and fn=='run' and(n==(root if name=='root-end-gap' else tail)):
      block.set()
      time.sleep(0.065)
      hits.append(name)
     elif name=='error-storm' and fn=='run' and(n==wait):
      for i in range(1000):
       f.f_locals['errors'].append('controlled-fixed-error')
      hits.append(name)
     elif name=='pipe-fence' and fn=='drain' and f.f_locals.get('data'):
      hits.append(name)
      need(gate.wait(15),'gate')
    if name!='pipe-fence' and fn=='run' and('stop' in f.f_locals)and f.f_locals['stop'].is_set():
     gate.set()
    return trace
   def walk(*args,**kw):
    if name=='census-error' and threading.current_thread().name=='owned-disk' and(not hits):
     hits.append(name)
     kw['onerror'](PermissionError('injected-enumeration-error'))
    return rw(*args,**kw)
   def peek(*args):
    if name=='eof233' and threading.current_thread().name=='owned-stdio' and(not hits):
     hits.append(name)
     g.ctypes.set_last_error(233)
     return 0
    return rp(*args)
   if name in('close-creation','close-transient'):
    program=negative(name,src,gh)
   row=None
   try:
    g.walk_tree=walk
    g.K.PeekNamedPipe=peek
    g.K.CreateJobObjectW=create
    g._memory_handle=memory
    if name in('slow-first', 'start-gap', 'root-end-gap', 'whole-job-gap', 'error-storm', 'pipe-fence', 'exit-race'):
     threading.settrace(trace)
     sys.settrace(trace)
    row=g.run([sys.executable,'-I','-S','-B','-u','-c',program],c['receipt'],c['budget'],c['seconds'],reserves=sink.reserves(g.Budget(c['budget'])))
    sink.put(dict(done(row,c,g),faults=hits,jobs=L(jobs)))
    expected=c['expected_censor']
    need(row.get('censor')==expected if expected is not None else row.get('qualified')is True,'verdict')
    need(name=='pipe-fence' or(row['process_closure_verified']and L(row['closure_checks'])==2 and all((x['members']==[]and x['run_nonce']==row['run_nonce']and(type(x['perf_counter_ns'])is int)for x in row['closure_checks']))),'closure')
    need((name=='pipe-fence' or row['postwrite_census_complete']is True)and row['guard_sha256']==gh,'census')
    b=strict(read(c['budget'],131072))['evidence_bounds']
    p=P(c['receipt']).with_suffix('.rss.jsonl.gz')
    if p.exists()and name!='pipe-fence':
     packed=read(p,b['rss']['gzip_bytes'])
     d=zlib.decompressobj(31)
     data=d.decompress(packed,b['rss']['raw_bytes']+1)
     need(d.eof and(not d.unused_data),'single-gzip-EOF')
     need(sha(data)==row['rss_evidence']['raw_sha256'],'raw')
    if name=='pipe-fence':
     reconcile(g,row,gate,sink,min(end,time.time()+8))
    for stream,proof in[]if name=='pipe-fence' else row['stdio'].items():
     data=read(P(c['receipt']).with_suffix('.'+stream),b['stdio'][stream]['capture_bytes']+b['stdio'][stream]['overflow_tail_bytes'])
     need(L(data)==proof['retained_bytes']and sha(data)==proof['retained_sha256'],'stdio')
     if expected is None:
      need(proof['complete']and proof['eof'],'EOF')
    if name=='sequential-128':
     need(L(row['native_identity_rows'])>=129,'identities')
    if name in('eof233','census-error','slow-first','start-gap','root-end-gap','whole-job-gap','missing-live-rss','exit-race','pipe-fence','error-storm'):
     need(hits,'injection')
    need(row['termination_native_attempts']<=4,'term')
    results.append(D(case=name,status='PASSED',receipt=c['receipt'],jobs=L(jobs),faults=hits))
   except Exception as error:
    ok=name=='final-preflight' and(not jobs)and('prospective-final-json-envelope' in str(error))
    results.append(D(case=name,status='PASSED_PREFLIGHT' if ok else 'FAILED',error=str(error)[:128],jobs=L(jobs),injections=hits))
    if row is None:
     sink.put(dict(kind='case-error',case=name,qualified=False,error=str(error)[:128],guard_receipt=None))
    if row is None and jobs:
     halt=True
   finally:
    if name!='pipe-fence':gate.set()
    sys.settrace(None)
    threading.settrace(None)
    g.walk_tree=rw
    g.K.PeekNamedPipe=rp
    g.K.CreateJobObjectW=rc
    g._memory_handle=rm
    if g._UNCLOSED:
     halt=True
  sink.put(dict(kind='results',results=[dict(case=x['case'],status=x['status'])for x in results]))
  sink.file.close()
  actual=census_view(g.Budget(m['cases'][-1]['budget']).check())
  out=dict(status='CONTROLS_PASSED' if all((r['status'].startswith('PASSED')for r in results))else 'FAILED_OR_INCOMPLETE',summary_path=str(sink.path),summary_bytes=sink.written,summary_sha256=sha(read(sink.path,65536)),final_caller_census=actual,guard_sha256=gh,manifest_sha256=sha(raw))
  encoded=g._encode(out,8192)
  sys.stdout.buffer.write(encoded)
  sys.stdout.buffer.flush()
 finally:
  watchdog.cancel()
  watchdog.join(1)
  need(not watchdog.is_alive(),'global-watchdog-unjoined')
if __name__=='__main__':
 main()

