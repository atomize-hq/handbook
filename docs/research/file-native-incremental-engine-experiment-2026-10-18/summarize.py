"""Derive compact measured tables; raw events remain authoritative."""
import argparse,gzip,json,statistics
from pathlib import Path

def events(path):
    b=path.read_bytes();b=gzip.decompress(b) if path.suffix=='.gz' else b
    return [json.loads(line) for line in b.splitlines()]
def measurement(row):
    out=row.get('output') or {};m=out.get('metrics',{});ns=m.get('ns',{})
    return {'controller_ms':row['elapsed_ns']/1e6,'engine_ms':out['wall_ns']/1e6 if 'wall_ns' in out else None,'peak_memory_bytes':out.get('peak_memory_bytes'),'exit':row['exit'],'censored':row['censored'],
        'disposition':out.get('result',{}).get('status') if isinstance(out.get('result'),dict) else None,
        'outcome':out.get('result',{}).get('outcome') if isinstance(out.get('result'),dict) else None,'error':out.get('error'),
        **{k:m.get(k) for k in ['documents_visited','edges_visited','page_reads','index_updates','touched_paths','bytes_metadata_read','bytes_metadata_written','bytes_payload_read','bytes_payload_written','bytes_control_read','bytes_control_written','bytes_result_read','bytes_result_written','bytes_read','bytes_written','copies','file_syncs','directory_syncs','unsupported_directory_syncs','namespace_ops']},
        'unique_pages':len(m['unique_pages']) if 'unique_pages' in m else None,
        **{k+'_ms':ns[k]/1e6 if k in ns else None for k in ['graph-transition-inclusive','projection-prepare-inclusive','preparation-inclusive','selection-inclusive','lock-wait','lock-hold','A-completion-inclusive']}}

def run(root):
    report={}
    for name in ['codec-windows-v1','matrix-common-pin','post-matrix','document-calibration-final','codec-common-pin','windows-final-extra','history-common-pin']:
        path=root/name/'events.jsonl'
        if not path.exists():continue
        rows=events(path);picked=[]
        for i,row in enumerate(rows,1):
            v=row['input'];action=v['action'];request=v.get('request',{}).get('request')
            history_probe=name=='history-common-pin' and (action in ['resolve','cancel','point','inspect','audit','lifecycle'] or action=='publish' and request=='deep-lost-reply')
            if history_probe or action in ['init','history','packet','checkpoint','refresh','resume','validation-trace'] or action=='publish' and (request or '').startswith(('update-','genuine-','change-','B-one','one-in-','incoming-5000','combined-')):
                picked.append({'raw_line':i,'action':action,'root':v['root'],'request':request,'request_id':v.get('request_id'),'selected_input':v.get('selected'),'n':v.get('count'),
                    'collection_result':(row.get('output') or {}).get('result') if action=='lifecycle' else None,
                    'metrics_scope':'outer API setup; per-publication counters are in the ledger' if action=='history' else 'this API invocation',**measurement(row)})
        report[name]={'events':len(rows),'censors':sum(v['censored'] for v in rows),'observations':picked}
    path=root/'history-common-pin/store/expected-ledger.jsonl'
    if path.exists():
        rows=events(path);groups={}
        for lo,hi in [(1,1000),(1001,10000),(10001,100000)]:
            for category in ['content','metadata','equal-content','restore','maintenance']:
                group=[v for v in rows if lo<=v['index']<=hi and v['category']==category]
                if not group:continue
                elapsed=[v['engine_ns']/1e6 for v in group]
                groups[f'{lo}-{hi}:{category}']={'count':len(group),'median_engine_ms':statistics.median(elapsed),'max_engine_ms':max(elapsed),
                    'median_pages':statistics.median(v['metrics']['page_reads'] for v in group),
                    'median_metadata_read':statistics.median(v['metrics']['bytes_metadata_read'] for v in group),
                    'median_metadata_written':statistics.median(v['metrics']['bytes_metadata_written'] for v in group)}
        report['history_homogeneous_groups']=groups
    return report

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--root',required=True);a.add_argument('--output',required=True);ns=a.parse_args()
    Path(ns.output).write_text(json.dumps(run(Path(ns.root)),indent=2))
