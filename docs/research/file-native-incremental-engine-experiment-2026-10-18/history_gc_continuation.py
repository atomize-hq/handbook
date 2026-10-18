"""Separately timed, quiescent relocation/retirement/GC of the owned history run."""
import argparse,json,time
from pathlib import Path
from controller import Driver,sha
from checks import expect_error

a=argparse.ArgumentParser();a.add_argument('--scratch',required=True);a.add_argument('--exe',required=True);a.add_argument('--deadline-epoch',type=float,required=True);ns=a.parse_args()
scratch=Path(ns.scratch).resolve();assert scratch==Path('C:/file-native-incremental-20261018').resolve()
assert (scratch/'final-verification-v3/completed.json').is_file()
old=(scratch/'history-common-pin/store').resolve();new=(scratch/'history-common-pin/disposable/store').resolve()
assert old.is_relative_to(scratch) and new.is_relative_to(scratch) and not new.exists()
assert old.parent==scratch/'history-common-pin' and new.parent.parent==old.parent
base=scratch/'history-gc-continuation';assert not base.exists()
d=Driver(ns.exe,base,min(860,max(0,ns.deadline_epoch-time.time()-40)))
names=['STORE.json','SELECTED.json','expected-ledger.jsonl'];before={name:sha((old/name).read_bytes()) for name in names}
first=json.loads((old/'expected-ledger.jsonl').open().readline())
lost=d.call(dict(action='resolve',root=str(old),request_id='deep-lost-reply'))['result']
assert lost['outcome']=='accepted'
new.parent.mkdir(exist_ok=False);old.rename(new)
assert {name:sha((new/name).read_bytes()) for name in names}==before
record=dict(old_root=str(old),new_root=str(new),quiescent_after_queue=True,unchanged_sha256=before,separate_continuation=True)
(base/'relocation.json').write_text(json.dumps(record,indent=2))
d.call(dict(action='lifecycle',root=str(new),operation='setup'))
d.call(dict(action='lifecycle',root=str(new),operation='retire',selected=first['selected']))
expect_error(d.call(dict(action='point',root=str(new),selected=first['selected'],id='d000001'),allow=True),'retired')
collection=d.call(dict(action='lifecycle',root=str(new),operation='collect'),cap=900,allow=True)
record['collection']=collection
assert d.call(dict(action='resolve',root=str(new),request_id='history-000001'))['result']==first['result']
assert d.call(dict(action='resolve',root=str(new),request_id='deep-lost-reply'))['result']==lost
pub=json.loads((new/f"publications/{first['selected']['publication']}.json").read_bytes())
record.update(early_and_lost_outcomes_exact=True,retired_candidate_exists=(new/f"attempts/{pub['attempt']}/candidate.json").exists())
(base/'summary.json').write_text(json.dumps(record,indent=2))
print(json.dumps(record,indent=2))
