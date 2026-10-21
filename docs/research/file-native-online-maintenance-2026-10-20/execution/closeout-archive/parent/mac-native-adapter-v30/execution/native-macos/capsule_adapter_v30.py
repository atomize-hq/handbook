"""Private native capsule bridge; execution requires an explicit parent phase release.
JSON is the existing donor request/response and machine receipt boundary. Donor
assertions stay in their original modules. New IO is bounded and lossless.
"""
import argparse, ast, contextlib, gzip, hashlib, importlib.util, json, os
from pathlib import Path
import shutil, subprocess, sys, threading, time, types
import admission_v20 as admission
import role_driver_v29 as roles
from guard_v20 import Events

ROOT=admission.ROOT
HERE=Path(__file__).resolve().parent
READER_ACTIONS=frozenset(('resolve','point','inspect','lookup-bench'))
STABLE_PACKAGE=ROOT/'source/docs/research/file-native-incremental-engine-experiment-2026-10-18'
STABLE_TARGET=ROOT/'target'
_LOCK=threading.RLock()
_PENDING={k:{'bytes':0,'files':0} for k in admission.LIMITS}
_SERIAL=0

@contextlib.contextmanager
def reserve(category,size,files):
    with _LOCK:
        need={k:dict(v) for k,v in _PENDING.items()}
        need[category]['bytes']+=size;need[category]['files']+=files
        admission.admit(admission.snapshot(shared=False),need,admission.limits())
        _PENDING[category]['bytes']+=size;_PENDING[category]['files']+=files
    try: yield
    finally:
        with _LOCK:
            _PENDING[category]['bytes']-=size;_PENDING[category]['files']-=files

def fixture(path):
    path=Path(path).resolve()
    if not path.is_relative_to(ROOT/'fixtures') or Path(path).is_symlink():
        raise ValueError('adapter-fixture-path')
    return path

def report(path,value,cap=8*1024**2):
    path=Path(path)
    raw=(json.dumps(value,ensure_ascii=False,separators=(',',':'))+'\n').encode()
    if len(raw)>cap:raise RuntimeError('adapter-report-cap')
    category='F08' if path.is_relative_to(ROOT/'fixtures') else 'F12'
    if category=='F12' and not path.is_relative_to(ROOT/'evidence'):raise ValueError('adapter-report-path')
    with reserve(category,len(raw),1):
        path.parent.mkdir(parents=True,exist_ok=True)
        tmp=path.with_name(path.name+'.writing')
        with tmp.open('xb') as out:out.write(raw)
        os.replace(tmp,path)

class Bridge:
    def __init__(self,config):
        common={'control_only','phase','role_identity_budget','run_name','stdout_cap','event_cap','role_evidence_cap'}
        real={'engine','parent_phase_release','source_root','package_path','target_path','bindings','adapter_files','donor_core_files','libzstd','capsule_base'}
        if not isinstance(config,dict) or set(config)-(common|real|{'control_argv'}):raise ValueError('adapter-config-closed-shape')
        if not {'control_only','phase','role_identity_budget','run_name'} <= set(config):raise ValueError('adapter-config-required')
        if type(config['control_only']) is not bool or not isinstance(config['run_name'],str) or not config['run_name'].replace('-','').isalnum() or len(config['run_name'])>80:raise ValueError('adapter-config-control-name')
        if type(config['role_identity_budget']) is not int or not 1 <= config['role_identity_budget'] <= 4096:raise ValueError('adapter-config-identity-budget')
        if config['phase'] not in ('tiny','immediate','capsule','strict','focus','recovery'):raise ValueError('adapter-config-phase')
        if config['control_only'] and (config['phase']!='tiny' or set(config)&real):raise ValueError('adapter-control-only-shape')
        if not config['control_only'] and (not real-{'capsule_base'} <= set(config) or 'control_argv' in config or config['phase']=='tiny'):raise ValueError('adapter-real-config-required')
        for key,default,maximum in [('stdout_cap',4*1024**2,8*1024**2-65536),('event_cap',2*1024**2,8*1024**2),('role_evidence_cap',131072,1024**2)]:
            if type(config.get(key,default)) is not int or not 4096 <= config.get(key,default) <= maximum:raise ValueError('adapter-config-cap:'+key)
        self.config=config;self.control=config['control_only']
        self.prefix=config.get('control_argv') if self.control else [config['engine']]
        if self.control and (not self.prefix or Path(self.prefix[0]).resolve()!=Path(sys.executable).resolve()):raise ValueError('bridge-control-Python')
        if not self.control:
            if config.get('parent_phase_release') is not True:raise RuntimeError('adapter-parent-phase-release-HOLD')
            if Path(config['source_root']).resolve()!=ROOT/'source':raise ValueError('adapter-stable-source-root')
            if config['package_path']!=str(STABLE_PACKAGE) or config['target_path']!=str(STABLE_TARGET):raise ValueError('adapter-stable-build-paths')
            required={'capsule_adapter_v30.py','role_driver_v29.py','ordinary_copy_v27.sh','ordinary_route_v27.py','ordinary_manifest_v27.py'}
            if {x['path'] for x in config['adapter_files']} != required:raise ValueError('adapter-complete-source-file-set')
            for row in config['adapter_files']:
                p=HERE/row['path']
                if '/' in row['path'] or p.is_symlink() or roles.sha256(p)!=row['sha256']:raise ValueError('adapter-source-binding')
        self.phase=config['phase'];self.identity_budget=config['role_identity_budget']
        self.folder=ROOT/'evidence/capsules'/config['run_name'];self.folder.mkdir(parents=True,exist_ok=False)
        self.events=Events(self.folder/'calls.jsonl.gz',config.get('event_cap',2*1024**2));self.lock=threading.Lock()
        self.calls=[]
    def close(self):self.events.close()
    def invoke(self,role,argv,payload=None,code=0,env=None,timeout=115):
        global _SERIAL
        with _LOCK:_SERIAL+=1;number=_SERIAL
        name=self.config['run_name']+'-'+str(number)
        inp=None
        if payload is not None:
            inp=self.folder/(str(number)+'-input.json');report(inp,payload,cap=1024**2)
        binary=str(Path(argv[0]).resolve());binding=self.config.get('bindings',{}).get(binary,{})
        started=time.monotonic()
        try:
            with reserve('F12',self.config.get('stdout_cap',4*1024**2)+65536+self.config.get('role_evidence_cap',131072)+524288,4):
                result=roles.invoke(role,argv,name=name,timeout=min(timeout,115),identity_budget=self.identity_budget,
                                input_path=inp,expected_exit=code,env=env,control_only=self.control,
                                binding=binding.get('path'),binding_sha256=binding.get('sha256'),
                                evidence_cap=self.config.get('role_evidence_cap',131072),
                                stdout_cap=self.config.get('stdout_cap',4*1024**2),stderr_cap=65536)
        except BaseException as exc:
            with self.lock:self.events.emit({'kind':'call-censored','number':number,'role':role,'input':payload,'error':repr(exc),'seconds':time.monotonic()-started})
            raise
        out=Path(result['outputs']['stdout']['path']).read_bytes();err=Path(result['outputs']['stderr']['path']).read_bytes()
        if not result['output_complete']:raise RuntimeError('adapter-output-incomplete')
        row={'number':number,'role':role,'argv':argv,'input':payload,'env':env,'exit':result['exit'],
             'seconds':result['seconds'],'role_result':str(ROOT/'evidence/roles'/name/'result.json'),
             'outputs':result['outputs'],'raw_output_complete':True}
        with self.lock:self.events.emit(row);self.calls.append(row)
        return subprocess.CompletedProcess(argv,result['exit'],out,err),result
    def call(self,root,action,code=0,env=None,timeout=120,exe_override=None,**fields):
        root=fixture(root)
        if exe_override and Path(exe_override).resolve()!=Path(self.prefix[0]).resolve():raise ValueError('adapter-exe-override')
        payload={'root':str(root),'action':action,**fields}
        role='reader' if action in READER_ACTIONS else 'checker'
        x=admission.census(root)
        if action in READER_ACTIONS or action in ('dump','audit') or (action=='packing' and fields.get('operation')=='status'):bs,fs=65536,32
        elif action=='init':bs,fs=(8388608,1024) if fields.get('count',16)>16 else (2097152,256)
        elif action=='packing':bs,fs=3*x['logical_bytes']+1048576,512
        else:bs,fs=2*x['logical_bytes']+1048576,768
        with reserve('F08',bs,fs):p,r=self.invoke(role,list(self.prefix),payload,code,env,timeout)
        row={'input':payload,'exe':str(self.prefix[0]),'exe_sha256':r['binary_sha256'],'env':env,
             'exit':p.returncode,'seconds':r['seconds'],'stdout':p.stdout.decode('utf-8'),
             'stderr':p.stderr.decode('utf-8'),'sampled_peak_rss':r['peak_RSS_bytes'],
             'missing_samples':0,'rss_limit':r['limit_bytes'],'violations':r['failures'],
             'native_role_result':str(Path(r['outputs']['stdout']['path']).parent/'result.json')}
        return (admission.strict_json(p.stdout) if p.stdout else None),row
    def ordinary(self,root,name,captured=None,save=None,code=0,destination_base=None):
        if self.control:raise RuntimeError('ordinary-real-shell-binding-required')
        if captured:captured=fixture(captured)
        if save:save=fixture(save)
        dest=fixture(Path(destination_base or ROOT/'fixtures/ordinary')/(name+'-copy'))
        argv=['/bin/bash',str(HERE/'ordinary_copy_v27.sh'),sys.executable,str(HERE),str(fixture(root)),str(dest),str(captured or ''),str(save or '')]
        x=admission.census(root)
        with reserve('F08',2*x['logical_bytes']+1048576,2*x['files']+64):
            p,r=self.invoke('reader',argv,code=code,timeout=30)
        if code and b'selected-generation-missing' not in p.stderr:raise RuntimeError('ordinary-wrong-refusal')
        row={'name':name,'root':str(root),'destination':str(dest),'exit':p.returncode,
             'stdout':p.stdout.decode(),'stderr':p.stderr.decode(),'selection_reads':1,
             'engine_or_pack_decoder_used':False,'role_result':r}
        return (admission.strict_json(p.stdout) if not code else None),row

class Driver:
    bridge=None
    def __init__(self,base):
        self.base=fixture(base);self.base.mkdir(parents=True,exist_ok=False)
        self.events=[];self.lock=threading.Lock();self.bindings={}
    def call(self,root,action,**kw):
        result,row=self.bridge.call(root,action,**kw)
        with self.lock:self.events.append(row);report(self.base/'events.json',self.events)
        return result
    def head(self,root):return admission.strict_json((fixture(root)/'SELECTED.json').read_bytes())
    def publish(self,root,id,ops=None,packet=False,env=None,code=0):return self.call(root,'publish',code=code,env=env,request=dict(request=id,base=self.head(root),ops=ops or [],packet=packet))

def install_fixture_io():
    original_text=Path.write_text;original_bytes=Path.write_bytes
    def text(path,data,*args,**kw):
        if path.is_relative_to(ROOT/'fixtures'):
            if path.name=='events.json':return report(path,admission.strict_json(data))
            with reserve('F08',len(data.encode(kw.get('encoding') or 'utf-8')),1):return original_text(path,data,*args,**kw)
        return original_text(path,data,*args,**kw)
    def binary(path,data):
        if path.is_relative_to(ROOT/'fixtures'):
            with reserve('F08',len(data),1):return original_bytes(path,data)
        return original_bytes(path,data)
    Path.write_text=text;Path.write_bytes=binary
    original_tree=shutil.copytree;original_file=shutil.copyfile
    def tree(src,dst,*args,**kw):
        fixture(dst);x=admission.census(src)
        with reserve('F08',2*x['logical_bytes']+65536,2*x['files']+32):return original_tree(src,dst,*args,**kw)
    def file(src,dst,*args,**kw):
        fixture(dst)
        with reserve('F08',Path(src).stat().st_size+65536,4):return original_file(src,dst,*args,**kw)
    shutil.copytree=tree;shutil.copyfile=file

def run_phase(config):
    bridge=Bridge(config)
    try:
        if bridge.control:raise RuntimeError('control-cannot-run-real-capsules')
        source=Path(config['source_root'])/'docs/research';pack=source/'file-native-packing-2026-10-20'
        sys.path[:0]=[str(pack),str(source/'file-native-navigation-2026-10-19')]
        shim=types.ModuleType('guard');shim.ROOT=ROOT;shim.CUTOFF=roles.ancestor_token()[0]['fixed_cutoff'];sys.modules['guard']=shim
        import driver
        driver.Driver=Driver;driver.EXE=Path(config['engine']);Driver.bridge=bridge
        import ctypes.util
        lib=Path(config['libzstd']['path'])
        if lib.is_symlink() or roles.sha256(lib)!=config['libzstd']['sha256']:raise ValueError('adapter-libzstd-binding')
        original_find=ctypes.util.find_library
        ctypes.util.find_library=lambda name:str(lib) if name=='zstd' else original_find(name)
        import independent
        original_decode=independent.Zstd.decode;decode_lock=threading.Lock()
        def decode(self,data,length):
            with decode_lock:return original_decode(self,data,length)
        independent.Zstd.decode=decode
        import capsule
        install_fixture_io();phase=config['phase'];base=ROOT/'fixtures'/config['run_name'];ordinary=[]
        def copy(root,name,captured=None,save=None,code=0,destination_base=None):
            value,row=bridge.ordinary(root,name,captured,save,code,destination_base);ordinary.append(row);return value
        if phase=='capsule':
            result=capsule.run(base);assert len(result['cases'])==52 and result['events']==267
        elif phase=='strict':
            import strict_supplement
            with reserve('F08',25165824,1000):result=strict_supplement.run(fixture(config['capsule_base']),base)
            assert len(result['cases'])==24
        elif phase=='recovery':
            import loose_regression as lr
            def bounded(argv,input=None,env=None,**kw):
                payload=admission.strict_json(input);action=payload['action']
                return recovery_call(bridge,payload,env)
            lr.prior.subprocess=types.SimpleNamespace(run=bounded)
            result=lr.prior.run(Path(config['engine']),base);assert len(result['cases'])==24 and result['events']==282
        elif phase=='immediate':
            d=Driver(base/'events');cases=[]
            for mode in ('A','B'):
                for count in (16,128):
                    name=mode+str(count);root=base/name/'disposable/store';d.call(root,'init',mode=mode,consumer='project',count=count);d.call(root,'lifecycle',operation='setup')
                    before=copy(root,'immediate-'+name+'-before');packed=d.call(root,'packing',operation='build',codec='raw',include_attempts=True,cleanup=True);after=copy(root,'immediate-'+name+'-after')
                    assert before['selected']==after['selected'] and before['files']==after['files'];cases.append({'case':'immediate-'+name,'status':'passed'})
            result={'cases':cases,'events':len(d.events),'ordinary_shell':ordinary};assert len(cases)==4 and len(d.events)==12 and len(ordinary)==8
        elif phase=='focus':
            base.mkdir(exist_ok=False);d=Driver(base/'events');original=pack/'fix2_regression.py';tree=ast.parse(original.read_text());ns={'__name__':'native_frozen_focus','__file__':str(original)}
            for node in tree.body:
                if isinstance(node,(ast.Import,ast.ImportFrom)):exec(compile(ast.Module(body=[node],type_ignores=[]),str(original),'exec'),ns)
            binding={'exe_sha256':roles.sha256(config['engine']),'files':config['donor_core_files']}
            ns.update(base=base,here=pack,exe=Path(config['engine']),binding=binding,d=d,shell=ordinary,cases=[])
            ns['ordinary']=lambda root,name,captured=None,save=None,code=0:copy(root,'focus-'+name,captured,save,code,base)
            start=next(n.lineno for n in tree.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='root' for t in n.targets))
            skip=next(i+1 for i,line in enumerate(original.read_text().splitlines()) if line.startswith('hold=exclusive_delete_handle'))
            selected=[n for n in tree.body if n.lineno>=start and n.lineno!=skip]
            report(bridge.folder/'focus-AST-selection.json',{'frozen_source_sha256':roles.sha256(original),'selected_original_top_level_line_numbers':[n.lineno for n in selected],'only_case_skipped':'B-control-no-delete-sharing-remains-retained'})
            for node in selected:exec(compile(ast.Module(body=[node],type_ignores=[]),str(original),'exec'),ns)
            result=ns['result'];assert len(result['cases'])==8 and result['events']==25 and len(ordinary)==9
        else:raise ValueError('adapter-phase')
        report(bridge.folder/'summary.json',{'phase':phase,'result':result,'calls':len(bridge.calls),'status':'passed-pending-parent-review'})
        return result
    finally:bridge.close()

def recovery_call(bridge,payload,env):
    action=payload['action'];fields={k:v for k,v in payload.items() if k not in ('action','root')}
    _,row=bridge.call(payload['root'],action,code=(0,2,86),env={k:v for k,v in (env or {}).items() if k.startswith('EXP_')},**fields)
    return subprocess.CompletedProcess(bridge.prefix,row['exit'],row['stdout'].encode('utf-8'),row['stderr'].encode('utf-8'))

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--execute-config',required=True);a=p.parse_args()
    cfg=Path(a.execute_config)
    if cfg.is_symlink() or cfg.stat().st_size>262144:raise ValueError('adapter-config-bound')
    run_phase(admission.strict_json(cfg.read_bytes()))
