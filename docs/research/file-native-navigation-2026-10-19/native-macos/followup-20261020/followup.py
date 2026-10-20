"""Separate bounded follow-up. Original censored artifacts remain read-only."""
import os,sys,json,time,signal,subprocess,threading,hashlib,datetime,shutil,tarfile,traceback
from pathlib import Path
from types import SimpleNamespace
O=Path(__file__).resolve().parent;N=O.parent;H=N.parent;W=N.parents[3];F=H.parent/'file-native-recovery-reclamation-2026-10-19';R=N/'runtime/followup-20261020';V=O/'evidence';EXE=N/'runtime/build/debug/file-native-incremental';G=Path('/Users/spensermcconnell/__Active_Code/atomize-hq/handbook/.git')
START=datetime.datetime.fromisoformat('2026-10-20T02:09:52+00:00').timestamp();CUTOFF=START+600;DEADLINE=START+1200
SHELL=r'''set -eu
store="$1"
out="$2"
mode="$3"
cp "$store/SELECTED.json" "$out/selected-once.json"
publication=$(sed -n 's/.*"publication": *"\([^"]*\)".*/\1/p' "$out/selected-once.json")
if [ "$mode" = B ]; then
 attempt=$(sed -n 's/.*"attempt": *"\([^"]*\)".*/\1/p' "$store/publications/$publication.json")
 tree="$store/generations/$attempt"
else
 tree="$store/view"
fi
printf '%s\n' "$tree" > "$out/captured-tree.txt"
cd "$tree"
find navigation -type f -name '*.yaml' | sort > "$out/listing.txt"
find navigation/groups -type f -name '*.yaml' | sort > "$out/groups.txt"
first=$(sed -n '1p' "$out/groups.txt")
grep -n 'source:' "$first" > "$out/search.txt"
source=$(sed -n '/^    source: /{s/^    source: "//;s/"$//;p;}' "$first" | sed -n '1p')
printf '%s\n' "$source" > "$out/chosen-source.txt"
cp "$source" "$out/copied-source"
shasum -a 256 "$out/copied-source" > "$out/copy-sha256.txt"
'''
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def save(name,value):(V/name).write_text(json.dumps(value,indent=2),encoding='utf-8')
def census(root):
 n=b=0
 for d,ds,fs in os.walk(root,followlinks=False):
  for f in fs:
   s=os.lstat(os.path.join(d,f));n+=1;b+=s.st_size
 return dict(root=str(root),files=n,logical_bytes=b)
def bindings():
 pins=json.loads((N/'evidence/source-pins.json').read_bytes());core={k:sha(W/k) for k in pins['files'] if '/src/' in k or k.endswith(('Cargo.toml','Cargo.lock'))};assert len(core)==16 and all(pins['files'][k]==v for k,v in core.items());assert sha(EXE)=='1de6432fc24922f10a6e4a1eddb95e07f1e236cf4435723e4fd2320e5dbdc786'
 return dict(core=core,exe_sha256=sha(EXE),source='4bde156323812ef18c0dea6798cfc2108aaad933',harness={str(p.relative_to(W)):sha(p) for p in [H/'oracle.py',H/'focused.py',H/'census.py',H/'legacy-fixtures.tar.gz',F/'capsule.py',F/'operational_backup.py',Path(__file__)]})
def snapshot():
 rows=[census(p) for p in [W,G,Path('/private/tmp/file-native-q-macos-20261019'),Path('/Users/spensermcconnell/.codex/worktrees/file-native-q-macos-20261019/handbook')]]
 return dict(timestamp=now(),roots=rows,owned_bytes=rows[0]['logical_bytes']+max(0,rows[1]['logical_bytes']-185803386),owned_files=rows[0]['files']+max(0,rows[1]['files']-5264),free_bytes=shutil.disk_usage(W).free)
def processes():
 p=subprocess.run(['ps','-axo','pid=,ppid=,pgid=,rss='],capture_output=True,text=True,timeout=2)
 return [tuple(map(int,x.split())) for x in p.stdout.splitlines() if len(x.split())==4]
def descendants(pid,table):
 ids={pid}
 for _ in range(20):ids|={a for a,b,g,r in table if b in ids}
 return ids

def worker():
 sys.path.insert(0,str(H));import oracle
 sys.path.insert(0,str(F));import capsule
 from focused import Driver
 from oracle import verify,Tree,parse_group
 operations=[]
 def bounded(args,input=None,capture_output=False,env=None,timeout=None,**kw):
  assert time.time()<CUTOFF,'followup-functional-cutoff'
  action=json.loads(input)['action'] if input else 'shell';limit=256*1024**2 if action in ['point','inspect','resolve'] else 2*1024**3
  if env is not None:env=dict(env,EXP_NAVIGATION='4')
  t=time.time();p=subprocess.Popen(args,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,env=env,**kw);peak=None;stop=threading.Event();violations=[]
  def sample():
   nonlocal peak
   while not stop.is_set():
    table=processes();ids=descendants(p.pid,table);values=[r*1024 for a,b,g,r in table if a in ids]
    if values:peak=max(peak or 0,sum(values))
    if values and sum(values)>limit:violations.append('rss-limit');p.kill();return
    stop.wait(.04)
  th=threading.Thread(target=sample,daemon=True);th.start()
  try:stdout,stderr=p.communicate(input,timeout=max(.01,min(120,CUTOFF-t)))
  except subprocess.TimeoutExpired:p.kill();stdout,stderr=p.communicate();violations.append('ordinary-child-or-cutoff-censor')
  finally:stop.set();th.join(timeout=3)
  row=dict(args=[str(x) for x in args],action=action,start=t,seconds=time.time()-t,exit=p.returncode,sampled_peak_rss_bytes=peak,limit=limit,violations=violations,stdout_sha256=hashlib.sha256(stdout).hexdigest(),stderr_sha256=hashlib.sha256(stderr).hexdigest());operations.append(row);save('child-resources.json',operations);assert not violations,row
  return subprocess.CompletedProcess(args,p.returncode,stdout,stderr)
 capsule.subprocess=SimpleNamespace(run=bounded)
 result=capsule.run(EXE,R/'recovery-capsule');assert len(result['cases'])==24 and result['events']==282,result
 save('recovery-result.json',result)
 d=Driver(EXE,R/'supplement');rows=[]
 def record(row):rows.append(row);save('supplement-cases.json',rows)
 with tarfile.open(H/'legacy-fixtures.tar.gz') as archive:archive.extractall(d.base/'legacy-input',filter='data')
 for mode in ['A','B']:
  for profile in [1,2,3,4]:
   assert time.time()<CUTOFF
   if profile==1:
    root=d.base/('readme-'+mode+'-1');shutil.copytree(d.base/'legacy-input/ordinary'/mode,root);initial=(root/'README.md').read_bytes();d.call(root,'select',candidate=json.loads((root/'FIXTURE-CANDIDATE.json').read_bytes()))
   else:root=d.init('readme-'+mode+'-'+str(profile),mode,16,profile);initial=(root/'README.md').read_bytes() if profile<=3 else None
   if profile<=3:assert initial and (root/'README.md').read_bytes()==initial
   else:assert not (root/'README.md').exists()
   d.publish(root,'upgrade',[],profile=4);d.call(root,'checkpoint',request_id='readme-checkpoint')
   assert (root/'README.md').read_bytes()==initial if initial is not None else not (root/'README.md').exists()
   proof=verify(root);record(dict(case='historical-readme-upgrade-checkpoint' if profile<4 else 'new-profile4-genesis-readme-absent',mode=mode,profile=profile,readme_sha256=hashlib.sha256(initial).hexdigest() if initial else None,proof=proof))
  for n in [128,1024]:
   assert time.time()<CUTOFF
   root=d.init('growing-'+mode+'-'+str(n),mode,n,4);selected=d.head(root);pub=json.loads((root/f"publications/{selected['publication']}.json").read_bytes());projection=Tree(root).entries(pub['projection']);groups={p:parse_group((root/f['source']).read_bytes()) for p,f in projection.items() if p.startswith('navigation/groups/')};target,group=next((p,g) for p,g in groups.items() if 'd000006' in g);members=len(group)
   assert len(groups)>1 if n==1024 else len(groups)==1
   state=d.call(root,'dump')['result']['state'];path=state['docs']['d000006']['path'];title=state['docs']['d000006']['title']+' growing longer title for local update'
   result=d.publish(root,'grow',[dict(kind='rename',id='d000006',text=title),dict(kind='path',id='d000006',text=path)])
   metrics=result['prepare']['metrics'];assert metrics['range_rows'].get('D',0)==0;assert metrics['navigation_parts']==2;assert metrics['range_calls'].get('E',0)<=members+3,(metrics,members)
   newpub=json.loads((root/f"publications/{d.head(root)['publication']}.json").read_bytes());after=Tree(root).entries(newpub['projection']);assert all(after[p]==f for p,f in projection.items() if p.startswith('navigation/groups/') and p!=target)
   proof=verify(root);record(dict(case='growing-title-local-leaf-no-sibling-regeneration',mode=mode,n=n,groups=len(groups),affected_members=members,target_group=target,metrics=metrics,proof=proof,scope='No full D scan; E range calls bounded by one leaf; all sibling FileSpecs unchanged. No syscall-level open trace claimed.'))
  root=d.base/('growing-'+mode+'-128')/'disposable/store';destination=d.base/('shell-'+mode);destination.mkdir();before=len(d.events)
  command=['bash','-c',SHELL,'native-followup',str(root),str(destination),mode];t=time.time();p=bounded(command);assert p.returncode==0,(p.stdout,p.stderr);assert len(d.events)==before
  tree=Path((destination/'captured-tree.txt').read_text().strip());selected=json.loads((destination/'selected-once.json').read_bytes());pub=json.loads((root/f"publications/{selected['publication']}.json").read_bytes());projection=Tree(root).entries(pub['projection']);source=(destination/'chosen-source.txt').read_text().strip();spec=projection[source];copied=(destination/'copied-source').read_bytes();assert copied==(root/spec['source']).read_bytes() and hashlib.sha256(copied).hexdigest()==spec['digest'];assert tree==root/('view' if mode=='A' else 'generations/'+pub['attempt'])
  record(dict(case='ordinary-bash-discovery-search-exact-copy',mode=mode,selected_once=selected,captured_tree=str(tree),path=source,sha256=spec['digest'],seconds=time.time()-t,engine_calls_during_extraction=0,script_sha256=hashlib.sha256(SHELL.encode()).hexdigest(),stdout=p.stdout.decode(),stderr=p.stderr.decode()))
  (V/('bash-'+mode+'.sh')).write_text(SHELL)
 save('worker-summary.json',dict(status='passed',time=now(),recovery_cases=24,recovery_events=282,supplement_cases=len(rows)))

if __name__=='__main__':
 if len(sys.argv)>1 and sys.argv[1]=='worker':
  try:worker()
  except BaseException:(V/'worker-failure.txt').write_text(traceback.format_exc());raise
 else:
  V.mkdir(exist_ok=False);R.mkdir(exist_ok=False);baseline=snapshot();assert baseline['owned_bytes']+268435456<805306368 and baseline['owned_files']+25000<75000 and baseline['free_bytes']>20*1024**3
  original={str(p.relative_to(N)):sha(p) for p in N.rglob('*') if p.is_file() and 'runtime' not in p.relative_to(N).parts and 'followup-20261020' not in p.relative_to(N).parts and '__pycache__' not in p.relative_to(N).parts}
  save('baseline.json',dict(start='2026-10-20T02:09:52Z',functional_cutoff='2026-10-20T02:19:52Z',closeout_deadline='2026-10-20T02:29:52Z',actual_supervisor_start=now(),resources=baseline,bindings=bindings(),original_artifact_hashes=original,setup_failure='First creation command had Python nested-string syntax error; no files or functional tests created. Retained separately in setup-attempt.txt.'))
  if time.time()>=CUTOFF:save('supervisor.json',dict(status='not-started-cutoff',time=now()));raise SystemExit(2)
  start=time.time();samples=[];reason=None;peak=None;lastdisk=0;disk=[]
  with (V/'worker.stdout').open('wb') as so,(V/'worker.stderr').open('wb') as se:
   p=subprocess.Popen([sys.executable,str(Path(__file__).resolve()),'worker'],stdout=so,stderr=se,start_new_session=True,env=dict(os.environ,TMPDIR=str(R)))
   while p.poll() is None:
    table=processes();ids=descendants(p.pid,table);values=[r*1024 for a,b,g,r in table if a in ids];checker=next((r*1024 for a,b,g,r in table if a==p.pid),None);aggregate=sum(values) if values else None
    if aggregate is not None:peak=max(peak or 0,aggregate)
    if time.time()>=CUTOFF:reason='followup-functional-cutoff'
    if time.time()-start>=900:reason='verifier-cap'
    if aggregate is not None and aggregate>3*1024**3:reason='aggregate-rss-limit'
    if checker is not None and checker>2*1024**3:reason='checker-rss-limit'
    if time.time()-lastdisk>2:
     c=census(R);disk.append(dict(timestamp=now(),**c));lastdisk=time.time();save('runtime-census-samples.json',disk)
     if c['logical_bytes']>268435456 or c['files']>25000:reason='followup-owned-allocation-limit'
     if shutil.disk_usage(W).free<20*1024**3:reason='free-reserve'
    samples.append(dict(time=time.time(),aggregate_rss_bytes=aggregate,checker_rss_bytes=checker,pids=sorted(ids)))
    if reason:os.killpg(p.pid,signal.SIGKILL);break
    time.sleep(.12)
   code=p.wait();remaining=[a for a,b,g,r in processes() if g==p.pid]
   if remaining:
    try:os.killpg(p.pid,signal.SIGKILL)
    except ProcessLookupError:pass
    time.sleep(.1)
   after=[a for a,b,g,r in processes() if g==p.pid]
  save('rss-samples.json',samples);save('supervisor.json',dict(finished=now(),worker_start_epoch=start,seconds=time.time()-start,exit=code,censor=reason,aggregate_sampled_peak_rss_bytes=peak,remaining_process_group_before_cleanup=remaining,remaining_process_group_after_cleanup=after,status='passed' if code==0 and not reason and not after else 'failed-or-censored'))
  save('post-bindings.json',bindings());save('post-census.json',snapshot());assert all(sha(N/name)==digest for name,digest in original.items());save('original-preservation.json',dict(files=len(original),all_original_bytes_unchanged=True))
  print((V/'supervisor.json').read_text(),flush=True)
