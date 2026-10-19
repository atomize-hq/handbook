"""Associate exported WSL working Rust/Cargo bytes and retained native binaries."""
import argparse,gzip,hashlib,io,json,subprocess,time,zipfile
from pathlib import Path

a=argparse.ArgumentParser();a.add_argument('--scratch',required=True);a.add_argument('--output',required=True);ns=a.parse_args()
scratch=Path(ns.scratch);source=Path(__file__).resolve().parent
end=json.loads((scratch/'final-proofs-v3/reserve-bound.json').read_bytes())['conservative_deadline_epoch'];assert time.time()+300<end
inputs=[r['path'] for r in json.loads((source/'evidence/windows-accounting-source-binding.json').read_bytes())['files']]
lanes=[('native-wsl-main','7ac70f93cce14ecd3d935852801f034c122000d8','/home/spenser/file-native-incremental-wsl-final-20261018'),('native-wsl-audit','7e00d22b470c1eed476127d2f585789036e79cf3','/home/spenser/file-native-incremental-wsl-audit-final'),('native-wsl-accounting','5a65a2155f9eb297d2cbb1b7c900fa749629605c','/home/spenser/file-native-incremental-wsl-accounting-final')]
prefix='docs/research/file-native-incremental-engine-experiment-2026-10-18/';rows=[]
for name,pin,root in lanes:
 manifest=source/'evidence'/name/'manifest.json'
 if not manifest.exists():rows.append(dict(lane=name,unperformed='No completed lossless export'));continue
 expected={p:hashlib.sha256(subprocess.check_output(['git','show',pin+':'+prefix+p],cwd=source,timeout=10)).hexdigest() for p in inputs}
 m=json.loads(manifest.read_bytes());files={r['path']:r for r in m['files']};sessions=[]
 for path,r in files.items():
  if not path.endswith('session.json'):continue
  session=json.loads(gzip.decompress((manifest.parent/r['archive']).read_bytes()))
  archive_path=str(Path(path).with_name('session-source.zip')).replace('\\','/');entry=files[archive_path]
  raw=gzip.decompress((manifest.parent/entry['archive']).read_bytes());digest=hashlib.sha256(raw).hexdigest();assert digest==session['source_archive_sha256']
  with zipfile.ZipFile(io.BytesIO(raw)) as z:
   actual={p:hashlib.sha256(z.read(p)).hexdigest() for p in inputs}
  assert actual==expected,(name,path,'Rust/Cargo differs from claimed source')
  sessions.append(dict(session=path,exe_sha256=session['sha256'],source_archive_sha256=digest,checked_input_files=len(inputs)))
 rows.append(dict(lane=name,pin=pin,guest_runtime=root,inputs=expected,sessions=sessions))
guest="""import hashlib,json,subprocess
from pathlib import Path
roots=ROOTS;binaries=[]
for name,root in roots:
 for basename in ['file-native-incremental','project-consumer','research-consumer']:
  p=Path(root)/'build/release'/basename
  if p.is_file():binaries.append(dict(lane=name,path=str(p),bytes=p.stat().st_size,sha256=hashlib.sha256(p.read_bytes()).hexdigest()))
print(json.dumps(dict(binaries=binaries,uname=subprocess.check_output(['uname','-a'],text=True),filesystem=subprocess.check_output(['df','-T','/home/spenser'],text=True),rustc=subprocess.check_output([str(Path.home()/'.cargo/bin/rustc'),'--version'],text=True),python=subprocess.check_output(['python3','--version'],text=True))))
""".replace('ROOTS',repr([(name,root) for name,_,root in lanes]))
actual=json.loads(subprocess.check_output(['wsl','-d','Ubuntu-24.04','--','python3','-c',guest],timeout=45).decode('utf-8'))
for row in rows:
 if 'sessions' not in row:continue
 hashes={r['sha256'] for r in actual['binaries'] if r['lane']==row['lane']}
 assert hashes and all(s['exe_sha256'] in hashes for s in row['sessions']),row['lane']
report=dict(epoch=time.time(),lanes=rows,retained_native_environment=actual,qualification='Every exported session Rust/Cargo input listed here equals its named Git pin; source ZIPs are working snapshots at controller creation. Completed build/controller records establish ordering. Session executable hashes also match retained guest binaries rehashed after work. Captured inputs include the separate control crate and are not all compiled into each binary. This is source/binary association, not reproducible compilation or historical toolchain reconstruction.')
Path(ns.output).write_text(json.dumps(report,indent=2));print(json.dumps(dict(lanes=len(rows),sessions=sum(len(r.get('sessions',[])) for r in rows),retained_binaries=len(actual['binaries']))))
