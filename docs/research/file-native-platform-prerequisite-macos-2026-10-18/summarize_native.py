"""Derive Mac tables from actual retained observations; no invented completions."""
import json
import pathlib
import statistics

HERE=pathlib.Path(__file__).resolve().parent
runtime=pathlib.Path(json.loads((HERE/'allocation.json').read_bytes())['runtime'])
data=runtime/'data/v1'

def load(path):return json.loads(path.read_bytes())
def rows(path):return [json.loads(line) for line in path.read_bytes().splitlines()]
def write(name,value):(HERE/name).write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n')

phases=[]
for p in sorted(data.rglob('native-phase-*.json')):
    phases.append(dict(path=str(p.relative_to(runtime)),**load(p)))
write('phase-summary.json',phases)
correctness=[]
for backend in ['manifest','cow']:
    for phase in ['correctness','regressions']:
        base=data/backend/phase
        result=dict(backend=backend,phase=phase,
            completed=(base/'native-phase-completed.json').exists(),
            raw_events=len(rows(base/(phase+'-raw.jsonl'))) if (base/(phase+'-raw.jsonl')).exists() else None)
        for name in ['race','semantic','fault','extended-fault','retention','duplicate-delete']:
            p=base/(name+'-observations.json')
            if p.exists():result[name+'_rows']=len(load(p))
        p=base/'io-observations.json'
        if p.exists():result['io']=load(p)
        p=base/'fault-observations.json'
        if p.exists():
            result['phase_oracles']={kind:sum(r.get('phase_oracle')==kind for r in load(p))
                for kind in ['must-not-select','visible-uncertain','must-retain']}
        correctness.append(result)
write('correctness-summary.json',correctness)

operations=[];points=[];stores=[]
for backend in ['manifest','cow']:
    for mode in ['A','B']:
        base=data/backend/('growth-'+mode)
        p=base/'growth-raw.jsonl'
        if not p.exists():continue
        obs=load(base/'growth-observations.json') if (base/'growth-observations.json').exists() else {}
        stores.append(dict(backend=backend,mode=mode,**{k:v for k,v in obs.items() if k not in ['backend','mode']}))
        for line,r in enumerate(rows(p),1):
            c=r['command'];out=r['output'] or {};m=out.get('metrics',{})
            item=dict(backend=backend,mode=mode,action=c['action'],exit=r['exit'],censored=r['censored'],
                status=out.get('status'),error=out.get('error'),raw_file=f'v1--{backend}--growth-{mode}--growth-raw.jsonl.gz',
                raw_line=line,external_ms=r['external_wall_ns']/1e6,engine_ms=out.get('wall_ns',0)/1e6,
                metadata_read=m.get('bytes_metadata_read'),metadata_written=m.get('bytes_metadata_written'),
                bytes_read=m.get('bytes_read'),bytes_written=m.get('bytes_written'),copies=m.get('copies'),
                file_syncs=m.get('file_syncs'),directory_syncs=m.get('directory_syncs'),
                unsupported_directory_syncs=m.get('unsupported_directory_syncs'),
                inclusive_stage_ms={k:v/1e6 for k,v in m.get('ns',{}).items()},
                engine_peak_memory_bytes=out.get('peak_memory_bytes'),native_process_usage=r.get('native_process_usage'))
            if c['action'] in ['init','publish','packet']:
                result=out.get('result') or {}
                item.update(request=c.get('request',{}).get('request'),packet_requested=c.get('request',{}).get('packet',c.get('packet',True)),
                    delivery=result.get('status'),outcome=(result.get('result') or {}).get('outcome'))
                if c['action']=='packet':
                    item['binding']='initial retained' if c['selected']==obs.get('initial') else 'current' if c['selected']==obs.get('current') else 'unknown'
                    assert item['binding']!='unknown'
                assert r['exit']==0 and not r['censored'] and out['status']=='ok'
                if c['action'] in ['init','publish']:
                    assert item['delivery']=='published' and item['outcome']=='accepted'
                operations.append(item)
            elif c['action']=='point':
                item.update(key=c['id'],binding='initial retained' if c['selected']==obs.get('initial') else 'current' if c['selected']==obs.get('current') else 'unknown')
                assert item['binding']!='unknown' and r['exit']==0 and not r['censored'] and out['status']=='ok'
                assert (out['result']['value'] is not None)==(c['id']=='d000001')
                points.append(item)
write('operations.json',operations);write('points.json',points);write('smoke-stores.json',stores)
lines=['# Native 1k smoke observations','',
    'One observation per initialization/commit/packet. All engine stages are inclusive and can overlap. Controller latency includes process response serialization and exit; prelaunch allocation checks are outside that request clock. Darwin wait4 peak RSS includes the entire process, including serialization/exit. No device-cold or percentile claim.','',
    '| Backend | Mode | Operation | Engine ms | Process ms | Metadata read / written B | Copied files | Process peak MiB |',
    '| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |']
for r in operations:
    usage=r['native_process_usage'];peak=f"{usage['peak_rss_bytes']/1024**2:.2f}" if usage else 'unavailable'
    name='initial' if r['action']=='init' else r['request'] if r['action']=='publish' else r['binding']+' selected packet'
    lines.append(f"| {r['backend']} | {r['mode']} | {name} | {r['engine_ms']:.2f} | {r['external_ms']:.2f} | {r['metadata_read']} / {r['metadata_written']} | {r['copies']} | {peak} |")
lines+=['','Current and initial-retained point medians below each use exactly three successful fresh processes. These are ordinary metadata key `d000001` and outside-range negative key `not-present`; they are not payload reads or full audits.','',
    '| Backend | Mode | Binding | Key | Samples | Engine median ms | Process median ms | Metadata read B |',
    '| --- | --- | --- | --- | ---: | ---: | ---: | ---: |']
for backend in ['manifest','cow']:
    for mode in ['A','B']:
        for binding in ['current','initial retained']:
            for key in ['d000001','not-present']:
                group=[r for r in points if (r['backend'],r['mode'],r['binding'],r['key'])==(backend,mode,binding,key)]
                if not group:continue
                assert len(group)==3
                lines.append(f"| {backend} | {mode} | {binding} | {key} | 3 | {statistics.median(r['engine_ms'] for r in group):.3f} | {statistics.median(r['external_ms'] for r in group):.3f} | {group[0]['metadata_read']} |")
lines+=['','| Backend | Mode | Selected publications | Logical bytes | Allocated file bytes | Files | Directories |',
    '| --- | --- | ---: | ---: | ---: | ---: | ---: |']
for r in stores:
    i=r['inventory'];assert r['selected_publications']==7
    lines.append(f"| {r['backend']} | {r['mode']} | 7 | {i['logical_bytes']} | {i['allocated_file_bytes']} | {i['files']} | {i['directories']} |")
(HERE/'performance.md').write_text('\n'.join(lines)+'\n')
print(json.dumps(dict(completed_phases=sum(p.get('status')=='passed' for p in phases),
    stopped_phases=sum('error' in p for p in phases),operations=len(operations),point_samples=len(points),smoke_stores=len(stores))))
