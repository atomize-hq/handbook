"""Read-only category census/admission for the macOS experiment guard.
Existing JSON contracts remain owner-managed; this is private harness state.
"""
import hashlib
import json
import os
from pathlib import Path
import shutil

ROOT = Path('/private/tmp/file-native-online-macos-20261020')
MIB = 1024**2
CLAIMS = {
    'F01': 'initial-native-worker-source4MiB-at-authority94734e76',
    'F04': 'CLAIM002:b77352544db52170902dd7b90a76c045943d38eaacb63523db08b71ebd9f2570',
    'F08': 'central:9b19a7b0eb3fab63f76c53b7b629b18ec1828c3c4346dc4d839380299bd5f984',
    'F12': 'initial-native-worker-functional-evidence24MiB',
    'F13': 'initial-native-worker-combined-positive-shared-growth4MiB',
    'C1': 'CLAIM007:8232ff614dd32922175b8fc558c57293760dfc6460b888dc4402640588f9be35:seal:e29fce55d34c685b2579e3cc2c277ed731cb72606985ccf425ca2742b006812a',
}
LIMITS = {'F01': (4*MIB, 1000), 'F04': (256*MIB, 5000),
          'F08': (96*MIB, 30000), 'F12': (24*MIB, 10000),
          'F13': (4*MIB, 1500), 'C1': (64*MIB, 5000)}
# Exact parent-delivered activated Claim007 binding. This category capacity
# does not authorize copying cache contents or starting engine/build workloads.
SHARED = {
    'Git': (Path('/Users/spensermcconnell/__Active_Code/atomize-hq/handbook/.git'), 187972892, 5492),
    # The 5716-byte earlier net Cargo growth remains charged, not rebased away.
    'Cargo': (Path('/Users/spensermcconnell/.cargo'), 1859800817, 81747),
}
CATEGORY_DIRS = {'F04': 'target', 'F08': 'fixtures', 'F12': 'evidence',
                 'C1': 'cargo-private'}


def strict_json(raw):
    def pairs(rows):
        answer = {}
        for k, v in rows:
            if k in answer:
                raise ValueError('duplicate-key:'+k)
            answer[k] = v
        return answer
    return json.loads(raw, object_pairs_hook=pairs,
                      parse_constant=lambda x: (_ for _ in ()).throw(ValueError(x)))


def census(path, skip_top=(), shared=False, stop=None):
    path = Path(path)
    logical = allocated = files = directories = symlinks = 0
    if not path.exists():
        return dict(logical_bytes=0, allocated_bytes=0, files=0, directories=0, symlinks=0)
    if path.is_symlink():
        raise ValueError('root-symlink:'+str(path))
    for directory, dirs, names in os.walk(path, followlinks=False,
                                           onerror=lambda e: (_ for _ in ()).throw(e)):
        if stop is not None and stop.is_set():
            raise InterruptedError("census-stopped")
        directories += 1
        if Path(directory) == path:
            dirs[:] = [n for n in dirs if n not in skip_top]
            names = [n for n in names if n not in skip_top]
        for name in list(dirs):
            q = Path(directory)/name
            if q.is_symlink():
                if not shared:
                    raise ValueError('directory-symlink:'+str(q))
                s = q.lstat()
                logical += s.st_size
                allocated += s.st_blocks*512
                files += 1
                symlinks += 1
                dirs.remove(name)
        for name in names:
            if stop is not None and stop.is_set():
                raise InterruptedError("census-stopped")
            q = Path(directory)/name
            s = q.lstat()
            if q.is_symlink():
                if not shared:
                    raise ValueError('file-symlink:'+str(q))
                symlinks += 1
            logical += s.st_size
            allocated += s.st_blocks*512
            files += 1
    return dict(logical_bytes=logical, allocated_bytes=allocated, files=files,
                directories=directories, symlinks=symlinks)


def limits(overrides=None):
    result = {k: {'bytes': v[0], 'files': v[1], 'claim': CLAIMS[k]}
              for k, v in LIMITS.items()}
    for k, v in (overrides or {}).items():
        if k not in result or set(v) != {'bytes', 'files'}:
            raise ValueError('invalid-reduced-limit:'+k)
        for unit, n in v.items():
            if type(n) is not int or n < 0 or n > result[k][unit]:
                raise ValueError('limit-may-only-be-reduced:'+k)
            result[k][unit] = n
    return result


def prospective(raw):
    if set(raw) != set(LIMITS):
        raise ValueError('prospective-requires-all-six-categories')
    for k, v in raw.items():
        if set(v) != {'bytes', 'files'} or any(type(n) is not int or n < 0 for n in v.values()):
            raise ValueError('invalid-prospective:'+k)
    return raw


def snapshot(shared=True, stop=None):
    categories = {'F01': census(ROOT, skip_top=tuple(CATEGORY_DIRS.values()), stop=stop)}
    for k, name in CATEGORY_DIRS.items():
        categories[k] = census(ROOT/name, stop=stop)
    if shared:
        support = {}
        for k, (path, baseline_b, baseline_f) in SHARED.items():
            row = census(path, shared=True, stop=stop)
            row['baseline_logical_bytes'] = baseline_b
            row['baseline_files'] = baseline_f
            row['positive_bytes'] = max(0, row['logical_bytes']-baseline_b)
            row['positive_files'] = max(0, row['files']-baseline_f)
            support[k] = row
        categories['F13'] = dict(logical_bytes=sum(x['positive_bytes'] for x in support.values()),
                                 files=sum(x['positive_files'] for x in support.values()),
                                 allocated_bytes=None, shared_components=support)
    return {'categories': categories, 'free_bytes': shutil.disk_usage(ROOT).free}


def admit(actual, requested, cap, free_floor=20*1024**3):
    if actual['free_bytes'] < free_floor:
        raise RuntimeError('free-space-refusal')
    for category, observed in actual['categories'].items():
        need = requested[category]
        if observed['logical_bytes']+need['bytes'] > cap[category]['bytes']:
            raise RuntimeError('prospective-byte-cap:'+category)
        if observed['files']+need['files'] > cap[category]['files']:
            raise RuntimeError('prospective-file-cap:'+category)
    return True


def source_binding(path, expected_pin):
    if path is None or expected_pin is None:
        raise ValueError('engine-source-release-required')
    b = Path(path).read_bytes()
    if len(b) > 256*1024:
        raise ValueError('source-binding-cap')
    row = strict_json(b)
    if row.get('source_pin') != expected_pin or row.get('engine_release') is not True:
        raise ValueError('source-pin-release-mismatch')
    for member in row.get('source_files', []):
        relative = Path(member['path'])
        if relative.is_absolute() or '..' in relative.parts:
            raise ValueError('source-binding-path')
        p = ROOT/'source'/relative
        if p.is_symlink() or not p.is_file():
            raise ValueError('source-member-missing')
        data = p.read_bytes()
        if len(data) != member['bytes'] or hashlib.sha256(data).hexdigest() != member['sha256']:
            raise ValueError('source-member-mismatch')
    if not row.get('source_files'):
        raise ValueError('empty-source-binding')
    return row


def bounded_receipt(path,row,byte_cap=524288):
    """Prewrite bound for each private evidence JSON file."""
    path=Path(path)
    if not path.is_relative_to(ROOT/'evidence') or path.is_symlink():
        raise RuntimeError('receipt-path-refusal')
    data=(json.dumps(row,indent=2)+'\n').encode()
    if len(data)>byte_cap:
        raise RuntimeError('receipt-prewrite-byte-cap')
    current=census(ROOT/'evidence')
    previous=path.stat().st_size if path.exists() else 0
    extra_file=0 if path.exists() else 1
    if current['logical_bytes']-previous+len(data)>LIMITS['F12'][0] or current['files']+extra_file>LIMITS['F12'][1]:
        raise RuntimeError('F12-receipt-prewrite-cap')
    with path.open('wb') as out:
        out.write(data)


def bounded_cache_write(relative, data, ceiling):
    """Private cache writer: exact operation ceiling plus current sealed cap.

    C1 binds activated Claim007. This helper cannot release copying/build GO.
    Future copy operations must use this bounded writer; arbitrary subprocess
    writes are detected by the guard live census, not a filesystem hard quota.
    """
    relative = Path(relative)
    if relative.is_absolute() or '..' in relative.parts or not relative.parts:
        raise ValueError('C1-relative-path-required')
    if type(data) is not bytes or len(data) > 1024*1024:
        raise ValueError('C1-single-write-buffer-cap')
    if set(ceiling) != {'bytes', 'files'} or any(type(x) is not int or x < 0 for x in ceiling.values()):
        raise ValueError('C1-operation-ceiling-shape')
    cache = ROOT/CATEGORY_DIRS['C1']
    path = cache/relative
    if cache.is_symlink() or any(p.is_symlink() for p in (path, *path.parents) if p.is_relative_to(cache)):
        raise ValueError('C1-symlink-refusal')
    actual = census(cache)
    previous = path.stat().st_size if path.exists() else 0
    if path.exists() and not path.is_file():
        raise ValueError('C1-regular-file-required')
    after = {'bytes': actual['logical_bytes']-previous+len(data),
             'files': actual['files']+(0 if path.exists() else 1)}
    effective = limits()['C1']
    for unit in ('bytes', 'files'):
        if after[unit] > effective[unit]:
            raise RuntimeError('C1-held-or-sealed-prewrite-cap:'+unit)
        if after[unit] > ceiling[unit]:
            raise RuntimeError('C1-operation-prewrite-cap:'+unit)
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open('wb') as out:
        out.write(data)
