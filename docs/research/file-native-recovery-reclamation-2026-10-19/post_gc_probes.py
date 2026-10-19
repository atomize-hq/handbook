"""Fresh-process historical probes compare against the untouched original."""
import argparse,hashlib,json,subprocess,sys,time
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parent.parent/'file-native-incremental-engine-experiment-2026-10-18'))
from verify_store import CachedPoints,checked
a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--original',required=True);a.add_argument('--clone',required=True);a.add_argument('--output',required=True);ns=a.parse_args();original=Path(ns.original);clone=Path(ns.clone);selected=json.loads((original/'SELECTED.json').read_bytes());head=json.loads(checked(original/f"publications/{selected['publication']}.json",selected['digest']));tree=CachedPoints(original);rows=[]
for request in ['initial','history-000001','history-000101','history-050000','history-100000','deep-lost-reply']:
 ref=dict(selected=selected,result_hash=head['result_hash']) if head['request']==request else tree.point(head['outcomes'],hashlib.sha256(request.encode()).hexdigest());assert ref
 expected=json.loads(checked(original/f"results/{ref['result_hash']}.json",ref['result_hash']))
 for action in ['resolve','point']:
  payload=dict(action=action,root=str(clone),**(dict(request_id=request) if action=='resolve' else dict(selected=ref['selected'],id='d000000')));start=time.perf_counter_ns();p=subprocess.run([ns.exe],input=json.dumps(payload).encode(),capture_output=True,timeout=180);elapsed=time.perf_counter_ns()-start;out=json.loads(p.stdout);retired=(clone/f"retired/{ref['selected']['publication']}.json").exists()
  if action=='resolve':assert p.returncode==0 and out['result']==expected;status='success'
  elif retired:assert p.returncode==2 and out['error']=='retired:snapshot';status='expected refusal'
  else:
   assert p.returncode==0
   publication=json.loads(checked(original/f"publications/{ref['selected']['publication']}.json",ref['selected']['digest']));header=tree.point(publication['graph']['tree'],'D/d000000');assert out['result']['value']==header
   doc=header['doc'];assert checked(clone/doc['original'],doc['digest'])==checked(original/doc['original'],doc['digest']);status='success'
  rows.append(dict(request=request,action=action,status=status,elapsed_ns=elapsed,input=payload,exit=p.returncode,response=out))
for fence in sorted((original/'fences').iterdir()):
 assert (clone/'fences'/fence.name).read_bytes()==fence.read_bytes();request=json.loads(fence.read_bytes());assert hashlib.sha256(request.encode()).hexdigest()==fence.name
 before=(clone/'SELECTED.json').read_bytes()
 for action in ['resolve','publish']:
  payload=dict(root=str(clone),action=action,**(dict(request_id=request) if action=='resolve' else dict(request=dict(request=request,base=json.loads(before),ops=[],packet=False))))
  p=subprocess.run([ns.exe],input=json.dumps(payload).encode(),capture_output=True,timeout=180);out=json.loads(p.stdout)
  if action=='resolve':assert p.returncode==0 and out['result']==dict(outcome='cancelled-fenced',terminal_noncommit=True);status='success'
  else:assert p.returncode==2 and out['error']=='refused:cancelled-fenced';status='expected refusal'
  assert (clone/'SELECTED.json').read_bytes()==before;rows.append(dict(request=request,action=action,status=status,input=payload,exit=p.returncode,response=out,restored_terminal_fence=True))
Path(ns.output).write_text(json.dumps(dict(binary_sha256=hashlib.sha256(Path(ns.exe).read_bytes()).hexdigest(),rows=rows,qualification='Fresh OS processes; caches uncontrolled. Intentionally retired point content is unavailable, never a retained-content pass.'),indent=2),encoding='utf-8');print([(r['request'],r['action'],r['status']) for r in rows])
