"""Read-only prospective record-container accounting from captured raw inventory."""
import collections,json,math,time
from pathlib import Path
from independent import role,MAX_RAW
root=Path('C:/file-native-packing-20261020');roles=collections.defaultdict(lambda:dict(files=0,raw_bytes=0,index_entry_bytes=0,oversize_files=0,oversize_bytes=0));start=time.monotonic()
for line in (root/'post-gc-operational-backup/files.jsonl').open('rb'):
    row=json.loads(line);r=role(row['path'])
    if r is None:continue
    d=roles[r]
    if row['bytes']>MAX_RAW:d['oversize_files']+=1;d['oversize_bytes']+=row['bytes'];continue
    d['files']+=1;d['raw_bytes']+=row['bytes'];d['index_entry_bytes']+=len(row['path'].encode())+50
raw=sum(r['raw_bytes'] for r in roles.values());index=sum(r['index_entry_bytes'] for r in roles.values())+64*53
result=dict(source='authenticated exact H101006 operational-capture files.jsonl; role-potential upper bound, final accepted/finalized eligibility rechecked by maintenance',roles=roles,eligible_raw_bytes=raw,index_upper_bytes=index,
P0_max_new_owned_bytes=raw+index+2*1024**2,PZ_new_owned_cap_bytes=350000000,PZ_exhaustion='censor before reserved write; never silently enlarge',
proposed_total_P_cap_bytes=2952790016,proposed_total_P_cap_files=560000,
preservation_complete_summary=str(root/'preservation-reopen/summary.json'),latest_complete_ownership_receipt=str(root/'receipts/candidate-build.json'),
proposed_stage='after complete small final-candidate fault/byte/restore pass and parent GO: freeze/measure qualified-old L plus new-binary L under fixed navigation; build P0 without deletion; independently verify; measure P0; build same frozen inventory/grouping PZ without deletion within reservation; independently verify and measure; return actual census before chosen long cleanup',
large_cleanup_admitted=False,packed_restore_admitted=False,native_admitted=False,
later_coexistence_constraints='Final packed archive+restore and N8k A/B need independent admissions after actual packing cleanup. Generated successful cache-bound fixtures currently ~148MB, and bounded record-subset containers may be archived/verified and reclaimed only through separately reviewed exact-target cleanup if needed. No protected originals/censors/cache residues are proposed for deletion.',
fixed_functional_cutoff='2026-10-20T07:55:16Z',fixed_closeout_deadline='2026-10-20T08:25:16Z',seconds=time.monotonic()-start)
(root/'receipts/large-packing-stage-forecast.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
