"""Install only a recorded WSL harness guard; preserve original capsule/Rust bytes."""
import hashlib,json,subprocess,time
from pathlib import Path
source=Path(__file__).resolve().parent;pin='7ac70f93cce14ecd3d935852801f034c122000d8';prefix='docs/research/file-native-incremental-engine-experiment-2026-10-18/'
names=['replay_lifetime.py','namespace_cases.py','deep_cases.py','walk_cases.py']
paths=subprocess.check_output(['git','ls-tree','-r','--name-only',pin,prefix]).decode().splitlines()
selected=[p for p in paths if p.endswith('.rs') or p[len(prefix):] in names]
expected={p[len(prefix):]:hashlib.sha256(subprocess.check_output(['git','show',pin+':'+p])).hexdigest() for p in selected}
remote='''import hashlib,json,shutil,time
from pathlib import Path
start=time.time();base=Path('/home/spenser/file-native-incremental-source-7ac70f9');expected=EXPECTED;names=NAMES
def sha(b):return hashlib.sha256(b).hexdigest()
for name,h in expected.items():assert sha((base/name).read_bytes())==h,name
assert not (base/'wsl_budget_adapter.py').exists()
for name in names:assert not (base/Path(name).with_suffix('.capsule.py')).exists()
adapter=Path(TRANSPORT).read_bytes();(base/'wsl_budget_adapter.py').write_bytes(adapter);rows=[]
for name in names:
 p=base/name;raw=p.read_bytes();copy=p.with_suffix('.capsule.py');copy.write_bytes(raw)
 wrapper="import sys\\nfrom pathlib import Path\\nfrom wsl_budget_adapter import main\\nif __name__=='__main__':sys.exit(main(Path(__file__).with_name("+repr(copy.name)+"),sys.argv[1:]))\\n"
 wrapper=wrapper.replace('\\\\n','\\n');p.write_text(wrapper)
 rows.append(dict(path=name,capsule_copy=copy.name,original_sha256=sha(raw),wrapper_sha256=sha(p.read_bytes())))
for name,h in expected.items():
 if name.endswith('.rs'):assert sha((base/name).read_bytes())==h,name
print(json.dumps(dict(core_pin=PIN,source=str(base),started_epoch=start,finished_epoch=time.time(),adapter_sha256=sha(adapter),original_helpers_and_all_rust_checked=True,unchanged_rust_files=sum(n.endswith('.rs') for n in expected),helpers=rows),indent=2))
'''.replace('EXPECTED',repr(expected)).replace('NAMES',repr(names)).replace('TRANSPORT',repr('/mnt/c/'+source.as_posix()[3:]+'/wsl_budget_adapter.py')).replace('PIN',repr(pin))
result=subprocess.run(['wsl','-d','Ubuntu-24.04','--','python3','-'],input=remote.encode(),stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=120)
(source/'evidence/wsl-budget-adapter-install.stderr').write_bytes(result.stderr)
assert result.returncode==0,result.stderr.decode(errors='replace')
report=json.loads(result.stdout);assert report['unchanged_rust_files']==21
(source/'evidence/wsl-budget-adapter-install.json').write_bytes(result.stdout);print(json.dumps(report))
