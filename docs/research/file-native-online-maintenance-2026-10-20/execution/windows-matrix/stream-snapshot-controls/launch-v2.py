"""Final control pin; retained initial censor and v2, post-final receipt census."""
import hashlib
import importlib.util
from pathlib import Path
import sys
here = Path(__file__).resolve().parent
guard = here.parent.parent / 'harness/job_guard.py'
assert hashlib.sha256(guard.read_bytes()).hexdigest() == '1e4a965139cac5b48e6cbef57d0ac8cc3c3d0294f36a392383308abcee3affae'
spec = importlib.util.spec_from_file_location('frozen_guard_v2', guard)
g = importlib.util.module_from_spec(spec)
spec.loader.exec_module(g)
root = Path('C:/file-native-online-matrix-20261020/receipts/stream-control-main')
budget = root / 'budget-claim009-011.json'
assert g.sha_file(budget) == 'b3b770422fd9f608be5d9ee2f756222b8874b0948843651d0662f5a0ace1e765'
module = here.parent.parent / 'harness/stream_snapshot.py'
before_hash = g.sha_file(module)
assert module.stat().st_size <= 131072
protocol = Path('C:/file-native-online-matrix-20261020/protocol/stream-control')
prior = g.census(protocol)
reserves = dict(stream_protocol=dict(bytes=2097152-prior['bytes'], files=166-prior['files']),
                stream_receipts=dict(bytes=327680, files=8),
                stream_landing_controls=dict(bytes=3072, files=1))
result = g.run([sys.executable, '-B', str(here / 'controls.py')], root / 'guard-run-final.json',
               budget, 90, reserves=reserves, cwd=str(here))
# This runs AFTER the frozen guard finalized JSON, stdout/stderr and gzip.
g.Budget(budget).check()
actual = dict(protocol=g.census(protocol), runtime=g.census(root), landing=g.census(here))
assert g.sha_file(module) == before_hash and module.stat().st_size <= 131072
assert actual['protocol']['bytes'] <= 2097152 and actual['protocol']['files'] <= 166
assert actual['runtime']['bytes'] <= 983040 and actual['runtime']['files'] <= 40
assert actual['landing']['bytes'] + 3072 <= 65536 and actual['landing']['files'] + 1 <= 9
assert actual['runtime']['bytes'] + actual['landing']['bytes'] + 3072 <= 1048576
print(dict(post_final_receipt_census=actual, owned_module_sha256=before_hash,
           final_Landing_report_reserved_bytes=3072))
print({k: result[k] for k in ['exit', 'censor', 'errors', 'process_closure_verified',
                             'job_aggregate_sampled_peak_rss', 'memory_sample_timing']})
sys.exit(0 if result.get('exit') == 0 and not result.get('censor') and result.get('process_closure_verified') else 1)
