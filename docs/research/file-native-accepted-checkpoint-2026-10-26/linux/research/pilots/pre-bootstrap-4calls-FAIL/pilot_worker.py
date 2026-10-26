"""One task-owned Linux bridge; original SDK and engine source remain unchanged."""
import contextlib,fcntl,hashlib,importlib.util,json,os,signal,subprocess,sys,threading,time,traceback,uuid
from pathlib import Path
ROOT=Path(__file__).resolve().parent
PACKET=ROOT/'source/mac-sdk-20261025';SDK=PACKET/'sdk'
SOURCE=Path('/home/spenser/file-native-online-linux-20261020/source/final-engine')
IDENTITY=Path('/tmp/file-native-host-inventory-linux-20261024-01a16fd3/telemetry-n10-correction-20261025/corrected-code-manifest.json')
PIN='0628322e4b48e0efad19cdc32238a622d4aaa7a8'
os.environ['FILE_NATIVE_ORACLE_RESEARCH']=str(PACKET/'research')
sys.dont_write_bytecode=True
sys.path[:0]=[str(SDK),str(ROOT)]
import focused_protocol as fp
import case_ledger
import oracle_mapping_bridge
import pilot_recipe_native as recipe
fp.FUNCTIONAL_CUTOFF=time.time()+300

def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def write(name,v):
    p=ROOT/name
    with p.open('x') as f:json.dump(v,f,indent=2);f.write('\n')
def proc(pid):
    try:
        a=Path('/proc',str(pid),'stat').read_text().rsplit(')',1)[1].split()
        return dict(pid=pid,ppid=int(a[1]),group=int(a[2]),start=int(a[19]),rss=int(a[21])*os.sysconf('SC_PAGE_SIZE'),state=a[0])
    except (OSError,IndexError,ValueError):return None

class Host(fp.ThreadCalls):
    def __init__(self):
        self.lock=threading.RLock();self.events_lock=threading.Lock();self.active={};self.by_thread={};self.handles=[];self.controls=[]
        self.calls=0;self.checkers=0;self.completed=[];self.guard_stop=threading.Event();self.guard_error=None;self.samples=[]
        self.binary=ROOT/'target/release/file-native-incremental';self.binary_sha256=sha(self.binary)
        self.mapping_oracle=oracle_mapping_bridge.Oracle(self)
        self.capabilities={'barrier:pack-catalog-captured','barrier:physical-before-selection','barrier:candidate-ready'}
        self.monitor=threading.Thread(target=self.observe,name='native-resource-observer');self.monitor.start()
    def emit(self,row):
        with self.events_lock:
            with (ROOT/'events.jsonl').open('a') as f:f.write(json.dumps(dict(monotonic_ns=time.monotonic_ns(),wall_ns=time.time_ns(),**row),sort_keys=True)+'\n');f.flush()
    def verify_sources(self):
        manifest=json.loads(IDENTITY.read_text())
        for name,r in manifest['files'].items():assert sha(SOURCE/name)==r['sha256'] and (SOURCE/name).stat().st_size==r['bytes'],name
        for r in json.loads((PACKET/'transfer-manifest.json').read_text())['members']:
            assert sha(PACKET/r['path'])==r['sha256'] and (PACKET/r['path']).stat().st_size==r['bytes'],r['path']
        assert sha(self.binary)==self.binary_sha256
        passed=json.loads(Path('/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/result.json').read_text())
        assert passed['status']=='PASS' and passed['passed']==31 and passed['failed']==0 and passed['filtered']==0
        assert passed['corrected_io_sha256']==manifest['files']['src/io.rs']['sha256']
        for path,phase in [('src/packing.rs','pack-catalog-captured'),('src/packing.rs','physical-before-selection'),('src/store.rs','candidate-ready')]:
            assert ('"'+phase+'"') in (SOURCE/path).read_text(),'source-hook-missing'
        return manifest
    def observe(self):
        try:
            while not self.guard_stop.wait(.2):
                with self.lock:members=list(self.active.values())
                rows=[]
                for member in members:
                    p=proc(member['p'].pid)
                    if p and p['start']==member['birth'] and p['state']!='Z':
                        assert p['rss']<=member['limit'],'per-role-RSS-limit';rows.append(p)
                own=proc(os.getpid());total=sum(p['rss'] for p in rows)+(own['rss'] if own else 0)
                assert total<=3*1024**3,'aggregate-RSS-limit'
                s=os.statvfs(ROOT);assert s.f_bavail*s.f_frsize>=20*1024**3,'free-space-floor'
                self.samples.append(dict(monotonic_ns=time.monotonic_ns(),aggregate=total,members=rows))
        except BaseException as e:
            self.guard_error=repr(e);self.emit(dict(kind='resource-stop',error=self.guard_error));self.stop_all()
    def stop_one(self,m):
        p=m['p'];now=proc(p.pid)
        if now and now['start']==m['birth'] and p.poll() is None:
            try:os.killpg(p.pid,signal.SIGTERM)
            except ProcessLookupError:pass
            try:p.wait(timeout=.5)
            except subprocess.TimeoutExpired:
                now=proc(p.pid)
                if now and now['start']==m['birth']:
                    try:os.killpg(p.pid,signal.SIGKILL)
                    except ProcessLookupError:pass
                p.wait(timeout=5)
    def stop_all(self):
        with self.lock:members=list(self.active.values())
        for m in members:self.stop_one(m)
    def command(self,argv,input_bytes,*,label,limit,timeout,on_started=None,env_add=None):
        token=uuid.uuid4().hex;env=os.environ.copy()
        for k in list(env):
            if k.startswith('EXP_'):env.pop(k)
        env.update({'TMPDIR':str(ROOT/'tmp'),'TMP':str(ROOT/'tmp'),'TEMP':str(ROOT/'tmp'),'PYTHONDONTWRITEBYTECODE':'1'})
        env.update(env_add or {})
        output={};threads=[];p=None;m=None;primary=None;started=time.monotonic()
        row=dict(token=token,label=label,argv=argv,cwd=str(ROOT),stdin_expected=len(input_bytes),stdin_sent=0,
                 environment=env_add or {},limit=limit,timeout=timeout,starts_ns=time.monotonic_ns())
        (ROOT/'logs'/f'{label}-{token}.input').write_bytes(input_bytes)
        def drain(stream,which):
            count=0
            with (ROOT/'logs'/f'{label}-{token}.{which}').open('xb') as f:
                while True:
                    b=stream.read(65536)
                    if not b:break
                    count+=len(b);f.write(b)
                    if count>8*1024**2:
                        output[which+'-error']='output-cap';self.stop_one(m);break
            stream.close();output[which+'-eof']=time.monotonic_ns();output[which+'-bytes']=count
        try:
            p=subprocess.Popen(argv,cwd=ROOT,env=env,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,start_new_session=True)
            observed=None;due=time.monotonic()+3
            while time.monotonic()<due:
                observed=proc(p.pid)
                if observed and observed['rss']>0:break
                assert p.poll() is None,'native-exit-before-stdin';time.sleep(.005)
            assert observed and 0<observed['rss']<=limit,'positive-native-start'
            m=dict(p=p,birth=observed['start'],limit=limit,token=token)
            with self.lock:self.active[token]=m;self.by_thread[threading.get_ident()]=m
            row.update(pid=p.pid,birth=observed['start'],positive_rss=observed['rss'],observed_ns=time.monotonic_ns())
            for which,stream in [('stdout',p.stdout),('stderr',p.stderr)]:
                t=threading.Thread(target=drain,args=(stream,which));threads.append(t);t.start()
            if on_started:on_started(dict(pid=p.pid,positive_rss=observed['rss'],before_stdin_delivery=True,observed_ns=row['observed_ns'],birth=observed['start']))
            p.stdin.write(input_bytes);p.stdin.flush();row['stdin_sent']=len(input_bytes);p.stdin.close();row['stdin_eof_ns']=time.monotonic_ns()
            p.wait(timeout=timeout)
        except BaseException as e:
            primary=e;row['error']=repr(e)
            if m:self.stop_one(m)
        finally:
            if p and p.stdin and not p.stdin.closed:p.stdin.close()
            for t in threads:t.join(5)
            if any(t.is_alive() for t in threads):
                if m:self.stop_one(m)
                for t in threads:t.join(5)
                primary=primary or RuntimeError('output-reader-not-joined')
            if p:
                if p.poll() is None and m:self.stop_one(m)
                row['exit']=p.wait(timeout=5);row['native_wait_ns']=time.monotonic_ns()
                group=[]
                for q in Path('/proc').iterdir():
                    if q.name.isdigit():
                        r=proc(int(q.name))
                        if r and r['group']==p.pid and r['state']!='Z':group.append(r)
                row['remaining_group']=group
                if group:primary=primary or RuntimeError('owned-descendant-remains')
            row.update(output);row['seconds']=time.monotonic()-started
            row['readers_joined']=not any(t.is_alive() for t in threads)
            with self.lock:
                if m:self.active.pop(token,None)
            write('logs/'+label+'-'+token+'.json',row);self.completed.append(row);self.emit(dict(kind='native-command-final',**row))
        if primary:raise primary
        assert not self.guard_error,self.guard_error
        return row,(ROOT/'logs'/f'{label}-{token}.stdout').read_bytes(),(ROOT/'logs'/f'{label}-{token}.stderr').read_bytes()
    def call(self,root,action,*,timeout=120,code=0,env=None,on_started=None,**fields):
        with self.lock:self.calls+=1;number=self.calls
        assert number<=37,'37-engine-invocation-cap'
        payload=dict(root=str(root),action=action,**fields);raw=json.dumps(payload,separators=(',',':')).encode()
        limit=256*1024**2 if action in fp.READERS else 2*1024**3
        row,stdout,stderr=self.command([str(self.binary)],raw,label=f'engine-{number:02d}-{action}',limit=limit,timeout=timeout,on_started=on_started,env_add=env)
        if row['exit']!=code:raise AssertionError(dict(input=payload,env=env,executable_sha256=self.binary_sha256,violations=[],stdout=stdout.decode(),exit=row['exit'],native_row=row))
        return fp.strict(stdout,8*1024**2)
    def before_call(self,root,action,role,seconds):
        assert root==ROOT/'disposable/store' and seconds>0 and not self.guard_error
    def wall_time(self):return time.time()
    def monotonic(self):return time.monotonic()
    def sleep(self,seconds):time.sleep(seconds)
    def spawn(self,callback):
        handle=super().spawn(callback);self.handles.append(handle);return handle
    def alive(self,handle):
        if not handle['thread'].is_alive():return False
        with self.lock:m=self.by_thread.get(handle['thread'].ident)
        native=proc(m['p'].pid) if m else None
        return bool(m and m['p'].poll() is None and native and native['start']==m['birth'])
    def control(self,root,name):
        d=ROOT/'controls'/name;d.mkdir(parents=True,exist_ok=False)
        c=dict(ready=str(d/'ready'),release=str(d/'release'));self.controls.append(c);return c
    def await_ready(self,control,handle,timeout):
        end=time.monotonic()+timeout
        while not Path(control['ready']).exists():
            if not handle['thread'].is_alive():return self.wait(handle,0)
            assert time.monotonic()<end,'ready-timeout';time.sleep(.005)
        assert self.alive(handle),'native-not-alive-at-ready'
        phase=Path(control['ready']).read_text();assert 'barrier:'+phase in self.capabilities
        self.emit(dict(kind='native-barrier-ready',control=control,phase=phase,pid=self.by_thread[handle['thread'].ident]['p'].pid))
    def release(self,control):
        p=Path(control['release'])
        if not p.exists():p.write_text('release');self.emit(dict(kind='native-barrier-release',control=control))
    def read(self,root,name,limit):
        p=root/name;assert p.is_relative_to(root) and not p.is_symlink()
        with p.open('rb') as f:
            assert os.fstat(f.fileno()).st_size<=limit
            b=f.read(limit+1);assert len(b)<=limit;return b
    def exists(self,root,name):return (root/name).exists()
    def inventory(self,root):
        return {p.relative_to(root).as_posix():dict(bytes=p.stat().st_size,sha256=sha(p)) for p in root.rglob('*') if p.is_file()}
    def quiesce(self,root):
        with self.lock:assert not self.active,'oracle-requires-no-active-native-call'
    def mapping(self,root,key):return self.mapping_oracle.mapping(root,key)
    def backlog(self,root):return self.mapping_oracle.backlog(root)
    def close_case(self,case,root):
        for c in self.controls:self.release(c)
        secondary=[]
        for h in self.handles:
            try:self.wait(h,15)
            except BaseException as e:secondary.append(repr(e))
        self.stop_all()
        for h in self.handles:h['thread'].join(5)
        with self.lock:assert not self.active,'active-native-call-on-closure'
        assert all(not h['thread'].is_alive() for h in self.handles),'callback-not-joined'
        return dict(processes_closed=True,handles_closed=all(r.get('readers_joined') and not r.get('remaining_group') for r in self.completed),python_callbacks_joined=True,secondary=secondary)
    def full_oracle(self,*,root,checkpoint,final_pin,known_requests,required_mapped_keys):
        self.quiesce(root);before=self.inventory(root);token=uuid.uuid4().hex
        output=ROOT/('oracle-'+token+'.json');config=ROOT/('oracle-'+token+'.input.json')
        config.write_text(json.dumps(dict(root=str(root),output=str(output),baseline_pin=PIN,known_requests=known_requests)))
        self.checkers+=1
        row,stdout,stderr=self.command([sys.executable,'-B',str(ROOT/'checker.py'),str(config)],b'',label='complete-oracle',limit=2*1024**3,timeout=120)
        assert row['exit']==0,dict(error='complete-oracle-failed',stderr=stderr.decode())
        proof=json.loads(output.read_text());assert proof['schema_pin']==PIN and proof['request_intents']
        assert self.inventory(root)==before,'independent-oracle-mutated-store'
        d=self.mapping_oracle.decoder(root)
        assert all(k in d.entries for k in required_mapped_keys)
        return dict(status='passed',final_pin=PIN,engine_code_imported=False,engine_audit_used=False,actual_H=len(d.accepted_chain()),mapped_keys=required_mapped_keys,source_unchanged=True,checks=sorted(fp.ORACLE_CHECKS),receipt_sha256=sha(output),receipt=str(output),underlying=proof)

class Harness(fp.Harness):
    @contextlib.contextmanager
    def case(self,case,root,forecast):
        self.a.verify_sources();assert not root.exists() and root==ROOT/'disposable/store'
        assert time.time()<fp.FUNCTIONAL_CUTOFF;self.active.add(root);self.ledgers[root]=fp.Inputs()
        self.note('case-start-native-task',case_id=case,source_identity='baseline-plus-telemetry-n10-patch',source_manifest_sha256=sha(IDENTITY),binary_sha256=self.driver.binary_sha256)
        failure=None
        try:yield
        except BaseException as e:failure=repr(e);self.note('case-failure',error=failure);raise
        finally:
            self.active.remove(root);closure=self.a.close_case(case,root);self.note('case-closure',failure=failure,closure=closure)
            assert closure['processes_closed'] and closure['handles_closed'] and closure['python_callbacks_joined']

def main():
    with (ROOT/'pilot.started.json').open('x') as f:json.dump(dict(start_ns=time.monotonic_ns()),f)
    host=Host();h=Harness(host,host,host.full_oracle,host.emit,final_pin=PIN);host.h=h
    result=dict(status='STARTED',baseline_pin=PIN,source_identity='baseline-plus-telemetry-n10-patch',root=str(ROOT/'disposable/store'))
    ledger=case_ledger.CaseLedger('Linux',PIN)
    try:
        answer=recipe.run(h,ROOT/'disposable/store',dict(N=64,H=3,engine_calls_max=37))
        host.verify_sources();assert host.calls<=37
        result.update(status='PASS',schedule=answer)
    except BaseException as e:
        result.update(status='FAIL',error=repr(e));(ROOT/'pilot-exception.txt').write_text(traceback.format_exc())
    finally:
        closure=host.close_case(recipe.CASE,ROOT/'disposable/store')
        host.guard_stop.set();host.monitor.join(5)
        result.update(engine_calls=host.calls,independent_checker_processes=host.checkers,closure=closure,resource_error=host.guard_error,monitor_joined=not host.monitor.is_alive(),source_verified=host.verify_sources())
        result['native_roles']=[r for r in host.completed if r.get('label','').startswith('engine-')]
        write('pilot-result.json',result);write('pilot-resources.json',host.samples)
        write('matrix-ledger-pending.json',ledger.result())
        print(json.dumps({k:result[k] for k in ['status','engine_calls','independent_checker_processes','closure']}))
if __name__=='__main__':main()
