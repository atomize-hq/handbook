"""Read-only parent comparison from four complete measurements; no engine execution."""
from pathlib import Path
import json, hashlib, statistics, collections

p = Path('C:/file-native-packing-20261020/operational-matrix/n8000-B')
f = Path('C:/file-native-packing-20261020/followup-20261020')
out = Path(__file__).resolve().parent
data = {name: json.loads((p / (name + '-path-v2-measurement.json')).read_bytes()) for name in ['L-old', 'L-new', 'P0', 'PZ']}
original = (p / 'matched-cohort-inventory.jsonl').read_bytes()
old = data['L-old']
reference_cases = [(x['case'], x['result_state_digest'], x['replay_objects'], x['replay_files']) for x in old['publications']]
reference_lookups = [(x['category'], x['id'], x['result_digest']) for x in old['warm']['result']['rows']]
threshold = old['ordinary_median_complete_engine_ns'] * 1.2 + 1000000
results = {}
for name, d in data.items():
    assert d['documents'] == 8000 and d['history_before'] == 8 and d['history_after'] == 15 and d['cohort_records'] == 28
    assert (p / (name + '-path-v2-inventory.jsonl')).read_bytes() == original
    assert [(x['case'], x['result_state_digest'], x['replay_objects'], x['replay_files']) for x in d['publications']] == reference_cases
    assert [(x['category'], x['id'], x['result_digest']) for x in d['warm']['result']['rows']] == reference_lookups
    assert all(len(x['warm_ns']) == 500 for x in d['warm']['result']['rows'])
    assert len(d['fresh']) == 120 and collections.Counter(x['category'] for x in d['fresh']) == {x[0]: 20 for x in reference_lookups}
    assert all(x['rss'] is not None and x['rss'] <= 268435456 for x in d['fresh'])
    assert all(x['prepare']['status'] == 'ok' and x['select']['status'] == 'ok' for x in d['publications'])
    assert d['ordinary_median_complete_engine_ns'] == statistics.median(x['complete_engine_ns'] for x in d['publications'])
    independent = d['independent']
    assert independent['semantic']['publications'] == 15 and independent['navigation']['documents'] == 8000
    assert independent['navigation']['discovery_relationships_and_extraction'] and independent['navigation']['readable_declared_current']
    job = json.loads((f / 'receipts' / ('B-' + name + '-job.json')).read_bytes())
    assert job['exit'] == 0 and job['censor'] is None and job['process_closure_verified']
    results[name] = {'median_ns': d['ordinary_median_complete_engine_ns'], 'ordinary_threshold_ns': threshold, 'ordinary_gate': d['ordinary_median_complete_engine_ns'] <= threshold, 'warm_ns': {x['category']: statistics.median(x['warm_ns']) for x in d['warm']['result']['rows']}, 'job_seconds': job['seconds'], 'missing_RSS_samples': job['missing_samples'], 'measurement_sha256': hashlib.sha256((p / (name + '-path-v2-measurement.json')).read_bytes()).hexdigest()}
for packed, loose in [('P0', 'L-old'), ('PZ', 'P0')]:
    ratios = {key: value / results[loose]['warm_ns'][key] for key, value in results[packed]['warm_ns'].items()}
    results[packed]['warm_ratios'] = ratios
    results[packed]['warm_2x_gate'] = all(x <= 2 for x in ratios.values())
tables = json.loads((f / 'tables.json').read_bytes())
for name in ['P0', 'PZ']:
    packing = tables['cells'][name]['packing']
    assert packing['original_cohort_files'] == 28 and packing['remaining_original_loose_duplicates'] == 8 and packing['remaining_original_loose_duplicate_bytes'] == 10268
    assert packing['eligible_physical_files_after'] == packing['pack_index_catalog_files'] + 8
    results[name]['packing'] = packing
    results[name]['independent_raw_tuple_digest'] = data[name]['independent']['inventory_digest']
assert data['P0']['independent']['raw_records'] == data['PZ']['independent']['raw_records'] == 28
assert data['P0']['independent']['raw_bytes'] == data['PZ']['independent']['raw_bytes'] == 1804894
assert data['P0']['independent']['inventory_digest'] == data['PZ']['independent']['inventory_digest']
compression = 1 - results['PZ']['packing']['pack_index_bytes'] / results['P0']['packing']['pack_index_bytes']
report = {'status': 'PARENT_FOUR_B_COMPARISONS_REVIEW_PASS_WITH_FROZEN_GATE_RESULTS', 'cohort_inventory_sha256': hashlib.sha256(original).hexdigest(), 'all_four_complete_inventory_case_replay_and_lookup_digests_match': True, 'ordinary_cases_each': 7, 'warm_samples_each_category': 500, 'fresh_processes_each': 120, 'cells': results, 'PZ_pack_index_reduction_over_P0': compression, 'compression_20percent_gate': compression >= .20, 'cache_note': 'Same-process warm after first lookup; fresh-process OS/device caches uncontrolled.', 'no_threshold_changes': True}
(out / 'B-four-parent-review.json').write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
print(json.dumps(report))
