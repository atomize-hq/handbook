"""Serial bounded corrections/calibration, after history and the WSL main lane."""
import argparse,json,os,shutil,subprocess,sys,time
from pathlib import Path
a=argparse.ArgumentParser();a.add_argument('--scratch',required=True);ns=a.parse_args()
scratch=Path(ns.scratch);source=Path(__file__).resolve().parent;log=scratch/'final-verification-v3';log.mkdir(exist_ok=False);rows=[]
# Wait for the already running finite lane; never extend its deadline.
wait_end=time.monotonic()+6*3600
while not (scratch/'serial-continuation-v2/completed.json').exists():
    if time.monotonic()>wait_end:raise TimeoutError('prior finite lane did not complete')
    time.sleep(15)
start=time.monotonic();end=start+7200;deadline_epoch=time.time()+7200
def command(name,args,cap):
    begin=time.monotonic();censored=False
    if begin>=end:
        row=dict(name=name,argv=[str(x) for x in args],unperformed='reporting deadline expired');rows.append(row)
        (log/'steps.json').write_text(json.dumps(dict(cap_seconds=7200,rows=rows),indent=2));return row
    with (log/(name+'.stdout')).open('wb') as out,(log/(name+'.stderr')).open('wb') as err:
        p=subprocess.Popen([str(x) for x in args],cwd=source,stdout=out,stderr=err)
        try:p.wait(timeout=max(1,min(cap,end-time.monotonic())))
        except subprocess.TimeoutExpired:
            censored=True;subprocess.run(['taskkill','/PID',str(p.pid),'/T','/F'],capture_output=True,timeout=30);p.wait(timeout=30)
    row=dict(name=name,argv=[str(x) for x in args],exit=p.returncode,censored=censored,seconds=time.monotonic()-begin);rows.append(row)
    (log/'steps.json').write_text(json.dumps(dict(core_pin='7e00d22b470c1eed476127d2f585789036e79cf3',cap_seconds=7200,rows=rows),indent=2));return row
python=sys.executable;exe=scratch/'build-audit/release/file-native-incremental.exe';matrix=scratch/'matrix-common-pin'
command('document-calibration',[python,source/'calibrate_documents.py','--exe',exe,'--base',scratch/'document-calibration-final'],900)
command('contention',[python,source/'contention_final.py','--exe',exe,'--base',scratch/'contention-final','--root',matrix/'n10000/disposable/store'],900)
command('quota-bytes',[python,source/'quota_bytes.py','--exe',exe,'--base',scratch/'quota-bytes-final'],900)
command('strict-history-audit',[python,'-c',"from controller import Driver; d=Driver(r'"+str(exe)+"',r'"+str(scratch/'strict-history-audit')+"',900); d.call(dict(action='audit',root=r'"+str(scratch/'history-common-pin/store')+"'),cap=900)"],900)
command('genuine-sources',[python,source/'verify_genuine.py','--matrix',matrix,'--output',source/'evidence/genuine-source-final.json'],600)
command('genuine-calibration',[python,source/'verify_genuine.py','--matrix',scratch/'document-calibration-final','--tiers','1000','10000','--output',source/'evidence/genuine-calibration-final.json'],600)
command('codec-geometry',[python,source/'codec_geometry.py','--base',scratch/'codec-common-pin','--output',source/'evidence/codec-geometry-final.json'],600)
roots=[matrix/f'{prefix}n{n}/disposable/store' for n in [1000,10000,100000] for prefix in ['','B-']]
roots += [scratch/f'document-calibration-final/n{n}/disposable/store' for n in [1000,10000]]+[scratch/'history-common-pin/store']
roots=[root for root in roots if (root/'SELECTED.json').exists()]
command('independent-projection',[python,source/'verify_projection.py','--roots',*roots,'--output',source/'evidence/independent-projection-final.json'],900)
command('selected-store-census',[python,source/'census.py','--roots',*roots,'--owners','--output',source/'evidence/selected-store-census-final.json'],900)
command('unselected-B-census',[python,source/'census.py','--roots',scratch/'post-matrix/B-n100000/disposable/store','--output',source/'evidence/unselected-B-census-final.json'],300)
command('measured-tables',[python,source/'summarize.py','--root',scratch,'--output',source/'evidence/measured-tables-final.json'],600)
# New strict-audit capsule builds on guest ext4; /mnt/c is transport only.
owned=[scratch,*[Path(str(scratch)+suffix) for suffix in ['-build','-build-control','-build-final']]]
local_bytes=sum((Path(d)/n).stat().st_size for root in owned for d,_,names in os.walk(root) for n in names if (Path(d)/n).exists())
if time.monotonic()+1920<=end and local_bytes+6*1024**3<=32*1024**3 and shutil.disk_usage(scratch).free>=140*1024**3:
    prefix=['wsl','-d','Ubuntu-24.04','--'];guest='/home/spenser/file-native-incremental-source-7e00d22'
    code="import pathlib,tarfile,shutil; p=pathlib.Path('"+guest+"'); p.mkdir(exist_ok=False); tarfile.open('/mnt/c/file-native-incremental-20261018/capsule-7e00d22.tar').extractall(p); shutil.copyfile('/mnt/c/file-native-incremental-20261018/audit_lane.py','/home/spenser/audit_lane.py')"
    transport=command('wsl-audit-source',prefix+['python3','-c',code],120)
    if transport.get('exit')==0 and time.monotonic()+1800<=end:command('wsl-audit',prefix+['python3','/home/spenser/audit_lane.py','--root','/home/spenser/file-native-incremental-wsl-audit-final','--source',guest,'--pin','7e00d22b470c1eed476127d2f585789036e79cf3','--reserve-gib','40','--deadline-epoch',str(deadline_epoch)],1800)
    else:rows.append(dict(name='wsl-audit',unperformed='transport failed or complete guest cap does not fit remaining deadline'))
else:rows.append(dict(name='wsl-audit',unperformed='full guest deadline does not fit, or combined allocation/free reserve',local_logical_bytes=local_bytes))
(log/'completed.json').write_text(json.dumps(dict(elapsed_seconds=time.monotonic()-start,rows=rows),indent=2))
