"""Checkpoint-specific faults, waiting cancellation and killed coordination."""
import argparse,json,threading,time
from controller import Driver
from checks import req,op,result,expect_error

def run(d):
    rows=[]
    boundaries=['checkpoint-built','checkpoint-verified','seal','expected-checked','selection-budget','membership-root','outcome-root','result-installed','publication-installed','attempt-hint','selector-staged','selector-visible','selection-boundary','accounting-transfer','accounting-release','reply']
    for boundary in boundaries:
        root=str(d.base/boundary/'disposable'/'store')
        d.call(dict(action='init',root=root,mode='A',consumer='project',count=16))
        d.call(dict(action='publish',root=root,request=req('before',[])))
        out=d.call(dict(action='checkpoint',root=root,request_id='checkpoint-victim'),env={'EXP_EXIT':boundary,'EXP_DEFER_REFRESH':'1'},allow=True)
        event=json.loads(d.events.read_bytes().splitlines()[-1]);assert event['exit']==86
        resolved=result(d.call(dict(action='resolve',root=root,request_id='checkpoint-victim')))
        if boundaries.index(boundary)<boundaries.index('selector-visible'):assert resolved['outcome']=='absent-as-of'
        if boundaries.index(boundary)>=boundaries.index('selection-boundary'):assert resolved['outcome']=='accepted'
        d.call(dict(action='publish',root=root,request=req('after',[])),env={'EXP_DEFER_REFRESH':'1'})
        assert result(d.call(dict(action='resolve',root=root,request_id='checkpoint-victim')))==resolved
        d.call(dict(action='audit',root=root));rows.append({'checkpoint_boundary':boundary,'original_exit':86,'recovered':resolved['outcome']})
    root=str(d.base/'waiting'/'store');d.call(dict(action='init',root=root,mode='A',consumer='research',count=16))
    ready=d.base/'holder-ready';release=d.base/'holder-release';hold=[];wait=[]
    holder=threading.Thread(target=lambda:hold.append(d.call(dict(action='publish',root=root,request=req('holder',[])),env={'EXP_BARRIER':'lock-acquired','EXP_READY':str(ready),'EXP_RELEASE':str(release)})))
    holder.start();deadline=time.monotonic()+15
    while not ready.exists():assert time.monotonic()<deadline;time.sleep(.01)
    waiter=threading.Thread(target=lambda:wait.append(d.call(dict(action='publish',root=root,request=req('waiting',[])),env={'EXP_CANCEL_ID':'waiting'},allow=True)))
    waiter.start();time.sleep(.05)
    d.call(dict(action='cancel-signal',root=root,request_id='waiting'));waiter.join(15)
    assert wait;expect_error(wait[0],'cancelled:while-waiting')
    release.write_text('release');holder.join(20);assert hold
    assert result(d.call(dict(action='cancel',root=root,request_id='waiting')))['terminal_noncommit'] is True
    d.call(dict(action='publish',root=root,request=req('killed-holder',[])),env={'EXP_EXIT':'lock-acquired'},allow=True)
    event=json.loads(d.events.read_bytes().splitlines()[-1]);assert event['exit']==86
    d.call(dict(action='publish',root=root,request=req('after-kill',[])));d.call(dict(action='audit',root=root))
    rows.append({'waiting_cancel':True,'killed_holder_recovered':True})
    return rows

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);ns=a.parse_args()
    d=Driver(ns.exe,ns.base,1800);(d.base/'deep-cases.json').write_text(json.dumps(run(d),indent=2))
