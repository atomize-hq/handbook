import subprocess,sys,pathlib,glob,time,json,os
r=pathlib.Path(__file__).parent;w=pathlib.Path('/Users/spensermcconnell/.codex/worktrees/file-native-q-macos-20261019/handbook');f=w/'docs/research/file-native-recovery-reclamation-2026-10-19';e=w/'docs/research/file-native-incremental-engine-experiment-2026-10-18';exe=r/'engine'
def stage(name,cap,args,cwd=None):
 return subprocess.run([sys.executable,r/'run.py',name,str(cap),*map(str,args)],cwd=cwd).returncode
stage('cargo-test-native',120,['cargo','test','--locked','--offline','--workspace','-j2'],e)
stage('extra',120,[sys.executable,f/'focused_extra.py','--exe',exe,'--base',r/'extra'])
stage('restored-fences',120,[sys.executable,f/'restored_fences.py','--exe',exe,'--capsule',r/'capsule','--output',r/'evidence/restored-fences-results.json'])
stage('delta',120,[sys.executable,r/'delta.py'])
stage('bash-extraction',120,['/bin/bash',r/'bash-extract.sh'])
lib=glob.glob(str(r/'build/release/deps/libfile_native_incremental-*.rlib'))[0]
stage('native-fileops-build',120,['rustc','--edition=2021',r/'native_ops.rs','--extern','file_native_incremental='+lib,'-L','dependency='+str(r/'build/release/deps'),'-o',r/'native_ops'])
stage('native-fileops',120,[r/'native_ops'])
stage('lock-and-checkpoint',120,[sys.executable,e/'deep_cases.py','--exe',exe,'--base',r/'deep'])
stage('legacy-build',120,['cargo','build','--locked','--offline','--release','-j2'],r/'legacy-source')
# Legacy binary was built to the shared target after final qualification; bind both immediately.
legacy=r/'build/release/file-native-incremental';import shutil,hashlib
shutil.copyfile(legacy,r/'legacy-engine');(r/'legacy-engine').chmod(0o755)
stage('legacy-create',120,[sys.executable,f/'legacy_compatibility.py','--exe',r/'legacy-engine','--fixtures',r/'unused','--base',r/'legacy-fixtures','--create','--packet'])
stage('legacy-replay',120,[sys.executable,f/'legacy_compatibility.py','--exe',r/'final-engine','--fixtures',r/'legacy-fixtures','--base',r/'legacy-replayed'])
