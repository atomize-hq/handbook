"""Read complete selected raw streams; no execution, cache, build, or engine work."""
import gzip,hashlib,json
from pathlib import Path
import admission_v20 as admission
R=admission.ROOT

def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def raw(path):
 with gzip.open(path,'rt') as f:return [json.loads(x) for x in f]

def verify_outer(name):
 p=R/'evidence/control'/name;d=json.loads((p/'result.json').read_bytes());rows=raw(p/'samples.jsonl.gz');rss=[x for x in rows if x.get('kind')=='rss']
 assert len(rows)==d['raw_rows_written'] and len(rss)>=d['sample_count']
 assert all(x['monotonic']<=rss[d['sample_count']-1]['monotonic'] for x in rss[:d['sample_count']])
 gap=max((b['monotonic']-a['monotonic'] for a,b in zip(rss,rss[1:])),default=0)
 assert gap<=.04 and d['sample_gap_max_s']<=.04
 assert d['censor'] is None and d['worker_exit']==0 and d['workload_GO'] and d['token_released'] and d['closure']=='PROVEN_EMPTY_TWICE'
 for proof in d['closure_proofs']:
  assert proof['proven'] and len(proof['observations'])>=2
  for o in proof['observations'][-2:]:
   assert o['empty'] and not o['remaining'] and not o['unknown_errno_observations']
   for g in o['groups']:assert g['valid'] and g['returned_bytes']==0 and g['errno'] in (0,3) and not g['members']
 return {'name':name,'raw_sha256':sha(p/'samples.jsonl.gz'),'gzip_EOF':True,'rss_rows':len(rss),'reported_workload_end_samples':d['sample_count'],'extra_closeout_RSS_rows':len(rss)-d['sample_count'],'all_raw_rows':len(rows),'max_gap_s':gap,'two_numeric_empty_group_observations':True,'token_released':True}

def verify_role(p):
 d=json.loads((p/'result.json').read_bytes());rows=raw(p/'samples.jsonl.gz');rss=[x for x in rows if x.get('kind')=='role-RSS'];assert len(rss)==d['samples']
 gap=max((b['monotonic']-a['monotonic'] for a,b in zip(rss,rss[1:])),default=0);assert gap<=.04 and d['max_start_gap_s']<=.04
 assert len(d['closure'])==2 and all(x['absent'] for x in d['closure'])
 for c in d['closure']:
  assert c['identities'] and all(x['returned_bytes']==0 and x['errno'] in (0,3) for x in c['identities'])
 assert d['limit_bytes']=={'reader':268435456,'checker':2147483648}[d['role']]
 for x in d['outputs'].values():assert Path(x['path']).stat().st_size==x['bytes'] and sha(x['path'])==x['sha256']
 if not d['failures']:assert d['output_complete'] and d['peak_RSS_bytes']>0 and d['seconds']<=d['guarded_seconds']
 else:assert p.name.startswith(('output-cap-tiny-v30','role-failure-'))
 return {'name':p.name,'role':d['role'],'rss_limit_bytes':d['limit_bytes'],'raw_sha256':sha(p/'samples.jsonl.gz'),'gzip_EOF':True,'samples':len(rss),'max_gap_s':gap,'exit':d['exit'],'failures':d['failures'],'output_complete':d['output_complete'],'output_bytes':sum(x['bytes'] for x in d['outputs'].values()),'two_positive_identity_absences':True}

def main():
 expected={'guard_v20.py':'c33fa117667de50c9010e27830a1ec63019deb7186d4a047e439bda04530cfb4','monitor_v20.py':'7b3f3fd0c79ddb3fd4115c22c0b17c1d7914ce2aca0aff54f16fcd4587bf52a7','keeper_v20.py':'5a96e449f32f971dbd57f2ca62cdc194920858ae9708d5ecd6bec25e19aa8978','admission_v20.py':'aaf37aa66d486805a5fed459ba632d39bc65749d67ee153bc92a575b69410fe3','role_driver_v23.py':'f79bc1737b0ff0ba4fff4e4db4e15ffb2acce82cd88cf79a81e66a3c516ffa98'}
 for n,h in expected.items():assert sha(R/'execution/native-macos'/n)==h
 outer=[verify_outer(n) for n in ['adapter-tiny-v30-outer','ordinary-tiny-v30-outer','output-cap-tiny-v30-outer','role-failure-v29b-outer']]
 paths=sorted(p for p in (R/'evidence/roles').iterdir() if p.name.startswith(('adapter-tiny-v30-','ordinary-tiny-v30-','output-cap-tiny-v30-','role-failure-')) and (p/'result.json').exists())
 roles=[verify_role(p) for p in paths];assert len(roles)==11
 for n,count in [('adapter-tiny-v30',5),('ordinary-tiny-v30',3),('output-cap-tiny-v30',1)]:
  rows=raw(R/'evidence/capsules'/n/'calls.jsonl.gz');assert len(rows)==count
 assert not (R/'cargo-private').exists() and not (R/'target').exists() and not (R/'HOST-FENCE.json').exists()
 row={'kind':'adapter-final-v30-selected-raw-verification','core_v20_and_role_v23_frozen':expected,'candidate_adapter_sha256':sha(R/'execution/native-macos/capsule_adapter_v30.py'),'candidate_role_driver_sha256':sha(R/'execution/native-macos/role_driver_v29.py'),'outer_controls':outer,'role_controls':roles,'capsule_call_logs_EOF':True,'full_engine_capsules_or_build_executed':False,'cache_directory_created':False}
 admission.bounded_receipt(R/'evidence/control/adapter-raw-review-v30.json',row)
 print(json.dumps({'outer_controls':len(outer),'role_controls':len(roles),'largest_outer_gap':max(x['max_gap_s'] for x in outer),'largest_role_gap':max(x['max_gap_s'] for x in roles),'review_sha256':sha(R/'evidence/control/adapter-raw-review-v30.json')}))
if __name__=='__main__':main()
