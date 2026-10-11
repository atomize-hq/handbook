"""One authorized fresh pair replaces an environment-overlapped diagnostic pair."""
import datetime,json,pathlib,runpy,sys
from run_pairs import HERE,root,run
from summarize import parity

mode=sys.argv[1]
base=pathlib.Path(sys.argv[2]);ref=pathlib.Path(sys.argv[3])
group='replaced' # Same eight-character length as pristine; independent physical owners.
if mode=='populate':
    original=HERE/'pristine-measurements-original.jsonl'
    assert not original.exists(),'Preserve the already archived original series'
    original.write_bytes((HERE/'pristine-measurements.jsonl').read_bytes())
    rows=[json.loads(x) for x in original.read_text().splitlines()]
    assert len(rows)==72
    excluded=[x for x in rows if (x['cohort'],x['history'],x['sample'])==('explicit',8,0)]
    assert len(excluded)==2
    (HERE/'diagnostic-excluded-pair.jsonl').write_text(''.join(json.dumps(x)+'\n' for x in excluded),encoding='utf-8')
    assert all(not root(group,'explicit',8,0,v).exists() for v in ['baseline','refactor']),'Replacement roots must be fresh'
    with (HERE/'replaced-population.jsonl').open('w',encoding='utf-8') as out:
        for variant in ['baseline','refactor']:
            # Approved common untimed refactor public-owner population driver.
            for row in run([ref,'populate',root(group,'explicit',8,0,variant),'explicit',8,0]):
                row['root_variant']=variant;out.write(json.dumps(row)+'\n');out.flush()
                print(json.dumps({'replacement_populated':variant}),flush=True)
    check=parity(group);assert check['pairs']==1
    print(json.dumps({'replacement_byte_parity':True}),flush=True)
elif mode=='measure':
    assert not (HERE/'replaced-measurements.jsonl').exists(),'Never reuse a measured replacement root'
    runpy.run_path(str(HERE/'verify_sources.py'))
    window=json.loads((HERE/'replacement-quiescent-start.json').read_text(encoding='utf-8-sig'))
    assert window['known_research_build_test_population_workers']==[],'Quiescence inventory must finish before measurement'
    assert (HERE/'replaced-byte-parity.json').exists()
    replacement=[]
    with (HERE/'replaced-measurements.jsonl').open('w',encoding='utf-8') as out:
        for order,(variant,exe) in enumerate([('baseline',base),('refactor',ref)]):
            launch=datetime.datetime.now(datetime.timezone.utc).isoformat()
            rows=run([exe,'measure',root(group,'explicit',8,0,variant),'explicit',8,0,variant,'off'])
            finished=datetime.datetime.now(datetime.timezone.utc).isoformat()
            assert len(rows)==1
            for row in rows:
                row.update(within_pair_order=order,measurement_stage='late_replacement',
                           overall_pristine_sequence_order=72+order,
                           process_launch_request_utc=launch,collection_finished_utc=finished)
                replacement.append(row);out.write(json.dumps(row)+'\n');out.flush()
                print(json.dumps({'replacement_measured':variant,'pair_ms':row['pair_wall_ms']}),flush=True)
    original=[json.loads(x) for x in (HERE/'pristine-measurements-original.jsonl').read_text().splitlines()]
    selected=[]
    for order,row in enumerate(original):
        if (row['cohort'],row['history'],row['sample'])!=('explicit',8,0):
            row['overall_pristine_sequence_order']=order;selected.append(row)
    selected+=replacement;assert len(selected)==72
    (HERE/'pristine-measurements.jsonl').write_text(''.join(json.dumps(x)+'\n' for x in selected),encoding='utf-8')
    selection={'reason':'First baseline overlapped a still-finishing environment/start-inventory snapshot; no build/test/population overlap. Both original pair observations excluded.',
               'authorization':'Root approved bounded fresh independent replacement with original baseline-first order.',
               'original_series':'pristine-measurements-original.jsonl','excluded_pair':'diagnostic-excluded-pair.jsonl',
               'accepted_series':'pristine-measurements.jsonl','accepted_observations':72,'actual_total_pristine_observations':74,
               'replacement_population':'replaced-population.jsonl','replacement_parity':'replaced-byte-parity.json',
               'replacement_measurements':'replaced-measurements.jsonl','replacement_quiescent_start':'replacement-quiescent-start.json',
               'actual_late_order':['baseline','refactor'],'replacement_follows_original_series':True,
               'resource_roots':'Original measured sample0 explicit/history8 diagnostic roots; not replacement roots.',
               'impact':'New top-level research-only helper is UNKNOWN/unindexed; source context calls existing run, parity, verify_sources without product edits.'}
    (HERE/'main-selection.json').write_text(json.dumps(selection,indent=2)+'\n',encoding='utf-8')
else:raise ValueError('populate|measure')
