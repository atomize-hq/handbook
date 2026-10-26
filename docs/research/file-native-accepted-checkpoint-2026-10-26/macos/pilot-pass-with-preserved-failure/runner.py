import sys
sys.dont_write_bytecode=True
import os,json,time,hashlib,threading,subprocess,signal,traceback,shutil,ast,fcntl,copy
from pathlib import Path
from contextlib import contextmanager
OUT=Path(__file__).resolve().parent
SDK=Path('/private/tmp/file-native-online-macos-20261020/source/pilot-sdk-20261021/sdk')
HELPERS=Path('/private/tmp/file-native-online-macos-20261020/execution/readiness-20261021')
PIN='0628322e4b48e0efad19cdc32238a622d4aaa7a8'
ROOT=OUT/'store/disposable/store'
BIN=OUT/'target/release/file-native-incremental'
os.environ['FILE_NATIVE_ORACLE_RESEARCH']=str(SDK.parent/'research');os.environ['DYLD_LIBRARY_PATH']='/opt/homebrew/lib'
sys.path[:0]=[str(OUT),str(SDK),str(HELPERS)]
from native_probe import identity,usage,group
from monitor_v20 import OwnedTree,valid_identity,same_birth
import focused_protocol as fp
import pilot_behavior
from case_ledger import CaseLedger
need=fp.require
START=time.monotonic();DEADLINE=START+300
fp.FUNCTIONAL_CUTOFF=time.time()+300
LOCK=threading.RLock();EVENTS=(OUT/'logs/pilot-events.jsonl').open('x');SAMPLES=(OUT/'logs/native-samples.jsonl').open('x')
def sha(b):return hashlib.sha256(b).hexdigest()
def encoded(v):return json.dumps(v,sort_keys=True,separators=(',',':'),ensure_ascii=False,allow_nan=False).encode()
def emit(row):
 with LOCK:
  EVENTS.write(json.dumps({'perf_ns':time.perf_counter_ns(),**row},default=str,separators=(',',':'))+'\n');EVENTS.flush()

def source_check():
 rows=json.loads((OUT/'source-before.json').read_text())
 for r in rows:need(sha(Path(r['path']).read_bytes())==r['after_sha256'],'engine-source-drift:'+r['path'])
 return rows

class Processes:
 def __init__(self):
  self.active={};self.all=[];self.failures=[];self.stopping=threading.Event();self.peak=0;self.missing=[]
  self.thread=threading.Thread(target=self.monitor,name='native-monitor');self.thread.start()
 def kill(self,c,sig):
  p=c['p'];row=identity(p.pid)
  if p.returncode is None and valid_identity(row) and same_birth(c['birth'],row) and row['pgid']==p.pid:
   try:os.killpg(p.pid,sig);emit({'kind':'owned-signal','pid':p.pid,'signal':sig})
   except ProcessLookupError:pass
  for pid,expected in list(c['tree'].known.items()):
   row=identity(pid)
   if valid_identity(row) and same_birth(expected,row) and row['pgid']!=p.pid:
    try:os.kill(pid,sig)
    except ProcessLookupError:pass
 def monitor(self):
  while not self.stopping.is_set():
   with LOCK:contexts=list(self.active.values())
   total=usage(os.getpid())['rss_bytes'];facts=[]
   for c in contexts:
    if c.get('done'):continue
    try:
     view=c['tree'].observe()
     for row in view['identities']:
      if row.get('state')==5:continue
      u=usage(row['pid']);after=identity(row['pid'])
      if u['return']==0 and valid_identity(after) and same_birth(row,after):
       rss=u['rss_bytes'];total+=rss;c['peak']=max(c['peak'],rss);facts.append({'pid':row['pid'],'role':c['role'],'rss':rss})
       if rss>c['limit']:raise RuntimeError('role-RSS-limit:'+c['role'])
      else:self.missing.append({'pid':row['pid'],'usage':u,'after':after})
    except BaseException as e:
     if not c.get('done') and c['p'].returncode is None:self.failures.append(repr(e));self.kill(c,signal.SIGTERM)
   self.peak=max(self.peak,total)
   with LOCK:SAMPLES.write(json.dumps({'elapsed':time.monotonic()-START,'aggregate_rss':total,'processes':facts})+'\n');SAMPLES.flush()
   if total>3*1024**3 or time.monotonic()>DEADLINE or shutil.disk_usage(OUT).free<20*1024**3:
    self.failures.append('aggregate-memory/deadline/free-floor')
    for c in contexts:self.kill(c,signal.SIGTERM)
   self.stopping.wait(.05)
 def command(self,label,argv,input_bytes,*,role='checker',limit=2*1024**3,timeout=60,env=None,on_started=None,root=None):
  need(time.monotonic()<DEADLINE,'pilot-workload-timeout');base=OUT/'logs'/label
  environment=os.environ.copy();environment.update({'TMPDIR':str(OUT/'tmp'),'TMP':str(OUT/'tmp'),'TEMP':str(OUT/'tmp'),'PYTHONDONTWRITEBYTECODE':'1','DYLD_LIBRARY_PATH':'/opt/homebrew/lib','FILE_NATIVE_ORACLE_RESEARCH':str(SDK.parent/'research')});environment.update(env or {})
  (base.with_suffix('.input')).write_bytes(input_bytes)
  p=subprocess.Popen(argv,cwd=OUT,env=environment,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
  raw={'pid':p.pid,'parent':os.getpid(),'pgid':os.getpgid(p.pid),'argv':argv,'role':role,'root':str(root) if root else None,'env_overrides':env or {},'start_ns':time.perf_counter_ns()}
  (base.with_suffix('.child.json')).write_text(json.dumps(raw,indent=2));birth=identity(p.pid);raw['birth']=birth
  c=None;stdout=b'';stderr=b'';failure=None;closures=[];started=time.monotonic()
  try:
   need(valid_identity(birth) and birth['ppid']==os.getpid() and birth['pgid']==p.pid,'original-child-binding')
   c={'p':p,'birth':birth,'tree':OwnedTree(birth),'role':role,'limit':limit,'peak':0,'root':root,'done':False}
   with LOCK:self.active[p.pid]=c;self.all.append(c)
   u=usage(p.pid);need(u['return']==0 and 0<u['rss_bytes']<=limit,'positive-role-RSS-before-input')
   actual={'pid':p.pid,'positive_rss':u['rss_bytes'],'before_stdin_delivery':True,'observed_ns':time.perf_counter_ns(),'birth':birth}
   raw['before_input']=actual
   if on_started:on_started(actual)
   stdout,stderr=p.communicate(input=input_bytes,timeout=min(timeout,DEADLINE-time.monotonic()))
  except BaseException as e:
   failure={'error':repr(e),'traceback':traceback.format_exc()}
  finally:
   if p.returncode is None:
    if c:self.kill(c,signal.SIGTERM)
    else:
     try:os.killpg(p.pid,signal.SIGTERM)
     except ProcessLookupError:pass
    try:stdout,stderr=p.communicate(timeout=3)
    except subprocess.TimeoutExpired:
     if c:self.kill(c,signal.SIGKILL)
     else:
      try:os.killpg(p.pid,signal.SIGKILL)
      except ProcessLookupError:pass
     stdout,stderr=p.communicate(timeout=5)
   if c:
    c['done']=True
    with LOCK:self.active.pop(p.pid,None)
   for _ in range(2):closures.append(group(p.pid));time.sleep(.015)
   escaped=[]
   if c:
    for pid,expected in c['tree'].known.items():
     row=identity(pid)
     if valid_identity(row) and same_birth(expected,row):escaped.append(row)
   raw.update(exit=p.returncode,seconds=time.monotonic()-started,stdout_eof=True,stderr_eof=True,stdin_closed=True,original_handle_waited=True,stdout_bytes=len(stdout),stderr_bytes=len(stderr),stdout_sha256=sha(stdout),stderr_sha256=sha(stderr),exception=failure,closure=closures,escaped_survivors=escaped,sampled_peak_process_rss=c['peak'] if c else None)
   base.with_suffix('.stdout').write_bytes(stdout);base.with_suffix('.stderr').write_bytes(stderr);base.with_suffix('.result.json').write_text(json.dumps(raw,indent=2));emit({'kind':'native-command-closed','label':label,**raw})
  need(failure is None,'native-command-exception:'+repr(failure));need(all(x['valid'] and not x['members'] for x in closures) and not escaped,'native-closure-unknown');need(not self.failures,'native-monitor-failure:'+repr(self.failures))
  return raw,stdout,stderr
 def finish(self):
  with LOCK:contexts=list(self.active.values())
  for c in contexts:self.kill(c,signal.SIGTERM)
  self.stopping.set();self.thread.join(timeout=5);need(not self.thread.is_alive(),'native-monitor-thread-close');need(not self.active,'native-processes-still-active')
  return {'processes_closed':True,'handles_closed':True,'native_commands':len(self.all),'peak_sampled_aggregate_rss':self.peak,'missing_rss':self.missing,'monitor_failures':self.failures}

class Driver:
 def __init__(self,pool):self.pool=pool;self.binary_sha256=sha(BIN.read_bytes());self.count=0;self.rows=[]
 def call(self,root,action,*,timeout=60,code=0,env=None,on_started=None,**fields):
  need(root==ROOT,'single-pilot-root')
  with LOCK:
   self.count+=1;n=self.count;need(n<=37,'37-engine-call-cap')
  payload={'root':str(root),'action':action,**fields};r,o,e=self.pool.command(f'engine-{n:02d}-{action}',[str(BIN)],encoded(payload),role='old-reader' if action=='resolve' else 'engine',limit=256*1024**2 if action=='resolve' else 2*1024**3,timeout=timeout,env=env,on_started=on_started,root=root)
  self.rows.append(r);need(r['exit']==code,'engine-exit:'+str(r['exit'])+':'+e.decode(errors='replace'));response=fp.strict(o,4*1024**2);need(response['status']=='ok','engine-error:'+str(response));return response

class Adapter(fp.ThreadCalls):
 capabilities=pilot_behavior.__dict__.get('REQUIRED_CAPABILITIES',frozenset({'barrier:pack-catalog-captured','barrier:physical-before-selection','barrier:candidate-ready'}))
 wall_time=staticmethod(time.time);monotonic=staticmethod(time.monotonic)
 def __init__(self,pool):self.pool=pool;self.callbacks=[];self.controls=[];self.host=self;self.control_seq=0;self.output_root=OUT/'checker';self.oracle_helper=None
 def spawn(self,callback):h=super().spawn(callback);self.callbacks.append(h);return h
 def control(self,root,name):
  self.control_seq+=1;prefix=OUT/'controls'/f'{self.control_seq}-{name}';c={'ready':str(prefix)+'.ready','release':str(prefix)+'.release','name':name};self.controls.append(c);return c
 def await_ready(self,c,h,seconds):
  stop=time.monotonic()+seconds
  while not Path(c['ready']).exists():
   if not self.alive(h):self.wait(h,0);raise RuntimeError('role-ended-before-ready')
   need(time.monotonic()<stop,'ready-timeout:'+c['name']);time.sleep(.005)
  emit({'kind':'actual-barrier-ready','control':c,'phase':Path(c['ready']).read_text(),'ready_ns':time.perf_counter_ns()})
 def release(self,c):
  Path(c['release']).touch(exist_ok=True);emit({'kind':'actual-barrier-release','control':c,'release_ns':time.perf_counter_ns()})
 def sleep(self,seconds):emit({'kind':'reader-hold-sleep','seconds':seconds});time.sleep(seconds)
 def read(self,root,name,limit):
  p=root/name;need(p.is_relative_to(ROOT),'read-path');need(p.stat().st_size<=limit,'bounded-read');return p.read_bytes()
 def exists(self,root,name):return (root/name).exists()
 def inventory(self,root):
  rows={}
  for p in sorted(root.rglob('*')):
   if p.is_file():b=p.read_bytes();rows[p.relative_to(root).as_posix()]={'bytes':len(b),'sha256':sha(b)}
  return rows
 def emit(self,row):emit(row)
 def quiesce(self,root):
  with LOCK:need(not any(c['root']==root and not c['done'] for c in self.pool.active.values()),'oracle-requires-quiescent-engine')
 def before_call(self,*args):need(not self.pool.failures,'native-monitor-failure')
 def mapping(self,root,key):return self.oracle_helper.mapping(root,key)
 def backlog(self,root):return self.oracle_helper.backlog(root)
 def close_case(self,*args):
  for c in self.controls:self.release(c)
  for h in self.callbacks:
   if self.alive(h):
    for c in list(self.pool.active.values()):self.pool.kill(c,signal.SIGTERM)
   h['thread'].join(10);need(not self.alive(h),'callback-not-closed')
  self.quiesce(ROOT);return {'processes_closed':True,'handles_closed':True,'python_callbacks_joined':True}

# Reuse exact independent decoder/mapping/backlog method bodies; omit only unrelated
# host framework import dependencies, never replace independent checks.
source=(SDK/'suite_oracle.py').read_text();module=ast.parse(source);cls=next(n for n in module.body if isinstance(n,ast.ClassDef) and n.name=='Oracle');cls.body=[n for n in cls.body if isinstance(n,ast.FunctionDef) and n.name in {'__init__','decoder','mapping','backlog'}];code=ast.fix_missing_locations(ast.Module(body=[cls],type_ignores=[]));ns={'Path':Path,'hashlib':hashlib,'fcntl':fcntl,'encoded':encoded,'need':need,'digest':lambda p:sha(Path(p).read_bytes())};exec(compile(code,str(SDK/'suite_oracle.py'),'exec'),ns);Independent=ns['Oracle']

class Harness(fp.Harness):
 @contextmanager
 def case(self,case_id,root,forecast):
  self.check_time();need(not root.exists(),'fresh-disposable-root');need(shutil.disk_usage(OUT).free>20*1024**3,'Mac-free-floor');source_check();self.active.add(root);self.ledgers[root]=fp.Inputs();emit({'kind':'case-start-current-authority','case_id':case_id,'root':str(root),'forecast':forecast,'source_identity':'baseline062 plus fixture10','not_full_guard_qualification':True})
  try:yield
  finally:
   self.active.remove(root);closure=self.a.close_case(case_id,root);emit({'kind':'case-closure','closure':closure})
 def cleanup(self,*args,**kwargs):
  result=super().cleanup(*args,**kwargs);self.note('pilot-cleanup-pass',batches=result);return result

POOL=None;H=None;failure=None;result={}
try:
 source_check();build=json.loads((OUT/'logs/build-result.json').read_text());need(build['exit']==0 and build['owned_tree_closed'],'CLI-build-binding');need(BIN.exists(),'native-CLI-missing')
 POOL=Processes();a=Adapter(POOL);driver=Driver(POOL);helper=Independent(a);a.oracle_helper=helper
 def oracle(*,root,checkpoint,final_pin,known_requests,required_mapped_keys):
  a.quiesce(root);before=a.inventory(root);(OUT/'checker/known-requests.json').write_text(json.dumps(known_requests,indent=2));r,o,e=POOL.command('independent-complete-oracle',[sys.executable,'-I','-S','-B',str(OUT/'checker.py')],b'',role='checker',timeout=120,root=None);need(r['exit']==0,'complete-independent-oracle-failed:'+e.decode(errors='replace'));proof=json.loads((OUT/'checker/complete-oracle.json').read_text());after=a.inventory(root);need(before==after,'oracle-mutated-root');d=helper.decoder(root)
  for key in required_mapped_keys:need(key in d.entries,'oracle-mapped-key')
  need(proof['request_intents'] and proof['semantic']['publications']==3 and proof['navigation']['documents']==64,'full-oracle-inputs')
  receipt=OUT/'checker/complete-oracle.json';return {'status':'passed','final_pin':final_pin,'actual_H':3,'mapped_keys':required_mapped_keys,'engine_code_imported':False,'engine_audit_used':False,'checks':sorted(fp.ORACLE_CHECKS),'source_unchanged':True,'receipt_sha256':sha(receipt.read_bytes()),'receipt':str(receipt),'underlying':proof,'before_after_inventory_sha256':sha(encoded(before))}
 H=Harness(driver,a,oracle,emit,final_pin=PIN);forecast={'human_scope':'one small A64H3 pilot,37enginecalls,oldreader>15s,complete independent v6 oracle','deadline_epoch':fp.FUNCTIONAL_CUTOFF,'source_identity':'baseline062 plus fixture10','RSS_aggregate':3*1024**3,'old_reader_RSS':256*1024**2,'checker_RSS':2*1024**3}
 result=pilot_behavior.run(H,ROOT,forecast);result['engine_calls']=driver.count;result['accepted_inputs']=H.ledgers[ROOT].oracle_inputs();result['oracle_receipts']=H.oracles;result['source_after']=source_check();(OUT/'accepted-inputs-results.json').write_text(json.dumps(H.ledgers[ROOT].accepted,indent=2));result['final_state']=H.state(ROOT);result['final_backlog']=a.backlog(ROOT);result['binary_sha256']=driver.binary_sha256
except BaseException as e:failure={'error':repr(e),'traceback':traceback.format_exc()};emit({'kind':'pilot-failure',**failure})
finally:
 if H:
  try:H.a.close_case('finalizer',ROOT)
  except BaseException as e:result['callback_close_error']=repr(e)
 if POOL:
  try:result['native_closure']=POOL.finish()
  except BaseException as e:result['native_close_error']=repr(e)
 result.update(status='FAIL' if failure or result.get('native_close_error') or result.get('callback_close_error') else 'PASS',failure=failure,elapsed_seconds=time.monotonic()-START,engine_calls=H.driver.count if H else 0,source_identity='baseline062 plus fixture10',adapter_sha256=sha(Path(__file__).read_bytes()))
 (OUT/'pilot-result.json').write_text(json.dumps(result,indent=2,default=str));emit({'kind':'pilot-final','status':result['status'],'engine_calls':result['engine_calls'],'failure':failure});EVENTS.close();SAMPLES.close()
 if H:
  ledger=CaseLedger('Mac',PIN);case=pilot_behavior.CASE;ledger.rows[case]={'case_id':case,'group':'separately-authorized-small-pilot','status':'PENDING','obligation':'retained A64H3 behavior only,not full matrix'}
  observation={'case_id':case,'status':'PASSED' if result['status']=='PASS' else 'FAILED','final_pin':PIN,'host':'Mac','raw_sha256':sha((OUT/'logs/pilot-events.jsonl').read_bytes()),'binary_sha256':H.driver.binary_sha256,'case_source_sha256':sha((OUT/'pilot_behavior.py').read_bytes()),'oracle_receipts':H.oracles,'guard_receipt_sha256':sha((OUT/'pilot-result.json').read_bytes()),'full_oracle':bool(H.oracles),'native':True,'processes_closed':not result.get('native_close_error'),'source_verified':failure is None,'version_mapping':'baseline062 + test-fixture n1-to10;same format','required_checks_complete':result['status']=='PASS','reason':str(failure),'qualification':'small functional pilot only; no full guard/matrix qualification'}
  ledger.record(case,observation);(OUT/'case-ledger.json').write_text(json.dumps(ledger.result(),indent=2,default=str))
 print(json.dumps({'status':result['status'],'engine_calls':result['engine_calls'],'failure':failure,'output':str(OUT)},indent=2));sys.exit(0 if result['status']=='PASS' else 1)
