"""Mac prerequisite adapter. Pinned helper assertions and engine are unchanged.

Only owned paths, finite guards, caps, native resource accounting and smoke
allocation differ. Run one named phase per invocation; failed roots are retained.
"""
import argparse
import hashlib
import json
import os
import pathlib
import runpy
import shutil
import subprocess
import sys
import threading
import time
import traceback
import types

HERE = pathlib.Path(__file__).resolve().parent
SOURCE = HERE.parent / 'file-native-engine-followup-2026-10-17'
ALLOCATION = json.loads((HERE / 'allocation.json').read_bytes())
RUNTIME = pathlib.Path(ALLOCATION['runtime'])
EXE = RUNTIME / 'build/release/file-native-publication-experiment'
sys.path.insert(0, str(SOURCE))
import baseline_harness as b
import experiment as e
import workloads as w

# Darwin wait4.ru_maxrss is bytes (Linux reports KiB). Exact process wait only;
# no cumulative child maximum, polling estimate, or cross-process peak sum.
class NativePopen(subprocess.Popen):
    def _try_wait(self, wait_flags):
        try:
            pid, status, usage = os.wait4(self.pid, wait_flags)
            if pid:
                self.native_rusage = dict(peak_rss_bytes=usage.ru_maxrss,
                    user_seconds=usage.ru_utime, system_seconds=usage.ru_stime,
                    source='Darwin wait4 rusage; ru_maxrss bytes')
            return pid, status
        except ChildProcessError:
            return self.pid, 0

subprocess.Popen = NativePopen
guard_lock = threading.Lock()
last_guard = (0, None)

def resource_snapshot():
    roots = {p: b.inventory(pathlib.Path(p)) for p in ALLOCATION['owned_roots']}
    return dict(epoch=time.time(), free_bytes=shutil.disk_usage(RUNTIME).free,
        allocated_file_bytes=sum(v['allocated_file_bytes'] for v in roots.values()),
        roots=roots, accounting='st_blocks*512, device/inode deduplicated within each root; file allocation excludes directories and APFS shared-extent accounting')

def budget(extra=0):
    global last_guard
    if time.time() >= ALLOCATION['launch_deadline_epoch']:
        raise RuntimeError('native-launch-deadline-report-reserve')
    if shutil.disk_usage(RUNTIME).free-extra < ALLOCATION['reserve_bytes']:
        raise RuntimeError('native-free-space-reserve')
    with guard_lock:
        if time.monotonic()-last_guard[0] > 5 or last_guard[1] is None:
            last_guard = (time.monotonic(), resource_snapshot())
            with (HERE/'resource-checks.jsonl').open('a') as f:
                f.write(json.dumps(last_guard[1])+'\n')
        # At most 8 bounded correctness workers, each far below 64 MiB growth.
        # 512 MiB conservative margin covers the <=5s accounting interval.
        if last_guard[1]['allocated_file_bytes']+extra+512*1024**2 > ALLOCATION['cap_bytes']:
            raise RuntimeError('native-owned-root-allocation-cap')

OriginalDriver = e.Driver
class NativeDriver(OriginalDriver):
    def __init__(self, base, exe, label, backend='manifest'):
        super().__init__(base, exe, label, backend)
        self.processes = {}
        self.native_wrapper_sha256 = b.sha(pathlib.Path(__file__).read_bytes())
        manifest = dict(wrapper_sha256=self.native_wrapper_sha256,
            allocation_sha256=b.sha((HERE/'allocation.json').read_bytes()),
            baseline=ALLOCATION['baseline'], source_pin_sha256=b.sha((HERE/'source-pins.json').read_bytes()),
            source_files={p.name:b.sha(p.read_bytes()) for p in SOURCE.glob('*.py')},
            binary_sha256=self.binary_sha256,
            caps=dict(small=120, growth=300), original_harness_sha256=self.harness_sha256)
        b.write(self.base/f'native-session-{self.session}.json', manifest)
    def budget(self, extra=0):
        budget(extra)
    def start(self, command, env=None):
        root = pathlib.Path(command['root']).resolve()
        if not root.is_relative_to(RUNTIME) or root == RUNTIME:
            raise RuntimeError('native-root-containment')
        pending = super().start(command, env)
        self.processes[pending[3]] = pending[0]
        return pending
    def finish(self, pending, timeout=120, allowed=(0,)):
        cap = 300 if self.label == 'growth' else 120
        return super().finish(pending, min(timeout, cap), allowed)
    def record(self, event):
        event['native_wrapper_sha256'] = self.native_wrapper_sha256
        event['native_process_usage'] = getattr(self.processes.get(event['seq']), 'native_rusage', None)
        event['platform'] = 'native-macos-arm64'
        super().record(event)

e.Driver = NativeDriver
w.allocation_check = budget

def smoke(driver, mode):
    root = driver.base/'growth'/f'1000-{mode}-project'
    driver.init(root, mode, 'project', 1000, timeout=300)
    initial = b.selected_state(root)[0]
    for packet in [False, True]:
        for count in [1, 8, 64]:
            name = f'commit-{count}-packet-{packet}'
            request = {**b.request(name, b.batch_ops(count, name)), 'packet':packet}
            out = driver.publish(root, mode, request, 'slot')
            assert out['result']['status']=='published'
            assert out['result']['result']['outcome']=='accepted'
    info = driver.inspect(root, True)
    assert len(info['history']) == 7
    current = info['selected']
    for selected in [current, initial]:
        out = driver.call(e.cmd(root, 'packet', selected=selected), timeout=300)
        b.check_packet(root/out['result']['path'])
        for key in ['d000001', 'not-present']:
            for trial in range(3):
                out = driver.call(e.cmd(root, 'point', selected=selected, id=key))
                assert (out['result']['value'] is not None) == (key=='d000001')
    b.write(driver.base/'growth-observations.json', dict(backend=driver.backend,
        mode=mode, count=1000, selected_publications=7, initial=initial,
        current=current, inventory=b.inventory(root), status='passed'))

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('phase', choices=['correctness','regressions','restoration','indirect-gc','backend-checks','navigation','contention','growth'])
    parser.add_argument('--backend', choices=['manifest','cow'], default='manifest')
    parser.add_argument('--mode', choices=['A','B'])
    parser.add_argument('--run', default='v1')
    args = parser.parse_args()
    label = args.phase + (('-'+args.mode) if args.mode else '')
    base = RUNTIME/'data'/args.run/args.backend/label
    base.mkdir(parents=True, exist_ok=False)
    try:
        budget()
        d = NativeDriver(base, EXE, 'growth' if args.phase=='growth' else args.phase, args.backend)
        if args.phase=='correctness': e.correctness(d)
        elif args.phase in ['regressions','restoration','indirect-gc','backend-checks']:
            sys.argv=[str(SOURCE/(args.phase.replace('-','_')+'.py')), '--base',str(base), '--exe',str(EXE)]
            if args.phase in ['regressions','restoration']: sys.argv += ['--backend',args.backend]
            runpy.run_path(sys.argv[0], run_name='__main__')
        elif args.phase=='navigation': w.navigation(d, types.SimpleNamespace())
        elif args.phase=='contention': w.contention(d, types.SimpleNamespace())
        elif args.phase=='growth':
            assert args.mode
            smoke(d, args.mode)
        b.write(base/'native-phase-completed.json', dict(phase=args.phase, backend=args.backend,
            mode=args.mode,status='passed',epoch=time.time(),resource=resource_snapshot()))
    except BaseException as error:
        b.write(base/'native-phase-stopped.json', dict(phase=args.phase,backend=args.backend,
            mode=args.mode,error=str(error),traceback=traceback.format_exc(),epoch=time.time()))
        raise

if __name__=='__main__': main()
