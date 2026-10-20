"""Independent Python pack/index/catalog parser; never imports engine code.

Zstandard uses Git's separately installed native DLL through its public C API,
with fixed output and window bounds. Semantic checks reuse the existing separate
C1/operational walker above this independently implemented byte resolver.
"""
import argparse,bisect,ctypes,hashlib,json,os,struct,sys,time
from pathlib import Path
HERE=Path(__file__).resolve().parent
MAX_RAW=8*1024**2;MAX_PACK=64*1024**2;MAX_INDEX=16*1024**2
def sha(b):return hashlib.sha256(b).hexdigest()
def strict(b):
    def pairs(rows):
        d={}
        for k,v in rows:
            if k in d:raise ValueError('duplicate-key')
            d[k]=v
        return d
    return json.loads(b,object_pairs_hook=pairs)
def role(key):
    p=key.split('/')
    if '\\' in key or ':' in key or len(key.encode())>512 or any(x in ['','.','..'] for x in p):return None
    if len(p)==2 and p[0] in ['publications','results','attempt-hints'] and p[1].endswith('.json'):
        return {'publications':'publication','results':'result','attempt-hints':'hint'}[p[0]]
    if len(p)==3 and p[0]=='attempts' and p[2]=='candidate.json':return 'candidate'
    if len(p)==4 and p[0]=='attempts' and p[2]=='allocations' and p[3].endswith('.json'):return 'allocation'
    return None
class Zstd:
    def __init__(self):
        self.path=Path('C:/Program Files/Git/mingw64/bin/libzstd.dll') if os.name=='nt' else Path('libzstd.so.1')
        self.lib=ctypes.CDLL(str(self.path));z=self.lib
        z.ZSTD_versionString.restype=ctypes.c_char_p
        z.ZSTD_createDCtx.restype=ctypes.c_void_p;z.ZSTD_freeDCtx.argtypes=[ctypes.c_void_p]
        z.ZSTD_DCtx_setParameter.argtypes=[ctypes.c_void_p,ctypes.c_int,ctypes.c_int];z.ZSTD_DCtx_setParameter.restype=ctypes.c_size_t
        z.ZSTD_decompressDCtx.argtypes=[ctypes.c_void_p,ctypes.c_void_p,ctypes.c_size_t,ctypes.c_void_p,ctypes.c_size_t];z.ZSTD_decompressDCtx.restype=ctypes.c_size_t
        z.ZSTD_findFrameCompressedSize.argtypes=[ctypes.c_void_p,ctypes.c_size_t];z.ZSTD_findFrameCompressedSize.restype=ctypes.c_size_t
        z.ZSTD_isError.argtypes=[ctypes.c_size_t];z.ZSTD_isError.restype=ctypes.c_uint
        self.context=z.ZSTD_createDCtx();assert self.context
        assert not z.ZSTD_isError(z.ZSTD_DCtx_setParameter(self.context,100,23))
        self.binding=dict(version=z.ZSTD_versionString().decode(),path=str(self.path),sha256=sha(self.path.read_bytes()) if self.path.is_file() else None)
    def decode(self,data,length):
        assert length<=MAX_RAW and len(data)<=MAX_RAW+65536 and data[:4]==b'\x28\xb5\x2f\xfd'
        source=ctypes.create_string_buffer(data);destination=ctypes.create_string_buffer(max(1,length))
        n=self.lib.ZSTD_findFrameCompressedSize(source,len(data));assert not self.lib.ZSTD_isError(n) and n==len(data)
        n=self.lib.ZSTD_decompressDCtx(self.context,destination,length,source,len(data))
        assert not self.lib.ZSTD_isError(n) and n==length
        return destination.raw[:n]
    def __del__(self):
        if getattr(self,'context',None):self.lib.ZSTD_freeDCtx(self.context)
class Decoder:
    def __init__(self,root):
        self.root=Path(root);self.config=strict((self.root/'STORE.json').read_bytes());self.entries={};self.descriptors=[];self.zstd=None
        c=self.config
        assert c['version'] in [1,2]
        if c['version']==1:assert 'catalog' not in c;return
        h=c['catalog'];assert len(h)==64
        raw=(self.root/f'pack-catalogs/{h}.json').read_bytes();assert len(raw)<=65536 and sha(raw)==h
        cat=strict(raw);assert set(cat)=={'version','store_id','packs'} and cat['version']==1 and cat['store_id']==c['store_id'] and len(cat['packs'])<=64
        last=None
        for d in cat['packs']:
            assert set(d)=={'pack','index','pack_bytes','index_bytes','records','codec','role','first','last'}
            assert type(d['codec']) is int and d['codec'] in [0,1] and 0<d['records']<=65536 and 0<=d['pack_bytes']<=MAX_PACK and 53<=d['index_bytes']<=MAX_INDEX
            assert role(d['first'])==role(d['last'])==d['role'] and d['first']<=d['last']
            assert last is None or (last['role'],last['first'])<(d['role'],d['first']) and (last['role']!=d['role'] or last['last']<d['first'])
            last=d
            pack=self.root/f"packs/{d['pack']}.pack";assert pack.stat().st_size==d['pack_bytes']
            data=(self.root/f"packs/{d['index']}.idx").read_bytes();assert len(data)==d['index_bytes'] and sha(data)==d['index']
            assert data[:8]==b'FNPIDX01' and data[8:40].hex()==d['pack']
            length,codec,count=struct.unpack_from('<QBI',data,40);assert (length,codec,count)==(d['pack_bytes'],d['codec'],d['records'])
            pos=53;end=0;previous='';keys=[]
            for i in range(count):
                n,=struct.unpack_from('<H',data,pos);pos+=2;assert n<=512
                key=data[pos:pos+n].decode();pos+=n;offset,encoded,raw=struct.unpack_from('<QII',data,pos);pos+=16;digest=data[pos:pos+32].hex();pos+=32
                assert key>previous and role(key)==d['role'] and key not in self.entries
                assert offset==end and raw<=MAX_RAW and encoded<=MAX_RAW+65536 and offset+encoded<=length
                assert codec!=0 or encoded==raw
                self.entries[key]=(d,offset,encoded,raw,digest);end=offset+encoded;previous=key;keys.append(key)
            assert pos==len(data) and end==length and keys[0]==d['first'] and keys[-1]==d['last'];self.descriptors.append(d)
    def raw(self,key):
        if key not in self.entries:return (self.root/key).read_bytes()
        d,offset,encoded,length,h=self.entries[key]
        with (self.root/f"packs/{d['pack']}.pack").open('rb') as f:f.seek(offset);data=f.read(encoded)
        assert len(data)==encoded
        if d['codec']:
            if self.zstd is None:self.zstd=Zstd()
            data=self.zstd.decode(data,length)
        assert len(data)==length and sha(data)==h
        return data
    def checked(self,path,h):
        path=Path(path);key=path.relative_to(self.root).as_posix();b=self.raw(key);assert sha(b)==h,(key,h);return b
    def verify_packs(self):
        rows=[]
        for d in self.descriptors:
            h=hashlib.sha256()
            with (self.root/f"packs/{d['pack']}.pack").open('rb') as f:
                for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
            assert h.hexdigest()==d['pack']
        for key in sorted(self.entries):
            b=self.raw(key);rows.append(dict(role=role(key),key=key,raw_length=len(b),sha256=sha(b)))
        return rows
def freeze(root,output,attempts=False):
    root=Path(root);d=Decoder(root);selector=strict((root/'SELECTED.json').read_bytes());keys=set();accepted=set()
    while selector:
        p=strict(d.checked(root/f"publications/{selector['publication']}.json",selector['digest']));accepted.add(p['attempt'])
        keys.update([f"publications/{p['id']}.json",f"results/{p['result_hash']}.json",f"attempt-hints/{p['attempt']}.json"]);selector=p['predecessor']
    reservations=strict((root/'RESERVATIONS.json').read_bytes()) if (root/'RESERVATIONS.json').exists() else {}
    if attempts:
        paths=set(d.entries)
        for directory,dirs,files in os.walk(root/'attempts'):
            paths.update((Path(directory)/name).relative_to(root).as_posix() for name in files)
        for key in paths:
            if role(key) in ['candidate','allocation'] and key.split('/')[1] in accepted and key.split('/')[1] not in reservations:keys.add(key)
    rows=[]
    for key in sorted(keys):
        b=d.raw(key)
        if len(b)<=MAX_RAW:rows.append(dict(role=role(key),key=key,raw_length=len(b),sha256=sha(b)))
    Path(output).write_text(''.join(json.dumps(row,separators=(',',':'))+'\n' for row in rows),encoding='utf-8');return rows
def verify(root,output,inventory=None,baseline=None,semantic=True):
    start=time.monotonic();d=Decoder(root);rows=d.verify_packs()
    if inventory:
        expected=[json.loads(line) for line in Path(inventory).read_text().splitlines()];assert rows==expected,('complete-raw-inventory-mismatch',len(rows),len(expected))
    result=dict(raw_records=len(rows),raw_bytes=sum(r['raw_length'] for r in rows),inventory_digest=sha(''.join(json.dumps(r,separators=(',',':'))+'\n' for r in rows).encode()),independent_codec=d.zstd.binding if d.zstd else None)
    if semantic:
        sys.path.insert(0,str(HERE.parent/'file-native-recovery-reclamation-2026-10-19'));import verify_operational
        verify_operational.checked=d.checked;result['semantic']=verify_operational.verify(root,baseline)
        # The complete current graph/YAML/ordinary-source oracle is separately independent.
        sys.path.insert(0,str(HERE.parent/'file-native-navigation-2026-10-19'));import oracle
        oracle.checked=d.checked;result['navigation']=oracle.verify(root)
    result['seconds']=time.monotonic()-start;Path(output).write_text(json.dumps(result,indent=2),encoding='utf-8');print(json.dumps(result,indent=2));return result
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--root',required=True);p.add_argument('--output',required=True);p.add_argument('--inventory');p.add_argument('--baseline');p.add_argument('--freeze',action='store_true');p.add_argument('--attempts',action='store_true');p.add_argument('--bytes-only',action='store_true');a=p.parse_args()
    if a.freeze:print(json.dumps(dict(records=len(freeze(a.root,a.output,a.attempts)))))
    else:verify(a.root,a.output,a.inventory,a.baseline,not a.bytes_only)
