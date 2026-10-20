"""Read-only ownership and immutable-input binding for the separately timed follow-up."""
import datetime, hashlib, json, subprocess, sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
OLD = HERE.parent
sys.path.insert(0, str(OLD))
import guard

R = guard.ROOT
OUT = R / 'followup-20261020'
OUT.mkdir(exist_ok=False)
(OUT / 'receipts').mkdir()
head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=guard.WORK).decode().strip()
status = subprocess.check_output(['git', 'status', '--porcelain', '--untracked-files=no'], cwd=guard.WORK).decode()
assert head == 'f59ba64d259abbb9ba140a3a2ef0663a35ab1725' and not status
core = json.loads((R / 'fix2/build-binding.json').read_bytes())['files']
bindings = {}
for name, row in core.items():
    raw = (guard.WORK / name).read_bytes()
    digest = hashlib.sha256(raw).hexdigest()
    frozen = subprocess.check_output(['git', 'show', 'd70f3864f2888d46d9112213b4a03f8d82745a8e:' + name], cwd=guard.WORK)
    assert digest == row['sha256'] and raw == frozen
    bindings[str(guard.WORK / name)] = digest
assert len(core) == 18
expected = {
    R / 'bin/candidate2.exe': '568f631acf528c7de911bb88ec85d07ead9d7b4435ebe970c61053f06e31a93b',
    Path('C:/file-native-navigation-20261019/bin/final.exe'): 'd4b7c495abfc029e8fabd6fcfc75501c5de04ce72dd38847a69bc8824022303f',
    R / 'build/release/qualified_lookup.exe': json.loads((R / 'long-matrix/L-old-measurement.json').read_bytes())['warm_binary_sha256'],
    OLD / 'independent.py': '94987dbe1c992ef2491bb9819ad90185d2f29aef704b1bfaaa8f7e052a449e66',
    OLD / 'guard.py': 'a83169e99aad14bf776e1aa032938c00a5e13c01b90c46ad909297f6b9bb1e68',
    OLD / 'reclaim_operational_image_fast_slots.ps1': '7d72b0e5af31f86f4903450a9bd400f6d974b84ec2075d1813074374c4c0f5be',
    OLD / 'final-evidence/raw-evidence.tar.gz': '5176e47d4a7569ed7ac57b29fe0972e5d2e1d37c713f44c4d38c95311ded815d',
}
capsule = R / 'operational-matrix/n8000-B/baseline-operational-backup'
expected.update({capsule / k: v for k, v in {
    'operational.bin.gz': 'c5a8f54d0e1c4de85b795d9b4da2a4b8deedfa9c1a0b0e79edd312744aa7af14',
    'files.jsonl': '642ea31054c7d0d224ff59b93882a043b7473ddccf1a4ca52ef70c00d98faa1d',
    'manifest.json': 'acfc9cf9f88f433367c20656b483de48b12d0a0b1456d8b06378c5919740ea9e',
}.items()})
for path, digest in expected.items():
    assert hashlib.sha256(path.read_bytes()).hexdigest() == digest, str(path)
    bindings[str(path)] = digest
for name in ['driver.py', 'sequenced_matrix_stage.py', 'operational_matrix.py', 'packed_backup_stage.py']:
    bindings[str(OLD / name)] = hashlib.sha256((OLD / name).read_bytes()).hexdigest()
for name in ['final-evidence/raw-index.json', 'final-evidence/external-retained-index.json', 'report.md']:
    path = OLD / name
    bindings[str(path)] = hashlib.sha256(path.read_bytes()).hexdigest()
ps = subprocess.run(['powershell', '-NoProfile', '-Command', 'Get-CimInstance Win32_Process | Select-Object ProcessId,ParentProcessId,Name,ExecutablePath,CommandLine | ConvertTo-Json -Depth 3 -Compress'], capture_output=True, timeout=30)
assert ps.returncode == 0
processes = json.loads(ps.stdout)
owned = [p for p in processes if any(s in (p.get('CommandLine') or '').lower() for s in ['file-native-packing-20261020', 'file-native-navigation-20261019/bin/final.exe']) or (p.get('Name') or '').lower() in ['candidate2.exe', 'qualified_lookup.exe', 'file-native-incremental.exe']]
(OUT / 'receipts/process-census.json').write_text(json.dumps(dict(observed_process_count=len(processes), matching_processes=owned, missing_commandlines=sum(p.get('CommandLine') is None for p in processes), absence_claim='matching visible processes only; no inference for inaccessible command lines'), indent=2))
actual = guard.current()
baseline = guard.census(R / 'operational-matrix/n8000-B/baseline/disposable/store')
config = dict(scope='new separately authorized follow-up; original run unchanged', start='2026-10-20T11:43:00Z', functional_cutoff='2026-10-20T12:53:00Z', artifact_closeout='2026-10-20T13:13:00Z', source_head=head, core='d70f3864f2888d46d9112213b4a03f8d82745a8e', windows_bytes_cap=2684354560, windows_files_cap=560000, free_bytes_min=140*1024**3, new_store_cap=600000000, archive_cap=67108864, returns_cap=33554432, whole_cell_seconds=900, bindings=bindings, actual=actual, retained_baseline=baseline, baseline_regular_reclaim_planned_bytes=320288418, baseline_regular_reclaim_planned_files=24515, admission=False)
assert actual['total_new_bytes'] + config['returns_cap'] <= config['windows_bytes_cap']
assert actual['total_new_files'] < config['windows_files_cap'] and actual['free_bytes'] >= config['free_bytes_min']
config['prospective_before_baseline_reclaim'] = actual['total_new_bytes'] + 600000000 + 67108864 + 33554432
config['prospective_after_planned_reclaim_NOT_credit'] = config['prospective_before_baseline_reclaim'] - 320288418
(HERE / 'preparation.json').write_text(json.dumps(config, indent=2))
(OUT / 'receipts/preparation.json').write_text(json.dumps(config, indent=2))
print(json.dumps({k: v for k, v in config.items() if k != 'bindings'}, indent=2))
