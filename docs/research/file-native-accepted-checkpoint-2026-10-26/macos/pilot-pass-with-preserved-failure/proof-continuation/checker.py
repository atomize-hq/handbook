import sys
sys.dont_write_bytecode=True
import os,json,hashlib
from pathlib import Path
OUT=Path(__file__).resolve().parent
SDK=Path('/private/tmp/file-native-online-macos-20261020/source/pilot-sdk-20261021/sdk')
os.environ['FILE_NATIVE_ORACLE_RESEARCH']=str(SDK.parent/'research')
os.environ['DYLD_LIBRARY_PATH']='/opt/homebrew/lib'
sys.path.insert(0,str(SDK))
import physical_decoder_v6 as v6
v6.FINAL_SOURCE_PIN='0628322e4b48e0efad19cdc32238a622d4aaa7a8'
inputs=json.loads((OUT/'checker/known-requests.json').read_text())
proof=v6.verify(Path('/private/tmp/file-native-macos-inventory-20261024/pilot-A64-H3-20261025/store/disposable/store'),OUT/'checker/complete-oracle.json',quiescent=True,known_requests=inputs,quota_evidence={})
assert proof['semantic']['publications']==3 and proof['navigation']['documents']==64
assert proof['navigation']['readable_declared_current'] and proof['navigation']['discovery_relationships_and_extraction']
print(json.dumps({'status':'passed','actual_H':proof['semantic']['publications'],'N':proof['navigation']['documents'],'receipt_sha256':hashlib.sha256((OUT/'checker/complete-oracle.json').read_bytes()).hexdigest()}))
