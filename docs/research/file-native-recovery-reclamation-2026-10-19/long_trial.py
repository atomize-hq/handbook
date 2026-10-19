"""Actual collection only on an independently restored, verified owned clone."""
import argparse,hashlib,json,os,subprocess,time
from pathlib import Path
from bounded import rss
from inventory import census

def run(exe,root,evidence,gate):
    root=Path(root).resolve();evidence=Path(evidence).resolve();evidence.mkdir(parents=True,exist_ok=False)
    authorized=Path('C:/file-native-recovery-20261019/history-restored/disposable/store').resolve()
    if root!=authorized:raise ValueError('exact-new-owned-destination-required')
    proof=json.loads(Path(gate).read_bytes());assert proof['status']=='operational-restore-verified' and Path(proof['destination']).resolve()==root
    assert root!=Path(proof['original']).resolve() and not (root/'DISPOSABLE.json').exists()
    clock=json.loads((Path(__file__).parent/'evidence/execution-clock.json').read_bytes());deadline=clock['work_deadline_epoch'];rows=[];count=0
    def call(action,cap=180,exit_at=None,code=0,**fields):
        nonlocal count
        assert time.time()+cap<deadline
        count+=1;payload=dict(action=action,root=str(root),**fields);env={k:v for k,v in os.environ.items() if not k.startswith('EXP_')}
        if exit_at:env['EXP_EXIT']=exit_at
        prefix=evidence/f'{count:04}-{action}';Path(str(prefix)+'.input.json').write_text(json.dumps(payload,indent=2),encoding='utf-8');start=time.time();peak=0;censor=None
        with Path(str(prefix)+'.stdout').open('wb') as out,Path(str(prefix)+'.stderr').open('wb') as err:
            p=subprocess.Popen([str(exe)],stdin=subprocess.PIPE,stdout=out,stderr=err,env=env);p.stdin.write(json.dumps(payload).encode());p.stdin.close()
            while p.poll() is None:
                peak=max(peak,rss(p.pid))
                if time.time()-start>cap:censor='time-cap'
                if peak>2*1024**3:censor='memory-cap'
                if censor:p.kill();p.wait(timeout=30);break
                time.sleep(.1)
        row=dict(input=payload,exit_at=exit_at,exit=p.returncode,seconds=time.time()-start,peak_memory_bytes=peak,status='censored' if censor else ('success' if p.returncode==code else 'failure'),censor=censor)
        rows.append(row);(evidence/'events.json').write_text(json.dumps(rows,indent=2),encoding='utf-8')
        assert not censor and p.returncode==code,row
        return json.loads(Path(str(prefix)+'.stdout').read_bytes()) if p.returncode==0 else None
    def selected():return json.loads((root/'SELECTED.json').read_bytes())
    def pub(name,ops):return call('publish',request=dict(request=name,base=selected(),ops=ops,packet=False))['result']
    def inventory(name):
        result=census(root);(evidence/(name+'.json')).write_text(json.dumps(result,indent=2),encoding='utf-8');return result
    before=inventory('before');call('lifecycle',operation='setup')
    # Controlled process exit at the first actual reclaimed file, then restart.
    call('lifecycle',operation='collect',cap=1800,code=86,exit_at='reclaim-file')
    first=call('lifecycle',operation='collect',cap=1800);after=inventory('after-no-retirement')
    assert selected()==proof['selected']
    (evidence/'no-retirement-completed.json').write_text(json.dumps(dict(before=before,after=after,collection=first,controlled_exit=86),indent=2),encoding='utf-8')
    # At most 100 prospectively selected early historical publications.
    cohort=[]
    with (root/'expected-ledger.jsonl').open('rb') as f:
        for _ in range(100):
            line=f.readline()
            if not line:break
            cohort.append(json.loads(line)['selected'])
    (evidence/'prospective-cohort.json').write_text(json.dumps(cohort,indent=2),encoding='utf-8')
    for s in cohort:call('lifecycle',operation='retire',selected=s)
    prior=call('point',id='d000000')['result']['value']['doc'];prior_bytes=(root/prior['original']).read_bytes()
    distinct=pub('reclamation-distinct-source',[dict(kind='import',id='d000000',text='# Unique reclamation source, October 19 2026\r\n')]);unique_selector=distinct['selected']
    unique=call('point',id='d000000')['result']['value']['doc'];assert unique['digest']!=prior['digest']
    accepted=call('resolve',request_id='reclamation-distinct-source')['result']
    pub('reclamation-restore-prior-bytes',[dict(kind='import',id='d000000',text=prior_bytes.decode())])
    if json.loads((root/'STORE.json').read_bytes())['mode']=='A':call('refresh')
    call('lifecycle',operation='retire',selected=unique_selector)
    second=call('lifecycle',operation='collect',cap=1800);final=inventory('after-retirement')
    assert not (root/unique['original']).exists()
    assert call('resolve',request_id='reclamation-distinct-source')['result']==accepted
    assert (root/prior['original']).read_bytes()==prior_bytes
    call('point',selected=unique_selector,id='d000000',code=2)
    call('audit')
    result=dict(status='success',first_collection=first,second_collection=second,retired_cohort=len(cohort),distinct_source_reclaimed=True,accepted_result_exact=True,unique_selector=unique_selector,before=before,after_no_retirement=after,after_retirement=final)
    (evidence/'completed.json').write_text(json.dumps(result,indent=2),encoding='utf-8');print(json.dumps({k:v for k,v in result.items() if not k.startswith(('before','after'))},indent=2))

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--root',required=True);a.add_argument('--evidence',required=True);a.add_argument('--gate',required=True);ns=a.parse_args();run(ns.exe,ns.root,ns.evidence,ns.gate)
