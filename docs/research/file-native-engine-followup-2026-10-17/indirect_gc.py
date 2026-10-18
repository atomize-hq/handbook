"""Actual retained/reclaimed large metadata indirections on disposable COW roots."""
import argparse,json,pathlib
from experiment import Driver,cmd,lifecycle
import baseline_harness as b
p=argparse.ArgumentParser();p.add_argument('--base',type=pathlib.Path,required=True);p.add_argument('--exe',type=pathlib.Path,required=True);a=p.parse_args()
d=Driver(a.base,a.exe,'indirect-gc','cow');rows=[]
for mode in ['A','B']:
    root=d.base/'disposable'/mode;d.init(root,mode,'research',16);lifecycle(d,root,'setup');snapshots=[]
    requests=[{**b.request('long-metadata',[dict(kind='rename',id='d000004',text='large metadata '+'m'*5000)]),'packet':False},{**b.request('changed-large-metadata',[b.edit('d000004')]),'packet':False}]
    for request in requests:
        result=d.publish(root,mode,request)['result']['result'];selected,publication,state=b.selected_state(root)
        descriptor=b.load(root/publication['candidate']['state']);node=b.load(root/'objects'/descriptor['root'][:2]/descriptor['root'])
        while node['kind']=='branch':
            identity=node['children'][0]['id'];node=b.load(root/'objects'/identity[:2]/identity)
        encoded=bytes(next(value for key,value in node['pairs'] if key=='d000004'));assert encoded.startswith(b'@')
        identity=encoded[1:].decode();path=root/'objects'/identity[:2]/identity;data=path.read_bytes();assert b.sha(data)==identity and json.loads(data)==state['docs']['d000004']
        snapshots.append(dict(selected=selected,identity=identity,path=path,result=result))
    old,current=snapshots;assert old['identity']!=current['identity']
    lifecycle(d,root,'collect');assert old['path'].exists() and current['path'].exists();d.inspect(root,True)
    lifecycle(d,root,'retire',selected=old['selected']);lifecycle(d,root,'collect')
    assert not old['path'].exists() and current['path'].exists();d.inspect(root,True)
    point=d.call(cmd(root,'point',selected=current['selected'],id='d000004'));assert len(point['result']['value']['title'])>4096
    assert d.call(cmd(root,'resolve',request_id='long-metadata'))['result']==old['result']
    assert d.publish(root,mode,requests[0])['result']['result']==old['result'];assert not old['path'].exists()
    rows.append(dict(mode=mode,helper_sha256=b.sha(pathlib.Path(__file__).read_bytes()),old_indirect_object=old['identity'],current_indirect_object=current['identity'],both_protected_before_retirement=True,obsolete_indirect_object_actually_reclaimed=True,current_indirect_object_survived=True,original_accepted_outcome_retained=old['result']))
b.write(d.base/'indirect-gc-observations.json',rows)
print(json.dumps(rows,indent=2))
