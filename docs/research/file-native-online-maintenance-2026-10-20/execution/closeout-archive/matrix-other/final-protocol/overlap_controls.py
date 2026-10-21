"""MEMORY ONLY v3 transport/overlap/exception controls. No native or disk fixture."""
import copy
import json
import threading
import time
import tracemalloc
from focused_protocol import Harness, MIB, Unsupported, require, sha
from fake_controls import PIN, ROOT, FORECAST, FakeDriver, RoundHarness, full_oracle
from remediation_controls import FaultAdapter


class Adapter(FaultAdapter):
    capabilities = FaultAdapter.capabilities | {'barrier:physical-before-selection'}
    def __init__(self, fault='', position=1, mutation=None, actor_order=None):
        super().__init__(fault, position)
        self.mutation = mutation
        self.driver = None
        self.actor_order = actor_order
        self.maintenance_done, self.publisher_done = threading.Event(), threading.Event()

    def monotonic(self):
        return time.monotonic()

    def await_ready(self, control, handle, timeout):
        return super().await_ready(control, handle, .2)

    def mapping(self, root, key):
        mapped = key in self.driver.mapped
        if self.mutation == 'mapping' and mapped:
            mapped = False
        return dict(mapped=mapped, independent=True, receipt_sha256='f'*64,
                    raw_sha256=sha(('raw:'+key).encode()))

    def read(self, root, name, limit):
        if name == 'STORE.json':
            return json.dumps(self.driver.store).encode()
        if name == 'PACKING.json':
            return json.dumps(self.driver.journal).encode()
        if name.startswith('physical-plans/'):
            return self.driver.plan
        return ('raw:'+name).encode()


class Driver(FakeDriver):
    def __init__(self, adapter):
        super().__init__()
        self.adapter = adapter
        adapter.driver = self
        self.head = dict(publication='initial-publication', digest='0'*64)
        self.initial = dict(outcome='accepted', request='initial', publication='initial-publication')
        self.ordinal = 1
        self.catalog = {'packs': [{'pack': 'base', 'index': 'base'}]}
        self.store = dict(catalog='0'*64, physical=dict(id='g1', ordinal=1, plan='0'*64))
        self.completed = copy.deepcopy(self.head)
        self.selected_work = dict(discovery=dict(completed=dict(selected=copy.deepcopy(self.head))))
        self.mapped = {'publications/initial-publication.json'}
        self.plan, self.journal, self.pid = b'{}', {}, 100

    def snapshot(self):
        return dict(store=copy.deepcopy(self.store), catalog=copy.deepcopy(self.catalog),
                    selected=copy.deepcopy(self.head),
                    work=copy.deepcopy(self.selected_work))

    def call(self, root, action, *, timeout=120, code=0, env=None, on_started=None, **fields):
        # Explicit transport keyword: it never enters input JSON or fields.
        require('on_started' not in fields, 'callback-leaked-into-engine-JSON')
        self.invocations.append((root, action, dict(env=copy.deepcopy(env), **copy.deepcopy(fields))))
        role = 'maintenance' if action == 'packing' else 'publisher'
        if on_started is not None and self.adapter.mutation != 'missing-start':
            self.pid += 1
            event = dict(pid=1 if self.adapter.mutation == 'same-pid' else self.pid,
                         observed_ns=time.perf_counter_ns(), positive_rss=100,
                         before_stdin_delivery=True)
            if self.adapter.mutation == 'rss':
                event['positive_rss'] = 0
            on_started(event)
        if action == 'packing':
            tail = copy.deepcopy(self.head)
            cutoff = dict(publication='wrong', digest='0'*64) if self.adapter.mutation == 'cutoff' else tail
            target = dict(catalog=sha(str(self.ordinal+1).encode()),
                          physical=dict(id='g'+str(self.ordinal+1), ordinal=self.ordinal+1))
            if self.adapter.mutation == 'same-mapping':
                target['catalog'] = self.store['catalog']
            if self.adapter.mutation == 'ordinal':
                target['physical']['ordinal'] = self.ordinal
            expected = sha(json.dumps(self.store).encode())
            candidate = dict(physical_parent=expected,
                generation=dict(id=target['physical']['id'], ordinal=target['physical']['ordinal'],
                                catalog=target['catalog']),
                discovery=dict(completed=dict(selected=cutoff), interval=None, deferred=[]))
            if self.adapter.mutation == 'work-binding':
                candidate['generation']['id'] = 'unbound'
            if self.adapter.mutation == 'interval':
                candidate['discovery']['interval'] = {}
            if self.adapter.mutation == 'deferred':
                candidate['discovery']['deferred'] = [{}]
            self.plan = json.dumps(candidate).encode()
            target['physical']['plan'] = sha(self.plan)
            self.journal = dict(status='installed', id=target['physical']['id'], expected_config=expected, target=target)
            if self.adapter.mutation == 'configuration':
                self.journal['expected_config'] = '0'*64
            if self.adapter.mutation == 'uninstalled':
                self.journal['status'] = 'building'
            if env:
                self.adapter.maintenance_done.clear()
                self.adapter.publisher_done.clear()
                self.adapter.checkpoint(env)
                if self.adapter.actor_order == 'publisher-first':
                    require(self.adapter.publisher_done.wait(2), 'memory-publisher-order-timeout')
            self.ordinal += 1
            self.store = copy.deepcopy(target)
            self.selected_work = copy.deepcopy(candidate)
            if self.adapter.mutation == 'selected-id':
                self.store['physical']['id'] = 'different-selected-id-'+str(self.ordinal)
            elif self.adapter.mutation == 'selected-catalog':
                self.store['catalog'] = 'f'*64
            elif self.adapter.mutation == 'selected-plan':
                self.store['physical']['plan'] = 'f'*64
            elif self.adapter.mutation == 'selected-ordinal':
                self.store['physical']['ordinal'] += 1
            elif self.adapter.mutation == 'selected-Work':
                self.selected_work['physical_parent'] = 'f'*64
            elif self.adapter.mutation == 'selected-cutoff':
                self.selected_work['discovery']['completed']['selected'] = dict(publication='different', digest='f'*64)
            self.catalog = {'packs': [{'pack': 'g'+str(self.ordinal), 'index': 'g'+str(self.ordinal)}]}
            self.completed = tail
            self.mapped.add('publications/'+tail['publication']+'.json')
            self.adapter.maintenance_done.set()
            return dict(status='ok', error=None, result=dict(status='registered'),
                        metrics=dict(physical_activation_holds_ns=[] if self.adapter.mutation == 'activation' else [1],
                                     pack_eligible_records=0 if self.adapter.mutation == 'eligible' else 5,
                                     rolling_publications_visited=1))
        if env:
            self.adapter.checkpoint(env)
        if action == 'resolve':
            return dict(status='ok', error=None, result=self.initial)
        require(action == 'publish', 'memory-only-publish-resolve-pack')
        if env and self.adapter.actor_order == 'maintenance-first':
            require(self.adapter.maintenance_done.wait(2), 'memory-maintenance-order-timeout')
        if self.adapter.mutation == 'callback' and env:
            raise RuntimeError('memory-publisher-callback-failure')
        request = fields['request']
        require(request['base'] == self.head, 'memory-single-publisher-base')
        self.head = dict(publication=request['request'], digest=sha(request['request'].encode()))
        result = dict(outcome='accepted', request=request['request'], publication=self.head['publication'])
        if env:
            self.adapter.publisher_done.set()
        return dict(status='ok', error=None, result=dict(result=result, selected=copy.deepcopy(self.head)))


class TraceHarness(RoundHarness):
    def state(self, root):
        return self.driver.snapshot()

    def pack(self, root, codec='zstd', **fields):
        return self.driver.call(root, 'packing', codec=codec, **fields)


def trace(fault='', position=1, mutation=None, capabilities=None, actor_order=None):
    a = Adapter(fault, position, mutation, actor_order)
    if capabilities is not None:
        a.capabilities = capabilities
    d, events = Driver(a), []
    h = TraceHarness(d, a, full_oracle, events.append, final_pin=PIN)
    failure = None
    try:
        with h.case('memory-v3-overlap', ROOT, FORECAST):
            h.ledgers[ROOT].accept(dict(request='initial', base=None, ops=[], packet=False), d.initial, d.head)
            h.concurrent_cycle(ROOT, 'memory-v3', 1)
    except BaseException as error:
        failure = error
    require(all(not a.alive(x) for x in a.handles), 'memory-thread-leak')
    require({id(x) for x in a.handles} <= set(a.waited), 'every-returned-callback-must-be-observed')
    require(set(a.closed_handles) == {id(x) for x in a.handles}, 'independent-case-close-required')
    return a, d, h, events, failure


def run():
    tracemalloc.start()
    a, d, h, events, failure = trace()
    require(failure is None, 'positive-trace:'+repr(failure))
    require(len(h.ledgers[ROOT].accepted) == 17, 'one-seed-plus15-exactly16-new-accepted')
    require(sum(row[1] == 'publish' for row in d.invocations) == 16, 'extra-or-retried-publisher')
    rounds = [x for x in events if x['kind'] == 'single-publisher-live-overlap']
    require(len(rounds) == 15, 'fifteen-real-barrier-memory-schedules')
    require(all(x['positive_interval_seconds'] > 0 and not x['construction_overlap_claimed']
                and x['observed_before_all_releases'] for x in rounds), 'positive-bounded-intervals')
    require(a.reader_releases == 4 and a.reader_waits == 4, 'four-old-readers-through-whole-cycle')
    require(all('on_started' not in row[2] for row in d.invocations), 'no-callback-in-engine-payload')
    checks = ['one-seed-plus15-coupled-rounds', 'four-readers', 'transport-only-start-callback',
              'positive-starts-ready-before-release', 'real-tail-cutoff-and-mapping']
    for mutation in ['missing-start', 'same-pid', 'rss', 'cutoff', 'uninstalled', 'activation',
                     'eligible', 'mapping', 'callback', 'same-mapping', 'ordinal', 'work-binding',
                     'interval', 'deferred', 'configuration']:
        result = trace(mutation=mutation)
        require(result[-1] is not None, 'failed-negative:'+mutation)
        checks.append('reject-'+mutation)
    result = trace(capabilities={'barrier:candidate-ready', 'barrier:pack-catalog-captured'})
    require(result[-1] is not None, 'missing-physical-barrier-must-fail')
    checks.append('reject-missing-physical-barrier')
    # Four reader entries are followed by maintenance5, publisher6. Every
    # negative observes all returned callbacks, including both paired actors.
    for fault, positions in [('spawn', [5, 6]), ('ready', [5, 6]), ('release', [1, 2]),
                             ('release-before', [1, 2]), ('wait', [1, 2]), ('wait-before', [1, 2]),
                             ('wait-clock', [1])]:
        for position in positions:
            result = trace(fault, position)
            require(result[-1] is not None, 'failed-fault:'+fault+str(position))
            checks.append(fault+'-'+str(position))
    # A pending terminal schema is refused before any legacy lifecycle call.
    a, d = Adapter(), FakeDriver()
    h = Harness(d, a, full_oracle, lambda row: None, final_pin=PIN)
    try:
        h.collect_failed_round(ROOT, 'pending', [], [], None)
    except Unsupported:
        require(not d.invocations, 'legacy-GC-must-never-be-called')
    else:
        raise RuntimeError('missing-terminal-schema-was-accepted')
    checks.append('pending-terminal-v2-fails-before-engine-call')
    try:
        with h.case('memory-no-explicit-start-transport', ROOT, FORECAST):
            h.call(ROOT, 'packing', on_started=lambda event: None)
    except RuntimeError as error:
        require('explicitly-consume-start-callback' in str(error) and not d.invocations,
                'legacy-driver-must-refuse-before-JSON-or-dispatch')
    else:
        raise RuntimeError('implicit-start-callback-transport-was-accepted')
    checks.append('legacy-driver-cannot-leak-callback-through-fields')
    for order in ['maintenance-first', 'publisher-first']:
        a, d, h, events, failure = trace(actor_order=order)
        require(failure is None, 'exact-target-positive:'+order+':'+repr(failure))
        joined = [x for x in events if x['kind'] == 'single-publisher-overlap-joined']
        require(len(joined) == 15 and all(x['exact_candidate_selected'] is True for x in joined),
                'actual-target-identity-not-recorded:'+order)
        checks.append('exact-target-positive-'+order)
        for mutation in ['selected-id', 'selected-catalog', 'selected-plan', 'selected-ordinal',
                         'selected-Work', 'selected-cutoff']:
            a, d, h, events, failure = trace(mutation=mutation, actor_order=order)
            expected = 'activated-Work-target-mismatch' if mutation in ('selected-Work', 'selected-cutoff') else 'activated-store-target-mismatch'
            require(isinstance(failure, RuntimeError) and str(failure) == expected,
                    'wrong-selected-identity-must-refuse:'+order+':'+mutation+':'+repr(failure))
            require(not any(x['kind'] in ('single-publisher-overlap-joined', 'schedule-complete-unqualified')
                            for x in events), 'wrong-target-recorded-as-success')
            checks.append('reject-'+mutation+'-'+order)
    _, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()
    require(peak < 8*MIB, 'memory-controls-allocation-bound')
    return dict(status='passed-memory-only', checks=checks, count=len(checks),
                peak_python_traced_bytes=peak, native_execution=False,
                engine_execution=False, disk_fixtures=False)


if __name__ == '__main__':
    print(json.dumps(run(), indent=2))

