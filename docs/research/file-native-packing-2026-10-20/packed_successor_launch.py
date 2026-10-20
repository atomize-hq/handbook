"""Prepared parent-conditional fresh local authority and real successor stage."""
import hashlib,json,os,runpy,sys
from pathlib import Path
from guard import current
from driver import Driver
from packed_restore_check import closed,exact,RESTORED,OUT,EXE,R,HERE

closed('packed-restored-reopen');exact();assert not (RESTORED/'DISPOSABLE.json').exists()
actual=current();assert actual['total_new_bytes']+4*1024**2<=2684354560 and actual['total_new_files']+100<=560000 and actual['free_bytes']>=140*1024**3
(OUT/'successor-fresh-admission.json').write_text(json.dumps(dict(actual=actual,new_successor_bound=4*1024**2,cap=2684354560,original_primary_unchanged=True),indent=2))
d=Driver(OUT/'authorize-events');setup=d.call(RESTORED,'lifecycle',exe_override=EXE,operation='setup')
authority=json.loads((RESTORED/'DISPOSABLE.json').read_bytes());assert os.path.normcase(os.path.normpath(authority.removeprefix('\\\\?\\')))==os.path.normcase(str(RESTORED.resolve()))
(OUT/'successor-authority-binding.json').write_text(json.dumps(dict(local_setup=setup,authority=authority,imported_from_archive=False,launcher_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()),indent=2))
sys.argv=[str(HERE/'qualified_restore_stage.py'),'successor'];runpy.run_path(str(HERE/'qualified_restore_stage.py'),run_name='__main__')
