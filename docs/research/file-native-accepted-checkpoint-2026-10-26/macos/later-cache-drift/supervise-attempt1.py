import sys
sys.dont_write_bytecode=True
import os,json,time,signal,subprocess,shutil,traceback,hashlib
from pathlib import Path
HELPERS=Path('/private/tmp/file-native-online-macos-20261020/execution/readiness-20261021')
sys.path.insert(0,str(HELPERS))
from native_probe import identity,usage,group
from monitor_v20 import OwnedTree,valid_identity,same_birth
OUT=Path(__file__).resolve().parent
SRC=Path('/private/tmp/file-native-online-macos-20261020/source/final-engine')
mode=sys.argv[1]
argv=['/Users/spensermcconnell/.cargo/bin/cargo','test','--manifest-path',str(SRC/'Cargo.toml'),'--locked','--workspace','--release','-j2','--target-dir',str(OUT/'target')]
argv+= {'build':['--no-run'],'list':['--','--list'],'test':['--','--test-threads=1','--nocapture']}[mode]
updates={'CARGO_HOME':str(OUT/'cargo-home'),'CARGO_NET_OFFLINE':'true','CARGO_INCREMENTAL':'0','EXP_PACK_TEST_ROOT':str(OUT/'fixtures'),'TMPDIR':str(OUT/'tmp'),'TMP':str(OUT/'tmp'),'TEMP':str(OUT/'tmp'),'PYTHONDONTWRITEBYTECODE':'1'}
env=os.environ.copy();env.update(updates)
start=time.monotonic();result={'mode':mode,'argv':argv,'cwd':str(SRC),'environment_overrides':updates,'start_epoch':time.time(),'timeout_seconds':1800,'aggregate_rss_limit_bytes':3*1024**3,'test_process_rss_limit_bytes':2*1024**3,'free_floor_bytes':20*1024**3,'resource_claim':'native sampled functional supervision; not hard peak/40ms qualification','offline_reason':'Both registry/static HTTP probes failed DNS in confined runtime; exact local cache provision attempted. No alternative dependency selected.'}
(OUT/'logs'/f'{mode}-command.json').write_text(json.dumps(result,indent=2))
p=None;anchor=None;tree=None;failure=None;peak=0;maxone=0;sample_count=0;missing=[];signals=[];close=[];exit_code=None;lastdisk=0
# Read-only prerequisite before any workload; no synthetic child campaign.
me=identity(os.getpid());mu=usage(os.getpid());assert valid_identity(me) and mu['return']==0 and mu['rss_bytes']>0,(me,mu)
assert shutil.disk_usage(OUT).free>20*1024**3
stdout=(OUT/'logs'/f'{mode}.stdout').open('wb');stderr=(OUT/'logs'/f'{mode}.stderr').open('wb');samples=(OUT/'logs'/f'{mode}-samples.jsonl').open('w')
def exited_without_reaping():
 return os.waitid(os.P_PID,p.pid,os.WEXITED|os.WNOHANG|os.WNOWAIT) is not None

def signal_owned(sig):
 # Unreaped original direct child pins its process-group identity until wait.
 row=identity(p.pid)
 if valid_identity(row) and (anchor is None or same_birth(anchor,row)) and row['pgid']==p.pid:
  try:os.killpg(p.pid,sig);signals.append({'pgid':p.pid,'signal':sig,'monotonic':time.monotonic()})
  except ProcessLookupError:pass
 # Track/recheck escaped descendants by original birth; never signal strangers.
 if tree:
  for pid,expected in list(tree.known.items()):
   if pid==p.pid:continue
   row=identity(pid)
   if valid_identity(row) and same_birth(expected,row) and row['pgid']!=p.pid:
    try:os.kill(pid,sig);signals.append({'pid':pid,'signal':sig})
    except ProcessLookupError:pass
try:
 p=subprocess.Popen(argv,cwd=SRC,env=env,stdout=stdout,stderr=stderr,start_new_session=True)
 # Persist raw handle identity immediately, even before libproc enrollment.
 result['child_pid']=p.pid;result['pgid']=os.getpgid(p.pid)
 (OUT/'logs'/f'{mode}-child.json').write_text(json.dumps({'pid':p.pid,'pgid':result['pgid'],'parent':os.getpid(),'launched_epoch':time.time()},indent=2))
 anchor=identity(p.pid);result['original_identity']=anchor
 (OUT/'logs'/f'{mode}-child.json').write_text(json.dumps({'pid':p.pid,'pgid':result['pgid'],'parent':os.getpid(),'identity':anchor},indent=2))
 if not valid_identity(anchor) or anchor['ppid']!=os.getpid() or anchor['pgid']!=p.pid:raise RuntimeError('original-child-enrollment-failed')
 tree=OwnedTree(anchor)
 while not exited_without_reaping():
  now=time.monotonic();observation=tree.observe();rows=[]
  for row in observation['identities']:
   if row.get('state')==5:continue
   u=usage(row['pid']);again=identity(row['pid'])
   if u['return']==0 and valid_identity(again) and same_birth(row,again):rows.append({'identity':row,'usage':u})
   else:missing.append({'identity':row,'usage':u,'after':again})
  total=sum(x['usage']['rss_bytes'] for x in rows)+usage(os.getpid())['rss_bytes'];one=max([x['usage']['rss_bytes'] for x in rows] or [0]);peak=max(peak,total);maxone=max(maxone,one);sample_count+=1
  samples.write(json.dumps({'elapsed':now-start,'aggregate_with_supervisor':total,'processes':rows})+'\n');samples.flush()
  if total>3*1024**3:raise RuntimeError('aggregate-RSS-limit')
  if mode=='test' and one>2*1024**3:raise RuntimeError('test-process-RSS-limit')
  if now-start>1800:raise RuntimeError('finite-timeout')
  if now-lastdisk>2:
   if shutil.disk_usage(OUT).free<20*1024**3:raise RuntimeError('free-space-floor')
   lastdisk=now
  time.sleep(.05)
except BaseException as exc:
 failure={'type':type(exc).__name__,'message':str(exc),'traceback':traceback.format_exc()}
finally:
 if p is not None:
  try:
   if not exited_without_reaping():
    signal_owned(signal.SIGTERM);until=time.monotonic()+3
    while time.monotonic()<until and not exited_without_reaping():time.sleep(.05)
    if not exited_without_reaping():signal_owned(signal.SIGKILL)
   # Root stays unreaped while descendants are reconciled, preventing PID reuse.
   g=group(p.pid);result['group_before_reap']=g
   if not g['valid']:raise RuntimeError('group-closure-query-unknown')
   if any(pid!=p.pid for pid in g['members']):
    signal_owned(signal.SIGTERM);time.sleep(.2);signal_owned(signal.SIGKILL)
   if tree:
    for pid,expected in list(tree.known.items()):
     if pid==p.pid:continue
     row=identity(pid)
     if valid_identity(row) and same_birth(expected,row) and row.get('state')!=5:signal_owned(signal.SIGKILL);break
   exit_code=p.wait(timeout=5)
   for _ in range(2):close.append(group(p.pid));time.sleep(.05)
   result['escaped_survivors']=[]
   if tree:
    for pid,expected in tree.known.items():
     row=identity(pid)
     if valid_identity(row) and same_birth(expected,row):result['escaped_survivors'].append(row)
  except BaseException as exc:
   result['cleanup_error']={'error':repr(exc),'traceback':traceback.format_exc()}
   try:signal_owned(signal.SIGKILL);exit_code=p.wait(timeout=5)
   except BaseException as final:result['last_wait_error']=repr(final)
 stdout.close();stderr.close();samples.close()
 result.update(exit=exit_code,duration_seconds=time.monotonic()-start,exception=failure,sampled_aggregate_peak_bytes=peak,sampled_max_process_bytes=maxone,sample_count=sample_count,missing_rss=missing,signals=signals,closure_observations=close,owned_tree_closed=bool(len(close)==2 and all(x['valid'] and x['members']==[] for x in close) and not result.get('escaped_survivors') and 'cleanup_error' not in result))
 result['helper_source_hashes']={str(HELPERS/name):hashlib.sha256((HELPERS/name).read_bytes()).hexdigest() for name in ['native_probe.py','monitor_v20.py']}
 (OUT/'logs'/f'{mode}-result.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
sys.exit(0 if exit_code==0 and not failure and result['owned_tree_closed'] else 1)
