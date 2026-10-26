"""Independent FNPIDX02 byte decoder, pinned to CP3 f9b762a1.

Full audit authenticates the entire immutable file and its exact preorder tree.
Route audit authenticates only the root and chosen path, as a separate contract.
This Python checker's memory use is not evidence of Rust reader allocations.
"""
import hashlib
import re
import struct

SOURCE_PIN = 'f9b762a1a0d352564542bb4850ad77ee14a7c6e4'
PAGE = 16384
SLOTS = 48
ROLES = ['publication','result','hint','candidate','allocation']
ID = r'[A-Za-z0-9_-]{1,99}'
PATTERNS = [rf'publications/{ID}\.json',r'results/[0-9a-f]{64}\.json',
    rf'attempt-hints/{ID}\.json',rf'attempts/{ID}/candidate\.json',
    rf'attempts/{ID}/allocations/{ID}\.json']


def uint(value, maximum):
    assert type(value) is int and 0 <= value <= maximum, 'paged-unsigned-bound'
    return value


def typed(key, role=None):
    assert type(key) is str and key.isascii() and 0 < len(key) <= 225, 'paged-key-bound'
    matches = [name for name,pattern in zip(ROLES,PATTERNS) if re.fullmatch(pattern,key)]
    assert len(matches) == 1 and (role is None or matches[0] == role), 'paged-key-role'
    return key


def root_descriptor(d):
    p = d['page']
    assert type(p) is dict and set(p) == {'version','hash','bytes','height'}, 'paged-root-shape'
    assert type(p['version']) is int and p['version'] == 2, 'paged-root-version'
    assert type(p['hash']) is str and re.fullmatch('[0-9a-f]{64}',p['hash']), 'paged-root-hash'
    assert uint(p['height'],2) <= 2 and 0 < uint(p['bytes'],PAGE) <= d['index_bytes'], 'paged-root-bound'
    assert 0 < uint(d['records'],65536) and 0 < uint(d['index_bytes'],16*1024**2), 'paged-index-bound'
    typed(d['first'],d['role']);typed(d['last'],d['role'])
    return dict(offset=0,length=p['bytes'],hash=p['hash'],height=p['height'],first=d['first'],
                records=d['records'],start=0,end=d['pack_bytes'])


class Cursor:
    def __init__(self,data):
        self.data = data
        self.pos = 0

    def take(self,n):
        assert n >= 0 and self.pos+n <= len(self.data), 'paged-truncated'
        answer = self.data[self.pos:self.pos+n]
        self.pos += n
        return answer

    def number(self,fmt):
        return struct.unpack('<'+fmt,self.take(struct.calcsize('<'+fmt)))[0]

    def key(self,role):
        n = self.number('H')
        assert 0 < n <= 225, 'paged-key-length'
        return typed(self.take(n).decode('ascii'),role)


def page(data,d,expected,upper=None,last=None):
    offset,length = expected['offset'],expected['length']
    assert 0 < length <= PAGE and 0 <= offset <= len(data)-length, 'paged-page-extent'
    raw = data[offset:offset+length]
    assert hashlib.sha256(raw).hexdigest() == expected['hash'], 'paged-page-hash'
    c = Cursor(raw)
    assert c.take(8) == b'FNPIDX02', 'paged-magic'
    height,role,codec,reserved = [c.number('B') for _ in range(4)]
    records,slots = c.number('I'),c.number('H')
    start,end = c.number('Q'),c.number('Q')
    pack_hash,pack_bytes = c.take(32).hex(),c.number('Q')
    first,final = c.key(d['role']),c.key(d['role'])
    assert height == expected['height'] and height <= 2, 'paged-height'
    assert role == ROLES.index(d['role']) and codec == d['codec'] and reserved == 0, 'paged-role-codec-reserved'
    assert records == expected['records'] and 0 < slots <= SLOTS, 'paged-count'
    assert start == expected['start'] and end == expected['end'] and 0 <= start <= end <= d['pack_bytes'], 'paged-parent-span'
    assert pack_hash == d['pack'] and pack_bytes == d['pack_bytes'], 'paged-pack-binding'
    assert first == expected['first'] and first <= final, 'paged-first-fence'
    assert upper is None or final < upper, 'paged-upper-fence'
    assert last is None or final == last, 'paged-last-fence'
    entries,children = [],[]
    previous,position,total = '',start,0
    for _ in range(slots):
        key = c.key(d['role'])
        assert previous < key and first <= key <= final, 'paged-key-order'
        if height == 0:
            record_offset,encoded,raw_length = c.number('Q'),c.number('I'),c.number('I')
            digest = c.take(32).hex()
            assert record_offset == position and raw_length <= 8*1024**2 and encoded <= 8*1024**2+65536, 'paged-record-bound'
            assert d['codec'] != 0 or encoded == raw_length, 'paged-raw-codec-size'
            position = record_offset+encoded
            assert position <= end, 'paged-record-span'
            entries.append((key,record_offset,encoded,raw_length,digest))
            total += 1
        else:
            child_offset,child_length = c.number('Q'),c.number('I')
            child_hash = c.take(32).hex()
            child_count,lo,hi = c.number('I'),c.number('Q'),c.number('Q')
            assert 0 < child_length <= PAGE and child_offset+child_length <= d['index_bytes'], 'paged-child-extent'
            assert 0 < child_count <= SLOTS**height and lo == position and lo <= hi <= end, 'paged-child-span-count'
            children.append(dict(offset=child_offset,length=child_length,hash=child_hash,height=height-1,
                first=key,records=child_count,start=lo,end=hi))
            position,total = hi,total+child_count
        previous = key
    assert c.pos == len(raw) and position == end and total == records, 'paged-exact-page-coverage'
    assert (entries[0][0] == first and previous == final) if height == 0 else children[0]['first'] == first, 'paged-slot-fences'
    return dict(first=first,last=final,entries=entries,children=children,length=length)


def full(data,d):
    expected = root_descriptor(d)
    assert len(data) == d['index_bytes'] and hashlib.sha256(data).hexdigest() == d['index'], 'paged-whole-index-hash-size'
    entries = []
    pending = [(expected,None,d['last'])]
    next_offset,pages = 0,0
    while pending:
        reference,upper,last = pending.pop()
        assert reference['offset'] == next_offset, 'paged-preorder-alias-gap'
        node = page(data,d,reference,upper,last)
        next_offset += reference['length']
        pages += 1
        entries.extend(node['entries'])
        children = node['children']
        for n in range(len(children)-1,-1,-1):
            pending.append((children[n],children[n+1]['first'] if n+1 < len(children) else upper,
                            node['last'] if n+1 == len(children) else None))
    assert next_offset == len(data) and len(entries) == d['records'], 'paged-whole-tree-coverage'
    assert entries[0][0] == d['first'] and entries[-1][0] == d['last'], 'paged-global-fences'
    assert all(a[0] < b[0] for a,b in zip(entries,entries[1:])), 'paged-global-order'
    assert entries[0][1] == 0 and entries[-1][1]+entries[-1][2] == d['pack_bytes'], 'paged-global-pack-coverage'
    assert all(a[1]+a[2] == b[1] for a,b in zip(entries,entries[1:])), 'paged-global-pack-contiguity'
    return dict(entries=entries,pages=pages,index_bytes=len(data),whole_index_and_tree_authenticated=True)


def route(data,d,key):
    typed(key,d['role'])
    assert len(data) == d['index_bytes'], 'paged-route-file-size'
    expected = root_descriptor(d)
    upper,last = None,d['last']
    pages,index_bytes = 0,0
    while True:
        node = page(data,d,expected,upper,last)
        pages,index_bytes = pages+1,index_bytes+expected['length']
        assert pages <= 3 and index_bytes <= 3*PAGE, 'paged-route-bound'
        if not node['children']:
            answer = next((e for e in node['entries'] if e[0] == key),None)
            return dict(entry=answer,pages=pages,index_bytes=index_bytes,route_authenticated=True,
                        whole_index_authenticated=False)
        children = node['children']
        eligible = [n for n,child in enumerate(children) if child['first'] <= key]
        if not eligible:
            return dict(entry=None,pages=pages,index_bytes=index_bytes,route_authenticated=True,
                        whole_index_authenticated=False)
        n = eligible[-1]
        if n+1 < len(children):
            upper,last = children[n+1]['first'],None
        else:
            last = node['last']
        expected = children[n]
