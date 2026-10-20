"""New follow-up clock and parent admission, reusing unchanged B path-v2 consumers."""
import argparse, datetime, hashlib, json, runpy, sys, time
from pathlib import Path

HERE = Path(__file__).resolve().parent
OLD = HERE.parent
sys.path.insert(0, str(OLD))
import guard

# Set the new cutoff before importing any original consumer. The old file stays unchanged.
guard.CUTOFF = datetime.datetime.fromisoformat('2026-10-20T12:53:00+00:00').timestamp()
assert datetime.datetime.fromisoformat('2026-10-20T11:43:00+00:00').timestamp() <= time.time() < guard.CUTOFF
p = argparse.ArgumentParser()
p.add_argument('representation', choices=['L-old', 'L-new', 'P0', 'PZ'])
p.add_argument('--admission', required=True)
p.add_argument('--child', action='store_true')
a = p.parse_args()
cfg = json.loads((HERE / 'preparation.json').read_bytes())
admission_path = Path(a.admission).resolve()
admission = json.loads(admission_path.read_bytes())
assert admission['scope'] == 'P-followup-20261020-B-cell' and admission['representation'] == a.representation and admission['go'] is True
assert admission['functional_cutoff'] == cfg['functional_cutoff']
for name, digest in cfg['bindings'].items():
    assert hashlib.sha256(Path(name).read_bytes()).hexdigest() == digest, name
out = guard.ROOT / 'followup-20261020/receipts'
if a.child:
    sys.argv = [str(OLD / 'sequenced_matrix_stage.py'), 'cell', '--mode', 'B', '--representation', a.representation, '--path-v2']
    runpy.run_path(str(OLD / 'sequenced_matrix_stage.py'), run_name='__main__')
else:
    actual = guard.current()
    assert actual['total_new_bytes'] + 600000000 + 67108864 + 33554432 <= cfg['windows_bytes_cap']
    assert actual['total_new_files'] + 50010 <= cfg['windows_files_cap'] and actual['free_bytes'] >= cfg['free_bytes_min']
    receipt = out / ('B-' + a.representation + '-job.json')
    assert not receipt.exists()
    (out / ('B-' + a.representation + '-binding.json')).write_text(json.dumps(dict(actual=actual, config_sha256=hashlib.sha256((HERE / 'preparation.json').read_bytes()).hexdigest(), launcher_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(), parent_admission=str(admission_path), parent_admission_sha256=hashlib.sha256(admission_path.read_bytes()).hexdigest(), functional_cutoff=cfg['functional_cutoff'], original_run_unchanged=True), indent=2))
    result = guard.run([sys.executable, str(Path(__file__).resolve()), a.representation, '--admission', str(admission_path), '--child'], 900, receipt, cwd=guard.WORK, allowance=2684354560, files=560000)
    print(json.dumps({k: result[k] for k in ['exit', 'censor', 'process_closure_verified', 'seconds', 'final_ownership']}, indent=2))
    assert result['exit'] == 0 and result['censor'] is None and result['process_closure_verified']
