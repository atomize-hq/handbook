import sys,json,hashlib
from pathlib import Path
sys.path.insert(0,'C:/Users/spmcc/.codex/worktrees/file-native-online-matrix-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/harness')
from driver import Driver
b=Path('C:/file-native-online-cp1-20261020');e=b/'evidence';events=e/'cp3-conversion-events.jsonl';d=Driver(b/'build/release/file-native-incremental.exe',events);old=Driver('C:/file-native-packing-20261020/bin/candidate2.exe',events);rows=[]
for mode in ['A','B']:
 r=b/f'fixtures/cp3/conversion-final-{mode}/disposable/store';old.call(r,'init',mode=mode,consumer='project',count=10);old.call(r,'lifecycle',operation='setup');expected={'initial':old.call(r,'resolve',request_id='initial')['result']}
 for n in range(3):
  name=f'final-{mode}-{n}';expected[name]=old.publish(r,name)['result']['result']
 old.call(r,'packing',operation='build',codec='zstd',include_attempts=True,cleanup=False);head=(r/'SELECTED.json').read_bytes();out=d.call(r,'packing',operation='convert',force=True,cleanup=False);assert (r/'SELECTED.json').read_bytes()==head
 config=json.loads((r/'STORE.json').read_bytes());work=json.loads((r/f"physical-plans/{config['physical']['plan']}.json").read_bytes());assert work.get('discovery') is None
 audit=d.call(r,'packing',operation='verify');assert audit['result']['records']==20
 for name,value in expected.items():assert d.call(r,'resolve',request_id=name)['result']==value
 d.call(r,'audit')
 rows.append(dict(mode=mode,records=20,exact_outcomes=4,input_correspondence='streaming key+raw-length+raw-SHA preimage equality',source_cache_peak=out['metrics']['pack_cache_peak_bytes'],page_actual_resident_peak=out['metrics']['pack_page_resident_peak_bytes'],page_parse_peak=out['metrics']['pack_page_parse_peak_bytes']))
(e/'cp3-conversion-results.json').write_text(json.dumps(dict(status='PASS',binary=d.binary_sha256,cases=rows),indent=2)+'\n');print(json.dumps(dict(status='PASS',cases=len(rows))))
