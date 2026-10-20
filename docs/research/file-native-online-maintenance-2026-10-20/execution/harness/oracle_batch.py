"""Independent sequential checker subprocesses inside one inherited stage Job."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import sys
from job_guard import sha_file, write_json

parser = argparse.ArgumentParser()
parser.add_argument('--module', required=True)
parser.add_argument('--module-sha256', required=True)
parser.add_argument('--output', required=True)
parser.add_argument('--root', action='append', required=True)
args = parser.parse_args()
assert os.environ.get('FILE_NATIVE_STAGE_JOB') == '1' and sys.dont_write_bytecode
module = Path(args.module).resolve()
assert sha_file(module) == args.module_sha256
output = Path(args.output)
assert not output.exists()
output.mkdir(parents=True)
rows = []
for item in args.root:
    label, root = item.split('=',1)
    assert label.isalnum()
    command = [sys.executable,'-B',str(Path(__file__).with_name('quiescent_oracle.py')),
               '--root',root,'--module',str(module),'--module-sha256',args.module_sha256,
               '--output',str(output/(label+'.json'))]
    # The already attached parent Job's process/aggregate hard caps and sampled
    # member monitor cover every child; no engine or nested Job is launched.
    result = subprocess.run(command,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=300)
    (output/(label+'.stdout')).write_bytes(result.stdout)
    (output/(label+'.stderr')).write_bytes(result.stderr)
    row = dict(label=label,root=root,exit=result.returncode,
               stdout=result.stdout.decode(),stderr=result.stderr.decode())
    rows.append(row)
    write_json(output/(label+'-batch-progress.json'),dict(module_sha256=args.module_sha256,rows=rows,
               no_engine_calls=True,no_fixture_copies=True))
    print(json.dumps(dict(label=label,exit=result.returncode)),flush=True)
assert sha_file(module) == args.module_sha256
write_json(output/'batch-results.json',dict(module_sha256=args.module_sha256,rows=rows,
           no_engine_calls=True,no_fixture_copies=True))
sys.exit(0 if all(row['exit'] == 0 for row in rows) else 1)
