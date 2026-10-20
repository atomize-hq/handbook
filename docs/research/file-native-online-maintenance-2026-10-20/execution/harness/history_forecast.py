"""Read-only quiescent boundary forecast under the shared Windows token."""
import hashlib
import json
import os
from pathlib import Path
from job_guard import Budget, write_json

runtime = Path('C:/file-native-online-matrix-20261020')
root = runtime/'history/authentic-N128/disposable/store'
selected_bytes = (root/'SELECTED.json').read_bytes()
selected = json.loads(selected_bytes)
raw = (root/f"publications/{selected['publication']}.json").read_bytes()
assert hashlib.sha256(raw).hexdigest() == selected['digest']
publication = json.loads(raw)
completed = publication['sequence']
assert 0 <= completed <= 10000 and completed % 100 == 0
parts = dict(remaining_history_publications=(10000-completed)*65536,
             all_1200_required_rolling_publications=1200*65536,
             physical_representation_coexistence=64*1024**2,
             full_frozen_inventories=16*1024**2,
             generation_deferred_and_journal_bookkeeping=16*1024**2,
             chunk_and_handoff_temporary_slack=16*1024**2)
measured = Budget(os.environ['FILE_NATIVE_BUDGET']).check({})
history = measured['categories']['history']
assert selected_bytes == (root/'SELECTED.json').read_bytes()
row = dict(completed_history_requests=completed, actual_H=completed+1,
           remaining_history_requests=10000-completed,
           current_history=history, parts=parts,
           future_additional_bytes=sum(parts.values()),
           logical_plus_future_bytes=history['absolute_actual']['bytes']+sum(parts.values()),
           census=measured, selected=selected, source_selected_unchanged=True,
           basis='64KiB per remaining/rolling accepted publication, above measured chunks; stage actual checks mandatory',
           no_second_H10k_clone=True, deletion_credit=0,
           qualification='forecast only; physical geometry and complete final representation remain unmeasured')
path = runtime/'receipts'/f'history-{completed}-boundary-forecast.json'
assert not path.exists()
write_json(path, row)
print(json.dumps({k:row[k] for k in ['completed_history_requests','actual_H','current_history','future_additional_bytes','logical_plus_future_bytes']}))
