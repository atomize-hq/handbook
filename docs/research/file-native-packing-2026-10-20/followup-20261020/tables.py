"""Follow-up-only B gate derivation; no writes to original reports or tables."""
import hashlib, json, math, statistics
from pathlib import Path

HERE = Path(__file__).resolve().parent
R = Path('C:/file-native-packing-20261020')
CASE = R / 'operational-matrix/n8000-B'
OUT = R / 'followup-20261020'
cells = {}
for name in ['L-old', 'L-new', 'P0', 'PZ']:
    label = name + '-path-v2'
    path = CASE / (label + '-measurement.json')
    job = OUT / 'receipts' / ('B-' + name + '-job.json')
    if not path.exists() or not job.exists():
        cells[name] = None
        continue
    guarded = json.loads(job.read_bytes())
    assert guarded['exit'] == 0 and guarded['censor'] is None and guarded['process_closure_verified']
    row = json.loads(path.read_bytes())
    assert row['mode'] == 'B' and row['documents'] == 8000 and row['history_before'] == 8 and row['history_after'] == 15
    assert len(row['publications']) == 7 and len(row['fresh']) == 120 and row['independent']['semantic']['publications'] == 15
    groups = {}
    for warm in row['warm']['result']['rows']:
        category = warm['category']
        fresh = [x for x in row['fresh'] if x['category'] == category]
        assert len(fresh) == 20 and len(warm['warm_ns']) == 500
        observations = dict(warm_ns=warm['warm_ns'], fresh_engine_ns=[x['engine_wall_ns'] for x in fresh], fresh_child_ns=[round(x['child_seconds']*1e9) for x in fresh], fresh_rss=[x['rss'] for x in fresh if x['rss'] is not None])
        groups[category] = dict(result_digest=warm['result_digest'], missing_fresh_rss=sum(x['rss'] is None for x in fresh), opens=warm['opens'], pack_storage_bytes=warm.get('pack_storage_bytes'), decoded_record_bytes=warm.get('decoded_record_bytes'))
        for kind, values in observations.items():
            values = sorted(values)
            groups[category][kind] = None if not values else dict(samples=len(values), median=statistics.median(values), p95=values[math.ceil(.95*len(values))-1], minimum=values[0], maximum=values[-1])
    assert set(groups) == {'early', 'middle', 'recent', 'missing', 'cancelled', 'lost-reply'}
    cells[name] = dict(measurement_sha256=hashlib.sha256(path.read_bytes()).hexdigest(), median_ns=row['ordinary_median_complete_engine_ns'], publication_cases=[dict(case=x['case'], complete_ns=x['complete_engine_ns'], prepare_ns=x['prepare']['wall_ns'], select_ns=x['select']['wall_ns'], state_digest=x['result_state_digest']) for x in row['publications']], groups=groups, original_cohort_records=row['cohort_records'], independent=row['independent'], new_loose_tail=row['new_loose_tail'], exe_sha256=row['binary_sha256'], job=dict(seconds=guarded['seconds'], closure=guarded['process_closure_verified'], aggregate_sampled_peak_rss=guarded['aggregate_sampled_peak_rss'], missing_samples=guarded['missing_samples']))
    if name in ['P0', 'PZ']:
        inventory = CASE / (label + '-operational-backup/files.jsonl')
        records = [json.loads(line) for line in inventory.read_text().splitlines()]
        paths = {x['path'] for x in records}
        original = {json.loads(line)['key'] for line in (CASE / 'matched-cohort-inventory.jsonl').read_text().splitlines()}
        assert len(original) == row['cohort_records']
        containers = [x for x in records if x['path'].startswith(('packs/', 'pack-catalogs/'))]
        remaining = len(paths & original)
        duplicate_bytes = sum(x['bytes'] for x in records if x['path'] in original)
        original_raw_bytes = sum(json.loads(line)['raw_length'] for line in (CASE / 'matched-cohort-inventory.jsonl').read_text().splitlines())
        physical = len(containers) + remaining
        reduction = 1 - physical / len(original)
        cells[name]['packing'] = dict(pack_index_bytes=sum(x['bytes'] for x in containers if x['path'].startswith('packs/')), pack_index_catalog_files=len(containers), remaining_original_loose_duplicates=remaining, remaining_original_loose_duplicate_bytes=duplicate_bytes, original_cohort_files=len(original), original_cohort_raw_bytes=original_raw_bytes, eligible_pack_index_catalog_and_duplicates_bytes=sum(x['bytes'] for x in containers)+duplicate_bytes, eligible_physical_files_after=physical, file_reduction=reduction, ninety_five_percent_gate=reduction >= .95, other_owner_journal_control_files_additionally_charged=True)
reference = cells['L-old']
for name, row in cells.items():
    if row is None:
        continue
    row['ordinary_over_qualified_L'] = None if reference is None else row['median_ns'] / reference['median_ns']
    row['ordinary_gate'] = None if reference is None else row['median_ns'] <= reference['median_ns'] * 1.2 + 1000000
    if reference:
        assert row['original_cohort_records'] == reference['original_cohort_records']
        assert [x['state_digest'] for x in row['publication_cases']] == [x['state_digest'] for x in reference['publication_cases']]
        assert [x['result_digest'] for x in row['groups'].values()] == [x['result_digest'] for x in reference['groups'].values()]
    comparator = reference if name == 'P0' else cells['P0'] if name == 'PZ' else None
    if comparator:
        row['warm_ratios'] = {k: v['warm_ns']['median'] / comparator['groups'][k]['warm_ns']['median'] for k, v in row['groups'].items()}
        row['warm_2x_gate'] = all(x <= 2 for x in row['warm_ratios'].values())
compression = None
if cells['P0'] and cells['PZ']:
    ratio = cells['PZ']['packing']['pack_index_bytes'] / cells['P0']['packing']['pack_index_bytes']
    compression = dict(PZ_over_P0_bytes=ratio, twenty_percent_gate=ratio <= .8)
result = dict(scope='separate followup only', original_run_unchanged=True, source='same authentic B N8000 H8 baseline; complete actual cohort', ordinary_gate='median prepare+select <= qualified L median*1.2 +1ms', cache_qualification='warm same-process after first lookup; fresh processes have uncontrolled OS/device caches', cells=cells, compression=compression)
(OUT / 'tables.json').write_text(json.dumps(result, indent=2))
print(json.dumps(dict(cells={k: None if v is None else {f: v.get(f) for f in ['median_ns', 'ordinary_gate', 'warm_ratios', 'warm_2x_gate', 'original_cohort_records', 'packing']} for k, v in cells.items()}, compression=compression), indent=2))
