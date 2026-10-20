"""Read-only final cross-cell bindings and fresh ownership census; no engine child."""
import datetime, hashlib, json, subprocess, sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
import guard

assert datetime.datetime.now(datetime.timezone.utc) < datetime.datetime.fromisoformat('2026-10-20T13:13:00+00:00')
R = guard.ROOT
FU = R / 'followup-20261020'
CASE = R / 'operational-matrix/n8000-B'
cfg = json.loads((HERE / 'preparation.json').read_bytes())
for name, digest in cfg['bindings'].items():
    assert hashlib.sha256(Path(name).read_bytes()).hexdigest() == digest, name
jobs = {}
for path in (FU / 'receipts').glob('*-job.json'):
    if path.name.startswith('artifact-'):
        continue
    row = json.loads(path.read_bytes())
    assert row['exit'] == 0 and row['censor'] is None and row['process_closure_verified'], path.name
    jobs[path.name] = {k: row[k] for k in ['started_utc', 'seconds', 'cap_seconds', 'effective_seconds', 'exit', 'censor', 'process_closure_verified', 'aggregate_sampled_peak_rss', 'missing_samples']}
measurements = {name: json.loads((CASE / (name + '-path-v2-measurement.json')).read_bytes()) for name in ['L-old', 'L-new', 'P0', 'PZ']}
reference = measurements['L-old']
for name, row in measurements.items():
    assert row['cohort_records'] == 28 and row['history_before'] == 8 and row['history_after'] == 15
    assert len(row['publications']) == 7 and len(row['fresh']) == 120
    assert len(row['warm']['result']['rows']) == 6 and all(len(x['warm_ns']) == 500 for x in row['warm']['result']['rows'])
    assert [x['result_state_digest'] for x in row['publications']] == [x['result_state_digest'] for x in reference['publications']]
    assert [x['result_digest'] for x in row['warm']['result']['rows']] == [x['result_digest'] for x in reference['warm']['result']['rows']]
    assert (CASE / (name + '-path-v2-inventory.jsonl')).read_bytes() == (CASE / 'matched-cohort-inventory.jsonl').read_bytes()
    assert row['independent']['semantic']['publications'] == 15 and row['independent']['semantic']['retired_publications'] == 12 and row['independent']['navigation']['documents'] == 8000
for name in ['P0', 'PZ']:
    assert measurements[name]['independent']['raw_records'] == 28 and measurements[name]['independent']['raw_bytes'] == 1804894
assert measurements['P0']['independent']['inventory_digest'] == measurements['PZ']['independent']['inventory_digest']
assert json.loads((FU / 'P0-PZ-grouping.json').read_bytes())['same_five_pack_grouping']
actual = guard.current()
assert actual['total_new_bytes'] + 32*1024**2 <= 2684354560 and actual['total_new_files'] + 1000 <= 560000 and actual['free_bytes'] >= 140*1024**3
source_status = subprocess.check_output(['git', 'diff', '--name-only', 'f59ba64d259abbb9ba140a3a2ef0663a35ab1725'], cwd=guard.WORK, text=True).splitlines()
assert all('/followup-20261020/' in name for name in source_status)
result = dict(status='ALL_FOUR_B_CELLS_COMPLETE_SEPARATE_FOLLOWUP', core='d70f3864f2888d46d9112213b4a03f8d82745a8e', all_original_bindings_unchanged=True, original_run_report_censors_and_archive_unchanged=True, identical_original_cohort_in_all_four_cells=True, complete_samples_per_cell=dict(warm_each_of_six=500, fresh=120, ordinary_publications=7), equal_seven_state_digest_sequences=True, equal_six_original_lookup_result_digests=True, equal_all_28_P0_PZ_decoded_raw_tuples=True, same_five_pack_grouping=True, history_before=8, history_after=15, all_independent_Nav8000=True, all_owned_Windows_functional_jobs_closed=True, jobs=jobs, final_actual=actual, return_and_positive_Git_reservation=32*1024**2, PZ_source_retained=True, recommendation='ADJUST; no production adoption', GitNexus='UNKNOWN: isolated worktree unregistered; frozen bindings and manual diff define scope')
(FU / 'final-verification.json').write_text(json.dumps(result, indent=2))
print(json.dumps({k: v for k, v in result.items() if k != 'jobs'}, indent=2))
