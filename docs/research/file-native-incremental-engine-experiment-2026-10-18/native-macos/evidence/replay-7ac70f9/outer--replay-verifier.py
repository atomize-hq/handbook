"""Check reached replay correction evidence without running another engine call."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parent
E = ROOT / 'evidence-replay'
LANE = ROOT / 'replay-lane'
PIN = '7ac70f93cce14ecd3d935852801f034c122000d8'
NS = 'docs/research/file-native-incremental-engine-experiment-2026-10-18'
def load(path):
    return json.loads(path.read_bytes())
def sha(data):
    return hashlib.sha256(data).hexdigest()
def write(path, value):
    path.write_text(json.dumps(value, indent=2) + '\n')

finished = load(E / 'finished.json')
assert finished['status'] == 'passed' and not finished['unperformed']
runs = [json.loads(line) for line in (E / 'runs.jsonl').read_bytes().splitlines()]
assert [r['label'] for r in runs] == ['test', 'build', 'replay', 'checks', 'supplement', 'adversarial', 'namespace']
A = load(ROOT / 'allocation-final.json')
for row in runs:
    assert row['exit'] == 0 and not row['censored']
    assert row['end_epoch'] < A['execution_deadline_epoch']
    for kind, item in row['streams'].items():
        data = (E / (row['label'] + '.' + kind)).read_bytes()
        assert sha(data) == item['sha256'] and len(data) == item['bytes']
assert b'2 passed; 0 failed' in (E / 'test.stdout').read_bytes()
pins = load(E / 'source-pins.json')
assert pins['pin'] == PIN and len(pins['files']) == 60
for name, item in pins['files'].items():
    data = (ROOT / 'source-replay-core' / name).read_bytes()
    assert sha(data) == item['sha256']
    assert subprocess.check_output(['git', 'show', PIN + ':' + name], cwd=ROOT / 'replay-checkout') == data
binaries = load(E / 'binary-pins.json')
for item in binaries.values():
    assert sha(Path(item['path']).read_bytes()) == item['sha256']
replay = load(LANE / 'replay/replay-lifetime.json')
assert {row['mode'] for row in replay} == {'A', 'B'} and len(replay) == 2
for row in replay:
    assert all(value is True for key, value in row.items() if key != 'mode')
checks = load(LANE / 'checks/checks.json')
named = [row for row in checks if 'consumer' in row]
faults = [row for row in checks if 'boundary' in row]
assert len(named) == 4 and {(row['mode'], row['consumer']) for row in named} == {(m, c) for m in ['A', 'B'] for c in ['project', 'research']}
assert all(row['status'] == 'passed' for row in named)
assert len(faults) == 23 and len({row['boundary'] for row in faults}) == 23
for row in faults:
    assert row['original_exit'] == 86
    if row['before_selection']:
        assert row['recovered_outcome'] == 'absent-as-of'
    if row['completed_boundary']:
        assert row['recovered_outcome'] == 'accepted'
supplement = load(LANE / 'supplement/supplement.json')
assert supplement['actual_retired_source_reclaimed'] is True and supplement['review_findings'] == 'regressions passed'
adversarial = load(LANE / 'adversarial/adversarial.json')
assert len(adversarial['passed']) == 9
namespace = load(LANE / 'namespace/namespace-cases.json')
assert namespace['expected_prior_bug'] is False and namespace['selected_sources_before'] == namespace['selected_sources_remaining'] == 16
counts = {}
json_responses = 0
exits = {}
for path in sorted(LANE.rglob('events.jsonl')):
    rows = [json.loads(line) for line in path.read_bytes().splitlines()]
    assert not any(row['censored'] for row in rows)
    counts[path.parent.name] = len(rows)
    session = load(path.parent / 'session.json')
    write(path.parent / 'outer-binding.json', dict(
        executable_sha256=session['sha256'], source_archive_sha256=session['source_archive_sha256'],
        launcher_sha256=sha((ROOT / 'launch_replay.py').read_bytes()),
        allocation_sha256=sha((ROOT / 'allocation-final.json').read_bytes()), cap_seconds=180,
        original_final_execution_deadline_epoch=A['execution_deadline_epoch'],
        wrapper_run_file_sha256=sha((E / 'runs.jsonl').read_bytes()),
        binding_origin='Postrun binding to actual retained launcher/runs and original allocation; Driver events/session bytes unchanged'))
    for row in rows:
        exits[str(row['exit'])] = exits.get(str(row['exit']), 0) + 1
        if row['output'] is None:
            assert row['exit'] == 86
            continue
        m = row['output']['metrics']
        for direction in ['read', 'written']:
            assert sum(m['bytes_' + role + '_' + direction] for role in ['payload', 'control', 'metadata', 'result']) == m['bytes_' + direction]
        json_responses += 1
assert counts == dict(adversarial=42, checks=414, namespace=5, replay=40, supplement=109)
assert exits == {'0': 504, '2': 70, '86': 36} and json_responses == 574
units = []
previous = load(ROOT / 'evidence-corrected/unit-temporary-roots.json')['preserved']
paths = {Path(x['path']) for x in previous}
for p in Path(tempfile.gettempdir()).glob('i1-bounds-*'):
    try:
        created = int(p.name.split('-')[3]) / 1e9
    except (ValueError, IndexError):
        continue
    if runs[0]['start_epoch'] <= created <= runs[0]['end_epoch']:
        paths.add(p)
for p in sorted(paths):
    files = [x for x in p.rglob('*') if x.is_file()]
    units.append(dict(path=str(p), files=len(files), logical_bytes=sum(x.stat().st_size for x in files), allocated_file_bytes=sum(x.stat().st_blocks * 512 for x in files)))
assert len(units) == 4
write(E / 'unit-temporary-roots.json', dict(preserved=units, qualification='Three predecessor fixtures plus one bounds fixture created within this retained Cargo test interval; no cleanup.'))
shutil.copyfile(ROOT / 'launch_replay.py', E / 'replay-launcher.py')
shutil.copyfile(ROOT / 'verify_replay_run.py', E / 'replay-verifier.py')
now = time.time()
write(E / 'replay-verification.json', dict(
    status='passed', pin=PIN, source_files=60, counts=counts, events=sum(counts.values()), censors=0,
    exit_counts=exits, byte_role_conservation_json_responses=json_responses,
    replay_N=512, replay_modes=replay, semantic_configurations=4, publication_boundaries=23,
    supplement=supplement, adversarial_groups=adversarial['passed'], namespace=namespace,
    build_and_locked_tests='passed: two Rust tests', unperformed=[], no_smoke_or_history_rerun=True,
    original_start_epoch=A['start_epoch'], elapsed_original_seconds=now-A['start_epoch'],
    remaining_original_work_seconds=A['execution_deadline_epoch']-now,
    remaining_original_total_seconds=A['total_deadline_epoch']-now, deadline_reset=False, epoch=now))
print((E / 'replay-verification.json').read_text())
