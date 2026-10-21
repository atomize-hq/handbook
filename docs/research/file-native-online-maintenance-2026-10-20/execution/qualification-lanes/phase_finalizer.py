
import base64
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import re
import time
import zlib
from long_source_profiles import (canonical, closed, encode_private_record, hex64,
          integer, need, no_links, read, sha, strict, METRIC_FIELDS_027)

API='history-postexit-finalizer-v1'
QPC_V2='qpc-ns-v2; whole-Job tail coverage; strict >40ms; coarse monotonic retained; no retrospective requalification'
CALL_FIELDS={
 'stream':{'v','kind','run','source_pin','binary_sha','guard_sha','driver_sha','profile_sha'},
 'begin':{'v','kind','call','seq','root','action','role','env','input_n','input_sha','input_scope','pid','creation','qpc_start'},
 'launch':{'v','kind','call','seq','argv','executable_sha256','producer_kind','producer_source_pin','source_files','startup_gate_sha256'},
 'start':{'v','kind','call','seq','pid','creation','qpc','rss','executable_sha256'},
 'data':{'v','kind','call','seq','stream','offset','n','sha','b64'},
 'rss':{'v','kind','call','seq','pid','creation','qpc','rss','peak','role','phase'},
 'end':{'v','kind','call','seq','exit','in_n','out_n','err_n','in_sha','out_sha','err_sha',
  'input_complete','out_eof','err_eof','rss_n','peak','qpc_end','first_write_qpc','native_closed',
  'io_closed','callbacks_joined','intent','violations'}}
PROVISIONAL='PROVISIONAL_UNQUALIFIED'
PHASE_BODY_KEYS={'schema','status','phase_id','plan_sha256','source_pin','binary_sha256',
 'budget_sha256','outer_run_id','source_profile_sha256','evidence','summary',
 'owned_closure_ref','checkpoint_ref','case_closure_ref','runtime_sha256','guard_sha256',
 'finalizer_sha256','native_call_index_ref'}


def source(row, name):
 raw=read(row['path'],integer(row['bytes'],0,2*1024**2,'source-bytes'))
 need(len(raw)==row['bytes'] and sha(raw)==row['sha256'] and row.get('reviewed') is True,
   'source-bytes:'+name)
 spec=importlib.util.spec_from_file_location('finalizer_'+name,row['path'])
 module=importlib.util.module_from_spec(spec)
 exec(compile(raw,row['path'],'exec'),module.__dict__)
 return module


def file_binding(row, maximum, label):
 need(type(row) is dict and {'path','bytes','sha256'}<=set(row) and hex64(row['sha256']) and
   type(row['path']) is str and row['path'].isascii(),
   'file-ref:'+label)
 size=integer(row['bytes'],0,maximum,'file-size:'+label)
 path=no_links(row['path'])
 h=hashlib.sha256();total=0
 with path.open('rb') as f:
  a=os.fstat(f.fileno())
  need(a.st_size==size,'file-size-binding:'+label)
  for block in iter(lambda:f.read(65536),b''):
   total+=len(block);need(total<=size,'file-grew:'+label);h.update(block)
  b=os.fstat(f.fileno())
 need(total==size and h.hexdigest()==row['sha256'] and
   (a.st_dev,a.st_ino,a.st_size,a.st_mtime_ns)==(b.st_dev,b.st_ino,b.st_size,b.st_mtime_ns),
   'closed-file-content:'+label)
 return path


def body_reference(row, limit, label):
 need(type(row) is dict and {'path','bytes','sha256'}<=set(row) and
   set(row)<= {'path','offset','bytes','sha256','source_pin','run','schema'},
   'closed-body-reference:'+label)
 path=no_links(row['path']);offset=integer(row.get('offset',0),0,2**63-1,'body-offset')
 count=integer(row['bytes'],1,limit,'body-bytes')
 with path.open('rb') as f:
  before=os.fstat(f.fileno())
  need(offset+count<=before.st_size,'body-span')
  f.seek(offset);raw=f.read(count)
  after=os.fstat(f.fileno())
 need(len(raw)==count and sha(raw)==row['sha256'] and
   (before.st_dev,before.st_ino,before.st_size,before.st_mtime_ns)==
   (after.st_dev,after.st_ino,after.st_size,after.st_mtime_ns),'body-ref-content')
 value=strict(raw,limit)
 need(raw==encode_private_record(value),'canonical-private-record')
 return value


def _lines(path, maximum_bytes, maximum_line):
 with no_links(path).open('rb') as f:
  total=offset=0
  while True:
   line=f.readline(maximum_line+1)
   if not line:break
   need(line.endswith(b'\n') and len(line)<=maximum_line,'complete-bounded-line')
   total+=len(line);need(total<=maximum_bytes,'stream-byte-bound')
   yield offset,line
   offset=total


def gzip_lines(row, limits):
 
 path=file_binding(row,limits['qpc_compressed_bytes'],'QPC-gzip')
 decoder=zlib.decompressobj(16+zlib.MAX_WBITS)
 pending=b'';expanded=rows=offset=0;compressed=hashlib.sha256()
 with path.open('rb') as f:
  while True:
   block=f.read(65536)
   if not block:break
   compressed.update(block)
   need(not decoder.eof,'gzip-concatenation-or-trailing')
   while block:
    out=decoder.decompress(block,65536)
    block=decoder.unconsumed_tail
    expanded+=len(out)
    need(expanded<=limits['qpc_expanded_bytes'],'QPC-expansion-bound')
    pending+=out
    while b'\n' in pending:
     line,pending=pending.split(b'\n',1);line+=b'\n'
     rows+=1
     need(rows<=limits['qpc_rows'] and len(line)<=limits['qpc_line_bytes'],
       'QPC-line-row-bound')
     yield offset,line
     offset+=len(line)
    need(len(pending)<=limits['qpc_line_bytes'],'QPC-pending-line-bound')
    need(not decoder.unused_data,'gzip-extra-member-or-padding')
    if decoder.eof:
     need(not block and f.read(1)==b'','gzip-exact-EOF')
     break
 need(decoder.eof and not pending and compressed.hexdigest()==row['sha256'],
   'gzip-truncated-or-incomplete-line')


def raw_capture(config):
 
 limit=config['limits'];ref=config['native_capture']
 path=file_binding(ref,limit['native_bytes'],'native-capture')
 calls={};header=None;line_count=0;total_frames=0
 for offset,line in _lines(path,ref['bytes'],limit['native_line_bytes']):
  line_count+=1
  need(line_count<=limit['native_rows'],'native-row-bound')
  row=strict(line,limit['native_line_bytes'])
  kind=row.get('kind')
  need(kind in CALL_FIELDS and set(row)==CALL_FIELDS[kind] and row['v']==2,
    'closed-v2-frame')
  need(line==encode_private_record(row),'native-canonical-frame')
  if kind=='stream':
   need(header is None and offset==0,'one-first-capture-header')
   header=row
   expected=config['identity']
   need(row['run']==expected['run'] and row['source_pin']==expected['source_pin'] and
     row['binary_sha']==expected['binary_sha256'] and
     row['guard_sha']==expected['guard_sha256'] and
     row['driver_sha']==expected['driver_sha256'] and
     row['profile_sha']==expected['profile_sha256'],'capture-header-bindings')
   continue
  need(header is not None and re.fullmatch('[0-9a-f]{32}',row['call']),
    'header-before-owned-call')
  token=row['call']
  if kind=='begin':
   need(token not in calls and len(calls)<limit['native_calls'],'unique-native-call')
   need(row['role'] in ('reader','checker','long-engine','publisher','maintenance'),
     'known-native-role')
   need(row['input_scope']=='planned' and row['seq']==0,'planned-BEGIN-seq0')
   if row['pid'] is not None:integer(row['pid'],1,2**32-1,'call-pid')
   if row['creation'] is not None:integer(row['creation'],1,2**64-1,'call-creation')
   integer(row['qpc_start'],1,2**64-1,'call-start')
   need(type(row['env']) is dict and len(canonical(row['env']))<=limit['env_bytes'],
     'bounded-exact-environment')
   calls[token]=dict(begin=row,launch=None,start=None,rss_identity=None,seq_next=0,planned=bytearray(),
    begin_offset=offset,end_offset=None,frames=0,bytes=0,
    digest=hashlib.sha256(),streams={k:hashlib.sha256() for k in ('in','out','err')},out_buffer=bytearray(),out_frames=[],
    sizes={k:0 for k in ('in','out','err')},rss=[],end=None)
  need(token in calls and calls[token]['end'] is None,'open-owned-call')
  c=calls[token]
  need(type(row['seq']) is int and row['seq']==c['seq_next'],'call-sequence-no-gap-or-duplicate')
  c['seq_next']+=1;c['digest'].update(line);c['frames']+=1;c['bytes']+=len(line)
  total_frames+=1
  if kind=='launch':
   need(c['launch'] is None and c['start'] is None,'one-LAUNCH-before-Popen')
   need(type(row['argv']) is list and 0<len(row['argv'])<=64 and
     all(type(x) is str and len(x.encode())<=4096 for x in row['argv']) and
     hex64(row['executable_sha256']) and row['producer_kind'] in ('engine','checker') and
     re.fullmatch('[0-9a-f]{40}',row['producer_source_pin']) and
     (row['startup_gate_sha256'] is None or hex64(row['startup_gate_sha256'])),
     'closed-authentic-launch-types')
   need(type(row['source_files']) is list and 0<len(row['source_files'])<=64,'bounded-launch-sources')
   for item in row['source_files']:
    closed(item,('path','sha256'),'launched-source')
    need(Path(item['path']).is_absolute() and hex64(item['sha256']),'launched-source-reference')
   c['launch']=row
  elif kind=='start':
   need(c['start'] is None and c['launch'] is not None and
     row['executable_sha256']==c['launch']['executable_sha256'],
     'exactly-one-START-matches-planned-launch-image')
   integer(row['pid'],1,2**32-1,'START-pid');integer(row['creation'],1,2**64-1,'START-creation')
   integer(row['qpc'],c['begin']['qpc_start'],2**64-1,'START-QPC')
   cap=config['caps']['reader_bytes'] if c['begin']['role']=='reader' else config['caps']['checker_bytes']
   integer(row['rss'],1,cap,'positive-before-input-RSS')
   need(c['begin']['pid'] in (None,row['pid']) and c['begin']['creation'] in (None,row['creation']),
     'START-matches-constructor-identity')
   need(c['rss_identity'] in (None,(row['pid'],row['creation'])),'START-matches-prestart-RSS-native-identity')
   c['start']=row
  elif kind=='data':
   need(row['stream'] in c['streams'] and hex64(row['sha']) and type(row['b64']) is str,
     'typed-data-frame')
   need(row['stream']=='in' or c['start'] is not None,'START-before-OUT-ERR')
   raw=base64.b64decode(row['b64'],validate=True)
   need(base64.b64encode(raw).decode()==row['b64'] and
     len(raw)==row['n'] and 0<len(raw)<=16384 and sha(raw)==row['sha'] and
     row['offset']==c['sizes'][row['stream']],'exact-lossless-data-frame')
   c['streams'][row['stream']].update(raw);c['sizes'][row['stream']]+=len(raw)
   if row['stream']=='in':c['planned'].extend(raw)
   if row['stream']=='out':
    c['out_buffer'].extend(raw);c['out_frames'].append((offset,len(line),sha(line)))
   need(c['sizes'][row['stream']]<=limit['per_call_stream_bytes'],'native-stream-bound')
  elif kind=='rss':
   need(c['launch'] is not None and
     (c['start'] is None or (row['pid'],row['creation'])==(c['start']['pid'],c['start']['creation'])) and
     c['rss_identity'] in (None,(row['pid'],row['creation'])) and row['role']==c['begin']['role'] and row['phase'] in ('suspended','start','sample'),
     'RSS-native-identity')
   c['rss_identity']=(row['pid'],row['creation'])
   for key in ('qpc','rss','peak'):integer(row[key],1,2**64-1,'RSS-'+key)
   role_limit=config['caps']['reader_bytes'] if row['role']=='reader' else config['caps']['checker_bytes']
   need(row['rss']<=role_limit and row['peak']<=role_limit,'role-RSS-cap')
   need(not c['rss'] or row['qpc']>=c['rss'][-1]['qpc'],'ordered-RSS')
   c['rss'].append(dict(qpc=row['qpc'],rss=row['rss'],peak=row['peak']))
   need(len(c['rss'])<=limit['rss_per_call'],'finite-native-RSS')
  elif kind=='end':
   need(c['start'] is not None,'no-START-constructor-prefix-is-not-qualified')
   need(row['exit'] in (0,2) and row['violations']==[] and row['intent'] is None,
     'no-hidden-native-censor')
   need(all(row[k] is True for k in ('input_complete','out_eof','err_eof',
     'native_closed','io_closed','callbacks_joined')),'native-actual-closure')
   need(row['qpc_end']>=c['begin']['qpc_start'] and row['rss_n']==len(c['rss']) and
     c['rss'],'native-complete-lifetime')
   for stream,field in (('in','in'),('out','out'),('err','err')):
    need(row[field+'_n']==c['sizes'][stream] and
      row[field+'_sha']==c['streams'][stream].hexdigest(),'native-final-stream-digest')
   need(len(c['planned'])==c['begin']['input_n'] and
     sha(bytes(c['planned']))==c['begin']['input_sha'],'planned-input-artifact')
   actual=integer(row['in_n'],0,len(c['planned']),'actual-completed-prefix')
   need(row['in_sha']==sha(bytes(c['planned'][:actual])) and
     row['input_complete']==(actual==len(c['planned'])),'actual-completion-prefix')
   if actual:
    integer(row['first_write_qpc'],c['start']['qpc'],row['qpc_end'],
      'first-positive-completion-operation-after-START')
   else:
    need(row['first_write_qpc'] is None,'no-write-counter-without-completed-byte')
   need(c['sizes']['in']==c['begin']['input_n'] and
     c['streams']['in'].hexdigest()==c['begin']['input_sha'],'complete-input-source')
   response=strict(bytes(c['out_buffer']),limit['per_call_stream_bytes'])
   need(type(response) is dict,'decoded-native-response')
   if row['exit']==2:
    need(c['begin']['action']=='publish' and response.get('status')=='error' and
      response.get('error') in ('conflict:expected-publication','conflict:captured-base'),
      'authentic-declared-semantic-conflict-only')
   cap=config['caps']['reader_bytes'] if c['begin']['role']=='reader' else config['caps']['checker_bytes']
   integer(row['peak'],max(c['start']['rss'],max(x['peak'] for x in c['rss'])),cap,'END-peak-covers-actual-samples')
   if response.get('peak_memory_bytes') is not None:
    cap=config['caps']['reader_bytes'] if c['begin']['role']=='reader' else config['caps']['checker_bytes']
    integer(response['peak_memory_bytes'],1,cap,'reported-native-peak')
   c['input']=strict(bytes(c['planned']),limit['per_call_stream_bytes']) if c['planned'] else None
   c['response_sha256']=sha(canonical(response))
   c['out_buffer'].clear();c['planned'].clear()
   c['end']=row;c['end_offset']=offset+len(line)
 need(header is not None and calls and all(c['end'] is not None for c in calls.values()),
   'complete-closed-native-file')
 result={}
 for token,c in calls.items():
  effective=dict(c['begin'],pid=c['start']['pid'],creation=c['start']['creation'])
  result[token]=dict(begin=effective,launch=c['launch'],input=c['input'],start=c['start'],end=c['end'],rss=c['rss'],
   out_frames=c['out_frames'],response_sha256=c['response_sha256'],
   ref=dict(schema='single-raw-ref-v1',path=str(path),run=header['run'],call=token,
    begin=c['begin_offset'],end=c['end_offset'],frames=c['frames'],
    bytes=c['bytes'],sha256=c['digest'].hexdigest()))
 return dict(header=header,calls=result,frames=total_frames,rows=line_count,
    file_sha256=ref['sha256'],file_bytes=ref['bytes'])


def rehydrate(value, native, config, *, depth=0, state=None):
 
 if state is None:state={'nodes':0,'responses':{}}
 state['nodes']+=1
 need(depth<=64 and state['nodes']<=250000,'phase-rehydration-bound')
 if type(value) is dict and value.get('schema')=='native-response-ref-v1':
  closed(value,('schema','raw_ref'),'native-response-projection')
  ref=value['raw_ref'];call=native['calls'].get(ref['call'])
  need(call is not None and ref==call['ref'],'projection-authentic-call-span')
  token=ref['call']
  if token not in state['responses']:
   out=bytearray();path=no_links(ref['path'])
   with path.open('rb') as stream:
    for offset,size,want in call['out_frames']:
     stream.seek(offset);line=stream.read(size)
     need(len(line)==size and sha(line)==want,'reopened-authentic-output-frame')
     frame=strict(line,config['limits']['native_line_bytes'])
     raw=base64.b64decode(frame['b64'],validate=True)
     need(frame['call']==token and frame['stream']=='out' and
       len(out)==frame['offset'] and sha(raw)==frame['sha'],'exact-output-projection')
     out.extend(raw)
     need(len(out)<=config['limits']['per_call_stream_bytes'],'response-RAM-bound')
   response=strict(bytes(out),config['limits']['per_call_stream_bytes'])
   need(sha(canonical(response))==call['response_sha256'],'parsed-native-response-hash')
   state['responses'][token]=response
  return state['responses'][token]
 if type(value) is dict:
  return {k:rehydrate(v,native,config,depth=depth+1,state=state) for k,v in value.items()}
 if type(value) is list:
  return [rehydrate(v,native,config,depth=depth+1,state=state) for v in value]
 return value


def clock_interval(bridge, qpc):
 
 closed(bridge,('clock_kind','frequency','start','end','max_bracket_ns','sha256'),'clock-bridge')
 need(bridge['clock_kind']=='windows-qpc' and hex64(bridge['sha256']),'native-clock-domain')
 copy=dict(bridge);want=copy.pop('sha256')
 need(sha(canonical(copy))==want,'actual-clock-receipt-bytes')
 f=integer(bridge['frequency'],1,2**63-1,'QPC-frequency')
 intervals=[]
 for point in (bridge['start'],bridge['end']):
  closed(point,('qpc_before','perf_ns','qpc_after'),'clock-bracket')
  before=integer(point['qpc_before'],0,2**64-1,'bracket-before')
  after=integer(point['qpc_after'],before,2**64-1,'bracket-after')
  perf=integer(point['perf_ns'],1,2**63-1,'bracket-perf')
  need((after-before)*10**9<=bridge['max_bracket_ns']*f,'clock-uncertainty-cap')
  lower=perf+((qpc-after)*10**9)//f
  upper=perf+((qpc-before)*10**9+f-1)//f
  intervals.append((lower,upper))
 lo=max(x[0] for x in intervals);hi=min(x[1] for x in intervals)
 need(lo<=hi,'inconsistent-clock-handshakes')
 return lo,hi


def _guard_file(config):
 row=config['guard_receipt']
 raw=read(row['path'],config['limits']['guard_json_bytes'])
 need(len(raw)==row['bytes'] and sha(raw)==row['sha256'],'same-verified-original-guard-buffer')
 value=strict(raw,config['limits']['guard_json_bytes'])
 identity=config['identity'];owner=value['owner'];timing=value['memory_sample_timing']
 need(value['guard_sha256']==identity['guard_sha256'] and
   value['budget_sha256']==identity['budget_sha256'] and
   owner['nonce']==identity['run'] and owner['job_root_pid']==identity['host_pid'] and
   owner['job_root_creation_time']==identity['host_creation'] and
   owner['state']=='closed','actual-original-guard-owner')
 need(value['exit']==0 and value['censor'] is None and value['errors']==[] and
   value['terminations']==[] and value['remaining_job_pids']==[] and
   value['missing_memory_samples']==0 and value['process_closure_verified'] is True,
   'no-original-guard-censor')
 need(timing['clock']=='perf_counter_ns' and timing['target_interval_ns']==20000000 and
   timing['censor_interval_ns']==40000000,'exact-original-QPC-program-clock')
 for key in ('intervals_over_40ms','slow_samples'):
  need(timing[key]==0,'guard-'+key)
 for key in ('max_interval_seconds','max_completion_interval_seconds',
    'max_collection_duration_seconds','max_sample_duration_seconds',
    'max_recorded_interval_seconds','root_end_coverage_gap_seconds',
    'job_end_coverage_gap_seconds'):
  need(type(timing[key]) in (int,float) and 0<=timing[key]<=.04,'guard-window-'+key)
 need(value['sampling_revision']==QPC_V2 or value['sampling_revision'].startswith('qpc-ns-v3;'),
   'supported-guard-source-shape')
 need(type(value['closure_checks']) is list and len(value['closure_checks'])==2,
   'two-original-Job-observations')
 previous=None
 for check in value['closure_checks']:
  need(check['members']==[] and type(check['epoch']) in (int,float) and
    math.isfinite(check['epoch']) and
    owner['resumed_epoch']<=check['epoch']<=owner['closed_epoch'],
    'numeric-two-empty-observations')
  need(previous is None or check['epoch']>=previous,'ordered-empty-observations')
  previous=check['epoch']
 if value['sampling_revision'].startswith('qpc-ns-v3;'):
  need(all(type(x.get('perf_counter_ns')) is int and
     x['perf_counter_ns']>timing['root_wait_returned_ns']
     for x in value['closure_checks']),'new-guard-numeric-postexit-empties')
 return value


def scan_guard(config, native):
 guard=_guard_file(config);timing=guard['memory_sample_timing'];identity=config['identity']
 need(Path(guard['raw_rss'])==Path(config['qpc_capture']['path']),'exact-QPC-stream-path')
 source_ref=config['bindings']['guard'];file_binding(source_ref,2*1024**2,'guard-program')
 old=guard['sampling_revision']==QPC_V2
 if old:
  text=read(source_ref['path'],2*1024**2).decode()
  wait=text.index('process.wait(timeout=max(.001, absolute_deadline-time.time()) + 10)')
  root_end=text.index("event='root-end-sampling-coverage'",wait)
  finally_close=text.index('for _ in range(2):',root_end)
  empty=text.index("closure_checks.append(dict(epoch=time.time(), members=remaining))",finally_close)
  need(wait<root_end<finally_close<empty,'old-exact-closure-control-flow')
 last_tick=last_complete=None;count=0;first=None;last=None;root_end=None;job_end=None
 peaks={};observations=[];identity_rows={};prefix=hashlib.sha256();prefix_hits={}
 wanted={body_reference(r['body_ref'],config['limits']['phase_bytes'],'phase-prefix-binding')
  ['evidence']['guard_observation']['observed_prefix_sha256'] for r in config['phase_refs']}
 for token,c in native['calls'].items():
  b,e=c['begin'],c['end']
  lo,hi=clock_interval(config['clock_bridge'],b['qpc_start'])
  endlo,endhi=clock_interval(config['clock_bridge'],e['qpc_end'])
  identity_rows.setdefault(b['pid'],[]).append(dict(creation=b['creation'],start=lo,
   end=endhi,role=b['role'],call=token,rss=c['rss']))
 for offset,line in gzip_lines(config['qpc_capture'],config['limits']):
  prefix.update(line)
  row=strict(line,config['limits']['qpc_line_bytes']);event=row.get('event')
  if event=='rss-sample':
   need(row.get('sample_complete') is True,'positive-full-QPC-sample')
   tick=integer(row['perf_counter_ns'],1,2**63-1,'QPC-tick')
   done=integer(row['completed_perf_counter_ns'],tick,2**63-1,'QPC-completion')
   need(done-tick<=40000000 and (last_tick is None or 0<tick-last_tick<=40000000) and
     (last_complete is None or 0<done-last_complete<=40000000),'raw-QPC-cadence')
   need(row['collection_duration_seconds']>=0 and row['collection_duration_seconds']<=.04,
     'raw-QPC-collection')
   need(type(row['processes']) is list and len(row['processes'])<=config['limits']['job_members'],
     'finite-QPC-members')
   total=0;seen=set();host=False
   for process in row['processes']:
    closed(process,('pid','creation_time','rss','peak'),'QPC-process')
    creation=integer(process['creation_time'],1,2**64-1,'same-HANDLE-creation')
    pid=integer(process['pid'],1,2**32-1,'sampled-pid')
    need(pid not in seen,'duplicate-QPC-PID');seen.add(pid)
    rss=integer(process['rss'],1,2**64-1,'sampled-RSS')
    peak=integer(process['peak'],rss,2**64-1,'sampled-peak');total+=rss
    if pid==identity['host_pid']:
     need(creation==identity['host_creation'],'actual-original-host-creation')
     host=True;limit=config['caps']['checker_bytes']
    else:
     matches=[x for x in identity_rows.get(pid,[]) if x['start']<=done and x['end']>=tick and x['creation']==creation]
     need(len(matches)==1,'PID-creation-lifetime-unambiguous')
     item=matches[0]
     need(item['rss'] and all(x['qpc']>=native['calls'][item['call']]['begin']['qpc_start']
           for x in item['rss']),'native-identity-linked-RSS')
     limit=config['caps']['reader_bytes'] if item['role']=='reader' else config['caps']['checker_bytes']
    need(rss<=limit and peak<=limit,'QPC-role-RSS-cap')
    peaks[pid]=max(peaks.get(pid,0),peak)
   need(total==row['job_rss'] and total<=config['caps']['aggregate_bytes'],
     'QPC-aggregate-RSS')
   need(0<=row['supervisor_rss']<=config['caps']['supervisor_bytes'],
     'QPC-supervisor-cap')
   if first is None:first=tick
   last=done;last_tick=tick;last_complete=done;count+=1
   observations.append((tick,done,host))
   if prefix.hexdigest() in wanted:prefix_hits[prefix.hexdigest()]=done
   need(len(observations)<=config['limits']['qpc_rows'],'bounded-QPC-index')
  elif event=='root-end-sampling-coverage':
   need(root_end is None and row['interval_seconds']<=.04,'root-exit-tail')
   root_end=integer(row['perf_counter_ns'],1,2**63-1,'root-exit-perf')
  elif event=='job-drain-sampling-coverage':
   need(job_end is None and row['empty_observed'] is True and row['members']==[] and
     0<=row['interval_seconds']<=.04,'actual-Job-tail')
   job_end=integer(row['perf_counter_ns'],1,2**63-1,'Job-empty-perf')
  else:
   raise RuntimeError('late/missing/unsupported-QPC-event:'+str(event))
 need(first and last and root_end and job_end and first<=root_end<=job_end and
   count==timing['completed_samples']==timing['samples'] and
   first<=timing['first_positive_full_sample_ns']<=first+40000000 and
   root_end==timing['root_wait_returned_ns'] and job_end==timing['job_empty_observed_ns'] and
   0<=root_end-last<=40000000 and 0<=job_end-last<=40000000,
   'whole-original-run-QPC-coverage')
 return dict(raw_sha256=config['qpc_capture']['sha256'],samples=count,
  first_ns=first,last_ns=last,host_exit_ns=root_end,job_empty_ns=job_end,
  observations=observations,prefix_hits=prefix_hits,peaks=peaks,owner=guard['owner'],guard=guard)



def actual_response(call, native, config):
 return rehydrate({'schema':'native-response-ref-v1','raw_ref':call['ref']},native,config)


def native_meta(ref, call, config, case_id):
 m=body_reference(ref,65536,'native-call-meta-v2')
 keys=('schema','call','run','raw_ref','process_started_ns','process_done_ns','spawn_total_ns',
  'source_pin','binary_sha256','producer_kind','producer_source_pin','identity',
  'source_proof_ref','image_proof_ref','qpc_frequency','clock_bridge_ref','closure_ref',
  'case_id','scope_id','case_scope')
 closed(m,keys,'native-call-meta-v2')
 launch=call['launch'];start=call['start'];end=call['end'];identity=config['identity']
 need(m['schema']=='native-call-meta-v2' and m['call']==call['ref']['call'] and
   m['run']==identity['run'] and m['raw_ref']==call['ref'] and
   m['case_id']==config['parent_case_id'] and m['scope_id']==case_id and
   re.fullmatch('[0-9a-f]{32}',m['case_scope']) and
   m['source_pin']==identity['source_pin'] and
   m['producer_kind']==launch['producer_kind'] and
   m['producer_source_pin']==launch['producer_source_pin'] and
   m['binary_sha256']==start['executable_sha256']==launch['executable_sha256'] and
   m['identity']=={k:start[k] for k in ('pid','creation','executable_sha256')},
   'actual-native-launch-start-meta-identity')
 bridge=body_reference(m['clock_bridge_ref'],65536,'actual-native-clock-bridge')
 need(bridge==config['clock_bridge'] and m['qpc_frequency']==bridge['frequency'],
   'actual-same-native-QPC-frequency-and-bridge')
 lo,hi=clock_interval(bridge,call['begin']['qpc_start'])
 _,last=clock_interval(bridge,end['qpc_end'])
 _,start_hi=clock_interval(bridge,start['qpc'])
 end_lo,_=clock_interval(bridge,end['qpc_end'])
 born=integer(m['process_started_ns'],1,start_hi,'scope-start-before-native-START')
 done=integer(m['process_done_ns'],max(born,end_lo),2**63-1,'scope-end-after-native-END')
 integer(m['spawn_total_ns'],done-born,2**63-1,'actual-spawn-inclusive-duration')
 for name in ('source_proof_ref','image_proof_ref','closure_ref'):
  proof=body_reference(m[name],65536,name)
  need(proof.get('call')==m['call'] and proof.get('run')==m['run'] and
    proof.get('executable_sha256')==m['binary_sha256'],'same-call-actual-proof:'+name)
  if name=='source_proof_ref':
   need(proof.get('producer_kind')==m['producer_kind'] and
     proof.get('producer_source_pin')==m['producer_source_pin'] and
     proof.get('argv_sha256')==sha(canonical(launch['argv'])) and
     proof.get('source_files_sha256')==sha(canonical(launch['source_files'])) and
     proof.get('startup_gate_sha256')==launch['startup_gate_sha256'] and
     proof.get('profile_sha256')==identity['profile_sha256'] and
     proof.get('admitted') is True,'actual-source-launch-policy-proof')
  elif name=='image_proof_ref':
   need(proof.get('pid')==start['pid'] and proof.get('creation')==start['creation'] and
     proof.get('same_handle') is True and proof.get('contained') is True,
     'same-native-handle-image-START-proof')
  else:
   need(proof.get('pid')==start['pid'] and proof.get('creation')==start['creation'] and
     all(proof.get(k) is True for k in ('process_reaped','io_closed','native_closed',
      'callbacks_joined','raw_writer_released')) and
     proof.get('raw_ref')==call['ref'],'actual-post-END-owned-closure')
 if launch['producer_kind']=='engine':
  need(launch['executable_sha256']==identity['binary_sha256'] and
    launch['producer_source_pin']==identity['source_pin'] and len(launch['argv'])==1,
    'same-final-engine-one-executable')
  value=call['input']
  need(type(value) is dict and value.get('root')==call['begin']['root'] and
    value.get('action')==call['begin']['action'],'actual-engine-stdin-root-action')
 else:
  sources={(x['path'],x['sha256']) for x in launch['source_files']}
  need(all((config['bindings'][n]['path'],config['bindings'][n]['sha256']) in sources
   for n in ('oracle','oracle_cli','tree_decoder','startup_gate')),'complete-checker-source-closure')
  need(launch['executable_sha256']==config['bindings']['python']['sha256'] and
    launch['startup_gate_sha256']==config['bindings']['startup_gate']['sha256'] and
    call['input'] is None and end['in_n']==0 and
    config['bindings']['oracle_cli']['path'] in launch['argv'] and
    call['begin']['root'] in launch['argv'],'actual-checker-argv-and-empty-stdin')
 call['meta']=m
 return m


def operation_kind(call):
 if call['launch']['producer_kind']=='checker':
  argv=call['launch']['argv'];mode='full'
  if '--mode' in argv:
   pos=argv.index('--mode');need(pos+1<len(argv),'checker-mode-argument');mode=argv[pos+1]
  need(mode in ('full','mapping','backlog'),'closed-source-checker-mode')
  return {'full':'full-oracle','mapping':'mapping-query','backlog':'backlog-query'}[mode]
 v=call['input'];action=v['action']
 if action=='lookup-bench':
  return 'paced-reader-one-query-batch' if 'concurrent_pacing' in v else 'lookup-bench-one'
 if action=='packing':
  need(v.get('operation') in ('build','cleanup','convert'),'source-packing-action')
  return v['operation']
 if action=='lifecycle':
  return {'quota':'quota','enable-terminal-v2':'terminal-v2-enable',
   'release-definitive-conflict':'terminal-v2-release'}[v['operation']]
 need(action in ('publish','resolve'),'source-engine-action')
 return action


def validate_case_calls(phase, index, native, config, evidence):
 need(index.get('phase_id')==phase['id'] and
   all(index.get(k)==config['identity'][k] for k in ('source_pin','binary_sha256','budget_sha256')) and
   index.get('outer_run_id')==config['identity']['run'],'complete-native-case-index-identity')
 entries=index['calls'];need(type(entries) is list and entries,'nonempty-ordered-native-case')
 used=set();counts={};by_sha={};prior=-1;scopes=set()
 for item in entries:
  closed(item,('raw_ref','meta_ref'),'ordered-raw-meta-index-entry')
  r=item['raw_ref'];token=r['call']
  need(token in native['calls'] and token not in used,'exact-case-owned-call-once')
  call=native['calls'][token]
  need(r==call['ref'] and r['begin']>prior and call['begin']['root']==phase['root'],
    'actual-case-root-ordered-call-span')
  prior=r['begin'];used.add(token)
  meta=native_meta(item['meta_ref'],call,config,phase['id']);scopes.add(meta['case_scope'])
  key=operation_kind(call);counts[key]=counts.get(key,0)+1;by_sha[r['sha256']]=call
 expected={'publish':100*phase['publishers'],'paced-reader-one-query-batch':40,'full-oracle':1}
 if phase['publishers']==4:expected.update({'terminal-v2-release':300,'quota':100})
 if phase['online']:
  expected.update({'build':100,'cleanup':101,'backlog-query':2})
  if phase['publishers']==1:expected['mapping-query']=198
 need(counts==expected and len(scopes)==1,'complete-source-native-case-schedule-no-extra-or-skip')
 for slot in evidence['slots']:
  for offer in slot['offers']:
   need(offer['raw_sha256'] in by_sha,'offer-linked-to-native-index')
   call=by_sha[offer['raw_sha256']];m=call['meta'];response=actual_response(call,native,config)
   need(call['input'].get('request')==offer['request'] and
     response['wall_ns']==offer['engine_ns'] and m['spawn_total_ns']==offer['spawn_ns'] and
     m['process_started_ns']==offer['start_ns'] and m['process_done_ns']==offer['end_ns'],
     'actual-request-and-native-engine-spawn-offer-clocks')
   if offer['outcome']=='accepted':
    need(call['end']['exit']==0 and response['status']=='ok' and response['error'] is None and
      response['result']['result']==offer['result'] and
      response['result']['selected']==offer['selected'],'actual-accepted-response-to-offer')
   else:
    need(call['end']['exit']==2 and response['status']=='error' and
      response['error']==offer['error']=='conflict:expected-publication',
      'actual-definitive-conflict-response-to-offer')
 readers=index['readers']
 need(type(readers) is list and len(readers)==40,'all40-authenticated-reader-headers')
 seen=set();peaks=[]
 for row in readers:
  pair=(row['reader'],row['batch']);need(pair not in seen,'unique-reader-batch');seen.add(pair)
  call=by_sha[row['raw_ref']['sha256']];response=actual_response(call,native,config)
  value=response['result'];header=value['concurrent_pacing'];v=call['input']
  need(row['raw_ref']==call['ref'] and header==row['header'] and v['count']==1000 and
    v['requests']==[row['request']] and header['reader']==pair[0] and header['batch']==pair[1] and
    header['phase_id']==phase['id'] and header['family']=='f05-selected' and
    header['pid']==call['start']['pid'] and header['completed']==1000 and
    header['deadline_ok'] is True and header['max_lag_ticks']<=header['frequency']//10 and
    header['cache_after_drop_bytes']==0 and header['successors']<=60,
    'actual-source-paced-reader-header')
  probe=value['rows'][0]
  donor=config['plan']['donors'][phase['donor']]
  expected=donor['expected_outcomes'][row['request']['category']]
  need(probe['id']==row['request']['id'] and probe['category']==row['request']['category'] and
    probe['result_digest']==sha(canonical(expected)) and len(probe['warm_ns'])==1000 and
    probe['first_lookup_ns']==row['first_ns'] and all(type(n) is int and n>=0 for n in probe['warm_ns']),
    'all1000-actual-warm-timings-and-exact-outcome')
  peaks.append(call['end']['peak'])
 need(seen=={(a,b) for a in range(4) for b in range(10)} and
   evidence['readers']['reader_rss_max']==max(peaks),'actual-complete-reader-RSS-and-batch-set')

 builds=[c for c in by_sha.values() if operation_kind(c)=='build']
 derived=[];holds=[];generations=[]
 for c in builds:
  response=actual_response(c,native,config);m=response['metrics'];r=response['result']
  need(response['status']=='ok' and r['status']=='registered','actual-selected-build')
  need(all(type(m[n]) is int and m[n]>=0 for n in METRIC_FIELDS_027.values()),'instrumented-counters')
  need(m[METRIC_FIELDS_027['forbidden_global_operations']]==0 and
   m[METRIC_FIELDS_027['unchanged_historical_body_opens']]==0,'forbidden-or-unchanged-work')
  derived.append({k:m[n] for k,n in METRIC_FIELDS_027.items()
   if k not in ('forbidden_global_operations','unchanged_historical_body_opens')})
  holds.extend(m['physical_activation_holds_ns'])
  generations.append((c['meta']['process_done_ns'],r['generation']['id']))
 need(evidence['maintenance']['metrics']==derived and
  evidence['maintenance']['activation_holds_ns']==holds and
  evidence['maintenance']['actual_mapping_changes']==len(generations) and
  len({g for _,g in generations})==len(generations),'raw-derived-maintenance-summary')
 ready_refs=index['ready_refs'];need(type(ready_refs) is list and len(ready_refs)==4,'four-ready-markers')
 origin=body_reference(index['origin_ref'],4096,'native-pace-origin')
 old=evidence['readers']['old_reader'];ready=body_reference(ready_refs[0],4096,'old-native-ready')
 first=next(r for r in readers if r['reader']==0 and r['batch']==0)
 call=by_sha[first['raw_ref']['sha256']];header=first['header']
 need(ready['pid']==call['start']['pid']==old['pid'] and ready['phase_id']==phase['id'] and
  ready['frequency']==origin['frequency']==header['frequency'] and
  header['origin']==origin['origin'] and header['ready_sha256']==ready_refs[0]['sha256'] and
  header['anchor_sha256']==index['origin_ref']['sha256'],'native-ready-anchor-first-reader')
 cap_lo,cap_hi=clock_interval(config['clock_bridge'],ready['capture_qpc'])
 rel_lo,rel_hi=clock_interval(config['clock_bridge'],header['pins_dropped'])
 _,origin_hi=clock_interval(config['clock_bridge'],origin['origin'])
 need(cap_lo<=old['captured_ns']<=cap_hi and rel_lo<=old['released_ns']<=rel_hi and
  rel_lo-cap_hi>15000000000 and rel_hi<=origin_hi+23600000000 and
  rel_hi<max(s['done_ns'] for s in evidence['slots']),'actual-old-pin-window')
 actual=[g for t,g in generations if t<=rel_lo]
 need(old['successor_mapping_ids']==actual and
  (len(actual)>=2 if phase['online'] else not actual),'actual-old-pin-successor-mappings')
 overlap=any(c['meta']['process_started_ns']<p['meta']['process_done_ns'] and
  p['meta']['process_started_ns']<c['meta']['process_done_ns']
  for c in builds for p in by_sha.values() if operation_kind(p)=='publish')
 need(evidence['maintenance']['positive_publisher_overlap']==overlap,'raw-native-maintenance-overlap')
 return used



def reconstruct_initial_case(case_id,index,native,config,donor,bootstrap=False):
 calls=[];tokens=set();prior=-1
 need(index['phase_id']==case_id and index['outer_run_id']==config['identity']['run'] and
  index['source_pin']==config['identity']['source_pin'],'initial-case-identity')
 for item in index['calls']:
  closed(item,('raw_ref','meta_ref'),'initial-raw-meta-entry')
  r=item['raw_ref'];call=native['calls'][r['call']]
  need(r==call['ref'] and r['call'] not in tokens and r['begin']>prior and
   call['begin']['root']==donor['root'],'initial-owned-call')
  prior=r['begin'];tokens.add(r['call'])
  native_meta(item['meta_ref'],call,config,case_id);calls.append(call)
 kinds=[operation_kind(c) for c in calls]
 clean=[c for c in calls if operation_kind(c)=='cleanup']
 need(1<=len(clean)<=160,'early-terminal-cleanup-prefix')
 for n,c in enumerate(clean):
  r=actual_response(c,native,config)['result']
  need(r['status']==('complete' if n==len(clean)-1 else 'cleaning'),'positive-terminal-cleanup')
 if bootstrap:
  need(kinds==['build']+['cleanup']*len(clean)+['full-oracle'],'bootstrap-source-order')
 else:
  sequence=[];read_calls=[]
  for fmt in range(2):
   if fmt:sequence.append('convert')
   for request in donor['lookup_requests']:
    sequence+=['resolve']*50+['lookup-bench-one']
    read_calls += [(request,False)]*50+[(request,True)]
   sequence.append('full-oracle')
  sequence+=['cleanup']*len(clean)+['terminal-v2-enable','quota','full-oracle']
  need(kinds==sequence,'both-representations-source-order')
  reads=[c for c in calls if operation_kind(c) in ('resolve','lookup-bench-one')]
  for call,(request,warm) in zip(reads,read_calls):
   v=call['input'];result=actual_response(call,native,config)['result']
   want=donor['expected_outcomes'][request['category']]
   if warm:
    need(v['requests']==[request] and v['count']==500 and len(result['rows'])==1 and
     result['rows'][0]['category']==request['category'] and result['rows'][0]['id']==request['id'] and
     len(result['rows'][0]['warm_ns'])==500 and
     result['rows'][0]['result_digest']==sha(canonical(want)),'initial-warm-source-query')
   else:need(v['request_id']==request['id'] and result==want,'initial-fresh-exact-outcome')
 return tokens


def reconstruct_whole_schedule(trace,native,config,phase_refs):
 closed(trace,('api','source_pin','profile_sha256','case_indices'),'source-case-manifest')
 need(trace['api']=='history-expanded-case-index-v2' and
  trace['source_pin']==config['identity']['source_pin'] and
  trace['profile_sha256']==config['bindings']['profiles']['sha256'],'whole-source-model')
 cases=trace['case_indices'];need(type(cases) is list and len(cases)==22,'all22-source-cases')
 by_id={}
 for row in cases:
  closed(row,('id','index_ref'),'source-case-index')
  need(row['id'] not in by_id,'unique-case-index');by_id[row['id']]=row['index_ref']
 expected={p['id'] for p in config['plan']['phases']}
 for identity in ('H1','H10'):
  expected.add('lookup-'+identity)
  for pubs in (1,4):expected.add('bootstrap-'+identity+'-p'+str(pubs))
 need(set(by_id)==expected,'exact16phases-two-lookups-four-bootstrap')
 used=set()
 for phase in config['plan']['phases']:
  need(by_id[phase['id']]==phase_refs[phase['id']],'body-index-matches-manifest')
  index=body_reference(by_id[phase['id']],config['limits']['phase_bytes'],'whole-phase-index')
  for row in index['calls']:
   token=row['raw_ref']['call'];need(token not in used,'call-not-reused-between-cases')
   used.add(token)
 for identity,donor in config['plan']['donors'].items():
  for case_id,bootstrap in [('lookup-'+identity,False)]+[
    ('bootstrap-'+identity+'-p'+str(p),True) for p in (1,4)]:
   index=body_reference(by_id[case_id],config['limits']['phase_bytes'],'initial-case-index')
   tokens=reconstruct_initial_case(case_id,index,native,config,donor,bootstrap)
   need(not used.intersection(tokens),'initial-calls-not-reused');used.update(tokens)
 need(used==set(native['calls']),'all-native-calls-in-mandatory-source-schedule')
 need(len({c['meta']['case_scope'] for c in native['calls'].values()})==22,'22-actual-distinct-case-scopes')
 counts={}
 for c in native['calls'].values():
  k=operation_kind(c);counts[k]=counts.get(k,0)+1
 need(counts['full-oracle']==26 and counts['mapping-query']==1188 and
  counts['backlog-query']==24 and counts['publish']==4000 and
  counts['terminal-v2-release']==2400 and counts['quota']==802,'whole-native-source-counts')
 return counts


def _phase_windows(plan, refs, config, native, guard, history_module):
 need(type(refs) is list and len(refs)==16,'exact-sixteen-final-phase-references')
 phases={p['id']:p for p in plan['phases']}
 need(len(phases)==16 and {r['id'] for r in refs}==set(phases),'exact16-phase-membership')
 results=[];seen_refs=set();seen_attachments=set();previous_end=None;actual_counts={}
 heads={d['root']:d['selected'] for d in plan['donors'].values()}
 for ref in refs:
  closed(ref,('id','body_ref'),'phase-index-entry')
  body=body_reference(ref['body_ref'],config['limits']['phase_bytes'],ref['id'])
  need(set(body)==PHASE_BODY_KEYS and body['schema']=='history-final-provisional-v1' and
    body['status']==PROVISIONAL and body['phase_id']==ref['id'] and
    body['plan_sha256']==plan['plan_sha256'],'FINAL-sealed-phase-body')
  key=(str(Path(ref['body_ref']['path'])),ref['body_ref'].get('offset',0),ref['body_ref']['sha256'])
  need(key not in seen_refs,'duplicate-phase-body-span');seen_refs.add(key)
  identity=config['identity']
  need(all(body[k]==identity[k] for k in ('source_pin','binary_sha256','budget_sha256')) and
    body['outer_run_id']==identity['run'] and
    body['source_profile_sha256']==config['bindings']['profiles']['sha256'] and
    body['runtime_sha256']==config['bindings']['runtime']['sha256'] and
    body['guard_sha256']==identity['guard_sha256'] and
    body['finalizer_sha256']==config['bindings']['finalizer']['sha256'],'phase-final-binding')
  need('receipt_sha256' not in body['evidence'] and
    'receipt_sha256' not in body['summary'],'external-body-hash-no-circular-placeholder')
  evidence=rehydrate(body['evidence'],native,config)
  evidence['receipt_sha256']=ref['body_ref']['sha256']
  observation=evidence['guard_observation']
  need(observation['status']==PROVISIONAL and observation['final_receipt'] is None and
    observation['outer_run_id']==identity['run'] and
    observation['host_pid']==identity['host_pid'] and
    observation['host_creation_id']==str(identity['host_creation']),
    'provisional-original-host-identity')
  start=integer(observation['phase_start_ns'],guard['first_ns'],guard['host_exit_ns'],'phase-start')
  end=integer(observation['phase_end_ns'],start,guard['host_exit_ns'],'phase-end')
  need(previous_end is None or start>=previous_end,'ordered-nonoverlapping-source-phases')
  previous_end=end
  hit=guard['prefix_hits'].get(observation['observed_prefix_sha256'])
  need(hit is not None and abs(hit-end)<=40000000,'actual-QPC-physical-prefix-hash-at-case-end')
  before=[x for x in guard['observations'] if x[0]<=start]
  within=[x for x in guard['observations'] if start<x[0]<=end]
  need(before,'sample-before-phase-start')
  covered=[before[-1]]+within
  need(0<=start-covered[0][0]<=40000000 and
    0<=end-covered[-1][0]<=40000000 and all(x[2] for x in covered),
    'host-present-sample-coverage-through-phase')
  root=phases[ref['id']]['root']
  need(evidence['slots'][0]['offers'][0]['request']['base']==heads[root],'actual-cross-phase-base-chain')
  heads[root]=evidence['slots'][-1]['accepted_selected']
  summary=history_module.validate_phase_evidence(phases[ref['id']],evidence)
  need({k:v for k,v in summary.items() if k!='receipt_sha256'}==body['summary'],'phase-source-validator-result')
  for name in ('owned_closure_ref','checkpoint_ref','case_closure_ref'):
   proof=body_reference(body[name],config['limits']['metadata_bytes'],name)
   need(type(proof) is dict and
     proof.get('case_id')==body['phase_id'] and
     all(proof.get(k)==body[k] for k in ('source_pin','binary_sha256',
      'budget_sha256','outer_run_id')) and
     proof.get('clock_domain')==observation['clock_domain'] and
     type(proof.get('start_ns')) is int and type(proof.get('end_ns')) is int and
     start<=proof['start_ns']<=proof['end_ns']<=end,'phase-run-source-clock-attachment')
   signature=(body[name]['path'],body[name].get('offset',0),body[name]['sha256'])
   need(signature not in seen_attachments,'attachment-not-reused-across16-cases')
   seen_attachments.add(signature)
   if name=='owned_closure_ref':
    need(body[name]['sha256']==observation['owned_drain_sha256'],'actual-owned-drain-prefix-link')
   else:
    need(proof.get('owned_drain_sha256')==observation['owned_drain_sha256'],
      'attachment-exact-owned-drain-link')
   if name=='owned_closure_ref':
    need(proof.get('children_empty') is True and proof.get('callbacks_joined') is True and
      proof.get('scope')=='owned-children-IO-handles','actual-owned-phase-drain')
   elif name=='checkpoint_ref':
    need(proof.get('go') is True and proof.get('no_other_semantic_writer') is True,
      'actual-closed-phase-census')
   else:
    need(proof.get('processes_closed') is True and proof.get('handles_closed') is True and
      proof.get('python_callbacks_joined') is True,'case-closure-before-final-seal')
  native.setdefault('case_index_refs',{})[ref['id']]=body['native_call_index_ref']
  index=body_reference(body['native_call_index_ref'],config['limits']['phase_bytes'],'complete-case-call-index')
  used=validate_case_calls(phases[ref['id']],index,native,config,evidence)
  for token in used:
   call=native['calls'][token]
   lo,hi=clock_interval(config['clock_bridge'],call['begin']['qpc_start'])
   elo,ehi=clock_interval(config['clock_bridge'],call['end']['qpc_end'])
   need(lo>=start and ehi<=end and call['meta']['process_started_ns']>=start and
    call['meta']['process_done_ns']<=end,'native-and-rich-scope-bracketed-by-full-case')
   action=call['begin']['action'];actual_counts[action]=actual_counts.get(action,0)+1
  results.append(dict(id=ref['id'],body_ref=ref['body_ref'],summary=summary,
       start_ns=start,end_ns=end,native_calls=len(used)))
 return results,actual_counts


def census(config, consumed_output=None):
 
 rows=config['census']['roots']
 need(type(rows) is dict and rows,'explicit-whole-root-census')
 paths=[Path(r['root']) for r in rows.values()]
 need(all(p.is_absolute() for p in paths) and
   all(not a.is_relative_to(b) and not b.is_relative_to(a)
    for i,a in enumerate(paths) for b in paths[i+1:]),'disjoint-census-roots')
 result={};all_bytes=all_files=all_dirs=0
 for name,spec in rows.items():
  need({'root','bytes','files','directories','remaining_reservation'}==set(spec),'census-root-spec')
  for k in ('bytes','files','directories'):integer(spec[k],0,2**63-1,'root-cap:'+k)
  root=no_links(spec['root']);used=count=dirs=0;h=hashlib.sha256()
  if root.exists():
   for folder,children,names in os.walk(root,followlinks=False):
    children.sort();names.sort()
    for n in children:
     p=no_links(Path(folder)/n);need(p.is_dir(),'census-directory')
     dirs+=1
     h.update(encode_private_record(dict(path=p.relative_to(root).as_posix(),directory=True)))
    for n in names:
     p=no_links(Path(folder)/n);need(p.is_file(),'census-file')
     size=p.stat().st_size;used+=size;count+=1
     h.update(encode_private_record(dict(path=p.relative_to(root).as_posix(),bytes=size)))
     need(count<=spec['files'] and dirs<=spec['directories'] and used<=spec['bytes'],
       'actual-census-cap:'+name)
  reserve=dict(spec['remaining_reservation'])
  closed(reserve,('bytes','files','directories'),'remaining-assigned-reservation')
  for k in reserve:integer(reserve[k],0,2**63-1,'root-reservation:'+k)
  if consumed_output is not None and name==consumed_output['category']:
   reserve['bytes']-=consumed_output['max_bytes'];reserve['files']-=1
  need(all(type(v) is int and v>=0 for v in reserve.values()),'nonnegative-live-reservations')
  if True:
   need(used+reserve['bytes']<=spec['bytes'] and count+reserve['files']<=spec['files'] and
     dirs+reserve['directories']<=spec['directories'],'actual-plus-valid-reservations:'+name)
  result[name]=dict(bytes=used,files=count,directories=dirs,
       membership_sha256=h.hexdigest(),remaining_reservation=reserve)
  all_bytes+=used+reserve['bytes'];all_files+=count+reserve['files'];all_dirs+=dirs+reserve['directories']
 carry=config['census']['foreign_carried_reservations']
 closed(carry,('bytes','files','directories','receipt_ref'),'foreign-reserved-footprint')
 carried=body_reference(carry['receipt_ref'],config['limits']['metadata_bytes'],'foreign-ledger')
 need(type(carried) is dict and carried.get('schema')=='foreign-carried-reservations-v1' and
   carried.get('source_pin')==config['identity']['source_pin'] and
   carried.get('budget_sha256')==config['identity']['budget_sha256'] and
   carried.get('run')==config['identity']['run'],'exact-foreign-ledger-identity')
 for k in ('bytes','files','directories'):
  integer(carry[k],0,2**63-1,'foreign-'+k)
  need(carried.get(k)==carry[k] and type(carried.get(k)) is int,'authenticated-foreign-amount:'+k)
 all_bytes+=carry['bytes'];all_files+=carry['files'];all_dirs+=carry['directories']
 caps=config['census']['combined_caps']
 closed(caps,('bytes','files','directories'),'combined-caps')
 for k in caps:integer(caps[k],0,2**63-1,'combined-cap:'+k)
 need(all_bytes<=caps['bytes'] and all_files<=caps['files'] and all_dirs<=caps['directories'],
   'whole-combined-resource-cap')
 return dict(roots=result,combined=dict(bytes=all_bytes,files=all_files,directories=all_dirs),
    foreign_reservations=carry,planned_deletion_credit=0)



def validate_native_lifetime(lifetime, completion, grant, config):
 closed(lifetime,('schema','run_nonce','guard_sha256','creator','creator_job_query',
  'job_handle_flags','source_audit_ref','empty_observations','job_close','host'),
  'actual-old-native-transcript')
 need(lifetime['schema']=='original-job-native-lifetime-v1' and
   lifetime['run_nonce']==grant['original_run_nonce'] and
   lifetime['guard_sha256']==grant['original_guard_sha256'],'old-Job-transcript-source')
 creator=closed(lifetime['creator'],('pid','creation'),'actual-old-creator')
 for k in creator:integer(creator[k],1,2**64-1,'creator-'+k)
 query=closed(lifetime['creator_job_query'],('success','is_member','exact_job_handle','perf_ns'),
     'creator-exact-Job-query')
 need(query['success'] is True and query['is_member'] is False and
   query['exact_job_handle'] is True,'creator-actually-outside-exact-old-Job')
 flags=closed(lifetime['job_handle_flags'],('query_success','flags','perf_ns'),'actual-handle-flags')
 need(flags['query_success'] is True and type(flags['flags']) is int and
   flags['flags']>=0 and flags['flags']&1==0,'old-Job-handle-not-inheritable')
 audit=body_reference(lifetime['source_audit_ref'],65536,'closed-old-Job-owner-universe-audit')
 need(audit['guard_sha256']==grant['original_guard_sha256'] and
   audit['no_job_handle_duplication'] is True and audit['no_job_handle_inheritance'] is True and
   audit['complete_cooperating_owner_universe']==[creator],
   'reviewed-exact-source-one-Job-handle-owner')
 guard_text=read(config['bindings']['guard']['path'],2*1024**2).decode()
 need('DuplicateHandle(' not in guard_text and 'DUPLICATE_SAME_ACCESS' not in guard_text,
   'no-unreviewed-old-Job-duplication')
 empty=lifetime['empty_observations']
 need(type(empty) is list and len(empty)==2,'exact-two-numeric-native-empties')
 first=integer(empty[0]['perf_ns'],1,2**63-1,'first-empty')
 last=integer(empty[1]['perf_ns'],first+1,2**63-1,'second-empty')
 need(all(set(x)=={'perf_ns','members'} and x['members']==[] for x in empty),
   'both-actual-native-member-sets-empty')
 close=closed(lifetime['job_close'],('success','completed_perf_ns'),'actual-final-Job-CloseHandle')
 ended=integer(close['completed_perf_ns'],last,grant['issued_perf_ns'],'Job-last-handle-end')
 need(close['success'] is True and
   integer(query['perf_ns'],1,ended,'creator-query-before-close') and
   integer(flags['perf_ns'],1,ended,'flags-before-close'),'positive-old-last-handle-close')
 host=closed(lifetime['host'],('pid','creation','exit_observed_perf_ns','exit_code',
  'io_closed','writer_closed','callbacks_joined'),'actual-old-host-exit')
 need(host['pid']==config['identity']['host_pid'] and
   host['creation']==config['identity']['host_creation'] and type(host['exit_code']) is int and
   all(host[k] is True for k in ('io_closed','writer_closed','callbacks_joined')) and
   integer(host['exit_observed_perf_ns'],1,first,'host-exit-before-empties'),
   'original-host-reaped-IO-writer-callbacks-closed')
 need(completion['schema']=='parent-original-guard-completion-v1' and
   completion['run_nonce']==grant['original_run_nonce'] and
   completion['guard_sha256']==grant['original_guard_sha256'] and
   completion['creator']==creator and completion['guard_reaped'] is True and
   completion['guard_exit_code']==0 and completion['stdout_eof'] is True and
   completion['stderr_eof'] is True and completion['callbacks_joined'] is True and
   integer(completion['completed_perf_ns'],ended,grant['issued_perf_ns'],'guard-returned-before-grant'),
   'actual-parent-observed-original-guard-completion')
 return ended


def validate_parent_identity(held, module, expected, ended):
 closed(expected,('pid','creation','image','image_sha256'),'new-actual-parent')
 pid=os.getppid();need(pid==expected['pid'],'actual-new-native-parent-pid')
 handle=held.K.OpenProcess(0x100000|0x1000,False,pid)
 need(bool(handle),'new-parent-native-handle')
 try:
  need(held.creation(handle)==expected['creation'] and
    held.K.WaitForSingleObject(handle,0)==258,'live-new-parent-creation')
  count=module.w.DWORD(32768);buffer=module.c.create_unicode_buffer(count.value)
  need(held.K.QueryFullProcessImageNameW(handle,0,buffer,module.c.byref(count)),
    'native-parent-image-query')
  need(os.path.normcase(buffer.value)==os.path.normcase(expected['image']) and
    sha(read(expected['image'],64*1024**2))==expected['image_sha256'],
    'actual-new-parent-native-image-hash')
  need(time.perf_counter_ns()>ended,'new-finalizer-after-old-Job-lifetime')
 finally:
  need(bool(held.K.CloseHandle(handle)),'positive-parent-handle-close')


def validate_join_inputs(config):
 closed(config,('api','go','runtime_go','identity','bindings','limits','caps','plan',
  'phase_refs','native_capture','qpc_capture','guard_receipt','clock_bridge',
  'expanded_trace','census','output','current_finalizer_identity','parent_case_id'),'finalizer-config')
 need(config['api']==API and config['go'] is True and config['runtime_go'] is True,
   'separate-postexit-finalizer-GO')
 i=closed(config['identity'],('run','source_pin','binary_sha256','budget_sha256','guard_sha256',
  'driver_sha256','profile_sha256','host_pid','host_creation'),'original-run-identity')
 need(re.fullmatch('[0-9a-f]{32}',i['run']) and re.fullmatch('[0-9a-f]{40}',i['source_pin']) and
   all(hex64(i[k]) for k in ('binary_sha256','budget_sha256','guard_sha256',
    'driver_sha256','profile_sha256')),'complete-final-source-binding')
 integer(i['host_pid'],1,2**32-1,'original-host-pid')
 integer(i['host_creation'],1,2**64-1,'original-host-creation')
 own=config['current_finalizer_identity']
 closed(own,('pid','creation','parent_postexit_grant_ref'),'current-finalizer-identity')
 need(own['pid']==os.getpid(),'separately-admitted-finalizer')
 grant_ref=own['parent_postexit_grant_ref']
 parent_root=Path('C:/Users/spmcc/.codex/visualizations/2026/10/20/01a15ab8-d85f-71b1-8d37-c3c5689790be')
 need(no_links(grant_ref['path']).is_relative_to(parent_root) and
   grant_ref.get('reviewed') is True,'actual-parent-owned-grant-path')
 grant=body_reference({k:v for k,v in grant_ref.items() if k!='reviewed'},65536,'parent-postexit-grant')
 keys=('schema','parent_released','original_run_nonce','original_guard_sha256',
    'original_guard_receipt_ref','original_completion_ref','original_native_lifetime_ref',
    'phase_index_ref','expected_source_pin','expected_binary_sha256','expected_runtime_sha256',
    'expected_profile_sha256','expected_finalizer_sha256','expected_new_guard_sha256',
    'expected_finalizer_parent','issued_perf_ns','deadline_perf_ns')
 closed(grant,keys,'actual-parent-postexit-grant')
 need(grant['schema']=='postexit-finalizer-grant-v1' and grant['parent_released'] is True and
   grant['original_run_nonce']==i['run'] and grant['original_guard_sha256']==i['guard_sha256'] and
   all(grant['expected_'+k]==i[k] for k in ('source_pin','binary_sha256')) and
   grant['expected_profile_sha256']==config['bindings']['profiles']['sha256'] and
   grant['expected_runtime_sha256']==config['bindings']['runtime']['sha256'] and
   grant['expected_finalizer_sha256']==config['bindings']['finalizer']['sha256'] and
   grant['expected_new_guard_sha256']==config['bindings']['new_guard']['sha256'] and
   grant['original_guard_receipt_ref']==config['guard_receipt'],
   'parent-grant-exact-source-input-bindings')
 integer(grant['issued_perf_ns'],1,time.perf_counter_ns(),'parent-grant-issued')
 integer(time.perf_counter_ns(),grant['issued_perf_ns'],grant['deadline_perf_ns'],'parent-grant-deadline')
 lifetime=body_reference(grant['original_native_lifetime_ref'],65536,'old-native-Job-lifetime')
 completion=body_reference(grant['original_completion_ref'],65536,'old-parent-completion')
 index=body_reference(grant['phase_index_ref'],1024*1024,'exact16-parent-phase-index')
 need(index['phase_refs']==config['phase_refs'] and index['original_run_nonce']==i['run'] and
   index['source_pin']==i['source_pin'],'grant-index-is-exact-final16')
 ended=validate_native_lifetime(lifetime,completion,grant,config)
 owner_path=Path('C:/file-native-online-control-20261020/current-owner.json')
 current=strict(read(owner_path,8192),8192)
 need(current['state']=='running' and current['nonce']!=i['run'] and
   current['job_root_pid']==os.getpid() and
   config['bindings']['new_guard']['sha256']==grant['expected_new_guard_sha256'] and
   current['guard_sha256']==grant['expected_new_guard_sha256'] and
   current['pid']==grant['expected_finalizer_parent']['pid'] and
   current['creation_time']==grant['expected_finalizer_parent']['creation'] and
   type(current['acquired_perf_ns']) is int and
   current['acquired_perf_ns']>=grant['issued_perf_ns']>ended,
   'actual-new-guard-owner')
 module=source(config['bindings']['native'],'native_identity')
 held=module.Native(current)
 try:
  validate_parent_identity(held,module,grant['expected_finalizer_parent'],ended)
  actual=held.creation(held.stage)
  need(held.stage_live() and held.members()=={os.getpid()} and
    actual==current['job_root_creation_time']==own['creation'] and
    not(os.getpid()==i['host_pid'] and actual==i['host_creation']),
    'live-current-Job-creation-distinct-from-closed-original')
 finally:
  held.close()
 limit_keys=('native_bytes','native_line_bytes','native_rows','native_calls','per_call_stream_bytes',
  'env_bytes','rss_per_call','qpc_compressed_bytes','qpc_expanded_bytes','qpc_line_bytes',
  'qpc_rows','job_members','guard_json_bytes','phase_bytes','metadata_bytes')
 closed(config['limits'],limit_keys,'all-bounded-read-limits')
 for k,v in config['limits'].items():integer(v,1,2**40,'limit:'+k)
 need(config['limits']['native_calls']<=20000 and config['limits']['native_rows']<=2000000 and
   config['limits']['qpc_rows']<=2000000 and
   config['limits']['per_call_stream_bytes']<=16*1024**2 and
   config['limits']['native_line_bytes']<=65536 and
   config['limits']['qpc_line_bytes']<=65536 and config['limits']['phase_bytes']<=4*1024**2 and
   config['limits']['job_members']<=12,'bounded-parser-memory')
 closed(config['caps'],('reader_bytes','checker_bytes','aggregate_bytes','supervisor_bytes'),'role-caps')
 need(config['caps']['reader_bytes']==256*1024**2 and config['caps']['checker_bytes']==2*1024**3 and
   config['caps']['aggregate_bytes']<=4*1024**3 and
   config['caps']['supervisor_bytes']<=768*1024**2,'unchanged-role-resource-limits')
 for name,row in config['bindings'].items():
  file_binding(row,64*1024**2 if name in ('python','engine') else 2*1024**2,'transitive-source:'+name)
 need(config['bindings']['guard']['sha256']==i['guard_sha256'],'actual-guard-source')
 need(no_links(__file__)==Path(config['bindings']['finalizer']['path']) and
   sha(read(__file__,65536))==config['bindings']['finalizer']['sha256'],'own-finalizer-source')
 return config


def join_history(config):
 validate_join_inputs(config)
 history=source(config['bindings']['history'],'history')
 plan=config['plan']
 need(plan['total_accepted']==1600 and plan['max_offers']==4000 and
   plan['max_sealed_losers']==2400 and len(plan['phases'])==16,'whole-history-plan')
 native=raw_capture(config)
 guard=scan_guard(config,native)
 phase_results,counts=_phase_windows(plan,config['phase_refs'],config,native,guard,history)
 trace=body_reference(config['expanded_trace'],config['limits']['metadata_bytes'],'source-expanded-trace')
 need(trace['profile_sha256']==config['bindings']['profiles']['sha256'] and
   trace['source_pin']==config['identity']['source_pin'],'same-source-operation-schedule')
 schedule_counts=reconstruct_whole_schedule(trace,native,config,native['case_index_refs'])
 before=census(config)
 return dict(api=API,status='QUALIFIED_ORIGINAL_HISTORY_JOIN',source_pin=config['identity']['source_pin'],
  original_run=config['identity'],source_bindings=config['bindings'],
  original_guard_receipt=config['guard_receipt'],native_capture=config['native_capture'],
  qpc_capture=config['qpc_capture'],phase_refs=config['phase_refs'],phases=phase_results,
  actual_phase_action_counts=counts,whole_native_operation_counts=schedule_counts,source_schedule=config['expanded_trace'],
  QPC=dict(samples=guard['samples'],first_ns=guard['first_ns'],last_ns=guard['last_ns'],
     host_exit_ns=guard['host_exit_ns'],job_empty_ns=guard['job_empty_ns']),
  two_empty_original_Job_observations=guard['guard']['closure_checks'],
  census_before_output=before,
  remaining_finalization_reservation=config['output'],
  qualification='original16-phasehistoryonly; parent must close this separately admitted finalizer run; other matrix cells separate')


def write_result(config):
 result=join_history(config)
 output=closed(config['output'],('path','category','max_bytes','reserved_files'),'bounded-finalizer-output')
 need(output['reserved_files']==1 and 0<output['max_bytes']<=1024*1024,'one-final-output-reservation')
 raw=encode_private_record(result)
 need(len(raw)<=output['max_bytes'],'result-prewrite-bound')
 root=Path(config['census']['roots'][output['category']]['root'])
 path=no_links(output['path'])
 need(path.is_relative_to(root) and path.parent.is_dir() and not path.exists(),
   'fresh-owned-finalizer-output')
 reserve=config['census']['roots'][output['category']]['remaining_reservation']
 need(reserve['bytes']>=output['max_bytes'] and reserve['files']>=1,'preheld-output-reserve')
 with path.open('xb') as f:
  need(f.write(raw)==len(raw),'finalizer-short-write');f.flush();os.fsync(f.fileno())
 after=census(config,consumed_output=output)
 return dict(output=dict(path=str(path),bytes=len(raw),sha256=sha(raw)),
    after_output_census=after,original_history_qualified=True,
    finalizer_outer_closure_pending=True)


if __name__=='__main__':
 import argparse
 parser=argparse.ArgumentParser()
 parser.add_argument('config')
 args=parser.parse_args()
 try:
  config=strict(read(args.config,1024*1024),1024*1024)
  print(json.dumps(write_result(config),sort_keys=True,separators=(',',':'),ensure_ascii=True))
 except BaseException as error:
  import sys
  sys.stderr.buffer.write(ascii(error).encode('ascii')[:2000]+b'\n')
  sys.stderr.buffer.flush()
  sys.exit(1)

