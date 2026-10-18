"""Fresh-process exact accepted outcomes after completed or censored collection."""
import argparse,json
from pathlib import Path
from controller import Driver
from checks import expect_error

a=argparse.ArgumentParser();a.add_argument('--scratch',required=True);a.add_argument('--exe',required=True);ns=a.parse_args()
scratch=Path(ns.scratch);root=scratch/'history-common-pin/disposable/store'
first=json.loads((root/'expected-ledger.jsonl').open().readline())
before=json.loads((scratch/'history-gc-continuation/events.jsonl').open().readline())
assert before['input']['request_id']=='deep-lost-reply' and before['output']['result']['outcome']=='accepted'
d=Driver(ns.exe,scratch/'history-GC-outcome-verification',180)
assert d.call(dict(action='resolve',root=str(root),request_id='history-000001'))['result']==first['result']
assert d.call(dict(action='resolve',root=str(root),request_id='deep-lost-reply'))['result']==before['output']['result']
expect_error(d.call(dict(action='point',root=str(root),selected=first['selected'],id='d000001'),allow=True),'retired')
print(json.dumps(dict(early_and_lost_original_outcomes_exact=True,retired_snapshot_refused=True),indent=2))
