"""Bind the B control fix build separately from immutable candidate1/4cb proof."""
import hashlib,json,shutil,subprocess
from pathlib import Path

here=Path(__file__).resolve().parent;work=here.parents[2];runtime=Path('C:/file-native-packing-20261020');out=runtime/'fix2';out.mkdir(exist_ok=False)
old=json.loads((here/'candidate/source-binding.json').read_bytes());before={};changed=[]
for name,binding in old['files'].items():
    raw=(work/name).read_bytes();digest=hashlib.sha256(raw).hexdigest();before[name]=dict(bytes=len(raw),sha256=digest)
    if digest!=binding['sha256']:changed.append(name)
assert changed==['docs/research/file-native-incremental-engine-experiment-2026-10-18/src/packing.rs']
(out/'source-before.json').write_text(json.dumps(dict(core_base='4cb6b9fba9ffabc294a6006ec54fe75366968db4',changed=changed,files=before),indent=2))
subprocess.run(['cargo','build','--locked','--offline','--workspace','--release','-j2','--target-dir',str(runtime/'build')],cwd=here.parent/'file-native-incremental-engine-experiment-2026-10-18',timeout=120,check=True)
for name,binding in before.items():assert hashlib.sha256((work/name).read_bytes()).hexdigest()==binding['sha256']
source=runtime/'build/release/file-native-incremental.exe';destination=runtime/'bin/candidate2.exe';assert not destination.exists();shutil.copyfile(source,destination)
digest=hashlib.sha256(destination.read_bytes()).hexdigest();assert digest==hashlib.sha256(source.read_bytes()).hexdigest()
result=dict(core_base='4cb6b9fba9ffabc294a6006ec54fe75366968db4',source_commit='not-yet-committed-exact-file-map-controls',files=before,source_unchanged_by_build=True,exe=str(destination),exe_sha256=digest,old_candidate1_untouched=hashlib.sha256((runtime/'bin/candidate1.exe').read_bytes()).hexdigest()=='478dd9ba9f687d4de81c60ce55f9ddf9e4e2fafcc493d581b3c63e12e42f7fe5',tests_not_claimed=True)
(out/'build-binding.json').write_text(json.dumps(result,indent=2));print(json.dumps({k:v for k,v in result.items() if k!='files'},indent=2))
