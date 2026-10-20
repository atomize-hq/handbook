from pathlib import Path, PurePosixPath
import json, hashlib, tarfile, gzip, time

start = time.monotonic()
p = Path('C:/Users/spmcc/.codex/worktrees/file-native-packing-20261020/handbook/docs/research/file-native-packing-2026-10-20/followup-20261020/evidence')
summary = json.loads((p / 'summary.json').read_bytes())
index = json.loads((p / 'raw-index.json').read_bytes())
external = json.loads((p / 'external-retained-index.json').read_bytes())
archive = p / 'raw-evidence.tar.gz'
assert archive.stat().st_size == summary['archive_bytes'] == 9317170
assert hashlib.sha256(archive.read_bytes()).hexdigest() == summary['archive_sha256'] == '46a56aa9c06510f965ddd76edf1ecfe6df6e56af6a805b3660d05c722c166ebb'
seen = set()
expanded = 0
with tarfile.open(archive, 'r|gz') as tar:
    for member in tar:
        n = PurePosixPath(member.name)
        assert member.isfile() and member.name not in seen and not n.is_absolute() and '..' not in n.parts and ':' not in member.name and '\\' not in member.name
        assert member.name in index and member.size == index[member.name]['bytes']
        seen.add(member.name)
        h = hashlib.sha256()
        stream = tar.extractfile(member)
        for block in iter(lambda: stream.read(1048576), b''):
            h.update(block)
        assert h.hexdigest() == index[member.name]['sha256']
        expanded += member.size
assert seen == set(index) and len(seen) == summary['members'] == 172 and expanded == summary['expanded_bytes'] == 55541654
with gzip.open(archive, 'rb') as stream:
    gzip_bytes = 0
    for block in iter(lambda: stream.read(1048576), b''):
        gzip_bytes += len(block)
assert gzip_bytes <= 128 * 1024**2
external_bytes = 0
for name, row in external.items():
    f = Path(row['path'])
    assert row['retained'] is True and f.is_file() and not f.is_symlink() and f.stat().st_size == row['bytes']
    h = hashlib.sha256()
    with f.open('rb') as stream:
        for block in iter(lambda: stream.read(1048576), b''):
            h.update(block)
    assert h.hexdigest() == row['sha256']
    external_bytes += row['bytes']
assert len(external) == summary['external_retained_entries'] == 13
print(json.dumps({'status': 'PARENT_DELTA_ALL_MEMBERS_EXTERNAL_BINDINGS_AND_GZIP_EOF_PASS', 'archive_bytes': summary['archive_bytes'], 'archive_sha256': summary['archive_sha256'], 'members': len(seen), 'raw_member_bytes': expanded, 'whole_gzip_EOF': True, 'external_entries': len(external), 'external_retained_bytes': external_bytes, 'seconds': time.monotonic() - start, 'sealed_report_encoding_correction': 'Separate corrected report and hash receipt required; raw archived report remains unchanged.'}))
