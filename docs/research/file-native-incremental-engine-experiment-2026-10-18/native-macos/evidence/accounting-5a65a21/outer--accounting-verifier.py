"""Derive actual accounting-case counts and validate reached source-bound proof."""
import collections
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parent
E = ROOT / 'evidence-accounting'
A = json.loads((ROOT / 'allocation-accounting-correction.json').read_bytes())
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
assert [row['label'] for row in runs] == ['build', 'tests', 'paired-intents', 'supplement', 'replay-lifetime']
assert finished['absolute_execution_deadline_epoch'] == A['start_epoch'] + 1500
case_start = finished['combined_case_start_epoch']
case_end = runs[-1]['end_epoch']
assert case_end - case_start < 900
for row in runs:
    assert row['exit'] == 0 and not row['censored'] and row['end_epoch'] < A['execution_deadline_epoch']
    assert row['end_epoch'] - row['start_epoch'] < row['command_cap_seconds']
    if row['label'] not in ['build', 'tests']:
        assert row['combined_case_deadline_epoch'] == min(case_start + 900, A['execution_deadline_epoch'])
        assert row['effective_deadline_epoch'] <= row['combined_case_deadline_epoch']
    for kind, item in row['streams'].items():
        data = (E / (row['label'] + '.' + kind)).read_bytes()
        assert sha(data) == item['sha256'] and len(data) == item['bytes']
assert b'2 passed; 0 failed' in (E / 'tests.stdout').read_bytes()
pins = load(E / 'source-pins.json')
assert pins['pin'] == A['pin'] and len(pins['files']) == 85
for name, item in pins['files'].items():
    data = (ROOT / 'source-accounting-capsule' / name).read_bytes()
    assert sha(data) == item['sha256']
    assert subprocess.check_output(['git', 'show', A['pin'] + ':' + name], cwd=ROOT / 'accounting-checkout') == data
binaries = load(E / 'binary-pins.json')
for item in binaries.values():
    assert sha(Path(item['path']).read_bytes()) == item['sha256']
lane = ROOT / 'accounting-lane'
paired = load(lane / 'paired-intents/paired-intents.json')
assert paired['expect_poisoned'] is False and len(paired['rows']) == 2
assert {row['mode'] for row in paired['rows']} == {'A', 'B'}
for row in paired['rows']:
    for key in ['unrelated_preparation_and_selection_succeeded', 'losing_intent_refused', 'loser_fenced_and_collected', 'original_acceptance_exact']:
        assert row[key] is True
    assert not row['reservations']
    assert row['collection']['status'] == 'collected' and row['collection']['accepted_evidence_preserved'] is True
supplement = load(lane / 'supplement/supplement.json')
assert supplement['review_findings'] == 'regressions passed' and supplement['actual_retired_source_reclaimed'] is True
assert supplement['reader_overlap'] is True and supplement['quota_slots'] == 8 and supplement['fallback'] is True
replay = load(lane / 'replay-lifetime/replay-lifetime.json')
assert len(replay) == 2 and {row['mode'] for row in replay} == {'A', 'B'}
for row in replay:
    assert all(value is True for key, value in row.items() if key != 'mode')
counts = {}
exits = collections.Counter()
json_responses = 0
for path in sorted(lane.rglob('events.jsonl')):
    rows = [json.loads(line) for line in path.read_bytes().splitlines()]
    assert not any(row['censored'] for row in rows)
    counts[path.parent.name] = len(rows)
    session = load(path.parent / 'session.json')
    write(path.parent / 'outer-binding.json', dict(
        executable_sha256=session['sha256'], source_archive_sha256=session['source_archive_sha256'],
        launcher_sha256=sha((ROOT / 'launch_accounting.py').read_bytes()),
        allocation_sha256=sha((ROOT / 'allocation-accounting-correction.json').read_bytes()),
        cap_seconds=180, absolute_execution_deadline_epoch=A['execution_deadline_epoch'],
        combined_case_deadline_epoch=finished['combined_case_deadline_epoch'],
        explicit_total_deadline_epoch=A['total_deadline_epoch'],
        wrapper_runs_sha256=sha((E / 'runs.jsonl').read_bytes()),
        binding_origin='Postrun association to actual retained native launcher and prospectively frozen absolute cutoff/allocation; original Driver event/session bytes unchanged'))
    for row in rows:
        exits[str(row['exit'])] += 1
        if row['output'] is None:
            assert row['exit'] == 86
            continue
        m = row['output']['metrics']
        for direction in ['read', 'written']:
            assert sum(m['bytes_' + role + '_' + direction] for role in ['payload', 'control', 'metadata', 'result']) == m['bytes_' + direction]
        json_responses += 1
assert set(counts) == {'paired-intents', 'supplement', 'replay-lifetime'}
paired_raw = [json.loads(line) for line in (lane / 'paired-intents/events.jsonl').read_bytes().splitlines()]
refusals = [row for row in paired_raw if row['exit'] == 2]
assert len(refusals) == 4 and all('intent-mismatch' in row['output']['error'] for row in refusals)
paths = {Path(row['path']) for row in load(ROOT / 'evidence-audit/unit-temporary-roots.json')['preserved']}
for path in Path(tempfile.gettempdir()).glob('i1-bounds-*'):
    try:
        created = int(path.name.split('-')[3]) / 1e9
    except (IndexError, ValueError):
        continue
    if runs[1]['start_epoch'] <= created <= runs[1]['end_epoch']:
        paths.add(path)
units = []
for path in sorted(paths):
    files = [item for item in path.rglob('*') if item.is_file()]
    units.append(dict(path=str(path), files=len(files), logical_bytes=sum(item.stat().st_size for item in files), allocated_file_bytes=sum(item.stat().st_blocks * 512 for item in files)))
assert len(units) == 6
write(E / 'unit-temporary-roots.json', dict(preserved=units, qualification='Five predecessor fixtures plus this locked-test fixture; no cleanup.'))
shutil.copyfile(ROOT / 'launch_accounting.py', E / 'accounting-launcher.py')
shutil.copyfile(ROOT / 'verify_accounting_run.py', E / 'accounting-verifier.py')
now = time.time()
write(E / 'accounting-verification.json', dict(
    status='passed', pin=A['pin'], source_files=85, locked_rust_tests=2, release_build='passed',
    counts=counts, events=sum(counts.values()), censors=0, exit_counts=dict(exits),
    counts_derived_from_actual_driver_records=True, byte_role_conservation_json_responses=json_responses,
    paired_intents=paired, public_different_intent_refusals=len(refusals), supplement=supplement,
    replay_N=512, replay=replay, command_seconds={row['label']: row['end_epoch']-row['start_epoch'] for row in runs},
    combined_case_seconds=case_end-case_start, combined_case_cap_seconds=900,
    absolute_execution_deadline_epoch=A['execution_deadline_epoch'], reporting_reserve_seconds=300,
    explicit_start_epoch=A['start_epoch'], explicit_total_deadline_epoch=A['total_deadline_epoch'], explicit_total_cap_seconds=1800,
    elapsed_seconds=now-A['start_epoch'], execution_remaining_seconds=A['execution_deadline_epoch']-now,
    total_remaining_seconds=A['total_deadline_epoch']-now, unperformed_requested_checks=[],
    original_windows_reset=False, broad_workloads_run=False, successor_dispatch=False, epoch=now))
print((E / 'accounting-verification.json').read_text())
