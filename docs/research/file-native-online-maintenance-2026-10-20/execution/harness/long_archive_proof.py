"""Read-only FNOP1 stream proof of the EXACT retained H101007 head.

Two authenticated bounded-memory passes; no store restoration or corpus copy.
This proves the source predecessor/input bridge, not the final candidate oracle.
"""
import gzip
import hashlib
import json
import os
from pathlib import Path
import struct
import sys
from job_guard import Budget, sha_file, write_json
from outcome_oracle import intent, strict
from baseline import RESEARCH
sys.path.insert(0, str(RESEARCH/'file-native-packing-2026-10-20'))
from independent import Zstd, role

R = Path('C:/file-native-packing-20261020')
ARCHIVE = R/'packed-operational-backup'
OUT = Path('C:/file-native-online-matrix-20261020/receipts/long-exact-archive-head-proof.json')
Budget(os.environ['FILE_NATIVE_BUDGET']).check({'receipts': dict(bytes=1024**2, files=5)})
manifest = strict((ARCHIVE/'manifest.json').read_bytes())
assert manifest['logical_bytes'] == 592458971 and manifest['files'] == 5807
assert manifest['transport_sha256'] == '2fabd1c2439fc95a37a0411c99025bfb5646dee855b6ed0e879fd190e35e324b'
assert sha_file(ARCHIVE/'files.jsonl') == manifest['inventory_sha256']
assert sha_file(ARCHIVE/'operational.bin.gz') == manifest['transport_sha256']
build_path = R/'long-matrix-fixed2/PZ-build.json'
assert sha_file(build_path) == 'a54a04a4fdc8dad85dee038502976886b015ed5b53bb1b8fbcf32dcffe8ca1ad'
build = strict(build_path.read_bytes())
frozen = strict((R/'long-matrix/frozen.json').read_bytes())
inputs = strict(Path('C:/file-native-online-matrix-20261020/receipts/long-intent-read-only-plan.json').read_bytes())
assert manifest['selected'] == frozen['selected'] and frozen['measurement_H'] == 101007
target_key = 'publications/'+manifest['selected']['publication']+'.json'
state = {}

def take(source, count):
    data = source.read(count)
    assert len(data) == count, 'truncated-FNOP1'
    return data


def scan(selector, visitor):
    count = total = 0
    with gzip.open(ARCHIVE/'operational.bin.gz', 'rb') as source, (ARCHIVE/'files.jsonl').open('rb') as inventory:
        assert take(source, 6) == b'FNOP1\n'
        for line in inventory:
            assert len(line) <= 4096
            row = strict(line)
            n, size = struct.unpack('<IQ', take(source, 12))
            assert 0 < n <= 4096 and 0 <= size <= 512*1024**2
            name = take(source, n).decode()
            assert name == row['path'] and size == row['bytes']
            region = selector(name, size)
            kept = bytearray()
            h = hashlib.sha256()
            position = 0
            while position < size:
                block = take(source, min(65536, size-position))
                h.update(block)
                if region is not None:
                    start, end = region
                    lo, hi = max(start, position), min(end, position+len(block))
                    if lo < hi:
                        kept.extend(block[lo-position:hi-position])
                position += len(block)
            assert h.digest() == take(source, 32) and h.hexdigest() == row['sha256']
            if region is not None:
                assert len(kept) == region[1]-region[0]
                visitor(name, bytes(kept))
            count += 1
            total += size
        assert not source.read(1)
    assert (count, total) == (manifest['files'], manifest['logical_bytes'])
    return dict(files=count, logical_bytes=total, complete_gzip_eof=True, all_embedded_file_hashes_verified=True)


def first_select(name, size):
    wanted = name in ['STORE.json', 'SELECTED.json'] or name == state.get('catalog_name') or name == state.get('index_name')
    if wanted:
        assert size <= (16*1024**2 if name.endswith('.idx') else 65536)
        return (0, size)
    return None


def first_visit(name, raw):
    if name == 'STORE.json':
        config = strict(raw)
        assert config['version'] == 2
        state['config'] = config
        state['catalog_name'] = 'pack-catalogs/'+config['catalog']+'.json'
    elif name == 'SELECTED.json':
        assert strict(raw) == manifest['selected']
    elif name == state.get('catalog_name'):
        assert hashlib.sha256(raw).hexdigest() == state['config']['catalog']
        catalog = strict(raw)
        assert catalog == build['catalog'] and catalog['version'] == 1
        assert sum(d['records'] for d in catalog['packs']) == 504833
        matches = [d for d in catalog['packs'] if d['role'] == 'publication' and d['first'] <= target_key <= d['last']]
        assert len(matches) == 1
        state['descriptor'] = matches[0]
        state['index_name'] = 'packs/'+matches[0]['index']+'.idx'
    elif name == state.get('index_name'):
        d = state['descriptor']
        assert len(raw) == d['index_bytes'] and hashlib.sha256(raw).hexdigest() == d['index']
        assert raw[:8] == b'FNPIDX01' and raw[8:40].hex() == d['pack']
        assert struct.unpack_from('<QBI', raw, 40) == (d['pack_bytes'], d['codec'], d['records'])
        pos = 53
        previous = ''
        end = 0
        first = None
        for _ in range(d['records']):
            n = struct.unpack_from('<H', raw, pos)[0]
            pos += 2
            assert n <= 512 and pos+n+48 <= len(raw)
            key = raw[pos:pos+n].decode()
            pos += n
            offset, encoded, length = struct.unpack_from('<QII', raw, pos)
            pos += 16
            h = raw[pos:pos+32].hex()
            pos += 32
            assert previous < key and role(key) == d['role'] and offset == end
            assert length <= 8*1024**2 and encoded <= 8*1024**2+65536 and offset+encoded <= d['pack_bytes']
            assert d['codec'] != 0 or length == encoded
            if key == target_key:
                state['entry'] = dict(offset=offset, encoded=encoded, raw=length, sha256=h)
            first = key if first is None else first
            previous, end = key, offset+encoded
        assert pos == len(raw) and end == d['pack_bytes'] and first == d['first'] and previous == d['last']
        assert 'entry' in state


first_pass = scan(first_select, first_visit)
entry = state['entry']
pack_name = 'packs/'+state['descriptor']['pack']+'.pack'

def second_select(name, size):
    if name == pack_name:
        assert size == state['descriptor']['pack_bytes']
        return (entry['offset'], entry['offset']+entry['encoded'])
    return None


def second_visit(name, data):
    assert name == pack_name
    if state['descriptor']['codec']:
        codec = Zstd()
        data = codec.decode(data, entry['raw'])
        state['codec'] = codec.binding
    assert len(data) == entry['raw'] and hashlib.sha256(data).hexdigest() == entry['sha256'] == manifest['selected']['digest']
    state['head'] = strict(data)


second_pass = scan(second_select, second_visit)
head = state['head']
original = inputs['protected_controls_before']['SELECTED.json']
source_selected = strict((R/'post-gc-operational-backup/manifest.json').read_bytes())['selected']
assert head['id'] == manifest['selected']['publication'] and head['sequence'] == 101006
assert head['predecessor'] == source_selected and head['request'] == 'P-navigation-upgrade'
nav_input = inputs['known_nonhistory_requests']['P-navigation-upgrade']
assert nav_input['base'] == source_selected and intent(nav_input) == head['intent']
events = strict((R/'long-matrix/prep-events/events.json').read_bytes())
event = next(e for e in events if e['input']['action'] == 'publish' and e['input']['request']['request'] == head['request'] and e['exit'] == 0)
reply = strict(event['stdout'])['result']
assert reply['selected'] == manifest['selected'] and reply['status'] == 'published'
result_bytes = json.dumps(reply['result'], sort_keys=True, indent=2, ensure_ascii=False).encode()
assert hashlib.sha256(result_bytes).hexdigest() == head['result_hash'] and reply['result']['outcome'] == 'accepted'
lost_proof = next(p for p in inputs['validated_source_nonhistory_proofs'] if p['request'] == 'deep-lost-reply')
source_root = Path(inputs['source'])
lost_pub_bytes = (source_root/f"publications/{lost_proof['selected']['publication']}.json").read_bytes()
assert hashlib.sha256(lost_pub_bytes).hexdigest() == lost_proof['selected']['digest']
lost_pub = strict(lost_pub_bytes)
lost_result_bytes = (source_root/f"results/{lost_pub['result_hash']}.json").read_bytes()
assert hashlib.sha256(lost_result_bytes).hexdigest() == lost_pub['result_hash']
assert strict(lost_result_bytes) == frozen['expected_outcomes']['deep-lost-reply']
assert intent(inputs['known_nonhistory_requests']['deep-lost-reply']) == lost_pub['intent']
write_json(OUT, dict(status='exact-H101007-archive-head-and-input-bridge-proved',
    archive=str(ARCHIVE), archive_sha256=manifest['transport_sha256'], archive_inventory_sha256=manifest['inventory_sha256'],
    archive_logical_bytes=592458971, actual_H=101007, packed_records=504833,
    original_PZ_build_receipt=str(build_path), original_PZ_build_sha256=sha_file(build_path),
    exact_archive_selected=manifest['selected'], authenticated_head=head,
    exact_head_predecessor_is_H101006=source_selected, final_accepted_input=nav_input,
    final_accepted_input_intent=head['intent'], final_accepted_result=reply['result'],
    lost_reply_input=inputs['known_nonhistory_requests']['deep-lost-reply'],
    lost_reply_selector=lost_proof['selected'], lost_reply_intent=lost_pub['intent'],
    lost_reply_result_exact_original_category=True, original_six_lookup_requests=frozen['original_lookup_requests'],
    first_authenticated_FN0P1_pass=first_pass, second_authenticated_FN0P1_pass=second_pass,
    independent_codec=state.get('codec'), original_ledger_bytes=246560316,
    no_engine_calls=True, no_restore=True, no_corpus_copy=True, no_long_mutation_lane_taken=True,
    qualification='exact retained cohort/ancestor/input proof only; full final-pin chosen representation oracle and backup/restore remain required'))
print(json.dumps(dict(status='exact-H101007-bridge-proved', actual_H=101007, records=504833, head_predecessor=source_selected)))
