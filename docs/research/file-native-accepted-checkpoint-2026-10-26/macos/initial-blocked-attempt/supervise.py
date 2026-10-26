import os,sys,json,time,subprocess,signal,shutil,hashlib
from pathlib import Path
OUT=Path(__file__).resolve().parent
SRC=Path('/private/tmp/file-native-online-macos-20261020/source/final-engine')
mode=sys.argv[1]
base=['/Users/spensermcconnell/.cargo/bin/cargo','test','--manifest-path',str(SRC/'Cargo.toml'),'--locked','--workspace','--release','-j2','--target-dir',str(OUT/'target')]
args=base+({'build':['--no-run'],'list':['--','--list'],'test':['--','--test-threads=1','--nocapture']}[mode])
env=os.environ.copy();updates={'EXP_PACK_TEST_ROOT':str(OUT/'fixtures'),'TMPDIR':str(OUT/'tmp'),'TMP':str(OUT/'tmp'),'TEMP':str(OUT/'tmp'),'CARGO_INCREMENTAL':'0','CARGO_NET_OFFLINE':'true'};env.update(updates)
limits={'timeout_seconds':1800 if mode=='test' else 1200,'aggregate_rss_bytes':3*1024**3,'test_process_rss_bytes':2*1024**3,'target_bytes':128*1024**2,'fixture_tmp_bytes':112*1024**2,'output_bytes':24*1024**2,'new_files':30000,'free_floor_bytes':20*1024**3}
meta={'mode':mode,'argv':args,'cwd':str(SRC),'environment_overrides':updates,'limits':limits,'start_epoch':time.time(),'resource_interpretation':'200ms native ps snapshots, 2s disk snapshots; functional supervision only, no hard peak or 40ms claim','budget_basis':'Reuse retained Mac128MiB build, native32MiB fixtures plus shared80MiB metadata and Mac24MiB evidence forecasts; no protected data or inside1GiB reserve consumed. Retained44GiB/5million combined envelope remains controlling, no fresh all-host census claimed.'}
(OUT/'logs'/f'{mode}-command.json').write_text(json.dumps(meta,indent=2))
start=time.monotonic();reason=None;peak=0;max_process=0;disk={};lastdisk=0;owned=set();samples=0
with (OUT/'logs'/f'{mode}.stdout').open('wb') as stdout,(OUT/'logs'/f'{mode}.stderr').open('wb') as stderr,(OUT/'logs'/f'{mode}-samples.jsonl').open('w') as sample:
 p=subprocess.Popen(args,cwd=SRC,env=env,stdout=stdout,stderr=stderr,start_new_session=True);pgid=p.pid
 while p.poll() is None:
  now=time.monotonic();raw=subprocess.run(['/bin/ps','-axo','pid=,ppid=,pgid=,rss='],capture_output=True,text=True)
  rows=[]
  for line in raw.stdout.splitlines():
   try:pid,ppid,group,rss=map(int,line.split());rows.append((pid,ppid,group,rss*1024))
   except ValueError:continue
  ids={p.pid};changed=True
  while changed:
   before=len(ids);ids.update(pid for pid,ppid,group,rss in rows if ppid in ids or group==pgid);changed=len(ids)!=before
  members=[r for r in rows if r[0] in ids];owned.update(ids);aggregate=sum(r[3] for r in members);single=max([r[3] for r in members] or [0]);peak=max(peak,aggregate);max_process=max(max_process,single);samples+=1
  sample.write(json.dumps({'elapsed':now-start,'aggregate_rss':aggregate,'processes':members})+'\n');sample.flush()
  if raw.returncode:reason='ps-sampling-failed'
  if aggregate>limits['aggregate_rss_bytes']:reason='aggregate-RSS-limit'
  if mode=='test' and single>limits['test_process_rss_bytes']:reason='test-process-RSS-limit'
  if now-start>limits['timeout_seconds']:reason='finite-timeout'
  if now-lastdisk>2:
   counts={};files=0
   for folder in ['target','fixtures','tmp','logs']:
    size=0
    for root,dirs,names in os.walk(OUT/folder):
     for name in names:
      try:size+=(Path(root)/name).stat().st_size;files+=1
      except FileNotFoundError:pass
    counts[folder]=size
   disk=counts;lastdisk=now
   if counts['target']>limits['target_bytes']:reason='target-disk-limit'
   if counts['fixtures']+counts['tmp']>limits['fixture_tmp_bytes']:reason='fixture-temp-disk-limit'
   if counts['logs']>limits['output_bytes']:reason='log-limit'
   if files>limits['new_files']:reason='new-file-limit'
   if shutil.disk_usage(OUT).free<limits['free_floor_bytes']:reason='free-space-floor'
  if reason:
   try:os.killpg(pgid,signal.SIGTERM)
   except ProcessLookupError:pass
   try:p.wait(timeout=3)
   except subprocess.TimeoutExpired:
    try:os.killpg(pgid,signal.SIGKILL)
    except ProcessLookupError:pass
   break
  time.sleep(.2)
 rc=p.wait()
# Identify surviving owned process group and stop only that original group if needed.
remaining=subprocess.run(['/bin/ps','-axo','pid=,ppid=,pgid=,rss='],capture_output=True,text=True).stdout
survivors=[line for line in remaining.splitlines() if len(line.split())==4 and int(line.split()[2])==pgid]
if survivors:
 try:os.killpg(pgid,signal.SIGKILL)
 except ProcessLookupError:pass
 reason=reason or 'descendants-survived-main-exit'
 time.sleep(.2)
remaining=subprocess.run(['/bin/ps','-axo','pid=,ppid=,pgid=,rss='],capture_output=True,text=True).stdout
survivors_after=[line for line in remaining.splitlines() if len(line.split())==4 and int(line.split()[2])==pgid]
result={**meta,'exit':rc,'duration_seconds':time.monotonic()-start,'stop_reason':reason,'sampled_aggregate_peak_bytes':peak,'sampled_max_process_bytes':max_process,'samples':samples,'last_disk_snapshot':disk,'owned_group':pgid,'survivors_at_main_exit':survivors,'survivors_after':survivors_after,'end_epoch':time.time()}
(OUT/'logs'/f'{mode}-result.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2));sys.exit(0 if rc==0 and not reason and not survivors_after else 1)
