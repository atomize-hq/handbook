import hashlib,json,subprocess,tarfile
from pathlib import Path
HERE=Path(__file__).resolve().parent;WORK=HERE.parents[2];RUNTIME=Path('C:/file-native-packing-20261020');out=HERE/'candidate';out.mkdir(exist_ok=False)
commit=subprocess.check_output(['git','rev-parse','HEAD'],cwd=WORK).decode().strip()
binding=json.loads((RUNTIME/'receipts/candidate1-binding.json').read_bytes());binding['core_commit']=commit
for name,row in binding['files'].items():
    raw=subprocess.check_output(['git','show',commit+':'+name],cwd=WORK);assert hashlib.sha256(raw).hexdigest()==row['sha256']
assert hashlib.sha256(Path(binding['exe']).read_bytes()).hexdigest()==binding['exe_sha256']
(out/'source-binding.json').write_text(json.dumps(binding,indent=2))
selected=[]
for directory in [RUNTIME/'receipts',RUNTIME/'full-capsule',RUNTIME/'strict-supplement',RUNTIME/'watchdog-group-check',RUNTIME/'watchdog-final-check',RUNTIME/'preservation-reopen']:
    for p in directory.iterdir():
        if p.is_file() and p.suffix in ['.json','.jsonl','.stdout','.stderr','.py'] and p.stat().st_size<=4*1024**2:selected.append(p)
selected=sorted(set(selected));assert sum(p.stat().st_size for p in selected)<16*1024**2
index=[]
with tarfile.open(out/'raw-receipts.tar.gz','w:gz',compresslevel=6) as archive:
    for p in selected:
        name=p.relative_to(RUNTIME).as_posix();raw=p.read_bytes();index.append(dict(path=name,bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest()));archive.add(p,arcname=name,recursive=False)
with tarfile.open(out/'raw-receipts.tar.gz') as archive:
    for row,member in zip(index,archive.getmembers()):
        raw=archive.extractfile(member).read();assert member.name==row['path'] and len(raw)==row['bytes'] and hashlib.sha256(raw).hexdigest()==row['sha256']
(out/'raw-receipts-index.json').write_text(json.dumps(dict(archive_sha256=hashlib.sha256((out/'raw-receipts.tar.gz').read_bytes()).hexdigest(),files=index),indent=2))
for name,source in [('capsule-summary.json',RUNTIME/'full-capsule/summary.json'),('strict-summary.json',RUNTIME/'strict-supplement/summary.json'),('preservation-summary.json',RUNTIME/'preservation-reopen/summary.json'),('large-stage-forecast.json',RUNTIME/'receipts/large-packing-stage-forecast.json')]:
    (out/name).write_bytes(source.read_bytes())
result=dict(status='candidate ready for independent parent review and bounded native qualification; long performance and final storage recommendation remain open',source_commit=commit,source_core_files=len(binding['files']),windows_exe_sha256=binding['exe_sha256'],windows_exe=binding['exe'],
checks=dict(rust_tests=5,full_p_capsule_cases=54,full_p_capsule_engine_events=275,independent_strict_supplements=24,exact_post_gc_publications=101006,exact_post_gc_ledger_rows=100000,retired_content_refusals=101),
native_proposal=dict(per_host_total_new_cap_bytes=805306368,per_host_total_new_files=50000,source_and_transport_bound=8388608,dependency_cache_growth_bound=201326592,target_build_bound=201326592,synthetic_cache_fixture_bound=88080384,small_AB_fault_restore_and_legacy_fixture_bound=117440512,evidence_and_git_bound=33554432,prospective_total_bytes=650117120,ordinary_seconds=120,complete_verifier_seconds=900,build_jobs=2,native_aggregate_rss_bytes=3221225472,checker_rss_bytes=2147483648,point_rss_bytes=268435456,functional_stage_seconds=1800,closeout_seconds=300,clamp_to_original_P_cutoff='2026-10-20T07:55:16Z',scope='locked tests/build; full capsule.py A/B N128/N16 and strict_supplement.py; actual old recovery capsule via loose_regression.py; host-correct independent zstd library binding; exact source before/after and shell discovery/extraction; no long history'),
native_commands=['cargo test --locked --offline --workspace --release -j2 --target-dir <owned>', 'cargo build --locked --offline --workspace --release -j2 --target-dir <owned>','FILE_NATIVE_P_EXE=<candidate> FILE_NATIVE_P_ROOT=<owned> python capsule.py --base <fresh-owned>','FILE_NATIVE_P_EXE=<candidate> FILE_NATIVE_P_ROOT=<owned> python strict_supplement.py --capsule-base <previous> --base <fresh-owned>','python loose_regression.py --exe <candidate> --base <fresh-owned>'],
native_dependencies='Exact Cargo.lock; cache fetch separately bounded if offline prerequisites absent. Independent Python decoder requires host libzstd via ctypes discovery; record actual version/path/hash rather than assuming Git Windows DLL. Parent owns admission/source transport/hosts.',
guard_sha256=hashlib.sha256((HERE/'guard.py').read_bytes()).hexdigest(),large_windows_packing_admitted=False,native_admitted=False,no_successor_chat_created=True)
(out/'handoff.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
