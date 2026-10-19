"""Derive compact tables from retained measurements; no engine or fixture edits."""
import argparse,json
from pathlib import Path
def run(root,out):
    root=Path(root);out=Path(out);out.mkdir(parents=True,exist_ok=True);cells=[];rows=[]
    for base in sorted(p for p in root.iterdir() if p.is_dir()):
        p=base/'measurements.json';s=base/'summary.json'
        values=json.loads(p.read_bytes()) if p.exists() else []
        summary=json.loads(s.read_bytes()) if s.exists() else None
        scope=summary.get('scope') if summary else None
        if values and not scope:scope=values[-1].get('scope','complete ordinary view and immutable projection')
        role='extra deferred-view control' if scope and 'deferred' in scope else 'primary complete-view cell'
        for row in values:row['_cell']=base.name;row['_role']=role
        rows.extend(values)
        cells.append(dict(cell=base.name,status=summary.get('status') if summary else 'censored/no completed measurements',scope=scope,role=role,cases=len(values),fixture_retained=(base/'cell/disposable/store').exists(),summary=summary))
    indexed={(r['n'],r['mode'],r['profile'],r['case']):r for r in rows if r['_role']=='primary complete-view cell'}
    def compact(r):
        r=dict(r,scope=r.get('scope') or 'complete ordinary view and immutable projection')
        m=r['prepare_metrics'];sm=r['select_metrics'];rm=r['refresh_metrics'] or {}
        return dict(cell=r['_cell'],role=r['_role'],scope=r.get('scope'),n=r['n'],mode=r['mode'],profile=r['profile'],case=r['case'],prepare_ms=r['prepare_seconds']*1000,select_ms=r['select_seconds']*1000,refresh_ms=r['refresh_seconds']*1000 if r['refresh_seconds'] is not None else None,projection_prepare_ms=m['ns'].get('projection-prepare-inclusive',0)/1e6,B_generation_ms=m['ns'].get('B-generation-inclusive',0)/1e6,B_verify_ms=sm['ns'].get('B-generation-verification-inclusive',0)/1e6,header_points=m['documents_visited'],header_range_rows=m['range_rows'].get('D',0),outgoing_rows=m['range_rows'].get('E',0),reverse_rows=m['range_rows'].get('R',0),range_leaf_pairs=m.get('range_leaf_pairs',{}),range_page_visits=m.get('range_page_visits',{}),generated_parts=m['navigation_parts'],generated_bytes=m['navigation_bytes'],navigation_changed_paths=r['navigation_changed_paths'],changed_paths=r['changed_paths'],A_touched_paths=rm.get('touched_paths'),replay_objects=r['replay_objects'],replay_files=r['replay_files'],checker_peak_rss=r['checker_peak_rss'],oracle_seconds=r['oracle_seconds'])
    result=dict(cells=cells,measurements=[compact(r) for r in rows],missing=[dict(n=10000,mode='B',profile=p,status='initialization censored before selection' if p==4 else 'unperformed after profile4 initialization censor') for p in [2,3,4]],sample_plan='one observation of each named operation in each cell; uncontrolled caches; engine wall clocks exclude independent oracle/GC/census; no SLA or latency distribution claim; full restored A10k comparators supersede extra deferred-view controls in the tables')
    (out/'measurement-summary.json').write_text(json.dumps(result,indent=2),encoding='utf-8')
    text=['# Derived measurement tables','',result['sample_plan'],'','## A payload dirtiness','','| N | Profile | Prepare ms | Header points | D rows | Generated parts | Bytes | Replay objects/files |','| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |']
    for n in [128,1000,10000]:
        for profile in [2,3,4]:
            r=indexed.get((n,'A',profile,'single-payload'))
            if r:
                c=compact(r);text.append(f"| {n} | {profile} | {c['prepare_ms']:.2f} | {c['header_points']} | {c['header_range_rows']} | {c['generated_parts']} | {c['generated_bytes']} | {c['replay_objects']}/{c['replay_files']} |")
    text+=['','## A title update locality','','| N | Profile | Prepare ms | Header points | D rows | E rows | Generated parts/bytes | Changed nav paths |','| --- | --- | ---: | ---: | ---: | ---: | --- | ---: |']
    for n in [128,1000,10000]:
        for profile in [2,3,4]:
            r=indexed.get((n,'A',profile,'single-title'))
            if r:
                c=compact(r);text.append(f"| {n} | {profile} | {c['prepare_ms']:.2f} | {c['header_points']} | {c['header_range_rows']} | {c['outgoing_rows']} | {c['generated_parts']}/{c['generated_bytes']} | {len(c['navigation_changed_paths'])} |")
    text+=['','## B complete materialization, payload case','','| N | Profile | Prepare ms | Full generation ms | Selection ms | Generation verification ms |','| --- | --- | ---: | ---: | ---: | ---: |']
    for n in [128,1000,10000]:
        for profile in [2,3,4]:
            r=indexed.get((n,'B',profile,'single-payload'))
            if r:
                c=compact(r);text.append(f"| {n} | {profile} | {c['prepare_ms']:.2f} | {c['B_generation_ms']:.2f} | {c['select_ms']:.2f} | {c['B_verify_ms']:.2f} |")
    text+=['','N10k B profile4 initialization reached its 120-second child cap before selection. Profiles2/3 were unperformed. No B10k timing is imputed.','']
    (out/'tables.md').write_text('\n'.join(text).rstrip()+'\n',encoding='utf-8')
    print(json.dumps(dict(cells=len(cells),complete=sum(c['status']=='success' for c in cells),primary_rows=sum(r['_role']=='primary complete-view cell' for r in rows),extra_deferred_rows=sum(r['_role']=='extra deferred-view control' for r in rows),measurements=len(rows))))
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--root',required=True);p.add_argument('--out',required=True);a=p.parse_args();run(a.root,a.out)
