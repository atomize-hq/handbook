"""Bounded post-matrix audits, qualified censor recovery and exact packet export."""
import argparse,hashlib,json,zipfile,time
from pathlib import Path
from controller import Driver
from checks import req,op,result
from verify_store import Tree

def bounded_source(id,k):
    kind=int(id[1:])%10
    if kind==2:head=f'{{\r\n  "id": "{id}", "exact_number": 1.00, "batch": {k}, "text": "captured café α"\r\n}}\r\n'
    elif kind in [0,1]:head=f'id: {id}\r\nbatch: {k}\r\n# Exact captured source bytes.\r\n'
    else:head=f'# Captured evidence {id}\r\n\r\nBatch {k:02}. Exact bounded source bytes.\r\n'
    return head.encode().ljust(4096,b' ').decode()

def run(d,matrix):
    rows=[]
    # A selected before a cap is distinct from a completed initialization.
    root=matrix/'n100000/disposable/store'
    if (root/'SELECTED.json').exists():
        selected=result(d.call(dict(action='inspect',root=str(root))))
        if (root/'PENDING.json').exists() or selected['publication']['request']!='update-64':
            recovery=d.call(dict(action='resume',root=str(root)),cap=900,allow=True)
            rows.append({'operation':'separate-A100k-recovery','observation':recovery})
    # The main controller can stop at an A censor; B stays an honest separate
    # complete-generation control, launched only if not already attempted.
    extra=None
    if not (matrix/'B-n100000').exists():
        extra=d.base/'B-n100000/disposable/store'
        init=d.call(dict(action='init',root=str(extra),mode='B',consumer='project',count=100000),cap=2400,allow=True)
        rows.append({'operation':'separate-B100k','observation':init})
        if init and init['status']=='ok' and result(init)['status']=='published':
            rows.append({'operation':'separate-B100k-successor','observation':d.call(dict(action='publish',root=str(extra),request=req('B-one',[op('label',text='B-label')])),cap=1200,allow=True)})
    roots=[matrix/f'{prefix}n{n}/disposable/store' for n in [1000,10000,100000] for prefix in ['','B-']]
    if extra:roots.append(extra)
    for root in roots:
        if not (root/'SELECTED.json').exists():
            rows.append({'root':str(root),'audit':'not-selected/unreached'});continue
        if root.parent.parent.name.startswith('n'):
            n=int(root.parent.parent.name[1:])
            for k in [1,8,64]:
                ids=[f'd{(i*n//k+17)%n:06}' for i in range(k)]
                base=result(d.call(dict(action='inspect',root=str(root))))['selected']
                ops=[op('import',id=id,text=bounded_source(id,k)) for id in ids]
                pub=d.call(dict(action='publish',root=str(root),request=req(f'genuine-{n}-{k}',ops,base=base)),cap=1200,env={'EXP_DEFER_REFRESH':'1'},allow=True)
                if pub and pub['status']=='ok':assert len(result(pub)['result']['changed'])==k
                rows.append({'operation':'genuine-scattered-changes','root':str(root),'requested_and_checked_changed':k,'observation':pub})
                (d.base/'post-matrix.json').write_text(json.dumps(rows,indent=2))
                if not pub or pub['status']!='ok':break
                d.call(dict(action='refresh',root=str(root)),cap=900,allow=True)
        audit=d.call(dict(action='audit',root=str(root)),cap=1200,allow=True)
        head=result(d.call(dict(action='inspect',root=str(root))))['publication'];stats={}
        if audit and audit['status']=='ok':
            for name in ['graph','projection','membership','outcomes','changes']:
                tree=Tree(root);tree.entries(head['graph']['tree'] if name=='graph' else head[name]);stats[name]=tree.stats()
        rows.append({'root':str(root),'full_audit':audit,'trees':stats})
        (d.base/'post-matrix.json').write_text(json.dumps(rows,indent=2))
    root=matrix/'n10000/disposable/store'
    for pub in (root/'publications').glob('*.json'):
        p=json.loads(pub.read_bytes())
        if p['request']!='combined-packet':continue
        packet=root/f"attempts/{p['packet_ref']['attempt']}/packet";manifest=(packet/'MANIFEST.json').read_bytes();digest=hashlib.sha256(manifest).hexdigest();assert digest==p['packet_ref']['manifest']
        expected=json.loads(manifest)
        assert {f.relative_to(packet).as_posix() for f in packet.rglob('*') if f.is_file()}==set(expected)|{'MANIFEST.json'}
        with zipfile.ZipFile(d.base/'exact-closure-5000.zip','w',compression=zipfile.ZIP_DEFLATED) as z:
            for name in sorted(set(expected)|{'MANIFEST.json'}):
                b=(packet/name).read_bytes()
                if name!='MANIFEST.json':assert hashlib.sha256(b).hexdigest()==expected[name]
                z.writestr(zipfile.ZipInfo(name,(1980,1,1,0,0,0)),b,compress_type=zipfile.ZIP_DEFLATED)
        originals=[json.loads(l) for l in (matrix/'events.jsonl').read_bytes().splitlines()]
        selected_packet=next(result(v['output']) for v in originals if v['input']['action']=='packet')
        assert digest==selected_packet['manifest_digest'],'selected-only and combined bytes differ'
        rows.append({'packet_manifest':digest,'files':len(expected),'selected_and_combined_exact_equal':True,'archive_sha256':hashlib.sha256((d.base/'exact-closure-5000.zip').read_bytes()).hexdigest()})
    return rows

if __name__=='__main__':
    a=argparse.ArgumentParser();a.add_argument('--exe',required=True);a.add_argument('--base',required=True);a.add_argument('--matrix',required=True);ns=a.parse_args()
    matrix=Path(ns.matrix);started=json.loads((matrix/'session.json').read_bytes())['started']
    d=Driver(ns.exe,ns.base,max(1,started+14400-time.time()))
    (d.base/'post-matrix.json').write_text(json.dumps(run(d,matrix),indent=2))
