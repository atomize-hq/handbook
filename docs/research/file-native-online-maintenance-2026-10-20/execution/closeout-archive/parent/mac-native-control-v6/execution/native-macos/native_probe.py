"""Bounded own-child libproc probe. No global process enumeration or signals.
Machine-managed JSON receipts preserve errno independently of returned RSS.
This is a prerequisite probe, NOT a qualified workload guard.
"""
import ctypes
import datetime
import hashlib
import json
import os
from pathlib import Path
import secrets
import subprocess
import sys
import time

LIB = ctypes.CDLL('/usr/lib/libproc.dylib', use_errno=True)
LIB.proc_pidinfo.argtypes = [ctypes.c_int, ctypes.c_int, ctypes.c_uint64,
                            ctypes.c_void_p, ctypes.c_int]
LIB.proc_pid_rusage.argtypes = [ctypes.c_int, ctypes.c_int, ctypes.c_void_p]
LIB.proc_listpids.argtypes = [ctypes.c_uint32, ctypes.c_uint32,
                            ctypes.c_void_p, ctypes.c_int]

class BSD(ctypes.Structure):
    _fields_ = [(n, ctypes.c_uint32) for n in
                ('flags', 'status', 'xstatus', 'pid', 'ppid', 'uid', 'gid',
                 'ruid', 'rgid', 'svuid', 'svgid', 'reserved')]
    _fields_ += [('comm', ctypes.c_char * 16), ('name', ctypes.c_char * 32)]
    _fields_ += [(n, ctypes.c_uint32) for n in
                ('nfiles', 'pgid', 'jobc', 'tdev', 'tpgid')]
    _fields_ += [('nice', ctypes.c_int32), ('start_sec', ctypes.c_uint64),
                ('start_usec', ctypes.c_uint64)]

class Usage(ctypes.Structure):
    _fields_ = [('uuid', ctypes.c_ubyte * 16)]
    _fields_ += [(n, ctypes.c_uint64) for n in
                ('user', 'system', 'idle', 'interrupt', 'pageins', 'wired',
                 'rss', 'footprint', 'start', 'exit')]


def identity(pid):
    b = BSD()
    ctypes.set_errno(0)
    n = LIB.proc_pidinfo(pid, 3, 0, ctypes.byref(b), ctypes.sizeof(b))
    error = ctypes.get_errno()
    record = {'pid_requested': pid, 'returned_bytes': n, 'errno': error,
              'expected_bytes': ctypes.sizeof(b)}
    if n == ctypes.sizeof(b):
        record.update(pid=b.pid, ppid=b.ppid, uid=b.uid, pgid=b.pgid,
                      state=b.status, start=[b.start_sec, b.start_usec])
    return record


def usage(pid):
    b = Usage()
    ctypes.set_errno(0)
    result = LIB.proc_pid_rusage(pid, 0, ctypes.byref(b))
    error = ctypes.get_errno()
    return {'return': result, 'errno': error,
            'rss_bytes': b.rss if result == 0 else None,
            'start_abstime': b.start if result == 0 else None}


def group(pgid):
    # proc_listpids returns BYTES. proc_listpgrppids uses a different return
    # convention; the initial exploratory count/byte mistake is not reused.
    buf = (ctypes.c_int * 256)()
    ctypes.set_errno(0)
    n = LIB.proc_listpids(2, pgid, buf, ctypes.sizeof(buf))
    error = ctypes.get_errno()
    valid = n >= 0 and n % ctypes.sizeof(ctypes.c_int) == 0 and n < ctypes.sizeof(buf) and error == 0
    return {'pgid_requested': pgid, 'returned_bytes': n, 'errno': error,
            'valid': valid, 'members': list(buf)[:n // 4] if valid else None,
            'monotonic_ns': time.monotonic_ns()}


def run():
    nonce = secrets.token_hex(16)
    child = subprocess.Popen(
        [sys.executable, '-B', '-c',
         'import sys; print(sys.argv[1],flush=True); sys.stdin.buffer.read(1)', nonce],
        stdin=subprocess.PIPE, stdout=subprocess.PIPE, start_new_session=True)
    try:
        assert child.stdout.readline().decode().strip() == nonce
        own = identity(os.getpid())
        captured = identity(child.pid)
        memory = usage(child.pid)
        members = group(child.pid)
        assert captured['returned_bytes'] == 136 and captured['errno'] == 0
        assert captured['pid'] == child.pid and captured['ppid'] == os.getpid()
        assert captured['uid'] == os.getuid() and captured['pgid'] == child.pid
        assert memory['return'] == 0 and memory['rss_bytes'] > 0
        assert members['valid'] and members['members'] == [child.pid]
    finally:
        child.stdin.close()
        child.wait(timeout=3)
        child.stdout.close()
    first = group(child.pid)
    time.sleep(.05)
    second = group(child.pid)
    assert first['valid'] and second['valid']
    assert first['members'] == second['members'] == []
    return {'kind': 'own-child-libproc-prerequisite', 'status': 'PASS_PREREQUISITE_ONLY',
            'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
            'source_sha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
            'nonce': nonce, 'nonce_handshake': True, 'supervisor': own,
            'child_identity': captured, 'child_rusage': memory,
            'live_group': members, 'child_exit': child.returncode,
            'two_numeric_own_group_closure_observations': [first, second],
            'signals_sent': False, 'global_process_enumeration': False,
            'qualification': 'No guard/RSS-frequency/orphan/watchdog qualification inferred',
            'absolute_functional_cutoff_utc': '2026-10-21T11:10:14Z'}

if __name__ == '__main__':
    result = run()
    output = Path(sys.argv[1])
    with output.open('x') as f:
        json.dump(result, f, indent=2)
        f.write('\n')
    print(json.dumps(result))
