"""Finite serial continuation; invoke after the running matrix process exits."""
import argparse,json,os,shutil,subprocess,sys,time
from pathlib import Path

a=argparse.ArgumentParser();a.add_argument('--scratch',required=True);ns=a.parse_args()
source=Path(__file__).resolve().parent;scratch=Path(ns.scratch).resolve();log=scratch/'serial-continuation';log.mkdir(exist_ok=False);rows=[]
def command(name,args,cap,required=True):
    started=time.monotonic();censored=False
    with (log/(name+'.stdout')).open('wb') as out,(log/(name+'.stderr')).open('wb') as err:
        p=subprocess.Popen(args,cwd=source,stdout=out,stderr=err)
        try:p.wait(timeout=max(1,cap))
        except subprocess.TimeoutExpired:
            censored=True;subprocess.run(['taskkill','/PID',str(p.pid),'/T','/F'],capture_output=True,timeout=30);p.wait(timeout=30)
    row={'name':name,'argv':[str(x) for x in args],'exit':p.returncode,'censored':censored,'seconds':time.monotonic()-started};rows.append(row)
    (log/'steps.json').write_text(json.dumps(rows,indent=2))
    if required and (p.returncode or censored):raise RuntimeError(row)
    return row
python=sys.executable;target=scratch/'build-current';exe=target/'release/file-native-incremental.exe'
correctness_end=time.monotonic()+7200
command('build',['cargo','build','--release','--locked','--workspace','--target-dir',str(target)],600)
command('tests',['cargo','test','--locked','--workspace','--target-dir',str(target)],600)
for script,name,extra in [('namespace_cases.py','namespace',[]),('checks.py','checks',['--faults']),('supplement.py','supplement',[]),('adversarial.py','adversarial',[]),('deep_cases.py','deep-cases',[]),('walk_cases.py','walk-cases',['--source',str(source),'--shell','powershell']),('run_differential.py','differential',[])]:
    command(name,[python,str(source/script),'--exe',str(exe),'--base',str(scratch/('final-'+name)),*extra],min(7200,max(1,correctness_end-time.monotonic())))
matrix=scratch/'matrix-common-pin';matrix_end=json.loads((matrix/'session.json').read_bytes())['started']+14400
command('post-matrix',[python,str(source/'finish_matrix.py'),'--exe',str(exe),'--base',str(scratch/'post-matrix'),'--matrix',str(matrix)],max(1,matrix_end-time.time()),False)
command('codec',[python,str(source/'controller.py'),'--exe',str(scratch.parent/'file-native-incremental-20261018-build-control/release/file-native-publication-experiment.exe'),'--base',str(scratch/'codec-common-pin'),'--stage','codec'],max(1,matrix_end-time.time()),False)
command('history',[python,str(source/'history_run.py'),'--exe',str(exe),'--base',str(scratch/'history-common-pin'),'--max-depth','100000'],14400,False)
# No Windows benchmark child is running while the guest lane uses the same disk.
def logical(root):
    total=0
    for directory,_,names in os.walk(root):
        for name in names:
            try:total+=(Path(directory)/name).stat().st_size
            except FileNotFoundError:pass
    return total
owned=[scratch,*[Path(str(scratch)+suffix) for suffix in ['-build','-build-control','-build-final']]]
bytes_now=sum(logical(root) for root in owned)
if bytes_now+6*1024**3>32*1024**3 or shutil.disk_usage(scratch).free<140*1024**3:
    rows.append({'name':'wsl','unperformed':'combined allocation/free reserve','local_logical_bytes':bytes_now});(log/'steps.json').write_text(json.dumps(rows,indent=2))
else:
    guest='/home/spenser/file-native-incremental-source-a1108c6';lane='/home/spenser/file-native-incremental-wsl-final-20261018';binary=lane+'/build/release/file-native-incremental'
    prefix=['wsl','-d','Ubuntu-24.04','--']
    command('wsl-native',prefix+['python3',guest+'/run_lane.py','--root',lane,'--source',guest,'--pin','a1108c634cb8362aca50e1a6228766802b96f087','--reserve-gib','40','--cap-gib','6'],7200,False)
    for script,name,extra in [('namespace_cases.py','namespace',[]),('deep_cases.py','deep-cases',[]),('walk_cases.py','walk-cases',['--source',guest,'--shell','bash'])]:
        command('wsl-'+name,prefix+['python3',guest+'/'+script,'--exe',binary,'--base',lane+'/'+name,*extra],900,False)
(log/'completed.json').write_text(json.dumps({'engine_pin':'a1108c634cb8362aca50e1a6228766802b96f087','rows':rows},indent=2))
