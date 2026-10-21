"""SOURCE-ONLY proposal. Scoped actual-executable proof before stdin release.
Not wired into a driver and never executed during preparation. Caller must hold
an owned stdin PIPE, keep it empty, and keep accepted role/outer samplers running.
No process enumeration, signals, engine launches, or global API changes.
"""
import ctypes
import os
from pathlib import Path
import time
from native_probe import LIB, identity, usage
import monitor_v20 as monitor

# proc_pidpath returns copied path bytes, not a process count or RSS value.
# Keep a fixed per-request buffer and capture errno separately. Scope is one
# already owned PID; there is no all-PID lookup or command-based policy choice.
_PATH=LIB.proc_pidpath
_PATH.argtypes=[ctypes.c_int,ctypes.c_void_p,ctypes.c_uint32]
_PATH.restype=ctypes.c_int


def prove_actual_exec(expected, executable, parent, deadline):
    """One positive native-exec/RSS proof or an explicit refusal; no retries.
    The caller owns deadline bounding and may retry before delivery while its
    sampler remains healthy. This function never treats unavailable RSS as zero.
    """
    if not monitor.valid_identity(expected) or not monitor.valid_identity(parent):
        raise RuntimeError('startgate-owned-identities-required')
    if time.monotonic()>=deadline:raise RuntimeError('startgate-deadline')
    binary=Path(executable).resolve(strict=True)
    before=identity(expected['pid'])
    if not monitor.same_identity(expected,before):raise RuntimeError('startgate-child-birth-unknown')
    if before['ppid']!=parent['pid'] or before['pgid']!=parent['pgid'] or before['uid']!=os.getuid():
        raise RuntimeError('startgate-child-ancestry')
    live_parent=identity(parent['pid'])
    if not monitor.same_identity(parent,live_parent):raise RuntimeError('startgate-parent-birth-unknown')
    buffer=ctypes.create_string_buffer(4096)
    ctypes.set_errno(0);n=_PATH(expected['pid'],buffer,len(buffer));error=ctypes.get_errno()
    path_row={'pid_requested':expected['pid'],'returned_bytes':n,'errno':error,'buffer_bytes':len(buffer)}
    if not 0<n<len(buffer) or error:raise RuntimeError('startgate-executable-path-unavailable')
    raw=buffer.value
    if not raw or len(raw)>n:raise RuntimeError('startgate-executable-path-framing')
    try:actual=Path(os.fsdecode(raw)).resolve(strict=True)
    except (OSError,UnicodeError) as e:raise RuntimeError('startgate-executable-path-invalid') from e
    if actual!=binary:raise RuntimeError('startgate-actual-executable-mismatch')
    memory=usage(expected['pid']);after=identity(expected['pid'])
    if not monitor.same_identity(before,after) or not monitor.same_identity(expected,after):
        raise RuntimeError('startgate-child-changed-during-proof')
    if memory['return'] or memory['rss_bytes'] is None or memory['rss_bytes']<=0:
        raise RuntimeError('startgate-positive-native-RSS-unavailable')
    if time.monotonic()>=deadline:raise RuntimeError('startgate-deadline')
    path_row['resolved_executable']=str(actual)
    return {'kind':'native-actual-exec-before-stdin','identity_before':before,'identity_after':after,
            'parent':live_parent,'path':path_row,'usage':memory,'monotonic':time.monotonic(),
            'input_delivered':False,'sampler_health_and_ancestor_token_recheck_required':True}


if __name__=='__main__':
    raise SystemExit('SOURCE_ONLY_PARENT_REVIEW_REQUIRED_NO_LIVE_PROBE')
