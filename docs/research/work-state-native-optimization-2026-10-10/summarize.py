"""Summarize observations; no production tail quantiles from these small groups."""
import collections,json,pathlib,statistics,sys
here=pathlib.Path(sys.argv[1])
rows=[];events=[]
for mode in ('reads','effects','pending'):
    p=here/f'{mode}.jsonl'
    if not p.exists(): continue
    for line in p.read_text(encoding='utf-8').splitlines():
        row=json.loads(line)
        (rows if 'wall_ms' in row else events).append(row)
groups=collections.defaultdict(list)
for r in rows:groups[(r['cohort'],r['operation'],r['history_slots_at_entry'],r['variant'],r['instrumented'])].append(r)
summary=[]
for key,rs in sorted(groups.items()):
    summary.append(dict(zip(('cohort','operation','history','variant','instrumented'),key),
        n=len(rs),median_ms=statistics.median(r['wall_ms'] for r in rs),
        min_ms=min(r['wall_ms'] for r in rs),max_ms=max(r['wall_ms'] for r in rs),
        median_cpu_ms=statistics.median(r['cpu_ms'] for r in rs),bytes=rs[0]['retained_work_bytes']))
effects=[]
for variant in sorted({r['variant'] for r in rows if r['operation']=='fresh_invocation'}):
    rs=[r for r in rows if r['variant']==variant and r['operation']=='fresh_invocation' and not r['instrumented']]
    prep={(r['history_slots_at_entry'],r['sample']):r for r in rows if r['variant']==variant and r['operation']=='fresh_preparation' and not r['instrumented']}
    if rs:
        effects.append({'variant':variant,'n':len(rs),'entry_history':[r['history_slots_at_entry'] for r in rs],
          'preparation_median_ms':statistics.median(prep[(r['history_slots_at_entry'],r['sample'])]['wall_ms'] for r in rs),
          'invocation_median_ms':statistics.median(r['wall_ms'] for r in rs),
          'paired_workflow_median_ms':statistics.median(r['wall_ms']+prep[(r['history_slots_at_entry'],r['sample'])]['wall_ms'] for r in rs)})
profiles=[]
for r in rows:
    if not r['instrumented']:continue
    p=r['profile'];spans=p['spans'];paths=p['paths']
    def calls(name):return sum(s['calls'] for s in spans if s['symbol'].endswith(':'+name) or s['symbol']==name)
    def exclusive(name):return sum(s['exclusive_wall_ms'] for s in spans if s['symbol'].endswith(':'+name) or s['symbol']==name)
    profiles.append({k:r[k] for k in ('cohort','operation','history_slots_at_entry','sample','variant')}|
      {'scan_passes':calls('scan_reusing'),'canonical_calls':calls('canonical_json_bytes'),
       'directory_list_calls':calls('list_directory_bounded'),'directory_opens':calls('open_windows_directory_guard'),
       'handle_information_calls':calls('windows_handle_info'),
       'list_entries_observed':sum(p['units'] for p in paths if p['kind']=='list'),
       'bounded_read_returned_bytes':sum(p['units'] for p in paths if p['kind']=='read'),
       'distinct_bounded_read_paths':sum(p['kind']=='read' for p in paths),
       'pairwise_lineage_inclusive_ms':sum(s['inclusive_wall_ms'] for s in spans if s['symbol']=='pairwise_and_lineage_checks'),
       'directory_list_exclusive_ms':exclusive('list_directory_bounded'),
       'canonical_exclusive_ms':exclusive('canonical_json_bytes')})
result={'observations':len(rows),'events':events,'groups':summary,'matched_effects':effects,'profiles':profiles}
(here/'summary.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'observations':len(rows),'groups':len(summary),'matched_effects':effects},indent=2))
