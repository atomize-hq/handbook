"""Artifact-only follow-up Job; cannot launch an engine or reopen functional work."""
import datetime, hashlib, json, sys
from pathlib import Path

assert len(sys.argv) == 1
HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent))
import guard

guard.CUTOFF = datetime.datetime.fromisoformat('2026-10-20T13:13:00+00:00').timestamp()
out = guard.ROOT / 'followup-20261020/receipts'
for path in out.glob('*-job.json'):
    assert json.loads(path.read_bytes())['process_closure_verified'], str(path)
actual = guard.current()
assert actual['total_new_bytes'] + 32*1024**2 <= 2684354560 and actual['total_new_files'] + 1000 <= 560000 and actual['free_bytes'] >= 140*1024**3
artifact = HERE / 'package_v2.py'
receipt = out / 'artifact-closeout-v2-job.json'
assert not receipt.exists()
(out / 'artifact-closeout-v2-binding.json').write_text(json.dumps(dict(scope='fixed artifact packager only; no engine or data mutation', original_functional_cutoff='2026-10-20T07:55:16Z', followup_functional_cutoff='2026-10-20T12:53:00Z', followup_artifact_cutoff='2026-10-20T13:13:00Z', actual=actual, return_and_Git_margin=32*1024**2, compressed_archive_prewrite_cap=14*1024**2, package_cap=15*1024**2, expanded_archive_cap=128*1024**2, artifact_sha256=hashlib.sha256(artifact.read_bytes()).hexdigest(), launcher_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()), indent=2))
result = guard.run([sys.executable, str(artifact)], 120, receipt, cwd=guard.WORK, allowance=2684354560, files=560000)
print(json.dumps({k: result[k] for k in ['exit', 'censor', 'process_closure_verified', 'seconds', 'final_ownership']}, indent=2))
assert result['exit'] == 0 and result['censor'] is None and result['process_closure_verified']
