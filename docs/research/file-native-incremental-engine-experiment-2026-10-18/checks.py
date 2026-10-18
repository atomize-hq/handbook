"""Named protocol regressions. Every native call is preserved by Driver."""
import argparse,json,os,hashlib,shutil
from pathlib import Path
from controller import Driver
def req(name,ops=(),base=None,packet=False):return {'request':name,'base':base,'ops':list(ops),'packet':packet}
def op(kind,id='d000003',**kw):return dict(kind=kind,id=id,**kw)
def result(v):return v['result']
def expect_error(v,part=None):
    assert v['status']=='error',v
    if part:assert part in v['error'],v
def exercise(d,mode,consumer):
    root=d.base/f'{mode}-{consumer}'/'disposable'/'store';r=str(root)
    def call(action,allow=False,env=None,**kw):
        return d.call(dict(action=action,root=r,**kw),allow=allow,env=env)
    first=result(call('init',mode=mode,consumer=consumer,count=16))
    initial=first['selected'];call('lifecycle',operation='setup')
    call('audit')
    a=req('label',[op('label',text='Alpha')])
    accepted=result(call('publish',request=a))['result']
    assert result(call('publish',request=a))['result']==accepted
    expect_error(call('publish',allow=True,request=req('label',[op('label',text='Different')])),'intent-mismatch')
    call('publish',request=req('lower',[op('label',id='d000004',text='alpha')]))
    expect_error(call('publish',allow=True,request=req('dup',[op('label',id='d000004',text='Alpha')])),'unique')
    # Final overlay swaps labels and path occupants; intermediate duplicates are allowed.
    call('publish',request=req('swap',[op('label',text='alpha'),op('label',id='d000004',text='Alpha')]))
    before=result(call('dump'))['state']['docs']
    call('publish',request=req('path-swap',[op('path',text=before['d000004']['path']),op('path',id='d000004',text=before['d000003']['path'])]))
    call('audit')
    call('publish',request=req('edges',[op('edge',id='d000005',text='requires',target='d000006'),op('edge',id='d000005',text='requires',target='d000006')]))
    call('publish',request=req('drop-one',[op('drop-edge',id='d000005',value=0)]))
    expect_error(call('publish',allow=True,request=req('delete-target',[op('delete',id='d000006')])),'local-reference')
    expect_error(call('publish',allow=True,request=req('target-kind',[op('kind',id='d000006',text='blocked')])),'target-kind')
    call('publish',request=req('delete-final',[op('delete',id='d000006'),op('drop-edge',id='d000005',value=1)]))
    call('publish',request=req('cost-final',[op('cost',id='d000003',value=1000),op('cost',id='d000003',value=0),op('cost',id='d000004',value=1000)]))
    expect_error(call('publish',allow=True,request=req('overflow',[op('cost',id='d000003',value=18446744073709551615)])),'overflow')
    # Failed preparation remains charged. Explicitly fence all unselected work before further admission.
    cleanup(d,r)
    prepared=result(call('prepare',request=req('stale',[op('cost',value=0)])))['candidate']
    call('publish',request=req('successor',[]))
    expect_error(call('select',allow=True,candidate=prepared),'expected-publication')
    cleanup(d,r)
    prepared=result(call('prepare',request=req('reopen',[op('cost',value=0)])))['candidate']
    call('select',candidate=prepared)
    # Tampered summaries and roots cannot become capabilities after reopen.
    prepared=result(call('prepare',request=req('tamper',[op('cost',value=0)])))['candidate']
    path=root/prepared;c=json.loads(path.read_bytes());c['changed']=['invented'];path.write_text(json.dumps(c));reseal(root,path)
    expect_error(call('select',allow=True,candidate=prepared),'replay')
    cleanup(d,r)
    call('checkpoint',request_id='checkpoint')
    assert result(call('resolve',request_id='label'))==accepted
    assert result(call('cancel',request_id='label'))==accepted
    assert result(call('cancel',request_id='never'))['terminal_noncommit'] is True
    expect_error(call('publish',allow=True,request=req('never',[])),'cancelled')
    call('audit')
    packet=result(call('packet'))
    verify_packet(root/packet['path'])
    cleanup(d,r)
    # Actual pin/retire/reclaim and original outcome after content retirement.
    call('lifecycle',operation='pin',name='reader',selected=initial)
    expect_error(call('lifecycle',allow=True,operation='retire',selected=initial),'protected')
    call('lifecycle',operation='release',name='reader',selected=initial)
    if mode=='A':call('refresh')
    call('lifecycle',operation='retire',selected=initial)
    call('lifecycle',operation='collect')
    expect_error(call('point',allow=True,selected=initial,id='d000001'),'retired')
    assert result(call('resolve',request_id='initial'))['outcome']=='accepted'
    call('audit')
    if mode=='A':
        assert result(call('view-audit'))['edited']==[]
    return {'mode':mode,'consumer':consumer,'initial':initial,'status':'passed'}
def reseal(root,path):
    # Deliberately update the independent seal to prove replay still refuses a
    # self-consistent forged candidate, rather than only testing its checksum.
    registry=root/'RESERVATIONS.json';v=json.loads(registry.read_bytes())
    candidate=json.loads(path.read_bytes());v[candidate['attempt']]['candidate_digest']=hashlib.sha256(path.read_bytes()).hexdigest()
    registry.write_text(json.dumps(v))

def cleanup(d,r):
    all=result(d.call({'action':'lifecycle','operation':'quota','root':r}))['reservations']
    for attempt,record in all.items():
        d.call({'action':'lifecycle','operation':'abandon','root':r,'attempt':attempt},allow=True)
    d.call({'action':'lifecycle','operation':'collect','root':r})
def verify_packet(path):
    manifest=json.loads((path/'MANIFEST.json').read_bytes())
    for p,h in manifest.items():assert hashlib.sha256((path/p).read_bytes()).hexdigest()==h
    prov=json.loads((path/'packet/provenance.json').read_bytes())
    assert prov['requested'] is True and len(prov['inputs'])==8
    assert hashlib.sha256((path/'packet/context.md').read_bytes()).hexdigest()==prov['output_sha256']
    for row in prov['inputs']:
        assert hashlib.sha256((path/'packet'/row['path']).read_bytes()).hexdigest()==row['sha256']
def faults(d):
    # Named authority boundaries are separately observed; no recovered outcome rewrites an exit event.
    rows=[]
    boundaries=['reservation-durable','allocation','semantic-root','projection-root','candidate-ready','seal','expected-checked',
      'selection-budget','membership-root','outcome-root','result-installed','publication-installed','attempt-hint','selector-staged','selector-visible',
      'selection-boundary','accounting-transfer','accounting-release','refresh-plan','refresh-file','refresh-before-complete','refresh-complete','reply']
    for point in boundaries:
        root=d.base/'faults'/point/'disposable'/'store';r=str(root)
        d.call(dict(action='init',root=r,mode='A',consumer='project',count=16))
        request=req('victim',[op('import',id='d000000',text='# changed exact bytes\r\n')],base=result(d.call(dict(action='inspect',root=r)))['selected'])
        out=d.call(dict(action='publish',root=r,request=request),env={'EXP_EXIT':point},allow=True)
        row=json.loads(d.events.read_bytes().splitlines()[-1]);assert row['exit']==86,(point,out)
        resolved=result(d.call(dict(action='resolve',root=r,request_id='victim')))
        after=point in boundaries[boundaries.index('selection-boundary'):]
        before=point in boundaries[:boundaries.index('selector-visible')]
        if after:assert resolved['outcome']=='accepted',(point,resolved)
        if before:assert resolved['outcome']=='absent-as-of',(point,resolved)
        if point=='publication-installed':
            selected=result(d.call(dict(action='inspect',root=r)))['selected']
            orphans=[p for p in (root/'publications').glob('*.json') if p.stem!=selected['publication']]
            assert len(orphans)==1
            sel={'publication':orphans[0].stem,'digest':hashlib.sha256(orphans[0].read_bytes()).hexdigest()}
            expect_error(d.call(dict(action='point',root=r,selected=sel,id='d000001'),allow=True),'not-selected')
        # Re-establish visible authority, publish a successor, then resolve original.
        d.call(dict(action='publish',root=r,request=req('successor',[])),env={'EXP_DEFER_REFRESH':'1'})
        again=result(d.call(dict(action='resolve',root=r,request_id='victim')))
        assert again==resolved
        d.call(dict(action='resume',root=r))
        d.call(dict(action='audit',root=r))
        rows.append({'boundary':point,'before_selection':before,'completed_boundary':after,'recovered_outcome':resolved['outcome'],'original_exit':86})
    return rows
if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);a.add_argument('--faults',action='store_true');ns=a.parse_args()
    d=Driver(ns.exe,ns.base,7200)
    rows=[exercise(d,mode,consumer) for mode in ['A','B'] for consumer in ['project','research']]
    if ns.faults:rows.extend(faults(d))
    (d.base/'checks.json').write_text(json.dumps(rows,indent=2))
