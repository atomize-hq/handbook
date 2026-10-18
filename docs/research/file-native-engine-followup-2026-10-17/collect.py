"""Freeze raw logs, checksums and summaries; safe to repeat into a fresh output."""
import argparse,gzip,hashlib,json,pathlib,statistics
import baseline_harness as b
from account import retention_account
p=argparse.ArgumentParser();p.add_argument('--root',type=pathlib.Path,required=True);p.add_argument('--output',type=pathlib.Path,required=True);p.add_argument('--account',action='store_true');a=p.parse_args();a.output.mkdir(parents=True,exist_ok=True)
index=[];operations=[];points=[];accounts=[];observations=[];bindings={}
postcap={}
for directory in [a.root,a.root/'manifest',a.root/'cow']:
    for name,phase in [('continuation-observations.json','post-cap A continuation'),('retained-probe-observations.json','post-cap retained probe')]:
        path=directory/name
        if path.exists():
            for row in b.load(path):
                if 'root' in row and 'session' in row:postcap[row['root'],row['session']]=phase
for path in sorted(a.root.rglob('*.jsonl')):
    # Runtime logs only, not copied source/evidence or generated document JSON.
    if 'source' in path.relative_to(a.root).parts:continue
    data=path.read_bytes();rows=[json.loads(s) for s in data.splitlines() if s.strip()]
    relative=path.relative_to(a.root).as_posix();dest=a.output/(relative.replace('/','--')+'.gz')
    with dest.open('wb') as f:
        with gzip.GzipFile(fileobj=f,mode='wb',mtime=0) as z:z.write(data)
    index.append(dict(file=dest.name,source=relative,records=len(rows),bytes=len(data),sha256=b.sha(data),compressed_sha256=b.sha(dest.read_bytes())))
    if path.name!='growth-raw.jsonl':continue
    for position,r in enumerate(rows,1):
        c=r.get('command',{});out=r.get('output') or {};root=c.get('root','');action=c.get('action');m=out.get('metrics',{})
        base=dict(backend=r.get('backend'),root=root,action=action,raw_file=dest.name,raw_line=position,censored=r.get('censored',False),external_ms=r.get('external_wall_ns',0)/1e6,engine_ms=out.get('wall_ns',0)/1e6,peak_memory_bytes=out.get('peak_memory_bytes'),bytes_read=m.get('bytes_read'),bytes_written=m.get('bytes_written'),metadata_read=m.get('bytes_metadata_read'),metadata_written=m.get('bytes_metadata_written'),stage_ms={k:v/1e6 for k,v in m.get('ns',{}).items()},status=out.get('status'),error=out.get('error'))
        base['session']=r.get('session');base['measurement_phase']=postcap.get((root,r.get('session')),'measured matrix')
        base['exit']=r.get('exit')
        if action in ['init','publish','packet']:
            delivery=out.get('result') or {}
            base['delivery_status']=delivery.get('status');base['publication_outcome']=(delivery.get('result') or {}).get('outcome')
            base['count']=c.get('count');base['request']=c.get('request',{}).get('request');base['packet_requested']=c.get('request',{}).get('packet',c.get('packet',True));operations.append(base)
        if action=='point':
            if root not in bindings:
                head=b.load(pathlib.Path(root)/'SELECTED.json');cursor=head
                while cursor:
                    data=(pathlib.Path(root)/'publications'/(cursor['publication']+'.json')).read_bytes();assert b.sha(data)==cursor['digest']
                    publication=json.loads(data);initial=cursor['publication'];cursor=publication['predecessor']
                bindings[root]=(head['publication'],initial)
            base['key']=c['id'];base['publication']=c.get('selected',{}).get('publication')
            current,initial=bindings[root];base['binding_role']='current' if base['publication']==current else 'initial retained' if base['publication']==initial else 'other binding'
            points.append(base)
for lane in ['base','manifest','cow','cow-extra']:
    directory=a.root if lane=='base' else a.root/lane
    if not directory.exists():continue
    paths=set(directory.glob('*observations.json'))|set(directory.glob('*-completed.json'))|set(directory.glob('*-stopped.json'))|set(directory.glob('manual-stop.json'))|set(directory.glob('failed-phases.json'))|set(directory.glob('harness-session-*.json'))
    for path in sorted(paths):
        out=a.output/(lane+'--'+path.name);out.write_bytes(path.read_bytes());observations.append(out.name)
    if a.account:
        for category in ['growth','contention']:
            parent=directory/category
            if not parent.exists():continue
            for root in sorted(parent.iterdir()):
                if root.is_dir() and (root/'SELECTED.json').exists():accounts.append(dict(root=str(root),backend=lane,account=retention_account(root)))
for name,obj in [('raw-index.json',index),('operations.json',operations),('points.json',points),('accounting.json',accounts),('observations-index.json',observations)]:b.write(a.output/name,obj)
print(json.dumps(dict(raw_logs=len(index),raw_records=sum(i['records'] for i in index),operations=len(operations),points=len(points),accounted_stores=len(accounts))))
