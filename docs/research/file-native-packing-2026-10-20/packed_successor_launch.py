"""Prepared parent-conditional fresh local authority and real successor stage."""
import hashlib,json,os,runpy,sys
from pathlib import Path
from guard import current
from driver import Driver
from packed_restore_check import closed,exact,RESTORED,OUT,EXE,R,HERE

closed('packed-restored-reopen');exact();assert not (RESTORED/'DISPOSABLE.json').exists()
actual=current();assert actual['total_new_bytes']+32*1024**2<=2684354560 and actual['total_new_files']+200<=560000 and actual['free_bytes']>=140*1024**3
(OUT/'successor-fresh-admission.json').write_text(json.dumps(dict(actual=actual,new_successor_bound=4*1024**2,all_stage_return_margin=32*1024**2,cap=2684354560,original_primary_unchanged=True),indent=2))
d=Driver(OUT/'authorize-events');setup=d.call(RESTORED,'lifecycle',exe_override=EXE,operation='setup')
authority=json.loads((RESTORED/'DISPOSABLE.json').read_bytes());assert os.path.normcase(os.path.normpath(authority.removeprefix('\\\\?\\')))==os.path.normcase(str(RESTORED.resolve()))
root_stat=RESTORED.stat();marker_stat=(RESTORED/'DISPOSABLE.json').stat()
(OUT/'successor-authority-binding.json').write_text(json.dumps(dict(local_setup=setup,authority=authority,root_identity=dict(device=root_stat.st_dev,inode=root_stat.st_ino),new_marker_identity=dict(device=marker_stat.st_dev,inode=marker_stat.st_ino),imported_from_archive=False,launcher_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()),indent=2))
sys.argv=[str(HERE/'qualified_restore_stage.py'),'successor'];runpy.run_path(str(HERE/'qualified_restore_stage.py'),run_name='__main__')
retired=0
for marker in sorted((RESTORED/'retired').glob('*.json')):
    assert d.call(RESTORED,'point',exe_override=EXE,code=2,selected=json.loads(marker.read_bytes()),id='d000000')['error']=='retired:snapshot';retired+=1
intent=json.loads((R/'long-matrix/P0-intent.json').read_bytes());frozen=json.loads((R/'long-matrix/frozen.json').read_bytes())
assert d.call(RESTORED,'resolve',exe_override=EXE,request_id=intent['request'],intent=intent['original_intent'])['result']==frozen['expected_outcomes'][intent['request']]
assert d.call(RESTORED,'resolve',exe_override=EXE,code=2,request_id=intent['request'],intent=hashlib.sha256(b'successor-wrong-intent').hexdigest())['error']==intent['different_intent_refusal']
assert retired==101
(OUT/'successor-preservation.json').write_text(json.dumps(dict(status='passed',retired_refusals=retired,matching_changed_intent=True,events=len(d.events)),indent=2))
