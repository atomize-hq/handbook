"""Verify final measured product source and the common driver before timing."""
import hashlib,json,pathlib
HERE=pathlib.Path(__file__).resolve().parent;PRIMARY=HERE.parents[2]
names=['pristine-baseline-v2','pristine-refactor-v2','profile-baseline-v3','profile-refactor-v3']
builds={name:json.loads((HERE/f'build-{name}.json').read_text()) for name in names}
driver_key='docs/research/work-registration-phase-refactor-2026-10-11/native/src/main.rs'
driver_hashes={build['sources'][driver_key] for build in builds.values()}
assert len(driver_hashes)==1 and next(iter(driver_hashes))==hashlib.sha256((HERE/'native_main.rs').read_bytes()).hexdigest()
for name,build in builds.items():assert build['exit']==0 and hashlib.sha256(pathlib.Path(build['retained_exe']).read_bytes()).hexdigest()==build['exe_sha256']
telemetry_sha=hashlib.sha256((HERE/'metrics.rs').read_bytes()).hexdigest()
for name in ['profile-baseline-v3','profile-refactor-v3']:
    assert builds[name]['sources']['crates/engine/src/research_profile.rs']==telemetry_sha
checked={}
for path,expected in builds['pristine-refactor-v2']['sources'].items():
    if path.startswith('crates/') or path in ['Cargo.toml','Cargo.lock']:
        actual=hashlib.sha256((PRIMARY/path).read_bytes()).hexdigest();assert actual==expected,('changed product/source dependency since measured snapshot',path)
        checked[path]=actual
result={'shared_driver_sha256':next(iter(driver_hashes)),'corrected_profile_telemetry_sha256':telemetry_sha,'primary_matches_refactor_measured_sources':checked,
        'binaries':{name:{'path':build['retained_exe'],'sha256':build['exe_sha256']} for name,build in builds.items()}}
(HERE/'verified-measurement-sources.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'shared_driver_verified':True,'matched_primary_files':len(checked),'retained_binaries_verified':4}))
