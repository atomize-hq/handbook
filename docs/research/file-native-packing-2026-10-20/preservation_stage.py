"""Parent-admitted exact post-GC preservation. One fresh destination only."""
import hashlib,importlib.util,json,sys,time
from pathlib import Path
HERE=Path(__file__).resolve().parent;RUNTIME=Path('C:/file-native-packing-20261020')
SOURCE=Path('C:/file-native-recovery-20261019/history-restored/disposable/store')
ARCHIVE=RUNTIME/'post-gc-operational-backup';RESTORED=RUNTIME/'history-restored/disposable/store'
action=sys.argv[1]
if action=='freeze':
    frozen=RUNTIME/'preservation-inputs';frozen.mkdir(exist_ok=False)
    original=HERE.parent/'file-native-recovery-reclamation-2026-10-19/operational_backup.py'
    text=original.read_text();assert text.count('>6*1024**3')==1
    # Only tighten this existing experimental harness's archive cap. Its original
    # byte protocol remains unchanged; retain both exact source hashes.
    bounded=text.replace('>6*1024**3','>750000000');(frozen/'operational_backup.py').write_text(bounded,encoding='utf-8')
    parent=Path('C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook/docs/research/file-native-recovery-reclamation-2026-10-19/evidence/post-gc-operational-verification-result.json')
    (frozen/'post-gc-reference.json').write_bytes(parent.read_bytes())
    files=[HERE/'guard.py',HERE/'preservation_stage.py',original,frozen/'operational_backup.py',frozen/'post-gc-reference.json']
    binding={str(p):dict(bytes=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in files}
    (frozen/'binding.json').write_text(json.dumps(binding,indent=2));print(json.dumps(binding,indent=2));sys.exit(0)
frozen=RUNTIME/'preservation-inputs';binding=json.loads((frozen/'binding.json').read_bytes())
for name,row in binding.items():assert hashlib.sha256(Path(name).read_bytes()).hexdigest()==row['sha256'],name
spec=importlib.util.spec_from_file_location('frozen_backup',frozen/'operational_backup.py');backup=importlib.util.module_from_spec(spec);spec.loader.exec_module(backup)
request=json.loads((RUNTIME/'receipts/preservation-stage-request.json').read_bytes())
def preserved():
    for name,row in request['source_bindings'].items():
        path=SOURCE/name;h=hashlib.sha256()
        with path.open('rb') as f:
            for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
        assert h.hexdigest()==row['sha256'] and path.stat().st_size==row['bytes'],name
if action=='capture':
    preserved();result=backup.capture(SOURCE,ARCHIVE,'0dbbd89775b0dc1b43b1fd1b817b0756342dba64',True);preserved()
    assert sum(p.stat().st_size for p in ARCHIVE.iterdir() if p.is_file())<=750000000
elif action=='restore':result=backup.restore(ARCHIVE,RESTORED)
elif action=='verify':
    sys.path.insert(0,str(HERE.parent/'file-native-recovery-reclamation-2026-10-19'));import verify_operational
    result=verify_operational.verify(RESTORED,frozen/'post-gc-reference.json')
    reference=json.loads((frozen/'post-gc-reference.json').read_bytes())
    for key in ['selected','publications','ledger_rows','permanent_digest','retired_publications','unique_content_roots','sources']:
        assert result[key]==reference[key],(key,result[key],reference[key])
    assert result['publications']==101006 and result['ledger_rows']==100000 and result['retired_publications']==101
    preserved()
else:raise ValueError(action)
(RUNTIME/'receipts'/('preservation-'+action+'-result.json')).write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
