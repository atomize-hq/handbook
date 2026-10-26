import contextlib,copy,hashlib,json,os,pathlib,shutil,sys,threading,time,traceback
O=pathlib.Path(__file__).resolve().parent
sys.path.insert(0,str(O))
import build_cli as build
from native_runtime import Runtime,need,save,sha,GIB
import focused_protocol as sdk
import pilot_behavior as behavior
import oracle_worker as independent
PYTHON='C:/Users/spmcc/.pyenv/pyenv-win/versions/3.11.9/python.exe'
BASELINE='0628322e4b48e0efad19cdc32238a622d4aaa7a8'
IDENTITY='baseline0628322e plus ONLY telemetry fixture n1-to10; io SHA836c8a6ceac0f1e972bf77ab3e54be30333f28e763ff1d0ec2f1bff3b34a599d'
ROOT=O/'disposable/store'
class Driver:
    def __init__(self,runtime,engine):self.r=runtime;self.engine=engine;self.binary_sha256=sha(engine)
    def call(self,root,action,*,timeout,code=0,env=None,on_started=None,**fields):
        need(sha(self.engine)==self.binary_sha256,'engine-binary-changed')
        limit=256*1024**2 if env and env.get('EXP_BARRIER')=='pack-catalog-captured' else 2*GIB
        raw=json.dumps(dict(root=str(root),action=action,**fields),separators=(',',':')).encode()
        out,err,row=self.r.run([str(self.engine)],raw,label='engine-'+action,limit=limit,timeout=timeout,env_extra=env,on_started=on_started,engine=True)
        result=sdk.strict(out,4*1024**2);need(result.get('status')=='ok','engine-semantic-error:'+repr(result))
        return result
class Adapter(sdk.ThreadCalls):
    def __init__(self,r):self.r=r;self.controls=[];self.callbacks=[];self.probes=0;self.capabilities=set(behavior.REQUIRED_CAPABILITIES) if hasattr(behavior,'REQUIRED_CAPABILITIES') else {'barrier:pack-catalog-captured','barrier:physical-before-selection','barrier:candidate-ready'}
    def wall_time(self):return time.time()
    def monotonic(self):return time.monotonic()
    def sleep(self,seconds):
        end=time.monotonic()+seconds
        while time.monotonic()<end:need(not self.r.violation,self.r.violation or 'monitor');time.sleep(min(.05,max(0,end-time.monotonic())))
    def before_call(self,root,action,role,seconds):need(root==ROOT and seconds>0 and self.r.engine_calls<37,'actual-owned-root-call-bound')
    def read(self,root,name,limit):
        p=root/name;need(p.is_relative_to(ROOT),'read-outside-store');raw=p.read_bytes();need(len(raw)<=limit,'read-bound');return raw
    def exists(self,root,name):return (root/name).exists()
    def inventory(self,root):return independent.inventory(root)
    def control(self,root,name):
        d=O/'controls'/name;d.mkdir(parents=True,exist_ok=False)
        value=dict(name=name,ready=str(d/'ready'),release=str(d/'release'))
        self.controls.append(value);return value
    def spawn(self,callback):
        h=super().spawn(callback);self.callbacks.append(h);return h
    def await_ready(self,control,handle,timeout):
        phases={'readiness-old':'pack-catalog-captured','readiness-maintenance':'physical-before-selection','readiness-publisher':'candidate-ready'}
        end=time.monotonic()+timeout
        while not pathlib.Path(control['ready']).exists():
            need(self.alive(handle),'barrier-process-ended:'+str(handle.get('error')))
            need(time.monotonic()<end,'real-barrier-ready-timeout');need(not self.r.violation,self.r.violation or 'monitor');time.sleep(.005)
        need(pathlib.Path(control['ready']).read_text()==phases[control['name']],'real-ready-phase')
        control['ready_ns']=time.perf_counter_ns();self.r.emit(dict(kind='actual-barrier-ready',control=copy.deepcopy(control)))
    def release(self,control):
        p=pathlib.Path(control['release']);first=not p.exists()
        if first:
            with p.open('xb') as f:f.write(b'release');f.flush();os.fsync(f.fileno())
            control['release_ns']=time.perf_counter_ns()
        row=dict(kind='actual-barrier-release',first_release=first,control=copy.deepcopy(control))
        if control.get('ready_ns') and control.get('release_ns'):row['held_ready_to_release_seconds']=(control['release_ns']-control['ready_ns'])/1e9
        self.r.emit(row)
    def quiesce(self,root):
        with self.r.state_lock:need(not any(x['engine'] for x in self.r.active.values()),'independent-probe-requires-quiescent-engine')
    def probe(self,mode,root,**fields):
        self.quiesce(root);self.probes+=1;need(self.probes<=4,'pilot-independent-probe-bound4')
        output=O/f'oracle-{self.probes}-{mode}.json'
        config=dict(mode=mode,root=str(root),output=str(output),baseline_pin=BASELINE,corrected_source_identity=IDENTITY,**fields)
        out,err,row=self.r.run([PYTHON,'-I','-S','-B',str(O/'oracle_worker.py')],json.dumps(config).encode(),label='independent-'+mode,timeout=90)
        proof=json.loads(out);save(f'oracle-{self.probes}-{mode}-response.json',proof);return proof
    def mapping(self,root,key):return self.probe('mapping',root,key=key)
    def backlog(self,root):return self.probe('backlog',root)
    def close_case(self,case_id,root):
        errors=[]
        for control in self.controls:
            try:self.release(control)
            except BaseException as e:errors.append(repr(e))
        for h in self.callbacks:
            try:self.wait(h,5)
            except BaseException as e:errors.append(repr(e))
        with self.r.state_lock:active=copy.deepcopy(self.r.active)
        if active:build.terminate_job(self.r.job,125)
        return dict(processes_closed=not active,handles_closed=all(x.get('handles_closed') for x in self.r.records),python_callbacks_joined=all(not self.alive(h) for h in self.callbacks),callback_errors=errors)
class Harness(sdk.Harness):
    @contextlib.contextmanager
    def case(self,case_id,root,forecast):
        need(not root.exists() and root==ROOT,'fresh-owned-NTFS-store-required')
        self.active.add(root);self.ledgers[root]=sdk.Inputs()
        self.note('task-pilot-case-start',case_id=case_id,root=str(root),source_identity=IDENTITY,actual_binary_sha=self.driver.binary_sha256,authority='human-authorized real pilot; no old guard/profile qualification asserted')
        try:yield
        finally:
            self.active.remove(root);closure=self.a.close_case(case_id,root);self.note('task-pilot-case-closure',closure=closure)
            need(closure['processes_closed'] and closure['handles_closed'] and closure['python_callbacks_joined'],'actual-native-case-closure')
def full_oracle(adapter,*,root,checkpoint,final_pin,known_requests,required_mapped_keys):
    return adapter.probe('full',root,known_requests=known_requests,mapped_keys=required_mapped_keys,oracle_checks=sorted(sdk.ORACLE_CHECKS))
def main():
    need(not (O/'pilot-result.json').exists() and not ROOT.exists(),'single-fresh-pilot-no-rerun')
    (O/'logs').mkdir(exist_ok=True);(O/'temp').mkdir(exist_ok=True);(O/'bin').mkdir(exist_ok=True)
    build.verify('pilot-before')
    passed=O.parent/'final062-fixture10-20261025'
    for binding in json.loads((passed/'test-binaries-before.json').read_text(encoding='utf-8-sig')):need(sha(binding['path'])==binding['sha256'],'previous-test-binary-changed')
    original=passed/'target/release/file-native-incremental.exe';engine=O/'bin/engine.exe'
    need(sha(original)=='e6116d2ddb7cf92f15368c47810d6f35ca11fc5dfe479069266b3643d3419535','prior-bound-CLI-bytes-changed')
    shutil.copyfile(original,engine)
    save('engine-binding.json',dict(path=str(engine),bytes=engine.stat().st_size,sha256=sha(engine),built_original_path=str(original),source_identity=IDENTITY,code_freeze_sha=build.FREEZE_SHA,corrected_io_sha=build.CORRECTED_IO_SHA))
    # One practical room check, retaining both prior runs and all protected data.
    retained=sum(build.inventory(O.parent/name)['bytes'] for name in ('final-engine-build-tests','final062-fixture10-20261025','real-online-pilot-20261025'))
    save('room-check.json',dict(free_bytes=shutil.disk_usage(O).free,free_floor_bytes=140*GIB,retained_windows_build_test_bytes=retained,held_windows_work_forecast=676345051,pilot_expected_addition_bytes=32*1024**2,inside_reserve_untouched=True,no_new_all_host_census=True))
    need(shutil.disk_usage(O).free>140*GIB and retained+32*1024**2<676345051,'actual-pilot-headroom')
    runtime=Runtime();adapter=Adapter(runtime);driver=Driver(runtime,engine)
    sdk.FUNCTIONAL_CUTOFF=time.time()+300 # Current finite invocation deadline only; no expired phase authority.
    h=Harness(driver,adapter,lambda **kw:full_oracle(adapter,**kw),runtime.emit,final_pin=BASELINE)
    result=dict(source_identity=IDENTITY,status='FAIL',engine_binary_sha=driver.binary_sha256)
    try:
        answer=behavior.run(h,ROOT,dict(documents=64,accepted_H=3,max_engine_calls=37),dict(authority='actual human pilot authorization',source_identity=IDENTITY,passed31_report=str(passed/'report.md'),supervision='native Windows Job/positive-before-stdin/RSS and real pipe EOF; not40ms qualification'))
        result.update(status='PASS',behavior=answer,actual_N=64,actual_H=3,engine_calls=runtime.engine_calls,oracle_processes=adapter.probes)
        save('accepted-inputs.json',h.ledgers[ROOT].accepted)
    except BaseException as e:
        result.update(error=repr(e),traceback=traceback.format_exc(),engine_calls=runtime.engine_calls,oracle_processes=adapter.probes)
    finally:
        result['native_closure']=runtime.close()
        try:build.verify('pilot-after')
        except BaseException as e:result.update(status='FAIL',source_verification_error=repr(e))
        result['previous_test_binaries_unchanged']=all(sha(x['path'])==x['sha256'] for x in json.loads((passed/'test-binaries-before.json').read_text(encoding='utf-8-sig')))
        if not result['previous_test_binaries_unchanged']:result['status']='FAIL'
        result['reader_controls']=adapter.controls;result['actual_output_inventory']=build.inventory(O)
        if not all(result['native_closure'].get(x) for x in ('job_empty','all_transports_closed','all_stdout_stderr_EOF')) or result['native_closure']['resource_violation']:result['status']='FAIL'
        save('pilot-result.json',result);print(json.dumps(result),flush=True)
if __name__=='__main__':main()
