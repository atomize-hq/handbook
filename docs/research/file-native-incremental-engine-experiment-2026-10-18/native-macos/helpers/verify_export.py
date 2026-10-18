"""Portable read-only verification of copied native evidence (no Mac binaries needed)."""
import argparse
import base64
import gzip
import hashlib
import json
from pathlib import Path, PurePosixPath
import tarfile
import unicodedata
import zipfile

p=argparse.ArgumentParser();p.add_argument('root',type=Path);a=p.parse_args();root=a.root
def sha(data):return hashlib.sha256(data).hexdigest()
def load(path):return json.loads(path.read_bytes())
def safe(names):
    seen=set()
    for name in names:
        path=PurePosixPath(name);assert not path.is_absolute() and '..' not in path.parts and '\\' not in name
        assert all(not part.endswith((' ','.')) and not any(c in part for c in '<>:"|?*') for part in path.parts)
        key=unicodedata.normalize('NFC',name).casefold();assert key not in seen,('path-collision',name);seen.add(key)

counts={};zip_count=tar_count=0
for indexpath in sorted(root.rglob('raw-index.json')):
    lane=indexpath.parent;count=censors=0
    binaries=load(lane/'outer--binary-pins.json');allowed={v['sha256'] for v in binaries.values()}
    pins=load(lane/'outer--source-pins.json')
    source_files={name.removeprefix('docs/research/file-native-incremental-engine-experiment-2026-10-18/'):v['sha256'] for name,v in pins['files'].items()}
    response_rows=load(lane/'response-bindings.json');responses={(r['file'],r['line']):r for r in response_rows};assert len(responses)==len(response_rows)
    for item in load(indexpath):
        archive=(lane/item['file']).read_bytes();assert sha(archive)==item['compressed_sha256']
        raw=gzip.decompress(archive);assert sha(raw)==item['sha256'] and len(raw)==item['bytes']
        rows=[json.loads(line) for line in raw.splitlines()];assert len(rows)==item['records']
        prefix=item['file'].removesuffix('--events.jsonl.gz')
        session=load(lane/(prefix+'--session.json'));binding=load(lane/(prefix+'--outer-binding.json'))
        assert session['sha256']==binding['executable_sha256'] and session['sha256'] in allowed
        assert session['source_hashes']==source_files and session['harness_sha256']==source_files['controller.py']
        sourcezip=lane/(prefix+'--session-source.zip');assert sha(sourcezip.read_bytes())==session['source_archive_sha256']==binding['source_archive_sha256']
        with zipfile.ZipFile(sourcezip) as z:
            safe(z.namelist());assert set(z.namelist())==set(session['source_hashes'])
            for name,digest in session['source_hashes'].items():assert sha(z.read(name))==digest
        zip_count+=1
        for line,row in enumerate(rows,1):
            out=base64.b64decode(row['stdout_b64'],validate=True);err=base64.b64decode(row['stderr_b64'],validate=True)
            assert sha(out)==row['stdout_sha256'] and len(out)==row['stdout_bytes']
            response=responses[item['file'],line]
            assert response['stderr_sha256']==sha(err) and response['stderr_bytes']==len(err)
            assert response['stdout_sha256']==sha(out) and response['stdout_bytes']==len(out) and response['exe_sha256']==row['exe_sha256']
            parsed=None;error=None
            try:parsed=json.loads(out)
            except Exception as exc:error=str(exc)
            assert json.dumps(parsed,sort_keys=True,allow_nan=False)==json.dumps(row['output'],sort_keys=True,allow_nan=False)
            assert error==row['parse_error'] and row['exe_sha256']==session['sha256']
            count+=1;censors+=bool(row['censored'])
    for path in lane.glob('outer--source-capsule.tar'):
        pins=load(lane/'outer--source-pins.json');assert sha(path.read_bytes())==pins['archive_sha256']
        with tarfile.open(path) as t:
            members=t.getmembers();safe([m.name for m in members]);assert all(m.isfile() or m.isdir() for m in members)
            files={m.name:sha(t.extractfile(m).read()) for m in members if m.isfile()}
            assert files=={name:v['sha256'] for name,v in pins['files'].items()}
        tar_count+=1
    expected=load(lane/'verification.json');assert expected['events']==count and expected['censors']==censors
    counts[str(lane.relative_to(root))]=dict(events=count,censors=censors)
safe([str(path.relative_to(root)) for path in root.rglob('*') if path.is_file() and '__pycache__' not in path.parts])
print(json.dumps(dict(status='passed',lanes=counts,session_archives=zip_count,git_source_archives=tar_count,
    qualification='Archive/response/source/session/binary-hash association verification; physical executable bytes remain on Mac and their recorded hash is not a reproducible-build claim.'),indent=2))
