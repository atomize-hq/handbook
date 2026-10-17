"""Bounded native-process driver. No third-party Python dependencies.

Generated stores are preserved outside the checkout. JSONL is append-only raw
evidence; assertions fail loudly and do not fabricate later gate completion.
"""
import argparse, ctypes, hashlib, json, math, os, pathlib, platform, subprocess
import sys, time, threading, concurrent.futures, shutil

HERE = pathlib.Path(__file__).resolve().parent
DEFAULT_BASE = pathlib.Path('C:/file-native-experiment-20261017-v3')
DEFAULT_EXE = pathlib.Path('C:/file-native-experiment-20261017-build/release/file-native-publication-experiment.exe')
GIB = 1024**3
DEADLINE = 1792255680  # 2026-10-17 16:48 UTC; stop launch, reserve reporting.
CAP = 64*GIB
RESERVE = 160*GIB
COUNTER = 0
LOG_LOCK = threading.Lock()

def sha(b): return hashlib.sha256(b).hexdigest()
def load(p): return json.loads(pathlib.Path(p).read_bytes())
def write(p, v):
    p = pathlib.Path(p); p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(json.dumps(v, ensure_ascii=False, indent=2)+'\n', encoding='utf-8')

def inventory(root):
    buckets = {}; total_files = total_dirs = total_bytes = allocated = 0
    if os.name == 'nt':
        k = ctypes.WinDLL('kernel32', use_last_error=True)
        class Standard(ctypes.Structure):
            _fields_=[('allocation',ctypes.c_int64),('eof',ctypes.c_int64),('links',ctypes.c_uint32),('delete_pending',ctypes.c_ubyte),('directory',ctypes.c_ubyte)]
        k.CreateFileW.argtypes=[ctypes.c_wchar_p,ctypes.c_uint32,ctypes.c_uint32,ctypes.c_void_p,ctypes.c_uint32,ctypes.c_uint32,ctypes.c_void_p];k.CreateFileW.restype=ctypes.c_void_p
        k.GetFileInformationByHandleEx.argtypes=[ctypes.c_void_p,ctypes.c_int,ctypes.c_void_p,ctypes.c_uint32]
        k.CloseHandle.argtypes=[ctypes.c_void_p]
    for directory, dirs, files in os.walk(root):
        total_dirs += len(dirs)
        for name in files:
            p = pathlib.Path(directory)/name; size = p.stat().st_size
            key = next((part for part in p.parts if part in ('originals','generations','attempts','publications','receipts','view','copies')), 'other')
            b = buckets.setdefault(key, {'files':0,'bytes':0,'allocated_bytes':0})
            if os.name == 'nt':
                h=k.CreateFileW(str(p),0,7,None,3,0,None)
                if h in (None,ctypes.c_void_p(-1).value):raise ctypes.WinError(ctypes.get_last_error())
                info=Standard()
                try:
                    if not k.GetFileInformationByHandleEx(h,1,ctypes.byref(info),ctypes.sizeof(info)):raise ctypes.WinError(ctypes.get_last_error())
                    alloc=info.allocation
                finally:k.CloseHandle(h)
            else: alloc = p.stat().st_blocks*512
            b['files'] += 1; b['bytes'] += size; b['allocated_bytes'] += alloc
            total_files += 1; total_bytes += size; allocated += alloc
    return {'files':total_files,'directories':total_dirs,'logical_bytes':total_bytes,'allocated_file_bytes':allocated,'buckets':buckets}

class Driver:
    def __init__(self, base, exe, label):
        self.base, self.exe, self.label = pathlib.Path(base), pathlib.Path(exe), label
        self.log = self.base/(label+'-raw.jsonl'); self.events = []
        legacy=[pathlib.Path('C:/file-native-experiment-20261017'),pathlib.Path('C:/file-native-experiment-20261017-v2')]
        self.legacy_allocated=sum(inventory(p)['allocated_file_bytes'] for p in legacy if p.exists() and p!=self.base)
    def record(self, event):
        with LOG_LOCK:
            with self.log.open('a', encoding='utf-8') as f: f.write(json.dumps(event, ensure_ascii=False)+'\n')
            self.events.append(event)
    def budget(self, extra=0):
        if time.time()>DEADLINE: raise RuntimeError('declared-workload-deadline')
        free = shutil.disk_usage(self.base).free
        if free-extra<RESERVE: raise RuntimeError('declared-free-space-reserve')
        inv = inventory(self.base)
        build = inventory(self.exe.parents[1])
        if inv['allocated_file_bytes']+build['allocated_file_bytes']+self.legacy_allocated+extra>CAP: raise RuntimeError('declared-generated-data-cap')
        if time.time()>DEADLINE: raise RuntimeError('declared-workload-deadline')
        return inv
    def start(self, command, env=None):
        global COUNTER
        with LOG_LOCK: COUNTER += 1; seq = COUNTER
        started = time.perf_counter_ns()
        p = subprocess.Popen([str(self.exe)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                             env={**os.environ, **(env or {})})
        p.stdin.write(json.dumps(command, ensure_ascii=False).encode('utf-8')); p.stdin.close(); p.stdin = None
        return p, command, started, seq, env or {}
    def finish(self, pending, timeout=120, allowed=(0,)):
        p, command, started, seq, env = pending
        try: stdout, stderr = p.communicate(timeout=timeout)
        except subprocess.TimeoutExpired:
            p.kill(); stdout, stderr = p.communicate()
            self.record({'seq':seq,'command':command,'env':env,'exit':p.returncode,'timeout':True,'external_wall_ns':time.perf_counter_ns()-started,'stdout':stdout.decode(errors='replace'),'stderr':stderr.decode(errors='replace')})
            raise
        result = json.loads(stdout) if stdout.strip() else None
        event = {'seq':seq,'command':command,'env':env,'exit':p.returncode,'external_wall_ns':time.perf_counter_ns()-started,'output':result,'stderr':stderr.decode(errors='replace')}
        self.record(event)
        assert p.returncode in allowed, event
        return result
    def call(self, command, env=None, allowed=(0,), timeout=120): return self.finish(self.start(command,env),timeout,allowed)
    def init(self, root, mode, consumer, count=12):
        return self.call({'action':'init','root':str(root),'mode':mode,'consumer':consumer,'count':count},timeout=600)
    def inspect(self, root, audit=False): return self.call({'action':'inspect','root':str(root),'audit':audit})['result']
    def publish(self, root, mode, request, policy='speculative', env=None, allowed=(0,)):
        return self.call({'action':'publish','root':str(root),'mode':mode,'request':request,'policy':policy},env,allowed,600)

def request(name, ops, base=None): return {'request':name,'base':base,'ops':ops}
def edit(id='d000000', text='\n# revision\n'): return {'kind':'edit','id':id,'text':text}
def selected_state(root):
    sel = load(root/'SELECTED.json'); pub_bytes=(root/'publications'/(sel['publication']+'.json')).read_bytes()
    assert sha(pub_bytes)==sel['digest']; pub=json.loads(pub_bytes); c=pub['candidate']
    state_bytes=(root/c['state']).read_bytes(); assert sha(state_bytes)==c['state_digest']
    return sel,pub,json.loads(state_bytes)

def wait_path(p, child=None):
    start=time.monotonic()
    while not p.exists():
        if child is not None and child.poll() is not None: raise RuntimeError('child died before barrier')
        if time.monotonic()-start>30: raise RuntimeError('barrier timeout '+str(p))
        time.sleep(.005)

def reader(config):
    root=pathlib.Path(config['root']); _,p,s=selected_state(root)
    fixed=root/(p['candidate']['closure'] if config['mode']=='B' else 'view')
    first=s['docs']['d000000']['path']; second=s['docs']['d000001']['path']
    before=(fixed/first).read_bytes(); pathlib.Path(config['ready']).write_text('ready')
    wait_path(pathlib.Path(config['release']))
    if config.get('reselect'):
        _,p,_=selected_state(root); fixed=root/p['candidate']['closure']
    after=(fixed/second).read_bytes()
    print(json.dumps({'first':sha(before),'second':sha(after),'fixed_path':str(fixed)}))

def ordinary_copy(src,dst):
    for directory, dirs, files in os.walk(src):
        relative=pathlib.Path(directory).relative_to(src); (dst/relative).mkdir(parents=True,exist_ok=True)
        for name in files:
            with open(pathlib.Path(directory)/name,'rb') as a, open(dst/relative/name,'xb') as b:
                while block:=a.read(65536): b.write(block)

def check_packet(directory):
    prov=load(directory/'packet/provenance.json'); output=(directory/'packet/context.md').read_bytes()
    assert sha(output)==prov['output_sha256']
    rebuilt=b''
    for item in prov['inputs']:
        source=(directory/'packet'/item['path']).read_bytes()
        assert sha(source)==item['sha256'] and len(source)==item['bytes']
        rebuilt+=f"\n\n<!-- source {item['id']} sha256 {item['sha256']} -->\n".encode()+source
    assert rebuilt==output

def races(d, repetitions):
    observations=[]
    for mode in ['A','B']:
        for consumer in ['project','research']:
            for trial in range(repetitions):
                root=d.base/'correctness'/f'{mode}-{consumer}-{trial:02}'
                d.init(root,mode,consumer)
                sel0,p0,s0=selected_state(root)
                # Two complete, independently prepared candidates share one base.
                a=request('winner',[edit()],sel0); b=request('loser',[edit('d000001')],sel0)
                def prep(r): return d.start({'action':'prepare','root':str(root),'mode':mode,'request':r})
                pending=[prep(a),prep(b)]; prepared=[d.finish(x)['result'] for x in pending]
                d.call({'action':'select','root':str(root),'candidate':prepared[0]['file']})
                losing=d.call({'action':'select','root':str(root),'candidate':prepared[1]['file']},allowed=(2,))
                assert losing['error']=='conflict:expected-publication'
                info=d.inspect(root); assert len(info['history'])==2
                assert (root/prepared[1]['candidate']['closure']).is_dir()
                # Dedup is rechecked after complete competing preparations.
                same=request('same-id',[edit('d000004','\nSAME\n')])
                pending=[prep(same),prep(same)]; candidates=[d.finish(x)['result'] for x in pending]
                one=d.call({'action':'select','root':str(root),'candidate':candidates[0]['file']})
                two=d.call({'action':'select','root':str(root),'candidate':candidates[1]['file']})
                assert two['result']['status']=='deduplicated' and one['result']['result']==two['result']['result']
                mismatch=d.publish(root,mode,request('same-id',[edit('d000004','different')]),allowed=(2,))
                assert 'intent-mismatch' in mismatch['error']
                # Cancellation fences an already prepared, still-eligible attempt.
                cancel=d.finish(prep(request('fenced',[edit('d000005')])) )['result']
                d.call({'action':'cancel','root':str(root),'request_id':'fenced'})
                fenced=d.call({'action':'select','root':str(root),'candidate':cancel['file']},allowed=(2,))
                assert fenced['error']=='refused:cancelled-fenced'
                # Suspended ordinary reader; publish two exact file replacements.
                _,oldpub,oldstate=selected_state(root)
                barrier=d.base/'barriers'/f'{mode}-{consumer}-{trial}'; barrier.mkdir(parents=True)
                cfg={'root':str(root),'mode':mode,'ready':str(barrier/'ready'),'release':str(barrier/'release')}
                child=subprocess.Popen([sys.executable,str(HERE/'run.py'),'reader',json.dumps(cfg)],stdout=subprocess.PIPE,stderr=subprocess.PIPE)
                wait_path(barrier/'ready',child)
                d.publish(root,mode,request('reader-update',[edit(),edit('d000001')]))
                (barrier/'release').write_text('go'); out,err=child.communicate(timeout=30);assert child.returncode==0,err
                seen=json.loads(out);_,newpub,newstate=selected_state(root)
                assert seen['first']==oldstate['docs']['d000000']['digest']
                assert seen['second']==(oldstate if mode=='B' else newstate)['docs']['d000001']['digest']
                # Historical complete closure/packet survives later publications.
                oldclosure=root/oldpub['candidate']['closure']; check_packet(oldclosure)
                copy=d.base/'copies'/f'{mode}-{consumer}-{trial}'; ordinary_copy(oldclosure,copy);check_packet(copy)
                # Lost B/A reply after selection remains original beneath C deletion.
                lost=request('lost-reply',[edit('d000005','\nORIGINAL RESULT\n')])
                d.publish(root,mode,lost,env={'EXP_EXIT':'reply'},allowed=(86,))
                original=d.call({'action':'resolve','root':str(root),'request_id':'lost-reply'})['result']
                d.publish(root,mode,request('successor-delete',[{'kind':'delete','id':'d000005'}]))
                resolved=d.call({'action':'resolve','root':str(root),'request_id':'lost-reply'})['result']
                assert original==resolved and original['outcome']=='accepted'
                duplicate=d.publish(root,mode,lost); assert duplicate['result']['result']==original
                # Whole-root identity changes even on a no-op restored state.
                before,_,_=selected_state(root)
                d.publish(root,mode,request('equal-content',[]))
                after,_,_=selected_state(root);assert before!=after
                # Pure disjoint command reprepare requires explicit new base.
                stale=request('safe-revalidate',[edit('d000006')],before)
                refused=d.publish(root,mode,stale,allowed=(2,));assert 'conflict' in refused['error']
                d.publish(root,mode,request('safe-revalidate',[edit('d000006')],after))
                # Editor capture includes exact bytes and a base; later edits stay private.
                capture,_,_=selected_state(root); editor=barrier/'editor.json'
                exact=b'{\r\n  "n": 1.00, "label": "caf\xc3\xa9"\r\n}\r\n';editor.write_bytes(exact)
                imported=request('import',[{'kind':'import','id':'d000002','text':editor.read_bytes().decode()}],capture)
                prepared_import=d.finish(prep(imported))['result'];editor.write_bytes(b'{"later":true}')
                d.call({'action':'select','root':str(root),'candidate':prepared_import['file']})
                _,_,import_state=selected_state(root);assert (root/import_state['docs']['d000002']['original']).read_bytes()==exact
                assert editor.read_bytes()==b'{"later":true}'
                # Repeated failures in one process preserve unique staging ownership.
                d.call({'action':'repeat-error','root':str(root),'mode':mode,'request':request('repeat-error',[edit('d000007')])})
                # Semantic invalid combinations refuse without publication.
                base,_,_=selected_state(root)
                for label,ops in [('delete-target',[{'kind':'delete','id':'d000001'}]),('missing-local',[{'kind':'edge','id':'d000000','text':'cites','target':'missing'}]),('unique',[{'kind':'label','id':'d000004','text':'one-label'}]),('budget',[{'kind':'cost','id':'d000000','value':600},{'kind':'cost','id':'d000001','value':600}])]:
                    failed=d.publish(root,mode,request(label,ops),allowed=(2,));assert failed['error'].startswith('semantic:'),failed
                    assert selected_state(root)[0]==base
                d.inspect(root,audit=True)
                observations.append({'mode':mode,'consumer':consumer,'trial':trial,'ordinary_reader':seen,'snapshot_packet_copy':'verified','history_publications':len(d.inspect(root)['history'])})
                if trial%5==0: print(f'race {mode}/{consumer} {trial+1}/{repetitions}',flush=True)
    write(d.base/'race-observations.json',observations)
    return observations

CHECKPOINTS = ['payload-before-write','file-write','file-sync','payload-installed','directory-create','state-file-write','state-file-sync','state-installed','copy-write','copy-sync','manifest-file-write','manifest-file-sync','closure-before-verify','closure-verified','closure-sealed','candidate-ready','lock-acquired','predecessor-established','before-dedup','after-dedup','expected-checked','publication-file-write','publication-file-sync','publication-installed','selector-file-write','selector-file-sync','selector-staged','selector-visible','selection-boundary','receipt-file-write','receipt-file-sync','receipt-before','receipt-written','reply']

def fault_suite(d):
    results=[]
    for mode in ['A','B']:
        points=CHECKPOINTS+(['view-copy-write','view-copy-sync','view-file-replaced','view-first-document','view-before-complete'] if mode=='A' else ['generation-directory-create','generation-copy-write','generation-copy-sync','generation-installed'])
        for checkpoint in points:
            for fault in ['exit','error','kill']:
                root=d.base/'faults'/f'{mode}-{checkpoint}-{fault}';d.init(root,mode,'project')
                initial=selected_state(root)[0]; r=request('fault-request',[edit(),edit('d000001')])
                command={'action':'publish','root':str(root),'mode':mode,'request':r,'policy':'speculative'}
                if fault=='kill':
                    ready=root/'KILL-READY';release=root/'KILL-RELEASE'
                    pending=d.start(command,{'EXP_BARRIER':checkpoint,'EXP_READY':str(ready),'EXP_RELEASE':str(release),'EXP_TRACE':'1'})
                    wait_path(ready,pending[0]);pending[0].kill(); outcome=d.finish(pending,allowed=(1,86,-9,4294967295))
                else: outcome=d.call(command,{'EXP_'+fault.upper():checkpoint,'EXP_TRACE':'1'},allowed=(0,2,86))
                current=selected_state(root)[0]; selected=current!=initial
                postboundary=checkpoint in ['selection-boundary','receipt-file-write','receipt-file-sync','receipt-before','receipt-written','reply','view-copy-write','view-copy-sync','view-file-replaced','view-first-document','view-before-complete']
                if postboundary:assert selected,('required accepted publication lost',mode,checkpoint,fault)
                elif checkpoint!='selector-visible':assert not selected,('preselection unexpectedly published',mode,checkpoint,fault)
                resolved=d.call({'action':'resolve','root':str(root),'request_id':'fault-request'})['result']
                if selected:
                    assert resolved['outcome']=='accepted'
                    if mode=='A': d.call({'action':'resume','root':str(root)})
                    # C establishes a possibly uncertain B before acknowledging C.
                    d.publish(root,mode,request('successor',[edit('d000004')]))
                    assert d.call({'action':'resolve','root':str(root),'request_id':'fault-request'})['result']==resolved
                else:
                    assert resolved['outcome']=='absent-as-of' and not resolved['terminal_noncommit']
                    d.publish(root,mode,r)
                d.inspect(root,audit=True)
                results.append({'mode':mode,'checkpoint':checkpoint,'fault':fault,'selected_at_reopen':selected,'phase_oracle':'must-retain' if postboundary else 'visible-uncertain' if checkpoint=='selector-visible' else 'must-not-select','reported_status':outcome['result'].get('status') if outcome and outcome['status']=='ok' else None,'oracle':'pass-process-fault'})
            print(f'fault {mode}/{checkpoint}',flush=True)
    # A real partial staged payload is retained; final identity remains retryable.
    for mode in ['A','B']:
        root=d.base/'faults'/f'{mode}-partial-payload';d.init(root,mode,'research')
        r=request('partial',[edit()]);failed=d.publish(root,mode,r,env={'EXP_ERROR':'payload-partial-write'},allowed=(2,))
        assert 'payload-partial-write' in failed['error'];d.publish(root,mode,r);d.inspect(root,audit=True)
        results.append({'mode':mode,'checkpoint':'payload-partial-write','fault':'partial-reported-error','oracle':'pass-retry'})
    write(d.base/'fault-observations.json',results);return results

def special_checks(d):
    observations={}
    # Store independence, while one process holds a publication lock at a barrier.
    project=d.base/'special/project';research=d.base/'special/research'
    d.init(project,'B','project');d.init(research,'A','research')
    ready=project/'READY';release=project/'RELEASE'
    pending=d.start({'action':'publish','root':str(project),'mode':'B','policy':'slot','request':request('held',[edit()])},{'EXP_BARRIER':'lock-acquired','EXP_READY':str(ready),'EXP_RELEASE':str(release)})
    wait_path(ready,pending[0]);t=time.perf_counter_ns();d.inspect(research);d.publish(research,'A',request('independent',[edit()]));elapsed=time.perf_counter_ns()-t
    release.write_text('go');d.finish(pending)
    # Explicit cross-store vector and copied immutable closures, with agreement.
    vector={};packet=d.base/'copies/cross-store'
    for name,root in [('project',project),('research',research)]:
        sel,pub,_=selected_state(root);vector[name]=sel;ordinary_copy(root/pub['candidate']['closure'],packet/name);check_packet(packet/name)
    write(packet/'agreement.json',{'roots':vector,'agreement':'these independently selected roots and retained copies form this packet; no atomic global commit'})
    observations['independent_store_elapsed_ns']=elapsed;observations['cross_store_vector']=vector
    # B per-open alias control observes old overview, new requirement.
    ready=project/'RESELECT-READY';release=project/'RESELECT-RELEASE'
    _,_,old=selected_state(project)
    cfg={'root':str(project),'mode':'B','ready':str(ready),'release':str(release),'reselect':True}
    p=subprocess.Popen([sys.executable,str(HERE/'run.py'),'reader',json.dumps(cfg)],stdout=subprocess.PIPE)
    wait_path(ready,p);d.publish(project,'B',request('per-open',[edit(),edit('d000001')]))
    release.write_text('go');out,_=p.communicate(timeout=30);assert p.returncode==0
    seen=json.loads(out);_,_,new=selected_state(project)
    assert seen['first']==old['docs']['d000000']['digest'] and seen['second']==new['docs']['d000001']['digest']
    observations['per_open_alias_control']=seen
    # Missing metadata/payload is typed damage, not ordinary absent key/view lag.
    for target in ['payload','state','publication','selector']:
        root=d.base/'damage'/target;d.init(root,'B','project');sel,pub,s=selected_state(root)
        path={'payload':root/s['docs']['d000001']['original'],'state':root/pub['candidate']['state'],'publication':root/'publications'/(sel['publication']+'.json'),'selector':root/'SELECTED.json'}[target]
        saved=path.read_bytes();path.unlink()
        failed=d.call({'action':'inspect','root':str(root),'audit':True},allowed=(2,))
        assert 'integrity' in failed['error'];path.write_bytes(saved)
        observations['missing_'+target]=failed['error']
    # Actual corrupted metadata bytes, preserving damaged evidence separately.
    root=d.base/'damage/corrupt';d.init(root,'B','research');_,pub,_=selected_state(root);p=root/pub['candidate']['state'];b=p.read_bytes();p.write_bytes(b[:len(b)//2]+b'X'+b[len(b)//2+1:])
    failed=d.call({'action':'inspect','root':str(root)},allowed=(2,));assert 'integrity-digest' in failed['error'];observations['corrupt_state']=failed['error']
    # Duplicate keys rejected before serde Value can collapse them.
    p=subprocess.run([str(d.exe)],input=b'{"action":"inspect","action":"init","root":"unused"}',capture_output=True)
    dup=json.loads(p.stdout);assert p.returncode==2 and 'duplicate-key' in dup['error'];observations['duplicate_key']=dup
    # Windows no-delete-sharing on actual selector, followed by bounded retry.
    if os.name=='nt':
        k=ctypes.WinDLL('kernel32',use_last_error=True);k.CreateFileW.argtypes=[ctypes.c_wchar_p,ctypes.c_uint32,ctypes.c_uint32,ctypes.c_void_p,ctypes.c_uint32,ctypes.c_uint32,ctypes.c_void_p];k.CreateFileW.restype=ctypes.c_void_p;k.CloseHandle.argtypes=[ctypes.c_void_p]
        before=selected_state(project)[0];h=k.CreateFileW(str(project/'SELECTED.json'),0x80000000,1|2,None,3,0,None)
        assert h not in (None,ctypes.c_void_p(-1).value)
        try:
            blocked=d.publish(project,'B',request('held-handle',[edit()]))
            assert blocked['result']['status']=='selection-uncertain' and selected_state(project)[0]==before
        finally:k.CloseHandle(h)
        retried=d.publish(project,'B',request('held-handle',[edit()]));assert retried['result']['result']['outcome']=='accepted'
        observations['windows_no_delete_share']={'blocked':blocked,'retry':retried}
    write(d.base/'special-observations.json',observations);return observations

def batch_ops(count, marker):
    # Appending whitespace preserves exact JSON syntax without reserialization.
    return [edit(f'd{i:06}', ' \n' if i%10==2 else f'\n# {marker}\n') for i in range(count)]

def growth(d, tiers, cases=None):
    reached=[]
    for n in tiers:
        # Actual upper-bound reservation includes originals, B full retained copies,
        # A views/packets and speculative abandoned candidates across both stores.
        estimate={1000:2*GIB,10000:12*GIB,100000:36*GIB}[n]
        d.budget(estimate)
        for first in (['A','B'] if n<100000 else ['slot','speculative']):
            for consumer in ['project','research']:
                for last in (['slot','speculative'] if n<100000 else ['A','B']):
                    mode,policy=(first,last) if n<100000 else (last,first)
                    key=f'{n}-{mode}-{consumer}-{policy}'
                    if cases and key not in cases:continue
                    d.budget(estimate//4)
                    root=d.base/'growth'/key
                    if root.exists():
                        # Positive selected lineage drives resumption; surviving
                        # payloads never reconstruct an interrupted initialization.
                        info=d.inspect(root)
                        done={p['request'] for p in info['history']}
                    else:
                        d.init(root,mode,consumer,n);done={'initial'}
                    for batch in ([1,64] if n==100000 else [1,8,64]):
                        if f'serial-{batch}' in done:continue
                        ops=batch_ops(batch,f'serial-{batch}')
                        capture=None
                        if batch==64:
                            # One bounded larger attachment exercises multi-buffer
                            # exact capture/copy without changing the 64-doc count.
                            ops[9]={'kind':'import','id':'d000009','text':('Captured attachment evidence.\n'*10000)[:262144]}
                            capture=selected_state(root)[0]
                        d.publish(root,mode,request(f'serial-{batch}',ops,capture),policy)
                    if n<100000:
                        for width in ([2,4,8] if n==1000 else [2,4]):
                            indices=[i for i in range(width) if f'wave-{width}-{i}' not in done]
                            if not indices:continue
                            ready_dir=root/f'wave-{width}-{time.time_ns()}';ready_dir.mkdir()
                            pending=[]
                            for i in indices:
                                r=request(f'wave-{width}-{i}',[edit(f'd{100+i:06}',f'\n# worker {i}\n')])
                                command={'action':'publish','root':str(root),'mode':mode,'policy':policy,'request':r}
                                # Deterministic common base: speculative candidates all
                                # finish before either can enter publication gate.
                                env={'EXP_BARRIER':'candidate-ready','EXP_READY':str(ready_dir/f'{i}.ready'),'EXP_RELEASE':str(ready_dir/'release')} if policy=='speculative' else {}
                                pending.append(d.start(command,env))
                            if policy=='speculative':
                                for i,p in zip(indices,pending):wait_path(ready_dir/f'{i}.ready',p[0])
                                (ready_dir/'release').write_text('go')
                            outcomes=[d.finish(p,timeout=600,allowed=(0,2)) for p in pending]
                            for i,out in zip(indices,outcomes):
                                if out['status']=='error':
                                    assert out['error']=='conflict:expected-publication' or out['error'].startswith('busy:lock-timeout:')
                                    # Explicitly reprepare the pure command against new
                                    # state, not replay arbitrary agent/external work.
                                    d.publish(root,mode,request(f'wave-{width}-{i}',[edit(f'd{100+i:06}',f'\n# worker {i}\n')]),policy)
                        if n==1000:
                            # One large preparation holds the slot or competes with
                            # one small request; bounded, no FIFO assertion.
                            commands=[request('slow',batch_ops(64,'slow')),request('small',[edit('d000900')])]
                            pending=[(i,d.start({'action':'publish','root':str(root),'mode':mode,'policy':policy,'request':r})) for i,r in enumerate(commands) if r['request'] not in done]
                            for i,p in pending:
                                out=d.finish(p,timeout=600,allowed=(0,2))
                                if out['status']=='error':
                                    assert out['error'].startswith(('conflict:','busy:lock-timeout:'))
                                    d.publish(root,mode,request('slow' if i==0 else 'small',batch_ops(64,'slow') if i==0 else [edit('d000900')]),policy)
                    info=d.inspect(root)
                    expected={1000:20,10000:10,100000:3}[n]
                    assert len(info['history'])==expected,(len(info['history']),expected)
                    # Fresh OS process, uncontrolled device cache; exact key present
                    # and absent checked by each inspect invocation.
                    for _ in range(3):
                        point=d.inspect(root);assert point['positive'] and point['negative'] is None
                    inv=inventory(root);reached.append({'count':n,'mode':mode,'consumer':consumer,'policy':policy,'selected_publications':len(info['history']),'inventory':inv})
                    write(d.base/'growth-observations.json',reached)
                    print(f'growth {n}/{mode}/{consumer}/{policy}: {inv["logical_bytes"]} bytes, {inv["files"]} files',flush=True)
    return reached

def main():
    if len(sys.argv)>1 and sys.argv[1]=='reader': reader(json.loads(sys.argv[2]));return
    ap=argparse.ArgumentParser();ap.add_argument('phase',choices=['correctness','growth','inventory']);ap.add_argument('--base',type=pathlib.Path,default=DEFAULT_BASE);ap.add_argument('--exe',type=pathlib.Path,default=DEFAULT_EXE);ap.add_argument('--repetitions',type=int,default=30);ap.add_argument('--tiers',nargs='+',type=int,default=[1000,10000,100000]);ap.add_argument('--cases',nargs='+');args=ap.parse_args()
    if args.phase=='correctness':
        args.base.mkdir(exist_ok=False)
        write(args.base/'environment.json',{'platform':platform.platform(),'python':sys.version,'executable':str(args.exe),'exe_sha256':sha(args.exe.read_bytes()),'started_epoch':time.time(),'deadline_epoch':DEADLINE,'cap_bytes':CAP,'reserve_bytes':RESERVE,'disk_before':shutil.disk_usage(args.base)._asdict(),'seed':20261017})
    elif not args.base.exists():raise RuntimeError('run correctness first')
    d=Driver(args.base,args.exe,args.phase)
    try:
        if args.phase=='correctness':
            d.budget(4*GIB);races(d,args.repetitions);fault_suite(d);special_checks(d)
            write(args.base/'correctness-completed.json',{'repetitions_per_configuration':args.repetitions,'race_configurations':4,'status':'local-process-suite-passed','native_linux':'unmet','ordinary_browsers':'separate-observation-required'})
        elif args.phase=='growth':
            assert (args.base/'correctness-completed.json').exists();growth(d,args.tiers,args.cases)
        else:write(args.base/'inventory.json',inventory(args.base))
    except BaseException as error:
        write(args.base/(args.phase+'-stopped.json'),{'type':type(error).__name__,'error':str(error),'epoch':time.time()});raise

if __name__=='__main__':main()
