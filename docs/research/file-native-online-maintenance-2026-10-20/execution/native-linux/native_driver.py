"""Thin Linux process adapter. Semantic oracles belong to the shared harness."""
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import threading
import time
import types

import native_guard as guard

READERS = {'resolve', 'point', 'inspect', 'lookup-bench'}
ACCEPTED_GUARD = '4ddf6c15c87a200530c2c5e15d6c5be26ff3251019d20f8dea6ca83948b634f9'


def owner():
    """Bind the calling stage to a live ancestor's nonce/start/category claim."""
    assert guard.digest(guard.__file__) == ACCEPTED_GUARD, 'guard-source-binding'
    parent = os.getppid()
    ancestors = {}
    while parent > 1:
        row = guard.process(parent)
        assert row is not None, 'ancestor-observation'
        ancestors[parent] = row
        parent = row['ppid']
    candidates = []
    for path in (guard.ROOT/'source').glob('guard-owner-*.json'):
        data = json.loads(path.read_bytes())
        row = ancestors.get(data['pid'])
        if row is not None and row['start'] == data['process_start']:
            command = row['cmd'].split()
            if len(command) >= 2 and Path(command[1]) == Path(guard.__file__).resolve():
                candidates.append(data)
    assert len(candidates) == 1, 'one-live-ancestor-guard-required'
    selected = candidates[0]
    assert time.time() < min(selected['absolute_deadline'], guard.CUTOFF)
    assert os.getpgrp() != selected['pid'], 'stage-must-be-owned-child-group'
    return selected


class Driver:
    """Same call/head/publish interface and raw events as the Windows driver.

    Each engine root inherits the outer guard's process group. Its per-call
    watcher enforces reader/checker memory and timeout. The outer accepted guard
    is authoritative for all descendants and positive stage-group closure.
    """
    def __init__(self, executable, output, *, expected_sha256, suite_pin,
                 binary_source_pin=None, role='candidate', event_limit_bytes=8*1024**2):
        self.owner = owner()
        assert self.owner['source_pin'] == suite_pin, 'suite-pin-versus-guard'
        self.suite_pin = suite_pin
        self.binary_source_pin = binary_source_pin or suite_pin
        self.role = role
        self.executable = Path(executable).resolve()
        self.binary_sha256 = guard.digest(self.executable)
        assert self.binary_sha256 == expected_sha256, 'executable-binding'
        self.output = Path(output).resolve()
        assert self.output.is_relative_to(guard.ROOT/'evidence'), 'owned-event-output'
        self.output.parent.mkdir(parents=True, exist_ok=True)
        self.event_limit = event_limit_bytes
        self.lock = threading.Lock()
        self.last = None
        self.events = []

    def call(self, root, action, *, timeout=120, code=0, env=None, **fields):
        root = Path(root).resolve()
        assert root.is_relative_to(guard.ROOT/'fixtures'), 'owned-fixture-only'
        assert guard.digest(self.executable) == self.binary_sha256, 'binary-changed'
        absolute = min(self.owner['absolute_deadline'], guard.CUTOFF, time.time()+timeout)
        assert absolute > time.time(), 'functional-or-stage-cutoff'
        payload = dict(root=str(root), action=action, **fields)
        encoded = json.dumps(payload, separators=(',', ':')).encode()
        assert len(encoded) <= 4*1024**2, 'bounded-engine-input'
        environment = {k: v for k, v in os.environ.items() if not k.startswith('EXP_')}
        environment['EXP_NAVIGATION'] = '4'
        if env:
            environment.update(env)
        limit = 256*1024**2 if action in READERS else 2*1024**3
        read_fd, write_fd = os.pipe()
        started = time.perf_counter_ns()
        process = subprocess.Popen([sys.executable, str(Path(guard.__file__).resolve()),
                                    '--child', str(read_fd), str(self.executable)],
                                   stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE, env=environment,
                                   close_fds=True, pass_fds=(read_fd,))
        os.close(read_fd)
        pidfd = os.pidfd_open(process.pid)
        initial = guard.process(process.pid)
        if (initial is None or initial['state'] == 'Z' or initial['rss'] is None or
                initial['hwm'] is None or not 0 < initial['rss'] <= limit):
            signal.pidfd_send_signal(pidfd, signal.SIGKILL)
            process.wait(timeout=5)
            os.close(write_fd)
            os.close(pidfd)
            raise RuntimeError('blocked-engine-positive-rss-required')
        assert initial['pgid'] == os.getpgrp(), 'engine-must-inherit-outer-group'
        stop = threading.Event()
        peak = [initial['hwm']]
        missing = [0]
        violations = []
        sample_count = [0]

        def kill():
            try:
                signal.pidfd_send_signal(pidfd, signal.SIGKILL)
            except ProcessLookupError:
                pass

        def monitor():
            try:
                while not stop.is_set():
                    row = guard.process(process.pid)
                    if row is None or row['state'] == 'Z':
                        missing[0] += 1
                        if process.poll() is not None:
                            return
                    elif row['start'] != initial['start']:
                        violations.append('engine-pid-identity-changed')
                        kill()
                        return
                    elif row['rss'] is None or row['hwm'] is None:
                        violations.append('unreadable-live-engine-rss')
                        kill()
                        return
                    else:
                        sample_count[0] += 1
                        peak[0] = max(peak[0], row['hwm'])
                        if row['rss'] > limit:
                            violations.append('reader-or-checker-rss-censor')
                            kill()
                            return
                    stop.wait(.01)
            except BaseException as e:
                violations.append('engine-memory-observation:'+repr(e))
                kill()

        watcher = threading.Thread(target=monitor, daemon=True)
        watcher.start()
        try:
            if time.time() >= absolute:
                violations.append('cutoff-before-engine-exec')
                kill()
            else:
                try:
                    os.write(write_fd, b'G')
                except OSError as e:
                    violations.append('engine-launch-pipe:'+repr(e))
                    kill()
            os.close(write_fd)
            try:
                stdout, stderr = process.communicate(encoded, timeout=max(.001, absolute-time.time()))
            except subprocess.TimeoutExpired:
                kill()
                stdout, stderr = process.communicate(timeout=5)
                violations.append('time-censor')
        finally:
            if process.poll() is None:
                kill()
                process.wait(timeout=5)
            stop.set()
            watcher.join(timeout=5)
            if watcher.is_alive():
                # Keep pidfd live for the outstanding monitor; outer stage fails.
                violations.append('engine-monitor-not-joined')
            else:
                os.close(pidfd)
        row = dict(input=payload, executable=str(self.executable),
                   executable_sha256=self.binary_sha256, env=env,
                   source_pin=self.suite_pin, binary_source_pin=self.binary_source_pin,
                   binary_role=self.role, guard_sha256=ACCEPTED_GUARD,
                   guard_nonce=self.owner['nonce'], claim_id=self.owner['claim_id'],
                   spawn_total_ns=time.perf_counter_ns()-started,
                   exit=process.returncode, stdout=stdout.decode(errors='replace'),
                   stderr=stderr.decode(errors='replace'),
                   stdout_sha256=__import__('hashlib').sha256(stdout).hexdigest(),
                   stderr_sha256=__import__('hashlib').sha256(stderr).hexdigest(),
                   initial_blocked_rss=initial['rss'], observed_peak_rss=peak[0],
                   missing_rss_samples=missing[0], memory_sample_count=sample_count[0],
                   rss_limit=limit, violations=violations,
                   engine_root_reaped=process.returncode is not None,
                   engine_monitor_joined=not watcher.is_alive(),
                   stage_descendant_closure_authority='outer-native-guard-receipt')
        try:
            response = json.loads(stdout) if stdout else None
        except (ValueError, UnicodeError) as e:
            row['violations'].append('response-json:'+repr(e))
            row['stdout_hex'] = stdout.hex()
            response = None
        if response and response.get('peak_memory_bytes') is not None and response['peak_memory_bytes'] > limit:
            row['violations'].append('reported-process-peak-rss-censor')
        raw = json.dumps(row, separators=(',', ':'))+'\n'
        with self.lock:
            current = self.output.stat().st_size if self.output.exists() else 0
            assert current+len(raw.encode()) <= self.event_limit, 'event-prewrite-cap'
            with self.output.open('a', encoding='utf-8') as output:
                output.write(raw)
                output.flush()
            self.last = row
            self.events.append(row)
        assert process.returncode == code and not row['violations'], row
        return response

    @staticmethod
    def head(root):
        return json.loads((Path(root)/'SELECTED.json').read_bytes())

    def publish(self, root, name, ops=None, **kwargs):
        return self.call(root, 'publish', request=dict(request=name, base=self.head(root),
                         ops=ops or [], packet=False), **kwargs)


def legacy_driver_module(executable, *, expected_sha256, suite_pin):
    """Explicit bridge for old Driver(base), preserving its events and EXE API.

    Only the candidate executable is admitted by this bridge. Old-client binary
    overrides require a separately bound Driver with compatibility role.
    """
    executable = Path(executable).resolve()
    module = types.ModuleType('driver')
    module.EXE = executable

    class LegacyDriver:
        def __init__(self, base):
            self.base = Path(base).resolve()
            assert self.base.is_relative_to(guard.ROOT/'fixtures')
            self.base.mkdir(parents=True, exist_ok=False)
            relative = self.base.relative_to(guard.ROOT/'fixtures')
            output = guard.ROOT/'evidence'/'legacy-events'/relative/'events.jsonl'
            self.backend = Driver(executable, output, expected_sha256=expected_sha256,
                                  suite_pin=suite_pin)
            self.events = self.backend.events

        def call(self, root, action, *, exe_override=None, **fields):
            assert exe_override is None or Path(exe_override).resolve() == executable, 'unbound-executable-override'
            return self.backend.call(root, action, **fields)

        head = staticmethod(Driver.head)

        def publish(self, root, id, ops=None, packet=False, **fields):
            return self.call(root, 'publish', request=dict(request=id, base=self.head(root),
                             ops=ops or [], packet=packet), **fields)

    module.Driver = LegacyDriver
    module.native_adapter = dict(semantic_decoder_forked=False,
                                 legacy_event_location='owned evidence/legacy-events',
                                 exact_executable_sha256=expected_sha256, suite_pin=suite_pin)
    return module
