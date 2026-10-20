"""Separately admitted exact image reclaim through the unchanged reviewed PowerShell helper."""
import argparse, datetime, hashlib, json, sys, time
from pathlib import Path

HERE = Path(__file__).resolve().parent
OLD = HERE.parent
sys.path.insert(0, str(OLD))
import guard

guard.CUTOFF = datetime.datetime.fromisoformat('2026-10-20T12:53:00+00:00').timestamp()
assert time.time() < guard.CUTOFF
p = argparse.ArgumentParser()
p.add_argument('slot', choices=['baseline', 'L-old-path-v2', 'L-new-path-v2', 'P0-path-v2', 'PZ-path-v2'])
p.add_argument('--admission', required=True)
a = p.parse_args()
cfg = json.loads((HERE / 'preparation.json').read_bytes())
admission_path = Path(a.admission).resolve()
admission = json.loads(admission_path.read_bytes())
assert admission['scope'] == 'P-followup-20261020-B-reclaim' and admission['slot'] == a.slot and admission['go'] is True
assert admission['functional_cutoff'] == cfg['functional_cutoff']
proof = Path(admission['parent_proof']).resolve()
assert hashlib.sha256(proof.read_bytes()).hexdigest() == admission['parent_proof_sha256']
helper = OLD / 'reclaim_operational_image_fast_slots.ps1'
assert hashlib.sha256(helper.read_bytes()).hexdigest() == '7d72b0e5af31f86f4903450a9bd400f6d974b84ec2075d1813074374c4c0f5be'
case = guard.ROOT / 'operational-matrix/n8000-B'
root = case / a.slot / 'disposable/store'
capsule = case / (a.slot + '-operational-backup')
manifest = json.loads((capsule / 'manifest.json').read_bytes())
assert manifest['files'] == admission['files'] and manifest['logical_bytes'] == admission['logical_bytes']
for name, key in [('manifest.json', 'manifest_sha256'), ('files.jsonl', 'inventory_sha256'), ('operational.bin.gz', 'transport_sha256')]:
    assert hashlib.sha256((capsule / name).read_bytes()).hexdigest() == admission[key]
out = guard.ROOT / 'followup-20261020/receipts'
receipt = out / ('reclaim-' + a.slot + '-job.json')
deletion_receipt = guard.ROOT / 'receipts' / ('followup-B-' + a.slot + '-reclaim.json')
assert not receipt.exists() and not deletion_receipt.exists()
actual = guard.current()
assert actual['total_new_bytes'] + 16*1024**2 <= cfg['windows_bytes_cap'] and actual['total_new_files'] + 100 <= cfg['windows_files_cap'] and actual['free_bytes'] >= cfg['free_bytes_min']
(out / ('reclaim-' + a.slot + '-binding.json')).write_text(json.dumps(dict(actual=actual, launcher_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(), parent_admission=str(admission_path), parent_admission_sha256=hashlib.sha256(admission_path.read_bytes()).hexdigest(), source_root=str(root.resolve()), capsule=str(capsule.resolve()), helper_sha256='7d72b0e5af31f86f4903450a9bd400f6d974b84ec2075d1813074374c4c0f5be', controls_and_directories_retained=True), indent=2))
argv = ['powershell', '-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', str(helper), '-Root', str(root.resolve()), '-CapsuleRoot', str(capsule.resolve()), '-ExpectedManifest', admission['manifest_sha256'], '-ExpectedInventory', admission['inventory_sha256'], '-ExpectedTransport', admission['transport_sha256'], '-ParentProof', str(proof), '-Receipt', str(deletion_receipt), '-Execute']
result = guard.run(argv, 900, receipt, cwd=guard.WORK, allowance=2684354560, files=560000)
print(json.dumps({k: result[k] for k in ['exit', 'censor', 'process_closure_verified', 'seconds', 'final_ownership']}, indent=2))
assert result['exit'] == 0 and result['censor'] is None and result['process_closure_verified']
