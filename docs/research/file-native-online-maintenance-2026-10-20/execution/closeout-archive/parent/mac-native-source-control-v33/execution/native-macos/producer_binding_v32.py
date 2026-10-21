"""Closed candidate/historical producer bindings. Source-only, no launch route."""
import hashlib,json,time
from pathlib import Path
import admission_v20 as admission


def digest(path):
 h=hashlib.sha256()
 with Path(path).open('rb') as f:
  for b in iter(lambda:f.read(65536),b''):h.update(b)
 return h.hexdigest()


def document(path,expected,cap):
 p=Path(path)
 if p.is_symlink() or not p.is_file() or p.stat().st_size>cap:raise RuntimeError('producer-document-path-size')
 raw=p.read_bytes()
 if hashlib.sha256(raw).hexdigest()!=expected:raise RuntimeError('producer-document-hash')
 return admission.strict_json(raw)


def bind(binary,token,binding,binding_sha256,*,request_bytes,env,case_context=None,historical_grant=None):
 row=document(binding,binding_sha256,65536)
 fields={'source_pin','source_binding','source_binding_sha256','guard_sha256','binary_path','binary_sha256','engine_release'}
 if not isinstance(row,dict) or set(row)!=fields or row['engine_release'] is not True:raise RuntimeError('producer-candidate-binding-shape')
 if token['source_pin']=='CONTROL_ONLY' or row['source_pin']!=token['source_pin']:raise RuntimeError('producer-active-source-pin')
 if row['guard_sha256']!=digest(Path(__file__).with_name('guard_v20.py')):raise RuntimeError('producer-active-guard')
 source=document(row['source_binding'],row['source_binding_sha256'],262144)
 verified=admission.source_binding(row['source_binding'],token['source_pin'])
 if verified['guard_sha256']!=row['guard_sha256']:raise RuntimeError('producer-active-source-guard')
 candidate=Path(row['binary_path'])
 if candidate.is_symlink() or not candidate.is_file() or digest(candidate)!=row['binary_sha256']:raise RuntimeError('producer-candidate-binary')
 binary=Path(binary).resolve(strict=True)
 if historical_grant is None:
  if str(binary)!=row['binary_path'] or digest(binary)!=row['binary_sha256']:raise RuntimeError('producer-exact-candidate-binary')
  return dict(row,producer_kind='candidate',producer_source_pin=row['source_pin'],active_candidate_source_pin=token['source_pin'])
 if not isinstance(historical_grant,dict) or set(historical_grant)!={'path','sha256'}:raise RuntimeError('historical-grant-reference')
 g=document(historical_grant['path'],historical_grant['sha256'],65536)
 required={'kind','parent_released','grant_id','parent_run_id','case_id','active_candidate_pin','active_candidate_source_binding_sha256','active_guard_sha256','historical_source_pin','historical_source_root','historical_source_manifest_path','historical_source_manifest_sha256','historical_build_receipt_path','historical_build_receipt_sha256','historical_binary_path','historical_binary_sha256','allowed_operations','allowed_request_sha256','allowed_environment_sha256','allowed_new_fixture_prefix','fixed_expiry'}
 if not isinstance(g,dict) or set(g)!=required or g['kind']!='native-historical-case-grant-v1' or g['parent_released'] is not True:raise RuntimeError('historical-grant-closed-shape')
 if case_context!={'parent_run_id':g['parent_run_id'],'case_id':g['case_id']}:raise RuntimeError('historical-exact-case-run')
 if (g['active_candidate_pin'],g['active_candidate_source_binding_sha256'],g['active_guard_sha256'])!=(token['source_pin'],row['source_binding_sha256'],row['guard_sha256']):raise RuntimeError('historical-active-binding')
 if type(g['fixed_expiry']) not in (int,float) or not time.time()<g['fixed_expiry']<=token['fixed_cutoff']:raise RuntimeError('historical-fixed-expiry')
 if not isinstance(g['historical_source_pin'],str) or g['historical_source_pin']==token['source_pin']:raise RuntimeError('historical-distinct-source-pin-required')
 for key in ('allowed_operations','allowed_request_sha256','allowed_environment_sha256'):
  if not isinstance(g[key],list) or not g[key] or len(g[key])>256 or any(not isinstance(v,str) for v in g[key]):raise RuntimeError('historical-allow-list')
 request=admission.strict_json(request_bytes)
 if not isinstance(request,dict) or request.get('action') not in g['allowed_operations']:raise RuntimeError('historical-operation')
 if hashlib.sha256(request_bytes).hexdigest() not in g['allowed_request_sha256']:raise RuntimeError('historical-exact-request')
 environment=json.dumps(dict({'EXP_NAVIGATION':'4'},**(env or {})),sort_keys=True,separators=(',',':')).encode()
 if hashlib.sha256(environment).hexdigest() not in g['allowed_environment_sha256']:raise RuntimeError('historical-exact-environment')
 relative=Path(g['allowed_new_fixture_prefix'])
 if relative.is_absolute() or '..' in relative.parts or not relative.parts or relative.parts[0]!='fixtures' or len(relative.parts)<2:raise RuntimeError('historical-fixture-grant-prefix')
 root=Path(request['root']);allowed=admission.ROOT/relative
 if not root.is_absolute() or '..' in root.parts or not root.resolve().is_relative_to(allowed.resolve()):raise RuntimeError('historical-owned-fixture-destination')
 for p in [root,*root.parents]:
  if p.is_relative_to(admission.ROOT) and p.is_symlink():raise RuntimeError('historical-fixture-symlink')
 for key in ('EXP_READY','EXP_RELEASE','EXP_PACK_CANCEL'):
  if env is not None and key in env:
   path=Path(env[key])
   if not path.is_absolute() or '..' in path.parts or not path.resolve().is_relative_to(allowed.resolve()):raise RuntimeError('historical-owned-control-environment')
 old=document(g['historical_source_manifest_path'],g['historical_source_manifest_sha256'],262144)
 if not isinstance(old,dict) or set(old)!={'source_pin','files'} or old['source_pin']!=g['historical_source_pin'] or not isinstance(old['files'],list) or not 1<=len(old['files'])<=512:raise RuntimeError('historical-source-manifest-shape')
 oldroot=Path(g['historical_source_root'])
 if not oldroot.is_absolute() or oldroot.is_symlink() or not oldroot.is_dir():raise RuntimeError('historical-source-root')
 seen=set()
 for member in old['files']:
  if not isinstance(member,dict) or set(member)!={'path','bytes','sha256'}:raise RuntimeError('historical-source-member-shape')
  if not isinstance(member['path'],str) or not member['path'] or type(member['bytes']) is not int or member['bytes']<0 or not isinstance(member['sha256'],str) or len(member['sha256'])!=64:raise RuntimeError('historical-source-member-types')
  rel=Path(member['path'])
  if rel.is_absolute() or '..' in rel.parts or member['path'] in seen:raise RuntimeError('historical-source-member-path')
  seen.add(member['path']);p=oldroot/rel
  if p.is_symlink() or not p.is_file() or not p.resolve().is_relative_to(oldroot.resolve()) or p.stat().st_size!=member['bytes'] or digest(p)!=member['sha256']:raise RuntimeError('historical-source-member-mismatch')
 # The sealed grant is the parent's explicit association of these immutable
 # historical source/build/binary bytes; never infer that association by name.
 receipt=document(g['historical_build_receipt_path'],g['historical_build_receipt_sha256'],262144)
 if not isinstance(receipt,dict):raise RuntimeError('historical-build-receipt-object')
 if str(binary)!=g['historical_binary_path'] or digest(binary)!=g['historical_binary_sha256']:raise RuntimeError('historical-exact-binary')
 return {'producer_kind':'historical','source_pin':g['historical_source_pin'],'producer_source_pin':g['historical_source_pin'],'active_candidate_source_pin':token['source_pin'],'binary_path':str(binary),'binary_sha256':g['historical_binary_sha256'],'guard_sha256':row['guard_sha256'],'historical_grant_sha256':historical_grant['sha256'],'historical_source_manifest_sha256':g['historical_source_manifest_sha256'],'historical_build_receipt_sha256':g['historical_build_receipt_sha256'],'case_context':case_context,'active_candidate_binding':row}


def interrupt_operations(reference,token,request_bytes,case_context):
 if reference is None:return frozenset()
 if not isinstance(reference,dict) or set(reference)!={'path','sha256'}:raise RuntimeError('interrupt-grant-reference')
 g=document(reference['path'],reference['sha256'],16384)
 fields={'kind','parent_released','case_id','parent_run_id','source_pin','guard_sha256','request_sha256','allowed_interrupts','fixed_expiry'}
 if not isinstance(g,dict) or set(g)!=fields or g['kind']!='native-owned-interrupt-grant-v1' or g['parent_released'] is not True:raise RuntimeError('interrupt-grant-shape')
 if case_context!={'case_id':g['case_id'],'parent_run_id':g['parent_run_id']}:raise RuntimeError('interrupt-grant-case')
 if g['source_pin']!=token['source_pin'] or g['guard_sha256']!=digest(Path(__file__).with_name('guard_v20.py')) or g['request_sha256']!=hashlib.sha256(request_bytes).hexdigest():raise RuntimeError('interrupt-grant-source-request')
 if type(g['fixed_expiry']) not in (int,float) or not time.time()<g['fixed_expiry']<=token['fixed_cutoff']:raise RuntimeError('interrupt-grant-expiry')
 ops=g['allowed_interrupts']
 if not isinstance(ops,list) or not ops or len(ops)>3 or len(set(ops))!=len(ops) or any(x not in ('interrupt','terminate','kill') for x in ops):raise RuntimeError('interrupt-grant-operations')
 return frozenset(ops)
