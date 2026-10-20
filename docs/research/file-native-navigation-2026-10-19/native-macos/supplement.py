import sys,json,time,hashlib,subprocess,os,signal,datetime
from pathlib import Path
OUT=Path(__file__).resolve().parent;sys.path.insert(0,str(OUT.parent))
from focused import Driver
from oracle import verify,Tree
CUTOFF=datetime.datetime.fromisoformat('2026-10-19T23:44:38+00:00').timestamp()
assert time.time()<CUTOFF
exe=OUT/'runtime/build/debug/file-native-incremental';d=Driver(exe,OUT/'runtime/supplement');rows=[]
legacy=b'# File-native collection\n\nRead SELECTED.json once and verify its publication digest. The publication binds exact immutable state. B: retain generations/<attempt>/ once and read collection.yaml and exact source paths. A: view/ may mix/lag; VIEW.json names completed state and PENDING.json records unfinished work. Ordinary edits are not automatically admitted.\n'
for mode in ['A','B']:
 for profile in [1,2,3,4]:
  assert time.time()<CUTOFF
  root=d.init('readme-'+mode+'-'+str(profile),mode,16,profile)
  assert (root/'README.md').read_bytes()==legacy if profile<=3 else not (root/'README.md').exists()
  old=(root/'README.md').read_bytes() if profile<=3 else None
  d.publish(root,'upgrade',[],profile=4)
  if old is not None:assert (root/'README.md').read_bytes()==old
  rows.append(dict(case='genesis-and-historical-readme',mode=mode,profile=profile,passed=True))
 for n in [128,1024]:
  assert time.time()<CUTOFF
  root=d.init('growing-'+mode+'-'+str(n),mode,n,4)
  r=d.publish(root,'growing-title',[dict(kind='rename',id='d000006',text='Growing title with a longer name')])
  metrics=r['prepare']['metrics'];assert metrics['range_rows'].get('D',0)==0
  proof=verify(root);rows.append(dict(case='growing-title',mode=mode,n=n,metrics=metrics,proof=proof))
 root=d.base/('growing-'+mode+'-128')/'disposable/store';sel=d.head(root);pub=json.loads((root/f"publications/{sel['publication']}.json").read_bytes());tree=root/('view' if mode=='A' else 'generations/'+pub['attempt'])
 script='set -eu\ncd "$1"\nfind navigation/groups -type f -name \'*.yaml\' | sort > "$2/groups.txt"\nfirst=$(sed -n \'1p\' "$2/groups.txt")\nsed -n \'/^    source: /{s/^    source: "//;s/"$//;p;}\' "$first" > "$2/sources.txt"\nsource=$(sed -n \'1p\' "$2/sources.txt")\ncp "$source" "$2/copied-source"\nprintf \'%s\\n\' "$source" > "$2/chosen-path.txt"\n'
 destination=d.base/('shell-'+mode);destination.mkdir()
 command=['bash','-c',script,'native-discovery',str(tree),str(destination)]
 result=subprocess.run(command,capture_output=True,timeout=max(.1,min(120,CUTOFF-time.time())))
 assert result.returncode==0,(result.stdout,result.stderr)
 path=(destination/'chosen-path.txt').read_text().strip();projection=Tree(root).entries(pub['projection']);spec=projection[path]
 copied=(destination/'copied-source').read_bytes();assert copied==(root/spec['source']).read_bytes();assert hashlib.sha256(copied).hexdigest()==spec['digest']
 rows.append(dict(case='ordinary-bash-discovery-exact-copy',mode=mode,captured_generation=str(tree),selected_once=sel,command=command,source_path=path,sha256=spec['digest'],stdout=result.stdout.decode(),stderr=result.stderr.decode()))
 (OUT/'evidence/supplement.json').write_text(json.dumps(rows,indent=2))
print(json.dumps(rows,indent=2))
