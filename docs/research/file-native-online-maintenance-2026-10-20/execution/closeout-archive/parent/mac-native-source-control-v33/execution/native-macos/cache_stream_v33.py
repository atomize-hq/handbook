"""SOURCE-ONLY lower-level streaming-copy proposal; not an activation route.
The future parent-reviewed caller must bind final pin/source/manifest/guard/C1
claim and explicit copy GO before calling. No cache directories are created by
import or the CLI. Synthetic controls will use F08 destinations, never real C1.
"""
import hashlib
import math
import os
import stat
from pathlib import Path
import time
import admission_v20 as admission

CHUNK=65536


def stream_member(source,destination,*,expected_bytes,expected_sha256,
                  operation_ceiling,deadline,synthetic=False):
    """Caller serializes copies and owns release/binding; partial files retained.
    Reserve the complete new file before mkdir/open. No replacement/negative
    credit, no whole-store copy, no >64KiB read or write, no implicit cap growth.
    """
    if type(deadline) not in (int,float) or not math.isfinite(deadline):raise ValueError('stream-copy-finite-deadline')
    category='F08' if synthetic else 'C1'
    base=admission.ROOT/('fixtures' if synthetic else 'cargo-private')
    source=Path(source);destination=Path(destination)
    if type(expected_bytes) is not int or expected_bytes<0 or type(operation_ceiling) is not int or expected_bytes>operation_ceiling:
        raise ValueError('stream-copy-operation-prewrite-ceiling')
    if not isinstance(expected_sha256,str) or len(expected_sha256)!=64 or any(c not in '0123456789abcdef' for c in expected_sha256):
        raise ValueError('stream-copy-expected-hash')
    if not destination.is_absolute() or not destination.is_relative_to(base) or '..' in destination.parts:
        raise ValueError('stream-copy-destination')
    if source.is_symlink() or not source.is_file() or destination.exists() or destination.is_symlink():
        raise ValueError('stream-copy-regular-fresh-file-required')
    for p in [base,*destination.parents]:
        if p.is_relative_to(admission.ROOT) and p.is_symlink():raise ValueError('stream-copy-parent-symlink')
    observed=source.stat();identity=(observed.st_dev,observed.st_ino,observed.st_size,observed.st_mtime_ns)
    if observed.st_size!=expected_bytes:raise ValueError('stream-copy-source-size')
    if time.monotonic()>=deadline:raise RuntimeError('stream-copy-deadline')
    request={k:{'bytes':0,'files':0} for k in admission.LIMITS};request[category]={'bytes':expected_bytes,'files':1}
    admission.admit(admission.snapshot(shared=False),request,admission.limits())
    destination.parent.mkdir(parents=True,exist_ok=True)
    count=0;h=hashlib.sha256()
    descriptor=os.open(source,os.O_RDONLY|os.O_NOFOLLOW|os.O_NONBLOCK)
    # Enter source ownership first. Any destination-open failure unwinds this
    # context and closes the source FD before propagating the refusal.
    with os.fdopen(descriptor,'rb') as inp:
      with destination.open('xb') as out:
        fstat=os.fstat(inp.fileno())
        if not stat.S_ISREG(fstat.st_mode):raise RuntimeError('stream-copy-source-file-type')
        if (fstat.st_dev,fstat.st_ino,fstat.st_size,fstat.st_mtime_ns)!=identity:raise RuntimeError('stream-copy-source-open-drift')
        while True:
            if time.monotonic()>=deadline:raise RuntimeError('stream-copy-deadline')
            block=inp.read(min(CHUNK,expected_bytes-count+1))
            if not block:break
            if count+len(block)>expected_bytes or count+len(block)>operation_ceiling:raise RuntimeError('stream-copy-input-exceeded-bound')
            out.write(block);h.update(block);count+=len(block)
        out.flush();os.fsync(out.fileno())
        after=os.fstat(inp.fileno())
        if (after.st_dev,after.st_ino,after.st_size,after.st_mtime_ns)!=identity:raise RuntimeError('stream-copy-source-final-drift')
    if count!=expected_bytes or h.hexdigest()!=expected_sha256:raise RuntimeError('stream-copy-exact-manifest-mismatch-partial-retained')
    return {'source':str(source),'destination':str(destination),'bytes':count,'sha256':h.hexdigest(),
            'chunk_ceiling':CHUNK,'category':category,'synthetic':synthetic}


if __name__=='__main__':
    raise SystemExit('SOURCE_ONLY_PARENT_FINAL_PIN_GUARD_BINDING_AND_COPY_GO_REQUIRED')
