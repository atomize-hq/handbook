"""Independent malformed-input and catalog-without-membership supplements."""
import argparse,copy,json,shutil,struct
from pathlib import Path
import independent
from independent import Decoder,sha
from driver import Driver
from capsule import rebind
def raw_container(root,key,raw):
    h=sha(raw);index=b'FNPIDX01'+bytes.fromhex(h)+struct.pack('<QBI',len(raw),0,1)+struct.pack('<H',len(key.encode()))+key.encode()+struct.pack('<QII',0,len(raw),len(raw))+bytes.fromhex(sha(raw))
    ih=sha(index);(root/f'packs/{h}.pack').write_bytes(raw);(root/f'packs/{ih}.idx').write_bytes(index)
    return dict(pack=h,index=ih,pack_bytes=len(raw),index_bytes=len(index),records=1,codec=0,role=independent.role(key),first=key,last=key)
def run(capsule_base,base):
    capsule_base=Path(capsule_base);base=Path(base);d=Driver(base);cases=[]
    for directory in sorted((capsule_base/'malformed').iterdir()):
        try:Decoder(directory/'disposable/store').verify_packs()
        except (AssertionError,ValueError,OSError,KeyError,IndexError,struct.error):cases.append(dict(case='independent-'+directory.name,status='refused'))
        else:raise AssertionError('independent decoder accepted malformed '+directory.name)
    seed=capsule_base/'malformed-seed-A/disposable/store'
    for field in ['version','records','pack_bytes','index_bytes','codec']:
        root=base/('bool-'+field)/'disposable/store';shutil.copytree(seed,root);c=json.loads((root/'STORE.json').read_bytes());cat=json.loads((root/f"pack-catalogs/{c['catalog']}.json").read_bytes())
        if field=='version':cat['version']=True
        else:cat['packs'][0][field]=True
        rebind(root,cat)
        opened=[];original=independent.bounded
        def checked(path,*args,**kw):opened.append(str(path));return original(path,*args,**kw)
        independent.bounded=checked
        try:
            try:Decoder(root)
            except AssertionError:pass
            else:raise AssertionError('numeric bool accepted')
            assert not any(path.endswith('.idx') for path in opened)
        finally:independent.bounded=original
        cases.append(dict(case='strict-numeric-type-'+field,status='refused-before-index-allocation'))
    for field,size in [('catalog',65537),('index',16*1024**2+1)]:
        root=base/('oversize-'+field)/'disposable/store';shutil.copytree(seed,root);c=json.loads((root/'STORE.json').read_bytes());cp=root/f"pack-catalogs/{c['catalog']}.json";cat=json.loads(cp.read_bytes())
        path=cp if field=='catalog' else root/f"packs/{cat['packs'][0]['index']}.idx"
        with path.open('r+b') as f:f.truncate(size)
        try:Decoder(root)
        except AssertionError as e:assert 'before-allocation-file-size-binding' in str(e)
        else:raise AssertionError('oversize input accepted')
        cases.append(dict(case='independent-oversize-'+field,status='refused-before-allocation'))
    root=base/'orphan-physical-catalog'/'disposable/store';shutil.copytree(seed,root);d.call(root,'lifecycle',operation='setup');selected=(root/'SELECTED.json').read_bytes();s=json.loads(selected);decoder=Decoder(root)
    p=json.loads(decoder.raw(f"publications/{s['publication']}.json"));p['id']='zz-orphan';p['attempt']='zz-orphan';p['request']='orphan-request';p['intent']='0'*64;p['predecessor']=s;p['sequence']+=1;p['allocation_owner']['attempt']='zz-orphan'
    result=json.dumps(dict(outcome='accepted',request=p['request'],publication=p['id'],state_digest=sha(json.dumps(p['graph'],indent=2).encode()),changed=[],removed=[]),indent=2).encode();p['result_hash']=sha(result);(root/f"results/{p['result_hash']}.json").write_bytes(result)
    raw=json.dumps(p,indent=2).encode();orphan=dict(publication=p['id'],digest=sha(raw));hint=json.dumps(orphan,indent=2).encode()
    c=json.loads((root/'STORE.json').read_bytes());cat=json.loads((root/f"pack-catalogs/{c['catalog']}.json").read_bytes())
    cat['packs'].extend([raw_container(root,'publications/zz-orphan.json',raw),raw_container(root,'attempt-hints/zz-orphan.json',hint)]);cat['packs'].sort(key=lambda x:(x['role'],x['first']));rebind(root,cat)
    Decoder(root).verify_packs()
    assert d.call(root,'resolve',request_id='orphan-request')['result']==dict(outcome='absent-as-of',terminal_noncommit=False)
    assert d.call(root,'point',code=2,selected=orphan,id='d000000')['error']=='refused:not-selected-history'
    assert d.call(root,'select',code=2,candidate='attempts/zz-orphan/candidate.json')['result'] is None
    assert (root/'SELECTED.json').read_bytes()==selected
    cases.append(dict(case='physical-catalog-and-hint-cannot-grant-membership',selected_control_unchanged=True,status='passed'))
    result=dict(status='passed',cases=cases,engine_events=len(d.events));(base/'summary.json').write_text(json.dumps(result,indent=2));print(json.dumps(dict(status='passed',cases=len(cases)),indent=2));return result
if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--capsule-base',required=True);a.add_argument('--base',required=True);n=a.parse_args();run(n.capsule_base,n.base)
