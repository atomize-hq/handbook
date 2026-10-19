"""Additional cases under the unchanged scheduler and original deadline."""
import pathlib
source=pathlib.Path(__file__).with_name('qualify.py').read_text()
# Reuse only definitions, not qualify.py's top-level execution block.
exec(compile(source.partition('\ntry:\n')[0],str(pathlib.Path(__file__).with_name('qualify.py')),'exec'))
exe=R/'build/debug/file-native-incremental'
try:
    stage('capsule-v2',[sys.executable,R/'bounded_capsule_v2.py',str(exe),R/'capsule-v2-data'],1800)
    stage('io-compile',['rustc','--edition=2021',R/'io_probe.rs','--extern','file_native_incremental='+str(R/'build/debug/libfile_native_incremental.rlib'),'-L','dependency='+str(R/'build/debug/deps'),'-o',R/'io-probe'],30)
    stage('io-probe',[R/'io-probe',R/'io-data'],30)
    stage('deep-cases',[sys.executable,E/'deep_cases.py','--exe',exe,'--base',R/'deep-data'],120)
    stage('paired-intents',[sys.executable,E/'paired_intents.py','--exe',exe,'--base',R/'paired-data','--deadline-epoch',str(STOP)],120)
    stage('replay-lifetime',[sys.executable,E/'replay_lifetime.py','--exe',exe,'--base',R/'replay-data'],120)
finally:
    (R/'continuation.json').write_text(json.dumps(dict(stages=rows,census=census(),original_deadline_utc='2026-10-19T22:38:49Z',epoch=time.time()),indent=2))
