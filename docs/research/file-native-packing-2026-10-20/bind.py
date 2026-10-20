import hashlib,json,shutil,subprocess,sys
from pathlib import Path
HERE=Path(__file__).resolve().parent;WORK=HERE.parents[2];CORE=HERE.parent/'file-native-incremental-engine-experiment-2026-10-18'
name=sys.argv[1];out=Path('C:/file-native-packing-20261020/receipts')/(name+'-binding.json');assert not out.exists()
exe=Path('C:/file-native-packing-20261020/build/release/file-native-incremental.exe');copy=Path('C:/file-native-packing-20261020/bin')/(name+'.exe');copy.parent.mkdir(exist_ok=True);shutil.copyfile(exe,copy)
files=list((CORE/'src').glob('*.rs'))+[CORE/'Cargo.toml',CORE/'Cargo.lock']
binding=dict(source_base_pin='afbd768a19032b34c53251f08198ea78a23a978d',files={p.relative_to(WORK).as_posix():dict(bytes=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in files},exe=str(copy),exe_sha256=hashlib.sha256(copy.read_bytes()).hexdigest())
out.write_text(json.dumps(binding,indent=2));print(json.dumps(dict(binding=str(out),files=len(files),exe_sha256=binding['exe_sha256'])))
