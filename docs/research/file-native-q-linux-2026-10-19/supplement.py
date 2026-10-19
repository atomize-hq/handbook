"""Final external-empty guard and ordinary Bash extraction; no baseline edits."""
import pathlib,sys,subprocess,json,hashlib,os,shutil
R=pathlib.Path(__file__).parent;F=R/'source/docs/research/file-native-recovery-reclamation-2026-10-19';sys.path.insert(0,str(F));from oracle import verify,navigation
exe=sys.argv[1];events=[];rows=[]
def call(root,action,**kw):
    v=dict(root=str(root),action=action,**kw);p=subprocess.run([exe],input=json.dumps(v).encode(),capture_output=True,timeout=120);events.append(dict(input=v,exit=p.returncode,stdout=p.stdout.decode(),stderr=p.stderr.decode()));(R/'supplement-events.json').write_text(json.dumps(events,indent=2));assert p.returncode==0,events[-1];return json.loads(p.stdout)
for mode in ['A','B']:
    root=R/('ordinary-'+mode)/'store';call(root,'init',mode=mode,count=16,consumer='project')
    if mode=='A':
        empty=root/'view/external-empty/child';empty.mkdir(parents=True);call(root,'resume');assert empty.is_dir();rows.append(dict(case='external-empty-directory',status='success'))
        # External directory is intentional; exact ordinary oracle excludes it after removing empty dirs only in owned test fixture.
        empty.rmdir();empty.parent.rmdir()
    info=call(root,'inspect')['result'];p=info['publication'];tree=root/('view' if mode=='A' else 'generations/'+p['attempt']);expected=navigation(tree)
    copied=R/('bash-copy-'+mode);script='set -eu; find "$1" -type f > "$3"; cat "$1/collection.yaml" > /dev/null; mkdir "$2"; cp -R "$1/." "$2/"'
    q=subprocess.run(['bash','-c',script,'q',str(tree),str(copied),str(R/('bash-list-'+mode+'.txt'))],capture_output=True,timeout=120);assert q.returncode==0,q.stderr
    assert navigation(copied)==expected;verify(root,readable=copied)
    rows.append(dict(case='ordinary-Bash-bound-tree-extraction',mode=mode,documents=len(expected),status='success'))
    from verify_operational import verify as operational
    (R/('independent-'+mode+'.json')).write_text(json.dumps(operational(root),indent=2))
# Native namespace, immutable no-replace installation, replacement and directory fsync APIs.
probe=R/'native-probe';probe.mkdir();(probe/'Case').write_bytes(b'A');(probe/'case').write_bytes(b'B');assert len(list(probe.iterdir()))==2
stage=probe/'stage';stage.write_bytes(b'checked');fd=os.open(stage,os.O_RDONLY);os.fsync(fd);os.close(fd);os.link(stage,probe/'immutable')
try:os.link(stage,probe/'immutable')
except FileExistsError:pass
else:raise AssertionError('replace by hardlink')
stage.unlink();stage.write_bytes(b'replacement');os.replace(stage,probe/'selector');fd=os.open(probe,os.O_RDONLY);os.fsync(fd);os.close(fd)
rows.append(dict(case='native-filesystem-API-probes-not-engine-race-suite',case_sensitive=True,directory_fsync=True,no_replace=True,replace=True,status='success'))
(R/'supplement-summary.json').write_text(json.dumps(rows,indent=2))
