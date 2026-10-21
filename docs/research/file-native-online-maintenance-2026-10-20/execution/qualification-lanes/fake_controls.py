"""Finite R3 controls. Source GO does not authorize importing or executing this suite."""
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import sys

def need(ok,message):
 if not ok:raise RuntimeError(message)

def sha(raw):
 return hashlib.sha256(raw).hexdigest()

def source(row,name):
 need(row.get('reviewed') is True and Path(row['path']).is_absolute(),'reviewed-control-source')
 path=Path(row['path'])
 for p in [path,*path.parents]:
  if p.exists():
   st=p.lstat()
   need(not p.is_symlink() and not getattr(st,'st_file_attributes',0)&1024,'source-link')
 with path.open('rb') as f:
  import os
  a=os.fstat(f.fileno());raw=f.read(2*1024**2+1);b=os.fstat(f.fileno())
 need(len(raw)==row['bytes']<=2*1024**2 and sha(raw)==row['sha256'] and a.st_nlink==1 and
  (a.st_dev,a.st_ino,a.st_size,a.st_mtime_ns)==(b.st_dev,b.st_ino,b.st_size,b.st_mtime_ns),
  'same-verified-source-buffer')
 spec=importlib.util.spec_from_file_location(name,row['path'])
 module=importlib.util.module_from_spec(spec);sys.modules[name]=module
 exec(compile(raw,row['path'],'exec'),module.__dict__)
 return module

class FakeHost:
 def __init__(self):self.events=[]
 def authorize(self,*args):
  self.events.append('authorize');raise RuntimeError('fake-runtime-refused')
 def poll(self):return dict(live=False)
 def record(self,*args,**kwargs):raise RuntimeError('no-fake-writer')
 def invoke(self,*args,**kwargs):raise RuntimeError('no-fake-native-runner')

def execute_r3_pure(profiles,finalizer,long):
 """Small in-memory parser/serializer units; no full forecast or file replay credit."""
 outcomes=[]
 def case(name,call,reject=False):
  try:
   try:value=call()
   except (RuntimeError,ValueError,TypeError,KeyError,AssertionError,OverflowError):
    need(reject,'unexpected-rejection');value=True
   else:need(not reject,'expected-rejection-missing')
   need(value is not False,'unexpected-false')
   outcomes.append(dict(id=name,status='passed',expected_rejection=reject))
  except BaseException as error:
   outcomes.append(dict(id=name,status='FAILED',error=ascii(error)[:400]))
 bridge=dict(schema='native-qpc-bridge-v1',run='a'*32,frequency=1000,
  qpc_before=100,perf_ns=1000000000,qpc_after=101)
 case('flat-bracket-retains-uncertainty',lambda:finalizer.clock_interval(bridge,101)==(1000000000,1001000000))
 for name,key,value in (('zero-frequency','frequency',0),('reversed-bracket','qpc_after',99)):
  bad=dict(bridge);bad[key]=value
  case(name,lambda bad=bad:finalizer.clock_interval(bad,101),True)
 case('invented-two-endpoint-bridge',lambda:finalizer.clock_interval(dict(start={},end={}),101),True)
 case('bool-QPC',lambda:finalizer.clock_interval(bridge,True),True)
 case('negative-QPC',lambda:finalizer.clock_interval(bridge,-1),True)
 begin=dict(qpc_start=10);start=dict(qpc=20)
 samples=[dict(qpc=15,phase='suspended'),dict(qpc=25,phase='running')]
 end=dict(qpc_end=30,in_n=0,first_write_qpc=None)
 case('empty-checker-chronology',lambda:finalizer.validate_chronology(begin,start,samples,end))
 changes=[
  ('END-before-START','end','qpc_end',19),
  ('running-RSS-after-END','sample1','qpc',31),
  ('running-RSS-before-START','sample1','qpc',19),
  ('suspended-RSS-after-START','sample0','qpc',21),
  ('unordered-RSS','sample1','qpc',14),
  ('bool-START-clock','start','qpc',True),
  ('negative-RSS-clock','sample0','qpc',-1),
  ('empty-input-with-write','end','first_write_qpc',21)]
 for name,target,key,value in changes:
  b=copy.deepcopy(begin);a=copy.deepcopy(start);r=copy.deepcopy(samples);e=copy.deepcopy(end)
  {'begin':b,'start':a,'sample0':r[0],'sample1':r[1],'end':e}[target][key]=value
  case(name,lambda b=b,a=a,r=r,e=e:finalizer.validate_chronology(b,a,r,e),True)
 for name,tick,reject in (('write-before-START',19,True),('write-after-END',31,True),
    ('positive-completed-write',22,False)):
  e=dict(end,in_n=1,first_write_qpc=tick)
  case(name,lambda e=e:finalizer.validate_chronology(begin,start,samples,e),reject)
 wire=dict(types={'counter':{'uint':64}},parameters={})
 for name,value in (('bool-u64',True),('negative-u64',-1),('oversized-u64',2**64)):
  case(name,lambda value=value:finalizer.wire_check(value,{'uint':64},wire),True)
 case('caller-named-node',lambda:finalizer.wire_check(1,{'named':'caller-type'},wire),True)
 case('unknown-wire-operator',lambda:finalizer.wire_check(1,{'caller':64},wire),True)
 node={'object':dict(x={'uint':64})}
 case('closed-source-object-positive',lambda:finalizer.wire_check(dict(x=1),node,wire))
 case('missing-source-field',lambda:finalizer.wire_check({},node,wire),True)
 case('extra-source-field',lambda:finalizer.wire_check(dict(x=1,y=0),node,wire),True)
 case('integer-is-not-literal-bool',lambda:finalizer.wire_check(1,{'literal':True},wire),True)
 case('source-ASCII-refuses-Unicode',lambda:finalizer.wire_check(chr(233),{'ascii':20},wire),True)
 case('actual-producer-UTF8-canonical',lambda:profiles.canonical(dict(text=chr(233)))==b'{"text":"\xc3\xa9"}')
 case('encoded-fragment-not-string-width',lambda:finalizer.wire_check({'x':1},{'bytes':2},wire),True)
 case('source-array-maximum',lambda:finalizer.wire_check([1,2,3],
  {'array':dict(count=2,item={'uint':64})},wire),True)
 cfg=profiles.capture_v2_schema({},128,8192,256)
 case('constructor-has-no-caller-schema',lambda:profiles.literal_capture_shape(cfg)==cfg)
 bad=dict(cfg,raw_frame_schema={'literal':0})
 case('caller-schema-substitution',lambda:profiles.literal_capture_shape(bad),True)
 minimum={'a':dict(bytes=100,files=1,directories=0),'b':dict(bytes=50,files=1,directories=0)}
 case('source-owned-allocation-positive',lambda:profiles.require_artifact_allocations(copy.deepcopy(minimum),minimum))
 for name,rows in (
   ('zero-mandatory-sinks',{k:dict(bytes=0,files=0,directories=0) for k in minimum}),
   ('omitted-mandatory-sink',{'a':minimum['a']}),
   ('understated-mandatory-sink',dict(minimum,a=dict(bytes=99,files=1,directories=0))),
   ('extra-mandatory-membership',dict(minimum,c=minimum['a'])),
   ('bool-allocation',dict(minimum,a=dict(bytes=True,files=1,directories=0)))):
  case(name,lambda rows=rows:profiles.require_artifact_allocations(rows,minimum),True)
 p=dict(source_operation='resolve',all_sink_components=dict(v=2,input_frames=10),
  stdout_bytes=20,rss_samples=2)
 cat,digest,export=profiles.seal_profile_catalogue([],{'history/resolve':p},{'selected-state':100})
 case('catalogue-global-canonical-hash',lambda:digest==profiles.sha(profiles.canonical(cat)))
 action=cat['profiles']['history/resolve']
 case('action-hash-is-canonical-payload',lambda:export['history/resolve']['profile_sha256']==
  profiles.sha(profiles.canonical(action)))
 case('auxiliary-is-not-call-profile',lambda:cat['profiles']['capture/sidebands']['profile_type']=='auxiliary'
  and 'capture/sidebands' not in export)
 case('unbound-preview-has-no-global-SHA',lambda:profiles.bind_local_catalogue(None,{'history/resolve':p},[])
  ['profile_catalogue_sha256'] is None)
 host=FakeHost()
 case('pending-long-refuses-before-private-sources',lambda:long.run_long(host,dict(go=False)),True)
 case('pending-long-did-not-touch-host',lambda:host.events==[])
 case('negative-live-poll',lambda:long.poll(host),True)
 case('duplicate-JSON-key',lambda:profiles.strict(b'{"a":1,"a":2}',100),True)
 case('nonfinite-JSON',lambda:profiles.strict(b'{"a":NaN}',100),True)
 need(len(outcomes)==48,'exact-forty-eight-R3-unit-cases')
 return outcomes

def run_controls(host,config):
 need(config.get('go') is True and config.get('runtime_go') is True and
  config.get('kind')=='finite-qualification-lanes-r3-pure-controls-v1',
  'source-only-no-control-RUN-GO')
 permit=host.authorize(config,'qualification-lanes-r3-pure-controls')
 need(permit.get('go') is True and len(permit.get('receipt_sha256',''))==64,
  'positive-finite-control-admission')
 import tracemalloc
 tracemalloc.start()
 try:
  profiles=source(config['bindings']['profile_calculator'],'long_source_profiles')
  finalizer=source(config['bindings']['finalizer'],'qualification_finalizer')
  long=source(config['bindings']['long_source'],'qualification_long')
  outcomes=execute_r3_pure(profiles,finalizer,long)
  _,peak=tracemalloc.get_traced_memory()
 finally:tracemalloc.stop()
 need(peak<=config['max_traced_bytes']<=16*1024**2,'finite-traced-control-memory')
 need(config['max_cases']==48,'exact-finite-case-admission')
 value=dict(status='passed' if all(x['status']=='passed' for x in outcomes) else 'FAILED',
  cases=outcomes,source_bindings=config['bindings'],peak_traced_bytes=peak,
  real_engine_calls=0,fixtures_written=0,native_containment_tested=False,
  full_forecast_tested=False,postexit_file_join_tested=False,runtime_qualification=False)
 raw=(json.dumps(value,sort_keys=True,separators=(',',':'),allow_nan=False)+'\n').encode()
 need(len(raw)<=config['max_output_bytes']<=32768,'bounded-controls-output')
 result=host.record(config['output'],raw,config['category'],config['max_output_bytes'])
 need(result.get('sha256')==sha(raw) and result.get('bytes')==len(raw),'control-raw-receipt')
 need(value['status']=='passed','controls-failed-see-raw-receipt')
 return result
