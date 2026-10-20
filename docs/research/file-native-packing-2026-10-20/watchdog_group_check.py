"""Root and descendant deadlines under deliberately blocked census/stdin."""
import hashlib,json,sys,time
from pathlib import Path
import guard
root=guard.ROOT/'watchdog-group-check';root.mkdir(exist_ok=False)
original=guard.current
def slow():time.sleep(3);return original()
guard.current=slow
stdin=root/'stdin';stdin.write_bytes(b'x'*(1024*1024))
proofs=[]
for label,command,input_path in [
 ('blocked-stdin',[sys.executable,'-c','import time;time.sleep(30)'],stdin),
 ('root-exits-descendant-survives',[sys.executable,'-c','import subprocess,sys;subprocess.Popen([sys.executable,"-c","import time;time.sleep(30)"])'],None)]:
    start=time.time();r=guard.run(command,1,root/(label+'.json'),stdin=input_path)
    t=r['independent_deadline'];assert r['censor']=='time-censor'
    assert t['requested_epoch']-start<1.3,(t,start)
    assert t['group_termination_observed_epoch']-start<1.5,(t,start)
    assert t['group_after_termination']==[] and r['group_remaining_before_closure']==[]
    assert r['sampled_peak_rss_by_pid'] is not None and r['job_query_errors']==0
    if label.startswith('root-exits'):
        assert r['exit']==0 and r['group_observations']['root_exit_seen_epoch']<t['requested_epoch']
        assert len(t['members_before_termination'])>=1
    proofs.append(dict(case=label,group_terminated_after_seconds=t['group_termination_observed_epoch']-start,root_exit=r['exit'],receipt=str(root/(label+'.json'))))
proof=dict(status='passed',guard_sha256=hashlib.sha256(Path(guard.__file__).read_bytes()).hexdigest(),cases=proofs)
(root/'proof.json').write_text(json.dumps(proof,indent=2));print(json.dumps(proof,indent=2))
