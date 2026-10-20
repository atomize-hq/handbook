"""Bind exact external helper bytes before a separately admitted restore check."""
import argparse,hashlib,json,runpy,sys
from pathlib import Path

HERE=Path(__file__).resolve().parent;R=Path('C:/file-native-packing-20261020')
parser=argparse.ArgumentParser();parser.add_argument('stage',choices=['verify','reopen','successor','verify-successor']);stage=parser.parse_args().stage
out=R/'packed-restored-qualification';out.mkdir(exist_ok=True);files={}
for name in ['qualified_restore_stage.py','packed_restore_check.py','packed_backup_stage.py','postcleanup_check.py','independent.py','driver.py','guard.py']:
    raw=(HERE/name).read_bytes();copy=out/(stage+'-'+name);assert not copy.exists();copy.write_bytes(raw);files[name]=dict(bytes=len(raw),sha256=hashlib.sha256(raw).hexdigest(),snapshot=str(copy))
binding=dict(stage=stage,core='d70f3864f2888d46d9112213b4a03f8d82745a8e',external_helpers=files,cap=2684354560)
(out/(stage+'-binding.json')).write_text(json.dumps(binding,indent=2));sys.argv=[str(HERE/'packed_restore_check.py'),stage]
runpy.run_path(str(HERE/'packed_restore_check.py'),run_name='__main__')
for name,row in files.items():assert hashlib.sha256((HERE/name).read_bytes()).hexdigest()==row['sha256']
