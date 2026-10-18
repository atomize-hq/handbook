"""Finite corruption/refusal checks outside measured workloads."""
import argparse, hashlib, json
from controller import Driver
from checks import req, op, result, expect_error, cleanup, reseal
from verify_store import Tree

def run(d):
    root=d.base/'disposable'/'store';r=str(root);rows=[]
    def call(action,allow=False,env=None,**kw):return d.call(dict(action=action,root=r,**kw),allow=allow,env=env)
    call('init',mode='A',consumer='project',count=128);call('lifecycle',operation='setup')
    for field,value in [('version',9),('store_id','wrong'),('graph',None),('changes','0'*64),('removed',['invented'])]:
        prepared=result(call('prepare',request=req('forged-'+field,[])))['candidate'];p=root/prepared;c=json.loads(p.read_bytes())
        if field=='graph':c['graph']['cost']+=1
        else:c[field]=value
        p.write_text(json.dumps(c));reseal(root,p)
        expect_error(call('select',allow=True,candidate=prepared));rows.append('forged-'+field)
        cleanup(d,r)
    # Force a genuine indirect header while keeping its normal file path bounded.
    call('publish',request=req('large-header',[op('rename',text='x'*3800),op('label',text='z'*800)]))
    head=result(call('inspect'));t=Tree(root);entries=t.entries(head['publication']['graph']['tree'])
    header=entries['D/d000003'];assert len(json.dumps(header).encode())>4096
    indirect=hashlib.sha256(json.dumps(header,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()
    # Find the exact indirection object independently of serialization key order.
    candidates=[]
    for path in (root/'objects').glob('*/*'):
        if path.stat().st_size>4096:
            b=path.read_bytes()
            try:
                if json.loads(b)==header:candidates.append(path)
            except (ValueError,UnicodeDecodeError):pass
    assert len(candidates)==1;path=candidates[0];saved=path.read_bytes();path.write_bytes(b'corrupt')
    call('point',id='d000099') # Unrelated point does not require the indirect value.
    expect_error(call('point',allow=True,id='d000003'),'object-digest')
    expect_error(call('audit',allow=True),'object-digest')
    expect_error(call('lifecycle',allow=True,operation='collect'),'object-digest')
    path.write_bytes(saved);call('audit');rows.append('indirect-point-audit-gc')
    # Missing required selected roots and result evidence must fail closed.
    head=result(call('inspect'))
    for category,h,action,kwargs in [
        ('graph',head['publication']['graph']['tree'],'point',{'id':'d000003'}),
        ('history',head['publication']['outcomes'],'resolve',{'request_id':'missing'}),
        ('result',head['publication']['result_hash'],'resolve',{'request_id':head['publication']['request']})]:
        p=root/('results/'+h+'.json' if category=='result' else 'objects/'+h[:2]+'/'+h)
        saved=p.read_bytes();p.unlink();expect_error(call(action,allow=True,**kwargs));p.write_bytes(saved)
        p.write_bytes(b'corrupt');expect_error(call(action,allow=True,**kwargs));p.write_bytes(saved)
        rows.append('missing-and-corrupt-'+category)
    call('audit')
    return {'passed':rows,'large_header_indirection':True}

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);ns=a.parse_args()
    d=Driver(ns.exe,ns.base,1800);(d.base/'adversarial.json').write_text(json.dumps(run(d),indent=2))
