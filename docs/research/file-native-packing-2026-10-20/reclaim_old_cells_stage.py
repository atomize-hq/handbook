"""Parent-admitted failed/valid old-L reclaims, with mandatory valid-slot dry run."""
import json,subprocess,time
from pathlib import Path
from guard import current
from packed_backup_stage import R,HERE,sha_file

old=HERE/'reclaim_operational_image_fast.ps1';slots=HERE/'reclaim_operational_image_fast_slots.ps1'
assert sha_file(old)=='3cd743cafebf5c4ff13f7c722c3c243ecac69d2d944e90e89d31f5891259efde'
before=b'(baseline|L-old|L-new|P0|PZ)';after=b'(baseline|L-old|L-new|P0|PZ|L-old-path-v2|L-new-path-v2|P0-path-v2|PZ-path-v2)'
assert old.read_bytes().count(before)==1 and slots.read_bytes()==old.read_bytes().replace(before,after)
slot_sha=sha_file(slots);actual=current();assert actual['total_new_bytes']+8*1024**2<=2684354560 and actual['free_bytes']>=140*1024**3
out=R/'receipts';events=[]
def invoke(script,slot,capsule,hashes,proof,receipt,execute,seconds):
    argv=['powershell','-NoProfile','-File',str(script),'-Root',str(R/'operational-matrix/n8000-A'/slot/'disposable/store'),'-CapsuleRoot',str(R/'operational-matrix/n8000-A'/capsule),'-ExpectedManifest',hashes[0],'-ExpectedInventory',hashes[1],'-ExpectedTransport',hashes[2],'-ParentProof',str(Path('C:/file-native-qnp-20261019')/proof),'-Receipt',str(out/receipt)]
    if execute:argv.append('-Execute')
    start=time.monotonic();body=subprocess.run(argv,capture_output=True,timeout=seconds);(out/(receipt+'.stdout')).write_bytes(body.stdout);(out/(receipt+'.stderr')).write_bytes(body.stderr)
    events.append(dict(argv=argv,exit=body.returncode,seconds=time.monotonic()-start,execute=execute));(out/'old-cell-reclaim-events.json').write_text(json.dumps(events,indent=2));assert body.returncode==0
    result=json.loads((out/receipt).read_bytes());assert result['failure'] is None
    return result
failed=invoke(old,'L-old','L-old-operational-backup',['062894fcc926a2d6c164a93fed8b97f29b538a8e158c17139411aef86443ea05','b79460df3a205231df84aac85cbf6eebe0f02ef69a4e490d34ba871d64b5d309','1daf9bcbcd3bbf06797db17259837d09a8b772586a4d12f6fb8b1e20d088418d'],'N8000-A-failed-L-old-parent-verify.txt','failed-L-old-fast-exec.json',True,850)
assert failed['removed_files']==16554 and failed['removed_bytes']==213842764
hashes=['6e6a2adb00cc9e8605fcee57024b56abb306894f903e18486db502b0c2146057','d663c5ed226b2ea469f54c742d27b22a9bb5813554672b9fcad3e66b151e3533','a5f06b3dc8486afe7e6e9d31786c829b77dbe555b4729e1c8c7136a394da66d1']
dry=invoke(slots,'L-old-path-v2','L-old-path-v2-operational-backup',hashes,'N8000-A-L-old-v2-parent-verify.txt','valid-L-old-fast-dry.json',False,120)
assert dry['planned_files']==16565 and dry['planned_bytes']==213924625 and dry['removed_files']==dry['removed_bytes']==0 and sha_file(slots)==slot_sha
valid=invoke(slots,'L-old-path-v2','L-old-path-v2-operational-backup',hashes,'N8000-A-L-old-v2-parent-verify.txt','valid-L-old-fast-exec.json',True,850)
assert valid['removed_files']==16565 and valid['removed_bytes']==213924625 and sha_file(slots)==slot_sha
result=dict(status='complete',failed_removed_files=failed['removed_files'],failed_removed_bytes=failed['removed_bytes'],valid_removed_files=valid['removed_files'],valid_removed_bytes=valid['removed_bytes'],valid_dry_exact=True,old_source_sha256=sha_file(old),fixed_slots_source_sha256=slot_sha,all_existing_excluded_controls_and_directories_retained=True,both_verified_archives_and_all_events_retained=True)
(out/'old-cell-reclaim-summary.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
