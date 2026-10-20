"""External Mac guard and I/O adapters. Importing this module executes no engine."""
import os,sys,json,time,hashlib,gzip,subprocess,threading,signal,contextlib,fcntl,shutil,runpy,stat,datetime
from pathlib import Path
ROOT=Path('/private/tmp/file-native-p-macos-20261020');C=ROOT/'cargo-private-proposed';Q=Path(__file__).resolve().parent
R=ROOT/'native-P-full-v1';F=R/'fixtures';CFG=json.loads((Q/'config.json').read_bytes());EXE=ROOT/'frozen-bin-private-proposed-v6/file-native-incremental';LIB=Path('/opt/homebrew/Cellar/zstd/1.5.7_1/lib/libzstd.1.5.7.dylib');EXPECTED=CFG['exe_sha256'];CORE='d70f3864f2888d46d9112213b4a03f8d82745a8e'
ORIGINAL=datetime.datetime.fromisoformat(CFG['original_functional_cutoff'].replace('Z','+00:00')).timestamp();CUTOFF=min(ORIGINAL,float(os.environ.get('P_STAGE_CUTOFF',ORIGINAL)))
HELPER=C/'.p-build-receipts-v6/build-supervisor.py';assert hashlib.sha256(HELPER.read_bytes()).hexdigest()=='a737c9f564290840c4c66c95570b9b9fc10aac6b20433882d31f78280116d645','native-helper-code-drift'
N=runpy.run_path(str(HELPER),run_name='reviewed_native_helpers_only');local=threading.local();lock=threading.RLock();reservations={};counter=0
class Refused(RuntimeError):pass
def digest(p):return N['digest'](Path(p))
def _strict_census(p):
 p=Path(p)
 if not p.exists():return {'bytes':0,'files':0}
 total=files=0
 for b,ds,ns in os.walk(p,followlinks=False,onerror=lambda e:(_ for _ in ()).throw(e)):
  for n in ds+ns:
   q=Path(b)/n;s=q.lstat()
   if stat.S_ISLNK(s.st_mode):raise Refused('census-alias:'+str(q))
   if n in ns:
    if not stat.S_ISREG(s.st_mode):raise Refused('census-nonregular:'+str(q))
    total+=s.st_size;files+=1
 return {'bytes':total,'files':files}
census_transients=[]
def census(p):
 started=time.monotonic()
 for attempt in range(3):
  try:return _strict_census(p)
  except FileNotFoundError as exc:
   if exc.filename is None or not Path(exc.filename).is_relative_to(R):raise
   observation={'epoch':time.time(),'attempt':attempt+1,'measurement':None,'errno':exc.errno,'vanished_live_qualification_path':exc.filename}
   census_transients.append(observation);assert len(census_transients)<=1000,'bounded-census-null-receipts'
   print(json.dumps(observation),file=sys.stderr,flush=True)
   if time.monotonic()-started>=1:break
 raise Refused('repeated-NULL-live-qualification-census')
def ownership():
 whole=census(ROOT);fixtures=census(F);private=census(C);free=os.statvfs(ROOT).f_bavail*os.statvfs(ROOT).f_frsize
 return {'native':whole,'fixtures':fixtures,'private':private,'free_bytes':free}
def admit_full():
 x=ownership();forecast=x['native']['bytes']+CFG['fixture_bytes']+CFG['report_coexistence_bytes']+CFG['closing_returns_bytes']+CFG['incidental_bytes'];files=x['native']['files']+CFG['fixture_files'];assert forecast<=CFG['whole_bytes'] and files<=CFG['whole_files'];assert x['private']['bytes']<=CFG['private_bytes'] and x['free_bytes']>20*1024**3
 return {'actual':x,'forecast_bytes':forecast,'forecast_files':files,'reclaim_assumed':0}
def sourcecheck():
 p=ROOT/'p-candidate-d70f-source-manifest.json';assert digest(p)==CFG['manifest_sha256'];m=json.loads(p.read_bytes());assert m['core_commit']==CORE and len(m['files'])==145 and len(m['final_core_files'])==18
 for row in m['files']:
  q=ROOT/'source-final'/row['path'];assert q.stat().st_size==row['bytes'] and digest(q)==row['sha256']
 assert digest(EXE)==EXPECTED and digest(LIB)==CFG['lib_sha256']
 assert digest(HELPER)=='a737c9f564290840c4c66c95570b9b9fc10aac6b20433882d31f78280116d645'
 assert digest(C/'.p-preservation-receipts-v6/archive-stage.py')=='38ef76e1cb55b1fe0b432871a327f986cada1acdfc821ee5c2dc3eab0ea4b743'
 return {'core':CORE,'source145_core18':True,'exe_sha256':EXPECTED,'lib_sha256':CFG['lib_sha256'],'manifest_sha256':CFG['manifest_sha256']}
def enforce():
 x=ownership()
 assert x['native']['bytes']+CFG['incidental_bytes']<=CFG['whole_bytes'] and x['native']['files']<=CFG['whole_files'];assert x['fixtures']['bytes']<=CFG['fixture_bytes'] and x['fixtures']['files']<=CFG['fixture_files'];assert x['private']['bytes']<=CFG['private_bytes'] and x['free_bytes']>20*1024**3
 reports=census(R);assert reports['bytes']-x['fixtures']['bytes']<=CFG['closing_returns_bytes'],'all-closing-returns-bound'
 return x
def append_record(path,row):
 data=gzip.compress((json.dumps(row,separators=(',',':'))+'\n').encode(),mtime=0)
 with lock, (R/'.ledger.lock').open('ab') as ledger_lock:
  fcntl.flock(ledger_lock,fcntl.LOCK_EX)
  rall=census(R);fx=census(F);used=rall['bytes']-fx['bytes']
  assert used+len(data)+131072<CFG['closing_returns_bytes'],'closing-return-bound'
  with Path(path).open('ab') as f:f.write(data)
@contextlib.contextmanager
def reserve(bs,files,name,covered=False):
 if getattr(local,'covered',0):yield;return
 global counter
 with lock:
  x=enforce();rb=sum(v[0] for v in reservations.values());rf=sum(v[1] for v in reservations.values());assert x['native']['bytes']+rb+bs+CFG['incidental_bytes']<=CFG['whole_bytes'];assert x['fixtures']['bytes']+rb+bs<=CFG['fixture_bytes'] and x['fixtures']['files']+rf+files<=CFG['fixture_files'];assert x['native']['files']+rf+files<=CFG['whole_files'];counter+=1;key=(os.getpid(),counter);reservations[key]=(bs,files)
  append_record(R/'operation-forecasts.jsonl.gz',{'epoch':time.time(),'name':name,'new_bytes':bs,'new_files':files,'existing':x,'other_reservations':rb})
 if covered:local.covered=getattr(local,'covered',0)+1
 try:yield
 finally:
  if covered:local.covered-=1
  with lock:reservations.pop(key,None)
def engine_forecast(root,action,fields):
 x=census(root)
 if action in ['resolve','point','inspect','lookup-bench','dump','audit'] or (action=='packing' and fields.get('operation')=='status'):return 65536,32
 if action=='init':return (8388608,1024) if fields.get('count',16)>16 else (2097152,256)
 if action=='packing':return 3*x['bytes']+1048576,512
 return 2*x['bytes']+1048576,768
def run(argv,seconds,name,input=None,env=None,limit=2147483648,disk=False):
 if time.time()>=CUTOFF:raise Refused('fixed-cutoff')
 deadline=min(CUTOFF,time.time()+seconds);started=time.monotonic();output={};violations=[];peak=per=None;samples=0;missing=0;reason=None;thread=None;watch=None
 proc=subprocess.Popen([sys.executable,'-B',str(Q/'launch.py'),json.dumps(list(map(str,argv)))],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,env=env,start_new_session=True)
 def communicate():
  try:output['streams']=proc.communicate(b'G'+(input or b''),timeout=max(.01,deadline-time.time())+3)
  except BaseException as e:output['error']=repr(e)
 try:
  watch=N['arm'](proc.pid,deadline)
  # Capture initial native RSS while the activation gate is still closed.
  initial=N['ps_group'](proc.pid);assert initial,'initial-native-RSS-NULL';peak=sum(z['rss_bytes'] for z in initial);per=max(z['rss_bytes'] for z in initial);samples=1;assert peak<=limit and per<=2147483648
  thread=threading.Thread(target=communicate,daemon=True);thread.start()
  while thread.is_alive():
   try:
    rows=N['ps_group'](proc.pid)
    if not rows:
     if proc.poll() is not None:break
     raise Refused('live-group-RSS-unavailable')
    total=sum(z['rss_bytes'] for z in rows);largest=max(z['rss_bytes'] for z in rows);peak=max(peak,total);per=max(per,largest);samples+=1
    append_record(R/'rss-samples.jsonl.gz',{'epoch':time.time(),'name':name,'pgid':proc.pid,'group_rss_bytes':total,'largest_process_rss_bytes':largest})
    assert total<=limit and largest<=2147483648,'RSS-limit'
    controller=int(os.environ.get('P_CONTROLLER_PID',os.getpid()));allrows=N['ps_tree'](controller);assert allrows,'aggregate-RSS-NULL';assert sum(z['rss_bytes'] for z in allrows)<=3221225472 and max(z['rss_bytes'] for z in allrows)<=2147483648,'aggregate-or-checker-RSS-limit'
    assert os.statvfs(ROOT).f_bavail*os.statvfs(ROOT).f_frsize>20*1024**3;assert time.time()<deadline,'child-cutoff'
   except BaseException:
    missing+=1;violations.append('native-measurement-or-resource-failure');raise
   time.sleep(.1)
  thread.join(timeout=4);assert not thread.is_alive() and 'streams'in output and 'error'not in output,output.get('error','missing-output')
  proc.wait(timeout=2);assert not N['group_present'](proc.pid),'descendant-group-remains'
 except BaseException as exc:
  reason=repr(exc)
  if peak is None:missing+=1
 finally:
  try:
   if N['group_present'](proc.pid):N['kill_group'](proc.pid)
   proc.wait(timeout=3)
   if thread is not None:thread.join(timeout=3);assert not thread.is_alive(),'capture-thread-remains'
   if watch is not None:N['cancel'](watch)
   assert not N['group_present'](proc.pid),'group-not-empty-after-termination'
  except BaseException as exc:reason=(reason or '')+'; closure:'+repr(exc)
 out,err=output.get('streams',(b'',b''));guard={'pgid':proc.pid,'peak_group_rss_bytes':peak,'peak_process_rss_bytes':per,'samples':samples,'missing_samples':missing,'violations':violations,'reason':reason,'remaining_group':N['group_present'](proc.pid),'deadline':deadline,'native_ps':True,'independent_watchdog_parent_EOF':True,'raw_output_complete':'streams'in output}
 row={'name':name,'argv':list(map(str,argv)),'input':input.decode() if input else None,'exit':proc.returncode,'seconds':time.monotonic()-started,'stdout':out.decode(),'stderr':err.decode(),'guard':guard};append_record(R/'raw-process-events.jsonl.gz',row)
 if reason:raise Refused(reason)
 return subprocess.CompletedProcess(argv,proc.returncode,out,err),{'seconds':row['seconds'],'guard':guard}
def write_report(p,data):
 if isinstance(data,str):data=data.encode('utf-8')
 # Reformat JSON only; preserve every value including exact raw stdout/stderr strings.
 value=data if isinstance(data,(dict,list)) else json.loads(data);data=json.dumps(value,ensure_ascii=False,separators=(',',':')).encode('utf-8')+b'\n';p=Path(p);p.parent.mkdir(parents=True,exist_ok=True);tmp=p.with_name(p.name+'.writing')
 with lock:
  assert not tmp.exists(),'retained-partial-report';old=p.stat().st_size if p.exists() else 0;assert old+len(data)<=CFG['report_coexistence_bytes'],'plain-JSON-old-plus-temp16MiB';x=enforce();outside=census(R)['bytes']-x['fixtures']['bytes'];assert p.is_relative_to(F) or outside-old+len(data)+131072<=CFG['closing_returns_bytes'],'all-closing-returns-prewrite-bound';assert x['native']['bytes']+len(data)+CFG['incidental_bytes']<=CFG['whole_bytes'];assert x['fixtures']['bytes']+(len(data) if p.is_relative_to(F) else 0)<=CFG['fixture_bytes']
  with tmp.open('xb') as f:f.write(data);f.flush();os.fsync(f.fileno())
  os.replace(tmp,p)
 return len(data)
_original_text=Path.write_text;_original_bytes=Path.write_bytes

def bounded_text(self,data,*args,**kw):
 if self.is_relative_to(F) and self.name=='events.json':return write_report(self,data)
 if self.is_relative_to(F):
  with reserve(len(data.encode(kw.get('encoding')or'utf-8'))+65536,4,'write-text:'+self.name):return _original_text(self,data,*args,**kw)
 return _original_text(self,data,*args,**kw)
def bounded_bytes(self,data):
 if self.is_relative_to(F):
  with reserve(len(data)+65536,4,'write-bytes:'+self.name):return _original_bytes(self,data)
 return _original_bytes(self,data)
def install_io():
 Path.write_text=bounded_text;Path.write_bytes=bounded_bytes
 original_tree=shutil.copytree;original_file=shutil.copyfile
 def tree(src,dst,*a,**kw):
  x=census(src)
  with reserve(2*x['bytes']+65536,2*x['files']+32,'copytree',covered=True):return original_tree(src,dst,*a,**kw)
 def file(src,dst,*a,**kw):
  with reserve(Path(src).stat().st_size+65536,4,'copyfile',covered=True):return original_file(src,dst,*a,**kw)
 shutil.copytree=tree;shutil.copyfile=file
