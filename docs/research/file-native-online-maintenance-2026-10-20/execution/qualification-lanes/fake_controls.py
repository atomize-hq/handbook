"""Pure fake controls; defining/importing these functions never runs a control.

Parent must review all source bytes and authorize a finite run separately.
No engine, subprocess, file fixture, corpus copy, restore or budget release exists.
These tests cannot qualify archive writes, native gates, source freeze or RSS.
"""
import copy
import gzip
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import struct


def need(ok, text):
    if not ok:
        raise RuntimeError(text)


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def source(binding, name):
    need(binding.get('reviewed') is True and binding.get('sha256') and
         Path(binding['path']).is_absolute(), 'reviewed-fake-source')
    raw = Path(binding['path']).read_bytes()
    need(len(raw) == binding['bytes'] and sha(raw) == binding['sha256'],
         'fake-source-byte-binding')
    spec = importlib.util.spec_from_file_location(name, binding['path'])
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def rejects(call):
    try:
        call()
    except (RuntimeError, ValueError, AssertionError, UnicodeError, EOFError, KeyError, TypeError):
        return True
    return False


class FakeHost:
    """No real invocation, polling, evidence storage or authorization."""
    def __init__(self):
        self.events = []

    def authorize(self, *args):
        self.events.append('authorize')
        raise RuntimeError('fake-host-refuses-runtime')

    def poll(self):
        self.events.append('poll')
        return dict(live=False)

    def checkpoint(self, *args):
        self.events.append('checkpoint')
        return dict(go=False)

    def invoke(self, *args):
        self.events.append('invoke')
        raise RuntimeError('fake-host-has-no-native-runner')

    def record(self, *args, **kwargs):
        self.events.append('record')
        raise RuntimeError('fake-host-has-no-artifact-writer')

    def drain(self, *args):
        self.events.append('drain')
        return dict(children_empty=False)


class GzipSession:
    def __init__(self):
        self.c = dict(max_compressed_bytes=4096, max_source_bytes=4096, max_files=8)
        self.polls = 0

    def tick(self):
        self.polls += 1


def drain(reader, size):
    result = bytearray()
    while len(result) < size:
        result.extend(reader.take(min(65536, size-len(result))))
    reader.finish()
    return bytes(result)


def old_inventory(legacy, paths=None):
    paths = paths or [('view/navigation/groups/root.yaml', b'root\n'),
                      ('view/navigation.yaml', b'navigation\n')]
    rows = [dict(path=name, bytes=len(data), sha256=sha(data)) for name, data in paths]
    lines = [legacy.encoded(row) for row in rows]
    manifest = dict(version=1, profile='quiescent-full-operational', files=len(rows),
                    logical_bytes=sum(r['bytes'] for r in rows),
                    inventory_sha256=sha(b''.join(lines)), transport_sha256='a'*64)
    return legacy.encoded(manifest), lines


def expected_samples(long):
    expected = {k: dict(outcome='missing', request=v) for k, v in long.REQUESTS}
    rows, warm = [], []
    for category, request in long.REQUESTS:
        for sample in range(50):
            rows.append(dict(category=category, id=request, sample=sample,
                             result=expected[category], engine_ns=10, spawn_total_ns=20,
                             raw=dict(sha256='a'*64)))
        warm.append(dict(category=category, id=request, first_lookup_ns=10,
                         warm_ns=[10]*500, warm_median_ns=10, warm_p95_ns=10,
                         warm_min_ns=10, warm_max_ns=10, opens=0, pack_storage_bytes=0,
                         decoded_record_bytes=0, index_cache_hits=0, result_digest=sha(json.dumps(expected[category], sort_keys=True,
                                            separators=(',', ':'), ensure_ascii=False).encode())))
    return rows, dict(rows=warm), expected


def forecast_fixture():
    limits = dict(cleanup_batches=128, sample_bytes=1024**2, cleanup_receipt_bytes=131072,
                  result_bytes=131072, evidence_bytes=52*1024**2, evidence_files=4096)
    profiles = {k:dict(input=4096, stdout=16384, stderr=2001, receipt=4096,
                      output=524288 if k == 'oracle' else 0)
                for k in ('resolve','lookup-bench','setup','convert','publish','cleanup','oracle')}
    profiles['support'] = dict(bytes=1024**2, files=32)
    return dict(limits=limits), profiles


def execute_pure(core, legacy, long, history=None, history_fixtures=None):
    """Returns raw case outcomes; caller must persist a bound receipt via LaneHost."""
    outcomes = []
    def case(name, call, expect_rejection=False):
        try:
            result = rejects(call) if expect_rejection else call()
            need(result is not False, 'unexpected-control-outcome:' + name)
            outcomes.append(dict(id=name, status='passed', expected_rejection=expect_rejection))
        except BaseException as error:
            outcomes.append(dict(id=name, status='FAILED', error=ascii(error)[:600]))
    host = FakeHost()
    case('pending-long-config-refuses-before-host',
         lambda: long.run_long(host, dict(schema='private-long-lane-v1', go=False)), True)
    case('no-host-call-before-release-validation', lambda: host.events == [])
    case('negative-live-poll-refuses', lambda: long.poll(host), True)
    case('duplicate-json-key', lambda: legacy.strict(b'{"a":1,"a":2}', 100), True)
    case('nonfinite-json', lambda: legacy.strict(b'{"a":NaN}', 100), True)
    case('bounded-json', lambda: legacy.strict(b'{}', 1), True)
    for name in ('../escape', '/absolute', 'a\\b', 'a//b', 'CON', 'a.'):
        case('path-refusal:' + name, lambda name=name: core.path_name(name), True)
    case('path-positive', lambda: core.path_name('physical-pins/generation.lock') ==
         'physical-pins/generation.lock')
    m, lines = old_inventory(legacy)
    parsed = legacy.parse_inventory(m, lines, core)
    case('legacy-DFS-order-preserved', lambda:
         [r['path'] for r in parsed['ordered_files']] ==
         ['view/navigation/groups/root.yaml', 'view/navigation.yaml'])
    case('legacy-postinventory-global-sorted', lambda:
         [r['path'] for r in parsed['canonical']['files']] ==
         ['view/navigation.yaml', 'view/navigation/groups/root.yaml'])
    bad = list(lines); bad[-1] = lines[0]
    case('legacy-duplicate-row', lambda: legacy.parse_inventory(m, bad, core), True)
    m2, lines2 = old_inventory(legacy, [('Foo/a', b'a'), ('foo/b', b'b')])
    case('legacy-directory-case-collision', lambda: legacy.parse_inventory(m2, lines2, core), True)
    m2, lines2 = old_inventory(legacy, [('a', b'a'), ('a/b', b'b')])
    case('legacy-file-directory-collision', lambda: legacy.parse_inventory(m2, lines2, core), True)
    m2, lines2 = old_inventory(legacy, [('writer.lock', b'a')])
    case('legacy-excluded-root-file', lambda: legacy.parse_inventory(m2, lines2, core), True)
    m2, lines2 = old_inventory(legacy, [('physical-pins/a.lock', b'a')])
    case('durable-pin-lock-is-data', lambda: legacy.parse_inventory(m2, lines2, core)['ordered_files'][0]['path']
         == 'physical-pins/a.lock')
    frame = b'FNOP1\n' + struct.pack('<IQ', 1, 3) + b'x' + b'abc' + hashlib.sha256(b'abc').digest()
    packed = gzip.compress(frame, mtime=0)
    def gz(raw, expected=frame):
        session = GzipSession()
        reader = core.StrictGzipReader(io.BytesIO(raw), session)
        result = drain(reader, len(expected))
        need(result == expected and session.polls > 0, 'strict-gzip-positive-and-poll')
        need(reader.sha.hexdigest() == sha(raw), 'compressed-sha')
        return True
    case('single-member-gzip-positive', lambda: gz(packed))
    case('truncated-gzip', lambda: gz(packed[:-1]), True)
    damaged = bytearray(packed); damaged[-8] ^= 1
    case('CRC-gzip', lambda: gz(bytes(damaged)), True)
    case('concatenated-gzip', lambda: gz(packed+packed), True)
    case('padding-gzip', lambda: gz(packed+b'\0'), True)
    case('trailing-gzip', lambda: gz(packed+b'x'), True)
    case('gzip-take-over64KiB',
         lambda: core.StrictGzipReader(io.BytesIO(packed), GzipSession()).take(65537), True)
    rows, warm, expected = expected_samples(long)
    case('complete300fresh3000warm-positive', lambda: bool(long.validate_samples(rows, warm, expected)))
    case('missing-fresh-sample', lambda: long.validate_samples(rows[:-1], warm, expected), True)
    bad = copy.deepcopy(rows); bad[1]['sample'] = 0
    case('duplicate-fresh-index', lambda: long.validate_samples(bad, warm, expected), True)
    bad = copy.deepcopy(rows); bad[0]['result'] = {}
    case('wrong-fresh-outcome', lambda: long.validate_samples(bad, warm, expected), True)
    bad = copy.deepcopy(rows); bad[0]['spawn_total_ns'] = 1
    case('spawn-engine-clocks-reversed', lambda: long.validate_samples(bad, warm, expected), True)
    bad = copy.deepcopy(warm); bad['rows'][0]['warm_ns'] = [1]*499
    case('missing-warm-sample', lambda: long.validate_samples(rows, bad, expected), True)
    bad = copy.deepcopy(warm); bad['rows'][0]['result_digest'] = 'b'*64
    case('wrong-warm-outcome-digest', lambda: long.validate_samples(rows, bad, expected), True)
    bad = copy.deepcopy(warm); del bad['rows'][0]['id']
    case('missing-warm-id', lambda: long.validate_samples(rows, bad, expected), True)
    bad = copy.deepcopy(warm); bad['rows'][0]['id'] = 'another-missing-id'
    case('wrong-warm-id-same-outcome', lambda: long.validate_samples(rows, bad, expected), True)
    summary = {k: dict(engine_median_ns=100, spawn_median_ns=200, warm_median_ns=100)
               for k, _ in long.REQUESTS}
    case('matched-engine-spawn-improvement-separated', lambda:
         long.compare_lookup(dict(summary=summary), dict(summary=summary))['early']['fresh_engine_speedup'] == 1)
    case('incomplete-independent-oracle', lambda: long.oracle_summary(dict(status='passed'), 101007), True)
    case('wrong-exact-long-H', lambda: long.oracle_summary(dict(status='passed', actual_H=101006), 101007), True)
    case('bare-pass-no-raw-reference', lambda: long.reference(dict(status='passed'), 'fake'), True)
    fc, profiles = forecast_fixture()
    case('source-schedule-five-oracles-606plusK', lambda:
         legacy.forecast_long(fc, profiles)['engine_calls'] == 734 and
         legacy.forecast_long(fc, profiles)['full_oracles'] == 5)
    bad = copy.deepcopy(fc); bad['limits']['evidence_bytes'] = 1
    case('whole-evidence-underallocated', lambda: legacy.forecast_long(bad, profiles), True)
    badp = copy.deepcopy(profiles); badp['oracle']['output'] = 2*1024**2
    case('five-oracles-over-M8-before-restore', lambda: legacy.forecast_long(fc, badp), True)
    case('pending-profiles-before-restore', lambda: legacy.forecast_long(fc, None), True)
    if history is not None:
        case('history-negative-GO-with-valid-receipt', lambda:
             history.validate_host_authorization(dict(go=False, final_pin='a'*40,
                  receipt_sha256='b'*64), 'a'*40), True)
        case('history-negative-live-with-valid-receipt', lambda:
             history.validate_live_poll(dict(live=False, receipt_sha256='b'*64)), True)
        need(type(history_fixtures) is dict and len(history_fixtures) <= 2,
             'maximum-two-reviewed-history-pure-fixtures')
        for name, fixture in history_fixtures.items():
            need(name in ('history_plan', 'validate_phase_evidence', 'forecast_history', 'validate_release'),
                 'closed-pure-history-hook-set')
            args = fixture['args']
            case('history:' + name, lambda name=name, args=args: bool(getattr(history, name)(*args)),
                 fixture.get('reject', False))
    return outcomes


def run_controls(host, config):
    """This function requires explicit fake-only runtime admission, separate from source GO."""
    need(config.get('go') is True and config.get('runtime_go') is True and
         config.get('kind') == 'finite-qualification-lanes-pure-controls-v1',
         'source-only-no-control-RUN-GO')
    permit = host.authorize(config, 'qualification-lanes-pure-controls')
    need(permit.get('go') is True and len(permit.get('receipt_sha256', '')) == 64,
         'positive-finite-control-admission')
    modules = {name: source(config['bindings'][name], 'controls_' + name)
               for name in ('stream', 'legacy_adapter', 'long_source', 'history_source')}
    outcomes = execute_pure(modules['stream'], modules['legacy_adapter'], modules['long_source'],
                            modules['history_source'], config.get('history_fixtures', {}))
    need(len(outcomes) <= config['max_cases'] <= 48, 'finite-case-cap')
    raw = (json.dumps(dict(status='passed' if all(x['status'] == 'passed' for x in outcomes) else 'FAILED',
            cases=outcomes, source_bindings=config['bindings'], real_engine_calls=0,
            archive_writer_tested=False, native_containment_tested=False,
            runtime_qualification=False), sort_keys=True, separators=(',', ':'))+'\n').encode()
    need(len(raw) <= config['max_output_bytes'] <= 32768, 'bounded-controls-output')
    result = host.record(config['output'], raw, config['category'], config['max_output_bytes'])
    need(result.get('sha256') == sha(raw) and result.get('bytes') == len(raw),
         'controls-raw-receipt')
    need(all(x['status'] == 'passed' for x in outcomes), 'controls-failed-see-raw-receipt')
    return result

