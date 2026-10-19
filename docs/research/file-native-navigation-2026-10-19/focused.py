"""Portable focused N delta handoff; ordinary children <=120s, capsule <=1800s."""
import argparse,hashlib,json,os,platform,subprocess,sys,threading,time
from pathlib import Path
from oracle import verify,navigation
HERE=Path(__file__).resolve().parent

def rss(pid):
    try:
        if os.name=='nt':
            import ctypes
            from ctypes import wintypes as w
            class M(ctypes.Structure):
                _fields_=[('cb',w.DWORD),('faults',w.DWORD)]+[(x,ctypes.c_size_t) for x in ['peak','rss','peakpage','page','peaknon','non','pagefile','peakfile']]
            k=ctypes.WinDLL('kernel32',use_last_error=True);k.OpenProcess.restype=w.HANDLE;k.CloseHandle.argtypes=[w.HANDLE]
            h=k.OpenProcess(0x410,False,pid)
            if not h:return None
            m=M();m.cb=ctypes.sizeof(m);get=ctypes.WinDLL('psapi').GetProcessMemoryInfo;get.argtypes=[w.HANDLE,ctypes.POINTER(M),w.DWORD];ok=get(h,ctypes.byref(m),m.cb);k.CloseHandle(h)
            return m.peak if ok else None
        if platform.system()=='Darwin':
            p=subprocess.run(['ps','-o','rss=','-p',str(pid)],capture_output=True,timeout=2);return int(p.stdout.strip())*1024 if p.stdout.strip() else None
        for line in Path(f'/proc/{pid}/status').read_text().splitlines():
            if line.startswith('VmHWM:'):return int(line.split()[1])*1024
    except (OSError,ValueError,subprocess.TimeoutExpired):return None

class Driver:
    def __init__(self,exe,base):
        self.exe=str(Path(exe).resolve());self.base=Path(base).resolve();self.base.mkdir(parents=True,exist_ok=False);self.events=[];self.start=time.monotonic()
    def call(self,root,action,code=0,profile=4,defer=False,exit_at=None,**fields):
        root=Path(root).resolve();assert root.is_relative_to(self.base)
        assert time.monotonic()-self.start<1800
        env={k:v for k,v in os.environ.items() if not k.startswith('EXP_')};env['EXP_NAVIGATION']=str(profile)
        if defer:env['EXP_DEFER_REFRESH']='1'
        if exit_at:env['EXP_EXIT']=exit_at
        payload=dict(root=str(root),action=action,**fields);begin=time.monotonic();peak=[0];stop=threading.Event();violation=[]
        p=subprocess.Popen([self.exe],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,env=env)
        limit=256*1024**2 if action in ['point','inspect','resolve'] else 2*1024**3
        def sample():
            while not stop.is_set():
                n=rss(p.pid)
                if n:peak[0]=max(peak[0],n)
                if n and n>limit:violation.append(n);p.kill();return
                stop.wait(.04)
        t=threading.Thread(target=sample,daemon=True);t.start()
        try:out,err=p.communicate(json.dumps(payload).encode(),timeout=120)
        except subprocess.TimeoutExpired:
            p.kill();out,err=p.communicate();violation.append('120s-censor')
        finally:stop.set();t.join(timeout=3)
        row=dict(input=payload,profile=profile,defer=defer,exit_at=exit_at,exit=p.returncode,seconds=time.monotonic()-begin,sampled_peak_rss=peak[0] or None,rss_limit=limit,violations=violation,stdout=out.decode(),stderr=err.decode());self.events.append(row)
        (self.base/'events.json').write_text(json.dumps(self.events,indent=2),encoding='utf-8')
        assert not violation and p.returncode==code,row
        return json.loads(out) if out else None
    def init(self,name,mode='A',n=128,profile=4):
        root=self.base/name/'disposable/store';self.call(root,'init',profile=profile,mode=mode,count=n,consumer='project');return root
    def head(self,root):return json.loads((root/'SELECTED.json').read_bytes())
    def publish(self,root,name,ops,packet=False,defer=False,profile=4):
        request=dict(request=name,base=self.head(root),ops=ops,packet=packet)
        prep=self.call(root,'prepare',profile=profile,request=request);candidate=prep['result']['candidate'];c=json.loads((root/candidate).read_bytes())
        selected=self.call(root,'select',profile=profile,candidate=candidate,defer=defer)
        return dict(prepare=prep,select=selected,replay_objects=len(c['replay_objects']),replay_files=len(c['replay_files']))

def run(exe,base,capsule=False,legacy=None):
    d=Driver(exe,base);cases=[]
    packaged=HERE/'legacy-fixtures.tar.gz'
    if legacy is None and packaged.exists():
        import tarfile
        target=d.base/'legacy-input'
        with tarfile.open(packaged) as archive:
            entries=archive.getmembers();assert len(entries)<10000 and sum(e.size for e in entries)<10*1024**2
            for e in entries:assert (target/e.name).resolve().is_relative_to(target.resolve()) and (e.isfile() or e.isdir())
            archive.extractall(target,filter='data')
        legacy=target/'ordinary'
    def check(root,label):
        d.call(root,'audit');row=dict(case=label,**verify(root));cases.append(row);(d.base/'cases.json').write_text(json.dumps(cases,indent=2),encoding='utf-8')
    for mode in ['A','B']:
        root=d.init('main-'+mode,mode);state=d.call(root,'dump')['result']['state'];check(root,'initial-'+mode)
        for label,ops in [('payload',[dict(kind='edit',id='d000004',text=' payload')]),('batch-payload',[dict(kind='edit',id=f'd{i:06}',text=' batch') for i in range(16,24)]),('label',[dict(kind='label',id='d000005',text='unexported')])]:
            r=d.publish(root,label,ops);assert r['prepare']['metrics']['navigation_parts']==0;check(root,label+'-'+mode)
        long='T'*4090
        r=d.publish(root,'local-split',[dict(kind='rename',id=f'd{i:06}',text=long+str(i)) for i in range(16,80)]);check(root,'byte-split-'+mode)
        assert len(list((root/('view' if mode=='A' else 'generations/'+json.loads((root/f"publications/{d.head(root)['publication']}.json").read_bytes())['attempt'])) .glob('navigation/groups/*.yaml')))>1
        d.publish(root,'local-merge',[dict(kind='rename',id=f'd{i:06}',text=state['docs'][f'd{i:06}']['title']) for i in range(16,80)]);check(root,'byte-merge-'+mode)
        d.publish(root,'high-degree',[dict(kind='edge',id='d000004',text='cites' if i%2 else 'requires',target='d000005') for i in range(130)]);check(root,'adjacency-duplicates-types-order-'+mode)
        d.publish(root,'remove-adjacency',[dict(kind='drop-edge',id='d000004',value=i) for i in range(129)]);check(root,'adjacency-collapse-'+mode)
        d.publish(root,'path-change',[dict(kind='path',id='d000005',text='docs/item-location-d000005/source.json')]);check(root,'target-location-'+mode)
        d.publish(root,'delete',[dict(kind='delete',id='d000004')]);check(root,'delete-'+mode)
        if mode=='A':
            managed=next((root/'view/navigation/groups').glob('*.yaml'));managed.write_bytes(b'external navigation bytes')
            d.publish(root,'external-edit',[dict(kind='rename',id='d000006',text='External repair')],defer=True)
            d.call(root,'refresh',code=86,exit_at='refresh-file');d.call(root,'resume');check(root,'external-interruption-'+mode)
            assert b'external navigation bytes' in [p.read_bytes() for p in (root/'divergence').rglob('*') if p.is_file()]
        # A selected source dialect supplies all discovery facts itself.
        root=d.init('source-complete-'+mode,mode,n=16);state=d.call(root,'dump')['result']['state'];ops=[];sources={}
        for id,doc in state['docs'].items():
            raw=json.dumps(dict(id=id,title=doc['title'],kind='self-describing-json',relationships=doc['edges']),ensure_ascii=False);sources[id]=raw
            ops.extend([dict(kind='kind',id=id,text='self-describing-json'),dict(kind='import',id=id,text=raw)])
        d.publish(root,'complete-sources',ops);check(root,'navigation-absent-'+mode)
        assert not (root/'view/navigation.yaml').exists() if mode=='A' else True
        assert verify(root)['supplemented_documents']==0
        d.publish(root,'packet-without-navigation',[],packet=True);d.call(root,'checkpoint',request_id='carry-profile');check(root,'absent-packet-replay-checkpoint-'+mode)
        d.publish(root,'source-no-longer-complete',[dict(kind='edit',id='d000004',text=' invalid JSON')]);check(root,'source-completeness-dirty-'+mode);assert verify(root)['supplemented_documents']==1
        d.publish(root,'source-complete-again',[dict(kind='import',id='d000004',text=sources['d000004'])]);check(root,'source-completeness-restored-'+mode)
        root=d.init('source-mixed-'+mode,mode,n=16);state=d.call(root,'dump')['result']['state'];doc=state['docs']['d000004']
        value=dict(id=doc['id'],title=doc['title'],kind='self-describing-json',relationships=doc['edges'])
        raw=json.dumps(value,ensure_ascii=False)
        d.publish(root,'mixed-one-complete',[dict(kind='kind',id=doc['id'],text='self-describing-json'),dict(kind='import',id=doc['id'],text=raw)])
        for label,newraw in [('whitespace',raw+'\n  '),('key-order',json.dumps(value,sort_keys=True,ensure_ascii=False))]:
            r=d.publish(root,'mixed-'+label,[dict(kind='import',id=doc['id'],text=newraw)])
            assert r['prepare']['metrics']['navigation_parts']==0 and r['select']['metrics']['navigation_parts']==0
            assert r['prepare']['metrics']['range_rows'].get('D',0)==0
            check(root,'mixed-complete-'+label+'-'+mode);assert verify(root)['supplemented_documents']==15
        # Frozen profile-2 candidates and genuinely pending A work survive.
        root=d.init('compatibility-'+mode,mode,n=16,profile=2);old=d.head(root);oldpath=root/f"publications/{old['publication']}.json";oldbytes=oldpath.read_bytes()
        d.publish(root,'sealed-profile2',[dict(kind='rename',id='d000004',text='Legacy sealed')],profile=2)
        if mode=='A':
            d.publish(root,'pending-profile2',[dict(kind='path',id='d000009',text='docs/item-attachments-d000009/evidence')],profile=2,defer=True)
            d.call(root,'refresh',code=86,profile=2,exit_at='refresh-plan');pending=(root/'PENDING.json').read_bytes()
        d.publish(root,'new-profile4',[],defer=mode=='A')
        if mode=='A':assert (root/'PENDING.json').read_bytes()==pending;d.call(root,'refresh')
        assert oldpath.read_bytes()==oldbytes;check(root,'frozen2-candidate-pending-upgrade-'+mode)
    if legacy:
        import shutil
        for mode in ['A','B']:
            root=d.base/('legacy1-'+mode);shutil.copytree(Path(legacy)/mode,root);old=d.head(root);p=root/f"publications/{old['publication']}.json";oldbytes=p.read_bytes()
            d.call(root,'audit');d.call(root,'select',candidate=json.loads((root/'FIXTURE-CANDIDATE.json').read_bytes()))
            d.publish(root,'upgrade-legacy1',[],defer=mode=='A')
            if mode=='A':d.call(root,'refresh')
            assert p.read_bytes()==oldbytes;check(root,'frozen1-candidate-upgrade-'+mode)
        packet_fixture=Path(legacy).parent/'packet'
        if packet_fixture.exists():
            for mode in ['A','B']:
                root=d.base/('legacy1-packet-'+mode);shutil.copytree(packet_fixture/mode,root)
                d.call(root,'audit');d.call(root,'select',candidate=json.loads((root/'FIXTURE-CANDIDATE.json').read_bytes()))
                d.publish(root,'upgrade-legacy1-packet',[],defer=mode=='A')
                if mode=='A':d.call(root,'refresh')
                check(root,'frozen1-packet-candidate-upgrade-'+mode)
    if capsule:
        # Reuse the actual recovery capsule, substituting only this independent
        # profile-4 oracle through Python's already-loaded `oracle` module.
        sys.path.insert(0,str(HERE.parent/'file-native-recovery-reclamation-2026-10-19'))
        import capsule as recovery
        from types import SimpleNamespace
        resources=[]
        def bounded(args,input=None,capture_output=False,env=None,timeout=None,**kw):
            remaining=1800-(time.monotonic()-d.start);assert remaining>0
            start=time.monotonic();p=subprocess.Popen(args,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,env=env,**kw);peak=[0];stop=threading.Event();bad=[]
            action=json.loads(input)['action'];limit=256*1024**2 if action in ['point','inspect','resolve'] else 2*1024**3
            def sample():
                while not stop.is_set():
                    n=rss(p.pid)
                    if n:peak[0]=max(peak[0],n)
                    if n and n>limit:bad.append('RSS');p.kill();return
                    stop.wait(.04)
            t=threading.Thread(target=sample,daemon=True);t.start()
            try:out,err=p.communicate(input,timeout=min(120,remaining))
            except subprocess.TimeoutExpired:p.kill();out,err=p.communicate();bad.append('time-censor')
            finally:stop.set();t.join(timeout=3)
            resources.append(dict(action=action,seconds=time.monotonic()-start,sampled_peak_rss=peak[0] or None,rss_limit=limit,violations=bad,exit=p.returncode))
            (d.base/'capsule-resources.json').write_text(json.dumps(resources,indent=2),encoding='utf-8');assert not bad
            return subprocess.CompletedProcess(args,p.returncode,out,err)
        recovery.subprocess=SimpleNamespace(run=bounded)
        recovery.run(Path(exe),d.base/'recovery-capsule')
    summary=dict(status='success',cases=cases,events=len(d.events),seconds=time.monotonic()-d.start,platform=platform.platform(),exe_sha256=hashlib.sha256(Path(exe).read_bytes()).hexdigest(),capsule=capsule,legacy1=bool(legacy))
    (d.base/'summary.json').write_text(json.dumps(summary,indent=2),encoding='utf-8');return summary

if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--exe',required=True);p.add_argument('--base',required=True);p.add_argument('--capsule',action='store_true');p.add_argument('--legacy-fixtures');a=p.parse_args();print(json.dumps(run(a.exe,a.base,a.capsule,a.legacy_fixtures),indent=2))
