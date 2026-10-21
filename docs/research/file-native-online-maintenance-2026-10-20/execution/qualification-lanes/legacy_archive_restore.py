"""Bounded legacy FNOP1 adapter; no engine/native runner or semantic decoder.

The old archive is per-directory DFS, not the newer manifest's global sort.
Reuse reviewed StrictGzipReader/path validation, preserve authenticated old order.
No function here is execution authority; LaneHost must admit all runtime work.
"""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re

STREAM_SHA = 'f8d223618d869bf85a11ec648254830e643aa1abbb86374233ac9f31fe46cc20'
ARCHIVE_SHA = '2fabd1c2439fc95a37a0411c99025bfb5646dee855b6ed0e879fd190e35e324b'
MANIFEST_SHA = '3b8f1b875a6b567cb386be34f4b3f7442f9e3538d8a58fbac524cf1473777281'
INVENTORY_SHA = '0974f4a1dac1f13618105b9fca425b2a56c99528accecb5255f0d4ec6515b9b5'
SOURCE_PIN = 'd70f3864f2888d46d9112213b4a03f8d82745a8e'
FILES, BYTES, BLOCK = 5807, 592458971, 65536
OLD_EXCLUDED = frozenset(('DISPOSABLE.json', 'writer.lock', 'preparation.lock',
                          'pack-readers.lock', 'pack-maintenance.lock'))


def require(test, message):
    if not test:
        raise RuntimeError(message)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def encoded(value):
    return (json.dumps(value, sort_keys=True, separators=(',', ':'),
                       ensure_ascii=True, allow_nan=False) + '\n').encode()


def strict(raw, maximum):
    require(type(raw) is bytes and len(raw) <= maximum, 'bounded-json')
    def pairs(rows):
        result = {}
        for k, v in rows:
            require(k not in result, 'duplicate-json-key')
            result[k] = v
        return result
    def invalid(_):
        raise ValueError('nonfinite-json')
    return json.loads(raw, object_pairs_hook=pairs, parse_constant=invalid)


def digest_file(path, host=None):
    digest = hashlib.sha256()
    with Path(path).open('rb') as f:
        for block in iter(lambda: f.read(BLOCK), b''):
            if host:
                poll(host)
            digest.update(block)
    return digest.hexdigest()


def binding(value, label):
    require(type(value) is dict and value.get('reviewed') is True, 'pending-binding:' + label)
    require(type(value.get('path')) is str and Path(value['path']).is_absolute(), 'binding-path:' + label)
    require(type(value.get('sha256')) is str and re.fullmatch('[0-9a-f]{64}', value['sha256']), 'binding-sha:' + label)
    require(digest_file(value['path']) == value['sha256'], 'binding-bytes:' + label)
    return value


def load_stream(config):
    item = binding(config['bindings']['stream'], 'stream')
    require(item['sha256'] == STREAM_SHA, 'stream-version')
    spec = importlib.util.spec_from_file_location('reviewed_fnop_stream', item['path'])
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def poll(host):
    row = host.poll()
    require(type(row) is dict and row.get('live') is True and
            re.fullmatch('[0-9a-f]{64}', row.get('receipt_sha256', '')), 'positive-guard-poll')
    return row


def record(host, path, value, category, maximum):
    raw = value if type(value) is bytes else encoded(value)
    require(len(raw) <= maximum, 'artifact-prewrite-bound')
    result = host.record(str(path), raw, category, maximum)
    require(type(result) is dict and result.get('sha256') == sha(raw) and
            result.get('bytes') == len(raw) and result.get('path') == str(path),
            'artifact-raw-binding')
    return result


def parse_inventory(manifest_raw, lines, core):
    """Pure parser, usable by fake controls without touching a corpus."""
    m = strict(manifest_raw, 65536)
    require(m.get('version') == 1 and m.get('profile') == 'quiescent-full-operational',
            'legacy-profile')
    require(type(m.get('files')) is int and 0 < m['files'] <= 8000 and
            type(m.get('logical_bytes')) is int and 0 <= m['logical_bytes'] <= BYTES,
            'legacy-count-byte-bound')
    for field in ('inventory_sha256', 'transport_sha256'):
        require(re.fullmatch('[0-9a-f]{64}', m.get(field, '')), 'legacy-hash-field')
    rows, names, directories, directory_keys = [], set(), set(), {}
    total, metadata = 0, 128
    inventory_digest = hashlib.sha256()
    for line in lines:
        require(type(line) is bytes and 0 < len(line) <= 4096, 'legacy-line-bound')
        inventory_digest.update(line)
        row = strict(line, 4096)
        require(type(row) is dict and set(row) == {'path', 'bytes', 'sha256'}, 'legacy-row-shape')
        name = core.path_name(row['path'])
        key = core.collision_key(name)
        require(key not in names and name not in core.EXCLUDED, 'legacy-duplicate-or-excluded')
        require(type(row['bytes']) is int and 0 <= row['bytes'] <= core.MAX_FILE and
                re.fullmatch('[0-9a-f]{64}', row['sha256']), 'legacy-row-bound')
        names.add(key)
        parent = name.rpartition('/')[0]
        while parent:
            pkey = core.collision_key(parent)
            require(pkey not in directory_keys or directory_keys[pkey] == parent,
                    'legacy-directory-case-collision')
            if parent not in directories:
                metadata = core.charge_metadata(metadata, parent)
                directories.add(parent)
                directory_keys[pkey] = parent
            require(len(directories) <= 16000, 'legacy-directory-limit')
            parent = parent.rpartition('/')[0]
        metadata = core.charge_metadata(metadata, row)
        total += row['bytes']
        require(total <= m['logical_bytes'] and len(rows) < m['files'], 'legacy-total-bound')
        rows.append(row)
    require(not names.intersection(directory_keys), 'legacy-file-directory-collision')
    require(inventory_digest.hexdigest() == m['inventory_sha256'], 'legacy-inventory-digest')
    require(len(rows) == m['files'] and total == m['logical_bytes'], 'legacy-inventory-completeness')
    canonical = dict(files=sorted(rows, key=lambda r: r['path']), directories=sorted(directories),
                     excluded=[], total_source_bytes=total)
    require(len(core.encode(canonical)) <= core.MAX_MANIFEST, 'legacy-manifest-bound')
    return dict(original=m, ordered_files=rows, canonical=canonical)


class ArchiveRestoreSession:
    """Resource-only shim for F09. No fake control_only/F11 successor reservation."""
    def __init__(self, host, config, core, inventory):
        self.host, self.config, self.core = host, config, core
        self.dest = Path(config['roots']['source'])
        self.c = dict(max_files=inventory['original']['files'], max_dirs=16000,
                      max_source_bytes=inventory['original']['logical_bytes'],
                      max_destination_bytes=inventory['original']['logical_bytes'],
                      max_compressed_bytes=config['archive']['compressed_bytes'])
        self.used = self.files = 0

    def tick(self):
        poll(self.host)

    def phase(self):
        self.tick()
        remaining = dict(bytes=self.c['max_destination_bytes'] - self.used,
                         files=self.c['max_files'] - self.files)
        receipt = self.host.checkpoint('long:legacy-restore',
                                      {self.config['categories']['source']: remaining})
        require(type(receipt) is dict and receipt.get('go') is True and
                re.fullmatch('[0-9a-f]{64}', receipt.get('receipt_sha256', '')),
                'legacy-phase-admission')

    def prewrite(self, size):
        self.tick()
        require(type(size) is int and size >= 0 and
                self.used + size <= self.c['max_destination_bytes'] and
                self.files + 1 <= self.c['max_files'], 'legacy-prewrite-bound')
        self.used += size
        self.files += 1


def restore_reader(source, parsed, session, core):
    """Exact old order and FNOP1 framing; strict gzip is the reviewed decoder."""
    session.phase()
    dest = core.no_links(session.dest)
    require(not os.path.lexists(dest), 'legacy-fresh-quarantine-required')
    dest.mkdir(exist_ok=False)
    for name in sorted(parsed['canonical']['directories'], key=lambda n: (n.count('/'), n)):
        session.tick()
        core.inside(dest / name, dest).mkdir(exist_ok=False)
    stream = core.StrictGzipReader(source, session)
    require(stream.take(6) == b'FNOP1\n', 'legacy-magic')
    seen, total = set(), 0
    for row in parsed['ordered_files']:
        header = stream.take(12)
        n, size = int.from_bytes(header[:4], 'little'), int.from_bytes(header[4:], 'little')
        require(0 < n <= 4096 and size <= core.MAX_FILE and
                total + size <= session.c['max_destination_bytes'], 'legacy-frame-bound')
        name = core.path_name(stream.take(n).decode('utf-8', 'strict'))
        key = core.collision_key(name)
        require(key not in seen and name == row['path'] and size == row['bytes'],
                'legacy-frame-inventory-binding')
        seen.add(key)
        target = core.inside(dest / name, dest)
        require(target.parent.is_dir(), 'legacy-parent-not-listed')
        session.prewrite(size)
        digest = hashlib.sha256()
        with target.open('xb') as out:
            remaining = size
            while remaining:
                block = stream.take(min(BLOCK, remaining))
                session.tick()
                require(out.write(block) == len(block), 'legacy-short-write')
                digest.update(block)
                remaining -= len(block)
            out.flush()
            os.fsync(out.fileno())
        require(digest.digest() == stream.take(32) and digest.hexdigest() == row['sha256'],
                'legacy-file-hash')
        total += size
    stream.finish()
    require(stream.sha.hexdigest() == parsed['original']['transport_sha256'],
            'legacy-compressed-sha')
    require(session.files == parsed['original']['files'] and
            total == parsed['original']['logical_bytes'], 'legacy-full-closure')
    session.phase()
    return dict(files=session.files, logical_bytes=total, raw_sha256=stream.raw_sha.hexdigest(),
                raw_bytes=stream.raw_count, compressed_sha256=stream.sha.hexdigest(),
                compressed_bytes=stream.count, gzip_single_member_eof=True, FNOP1_exact_eof=True)


class InventorySession:
    def __init__(self, host, limits):
        self.host, self.c = host, limits

    def tick(self):
        poll(self.host)


def inventory(host, config, root, *, maximum=None):
    core = load_stream(config)
    limits = dict(max_files=7839, max_dirs=16000,
                  max_source_bytes=maximum or config['limits']['source_construction_bytes'])
    return core.inventory(Path(root), InventorySession(host, limits))


def legacy_restore(host, config):
    """Only an already authorized LaneHost may call this real archive operation."""
    require(config.get('go') is True and config.get('runtime_go') is True,
            'source-only-config-no-runtime-GO')
    permit = host.authorize(config, 'long')
    require(type(permit) is dict and permit.get('go') is True and
            permit.get('final_pin') == config['final_pin'] and
            re.fullmatch('[0-9a-f]{64}', permit.get('receipt_sha256', '')), 'long-authorize')
    core = load_stream(config)
    archive, dest = Path(config['roots']['archive']), Path(config['roots']['source'])
    require(archive.is_absolute() and dest.is_absolute() and
            not dest.is_relative_to(archive) and not archive.is_relative_to(dest),
            'legacy-independent-roots')
    core.no_links(archive)
    for item, expected in [('manifest.json', MANIFEST_SHA), ('files.jsonl', INVENTORY_SHA)]:
        require(digest_file(archive / item, host) == expected, 'protected-donor-binding:' + item)
    with core.plain_open(archive / 'manifest.json') as f:
        manifest_raw = f.read(65537)
    with core.plain_open(archive / 'files.jsonl') as lines:
        parsed = parse_inventory(manifest_raw, iter(lambda: lines.readline(4097), b''), core)
    m = parsed['original']
    require(m['files'] == FILES and m['logical_bytes'] == BYTES and
            m['source_pin'] == SOURCE_PIN and set(m['excluded']) == OLD_EXCLUDED and
            m['transport_sha256'] == ARCHIVE_SHA and m['selected'] == config['archive']['selected'],
            'exact-retained-archive-required')
    require(len(parsed['canonical']['directories']) == 310, 'original-implied-directories')
    session = ArchiveRestoreSession(host, config, core, parsed)
    session.phase()
    require(not os.path.lexists(dest), 'legacy-retry-needs-new-admission')
    core.no_links(dest.parent)
    dest.parent.mkdir(parents=True, exist_ok=True)
    try:
        with core.plain_open(archive / 'operational.bin.gz') as source:
            proof = restore_reader(source, parsed, session, core)
        actual = core.inventory(dest, session)
        core.compare_destination(parsed['canonical'], actual)
        require(digest_file(archive / 'manifest.json', host) == MANIFEST_SHA and
                digest_file(archive / 'files.jsonl', host) == INVENTORY_SHA, 'donor-metadata-changed')
        require(proof['compressed_bytes'] == config['archive']['compressed_bytes'], 'archive-length')
        proof.update(status='legacy-byte-restore-complete-quarantined', empty_directories='implied-only',
                     directories=310, source_inventory_sha256=INVENTORY_SHA,
                     file_inventory_sha256=core.inventory_digest(actual), engine_open_allowed=False)
        receipt = record(host, Path(config['roots']['evidence']) / 'archive-byte-proof.json', proof,
                         config['categories']['source_evidence'], 16384)
        return dict(proof, receipt=receipt)
    except BaseException as error:
        record(host, Path(config['roots']['evidence']) / 'archive-byte-failure.json',
               dict(status='quarantined-incomplete', error=ascii(error)[:1000],
                    no_delete=True, no_automatic_retry=True), config['categories']['source_evidence'], 4096)
        raise




def forecast_long(c, profiles):
    """Pure schedule arithmetic. Profiles must come from a pinned source calculator."""
    require(type(profiles) is dict and set(profiles) ==
            {'resolve','lookup-bench','setup','convert','publish','cleanup','oracle','support'},
            'pending-typed-source-profiles')
    k = c['limits']['cleanup_batches']
    require(type(k) is int and 0 < k <= 100000, 'finite-cleanup')
    counts = dict(resolve=601, **{'lookup-bench':2}, setup=1, convert=1, publish=1,
                  cleanup=k, oracle=5)
    eb = ef = 0
    for name, count in counts.items():
        p = profiles[name]
        require(type(p) is dict and set(p) == {'input','stdout','stderr','receipt','output'},
                'closed-profile:'+name)
        require(all(type(v) is int and 0 <= v <= 8*1024**2 for v in p.values()),
                'profile-byte-bound')
        require((name == 'oracle' or p['input'] > 0) and p['stdout'] > 0 and p['receipt'] > 0 and p['stderr'] >= 2001,
                'raw-output-and-failure-bound')
        require((name == 'oracle') == (p['output'] > 0), 'oracle-output-only-in-M')
        eb += count * sum(p[x] for x in ('input','stdout','stderr','receipt'))
        ef += count * 4
    support = profiles['support']
    require(type(support) is dict and set(support) == {'bytes','files'} and
            all(type(v) is int and v > 0 for v in support.values()), 'whole-guard-host-support')
    # Two samples, cleanup index, result/failure, archive proof/failure, admission/finalizer.
    eb += 2*c['limits']['sample_bytes'] + c['limits']['cleanup_receipt_bytes']
    eb += c['limits']['result_bytes'] + 4*16384 + support['bytes']
    ef += 9 + support['files']
    mb = 2*1024**2 + 2*96*1024 + 4096 + 5*(profiles['oracle']['output']+16384+4096) + 3*16384
    mf = 12+2+1+5+5+5+3
    require(eb <= c['limits']['evidence_bytes'] and ef <= c['limits']['evidence_files'],
            'whole-E-source-schedule-does-not-fit')
    require(mb <= 8*1024**2 and mf <= 40, 'five-oracles-plus-stream-knownmaps-M8-40')
    return dict(engine_calls=606+k, lookup_calls=602, full_oracles=5,
                E_bytes=eb, E_files=ef, M_bytes=mb, M_files=mf,
                profile_counts=counts, profiles=profiles, deletion_credit=0,
                future_expiry_credit=0, source_calculator_required=True)

