"""Read-only final-reader comparison; old raw-container creation stays identified."""
import json, math, statistics
from pathlib import Path

R=Path('C:/file-native-packing-20261020')
OUT=R/'long-matrix-fixed2/tables'
OUT.mkdir(exist_ok=False)
read=lambda p:json.loads((R/p).read_bytes())
cells={'L-old':read('long-matrix/L-old-measurement.json'),'L-new':read('long-matrix/L-new-measurement.json'),'P0':read('long-matrix-fixed2/summary.json'),'PZ':read('long-matrix-fixed2/PZ-measurement.json')}
def stats(samples):
    s=sorted(samples)
    return dict(samples=len(s),median=statistics.median(s),p95=s[math.ceil(.95*len(s))-1],minimum=s[0],maximum=s[-1])
result=dict(cells={},ratios={},storage={},qualifications=['P0 containers were created by 4cb6b9f; P0 and PZ final readers are d70f3864.','Warm is same-process after first lookup, 500 samples per category. Fresh is 20 process starts per category, with uncontrolled OS/device caches.','Original cohort keys are identical; no new loose tail substitutes for an original key.','Allocation is regular-file data only, excluding MFT/directory metadata. Logical instrumentation does not measure physical device writes.','Loose and old P0 duplicates still coexist at this measurement checkpoint.'])
lines=['# Final-reader original-cohort measurements','','| Category | Qualified L µs | New loose µs | P0 µs | P0 / L | PZ µs | PZ / P0 |','|---|---:|---:|---:|---:|---:|---:|']
for name,row in cells.items():
    groups={}
    for warm in row['warm']['result']['rows']:
        fresh=[f for f in row['fresh'] if f['category']==warm['category']]
        groups[warm['category']]=dict(warm_ns=stats(warm['warm_ns']),fresh_engine_ns=stats([f.get('engine_ns',f.get('engine_wall_ns')) for f in fresh]),fresh_controller_ns=stats([f.get('controller_ns',f.get('external_process_ns')) for f in fresh]),fresh_rss=stats([f['rss'] for f in fresh if f['rss'] is not None]),first_lookup_ns=warm['first_lookup_ns'],opens_per_warm=warm['opens']/500,pack_storage_bytes_per_warm=warm.get('pack_storage_bytes',0)/500,decoded_bytes_per_warm=warm.get('decoded_record_bytes',0)/500,result_digest=warm['result_digest'])
    result['cells'][name]=dict(groups=groups,warm_metrics=row['warm']['metrics'],warm_peak_rss=row['warm'].get('peak_memory_bytes'),binary_sha256=row.get('exe_sha256',row.get('fresh_binary_sha256')),cache_qualification=row['cache_qualification'])
for category in result['cells']['P0']['groups']:
    med={n:r['groups'][category]['warm_ns']['median'] for n,r in result['cells'].items()};a=med['P0']/med['L-old'];b=med['PZ']/med['P0']
    result['ratios'][category]=dict(P0_over_qualified_L=a,PZ_over_P0=b,P0_2x_gate=a<=2,PZ_2x_gate=b<=2)
    assert len({r['groups'][category]['result_digest'] for r in result['cells'].values()})==1
    lines.append(f"| {category} | {med['L-old']/1000:.2f} | {med['L-new']/1000:.2f} | {med['P0']/1000:.2f} | {a:.3f}× | {med['PZ']/1000:.2f} | {b:.3f}× |")
for name,path in [('P0','long-matrix/P0-build.json'),('PZ','long-matrix-fixed2/PZ-build.json')]:
    b=read(path);result['storage'][name]=dict(pack_plus_index_bytes=b['pack_plus_index_bytes'],packs=len(b['catalog']['packs']),representation_files=2*len(b['catalog']['packs'])+1,records=sum(p['records'] for p in b['catalog']['packs']))
result['PZ_over_P0_bytes']=result['storage']['PZ']['pack_plus_index_bytes']/result['storage']['P0']['pack_plus_index_bytes'];result['twenty_percent_gate']=result['PZ_over_P0_bytes']<=.8
lines+=['',f"P0 pack/index: {result['storage']['P0']['pack_plus_index_bytes']:,} bytes; PZ: {result['storage']['PZ']['pack_plus_index_bytes']:,} bytes ({result['PZ_over_P0_bytes']:.4%} of P0).",'']+result['qualifications']
(OUT/'final-tables.json').write_text(json.dumps(result,indent=2));(OUT/'final-tables.md').write_text('\n'.join(lines)+'\n',encoding='utf-8')
print('\n'.join(lines))
