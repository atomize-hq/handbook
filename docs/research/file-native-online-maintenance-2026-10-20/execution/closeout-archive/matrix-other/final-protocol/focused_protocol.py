"""Shared, source-only focused schedules. No native process/guard policy here.

The injected Driver preserves raw native events. Adapter owns forecasts, paths,
barriers, process identity/RSS/closure and evidence persistence. Oracle is an
independent complete decoder, never engine audit. See README for exact boundary.
"""
import copy
import hashlib
import inspect
import json
import math
import threading
import time
from contextlib import contextmanager, ExitStack

MIB = 1024**2
READERS = frozenset({'resolve', 'point', 'inspect', 'lookup-bench'})
FUNCTIONAL_CUTOFF = 1792581014  # 2026-10-21T11:10:14Z; never reset per worker
ORACLE_CHECKS = frozenset({
    'complete_physical', 'all_accepted_outcomes', 'exact_request_intents',
    'selected_membership', 'graph', 'source_bytes', 'navigation', 'provenance',
    'retired_refusals', 'b_loose_controls', 'ordinary_selected_once_b',
    'read_only_inventory',
})


class Unsupported(RuntimeError):
    pass


class ObservedExit:
    """Harness-only evidence of an explicitly allowed, raw-bound process exit.

    This is not an engine result or a passed crash case. Deletion identity,
    recovery, full oracles and native closure must still be established.
    """
    def __init__(self, code, row):
        self.code = code
        self.row = copy.deepcopy(row)


def require(condition, message):
    # Assertions are deliberately not used for qualification checks: python -O
    # cannot silently remove them.
    if not condition:
        raise RuntimeError(message)


def strict(raw, limit):
    require(type(raw) is bytes and len(raw) <= limit, 'bounded-bytes-required')
    def unique(items):
        result = {}
        for key, value in items:
            require(key not in result, 'duplicate-json-key')
            result[key] = value
        return result
    def invalid(value):
        raise ValueError('nonfinite-json:'+value)
    return json.loads(raw, object_pairs_hook=unique, parse_constant=invalid)


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def relative(name):
    require(isinstance(name, str) and name and '\\' not in name and ':' not in name
            and not name.startswith('/') and all(p not in ('', '.', '..') for p in name.split('/')),
            'owned-relative-path-required')
    return name


def normalize(request):
    """Match the retained independent outcome oracle's Request/Op normalization."""
    require(type(request) is dict and set(request) <= {'request', 'base', 'ops', 'packet'}
            and {'request', 'base', 'ops'} <= set(request), 'closed-request-shape')
    require(isinstance(request['request'], str) and type(request['ops']) is list,
            'request-types')
    base = request['base']
    if base is not None:
        require(type(base) is dict and set(base) == {'publication', 'digest'}, 'selector-shape')
        base = dict(publication=base['publication'], digest=base['digest'])
    ops = []
    for op in request['ops']:
        require(type(op) is dict and {'kind', 'id'} <= set(op)
                and set(op) <= {'kind', 'id', 'text', 'target', 'value'}, 'closed-op-shape')
        value = op.get('value', 0)
        require(type(value) is int and 0 <= value < 2**64, 'u64-op-value')
        ops.append(dict(kind=op['kind'], id=op['id'], text=op.get('text', ''),
                        target=op.get('target', ''), value=value))
    packet = request.get('packet', True)
    require(type(packet) is bool, 'packet-boolean')
    return dict(request=request['request'], base=base, ops=ops, packet=packet)


def intent(request):
    return sha(json.dumps(normalize(request), indent=2, ensure_ascii=False,
                          allow_nan=False).encode())


class Inputs:
    """Never overwrite differing captured bases by request ID under concurrency."""
    def __init__(self):
        self.attempts = {}
        self.accepted = {}
        self.lock = threading.Lock()

    def record(self, request):
        request = copy.deepcopy(request)
        key = (request['request'], intent(request))
        with self.lock:
            self.attempts.setdefault(key, request)
        return key

    def accept(self, request, result, selected):
        key = self.record(request)
        require(result.get('outcome') == 'accepted' and result.get('request') == key[0]
                and result.get('publication') == selected.get('publication'), 'accepted-result-binding')
        with self.lock:
            prior = self.accepted.get(key[0])
            value = dict(intent=key[1], request=copy.deepcopy(request),
                         result=copy.deepcopy(result), selected=copy.deepcopy(selected))
            require(prior is None or prior == value, 'conflicting-accepted-input')
            self.accepted[key[0]] = value

    def oracle_inputs(self):
        with self.lock:
            return {name: copy.deepcopy(row['request']) for name, row in self.accepted.items()}


class ThreadCalls:
    """Optional Adapter mixin; threads only call the injected guarded Driver.

    Adapter close_case MUST also close native descendants if a join fails. This
    helper provides no kill/cancel implementation and no native safety claim.
    """
    def spawn(self, callback):
        state = {'response': None, 'error': None}
        def run():
            try:
                state['response'] = callback()
            except BaseException as error:
                state['error'] = error
        state['thread'] = threading.Thread(target=run, daemon=False)
        state['thread'].start()
        return state

    def alive(self, handle):
        return handle['thread'].is_alive()

    def wait(self, handle, timeout):
        handle['thread'].join(timeout)
        require(not self.alive(handle), 'join-timeout-adapter-must-close-descendants')
        if handle['error'] is not None:
            raise handle['error']
        return handle['response']


def mapping_identity(state):
    """Catalog/descriptor content identity, separate from id/ordinal/work CAS."""
    return sha(json.dumps(state['catalog'], sort_keys=True, separators=(',', ':')).encode())


def descriptor_identities(state):
    return {json.dumps(row, sort_keys=True, separators=(',', ':'))
            for row in state['catalog']['packs']}


def transition(before, after):
    old, new = before['store']['physical'], after['store']['physical']
    require(new['id'] != old['id'] and new['ordinal'] == old['ordinal']+1,
            'distinct-next-physical-generation-required')
    require(mapping_identity(before) != mapping_identity(after), 'unchanged-mapping-not-activation-stress')
    require(descriptor_identities(after)-descriptor_identities(before), 'new-mapping-descriptor-required')


def quantiles(values):
    require(bool(values), 'no-samples')
    ordered = sorted(values)
    return dict(count=len(values), p50=ordered[math.ceil(len(values)*.50)-1],
                p95=ordered[math.ceil(len(values)*.95)-1],
                exploratory_p99=ordered[math.ceil(len(values)*.99)-1], max=ordered[-1])


class Harness:
    def __init__(self, driver, adapter, oracle, emit, *, final_pin):
        require(__debug__, 'optimized-Python-disables-injected-driver-checks')
        require(len(final_pin) == 40 and all(c in '0123456789abcdef' for c in final_pin), 'exact-final-pin')
        self.driver, self.a, self.oracle, self.emit = driver, adapter, oracle, emit
        self.final_pin = final_pin
        self.ledgers = {}
        self.lock = threading.Lock()
        self.active = set()
        self.oracles = []

    def adopt_inputs(self, root, records, *, seed_receipt_sha256):
        """Attach complete externally authenticated donor inputs without cloning."""
        require(len(seed_receipt_sha256) == 64 and root not in self.ledgers,
                'fresh-authenticated-donor-input-binding')
        ledger = Inputs()
        for row in records:
            ledger.accept(row['request'], row['result'], row['selected'])
        require(bool(ledger.accepted), 'complete-donor-inputs-required')
        self.ledgers[root] = ledger
        self.note('donor-inputs-adopted', root=str(root), seed_receipt_sha256=seed_receipt_sha256,
                  accepted=len(ledger.accepted), no_clone=True)

    def note(self, kind, **fields):
        # Sink must reserve before append, flush, retain every failed prefix and
        # must never redact or replace a failed event with a later success.
        with self.lock:
            self.emit(dict(kind=kind, final_pin=self.final_pin, **copy.deepcopy(fields)))

    def check_time(self):
        require(self.a.wall_time() < FUNCTIONAL_CUTOFF, 'functional-cutoff')

    @contextmanager
    def case(self, case_id, root, forecast):
        """Forecast/GO before any store/control writes; closure even on failure."""
        self.check_time()
        require(root not in self.active, 'case-root-already-owned')
        permit = self.a.preflight(case_id, root, copy.deepcopy(forecast), self.final_pin)
        require(permit.get('go') is True and permit.get('case_id') == case_id
                and permit.get('root') == str(root) and permit.get('final_pin') == self.final_pin,
                'case-forecast-or-meta-GO-missing')
        require(permit.get('reader_limit') == 256*MIB and permit.get('checker_limit') == 2048*MIB,
                'independent-reader-checker-roles-required')
        require(permit.get('default_payloads') is True, 'default-payload-profile-required')
        require(permit.get('heavy_token') and permit.get('host_guard'), 'guard-token-required')
        require(permit.get('binary_sha256') == self.driver.binary_sha256
                and permit.get('binary_source_pin') == self.final_pin
                and all(isinstance(permit.get(key), str) and len(permit[key]) == 64 for key in
                        ('binary_sha256', 'source_manifest_sha256', 'adapter_sha256', 'guard_sha256')),
                'source-binary-adapter-guard-bindings-required')
        self.active.add(root)
        self.ledgers.setdefault(root, Inputs())
        self.note('case-start', case_id=case_id, root=str(root), forecast=forecast, permit=permit)
        failure = None
        try:
            yield
        except BaseException as error:
            failure = repr(error)
            self.note('case-failure', case_id=case_id, root=str(root), error=failure)
            raise
        finally:
            self.active.remove(root)
            closure = self.a.close_case(case_id, root)
            self.note('case-closure', case_id=case_id, root=str(root), failure=failure, closure=closure)
            require(closure.get('processes_closed') is True and closure.get('handles_closed') is True
                    and closure.get('python_callbacks_joined') is True,
                    'positive-native-process-handle-and-Python-callback-closure-required')
            # This is only a schedule outcome. External CaseLedger still requires
            # authentic raw evidence, full oracles and source/native bindings.
            if failure is None:
                self.note('schedule-complete-unqualified', case_id=case_id, root=str(root))

    def call(self, root, action, *, timeout=120, code=0, env=None, on_started=None,
             allowed_errors=(), observed_exits=(), **fields):
        require(root in self.active, 'unadmitted-root')
        self.check_time()
        require(set(observed_exits) <= {86}, 'only-explicit-injected-exit86-is-observable')
        require(not (observed_exits and code != 0), 'observed-exit-uses-normal-code0-path')
        role = 'reader' if action in READERS else 'checker'
        self.a.before_call(root, action, role, min(timeout, FUNCTIONAL_CUTOFF-self.a.wall_time()))
        # Driver options are distinct from the engine JSON payload. In both
        # actual Driver signatures env is consumed before **engine_fields.
        payload = dict(root=str(root), action=action, **copy.deepcopy(fields))
        environment = copy.deepcopy(env)
        require(on_started is None or callable(on_started), 'start-callback-must-be-callable')
        if on_started is not None:
            parameter = inspect.signature(self.driver.call, follow_wrapped=False).parameters.get('on_started')
            require(parameter is not None and parameter.kind in
                    (inspect.Parameter.KEYWORD_ONLY, inspect.Parameter.POSITIONAL_OR_KEYWORD),
                    'driver-must-explicitly-consume-start-callback-outside-engine-JSON')
        transport = {} if on_started is None else {'on_started': on_started}
        self.note('request-before-dispatch', input=payload, env=environment,
                  expected_exit=code, observed_exits=list(observed_exits), role=role)
        try:
            response = self.driver.call(root, action, timeout=min(timeout, FUNCTIONAL_CUTOFF-self.a.wall_time()),
                                        code=code, env=environment, **transport, **fields)
        except AssertionError as error:
            # Existing Windows/Linux Driver.call accepts one expected exit, but
            # a concurrent publish can legitimately return 0 OR semantic exit2.
            # Those drivers retain their exact raw row in AssertionError(row).
            # Recover only this invocation's authentic, violation-free exit2;
            # never use shared driver.last (which races concurrent calls).
            row = error.args[0] if error.args and type(error.args[0]) is dict else {}
            bound = (row.get('input') == payload and 'env' in row and row['env'] == environment
                     and row.get('executable_sha256') == self.driver.binary_sha256
                     and row.get('violations') == [] and isinstance(row.get('stdout'), str))
            if bound and row.get('exit') in observed_exits:
                self.note('observed-injected-process-exit', input=payload, env=environment,
                          exit=row['exit'], raw_driver_row=row, qualification='crash-observed-unqualified')
                return ObservedExit(row['exit'], row)
            if not (bound and allowed_errors and row.get('exit') == 2):
                self.note('call-failure', input=payload, expected_exit=code, error=repr(error))
                raise
            response = strict(row['stdout'].encode(), 4*MIB)
            require(response.get('status') == 'error' and response.get('error') in allowed_errors,
                    'unexpected-native-refusal')
            self.note('authentic-semantic-refusal', input=payload, response=response,
                      raw_driver_row_retained=True)
        except BaseException as error:
            self.note('call-failure', input=payload, expected_exit=code, error=repr(error))
            raise
        self.note('call-result', input=payload, expected_exit=code, response=response)
        if code == 0:
            require(type(response) is dict and (response.get('status') == 'ok' or
                    response.get('status') == 'error' and response.get('error') in allowed_errors),
                    'engine-success-or-declared-semantic-envelope')
        elif code == 2:
            require(type(response) is dict and response.get('status') == 'error', 'engine-error-envelope')
        return response

    def join_callbacks(self, root, label, handles, controls=(), primary=None):
        """Release EVERY control and observe EVERY spawned callback on all paths.

        A bounded wait failure remains a failure and requires adapter native
        closure; it cannot be replaced by a nominal closure receipt. Secondary
        failures are collected before evidence emission so a sink error cannot
        skip later joins. The original exception takes precedence after all
        cleanup attempts, with each secondary failure retained independently.
        """
        secondary, responses = [], [None]*len(handles)
        for index, control in enumerate(controls):
            try:
                self.a.release(control)
            except BaseException as error:
                secondary.append(dict(operation='release', index=index, error=repr(error)))
        for index, handle in enumerate(handles):
            remaining = 0.0
            try:
                remaining = max(0.0, FUNCTIONAL_CUTOFF-self.a.wall_time())
            except BaseException as error:
                secondary.append(dict(operation='wait-clock', index=index, error=repr(error)))
            try:
                responses[index] = self.a.wait(handle, min(15.0, remaining))
            except BaseException as error:
                secondary.append(dict(operation='wait', index=index, error=repr(error)))
        self.note('callback-cleanup', root=str(root), label=label,
                  spawned=len(handles), observed_waits=len(handles),
                  primary=repr(primary) if primary is not None else None,
                  secondary=secondary, responses=responses,
                  positive_python_join=not any(x['operation'] == 'wait' for x in secondary))
        if primary is not None:
            raise primary
        require(not secondary, 'callback-cleanup-secondary-failures:'+repr(secondary))
        return responses

    def cleanup_to_exit(self, root, phase, *, max_calls=8):
        """Bounded cooperative continuation; exit0 is never a crash pass."""
        require(type(max_calls) is int and 1 <= max_calls <= 32, 'bounded-cleanup-crash-attempts')
        require('exit:'+phase in self.a.capabilities, 'source-checkpoint-not-advertised:'+phase)
        for number in range(max_calls):
            value = self.call(root, 'packing', operation='cleanup', code=0,
                              env={'EXP_EXIT': phase}, observed_exits=(86,))
            if isinstance(value, ObservedExit):
                self.note('cleanup-crash-reached', root=str(root), phase=phase,
                          calls=number+1, qualification='actual-deletion-proof-and-oracles-still-required')
                return value
            result = value['result']
            require(type(result.get('candidates')) is int and 0 <= result['candidates'] <= 1000,
                    'cleanup-crash-continuation-candidate-bound')
            self.note('cleanup-crash-continuation', root=str(root), phase=phase, index=number, result=result)
            require(result.get('status') == 'cleaning', 'cleanup-complete-without-requested-crash')
        raise RuntimeError('cleanup-crash-checkpoint-not-reached-within-frozen-attempts')

    def read(self, root, name, limit=128*1024):
        raw = self.a.read(root, relative(name), limit)
        require(type(raw) is bytes and len(raw) <= limit, 'adapter-read-bound')
        return raw

    def document(self, root, name, limit=128*1024):
        return strict(self.read(root, name, limit), limit)

    def selected(self, root):
        return self.document(root, 'SELECTED.json', 65536)

    def state(self, root):
        store = self.document(root, 'STORE.json', 65536)
        require(store.get('catalog') is not None and store.get('physical') is not None,
                'generation-catalog-required')
        catalog_raw = self.read(root, 'pack-catalogs/'+store['catalog']+'.json', 65536)
        require(sha(catalog_raw) == store['catalog'], 'catalog-hash')
        plan_raw = self.read(root, 'physical-plans/'+store['physical']['plan']+'.json')
        require(sha(plan_raw) == store['physical']['plan'], 'work-plan-hash')
        value = dict(store=store, catalog=strict(catalog_raw, 65536), work=strict(plan_raw, 128*1024),
                     selected=self.selected(root))
        self.note('selected-state', root=str(root), state=value)
        return value

    def containers(self, state):
        names = []
        for desc in state['catalog']['packs']:
            prefix = 'packs/'+(desc['instance']+'/' if desc.get('instance') else '')
            names.extend([relative(prefix+desc['pack']+'.pack'), relative(prefix+desc['index']+'.idx')])
        return names

    def initialize(self, root, mode, count=64):
        self.call(root, 'init', mode=mode, consumer='project', count=count)
        self.call(root, 'lifecycle', operation='setup')
        request = dict(request='initial', base=None, ops=[], packet=False)
        result = self.call(root, 'resolve', request_id='initial')['result']
        self.ledgers[root].accept(request, result, self.selected(root))

    def publish(self, root, name, ops=None, *, retries=1, code=0, env=None):
        for retry in range(retries):
            request = dict(request=name, base=self.selected(root), ops=ops or [], packet=False)
            self.ledgers[root].record(request)
            # Conflicts are returned as normal publish outcomes by the engine;
            # process errors/timeouts are never silently retried here.
            response = self.call(root, 'publish', request=request, code=code, env=env,
                                 allowed_errors=('conflict:captured-base', 'conflict:expected-publication')
                                 if code == 0 else ())
            if code != 0:
                return response
            if response.get('status') == 'error':
                self.note('semantic-conflict', request=request, response=response, retry=retry)
                continue
            result = response['result']
            if result.get('result', {}).get('outcome') == 'accepted':
                self.ledgers[root].accept(request, result['result'], result['selected'])
                return response
            self.note('semantic-publication-nonaccept', request=request, response=response, retry=retry)
            # Only an explicit semantic conflict gets another captured base.
            require(result.get('status') == 'conflict' or result.get('result', {}).get('outcome') == 'conflict',
                    'unexpected-publication-nonaccept')
        raise RuntimeError('semantic-retry-bound')

    def pack(self, root, codec='zstd', **fields):
        options = dict(operation='build', codec=codec, include_attempts=True, cleanup=False, force=True)
        options.update(fields)
        return self.call(root, 'packing', **options)

    def cleanup(self, root, *, max_batches=160, require_complete=True):
        """A bounded pass; all batches retained. Not equivalent to two-pass gate."""
        batches = []
        for _ in range(max_batches):
            row = self.call(root, 'packing', operation='cleanup')
            result = row['result']
            require(type(result.get('candidates')) is int and 0 <= result['candidates'] <= 1000,
                    'cleanup-candidate-counter-required')
            require(type(result.get('batch_ns')) is int and result['batch_ns'] >= 0, 'cleanup-work-clock-required')
            if result['batch_ns'] > 50_000_000:
                self.note('cleanup-cooperative-target-miss', result=result,
                          qualification='native calls are indivisible; retain overrun')
            batches.append(result)
            if result['status'] == 'complete':
                return batches
        if require_complete:
            raise RuntimeError('cleanup-batch-watchdog')
        return batches

    def drain_two_passes(self, root):
        passes = []
        for number in range(2):
            # One maintenance invocation per pass. Do not relabel 160 batches
            # as a single pass to conceal a missed two-pass drainage target.
            batches = self.cleanup(root, max_batches=1, require_complete=False)
            pending = self.a.backlog(root)  # independent reachability/census, not journal text
            require({'unprotected', 'protected', 'excluded'} <= set(pending), 'complete-backlog-categories')
            require(all(type(pending[key]) is int and pending[key] >= 0 for key in
                        ('unprotected', 'protected', 'excluded')), 'nonnegative-backlog-counters')
            passes.append(dict(number=number+1, batches=batches, backlog=pending))
        self.note('two-drainage-passes', root=str(root), passes=passes)
        require(passes[-1]['backlog']['unprotected'] == 0, 'unprotected-backlog-after-two-passes')
        return passes

    def verify(self, root, checkpoint, *, mapped_keys=()):
        self.check_time()
        self.a.quiesce(root)  # proves no active engine I/O, does not revoke held readers
        inputs = self.ledgers[root].oracle_inputs()
        proof = self.oracle(root=root, checkpoint=checkpoint, final_pin=self.final_pin,
                            known_requests=inputs, required_mapped_keys=list(mapped_keys))
        self.note('independent-full-oracle', root=str(root), checkpoint=checkpoint, proof=proof)
        require(proof.get('status') == 'passed' and proof.get('final_pin') == self.final_pin,
                'full-independent-oracle-required')
        require(proof.get('engine_code_imported') is False and proof.get('engine_audit_used') is False,
                'engine-audit-is-not-independent-oracle')
        require(ORACLE_CHECKS <= set(proof.get('checks', [])), 'incomplete-independent-oracle')
        require(proof.get('actual_H') == len(inputs), 'complete-accepted-history-count')
        require(set(mapped_keys) <= set(proof.get('mapped_keys', [])), 'new-tail-not-authentically-mapped')
        require(proof.get('receipt_sha256') and proof.get('source_unchanged') is True,
                'bound-read-only-oracle-receipt')
        self.oracles.append(proof)
        return proof

    def check_outcomes(self, root):
        for name, expected in list(self.ledgers[root].accepted.items()):
            require(self.call(root, 'resolve', request_id=name)['result'] == expected['result'],
                    'exact-outcome:'+name)

    @contextmanager
    def barrier(self, root, name, action, phase, **fields):
        require('barrier:'+phase in self.a.capabilities, 'unsupported-checkpoint:'+phase)
        control = self.a.control(root, name)
        env = dict(fields.pop('env', {}) or {}, EXP_BARRIER=phase,
                   EXP_READY=control['ready'], EXP_RELEASE=control['release'])
        handle = self.a.spawn(lambda: self.call(root, action, env=env, timeout=60, **fields))
        failure = None
        try:
            self.a.await_ready(control, handle, 10)
            require(self.a.alive(handle), 'barrier-process-ended-before-schedule')
            yield handle, control
        except BaseException as error:
            failure = error
            raise
        finally:
            responses = self.join_callbacks(root, phase, [handle], [control], primary=failure)
            self.note('barrier-completed', root=str(root), phase=phase, response=responses[0])

    def coupled_capture(self, root, label, codec='zstd'):
        """Run on an initialized/discovered lineage; no fixture clone required."""
        before = self.state(root)
        containers = self.containers(before)
        request_id = label+'-tail-during-gap'
        expected = None
        held_since = self.a.monotonic()
        with self.barrier(root, label+'-capture', 'resolve', 'pack-catalog-captured',
                          request_id=request_id) as (handle, control):
            held_since = self.a.monotonic()
            published = self.publish(root, request_id)['result']
            expected = published['result']
            key = 'publications/'+published['selected']['publication']+'.json'
            raw = self.read(root, key, 128*1024)
            # Mapping observation is independently decoded by Adapter/Oracle
            # outside measured engine operations, with an explicit receipt.
            for successor in range(2):
                if successor:
                    self.publish(root, label+'-second-successor')
                prior = self.state(root)
                self.pack(root, codec)
                after = self.state(root)
                transition(prior, after)
                mapping = self.a.mapping(root, key)
                require(mapping.get('mapped') is True and mapping.get('raw_sha256') == sha(raw)
                        and mapping.get('independent') is True and mapping.get('receipt_sha256'),
                        'coupled-tail-mapping-proof')
                self.cleanup(root, max_batches=1, require_complete=False)
                require(self.a.alive(handle), 'old-reader-ended-before-two-successors')
                require(self.read(root, key, 128*1024) == raw, 'old-unmapped-loose-fallback-deleted')
                require(all(self.a.exists(root, p) for p in containers), 'old-container-deleted-while-pinned')
                require(self.call(root, 'resolve', request_id=request_id)['result'] == expected,
                        'new-reader-exact-result')
            self.a.sleep(max(0.0, 16.1-(self.a.monotonic()-held_since)))
            require(self.a.alive(handle) and self.a.monotonic()-held_since > 15,
                    'old-reader-must-survive-more-than-15s')
            # Release/resume occurs in context manager; no deletion assertion
            # after release is made until the reader has positively joined.
        require(handle['response']['result'] == expected, 'old-reader-exact-late-tail-result')
        self.note('old-reader-resumed', root=str(root), key=key,
                  held_seconds=self.a.monotonic()-held_since, exact_result=expected)
        self.drain_two_passes(root)
        # Rolling successors may intentionally share the stable base. A live
        # selected container must survive even after the old reader releases.
        # Actual obsolete-container deletion is a separate mandatory crash case.
        selected_containers = set(self.containers(self.state(root)))
        obsolete = set(containers)-selected_containers
        shared = set(containers) & selected_containers
        require(not any(self.a.exists(root, p) for p in obsolete), 'obsolete-old-container-not-reaped')
        require(all(self.a.exists(root, p) for p in shared), 'shared-live-container-was-reaped')
        hproof = dict(obsolete_containers=sorted(obsolete), shared_live_containers=sorted(shared),
                      actual_old_container_removal_covered_here=bool(obsolete))
        self.note('post-reader-container-reachability', root=str(root), proof=hproof)
        if self.document(root, 'STORE.json', 65536)['mode'] == 'A':
            require(not self.a.exists(root, key), 'A-loose-fallback-not-reaped-after-resume')
        else:
            require(self.read(root, key, 128*1024) == raw, 'B-publication-control-must-remain-loose')
        self.verify(root, label+'-post-resume-reap', mapped_keys=[key])

    def single_publisher_round(self, root, label, name, codec='zstd'):
        """Map a real prior tail while a positively started publisher is held.

        The physical candidate is fully constructed before publication begins.
        These are live process intervals spanning physical activation, not a
        claim that constructor CPU overlaps publisher CPU. No fixture writes.
        """
        require({'barrier:physical-before-selection', 'barrier:candidate-ready'} <=
                set(self.a.capabilities), 'real-physical-and-publisher-barriers-required')
        before = self.state(root)
        tail = before['selected']
        key = 'publications/'+tail['publication']+'.json'
        raw = self.read(root, key, 128*1024)
        unmapped = self.a.mapping(root, key)
        require(unmapped.get('mapped') is False and unmapped.get('independent') is True
                and unmapped.get('receipt_sha256'), 'positive-unmapped-tail-proof-required')
        prior_discovery = before['work'].get('discovery') or {}
        require(type(prior_discovery.get('completed')) is dict
                and prior_discovery.get('interval') is None
                and prior_discovery['completed'].get('selected') != tail,
                'tail-must-follow-completed-physical-cutoff')
        controls = [self.a.control(root, label+'-maintenance'), self.a.control(root, label+'-publisher')]
        request = dict(request=name, base=tail, ops=[], packet=False)
        self.ledgers[root].record(request)
        calls, started, times = [], {}, {}
        primary = None
        def callback(role):
            def observed(event):
                require(type(event) is dict and type(event.get('pid')) is int and event['pid'] > 0
                        and type(event.get('positive_rss')) is int and 0 < event['positive_rss'] <= 2048*MIB
                        and event.get('before_stdin_delivery') is True
                        and type(event.get('observed_ns')) is int and event['observed_ns'] > 0,
                        'positive-native-start-before-stdin-required')
                require(role not in started, 'duplicate-native-start-callback')
                started[role] = copy.deepcopy(event)
                times[role+'-started'] = time.perf_counter_ns()
            return observed
        def environment(control, phase):
            return dict(EXP_BARRIER=phase, EXP_READY=control['ready'], EXP_RELEASE=control['release'])
        try:
            calls.append(self.a.spawn(lambda: self.call(root, 'packing', operation='build', codec=codec,
                include_attempts=True, cleanup=False, force=True, timeout=60,
                env=environment(controls[0], 'physical-before-selection'), on_started=callback('maintenance'))))
            self.a.await_ready(controls[0], calls[0], 10)
            times['maintenance-ready'] = time.perf_counter_ns()
            require('maintenance' in started and self.a.alive(calls[0]), 'real-maintenance-not-held')
            journal = self.document(root, 'PACKING.json')
            target = journal.get('target') or {}
            binding = target.get('physical') or {}
            require(journal.get('status') == 'installed' and journal.get('id') == binding.get('id')
                    and isinstance(binding.get('id'), str) and binding['id'] != before['store']['physical']['id']
                    and binding.get('ordinal') == before['store']['physical']['ordinal']+1
                    and target.get('catalog') != before['store']['catalog']
                    and journal.get('expected_config') == sha(self.read(root, 'STORE.json', 4096)),
                    'physical-ready-must-have-new-installed-mapping')
            plan_raw = self.read(root, 'physical-plans/'+binding['plan']+'.json', 128*1024)
            require(sha(plan_raw) == binding['plan'], 'candidate-work-hash')
            candidate = strict(plan_raw, 128*1024)
            generation = candidate.get('generation') or {}
            require(candidate.get('physical_parent') == journal['expected_config']
                    and generation.get('id') == binding['id'] and generation.get('ordinal') == binding['ordinal']
                    and generation.get('catalog') == target['catalog'], 'candidate-work-binding')
            discovery = candidate.get('discovery') or {}
            require((discovery.get('completed') or {}).get('selected') == tail
                    and discovery.get('interval') is None and discovery.get('deferred') == [],
                    'real-candidate-must-complete-eligible-tail-cutoff')
            calls.append(self.a.spawn(lambda: self.call(root, 'publish', request=request, timeout=60,
                env=environment(controls[1], 'candidate-ready'), on_started=callback('publisher'))))
            self.a.await_ready(controls[1], calls[1], 10)
            times['publisher-ready'] = time.perf_counter_ns()
            require(set(started) == {'maintenance', 'publisher'}
                    and started['maintenance']['pid'] != started['publisher']['pid']
                    and all(self.a.alive(handle) for handle in calls), 'two-real-held-native-processes-required')
            times['both-live-before-any-release'] = time.perf_counter_ns()
            self.note('single-publisher-overlap-observations', root=str(root), label=label,
                      observations_ns=times, clock='time.perf_counter_ns', native_start_events=started)
            require(times['maintenance-started'] < times['maintenance-ready'] < times['publisher-started']
                    < times['publisher-ready']
                    <= times['both-live-before-any-release'], 'positive-publisher-maintenance-interval-required')
            self.note('single-publisher-live-overlap', root=str(root), label=label, request=request,
                      tail=tail, candidate_target=target, native_start_events=started, observations_ns=times,
                      clock='time.perf_counter_ns',
                      positive_interval_seconds=(times['publisher-ready']-times['publisher-started'])/1e9,
                      construction_overlap_claimed=False, observed_before_all_releases=True)
        except BaseException as error:
            primary = error
        finally:
            joined = self.join_callbacks(root, label, calls, controls, primary=primary)
        maintenance, published = joined
        metric = maintenance.get('metrics') or {}
        require(len(metric.get('physical_activation_holds_ns', [])) == 1
                and metric.get('pack_eligible_records', 0) > 0
                and metric.get('rolling_publications_visited', 0) > 0,
                'real-eligible-physical-activation-required')
        value = published['result']
        self.ledgers[root].accept(request, value['result'], value['selected'])
        after = self.state(root)
        # No second maintainer or cleanup runs in this pair. Ordinary no-op
        # publication cannot normalize the physical plan. Bind the selection
        # to the exact installed target inspected before either release.
        require(after['store'] == target, 'activated-store-target-mismatch')
        require(after['work'] == candidate, 'activated-Work-target-mismatch')
        transition(before, after)
        mapped = self.a.mapping(root, key)
        require(mapped.get('mapped') is True and mapped.get('independent') is True
                and mapped.get('receipt_sha256') and mapped.get('raw_sha256') == sha(raw),
                'actual-activated-tail-mapping-proof-required')
        self.note('single-publisher-overlap-joined', root=str(root), label=label,
                  observations_ns=times, clock='time.perf_counter_ns', native_start_events=started, mapping=mapped,
                  selected_target=after['store'], captured_work_sha256=sha(plan_raw), exact_candidate_selected=True,
                  physical_activation_holds_ns=metric['physical_activation_holds_ns'])
        return published

    def concurrent_cycle(self, root, label, publishers, accepted=16, readers=4, codec='zstd',
                         collect_conflicts=False):
        require(publishers in (1, 4) and readers == 4 and accepted >= 16, 'required-concurrency-shape')
        accepted_before = len(self.ledgers[root].accepted)
        self.a.begin_phase(root, label)
        # Four independent native readers remain alive while publishers and
        # maintenance run. Threads without positive native intervals do not count.
        with ExitStack() as stack:
            held = [stack.enter_context(self.barrier(root, f'{label}-reader-{i}', 'resolve',
                    'pack-catalog-captured', request_id='initial'))[0] for i in range(readers)]
            for number in range(accepted):
                if publishers == 1:
                    if number == 0:
                        # Exactly one of the required sixteen is the eligible
                        # seed after bootstrap; fifteen coupled rounds follow.
                        self.publish(root, f'{label}-p0-{number}', retries=1)
                    else:
                        self.single_publisher_round(root, f'{label}-coupled{number}',
                                                    f'{label}-p0-{number}', codec)
                else:
                    # Exactly four attempts/round, all on the SAME captured base.
                    # All four reach candidate-ready before any is released:
                    # actual overlapping prepared work, not serialized frontend.
                    # Exactly one may select; all failed B generations stay charged.
                    require('barrier:candidate-ready' in self.a.capabilities, 'candidate-ready-map-required')
                    base = self.selected(root)
                    requests = [dict(request=f'{label}-round{number}-p{i}', base=base,
                                     ops=[], packet=False) for i in range(4)]
                    controls = [self.a.control(root, f'{label}-round{number}-p{i}') for i in range(4)]
                    for request in requests:
                        self.ledgers[root].record(request)
                    calls = []
                    primary = None
                    candidates = None
                    try:
                        for request, control in zip(requests, controls):
                            env = dict(EXP_BARRIER='candidate-ready', EXP_READY=control['ready'],
                                       EXP_RELEASE=control['release'])
                            calls.append(self.a.spawn(lambda request=request, env=env:
                                self.call(root, 'publish', request=request, env=env, timeout=60,
                                          allowed_errors=('conflict:captured-base', 'conflict:expected-publication'))))
                        for control, handle in zip(controls, calls):
                            self.a.await_ready(control, handle, 10)
                        require(all(self.a.alive(handle) for handle in calls), 'four-native-preparations-not-overlapping')
                        self.pack(root, codec)
                        # Capture exact candidate owner bytes while every native
                        # publisher is positively paused, before any can select.
                        candidates = self.a.prepared_candidates(root, requests) if collect_conflicts else None
                    except BaseException as error:
                        primary = error
                    finally:
                        # Also runs when spawn, ready, pack or owner-capture fails.
                        joined = self.join_callbacks(root, label+'-round'+str(number), calls,
                                                     controls, primary=primary)
                    outcomes, failures = [], []
                    for request, response in zip(requests, joined):
                        try:
                            outcomes.append(response)
                            if response.get('status') == 'ok':
                                value = response['result']
                                self.ledgers[root].accept(request, value['result'], value['selected'])
                            else:
                                self.note('semantic-conflict', request=request, response=response, retry=0)
                        except BaseException as error:
                            failures.append(repr(error))
                    require(not failures, 'concurrent-call-failures:'+repr(failures))
                    require(sum(row.get('status') == 'ok' for row in outcomes) == 1,
                            'same-base-four-prepared-round-must-select-exactly-once')
                    if collect_conflicts:
                        self.collect_failed_round(root, label+'-round'+str(number), requests, outcomes, candidates)
                require(all(self.a.alive(handle) for handle in held), 'four-held-reader-processes-ended')
        require(all(handle['response']['result'] == self.ledgers[root].accepted['initial']['result']
                    for handle in held), 'concurrent-old-reader-outcomes')
        require(len(self.ledgers[root].accepted)-accepted_before == accepted,
                'new-accepted-publication-count')
        self.pack(root, codec)
        self.drain_two_passes(root)
        overlap = self.a.overlap(root, label, publishers=publishers, readers=readers)
        self.note('concurrency-native-intervals', root=str(root), label=label, proof=overlap)
        require(overlap.get('positive_native_intervals') is True
                and overlap.get('independent_reader_processes') == 4
                and overlap.get('publisher_processes') >= publishers
                and overlap.get('maintenance_overlap') is True, 'native-concurrency-not-proven')
        self.note('bounded-concurrency-attempts', label=label, accepted=accepted,
                  attempted=accepted*publishers, failed_prepared_generations=accepted*(publishers-1),
                  raw_failed_prefix_evidence_retained=True, no_implicit_retries=True,
                  failed_corpus_policy='engine-abandon-collect-each-round' if collect_conflicts else 'retain-all')
        self.verify(root, label+'-cycle-boundary')

    def collect_failed_round(self, root, label, requests, outcomes, candidates):
        """SOURCE PLAN ONLY: exact terminal-v2 source/schema adapter pending.

        Reviewed design requires explicit v2 opt-in, definitive preselection
        conflicts, durable attempt fence then atomic active-to-terminal charge,
        all garbage retained and reclaimed=0. Legacy owners, selected/lost
        replies, packets and selection-started attempts cannot use it. Active8,
        terminal4096; small cases forecast48 terminal receipts per A/B root.
        No handler/request/response names are guessed here. V2 GC must refuse.
        """
        raise Unsupported('pending-exact-source-bound-terminal-v2-interface; legacy-abandon-collect-disabled')

    def small(self, root, mode, forecast, codec='zstd'):
        with self.case('S-'+mode+'-'+codec, root, forecast):
            self.initialize(root, mode, 64)
            for n in range(63):
                self.publish(root, f'S-{mode}-{codec}-initial-{n}')
            require(len(self.ledgers[root].accepted) == 64, 'initial-H64-includes-genesis')
            self.pack(root, codec, bootstrap=True)
            self.cleanup(root, max_batches=160)  # initial bootstrap, not the small-tail gate
            self.verify(root, 'N64-H64-bootstrap')
            for cycle, publishers in enumerate((1, 4, 1)):
                self.concurrent_cycle(root, f'S-{mode}-{codec}-c{cycle+1}', publishers, codec=codec,
                                      collect_conflicts=forecast.get('conflict_policy') ==
                                      'engine-abandon-collect-each-round')
            require(len(self.ledgers[root].accepted) == 112, 'three-new-16-publication-cycles')
            self.coupled_capture(root, 'S-'+mode+'-'+codec, codec)
            self.check_outcomes(root)
            self.verify(root, 'small-final')

    def mapping_stress(self, root, forecast, *, count=1000, native_focused=False):
        require((native_focused and 1 <= count <= 64) or (not native_focused and count >= 1000),
                'stress-sample-scope')
        with self.case('X-native-pressure' if native_focused else 'X-Windows1000', root, forecast):
            self.initialize(root, 'A', 10)
            self.pack(root, bootstrap=True)
            self.cleanup(root, max_batches=160)  # initial bootstrap, not a two-call claim
            holds, work_holds, ids, catalogs = [], [], set(), set()
            for number in range(count):
                before = self.state(root)
                selected_before = self.selected(root)
                published = self.publish(root, f'X-new-tail-{number}')['result']
                require(published['selected'] != selected_before, 'authentic-new-semantic-tail-required')
                semantic = self.read(root, 'SELECTED.json', 65536)
                packed = self.pack(root)
                after = self.state(root)
                transition(before, after)
                require(after['store']['physical']['id'] not in ids, 'reused-generation-id')
                require(mapping_identity(after) not in catalogs, 'reused-mapping-catalog')
                ids.add(after['store']['physical']['id'])
                catalogs.add(mapping_identity(after))
                require(self.read(root, 'SELECTED.json', 65536) == semantic, 'physical-work-changed-semantic')
                metric = packed['metrics']
                require(len(metric['physical_activation_holds_ns']) == 1, 'one-mapping-activation-sample')
                holds.extend(metric['physical_activation_holds_ns'])
                work_holds.extend(metric['physical_work_holds_ns'])
                self.drain_two_passes(root)
                key = 'publications/'+published['selected']['publication']+'.json'
                mapped = self.a.mapping(root, key)
                require(mapped.get('mapped') is True and mapped.get('independent') is True,
                        'stress-tail-coverage')
                require(self.call(root, 'resolve', request_id=f'X-new-tail-{number}')['result'] ==
                        published['result'], 'stress-exact-new-outcome')
                self.note('mapping-stress-sample', index=number, before=before['store']['physical'],
                          after=after['store']['physical'], catalog_identity=mapping_identity(after),
                          activation_holds_ns=metric['physical_activation_holds_ns'],
                          work_holds_ns=metric['physical_work_holds_ns'], mapping=mapped)
                if (number+1) % 50 == 0:
                    self.verify(root, f'X-completed-{number+1}')
            self.check_outcomes(root)
            self.verify(root, 'X-final-all-authentic-inputs')
            self.note('stress-summary-unqualified', samples=count, native_focused=native_focused,
                      activation=quantiles(holds), work=quantiles(work_holds) if work_holds else None,
                      activation_gate_met=(len(holds) >= 1000 and quantiles(holds)['p95'] <= 10_000_000
                                           and quantiles(holds)['exploratory_p99'] <= 25_000_000
                                           and max(holds) <= 100_000_000),
                      qualification='native <=64 samples cannot pass Windows >=1000 requirement')
