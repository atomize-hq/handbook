"""Regenerate native summary from compact raw spans; times are milliseconds."""
import collections
import json
import pathlib
import statistics
import sys

def summarize_native(source, destination):
    rows=[json.loads(line) for line in pathlib.Path(source).read_text(encoding='utf-8-sig').splitlines() if line]
    groups=collections.defaultdict(list)
    for row in rows:
        groups[(row['cohort'],row['history'],row['operation'],row['instrumented'])].append(row)
    summaries=[]
    for key, observations in groups.items():
        cohort,history,op,enabled=key
        symbols=collections.defaultdict(list)
        for row in observations:
            for span in row['spans']:
                symbols[span['symbol']].append(span)
        summaries.append({'cohort':cohort,'history':history,'operation':op,'instrumented':enabled,
            'n':len(observations),'wall_median_ms':statistics.median(r['wall_ms'] for r in observations),
            'wall_min_ms':min(r['wall_ms'] for r in observations),'wall_max_ms':max(r['wall_ms'] for r in observations),
            'cpu_median_ms':statistics.median(r['cpu_ms'] for r in observations),
            'symbols':[{ 'symbol':name, **{field:statistics.median(s[field] for s in values) for field in (
                'calls','inclusive_wall_ms','exclusive_wall_ms','inclusive_cpu_ms','exclusive_cpu_ms')}}
                for name, values in sorted(symbols.items())]})
    pathlib.Path(destination).write_text(json.dumps({'source_rows':len(rows),'groups':summaries},indent=2)+'\n',encoding='utf-8')

if __name__=='__main__':
    summarize_native(*sys.argv[1:])
