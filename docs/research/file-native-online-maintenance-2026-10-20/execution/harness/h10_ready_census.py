"""Quiescent ready-cohort F05 census and unchanged complete continuation forecast."""
import json
import os
from pathlib import Path
import sys
from job_guard import Budget, sha_file, write_json

runtime = Path('C:/file-native-online-matrix-20261020')
r = runtime/'receipts'
pipeline = json.loads((r/'H10000-baseline-pipeline.json').read_bytes())
assert len(pipeline['stages']) == 5
assert all(s['exit'] == 0 and not s['censor'] and not s['errors'] and s['process_closure_verified'] for s in pipeline['stages'])
base = json.loads((r/'H10000-baseline.json').read_bytes())
packed = json.loads((r/'H10000-PZ-baseline.json').read_bytes())
assert base['actual_H'] == packed['request_intents']['actual_H'] == 10002
assert packed['frozen_records'] == 50010
root = Path(base['fixture'])
selected_bytes = (root/'SELECTED.json').read_bytes()
assert json.loads(selected_bytes) == base['selected']
parts = dict(all_1200_required_rolling_publications=1200*65536,
             future_physical_representation_coexistence=64*1024**2,
             full_frozen_inventories=16*1024**2,
             generation_deferred_and_journal_bookkeeping=16*1024**2,
             chunk_and_handoff_temporary_slack=16*1024**2)
future_bytes = sum(parts.values())
future_files = 1200*32+13600
measured = Budget(os.environ['FILE_NATIVE_BUDGET']).check({'history':dict(bytes=future_bytes,files=future_files)})
sys.path.insert(0,'C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook/docs/research/file-native-packing-2026-10-20')
import allocation
native = allocation.census(root,r/'H10000-native-allocation.json')
assert selected_bytes == (root/'SELECTED.json').read_bytes()
current = measured['categories']['history']['absolute_actual']
row = dict(actual_H=10002,selected=base['selected'],complete_accepted_cohort=True,
           history_area_current=current,parts=parts,future_additional_bytes=future_bytes,
           future_additional_files=future_files,
           current_plus_future_bytes=current['bytes']+future_bytes,
           current_plus_future_files=current['files']+future_files,
           native_operational_store=native,census=measured,
           old_PZ_frozen_inventory_records=50010,source_selected_unchanged=True,
           planned_deletion_credit=0,no_second_H10_clone=True,
           qualification='forecast retains all1200 rolling publications and future coexistence; actual stage/cap checks mandatory; native regular-file allocation does not measure directory/device allocation')
write_json(r/'H10000-ready-F05-census.json',row)
print(json.dumps({k:row[k] for k in ['actual_H','history_area_current','future_additional_bytes','current_plus_future_bytes','current_plus_future_files']}))
