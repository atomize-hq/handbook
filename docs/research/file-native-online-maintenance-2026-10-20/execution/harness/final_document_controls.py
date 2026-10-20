"""Two matched final-pin shallow stores, without then with physical maintenance.

Execution requires an externally frozen final-pin plan and the admitted F07 cap.
No d70 N1k control, retained third seed, extra semantic operation, or percentile.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor
import importlib
import json
import os
from pathlib import Path
import shutil
import sys
import time
from driver import Driver
from job_guard import Budget, census, sha_file, write_json

parser = argparse.ArgumentParser()
parser.add_argument('--plan', required=True)
parser.add_argument('--count', required=True, type=int, choices=[64, 1000])
parser.add_argument('--mode', required=True, choices=['A', 'B'])
args = parser.parse_args()
assert os.environ.get('FILE_NATIVE_STAGE_JOB') == '1'
assert sys.dont_write_bytecode
plan_path = Path(args.plan)
plan = json.loads(plan_path.read_bytes())
assert plan['version'] == 1 and len(plan['source_pin']) == 40
assert plan['status'] == 'final-pin-frozen-and-F07-admitted'
assert sha_file(plan['binary']) == plan['binary_sha256']
for source in plan['source_files']:
    assert sha_file(source['path']) == source['sha256']
assert sha_file(plan['oracle_path']) == plan['oracle_sha256']
oracle_path = Path(plan['oracle_path']).resolve()
sys.path.insert(0, str(oracle_path.parent))
oracle = importlib.import_module(oracle_path.stem)
assert Path(oracle.__file__).resolve() == oracle_path
operations = [('payload', [dict(kind='edit', id='d000004', text=' matched payload')]),
              ('title', [dict(kind='rename', id='d000004', text='Matched title')])]
assert plan['operations'] == [dict(case=name, ops=ops) for name, ops in operations]
schedule = plan['maintenance']
assert set(schedule) == {'before_pair', 'alongside_payload', 'alongside_title', 'after_pair'}
for commands in schedule.values():
    assert type(commands) is list and len(commands) <= 64
    for command in commands:
        assert command['action'] == 'packing' and 'root' not in command
assert schedule['alongside_payload'] and schedule['alongside_title']
assert plan['maintenance_overlap_checkpoint'] == 'physical-before-selection'

runtime = Path('C:/file-native-online-matrix-20261020')
receipts = runtime/'receipts'
label = f'final-N{args.count}-{args.mode}'
base = runtime/'document-controls'/label
control = base/'without-maintenance/disposable/store'
maintained = base/'with-maintenance/disposable/store'
assert not base.exists()
budget = Budget(os.environ['FILE_NATIVE_BUDGET'])
payload = sum(4096 if i % 10 <= 6 else 16384 if i % 10 <= 8 else 65536
              for i in range(args.count))
metadata = (9 if args.count == 1000 else 1)*1024**2
forecast = (4 if args.mode == 'A' else 8)*payload+metadata+1024**2
budget.check({'document_controls':dict(bytes=forecast, files=args.count*12+2000),
              'receipts':dict(bytes=4*1024**2,files=30)})
write_json(receipts/(label+'-frozen-plan.json'), dict(plan=plan, plan_sha256=sha_file(plan_path),
           count=args.count, mode=args.mode, prospective_pair_bytes=forecast,
           payload_floor_bytes=(4 if args.mode == 'A' else 8)*payload,
           qualification='prospective forecast and actual category cap; source/metadata growth not an unbounded guarantee'))
d = Driver(plan['binary'], receipts/(label+'-ordinary-events.jsonl'))
d.call(control, 'init', count=args.count, mode=args.mode, consumer='project', timeout=240)
d.call(control, 'lifecycle', operation='setup')
initial = census(control)
budget.check({'document_controls':dict(bytes=initial['bytes']+1024**2,files=initial['files']+100)})
shutil.copytree(control, maintained, ignore=shutil.ignore_patterns('DISPOSABLE.json'))
d.call(maintained, 'lifecycle', operation='setup')
assert (control/'SELECTED.json').read_bytes() == (maintained/'SELECTED.json').read_bytes()
initial_selected = Driver.head(control)
observations = {}
maintenance_rows = []

def physical(commands, phase):
    md = Driver(plan['binary'], receipts/(label+'-maintenance-events.jsonl'))
    for index, command in enumerate(commands):
        budget.check({'document_controls':dict(bytes=1024**2,files=1000)})
        command = dict(command)
        if phase.startswith('alongside_') and index == 0:
            environment = dict(command.pop('env', {}))
            assert not any(k in environment for k in ['EXP_BARRIER','EXP_READY','EXP_RELEASE'])
            environment.update(EXP_BARRIER=plan['maintenance_overlap_checkpoint'],
                               EXP_READY=str(receipts/(label+'-'+phase+'.ready')),
                               EXP_RELEASE=str(receipts/(label+'-'+phase+'.release')))
            command['env'] = environment
        started = time.perf_counter_ns()
        response = md.call(maintained, timeout=120, **command)
        maintenance_rows.append(dict(phase=phase,index=index,started_ns=started,
                                    ended_ns=time.perf_counter_ns(),response=response,
                                    process_started_ns=md.last['process_started_ns'],
                                    stdin_started_ns=md.last['stdin_started_ns'],
                                    process_done_ns=md.last['process_done_ns']))

for variant, root in [('without-maintenance', control), ('with-maintenance', maintained)]:
    if variant == 'with-maintenance':
        physical(schedule['before_pair'], 'before_pair')
    rows = []
    for case, ops in operations:
        positive_launch = []
        release_ns = []
        readable_reserve = payload if args.mode == 'B' else 0
        budget.check({'document_controls':dict(bytes=readable_reserve+2*1024**2,files=args.count+2000)})
        if variant == 'with-maintenance':
            phase = 'alongside_'+case
            ready = receipts/(label+'-'+phase+'.ready')
            release = receipts/(label+'-'+phase+'.release')
            assert not ready.exists() and not release.exists()
            with ThreadPoolExecutor(max_workers=1) as pool:
                future = pool.submit(physical, schedule[phase], phase)
                try:
                    deadline = time.monotonic()+45
                    while not ready.exists():
                        if future.done():
                            future.result()
                            raise AssertionError('maintenance-completed-without-overlap-checkpoint')
                        assert time.monotonic() < deadline, 'maintenance-overlap-checkpoint-timeout'
                        time.sleep(.005)
                    def launch(event):
                        assert event['positive_rss'] > 0 and event['before_stdin_delivery']
                        positive_launch.append(event)
                        release.write_bytes(b'ordinary-engine-positive-start')
                        release_ns.append(time.perf_counter_ns())
                    started = time.perf_counter_ns()
                    response = d.publish(root, 'final-document-'+case, ops=ops, timeout=45,on_started=launch)
                    ordinary_end = time.perf_counter_ns()
                finally:
                    if not release.exists():
                        release.write_bytes(b'release-after-failed-positive-start')
                future.result()
            assert len(positive_launch) == len(release_ns) == 1
            command_rows = [row for row in maintenance_rows if row['phase'] == phase and row['index'] == 0]
            assert len(command_rows) == 1
            first_maintenance = command_rows[0]
            active_overlap = (min(d.last['process_done_ns'],first_maintenance['process_done_ns'])
                              -max(d.last['stdin_started_ns'],release_ns[0]))
        else:
            def launch(event):
                assert event['positive_rss'] > 0 and event['before_stdin_delivery']
                positive_launch.append(event)
                (receipts/(label+'-control-'+case+'.positive-start')).write_bytes(b'ordinary-engine-positive-start')
            started = time.perf_counter_ns()
            response = d.publish(root, 'final-document-'+case, ops=ops, timeout=45,on_started=launch)
            ordinary_end = time.perf_counter_ns()
            active_overlap = None
        assert response['result']['status'] == 'published'
        rows.append(dict(case=case, started_ns=started, ended_ns=ordinary_end,
                         engine_ns=response['wall_ns'], spawn_total_ns=d.last['spawn_total_ns'],
                         positive_launch=positive_launch,maintenance_release_ns=release_ns,
                         ordinary_stdin_started_ns=d.last['stdin_started_ns'],
                         ordinary_process_done_ns=d.last['process_done_ns'],
                         concurrent_process_interval_ns=active_overlap,response=response))
    if variant == 'with-maintenance':
        physical(schedule['after_pair'], 'after_pair')
    verified = oracle.verify(root, receipts/(label+'-'+variant+'-independent.json'), quiescent=True)
    assert verified['semantic']['publications'] == 3
    observations[variant] = dict(rows=rows, independent=verified, final_census=census(root))
for left, right in zip(observations['without-maintenance']['rows'], observations['with-maintenance']['rows']):
    assert left['case'] == right['case']
    assert left['response']['result']['result']['state_digest'] == right['response']['result']['result']['state_digest']
assert sha_file(plan['binary']) == plan['binary_sha256']
assert sha_file(plan['oracle_path']) == plan['oracle_sha256']
for source in plan['source_files']:
    assert sha_file(source['path']) == source['sha256']
qualified_overlap = all(row['concurrent_process_interval_ns'] > 0
                        for row in observations['with-maintenance']['rows'])
write_json(receipts/(label+'-comparison.json'),dict(source_pin=plan['source_pin'],
           binary_sha256=plan['binary_sha256'],plan_sha256=sha_file(plan_path),
           count=args.count,mode=args.mode,initial_selected=initial_selected,
           observations=observations,maintenance=maintenance_rows,
           actual_H_per_variant=3,semantic_operations_per_variant=2,
           full_oracles_pass=True,retained_old_N64_artifacts_unchanged=True,
           maintenance_overlap_checkpoint=plan['maintenance_overlap_checkpoint'],
           release_at_positive_ordinary_start_before_input=True,
           qualified_concurrent_process_overlap=qualified_overlap,
           qualification='two matched operations; no percentile claim; post-release process intervals and engine clocks retained separately; no claim about continuous simultaneous CPU execution; any nonoverlap remains unqualified'))
print(json.dumps(dict(status='passed' if qualified_overlap else 'unqualified-no-overlap',
                     count=args.count,mode=args.mode,variants=2,actual_H_each=3)))
sys.exit(0 if qualified_overlap else 2)
