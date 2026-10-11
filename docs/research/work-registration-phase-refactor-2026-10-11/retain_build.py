"""Retain a completed executable before reusing its Cargo dependency target."""
import hashlib,json,pathlib,shutil,sys
HERE=pathlib.Path(__file__).resolve().parent
name=sys.argv[1];record_path=HERE/f'build-{name}.json'
record=json.loads(record_path.read_text(encoding='utf-8'))
source=pathlib.Path(record['exe'])
assert record['exit']==0 and hashlib.sha256(source.read_bytes()).hexdigest()==record['exe_sha256']
destination=pathlib.Path('C:/Users/spmcc/AppData/Local/Temp/hcm-wr-phase-binaries-20261011')/f'{name}.exe'
destination.parent.mkdir(exist_ok=True);shutil.copyfile(source,destination)
assert hashlib.sha256(destination.read_bytes()).hexdigest()==record['exe_sha256']
record['retained_exe']=str(destination);record_path.write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'retained_exe':str(destination),'sha256':record['exe_sha256']}))
