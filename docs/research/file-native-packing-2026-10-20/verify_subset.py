"""Independent subset container parser; no engine resolver or operational control."""
import argparse,hashlib,json,struct,time
from pathlib import Path
from independent import Zstd,bounded,strict,role,sha,MAX_INDEX,MAX_PACK,MAX_RAW
def verify(source,root,output):
    source=Path(source);root=Path(root);start=time.monotonic();origin=strict(bounded(source/'manifest.json',65536));assert origin['profile']=='authenticated-record-subset-nonoperational' and origin['operational_store'] is False
    assert type(origin['raw_bytes']) is int and 0<=origin['raw_bytes']<=64*1024**2 and type(origin['records']) is int and 0<origin['records']<=50000
    inventory=bounded(source/'inventory.jsonl',16*1024**2);assert sha(inventory)==origin['inventory_sha256']
    digest=hashlib.sha256()
    with (source/'records.bin').open('rb') as f:
        assert f.seek(0,2)==origin['raw_bytes'];f.seek(0)
        for block in iter(lambda:f.read(65536),b''):digest.update(block)
    assert digest.hexdigest()==origin['corpus_sha256']
    manifest=strict(bounded(root/'sample.json',65536));assert manifest['profile']=='authenticated-record-subset-nonoperational' and manifest['no_operational_controls'] is True and not (root/'STORE.json').exists() and not (root/'SELECTED.json').exists()
    rows=[];codec=None;zstd=None
    for d in manifest['packs']:
        assert all(type(d[x]) is int for x in ['pack_bytes','index_bytes','records','codec']) and d['codec'] in [0,1] and 0<d['records']<=65536 and d['pack_bytes']<=MAX_PACK and d['index_bytes']<=MAX_INDEX
        data=bounded(root/f"packs/{d['index']}.idx",d['index_bytes'],d['index_bytes']);assert sha(data)==d['index'] and data[:8]==b'FNPIDX01' and data[8:40].hex()==d['pack']
        size,codec,count=struct.unpack_from('<QBI',data,40);assert (size,codec,count)==(d['pack_bytes'],d['codec'],d['records']);pack=root/f"packs/{d['pack']}.pack"
        digest=hashlib.sha256()
        with pack.open('rb') as f:
            assert f.seek(0,2)==size;f.seek(0)
            for b in iter(lambda:f.read(1024*1024),b''):digest.update(b)
        assert digest.hexdigest()==d['pack'];pos=53;end=0;keys=[]
        with pack.open('rb') as f:
            for _ in range(count):
                n=struct.unpack_from('<H',data,pos)[0];pos+=2;assert n<=512;key=data[pos:pos+n].decode();pos+=n
                offset,encoded,length=struct.unpack_from('<QII',data,pos);pos+=16;raw_hash=data[pos:pos+32].hex();pos+=32
                assert offset==end and length<=MAX_RAW and encoded<=MAX_RAW+65536 and offset+encoded<=size and role(key)==d['role'] and (not keys or keys[-1]<key)
                f.seek(offset);raw=f.read(encoded);assert len(raw)==encoded
                if codec:
                    if zstd is None:zstd=Zstd()
                    raw=zstd.decode(raw,length)
                assert len(raw)==length and sha(raw)==raw_hash;end=offset+encoded;keys.append(key);rows.append(dict(role=role(key),key=key,raw_length=length,sha256=raw_hash))
        assert pos==len(data) and end==size and keys[0]==d['first'] and keys[-1]==d['last']
    expected=[]
    for line in inventory.splitlines():
        row=strict(line);expected.append({k:row[k] for k in ['role','key','raw_length','sha256']})
    rows.sort(key=lambda x:x['key']);assert rows==expected and len(rows)==manifest['records']==origin['records'] and sum(x['raw_length'] for x in rows)==manifest['raw_bytes']==origin['raw_bytes']
    result=dict(status='passed',profile='record-subset-not-operational-history',records=len(rows),raw_bytes=manifest['raw_bytes'],pack_plus_index_bytes=manifest['pack_plus_index_bytes'],packs=len(manifest['packs']),codec=codec,independent_codec=zstd.binding if zstd else None,source_inventory_sha256=sha(inventory),source_corpus_sha256=origin['corpus_sha256'],source_capture_transport_sha256=origin['source_capture_transport_sha256'],source_capture_inventory_sha256=origin['source_capture_inventory_sha256'],seconds=time.monotonic()-start)
    Path(output).write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2));return result
if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--source',required=True);a.add_argument('--root',required=True);a.add_argument('--output',required=True);n=a.parse_args();verify(n.source,n.root,n.output)
