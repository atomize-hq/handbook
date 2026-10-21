# SOURCE ONLY: direct R3 runtime API +035 formulas; effective046 admission and post-exit qualification require independent review/GO. No module/control/native execution in source preparation.
import copy
import hashlib
import json
import math
import time
from datetime import datetime
from pathlib import PureWindowsPath, PurePosixPath
VERSION = 1
MIB = 1024 * 1024
F05_BYTES = 1073741824
F05_FILES = 400000
QPC_SHA256 = '50ecc7c40c1cdda6493817261b5755fe2d823fdae271118698828c39a0400378'
SOURCE_BUDGET_SHA256 = 'dd1b8ebfb2b35e32fde2873da2d8e5a4becb77c49e1f27a9649ac63ffa97c785'
FUNCTIONAL_UTC = '2026-10-21T11:10:14Z'
SOURCE_FREEZE_UTC = '2026-10-21T04:10:14Z'
CATEGORIES = ('early', 'middle', 'recent', 'missing', 'cancelled', 'lostreply')
ROOTS = {'H1': 'C:/file-native-online-matrix-20261020/history/H1000-baseline/disposable/store', 'H10': 'C:/file-native-online-matrix-20261020/history/authentic-N128/disposable/store'}
HASH_FIELDS = ('engine_manifest_sha256', 'binary_sha256', 'core_sha256', 'runner_sha256', 'oracle_sha256', 'oracle_review_sha256', 'terminal_api_sha256', 'terminal_review_sha256', 'qpc_sha256', 'budget_sha256', 'phase_go_sha256', 'source_budget_sha256', 'history_source_sha256', 'reader_probe_sha256', 'growth_capsule_sha256', 'postexit_finalizer_sha256', 'clock_handshake_sha256')
F05_COMPONENTS = ('accepted_writes', 'sealed_losers', 'terminal_receipts', 'terminal_registry_and_atomic_stages', 'accepted_registry_stages', 'legacy_to_paged_coexistence', 'rolling_and_consolidation_peak', 'generation_pins_work_catalogs_journals', 'frozen_inventories', 'oracle_metadata', 'controls_and_zero_byte_files', 'finalization_and_failed_prefixes')
CONTROL_PLAN = {'execution': 'PENDING_PARENT_SOURCE_REVIEW_AND_SEPARATE_MEMORY_CONTROL_GO', 'native_processes': 0, 'engine_calls': 0, 'files_written': 0, 'max_donors': 2, 'max_phase_rows_per_case': 100, 'max_offers_per_phase_case': 400, 'max_lookup_rows_per_representation': 300, 'max_cases': 48, 'proposed_traced_python_bytes': 16 * MIB, 'positive': ['two exact donors / four cells / sixteen phases / 1600 acceptances', 'complete closed component forecast with zero-valued proven exclusions', 'complete final bindings', 'two-format six-category lookup', 'one-publisher phase', 'four-publisher phase', 'H10/H1 work comparison'], 'negative': ['unknown/missing config fields', 'bool accepted as int', 'wrong donor/root/initial H', 'duplicate lookup category', 'non-ASCII or long ID', 'null final pin', 'wrong QPC/source budget', 'partial774M forecast', 'missing terminal/stage/file component', 'planned deletion credit', 'F05 overcap', 'missing evidence allowance', 'lookup after semantic write', 'paged before legacy', 'wrong lookup outcome / omitted raw sample', 'only old baseline binding', '99 or101 acceptances', 'extra offer or hidden retry', 'wrong request ops/base', 'p1 semantic conflict', 'p4 not exactly one winner', 'missing losing owner receipt', 'quota or time censor relabeled conflict', 'reader exhaustion', 'four threads without native identities', 'reader result mismatch', 'held reader <=15s or fewer than2 mapped successors', 'held reader dummy idle before workload', 'protected generation overflow', 'maintenance during control', 'zero real online overlap', 'missing full oracle/hash closure', 'H10 work >2x H1', 'nonzero history enumeration under global exclusion', 'failed native/Python closure', 'unreleased runner API']}

class HistoryRefusal(RuntimeError):
    pass

def _need(condition, reason):
    if not condition:
        raise HistoryRefusal(reason)

def _closed(value, keys, name):
    _need(type(value) is dict and set(value) == set(keys), 'closed-shape:' + name)
    return value

def _integer(value, low, high, name):
    _need(type(value) is int and low <= value <= high, 'integer-bound:' + name)
    return value

def _digest(value, length=64):
    return type(value) is str and len(value) == length and all((c in '0123456789abcdef' for c in value))

def _same(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, ensure_ascii=False, separators=(',', ':'), allow_nan=False).encode('utf-8')).hexdigest()

def history_plan(config):
    _closed(config, ('version', 'donors', 'slot_interval_ns', 'reader_calls', 'reader_queries_per_call'), 'history-plan')
    _need(config['version'] == VERSION and type(config['version']) is int, 'plan-version')
    _need(config['slot_interval_ns'] == 400000000 and type(config['slot_interval_ns']) is int, 'frozen-matched-400ms-offer-cadence')
    _need(type(config['reader_calls']) is int and config['reader_calls'] == 40 and (type(config['reader_queries_per_call']) is int) and (config['reader_queries_per_call'] == 1000), 'finite-40call-40000query-source-bound')
    _need(type(config['donors']) is list and len(config['donors']) == 2, 'two-donors-required')
    donors, phases, lookups = ({}, [], [])
    for donor in config['donors']:
        _closed(donor, ('id', 'root', 'actual_H', 'selected', 'seed_receipt_sha256', 'accepted_inputs_sha256', 'request_provenance_sha256', 'lookup_requests', 'expected_outcomes'), 'donor')
        identity = donor['id']
        _need(identity in ROOTS and identity not in donors, 'distinct-authentic-donor')
        _need(donor['root'] == ROOTS[identity], 'owned-existing-lineage-only')
        _need(type(donor['actual_H']) is int and donor['actual_H'] == {'H1': 1002, 'H10': 10002}[identity], 'exact-unmutated-lookup-H')
        _closed(donor['selected'], ('publication', 'digest'), 'selected')
        _need(type(donor['selected']['publication']) is str and donor['selected']['publication'].isascii() and (1 <= len(donor['selected']['publication']) <= 99) and _digest(donor['selected']['digest']), 'captured-selected-binding')
        _need(all((_digest(donor[key]) for key in ('seed_receipt_sha256', 'accepted_inputs_sha256', 'request_provenance_sha256'))), 'complete-donor-provenance')
        requests = donor['lookup_requests']
        _need(type(requests) is list and len(requests) == 6, 'six-original-categories')
        _need(type(donor['expected_outcomes']) is dict and set(donor['expected_outcomes']) == set(CATEGORIES), 'six-exact-outcomes')
        categories, names = (set(), set())
        for request in requests:
            _closed(request, ('category', 'id'), 'lookup-request')
            category, name = (request['category'], request['id'])
            _need(category in CATEGORIES and category not in categories, 'unique-six-categories')
            _need(type(name) is str and name.isascii() and (1 <= len(name) <= 64) and (name not in names), 'original-bounded-distinct-lookup-id')
            categories.add(category)
            names.add(name)
        donors[identity] = copy.deepcopy(donor)
        lookups.append(dict(donor=identity, root=donor['root'], actual_H=donor['actual_H'], selected=copy.deepcopy(donor['selected']), order=['final-candidate-legacy', 'convert-paged', 'final-candidate-paged'], fresh_per_category=50, warm_per_category=500, semantic_writes_permitted=0, categories=list(CATEGORIES)))
        head = donor['actual_H']
        for publishers in (1, 4):
            for cycle in range(4):
                label = identity.lower() + '-p' + str(publishers) + '-' + ('control' if cycle == 0 else 'online' + str(cycle))
                phases.append(dict(id=label, donor=identity, root=donor['root'], publishers=publishers, online=cycle > 0, cycle=cycle, H_start=head, H_end=head + 100, accepted=100, max_offers=100 * publishers, max_sealed_losers=100 * (publishers - 1), reader_slots=4, reader_calls=40, warm_queries=40000, first_queries_max=40, slot_interval_ns=400000000, bootstrap_cleanup_max_calls=160, online_cleanup_max_calls=99, boundary_cleanup_calls=2))
                head += 100
    _need(set(donors) == set(ROOTS) and len(phases) == 16, 'complete-four-cells')
    return dict(version=VERSION, method='authentic-A-N128-ops-empty-packet-false', interpretation='publication/history maintenance contention; not mixed payload transition', donors=donors, lookups=lookups, phases=phases, total_accepted=1600, max_offers=4000, max_sealed_losers=2400, initial_mixed_sources_preserved=True, default_payloads=True, profile=4, no_clone=True, F05_bytes=F05_BYTES, F05_files=F05_FILES, fixed_functional_cutoff=FUNCTIONAL_UTC, finite_command_bounds=dict(publish_offers=4000, online_mapping_builds=1200, config_bootstrap_builds=4, bootstrap_cleanup=640, lookup_cleanup=320, online_cleanup=1188, boundary_cleanup=24, terminal_releases=2400, reader_calls=640, reader_warm_queries=640000, reader_first_queries=640, fresh_lookup_processes=1200, initial_warm_queries=12000), plan_sha256=_same(config))

def forecast_history(components):
    _closed(components, ('version', 'complete', 'current', 'F05', 'protocol', 'evidence', 'source_pin', 'proof_sha256', 'planned_deletion_credit', 'future_reassignment_credit'), 'whole-history-forecast')
    _need(components['version'] == VERSION and type(components['version']) is int and (components['complete'] is True), 'complete-finite-forecast-required')
    _need(_digest(components['source_pin'], 40) and _digest(components['proof_sha256']), 'final-source-bound-forecast')
    _need(type(components['planned_deletion_credit']) is int and components['planned_deletion_credit'] == 0 and (type(components['future_reassignment_credit']) is int) and (components['future_reassignment_credit'] == 0), 'no-planned-or-future-reassignment-credit')
    _closed(components['F05'], F05_COMPONENTS, 'F05-complete-components')
    all_parts = [('current', components['current'])] + list(components['F05'].items())
    total_bytes = total_files = total_dirs = 0
    for name, part in all_parts:
        _closed(part, ('bytes', 'files', 'directories', 'source_pin', 'proof_sha256'), name)
        total_bytes += _integer(part['bytes'], 0, F05_BYTES, name + 'bytes')
        total_files += _integer(part['files'], 0, F05_FILES, name + 'files')
        total_dirs += _integer(part['directories'], 0, 5000000, name + 'directories')
        _need(part['source_pin'] == components['source_pin'] and _digest(part['proof_sha256']), 'missing-component-source-proof:' + name)
    _need(components['current']['bytes'] >= 552141207 and components['current']['files'] >= 129768, 'no-unproved-current-census-subtraction')
    _need(total_bytes <= F05_BYTES and total_files <= F05_FILES, 'F05-whole-phase-overcap')
    supplemental = {}
    for group in ('protocol', 'evidence'):
        part = components[group]
        _closed(part, ('additional_bytes', 'additional_files', 'available_bytes', 'available_files', 'source_pin', 'proof_sha256', 'includes_finalization', 'membership_disjoint'), group)
        _need(part['source_pin'] == components['source_pin'] and _digest(part['proof_sha256']) and (part['includes_finalization'] is True) and (part['membership_disjoint'] is True), 'bound-all-raw-and-finalization:' + group)
        for key in ('additional_bytes', 'additional_files', 'available_bytes', 'available_files'):
            _integer(part[key], 0, 2 ** 63 - 1, group + key)
        _need(0 < part['additional_bytes'] <= part['available_bytes'] and 0 < part['additional_files'] <= part['available_files'], 'supplemental-headroom:' + group)
        supplemental[group] = copy.deepcopy(part)
    return dict(status='component-arithmetic-only-PROVISIONAL_UNQUALIFIED', source_pin=components['source_pin'], bytes=total_bytes, files=total_files, directories=total_dirs, byte_headroom=F05_BYTES - total_bytes, file_headroom=F05_FILES - total_files, protocol=supplemental['protocol'], evidence=supplemental['evidence'], proof_sha256=components['proof_sha256'], components_sha256=_same(components), actual_reclaimed_credit=0, still_requires_fresh_actual_preflight=True)

def validate_release(bindings, *, expected_source_budget_sha256):
    _closed(bindings, ('version', 'source_pin', *HASH_FIELDS, 'api', 'source_review', 'oracle_review', 'terminal_review', 'runtime_go', 'default_payloads', 'profile', 'strict_25Hz', 'phase_go_scope', 'cutoff', 'source_freeze', 'same_final_pin_native'), 'history-release')
    _need(type(bindings['version']) is int and bindings['version'] == VERSION, 'release-version')
    _need(_digest(bindings['source_pin'], 40), 'final-source-pin-not-released')
    for key in HASH_FIELDS:
        _need(_digest(bindings[key]), 'final-binding-unreleased:' + key)
    _need(bindings['api'] == 'history-native-v1', 'unreleased-runner-interface')
    _need(_digest(expected_source_budget_sha256) and bindings['qpc_sha256'] != QPC_SHA256 and bindings['source_budget_sha256'] == expected_source_budget_sha256, 'exact-QPC-and-admitted-source-budget')
    _need(all((bindings[key] is True for key in ('source_review', 'oracle_review', 'terminal_review', 'runtime_go', 'default_payloads', 'strict_25Hz', 'same_final_pin_native'))), 'final-review-or-separate-runtime-GO-required')
    _need(type(bindings['profile']) is int and bindings['profile'] == 4, 'default-profile4')
    freeze = _closed(bindings['source_freeze'], ('engine', 'harness', 'manifest_ref', 'review_ref'), 'R3-source-freeze')
    _need(freeze['engine'] == SOURCE_FREEZE_UTC, 'fixed-engine-checkpoint')
    stamp = freeze['harness']
    _need(type(stamp) is str and 20 <= len(stamp) <= 27 and stamp.endswith('Z'), 'actual-harness-UTC')
    try:
        instant = datetime.fromisoformat(stamp[:-1] + '+00:00')
    except ValueError as error:
        raise HistoryRefusal('actual-harness-UTC') from error
    _need(stamp[:19] == instant.strftime('%Y-%m-%dT%H:%M:%S') and
          (len(stamp) == 20 or (stamp[19] == '.' and stamp[20:-1].isascii() and stamp[20:-1].isdigit())) and
          datetime.fromisoformat(SOURCE_FREEZE_UTC[:-1] + '+00:00') <= instant <
          datetime.fromisoformat(FUNCTIONAL_UTC[:-1] + '+00:00'), 'prospective-harness-freeze-within-fixed-clock')
    # Native parent authority verifies these ranges before this pure validator.
    for name in ('manifest_ref', 'review_ref'):
        ref = _closed(freeze[name], ('path', 'offset', 'bytes', 'sha256'), name)
        path = ref['path']
        _need(type(path) is str and path.isascii() and 1 <= len(path) <= 4096 and
              (PureWindowsPath(path).is_absolute() or PurePosixPath(path).is_absolute()), 'absolute-freeze-reference')
        _integer(ref['offset'], 0, 2 ** 63 - 1, name + '-offset')
        _integer(ref['bytes'], 1, 64 * MIB, name + '-bytes')
        _need(_digest(ref['sha256']), 'freeze-reference-digest')
    _need(bindings['phase_go_scope'] == 'all1600-plus-initial-exact-lookups' and bindings['cutoff'] == FUNCTIONAL_UTC, 'bounded-phase-GO-and-shared-clock')
    return copy.deepcopy(bindings)

def _quantiles(values):
    _need(type(values) is list and bool(values), 'nonempty-latency-samples')
    for value in values:
        _integer(value, 0, 2 ** 63 - 1, 'latency-ns')
    ordered = sorted(values)
    return dict(count=len(values), median=ordered[(len(values) - 1) // 2], p95=ordered[math.ceil(len(values) * 0.95) - 1], max=ordered[-1], p99_qualified=False)

def _metric_totals(rows):
    keys = ('rolling_publications_visited', 'rolling_publication_body_opens', 'rolling_discovery_metadata_bytes', 'rolling_deferred_rechecked', 'pack_metadata_stats', 'pack_index_metadata_bytes', 'pack_storage_bytes', 'pack_decoded_bytes')
    totals = {key: 0 for key in keys}
    for row in rows:
        _need(type(row) is dict and all((key in row for key in keys)), 'source-bound-work-counters-required')
        for key in keys:
            totals[key] += _integer(row[key], 0, 2 ** 63 - 1, 'metric:' + key)
    return totals

def validate_lookup_evidence(plan, evidence):
    _closed(evidence, ('donor', 'actual_H', 'selected_before', 'selected_after', 'source_pin', 'binary_sha256', 'order', 'semantic_writes', 'representations', 'complete_oracle', 'semantic_unchanged', 'raw_samples_retained', 'receipt_sha256', 'post_measurement_ready'), 'lookup-evidence')
    donor = plan['donors'].get(evidence['donor'])
    _need(donor is not None and evidence['actual_H'] == donor['actual_H'] and (type(evidence['actual_H']) is int), 'lookup-exact-initial-H')
    _need(evidence['selected_before'] == donor['selected'] == evidence['selected_after'], 'lookup-same-captured-selected')
    _need(evidence['order'] == ['final-candidate-legacy', 'convert-paged', 'final-candidate-paged'] and type(evidence['semantic_writes']) is int and (evidence['semantic_writes'] == 0), 'legacy-before-paged-before-semantic-writes')
    _need(_digest(evidence['source_pin'], 40) and _digest(evidence['binary_sha256']) and _digest(evidence['receipt_sha256']) and all((evidence[key] is True for key in ('complete_oracle', 'semantic_unchanged', 'raw_samples_retained'))), 'complete-bound-lookup-proof')
    ready = evidence['post_measurement_ready']
    _closed(ready, ('actual_cleanup_complete', 'private_v2_enabled', 'NOOP_preconditions', 'initial_active_reservations', 'no_semantic_change', 'receipt_sha256'), 'post-lookup-ready')
    _need(all((ready[key] is True for key in ('actual_cleanup_complete', 'private_v2_enabled', 'NOOP_preconditions', 'no_semantic_change'))) and type(ready['initial_active_reservations']) is int and (ready['initial_active_reservations'] == 0) and _digest(ready['receipt_sha256']), 'post-measurement-cleanup-before-v2-control')
    representations = evidence['representations']
    _need(type(representations) is list and len(representations) == 2, 'both-finalcandidate-formats')
    summaries = []
    original_ids = {row['category']: row['id'] for row in donor['lookup_requests']}
    for fmt, representation in zip(('legacy', 'paged'), representations):
        _closed(representation, ('format', 'source_pin', 'binary_sha256', 'actual_H', 'fresh', 'warm', 'full_oracle_sha256'), 'lookup-representation')
        _need(representation['format'] == fmt and representation['source_pin'] == evidence['source_pin'] and (representation['binary_sha256'] == evidence['binary_sha256']) and (representation['actual_H'] == donor['actual_H']) and _digest(representation['full_oracle_sha256']), 'same-finalcandidate-format-binding')
        fresh, warm = (representation['fresh'], representation['warm'])
        _need(type(fresh) is list and len(fresh) == 300 and (type(warm) is list) and (len(warm) == 6), 'complete-50fresh-500warm')
        by_category, samples, pids = ({key: [] for key in CATEGORIES}, set(), set())
        for row in fresh:
            _closed(row, ('category', 'id', 'sample', 'result', 'engine_ns', 'spawn_ns', 'pid', 'rss_bytes', 'metrics', 'raw_sha256'), 'fresh-sample')
            category = row['category']
            _need(category in CATEGORIES and row['id'] == original_ids[category] and (row['result'] == donor['expected_outcomes'][category]), 'fresh-exact-outcome')
            sample = _integer(row['sample'], 0, 49, 'fresh-sample-number')
            _need((category, sample) not in samples, 'duplicate-fresh-sample')
            samples.add((category, sample))
            _integer(row['pid'], 1, 2 ** 32 - 1, 'fresh-native-pid')
            _integer(row['rss_bytes'], 1, 256 * MIB, 'fresh-reader-RSS')
            _integer(row['engine_ns'], 0, 2 ** 63 - 1, 'fresh-engine-ns')
            _integer(row['spawn_ns'], row['engine_ns'], 2 ** 63 - 1, 'fresh-spawn-ns')
            _need(type(row['metrics']) is dict and _digest(row['raw_sha256']), 'fresh-raw-metrics')
            pids.add(row['pid'])
            by_category[category].append(row)
        _need(len(pids) >= 2, 'fresh-native-process-evidence-required')
        seen = set()
        for row in warm:
            _closed(row, ('category', 'id', 'result', 'first_ns', 'warm_ns', 'metrics', 'raw_sha256'), 'warm-sample')
            category = row['category']
            _need(category in CATEGORIES and category not in seen and (row['id'] == original_ids[category]) and (row['result'] == donor['expected_outcomes'][category]), 'warm-exact-six-outcomes')
            seen.add(category)
            _need(type(row['warm_ns']) is list and len(row['warm_ns']) == 500 and (type(row['metrics']) is dict) and _digest(row['raw_sha256']), 'warm-full-raw-samples')
            _integer(row['first_ns'], 0, 2 ** 63 - 1, 'separate-first-lookup')
            _quantiles(row['warm_ns'])
        summaries.append(dict(format=fmt, categories={category: dict(engine=_quantiles([row['engine_ns'] for row in rows]), spawn=_quantiles([row['spawn_ns'] for row in rows])) for category, rows in by_category.items()}, cache='OS/device caches uncontrolled'))
    return dict(status='lookup-evidence-complete', donor=donor['id'], actual_H=donor['actual_H'], summaries=summaries, receipt_sha256=evidence['receipt_sha256'], p99_qualified=False)

def validate_phase_evidence(phase, evidence):
    _closed(evidence, ('phase', 'source_pin', 'binary_sha256', 'receipt_sha256', 'H_start', 'H_end', 'slots', 'readers', 'maintenance', 'terminal_receipts', 'oracle', 'closure', 'guard_observation', 'frontend_censors', 'maintenance_refusals', 'hidden_retries'), 'phase-evidence')
    _need(evidence['phase'] == phase['id'] and evidence['H_start'] == phase['H_start'] and (evidence['H_end'] == phase['H_end']) and (type(evidence['H_start']) is int) and (type(evidence['H_end']) is int), 'actual-phase-H-interval')
    _need(_digest(evidence['source_pin'], 40) and _digest(evidence['binary_sha256']) and _digest(evidence['receipt_sha256']), 'phase-source-raw-binding')
    _need(all((type(evidence[key]) is int and evidence[key] == 0 for key in ('frontend_censors', 'maintenance_refusals', 'hidden_retries'))), 'frontend-failure-or-hidden-retry')
    slots = evidence['slots']
    _need(type(slots) is list and len(slots) == 100, 'exact100-independent-logical-slots')
    accepted, losers, all_requests, selected_ids, logical, success_engine, success_spawn = (0, set(), set(), set(), [], [], [])
    for number, slot in enumerate(slots):
        _closed(slot, ('number', 'offered_ns', 'dispatch_ns', 'done_ns', 'helper_ns', 'backpressure_ns', 'offers', 'accepted_selected'), 'logical-slot')
        _need(type(slot['number']) is int and slot['number'] == number, 'slot-order')
        offered = _integer(slot['offered_ns'], 1, 2 ** 63 - 1, 'logical-offer-clock')
        dispatch = _integer(slot['dispatch_ns'], offered, 2 ** 63 - 1, 'actual-dispatch-clock')
        done = _integer(slot['done_ns'], dispatch, 2 ** 63 - 1, 'logical-slot-done')
        for key in ('helper_ns', 'backpressure_ns'):
            _integer(slot[key], 0, done - offered, key)
        if number:
            _need(offered == slots[0]['offered_ns'] + number * phase['slot_interval_ns'], 'matched-fixed-offer-cadence')
        _need(type(slot['offers']) is list and len(slot['offers']) == phase['publishers'], 'exact-bounded-offer-count')
        winners, bases = ([], [])
        for publisher, offer in enumerate(slot['offers']):
            _closed(offer, ('request', 'outcome', 'result', 'selected', 'engine_ns', 'spawn_ns', 'start_ns', 'end_ns', 'raw_sha256', 'error'), 'publish-offer')
            request = offer['request']
            _closed(request, ('request', 'base', 'ops', 'packet'), 'NOOP-request')
            name = 'hx-' + phase['id'] + '-r' + str(number).zfill(3) + '-p' + str(publisher)
            _need(request['request'] == name and name.isascii() and (len(name) <= 64) and (name not in all_requests) and (request['ops'] == []) and (request['packet'] is False), 'exact-offered-authentic-NOOP')
            all_requests.add(name)
            _closed(request['base'], ('publication', 'digest'), 'exact-captured-base')
            _need(_digest(request['base']['digest']), 'base-digest')
            bases.append(request['base'])
            if number:
                _need(request['base'] == slots[number - 1]['accepted_selected'], 'next-captured-accepted-head')
            _need(_digest(offer['raw_sha256']), 'every-offer-raw-retained')
            _integer(offer['start_ns'], dispatch, done, 'native-offer-start')
            _integer(offer['end_ns'], offer['start_ns'], done, 'native-offer-end')
            _integer(offer['engine_ns'], 0, 2 ** 63 - 1, 'publish-engine-ns')
            _integer(offer['spawn_ns'], offer['engine_ns'], 2 ** 63 - 1, 'publish-spawn-ns')
            if offer['outcome'] == 'accepted':
                _need(offer['error'] is None and type(offer['result']) is dict and (offer['result'].get('outcome') == 'accepted') and (offer['result'].get('request') == name) and (offer['selected'] == slot['accepted_selected']) and (offer['result'].get('publication') == offer['selected'].get('publication')), 'exact-accepted-outcome-binding')
                winners.append(offer)
                success_engine.append(offer['engine_ns'])
                success_spawn.append(offer['spawn_ns'])
            else:
                _need(phase['publishers'] == 4 and offer['outcome'] == 'definitive-stale-sealed' and (offer['error'] == 'conflict:expected-publication'), 'only-authentic-sealed-stale-conflict')
                losers.add(name)
        _need(all((base == bases[0] for base in bases)) and len(winners) == 1, 'same-base-round-exactly-one-accepted')
        selected = slot['accepted_selected']
        _closed(selected, ('publication', 'digest'), 'accepted-selected')
        _need(_digest(selected['digest']) and selected['publication'] not in selected_ids, '100-distinct-accepted-publications')
        selected_ids.add(selected['publication'])
        accepted += 1
        logical.append(done - offered)
    receipts = evidence['terminal_receipts']
    _need(type(receipts) is list and len(receipts) == phase['max_sealed_losers'], 'all-sealed-losers-terminal-proof')
    released = set()
    for row in receipts:
        _closed(row, ('request', 'attempt', 'receipt_sha256', 'receipt_bytes', 'retained_charge', 'selection_never_started', 'stale_base_authenticated', 'reclaimed_bytes'), 'terminal-receipt')
        _need(row['request'] in losers and row['request'] not in released and (type(row['attempt']) is str) and (1 <= len(row['attempt']) <= 99) and _digest(row['receipt_sha256']) and (row['selection_never_started'] is True) and (row['stale_base_authenticated'] is True), 'authenticated-definitive-owner-only')
        _integer(row['receipt_bytes'], 1, 8192, 'bounded-fence-receipt')
        _integer(row['retained_charge'], 1, 2 ** 63 - 1, 'retained-terminal-charge')
        _need(type(row['reclaimed_bytes']) is int and row['reclaimed_bytes'] == 0, 'terminal-no-GC-credit')
        released.add(row['request'])
    _need(released == losers and accepted == 100, 'complete100accepts-and-all-losers')
    readers = evidence['readers']
    _closed(readers, ('initial_native_pids', 'calls', 'warm_queries', 'first_queries', 'exhausted', 'ended_before_publishers', 'all_responses_exact', 'raw_lossless', 'rows_sha256', 'old_reader', 'positive_native_overlap', 'reader_rss_max', 'censors'), 'four-readers')
    pids = readers['initial_native_pids']
    _need(type(pids) is list and len(pids) == 4 and (len(set(pids)) == 4) and all((type(pid) is int and pid > 0 for pid in pids)), 'four-distinct-native-readers-not-threads')
    calls = _integer(readers['calls'], 4, 40, 'reader-call-cap')
    _need(type(readers['warm_queries']) is int and readers['warm_queries'] == calls * 1000 and (type(readers['first_queries']) is int) and (readers['first_queries'] == calls), 'all-warm-and-first-queries-counted')
    _need(readers['exhausted'] is False and readers['ended_before_publishers'] is False and (readers['all_responses_exact'] is True) and (readers['raw_lossless'] is True) and (readers['positive_native_overlap'] is True) and _digest(readers['rows_sha256']) and (type(readers['censors']) is int) and (readers['censors'] == 0), 'finite-reader-load-not-qualified')
    _integer(readers['reader_rss_max'], 1, 256 * MIB, 'actual-native-reader-RSS')
    old = readers['old_reader']
    _closed(old, ('pid', 'captured_ns', 'released_ns', 'publishers_progress_during_hold', 'new_readers_progress_during_hold', 'successor_mapping_ids', 'held_generation_peak', 'released_before_publishers_end', 'exact_resume'), 'held-old-reader')
    _need(old['pid'] in pids and type(old['captured_ns']) is int and (type(old['released_ns']) is int) and (old['released_ns'] - old['captured_ns'] > 15000000000) and (old['publishers_progress_during_hold'] is True) and (old['new_readers_progress_during_hold'] is True) and (old['released_before_publishers_end'] is True) and (old['exact_resume'] is True), 'held-old-reader-must-overlap-real-work-more-than15s')
    _integer(old['held_generation_peak'], 1, 60, 'protected-generation-source-bound')
    successors = old['successor_mapping_ids']
    _need(type(successors) is list and (len(set(successors)) >= 2 if phase['online'] else len(successors) == 0), 'online-two-real-successors-control-zero')
    maintenance = evidence['maintenance']
    _closed(maintenance, ('calls', 'builds', 'positive_publisher_overlap', 'actual_mapping_changes', 'zero_forbidden_under_global_exclusion', 'unchanged_historical_body_opens', 'metrics', 'activation_holds_ns', 'online_cleanup_calls', 'boundary_cleanup_calls', 'final_unprotected_backlog', 'raw_sha256'), 'maintenance')
    _need(maintenance['zero_forbidden_under_global_exclusion'] is True and type(maintenance['unchanged_historical_body_opens']) is int and (maintenance['unchanged_historical_body_opens'] == 0) and _digest(maintenance['raw_sha256']), 'no-global-or-hidden-history-scan')
    for key in ('calls', 'builds', 'actual_mapping_changes', 'online_cleanup_calls', 'boundary_cleanup_calls'):
        _integer(maintenance[key], 0, 1000, 'maintenance:' + key)
    if phase['online']:
        _integer(maintenance['final_unprotected_backlog'], 0, 1000, 'measured-boundary-backlog')
        _need(maintenance['positive_publisher_overlap'] is True and 2 <= maintenance['actual_mapping_changes'] <= 100 and (maintenance['builds'] == 100) and (maintenance['online_cleanup_calls'] == 99) and (maintenance['boundary_cleanup_calls'] == 2) and (maintenance['final_unprotected_backlog'] == 0), 'real-online-work-and-two-call-drainage')
        _need(type(maintenance['metrics']) is list and len(maintenance['metrics']) == 100, 'all-online-build-metrics')
        work = _metric_totals(maintenance['metrics'])
        _need(work['rolling_publications_visited'] > 0, 'actual-accepted-tail-discovery')
        activation = _quantiles(maintenance['activation_holds_ns'])
    else:
        _need(maintenance['calls'] == 0 and maintenance['builds'] == 0 and (maintenance['actual_mapping_changes'] == 0) and (maintenance['online_cleanup_calls'] == 0) and (maintenance['boundary_cleanup_calls'] == 0) and (maintenance['metrics'] == []) and (maintenance['activation_holds_ns'] == []) and (maintenance['positive_publisher_overlap'] is False), 'no-concurrent-maintenance-control')
        work, activation = (None, None)
    oracle = evidence['oracle']
    _closed(oracle, ('passed', 'actual_H', 'all_accepted_outcomes', 'all_request_intents', 'source_graph_navigation', 'reverse_selected_cohort', 'full_file_directory_hash_proof', 'quiescent', 'engine_audit_used', 'source_unchanged', 'receipt_sha256'), 'full-v5-oracle')
    _need(all((oracle[key] is True for key in ('passed', 'all_accepted_outcomes', 'all_request_intents', 'source_graph_navigation', 'reverse_selected_cohort', 'full_file_directory_hash_proof', 'quiescent', 'source_unchanged'))) and oracle['actual_H'] == phase['H_end'] and (type(oracle['actual_H']) is int) and (oracle['engine_audit_used'] is False) and _digest(oracle['receipt_sha256']), 'complete-independent-phase-oracle-required')
    _need(evidence['closure'] == dict(processes_closed=True, handles_closed=True, callbacks_joined=True, empty_observations=2, post_close_census=True, scope='owned-children-IO-handles'), 'positive-closure-and-final-census')
    guard = _guard_observation(evidence['guard_observation'], slots)
    _need(guard['source_pin'] == evidence['source_pin'] and guard['binary_sha256'] == evidence['binary_sha256'], 'phase-prefix-source-binding')
    return dict(status='PROVISIONAL_UNQUALIFIED', phase=phase['id'], accepted=accepted, offers=len(all_requests), sealed_losers=len(losers), H_start=phase['H_start'], H_end=phase['H_end'], engine=_quantiles(success_engine), spawn=_quantiles(success_spawn), logical_slot_e2e=_quantiles(logical), helper=_quantiles([slot['helper_ns'] for slot in slots]), backpressure=_quantiles([slot['backpressure_ns'] for slot in slots]), discovery_work=work, activation=activation, receipt_sha256=evidence['receipt_sha256'], p99_qualified=False, outer_QPC_final_two_empty_Job_observations_still_required=True, pending_finalizer=dict(outer_run_id=guard['outer_run_id'], finalizer_sha256=guard['finalizer_sha256'], final_receipt=None))

def compare_history(plan, summaries):
    _need(type(summaries) is list and len(summaries) == 16, 'all16-history-phases-required')
    _need(all((row.get('status') == 'PROVISIONAL_UNQUALIFIED' for row in summaries)), 'post-exit-finalizer-not-this-helper')
    by_id = {row['phase']: row for row in summaries}
    _need(len(by_id) == 16 and set(by_id) == {row['id'] for row in plan['phases']}, 'complete-unique-phase-set')
    _need(sum((row['accepted'] for row in summaries)) == 1600 and sum((row['offers'] for row in summaries)) == 4000 and (sum((row['sealed_losers'] for row in summaries)) == 2400), 'complete-authentic-offer-accounting')
    outer_ids = {row['pending_finalizer']['outer_run_id'] for row in summaries}
    finalizers = {row['pending_finalizer']['finalizer_sha256'] for row in summaries}
    _need(len(outer_ids) == 1 and len(finalizers) == 1 and all((row['pending_finalizer']['final_receipt'] is None for row in summaries)), 'one-real-outer-run-pending-finalizer')
    comparisons = []
    for publishers in (1, 4):
        for cycle in (1, 2, 3):
            small = by_id['h1-p' + str(publishers) + '-online' + str(cycle)]
            large = by_id['h10-p' + str(publishers) + '-online' + str(cycle)]
            ratios = {}
            for key, denominator in small['discovery_work'].items():
                numerator = large['discovery_work'][key]
                if key.startswith('rolling_'):
                    _need(numerator <= 2 * denominator, 'H10-H1-fixed-delta-work-target:' + key)
                ratios[key] = dict(H1=denominator, H10=numerator, ratio=None if denominator == 0 else numerator / denominator)
            comparisons.append(dict(publishers=publishers, cycle=cycle, discovery=ratios))
    latency = []
    for phase in plan['phases']:
        if phase['online']:
            value = by_id[phase['id']]
            control = by_id[phase['donor'].lower() + '-p' + str(phase['publishers']) + '-control']
            dimensions = {}
            for clock in ('engine', 'spawn', 'logical_slot_e2e'):
                reference, observed = (control[clock]['p95'], value[clock]['p95'])
                dimensions[clock] = dict(control_p95=reference, online_p95=observed, ratio=None if reference == 0 else observed / reference, target_passed=observed * 4 <= reference * 5, p99_qualified=False)
            latency.append(dict(phase=phase['id'], clocks=dimensions))
    return dict(status='PROVISIONAL_UNQUALIFIED', actual_accepted=1600, offered=4000, sealed_losers=2400, final_H={identity: donor['actual_H'] + 800 for identity, donor in plan['donors'].items()}, fixed_delta=comparisons, latency=latency, phase_receipts={row['phase']: dict(receipt_sha256=row['receipt_sha256'], pending_finalizer=row['pending_finalizer']) for row in summaries}, performance_passed=all((row['clocks'][clock]['target_passed'] for row in latency for clock in ('engine', 'spawn', 'logical_slot_e2e'))), no_fresh_or_publisher_p99_claim=True)

def _release_conflicts(h, runner, root, requests, responses, candidates):
    proof = runner.release_definitive_conflicts(h, root, requests, responses, candidates)
    _closed(proof, ('api_sha256', 'receipts', 'retained_raw', 'reclaimed_bytes', 'semantic_collect_called', 'global_history_scan', 'active_slots_after'), 'terminal-v2-adapter-proof')
    _need(proof['api_sha256'] == runner.bindings()['terminal_api_sha256'] and proof['retained_raw'] is True and (type(proof['reclaimed_bytes']) is int) and (proof['reclaimed_bytes'] == 0) and (proof['semantic_collect_called'] is False) and (proof['global_history_scan'] is False) and (type(proof['receipts']) is list) and (len(proof['receipts']) == 3), 'exact-reviewed-v2-point-release-only')
    _integer(proof['active_slots_after'], 0, 8, 'v2-active-slots')
    h.note('history-retained-terminal-conflicts', root=str(root), proof=proof)
    return proof

def _publish_round(h, runner, phase, number):
    root = phase['root']
    label = 'hx-' + phase['id'] + '-r' + str(number).zfill(3)
    base = h.selected(root)
    requests = [dict(request=label + '-p' + str(publisher), base=copy.deepcopy(base), ops=[], packet=False) for publisher in range(phase['publishers'])]
    if phase['publishers'] == 1:
        if phase['online'] and number > 0:
            response = h.single_publisher_round(root, label, requests[0]['request'])
        else:
            response = h.publish(root, requests[0]['request'], ops=[], retries=1)
        return dict(requests=requests, responses=[response], terminal=None)
    controls, callbacks = ([], [])
    candidates, responses, primary = (None, None, None)
    try:
        for request in requests:
            h.ledgers[root].record(request)
            control = h.a.control(root, request['request'])
            controls.append(control)
            environment = dict(EXP_BARRIER='candidate-ready', EXP_READY=control['ready'], EXP_RELEASE=control['release'])
            callbacks.append(h.a.spawn(lambda request=request, environment=environment: h.call(root, 'publish', request=request, env=environment, timeout=120, allowed_errors=('conflict:expected-publication',))))
        for control, callback in zip(controls, callbacks):
            h.a.await_ready(control, callback, 10)
        _need(all((h.a.alive(callback) for callback in callbacks)), 'four-sealed-publishers-live')
        candidates = h.a.prepared_candidates(root, requests)
        if phase['online'] and number > 0:
            h.pack(root, cleanup=False)
    except BaseException as error:
        primary = error
    finally:
        responses = h.join_callbacks(root, label, callbacks, controls, primary=primary)
    _need(len(responses) == 4, 'observe-every-publisher-result')
    winners = []
    for request, response in zip(requests, responses):
        if response.get('status') == 'ok':
            value = response['result']
            h.ledgers[root].accept(request, value['result'], value['selected'])
            winners.append(response)
        else:
            _need(response.get('status') == 'error' and response.get('error') == 'conflict:expected-publication', 'unexpected-p4-censor-is-not-a-conflict')
    _need(len(winners) == 1, 'exact-one-of-four-same-base-winners')
    terminal = _release_conflicts(h, runner, root, requests, responses, candidates)
    return dict(requests=requests, responses=responses, terminal=terminal)

def _boundary(h, runner, root, label, expected_H):
    proof = h.verify(root, label)
    _need(proof.get('actual_H') == expected_H and len(h.ledgers[root].accepted) == expected_H, 'full-accepted-H-boundary')
    normalized = proof.get('history_oracle')
    _need(type(normalized) is dict, 'released-v5-normalized-history-oracle-required')
    _need(normalized.get('reverse_selected_cohort') is True and normalized.get('full_file_directory_hash_proof') is True, 'v5R1-and-complete-allpath-proof')
    return normalized

def validate_host_authorization(receipt, final_pin):
    _need(type(receipt) is dict and receipt.get('go') is True and (receipt.get('final_pin') == final_pin) and _digest(final_pin, 40) and _digest(receipt.get('receipt_sha256')), 'positive-same-pin-LaneHost-authorization')
    return receipt

def validate_live_poll(receipt):
    _need(type(receipt) is dict and receipt.get('live') is True and _digest(receipt.get('receipt_sha256')), 'positive-live-poll-required')
    return receipt

def validate_reader_capacity(plan, profile):
    _closed(profile, ('source_pin', 'probe_sha256', 'clock_handshake_sha256', 'method_sha256', 'publisher_horizon_ns', 'reader_deadline_ns', 'old_pin_end_bound_ns', 'header_bytes', 'milestones', 'streams', 'category_map', 'exhaustion', 'warm_horizon_ns'), 'paced-capacity')
    _need(_digest(profile['source_pin'], 40) and all((_digest(profile[k]) for k in ('probe_sha256', 'clock_handshake_sha256', 'method_sha256'))), 'bound-native-pace-source')
    _need(type(profile['warm_horizon_ns']) is int and profile['warm_horizon_ns'] == 45000000000, 'fixed-45s-native-warm-horizon')
    horizon = _integer(profile['publisher_horizon_ns'], 39600000001, 1200000000000, 'publisher-horizon')
    deadline = _integer(profile['reader_deadline_ns'], horizon, 1200000000000, 'absolute-reader-deadline')
    _integer(profile['header_bytes'], 1, 2048, 'exact-encoded-pace-header')
    _integer(profile['milestones'], 1, 10, 'actual-native-milestones')
    _need(profile['exhaustion'] == 'fail-no-renewal-no-skipped-query', 'finite-exhaustion-policy')
    _need(type(profile['streams']) is list and len(profile['streams']) == 4, 'four-paced-native-streams')
    _need(type(profile['category_map']) is list and len(profile['category_map']) == 40 and all((c in CATEGORIES for c in profile['category_map'])), 'frozen-40batch-category-map')
    ids = set()
    for stream in profile['streams']:
        _closed(stream, ('id', 'batches', 'queries_per_batch', 'first_queries', 'offset_ns', 'interval_ns', 'last_native_query_due_ns'), 'paced-stream')
        identity = _integer(stream['id'], 0, 3, 'reader-stream-id')
        _need(identity not in ids, 'distinct-native-reader-streams')
        ids.add(identity)
        _need(type(stream['batches']) is int and stream['batches'] == 10 and (type(stream['queries_per_batch']) is int) and (stream['queries_per_batch'] == 1000) and (type(stream['first_queries']) is int) and (stream['first_queries'] == 10), 'exact40calls40kwarm40first')
        offset = _integer(stream['offset_ns'], 0, deadline, 'native-query-offset')
        interval = _integer(stream['interval_ns'], 1, deadline, 'native-absolute-query-spacing')
        _need((offset, interval) == ((16000000000, 2900000) if identity == 0 else (0, 4500000)), 'fixed-matched-45s-native-schedule')
        last_due = offset + 9999 * interval
        _need(type(stream['last_native_query_due_ns']) is int and stream['last_native_query_due_ns'] == last_due and (horizon < last_due < deadline), 'finite-read-span-covers-publisher-horizon')
        _need(offset >= 16000000000 if identity == 0 else offset == 0, 'old-reader-pre-read-hold')
    end_bound = _integer(profile['old_pin_end_bound_ns'], 16000000001, deadline, 'actual-old-pin-end')
    old_stream = next((stream for stream in profile['streams'] if stream['id'] == 0))
    _need(old_stream['offset_ns'] + 999 * old_stream['interval_ns'] < end_bound and math.ceil(end_bound / 400000000) <= 60, 'pin-IOfinish-not-barrier-release-bound')
    _need(plan['finite_command_bounds']['reader_calls'] == 640 and plan['finite_command_bounds']['reader_warm_queries'] == 640000, 'whole-reader-counts')
    return dict(status='capacity-arithmetic-only-PROVISIONAL_UNQUALIFIED', profile_sha256=_same(profile))

def _guard_observation(guard, slots):
    _closed(guard, ('status', 'guard_sha256', 'outer_run_id', 'host_pid', 'host_creation_id', 'clock_domain', 'phase_start_ns', 'phase_end_ns', 'observed_prefix_sha256', 'owned_drain_sha256', 'finalizer_sha256', 'final_receipt', 'source_pin', 'binary_sha256', 'budget_sha256'), 'provisional-phase-guard')
    _need(guard['status'] == 'PROVISIONAL_UNQUALIFIED' and _digest(guard['guard_sha256']) and (guard['guard_sha256'] != QPC_SHA256) and (guard['final_receipt'] is None), 'no-final-outer-PASS-inside-host')
    for key in ('outer_run_id', 'host_creation_id', 'clock_domain'):
        _need(type(guard[key]) is str and 1 <= len(guard[key]) <= 128, 'authentic-native-identity:' + key)
    _integer(guard['host_pid'], 1, 2 ** 32 - 1, 'native-host-pid')
    start = _integer(guard['phase_start_ns'], 1, slots[0]['dispatch_ns'], 'phase-window-start')
    _integer(guard['phase_end_ns'], max(start, slots[-1]['done_ns']), 2 ** 63 - 1, 'phase-window-end')
    _need(all((_digest(guard[k]) for k in ('observed_prefix_sha256', 'owned_drain_sha256', 'finalizer_sha256', 'binary_sha256', 'budget_sha256'))) and _digest(guard['source_pin'], 40), 'bound-prefix-drain-finalizer-links')
    return guard

def validate_reader_origin(value, phase, selected, bindings):
    _closed(value, ('phase', 'origin_perf_ns', 'origin_qpc', 'qpc_frequency', 'clock_handshake_sha256', 'receipt_sha256', 'markers'), 'native-reader-origin')
    _need(value['phase'] == phase['id'] and value['clock_handshake_sha256'] == bindings['clock_handshake_sha256'] and _digest(value['receipt_sha256']), 'actual-clock-origin-binding')
    origin = _integer(value['origin_qpc'], 1, 2 ** 63 - 1, 'future-raw-QPC-origin')
    _integer(value['qpc_frequency'], 1, 2 ** 63 - 1, 'raw-QPC-frequency')
    _integer(value['origin_perf_ns'], 1, 2 ** 63 - 1, 'bracket-mapped-perf-origin')
    _need(type(value['markers']) is list and len(value['markers']) == 4, 'four-native-ready-markers')
    pids = set()
    physical = set()
    for marker in value['markers']:
        _closed(marker, ('pid', 'creation_id', 'ready_qpc', 'selected', 'physical_generation', 'receipt_sha256'), 'native-ready')
        pid = _integer(marker['pid'], 1, 2 ** 32 - 1, 'ready-native-pid')
        _need(pid not in pids and marker['selected'] == selected and _digest(marker['receipt_sha256']), 'distinct-real-semantic-captures')
        pids.add(pid)
        for key in ('creation_id', 'physical_generation'):
            _need(type(marker[key]) is str and 1 <= len(marker[key]) <= 128, 'ready-native-binding')
        physical.add(marker['physical_generation'])
        _integer(marker['ready_qpc'], 1, origin - 1, 'ready-before-fixed-origin')
    _need(len(physical) == 1, 'same-prewrite-physical-generation')
    return value['origin_perf_ns']

def run_history(host, config):
    _closed(config, ('go', 'plan', 'bindings', 'forecast', 'phase_forecasts', 'runtime_bindings'), 'run-history')
    _need(config['go'] is True, 'default-go-false-separate-runtime-release')
    bindings = config['bindings']
    _need(type(bindings) is dict and _digest(bindings.get('source_pin'), 40), 'parent-authorization-source-pin')
    authorization = validate_host_authorization(host.authorize(config, 'history'), bindings['source_pin'])
    from pathlib import Path
    import runtime_contracts as shared
    rb = config['runtime_bindings']
    _need(type(rb) is dict and rb.get('history') == bindings and (rb.get('runtime_go') is True), 'supplemental-source-bindings')
    spec = rb['runtime_source']
    _need(_digest(spec['sha256']) and spec['sha256'] == bindings['runner_sha256'] and type(spec['bytes']) is int and 0 < spec['bytes'] <= 94208 and (Path(shared.__file__).resolve() == Path(spec['path']).resolve()), 'exact-admitted-R3-runtime-module')
    raw = Path(shared.__file__).read_bytes()
    _need(len(raw) == spec['bytes'] and hashlib.sha256(raw).hexdigest() == spec['sha256'] and (shared.API == 'shared-runtime-contracts-v1'), 'actual036-source-bytes')
    admitted_source = shared.validate_source_admission(rb)
    _need(authorization.get('source_admission') == admitted_source == rb['source_admission'] and authorization.get('source_freeze') == bindings['source_freeze'], 'parent-granted-source-admission-and-freeze')
    bindings = validate_release(bindings, expected_source_budget_sha256=admitted_source['sha256'])
    plan = history_plan(config['plan'])
    forecast = forecast_history(config['forecast'])
    _need(rb['profile_config']['components'] == config['forecast'], 'same-source-profile-components')
    capacity = shared.reader_capacity(rb)
    validate_reader_capacity(plan, capacity)
    model = shared.derive_whole_forecast(plan, rb, config['forecast']['current'])
    expected = dict(F05_bytes=forecast['bytes'], F05_files=forecast['files'], protocol_bytes=forecast['protocol']['additional_bytes'], protocol_files=forecast['protocol']['additional_files'], evidence_bytes=forecast['evidence']['additional_bytes'], evidence_files=forecast['evidence']['additional_files'])
    _need(all((type(model[k]) is int and model[k] == v for k, v in expected.items())) and model['counts'] == plan['finite_command_bounds'], 'concrete035-model-equals-all-reservations')
    source_admission = dict(whole_profile=model, reader_capacity_sha256=_same(capacity))
    _need(type(config['phase_forecasts']) is dict and set(config['phase_forecasts']) == {phase['id'] for phase in plan['phases']} | {'lookup-H1', 'lookup-H10', 'bootstrap-H1-p1', 'bootstrap-H1-p4', 'bootstrap-H10-p1', 'bootstrap-H10-p4'}, 'all-lookup-bootstrap-reader-phase-oracle-finalization-forecasts-required')
    runner = shared.make_runner(host, rb)
    host.history = runner
    _need(runner is not None and getattr(runner, 'api', None) == 'history-native-v1', 'released-native-runner-required')
    _need(runner.bindings() == bindings, 'runner-exact-release-match')
    permit = runner.preflight(plan, forecast, bindings)
    h = host.harness
    _need(bindings['source_pin'] == forecast['source_pin'] == h.final_pin, 'same-final-source-in-all-inputs')
    _need(permit.get('go') is True and permit.get('plan_sha256') == plan['plan_sha256'] and (permit.get('fresh_whole_census') is True) and (permit.get('no_other_semantic_writer') is True) and (permit.get('no_clone') is True) and (permit.get('F05_bytes') == F05_BYTES) and (permit.get('F05_files') == F05_FILES), 'fresh-whole-phase-admission-required')
    _need(permit.get('reader_capacity_sha256') == source_admission['reader_capacity_sha256'] and permit.get('quantitative_profile_sha256') == source_admission['whole_profile']['profile_sha256'], 'runner-must-admit-exact-reader-capacity-and-quantitative-profile')
    h.note('history-plan-admitted', plan=plan, forecast=forecast, bindings=bindings, permit=permit, source_admission=source_admission)
    lookup_results, summaries, phase_seals = ([], [], [])
    for lookup in plan['lookups']:
        donor = plan['donors'][lookup['donor']]
        adopted = runner.adopt(h, donor)
        _need(adopted.get('complete') is True and adopted.get('actual_H') == donor['actual_H'] and (adopted.get('accepted_inputs_sha256') == donor['accepted_inputs_sha256']) and (adopted.get('request_provenance_sha256') == donor['request_provenance_sha256']) and (len(h.ledgers[donor['root']].accepted) == donor['actual_H']), 'complete-authenticated-donor-inputs')
        with h.case('lookup-' + donor['id'], donor['root'], config['phase_forecasts']['lookup-' + donor['id']]):
            _need(h.selected(donor['root']) == donor['selected'], 'exact-selected-before-lookup')
            evidence = runner.lookup(h, donor, lookup)
            _need(evidence.get('source_pin') == h.final_pin and evidence.get('binary_sha256') == bindings['binary_sha256'], 'new-finalcandidate-not-old-baseline')
            lookup_results.append(validate_lookup_evidence(plan, evidence))
            h.note('history-initial-lookups-complete', receipt_sha256=evidence['receipt_sha256'], donor=donor['id'])
            _boundary(h, runner, donor['root'], 'lookup-' + donor['id'] + '-full', donor['actual_H'])
        runner.seal_case('lookup-' + donor['id'])
    for identity, donor in plan['donors'].items():
        root = donor['root']
        for phase in [row for row in plan['phases'] if row['donor'] == identity]:
            if phase['cycle'] == 1:
                label = 'bootstrap-' + identity + '-p' + str(phase['publishers'])
                with h.case(label, root, config['phase_forecasts'][label]):
                    _need(len(h.ledgers[root].accepted) == phase['H_start'], 'actual-control-count-before-bootstrap')
                    selected_before = h.selected(root)
                    h.pack(root, cleanup=False)
                    h.cleanup(root, max_batches=160)
                    _need(h.selected(root) == selected_before, 'bootstrap-physical-only')
                    _boundary(h, runner, root, label + '-reopen-full', phase['H_start'])
                runner.seal_case(label)
            with h.case(phase['id'], root, config['phase_forecasts'][phase['id']]):
                _need(len(h.ledgers[root].accepted) == phase['H_start'], 'no-hidden-pubs-before-phase')
                observations = []
                with runner.phase(h, phase) as lease:
                    origin = lease.tick(dict(stage='reader-origin-before-publish', phase=phase['id']))
                    start = validate_reader_origin(origin, phase, h.selected(root), bindings)
                    _need(time.perf_counter_ns() < start, 'all-four-ready-before-fixed-origin-no-reset')
                    for number in range(100):
                        offered = start + number * phase['slot_interval_ns']
                        lease.tick(dict(stage='before-offer', number=number, offered_ns=offered))
                        while time.perf_counter_ns() < offered:
                            validate_live_poll(host.poll())
                            h.a.sleep(min(0.04, max(0, offered - time.perf_counter_ns()) / 1000000000.0))
                        validate_live_poll(host.poll())
                        result = _publish_round(h, runner, phase, number)
                        if phase['online'] and number > 0:
                            h.cleanup(root, max_batches=1, require_complete=False)
                        observation = dict(number=number, offered_ns=offered, result=result, selected=h.selected(root), state=h.state(root) if phase['online'] else None)
                        observations.append(observation)
                        lease.tick(dict(stage='after-offer', **observation))
                    _need(len(h.ledgers[root].accepted) == phase['H_end'], 'exact100-new-accepted')
                    if phase['online']:
                        h.pack(root, cleanup=False)
                    evidence = lease.finish()
                if phase['online']:
                    drainage = h.drain_two_passes(root)
                    evidence['maintenance']['boundary_cleanup_calls'] = len(drainage)
                    evidence['maintenance']['final_unprotected_backlog'] = drainage[-1]['backlog']['unprotected']
                else:
                    evidence['maintenance']['boundary_cleanup_calls'] = 0
                evidence['oracle'] = _boundary(h, runner, root, phase['id'] + '-full', phase['H_end'])
                closure = host.drain(phase['id'])
                _need(type(closure) is dict and closure.get('children_empty') is True and _digest(closure.get('receipt_sha256')), 'owned-native-IO-handle-drain')
                evidence['closure'] = closure.get('closure')
                _need(evidence['closure'] == dict(processes_closed=True, handles_closed=True, callbacks_joined=True, empty_observations=2, post_close_census=True, scope='owned-children-IO-handles'), 'actual-normalized-owned-closure-required')
                guard = evidence['guard_observation']
                _need(guard.get('owned_drain_sha256') in (None, closure['receipt_sha256']), 'actual-owned-drain-receipt-match')
                guard['owned_drain_sha256'] = closure['receipt_sha256']
                _need(guard.get('budget_sha256') == bindings['budget_sha256'] and guard.get('finalizer_sha256') == bindings['postexit_finalizer_sha256'], 'prefix-budget-and-postexit-finalizer-binding')
                checkpoint = host.checkpoint(phase['id'] + '-closed', config['phase_forecasts'][phase['id']])
                _need(type(checkpoint) is dict and checkpoint.get('go') is True and _digest(checkpoint.get('receipt_sha256')), 'positive-whole-phase-checkpoint')
                _need(evidence.get('source_pin') == h.final_pin and evidence.get('binary_sha256') == bindings['binary_sha256'], 'phase-finalsource-binding')
            sealed = shared.seal_final_provisional(host, phase, evidence, rb)
            actual = shared.sealed_phase_evidence(host, sealed, rb)
            summary = validate_phase_evidence(phase, actual)
            _need(summary['receipt_sha256'] == sealed['body']['sha256'], 'actual-sealed-body-SHA-not-prefix')
            summaries.append(summary)
            phase_seals.append(sealed)
    comparison = compare_history(plan, summaries)
    expanded_trace = runner.seal_trace()
    h.note('history-all1600-provisional', comparison=comparison, initial_lookup_receipts=lookup_results, semantic_write_freeze=True, future_forecast_expiry_requires_actual_quiescent_census=True)
    return dict(comparison=comparison, lookups=lookup_results, phases=summaries, phase_seals=phase_seals, expanded_trace=expanded_trace, qualification='PROVISIONAL_UNQUALIFIED: post-exit receipt join and all other mandatory cells remain required')
