import hashlib,json,os,shutil,subprocess,sys,tarfile,time
from pathlib import Path,PurePosixPath
started=time.time();source=Path('/home/spenser/file-native-incremental-source-5a65a21');runtime=Path('/home/spenser/file-native-incremental-linux-accounting-final');archive=Path('/home/spenser/capsule-5a65a21.tar')
source.mkdir(exist_ok=False)
with tarfile.open(archive) as t:
 for m in t.getmembers():assert not PurePosixPath(m.name).is_absolute() and '..' not in PurePosixPath(m.name).parts and (m.isdir() or m.isfile())
 t.extractall(source)
p=subprocess.run([sys.executable,str(source/'accounting_lane.py'),'--root',str(runtime),'--source',str(source),'--pin','5a65a2155f9eb297d2cbb1b7c900fa749629605c','--deadline-epoch',str(started+1500)],timeout=max(1,1800-(time.time()-started)))
sys.path.insert(0,str(source));from pack_evidence import export
destination=Path('/home/spenser/file-native-incremental-linux-accounting-evidence');manifest=export(runtime,destination)
sha=lambda b:hashlib.sha256(b).hexdigest();exe=runtime/'build/release/file-native-incremental'
owned=[p for p in Path.home().glob('file-native-incremental-*') if p.is_dir()]
size=sum((Path(d)/n).stat().st_size for root in owned for d,_,names in os.walk(root) for n in names if (Path(d)/n).exists())
environment=dict(pin='5a65a2155f9eb297d2cbb1b7c900fa749629605c',source_tar_sha256=sha(archive.read_bytes()),exe_sha256=sha(exe.read_bytes()) if exe.exists() else None,outer_exit=p.returncode,elapsed_including_export=time.time()-started,cap_seconds=1800,work_deadline_epoch=started+1500,owned_directory_logical_bytes=size,additional_source_tar_bytes=archive.stat().st_size,free_bytes=shutil.disk_usage(runtime).free)
assert size+archive.stat().st_size<=6*1024**3 and environment['free_bytes']>=20*1024**3 and environment['elapsed_including_export']<1800
(destination/'accounting-environment.json').write_text(json.dumps(environment,indent=2));shutil.copyfile(__file__,destination/'linux_accounting_launch.py')
with tarfile.open('/home/spenser/native-linux-accounting-evidence.tar.gz','w:gz') as t:t.add(destination,arcname='native-linux-accounting')
print(json.dumps(dict(environment=environment,events=manifest['events'],files=len(manifest['files']))))
