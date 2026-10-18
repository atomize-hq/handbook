"""Preserve the already transported 5a source TAR, without rebuilding it."""
import gzip,hashlib,io,json,subprocess,tarfile
from pathlib import Path

scratch=Path('C:/file-native-incremental-20261018')
work=Path('C:/Users/spmcc/.codex/worktrees/file-native-incremental-20261018/handbook')
relative='docs/research/file-native-incremental-engine-experiment-2026-10-18'
destination=work/relative/'evidence/source-capsules-accounting'
destination.mkdir(exist_ok=False)
pin='5a65a2155f9eb297d2cbb1b7c900fa749629605c'
source=scratch/'capsule-5a65a21.tar';raw=source.read_bytes()
digest=hashlib.sha256(raw).hexdigest()
assert digest=='7e51fed2dcb487afdbbe17cbd1eb77cf7efba8c4262082c790fb83a0b24ec7fd'
hashes={}
with tarfile.open(fileobj=io.BytesIO(raw)) as archive:
 for member in archive.getmembers():
  if member.isfile() and (member.name.endswith('.rs') or member.name.endswith('/Cargo.toml') or member.name in {'Cargo.toml','Cargo.lock'}):
   captured=archive.extractfile(member).read()
   committed=subprocess.check_output(['git','show',pin+':'+relative+'/'+member.name],cwd=work)
   assert captured==committed,member.name
   hashes[member.name]=hashlib.sha256(captured).hexdigest()
encoded=gzip.compress(raw,compresslevel=9,mtime=0)
assert gzip.decompress(encoded)==raw
target=destination/(source.name+'.gz');target.write_bytes(encoded)
receipt=dict(pin=pin,source=source.name,tar_bytes=len(raw),tar_sha256=digest,archive=target.name,archive_bytes=len(encoded),archive_sha256=hashlib.sha256(encoded).hexdigest(),rust_and_cargo_hashes=hashes,all_listed_inputs_equal_git_pin=True,qualification='Exact existing transport bytes, including their then-current evidence/helpers; this receipt does not claim a reproducible build.')
(destination/'manifest.json').write_text(json.dumps(receipt,indent=2))
print(json.dumps(dict(files=len(hashes),archive_bytes=len(encoded),tar_sha256=digest)))
