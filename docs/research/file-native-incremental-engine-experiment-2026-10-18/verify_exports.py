"""Independent exported-byte, response-type, session and source-archive verifier."""
import argparse,base64,gzip,hashlib,io,json,zipfile
from pathlib import Path
from pack_evidence import typed
def sha(b):return hashlib.sha256(b).hexdigest()
def verify(root):
    rows=[]
    for path in sorted(root.rglob('manifest.json')):
        manifest=json.loads(path.read_bytes())
        if manifest.get('lossless_bytes_verified') is not True:continue
        files={}
        for entry in manifest['files']:
            compressed=(path.parent/entry['archive']).read_bytes();assert sha(compressed)==entry['archive_sha256'] and len(compressed)==entry['archive_bytes']
            raw=gzip.decompress(compressed);assert sha(raw)==entry['sha256'] and len(raw)==entry['bytes'];files[entry['path']]=raw
        sessions={str(Path(name).parent).replace('\\','/'):json.loads(raw) for name,raw in files.items() if name.endswith('session.json')}
        source_archives=0;missing_source_archives=[]
        for parent,session in sessions.items():
            if 'source_archive_sha256' not in session:missing_source_archives.append(parent);continue
            key='session-source.zip' if parent=='.' else parent+'/session-source.zip';raw=files[key]
            assert sha(raw)==session['source_archive_sha256']
            with zipfile.ZipFile(io.BytesIO(raw)) as z:
                assert {n:sha(z.read(n)) for n in z.namelist()}==session['source_hashes']
            source_archives+=1
        count=0;censors=0;exits={}
        for name,raw in files.items():
            if not name.endswith('events.jsonl'):continue
            session=sessions[str(Path(name).parent).replace('\\','/')]
            for line in raw.splitlines():
                row=json.loads(line);out=base64.b64decode(row['stdout_b64'],validate=True)
                assert sha(out)==row['stdout_sha256'] and len(out)==row['stdout_bytes'];assert row['exe_sha256']==session['sha256']
                if row['output'] is not None:typed(json.loads(out),row['output'])
                assert type(row['exit']) is int and type(row['censored']) is bool
                count+=1;censors+=row['censored'];exits[str(row['exit'])]=exits.get(str(row['exit']),0)+1
        assert count==manifest['events']
        rows.append(dict(manifest=path.relative_to(root).as_posix(),files=len(files),events=count,censors=censors,exits=exits,verified_source_archives=source_archives,sessions_without_compiled_source_archive=missing_source_archives))
    return dict(verified=True,manifests=rows,qualification='Checks exported lossless evidence manifests; native Mac uses its separate portable verifier, and history proof capsules use pack_history verification. Missing early-development source archives remain unavailable, not reconstructed.')
if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--root',required=True);a.add_argument('--output',required=True);ns=a.parse_args();Path(ns.output).write_text(json.dumps(verify(Path(ns.root)),indent=2))
