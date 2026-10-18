import argparse,json,time
from controller import Driver
from checks import result,req,expect_error
from verify_store import verify_history
a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);a.add_argument('--max-depth',type=int,default=100000);ns=a.parse_args()
d=Driver(ns.exe,ns.base,13500);root=str(d.base/'store');rows=[]
d.call(dict(action='init',root=root,mode='A',consumer='project',count=16))
start=0
for depth in [1000,10000,100000]:
    if depth>ns.max_depth:break
    observation=d.call(dict(action='history',root=root,count=depth-start,start=start),cap=14400,env={'EXP_DEFER_REFRESH':'1'},allow=True)
    # A censored call remains censored; independently count durable evidence.
    verification=verify_history(root,allow_censored_prefix=not observation or observation['status']!='ok');rows.append({'requested_depth':depth,'history_observation':observation,'verification':verification})
    (d.base/'history.json').write_text(json.dumps(rows,indent=2))
    if not observation or observation['status']!='ok':break
    for i in [1,depth//2,depth]:
        d.call(dict(action='resolve',root=root,request_id=f'history-{i:06}'))
    first=json.loads((d.base/'store/expected-ledger.jsonl').open().readline())
    p=json.loads((d.base/f"store/publications/{first['selected']['publication']}.json").read_bytes())
    c=json.loads((d.base/f"store/attempts/{p['attempt']}/candidate.json").read_bytes())
    retry=d.call(dict(action='publish',root=root,request=c['request']))
    assert retry['result']['result']==first['result']
    assert d.call(dict(action='cancel',root=root,request_id='history-000001'))['result']==first['result']
    d.call(dict(action='cancel',root=root,request_id=f'cancelled-at-{depth}'))
    d.call(dict(action='resolve',root=root,request_id=f'cancelled-at-{depth}'))
    d.call(dict(action='point',root=root,selected=first['selected'],id='d000001'))
    d.call(dict(action='resolve',root=root,request_id='missing'))
    d.call(dict(action='inspect',root=root))
    d.call(dict(action='point',root=root,id='d000001'))
    d.call(dict(action='audit',root=root),cap=900)
    start=depth
# Checkpoint and refresh are separate from append timings.
if start and time.monotonic()<d.end-120:
    d.call(dict(action='checkpoint',root=root,request_id='history-checkpoint'),cap=900,env={'EXP_DEFER_REFRESH':'1'})
    d.call(dict(action='refresh',root=root),cap=900)
    rows.append({'after_checkpoint':verify_history(root)})
    victim=req('deep-lost-reply',[])
    d.call(dict(action='publish',root=root,request=victim),env={'EXP_EXIT':'reply','EXP_DEFER_REFRESH':'1'},allow=True)
    event=json.loads(d.events.read_bytes().splitlines()[-1]);assert event['exit']==86
    original=d.call(dict(action='resolve',root=root,request_id='deep-lost-reply'))['result'];assert original['outcome']=='accepted'
    for i in range(1000):
        d.call(dict(action='publish',root=root,request=req(f'after-lost-{i:04}',[])),env={'EXP_DEFER_REFRESH':'1'})
    assert d.call(dict(action='resolve',root=root,request_id='deep-lost-reply'))['result']==original
    expect_error(d.call(dict(action='publish',root=root,request=req('deep-lost-reply',[],packet=True)),allow=True),'intent-mismatch')
    rows.append({'lost_reply_original_exit':86,'later_real_successors':1000,'original_outcome_retained':True,'full_verification':verify_history(root)})
    (d.base/'history.json').write_text(json.dumps(rows,indent=2))
    d.call(dict(action='lifecycle',root=root,operation='setup'))
    collection=d.call(dict(action='lifecycle',root=root,operation='collect'),cap=900,allow=True)
    rows.append({'bounded_long_history_collection':collection})
    assert d.call(dict(action='resolve',root=root,request_id='history-000001'))['result']==first['result']
    assert d.call(dict(action='resolve',root=root,request_id='deep-lost-reply'))['result']==original
(d.base/'history.json').write_text(json.dumps(rows,indent=2))
