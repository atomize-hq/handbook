"""Authenticated H1k/H10k record subsets, never shortened operational stores."""
import argparse,collections,gzip,hashlib,json,struct,time
from pathlib import Path
from independent import Decoder,role,sha,MAX_RAW
R=Path('C:/file-native-packing-20261020');ROOT=R/'history-restored/disposable/store';CAPTURE=R/'post-gc-operational-backup'
def file_sha(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
    return h.hexdigest()
def prepare(h,output):
    assert h in [1000,10000];output=Path(output);output.mkdir(parents=True,exist_ok=False);start=time.monotonic();manifest=json.loads((CAPTURE/'manifest.json').read_bytes())
    assert file_sha(CAPTURE/'operational.bin.gz')==manifest['transport_sha256'] and file_sha(CAPTURE/'files.jsonl')==manifest['inventory_sha256']
    selectors={};first=None
    with (ROOT/'expected-ledger.jsonl').open('rb') as f:
        for i in range(1,h):
            row=json.loads(f.readline());assert row['index']==i;selectors[row['selected']['publication']]=dict(selected=row['selected'],sequence=i)
            if first is None:first=row['selected']
    d=Decoder(ROOT);raw=d.raw(f"publications/{first['publication']}.json");assert sha(raw)==first['digest'];initial=json.loads(raw)['predecessor'];selectors[initial['publication']]=dict(selected=initial,sequence=0);del d
    publications={};attempts={};results={};rows=[];written=set();total=0;cap=(8 if h==1000 else 64)*1024**2
    def take(f,n):
        b=f.read(n);assert len(b)==n;return b
    with (output/'records.bin').open('xb') as corpus:
        def save(key,raw,digest):
            nonlocal total
            assert key not in written and len(raw)<=MAX_RAW and total+len(raw)<=cap and sha(raw)==digest
            rows.append(dict(role=role(key),key=key,raw_length=len(raw),sha256=digest,offset=total));written.add(key);corpus.write(raw);total+=len(raw)
        def scan(pass_number):
            with gzip.open(CAPTURE/'operational.bin.gz','rb') as f,(CAPTURE/'files.jsonl').open('rb') as inventory:
                assert take(f,6)==b'FNOP1\n'
                for line in inventory:
                    record=json.loads(line);n,size=struct.unpack('<IQ',take(f,12));assert n<=4096 and size<=512*1024**2
                    key=take(f,n).decode();assert key==record['path'] and size==record['bytes'];kind=role(key)
                    wanted=False
                    if pass_number==1:
                        if kind=='publication' and key.split('/')[-1][:-5] in selectors:wanted=True
                        elif kind=='result' and key.split('/')[-1][:-5] in results:wanted=True
                    elif kind=='hint':wanted=key.split('/')[-1][:-5] in attempts
                    elif kind in ['candidate','allocation']:wanted=key.split('/')[1] in attempts
                    if wanted:
                        assert size<=MAX_RAW;raw=take(f,size);digest=take(f,32).hex();assert digest==record['sha256'] and sha(raw)==digest
                        value=json.loads(raw)
                        if kind=='publication':
                            bound=selectors[value['id']];assert bound['sequence']==value['sequence'] and bound['selected']['digest']==digest
                            publications[value['id']]=value;attempts[value['attempt']]=bound['selected'];results[value['result_hash']]=value
                        elif kind=='result':
                            p=results[key.split('/')[-1][:-5]];assert value['outcome']=='accepted' and value['request']==p['request'] and value['publication']==p['id']
                        elif kind=='hint':assert value==attempts[key.split('/')[-1][:-5]]
                        save(key,raw,digest)
                    else:
                        remaining=size
                        while remaining:remaining-=len(take(f,min(1024*1024,remaining)))
                        assert take(f,32).hex()==record['sha256']
                assert not f.read(1)
        scan(1);assert len(publications)==h and len(results)==h;scan(2)
    rows.sort(key=lambda x:x['key']);counts=collections.Counter(x['role'] for x in rows);assert counts['publication']==counts['result']==counts['hint']==h and len(rows)<=50000
    raw=''.join(json.dumps(row,separators=(',',':'))+'\n' for row in rows).encode();(output/'inventory.jsonl').write_bytes(raw)
    result=dict(profile='authenticated-record-subset-nonoperational',operational_store=False,subset_publications=h,source_publications=101006,source_capture_transport_sha256=manifest['transport_sha256'],source_capture_inventory_sha256=manifest['inventory_sha256'],source_permanent_verification='preservation-verify-result.json',records=len(rows),roles=counts,raw_bytes=total,corpus_sha256=file_sha(output/'records.bin'),inventory_sha256=sha(raw),selection='initial plus original accepted ledger rows 1..H-1; exact selector digests and source archive bytes verified; finalized retained attempts only where present',seconds=time.monotonic()-start)
    (output/'manifest.json').write_text(json.dumps(result,indent=2));print(json.dumps(result,indent=2));return result
if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--h',type=int,required=True);a.add_argument('--output',required=True);n=a.parse_args();prepare(n.h,n.output)
