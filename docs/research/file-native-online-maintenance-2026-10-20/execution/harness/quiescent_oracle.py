"""Full independent oracle with complete file and directory read-only proof."""
import argparse
import importlib
import json
import os
from pathlib import Path
import sys
from job_guard import sha_file, write_json

parser = argparse.ArgumentParser()
parser.add_argument('--root', required=True)
parser.add_argument('--module', required=True)
parser.add_argument('--module-sha256', required=True)
parser.add_argument('--output', required=True)
parser.add_argument('--known-requests')
args = parser.parse_args()
assert sys.dont_write_bytecode and os.environ.get('FILE_NATIVE_STAGE_JOB') == '1'
root = Path(args.root).resolve()
module_path = Path(args.module).resolve()
assert sha_file(module_path) == args.module_sha256

def snapshot():
    files = {}
    directories = []
    stack = [root]
    while stack:
        directory = stack.pop()
        assert not directory.lstat().st_file_attributes & 1024, 'reparse-point-refused'
        directories.append(directory.relative_to(root).as_posix())
        for entry in directory.iterdir():
            assert not entry.lstat().st_file_attributes & 1024, 'reparse-point-refused'
            if entry.is_dir():
                stack.append(entry)
            else:
                assert entry.is_file()
                files[entry.relative_to(root).as_posix()] = dict(bytes=entry.stat().st_size,sha256=sha_file(entry))
    return dict(files=files,directories=sorted(directories))

before = snapshot()
output = Path(args.output)
write_json(output.with_suffix('.before.json'), before)
sys.path.insert(0, str(module_path.parent))
module = importlib.import_module(module_path.stem)
assert Path(module.__file__).resolve() == module_path
known = json.loads(Path(args.known_requests).read_bytes()) if args.known_requests else None
answer = module.verify(root, output, quiescent=True, known_requests=known)
after = snapshot()
write_json(output.with_suffix('.after.json'), after)
assert before == after and sha_file(module_path) == args.module_sha256
write_json(output.with_suffix('.read-only.json'), dict(root=str(root),module=str(module_path),
           module_sha256=args.module_sha256,source_file_count=len(before['files']),
           source_logical_bytes=sum(x['bytes'] for x in before['files'].values()),
           source_directory_count=len(before['directories']),
           all_file_hashes_sizes_names_and_directories_unchanged=True,
           before_sha256=sha_file(output.with_suffix('.before.json')),
           after_sha256=sha_file(output.with_suffix('.after.json')),
           no_engine_calls=True,no_fixture_copies=True))
print(json.dumps(dict(status='passed',actual_H=answer['semantic']['publications'],
                     records=answer['raw_records'],source_files=len(before['files']),
                     source_directories=len(before['directories']),source_unchanged=True)))
