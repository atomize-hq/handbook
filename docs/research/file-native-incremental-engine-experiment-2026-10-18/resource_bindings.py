"""Join complete observed process resources to exact controller calls, or leave absent."""
import argparse,base64,hashlib,json,os,re
from pathlib import Path

a=argparse.ArgumentParser();a.add_argument('--scratch',required=True);a.add_argument('--output',required=True);ns=a.parse_args()
scratch=Path(ns.scratch);observed=[]
for name in ['windows-resource-observer.jsonl','windows-audit-resource-observer.jsonl']:
 for line_no,line in enumerate((scratch/name).read_bytes().splitlines(),1):
  value=json.loads(line)
  if value['kind']=='exited-process':observed.append((name,line_no,line,value))
rows=[];calls=0
for directory,dirs,names in os.walk(scratch):
 dirs[:]=[name for name in dirs if name not in {'store','disposable','target','__pycache__','.git'} and not name.startswith('build') and not (Path(directory)/name/'STORE.json').exists()]
 if 'events.jsonl' not in names:continue
 path=Path(directory)/'events.jsonl'
 for line_no,line in enumerate(path.read_bytes().splitlines(),1):
  event=json.loads(line);calls+=1;start=event.get('controller_start_ns');end=event.get('controller_end_ns')
  if start is None or end is None:continue
  # Older drivers omitted PID. Only writer-producing dispositions identify this
  # process; read-only/dedup calls may return a concurrent writer's publication.
  action=event['input']['action'];result=(event.get('output') or {}).get('result') or {}
  if not isinstance(result,dict):result={}
  writer=action in {'init','publish','select','checkpoint'} and result.get('status') in {'published','published/delivery-pending','selection-uncertain'}
  resident=action=='history' and result.get('completed',0)>0 and result.get('completed')==event['input'].get('count') and result.get('independent_ledger')=='expected-ledger.jsonl'
  publication=result.get('selected',{}).get('publication','') if writer or resident else ''
  identity=re.fullmatch(r'publication-(\d+)-(\d+)-\d+',publication)
  publications=[identity.groups()] if identity else []
  matches=[]
  for name,observer_line,raw,value in observed:
   if value['exe_sha256']!=event['exe_sha256'] or not start<=value['first_observed_ns']<=end or value['observed_exit_ns']>end+2_000_000_000:continue
   if event.get('process_id') is not None:
    if event['process_id']!=value['pid']:continue
    method='explicit process_id, executable hash and monotonic call window'
   else:
    created=(value['created_filetime']-116444736000000000)*100;exited=(value['exit_filetime']-116444736000000000)*100
    if not any(int(pid)==value['pid'] and created<=int(epoch)<=exited for pid,epoch in publications):continue
    method='selected writer PID/epoch inside OS process lifetime, executable hash and monotonic call window'
   matches.append(dict(observer=name,observer_line=observer_line,observer_line_b64=base64.b64encode(raw).decode(),observer_line_sha256=hashlib.sha256(raw).hexdigest(),identity_method=method,resources=value))
  assert len(matches)<=1,(str(path),line_no,'ambiguous observed process')
  if matches:rows.append(dict(raw_event=path.relative_to(scratch).as_posix(),raw_line=line_no,action=event['input']['action'],request=event['input'].get('request',{}).get('request'),controller_ms=event['elapsed_ns']/1e6,**matches[0]))
report=dict(controller_calls_scanned=calls,bound_observed_calls=len(rows),rows=rows,qualification='Only exited observed processes with checked identities are joined. Unmatched calls have unavailable CPU/peak observations, not zero. Complete OS process CPU includes startup. Sampling and filesystem watchdogs add observation overhead; these are not physical-I/O measurements.')
Path(ns.output).write_text(json.dumps(report,indent=2));print(json.dumps(dict(calls=calls,bound=len(rows))))
