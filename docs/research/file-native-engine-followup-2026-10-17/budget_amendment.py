"""Record native evidence and preserve exact harness bytes before cap amendment."""
import argparse,datetime,hashlib,json,pathlib,time,zipfile
p=argparse.ArgumentParser();p.add_argument('--base',type=pathlib.Path,required=True);p.add_argument('--source',type=pathlib.Path,required=True);a=p.parse_args();h=pathlib.Path(__file__).resolve().parent
for backend in ['manifest','cow']:assert not list((a.base/backend/'growth').glob('100000-*')),'large cases already started'
names=['experiment.py','baseline_harness.py','workloads.py','regressions.py','terminal_copy.py']
source_hashes={name:hashlib.sha256((h/name).read_bytes()).hexdigest() for name in names}
for name,digest in source_hashes.items():assert hashlib.sha256((a.source/name).read_bytes()).hexdigest()==digest,('source copy differs',name)
raw=a.base/'manifest/growth-raw.jsonl';data=raw.read_bytes();prefix=data[:data.rfind(b'\n')+1];events=[json.loads(s) for s in prefix.splitlines() if s]
profile=[]
for r in events:
    c=r['command'];out=r.get('output') or {}
    if '/10000-' not in c['root'] or r['censored'] or out.get('status')!='ok':continue
    if c['action']=='init' or (c['action']=='publish' and (c.get('request',{}).get('request') or '').startswith(('commit-1-','commit-8-','commit-64-'))):
        profile.append(dict(root=c['root'],action=c['action'],request=c.get('request',{}).get('request'),engine_ms=out['wall_ns']/1e6,directory_syncs=out['metrics']['directory_syncs'],file_syncs=out['metrics']['file_syncs']))
assert any(r['action']=='init' and '/10000-A-' in r['root'] for r in profile)
assert any(r['action']=='publish' and '/10000-B-' in r['root'] for r in profile)
evidence=h/'evidence';archive=evidence/'harness-before-wsl-large-budget.zip';manifest=evidence/'harness-before-wsl-large-budget.json'
assert not archive.exists() and not manifest.exists(),'never overwrite prior amendment evidence'
with zipfile.ZipFile(archive,'x',compression=zipfile.ZIP_DEFLATED,compresslevel=9) as z:
    for name in names:
        info=zipfile.ZipInfo(name,date_time=(1980,1,1,0,0,0));info.compress_type=zipfile.ZIP_DEFLATED;z.writestr(info,(h/name).read_bytes())
record=dict(decision_epoch=time.time(),decision_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),before_any_wsl_large_root=True,reason='Observed 10k A initialization ~189s and B complete-directory commits ~63-66s project beyond prior 1800/600s large ceilings at 100k. Allow bounded scaling margin before launch; no weaker work or guarantees.',old_caps_seconds=dict(large_init=1800,publish=600),future_wsl_100k_caps_seconds=dict(large_init=2400,publish=900),unchanged=dict(file_allocation_cap_bytes=48*1024**3,host_reserve_bytes=160*1024**3,max_workers=8,history_tiers=[20,10,3],rust_core='v4 unchanged'),archive=archive.name,archive_sha256=hashlib.sha256(archive.read_bytes()).hexdigest(),source_hashes=source_hashes,profile_raw=str(raw),profile_raw_prefix_bytes=len(prefix),profile_raw_prefix_sha256=hashlib.sha256(prefix).hexdigest(),profile=profile)
manifest.write_text(json.dumps(record,indent=2)+'\n',encoding='utf8');print(json.dumps({k:v for k,v in record.items() if k not in ['profile','source_hashes']},indent=2))
