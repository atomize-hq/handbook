"""All actual final workspace test functions; one retained unit fixture lineage."""
import hashlib,re,uuid
from pathlib import Path
from suite_bootstrap import need,encoded,digest

def run(c):
    cached=getattr(c.a,'workspace_proof',None)
    if cached is not None:
        need(cached['final_pin']==c.a.m['final_pin'],'same-final-workspace-proof')
        return cached
    spec=c.a.m['workspace']
    c.init('A',16)
    need(spec['source_pin']==c.a.m['final_pin'] and spec['reviewed'] is True,'final-workspace-source-manifest')
    need(spec['expected_test_functions'] and len(set(spec['expected_test_functions']))==len(spec['expected_test_functions']),'all-real-test-identities')
    fixture=c.root.parent/'unit-fixtures'
    need(not fixture.exists(),'one-fresh-unit-fixture-prefix')
    key='workspace-fixtures-'+uuid.uuid4().hex
    c.a.budget.reserve_control(key,spec['fixture_max_bytes'],files=spec['fixture_max_files'],directories=spec['fixture_max_directories'])
    fixture.mkdir(parents=True,exist_ok=False)
    env={'EXP_PACK_TEST_ROOT':str(fixture),'TMPDIR':str(fixture),'CARGO_INCREMENTAL':'0',
         'CARGO_NET_OFFLINE':'true','CARGO_HOME':None}
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
