"""Preserve both failed archive names, then run the admitted fresh path-v2 slot."""
import json,runpy,shutil,sys
from pathlib import Path
from guard import current
from packed_backup_stage import R,HERE,sha_file

receipt=json.loads((R/'receipts/matrix-failed-A-L-old-preservation.json').read_bytes());assert receipt['exit']==0 and receipt['process_closure_verified']
case=R/'operational-matrix/n8000-A';original=case/'L-old-operational-backup';named=case/'L-old-failed-path-operational-backup';extra=sum(p.stat().st_size for p in original.iterdir() if p.is_file())
actual=current();assert actual['total_new_bytes']+extra+600000000+67108864+33554432<=2684354560 and actual['total_new_files']+50013<=560000 and actual['free_bytes']>=140*1024**3
named.mkdir(exist_ok=False);rows={}
for p in original.iterdir():
    assert p.is_file();destination=named/p.name;shutil.copyfile(p,destination);assert p.stat().st_size==destination.stat().st_size and sha_file(p)==sha_file(destination);rows[p.name]=dict(bytes=p.stat().st_size,sha256=sha_file(p))
(R/'receipts/failed-prefix-archive-name-binding.json').write_text(json.dumps(dict(original_failed_archive=str(original),additional_explicit_failed_archive=str(named),both_retained=True,both_counted_bytes=2*extra,archive_files=rows,no_relabelled_pass=True,wrapper_sha256=sha_file(__file__),preflight=actual),indent=2))
sys.argv=[str(HERE/'sequenced_matrix_stage.py'),'cell','--mode','A','--representation','L-old','--path-v2'];runpy.run_path(str(HERE/'sequenced_matrix_stage.py'),run_name='__main__')
