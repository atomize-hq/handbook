"""NEW Linux Driver revision. Static source only until parent control release.

Actual ELF observation precedes request delivery. Exactly one call owner drains
pipes/reaps; other threads use identity-bound opaque tokens and completion events.
The accepted outer guard remains the descendant/group closure authority.
"""
import base64
import copy
import fcntl
import json
import os
from pathlib import Path
import selectors
import signal
import subprocess
import sys
import threading
import time
import uuid

import native_guard as guard
from native_driver import ACCEPTED_GUARD, owner
from native_resources_v2 import canonical, confined, digest_bytes, need, regular_read, strict

READERS = frozenset({'resolve', 'point', 'inspect', 'lookup-bench'})


class Driver:
    def __init__(self, executable, output, *, expected_sha256, suite_pin,
                 binary_source_pin, reservations, historical_policy=None, start_observer=None):
        need(__debug__ and sys.platform == 'linux', 'native-linux-unoptimized-python-required')
        self.owner = owner()
        need(guard.digest(Path(__file__).with_name('native_driver.py')) ==
             'dfe8ef569b4981a76e199a2763de8a3cf9988bb24785b280fc3662bcd09427ce',
             'accepted-owner-source-unchanged')
        need(self.owner['source_pin'] == suite_pin, 'suite-versus-guard')
        self.suite_pin, self.binary_source_pin = suite_pin, binary_source_pin
        self.executable = Path(executable)
        need(self.executable.is_absolute() and self.executable.resolve() == self.executable,
             'exact-canonical-executable')
        if historical_policy is None:
            need(binary_source_pin == suite_pin, 'candidate-source-pin-equality')
            need(self.executable.is_relative_to(guard.ROOT/'target') or
                 self.executable.is_relative_to(guard.ROOT/'source'), 'owned-candidate-binary')
            self.role = 'candidate'
        else:
            need(binary_source_pin != suite_pin, 'historical-source-must-remain-distinct')
            self.role = 'historical'
        self.policy = historical_policy
        self.binary_sha256 = expected_sha256
        self.verify_binary()
        need(regular_read(self.executable, self.executable.stat().st_size)[:4] == b'\x7fELF', 'native-ELF-only')
        self.output = confined(guard.ROOT/'evidence', Path(output))
        need(self.output.parent.is_dir() and not self.output.exists(), 'fresh-existing-owned-event-parent')
        self.reservations = reservations
        key = 'event-file-'+uuid.uuid4().hex
        reservations.reserve_evidence(key, 4096, 1)
        with self.output.open('xb'):
            pass
        reservations.settle(key)
        self.lock, self.registry = threading.RLock(), {}
        self.events, self.last = [], None
        self.start_observer = start_observer

    def verify_binary(self):
        need(guard.digest(guard.__file__) == ACCEPTED_GUARD, 'accepted-guard-unchanged')
        need(guard.digest(self.executable) == self.binary_sha256, 'executable-bytes-changed')

    def append(self, row, state):
        raw = canonical(row)+b'\n'
        with self.lock:
            need(state['written']+len(raw) <= state['evidence_bound'], 'event-reservation-exhausted')
            with self.output.open('ab') as stream:
                stream.write(raw)
                stream.flush()
            state['written'] += len(raw)

    def _identity(self, state, *, live=True):
        row = guard.process(state['pid'])
        need(row is not None and row['start'] == state['start'] and row['pgid'] == os.getpgrp(),
             'owned-native-identity')
        if live:
            need(row['state'] != 'Z', 'native-already-exited')
        return row

    def _signal_locked(self, state, sig, reason):
        need(not state['closed'], 'released-native-token')
        row = self._identity(state)
        signal.pidfd_send_signal(state['pidfd'], sig)
        state['signals'].append(dict(signal=int(sig), reason=reason, monotonic_ns=time.perf_counter_ns(),
                                     pid=state['pid'], process_start_ticks=row['start']))

    def _state(self, token):
        need(isinstance(token, str) and token in self.registry, 'foreign-or-forged-native-token')
        return self.registry[token]

    def interrupt(self, token, kind):
        need(kind in ('cancel', 'death'), 'closed-interrupt-kind')
        with self.lock:
            state = self._state(token)
            need(state['action'] in READERS and not state['closed'], 'live-reader-control-only')
            need(state['intentional'] in (None, kind), 'conflicting-interrupt')
            if state['intentional'] is None:
                self._signal_locked(state, signal.SIGTERM if kind == 'cancel' else signal.SIGKILL,
                                    'registered-reader-'+kind)
                state['intentional'] = kind
                state['interrupt_ns'] = time.perf_counter_ns()
        return self.wait(token, 5)

    def wait(self, token, timeout):
        with self.lock:
            state = self._state(token)
        need(state['done'].wait(max(0, min(timeout, self.owner['absolute_deadline']-time.time(),
                                         guard.CUTOFF-time.time()))), 'native-owner-completion-timeout')
        with self.lock:
            need(state['closed'] and state['row'] is not None, 'native-incomplete-owner-receipt')
            return copy.deepcopy(state['row'])

    def live(self, token):
        with self.lock:
            state = self._state(token)
            if state['closed']:
                return False
            try:
                self._identity(state)
                return True
            except RuntimeError:
                return False

    def controls(self):
        with self.lock:
            return {k: dict(closed=v['closed'], action=v['action'], pid=v['pid'],
                            process_start_ticks=v['start'], callback_joined=v.get('callback_joined', False),
                            resources_closed=v.get('resources_closed', False))
                    for k, v in self.registry.items()}

    def close_owned(self, reason):
        with self.lock:
            active = [v for v in self.registry.values() if not v['closed']]
            for state in active:
                try:
                    self._signal_locked(state, signal.SIGKILL, reason)
                except (ProcessLookupError, RuntimeError) as error:
                    state['violations'].append('close-owned:'+repr(error))
        for state in active:
            need(state['done'].wait(5), 'owned-call-owner-not-joined')

    def call(self, root, action, *, timeout=120, code=0, env=None, on_started=None, **fields):
        root = confined(guard.ROOT/'fixtures', Path(root))
        self.verify_binary()
        payload = dict(root=str(root), action=action, **fields)
        encoded = json.dumps(payload, separators=(',', ':'), allow_nan=False).encode()
        need(len(encoded) <= 4*1024**2, 'engine-input-bound')
        historical_invocation = self.policy(payload, env, code) if self.policy is not None else None
        environment = {k: v for k, v in os.environ.items() if not k.startswith('EXP_')}
        environment['EXP_NAVIGATION'] = '4'
        if env is not None:
            need(type(env) is dict and all(isinstance(k, str) and isinstance(v, str)
                 and k.startswith('EXP_') and '\0' not in k+v for k, v in env.items()), 'closed-engine-env')
            environment.update(env)
        deadline = min(time.time()+timeout, self.owner['absolute_deadline'], guard.CUTOFF)
        need(deadline > time.time(), 'fixed-deadline')
        limit = 256*1024**2 if action in READERS else 2*1024**3
        token = uuid.uuid4().hex
        # Actual pipe capacities are known BEFORE prospective admission/launch.
        pairs = [os.pipe() for _ in range(4)]
        (gate_r, gate_w), (input_r, input_w), (out_r, out_w), (err_r, err_w) = pairs
        all_fds = {fd for pair in pairs for fd in pair}
        process = watcher = callback = None
        selector = selectors.DefaultSelector()
        stop = threading.Event()
        state = dict(token=token, action=action, written=0, closed=False, row=None, done=threading.Event(),
                     intentional=None, signals=[], violations=[], callback_joined=True, stdin_delivered=False,
                     created_ns=time.perf_counter_ns(), peak=0, memory_samples=0)
        stdout, stderr = bytearray(), bytearray()
        profile = None

        def close_fd(fd):
            if fd in all_fds:
                os.close(fd)
                all_fds.remove(fd)

        def terminate(reason):
            with self.lock:
                if process is not None and not state['closed'] and 'pidfd' in state:
                    try:
                        self._signal_locked(state, signal.SIGKILL, reason)
                    except (ProcessLookupError, RuntimeError, KeyError):
                        # This pidfd was opened on our Popen child, even if its
                        # first /proc observation failed before start was set.
                        try:
                            signal.pidfd_send_signal(state['pidfd'], signal.SIGKILL)
                        except ProcessLookupError:
                            pass

        recorded = {'stdout': 0, 'stderr': 0}
        def flush_raw(stream, force=False):
            target = stdout if stream == 'stdout' else stderr
            while len(target)-recorded[stream] >= 65536 or force and recorded[stream] < len(target):
                end = min(len(target), recorded[stream]+65536)
                block = target[recorded[stream]:end]
                self.append(dict(kind='native-raw-chunk', call=token, stream=stream,
                                 offset=recorded[stream], bytes_b64=base64.b64encode(block).decode()), state)
                recorded[stream] = end

        def monitor():
            try:
                while not stop.is_set():
                    now = time.time()
                    row = guard.process(process.pid)
                    if row is not None and row['state'] != 'Z':
                        need(row['start'] == state['start'] and row['rss'] is not None and
                             row['hwm'] is not None, 'live-memory-identity-required')
                        state['memory_samples'] += 1
                        state['peak'] = max(state['peak'], row['hwm'])
                        if row['rss'] > limit:
                            state['violations'].append('reader-or-checker-rss-censor')
                            terminate('rss')
                    if now >= deadline:
                        if 'time-censor' not in state['violations']:
                            state['violations'].append('time-censor')
                        terminate('deadline')
                    if state['intentional'] == 'cancel' and time.perf_counter_ns()-state['interrupt_ns'] > 500_000_000:
                        terminate('registered-cancel-escalation')
                    stop.wait(.01)
            except BaseException as error:
                state['violations'].append('memory-observer:'+repr(error))
                terminate('memory-observer-failure')

        failure = None
        try:
            tail = sum(fcntl.fcntl(fd, fcntl.F_GETPIPE_SZ) for fd in (out_r, err_r))
            profile = self.reservations.reserve_call(token, payload, env, tail)
            raw_bound = profile['stdout_bytes']+profile['stderr_bytes']+tail
            state['evidence_bound'] = 2*raw_bound+profile['event_bytes']
            process = subprocess.Popen([sys.executable, str(Path(guard.__file__).resolve()), '--child',
                                        str(gate_r), str(self.executable)], stdin=input_r, stdout=out_w,
                                       stderr=err_w, env=environment, close_fds=True, pass_fds=(gate_r,))
            for fd in (gate_r, input_r, out_w, err_w):
                close_fd(fd)
            state['pid'], state['pidfd'] = process.pid, os.pidfd_open(process.pid)
            initial = guard.process(process.pid)
            need(initial is not None and initial['state'] != 'Z' and initial['rss'] is not None and
                 initial['hwm'] is not None and 0 < initial['rss'] <= limit and
                 initial['pgid'] == os.getpgrp(), 'positive-owned-blocked-rss')
            state['start'], state['peak'] = initial['start'], initial['hwm']
            state['initial_blocked_rss'] = initial['rss']
            with self.lock:
                self.registry[token] = state
            watcher = threading.Thread(target=monitor, name='native-memory-'+token, daemon=False)
            watcher.start()
            os.write(gate_w, b'G')
            state['resumed_ns'] = time.perf_counter_ns()
            close_fd(gate_w)
            # No stdin byte exists in the child pipe during this bounded exec wait.
            while True:
                need(time.time() < deadline, 'deadline-before-actual-exec')
                row = self._identity(state)
                try:
                    actual = os.readlink(f'/proc/{process.pid}/exe')
                except FileNotFoundError:
                    actual = None
                if actual == str(self.executable):
                    need(row['rss'] is not None and row['hwm'] is not None and
                         0 < row['rss'] <= limit, 'actual-ELF-positive-rss')
                    # Compare live image inode as well as text; path reuse refuses.
                    live_stat, bound_stat = os.stat(f'/proc/{process.pid}/exe'), self.executable.stat()
                    need((live_stat.st_dev, live_stat.st_ino) == (bound_stat.st_dev, bound_stat.st_ino),
                         'actual-ELF-inode-binding')
                    self.verify_binary()
                    state['observed_ns'] = time.perf_counter_ns()
                    state['actual_exec_rss'] = row['rss']
                    break
                time.sleep(.001)
            event = dict(pid=process.pid, process_start_ticks=state['start'], pgid=os.getpgrp(),
                         control_token=token, guard_nonce=self.owner['nonce'], resumed_ns=state['resumed_ns'],
                         observed_ns=state['observed_ns'], positive_rss=state['actual_exec_rss'],
                         before_stdin_delivery=True, executable_sha256=self.binary_sha256,
                         suite_pin=self.suite_pin, binary_source_pin=self.binary_source_pin,
                         binary_role=self.role, rss_limit=limit, input=copy.deepcopy(payload), env=copy.deepcopy(env),
                         call_owner_thread=threading.get_ident())
            self.append(dict(kind='native-positive-start', **event), state)
            callbacks = [x for x in (self.start_observer, on_started) if x is not None]
            if callbacks:
                errors = []
                def invoke():
                    try:
                        for hook in callbacks:
                            hook(copy.deepcopy(event))
                    except BaseException as error:
                        errors.append(error)
                callback = threading.Thread(target=invoke, name='native-start-'+token, daemon=False)
                callback.start()
                callback.join(max(0, min(.5, deadline-time.time())))
                state['callback_joined'] = not callback.is_alive()
                need(state['callback_joined'], 'start-callback-timeout-before-stdin')
                need(not errors, 'start-callback-error:'+repr(errors))
            need(time.time() < deadline and not state['violations'], 'deadline-or-censor-before-stdin')
            self._identity(state)
            for fd in (input_w, out_r, err_r):
                os.set_blocking(fd, False)
            selector.register(input_w, selectors.EVENT_WRITE, 'stdin')
            selector.register(out_r, selectors.EVENT_READ, 'stdout')
            selector.register(err_r, selectors.EVENT_READ, 'stderr')
            offset, kill_started = 0, None
            while selector.get_map():
                if time.time() >= deadline and kill_started is None:
                    state['violations'].append('time-censor')
                    terminate('io-deadline')
                    kill_started = time.monotonic()
                if state['intentional'] is not None or state['violations']:
                    kill_started = kill_started or time.monotonic()
                need(kill_started is None or time.monotonic()-kill_started < 5,
                     'post-kill-pipes-not-closed-full-raw-incomplete')
                for key, _ in selector.select(.01):
                    fd, stream = key.fd, key.data
                    if stream == 'stdin':
                        try:
                            sent = os.write(fd, encoded[offset:offset+65536])
                            state.setdefault('stdin_ns', time.perf_counter_ns())
                            state['stdin_delivered'] = True
                            offset += sent
                        except BrokenPipeError:
                            offset = len(encoded)
                        if offset == len(encoded):
                            selector.unregister(fd)
                            close_fd(fd)
                        continue
                    try:
                        block = os.read(fd, 16384)
                    except BlockingIOError:
                        continue
                    if not block:
                        selector.unregister(fd)
                        close_fd(fd)
                        continue
                    target = stdout if stream == 'stdout' else stderr
                    target.extend(block)
                    flush_raw(stream)
                    if len(target) > profile[stream+'_bytes']:
                        if 'output-bound:'+stream not in state['violations']:
                            state['violations'].append('output-bound:'+stream)
                        terminate('output-bound')
                        kill_started = kill_started or time.monotonic()
                    need(len(stdout)+len(stderr) <= raw_bound, 'pipe-tail-reservation-exhausted-full-raw-incomplete')
            process.wait(timeout=max(.001, min(5, deadline-time.time())))
        except BaseException as error:
            failure = repr(error)
            state['violations'].append('call-owner:'+failure)
            terminate('call-owner-failure')
        finally:
            if process is not None:
                if process.poll() is None:
                    terminate('call-finally')
                    try:
                        process.wait(timeout=5)
                    except subprocess.TimeoutExpired:
                        state['violations'].append('engine-root-not-reaped')
                stop.set()
                if watcher is not None:
                    watcher.join(5)
                if callback is not None and callback.is_alive():
                    callback.join(.1)
                    state['callback_joined'] = not callback.is_alive()
                if watcher is not None and watcher.is_alive():
                    state['violations'].append('engine-monitor-not-joined')
                if not state['callback_joined']:
                    state['violations'].append('python-start-callback-not-joined')
                # Setup/callback exceptions still retain every byte already in
                # the bounded pipes. No second thread reads or reaps the child.
                for fd, target, name in ((out_r, stdout, 'stdout'), (err_r, stderr, 'stderr')):
                    if fd not in all_fds:
                        continue
                    os.set_blocking(fd, False)
                    until = time.monotonic()+1
                    eof = False
                    while time.monotonic() < until:
                        try:
                            block = os.read(fd, 16384)
                        except BlockingIOError:
                            time.sleep(.001)
                            continue
                        if not block:
                            eof = True
                            break
                        target.extend(block)
                        if profile is not None and len(stdout)+len(stderr) > raw_bound:
                            state['violations'].append('pipe-tail-reservation-exhausted-full-raw-incomplete')
                            break
                    if not eof:
                        state['violations'].append('raw-pipe-eof-not-observed:'+name)
            selector.close()
            for fd in list(all_fds):
                close_fd(fd)
            if process is not None and profile is not None:
                if historical_invocation is not None and historical_invocation['expected_error'] is not None:
                    try:
                        observed = strict(bytes(stdout), profile['stdout_bytes'])
                        need(observed.get('status') == 'error' and
                             observed.get('error') == historical_invocation['expected_error'],
                             'historical-exact-refusal-mismatch')
                    except BaseException as error:
                        state['violations'].append('historical-refusal:'+repr(error))
                row = dict(input=payload, env=copy.deepcopy(env), executable=str(self.executable),
                           executable_sha256=self.binary_sha256, source_pin=self.suite_pin,
                           suite_pin=self.suite_pin, binary_source_pin=self.binary_source_pin,
                           binary_role=self.role, guard_sha256=ACCEPTED_GUARD,
                           guard_nonce=self.owner['nonce'], claim_id=self.owner['claim_id'], call_token=token,
                           exit=process.returncode, stdout=bytes(stdout).decode(errors='replace'),
                           stderr=bytes(stderr).decode(errors='replace'), stdout_sha256=digest_bytes(stdout),
                           stderr_sha256=digest_bytes(stderr), violations=state['violations'],
                           initial_blocked_rss=state.get('initial_blocked_rss'),
                           actual_exec_rss=state.get('actual_exec_rss'), observed_peak_rss=state['peak'],
                           rss_limit=limit, memory_sample_count=state['memory_samples'],
                           process_start_ticks=state.get('start'), pid=process.pid,
                           resumed_ns=state.get('resumed_ns'), observed_ns=state.get('observed_ns'),
                           stdin_ns=state.get('stdin_ns'), stdin_delivered=state['stdin_delivered'],
                           done_ns=time.perf_counter_ns(), intentional_interrupt=state['intentional'],
                           signals=state['signals'], python_callback_joined=state['callback_joined'],
                           engine_root_reaped=process.returncode is not None,
                           engine_monitor_joined=watcher is None or not watcher.is_alive(),
                           historical_invocation=historical_invocation,
                           stage_descendant_closure_authority='accepted-outer-guard-receipt-required')
                # Resource/control censors cannot be interpreted as semantic exit2.
                with self.lock:
                    state['closed'] = True
                    if 'pidfd' in state and (watcher is None or not watcher.is_alive()):
                        os.close(state['pidfd'])
                    state['resources_closed'] = (process.returncode is not None and
                                                 (watcher is None or not watcher.is_alive()))
                try:
                    flush_raw('stdout', True)
                    flush_raw('stderr', True)
                    self.append(dict(kind='native-call', **row), state)
                    self.reservations.settle(token)
                    with self.lock:
                        self.last = row
                        self.events.append(row)
                        state['row'] = row
                finally:
                    state['done'].set()
            elif profile is not None:
                self.reservations.settle(token)
        need(process is not None and state['row'] is not None, 'no-native-call-receipt:'+str(failure))
        row = state['row']
        if process.returncode != code or row['violations']:
            raise AssertionError(row)
        # Explicit injected86 and identity-registered death need not emit JSON.
        if not stdout and code == 86:
            return None
        return strict(bytes(stdout), profile['stdout_bytes'])

    @staticmethod
    def head(root):
        return strict(regular_read(confined(guard.ROOT/'fixtures', Path(root)/'SELECTED.json'), 65536), 65536)

    def publish(self, root, name, ops=None, **fields):
        return self.call(root, 'publish', request=dict(request=name, base=self.head(root),
                         ops=ops or [], packet=False), **fields)
