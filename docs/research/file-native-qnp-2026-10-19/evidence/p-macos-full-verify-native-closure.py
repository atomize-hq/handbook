"""Read-only verification of retained native closure observations and PGID coverage.
Usage: python3 -B verify-native-closure.py AUDIT_JSON ORIGINAL_NATIVE_RECEIPTS_DIR
This verifies evidence; it does not perform a new live process census or send signals.
"""
import sys,json,gzip
from pathlib import Path
p=Path(sys.argv[1]);r=Path(sys.argv[2]);a=json.loads(p.read_bytes());groups={8284}
for n in ['raw-process-events.jsonl.gz','rss-samples.jsonl.gz']:
 with gzip.open(r/n,'rt') as f:
  for line in f:
   row=json.loads(line);groups.add(row['guard']['pgid'] if 'guard' in row else row['pgid'])
first=a['original_cutoff_rows_available_columns'];assert first['closure_at_that_observation']=='UNKNOWN' and first['state'] is None and first['UID'] is None and first['RSS_bytes']==0 and first['signal_zero_errno']==1
obs=a['later_native_numeric_PGID_observations'];assert len(obs)==2
for x in obs:
 assert x['ps_columns']==['PID','PPID','PGID','state','UID','RSS_KiB','args'] and x['self_command_text_matches_used'] is False
 assert x['ps_exit']==0 and x['matching_numeric_PGID_rows']==[] and set(x['requested_PGIDs'])==groups
assert len(groups)==368 and a['later_all_requested_PGIDs_absent'] is True and a['original_cutoff_failure_preserved'] is True
print(json.dumps({'status':'PASS_EVIDENCE_VERIFICATION_ONLY','PGIDs':len(groups),'original_cutoff_closure':'UNKNOWN','later_numeric_PGID_absence_times':[x['utc'] for x in obs],'no_new_live_census_or_signals':True}))
