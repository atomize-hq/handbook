"""Parent-authorized tiny controls under exactly the frozen shared Job guard."""
import hashlib
import importlib.util
from pathlib import Path
import sys

here = Path(__file__).resolve().parent
guard = here.parent.parent / 'harness/job_guard.py'
assert hashlib.sha256(guard.read_bytes()).hexdigest() == '1e4a965139cac5b48e6cbef57d0ac8cc3c3d0294f36a392383308abcee3affae'
spec = importlib.util.spec_from_file_location('frozen_guard', guard)
g = importlib.util.module_from_spec(spec)
spec.loader.exec_module(g)
root = Path('C:/file-native-online-matrix-20261020/receipts/stream-control-main')
budget = root / 'budget.json'
assert g.sha_file(budget) == 'b6cb1f7e2e4069e7ab9833e83c84ed923d59beaf2c9fc7363eceb1ed8b4b63c3'
result = g.run([sys.executable, '-B', str(here / 'controls.py')], root / 'guard-run.json',
               budget, 240, cwd=str(here))
print({k: result[k] for k in ['exit', 'censor', 'errors', 'process_closure_verified',
                             'job_aggregate_sampled_peak_rss', 'memory_sample_timing']})
sys.exit(0 if result.get('exit') == 0 and not result.get('censor') and result.get('process_closure_verified') else 1)
