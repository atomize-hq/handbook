import hashlib,json,sys,time
from pathlib import Path
import guard
root=guard.ROOT/'watchdog-final-check';root.mkdir(exist_ok=False)
# Actual current ownership once, then a deliberately slow synthetic census.
# The synthetic snapshots are watchdog fault inputs, not fresh ownership claims.
snapshot=guard.current()
def slow():time.sleep(3);return {**snapshot,'synthetic_slow_census':True}
guard.current=slow;stdin=root/'stdin';stdin.write_bytes(b'x'*1024**2);rows=[]
for label,command,input_path in [
 ('blocked-stdin',[sys.executable,'-c','import time;time.sleep(30)'],stdin),
 ('root-exits-descendant-survives',[sys.executable,'-c','import subprocess,sys;subprocess.Popen([sys.executable,"-c","import time;time.sleep(30)"])'],None)]:
    start=time.time();r=guard.run(command,1,root/(label+'.json'),stdin=input_path,allowance=2415919104,files=560000)
    t=r['independent_deadline'];assert r['censor']=='time-censor' and t['group_after_termination']==[] and r['process_closure_verified']
    assert t['group_termination_observed_epoch']-start<1.5
    if label.startswith('root-exits'):assert r['exit']==0 and r['group_observations']['root_exit_seen_epoch']<t['requested_epoch']
    rows.append(dict(case=label,group_terminated_after_seconds=t['group_termination_observed_epoch']-start,receipt=str(root/(label+'.json'))))
# Normal completed compiler/service descendants must also close promptly without
# changing an actual successful root exit into a stage-time censor.
guard.current=lambda:snapshot
r=guard.run([sys.executable,'-c','import subprocess,sys,time;subprocess.Popen([sys.executable,"-c","import time;time.sleep(30)"]);time.sleep(.1)'],3,root/'normal-completion.json',allowance=2415919104,files=560000)
assert r['exit']==0 and r['censor'] is None and r['process_closure_verified'] and r['independent_deadline']['reason']=='root-completed-group-cleanup'
rows.append(dict(case='normal-root-completion-owned-descendant-cleanup',receipt=str(root/'normal-completion.json')))
proof=dict(status='passed',guard_sha256=hashlib.sha256(Path(guard.__file__).read_bytes()).hexdigest(),cases=rows)
(root/'proof.json').write_text(json.dumps(proof,indent=2));print(json.dumps(proof,indent=2))
