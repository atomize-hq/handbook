"""Exact final-pin native build, focused capsule and README upgrade smoke."""
import os,sys,time,json,signal,subprocess,hashlib,shutil,resource
from pathlib import Path
BASE=Path(__file__).resolve().parent
OWNED=BASE.parent
E=BASE/'source/docs/research/file-native-incremental-engine-experiment-2026-10-18'
H=BASE/'source/docs/research/file-native-navigation-2026-10-19'
sys.path.insert(0,str(H))
from census import census
CUTOFF=1792453485
os.environ['PATH']='/home/spenser/.cargo/bin:'+os.environ['PATH']
os.environ['CARGO_PROFILE_DEV_DEBUG']='0';os.environ['CARGO_PROFILE_TEST_DEBUG']='0';os.environ['CARGO_INCREMENTAL']='0'
os.environ['TMPDIR']=str(BASE/'tmp');(BASE/'tmp').mkdir(exist_ok=False)
stages=[]

def run_stage(name,args,cap,cwd=None):
    start=time.time();deadline=min(start+cap,CUTOFF);reason=None;peak=None;single=None;samples=0;peak_bytes=peak_files=0;minimum_free=None
    with (BASE/(name+'.stdout')).open('wb') as out,(BASE/(name+'.stderr')).open('wb') as err:
        p=subprocess.Popen(args,cwd=cwd,stdout=out,stderr=err,start_new_session=True)
        last=0
        while True:
            measurements=[]
            for path in Path('/proc').iterdir():
                if not path.name.isdigit():continue
                try:
                    stat=(path/'stat').read_text().rsplit(')',1)[1].split()
                    if int(stat[2])!=p.pid:continue
                    rss=next((int(x.split()[1])*1024 for x in (path/'status').read_text().splitlines() if x.startswith('VmRSS:')),None)
                    if rss is not None:measurements.append(rss)
                except (OSError,ValueError,IndexError):pass
            if measurements:
                samples+=1;single=max(single or 0,max(measurements));peak=max(peak or 0,sum(measurements)+resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*1024)
                if max(measurements)>2*1024**3:reason='process-RSS-censor'
                if sum(measurements)+resource.getrusage(resource.RUSAGE_SELF).ru_maxrss*1024>3*1024**3:reason='aggregate-RSS-censor'
            if time.time()>=deadline:reason='original-cutoff-or-stage-cap-censor'
            if time.time()-last>=.5:
                current=census(OWNED);peak_bytes=max(peak_bytes,current['logical_bytes']);peak_files=max(peak_files,current['files']);free=shutil.disk_usage(BASE).free;minimum_free=min(minimum_free or free,free);last=time.time()
                if current['logical_bytes']>700*1024**2 or current['files']>95000:reason='combined-native-capacity-censor'
                if free<20*1024**3:reason='free-reserve-censor'
            if reason:
                try:os.killpg(p.pid,signal.SIGKILL)
                except ProcessLookupError:pass
                p.wait();break
            if p.poll() is not None:break
            time.sleep(.04)
        try:os.killpg(p.pid,signal.SIGKILL)
        except ProcessLookupError:pass
    row=dict(stage=name,args=list(map(str,args)),start_epoch=start,end_epoch=time.time(),seconds=time.time()-start,exit=p.returncode,censor=reason,cap_seconds=cap,effective_deadline_epoch=deadline,peak_aggregate_rss_including_supervisor=peak,peak_single_process_rss=single,samples=samples,missing_measurement=peak is None,peak_combined_native_logical_bytes=peak_bytes,peak_combined_native_files=peak_files,minimum_free_bytes=minimum_free,owned_after=census(OWNED))
    stages.append(row);(BASE/'stages.json').write_text(json.dumps(stages,indent=2));print(json.dumps(row),flush=True)
    assert reason is None and p.returncode==0,row

assert time.time()<CUTOFF
before=census(OWNED)
admission=dict(epoch=time.time(),current_combined_native=before,new_source_build_fixtures_return_forecast_bytes=320*1024**2,local_and_git_growth_allowance_bytes=64*1024**2,worst_case_additional_lane_bytes=before['logical_bytes']+384*1024**2,lane_limit_bytes=768*1024**2,new_file_forecast=before['files']+25000,free_bytes=shutil.disk_usage(BASE).free,original_cutoff_epoch=CUTOFF,original_deadline_epoch=1792454085)
(BASE/'admission.json').write_text(json.dumps(admission,indent=2));assert admission['worst_case_additional_lane_bytes']<768*1024**2 and admission['new_file_forecast']<100000 and admission['free_bytes']>=20*1024**3
manifest=json.loads((BASE/'source-manifest.json').read_bytes())
for path,row in manifest['files'].items():assert hashlib.sha256((BASE/'source'/path).read_bytes()).hexdigest()==row['sha256']
prior=OWNED/'build/release/file-native-incremental';prior_hash=hashlib.sha256(prior.read_bytes()).hexdigest();assert prior_hash=='5295ce01770f9c4ba8b88bd4848f745c23c327ca78d6cd1a80770c2e1b69e89e'
(BASE/'environment.json').write_text(json.dumps(dict(start_epoch=time.time(),compiler=subprocess.check_output(['rustc','--version']).decode(),cargo=subprocess.check_output(['cargo','--version']).decode(),python=sys.version,source_verified=True,preserved_6e66_binary_sha256=prior_hash,supervisor_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()),indent=2))
run_stage('tests',['cargo','test','--locked','--offline','--workspace','-j2','--target-dir',str(BASE/'build')],120,E)
run_stage('build',['cargo','build','--locked','--offline','--release','-j2','--target-dir',str(BASE/'build')],120,E)
exe=BASE/'build/release/file-native-incremental'
(BASE/'executable.json').write_text(json.dumps(dict(candidate=manifest['checkout'],path=str(exe),bytes=exe.stat().st_size,sha256=hashlib.sha256(exe.read_bytes()).hexdigest(),source_manifest_sha256=hashlib.sha256((BASE/'source-manifest.json').read_bytes()).hexdigest(),core_Rust_Cargo_files=manifest['core_Rust_Cargo_files']),indent=2))
run_stage('focused',[sys.executable,str(H/'focused.py'),'--exe',str(exe),'--base',str(BASE/'focused-data'),'--capsule'],1800)
run_stage('readme-smoke',[sys.executable,str(BASE/'readme_smoke.py')],120)
assert hashlib.sha256(prior.read_bytes()).hexdigest()==prior_hash
(BASE/'completed.json').write_text(json.dumps(dict(candidate=manifest['checkout'],status='success',completed_epoch=time.time(),original_cutoff_epoch=CUTOFF,original_deadline_epoch=1792454085,combined_owned=census(OWNED),final_executable_sha256=hashlib.sha256(exe.read_bytes()).hexdigest(),preserved_6e66_executable_sha256=prior_hash,full_focused=json.loads((BASE/'focused-data/summary.json').read_bytes()),readme_smoke=json.loads((BASE/'readme-data/summary.json').read_bytes())),indent=2))
