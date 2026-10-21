"""Private long qualification orchestration. Import has no runtime effects.

LaneHost owns native admission, process containment, deadlines and raw receipts.
This module owns ordering, exact donor checks, sample completeness, byte closure,
independent oracle comparisons and destination-only accepted successor checks.
The example config is intentionally unable to run.
"""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import statistics
import time

ACTUAL_H = 101007
EXPECTED_HISTORY = 100000
PERMANENT = 'f398ab10c2020b988d46530c6e93e0c6c6e0188d6e66e9b8e4e07e96c107f63e'
INTENT_PLAN_SHA = '3f91bbc7185c515c47fd20bc416b1f107797c39bfdbe26c1233e87d3c1266886'
SELECTED = dict(publication='publication-26408-1792467841305851000-19',
                digest='6d11b0a4e06e4c85ae391516bc99c840f4724bb547d018268bd9e9a2c7ea81cf')
REQUESTS = [('early', 'history-000001'), ('middle', 'history-050000'),
            ('recent', 'reclamation-restore-prior-bytes'), ('missing', 'P-benchmark-missing'),
            ('cancelled', 'P-benchmark-cancelled'), ('lost-reply', 'deep-lost-reply')]
SUCCESSOR = 'online-long-stream-successor-20261020'
MiB = 1024**2


def need(ok, message):
    if not ok:
        raise RuntimeError(message)


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


def encode(value):
    return (json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=True,
                       allow_nan=False) + '\n').encode()


def hex64(value):
    return type(value) is str and re.fullmatch('[0-9a-f]{64}', value) is not None


def integer(value, lower, upper, label):
    need(type(value) is int and lower <= value <= upper, label)
    return value


def bounded_json(path, maximum):
    with Path(path).open('rb') as f:
        raw = f.read(maximum + 1)
    need(len(raw) <= maximum, 'bounded-json-file')
    def pairs(items):
        result = {}
        for k, v in items:
            need(k not in result, 'duplicate-json-key')
            result[k] = v
        return result
    def invalid(_):
        raise ValueError('nonfinite-json')
    return json.loads(raw, object_pairs_hook=pairs, parse_constant=invalid), raw


def reference(row, label):
    need(type(row) is dict and type(row.get('path')) is str and
         Path(row['path']).is_absolute() and hex64(row.get('sha256')) and
         type(row.get('bytes')) is int and row['bytes'] >= 0, 'raw-reference:' + label)
    return row


def check_ref(row, maximum, label):
    reference(row, label)
    need(row['bytes'] <= maximum, 'raw-size:' + label)
    value, raw = bounded_json(row['path'], maximum)
    need(len(raw) == row['bytes'] and digest(raw) == row['sha256'], 'raw-bytes:' + label)
    return value


def poll(host):
    receipt = host.poll()
    need(type(receipt) is dict and receipt.get('live') is True and
         hex64(receipt.get('receipt_sha256')), 'positive-live-poll')
    return receipt


def checkpoint(host, c, label):
    poll(host)
    need(label in c['phase_reserves'], 'missing-phase-forecast:' + label)
    result = host.checkpoint('long:' + label, c['phase_reserves'][label])
    need(type(result) is dict and result.get('go') is True and
         result.get('no_other_semantic_writer') is True and
         hex64(result.get('receipt_sha256')) and
         result.get('forecast_sha256') == c['_forecast_sha256'], 'fresh-phase-admission:' + label)
    return result


def record(host, c, name, value, maximum=None, metadata=False):
    raw = encode(value)
    limit = maximum or c['limits']['record_bytes']
    need(len(raw) <= limit, 'prewrite-evidence-limit')
    base = Path(c['roots']['metadata'])/'receipts' if metadata else Path(c['roots']['evidence'])
    category = c['categories']['destination_metadata'] if metadata else c['categories']['source_evidence']
    path = str(base / name)
    row = host.record(path, raw, category, limit)
    reference(row, name)
    need(row['path'] == path and row['bytes'] == len(raw) and
         row['sha256'] == digest(raw), 'exclusive-evidence-raw-binding')
    return row


def binding(row, label):
    need(type(row) is dict and row.get('reviewed') is True, 'pending-binding:' + label)
    reference(dict(path=row.get('path'), sha256=row.get('sha256'), bytes=row.get('bytes')), label)
    path = Path(row['path'])
    for p in [path, *path.parents]:
        if os.path.lexists(p):
            s = p.lstat()
            need(not p.is_symlink() and not getattr(s, 'st_file_attributes', 0) & 1024,
                 'bound-source-link-or-reparse')
    need(path.stat().st_size == row['bytes'], 'bound-source-size')
    h = hashlib.sha256()
    with path.open('rb') as f:
        for block in iter(lambda: f.read(65536), b''):
            h.update(block)
    need(h.hexdigest() == row['sha256'], 'bound-source-sha:' + label)
    return row


def load_adapter(c):
    row = binding(c['bindings']['legacy_adapter'], 'legacy-adapter')
    spec = importlib.util.spec_from_file_location('legacy_archive_restore', row['path'])
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def validate_config(c):
    """Pure validation: pending release/budget/source cells refuse before host use."""
    need(type(c) is dict and c.get('schema') == 'private-long-lane-v1' and
         c.get('go') is True and c.get('runtime_go') is True, 'source-only-no-runtime-GO')
    need(type(c.get('final_pin')) is str and re.fullmatch('[0-9a-f]{40}', c['final_pin']),
         'pending-final-pin')
    need(c.get('qualification') == 'research-only-process-crash', 'qualification-scope')
    required_bindings = {'engine', 'python', 'oracle', 'oracle_cli', 'stream', 'stream_guard',
                         'legacy_adapter', 'intent_plan', 'request_provenance',
                         'lookup_expectations', 'startup_gate', 'long_source', 'cohort_inventory', 'profile_calculator'}
    need(set(c['bindings']) == required_bindings, 'complete-transitive-source-set')
    for name, row in c['bindings'].items():
        need(type(row) is dict and row.get('reviewed') is True and hex64(row.get('sha256')) and
             type(row.get('path')) is str and Path(row['path']).is_absolute() and
             type(row.get('bytes')) is int and row['bytes'] >= 0, 'pending-source:' + name)
    need(hex64(c.get('cohort_digest')), 'pending-protected-raw-cohort-digest')
    need(c['bindings']['intent_plan']['sha256'] == INTENT_PLAN_SHA, 'exact-intent-plan')
    roots = c['roots']
    need(set(roots) == {'archive', 'source', 'destination', 'metadata', 'evidence'}, 'root-set')
    paths = [Path(roots[k]) for k in ('source', 'destination', 'metadata', 'evidence')]
    need(all(p.is_absolute() for p in paths) and len(set(paths)) == 4, 'absolute-distinct-roots')
    need(all(not a.is_relative_to(b) for a in paths for b in paths if a != b), 'independent-owned-roots')
    need(roots['source'].replace('\\', '/') ==
         'C:/file-native-online-matrix-20261020/long-source/disposable/store' and
         roots['destination'].replace('\\', '/') ==
         'C:/file-native-online-matrix-20261020/long-restored/disposable/store' and
         roots['metadata'].replace('\\', '/') ==
         'C:/file-native-online-matrix-20261020/long-capture', 'admitted-long-root-identities')
    need(roots['archive'].replace('\\', '/') ==
         'C:/file-native-packing-20261020/packed-operational-backup', 'protected-archive-identity')
    limits = c['limits']
    expected = dict(source_construction_bytes=959805444, source_construction_files=7839,
                    destination_all_bytes=676345051, destination_all_files=8000,
                    metadata_bytes=8*MiB, metadata_files=40, engine_reserve_bytes=320*MiB,
                    successor_bytes=16*MiB, source_final_files=7908)
    for key, value in expected.items():
        need(limits.get(key) == value, 'fixed-ledger-limit:' + key)
    integer(limits['record_bytes'], 4096, 128*1024, 'record-bound')
    integer(limits['evidence_bytes'], 1, 52*MiB, 'assigned-evidence-bytes')
    integer(limits['evidence_files'], 1, 4096, 'assigned-evidence-files')
    integer(limits['cleanup_batches'], 1, 100000, 'explicit-cleanup-batch-bound')
    integer(limits['sample_bytes'], 4096, 4*MiB, 'sample-artifact-bound')
    integer(limits['cleanup_receipt_bytes'], 4096, 8*MiB, 'cleanup-artifact-bound')
    integer(limits['result_bytes'], 4096, 128*1024, 'result-artifact-bound')
    need(c['evidence_admission']['assigned'] is True and
         hex64(c['evidence_admission']['receipt_sha256']) and
         c['evidence_admission']['reviewed_source_forecast'] is True,
         'F12-is-not-automatically-32MiB')
    need(c['archive']['selected'] == SELECTED and
         c['archive']['compressed_bytes'] == 313275597, 'exact-archive-plan')
    need(c['cli']['api'] == 'reviewed-v5-R1-cli' and
         c['cli']['startup_gate'] == 'owned-before-any-corpus-read-or-write' and
         type(c['cli']['oracle_argv']) is list and c['cli']['oracle_argv'] and
         all(type(x) is str for x in c['cli']['oracle_argv']), 'pending-oracle-CLI-gate')
    needed = {'source-restore', 'legacy-oracle', 'legacy-samples', 'convert', 'converted-oracle',
              'paged-samples', 'cleanup', 'cleaned-oracle', 'stream-restore',
              'restored-oracle', 'successor', 'successor-oracle', 'closure'}
    need(set(c['phase_reserves']) == needed and all(type(x) is dict and x
         for x in c['phase_reserves'].values()), 'all-phase-budgets-required')
    need(c['claims']['F14_expiry'] == 'after-reaped-convert-full-oracle-census' and
         c['claims']['F11_expiry'] == 'after-destination-successor-full-oracle',
         'no-forecast-shrink-before-complete-proof')
    return c


def invoke(host, c, label, root, action, fields, *, timeout=120):
    """Single JSON engine operation; LaneHost withholds stdin until containment."""
    poll(host)
    engine = c['bindings']['engine']
    release = c.get('_legacy_release' if Path(root) == Path(c['roots']['source']) else '_dest_release', {})
    need(release.get('engine_open_allowed') is True and release.get('root') == str(root),
         'quarantine-full-independent-oracle-before-any-engine')
    key = ('setup' if action == 'lifecycle' else fields['operation']
           if action == 'packing' else action)
    profile = c['_forecast']['profiles'][key]
    payload = dict(root=str(root), action=action)
    payload.update(dict(request=fields) if action == 'publish' else fields)
    need(len(encode(payload)) <= profile['input'], 'typed-input-bound')
    spec = dict(kind='engine-json', api='lane-invoke-v1', label=label,
                executable=engine, final_pin=c['final_pin'], root=str(root),
                action=action, fields=payload, quarantine_release=release, profile=profile, role='reader' if action in
                ('resolve', 'lookup-bench') else 'long-engine', timeout_seconds=timeout,
                stdin_release='after-owned-containment-and-budget-gate',
                max_input_bytes=profile['input'], max_stdout_bytes=profile['stdout'],
                max_stderr_bytes=profile['stderr'], max_receipt_bytes=profile['receipt'],
                allowed_exit_codes=[0], growth=c['phase_reserves'][c['_phase']])
    out = host.invoke(spec)
    need(type(out) is dict and out.get('exit_code') == 0 and
         out.get('source_pin') == c['final_pin'] and out.get('binary_sha256') == engine['sha256']
         and out.get('contained_before_input') is True and out.get('children_empty') is True
         and hex64(out.get('budget_receipt_sha256')) and hex64(out.get('closure_receipt_sha256')),
         'engine-native-source-budget-closure')
    raw = check_ref(out['raw'], profile['stdout'], label)
    need(set(raw) == {'status','result','error','metrics','wall_ns','peak_memory_bytes'} and
         raw['status'] == 'ok' and raw['error'] is None and
         out.get('engine_ns') == raw['wall_ns'], 'authentic-9ec7-engine-envelope')
    need(type(out.get('engine_ns')) is int and out['engine_ns'] >= 0 and
         type(out.get('spawn_total_ns')) is int and out['spawn_total_ns'] >= out['engine_ns'],
         'separate-engine-and-spawn-clocks')
    return dict(result=raw['result'], metrics=raw.get('metrics'), raw=out['raw'],
                engine_ns=out['engine_ns'], spawn_total_ns=out['spawn_total_ns'],
                observed_peak_rss=out.get('observed_peak_rss'),
                budget_receipt_sha256=out['budget_receipt_sha256'],
                closure_receipt_sha256=out['closure_receipt_sha256'])


def phase(host, c, name):
    c['_phase'] = name
    return checkpoint(host, c, name)


def selected(adapter, c, root):
    core = adapter.load_stream(c)
    with core.plain_open(Path(root) / 'SELECTED.json') as f:
        return adapter.strict(f.read(65537), 65536)


def prepare_known(host, c):
    """Persist only the independently prepared nonhistory map; recipe stays in oracle."""
    row = c['bindings']['intent_plan']
    plan = check_ref(row, MiB, 'intent-plan')
    need(plan['target_actual_H'] == ACTUAL_H and
         plan['history_recipe_publications'] == EXPECTED_HISTORY and
         type(plan['known_nonhistory_requests']) is dict and
         len(plan['known_nonhistory_requests']) == 1007, 'independent-known-inputs')
    return record(host, c, 'known-nonhistory.json', plan['known_nonhistory_requests'], 96*1024, True)


def oracle_summary(value, expected_h, baseline=None):
    """Normalize actual accepted v5-R1 nested output; no synthetic oracle PASS fields."""
    semantic, intents = value['semantic'], value['request_intents']
    need(value['raw_records'] == 504833 and value['raw_bytes'] == 579015428 and
         hex64(value['inventory_digest']) and
         semantic['publications'] == expected_h and semantic['ledger_rows'] == 100000 and
         semantic['retired_publications'] == 101 and
         intents['publications'] == intents['actual_H'] == expected_h and
         intents['authentic_history_recipe_intents'] == EXPECTED_HISTORY and
         intents['engine_code_imported'] is False, 'complete-v5-R1-physical-and-exact-intents')
    proof = value['mapped_cohort_closure']
    need(proof['accepted_publications'] == expected_h and proof['mapped_keys'] == 504833 and
         proof['captured_selected_authority'] is True and proof['candidate_context'] is False,
         'full-selected-mapped-cohort-closure')
    need(value['navigation']['immutable_discovery_semantics_verified'] is True,
         'complete-independent-navigation')
    if baseline is None:
        need(semantic['selected'] == SELECTED and semantic['permanent_digest'] == PERMANENT,
             'exact-retained-semantic-baseline')
    else:
        need(set(baseline) == {'selected','permanent_digest','publications'} and
             baseline['selected'] == SELECTED and baseline['publications'] == ACTUAL_H and
             baseline['permanent_digest'] == PERMANENT, 'top-level-successor-baseline')
    return dict(selected=semantic['selected'], permanent_digest=semantic['permanent_digest'],
                publications=semantic['publications'], actual_H=expected_h,
                full_physical_audit=True, exact_intents=True)


def run_oracle(host, c, adapter, root, label, known, expected_h, baseline=None):
    before = adapter.inventory(host, c, root, maximum=c['limits']['source_construction_bytes'])
    core = adapter.load_stream(c)
    tokens = dict(ROOT=str(root), OUTPUT=str(Path(c['roots']['metadata']) / label / 'oracle.json'),
                  INVENTORY=c['bindings']['cohort_inventory']['path'], KNOWN=known['path'],
                  PROVENANCE=c['bindings']['request_provenance']['path'])
    argv = []
    for token in c['cli']['oracle_argv']:
        if token.startswith('{') and token.endswith('}'):
            need(token[1:-1] in tokens, 'unknown-oracle-argv-placeholder')
            token = tokens[token[1:-1]]
        argv.append(token)
    if baseline is not None:
        argv += ['--baseline', baseline['path']]
    profile = c['_forecast']['profiles']['oracle']
    spec = dict(kind='python-argv', api='lane-invoke-v1', label=label,
                executable=c['bindings']['python'], script=c['bindings']['oracle_cli'],
                argv=argv, transitive_bindings=c['bindings'], profile=profile,
                startup_gate=c['bindings']['startup_gate'],
                required_gate=c['cli']['startup_gate'], role='checker', timeout_seconds=2400,
                max_stdin_bytes=0, max_stdout_bytes=profile['stdout'],
                max_stderr_bytes=profile['stderr'], max_receipt_bytes=profile['receipt'],
                output=dict(path=tokens['OUTPUT'], max_bytes=profile['output'],
                            category=c['categories']['destination_metadata']),
                growth=c['phase_reserves'][c['_phase']], allowed_exit_codes=[0])
    result = host.invoke(spec)
    need(type(result) is dict and result.get('exit_code') == 0 and
         result.get('startup_gate_before_corpus_IO') is True and
         result.get('children_empty') is True and result.get('source_pin') == c['final_pin'] and
         result.get('oracle_sha256') == c['bindings']['oracle']['sha256'] and
         hex64(result.get('budget_receipt_sha256')) and hex64(result.get('closure_receipt_sha256')),
         'oracle-pinned-native-admission-and-closure')
    value = check_ref(result['output'], profile['output'], label)
    need(value['inventory_digest'] == c['cohort_digest'], 'exact-all-504833-record-cohort')
    normalized = oracle_summary(value, expected_h, baseline and baseline['value'])
    after = adapter.inventory(host, c, root, maximum=c['limits']['source_construction_bytes'])
    core.compare_source(before, after)
    proof = record(host, c, label+'-inventory-proof.json',
                   dict(before=core.inventory_digest(before), after=core.inventory_digest(after),
                        files=len(before['files']), directories=len(before['directories']),
                        full_file_directory_hash_proof=True), 4096, True)
    summary = record(host, c, label+'-normalized.json', normalized, 16384, True)
    return dict(value=normalized, raw=result['output'], inventory=proof, normalized=summary,
                full_file_directory_hash_proof=True,
                budget_receipt_sha256=result['budget_receipt_sha256'],
                closure_receipt_sha256=result['closure_receipt_sha256'])


def validate_samples(rows, warm, expected):
    need(type(rows) is list and len(rows) == 300, 'exact300-fresh-samples')
    need(type(warm) is dict and type(warm.get('rows')) is list and
         len(warm['rows']) == 6, 'six-warm-categories')
    summaries = {}
    for category, request_id in REQUESTS:
        samples = [r for r in rows if r['category'] == category]
        need(len(samples) == 50 and {r['sample'] for r in samples} == set(range(50)),
             'exact50-fresh-each-category')
        need(all(r['id'] == request_id and r['result'] == expected[category] and
                 type(r['engine_ns']) is int and r['engine_ns'] >= 0 and
                 type(r['spawn_total_ns']) is int and r['spawn_total_ns'] >= r['engine_ns']
                 and hex64(r['raw']['sha256']) for r in samples), 'fresh-raw-outcome-clock')
        candidates = [r for r in warm['rows'] if r['category'] == category]
        need(len(candidates) == 1, 'unique-warm-category')
        w = candidates[0]
        need(set(w) == {'category','id','first_lookup_ns','warm_ns','warm_median_ns',
             'warm_p95_ns','warm_min_ns','warm_max_ns','result_digest','opens',
             'pack_storage_bytes','decoded_record_bytes','index_cache_hits'} and
             w['id'] == request_id, 'closed-warm-shape-and-exact-original-id')
        need(len(w['warm_ns']) == 500 and all(type(n) is int and n >= 0 for n in w['warm_ns']),
             'exact500-warm-samples')
        outcome_digest = digest(json.dumps(expected[category], sort_keys=True,
                                separators=(',', ':'), ensure_ascii=False).encode())
        need(w['result_digest'] == outcome_digest, 'warm-exact-outcome')
        summaries[category] = dict(engine_median_ns=statistics.median(r['engine_ns'] for r in samples),
            engine_max_ns=max(r['engine_ns'] for r in samples),
            spawn_median_ns=statistics.median(r['spawn_total_ns'] for r in samples),
            spawn_max_ns=max(r['spawn_total_ns'] for r in samples),
            warm_median_ns=statistics.median(w['warm_ns']), warm_max_ns=max(w['warm_ns']))
    return summaries


def sample(host, c, adapter, label):
    root = Path(c['roots']['source'])
    expected = check_ref(c['bindings']['lookup_expectations'], MiB, 'lookup-expectations')
    need(expected['actual_H'] == ACTUAL_H and expected['selected'] == SELECTED and
         expected['requests'] == [dict(category=k, id=v) for k, v in REQUESTS], 'same-matched-six-requests')
    before = selected(adapter, c, root)
    need(before == SELECTED, 'sample-original-semantic-head')
    warm = invoke(host, c, label + '-warm', root, 'lookup-bench',
                  dict(count=500, requests=expected['requests']))
    rows = []
    for category, request_id in REQUESTS:
        for number in range(50):
            r = invoke(host, c, label + '-' + category + '-' + str(number), root,
                       'resolve', dict(request_id=request_id))
            rows.append(dict(category=category, id=request_id, sample=number, **r))
    summary = validate_samples(rows, warm['result'], expected['expected_outcomes'])
    need(selected(adapter, c, root) == before, 'lookup-mutated-selected')
    ref = record(host, c, label + '-samples.json',
                 dict(actual_H=ACTUAL_H, source_pin=c['final_pin'],
                      binary_sha256=c['bindings']['engine']['sha256'], fresh=rows,
                      warm=warm, summary=summary,
                      cache='new-process-only-OS-and-device-uncontrolled', p99_claim=False),
                 c['limits']['sample_bytes'])
    return dict(summary=summary, raw=ref)


def cleanup(host, c, adapter):
    root = Path(c['roots']['source'])
    before = selected(adapter, c, root)
    receipts = []
    for index in range(c['limits']['cleanup_batches']):
        poll(host)
        r = invoke(host, c, 'cleanup-' + str(index), root, 'packing',
                   dict(operation='cleanup', max_candidates=1000, time_budget_ms=50))
        receipts.append(r)
        value = r['result']
        need(type(value) is dict and value.get('status') in ('cleaning', 'complete') and
             value.get('candidate_limit') == 1000 and value.get('cooperative_limit_ms') == 50,
             '9ec7-bounded-cleanup-terminal-API')
        need(selected(adapter, c, root) == before, 'cleanup-semantic-change')
        if value['status'] == 'complete':
            need(all(value.get(k) == 0 for k in ('pending_loose_jobs', 'pending_old_containers',
                 'pending_metadata', 'pending_aborted_owners')), 'cleanup-all-work-terminal')
            return record(host, c, 'cleanup-batches.json', receipts, c['limits']['cleanup_receipt_bytes'])
    raise RuntimeError('cleanup-budget-exhausted-no-expiry-or-silent-default160')


def compare_lookup(legacy, paged):
    comparison = {}
    for category, _ in REQUESTS:
        a, b = legacy['summary'][category], paged['summary'][category]
        allowance = max(a['warm_median_ns'] * .20, 200000)
        comparison[category] = dict(
            fresh_engine_speedup=(a['engine_median_ns'] / b['engine_median_ns']
                                  if b['engine_median_ns'] else None),
            fresh_spawn_speedup=(a['spawn_median_ns'] / b['spawn_median_ns']
                                 if b['spawn_median_ns'] else None),
            warm_tolerance_ns=allowance,
            warm_within_tolerance=b['warm_median_ns'] <= a['warm_median_ns'] + allowance)
    return comparison


def freeze_stream_config(c):
    stream = dict(c['stream'])
    need(stream['source'] == c['roots']['source'] and
         stream['destination'] == c['roots']['destination'] and
         stream['metadata_root'] == str(Path(c['roots']['metadata']) / 'stream') and
         stream['max_metadata_bytes'] == 2*MiB and
         stream['max_destination_bytes'] <= 651179227 and
         stream['max_source_bytes'] <= 651179227 and
         stream['max_files'] <= 7908 and stream['control_only'] is False,
         'frozen-stream-subset-inside-F11-M-and-successor')
    need(stream['binding']['source_pin'] == c['final_pin'] and
         stream['binding']['oracle']['sha256'] == c['bindings']['oracle']['sha256'],
         'same-final-stream-oracle-binding')
    return stream


def release_engine(host, c, root, byte_proof, oracle):
    release = host.authorize(dict(root=str(root), final_pin=c['final_pin'], byte_proof=byte_proof,
         oracle=oracle['raw'], inventory=oracle['inventory'], forecast_sha256=c['_forecast_sha256']),
         'long:verified-engine-release')
    need(release.get('go') is True and release.get('engine_open_allowed') is True and
         release.get('root') == str(root) and release.get('final_pin') == c['final_pin'] and
         release.get('oracle_sha256') == oracle['raw']['sha256'] and
         hex64(release.get('receipt_sha256')), 'narrow-native-quarantine-release')
    return release


def run_long(host, config):
    """No deletion/forecast expiry is automatic. Every failure retains quarantine."""
    c = dict(validate_config(config))
    adapter = load_adapter(c)
    profile_binding = binding(c['bindings']['profile_calculator'], 'profile-calculator')
    spec = importlib.util.spec_from_file_location('reviewed_long_profiles', profile_binding['path'])
    calculator = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(calculator)
    need(getattr(calculator, 'API', None) == 'long-source-profiles-v1',
         'PENDING-source-derived-action-QPC-host-oracle-profiles')
    c['_forecast'] = adapter.forecast_long(c, calculator.long_profiles(c))
    c['_forecast_sha256'] = digest(encode(c['_forecast']))
    permit = host.authorize(c, 'long')
    need(type(permit) is dict and permit.get('go') is True and
         permit.get('final_pin') == c['final_pin'] and
         hex64(permit.get('receipt_sha256')), 'parent-runtime-GO')
    for name, row in c['bindings'].items():
        binding(row, name)
    root, dest = Path(c['roots']['source']), Path(c['roots']['destination'])
    need(not os.path.lexists(root) and not os.path.lexists(dest), 'fresh-owned-quarantines-only')
    stream_config = freeze_stream_config(c)
    proof = {}
    try:
        phase(host, c, 'source-restore')
        proof['source_forecast'] = record(host, c, 'long-forecast.json', c['_forecast'], 16384)
        proof['archive'] = adapter.legacy_restore(host, c)
        need(proof['archive']['engine_open_allowed'] is False, 'byte-quarantine')
        known = prepare_known(host, c)
        phase(host, c, 'legacy-oracle')
        proof['legacy_oracle'] = run_oracle(host, c, adapter, root, 'legacy', known, ACTUAL_H)
        c['_legacy_release'] = release_engine(host, c, root, proof['archive']['receipt'], proof['legacy_oracle'])
        phase(host, c, 'legacy-samples')
        proof['legacy'] = sample(host, c, adapter, 'legacy')
        phase(host, c, 'convert')
        invoke(host, c, 'lifecycle-marker', root, 'lifecycle', dict(operation='setup'))
        converted = invoke(host, c, 'convert', root, 'packing',
            dict(operation='convert', force=True, cleanup=False, reserve_bytes=320*MiB), timeout=2400)
        need(selected(adapter, c, root) == SELECTED, 'convert-semantic-head')
        proof['convert'] = converted
        phase(host, c, 'converted-oracle')
        proof['converted_oracle'] = run_oracle(host, c, adapter, root, 'converted', known, ACTUAL_H)
        proof['convert_drain'] = host.drain('long:converted-oracle')
        need(proof['convert_drain'].get('children_empty') is True and
             hex64(proof['convert_drain'].get('receipt_sha256')), 'convert-owned-closure')
        phase(host, c, 'paged-samples')
        proof['paged'] = sample(host, c, adapter, 'paged')
        proof['comparison'] = compare_lookup(proof['legacy'], proof['paged'])
        phase(host, c, 'cleanup')
        proof['cleanup'] = cleanup(host, c, adapter)
        phase(host, c, 'cleaned-oracle')
        proof['cleaned_oracle'] = run_oracle(host, c, adapter, root, 'cleaned', known, ACTUAL_H)
        frozen_source = adapter.inventory(host, c, root, maximum=651179227)
        need(len(frozen_source['files']) <= 7908 and
             sum(r['bytes'] for r in frozen_source['files']) <= 651179227,
             'F11-source-selected-envelope')
        phase(host, c, 'stream-restore')
        core = adapter.load_stream(c)
        def independent_destination(destination, byte_proof):
            need(Path(destination) == dest and byte_proof['both_helpers_closed'] is True,
                 'byte-closure-before-oracle')
            phase(host, c, 'restored-oracle')
            restored = run_oracle(host, c, adapter, dest, 'restored', known, ACTUAL_H)
            proof['restored_oracle'] = restored
            return dict(status='passed', oracle_sha256=c['bindings']['oracle']['sha256'],
                        raw=restored['raw'], actual_H=ACTUAL_H)
        proof['transport'] = core.capture_restore(stream_config, independent_destination)
        c['_dest_release'] = release_engine(host, c, dest, proof['transport'], proof['restored_oracle'])
        core.compare_source(frozen_source, adapter.inventory(host, c, root, maximum=651179227))
        phase(host, c, 'successor')
        request = dict(request=SUCCESSOR, base=SELECTED, ops=[], packet=False)
        known_value = check_ref(known, MiB, 'known-nonhistory')
        need(SUCCESSOR not in known_value, 'successor-unique')
        known_value[SUCCESSOR] = request
        successor_known = record(host, c, 'known-successor.json', known_value, 96*1024, True)
        publish = invoke(host, c, 'accepted-destination-successor', dest, 'publish', request)
        need(publish['result'].get('status') == 'published' and
             publish['result']['result'].get('outcome') == 'accepted', 'successor-must-be-accepted')
        resolved = invoke(host, c, 'resolve-destination-successor', dest, 'resolve',
                          dict(request_id=SUCCESSOR))
        need(resolved['result'] == publish['result']['result'], 'durable-exact-successor-resolution')
        need(selected(adapter, c, root) == SELECTED, 'source-remains-frozen-after-destination-write')
        baseline_value = {k: proof['restored_oracle']['value'][k]
                          for k in ('selected', 'permanent_digest', 'publications')}
        baseline_ref = record(host, c, 'successor-baseline.json', baseline_value, 4096, True)
        phase(host, c, 'successor-oracle')
        proof['successor_oracle'] = run_oracle(host, c, adapter, dest, 'successor', successor_known,
                                               ACTUAL_H+1, dict(value=baseline_value, **baseline_ref))
        proof['successor_publish'], proof['successor_resolve'] = publish, resolved
        core.compare_source(frozen_source, adapter.inventory(host, c, root, maximum=651179227))
        phase(host, c, 'closure')
        proof['closure'] = host.drain('long:final')
        need(proof['closure'].get('children_empty') is True and
             hex64(proof['closure'].get('receipt_sha256')), 'final-owned-process-IO-handle-closure')
        proof.update(status='PROVISIONAL_UNQUALIFIED-awaiting-postexit-QPC-join',
                     source_pin=c['final_pin'], actual_H_before=ACTUAL_H, actual_H_after=ACTUAL_H+1,
                     F11_retained_through_postsuccessor_oracle=True,
                     outer_QPC_two_empty_Job_polls='parent-required-after-this-process-exits',
                     claim_expiry='parent-only-after-current-actual-census',
                     mandatory_other_matrix='not-qualified-by-this-lane')
        return record(host, c, 'long-result.json', proof, c['limits']['result_bytes'])
    except BaseException as error:
        closure = host.drain('long:failure')
        record(host, c, 'long-failure.json',
               dict(status='incomplete-retain-all-quarantines', error=ascii(error)[:1500],
                    closure=closure, deletion=False, automatic_retry=False,
                    forecast_expiry=False), 16384)
        raise

