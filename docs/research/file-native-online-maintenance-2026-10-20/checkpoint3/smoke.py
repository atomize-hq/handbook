import sys,json,hashlib,threading,time,gzip
from pathlib import Path
sys.path.insert(0,'C:/Users/spmcc/.codex/worktrees/file-native-online-matrix-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/harness')
from driver import Driver
b=Path('C:/file-native-online-cp1-20261020');e=b/'evidence';name=sys.argv[1] if len(sys.argv)>1 else 'smoke';events=e/f'cp3-{name}-events.jsonl';results=e/f'cp3-{name}-results.json'
d=Driver(b/'build/release/file-native-incremental.exe',events)
old=Driver('C:/file-native-packing-20261020/bin/candidate2.exe',events)
assert old.binary_sha256=='568f631acf528c7de911bb88ec85d07ead9d7b4435ebe970c61053f06e31a93b'
rows=[]
def note(name,**kw):
 rows.append(dict(case=name,**kw));results.write_text(json.dumps(dict(status='RUNNING',binary=d.binary_sha256,cases=rows),indent=2)+'\n')
def catalog(r):
 c=json.loads((r/'STORE.json').read_bytes());return json.loads((r/f"pack-catalogs/{c['catalog']}.json").read_bytes())
def work(r):
 c=json.loads((r/'STORE.json').read_bytes());return json.loads((r/f"physical-plans/{c['physical']['plan']}.json").read_bytes())
def clean(r):
 for n in range(120):
  out=d.call(r,'packing',operation='cleanup')['result']
  if out['status']=='complete':return out
 raise AssertionError(out)
def wait(p):
 until=time.monotonic()+10
 while not p.exists():assert time.monotonic()<until;time.sleep(.005)
for mode in ['A','B']:
 r=b/f'fixtures/cp3/{name}-{mode}/disposable/store';old.call(r,'init',mode=mode,consumer='project',count=16);old.call(r,'lifecycle',operation='setup');expected={'initial':old.call(r,'resolve',request_id='initial')['result']}
 for n in range(3):
  request=f'legacy-{mode}-{n}';expected[request]=old.publish(r,request)['result']['result']
 old.call(r,'packing',operation='build',codec='zstd',include_attempts=True,cleanup=False)
 original_publication=(r/f"publications/{expected['initial']['publication']}.json").read_bytes()
 before={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in (r/'packs').glob('*') if p.is_file()};semantic=(r/'SELECTED.json').read_bytes()
 if mode=='A':
  ready=r.parent/'legacy-ready';release=r.parent/'legacy-release';held=[]
  def hold():held.append(old.call(r,'resolve',request_id='initial',env={'EXP_BARRIER':'pack-catalog-captured','EXP_READY':str(ready),'EXP_RELEASE':str(release)}))
  t=threading.Thread(target=hold);t.start();wait(ready)
  out=d.call(r,'packing',operation='convert',force=True,cleanup=False,code=2,env={'EXP_WAIT_MS':'100'})
  assert out['error']=='busy:legacy-readers',out
  release.write_text('release');t.join(10);assert not t.is_alive() and held
  out=d.call(r,'packing',operation='resume',cleanup=False)
 else:out=d.call(r,'packing',operation='convert',force=True,cleanup=False)
 assert (r/'SELECTED.json').read_bytes()==semantic
 assert work(r).get('discovery') is None
 cat=catalog(r);assert cat['version']==4 and all(x['page']['version']==2 and x['instance'] for x in cat['packs'])
 assert all(hashlib.sha256((r/'packs'/name).read_bytes()).hexdigest()==sha for name,sha in before.items())
 audited=d.call(r,'packing',operation='verify')['result'];assert audited['records']==sum(x['records'] for x in cat['packs'])
 refusal=old.call(r,'resolve',request_id='initial',code=2);assert refusal['status']=='error' and refusal['metrics']['bytes_written']==0,refusal
 cleaned=clean(r);assert not any((r/'packs'/name).exists() for name in before)
 note('legacy-full-conversion',mode=mode,records=audited['records'],old_pair_files=len(before),actual_reaped=cleaned['successfully_deleted_bytes_recorded'],discovery_preserved_none=True,upgrade_wait_ns=out['metrics']['physical_upgrade_wait_ns'])
 # Explicit ordinary rolling bootstrap remains required after conversion.
 out=d.call(r,'packing',operation='build',codec='zstd',force=True,bootstrap=True,cleanup=False);clean(r)
 for cycle in range(3):
  for n in range(16):
   request=f'{mode}-tail-{cycle}-{n}';expected[request]=d.publish(r,request)['result']['result']
  semantic=(r/'SELECTED.json').read_bytes();out=d.call(r,'packing',operation='build',codec='zstd',force=True,cleanup=False)
  assert out['metrics']['rolling_publications_visited']==16 and (r/'SELECTED.json').read_bytes()==semantic
  clean(r);d.call(r,'packing',operation='verify');d.call(r,'audit')
  assert len(set(x.get('segment',0) for x in catalog(r)['packs']))<=4
  note('paged-rolling-cycle',mode=mode,cycle=cycle,publications=16,work_bytes=len((r/f"physical-plans/{json.loads((r/'STORE.json').read_bytes())['physical']['plan']}.json").read_bytes()))
 for request,result in expected.items():
  out=d.call(r,'resolve',request_id=request);assert out['result']==result
  m=out['metrics'];assert m['physical_capture_bytes']<=65536 and m['pack_key_page_read_max']<=12 and m['pack_key_index_bytes_max']<=196608 and m['pack_cache_peak_bytes']<=67108864
 note('exact-outcomes',mode=mode,count=len(expected))
 # A mapped raw failure refuses even while an exact loose duplicate survives.
 result=d.call(r,'resolve',request_id='initial')['result'];publication=result['publication'];key=f'publications/{publication}.json';loose=r/key
 if not loose.exists():loose.write_bytes(original_publication)
 assert loose.read_bytes()==original_publication
 desc=next(x for x in catalog(r)['packs'] if x['role']=='publication' and x['first']<=key<=x['last'])
 # Hash-bind failure in a touched root, with loose exact bytes present.
 idx=r/f"packs/{desc['instance']}/{desc['index']}.idx"
 with idx.open('r+b') as f:byte=f.read(1);f.seek(0);f.write(bytes([byte[0]^1]))
 fail=d.call(r,'resolve',request_id='initial',code=2);assert 'paged-page-hash' in fail['error'],fail
 with idx.open('r+b') as f:f.write(byte)
 assert d.call(r,'resolve',request_id='initial')['result']==expected['initial']
 note('mapped-root-damage-loose-duplicate-refuses',mode=mode)
results.write_text(json.dumps(dict(status='PASS',binary=d.binary_sha256,cases=rows),indent=2)+'\n')
print(json.dumps(dict(status='PASS',cases=len(rows))))
