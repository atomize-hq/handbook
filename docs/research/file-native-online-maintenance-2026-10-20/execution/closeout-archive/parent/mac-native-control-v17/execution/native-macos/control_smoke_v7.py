"""Bounded control proofs. No engine, compiler, dependency copy or corpus.
This supervisor publishes its own RSS while launching explicit guard children.
"""
import ctypes
import fcntl
import gzip
import gc
import hashlib
import json
import os
from pathlib import Path
import secrets
import signal
import subprocess
import sys
import threading
import time
import zlib
import admission_v7 as admission
import keeper_v7 as keeper
import monitor_v7 as monitor
from native_probe import identity,usage

ROOT=admission.ROOT
GUARD=Path(__file__).with_name('guard_v7.py')
HERE=Path(__file__).parent
RESULT=ROOT/'evidence/control/v16-remediation-smokes'


def base_prospective():
    return {k:{'bytes':131072 if k=='F01' else 2*1024**2 if k=='F12' else 0,
               'files':8 if k in ('F01','F12') else 0} for k in admission.LIMITS}


def write(name,row):
    data=(json.dumps(row,indent=2)+'\n').encode()
    if len(data)>262144:
        raise RuntimeError('control-receipt-cap')
    (RESULT/(name+'.json')).write_bytes(data)


def raw_check(path):
    data=path.read_bytes()
    z=zlib.decompressobj(31)
    raw=z.decompress(data,4*1024**2)
    complete=z.eof and not z.unused_data and not z.unconsumed_tail
    rows=[]
    if complete:
        rows=[json.loads(line) for line in raw.splitlines()]
    rss=[r for r in rows if r.get('kind')=='rss']
    gaps=[b['monotonic']-a['monotonic'] for a,b in zip(rss,rss[1:])]
    return {'compressed_sha256':hashlib.sha256(data).hexdigest(),'compressed_bytes':len(data),
            'gzip_eof_complete':complete,'json_rows':len(rows) if complete else None,
            'rss_rows':len(rss) if complete else None,'max_RSS_start_gap_s':max(gaps,default=0) if complete else None,
            'rows_match_receipt':None,'raw_censored':not complete}


def execute(name,code,extra=(),prospect=None,control=True,timeout=20):
    if (RESULT/'driver-publication-failure.json').exists():
        raise RuntimeError('driver-publication-failure-before-next-case')
    request=prospect or base_prospective()
    argv=[sys.executable,'-B',str(GUARD),'--name',name,'--seconds','12',
          '--prospective',json.dumps(request,separators=(',',':'))]
    if control:argv+=['--control-only']
    argv+=list(extra)+['--',sys.executable,'-B','-c',code]
    # Allow the report-only publisher to refresh after JSON/EOF processing;
    # no new workload starts behind stale helper telemetry.
    until=time.monotonic()+2
    while time.monotonic()<until:
        with (ROOT/'mac-supervisors.lock').open('a+b') as lock:
            fcntl.flock(lock.fileno(),fcntl.LOCK_SH)
            peers=admission.strict_json((ROOT/'mac-supervisors.json').read_bytes())
        if all(time.monotonic()-x['sample_monotonic']<.03 for x in peers.values()):break
        time.sleep(.005)
    else:raise RuntimeError('helper-telemetry-not-fresh-before-next-control')
    begun=time.time()
    p=subprocess.run(argv,capture_output=True,text=True,timeout=timeout)
    row={'name':name,'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr,'started_epoch':begun,
         'argv':argv,'guard_sha256':hashlib.sha256(GUARD.read_bytes()).hexdigest()}
    result=ROOT/'evidence/control'/name/'result.json'
    if result.exists():
        r=json.loads(result.read_bytes());row['result']=r
        raw=result.with_name('samples.jsonl.gz')
        row['raw']=raw_check(raw)
        row['raw']['rows_match_receipt']=row['raw']['json_rows']==r.get('raw_rows_written')
        record=ROOT/'evidence/control'/name/'keeper.json'
        if record.exists():row['keeper']=json.loads(record.read_bytes())
    write(name,row)
    print(json.dumps({'case':name,'exit':p.returncode,'censor':row.get('result',{}).get('censor'),
                      'closure':row.get('result',{}).get('closure'),'GO':row.get('result',{}).get('workload_GO'),
                      'EOF':row.get('raw',{}).get('gzip_eof_complete')}),flush=True)
    return row


def recover_hold(result):
    fence=admission.strict_json(keeper.FENCE.read_bytes())
    assert fence['nonce']==result['nonce']
    known={r['pid']:r for r in fence['known']}
    proof=keeper.closure(fence['anchor'],known)
    assert proof['proven'] and len(proof['observations'])>=2
    with (ROOT/'mac-heavy.lock').open('a+b') as token:
        try:
            fcntl.flock(token.fileno(),fcntl.LOCK_EX|fcntl.LOCK_NB)
            raise AssertionError('UNKNOWN-fence-did-not-retain-token')
        except BlockingIOError:
            pass
    refused=execute('fenced-probe-v16','raise RuntimeError("MUST_NOT_LAUNCH")')
    assert refused['exit']==1 and 'durable-host-fence' in refused['stdout']
    request=Path(fence['recovery_socket'])
    assert request.parent==ROOT and request.suffix=='.json'
    request.write_text(json.dumps({'nonce':fence['nonce'],'reviewed_release_after_positive_closure':True})+'\n')
    until=time.monotonic()+3
    while keeper.FENCE.exists() and time.monotonic()<until:time.sleep(.01)
    assert not keeper.FENCE.exists()
    # No signals or stale-PID recovery: a closed token is observed via flock.
    until=time.monotonic()+3
    with (ROOT/'mac-heavy.lock').open('a+b') as token:
        while True:
            try:
                fcntl.flock(token.fileno(),fcntl.LOCK_EX|fcntl.LOCK_NB)
                break
            except BlockingIOError:
                if time.monotonic()>=until:raise
                time.sleep(.005)
    write('explicit-HOLD-recovery-v16',{'fence':fence,'closure_before_request':proof,
          'new_heavy_refused':True,'token_retained_until_explicit_request':True,
          'release_only_after_two_positive_numeric_observations':True})


def run():
    gc.disable()
    sys.setswitchinterval(.001)
    RESULT.mkdir(exist_ok=False)
    nonce='control-driver-'+secrets.token_hex(8)
    registry=monitor.Registry(nonce)
    stop=threading.Event();failure=[];peer_refusals=[]
    me=identity(os.getpid())
    def publish():
        while not stop.is_set():
            try:
                i=identity(os.getpid());u=usage(os.getpid())
                if not monitor.same_identity(me,i) or u['return'] or not u['rss_bytes']:
                    raise RuntimeError('driver-positive-RSS-failed')
                peers=registry.update({'nonce':nonce,'sample_monotonic':time.monotonic(),'rss_bytes':u['rss_bytes'],'processes':[{'identity':i,'usage':u}],'stage':'report-only-control-driver'})
                now=time.monotonic()
                stale=[key for key,row in peers.items() if now-row['sample_monotonic']>.04]
                if stale and (not peer_refusals or peer_refusals[-1]['peers']!=stale):
                    peer_refusals.append({'monotonic':now,'peers':stale})
                if sum(row['rss_bytes'] for row in peers.values())>768*1024**2:
                    raise RuntimeError('all-supervisors-rss-cap')
            except BaseException as e:
                failure.append(repr(e));write('driver-publication-failure',{'error':repr(e),'monotonic':time.monotonic()});return
            stop.wait(.02)
    thread=threading.Thread(target=publish,daemon=True);thread.start()
    rows=[]
    try:
        # Real per-category bytes/files admission refuses every over-limit row,
        # without allocating the proposed excess or touching protected data.
        current=admission.snapshot(shared=True);caps=admission.limits();admissions=[]
        for category in admission.LIMITS:
            for unit in ('bytes','files'):
                requested=base_prospective();requested[category][unit]=caps[category][unit]+1
                try:
                    admission.admit(current,requested,caps)
                    raise AssertionError('over-limit-admission')
                except RuntimeError as error:
                    admissions.append({'category':category,'unit':unit,'refusal':str(error)})
        write('all-category-byte-file-admission-v16',{'actual':current,'limits':caps,'claims':admission.CLAIMS,
                                                   'refusals':admissions,'no_excess_written':True})
        rows.append(execute('normal-final-v16','import time; time.sleep(.5)'))
        rows.append(execute('nested-final-v16','import subprocess,sys; subprocess.run([sys.executable,"-B","-c","import time; time.sleep(.5)"],check=True)'))
        rows.append(execute('orphan-final-v16','import subprocess,sys; subprocess.Popen([sys.executable,"-B","-c","import time; time.sleep(4)"])'))
        rows.append(execute('memory-final-v16','import time; x=bytearray(96*1024**2); time.sleep(4)', ['--limit',str(64*1024**2)]))
        rows.append(execute('missing-final-v16','import time; time.sleep(4)',['--test-fault','missing-RSS']))
        rows.append(execute('cadence-final-v16','import time; time.sleep(4)',['--test-fault','sample-gap']))
        rows.append(execute('watchdog-death-final-v16','import time; time.sleep(4)',['--test-fault','watchdog-death']))
        rows.append(execute('output-final-v16','import os; os.write(1,b"x"*4096)',['--output-cap','512']))
        prospect=base_prospective();prospect['F08']={'bytes':1024,'files':1}
        code='from pathlib import Path; import time; p=Path("'+str(ROOT/'fixtures/live-byte-v16')+'"); p.parent.mkdir(exist_ok=True); p.write_bytes(b"x"*8192); time.sleep(1)'
        rows.append(execute('live-byte-final-v16',code,prospect=prospect))
        prospect=base_prospective();prospect['F08']={'bytes':4096,'files':1}
        code='from pathlib import Path; import time; p=Path("'+str(ROOT/'fixtures')+'"); p.mkdir(exist_ok=True); [(p/("live-file-v16-"+str(i))).write_bytes(b"x") for i in range(3)]; time.sleep(1)'
        rows.append(execute('live-file-final-v16',code,prospect=prospect))
        rows.append(execute('escape-final-v16','import subprocess,sys,time; subprocess.run([sys.executable,"-B","-c","import os,time; os.setsid(); time.sleep(4)"])'))
        rows.append(execute('source-pin-refusal-v16','raise RuntimeError("MUST_NOT_LAUNCH")',control=False))
        rows.append(execute('free-refusal-v16','raise RuntimeError("MUST_NOT_LAUNCH")',['--free-floor',str(10**15)]))
        # One same-inode token held by this explicit control driver. The second
        # guard and its keeper publish RSS while waiting and cannot launch work.
        with (ROOT/'mac-heavy.lock').open('a+b') as token:
            fcntl.flock(token.fileno(),fcntl.LOCK_EX|fcntl.LOCK_NB)
            rows.append(execute('fixed-token-wait-v16','raise RuntimeError("MUST_NOT_LAUNCH")',
                         ['--test-fixed-epoch',str(time.time()+.35)]))
        rows.append(execute('fixed-setup-wait-v16','raise RuntimeError("MUST_NOT_LAUNCH")',
                     ['--test-fixed-epoch',str(time.time()+.35),'--setup-delay','2']))
        rows.append(execute('local-timeout-final-v16','import time; time.sleep(20)',['--seconds','6']))
        hold=execute('closure-HOLD-final-v16','pass',['--test-fault','force-closure-HOLD'])
        rows.append(hold)
        assert hold['result']['ownership_HOLD'] and not hold['result']['token_released']
        recover_hold(hold['result'])
        # Abruptly kill only this explicitly created supervisor, after its own
        # command prints READY. Its independent keeper must retain and close the
        # token; the killed RSS writer is expected to be EOF-incomplete.
        name='supervisor-death-final-v16'
        cmd=[sys.executable,'-B',str(GUARD),'--name',name,'--control-only','--seconds','12',
             '--prospective',json.dumps(base_prospective()),'--',sys.executable,'-B','-c',
             'import time; print("READY",flush=True); time.sleep(10)']
        p=subprocess.Popen(cmd,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
        expected=identity(p.pid);path=ROOT/'evidence/control'/name;until=time.monotonic()+10
        while time.monotonic()<until:
            if (path/'worker.stdout').exists() and b'READY' in (path/'worker.stdout').read_bytes():break
            if p.poll() is not None:raise AssertionError('supervisor-died-before-READY')
            time.sleep(.01)
        else:raise AssertionError('supervisor-READY-timeout')
        owner=admission.strict_json((ROOT/'mac-heavy.lock').read_bytes());assert owner['parent']['pid']==p.pid
        child_query=monitor.children(p.pid);assert child_query['valid']
        keepers=[identity(pid) for pid in child_query['children'] if monitor.same_identity(owner['keeper'],identity(pid))]
        assert len(keepers)==1
        q=monitor.children(keepers[0]['pid']);assert q['valid']
        anchors=[identity(pid) for pid in q['children'] if identity(pid).get('pgid')==pid]
        assert len(anchors)==1
        anchor=anchors[0];assert monitor.same_identity(expected,identity(p.pid))
        signal_row=keeper.exact_signal(expected,signal.SIGKILL);assert signal_row['sent'];p.wait(timeout=2)
        proof=keeper.closure(anchor,{anchor['pid']:anchor},seconds=3)
        until=time.monotonic()+3
        while not (path/'keeper.json').exists() and time.monotonic()<until:time.sleep(.01)
        k=json.loads((path/'keeper.json').read_bytes());raw=raw_check(path/'samples.jsonl.gz')
        row={'explicit_owned_supervisor':expected,'signal':signal_row,'external_numeric_closure':proof,
             'keeper':k,'RSS_stream':raw,'qualification':'RSS writer death is censored if gzip EOF is incomplete; external closure is separate proof'}
        write(name,row);assert proof['proven'];p.stdout.close();p.stderr.close()
        print(json.dumps({'case':name,'closure':proof['proven'],'gzipEOF':raw['gzip_eof_complete']}),flush=True)
        if failure:raise RuntimeError('driver-RSS-publication:'+repr(failure))
        write('summary',{'status':'CONTROL_SMOKES_RECORDED_REVIEW_REQUIRED','guard_sha256':hashlib.sha256(GUARD.read_bytes()).hexdigest(),
                        'cases':[{'name':r['name'],'exit':r['exit'],'censor':r.get('result',{}).get('censor'),
                                  'closure':r.get('result',{}).get('closure'),'GO':r.get('result',{}).get('workload_GO')} for r in rows],
                        'driver_RSS_errors':failure,'peer_stale_refusals_retained':peer_refusals,'no_engine_build_corpus':True})
    finally:
        stop.set();thread.join(timeout=1);registry.update(remove=True)

if __name__=='__main__':run()
