"""Serial same-process resource/error samples after the pristine fresh effect."""
import json,pathlib,sys
from run_pairs import HERE,root,run
base=pathlib.Path(sys.argv[1]);ref=pathlib.Path(sys.argv[2])
with (HERE/'resource-measurements.jsonl').open('w',encoding='utf-8') as out:
    for variant,exe in [('baseline',base),('refactor',ref)]:
        rows=run([exe,'resources',root('pristine','explicit',8,0,variant),'explicit',8,0,variant,'on'])
        assert sum(row['event']=='resource' for row in rows)==48
        assert sum(row['event']=='lock_wait' for row in rows)==1
        complete=[row for row in rows if row['event']=='resources_complete'];assert len(complete)==1
        assert complete[0]['handles_before']==complete[0]['handles_after']
        for row in rows:
            if row['event']=='resource':
                assert row['handles_before']==row['handles_after']
                assert row['profile']['retained_locks_after_phase']==0
            out.write(json.dumps(row)+'\n')
        out.flush();print(json.dumps({'resource_variant_complete':variant,'operations':48}),flush=True)
