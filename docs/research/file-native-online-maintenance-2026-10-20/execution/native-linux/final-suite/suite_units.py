"""All actual final workspace test functions; one retained unit fixture lineage."""
import hashlib,re,uuid
from pathlib import Path
from suite_bootstrap import need,encoded,digest,owned,strict

def run(c):
    cached=getattr(c.a,'workspace_proof',None)
    if cached is not None:
        need(cached['final_pin']==c.a.m['final_pin'],'same-final-workspace-proof')
        return cached
    spec=c.a.m['workspace']
    c.init('A',16)
    need(spec['source_pin']==c.a.m['final_pin'] and spec['reviewed'] is True,'final-workspace-source-manifest')
    need(spec['expected_test_functions'] and len(set(spec['expected_test_functions']))==len(spec['expected_test_functions']),'all-real-test-identities')
    prior=spec.get('prior_native_unit_phase')
    if prior:
        need(prior['reviewed'] is True,'reviewed-prior-native-unit-phase')
        path=owned(prior['path']);need(digest(path)==prior['sha256'],'prior-unit-phase-SHA')
        phase=strict(path.read_bytes(),4*1024**2)
        from suite_driver import verify_phase_terminal
        need(prior['terminal']['receipt_sha256']==prior['sha256'],'prior-unit-terminal-receipt-SHA')
        phase=verify_phase_terminal(prior['terminal'],phase);proof=phase['result']
        need(phase['kind']=='V5_PHASE_FINAL' and phase['status']=='PASSED' and phase['scope']=='workspace-v5' and phase['final_pin']==c.a.m['final_pin'] and proof['stage']=='test','same-final31-phase')
        need(proof['source_files']=={k:([v['bytes'],v['sha256']] if type(v) is dict else v) for k,v in spec['source_files'].items()} and len(proof['tests'])==31 and set(proof['tests'])==set(spec['expected_test_functions']),'exact-source31-prior-unit-identities')
        stdout=owned(proof['stdout_path']);stderr=owned(proof['stderr_path'])
        need(digest(stdout)==proof['stdout_sha256'] and digest(stderr)==proof['stderr_sha256'],'retained-full-unit-streams')
        passed=re.findall(r"^test (.+) \.\.\. ok$",stdout.read_text(),re.M)
        need(len(passed)==31 and set(passed)==set(proof['tests']) and 'FAILED' not in stdout.read_text() and 'FAILED' not in stderr.read_text(),'all31-reverified-results')
        root=owned(proof['fixture_prefix']);actual={}
        for p in root.rglob('*'):
            need(not p.is_symlink(),'prior-unit-fixture-alias')
            if p.is_file():actual[p.relative_to(root).as_posix()]=dict(bytes=p.stat().st_size,sha256=digest(p))
        need(actual==proof['fixtures'],'complete-prior-unit-fixture-census')
        smoke=c.verify('prior-all31-native-units-same-final-smoke')
        result=dict(status='passed',final_pin=c.a.m['final_pin'],tests=proof['tests'],actual_test_functions=31,prior_phase_sha256=prior['sha256'],artifacts=actual,oracle=smoke,oracle_scope='smoke + exact31 prior native assertions and complete fixture bytes',no_heavy_rerun=True)
        c.a.emit(dict(kind='workspace-prior31-reuse',**result));c.a.workspace_proof=result;return result
    import native_guard as guard
    need(not (guard.ROOT/'fixtures'/('native-final31-'+c.a.m['final_pin'])).exists(),'prior-unit-attempt-needs-reviewed-reuse-binding')
    cfg=c.a.m.get('private_cache',{})
    need(cfg.get('enabled') is True and cfg.get('reviewed') is True and cfg.get('qualified_copy') and cfg.get('copy_go') is False,'no-global-Cargo-cache-fallback')
    need(owned(spec['manifest_path'],guard.ROOT/'source')==owned(spec['source_root'],guard.ROOT/'source')/'Cargo.toml','actual-final-unit-manifest')
    need(owned(spec['target_dir'],guard.ROOT/'target')==guard.ROOT/'target'/('native-v5-'+c.a.m['final_pin']),'unit-target-owned-fresh-lineage')
    for tool in [c.a.m['rustc'],c.a.m['rustdoc']]:need(digest(tool)==c.a.m['tools'][tool]['sha256'],'exact-unit-Rust-tool')
    fixture=c.root.parent/'unit-fixtures'
    need(not fixture.exists(),'one-fresh-unit-fixture-prefix')
    key='workspace-fixtures-'+uuid.uuid4().hex
    c.a.budget.reserve_control(key,spec['fixture_max_bytes'],files=spec['fixture_max_files'],directories=spec['fixture_max_directories'])
    fixture.mkdir(parents=True,exist_ok=False)
    env={'EXP_PACK_TEST_ROOT':str(fixture),'TMPDIR':str(fixture),'CARGO_INCREMENTAL':'0',
         'CARGO_NET_OFFLINE':'true','CARGO_HOME':c.a.m['private_cache']['root'] if c.a.m.get('private_cache',{}).get('enabled') else None,'RUSTC':c.a.m['rustc'],'RUSTDOC':c.a.m['rustdoc']}
    common=[c.a.m['cargo'],'test','--manifest-path',spec['manifest_path'],'--locked','--offline',
            '--workspace','--release','-j2','--target-dir',spec['target_dir']]
    try:
        listing=c.a.commands.run(c.root,'workspace',common+['--','--list'],seconds=spec['seconds'],env=env)
        names=re.findall(r'^(.+): test$',listing['stdout'],re.M)
        need(set(names)==set(spec['expected_test_functions']),'final-test-list-mismatch-no-five-slot-ceiling')
        result=c.a.commands.run(c.root,'workspace',common+['--','--test-threads=1'],seconds=spec['seconds'],env=env)
        passed=re.findall(r'^test (.+) \.\.\. ok$',result['stdout'],re.M)
        need(set(passed)==set(names),'not-every-listed-test-passed')
        need('FAILED' not in result['stdout'] and 'FAILED' not in result['stderr'],'workspace-failure')
    finally:c.a.budget.settle(key)
    artifacts={}
    for p in fixture.rglob('*'):
        need(not p.is_symlink(),'unit-artifact-symlink')
        if p.is_file():artifacts[p.relative_to(fixture).as_posix()]=dict(bytes=p.stat().st_size,sha256=digest(p))
    need(artifacts,'unit-fixture-census-required')
    # The generic ledger's full oracle is a genuine same-final engine smoke
    # oracle PLUS the complete distinct unit fixture census and actual test set.
    # It is not represented as semantic verification of non-store test indexes.
    smoke=c.verify('post-all-workspace-tests')
    proof=dict(status='passed',final_pin=c.a.m['final_pin'],tests=names,actual_test_functions=len(names),
               list_command=listing['token'],test_command=result['token'],artifacts=artifacts,
               artifact_manifest_sha256=hashlib.sha256(encoded(artifacts)).hexdigest(),
               oracle=smoke,oracle_scope='same-final-engine-smoke + all native test assertions + full unit fixture byte census',
               no_five_test_ceiling=True,unit_fixture_prefix=str(fixture))
    c.a.emit(dict(kind='workspace-all-actual-tests',**proof));c.a.workspace_proof=proof
    return proof
