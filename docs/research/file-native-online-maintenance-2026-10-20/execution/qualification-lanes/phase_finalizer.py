
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
          integer, need, no_links, read, sha, strict, METRIC_FIELDS_027,
          source_literal, wire_resolve, schema_width)

API='history-postexit-finalizer-v2-r3'
QPC_V2='qpc-ns-v2; whole-Job tail coverage; strict >40ms; coarse monotonic retained; no retrospective requalification'
PROVISIONAL='PROVISIONAL_UNQUALIFIED'
PHASE_BODY_KEYS={'schema','status','phase_id','plan_sha256','source_pin','binary_sha256',
 'budget_sha256','outer_run_id','source_profile_sha256','evidence','summary',
 'owned_closure_ref','checkpoint_ref','case_closure_ref','runtime_sha256','guard_sha256',
 'finalizer_sha256','native_call_index_ref'}



def wire_contract(config):
 wire=config.get('_wire') or source_literal(config['bindings']['evidence'],'WIRE_CONTRACT')
 need(wire['api']=='windows-capture-v2-r3','actual-R3-capture-contract')
 return wire

def wire_check(value,node,wire,depth=0):
 need(depth<=32 and type(node) is dict and len(node)==1,'wire-validation-expression')
 kind,bound=next(iter(node.items()))
 if kind=='named':return wire_check(value,wire['types'][bound],wire,depth+1)
 if kind=='literal':need(type(value) is type(bound) and value==bound,'source-literal');return
 if kind=='oneof':
  for choice in bound:
   try:wire_check(value,choice,wire,depth+1);return
   except (RuntimeError,ValueError,TypeError):pass
  raise RuntimeError('source-union')
 if kind=='object':
  closed(value,bound,'source-owned-object')
  for k,v in bound.items():wire_check(value[k],v,wire,depth+1)
  return
 def quantity(n):
  return wire['parameters'][n['parameter']][1] if type(n) is dict else n
 if kind=='array':
  need(type(value) is list and len(value)<=quantity(bound['count']),'source-array-count')
  for v in value:wire_check(v,bound['item'],wire,depth+1)
 elif kind=='uint':integer(value,0,(1<<quantity(bound))-1,'source-uint')
 elif kind in ('string','ascii'):
  need(type(value) is str and len(value.encode())<=quantity(bound) and
   (kind!='ascii' or value.isascii()),'source-string')
 elif kind=='bytes':
  need(len(canonical(value))<=quantity(bound),'source-encoded-fragment')
 elif kind=='float64':
  need(type(value) in (int,float) and math.isfinite(value),'source-float')
 else:raise RuntimeError('unknown-source-type')

def wire_row(row,name,config):
 wire=wire_contract(config)
 # DATA's empty literal is only its source-owned WIDTH baseline.
 node=wire['record_schemas'][name]
 if name=='data':
  node={'object':dict(node['object'],b64={'ascii':4*((16384+2)//3)})}
 wire_check(row,node,wire)
 return row

def range_key(ref):
 closed(ref,('path','offset','bytes','sha256'),'actual-tape-range')
 return (str(no_links(ref['path'])),integer(ref['offset'],0,2**63-1,'range-offset'),
  integer(ref['bytes'],1,4259840,'range-bytes'),ref['sha256'])

def retained_row(ref,native,config,name=None):
 key=range_key(ref);known=native['records'].get(key[1])
 need(Path(key[0])==Path(config['native_capture']['path']) and known==ref,
  'reference-is-exact-verified-physical-row')
 row=body_reference(ref,config['limits']['native_line_bytes'],'retained-tape-row')
 if name:wire_row(row,name,config)
 return row

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


def body_reference(row, limit, label, *, canonical_record=True):
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
 if canonical_record:need(raw==encode_private_record(value),'canonical-private-record')
 return value


def guard_encoding(value):
 return json.dumps(value,ensure_ascii=True,separators=(',',':'),allow_nan=False).encode('utf-8')+b'\n'


def parent_reference(row,limit,label):
 return body_reference(row,limit,label,canonical_record=False)


def _lines(path, maximum_bytes, maximum_line):
 with no_links(path).open('rb') as f:
  before=os.fstat(f.fileno());total=offset=0
  while True:
   line=f.readline(maximum_line+1)
   if not line:break
   need(line.endswith(b'\n') and len(line)<=maximum_line,'complete-bounded-line')
   total+=len(line);need(total<=maximum_bytes,'stream-byte-bound')
   yield offset,line
   offset=total
  after=os.fstat(f.fileno())
  need((before.st_dev,before.st_ino,before.st_size,before.st_mtime_ns)==
   (after.st_dev,after.st_ino,after.st_size,after.st_mtime_ns),'same-handle-stream-unchanged')


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



def validate_chronology(begin,start,samples,end):
 first=integer(begin['qpc_start'],1,2**64-1,'BEGIN-clock')
 opened=integer(start['qpc'],first,2**64-1,'START-clock')
 closed_at=integer(end['qpc_end'],opened,2**64-1,'END-clock')
 previous=first
 for sample in samples:
  tick=integer(sample['qpc'],previous,closed_at,'RSS-clock');previous=tick
  need((sample['phase']=='suspended' and tick<=opened) or
   (sample['phase']=='running' and tick>=opened),'RSS-phase-clock')
 if end['in_n']:
  integer(end['first_write_qpc'],opened,closed_at,'first-completed-write-clock')
 else:need(end['first_write_qpc'] is None,'empty-input-has-no-write-clock')

def raw_capture(config):
 
 limit=config['limits'];ref=config['native_capture']
 path=file_binding(ref,limit['native_bytes'],'native-capture')
 calls={};header=None;line_count=0;total_frames=0;records={};sidebands=[];bridges=[];metas={};closures={};whole=hashlib.sha256()
 wire=wire_contract(config)
 fields={k:set(v['object']) for k,v in wire['record_schemas'].items()}
 held_stat=path.stat()
 for offset,line in _lines(path,ref['bytes'],limit['native_line_bytes']):
  line_count+=1
  need(line_count<=limit['native_rows'],'native-row-bound')
  row=strict(line,limit['native_line_bytes'])
  whole.update(line)
  need(type(row) is dict and line==encode_private_record(row),'native-canonical-record')
  physical=dict(path=str(path),offset=offset,bytes=len(line),sha256=sha(line))
  if row.get('kind') not in ('data','rss'):records[offset]=physical
  kind=row.get('kind')
  if kind not in ('stream',*wire['invariants']['frame_members']):
   need(header is not None,'header-before-sideband')
   name={'native-qpc-bridge-v1':'bridge','native-call-meta-v2':'native_meta',
    'capture-sideband-v1':'sideband','native-case-proof-v1':'case_proof',
    'native-case-index-page-v1':'case_index_page','native-case-index-v1':'case_index'}.get(row.get('schema'))
   need(name is not None,'no-unknown-mixed-tape-record')
   wire_row(row,name,config)
   if 'run' in row:need(row['run']==header['run'],'all-physical-rows-same-run')
   if name=='bridge':bridges.append((row,physical))
   if name=='native_meta':
    need(row['call'] not in metas,'one-meta-per-call');metas[row['call']]=(row,physical)
   if name=='sideband':
    bounds=config['_catalogue']['profiles']['capture/sidebands']['sideband_bounds']
    need(row['sideband_kind'] in bounds and len(canonical(row['payload']))<=bounds[row['sideband_kind']] and
     row['payload'].get('kind')==row['sideband_kind'],'actual-source-sideband-kind-and-width')
    need(sha(canonical(row['payload']))==row['payload_sha256'],'sideband-payload-digest')
    if row['sideband_kind']=='invocation-closure':
     token=row['payload']['call'];need(token not in closures,'one-invocation-closure');closures[token]=physical
    sidebands.append(({k:v for k,v in row.items() if k!='payload'},physical))
   continue
  wire_row(row,kind,config)
  if kind=='stream':
   need(header is None and offset==0,'one-first-capture-header')
   header=row;header_ref=physical
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
   need(row['input_scope']=='planned' and type(row['seq']) is int and row['seq']==0,'planned-BEGIN-seq0')
   if row['pid'] is not None:integer(row['pid'],1,2**32-1,'call-pid')
   if row['creation'] is not None:integer(row['creation'],1,2**64-1,'call-creation')
   integer(row['qpc_start'],1,2**64-1,'call-start')
   need((row['env'] is None or type(row['env']) is dict and all(type(k) is str and type(v) is str
    and '\\0' not in k+v for k,v in row['env'].items())) and len(canonical(row['env']))<=limit['env_bytes'],
     'bounded-exact-environment')
   calls[token]=dict(begin=row,launch=None,start=None,rss_identity=None,seq_next=0,planned=bytearray(),
    begin_offset=offset,end_offset=None,frames=0,bytes=0,partials=set(),
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
     hex64(row['executable_sha256']) and row['producer_kind'] in ('candidate','historical','python-argv','native-argv') and
     re.fullmatch('[0-9a-f]{40}',row['producer_source_pin']) and
     (row['startup_gate_sha256'] is None or hex64(row['startup_gate_sha256'])),
     'closed-authentic-launch-types')
   need(type(row['source_files']) is list and 0<len(row['source_files'])<=64,'bounded-launch-sources')
   for item in row['source_files']:
    closed(item,('path','sha256'),'launched-source')
    need(Path(item['path']).is_absolute() and hex64(item['sha256']),'launched-source-reference')
   c['launch']=row;c['launch_ref']=physical
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
   c['start']=row;c['start_ref']=physical
  elif kind=='data':
   need(row['stream'] in c['streams'] and hex64(row['sha']) and type(row['b64']) is str,
     'typed-data-frame')
   need(row['stream']=='in' or c['start'] is not None,'START-before-OUT-ERR')
   raw=base64.b64decode(row['b64'],validate=True)
   need(base64.b64encode(raw).decode()==row['b64'] and
     len(raw)==row['n'] and 0<len(raw)<=16384 and sha(raw)==row['sha'] and
     row['offset']==c['sizes'][row['stream']],'exact-lossless-data-frame')
   need(row['stream'] not in c['partials'],'one-final-coalesced-partial')
   if len(raw)<16384:c['partials'].add(row['stream'])
   c['streams'][row['stream']].update(raw);c['sizes'][row['stream']]+=len(raw)
   if row['stream']=='in':c['planned'].extend(raw)
   if row['stream']=='out':
    c['out_buffer'].extend(raw);c['out_frames'].append((offset,len(line),sha(line)))
   need(c['sizes'][row['stream']]<=limit['per_call_stream_bytes'],'native-stream-bound')
  elif kind=='rss':
   need(c['launch'] is not None and
     (c['start'] is None or (row['pid'],row['creation'])==(c['start']['pid'],c['start']['creation'])) and
     c['rss_identity'] in (None,(row['pid'],row['creation'])) and row['role']==c['begin']['role'] and row['phase'] in ('suspended','running'),
     'RSS-native-identity')
   need(row['qpc']>=c['begin']['qpc_start'] and
    ((row['phase']=='suspended' and c['start'] is None) or
     (row['phase']=='running' and c['start'] is not None and row['qpc']>=c['start']['qpc'])),
    'suspended-before-START-running-after-START')
   c['rss_identity']=(row['pid'],row['creation'])
   for key in ('qpc','rss','peak'):integer(row[key],1,2**64-1,'RSS-'+key)
   role_limit=config['caps']['reader_bytes'] if row['role']=='reader' else config['caps']['checker_bytes']
   need(row['rss']<=role_limit and row['peak']<=role_limit,'role-RSS-cap')
   need(not c['rss'] or row['qpc']>=c['rss'][-1]['qpc'],'ordered-RSS')
   c['rss'].append(dict(qpc=row['qpc'],rss=row['rss'],peak=row['peak'],phase=row['phase']))
   need(len(c['rss'])<=limit['rss_per_call'],'finite-native-RSS')
  elif kind=='end':
   need(c['start'] is not None,'no-START-constructor-prefix-is-not-qualified')
   need(row['exit'] in (0,2) and row['violations']==[] and row['intent'] is None,
     'no-hidden-native-censor')
   need(all(row[k] is True for k in ('input_complete','out_eof','err_eof',
     'native_closed','io_closed','callbacks_joined')),'native-actual-closure')
   need(row['qpc_end']>=c['start']['qpc'] and all(x['qpc']<=row['qpc_end'] for x in c['rss']) and row['rss_n']==len(c['rss']) and
     c['rss'],'native-complete-lifetime')
   for stream,field in (('in','in'),('out','out'),('err','err')):
    need(row[field+'_n']==c['sizes'][stream] and
      row[field+'_sha']==c['streams'][stream].hexdigest(),'native-final-stream-digest')
   need(len(c['planned'])==c['begin']['input_n'] and
     sha(bytes(c['planned']))==c['begin']['input_sha'],'planned-input-artifact')
   actual=integer(row['in_n'],0,len(c['planned']),'actual-completed-prefix')
   need(row['in_sha']==sha(bytes(c['planned'][:actual])) and
     row['input_complete']==(actual==len(c['planned'])),'actual-completion-prefix')
   validate_chronology(c['begin'],c['start'],c['rss'],row)
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
   c['end']=row;c['end_ref']=physical;c['end_offset']=offset+len(line)
 need(header is not None and calls and all(c['end'] is not None for c in calls.values()),
   'complete-closed-native-file')
 result={}
 for token,c in calls.items():
  effective=dict(c['begin'],pid=c['start']['pid'],creation=c['start']['creation'])
  result[token]=dict(begin=effective,launch=c['launch'],input=c['input'],start=c['start'],end=c['end'],rss=c['rss'],
   out_frames=c['out_frames'],response_sha256=c['response_sha256'],
   start_ref=c['start_ref'],end_ref=c['end_ref'],launch_ref=c['launch_ref'],
   ref=dict(schema='single-raw-ref-v1',path=str(path),run=header['run'],call=token,
    begin=c['begin_offset'],end=c['end_offset'],frames=c['frames'],
    bytes=c['bytes'],sha256=c['digest'].hexdigest()))
 need(whole.hexdigest()==ref['sha256'] and held_stat==path.stat(),'whole-mixed-tape-unchanged-digest')
 need(len(bridges)==1 and len(metas)==len(calls) and set(metas)==set(calls),'complete-mixed-call-metadata')
 need(bridges[0][0]==config['clock_bridge'],'one-actual-flat-clock-bridge')
 return dict(header=header,header_ref=header_ref,calls=result,frames=total_frames,rows=line_count,
    records=records,sidebands=sidebands,bridges=bridges,metas=metas,closure_refs=closures,
    file_sha256=ref['sha256'],file_bytes=ref['bytes'])


def rehydrate(value, native, config, *, depth=0, state=None):
 
 if state is None:state={'nodes':0,'responses':{}}
 state['nodes']+=1
 need(depth<=64 and state['nodes']<=250000,'phase-rehydration-bound')
 if type(value) is dict and value.get('schema')=='physical-state-ref-v1':
  wire_row(value,'physical_state_ref',config)
  need(value['source_proof_ref']==native['header_ref'] and value['run']==config['identity']['run'],
   'physical-state-source-header')
  row=retained_row(value['record_ref'],native,config,'sideband')
  need(row['sideband_kind']=='selected-state' and
   all(row[k]==value[k] for k in ('run','case_id','scope_id','case_scope')) and
   type(row['payload'].get('state')) is dict and
   row['payload']['state'].get('schema')!='physical-state-ref-v1' and
   sha(canonical(row['payload']['state']))==value['state_sha256'],
   'actual-full-state-bytes-and-scope')
  return row['payload']['state']
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


def clock_interval(bridge,qpc):
 closed(bridge,('schema','run','frequency','qpc_before','perf_ns','qpc_after'),'actual-flat-clock-bridge')
 need(bridge['schema']=='native-qpc-bridge-v1','source-flat-native-clock')
 f=integer(bridge['frequency'],1,2**63-1,'QPC-frequency')
 before=integer(bridge['qpc_before'],0,2**64-1,'QPC-before')
 after=integer(bridge['qpc_after'],before,2**64-1,'QPC-after')
 perf=integer(bridge['perf_ns'],1,2**63-1,'perf-bracket')
 integer(qpc,0,2**64-1,'mapped-QPC')
 return (perf+((qpc-after)*10**9)//f,perf+((qpc-before)*10**9+f-1)//f)


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
 contract=source_literal(config['bindings']['guard'],'GUARD_WIRE_CONTRACT')
 need(value['sampling_revision']==contract['sampling_revision'],'actual-source-owned-guard-revision')
 need(value['bounds_sha256']==sha(guard_encoding(config['_guard_bounds'])),
   'actual-guard-bounds-bytes')
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
 if True:
  need(all(type(x.get('perf_counter_ns')) is int and
     x['perf_counter_ns']>timing['root_wait_returned_ns']
     for x in value['closure_checks']),'new-guard-numeric-postexit-empties')
 return value


def scan_guard(config, native):
 guard=_guard_file(config);timing=guard['memory_sample_timing'];identity=config['identity']
 need(Path(guard['raw_rss'])==Path(config['qpc_capture']['path']),'exact-QPC-stream-path')
 source_ref=config['bindings']['guard'];file_binding(source_ref,2*1024**2,'guard-program')
 contract=source_literal(config['bindings']['guard'],'GUARD_WIRE_CONTRACT')
 last_tick=last_complete=None;count=0;first=None;last=None;root_end=None;job_end=None
 peaks={};observations=[];identity_rows={};prefix=hashlib.sha256();prefix_hits={};last_written=None;awaiting=None
 written={};tails={}
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
  need(row['run_nonce']==identity['run'] and event in contract['rows'],'actual-source-guard-row-and-run')
  if 'required' in contract['rows'][event]:
   closed(row,contract['rows'][event]['required'],'actual-guard-row-fields')
  if event=='rss-sample':
   need(row.get('sample_complete') is True and awaiting is None,'one-positive-full-QPC-sample-before-write-completion')
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
   supervisors=row['supervisors'];sup_total=0;sup_seen=set()
   need(type(supervisors) is list and len(supervisors)<=contract['dimensions']['registered_supervisors'],
    'source-bounded-supervisors')
   for process in supervisors:
    key=(integer(process['pid'],1,2**32-1,'supervisor-pid'),
     integer(process['creation_time'],1,2**64-1,'supervisor-creation'))
    need(key not in sup_seen,'unique-supervisor-identity');sup_seen.add(key)
    if set(process)==set(contract['rows']['rss-sample']['supervisor_live']):
     sup_total+=integer(process['rss'],1,config['caps']['supervisor_bytes'],'supervisor-rss')
     integer(process['peak'],process['rss'],config['caps']['supervisor_bytes'],'supervisor-peak')
    else:
     closed(process,contract['rows']['rss-sample']['supervisor_departed'],'actual-supervisor-departure')
     need(process['state'] in contract['rows']['rss-sample']['supervisor_departure_states'],
      'source-owned-departure-state')
   need(sup_total==row['supervisor_rss'],'actual-summed-supervisor-RSS')
   need(row['sample_id']==count+1,'strict-guard-sample-number')
   awaiting=(row['sample_id'],tick,done);sample_offset=offset
   if first is None:first=tick
   last=done;last_tick=tick;last_complete=done;count+=1
   observations.append((tick,done,host))
   if prefix.hexdigest() in wanted:prefix_hits[prefix.hexdigest()]=done
   need(len(observations)<=config['limits']['qpc_rows'],'bounded-QPC-index')
  elif event=='rss-write-complete':
   need(awaiting==(row['sample_id'],row['perf_counter_ns'],row['completed_perf_counter_ns']),
    'actual-corresponding-RSS-write')
   now=integer(row['recorded_perf_counter_ns'],awaiting[2],2**63-1,'actual-write-complete')
   need(now-awaiting[1]<=40000000 and
    (last_written is None or 0<now-last_written<=40000000) and
    0<=row['duration_seconds']<=.04,'complete-recorded-cadence')
   written[now]=dict(sample_id=awaiting[0],start_ns=awaiting[1],completed_ns=awaiting[2],
    sample_uncompressed_offset=sample_offset,write_uncompressed_offset=offset)
   last_written=now;awaiting=None
  elif event in ('root-end-sampling-coverage','job-drain-sampling-coverage'):
   need(event not in tails and row['sample_complete'] is False,'one-actual-endpoint-event')
   end=integer(row['perf_counter_ns'],1,2**63-1,'endpoint-perf')
   previous=integer(row['previous_recorded_perf_counter_ns'],1,end,'endpoint-predecessor')
   gap=end-previous
   need(previous in written and gap<=40000000 and row['interval_seconds']==gap/10**9 and
    timing[event]==dict(perf_counter_ns=end,gap_ns=gap,raw_recorded=True),'actual-completed-predecessor-and-coverage')
   tails[event]=dict(endpoint_ns=end,write_completed_ns=previous,gap_ns=gap,
    coverage_uncompressed_offset=offset,**written[previous])
   if event=='root-end-sampling-coverage':root_end=end
   else:
    need(row['empty_observed'] is True and row['members']==[],'actual-Job-empty-endpoint')
    job_end=end
  elif event in ('disk-budget','pid-exit-race'):
   closed(row,('event','sample_complete','value','run_nonce'),'source-guard-auxiliary-row')
   need(row['sample_complete'] is False and type(row['value']) is dict,'actual-guard-side-observation')
   if event=='pid-exit-race':
    need(row['value'] in guard['pid_exit_races'],'retained-actual-native-exit-observation')
   else:
    need(row['value'] in guard['disk_samples'],'retained-actual-budget-observation')
  else:
   raise RuntimeError('late/missing/unsupported-QPC-event:'+str(event))
 need(awaiting is None and last_written is not None,'last-RSS-record-fully-written')
 need(first and last and root_end and job_end and first<=root_end<=job_end and
   count==len(written)==timing['completed_samples']==timing['samples']==timing['recorded_write_markers'] and
   first<=timing['first_positive_full_sample_ns']<=first+40000000 and
   root_end==timing['root_wait_returned_ns'] and job_end==timing['job_empty_observed_ns'],
   'whole-original-run-QPC-coverage')
 return dict(raw_sha256=config['qpc_capture']['sha256'],samples=count,
  first_ns=first,last_ns=last,host_exit_ns=root_end,job_empty_ns=job_end,
  endpoint_coverage=tails,observations=observations,prefix_hits=prefix_hits,peaks=peaks,owner=guard['owner'],guard=guard)



def actual_response(call, native, config):
 return rehydrate({'schema':'native-response-ref-v1','raw_ref':call['ref']},native,config)


def checker_binding(call,config):
 if 'checker_request' in call:return
 native=config['_native'];token=call['ref']['call'];meta=native['metas'][token][0]
 closure=retained_row(native['closure_refs'][token],native,config,'sideband')['payload']
 need(closure['raw_ref']==call['ref'] and closure['resources_closed'] is True and
  closure['callbacks_joined'] is True,'actual-checker-call-closure')
 rr,ir=closure['request_ref'],closure['output_ref'];argv=call['launch']['argv']
 need(len(argv)==10 and argv[1:5]==['-I','-S','-B','-u'] and
  Path(argv[0])==Path(config['bindings']['python']['path']) and
  Path(argv[5])==Path(config['bindings']['oracle_cli']['path']) and
  argv[6]=='--request' and argv[8]=='--output' and
  Path(argv[7])==Path(rr['path']) and Path(argv[9])==Path(ir['path']) and
  rr['offset']==ir['offset']==0,'actual-isolated-checker-request-and-output')
 contract=source_literal(config['bindings']['oracle_cli'],'CONTRACT')
 request=parent_reference(rr,contract['request_carrier_max_bytes'],'original-checker-request')
 mode=request['mode'];need(mode in ('full','history_oracle','mapping','backlog'),'HISTORY-checker-mode')
 spec=contract['mode_fields'][mode]
 if type(spec) is str:spec=contract['mode_fields'][spec]
 need(set(contract['request_required'])|set(spec[0])<=set(request)<=
  set(contract['request_required'])|set(spec[0])|set(spec[1])|{'sources'},'actual-checker-request-fields')
 need(type(request['version']) is int and request['version']==contract['version'] and
  request['root']==call['begin']['root'] and request['quiescent'] is (mode!='mapping') and
  all(request[k]==meta[k] for k in ('case_id','scope_id','case_scope')),'actual-request-root-and-scope')
 limits=closed(request['limits'],contract['limits_required'],'captured-checker-limits')
 for v in limits.values():integer(v,1,2**63-1,'checker-cap')
 need(rr['bytes']<=limits['max_request_bytes'] and
  2*limits['proof_bytes']+limits['payload_bytes']+limits['error_bytes']+16384<=limits['total_output_bytes'],
  'actual-whole-output-admission')
 index=parent_reference(ir,16384,'actual-checker-index')
 need(index['version']==1 and type(index['version']) is int and index['mode']==mode and
  index['engine_pin']==config['identity']['source_pin'] and index['status']=='passed' and index['error'] is None and
  all(index[k]==meta[k] for k in ('case_id','scope_id','case_scope')),'authentic-completed-index')
 stdout=actual_response(call,native,config)
 need(stdout==dict(index=ir['path'],sha256=ir['sha256'],status='passed'),'native-stdout-to-actual-index')
 for path,row in index['sources'].items():
  need(config['_frozen_files'].get(os.path.normcase(str(no_links(path))))==row,'actual-reviewed-checker-source')
 audit=index['index_open_audit'];proof=index['proof']
 need(audit['spawn_events']==0 and audit['unexpected_modules']==[] and
  proof['source_unchanged'] is True and proof['before']['complete'] is True and
  proof['after']['complete'] is True and proof['before']['state_sha256']==proof['after']['state_sha256'],
  'complete-source-authenticated-readonly-proof')
 roles={};total=ir['bytes'];group=0
 for row in index['artifacts']:
  kind=row['role'];need(kind in contract['artifacts'] and kind!='partial_suffix' and kind not in roles and
   row['complete'] is True and Path(row['path'])==Path(ir['path']).with_suffix(contract['artifacts'][kind]),
   'all-original-companions-no-partial-or-foreign-prefix')
  ref={k:row[k] for k in ('path','bytes','sha256')};ref['offset']=0
  maximum=limits['proof_bytes'] if kind in ('before','after') else limits['error_bytes'] if kind=='error' else limits['payload_bytes']
  file_binding({k:ref[k] for k in ('path','bytes','sha256')},maximum,'checker-companion')
  total+=ref['bytes'];group+=ref['bytes'] if kind not in ('before','after','error') else 0;roles[kind]=ref
 need({'before','after','payload'}<=set(roles) and 'error' not in roles and
  total<=limits['total_output_bytes'] and group<=limits['payload_bytes'],'complete-bounded-artifact-group')
 payload=parent_reference(roles['payload'],min(limits['normalized_reserve_bytes'],64*1024**2),'checker-payload')
 need(payload['status']=='passed' and payload['final_pin']==config['identity']['source_pin'] and
  payload['engine_code_imported'] is False and payload['engine_audit_used'] is False and
  payload['source_unchanged'] is True and payload['quiescent'] is request['quiescent'],'actual-independent-result')
 thin=dict(index_ref=ir,payload_ref=roles['payload'],request_ref=rr)
 if mode in ('full','history_oracle'):
  need({'result','progress'}<=set(roles) and payload['all_files_directories_hashed'] is True,'full-allpath-result')
  full=parent_reference(roles['result'],min(limits['payload_bytes'],64*1024**2),'full-verifier-result')
  semantic,intents,r1=full['semantic'],full['request_intents'],full['mapped_cohort_closure']
  H=integer(semantic['publications'],1,2000000,'actual-full-H')
  known=parent_reference(request['known_requests_ref'],min(limits['max_known_request_bytes'],64*1024**2),'exact-known-requests')
  need(type(known) is dict and len(known)==H and all(type(v) is dict and v['request']==k for k,v in known.items()) and
   intents['publications']==intents['actual_H']==r1['accepted_publications']==H and
   semantic['selected']==intents['selected'] and intents['engine_code_imported'] is False and
   r1['profile']=='every-mapped-key-to-authenticated-accepted-role-owner-v1' and
   r1['captured_selected_authority'] is True and r1['candidate_context'] is False and
   full['navigation']['immutable_discovery_semantics_verified'] is True and
   full['schema_pin']==config['identity']['source_pin'] and
   full['companion_sha256']==source_literal(config['bindings']['oracle'],'COMPANIONS'),
   'actual-complete-H-intents-graph-navigation-and-mapped-cohort')
  if full['quota_identity_manifest'] is not None:
   need('identities' in roles and Path(full['quota_identity_manifest'])==Path(roles['identities']['path']),
    'actual-full-quota-identity-companion')
  scope=meta['scope_id'];phases=[p for p in config['plan']['phases'] if p['id']==scope]
  if phases:expected=phases[0]['H_end'];labels={scope+'-full'}
  else:
   donors=[d for d in config['plan']['donors'].values() if d['root']==request['root']]
   need(len(donors)==1,'exact-oracle-donor');donor=donors[0]
   if scope=='lookup-'+donor['id']:
    expected=donor['actual_H'];labels={scope+'-full',scope+'-legacy-internal-full',scope+'-paged-internal-full'}
   else:
    ps=[p for p in config['plan']['phases'] if p['donor']==donor['id'] and p['cycle']==1 and
     scope=='bootstrap-'+donor['id']+'-p'+str(p['publishers'])]
    need(len(ps)==1,'actual-bootstrap-oracle');expected=ps[0]['H_start'];labels={scope+'-reopen-full'}
  need(H==expected and request['checkpoint'] in labels,'actual-full-H-and-source-checkpoint')
  thin.update(actual_H=H,selected=semantic['selected'],fullproof_ref=roles['result'])
 elif mode=='backlog':
  need(payload['complete_candidate_coverage'] is True and payload['independent'] is True and
   sum(integer(payload[k],0,2**63-1,'actual-path-partition') for k in ('unprotected','protected','excluded'))==
    payload['existing_paths'] and 'backlog_paths' in roles,'complete-existing-path-backlog')
  thin.update({k:payload[k] for k in ('unprotected','protected','excluded','source_H')})
 elif mode=='mapping':
  entry=payload['entry']
  need(payload['independent'] is True and type(payload['mapped']) is bool and
   ((payload['mapped'] and type(entry) is list and len(entry)==5 and hex64(entry[4])) or
    (not payload['mapped'] and entry is None)),'actual-selected-index-mapping')
 call['checker_request']=request;call['checker_result']=thin

def oracle_receipts(native,config):
 by_index={};by_receipt={}
 for header,ref in native['sidebands']:
  if header['sideband_kind']!='independent-oracle':continue
  proof=retained_row(ref,native,config,'sideband')['payload']['proof'];key=proof['index_ref']['sha256']
  need(key not in by_index,'one-native-full-oracle-receipt');by_index[key]=(proof,ref)
 full=[c for c in native['calls'].values() if c['launch']['producer_kind']=='python-argv' and
  c['checker_request']['mode'] in ('full','history_oracle')]
 need(len(full)==len(by_index)==26,'all26-full-native-proof-receipts')
 for call in full:
  checked=call['checker_result'];proof,ref=by_index.pop(checked['index_ref']['sha256'])
  need(proof['index_ref']==checked['index_ref'] and proof['native_raw']['raw']==call['ref'] and
   proof['actual_H']==checked['actual_H'] and proof['history_oracle']['actual_H']==checked['actual_H'] and
   proof['artifact_refs']['result']==checked['fullproof_ref'],'actual-full-result-receipt-bijection')
  by_receipt[ref['sha256']]=proof['history_oracle']
 config['_oracle_receipts']=by_receipt

def native_meta(ref,call,config,case_id):
 native=config['_native']
 m=retained_row(ref,native,config,'native_meta')
 need(native['metas'][call['ref']['call']]==(m,ref),'actual-retained-call-meta')
 launch=call['launch'];start=call['start'];end=call['end'];identity=config['identity']
 need(m['schema']=='native-call-meta-v2' and m['call']==call['ref']['call'] and
  m['run']==identity['run'] and m['raw_ref']==call['ref'] and
  m['case_id']==config['parent_case_id'] and m['scope_id']==case_id and
  re.fullmatch('[0-9a-f]{32}',m['case_scope']) and m['source_pin']==identity['source_pin'] and
  m['producer_kind']==launch['producer_kind'] and m['producer_source_pin']==launch['producer_source_pin'] and
  m['binary_sha256']==start['executable_sha256']==launch['executable_sha256'] and
  m['identity']=={k:start[k] for k in ('pid','creation','executable_sha256')},
  'native-launch-start-source-and-scope')
 need(m['source_proof_ref']==native['header_ref'] and m['image_proof_ref']==call['start_ref'] and
  m['closure_ref']==call['end_ref'],'actual-source-header-START-END-proof-domains')
 need(retained_row(m['source_proof_ref'],native,config,'stream')==native['header'] and
  retained_row(m['image_proof_ref'],native,config,'start')==start and
  retained_row(m['closure_ref'],native,config,'end')==end,'actual-held-producer-evidence')
 bridge=retained_row(m['clock_bridge_ref'],native,config,'bridge')
 need(bridge==config['clock_bridge'] and m['qpc_frequency']==bridge['frequency'],
  'actual-native-QPC-bracket')
 _,start_hi=clock_interval(bridge,start['qpc']);_,end_hi=clock_interval(bridge,end['qpc_end'])
 born=integer(m['process_started_ns'],1,start_hi,'scope-begins-before-START')
 minimum=max([born,clock_interval(bridge,start['qpc'])[0]]+
  [clock_interval(bridge,r['qpc'])[0] for r in call['rss']])
 done=integer(m['process_done_ns'],minimum,end_hi,'scope-done-before-END-serialization')
 need(m['spawn_total_ns']==done-born and type(m['spawn_total_ns']) is int,
  'one-actual-spawn-clock-subtraction')
 catalogue=config['_catalogue'];p=catalogue['profiles'].get(m['action_profile_id'])
 need(type(p) is dict and p.get('profile_type')=='action' and
  m['action_profile_sha256']==sha(canonical(p)),'actual-catalogue-action-profile')
 group=catalogue['profiles'].get(p['group_id'])
 need(group and group['profile_type']=='group' and p['profile_id'] in group['members'],
  'authenticated-group-member')
 need(end['out_n']<=p['stdout_bytes'] and end['err_n']<=p['stderr_bytes'] and
  end['rss_n']<=p['rss_samples'],'actual-member-native-limits')
 need(len(canonical(launch['argv']))<=p['argv_max_bytes'] and
  len(canonical(launch['source_files']))<=p['source_files_max_bytes'] and
  launch['source_files']==p['producer_source_files'],'actual-action-launch-source-policy')
 if launch['producer_kind']=='python-argv':checker_binding(call,config)
 need(p['source_operation']==operation_kind(call),'actual-source-operation-profile')
 if launch['producer_kind']=='candidate':
  need(launch['executable_sha256']==identity['binary_sha256'] and
   launch['producer_source_pin']==identity['source_pin'] and len(launch['argv'])==1,
   'exact-final-engine')
  value=call['input']
  need(type(value) is dict and value.get('root')==call['begin']['root'] and
   value.get('action')==call['begin']['action'],'actual-engine-stdin')
 else:
  need(launch['producer_kind']=='python-argv','history-candidate-or-independent-checker')
  sources={(x['path'],x['sha256']) for x in launch['source_files']}
  need(all((config['bindings'][n]['path'],config['bindings'][n]['sha256']) in sources
   for n in ('oracle','oracle_cli','tree_decoder','startup_gate')),'all-checker-sources')
  need(launch['executable_sha256']==config['bindings']['python']['sha256'] and
   call['input'] is None and end['in_n']==0 and
   config['bindings']['oracle_cli']['path'] in launch['argv'] and
   call['checker_request']['root']==call['begin']['root'],'actual-independent-checker-argv')
 call['meta']=m
 return m


def operation_kind(call):
 if call['launch']['producer_kind']=='python-argv':
  mode=call['checker_request']['mode']
  return {'full':'full-oracle','history_oracle':'full-oracle','mapping':'mapping-query','backlog':'backlog-query'}[mode]
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



def native_case_index(ref,scope_id,native,config):
 index=retained_row(ref,native,config,'case_index');i=config['identity']
 need(index['scope_id']==scope_id and index['case_id']==config['parent_case_id'] and
  index['run']==i['run'] and all(index[k]==i[k] for k in
   ('source_pin','binary_sha256','budget_sha256')) and
  index['profile_catalogue_sha256']==i['profile_sha256'] and
  re.fullmatch('[0-9a-f]{32}',index['case_scope']),'actual-native-case-index')
 integer(index['start_ns'],1,index['end_ns'],'case-index-start')
 count=integer(index['total_calls'],1,config['limits']['native_calls'],'case-index-calls')
 need(len(index['pages'])==(count+127)//128,'complete128-call-page-coverage')
 entries=[]
 for ordinal,ref in enumerate(index['pages']):
  page=retained_row(ref,native,config,'case_index_page')
  need(page['ordinal']==ordinal and page['first_call']==len(entries) and
   all(page[k]==index[k] for k in ('run','case_id','scope_id','case_scope')) and
   len(page['calls'])==min(128,count-len(entries)),'strict-native-page-order-and-scope')
  entries.extend(page['calls'])
 need(len(entries)==count,'all-native-case-pairs')
 readers=[]
 for pair in entries:
  call=native['calls'].get(pair['raw_ref']['call'])
  need(call is not None and pair['raw_ref']==call['ref'],'index-exact-native-reference')
  m=native_meta(pair['meta_ref'],call,config,scope_id)
  need(m['case_scope']==index['case_scope'] and
   index['start_ns']<=m['process_started_ns']<=m['process_done_ns']<=index['end_ns'] and
   call['begin']['root']==index['root'],'all-meta-same-case-lifetime')
  if operation_kind(call)=='paced-reader-one-query-batch':
   v=call['input'];value=actual_response(call,native,config)['result']
   header=value['concurrent_pacing'];probe=value['rows'][0]
   readers.append(dict(reader=header['reader'],batch=header['batch'],raw_ref=call['ref'],
    header=header,request=v['requests'][0],first_ns=probe['first_lookup_ns']))
 # This in-memory view is reconstructed from retained native pages and responses.
 return dict(index,phase_id=scope_id,outer_run_id=index['run'],calls=entries,readers=readers)

def case_proof(ref,index,native,config,kind):
 contract=source_literal(config['bindings']['hooks'],'PROOF_PAYLOAD_CONTRACT')
 spec=contract['kinds'][kind]
 need(ref['bytes']<=spec['max_bytes']+65536,'source-case-proof-prewrite-bound')
 proof=retained_row(ref,native,config,'case_proof')
 need(proof['kind']==kind and all(proof[k]==index[k] for k in
  ('run','source_pin','binary_sha256','budget_sha256','profile_catalogue_sha256',
   'case_id','scope_id','case_scope','root','start_ns')) and
  index['start_ns']<=proof['end_ns']<=index['end_ns'] and
  retained_row(proof['clock_bridge_ref'],native,config,'bridge')==config['clock_bridge'],
  'actual-case-proof-envelope-and-window')
 return proof['payload']


def validate_terminal_calls(calls,evidence,phase,native,config):
 losers={}
 for slot in evidence['slots']:
  for offer in slot['offers']:
   if offer['outcome']!='accepted':losers[offer['request']['request']]=offer
 by_attempt={r['attempt']:r for r in evidence['terminal_receipts']}
 need(len(by_attempt)==len(evidence['terminal_receipts'])==len(losers),'bijective-terminal-attempts')
 used=set()
 for call in calls:
  if operation_kind(call)!='terminal-v2-release':continue
  v=call['input'];response=actual_response(call,native,config);r=response['result'];attempt=v['attempt']
  need(type(attempt) is str and re.fullmatch('[A-Za-z0-9_-]{1,99}',attempt) and
   response['status']=='ok' and response['error'] is None,'actual-successful-release-and-confined-attempt')
  need(attempt in by_attempt and attempt not in used,'one-source-release-per-terminal-attempt')
  row=by_attempt[attempt];offer=losers[row['request']];used.add(attempt)
  candidate_raw=read(Path(phase['root'])/'attempts'/attempt/'candidate.json',4*1024**2)
  candidate=strict(candidate_raw,4*1024**2)
  need(candidate['attempt']==attempt and candidate['request']==offer['request'] and
   candidate['expected']==offer['request']['base'],'actual-sealed-losing-candidate')
  fence_raw=read(Path(phase['root'])/'abandoned'/attempt,8192);fence=strict(fence_raw,8192)
  need(sha(fence_raw)==row['receipt_sha256']==r['receipt_sha256'] and
   len(fence_raw)==row['receipt_bytes'] and
   fence['candidate_digest']==sha(candidate_raw) and fence['request']==row['request'] and
   fence['expected']==offer['request']['base'] and
   fence['reason']=='definitive-expected-publication-conflict' and
   fence['charge_bytes']==row['retained_charge']==r['charge_bytes'] and
   r['attempt']==attempt and r['status'] in ('terminal-charged','terminal-already-charged') and
   r['reclaimed_bytes']==row['reclaimed_bytes']==0 and call['end']['exit']==0 and
   call['meta']['process_started_ns']>=offer['end_ns'],'actual-release-response-and-fence-byte-chain')
 need(used==set(by_attempt),'all-sealed-losers-released-exactly-once')

def validate_case_calls(phase, index, native, config, evidence):
 need(evidence['readers']['rows_sha256']==config['_phase_index_ref']['sha256'],'reader-summary-exact-completed-native-index')
 need(index.get('phase_id')==phase['id'] and
   all(index.get(k)==config['identity'][k] for k in ('source_pin','binary_sha256','budget_sha256')) and
   index.get('outer_run_id')==config['identity']['run'],'complete-native-case-index-identity')
 oracle=evidence['oracle'];receipt=oracle['receipt_sha256']
 need(config['_oracle_receipts'].get(receipt)=={k:v for k,v in oracle.items() if k!='receipt_sha256'},
  'phase-oracle-summary-exact-original-native-receipt')
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
 validate_terminal_calls(list(by_sha.values()),evidence,phase,native,config)
 if phase['online']:
  backlog=[c for c in by_sha.values() if operation_kind(c)=='backlog-query']
  need(len(backlog)==2 and all(c['checker_result']['source_H']==phase['H_end'] for c in backlog) and
   backlog[-1]['checker_result']['unprotected']==evidence['maintenance']['final_unprotected_backlog']==0,
   'actual-complete-two-pass-backlog-boundary')

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
 origin=body_reference(index['origin_ref'],4096,'native-pace-anchor')
 closed(origin,('clock_kind','version','phase_id','frequency','origin'),'actual-native-anchor')
 need(origin['clock_kind']=='windows-qpc' and origin['version']==1 and
  origin['phase_id']==phase['id'],'one-actual-phase-anchor')
 markers=[]
 for reader,ready_ref in enumerate(ready_refs):
  ready=body_reference(ready_ref,4096,'native-ready-'+str(reader));markers.append(ready)
  first=next(r for r in readers if r['reader']==reader and r['batch']==0)
  call=by_sha[first['raw_ref']['sha256']];header=first['header']
  need(ready['reader']==reader and ready['batch']==0 and ready['pid']==call['start']['pid'] and
   ready['phase_id']==phase['id'] and ready['frequency']==origin['frequency'] and
   ready['category']==first['request']['category'] and ready['request']==first['request']['id'] and
   ready['selector']==evidence['slots'][0]['offers'][0]['request']['base'] and
   0<ready['capture_qpc']<=ready['first_done_qpc']<=ready['ready_qpc']<origin['origin'],
   'four-actual-native-readiness-and-prewrite-capture')
  need(all(header[k]==ready[k] for k in ('physical','catalog','selector')),
   'four-first-headers-match-native-ready')
 need(len({r['pid'] for r in markers})==4 and
  len({sha(canonical(r['physical'])) for r in markers})==1 and
  [r['pid'] for r in markers]==evidence['readers']['initial_native_pids'],
  'four-distinct-initial-native-readers-one-physical-generation')
 for row in readers:
  header=row['header'];reader=row['reader'];ready=markers[reader]
  need(header['frequency']==origin['frequency'] and header['origin']==origin['origin'] and
   header['anchor_sha256']==index['origin_ref']['sha256'] and
   header['ready_sha256']==ready_refs[reader]['sha256'] and
   header['clock_kind']=='windows-qpc' and header['period_us']==(2900 if reader==0 else 4500) and
   header['hold_us']==(16000000 if reader==0 else 0),'all40-headers-one-fixed-anchor-and-ready')
 old=evidence['readers']['old_reader'];ready=markers[0]
 first=next(r for r in readers if r['reader']==0 and r['batch']==0)
 call=by_sha[first['raw_ref']['sha256']];header=first['header']
 need(ready['pid']==old['pid'],'actual-old-reader-initial-native-identity')
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
  index=native_case_index(by_id[phase['id']],phase['id'],native,config)
  for row in index['calls']:
   token=row['raw_ref']['call'];need(token not in used,'call-not-reused-between-cases')
   used.add(token)
 for identity,donor in config['plan']['donors'].items():
  for case_id,bootstrap in [('lookup-'+identity,False)]+[
    ('bootstrap-'+identity+'-p'+str(p),True) for p in (1,4)]:
   index=native_case_index(by_id[case_id],case_id,native,config)
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
  index=native_case_index(body['native_call_index_ref'],body['phase_id'],native,config)
  for name,kind,index_key in (('owned_closure_ref','owned-drain','owned_drain_ref'),
   ('checkpoint_ref','checkpoint','checkpoint_ref'),('case_closure_ref','case-close','case_closure_ref')):
   need(body[name]==index[index_key],'same-attachment-used-by-live-and-postexit-consumers')
   payload=case_proof(body[name],index,native,config,kind)
   signature=range_key(body[name])
   need(signature not in seen_attachments,'attachment-not-reused');seen_attachments.add(signature)
   if name=='owned_closure_ref':
    need(body[name]['sha256']==observation['owned_drain_sha256'] and
     payload['children_empty'] is True and payload['failures']==[] and
     all(payload[k] is True for k in ('processes_closed','handles_closed','callbacks_joined')) and
     len(payload['owned_members_polls'])==2 and
     all(p['owned_children']==[] for p in payload['owned_members_polls']),
     'actual-owned-drain')
   elif name=='checkpoint_ref':
    need(payload['go'] is True and payload['no_other_semantic_writer'] is True and
     payload['planned_reclaim_credit']==0,'actual-positive-source-checkpoint')
   else:
    need(payload['processes_closed'] is True and payload['handles_closed'] is True and
     payload['python_callbacks_joined'] is True and payload['failures']==[] and
     payload['callback_registry']['callbacks']==payload['callback_registry']['joined'] and
     payload['case_scope']==index['case_scope'] and payload['scope_id']==body['phase_id'],
     'actual-case-native-resource-closure')
  native.setdefault('case_index_refs',{})[ref['id']]=body['native_call_index_ref']
  index=native_case_index(body['native_call_index_ref'],body['phase_id'],native,config)
  config['_phase_index_ref']=body['native_call_index_ref']
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
 def walk_error(error):raise error
 for name,spec in rows.items():
  need({'root','bytes','files','directories','remaining_reservation'}==set(spec),'census-root-spec')
  for k in ('bytes','files','directories'):integer(spec[k],0,2**63-1,'root-cap:'+k)
  root=no_links(spec['root']);used=count=dirs=0;h=hashlib.sha256()
  if root.exists():
   need(root.is_dir(),'census-root-directory')
   for folder,children,names in os.walk(root,onerror=walk_error,followlinks=False):
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
 carried=parent_reference(carry['receipt_ref'],config['limits']['metadata_bytes'],'foreign-ledger')
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
 need(caps['bytes']<=44*1024**3 and caps['files']<=5000000,'fixed-whole-experiment-ceilings')
 need(all_bytes<=caps['bytes'] and all_files<=caps['files'] and all_dirs<=caps['directories'],
   'whole-combined-resource-cap')
 return dict(roots=result,combined=dict(bytes=all_bytes,files=all_files,directories=all_dirs),
    foreign_reservations=carry,planned_deletion_credit=0)



def closed_job_lifetime(value,creator,guard_sha,audit,deadline):
 closed(creator,('pid','creation'),'native-creator')
 need(value['guard_sha256']==guard_sha and value['creator']==creator,'exact-Job-owner-source')
 integer(creator['pid'],1,2**32-1,'creator-pid');integer(creator['creation'],1,2**64-1,'creator-creation')
 q=closed(value['creator_job_query'],('success','is_member','exact_job_handle','perf_ns'),'exact-Job-query')
 flags=closed(value['job_handle_flags'],('query_success','flags','perf_ns'),'Job-flags')
 need(q['success'] is True and q['is_member'] is False and q['exact_job_handle'] is True and
  flags['query_success'] is True and type(flags['flags']) is int and 0<=flags['flags']<2**32 and
  flags['flags']&1==0,'creator-outside-noninheritable-Job')
 need(audit['guard_sha256']==guard_sha and audit['no_job_handle_duplication'] is True and
  audit['no_job_handle_inheritance'] is True and audit['complete_cooperating_owner_universe']==[creator],
  'reviewed-complete-Job-handle-owner-universe')
 host=closed(value['host'],('pid','creation','exit_observed_perf_ns','exit_code',
  'io_closed','writer_closed','callbacks_joined'),'actual-Job-host')
 integer(host['pid'],1,2**32-1,'host-pid');integer(host['creation'],1,2**64-1,'host-creation')
 need(host['exit_code']==0 and type(host['exit_code']) is int and
  all(host[k] is True for k in ('io_closed','writer_closed','callbacks_joined')),'actual-host-resource-closure')
 exited=integer(host['exit_observed_perf_ns'],1,deadline,'host-native-exit')
 empty=value['empty_observations'];need(type(empty) is list and len(empty)==2,'two-native-empty-observations')
 previous=exited
 for row in empty:
  closed(row,('perf_ns','members'),'native-empty')
  need(row['members']==[],'actual-empty-Job-members')
  previous=integer(row['perf_ns'],previous+1,deadline,'empty-after-native-exit')
 close=closed(value['job_close'],('success','completed_perf_ns'),'actual-Job-close')
 ended=integer(close['completed_perf_ns'],previous,deadline,'last-Job-handle-close')
 need(close['success'] is True,'positive-last-Job-handle-close')
 integer(q['perf_ns'],1,exited,'creator-query-before-host-exit')
 integer(flags['perf_ns'],1,exited,'flags-query-before-host-exit')
 return ended


def validate_native_lifetime(lifetime,completion,grant,config):
 wire=source_literal(config['bindings']['guard'],'GUARD_WIRE_CONTRACT')
 closed(lifetime,wire['native_lifetime']['keys'],'actual-inner-lifetime')
 closed(completion,wire['parent_completion']['required'],'actual-parent-completion')
 need(lifetime['schema']==wire['native_lifetime']['schema'] and
  lifetime['run_nonce']==grant['original_run_nonce'] and
  completion['schema']==wire['parent_completion']['schema'] and
  completion['run_nonce']==lifetime['run_nonce'] and
  completion['guard_sha256']==grant['original_guard_sha256'],'original-inner-native-run')
 audit=parent_reference(lifetime['source_audit_ref'],65536,'native-owner-source-audit')
 ended=closed_job_lifetime(lifetime,lifetime['creator'],grant['original_guard_sha256'],audit,grant['issued_perf_ns'])
 need(lifetime['host']['pid']==config['identity']['host_pid'] and
  lifetime['host']['creation']==config['identity']['host_creation'],'actual-original-experiment-host')
 source=source_literal(config['bindings']['parent_capture'],'CAPTURE_WIRE_CONTRACT')
 need(source['api']=='bounded-unlocked-outer-wire-r3','actual-unlocked-parent-publisher-source')
 review=parent_reference(audit['source_review_ref'],65536,'actual-source-owner-audit-review')
 need(review['source_reviewed'] is True and
  review['capture_sha256']==config['bindings']['parent_capture']['sha256'] and
  review['guard_sha256']==grant['original_guard_sha256'] and
  review['no_job_handle_duplication'] is True and review['no_job_handle_inheritance'] is True,
  'actual-reviewed-original-guard-and-parent-source')
 outer=completion['outer_controller_lifetime']
 closed(outer,('schema','run_nonce','creator','creator_job_query','job_handle_flags','source_audit_ref',
  'source_audit_selector','host','empty_observations','job_close','guard_sha256'),'actual-outer-lifetime')
 need(outer['schema']=='unlocked-outer-job-native-lifetime-v1' and
  outer['source_audit_ref']==lifetime['source_audit_ref'] and
  outer['source_audit_selector']=='outer_controller_audit' and
  (outer['host']['pid'],outer['host']['creation'])==
   (lifetime['creator']['pid'],lifetime['creator']['creation']),'actual-guard-worker-is-original-inner-creator')
 outer_end=closed_job_lifetime(outer,outer['creator'],config['bindings']['parent_capture']['sha256'],
  audit['outer_controller_audit'],grant['issued_perf_ns'])
 exit_row=closed(completion['native_guard_worker_exit'],('pid','creation','wait_result','exit_code',
  'handle_close_result','closed_perf_ns'),'actual-native-worker-wait-and-close')
 need((exit_row['pid'],exit_row['creation'])==(outer['host']['pid'],outer['host']['creation']) and
  exit_row['wait_result']==0 and type(exit_row['wait_result']) is int and
  exit_row['exit_code']==0 and type(exit_row['exit_code']) is int and
  exit_row['handle_close_result'] is True and
  ended<=outer['host']['exit_observed_perf_ns']<=exit_row['closed_perf_ns']<=outer_end,
  'inner-close-before-guard-worker-reaped-and-outer-Job-close')
 need(completion['creator']==lifetime['creator'] and completion['guard_reaped'] is True and
  completion['guard_exit_code']==0 and type(completion['guard_exit_code']) is int and all(completion[k] is True for k in
   ('stdout_eof','stderr_eof','callbacks_joined')),'actual-original-guard-worker-completion')
 stored=parent_reference(config['guard_receipt'],config['limits']['guard_json_bytes'],'actual-stored-original-guard')
 original=completion['original_returned_completion']
 need(all(original[k]==stored[k] for k in ('native_host','creator_lifetime','job_close',
  'closure_checks','run_nonce','guard_sha256','budget_sha256','bounds_sha256')) and
  stored['native_host']==lifetime['host'] and
  (stored['creator_lifetime']['pid'],stored['creator_lifetime']['creation_time'])==
   (lifetime['creator']['pid'],lifetime['creator']['creation']) and
  [(x['perf_counter_ns'],x['members']) for x in stored['closure_checks']]==
   [(x['perf_ns'],x['members']) for x in lifetime['empty_observations']] and
  stored['job_close']['checked'] is True and
  stored['job_close']['perf_counter_ns']==lifetime['job_close']['completed_perf_ns'],
  'actual-same-original-receipt-and-normalized-native-lifetime')
 need(original['qualified'] is True and original['censor'] is None and original['exit']==0 and
  original['process_closure_verified'] is True and original['postwrite_census_complete'] is True and
  original['receipt_sha256']==config['guard_receipt']['sha256'] and
  original['budget_sha256']==config['identity']['budget_sha256'] and
  original['guard_sha256']==grant['original_guard_sha256'] and
  original['run_nonce']==lifetime['run_nonce'] and original['native_host']==lifetime['host'],
  'actual-returned-inner-completion-not-stored-future-claim')
 outer_receipt=parent_reference(completion['outer_receipt_ref'],config['_outer_guard_bounds']['final']['json_bytes'],'actual-outer-receipt')
 need(all(completion[k]==outer_receipt[k] for k in ('budget_sha256','bounds_sha256','config_ref')),
  'same-actual-outer-completion-and-receipt-binding')
 file_binding(outer_receipt['config_ref'],131072,'original-capture-config')
 need(outer_receipt['budget_sha256']==config['_outer_guard_ref']['sha256'] and
  outer_receipt['bounds_sha256']==sha(guard_encoding(config['_outer_guard_bounds'])),
  'actual-original-outer-budget-and-insertion-order-bounds')
 need(outer_receipt['guard_sha256']==outer['guard_sha256'] and
  outer_receipt['dependency_guard_sha256']==lifetime['guard_sha256'] and
  outer_receipt['run_nonce']==outer['run_nonce'] and outer_receipt['host']==outer['host'] and
  outer_receipt['closure_checks']==outer['empty_observations'] and
  outer_receipt['job_close']==outer['job_close'] and
  outer_receipt['process_closure_verified'] is True and outer_receipt['errors']==[] and
  outer_receipt['first_error'] is None,'actual-parent-outer-native-receipt')
 integer(completion['completed_perf_ns'],outer_end,grant['issued_perf_ns'],'both-Jobs-close-before-grant')
 return outer_end


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


def review_range(ref,maximum):
 closed(ref,('path','offset','bytes','sha256'),'sealed-review-range')
 path,offset,count,digest=range_key(ref);need(count<=maximum,'bounded-review-range')
 with Path(path).open('rb') as f:
  before=os.fstat(f.fileno());need(offset+count<=before.st_size,'review-span')
  f.seek(offset);raw=f.read(count);after=os.fstat(f.fileno())
 need(len(raw)==count and sha(raw)==digest and
  (before.st_dev,before.st_ino,before.st_size,before.st_mtime_ns)==
  (after.st_dev,after.st_ino,after.st_size,after.st_mtime_ns),'sealed-review-content')
 return raw

def validate_source_freeze(config,authority):
 from datetime import datetime
 freeze=closed(authority['source_freeze'],('engine','harness','manifest_ref','review_ref'),'source-freeze')
 need(freeze['engine']=='2026-10-21T04:10:14Z','fixed-engine-checkpoint')
 stamp=freeze['harness']
 need(type(stamp) is str and re.fullmatch(r'2026-10-21T\d{2}:\d{2}:\d{2}(?:\.\d{1,6})?Z',stamp),
  'actual-prospective-harness-UTC')
 try:instant=datetime.fromisoformat(stamp[:-1]+'+00:00')
 except ValueError:need(False,'valid-harness-calendar-time')
 need(datetime.fromisoformat('2026-10-21T04:10:14+00:00')<=instant<
  datetime.fromisoformat('2026-10-21T11:10:14+00:00'),'fixed-functional-clock')
 root=Path('C:/Users/spmcc/.codex/visualizations/2026/10/20/01a15ab8-d85f-71b1-8d37-c3c5689790be')
 for name in ('manifest_ref','review_ref'):
  ref=closed(freeze[name],('path','offset','bytes','sha256'),name)
  need(no_links(ref['path']).is_relative_to(root) and ref['offset']==0,'actual-parent-machine-reference')
 need(sum(integer(freeze[n]['bytes'],1,65536,'machine-record-bytes') for n in ('manifest_ref','review_ref'))<=65536,'two-machine-record-allocation')
 manifest=body_reference(freeze['manifest_ref'],65536,'frozen-source-manifest')
 review=body_reference(freeze['review_ref'],65536,'parent-source-review-acceptance')
 closed(manifest,('kind','version','final_pin','frozen_utc','source_files'),'source-manifest')
 closed(review,('kind','version','verdict','final_pin','source_manifest_sha256','source_files_sha256','review_refs'),'review-acceptance')
 pin=config['identity']['source_pin'];files=manifest['source_files']
 need(pin=='0628322e4b48e0efad19cdc32238a622d4aaa7a8','fixed-engine-062')
 need(manifest['kind']=='private-harness-source-manifest-v1' and type(manifest['version']) is int and
  manifest['version']==1 and manifest['final_pin']==pin and manifest['frozen_utc']==stamp and
  type(files) is dict and 0<len(files)<=512,'actual-frozen-source-closure')
 need(review['kind']=='private-harness-review-acceptance-v1' and type(review['version']) is int and
  review['version']==1 and review['verdict']=='SOURCE_ACCEPTED_FOR_BOUNDED_PHASE_ADMISSION' and
  review['final_pin']==pin and review['source_manifest_sha256']==freeze['manifest_ref']['sha256'] and
  review['source_files_sha256']==sha(canonical(files)),'exact-parent-reviewed-closure-association')
 paths={}
 for path,row in files.items():
  closed(row,('bytes','sha256'),'manifest-source-entry')
  key=os.path.normcase(str(no_links(path)));need(key not in paths,'unique-source-path')
  file_binding(dict(path=path,**row),64*1024**2,'frozen-source-file');paths[key]=row
 for name in ('history','profiles','runtime','finalizer'):
  row=config['bindings'][name];key=os.path.normcase(str(no_links(row['path'])))
  need(paths.get(key)=={k:row[k] for k in ('bytes','sha256')},'reviewed-private-helper:'+name)
 links=review['review_refs'];need(type(links) is list and 0<len(links)<=64,'actual-independent-review-links')
 used=0
 for ref in links:
  used+=integer(ref['bytes'],1,1024**2,'review-bytes');need(used<=1024**2,'aggregate-review-byte-cap')
  review_range(ref,1024**2)
 config['_source_freeze']=freeze;config['_source_files_sha256']=review['source_files_sha256'];config['_frozen_files']=paths

def validate_freeze_capture(native,config):
 notes={};gates={}
 for head,ref in native['sidebands']:
  kind=head['sideband_kind']
  if kind=='script-startup-gate-ready':
   payload=retained_row(ref,native,config,'sideband')['payload'];token=payload['call_token']
   need(token not in gates,'one-actual-script-gate');gates[token]=(payload,ref)
  if kind in ('history-plan-admitted','LaneHost-start'):
   need(kind not in notes,'one-original-source-authorization-note')
   notes[kind]=retained_row(ref,native,config,'sideband')['payload']
 need(set(notes)=={'history-plan-admitted','LaneHost-start'} and
  notes['history-plan-admitted']['bindings']['source_freeze']==config['_source_freeze'] and
  notes['LaneHost-start']['source_manifest_sha256']==config['_source_files_sha256'],
  'actual-original-host-source-freeze-and-complete-filemap')
 start=notes['LaneHost-start']
 need(start['guard_budget_refs']==dict(inner=config['_guard_ref'],outer=config['_outer_guard_ref']) and
  start['guard_bounds']==dict(inner=config['_guard_bounds'],outer=config['_outer_guard_bounds']),
  'actual-native-captured-both-budget-refs-and-numeric-bounds')
 bootstrap=notes['LaneHost-start']['gate_receipt']
 expected={t for t,c in native['calls'].items() if c['launch']['producer_kind']=='python-argv'}
 need(set(gates)==expected,'all-and-only-actual-independent-checker-gates')
 roots=[no_links(row['root']) for row in config['census']['roots'].values()]
 for token,(payload,ref) in gates.items():
  call=native['calls'][token];gate=payload['gate'];meta=native['metas'][token][0]
  wire_check(gate,wire_contract(config)['types']['script_gate'],wire_contract(config))
  need(len(canonical(gate))<=8192 and gate['root']==call['begin']['root'] and
   sha(canonical(gate))==call['launch']['startup_gate_sha256'] and
   call['start_ref']['offset']<ref['offset']<call['end_ref']['offset'],'actual-source-gate-before-release')
  need(all(payload['context'][k]==meta[k] for k in ('case_id','scope_id','case_scope')),
   'gate-owned-native-case-context')
  raw=json.dumps(dict(pid=bootstrap['pid'],nonce=bootstrap['nonce'],
   manifest_sha256=gate['manifest_sha256']),separators=(',',':')).encode()
  need(sha(raw)==bootstrap['gate_sha256'],'same-actual-bootstrap-manifest')
  ready=no_links(gate['ready']);release=no_links(gate['release'])
  need(ready!=release and ready.parent==release.parent and
   any(ready.is_relative_to(root) for root in roots),'owned-original-script-gate-files')
  rr=closed(payload['ready_ref'],('path','offset','bytes','sha256'),'retained-script-ready')
  need(no_links(rr['path'])==ready and rr['offset']==0,'exact-gate-ready-file')
  file_binding({k:rr[k] for k in ('path','bytes','sha256')},8192,'whole-script-ready-file')
  observed=strict(review_range(rr,8192),8192)
  need(observed==dict(pid=call['start']['pid'],nonce=gate['nonce'],
   manifest_sha256=gate['manifest_sha256']),'same-native-checker-ready-before-corpus-IO')
  raw=canonical(dict(nonce=gate['nonce'],pid=call['start']['pid']))+b'\n'
  file_binding(dict(path=str(release),bytes=len(raw),sha256=sha(raw)),8192,'actual-script-release')
  integer(payload['deadline_perf_ns'],1,2**63-1,'actual-script-deadline')

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
 grant=parent_reference({k:v for k,v in grant_ref.items() if k!='reviewed'},65536,'parent-postexit-grant')
 keys=('schema','parent_released','original_run_nonce','original_guard_sha256',
    'original_guard_receipt_ref','original_completion_ref','original_native_lifetime_ref',
    'phase_index_ref','expected_source_pin','expected_binary_sha256','expected_runtime_sha256',
    'expected_profile_sha256','expected_finalizer_sha256','expected_new_guard_sha256',
    'expected_finalizer_parent','issued_perf_ns','deadline_perf_ns',
    'authority_ref','profile_catalogue_ref')
 closed(grant,keys,'actual-parent-postexit-grant')
 need(grant['schema']=='postexit-finalizer-grant-v2-r3' and grant['parent_released'] is True and
   grant['original_run_nonce']==i['run'] and grant['original_guard_sha256']==i['guard_sha256'] and
   all(grant['expected_'+k]==i[k] for k in ('source_pin','binary_sha256')) and
   grant['expected_profile_sha256']==config['bindings']['profiles']['sha256'] and
   grant['expected_runtime_sha256']==config['bindings']['runtime']['sha256'] and
   grant['expected_finalizer_sha256']==config['bindings']['finalizer']['sha256'] and
   grant['expected_new_guard_sha256']==config['bindings']['new_guard']['sha256'] and
   grant['original_guard_receipt_ref']==config['guard_receipt'],
   'parent-grant-exact-source-input-bindings')
 authority=parent_reference(grant['authority_ref'],4*1024**2,'immutable-finalizer-allocation')
 closed(authority,('schema','run','source_pin','budget_sha256','parent_case_id','census',
  'output','limits','caps','guard_bounds_ref','outer_guard_bounds_ref','source_freeze'),'finalizer-allocation')
 need(authority['schema']=='finalizer-allocation-v1' and authority['run']==i['run'] and
  authority['source_pin']==i['source_pin'] and authority['budget_sha256']==i['budget_sha256'] and
  authority['parent_case_id']==config['parent_case_id'] and
  all(authority[k]==config[k] for k in ('census','output','limits','caps')),
  'exact-authoritative-root-membership-caps-and-holds')
 validate_source_freeze(config,authority)
 guard_ref=authority['guard_bounds_ref'];config['_guard_ref']=guard_ref
 config['_outer_guard_ref']=authority['outer_guard_bounds_ref']
 catalogue=parent_reference(grant['profile_catalogue_ref'],4*1024**2,'global-profile-catalogue')
 closed(catalogue,('version','schedule_sha256','profiles'),'global-profile-catalogue')
 need(catalogue['version']==3 and hex64(catalogue['schedule_sha256']) and
  sha(canonical(catalogue))==i['profile_sha256'],'global-catalogue-sha-distinct-from-source-file')
 kinds=source_literal(config['bindings']['hooks'],'SIDEBAND_KINDS')
 auxiliary=catalogue['profiles']['capture/sidebands']
 closed(auxiliary,('profile_id','profile_type','sideband_bounds'),'source-sideband-auxiliary')
 need(auxiliary['profile_id']=='capture/sidebands' and auxiliary['profile_type']=='auxiliary' and
  set(auxiliary['sideband_bounds'])==set(kinds),'exact-source-owned-sideband-membership')
 for amount in auxiliary['sideband_bounds'].values():integer(amount,1,4194304,'sideband-ceiling')
 for name,p in catalogue['profiles'].items():
  need(p['profile_id']==name and p['profile_type'] in ('action','group','auxiliary') and
   'profile_sha256' not in p and 'profile_catalogue_sha256' not in p,'canonical-noncircular-profile-payload')
  if p['profile_type']=='action':
   group=catalogue['profiles'][p['group_id']]
   need(group['profile_type']=='group' and name in group['members'],'profile-membership')
   for key,value in p.items():
    if type(value) in (int,float):need(type(group[key]) is type(value) and value<=group[key],'member-under-group')
   need(set(p['all_sink_components'])==set(group['all_sink_components']) and
    all(type(v) is int and 0<=v<=group['all_sink_components'][k]
     for k,v in p['all_sink_components'].items()),'member-components-under-group')
 need(guard_ref['sha256']==i['budget_sha256'] and
  guard_ref.get('offset',0)==config['_outer_guard_ref'].get('offset',0)==0,'exact-original-inner-and-outer-guard-budgets')
 guard_budget=parent_reference(guard_ref,131072,'actual-original-guard-budget')
 bounds=guard_budget['evidence_bounds']
 outer_budget=parent_reference(config['_outer_guard_ref'],131072,'actual-original-outer-budget')
 config['_outer_guard_bounds']=outer_budget['evidence_bounds']
 contract=source_literal(config['bindings']['guard'],'GUARD_WIRE_CONTRACT')
 closed(bounds,contract['bounds_contract']['root_keys'],'actual-source-guard-bounds')
 closed(config['_outer_guard_bounds'],contract['bounds_contract']['root_keys'],'actual-outer-bounds')
 integer(config['_outer_guard_bounds']['final']['json_bytes'],8192,32*1024**2,'bounded-actual-outer-receipt')
 need(8192<=config['limits']['guard_json_bytes']==bounds['final']['json_bytes']<=32*1024**2,
  'actual-guard-receipt-parser-bound')
 need(bounds['rss']['raw_bytes']==config['limits']['qpc_expanded_bytes'] and
  bounds['rss']['gzip_bytes']==config['limits']['qpc_compressed_bytes'] and
  bounds['rss']['row_bytes']==config['limits']['qpc_line_bytes'] and
  bounds['rss']['total_rows']==config['limits']['qpc_rows'],'guard-parser-limits-equal-actual-admission')
 config['_wire']=wire_contract(config)
 config['_catalogue']=catalogue;config['_guard_bounds']=bounds;config['_parent_grant']=grant
 integer(grant['issued_perf_ns'],1,time.perf_counter_ns(),'parent-grant-issued')
 integer(time.perf_counter_ns(),grant['issued_perf_ns'],grant['deadline_perf_ns'],'parent-grant-deadline')
 lifetime=parent_reference(grant['original_native_lifetime_ref'],65536,'old-native-Job-lifetime')
 completion=parent_reference(grant['original_completion_ref'],65536,'old-parent-completion')
 index=parent_reference(grant['phase_index_ref'],1024*1024,'exact16-parent-phase-index')
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
   config['limits']['native_line_bytes']<=4259840 and
   config['limits']['qpc_line_bytes']<=4194304 and config['limits']['phase_bytes']<=4*1024**2 and
   config['limits']['job_members']<=12,'bounded-parser-memory')
 closed(config['caps'],('reader_bytes','checker_bytes','aggregate_bytes','supervisor_bytes'),'role-caps')
 need(config['caps']['reader_bytes']==256*1024**2 and config['caps']['checker_bytes']==2*1024**3 and
   config['caps']['aggregate_bytes']<=4*1024**3 and
   config['caps']['supervisor_bytes']<=768*1024**2,'unchanged-role-resource-limits')
 for name,row in config['bindings'].items():
  file_binding(row,64*1024**2 if name in ('python','engine') else 2*1024**2,'transitive-source:'+name)
 need(config['bindings']['guard']['sha256']==i['guard_sha256'],'actual-guard-source')
 need(no_links(__file__)==Path(config['bindings']['finalizer']['path']) and
   sha(read(__file__,integer(config['bindings']['finalizer']['bytes'],1,105472,'actual-own-source-bytes')))==config['bindings']['finalizer']['sha256'],'own-finalizer-source')
 return config


def join_history(config):
 validate_join_inputs(config)
 plan=config['plan']
 need(plan['total_accepted']==1600 and plan['max_offers']==4000 and
   plan['max_sealed_losers']==2400 and len(plan['phases'])==16,'whole-history-plan')
 native=raw_capture(config)
 config['_native']=native
 validate_freeze_capture(native,config)
 for call in native['calls'].values():
  if call['launch']['producer_kind']=='python-argv':checker_binding(call,config)
 oracle_receipts(native,config)
 history=source(config['bindings']['history'],'history')
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
     host_exit_ns=guard['host_exit_ns'],job_empty_ns=guard['job_empty_ns'],endpoint_coverage=guard['endpoint_coverage']),
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


