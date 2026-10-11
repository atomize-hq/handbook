"""Verify release and summarize same-process native resource observations."""
import collections,json,pathlib,statistics
HERE=pathlib.Path(__file__).resolve().parent
rows=[json.loads(x) for x in (HERE/'resource-measurements.jsonl').read_text(encoding='utf-8').splitlines()]
results=[]
tables=['| Source | Same-process operations / handles before-after | Sampled private / OS working-set peak (MiB) | Sampled handle peak | Initial fixed-history replay private-after range (MiB) | Final fixed-history refusal/replay private-after range (MiB) |',
        '| --- | --- | --- | --- | --- | --- |']
lock_tables=['| Source | Injected parent delay (ms) | Child work-lock wait (ms) | Child acquired work-lock held (ms) | Full child current-read wall / process CPU (ms) |',
             '| --- | --- | --- | --- | --- |']
expected={'committed_replay':{'replayed':6},'revoked_fresh_invocation':{'refused':6},'revoked_original_replay':{'replayed':6},
          'injected-BeforeIntent':{'refused':3},'recover-BeforeIntent':{'unaccepted':3},
          'injected-Intent':{'uncertain':3},'recover-Intent':{'committed':3},
          'injected-Snapshot':{'uncertain':3},'recover-Snapshot':{'committed':3},
          'injected-PublicationAttempt':{'uncertain':3},'recover-PublicationAttempt':{'uncertain':3},
          'injected-Reply':{'uncertain':3},'recover-Reply':{'replayed':3}}
for variant in ['baseline','refactor']:
    operations=[x for x in rows if x['variant']==variant and x['event']=='resource']
    complete=[x for x in rows if x['variant']==variant and x['event']=='resources_complete']
    race=[x for x in rows if x['variant']==variant and x['event']=='lock_wait']
    assert len(operations)==48 and len(complete)==len(race)==1
    assert complete[0]['handles_before']==complete[0]['handles_after']
    assert all(x['handles_before']==x['handles_after'] and x['profile']['retained_locks_after_phase']==0 for x in operations)
    assert race[0]['profile']['retained_locks_after_phase']==0
    phases={}
    for phase in sorted({x['phase'] for x in operations}):
        selected=[x for x in operations if x['phase']==phase]
        phases[phase]={'repetitions':len(selected),'dispositions':dict(collections.Counter(x['disposition'] for x in selected)),
            'handles_before':[x['handles_before'] for x in selected],'handles_after':[x['handles_after'] for x in selected],
            'private_after_bytes':[x['memory_after']['private_bytes'] for x in selected],
            'working_after_bytes':[x['memory_after']['working_set_bytes'] for x in selected]}
    assert {k:v['dispositions'] for k,v in phases.items()}==expected
    private=[x['memory_after']['private_bytes'] for x in operations]
    working=[x['memory_after']['working_set_bytes'] for x in operations]
    wait=sum(x['inclusive_wall_ms'] for x in race[0]['profile']['spans'] if x['symbol']=='lock_wait')
    assert wait>=100,('controlled contention not observed',variant,wait)
    wait_spans=[x for x in race[0]['profile']['spans'] if x['symbol']=='lock_wait']
    assert len(wait_spans)==1 and wait_spans[0]['calls']==1
    held=[dict(x,held_wall_ms=x['units']/1000) for x in race[0]['profile']['paths'] if x['kind']=='lock_held_us']
    assert len(held)==1 and held[0]['path']=='.handbook/work-state/work.lock' and held[0]['calls']==1
    initial=[x['memory_after']['private_bytes'] for x in operations if x['phase']=='committed_replay']
    tail=[x['memory_after']['private_bytes'] for x in operations if x['phase'] in ['revoked_fresh_invocation','revoked_original_replay']]
    entries=max(x['profile']['gauges'].get('artifact_entries_peak',0) for x in operations)
    buffers=max(x['profile']['gauges'].get('artifact_buffer_bytes_peak',0) for x in operations)
    assert entries<=32
    results.append({'variant':variant,'operations':48,'process_handles_before':complete[0]['handles_before'],'process_handles_after':complete[0]['handles_after'],
        'private_after_first_last_min_max_bytes':[private[0],private[-1],min(private),max(private)],
        'working_after_first_last_min_max_bytes':[working[0],working[-1],min(working),max(working)],
        'sampled_process_peak_private_bytes':complete[0]['sampled_peak_private_bytes'],
        'os_peak_working_set_bytes':complete[0]['os_peak_working_set_bytes'],
        'sampled_process_peak_handles':complete[0]['sampled_peak_process_handles'],
        'handle_sampling_interval_ms':100,'handle_peak_is_sampled_not_exact':True,
        'controlled_delay_ms':200,'observed_lock_wait_ms':wait,'race_wall_ms':race[0]['wall_ms'],
        'race_process_cpu_ms_includes_both_threads':race[0]['cpu_ms'],
        'race_child_only_thread_local_wait_spans':wait_spans,'race_child_acquired_lock_held_paths':held,
        'race_wait_path':'.handbook/work-state/work.lock',
        'race_wait_path_owner_evidence':'work_state/store.rs LOCK constant and WorkLock::open exclusive acquisition; child has one acquisition/wait',
        'race_parent_guard_lifetime_not_separately_profiled':True,
        'resource_artifact_entries_peak':entries,'resource_artifact_raw_key_expected_buffer_bytes_peak':buffers,
        'cache_gauge_excludes_decoded_intent_output_rc_allocator':True,
        'fixed_initial_history':9,'fixed_final_history':23,
        'fixed_history_private_initial_sequence_bytes':initial,'fixed_history_private_final_sequence_bytes':tail,
        'fixed_history_tail_strictly_monotonic_increase':all(a<b for a,b in zip(tail,tail[1:])),
        'deliberate_history_growth_between_fixed_tails':True,'phases':phases})
    mib=1024*1024
    tables.append(f"| {variant} | 48 / {complete[0]['handles_before']}→{complete[0]['handles_after']} | {complete[0]['sampled_peak_private_bytes']/mib:.2f} / {complete[0]['os_peak_working_set_bytes']/mib:.2f} | {complete[0]['sampled_peak_process_handles']} | {min(initial)/mib:.2f}–{max(initial)/mib:.2f} | {min(tail)/mib:.2f}–{max(tail)/mib:.2f} |")
    lock_tables.append(f"| {variant} | 200 | {wait:.4f} | {held[0]['held_wall_ms']:.3f} | {race[0]['wall_ms']:.4f} / {race[0]['cpu_ms']:.2f} |")
(HERE/'resource-summary.json').write_text(json.dumps(results,indent=2)+'\n',encoding='utf-8')
(HERE/'resource-tables.md').write_text('\n'.join(tables)+'\n\n'+ '\n'.join(lock_tables)+'\n\nThe 200 ms delay occurs in the parent BeforeIntent callback; registration continues under its guard. Wait wall spans are child-thread-local, CPU is whole-process and includes both threads. Parent guard lifetime is not separately profiled. Private/handle peaks are sampled every 100 ms; working-set peak is OS reported. History deliberately grows 9→23 between the fixed-history tails. Tail values fluctuate; they do not establish absence of memory retention.\n',encoding='utf-8')
print(json.dumps({'variants':2,'same_process_resource_operations':96,'no_retained_lock_or_handle_accumulation':True}))
