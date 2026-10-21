"""SOURCE ONLY targeted F1/F2/census controls; never engine qualification.

Future entrypoint through parent-verified stdlib bootstrap and exact capsule.
No invocation/manifest is released by this source. No code runs on import.
"""
import copy
import fcntl
import json
import os
from pathlib import Path
import queue
import threading
import time
import uuid


def main(capsule):
    from native_bootstrap_v3 import verify_bound_manifest
    import native_resources_v3 as resources
    from native_resources_v3 import canonical, digest_bytes, need, regular_read
    import native_guard as guard
    manifest = capsule.manifest
    results = []
    def note(name, **fields):
        results.append(dict(control=name, **fields))

    # F1 checks execute the ACTUAL closure validator with precisely described
    # deltas. No module bytes/manifest files are changed; full original manifest
    # plus each delta reconstructs every test INPUT. All failures precede any
    # Driver construction/child launch in this control entrypoint.
    folder = Path(manifest['implementation_directory'])
    source_files = manifest['source_files']
    bootstrap_sha = source_files[(folder/'native_bootstrap_v3.py').relative_to(guard.ROOT/'source').as_posix()]['sha256']
    loader_sha = source_files[(folder/'native_capsule_v3.py').relative_to(guard.ROOT/'source').as_posix()]['sha256']
    for name in sorted(manifest['implementation']):
        changed = copy.deepcopy(manifest)
        relative = changed['implementation'][name]
        changed['source_files'].pop(relative)
        try:
            verify_bound_manifest(changed, bootstrap_sha, loader_sha)
        except RuntimeError as error:
            note('missing-implementation', file=relative, refusal=str(error), children_launched=0,
                 original_manifest_sha256=capsule.expected_sha256)
        else:
            raise RuntimeError('missing implementation unexpectedly admitted:'+relative)
    changed = copy.deepcopy(manifest)
    relative = manifest['implementation']['native_driver_v3.py']
    changed['source_files'][relative]['sha256'] = '0'*64
    try:
        verify_bound_manifest(changed, bootstrap_sha, loader_sha)
    except RuntimeError as error:
        note('changed-implementation-binding', file=relative, expected_sha256='0'*64,
             refusal=str(error), children_launched=0)
    else:
        raise RuntimeError('changed implementation unexpectedly admitted')

    # A parent-sealed CONTROLS-ONLY case, separate from158 pending engine IDs.
    case_id = 'native-v3-interface-controls'
    case = capsule.admission['cases'][case_id]
    root = Path(case['root'])
    output = guard.ROOT/'evidence'/('v3-control-events-'+uuid.uuid4().hex+'.jsonl')
    driver, adapter, _ = capsule.construct(output)
    forecast = dict(kind='native-interface-controls-only', max_calls=5,
                    fixtures_bytes=1048576, fixtures_files=128, no_engine_evidence=True)
    adapter.preflight(case_id, root, forecast, manifest['suite_pin'])
    key = 'control-root-'+uuid.uuid4().hex
    driver.reservations.reserve_control(key, 0, directories=1)
    root.mkdir(exist_ok=False)
    driver.reservations.settle(key)
    def call(**fields):
        adapter.before_call(root, 'resolve', 'reader', 5)
        return driver.call(root, 'resolve', timeout=5, **fields)
    def failed_call(label, **fields):
        try:
            call(**fields)
        except AssertionError as error:
            need(len(error.args) == 1 and type(error.args[0]) is dict, 'exact-control-call-row')
            row = error.args[0]
            need(row['input']['root'] == str(root) and row['executable_sha256'] == driver.binary_sha256,
                 'control-row-binary-input-binding')
            note(label, raw_call_token=row['call_token'], raw_event_file=str(output),
                 input_bytes_expected=row['input_bytes_expected'], input_bytes_sent=row['input_bytes_sent'],
                 input_fully_delivered=row['input_fully_delivered'], stdin_closed=row['stdin_closed'],
                 exit=row['exit'], violations=row['violations'])
            return row
        raise RuntimeError(label+': expected exact raw refusal/censor')

    failure = None
    try:
        call(peer='fast')
        normal = driver.events[-1]  # Sequential control only; never concurrent recovery.
        need(normal['input_fully_delivered'] and normal['stdin_closed'] and
             normal['input_bytes_sent'] == normal['input_bytes_expected'] and not normal['input_broken_pipe'],
             'normal-complete-input-proof')
        note('normal-complete-input', raw_call_token=normal['call_token'])
        semantic = failed_call('semantic-exit2', peer='code2')
        need(semantic['exit'] == 2 and semantic['violations'] == [] and semantic['input_fully_delivered'],
             'semantic-only-exit2-contract')
        ready = root/'early-stdin-closed'
        env = dict(EXP_PEER_EARLY_JSON='1', EXP_PEER_EARLY_READY=str(ready))
        def early_started(event):
            end = time.monotonic()+.2
            while not ready.exists() and time.monotonic() < end:
                time.sleep(.001)
            need(regular_read(ready, 64) == b'early-stdin-closed', 'peer-closed-stdin-before-delivery')
        early = failed_call('valid-json-before-input', peer='fast', env=env, on_started=early_started)
        need(not early['input_fully_delivered'] and early['input_bytes_sent'] == 0 and
             'incomplete-input-delivery' in early['violations'], 'early-valid-json-must-not-pass')
        # Query a temporary OWNED descriptor, not a process. No profile/cap is
        # reduced if the native default geometry is outside this reviewed plan.
        read_fd, write_fd = os.pipe()
        try:
            capacity = fcntl.fcntl(write_fd, fcntl.F_GETPIPE_SZ)
        finally:
            os.close(read_fd); os.close(write_fd)
        need(capacity <= 65536, 'control-input-geometry-needs-parent-reforecast')
        partial = failed_call('partial-written-then-BrokenPipe', peer='fast',
                              padding='x'*(2*capacity+1024), env={'EXP_PEER_PARTIAL_JSON': '1'})
        need(0 < partial['input_bytes_sent'] < partial['input_bytes_expected'] and
             partial['input_broken_pipe'] and not partial['input_fully_delivered'] and
             'incomplete-input-delivery' in partial['violations'], 'partial-input-must-not-pass')
        call(peer='exit86', code=86)
        note('explicit-injected-exit86', raw_call_token=driver.events[-1]['call_token'],
             engine_qualification=False)

        # Real atomic rename on a native worker thread, synchronized exactly
        # after scandir has listed an old name and BEFORE lstat. The wrapper
        # changes no filesystem observations or inventory data; it only schedules
        # a real rename, so a stale name genuinely raises FileNotFoundError.
        rename_root = root/'rename-control'
        original = resources.os.scandir
        allocation = 'rename-fixture-'+uuid.uuid4().hex
        driver.reservations.reserve_control(allocation, 8192, files=1, directories=1)
        rename_root.mkdir()
        (rename_root/'a').write_bytes(b'x'*8192)
        driver.reservations.settle(allocation)
        requests, replies = queue.Queue(), queue.Queue()
        errors = []
        def rename_worker():
            try:
                while True:
                    request = requests.get()
                    if request is None:
                        return
                    source, target = request
                    need(rename_lease in driver.reservations.pending, 'growth-lease-lost-during-retry')
                    source.rename(target)
                    replies.put(True)
            except BaseException as error:
                errors.append(error)
                replies.put(False)
        rename_lease = 'held-rename-'+uuid.uuid4().hex
        driver.reservations.reserve_control(rename_lease, 0)
        worker = threading.Thread(target=rename_worker, name='owned-rename-control', daemon=False)
        worker.start()
        state = dict(remaining=1, toggles=0)
        class RenameAfterList:
            def __init__(self, path):
                self.context = original(path)
            def __enter__(self):
                self.items = list(self.context.__enter__())
                return self.items
            def __exit__(self, *args):
                self.context.__exit__(*args)
                if state['remaining']:
                    source = Path(self.items[0].path)
                    target = source.with_name('b' if source.name == 'a' else 'a')
                    requests.put((source, target))
                    need(replies.get(timeout=1) is True, 'real-rename-thread-failed')
                    state['remaining'] -= 1
                    state['toggles'] += 1
        def scheduled_scandir(path):
            return RenameAfterList(path) if Path(path) == rename_root else original(path)
        try:
            resources.os.scandir = scheduled_scandir
            observed = resources.census(rename_root, deadline=driver.owner['absolute_deadline'])
            need(observed['retries'] == 1 and observed['file_count'] == 1 and observed['bytes'] == 8192 and
                 rename_lease in driver.reservations.pending, 'bounded-rename-retry-proof')
            note('one-real-rename-retries-completely', retries=observed['retries'], bytes=observed['bytes'],
                 files=observed['file_count'], held_growth_lease=True)
            state['remaining'] = 4
            try:
                resources.census(rename_root, deadline=driver.owner['absolute_deadline'])
            except RuntimeError as error:
                need('bounded-census-retries-exhausted' in str(error) and state['remaining'] == 0 and
                     rename_lease in driver.reservations.pending, 'exhausted-retry-must-retain-lease')
                note('four-real-renames-fail-closed', refusal=str(error), held_growth_lease=True,
                     no_missing_as_zero=True)
            else:
                raise RuntimeError('unstable census silently admitted')
        finally:
            resources.os.scandir = original
            requests.put(None)
            worker.join(2)
            need(not worker.is_alive() and not errors, 'rename-Python-worker-not-joined')
            driver.reservations.settle(rename_lease)
    except BaseException as error:
        failure = repr(error)
        raise
    finally:
        closure = adapter.close_case(case_id, root)
        report = dict(kind='INTERFACE CONTROLS ONLY; NO ENGINE CASE QUALIFIED', failure=failure,
                      source_manifest_sha256=capsule.expected_sha256, results=results, closure=closure,
                      raw_event_file=str(output), raw_sha256=guard.digest(output))
        key = 'control-final-report-'+uuid.uuid4().hex
        raw = canonical(report)+b'\n'
        driver.reservations.reserve_evidence(key, len(raw), 1)
        path = output.with_name(output.stem+'-report.json')
        with path.open('xb') as stream:
            stream.write(raw)
        driver.reservations.settle(key)
        need(closure['processes_closed'] and closure['handles_closed'] and closure['python_callbacks_joined'],
             'control-stage-positive-direct-closure-required')


if __name__ == '__main__':
    main(BOUND_CAPSULE)
