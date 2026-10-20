"""Raw append-only engine observations inside the single shared stage Job."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import threading
import time
from job_guard import FUNCTIONAL, NT, K, w, rss, sha_file

READERS = {'resolve', 'point', 'inspect', 'lookup-bench'}


class Driver:
    def __init__(self, executable, output):
        assert os.environ.get('FILE_NATIVE_STAGE_JOB') == '1', 'guarded-stage-required'
        self.executable = Path(executable)
        self.binary_sha256 = sha_file(executable)
        self.output = Path(output)
        self.output.parent.mkdir(parents=True, exist_ok=True)
        self.lock = threading.Lock()
        self.last = None

    def call(self, root, action, *, timeout=120, code=0, env=None, **fields):
        assert time.time() < FUNCTIONAL, 'functional-cutoff'
        payload = dict(root=str(root), action=action, **fields)
        encoded = json.dumps(payload, separators=(',', ':')).encode()
        assert len(encoded) <= 4 * 1024**2
        environment = {k: v for k, v in os.environ.items() if not k.startswith('EXP_')}
        environment['EXP_NAVIGATION'] = '4'
        if env:
            environment.update(env)
        limit = (256 * 1024**2) if action in READERS else (2 * 1024**3)
        started = time.perf_counter_ns()
        process = subprocess.Popen([str(self.executable)], stdin=subprocess.PIPE,
                                   stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                   env=environment, close_fds=True, creationflags=0x4)
        initial_rss = rss(process.pid)
        if initial_rss is None or initial_rss[0] <= 0 or initial_rss[0] > limit:
            process.kill()
            process.wait(timeout=5)
            raise RuntimeError('suspended-engine-rss-refusal')
        stop = threading.Event()
        peak = [initial_rss[1]]
        missing = [0]
        violations = []
        def monitor():
            while not stop.is_set():
                value = rss(process.pid)
                if value is None:
                    missing[0] += 1
                    if process.poll() is None:
                        violations.append('unreadable-live-engine-rss')
                        process.kill()
                        return
                else:
                    peak[0] = max(peak[0], value[1])
                    if value[0] > limit:
                        violations.append('reader-or-checker-rss-censor')
                        process.kill()
                        return
                stop.wait(.01)
        watcher = threading.Thread(target=monitor, daemon=True)
        watcher.start()
        if NT.NtResumeProcess(w.HANDLE(process._handle)) != 0:
            process.kill()
            process.wait(timeout=5)
            stop.set()
            watcher.join(timeout=1)
            raise RuntimeError('engine-resume-failed')
        try:
            stdout, stderr = process.communicate(encoded, timeout=min(timeout, max(.001, FUNCTIONAL-time.time())))
        except subprocess.TimeoutExpired:
            process.kill()
            stdout, stderr = process.communicate()
            violations.append('time-censor')
        finally:
            stop.set()
            watcher.join(timeout=1)
        row = dict(input=payload, executable=str(self.executable),
                   executable_sha256=self.binary_sha256, env=env,
                   spawn_total_ns=time.perf_counter_ns()-started,
                   exit=process.returncode, stdout=stdout.decode(), stderr=stderr.decode(),
                   initial_suspended_rss=initial_rss[0], observed_peak_rss=peak[0],
                   missing_rss_samples=missing[0], rss_limit=limit, violations=violations)
        response = json.loads(stdout) if stdout else None
        if response and response.get('peak_memory_bytes') is not None and response['peak_memory_bytes'] > limit:
            row['violations'].append('reported-process-peak-rss-censor')
        with self.lock:
            with self.output.open('a', encoding='utf-8') as output:
                output.write(json.dumps(row, separators=(',', ':'))+'\n')
                output.flush()
            self.last = row
        assert process.returncode == code and not row['violations'], row
        return response

    @staticmethod
    def head(root):
        return json.loads((Path(root)/'SELECTED.json').read_bytes())

    def publish(self, root, name, ops=None, **kwargs):
        return self.call(root, 'publish', request=dict(request=name, base=self.head(root),
                         ops=ops or [], packet=False), **kwargs)
