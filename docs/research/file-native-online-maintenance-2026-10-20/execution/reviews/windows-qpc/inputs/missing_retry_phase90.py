"""Separate parent-authorized90s corrective phase; execute ONLY after fresh GO and prior Job closure."""
import ast,gzip,hashlib,importlib.util,json,msvcrt,os,sys,threading,time
from pathlib import Path
BASE=Path('C:/file-native-online-matrix-20261020');OUT=BASE/'receipts/windows-cadence'
SRC=Path('C:/Users/spmcc/.codex/worktrees/file-native-online-matrix-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/windows-cadence/job_guard_qpc.py')
BUDGET=BASE/'receipts/budget-final-source-v3.json'
EXPECTED='50ecc7c40c1cdda6493817261b5755fe2d823fdae271118698828c39a0400378'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
# Original300s phase is CLOSED/incomplete. This separately authorized90s phase
# starts ONLY when parent later executes this NEW input after positive token closure.
PHASE_START=time.time();ABSOLUTE_END=PHASE_START+90
assert sha(SRC)==EXPECTED
s=importlib.util.spec_from_file_location('cadence_retry',SRC);G=importlib.util.module_from_spec(s);s.loader.exec_module(G)
lines=SRC.read_text().splitlines();watch=next(n for n in ast.walk(ast.parse(SRC.read_bytes())) if isinstance(n,ast.FunctionDef) and n.name=='memory_watch')
missing=next(i+1 for i,v in enumerate(lines) if watch.lineno<=i+1<=watch.end_lineno and v.strip()=='if value is None:')
hits=[]
def trace(frame,event,arg):
 if frame.f_code.co_filename!=str(SRC) or frame.f_code.co_name!='memory_watch':return None
 if event=='line' and frame.f_lineno==missing and not hits:
  local=frame.f_locals
  hits.append(dict(pid=local['pid'],native_value=local['value'],injected_value=None))
  local['value']=None
 return trace
# Reserve the complete new live prefix and final receipts before any child.
pre=G.Budget(BUDGET).check({k:dict(bytes=196608,files=6) for k in ['receipts','windows_cadence_receipts']})
def refuse(why):
 data=(json.dumps(dict(status='CORRECTIVE_NOT_STARTED',reason=why,phase_start_epoch=PHASE_START,phase_end_epoch=ABSOLUTE_END,observed_epoch=time.time(),child_started=False,source_sha256=sha(SRC),input_sha256=sha(__file__),preflight=pre),indent=2)+'\n').encode()
 with (OUT/'corrective-phase-not-started.json').open('xb') as f:f.write(data)
 raise SystemExit(2)
if ABSOLUTE_END-time.time()<=60:refuse('90s-phase-insufficient-after-preflight')
# Never enter the frozen guard's functional-only token wait behind an existing Job.
h=G.TOKEN.open('r+b');h.seek(0)
try:
 try:msvcrt.locking(h.fileno(),msvcrt.LK_NBLCK,1)
 except OSError:refuse('heavy-token-occupied-no-blocking-wait-started')
 h.seek(0);msvcrt.locking(h.fileno(),msvcrt.LK_UNLCK,1)
finally:h.close()
remaining=G.census(OUT)
reserve={k:dict(bytes=min(131072,1048576-remaining['bytes']),files=5) for k in ['receipts','windows_cadence_receipts']}
started=time.time();threading.settrace(trace)
try:
 row=G.run([sys.executable,'-B','-c','import time;time.sleep(20)'],OUT/'missing-owned-rss-v2.json',BUDGET,min(25,ABSOLUTE_END-time.time()-30),reserves=reserve)
finally:threading.settrace(None)
raw=[]
with gzip.open(OUT/'missing-owned-rss-v2.rss.jsonl.gz','rb') as f:
 for line in f:assert line.endswith(b'\n');raw.append(json.loads(line))
 assert f.read()==b''
owner=row['owner'];closed=len(row['closure_checks'])==2 and all(x['members']==[] for x in row['closure_checks']) and row['remaining_job_pids']==[] and owner['state']=='closed' and G.creation_time(owner['job_root_pid'])!=owner['job_root_creation_time']
h=G.TOKEN.open('r+b');h.seek(0)
try:
 msvcrt.locking(h.fileno(),msvcrt.LK_NBLCK,1);h.seek(0);msvcrt.locking(h.fileno(),msvcrt.LK_UNLCK,1);token=True
finally:h.close()
passed=bool(hits) and row['censor']=='memory-monitor-error' and any('unreadable-owned-pid-rss:' in e for e in row['errors']) and owner.get('resumed_epoch') is None and closed and token and time.time()<=ABSOLUTE_END
r=dict(status='CORRECTIVE_INJECTED_FAULT_PASSED' if passed else 'CORRECTIVE_INJECTED_FAULT_FAILED',passed=passed,source_sha256=sha(SRC),control_input_sha256=sha(__file__),original_input_sha256=sha(BASE/'protocol/windows-cadence/controls.py'),original_failed_prefix=str(OUT/'missing-owned-rss.json'),separately_authorized_phase_start_epoch=PHASE_START,separately_authorized_phase_end_epoch=ABSOLUTE_END,started_epoch=started,ended_epoch=time.time(),injection=hits,raw_rows=len(raw),gzip_full_EOF=True,native_closure_proven=closed,token_reacquired_and_released=token,receipt=str(OUT/'missing-owned-rss-v2.json'),actual_censor=row['censor'],errors=row['errors'],owner=owner,qualification='CPython trace modifies a captured sampled local only; actual native RSS query succeeded before simulated unreadability. No source/time/module monkeypatch; no engine qualification.')
data=(json.dumps(r,indent=2)+'\n').encode();assert G.census(OUT)['bytes']+len(data)<=1048576 and G.census(OUT)['files']+1<=40
with (OUT/'missing-owned-rss-v2-result.json').open('xb') as f:f.write(data)
print(json.dumps(r),flush=True)

post=G.Budget(BUDGET).check({k:dict(bytes=65536,files=1) for k in ['receipts','windows_cadence_receipts']})
files={}
for root in [OUT,BASE/'protocol/windows-cadence']:
 for p in root.rglob('*'):
  if p.is_file():
   info=dict(bytes=p.stat().st_size,sha256=sha(p))
   if p.name.endswith('.gz'):
    with gzip.open(p,'rb') as f:
     plain=f.read();assert f.read()==b''
    info.update(gzip_full_EOF=True,uncompressed_bytes=len(plain))
   files[str(p)]=info
final=dict(status=r['status'],separate_phase_seconds=time.time()-PHASE_START,separate_phase_limit_seconds=90,within_phase_limit=time.time()<=ABSOLUTE_END,preflight=pre,postfinal_full_census=post,all_original_and_retry_files=files,manifest_self_excluded=True,own_receipts=G.census(OUT),own_protocol=G.census(BASE/'protocol/windows-cadence'),source=G.census(SRC.parent),source_sha256=sha(SRC),frozen_reference_sha256=sha(SRC.parent.parent/'harness/job_guard.py'),original_failure_preserved=True)
encoded=(json.dumps(final,indent=2)+'\n').encode();assert len(encoded)<=65536
assert G.census(OUT)['bytes']+len(encoded)<=1048576 and G.census(OUT)['files']+1<=40
with (OUT/'corrective-phase-review.json').open('xb') as f:f.write(encoded)
print(json.dumps(dict(final_status=final['status'],within_phase_limit=final['within_phase_limit'],phase_seconds=final['separate_phase_seconds'],manifest_sha256=sha(OUT/'corrective-phase-review.json'))),flush=True)
