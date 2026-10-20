import os,sys,time,json,signal,subprocess,select,datetime
from pathlib import Path
R=Path('/private/tmp/file-native-p-macos-20261020/cargo-private-proposed/.p-build-receipts-v5')
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def group(pgid):
 p=subprocess.Popen(['/bin/ps','-axo','pid,ppid,pgid,rss,command'],stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
 try:out,err=p.communicate(timeout=1)
 except subprocess.TimeoutExpired:p.kill();p.communicate(timeout=1);raise
 assert p.returncode==0
 rows=[]
 for line in out.splitlines()[1:]:
  z=line.split(None,4)
  if len(z)<5:continue
  try:pid,ppid,g,rss=map(int,z[:4])
  except ValueError:continue
  if g==pgid and pid!=p.pid:rows.append(dict(pid=pid,ppid=ppid,pgid=g,rss_bytes=rss*1024,command=z[4]))
 return {'utc':now(),'excluded_probe_PID':p.pid,'rows':rows}
def safe_signal(pgid,sig,events):
 try:os.killpg(pgid,sig);events.append({'utc':now(),'pgid':pgid,'signal':int(sig),'result':'sent'});return True
 except ProcessLookupError:events.append({'utc':now(),'pgid':pgid,'signal':int(sig),'result':'ESRCH'});return True
 except PermissionError as e:
  snapshot=group(pgid);events.append({'utc':now(),'pgid':pgid,'signal':int(sig),'errno':e.errno,'error':str(e),'native_snapshot':snapshot,'empty_group_confirmed':not snapshot['rows']})
  return not snapshot['rows']
def close_group(pgid,events):
 if not safe_signal(pgid,signal.SIGTERM,events):return False
 time.sleep(.8)
 return safe_signal(pgid,signal.SIGKILL,events)
def main():
 start=time.time();events=[];snapshots=[]
 child=subprocess.Popen([sys.executable,'-B',__file__,'--child'],stdin=subprocess.PIPE,stdout=subprocess.PIPE,start_new_session=True)
 rd,wr=os.pipe()
 watch=subprocess.Popen([sys.executable,'-B',__file__,'--watchdog',str(child.pid),str(start+8),str(rd)],pass_fds=(rd,),start_new_session=True);os.close(rd)
 # Dedicated independent timer armed before any ps/census/input measurement.
 child.stdin.write(b'G');child.stdin.flush();child.stdin.close()
 assert os.statvfs(R).f_bavail*os.statvfs(R).f_frsize>20*1024**3
 raw=b'';ready=[];deadline=start+2
 while time.time()<deadline and len(ready)<2:
  if select.select([child.stdout],[],[],.1)[0]:
   raw+=os.read(child.stdout.fileno(),4096)
   ready=[json.loads(x) for x in raw.splitlines() if x.strip()]
 assert len(ready)==2
 snap=group(child.pid);snapshots.append(snap);ids={z['pid'] for z in ready}
 assert len(snap['rows'])==2 and {z['pid'] for z in snap['rows']}==ids
 assert all(z['pgid']==child.pid for z in snap['rows'])
 assert sum(z['rss_bytes'] for z in snap['rows'])<3*1024**3 and max(z['rss_bytes'] for z in snap['rows'])<2*1024**3
 success=close_group(child.pid,events);child.wait(timeout=1)
 time.sleep(.1);after=group(child.pid);snapshots.append(after)
 # Observe post-exit group-zero behavior; EPERM is never itself treated as closure.
 zero=safe_signal(child.pid,0,events)
 empty=not after['rows']
 if empty:os.write(wr,b'X')
 os.close(wr)
 watch.wait(timeout=max(.1,start+10-time.time()))
 final=group(child.pid);snapshots.append(final)
 result={'utc':now(),'start_epoch':start,'whole_elapsed_seconds':time.time()-start,'dummy_PGID':child.pid,'dummy_ready':ready,'events':events,'snapshots':snapshots,'live_group_shutdown_test_pass':success and empty and not final['rows'],'parent_child_exit':child.returncode,'watchdog_exit':watch.returncode,'future_build':'HOLD pending parent review','Cargo_compiler_engine_invoked':False,'method':'normal approved same-user signals only; no alternate privilege route','independent_watchdog_armed_before_measurements':True}
 with (R/'guard-dummy-result.json').open('x') as f:json.dump(result,f,indent=2);f.write('\n')
 print(json.dumps(result,indent=2))
if __name__=='__main__':
 if '--grandchild' in sys.argv:
  signal.signal(signal.SIGTERM,signal.SIG_IGN);print(json.dumps({'role':'TERM-ignoring-grandchild','pid':os.getpid(),'pgid':os.getpgrp()}),flush=True);time.sleep(7);os._exit(0)
 elif '--child' in sys.argv:
  assert sys.stdin.buffer.read(1)==b'G';print(json.dumps({'role':'child','pid':os.getpid(),'pgid':os.getpgrp()}),flush=True)
  p=subprocess.Popen([sys.executable,'-B',__file__,'--grandchild']);p.wait(timeout=8)
 elif '--watchdog' in sys.argv:
  pgid=int(sys.argv[2]);deadline=float(sys.argv[3]);fd=int(sys.argv[4]);events=[];reason=None
  while time.time()<deadline:
   if select.select([fd],[],[],min(.1,max(0,deadline-time.time())))[0]:
    b=os.read(fd,1)
    if b==b'X':reason='cancelled after observed empty group';break
    reason='supervisor pipe EOF';break
  if reason!='cancelled after observed empty group':close_group(pgid,events)
  with (R/'guard-dummy-watchdog.json').open('x') as f:json.dump({'utc':now(),'reason':reason or 'hard deadline','events':events},f,indent=2);f.write('\n')
 else:main()
