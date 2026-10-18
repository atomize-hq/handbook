"""Explicit 1800-second audit-only correction followup; original windows stay closed."""
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT))
from outer import inventory
A = json.loads((ROOT / 'allocation-audit-correction.json').read_bytes())
SOURCE = Path(A['source'])
E = ROOT / 'evidence-audit'
LANE = ROOT / 'audit-lane'
LANE.mkdir(exist_ok=False)
target = LANE / 'build'
exe = target / 'release/file-native-incremental'
commands = [
    ('test', ['cargo', 'test', '--locked', '--workspace', '--target-dir', str(target)], 300),
    ('build', ['cargo', 'build', '--release', '--locked', '--workspace', '--target-dir', str(target)], 300),
    ('strict', [sys.executable, str(SOURCE / 'audit_evidence.py'), '--exe', str(exe), '--base', str(LANE / 'strict'), '--strict'], 900),
]
env = {**os.environ, 'PYTHONPATH': str(SOURCE), 'PYTHONDONTWRITEBYTECODE': '1'}
ok = True
completed = []
for label, argv, cap in commands:
    snap = inventory()
    assert time.time() < A['total_deadline_epoch']
    assert snap['free_bytes'] >= A['reserve_bytes']
    assert max(snap['logical_bytes'], snap['allocated_file_bytes']) < A['logical_cap_bytes'] - 512 * 1024 ** 2
    start = time.time()
    effective_deadline = min(start + cap, A['total_deadline_epoch'])
    reason = None
    with (E / (label + '.stdout')).open('xb') as out, (E / (label + '.stderr')).open('xb') as err:
        p = subprocess.Popen(argv, cwd=SOURCE, env=env, stdout=out, stderr=err, start_new_session=True)
        while p.poll() is None:
            snap = inventory()
            with (E / 'resources.jsonl').open('a') as f:
                f.write(json.dumps(dict(label=label, **snap)) + '\n')
            if time.time() >= effective_deadline:
                reason = 'audit-suite-or-explicit-1800-second-cap'
            elif snap['free_bytes'] < A['reserve_bytes']:
                reason = 'free-space-floor'
            elif max(snap['logical_bytes'], snap['allocated_file_bytes']) > A['logical_cap_bytes'] - 512 * 1024 ** 2:
                reason = 'whole-root-cap-margin'
            if reason:
                os.killpg(p.pid, signal.SIGKILL)
                break
            time.sleep(2)
        p.wait()
        if p.returncode:
            try:
                os.killpg(p.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
    streams = {}
    for kind in ['stdout', 'stderr']:
        data = (E / (label + '.' + kind)).read_bytes()
        streams[kind] = dict(bytes=len(data), sha256=hashlib.sha256(data).hexdigest())
    row = dict(label=label, argv=argv, cwd=str(SOURCE), start_epoch=start, end_epoch=time.time(),
               exit=p.returncode, censored=bool(reason), reason=reason, suite_cap_seconds=cap,
               effective_deadline_epoch=effective_deadline, pin=A['pin'],
               launcher_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(), streams=streams)
    with (E / 'runs.jsonl').open('a') as f:
        f.write(json.dumps(row) + '\n')
    print(json.dumps(row), flush=True)
    completed.append(label)
    if label == 'build' and p.returncode == 0:
        binaries = {}
        for name in ['file-native-incremental', 'project-consumer', 'research-consumer']:
            path = target / 'release' / name
            data = path.read_bytes()
            binaries[name] = dict(path=str(path), bytes=len(data), sha256=hashlib.sha256(data).hexdigest())
        (E / 'binary-pins.json').write_text(json.dumps(binaries, indent=2) + '\n')
    if p.returncode or reason:
        ok = False
        break
(E / 'finished.json').write_text(json.dumps(dict(
    status='passed' if ok else 'stopped', pin=A['pin'], epoch=time.time(), resources=inventory(),
    completed=completed, unperformed=[x[0] for x in commands if x[0] not in completed],
    explicit_correction_start_epoch=A['start_epoch'],
    explicit_correction_deadline_epoch=A['total_deadline_epoch'],
    original_native_window_reopened=False), indent=2) + '\n')
raise SystemExit(0 if ok else 1)
