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
    need(m['runtime_scope']=='interface-controls-v5' and m['engine_case_qualification'] is False,'controls-scope-only')
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
    slot='interface-v5';spec=dict(m['control_case'],root=str(root),forecast_sha256=sha(canonical(m['control_forecast'])))
    admission=dict(go=True,final_pin=m['final_pin'],claim_id=claim['claim_id'],ledger_sha256=claim['ledger_sha256'],cases={slot:spec})
    budget=SuiteReservations(m,claim,admission,expected_ledger_sha256=claim['ledger_sha256'])
    budget.preflight(slot,root,m['control_forecast'])
    budget.reserve_control('control-root',0,directories=8);root.mkdir(parents=True);budget.settle('control-root')
    wire=out/'controls-v5-wires.jsonl';budget.reserve_evidence('wire-file',4096,1)
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
    d=JsonDriver(PEER4['path'],out/'controls-v5-events.jsonl',expected_sha256=PEER4['sha256'],
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
        report=dict(kind='V5 INTERFACE CONTROLS ONLY; ZERO ENGINE CASES QUALIFIED',failure=failure,
            harness_pin=m['final_pin'],peer_producer=PEER4,manifest_sha256=manifest_sha,results=results,
            registry=registry,guard_nonce=claim['nonce'],claim=m['claim'],raw_files={str(p):dict(bytes=p.stat().st_size,sha256=digest(p)) for p in [d.output,wire]},raw_events_sha256=digest(d.output),raw_wire_sha256=digest(wire),outer_guard_closure_required=True)
        raw=encoded(report)+b'\n';budget.reserve_evidence('report',len(raw),1)
        with (out/'controls-v5-report.json').open('xb') as f:f.write(raw)
        budget.settle('report')
        need(all(v['closed'] and v['resources_closed'] and v['callback_joined'] for v in registry.values()),'all-control-native-handles-closed')



# Shared post-exit proofs; loaded only after closed source admission.
from suite_bootstrap import owned,strict
import native_guard as guard
ROOT=guard.ROOT
def verify_raw_packet(m,packet):
    """Read every retained native call, chunk and direct closure; no subprocess."""
    import base64
    starts={};calls={};chunks={};wires={};commands=[];closures={};registry=[]
    for name,row in packet['raw_files'].items():
        p=owned(name,ROOT/'evidence')
        need(p.stat().st_size==row['bytes'] and digest(p)==row['sha256'],'full-raw-artifact')
        with p.open('rb') as stream:
            for line in stream:
                event=strict(line,16*1024**2);kind=event.get('kind')
                if kind=='native-positive-start':
                    token=event['control_token'];need(token not in starts,'duplicate-native-start');starts[token]=event
                elif kind=='native-call':
                    token=event['call_token'];need(token not in calls,'duplicate-native-call');calls[token]=event
                elif kind=='native-raw-chunk':
                    key=(event['call'],event['stream']);need(event['stream'] in {'stdout','stderr'},'raw-stream-name')
                    data=base64.b64decode(event['bytes_b64'],validate=True)
                    prior=chunks.setdefault(key,bytearray());need(event['offset']==len(prior),'contiguous-full-native-stream')
                    prior.extend(data)
                elif kind=='native-command':commands.append(event)
                elif kind=='suite-native-input-wire':
                    token=event['call'];need(token not in wires,'one-native-input-wire');wires[token]=event
                elif kind=='case-closure':
                    case_id=event['case_id'];need(case_id not in closures,'duplicate-case-closure');closures[case_id]=event
                    c=event['closure']
                    while len(registry)<len(c['registries']):registry.append({})
                    need(len(registry)==len(c['registries']),'stable-driver-registry-order')
                    for old,delta in zip(registry,c['registries']):
                        need(not(set(old)&set(delta)),'immutable-closed-registry-row');old.update(delta)
                    need(hashlib.sha256(encoded(registry)).hexdigest()==c['global_registry_sha256'] and
                         sum(len(r) for r in registry)==c['global_registry_entries'],'lossless-exact-global-closure-reconstruction')
    need(starts and set(starts)==set(calls),'every-native-start-has-one-final-row')
    need(set(wires)==set(calls),'exact-wire-artifact-for-every-native-call')
    need(registry==packet['final_native_registry'] and all(r['closed'] and r['resources_closed'] and r['callback_joined']
         for rows in registry for r in rows.values()),'retained-full-final-registry')
    need(all(key[0] in calls for key in chunks),'no-orphan-raw-chunks')
    for token,row in calls.items():
        start=starts[token]
        need(row['guard_nonce']==packet['guard_nonce'] and row['source_pin']==m['final_pin'] and
             row['suite_pin']==m['final_pin'] and row['claim_id']==m['claim']['claim_id'],'native-call-claim-source')
        need(start['pid']==row['pid'] and start['process_start_ticks']==row['process_start_ticks'] and
             start['input']==row['input'] and start['env']==row['env'],'same-retained-native-call-identity')
        need(start['before_stdin_delivery'] is True and row['observed_ns']<=row['stdin_ns']<=row['done_ns'],'positive-native-start-before-input')
        need(row['initial_blocked_rss']>0 and row['actual_exec_rss']>0 and
             row['observed_peak_rss']>0 and row['observed_peak_rss']<=row['rss_limit'],'real-positive-role-RSS')
        need(row['engine_root_reaped'] and row['engine_monitor_joined'] and row['python_callback_joined'] and
             not row['violations'],'uncensored-native-call-closure')
        if row['binary_role']=='candidate':
            need(row['executable_sha256']==m['candidate']['sha256'] and row['binary_source_pin']==m['final_pin'],'same-final-ELF')
        else:
            binding=m['compatibility'][row['historical_case_id']]
            need(row['binary_role']=='historical' and row['executable_sha256']==binding['producer_binary_sha256'] and
                 row['binary_source_pin']==binding['producer_source_pin']!=m['final_pin'],'distinct-granted-historical-ELF')
        wire=base64.b64decode(wires[token]['bytes_b64'],validate=True)
        need(wires[token]['before_stdin_delivery'] and len(wire)==wires[token]['bytes'] and
             hashlib.sha256(wire).hexdigest()==wires[token]['sha256'] and strict(wire,4*1024**2)==row['input'],'native-raw-wire-json-binding')
        need(row['input_fully_delivered'] and row['stdin_closed'] and not row['input_broken_pipe'] and
             len(wire)==row['input_bytes_expected']==row['input_bytes_sent'],'complete-engine-input')
        need(hashlib.sha256(wire).hexdigest()==row['input_wire_sha256']==row['input_written_prefix_sha256'],'exact-native-input-wire')
        for stream in ['stdout','stderr']:
            raw=bytes(chunks.get((token,stream),b''))
            need(hashlib.sha256(raw).hexdigest()==row[stream+'_sha256'] and
                 raw.decode(errors='replace')==row[stream],'complete-native-raw-stream')
        if row['intentional_interrupt']:
            need(row['intentional_interrupt'] in {'cancel','death'} and row['exit'] in {-9,-15},'exact-intentional-native-termination')
        elif row['exit']==86:need(row['env'] and row['env'].get('EXP_EXIT'),'source-declared-exit86')
        else:need(row['exit'] in {0,2},'unexpected-native-exit')
    for row in commands:
        need(row['guard_nonce']==packet['guard_nonce'] and row['qualification_pin']==m['final_pin'],'same-native-command-stage')
        need(row['root_reaped'] and row['watcher_joined'] and not row['errors'],'native-command-closure')
        proof=row['actual_exec']
        need(proof and proof['before_stdin_delivery'] and proof['rss']>0 and proof['inode']>0 and
             proof['sha256']==row['binary_sha256']==m['tools'][row['argv'][0]]['sha256'],'actual-command-ELF-inode-RSS-proof')
        need(row['initial_blocked_rss']>0 and row['peak_rss']>0 and row['peak_rss']<=row['rss_limit'],'native-command-role-memory')
        raw=base64.b64decode(row['stdin_bytes_b64'],validate=True)
        need(row['input_fully_delivered'] and row['stdin_closed'] and
             len(raw)==row['stdin_bytes']==row['input_bytes_sent'] and hashlib.sha256(raw).hexdigest()==row['stdin_sha256'],'full-command-input')
        for stream in ['stdout','stderr']:
            raw=base64.b64decode(row[stream+'_bytes_b64'],validate=True)
            need(hashlib.sha256(raw).hexdigest()==row[stream+'_sha256'] and raw.decode(errors='replace')==row[stream],'full-command-raw-stream')
    for row in packet['observations']:
        case_id=row['case_id'];need(case_id in closures,'actual-direct-case-closure')
        end=closures[case_id];c=end['closure']
        need(end['failure'] is None and c['processes_closed'] and c['handles_closed'] and
             c['python_callbacks_joined'] and not c['errors'],'case-not-positively-closed')
    return dict(engine_calls=len(calls),generic_native_commands=len(commands),direct_case_closures=len(closures),
                all_wire_hashes_and_raw_streams_verified=True)
def verify_guard_memory(g):
    import zlib
    path=owned(g['memory_samples_path'],ROOT/'evidence')
    need(digest(path)==g['memory_samples_sha256'],'guard-memory-file-hash')
    decoder=zlib.decompressobj(16+zlib.MAX_WBITS)
    pending=b'';count=0;prior=None;maximum=0.;positive=0;zombies=0
    def sample(raw):
        nonlocal count,prior,maximum,positive,zombies
        row=strict(raw,1024**2);count+=1
        need(count<=g['memory_sample_count'],'unexpected-extra-memory-row')
        epoch=row['epoch']
        if prior is not None:
            gap=epoch-prior;need(0<=gap<=.04,'raw-memory-cadence-not-qualified');maximum=max(maximum,gap)
        prior=epoch
        live=[];z=set(row['zombie_pids']);zombies+=len(z)
        for member in row['members']:
            need(g['tracked_identity'].get(str(member['pid']))==member['start'],'retained-guard-PID-start')
            if member['pid'] in z:
                need(member['rss'] is None or member['rss']>=0,'explicit-zombie-RSS')
            else:
                need(type(member['rss']) is int and member['rss']>0 and
                     type(member['hwm']) is int and member['hwm']>=member['rss'],'positive-live-native-RSS')
                live.append(member['rss']);positive+=1
        need(sum(live)==row['heavy_rss'] and row['heavy_rss']<=g['heavy_rss_limit'] and
             all(r<=g['process_rss_limit'] for r in live),'actual-heavy-and-process-memory')
        need(0<row['supervisor_rss']<=g['supervisor_rss_limit'] and
             row['heavy_rss']+row['supervisor_rss']<g['native_host_rss_limit'],'actual-supervisor-host-memory')
    with path.open('rb') as stream:
        for block in iter(lambda:stream.read(65536),b''):
            need(not decoder.eof,'gzip-trailing-member-or-bytes')
            incoming=block
            while incoming:
                piece=decoder.decompress(incoming,1024**2);incoming=decoder.unconsumed_tail
                pending+=piece
                while b'\n' in pending:
                    line,pending=pending.split(b'\n',1);need(bool(line),'blank-memory-row');sample(line)
                need(len(pending)<=1024**2,'memory-row-byte-bound')
                need(not decoder.unused_data,'gzip-trailing-member-or-bytes')
    need(decoder.eof and not decoder.unused_data and not decoder.unconsumed_tail and not pending,'complete-single-gzip-member')
    need(count==g['memory_sample_count'] and count>=2 and positive>0,'complete-positive-native-memory-series')
    return dict(samples=count,last_epoch=prior,raw_max_interval_seconds=maximum,positive_live_member_observations=positive,
                explicit_zombie_observations=zombies,single_gzip_member=True)


def finalize_controls(m,packet,g):
    """Verify eight NEW peer calls and their typed negative input proofs."""
    import base64
    need(packet['failure'] is None and packet['manifest_sha256']==m['manifest_sha256'] and packet['claim']==m['claim'] and packet['guard_nonce']==g['owner_nonce'],'controls-source-owner')
    names=['normal-full-input','semantic-exit2','early-JSON-refuses','partial-BrokenPipe-refuses','injected86','reader-cancel','reader-death','maintainer-death']
    negative=[r for r in packet['results'] if r['test']=='missing-source']
    need(len(negative)==38 and len({r['key'] for r in negative})==38 and all(r['peer_children']==0 and r['refusal'] for r in negative),'all38-prepeer-refusals')
    for label in ['wrong-source-sha','wrong-owner-policy']:
        rows=[r for r in packet['results'] if r['test']==label];need(len(rows)==1 and rows[0]['peer_children']==0 and rows[0]['refusal'],'prepeer-negative:'+label)
    tests=[r for r in packet['results'] if r['test'] in names];need([r['test'] for r in tests]==names,'exact8-new-controls')
    starts={};calls={};wires={};chunks={}
    for name,info in packet['raw_files'].items():
        p=owned(name,ROOT/'evidence');need(p.stat().st_size==info['bytes'] and digest(p)==info['sha256'],'complete-control-raw')
        with p.open('rb') as stream:
            for raw in stream:
                r=strict(raw,4*1024**2);kind=r.get('kind')
                if kind in {'native-positive-start','native-call','suite-native-input-wire'}:
                    table,key=(starts,r.get('control_token')) if kind=='native-positive-start' else (calls,r.get('call_token')) if kind=='native-call' else (wires,r.get('call'))
                    need(key and key not in table,'unique-native-row');table[key]=r
                elif kind=='native-raw-chunk':
                    key=(r['call'],r['stream']);data=base64.b64decode(r['bytes_b64'],validate=True);old=chunks.setdefault(key,bytearray());need(r['offset']==len(old),'contiguous-control-output');old.extend(data)
    tokens={r['call'] for r in tests};need(len(tokens)==8 and tokens==set(starts)==set(calls)==set(wires)==set(packet['registry']),'exact8-raw-registry')
    need(all(r['closed'] and r['resources_closed'] and r['callback_joined'] for r in packet['registry'].values()),'control-handles-joined')
    for test in tests:
        token=test['call'];r=calls[token];a=starts[token];w=wires[token];wire=base64.b64decode(w['bytes_b64'],validate=True);label=test['test']
        need(r['guard_nonce']==g['owner_nonce'] and r['claim_id']==m['claim']['claim_id'] and r['suite_pin']==m['final_pin'],'native-control-claim')
        need(r['executable_sha256']==PEER4['sha256'] and r['binary_source_pin']==PEER4['source_pin']!=m['final_pin'],'peer-only-producer')
        need(a['pid']==r['pid'] and a['process_start_ticks']==r['process_start_ticks'] and a['input']==r['input'] and a['env']==r['env'],'control-PID-start')
        need(a['before_stdin_delivery'] and r['observed_ns']<=r['stdin_ns']<=r['done_ns'] and r['actual_exec_rss']>0 and r['initial_blocked_rss']>0 and 0<r['observed_peak_rss']<=r['rss_limit'],'positive-native-control-start')
        need(r['engine_root_reaped'] and r['engine_monitor_joined'] and r['python_callback_joined'],'native-control-closure')
        need(w['before_stdin_delivery'] and len(wire)==w['bytes']==r['input_bytes_expected'] and strict(wire,262144)==r['input'] and hashlib.sha256(wire).hexdigest()==r['input_wire_sha256']==w['sha256'],'complete-control-wire')
        sent=r['input_bytes_sent'];need(0<=sent<=len(wire) and hashlib.sha256(wire[:sent]).hexdigest()==r['input_written_prefix_sha256'],'actual-written-prefix')
        if label in {'early-JSON-refuses','partial-BrokenPipe-refuses'}:
            need(r['violations']==['response-before-complete-input','incomplete-input-delivery'] and r['exit']==0 and not r['input_fully_delivered'],'typed-negative-input')
            need((sent==0) if label=='early-JSON-refuses' else (0<sent<len(wire) and r['input_broken_pipe']),'early-versus-BrokenPipe')
        else:need(not r['violations'] and r['input_fully_delivered'] and r['stdin_closed'] and sent==len(wire),'fully-delivered-normal-or-interrupted-input')
        for stream in ['stdout','stderr']:
            raw=bytes(chunks.get((token,stream),b''));need(hashlib.sha256(raw).hexdigest()==r[stream+'_sha256'] and raw.decode(errors='replace')==r[stream],'full-control-output')
        if label.endswith('cancel'):need(r['intentional_interrupt']=='cancel' and r['exit']==-15,'real-reader-cancel')
        elif label.endswith('death'):need(r['intentional_interrupt']=='death' and r['exit']==-9,'real-owned-death')
        elif label=='injected86':need(r['exit']==86 and r['input']['peer']=='exit86','explicit-peer86')
        elif label=='semantic-exit2':need(r['exit']==2,'semantic2-only')
        else:need(r['exit']==0,'peer-exit0')
    need(all(k[0] in tokens and k[1] in {'stdout','stderr'} for k in chunks),'no-orphan-output')
    return dict(status='PASSED',scope='interface-controls-v5',engine_cases_qualified=0,peer_calls=8,prepeer_refusals=40,raw_files=packet['raw_files'])
