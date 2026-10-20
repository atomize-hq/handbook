"""CONTROL ONLY; synthetic FNOP1 faults, never engine/native qualification."""
import gzip
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import struct
import sys
import time
import types
import zlib

HERE = Path(__file__).resolve().parent
MODULE = HERE.parent.parent / 'harness/stream_snapshot.py'
BASE = Path('C:/file-native-online-matrix-20261020/protocol/stream-control')
EVIDENCE = Path('C:/file-native-online-matrix-20261020/receipts/stream-control-main')
BUDGET = EVIDENCE / 'budget.json'
spec = importlib.util.spec_from_file_location('stream_snapshot', MODULE)
s = importlib.util.module_from_spec(spec)
spec.loader.exec_module(s)
s.job_member()
assert not BASE.exists()
BASE.mkdir()
source = BASE / 'source'
source.mkdir()
for d in ['empty/deep', 'nested', 'physical-pins']:
    (source / d).mkdir(parents=True)
for name, raw in {'STORE.json': b'{"version":3,"control":true}',
                  'SELECTED.json': b'{"publication":"synthetic-only"}',
                  'nested/exact.bin': 'exact caf\u00e9\r\n'.encode() * 7,
                  'test-ledger.jsonl': bytes(range(256)) * 512 + b'ledger-tail',
                  'physical-pins/x.lock': b'{"version":1,"id":"x","catalog":"durable-control"}'}.items():
    (source / name).write_bytes(raw)
for name in sorted(s.EXCLUDED):
    (source / name).write_bytes(b'ephemeral-or-foreign-disposal-identity')

def sha(path):
    return s.digest_file(path)

binding = dict(source_pin='0' * 40, module_sha256=sha(MODULE),
               binary=dict(path=str(Path(__file__).resolve()), sha256=sha(__file__)),
               oracle=dict(path=str(Path(__file__).resolve()), sha256=sha(__file__)),
               claim=dict(path=str(EVIDENCE / 'owner.json'), sha256=sha(EVIDENCE / 'owner.json')),
               source_bindings={name: sha(source / name) for name in ['STORE.json', 'SELECTED.json']})
deadline = min(time.time() + 240, s.CUTOFF)
rows = []

def config(label):
    return dict(version=1, quiescent=True, source=str(source), source_parent=str(BASE),
                destination=str(BASE / ('dest-' + label)), destination_parent=str(BASE),
                metadata_root=str(BASE / ('meta-' + label)), metadata_parent=str(BASE),
                max_files=100, max_dirs=100, max_source_bytes=262144,
                max_destination_bytes=262144, max_metadata_bytes=65536,
                max_compressed_bytes=524288, deadline_epoch=deadline,
                budget_path=str(BUDGET), budget_sha256=sha(BUDGET), binding=binding)

def record(label, **extra):
    row = dict(case=label, **extra)
    rows.append(row)
    print(json.dumps(row), flush=True)

def expect(label, fn, contains):
    try:
        fn()
    except (ValueError, zlib.error, OSError) as error:
        assert contains in str(error), (label, repr(error), contains)
        record(label, status='expected-refusal', error=repr(error), outputs_retained=True)
    else:
        raise AssertionError('control-did-not-refuse:' + label)

def verifier(dest, proof):
    assert proof['both_helpers_closed'] and all(p['exit'] == 0 for p in proof['helpers'])
    assert proof['transported_source_inventory_sha256'] == proof['destination_inventory_sha256']
    assert (dest / 'physical-pins/x.lock').read_bytes() == (source / 'physical-pins/x.lock').read_bytes()
    assert (dest / 'empty/deep').is_dir()
    assert sha(dest / 'test-ledger.jsonl') == sha(source / 'test-ledger.jsonl')
    assert all(not (dest / name).exists() for name in s.EXCLUDED)
    return dict(status='passed', oracle_sha256=binding['oracle']['sha256'],
                scope='synthetic-byte-control-only-no-engine-semantics')

good = s.capture_restore(config('positive'), verifier)
record('complete-pipe-durable-pin-empty-directory-all-ledger', status='passed',
       proof=good, scope='CONTROL ONLY')
expect('destination-preexistence', lambda: s.capture_restore(config('positive')), 'fresh-destination')

def bad_verifier(dest, proof):
    assert proof['both_helpers_closed'] and proof['transport']['gzip_crc_trailer_single_member_eof']
    return dict(status='failed', oracle_sha256=binding['oracle']['sha256'])
expect('failed-verifier-retains-quarantine', lambda: s.capture_restore(config('verifier-failed'), bad_verifier),
       'independent-verifier-failed')
assert (BASE / 'meta-verifier-failed/failure.json').is_file()

original_inventory = s.inventory
calls = [0]
def mutate_at_final_inventory(root, runtime, *args, **kwargs):
    if Path(root) == source:
        calls[0] += 1
        if calls[0] == 2:
            (source / 'nested/exact.bin').write_bytes(b'changed-after-stream')
    return original_inventory(root, runtime, *args, **kwargs)
s.inventory = mutate_at_final_inventory
try:
    expect('source-after-stream-content-change', lambda: s.capture_restore(config('source-changed')),
           'source-before-after-content-or-directory-change')
finally:
    s.inventory = original_inventory

data = b'abc'
row = dict(path='one.bin', bytes=len(data), sha256=hashlib.sha256(data).hexdigest())
manifest = dict(files=[row], directories=[], excluded=[], total_source_bytes=len(data))
def frame(name='one.bin', payload=data, size=None, digest=None):
    n = name.encode()
    return struct.pack('<IQ', len(n), len(payload) if size is None else size) + n + payload + (
        hashlib.sha256(payload).digest() if digest is None else digest)
raw = b'FNOP1\n' + frame()

def malformed(label, payload, contains, m=None, adjust=None):
    input_path = BASE / ('input-' + label + '.gz')
    input_path.write_bytes(payload)
    c = config(label)
    if adjust:
        c.update(adjust)
    runtime = s.Runtime(c)
    expect(label, lambda: s.consume(runtime, manifest if m is None else m, io.BytesIO(payload)), contains)
    rows[-1].update(input=str(input_path), input_sha256=sha(input_path), input_bytes=len(payload),
                    quarantine=str(runtime.dest), quarantine_exists=runtime.dest.exists())

malformed('truncated-gzip', gzip.compress(raw, mtime=0)[:-1], 'truncated-gzip')
malformed('truncated-frame', gzip.compress(raw[:-10], mtime=0), 'truncated-FNOP1')
malformed('wrong-file-hash', gzip.compress(b'FNOP1\n' + frame(digest=b'X' * 32), mtime=0), 'record-file-hash')
malformed('wrong-size', gzip.compress(b'FNOP1\n' + frame(size=4), mtime=0), 'record-path-size-binding')
malformed('unsafe-dot-path', gzip.compress(b'FNOP1\n' + frame('../escape'), mtime=0), 'unsafe-path')
malformed('unsafe-absolute-path', gzip.compress(b'FNOP1\n' + frame('/escape'), mtime=0), 'unsafe-path')
malformed('extra-FNOP1-EOF', gzip.compress(raw + b'Z', mtime=0), 'extra-FNOP1')
malformed('extra-gzip-EOF', gzip.compress(raw, mtime=0) + b'Z', 'gzip-trailing')
malformed('concatenated-gzip', gzip.compress(raw, mtime=0) + gzip.compress(b'', mtime=0), 'gzip-trailing')
bad_crc = bytearray(gzip.compress(raw, mtime=0)); bad_crc[-8] ^= 1
malformed('wrong-gzip-CRC', bytes(bad_crc), 'incorrect data check')
two = dict(files=[row, dict(row, path='two.bin')], directories=[], excluded=[], total_source_bytes=6)
malformed('duplicate-frame-path', gzip.compress(raw + frame(), mtime=0), 'duplicate-case', two)
malformed('case-collision-frame-path', gzip.compress(raw + frame('ONE.bin'), mtime=0), 'duplicate-case', two)
bad_manifest = dict(files=[row, dict(row, path='ONE.bin')], directories=[], excluded=[], total_source_bytes=6)
bad_manifest['files'].sort(key=lambda r: r['path'])
malformed('case-collision-manifest', gzip.compress(raw, mtime=0), 'manifest-duplicate', bad_manifest)
malformed('record-allocation-limit', gzip.compress(b'FNOP1\n' + frame(size=s.MAX_FILE + 1), mtime=0),
          'record-size-count-total-bound')
malformed('destination-output-budget', gzip.compress(raw, mtime=0), 'record-size-count-total-bound',
          adjust=dict(max_destination_bytes=2))

# Frozen Budget.census rejects retained links globally. Inject the exact lstat
# result instead of leaving a real symlink/reparse point in shared owned roots.
target = source / 'nested/exact.bin'
real_lstat = Path.lstat
for label, mode, attrs in [('mock-symlink', 0o120777, 0), ('mock-Windows-reparse', 0o100666, 0x400)]:
    def fake_lstat(self, mode=mode, attrs=attrs):
        if self == target:
            return types.SimpleNamespace(st_mode=mode, st_file_attributes=attrs)
        return real_lstat(self)
    Path.lstat = fake_lstat
    try:
        expect(label, lambda: s.no_links(target), 'link-or-reparse')
    finally:
        Path.lstat = real_lstat
    rows[-1]['qualification'] = 'fault-injected filesystem lstat; no actual retained link created'

expect('absolute-source-destination-containment',
       lambda: s.inside(BASE.parent / 'outside', BASE), 'outside-owned-root')
expect('unsafe-backslash', lambda: s.path_name('a\\b'), 'unsafe-path')
expect('unsafe-Windows-device', lambda: s.path_name('NUL.txt'), 'device-path')
past = config('cutoff'); past['deadline_epoch'] = time.time() - 1
expect('absolute-cutoff', lambda: s.Runtime(past), 'absolute-cutoff')
runtime = s.Runtime(config('prewrite'))
runtime.used = runtime.c['max_destination_bytes']
expect('prospective-prewrite-cap', lambda: runtime.prewrite(1), 'destination-prewrite-byte-cap')

actual = s.load_guard().census(BASE)
assert actual['bytes'] <= 2097152 and actual['files'] <= 150, actual
s.load_guard().Budget(BUDGET).check()
summary = dict(status='CONTROL_ONLY_PASS', source_sha256=sha(MODULE), source_bytes=MODULE.stat().st_size,
               control_source_sha256=sha(__file__), cases=len(rows), results=rows,
               protocol_actual=actual, no_engine=True, no_real_store=True,
               no_retained_real_compressed_transport=True,
               symlink_limit='lstat fault injection only; no native link creation because shared census must remain usable',
               qualification='tiny synthetic transport/byte/quarantine controls only; parent review and full long qualification pending')
out = EVIDENCE / 'controls.json'
with out.open('x', encoding='utf-8') as f:
    json.dump(summary, f, indent=2)
    f.write('\n')
print(json.dumps(dict(status=summary['status'], cases=len(rows), protocol_actual=actual)), flush=True)
