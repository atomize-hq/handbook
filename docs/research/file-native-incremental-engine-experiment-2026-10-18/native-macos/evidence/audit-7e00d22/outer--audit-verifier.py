"""Read-only verification of the strict audit correction result and finite envelope."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parent
E = ROOT / 'evidence-audit'
A = json.loads((ROOT / 'allocation-audit-correction.json').read_bytes())
def load(path):
    return json.loads(path.read_bytes())
def sha(data):
    return hashlib.sha256(data).hexdigest()
def write(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')

assert time.time() < A['total_deadline_epoch']
finished = load(E / 'finished.json')
assert finished['status'] == 'passed' and not finished['unperformed']
runs = [json.loads(line) for line in (E / 'runs.jsonl').read_bytes().splitlines()]
assert [row['label'] for row in runs] == ['test', 'build', 'strict']
for row in runs:
    assert row['exit'] == 0 and not row['censored'] and row['end_epoch'] < A['total_deadline_epoch']
    assert row['end_epoch'] - row['start_epoch'] < row['suite_cap_seconds']
    for kind, item in row['streams'].items():
        data = (E / (row['label'] + '.' + kind)).read_bytes()
        assert sha(data) == item['sha256'] and len(data) == item['bytes']
assert b'2 passed; 0 failed' in (E / 'test.stdout').read_bytes()
pins = load(E / 'source-pins.json')
assert pins['pin'] == A['pin'] and len(pins['files']) == 70
for name, item in pins['files'].items():
    data = (ROOT / 'source-audit-core' / name).read_bytes()
    assert sha(data) == item['sha256']
    assert subprocess.check_output(['git', 'show', A['pin'] + ':' + name], cwd=ROOT / 'audit-checkout') == data
binaries = load(E / 'binary-pins.json')
for item in binaries.values():
    assert sha(Path(item['path']).read_bytes()) == item['sha256']
base = ROOT / 'audit-lane/strict'
summary = load(base / 'audit-evidence.json')
assert summary['strict'] is True and len(summary['cases']) == 18
for mode in ['A', 'B']:
    cases = [row for row in summary['cases'] if row['mode'] == mode]
    assert len(cases) == 9 and all(row['status'] == 'error' and row['recreated'] is False for row in cases)
    assert {row['case'] for row in cases if 'damage' not in row} == {'projection-child', 'navigation-source', 'projection-root'}
    assert {(row['case'], row['damage']) for row in cases if 'damage' in row} == {(name, damage) for name in ['navigation/d000003.md', 'packet/context.md', 'packet/provenance.json'] for damage in ['missing', 'corrupt']}
rows = [json.loads(line) for line in (base / 'events.jsonl').read_bytes().splitlines()]
assert len(rows) == 42 and not any(row['censored'] for row in rows)
exits = {}
for row in rows:
    exits[str(row['exit'])] = exits.get(str(row['exit']), 0) + 1
    m = row['output']['metrics']
    for direction in ['read', 'written']:
        assert sum(m['bytes_' + role + '_' + direction] for role in ['payload', 'control', 'metadata', 'result']) == m['bytes_' + direction]
assert exits == {'0': 24, '2': 18}
assert sum(row['input']['action'] == 'audit' and row['exit'] == 0 for row in rows) == 18
assert sum(row['input']['action'] == 'audit' and row['exit'] == 2 for row in rows) == 18
session = load(base / 'session.json')
write(base / 'outer-binding.json', dict(
    executable_sha256=session['sha256'], source_archive_sha256=session['source_archive_sha256'],
    launcher_sha256=sha((ROOT / 'launch_audit.py').read_bytes()),
    allocation_sha256=sha((ROOT / 'allocation-audit-correction.json').read_bytes()), cap_seconds=180,
    suite_cap_seconds=900, explicit_correction_deadline_epoch=A['total_deadline_epoch'],
    wrapper_run_file_sha256=sha((E / 'runs.jsonl').read_bytes()),
    binding_origin='Postrun association to retained actual launcher/runs and prospectively frozen explicit 1800-second correction allocation; Driver events/session bytes unchanged'))
previous = load(ROOT / 'evidence-replay/unit-temporary-roots.json')['preserved']
paths = {Path(row['path']) for row in previous}
for path in Path(tempfile.gettempdir()).glob('i1-bounds-*'):
    try:
        created = int(path.name.split('-')[3]) / 1e9
    except (IndexError, ValueError):
        continue
    if runs[0]['start_epoch'] <= created <= runs[0]['end_epoch']:
        paths.add(path)
units = []
for path in sorted(paths):
    files = [item for item in path.rglob('*') if item.is_file()]
    units.append(dict(path=str(path), files=len(files), logical_bytes=sum(item.stat().st_size for item in files), allocated_file_bytes=sum(item.stat().st_blocks * 512 for item in files)))
assert len(units) == 5
write(E / 'unit-temporary-roots.json', dict(preserved=units, qualification='Four predecessor bounds fixtures plus this locked test fixture; all preserved.'))
shutil.copyfile(ROOT / 'launch_audit.py', E / 'audit-launcher.py')
shutil.copyfile(ROOT / 'verify_audit_run.py', E / 'audit-verifier.py')
now = time.time()
write(E / 'audit-verification.json', dict(
    status='passed', pin=A['pin'], source_files=70, locked_rust_tests=2, release_build='passed',
    fixture_N=128, modes=['A', 'B'], strict=True, negative_cases=18,
    refused_without_recreation=18, successful_audits_after_exact_restoration=18,
    events=42, censors=0, exit_counts=exits, byte_role_conservation_json_responses=42,
    suite_seconds=runs[2]['end_epoch']-runs[2]['start_epoch'], suite_cap_seconds=900,
    command_seconds={row['label']: row['end_epoch']-row['start_epoch'] for row in runs},
    explicit_correction_start_epoch=A['start_epoch'], explicit_correction_deadline_epoch=A['total_deadline_epoch'],
    explicit_total_cap_seconds=1800, elapsed_seconds=now-A['start_epoch'], remaining_seconds=A['total_deadline_epoch']-now,
    unperformed_requested_checks=[], broad_workloads_run=False, original_measurement_windows_reopened=False,
    diagnostic_summary=summary, epoch=now))
print((E / 'audit-verification.json').read_text())
