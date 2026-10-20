"""B control obligations: frozen shell readers, writer advance, retirement/refusal."""
import argparse,hashlib,json,shutil,subprocess
from pathlib import Path
from driver import Driver
from capsule import asynchronous,wait,finished,exclusive_delete_handle
from independent import verify

parser=argparse.ArgumentParser();parser.add_argument('--base',required=True);args=parser.parse_args();base=Path(args.base);base.mkdir(parents=True,exist_ok=False)
here=Path(__file__).resolve().parent;exe=Path('C:/file-native-packing-20261020/bin/candidate2.exe');binding=json.loads(Path('C:/file-native-packing-20261020/fix2/build-binding.json').read_bytes());assert hashlib.sha256(exe.read_bytes()).hexdigest()==binding['exe_sha256']
for source in ['fix2_regression.py','selected_once_copy.ps1','capsule.py','driver.py','guard.py']:(base/source).write_bytes((here/source).read_bytes())
d=Driver(base/'events');shell=[];cases=[]
def ordinary(root,name,captured=None,save=None,code=0):
    command=['powershell','-NoProfile','-ExecutionPolicy','Bypass','-File',str(here/'selected_once_copy.ps1'),'-Root',str(root),'-Destination',str(base/(name+'-copy'))]
    if captured:command+=['-CapturedSelection',str(captured)]
    if save:command+=['-SaveSelection',str(save)]
    child=subprocess.run(command,capture_output=True,timeout=30);(base/(name+'.stdout')).write_bytes(child.stdout);(base/(name+'.stderr')).write_bytes(child.stderr);shell.append(dict(name=name,command=command,exit=child.returncode,stdout_sha256=hashlib.sha256(child.stdout).hexdigest(),stderr_sha256=hashlib.sha256(child.stderr).hexdigest()))
    assert (child.returncode==0)==(code==0),shell[-1]
    return json.loads(child.stdout) if child.returncode==0 else None

root=base/'B/disposable/store';d.call(root,'init',exe_override=exe,mode='B',consumer='project',count=16);d.call(root,'lifecycle',exe_override=exe,operation='setup')
captured=base/'captured-initial.json';original=ordinary(root,'original',save=captured);initial=original['selected'];assert json.loads(captured.read_bytes())==initial
pub0=root/f"publications/{initial['publication']}.json";pub0_raw=pub0.read_bytes();original_outcome=d.call(root,'resolve',exe_override=exe,request_id='initial')['result']
d.call(root,'lifecycle',exe_override=exe,operation='pin',name='shell-frozen',selected=initial)
d.call(root,'publish',exe_override=exe,request=dict(request='after-capture',base=d.head(root),ops=[dict(kind='label',id='d000004',text='new label after shell capture')],packet=False));middle=d.head(root);before_current=ordinary(root,'before-current')
first=d.call(root,'packing',exe_override=exe,operation='build',codec='raw',include_attempts=True,reserve_bytes=1048576,cleanup=True)['result'];assert first['retained_loose_control_records']==2
assert ordinary(root,'frozen-after-cleanup',captured=captured)['files']==original['files'];assert ordinary(root,'current-after-cleanup')['files']==before_current['files'];cases.append('historical-selector-captured-before-publish-and-cleanup-exact-copy')
repeat=d.call(root,'packing',exe_override=exe,operation='cleanup')['result'];assert repeat['successfully_deleted_bytes_recorded']==first['successfully_deleted_bytes_recorded'] and repeat['retained_loose_control_bytes']==first['retained_loose_control_bytes'];cases.append('retained-controls-have-no-deletion-credit')
d.call(root,'packing',exe_override=exe,operation='repack',codec='raw',include_attempts=True,reserve_bytes=1048576,cleanup=True)
assert ordinary(root,'frozen-after-repack',captured=captured)['files']==original['files'];cases.append('historical-generation-discovery-after-repack')
ready=base/'ready';release=base/'release';thread,state=asynchronous(d,root,'packing',exe_override=exe,operation='build',codec='raw',cleanup=True,reserve_bytes=1048576,env={'EXP_BARRIER':'pack-before-catalog','EXP_READY':str(ready),'EXP_RELEASE':str(release)})
wait(ready);d.call(root,'publish',exe_override=exe,request=dict(request='writer-during-control-build',base=d.head(root),ops=[],packet=False));advanced=d.head(root)
assert ordinary(root,'frozen-during-build',captured=captured)['files']==original['files'];release.write_bytes(b'go');finished(thread,state);assert d.head(root)==advanced
assert ordinary(root,'frozen-after-writer',captured=captured)['files']==original['files'];ordinary(root,'current-after-writer');cases.append('concurrent-semantic-writer-preserves-frozen-and-new-current-discovery')
hold=exclusive_delete_handle(pub0);held=d.call(root,'packing',exe_override=exe,operation='cleanup')['result'];assert held['status']=='complete' and pub0.read_bytes()==pub0_raw;hold();cases.append('B-control-no-delete-sharing-remains-retained')
assert d.call(root,'lifecycle',exe_override=exe,operation='retire',selected=initial,code=2)['error']=='refused:protected-snapshot'
d.call(root,'lifecycle',exe_override=exe,operation='release',name='shell-frozen');d.call(root,'lifecycle',exe_override=exe,operation='retire',selected=initial);d.call(root,'lifecycle',exe_override=exe,operation='collect')
assert d.call(root,'point',exe_override=exe,selected=initial,id='d000004',code=2)['error']=='retired:snapshot';assert d.call(root,'resolve',exe_override=exe,request_id='initial')['result']==original_outcome
ordinary(root,'retired-generation-refuses-copy',captured=captured,code=1);d.call(root,'packing',exe_override=exe,operation='repack',codec='raw',include_attempts=True,reserve_bytes=1048576,cleanup=True);assert pub0.read_bytes()==pub0_raw;cases.append('retirement-removes-content-but-keeps-permanent-B-publication-control')
for kind in ['missing','conflicting']:
    clone=base/(kind+'-control/disposable/store');shutil.copytree(root,clone);d.call(clone,'lifecycle',exe_override=exe,operation='setup');path=clone/f"publications/{middle['publication']}.json"
    if kind=='missing':path.rename(path.with_suffix('.missing'))
    else:path.write_bytes(path.read_bytes()+b' ')
    response=d.call(clone,'packing',exe_override=exe,operation='cleanup',code=2);assert response['error']==('integrity:B-loose-control-publication-missing' if kind=='missing' else 'integrity:cleanup-conflicting-duplicate');cases.append(kind+'-control-fails-closed')
a=base/'A/disposable/store';d.call(a,'init',exe_override=exe,mode='A',consumer='project',count=16);d.call(a,'lifecycle',exe_override=exe,operation='setup');out=d.call(a,'packing',exe_override=exe,operation='build',codec='raw',include_attempts=True,reserve_bytes=1048576,cleanup=True)['result'];assert out['retained_loose_control_records']==out['retained_loose_control_bytes']==0 and not list((a/'publications').glob('*.json'));cases.append('A-cleanup-retains-no-publication-control-duplicates')
verification=verify(root,base/'B-independent.json');d.call(a,'audit',exe_override=exe)
result=dict(status='passed',exe=str(exe),exe_sha256=binding['exe_sha256'],cases=cases,events=len(d.events),ordinary_shell=shell,B_initial_control_bytes=first['retained_loose_control_bytes'],B_initial_control_records=first['retained_loose_control_records'],retired_control_bytes_unchanged=pub0.read_bytes()==pub0_raw,independent=verification,core_files=binding['files']);(base/'summary.json').write_text(json.dumps(result,indent=2));print(json.dumps(dict(status='passed',cases=len(cases),events=len(d.events),ordinary_shell_processes=len(shell),exe_sha256=binding['exe_sha256']),indent=2))
