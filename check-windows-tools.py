"""Fail before build on unresolved or non-C workload tool locations."""
import json
import os
from pathlib import Path
import sys

paths = json.loads(os.environ['FN_TOOL_PATHS'])
paths.update({f'path_{n}': p for n, p in enumerate(os.environ['PATH'].split(os.pathsep)) if p})
for key in ('INCLUDE', 'LIB', 'LIBPATH'):
    paths.update({f'{key}_{n}': p for n, p in enumerate(os.environ.get(key, '').split(os.pathsep)) if p})
for key in ('VSINSTALLDIR', 'VCToolsInstallDir', 'WindowsSdkDir'):
    if os.environ.get(key):
        paths[key] = os.environ[key]
drives = {}
for key, raw in paths.items():
    try:
        drives[key] = Path(raw).resolve(strict=True).drive.upper() if raw else 'UNRESOLVED'
    except (OSError, ValueError):
        drives[key] = 'UNRESOLVED'
transport = {key: Path(os.environ[key]).resolve().drive.upper() for key in ('GITHUB_WORKSPACE', 'RUNNER_TEMP')}
(Path(os.environ['PUBLIC_DIR']) / 'tool-context.json').write_text(json.dumps({'workload_tool_drives': drives, 'transport_drives': transport}, indent=2) + '\n')
if not drives or any(value != 'C:' for value in drives.values()):
    raise SystemExit('unresolved or non-C build dependency; no timing')
