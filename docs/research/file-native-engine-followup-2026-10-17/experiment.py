"""New bounded driver. No expired total deadline and no reused initialization roots."""
import argparse, base64, concurrent.futures, hashlib, json, os, pathlib, platform, shutil
import subprocess, sys, threading, time, traceback
import baseline_harness as b
H=pathlib.Path(__file__).resolve().parent
class Driver(b.Driver):
    def __init__(self,base,exe,label,backend='manifest'):
        self.base,self.exe,self.label=pathlib.Path(base),pathlib.Path(exe),label
        self.base.mkdir(parents=True,exist_ok=True);self.log=self.base/(label+'-raw.jsonl');self.events=[]
        self.backend=backend;self.serial=0;self.mutex=threading.Lock()
        self.session=time.time_ns()
        self.binary_sha256=b.sha(self.exe.read_bytes())
        harness={name:b.sha((H/name).read_bytes()) for name in ['experiment.py','baseline_harness.py','workloads.py','regressions.py','terminal_copy.py']}
        self.harness_sha256=b.sha(json.dumps(harness,sort_keys=True).encode())
        b.write(self.base/f'harness-session-{self.session}.json',dict(files=harness,sha256=self.harness_sha256))
    def budget(self,extra=0):
        if shutil.disk_usage(self.base).free-extra<160*1024**3:raise RuntimeError('host/guest-reserve')
        if os.name!='nt' and pathlib.Path('/mnt/c').exists() and shutil.disk_usage('/mnt/c').free-extra<160*1024**3:raise RuntimeError('windows-host-reserve')
    def start(self,c,env=None):
        self.budget();start=time.perf_counter_ns()
        with self.mutex:self.serial+=1;seq=self.serial
        environment={**os.environ,'EXP_BACKEND':self.backend,'EXP_LAYOUT':'sharded',**(env or {})}
        progress=self.base/f'{self.label}-{self.session}-{seq:06}-progress.log';errfile=progress.open('xb')
        p=subprocess.Popen([str(self.exe)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=errfile,env=environment)
        p.stdin.write(json.dumps(c,ensure_ascii=False).encode());p.stdin.close();p.stdin=None
        def collect():
            a,_=p.communicate();end=time.perf_counter_ns();errfile.close();p.observed=(a,progress.read_bytes(),end)
        p.collector=threading.Thread(target=collect,daemon=True);p.collector.start()
        return p,c,start,seq,env or {}
    def finish(self,pending,timeout=120,allowed=(0,)):
        p,c,start,seq,env=pending
        p.collector.join(max(0,timeout-(time.perf_counter_ns()-start)/1e9))
        censored=p.collector.is_alive()
        if censored:p.kill();p.collector.join()
        a,z,end=p.observed;out=None;parse_error=None
        if a.strip():
            try:out=json.loads(a)
            except (ValueError,UnicodeError) as error:parse_error=f'{type(error).__name__}: {error}'
        elif p.returncode==0:parse_error='empty stdout from a successful process'
        event=dict(session=self.session,seq=seq,command=c,env=env,backend=self.backend,exit=p.returncode,censored=censored,start_monotonic_ns=start,end_monotonic_ns=end,external_wall_ns=end-start,output=out,stdout_base64=base64.b64encode(a).decode('ascii'),stdout_bytes=len(a),stdout_sha256=b.sha(a),stdout_parse_error=parse_error,stderr=z.decode(errors='replace'),binary_sha256=self.binary_sha256,harness_sha256=self.harness_sha256)
        self.record(event)
        if censored:raise TimeoutError(str(c))
        if parse_error:raise RuntimeError('malformed engine response; exact stdout retained in raw event: '+parse_error)
        assert p.returncode in allowed,event
        return out
    def init(self,root,mode,consumer,count=12,packet=True,timeout=600):
        return self.call(dict(action='init',root=str(root),mode=mode,consumer=consumer,count=count,packet=packet),timeout=timeout)
def cmd(root,action,**kw):return dict(root=str(root),action=action,**kw)
def lifecycle(d,root,op,allowed=(0,),**kw):return d.call(cmd(root,'lifecycle',operation=op,**kw),allowed=allowed)
def semantics(d,reps=3):
    rows=[]
    for mode in ['A','B']:
        for consumer in ['project','research']:
            for trial in range(reps):
                cases=[('delete-edge',[dict(kind='delete',id='d000010')],[dict(kind='edge',id='d000004',text='cites',target='d000010')],'semantic:local-reference'),('unique',[dict(kind='label',id='d000004',text='shared')],[dict(kind='label',id='d000006',text='shared')],'semantic:unique'),('budget',[dict(kind='cost',id='d000004',value=600)],[dict(kind='cost',id='d000006',value=600)],'semantic:budget'),('rename-edge',[dict(kind='rename',id='d000010',text='CON. café α')],[dict(kind='edge',id='d000004',text='cites',target='d000010')],None)]
                for name,left,right,error in cases:
                    root=d.base/'semantic'/f'{mode}-{consumer}-{trial}-{name}';d.init(root,mode,consumer,16);sel=b.selected_state(root)[0]
                    pending=[d.start(cmd(root,'prepare',mode=mode,request=b.request(side,ops,sel))) for side,ops in [('left',left),('right',right)]]
                    prep=[d.finish(p)['result'] for p in pending]
                    d.call(cmd(root,'select',candidate=prep[0]['file']))
                    out=d.call(cmd(root,'select',candidate=prep[1]['file']),allowed=(2,));assert out['error']=='conflict:expected-publication'
                    out=d.publish(root,mode,b.request('right',right,b.selected_state(root)[0]),allowed=(2,) if error else (0,))
                    if error:assert out['error']==error
                    d.inspect(root,True);rows.append(dict(mode=mode,consumer=consumer,trial=trial,case=name,result=error or 'accepted'))
    b.write(d.base/'semantic-observations.json',rows)
def io_checks(d):
    root=d.base/'io';d.init(root,'A','research')
    d.call(cmd(root,'io-probe'),env={'EXP_EXIT':'directory-create'},allowed=(86,))
    d.call(cmd(root,'io-probe',short=True),allowed=(2,))
    pending=[d.start(cmd(root,'io-probe')) for _ in range(8)]
    results=[d.finish(p) for p in pending];assert len({r['result']['sha256'] for r in results})==1
    before=(root/'probe/nested/content').stat()
    d.call(cmd(root,'io-probe'));after=(root/'probe/nested/content').stat()
    assert before.st_ino==after.st_ino
    bad=d.call(cmd(root,'io-probe',text='different content'),allowed=(2,));assert 'integrity-digest' in bad['error']
    # Selected corruption is detected and preserved, never silently repaired.
    saved=(root/'probe/nested/content').read_bytes();(root/'probe/nested/content').write_bytes(b'corrupt')
    bad=d.call(cmd(root,'io-probe'),allowed=(2,));assert 'integrity-digest' in bad['error']
    b.write(d.base/'io-observations.json',dict(writers=8,stable_inode=True,short_retry=True,nested_mkdir_retry=True,preexisting_parent_sync=results[0]['metrics']['directory_syncs'],corruption_refused=bad['error'],saved_sha256=b.sha(saved)))
def retention(d):
    rows=[]
    for mode in ['A','B']:
        root=d.base/'disposable'/mode;d.init(root,mode,'research',128);old,oldpub,olds=b.selected_state(root)
        lifecycle(d,root,'setup');lifecycle(d,root,'pin',selected=old,name='suspended-tool-reader')
        # A real ordinary copier binds the exact retained directory once. A uses
        # its retained packet here; mutable-view mixing is tested separately.
        reader_dest=d.base/'suspended-retention-copy'/mode
        cfg=dict(root=str(root),mode='B',destination=str(reader_dest),ready=str(root/'READER-READY'),release=str(root/'READER-RELEASE'))
        reader=subprocess.Popen([sys.executable,str(H/'terminal_copy.py'),json.dumps(cfg)],stdout=subprocess.PIPE,stderr=subprocess.PIPE)
        b.wait_path(root/'READER-READY',reader)
        packet=d.call(cmd(root,'packet',selected=old))['result'];external=d.base/'retained-outside'/mode;b.ordinary_copy(root/packet['path'],external);b.check_packet(external)
        d.publish(root,mode,b.request('replacement',[b.edit('d000004')]))
        rebound=lifecycle(d,root,'pin',name='suspended-tool-reader',allowed=(2,));assert rebound['error']=='refused:pin-name-already-bound'
        refused=lifecycle(d,root,'retire',selected=old,allowed=(2,));assert 'retained-snapshot' in refused['error']
        lifecycle(d,root,'collect');(root/'READER-RELEASE').write_text('go');reader_out,reader_error=reader.communicate(timeout=60);assert reader.returncode==0,reader_error
        manifest=b.load(reader_dest/'MANIFEST.json');assert b.sha((reader_dest/'MANIFEST.json').read_bytes())==oldpub['candidate']['manifest_digest'];assert all(b.sha((reader_dest/n).read_bytes())==h for n,h in manifest['files'].items());b.check_packet(reader_dest)
        lifecycle(d,root,'release',name='suspended-tool-reader')
        refused=lifecycle(d,root,'retire',selected=old,allowed=(2,));assert 'retained-snapshot' in refused['error']
        lifecycle(d,root,'release-packet',path=packet['path'])
        # Registration/retirement serialize: paused registration holds writer lock.
        ready=root/'PIN-READY';release=root/'PIN-RELEASE'
        pending=d.start(cmd(root,'lifecycle',operation='pin',selected=old,name='racing-reader'),{'EXP_BARRIER':'pin-registered','EXP_READY':str(ready),'EXP_RELEASE':str(release)})
        b.wait_path(ready,pending[0]);refused=d.call(cmd(root,'lifecycle',operation='retire',selected=old),env={'EXP_WAIT_MS':'60'},allowed=(2,));assert 'busy:lock-timeout' in refused['error']
        release.write_text('go');d.finish(pending);lifecycle(d,root,'release',name='racing-reader')
        # A paused preparation holds a distinct shared gate; reclamation refuses.
        ready=root/'PREP-READY';release=root/'PREP-RELEASE'
        pending=d.start(cmd(root,'prepare',mode=mode,request=b.request('pending',[b.edit('d000006')])),{'EXP_BARRIER':'cow-root-prepared' if d.backend=='cow' else 'state-installed','EXP_READY':str(ready),'EXP_RELEASE':str(release)})
        b.wait_path(ready,pending[0]);refused=lifecycle(d,root,'collect',allowed=(2,));assert 'active-preparation' in refused['error'];release.write_text('go');candidate=d.finish(pending)['result']
        lifecycle(d,root,'collect');d.call(cmd(root,'select',candidate=candidate['file']))
        pages_before={p.relative_to(root).as_posix() for p in (root/'objects').rglob('*') if p.is_file()}
        lost=d.call(cmd(root,'resolve',request_id='initial'))['result']
        out=d.call(cmd(root,'lifecycle',operation='retire',selected=old),env={'EXP_EXIT':'retirement-gate-durable'},allowed=(86,))
        # Authoritative retirement must survive even if discoverability cache was
        # never written. Audit and explicit historical reads agree after reopen.
        d.inspect(root,True)
        out=d.call(cmd(root,'point',selected=old,id='d000001'),allowed=(2,));assert out['error']=='retired:snapshot'
        pending=d.start(cmd(root,'lifecycle',operation='collect'),{'EXP_BARRIER':'reclaim-file','EXP_READY':str(root/'GC-READY'),'EXP_RELEASE':str(root/'GC-RELEASE')})
        b.wait_path(root/'GC-READY',pending[0]);pending[0].kill();d.finish(pending,allowed=(1,86,-9,4294967295))
        collected=lifecycle(d,root,'collect');d.inspect(root,True)
        pages_after={p.relative_to(root).as_posix() for p in (root/'objects').rglob('*') if p.is_file()}
        if d.backend=='cow':assert pages_before-pages_after,'actual obsolete COW pages must be reclaimed'
        assert d.call(cmd(root,'resolve',request_id='initial'))['result']==lost;b.check_packet(external)
        duplicate=d.call(cmd(root,'select',candidate=f"attempts/{oldpub['candidate']['attempt']}/candidate.json"));assert duplicate['result']['result']==lost
        refused=lifecycle(d,root,'pin',selected=old,name='too-late',allowed=(2,));assert 'retired:' in refused['error']
        rows.append(dict(mode=mode,backend=d.backend,retired=old,suspended_ordinary_reader=json.loads(reader_out),interrupted_sweep_reopened=True,collection=collected,actual_pages_reclaimed=sorted(pages_before-pages_after),actual_pages_retained=len(pages_after),external_packet='verified',original_outcome=lost))
    b.write(d.base/'retention-observations.json',rows)
    agreement=dict(roots={r['mode']:r['retired'] for r in rows},packets={r['mode']:str(d.base/'retained-outside'/r['mode']) for r in rows},guarantee='independently captured retained copies; no global atomicity')
    b.write(d.base/'retained-outside/agreement.json',agreement)
    for packet in agreement['packets'].values():b.check_packet(pathlib.Path(packet))
def damaged_view(d):
    root=d.base/'damaged-view';d.init(root,'A','research',16);sel,p,s=b.selected_state(root)
    first=root/'view'/s['docs']['d000000']['path'];second=root/'view'/s['docs']['d000001']['path']
    first.write_bytes(b'UNADMITTED USER EDIT\r\n');second.unlink()
    (root/'view/NAVIGATION.md').write_bytes(b'UNADMITTED NAVIGATION EDIT\r\n');(root/'view/packet/context.md').unlink()
    out=d.call(cmd(root,'view-audit'))['result'];assert set(out['edited'])=={'d000000','NAVIGATION.md'} and set(out['missing'])=={'d000001','packet/context.md'};d.inspect(root,True)
    d.call(cmd(root,'resume'),env={'EXP_EXIT':'view-divergence-preserved'},allowed=(86,));d.call(cmd(root,'resume'));assert b.selected_state(root)[0]==sel
    kept=[p for p in (root/'divergence').rglob('*') if p.is_file()];assert any(p.read_bytes()==b'UNADMITTED USER EDIT\r\n' for p in kept)
    assert any(p.read_bytes()==b'UNADMITTED NAVIGATION EDIT\r\n' for p in kept)
    out=d.call(cmd(root,'view-audit'))['result'];assert not out['missing'] and not out['edited']
    # Cross-depth attachment move must refuse the exact source's invalid locator.
    out=d.publish(root,'A',b.request('bad-depth',[dict(kind='rename',id='d000009',text='moved')]),allowed=(2,));assert 'missing-readable-link' in out['error'] or 'link-escape' in out['error']
    b.write(d.base/'damaged-view-observations.json',dict(selected=sel,preserved=[str(p) for p in kept],rebuild='render-only',cross_depth=out['error']))
def correctness(d):
    b.races(d,3);semantics(d);b.fault_suite(d);b.special_checks(d);io_checks(d);retention(d);damaged_view(d);duplicate_delete(d);extended_faults(d);orphan_reads(d)
    b.write(d.base/'correctness-completed.json',dict(backend=d.backend,platform=platform.platform(),status='passed'))
def duplicate_delete(d):
    rows=[]
    for policy in ['slot','speculative']:
        root=d.base/'duplicate-delete'/policy;d.init(root,'B','research',16)
        request=b.request('once-delete',[dict(kind='delete',id='d000010')]);ready=root/'READY';release=root/'RELEASE'
        pending=d.start(cmd(root,'publish',mode='B',policy=policy,request=request),{'EXP_BARRIER':'request-unresolved','EXP_READY':str(ready),'EXP_RELEASE':str(release)})
        b.wait_path(ready,pending[0]);winner=d.publish(root,'B',request,'slot')['result']['result']
        d.call(cmd(root,'cancel-signal',request_id='once-delete'))
        release.write_text('go');loser=d.finish(pending)['result'];assert loser['status']=='deduplicated' and loser['result']==winner
        assert len(d.inspect(root)['history'])==2;rows.append(dict(policy=policy,result=winner,late_signal='does not cancel accepted request'))
    b.write(d.base/'duplicate-delete-observations.json',rows)
def orphan_reads(d):
    root=d.base/'orphan-reads';d.init(root,'B','research',16);before=set((root/'publications').glob('*.json'))
    d.publish(root,'B',b.request('unselected',[b.edit()]),env={'EXP_EXIT':'publication-installed'},allowed=(86,))
    orphan=(set((root/'publications').glob('*.json'))-before).pop();sel=dict(publication=orphan.stem,digest=b.sha(orphan.read_bytes()))
    for action in ['point','packet','lifecycle']:
        out=d.call(cmd(root,action,selected=sel,id='d000001',operation='pin',name='orphan'),allowed=(2,));assert out['error']=='refused:not-selected-lineage'
    assert d.call(cmd(root,'resolve',request_id='unselected'))['result']['outcome']=='absent-as-of'
    b.write(d.base/'orphan-read-observations.json',dict(unselected=sel,refused=['point','packet','pin']))
def extended_faults(d):
    rows=[]
    for mode in ['A','B']:
        hooks=[('view-copy-write' if mode=='A' else 'generation-copy-write',i) for i in [1,8,16]]
        if mode=='A':hooks.extend([('view-derived-written',i) for i in [1,2]])
        if d.backend=='cow':hooks.extend([('cow-object-installed',1),('cow-root-prepared',1)])
        for hook,nth in hooks:
            for fault in ['exit','error','kill']:
                root=d.base/'extended-faults'/f'{mode}-{hook}-{nth}-{fault}';d.init(root,mode,'project',16);old=b.selected_state(root)[0]
                c=cmd(root,'publish',mode=mode,policy='speculative',request=b.request('fault',b.batch_ops(16,'first-middle-last')))
                env={'EXP_NTH':str(nth),'EXP_TRACE':'1'}
                if fault=='kill':
                    env.update(EXP_BARRIER=hook,EXP_READY=str(root/'READY'),EXP_RELEASE=str(root/'RELEASE'));pending=d.start(c,env);b.wait_path(root/'READY',pending[0]);pending[0].kill();d.finish(pending,allowed=(1,86,-9,4294967295))
                else:env['EXP_'+fault.upper()]=hook;d.call(c,env=env,allowed=(0,2,86))
                new=b.selected_state(root)[0];post=mode=='A' and hook in ['view-copy-write','view-derived-written'];assert (new!=old)==post,(mode,hook,nth,fault)
                if post:d.call(cmd(root,'resume'))
                else:d.publish(root,mode,b.request('fault',b.batch_ops(16,'first-middle-last')),'slot')
                d.inspect(root,True);rows.append(dict(mode=mode,hook=hook,nth=nth,fault=fault,oracle='must-retain' if post else 'must-not-select'))
    if os.name!='nt':
        root=d.base/'unix-link-interruption';d.init(root,'A','research');d.call(cmd(root,'io-probe'),env={'EXP_EXIT':'immutable-name-installed'},allowed=(86,));before=(root/'probe/nested/content').stat().st_ino;d.call(cmd(root,'io-probe'));assert before==(root/'probe/nested/content').stat().st_ino
        rows.append(dict(hook='immutable-name-installed',oracle='same inode retained on retry'))
    b.write(d.base/'extended-fault-observations.json',rows)
def main():
    ap=argparse.ArgumentParser();ap.add_argument('phase',choices=['correctness','growth','contention','navigation']);ap.add_argument('--base',type=pathlib.Path,required=True);ap.add_argument('--exe',type=pathlib.Path,required=True);ap.add_argument('--backend',choices=['manifest','cow'],default='manifest');ap.add_argument('--tiers',type=int,nargs='+',default=[1000,10000,100000]);ap.add_argument('--modes',nargs='+',default=['A','B']);args=ap.parse_args()
    d=Driver(args.base,args.exe,args.phase,args.backend)
    try:
        if args.phase=='correctness':correctness(d)
        else:
            from workloads import growth,contention,navigation
            {'growth':growth,'contention':contention,'navigation':navigation}[args.phase](d,args)
    except Exception as e:
        b.write(d.base/(args.phase+'-stopped.json'),dict(error=str(e),traceback=traceback.format_exc(),time=time.time()));raise
if __name__=='__main__':main()
