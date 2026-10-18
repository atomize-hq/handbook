"""Focused regressions for causal review findings and every additional transition."""
import argparse,json,hashlib,threading,time
from pathlib import Path
from controller import Driver
from checks import req,op,result,expect_error,cleanup,reseal
def run(d):
    root=d.base/'review'/'disposable'/'store';r=str(root)
    def call(action,allow=False,env=None,**kw):return d.call(dict(action=action,root=r,**kw),allow=allow,env=env)
    initial=result(call('init',mode='A',consumer='project',count=16))['selected'];call('lifecycle',operation='setup')
    docs=result(call('dump'))['state']['docs'];origin=docs['d000000'];old=(root/origin['original']).read_bytes()
    accepted=result(call('publish',request=req('owner',[op('import',id='d000000',text='intermediate'),op('import',id='d000000',text='final')],base=initial)))['result']
    pub=result(call('inspect'))['publication'];assert pub['allocation_owner']['includes_intermediates_and_stages'] is True
    # A real reader overlaps retirement/collection. Neither may reclaim its state.
    ready=d.base/'reader-ready';release=d.base/'reader-release';box=[]
    thread=threading.Thread(target=lambda:box.append(call('point',selected=initial,id='d000000',env={'EXP_BARRIER':'reader-opened','EXP_READY':str(ready),'EXP_RELEASE':str(release)})))
    thread.start();deadline=time.monotonic()+15
    while not ready.exists():
        assert time.monotonic()<deadline;time.sleep(.01)
    expect_error(call('lifecycle',allow=True,operation='retire',selected=initial),'active-preparation')
    expect_error(call('lifecycle',allow=True,operation='collect'),'active-preparation')
    release.write_text('release');thread.join(30);assert box and result(box[0])['value']['doc']['digest']==origin['digest']
    firstpub=json.loads((root/f"publications/{initial['publication']}.json").read_bytes())
    firstcandidate=f"attempts/{firstpub['attempt']}/candidate.json"
    call('lifecycle',operation='retire',selected=initial);call('lifecycle',operation='collect')
    assert not (root/origin['original']).exists(),'retired content was not actually reclaimed'
    assert not (root/firstcandidate).exists()
    assert result(call('select',candidate=firstcandidate))['outcome']=='accepted'
    assert result(call('resolve',request_id='initial'))['outcome']=='accepted'
    # Restore exact old authored bytes under a new publication identity.
    current=result(call('inspect'))['selected']
    restored=result(call('publish',request=req('restored',[op('import',id='d000000',text=old.decode())],base=current)))
    assert restored['selected']!=initial and (root/origin['original']).read_bytes()==old
    # Unfinished b write, successor returns to a. Net watermark diff alone would fail.
    current=result(call('inspect'))['selected']
    call('publish',allow=True,request=req('b',[op('import',id='d000000',text='b')],base=current),env={'EXP_EXIT':'refresh-file'})
    current=result(call('inspect'))['selected']
    call('publish',request=req('a-again',[op('import',id='d000000',text=old.decode())],base=current),env={'EXP_DEFER_REFRESH':'1'})
    call('lifecycle',operation='collect')
    call('refresh');assert result(call('view-audit'))=={'missing':[],'edited':[],'marker_is_integrity_proof':False}
    # Derived divergence is preserved once under a stable identity despite death.
    nav=root/'view/NAVIGATION.md';nav.write_bytes(b'external derived edit\r\n')
    call('resume',allow=True,env={'EXP_EXIT':'refresh-preserved'})
    pending=root/'PENDING.json';saved=pending.read_bytes();v=json.loads(saved)
    v['actions']=[];pending.write_text(json.dumps(v))
    expect_error(call('refresh',allow=True),'refresh-plan-actions');pending.write_bytes(saved)
    call('refresh')
    backups=[p for p in (root/'divergence').rglob('*') if p.is_file() and p.read_bytes()==b'external derived edit\r\n']
    assert len(backups)==1
    mark=root/'VIEW.json';saved=mark.read_bytes();v=json.loads(saved);v['sequence']+=1;mark.write_text(json.dumps(v))
    expect_error(call('refresh',allow=True),'watermark-binding');mark.write_bytes(saved)
    # Packet manifests and namespace are bound before reopened selection.
    prepared=result(call('prepare',request=req('packet-tamper',packet=True)))['candidate'];c=json.loads((root/prepared).read_bytes())
    manifest=root/f"attempts/{c['attempt']}/packet/MANIFEST.json";saved=manifest.read_bytes();v=json.loads(saved);del v['packet/context.md'];manifest.write_text(json.dumps(v))
    expect_error(call('select',allow=True,candidate=prepared),'packet-manifest');manifest.write_bytes(saved)
    call('select',candidate=prepared);call('audit')
    prepared=result(call('prepare',request=req('packet-redirect',packet=True)))['candidate']
    path=root/prepared;saved=path.read_bytes();candidate=json.loads(saved)
    fake=root/'attempts/fake/packet';fake.mkdir(parents=True);fake_manifest=b'{}';(fake/'MANIFEST.json').write_bytes(fake_manifest)
    candidate['packet_ref']['attempt']='fake';candidate['packet_ref']['manifest']=hashlib.sha256(fake_manifest).hexdigest();path.write_text(json.dumps(candidate));reseal(root,path)
    expect_error(call('select',allow=True,candidate=prepared),'packet-ref-replay');path.write_bytes(saved);reseal(root,path)
    cleanup(d,r)
    ready=d.base/'resolve-ready';release=d.base/'resolve-release';box=[]
    thread=threading.Thread(target=lambda:box.append(call('resolve',request_id='owner',env={'EXP_BARRIER':'resolve-history-captured','EXP_READY':str(ready),'EXP_RELEASE':str(release)})))
    thread.start();deadline=time.monotonic()+15
    while not ready.exists():assert time.monotonic()<deadline;time.sleep(.01)
    call('publish',request=req('during-resolve',[]))
    expect_error(call('lifecycle',allow=True,operation='collect'),'active-preparation')
    release.write_text('release');thread.join(30);assert box and result(box[0])==accepted
    # Candidate count really applies to stale sealed artifacts. Accepted retry is first.
    cleanup(d,r)
    for i in range(8):call('prepare',request=req(f'quota-{i}',[]))
    expect_error(call('prepare',allow=True,request=req('quota-full',[])),'admission-full')
    assert result(call('publish',request=req('owner',[op('import',id='d000000',text='intermediate'),op('import',id='d000000',text='final')],base=initial)))['result']==accepted
    cleanup(d,r)
    # A historical live snapshot may have lost obsolete history/change indexes.
    historical=result(call('inspect'))['selected']
    call('publish',request=req('after-historical',[op('label',text='newer')]))
    call('lifecycle',operation='collect')
    assembled=result(call('packet',selected=historical));assert assembled['selected']==historical
    expect_error(call('select',allow=True,candidate=assembled['candidate']),'candidate-version-intent')
    call('lifecycle',operation='collect');cleanup(d,r)
    # Candidate digest protects collection roots independently of selector replay.
    prepared=result(call('prepare',request=req('sealed-roots',[])))['candidate'];path=root/prepared;saved=path.read_bytes()
    candidate=json.loads(saved);candidate['graph']['tree']=None;path.write_text(json.dumps(candidate))
    expect_error(call('lifecycle',allow=True,operation='collect'),'sealed-candidate-digest')
    path.write_bytes(saved);cleanup(d,r)
    # Selected orphan allocations remain charged until all owned files are gone.
    before_results=set((root/'results').glob('*'));before_pubs=set((root/'publications').glob('*'))
    call('publish',allow=True,request=req('orphan-cleanup',[]),env={'EXP_EXIT':'selector-staged'})
    reservations=result(call('lifecycle',operation='quota'))['reservations'];attempt=next(a for a,v in reservations.items() if v['request']=='orphan-cleanup')
    assert set((root/'results').glob('*'))-before_results and set((root/'publications').glob('*'))-before_pubs
    cleanup(d,r)
    assert set((root/'results').glob('*'))==before_results and set((root/'publications').glob('*'))==before_pubs
    assert not (root/f'attempt-hints/{attempt}.json').exists()
    assert not any(p.is_file() for p in (root/f'attempts/{attempt}').rglob('*'))
    # Additional non-publication lifecycle/repair boundaries.
    for point,action,kw in [
        ('checkpoint-built','checkpoint',{'request_id':'cp-built'}),
        ('checkpoint-verified','checkpoint',{'request_id':'cp-verified'}),
        ('pin-registered','lifecycle',{'operation':'pin','name':'fault-reader'}),
        ('pin-release','lifecycle',{'operation':'release','name':'fault-reader'}),
        ('cancel-durable','cancel',{'request_id':'fault-cancel'})]:
        call(action,allow=True,env={'EXP_EXIT':point},**kw)
        event=json.loads(d.events.read_bytes().splitlines()[-1]);assert event['exit']==86,(point,event['output'])
    cleanup(d,r)
    call('checkpoint',allow=True,request_id='cp-reopen',env={'EXP_EXIT':'seal'})
    reservation=result(call('lifecycle',operation='quota'))['reservations']
    attempt=next(a for a,v in reservation.items() if v['request']=='cp-reopen')
    call('select',candidate=f'attempts/{attempt}/candidate.json')
    call('checkpoint',allow=True,request_id='cp-stale',env={'EXP_EXIT':'seal'})
    reservation=result(call('lifecycle',operation='quota'))['reservations']
    attempt=next(a for a,v in reservation.items() if v['request']=='cp-stale')
    call('publish',request=req('after-stale-cp',[]))
    expect_error(call('select',allow=True,candidate=f'attempts/{attempt}/candidate.json'),'expected-publication')
    cleanup(d,r)
    q=result(call('prepare',request=req('abandon-fault',[op('import',id='d000000',text='unchosen')],base=result(call('inspect'))['selected'])))['candidate']
    attempt=json.loads((root/q).read_bytes())['attempt']
    call('lifecycle',allow=True,operation='abandon',attempt=attempt,env={'EXP_EXIT':'abandonment-fenced'})
    expect_error(call('select',allow=True,candidate=q),'abandoned')
    call('lifecycle',allow=True,operation='collect',env={'EXP_EXIT':'reclaim-file'})
    call('lifecycle',allow=True,operation='collect',env={'EXP_EXIT':'reclaim-accounting'})
    call('lifecycle',operation='collect');call('audit')
    # Explicit opaque global fallback.
    f=d.base/'fallback';fr=str(f)
    d.call(dict(action='init',root=fr,count=16,mode='A',consumer='research',fallback=True))
    title=result(d.call(dict(action='dump',root=fr)))['state']['docs']['d000004']['title']
    expect_error(d.call(dict(action='publish',root=fr,request=req('global-duplicate',[op('rename',text=title)])),allow=True),'opaque-global')
    d.call(dict(action='audit',root=fr))
    return {'review_findings':'regressions passed','actual_retired_source_reclaimed':True,'restoration':restored['selected'],
        'reader_overlap':True,'stable_divergence_backups':len(backups),'quota_slots':8,'additional_boundaries':8,'fallback':True}
if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);ns=a.parse_args()
    d=Driver(ns.exe,ns.base,7200);(d.base/'supplement.json').write_text(json.dumps(run(d),indent=2))
