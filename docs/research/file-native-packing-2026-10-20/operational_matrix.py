"""Paired A/B ordinary-publication cells from authentic operational images.

One real baseline, fresh independently restored cells, same seven requests and
logical state sequence. A/B and original historical lookups stay distinct.
Large counts require their own parent admission. No control-state rewind.
"""
import argparse,json,statistics,sys,time
from pathlib import Path
from driver import Driver
from independent import Decoder,freeze,verify,sha
HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE.parent/'file-native-recovery-reclamation-2026-10-19'))
import operational_backup as backup
backup.EXCLUDED|={'pack-readers.lock','pack-maintenance.lock'}
R=Path('C:/file-native-packing-20261020');BASE=R/'operational-matrix';OLD=Path('C:/file-native-navigation-20261019/bin/final.exe');NEW=R/'bin/candidate2.exe';HELPER=R/'build/release/qualified_lookup.exe'
def write(path,row):Path(path).write_text(json.dumps(row,indent=2),encoding='utf-8')
def read(path):return json.loads(Path(path).read_bytes())
def initial(mode,count):
    case=BASE/f'n{count}-{mode}';case.mkdir(parents=True,exist_ok=False);d=Driver(case/'baseline-events');root=case/'baseline/disposable/store'
    d.call(root,'init',exe_override=OLD,mode=mode,count=count,consumer='project');d.call(root,'lifecycle',exe_override=OLD,operation='setup');first=d.head(root);d.call(root,'lifecycle',exe_override=OLD,operation='pin',name='initial',selected=first)
    for i in range(7):
        previous=d.head(root);name='lost-reply' if i==1 else f'prefix-{i}'
        d.call(root,'publish',exe_override=OLD,code=86 if i==1 else 0,env={'EXP_EXIT':'reply'} if i==1 else None,request=dict(request=name,base=previous,ops=[dict(kind='label',id='d000004',text=f'prefix-label-{i}')],packet=False))
        assert d.call(root,'resolve',exe_override=OLD,request_id=name)['result']['outcome']=='accepted'
        if previous!=first:d.call(root,'lifecycle',exe_override=OLD,operation='retire',selected=previous);d.call(root,'lifecycle',exe_override=OLD,operation='collect')
    d.call(root,'cancel',exe_override=OLD,request_id='cancelled');verify(root,case/'baseline-independent.json')
    captured=backup.capture(root,case/'baseline-operational-backup','4bde-qualified-loose-source',True)
    requests=[dict(category='early',id='initial'),dict(category='middle',id='prefix-3'),dict(category='recent',id='prefix-6'),dict(category='missing',id='missing'),dict(category='cancelled',id='cancelled'),dict(category='lost-reply',id='lost-reply')]
    write(case/'baseline.json',dict(mode=mode,documents=count,initial=first,selected=d.head(root),history=8,requests=requests,capture=captured,qualified_binary_sha256=sha(OLD.read_bytes())))
    print(json.dumps(dict(status='baseline-captured',mode=mode,documents=count)))
def operations():
    # The existing API couples title and derived path movement in rename; there
    # is no independent path opcode. Labels describe those actual operations.
    return [('payload',[dict(kind='edit',id='d000004',text=' ordinary payload')]),('batch-payload',[dict(kind='edit',id=f'd{i:06}',text=' batch') for i in range(16,24)]),('label',[dict(kind='label',id='d000005',text='ordinary-label')]),('title-and-derived-path',[dict(kind='rename',id='d000004',text='Ordinary exact matched title')]),('batch-title-and-derived-path',[dict(kind='rename',id=f'd{i:06}',text=f'Ordinary batch title {i}') for i in range(16,24)]),('second-derived-path-move',[dict(kind='rename',id='d000005',text='Ordinary second derived location')]),('relationship',[dict(kind='edge',id='d000004',text='cites',target='d000005')])]
def cell(mode,count,representation):
    case=BASE/f'n{count}-{mode}';baseline=read(case/'baseline.json');d=Driver(case/(representation+'-events'));root=case/representation/'disposable/store'
    restored=backup.restore(case/'baseline-operational-backup',root);engine=OLD if representation=='L-old' else NEW
    d.call(root,'inspect',exe_override=engine);d.call(root,'lifecycle',exe_override=engine,operation='setup');d.call(root,'lifecycle',exe_override=engine,operation='pin',name='frozen-cohort',selected=baseline['selected'])
    inventory=case/(representation+'-inventory.jsonl');frozen=freeze(root,inventory,True)
    if representation in ['P0','PZ']:d.call(root,'packing',exe_override=NEW,operation='build',codec='raw' if representation=='P0' else 'zstd',include_attempts=True,cleanup=True)
    # Original cohort lookup costs precede all new loose tail publications.
    warm=d.call(root,'lookup-bench',exe_override=HELPER if representation=='L-old' else NEW,count=500,requests=baseline['requests']);fresh=[]
    for request in baseline['requests']:
        for i in range(20):
            out=d.call(root,'resolve',exe_override=engine,request_id=request['id']);fresh.append(dict(category=request['category'],sample=i,child_seconds=d.events[-1]['seconds'],engine_wall_ns=out['wall_ns'],rss=out.get('peak_memory_bytes'),metrics=out['metrics']))
    publications=[]
    for i,(name,ops) in enumerate(operations()):
        previous=d.head(root);req=dict(request='ordinary-'+str(i),base=previous,ops=ops,packet=False)
        prepared=d.call(root,'prepare',exe_override=engine,request=req);path=prepared['result']['candidate'];c=json.loads((root/path).read_bytes())
        selected=d.call(root,'select',exe_override=engine,candidate=path);assert selected['result']['status']=='published'
        publications.append(dict(case=name,prepare=prepared,select=selected,complete_engine_ns=prepared['wall_ns']+selected['wall_ns'],replay_objects=len(c['replay_objects']),replay_files=len(c['replay_files']),result_state_digest=selected['result']['result']['state_digest']))
        if previous not in [baseline['initial'],baseline['selected']]:d.call(root,'lifecycle',exe_override=engine,operation='retire',selected=previous);d.call(root,'lifecycle',exe_override=engine,operation='collect')
    tail=d.call(root,'lookup-bench',exe_override=HELPER if representation=='L-old' else NEW,count=500,requests=[dict(category='new-loose-tail-separate',id='ordinary-6')])
    verification=verify(root,case/(representation+'-independent.json'),inventory if representation in ['P0','PZ'] else None)
    if representation in ['P0','PZ']:
        expected=[json.loads(line) for line in (case/'L-old-inventory.jsonl').read_text().splitlines()];assert expected==frozen
    if representation!='L-old':
        old=read(case/'L-old-measurement.json');assert [r['result_state_digest'] for r in publications]==[r['result_state_digest'] for r in old['publications']]
        assert [r['result_digest'] for r in warm['result']['rows']]==[r['result_digest'] for r in old['warm']['result']['rows']]
    result=dict(mode=mode,documents=count,representation=representation,history_before=8,history_after=15,restore=restored,cohort_records=len(frozen),warm=warm,fresh=fresh,fresh_process_starts=len(fresh),new_loose_tail=tail,publications=publications,ordinary_median_complete_engine_ns=statistics.median(r['complete_engine_ns'] for r in publications),independent=verification,fixture=str(root),binary_sha256=sha(engine.read_bytes()),cache_qualification='same-process warm after first lookup; fresh process not device cold',retention='same initial and original-cohort pins in each authentic restore; retire only new unpinned predecessor tails outside measured publication clocks')
    write(case/(representation+'-measurement.json'),result);print(json.dumps(dict(status='measured',mode=mode,documents=count,representation=representation,publication_cases=len(publications),median_ns=result['ordinary_median_complete_engine_ns'])))
if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('stage',choices=['initial','cell']);a.add_argument('--mode',required=True);a.add_argument('--count',required=True,type=int);a.add_argument('--representation');n=a.parse_args()
    if n.stage=='initial':initial(n.mode,n.count)
    else:cell(n.mode,n.count,n.representation)
