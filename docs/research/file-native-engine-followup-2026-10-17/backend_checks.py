"""Focused actual-page damage, historical reopen and reclaimed-page accounting."""
import argparse,pathlib,json
from experiment import Driver,cmd
import baseline_harness as b
p=argparse.ArgumentParser();p.add_argument('--base',type=pathlib.Path,required=True);p.add_argument('--exe',type=pathlib.Path,required=True);a=p.parse_args()
d=Driver(a.base,a.exe,'backend-checks','cow');root=d.base/'page-damage';d.init(root,'B','research',128)
sel,pub,s=b.selected_state(root);desc=b.load(root/pub['candidate']['state']);page=b.load(root/'objects'/desc['root'][:2]/desc['root'])
assert page['kind']=='branch';last=page['children'][-1]['id'];target=root/'objects'/last[:2]/last
original=target.read_bytes();(d.base/'damaged-page-original.bin').write_bytes(original);target.write_bytes(original[:-1]+b'!')
point=d.call(cmd(root,'point',id='d000001'));assert point['result']['value']['id']=='d000001'
audit=d.call(cmd(root,'inspect',audit=True),allowed=(2,));assert 'integrity-digest' in audit['error']
indirect_points=[]
for mode in ['A','B']:
    for tier in [100000,10000]:
        growth=d.base.parent/'cow'/'growth'/f'{tier}-{mode}-project'
        if not (growth/'SELECTED.json').exists():continue
        binding=b.load(growth/'SELECTED.json');publication=b.load(growth/'publications'/(binding['publication']+'.json'));descriptor=b.load(growth/publication['candidate']['state'])
        node=b.load(growth/'objects'/descriptor['root'][:2]/descriptor['root'])
        while node['kind']=='branch':
            identity=node['children'][0]['id'];node=b.load(growth/'objects'/identity[:2]/identity)
        encoded=bytes(next(value for key,value in node['pairs'] if key=='d000000'));assert encoded.startswith(b'@')
        identity=encoded[1:].decode();data=(growth/'objects'/identity[:2]/identity).read_bytes();assert b.sha(data)==identity
        result=d.call(cmd(growth,'point',selected=binding,id='d000000'));value=result['result']['value'];assert value==json.loads(data)
        assert sum(edge['kind']=='constituent' for edge in value['edges'])==5000
        assert b.sha((growth/value['original']).read_bytes())==value['digest']
        indirect_points.append(dict(root=str(growth),selected=binding,actual_indirect_object=identity,constituents=5000,point_metadata_read=result['metrics']['bytes_metadata_read'],qualification='functional large-value path check; ordinary-key timing table remains separate'))
        break
rows=[]
for mode in ['A','B']:
    r=d.base.parent/'cow'/'disposable'/mode
    if not r.exists():r=d.base/'disposable'/mode
    if r.exists():
        retired=list((r/'retired-attempts').glob('*.json'))
        for record in retired:
            retirement=b.load(record);p=b.load(r/'publications'/(retirement['selected']['publication']+'.json'));c=p['candidate']
            rows.append(dict(mode=mode,retirement=retirement,state_removed=not (r/c['state']).exists(),closure_remaining_files=sum(x.is_file() for x in (r/c['closure']).rglob('*')),remaining_actual_pages=sum(x.is_file() for x in (r/'objects').rglob('*'))))
b.write(d.base/'backend-observations.json',dict(helper_sha256=b.sha(pathlib.Path(__file__).read_bytes()),point='selected path success despite unrelated page damage',audit_error=audit['error'],damaged_page=last,selected=sel,large_indirect_points=indirect_points,retirement=rows))
