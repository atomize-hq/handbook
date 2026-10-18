"""Executable native affected-regression capsule; no production/repository mutation."""
import argparse,hashlib,json,os,platform,subprocess,sys,time
from pathlib import Path
from controller import Driver
from checks import exercise,faults,result,req,op
from supplement import run as supplement
from matrix import run as matrix
from adversarial import run as adversarial
a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);a.add_argument('--pin',required=True);ns=a.parse_args()
base=Path(ns.base);base.mkdir(parents=True,exist_ok=False)
source=Path(__file__).parent
pins={str(p.relative_to(source)).replace('\\','/'):hashlib.sha256(p.read_bytes()).hexdigest() for p in source.rglob('*') if p.is_file() and '__pycache__' not in p.parts and 'evidence' not in p.parts and '.git' not in p.parts and 'target' not in p.parts}
environment={'pin':ns.pin,'platform':platform.platform(),'machine':platform.machine(),'python':sys.version,'source_hashes':pins,
    'exe_sha256':hashlib.sha256(Path(ns.exe).read_bytes()).hexdigest(),'argv':sys.argv}
for name,cmd in [('uname',['uname','-a']),('df',['df','-h',str(base)]),('rust',['rustc','--version']),('cargo',['cargo','--version'])]:
    p=subprocess.run(cmd,capture_output=True,timeout=20);(base/f'{name}.stdout').write_bytes(p.stdout);(base/f'{name}.stderr').write_bytes(p.stderr)
    environment[name]={'exit':p.returncode,'stdout_sha256':hashlib.sha256(p.stdout).hexdigest()}
(base/'environment.json').write_text(json.dumps(environment,indent=2))
d=Driver(ns.exe,base/'checks',7200);rows=[exercise(d,m,c) for m in ['A','B'] for c in ['project','research']];rows.extend(faults(d))
(base/'checks/summary.json').write_text(json.dumps(rows,indent=2))
d=Driver(ns.exe,base/'supplement',3600);(base/'supplement/summary.json').write_text(json.dumps(supplement(d),indent=2))
d=Driver(ns.exe,base/'adversarial',1800);(d.base/'summary.json').write_text(json.dumps(adversarial(d),indent=2))

d=Driver(ns.exe,base/'smoke',3600);matrix(d,[1000])
for consumer in ['project','research']:
    exe=Path(ns.exe).with_name(f'{consumer}-consumer')
    if os.name=='nt':exe=exe.with_suffix('.exe')
    d=Driver(exe,base/f'consumer-{consumer}',900);r=str(d.base/'store')
    d.call(dict(action='init',root=r,mode='A',consumer=consumer,count=16))
    d.call(dict(action='publish',root=r,request=req('consumer',[op('label',text='library-consumer')],packet=True)))
    d.call(dict(action='resolve',root=r,request_id='consumer'))
    d.call(dict(action='audit',root=r))
    command=['bash',str(source/'walkthrough.sh'),r,str(d.base/'copied')]
    p=subprocess.run(command,capture_output=True,timeout=180);(d.base/'walk.stdout').write_bytes(p.stdout);(d.base/'walk.stderr').write_bytes(p.stderr);assert p.returncode==0
# B selected-once ordinary copying; positive exact path is bound before copying.
d=Driver(ns.exe,base/'B-copy',900);r=str(d.base/'store')
d.call(dict(action='init',root=r,mode='B',consumer='research',count=16))
p=subprocess.run(['bash',str(source/'walkthrough.sh'),r,str(d.base/'copied')],capture_output=True,timeout=180)
(d.base/'walk.stdout').write_bytes(p.stdout);(d.base/'walk.stderr').write_bytes(p.stderr);assert p.returncode==0
(base/'completed.json').write_text(json.dumps({'pin':ns.pin,'affected_checks':True,'smoke_n':1000,'consumers':2,'shell_copies':3,'finished':time.time()},indent=2))
