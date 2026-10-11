"""Expose absolute process memory and callback handle samples per fixed cell."""
import json,pathlib,statistics
HERE=pathlib.Path(__file__).resolve().parent
rows=[json.loads(x) for x in (HERE/'pristine-measurements.jsonl').read_text().splitlines()]
assert len(rows)==72
cells=[]
for cohort in ['explicit','kind']:
    for history in [8,16,31]:
        cell={'cohort':cohort,'history':history}
        for variant in ['baseline','refactor']:
            selected=[x for x in rows if (x['cohort'],x['history'],x['variant'])==(cohort,history,variant)]
            assert len(selected)==6
            cell[variant]={}
            for field in ['sampled_peak_private_bytes','os_peak_working_set_bytes','sampled_peak_process_handles','handles_before','handles_after']:
                values=[x[field] for x in selected]
                cell[variant][field]={'min':min(values),'median':statistics.median(values),'max':max(values),'samples':values}
        cells.append(cell)
attribution=[json.loads(x) for x in (HERE/'attribution-measurements.jsonl').read_text().splitlines()]
handles=[{'cohort':x['cohort'],'history':x['history'],'variant':x['variant'],'instrumented':x['instrumented'],
          'before':x['handles_before'],'boundary_sample_max':x['boundary_max_handles'],'after':x['handles_after'],
          'sampled_peak_process_handles':x['sampled_peak_process_handles'],
          'sampled_peak_private_bytes':x['sampled_peak_private_bytes'],'os_peak_working_set_bytes':x['os_peak_working_set_bytes']}
         for x in attribution]
result={'pristine_cells':cells,'attribution_handle_samples':handles,
        'private_and_handle_sampling_interval_ms':100,'working_set_peak_is_os_reported':True,
        'polling_and_boundary_handle_maxima_are_sampled_not_exact_peaks':True,
        'cache_gauge_excludes_decoded_intent_output_rc_and_allocator':True}
(HERE/'memory-summary.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
lines=['| Cohort / history | Baseline private peak median / max (MiB) | Refactor private peak median / max (MiB) | Baseline / refactor working-set peak max (MiB) | Baseline / refactor sampled handle peak median / max |',
       '| --- | --- | --- | --- | --- |']
for cell in cells:
    b=cell['baseline'];r=cell['refactor'];factor=1024*1024
    lines.append(f"| {cell['cohort']} / {cell['history']} | {b['sampled_peak_private_bytes']['median']/factor:.2f} / {b['sampled_peak_private_bytes']['max']/factor:.2f} | {r['sampled_peak_private_bytes']['median']/factor:.2f} / {r['sampled_peak_private_bytes']['max']/factor:.2f} | {b['os_peak_working_set_bytes']['max']/factor:.2f} / {r['os_peak_working_set_bytes']['max']/factor:.2f} | {b['sampled_peak_process_handles']['median']} / {b['sampled_peak_process_handles']['max']} · {r['sampled_peak_process_handles']['median']} / {r['sampled_peak_process_handles']['max']} |")
(HERE/'memory-tables.md').write_text('\n'.join(lines)+'\n',encoding='utf-8')
print(json.dumps({'fixed_cells_with_absolute_memory_peaks':6,'attribution_handle_samples':len(handles)}))
