"""Native lifecycle/clock/barrier adapter; source only until final release."""
import base64,gzip,json,math,threading,time,zlib
from pathlib import Path
def need(ok,message):
 if not ok:raise RuntimeError(message)
def finite(value):
 need(type(value) in (int,float) and math.isfinite(value) and value>0,'finite-positive-wait');return float(value)
def raw_ns():
 # Darwin clock 6, not CLOCK_MONOTONIC_RAW(4) or Python's implicit clock.
 need(time.CLOCK_MONOTONIC==6,'Darwin-clock-ABI')
 value=time.clock_gettime_ns(time.CLOCK_MONOTONIC)
 need(type(value) is int and value>=0,'raw-clock-unavailable');return value
class Lifecycle:
 def __init__(self,facade):
  self.f=facade;self.lock=threading.RLock();self.local=threading.local();self.handles=[];self.controls=[];self.phases={}
 def attach(self,call):
  h=getattr(self.local,'handle',None)
  with self.lock:
   if h is not None:h['calls'].append(call);h['case_id']=call.case['case_id'];h['root']=str(call.root)
 def spawn(self,callback):
  context=self.f.sole_case()
  h={'response':None,'error':None,'calls':[],'case_id':context['case_id'],'root':context['root']}
  def run():
   self.local.handle=h
   try:h['response']=callback()
   except BaseException as exc:h['error']=exc
   finally:self.local.handle=None
  h['thread']=threading.Thread(target=run,daemon=False)
  with self.lock:self.handles.append(h)
  self.f.lease.auth();h['thread'].start();return h
 def alive(self,handle):return handle['thread'].is_alive()
 def wait(self,handle,timeout):
  timeout=self.f.clip(finite(timeout));handle['thread'].join(timeout)
  need(not self.alive(handle),'native-callback-join-timeout')
  if handle['error'] is not None:raise handle['error']
  return handle['response']
 def active_proof(self,call):
  need(call.owner.started_proof_ref and not call.owner.closed,'native-call-not-live')
  p=Path(call.owner.started_proof_ref);raw=self.f.read_evidence(p,call.forecast['RSS_cap'])
  d=zlib.decompressobj(31);decoded=d.decompress(raw,8*1024**2)
  need(not d.unconsumed_tail,'active-proof-decode-cap')
  rows=[]
  for line in decoded.splitlines():
   try:rows.append(self.f.strict(line))
   except ValueError:continue
  proofs=[v for v in rows if v.get('kind') in ('role-native-start-proof','role-before-input-recheck')]
  need(bool(proofs),'positive-native-proof-required');v=proofs[-1]
  expected=v['identity_after'];binary=Path(v['path']['resolved_executable'])
  need(binary==Path(call.argv[0]).resolve(),'native-proof-image-binding')
  return self.f.runtime.roles.prove_actual_exec(expected,binary,v['parent'],min(call.owner._deadline,self.f.lease.deadline))
 def control(self,root,name):
  ctx=self.f.context(root);need(isinstance(name,str) and name and len(name)<=96 and all(c.isalnum() or c in '-_' for c in name),'control-name')
  base=self.f.lease.path(Path(ctx['control_root'])/name,'F08')
  need(not base.exists(),'fresh-control-prefix')
  for q in (base,base.parent):self.f.lease.directory(q)
  base.mkdir(parents=True,exist_ok=False)
  v={'root':str(root),'case_id':ctx['case_id'],'ready':str(base/'ready'),'release':str(base/'release'),'cancel':str(base/'cancel'),'ended':str(base/'cancel-ended'),'phase':None}
  for key in ('ready','release','cancel','ended'):self.f.lease.file(Path(v[key]),ctx['control_file_cap'])
  with self.lock:self.controls.append(v)
  return v
 def await_ready(self,control,handle,timeout):
  end=time.monotonic()+self.f.clip(finite(timeout))
  while time.monotonic()<end:
   self.f.lease.auth();need(self.alive(handle),'native-ended-before-checkpoint')
   calls=list(handle['calls'])
   if calls:
    call=calls[-1];phase=(call.env or {}).get('EXP_BARRIER')
    need(phase in self.f.contract['checkpoints'],'source-bound-ready-phase')
    mapping=self.f.contract['checkpoints'][phase]
    need(mapping['source_pin']==self.f.final_pin and mapping['capability']=='barrier:'+phase,'ready-source-map')
    if Path(control['ready']).exists():
     proof=self.active_proof(call)
     actual=self.f.read_fixture(Path(control['ready']),self.f.context(control['root'])['control_file_cap'])
     expected=base64.b64decode(mapping['ready_b64'],validate=True)
     need(actual==expected,'authentic-engine-ready-bytes')
     control['phase']=phase;control['call']=call;control['proof']=proof;control['ready_raw_ns']=raw_ns()
     self.f.emit({'kind':'native-ready','case_id':control['case_id'],'phase':phase,'proof':proof,'ready_sha256':self.f.sha(actual),'raw_ns':control['ready_raw_ns']})
     return control
   time.sleep(min(.002,max(0,end-time.monotonic())))
  raise TimeoutError('authentic-native-ready-timeout')
 def release(self,control):
  need(control.get('phase') in self.f.contract['checkpoints'],'release-without-authentic-ready')
  self.active_proof(control['call'])
  data=base64.b64decode(self.f.contract['checkpoints'][control['phase']]['release_b64'],validate=True)
  self.f.write_fixture(Path(control['release']),data,fresh=True)
  self.f.emit({'kind':'native-release','case_id':control['case_id'],'phase':control['phase'],'raw_ns':raw_ns()})
 def signal(self,control,kind):
  need(kind=='cancel' and control.get('phase') in self.f.contract['checkpoints'],'source-bound-cancel')
  self.active_proof(control['call']);m=self.f.contract['checkpoints'][control['phase']]
  self.f.write_fixture(Path(control['cancel']),base64.b64decode(m['cancel_b64'],validate=True),fresh=True)
 def end_signal(self,control,kind):
  need(kind=='cancel' and Path(control['cancel']).is_file(),'cancel-signal-not-retained')
  # Retain the asserted signal and its hash. Source map defines whether the
  # engine consumes an explicit ending marker; no unlink/refund is permitted.
  m=self.f.contract['checkpoints'][control['phase']]
  need('cancel_end_b64' in m,'reviewed-cancel-ending-protocol')
  self.f.write_fixture(Path(control['ended']),base64.b64decode(m['cancel_end_b64'],validate=True),fresh=True)
 def interrupt(self,handle,kind):
  calls=list(handle['calls']);need(len(calls)==1 and kind in ('interrupt','terminate','kill'),'exact-owned-interrupt')
  call=calls[0];self.active_proof(call);ticket=call.owner.interrupt(kind)
  handle['interrupt']={'kind':kind,'ticket':ticket,'call':call};return ticket
 def finish_interrupted(self,handle,kind):
  v=handle.get('interrupt');need(v is not None and v['kind']==kind,'owned-interrupt-ack-required')
  ack=v['ticket'].wait(self.f.clip(2));response=None;error=None
  try:response=self.wait(handle,self.f.clip(5))
  except BaseException as exc:error=exc
  need(v['call'].owner.closed and not self.alive(handle),'interrupted-native-and-Python-not-closed')
  self.f.emit({'kind':'native-interrupt-closed','case_id':v['call'].case['case_id'],'ack':ack,'response':response,'error':repr(error) if error else None})
  if error is not None and not isinstance(error,AssertionError):raise error
  return {'ack':ack,'closed':True,'raw':v['call'].row,'response':response}
 def begin_phase(self,root,label):
  ctx=self.f.context(root);key=(str(root),label);need(key not in self.phases,'phase-prefix-reuse')
  self.phases[key]={'case_id':ctx['case_id'],'raw_start_ns':raw_ns(),'first_call':len(self.f.calls)}
 def overlap(self,root,label,publishers,readers):
  phase=self.phases[(str(root),label)];end=raw_ns();intervals=[]
  for call in self.f.calls[phase['first_call']:]:
   if str(call.root)!=str(root):continue
   need(call.done and call.owner.closed,'overlap-needs-complete-native-calls')
   for v in self.f.decoder.work_intervals(call.row,self.f.contract['work_source_map']):
    need(set(v)=={'identity','start_ns','end_ns','kind','clock_id','frequency','source_pin','positive_work'},'work-interval-closed-shape')
    need(v['source_pin']==self.f.final_pin and v['clock_id']==6 and v['frequency']==1000000000 and v['positive_work'] is True,'actual-positive-work-clock-binding')
    need(type(v['start_ns']) is int and type(v['end_ns']) is int and call.raw_start_ns<=v['start_ns']<v['end_ns']<=call.raw_end_ns,'native-work-interval-not-startup')
    need(self.f.runtime.monitor.same_identity(v['identity'],call.native_identity),'work-native-birth-binding')
    if v['end_ns']>phase['raw_start_ns'] and v['start_ns']<end:intervals.append(v)
  maint=[v for v in intervals if v['kind']=='maintenance'];pubs=[v for v in intervals if v['kind']=='publisher'];reads=[v for v in intervals if v['kind']=='reader']
  def key(v):return (v['identity']['pid'],tuple(v['identity']['start']))
  def overlaps(x,y):return min(x['end_ns'],y['end_ns'])>max(x['start_ns'],y['start_ns'])
  active_reads={key(v) for v in reads if any(overlaps(v,m) and any(overlaps(p,m) and overlaps(p,v) for p in pubs) for m in maint)}
  active_pubs={key(v) for v in pubs if any(overlaps(v,m) for m in maint)}
  need(len(active_reads)>=readers and len(active_pubs)>=publishers and bool(maint),'positive-native-work-overlap-incomplete')
  proof={'independent_reader_processes':len(active_reads),'publisher_processes':len(active_pubs),'maintenance_overlap':True,'intervals':intervals,'raw_start_ns':phase['raw_start_ns'],'raw_end_ns':end}
  self.f.emit({'kind':'positive-native-work-overlap','case_id':phase['case_id'],'proof':proof});return proof
 def close_case(self,case_id,root):
  errors=[]
  for control in list(self.controls):
   if control['case_id']==case_id and control.get('phase') and not Path(control['release']).exists():
    try:self.release(control)
    except BaseException as exc:errors.append({'operation':'release','error':repr(exc)})
  for call in self.f.calls:
   if call.case['case_id']!=case_id or call.owner.closed:continue
   call.owner._abort.set()
   try:call.owner.wait(self.f.clip(2))
   except BaseException as exc:errors.append({'operation':'native-close','error':repr(exc)})
  handles=[h for h in self.handles if h['case_id']==case_id]
  for h in handles:
   h['thread'].join(self.f.clip(2))
   if h['thread'].is_alive():errors.append({'operation':'Python-join','error':'still-live'})
  calls=[v for v in self.f.calls if v.case['case_id']==case_id]
  closed=all(v.owner.closed for v in calls);joined=all(not self.alive(h) for h in handles)
  if not closed or not joined:self.f.lease.hold=True
  proof={'processes_closed':closed,'handles_closed':closed,'python_callbacks_joined':joined,'calls':[v.call_id for v in calls],'callback_count':len(handles),'errors':errors,'guard_receipt':self.f.contract['host_guard']}
  self.f.emit({'kind':'case-native-closure','case_id':case_id,'proof':proof})
  need(closed and joined,'native-case-closeout-HOLD');return proof
if __name__=='__main__':raise SystemExit('SOURCE_ONLY_FINAL_NATIVE_RUNTIME_GO_REQUIRED')
