"""Final evidence checks inherit a conservative bound on the reporting reserve."""
import argparse,json,subprocess,sys,time
from pathlib import Path

a=argparse.ArgumentParser();a.add_argument('--scratch',required=True);ns=a.parse_args()
scratch=Path(ns.scratch);source=Path(__file__).resolve().parent;log=scratch/'final-proofs-v3';log.mkdir(exist_ok=False)
prior=scratch/'final-verification-v3';prerequisite=scratch/'serial-continuation-v2/completed.json';wait_end=time.monotonic()+6*3600
# On this pinned Windows Python 3.11 runtime st_ctime is file creation time.
# The reporting lane cannot start before its prerequisite flag exists. A first
# work-log absence would not establish that: the lane starts before opening it.
assert sys.platform=='win32' and sys.version_info[:2]==(3,11)
while not prerequisite.exists():
    if time.monotonic()>wait_end:raise TimeoutError('reporting lane did not start')
    time.sleep(15)
creation=prerequisite.stat().st_ctime;end=creation+7200-1
(log/'reserve-bound.json').write_text(json.dumps(dict(prerequisite=str(prerequisite),prerequisite_creation_epoch=creation,conservative_deadline_epoch=end,original_reserve_seconds=7200,additional_margin_seconds=1),indent=2))
while not (prior/'completed.json').exists():
    if time.time()>=end:raise TimeoutError('existing report reserve expired')
    time.sleep(15)
rows=[]

def command(name,args,cap):
    if time.time()+cap>end:
        row=dict(name=name,unperformed='full declared cap does not fit original reporting reserve',cap_seconds=cap)
    else:
        begin=time.time();censored=False
        with (log/(name+'.stdout')).open('wb') as out,(log/(name+'.stderr')).open('wb') as err:
            p=subprocess.Popen([str(x) for x in args],cwd=source,stdout=out,stderr=err)
            try:p.wait(timeout=cap)
            except subprocess.TimeoutExpired:
                censored=True;subprocess.run(['taskkill','/PID',str(p.pid),'/T','/F'],capture_output=True,timeout=30);p.wait(timeout=30)
        row=dict(name=name,argv=[str(x) for x in args],exit=p.returncode,censored=censored,started_epoch=begin,seconds=time.time()-begin)
    rows.append(row);(log/'steps.json').write_text(json.dumps(dict(deadline_epoch=end,rows=rows),indent=2));return row

python=sys.executable;exe=scratch/'build-audit/release/file-native-incremental.exe';history=scratch/'history-common-pin/store'
command('strict-A100k-audit',[python,'-c',"from controller import Driver; d=Driver(r'"+str(exe)+"',r'"+str(scratch/'strict-large-audit')+"',900); d.call(dict(action='audit',root=r'"+str(scratch/'matrix-common-pin/n100000/disposable/store')+"'),cap=900)"],900)
proof=source/'evidence/history-proof-final'
packed=command('history-proof',[python,source/'pack_history.py','--root',history,'--destination',proof],900)
restored=None
if packed.get('exit')==0:
    restored=command('restore-history-proof',[python,source/'pack_history.py','--verify',proof,'--restore',scratch/'history-proof-final-restored/disposable/store'],900)
if restored and restored.get('exit')==0:
    gc=command('history-gc-continuation',[python,source/'history_gc_continuation.py','--scratch',scratch,'--exe',exe,'--deadline-epoch',end],900)
    if (scratch/'history-common-pin/disposable/store/SELECTED.json').exists():history=scratch/'history-common-pin/disposable/store'
    if (scratch/'history-gc-continuation/relocation.json').exists():
        command('post-GC-exact-outcomes',[python,source/'verify_gc_outcomes.py','--scratch',scratch,'--exe',exe],180)
        command('independent-post-GC-history',[python,'-c',"from verify_store import verify_history; import json; print(json.dumps(verify_history(r'"+str(history)+"',allow_retired_content=True),indent=2))"],600)
        command('post-GC-history-census',[python,source/'census.py','--roots',history,'--owners','--output',source/'evidence/history-census-post-GC.json'],600)
matrix=scratch/'matrix-common-pin';roots=[matrix/f'{prefix}n{n}/disposable/store' for n in [1000,10000,100000] for prefix in ['','B-']]
roots += [scratch/f'document-calibration-final/n{n}/disposable/store' for n in [1000,10000]]+[history]
roots=[root for root in roots if (root/'SELECTED.json').exists()]
command('current-root-geometry',[python,source/'root_geometry.py','--roots',*roots,'--output',source/'evidence/current-root-geometry-final.json'],600)
(log/'completed.json').write_text(json.dumps(dict(deadline_epoch=end,rows=rows),indent=2))
