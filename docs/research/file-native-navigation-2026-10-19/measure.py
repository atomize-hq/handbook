"""One bounded disposable cell; fixture cleanup is the caller's explicit stage."""
import argparse,hashlib,json,time
from pathlib import Path
from focused import Driver,rss
from oracle import Tree,verify,parse_group
from census import census

def run(exe,base,n,mode,profile,deadline):
    d=Driver(exe,base);root=d.init('cell',mode,n,profile);d.call(root,'lifecycle',operation='setup');rows=[]
    def selected():return json.loads((root/f"publications/{d.head(root)['publication']}.json").read_bytes())
    def projection():return Tree(root).entries(selected()['projection'])
    def graph():return Tree(root).entries(selected()['graph']['tree'])
    def navfiles(p):return {k:v['digest'] for k,v in p.items() if k=='collection.yaml' or k=='navigation.yaml' or k.startswith('navigation/')}
    def change(label,ops,external=None):
        assert time.time()<deadline,'original-task-deadline-censor'
        before=d.head(root);prior=projection()
        if external is not None:
            tree=root/('view' if mode=='A' else 'generations/'+selected()['attempt']);path=tree/external;saved=path.read_bytes();path.write_bytes(b'external measured navigation')
            if mode=='B':
                try:verify(root)
                except AssertionError:
                    (d.base/'external-edit-expected-refusal.json').write_text(json.dumps(dict(check='independent ordinary-tree hash verification',status='expected-refusal',path=str(path))),encoding='utf-8')
                else:raise AssertionError('external B edit escaped independent verifier')
                path.write_bytes(saved)
        request=dict(request=label,base=before,ops=ops,packet=False)
        prep=d.call(root,'prepare',profile=profile,request=request);candidate=prep['result']['candidate'];c=json.loads((root/candidate).read_bytes());target=Tree(root).entries(c['projection'])
        changed=[p for p in prior.keys()|target.keys() if prior.get(p)!=target.get(p)]
        chosen=d.call(root,'select',profile=profile,candidate=candidate,defer=True)
        refresh=d.call(root,'refresh',profile=profile) if mode=='A' else None
        start=time.monotonic();proof=verify(root);oracle_seconds=time.monotonic()-start
        assert oracle_seconds<900
        if label in ['single-payload','batch-payload','label'] and profile>=3:
            assert prep['metrics']['navigation_parts']==chosen['metrics']['navigation_parts']==0
            assert prep['metrics']['range_rows'].get('D',0)==0
        if label in ['single-payload','batch-payload','label'] and profile==2:assert prep['metrics']['range_rows']['D']==n
        if external is not None and mode=='A':assert b'external measured navigation' in [p.read_bytes() for p in (root/'divergence').rglob('*') if p.is_file()]
        row=dict(case=label,n=n,mode=mode,profile=profile,prepare_seconds=prep['wall_ns']/1e9,select_seconds=chosen['wall_ns']/1e9,refresh_seconds=refresh['wall_ns']/1e9 if refresh else None,prepare_metrics=prep['metrics'],select_metrics=chosen['metrics'],refresh_metrics=refresh['metrics'] if refresh else None,changed_paths=sorted(changed),navigation_changed_paths=sorted(p for p in changed if p in navfiles(prior)|navfiles(target)),navigation_files=len(navfiles(target)),replay_objects=len(c['replay_objects']),replay_files=len(c['replay_files']),oracle_seconds=oracle_seconds,oracle=proof,checker_peak_rss=rss(__import__('os').getpid()))
        rows.append(row);(d.base/'measurements.json').write_text(json.dumps(rows,indent=2),encoding='utf-8')
        # Retire only this disposable cell's preceding snapshot after successful
        # proof; preserve permanent accepted outcomes. Outside measured clocks.
        d.call(root,'lifecycle',operation='retire',selected=before);d.call(root,'lifecycle',operation='collect')
        footprint=census(root);row['post_collection_census']=footprint
        assert footprint['logical_bytes']<512*1024**2 and footprint['files']<80000
        assert not row['checker_peak_rss'] or row['checker_peak_rss']<=2*1024**3
        (d.base/'measurements.json').write_text(json.dumps(rows,indent=2),encoding='utf-8')
        print(json.dumps(dict(n=n,mode=mode,profile=profile,case=label,prepare_s=row['prepare_seconds'],navigation_parts=prep['metrics']['navigation_parts'],D_rows=prep['metrics']['range_rows'].get('D',0),E_rows=prep['metrics']['range_rows'].get('E',0),changed_paths=len(changed))),flush=True)
        return row
    ids=[f'd{(i*n//8+17)%n:06}' for i in range(8)]
    change('single-payload',[dict(kind='edit',id='d000004',text=' one payload')])
    change('batch-payload',[dict(kind='edit',id=id,text=' batch payload') for id in ids])
    change('label',[dict(kind='label',id='d000005',text='unexported measurement')])
    g=graph();path=g['D/d000006']['doc']['path']
    change('single-title',[dict(kind='rename',id='d000006',text='Measured title'),dict(kind='path',id='d000006',text=path)])
    g=graph();ops=[]
    for id in ids:ops.extend([dict(kind='rename',id=id,text='Batch title '+id),dict(kind='path',id=id,text=g['D/'+id]['doc']['path'])])
    change('batch-title',ops)
    change('path',[dict(kind='path',id='d000005',text='docs/item-measured-d000005/source.json')])
    change('relations',[dict(kind='edge',id='d000004',text='cites',target='d000005')]*2)
    change('high-degree',[dict(kind='edge',id='d000000',text='requires' if i%2 else 'cites',target='d000005') for i in range(130)])
    g=graph();path=g['D/d000000']['doc']['path']
    change('high-degree-title',[dict(kind='rename',id='d000000',text='High degree title'),dict(kind='path',id='d000000',text=path)])
    g=graph();p=projection();prior_nav=navfiles(p)
    if profile==4:
        groups=[(k,parse_group((root/v['source']).read_bytes())) for k,v in p.items() if k.startswith('navigation/groups/')]
        key,group=max(groups,key=lambda x:len(x[1]));split_ids=[id for id in sorted(group) if id!='d000009'][:64]
    else:split_ids=[f'd{i:06}' for i in range(16,80)]
    assert len(split_ids)==64
    split=change('split',[dict(kind='rename',id=id,text='S'*4080+id) for id in split_ids])
    if profile==4:assert split['navigation_files']>len(prior_nav)
    ops=[]
    for id in split_ids:ops.extend([dict(kind='rename',id=id,text=g['D/'+id]['doc']['title']),dict(kind='path',id=id,text=g['D/'+id]['doc']['path'])])
    change('merge',ops);assert navfiles(projection())==prior_nav
    p=projection();external='collection.yaml' if profile!=4 else sorted(k for k in p if k.startswith('navigation/groups/'))[0]
    target='d000006' if profile!=4 else next(iter(parse_group((root/p[external]['source']).read_bytes())))
    change('external-edit',[dict(kind='rename',id=target,text='External edit measured')],external=external)
    g=graph();ops=[]
    for k in g:
        if k.startswith('R/d000004/'):
            _,_,source,ordinal=k.split('/');ops.append(dict(kind='drop-edge',id=source,value=int(ordinal,16)))
    change('delete',ops+[dict(kind='delete',id='d000004')])
    d.call(root,'audit',profile=profile)
    summary=dict(status='success',n=n,mode=mode,profile=profile,cases=len(rows),seconds=time.monotonic()-d.start,fixture=str(root),final=census(root),exe_sha256=hashlib.sha256(Path(exe).read_bytes()).hexdigest())
    (d.base/'summary.json').write_text(json.dumps(summary,indent=2),encoding='utf-8');return summary
if __name__=='__main__':
    p=argparse.ArgumentParser();p.add_argument('--exe',required=True);p.add_argument('--base',required=True);p.add_argument('--n',type=int,required=True);p.add_argument('--mode',choices=['A','B'],required=True);p.add_argument('--profile',type=int,choices=[2,3,4],required=True);p.add_argument('--deadline',type=float,required=True);a=p.parse_args();run(a.exe,a.base,a.n,a.mode,a.profile,a.deadline)
