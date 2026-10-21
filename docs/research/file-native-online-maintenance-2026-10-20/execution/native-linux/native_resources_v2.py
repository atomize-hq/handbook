"""Source-only native allocation adapter; no admission is inferred from forecasts."""
import hashlib
import json
import os
from pathlib import Path
import stat
import threading

import native_guard as guard


def need(condition, message):
    if not condition:
        raise RuntimeError(message)


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'),
                      ensure_ascii=False, allow_nan=False).encode()


def strict(raw, limit=262144):
    need(type(raw) is bytes and len(raw) <= limit, 'bounded-json-bytes')
    def pairs(items):
        result = {}
        for key, value in items:
            need(key not in result, 'duplicate-json-key')
            result[key] = value
        return result
    def invalid(value):
        raise ValueError('nonfinite-json:'+value)
    return json.loads(raw, object_pairs_hook=pairs, parse_constant=invalid)


def digest_bytes(raw):
    return hashlib.sha256(raw).hexdigest()


def confined(base, path):
    """Reject every existing symlink component, including a dangling leaf."""
    base, path = Path(base), Path(path)
    need(base.is_absolute() and path.is_absolute(), 'absolute-owned-path')
    need('..' not in path.parts and path.is_relative_to(base), 'path-containment')
    current = Path(path.anchor)
    for part in path.parts[1:]:
        current /= part
        try:
            mode = current.lstat().st_mode
        except FileNotFoundError:
            continue
        need(not stat.S_ISLNK(mode), 'symlink-component:'+str(current))
    return path


def regular_read(path, limit):
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
    try:
        before = os.fstat(fd)
        need(stat.S_ISREG(before.st_mode) and before.st_size <= limit, 'bounded-regular-file')
        chunks, count = [], 0
        while True:
            block = os.read(fd, min(65536, limit+1-count))
            if not block:
                break
            chunks.append(block)
            count += len(block)
            need(count <= limit, 'read-growth-limit')
        after = os.fstat(fd)
        need((before.st_dev, before.st_ino, before.st_size, before.st_mtime_ns) ==
             (after.st_dev, after.st_ino, after.st_size, after.st_mtime_ns), 'read-changed')
        return b''.join(chunks)
    finally:
        os.close(fd)


def inventory(root, hashes=False):
    root = confined(guard.ROOT/'fixtures', root)
    files, directories, logical, allocated = {}, [], 0, 0
    if root.exists():
        for folder, dirs, names in os.walk(root, followlinks=False):
            for name in sorted(dirs):
                p = confined(root, Path(folder)/name)
                directories.append(p.relative_to(root).as_posix())
            for name in sorted(names):
                p = confined(root, Path(folder)/name)
                info = p.lstat()
                need(stat.S_ISREG(info.st_mode), 'nonregular-fixture-file')
                row = dict(bytes=info.st_size, device=info.st_dev, inode=info.st_ino)
                if hashes:
                    row['sha256'] = guard.digest(p)
                files[p.relative_to(root).as_posix()] = row
                logical += info.st_size
                allocated += info.st_blocks*512
    return dict(files=files, directories=directories, bytes=logical,
                file_count=len(files), directory_count=len(directories), allocated_bytes=allocated)


class Reservations:
    """Parent-sealed incremental growth, counting each existing root once.

    A global category census already includes every live/failed root. In-flight
    deltas are added conservatively; current bytes are never multiplied by the
    number of publishers. No future removal funds a reservation. Only a complete
    post-call census can settle one. The accepted outer guard remains authoritative.
    """
    def __init__(self, claim, admission, *, expected_ledger_sha256):
        need(set(admission) == {'ledger_sha256', 'claim_id', 'final_pin', 'go', 'cases'},
             'closed-admission-record')
        need(admission['go'] is True and admission['ledger_sha256'] == expected_ledger_sha256 ==
             claim['ledger_sha256'] and admission['claim_id'] == claim['claim_id'] and
             admission['final_pin'] == claim['source_pin'], 'parent-admission-versus-live-guard')
        self.claim, self.admission = claim, admission
        self.lock, self.pending, self.used, self.case_specs = threading.RLock(), {}, {}, {}

    def check(self, extra):
        for category in ('fixtures', 'evidence'):
            current = guard.inventory(guard.ROOT/category)
            reservations = [v[category] for v in self.pending.values()]
            add = extra.get(category, dict(bytes=0, files=0))
            byte_cap = min(guard.LIMITS[category][0], self.claim['before'][category]['bytes']+
                           self.claim['categories'][category]['bytes'])
            file_cap = min(guard.LIMITS[category][1], self.claim['before'][category]['files']+
                           self.claim['categories'][category]['files'])
            need(current['bytes']+sum(v['bytes'] for v in reservations)+add['bytes'] <=
                 byte_cap, 'prospective-byte-limit:'+category)
            need(current['files']+sum(v['files'] for v in reservations)+add['files'] <=
                 file_cap, 'prospective-file-limit:'+category)
        need(__import__('shutil').disk_usage(guard.ROOT).free >= 20*1024**3, 'free-floor')

    def preflight(self, case_id, root, forecast):
        root = confined(guard.ROOT/'fixtures', root)
        with self.lock:
            spec = self.admission['cases'][case_id]
            need(spec['root'] == str(root) and spec['forecast_sha256'] == digest_bytes(canonical(forecast)),
                 'case-root-forecast-not-parent-sealed')
            need(set(spec) == {'root', 'forecast_sha256', 'ceiling', 'calls', 'controls'}, 'case-spec-shape')
            self.check({})
            self.case_specs[str(root)] = spec
            return spec

    def reserve_call(self, key, payload, environment, pipe_tail_bytes):
        root = confined(guard.ROOT/'fixtures', Path(payload['root']))
        with self.lock:
            spec = self.case_specs[str(root)]
            operation = payload.get('operation', '')
            profile_key = payload['action']+'/'+operation
            profile = spec['calls'][profile_key]
            need(set(profile) == {'max_calls', 'growth', 'stdout_bytes', 'stderr_bytes',
                                  'event_bytes', 'input_bytes', 'required_fields', 'env_keys'}, 'call-profile-shape')
            need(all(payload.get(k) == v for k, v in profile['required_fields'].items()), 'call-required-fields')
            need(set(environment or {}) <= set(profile['env_keys']), 'call-env-allowlist')
            need(len(canonical(payload)) <= profile['input_bytes'], 'input-forecast-bound')
            count_key = (str(root), profile_key)
            used = self.used.get(count_key, 0)
            need(used < profile['max_calls'], 'frozen-call-count-exhausted')
            growth = profile['growth']
            need(set(growth) == {'bytes', 'files', 'directories'} and
                 all(type(v) is int and v >= 0 for v in growth.values()), 'exact-call-growth')
            current = inventory(root)
            same = [v['fixtures'] for v in self.pending.values() if v['root'] == str(root)]
            for field, actual in [('bytes', current['bytes']), ('files', current['file_count']),
                                  ('directories', current['directory_count'])]:
                need(actual+sum(v[field] for v in same)+growth[field] <= spec['ceiling'][field],
                     'case-ceiling:'+field)
            # Chunk frames and the final invocation row share ONE append-only
            # stage file. Tail capacity comes from F_GETPIPE_SZ on the actual
            # pipes before launch, not a root multiplier or guessed 1MiB default.
            raw_bound = profile['stdout_bytes']+profile['stderr_bytes']+pipe_tail_bytes
            need(profile['event_bytes'] >= 12*profile['input_bytes']+6*raw_bound+65536,
                 'full-input-and-raw-event-forecast')
            evidence = dict(bytes=2*raw_bound+profile['event_bytes'], files=0)
            extra = dict(root=str(root), fixtures=dict(growth), evidence=evidence)
            self.check(extra)
            self.pending[key] = extra
            self.used[count_key] = used+1
            return dict(profile)

    def reserve_evidence(self, key, byte_count, files=1):
        with self.lock:
            need(key not in self.pending, 'unique-reservation')
            extra = dict(root=None, fixtures=dict(bytes=0, files=0, directories=0),
                         evidence=dict(bytes=byte_count, files=files))
            self.check(extra)
            self.pending[key] = extra

    def reserve_control(self, key, byte_count, files=0, directories=0):
        with self.lock:
            need(key not in self.pending, 'unique-control-reservation')
            extra = dict(root=None, fixtures=dict(bytes=byte_count, files=files, directories=directories),
                         evidence=dict(bytes=0, files=0))
            self.check(extra)
            self.pending[key] = extra

    def settle(self, key):
        with self.lock:
            need(key in self.pending, 'unknown-reservation')
            saved = self.pending.pop(key)
            try:
                if saved['root'] is not None:
                    actual = inventory(Path(saved['root']))
                    ceiling = self.case_specs[saved['root']]['ceiling']
                    need(actual['bytes'] <= ceiling['bytes'] and actual['file_count'] <= ceiling['files']
                         and actual['directory_count'] <= ceiling['directories'], 'actual-case-ceiling')
                self.check({})
            except BaseException:
                self.pending[key] = saved
                raise
