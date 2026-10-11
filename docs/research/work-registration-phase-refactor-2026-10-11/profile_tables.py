"""Render bounded attribution with I1 distinct from held I2-I4 observations."""
import collections,json,pathlib
HERE=pathlib.Path(__file__).resolve().parent
rows=[json.loads(x) for x in (HERE/'attribution-measurements.jsonl').read_text().splitlines() if json.loads(x)['instrumented']]
data=[];lines=['| Cohort / history / source / phase / observation | Scans / inventories / manifests | Output / item-output calls | Canonical encodes / MiB | Artifact hit / miss (intent MiB) | Historical proof hit / miss (proof MiB) | Lists / entries / bounded leaf MiB |',
              '| --- | --- | --- | --- | --- | --- | --- |']
for row in rows:
    for phase,field in [('preparation','preparation_profile'),('invocation','invocation_profile')]:
        profile=row[field]
        observations=sorted({x['observation'] for x in profile['spans']}|{x['observation'] for x in profile['counts']}|{x['observation'] for x in profile['paths']}|{0})
        for observation in observations:
            spans={x['symbol']:x for x in profile['spans'] if x['observation']==observation}
            counters={x['counter']:x for x in profile['counts'] if x['observation']==observation}
            paths=[x for x in profile['paths'] if x['observation']==observation]
            calls=lambda label:spans.get(label,{}).get('calls',0)
            count=lambda label,key:counters.get(label,{}).get(key,0)
            listings=[x for x in paths if x['kind']=='list'];reads=[x for x in paths if x['kind']=='read']
            cache_hit=count('artifact_hit_intent_bytes','calls');cache_miss=count('artifact_miss_intent_bytes','calls')
            result={'cohort':row['cohort'],'history':row['history'],'variant':row['variant'],'phase':phase,'observation':observation,
                'scan_inventory_manifest':[calls(x) for x in ['work_scan','work_state/store.rs:inventory','work_state/store.rs:manifest']],
                'output_calls':calls('work_state/transaction.rs:output'),'item_output_calls':calls('work_state/transaction.rs:item_output'),
                'canonical_encoding_calls':count('canonical_encoded_bytes','calls'),'canonical_encoding_bytes':count('canonical_encoded_bytes','units'),
                'bounded_work_encoding_calls':count('work_encoded_bytes','calls'),'bounded_work_encoding_bytes':count('work_encoded_bytes','units'),
                'artifact_hit_calls':cache_hit,'artifact_hit_intent_bytes':count('artifact_hit_intent_bytes','units'),
                'artifact_miss_calls':cache_miss,'artifact_miss_intent_bytes':count('artifact_miss_intent_bytes','units'),
                'pure_historical_proof_hit_calls':count('pure_historical_proof_hit_bytes','calls'),
                'pure_historical_proof_hit_bytes':count('pure_historical_proof_hit_bytes','units'),
                'pure_historical_proof_miss_calls':count('pure_historical_proof_miss_bytes','calls'),
                'pure_historical_proof_miss_bytes':count('pure_historical_proof_miss_bytes','units'),
                'list_calls':sum(x['calls'] for x in listings),'listed_entries':sum(x['units'] for x in listings),
                'bounded_read_calls':sum(x['calls'] for x in reads),'bounded_read_bytes':sum(x['units'] for x in reads),
                'unique_bounded_read_paths':len(reads),'spans':list(spans.values())}
            data.append(result);label='outside scans' if observation==0 else (f'I{observation}' if phase=='invocation' else f'P{observation}')
            factor=1024*1024
            lines.append(f"| {row['cohort']} / {row['history']} / {row['variant']} / {phase} / {label} | {' / '.join(str(x) for x in result['scan_inventory_manifest'])} | {result['output_calls']} / {result['item_output_calls']} | {result['canonical_encoding_calls']} / {result['canonical_encoding_bytes']/factor:.2f} | {cache_hit} / {cache_miss} ({result['artifact_hit_intent_bytes']/factor:.2f} / {result['artifact_miss_intent_bytes']/factor:.2f}) | {result['pure_historical_proof_hit_calls']} / {result['pure_historical_proof_miss_calls']} ({result['pure_historical_proof_hit_bytes']/factor:.2f} / {result['pure_historical_proof_miss_bytes']/factor:.2f}) | {result['list_calls']} / {result['listed_entries']} / {result['bounded_read_bytes']/factor:.2f} |")
(HERE/'profile-observations.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf-8')
(HERE/'profile-tables.md').write_text('\n'.join(lines)+'\n',encoding='utf-8')
# Inclusive/exclusive per-owner spans retain nesting; these values must never be
# added to parent scan/wrapper values as independently removable work.
owners=['work_scan','local_authority/work_registration.rs:with_work_registration_current','local_authority/work_registration.rs:with_work_registration_establishment',
        'local_authority/work_registration.rs:validate_current','local_authority/work_registration.rs:validate_historical','local_authority/work_registration.rs:capture_proof',
        'local_authority/local_check.rs:work_registration_current','work_state/transaction.rs:current_grant',
        'repository_durability.rs:list_directory_bounded','repository_durability.rs:read_regular_bounded',
        'repository_durability.rs:pin_windows_parent','repository_durability.rs:open_windows_directory_guard',
        'repository_durability.rs:windows_handle_info','repository_durability.rs:acquire_existing_exclusive_lock_v1',
        'repository_durability.rs:lock_existing','definition_identity.rs:canonical_json_bytes','lock_wait']
span_rows=[];phase_gauges=[];held_rows=[]
for row in rows:
    for phase,field in [('preparation','preparation_profile'),('invocation','invocation_profile')]:
        profile=row[field]
        phase_gauges.append({'cohort':row['cohort'],'history':row['history'],'variant':row['variant'],'phase':phase,
                             'gauges':profile['gauges'],'gauge_excludes_decoded_intent_output_rc_allocator':True})
        held_rows.append({'cohort':row['cohort'],'history':row['history'],'variant':row['variant'],'phase':phase,
                          'paths':[dict(x,held_wall_ms=x['units']/1000) for x in profile['paths'] if x['kind']=='lock_held_us'],
                          'overlapping_guards_are_not_additive_removable_cost':True})
        for label in owners+sorted({x['symbol'] for x in profile['spans'] if x['symbol'].startswith('work_state/scope.rs:permits')}):
            selected=[x for x in profile['spans'] if x['symbol']==label]
            if selected:span_rows.append({'cohort':row['cohort'],'history':row['history'],'variant':row['variant'],'phase':phase,'symbol':label,
                **{key:sum(x[key] for x in selected) for key in ['calls','inclusive_wall_ms','exclusive_wall_ms','inclusive_cpu_ms','exclusive_cpu_ms']}})
(HERE/'profile-owner-spans.json').write_text(json.dumps(span_rows,indent=2)+'\n',encoding='utf-8')
(HERE/'profile-cache-gauges.json').write_text(json.dumps(phase_gauges,indent=2)+'\n',encoding='utf-8')
(HERE/'profile-lock-held.json').write_text(json.dumps(held_rows,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'phase_observation_rows':len(data),'nested_owner_span_rows':len(span_rows)}))
