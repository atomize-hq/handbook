"""Source-derived P52, strict24 and B-control8 constructors; all real native I/O.

Baseline obligations: b19f6152 packing capsule,6708b2a1 strict,5385233d focused.
Version differences are explicit: generation pins/rolling coverage/FNPIDX02, not
old busy assertions or FNPIDX01 fixed offsets. Windows sharing is Linux-inapplicable.
"""
import copy, hashlib, json, struct
from pathlib import Path
from suite_bootstrap import need, encoded, digest
from native_resources_v3 import strict
import paged_index

MALFORMED=('missing-catalog','corrupt-catalog','missing-index','corrupt-index','missing-pack',
 'truncated-pack','corrupt-record','offset-overflow','raw-length-bound','duplicate-key','omitted-entry',
 'wrong-pack-binding','raw-hash-mismatch','bad-codec','too-many-packs','duplicate-json-key')

def pretty(value):return json.dumps(value,indent=2,ensure_ascii=False).encode()
def sha(raw):return hashlib.sha256(raw).hexdigest()
def integrity(error):return isinstance(error,str) and error.startswith(('integrity:','integrity-read:'))

def parts(c):
    config=c.document('STORE.json');cat=c.document('pack-catalogs/'+config['catalog']+'.json',65536)
    d=next(x for x in cat['packs'] if x['role']=='publication')
    prefix='packs/'+(d['instance']+'/' if d.get('instance') else '')
    return config,cat,d,prefix+d['index']+'.idx',prefix+d['pack']+'.pack'

def rebind(c,cat,index=None,catalog_raw=None):
    config=c.document('STORE.json')
    if index is not None:
        d,data=index;prefix='packs/'+(d['instance']+'/' if d.get('instance') else '')
        d['index']=sha(data);d['index_bytes']=len(data)
        d['page']['hash']=sha(data[:d['page']['bytes']])
        c.write(prefix+d['index']+'.idx',data)
    raw=pretty(cat) if catalog_raw is None else catalog_raw
    catalog=sha(raw);c.write('pack-catalogs/'+catalog+'.json',raw)
    work=c.document('physical-plans/'+config['physical']['plan']+'.json',131072)
    work['generation']['catalog']=catalog;work['generation']['packs']=copy.deepcopy(cat['packs'])
    work_raw=pretty(work);plan=sha(work_raw);c.write('physical-plans/'+plan+'.json',work_raw)
    config['catalog']=catalog;config['physical']['plan']=plan;c.write('STORE.json',pretty(config))
    pin=c.document('physical-pins/'+config['physical']['id']+'.lock',1024)
    pin['catalog']=catalog;c.write('physical-pins/'+config['physical']['id']+'.lock',pretty(pin))

def page_positions(raw,d):
    ref=paged_index.root_descriptor(d)
    node=paged_index.page(raw,d,ref,last=d['last'])
    need(ref['height']==0,'small-malformed-seed-must-be-leaf-root')
    cur=paged_index.Cursor(raw[:ref['length']]);cur.take(8)
    fields={}
    for key,fmt in [('height','B'),('role','B'),('codec','B'),('reserved','B'),('records','I'),
                    ('slots','H'),('start','Q'),('end','Q')]:
        fields[key]=cur.pos;cur.number(fmt)
    fields['pack_hash']=cur.pos;cur.take(32);cur.number('Q');cur.key(d['role']);cur.key(d['role'])
    entries=[]
    for row in node['entries']:
        start=cur.pos;key=cur.key(d['role']);value=cur.pos;cur.take(48)
        entries.append(dict(key=key,start=start,value=value,end=cur.pos))
    need(cur.pos==ref['length'],'exact-paged-constructor-boundary')
    return fields,entries

def seed(c,mode='A',n=16):
    if c.id in c.a.m.get('donor_cases',{}) and c.root.exists():
        need(c.root in c.h.ledgers and c.h.ledgers[c.root].accepted and
             c.document('STORE.json')['mode']==mode and c.root not in c.a.poisoned_roots,'same-mode-authenticated-shallow-donor')
        c.verify('existing-shallow-donor-no-clone');return c
    c.init(mode,n);c.publish('second');c.publish('third')
    c.pack('raw',bootstrap=True,cleanup=False)
    return c

def malformed(c,kind):
    seed(c);config,cat,d,ip,pp=parts(c);cp='pack-catalogs/'+config['catalog']+'.json'
    if kind=='missing-catalog':c.a.preserve_move(c.root,cp)
    elif kind=='corrupt-catalog':c.write(cp,c.read(cp,65536)+b' ')
    elif kind=='missing-index':c.a.preserve_move(c.root,ip)
    elif kind=='corrupt-index':
        data=bytearray(c.read(ip,16*1024**2));data[0]^=1;c.write(ip,bytes(data))
    elif kind=='missing-pack':c.a.preserve_move(c.root,pp)
    elif kind=='truncated-pack':
        data=c.read(pp,64*1024**2);c.write(pp,data[:-1])
    elif kind=='corrupt-record':
        decoder=c.a.oracle.decoder(c.root);key='publications/'+c.selected()['publication']+'.json'
        entry=decoder.entries[key];data=bytearray(c.read(pp,64*1024**2))
        # Reviewed byte parser supplies the logical entry; no guessed index offset.
        rows=paged_index.full(c.read(ip,16*1024**2),d)['entries']
        offset=next(row[1] for row in rows if row[0]==key);data[offset]^=1;c.write(pp,bytes(data))
    elif kind=='too-many-packs':cat['packs']=[copy.deepcopy(cat['packs'][0]) for _ in range(65)];rebind(c,cat)
    elif kind=='duplicate-json-key':
        raw=c.read(cp,65536);pos=raw.index(b'"version"');colon=raw.index(b':',pos);end=colon+1
        while raw[end:end+1] in (b' ',b'\n'):end+=1
        tail=end
        while raw[tail:tail+1].isdigit():tail+=1
        raw=raw[:tail]+b', "version": '+raw[end:tail]+raw[tail:]
        rebind(c,cat,catalog_raw=raw)
    else:
        raw=c.read(ip,16*1024**2);fields,entries=page_positions(raw,d);data=bytearray(raw)
        need(entries,'malformed-seed-index-empty')
        if kind=='offset-overflow':struct.pack_into('<Q',data,entries[0]['value'],2**64-1)
        elif kind=='raw-length-bound':struct.pack_into('<I',data,entries[0]['value']+12,8*1024**2+1)
        elif kind=='wrong-pack-binding':data[fields['pack_hash']]^=1
        elif kind=='raw-hash-mismatch':
            for e in entries:data[e['value']+16]^=1
        elif kind=='bad-codec':data[fields['codec']]=2;d['codec']=2
        elif kind=='duplicate-key':
            need(len(entries)>1,'two-entry-duplicate-constructor')
            a,b=entries[:2];data[b['start']:b['value']]=data[a['start']:a['value']]
            d['page']['bytes']=len(data)
        elif kind=='omitted-entry':
            need(len(entries)>1,'two-entry-omission-constructor')
            e=entries[0];del data[e['start']:e['end']]
            d['records']-=1;struct.pack_into('<I',data,fields['records'],d['records'])
            struct.pack_into('<H',data,fields['slots'],len(entries)-1);d['page']['bytes']=len(data)
        else:raise RuntimeError('unknown-closed-malformation:'+kind)
        rebind(c,cat,(d,bytes(data)))
    refusal=c.call('resolve',request_id='initial',code=2)
    need(refusal['error'] and refusal.get('result') is None,'mapped-corruption-must-refuse-even-with-loose-duplicate')
    return c.a.oracle.refuse(c.root,kind)

def operational(c,mode):
    c.init(mode,128);initial=c.selected()
    c.call('lifecycle',operation='pin',name='original',selected=initial)
    if mode=='A':
        p=c.document('publications/'+initial['publication']+'.json')
        raw=pretty(dict(publication='oversize-journal',result=p['result_hash']))+b' '*(8*1024**2+1)
        name='attempts/'+p['attempt']+'/allocations/oversize-journal.json';c.write(name,raw)
    c.publish('packet-owner',packet=True);packet_head=c.selected()
    c.call('checkpoint',request_id='packet-checkpoint')
    request=dict(request='eligible',base=c.selected(),ops=[],packet=False)
    candidate=c.call('prepare',request=request)['result']['candidate'];candidate_raw=c.read(candidate,4*1024**2)
    if mode=='A':
        c.publish('pending-A',[dict(kind='rename',id='d000004',text='pending packed title')],env={'EXP_DEFER_REFRESH':'1'})
        c.call('refresh',code=86,env={'EXP_EXIT':'refresh-plan'})
        pending=c.read('PENDING.json')
    c.pack('raw',bootstrap=True,cleanup=False);need(c.read(candidate,4*1024**2)==candidate_raw,'persistent-candidate-lost')
    if mode=='A':
        need(c.read('PENDING.json')==pending,'pending-refresh-owner-lost')
        restored=c.a.backup_restore(c,'pending-restore')
        need(restored.read('PENDING.json')==pending,'pending-not-restored')
        restored.call('refresh');restored.verify('pending-restored')
        c.call('select',candidate=candidate,code=2)
        c.call('lifecycle',operation='abandon',attempt=candidate.split('/')[1]);c.call('refresh')
    else:c.call('select',candidate=candidate)
    c.verify('P0')
    before=c.h.state(c.root);c.publish('loose-tail');tail=c.selected()
    c.pack('zstd')
    mapped=c.a.mapping(c.root,'publications/'+tail['publication']+'.json')
    need(mapped['mapped'],'rolling-accepted-tail-not-incorporated')
    c.verify('PZ')
    restored=c.a.backup_restore(c,'packed-restored');restored.call('inspect')
    restored.call('packing',operation='cleanup');restored.publish('restored-valid-successor');restored.verify('restored-successor')
    c.h.cleanup(c.root,max_batches=160)
    p=c.a.oracle.decoder(c.root);packet=json.loads(p.raw('publications/'+packet_head['publication']+'.json'))
    manifest='attempts/'+packet['packet_ref']['attempt']+'/packet/MANIFEST.json';packet_raw=c.read(manifest,4*1024**2)
    c.call('lifecycle',operation='retire',selected=packet_head)
    c.call('lifecycle',operation='release',name='original');c.call('lifecycle',operation='retire',selected=initial)
    c.call('lifecycle',operation='collect');c.pack('zstd',operation='repack')
    c.drain();need(c.read(manifest,4*1024**2)==packet_raw,'persistent-packet-lost')
    need(c.call('point',selected=initial,id='d000000',code=2)['error']=='retired:snapshot','retired-content-revived')
    return c.verify('operational-final')

def interruption(c,mode,phase,nth):
    c.init(mode,16);c.publish('second');c.publish('third')
    expected=c.call('resolve',request_id='initial')['result']
    c.pack('raw',bootstrap=True,cleanup=True,code=86,env={'EXP_EXIT':phase,'EXP_NTH':str(nth)})
    need(c.call('resolve',request_id='initial')['result']==expected,'crash-changed-outcome')
    c.verify('critical-crash')
    j=c.document('PACKING.json');need(j['reserved_bytes']>0,'durable-maintenance-reservation')
    if j['status']=='building':
        c.call('packing',operation='abort');c.pack('raw',bootstrap=True)
    else:c.call('packing',operation='resume',cleanup=False)
    c.h.cleanup(c.root,max_batches=160)
    return c.verify('recovered')

def backup_interruption(c,phase):
    c.init('A',16);c.publish('second')
    c.pack('raw',bootstrap=True,code=86,env={'EXP_EXIT':phase})
    restored=c.a.backup_restore(c,'restored')
    if restored.document('PACKING.json')['status']=='building':
        restored.call('packing',operation='abort');restored.pack('raw',bootstrap=True)
    else:restored.call('packing',operation='resume',cleanup=False)
    restored.h.cleanup(restored.root,max_batches=160);restored.publish('restored-successor')
    return restored.verify('orphan-restored-successor')

def protected(c,mode,phase):
    seed(c,mode);c.h.cleanup(c.root,max_batches=160)
    expected=c.call('resolve',request_id='initial')['result']
    # The final protocol progresses under an old generation pin; the old busy
    # assertion is deliberately replaced, with the same named read checkpoint.
    with c.h.barrier(c.root,'legacy-read-'+phase,'resolve',phase,request_id='initial') as (handle,control):
        for n in range(2):
            c.publish('reader-successor-'+str(n));c.pack('zstd')
        need(c.a.alive(handle),'old-reader-ended-before-successors')
    need(handle['response']['result']==expected,'resumed-old-reader-result')
    c.drain();return c.verify('protected-read-final')

def cancelled_drain(c,mode):
    seed(c,mode);control=c.a.control(c.root,'cancel-capture')
    with c.h.barrier(c.root,'captured-reader','resolve','pack-catalog-captured',request_id='initial'):
        c.a.signal(control,'cancel')
        try:
            response=c.pack('zstd',code=2,env={'EXP_PACK_CANCEL':control['cancel']})
            need(response['error'].startswith('cancelled:'),'packing-cancellation-not-observed')
        finally:c.a.end_signal(control,'cancel')
    c.pack('zstd');c.drain();return c.verify('cancel-gate-and-pin-release')

def contenders(c,mode):
    seed(c,mode);c.h.cleanup(c.root,max_batches=160)
    prefix=c.id+'-' if c.id in c.a.m.get('donor_cases',{}) else ''
    request=dict(request=prefix+'stale-candidate',base=c.selected(),ops=[],packet=False)
    prepared=c.call('prepare',request=request)['result']['candidate']
    c.publish(prefix+'eligible-tail')
    with c.h.barrier(c.root,'maintainer','packing','pack-before-catalog',operation='build',codec='raw',
                     include_attempts=True,cleanup=False,force=True):
        c.call('packing',operation='status',code=2,env={'EXP_WAIT_MS':'100'})
        c.publish(prefix+'semantic-during-build')
    need(c.call('select',candidate=prepared,code=2)['error']=='conflict:expected-publication','stale-semantic-candidate')
    c.h.cleanup(c.root,max_batches=160);return c.verify('writer-and-maintainer-final')

def old_deletion(c,mode):
    c.init(mode,16);initial=c.selected();c.pack('raw',bootstrap=True);c.h.cleanup(c.root,max_batches=160)
    c.publish('later');c.call('lifecycle',operation='retire',selected=initial);c.call('lifecycle',operation='collect')
    before=c.a.inventory(c.root)
    c.pack('zstd',operation='repack',segment_limit=1,cleanup=True,code=86,env={'EXP_EXIT':'pack-old-delete'})
    proof=c.a.removal_proof(c.root,before,'pack-old-delete')
    need(proof['actually_removed'] and any(p.endswith(('.pack','.idx')) for p in proof['removed_paths']),'real-old-container-removal')
    c.verify('actual-old-delete-crash');c.call('packing',operation='resume',cleanup=False)
    c.h.cleanup(c.root,max_batches=160);return c.verify('old-delete-recovered')

def strict_case(c,name):
    if name.startswith('independent-') and name[len('independent-'):] in MALFORMED:
        return malformed(c,name[len('independent-'):])
    seed(c);config,cat,d,ip,pp=parts(c)
    if name.startswith('strict-numeric-type-'):
        field=name[len('strict-numeric-type-'):]
        if field=='version':cat['version']=True
        else:cat['packs'][0][field]=True
        rebind(c,cat)
        return c.a.oracle.refuse(c.root,name,require_before_index=True)
    if name in {'independent-oversize-catalog','independent-oversize-index'}:
        path='pack-catalogs/'+config['catalog']+'.json' if name.endswith('catalog') else ip
        size=65537 if name.endswith('catalog') else 16*1024**2+1
        c.write(path,b' '*size)
        return c.a.oracle.refuse(c.root,name,require_before_index=True)
    need(name=='physical-catalog-and-hint-cannot-grant-membership','strict-known-case')
    c.verify('healthy-before-physical-orphan')
    from suite_extensions import leaf
    selected=c.read('SELECTED.json');selector=c.selected()
    decoder=c.a.oracle.decoder(c.root)
    p=json.loads(decoder.raw('publications/'+selector['publication']+'.json'))
    p.update(id='zz-orphan',attempt='zz-orphan',request='orphan-request',intent='0'*64,
             predecessor=selector,sequence=p['sequence']+1)
    p['allocation_owner']['attempt']='zz-orphan'
    result=pretty(dict(outcome='accepted',request=p['request'],publication=p['id'],
                      state_digest=sha(pretty(p['graph'])),changed=[],removed=[]))
    p['result_hash']=sha(result);c.write('results/'+p['result_hash']+'.json',result)
    body=pretty(p);orphan=dict(publication=p['id'],digest=sha(body));hint=pretty(orphan)
    # Add actual authenticated publication AND hint physical records. The
    # existing immutable birth owns these disposable adversarial fixture bytes.
    for role,key,data in [('publication','publications/zz-orphan.json',body),
                           ('hint','attempt-hints/zz-orphan.json',hint)]:
        raw,desc=leaf(role,[(key,0,len(data),len(data),sha(data))],sha(data),len(data))
        desc['instance']=d['instance'];desc['segment']=d.get('segment',0)
        prefix='packs/'+desc['instance']+'/'
        c.write(prefix+desc['pack']+'.pack',data);c.write(prefix+desc['index']+'.idx',raw)
        cat['packs'].append(desc)
    cat['packs'].sort(key=lambda x:(x.get('segment',0),x['role'],x['first']))
    rebind(c,cat)
    import physical_decoder_v5 as v5
    physical=v5.Decoder(c.root,quiescent=True)
    need(physical.raw('publications/zz-orphan.json')==body and physical.raw('attempt-hints/zz-orphan.json')==hint,
         'actual-authenticated-physical-orphan-and-hint')
    response=c.call('resolve',request_id='orphan-request')['result']
    need(response==dict(outcome='absent-as-of',terminal_noncommit=False),'physical-hint-granted-acceptance')
    need(c.call('point',selected=orphan,id='d000000',code=2)['error']=='refused:not-selected-history','physical-catalog-authority')
    need(c.call('select',candidate='attempts/zz-orphan/candidate.json',code=2).get('result') is None,'physical-orphan-cannot-select')
    need(c.read('SELECTED.json')==selected,'physical-changed-selected')
    # Final cohort4 deliberately rejects physically mapped unaccepted owners.
    # Preserve the old native membership assertions and verify this stronger
    # independent final-version refusal instead of calling the fixture healthy.
    refused=c.a.oracle.refuse(c.root,'physical-orphan-not-accepted-cohort')
    need(any(reason in refused['reason'] for reason in ['mapped-publication-not-accepted','mapped-attempt-not-accepted']),
         'full-independent-refusal-must-be-the-unaccepted-physical-cohort')
    return refused

def focused(c,name):
    from suite_ordinary import ordinary
    if name=='A-cleanup-retains-no-publication-control-duplicates':
        c.init('A',16);c.pack('raw',bootstrap=True);c.h.cleanup(c.root,max_batches=160)
        need(not list((c.root/'publications').glob('*.json')),'A-retained-publication-duplicates')
        return c.verify('A-no-loose-controls')
    c.init('B',16);initial=c.selected();initial_path='publications/'+initial['publication']+'.json'
    initial_raw=c.read(initial_path);outcome=c.call('resolve',request_id='initial')['result']
    c.call('lifecycle',operation='pin',name='shell-frozen',selected=initial)
    original=ordinary(c,'original',selected=initial)
    c.publish('after-capture',[dict(kind='label',id='d000004',text='new label after shell capture')]);middle=c.selected()
    c.pack('raw',bootstrap=True);c.h.cleanup(c.root,max_batches=160)
    if name=='historical-selector-captured-before-publish-and-cleanup-exact-copy':
        need(ordinary(c,'frozen',selected=initial)['files']==original['files'],'selected-once-byte-copy')
    elif name=='retained-controls-have-no-deletion-credit':
        before=c.call('packing',operation='status')['result'];c.call('packing',operation='cleanup')
        need(c.read(initial_path)==initial_raw,'retained-control-deleted')
        after=c.call('packing',operation='status')['result']
        need(after.get('retained_loose_control_bytes')==before.get('retained_loose_control_bytes'),'retained-control-deletion-credit')
    elif name=='historical-generation-discovery-after-repack':
        c.pack('raw',operation='repack');c.h.cleanup(c.root,max_batches=160)
        need(ordinary(c,'repacked-frozen',selected=initial)['files']==original['files'],'historical-generation-after-repack')
    elif name=='concurrent-semantic-writer-preserves-frozen-and-new-current-discovery':
        c.publish('pack-tail')
        with c.h.barrier(c.root,'B-maintainer','packing','pack-before-catalog',operation='build',codec='raw',
                         cleanup=False,include_attempts=True,force=True):
            c.publish('writer-during-build')
            need(ordinary(c,'during',selected=initial)['files']==original['files'],'frozen-shell-during-maintenance')
        ordinary(c,'new-current')
    else:
        c.call('lifecycle',operation='release',name='shell-frozen')
        c.call('lifecycle',operation='retire',selected=initial);c.call('lifecycle',operation='collect')
        need(c.call('point',selected=initial,id='d000004',code=2)['error']=='retired:snapshot','retired-content-refusal')
        need(c.call('resolve',request_id='initial')['result']==outcome,'permanent-retired-outcome')
        if name=='retirement-removes-content-but-keeps-permanent-B-publication-control':
            ordinary(c,'retired',selected=initial,expected=23);c.pack('raw',operation='repack')
            need(c.read(initial_path)==initial_raw,'B-permanent-control-retired')
        elif name in {'missing-control-fails-closed','conflicting-control-fails-closed'}:
            c.publish('real-control-cleanup-tail');tail=c.selected()
            c.pack('raw',cleanup=False)
            path='publications/'+tail['publication']+'.json'
            if name.startswith('missing'):c.a.preserve_move(c.root,path)
            else:c.write(path,c.read(path)+b' ')
            response=c.call('packing',operation='cleanup',code=2)
            need(integrity(response['error']),'B-control-integrity-refusal')
            return c.a.oracle.refuse(c.root,name)
        else:raise RuntimeError('unknown-focused-ID:'+name)
    return c.verify('focused-final')

def packing_dispatch(c,name):
    if name in MALFORMED:return malformed(c,name)
    if name.startswith('operational-'):return operational(c,name[-1])
    if name=='orphan-unregistered-backup-restore':return backup_interruption(c,'pack-install')
    if name=='unselected-installed-backup-restore':return backup_interruption(c,'pack-before-catalog')
    if name.startswith('interruption-'):
        mode,rest=name[len('interruption-'):].split('-',1);phase,nth=rest.rsplit('-',1)
        return interruption(c,mode,phase,int(nth))
    if name.startswith('protected-read-'):
        mode,phase=name[len('protected-read-'):].split('-',1);return protected(c,mode,phase)
    if name.startswith('writer-and-competing-maintenance-'):return contenders(c,name[-1])
    if name.startswith('cancelled-drain-releases-barriers-'):return cancelled_drain(c,name[-1])
    if name.startswith('repack-old-pack-deletion-interruption-'):return old_deletion(c,name[-1])
    raise RuntimeError('closed-P52-ID-unimplemented:'+name)

def private_cache_clone(m):
    """Explicit V5 C1 entry; default false. Copies only one sealed dependency set."""
    import os,tomllib,time,stat
    from suite_adapter import suite_owner,suite_limits
    from suite_bootstrap import owned,encoded
    import native_guard as guard
    cfg=m['private_cache'];claim=suite_owner(m);dest=owned(cfg['root'],guard.ROOT)
    need(cfg['enabled'] is True and cfg['copy_go'] is True and cfg['reviewed'] is True,'explicit-C1-copy-GO')
    need(dest==guard.ROOT/'cargo-private' and 'cargo-private' in claim['categories'],'separate-Linux-C1-claim')
    need(claim['before']['cargo-private']['bytes']==0 and claim['before']['cargo-private']['files']==0,'fresh-C1-namespace')
    need(not any(dest.iterdir()),'C1-empty-no-prefix-overwrite')
    cap=min(suite_limits(m)['cargo-private'][0],claim['categories']['cargo-private']['bytes'])
    file_cap=min(suite_limits(m)['cargo-private'][1],claim['categories']['cargo-private']['files'])
    source=Path('/home/spenser/.cargo');registry='index.crates.io-1949cf8c6b5b557f'
    lock=owned(Path(m['workspace']['source_root'])/'Cargo.lock',guard.ROOT/'source')
    need(digest(lock)=='20e60d9dfc38e093a3926f6867b5519356685bdb6ca38c488857216f5bc9bb7d','exact-C1-lock')
    packages=[p for p in tomllib.loads(lock.read_text())['package'] if p.get('source','').startswith('registry+')]
    need(len(packages)==45,'exact45-locked-registry-packages')
    rows={}
    def member(path):
        need(path.is_file() and all(not p.is_symlink() for p in [path,*path.parents] if p.is_relative_to(source)),'C1-regular-source-without-parent-alias')
        info=path.stat();need(info.st_nlink==1,'C1-no-source-alias')
        rows[path.relative_to(source).as_posix()]=[info.st_size,digest(path)]
    roots=0
    for p in packages:
        n=p['name'];stem=n+'-'+p['version'];arc=source/'registry/cache'/registry/(stem+'.crate')
        need(digest(arc)==p['checksum'],'locked-archive-checksum');member(arc)
        src=source/'registry/src'/registry/stem
        if src.is_dir() and n not in {'winapi','winapi-i686-pc-windows-gnu','winapi-x86_64-pc-windows-gnu'}:
            roots+=1
            for folder,dirs,names in os.walk(src,followlinks=False):
                need(not any((Path(folder)/x).is_symlink() for x in dirs),'C1-no-source-directory-alias')
                for name in names:member(Path(folder)/name)
        rel='1/'+n if len(n)==1 else '2/'+n if len(n)==2 else '3/'+n[0]+'/'+n if len(n)==3 else n[:2]+'/'+n[2:4]+'/'+n
        member(source/'registry/index'/registry/'.cache'/rel)
    member(source/'registry/index'/registry/'config.json')
    manifest=[[k,*v] for k,v in sorted(rows.items())];total=sum(v[1] for v in manifest)
    need(roots==32 and total==39205355 and len(manifest)==1653 and
         hashlib.sha256(json.dumps(manifest,separators=(',',':')).encode()).hexdigest()==cfg['manifest_sha256']=='7f0443c355439d2a45ddac9b7ce646f5c4067ffaf94493d61e039e9dcfa90f78','exact-Linux-C1-closure')
    need(total<=cap and len(manifest)<=file_cap,'C1-entire-prospective-copy')
    remaining=total;remaining_files=len(manifest)
    for rel,size,expected in manifest:
        need(time.time()<claim['absolute_deadline'],'C1-copy-deadline')
        current=guard.inventory(dest)
        need(current['bytes']+remaining<=cap and current['files']+remaining_files<=file_cap,'C1-prewrite-all-retained-and-remaining')
        need(__import__('shutil').disk_usage(dest).free>=20*1024**3,'C1-free-floor')
        src=source/rel;need(all(not p.is_symlink() for p in [src,*src.parents] if p.is_relative_to(source)),'C1-source-parent-unchanged');target=owned(dest/rel,dest);before=src.stat();target.parent.mkdir(parents=True,exist_ok=True)
        h=hashlib.sha256();count=0
        fd=os.open(src,os.O_RDONLY|os.O_NOFOLLOW)
        with os.fdopen(fd,'rb') as inp:
            opened=os.fstat(inp.fileno());need((opened.st_dev,opened.st_ino,opened.st_size,opened.st_mtime_ns)==(before.st_dev,before.st_ino,size,before.st_mtime_ns),'C1-source-open-identity')
            with target.open('xb') as out:
                while True:
                    need(time.time()<claim['absolute_deadline'],'C1-chunk-deadline')
                    block=inp.read(min(65536,size-count+1))
                    if not block:break
                    need(count+len(block)<=size,'C1-chunk-before-write-bound')
                    out.write(block);h.update(block);count+=len(block)
                out.flush();os.fsync(out.fileno())
            after=os.fstat(inp.fileno());need((after.st_dev,after.st_ino,after.st_size,after.st_mtime_ns)==(before.st_dev,before.st_ino,size,before.st_mtime_ns),'C1-source-final-identity')
        need(count==size and h.hexdigest()==expected and target.stat().st_size==size and digest(target)==expected,'C1-exact-copied-member')
        remaining-=size;remaining_files-=1
    result=dict(kind='V5_PRIVATE_C1_COPY_ONLY',engine_qualification=False,final_pin=m['final_pin'],claim=m['claim'],files=1653,bytes=total,manifest_sha256=cfg['manifest_sha256'],partials_retained=True,global_cache_modified=False)
    raw=encoded(result)+b'\n';output=owned(m['phase']['packet_path'],guard.ROOT/'evidence')
    need(len(raw)<=m['phase']['packet_max_bytes'],'C1-report-reservation')
    current=guard.inventory(guard.ROOT/'evidence');e=claim['categories']['evidence'];before=claim['before']['evidence']
    need(current['bytes']+len(raw)<=min(24*1024**2,before['bytes']+e['bytes']) and current['files']+1<=before['files']+e['files'],'C1-report-allocation')
    with output.open('xb') as out:out.write(raw);out.flush();os.fsync(out.fileno())
    return result

def native_stage_command(m):
    import re
    import native_guard as guard
    from suite_bootstrap import owned,need,digest
    w=m['workspace'];stage=m['phase']['native_stage'];need(stage in {'build','list','test'},'closed-native-Cargo-stage')
    need(w['reviewed'] is True and w['source_pin']==m['final_pin'],'same-final-workspace-source-release')
    root=owned(w['source_root'],guard.ROOT/'source');need(root==guard.ROOT/'source/final-engine','frozen25-source-prefix')
    for name,row in w['source_files'].items():
        size,sha=(row if type(row) is list else [row['bytes'],row['sha256']]);p=owned(root/name,root)
        need(p.stat().st_size==size and digest(p)==sha,'same-frozen-Rust-Cargo:'+name)
    need(len(w['source_files'])==25,'exact25-native-source-files')
    target=owned(w['target_dir'],guard.ROOT/'target')
    need(target==guard.ROOT/'target'/('native-v5-'+m['final_pin']),'fresh-bounded-private-target-prefix')
    names=[r[0] for r in w['unit_catalog']];need(len(names)==len(set(names))==31,'exact-source31-native-unit-catalog')
    cache=m['private_cache'];need(cache['enabled'] is True and cache['reviewed'] is True and cache['copy_go'] is False,'separately-admitted-private-Cargo-home')
    receipt=owned(cache['qualified_copy']['path'],guard.ROOT/'evidence')
    need(digest(receipt)==cache['qualified_copy']['sha256'],'source-bound-C1-phase-receipt')
    proof=__import__('suite_bootstrap',fromlist=['strict']).strict(receipt.read_bytes(),4*1024**2)
    need(proof['kind']=='V5_PHASE_FINAL' and proof['status']=='PASSED' and proof['scope']=='private-cache-v5' and proof['final_pin']==m['final_pin'] and proof['result']['manifest_sha256']==cache['manifest_sha256'],'actual-qualified-C1-copy')
    for tool in [m['cargo'],m['rustc'],m['rustdoc']]:
        need('/.cargo/bin/' not in tool and digest(tool)==m['tools'][tool]['sha256'],'exact-sysroot-tool')
    fixture=owned(w['fixture_prefix'],guard.ROOT/'fixtures')
    need(fixture==guard.ROOT/'fixtures'/('native-final31-'+m['final_pin']),'one-final31-fixture-lineage')
    if stage=='test':need(not fixture.exists(),'heavy-unit-lineage-only-once-no-rerun')
    common=['--manifest-path',str(root/'Cargo.toml'),'--locked','--offline','--workspace','--release','-j2','--target-dir',str(target)]
    cargo=[m['cargo'],'build' if stage=='build' else 'test',*common]
    cargo+=['--message-format','json-render-diagnostics'] if stage=='build' else ['--','--list'] if stage=='list' else ['--','--test-threads=1']
    return ['/usr/bin/env','CARGO_HOME='+cache['root'],'CARGO_NET_OFFLINE=true','CARGO_INCREMENTAL=0','RUSTC='+m['rustc'],'RUSTDOC='+m['rustdoc'],'TMPDIR='+str(guard.ROOT/'fixtures'/('native-final31-tmp-'+m['final_pin']) if stage=='test' else target/'owned-tmp'),'EXP_PACK_TEST_ROOT='+str(fixture),*cargo]


def native_stage_finalize(m,g):
    import re
    from suite_bootstrap import owned,strict,need,digest,encoded
    import native_guard as guard
    stage=m['phase']['native_stage'];out=owned(m['phase']['guard_receipt_path'],guard.ROOT/'evidence')
    stdout=out.with_suffix('.stdout');stderr=out.with_suffix('.stderr')
    need(digest(stdout)==g['stdout_sha256'] and digest(stderr)==g['stderr_sha256'],'complete-Cargo-stdout-stderr')
    proof=g['root_exec'];need(proof and proof['sha256']==m['tools'][m['cargo']]['sha256'] and proof['rss']>0 and proof['inode']>0,'actual-Cargo-executable-identity-RSS')
    need(g['command']==native_stage_command_for_result(m),'same-exact-native-stage-command')
    raw=stdout.read_bytes();text=raw.decode('utf-8');names=[r[0] for r in m['workspace']['unit_catalog']]
    result=dict(stage=stage,final_pin=m['final_pin'],stdout_path=str(stdout),stdout_sha256=digest(stdout),stderr_path=str(stderr),stderr_sha256=digest(stderr),source_files=m['workspace']['source_files'],tests=[])
    if stage=='build':
        messages=[strict(line,4*1024**2) for line in raw.splitlines() if line];need(messages and messages[-1].get('reason')=='build-finished' and messages[-1]['success'],'full-Cargo-build-finished')
        artifacts=[r for r in messages if r.get('reason')=='compiler-artifact' and r.get('target',{}).get('name')=='file-native-incremental' and r.get('executable')]
        need(len(artifacts)==1,'one-actual-final-engine-artifact');p=owned(artifacts[0]['executable'],guard.ROOT/'target')
        result['binary']=dict(path=str(p),bytes=p.stat().st_size,sha256=digest(p),source_pin=m['final_pin'])
    else:
        listed=re.findall(r'^(.+): test$',text,re.M) if stage=='list' else re.findall(r'^test (.+) \.\.\. ok$',text,re.M)
        need(len(listed)==31 and set(listed)==set(names) and 'FAILED' not in text and 'FAILED' not in stderr.read_text(),'all31-exact-native-unit-identities')
        result['tests']=listed
        if stage=='test':
            fixture=owned(m['workspace']['fixture_prefix'],guard.ROOT/'fixtures');rows={}
            for p in fixture.rglob('*'):
                need(not p.is_symlink(),'unit-fixture-no-alias')
                if p.is_file():rows[p.relative_to(fixture).as_posix()]=dict(bytes=p.stat().st_size,sha256=digest(p))
            need(rows,'complete-real-unit-fixture-census');result['fixture_prefix']=str(fixture);result['fixtures']=rows
    return result


def native_stage_command_for_result(m):
    import copy
    import native_guard as guard
    w=copy.deepcopy(m);stage=w['phase']['native_stage']
    if stage!='test':return native_stage_command(w)
    w['phase']['native_stage']='list';command=native_stage_command(w)
    command[-1]='--test-threads=1'
    target=Path(m['workspace']['target_dir']);fixture=Path(m['workspace']['fixture_prefix'])
    return [str(x).replace('TMPDIR='+str(target/'owned-tmp'),'TMPDIR='+str(guard.ROOT/'fixtures'/('native-final31-tmp-'+m['final_pin']))) for x in command]
