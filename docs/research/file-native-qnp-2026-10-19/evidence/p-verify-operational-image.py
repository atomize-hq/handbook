"""Independent read-only FNOP1 verifier; outer caller enforces 115s process limit."""
import gzip, hashlib, json, os, struct, sys, time
from pathlib import Path, PurePosixPath

capsule, origin = map(Path, sys.argv[1:3])
expected_transport, expected_inventory = sys.argv[3:5]
expected_files, expected_bytes = map(int, sys.argv[5:7])
started = time.monotonic()
assert 0 < expected_files <= 100000 and 0 < expected_bytes <= 600000000

def digest(path):
    value = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1048576), b''):
            value.update(block)
    return value.hexdigest()

def regular(path):
    info = path.lstat()
    assert not path.is_symlink() and not getattr(info, 'st_file_attributes', 0) & 1024
    assert path.is_file()
    return info

manifest_path = capsule / 'manifest.json'
assert regular(manifest_path).st_size < 65536
manifest = json.loads(manifest_path.read_bytes())
assert manifest['version'] == 1 and manifest['profile'] == 'quiescent-full-operational'
assert Path(manifest['original']).resolve() == origin.resolve()
assert manifest['files'] == expected_files and manifest['logical_bytes'] == expected_bytes
assert digest(capsule / 'operational.bin.gz') == manifest['transport_sha256'] == expected_transport
assert digest(capsule / 'files.jsonl') == manifest['inventory_sha256'] == expected_inventory
exclusions = {'DISPOSABLE.json', 'writer.lock', 'preparation.lock', 'pack-readers.lock', 'pack-maintenance.lock'}
assert set(manifest['excluded']) == exclusions and len(manifest['excluded']) == 5
controls = {name: (origin / name).read_bytes() for name in ['SELECTED.json', 'STORE.json']}
assert json.loads(controls['SELECTED.json']) == manifest['selected']
seen, total = set(), 0

def take(stream, count):
    value = stream.read(count)
    assert len(value) == count
    return value

with gzip.open(capsule / 'operational.bin.gz', 'rb') as stream:
    assert take(stream, 6) == b'FNOP1\n'
    with (capsule / 'files.jsonl').open('rb') as inventory:
        for line in inventory:
            assert len(line) <= 16384
            row = json.loads(line)
            length, size = struct.unpack('<IQ', take(stream, 12))
            assert 0 < length <= 4096 and 0 <= size <= 512 * 1024**2
            name = take(stream, length).decode('utf-8')
            posix = PurePosixPath(name)
            assert name == row['path'] and size == row['bytes'] and name not in seen
            assert not posix.is_absolute() and '..' not in posix.parts and ':' not in name and '\\' not in name
            assert str(posix) == name and name not in exclusions
            seen.add(name)
            value, remaining = hashlib.sha256(), size
            while remaining:
                block = take(stream, min(1048576, remaining))
                value.update(block)
                remaining -= len(block)
            assert take(stream, 32) == value.digest() and value.hexdigest() == row['sha256']
            path = origin / name
            assert regular(path).st_size == size and digest(path) == row['sha256']
            total += size
            assert len(seen) <= expected_files and total <= expected_bytes
    assert stream.read(1) == b''
actual = set()
for parent, dirs, files in os.walk(origin, followlinks=False):
    for name in dirs:
        path = Path(parent) / name
        assert not path.is_symlink() and not getattr(path.lstat(), 'st_file_attributes', 0) & 1024
    for name in files:
        path = Path(parent) / name
        regular(path)
        relative = path.relative_to(origin).as_posix()
        if relative not in exclusions:
            actual.add(relative)
assert actual == seen and len(seen) == expected_files and total == expected_bytes
assert all((origin / name).read_bytes() == raw for name, raw in controls.items())
print(json.dumps({'status': 'PARENT_INDEPENDENT_FNOP1_STREAM_AND_ORIGINAL_FILES_PASS',
                  'transport_sha256': expected_transport, 'inventory_sha256': expected_inventory,
                  'manifest_sha256': digest(manifest_path), 'files': len(seen), 'logical_bytes': total,
                  'whole_gzip_EOF': True, 'complete_current_source_pathset_and_hashes_match': True,
                  'SELECTED_STORE_unchanged': True, 'source': str(origin), 'capsule': str(capsule),
                  'seconds': time.monotonic() - started, 'deletion_performed': False}))
