"""Prepared one N8000 A/B baseline or cell, ending with bounded full archive.

Every invocation requires fresh parent admission. No store reclamation occurs
here; parent verifies each sole operational image before separate exact reclaim.
"""
import argparse,hashlib,json,subprocess,sys
from pathlib import Path
import operational_matrix as matrix
from packed_backup_stage import adapted,sha_file,R,HERE
from guard import current,census

parser=argparse.ArgumentParser();parser.add_argument('stage',choices=['baseline','cell']);parser.add_argument('--mode',choices=['A','B'],required=True);parser.add_argument('--representation',choices=['L-old','L-new','P0','PZ']);parser.add_argument('--path-v2',action='store_true');args=parser.parse_args()
assert (args.stage=='cell')==(args.representation is not None)
store_limit=402653184 if args.mode=='A' else 600000000
actual=current();assert actual['total_new_bytes']+store_limit+67108864+33554432<=2684354560 and actual['total_new_files']+50010<=560000 and actual['free_bytes']>=140*1024**3
old=Path('C:/file-native-navigation-20261019/bin/final.exe');new=R/'bin/candidate2.exe';helper=R/'build/release/qualified_lookup.exe'
assert sha_file(old)=='d4b7c495abfc029e8fabd6fcfc75501c5de04ce72dd38847a69bc8824022303f' and sha_file(new)=='568f631acf528c7de911bb88ec85d07ead9d7b4435ebe970c61053f06e31a93b'
assert sha_file(helper)==json.loads((R/'long-matrix/L-old-measurement.json').read_bytes())['warm_binary_sha256']
assert sha_file(HERE/'independent.py')=='94987dbe1c992ef2491bb9819ad90185d2f29aef704b1bfaaa8f7e052a449e66'
core=json.loads((R/'fix2/build-binding.json').read_bytes())['files']
for path,row in core.items():assert sha_file(path)==row['sha256']
base=R/'operational-matrix';base.mkdir(exist_ok=True);epoch='-path-v2' if args.path_v2 else '';cell='baseline' if args.stage=='baseline' else args.representation+epoch;name=f'n8000-{args.mode}-{cell}'
out=base/'stage-evidence'/name;out.mkdir(parents=True,exist_ok=False)
sources={}
for item in ['sequenced_matrix_stage.py','operational_matrix.py','packed_backup_stage.py','independent.py','driver.py','guard.py']:
    raw=(HERE/item).read_bytes();(out/item).write_bytes(raw);sources[item]=hashlib.sha256(raw).hexdigest()
checker=out/'independent-archive-checker.py';checker.write_bytes(Path('C:/file-native-qnp-20261019/verify-operational-image.py').read_bytes())
(out/'admission.json').write_text(json.dumps(dict(actual=actual,new_store_cap=store_limit,new_archive_prewrite_cap=67108864,all_returns=33554432,cap=2684354560,files=50010,source_raw_authored_exact_bytes=101580800),indent=2))
backup,capture_quota,restore_quota=adapted('matrix-'+name);capture_quota.limit=67108864;restore_quota.limit=store_limit;backup.MAX_FILES=50000;backup.MAX_BYTES=store_limit;matrix.backup=backup
(out/'effective-binding.json').write_text(json.dumps(dict(sources=sources,old_exe_sha256=sha_file(old),new_exe_sha256=sha_file(new),qualified_warm_helper_sha256=sha_file(helper),effective_archive_prewrite_cap=capture_quota.limit,effective_restore_prewrite_cap=restore_quota.limit,effective_source_files_cap=backup.MAX_FILES,effective_source_file_bytes_cap=backup.MAX_BYTES,H_initial=8,H_after=15,representations=['L-old','L-new','P0','PZ'],original_cohort_preserved=True,path_only_case_supported_by_actual_graph_transition=True),indent=2))
case=base/f'n8000-{args.mode}'
if args.stage=='baseline':
    matrix.initial(args.mode,8000);root=case/'baseline/disposable/store';capsule=case/'baseline-operational-backup';manifest=json.loads((capsule/'manifest.json').read_bytes())
else:
    matrix.cell(args.mode,8000,args.representation,epoch=epoch,store_limit=store_limit if args.mode=='A' else None);root=case/cell/'disposable/store';capsule=case/(cell+'-operational-backup');creator='4bde156323812ef18c0dea6798cfc2108aaad933' if args.representation=='L-old' else 'd70f3864f2888d46d9112213b4a03f8d82745a8e';manifest=backup.capture(root,capsule,creator,True)
assert census(root)['bytes']<=store_limit and census(root)['files']<=50000 and capture_quota.used<=67108864
checked=subprocess.run([sys.executable,str(checker),str(capsule),str(root),manifest['transport_sha256'],manifest['inventory_sha256'],str(manifest['files']),str(manifest['logical_bytes'])],capture_output=True,timeout=115)
(out/'worker-archive-check.stdout').write_bytes(checked.stdout);(out/'worker-archive-check.stderr').write_bytes(checked.stderr);assert checked.returncode==0
archive_check=json.loads(checked.stdout);assert archive_check['whole_gzip_EOF'] and archive_check['complete_current_source_pathset_and_hashes_match']
for path,row in core.items():assert sha_file(path)==row['sha256']
assert sha_file(old)=='d4b7c495abfc029e8fabd6fcfc75501c5de04ce72dd38847a69bc8824022303f' and sha_file(new)=='568f631acf528c7de911bb88ec85d07ead9d7b4435ebe970c61053f06e31a93b'
assert sha_file(helper)==json.loads((R/'long-matrix/L-old-measurement.json').read_bytes())['warm_binary_sha256']
for item,digest in sources.items():assert sha_file(HERE/item)==digest
actual_core='4bde156323812ef18c0dea6798cfc2108aaad933' if args.stage=='baseline' or args.representation=='L-old' else 'd70f3864f2888d46d9112213b4a03f8d82745a8e'
result=dict(status='measured-and-archived-pending-parent-independent-stream-verification',mode=args.mode,documents=8000,cell=cell,root=str(root),capsule=str(capsule),manifest=manifest,archive_files={p.name:dict(bytes=p.stat().st_size,sha256=sha_file(p)) for p in capsule.iterdir() if p.is_file()},source_store=census(root),source_reclaimed=False,capture_charged_bytes=capture_quota.used,restore_charged_bytes=restore_quota.used,actual_creation_or_reader_core=actual_core,frozen_final_candidate_core='d70f3864f2888d46d9112213b4a03f8d82745a8e',packing_core='d70f3864f2888d46d9112213b4a03f8d82745a8e' if args.representation in ['P0','PZ'] else None,worker_invoked_independently_authored_archive_parser=archive_check,archive_parser_sha256=sha_file(checker),parent_separate_review_pending=True)
(out/'summary.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
