"""Read-only final native accounting and task-process closure snapshot."""
import os,json,time,shutil,hashlib,sys
from pathlib import Path
BASE=Path(__file__).resolve().parent
sys.path.insert(0,str(BASE/'source/docs/research/file-native-navigation-2026-10-19'))
from census import census
active=[]
for p in Path('/proc').iterdir():
    if not p.name.isdigit() or int(p.name)==os.getpid():continue
    try:
        args=(p/'cmdline').read_bytes().decode(errors='replace').split('\0')
        if args and ('python' in Path(args[0]).name or 'file-native-incremental' in Path(args[0]).name) and any(str(BASE) in arg for arg in args):active.append(dict(pid=int(p.name),args=args))
    except OSError:pass
row=dict(epoch=time.time(),harness_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),native_owned=census(BASE),new_test_temporary=census('/tmp/i1-bounds-250183-1792450605137123489-0'),free_bytes=shutil.disk_usage(BASE).free,task_python_engine_processes=active,filter_scope='owned-root Python and engine argv only; this read-only checker excluded',qualified_executable_sha256=hashlib.sha256((BASE/'build/release/file-native-incremental').read_bytes()).hexdigest(),original_execution_cutoff_epoch=1792453485,original_lane_deadline_epoch=1792454085)
assert not active
print(json.dumps(row,indent=2))
