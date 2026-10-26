"""Fresh locked release build followed by exactly one existing real pilot."""
import os,sys,json,hashlib,importlib.util,datetime,traceback
from pathlib import Path
sys.dont_write_bytecode=True
ROOT=Path(__file__).resolve().parent
spec=importlib.util.spec_from_file_location('native_supervisor','/tmp/file-native-host-inventory-linux-20261024-01a16fd3/run-final062-resumed.py');s=importlib.util.module_from_spec(spec);spec.loader.exec_module(s);s.OUT=ROOT
source=s.SOURCE;pre=json.loads((ROOT/'preflight.json').read_text())
def write(name,value):(ROOT/name).write_text(json.dumps(value,indent=2)+'\n')
def verify():
    return all((source/r['path']).stat().st_size==r['bytes'] and s.digest(source/r['path'])==r['sha256'] for r in pre['files'])
env=os.environ.copy()
removed=[k for k in env if k.startswith('EXP_') or k in ['RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','RUST_TEST_THREADS','RUST_TEST_FILTER']]
for k in removed:env.pop(k)
env.update(TMPDIR=str(ROOT/'tmp'),TMP=str(ROOT/'tmp'),TEMP=str(ROOT/'tmp'),CARGO_INCREMENTAL='0',CARGO_TERM_COLOR='never',PYTHONDONTWRITEBYTECODE='1')
result={'status':'STARTED','started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_identity':pre['source_identity'],'reviewed_manifest_sha256':pre['reviewed_manifest_sha256'],'removed_ambient_environment_names':removed,'stages':[],'attempts_allowed':1,'pilot_attempts':0,'engine_source_modified':False,'full_matrix_released':False,'limits':{'native_floor_bytes':s.FLOOR,'aggregate_rss_bytes':s.AGG_RSS,'per_process_rss_bytes':s.PROCESS_RSS,'task_allocated_bytes':s.OWNED_BYTES,'task_files':s.OWNED_FILES,'old_reader_rss_bytes':256*1024**2,'engine_call_max':37,'compile_timeout_seconds':600,'pilot_outer_timeout_seconds':480,'pilot_functional_seconds':300}}
try:
    assert verify(),'reviewed-source-mismatch'
    assert s.free_bytes()>=s.FLOOR+1024**3,'insufficient-free-plus-margin'
    cmd=['cargo','build','--manifest-path',str(source/'Cargo.toml'),'--locked','--offline','--release','-j2','--bin','file-native-incremental','--target-dir',str(ROOT/'target')]
    write('build-command.json',{'argv':cmd,'cwd':str(source),'environment':{k:env[k] for k in ['TMPDIR','TMP','TEMP','CARGO_INCREMENTAL','CARGO_TERM_COLOR']}})
    build=s.stage('build-cli',cmd,600,env);result['stages'].append(build)
    assert build['exit']==0 and not build['termination_reason'] and build['process_closure_verified'],'build-failed'
    assert verify(),'source-changed-during-build'
    binary=ROOT/'target/release/file-native-incremental';binding={'source_identity':pre['source_identity'],'reviewed_manifest_sha256':pre['reviewed_manifest_sha256'],'source_files_verified':26,'binary':str(binary),'binary_sha256':s.digest(binary),'binary_bytes':binary.stat().st_size,'source_correspondence_after_build':True,'stage':build};write('cli-build-binding.json',binding);result['binary_sha256']=binding['binary_sha256']
    assert not (ROOT/'pilot.started.json').exists(),'pilot-already-started'
    assert s.free_bytes()>=s.FLOOR+1024**3,'insufficient-pilot-margin'
    result['pilot_attempts']=1;write('run-status.json',result)
    pilot=s.stage('pilot-observed',[sys.executable,'-B',str(ROOT/'pilot_observed.py')],480,env);result['stages'].append(pilot)
    answer=json.loads((ROOT/'pilot-result.json').read_text());result['pilot_status']=answer['status'];result['engine_calls']=answer['engine_calls'];result['independent_checker_processes']=answer['independent_checker_processes']
    result['status']='PASS_FUNCTIONAL' if answer['status']=='PASS' and pilot['exit']==0 and not pilot['termination_reason'] and pilot['process_closure_verified'] else 'FAIL_PILOT'
except BaseException as e:
    result['status']='FAIL_OR_BLOCKED';result['error']=repr(e);(ROOT/'run-exception.txt').write_text(traceback.format_exc())
finally:
    result['source_matches_reviewed_after']=verify();result['protected_after']={name:s.digest(Path(name)) for name in pre['protected_before']};result['protected_record_hashes_unchanged']=result['protected_after']==pre['protected_before'];result['native_free_after']=s.free_bytes();result['owned_after']=s.census();result['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat();write('run-result.json',result);print(json.dumps({'status':result['status'],'engine_calls':result.get('engine_calls'),'source_matches':result['source_matches_reviewed_after'],'protected_records_unchanged':result['protected_record_hashes_unchanged']}),flush=True)
