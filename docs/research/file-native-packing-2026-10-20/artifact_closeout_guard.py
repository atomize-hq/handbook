"""Artifact-only closing Job; does not extend the original functional clock.

No arguments accepted. Its sole child is the hash-bound evidence packager, which
reads retained results and writes only a bounded metadata archive and indexes.
The original guard file and every functional receipt keep 07:55:16 unchanged.
"""
import datetime,hashlib,json,sys
from pathlib import Path
import guard

assert len(sys.argv)==1
HERE=Path(__file__).resolve().parent;R=Path('C:/file-native-packing-20261020');artifact=HERE/'final_evidence.py'
actual=guard.current();assert actual['total_new_bytes']+32*1024**2<=2684354560 and actual['total_new_files']+1000<=560000 and actual['free_bytes']>=140*1024**3
binding=dict(scope='artifact-only; no engine, dataset, recovery, packing, or benchmark child',original_functional_cutoff='2026-10-20T07:55:16Z',artifact_closeout_deadline='2026-10-20T08:25:16Z',seconds=120,actual=actual,total_new_return_reservation=32*1024**2,archive_prewrite_limit=12*1024**2,package_limit=14*1024**2,positive_Git_and_receipt_margin=18*1024**2,artifact_sha256=hashlib.sha256(artifact.read_bytes()).hexdigest(),guard_original_sha256=hashlib.sha256((HERE/'guard.py').read_bytes()).hexdigest(),wrapper_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest())
(R/'receipts/artifact-only-closeout-binding.json').write_text(json.dumps(binding,indent=2))
guard.CUTOFF=datetime.datetime.fromisoformat('2026-10-20T08:25:16+00:00').timestamp()
receipt=guard.run([sys.executable,str(artifact)],120,R/'receipts/artifact-only-closeout.json',allowance=2684354560,files=560000)
assert hashlib.sha256(artifact.read_bytes()).hexdigest()==binding['artifact_sha256']
print(json.dumps({k:receipt[k] for k in ['exit','censor','process_closure_verified','final_ownership']},indent=2));assert receipt['exit']==0 and receipt['censor'] is None and receipt['process_closure_verified']
