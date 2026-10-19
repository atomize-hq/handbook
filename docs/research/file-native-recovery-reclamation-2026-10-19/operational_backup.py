"""Quiescent cooperating full operational capture, never a selected-history proof.

One streaming transport, one independently created destination. The caller must
exclude concurrent mutators for the entire capture. No portable delete authority.
Incomplete transports have no manifest and can never report restore success.
"""
import argparse, gzip, hashlib, json, os, struct, time
from pathlib import Path, PurePosixPath

MAGIC = b'FNOP1\n'
EXCLUDED = {'writer.lock', 'preparation.lock', 'DISPOSABLE.json'}
MAX_FILES = 5_000_000
MAX_BYTES = 32 * 1024**3

def sha_file(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as f:
        for b in iter(lambda:f.read(1024*1024), b''): h.update(b)
    return h.hexdigest()

def paths(root):
    # Bounded per-directory lists; do not materialize the million-file namespace.
    def walk(directory):
        with os.scandir(directory) as entries:
            for e in sorted(entries,key=lambda e:e.name):
                if e.is_symlink(): raise ValueError('symlink')
                p=Path(e.path)
                if e.is_dir(follow_symlinks=False): yield from walk(p)
                elif e.is_file(follow_symlinks=False): yield p
                else: raise ValueError('special-file')
    yield from walk(root)

def capture(root, destination, source_pin, quiescent=False):
    if not quiescent: raise ValueError('explicit-quiescent-contract-required')
    root=Path(root).resolve(); destination=Path(destination).resolve()
    if destination.is_relative_to(root): raise ValueError('nested-capture')
    destination.mkdir(parents=True,exist_ok=False)
    selected=(root/'SELECTED.json').read_bytes(); count=total=0; start=time.monotonic()
    inventory=hashlib.sha256()
    with gzip.open(destination/'operational.bin.gz','wb',compresslevel=1) as out, (destination/'files.jsonl').open('wb') as manifest:
        out.write(MAGIC)
        for p in paths(root):
            relative=p.relative_to(root).as_posix()
            if relative in EXCLUDED: continue
            name=relative.encode(); size=p.stat().st_size
            count+=1;total+=size
            if count>MAX_FILES or total>MAX_BYTES or size>512*1024**2: raise ValueError('capture-cap')
            out.write(struct.pack('<IQ',len(name),size));out.write(name);h=hashlib.sha256();n=0
            with p.open('rb') as f:
                for b in iter(lambda:f.read(1024*1024),b''): out.write(b);h.update(b);n+=len(b)
            if n!=size: raise ValueError('changed-during-capture')
            digest=h.digest();out.write(digest)
            line=json.dumps(dict(path=relative,bytes=size,sha256=digest.hex()),separators=(',',':')).encode()+b'\n'
            manifest.write(line);inventory.update(line)
            if count%10000==0: print(json.dumps(dict(phase='capture',files=count,bytes=total,seconds=time.monotonic()-start)),flush=True)
    if selected!=(root/'SELECTED.json').read_bytes(): raise ValueError('selector-changed')
    result=dict(version=1,profile='quiescent-full-operational',source_pin=source_pin,original=str(root),selected=json.loads(selected),files=count,logical_bytes=total,
                inventory_sha256=inventory.hexdigest(),transport_sha256=sha_file(destination/'operational.bin.gz'),excluded=sorted(EXCLUDED),seconds=time.monotonic()-start)
    (destination/'manifest.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8');return result

def restore(capsule,destination):
    capsule=Path(capsule); destination=Path(destination).resolve(); start=time.monotonic()
    m=json.loads((capsule/'manifest.json').read_bytes())
    if m['version']!=1 or m['profile']!='quiescent-full-operational': raise ValueError('backup-profile')
    if sha_file(capsule/'operational.bin.gz')!=m['transport_sha256'] or sha_file(capsule/'files.jsonl')!=m['inventory_sha256']: raise ValueError('transport-digest')
    if destination.exists() or destination==Path(m['original']).resolve(): raise ValueError('new-destination-required')
    destination.mkdir(parents=True,exist_ok=False); count=total=0
    def take(f,n):
        b=f.read(n)
        if len(b)!=n:raise ValueError('truncated-transport')
        return b
    with gzip.open(capsule/'operational.bin.gz','rb') as source,(capsule/'files.jsonl').open('rb') as inventory:
        if take(source,len(MAGIC))!=MAGIC:raise ValueError('magic')
        for line in inventory:
            row=json.loads(line); n,size=struct.unpack('<IQ',take(source,12))
            if n>4096 or size>512*1024**2:raise ValueError('record-cap')
            name=take(source,n).decode();p=PurePosixPath(name)
            if name!=row['path'] or size!=row['bytes'] or n>4096 or p.is_absolute() or '..' in p.parts or '\\' in name or ':' in name or name in EXCLUDED:raise ValueError('restore-path-binding')
            target=destination/name;target.parent.mkdir(parents=True,exist_ok=True);h=hashlib.sha256();remaining=size
            with target.open('xb') as out:
                while remaining:
                    b=take(source,min(1024*1024,remaining));out.write(b);h.update(b);remaining-=len(b)
            if h.digest()!=take(source,32) or h.hexdigest()!=row['sha256']:raise ValueError('file-digest')
            count+=1;total+=size
            if count>MAX_FILES or total>MAX_BYTES:raise ValueError('restore-cap')
            if count%10000==0:print(json.dumps(dict(phase='restore',files=count,bytes=total,seconds=time.monotonic()-start)),flush=True)
        if source.read(1):raise ValueError('extra-transport')
    if count!=m['files'] or total!=m['logical_bytes'] or json.loads((destination/'SELECTED.json').read_bytes())!=m['selected']:raise ValueError('restore-completeness')
    return dict(status='bytes-restored-operational-reopening-still-required',files=count,logical_bytes=total,seconds=time.monotonic()-start,destination=str(destination),disposable_authority_imported=False)

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('action',choices=['capture','restore']);a.add_argument('source');a.add_argument('destination');a.add_argument('--source-pin');a.add_argument('--quiescent',action='store_true');ns=a.parse_args()
    print(json.dumps(capture(ns.source,ns.destination,ns.source_pin,ns.quiescent) if ns.action=='capture' else restore(ns.source,ns.destination),indent=2))
