"""Pure in-memory control checks; no files, subprocesses, engines or fixtures.

Run with python -B fake_controls.py. This is harness validation only. It cannot
qualify any engine/native case and never writes a PASSED CaseLedger observation.
"""
import copy
import json
import threading
import tracemalloc
from focused_protocol import (Harness, Inputs, MIB, ORACLE_CHECKS, ThreadCalls,
                              FUNCTIONAL_CUTOFF, intent, relative, require,
                              sha, strict, transition)
from focused_schedules import injected_case
from case_ledger import CaseLedger, PACKING, RECOVERY, STRICT, required_cases

PIN = 'a'*40
ROOT = '/in-memory/disposable/store'
FORECAST = dict(bytes=1024, files=10, mode='memory-only', admitted_native=False)


def default_geometry():
    """Source-derived no-op N64/profile4 geometry; allocates no payload files.

    Exact-length strings come from baseline_model::fixture, navigation::record,
    projection::generation/FileSpec and static_files in the inspected CP2 source.
    Hash values are placeholders of identical encoded length. This is not a
    final-pin fixture census; candidate/COW bounds below are separate ceilings.
    """
    quote = lambda value: json.dumps(value, ensure_ascii=False, separators=(',', ':'))
    special = ['Overview', 'Requirement α', 'Decision café', 'CON', 'con.', 'AUX ',
               'Case', 'case', 'é', 'e\u0301']
    description = ('version: 4\nlayout: stable-id-sha256-binary-prefix\ngroups: navigation/groups/\n'
                   'relationships: navigation/relationships/\nmaximum_group_records: 512\n'
                   'maximum_part_bytes: 262144\nsource_complete_kind: self-describing-json\n'
                   'source_complete_rule: exact id, title, kind and ordered relationships in source JSON; '
                   'location is its ordinary tree path\n')
    provenance = ('{"requested":false,"recipe":"not-requested","renderer":"exact-concat-v1",'
                  '"inputs":[],"omissions":["no new context packet requested"],"errors":[],"output_sha256":'
                  '"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"}')
    body = 'version: 4\nstable_id_sha256_prefix: ""\ndocuments:\n'
    manifest, payload = {}, 0
    digest = 'f'*64
    for i in range(64):
        doc_id = f'd{i:06}'
        title = special[i] if i < 10 else f'project source {i}'
        ext = 'yaml' if i % 10 in (0, 1) else 'json' if i % 10 == 2 else 'txt' if i % 10 == 9 else 'md'
        slug = ''.join(c.lower() if c.isascii() and c.isalnum() else '-' for c in title)[:38]
        path = f'docs/item-{slug}-{doc_id}.{ext}'
        kind = 'overview' if i % 10 == 0 else 'requirement' if i % 10 == 1 else 'exact-json' if i % 10 == 2 else 'decision-source'
        if i == 9:
            path = f'docs/item-attachments-{doc_id}/evidence/deep/source.txt'
            kind = 'attachment'
        size = 4096 if i % 10 <= 6 else 16384 if i % 10 in (7, 8) else 65536
        payload += size
        manifest[path] = dict(source='originals/ff/'+digest+'-'+path.rsplit('/', 1)[1], digest=digest)
        body += f'  - id: {quote(doc_id)}\n    title: {quote(title)}\n    kind: {quote(kind)}\n    source: {quote(path)}\n'
        edges = ([('requires', 'd000001', False), ('attachment', 'd000009', False)] if i == 0 else
                 [('cites', 'd000002', False)] if i == 1 else
                 [('cites', 'https://unavailable.invalid/source', True)] if i == 2 else [])
        body += '    relationships:\n' if edges else '    relationships: []\n'
        for ordinal, (edge_kind, target, external) in enumerate(edges):
            body += (f'      - ordinal: {ordinal}\n        kind: {quote(edge_kind)}\n'
                     f'        target: {quote(target)}\n        external: {str(external).lower()}\n')
    require(len(body.encode()) <= 256*1024 and len(manifest) == 64, 'one-profile4-root-group')
    for path in ('navigation/groups/root.yaml', 'navigation.yaml', 'packet/context.md', 'packet/provenance.json'):
        manifest[path] = dict(source='derived/ff/'+digest, digest=digest)
        if path == 'packet/provenance.json':
            manifest[path]['projection_profile'] = 4
    manifest_bytes = len(json.dumps(manifest, sort_keys=True, indent=2, ensure_ascii=False).encode())
    supplement = len(body.encode())+len(description.encode())+len(provenance.encode())+manifest_bytes
    # Bounds for FAILED NO-OP preparation only: unchanged graph/projection/change
    # roots, no new COW pages, two static replay-file paths; safe_id<100,
    # request<=256. 16KiB candidate +1KiB retained reservation is conservative.
    # Accepted metadata upper additionally covers permanent records and two
    # compact COW trees at H<=114 (height1, leaf64, fanout32; no history rewrite).
    failed_external = 17*1024
    accepted_external = 128*1024
    generations, failed, accepted = 162, 48, 114
    per_generation = payload+supplement
    total = payload+generations*per_generation+failed*failed_external+accepted*accepted_external
    return dict(profile='inspected-CP2-project-N64-profile4-default-payload-no-op',
                schedule=[1, 4, 1], actual_final_H=accepted, failed_prepared_generations=failed,
                total_generation_copies=generations, source_payload_bytes=payload,
                group_bytes=len(body.encode()), description_bytes=len(description.encode()),
                provenance_bytes=len(provenance.encode()), manifest_bytes=manifest_bytes,
                generation_regular_files=len(manifest)+1, generation_supplement_bytes=supplement,
                per_generation_bytes=per_generation,
                failed_noop_external_metadata_upper=failed_external,
                accepted_external_metadata_upper=accepted_external,
                B_payload_only_floor=payload*(generations+1),
                B_generation_plus_external_upper=total,
                qualification='source-derived conditional bound; final source/IDs/no-op replay must be rebound; '
                              'init graph/projection objects, physical metadata, raw receipts, failed process prefixes '
                              'and any failed-round-collection overhead remain separately charged')


class FakeAdapter(ThreadCalls):
    """Models observations only. No fake engine implementation or filesystem."""
    capabilities = set()
    def __init__(self):
        self.now = FUNCTIONAL_CUTOFF-1000
        self.permit = True
        self.closed = True
        self.calls = []
        self.pending = 0
        self.role_limits = (256*MIB, 2048*MIB)
        self.quiesced = False

    def wall_time(self):
        return self.now

    def preflight(self, case_id, root, forecast, final_pin):
        return dict(go=self.permit, case_id=case_id, root=str(root), final_pin=final_pin,
                    reader_limit=self.role_limits[0], checker_limit=self.role_limits[1],
                    default_payloads=True, heavy_token='MEMORY-ONLY', host_guard='MEMORY-ONLY',
                    binary_sha256='b'*64, binary_source_pin=final_pin,
                    source_manifest_sha256='c'*64, adapter_sha256='d'*64, guard_sha256='e'*64)

    def close_case(self, case_id, root):
        return dict(processes_closed=self.closed, handles_closed=self.closed,
                    python_callbacks_joined=self.closed, qualification='no-native-processes')

    def before_call(self, root, action, role, timeout):
        self.calls.append((root, action, role, timeout))

    def backlog(self, root):
        return dict(unprotected=self.pending, protected=0, excluded=0)

    def quiesce(self, root):
        self.quiesced = True


class FakeDriver:
    binary_sha256 = 'b'*64
    def __init__(self):
        self.invocations = []
        self.conflict = False
        self.violation = False

    def call(self, root, action, *, timeout=120, code=0, env=None, **fields):
        self.invocations.append((root, action, dict(timeout=timeout, code=code, env=copy.deepcopy(env),
                                                  **copy.deepcopy(fields))))
        if self.conflict:
            response = dict(status='error', error='conflict:captured-base', result=None)
            payload = dict(root=str(root), action=action, **copy.deepcopy(fields))
            raise AssertionError(dict(input=payload, exit=2, violations=['rss-censor'] if self.violation else [],
                                      env=copy.deepcopy(env), executable_sha256=self.binary_sha256,
                                      stdout=json.dumps(response)))
        return dict(status='ok', error=None, result=dict(status='complete', candidates=1, batch_ns=100),
                    metrics=dict(physical_activation_holds_ns=[], physical_work_holds_ns=[]))


class RoundAdapter(FakeAdapter):
    capabilities = {'barrier:candidate-ready', 'barrier:pack-catalog-captured'}
    def __init__(self):
        super().__init__()
        self.controls = {}

    def begin_phase(self, root, label):
        self.phase = label

    def control(self, root, name):
        value = dict(ready=name+'/ready', release=name+'/release', cancel=name+'/cancel')
        self.controls[value['ready']] = (threading.Event(), threading.Event())
        return value

    def await_ready(self, control, handle, timeout):
        require(self.controls[control['ready']][0].wait(timeout), 'fake-ready-timeout')
        require(self.alive(handle), 'fake-process-ended-before-ready')

    def release(self, control):
        self.controls[control['ready']][1].set()

    def checkpoint(self, env):
        ready, release = self.controls[env['EXP_READY']]
        ready.set()
        require(release.wait(10), 'fake-release-timeout')

    def overlap(self, root, label, **counts):
        return dict(positive_native_intervals=True, independent_reader_processes=counts['readers'],
                    publisher_processes=counts['publishers'], maintenance_overlap=True,
                    qualification='FAKE ONLY: not actual native overlap')


class RoundDriver(FakeDriver):
    def __init__(self, adapter):
        super().__init__()
        self.adapter = adapter
        self.head = dict(publication='initial-publication', digest='0'*64)
        self.initial = dict(outcome='accepted', request='initial', publication='initial-publication')
        self.lock = threading.Lock()

    def call(self, root, action, *, timeout=120, code=0, env=None, **fields):
        self.invocations.append((root, action, dict(timeout=timeout, code=code, env=copy.deepcopy(env),
                                                  **copy.deepcopy(fields))))
        self.adapter.checkpoint(env)
        if action == 'resolve':
            return dict(status='ok', result=self.initial, error=None)
        require(action == 'publish', 'fake-only-publish-resolve')
        request = fields['request']
        with self.lock:
            if request['base'] == self.head:
                self.head = dict(publication=request['request'], digest=sha(request['request'].encode()))
                result = dict(outcome='accepted', request=request['request'], publication=self.head['publication'])
                return dict(status='ok', result=dict(result=result, selected=copy.deepcopy(self.head)), error=None)
        response = dict(status='error', result=None, error='conflict:expected-publication')
        payload = dict(root=str(root), action=action, **copy.deepcopy(fields))
        raise AssertionError(dict(input=payload, exit=2, violations=[], env=copy.deepcopy(env),
                                  executable_sha256=self.binary_sha256, stdout=json.dumps(response)))


class RoundHarness(Harness):
    def selected(self, root):
        return copy.deepcopy(self.driver.head)

    def pack(self, root, codec='zstd', **fields):
        self.note('FAKE-maintenance-only', codec=codec)

    def drain_two_passes(self, root):
        self.note('FAKE-drainage-only')


def full_oracle(**fields):
    return dict(status='passed', final_pin=fields['final_pin'], engine_code_imported=False,
                engine_audit_used=False, checks=list(ORACLE_CHECKS),
                actual_H=len(fields['known_requests']), mapped_keys=fields['required_mapped_keys'],
                receipt_sha256='f'*64, source_unchanged=True,
                qualification='MEMORY-ONLY CALLBACK CONTRACT; NO ENGINE PROOF')


def run():
    tracemalloc.start()
    results = []
    def passed(name):
        results.append(dict(control=name, status='passed-memory-only'))
    def refuses(name, callback):
        try:
            callback()
        except (RuntimeError, ValueError, AssertionError, TypeError):
            passed(name)
        else:
            raise RuntimeError('negative-control-did-not-refuse:'+name)

    refuses('duplicate-top-level-json', lambda: strict(b'{"x":1,"x":2}', 100))
    refuses('duplicate-nested-json', lambda: strict(b'{"x":{"a":1,"a":2}}', 100))
    refuses('nonfinite-json', lambda: strict(b'{"x":NaN}', 100))
    refuses('overbound-json', lambda: strict(b'{}', 1))
    for path in ('/absolute', '../parent', 'a/../b', 'a//b', 'C:/foreign', 'a\\b'):
        refuses('unsafe-path-'+path, lambda p=path: relative(p))
    require(relative('physical-pins/a.lock') == 'physical-pins/a.lock', 'durable-pin-relative-path')
    passed('durable-pin-path-accepted')
    base1, base2 = dict(publication='p1', digest='1'*64), dict(publication='p2', digest='2'*64)
    first = dict(request='same-id', base=base1, ops=[], packet=False)
    second = dict(request='same-id', base=base2, ops=[], packet=False)
    inputs = Inputs()
    inputs.record(first)
    inputs.record(second)
    require(len(inputs.attempts) == 2 and intent(first) != intent(second), 'concurrent-input-intents-collapsed')
    accepted = dict(outcome='accepted', request='same-id', publication='p2')
    inputs.accept(second, accepted, base2)
    require(inputs.oracle_inputs()['same-id'] == second, 'wrong-accepted-input')
    passed('distinct-inputs-and-unique-accepted-intent')
    refuses('accepted-intent-overwrite', lambda: inputs.accept(first,
            dict(outcome='accepted', request='same-id', publication='p1'), base1))
    refuses('bool-u64-op', lambda: intent(dict(request='x', base=None,
            ops=[dict(kind='cost', id='d', value=True)], packet=False)))

    ledger = CaseLedger('Linux', PIN)
    require(len(PACKING) == 52 and len(RECOVERY) == 24 and len(STRICT) == 24, 'full-donor-case-counts')
    require(ledger.result()['status'] == 'INCOMPLETE' and
            all(row['status'] == 'PENDING' for row in ledger.rows.values()), 'initial-pending-ledger')
    passed('all-required-cases-initially-pending')
    refuses('cannot-pass-without-native-raw-oracle', lambda: ledger.record('S-A-zstd',
            dict(case_id='S-A-zstd', status='PASSED', host='Linux', final_pin=PIN)))
    ledger.record('S-A-zstd', dict(case_id='S-A-zstd', status='FAILED', reason='retained fake failure prefix'))
    ledger.record('S-A-zstd', dict(case_id='S-A-zstd', status='PENDING', reason='rerun not executed'))
    require(len(ledger.observations) == 2 and ledger.observations[0]['status'] == 'FAILED', 'failed-prefix-erased')
    passed('failed-prefix-preserved')
    windows = required_cases('Windows')
    require('X-Windows1000' in windows and 'P/windows-loose-no-delete-sharing-A' in windows
            and 'X-Windows1000' not in required_cases('Mac'), 'native-applicability-map')
    passed('Windows1000-and-native-applicability-distinct')

    state1 = dict(store=dict(physical=dict(id='a', ordinal=1)), catalog=dict(packs=[dict(pack='old')]))
    state2 = copy.deepcopy(state1)
    state2['store']['physical'] = dict(id='b', ordinal=2)
    refuses('ID-ordinal-only-is-not-real-mapping-change', lambda: transition(state1, state2))
    state2['catalog']['packs'] = [dict(pack='new')]
    transition(state1, state2)
    passed('new-descriptor-mapping-activation-accepted')

    a, d, observations = FakeAdapter(), FakeDriver(), []
    h = Harness(d, a, full_oracle, observations.append, final_pin=PIN)
    a.permit = False
    def no_go():
        with h.case('negative-budget', ROOT, FORECAST):
            h.call(ROOT, 'inspect')
    refuses('case-budget-refuses-before-dispatch', no_go)
    require(not d.invocations, 'engine-dispatched-without-case-permit')
    a.permit = True
    a.role_limits = (2048*MIB, 2048*MIB)
    refuses('reader-cannot-inherit-checker-limit', no_go)
    require(not d.invocations, 'engine-dispatched-with-wrong-reader-limit')
    a.role_limits = (256*MIB, 2048*MIB)
    a.now = FUNCTIONAL_CUTOFF
    refuses('global-cutoff-refuses-before-dispatch', no_go)
    a.now -= 100
    with h.case('memory-positive', ROOT, FORECAST):
        h.call(ROOT, 'resolve', request_id='x')
        h.call(ROOT, 'packing', operation='status')
        require(a.calls[-2][2] == 'reader' and a.calls[-1][2] == 'checker', 'action-role-not-basename')
        passed('reader-checker-action-roles')
        d.invocations.clear()
        h.drain_two_passes(ROOT)
        require(len(d.invocations) == 2, 'two-passes-were-hidden-batch-loop')
        passed('drainage-exactly-two-cleanup-invocations')
        a.pending = 1
        refuses('drainage-pending-backlog-refuses', lambda: h.drain_two_passes(ROOT))
        a.pending = 0
        d.conflict = True
        response = h.call(ROOT, 'publish', request=first,
                          allowed_errors=('conflict:captured-base',))
        require(response['error'] == 'conflict:captured-base', 'exact-semantic-row-recovery')
        passed('exact-percall-semantic-conflict-row')
        d.violation = True
        refuses('rss-censor-not-treated-as-semantic-conflict', lambda: h.call(ROOT, 'publish', request=first,
                allowed_errors=('conflict:captured-base',)))
        d.conflict = d.violation = False
        h.verify(ROOT, 'fake-contract-only')
        passed('complete-oracle-callback-contract')
        h.oracle = lambda **fields: dict(status='passed', final_pin=PIN, checks=['engine-audit'])
        refuses('engine-audit-cannot-substitute-full-oracle', lambda: h.verify(ROOT, 'negative'))
    require(any(x['kind'] == 'schedule-complete-unqualified' for x in observations), 'source-only-schedule-status')
    require(not any(x.get('status') == 'PASSED' for x in ledger.observations), 'fake-qualified-engine-case')
    passed('memory-results-never-qualify-engine')
    refuses('unsupported-paged-hook-pending', lambda: injected_case(h, ROOT, FORECAST,
            dict(case_id='CP3-touched-route-leaf-raw', capability='paged-corruption',
                 source_sha256='a'*64, schema_pin=PIN, required_checks=['required-route-refusal'])))
    a.closed = False
    def missing_closure():
        with h.case('negative-closure', ROOT, FORECAST):
            pass
    refuses('positive-process-handle-closure-required', missing_closure)
    a.closed = True
    outcome = a.spawn(lambda: 42)
    require(a.wait(outcome, 2) == 42 and not a.alive(outcome), 'threaded-driver-call-contract')
    passed('thread-join-surface-control')
    rounds, round_events = RoundAdapter(), []
    round_driver = RoundDriver(rounds)
    round_harness = RoundHarness(round_driver, rounds, full_oracle, round_events.append, final_pin=PIN)
    with round_harness.case('fake-bounded-four-publishers', ROOT, FORECAST):
        round_harness.ledgers[ROOT].accept(dict(request='initial', base=None, ops=[], packet=False),
                                         round_driver.initial, round_driver.head)
        round_harness.concurrent_cycle(ROOT, 'memory-round', 4)
    publish_calls = [row for row in round_driver.invocations if row[1] == 'publish']
    require(len(publish_calls) == 64 and len(round_harness.ledgers[ROOT].accepted) == 17,
            'bounded-four-publisher-attempts-or-new-accepted-count')
    require(sum(row['kind'] == 'semantic-conflict' for row in round_events) == 48,
            'bounded-round-conflict-count')
    require(len(round_harness.ledgers[ROOT].attempts) == 65, 'attempt-inputs-collapsed')
    passed('four-paused-candidates-64attempts-16accepted-48conflicts-memory-trace')
    geometry = default_geometry()
    require(geometry['B_payload_only_floor'] == 126853120 and geometry['generation_regular_files'] == 69,
            'default-profile-1-4-1-geometry')
    passed('default-N64-B-geometry-and-bounded-1-4-1-schedule')
    _, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()
    require(peak < 8*MIB, 'fake-controls-python-allocation-bound')
    return dict(status='PASSED_MEMORY_ONLY_CONTROLS', controls=len(results), checks=results,
                tracemalloc_peak_bytes=peak, python_allocation_limit_bytes=8*MIB,
                native_RSS_qualification=False, engine_calls=0, fixture_writes=0,
                filesystem_writes=0, bytecode_writes=False,
                required_engine_cases_still_pending=True,
                qualification='syntax and pure fake adapter controls only; not native or engine evidence')


if __name__ == '__main__':
    print(json.dumps(run(), indent=2))
