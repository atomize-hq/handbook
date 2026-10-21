"""SOURCE ONLY until parent approves this exact source and forecasts.
One accepted v20 outer control guard; no engine/Cargo/real cache/full fixtures.
"""
import errno,gzip,hashlib,json,os,sys,threading,time
from pathlib import Path
import admission_v20 as admission
import role_driver_v33 as roles
import capsule_adapter_v33 as adapter
import cache_stream_v33 as cache
R=admission.ROOT;H=Path(__file__).resolve().parent;BASE=R/'fixtures/native-controls-v33'
REPORT=R/'evidence/control/native-controls-v33-proof.json'
CAPS={'evidence_cap':16384,'stdout_cap':8192,'stderr_cap':8192,'receipt_cap':32768,'identity_budget':16,'control_only':True,'timeout':5}
results=[]

def request(name,value):
 p=R/'evidence/control'/(name+'-input.json');raw=(json.dumps(value,separators=(',',':'))+'\n').encode();p.write_bytes(raw);return p

def owner(name,value,*,callback=None,expected=0,interrupts=(),extra=()):
 p=request(name,value)
 return roles.OwnedCall('reader',[sys.executable,'-B',str(H/'fake_native_v32.py'),*extra],name=name,input_path=p,input_cap=262144,on_started=callback,expected_exit=expected,control_interrupts=interrupts,**CAPS)

def raw_proof(name,expected_delivery=None):
 p=R/'evidence/roles'/name;result=json.loads((p/'result.json').read_bytes());rows=[json.loads(x) for x in gzip.open(p/'samples.jsonl.gz','rt')]
 proof=[x for x in rows if x['kind']=='role-native-start-proof'];delivery=[x for x in rows if x['kind']=='role-input-first-byte']
 if expected_delivery:
  assert proof and delivery and proof[0]['input_bytes_delivered']==0 and proof[0]['positive_sample_count']>0 and proof[0]['monotonic']<delivery[0]['monotonic']
 if expected_delivery is False:assert not delivery and result['input_bytes_delivered']==0
 assert len(result['closure'])==2 and all(c['absent'] for c in result['closure'])
 for c in result['closure']:assert c['identities'] and all(i['returned_bytes']==0 and i['errno'] in (0,3) for i in c['identities'])
 return result

def main():
 BASE.mkdir(parents=True,exist_ok=False)
 for bad in (float('nan'),float('inf'),0,-1):
  try:roles.OwnedCall('reader',[sys.executable],name='MUSTNOTSTART-v33',timeout=bad)
  except ValueError:pass
  else:raise AssertionError('invalid owned deadline admitted')
 for bad in (float('nan'),float('inf')):
  try:cache.stream_member(BASE/'not-opened',BASE/'MUSTNOTWRITE',expected_bytes=0,expected_sha256='0'*64,operation_ceiling=0,deadline=bad,synthetic=True)
  except ValueError:pass
  else:raise AssertionError('nonfinite copy deadline admitted')
 assert not (BASE/'MUSTNOTWRITE').exists()
 results.append({'case':'finite-deadline-front-gates','native_calls':0,'stream_files_created':0})
 marks=[]
 def started(call):
  assert call.started_proof_ref and not call.input_released;marks.append(call.call_id)
 fast=owner('fast-native-v33',{'root':str(BASE/'store'),'action':'inspect'},callback=started);r=fast.wait();assert r['exit']==0 and marks==['fast-native-v33'];raw_proof('fast-native-v33',True)
 results.append({'case':'fast-native-immediate-EOF-exit','positive_actual_exec_before_first_byte':True})
 body={'root':str(BASE/'store'),'action':'inspect','padding':'x'*196608}
 back=owner('pipe-backpressure-v33',body,extra=('--backpressure',));back.wait();pr=raw_proof('pipe-backpressure-v33',True);assert pr['input_bytes_delivered']>65536
 results.append({'case':'nonblocking-input-and-output-under-backpressure','bytes':pr['input_bytes_delivered']})
 def fail(_):raise RuntimeError('CONTROL_CALLBACK_FAILURE')
 blocked=owner('callback-failure-v33',{'root':str(BASE/'store'),'action':'inspect'},callback=fail)
 try:blocked.wait()
 except RuntimeError as e:assert 'CONTROL_CALLBACK_FAILURE' in str(e)
 else:raise AssertionError('callback failure not censored')
 raw_proof('callback-failure-v33',False);results.append({'case':'callback-failure-before-stdin','expected_censor':True})
 real_probe=roles.prove_actual_exec
 def mismatch(expected,binary,parent,deadline):return real_probe(expected,'/bin/echo',parent,deadline)
 roles.prove_actual_exec=mismatch
 try:
  bad=owner('native-path-mismatch-v33',{'root':str(BASE/'store'),'action':'inspect'})
  try:bad.wait()
  except RuntimeError as e:assert 'actual-executable-mismatch' in str(e)
  else:raise AssertionError('native path mismatch accepted')
 finally:roles.prove_actual_exec=real_probe
 raw_proof('native-path-mismatch-v33',False);results.append({'case':'wrong-actual-native-path','expected_censor':True,'fault_scope':'new module function injection only, restored'})
 ready=BASE/'owned-ready.json';life=owner('owned-interrupt-v33',{'root':str(BASE/'store'),'action':'wait-owned','ready':str(ready),'case_id':'owned-interrupt'},expected=-15,interrupts=('terminate',))
 life.wait_started(2);end=time.monotonic()+2
 while not ready.exists():assert time.monotonic()<end;time.sleep(.002)
 ack=life.interrupt('terminate').wait(2);assert ack['operation']=='terminate';res=life.wait();assert res['exit']==-15;raw_proof('owned-interrupt-v33',True)
 results.append({'case':'owned-native-interrupt-after-genuine-child-barrier','exit':-15,'two_positive_closure_observations':True})
 # Two concurrent publication conflicts must retain their own exact per-call
 # input and env in AssertionError(row), without role resource censoring.
 forecasts=[]
 for i in (1,2):forecasts.append({'id':'conflict-'+str(i),'case_id':'conflict-'+str(i),'action':'publish','F08_bytes':4096,'F08_files':2,'stdout_cap':8192,'stderr_cap':8192,'input_cap':8192,'RSS_cap':16384,'receipt_cap':32768,'role':'checker'})
 forecasts.append({'id':'plain-none','case_id':'plain-none','action':'inspect','F08_bytes':0,'F08_files':0,'stdout_cap':8192,'stderr_cap':8192,'input_cap':8192,'RSS_cap':16384,'receipt_cap':32768,'role':'reader'})
 forecasts.append({'id':'observed-exit','case_id':'observed-exit','action':'packing','F08_bytes':0,'F08_files':0,'stdout_cap':8192,'stderr_cap':8192,'input_cap':8192,'RSS_cap':16384,'receipt_cap':32768,'role':'checker','exit_phases':['control-ready-exit']})
 fp=R/'evidence/control/native-controls-v33-forecasts.json';fp.write_text(json.dumps({'source_pin':'CONTROL_ONLY','rows':forecasts}))
 cfg={'run_name':'semantic-native-v33','control_only':True,'engine_argv':[sys.executable,'-B',str(H/'fake_native_v32.py')],'candidate_binding':None,'candidate_binding_sha256':None,'forecasts':str(fp),'forecasts_sha256':roles.sha256(fp),'source_pin':'CONTROL_ONLY','role_identity_budget':16,'producers':{}}
 native=adapter.NativeAdapter(cfg);driver=adapter.Driver(native);states=[None,None];threads=[]
 def publish(index):
  try:driver.call(BASE/'store','publish',timeout=5,case_id='conflict-'+str(index),forecast_id='conflict-'+str(index),env={'EXP_CONTROL_CONFLICT':'1'},request={'request':'distinct-'+str(index)})
  except AssertionError as e:states[index-1]={'row':e.args[0]}
  except BaseException as e:states[index-1]={'error':repr(e)}
  else:states[index-1]={'error':'genuine conflict returned success'}
 try:
  before=native._serial
  try:native.start(BASE/'store','inspect',case_id='plain-none',forecast_id='plain-none',not_finite=float('nan'))
  except ValueError:pass
  else:raise AssertionError('nonfinite engine input serialized')
  assert native._serial==before
  try:
   for i in (1,2):
    t=threading.Thread(target=publish,args=(i,),daemon=False);threads.append(t);t.start()
  finally:
   for t in threads:t.join(8)
   if any(t.is_alive() for t in threads):
    for call in native._calls:call._owner._abort.set()
    for t in threads:t.join(1)
   assert not any(t.is_alive() for t in threads),'Python callbacks not joined'
  for i,state in enumerate(states,1):
   assert state is not None and 'row' in state,state
   row=state['row'];assert isinstance(row,dict) and row['input']['request']['request']=='distinct-'+str(i) and row['input'].get('env') is None and row['exit']==2 and row['violations']==[] and row['executable_sha256']==driver.binary_sha256 and row['env']=={'EXP_CONTROL_CONFLICT':'1'}
  null_call=native.start(BASE/'store','inspect',case_id='plain-none',forecast_id='plain-none',env=None);null_call.wait();assert null_call.row['env'] is None
  exit_call=native.start(BASE/'store','packing',case_id='observed-exit',forecast_id='observed-exit',env={'EXP_EXIT':'control-ready-exit'})
  try:exit_call.wait()
  except AssertionError as e:
   row=e.args[0];assert row['exit']==86 and row['violations']==[] and row['env']=={'EXP_EXIT':'control-ready-exit'} and row['executable_sha256']==driver.binary_sha256
  else:raise AssertionError('raw-bound observed86 did not retain its own row')
  assert driver.binary_sha256==roles.sha256(Path(sys.executable).resolve()) and driver.source_binding['source_pin']=='CONTROL_ONLY'
 finally:assert native.close()==[]
 results.append({'case':'two-concurrent-semantic-conflicts','exact_distinct_rows':True,'env_outside_payload':True,'absent_env_preserved_as_None':True,'raw_exit86_is_not_crash_pass':True})
 # Actual Bash, not a Python wrapper proof, blocks before all ordinary reads.
 store=BASE/'ordinary/disposable/store';generation=store/'generations/g0';generation.mkdir(parents=True);(generation/'doc.txt').write_bytes(b'tiny\n');(generation/'MANIFEST.json').write_text('{}')
 pub={'attempt':'g0','generation_manifest':roles.sha256(generation/'MANIFEST.json')};(store/'publications').mkdir();(store/'publications/p0.json').write_text(json.dumps(pub));(store/'SELECTED.json').write_text(json.dumps({'publication':'p0','digest':roles.sha256(store/'publications/p0.json')}));(store/'STORE.json').write_text('{"mode":"B"}')
 out=BASE/'ordinary-copy';saved=BASE/'saved-selected.json'
 def bash_started(call):assert not out.exists() and not saved.exists() and not call.input_released
 argv=['/bin/bash',str(H/'bash_start_gate_v31.sh'),str(H/'ordinary_copy_v27.sh'),sys.executable,str(H),str(store),str(out),'',str(saved)]
 bash=roles.OwnedCall('reader',argv,name='actual-Bash-gate-v33',start_mode='bash-line',gate_sha256=roles.sha256(H/'bash_start_gate_v31.sh'),on_started=bash_started,**CAPS);result=bash.wait();body=json.loads(Path(result['outputs']['stdout']['path']).read_bytes());assert body['selection_read_count']==1 and body['engine_or_pack_decoder_used'] is False;raw_proof('actual-Bash-gate-v33',True)
 results.append({'case':'actual-native-Bash-before-SELECTED-read','selected_once':True,'exact_copy':True})
 # Streaming controls remain entirely F08 and never create the real C1 root.
 source=BASE/'stream-source';source.write_bytes(b's'*137123);wanted=roles.sha256(source);good=BASE/'stream-good'
 copied=cache.stream_member(source,good,expected_bytes=137123,expected_sha256=wanted,operation_ceiling=137123,deadline=time.monotonic()+3,synthetic=True);assert copied['chunk_ceiling']==65536 and roles.sha256(good)==wanted
 small=BASE/'stream-too-small'
 try:cache.stream_member(source,small,expected_bytes=137123,expected_sha256=wanted,operation_ceiling=137122,deadline=time.monotonic()+3,synthetic=True)
 except ValueError:pass
 else:raise AssertionError('stream prewrite ceiling not enforced')
 assert not small.exists()
 wrong=BASE/'stream-wrong-hash'
 try:cache.stream_member(source,wrong,expected_bytes=137123,expected_sha256='0'*64,operation_ceiling=137123,deadline=time.monotonic()+3,synthetic=True)
 except RuntimeError as e:assert 'manifest-mismatch-partial-retained' in str(e)
 else:raise AssertionError('stream wrong hash accepted')
 assert wrong.exists() and wrong.stat().st_size==137123
 destination=BASE/'stream-open-failure';opened=[];original_os_open=os.open;original_path_open=Path.open
 def track(path,*args,**kw):
  fd=original_os_open(path,*args,**kw)
  if Path(path)==source:opened.append(fd)
  return fd
 def denied(path,*args,**kw):
  if path==destination:raise PermissionError('CONTROL_DESTINATION_OPEN_FAILURE')
  return original_path_open(path,*args,**kw)
 os.open=track;Path.open=denied
 try:
  try:cache.stream_member(source,destination,expected_bytes=137123,expected_sha256=wanted,operation_ceiling=137123,deadline=time.monotonic()+3,synthetic=True)
  except PermissionError:pass
  else:raise AssertionError('destination failure not injected')
 finally:os.open=original_os_open;Path.open=original_path_open
 assert len(opened)==1
 try:os.fstat(opened[0])
 except OSError as e:assert e.errno==errno.EBADF
 else:raise AssertionError('source FD leaked after destination open failure')
 assert not (R/'cargo-private').exists()
 results.append({'case':'streaming-synthetic-and-source-FD-close','real_C1_created':False,'source_FD_EBADF_after_destination_refusal':True})
 import control_historical_v33
 historical=control_historical_v33.main();assert len(historical)==7
 results.append({'case':'synthetic-historical-validator','negative_controls':len(historical),'old_binary_launched':False})
 admission.bounded_receipt(REPORT,{'kind':'native-v33-proposed-controls','engine_or_Cargo_executed':False,'real_C1_created':False,'cases':results})
 print('NATIVE_V32_CONTROLS_PASS_PENDING_PARENT_REVIEW')

if __name__=='__main__':main()
