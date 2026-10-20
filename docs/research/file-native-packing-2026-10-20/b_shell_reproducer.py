"""Fresh B shell discovery immediately after packing+cleanup, before publishing."""
import argparse,json,subprocess,hashlib
from pathlib import Path
from driver import Driver

parser=argparse.ArgumentParser();parser.add_argument('--name',required=True);parser.add_argument('--exe',required=True);parser.add_argument('--expect',choices=['regression','pass'],required=True);args=parser.parse_args()
runtime=Path('C:/file-native-packing-20261020');base=runtime/'B-shell'/args.name;base.mkdir(parents=True,exist_ok=False);root=base/'disposable/store';exe=Path(args.exe);d=Driver(base/'events')
(base/'harness-source.py').write_bytes(Path(__file__).read_bytes());(base/'selected_once_copy.ps1').write_bytes(Path(__file__).with_name('selected_once_copy.ps1').read_bytes())
d.call(root,'init',exe_override=exe,mode='B',count=16,consumer='project');d.call(root,'lifecycle',exe_override=exe,operation='setup');selected=(root/'SELECTED.json').read_bytes()
results={}
for phase in ['before','after']:
    if phase=='after':d.call(root,'packing',exe_override=exe,operation='build',codec='raw',include_attempts=True,reserve_bytes=1048576,cleanup=True)
    command=['powershell','-NoProfile','-ExecutionPolicy','Bypass','-File',str(Path(__file__).with_name('selected_once_copy.ps1')),'-Root',str(root),'-Destination',str(base/(phase+'-copy'))]
    child=subprocess.run(command,capture_output=True,timeout=30);(base/(phase+'.stdout')).write_bytes(child.stdout);(base/(phase+'.stderr')).write_bytes(child.stderr)
    results[phase]=dict(exit=child.returncode,command=command,stdout_sha256=hashlib.sha256(child.stdout).hexdigest(),stderr_sha256=hashlib.sha256(child.stderr).hexdigest())
    if phase=='before' or args.expect=='pass':
        assert child.returncode==0,results[phase];body=json.loads(child.stdout);assert body['selected']==json.loads(selected) and len(body['files'])>=16;results[phase]['body']=body
    assert (root/'SELECTED.json').read_bytes()==selected
if args.expect=='regression':assert results['after']['exit']!=0 and not (root/f"publications/{json.loads(selected)['publication']}.json").exists()
else:assert results['before']['body']['files']==results['after']['body']['files']
report=dict(status='confirmed-B-shell-regression' if args.expect=='regression' else 'passed-immediate-B-shell-exact-copy',exe=str(exe),exe_sha256=hashlib.sha256(exe.read_bytes()).hexdigest(),selected_unchanged=True,no_post_packing_publication=True,results=results,fixture=str(root));(base/'summary.json').write_text(json.dumps(report,indent=2));print(json.dumps({k:v for k,v in report.items() if k!='results'},indent=2))
