"""Sanitize any surviving trace after interruption; never publish its raw contents."""
import json
from pathlib import Path
import sys
from driver import trace_summary
root, output = Path(sys.argv[1]), Path(sys.argv[2])
output.mkdir(parents=True, exist_ok=True)
observed = []
if root.is_dir():
    for folder in sorted(root.glob('attempt-*')):
        suffix = folder.name.removeprefix('attempt-')
        if not suffix.isdigit():
            continue
        slot = int(suffix)
        trace = folder / 'proof' / 'trace.jsonl'
        if trace.is_file():
            observed.append({'slot': slot, 'trace': trace_summary(trace)})
(output / 'recovered.json').write_text(json.dumps({'observed': observed}, indent=2) + '\n')
