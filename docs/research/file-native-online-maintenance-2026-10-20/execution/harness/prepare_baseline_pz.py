"""Original whole-index PZ reader control on the same authentic accepted cohort."""
import argparse
import json
from pathlib import Path
import sys
from baseline import BASELINE, EXPECTED, RUNTIME, RESEARCH
from driver import Driver
from job_guard import Budget, sha_file, write_json

parser = argparse.ArgumentParser()
parser.add_argument('--root', required=True)
parser.add_argument('--name', required=True)
args = parser.parse_args()
assert sha_file(BASELINE) == EXPECTED
root = Path(args.root)
before = (root/'SELECTED.json').read_bytes()
assert json.loads((root/'STORE.json').read_bytes())['version'] == 1
import os
Budget(os.environ['FILE_NATIVE_BUDGET']).check({'history': dict(bytes=32*1024**2, files=1000)})
sys.path.insert(0, str(RESEARCH/'file-native-packing-2026-10-20'))
from independent import freeze, verify
inventory = root.parents[2]/(args.name+'-frozen-inventory.jsonl')
assert not inventory.exists()
frozen = freeze(root, inventory, True)
d = Driver(BASELINE, RUNTIME/'receipts'/(args.name+'-PZ-build-events.jsonl'))
response = d.call(root, 'packing', operation='build', codec='zstd', include_attempts=True, cleanup=False)
assert before == (root/'SELECTED.json').read_bytes()
independent = verify(root, RUNTIME/'receipts'/(args.name+'-PZ-independent.json'), inventory)
from outcome_oracle import verify as verify_intents
intents = verify_intents(root)
write_json(RUNTIME/'receipts'/(args.name+'-PZ-baseline.json'), dict(response=response,
           full_independent=independent, request_intents=intents, frozen_records=len(frozen),
           exact_frozen_inventory=str(inventory), selected_unchanged=True,
           qualification='original whole-relevant-index reader; loose duplicates retained and fully charged; no deletion or file-reduction credit'))
print(json.dumps(dict(status='passed', frozen_records=len(frozen), actual_H=intents['actual_H'])))
