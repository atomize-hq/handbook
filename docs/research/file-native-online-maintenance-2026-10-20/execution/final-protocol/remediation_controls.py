"""Exact Driver and exceptional-path MEMORY ONLY controls for review F1-F4.

No filesystem/engine/native process/fixture operations. The thread registry here
models adapter closure solely to prove every Python callback is observed. Fake
closure/overlap/oracle values cannot qualify any real engine or native case.
"""
import copy
import json
import threading
import tracemalloc
from contextlib import contextmanager
from types import SimpleNamespace

from focused_protocol import Harness, ObservedExit, require
from focused_schedules import automatic_policy, owned_decline
from fake_controls import (PIN, ROOT, FORECAST, FakeAdapter, FakeDriver, RoundAdapter,
                           RoundDriver, RoundHarness, full_oracle)


class ContractDriver(FakeDriver):
    """Exactly the real Windows/Linux transport/payload separation."""
    def __init__(self, mutation=None, exit_code=2, sequence=None):
        super().__init__()
        self.mutation = mutation
        self.exit_code = exit_code
        self.sequence = list(sequence or [])
        self.rows = []

    def call(self, root, action, *, timeout=120, code=0, env=None, **fields):
        payload = dict(root=str(root), action=action, **copy.deepcopy(fields))
        self.invocations.append((root, action, dict(timeout=timeout, code=code, env=copy.deepcopy(env), **fields)))
        exit_code = self.sequence.pop(0) if self.sequence else self.exit_code
        response = (dict(status='error', error='conflict:captured-base', result=None) if exit_code == 2 else
                    dict(status='ok', error=None, result=dict(status='cleaning', candidates=1, batch_ns=100)))
        row = dict(input=payload, env=copy.deepcopy(env), executable_sha256=self.binary_sha256,
                   exit=exit_code, stdout='' if exit_code == 86 else json.dumps(response), violations=[])
        if self.mutation == 'input':
            row['input']['action'] = 'unrelated-action'
        elif self.mutation == 'env':
            row['env'] = {'EXP_BARRIER': 'wrong-owner'}
        elif self.mutation == 'missing-env':
            row.pop('env')
        elif self.mutation == 'binary':
            row['executable_sha256'] = '0'*64
        elif self.mutation in ('rss', 'timeout'):
            row['violations'] = [self.mutation+'-censor']
        elif self.mutation == 'semantic-error':
            row['stdout'] = json.dumps(dict(status='error', error='integrity:unrelated', result=None))
        elif self.mutation == 'complete':
            response['result']['status'] = 'complete'
            row['stdout'] = json.dumps(response)
        self.rows.append(copy.deepcopy(row))
        if exit_code != code or row['violations']:
            raise AssertionError(row)
        return response


class FaultAdapter(RoundAdapter):
    def __init__(self, fault, position=1, secondary_release=False, secondary_wait=False):
        super().__init__()
        self.fault, self.position = fault, position
        self.secondary_release, self.secondary_wait = secondary_release, secondary_wait
        self.spawn_calls = self.ready_calls = self.release_calls = self.wait_calls = 0
        self.handles, self.waited, self.closed_handles = [], [], []
        self.reader_releases = self.reader_waits = 0
        self.ready_by_thread = {}

    def wall_time(self):
        if self.fault == 'wait-clock' and self.release_calls:
            raise RuntimeError('wait-clock-fault')
        return super().wall_time()

    def spawn(self, callback):
        self.spawn_calls += 1
        if self.fault == 'spawn' and self.spawn_calls == self.position:
            raise RuntimeError('spawn-fault-'+str(self.position))
        handle = super().spawn(callback)
        handle['fault_role'] = 'reader' if self.spawn_calls <= 4 else 'publisher'
        self.handles.append(handle)
        return handle

    def await_ready(self, control, handle, timeout):
        self.ready_calls += 1
        super().await_ready(control, handle, timeout)
        if self.fault == 'ready' and self.ready_calls == self.position:
            raise RuntimeError('ready-fault-'+str(self.position))

    def release(self, control):
        self.release_calls += 1
        if '-reader-' in control['ready']:
            self.reader_releases += 1
        if self.fault == 'release-before' and self.release_calls == self.position:
            raise RuntimeError('release-before-fault-'+str(self.position))
        super().release(control)
        # The release has occurred; its failed receipt is still independently
        # retained. A genuinely unreleased native process requires outer guard
        # closure, separately modelled by the registry below, never fake PASS.
        if ((self.fault == 'release' and self.release_calls == self.position) or
                self.secondary_release and self.release_calls == 1 or
                self.fault == 'reader-release' and '-reader-' in control['ready']
                and self.reader_releases == self.position):
            raise RuntimeError('release-fault-'+str(self.release_calls))

    def checkpoint(self, env):
        self.ready_by_thread[threading.get_ident()] = env['EXP_READY']
        super().checkpoint(env)

    def wait(self, handle, timeout):
        self.wait_calls += 1
        self.waited.append(id(handle))
        if handle['fault_role'] == 'reader':
            self.reader_waits += 1
        ready_path = self.ready_by_thread.get(handle['thread'].ident)
        if self.fault == 'release-before' and ready_path and not self.controls[ready_path][1].is_set():
            # A bounded native wait would time out. Model the timeout immediately
            # and require the independent registry close path to finish it.
            raise RuntimeError('unreleased-memory-callback-bounded-wait-failure')
        if self.fault == 'wait-before' and self.wait_calls == self.position:
            raise RuntimeError('wait-before-fault-'+str(self.position))
        value = super().wait(handle, timeout)
        if ((self.fault == 'wait' and self.wait_calls == self.position) or
                self.secondary_wait and self.wait_calls == 2 or
                self.fault == 'reader-wait' and handle['fault_role'] == 'reader'
                and self.reader_waits == self.position):
            raise RuntimeError('wait-fault-'+str(self.wait_calls))
        return value

    def close_case(self, case_id, root):
        # Memory-only registry cleanup is independently checked AFTER every
        # harness wait was attempted. It cannot hide omitted harness waits.
        for _, release in self.controls.values():
            release.set()
        for handle in self.handles:
            handle['thread'].join(2)
            self.closed_handles.append(id(handle))
        closed = all(not self.alive(handle) for handle in self.handles)
        return dict(processes_closed=closed, handles_closed=closed, python_callbacks_joined=closed,
                    qualification='memory thread registry only; no native process proof')


class FaultDriver(RoundDriver):
    def call(self, root, action, *, timeout=120, code=0, env=None, **fields):
        if (self.adapter.fault == 'callback' and action == 'publish'
                and fields['request']['request'].endswith('-p'+str(self.adapter.position))):
            self.adapter.checkpoint(env)
            raise RuntimeError('callback-fault-'+str(self.adapter.position))
        return super().call(root, action, timeout=timeout, code=code, env=env, **fields)


class FaultHarness(RoundHarness):
    def pack(self, root, codec='zstd', **fields):
        if self.a.fault == 'pack':
            raise RuntimeError('pack-fault-primary')
        return super().pack(root, codec, **fields)


class AutoHarness:
    """A schedule trace, not an implementation/model of engine admission."""
    def __init__(self, admit_at=551):
        self.admit_at = admit_at
        self.ledgers = {ROOT: SimpleNamespace(accepted={})}
        self.store = b'original-store'
        self.journal = None
        self.owners = set()
        self.packs, self.notes, self.oracles = [], [], []
        self.published = 0
        self.cleaned = False
        self.a = SimpleNamespace(inventory=lambda root: dict(files={}, directories=[]), exists=self.exists)

    @contextmanager
    def case(self, case_id, root, forecast):
        yield

    def initialize(self, root, mode, count):
        require(mode == 'A' and count == 10, 'auto-default-N10-A')
        self.ledgers[root].accepted['initial'] = None

    def read(self, root, name, limit):
        return self.store if name == 'STORE.json' else str(len(self.ledgers[root].accepted)).encode()

    def exists(self, root, name):
        if name == 'PACKING.json':
            return self.journal is not None
        if name.startswith('packs/'):
            return name.split('/')[1] in self.owners
        raise RuntimeError('unexpected-auto-memory-path')

    def publish(self, root, name):
        require(name not in self.ledgers[root].accepted, 'duplicate-auto-attempt')
        self.ledgers[root].accepted[name] = None
        self.published += 1

    def pack(self, root, **fields):
        require(fields.get('force') is False and fields.get('bootstrap') is True, 'automatic-never-force-or-repair')
        h = len(self.ledgers[root].accepted)
        self.packs.append((h, copy.deepcopy(fields)))
        if h < self.admit_at:
            if h > 1:
                owner = 'declined-'+str(h)
                self.owners.add(owner)
                self.journal = dict(id=owner, status='aborted-owned')  # actual omitted target contract
            return dict(status='error', error='refused:automatic-compaction-benefit', result=None)
        self.store = b'new-physical-store'
        return dict(status='ok', error=None, result=dict(status='registered', forced_correctness=False))

    def document(self, root, name):
        require(name == 'PACKING.json', 'auto-memory-document-only-journal')
        return copy.deepcopy(self.journal)

    def note(self, kind, **fields):
        self.notes.append(dict(kind=kind, **fields))

    def verify(self, root, checkpoint):
        self.oracles.append(checkpoint)

    def cleanup(self, root, *, max_batches):
        require(max_batches == 160, 'bounded-bootstrap-cleanup-not-two-call-pass')
        self.owners.clear()
        self.cleaned = True

    def check_outcomes(self, root):
        pass


def run():
    tracemalloc.start()
    checks, exception_matrix = [], []
    def passed(name):
        checks.append(name)
    def refuses(name, callback):
        try:
            callback()
        except (RuntimeError, ValueError, AssertionError, TypeError, KeyError):
            passed(name)
        else:
            raise RuntimeError('remediation-negative-control-did-not-refuse:'+name)

    # F1 exact real-signature inputs, with environment separately recorded.
    for env_name, options in [('absent', {}), ('None', {'env': None}),
                              ('barrier', {'env': {'EXP_BARRIER': 'candidate-ready', 'EXP_READY': 'owned-r', 'EXP_RELEASE': 'owned-x'}})]:
        driver, adapter, events = ContractDriver(), FakeAdapter(), []
        harness = Harness(driver, adapter, full_oracle, events.append, final_pin=PIN)
        with harness.case('F1-exact-env-'+env_name, ROOT, FORECAST):
            value = harness.call(ROOT, 'publish', request={'request': 'r'},
                                 allowed_errors=('conflict:captured-base',), **options)
            require(value['error'] == 'conflict:captured-base' and 'env' not in driver.rows[-1]['input'],
                    'env-leaked-into-engine-input')
        passed('F1-real-signature-'+env_name)
    for mutation in ('input', 'env', 'missing-env', 'binary', 'rss', 'timeout', 'semantic-error'):
        driver, adapter = ContractDriver(mutation), FakeAdapter()
        harness = Harness(driver, adapter, full_oracle, lambda row: None, final_pin=PIN)
        with harness.case('F1-negative-'+mutation, ROOT, FORECAST):
            refuses('F1-reject-'+mutation, lambda: harness.call(ROOT, 'publish', request={'request': 'r'},
                    env={'EXP_BARRIER': 'candidate-ready'}, allowed_errors=('conflict:captured-base',)))

    # F2 matches the serializer's omitted optional field, without changing it.
    require(owned_decline(dict(id='birth', status='aborted-owned')) == 'birth', 'omitted-None-target')
    require(owned_decline(dict(id='birth', status='aborted-owned', target=None)) == 'birth', 'explicit-None-target')
    passed('F2-omitted-or-null-target')
    for target in ({}, {'physical': {}}, False, 0, 'not-null'):
        refuses('F2-nonnull-target-'+repr(target), lambda target=target:
                owned_decline(dict(id='birth', status='aborted-owned', target=target)))
    refuses('F2-not-aborted-owner', lambda: owned_decline(dict(id='birth', status='installed')))
    refuses('F2-missing-owner-identity', lambda: owned_decline(dict(status='aborted-owned')))

    # F3 source-calibrated H551 is a finite schedule, not a guarantee of success.
    auto_forecast = dict(automatic_history_ceiling=551, automatic_cleanup_max_calls=160)
    for admitted_at in (151, 551):
        h = AutoHarness(admit_at=admitted_at)
        automatic_policy(h, ROOT, auto_forecast)
        require(len(h.ledgers[ROOT].accepted) == admitted_at and h.published == admitted_at-1
                and h.cleaned and not h.owners, 'auto-bounded-history-and-actual-owner-cleanup')
        require([row[0] for row in h.packs] == ([1, 31, 151] if admitted_at == 151 else [1, 31, 151, 551]),
                'auto-hidden-builds-or-publications')
        passed('F3-finite-admission-trace-H'+str(admitted_at))
    h = AutoHarness(admit_at=552)
    refuses('F3-H551-still-declines-is-incomplete', lambda: automatic_policy(h, ROOT, auto_forecast))
    require(len(h.ledgers[ROOT].accepted) == 551 and len(h.packs) == 4 and not h.cleaned,
            'auto-ceiling-silently-extended-or-refusal-erased')
    require(any(row['kind'] == 'automatic-H151-decline-retained' for row in h.notes), 'H151-negative-not-retained')
    require(any(row['kind'] == 'automatic-ceiling-decline-unqualified' for row in h.notes), 'ceiling-decline-not-retained')
    passed('F3-all-declined-owners-and-failures-retained-at-ceiling')
    refuses('F3-no-H551-forecast-refuses-before-init', lambda: automatic_policy(AutoHarness(), ROOT, {}))
    empty = AutoHarness()
    refuses('F3-no-cleanup-bound-refuses-before-init', lambda: automatic_policy(empty, ROOT,
            dict(automatic_history_ceiling=551)))
    require(not empty.ledgers[ROOT].accepted, 'auto-wrote-before-cleanup-forecast-validation')

    # F4 failure matrix. Every handle actually returned by spawn MUST reach wait.
    scenarios = [(kind, i, False, False) for kind in ('spawn', 'ready', 'release', 'wait') for i in range(1, 9)]
    scenarios += [('wait-before', i, False, False) for i in (1, 4, 5, 8)]
    scenarios += [('callback', i, False, False) for i in range(4)]
    scenarios += [(kind, i, False, False) for kind in ('reader-release', 'reader-wait') for i in range(1, 5)]
    scenarios += [('release-before', i, False, False) for i in (1, 4)]
    scenarios += [('pack', 1, False, False), ('pack', 1, True, True)]
    scenarios += [('wait-clock', 1, False, False)]
    for kind, position, secondary_release, secondary_wait in scenarios:
        adapter = FaultAdapter(kind, position, secondary_release, secondary_wait)
        driver, events = FaultDriver(adapter), []
        harness = FaultHarness(driver, adapter, full_oracle, events.append, final_pin=PIN)
        failure = None
        try:
            with harness.case('F4-'+kind+'-'+str(position), ROOT, FORECAST):
                harness.ledgers[ROOT].accept(dict(request='initial', base=None, ops=[], packet=False),
                                             driver.initial, driver.head)
                harness.concurrent_cycle(ROOT, 'fault-trace', 4)
        except BaseException as error:
            failure = repr(error)
        require(failure is not None, 'injected-fault-not-observed')
        ids = {id(handle) for handle in adapter.handles}
        require(ids == set(adapter.waited) and len(adapter.waited) == len(ids),
                'spawned-callback-missed-or-duplicate-wait:'+repr((kind, position)))
        require(all(not adapter.alive(handle) for handle in adapter.handles), 'memory-nondaemon-thread-still-live')
        cleanup = [row for row in events if row['kind'] == 'callback-cleanup']
        require(all(row['observed_waits'] == row['spawned'] for row in cleanup), 'cleanup-wait-accounting')
        if kind == 'pack':
            require('pack-fault-primary' in failure, 'primary-maintenance-failure-masked')
        if secondary_release:
            require(any(len(row['secondary']) >= 2 and row['primary'] and 'pack-fault-primary' in row['primary']
                        for row in cleanup), 'primary-plus-secondary-failures-not-retained')
        exception_matrix.append(dict(fault=kind, position=position, spawned=len(ids), waited=len(adapter.waited),
                                     all_threads_joined=True, primary_and_secondary_retained=True,
                                     secondary_release=secondary_release, secondary_wait=secondary_wait,
                                     status='expected-memory-failure-retained'))
    passed('F4-every-spawn-ready-release-wait-position-plus-callback-maintenance-and-combined-failures')

    # A cooperative cleanup return cannot be mislabeled as the injected crash.
    adapter, driver, events = FakeAdapter(), ContractDriver(sequence=[0, 86]), []
    adapter.capabilities = {'exit:pack-old-delete'}
    h = Harness(driver, adapter, full_oracle, events.append, final_pin=PIN)
    with h.case('bounded-cleanup-exit', ROOT, FORECAST):
        observed = h.cleanup_to_exit(ROOT, 'pack-old-delete', max_calls=8)
        require(isinstance(observed, ObservedExit) and observed.code == 86 and len(driver.invocations) == 2,
                'bounded-cleanup-not-exact-observed-exit')
    passed('crash-exit0-cleaning-then-authentic86-two-call-trace')
    for mutation in ('input', 'env', 'binary', 'rss', 'timeout'):
        adapter, driver = FakeAdapter(), ContractDriver(mutation, exit_code=86)
        adapter.capabilities = {'exit:pack-old-delete'}
        h = Harness(driver, adapter, full_oracle, lambda row: None, final_pin=PIN)
        with h.case('observed-exit-negative-'+mutation, ROOT, FORECAST):
            refuses('crash-reject-unbound86-'+mutation, lambda: h.cleanup_to_exit(ROOT, 'pack-old-delete'))
    for mutation in (None, 'complete'):
        adapter, driver = FakeAdapter(), ContractDriver(mutation, exit_code=0)
        adapter.capabilities = {'exit:pack-old-delete'}
        h = Harness(driver, adapter, full_oracle, lambda row: None, final_pin=PIN)
        with h.case('crash-not-reached', ROOT, FORECAST):
            refuses('crash-exit0-is-not-pass-'+str(mutation), lambda: h.cleanup_to_exit(ROOT, 'pack-old-delete', max_calls=3))
        require(len(driver.invocations) <= 3, 'cleanup-crash-unbounded-retry')

    _, peak = tracemalloc.get_traced_memory()
    tracemalloc.stop()
    require(peak < 16*1024**2, 'remediation-memory-control-allocation-bound')
    return dict(status='PASSED_MEMORY_ONLY_REMEDIATION_CONTROLS', checks=checks,
                checks_count=len(checks), exception_matrix=exception_matrix,
                exception_scenarios=len(exception_matrix), tracemalloc_peak_bytes=peak,
                python_allocation_limit_bytes=16*1024**2, real_engine_calls=0,
                native_processes=0, fixture_writes=0, filesystem_writes=0,
                bytecode_writes=False, real_case_qualification=False,
                required_engine_cases_still_pending=True)


if __name__ == '__main__':
    print(json.dumps(run(), indent=2))
