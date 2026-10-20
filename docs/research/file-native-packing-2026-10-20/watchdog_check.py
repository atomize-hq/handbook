"""Deadline and RSS sampler continue while census and child stdin block."""
import hashlib,json,sys,time
from pathlib import Path
import guard
root=guard.ROOT/'watchdog-check';root.mkdir(exist_ok=False)
original=guard.current
def slow():time.sleep(3);return original()
guard.current=slow
stdin=root/'stdin';stdin.write_bytes(b'x'*(1024*1024))
start=time.time()
r=guard.run([sys.executable,'-c','import time;time.sleep(30)'],1,root/'receipt.json',stdin=stdin)
t=r['independent_deadline'];assert r['censor']=='time-censor' and r['exit']!=0
assert t['requested_epoch']-start<1.3,(t,start)
assert t['child_exit_epoch']-start<1.5,(t,start)
assert not r['group_remaining_before_closure']
assert r['sampled_peak_rss_by_pid'] is not None
proof=dict(status='passed',condition='census deliberately sleeps 3s; 1MiB stdin to non-reading 30s child; independent deadline 1s',termination_after_seconds=t['child_exit_epoch']-start,supervisor_elapsed_seconds=time.time()-start,guard_sha256=hashlib.sha256(Path(guard.__file__).read_bytes()).hexdigest(),receipt=str(root/'receipt.json'))
(root/'proof.json').write_text(json.dumps(proof,indent=2));print(json.dumps(proof,indent=2))
