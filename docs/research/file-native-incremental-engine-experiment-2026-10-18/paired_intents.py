"""Two eligible preparations sharing an ID must not poison unrelated recovery."""
import argparse,json,time
from pathlib import Path
from controller import Driver
from checks import req,op,result,expect_error

a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);a.add_argument('--expect-poisoned',action='store_true');a.add_argument('--deadline-epoch',type=float);ns=a.parse_args()
d=Driver(ns.exe,ns.base,min(900,max(0,ns.deadline_epoch-time.time())) if ns.deadline_epoch is not None else 900);rows=[]
for mode in ['A','B']:
 root=d.base/mode/'disposable/store';r=str(root)
 def call(action,allow=False,**kw):return d.call(dict(action=action,root=r,**kw),allow=allow)
 call('init',mode=mode,consumer='project',count=16);call('lifecycle',operation='setup')
 first=req('paired-id',[op('label',text='first-intent')]);second=req('paired-id',[op('label',text='other-intent')])
 one=result(call('prepare',request=first))['candidate'];two=result(call('prepare',request=second))['candidate']
 loser=Path(two).parent.name
 accepted=result(call('select',candidate=one))['result']
 expect_error(call('select',allow=True,candidate=two),'intent-mismatch')
 unrelated=call('prepare',allow=True,request=req('unrelated',[op('cost',value=0)]))
 if ns.expect_poisoned:
  expect_error(unrelated,'intent-mismatch');expect_error(call('lifecycle',allow=True,operation='abandon',attempt=loser),'intent-mismatch')
  assert call('resolve',request_id='paired-id')['result']==accepted
  rows.append(dict(mode=mode,poisoned_recovery_reproduced=True,loser_cannot_be_abandoned=True,original_acceptance_readable=True));continue
 candidate=result(unrelated)['candidate'];call('select',candidate=candidate)
 # Losing intent remains refused; only its own reservation may be abandoned.
 expect_error(call('publish',allow=True,request=second),'intent-mismatch')
 call('lifecycle',operation='abandon',attempt=loser);collection=result(call('lifecycle',operation='collect'))
 assert call('resolve',request_id='paired-id')['result']==accepted
 assert result(call('publish',request=first))['result']==accepted
 call('publish',request=req('after-cleanup',[op('label',id='d000004',text='after-cleanup')]))
 call('audit');quota=result(call('lifecycle',operation='quota'))['reservations'];assert not quota
 rows.append(dict(mode=mode,unrelated_preparation_and_selection_succeeded=True,losing_intent_refused=True,loser_fenced_and_collected=True,original_acceptance_exact=True,reservations=quota,collection=collection))
(d.base/'paired-intents.json').write_text(json.dumps(dict(expect_poisoned=ns.expect_poisoned,rows=rows),indent=2))
print(json.dumps(rows,indent=2))
