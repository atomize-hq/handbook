import sys,time
T0=time.time()
import os,json,hashlib,gzip,shlex,importlib.util
from pathlib import Path
H=lambda b:hashlib.sha256(b).hexdigest()
def enc(x):return json.dumps(x,separators=(',',':')).encode()
def pairs(x):
 d=dict(x);assert len(d)==len(x);return d
def read(p):
 b=p.read_bytes();assert len(b)<=131072
 return json.loads(b,object_pairs_hook=pairs),b
P=Path(__file__).resolve().parents[1];D=P/'final-suite';G=P.parent/'windows-bounded-guard';A=P/'source-copy/copy-admission.json'
MAP='b5a175016915cd4f3a3d4ea1d2ac849aeef9297875f4eebf9dc65e913bcf63aa'
REMOTE=r'''
import os,sys,time,signal
start=time.time();end=float(sys.argv[1]);assert start<end<=min(start+60,1792581014)
signal.setitimer(signal.ITIMER_REAL,end-start)
import json,hashlib,zlib,shutil,stat,fcntl
from pathlib import Path
H=lambda b:hashlib.sha256(b).hexdigest()
def enc(x):return json.dumps(x,separators=(',',':')).encode()
R=Path('/home/spenser/file-native-online-linux-20261020');S=R/'source';D=S/'final-suite-v5'
assert os.getuid()==1000 and R.resolve()==R and S.resolve()==S
assert all(p.stat().st_uid==1000 and not p.is_symlink() for p in [R,S])
assert H(Path('/proc/self/exe').read_bytes())=='44c4dbc2292ba0f2f1b6eba5328ceefd41261e962b129c7eb9a670b1640a1acf'
rid=[R.stat().st_dev,R.stat().st_ino,S.stat().st_ino]
pid=os.getpid()
def birth():
 x=Path('/proc/self/stat').read_text();return int(x[x.rfind(')')+2:].split()[19])
born=birth()
def empty():
 assert all(not (p/'children').read_text().strip() for p in Path('/proc/self/task').iterdir())
empty()
raw=sys.stdin.buffer.read(1048577);assert len(raw)<=196608
z=zlib.decompressobj(31);data=z.decompress(raw,1048577)
assert len(data)<=1048576 and z.eof and not z.unconsumed_tail and not z.unused_data
q=json.loads(data);assert set(q)=={'files','reuse'}
C='b5a175016915cd4f3a3d4ea1d2ac849aeef9297875f4eebf9dc65e913bcf63aa'
tab={'files':sorted([r[:3] for r in q['files']]),'reuse':sorted(q['reuse'])}
assert H(enc(tab))==C
assert len(q['files'])==35 and sum(r[1] for r in q['files'])==588031
items=[]
for name,size,sha,text in q['files']:
 b=text.encode();assert len(b)==size and H(b)==sha
 p=Path(name);assert not p.is_absolute() and '..' not in p.parts
 items.append((name,b,sha))
for name,size,sha in q['reuse']:
 p=S/name;assert p.name==name and p.stat().st_size==size and H(p.read_bytes())==sha
lock=(R/'heavy-session.lock').open('rb');fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
def inv(root):
 rows=[]
 for folder,ds,fs in os.walk(root,followlinks=False):
  for name in ds+fs:
   p=Path(folder)/name;s=p.lstat();assert not stat.S_ISLNK(s.st_mode) and s.st_uid==1000
   if name in fs:
    assert stat.S_ISREG(s.st_mode) and s.st_nlink==1
    rows.append([p.relative_to(root).as_posix(),s.st_size,H(p.read_bytes())])
 return sorted(rows)
old=inv(S);assert len(old)==110 and sum(r[1] for r in old)==1292813 and not D.exists()
oh=H(enc(old));done=[];total=0
sfd=os.open(S,os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW)
try:
 os.mkdir('final-suite-v5',0o700,dir_fd=sfd)
 dfd=os.open('final-suite-v5',os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW,dir_fd=sfd)
 try:
  for name,b,sha in items:
   assert time.time()<end and shutil.disk_usage(R).free>=20*1024**3
   assert [r for r in inv(S) if not r[0].startswith('final-suite-v5/')]==old and inv(D)==sorted(done)
   assert 1292813+total+max(0,637964-total)+2500000+196608<=4718592
   cur=os.dup(dfd)
   try:
    parts=Path(name).parts
    for part in parts[:-1]:
     try:os.mkdir(part,0o700,dir_fd=cur);os.fsync(cur)
     except FileExistsError:pass
     nxt=os.open(part,os.O_RDONLY|os.O_DIRECTORY|os.O_NOFOLLOW,dir_fd=cur)
     assert os.fstat(nxt).st_uid==1000
     os.close(cur);cur=nxt
    fd=os.open(parts[-1],os.O_RDWR|os.O_CREAT|os.O_EXCL|os.O_NOFOLLOW,0o600,dir_fd=cur)
    try:
     pos=0
     while pos<len(b):
      assert time.time()<end
      n=os.write(fd,b[pos:pos+65536]);assert n>0;pos+=n
     os.fsync(fd);os.lseek(fd,0,0);check=b''
     while len(check)<=len(b):
      x=os.read(fd,65536)
      if not x:break
      check+=x
     s=os.fstat(fd);named=os.stat(parts[-1],dir_fd=cur,follow_symlinks=False)
     assert H(check)==sha and len(check)==len(b) and s.st_nlink==1 and s.st_uid==1000 and (s.st_dev,s.st_ino)==(named.st_dev,named.st_ino)
    finally:os.close(fd)
    os.fsync(cur)
   finally:os.close(cur)
   total+=len(b);done.append([name,len(b),sha])
  os.fsync(dfd);os.fsync(sfd)
 finally:os.close(dfd)
finally:os.close(sfd)
assert inv(D)==sorted(done) and [r for r in inv(S) if not r[0].startswith('final-suite-v5/')]==old
assert rid==[R.stat().st_dev,R.stat().st_ino,S.stat().st_ino] and birth()==born
empty()
out=enc(dict(kind='FIXED_SOURCE_COPY_COMPLETE',pid=pid,start=born,uid=os.getuid(),root=rid,files=35,bytes=total,before=1292813,after=1292813+total,old_sha256=oh,map_sha256=C,deadline_epoch=end,completed=time.time(),no_children=True))+b'\n'
assert len(out)<=16384 and time.time()<end
sys.stdout.buffer.write(out);sys.stdout.buffer.flush()
assert time.time()<end
'''
def main():
 assert sys.platform=='win32' and len(sys.argv)==2 and len(sys.argv[1])==64
 c,raw=read(A);assert H(raw)==sys.argv[1] and c['go'] is True and c['kind']=='fixed-source-copy-v1'
 assert c['operator_sha256']==H(Path(__file__).read_bytes()) and c['native27_passed']==27
 limits=c['evidence_bounds']['stdio'];assert limits['stdout']['capture_bytes']<=16384 and limits['stderr']['capture_bytes']<=8192
 end=c['deadline_epoch'];assert T0<time.time()<end<=min(T0+60,1792581014)
 m,b=read(D/'suite_manifest.json');assert H(b)=='d7f7f31a860f2e8072aaafed4efee86216bcf90ab58daf846d16839e651d35e8'
 files=[];reuse=[]
 for key,row in m['source_files'].items():
  alias,name=key.split('/',1)
  p=D/name if alias=='suite' else P/name if alias=='native' else P.parent/'final-protocol'/name if alias=='shared' else P.parent/'harness'/name if alias=='harness' else Path('C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook/docs/research')/name
  assert all(not x.is_symlink() and not x.is_junction() for x in [p,*p.parents]) and p.stat().st_nlink==1
  b=p.read_bytes();assert [len(b),H(b)]==row
  if alias=='native':reuse.append([name,*row])
  else:files.append([name if alias=='suite' else key,*row,b.decode('utf-8')])
 for name in ['suite_manifest.json','source_receipt.json']:
  p=D/name;assert not p.is_symlink() and not p.is_junction() and p.stat().st_nlink==1
  b=p.read_bytes();files.append([name,len(b),H(b),b.decode('utf-8')])
 assert H(enc({'files':sorted([x[:3] for x in files]),'reuse':sorted(reuse)}))==MAP
 omit={'peer-compile-v3','targeted-controls-v3-run'}
 local=sum(p.stat().st_size for p in P.rglob('*') if p.is_file() and p.relative_to(P).parts[0] not in omit)
 local+=310284+51200+sum(p.stat().st_size for p in (P.parent/'admission').glob('native-linux*.json'))
 assert local<=2500000
 pkt=gzip.compress(enc({'files':files,'reuse':sorted(reuse)}),mtime=0);assert len(pkt)<=196608
 ssh=Path('C:/Windows/System32/OpenSSH/ssh.exe')
 assert ssh.stat().st_size==1251328 and ssh.stat().st_nlink==2 and H(ssh.read_bytes())=='8e1212d6b10bbab3dfd85077c1a7d0b60373fde702dfa9ea8c57924793bdb317'
 guard=G/'job_guard_bounded.py';assert H(guard.read_bytes())==c['guard_sha256']
 spec=importlib.util.spec_from_file_location('gcopy',guard);g=importlib.util.module_from_spec(spec);spec.loader.exec_module(g)
 fp=G/'linux-source-copy/transport.json'
 argv=[str(ssh),'-T','-oBatchMode=yes','-oStrictHostKeyChecking=yes','-oConnectTimeout=10','spenser-linux-codex','/usr/bin/python3 -I -S -B -c '+shlex.quote(REMOTE)+' '+shlex.quote(str(end))]
 row=g.run(argv,fp,A,max(.001,end-time.time()),stdin=pkt,cwd=str(P))
 assert row['qualified'] and row['exit']==0 and time.time()<end
 out=fp.with_suffix('.stdout').read_bytes();assert len(out)<=16384 and fp.with_suffix('.stderr').stat().st_size<=8192
 v=json.loads(out);assert v['kind']=='FIXED_SOURCE_COPY_COMPLETE' and v['files']==35 and v['bytes']==588031 and v['map_sha256']==MAP and v['no_children'] is True and v['uid']==1000 and v['pid']>0 and v['start']>0 and v['deadline_epoch']==end
 print(json.dumps(dict(kind='SOURCE_COPY_ONLY',receipt_sha256=H(out),native_qualification=0),separators=(',',':')))
if __name__=='__main__':main()
