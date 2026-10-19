import subprocess,time,os,json,sys,signal,pathlib
r=pathlib.Path(__file__).parent;name=sys.argv[1];cap=float(sys.argv[2]);args=sys.argv[3:];started=time.time();cap=min(cap,max(0,1792449551-started-90));out=r/'evidence'/name
if cap<=0:raise SystemExit('original-deadline-closeout')
env=dict(os.environ,CARGO_TARGET_DIR=str(r/'build'),CARGO_BUILD_JOBS='2',CARGO_INCREMENTAL='0',CARGO_PROFILE_DEV_DEBUG='0',CARGO_PROFILE_TEST_DEBUG='0',TMPDIR=str(r/'tmp'),PYTHONDONTWRITEBYTECODE='1')
censor=None;peak=0;samples=0
with out.with_suffix('.stdout').open('wb') as o,out.with_suffix('.stderr').open('wb') as e:
 p=subprocess.Popen(args,stdout=o,stderr=e,env=env,start_new_session=True)
 while p.poll() is None:
  q=subprocess.run(['/bin/ps','-axo','pid=,ppid=,rss='],capture_output=True,text=True,timeout=3)
  if q.returncode:raise RuntimeError('native-rss-sampler-unavailable:'+q.stderr)
  procs=[list(map(int,line.split())) for line in q.stdout.splitlines()];owned={p.pid};changed=True
  while changed:
   before=len(owned);owned.update(a for a,b,c in procs if b in owned);changed=len(owned)!=before
  mem=sum(c*1024 for a,b,c in procs if a in owned);peak=max(peak,mem);samples+=1
  if mem>3221225472:censor='aggregate-rss-cap'
  if any(c*1024>2147483648 for a,b,c in procs if a in owned):censor='process-rss-cap'
  if time.time()-started>=cap:censor='time-cap'
  if censor:os.killpg(p.pid,signal.SIGKILL);p.wait();break
  time.sleep(.05)
row=dict(args=args,start_epoch=started,seconds=time.time()-started,cap=cap,exit=p.returncode,status='censored' if censor else ('success' if p.returncode==0 else 'failure'),censor=censor,sampled_aggregate_peak_bytes=peak,samples=samples,sampler='native ps rss KiB times1024, descendant closure, 50ms interval')
out.with_suffix('.json').write_text(json.dumps(row,indent=2));print(json.dumps(row))
