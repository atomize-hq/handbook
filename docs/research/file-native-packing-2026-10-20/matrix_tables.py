"""Derive gates only from completed fresh path-v2 cells; missing cells stay null."""
import json,statistics,math
from pathlib import Path
R=Path('C:/file-native-packing-20261020');BASE=R/'operational-matrix'
def dist(values):
    values=sorted(values)
    return None if not values else dict(samples=len(values),median=statistics.median(values),p95=values[math.ceil(.95*len(values))-1],minimum=values[0],maximum=values[-1])
result=dict(modes={},failed_original_epoch_excluded=True,ordinary_gate='median prepare+select <= qualified L median*1.2 +1ms',fresh_cache_qualification='fresh process, uncontrolled OS/device caches')
lines=['# Representative N8000 matched cells','','| Mode | Cell | Seven-publication median ms | / qualified L | Ordinary gate |','|---|---|---:|---:|---|']
for mode in ['A','B']:
    cells={}
    for name in ['L-old','L-new','P0','PZ']:
        path=BASE/f'n8000-{mode}'/(name+'-path-v2-measurement.json')
        if not path.exists():cells[name]=None;continue
        row=json.loads(path.read_bytes());assert row['history_before']==8 and row['history_after']==15 and len(row['publications'])==7 and len(row['fresh'])==120 and row['independent']['semantic']['publications']==15
        groups={}
        for warm in row['warm']['result']['rows']:
            fresh=[x for x in row['fresh'] if x['category']==warm['category']]
            groups[warm['category']]=dict(warm_ns=dist(warm['warm_ns']),fresh_engine_ns=dist([x['engine_wall_ns'] for x in fresh]),fresh_child_ns=dist([round(x['child_seconds']*1e9) for x in fresh]),fresh_rss=dist([x['rss'] for x in fresh if x['rss'] is not None]),result_digest=warm['result_digest'],opens=warm['opens'],pack_storage_bytes=warm.get('pack_storage_bytes'),decoded_bytes=warm.get('decoded_record_bytes'))
        cells[name]=dict(median_ns=row['ordinary_median_complete_engine_ns'],publication_cases=[dict(case=x['case'],prepare_ns=x['prepare']['wall_ns'],select_ns=x['select']['wall_ns'],complete_ns=x['complete_engine_ns'],replay_objects=x['replay_objects'],replay_files=x['replay_files'],state_digest=x['result_state_digest']) for x in row['publications']],groups=groups,exe_sha256=row['binary_sha256'],original_cohort_records=row['cohort_records'],new_loose_tail=row['new_loose_tail'],fixture=row['fixture'],cache_qualification=row['cache_qualification'])
        if name in ['P0','PZ']:
            inventory=BASE/f'n8000-{mode}'/(name+'-path-v2-operational-backup/files.jsonl')
            records=[json.loads(line) for line in inventory.read_text().splitlines()];paths={x['path'] for x in records};original={json.loads(line)['key'] for line in (BASE/f'n8000-{mode}/matched-cohort-inventory.jsonl').read_text().splitlines()}
            representation=[x for x in records if x['path'].startswith(('packs/','pack-catalogs/'))];remaining=len(paths&original);physical=len(representation)+remaining;reduction=1-physical/len(original)
            cells[name]['packing']=dict(pack_index_bytes=sum(x['bytes'] for x in representation if x['path'].startswith('packs/')),pack_index_catalog_files=len(representation),remaining_original_loose_duplicates=remaining,original_cohort_files=len(original),eligible_physical_files_after=physical,file_reduction=reduction,ninety_five_percent_gate=reduction>=.95,all_other_owner_journal_control_files_additionally_charged_to_whole_store=True)
    reference=cells['L-old']
    for name,row in cells.items():
        if row is None:lines.append(f'| {mode} | {name} | pending | - | null |');continue
        ratio=None if reference is None else row['median_ns']/reference['median_ns'];gate=None if reference is None else row['median_ns']<=reference['median_ns']*1.2+1000000
        row.update(ordinary_over_qualified_L=ratio,ordinary_gate=gate)
        if reference:assert [x['state_digest'] for x in row['publication_cases']]==[x['state_digest'] for x in reference['publication_cases']]
        if name=='P0' and reference:row['warm_ratios']={k:v['warm_ns']['median']/reference['groups'][k]['warm_ns']['median'] for k,v in row['groups'].items()}
        if name=='PZ' and cells['P0']:row['warm_ratios']={k:v['warm_ns']['median']/cells['P0']['groups'][k]['warm_ns']['median'] for k,v in row['groups'].items()}
        if 'warm_ratios' in row:row['warm_2x_gate']=all(x<=2 for x in row['warm_ratios'].values())
        lines.append(f"| {mode} | {name} | {row['median_ns']/1e6:.3f} | {'-' if ratio is None else f'{ratio:.3f}x'} | {gate} |")
    result['modes'][mode]=cells
    if cells['P0'] and cells['PZ']:
        ratio=cells['PZ']['packing']['pack_index_bytes']/cells['P0']['packing']['pack_index_bytes'];result['modes'][mode]['compression']=dict(PZ_over_P0_bytes=ratio,twenty_percent_gate=ratio<=.8)
(BASE/'tables.json').write_text(json.dumps(result,indent=2),encoding='utf-8');(BASE/'tables.md').write_text('\n'.join(lines)+'\n',encoding='utf-8');print('\n'.join(lines))
