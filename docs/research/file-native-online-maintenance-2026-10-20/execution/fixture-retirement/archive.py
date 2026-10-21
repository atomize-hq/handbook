"""FNFXAR01 forensic capture. Parent permits and live Context checks are mandatory.
Native Jobs, budgets and writer quiescence are caller-owned. POSIX deletion uses
a cooperating namespace freeze; it is not an atomic expected-inode unlink.
"""
import base64
import bisect
import contextlib
import ctypes
import gzip
import hashlib
import json
import math
import os
from pathlib import Path
import stat
import struct
import sys
import time
import zlib

MAGIC = b'FNFXAR01'
CHUNK = 65536
FRAME = struct.Struct('<BIQ')
DATA, HEAD, DIR, FILE, FILE_END, SPECIAL, DIR_END, END = range(8)
ARCHIVE_CAP = 67108864
SIDECARS = {'archive-proof.json': 131072, 'retirement-permit.json': 131072,
            'retirement.journal': 655360, 'retirement-result.json': 131072}
NODE_KEYS = {'root', 'parts', 'identity', 'kind', 'mode', 'size', 'links',
             'mtime', 'ctime', 'attributes', 'allocated', 'device', 'mount_identity',
             'protected_scope_sha256'}
KINDS = {'dir', 'file', 'symlink', 'reparse', 'fifo', 'socket', 'char', 'block'}


class NoGo(RuntimeError):
    """A refusal never authorizes fallback, deletion, retry or a larger cap."""


def need(value, code):
    if not value:
        raise NoGo(code)


def encode(value):
    return json.dumps(value, sort_keys=True, ensure_ascii=True,
                      separators=(',', ':'), allow_nan=False).encode('ascii')


def decode(raw):
    def pairs(items):
        result = {}
        for key, value in items:
            need(key not in result, 'duplicate-json-key')
            result[key] = value
        return result
    def constant(_):
        raise NoGo('nonfinite-json')
    try:
        return json.loads(raw, object_pairs_hook=pairs, parse_constant=constant)
    except (ValueError, UnicodeError) as error:
        raise NoGo('invalid-json') from error


def small_file(path, cap):
    raw, _ = reference_read(path, cap=cap)
    return raw


def sha_file(path):
    _, digest = reference_read(path)
    return digest


@contextlib.contextmanager
def reference_handle(path):
    """Pin every ancestor and the final regular file without following links."""
    path = Path(absolute(str(path)))
    need(len(path.parts) <= 128, 'authority-path-depth')
    handles = []
    try:
        if os.name == 'nt':
            current = Path(path.anchor)
            for component in [None, *path.parts[1:-1]]:
                if component is not None:
                    current /= component
                handle = windows().open(current)
                handles.append(handle)
                basic = windows().query(handle, 0, 40)
                standard = windows().query(handle, 1, 24)
                attrs = struct.unpack_from('<I', basic, 32)[0]
                need(standard[21] and not attrs & 0x400, 'linked-authority-ancestor')
            handle = windows().open(path)
            handles.append(handle)
            before = windows().meta(handle, os.lstat(path))
            need(before['kind'] == 'file', 'nonregular-authority')
        else:
            handle = os.open(path.anchor, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
            handles.append(handle)
            for component in path.parts[1:-1]:
                handle = os.open(component, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW,
                                 dir_fd=handle)
                handles.append(handle)
            handle = os.open(path.name, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK,
                             dir_fd=handle)
            handles.append(handle)
            before = reference_stat(os.fstat(handle))
            need(stat.S_ISREG(before[2]), 'nonregular-authority')
        yield handle, before
        after = windows().meta(handle, os.lstat(path)) if os.name == 'nt' \
            else reference_stat(os.fstat(handle))
        need(after == before, 'authority-mutated-during-read')
    finally:
        for handle in reversed(handles):
            windows().close(handle) if os.name == 'nt' else os.close(handle)


def reference_stat(info):
    return (info.st_dev, info.st_ino, info.st_mode, info.st_size,
            info.st_mtime_ns, info.st_ctime_ns, info.st_nlink)


def reference_read(path, cap=None, poll=None):
    """A digest and optional decode buffer from ONE pinned regular-file handle."""
    digest, chunks, size = hashlib.sha256(), [], 0
    with reference_handle(path) as (handle, _):
        while True:
            if poll:
                poll()
            amount = CHUNK if cap is None else min(CHUNK, cap + 1 - size)
            need(amount > 0, 'metadata-byte-cap')
            block = windows().read(handle, amount) if os.name == 'nt' else os.read(handle, amount)
            if not block:
                break
            digest.update(block)
            size += len(block)
            if cap is not None:
                need(size <= cap, 'metadata-byte-cap')
                chunks.append(block)
    return b''.join(chunks) if cap is not None else None, digest.hexdigest()


def absolute(path):
    need(isinstance(path, str) and os.path.isabs(path), 'absolute-path-required')
    need(os.path.abspath(path) == path, 'noncanonical-absolute-path')
    return path


def overlaps(a, b):
    a, b = os.path.normcase(a), os.path.normcase(b)
    try:
        return os.path.commonpath([a, b]) in [a, b]
    except ValueError:
        return False


def ancestors(path):
    item = Path(path)
    for parent in reversed([item, *item.parents]):
        info = os.lstat(parent)
        need(stat.S_ISDIR(info.st_mode) and not stat.S_ISLNK(info.st_mode)
             and not getattr(info, 'st_file_attributes', 0) & 0x400,
             'linked-or-nondirectory-ancestor')


def raw_name(name):
    return name.encode('utf-16-le', 'surrogatepass') if os.name == 'nt' else os.fsencode(name)


def pack_name(name):
    return base64.b64encode(raw_name(name)).decode('ascii')


def name_bytes(token, windows_host):
    """Validate the captured namespace, never the inspector's filesystem codec."""
    try:
        raw = base64.b64decode(token, validate=True)
    except (ValueError, TypeError) as error:
        raise NoGo('invalid-native-name') from error
    need(base64.b64encode(raw).decode('ascii') == token and 0 < len(raw) <= 4096,
         'noncanonical-or-oversize-name')
    if windows_host:
        try:
            value = raw.decode('utf-16-le', 'surrogatepass')
        except UnicodeError as error:
            raise NoGo('invalid-utf16-name') from error
        need(value not in ['.', '..'] and '\x00' not in value
             and '/' not in value and '\\' not in value, 'unsafe-name')
    else:
        need(raw not in [b'.', b'..'] and b'\x00' not in raw and b'/' not in raw,
             'unsafe-name')
    return raw


def display_name(token, windows_host):
    raw = name_bytes(token, windows_host)
    return raw.decode('utf-16-le', 'surrogatepass') if windows_host \
        else raw.decode('utf-8', 'surrogateescape')


def unpack_name(token, windows_host):
    """Only matching-host operations may reconstruct a native path component."""
    need(windows_host == (os.name == 'nt'), 'foreign-native-path-reconstruction')
    raw = name_bytes(token, windows_host)
    return raw.decode('utf-16-le', 'surrogatepass') if windows_host else os.fsdecode(raw)


def is_sha(value):
    return isinstance(value, str) and len(value) == 64 and all(c in '0123456789abcdef' for c in value)


def integer(value, low, high):
    return type(value) is int and low <= value <= high


def load_plan(path, expected_sha, operation):
    raw = small_file(path, 131072)
    need(hashlib.sha256(raw).hexdigest() == expected_sha, 'parent-plan-sha')
    plan = decode(raw)
    keys = {'version', 'operation', 'host', 'source_pin', 'tool_sha256', 'guard_sha256',
            'roots', 'protected_roots', 'artifact_root', 'limits', 'evidence', 'native_scope_sha256',
            'new_final_fixture', 'namespace_freeze_nonce', 'control_only', 'archive_sha256'}
    need(isinstance(plan, dict) and set(plan) == keys and type(plan['version']) is int and plan['version'] == 1
         and plan['operation'] == operation, 'plan-shape-or-operation')
    need(plan['host'] == sys.platform and type(plan['control_only']) is bool
         and plan['new_final_fixture'] is True and isinstance(plan['namespace_freeze_nonce'], str)
         and 0 < len(plan['namespace_freeze_nonce']) <= 128, 'plan-owner')
    need(isinstance(plan['source_pin'], str) and len(plan['source_pin']) == 40
         and all(c in '0123456789abcdef' for c in plan['source_pin']), 'source-pin')
    need(plan['tool_sha256'] == sha_file(__file__) and is_sha(plan['guard_sha256']) and is_sha(plan['native_scope_sha256']),
         'executed-tool-binding')
    limits = plan['limits']
    need(set(limits) == {'logical_bytes', 'entries', 'stream_bytes', 'depth',
                         'deadline_epoch', 'archive_bytes'}, 'limits-shape')
    for key, maximum in [('logical_bytes', 8 * 1024**3), ('entries', 5000000),
                         ('stream_bytes', 16 * 1024**3), ('depth', 64),
                         ('archive_bytes', ARCHIVE_CAP)]:
        need(integer(limits[key], 1, maximum), 'invalid-limit:' + key)
    need(type(limits['deadline_epoch']) in [int, float] and math.isfinite(limits['deadline_epoch'])
         and time.time() < limits['deadline_epoch'] <= 1792581014, 'phase-deadline')
    roots = plan['roots']
    need(isinstance(roots, list) and 0 < len(roots) <= 64, 'root-count')
    paths = []
    for root in roots:
        need(set(root) == {'path', 'identity'} and isinstance(root['identity'], str), 'root-shape')
        path = absolute(root['path'])
        need(not any(overlaps(path, other) for other in paths), 'overlapping-roots')
        ancestors(path)
        paths.append(path)
    protected = [absolute(p) for p in plan['protected_roots']]
    need(protected, 'protected-root-list-required')
    protected.append(str(Path(__file__).absolute().parent))
    need(not any(overlaps(a, b) for a in paths for b in protected), 'protected-root')
    artifact = absolute(plan['artifact_root'])
    need(not any(overlaps(artifact, p) for p in paths), 'archive-inside-source')
    need(not any(overlaps(artifact, p) for p in protected), 'archive-in-protected-root')
    ancestors(str(Path(artifact).parent))
    need(operation == 'ARCHIVE' and plan['archive_sha256'] is None or operation == 'RETIRE'
         and is_sha(plan['archive_sha256']), 'archive-sha-binding')
    plan['_sha'] = expected_sha
    return plan


def check(plan, context, phase, reserve=0):
    need(context is not None and hasattr(context, 'check'), 'NO_GO:no-reviewed-context')
    need(time.time() < plan['limits']['deadline_epoch'], 'phase-deadline')
    proof = context.check(phase=phase, plan_sha256=plan['_sha'],
                          prospective_bytes=reserve, roots=[dict(r) for r in plan['roots']])
    keys = {'go', 'epoch', 'phase', 'source_pin', 'host', 'freeze_nonce', 'guard_sha256',
            'prior_groups_empty', 'io_closed', 'handles_closed', 'namespace_frozen',
            'references_frozen', 'archive_protected', 'native_delete_reviewed'}
    need(isinstance(proof, dict) and set(proof) == keys, 'live-proof-shape')
    need(proof['go'] is True and proof['phase'] == phase
         and proof['source_pin'] == plan['source_pin'] and proof['host'] == plan['host']
         and proof['freeze_nonce'] == plan['namespace_freeze_nonce']
         and proof['guard_sha256'] == plan['guard_sha256'], 'live-proof-binding')
    need(type(proof['epoch']) in [int, float] and 0 <= time.time() - proof['epoch'] <= 1,
         'stale-live-proof')
    for key in ['prior_groups_empty', 'io_closed', 'handles_closed', 'namespace_frozen', 'references_frozen']:
        need(proof[key] is True, 'missing-quiescence:' + key)
    if phase == 'retire':
        need(proof['archive_protected'] is True and proof['native_delete_reviewed'] is True,
             'NO_GO:retirement-native-capability')


def scope(plan, context, node, root_index):
    """Mandatory reviewed native mount/protected-identity boundary, not st_dev."""
    need(hasattr(context, 'native_scope'), 'NO_GO:native-mount-scope-proof-required')
    root = plan['roots'][root_index]
    parent_path = node.parent.path if node.parent else str(Path(node.path).parent)
    proof = context.native_scope(path=node.path, handle=node.handle,
        identity=node.meta['identity'], parent_path=parent_path,
        parent_handle=node.parent.handle if node.parent else None,
        root=dict(root), protected_roots=list(plan['protected_roots']), plan_sha256=plan['_sha'])
    keys = {'go', 'epoch', 'backend', 'namespace', 'mount', 'parent_mount', 'root_mount',
            'root_identity', 'node_identity', 'parent_identity', 'protected_roots_sha256',
            'protected_scope_sha256', 'source_pin', 'guard_sha256', 'freeze_nonce',
            'no_protected_alias', 'provider_sha256'}
    need(isinstance(proof, dict) and set(proof) == keys, 'native-scope-proof-shape')
    backends = {'linux': 'linux-statx-mnt-id', 'darwin': 'darwin-native-mount-identity',
                'win32': 'win32-pinned-volume-reparse'}
    need(plan['host'] in backends and proof['go'] is True
         and proof['backend'] == backends[plan['host']]
         and proof['no_protected_alias'] is True
         and proof['provider_sha256'] == plan['native_scope_sha256'], 'native-scope-refusal')
    need(type(proof['epoch']) in [int, float] and 0 <= time.time() - proof['epoch'] <= 1,
         'stale-native-scope-proof')
    for key in ['namespace', 'mount', 'parent_mount', 'root_mount', 'parent_identity']:
        need(isinstance(proof[key], str) and 0 < len(proof[key]) <= 256, 'native-scope-id')
    need(proof['root_identity'] == root['identity']
         and proof['node_identity'] == node.meta['identity']
         and proof['source_pin'] == plan['source_pin']
         and proof['guard_sha256'] == plan['guard_sha256']
         and proof['freeze_nonce'] == plan['namespace_freeze_nonce'], 'native-scope-binding')
    if node.parent:
        need(proof['parent_identity'] == node.parent.meta['identity'], 'native-parent-scope-binding')
    if node.path == root['path']:
        # An admitted root itself may not be a bind/mount alias either.
        need(proof['mount'] == proof['parent_mount'], 'mounted-root-refusal')
    need(proof['mount'] == proof['root_mount'] == proof['parent_mount'],
         'mounted-descendant-or-file-refusal')
    protected = hashlib.sha256(encode(plan['protected_roots'])).hexdigest()
    need(proof['protected_roots_sha256'] == protected
         and proof['protected_scope_sha256'] in {r['sha256'] for r in plan['evidence']['quiescence']},
         'protected-native-identity-proof')
    mount = {key: proof[key] for key in ['backend', 'namespace', 'mount', 'parent_mount', 'root_mount']}
    return dict(mount_identity=mount,
                protected_scope_sha256=proof['protected_scope_sha256'])


def evidence(plan, context):
    refs = plan['evidence']
    need(set(refs) == {'host_review', 'case_ledger', 'oracles', 'negatives', 'quiescence'},
         'evidence-shape')
    host_raw = None
    for key, group in refs.items():
        need(isinstance(group, list) and group, 'missing-evidence:' + key)
        for ref in group:
            need(isinstance(ref, dict) and set(ref) == {'path', 'sha256'}
                 and is_sha(ref['sha256']), 'evidence-reference-shape')
            raw, digest = reference_read(absolute(ref['path']),
                cap=131072 if key == 'host_review' else None,
                poll=lambda: check(plan, context, 'capture'))
            need(digest == ref['sha256'], 'evidence-sha')
            if key == 'host_review':
                host_raw = raw
    need(len(refs['host_review']) == 1 and host_raw is not None, 'host-review-count')
    # No second pathname open: these are exactly the bytes whose digest matched.
    review = decode(host_raw)
    keys = {'version', 'host', 'source_pin', 'required_case_ids', 'completed_case_ids',
            'complete', 'full_oracles', 'negative_receipts_complete', 'positive_native_closure',
            'new_final_fixture', 'control_only'}
    need(set(review) == keys and type(review['version']) is int and review['version'] == 1 and review['host'] == plan['host']
         and review['source_pin'] == plan['source_pin']
         and review['control_only'] is plan['control_only'], 'host-review-binding')
    ids = review['required_case_ids']
    need(isinstance(ids, list) and ids and len(ids) <= 4096
         and all(isinstance(x, str) for x in ids) and ids == sorted(set(ids))
         and ids == review['completed_case_ids'], 'incomplete-case-ledger')
    for key in ['complete', 'full_oracles', 'negative_receipts_complete',
                'positive_native_closure', 'new_final_fixture']:
        need(review[key] is True, 'host-incomplete:' + key)


class Windows:
    """Lazy native boundary; OPEN_REPARSE_POINT and same-handle stream/ID checks."""
    def __init__(self):
        from ctypes import wintypes as w
        self.w = w
        self.k = ctypes.WinDLL('kernel32', use_last_error=True)
        k = self.k
        k.CreateFileW.argtypes = [w.LPCWSTR, w.DWORD, w.DWORD, ctypes.c_void_p,
                                  w.DWORD, w.DWORD, w.HANDLE]
        k.CreateFileW.restype = w.HANDLE
        k.CloseHandle.argtypes = [w.HANDLE]
        k.GetFileInformationByHandleEx.argtypes = [w.HANDLE, ctypes.c_int, ctypes.c_void_p, w.DWORD]
        k.SetFileInformationByHandle.argtypes = [w.HANDLE, ctypes.c_int, ctypes.c_void_p, w.DWORD]
        k.ReadFile.argtypes = [w.HANDLE, ctypes.c_void_p, w.DWORD,
                              ctypes.POINTER(w.DWORD), ctypes.c_void_p]
        k.DeviceIoControl.argtypes = [w.HANDLE, w.DWORD, ctypes.c_void_p, w.DWORD,
                                     ctypes.c_void_p, w.DWORD, ctypes.POINTER(w.DWORD), ctypes.c_void_p]

    def open(self, path, delete=False):
        handle = self.k.CreateFileW(str(path), 0x80000080 | (0x10000 if delete else 0),
                                    1, None, 3, 0x02200000, None)
        need(handle not in [None, ctypes.c_void_p(-1).value], 'native-open-or-sharing-refusal')
        return handle

    def query(self, handle, kind, size):
        raw = ctypes.create_string_buffer(size)
        need(self.k.GetFileInformationByHandleEx(handle, kind, raw, size), 'native-info:' + str(kind))
        return raw.raw

    def meta(self, handle, fallback):
        identity = self.query(handle, 18, 24)
        basic = self.query(handle, 0, 40)
        standard = self.query(handle, 1, 24)
        streams = self.query(handle, 7, CHUNK)
        offset = 0
        while True:
            nxt, length, _, _ = struct.unpack_from('<IIqq', streams, offset)
            need(length % 2 == 0 and offset + 24 + length <= CHUNK, 'native-stream-shape')
            name = streams[offset + 24:offset + 24 + length].decode('utf-16-le')
            need(name in ['', '::$DATA'], 'unsupported-ADS-preserve-source')
            if not nxt:
                break
            need(nxt >= 24 + length and offset + nxt < CHUNK, 'native-stream-offset')
            offset += nxt
        attrs = struct.unpack_from('<I', basic, 32)[0]
        size = struct.unpack_from('<q', standard, 8)[0]
        kind = 'reparse' if attrs & 0x400 else ('dir' if standard[21] else 'file')
        need(not standard[20], 'delete-pending-source')
        return dict(identity='win:' + identity.hex(), kind=kind, mode=fallback.st_mode,
                    size=size, links=struct.unpack_from('<I', standard, 16)[0],
                    mtime=struct.unpack_from('<q', basic, 16)[0],
                    ctime=struct.unpack_from('<q', basic, 24)[0], attributes=attrs,
                    allocated=struct.unpack_from('<q', standard)[0], device=identity[:8].hex())

    def special(self, handle, meta):
        need(meta['size'] == 0, 'unsupported-reparse-default-stream')
        data = ctypes.create_string_buffer(16384)
        size = self.w.DWORD()
        need(self.k.DeviceIoControl(handle, 0x900A8, None, 0, data, len(data),
                                   ctypes.byref(size), None), 'reparse-metadata-unreadable')
        need(8 <= size.value <= 16384, 'reparse-metadata-length')
        return data.raw[:size.value]

    def read(self, handle, amount=CHUNK):
        data = ctypes.create_string_buffer(amount)
        size = self.w.DWORD()
        need(self.k.ReadFile(handle, data, amount, ctypes.byref(size), None), 'native-read')
        return data.raw[:size.value]

    def close(self, handle):
        need(self.k.CloseHandle(handle), 'native-close-handle')

    def unlink(self, handle):
        flag = ctypes.c_ubyte(1)
        need(self.k.SetFileInformationByHandle(handle, 4, ctypes.byref(flag), 1),
             'native-disposition-refusal')


_WIN = None


def windows():
    global _WIN
    if _WIN is None:
        _WIN = Windows()
    return _WIN


def no_xattrs(path, handle=None):
    if os.name == 'nt':
        return  # Same-handle FileStreamInfo checks ADS without following reparses.
    if hasattr(os, 'listxattr'):
        values = os.listxattr(handle) if handle is not None else os.listxattr(path, follow_symlinks=False)
        need(not values, 'unsupported-xattrs-preserve-source')
    elif sys.platform == 'darwin':
        libc = ctypes.CDLL(None, use_errno=True)
        fn = libc.flistxattr if handle is not None else libc.listxattr
        fn.restype = ctypes.c_ssize_t
        if handle is not None:
            fn.argtypes = [ctypes.c_int, ctypes.c_void_p, ctypes.c_size_t, ctypes.c_int]
            n = fn(handle, None, 0, 0)
        else:
            fn.argtypes = [ctypes.c_char_p, ctypes.c_void_p, ctypes.c_size_t, ctypes.c_int]
            n = fn(os.fsencode(path), None, 0, 1)
        need(n == 0, 'unsupported-forks-xattrs-or-proof-failure')
    else:
        raise NoGo('unsupported-xattr-absence-proof')


class Node:
    def __init__(self, path, parent=None, name=None, delete=False):
        self.path, self.parent, self.name = str(path), parent, name
        self.delete = delete
        self.handle = None

    def lstat(self):
        return os.stat(self.name, dir_fd=self.parent.handle, follow_symlinks=False) \
            if os.name != 'nt' and self.parent else os.lstat(self.path)

    def __enter__(self):
        try:
            return self.open()
        except BaseException:
            self.__exit__()
            raise

    def open(self):
        before = self.lstat()
        if os.name == 'nt':
            self.handle = windows().open(self.path, delete=self.delete)
            self.meta = windows().meta(self.handle, before)
        else:
            mode = before.st_mode
            kind = next((name for test, name in [(stat.S_ISREG, 'file'), (stat.S_ISDIR, 'dir'),
                (stat.S_ISLNK, 'symlink'), (stat.S_ISFIFO, 'fifo'), (stat.S_ISSOCK, 'socket'),
                (stat.S_ISCHR, 'char'), (stat.S_ISBLK, 'block')] if test(mode)), None)
            need(kind is not None, 'unsupported-file-type')
            self.meta = dict(identity='posix:%s:%s' % (before.st_dev, before.st_ino), kind=kind,
                mode=mode, size=before.st_size, links=before.st_nlink, mtime=before.st_mtime_ns,
                ctime=before.st_ctime_ns, attributes=before.st_rdev,
                allocated=getattr(before, 'st_blocks', 0) * 512, device=str(before.st_dev))
            if kind in ['file', 'dir']:
                flags = os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK
                if kind == 'dir':
                    flags |= os.O_DIRECTORY
                self.handle = os.open(self.name, flags, dir_fd=self.parent.handle) \
                    if self.parent else os.open(self.path, flags)
                opened = os.fstat(self.handle)
                need((opened.st_dev, opened.st_ino, opened.st_mode) ==
                     (before.st_dev, before.st_ino, mode), 'changed-open-identity')
            no_xattrs(self.path, self.handle)
        return self

    def __exit__(self, *args):
        if self.handle is not None:
            windows().close(self.handle) if os.name == 'nt' else os.close(self.handle)
            self.handle = None

    def read(self):
        blocks, size = [], 0
        while size < CHUNK:
            block = windows().read(self.handle, CHUNK - size) if os.name == 'nt' \
                else os.read(self.handle, CHUNK - size)
            if not block:
                break
            blocks.append(block)
            size += len(block)
        return b''.join(blocks)

    def special(self):
        if self.meta['kind'] == 'reparse':
            return windows().special(self.handle, self.meta)
        if self.meta['kind'] == 'symlink':
            return os.fsencode(os.readlink(self.name, dir_fd=self.parent.handle))
        return b''

    def stable(self):
        if os.name == 'nt':
            need(windows().meta(self.handle, self.lstat()) == self.meta, 'changed-open-metadata')
        else:
            info = self.lstat()
            need((info.st_dev, info.st_ino, info.st_size, info.st_mtime_ns,
                  info.st_ctime_ns, info.st_nlink, info.st_mode) ==
                 (int(self.meta['device']), int(self.meta['identity'].split(':')[-1]),
                  self.meta['size'], self.meta['mtime'], self.meta['ctime'],
                  self.meta['links'], self.meta['mode']), 'changed-source-metadata')


def names(node):
    """Sorted 64KiB pages; repeated scans replace an unbounded inventory spool."""
    last = None
    while True:
        page, size = [], 0
        with os.scandir(node.path if os.name == 'nt' else node.handle) as entries:
            for entry in entries:
                raw = raw_name(entry.name)
                need(0 < len(raw) <= 4096, 'name-bound')
                if last is not None and raw <= last:
                    continue
                cost = len(raw) * 4 + 256
                bisect.insort(page, (raw, entry.name, cost))
                size += cost
                while size > CHUNK:
                    size -= page.pop()[2]
        if not page:
            return
        for raw, name, _ in page:
            yield name
        last = page[-1][0]


def nodes(plan, context):
    def descend(node, index, parts, device):
        check(plan, context, 'capture')
        with node:
            meta = dict(node.meta, root=index, parts=parts, **scope(plan, context, node, index))
            need(meta['device'] == device or device is None, 'mount-crossing')
            need(len(parts) <= plan['limits']['depth'], 'depth-bound')
            if meta['kind'] == 'dir':
                yield DIR, meta, node
                for name in names(node):
                    yield from descend(Node(Path(node.path) / name, node, name),
                                       index, parts + [pack_name(name)], meta['device'])
                node.stable()
                yield DIR_END, meta, node
            else:
                yield FILE if meta['kind'] == 'file' else SPECIAL, meta, node
                node.stable()
    for index, root in enumerate(plan['roots']):
        with Node(root['path']) as top:
            need(top.meta['kind'] == 'dir' and top.meta['identity'] == root['identity'], 'root-identity')
        yield from descend(Node(root['path']), index, [], None)


def source_events(plan, context, aliases=True):
    count = total = 0
    for kind, meta, node in nodes(plan, context):
        if kind != DIR_END:
            count += 1
            need(count <= plan['limits']['entries'], 'entry-cap')
        if kind != DIR_END and meta['kind'] != 'dir' and aliases and meta['links'] > 1:
            found = sum(k != DIR_END and m['kind'] != 'dir' and m['identity'] == meta['identity']
                        for k, m, _ in nodes(plan, context))
            need(found == meta['links'], 'unresolved-external-hardlink-alias')
        if kind == FILE:
            total += meta['size']
            need(total <= plan['limits']['logical_bytes'], 'logical-byte-cap')
            yield FILE, meta, meta['size']
            digest, size = hashlib.sha256(), 0
            while True:
                check(plan, context, 'capture')
                block = node.read()
                if not block:
                    break
                digest.update(block)
                size += len(block)
                need(size <= meta['size'], 'growing-source-file')
                yield DATA, block, 0
            need(size == meta['size'], 'short-source-file')
            yield FILE_END, {'sha256': digest.hexdigest()}, 0
        elif kind == SPECIAL:
            data = node.special()
            need(len(data) <= CHUNK, 'special-metadata-cap')
            yield SPECIAL, dict(meta, forensic_bytes=len(data),
                                forensic_sha256=hashlib.sha256(data).hexdigest()), len(data)
            if data:
                yield DATA, data, 0
        else:
            yield kind, meta, 0


class EventDigest:
    def __init__(self):
        self.hash = hashlib.sha256()
        self.events = self.files = self.dirs = self.specials = self.logical_bytes = 0

    def add(self, kind, meta, size):
        if kind == DATA:
            self.hash.update(meta)
            return meta
        raw = encode(meta)
        need(len(raw) <= CHUNK, 'frame-metadata-cap')
        data = FRAME.pack(kind, len(raw), size) + raw
        self.hash.update(data)
        self.events += 1
        self.files += kind == FILE
        self.dirs += kind == DIR
        self.specials += kind == SPECIAL
        self.logical_bytes += size if kind == FILE else 0
        return data

    def result(self):
        return dict(events=self.events, files=self.files, dirs=self.dirs, specials=self.specials,
                    logical_bytes=self.logical_bytes, tree_sha256=self.hash.hexdigest())


def census(plan, context):
    digest = EventDigest()
    for event in source_events(plan, context):
        digest.add(*event)
    return digest.result()


class Sink:
    def __init__(self, path, cap, plan, context):
        self.file = open(path, 'xb')
        self.cap, self.plan, self.context = cap, plan, context
        self.size, self.hash = 0, hashlib.sha256()

    def write(self, data):
        check(self.plan, self.context, 'capture', len(data))
        need(self.size + len(data) <= self.cap, 'compressed-archive-cap')
        n = self.file.write(data)
        need(n == len(data), 'short-archive-write')
        self.size += n
        self.hash.update(data)
        return n

    def flush(self):
        self.file.flush()

    def tell(self):
        return self.size

    def close(self):
        self.file.flush()
        os.fsync(self.file.fileno())
        self.file.close()


class StrictReader:
    def __init__(self, path, compressed_sha, limit):
        self.file = open(path, 'rb')
        self.z = zlib.decompressobj(31)
        self.pending = self.buffer = b''
        self.total, self.limit = 0, limit
        self.hash, self.expected = hashlib.sha256(), compressed_sha

    def read(self, size):
        need(0 <= size <= CHUNK, 'read-bound')
        while len(self.buffer) < size and not self.z.eof:
            raw = self.pending
            if not raw:
                raw = self.file.read(CHUNK)
                need(bool(raw), 'truncated-gzip')
                self.hash.update(raw)
            try:
                plain = self.z.decompress(raw, CHUNK)
            except zlib.error as error:
                raise NoGo('gzip-integrity') from error
            self.pending = self.z.unconsumed_tail
            self.total += len(plain)
            need(self.total <= self.limit, 'inflated-stream-cap')
            self.buffer += plain
            need(not self.z.unused_data, 'trailing-or-concatenated-gzip')
        result, self.buffer = self.buffer[:size], self.buffer[size:]
        return result

    def exact(self, size):
        data = self.read(size)
        need(len(data) == size, 'truncated-frame')
        return data

    def finish(self):
        need(self.read(1) == b'' and self.z.eof and not self.pending
             and self.file.read(1) == b'', 'gzip-or-inner-trailing-data')
        need(self.hash.hexdigest() == self.expected, 'compressed-sha')

    def close(self):
        self.file.close()


def archive_header(plan):
    return dict(format='FNFXAR01', version=1, host=plan['host'], source_pin=plan['source_pin'],
                tool_sha256=plan['tool_sha256'], roots=plan['roots'],
                capture_plan_sha256=plan.get('_capture_sha', plan['_sha']))


def archive_events(path, expected_sha, plan):
    reader = StrictReader(path, expected_sha, plan['limits']['stream_bytes'])
    digest, prefix = EventDigest(), hashlib.sha256()
    stack, next_root, active = [], 0, None
    try:
        magic = reader.exact(len(MAGIC))
        need(magic == MAGIC, 'archive-magic')
        prefix.update(magic)
        first = True
        while True:
            previous = prefix.hexdigest()
            header = reader.exact(FRAME.size)
            kind, length, size = FRAME.unpack(header)
            need(length <= CHUNK and size <= plan['limits']['logical_bytes'], 'frame-length')
            raw = reader.exact(length)
            meta = decode(raw)
            need(isinstance(meta, dict) and encode(meta) == raw, 'noncanonical-frame-metadata')
            if kind == END:
                need(not first and not stack and active is None and next_root == len(plan['roots']), 'incomplete-tree')
                need(encode(meta) == encode(dict(digest.result(), prefix_sha256=previous)) and size == 0, 'terminal-digest')
                reader.finish()
                return
            prefix.update(header)
            prefix.update(raw)
            if first:
                need(kind == HEAD and size == 0 and encode(meta) == encode(archive_header(plan)), 'archive-header-binding')
                first = False
                continue
            need(kind in [DIR, FILE, FILE_END, SPECIAL, DIR_END], 'unknown-frame-type')
            if kind in [DIR, FILE, SPECIAL]:
                keys = NODE_KEYS | ({'forensic_bytes', 'forensic_sha256'} if kind == SPECIAL else set())
                need(set(meta) == keys and meta['kind'] in KINDS and active is None, 'node-shape-or-state')
                need(all(integer(meta[k], 0, 2**64 - 1) for k in
                         ['mode', 'size', 'links', 'attributes', 'allocated'])
                     and all(integer(meta[k], -2**63, 2**63 - 1) for k in ['mtime', 'ctime'])
                     and meta['links'] > 0 and isinstance(meta['identity'], str)
                     and 0 < len(meta['identity']) <= 128 and isinstance(meta['device'], str)
                     and 0 < len(meta['device']) <= 128
                     and is_sha(meta['protected_scope_sha256']), 'node-field-types')
                mount = meta['mount_identity']
                need(isinstance(mount, dict) and set(mount) ==
                     {'backend', 'namespace', 'mount', 'parent_mount', 'root_mount'}
                     and all(isinstance(v, str) and 0 < len(v) <= 256 for v in mount.values())
                     and mount['mount'] == mount['parent_mount'] == mount['root_mount'],
                     'native-mount-metadata')
                parts, root = meta['parts'], meta['root']
                need(integer(root, 0, len(plan['roots']) - 1) and isinstance(parts, list)
                     and len(parts) <= plan['limits']['depth'], 'node-path')
                for token in parts:
                    name_bytes(token, plan['host'] == 'win32')
                if not parts:
                    need(kind == DIR and not stack and root == next_root
                         and meta['identity'] == plan['roots'][root]['identity'], 'root-order')
                    next_root += 1
                else:
                    need(stack and root == stack[-1][0]['root'] and parts[:-1] == stack[-1][0]['parts'], 'path-parent')
                    name = base64.b64decode(parts[-1], validate=True)
                    need(stack[-1][1] is None or name > stack[-1][1], 'duplicate-or-unsorted-name')
                    stack[-1][1] = name
                need((kind == DIR) == (meta['kind'] == 'dir')
                     and (kind == FILE) == (meta['kind'] == 'file'), 'type-confusion')
                if kind == DIR:
                    need(size == 0, 'directory-payload')
                    stack.append([meta, None])
                if kind == FILE:
                    need(size == meta['size'], 'file-size')
                    active = hashlib.sha256()
                if kind == SPECIAL:
                    need(integer(meta['forensic_bytes'], 0, CHUNK) and size == meta['forensic_bytes']
                         and is_sha(meta['forensic_sha256']), 'forensic-size')
            elif kind == DIR_END:
                need(size == 0 and stack and active is None and meta == stack[-1][0], 'directory-end')
                stack.pop()
            else:
                need(size == 0 and set(meta) == {'sha256'} and active is not None
                     and is_sha(meta['sha256']) and meta['sha256'] == active.hexdigest(), 'file-digest')
                active = None
            digest.add(kind, meta, size)
            need(digest.files + digest.dirs + digest.specials <= plan['limits']['entries']
                 and digest.logical_bytes <= plan['limits']['logical_bytes'], 'archive-census-cap')
            yield kind, meta, size
            forensic = hashlib.sha256()
            remaining = size
            while remaining:
                block = reader.exact(min(CHUNK, remaining))
                remaining -= len(block)
                prefix.update(block)
                digest.add(DATA, block, 0)
                if kind == FILE:
                    active.update(block)
                if kind == SPECIAL:
                    forensic.update(block)
                yield DATA, block, 0
            if kind == SPECIAL:
                need(forensic.hexdigest() == meta['forensic_sha256'], 'forensic-sha')
    finally:
        reader.close()


def artifact_members(plan):
    root = Path(plan['artifact_root'])
    ancestors(str(root))
    members = []
    with os.scandir(root) as entries:
        for entry in entries:
            need(len(members) < 5 and entry.name in {'archive.gz', *SIDECARS}, 'artifact-membership')
            info = entry.stat(follow_symlinks=False)
            need(stat.S_ISREG(info.st_mode) and not getattr(info, 'st_file_attributes', 0) & 0x400
                 and info.st_nlink == 1, 'linked-or-special-artifact')
            cap = plan['limits']['archive_bytes'] if entry.name == 'archive.gz' else SIDECARS[entry.name]
            need(info.st_size <= cap, 'artifact-individual-cap')
            members.append((entry.name, info.st_size))
    need(sum(size for name, size in members if name != 'archive.gz') <= 1048576, 'sidecar-total-cap')
    return members


def sidecar(plan, name, row, raw=None):
    need(name in SIDECARS and name != 'retirement.journal', 'sidecar-name')
    data = encode(row) + b'\n' if raw is None else raw
    need(len(data) <= SIDECARS[name], 'sidecar-cap')
    root = Path(plan['artifact_root'])
    members = artifact_members(plan)
    total = sum(size for key, size in members if key != 'archive.gz')
    need(total + len(data) <= 1048576, 'sidecar-total-cap')
    with (root / name).open('xb') as stream:
        stream.write(data)
        stream.flush()
        os.fsync(stream.fileno())


def compare(plan, context, expected_sha):
    artifact_members(plan)
    path = Path(plan['artifact_root']) / 'archive.gz'
    need(path.stat().st_size <= plan['limits']['archive_bytes'], 'archive-byte-cap')
    archived, current = archive_events(path, expected_sha, plan), source_events(plan, context)
    digest, sentinel = EventDigest(), object()
    try:
        while True:
            left, right = next(archived, sentinel), next(current, sentinel)
            if left is sentinel or right is sentinel:
                need(left is sentinel and right is sentinel, 'source-entry-change')
                break
            need(left == right, 'source-content-or-identity-change')
            digest.add(*left)
    finally:
        archived.close()
        current.close()
    return dict(status='VERIFIED', compressed_sha256=expected_sha, census=digest.result(),
                source_pin=plan['source_pin'], tool_sha256=sha_file(__file__),
                strict_single_gzip_EOF=True, source_compared_in_full=True, control_only=plan['control_only'])


def verify(plan_path, plan_sha, expected_archive_sha, context):
    plan = load_plan(plan_path, plan_sha, 'ARCHIVE')
    check(plan, context, 'capture')
    evidence(plan, context)
    return compare(plan, context, expected_archive_sha)


def archive(plan_path, plan_sha, context):
    plan = load_plan(plan_path, plan_sha, 'ARCHIVE')
    check(plan, context, 'capture', plan['limits']['archive_bytes'] + 1048576)
    evidence(plan, context)
    before = census(plan, context)
    root = Path(plan['artifact_root'])
    need(not root.exists(), 'new-artifact-prefix-required')
    root.mkdir()
    sink = None
    try:
        sink = Sink(root / 'archive.gz', plan['limits']['archive_bytes'], plan, context)
        digest, prefix, count = EventDigest(), hashlib.sha256(), 0
        with gzip.GzipFile(filename='', mode='wb', fileobj=sink, mtime=0, compresslevel=1) as output:
            def emit(data):
                nonlocal count
                count += len(data)
                need(count <= plan['limits']['stream_bytes'], 'stream-byte-cap')
                prefix.update(data)
                output.write(data)
            emit(MAGIC)
            header = encode(archive_header(plan))
            need(len(header) <= CHUNK, 'header-cap')
            emit(FRAME.pack(HEAD, len(header), 0) + header)
            for event in source_events(plan, context):
                emit(digest.add(*event))
            need(digest.result() == before, 'source-changed-during-capture')
            terminal = encode(dict(digest.result(), prefix_sha256=prefix.hexdigest()))
            need(count + FRAME.size + len(terminal) <= plan['limits']['stream_bytes'], 'stream-byte-cap')
            output.write(FRAME.pack(END, len(terminal), 0) + terminal)
        sink.close()
        proof = compare(plan, context, sink.hash.hexdigest())
        need(proof['census'] == before, 'source-changed-after-capture')
        sidecar(plan, 'archive-proof.json', dict(proof, capture_plan_sha256=plan_sha,
                    archive_bytes=sink.size, retirement_authorized=False))
        return proof
    except BaseException as error:
        if sink and not sink.file.closed:
            sink.close()
        if not (root / 'archive-proof.json').exists():
            sidecar(plan, 'archive-proof.json', dict(status='NO_GO', error=repr(error)[:1024],
                    source_retained=True, partial_archive_retained=True, retirement_authorized=False))
        raise


def stable_delete_meta(actual, expected):
    # Our earlier directory/hardlink unlinks change ctime, nlink and directory times.
    keys = ['identity', 'kind', 'mode', 'attributes', 'device']
    if expected['kind'] != 'dir':
        keys += ['size', 'mtime']
    need(all(actual[k] == expected[k] for k in keys), 'changed-retirement-identity')
    need(expected['kind'] == 'dir' or 0 < actual['links'] <= expected['links'], 'new-hardlink-alias')


def unlink_expected(node, meta, file_sha, context, plan):
    check(plan, context, 'retire')
    pinned = Node(node.path, node.parent, node.name, delete=os.name == 'nt')
    with pinned:
        stable_delete_meta(pinned.meta, meta)
        need(scope(plan, context, pinned, meta['root']) ==
             {k: meta[k] for k in ['mount_identity', 'protected_scope_sha256']},
             'changed-retirement-scope')
        if meta['kind'] == 'file':
            digest = hashlib.sha256()
            while True:
                check(plan, context, 'retire')
                block = pinned.read()
                if not block:
                    break
                digest.update(block)
            need(digest.hexdigest() == file_sha, 'changed-retirement-bytes')
        elif meta['kind'] == 'dir':
            need(next(names(pinned), None) is None, 'new-or-unlisted-directory-entry')
        else:
            need(hashlib.sha256(pinned.special()).hexdigest() == file_sha,
                 'changed-retirement-special')
        check(plan, context, 'retire')
        if os.name == 'nt':
            # Content and native identity were verified on THIS DELETE handle.
            # No close/reopen gap occurs before disposition; close below completes it.
            stable_delete_meta(windows().meta(pinned.handle, pinned.lstat()), meta)
            windows().unlink(pinned.handle)
        else:
            # Keep the verified inode open; the parent still must freeze namespace
            # writers because stat/unlinkat is not an atomic expected-ID unlink.
            info = pinned.lstat()
            need('posix:%s:%s' % (info.st_dev, info.st_ino) == meta['identity'],
                 'changed-before-unlink')
            if meta['kind'] == 'dir':
                os.rmdir(pinned.name, dir_fd=pinned.parent.handle)
            else:
                os.unlink(pinned.name, dir_fd=pinned.parent.handle)
    try:
        node.lstat()
    except FileNotFoundError:
        return
    raise NoGo('deletion-not-observed-or-path-replaced')


def retire(permit_path, permit_sha, context):
    """No default permit; no implicit retry of an interrupted deletion."""
    plan = load_plan(permit_path, permit_sha, 'RETIRE')
    check(plan, context, 'retire')
    evidence(plan, context)
    root = Path(plan['artifact_root'])
    proof = decode(small_file(root / 'archive-proof.json', SIDECARS['archive-proof.json']))
    need(proof.get('status') == 'VERIFIED' and proof.get('source_pin') == plan['source_pin']
         and proof.get('tool_sha256') == sha_file(__file__)
         and proof.get('compressed_sha256') == plan['archive_sha256']
         and proof.get('strict_single_gzip_EOF') is True and proof.get('source_compared_in_full') is True
         and proof.get('control_only') is plan['control_only'], 'backup-proof-binding')
    plan['_capture_sha'] = proof['capture_plan_sha256']
    need(compare(plan, context, plan['archive_sha256'])['census'] == proof['census'], 'frozen-census-mismatch')
    check(plan, context, 'retire')
    need(not (root / 'retirement.journal').exists(), 'partial-retirement-requires-parent-adjudication')
    permit_raw = small_file(permit_path, 131072)
    permit_target = root / 'retirement-permit.json'
    if os.path.abspath(permit_path) == os.path.abspath(permit_target):
        artifact_members(plan)
        need(hashlib.sha256(permit_raw).hexdigest() == permit_sha, 'permit-changed')
    else:
        sidecar(plan, 'retirement-permit.json', None, raw=permit_raw)
    journal = (root / 'retirement.journal').open('xb')
    stack, held, ordinal, logical, batch = [], [], 0, 0, 0
    def record(kind, values):
        raw = encode(dict(kind=kind, **values))
        need(len(raw) <= 512 and journal.tell() + len(raw) + 4 <= SIDECARS['retirement.journal'], 'journal-cap')
        journal.write(struct.pack('<I', len(raw)) + raw)
        journal.flush()
        os.fsync(journal.fileno())
    record('intent', dict(start=0, count=min(4096, plan['limits']['entries']), permit_sha256=permit_sha))
    status, failure = 'PARTIAL', None
    try:
        for kind, meta, _ in archive_events(root / 'archive.gz', plan['archive_sha256'], plan):
            if kind == DATA:
                continue
            if kind == DIR:
                if not meta['parts']:
                    path = Path(plan['roots'][meta['root']]['path'])
                    parent = Node(path.parent)
                    parent.__enter__()
                    held.append(parent)
                    node = Node(path, parent, path.name)
                else:
                    parent = stack[-1][1]
                    name = unpack_name(meta['parts'][-1], os.name == 'nt')
                    node = Node(Path(parent.path) / name, parent, name)
                node.__enter__()
                try:
                    stable_delete_meta(node.meta, meta)
                    need(scope(plan, context, node, meta['root']) ==
                         {k: meta[k] for k in ['mount_identity', 'protected_scope_sha256']}, 'changed-directory-scope')
                except BaseException:
                    node.__exit__()
                    raise
                stack.append((meta, node))
                continue
            if kind == FILE:
                pending = meta
                continue
            if kind == FILE_END:
                target, file_sha = pending, meta['sha256']
            elif kind == SPECIAL:
                target, file_sha = meta, meta['forensic_sha256']
            else:
                target, node = stack.pop()
                node.__exit__()
                file_sha = None
            parent = stack[-1][1] if stack else held[-1]
            name = unpack_name(target['parts'][-1], os.name == 'nt') if target['parts'] \
                else Path(plan['roots'][target['root']]['path']).name
            unlink_expected(Node(Path(parent.path) / name, parent, name), target, file_sha, context, plan)
            ordinal += 1
            logical += target['size'] if target['kind'] == 'file' else 0
            if ordinal % 4096 == 0:
                record('commit', dict(end=ordinal, removed_entries=ordinal, unlinked_logical_bytes=logical))
                batch += 1
                need(batch < 2048, 'journal-batch-bound')
                record('intent', dict(start=ordinal, count=4096, permit_sha256=permit_sha))
        record('commit', dict(end=ordinal, removed_entries=ordinal, unlinked_logical_bytes=logical))
        need(all(not os.path.lexists(p['path']) for p in plan['roots']), 'root-removal-not-observed')
        status = 'RETIRED'
    except BaseException as error:
        failure = repr(error)[:1024]
        record('stop', dict(last_observed_ordinal=ordinal, error=failure[:48],
                            ambiguous_uncommitted_credit=0))
    finally:
        for _, node in reversed(stack):
            node.__exit__()
        for node in reversed(held):
            node.__exit__()
        journal.close()
    row = dict(status=status, error=failure, permit_sha256=permit_sha,
        archive_sha256=plan['archive_sha256'], source_pin=plan['source_pin'],
        removed_entries_observed=ordinal, unlinked_logical_bytes_observed=logical,
        native_reclaimed_bytes=None, dedup_credit_bytes=0, unused_forecast_expiry_bytes=0,
        journal_sha256=sha_file(root / 'retirement.journal'), archive_retained=True,
        resume_authorized=False, control_only=plan['control_only'])
    sidecar(plan, 'retirement-result.json', row)
    return row
