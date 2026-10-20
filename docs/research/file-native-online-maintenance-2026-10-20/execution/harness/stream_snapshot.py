"""Private FNOP1/gzip pipe capture and independent quarantined byte restore.

No engine imports, persisted compressed transport, deletion or recovery open.
Caller excludes ALL source mutators for the whole operation and retains source.
An exact file/empty-directory sidecar supplements, never changes, FNOP1 framing.
Transport completion alone is NOT semantic qualification or an accepted successor.
Run only within the frozen job_guard Job, with a parent-admitted budget/config.
"""
import argparse
import ctypes
import gzip
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import stat
import struct
import subprocess
import sys
import time
import unicodedata
import zlib

MAGIC = b'FNOP1\n'
BLOCK = 65536
EXCLUDED = frozenset({'writer.lock', 'preparation.lock', 'pack-readers.lock',
                      'pack-maintenance.lock', 'physical-capture.lock', 'DISPOSABLE.json'})
GUARD_SHA = '1e4a965139cac5b48e6cbef57d0ac8cc3c3d0294f36a392383308abcee3affae'
CUTOFF = 1792581014.0  # 2026-10-21T11:10:14Z; never restarted.
MAX_SOURCE = 651179227
MAX_FILE = 512 * 1024**2
MAX_MANIFEST = 2 * 1024**2
PROTECTED = [Path('C:/Users/spmcc/Documents/__Project_Code/handbook'),
             Path('C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook')]


def require(condition, reason):
    if not condition:
        raise ValueError(reason)


def digest_file(path):
    h = hashlib.sha256()
    with plain_open(Path(path)) as f:
        for b in iter(lambda: f.read(BLOCK), b''):
            h.update(b)
    return h.hexdigest()


def no_links(path):
    """Reject all Windows reparse points as well as POSIX symlinks, at every ancestor."""
    path = Path(os.path.abspath(path))
    for item in [*reversed(path.parents), path]:
        if not os.path.lexists(item):
            continue
        s = item.lstat()
        require(not stat.S_ISLNK(s.st_mode) and not (getattr(s, 'st_file_attributes', 0) & 0x400),
                'link-or-reparse:' + str(item))
    return path


def plain_open(path):
    path = no_links(path)
    if os.name == 'nt':
        # OPEN_REPARSE_POINT + no write/delete sharing: do not follow a raced leaf.
        import msvcrt
        from ctypes import wintypes as w
        k = ctypes.WinDLL('kernel32', use_last_error=True)
        k.CreateFileW.argtypes = [w.LPCWSTR, w.DWORD, w.DWORD, ctypes.c_void_p,
                                 w.DWORD, w.DWORD, w.HANDLE]
        k.CreateFileW.restype = w.HANDLE
        k.CloseHandle.argtypes = [w.HANDLE]
        handle = k.CreateFileW(str(path), 0x80000000, 1, None, 3, 0x00200000, None)
        if handle == ctypes.c_void_p(-1).value:
            raise ctypes.WinError(ctypes.get_last_error())
        try:
            fd = msvcrt.open_osfhandle(handle, os.O_RDONLY | os.O_BINARY)
        except BaseException:
            k.CloseHandle(handle)
            raise
    else:
        fd = os.open(path, os.O_RDONLY | getattr(os, 'O_NOFOLLOW', 0))
    f = os.fdopen(fd, 'rb')
    try:
        s = os.fstat(f.fileno())
        require(stat.S_ISREG(s.st_mode) and not (getattr(s, 'st_file_attributes', 0) & 0x400),
                'not-plain-file')
        return f
    except BaseException:
        f.close()
        raise


def path_name(name):
    require(isinstance(name, str) and 0 < len(name.encode('utf-8')) <= 4096, 'name-bound')
    require(not any(ord(c) < 32 or c in '\\:*?"<>|\x7f' for c in name), 'unsafe-path')
    parts = name.split('/')
    for p in parts:
        require(p not in ('', '.', '..') and p[-1] not in ' .', 'unsafe-path')
        require(p.split('.')[0].rstrip(' .').upper() not in {'CON', 'PRN', 'AUX', 'NUL',
                'CONIN$', 'CONOUT$', 'CLOCK$',
                *('COM' + str(n) for n in range(1, 10)), *('LPT' + str(n) for n in range(1, 10)),
                *('COM' + n for n in '\u00b9\u00b2\u00b3'), *('LPT' + n for n in '\u00b9\u00b2\u00b3')},
                'device-path')
    require(not name.startswith('/'), 'absolute-path')
    return name


def collision_key(name):
    return unicodedata.normalize('NFC', name).casefold()


def encode(row):
    return (json.dumps(row, ensure_ascii=True, sort_keys=True, separators=(',', ':')) + '\n').encode()


def strict_json(raw):
    def pairs(items):
        out = {}
        for k, v in items:
            require(k not in out, 'duplicate-json-key')
            out[k] = v
        return out
    return json.loads(raw, object_pairs_hook=pairs,
                      parse_constant=lambda _: (_ for _ in ()).throw(ValueError('nonfinite-json')))


def read_json(path, bound=MAX_MANIFEST):
    with plain_open(Path(path)) as f:
        raw = f.read(bound + 1)
    require(len(raw) <= bound, 'json-byte-bound')
    return strict_json(raw)


def inside(path, parent):
    require(Path(path).is_absolute() and Path(parent).is_absolute(), 'absolute-owned-paths-required')
    p, root = no_links(path), no_links(parent)
    require(p != root and p.is_relative_to(root), 'outside-owned-root')
    return p


def job_member():
    require(os.name == 'nt' and os.environ.get('FILE_NATIVE_STAGE_JOB') == '1', 'reviewed-Windows-Job-required')
    from ctypes import wintypes as w
    k = ctypes.WinDLL('kernel32', use_last_error=True)
    k.GetCurrentProcess.restype = w.HANDLE
    k.IsProcessInJob.argtypes = [w.HANDLE, w.HANDLE, ctypes.POINTER(w.BOOL)]
    value = w.BOOL()
    require(k.IsProcessInJob(k.GetCurrentProcess(), None, ctypes.byref(value)) and value.value,
            'positive-native-Job-membership-required')


def load_guard():
    p = Path(__file__).with_name('job_guard.py')
    require(digest_file(p) == GUARD_SHA, 'frozen-guard-hash')
    spec = importlib.util.spec_from_file_location('frozen_stream_job_guard', p)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


class Runtime:
    """Phase admission plus bounded per-block deadline/RSS/free-space checks.

    The existing outer guard owns global live census, RSS sampling, termination
    and token lifetime. Per-write output accounting below is prospective.
    """
    def __init__(self, config):
        job_member()
        self.c = config
        require(config['version'] == 1 and config['quiescent'] is True, 'quiescent-contract-required')
        require(type(config['control_only']) is bool, 'explicit-control-or-real-profile')
        for key, maximum in [('max_files', 7936), ('max_dirs', 16000),
                             ('max_source_bytes', MAX_SOURCE), ('max_destination_bytes', MAX_SOURCE),
                             ('max_metadata_bytes', 8 * 1024**2), ('max_compressed_bytes', 1024**3)]:
            require(type(config[key]) is int and 0 < config[key] <= maximum, 'bound:' + key)
        require(type(config['deadline_epoch']) in (int, float) and
                0 < config['deadline_epoch'] <= CUTOFF, 'absolute-cutoff')
        self.source = inside(config['source'], config['source_parent'])
        self.dest = inside(config['destination'], config['destination_parent'])
        self.meta = inside(config['metadata_root'], config['metadata_parent'])
        for a, b in [(self.source, self.dest), (self.source, self.meta), (self.dest, self.meta)]:
            require(not a.is_relative_to(b) and not b.is_relative_to(a), 'overlapping-roots')
        for p in PROTECTED:
            require(not self.source.is_relative_to(p) and not self.dest.is_relative_to(p), 'protected-source-or-destination')
        require(Path(config['budget_path']).resolve() == Path(os.environ['FILE_NATIVE_BUDGET']).resolve(), 'budget-env-binding')
        require(digest_file(config['budget_path']) == config['budget_sha256'], 'budget-hash-binding')
        self.guard = load_guard()
        self.budget = self.guard.Budget(config['budget_path'])
        binding = config['binding']
        require(re.fullmatch('[0-9a-f]{40}', binding['source_pin']) is not None, 'source-pin')
        require(digest_file(__file__) == binding['module_sha256'] and Path(__file__).stat().st_size <= 131072,
                'frozen-owned-module')
        for key in ['binary', 'oracle', 'claim']:
            require(digest_file(binding[key]['path']) == binding[key]['sha256'], 'frozen-binding:' + key)
        for key in ['STORE.json', 'SELECTED.json']:
            require(re.fullmatch('[0-9a-f]{64}', binding['source_bindings'][key]) is not None, 'source-binding-hash')
        self.used = 0
        self.files = 0
        self.last_check = 0
        self.tick()

    def tick(self):
        require(time.time() < self.c['deadline_epoch'], 'absolute-cutoff')
        if time.monotonic() - self.last_check >= .2:
            self.last_check = time.monotonic()
            value = self.guard.rss(os.getpid())
            require(value is not None and 0 < value[0] <= 256 * 1024**2, 'stream-process-RSS')
            require(shutil.disk_usage(self.dest.parent).free >= 140 * 1024**3, 'Windows-free-floor')

    def phase(self, output=None, metadata_required=True):
        self.tick()
        require(Path(__file__).stat().st_size <= 131072 and
                digest_file(__file__) == self.c['binding']['module_sha256'], 'owned-module-changed')
        require(digest_file(self.c['budget_path']) == self.c['budget_sha256'], 'phase-budget-hash')
        # The outer frozen guard performs the full global preflight/live census.
        # Each phase independently checks exact most-specific OWNED roots, plus
        # remaining output reservations. Never sum ancestor/subset actuals twice.
        leaves = [self.source, self.dest, self.meta, Path(__file__).resolve(), Path(self.c['budget_path'])]
        selected, owners = {}, {}
        for leaf in leaves:
            matches = [(len(Path(rule['root']).parts), name, rule) for name, rule in
                       self.budget.categories.items() if leaf.is_relative_to(Path(rule['root']))]
            require(matches, 'unregistered-stream-root:' + str(leaf))
            _, name, rule = max(matches, key=lambda item: item[0])
            selected[name] = rule
            owners[str(leaf)] = name
        reserves = {name: [0, 0] for name in selected}
        expected_bytes, expected_files = output if output is not None else (
            self.c['max_destination_bytes'], self.c['max_files'])
        require(0 <= expected_bytes <= self.c['max_destination_bytes'] and
                0 <= expected_files <= self.c['max_files'], 'phase-output-reservation-bound')
        require(metadata_required or self.c['control_only'], 'real-profile-metadata-required')
        outputs = [(self.dest, expected_bytes, expected_files)]
        if metadata_required:
            outputs.append((self.meta, self.c['max_metadata_bytes'], 12))
        for leaf, byte_limit, file_limit in outputs:
            actual = self.guard.census(leaf)
            require(actual['bytes'] <= byte_limit and actual['files'] <= file_limit, 'phase-output-own-cap')
            extra = reserves[owners[str(leaf)]]
            extra[0] += byte_limit - actual['bytes']
            extra[1] += file_limit - actual['files']
        if not self.c['control_only']:
            extra = reserves[owners[str(self.dest)]]
            extra[0] += 16 * 1024**2  # Held accepted-successor positive-growth headroom.
            extra[1] += 52  # Together with 12 capture metadata files: 64 of 8,000.
        for name, rule in selected.items():
            actual = self.guard.census(rule['root'])
            if rule.get('charge_mode') == 'positive_delta':
                actual = dict(bytes=max(0, actual['bytes'] - rule['baseline_bytes']),
                              files=max(0, actual['files'] - rule['baseline_files']))
            reserve = reserves[name]
            require(actual['bytes'] + reserve[0] <= rule['bytes'] and
                    actual['files'] + reserve[1] <= rule['files'], 'phase-prospective-owned-cap:' + name)

    def prewrite(self, size, new_file=False):
        self.tick()
        require(type(size) is int and size >= 0 and self.used + size <= self.c['max_destination_bytes'],
                'destination-prewrite-byte-cap')
        require(self.files + int(new_file) <= self.c['max_files'], 'destination-prewrite-file-cap')
        self.used += size
        self.files += int(new_file)

    def metadata(self, name, row):
        path_name(name)
        require('/' not in name, 'metadata-name')
        raw = encode(row)
        require(len(raw) <= MAX_MANIFEST, 'metadata-record-bound')
        self.tick()
        # Producer/consumer receipts share one metadata reservation. Serialize
        # check+write with a retained one-byte gate; process death releases it.
        import msvcrt
        gate = no_links(self.meta / 'metadata-quota.lock').open('r+b')
        held = False
        try:
            require(os.fstat(gate.fileno()).st_size == 1, 'metadata-quota-gate')
            while not held:
                self.tick()
                gate.seek(0)
                try:
                    msvcrt.locking(gate.fileno(), msvcrt.LK_NBLCK, 1)
                    held = True
                except OSError:
                    time.sleep(.002)
            entries = list(self.meta.iterdir())
            require(len(entries) < 12, 'metadata-prewrite-file-cap')
            total = sum(no_links(p).stat().st_size for p in entries)
            # Both helper stderr files are bounded by the 2,000-character main
            # error emitter. Keep 4KiB each prospectively even before creation.
            stderr_reserve = sum(max(0, 4096 - (self.meta / (role + '.stderr')).stat().st_size)
                                 if (self.meta / (role + '.stderr')).exists() else 4096
                                 for role in ['produce', 'consume'])
            require(total + len(raw) + stderr_reserve <= self.c['max_metadata_bytes'], 'metadata-prewrite-cap')
            with (self.meta / name).open('xb') as f:
                require(f.write(raw) == len(raw), 'metadata-short-write')
                f.flush()
                os.fsync(f.fileno())
        finally:
            if held:
                gate.seek(0)
                msvcrt.locking(gate.fileno(), msvcrt.LK_UNLCK, 1)
            gate.close()


def inventory(root, runtime, include_excluded=True):
    """Full content hashes and actual directory list; no inode/mtime shortcut."""
    root = no_links(root)
    require(root.is_dir(), 'inventory-root')
    files, dirs, excluded, seen = [], [], [], set()
    total = 0
    metadata_bytes = 128  # Closed shape, brackets, separators and integer headroom.
    stack = [root]
    while stack:
        d = stack.pop()
        runtime.tick()
        with os.scandir(d) as it:
            entries = []
            for e in it:
                require(len(entries) < runtime.c['max_files'] + runtime.c['max_dirs'] + len(EXCLUDED), 'directory-entry-cap')
                entries.append(e)
        for e in sorted(entries, key=lambda x: x.name):
            p = no_links(e.path)
            name = path_name(p.relative_to(root).as_posix())
            k = collision_key(name)
            require(k not in seen, 'inventory-case-collision')
            seen.add(k)
            mode = p.lstat().st_mode
            if stat.S_ISDIR(mode):
                metadata_bytes = charge_metadata(metadata_bytes, name)
                dirs.append(name)
                require(len(dirs) <= runtime.c['max_dirs'], 'directory-count-cap')
                stack.append(p)
            else:
                require(stat.S_ISREG(mode), 'special-file')
                h, size = hashlib.sha256(), 0
                with plain_open(p) as f:
                    for b in iter(lambda: f.read(BLOCK), b''):
                        runtime.tick()
                        size += len(b)
                        require(size <= MAX_FILE and total + size <= runtime.c['max_source_bytes'], 'source-byte-cap')
                        h.update(b)
                total += size
                row = dict(path=name, bytes=size, sha256=h.hexdigest())
                metadata_bytes = charge_metadata(metadata_bytes, row)
                (excluded if name in EXCLUDED else files).append(row)
                require(len(files) <= runtime.c['max_files'] and len(excluded) <= len(EXCLUDED), 'source-file-cap')
    result = dict(files=sorted(files, key=lambda r: r['path']), directories=sorted(dirs),
                  excluded=sorted(excluded, key=lambda r: r['path']) if include_excluded else [],
                  total_source_bytes=total)
    require(len(encode(result)) <= MAX_MANIFEST, 'inventory-metadata-bound')
    return result


def charge_metadata(used, entry):
    # One entry is already name/record bounded. Never encode a huge aggregate
    # merely to find that it exceeds the sidecar bound.
    used += len(encode(entry)) + 1
    require(used <= MAX_MANIFEST, 'inventory-metadata-bound-before-append')
    return used


def validate_manifest(m, runtime):
    require(set(m) == {'files', 'directories', 'excluded', 'total_source_bytes'}, 'manifest-shape')
    require(isinstance(m['files'], list) and len(m['files']) <= runtime.c['max_files'] and
            isinstance(m['directories'], list) and len(m['directories']) <= runtime.c['max_dirs'] and
            isinstance(m['excluded'], list) and len(m['excluded']) <= len(EXCLUDED), 'manifest-count-cap')
    seen, dirset = set(), set(m['directories'])
    metadata_bytes = 128
    require(m['directories'] == sorted(m['directories']), 'directory-order')
    for name in m['directories']:
        path_name(name)
        metadata_bytes = charge_metadata(metadata_bytes, name)
        key = collision_key(name)
        require(key not in seen, 'manifest-duplicate-or-case-collision')
        seen.add(key)
    total = 0
    require(m['files'] == sorted(m['files'], key=lambda r: r['path']), 'file-order')
    for row in m['files'] + m['excluded']:
        require(set(row) == {'path', 'bytes', 'sha256'}, 'manifest-record-shape')
        name = path_name(row['path'])
        key = collision_key(name)
        require(key not in seen, 'manifest-duplicate-or-case-collision')
        seen.add(key)
        require(type(row['bytes']) is int and 0 <= row['bytes'] <= MAX_FILE and
                re.fullmatch('[0-9a-f]{64}', row['sha256']) is not None, 'manifest-file-bound')
        metadata_bytes = charge_metadata(metadata_bytes, row)
        total += row['bytes']
        require(total <= runtime.c['max_source_bytes'], 'manifest-total-cap')
        require((row in m['excluded']) == (name in EXCLUDED), 'exact-exclusion-list')
    for name in [*m['directories'], *(r['path'] for r in m['files'])]:
        parent = name.rpartition('/')[0]
        require(not parent or parent in dirset, 'manifest-directory-closure')
    require(type(m['total_source_bytes']) is int and total == m['total_source_bytes'], 'manifest-total-binding')


def inventory_digest(m):
    return hashlib.sha256(encode(m)).hexdigest()


def transported_inventory(m):
    return dict(files=m['files'], directories=m['directories'], excluded=[],
                total_source_bytes=sum(row['bytes'] for row in m['files']))


class HashWriter:
    def __init__(self, out, runtime):
        self.out, self.runtime = out, runtime
        self.sha, self.count = hashlib.sha256(), 0

    def write(self, data):
        self.runtime.tick()
        require(self.count + len(data) <= self.runtime.c['max_compressed_bytes'], 'compressed-output-cap')
        n = self.out.write(data)
        require(n == len(data), 'short-pipe-write')
        self.sha.update(data)
        self.count += n
        return n

    def flush(self):
        self.out.flush()


def produce(runtime, manifest, sink):
    """Only encoder: MAGIC; LE <IQ>; name; exact bytes; SHA256; raw EOF."""
    validate_manifest(manifest, runtime)
    runtime.phase(output=(sum(row['bytes'] for row in manifest['files']), len(manifest['files'])))
    outer = HashWriter(sink, runtime)
    raw_sha, raw_count = hashlib.sha256(), 0
    with gzip.GzipFile(fileobj=outer, mode='wb', filename='', mtime=0, compresslevel=1) as gz:
        def put(b):
            nonlocal raw_count
            runtime.tick()
            require(gz.write(b) == len(b), 'short-gzip-write')
            raw_sha.update(b)
            raw_count += len(b)
        put(MAGIC)
        for row in manifest['files']:
            name = row['path'].encode('utf-8')
            put(struct.pack('<IQ', len(name), row['bytes']))
            put(name)
            h, size = hashlib.sha256(), 0
            with plain_open(runtime.source / row['path']) as f:
                while True:
                    b = f.read(BLOCK)
                    if not b:
                        break
                    size += len(b)
                    require(size <= row['bytes'], 'source-grew-during-capture')
                    h.update(b)
                    put(b)
            require(size == row['bytes'] and h.hexdigest() == row['sha256'], 'source-changed-during-capture')
            put(h.digest())
    outer.flush()
    result = dict(compressed_sha256=outer.sha.hexdigest(), compressed_bytes=outer.count,
                  raw_sha256=raw_sha.hexdigest(), raw_bytes=raw_count,
                  files=len(manifest['files']), bytes=sum(r['bytes'] for r in manifest['files']),
                  inventory_sha256=inventory_digest(manifest))
    runtime.phase()
    return result


class StrictGzipReader:
    """Independent single-member decoder, bounded expansion, CRC/trailer and EOF.

    gzip.GzipFile accepts concatenated members; this parser deliberately refuses
    them and any compressed trailing byte, including zero padding.
    """
    def __init__(self, source, runtime):
        self.source, self.runtime = source, runtime
        self.decoder = zlib.decompressobj(31)
        self.pending, self.buffer = b'', b''
        self.sha, self.raw_sha = hashlib.sha256(), hashlib.sha256()
        self.count = self.raw_count = 0
        self.closed = False

    def fill(self):
        self.runtime.tick()
        if self.closed:
            return
        if self.decoder.eof:
            require(not self.decoder.unused_data and not self.pending, 'gzip-trailing-or-concatenated')
            extra = self.source.read(1)
            require(extra == b'', 'gzip-trailing-or-concatenated')
            self.closed = True
            return
        data = self.pending
        if not data:
            data = self.source.read(BLOCK)
            require(data, 'truncated-gzip')
            self.count += len(data)
            require(self.count <= self.runtime.c['max_compressed_bytes'], 'compressed-input-cap')
            self.sha.update(data)
        out = self.decoder.decompress(data, BLOCK)
        self.pending = self.decoder.unconsumed_tail
        if self.decoder.eof:
            require(not self.decoder.unused_data, 'gzip-trailing-or-concatenated')
            # At EOF unconsumed_tail can duplicate unused_data; both are refused.
            require(not self.pending, 'gzip-trailing-or-concatenated')
        self.raw_count += len(out)
        require(self.raw_count <= self.runtime.c['max_source_bytes'] +
                self.runtime.c['max_files'] * (4096 + 44) + len(MAGIC), 'raw-expansion-cap')
        self.raw_sha.update(out)
        self.buffer += out

    def take(self, n):
        require(type(n) is int and 0 <= n <= BLOCK, 'bounded-read-required')
        while len(self.buffer) < n and not self.closed:
            self.fill()
        require(len(self.buffer) >= n, 'truncated-FNOP1')
        out, self.buffer = self.buffer[:n], self.buffer[n:]
        return out

    def finish(self):
        require(not self.buffer, 'extra-FNOP1-bytes')
        while not self.closed:
            self.fill()
            require(not self.buffer, 'extra-FNOP1-bytes')


def consume(runtime, manifest, source):
    """Independent decoder/writer; destination stays quarantined on all errors."""
    validate_manifest(manifest, runtime)
    output = (sum(row['bytes'] for row in manifest['files']), len(manifest['files']))
    runtime.phase(output=output, metadata_required=runtime.meta.exists())
    require(not os.path.lexists(runtime.dest), 'fresh-quarantine-required')
    runtime.dest.mkdir(exist_ok=False)
    for name in sorted(manifest['directories'], key=lambda n: (n.count('/'), n)):
        runtime.tick()
        target = inside(runtime.dest / name, runtime.dest)
        target.mkdir(exist_ok=False)
    stream = StrictGzipReader(source, runtime)
    require(stream.take(6) == b'FNOP1\n', 'FNOP1-magic')
    seen = set()
    count = total = 0
    for row in manifest['files']:
        # Independently unpack framing, never call the encoder or engine codec.
        header = stream.take(12)
        n = int.from_bytes(header[:4], 'little')
        size = int.from_bytes(header[4:], 'little')
        require(0 < n <= 4096 and size <= MAX_FILE and total + size <= runtime.c['max_destination_bytes'] and
                count < runtime.c['max_files'], 'record-size-count-total-bound')
        name = path_name(stream.take(n).decode('utf-8', 'strict'))
        key = collision_key(name)
        require(key not in seen and name not in EXCLUDED, 'duplicate-case-or-excluded-path')
        seen.add(key)
        require(name == row['path'] and size == row['bytes'], 'record-path-size-binding')
        target = inside(runtime.dest / name, runtime.dest)
        require(target.parent.is_dir(), 'unlisted-parent-directory')
        runtime.prewrite(size, new_file=True)
        h = hashlib.sha256()
        with target.open('xb') as out:
            remaining = size
            while remaining:
                b = stream.take(min(BLOCK, remaining))
                runtime.tick()
                require(out.write(b) == len(b), 'short-restored-write')
                h.update(b)
                remaining -= len(b)
            out.flush()
            os.fsync(out.fileno())
        require(h.digest() == stream.take(32) and h.hexdigest() == row['sha256'], 'record-file-hash')
        count += 1
        total += size
    stream.finish()
    runtime.phase(output=output, metadata_required=runtime.meta.exists())
    return dict(compressed_sha256=stream.sha.hexdigest(), compressed_bytes=stream.count,
                raw_sha256=stream.raw_sha.hexdigest(), raw_bytes=stream.raw_count,
                files=count, bytes=total, inventory_sha256=inventory_digest(manifest),
                gzip_crc_trailer_single_member_eof=True, FNOP1_exact_eof=True)


def compare_source(expected, actual):
    require(actual == expected, 'source-before-after-content-or-directory-change')


def compare_destination(manifest, actual):
    require(not actual['excluded'] and actual['files'] == manifest['files'] and
            actual['directories'] == manifest['directories'] and
            actual['total_source_bytes'] == sum(r['bytes'] for r in manifest['files']),
            'independent-destination-content-or-directory-mismatch')


def child(action, config_path, manifest_path, manifest_sha):
    config = read_json(config_path, 65536)
    r = Runtime(config)
    require(digest_file(manifest_path) == manifest_sha, 'frozen-manifest-hash')
    manifest = read_json(manifest_path)
    try:
        proof = produce(r, manifest, sys.stdout.buffer) if action == 'produce' else consume(r, manifest, sys.stdin.buffer)
        r.metadata(action + '.json', proof)
    except BaseException as e:
        r.metadata(action + '-failure.json', dict(error=repr(e)[:2000], quarantine_retained=True))
        raise


def capture_restore(config, verifier=None):
    """Caller-provided verifier(destination, proof) runs ONLY after all byte proof.

    A verifier must return status='passed' and the frozen oracle SHA. Without it,
    this returns transport-complete-awaiting-independent-oracle, never complete.
    The caller still owns full physical/semantic oracles and accepted successor.
    Source/binary/oracle/claim and this module must be independently parent frozen.
    """
    r = Runtime(config)
    r.phase()
    require(not os.path.lexists(r.dest) and not os.path.lexists(r.meta), 'fresh-destination-and-metadata-required')
    before = inventory(r.source, r)
    validate_manifest(before, r)
    for name, expected in config['binding']['source_bindings'].items():
        require(name in ('STORE.json', 'SELECTED.json') and
                next((row['sha256'] for row in before['files'] if row['path'] == name), None) == expected,
                'source-STORE-SELECTED-binding')
    require(set(config['binding']['source_bindings']) == {'STORE.json', 'SELECTED.json'}, 'required-source-bindings')
    require(before['total_source_bytes'] <= config['max_source_bytes'] <= MAX_SOURCE, 'F11-source-headroom')
    # F11's 8,000-file reservation includes capture metadata and successor growth.
    require(len(before['files']) + 64 <= 8000, 'F11-metadata-successor-file-headroom')
    require(sum(row['bytes'] for row in before['files']) <= config['max_destination_bytes'], 'destination-admission')
    r.meta.mkdir(exist_ok=False)
    with (r.meta / 'metadata-quota.lock').open('xb') as gate:
        gate.write(b'1')
        gate.flush()
        os.fsync(gate.fileno())
    r.metadata('source-manifest.json', before)
    r.metadata('request.json', config)
    manifest_path = r.meta / 'source-manifest.json'
    manifest_sha = digest_file(manifest_path)
    config_path = r.meta / 'request.json'
    helpers = []
    streams = []
    closed = False
    started = time.time()
    try:
        r.phase()
        for role in ['produce', 'consume']:
            err = (r.meta / (role + '.stderr')).open('xb')
            streams.append(err)
            command = [sys.executable, '-B', str(Path(__file__).resolve()), role,
                       str(config_path), str(manifest_path), manifest_sha]
            p = subprocess.Popen(command, stdin=subprocess.DEVNULL if role == 'produce' else helpers[0].stdout,
                                 stdout=subprocess.PIPE if role == 'produce' else subprocess.DEVNULL,
                                 stderr=err, close_fds=True)
            helpers.append(p)
            if role == 'consume':
                helpers[0].stdout.close()
        while any(p.poll() is None for p in helpers):
            r.tick()
            require(not any(p.poll() not in (None, 0) for p in helpers), 'stream-helper-failed')
            for p in helpers:
                if p.poll() is None:
                    value = r.guard.rss(p.pid)
                    if value is None:
                        require(p.poll() is not None, 'unreadable-live-helper-RSS')
                    else:
                        require(0 < value[0] <= 256 * 1024**2, 'helper-RSS-bound')
            time.sleep(.02)
        require(all(p.wait(timeout=2) == 0 for p in helpers), 'stream-helper-failed')
        closed = True
        for f in streams:
            f.close()
        produced, restored = read_json(r.meta / 'produce.json'), read_json(r.meta / 'consume.json')
        require(all(produced[k] == restored[k] for k in produced), 'independent-stream-proof-mismatch')
        require(restored['files'] == len(before['files']) and restored['inventory_sha256'] == inventory_digest(before), 'complete-manifest-proof')
        r.phase()
        after = inventory(r.source, r)
        compare_source(before, after)
        independent_dest = inventory(r.dest, r)
        compare_destination(before, independent_dest)
        require(inventory_digest(transported_inventory(before)) == inventory_digest(independent_dest),
                'source-destination-inventory-digest')
        r.phase()
        proof = dict(status='transport-complete-awaiting-independent-oracle',
                     destination=str(r.dest), source=str(r.source), quarantine=True,
                     binding=config['binding'], source_inventory_sha256=inventory_digest(before),
                     source_after_inventory_sha256=inventory_digest(after),
                     destination_inventory_sha256=inventory_digest(independent_dest),
                     transported_source_inventory_sha256=inventory_digest(transported_inventory(before)),
                     directory_sha256=hashlib.sha256(encode(before['directories'])).hexdigest(),
                     files=len(before['files']), directories=len(before['directories']),
                     excluded_exact=sorted(EXCLUDED), transport=restored,
                     helpers=[dict(pid=p.pid, exit=p.returncode) for p in helpers],
                     both_helpers_closed=True, seconds=time.time()-started,
                     no_retained_compressed_transport=True, accepted_successor='parent-required-on-destination-only')
        r.metadata('transport-proof.json', proof)
        if verifier is not None:
            result = verifier(r.dest, proof)
            require(isinstance(result, dict) and result.get('status') == 'passed' and
                    result.get('oracle_sha256') == config['binding']['oracle']['sha256'], 'independent-verifier-failed')
            proof.update(status='transport-and-provided-independent-verifier-passed', verifier=result)
        r.phase()
        r.metadata('result.json', proof)
        return proof
    except BaseException as e:
        for p in helpers:
            if p.poll() is None:
                p.kill()
        for p in helpers:
            try:
                p.wait(timeout=5)
            except subprocess.TimeoutExpired:
                pass
        closed = all(p.poll() is not None for p in helpers)
        r.metadata('failure.json', dict(status='quarantined-incomplete', error=repr(e)[:2000],
                   helpers=[dict(pid=p.pid, exit=p.returncode) for p in helpers],
                   both_helpers_closed=closed, outputs_retained=True))
        raise
    finally:
        for f in streams:
            f.close()
        for p in helpers:
            if p.stdout:
                p.stdout.close()
        require(closed or not helpers, 'helper-closure-unknown-outer-Job-must-reap')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('action', choices=['capture-restore', 'produce', 'consume'])
    parser.add_argument('config')
    parser.add_argument('manifest', nargs='?')
    parser.add_argument('manifest_sha', nargs='?')
    args = parser.parse_args()
    try:
        if args.action == 'capture-restore':
            print(json.dumps(capture_restore(read_json(args.config, 65536))))
        else:
            child(args.action, args.config, args.manifest, args.manifest_sha)
    except BaseException as error:
        sys.stderr.buffer.write(repr(error).encode('ascii', 'backslashreplace')[:2000] + b'\n')
        sys.stderr.buffer.flush()
        sys.exit(1)
