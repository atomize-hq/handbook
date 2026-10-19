"""Focused fresh-process correctness and full operational restoration capsule."""
import argparse,hashlib,json,os,platform,subprocess,time
from pathlib import Path
from operational_backup import capture,restore
from oracle import verify,navigation

def run(exe,scratch):
    scratch=Path(scratch).resolve();scratch.mkdir(parents=True,exist_ok=False);events=[];cases=[];started=time.monotonic()
    def call(root,action,code=0,exit_at=None,defer=False,**fields):
        assert Path(root).resolve().is_relative_to(scratch)
        assert time.monotonic()-started<1800
        payload=dict(root=str(root),action=action,**fields);env={k:v for k,v in os.environ.items() if not k.startswith('EXP_')}
        if exit_at:env['EXP_EXIT']=exit_at
        if defer:env['EXP_DEFER_REFRESH']='1'
        begin=time.monotonic();p=subprocess.run([str(exe)],input=json.dumps(payload).encode(),capture_output=True,env=env,timeout=180)
        row=dict(input=payload,exit_at=exit_at,defer=defer,exit=p.returncode,seconds=time.monotonic()-begin,stdout=p.stdout.decode(),stderr=p.stderr.decode());events.append(row)
        (scratch/'events.json').write_text(json.dumps(events,indent=2),encoding='utf-8')
        assert p.returncode==code,row
        return json.loads(p.stdout) if p.stdout else None
    def selected(root):return json.loads((root/'SELECTED.json').read_bytes())
    def pub(root,id,ops,**kw):return call(root,'publish',request=dict(request=id,base=selected(root),ops=ops,packet=False),**kw)
    def reference(root):return call(root,'dump')['result']['state']
    def init(name,mode='A',n=16):
        root=scratch/name/'disposable/store';call(root,'init',mode=mode,count=n,consumer='project');call(root,'lifecycle',operation='setup');return root
    ancestor='docs/item-attachments-d000009/evidence';deep=ancestor+'/deep/source.txt'
    try:
        for mode in ['A','B']:
            for n in [16,128]:
                root=init(f'main-{mode}-{n}',mode,n);initial=reference(root)
                verify(root,initial)
                # Ordered duplicate local relationships plus captured external edges.
                result=pub(root,'semantic-change',[dict(kind='delete',id='d000000'),dict(kind='rename',id='d000001',text='Renamed title'),dict(kind='edge',id='d000001',text='cites',target='d000002'),dict(kind='edge',id='d000001',text='cites',target='d000002'),dict(kind='path',id='d000009',text=ancestor)])
                assert result['result']['status']=='published',result
                state=reference(root);assert 'd000000' not in state['docs'];assert state['docs']['d000001']['title']=='Renamed title'
                assert state['docs']['d000001']['edges'][-2:]==[dict(kind='cites',target='d000002',external=False)]*2
                verify(root,state)
                pub(root,'inverse',[dict(kind='path',id='d000009',text=deep)]);verify(root,reference(root))
                # Deletion is allowed once incoming references are explicitly removed.
                state=reference(root);target='d000002';ops=[]
                for id,d in state['docs'].items():
                    for ordinal,e in enumerate(d['edges']):
                        if not e['external'] and e['target']==target:ops.append(dict(kind='drop-edge',id=id,value=ordinal))
                pub(root,'remove-target',ops+[dict(kind='delete',id=target)]);verify(root,reference(root))
                call(root,'audit')
                # Fences, pin, packet owner and a genuinely eligible candidate survive.
                call(root,'cancel',request_id='terminal-fence');pin=selected(root);call(root,'lifecycle',operation='pin',name='durable-pin',selected=pin)
                packet=call(root,'packet')['result'];packet_bytes={p.relative_to(root).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in (root/packet['path']).rglob('*') if p.is_file()}
                candidate=call(root,'prepare',request=dict(request='eligible',base=selected(root),ops=[dict(kind='label',id='d000003',text='eligible')],packet=False))['result']['candidate']
                if mode=='A':
                    call(root,'resume',code=86,exit_at='refresh-plan')
                archive=scratch/f'backup-{mode}-{n}';capture(root,archive,'working-source-in-session-manifest',True)
                clone=scratch/f'restored-{mode}-{n}'/'disposable/store';restore(archive,clone)
                assert not (clone/'DISPOSABLE.json').exists()
                assert selected(clone)==selected(root)
                assert call(clone,'resolve',request_id='terminal-fence')['result']['outcome']=='cancelled-fenced'
                assert json.loads((clone/'PINS.json').read_bytes())['durable-pin']==pin
                for path,h in packet_bytes.items():assert hashlib.sha256((clone/path).read_bytes()).hexdigest()==h
                call(clone,'lifecycle',operation='setup');call(clone,'select',candidate=candidate)
                assert call(clone,'resolve',request_id='eligible')['result']['outcome']=='accepted'
                if mode=='A':call(clone,'resume')
                pub(clone,'valid-successor',[]);call(clone,'audit');verify(clone,reference(clone))
                # Incomplete required archive or restored evidence cannot claim success.
                damaged=clone/'SELECTED.json';saved=damaged.read_bytes();damaged.unlink();call(clone,'inspect',code=2);damaged.write_bytes(saved)
                call(clone,'lifecycle',operation='retire',selected=pin,code=2)
                cases.append(dict(case='navigation-and-operational-backup',mode=mode,n=n,status='success'))
        for direction in ['up','down']:
            for phase in ['refresh-plan','refresh-preserved','refresh-file','refresh-namespace','refresh-before-complete','refresh-complete','refresh-namespace-preserved']:
                root=init(direction+'-'+phase)
                if direction=='down':pub(root,'up-first',[dict(kind='path',id='d000009',text=ancestor)])
                else:
                    path=root/'view'/deep;path.write_bytes(b'external divergent source')
                    extra=root/'view'/ancestor/'external.txt';extra.write_bytes(b'external subtree')
                target=ancestor if direction=='up' else deep
                if phase=='refresh-namespace-preserved' and direction=='down':continue
                # refresh-preserved requires a divergent file in both directions.
                if direction=='down' and phase=='refresh-preserved':(root/'view'/ancestor).write_bytes(b'changed ancestor')
                pub(root,'interrupted',[dict(kind='path',id='d000009',text=target)],defer=True)
                call(root,'refresh',code=86,exit_at=phase)
                pub(root,'correcting',[dict(kind='path',id='d000009',text='docs/item-corrected-d000009.txt')],defer=True)
                call(root,'refresh');call(root,'resume');call(root,'audit');verify(root,reference(root))
                if direction=='up':
                    preserved=[p.read_bytes() for p in (root/'divergence').rglob('*') if p.is_file()]
                    assert b'external divergent source' in preserved and b'external subtree' in preserved
                cases.append(dict(case='namespace-interruption-correcting-successor',direction=direction,phase=phase,status='success'))
        for mode in ['A','B']:
            root=init('accounting-'+mode,mode)
            pub(root,'transfer',[dict(kind='label',id='d000000',text='transfer')],code=86,exit_at='accounting-transfer');interrupted=selected(root)
            accepted=call(root,'resolve',request_id='transfer')['result'];call(root,'checkpoint',request_id='checkpoint')
            call(root,'lifecycle',operation='retire',selected=interrupted)
            call(root,'lifecycle',operation='collect');call(root,'lifecycle',operation='collect')
            assert call(root,'resolve',request_id='transfer')['result']==accepted
            call(root,'audit');cases.append(dict(case='checkpoint-transfer-retire-repeat-gc',mode=mode,status='success'))
            # A half-prepared root stays charged and refuses GC after restoration.
            root=init('half-'+mode,mode);pub(root,'half',[],code=86,exit_at='allocation')
            archive=scratch/('half-backup-'+mode);capture(root,archive,'working-source-in-session-manifest',True)
            clone=scratch/('half-restore-'+mode)/'disposable/store';restore(archive,clone);call(clone,'lifecycle',operation='setup')
            call(clone,'lifecycle',operation='collect',code=2)
            registry=json.loads((clone/'RESERVATIONS.json').read_bytes());assert len(registry)==1
            call(clone,'lifecycle',operation='abandon',attempt=next(iter(registry)));call(clone,'lifecycle',operation='collect');pub(clone,'after-half',[])
            cases.append(dict(case='half-preparation-backup-recovery',mode=mode,status='success'))
        root=init('impossible');before=selected(root)
        pub(root,'impossible',[dict(kind='path',id='d000008',text=ancestor)],code=2);assert selected(root)==before
        cases.append(dict(case='impossible-final-namespace',status='expected refusal'))
        # Skipped A generations and retained selected-once B ordinary paths.
        for mode in ['A','B']:
            root=init('selected-once-'+mode,mode);old=reference(root)
            oldpub=call(root,'inspect')['result']['publication'];oldtree=root/f"generations/{oldpub['attempt']}"
            oldnav=navigation(oldtree) if mode=='B' else None
            pub(root,'skip-1',[dict(kind='delete',id='d000000')],defer=True)
            pub(root,'skip-2',[dict(kind='path',id='d000009',text=ancestor)],defer=True)
            if mode=='A':call(root,'refresh')
            else:assert navigation(oldtree)==oldnav
            verify(root,reference(root));cases.append(dict(case='skipped-or-selected-once',mode=mode,status='success'))
    finally:
        receipt=dict(platform=platform.platform(),binary=str(exe),binary_sha256=hashlib.sha256(Path(exe).read_bytes()).hexdigest(),cases=cases,events=len(events),seconds=time.monotonic()-started)
        (scratch/'summary.json').write_text(json.dumps(receipt,indent=2),encoding='utf-8')
    print(json.dumps(receipt,indent=2));return receipt

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--scratch',required=True);ns=a.parse_args();run(Path(ns.exe),ns.scratch)
