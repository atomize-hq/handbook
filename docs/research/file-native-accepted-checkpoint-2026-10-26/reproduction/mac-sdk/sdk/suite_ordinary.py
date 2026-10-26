"""Real selected-once Bash routes. No engine, pack decoder or generation scan."""
import hashlib, json
from pathlib import Path
from suite_bootstrap import need
from native_resources_v3 import strict

SCRIPT=r'''set -euo pipefail
/usr/bin/python3 - "$@" <<'PY'
import hashlib,json,shutil,sys
from pathlib import Path
root,dest,kind,captured=map(str,sys.argv[1:5]);r=Path(root);out=Path(dest)
config=json.loads((r/'STORE.json').read_bytes())
selected=json.loads(captured) if captured else json.loads((r/'SELECTED.json').read_bytes())
if config['mode']=='B':
 raw=(r/('publications/'+selected['publication']+'.json')).read_bytes()
 if hashlib.sha256(raw).hexdigest()!=selected['digest']:raise SystemExit(21)
 pub=json.loads(raw);tree=r/'generations'/pub['attempt']
else:
 if captured:raise SystemExit(22)
 tree=r/'view'
if not tree.is_dir():raise SystemExit(23)
def hashes(base):
 result={}
 for p in sorted(base.rglob('*')):
  if p.is_symlink():raise SystemExit(24)
  if p.is_file():
   h=hashlib.sha256()
   with p.open('rb') as f:
    for chunk in iter(lambda:f.read(1048576),b''):h.update(chunk)
   result[p.relative_to(base).as_posix()]=h.hexdigest()
 return result
before=hashes(tree);out.mkdir(parents=True,exist_ok=False)
shutil.copytree(tree,out/'exact-copy')
after=hashes(out/'exact-copy')
if before!=after:raise SystemExit(25)
titles=[]
if kind=='title':
 if (tree/'navigation.yaml').is_file():
  marker=(tree/'navigation.yaml').read_text(encoding='utf-8')
  if not marker.startswith('version: 4\nlayout: stable-id-sha256-binary-prefix\ngroups: navigation/groups/\n'):raise SystemExit(27)
  parts=sorted((tree/'navigation/groups').glob('*.yaml'))
 elif (tree/'collection.yaml').is_file():parts=[tree/'collection.yaml']
 else:raise SystemExit(28)
 for part in parts:
  for line in part.read_text(encoding='utf-8').splitlines():
   if line.startswith('    title: '):titles.append(json.loads(line[len('    title: '):]))
 if not titles:raise SystemExit(26)
print(json.dumps(dict(selected=selected,files=before,exact_copy=True,title_count=len(titles),kind=kind)))
PY
'''

def ordinary(c,label,selected=None,kind='payload',expected=0):
    need(kind in {'payload','title'},'ordinary-route-kind')
    need(label and '/' not in label and '..' not in label,'owned-output-label')
    destination=c.root.parent/('ordinary-'+label)
    need(not destination.exists(),'preserve-every-ordinary-prefix')
    # Expected upper comes from the independent current-tree census; one copy,
    # never a multiple of the entire history root.
    tree=c.root/'view'
    if c.document('STORE.json')['mode']=='B':
        s=selected or c.selected();raw=c.read('publications/'+s['publication']+'.json')
        need(hashlib.sha256(raw).hexdigest()==s['digest'],'ordinary-control-binding-before-launch')
        tree=c.root/'generations'/strict(raw)['attempt']
    size=files=dirs=0
    if tree.exists():
        for p in tree.rglob('*'):
            need(not p.is_symlink(),'ordinary-source-symlink')
            if p.is_file():size+=p.stat().st_size;files+=1
            elif p.is_dir():dirs+=1
    key='ordinary-copy-'+label
    c.a.budget.reserve_control(key,size,files=files,directories=dirs+2)
    try:
        argv=[c.a.m['bash'],'--noprofile','--norc','-s','--',str(c.root),str(destination),kind,
              json.dumps(selected,separators=(',',':')) if selected else '']
        # Bash -s receives source only after actual Bash ELF/RSS proof.
        row=c.a.commands.run(c.root,'ordinary',argv,SCRIPT.encode(),expected=expected,seconds=30)
    finally:c.a.budget.settle(key)
    if expected!=0:return dict(refused=True,exit=row['exit'],raw_token=row['token'])
    answer=strict(row['stdout'].encode(),1024**2)
    need(answer['exact_copy'] is True,'ordinary-exact-copy-required')
    return answer

def run(c):
    routes=[]
    # Original immediate A/B16/128 payload+title routes:8 actual shell processes.
    for mode in ('A','B'):
        for n in (16,128):
            current=c if mode=='A' and n==16 else c.__class__(c.a,c.id,c.a.alias(c.id,'immediate-'+mode+str(n)))
            current.init(mode,n)
            for kind in ('payload','title'):
                label='immediate-'+mode+'-N'+str(n)+'-'+kind
                routes.append(dict(route=label,result=ordinary(current,label,kind=kind)))
    b=c.__class__(c.a,c.id,c.a.alias(c.id,'selected-once-B'));b.init('B',16)
    selected=b.selected();before=ordinary(b,'original',selected=selected)
    routes.append(dict(route='original',result=before))
    b.call('lifecycle',operation='pin',name='shell-frozen',selected=selected)
    b.publish('after-capture',[dict(kind='label',id='d000004',text='new label')])
    routes.append(dict(route='before-current',result=ordinary(b,'before-current')))
    b.pack('raw',bootstrap=True);b.h.cleanup(b.root,max_batches=160)
    for label,captured in [('frozen-after-cleanup',selected),('current-after-cleanup',None)]:
        row=ordinary(b,label,selected=captured)
        if captured:need(row['files']==before['files'],'frozen-after-cleanup-files')
        routes.append(dict(route=label,result=row))
    b.pack('raw',operation='repack');b.h.cleanup(b.root,max_batches=160)
    row=ordinary(b,'frozen-after-repack',selected=selected);need(row['files']==before['files'],'frozen-after-repack-files')
    routes.append(dict(route='frozen-after-repack',result=row))
    b.publish('maintenance-tail')
    with b.h.barrier(b.root,'ordinary-maintainer','packing','pack-before-catalog',
                     operation='build',codec='raw',force=True,cleanup=False,include_attempts=True):
        b.publish('writer-during-maintenance')
        routes.append(dict(route='frozen-during-build',result=ordinary(b,'frozen-during-build',selected=selected)))
    for label,captured in [('frozen-after-writer',selected),('current-after-writer',None)]:
        row=ordinary(b,label,selected=captured)
        if captured:need(row['files']==before['files'],'frozen-after-writer-files')
        routes.append(dict(route=label,result=row))
    b.call('lifecycle',operation='release',name='shell-frozen')
    b.call('lifecycle',operation='retire',selected=selected);b.call('lifecycle',operation='collect')
    routes.append(dict(route='retired-generation-refuses-copy',result=ordinary(b,'retired-generation-refuses-copy',selected=selected,expected=23)))
    need(len(routes)==17 and len({r['route'] for r in routes})==17,'all-original17-routes')
    proof=b.verify('all17-ordinary-routes')
    c.a.emit(dict(kind='ordinary-all17',routes=routes,extra_routes_invented=False))
    return proof
