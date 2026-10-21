"""Explicit future launcher. No operation is admitted by this source file."""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import sys

parser = argparse.ArgumentParser()
parser.add_argument('mode', choices=['build', 'protocol', 'pace'])
parser.add_argument('name')
parser.add_argument('--budget', required=True)
parser.add_argument('--budget-sha256', required=True)
parser.add_argument('--clock')
parser.add_argument('--clock-sha256')
args = parser.parse_args()
assert args.name.startswith('cp4-') and all(c.isalnum() or c == '-' for c in args.name)
root = Path('C:/Users/spmcc/.codex/worktrees/file-native-online-cp2-20261020/handbook')
crate = root / 'docs/research/file-native-incremental-engine-experiment-2026-10-18'
base = Path('C:/file-native-online-cp1-20261020')
guard = Path('C:/Users/spmcc/.codex/worktrees/file-native-online-matrix-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/windows-cadence/job_guard_qpc.py')
assert hashlib.sha256(guard.read_bytes()).hexdigest() == '50ecc7c40c1cdda6493817261b5755fe2d823fdae271118698828c39a0400378'
budget = Path(args.budget)
assert hashlib.sha256(budget.read_bytes()).hexdigest() == args.budget_sha256
limits = json.loads(budget.read_bytes())['categories']
assert Path(limits['build']['root']).resolve() == (base / 'build').resolve()
assert limits['build']['bytes'] == 268435456 and limits['fixtures']['bytes'] == 201326592
assert limits['source']['bytes'] == 1048576 and limits['source_admin']['bytes'] == 2097152
assert limits['evidence']['bytes'] == 50331648
spec = importlib.util.spec_from_file_location('cp4_guard_qpc', str(guard))
qpc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(qpc)
evidence = base / 'evidence'
unit_root = base / f'fixtures/cp3/{args.name}/unit'
os.environ['EXP_PACK_TEST_ROOT'] = str(unit_root)
if args.mode == 'build':
    inputs = {str(p.relative_to(root)).replace('\\', '/'): dict(bytes=p.stat().st_size, sha256=hashlib.sha256(p.read_bytes()).hexdigest())
              for p in crate.rglob('*') if p.is_file() and (p.suffix == '.rs' or p.name in ['Cargo.toml', 'Cargo.lock'])}
    snapshot = evidence / f'{args.name}-input.json'
    assert not snapshot.exists()
    controls = {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in Path(__file__).parent.iterdir() if p.is_file()}
    snapshot.write_text(json.dumps(dict(source_files=inputs, control_files=controls, budget_sha256=args.budget_sha256, guard_sha256=hashlib.sha256(guard.read_bytes()).hexdigest(), binary_before=hashlib.sha256((base/'build/release/file-native-incremental.exe').read_bytes()).hexdigest()), indent=2) + '\n')
    common = ['--locked', '--offline', '--workspace', '--release', '-j2', '--target-dir', str(base/'build')]
    # Compile before redirecting the small bounds unit's temp directory. Native
    # build/link scratch remains inside this SAME target's explicit tmp subtree.
    code = ('import os,subprocess,sys;from pathlib import Path;'
            f'common={common!r};cargo="C:/Users/spmcc/.cargo/bin/cargo.exe";'
            f't=Path({str(base / "build/tmp")!r});t.mkdir(exist_ok=True);os.environ.update(TMP=str(t),TEMP=str(t));'
            'subprocess.run([cargo,"build",*common],check=True);'
            'subprocess.run([cargo,"test",*common,"--no-run"],check=True);'
            f'u=Path({str(unit_root / "temp")!r});u.mkdir(parents=True);os.environ.update(TMP=str(u),TEMP=str(u));'
            'subprocess.run([cargo,"test",*common,"--","--test-threads=1","--nocapture"],check=True)')
    argv = [sys.executable, '-B', '-c', code]
    reserves = dict(build=dict(bytes=50331648, files=150), fixtures=dict(bytes=85957054, files=800), evidence=dict(bytes=393216, files=25))
    seconds, cwd = 240, crate
elif args.mode == 'protocol':
    argv = [sys.executable, '-B', str(Path(__file__).with_name('protocol.py')), args.name]
    reserves = dict(fixtures=dict(bytes=8912896, files=2500), evidence=dict(bytes=2621440, files=50))
    seconds, cwd = 600, root
else:
    assert args.clock and args.clock_sha256, 'reviewed-platform-clock-required'
    argv = [sys.executable, '-B', str(Path(__file__).with_name('pace_controls.py')), args.name, '--clock', args.clock, '--clock-sha256', args.clock_sha256]
    reserves = dict(fixtures=dict(bytes=6291456, files=1200), evidence=dict(bytes=4194304, files=25))
    seconds, cwd = 180, root
result = qpc.run(argv, str(evidence / f'{args.name}-guard.json'), str(budget), seconds, reserves, cwd=str(cwd))
print(json.dumps({k: result.get(k) for k in ['exit', 'errors', 'censor', 'process_closure_verified', 'job_aggregate_sampled_peak_rss']}))
