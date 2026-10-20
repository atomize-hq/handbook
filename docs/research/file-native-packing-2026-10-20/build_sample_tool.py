import hashlib,json,subprocess
from pathlib import Path
HERE=Path(__file__).resolve().parent;cwd=HERE/'sample-tool'
for argv in [['cargo','generate-lockfile','--offline'],['cargo','build','--locked','--offline','--release','-j2','--target-dir','C:/file-native-packing-20261020/build']]:subprocess.run(argv,cwd=cwd,check=True,timeout=120)
exe=Path('C:/file-native-packing-20261020/build/release/packing-sample.exe')
core=HERE.parent/'file-native-incremental-engine-experiment-2026-10-18/src/pack.rs'
result=dict(exe=str(exe),exe_sha256=hashlib.sha256(exe.read_bytes()).hexdigest(),frozen_pack_rs_sha256=hashlib.sha256(core.read_bytes()).hexdigest(),tool_sources={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in [cwd/'Cargo.toml',cwd/'Cargo.lock',cwd/'main.rs']})
Path('C:/file-native-packing-20261020/receipts/sample-tool-binding.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
