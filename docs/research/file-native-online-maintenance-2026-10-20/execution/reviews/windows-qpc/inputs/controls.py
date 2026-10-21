"""Parent-authorized benign native controls; injected faults never qualify normal cadence."""
import ast, ctypes, gzip, hashlib, importlib.util, json, msvcrt, os, sys, threading, time
from pathlib import Path
BASE=Path('C:/file-native-online-matrix-20261020')
PRO=BASE/'protocol/windows-cadence'
OUT=BASE/'receipts/windows-cadence'
SRC=Path('C:/Users/spmcc/.codex/worktrees/file-native-online-matrix-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/windows-cadence/job_guard_qpc.py')
BUDGET=BASE/'receipts/budget-final-source-v3.json'
EXPECTED='50ecc7c40c1cdda6493817261b5755fe2d823fdae271118698828c39a0400378'
sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest()
assert sha(SRC)==EXPECTED
assert sha(BUDGET)=='0a1a73bd7f8094f1bf2e7f79cf40b6eff970acc2bb7c1640fbad52f3846be55a'
spec=importlib.util.spec_from_file_location('cadence_guard',SRC)
G=importlib.util.module_from_spec(spec);spec.loader.exec_module(G)
budget=G.Budget(BUDGET)
begin=time.perf_counter();deadline=begin+300
assert not OUT.exists();OUT.mkdir()
rows=[];injections=[]
lines=SRC.read_text().splitlines()
line=lambda text:next(i+1 for i,s in enumerate(lines) if s.strip()==text)
loc=dict(tick=line('tick = time.perf_counter_ns()'),collect=line('pids = current_members()'),
         missing=line('if value is None:'),tail=line('if current_members():'),release=line('if timer:'))
# The first "if value is None" belongs to supervisor_rss; select memory_watch's one.
watch=next(n for n in ast.walk(ast.parse(SRC.read_bytes())) if isinstance(n,ast.FunctionDef) and n.name=='memory_watch')
loc['missing']=next(i+1 for i,s in enumerate(lines) if watch.lineno<=i+1<=watch.end_lineno and s.strip()=='if value is None:')
identity='import sys,os,json,time;sys.path.insert(0,'+repr(str(SRC.parent))+');import job_guard_qpc as g;print(json.dumps(dict(pid=os.getpid(),creation_time=g.creation_time(os.getpid()))),flush=True);'
child=identity+'time.sleep(20)'
stress=identity+'import subprocess;subprocess.Popen([sys.executable,"-B","-c",'+repr(child)+']);f=open('+repr(str(PRO/'stress.bin'))+',"xb");until=time.perf_counter()+5\nwhile time.perf_counter()<until:\n sum(i*i for i in range(1000));f.seek(0);f.write(b"x"*4096);f.flush();os.fsync(f.fileno())\nf.close();print("normal-nested-cpu-io-complete",flush=True)'
tail=identity+'import subprocess;subprocess.Popen([sys.executable,"-B","-c",'+repr(child)+']);time.sleep(.2)'
cases=[('normal-nested-cpu-io',None,stress,30,None),('absolute-timeout',None,child,21,'time-censor'),
 ('first-slow','first',child,30,'memory-sample-cadence-censor'),
 ('start-gap','start',child,30,'memory-sample-cadence-censor'),
 ('missing-owned-rss','missing',child,30,'memory-monitor-error'),
 ('whole-job-tail-gap','tail',tail,30,'memory-sample-cadence-censor')]

def reserve():
    x=G.census(OUT);p=G.census(PRO)
    assert x['bytes']<=1048576 and x['files']<=38
    assert p['bytes']<=1048576 and p['files']<=100
    return {k:dict(bytes=1048576-x['bytes'],files=40-x['files']) for k in ['receipts','windows_cadence_receipts']}

def token_proof(owner):
    current=json.loads(G.OWNER.read_bytes())
    h=G.TOKEN.open('r+b')
    try:
        h.seek(0)
        try:msvcrt.locking(h.fileno(),msvcrt.LK_NBLCK,1)
        except OSError:
            return dict(free=False,other_valid_owner=current.get('nonce')!=owner['nonce'] and current.get('creation_time')==G.creation_time(current['pid']),observed_owner=current)
        h.seek(0);msvcrt.locking(h.fileno(),msvcrt.LK_UNLCK,1)
        return dict(free=True,observed_owner=current)
    finally:h.close()

try:
    # Source cap refuses before creating a Job or child; it is not live RSS proof.
    try:budget.check({'source':dict(bytes=1048577,files=0)})
    except ValueError as error:
        assert str(error).startswith('prospective-category-cap:source:')
        rows.append(dict(case='prospective-cap-negative',passed=True,child_started=False,error=str(error)))
    else:raise AssertionError('prospective cap accepted')
    for name,fault,command,seconds,expected in cases:
        if deadline-time.perf_counter()<55:
            rows.append(dict(case=name,passed=False,status='NOT_STARTED_TOTAL_300S_BOUND',remaining_seconds=deadline-time.perf_counter()));continue
        stage_begin=time.perf_counter();block=threading.Event();release=threading.Event();hits=[]
        def trace(frame,event,arg):
            if frame.f_code.co_filename!=str(SRC) or frame.f_code.co_name not in ['run','memory_watch']:return None
            if event!='line':return trace
            n=frame.f_lineno;fn=frame.f_code.co_name
            if fault=='tail' and fn=='memory_watch' and n==loc['tick'] and block.is_set():release.wait(2)
            if not hits:
                if fault=='first' and fn=='memory_watch' and n==loc['collect']:
                    t=time.perf_counter_ns();time.sleep(.06);hits.append(dict(kind='real-sleep-first-full',before_ns=t,after_ns=time.perf_counter_ns()))
                elif fault=='start' and fn=='memory_watch' and n==loc['tick'] and frame.f_locals.get('last_tick') is not None:
                    t=time.perf_counter_ns();time.sleep(.06);hits.append(dict(kind='real-sleep-before-next-start',before_ns=t,after_ns=time.perf_counter_ns()))
                elif fault=='missing' and fn=='memory_watch' and n==loc['missing']:
                    value=frame.f_locals['value'];frame.f_locals['value']=None
                    hits.append(dict(kind='simulated-local-unreadable-RSS',native_value_before=value,pid=frame.f_locals['pid']))
                elif fault=='tail' and fn=='run' and n==loc['tail'] and 'coverage_end_ns' in frame.f_locals:
                    block.set();t=time.perf_counter_ns();time.sleep(.065)
                    hits.append(dict(kind='sampler-block-plus-real-descendant-tail',before_ns=t,after_ns=time.perf_counter_ns()))
            if fault=='tail' and fn=='run' and n==loc['release']:release.set()
            return trace
        try:
            if fault:threading.settrace(trace);sys.settrace(trace)
            row=G.run([sys.executable,'-B','-c',command],OUT/(name+'.json'),BUDGET,seconds,reserves=reserve())
        except BaseException as error:
            rows.append(dict(case=name,passed=False,exception=repr(error),injections=hits));continue
        finally:
            release.set();sys.settrace(None);threading.settrace(None)
        raw=[]
        with gzip.open(OUT/(name+'.rss.jsonl.gz'),'rb') as stream:
            total=0
            for data in stream:
                total+=len(data);assert total<=8388608 and data.endswith(b'\n');raw.append(json.loads(data))
            assert stream.read()==b''
        owner=row['owner'];token=token_proof(owner)
        root_start=owner.get('job_root_creation_time');root_now=G.creation_time(owner['job_root_pid'])
        closure=len(row['closure_checks'])==2 and all(x['members']==[] for x in row['closure_checks']) and row['remaining_job_pids']==[] and owner['state']=='closed' and root_now!=root_start
        full=[x for x in raw if x.get('event')=='rss-sample']
        actual=row['censor'];passed=actual==expected and closure and (token.get('free') or token.get('other_valid_owner')) and row['guard_sha256']==EXPECTED
        if fault:passed=passed and bool(hits)
        if fault=='first':passed=passed and any(x.get('event')=='full-sample-cadence-late' and x['duration_seconds']>.04 for x in raw) and 'resumed_epoch' not in owner
        if fault=='start':passed=passed and any(x.get('event')=='cadence-gap-before-rss' and x['interval_seconds']>.04 for x in raw)
        if fault=='missing':passed=passed and any('unreadable-owned-pid-rss:' in e for e in row['errors']) and 'resumed_epoch' not in owner
        if fault=='tail':passed=passed and any(x.get('event')=='job-drain-sampling-coverage' and x['empty_observed'] and x['interval_seconds']>.04 for x in raw)
        if name=='absolute-timeout':passed=passed and any(x['reason']=='time-censor' for x in row['terminations']) and owner.get('resumed_epoch') is not None
        if expected is None:passed=passed and row['exit']==0 and bool(full) and row['memory_sample_timing']['slow_samples']==0 and row['memory_sample_timing']['intervals_over_40ms']==0
        identities=[]
        for text in (OUT/(name+'.stdout')).read_text(errors='replace').splitlines():
            try:
                value=json.loads(text)
                if set(value)=={'pid','creation_time'}:identities.append(dict(**value,ended_or_reused=G.creation_time(value['pid'])!=value['creation_time']))
            except json.JSONDecodeError:pass
        if name=='normal-nested-cpu-io':passed=passed and len(identities)>=2 and all(x['ended_or_reused'] for x in identities)
        rows.append(dict(case=name,passed=bool(passed),injected=bool(fault),injections=hits,expected_censor=expected,actual_censor=actual,receipt=str(OUT/(name+'.json')),elapsed_seconds=time.perf_counter()-stage_begin,native_closure_proven=closure,guard_process_closure_verified=row['process_closure_verified'],token_proof=token,owner=owner,stdout_identities=identities,raw_rows=len(raw),full_samples=len(full),raw_gzip_full_EOF=True,timing=row['memory_sample_timing'],raw_events=[x for x in raw if x.get('event')!='rss-sample']))
        print(json.dumps(dict(case=name,passed=bool(passed),censor=actual,samples=len(full),seconds=time.perf_counter()-stage_begin)),flush=True)
finally:
    release.set() if 'release' in globals() else None
    sys.settrace(None);threading.settrace(None)
    assert sha(SRC)==EXPECTED
    files={}
    for root in [PRO,OUT]:
        for p in root.rglob('*'):
            if p.is_file():files[str(p)]=dict(bytes=p.stat().st_size,sha256=sha(p))
    before_final=budget.check({k:dict(bytes=131072,files=1) for k in ['receipts','windows_cadence_receipts']})
    summary=dict(status='CONTROLS_PASSED' if len(rows)==7 and all(x['passed'] for x in rows) else 'CONTROLS_FAILED_OR_INCOMPLETE',cases=rows,elapsed_seconds=time.perf_counter()-begin,total_control_limit_seconds=300,source_sha256=EXPECTED,control_input_sha256=sha(__file__),budget_sha256=sha(BUDGET),frozen_reference_actual=sha(SRC.parent.parent/'harness/job_guard.py'),frozen_reference_expected='1e4a965139cac5b48e6cbef57d0ac8cc3c3d0294f36a392383308abcee3affae',files=files,manifest_self_excluded=True,postfinal_census_including_summary_reserve=before_final,qualification='benign Python only; real QPC/OS Job; faults are labeled local trace injection; no engine qualification; all failure prefixes retained')
    encoded=(json.dumps(summary,indent=2)+'\n').encode();assert len(encoded)<=131072
    assert G.census(OUT)['bytes']+len(encoded)<=1048576 and G.census(OUT)['files']+1<=40
    with (OUT/'summary.json').open('xb') as out:out.write(encoded)
    print(json.dumps(dict(status=summary['status'],elapsed=summary['elapsed_seconds'],summary_sha256=sha(OUT/'summary.json'),own_receipts=G.census(OUT),own_protocol=G.census(PRO))),flush=True)
