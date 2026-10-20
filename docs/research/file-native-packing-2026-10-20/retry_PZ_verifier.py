"""Fresh complete verification after independent DCtx serialization correction."""
import hashlib,json
from pathlib import Path
from guard import current
import final_long_matrix

r=Path('C:/file-native-packing-20261020');here=Path(__file__).resolve().parent;a=current();assert a['total_new_bytes']+33554432<=2766562918 and a['total_new_files']+2000<=560000 and a['free_bytes']>=140*1024**3
evidence=dict(actual=a,cap=2766562918,all_remaining_verification_measurement_margin=33554432,prospective=a['total_new_bytes']+33554432,core_unchanged='d70f3864f2888d46d9112213b4a03f8d82745a8e',independent_source_sha256=hashlib.sha256((here/'independent.py').read_bytes()).hexdigest(),previous_failure_preserved='final-PZ-verify.json',diagnostic_before='verifier-context-before.json',diagnostic_after='verifier-context-after.json',scope='same complete raw and semantic oracle; only serialize independent decoder-owned DCtx; no engine substitution or data mutation',documentation='https://facebook.github.io/zstd/doc/api_manual_v1.5.7.html#Chapter3')
path=r/'receipts/final-PZ-verifier-after-lock-binding.json';assert not path.exists();path.write_text(json.dumps(evidence,indent=2));final_long_matrix.complete();proof=r/'long-matrix-fixed2/PZ-independent.json';evidence.update(status='complete-verification-passed',proof_sha256=hashlib.sha256(proof.read_bytes()).hexdigest());(r/'receipts/final-PZ-verifier-after-lock-result.json').write_text(json.dumps(evidence,indent=2))
