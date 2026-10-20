"""Small byte-protocol/prewrite-bound check, never a semantic engine fixture."""
import json
from pathlib import Path
from packed_backup_stage import adapted,R

base=R/'packed-adapter-check';base.mkdir(exist_ok=False)
source=base/'source';source.mkdir();(source/'SELECTED.json').write_bytes(b'{}');(source/'STORE.json').write_bytes(b'{"version":2}')
(source/'nested').mkdir();(source/'nested/exact.txt').write_bytes('exact café\r\n'.encode()*73)
(source/'DISPOSABLE.json').write_bytes(b'old-path-authority');(source/'pack-readers.lock').touch();(source/'pack-maintenance.lock').touch()
backup,capture_quota,restore_quota=adapted('adapter-check')
capture=backup.capture(source,base/'capture','byte-protocol-only',True);restore=backup.restore(base/'capture',base/'restored')
assert capture['files']==3 and capture['logical_bytes']==restore_quota.used
for path in ['SELECTED.json','STORE.json','nested/exact.txt']:assert (source/path).read_bytes()==(base/'restored'/path).read_bytes()
assert not (base/'restored/DISPOSABLE.json').exists() and not (base/'restored/pack-readers.lock').exists()
capture_quota.limit=capture_quota.used+8
try:backup.capture(source,base/'censored-capture','byte-protocol-only',True)
except ValueError as error:assert str(error)=='prewrite-byte-cap'
else:raise AssertionError('capture limit failed')
assert not (base/'censored-capture/manifest.json').exists()
restore_quota.limit=restore_quota.used+8
try:backup.restore(base/'capture',base/'censored-restore')
except ValueError as error:assert str(error)=='prewrite-byte-cap'
else:raise AssertionError('restore limit failed')
assert capture_quota.used<=capture_quota.limit and restore_quota.used<=restore_quota.limit
result=dict(status='passed',profile='FNOP1 byte transport only, no engine semantics claimed',capture=capture,restore=restore,excluded_disposable_and_pack_locks=True,prewrite_capture_and_restore_refusals_preserved=True,whole_packed_operational_restore_still_unperformed=True)
(base/'summary.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
