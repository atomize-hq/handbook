"""Render final tables from retained observations; never rerun native effects."""
import json,pathlib,statistics
HERE=pathlib.Path(__file__).resolve().parent
summary=json.loads((HERE/'summary.json').read_text(encoding='utf-8'))
lines=['| Cohort / history | Baseline pair range / SD (ms) | Refactor pair range / SD (ms) | Paired saving range / SD (ms) | Descriptive paired-median bootstrap 95% interval (ms) |',
       '| --- | --- | --- | --- | --- |']
cpu=['| Cohort / history | Baseline preparation / invocation / actual-pair CPU median (ms) | Refactor preparation / invocation / actual-pair CPU median (ms) |',
     '| --- | --- | --- |']
for row in summary['pristine']:
    cells=[]
    for variant in ['baseline','refactor','paired_saved']:
        cell=row['pair_wall_ms'][variant];cells.append(f"{cell['min']:.2f}–{cell['max']:.2f} / {cell['sample_sd']:.2f}")
    interval=row['paired_median_saved_ms_bootstrap_95_percentile']
    lines.append(f"| {row['cohort']} / {row['history']} | {' | '.join(cells)} | {interval[0]:.2f}–{interval[1]:.2f} |")
    medians=[' / '.join(f"{row[field][variant]['median']:.2f}" for field in ['preparation_cpu_ms','invocation_cpu_ms','pair_cpu_ms']) for variant in ['baseline','refactor']]
    cpu.append(f"| {row['cohort']} / {row['history']} | {' | '.join(medians)} |")
(HERE/'performance-detail-tables.md').write_text('\n'.join(lines)+'\n\n'+'\n'.join(cpu)+'\n\nSix diverse matched fixture pairs per cell; intervals are descriptive seeded 10,000-resample percentile bootstrap intervals, not production latency guarantees. Every cell has six positive pair differences (exact two-sided sign-flip p=0.03125). Full phase wall/CPU min, median, mean, maximum and sample SD remain in summary.json.\n',encoding='utf-8')
controls=[json.loads(line) for line in (HERE/'controls-measurements.jsonl').read_text(encoding='utf-8').splitlines()]
lines=['| Cohort / isolated dimension / history 16 | Baseline preparation / invocation / actual pair (ms) | Refactor preparation / invocation / actual pair (ms) | Actual paired saving (ms) |',
       '| --- | --- | --- | --- |']
for cohort,sample in [('explicit',6),('explicit',7),('kind',6),('kind',7),('kind',8),('kind',9)]:
    pair={row['variant']:row for row in controls if row['cohort']==cohort and row['sample']==sample}
    assert set(pair)=={'baseline','refactor'}
    label={6:'native bytes',7:'work bounds',8:'second kind version',9:'planning role'}[sample]
    medians=[' / '.join(f"{pair[variant][field]:.2f}" for field in ['preparation_wall_ms','invocation_wall_ms','pair_wall_ms']) for variant in ['baseline','refactor']]
    saving=pair['baseline']['pair_wall_ms']-pair['refactor']['pair_wall_ms'];assert saving>0
    lines.append(f"| {cohort} / {label} | {' | '.join(medians)} | {saving:.2f} |")
(HERE/'control-tables.md').write_text('\n'.join(lines)+'\n\nOne independent matched pair per isolated dimension/cohort. These corroborate dimension coverage and do not replace the six repeats in each required cell.\n',encoding='utf-8')
overhead=summary['attribution']['enabled_disabled_controls']
lines=['| Cohort / history / source | Enabled invocation (ms) | Disabled invocation (ms) | Enabled minus disabled (ms) |','| --- | --- | --- | --- |']
for row in overhead:
    lines.append(f"| {row['cohort']} / {row['history']} / {row['variant']} | {row['enabled_invocation_wall_ms']:.4f} | {row['disabled_invocation_wall_ms']:.4f} | {row['enabled_minus_disabled_invocation_ms']:.4f} |")
values=[row['enabled_minus_disabled_invocation_ms'] for row in overhead]
lines.append(f"\nOne independent enabled/disabled pair per cell/source; median {statistics.median(values):.4f} ms, range {min(values):.4f}–{max(values):.4f} ms. The negative observation is retained as measurement noise. No overhead subtraction is applied to pristine release latency. Attribution pair timing includes preparation profile materialization.\n")
(HERE/'instrumentation-overhead-tables.md').write_text('\n'.join(lines),encoding='utf-8')
observations=json.loads((HERE/'profile-observations.json').read_text(encoding='utf-8'))
lines=['| Cohort / history / source / held observations I2–I4 | Output or item-output reconstructions | Artifact hits / misses | Canonical encodes / MiB | Bounded work encodes / MiB | Lists / entries | Returned bounded leaf reads / MiB |',
       '| --- | --- | --- | --- | --- | --- | --- |']
for cohort in ['explicit','kind']:
    for history in [8,16,31]:
        for variant in ['baseline','refactor']:
            selected=[x for x in observations if x['cohort']==cohort and x['history']==history and x['variant']==variant and x['phase']=='invocation' and x['observation'] in [2,3,4]]
            assert len(selected)==3
            total={key:sum(x[key] for x in selected) for key in ['output_calls','item_output_calls','artifact_hit_calls','artifact_miss_calls','canonical_encoding_calls','canonical_encoding_bytes','bounded_work_encoding_calls','bounded_work_encoding_bytes','list_calls','listed_entries','bounded_read_calls','bounded_read_bytes']}
            mib=1024*1024
            lines.append(f"| {cohort} / {history} / {variant} | {total['output_calls']+total['item_output_calls']} | {total['artifact_hit_calls']} / {total['artifact_miss_calls']} | {total['canonical_encoding_calls']} / {total['canonical_encoding_bytes']/mib:.2f} | {total['bounded_work_encoding_calls']} / {total['bounded_work_encoding_bytes']/mib:.2f} | {total['list_calls']} / {total['listed_entries']} | {total['bounded_read_calls']} / {total['bounded_read_bytes']/mib:.2f} |")
(HERE/'profile-held-history-tables.md').write_text('\n'.join(lines)+'\n\nI1 has a separate temporary cache and remains separate in profile-tables.md. Each held artifact map also derives the new candidate outside scans once; it is already present for I3/I4 hits. Proof hit/miss counters are refactor-only; the baseline already had a per-scan historical-proof set. These counters are not physical disk I/O.\n',encoding='utf-8')
print(json.dumps({'pristine_cells':6,'isolated_dimension_pairs':6,'overhead_controls':len(overhead),'derived_held_history_cells':12}))
