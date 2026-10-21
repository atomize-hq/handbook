"""Native shared-core adapter. No parser fork, invented command or implicit GO.

Independent mapping/backlog/oracle and versioned mutation constructors are
injected ONLY through an exact source-bound extension manifest. Missing review
or capability refuses before the case writes. This is source-only preparation.
"""
import base64
import copy
import inspect
import os
from pathlib import Path
import threading
import time
import uuid
from contextlib import contextmanager

import native_guard as guard
from native_driver import ACCEPTED_GUARD
from native_driver_v3 import READERS
from native_resources_v3 import canonical, confined, digest_bytes, inventory, need, regular_read


class NativeAdapter:
    def __init__(self, driver, binding, *, hooks=None):
        # Capsule verifies these exact source files before constructing us.
        self.driver, self.binding = driver, binding
        self.budget = driver.reservations
        self.hooks = hooks or {}
        self.capabilities = frozenset(binding['capabilities'])
        need(not self.capabilities or binding['capabilities_reviewed'] is True, 'unreviewed-capabilities')
        self.lock = threading.RLock()
        self.callbacks, self.threads, self.calls, self.contexts = [], {}, [], {}
        self.control_sets, self.phase, self.case_roots, self.drivers = {}, {}, {}, [driver]
        self.historical_phases, self.historical_seen = {}, set()
        driver.start_observer = self.started

    @staticmethod
    def wall_time():
        return time.time()

    @staticmethod
    def monotonic():
        return time.monotonic()

    def sleep(self, seconds):
        need(0 <= seconds <= max(0, min(guard.CUTOFF, self.driver.owner['absolute_deadline'])-time.time()),
             'sleep-crosses-fixed-deadline')
        time.sleep(seconds)

    def preflight(self, case_id, root, forecast, final_pin):
        need(final_pin == self.binding['final_pin'] == self.driver.suite_pin, 'exact-final-pin')
        version = self.binding['case_versions'][case_id]
        need(version['status'] == 'reviewed-source-map' and version['final_pin'] == final_pin and
             version['required_checks'] and version['source_sha256'] in self.binding['source_hashes'],
             'missing-reviewed-final-version-map:'+case_id)
        self.budget.preflight(case_id, root, forecast)
        with self.lock:
            self.case_roots[str(root)] = case_id
        return dict(go=True, case_id=case_id, root=str(root), final_pin=final_pin,
                    reader_limit=256*1024**2, checker_limit=2*1024**3, default_payloads=True,
                    heavy_token=self.driver.owner['nonce'], host_guard=self.driver.owner['claim_id'],
                    binary_sha256=self.driver.binary_sha256, binary_source_pin=self.driver.binary_source_pin,
                    source_manifest_sha256=self.binding['source_manifest_sha256'],
                    adapter_sha256=self.binding['adapter_sha256'], guard_sha256=ACCEPTED_GUARD)

    def before_call(self, root, action, role, timeout):
        root = confined(guard.ROOT/'fixtures', Path(root))
        need(str(root) in self.case_roots, 'unadmitted-native-case-root')
        need(role == ('reader' if action in READERS else 'checker') and timeout > 0,
             'independent-action-role-and-time')
        self.driver.verify_binary()
        self.budget.check({})
        with self.lock:
            self.contexts[threading.get_ident()] = dict(root=str(root), action=action,
                                                       phase=self.phase.get(str(root)))

    def started(self, event):
        with self.lock:
            owner_thread = event['call_owner_thread']
            context = self.contexts.get(owner_thread)
            need(context and context['root'] == event['input']['root'] and
                 context['action'] == event['input']['action'], 'start-versus-before-call')
            record = dict(event, phase=context['phase'], ready_ns=None)
            self._native_driver(record)
            self.calls.append(record)
            handle = self.threads.get(owner_thread)
            if handle is not None:
                need(handle.get('native') is None, 'one-native-call-per-spawn-handle')
                handle['native'] = record

    def _native_driver(self, event):
        matches = [driver for driver in self.drivers if driver.binary_sha256 == event['executable_sha256']
                   and driver.binary_source_pin == event['binary_source_pin'] and
                   event['control_token'] in driver.registry]
        need(len(matches) == 1, 'exact-driver-token-binary-source-registry')
        return matches[0]

    def register_historical(self, driver, case_id, phase, root):
        self.validate_historical_context(case_id, phase, root)
        need(self.case_roots.get(str(root)) == case_id and driver.role == 'historical' and
             driver.case_id == case_id and driver.phase == phase, 'historical-live-case-phase-registration')
        with self.lock:
            self.drivers.append(driver)

    def validate_historical_context(self, case_id, phase, root):
        need(self.case_roots.get(str(root)) == case_id and
             self.historical_phases.get(str(root)) == (case_id, phase), 'active-historical-case-phase-required')

    @contextmanager
    def historical_phase(self, case_id, phase, root):
        """Entered by reviewed compatibility extension; a phase cannot be reopened."""
        root = confined(guard.ROOT/'fixtures', Path(root))
        key = (case_id, phase, str(root))
        with self.lock:
            need(self.case_roots.get(str(root)) == case_id and str(root) not in self.historical_phases and
                 key not in self.historical_seen, 'fresh-active-historical-phase')
            self.historical_seen.add(key)
            self.historical_phases[str(root)] = (case_id, phase)
        try:
            yield
        finally:
            with self.lock:
                self.historical_phases.pop(str(root), None)
            errors = []
            for driver in self.drivers:
                if driver.role == 'historical' and driver.case_id == case_id and driver.phase == phase:
                    try:
                        driver.close_owned('historical-phase-finally')
                    except BaseException as error:
                        errors.append(repr(error))
            need(not errors, 'historical-phase-close:'+repr(errors))

    def historical_call(self, driver, case_id, phase, root, action, **options):
        self.validate_historical_context(case_id, phase, root)
        need(driver in self.drivers and driver.case_id == case_id and driver.phase == phase and
             self.case_roots.get(str(root)) == case_id, 'historical-only-active-granted-case')
        self.before_call(root, action, 'reader' if action in READERS else 'checker', options.get('timeout', 120))
        return driver.call(root, action, **options)

    def read(self, root, relative, limit):
        root = confined(guard.ROOT/'fixtures', Path(root))
        need(isinstance(relative, str) and relative and not Path(relative).is_absolute(), 'relative-read')
        return regular_read(confined(Path(root), Path(root)/relative), limit)

    def exists(self, root, relative):
        root = confined(guard.ROOT/'fixtures', Path(root))
        need(not Path(relative).is_absolute(), 'relative-exists')
        path = confined(Path(root), Path(root)/relative)
        return path.exists()

    def file_sha256(self, root, relative):
        root = confined(guard.ROOT/'fixtures', Path(root))
        need(not Path(relative).is_absolute(), 'relative-hash')
        path = confined(Path(root), Path(root)/relative)
        need(path.is_file(), 'regular-hash-file')
        return guard.digest(path)

    def inventory(self, root):
        return inventory(Path(root), hashes=True)

    def control(self, root, name):
        root = confined(guard.ROOT/'fixtures', Path(root))
        need(isinstance(name, str) and name and len(name) <= 160 and '/' not in name and
             '\\' not in name and '..' not in name, 'control-name')
        with self.lock:
            spec = self.budget.case_specs[str(root)]['controls']
            existing = self.control_sets.setdefault(str(root), [])
            need(len(existing) < spec['max_controls'], 'case-control-count-exhausted')
            # Every control is a fresh sibling namespace outside the engine root.
            path = root.parent/('.native-control-'+uuid.uuid4().hex)
            confined(guard.ROOT/'fixtures', path)
            key = 'control-directory-'+uuid.uuid4().hex
            self.budget.reserve_control(key, 0, directories=1)
            path.mkdir(exist_ok=False)
            self.budget.settle(key)
            control = dict(ready=str(path/'ready'), release=str(path/'release'), cancel=str(path/'cancel'))
            existing.append(control)
            return copy.deepcopy(control)

    def _control(self, control):
        matches = [(root, row) for root, rows in self.control_sets.items() for row in rows if row == control]
        need(len(matches) == 1, 'unknown-native-control')
        return matches[0]

    def _write_control(self, control, field, raw):
        root, _ = self._control(control)
        spec = self.budget.case_specs[root]['controls']
        need(len(raw) <= spec['max_file_bytes'], 'control-file-byte-bound')
        path = confined(guard.ROOT/'fixtures', Path(control[field]))
        if path.exists():
            need(regular_read(path, spec['max_file_bytes']) == raw, 'control-rewrite-refused')
            return
        key = 'control-write-'+uuid.uuid4().hex
        self.budget.reserve_control(key, len(raw), files=1)
        with path.open('xb') as stream:
            stream.write(raw)
            stream.flush()
        self.budget.settle(key)

    def release(self, control):
        self._write_control(control, 'release', b'release')

    def signal(self, control, name):
        need(name == 'cancel', 'cancel-control-only')
        self._write_control(control, 'cancel', b'cancel')

    def end_signal(self, control, name):
        need(name == 'cancel', 'cancel-control-only')
        self._control(control)
        path = confined(guard.ROOT/'fixtures', Path(control['cancel']))
        retained = path.with_name('cancel.retained')
        need(path.exists() and not retained.exists(), 'one-retained-cancel-prefix')
        # Same owned control namespace, no bytes removed, no engine state edited.
        path.rename(retained)

    def spawn(self, callback):
        state = dict(response=None, error=None, native=None, error_observed=False)
        def invoke():
            with self.lock:
                self.threads[threading.get_ident()] = state
            try:
                state['response'] = callback()
            except BaseException as error:
                state['error'] = error
            finally:
                with self.lock:
                    self.threads.pop(threading.get_ident(), None)
        state['thread'] = threading.Thread(target=invoke, name='native-case-callback', daemon=False)
        with self.lock:
            self.callbacks.append(state)
        state['thread'].start()
        return state

    @staticmethod
    def alive(handle):
        return handle['thread'].is_alive()

    def wait(self, handle, timeout):
        handle['thread'].join(max(0, min(timeout, guard.CUTOFF-time.time(),
                                         self.driver.owner['absolute_deadline']-time.time())))
        need(not self.alive(handle), 'python-callback-join-timeout')
        if handle['error'] is not None:
            handle['error_observed'] = True
            raise handle['error']
        return handle['response']

    def await_ready(self, control, handle, timeout):
        self._control(control)
        end = min(time.monotonic()+timeout, time.monotonic()+guard.CUTOFF-time.time(),
                  time.monotonic()+self.driver.owner['absolute_deadline']-time.time())
        while time.monotonic() < end:
            need(self.alive(handle), 'callback-ended-before-native-ready')
            native = handle.get('native')
            if native is not None and Path(control['ready']).exists():
                environment = native['env'] or {}
                phase = environment.get('EXP_BARRIER')
                mapping = self.binding['checkpoints'][phase]
                need(mapping['status'] == 'reviewed-source-map' and
                     mapping['final_pin'] == self.binding['final_pin'] and
                     mapping['source_sha256'] in self.binding['source_hashes'], 'ready-checkpoint-source-map')
                need(environment.get('EXP_READY') == control['ready'] and
                     environment.get('EXP_RELEASE') == control['release'], 'native-ready-environment-binding')
                expected = base64.b64decode(mapping['ready_bytes_b64'], validate=True)
                need(len(expected) <= 4096 and regular_read(Path(control['ready']), 4096) == expected,
                     'authentic-exact-native-ready-bytes')
                need(self._native_driver(native).live(native['control_token']), 'ready-process-not-live')
                native['ready_ns'] = time.perf_counter_ns()
                return
            time.sleep(.002)
        raise RuntimeError('native-ready-timeout')

    def begin_phase(self, root, label):
        with self.lock:
            self.phase[str(root)] = label

    def overlap(self, root, label, publishers, readers):
        with self.lock:
            rows = [copy.deepcopy(row) for row in self.calls if row['phase'] == label and
                    row['input']['root'] == str(root)]
        completed = {row['call_token']: row for row in self.driver.events}
        reader_rows = [r for r in rows if r['input']['action'] in READERS and r['ready_ns'] is not None]
        publisher_rows = [r for r in rows if r['input']['action'] == 'publish']
        maint = [r for r in rows if r['input']['action'] == 'packing']
        def interval(row):
            end = completed.get(row['control_token'])
            need(end is not None and end['engine_root_reaped'] and end['engine_monitor_joined'] and
                 end['input_fully_delivered'] and not end['violations'], 'complete-native-overlap-interval')
            return row['ready_ns'] or end['stdin_ns'], end['done_ns']
        rints, pints, mints = [interval(r) for r in reader_rows], [interval(r) for r in publisher_rows], [interval(r) for r in maint]
        need(len({r['pid'] for r in reader_rows}) == readers == 4, 'four-independent-native-reader-PIDs')
        # Active means the source-mapped native ready checkpoint, not wrapper or
        # pre-input starts. Four-publisher cohorts must all reach candidate-ready.
        if publishers == 4:
            cohorts = [r for r in publisher_rows if r['ready_ns'] is not None]
            simultaneous = any(sum(a <= point <= b for a, b in [interval(r) for r in cohorts]) >= 4
                               for point in [r['ready_ns'] for r in cohorts])
        else:
            simultaneous = bool(pints)
        overlap = any(all(a <= m0 and m1 <= b for a, b in rints) and
                      any(p0 < m1 and m0 < p1 for p0, p1 in pints) for m0, m1 in mints)
        # With one publisher the schedule places pack AFTER publish. Native
        # simultaneous publisher/maintenance overlap may genuinely be absent;
        # retain that failed requirement instead of fabricating an interval.
        return dict(positive_native_intervals=simultaneous and overlap,
                    independent_reader_processes=len(reader_rows), publisher_processes=publishers if simultaneous else 0,
                    maintenance_overlap=overlap, intervals=rows,
                    one_publisher_sequential_schedule_cannot_infer_overlap=publishers == 1)

    def interrupt(self, handle, kind):
        native = handle.get('native')
        need(native is not None and self.alive(handle), 'registered-live-native-call-required')
        row = self._native_driver(native).interrupt(native['control_token'], kind)
        expected_signals = {-15, -9} if kind == 'cancel' else {-9}
        need(row['intentional_interrupt'] == kind and row['pid'] == native['pid'] and
             row['process_start_ticks'] == native['process_start_ticks'] and row['exit'] in expected_signals and
             row['violations'] == [] and row['engine_root_reaped'] and row['engine_monitor_joined'] and
             row['python_callback_joined'], 'exact-registered-interrupt-receipt')
        handle['expected_interrupt_row'] = row
        return dict(identity_verified=True, closed=True, native_row=row,
                    cancellation_scope='owned-process-termination; no semantic noncommit fence',
                    pin_release_requires_post_death_engine_cleanup=True)

    def finish_interrupted(self, handle, kind):
        if self.alive(handle):
            handle['thread'].join(max(0, min(5, guard.CUTOFF-time.time())))
        need(not self.alive(handle), 'interrupted-python-callback-not-joined')
        error = handle['error']
        if kind == 'error':
            if error is not None:
                raise error
            return
        expected = handle.get('expected_interrupt_row')
        need(expected is not None and isinstance(error, AssertionError) and len(error.args) == 1 and
             error.args[0] == expected, 'only-exact-owned-interrupt-assertion-is-consumable')
        handle['error_observed'] = True

    def quiesce(self, root):
        active = []
        for row in self.calls:
            if row['input']['root'] == str(root) and self._native_driver(row).live(row['control_token']):
                # Only authentic paused readers are allowed through an oracle.
                need(row['input']['action'] in READERS and row['ready_ns'] is not None,
                     'active-mutation-or-unpaused-reader-during-oracle')
                active.append(row['control_token'])
        return dict(paused_readers=active, no_active_mutation=True)

    def close_case(self, case_id, root):
        # Retain every secondary error; failure cannot skip another join.
        errors = []
        for driver in self.drivers:
            try:
                driver.close_owned('case-finally:'+case_id)
            except BaseException as error:
                errors.append(repr(error))
        for callback in self.callbacks:
            callback['thread'].join(5)
            if self.alive(callback):
                errors.append('python-callback-not-joined')
            if callback['error'] is not None and not callback['error_observed']:
                errors.append('unobserved-python-callback-error:'+repr(callback['error']))
        registries = [driver.controls() for driver in self.drivers]
        closed = all(row['closed'] and row['resources_closed'] for registry in registries for row in registry.values())
        callbacks = all(not self.alive(row) for row in self.callbacks)
        with self.lock:
            self.case_roots.pop(str(root), None)
            self.historical_phases.pop(str(root), None)
        with self.budget.lock:
            # Completed case authority cannot launch a later unadmitted call.
            self.budget.case_specs.pop(str(root), None)
        return dict(processes_closed=closed and not errors, handles_closed=closed and not errors,
                    python_callbacks_joined=callbacks and all(row['callback_joined'] for registry in registries
                                                              for row in registry.values()),
                    errors=errors, registries=registries,
                    callback_errors=[repr(c['error']) for c in self.callbacks if c['error'] is not None],
                    native_guard_receipt='PENDING accepted outer guard final two-empty observations',
                    outer_guard_closure_still_required=True)

    def removal_proof(self, root, before, phase):
        after = self.inventory(root)
        removed = sorted(set(before['files'])-set(after['files']))
        return dict(actually_removed=bool(removed), removed_paths=removed,
                    removed_identities={key: before['files'][key] for key in removed},
                    no_planned_deletion_credit=True, phase=phase,
                    inventory_sha256=digest_bytes(canonical(after)))

    def _hook(self, name, *args, **kwargs):
        hook = self.hooks.get(name)
        need(hook is not None, 'PENDING shared reviewed independent/extension hook:'+name)
        spec = self.binding['hooks'][name]
        need(spec['reviewed'] is True and spec['schema_pin'] == self.binding['final_pin'] and
             guard.digest(spec['path']) == spec['sha256'] and spec['sha256'] in self.binding['source_hashes'],
             'source-bound-shared-hook-required')
        need(Path(inspect.getsourcefile(hook)).resolve() == Path(spec['path']).resolve() and
             hook.__name__ == spec['function'], 'injected-function-versus-reviewed-source')
        return hook(*args, **kwargs)

    def independent_oracle(self, **kwargs):
        return self._hook('oracle', **kwargs)

    def mapping(self, root, key):
        return self._hook('mapping', root, key)

    def backlog(self, root):
        return self._hook('backlog', root)

    def crash_source_binding(self, root, phase):
        return copy.deepcopy(self.binding['crash_constructors'][phase])

    def extend(self, harness, root, spec):
        return self._hook('extend:'+spec['case_id'], harness, root, spec)
