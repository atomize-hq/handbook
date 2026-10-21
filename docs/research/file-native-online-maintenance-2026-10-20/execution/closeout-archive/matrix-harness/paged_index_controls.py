"""Independent, tiny FNPIDX02 byte/tree negatives; no engine or real fixtures."""
import argparse
import copy
import hashlib
import json
import os
from pathlib import Path
import struct
import sys
import paged_index as pages
import physical_decoder_v5 as physical
from job_guard import sha_file, write_json
from matrix_budget import check_matrix

parser = argparse.ArgumentParser()
parser.add_argument('--root',required=True)
parser.add_argument('--output',required=True)
parser.add_argument('--core-sha256',required=True)
parser.add_argument('--decoder-sha256',required=True)
args = parser.parse_args()
assert os.environ.get('FILE_NATIVE_STAGE_JOB') == '1' and sys.dont_write_bytecode
assert sha_file(pages.__file__) == args.core_sha256
assert sha_file(physical.__file__) == args.decoder_sha256
check_matrix({'protocol':dict(bytes=2*1024**2,files=300),'receipts':dict(bytes=512*1024,files=10)})
root,output = Path(args.root).resolve(),Path(args.output).resolve()
runtime = Path('C:/file-native-online-matrix-20261020').resolve()
assert root.is_relative_to(runtime/'protocol'), 'synthetic-root-outside-owned-protocol'
assert output.is_relative_to(runtime/'receipts'), 'receipt-outside-owned-receipts'
assert not output.is_relative_to(runtime/'receipts/stream-control-main'), 'reserved-stream-prefix'
assert not root.exists() and not output.exists()
root.mkdir()
rows,raw_bindings = [],[]


def fixture(n,payload=b'raw'):
    """Independent little-endian fixture encoder, not an engine builder import."""
    keys = [f'results/{i*2:064x}.json' for i in range(n)]
    pack = payload*n
    d = dict(pack=hashlib.sha256(pack).hexdigest(),pack_bytes=len(pack),codec=0,records=n,
        role='result',first=keys[0],last=keys[-1],segment=1,instance='synthetic-paged')
    nodes = [dict(lo=i,hi=min(i+48,n),height=0,children=[]) for i in range(0,n,48)]
    while len(nodes)>1:
        nodes = [dict(lo=group[0]['lo'],hi=group[-1]['hi'],height=group[0]['height']+1,children=group)
                 for group in [nodes[i:i+48] for i in range(0,len(nodes),48)]]
    tree = nodes[0]
    next_offset = 0
    def assign(node):
        nonlocal next_offset
        node['offset'] = next_offset
        node['length'] = 78+len(keys[node['lo']])+len(keys[node['hi']-1])
        node['length'] += (node['hi']-node['lo'])*(len(keys[0])+50) if not node['children'] else len(node['children'])*(len(keys[0])+66)
        next_offset += node['length']
        for child in node['children']:assign(child)
    assign(tree)
    data = bytearray(next_offset)
    def key(value):return struct.pack('<H',len(value))+value.encode('ascii')
    def emit(node):
        for child in node['children']:emit(child)
        lo,hi = node['lo'],node['hi']
        b = bytearray(b'FNPIDX02'+bytes([node['height'],1,0,0]))
        b += struct.pack('<IHQQ',hi-lo,len(node['children']) or hi-lo,lo*len(payload),hi*len(payload))
        b += bytes.fromhex(d['pack'])+struct.pack('<Q',len(pack))+key(keys[lo])+key(keys[hi-1])
        if node['children']:
            for child in node['children']:
                b += key(keys[child['lo']])+struct.pack('<QI',child['offset'],child['length'])
                b += bytes.fromhex(child['hash'])+struct.pack('<IQQ',child['hi']-child['lo'],child['lo']*len(payload),child['hi']*len(payload))
        else:
            for i in range(lo,hi):
                b += key(keys[i])+struct.pack('<QII',i*len(payload),len(payload),len(payload))+hashlib.sha256(payload).digest()
        assert len(b) == node['length']
        node['hash'] = hashlib.sha256(b).hexdigest()
        data[node['offset']:node['offset']+node['length']] = b
    emit(tree)
    d.update(index=hashlib.sha256(data).hexdigest(),index_bytes=len(data),
        page=dict(version=2,hash=tree['hash'],bytes=tree['length'],height=tree['height']))
    return bytes(data),d,tree,pack


def save(label,data,d):
    path = root/(label+'.idx')
    with path.open('xb') as stream:stream.write(data)
    descriptor = root/(label+'.descriptor.json')
    write_json(descriptor,d)
    raw_bindings.append(dict(case=label,path=str(path),sha256=sha_file(path),bytes=len(data),descriptor=str(descriptor),descriptor_sha256=sha_file(descriptor)))
    assert sum(r['bytes'] for r in raw_bindings) < 2*1024**2


def rebound(data,d,root_length=None):
    d = copy.deepcopy(d)
    d['index_bytes'],d['index'] = len(data),hashlib.sha256(data).hexdigest()
    if root_length is not None:d['page']['bytes'] = root_length
    d['page']['hash'] = hashlib.sha256(data[:d['page']['bytes']]).hexdigest()
    return d


def refuses(label,call):
    try:call()
    except (AssertionError,ValueError,KeyError,TypeError,UnicodeError,struct.error) as error:
        rows.append(dict(case=label,refused=True,error_type=type(error).__name__,reason=str(error)))
    else:raise AssertionError('negative-was-accepted:'+label)


def negative(label,data,d):
    save(label,data,d)
    refuses(label,lambda:pages.full(data,d))


positives = []
for n in [1,48,49,2400]:
    data,d,tree,pack = fixture(n)
    save('positive-'+str(n),data,d)
    full = pages.full(data,d)
    assert len(full['entries']) == n and full['whole_index_and_tree_authenticated']
    selected = sorted(set([0,min(47,n-1),min(48,n-1),n-1]))
    for i in selected:
        hit = pages.route(data,d,f'results/{i*2:064x}.json')
        absent = pages.route(data,d,f'results/{i*2+1:064x}.json')
        assert hit['entry'][0] == f'results/{i*2:064x}.json' and absent['entry'] is None
        assert hit['pages'] == tree['height']+1 and hit['index_bytes'] <= 3*16384
    positives.append(dict(records=n,height=tree['height'],pages=full['pages'],index_bytes=len(data)))
height2_data,height2_d,height2_tree,_ = fixture(2400)
four = [pages.route(height2_data,height2_d,f'results/{2400:064x}.json') for _ in range(4)]
assert sum(r['pages'] for r in four) == 12 and sum(r['index_bytes'] for r in four) <= 192*1024

leaf,d,tree,_ = fixture(2)
for label,at,fmt,value in [
    ('height',8,'B',3),('role',9,'B',4),('codec',10,'B',1),('reserved',11,'B',1),
    ('subtree-count',12,'I',3),('zero-slots',16,'H',0),('49-slots',16,'H',49),
    ('pack-start',18,'Q',1),('pack-end',26,'Q',5),('pack-length',66,'Q',7),
    ('empty-first-key',74,'H',0),('oversized-first-key',74,'H',226)]:
    changed=bytearray(leaf);struct.pack_into('<'+fmt,changed,at,value)
    negative('leaf-'+label,bytes(changed),rebound(changed,d))
for label,at in [('magic',0),('pack-hash',34),('nonascii-first-key',76)]:
    changed=bytearray(leaf);changed[at]=255
    negative('leaf-'+label,bytes(changed),rebound(changed,d))
header = 78+len(d['first'])+len(d['last'])
record_offset = header+2+len(d['first'])
for label,at,fmt,value in [('record-offset',record_offset,'Q',1),
    ('raw-codec-length',record_offset+8,'I',2),('raw-length-bound',record_offset+12,'I',8*1024**2+1)]:
    changed=bytearray(leaf);struct.pack_into('<'+fmt,changed,at,value)
    negative('leaf-'+label,bytes(changed),rebound(changed,d))
changed=bytearray(leaf);second=header+len(d['first'])+50+2
changed[second:second+len(d['first'])]=d['first'].encode()
negative('leaf-duplicate-key',bytes(changed),rebound(changed,d))
negative('leaf-truncated',leaf[:-1],rebound(leaf[:-1],d,len(leaf)-1))
negative('leaf-padding',leaf+b'X',rebound(leaf+b'X',d,len(leaf)+1))
negative('index-trailing-byte',leaf+b'X',rebound(leaf+b'X',d))
negative('page-over16KiB',leaf+b'X'*16384,rebound(leaf+b'X'*16384,d,len(leaf)+16384))
for label,field,value in [('root-version','version',3),('root-height-bool','height',True),
    ('root-bytes-bool','bytes',True),('root-unknown-field','unexpected',0)]:
    changed=copy.deepcopy(d);changed['page'][field]=value
    negative(label,leaf,changed)
for label,key in [('typed-too-long-id','publications/'+'a'*100+'.json'),
    ('typed-nonascii','publications/é.json'),('typed-wrong-result-hash','results/'+'a'*65+'.json'),
    ('typed-extra-path','attempts/a/extra/candidate.json')]:
    refuses(label,lambda key=key:pages.typed(key))
for key,role in [('publications/'+'a'*99+'.json','publication'),('results/'+'a'*64+'.json','result'),
    ('attempt-hints/'+'a'*99+'.json','hint'),('attempts/'+'a'*99+'/candidate.json','candidate'),
    ('attempts/'+'a'*99+'/allocations/'+'b'*99+'.json','allocation')]:
    assert pages.typed(key,role) == key

branch,bd,bt,_ = fixture(49)
branch_header = 78+len(bd['first'])+len(bd['last'])
child_offset = branch_header+2+len(bd['first'])
for label,at,fmt,value in [('child-alias',child_offset,'Q',0),('child-gap',child_offset,'Q',bt['children'][0]['offset']+1),
    ('child-zero-length',child_offset+8,'I',0),('child-over16KiB',child_offset+8,'I',16385),
    ('child-span-gap',child_offset+48,'Q',1),('child-count-zero',child_offset+44,'I',0)]:
    changed=bytearray(branch);struct.pack_into('<'+fmt,changed,at,value)
    negative(label,bytes(changed),rebound(changed,bd))
changed=bytearray(branch);changed[child_offset+12]^=1
negative('child-page-hash',bytes(changed),rebound(changed,bd))
first_child = bt['children'][0]
changed=bytearray(branch);last_pos=first_child['offset']+76+len(bd['first'])+2
next_first=f"results/{96:064x}.json".encode()
changed[last_pos:last_pos+len(next_first)]=next_first
child_hash=hashlib.sha256(changed[first_child['offset']:first_child['offset']+first_child['length']]).digest()
changed[child_offset+12:child_offset+44]=child_hash
negative('child-crosses-sibling-upper-fence',bytes(changed),rebound(changed,bd))

# Untouched corruption is permitted by a route proof and refused by full audit.
last_child = bt['children'][-1]
changed=bytearray(branch);changed[last_child['offset']]^=1
save('untouched-child-corruption',bytes(changed),bd)
assert pages.route(changed,bd,bd['first'])['entry'] is not None
refuses('touched-corrupt-child-route',lambda:pages.route(changed,bd,bd['last']))
refuses('untouched-corrupt-child-full-file-hash',lambda:pages.full(changed,bd))
changed_d=copy.deepcopy(bd);changed_d['index']=hashlib.sha256(changed).hexdigest()
negative('untouched-corrupt-child-full-page-hash',bytes(changed),changed_d)

# Actual tiny pack/index files exercise catalog4, full audit and mapped variants.
descriptors=[]
data,descriptor,_,pack = fixture(1)
for n in range(1,6):
    desc={**copy.deepcopy(descriptor),'instance':'synthetic-paged-'+str(n),'segment':n}
    folder=root/'packs'/desc['instance'];folder.mkdir(parents=True)
    (folder/(desc['index']+'.idx')).write_bytes(data)
    (folder/(desc['pack']+'.pack')).write_bytes(pack)
    descriptors.append(desc)

class ByteControl(physical.Decoder):
    def __init__(self):
        self.root=root;self.config=dict(version=3,store_id='synthetic-store')
        self.required=set();self.entries={};self.descriptors=[];self.zstd=None
        self.cat=dict(version=4,store_id='synthetic-store',packs=descriptors[:4])
    def read_metadata(self,*unused):return self.cat

reader=ByteControl();reader.descriptors=reader.catalog('unused')['packs']
for desc in reader.descriptors:reader.load_index(desc)
assert len(reader.verify_packs()) == 1 and len(reader.variants[descriptor['first']]) == 4
reader.cat['packs']=descriptors
refuses('catalog-fifth-segment',lambda:reader.catalog('unused'))
reader.cat['packs']=[descriptors[0],{**descriptors[0],'segment':2}]
refuses('catalog-physical-instance-alias',lambda:reader.catalog('unused'))
for version in [2,3]:
    reader.cat=dict(version=version,store_id='synthetic-store',packs=descriptors[:1])
    refuses('paged-descriptor-in-catalog'+str(version),lambda:reader.catalog('unused'))
loose=root/descriptor['first'];loose.parent.mkdir();loose.write_bytes(pack)
bad_pack=root/'packs'/descriptors[3]['instance']/(descriptor['pack']+'.pack')
bad_pack.write_bytes(b'bad')
refuses('corrupt-fourth-body-despite-good-first-and-loose',lambda:reader.raw(descriptor['first']))
refuses('full-pack-hash-corruption',lambda:reader.verify_packs())
(root/'preserved-corrupt-pack.bin').write_bytes(b'bad');bad_pack.write_bytes(pack)
assert reader.raw(descriptor['first']) == pack
conflicting,cd,_,cp=fixture(1,b'bad');cd.update(instance='synthetic-conflict',segment=6)
folder=root/'packs'/cd['instance'];folder.mkdir();(folder/(cd['index']+'.idx')).write_bytes(conflicting)
(folder/(cd['pack']+'.pack')).write_bytes(cp)
refuses('conflicting-mapped-raw-identity',lambda:reader.load_index(cd))

# A legacy FNPIDX01 descriptor can coexist with paged ones in catalog4.
key=descriptor['first'].encode()
legacy=(b'FNPIDX01'+bytes.fromhex(descriptor['pack'])+struct.pack('<QBI',len(pack),0,1)
    +struct.pack('<H',len(key))+key+struct.pack('<QII',0,len(pack),len(pack))+hashlib.sha256(pack).digest())
ld={k:v for k,v in descriptor.items() if k not in ['page','instance','segment']}
ld.update(index=hashlib.sha256(legacy).hexdigest(),index_bytes=len(legacy))
(root/'packs'/(ld['index']+'.idx')).write_bytes(legacy)
(root/'packs'/(ld['pack']+'.pack')).write_bytes(pack)
mixed=ByteControl();mixed.cat['packs']=[ld]+descriptors[:3]
mixed.descriptors=mixed.catalog('unused')['packs']
for desc in mixed.descriptors:mixed.load_index(desc)
assert len(mixed.verify_packs()) == 1 and mixed.raw(descriptor['first']) == pack

# Page-bearing descriptors pass through generation, work jobs/retired, journal.
work=dict(version=2,store_id='synthetic-store',generation=dict(id='synthetic-paged-1',ordinal=1,
    catalog='a'*64,plan=None,packs=copy.deepcopy(descriptors[:1])),older=[],
    jobs=[dict(descriptor=copy.deepcopy(descriptors[0]),frontier=1,position=0)],
    retired=[dict(descriptor=copy.deepcopy(descriptors[0]),pack_done=False,index_done=False)],garbage=[],
    removed_bytes=0,removed_files=0,reconciled_absent=0,physical_parent='b'*64,
    transition_parent=None,birth_claims=[],discovery=None)
physical.work2(work)
for container in ['generation','jobs','retired']:
    changed=copy.deepcopy(work)
    desc=changed['generation']['packs'][0] if container=='generation' else changed[container][0]['descriptor']
    desc['page']['unknown']=0
    refuses('unknown-page-field-in-'+container,lambda changed=changed:physical.work2(changed))
(root/'PACKING.json').write_bytes(b'journal-presence-only')
class JournalControl(physical.Decoder):
    def __init__(self):
        self.root=root;self.config=dict(physical=dict(id='synthetic-paged-1'));self.required=set()
        self.j=dict(version=2,id='synthetic-paged-1',status='complete',expected_config='a'*64,
            catalog='b'*64,old=descriptors[:1],reserved_bytes=0,used_bytes=0,deleted_bytes=0,deferred_bytes=0)
    def read_metadata(self,*unused):return self.j
journal=JournalControl();assert journal.journal()['selected_by_STORE']
journal.j['old']=[{**copy.deepcopy(descriptors[0]),'page':{**descriptors[0]['page'],'unknown':0}}]
refuses('unknown-page-field-in-journal',lambda:journal.journal())

assert sha_file(pages.__file__) == args.core_sha256 and sha_file(physical.__file__) == args.decoder_sha256
write_json(output,dict(status='passed',source_pin=pages.SOURCE_PIN,core_sha256=args.core_sha256,
    decoder_sha256=args.decoder_sha256,positive_trees=positives,negative_cases=rows,raw_fixture_bindings=raw_bindings,
    four_height2_routes_pages=12,four_height2_routes_index_bytes=sum(r['index_bytes'] for r in four),
    four_route_index_byte_cap=192*1024,
    mixed_legacy_paged_catalog=True,all_four_mapped_duplicate_bodies_verified=True,
    work_and_journal_descriptor_closure=True,point_untouched_corruption_deferred_full_audit_refused=True,
    no_engine=True,no_real_store_reads=True,
    qualification='independent synthetic byte/control tests; no Rust allocation/performance or native-lifetime qualification'))
print(json.dumps(dict(status='passed',negative_cases=len(rows),positive_trees=len(positives))))
