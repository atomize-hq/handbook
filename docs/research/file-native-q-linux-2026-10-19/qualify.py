"""Q Linux external scheduling only; pinned engine/harness bytes stay unchanged."""
import os,sys,time,json,hashlib,pathlib,subprocess,shutil,signal,tarfile
R=pathlib.Path(__file__).resolve().parent
E=R/'source/docs/research/file-native-incremental-engine-experiment-2026-10-18'
F=R/'source/docs/research/file-native-recovery-reclamation-2026-10-19'
STOP=1792449439.0 # 22:37:19 UTC: original deadline minus90s
rows=[]
os.environ.update(PATH=str(pathlib.Path.home()/'.cargo/bin')+':'+os.environ.get('PATH',''),CARGO_TARGET_DIR=str(R/'build'),CARGO_INCREMENTAL='0',CARGO_PROFILE_DEV_DEBUG='0',CARGO_PROFILE_TEST_DEBUG='0',PYTHONDONTWRITEBYTECODE='1')
def census():
    n=b=0
    for d,ds,fs in os.walk(R):
        for name in fs:
            try:s=os.stat(os.path.join(d,name));n+=1;b+=s.st_size
            except FileNotFoundError:pass
    return dict(files=n,logical_bytes=b,free_bytes=shutil.disk_usage(R).free)
def stage(name,args,cap,cwd=None):
    now=time.time();limit=min(cap,STOP-now)
    if limit<=1:
        rows.append(dict(name=name,status='censored',reason='original-deadline-admission'));return False
    c=census()
    if c['logical_bytes']>512*1024**2 or c['files']>100000 or c['free_bytes']<20*1024**3:raise RuntimeError('resource-admission:'+str(c))
    peak=0;reason=None
    with (R/(name+'.stdout')).open('wb') as out,(R/(name+'.stderr')).open('wb') as err:
        p=subprocess.Popen([str(x) for x in args],stdout=out,stderr=err,cwd=cwd,start_new_session=True)
        while p.poll() is None:
            ps=subprocess.check_output(['ps','-eo','pid=,ppid=,rss='],text=True);data=[list(map(int,s.split())) for s in ps.splitlines()];ids={p.pid}
            for _ in range(5):ids|={pid for pid,ppid,rss in data if ppid in ids}
            rss=sum(rss*1024 for pid,ppid,rss in data if pid in ids);peak=max(peak,rss)
            if rss>3*1024**3:reason='aggregate-RSS-cap'
            if time.time()-now>=limit:reason='time-cap-original-deadline' if limit<cap else 'stage-time-cap'
            if reason:os.killpg(p.pid,signal.SIGKILL);p.wait();break
            time.sleep(.1)
    row=dict(name=name,args=[str(x) for x in args],start_epoch=now,seconds=time.time()-now,exit=p.returncode,peak_aggregate_sampled_rss=peak,status='censored' if reason else ('success' if p.returncode==0 else 'failure'),reason=reason);rows.append(row)
    (R/'stages.json').write_text(json.dumps(rows,indent=2));return row['status']=='success'
try:
    binding=json.loads((F/'evidence/final-code-build-binding.json').read_bytes());verified={}
    for field,folder in [('rust_sha256',E/'src'),('cargo_sha256',E)]:
        for name,h in binding[field].items():
            actual=hashlib.sha256((folder/name).read_bytes()).hexdigest();assert actual==h,(name,actual,h);verified[name]=actual
    (R/'binding.json').write_text(json.dumps(dict(checkout='f0904a63cb2500b9bea3a4034d19cbe38bfac729',core=binding['core_source_commit'],files=verified),indent=2))
    (R/'inventory.json').write_text(json.dumps(dict(platform=__import__('platform').platform(),filesystem=subprocess.check_output(['findmnt','-T',str(R),'-o','FSTYPE,OPTIONS','-n'],text=True),rust=subprocess.check_output(['rustc','-Vv'],text=True),cargo=subprocess.check_output(['cargo','-V'],text=True),initial=census()),indent=2))
    assert stage('build',['cargo','build','--locked','--offline','--workspace','-j2'],120,E)
    exe=R/'build/debug/file-native-incremental';b=json.loads((R/'binding.json').read_bytes());b['executable_sha256']=hashlib.sha256(exe.read_bytes()).hexdigest();(R/'binding.json').write_text(json.dumps(b,indent=2))
    stage('tests',['cargo','test','--locked','--offline','--workspace','-j2'],120,E)
    stage('capsule',[sys.executable,R/'bounded_capsule.py',str(exe),R/'capsule-data'],1800)
    stage('extra',[sys.executable,F/'focused_extra.py','--exe',exe,'--base',R/'extra-data'],120)
    stage('legacy',[sys.executable,F/'legacy_compatibility.py','--exe',exe,'--fixtures',pathlib.Path.home()/'file-native-recovery-20261019/legacy-fixtures','--base',R/'legacy-data'],120)
    stage('supplement',[sys.executable,R/'supplement.py',str(exe)],120)
finally:
    (R/'closeout.json').write_text(json.dumps(dict(stages=rows,census=census(),original_deadline_utc='2026-10-19T22:38:49Z',stop_epoch=STOP,epoch=time.time()),indent=2))
