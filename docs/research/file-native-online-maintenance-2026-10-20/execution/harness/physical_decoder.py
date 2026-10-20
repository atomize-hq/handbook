"""Independent CP1 STORE3/catalog2/FNPIDX01 verifier for QUIESCENT fixtures.

Schema inspected at62ce963085ac054bcc34eb21c6733b94db160975. No engine imports.
Legacy STORE1/2 parsing delegates unchanged to the prior independent Python
implementation. This is not the CP3 paged-format oracle and cannot qualify it.
Durable physical-pins/*.lock are DATA, not ephemeral backup exclusions.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import struct
import sys
import time

PRIOR = Path(os.environ.get('FILE_NATIVE_ORACLE_RESEARCH',
    'C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook/docs/research'
    if os.name == 'nt' else str(Path(__file__).resolve().parents[3])))
sys.path.insert(0, str(PRIOR/'file-native-packing-2026-10-20'))
import independent as legacy

SCHEMA_PIN = '62ce963085ac054bcc34eb21c6733b94db160975'
HASH = re.compile(r'[0-9a-f]{64}')
ID = re.compile(r'[A-Za-z0-9_-]{1,99}')
DESC = {'pack', 'index', 'pack_bytes', 'index_bytes', 'records', 'codec', 'role', 'first', 'last'}
GEN = {'id', 'ordinal', 'catalog', 'plan', 'packs'}
WORK = {'version', 'store_id', 'generation', 'older', 'jobs', 'retired', 'garbage',
        'removed_bytes', 'removed_files', 'reconciled_absent'}


def unsigned(value, maximum=2**64-1):
    assert type(value) is int and 0 <= value <= maximum
    return value


def shape(value, keys):
    assert type(value) is dict and set(value) == keys


def digest(data):
    return hashlib.sha256(data).hexdigest()


def descriptor(d, require_instance=False):
    assert type(d) is dict and set(d) in [DESC, DESC|{'instance'}]
    for name in ['pack', 'index']:
        assert type(d[name]) is str and HASH.fullmatch(d[name])
    assert unsigned(d['pack_bytes'], 64*1024**2) >= 0
    assert 53 <= unsigned(d['index_bytes'], 16*1024**2)
    assert 1 <= unsigned(d['records'], 65536)
    assert unsigned(d['codec'], 1) in [0, 1]
    assert all(type(d[k]) is str for k in ['role', 'first', 'last'])
    assert legacy.role(d['first']) == legacy.role(d['last']) == d['role'] and d['first'] <= d['last']
    instance = d.get('instance')
    if instance is not None:
        assert type(instance) is str and ID.fullmatch(instance)
    if require_instance:
        assert instance is not None
    return d


def container_path(d, extension):
    assert extension in ['pack', 'idx']
    h = d['pack' if extension == 'pack' else 'index']
    return ('packs/'+d['instance']+'/' if d.get('instance') else 'packs/')+h+'.'+extension


def generation(g):
    shape(g, GEN)
    assert type(g['id']) is str and ID.fullmatch(g['id'])
    assert unsigned(g['ordinal']) > 0 and type(g['catalog']) is str and HASH.fullmatch(g['catalog'])
    assert g['plan'] is None or type(g['plan']) is str and HASH.fullmatch(g['plan'])
    assert type(g['packs']) is list and len(g['packs']) <= 64
    for d in g['packs']:
        descriptor(d, True)


def metadata_path(name):
    assert type(name) is str and '\\' not in name and ':' not in name
    p = name.split('/')
    assert all(part not in ['', '.', '..'] for part in p)
    if len(p) == 2 and p[0] in ['physical-pins', 'physical-progress']:
        suffix = '.lock' if p[0] == 'physical-pins' else '.json'
        assert p[1].endswith(suffix) and ID.fullmatch(p[1][:-len(suffix)])
    elif len(p) == 2 and p[0] in ['physical-plans', 'pack-catalogs']:
        assert p[1].endswith('.json') and HASH.fullmatch(p[1][:-5])
    elif len(p) == 3 and p[0] == 'physical-stages':
        assert ID.fullmatch(p[1]) and p[2] == 'STORE.json'
    elif len(p) == 3 and p[0] == 'packs':
        assert ID.fullmatch(p[1]) and re.fullmatch(r'[0-9a-f]{64}\.(pack|idx)', p[2])
    else:
        raise AssertionError('physical-metadata-path')


def work(w):
    shape(w, WORK)
    assert w['version'] == 1 and type(w['version']) is int and type(w['store_id']) is str
    generation(w['generation'])
    assert type(w['older']) is list and len(w['older']) < 64
    assert all(type(w[k]) is list and len(w[k]) <= 4096 for k in ['jobs', 'retired', 'garbage'])
    for field in ['removed_bytes', 'removed_files', 'reconciled_absent']:
        unsigned(w[field])
    seen = set()
    for g in w['older']:
        generation(g)
        assert g['ordinal'] < w['generation']['ordinal'] and g['id'] not in seen
        assert g['plan'] is not None
        seen.add(g['id'])
    for j in w['jobs']:
        shape(j, {'descriptor', 'frontier', 'position'})
        descriptor(j['descriptor'])
        assert 0 < unsigned(j['frontier']) <= w['generation']['ordinal']
        assert unsigned(j['position'], 2**32-1) <= j['descriptor']['records']
    for r in w['retired']:
        shape(r, {'descriptor', 'pack_done', 'index_done'})
        descriptor(r['descriptor'])
        assert type(r['pack_done']) is bool and type(r['index_done']) is bool
    for name in w['garbage']:
        metadata_path(name)


class Decoder(legacy.Decoder):
    def __init__(self, root, *, quiescent=False):
        assert quiescent, 'explicit-quiescent-verifier-contract-required'
        self.root = Path(root)
        binding_bytes = legacy.bounded(self.root/'STORE.json', 65536)
        self.config = legacy.strict(binding_bytes)
        if self.config.get('version') != 3:
            super().__init__(root)  # Preserve legacy eager whole-index behavior.
            self.physical_metadata = dict(legacy=True, schema_pin=SCHEMA_PIN)
            return
        c = self.config
        allowed = {'version', 'store_id', 'mode', 'consumer', 'fallback', 'navigation_version', 'catalog', 'physical'}
        assert set(c) <= allowed and {'version', 'store_id', 'mode', 'consumer', 'fallback', 'catalog', 'physical'} <= set(c)
        assert len(binding_bytes) <= 4096
        assert type(c['version']) is int and type(c['store_id']) is str
        assert c['mode'] in ['A', 'B'] and c['consumer'] in ['project', 'research'] and type(c['fallback']) is bool
        if 'navigation_version' in c:
            assert type(c['navigation_version']) is int and c['navigation_version'] in [1, 2, 3, 4]
        b = c['physical']
        shape(b, {'id', 'ordinal', 'plan'})
        assert type(b['id']) is str and ID.fullmatch(b['id']) and unsigned(b['ordinal']) > 0
        assert type(b['plan']) is str and HASH.fullmatch(b['plan']) and type(c['catalog']) is str and HASH.fullmatch(c['catalog'])
        self.entries = {}
        self.descriptors = []
        self.zstd = None
        self.required = {'STORE.json'}
        self.pin(c['physical'], c['catalog'])
        catalog = self.catalog(c['catalog'])
        self.descriptors = catalog['packs']
        for d in self.descriptors:
            self.load_index(d)
        self.physical_metadata = self.selected_work(catalog)

    def read_metadata(self, name, limit, h=None):
        self.required.add(name)
        data = legacy.bounded(self.root/name, limit)
        assert h is None or digest(data) == h
        return legacy.strict(data)

    def pin(self, binding, catalog_hash):
        expected = dict(version=1, store_id=self.config['store_id'], id=binding['id'],
                        ordinal=binding['ordinal'], catalog=catalog_hash)
        actual = self.read_metadata('physical-pins/'+binding['id']+'.lock', 1024)
        shape(actual, {'version', 'store_id', 'id', 'ordinal', 'catalog'})
        assert unsigned(actual['version'], 1) == 1 and unsigned(actual['ordinal']) > 0
        assert actual == expected, 'durable-pin-identity-binding'

    def catalog(self, h):
        cat = self.read_metadata('pack-catalogs/'+h+'.json', 59*1024, h)
        shape(cat, {'version', 'store_id', 'packs'})
        assert cat['version'] == 2 and type(cat['version']) is int and cat['store_id'] == self.config['store_id']
        assert type(cat['packs']) is list and len(cat['packs']) <= 64
        previous = None
        indexes = set()
        for d in cat['packs']:
            descriptor(d, True)
            assert d['index'] not in indexes
            indexes.add(d['index'])
            assert previous is None or (previous['role'], previous['first']) < (d['role'], d['first']) and (previous['role'] != d['role'] or previous['last'] < d['first'])
            previous = d
            for ext, length in [('pack', d['pack_bytes']), ('idx', d['index_bytes'])]:
                name = container_path(d, ext)
                self.required.add(name)
                assert (self.root/name).stat().st_size == length
        return cat

    def load_index(self, d):
        data = legacy.bounded(self.root/container_path(d, 'idx'), d['index_bytes'], d['index_bytes'])
        assert digest(data) == d['index'] and data[:8] == b'FNPIDX01'
        assert data[8:40].hex() == d['pack']
        size, codec, count = struct.unpack_from('<QBI', data, 40)
        assert (size, codec, count) == (d['pack_bytes'], d['codec'], d['records'])
        pos = 53
        end = 0
        previous = ''
        first = None
        for _ in range(count):
            assert pos+2 <= len(data)
            n = struct.unpack_from('<H', data, pos)[0]
            pos += 2
            assert n <= 512 and pos+n+48 <= len(data)
            key = data[pos:pos+n].decode()
            pos += n
            offset, encoded, raw = struct.unpack_from('<QII', data, pos)
            pos += 16
            h = data[pos:pos+32].hex()
            pos += 32
            assert key > previous and legacy.role(key) == d['role'] and key not in self.entries
            assert offset == end and raw <= 8*1024**2 and encoded <= 8*1024**2+65536 and offset+encoded <= size
            assert codec != 0 or encoded == raw
            self.entries[key] = (d, offset, encoded, raw, h)
            first = key if first is None else first
            previous = key
            end = offset+encoded
        assert pos == len(data) and end == size and first == d['first'] and previous == d['last']

    def selected_work(self, catalog):
        b = self.config['physical']
        base = self.read_metadata('physical-plans/'+b['plan']+'.json', 4*1024**2, b['plan'])
        work(base)
        g = base['generation']
        assert base['store_id'] == self.config['store_id'] and g['id'] == b['id'] and g['ordinal'] == b['ordinal']
        assert g['catalog'] == self.config['catalog'] and g['packs'] == catalog['packs']
        active = base
        progress_name = 'physical-progress/'+b['id']+'.json'
        if (self.root/progress_name).exists():
            p = self.read_metadata(progress_name, 4*1024**2)
            shape(p, {'version', 'plan', 'work'})
            assert p['version'] == 1 and type(p['version']) is int and p['plan'] == b['plan']
            active = p['work']
            work(active)
            assert active['store_id'] == base['store_id'] and active['generation'] == base['generation']
            assert all(g in base['older'] for g in active['older'])
            for j in active['jobs']:
                assert any(old['descriptor'] == j['descriptor'] and old['frontier'] == j['frontier'] and old['position'] <= j['position'] for old in base['jobs'])
            for r in active['retired']:
                assert any(old['descriptor'] == r['descriptor'] and (not old['pack_done'] or r['pack_done']) and (not old['index_done'] or r['index_done']) for old in base['retired'])
            assert all(active[k] >= base[k] for k in ['removed_bytes', 'removed_files', 'reconciled_absent'])
            derived = set(base['garbage'])
            for old in base['older']:
                derived.update(['physical-pins/'+old['id']+'.lock', 'physical-progress/'+old['id']+'.json',
                                'physical-plans/'+old['plan']+'.json', 'pack-catalogs/'+old['catalog']+'.json'])
            assert set(active['garbage']) <= derived
        # Even reaped progress generations retain immutable-plan pin identities
        # until a successor omits them. Their drained containers may be gone.
        for old in base['older']:
            self.pin(old, old['catalog'])
        for old in active['older']:
            retained = self.catalog(old['catalog'])
            assert retained['packs'] == old['packs']
            previous_plan = self.read_metadata('physical-plans/'+old['plan']+'.json', 4*1024**2, old['plan'])
            work(previous_plan)
            assert previous_plan['generation']['id'] == old['id'] and previous_plan['generation']['catalog'] == old['catalog']
        for j in active['jobs']:
            for ext, size in [('pack', j['descriptor']['pack_bytes']), ('idx', j['descriptor']['index_bytes'])]:
                name = container_path(j['descriptor'], ext)
                self.required.add(name)
                assert (self.root/name).stat().st_size == size
        return dict(schema_pin=SCHEMA_PIN, version=3, selected_generation=b,
                    retained_generations=len(active['older']), immutable_sealed_pin_roots=len(base['older']),
                    pending_loose_jobs=len(active['jobs']), retired_container_jobs=len(active['retired']),
                    pending_garbage=len(active['garbage']), observed_removed_bytes=active['removed_bytes'],
                    observed_removed_files=active['removed_files'], counters_are_not_admission_credit=True,
                    required_paths=sorted(self.required), quiescent_native_lock_state_unmeasured=True)

    def raw(self, key):
        if self.config['version'] != 3:
            return super().raw(key)
        if key not in self.entries:
            return legacy.bounded(self.root/key, 128*1024**2)
        d, offset, encoded, length, h = self.entries[key]
        with (self.root/container_path(d, 'pack')).open('rb') as source:
            assert source.seek(0, 2) == d['pack_bytes']
            source.seek(offset)
            data = source.read(encoded)
        assert len(data) == encoded
        if d['codec']:
            if self.zstd is None:
                self.zstd = legacy.Zstd()
            data = self.zstd.decode(data, length)
        assert len(data) == length and digest(data) == h
        return data

    def verify_packs(self):
        if self.config['version'] != 3:
            return super().verify_packs()
        for d in self.descriptors:
            h = hashlib.sha256()
            with (self.root/container_path(d, 'pack')).open('rb') as source:
                for block in iter(lambda: source.read(1024**2), b''):
                    h.update(block)
            assert h.hexdigest() == d['pack']
        return [dict(role=legacy.role(key), key=key, raw_length=len(raw), sha256=digest(raw))
                for key in sorted(self.entries) for raw in [self.raw(key)]]


def verify(root, output, inventory=None, baseline=None, *, quiescent=False, known_requests=None):
    assert quiescent, 'explicit-quiescent-verifier-contract-required'
    started = time.monotonic()
    d = Decoder(root, quiescent=True)
    rows = d.verify_packs()
    if inventory:
        assert rows == legacy.expected_inventory(inventory), 'complete-raw-inventory-mismatch'
    sys.path.insert(0, str(PRIOR/'file-native-recovery-reclamation-2026-10-19'))
    import verify_operational
    verify_operational.checked = d.checked
    semantic = verify_operational.verify(root, baseline)
    sys.path.insert(0, str(PRIOR/'file-native-navigation-2026-10-19'))
    import oracle
    oracle.checked = d.checked
    navigation = oracle.verify(root)
    from outcome_oracle import verify as intents
    request_intents = intents(root, decoder=d, known_requests=known_requests)
    inventory_hash = hashlib.sha256()
    for row in rows:
        inventory_hash.update((json.dumps(row, separators=(',', ':'))+'\n').encode())
    result = dict(schema_pin=SCHEMA_PIN, raw_records=len(rows), raw_bytes=sum(r['raw_length'] for r in rows),
                  inventory_digest=inventory_hash.hexdigest(),
                  physical_metadata=d.physical_metadata, semantic=semantic, navigation=navigation,
                  request_intents=request_intents, independent_codec=d.zstd.binding if d.zstd else None,
                  seconds=time.monotonic()-started,
                  qualification='quiescent complete selected representation; native pin contention/liveness require separate schedules; no paged-format qualification')
    Path(output).write_text(json.dumps(result, indent=2)+'\n', encoding='utf-8')
    return result


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--root', required=True)
    parser.add_argument('--output', required=True)
    parser.add_argument('--inventory')
    parser.add_argument('--baseline')
    parser.add_argument('--known-requests')
    parser.add_argument('--quiescent', action='store_true')
    args = parser.parse_args()
    known = json.loads(Path(args.known_requests).read_bytes()) if args.known_requests else None
    answer = verify(args.root, args.output, args.inventory, args.baseline,
                    quiescent=args.quiescent, known_requests=known)
    print(json.dumps(dict(status='passed', actual_H=answer['semantic']['publications'],
                          records=answer['raw_records'], schema_pin=SCHEMA_PIN)))
