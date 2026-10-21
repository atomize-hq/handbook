"""Closed Linux final-suite source loader. No workload is authorized by this file."""
import os,sys,time,signal,math
PHASE_STARTED=time.time();PHASE_DEADLINE=None
if __name__=='__main__' and '--phase' in sys.argv:
    # Absolute parent clock includes all subsequent stdlib/owned imports,
    # admission, census, child work, gzip closure and post-exit finalization.
    PHASE_DEADLINE=float(sys.argv[sys.argv.index('--phase-deadline')+1])
    if not math.isfinite(PHASE_DEADLINE) or PHASE_DEADLINE<=PHASE_STARTED:raise SystemExit('expired-phase-before-import')
    def phase_alarm(signum,frame):raise TimeoutError('absolute-V5-phase-deadline')
    signal.signal(signal.SIGALRM,phase_alarm)
    signal.setitimer(signal.ITIMER_REAL,PHASE_DEADLINE-PHASE_STARTED)
EARLY_ROWS=[];EARLY_STOP=None;EARLY_THREAD=None

def caller_sample():
    values={}
    with open('/proc/self/status') as f:
        for line in f:
            if line.startswith(('VmRSS:','VmHWM:')):
                k,v=line.split(':',1);values[k]=int(v.split()[0])*1024
    with open('/proc/self/stat') as f:raw=f.read()
    start=int(raw[raw.rfind(')')+2:].split()[19]);rss=values.get('VmRSS');hwm=values.get('VmHWM')
    if not(rss and hwm and rss<=hwm<=768*1024**2):os._exit(126)
    return dict(epoch=time.time(),caller_pid=os.getpid(),caller_start=start,caller_rss=rss,caller_hwm=hwm,supervisor_rss=rss,heavy_rss=0,members=[],zombie_pids=[])
if __name__=='__main__':
    sys.modules['suite_bootstrap']=sys.modules[__name__]
if PHASE_DEADLINE is not None:
    import threading
    EARLY_STOP=threading.Event()
    def early_observe():
        due=time.monotonic();last=None
        while not EARLY_STOP.is_set():
            try:row=caller_sample()
            except BaseException:os._exit(126)
            now=row['epoch']
            if (last is not None and not 0<=now-last<=.04) or len(EARLY_ROWS)>=50000:os._exit(126)
            EARLY_ROWS.append(row)
            last=now;due+=.02;EARLY_STOP.wait(max(0,due-time.monotonic()))
    EARLY_THREAD=threading.Thread(target=early_observe,daemon=True);EARLY_THREAD.start()
import argparse, hashlib, importlib.abc, importlib.machinery, json

from pathlib import Path

ROOT=Path('/home/spenser/file-native-online-linux-20261020')
GUARD='4ddf6c15c87a200530c2c5e15d6c5be26ff3251019d20f8dea6ca83948b634f9'
OWNER='dfe8ef569b4981a76e199a2763de8a3cf9988bb24785b280fc3662bcd09427ce'

def need(test,message):
    if not test: raise RuntimeError(message)

def encoded(value):
    return json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False,allow_nan=False).encode()

def digest(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as f:
        for b in iter(lambda:f.read(1048576),b''): h.update(b)
    return h.hexdigest()

def strict(raw,limit=262144):
    need(len(raw)<=limit,'json-byte-bound')
    def pairs(items):
        answer={}
        for k,v in items:
            need(k not in answer,'duplicate-json-key');answer[k]=v
        return answer
    def bad(v): raise ValueError('nonfinite:'+v)
    return json.loads(raw,object_pairs_hook=pairs,parse_constant=bad)

def owned(path,base=ROOT):
    p=Path(path);need(p.is_absolute() and p.is_relative_to(base) and '..' not in p.parts,'owned-absolute-path')
    need(p.resolve()==p,'canonical-owned-path')
    current=Path(p.anchor)
    for part in p.parts[1:]:
        current/=part;need(not current.is_symlink(),'no-symlink-component')
    return p


class OwnedBirthUncertain(RuntimeError):
    """No descriptor from an uncertain observation is ever published or signalled."""
    def __init__(self,pid,start,stage,observed=None,reason=None):
        self.identity=dict(pid=pid,expected_start=start,stage=stage,observed_start=observed,reason=reason)
        super().__init__('owned-process-identity-uncertain',self.identity)

def retain_birth(starts,handles,row,lock,stage,parent=None):
    import native_guard as native
    pid=row['pid'];start=row['start']
    with lock:
        if pid in starts:
            if starts[pid]!=start or pid not in handles:
                raise OwnedBirthUncertain(pid,start,stage,starts.get(pid),'existing-birth-mismatch')
            return
        parent=parent or native.process(os.getpid(),with_memory=False)
        if parent is None or row['ppid']!=parent['pid'] or (parent['pid']!=os.getpid() and starts.get(parent['pid'])!=parent['start']):
            raise OwnedBirthUncertain(pid,start,stage,reason='unproved-parent-edge')
        def parent_live():
            p=native.process(parent['pid'],with_memory=False)
            if p is None or p['start']!=parent['start'] or p['state']=='Z':
                raise OwnedBirthUncertain(pid,start,stage,reason='parent-birth-not-live')
        fd=None;observed=None
        try:
            parent_live()
            fd=os.pidfd_open(pid)
            after=native.process(pid,with_memory=False)
            observed=None if after is None else after['start']
            values=[int(line.split(':',1)[1].split()[0]) for line in Path('/proc/self/fdinfo',str(fd)).read_text().splitlines() if line.startswith('Pid:')]
            if after is None or observed!=start or values!=[pid] or after['ppid']!=parent['pid']:
                raise OwnedBirthUncertain(pid,start,stage,observed,'post-open-birth-or-fdinfo-mismatch')
            parent_live()
        except BaseException as error:
            if fd is not None:os.close(fd)
            if isinstance(error,OwnedBirthUncertain):raise
            raise OwnedBirthUncertain(pid,start,stage,observed,type(error).__name__) from error
        starts[pid]=start;handles[pid]=fd


def observed_children(parent):
    import native_guard as native
    def live():
        p=native.process(parent['pid'],with_memory=False)
        if p is None or p['start']!=parent['start'] or p['state']=='Z':
            raise OwnedBirthUncertain(parent['pid'],parent['start'],'child-discovery',reason='parent-birth-not-live')
    live();pids=set()
    for task in Path('/proc',str(parent['pid']),'task').iterdir():
        try:pids.update(map(int,(task/'children').read_text().split()))
        except FileNotFoundError:pass
    rows=[]
    for pid in pids:
        row=native.process(pid)
        if row is None:continue
        if row['ppid']!=parent['pid']:
            raise OwnedBirthUncertain(pid,row['start'],'child-discovery',reason='fresh-parent-edge-changed')
        rows.append(row)
    live();return rows

class ClosedImports(importlib.abc.MetaPathFinder):
    def __init__(self,files): self.files=files
    def find_spec(self,fullname,path=None,target=None):
        spec=importlib.machinery.PathFinder.find_spec(fullname,path)
        if spec is not None and spec.origin and spec.origin not in ('frozen','built-in'):
            p=Path(spec.origin).absolute()
            if p.is_relative_to(ROOT):
                need(str(p) in self.files and digest(owned(p))==self.files[str(p)]['sha256'],'unbound-owned-import:'+fullname)
        return spec

def verify_sources(m):
    files=m['source_files'];need(len(files)==38 and hashlib.sha256(json.dumps(sorted(files),separators=(',',':')).encode()).hexdigest()=='4afeef6f8b3891bfe869732b8d9beb83d096491cc8eafde6f6b36d29334e0b7d','closed38-source-paths')
    if m.get('source_roots'):
        normalized={}
        for name,row in files.items():
            if type(row) is list:
                need(len(row)==2,'compact-source-record');row=dict(bytes=row[0],sha256=row[1])
            alias,relative=name.split('/',1)
            base=owned(m['source_roots'][alias],ROOT/'source')
            p=owned(base/relative,base)
            need(str(p) not in normalized,'unique-source-closure-path')
            normalized[str(p)]=row
        files=normalized;m['source_files']=files
    for name,row in files.items():
        p=owned(name,ROOT/'source');need(set(row)=={'bytes','sha256'},'source-record')
        need(p.stat().st_size==row['bytes'] and digest(p)==row['sha256'],'source-changed:'+name)
    for n,h in [('native_guard.py',GUARD),('native_driver.py',OWNER)]:
        p=ROOT/'source'/n;need(str(p) in files and files[str(p)]['sha256']==h,'accepted-source:'+n)
    required={'suite_bootstrap.py','suite_driver.py','suite_adapter.py','suite_oracle.py','suite_packing.py',
              'suite_recovery.py','suite_extensions.py','suite_ordinary.py','suite_units.py','suite_main.py'}
    folder=Path(__file__).resolve().parent
    need(all(str(folder/n) in files for n in required),'all-actual-suite-implementation-required')
    for folder_name in m['import_directories']: owned(folder_name,ROOT/'source')
    return files

def load(path,expected):
    path=owned(path,ROOT/'source');need(digest(path)==expected,'parent-manifest-binding')
    m=strict(path.read_bytes());need(m['version']==1 and m['platform']=='Linux','suite-format-platform')
    files=verify_sources(m)
    # Runtime remains NO_GO until the parent seals pin, body, all hooks and oracle.
    need(m['go'] is True and m['final_pin'] and len(m['final_pin'])==40,'parent-runtime-GO-required')
    scope=m.get('runtime_scope','full-body');need(scope in {'full-body','interface-controls-v5','private-cache-v5','workspace-v5','native-build-v5'},'closed-runtime-scope')
    if scope in {'interface-controls-v5','private-cache-v5','native-build-v5','workspace-v5'}:need(m['engine_case_qualification'] is False,'nonworkload-never-engine-qualification')
    else:
        need(m['candidate']['source_pin']==m['final_pin'],'same-final-candidate-pin')
        need(digest(owned(m['candidate']['path'],ROOT/'target'))==m['candidate']['sha256'],'candidate-binary-binding')
        need(m['oracle']['reviewed'] is True and m['oracle']['final_pin']==m['final_pin'],'final-independent-oracle-release')
    need(m['claim']['ledger_sha256'] and m['claim']['claim_id'],'sealed-runtime-claim')
    m['manifest_path']=str(path)
    m['manifest_sha256']=expected
    growth=owned(m['growth_path'],ROOT/'source')
    need(str(growth) in files and digest(growth)==files[str(growth)]['sha256'],'bound-growth-data')
    m['growth']=strict(growth.read_bytes(),65536)
    return m

def main():
    p=argparse.ArgumentParser();p.add_argument('--manifest',required=True);p.add_argument('--sha256',required=True)
    p.add_argument('--prepare-controls',action='store_true');p.add_argument('--authority');p.add_argument('--authority-sha256')
    p.add_argument('--phase',action='store_true');p.add_argument('--phase-body',action='store_true');p.add_argument('--phase-deadline',type=float)
    ns=p.parse_args();need(sys.platform=='linux' and __debug__ and sys.flags.isolated and sys.flags.no_site,'isolated-native-python')
    if ns.prepare_controls:
        path=owned(ns.manifest,ROOT/'source');need(digest(path)==ns.sha256,'seed-manifest-SHA')
        seed=strict(path.read_bytes());verify_sources(json.loads(json.dumps(seed)))
        authority=owned(ns.authority,ROOT/'source');need(digest(authority)==ns.authority_sha256,'parent-authority-SHA')
        result=encoded(control_config(seed,strict(authority.read_bytes())))
        need(len(result)<=16384,'sealed-control-config-byte-cap');sys.stdout.buffer.write(result);return
    m=load(ns.manifest,ns.sha256)
    need(not(ns.phase and ns.phase_body),'one-phase-entry-mode')
    sys.meta_path.insert(0,ClosedImports(m['source_files']))
    for item in reversed(m['import_directories']):sys.path.insert(0,item)
    os.environ['FILE_NATIVE_ORACLE_RESEARCH']=str(owned(m['oracle']['research_root'],ROOT/'source'))
    os.environ['PYTHONDONTWRITEBYTECODE']='1'
    if ns.phase:
        from suite_adapter import phase_launch
        return phase_launch(m,ns.sha256,PHASE_DEADLINE)
    if ns.phase_body:
        from suite_driver import suite_guard_main
        argv=[x.replace('@MANIFEST_PATH@',m['manifest_path']).replace('@MANIFEST_SHA256@',m['manifest_sha256']) for x in m['phase']['guard_argv_template']]
        returncode=suite_guard_main(ns.manifest,ns.sha256,argv)
        raise SystemExit(returncode)
    if m.get('runtime_scope') in {'native-build-v5','workspace-v5'}:
        raise RuntimeError('native-stage-runs-only-as-exact-reviewed-guard-command')
    if m.get('runtime_scope')=='private-cache-v5':
        from suite_packing import private_cache_clone
        return private_cache_clone(m)
    if m.get('runtime_scope')=='interface-controls-v5':
        from suite_extensions import interface_controls_v4
        interface_controls_v4(m,ns.sha256)
    else:
        from suite_main import run
        run(m,ns.sha256)









def verify_raw_packet(m,packet):
    from suite_extensions import verify_raw_packet as check
    return check(m,packet)

def verify_guard_memory(g):
    from suite_extensions import verify_guard_memory as check
    return check(g)


def control_config(seed,authority):
    """Pure concrete finite8 config builder. Parent must seal/review its output.
    Authority supplies exact paths, final pin, claim, phase and source policy;
    go defaults false and never follows merely from successful construction.
    """
    import copy
    m=copy.deepcopy(seed)
    for key in ['final_pin','claim','phase','native_guard_policy','control_root','evidence_root','tools']:
        need(key in authority,'missing-explicit-control-authority:'+key);m[key]=copy.deepcopy(authority[key])
    m['go']=authority.get('go',False);m['runtime_scope']='interface-controls-v5';m['engine_case_qualification']=False;m['private_cache']={'enabled':False}
    m['interface_peer']=dict(path=str(ROOT/'target/native-peer-v3-targeted'),sha256='fa2b5ae4ae2fe27bf3d5c320b2178bd3947ab01e1f028a2599fc8d38a224f770',source_pin='efd07cb5dee46a335be8c03a2f2c0462d7f923ca')
    m['control_forecast']=dict(kind='V5-finite8-peer-only',source_pin=m['final_pin'],bytes=1048576,files=128,directories=128)
    profile=dict(growth=dict(bytes=65536,files=16,directories=8),stdout_bytes=8192,stderr_bytes=8192,event_bytes=327680,input_bytes=262144,required_fields={},env_keys=['EXP_PEER_EARLY_JSON','EXP_PEER_EARLY_READY','EXP_PEER_PARTIAL_JSON','EXP_READY','EXP_RELEASE'])
    m['control_case']=dict(ceiling=dict(bytes=1048576,files=128,directories=128),calls={'resolve/':dict(profile,max_calls=7),'packing/build':dict(profile,max_calls=1)},controls=dict(max_controls=32))
    phase=m['phase'];phase['mode']='controls'
    output=owned(m['evidence_root'],ROOT/'evidence');boot=str(Path(m['source_roots']['suite'])/'suite_bootstrap.py')
    defaults={'owner_path':'phase-owner.json','io_lock_path':'phase-io.lock','memory_path':'phase-memory.jsonl.gz','receipt_path':'phase-final.json','packet_path':'controls-v5-report.json','guard_receipt_path':'controls-v5-guard.json','stdout_path':'phase-child.stdout','stderr_path':'phase-child.stderr'}
    for key,name in defaults.items():phase[key]=str(output/name)
    phase['memory_max_bytes']=131072;phase['receipt_max_bytes']=262144;phase['guard_receipt_max_bytes']=262144;phase['packet_max_bytes']=1048576;phase['caller_stream_max_bytes']=65536
    phase['guard_launch_template']=[m['python'],'-I','-S','-B',boot,'--phase-body','--manifest','@MANIFEST_PATH@','--sha256','@MANIFEST_SHA256@']
    p=m['native_guard_policy'];p['version']=5
    p['reserve']=['evidence=1048576:8' if x.startswith('evidence=') else x for x in p['reserve']]
    p['command_template']=[m['python'],'-I','-S','-B',boot,'--manifest','@MANIFEST_PATH@','--sha256','@MANIFEST_SHA256@']
    args=['--receipt',phase['guard_receipt_path'],'--seconds',str(p['seconds']),'--pin',m['final_pin'],'--claim-id',m['claim']['claim_id'],'--ledger-sha256',m['claim']['ledger_sha256']]
    for reserve in p['reserve']:args+=['--reserve',reserve]
    phase['guard_argv_template']=args+['--monitor-cargo','--cwd',str(ROOT),'--',*p['command_template']]
    keep={'version','platform','source_files','source_roots','import_directories','growth_path','oracle','python','native_guard_policy','private_cache','go','final_pin','runtime_scope','engine_case_qualification','claim','phase','control_root','evidence_root','interface_peer','control_forecast','control_case','tools'}
    return {k:m[k] for k in keep if k in m}


if __name__=='__main__':main()
