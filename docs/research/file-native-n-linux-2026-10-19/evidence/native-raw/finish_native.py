"""Sequential final bounded controls; no nested process-group supervisors."""
import json,sys
import supervise as s
assert (s.BASE/'host-after-all.json').exists() or (s.BASE/'matrix-failed.json').exists()
s.STAGES=json.loads((s.BASE/'stages.json').read_bytes())
exe=s.BASE/'build/release/file-native-incremental'
s.stage('fact-dirtiness-control',[sys.executable,str(s.H/'stage1.py'),'--exe',str(exe),'--base',str(s.BASE/'stage1-data'),'--n','128'],120,checker=True)
s.stage('historical-and-restored-oracles',[sys.executable,str(s.BASE/'post_native.py')],900,checker=True)
(s.BASE/'host-after-post.json').write_text(json.dumps(s.host_scope(),indent=2))
