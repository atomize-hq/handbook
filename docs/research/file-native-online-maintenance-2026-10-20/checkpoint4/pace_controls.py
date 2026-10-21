import argparse,hashlib,importlib.util,json,os,sys,threading,time
"""Future finite proof; runtime admission required."""
import concurrent.futures as cf
from pathlib import Path
P=argparse.ArgumentParser()
P.add_argument('name')
P.add_argument('--clock',required=True)
P.add_argument('--clock-sha256',required=True)
A=P.parse_args()
assert os.environ.get('FILE_NATIVE_STAGE_JOB')=='1'
assert len(A.name)<=99 and A.name.startswith('cp4-') and all(c.isascii() and (c.isalnum() or c=='-') for c in A.name)
BASE=Path('C:/file-native-online-cp1-20261020')
T=BASE/f'fixtures/cp3/{A.name}/disposable/store'
assert not T.parent.parent.exists(),'fresh-prefix'
cp=Path(A.clock).resolve()
assert hashlib.sha256(cp.read_bytes()).hexdigest()==A.clock_sha256
spec=importlib.util.spec_from_file_location('approved_pace_clock',cp)
C=importlib.util.module_from_spec(spec)
spec.loader.exec_module(C)
sys.path.insert(0,'C:/Users/spmcc/.codex/worktrees/file-native-online-matrix-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/harness')
from driver import Driver
LOG=BASE/f'evidence/{A.name}-events.jsonl'
OUT=BASE/f'evidence/{A.name}-pace.json'
assert not LOG.exists() and not OUT.exists()
D=C.bind_driver(Driver,BASE/'build/release/file-native-incremental.exe',LOG)
assert D.proof_contract=='native-owned-v1'
F=C.frequency()
assert isinstance(F,int) and F>0
R=dict(status='RUNNING',clock_sha256=A.clock_sha256,binary=D.binary_sha256,
  frequency=F,bridge_before=C.bridge(),readers=[],publishers=[],cleanup=[],errors=[])
LOCK=threading.RLock()
def save():
 with LOCK:
  OUT.write_bytes((json.dumps(R,separators=(',',':'))+'\n').encode())
def check(test,message):
 assert test,message
def proof(pid):
 v=D.process_proof(pid)
 check(v['pid']==pid and v['alive'] and v['creation_time'] and v['owned_token'] and
   Path(v['image_path']).resolve()==D.executable.resolve() and
   v['image_sha256']==D.binary_sha256,'owner')
 return {k:v[k] for k in ['pid','alive','creation_time','image_path','image_sha256','owned_token']}
def digest(v):
 return hashlib.sha256(json.dumps(v,sort_keys=True,separators=(',',':')).encode()).hexdigest()
def request(name):
 return dict(request=name,base=D.head(T),ops=[],packet=False)
D.call(T,'init',mode='A',consumer='project',count=10)
D.call(T,'lifecycle',operation='setup')
for name in ['early','middle','recent']:
 D.publish(T,name)
D.call(T,'cancel',request_id='cancelled')
c=D.call(T,'prepare',request=request('lost'))['result']['candidate']
D.call(T,'select',candidate=c,code=86,env={'EXP_EXIT':'selector-visible'})
D.call(T,'packing',operation='build',bootstrap=True,force=True,codec='raw',cleanup=False)
IDS=['early','middle','recent','missing','cancelled','lost']
Q=[dict(id=x,category=x) for x in IDS]
E={}
for x in IDS:
 for i in range(50):
  v=D.call(T,'resolve',request_id=x)['result']
  if i==0:
   E[x]=v
  check(v==E[x],'baseline')
baseline=D.call(T,'lookup-bench',count=500,requests=Q)['result']
check('concurrent_pacing' not in baseline and len(baseline['rows'])==6,'unpaced')
for r in baseline['rows']:
 check(len(r['warm_ns'])==500 and r['result_digest']==digest(E[r['id']]),'samples')
phase=A.name
nr='\\\\?\\'+str(T.resolve())
control=Path(nr).parent/'lookup-pace'/phase
control.mkdir(parents=True)
READY=[control / f'reader-{i}.ready.json' for i in range(4)]
START={}
CURRENT={}
B={}
AH=None
O=None
EXIT={}
def reader(i):
 try:
  for b in range(10):
   row=Q[(i*10+b) % 6]
   p=dict(family="cp4-control",version=1,phase_id=phase,reader=i,batch=b,control_dir=str(control))
   if b:
    p.update(anchor_sha256=AH,ready_sha256=B[i]['sha256'])
   local={}
   def started(event):
    local.update(proof=proof(event['pid']),event=event)
    with LOCK:
     CURRENT[i]=local['proof']
     if b==0:
      START[i]=local['proof']
   out=D.call(T,'lookup-bench',count=1000,requests=[row],
      concurrent_pacing=p,env={'FILE_NATIVE_PACED_ROOT':nr},
      on_started=started,timeout=55)['result']
   joined=C.counter()
   h=out['concurrent_pacing']
   r=out['rows'][0]
   check(h['pid']==local['proof']['pid'] and h['reader']==i and h['batch']==b and
     h['clock_kind']=='windows-qpc' and h['frequency']==F and h['origin']==O and h['anchor_sha256']==AH and
     h['ready_sha256']==B[i]['sha256'],'binding')
   check(len(json.dumps(h,separators=(',',':')).encode())<=2048 and
     h['completed']==len(r['warm_ns'])==1000 and
     r['id']==row['id'] and r['result_digest']==digest(E[row['id']]),'row')
   check(h['cache_current_bytes']<=h['cache_peak_bytes'] and h['cache_after_drop_bytes']==0,'cache-fold')
   check(h['deadline_ok'] and h['max_lag_ticks']<=F//10 and h['successors']<=60,'deadline')
   marks=h['milestones']
   check(len(marks)==10 and [m[0] for m in marks]==[b*1000+j for j in range(0,1000,111)],'milestones')
   for ordinal,start,end in marks:
    due=O+((16000000+ordinal*2900 if i==0 else ordinal*4500)*F)//1000000
    check(due<=start<=due+F//10 and start<=end,'ordinal')
   check(h['last_read']<=h['io_done']<=h['pins_dropped']<=h['reply_ready']<=joined,'pins')
   with LOCK:
    R['readers'].append(dict(reader=i,batch=b,proof=local,header=h,stream_proof_sha256=B[i]['proof_sha256'],process_join_qpc=joined))
    EXIT[(i,b)]=joined
    save()
 except BaseException as e:
  with LOCK:
   R['errors'].append(repr(e))
   save()
  raise

def pace_work_state():
    cfg=(T/'STORE.json').read_bytes()
    check(len(cfg)<=4096,'config-size')
    c=json.loads(cfg)
    plan=c['physical']['plan']
    check(len(plan)==64 and all(x in '0123456789abcdef' for x in plan),'plan-id')
    with (T/f'physical-plans/{plan}.json').open('rb') as f:
        raw=f.read(131073)
    check(len(raw)<=131072 and hashlib.sha256(raw).hexdigest()==plan,'selected-work')
    check((T/'STORE.json').read_bytes()==cfg,'work-stable-binding')
    w=json.loads(raw)
    members=len(w['generation']['packs'])+sum(len(g['packs']) for g in w['older'])+len(w['jobs'])+len(w['retired'])
    return dict(plan=plan,bytes=len(raw),ordinal=c['physical']['ordinal'],
                generation=w['generation']['id'],older=w['older'],members=members)
PRUNED={}
def pace_cleanup(slot):
    for cleanup_pass in range(4):
        before=pace_work_state()
        begin=C.counter()
        D.call(T,'packing',operation='cleanup')
        after=pace_work_state()
        end=C.counter()
        ids={g['id'] for g in after['older']}
        for g in before['older']:
            if g['id'] not in ids:
                PRUNED[g['id']]=g
        old=D.process_proof(START[0]['pid'])
        keys=('pid','creation_time','image_path','image_sha256','owned_token')
        held_bound=old['alive'] and all(old[k]==START[0][k] for k in keys)
        with LOCK:
            observed=dict(CURRENT)
        ordinary={}
        for i in (1,2,3):
            if i in observed:
                p=D.process_proof(observed[i]['pid'])
                if p['alive'] and all(p[k]==observed[i][k] for k in keys):
                    ordinary[i]=p
        initial=B[0]['marker']['physical']['id']
        absent=[g['id'] for g in PRUNED.values() if g['id']!=initial and g['plan'] and
                not (T/f"physical-plans/{g['plan']}.json").exists()]
        event=dict(slot=slot,cleanup_pass=cleanup_pass,begin=begin,end=end,
                   work_before_bytes=before['bytes'],work_after_bytes=after['bytes'],
                   descriptor_members=after['members'],retained=[g['id'] for g in after['older']],
                   pruned=list(PRUNED),middle_plan_absent=absent,
                   held_owner=old,held_bound=held_bound,ordinary_owners=ordinary)
        with LOCK:
            R['cleanup'].append(event)
            save()
    check(pace_work_state()['bytes']<=98304,'next-build-work-headroom')


def middle_proof(event,held):
    if not (event['middle_plan_absent'] and len(event['ordinary_owners'])==3 and
            event['held_bound'] and event['end']<held['header']['pins_dropped']):
        return False
    for i,p in event['ordinary_owners'].items():
        rows=[r for r in R['readers'] if r['reader']==i and
              r['proof']['proof']['owned_token']==p['owned_token']]
        if len(rows)!=1:
            return False
        h=rows[0]['header']
        if not (h['first_read']<=event['begin']<=event['end']<=h['last_read'] and
                any(m[2]<=event['begin'] for m in h['milestones']) and
                any(m[1]>=event['end'] for m in h['milestones'])):
            return False
    return True

def publishers():
 for i in range(100):
  due=O+i*400000*F//1000000
  delay=(due-C.counter())/F
  if delay>0:
   time.sleep(delay)
  start=C.counter()
  D.publish(T,f'paced-pub-{i}')
  end=C.counter()
  with LOCK:
   R['publishers'].append(dict(slot=i,due=due,start=start,end=end))
  if i % 10==9:
   pre=pace_work_state()
   check(pre['bytes']<=98304,'build-work-headroom')
   D.call(T,'packing',operation='build',force=True,codec='raw',cleanup=False)
   check(pace_work_state()['ordinal']==pre['ordinal']+1,'actual-activation')
   pace_cleanup(i)
  save()
save()
with cf.ThreadPoolExecutor(max_workers=5) as pool:
 futures=[pool.submit(reader,i) for i in range(4)]
 deadline=time.monotonic()+5
 while len(B) < 4:
  check(time.monotonic() < deadline and not R['errors'],'readies')
  for i,path in enumerate(READY):
   if i in B or i not in START or not path.exists():
    continue
   try:
    with path.open('rb') as f:
     raw=f.read(4097)
    check(len(raw)<=4096,'ready-size')
    r=json.loads(raw)
   except json.JSONDecodeError:
    continue
   check(len(raw)<=4096 and r['pid']==START[i]['pid'] and
     r['reader']==i and r['batch']==0 and r['phase_id']==phase and
     r['clock_kind']=='windows-qpc' and r['frequency']==F and r['request']==Q[(i*10)%6]['id'] and
     r['category']==Q[(i*10)%6]['category'],'ready')
   check(proof(r['pid'])==START[i] and r['capture_qpc']<=r['first_done_qpc']<=r['ready_qpc'],'owner')
   cfg=json.loads((T/'STORE.json').read_bytes())
   check(r['physical']==cfg['physical'] and r['catalog']==cfg['catalog'] and
     r['selector']==D.head(T),'capture')
   B[i]=dict(sha256=hashlib.sha256(raw).hexdigest(),marker=r,proof=START[i],proof_sha256=digest(START[i]))
  time.sleep(.001)
 for i in range(4):
  check(proof(START[i]['pid'])==START[i],'live')
 O=C.counter()+F
 anchor=dict(clock_kind="windows-qpc",version=1,phase_id=phase,frequency=F,origin=O)
 raw=json.dumps(anchor,separators=(',',':')).encode()
 stage=control/'anchor-stage'
 with stage.open('xb') as f:
  f.write(raw)
  f.flush()
  os.fsync(f.fileno())
 assert not (control/'anchor.json').exists()
 stage.rename(control/'anchor.json')
 AH=hashlib.sha256(raw).hexdigest()
 R.update(anchor=anchor,anchor_sha256=AH,ready=B)
 pub=pool.submit(publishers)
 for future in futures+[pub]:
  future.result()
R['bridge_after']=C.bridge()
check(len(R['readers'])==40 and len(R['publishers'])==100 and not R['errors'],'counts')
check(max(r['end'] for r in R['publishers'])<=O+45*F,'pub-time')
check(O+44997100*F//1000000<=max(EXIT.values())<=O+45100000*F//1000000,'span')
held=next(r for r in R['readers'] if r['reader']==r['batch']==0)
check(len(R['cleanup'])==40,'finite-cleanup-count')
check(any(middle_proof(e,held) for e in R['cleanup']),'actual-middle-cleanup-during-old-pin')
check(held['header']['pins_dropped'] >= O+18897100*F//1000000,'held')
R.update(status='PASS_BOUNDED_PACING_ONLY',warm_queries=40000,separately_timed_first=40,
  baseline_fresh=300,baseline_warm=3000,no_extra_paced_queries=True,physical_activations_max=10)
save()
