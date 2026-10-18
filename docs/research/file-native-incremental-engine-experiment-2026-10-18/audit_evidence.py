"""Owned-fixture proof that full audit observes missing evidence before rendering."""
import argparse,json,time
from controller import Driver
from checks import req,result
from verify_store import Tree

def run(d,strict):
    rows=[];start=time.perf_counter_ns()
    for mode in ['A','B']:
        root=d.base/mode/'disposable/store';r=str(root)
        def call(action,allow=False,**kw):return d.call(dict(action=action,root=r,**kw),allow=allow)
        call('init',mode=mode,consumer='project',count=128);call('lifecycle',operation='setup')
        def selected():
            s=json.loads((root/'SELECTED.json').read_bytes())
            return json.loads((root/'publications'/f"{s['publication']}.json").read_bytes())
        p=selected();node=Tree(root).node(p['projection']);assert node[0]=='branch'
        child=node[2][0][0];entries=Tree(root).entries(p['projection'])
        cases=[('projection-child',root/'objects'/child[:2]/child,True),
               ('navigation-source',root/entries['navigation/d000003.md']['source'],True),
               ('projection-root',root/'objects'/p['projection'][:2]/p['projection'],False)]
        for name,path,old_heals in cases:
            saved=path.read_bytes();path.unlink();v=call('audit',allow=True)
            healed=path.exists();rows.append(dict(mode=mode,case=name,status=v['status'],error=v.get('error'),recreated=healed))
            if strict:assert v['status']=='error' and not healed,(name,v)
            else:assert (v['status']=='ok' and healed)==old_heals,(name,v)
            path.write_bytes(saved);call('audit')
        call('publish',request=req('audit-requested-packet',packet=True))
        entries=Tree(root).entries(selected()['projection'])
        for name in ['navigation/d000003.md','packet/context.md','packet/provenance.json']:
            path=root/entries[name]['source'];saved=path.read_bytes()
            for damage in ['missing','corrupt']:
                if damage=='missing':path.unlink()
                else:path.write_bytes(b'corrupt selected immutable source\n')
                v=call('audit',allow=True);healed=path.exists() and path.read_bytes()==saved
                rows.append(dict(mode=mode,case=name,damage=damage,status=v['status'],error=v.get('error'),recreated=healed))
                if strict:assert v['status']=='error' and not healed,(name,damage,v)
                path.write_bytes(saved);call('audit')
    return dict(strict=strict,diagnostic_start_ns=start,diagnostic_end_ns=time.perf_counter_ns(),cases=rows)

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);a.add_argument('--strict',action='store_true');ns=a.parse_args()
    d=Driver(ns.exe,ns.base,900);(d.base/'audit-evidence.json').write_text(json.dumps(run(d,ns.strict),indent=2))
