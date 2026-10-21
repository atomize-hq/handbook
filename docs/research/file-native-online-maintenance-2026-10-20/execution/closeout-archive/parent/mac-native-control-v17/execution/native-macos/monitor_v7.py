"""Owned-PID/PPID sampling and bounded, published supervisor accounting.
Never enumerates systemwide PIDs. No signaling in this module.
"""
import ctypes
import fcntl
import gzip
import json
import os
from pathlib import Path
import threading
import time
from native_probe import LIB, identity, usage, group
from admission_v7 import ROOT, strict_json

MAX_MEMBERS = 64
INTERVAL = .01
MAX_GAP = .04


def valid_identity(row):
    return row['returned_bytes'] == 136 and row['errno'] == 0 and row.get('uid') == os.getuid()


def same_identity(a, b):
    return valid_identity(a) and valid_identity(b) and all(a.get(k) == b.get(k) for k in ('pid','uid','pgid','start'))


def same_birth(a,b):
    return valid_identity(a) and valid_identity(b) and all(a.get(k)==b.get(k) for k in ('pid','uid','start'))


def children(pid):
    array = (ctypes.c_int * (MAX_MEMBERS+1))()
    ctypes.set_errno(0)
    n = LIB.proc_listpids(6, pid, array, ctypes.sizeof(array))
    error = ctypes.get_errno()
    valid = n >= 0 and n % 4 == 0 and n < ctypes.sizeof(array) and error == 0
    return {'parent_requested': pid, 'returned_bytes': n, 'errno': error,
            'valid': valid, 'children': list(array)[:n//4] if valid else None}


class OwnedTree:
    def __init__(self, anchor):
        self.anchor = anchor
        self.known = {anchor['pid']: anchor}
        self.observations = []
        self.escaped = []

    def observe(self):
        members = group(self.anchor['pgid'])
        if not members['valid']:
            raise RuntimeError('owned-group-query-unknown')
        observed = {}
        for pid in members['members']:
            row = identity(pid)
            if not valid_identity(row):
                first_failure = row
                until = time.monotonic()+.008
                ended = False
                while time.monotonic() < until:
                    again = group(self.anchor['pgid'])
                    if again['valid'] and pid not in again['members']:
                        ended = True
                        break
                    row = identity(pid)
                    if valid_identity(row):
                        break
                    time.sleep(.001)
                if ended:
                    continue
                if not valid_identity(row):
                    raise RuntimeError('live-group-identity-unknown:'+json.dumps(first_failure))
            if row['pgid'] != self.anchor['pgid']:
                raise RuntimeError('group-identity-mismatch')
            if pid in self.known and not same_identity(self.known[pid], row):
                raise RuntimeError('owned-member-identity-reused')
            observed[pid] = row
            self.known[pid] = row
        queue = list(observed.values())
        visited = set()
        while queue:
            parent = queue.pop()
            if parent['pid'] in visited:
                continue
            visited.add(parent['pid'])
            query = children(parent['pid'])
            if not query['valid']:
                raise RuntimeError('scoped-PPID-query-unknown')
            for pid in query['children']:
                row = identity(pid)
                if not valid_identity(row):
                    again = children(parent['pid'])
                    if again['valid'] and pid not in again['children']:
                        continue
                    raise RuntimeError('live-descendant-identity-unknown')
                # A child returned by a scoped PPID query must still bind that
                # exact observed parent, or be an already retained owned PID.
                if row['ppid'] != parent['pid'] and pid not in self.known:
                    raise RuntimeError('descendant-parent-race-unknown')
                if pid in self.known and not same_birth(self.known[pid], row):
                    raise RuntimeError('descendant-starttime-mismatch')
                self.known[pid] = row
                observed[pid] = row
                queue.append(row)
                if row['pgid'] != self.anchor['pgid']:
                    self.escaped.append(row)
            if len(self.known) > MAX_MEMBERS:
                raise RuntimeError('owned-process-count-cap')
        for pid, expected in list(self.known.items()):
            if pid in observed:
                continue
            row = identity(pid)
            if valid_identity(row) and same_birth(expected, row):
                self.known[pid] = row
                observed[pid] = row  # includes reparented escaped descendants
                if row['pgid'] != self.anchor['pgid']:
                    self.escaped.append(row)
            elif row['returned_bytes'] == 0 and row['errno'] in (0, 3):
                continue  # absence is later established by closure, not RSS
            else:
                raise RuntimeError('retained-owned-identity-unknown')
        result = {'group': members, 'identities': list(observed.values()),
                  'escaped': list({(x['pid'], tuple(x['start'])): x for x in self.escaped}.values())}
        self.observations.append({'monotonic_ns': time.monotonic_ns(),
                                  'pids': sorted(observed), 'escaped_pids': [x['pid'] for x in result['escaped']]})
        self.observations = self.observations[-16:]
        return result


class Registry:
    """Each owner publishes only self/explicit-child samples; peers read totals.
    Registry telemetry never grants permission to signal another owner.
    """
    def __init__(self, nonce):
        self.nonce = nonce
        self.lock_path = ROOT/'mac-supervisors.lock'
        self.path = ROOT/'mac-supervisors.json'

    def update(self, row=None, remove=False):
        with self.lock_path.open('a+b') as lock:
            until = time.monotonic()+.015
            while True:
                try:
                    fcntl.flock(lock.fileno(), fcntl.LOCK_EX|fcntl.LOCK_NB)
                    break
                except BlockingIOError:
                    if time.monotonic() >= until:
                        raise RuntimeError('supervisor-registry-contention')
                    time.sleep(.0005)
            data = strict_json(self.path.read_bytes()) if self.path.exists() else {}
            if not isinstance(data, dict) or len(data) > 16:
                raise RuntimeError('supervisor-registry-shape-cap')
            if remove:
                data.pop(self.nonce, None)
            elif row is not None:
                data[self.nonce] = row
            encoded = json.dumps(data, separators=(',', ':')).encode()
            if len(encoded) > 32768:
                raise RuntimeError('supervisor-registry-byte-cap')
            # Held inode lock protects this small telemetry rewrite. It is not
            # durable workload ownership; the separate HOST-FENCE is durable.
            with self.path.open('wb') as out:
                out.write(encoded)
            return data

    def publish(self, processes, stage):
        now = time.monotonic()
        own = {'nonce': self.nonce, 'sample_monotonic': now,
               'rss_bytes': sum(r['usage']['rss_bytes'] for r in processes),
               'processes': processes, 'stage': stage}
        peers = self.update(own)
        now = time.monotonic()
        total = 0
        for key, row in peers.items():
            if row.get('nonce') != key or not isinstance(row.get('rss_bytes'), int):
                raise RuntimeError('supervisor-registry-identity-shape')
            age = now-row['sample_monotonic']
            if age < 0 or age > MAX_GAP:
                raise RuntimeError('supervisor-registry-sample-stale')
            total += row['rss_bytes']
        if total > 768*1024**2:
            raise RuntimeError('all-supervisors-rss-cap')
        return total


class Sampler:
    def __init__(self, nonce, helper_identities, emit, send_heartbeat, rss_cap=2*1024**3):
        self.registry = Registry(nonce)
        self.helpers = helper_identities
        self.emit = emit
        self.send_heartbeat = send_heartbeat
        self.rss_cap = rss_cap
        self.tree = None
        self.sent_known = set()
        self.tree_lock = threading.Lock()
        self.stop_event = threading.Event()
        self.ready = threading.Event()
        self.failure = None
        self.last = None
        self.count = 0
        self.max_gap = 0
        self.peak_group = 0
        self.peak_helpers = 0
        self.thread = threading.Thread(target=self.loop, daemon=True)

    def attach(self, anchor):
        with self.tree_lock:
            self.tree = OwnedTree(anchor)
        self.ready.clear()

    def snapshot(self):
        rows = []
        heavy = 0
        with self.tree_lock:
            tree = self.tree.observe() if self.tree else None
        known = tree['identities'] if tree else []
        for expected in self.helpers+known:
            actual = identity(expected['pid'])
            memory = usage(expected['pid'])
            row = {'identity': actual, 'usage': memory}
            helper = expected in self.helpers
            if not same_identity(expected, actual) or memory['return'] != 0 or memory['rss_bytes'] is None:
                if not helper and actual['returned_bytes'] == 0 and actual['errno'] in (0, 3):
                    continue
                self.emit({'kind': 'missing-live-rss', 'row': row})
                raise RuntimeError('missing-live-rss-or-identity')
            rows.append(row)
            if not helper:
                heavy += memory['rss_bytes']
                self.peak_group = max(self.peak_group, heavy)
                if memory['rss_bytes'] > self.rss_cap:
                    self.emit({'kind':'rss-overshoot', 'row':row, 'partial_group_rss':heavy})
                    raise RuntimeError('per-process-rss-cap')
        helper_rows = rows[:len(self.helpers)]
        all_helpers = self.registry.publish(helper_rows, 'heavy' if tree else 'setup-or-token-wait')
        self.peak_helpers = max(self.peak_helpers, all_helpers)
        result = {'kind':'rss', 'monotonic':time.monotonic(), 'processes':rows,
                  'all_helpers_rss':all_helpers, 'owned_group_rss':heavy, 'tree':tree}
        self.emit(result)
        if tree:
            for expected in self.tree.known.values():
                key=(expected['pid'],tuple(expected['start']),expected['pgid'])
                if key not in self.sent_known:
                    self.send_heartbeat({'known':[expected]})
                    self.sent_known.add(key)
        if heavy > 2*1024**3 or heavy+all_helpers > 3*1024**3:
            raise RuntimeError('aggregate-rss-cap')
        if tree and tree['escaped']:
            raise RuntimeError('owned-descendant-escaped-session')
        return result

    def loop(self):
        previous = None
        target = time.monotonic()
        try:
            while not self.stop_event.is_set():
                result = self.snapshot()
                end = time.monotonic()
                if previous is not None:
                    gap = end-previous
                    self.max_gap = max(self.max_gap, gap)
                    if gap > MAX_GAP:
                        self.emit({'kind':'cadence-failure','gap_seconds':gap})
                        raise RuntimeError('rss-sample-gap-over-40ms')
                previous = end
                self.last = result
                self.last['completed_monotonic'] = end
                self.count += 1
                self.send_heartbeat({'sample_completed':end})
                self.ready.set()
                target += INTERVAL
                self.stop_event.wait(max(0, target-time.monotonic()))
        except BaseException as error:
            self.failure = repr(error)
            try:
                self.emit({'kind':'sampler-failure','error':self.failure,'monotonic':time.monotonic()})
            except BaseException:
                pass
            try:
                self.send_heartbeat({'failure':self.failure})
            except BaseException:
                pass
            self.ready.set()

    def before_go(self):
        if not self.ready.wait(.2) or self.failure or self.last is None:
            raise RuntimeError('positive-RSS-before-GO-failed:'+str(self.failure))
        if time.monotonic()-self.last['completed_monotonic'] > MAX_GAP:
            raise RuntimeError('positive-RSS-before-GO-stale')
        if self.tree and self.last.get('tree') is None:
            raise RuntimeError('anchor-RSS-before-GO-missing')
        if any(x['usage']['rss_bytes'] <= 0 for x in self.last['processes']):
            raise RuntimeError('positive-RSS-before-GO-nonpositive')
        return self.last

    def stop(self):
        self.stop_event.set()
        self.thread.join(timeout=.2)
        if self.thread.is_alive():
            raise RuntimeError('sampler-not-closed')
