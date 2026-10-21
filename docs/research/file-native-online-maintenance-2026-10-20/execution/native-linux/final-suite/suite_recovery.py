"""R24 native constructors, preserving the protected capsule's exact obligations."""
from pathlib import Path
import hashlib
from suite_bootstrap import need
from native_resources_v3 import strict

ANCESTOR='docs/item-attachments-d000009/evidence'
DEEP=ANCESTOR+'/deep/source.txt'

def reference(c):return c.call('dump')['result']['state']
def navigation_case(c,mode,n):
    c.init(mode,n);initial=reference(c);c.verify('initial-navigation')
    c.publish('semantic-change',[dict(kind='delete',id='d000000'),dict(kind='rename',id='d000001',text='Renamed title'),
        dict(kind='edge',id='d000001',text='cites',target='d000002'),dict(kind='edge',id='d000001',text='cites',target='d000002'),
        dict(kind='path',id='d000009',text=ANCESTOR)])
    state=reference(c);need(set(state['docs'])==set(initial['docs'])-{'d000000'},'exact-document-deletion')
    need(state['docs']['d000001']['title']=='Renamed title' and state['docs']['d000009']['path']==ANCESTOR,'exact-title-path')
    expected=[dict(kind='cites',target='d000002',external=False)]*2
    need(state['docs']['d000001']['edges'][-2:]==expected,'ordered-duplicate-relationships')
    c.verify('ordered-edges')
    c.publish('inverse',[dict(kind='path',id='d000009',text=DEEP)]);c.verify('inverse-namespace')
    target='d000002';ops=[]
    for key,doc in reference(c)['docs'].items():
        for ordinal,edge in enumerate(doc['edges']):
            if not edge['external'] and edge['target']==target:ops.append(dict(kind='drop-edge',id=key,value=ordinal))
    c.publish('remove-target',ops+[dict(kind='delete',id=target)]);c.verify('removed-target')
    c.call('cancel',request_id='terminal-fence');pin=c.selected()
    c.call('lifecycle',operation='pin',name='durable-pin',selected=pin)
    packet=c.call('packet')['result'];packet_prefix=packet['path']
    packet_bytes={p.relative_to(c.root).as_posix():hashlib.sha256(p.read_bytes()).hexdigest()
                  for p in (c.root/packet_prefix).rglob('*') if p.is_file()}
    if mode=='A':c.publish('pending-backup-transition',[dict(kind='path',id='d000009',text=ANCESTOR)],env={'EXP_DEFER_REFRESH':'1'})
    request=dict(request='eligible',base=c.selected(),ops=[dict(kind='label',id='d000003',text='eligible')],packet=False)
    candidate=c.call('prepare',request=request)['result']['candidate']
    if mode=='A':
        c.call('refresh',code=86,env={'EXP_EXIT':'refresh-file'})
        need(not (c.root/'view'/ANCESTOR).exists(),'genuine-pending-refresh')
    restored=c.a.backup_restore(c,'operational-restored')
    need(restored.selected()==c.selected() and not (restored.root/'DISPOSABLE.json').is_symlink(),'selected-restored-exact')
    if mode=='A':
        need(restored.read('PENDING.json')==c.read('PENDING.json') and restored.read('VIEW.json')==c.read('VIEW.json'),'pending-view-restored')
    restored.verify('full-operational-restoration')
    need(restored.call('resolve',request_id='terminal-fence')['result']['outcome']=='cancelled-fenced','fence-survived')
    need(restored.document('PINS.json')['durable-pin']==pin,'persistent-pin-restored')
    for name,h in packet_bytes.items():need(hashlib.sha256(restored.read(name,8*1024**2)).hexdigest()==h,'packet-bytes-restored')
    restored.call('select',candidate=candidate)
    need(restored.call('resolve',request_id='eligible')['result']['outcome']=='accepted','eligible-candidate-restored')
    if mode=='A':restored.call('resume')
    restored.publish('valid-successor',[]);restored.verify('accepted-successor')
    restored.a.preserve_move(restored.root,'SELECTED.json')
    restored.call('inspect',code=2)
    saved=restored.read('SELECTED.json.withheld');restored.write('SELECTED.json',saved)
    restored.call('lifecycle',operation='retire',selected=pin,code=2)
    return restored.verify('required-evidence-refusal-and-restoration')

def namespace_case(c,direction,phase):
    c.init('A',16)
    if direction=='down':c.publish('up-first',[dict(kind='path',id='d000009',text=ANCESTOR)])
    else:
        c.write('view/'+DEEP,b'external divergent source')
        c.write('view/'+ANCESTOR+'/external.txt',b'external subtree')
    target=ANCESTOR if direction=='up' else DEEP
    if direction=='down' and phase=='refresh-preserved':c.write('view/'+ANCESTOR,b'changed ancestor')
    c.publish('interrupted',[dict(kind='path',id='d000009',text=target)],env={'EXP_DEFER_REFRESH':'1'})
    c.call('refresh',code=86,env={'EXP_EXIT':phase})
    proof=c.verify('critical-namespace-crash')
    need(proof['underlying']['navigation']['readable_declared_current'] is False,'incomplete-view-must-be-declared')
    if phase=='refresh-namespace-preserved':
        c.write('view/'+ANCESTOR+'/second-external.txt',b'external second incarnation')
    c.publish('correcting',[dict(kind='path',id='d000009',text='docs/item-corrected-d000009.txt')],env={'EXP_DEFER_REFRESH':'1'})
    c.call('refresh');c.call('resume')
    if direction=='up':
        preserved=[p.read_bytes() for p in (c.root/'divergence').rglob('*') if p.is_file()]
        need(b'external divergent source' in preserved and b'external subtree' in preserved,'external-namespace-divergence-lost')
        if phase=='refresh-namespace-preserved':need(b'external second incarnation' in preserved,'second-incarnation-lost')
    return c.verify('correcting-successor')

def accounting(c,mode):
    c.init(mode,16);accepted=c.lost('transfer','accounting-transfer',[dict(kind='label',id='d000000',text='transfer')])
    interrupted=c.selected();c.call('checkpoint',request_id='checkpoint')
    c.call('lifecycle',operation='retire',selected=interrupted)
    c.call('lifecycle',operation='collect');c.call('lifecycle',operation='collect')
    need(c.call('resolve',request_id='transfer')['result']==accepted,'permanent-outcome-after-repeat-collection')
    return c.verify('accounting-transfer')

def half_preparation(c,mode):
    c.init(mode,16);c.publish('half',[],code=86,env={'EXP_EXIT':'allocation'})
    restored=c.a.backup_restore(c,'half-restored')
    restored.call('lifecycle',operation='collect',code=2)
    registry=restored.document('RESERVATIONS.json')
    need('version' not in registry and len(registry)==1,'legacy-capsule-must-not-be-auto-converted-to-v2')
    restored.call('lifecycle',operation='abandon',attempt=next(iter(registry)))
    restored.call('lifecycle',operation='collect');restored.publish('after-half',[])
    return restored.verify('half-preparation-restored')

def selected_once(c,mode):
    from suite_ordinary import ordinary
    c.init(mode,16);saved=c.selected();old=ordinary(c,'selected-once-before',selected=saved) if mode=='B' else None
    c.publish('skip-1',[dict(kind='delete',id='d000000')],env={'EXP_DEFER_REFRESH':'1'})
    c.publish('skip-2',[dict(kind='path',id='d000009',text=ANCESTOR)],env={'EXP_DEFER_REFRESH':'1'})
    if mode=='A':
        need(c.verify('declared-mixed-view')['underlying']['navigation']['readable_declared_current'] is False,'A-skipped-view-declaration')
        c.call('refresh')
    else:need(ordinary(c,'selected-once-after',selected=saved)['files']==old['files'],'immutable-B-selected-once')
    return c.verify('skipped-or-selected-once-final')

def dispatch(c,name):
    if name.startswith('navigation-and-operational-backup/'):
        _,mode,n=name.split('/');return navigation_case(c,mode,int(n[1:]))
    if name.startswith('namespace-interruption-correcting-successor/'):
        _,direction,phase=name.split('/');return namespace_case(c,direction,phase)
    if name.startswith('checkpoint-transfer-retire-repeat-gc/'):return accounting(c,name[-1])
    if name.startswith('half-preparation-backup-recovery/'):return half_preparation(c,name[-1])
    if name=='impossible-final-namespace':
        c.init('A',16);before=c.selected();c.publish('impossible',[dict(kind='path',id='d000008',text=ANCESTOR)],code=2)
        need(c.selected()==before,'impossible-operation-changed-selected');return c.verify('impossible-refusal')
    if name.startswith('skipped-or-selected-once/'):return selected_once(c,name[-1])
    raise RuntimeError('closed-R24-ID-unimplemented:'+name)
