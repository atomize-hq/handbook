"""Fresh H1k census and explicit full F05 continuation forecast; no deletion credit."""
import json
import os
from pathlib import Path
import sys
from job_guard import Budget, write_json

runtime = Path('C:/file-native-online-matrix-20261020')
receipts = runtime/'receipts'
genesis = json.loads((receipts/'history-genesis.json').read_bytes())
previous = genesis['corpus']['bytes']
growth = []
for end in range(100, 1001, 100):
    row = json.loads((receipts/f'history-{end-100}-{end}-chunk.json').read_bytes())
    size = row['corpus']['bytes']
    growth.append(dict(accepted=100, logical_growth=size-previous))
    previous = size
assert max(r['logical_growth'] for r in growth) <= 100*65536
intents = json.loads((receipts/'history-1000-intent-independent.json').read_bytes())
assert intents['publications'] == 1001 and intents['authentic_history_recipe_intents'] == 1000
baseline = json.loads((receipts/'H1000-baseline.json').read_bytes())
assert baseline['actual_H'] == 1002 and baseline['exact_retry_deduplicated'] and baseline['changed_intent_refused']
parts = dict(remaining_9000_history_publications=9000*65536,
             all_1200_required_rolling_cycle_publications=1200*65536,
             new_and_old_physical_representations_coexistence=64*1024**2,
             full_frozen_inventories=16*1024**2,
             generation_deferred_and_journal_bookkeeping=16*1024**2,
             chunk_and_handoff_temporary_slack=16*1024**2)
prospective = sum(parts.values())
measured = Budget(os.environ['FILE_NATIVE_BUDGET']).check(
    {'history': dict(bytes=prospective, files=340000)})
sys.path.insert(0, 'C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook/docs/research/file-native-packing-2026-10-20')
import allocation
native = allocation.census(runtime/'history/authentic-N128/disposable/store', receipts/'history-1000-native-allocation.json')
write_json(receipts/'H10000-prospective-admission.json', dict(status='admitted-staged-continuation',
           parts=parts, prospective_additional_bytes=prospective, current_and_prospective=measured,
           observed_100publication_chunk_growth=growth, native_H1k_allocation=native,
           growth_basis='64KiB logical owned-growth forecast per accepted publication, above all observed100publication chunks; stage checks and actual cap remain mandatory',
           H10k_representation_plan='same fresh-owned operational source measured then physically converted; no extra full H10k clone; same frozen accepted cohort and exact raw identities',
           protected_originals_unchanged=True, planned_deletion_credit=0,
           qualification='prospective forecast, not a universal bound or measured future peak; abort each100-publication stage if actual/admitted bounds fail'))
print(json.dumps(dict(status='admitted-staged-continuation', prospective_additional_bytes=prospective,
                      current_history=measured['categories']['history']['absolute_actual'])))
