import argparse,json,statistics,math
from pathlib import Path
R=Path('C:/file-native-packing-20261020');BASE=R/'long-matrix'
def percentile(values,p):
    values=sorted(values);return values[max(0,min(len(values)-1,math.ceil(len(values)*p)-1))]
def distribution(values):return dict(samples=len(values),median=statistics.median(values),p95=percentile(values,.95),minimum=min(values),maximum=max(values))
def main(output):
    cells={}
    for name in ['L-old','L-new','P0','PZ']:
        path=BASE/(name+'-measurement.json')
        if not path.exists():continue
        row=json.loads(path.read_bytes());groups={}
        events=json.loads((BASE/(name+'-events/events.json')).read_bytes())
        fresh_events=[x for x in events if x['input']['action']=='resolve']
        for warm in row['warm']['result']['rows']:
            category=warm['category'];fresh=[(f,e) for f,e in zip(row['fresh'],fresh_events) if f['category']==category]
            groups[category]=dict(warm_ns=distribution(warm['warm_ns']),fresh_engine_ns=distribution([x[0]['engine_wall_ns'] for x in fresh]),fresh_observed_child_ns=distribution([round(x[1]['seconds']*1e9) for x in fresh]),fresh_controller_ns=distribution([x[0]['external_process_ns'] for x in fresh]),first_lookup_ns=warm['first_lookup_ns'],result_digest=warm['result_digest'],opens_per_warm=warm['opens']/len(warm['warm_ns']),pack_storage_bytes_per_warm=warm.get('pack_storage_bytes',0)/len(warm['warm_ns']),decoded_record_bytes_per_warm=warm.get('decoded_record_bytes',0)/len(warm['warm_ns']))
        cells[name]=dict(groups=groups,warm_exe_sha256=row['warm_binary_sha256'],fresh_exe_sha256=row['fresh_binary_sha256'],warm_peak_rss=row['warm'].get('peak_memory_bytes'),warm_metrics=row['warm']['metrics'],cache_qualification=row['cache_qualification'])
    rows=[];lines=['# Matched original-cohort lookup observations','', '| Category | Qualified L median µs | New loose median µs | P0 median µs | P0 / qualified L | PZ median µs | PZ / P0 |','|---|---:|---:|---:|---:|---:|---:|']
    for category in ['early','middle','recent','missing','cancelled','lost-reply']:
        med={name:value['groups'][category]['warm_ns']['median'] for name,value in cells.items()}
        ratio0=med.get('P0',0)/med['L-old'] if 'P0' in med else None;ratioz=med.get('PZ',0)/med['P0'] if 'PZ' in med and 'P0' in med else None
        rows.append(dict(category=category,warm_median_ns=med,P0_over_qualified_L=ratio0,PZ_over_P0=ratioz,P0_2x_gate=None if ratio0 is None else ratio0<=2,PZ_2x_gate=None if ratioz is None else ratioz<=2))
        show=lambda name:'—' if name not in med else f'{med[name]/1000:.2f}'
        lines.append(f"| {category} | {show('L-old')} | {show('L-new')} | {show('P0')} | {'—' if ratio0 is None else f'{ratio0:.3f}×'} | {show('PZ')} | {'—' if ratioz is None else f'{ratioz:.3f}×'} |")
    storage={}
    for name in ['P0','PZ']:
        path=BASE/(name+'-build.json')
        if path.exists():
            d=json.loads(path.read_bytes());storage[name]=dict(pack_plus_index_bytes=d['pack_plus_index_bytes'],packs=len(d['catalog']['packs']),logical_records=sum(x['records'] for x in d['catalog']['packs']),representation_files=2*len(d['catalog']['packs'])+1,cleanup_complete=False)
    if storage:
        lines+=['','| Cell | Logical records | Pack/index/catalog files | Pack + index bytes |','|---|---:|---:|---:|']
        for name,d in storage.items():lines.append(f"| {name} | {d['logical_records']:,} | {d['representation_files']} | {d['pack_plus_index_bytes']:,} |")
    ratio=None
    if 'P0' in storage and 'PZ' in storage:ratio=storage['PZ']['pack_plus_index_bytes']/storage['P0']['pack_plus_index_bytes']
    lines+=['','All cells retain fixed original lookup keys. New loose tails are separate. Warm means the same process after first lookup. Fresh processes have uncontrolled OS/device caches. Representation files do not claim a compacted footprint while loose/old-container duplicates coexist. Instrumented opens/bytes are not physical device writes.']
    result=dict(cells=cells,matched=rows,storage=storage,PZ_over_P0_bytes=ratio,PZ_20_percent_smaller_gate=None if ratio is None else ratio<=.8)
    output=Path(output);output.mkdir(parents=True,exist_ok=True);(output/'lookup-tables.json').write_text(json.dumps(result,indent=2));(output/'lookup-tables.md').write_text('\n'.join(lines)+'\n',encoding='utf-8');print('\n'.join(lines))
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--output',required=True);a=p.parse_args();main(a.output)
