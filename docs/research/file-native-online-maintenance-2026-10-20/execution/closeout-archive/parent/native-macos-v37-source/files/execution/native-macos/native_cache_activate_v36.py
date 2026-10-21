"""Closed C1 copy operator; inert on import."""
import io,fcntl,hashlib,json,math,os,stat,tarfile,time,tomllib
from pathlib import Path
ROOT=Path('/private/tmp/file-native-online-macos-20261020')
PIN='0628322e4b48e0efad19cdc32238a622d4aaa7a8'
CAP=67108864
STREAM_SHA='f3b58ce6a0600081fe86f67d1ea97da4d813c1982ed3000fe02054fd5a83b85b'
MANIFEST_SHA='464dcf878410def63bd3b04d4f48f17d75418c04262cb4745ebc0e6d5e1de6b3'
CUTOFF=1792581014
def need(ok,message):
 if not ok:raise RuntimeError(message)
def digest(path):
 h=hashlib.sha256()
 with Path(path).open('rb') as f:
  for b in iter(lambda:f.read(65536),b''):h.update(b)
 return h.hexdigest()
def strict(raw):
 def pairs(items):
  d={}
  for k,v in items:need(k not in d,'duplicate-member');d[k]=v
  return d
 def bad(v):raise ValueError('nonfinite:'+v)
 return json.loads(raw,object_pairs_hook=pairs,parse_constant=bad)
def owned(path,base):
 p=Path(path);need(p.is_absolute() and p.is_relative_to(base) and '..' not in p.parts,'owned-path')
 for q in [p,*p.parents]:need(not q.is_symlink(),'path-alias')
 return p
def fact(path):
 p=Path(path);s=p.lstat()
 need(stat.S_ISREG(s.st_mode) and s.st_nlink==1,'one-regular-owned-copy')
 return (s.st_dev,s.st_ino,s.st_size,s.st_mtime_ns)
def activate(release_bytes,expected_sha256,runtime_factory,transport_blobs):
 need(type(release_bytes) is bytes and len(release_bytes)<=1024**2 and hashlib.sha256(release_bytes).hexdigest()==expected_sha256,'exact-copy-release')
 r=strict(release_bytes);need(r['kind']=='native-cache-copy-v37' and r['parent_released'] is True and r['cache_copy_go'] is True,'explicit-copy-phase-GO')
 need(r['source_pin']==PIN and r['engine_manifest_sha256']==MANIFEST_SHA and r['root']==str(ROOT),'fixed-engine-source')
 need(r['C1_claim_sha256']=='8232ff614dd32922175b8fc558c57293760dfc6460b888dc4402640588f9be35' and r['C1_seal_sha256']=='e29fce55d34c685b2579e3cc2c277ed731cb72606985ccf425ca2742b006812a','activated-C1-claim')
 need(r['capacity']=={'bytes':CAP,'files':5000} and r['runtime_go'] is False,'copy-only-no-implicit-runtime')
 need(r['fixed_cutoff']==CUTOFF and time.time()<CUTOFF,'functional-cutoff')
 deadline=r['deadline_monotonic'];need(type(deadline) in (int,float) and math.isfinite(deadline) and time.monotonic()<deadline,'bounded-copy-deadline')
 need(r['stage_seconds']>0 and 0<deadline-r['issued_monotonic']<=r['stage_seconds'],'fixed-copy-stage')
 sources=r['source_files']
 for path,row in sources.items():
  p=owned(path,ROOT);need(fact(p)[2]==row['bytes'] and digest(p)==row['sha256'],'source-drift')
 b=ROOT/'source/final-engine/_admission/source-copy-budget.json'
 need(digest(b)=='1207cc066b5fece7fb7d8ce2cd0e7e8e45c693a39027ac5b92f92174dcdca62a','engine25-manifest')
 for n,v in strict(b.read_bytes())['source_files'].items():
  p=ROOT/'source/final-engine'/n;need(sources.get(str(p))==v and fact(p)[2]==v['bytes'] and digest(p)==v['sha256'],'engine25-source')
 cache=owned(ROOT/'cargo-private',ROOT);need(not cache.exists(),'fresh-empty-private-cache')
 lock=tomllib.loads((ROOT/'source/final-engine/Cargo.lock').read_text())
 packages={(p['name'],p['version']):p['checksum'] for p in lock['package'] if p.get('source','').startswith('registry+')}
 need(len(packages)==45,'exact45-locked-registry-packages')
 rows=r['entries'];need(type(rows) is list and 45<=len(rows)<=5000,'finite-copy-manifest')
 seen=set();identities={};archives={};index_records={};copy_bytes=0;ram={}
 def source_data(row):
  if row['source'].startswith('ram:'):return ram[row['source']]
  return Path(row['source']).read_bytes()
 source_base=Path(r['source_cache_root']);need(source_base.is_absolute(),'explicit-readonly-source-cache')
 for row in rows:
  need(set(row)=={'source','destination','bytes','sha256','identity','kind'},'closed-file-entry')
  dst=owned(row['destination'],cache)
  if row['source'].startswith('ram:'):
   raw=transport_blobs[row['source'][4:]]
   need(type(raw) is bytes and len(raw)<=2*1024**2 and len(raw)==row['bytes'] and hashlib.sha256(raw).hexdigest()==row['sha256'] and row['identity'] is None,'RAM-member')
   ram[row['source']]=raw;p=None
  else:p=owned(row['source'],source_base)
  need(str(dst) not in seen and not dst.exists(),'fresh-unique-copy-destination');seen.add(str(dst))
  need(type(row['bytes']) is int and 0<=row['bytes']<=CAP and (p is None or (fact(p)==tuple(row['identity']) and fact(p)[2]==row['bytes'] and digest(p)==row['sha256'])),'source-member')
  if p is not None:identities[str(p)]=fact(p)
  copy_bytes+=row['bytes']
  need(row['kind'] in ('crate','index','public-config'),'only-public-registry-inputs')
  rel=dst.relative_to(cache)
  need(rel.parts[0]=='registry' and rel.parts[1] in ('cache','index') and rel.parts[2]=='index.crates.io-1949cf8c6b5b557f','exact-public-registry-prefix')
  if row['kind']=='crate':
   matches=[key for key in packages if dst.name==key[0]+'-'+key[1]+'.crate']
   need(len(matches)==1 and matches[0] not in archives and packages[matches[0]]==row['sha256'] and rel.parts[1]=='cache','exact-locked-crate')
   archives[matches[0]]=row
  elif row['kind']=='public-config':
   need(rel.parts[1]=='index' and dst.name=='config.json' and row['bytes']<=4096,'only-public-config')
   v=strict(source_data(row));need(v=={'dl':'https://static.crates.io/crates','api':'https://crates.io'},'public-config-no-credentials')
  else:
   need(rel.parts[1]=='index' and '.cache' in rel.parts and row['bytes']<=4*1024**2,'bounded-sparse-index')
   for chunk in source_data(row).split(b'\0'):
    if not chunk.startswith(b'{'):continue
    v=strict(chunk);key=(v['name'],v['vers'])
    if key in packages:
     need(v['cksum']==packages[key],'exact-locked-sparse-checksum');index_records[key]=row['sha256']
 need(set(archives)==set(packages) and set(index_records)==set(packages),'locked45-closure')
 resolution=r['host_resolution']
 need(resolution['parent_reviewed'] is True and resolution['target'] in ('aarch64-apple-darwin','x86_64-apple-darwin') and resolution['source_pin']==PIN,'Darwin-unpack')
 selected={(x['name'],x['version']) for x in resolution['packages']};need(selected<=set(packages) and selected,'actual-host-package-subset')
 future_bytes=0;future_files=0
 for key in sorted(selected):
  row=archives[key];count=0;size=0
  with tarfile.open(fileobj=io.BytesIO(ram[row['source']]),mode='r|gz') if row['source'] in ram else tarfile.open(row['source'],mode='r|gz') as tar:
   for member in tar:
    name=Path(member.name);need(not name.is_absolute() and '..' not in name.parts and name.parts[0]==key[0]+'-'+key[1],'crate-extraction-path')
    need(member.isfile() or member.isdir(),'crate-link-or-special-member-refused')
    if member.isfile():count+=1;size+=member.size
    need(size<=CAP and count<=5000,'future-unpack-bound')
  expected=next(x for x in resolution['packages'] if (x['name'],x['version'])==key)
  need((size,count)==(expected['unpacked_bytes'],expected['unpacked_files']),'exact-future-unpack-forecast')
  future_bytes+=size;future_files+=count
 bookkeeping=r['bookkeeping'];need(set(bookkeeping)=={'bytes','files'} and all(type(v) is int and v>=0 for v in bookkeeping.values()),'finite-bookkeeping')
 need(copy_bytes+future_bytes+bookkeeping['bytes']<=CAP and len(rows)+future_files+bookkeeping['files']<=5000,'C1-total-cap')
 need(r['forecast']=={'copy_bytes':copy_bytes,'copy_files':len(rows),'unpack_bytes':future_bytes,'unpack_files':future_files},'exact-operation-forecast')
 p=Path(runtime_factory.__code__.co_filename).resolve();need(str(p) in sources,'closed-runtime-factory')
 runtime=runtime_factory()
 for module in (runtime.roles,runtime.monitor,runtime.admission,runtime.cache_stream):
  p=Path(module.__file__).resolve();need(str(p) in sources and digest(p)==sources[str(p)]['sha256'],'bound-runtime-module')
 need(digest(runtime.cache_stream.__file__)==STREAM_SHA,'unchanged-stream33')
 manifest=ROOT/'source/final-engine/_admission/cp4-final31-source-freeze-final-v3.json'
 need(digest(manifest)==MANIFEST_SHA,'immutable-engine-source-manifest')
 token,chain=runtime.roles.ancestor_token()
 need(token['source_pin']==PIN and token['claims']==r['claims']==runtime.admission.CLAIMS and token['fixed_cutoff']==CUTOFF,'native-claims')
 need(token['nonce']==r['guard_nonce'] and token['parent']==r['guard_parent'] and token['keeper']==r['guard_keeper'],'actual-guard-owner-binding')
 def auth():
  need(time.time()<CUTOFF and time.monotonic()<deadline and not (ROOT/'HOST-FENCE.json').exists(),'copy-deadline-or-fence')
  p=ROOT/'mac-heavy.lock';need(not p.is_symlink() and p.stat().st_size<=16384,'actual-heavy-token-path')
  with p.open('rb') as f:
   current=strict(f.read())
   try:fcntl.flock(f.fileno(),fcntl.LOCK_EX|fcntl.LOCK_NB)
   except BlockingIOError:pass
   else:raise RuntimeError('heavy-token-not-held')
  need(all(current[k]==token[k] for k in ('nonce','parent','keeper','source_pin','claims','fixed_cutoff')),'guard-token-drift')
  for expected in (token['parent'],token['keeper'],chain[0]):need(runtime.monitor.same_identity(expected,runtime.roles.identity(expected['pid'])),'native-owner-birth-drift')
  need(os.statvfs(ROOT).f_bavail*os.statvfs(ROOT).f_frsize>=20*1024**3,'free-space-floor')
 auth()
 before=runtime.admission.snapshot(shared=False)
 need(before['categories']['C1']['logical_bytes']==0 and before['categories']['C1']['files']==0,'C1-not-empty')
 request={k:{'bytes':0,'files':0} for k in runtime.admission.LIMITS};request['C1']={'bytes':copy_bytes+future_bytes+bookkeeping['bytes'],'files':len(rows)+future_files+bookkeeping['files']}
 runtime.admission.admit(before,request,runtime.admission.limits())
 receipt=owned(r['receipt_path'],ROOT/'evidence')
 need(not receipt.exists() and r['receipt_cap']<=65536 and r['receipt_cap']>=4096,'bounded-fresh-copy-receipt')
 request={k:{'bytes':0,'files':0} for k in runtime.admission.LIMITS};request['F12']={'bytes':r['receipt_cap'],'files':1}
 runtime.admission.admit(before,request,runtime.admission.limits())
 copied=[]
 for row in rows:
  auth()
  if row['source'] in ram:
   raw=ram[row['source']];need(hashlib.sha256(raw).hexdigest()==row['sha256'],'RAM-recheck')
   request={k:{'bytes':0,'files':0} for k in runtime.admission.LIMITS};request['C1']={'bytes':len(raw),'files':1}
   runtime.admission.admit(runtime.admission.snapshot(shared=False),request,runtime.admission.limits())
   dst=owned(row['destination'],cache);dst.parent.mkdir(parents=True,exist_ok=True)
   with dst.open('xb') as out:
    for offset in range(0,len(raw),65536):auth();out.write(raw[offset:offset+65536])
    out.flush();os.fsync(out.fileno())
   need(digest(dst)==row['sha256'],'RAM-copy-posthash')
   copied.append({'destination':str(dst),'bytes':len(raw),'sha256':row['sha256'],'chunk_ceiling':65536})
   continue
  need(fact(row['source'])==identities[row['source']] and digest(row['source'])==row['sha256'],'source-recheck')
  copied.append(runtime.cache_stream.stream_member(row['source'],row['destination'],expected_bytes=row['bytes'],expected_sha256=row['sha256'],operation_ceiling=row['bytes'],deadline=deadline,synthetic=False))
 auth()
 for path,identity in identities.items():need(fact(path)==identity,'readonly-cache-source-drift')
 actual={}
 for d,dirs,names in os.walk(cache,followlinks=False,onerror=lambda e:(_ for _ in ()).throw(e)):
  need(all(not (Path(d)/n).is_symlink() for n in dirs),'cache-directory-alias')
  for n in names:
   p=Path(d)/n;actual[str(p)]={'bytes':fact(p)[2],'sha256':digest(p)}
 expected={row['destination']:{'bytes':row['bytes'],'sha256':row['sha256']} for row in rows}
 need(actual==expected,'copy-census')
 result={'kind':'native-cache-copy-v37-result','source_pin':PIN,'release_sha256':expected_sha256,'guard_nonce':token['nonce'],'copied_bytes':copy_bytes,'copied_files':len(rows),'private_census':actual,'future_unpack_bytes':future_bytes,'future_unpack_files':future_files,'runtime_go':False,'outer_guard_final_closure_pending':True,'failure_prefixes_retained':True}
 raw=json.dumps(result,sort_keys=True,separators=(',',':'),allow_nan=False).encode()
 need(len(raw)<=r['receipt_cap'],'receipt-cap')
 receipt.parent.mkdir(parents=True,exist_ok=True)
 with receipt.open('xb') as out:out.write(raw);out.flush();os.fsync(out.fileno())
 return result
if __name__=='__main__':raise SystemExit('SOURCE_ONLY_EXPLICIT_CACHE_COPY_GO_REQUIRED')
