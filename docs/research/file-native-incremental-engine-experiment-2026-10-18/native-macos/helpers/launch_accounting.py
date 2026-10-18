"""One accounting correction: absolute 1500-second execution cut, 1800 total."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT))
from outer import inventory
parser = argparse.ArgumentParser()
parser.add_argument('--deadline-epoch', required=True, type=float)
args = parser.parse_args()
A = json.loads((ROOT / 'allocation-accounting-correction.json').read_bytes())
assert args.deadline_epoch == A['execution_deadline_epoch'] == A['start_epoch'] + 1500
assert A['total_deadline_epoch'] == A['start_epoch'] + 1800
SOURCE = Path(A['source'])
E = ROOT / 'evidence-accounting'
LANE = ROOT / 'accounting-lane'
LANE.mkdir(exist_ok=False)
target = LANE / 'build'
exe = target / 'release/file-native-incremental'

def global_inventory():
    snap = inventory()
    extras = [Path('/private/tmp/file-native-prerequisite-macos-20261018-source/.git')]
    extras.extend(Path(tempfile.gettempdir()).glob('i1-bounds-*'))
    for root in extras:
        for directory, _, names in os.walk(root):
            for name in names:
                try:
                    item = (Path(directory) / name).stat()
                except FileNotFoundError:
                    continue
                snap['logical_bytes'] += item.st_size
                snap['allocated_file_bytes'] += item.st_blocks * 512
                snap['files'] += 1
    snap['scope'] = 'Whole incremental root + conservative isolated Git database + all bounds temporary fixtures'
    return snap

commands = [
    ('build', ['cargo', 'build', '--release', '--locked', '--workspace', '--target-dir', str(target)], 300),
    ('tests', ['cargo', 'test', '--locked', '--workspace', '--target-dir', str(target)], 300),
    ('paired-intents', [sys.executable, str(SOURCE / 'paired_intents.py'), '--exe', str(exe), '--base', str(LANE / 'paired-intents')], 900),
    ('supplement', [sys.executable, str(SOURCE / 'supplement.py'), '--exe', str(exe), '--base', str(LANE / 'supplement')], 900),
    ('replay-lifetime', [sys.executable, str(SOURCE / 'replay_lifetime.py'), '--exe', str(exe), '--base', str(LANE / 'replay-lifetime')], 900),
]
env = {**os.environ, 'PYTHONPATH': str(SOURCE), 'PYTHONDONTWRITEBYTECODE': '1'}
ok = True
completed = []
case_start = case_deadline = None
stop_reason = None
for label, original_argv, cap in commands:
    snap = global_inventory()
    if time.time() >= args.deadline_epoch:
        stop_reason = 'absolute-1500-second-execution-cutoff'
    elif snap['free_bytes'] < A['reserve_bytes']:
        stop_reason = 'free-space-floor'
    elif max(snap['logical_bytes'], snap['allocated_file_bytes']) >= A['logical_cap_bytes'] - 512 * 1024 ** 2:
        stop_reason = 'whole-owned-allocation-margin'
    if stop_reason:
        ok = False
        break
    argv = list(original_argv)
    if label == 'paired-intents':
        case_start = time.time()
        case_deadline = min(case_start + 900, args.deadline_epoch)
        argv.extend(['--deadline-epoch', str(case_deadline)])
    start = time.time()
    effective_deadline = min(start + cap, args.deadline_epoch, case_deadline if label not in ['build', 'tests'] else args.deadline_epoch)
    if start >= effective_deadline:
        ok = False
        stop_reason = 'combined-case-phase-expired'
        break
    reason = None
    with (E / (label + '.stdout')).open('xb') as out, (E / (label + '.stderr')).open('xb') as err:
        p = subprocess.Popen(argv, cwd=SOURCE, env=env, stdout=out, stderr=err, start_new_session=True)
        while p.poll() is None:
            snap = global_inventory()
            with (E / 'resources.jsonl').open('a') as f:
                f.write(json.dumps(dict(label=label, **snap)) + '\n')
            if time.time() >= effective_deadline:
                reason = 'command-or-combined-case-or-absolute-execution-deadline'
            elif snap['free_bytes'] < A['reserve_bytes']:
                reason = 'free-space-floor'
            elif max(snap['logical_bytes'], snap['allocated_file_bytes']) >= A['logical_cap_bytes'] - 512 * 1024 ** 2:
                reason = 'whole-owned-allocation-margin'
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
               exit=p.returncode, censored=bool(reason), reason=reason, command_cap_seconds=cap,
               effective_deadline_epoch=effective_deadline, pin=A['pin'],
               absolute_execution_deadline_epoch=args.deadline_epoch,
               combined_case_start_epoch=case_start, combined_case_deadline_epoch=case_deadline,
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
        stop_reason = reason or 'command-failed'
        break
(E / 'finished.json').write_text(json.dumps(dict(
    status='passed' if ok else 'stopped', pin=A['pin'], epoch=time.time(), resources=global_inventory(),
    completed=completed, unperformed=[row[0] for row in commands if row[0] not in completed],
    stop_reason=stop_reason, combined_case_start_epoch=case_start, combined_case_deadline_epoch=case_deadline,
    combined_case_elapsed_seconds=time.time()-case_start if case_start else None,
    explicit_start_epoch=A['start_epoch'], absolute_execution_deadline_epoch=args.deadline_epoch,
    explicit_total_deadline_epoch=A['total_deadline_epoch'], original_windows_reopened=False), indent=2) + '\n')
raise SystemExit(0 if ok else 1)
