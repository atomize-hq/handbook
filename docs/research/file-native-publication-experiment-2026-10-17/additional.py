"""Bounded slow/small sequence and combined import/move checks (32 MiB allowance)."""
import json,pathlib,time
from run import Driver,DEFAULT_BASE,DEFAULT_EXE,request,edit,batch_ops,selected_state,write,sha,check_packet

def main():
    d=Driver(DEFAULT_BASE,DEFAULT_EXE,'additional');observations=[]
    for mode in ['A','B']:
        for policy in ['slot','speculative']:
            root=d.base/'slow-sequence'/f'{mode}-{policy}';d.init(root,mode,'project',128)
            reqs=[request('slow-64',batch_ops(64,'bounded-slow'))]+[request(f'small-{i}',[edit(f'd{100+i:06}',f'\n# small {i}\n')]) for i in range(3)]
            pending=[];started={}
            for r in reqs:
                item=d.start({'action':'publish','root':str(root),'mode':mode,'policy':policy,'request':r})
                pending.append(item);started[r['request']]=item[2]
                time.sleep(.02)
            attempts=[d.finish(p,allowed=(0,2)) for p in pending]
            completions=[]
            for r,out in zip(reqs,attempts):
                refused=out['status']=='error'
                if refused:
                    assert out['error'].startswith(('conflict:','busy:'))
                    out=d.publish(root,mode,r,policy)
                assert out['result']['result']['outcome']=='accepted'
                completions.append({'request':r['request'],'initial_refusal':refused,'accepted_engine_wall_ns':out['wall_ns'],'driver_observed_completion_upper_ns':time.perf_counter_ns()-started[r['request']],'lock_wait_ns':out['metrics']['ns'].get('lock-wait',0),'lock_hold_ns':out['metrics']['ns'].get('lock-hold',0)})
            assert len(d.inspect(root)['history'])==5
            observations.append({'mode':mode,'policy':policy,'slow_documents':64,'small_sequence_length':3,'completions':completions,'fairness_claim':'bounded observed completion only; lock is not FIFO'})
    write(d.base/'slow-sequence-observations.json',observations)
    imports=[]
    for mode in ['A','B']:
        root=d.base/'import-move'/mode;d.init(root,mode,'research')
        base,oldpub,oldstate=selected_state(root)
        editor=root/'private-editor.json';exact=b'{\r\n "value": 1.00, "text":"caf\xc3\xa9"\r\n}\r\n';editor.write_bytes(exact)
        capture=request('captured-import-move',[{'kind':'import','id':'d000002','text':editor.read_bytes().decode('utf-8')},{'kind':'rename','id':'d000002','text':'AUX. café α / moved '}],base)
        prepared=d.call({'action':'prepare','root':str(root),'mode':mode,'request':capture})['result']
        editor.rename(root/'later-editor-move.json');(root/'later-editor-move.json').write_bytes(b'{"uncaptured":true}')
        d.call({'action':'select','root':str(root),'candidate':prepared['file']})
        _,pub,state=selected_state(root);doc=state['docs']['d000002']
        assert doc['path']!=oldstate['docs']['d000002']['path'] and doc['id']=='d000002'
        assert (root/doc['original']).read_bytes()==exact and (root/'later-editor-move.json').read_bytes()==b'{"uncaptured":true}'
        directory=root/(pub['candidate']['closure'] if mode=='B' else 'view')
        assert (directory/doc['path']).read_bytes()==exact and not (directory/oldstate['docs']['d000002']['path']).exists()
        check_packet(root/pub['candidate']['closure']);check_packet(root/oldpub['candidate']['closure']);d.inspect(root,audit=True)
        imports.append({'mode':mode,'stable_id':doc['id'],'captured_sha256':sha(exact),'captured_base':base,'selected_path':doc['path'],'later_editor_edit_and_move':'remained private','old_and_new_retained_packet':'verified'})
    write(d.base/'import-move-observations.json',imports)
    write(d.base/'additional-completed.json',{'slow_small_configurations':len(observations),'combined_import_move_cases':len(imports),'allowance_bytes':32*1024**2,'measurement_condition':'ran alongside one large-tier initialization; host/cache not isolated'})
    print(json.dumps({'slow_small_configurations':len(observations),'import_move_cases':len(imports)}))

if __name__=='__main__':main()
