"""Source-only native allocation adapter; no admission is inferred from forecasts."""
import hashlib
import copy
import json
import os
from pathlib import Path
import stat
import threading
import time

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


def census(root, *, hashes=False, allow_initial_absence=False, deadline=None):
    """At most four FULL retries; a vanished member invalidates the whole pass.

    A successful concurrent walk is an observation, not an atomic snapshot. Budget
    anchors + ALL outstanding/settled growth remain the conservative floor until
    writers quiesce. No missing/unreadable member becomes a zero-sized entry.
    Only preflight of an explicitly fresh absent root may return an empty census.
    """
    root = confined(guard.ROOT, Path(root))
    end = min(guard.CUTOFF, time.time()+2, deadline if deadline is not None else guard.CUTOFF)
    errors = []
    for attempt in range(4):
        try:
            need(time.time() < end, 'census-deadline')
            try:
                initial = root.lstat()
            except FileNotFoundError:
                need(allow_initial_absence and not errors, 'missing-existing-census-root')
                return dict(files={}, directories=[], bytes=0, file_count=0, directory_count=0,
                            allocated_bytes=0, initial_absence_proven=True, retries=0)
            need(stat.S_ISDIR(initial.st_mode), 'regular-census-directory')
            files, directories, logical, allocated = {}, [], 0, 0
            pending = [root]
            while pending:
                folder = pending.pop()
                need(time.time() < end, 'census-deadline')
                before = folder.lstat()
                with os.scandir(folder) as entries:
                    children = list(entries)
                for entry in sorted(children, key=lambda e: e.name):
                    need(time.time() < end, 'census-deadline')
                    p = Path(entry.path)
                    info = p.lstat()  # FileNotFound invalidates THIS WHOLE pass.
                    need(not stat.S_ISLNK(info.st_mode), 'census-symlink')
                    name = p.relative_to(root).as_posix()
                    if stat.S_ISDIR(info.st_mode):
                        directories.append(name)
                        pending.append(p)
                    else:
                        need(stat.S_ISREG(info.st_mode), 'nonregular-census-file')
                        row = dict(bytes=info.st_size, device=info.st_dev, inode=info.st_ino)
                        if hashes:
                            row['sha256'] = guard.digest(p)
                            again = p.lstat()
                            need((info.st_dev, info.st_ino, info.st_size, info.st_mtime_ns) ==
                                 (again.st_dev, again.st_ino, again.st_size, again.st_mtime_ns),
                                 'hashed-census-file-changed')
                        files[name] = row
                        logical += info.st_size
                        allocated += info.st_blocks*512
                after = folder.lstat()
                if (before.st_dev, before.st_ino, before.st_mtime_ns) != (after.st_dev, after.st_ino, after.st_mtime_ns):
                    raise FileNotFoundError('directory-renamed-or-mutated-during-census')
            return dict(files=files, directories=directories, bytes=logical, file_count=len(files),
                        directory_count=len(directories), allocated_bytes=allocated,
                        initial_absence_proven=False, retries=attempt)
        except FileNotFoundError as error:
            errors.append(repr(error))
            if attempt < 3 and time.time()+.01 < end:
                time.sleep(.01)
    raise RuntimeError('bounded-census-retries-exhausted:'+repr(errors))


def inventory(root, hashes=False):
    return census(confined(guard.ROOT/'fixtures', root), hashes=hashes)


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
        self.anchor = {k: dict(claim['before'][k]) for k in ('fixtures', 'evidence')}
        self.charged = {k: dict(bytes=0, files=0, directories=0) for k in self.anchor}
        self.root_anchor, self.root_charged = {}, {}

    def observation(self, root, **kwargs):
        return census(root, deadline=self.claim['absolute_deadline'], **kwargs)

    def check(self, extra):
        for category in ('fixtures', 'evidence'):
            current = self.observation(guard.ROOT/category)
            reservations = [v[category] for v in self.pending.values()]
            add = extra.get(category, dict(bytes=0, files=0))
            byte_cap = min(guard.LIMITS[category][0], self.claim['before'][category]['bytes']+
                           self.claim['categories'][category]['bytes'])
            file_cap = min(guard.LIMITS[category][1], self.claim['before'][category]['files']+
                           self.claim['categories'][category]['files'])
            byte_floor = max(current['bytes'], self.anchor[category]['bytes']+self.charged[category]['bytes'])
            file_floor = max(current['file_count'], self.anchor[category]['files']+self.charged[category]['files'])
            need(byte_floor+sum(v['bytes'] for v in reservations)+add['bytes'] <=
                 byte_cap, 'prospective-byte-limit:'+category)
            need(file_floor+sum(v['files'] for v in reservations)+add['files'] <=
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
            actual = self.observation(root, allow_initial_absence=True)
            self.root_anchor[str(root)] = dict(bytes=actual['bytes'], files=actual['file_count'],
                                               directories=actual['directory_count'])
            self.root_charged[str(root)] = dict(bytes=0, files=0, directories=0)
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
            first_call = not any(k[0] == str(root) for k in self.used)
            current = self.observation(root, allow_initial_absence=first_call)
            same = [v['fixtures'] for v in self.pending.values() if v['root'] == str(root)]
            for field, actual in [('bytes', current['bytes']), ('files', current['file_count']),
                                  ('directories', current['directory_count'])]:
                floor = max(actual, self.root_anchor[str(root)][field]+self.root_charged[str(root)][field])
                need(floor+sum(v[field] for v in same)+growth[field] <= spec['ceiling'][field],
                     'case-ceiling:'+field)
            # Chunk frames and the final invocation row share ONE append-only
            # stage file. Tail capacity comes from F_GETPIPE_SZ on the actual
            # pipes before launch, not a root multiplier or guessed 1MiB default.
            raw_bound = profile['stdout_bytes']+profile['stderr_bytes']+pipe_tail_bytes
            # INPUT is already encoded data: it appears exactly twice (start,
            # final row). Reserve actual canonical length, not six hypothetical
            # escapes of already escaped JSON. Env may also appear in the closed
            # historical invocation. Stdout/stderr still need worst-case escaping.
            event_bound = 2*len(canonical(payload))+4*len(canonical(environment))+6*raw_bound+131072
            need(profile['event_bytes'] >= event_bound, 'full-input-and-raw-event-forecast')
            evidence = dict(bytes=2*raw_bound+event_bound, files=0)
            extra = dict(root=str(root), fixtures=dict(growth), evidence=evidence)
            self.check(extra)
            self.pending[key] = extra
            self.used[count_key] = used+1
            return dict(profile, event_bytes=event_bound, declared_event_bytes=profile['event_bytes'])

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
            saved = self.pending[key]  # Reservation stays held through every retry.
            previous = copy.deepcopy((self.anchor, self.charged, self.root_anchor, self.root_charged))
            try:
                if saved['root'] is not None:
                    actual = inventory(Path(saved['root']))
                    ceiling = self.case_specs[saved['root']]['ceiling']
                    need(actual['bytes'] <= ceiling['bytes'] and actual['file_count'] <= ceiling['files']
                         and actual['directory_count'] <= ceiling['directories'], 'actual-case-ceiling')
                # Transfer forecast charge before retiring the pending lease.
                # Keep it until ALL writers end; concurrent walks cannot reclaim
                # omitted/renamed bytes or failed prefixes from this floor.
                for category in self.charged:
                    for field in ('bytes', 'files', 'directories'):
                        self.charged[category][field] += saved[category].get(field, 0)
                if saved['root'] is not None:
                    for field in ('bytes', 'files', 'directories'):
                        self.root_charged[saved['root']][field] += saved['fixtures'][field]
                self.pending.pop(key)
                if not self.pending:
                    observed = {k: self.observation(guard.ROOT/k) for k in self.anchor}
                    for category, actual in observed.items():
                        self.anchor[category] = dict(bytes=actual['bytes'], files=actual['file_count'])
                        self.charged[category] = dict(bytes=0, files=0, directories=0)
                    for root in self.root_anchor:
                        # A fresh, still-absent preflight root is not silently
                        # recounted as zero; preserve its explicit initial floor.
                        if root == saved['root'] or Path(root).exists():
                            actual = self.observation(Path(root))
                            self.root_anchor[root] = dict(bytes=actual['bytes'], files=actual['file_count'],
                                                          directories=actual['directory_count'])
                            self.root_charged[root] = dict(bytes=0, files=0, directories=0)
                self.check({})
            except BaseException:
                self.anchor, self.charged, self.root_anchor, self.root_charged = previous
                self.pending[key] = saved
                raise
