"""External Linux process-group and aggregate RSS/resource enforcement."""
import os,sys,json,time,signal,subprocess,hashlib,shutil,platform
from pathlib import Path

BASE=Path(__file__).resolve().parent
E=BASE/'source/docs/research/file-native-incremental-engine-experiment-2026-10-18'
H=BASE/'source/docs/research/file-native-navigation-2026-10-19'
sys.path.insert(0,str(H))
from census import census
CUTOFF=1792453485 # 2026-10-19T23:44:45Z, immutable original closeout cutoff
os.environ['PATH']='/home/spenser/.cargo/bin:'+os.environ['PATH']
os.environ['CARGO_PROFILE_DEV_DEBUG']='0'
os.environ['CARGO_PROFILE_TEST_DEBUG']='0'
os.environ['CARGO_INCREMENTAL']='0'
STAGES=[]

def host_scope():
    rows=[]
    for p in sorted(Path('/home/spenser').glob('file-native*')):
        rows.append(census(p) if p.is_dir() else dict(root=str(p),files=1,logical_bytes=p.stat().st_size))
    for p in [Path('/home/spenser/.cargo/registry'),Path('/home/spenser/.cargo/git')]:
        if p.exists():rows.append(census(p))
    return dict(roots=rows,logical_bytes=sum(x['logical_bytes'] for x in rows),files=sum(x['files'] for x in rows),free_bytes=shutil.disk_usage(BASE).free)

def stage(name,args,seconds,cwd=None,checker=False):
    if time.time()>=CUTOFF:raise RuntimeError('original-execution-cutoff-censor')
    start=time.time();deadline=min(start+seconds,CUTOFF);peak=None;peak_single=None;samples=0;reason=None
    with (BASE/(name+'.stdout')).open('wb') as out,(BASE/(name+'.stderr')).open('wb') as err:
        p=subprocess.Popen(args,cwd=cwd,stdout=out,stderr=err,start_new_session=True)
        last=0
        while True:
            total=0;seen=0
            for entry in Path('/proc').iterdir():
                if not entry.name.isdigit():continue
                try:
                    stat=(entry/'stat').read_text().rsplit(')',1)[1].split()
                    if int(stat[2])!=p.pid:continue
                    status=(entry/'status').read_text().splitlines();rss=next((int(s.split()[1])*1024 for s in status if s.startswith('VmRSS:')),None)
                    if rss is not None:total+=rss;seen+=1;peak_single=max(peak_single or 0,rss)
                    limit=2*1024**3 if checker or int(entry.name)==p.pid else 3*1024**3
                    if rss is not None and rss>limit:reason='per-process-RSS-censor'
                except (OSError,ValueError,IndexError):pass
            if seen:samples+=1;peak=max(peak or 0,total)
            if total>3*1024**3:reason='aggregate-RSS-censor'
            if time.time()>deadline:reason='outer-time-censor'
            if time.time()-last>=2:
                owned=census(BASE);last=time.time()
                if owned['logical_bytes']>700*1024**2 or owned['files']>95000:reason='owned-capacity-censor'
                if shutil.disk_usage(BASE).free<20*1024**3:reason='free-reserve-censor'
            if reason:
                try:os.killpg(p.pid,signal.SIGKILL)
                except ProcessLookupError:pass
                p.wait();break
            if p.poll() is not None:break
            time.sleep(.04)
        # No detached or surviving group member is allowed to outlive the stage.
        try:os.killpg(p.pid,signal.SIGKILL)
        except ProcessLookupError:pass
    row=dict(stage=name,args=list(map(str,args)),cwd=str(cwd) if cwd else str(BASE),start_epoch=start,end_epoch=time.time(),seconds=time.time()-start,exit=p.returncode,censor=reason,peak_aggregate_rss=peak,peak_single_process_rss=peak_single,samples=samples,missing_measurement=peak is None,outer_cap_seconds=seconds,checker=checker,owned_after=census(BASE))
    STAGES.append(row);(BASE/'stages.json').write_text(json.dumps(STAGES,indent=2));print(json.dumps(row),flush=True)
    if reason or p.returncode:raise RuntimeError(row)
    return row

if __name__=='__main__':
    before=host_scope();(BASE/'host-before.json').write_text(json.dumps(before,indent=2));assert before['free_bytes']>=20*1024**3
    manifest=json.loads((BASE/'source-manifest.json').read_bytes())
    for path,row in manifest['files'].items():assert hashlib.sha256((BASE/'source'/path).read_bytes()).hexdigest()==row['sha256']
    (BASE/'environment.json').write_text(json.dumps(dict(platform=platform.platform(),python=sys.version,start_epoch=time.time(),cutoff_epoch=CUTOFF,source_verified=True,cargo=subprocess.check_output(['cargo','--version']).decode(),rustc=subprocess.check_output(['rustc','--version']).decode()),indent=2))
    stage('tests',['cargo','test','--locked','--offline','--workspace','-j2','--target-dir',str(BASE/'build')],120,E)
    stage('build',['cargo','build','--locked','--offline','--release','-j2','--target-dir',str(BASE/'build')],120,E)
    exe=BASE/'build/release/file-native-incremental'
    (BASE/'executable.json').write_text(json.dumps(dict(path=str(exe),bytes=exe.stat().st_size,sha256=hashlib.sha256(exe.read_bytes()).hexdigest(),source_manifest_sha256=hashlib.sha256((BASE/'source-manifest.json').read_bytes()).hexdigest()),indent=2))
    stage('focused',[sys.executable,str(H/'focused.py'),'--exe',str(exe),'--base',str(BASE/'focused-data'),'--capsule'],1800,checker=True)
    (BASE/'host-after-focused.json').write_text(json.dumps(host_scope(),indent=2))
