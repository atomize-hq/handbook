
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import re

API = 'long-source-profiles-v1'
CAPTURE_API = 'single-raw-ref-v1'
INVOKE_API = 'lane-invoke-v2'
MiB = 1024**2
SOURCE_FREEZE = '2026-10-21T04:10:14Z'
FUNCTIONAL = '2026-10-21T11:10:14Z'
REQUESTS = (('early','history-000001'),('middle','history-050000'),
   ('recent','reclamation-restore-prior-bytes'),('missing','P-benchmark-missing'),
   ('cancelled','P-benchmark-cancelled'),('lost-reply','deep-lost-reply'))
SELECTED = {'publication':'publication-26408-1792467841305851000-19',
   'digest':'6d11b0a4e06e4c85ae391516bc99c840f4724bb547d018268bd9e9a2c7ea81cf'}
NO_PACKET=b'{"requested":false,"recipe":"not-requested","renderer":"exact-concat-v1","inputs":[],"omissions":["no new context packet requested"],"errors":[],"output_sha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"}'
KNOWN_BYTES = (89572, 89830)
KNOWN_SHA = ('e1567342966ea6e22ea44dff3cd35158ec8caff1ecaddad3f4435f7b41ebe02e',
    '1c16e63bdc566bdc5f6fd8037025faf8fac27ba1db03d0244b510fd3b7e5a6f0')
INTENT_PLAN_SHA = '3f91bbc7185c515c47fd20bc416b1f107797c39bfdbe26c1233e87d3c1266886'
SUCCESSOR = 'online-long-stream-successor-20261020'
NUMERIC_METRIC_TYPES = {'u64':20,'u128':39,'bool':5,'Option<u64>':20}
SOURCE_FILES = ('api.rs','io.rs','packing.rs','store.rs','disk.rs','physical_lifetime.rs','objects.rs','rolling.rs','pack.rs','paged.rs','lifetime.rs','projection.rs','graph.rs','lib.rs')
SCOPE_METHOD = 'closed-rust-metrics-and-immutable-point-routes-v1'

def need(ok, message):
 if not ok:
  raise RuntimeError(message)

def closed(value, keys, name):
 need(type(value) is dict and set(value) == set(keys), 'closed:'+name)
 return value

def integer(value, low, high, name):
 need(type(value) is int and low <= value <= high, 'integer:'+name)
 return value

def hex64(value):
 return type(value) is str and re.fullmatch('[0-9a-f]{64}', value) is not None

def sha(raw):
 return hashlib.sha256(raw).hexdigest()

def canonical(value):
 return json.dumps(value, sort_keys=True, separators=(',',':'), ensure_ascii=True,
       allow_nan=False).encode()

def no_links(path):
 p = Path(path)
 need(p.is_absolute() and '..' not in p.parts, 'absolute-confined-path')
 for node in [p, *p.parents]:
  if os.path.lexists(node):
   s = node.lstat()
   need(not node.is_symlink() and not getattr(s,'st_file_attributes',0)&1024,
     'link-or-reparse')
   if node.is_file():
    need(s.st_nlink == 1, 'hardlinked-input')
 return p

def read(path, maximum):
 path = no_links(path)
 with path.open('rb') as f:
  a = os.fstat(f.fileno())
  need(a.st_size <= maximum, 'input-size')
  raw = f.read(maximum+1)
  b = os.fstat(f.fileno())
 need(len(raw) <= maximum and (a.st_dev,a.st_ino,a.st_size,a.st_mtime_ns) ==
   (b.st_dev,b.st_ino,b.st_size,b.st_mtime_ns), 'input-changed')
 return raw

def strict(raw, maximum):
 need(type(raw) is bytes and len(raw) <= maximum, 'json-bound')
 def pairs(items):
  result = {}
  for k,v in items:
   need(k not in result, 'duplicate-json-key')
   result[k] = v
  return result
 def invalid(_):
  raise ValueError('nonfinite-json')
 return json.loads(raw, object_pairs_hook=pairs, parse_constant=invalid)

def bound_source(row, maximum=8*MiB):
 need(type(row) is dict and row.get('reviewed') is True and hex64(row.get('sha256')),
   'source-not-released')
 size = integer(row.get('bytes'),0,maximum,'source-bytes')
 raw = read(row['path'],maximum)
 need(len(raw) == size and sha(raw) == row['sha256'], 'source-binding')
 return raw

def load_source(row, name):
 raw=bound_source(row)
 spec = importlib.util.spec_from_file_location(name,row['path'])
 module = importlib.util.module_from_spec(spec)
 exec(compile(raw,row['path'],'exec'),module.__dict__)
 return module

def json_object_width(parts):
 
 need(type(parts) is dict and all(type(k) is str and type(v) is int and v >= 0
         for k,v in parts.items()), 'width-object')
 return 2 + max(0,len(parts)-1) + sum(len(canonical(k))+1+v for k,v in parts.items())

def json_array_width(count, item_width):
 integer(count,0,10**9,'array-count')
 integer(item_width,0,2**63-1,'array-width')
 return 2 + count*item_width + max(0,count-1)

def ascii_string_width(size):
 return integer(size,0,8*MiB,'ascii-width')+2

def unknown_string_width(utf8_bytes):
 return 2+6*integer(utf8_bytes,0,8*MiB,'escaped-width')

def map_width(keys, value_width):
 need(type(keys) in (tuple,list) and len(keys)==len(set(keys)), 'distinct-map-keys')
 return json_object_width({key:value_width for key in keys})

def _rust_block(text, marker):
 start = text.find(marker)
 need(start >= 0, 'source-marker:'+marker)
 start = text.find('{',start)
 need(start >= 0, 'source-block')
 depth=0
 for i in range(start,len(text)):
  if text[i]=='{':
   depth+=1
  elif text[i]=='}':
   depth-=1
   if depth==0:
    return text[start+1:i]
 raise RuntimeError('unclosed-source-block')

def metrics_schema(io_source):
 
 body = _rust_block(io_source,'pub struct Metrics')
 body = re.sub(r'//[^\n]*','',body)
 segments=[]; angle=paren=square=0; start=0
 for i,ch in enumerate(body):
  if ch=='<': angle+=1
  elif ch=='>': angle-=1
  elif ch=='(': paren+=1
  elif ch==')': paren-=1
  elif ch=='[': square+=1
  elif ch==']': square-=1
  elif ch==',' and angle==paren==square==0:
   segments.append(body[start:i]); start=i+1
  need(min(angle,paren,square)>=0,'source-type-brackets')
 need(angle==paren==square==0,'source-type-brackets')
 segments.append(body[start:])
 result={}
 for item in segments:
  item=item.strip()
  if not item: continue
  skipped = '#[serde(skip)]' in item.replace(' ','')
  item=re.sub(r'#\[[^\]]+\]','',item).strip()
  match=re.fullmatch(r'pub(?:\(crate\))?\s+([A-Za-z_]\w*)\s*:\s*(.+)',item,re.S)
  need(match is not None,'unrecognized-Metrics-field')
  name,kind=match.groups(); kind=re.sub(r'\s+','',kind)
  need(name not in result,'duplicate-Metrics-field')
  if skipped: continue
  need(kind in NUMERIC_METRIC_TYPES or kind in ('Vec<u64>','BTreeMap<String,u64>',
    'BTreeMap<String,u128>','std::collections::BTreeSet<String>'),
    'unrecognized-serialized-Metrics-type:'+kind)
  result[name]=kind
 need(result.get('unique_pages')=='std::collections::BTreeSet<String>' and
   result.get('ns')=='BTreeMap<String,u128>', 'expected-Metrics-contract')
 return result

def engine_scope(bindings):
 
 need(type(bindings) is dict and set(SOURCE_FILES) <= set(bindings) and
   len(bindings)<=64 and all(k.endswith('.rs') for k in bindings),'complete-engine-source-set')
 texts={name:bound_source(row,2*MiB).decode('utf-8') for name,row in bindings.items()}
 need('const MAX_HEIGHT: u8 = 63' in texts['disk.rs'] and
   'const MAX_KEY_BYTES: usize = 1024' in texts['disk.rs'],'disk-bound-source')
 need('store.publish(serde_json::from_value(v["request"].clone())?' in texts['api.rs'],
   'nested-publish-source')
 need('"lookup-bench"=>crate::packing::probe' in texts['api.rs'],'lookup-source')
 need('physical_upgrade_holds_ns' in texts['io.rs'],'captured-Metrics')
 timing=set()
 for text in texts.values():
  timing.update(re.findall(r'\.elapsed\(\s*"([^"]+)"',text))
  timing.update(re.findall(r'\.ns\.(?:insert|entry)\(\s*"([^"]+)"',text))
  for argument in re.findall(r'\b(?:m|metrics)\.elapsed\((if[^;]+?),\s*[A-Za-z_]',text):
   timing.update(re.findall(r'"([^"]+)"',argument))
 need(timing and all(len(x.encode())<=128 for x in timing),'finite-timing-key-source')
 schema=metrics_schema(texts['io.rs'])
 return dict(schema=schema,timing=sorted(timing),
    source_sha256=sha(canonical({k:dict(bytes=v['bytes'],sha256=v['sha256'])
            for k,v in bindings.items()})),
    scope_method=SCOPE_METHOD)

def authenticate_geometry(bindings, root, requests):
 
 need(type(requests) in (list,tuple) and len(requests)==6,'six-geometry-requests')
 need(tuple((r['category'],r['id']) for r in requests)==REQUESTS,'exact-six-geometry-IDs')
 decoder_module=load_source(bindings['oracle'],'profile_existing_v5')
 tree_module=load_source(bindings['tree_decoder'],'profile_existing_tree')
 need(hasattr(tree_module,'CachedPoints'),'existing-CachedPoints-required')
 decoder=decoder_module.Decoder(Path(root),quiescent=True)
 seen={}
 class CapturedPoints(tree_module.CachedPoints):
  def obj(self, h):
   raw=decoder.checked(Path(root)/'objects'/h[:2]/h,h)
   need(len(raw)<=MiB,'existing-object-bound')
   seen[h]=dict(bytes=len(raw),sha256=sha(raw),b64=__import__('base64').b64encode(raw).decode())
   return raw
 selected=strict(read(Path(root)/'SELECTED.json',4096),4096)
 need(selected==SELECTED,'geometry-exact-SELECTED')
 publication=strict(decoder.checked(Path(root)/'publications'/
          (selected['publication']+'.json'),selected['digest']),128*MiB)
 need(publication['sequence']+1==101007,'geometry-exact-H')
 rows=[]
 for request in requests:
  points=CapturedPoints(root)
  outcome=points.point(publication['outcomes'],sha(request['id'].encode()))
  if outcome is not None:
   membership=points.point(publication['membership'],outcome['selected']['publication'])
   need(membership==outcome['selected']['digest'] and outcome['request']==request['id'],
     'geometry-outcome-membership')
   result=decoder.checked(Path(root)/'results'/(outcome['result_hash']+'.json'),
           outcome['result_hash'])
   value=strict(result,128*MiB)
   need(value.get('request')==request['id'] and value.get('outcome')=='accepted',
     'geometry-exact-outcome')
   result_bytes=len(canonical(value))
  else:
   result_bytes=128  # Both typed nonacceptance variants fit this source grammar.
  need(len(points.cache)<=128,'point-two-paths-bound')
  rows.append(dict(category=request['category'],id=request['id'],
       route_objects=sorted(points.cache),result_bytes=result_bytes,
       unique_objects_bound=len(points.cache)+3))
 receipt=dict(version=1,root=str(root),selected=selected,actual_H=101007,
     publication_b64=__import__('base64').b64encode(decoder.checked(Path(root)/'publications'/
     (selected['publication']+'.json'),selected['digest'])).decode(),
     decoder_sha256=bindings['oracle']['sha256'],
     tree_sha256=bindings['tree_decoder']['sha256'],objects=seen,rows=rows)
 receipt['sha256']=sha(canonical(receipt))
 return receipt

def geometry_bounds(proof, oracle_sha, tree_sha):
 
 if proof is None:
  return dict(per_request={c:131 for c,_ in REQUESTS},warm_union=786,
     method='source-MAX_HEIGHT63',proof_sha256=None)
 closed(proof,('version','root','selected','actual_H','decoder_sha256','tree_sha256',
      'objects','rows','sha256','publication_b64'),'geometry')
 copy=dict(proof); claimed=copy.pop('sha256')
 need(hex64(claimed) and sha(canonical(copy))==claimed and proof['version']==1 and
   proof['selected']==SELECTED and proof['actual_H']==101007 and
   proof['decoder_sha256']==oracle_sha and proof['tree_sha256']==tree_sha,
   'geometry-source-and-content-binding')
 need(type(proof['objects']) is dict and len(proof['objects'])<=786,'geometry-object-count')
 need(type(proof['rows']) is list and len(proof['rows'])==6,'geometry-six-rows')
 per={}
 for expected,row in zip(REQUESTS,proof['rows']):
  closed(row,('category','id','route_objects','result_bytes','unique_objects_bound'),'geometry-row')
  need((row['category'],row['id'])==expected,'geometry-ID')
  need(type(row['route_objects']) is list and len(row['route_objects'])<=128 and
    len(set(row['route_objects']))==len(row['route_objects']) and
    all(h in proof['objects'] and hex64(h) for h in row['route_objects']),'geometry-route')
  need(row['unique_objects_bound']==len(row['route_objects'])+3,'geometry-object-expression')
  integer(row['result_bytes'],1,128*MiB,'geometry-result')
  per[row['category']]=row['unique_objects_bound']
 return dict(per_request=per,warm_union=min(786,sum(per.values())),
    method='recomputed-shared-decoder-point-routes',proof_sha256=claimed)

def verify_embedded_point_geometry(proof, tree_binding, oracle_sha):
 
 if proof is None:return geometry_bounds(None,oracle_sha,tree_binding['sha256'])
 preliminary=geometry_bounds(proof,oracle_sha,tree_binding['sha256'])
 import base64
 publication_raw=base64.b64decode(proof['publication_b64'],validate=True)
 need(sha(publication_raw)==SELECTED['digest'],'embedded-selected-publication-hash')
 publication=strict(publication_raw,128*MiB)
 need(publication['id']==SELECTED['publication'] and publication['sequence']+1==101007,
   'embedded-selected-H')
 module=load_source(tree_binding,'embedded_existing_CachedPoints')
 objects={}
 for h,row in proof['objects'].items():
  closed(row,('bytes','sha256','b64'),'embedded-object')
  raw=base64.b64decode(row['b64'],validate=True)
  need(len(raw)==row['bytes']<=MiB and sha(raw)==row['sha256']==h,'embedded-C1-hash')
  objects[h]=raw
 class EmbeddedPoints(module.CachedPoints):
  def obj(self,h):
   need(h in objects,'missing-authenticated-route-object')
   return objects[h]
 per={};used=set()
 for category,request in REQUESTS:
  points=EmbeddedPoints(proof['root'])
  outcome=points.point(publication['outcomes'],sha(request.encode()))
  if outcome is not None:
   need(outcome['request']==request and
     points.point(publication['membership'],outcome['selected']['publication'])==
     outcome['selected']['digest'],'embedded-accepted-membership')
  used.update(points.cache)
  per[category]=len(points.cache)+3
 need(per==preliminary['per_request'],'claimed-route-geometry-must-recompute')
 return dict(per_request=per,warm_union=min(786,len(used)+18),
  method='embedded-SHA-bound-existing-CachedPoints',proof_sha256=proof['sha256'])

def metrics_width(scope, *, unique_pages, action, max_work_saves):
 fields={}
 for name,kind in scope['schema'].items():
  if kind in NUMERIC_METRIC_TYPES:
   fields[name]=NUMERIC_METRIC_TYPES[kind]
  elif name=='unique_pages':
   fields[name]=json_array_width(unique_pages,66)
  elif name=='ns':
   fields[name]=map_width(scope['timing'],39)
  elif kind=='Vec<u64>':
   need(name in ('physical_upgrade_holds_ns','physical_activation_holds_ns',
        'physical_work_holds_ns'),'unknown-vector-scope')
   count=max_work_saves if name=='physical_work_holds_ns' else 2
   fields[name]=json_array_width(count,20)
  elif kind=='BTreeMap<String,u64>':
   if action in ('resolve','lookup-bench','setup','convert','cleanup'):
    fields[name]=2
   else:
    fields[name]=map_width(('D','E','R','L','C','X','B','S'),20)
  else:
   raise RuntimeError('unhandled-Metrics:'+name)
 return json_object_width(fields)

def result_width(action, *, outcomes, max_packs=64):
 if action=='resolve':
  need(type(outcomes) is dict and outcomes,'exact-result-shapes')
  return max(len(canonical(v)) for v in outcomes.values())
 if action=='lookup-bench':
  row=json_object_width(dict(category=unknown_string_width(16),id=ascii_string_width(64),
   first_lookup_ns=39,warm_ns=json_array_width(500,39),warm_median_ns=39,
   warm_p95_ns=39,warm_min_ns=39,warm_max_ns=39,result_digest=66,opens=20,
   pack_storage_bytes=20,decoded_record_bytes=20,index_cache_hits=20))
  return json_object_width(dict(rows=json_array_width(6,row),
   cache_qualification=unknown_string_width(128)))
 if action=='setup':
  return len(canonical({'status':'disposable'}))
 if action=='publish':
  selected=json_object_width(dict(publication=ascii_string_width(99),digest=66))
  accepted=json_object_width(dict(outcome=ascii_string_width(8),request=ascii_string_width(64),
   publication=ascii_string_width(99),state_digest=66,changed=2,removed=2))
  return json_object_width(dict(status=ascii_string_width(9),result=accepted,selected=selected,
   delivery_error=4,readable=ascii_string_width(4)))
 if action=='convert':
  generation=json_object_width(dict(id=ascii_string_width(99),ordinal=20,plan=66))
  return json_object_width(dict(status=ascii_string_width(10),catalog=66,generation=generation,
   discovery=4,forced_correctness=5,reserved_bytes=20,used_bytes=20,
   successfully_deleted_bytes_recorded=20,historical_allocation_owners_unchanged=5))
 if action=='cleanup':
  fields=('status','catalog','generation','candidates','batch_ns','cooperative_limit_ms',
   'completion_basis','candidate_limit','blocked_checks','pending_loose_jobs',
   'pending_old_containers','pending_metadata','protected_metadata_until_successor',
   'retained_generations','successfully_deleted_bytes_recorded','deleted_this_batch_bytes',
   'deleted_this_batch_files','reconciled_absent_without_credit','retained_loose_control_records',
   'retained_loose_control_bytes','native_allocation_credit','historical_allocation_owners_unchanged',
   'pending_aborted_owners','aborted_owner_adoption_backpressure','maintenance_deferral',
   'completed','selected_authority','legacy_whole_index_indivisible')
  return json_object_width({k:unknown_string_width(256) if k in
   ('status','completion_basis','maintenance_deferral','selected_authority') else
   json_object_width(dict(id=ascii_string_width(99),ordinal=20,plan=66)) if k=='generation'
   else 66 if k=='catalog' else 39 for k in fields})
 raise RuntimeError('unsupported-action:'+action)

def stdout_width(action, scope, geometry, outcomes):
 unique=(geometry['warm_union'] if action=='lookup-bench' else
   max(geometry['per_request'].values()) if action=='resolve' else
   1024 if action=='publish' else 131)
 metrics=metrics_width(scope,unique_pages=unique,action=action,
        max_work_saves=1002 if action=='cleanup' else 8)
 result=result_width(action,outcomes=outcomes)
 value=json_object_width(dict(status=ascii_string_width(2),result=result,error=4,
         metrics=metrics,wall_ns=39,peak_memory_bytes=20))
 return value+1

def oracle_output_bound(shape):
 
 closed(shape,('required_paths','path_utf8_bytes','birth_claims','owned_directories',
      'retained_generations','abandoned_owners','descriptor_count','discovery_none'),
     'oracle-output-shape')
 need(shape['discovery_none'] is True,'long-convert-discovery-must-remain-None')
 paths=integer(shape['required_paths'],1,4096,'oracle-path-count')
 name=integer(shape['path_utf8_bytes'],1,4096,'oracle-path-name')
 claims=integer(shape['birth_claims'],1,64,'oracle-births')
 dirs=integer(shape['owned_directories'],0,64,'oracle-dirs')
 older=integer(shape['retained_generations'],0,64,'oracle-older')
 owners=integer(shape['abandoned_owners'],0,64,'oracle-abandoned')
 descriptors=integer(shape['descriptor_count'],1,64,'oracle-packs')
 need(paths>=4*64+20 and name>=175 and claims>=1 and descriptors==64 and
   older==owners==dirs==0, 'fresh-legacy-force-convert-source-geometry-floor')
 path_list=json_array_width(paths,unknown_string_width(name)+32)
 descriptor_audit=json_object_width(dict(path=unknown_string_width(name),index_sha256=66,
  root_page_sha256=66,height=20,pages=20,index_bytes=20,records=20,
  exact_whole_index_preorder_tree_authenticated=5))+256
 birth_list=json_array_width(claims,ascii_string_width(99)+32)
 owned_list=json_array_width(dirs,json_object_width(dict(path=unknown_string_width(name),
  present=5,children=20,empty_only_removal=5))+128)
 journal_owner=json_object_width(dict(id=ascii_string_width(99),catalog=66,plan=66,
  pending_owned_refs=json_array_width(2,unknown_string_width(name))))+512
 journal=4096+json_array_width(owners,journal_owner)+json_array_width(older,512)
 fixed=89*(2+6*128+1+2+6*256+32)+8192
 raw=fixed+path_list+birth_list+owned_list+journal+json_array_width(descriptors,descriptor_audit)
 return dict(raw_bytes=raw,expression=dict(fixed=fixed,paths=path_list,births=birth_list,
    owned=owned_list,journal=journal,audits=json_array_width(descriptors,descriptor_audit)),
    shape=dict(shape),format='full-canonical-v5-pretty-result')


def native_meta_schema(path_bytes,root_bytes):
 u64={'uint':64};a=lambda n:{'ascii':n};o=lambda x:{'object':x}
 ref=o(dict(path=a(path_bytes),offset=u64,bytes=u64,sha256=a(64)))
 raw=o(dict(schema={'literal':CAPTURE_API},path={'string':path_bytes},run=a(32),call=a(32),
  begin=u64,end=u64,frames=u64,bytes=u64,sha256=a(64)))
 return o(dict(schema={'literal':'native-call-meta-v2'},call=a(32),run=a(32),raw_ref=raw,
  process_started_ns=u64,process_done_ns=u64,spawn_total_ns=u64,source_pin=a(40),
  binary_sha256=a(64),producer_kind=a(32),producer_source_pin=a(40),
  identity=o(dict(pid={'uint':32},creation=u64,executable_sha256=a(64))),
  source_proof_ref=ref,image_proof_ref=ref,qpc_frequency=u64,clock_bridge_ref=ref,
  closure_ref=ref,case_id=a(128),scope_id=a(128),case_scope=a(32)))

def launch_width(rows,kind,pin,executable_sha):
 need(type(rows) is list and 0<len(rows)<=128,'finite-literal-launch-catalog')
 widths=[]
 for row in rows:
  closed(row,('argv','executable_sha256','producer_kind','producer_source_pin',
       'source_files','startup_gate_sha256'),'exact-launch-data')
  need(type(row['argv']) is list and 0<len(row['argv'])<=64 and
    all(type(a) is str and len(a.encode())<=4096 for a in row['argv']) and
    row['executable_sha256']==executable_sha and row['producer_kind']==kind and
    row['producer_source_pin']==pin,'exact-launch-producer')
  need(type(row['source_files']) is list and 0<len(row['source_files'])<=64 and
       (row['startup_gate_sha256'] is None or hex64(row['startup_gate_sha256'])),'launch-source-list')
  for source in row['source_files']:
   closed(source,('path','sha256'),'launch-source')
   need(Path(source['path']).is_absolute() and hex64(source['sha256']),'launch-source-reference')
  widths.append(json_object_width(dict(v=1,kind=len(canonical('launch')),call=34,seq=20,
    **{k:len(canonical(v)) for k,v in row.items()}))+1)
 return max(widths)

def native_history_action(name):
 return name in ('resolve','lookup-bench-one','paced-reader-one-query-batch','publish',
  'build','cleanup','convert','quota','terminal-v2-enable','terminal-v2-release',
  'full-oracle','mapping-query','backlog-query')

def body_ref_width(capture):
 return json_object_width(dict(path=ascii_string_width(capture['dimensions']['tape_path_bytes']),
  offset=20,bytes=20,sha256=66))

def case_native_count(phase):
 return 100*phase['publishers']+40+(400 if phase['publishers']==4 else 0)+(
  201+(198 if phase['publishers']==1 else 0)+2 if phase['online'] else 0)+1

def capture_v2_schema(source_bindings, root_bytes, env_width, tape_path_bytes):
 
 u64={'uint':64};u32={'uint':32};boolean={'oneof':[{'literal':True},{'literal':False}]}
 text=lambda n:{'string':n}
 obj=lambda v:{'object':v}
 base={'v':{'literal':2}}
 begin=obj(dict(base,kind={'literal':'begin'},call={'ascii':32},root={'ascii':root_bytes},
  action={'ascii':32},role={'ascii':16},env={'bytes':env_width},
  input_n=u64,input_sha={'ascii':64},input_scope={'literal':'planned'},
  pid={'oneof':[u32,{'literal':None}]},creation={'oneof':[u64,{'literal':None}]},qpc_start=u64))
 start=obj(dict(base,kind={'literal':'start'},call={'ascii':32},seq=u64,pid=u32,
  creation=u64,qpc=u64,rss=u32,executable_sha256={'ascii':64}))
 data=obj(dict(base,kind={'literal':'data'},call={'ascii':32},stream={'ascii':3},
  offset=u64,n=u64,sha={'ascii':64},b64={'literal':''}))
 rss=obj(dict(base,kind={'literal':'rss'},call={'ascii':32},pid=u32,creation=u64,
  qpc=u64,rss=u32,peak=u32,role={'ascii':16},phase={'ascii':9}))
 end=obj(dict(base,kind={'literal':'end'},call={'ascii':32},exit={'uint':32},
  in_n=u64,out_n=u64,err_n=u64,in_sha={'ascii':64},out_sha={'ascii':64},
  err_sha={'ascii':64},input_complete=boolean,out_eof=boolean,err_eof=boolean,
  rss_n=u64,peak=u32,qpc_end=u64,first_write_qpc={'oneof':[u64,{'literal':None}]},native_closed=boolean,io_closed=boolean,
  callbacks_joined=boolean,intent={'literal':None},violations={'literal':[]}))
 for frame in (begin,data,rss,end):frame['object']['seq']=u64
 ref=obj(dict(schema={'literal':'single-raw-ref-v1'},path={'ascii':tape_path_bytes},
  run={'ascii':32},call={'ascii':32},begin=u64,end=u64,frames=u64,bytes=u64,sha256={'ascii':64}))
 header=obj(dict(base,kind={'literal':'stream'},run={'ascii':32},source_pin={'ascii':40},
  binary_sha={'ascii':64},guard_sha={'ascii':64},driver_sha={'ascii':64},profile_sha={'ascii':64}))
 poll=obj(dict(kind={'literal':'fast-native-poll'},observed_ns=u64,pid=u32,
  creation_time=u64,rss=u32,guard_nonce={'ascii':32},receipt_sha256={'ascii':64}))
 member=obj(dict(pid=u32,creation_time=u64,rss=u32,peak=u32))
 supervisor=obj(dict(pid=u32,creation_time=u64,rss=u32,peak=u32))
 qpc=obj(dict(event={'literal':'rss-sample'},epoch={'float64':None},perf_counter_ns=u64,
  completed_perf_counter_ns=u64,collection_duration_seconds={'float64':None},
  completion_interval_seconds={'float64':None},monotonic={'float64':None},
  monotonic_interval_seconds={'float64':None},interval_seconds={'float64':None},
  sample_complete={'literal':True},job_rss=u32,
  processes={'array':{'count':10,'item':member}},supervisor_rss=u32,
  supervisors={'array':{'count':16,'item':supervisor}}))
 fatal=obj(dict(kind={'literal':'fatal'},run={'ascii':32},call={'ascii':32},
  source_pin={'ascii':40},budget_sha={'ascii':64},reason=text(2001),
  violations={'array':{'count':16,'item':text(2001)}},
  native_closed=boolean,io_closed=boolean,callbacks_joined=boolean))
 return dict(api=CAPTURE_API,invoke_api=INVOKE_API,encoding='jsonl-base64',
  chunk_bytes=16384,writer_max_chunk_bytes=65536,max_live_writers=10,rss_interval_ns=10000000,
  qpc_interval_ns=20000000,poll_interval_ns=250000000,
  dimensions=dict(root_bytes=root_bytes,env_width=env_width,tape_path_bytes=tape_path_bytes),
  source_bindings=source_bindings,reference_schema=ref,raw_frame_schema=data,
  meta_schema=native_meta_schema(tape_path_bytes,root_bytes),
  call_schema={'object':dict(header=header,begin=begin,start=start,end=end)},rss_schema=rss,poll_schema=poll,
  qpc_schema=qpc,final_schema=fatal)

def literal_capture_shape(value):
 
 closed(value,('api','invoke_api','encoding','chunk_bytes','max_live_writers',
      'rss_interval_ns','qpc_interval_ns','poll_interval_ns','writer_max_chunk_bytes',
      'dimensions','source_bindings','reference_schema','raw_frame_schema','call_schema',
      'rss_schema','poll_schema','qpc_schema','final_schema','meta_schema'),
     'capture-contract')
 d=closed(value['dimensions'],('root_bytes','env_width','tape_path_bytes'),'wire-dimensions')
 integer(d['root_bytes'],128,4096,'all-planned-root-width')
 integer(d['env_width'],8192,8192,'native-hard-env-cap')
 integer(d['tape_path_bytes'],128,4096,'actual-capture-path-width')
 expected=capture_v2_schema(value['source_bindings'],**d)
 need(value==expected,'literal-source-owned-v2-wire-schema')
 return value


def capture_contract(value):
 literal_capture_shape(value)
 need(type(value['source_bindings']) is dict and
   {'evidence','transport','growth','native','deadlines'}<=set(value['source_bindings']),
   'complete-native-producer-source')
 texts={k:bound_source(row,2*MiB).decode() for k,row in value['source_bindings'].items()}
 need('COALESCED_STREAM_V1 = True' in texts['evidence'],
   'reviewed-coalescing-producer-required-before-ceil-bound')
 return value

def schema_width(schema):
 
 need(type(schema) is dict and len(schema)==1,'wire-type-expression')
 kind,value=next(iter(schema.items()))
 if kind=='literal': return len(canonical(value))
 if kind=='uint':
  integer(value,1,128,'uint-bits')
  return len(str((1<<value)-1))
 if kind=='float64': return 32
 if kind=='ascii': return ascii_string_width(value)
 if kind=='string': return unknown_string_width(value)
 if kind=='bytes':
  integer(value,0,32*MiB,'binary-width')
  return value
 if kind=='array':
  closed(value,('count','item'),'array-type')
  return json_array_width(value['count'],schema_width(value['item']))
 if kind=='object':
  need(type(value) is dict and len(value)<=512,'finite-wire-fields')
  return json_object_width({k:schema_width(v) for k,v in value.items()})
 if kind=='oneof':
  need(type(value) is list and 0<len(value)<=16,'finite-wire-union')
  return max(schema_width(v) for v in value)
 raise RuntimeError('unknown-wire-type:'+kind)

def frame_bound(payload, contract):
 integer(payload,0,2**63-1,'raw-payload')
 n=(payload+contract['chunk_bytes']-1)//contract['chunk_bytes']
 header=schema_width(contract['raw_frame_schema'])+1
 if contract['encoding']=='jsonl-base64':
  return 4*((payload+2)//3)+4*n+n*header,n
 return payload+n*header,n

def sample_bound(duration_ns, interval_ns, forced):
 integer(duration_ns,0,2**63-1,'active-ns')
 integer(interval_ns,1,10**9,'sample-interval')
 integer(forced,0,10**8,'forced-samples')
 return (duration_ns+interval_ns-1)//interval_ns+forced

def _long_inputs(c):
 p=closed(c['profile_inputs'],('engine_sources','capture','call_seconds','stage_seconds',
  'max_job_members','max_supervisors','oracle_shape','geometry','tree_decoder',
  'retained_artifacts','available','max_cleanup_batches','directory_reserve','launch_profiles'),
  'long-profile-inputs')
 scope=engine_scope(p['engine_sources'])
 capture=capture_contract(p['capture'])
 need(all(str(root).isascii() and len(str(root).encode())<=capture['dimensions']['root_bytes'] for root in c['roots'].values()),
   'wire-root-width-covers-all-long-paths')
 bound_source(c['bindings']['oracle'])
 tree_sha=bound_source(p['tree_decoder'])
 g=p['geometry']
 geometry=verify_embedded_point_geometry(g,p['tree_decoder'],c['bindings']['oracle']['sha256'])
 expected=strict(bound_source(c['bindings']['lookup_expectations'],MiB),MiB)
 need(expected['selected']==SELECTED and expected['actual_H']==101007 and
   expected['requests']==[dict(category=k,id=v) for k,v in REQUESTS],
   'authenticated-exact-lookup-inputs')
 outcomes=expected['expected_outcomes']
 need(type(outcomes) is dict and set(outcomes)=={k for k,_ in REQUESTS},
   'six-outcome-shapes')
 for value in outcomes.values():
  need(type(value) is dict and value.get('outcome') in
    ('accepted','absent-as-of','cancelled-fenced'),'typed-outcomes')
 return p,scope,capture,geometry,outcomes

def _basic_profiles(c,p,scope,capture,geometry,outcomes):
 oracle=oracle_output_bound(p['oracle_shape'])
 need(type(p['call_seconds']) is dict and set(p['call_seconds'])==
   {'resolve','lookup-bench','setup','convert','publish','cleanup','oracle'},
   'all-call-deadlines')
 result={}
 root=max(len(canonical(c['roots']['source'])),len(canonical(c['roots']['destination'])))
 inputs={
  'resolve':root+len(canonical({'action':'resolve','request_id':SUCCESSOR}))+32,
  'lookup-bench':root+len(canonical({'action':'lookup-bench','count':500,
   'requests':[dict(category=k,id=v) for k,v in REQUESTS]}))+32,
  'setup':root+len(canonical({'action':'lifecycle','operation':'setup'}))+32,
  'convert':root+len(canonical({'action':'packing','operation':'convert',
   'force':True,'cleanup':False,'reserve_bytes':335544320}))+32,
  'publish':root+len(canonical({'action':'publish','request':
   dict(request=SUCCESSOR,base=SELECTED,ops=[],packet=False)}))+32,
  'cleanup':root+len(canonical({'action':'packing','operation':'cleanup',
   'max_candidates':1000,'time_budget_ms':50}))+32,
  'oracle':0}
 for action in inputs:
  integer(p['call_seconds'][action],1,2400,'finite-call-deadline')
  stdout=4096 if action=='oracle' else stdout_width(action,scope,geometry,outcomes)
  launch=launch_width(p['launch_profiles'][action],
   'checker' if action=='oracle' else 'engine',c['final_pin'],
   c['bindings']['python' if action=='oracle' else 'engine']['sha256'])
  result[action]=dict(input=inputs[action],stdout=stdout,stderr=2001,
   receipt=schema_width(capture['call_schema'])+1+launch,
   output=oracle['raw_bytes'] if action=='oracle' else 0)
 return result,oracle

def operation_schedule(c,p):
 k=integer(p['max_cleanup_batches'],1,100000,'explicit-cleanup-bound')
 need(k==c['limits']['cleanup_batches'],'same-cleanup-bound')
 ops=[]
 def add(stage,action,count):
  integer(count,0,100000,'operation-count')
  for n in range(count):
   ops.append(dict(ordinal=len(ops),stage=stage,action=action,index=n,
       seconds=p['call_seconds'][action]))
 add('legacy-oracle','oracle',1)
 add('legacy','lookup-bench',1);add('legacy','resolve',300)
 add('setup','setup',1);add('convert','convert',1);add('converted-oracle','oracle',1)
 add('paged','lookup-bench',1);add('paged','resolve',300)
 add('cleanup','cleanup',k);add('cleaned-oracle','oracle',1)
 add('restored-oracle','oracle',1);add('successor','publish',1);add('successor','resolve',1)
 add('successor-oracle','oracle',1)
 need(len(ops)==611+k,'all-engine-and-five-oracle-operations')
 return ops

def _sink(name,category,rows,width,files=1,directories=0):
 integer(rows,0,10**9,'sink-rows')
 integer(width,0,2**63-1,'sink-width')
 integer(files,0,10**7,'sink-files')
 return dict(name=name,category=category,rows=rows,row_bytes=width,bytes=rows*width,
    files=files,directories=directories)

def long_profile_model(c):
 p,scope,capture,geometry,outcomes=_long_inputs(c)
 profiles,oracle=_basic_profiles(c,p,scope,capture,geometry,outcomes)
 ops=operation_schedule(c,p)
 total_ns=integer(p['stage_seconds'],1,86400,'whole-stage-seconds')*10**9
 integer(p['max_job_members'],1,12,'whole-Job-member-bound')
 integer(p['max_supervisors'],0,16,'supervisor-bound')
 need(capture['max_live_writers']>=2,'duplex-stream-inflight-bound')
 reference_width=schema_width(capture['reference_schema'])+1
 all_rows=[]; encoded_raw=0; raw_frames=0; raw_json_calls=0
 for op in ops:
  prof=profiles[op['action']]
  pieces=[frame_bound(prof[k],capture) for k in ('input','stdout','stderr')]
  width,frames=sum(x[0] for x in pieces),sum(x[1] for x in pieces)
  encoded_raw+=width;raw_frames+=frames;raw_json_calls+=prof['receipt']
 driver_active_ns=min(total_ns,sum(op['seconds']*10**9 for op in ops))
 native_samples=sample_bound(driver_active_ns,capture['rss_interval_ns'],2*len(ops))
 qpc_samples=sample_bound(total_ns,capture['qpc_interval_ns'],4)
 host_polls=sample_bound(total_ns,capture['poll_interval_ns'],2*len(ops))
 all_rows.append(_sink('lossless-native-output','E',1,encoded_raw))
 all_rows.append(_sink('native-call-reference-rows','E',1,raw_json_calls,0))
 all_rows.append(_sink('native-call-meta-v2','E',len(ops),schema_width(capture['meta_schema'])+1,1))
 all_rows.append(_sink('native-RSS','E',native_samples,
        schema_width(capture['rss_schema'])+1,0))
 all_rows.append(_sink('host-polls','E',host_polls,
        schema_width(capture['poll_schema'])+1,0))
 all_rows.append(_sink('outer-QPC','E',qpc_samples,
        schema_width(capture['qpc_schema'])+1,1))
 all_rows.append(_sink('harness-and-lane-refs','E',4*len(ops),reference_width,1))

 tail_payload=2*capture['writer_max_chunk_bytes']*capture['max_live_writers']
 tail_bytes,tail_frames=frame_bound(tail_payload,capture)
 footer=schema_width(capture['final_schema'])+1
 all_rows.append(_sink('one-fatal-tail-and-drain','E',1,
        tail_bytes+(2*capture['max_live_writers']+4)*footer,1))
 all_rows.append(_sink('five-full-oracles','M',5,profiles['oracle']['output'],5,5))
 all_rows.append(_sink('five-normalized-oracles','M',5,16384,5))
 all_rows.append(_sink('five-inventory-digest-proofs','M',5,4096,5))
 all_rows.append(_sink('known-nonhistory-and-successor','M',1,2*96*1024,2))
 all_rows.append(_sink('successor-baseline','M',1,4096,1))
 all_rows.append(_sink('stream-subset','M',1,2*MiB,12,1))
 all_rows.append(_sink('metadata-final-failure','M',3,16384,3))
 need(type(p['retained_artifacts']) is dict and set(p['retained_artifacts'])==
   {'samples','cleanup_index','result','failure','archive_proof','archive_failure',
    'forecast','admission','finalizer','census','source_geometry'},'all-extra-artifacts')
 for name,row in p['retained_artifacts'].items():
  closed(row,('bytes','files','directories'),'retained-artifact')
  for k in row:integer(row[k],0,2**63-1,'artifact-'+k)
  all_rows.append(dict(name=name,category='E',rows=1,row_bytes=row['bytes'],**row))
 e=sum(x['bytes'] for x in all_rows if x['category']=='E')
 ef=sum(x['files'] for x in all_rows if x['category']=='E')
 m=sum(x['bytes'] for x in all_rows if x['category']=='M')
 mf=sum(x['files'] for x in all_rows if x['category']=='M')
 need(m<=8*MiB and mf<=40,'M-five-oracles-stream-knownmaps-over-cap')
 available=closed(p['available'],('E_bytes','E_files','M_bytes','M_files'),
      'assigned-profile-available')
 for key in available:integer(available[key],0,2**63-1,'assigned-'+key)
 need(e<=available['E_bytes'] and ef<=available['E_files'] and
   m<=available['M_bytes']<=8*MiB and mf<=available['M_files']<=40,
   'whole-all-sink-allocation-shortfall')
 counts={}
 for op in ops:counts[op['action']]=counts.get(op['action'],0)+1
 need(counts==dict(resolve=601,**{'lookup-bench':2},setup=1,convert=1,publish=1,
       cleanup=p['max_cleanup_batches'],oracle=5),'exact-long-counts')
 return dict(api=API,capture_api=CAPTURE_API,invoke_api=INVOKE_API,
  profiles=profiles,expanded_schedule=ops,sinks=all_rows,counts=counts,
  E_bytes=e,E_files=ef,M_bytes=m,M_files=mf,
  native_samples=native_samples,qpc_samples=qpc_samples,host_polls=host_polls,
  failure_escrow=dict(payload_bytes=tail_payload,encoded_bytes=tail_bytes,
   frames=tail_frames,max_live_writers=capture['max_live_writers'],
   abort_first_terminal_failure=True,settle_only_after_writer_closure=True),
  source_scope=scope,geometry=geometry,oracle_output=oracle,
  planned_deletion_credit=0,future_expiry_credit=0,compression_ratio_credit=0)

def derive_long_forecast(config):
 model=long_profile_model(config)
 model['transport_profiles']={k:transport_profile(k,p,model,config)
         for k,p in model['profiles'].items()}
 model['profile_sha256']=sha(canonical(model))
 model['expanded_trace_sha256']=sha(canonical(model['expanded_schedule']))
 model['all_sink_membership_sha256']=sha(canonical(model['sinks']))
 return model

def transport_profile(action, profile, model, config):
 capture=config['profile_inputs']['capture']
 seconds=config['profile_inputs']['call_seconds'][action]
 rss=min(model['native_samples'],sample_bound(seconds*10**9,capture['rss_interval_ns'],2))
 refs=schema_width(capture['reference_schema'])+1
 chunks={k:frame_bound(profile[k],capture)[0] for k in ('input','stdout','stderr')}
 components=dict(v=2,input_frames=chunks['input'],stdout_frames=chunks['stdout'],
  stderr_frames=chunks['stderr'],rss_frames=rss*1024,
  call_header_footer=max(9216,profile['receipt'])+max(16384,profile['receipt']),reference_rows=4*refs,harness_projection=refs,
  helper_records=schema_width(capture['meta_schema'])+1,oracle_metadata=profile['output'],outer_guard=0,finalization=0)
 return dict(api_version=2,capture_schema=CAPTURE_API,stdout_bytes=profile['stdout'],
  stderr_bytes=profile['stderr'],rss_samples=rss,callback_seconds=.5,
  timeout_seconds=seconds,chunk_bytes=16384,
  writer_max_chunk_bytes=capture['writer_max_chunk_bytes'],
  failure_tail_bytes=2*capture['writer_max_chunk_bytes'],
  begin_max_bytes=max(9216,profile['receipt']),end_max_bytes=max(16384,profile['receipt']),
  reference_max_bytes=refs,projection_max_bytes=refs,
  all_sink_components=components,global_sample_reservation=model['native_samples'],
  source_operation=action,profile_sha256=sha(canonical(dict(action=action,profile=profile,
   capture=capture,source=model['source_scope']['source_sha256']))))

def long_profiles(config):
 return derive_long_forecast(config)['profiles']

def fixed_reader_capacity(source_pin, probe_sha, clock_sha, method_sha):
 need(re.fullmatch('[0-9a-f]{40}',source_pin) and
   all(hex64(x) for x in (probe_sha,clock_sha,method_sha)),'reader-source-bindings')
 streams=[]
 for slot in range(4):
  offset,interval=(16000000000,2900000) if slot==0 else (0,4500000)
  streams.append(dict(id=slot,batches=10,queries_per_batch=1000,first_queries=10,
   offset_ns=offset,interval_ns=interval,last_native_query_due_ns=offset+9999*interval))
 return dict(source_pin=source_pin,probe_sha256=probe_sha,clock_handshake_sha256=clock_sha,
  method_sha256=method_sha,publisher_horizon_ns=44000000000,
  reader_deadline_ns=48000000000,old_pin_end_bound_ns=23600000000,
  header_bytes=2048,milestones=10,streams=streams,
  category_map=[('early','middle','recent','missing','cancelled','lostreply')[n%6]
       for n in range(40)],exhaustion='fail-no-renewal-no-skipped-query',
  warm_horizon_ns=45000000000)

def history_schedule(plan):
 
 need(plan['total_accepted']==1600 and plan['max_offers']==4000 and
   plan['max_sealed_losers']==2400 and len(plan['phases'])==16,'full-history-plan')
 operations=[]
 def add(stage,action,count):
  for n in range(integer(count,0,10**7,'history-op-count')):
   operations.append(dict(ordinal=len(operations),stage=stage,action=action,index=n))
 for identity in ('H1','H10'):
  add('lookup-'+identity,'resolve',600)
  add('lookup-'+identity,'lookup-bench-one',12)
  add('lookup-'+identity,'convert',1)
  add('lookup-'+identity,'cleanup',160)
  add('lookup-'+identity,'full-oracle',3)
  add('lookup-'+identity,'terminal-v2-enable',1)
  add('lookup-'+identity,'quota',1)
 for phase in plan['phases']:
  label=phase['id'];pubs=phase['publishers']
  if phase['cycle']==1:
   add('bootstrap-'+label,'build',1)
   add('bootstrap-'+label,'cleanup',160)
   add('bootstrap-'+label,'full-oracle',1)
  add(label,'publish',100*pubs)
  if pubs==4:add(label,'quota',100)
  add(label,'paced-reader-one-query-batch',40)
  add(label,'native-origin',1)
  add(label,'reader-ready',4)
  if pubs==4:
   add(label,'terminal-v2-release',300)
   add(label,'publisher-ready',400)
  if phase['online']:
   add(label,'build',100)
   add(label,'cleanup',99)
   add(label,'cleanup',2)
   add(label,'backlog-query',2)
   add(label,'selected-state',100)
   if pubs==1:
    add(label,'mapping-query',198)
    add(label,'selected-state',99)
  add(label,'full-oracle',1)
  add(label,'owned-drain',1)
  add(label,'phase-checkpoint',1)
 return operations


def reader_capacity(plan, bindings):
 need(plan['total_accepted']==1600 and len(plan['phases'])==16,'reader-full-plan')
 return fixed_reader_capacity(bindings['source_pin'],bindings['reader_probe_sha256'],
         bindings['clock_handshake_sha256'],bindings['reader_probe_sha256'])

def cow_insert_bound(height, key_bytes, value_bytes):
 
 h=integer(height,0,63,'C1-height')
 k=integer(key_bytes,1,1024,'C1-key')
 v=integer(value_bytes,0,8192,'C1-value')
 leaf=2*7+65*(2+k+4+v)
 child=32+2+k+2+k
 branches=h*(2*8+33*child)
 root=8+2*child
 return dict(bytes=leaf+branches+root,files=2*(h+1)+1,
    expression=dict(leaf=leaf,branches=branches,possible_root=root))

def authenticate_history_geometry(plan, bindings):
 
 decoder_module=load_source(bindings['oracle'],'history_profile_v5')
 tree_module=load_source(bindings['tree_decoder'],'history_profile_tree')
 result={}
 for identity,donor in plan['donors'].items():
  root=Path(donor['root'])
  selected=strict(read(root/'SELECTED.json',4096),4096)
  need(selected==donor['selected'],'history-geometry-selected')
  d=decoder_module.Decoder(root,quiescent=True)
  publication=strict(d.checked(root/'publications'/(selected['publication']+'.json'),
          selected['digest']),128*MiB)
  need(publication['sequence']+1==donor['actual_H'],'history-geometry-H')
  checked={};captured_bytes=[0]
  class CapturedTree(tree_module.Tree):
   def obj(self,h):
    need(len(checked)<4*donor['actual_H']+64 or h in checked,
      'finite-existing-history-node-walk')
    raw=d.checked(root/'objects'/h[:2]/h,h)
    need(len(raw)<=MiB,'existing-C1-object-size')
    if h not in checked:
     captured_bytes[0]+=len(raw);need(captured_bytes[0]<=24*MiB,'bounded-witness-RAM')
    checked[h]=dict(bytes=len(raw),sha256=sha(raw),b64=__import__('base64').b64encode(raw).decode())
    return raw
  trees={}
  for name in ('membership','outcomes'):
   tree=CapturedTree(root)
   values=tree.entries(publication[name])
   need(len(values)==donor['actual_H']-1,'all-current-history-index-entries')
   trees[name]=dict(height=tree.height,pages=tree.pages,leaves=tree.leaves,entries=len(values),
        max_value=max((len(canonical(v)) for v in values.values()),default=0),
        max_key=max((len(k.encode()) for k in values),default=0),
        min_occupancy=min(tree.occupancy),max_occupancy=max(tree.occupancy))
  class CapturedPoints(tree_module.CachedPoints):
   obj=CapturedTree.obj
  points=CapturedPoints(root)
  static_proofs={name:points.point(publication['projection'],name)
        for name in ('packet/context.md','packet/provenance.json')}
  store_raw=read(root/'STORE.json',65536)
  raw_bytes=records=key_bytes=0;roles={}
  for key,entry in d.entries.items():
   descriptor,offset,encoded,length,digest=entry
   integer(length,0,8*MiB,'authenticated-index-raw-length')
   role=descriptor['role']; row=roles.setdefault(role,dict(records=0,raw_bytes=0,key_bytes=0))
   row['records']+=1;row['raw_bytes']+=length;row['key_bytes']+=len(key.encode())
   records+=1;raw_bytes+=length;key_bytes+=len(key.encode())
  need(records<=64*65536,'catalog-record-cap')
  proof=dict(root=str(root),selected=selected,actual_H=donor['actual_H'],trees=trees,
       checked_objects=checked,raw_bytes=raw_bytes,records=records,key_bytes=key_bytes,
       roles=roles,descriptors=len(d.descriptors),store_sha256=sha(store_raw),
       store_b64=__import__('base64').b64encode(store_raw).decode(),static_proofs=static_proofs,
       decoder_sha256=bindings['oracle']['sha256'],tree_sha256=bindings['tree_decoder']['sha256'],
       publication_b64=__import__('base64').b64encode(d.checked(root/'publications'/
      (selected['publication']+'.json'),selected['digest'])).decode())
  proof['sha256']=sha(canonical(proof));result[identity]=proof
 return result

def _history_physical(plan, geometry, source):
 
 need('pub const MAX_OBJECT:usize=1024*1024;' in source['lib.rs'] and
   'b"FNC1"' in source['disk.rs'] and
   'const LEAF_CAPACITY: usize = 64' in source['disk.rs'] and
   'const FANOUT: usize = 32' in source['disk.rs'],'C1-source-arithmetic')
 outcome=json_object_width(dict(request=258,intent=66,selected=json_object_width(
  dict(publication=101,digest=66)),result_hash=66,attempt=101))
 bounds=[]
 for proof in geometry.values():
  heights={}
  for name,t in proof['trees'].items():
   h=t['height'];leaves=t['leaves']
   # Each existing leaf can split after one insertion; each subsequent
   # leaf split needs at least32 new keys. Two new root heights require
   # at least32 leaves from height0, or512*2**(h-1) otherwise.
   future=leaves+min(800,leaves)+max(0,800-leaves)//32
   need(0<=h<=2 and future<(32 if h==0 else 512*2**(h-1)),
    'authenticated-800-insert-single-height-growth-bound')
   heights[name]=h+1
  membership=cow_insert_bound(heights['membership'],max(99,proof['trees']['membership']['max_key']),66)
  outcomes=cow_insert_bound(heights['outcomes'],64,max(outcome,proof['trees']['outcomes']['max_value']))
  pages=membership['files']+outcomes['files']
  candidate=16384+json_array_width(pages,66)
  body=membership['bytes']+outcomes['bytes']+candidate+4*16384
  bounds.append(dict(bytes=body,files=pages+8))
 per=max(row['bytes'] for row in bounds); files=max(row['files'] for row in bounds)
 representation=0; representation_files=0
 for proof in geometry.values():
  encoded=proof['raw_bytes']+(proof['raw_bytes']+255)//256+256*proof['records']
  indexes=2*(proof['key_bytes']+96*proof['records'])+64*4096
  representation+=encoded+indexes
  representation_files+=4*max(1,proof['descriptors'])+16
 retained_work=0
 terms={
  'accepted_writes':dict(bytes=1600*per,files=1600*files,directories=1600),
  'sealed_losers':dict(bytes=2400*(16384+json_array_width(16,66)),files=2400,directories=2400),
  'terminal_receipts':dict(bytes=2400*4096,files=2400,directories=1),
  'terminal_registry_and_atomic_stages':dict(bytes=(8+2)*65536,files=10,directories=0),
  'accepted_registry_stages':dict(bytes=8*65536,files=8,directories=0),
  'legacy_to_paged_coexistence':dict(bytes=representation,files=representation_files,directories=8),
  'rolling_and_consolidation_peak':dict(bytes=320*MiB,files=160,directories=4),
  'generation_pins_work_catalogs_journals':dict(bytes=64*128*1024+64*4096,files=256,directories=8),
  'frozen_inventories':dict(bytes=retained_work,files=0,directories=0),
  'oracle_metadata':dict(bytes=24*2*MiB,files=24*3,directories=24),
  'controls_and_zero_byte_files':dict(bytes=(3200+1188)*32+80*4096,
   files=3*(3200+1188)+80,directories=1616),
  'finalization_and_failed_prefixes':dict(bytes=16*MiB,files=256,directories=8)}
 return terms

def _derive_history_forecast(plan, bindings, components, data):
 
 closed(data,('capture','engine_sources','geometry_bindings','artifact_sinks',
     'available','stage_seconds','source_pin','capsule_sha256','geometry_witnesses','record_schemas','launch_profiles','checker_sha256'),
     'history-model-data')
 need(data['source_pin']==bindings['history']['source_pin'] and hex64(data['capsule_sha256']),
   'history-model-final-source')
 capture=capture_contract(data['capture'])
 need(all(d['root'].isascii() and len(d['root'].encode())<=capture['dimensions']['root_bytes'] for d in plan['donors'].values()),
   'wire-root-width-covers-history-roots')
 scope=engine_scope(data['engine_sources'])
 metric_names=actual_metric_names(scope)
 source={name:bound_source(row,2*MiB).decode() for name,row in data['engine_sources'].items()}
 geometry=verify_embedded_history_geometry(plan,data['geometry_bindings'],data['geometry_witnesses'])
 ops=history_schedule(plan)
 g=dict(per_request={k:131 for k in ('early','middle','recent','missing','cancelled','lost-reply')},
     warm_union=786)
 outcomes={k:dict(outcome='accepted',request='x'*64,publication='p'*99,
     state_digest='a'*64,changed=[],removed=[]) for k in g['per_request']}
 prof={}
 for name in {o['action'] for o in ops if native_history_action(o['action'])}:
  if name=='resolve':
   stdout=stdout_width('resolve',scope,g,outcomes)
  elif name in ('lookup-bench-one','paced-reader-one-query-batch'):
   warm=1000 if name=='paced-reader-one-query-batch' else 500
   row=json_object_width(dict(category=unknown_string_width(64),id=unknown_string_width(256),
    first_lookup_ns=39,warm_ns=json_array_width(warm,39),warm_median_ns=39,
    warm_p95_ns=39,warm_min_ns=39,warm_max_ns=39,result_digest=66,opens=20,
    pack_storage_bytes=20,decoded_record_bytes=20,index_cache_hits=20))
   result=json_object_width(dict(rows=json_array_width(1,row),
    concurrent_pacing=2048,cache_qualification=unknown_string_width(128)))
   stdout=json_object_width(dict(status=4,result=result,error=4,metrics=metrics_width(
    scope,unique_pages=131,action='lookup-bench',max_work_saves=2),
    wall_ns=39,peak_memory_bytes=20))+1
  elif name=='publish':
   stdout=stdout_width('publish',scope,g,outcomes)
  elif name in ('cleanup','convert','build'):
   stdout=stdout_width('cleanup' if name=='cleanup' else 'convert',scope,g,outcomes)+(32768 if name=='build' else 0)
  elif name=='full-oracle':
   stdout=4096
  elif name=='selected-state':
   stdout=128*1024+59*1024+2*4096
  else:
   result=65536+4096 if name=='quota' else 4096
   stdout=json_object_width(dict(status=4,result=result,error=4,metrics=metrics_width(
    scope,unique_pages=131,action='resolve',max_work_saves=4),wall_ns=39,peak_memory_bytes=20))+1
  checker=name in ('full-oracle','mapping-query','backlog-query')
  launch=launch_width(data['launch_profiles'][name],'checker' if checker else 'engine',
   bindings['history']['source_pin'],data['checker_sha256'] if checker else bindings['history']['binary_sha256'])
  prof[name]=dict(stdout=stdout,stderr=2001,input=0 if checker else 4096,
   call_record=schema_width(capture['call_schema'])+1+launch)
 raw=refs=0;counts={}
 for op in ops:
  if not native_history_action(op['action']):continue
  p=prof[op['action']]
  size=sum(frame_bound(p[k],capture)[0] for k in ('input','stdout','stderr'))
  raw+=size+p['call_record']
  refs+=4*(schema_width(capture['reference_schema'])+1)
  counts[op['action']]=counts.get(op['action'],0)+1
 stage=integer(data['stage_seconds'],1,86400,'history-whole-seconds')*10**9
 reader_ns=16*4*48000000000
 need(stage>=16*48000000000,'sixteen-fixed-reader-windows-fit-stage')
 other_ns=16*5*44000000000+max(0,stage-16*48000000000)
 native_n=sample_bound(reader_ns+other_ns,capture['rss_interval_ns'],2*sum(native_history_action(o['action']) for o in ops))
 qpc_n=sample_bound(stage,capture['qpc_interval_ns'],4)
 poll_n=sample_bound(stage,capture['poll_interval_ns'],2*len(ops))
 failure,_=frame_bound(2*capture['writer_max_chunk_bytes']*capture['max_live_writers'],capture)
 sinks=[_sink('history-native-call-meta-v2','evidence',sum(counts.values()),schema_width(capture['meta_schema'])+1,1),
     _sink('history-single-native-capture','evidence',1,raw,1),
     _sink('history-reference-records','evidence',1,refs,1),
     _sink('history-native-RSS','evidence',native_n,schema_width(capture['rss_schema'])+1,0),
     _sink('history-QPC','evidence',qpc_n,schema_width(capture['qpc_schema'])+1,1),
     _sink('history-polls','evidence',poll_n,schema_width(capture['poll_schema'])+1,0),
     _sink('history-shared-fatal-footer','evidence',1,
     failure+16*(schema_width(capture['final_schema'])+1),1)]
 need(type(data['artifact_sinks']) is list and len(data['artifact_sinks'])<=128,
   'all-history-artifact-sinks')
 for item in data['artifact_sinks']:
  closed(item,('name','category','rows','row_bytes','bytes','files','directories'),'history-sink')
  need(item['category'] in ('protocol','evidence') and
    item['bytes']==item['rows']*item['row_bytes'],'history-artifact-expression')
  for key in ('rows','row_bytes','bytes','files','directories'):
   integer(item[key],0,2**63-1,'history-sink-'+key)
  sinks.append(dict(item))
 physical=_history_physical(plan,geometry,source)
 f05bytes=components['current']['bytes']+sum(x['bytes'] for x in physical.values())
 f05files=components['current']['files']+sum(x['files'] for x in physical.values())
 need(set(physical)==set(components['F05']),'all-history-retention-classes')
 for name,row in physical.items():
  need(all(components['F05'][name][k]>=row[k] for k in ('bytes','files','directories')),
    'caller-physical-component-understates-derived:'+name)
 f05bytes=components['current']['bytes']+sum(x['bytes'] for x in components['F05'].values())
 f05files=components['current']['files']+sum(x['files'] for x in components['F05'].values())
 totals={cat:dict(bytes=sum(s['bytes'] for s in sinks if s['category']==cat),
      files=sum(s['files'] for s in sinks if s['category']==cat))
   for cat in ('protocol','evidence')}
 result=dict(source_pin=data['source_pin'],capsule_sha256=data['capsule_sha256'],
  counts=plan['finite_command_bounds'],F05_bytes=f05bytes,F05_files=f05files,
  protocol_bytes=totals['protocol']['bytes'],protocol_files=totals['protocol']['files'],
  evidence_bytes=totals['evidence']['bytes'],evidence_files=totals['evidence']['files'],
  expanded_trace_sha256=sha(canonical(ops)),all_sink_membership_sha256=sha(canonical(dict(F05=physical,supplemental=sinks))),
  operation_counts=counts,sinks=sinks,physical=physical,source_schedule=ops,
  geometry=geometry,metric_names=metric_names,native_samples=native_n,qpc_samples=qpc_n,host_polls=poll_n,
  planned_deletion_credit=0)
 result['profile_sha256']=sha(canonical(result))
 need(f05bytes<=data['available']['F05']['bytes'] and
   f05files<=data['available']['F05']['files'],
   'whole-F05-source-bound-does-not-fit:'+str(f05bytes)+':'+str(f05files))
 for cat in ('protocol','evidence'):
  need(totals[cat]['bytes']<=data['available'][cat]['bytes'] and
    totals[cat]['files']<=data['available'][cat]['files'],
    'history-supplemental-source-bound-does-not-fit:'+cat)
 return result

_HISTORY_MODELS = {}

def encode_private_record(value):
 return canonical(value)+b'\n'

def derive_history_forecast(plan, runtime_bindings, current):
 
 config=closed(runtime_bindings['profile_config'],('components','data'),'history-profile-config')
 need(config['components']['current']==current,'same-original-history-census')
 model=_derive_history_forecast(plan,runtime_bindings,config['components'],config['data'])
 key=sha(canonical(dict(plan=plan,bindings=runtime_bindings,current=current)))
 _HISTORY_MODELS[key]=model
 names=('source_pin','capsule_sha256','counts','F05_bytes','F05_files','protocol_bytes',
     'protocol_files','evidence_bytes','evidence_files','expanded_trace_sha256',
     'all_sink_membership_sha256','profile_sha256')
 return {k:model[k] for k in names}

def history_phase_profile(plan, phase, runtime_bindings, current):
 need(phase in plan['phases'],'exact-history-phase')
 key=sha(canonical(dict(plan=plan,bindings=runtime_bindings,current=current)))
 need(key in _HISTORY_MODELS,'same-process-source-model-before-first-write')
 model=_HISTORY_MODELS[key]
 operations=[x for x in model['source_schedule'] if x['stage']==phase['id']]
 need(operations,'nonempty-source-phase')
 capture=runtime_bindings['profile_config']['data']['capture']
 raw_width=schema_width(capture['reference_schema']);meta_width=body_ref_width(capture)
 schemas=runtime_bindings['profile_config']['data']['record_schemas']
 closed(schemas,('origin','slot','call_index','closure'),'actual-record-schemas')
 return dict(complete=True,source_pin=model['source_pin'],profile_sha256=model['profile_sha256'],
  phase=phase['id'],case_index_max_calls=case_native_count(phase),origin_record_bytes=schema_width(schemas['origin'])+1,
  slot_record_bytes=schema_width(schemas['slot'])+1,
  call_index_record_bytes=max(schema_width(schemas['call_index'])+1,
   1024+case_native_count(phase)*json_object_width(dict(raw_ref=raw_width,meta_ref=meta_width))+
   40*(2048+raw_width+meta_width+2048)),
  closure_record_bytes=schema_width(schemas['closure'])+1,
  metric_names=model['metric_names'],operations=operations,
  expanded_trace_sha256=sha(canonical(operations)),
  whole_sink_membership_sha256=model['all_sink_membership_sha256'],
  original_current_sha256=sha(canonical(current)),deletion_credit=0)


def initial_case_profile(plan,case_id,runtime_bindings,current):
 key=sha(canonical(dict(plan=plan,bindings=runtime_bindings,current=current)))
 need(key in _HISTORY_MODELS,'whole-source-model-first')
 model=_HISTORY_MODELS[key];capture=runtime_bindings['profile_config']['data']['capture']
 lookups={'lookup-'+x for x in ('H1','H10')}
 boot={'bootstrap-'+x+'-p'+str(p) for x in ('H1','H10') for p in (1,4)}
 need(case_id in lookups|boot,'exact-source-initial-case')
 n=778 if case_id in lookups else 162
 pair=json_object_width(dict(raw_ref=schema_width(capture['reference_schema']),
  meta_ref=body_ref_width(capture)))
 return dict(complete=True,source_pin=model['source_pin'],profile_sha256=model['profile_sha256'],
  case_index_max_calls=n,call_index_record_bytes=4096+n*pair)

def global_trace_profile(plan,runtime_bindings,current):
 key=sha(canonical(dict(plan=plan,bindings=runtime_bindings,current=current)))
 need(key in _HISTORY_MODELS,'whole-source-model-first')
 capture=runtime_bindings['profile_config']['data']['capture']
 row=json_object_width(dict(id=ascii_string_width(128),index_ref=body_ref_width(capture)))
 return dict(complete=True,source_pin=runtime_bindings['history']['source_pin'],
  trace_record_bytes=1024+json_array_width(22,row),case_count=22)

METRIC_FIELDS_027={
 'rolling_publications_visited':'rolling_publications_visited',
 'rolling_publication_body_opens':'rolling_publication_body_opens',
 'rolling_discovery_metadata_bytes':'rolling_discovery_metadata_bytes',
 'rolling_deferred_rechecked':'rolling_deferred_rechecked',
 'pack_metadata_stats':'pack_metadata_stats',
 'pack_index_metadata_bytes':'pack_index_metadata_bytes',
 'pack_storage_bytes':'pack_storage_bytes',
 'pack_decoded_bytes':'pack_decoded_bytes',
 'forbidden_global_operations':'forbidden_global_operations',
 'unchanged_historical_body_opens':'unchanged_historical_body_opens'}

def actual_metric_names(scope):
 for semantic,native in METRIC_FIELDS_027.items():
  need(scope['schema'].get(native) in ('u64','u128'),
    'missing-actual-native-counter:'+semantic+':'+native)
 return dict(METRIC_FIELDS_027)

def verify_embedded_history_geometry(plan, bindings, witnesses):
 
 import base64
 need(type(witnesses) is dict and set(witnesses)==set(plan['donors']),'both-admitted-witnesses')
 module=load_source(bindings['tree_decoder'],'embedded_history_Tree')
 result={}
 for identity,donor in plan['donors'].items():
  proof=witnesses[identity]
  copy=dict(proof);expected_sha=copy.pop('sha256')
  need(sha(canonical(copy))==expected_sha and proof['selected']==donor['selected'] and
    proof['actual_H']==donor['actual_H'] and
    proof['tree_sha256']==bindings['tree_decoder']['sha256'] and
    proof['decoder_sha256']==bindings['oracle']['sha256'],'admitted-geometry-source')
  raw=base64.b64decode(proof['publication_b64'],validate=True)
  need(sha(raw)==donor['selected']['digest'],'history-publication-anchor')
  publication=strict(raw,128*MiB);objects={}
  for h,row in proof['checked_objects'].items():
   b=base64.b64decode(row['b64'],validate=True)
   need(len(b)==row['bytes']<=MiB and sha(b)==row['sha256']==h,'history-C1-anchor')
   objects[h]=b
  class EmbeddedTree(module.Tree):
   def obj(self,h):
    need(h in objects,'complete-embedded-current-tree')
    return objects[h]
  for name in ('membership','outcomes'):
   tree=EmbeddedTree(proof['root']);entries=tree.entries(publication[name])
   need(len(entries)==donor['actual_H']-1 and
     tree.height==proof['trees'][name]['height'] and tree.pages==proof['trees'][name]['pages'] and
     tree.leaves==proof['trees'][name]['leaves'] and
     max((len(k.encode()) for k in entries),default=0)==proof['trees'][name]['max_key'] and
     max((len(canonical(v)) for v in entries.values()),default=0)==proof['trees'][name]['max_value'],
     'history-tree-height-recomputed-not-declared')
  class EmbeddedPoints(module.CachedPoints):
   def obj(self,h):
    need(h in objects,'embedded-projection-route');return objects[h]
  points=EmbeddedPoints(proof['root'])
  static={name:points.point(publication['projection'],name)
    for name in ('packet/context.md','packet/provenance.json')}
  store_raw=base64.b64decode(proof['store_b64'],validate=True)
  need(sha(store_raw)==proof['store_sha256'],'embedded-STORE-anchor')
  config=strict(store_raw,65536)
  need(config['mode']=='A' and config['navigation_version']==4 and config['fallback'] is False and
    publication['graph']['count']==128 and publication['graph']['fallback'] is False and
    len(canonical(publication['graph']))<=4096 and
    static==proof['static_proofs'] and
    static['packet/provenance.json']['projection_profile']==4,'authenticated-NOOP-preconditions')
  for name,raw in (('packet/context.md',b''),('packet/provenance.json',NO_PACKET)):
   h=sha(raw);expected=dict(source='derived/'+h[:2]+'/'+h,digest=h)
   if name.endswith('json'):expected['projection_profile']=4
   need(static[name]==expected,'unchanged-static-NOOP-dependencies')
  native_proof=proof.get('cohort_oracle_ref')
  need(native_proof is not None,'admitted-cohort-oracle-byte-reference-required')
  output=strict(bound_source(native_proof,16*MiB),16*MiB)
  need(output['raw_records']==proof['records'] and output['raw_bytes']==proof['raw_bytes'] and
    output['semantic']['selected']==donor['selected'] and
    output['semantic']['publications']==donor['actual_H'],'full-cohort-dimensions-source')
  result[identity]=proof
 return result

def produce_geometry(host, config):
 
 closed(config,('go','runtime_go','final_pin','kind','plan','bindings',
       'cohort_oracles','reserves','output','category','max_bytes'),
     'geometry-prefix-config')
 need(config['go'] is True and config['runtime_go'] is True and
   config['kind'] in ('long','history'),'separate-readonly-geometry-GO')
 permit=host.authorize(config,'geometry-'+config['kind'])
 need(permit.get('go') is True and permit.get('final_pin')==config['final_pin'],
   'positive-exact-geometry-authorization')
 admission=host.checkpoint('readonly-geometry-before-read',config['reserves'])
 need(admission.get('go') is True,'geometry-whole-prefix-admission')
 if config['kind']=='long':
  result=authenticate_geometry(config['bindings'],config['plan']['root'],
       [dict(category=k,id=v) for k,v in REQUESTS])
 else:
  result=authenticate_history_geometry(config['plan'],config['bindings'])
  for identity,proof in result.items():
   proof['cohort_oracle_ref']=config['cohort_oracles'][identity]
   proof.pop('sha256')
   proof['sha256']=sha(canonical(proof))
 raw=encode_private_record(result)
 need(len(raw)<=config['max_bytes']<=32*MiB,'geometry-output-prewrite-bound')
 host.poll()
 ref=host.record(config['output'],raw,config['category'],config['max_bytes'])
 need(ref['bytes']==len(raw) and ref['sha256']==sha(raw),'geometry-output-actual-binding')
 return dict(status='READONLY_GEOMETRY_ONLY_UNQUALIFIED',output=ref,
    no_engine_calls=True,no_corpus_writes=True)

history_case_profile=initial_case_profile
history_trace_profile=global_trace_profile

