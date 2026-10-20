"""Same calibrated allocation API, with a bounded queue hiding open latency.

Read-only whole-store measurement. Requires quiescence and an external 120s
watchdog. A partial prefix is retained on censor and never called complete.
"""
import argparse,itertools,json,os,time
from collections import defaultdict,deque
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from allocation import allocation

parser=argparse.ArgumentParser();parser.add_argument('root');parser.add_argument('output');args=parser.parse_args()
root=Path(args.root).resolve();output=Path(args.output);start=time.monotonic();selected=(root/'SELECTED.json').read_bytes();store=(root/'STORE.json').read_bytes()
roles=defaultdict(lambda:dict(files=0,logical_bytes=0,allocation_bytes=0));total=dict(files=0,logical_bytes=0,allocation_bytes=0)
paths=(Path(directory)/name for directory,dirs,files in os.walk(root,followlinks=False) for name in files)
pending=deque()
with ThreadPoolExecutor(max_workers=16) as pool:
    for candidate in itertools.chain(paths,itertools.repeat(None,64)):
        if candidate is not None:
            assert not candidate.is_symlink();pending.append((candidate,pool.submit(allocation,candidate)))
        if pending and (len(pending)>=64 or candidate is None):
            path,future=pending.popleft();allocated,eof=future.result();role=path.relative_to(root).parts[0]
            for row in [roles[role],total]:row['files']+=1;row['logical_bytes']+=eof;row['allocation_bytes']+=allocated
            if total['files']%10000==0:output.with_suffix('.prefix.json').write_text(json.dumps(dict(status='incomplete-prefix',total=total,roles=roles,seconds=time.monotonic()-start),indent=2))
assert not pending and selected==(root/'SELECTED.json').read_bytes() and store==(root/'STORE.json').read_bytes()
ledger=roles.get('expected-ledger.jsonl',dict(files=0,logical_bytes=0,allocation_bytes=0));engine={k:total[k]-ledger[k] for k in total}
result=dict(status='complete',api='GetFileInformationByHandleEx(FileStandardInfo=1).AllocationSize',total=total,engine_excluding_test_ledger=engine,roles=roles,workers=16,max_pending=64,seconds=time.monotonic()-start,selected_unchanged=True,store_unchanged=True,directory_filesystem_metadata=None,physical_device_writes=None,qualification='regular-file data allocation only; coexisting immutable loose duplicates retain matched original-cohort bytes; whole-store loose allocation before activation was not measured')
output.write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2))
