"""Restore equal content without rolling back publication/outcome or shared-root lifetime."""
import argparse,pathlib
from experiment import Driver,cmd,lifecycle
import baseline_harness as b
p=argparse.ArgumentParser();p.add_argument('--base',type=pathlib.Path,required=True);p.add_argument('--exe',type=pathlib.Path,required=True);p.add_argument('--backend',choices=['manifest','cow'],required=True);a=p.parse_args();d=Driver(a.base,a.exe,'restoration',a.backend)
rows=[]
for mode in ['A','B']:
    root=d.base/'disposable'/mode;d.init(root,mode,'research',16);lifecycle(d,root,'setup')
    first=b.request('first-lost-reply',[b.edit('d000004','\n# first accepted revision\n')])
    d.publish(root,mode,first,env={'EXP_EXIT':'reply'},allowed=(86,));sel1,pub1,s1=b.selected_state(root);oldbytes=(root/s1['docs']['d000004']['original']).read_bytes()
    result1=d.call(cmd(root,'resolve',request_id='first-lost-reply'))['result']
    abandoned=d.call(cmd(root,'prepare',mode=mode,request=b.request('stale-from-first',[b.edit('d000006')],sel1)))['result']
    second=b.request('second-accepted',[b.edit('d000004','\n# later accepted revision\n')]);result2=d.publish(root,mode,second)['result']['result']
    restore=b.request('restore-old-content',[dict(kind='import',id='d000004',text=oldbytes.decode('utf8'))],b.selected_state(root)[0])
    result3=d.publish(root,mode,restore)['result']['result'];sel3,pub3,s3=b.selected_state(root)
    assert result1['state_digest']==result3['state_digest'] and sel1!=sel3
    stale=d.call(cmd(root,'select',candidate=abandoned['file']),allowed=(2,));assert stale['error']=='conflict:expected-publication'
    assert s3['docs']['d000004']['digest']==s1['docs']['d000004']['digest']
    assert d.call(cmd(root,'resolve',request_id='second-accepted'))['result']==result2
    descriptor1=b.load(root/pub1['candidate']['state']);descriptor3=b.load(root/pub3['candidate']['state'])
    if a.backend=='cow':assert descriptor1['root']==descriptor3['root']
    lifecycle(d,root,'retire',selected=sel1);lifecycle(d,root,'collect');d.inspect(root,True)
    assert not (root/pub1['candidate']['state']).exists()
    assert d.call(cmd(root,'resolve',request_id='first-lost-reply'))['result']==result1
    assert d.call(cmd(root,'resolve',request_id='second-accepted'))['result']==result2
    assert d.publish(root,mode,first)['result']['result']==result1
    retired=d.call(cmd(root,'point',selected=sel1,id='d000004'),allowed=(2,));assert retired['error']=='retired:snapshot'
    current=d.call(cmd(root,'point',selected=sel3,id='d000004'));assert current['result']['value']['digest']==s1['docs']['d000004']['digest']
    lifecycle(d,root,'abandon',candidate=abandoned['file']);lifecycle(d,root,'collect')
    refusal=d.call(cmd(root,'select',candidate=abandoned['file']),allowed=(2,));assert refusal['error']=='refused:abandoned-candidate'
    assert not (root/abandoned['candidate']['state']).exists();d.inspect(root,True)
    rows.append(dict(mode=mode,backend=a.backend,helper_sha256=b.sha(pathlib.Path(__file__).read_bytes()),restored_equal_state_digest=result3['state_digest'],distinct_publications=[sel1,sel3],second_accepted_outcome_retained=result2,retired_first_snapshot=True,restored_current_snapshot_survived=True,cow_root_reused=a.backend=='cow' and descriptor1['root']==descriptor3['root'],deleted_old_state_file=not (root/pub1['candidate']['state']).exists(),explicit_abandonment='state reclaimed; later selection refused'))
b.write(d.base/'restoration-observations.json',rows)
