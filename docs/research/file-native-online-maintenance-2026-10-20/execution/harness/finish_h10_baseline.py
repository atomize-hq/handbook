"""Finish admitted same-cohort H10 controls only after positive producer closure."""
import argparse
import json
from pathlib import Path
import sys
import time
from job_guard import FUNCTIONAL, run, sha_file, supervisors, supervisor_rss, write_json

parser = argparse.ArgumentParser()
parser.add_argument('--budget', required=True)
args = parser.parse_args()
h = Path(__file__).resolve().parent
runtime = Path('C:/file-native-online-matrix-20261020')
r = runtime/'receipts'
root = runtime/'history/authentic-N128/disposable/store'
producer_receipt = r/'history-9900-10000-guard.json'
supervisor_rss(supervisors())
deadline = min(time.time()+1200, FUNCTIONAL)
while not producer_receipt.exists():
    assert time.time() < deadline, 'producer-closure-wait-deadline'
    supervisor_rss()
    time.sleep(.25)
producer = json.loads(producer_receipt.read_bytes())
assert producer['exit'] == 0 and not producer['censor'] and producer['process_closure_verified']
assert not producer['errors']
summary = json.loads((r/'history-10000-summary.json').read_bytes())
assert summary['actual_H'] == 10001 and summary['readable_A_current_qualified']
exe = 'C:/file-native-packing-20261020/bin/candidate2.exe'
assert sha_file(exe) == '568f631acf528c7de911bb88ec85d07ead9d7b4435ebe970c61053f06e31a93b'
stages = [
    ('H10000-request-intents', 'qualify_history.py',
     ['--root',str(root),'--output',str(r/'history-10000-intent-independent.json')],
     {'receipts':dict(bytes=1024**2,files=10)},600),
    ('H10000-matched-baseline', 'baseline.py',
     ['history-match','--target','10000','--in-place'],
     {'history':dict(bytes=16*1024**2,files=1000),'receipts':dict(bytes=4*1024**2,files=20)},600),
    ('H10000-L-old-lookup', 'lookup_samples.py',
     ['--exe',exe,'--root',str(root),'--baseline',str(r/'H10000-baseline.json'),
      '--output',str(r/'H10000-L-old-lookup'),'--fresh','50'],
     {'receipts':dict(bytes=4*1024**2,files=10)},600),
    ('H10000-PZ-original', 'prepare_baseline_pz.py',
     ['--root',str(root),'--name','H10000'],
     {'history':dict(bytes=48*1024**2,files=1000),'receipts':dict(bytes=4*1024**2,files=20)},900),
    ('H10000-PZ-old-lookup', 'lookup_samples.py',
     ['--exe',exe,'--root',str(root),'--baseline',str(r/'H10000-baseline.json'),
      '--output',str(r/'H10000-PZ-old-lookup'),'--fresh','50'],
     {'receipts':dict(bytes=4*1024**2,files=10)},600),
]
outcomes = []
try:
    for name, script, fields, reserve, seconds in stages:
        receipt = r/(name+'-guard.json')
        assert not receipt.exists(), 'never-overwrite-prior-stage-receipt'
        write_json(r/(name+'-source-binding.json'),dict(
                   baseline_executable_sha256=sha_file(exe),
                   files=[dict(path=str(p),sha256=sha_file(p)) for p in sorted(h.glob('*.py'))]))
        result = run([sys.executable,'-B',str(h/script),*fields],receipt,args.budget,seconds,reserve)
        compact = {key:result.get(key) for key in ['exit','censor','errors','process_closure_verified','token_wait_seconds']}
        compact['stage'] = name
        outcomes.append(compact)
        print(json.dumps(compact),flush=True)
        assert result['exit'] == 0 and not result['censor'] and result['process_closure_verified'] and not result['errors'], name
finally:
    write_json(r/'H10000-baseline-pipeline.json',dict(
               producer_receipt=str(producer_receipt),producer_receipt_sha256=sha_file(producer_receipt),
               stages=outcomes,no_second_H10_clone=True,no_long_lane=True,
               qualification='baseline controls only; final candidate matrix still pending'))
