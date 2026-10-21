"""Closed158-case dispatcher and post-guard evidence reconciliation.

A schedule completion is provisional. Only finalize(), after the native guard
has exited, may produce PASSED ledger rows from actual retained file hashes.
"""
import copy,hashlib,json,time
from pathlib import Path
import native_guard as guard
from case_ledger import CaseLedger,required_cases
from suite_bootstrap import need,digest,encoded,strict,owned
from suite_adapter import Host,suite_limits
import suite_packing as packing
import suite_recovery as recovery
import suite_ordinary as ordinary
import suite_units as units
from suite_extensions import EXTENSIONS,BOUNDARIES,DONOR_CASES

def expand(m):
    """Expand compact parent-reviewed rules into the identical158 exact IDs."""
    m=copy.deepcopy(m);cases=required_cases('Linux')
    ws=m['workspace']
    ws['source_files']={k:(dict(bytes=v[0],sha256=v[1]) if type(v) is list else v) for k,v in ws['source_files'].items()}
    ws['source_sha256s']=sorted({v['sha256'] for v in ws['source_files'].values()})
    ws['expected_test_functions']=[row[0] for row in ws['unit_catalog']]
    checkpoint=m['checkpoint_release'];m['checkpoints']={};m['crash_constructors']={}
    for phase,site in checkpoint['sites'].items():
        file,line=site
        row=dict(status=checkpoint['status'],final_pin=m['final_pin'],source_sha256=ws['source_files'][file]['sha256'],
                 source_file=file,source_line=line,ready_bytes_b64=__import__('base64').b64encode(phase.encode()).decode(),checkpoint=phase)
        m['checkpoints'][phase]=row;m['crash_constructors'][phase]=copy.deepcopy(row)
    m['capabilities']=[kind+phase for phase in m['checkpoints'] for kind in ['barrier:','exit:','constructor:']]
    release=m['case_release'];m['case_versions']={};m['case_families']={};m['case_specs']={}
    folders=m['implementation_directory']
    for case_id,row in cases.items():
        prefix=case_id.split('/',1)[0]
        implementation={'P':'suite_packing.py','R':'suite_recovery.py','I':'suite_packing.py',
            'F':'suite_packing.py','Workspace':'suite_units.py','Ordinary':'suite_ordinary.py'}.get(prefix,'suite_extensions.py')
        family=m['case_family_overrides'].get(case_id,m['group_families'].get(prefix,'new-shallow'))
        source=m['source_files'][str(Path(folders)/implementation)]['sha256']
        m['case_families'][case_id]=family
        m['case_versions'][case_id]=dict(status=release['status'],final_pin=m['final_pin'],source_sha256=source,
            version_mapping=release['version_mapping'],required_checks=[row['obligation']],
            entrypoint=EXTENSIONS[case_id].__name__ if case_id in EXTENSIONS else implementation+':dispatch')
        spec=copy.deepcopy(m['extension_defaults']);spec.update(m['case_spec_overrides'].get(case_id,{}))
        spec['obligations']=[row['obligation']];spec['constructor_reviewed']=release['constructors_reviewed']
        m['case_specs'][case_id]=spec
    m['selected_case_ids']=[k for k in cases if k!='Final-two-pass-drain']+['Final-two-pass-drain']
    # Unchanged source-reviewed templates define every action/operation pair.
    templates={name:copy.deepcopy(m['growth']['call_template']) for name in m['growth']['call_names']}
    for name in ['workspace/workspace','oracle/oracle']:
        templates[name].update(stdout_bytes=262144,stderr_bytes=262144,event_bytes=6*1024**2,input_bytes=262144)
    templates['dump/'].update(stdout_bytes=262144,stderr_bytes=65536,event_bytes=4*1024**2)
    for family in m['growth']['families'].values():
        family['forecast']['source_pin']=m['final_pin']
        family['calls']={}
        for name,profile in templates.items():
            p=copy.deepcopy(profile);p['max_calls']=family['call_counts'].get(name,family['call_counts'].get('*',0))
            if p['max_calls']:
                p['growth']=copy.deepcopy(family['call_growth'].get(name,m['growth']['call_growth'].get(name,family['default_call_growth'])))
                family['calls'][name]=p
    return m

def dispatch(c,spec):
    if c.id.startswith('P/'):return packing.packing_dispatch(c,c.id[2:])
    if c.id.startswith('R/'):return recovery.dispatch(c,c.id[2:])
    if c.id.startswith('I/'):return packing.strict_case(c,c.id[2:])
    if c.id.startswith('F/'):return packing.focused(c,c.id[2:])
    if c.id.startswith('Workspace/unit-slot-'):return units.run(c)
    if c.id=='Ordinary/all-routes':return ordinary.run(c)
    need(c.id in EXTENSIONS,'every-required-ID-has-concrete-dispatch')
    return EXTENSIONS[c.id](c,spec)

def validate(m):
    """Pure prelaunch validation; no Host, sinks, fixtures, import or children."""
    cases=required_cases('Linux');need(len(cases)==158,'exact-required-Linux-case-count')
    expected=set(cases)
    need(m['donor_cases']==DONOR_CASES,'closed21-existing-shallow-donor-reuse-map')
    for key in ['case_versions','case_families','case_specs']:
        need(set(m[key])==expected,'complete-exact-case-map:'+key)
    mapped={r['sha256'] for r in m['source_files'].values()}|set(m['workspace']['source_sha256s'])
    for case_id,row in m['case_versions'].items():
        need(row['status']=='reviewed-source-map' and row['final_pin']==m['final_pin'],'released-source-case:'+case_id)
        need(row['source_sha256'] in mapped and row['version_mapping'] and row['required_checks'],'actual-case-source-version:'+case_id)
        spec=m['case_specs'][case_id]
        need(spec['obligations'] and spec['constructor_reviewed'] is True,'all-source-reviewed-obligations:'+case_id)
        if not case_id.startswith(('P/','R/','I/','F/','Workspace/')) and case_id!='Ordinary/all-routes':
            need(case_id in EXTENSIONS,'missing-concrete-required-extension:'+case_id)
    # No manifest can declare a source-plan forecast to be runtime admission.
    growth=m['growth']
    need(growth['runtime_go'] is True and growth['parent_reviewed'] is True,'full-body-growth-NO_GO')
    limits=suite_limits(m)
    need(growth['fixture_cap_bytes']<=limits['fixtures'][0],'new-reviewed-fixture-cap')
    need(growth['evidence_cap_bytes']<=limits['evidence'][0],'new-reviewed-evidence-cap')
    need(growth['retained_total_upper_bytes']<=growth['fixture_cap_bytes'],'all-retained-lineages-cannot-fit')
    need(growth['no_gc_credit'] is True and growth['failed_B_generations_retained'] is True,'no-forecast-only-deletion-credit')
    for case_id in cases:
        family=growth['families'][m['case_families'][case_id]]
        need(set(family['ceiling'])=={'bytes','files','directories'},'source-reviewed-family-ceiling')
        need(family['max_roots']>=1 and family['oracle_output_bytes']>0,'complete-family-profile')
        need(family['forecast']['source_pin']==m['final_pin'],'growth-same-final-source')
        need(family['calls'] and family['controls']['max_controls']>=1,'finite-call-control-profile')
    ws=m['workspace']
    for name,row in ws['source_files'].items():
        p=owned(Path(ws['source_root'])/name,guard.ROOT/'source')
        need(p.stat().st_size==row['bytes'] and digest(p)==row['sha256'],'same-final-Rust-Cargo-source:'+name)
    need(ws['reviewed'] is True and ws['source_pin']==m['final_pin'],'final-workspace-release')
    need(len(ws['expected_test_functions'])>=20 and len(set(ws['expected_test_functions']))==len(ws['expected_test_functions']),'all-actual-workspace-tests')
    need(len(ws['source_sha256s'])>=1,'actual-unit-source-hashes')
    need(ws['no_private_cache'] is True and ws['fixture_max_bytes']>=80*1024**2,'unit80MiB-plus-new-test-fixtures')
    need(set(m['boundary_test_bindings'])==BOUNDARIES,'closed-boundary-assertion-catalog')
    for name in BOUNDARIES:
        b=m['boundary_test_bindings'][name]
        need(b['status']=='reviewed-source-map' and b['final_pin']==m['final_pin'],'boundary-source-release:'+name)
        need(b['test_functions'] and set(b['test_functions'])<=set(ws['expected_test_functions']),'actual-boundary-test-identities:'+name)
        need(b['source_sha256'] in ws['source_sha256s'] and b['exact_native_assertions'] is True,'exact-boundary-assertions:'+name)
        need(b['obligations']==m['case_specs'][name]['obligations'],'boundary-obligations-not-reduced:'+name)
    for phase,row in m['checkpoints'].items():
        need(row['status']=='reviewed-source-map' and row['final_pin']==m['final_pin'] and row['source_sha256'] in mapped,'native-checkpoint-source:'+phase)
    required_phases={'physical-before-selection','physical-after-selection','pack-before-catalog',
        'pack-read-lease','pack-catalog-captured','resolve-history-captured','pack-record-located'}
    need(required_phases<=set(m['checkpoints']),'all-held-read-and-maintainer-native-checkpoints')
    need(set(m['compatibility'])=={'Compat-STORE1-STORE2-to-new','Compat-STORE3-to-final'},'all-legacy-version-producers')
    for name,b in m['compatibility'].items():
        need(b['status']=='reviewed-producer-map' and b['final_pin']==m['final_pin'],'explicit-legacy-source-release:'+name)
        need(b['producer_source_pin']!=m['final_pin'] and b['producer_binary_sha256'] and b['authority_sha256'],'historical-source-not-final:'+name)
        need(b['producer_calls'] and b['store_versions'] and b['legacy_reader_refusal'],'complete-version-transition-controls:'+name)
    need(m['templates_reviewed'] and m['capabilities_reviewed'],'all-template-native-hook-source-review')
    for name in [m['python'],m['bash'],m['cargo']]:
        need(name in m['tools'] and digest(name)==m['tools'][name]['sha256'],'actual-native-toolchain-binding')
    need('/.cargo/bin/' not in m['cargo'],'actual-toolchain-Cargo-required-not-rustup-dispatcher')
    need(m['candidate_constructor']['status']=='reviewed-source-map','exact-unselected-candidate-journal-fields')
    need(m['oracle']['reviewed'] and m['oracle']['final_pin']==m['final_pin'],'full-independent-oracle-release')
    need(m['selected_case_ids']==[k for k in cases if k!='Final-two-pass-drain']+['Final-two-pass-drain'],'all158-case-order-final-drain-last')
    return cases

def oracle_receipts(value):
    answer=[]
    def walk(v):
        if isinstance(v,dict):
            if v.get('receipt') and v.get('receipt_sha256'):
                p=owned(v['receipt'],guard.ROOT/'evidence')
                need(p.is_file() and digest(p)==v['receipt_sha256'],'actual-oracle-receipt-binding')
                answer.append(dict(path=str(p),sha256=v['receipt_sha256']))
            for x in v.values():walk(x)
        elif isinstance(v,(list,tuple)):
            for x in v:walk(x)
    walk(value)
    return list({x['path']:x for x in answer}.values())

def run(m,manifest_sha):
    m=expand(m)
    cases=validate(m) # Every blocker is resolved before creating even a sink.
    host=Host(m,manifest_sha);ledger=CaseLedger('Linux',m['final_pin'])
    started=time.time();provisional=[];failure=None
    for case_id in m['selected_case_ids']:
        first=len(host.h.oracles);result=None
        try:
            with host.case(case_id) as c:result=dispatch(c,m['case_specs'][case_id])
            receipts=oracle_receipts([result,host.h.oracles[first:]])
            need(receipts,'real-full-oracle-or-complete-expected-refusal-for-every-case')
            observation=dict(case_id=case_id,status='PENDING',
                reason='schedule complete; final native guard receipt and raw file binding unavailable until guard exits',
                schedule_completed=True,oracle_receipts=receipts,final_pin=m['final_pin'],
                case_source_sha256=m['case_versions'][case_id]['source_sha256'],
                version_mapping=m['case_versions'][case_id]['version_mapping'],
                required_checks=m['case_versions'][case_id]['required_checks'],
                host='Linux',full_oracle=True,native=True,source_verified=True,processes_closed=True)
        except BaseException as error:
            failure=repr(error)
            observation=dict(case_id=case_id,status='FAILED',reason=failure,schedule_completed=False)
        ledger.record(case_id,observation);provisional.append(observation)
        if not failure:host.case_results[case_id]=copy.deepcopy(observation)
        host.emit(dict(kind='provisional-required-case',observation=observation))
        if failure:break # Failure/censor never triggers an automatic rerun.
    raw_paths=[host._sink,host.wire_output,host.driver.output,*[d.output for d in host.drivers if d is not host.driver]]
    raw={str(p):dict(bytes=p.stat().st_size,sha256=digest(p)) for p in raw_paths}
    packet=dict(kind='PROVISIONAL; NO CASE PASSED BEFORE OUTER GUARD',manifest_path=m['manifest_path'],
        manifest_sha256=manifest_sha,final_pin=m['final_pin'],candidate=m['candidate'],
        claim=m['claim'],started_epoch=started,ended_epoch=time.time(),failure=failure,
        observations=provisional,ledger=ledger.result(),raw_files=raw,source_files=m['source_files'],
        required_case_ids=list(cases),guard_nonce=host.driver.owner['nonce'],final_native_registry=host.final_registry)
    out=host.output_root/'suite-provisional.json';raw=encoded(packet)+b'\n'
    token='provisional-ledger';host.budget.reserve_evidence(token,len(raw),1)
    with out.open('xb') as stream:stream.write(raw)
    host.budget.settle(token)
    if failure:raise RuntimeError('final-suite-stopped-no-retry:'+failure)
    return packet

def finalize(m,packet,guard_path):
    """Return a ledger; caller reserves and retains its one new output file."""
    gpath=owned(guard_path,guard.ROOT/'evidence');g=strict(gpath.read_bytes(),262144)
    need(g['source_pin']==m['final_pin'] and g['claim_id']==m['claim']['claim_id'] and
         g['ledger_sha256']==m['claim']['ledger_sha256'] and g['owner_nonce']==packet['guard_nonce'],'same-source-claim-guard')
    need(g['exit']==0 and not g['censors'] and not g['observation_errors'],'uncensored-outer-guard')
    need(g['monitors_joined'] and g['process_closure_verified'] and len(g['closure_observations'])==2 and
         all(not row['pids'] for row in g['closure_observations']),'actual-two-empty-native-closure')
    need(g['rss_sampling_cadence_qualified'] and g['memory_intervals_over_40ms']==0,'accepted-native-cadence')
    need(digest(g['memory_samples_path'])==g['memory_samples_sha256'],'raw-memory-artifact-hash')
    need(packet['failure'] is None and packet['manifest_sha256']==digest(packet['manifest_path']),'completed-source-bound-packet')
    command=g['command']
    need('--manifest' in command and '--sha256' in command and
         command[command.index('--manifest')+1]==packet['manifest_path'] and
         command[command.index('--sha256')+1]==packet['manifest_sha256'],'same-guard-command-manifest')
    from suite_bootstrap import verify_raw_packet,verify_guard_memory
    from suite_driver import verify_guard_policy
    verify_guard_policy(m,packet,g)
    raw_proof=verify_raw_packet(m,packet)
    raw_proof['memory']=verify_guard_memory(g)
    for name,row in packet['source_files'].items():
        need(Path(name).stat().st_size==row['bytes'] and digest(name)==row['sha256'],'unchanged-preimport-source-closure')
    for name,row in packet['raw_files'].items():
        need(Path(name).stat().st_size==row['bytes'] and digest(name)==row['sha256'],'retained-full-raw-file')
    ledger=CaseLedger('Linux',m['final_pin'])
    need(packet['required_case_ids']==list(ledger.rows),'exact-all158-case-obligations')
    raw_sha=hashlib.sha256(encoded(packet['raw_files'])).hexdigest()
    for obs in packet['observations']:
        need(obs['schedule_completed'] and obs['status']=='PENDING','no-failed-or-missing-schedule')
        oracle_receipts(obs)
        result=dict(obs,status='PASSED',raw_sha256=raw_sha,binary_sha256=m['candidate']['sha256'],
                    guard_receipt_sha256=digest(gpath),required_checks_complete=True)
        ledger.record(obs['case_id'],result)
    answer=ledger.result()
    answer['raw_verification']=raw_proof
    need(not answer['missing'],'all158-required-evidence-present')
    return answer

