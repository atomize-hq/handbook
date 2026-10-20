"""Control-only capsule dispatch/source/binary/interface binding check."""
import json
from pathlib import Path
import native_guard as guard
from native_capsule_adapter import run
from native_driver import ACCEPTED_GUARD

r = guard.ROOT
source = r/'source'
out = r/'evidence/capsule-adapter-control'
out.mkdir(exist_ok=False)
names = ['fake_capsule.py', 'fake_protocol.py', 'native_capsule_adapter.py', 'native_driver.py']
manifest = dict(suite_pin='CONTROL-ONLY-CAPSULE-NO-ENGINE', guard_sha256=ACCEPTED_GUARD,
                executable=dict(path=str(source/'fake_protocol.py'),
                                sha256=guard.digest(source/'fake_protocol.py'),
                                source_pin='CONTROL-ONLY-CAPSULE-NO-ENGINE'),
                source_files={name: dict(bytes=(source/name).stat().st_size,
                                        sha256=guard.digest(source/name)) for name in names},
                stages={'control': dict(script='fake_capsule.py', driver_interface='legacy',
                                        argv=['--base', str(r/'fixtures/control-capsule-adapter')],
                                        receipt=str(out/'adapter-receipt.json'),
                                        case_mapping={'CONTROL-ONLY': 'fake-protocol-interface-check'})})
p = out/'manifest.json'
p.write_text(json.dumps(manifest, indent=2)+'\n')
result = run(p, 'control')
assert result['status'] == 'passed' and result['source_and_binary_verified_before_after']
assert not result['semantic_decoder_substituted'] and not result['windows_controls_stubbed']
print(json.dumps(dict(status='passed-control-only', no_engine_invocation=True,
                      adapter_sha256=guard.digest(source/'native_capsule_adapter.py'),
                      driver_sha256=guard.digest(source/'native_driver.py'),
                      receipt_sha256=guard.digest(out/'adapter-receipt.json')), indent=2))
