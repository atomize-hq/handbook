import sys,json,hashlib,copy,struct
from pathlib import Path
sys.path.insert(0,'C:/Users/spmcc/.codex/worktrees/file-native-online-matrix-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/harness')
from driver import Driver
b=Path('C:/file-native-online-cp1-20261020');e=b/'evidence';name=sys.argv[1];red='red' in name;events=e/f'cp3-{name}-events.jsonl';d=Driver(b/'build/release/file-native-incremental.exe',events);rows=[]
def sha(data):return hashlib.sha256(data).hexdigest()
def encoded(value):return json.dumps(value,indent=2).encode()
def put(path,data):path.parent.mkdir(parents=True,exist_ok=True);assert not path.exists();path.write_bytes(data)
def config(r):return json.loads((r/'STORE.json').read_bytes())
def work(r):c=config(r);return json.loads((r/f"physical-plans/{c['physical']['plan']}.json").read_bytes())
def catalog(r):c=config(r);return json.loads((r/f"pack-catalogs/{c['catalog']}.json").read_bytes())
def path(r,x,ext):return r/f"packs/{x['instance']}/{x['pack' if ext=='pack' else 'index']}.{ext}"
def leaf(r,x):
 data=path(r,x,'idx').read_bytes();assert data[:8]==b'FNPIDX02' and data[8]==0 and sha(data)==x['index']==x['page']['hash'];pos=74
 for _ in range(2):n=struct.unpack_from('<H',data,pos)[0];pos+=2+n
 entries=[]
 while pos<len(data):
  n=struct.unpack_from('<H',data,pos)[0];pos+=2;key=data[pos:pos+n].decode();pos+=n;off,enc,raw=struct.unpack_from('<QII',data,pos);h=data[pos+16:pos+48];pos+=48;entries.append((key,off,enc,raw,h))
 assert len(entries)==x['records'];return data,entries
def raw_map(r):
 result={}
 for x in catalog(r)['packs']:
  data,entries=leaf(r,x);pack=path(r,x,'pack').read_bytes();assert sha(pack)==x['pack'] and x['codec']==0
  for key,off,enc,raw,h in entries:
   value=pack[off:off+enc];assert enc==raw and sha(value)==h.hex()
   if key in result:assert result[key]==value
   result[key]=value
 return result
def overlap_fixture(r,none):
 c=config(r);oldbytes=(r/'STORE.json').read_bytes();oldwork=work(r);oldcat=catalog(r);descriptors=[]
 for part in range(2):
  birth=f'input-{name}-{part}'
  for original in oldcat['packs']:
   x=copy.deepcopy(original);x['instance']=birth;x['segment']=part+1
   for ext in ['pack','idx']:put(path(r,x,ext),path(r,original,ext).read_bytes())
   descriptors.append(x)
   if original['role']=='result':
    source,entries=leaf(r,original);keys=[f"results/{part:060x}{n:04x}.json" for n in range(len(entries))];assert keys[-1]<original['first']
    def key(s):v=s.encode();return struct.pack('<H',len(v))+v
    data=source[:74]+key(keys[0])+key(keys[-1])
    for new,(_,off,enc,raw,h) in zip(keys,entries):data+=key(new)+struct.pack('<QII',off,enc,raw)+h
    alias=copy.deepcopy(x);alias['first']=keys[0];alias['last']=keys[-1];alias['index']=sha(data);alias['index_bytes']=len(data);alias['page']['hash']=sha(data);alias['page']['bytes']=len(data);put(path(r,alias,'idx'),data);descriptors.append(alias)
 descriptors.sort(key=lambda x:(x['segment'],x['role'],x['first']))
 cat=dict(version=4,store_id=c['store_id'],packs=descriptors);data=encoded(cat);h=sha(data);put(r/f'pack-catalogs/{h}.json',data)
 g=copy.deepcopy(oldwork['generation']);g['plan']=c['physical']['plan'];w=copy.deepcopy(oldwork);w['older'].append(g);w['retired'].extend(dict(descriptor=x,pack_done=False,index_done=False) for x in oldcat['packs'])
 identity=f'overlap-{name}';ordinal=c['physical']['ordinal']+1;w['generation']=dict(id=identity,ordinal=ordinal,catalog=h,plan=None,packs=descriptors);w['physical_parent']=sha(oldbytes);w['transition_parent']=None;w['birth_claims']=[]
 if none:w.pop('discovery',None)
 data=encoded(w);wh=sha(data);put(r/f'physical-plans/{wh}.json',data);put(r/f'physical-pins/{identity}.lock',encoded(dict(version=1,store_id=c['store_id'],id=identity,ordinal=ordinal,catalog=h)))
 c['catalog']=h;c['physical']=dict(id=identity,ordinal=ordinal,plan=wh);(r/'STORE.json').write_bytes(encoded(c))
 return descriptors
for mode in (['A'] if red else ['A','B']):
 r=b/f'fixtures/cp3/{name}-{mode}/disposable/store';d.call(r,'init',mode=mode,consumer='project',count=10);d.call(r,'lifecycle',operation='setup');expected={'initial':d.call(r,'resolve',request_id='initial')['result']}
 for n in range(2):request=f'{name}-{mode}-{n}';expected[request]=d.publish(r,request)['result']['result']
 d.call(r,'packing',operation='build',bootstrap=True,force=True,codec='raw',cleanup=False)
 inputs=overlap_fixture(r,mode=='A');before=raw_map(r);discovery=copy.deepcopy(work(r).get('discovery'));selected=(r/'SELECTED.json').read_bytes();parent=(r/'STORE.json').read_bytes();d.call(r,'packing',operation='verify')
 if red:
  out=d.call(r,'packing',operation='convert',force=True,cleanup=False,code=2);assert (r/'STORE.json').read_bytes()==parent;assert 'exist' in out['error'].lower(),out
  rows.append(dict(case='f9-F3-reproduced',mode=mode,error=out['error'],source_births=2,duplicate_and_different_index_keys=True,selected_unchanged=True,journal=json.loads((r/'PACKING.json').read_bytes())['status']));continue
 d.call(r,'packing',operation='convert',force=True,cleanup=False,code=86,env={'EXP_EXIT':'pack-index'});aborted=json.loads((r/'PACKING.json').read_bytes())['id'];assert (r/'STORE.json').read_bytes()==parent
 fail=d.call(r,'packing',operation='resume',cleanup=False,code=2);assert 'incomplete' in fail['error'];d.call(r,'packing',operation='abort')
 out=d.call(r,'packing',operation='convert',force=True,cleanup=False);assert config(r)['physical']['id']!=aborted;assert (r/'SELECTED.json').read_bytes()==selected and work(r).get('discovery')==discovery
 assert raw_map(r)==before;verified=d.call(r,'packing',operation='verify')['result'];assert verified['records']==len(before)
 for request,value in expected.items():assert d.call(r,'resolve',request_id=request)['result']==value
 for x in catalog(r)['packs']:
  p=path(r,x,'idx')
  with p.open('r+b') as f:v=f.read(1);f.seek(0);f.write(bytes([v[0]^1]))
  assert d.call(r,'packing',operation='verify',code=2)['status']=='error'
  with p.open('r+b') as f:f.write(v)
 for p in set(path(r,x,'pack') for x in catalog(r)['packs']):
  with p.open('r+b') as f:v=f.read(1);f.seek(0);f.write(bytes([v[0]^1]))
  assert d.call(r,'packing',operation='verify',code=2)['status']=='error'
  with p.open('r+b') as f:f.write(v)
 for n in range(120):
  cleaned=d.call(r,'packing',operation='cleanup')['result']
  if cleaned['status']=='complete':break
 else:raise AssertionError(cleaned)
 assert not (r/f'packs/{aborted}').exists() and all(not path(r,x,'idx').exists() and not path(r,x,'pack').exists() for x in inputs)
 assert raw_map(r)==before and work(r).get('discovery')==discovery
 for request,value in expected.items():assert d.call(r,'resolve',request_id=request)['result']==value
 d.call(r,'audit');d.publish(r,f'{name}-{mode}-successor')
 rows.append(dict(case='F3-owner-dedup-alias-crash',mode=mode,input_descriptors=len(inputs),logical_records=len(before),output_descriptors=len(catalog(r)['packs']),discovery_none=discovery is None,all_variant_corruptions_refused=True,aborted_birth_reaped=True,accepted_outcomes=len(expected)+1,conversion_metrics=out['metrics']))
(e/f'cp3-{name}-results.json').write_text(json.dumps(dict(status='REPRODUCED_F9_F3' if red else 'PASS',binary=d.binary_sha256,cases=rows),indent=2)+'\n');print(json.dumps(dict(status='PASS',cases=len(rows))))
