"""One guarded100-publication stage at a time, yielding the shared token."""
import argparse
import json
from pathlib import Path
import sys
from job_guard import Budget, run, sha_file, write_json

parser = argparse.ArgumentParser()
parser.add_argument('--budget', required=True)
parser.add_argument('--target', required=True, type=int, choices=[1000, 10000])
parser.add_argument('--chunks', type=int, default=10)
args = parser.parse_args()
runtime = Path('C:/file-native-online-matrix-20261020')
root = runtime/'history/authentic-N128/disposable/store'
baseline = Path(__file__).with_name('baseline.py')
for _ in range(args.chunks):
    selected = json.loads((root/'SELECTED.json').read_bytes())
    raw = (root/f"publications/{selected['publication']}.json").read_bytes()
    import hashlib
    assert hashlib.sha256(raw).hexdigest() == selected['digest']
    publication = json.loads(raw)
    offset = publication['sequence']
    if offset >= args.target:
        break
    end = min(offset+100, args.target)
    receipt = runtime/'receipts'/f'history-{offset}-{end}-guard.json'
    assert not receipt.exists(), 'never-overwrite-a-censored-stage'
    Budget(args.budget).check({'history': dict(bytes=(end-offset)*65536+16*1024**2,
                                              files=(end-offset)*32+5000)})
    write_json(runtime/'receipts'/f'history-{offset}-{end}-source-binding.json',
               dict(files=[dict(path=str(p.resolve()), sha256=sha_file(p))
                           for p in sorted(baseline.parent.glob('*.py'))]))
    result = run([sys.executable, '-B', str(baseline), 'history-append',
                  '--target', str(args.target), '--chunk', '100'], receipt,
                  args.budget, 1200 if end == args.target else 180)
    print(json.dumps(dict(start=offset, end=end, exit=result.get('exit'),
                          censor=result.get('censor'), token_wait_seconds=result.get('token_wait_seconds'),
                          process_closure_verified=result.get('process_closure_verified'))), flush=True)
    if result.get('exit') != 0 or result.get('censor') or not result.get('process_closure_verified'):
        sys.exit(1)
