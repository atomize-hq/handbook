"""Schedule supplements and growth only after the original focused capsule passed."""
import json,sys
import supervise as s
assert json.loads((s.BASE/'focused-data/summary.json').read_bytes())['status']=='success'
s.STAGES=json.loads((s.BASE/'stages.json').read_bytes())
s.stage('supplement',[sys.executable,str(s.BASE/'supplement_native.py')],900,checker=True)
s.stage('matrix-controller',[sys.executable,str(s.BASE/'matrix_native.py')],1800,checker=True)
(s.BASE/'host-after-all.json').write_text(json.dumps(s.host_scope(),indent=2))
