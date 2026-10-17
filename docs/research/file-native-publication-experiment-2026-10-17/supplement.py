"""Additional causal tests; run while the main driver is idle (<=8 workers)."""
import json, os, pathlib, subprocess, sys, time
from run import Driver, DEFAULT_BASE, DEFAULT_EXE, request, edit, selected_state, write, load, sha, wait_path, check_packet
HERE=pathlib.Path(__file__).resolve().parent

def copy_reader(cfg):
    root=pathlib.Path(cfg['root']);_,pub,state=selected_state(root)
    source=root/(pub['candidate']['closure'] if cfg['mode']=='B' else 'view');dest=pathlib.Path(cfg['dest'])
    first=state['docs']['d000000']['path']; files=[first]+[p.relative_to(source).as_posix() for p in source.rglob('*') if p.is_file() and p.relative_to(source).as_posix()!=first]
    for i,name in enumerate(files):
        out=dest/name;out.parent.mkdir(parents=True,exist_ok=True)
        with open(source/name,'rb') as a,open(out,'xb') as b:
            while block:=a.read(65536):b.write(block)
        if i==0:
            pathlib.Path(cfg['ready']).write_text('ready');wait_path(pathlib.Path(cfg['release']))
    print(json.dumps({'copied':len(files),'fixed_path':str(source)}))

def main():
    if len(sys.argv)>1 and sys.argv[1]=='copy-reader':copy_reader(json.loads(sys.argv[2]));return
    d=Driver(DEFAULT_BASE,DEFAULT_EXE,'supplement');observations=load(d.base/'semantic-race-observations.json') if (d.base/'semantic-race-observations.json').exists() else []
    d.budget(3*1024**3)
    for mode in ([] if observations else ['A','B']):
        for consumer in ['project','research']:
            for trial in range(30):
                cases=[
                    ('delete-add-edge',[{'kind':'delete','id':'d000010'}],[{'kind':'edge','id':'d000004','text':'cites','target':'d000010'}],'semantic:local-reference'),
                    ('unique',[{'kind':'label','id':'d000004','text':'shared-label'}],[{'kind':'label','id':'d000006','text':'shared-label'}],'semantic:unique'),
                    ('budget',[{'kind':'cost','id':'d000004','value':600}],[{'kind':'cost','id':'d000006','value':600}],'semantic:budget'),
                    ('rename-edge',[{'kind':'rename','id':'d000010','text':'CON. café α '}],[{'kind':'edge','id':'d000004','text':'cites','target':'d000010'}],None),
                ]
                for name,left,right,error in cases:
                    root=d.base/'semantic-races'/f'{mode}-{consumer}-{trial:02}-{name}';d.init(root,mode,consumer,16);base=selected_state(root)[0]
                    commands=[{'action':'prepare','root':str(root),'mode':mode,'request':request(side,ops,base)} for side,ops in [('left',left),('right',right)]]
                    pending=[d.start(c) for c in commands];prepared=[d.finish(p)['result'] for p in pending]
                    d.call({'action':'select','root':str(root),'candidate':prepared[0]['file']})
                    conflict=d.call({'action':'select','root':str(root),'candidate':prepared[1]['file']},allowed=(2,));assert conflict['error']=='conflict:expected-publication'
                    after=selected_state(root)[0]
                    revalidated=d.publish(root,mode,request('right',right,after),allowed=(2,) if error else (0,))
                    if error:assert revalidated['error']==error and selected_state(root)[0]==after
                    else:
                        _,pub,state=selected_state(root);assert state['docs']['d000004']['edges'][-1]['target']=='d000010'
                        assert 'item-con' in state['docs']['d000010']['path'];d.inspect(root,audit=True)
                    observations.append({'mode':mode,'consumer':consumer,'trial':trial,'case':name,'explicit_revalidation':error or 'accepted-stable-reference'})
            print(f'semantic races {mode}/{consumer}: 30 x 4 passed',flush=True)
    write(d.base/'semantic-race-observations.json',observations)
    # Suspend an actual userspace directory copy after the overview file.
    copies=[]
    for mode in ['A','B']:
        suffix=f'{mode}-{time.time_ns()}'
        root=d.base/'supplement-copy'/suffix;d.init(root,mode,'project');_,_,old=selected_state(root)
        cfg={'root':str(root),'mode':mode,'dest':str(d.base/'copies'/f'suspended-{suffix}'),'ready':str(root/'COPY-READY'),'release':str(root/'COPY-RELEASE')}
        p=subprocess.Popen([sys.executable,str(HERE/'supplement.py'),'copy-reader',json.dumps(cfg)],stdout=subprocess.PIPE,stderr=subprocess.PIPE)
        wait_path(root/'COPY-READY',p);d.publish(root,mode,request('during-copy',[edit(),edit('d000001')]))
        (root/'COPY-RELEASE').write_text('go');out,err=p.communicate(timeout=30);assert p.returncode==0,err
        dest=pathlib.Path(cfg['dest']);_,_,new=selected_state(root)
        assert sha((dest/old['docs']['d000000']['path']).read_bytes())==old['docs']['d000000']['digest']
        assert sha((dest/old['docs']['d000001']['path']).read_bytes())==(old if mode=='B' else new)['docs']['d000001']['digest']
        if mode=='B':
            manifest=load(dest/'MANIFEST.json');assert all(sha((dest/n).read_bytes())==h for n,h in manifest['files'].items());check_packet(dest)
        copies.append({'mode':mode,'observation':'A/A coherent retained generation' if mode=='B' else 'A/B mixed mutable copy','copy':json.loads(out)})
    write(d.base/'suspended-copy-observations.json',copies)
    checks=[]
    for mode in ['A','B']:
        root=d.base/'supplement-validation'/mode;d.init(root,mode,'research')
        # Null-base import is refused on all routes; nested duplicates are refused.
        r=request('uncaptured',[{'kind':'import','id':'d000002','text':'{}'}])
        for action,policy in [('publish','slot'),('publish','speculative'),('prepare','speculative')]:
            result=d.call({'action':action,'root':str(root),'mode':mode,'policy':policy,'request':r},allowed=(2,));assert result['error']=='refused:import-base-required';checks.append({'mode':mode,'check':'import-base','route':action+'/'+policy,'result':result['error']})
        text=json.dumps({'action':'publish','root':str(root),'mode':mode,'request':request('nested',[edit()])})
        text=text.replace('"kind": "edit"','"kind": "delete", "kind": "edit"')
        result=subprocess.run([str(d.exe)],input=text.encode(),capture_output=True);out=json.loads(result.stdout);assert result.returncode==2 and 'duplicate-key:kind' in out['error'];checks.append({'mode':mode,'check':'nested-duplicate','output':out})
        # Same-process partial payload failures use distinct owned stages, then succeed.
        commands=[request(f'partial-{i}',[edit('d000004',f'\n# partial {i}\n')]) for i in range(3)]
        # repeat-error exercises same-process staged candidate failures; partial
        # payload is separately exercised with actual short staged bytes by run.py.
        d.call({'action':'repeat-error','root':str(root),'mode':mode,'request':commands[0]})
        # A missing projection is distinct from missing immutable authority bytes.
        if mode=='A':
            _,_,s=selected_state(root);view=root/'view'/s['docs']['d000001']['path'];view.unlink()
            d.inspect(root,audit=True);checks.append({'mode':mode,'check':'missing-view-authority-intact','result':'audit passed'})
    write(d.base/'supplement-validation.json',checks)
    write(d.base/'supplement-completed.json',{'semantic_race_observations':len(observations),'copies':len(copies),'validation_checks':len(checks)})

if __name__=='__main__':main()
