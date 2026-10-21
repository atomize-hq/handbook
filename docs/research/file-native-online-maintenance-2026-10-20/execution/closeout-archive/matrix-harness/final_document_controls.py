CONTRACT={"version":1,"counts":[64,1000],"modes":["A","B"],"stores":8,"semantic_publications":16,"full_oracles":8,"H_per_store":3,"engine_pin":"0628322e4b48e0efad19cdc32238a622d4aaa7a8","derivatives":["D-N64-A-with","D-N64-B-without","D-N64-B-with","D-N1000-A-without","D-N1000-A-with","D-N1000-B-without","D-N1000-B-with"],"overlap":"maintainer held until publisher closed"}
OPERATIONS=[('payload',[{'kind':'edit','id':'d000004','text':' matched payload'}]),('title',[{'kind':'rename','id':'d000004','text':'Matched title'}])]
def _context(c):
 _,scope=c.a.case_for(c.root)
 c.host.driver.context.value=dict(root=str(c.root),case_id=scope['case_id'],scope_id=scope['scope_id'],case_scope=scope['case_scope'])
def _start(c,role,events):
 def observed(event):
  need(role not in events and type(event) is dict,'D-one-native-start')
  need(type(event.get('pid')) is int and event['pid']>0 and type(event.get('positive_rss')) is int and 0<event['positive_rss']<=2048*1024**2,'D-positive-native-RSS')
  need(event.get('before_stdin_delivery') is True and event.get('same_job') is True and event.get('native_handle_identity') is True,'D-owned-start-before-input')
  need(event.get('executable_sha256')==c.h.driver.binary_sha256 and event.get('final_pin')==c.h.final_pin,'D-final-native-image')
  events[role]=copy.deepcopy(event)
 return observed
def _live(c,event):
 driver=c.host.driver;token=event['call_token']
 need(driver.live(token),'D-retained-native-process-live')
 identity=copy.deepcopy(driver.registry[token]['public_identity'])
 need((identity['pid'],identity['creation_time'])==(event['pid'],event['creation_time']),'D-same-retained-native-identity')
 return dict(observed_ns=time.perf_counter_ns(),call_token=token,native_identity=identity)
def _held(c,handle,event):
 need(handle['native_tokens']==[event['call_token']] and c.a.alive(handle),'D-same-single-held-maintainer')
 return _live(c,event)
def _installed(c,target,catalog_bytes,plan_bytes):
 store=c.document('STORE.json',4096)
 need(store==target,'D-installed-exact-candidate-STORE')
 catalog=c.read('pack-catalogs/'+target['catalog']+'.json',65536)
 plan=c.read('physical-plans/'+target['physical']['plan']+'.json',131072)
 need(catalog==catalog_bytes and hashlib.sha256(catalog).hexdigest()==target['catalog'],'D-installed-candidate-catalog')
 need(plan==plan_bytes and hashlib.sha256(plan).hexdigest()==target['physical']['plan'],'D-installed-candidate-plan')
 decoded=json.loads(plan);packs=json.loads(catalog)['packs'];generation=decoded['generation']
 need((generation['id'],generation['ordinal'],generation['catalog'],generation['packs'])==(target['physical']['id'],target['physical']['ordinal'],target['catalog'],packs),'D-installed-generation-pack-binding')
 return dict(store=store,catalog_sha256=target['catalog'],plan_sha256=target['physical']['plan'],generation=generation)
def _completed(c,event):
 token=c.host.driver.context.completed_token
 row=c.host.driver.wait_control(token,5)
 need(token==event['call_token']==row['call_token'],'D-actual-completed-token')
 need(row['violations']==[] and all(row[k] is True for k in ['resources_closed','raw_retained','python_callback_joined','stdout_eof','stderr_eof']),'D-uncensored-native-closure')
 need(row['exit']==0 and row['executable_sha256']==c.h.driver.binary_sha256,'D-actual-final-exit')
 need(row['process_started_ns']<=event['observed_ns']<=row['stdin_started_ns']<=row['process_done_ns'],'D-native-clock-order')
 state=c.host.driver.registry[token]
 return dict(token=token,process_started_ns=row['process_started_ns'],stdin_started_ns=row['stdin_started_ns'],recorded_completion_ns=row['process_done_ns'],transport_total_ns=row['spawn_total_ns'],raw_ref=copy.deepcopy(state['raw_reference']),metadata_ref=copy.deepcopy(c.host.events.calls[token]['meta_ref']),positive_start=copy.deepcopy(event))
def _operation(c,label,ops,codec,maintained,bootstrap):
 events={};captures={};controls=[];handles=[];primary=None;release_ns=[];held=[];target=None;catalog_bytes=None;plan_bytes=None
 def maintenance():
  value=c.h.pack(c.root,codec=codec,bootstrap=bootstrap,force=True,cleanup=False,reserve_bytes=64*1024**2,env=dict(EXP_BARRIER='physical-before-selection',EXP_READY=controls[0]['ready'],EXP_RELEASE=controls[0]['release']),on_started=_start(c,'maintenance',events))
  captures['maintenance']=_completed(c,events['maintenance']);captures['maintenance_response']=value
  return value
 def publisher_started(event):
  _start(c,'publisher',events)(event)
  if maintained:
   held.append(dict(phase='publisher-positive-before-input',maintenance=_held(c,handles[0],events['maintenance']),publisher=_live(c,event)))
 result=None
 try:
  _context(c)
  if maintained:
   prior=c.document('STORE.json',4096)
   controls.append(c.a.control(c.root,label))
   handles.append(c.a.spawn(maintenance))
   c.a.await_ready(controls[0],handles[0],15)
   need('maintenance' in events and c.a.alive(handles[0]),'D-real-held-physical-construction')
   journal=c.document('PACKING.json',65536)
   target=copy.deepcopy(journal.get('target'))
   need(type(target) is dict and target['store_id']==prior['store_id'],'D-candidate-store')
   need(type(target) is dict and journal.get('status')=='installed' and journal.get('id')==target['physical']['id'],'D-installed-before-selection')
   need(target['physical']['id']!=(prior.get('physical') or {}).get('id') and target['physical']['ordinal']==(prior.get('physical') or {}).get('ordinal',0)+1 and target['catalog']!=prior.get('catalog'),'D-real-new-physical-generation')
   catalog_bytes=c.read('pack-catalogs/'+target['catalog']+'.json',65536)
   plan_bytes=c.read('physical-plans/'+target['physical']['plan']+'.json',131072)
   need(hashlib.sha256(catalog_bytes).hexdigest()==target['catalog'] and hashlib.sha256(plan_bytes).hexdigest()==target['physical']['plan'],'D-captured-candidate-bytes')
   held.append(dict(phase='maintenance-ready-before-publisher',maintenance=_held(c,handles[0],events['maintenance'])))
  request=dict(request='final-document-'+label.split('-')[-1],base=c.selected(),ops=copy.deepcopy(ops),packet=False)
  started=time.perf_counter_ns()
  result=c.call('publish',request=request,timeout=45,on_started=publisher_started)
  ended=time.perf_counter_ns()
  captures['publisher']=_completed(c,events['publisher'])
  need(type(result.get('wall_ns')) is int and result['wall_ns']>=0,'D-actual-engine-wall-clock')
  need(result.get('status')=='ok' and result['result'].get('status')=='published' and result['result']['result'].get('outcome')=='accepted','D-real-payload-title-publication')
  if maintained:
   held.append(dict(phase='accepted-publisher-closed-before-release',maintenance=_held(c,handles[0],events['maintenance']),publisher_closed_observed_ns=time.perf_counter_ns()))
   c.a.release(controls[0]);release_ns.append(time.perf_counter_ns())
 except BaseException as error:primary=error
 finally:
  if handles or controls:c.h.join_callbacks(c.root,label,handles,controls,primary)
  elif primary is not None:raise primary
 interval=None;installed=None
 if maintained:
  need(set(events)=={'maintenance','publisher'} and events['maintenance']['pid']!=events['publisher']['pid'] and len(release_ns)==1 and len(held)==3,'D-two-retained-native-processes')
  interval=held[-1]['maintenance']['observed_ns']-held[1]['publisher']['observed_ns']
  need(interval>0 and held[-1]['publisher_closed_observed_ns']<release_ns[0],'D-publisher-work-inside-held-maintainer')
  returned=captures['maintenance_response']['result']
  need((returned['status'],returned['catalog'],returned['generation'])==('registered',target['catalog'],target['physical']),'D-returned-registered-target')
  installed=_installed(c,target,catalog_bytes,plan_bytes)
 observation=dict(label=label,response=result,engine_ns=result['wall_ns'],harness_elapsed_ns=ended-started,native=captures,maintenance_release_ns=release_ns,held_native_observation_interval_ns=interval,held_native_observations=held,candidate_target=target,installed=installed,engine_metrics=result.get('metrics'),qualification='publisher accepted and native-closed while same maintainer held; timestamps are observations, not native exit; no CPU overlap/percentile inference')
 c.h.note('D-operation',root=str(c.root),observation=observation)
 return observation
def _pair(control,n,mode,codec):
 control.init(mode=mode,n=n)
 selected=control.selected()
 publication=control.document('publications/'+selected['publication']+'.json',4*1024**2)
 need(publication['graph']['count']==n and publication['sequence']==0,'D-authentic-default-genesis')
 maintained=control.clone('D-N'+str(n)+'-'+mode+'-with')
 maintained.call('lifecycle',operation='setup')
 need(control.selected()==maintained.selected()==selected,'D-matched-initial-selection')
 observations={}
 for variant,c in [('without',control),('with',maintained)]:
  rows=[]
  for number,(name,ops) in enumerate(OPERATIONS):
   rows.append(_operation(c,'D-N'+str(n)+'-'+mode+'-'+variant+'-'+name,ops,codec,variant=='with',number==0))
  proof=c.verify('D-N'+str(n)+'-'+mode+'-'+variant)
  need(proof['actual_H']==3,'D-full-H3-oracle')
  observations[variant]=dict(root=str(c.root),rows=rows,full_oracle=proof)
 for left,right in zip(observations['without']['rows'],observations['with']['rows']):
  need(left['response']['result']['result']['state_digest']==right['response']['result']['result']['state_digest'],'D-matched-state-digest')
 control.h.note('D-matched-pair',count=n,mode=mode,initial_selected=selected,observations=observations,no_third_seed=True,forced_shallow_correctness=True)
 return dict(count=n,mode=mode,roots=[str(control.root),str(maintained.root)],H_each=3,publications=4,full_oracles=2,qualified_process_overlap=all(r['held_native_observation_interval_ns']>0 for r in observations['with']['rows']))
def run(case,args):
 need(type(args) is dict and set(args)=={'version','codec'} and type(args['version']) is int and args['version']==1 and args['codec']=='zstd','D-closed-constructor-data')
 need(case.h.final_pin==CONTRACT['engine_pin'],'D-final-pin')
 results=[]
 for n in CONTRACT['counts']:
  for mode in CONTRACT['modes']:
   case.h.check_time()
   if n==64 and mode=='A':control=case
   else:
    root=case.a.alias(case.id,'D-N'+str(n)+'-'+mode+'-without',source=None)
    control=type(case)(case.host,case.id,root)
   results.append(_pair(control,n,mode,args['codec']))
 need(len(results)==4 and sum(r['publications'] for r in results)==16 and sum(r['full_oracles'] for r in results)==8,'D-all-original-cells')
 case.h.note('D-source-complete',results=results,stores=8,semantic_publications=16,full_oracles=8,ordinary_extraction='separate mandatory capsules',default_payloads=True,forced_shallow_correctness=True,no_percentile=True)
 return dict(status='passed',results=results,stores=8,semantic_publications=16,full_oracles=8)
