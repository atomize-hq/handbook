import concurrent.futures,json,os,pathlib,shutil,subprocess,sys,time
import baseline_harness as b
def command(root,action,**kw):return dict(root=str(root),action=action,**kw)
def allocation_check(extra=0):
    if os.name=='nt':
        native=pathlib.Path('C:/file-native-followup-20261017');ledger=native/'allocation-ledger.json';platform='windows'
        builds=list(pathlib.Path('C:/').glob('file-native-followup-20261017-build*'))
    else:
        native=pathlib.Path('/home/spenser/file-native-followup-20261017');ledger=pathlib.Path('/mnt/c/file-native-followup-20261017/allocation-ledger.json');platform='wsl'
        builds=[] # ext4 builds are already below native
    values=b.load(ledger) if ledger.exists() else {}
    actual=b.inventory(native)['allocated_file_bytes']+sum(b.inventory(p)['allocated_file_bytes'] for p in builds)
    values[platform]=dict(allocated_file_bytes=actual,epoch=time.time())
    b.write(ledger,values)
    if sum(v['allocated_file_bytes'] for v in values.values())+extra>48*1024**3:raise RuntimeError('combined-new-run-allocation-cap')
    return values
def growth(d,args):
    reached=b.load(d.base/'growth-observations.json') if (d.base/'growth-observations.json').exists() else []
    allocated=0
    allocation_check()
    for n in args.tiers:
        for mode in args.modes:
            allocation_check(6*1024**3 if n==100000 else 1024**3)
            # Conservative 24 GiB per measured platform lane leaves the other
            # half of the declared 48 GiB allocation available for the peer.
            if allocated+(6*1024**3 if n==100000 else 1024**3)>24*1024**3:raise RuntimeError('per-platform-allocation-cap')
            d.budget(6*1024**3 if n==100000 else 1024**3)
            root=d.base/'growth'/f'{n}-{mode}-project';expected={1000:20,10000:10,100000:3}[n]
            if root.exists():
                info=d.inspect(root);done={p['request'] for p in info['history']}
            else:
                # Profile's measured 10k phases justify an instrumented 1,800 s
                # large initialization, rather than another blind 600 s repeat.
                d.init(root,mode,'project',n,timeout=(2400 if os.name!='nt' else 1800) if n==100000 else 600);done={'initial'}
            start=time.perf_counter_ns()
            batches=[1,64] if n==100000 else [1,8,64]
            operations=[]
            for packet in ([True] if n==100000 else [False,True]):
                for count in batches:
                    name=f'commit-{count}-packet-{packet}'
                    r={**b.request(name,b.batch_ops(count,name)), 'packet':packet}
                    operations.append((name,r))
            while len(operations)<expected-1:
                i=len(operations);name=f'commit-fill-{i}';operations.append((name,{**b.request(name,[b.edit('d000100',f'\n# retained {i}\n')]),'packet':False}))
            for name,r in operations:
                if name in done:continue
                # Native 10k WSL profiles were recorded before the large tier:
                # 189s initialization and 63-66s complete B commits. Keep finite
                # scaling margin for 100k without weakening any closure/sync.
                d.call(command(root,'publish',mode=mode,policy='slot',request=r),timeout=900 if n==100000 and os.name!='nt' else 600)
            info=d.inspect(root);assert len(info['history'])==expected
            sel=info['selected']
            # Packet work from an already selected revision, independently timed.
            packet=d.call(command(root,'packet',selected=sel),timeout=600)
            b.check_packet(root/packet['result']['path'])
            initial=b.load(root/'SELECTED.json');pub=b.load(root/'publications'/(initial['publication']+'.json'))
            history=info['history'];old=history[-1]['publication'];raw=(root/'publications'/(old+'.json')).read_bytes();oldsel=dict(publication=old,digest=b.sha(raw))
            for selected in [sel,oldsel]:
                for key in ['d000001','not-present']:
                    for _ in range(3):
                        point=d.call(command(root,'point',selected=selected,id=key));assert (point['result']['value'] is not None)==(key=='d000001')
            inv=b.inventory(root)
            allocated+=inv['allocated_file_bytes']
            allocation_check()
            row=dict(count=n,mode=mode,backend=d.backend,selected_generations=expected,inventory=inv,post_init_controller_ns=time.perf_counter_ns()-start)
            reached.append(row);b.write(d.base/'growth-observations.json',reached)
            print(f'growth {d.backend}/{n}/{mode}: {expected} states {inv["logical_bytes"]} bytes',flush=True)
    # Research domain has independent small/medium correctness and cost evidence.
    if 1000 in args.tiers:
        for mode in args.modes:
            root=d.base/'growth'/f'1000-{mode}-research';d.init(root,mode,'research',1000)
            for i in range(19):d.publish(root,mode,{**b.request(f'research-{i}',b.batch_ops([1,8,64][i%3],f'research-{i}')),'packet':i%2==0},'slot')
            assert len(d.inspect(root)['history'])==20
            reached.append(dict(count=1000,mode=mode,consumer='research',backend=d.backend,selected_generations=20,inventory=b.inventory(root)));b.write(d.base/'growth-observations.json',reached)
def contention(d,args):
    rows=[]
    for mode in ['A','B']:
        for policy in ['slot','speculative']:
            root=d.base/'contention'/f'{mode}-{policy}';d.init(root,mode,'project',128)
            for width in [1,2,4,8]:
                ready=root/f'wave-{width}';ready.mkdir()
                # Speculation overlaps at complete candidates. Admission overlaps
                # at the driver release before any worker is launched.
                requests=[{**b.request(f'wave-{width}-{i}',b.batch_ops(64,'slow') if i==0 else [b.edit(f'd{100+i:06}',f'\n# small {i}\n')]),'packet':True} for i in range(width)]
                pending=[]
                for i,r in enumerate(requests):
                    env={'EXP_BARRIER':'candidate-ready','EXP_READY':str(ready/f'{i}.ready'),'EXP_RELEASE':str(ready/'release')} if policy=='speculative' else {}
                    pending.append(d.start(command(root,'publish',mode=mode,policy=policy,request=r),env))
                if policy=='speculative':
                    for i,p in enumerate(pending):b.wait_path(ready/f'{i}.ready',p[0])
                    (ready/'release').write_text('go')
                starts=[p[2] for p in pending]
                outputs=[d.finish(p,allowed=(0,2)) for p in pending]
                for i,(r,out) in enumerate(zip(requests,outputs)):
                    attempts=[out];retries=0;completed=pending[i][0].observed[2]
                    while out['status']=='error' and retries<3:
                        assert out['error'].startswith(('busy:','conflict:'));out=d.publish(root,mode,r,policy);attempts.append(out);retries+=1;completed=d.events[-1]['end_monotonic_ns']
                    assert out['status']=='ok'
                    rows.append(dict(mode=mode,policy=policy,width=width,request=r['request'],complete_controller_ns=completed-starts[i],attempts=len(attempts),retries=retries,request_result=out['result']['result'],attempt_walls=[x['wall_ns'] for x in attempts],waits=[x['metrics']['ns'].get('lock-wait') for x in attempts],holds=[x['metrics']['ns'].get('lock-hold') for x in attempts]))
            # Waiting cancellation and process death holding coordination.
            ready=root/'HELD-READY';release=root/'HELD-RELEASE'
            held=d.start(command(root,'publish',mode=mode,policy='slot',request=b.request('killed-holder',[b.edit()])),{'EXP_BARRIER':'lock-acquired','EXP_READY':str(ready),'EXP_RELEASE':str(release)})
            b.wait_path(ready,held[0])
            busy=d.call(command(root,'publish',mode=mode,policy='slot',request=b.request('busy',[b.edit()])),env={'EXP_WAIT_MS':'80'},allowed=(2,));assert busy['metrics']['ns']['lock-wait']>=80_000_000
            queued=d.start(command(root,'publish',mode=mode,policy='slot',request=b.request('queued',[b.edit()])),{'EXP_CANCEL_ID':'queued'})
            time.sleep(.03);d.call(command(root,'cancel-signal',request_id='queued'));cancel=d.finish(queued,allowed=(2,));assert cancel['error']=='cancelled:while-waiting'
            held[0].kill();d.finish(held,allowed=(1,86,-9,4294967295))
            d.call(command(root,'cancel',request_id='queued'));fenced=d.publish(root,mode,b.request('queued',[b.edit()]),'slot',allowed=(2,));assert 'cancelled' in fenced['error']
            d.publish(root,mode,b.request('after-death',[b.edit()]),'slot');d.inspect(root,True)
            rows.append(dict(mode=mode,policy=policy,cancellation='wait then durable fence',busy_wait_ns=busy['metrics']['ns']['lock-wait'],holder_death='lock released',inventory=b.inventory(root)))
    b.write(d.base/'contention-observations.json',rows)
def navigation(d,args):
    # The actual PowerShell/Bash scripts are invoked by the orchestration. This
    # prepares positively selected paths and independently verifies extracted data.
    rows=[]
    for mode in ['A','B']:
        root=d.base/'navigation'/mode;d.init(root,mode,'research',16);sel,p,s=b.selected_state(root)
        fixed=root/(p['candidate']['closure'] if mode=='B' else 'view')
        packet=root/p['candidate']['closure'];copy=d.base/'extracted'/mode;b.ordinary_copy(packet,copy);b.check_packet(copy)
        manifest=b.load(copy/'MANIFEST.json');assert all(b.sha((copy/path).read_bytes())==h for path,h in manifest['files'].items())
        rows.append(dict(mode=mode,root=str(root),selected=sel,readable=str(fixed),packet=str(packet),copy=str(copy),manifest_digest=p['candidate']['manifest_digest'],closure_files=len(manifest['files']),grammar='fixture Markdown links and explicit NAVIGATION locators'))
        cfg=dict(root=str(root),mode=mode,destination=str(d.base/'suspended-copy'/mode),ready=str(root/'COPY-READY'),release=str(root/'COPY-RELEASE'))
        child=subprocess.Popen([sys.executable,str(pathlib.Path(__file__).parent/'terminal_copy.py'),json.dumps(cfg)],stdout=subprocess.PIPE,stderr=subprocess.PIPE)
        b.wait_path(root/'COPY-READY',child);d.publish(root,mode,b.request('copy-overlap',[b.edit(),b.edit('d000001')]),'slot')
        (root/'COPY-RELEASE').write_text('go');out,err=child.communicate(timeout=60);assert child.returncode==0,err
        observed=json.loads(out);new=b.selected_state(root)[2]
        assert observed['first']==s['docs']['d000000']['digest'] and observed['second']==(s if mode=='B' else new)['docs']['d000001']['digest']
        rows[-1]['suspended_copy']=observed;rows[-1]['copy_observation']='fixed coherent' if mode=='B' else 'mixed mutable'
        if mode=='B':
            copied=pathlib.Path(cfg['destination']);assert b.sha((copied/'MANIFEST.json').read_bytes())==p['candidate']['manifest_digest'];m=b.load(copied/'MANIFEST.json');assert all(b.sha((copied/n).read_bytes())==h for n,h in m['files'].items());b.check_packet(copied)
    b.write(d.base/'navigation-paths.json',rows)
