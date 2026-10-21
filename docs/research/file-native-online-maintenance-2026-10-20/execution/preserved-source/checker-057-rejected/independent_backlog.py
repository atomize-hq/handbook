CONTRACT={"version":1,"mode":"backlog","quiescent":True,"unit":"distinct normalized existing store-relative file/directory paths","partition":"R_live=U+X+P; absent separate","unknown":"incomplete eligibility/ownership/native proof censors"}
import hashlib,json,os,stat,time
from pathlib import Path
import physical_decoder_v5 as v5
import quota_v2_oracle as quota
class Censor(RuntimeError):pass
def poll():
 n=os.environ.get('FILE_NATIVE_SCRIPT_DEADLINE_PERF_NS','')
 if not n.isdecimal() or time.perf_counter_ns()>=int(n) or time.time()>=1792581014:raise Censor('backlog-deadline')
def safe_path(path):
 p=Path(path)
 if not p.is_absolute():raise Censor('pin-absolute-path')
 for q in [*reversed(p.parents),p]:
  st=q.lstat()
  if q.is_symlink() or getattr(st,'st_file_attributes',0)&1024:raise Censor('reparse-path')
 return p
def verify(root,limits,native_probes):
 root=Path(root);poll()
 d=v5.Decoder(root,quiescent=True);d.verify_packs()
 if d.config['version']!=3:raise Censor('backlog-STORE3-required')
 c=d.config;b=c['physical']
 w=d.read_metadata('physical-plans/'+b['plan']+'.json',v5.WORK_LIMIT,b['plan']);v5.work2(w)
 chain=d.accepted_chain();head=max(chain);anchor=chain[head][0];discovery=w.get('discovery')
 if discovery is None:raise Censor('backlog-Discovery-cohort-UNKNOWN')
 deferred_masks={x['publication']['selected']['publication']:x['roles'] for x in discovery['deferred']}
 reg=quota.registry(v5.prior.legacy.bounded(root/'RESERVATIONS.json',65536),c['store_id'])
 if reg['active']:raise Censor('backlog-active-owner-UNKNOWN')
 owners=d.cohort_owners();R={};audits=[];probes=[];live=set();B=set()
 def propose(name,origin,disposition,reason,kind='file'):
  poll()
  if type(name) is not str or name.startswith('/') or '\\' in name or ':' in name or any(x in {'','.','..'} for x in name.split('/')) or len(name.encode())>limits['max_path_utf8']:raise Censor('backlog-path')
  if name not in R:
   if len(R)>=limits['max_files']:raise Censor('backlog-proposal-bound')
   R[name]=dict(origins=set(),eligible=False,excluded=set(),unknown=set(),expected_kind=kind)
  r=R[name]
  if r['expected_kind']!=kind:raise Censor('backlog-path-kind-conflict')
  r['origins'].add(origin)
  if disposition=='U':r['eligible']=True
  elif disposition=='X':r['excluded'].add(reason)
  else:r['unknown'].add(reason)
 def pack_refs(descriptors,target):
  for desc in descriptors:
   for ext in ['pack','idx']:
    name=v5.prior.container_path(desc,ext);target.add(name)
    if desc.get('instance'):target.add('packs/'+desc['instance'])
 pack_refs(d.descriptors,live)
 live.update(['physical-pins/'+b['id']+'.lock','physical-progress/'+b['id']+'.json','physical-plans/'+b['plan']+'.json','pack-catalogs/'+c['catalog']+'.json'])
 def pending(name,origin,kind='file'):
  propose(name,origin,'U','pending-retirement',kind)
  if name.startswith('physical-stages/') and name.count('/')==2:
   parent=name.rsplit('/',1)[0]
   if (root/parent).exists():
    children=list((root/parent).iterdir())
    if len(children)>1 or any(x.name!='STORE.json' for x in children):raise Censor('backlog-unclassified-stage-child')
    propose(parent,origin,'U','stage-parent-removal','directory')
 if type(native_probes) is not list or len(native_probes)!=1+len(w['older']):raise Censor('backlog-native-probe-coverage')
 for g,probe in zip(w['older'],native_probes[1:]):
  d.pin(g,g['catalog'])
  if probe.get('path')!=str(root/('physical-pins/'+g['id']+'.lock')) or not all(probe.get(k) is True for k in ['native_probe','exclusive','released']):raise Censor('backlog-native-probe-binding')
  probes.append(dict(generation=g,probe=probe))
  if not probe['lock_acquired'] or not probe['released']:raise Censor('backlog-busy-older-native-reader-identity-UNKNOWN')
  for name in ['physical-pins/'+g['id']+'.lock','physical-progress/'+g['id']+'.json','pack-catalogs/'+g['catalog']+'.json']:pending(name,'older-reap')
  if g['plan'] is not None:pending('physical-plans/'+g['plan']+'.json','older-reap')
 # Raw cohort proposals precede policy exclusions, including already mapped roles.
 for selected,p in chain.values():
  keys=['publications/'+p['id']+'.json','results/'+p['result_hash']+'.json','attempt-hints/'+p['attempt']+'.json','attempts/'+p['attempt']+'/candidate.json','attempts/'+p['attempt']+'/allocations/'+p['id']+'.json']
  for n,key in enumerate(keys):
   if n==0 and c['mode']=='B':
    loose=v5.prior.legacy.bounded(root/key,8*1024**2)
    if v5.prior.digest(loose)!=selected['digest']:raise Censor('backlog-B-control-conflict')
    B.add(key);propose(key,'cohort','X','B-publication-control');continue
   if n>=3 and not discovery['include_attempts'] and not deferred_masks.get(p['id'],0)&(1<<n):propose(key,'cohort','X','outside-discovery-attempt-cohort');continue
   if key in d.entries:propose(key,'cohort','X','already-mapped-discovery-role');continue
   terminal=d.terminal_exclusion(p,selected,n,key)
   if terminal is not None:propose(key,'cohort','X',terminal['reason']);continue
   raw=d.raw(key);d.cohort_record(key,raw,owners)
   if len(raw)>8*1024**2:raise Censor('backlog-unclassified-oversize')
   propose(key,'cohort','U','currently-eligible-unmapped-role')
 for number,job in enumerate(w['jobs']):
  reader=v5.Decoder.__new__(v5.Decoder);reader.root=root;reader.config=c;reader.entries={};reader.descriptors=[job['descriptor']];reader.zstd=None;reader.required=set();reader.candidate_context=False;reader.authority=d
  reader.load_index(job['descriptor']);rows=reader.verify_packs()
  if len(rows)!=job['descriptor']['records'] or not 0<=job['position']<=len(rows):raise Censor('backlog-job-index-cursor')
  h=hashlib.sha256()
  for row in rows:h.update((json.dumps(row,separators=(',',':'))+'\n').encode())
  audits.append(dict(job=number,descriptor=job['descriptor'],position=job['position'],frontier=job['frontier'],records=len(rows),full_raw_inventory_sha256=h.hexdigest()))
  for row in rows[job['position']:]:
   key=row['key'];role=v5.prior.legacy.role(key)
   if c['mode']=='B' and role=='publication':propose(key,'job-suffix','X','B-publication-control');continue
   if not (root/key).exists():propose(key,'job-suffix','U','native-absent-cursor');continue
   if key not in d.entries:
    if role not in {'candidate','allocation'}:raise Censor('backlog-selected-replacement-missing')
    propose(key,'job-suffix','X','semantic-GC-only-without-selected-replacement');continue
   raw=reader.raw(key);selected_raw=d.raw(key);loose=v5.prior.legacy.bounded(root/key,8*1024**2)
   if raw!=selected_raw or raw!=loose:raise Censor('backlog-loose-replacement-conflict')
   propose(key,'job-suffix','U','authenticated-pending-loose-retirement')
 for retired in w['retired']:
  desc=retired['descriptor']
  for ext,done in [('pack',retired['pack_done']),('idx',retired['index_done'])]:
   name=v5.prior.container_path(desc,ext)
   propose(name,'retired-extension','?' if done else 'U','done-bit-needs-live-sharing' if done else 'unfinished-extension')
  if desc.get('instance'):pending('packs/'+desc['instance'],'retired-parent','directory')
 for name in w['garbage']:pending(name,'selected-garbage','directory' if name.startswith('packs/') and name.count('/')==1 else 'file')
 j=d.read_metadata('PACKING.json',65536) if (root/'PACKING.json').exists() else None
 if j is not None:
  d.journal()
  if j.get('version')!=2:raise Censor('backlog-legacy-journal-ownership-UNKNOWN')
  if j['status'] in {'building','installed'} and j['id']!=b['id']:raise Censor('backlog-active-unselected-owner-UNKNOWN')
  abandoned=[]
  if j['status']=='aborted-owned':
   abandoned=list(j.get('abandoned',[]))
   if not j.get('ownership_transferred',False):abandoned.append(dict(id=j['id'],catalog=j['catalog'],plan=(j.get('target') or {}).get('physical',{}).get('plan')))
  elif j.get('abandoned'):raise Censor('backlog-unclassified-journal-abandoned-UNKNOWN')
  unique={}
  for owner in abandoned:
   if owner['id'] in unique and unique[owner['id']]!=owner:raise Censor('backlog-aborted-owner-conflict')
   unique[owner['id']]=owner
  if len(unique)>64:raise Censor('backlog-owner-bound')
  for owner in unique.values():
   if owner['id']==b['id'] or any(g['id']==owner['id'] for g in w['older']):raise Censor('backlog-aborted-selected-owner')
   directory='packs/'+owner['id'];pending(directory,'aborted-owner','directory')
   if (root/directory).exists():
    children=list((root/directory).iterdir())
    if len(children)>128:raise Censor('backlog-orphan-container-bound')
    for child in children:
     if not child.is_file() or child.is_symlink():raise Censor('backlog-orphan-container-type')
     name=child.relative_to(root).as_posix();v5.metadata2(name);pending(name,'aborted-owner')
   if owner['catalog'] is not None:pending('pack-catalogs/'+owner['catalog']+'.json','aborted-owner')
   if owner['plan'] is not None:
    name='physical-plans/'+owner['plan']+'.json'
    if (root/name).exists():
     plan=d.read_metadata(name,v5.WORK_LIMIT,owner['plan']);v5.work2(plan)
     if plan['store_id']!=c['store_id'] or plan['generation']['id']!=owner['id'] or plan['generation']['catalog']!=owner['catalog']:raise Censor('backlog-aborted-plan-owner')
     for garbage in plan['garbage']:pending(garbage,'aborted-flattened','directory' if garbage.startswith('packs/') and garbage.count('/')==1 else 'file')
    pending(name,'aborted-owner')
   for name in ['physical-progress/'+owner['id']+'.work-stage','physical-progress/'+owner['id']+'.selector-stage','physical-pins/'+owner['id']+'.lock','physical-progress/'+owner['id']+'.json','physical-stages/'+owner['id']+'/STORE.json']:pending(name,'aborted-owner')
 counts=dict(unprotected=0,protected=0,excluded=0,absent=0);physical_bytes=0
 for name,r in R.items():
  poll();path=root/name
  try:st=path.lstat()
  except FileNotFoundError:r.update(kind='absent',disposition='absent');counts['absent']+=1;continue
  safe_path(path)
  kind='file' if stat.S_ISREG(st.st_mode) else 'directory' if stat.S_ISDIR(st.st_mode) else None
  if kind!=r['expected_kind']:raise Censor('backlog-native-type')
  if kind=='file':physical_bytes+=st.st_size
  if physical_bytes>limits['max_logical_bytes']:raise Censor('backlog-logical-byte-bound')
  r.update(kind=kind,bytes=st.st_size,identity=[st.st_dev,st.st_ino])
  if name in live or name in B:r['disposition']='excluded';r['excluded'].add('live-selected-reference' if name in live else 'B-publication-control')
  elif r['eligible']:r['disposition']='unprotected'
  elif r['unknown']:raise Censor('backlog-done-path-without-live-sharing-UNKNOWN')
  elif r['excluded']:r['disposition']='excluded'
  else:raise Censor('backlog-path-disposition-UNKNOWN')
  counts[r['disposition']]+=1
 if d.accepted_chain()[head][0]!=anchor or v5.prior.legacy.strict((root/'SELECTED.json').read_bytes())!=anchor:raise Censor('backlog-anchor-race')
 def evidence():
  for audit in audits:yield dict(kind='full-job-index-audit',**audit)
  for name,r in sorted(R.items()):
   path=root/name
   if r['kind']=='absent':
    if path.exists() or path.is_symlink():raise Censor('backlog-native-presence-race')
   else:
    st=path.lstat()
    if [st.st_dev,st.st_ino]!=r['identity'] or st.st_size!=r['bytes'] or (stat.S_ISDIR(st.st_mode)!=(r['kind']=='directory')):raise Censor('backlog-native-presence-race')
   yield dict(path=name,**{k:(sorted(v) if isinstance(v,set) else v) for k,v in r.items()})
  if v5.prior.legacy.strict((root/'SELECTED.json').read_bytes())!=anchor or v5.prior.legacy.strict((root/'STORE.json').read_bytes())!=c:raise Censor('backlog-anchor-race')
  yield dict(kind='stable-anchor-and-native-path-presence-closure',anchor=anchor,work_sha256=b['plan'])
 result=dict(status='passed',independent=True,quiescent=True,unit=CONTRACT['unit'],complete_candidate_coverage=True,**counts,raw_proposal_paths=len(R),existing_paths=len(R)-counts['absent'],source_H=head+1,stable_anchor=anchor,discovery=discovery,work_sha256=b['plan'],pin_probes=probes,protected_basis='all older authenticated exclusive native probes acquired and released; no busy owner inferred as shared reader',excluded_owner_facts=dict(unit='registry terminal owner facts; separate from PATH partition',count=reg.get('terminal_count',0)),job_index_audits=len(audits),cohort_proof=d.mapped_cohort_proof)
 return result,evidence()
