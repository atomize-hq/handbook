"""Ordinary userspace file enumeration/read/copy. Never invokes the engine."""
import hashlib,json,pathlib,shutil,sys,time
c=json.loads(sys.argv[1]);root=pathlib.Path(c['root'])
selected=json.loads((root/'SELECTED.json').read_bytes());data=(root/'publications'/(selected['publication']+'.json')).read_bytes();assert hashlib.sha256(data).hexdigest()==selected['digest']
pub=json.loads(data);source=root/(pub['candidate']['closure'] if c['mode']=='B' else 'view')
paths=sorted(p for p in source.rglob('*') if p.is_file())
overview=next(p for p in paths if 'd000000.' in p.name);paths.remove(overview);paths.insert(0,overview)
dest=pathlib.Path(c['destination']);dest.mkdir(parents=True,exist_ok=False)
first=None
for i,p in enumerate(paths):
    rel=p.relative_to(source);out=dest/rel;out.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,out)
    if i==0:
        first=hashlib.sha256(out.read_bytes()).hexdigest();pathlib.Path(c['ready']).write_text('ready')
        start=time.monotonic()
        while not pathlib.Path(c['release']).exists():
            if time.monotonic()-start>60:raise TimeoutError('copy barrier')
            time.sleep(.005)
requirement=next(p for p in dest.rglob('*') if p.is_file() and 'd000001.' in p.name)
print(json.dumps(dict(selected=selected,fixed_path=str(source),files=len(paths),first=first,second=hashlib.sha256(requirement.read_bytes()).hexdigest())))
