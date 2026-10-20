"""Prepared authenticated H1k/H10k controls with conservative container bounds."""
import hashlib,json,subprocess,sys,threading,time
from pathlib import Path
import subset_corpus
from verify_subset import verify
from independent import Decoder
from driver import rss
from guard import current,census

HERE=Path(__file__).resolve().parent;R=Path('C:/file-native-packing-20261020');BASE=R/'subsets';OUT=R/'fix2/corpus-controls'
SAMPLE=R/'build/release/packing-sample.exe';PROTECTED=Path('C:/file-native-recovery-20261019/history-restored/disposable/store')
read=lambda p:json.loads(Path(p).read_bytes())
def file_sha(path):
    h=hashlib.sha256()
    with Path(path).open('rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''):h.update(b)
    return h.hexdigest()
def child(argv):
    p=subprocess.Popen(argv,stdout=subprocess.PIPE,stderr=subprocess.PIPE);stop=threading.Event();peak=None;missing=0;bad=[];start=time.monotonic()
    def watch():
        nonlocal peak,missing
        while not stop.is_set():
            value=rss(p.pid)
            if value is None:missing+=1
            else:
                peak=max(peak or 0,value[1])
                if value[0]>2*1024**3:bad.append('rss');p.kill();return
            stop.wait(.01)
    sampler=threading.Thread(target=watch,daemon=True);sampler.start()
    try:stdout,stderr=p.communicate(timeout=120)
    except subprocess.TimeoutExpired:p.kill();stdout,stderr=p.communicate();bad.append('120-second-censor')
    finally:stop.set();sampler.join(timeout=2)
    row=dict(argv=argv,exit=p.returncode,seconds=time.monotonic()-start,stdout=stdout.decode(),stderr=stderr.decode(),peak_rss=peak,missing_samples=missing,violations=bad);events.append(row);(OUT/'events.json').write_text(json.dumps(events,indent=2));assert p.returncode==0 and not bad
    return json.loads(stdout)

actual=current();assert actual['total_new_bytes']+228000000+67108864+33554432<=2684354560 and actual['total_new_files']+13000<=560000 and actual['free_bytes']>=140*1024**3
OUT.mkdir(exist_ok=False);(OUT/'admission.json').write_text(json.dumps(dict(actual=actual,new_controls_reservation=228000000,future_F_reservation=67108864,all_returns=33554432,cap=2684354560),indent=2))
assert file_sha(SAMPLE)=='1c591b34f961ea58037f3e0327218525e3763c566e91d1355c0ce609d937e98e'
pack=HERE.parent/'file-native-incremental-engine-experiment-2026-10-18/src/pack.rs';assert file_sha(pack)=='4d854885a54ee95ffed44bfec704f95c5c91d95b98964d419a7607dd2a48b2e3'
source=read(R/'fix2/build-binding.json')['files']
for path,row in source.items():assert file_sha(path)==row['sha256']
protected_ledger=PROTECTED/'expected-ledger.jsonl';ledger_hash=file_sha(protected_ledger);expected=None
with (R/'post-gc-operational-backup/files.jsonl').open('rb') as f:
    for line in f:
        row=json.loads(line)
        if row['path']=='expected-ledger.jsonl':expected=row;break
assert expected and ledger_hash==expected['sha256'] and protected_ledger.stat().st_size==expected['bytes']
with protected_ledger.open('rb') as f:first_row=json.loads(f.readline())
assert first_row['index']==1;first_selector=first_row['selected'];reader=Decoder(PROTECTED)
first_raw=reader.raw(f"publications/{first_selector['publication']}.json");assert hashlib.sha256(first_raw).hexdigest()==first_selector['digest']
first_publication=json.loads(first_raw);assert first_publication['sequence']==1 and first_publication['predecessor'];del reader
subset_corpus.ROOT=PROTECTED # Read-only selectors; all emitted records still authenticate the capture.
sources={}
for name in ['corpus_controls_stage.py','subset_corpus.py','verify_subset.py','independent.py','guard.py','driver.py']:
    raw=(HERE/name).read_bytes();(OUT/name).write_bytes(raw);sources[name]=hashlib.sha256(raw).hexdigest()
(OUT/'binding.json').write_text(json.dumps(dict(sources=sources,tool_sha256=file_sha(SAMPLE),pack_sha256=file_sha(pack),protected_selector_reader=str(PROTECTED),protected_ledger_sha256=ledger_hash,first_original_selector=first_selector,first_original_publication_sha256=hashlib.sha256(first_raw).hexdigest(),original_initial_selector=first_publication['predecessor'],emitted_raw_source='fully authenticated exact post-GC FNOP1 capture',operational_rewind=False),indent=2))
events=[];results={};new_paths=[]
for h,codec,ceiling in [(1000,'zstd',8000000),(10000,'raw',64000000),(10000,'zstd',80000000)]:
    corpus=BASE/f'H{h}-corpus'
    if h==10000 and not corpus.exists():
        prepared=subset_corpus.prepare(h,corpus);assert census(corpus)['bytes']<=72000000;new_paths.append(corpus)
    inventory=[json.loads(line) for line in (corpus/'inventory.jsonl').read_bytes().splitlines()];raw=sum(x['raw_length'] for x in inventory)
    roles={x['role'] for x in inventory};index=sum(50+len(x['key'].encode()) for x in inventory)+53*len(roles)
    # ZSTD_COMPRESSBOUND's extra term is <=64 bytes per frame. The frozen
    # builder uses the same bound; actual grouping must match P0 afterward.
    encoded=raw if codec=='raw' else sum(x['raw_length']+(x['raw_length']>>8)+64 for x in inventory)
    upper=encoded+index+65536;assert upper<=ceiling
    destination=BASE/f"H{h}-{'P0' if codec=='raw' else 'PZ'}"
    assert not destination.exists();result=child([str(SAMPLE),str(corpus/'records.bin'),str(corpus/'inventory.jsonl'),str(destination),codec]);assert census(destination)['bytes']<=ceiling;new_paths.append(destination)
    checked=verify(corpus,destination,destination/'independent.json');assert checked['status']=='passed'
    results[f'H{h}-{codec}']=dict(container=result,independent=checked,prospective_container_bound=upper,ceiling=ceiling)
for h in [1000,10000]:
    p0=read(BASE/f'H{h}-P0/sample.json');pz=read(BASE/f'H{h}-PZ/sample.json');keys=['role','first','last','records'];assert [{k:p[k] for k in keys} for p in p0['packs']]==[{k:p[k] for k in keys} for p in pz['packs']]
    results[f'H{h}-ratios']=dict(PZ_over_P0_bytes=pz['pack_plus_index_bytes']/p0['pack_plus_index_bytes'],P0_bytes=p0['pack_plus_index_bytes'],PZ_bytes=pz['pack_plus_index_bytes'],operational_store=False)
assert file_sha(protected_ledger)==ledger_hash
assert file_sha(PROTECTED/f"publications/{first_selector['publication']}.json")==first_selector['digest']
for path,row in source.items():assert file_sha(path)==row['sha256']
for name,digest in sources.items():assert file_sha(HERE/name)==digest
growth={str(p):census(p) for p in new_paths};assert sum(x['bytes'] for x in growth.values())+census(OUT)['bytes']<=228000000
summary=dict(status='passed',core='d70f3864f2888d46d9112213b4a03f8d82745a8e',pack_implementation_sha256=file_sha(pack),tool_sha256=file_sha(SAMPLE),results=results,new_growth=growth,protected_selectors_and_core_unchanged=True,existing_H1000_P0_preserved=True,no_STORE_no_SELECTED=True,not_operational_history=True)
(OUT/'summary.json').write_text(json.dumps(summary,indent=2));print(json.dumps(summary,indent=2))
