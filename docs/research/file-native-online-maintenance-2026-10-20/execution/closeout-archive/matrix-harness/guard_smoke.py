"""Benign timeout, descendant closure, memory, and prospective-admission probes."""
import argparse
import json
from pathlib import Path
import sys
import time
import subprocess
import threading
from job_guard import Budget, run, sha_file, write_json

parser = argparse.ArgumentParser()
parser.add_argument('--budget', required=True)
parser.add_argument('--output', required=True)
args = parser.parse_args()
base = Path(args.output)
base.mkdir(parents=True, exist_ok=False)
rows = []
for name, command, seconds, data, expected in [
    ('normal', 'print("benign-smoke")', 10, None, None),
    ('blocked-stdin', 'import time;time.sleep(30)', 4, b'x' * 1024**2, 'time-censor'),
    ('descendant', 'import subprocess,sys,time;subprocess.Popen([sys.executable,"-c","import time;time.sleep(30)"]);time.sleep(.1)', 10, None, None),
]:
    started = time.monotonic()
    row = run([sys.executable, '-B', '-c', command], base / (name + '.json'),
              args.budget, seconds, stdin=data)
    assert row['censor'] == expected and row['process_closure_verified'], row
    assert row['per_process_observed_peak_rss'] and row['job_aggregate_sampled_peak_rss']
    if name == 'blocked-stdin':
        assert row['total_seconds'] < 10
        timeout = next(x for x in row['terminations'] if x['reason'] == 'time-censor')
        assert abs(timeout['epoch']-row['absolute_deadline_epoch']) < .2
    if name == 'descendant':
        assert row['exit'] == 0 and row['terminations'][0]['reason'] == 'root-completed-descendant-cleanup'
    rows.append(dict(case=name, receipt=str(base / (name + '.json')),
                     elapsed_seconds=time.monotonic()-started))
# Both guards register as supervisors; exactly one Job may hold the heavy token.
state = {}
def holder():
    state['result'] = run([sys.executable, '-B', '-c', 'import time;time.sleep(5)'],
                          base/'token-holder.json', args.budget, 10)
thread = threading.Thread(target=holder)
thread.start()
from job_guard import OWNER
until = time.monotonic()+10
while time.monotonic() < until:
    try:
        owner = json.loads(OWNER.read_bytes())
        if owner['receipt'] == str((base/'token-holder.json').resolve()) and owner['state'] == 'running':
            break
    except (FileNotFoundError, json.JSONDecodeError):
        pass
    time.sleep(.01)
else:
    raise AssertionError('token-holder-not-ready')
competitor = subprocess.Popen([sys.executable, '-B', str(Path(__file__).with_name('job_guard.py')),
    '--budget', args.budget, '--receipt', str(base/'token-waiter.json'), '--seconds', '10',
    '--', sys.executable, '-B', '-c', 'print("token-waiter-benign")'],
    stdout=subprocess.PIPE, stderr=subprocess.PIPE)
out, err = competitor.communicate(timeout=20)
thread.join(timeout=20)
assert not thread.is_alive() and competitor.returncode == 0, (out, err, state)
waiter = json.loads((base/'token-waiter.json').read_bytes())
assert waiter['token_wait_seconds'] >= .2 and waiter['token_contentions'] > 0 and waiter['process_closure_verified']
assert state['result']['process_closure_verified'] and not state['result']['censor']
assert state['result']['closure_checks'][-1]['epoch'] <= waiter['token_acquired_epoch']
rows.append(dict(case='shared-token-contention', holder=str(base/'token-holder.json'),
                 waiter=str(base/'token-waiter.json'), waited_seconds=waiter['token_wait_seconds']))
budget = Budget(args.budget)
name = next(iter(budget.categories))
try:
    budget.check({name: dict(bytes=budget.categories[name]['bytes']+1, files=0)})
except ValueError as error:
    assert str(error).startswith('prospective-category-cap:')
    rows.append(dict(case='prospective-cap-refusal', error=str(error), child_started=False))
else:
    raise AssertionError('prospective-overflow-accepted')
write_json(base / 'summary.json', dict(status='passed', cases=rows,
           guard_sha256=sha_file(Path(__file__).with_name('job_guard.py')),
           qualification='benign Python processes only; no engine, build or corpus'))
print(json.dumps(dict(status='passed', cases=len(rows))))
