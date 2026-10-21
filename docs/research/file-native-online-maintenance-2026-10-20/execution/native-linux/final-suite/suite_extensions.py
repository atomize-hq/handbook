"""Concrete schedules for all new Linux case IDs; no execution on import.

Source release is separate from runtime admission. Boundary/compatibility cases
consume exact source-reviewed native test/producer identities, never guessed
engine flags, synthesized accepted history, or compact substitute payloads.
"""
import copy, hashlib, json, os, struct, time, uuid
from pathlib import Path
import paged_index
import focused_schedules as schedules
from suite_bootstrap import need, encoded, digest
from suite_packing import seed, parts, page_positions, rebind, pretty, sha, contenders
from suite_adapter import Case

def donor(c,mode='A',n=16):
    seed(c,mode,n);c.h.cleanup(c.root,max_batches=160)
    return c

def shared(c,spec):
    name=c.id
    functions={
        'D-partial-fixed-cutoff':schedules.partial_interval,
        'D-reservation-late-eligible':schedules.reservation_late,
        'D-cutoff-Q-stale-CAS':schedules.cutoff_and_stale_cas,
        'F1-terminal-retired-bootstrap-delta':schedules.terminal_retirement,
        'F2-auto-decline-later-admit':schedules.automatic_policy,
        'D-stable-base-segment-pressure':schedules.stable_base,
    }
    if name in functions:return functions[name](c.h,c.root,c.a.forecast(name))
    if name in {'S-A-zstd','S-B-zstd'}:
        return c.h.small(c.root,name[2],c.a.forecast(name),'zstd')
    if name=='X-native-pressure':
        return c.h.mapping_stress(c.root,c.a.forecast(name),count=spec['native_pressure_count'],native_focused=True)
    raise RuntimeError('undeclared-shared-extension')

def p0(c,spec):
    proofs=[]
    for mode in ('A','B'):
        d=c if mode=='A' else Case(c.a,c.id,c.a.alias(c.id,'matched-B'))
        d.init(mode,16);d.publish('P0-tail')
        semantic=d.read('SELECTED.json');d.pack('zstd',bootstrap=True)
        d.h.cleanup(d.root,max_batches=160)
        need(d.read('SELECTED.json')==semantic,'P0-semantic-authority')
        proofs.append(d.verify('P0-'+mode))
    return proofs

def owners(c,spec):
    donor(c)
    return schedules.persistent_owners(c.h,c.root,c.a.forecast(c.id))

def semantic_contenders(c,spec):
    return contenders(c,'A')

def reader_end(c,spec):
    donor(c);saved=c.selected();expected=c.call('resolve',request_id='initial')['result']
    c.call('lifecycle',operation='pin',name='durable-'+c.id,selected=saved)
    packet=c.call('packet',selected=saved)['result']
    manifest=packet['path']+'/MANIFEST.json';raw=c.read(manifest,4*1024**2)
    kind=c.id.removeprefix('S-reader-')
    phase=spec['reader_capture_phase']
    control=c.a.control(c.root,'reader-'+kind)
    environment=dict(EXP_BARRIER=phase,EXP_READY=control['ready'],EXP_RELEASE=control['release'])
    if kind=='error':environment['EXP_ERROR']=phase
    handle=c.a.spawn(lambda:c.h.call(c.root,'resolve',request_id='initial',env=environment,code=2 if kind=='error' else 0,timeout=60))
    ended=False
    try:
        c.a.await_ready(control,handle,10)
        native=copy.deepcopy(handle['native'])
        need(c.a.alive(handle) and native['ready_ns'] is not None,'live-native-reader-required')
        # Two successive actual mapping activations, with the same old read held.
        for n in range(2):
            c.publish(c.id+'-successor-'+str(n));c.pack('raw')
        need(c.a.alive(handle),'reader-ended-before-two-mapping-successors')
        c.verify('reader-held-old-generation')
        if kind=='error':
            c.a.release(control);response=c.a.wait(handle,10)
            need(response['status']=='error' and response['error']==spec['reader_error'],'source-mapped-reader-error')
            ended=True
        else:
            proof=c.a.interrupt(handle,'cancel' if kind=='cancel' else 'death')
            c.a.finish_interrupted(handle,'cancel' if kind=='cancel' else 'death');ended=True
            need(proof['identity_verified'] and proof['closed'],'native-reader-termination-proof')
            c.a.emit(dict(kind='reader-native-end',case_id=c.id,proof=proof))
        need(c.read(manifest,4*1024**2)==raw,'reader-end-revoked-durable-packet')
        need(c.call('resolve',request_id='initial')['result']==expected,'reader-end-changed-outcome')
        c.call('point',selected=saved,id='d000000')
        c.drain();return c.verify('reader-ended-durable-owners-survive')
    finally:
        if not ended:
            try:c.a.release(control)
            finally:
                if c.a.alive(handle):
                    try:c.a.interrupt(handle,'death');c.a.finish_interrupted(handle,'death')
                    except BaseException as error:c.a.emit(dict(kind='reader-finally-failure',case_id=c.id,error=repr(error)))

def maintainer_death(c,spec):
    donor(c)
    for number,phase in enumerate(('physical-before-selection','physical-after-selection')):
        c.publish('maintainer-death-tail-'+str(number))
        before_semantic=c.read('SELECTED.json');before=c.h.state(c.root)
        control=c.a.control(c.root,'maintainer-death-'+str(number))
        env=dict(EXP_BARRIER=phase,EXP_READY=control['ready'],EXP_RELEASE=control['release'])
        handle=c.a.spawn(lambda:c.h.call(c.root,'packing',operation='build',codec='raw',
                                         include_attempts=True,cleanup=False,force=True,env=env,timeout=60))
        ended=False
        try:
            c.a.await_ready(control,handle,10)
            proof=c.a.interrupt(handle,'death');c.a.finish_interrupted(handle,'death');ended=True
            need(proof['identity_verified'] and proof['closed'],'native-maintainer-death-required')
            need(c.read('SELECTED.json')==before_semantic,'maintainer-death-semantic-change')
            after=c.h.state(c.root)
            if phase=='physical-before-selection':
                need(before['store']==after['store'],'preselection-death-selected-generation')
            else:need(before['store']['physical']!=after['store']['physical'],'postselection-death-not-exercised')
            c.verify('native-death-'+phase)
            journal=c.document('PACKING.json')
            if journal['status']=='building':
                c.call('packing',operation='abort');c.pack('raw')
            else:c.call('packing',operation='resume',cleanup=False)
            c.drain();c.verify('native-death-recovered-'+phase)
        finally:
            if not ended:
                try:c.a.release(control)
                finally:
                    if c.a.alive(handle):
                        c.a.interrupt(handle,'death');c.a.finish_interrupted(handle,'death')
    return c.verify('both-native-maintainer-deaths')

def crash(c,spec):
    donor(c)
    phase=c.id.removeprefix('S-crash-')
    return schedules.crash_extension(c.h,c.root,c.a.forecast(c.id),phase=phase,label=phase)

def backup(c,spec):
    donor(c)
    saved=c.selected()
    c.call('lifecycle',operation='pin',name='retained-'+c.id,selected=saved)
    c.call('packet',selected=saved)
    request=dict(request=c.id+'-prepared',base=saved,ops=[],packet=True)
    candidate=c.call('prepare',request=request)['result']['candidate']
    c.publish(c.id+'-source-successor');c.pack('raw');c.h.cleanup(c.root,max_batches=160)
    before={p.relative_to(c.root).as_posix():digest(p) for p in (c.root/'physical-pins').glob('*.lock')}
    restored=c.a.backup_restore(c,'restored')
    after={p.relative_to(restored.root).as_posix():digest(p) for p in (restored.root/'physical-pins').glob('*.lock')}
    need(before and before==after,'every-durable-pin-lock-restored')
    need(restored.read(candidate,4*1024**2)==c.read(candidate,4*1024**2),'prepared-bytes-restored')
    restored.verify('complete-shallow-restoration')
    restored.publish(c.id+'-accepted-after-restore')
    restored.pack('raw');restored.drain()
    return restored.verify('restoration-accepted-successor')

def final_drain(c,spec):
    reports=[];excluded=[]
    for root in sorted(c.h.ledgers,key=str):
        if root==c.root or not (root/'STORE.json').is_file():continue
        if root in c.a.poisoned_roots:
            excluded.append(dict(root=str(root),reason='retained-intentionally-malformed-corpus-with-full-refusal-receipt'));continue
        config=c.h.document(root,'STORE.json',65536)
        if not config.get('physical'):
            excluded.append(dict(root=str(root),reason='never-a-physical-maintenance-lineage'));continue
        with c.a.existing_lineage(c.id,root) as lineage:
            passes=lineage.drain()
            need(len(passes)==2 and passes[-1]['backlog']['unprotected']==0,'exactly-two-cleanup-calls-per-maintained-lineage')
            reports.append(dict(root=str(root),passes=passes,oracle=lineage.verify('all-producers-ended-two-passes')))
    need(reports,'actual-maintained-lineage-roster')
    row=dict(kind='final-all-maintained-lineages-two-pass-drain',lineages=reports,excluded=excluded,
             hidden_cleanup_loop=False,new_publications_during_final_drain=0)
    c.a.emit(row);return row

def boundary_units(c,spec):
    # These are real compiled native Rust assertions at full boundary values.
    # The release map must identify each obligation and its test source lines;
    # a broad workspace pass without that map cannot satisfy a boundary case.
    from suite_units import run
    binding=c.a.m['boundary_test_bindings'][c.id]
    need(binding['status']=='reviewed-source-map' and binding['final_pin']==c.a.m['final_pin'],'boundary-source-review')
    need(binding['obligations']==spec['obligations'] and binding['exact_native_assertions'] is True,'all-boundary-obligations')
    proof=run(c)
    need(set(binding['test_functions'])<=set(proof['tests']) and binding['test_functions'],'actual-native-boundary-tests')
    need(binding['source_sha256'] in c.a.m['workspace']['source_sha256s'],'bound-Rust-test-source')
    row=dict(case_id=c.id,kind='native-boundary-assertions',binding=binding,workspace_proof=proof,
             fixture_manifest_sha256=proof['artifact_manifest_sha256'],compact_payload_substitute=False)
    c.a.emit(row)
    return row

def leaf(role,items,pack_hash,pack_bytes,codec=0):
    """CONSTRUCTOR only. All reads/authentication use unchanged paged_index."""
    need(items and len(items)<=paged_index.SLOTS and all(items[i][0]<items[i+1][0] for i in range(len(items)-1)),'constructor-leaf-order')
    def key(k):
        b=k.encode('ascii');need(len(b)<=225,'constructor-key-bound');return struct.pack('<H',len(b))+b
    first,last=items[0][0],items[-1][0]
    raw=b'FNPIDX02'+struct.pack('<BBBBIHQQ',0,paged_index.ROLES.index(role),codec,0,len(items),len(items),0,pack_bytes)
    raw+=bytes.fromhex(pack_hash)+struct.pack('<Q',pack_bytes)+key(first)+key(last)
    for k,offset,size,rawlen,h in items:raw+=key(k)+struct.pack('<QII',offset,size,rawlen)+bytes.fromhex(h)
    need(len(raw)<=16384,'constructor-single-page')
    d=dict(role=role,codec=codec,records=len(items),first=first,last=last,pack=pack_hash,pack_bytes=pack_bytes,
           index=sha(raw),index_bytes=len(raw),page=dict(version=2,hash=sha(raw),bytes=len(raw),height=0))
    need(paged_index.full(raw,d)['entries']==items,'constructor-authenticated-shared-reader')
    return raw,d

def outcome_categories(c,spec):
    if c.id in c.a.m.get('donor_cases',{}):donor(c)
    else:c.init('A',16)
    selections=[c.selected()]
    for n in range(20):c.publish('CP3-'+str(n));selections.append(c.selected())
    cancelled=dict(request='CP3-cancelled',base=c.selected(),ops=[],packet=False)
    c.h.ledgers[c.root].record(cancelled)
    c.call('cancel',request_id=cancelled['request'])
    lost=c.lost('CP3-lost','selection-boundary')
    c.pack('raw',bootstrap=True);c.h.cleanup(c.root,max_batches=160)
    accepted_names=list(c.h.ledgers[c.root].accepted)
    for name in (accepted_names[0],accepted_names[len(accepted_names)//2],'CP3-19'):
        need(c.call('resolve',request_id=name)['result']==c.h.ledgers[c.root].accepted[name]['result'],'CP3-exact-category')
    absent=c.call('resolve',request_id='CP3-never-attempted')['result']
    need(absent==dict(outcome='absent-as-of',terminal_noncommit=False),'CP3-authenticated-absent')
    need(c.call('resolve',request_id='CP3-cancelled')['result']['outcome']=='cancelled-fenced','CP3-fenced-category')
    need(c.call('resolve',request_id='CP3-lost')['result']==lost,'CP3-lost-reply-category')
    return c.verify('six-outcome-categories')

def paged_corruption(c,spec):
    # Authentic multi-page selected publication index; original default payload.
    c.init('A',10)
    for n in range(spec['paged_seed_publications']):c.publish('CP3-page-'+str(n))
    c.pack('raw',bootstrap=True,cleanup=False)
    c.h.cleanup(c.root,max_batches=160);c.verify('complete-healthy-multipage-donor')
    config,cat,d,ip,pp=parts(c);raw=c.read(ip,16*1024**2)
    rootref=paged_index.root_descriptor(d);node=paged_index.page(raw,d,rootref,last=d['last'])
    need(rootref['height']>0 and len(node['children'])>1,'actual-multipage-route-seed')
    # Follow the exact authenticated route to the oldest accepted key.
    key='publications/'+c.h.ledgers[c.root].accepted['initial']['selected']['publication']+'.json'
    ref=rootref
    while ref['height']:
        n=paged_index.page(raw,d,ref,last=d['last'] if ref==rootref else None)
        candidates=[x for x in n['children'] if x['first']<=key]
        need(candidates,'authenticated-route-candidate');ref=candidates[-1]
    touched=ref
    accepted_raw=c.a.oracle.decoder(c.root).raw(key)
    variants=[]
    for role in spec['corruption_roles']:
        x=c.clone('malformed-'+role);data=bytearray(raw)
        x.write(key,accepted_raw) # Exact loose duplicate must not mask mapped damage.
        if role=='route':offset=0
        elif role=='leaf':offset=touched['offset']
        elif role=='raw':
            decoder=x.a.oracle.decoder(x.root);entry=decoder.entries[key]
            name='packs/'+(entry[0]['instance']+'/' if entry[0].get('instance') else '')+entry[0]['pack']+'.pack'
            body=bytearray(x.read(name,64*1024**2));body[entry[1]]^=1;x.write(name,bytes(body));offset=None
        elif role=='untouched':
            other=next(ch for ch in node['children'] if not(ch['start']<=touched['start']<ch['end']))
            offset=other['offset']
        else:raise RuntimeError('closed-paged-corruption-role')
        if offset is not None:data[offset]^=1;x.write(ip,bytes(data))
        if role=='untouched':
            need(x.call('resolve',request_id='initial')['result']==x.h.ledgers[x.root].accepted['initial']['result'],
                 'untouched-damage-point-lookup-may-defer')
        else:
            response=x.call('resolve',request_id='initial',code=2)
            need(response['error'].startswith('integrity:'),'required-paged-corruption-must-refuse')
        variants.append(x.a.oracle.refuse(x.root,'CP3-'+role))
    c.verify('healthy-multipage-donor-preserved')
    return variants

def negative_segments(c,spec):
    donor(c,n=10)
    for cycle in range(3):
        c.publish('negative-segment-'+str(cycle));c.pack('raw',segment_limit=4)
    before=c.h.state(c.root)
    roles=[d for d in before['catalog']['packs'] if d['role']=='hint']
    need(len(roles)>=spec['minimum_relevant_segments'],'actual-relevant-segment-census')
    result=c.call('resolve',request_id='never-in-any-segment')['result']
    need(result==dict(outcome='absent-as-of',terminal_noncommit=False),'all-segment-negative-proof')
    # Every required segment's authenticated route must be consulted. A corrupt
    # segment cannot be bypassed by a negative result from another segment.
    proofs=[]
    for n,d in enumerate(roles):
        x=c.clone('negative-'+str(n));prefix='packs/'+(d['instance']+'/' if d.get('instance') else '')
        name=prefix+d['index']+'.idx';raw=bytearray(x.read(name,16*1024**2));raw[0]^=1;x.write(name,bytes(raw))
        need(x.call('resolve',request_id='never-in-any-segment',code=2)['error'].startswith('integrity:'),'skipped-relevant-negative-segment')
        proofs.append(x.a.oracle.refuse(x.root,'negative-segment-'+str(n)))
    c.verify('all-segment-authenticated-negative')
    return proofs

def candidate_verification(c,spec):
    donor(c);original=c;c=c.clone('malformed-unselected-candidate')
    c.publish('candidate-delta')
    c.pack('raw',cleanup=False,code=86,env={'EXP_EXIT':'physical-before-selection'})
    semantic=c.read('SELECTED.json');store=c.read('STORE.json')
    j=c.document('PACKING.json')
    # Exact source-reviewed journal candidate field, not a guessed pathname.
    binding=c.a.m['candidate_constructor']
    need(binding['status']=='reviewed-source-map' and binding['final_pin']==c.a.m['final_pin'],'candidate-journal-source-map')
    value=j
    for field in binding['candidate_catalog_fields']:value=value[field]
    catalog=c.document('pack-catalogs/'+value+'.json',65536)
    d=next(d for d in catalog['packs'] if d.get('page'))
    prefix='packs/'+(d['instance']+'/' if d.get('instance') else '')
    name=prefix+d['index']+'.idx';body=bytearray(c.read(name,16*1024**2));body[-1]^=1;c.write(name,bytes(body))
    need(c.call('packing',operation='resume',cleanup=False,code=2)['error'].startswith('integrity:'),'candidate-full-validation')
    need(c.read('STORE.json')==store and c.read('SELECTED.json')==semantic,'bad-candidate-selected')
    refusal=c.a.oracle.refuse(c.root,'candidate-full-verification')
    original.verify('healthy-donor-preserved-after-rejected-candidate')
    return refusal

def historical_phase(c,release,phase,root,steps):
    from suite_driver import JsonDriver
    from native_resources_v3 import canonical
    used=set()
    need(1<=len(steps)<=128,'finite-source-bound-old-operation-count')
    for row in steps:
        need(row['input']['root']==str(root) and row['input_sha256']==sha(canonical(row['input'])),
             'old-exact-root-input-binding')
    def policy(payload,env,code):
        c.a.validate_historical_context(c.id,phase,root)
        need(digest(release['producer_path'])==release['producer_binary_sha256'],'old-producer-changed')
        matches=[(i,row) for i,row in enumerate(steps) if row['input']==payload and row['env']==env and row['code']==code]
        need(len(matches)==1 and matches[0][0] not in used,'old-invocation-not-granted-or-repeated')
        i,row=matches[0];used.add(i)
        return dict(expected_error=row.get('expected_error'))
    class Producer(JsonDriver):
        def append(self,row,state):
            super().append(dict(row,historical_case_id=c.id,historical_phase=phase,
                                producer_authority_sha256=release['authority_sha256']),state)
    with c.a.historical_phase(c.id,phase,root):
        driver=Producer(release['producer_path'],c.a.output_root/('historical-'+uuid.uuid4().hex+'.jsonl'),
             expected_sha256=release['producer_binary_sha256'],suite_pin=c.a.m['final_pin'],
             binary_source_pin=release['producer_source_pin'],reservations=c.a.budget,
             historical_policy=policy,start_observer=c.a.started,wire_sink=c.a.wire_emit,suite_manifest=c.a.m)
        driver.case_id=c.id;driver.phase=phase
        c.a.register_historical(driver,c.id,phase,root)
        for row in steps:
            data=copy.deepcopy(row['input']);data.pop('root');action=data.pop('action')
            response=c.a.historical_call(driver,c.id,phase,root,action,code=row['code'],env=row['env'],**data)
            if row['code']==2:need(response['error']==row['expected_error'],'exact-old-client-refusal')
            request=row.get('capture_request')
            if request is not None:
                result=response['result'];need(result['outcome']=='accepted','actual-old-accepted-result')
                selected=c.a.h.selected(root)
                c.a.h.ledgers[root].accept(request,result,selected)
        need(len(used)==len(steps),'all-reviewed-historical-invocations-exercised')
    need(all(not d.live(token) for d in c.a.drivers for token in d.registry),'old-operations-ended-before-final')

def compatibility(c,spec):
    release=c.a.m['compatibility'][c.id]
    need(release['status']=='reviewed-producer-map' and release['final_pin']==c.a.m['final_pin'],'historical-review')
    need(release['producer_source_pin']!=c.a.m['final_pin'],'distinct-producer-final-pins')
    proofs=[]
    for version in release['store_versions']:
        root=c.a.alias(c.id,'legacy-store-'+str(version));d=Case(c.a,c.id,root)
        steps=release['producer_calls'][str(version)]
        historical_phase(c,release,'produce-store-'+str(version),root,steps)
        need(d.document('STORE.json')['version']==version,'actual-produced-store-version')
        need(d.h.ledgers[root].accepted,'complete-source-bound-old-inputs')
        d.call('lifecycle',operation='setup')
        d.pack('raw',bootstrap=True);d.h.cleanup(root,max_batches=160)
        d.verify('legacy-converted-'+str(version))
        historical_phase(c,release,'old-refusal-'+str(version),root,release['legacy_reader_refusal'][str(version)])
        restored=d.a.backup_restore(d,'legacy-restored-'+str(version))
        restored.verify('legacy-complete-restoration-'+str(version))
        restored.publish('accepted-after-legacy-restore')
        restored.pack('raw');restored.drain();proofs.append(restored.verify('legacy-successor-'+str(version)))
    return proofs


DONOR_CASES={
 'S-persistent-owners':'P/operational-A',
 'S-two-maintainers-semantic-crossing':'P/writer-and-competing-maintenance-A',
 'S-reader-error':'P/protected-read-A-resolve-history-captured',
 'S-reader-cancel':'P/protected-read-A-pack-catalog-captured',
 'S-reader-death':'P/protected-read-A-pack-record-located',
 'S-maintainer-death':'P/interruption-A-pack-accounting-1',
 'CP3-six-outcome-categories':'P/protected-read-A-pack-read-lease',
 'CP3-authenticated-negative-all-segments':'P/cancelled-drain-releases-barriers-A',
 'CP3-candidate-full-verification':'P/protected-read-A-pack-read-lease',
 'Backup-durable-pin-locks':'P/operational-A',
 'Backup-shallow-final-restore-successor':'P/operational-A',
 'S-crash-pack-reservation':'P/interruption-A-pack-reservation-1',
 'S-crash-pack-install':'P/interruption-A-pack-install-1',
 'S-crash-pack-before-catalog':'P/interruption-A-pack-before-catalog-1',
 'S-crash-physical-before-selection':'P/interruption-A-pack-before-catalog-1',
 'S-crash-physical-after-selection':'P/interruption-A-pack-catalog-1',
 'S-crash-pack-accounting':'P/interruption-A-pack-accounting-1',
 'S-crash-pack-loose-delete':'P/interruption-A-pack-loose-delete-1',
 'S-crash-pack-old-delete':'P/repack-old-pack-deletion-interruption-A',
 'S-crash-physical-after-owned-file-delete':'P/interruption-A-pack-install-1',
 'S-crash-physical-progress-plan-installed':'P/interruption-A-pack-accounting-1',
}

BOUNDARIES={
 'S-path-reuse-fresh-birth','D-terminal-oversize','D-deferred128-and-state32KiB',
 'D-segment4-and-collateral64MiB','Q-raw8MiB','Q-pack64MiB-index16MiB-record65536',
 'Q-catalog64-Work128KiB-cache64MiB','CP3-duplicates-segments',
}
EXTENSIONS={name:boundary_units for name in BOUNDARIES}
EXTENSIONS.update({name:shared for name in [
 'S-A-zstd','S-B-zstd','D-partial-fixed-cutoff','D-reservation-late-eligible',
 'D-cutoff-Q-stale-CAS','F1-terminal-retired-bootstrap-delta','F2-auto-decline-later-admit',
 'D-stable-base-segment-pressure','X-native-pressure']})
EXTENSIONS.update({
 'C-P0-small':p0,'S-persistent-owners':owners,'S-two-maintainers-semantic-crossing':semantic_contenders,
 'S-reader-error':reader_end,'S-reader-cancel':reader_end,'S-reader-death':reader_end,
 'S-maintainer-death':maintainer_death,'CP3-six-outcome-categories':outcome_categories,
 'CP3-authenticated-negative-all-segments':negative_segments,
 'CP3-touched-route-leaf-raw':paged_corruption,'CP3-untouched-full-audit':paged_corruption,
 'CP3-candidate-full-verification':candidate_verification,
 'Compat-STORE1-STORE2-to-new':compatibility,'Compat-STORE3-to-final':compatibility,
 'Backup-durable-pin-locks':backup,'Backup-shallow-final-restore-successor':backup,
 'Final-two-pass-drain':final_drain,
})
EXTENSIONS.update({'S-crash-'+phase:crash for phase in schedules.CRASH_PHASES})

PEER4=dict(path='/home/spenser/file-native-online-linux-20261020/target/native-peer-v3-targeted',
 sha256='fa2b5ae4ae2fe27bf3d5c320b2178bd3947ab01e1f028a2599fc8d38a224f770',
 source_pin='efd07cb5dee46a335be8c03a2f2c0462d7f923ca')
def validate_controls(m):
    need(m['runtime_scope']=='interface-controls-v4' and m['engine_case_qualification'] is False,'controls-scope-only')
    need(m['interface_peer']==PEER4 and digest(PEER4['path'])==PEER4['sha256'],'existing-reviewed-native-peer')
    need(m['final_pin']!=PEER4['source_pin'],'peer-producer-versus-new-harness-pin')
    need(m['native_guard_policy']['fixture_cap_bytes']==96*1024**2,'controls-current-F08-only')
    from suite_bootstrap import owned
    import native_guard as g
    owned(m['control_root'],g.ROOT/'fixtures')
    need(not Path(m['control_root']).exists() and m['control_case']['ceiling']['bytes']<=1024**2,'finite-fresh-control-root')
    need(m['control_case']['calls']['resolve/']['max_calls']==7 and
         m['control_case']['calls']['packing/build']['max_calls']==1,'exact-eight-peer-control-plan')

def interface_controls_v4(m,manifest_sha):
    """Fresh finite v4 controls ONLY. Never invokes an engine or CaseLedger."""
    import base64,copy,fcntl,threading
    import native_guard as g
    from native_resources_v3 import canonical
    from suite_adapter import suite_owner,SuiteReservations
    from suite_driver import JsonDriver
    from suite_bootstrap import verify_sources
    validate_controls(m);claim=suite_owner(m);root=Path(m['control_root']);out=Path(m['evidence_root'])
    results=[];raw_manifest=json.loads(Path(m['manifest_path']).read_bytes())
    for key in raw_manifest['source_files']:
        changed=copy.deepcopy(raw_manifest);changed['source_files'].pop(key)
        try:verify_sources(changed)
        except (RuntimeError,ValueError,KeyError) as e:results.append(dict(test='missing-source',key=key,refusal=str(e),peer_children=0))
        else:raise RuntimeError('missing source admitted:'+key)
    changed=copy.deepcopy(raw_manifest);changed['source_files']['suite/suite_driver.py'][1]='0'*64
    try:verify_sources(changed)
    except RuntimeError as e:results.append(dict(test='wrong-source-sha',refusal=str(e),peer_children=0))
    else:raise RuntimeError('wrong source SHA admitted')
    bad=copy.deepcopy(m);bad['manifest_sha256']='0'*64
    try:suite_owner(bad)
    except RuntimeError as e:results.append(dict(test='wrong-owner-policy',refusal=str(e),peer_children=0))
    else:raise RuntimeError('wrong owner policy admitted')
    slot='interface-v4';spec=dict(m['control_case'],root=str(root),forecast_sha256=sha(canonical(m['control_forecast'])))
    admission=dict(go=True,final_pin=m['final_pin'],claim_id=claim['claim_id'],ledger_sha256=claim['ledger_sha256'],cases={slot:spec})
    budget=SuiteReservations(m,claim,admission,expected_ledger_sha256=claim['ledger_sha256'])
    budget.preflight(slot,root,m['control_forecast'])
    budget.reserve_control('control-root',0,directories=8);root.mkdir(parents=True);budget.settle('control-root')
    wire=out/'controls-v4-wires.jsonl';budget.reserve_evidence('wire-file',4096,1)
    with wire.open('xb'):pass
    budget.settle('wire-file');lock=threading.RLock()
    def spool(row):
        raw=encoded(row)+b'\n'
        with lock:
            lease=budget.pending.get(row['reservation']);need(lease and len(raw)<=lease['evidence']['bytes'],'held-control-wire-lease')
            with wire.open('ab') as f:f.write(raw);f.flush()
    counter=dict(n=0)
    def policy(payload,env,code):
        need(payload['root']==str(root) and payload['action'] in {'resolve','packing'} and
             payload.get('peer') in {'fast','code2','exit86'} and code in {0,2,86},'closed-peer-only-input')
        need(set(payload)<={'root','action','peer','padding','operation'} and
             set(env or {})<={'EXP_PEER_EARLY_JSON','EXP_PEER_EARLY_READY','EXP_PEER_PARTIAL_JSON','EXP_READY','EXP_RELEASE'},'closed-peer-fields')
        counter['n']+=1;need(counter['n']<=8,'eight-peer-limit')
        return dict(expected_error='conflict:captured-base' if code==2 else None)
    d=JsonDriver(PEER4['path'],out/'controls-v4-events.jsonl',expected_sha256=PEER4['sha256'],
        suite_pin=m['final_pin'],binary_source_pin=PEER4['source_pin'],reservations=budget,
        historical_policy=policy,wire_sink=spool,suite_manifest=m)
    failure=None;worker=None
    def invoke(name,*,expected_failure=False,action='resolve',**kw):
        try:d.call(root,action,timeout=5,**kw)
        except AssertionError as error:
            need(expected_failure and len(error.args)==1 and type(error.args[0]) is dict,'exact-owned-control-row')
            row=error.args[0]
        else:
            need(not expected_failure,'missing-control-refusal');row=d.source_row(d.last)
        results.append(dict(test=name,call=row['call_token'],exit=row['exit'],
            input_expected=row['input_bytes_expected'],input_sent=row['input_bytes_sent'],
            full=row['input_fully_delivered'],violations=row['violations']))
        return row
    try:
        row=invoke('normal-full-input',peer='fast');need(row['input_fully_delivered'] and not row['violations'],'normal-wire')
        row=invoke('semantic-exit2',peer='code2',code=2);need(row['exit']==2 and not row['violations'],'semantic-only-exit2')
        ready=root/'early'
        def early(event):
            end=time.monotonic()+.2
            while not ready.exists() and time.monotonic()<end:time.sleep(.001)
            need(ready.read_bytes()==b'early-stdin-closed','real-peer-early-stdin-close')
        row=invoke('early-JSON-refuses',peer='fast',expected_failure=True,
            env={'EXP_PEER_EARLY_JSON':'1','EXP_PEER_EARLY_READY':str(ready)},on_started=early)
        need(row['input_bytes_sent']==0 and not row['input_fully_delivered'],'early-input-refusal')
        r,w=os.pipe()
        try:capacity=fcntl.fcntl(w,fcntl.F_GETPIPE_SZ)
        finally:os.close(r);os.close(w)
        need(capacity<=65536,'parent-reforecast-pipe-geometry')
        row=invoke('partial-BrokenPipe-refuses',peer='fast',padding='x'*(2*capacity+1024),
            env={'EXP_PEER_PARTIAL_JSON':'1'},expected_failure=True)
        need(0<row['input_bytes_sent']<row['input_bytes_expected'] and row['input_broken_pipe'],'partial-native-input-proof')
        invoke('injected86',peer='exit86',code=86)
        for label,action,kind in [('reader-cancel','resolve','cancel'),('reader-death','resolve','death'),('maintainer-death','packing','death')]:
            ready=root/(label+'-ready');release=root/(label+'-release');events=[];errors=[]
            fields=dict(peer='fast',env={'EXP_READY':str(ready),'EXP_RELEASE':str(release)},on_started=lambda event:events.append(event))
            if action=='packing':fields['operation']='build'
            def work():
                try:d.call(root,action,timeout=5,**fields)
                except BaseException as error:errors.append(error)
            worker=threading.Thread(target=work,daemon=False);worker.start()
            end=time.monotonic()+3
            while time.monotonic()<end and not(ready.exists() and events):time.sleep(.001)
            need(ready.read_bytes()==b'ready' and events and worker.is_alive(),'actual-owned-peer-ready')
            row=d.interrupt(events[0]['control_token'],kind);worker.join(5)
            need(not worker.is_alive() and len(errors)==1 and isinstance(errors[0],AssertionError) and
                 errors[0].args==(row,) and row['intentional_interrupt']==kind and not row['violations'],'exact-native-interrupt-row')
            need(row['engine_root_reaped'] and row['engine_monitor_joined'] and row['python_callback_joined'],'positive-native-interrupt-closure')
            results.append(dict(test=label,call=row['call_token'],pid=row['pid'],start=row['process_start_ticks'],exit=row['exit']))
        need(counter['n']==8,'all-eight-real-native-peer-calls')
    except BaseException as e:failure=repr(e);raise
    finally:
        d.close_owned('v4-control-finally')
        if worker is not None:worker.join(5);need(not worker.is_alive(),'control-Python-worker-joined')
        registry=d.controls()
        report=dict(kind='V4 INTERFACE CONTROLS ONLY; ZERO ENGINE CASES QUALIFIED',failure=failure,
            harness_pin=m['final_pin'],peer_producer=PEER4,manifest_sha256=manifest_sha,results=results,
            registry=registry,raw_events_sha256=digest(d.output),raw_wire_sha256=digest(wire),outer_guard_closure_required=True)
        raw=encoded(report)+b'\n';budget.reserve_evidence('report',len(raw),1)
        with (out/'controls-v4-report.json').open('xb') as f:f.write(raw)
        budget.settle('report')
        need(all(v['closed'] and v['resources_closed'] and v['callback_joined'] for v in registry.values()),'all-control-native-handles-closed')


