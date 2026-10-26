from pathlib import Path
import os,sys,json,hashlib,importlib.util
out=Path(__file__).resolve().parent
source=Path('/home/spenser/.codex/worktrees/file-native-accepted-checkpoint-20261026/handbook/docs/research/file-native-incremental-engine-experiment-2026-10-18')
spec=importlib.util.spec_from_file_location('existing_supervisor','/tmp/file-native-host-inventory-linux-20261024-01a16fd3/run-final062-resumed.py');s=importlib.util.module_from_spec(spec);spec.loader.exec_module(s);s.OUT=out;s.SOURCE=source
name,pattern=sys.argv[1:];fixture=out/'fixtures'/name;fixture.mkdir()
def manifest():return [{'path':str(p.relative_to(source)),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(source.rglob('*')) if p.is_file()]
before=manifest();env=os.environ.copy()
for k in list(env):
 if k.startswith('EXP_') or k in ['RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','RUST_TEST_FILTER','RUST_TEST_THREADS']:env.pop(k)
env.update(EXP_PACK_TEST_ROOT=str(fixture),TMPDIR=str(out/'tmp'),CARGO_INCREMENTAL='0',CARGO_TERM_COLOR='never',PYTHONDONTWRITEBYTECODE='1')
argv=['cargo','test','--lib',pattern,'--manifest-path',str(source/'Cargo.toml'),'--locked','--offline','--release','-j2','--target-dir',str(out/'target'),'--','--nocapture','--test-threads=1']
(out/(name+'-command.json')).write_text(json.dumps({'argv':argv,'env_selected':{k:env[k] for k in ['EXP_PACK_TEST_ROOT','TMPDIR','CARGO_INCREMENTAL']}},indent=2)+'\n')
r=s.stage(name,argv,600,env);assert before==manifest(),'Source changed during stage'
binary=out/'target/release/deps/file_native_incremental-f5485a3108d76dc6'
(out/(name+'-binding.json')).write_text(json.dumps({'source_files':before,'binary':str(binary),'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest() if binary.exists() else None,'source_unchanged_during_stage':True,'exit':r['exit']},indent=2)+'\n')
