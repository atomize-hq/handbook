"""Final scoped host census, preservation comparison and task-process closure."""
import os,sys,json,hashlib,time,shutil
from pathlib import Path
BASE=Path(__file__).resolve().parent;OWNED=BASE.parent
sys.path.insert(0,str(BASE/'source/docs/research/file-native-navigation-2026-10-19'))
from census import census
rows=[]
for p in sorted(Path('/home/spenser').glob('file-native*')):rows.append(census(p) if p.is_dir() else dict(root=str(p),files=1,logical_bytes=p.stat().st_size))
for p in [Path('/home/spenser/.cargo/registry'),Path('/home/spenser/.cargo/git')]:
    if p.exists():rows.append(census(p))
old=json.loads((OWNED/'host-before.json').read_bytes());before={r['root']:r for r in old['roots']};after={r['root']:r for r in rows}
changed=[dict(root=k,before=v,after=after[k]) for k,v in before.items() if k!=str(OWNED) and (v['files'],v['logical_bytes'])!=(after[k]['files'],after[k]['logical_bytes'])]
active=[]
for path in Path('/proc').iterdir():
    if not path.name.isdigit() or int(path.name)==os.getpid():continue
    try:
        args=(path/'cmdline').read_bytes().decode(errors='replace').split('\0')
        if args and ('python' in Path(args[0]).name or 'file-native-incremental' in Path(args[0]).name) and any(str(OWNED) in arg for arg in args):active.append(dict(pid=int(path.name),args=args))
    except OSError:pass
assert not changed and not active
row=dict(epoch=time.time(),harness_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),candidate='4bde156323812ef18c0dea6798cfc2108aaad933',native_owned=census(OWNED),new_test_temporary=census('/tmp/i1-bounds-250183-1792450605137123489-0'),relevant_preserved_host_scope=rows,changed_prior_scopes=changed,task_processes=active,filter_scope='owned-root Python/engine argv, this checker excluded',free_bytes=shutil.disk_usage(BASE).free,shared_cargo_logical_growth_bytes=json.loads((BASE/'review-summary.json').read_bytes())['shared_cargo_logical_growth_bytes'],original_execution_cutoff_epoch=1792453485,original_lane_deadline_epoch=1792454085)
print(json.dumps(row,indent=2))
