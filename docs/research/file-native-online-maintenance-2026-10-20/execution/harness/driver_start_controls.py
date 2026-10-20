"""Positive pre-input callback and callback-failure child closure controls."""
import json
from pathlib import Path
from baseline import BASELINE, EXPECTED, oracle
from driver import Driver
from job_guard import Budget, sha_file, write_json
import os

runtime = Path('C:/file-native-online-matrix-20261020')
r = runtime/'receipts'
base = runtime/'protocol/driver-start-controls'
assert not base.exists() and sha_file(BASELINE) == EXPECTED
Budget(os.environ['FILE_NATIVE_BUDGET']).check({'protocol':dict(bytes=4*1024**2,files=1000)})
events = []
results = []
for label in ['without-hook', 'positive-hook']:
    root = base/label/'disposable/store'
    d = Driver(BASELINE,r/('driver-start-'+label+'-events.jsonl'))
    def ready(event):
        assert event['positive_rss'] > 0 and event['before_stdin_delivery']
        assert not root.exists(), 'engine-input-must-not-yet-be-delivered'
        events.append(event)
    response = d.call(root,'init',count=10,mode='A',consumer='project',on_started=ready if label == 'positive-hook' else None)
    assert response['result']['status'] == 'published'
    proof = oracle(root,r/('driver-start-'+label+'-independent.json'))
    if label == 'positive-hook':
        assert len(events) == 1 and d.last['positive_start_event'] == events[0]
        assert d.last['positive_start_event']['observed_ns'] <= d.last['stdin_started_ns'] < d.last['process_done_ns']
    results.append(response['result']['result']['state_digest'])
assert results[0] == results[1]
failure_root = base/'raised-hook/disposable/store'
d = Driver(BASELINE,r/'driver-start-raised-hook-events.jsonl')
def fail(event):
    assert event['positive_rss'] > 0 and event['before_stdin_delivery'] and not failure_root.exists()
    raise RuntimeError('intentional-start-hook-control')
try:
    d.call(failure_root,'init',count=10,mode='A',consumer='project',on_started=fail)
except AssertionError:
    assert not failure_root.exists() and d.last['stdin_started_ns'] is None
    assert d.last['exit'] != 0 and any('intentional-start-hook-control' in x for x in d.last['violations'])
else:
    raise AssertionError('raised-hook-was-not-refused')
guard = Path(__file__).with_name('job_guard.py')
assert sha_file(guard) == '1e4a965139cac5b48e6cbef57d0ac8cc3c3d0294f36a392383308abcee3affae'
write_json(r/'driver-start-controls.json',dict(status='passed',positive_callback_before_input=True,
           with_and_without_hook_state_digests_match=True,raised_hook_killed_and_waited=True,
           raised_hook_input_not_sent=True,positive_event=events[0],
           old_driver_sha256=sha_file(r/'driver-before-start-hook.py.txt'),
           new_driver_sha256=sha_file(Path(__file__).with_name('driver.py')),
           frozen_guard_sha256=sha_file(guard),baseline_executable_sha256=EXPECTED))
print(json.dumps(dict(status='passed',positive_start=True,raised_callback_refused=True,identical_state=True)))
