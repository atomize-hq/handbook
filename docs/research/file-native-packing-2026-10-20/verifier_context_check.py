"""Read-only deterministic frame corpus; serial/concurrent independent DCtx test."""
import argparse,collections,hashlib,json
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from independent import Decoder,Zstd,sha

p=argparse.ArgumentParser();p.add_argument('--name',required=True);p.add_argument('--expect',choices=['race','pass'],required=True);args=p.parse_args();r=Path('C:/file-native-packing-20261020');root=r/'history-restored/disposable/store';out=r/'long-matrix-fixed2'/args.name;out.mkdir(exist_ok=False)
(out/'harness.py').write_bytes(Path(__file__).read_bytes());source=Path(__file__).with_name('independent.py').read_bytes();(out/'independent.py').write_bytes(source);selected=(root/'SELECTED.json').read_bytes();store=(root/'STORE.json').read_bytes();d=Decoder(root);frames=[];roles=collections.Counter()
for key,(descriptor,offset,encoded,length,digest) in d.entries.items():
    if roles[descriptor['role']]>=4:continue
    with (root/f"packs/{descriptor['pack']}.pack").open('rb') as file:file.seek(offset);data=file.read(encoded)
    expected=(root/key).read_bytes();assert len(data)==encoded and len(expected)==length and sha(expected)==digest;frames.append((key,data,expected));roles[descriptor['role']]+=1
assert len(frames)==20;z=Zstd()
for index,(key,data,expected) in enumerate(frames):assert z.decode(data,len(expected))==expected;(out/f'frame-{index}.zst').write_bytes(data)
def one(index):
    key,data,expected=frames[index%len(frames)]
    try:return 'ok' if z.decode(data,len(expected))==expected else 'wrong-raw-bytes'
    except Exception as error:return type(error).__name__
with ThreadPoolExecutor(max_workers=4) as pool:counts=collections.Counter(pool.map(one,range(10000)))
assert (counts['ok']<10000) if args.expect=='race' else (counts=={'ok':10000})
assert selected==(root/'SELECTED.json').read_bytes() and store==(root/'STORE.json').read_bytes()
result=dict(status='race-reproduced' if args.expect=='race' else 'concurrent-decoder-check-passed',calls=10000,workers=4,results=counts,serial_frames_passed=len(frames),frame_inventory=[dict(key=key,encoded_bytes=len(data),raw_bytes=len(expected),encoded_sha256=sha(data),raw_sha256=sha(expected)) for key,data,expected in frames],independent_source_sha256=sha(source),codec_binding=z.binding,selected_and_store_unchanged=True,engine_not_called=True)
(out/'summary.json').write_text(json.dumps(result,indent=2));print(json.dumps({k:v for k,v in result.items() if k!='frame_inventory'},indent=2))
