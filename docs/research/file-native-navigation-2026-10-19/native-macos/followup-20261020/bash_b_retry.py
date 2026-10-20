"""Repair only the Bash publication-field parser; preserve failed shell-B evidence."""
import sys,time,datetime,json,os,signal,subprocess,hashlib
from pathlib import Path
import followup as f
cutoff=datetime.datetime.fromisoformat('2026-10-20T02:19:38+00:00').timestamp();assert cutoff-time.time()>5
sys.path.insert(0,str(f.H));from oracle import Tree
root=f.R/'supplement/growing-B-128/disposable/store';out=f.R/'supplement/shell-B-parser-retry';out.mkdir(exist_ok=False)
script=f.SHELL.replace('s/.*"attempt":','s/^  "attempt":');assert script!=f.SHELL
(f.V/'bash-B-parser-retry.sh').write_text(script)
start=time.time();peak=None;checker_peak=None;reason=None;samples=[]
with (f.V/'bash-B-parser-retry.stdout').open('wb') as so,(f.V/'bash-B-parser-retry.stderr').open('wb') as se:
 p=subprocess.Popen(['bash','-c',script,'native-followup-B-parser-retry',str(root),str(out),'B'],stdout=so,stderr=se,start_new_session=True)
 while p.poll() is None:
  table=f.processes();ids=f.descendants(p.pid,table);values=[rss*1024 for pid,ppid,g,rss in table if pid in ids];v=sum(values) if values else None;checker=next((rss*1024 for pid,ppid,g,rss in table if pid==os.getpid()),None)
  if v is not None:peak=max(peak or 0,v)
  if checker is not None:checker_peak=max(checker_peak or 0,checker)
  samples.append(dict(time=time.time(),shell_descendant_rss_bytes=v,checker_rss_bytes=checker))
  if time.time()>=min(cutoff,start+120):reason='cutoff-or-120s'
  if v is not None and v>2*1024**3:reason='shell-rss'
  if checker and checker>2*1024**3:reason='checker-rss'
  if v is not None and checker is not None and v+checker>3*1024**3:reason='aggregate-rss'
  if reason:os.killpg(p.pid,signal.SIGKILL);break
  time.sleep(.02)
 code=p.wait()
closure=[pid for pid,ppid,g,rss in f.processes() if g==p.pid]
receipt=dict(start_epoch=start,end_epoch=time.time(),seconds=time.time()-start,cutoff='2026-10-20T02:19:38Z',exit=code,censor=reason,shell_peak_rss_bytes=peak,checker_peak_rss_bytes=checker_peak,samples=samples,remaining_process_group=closure,script_sha256=hashlib.sha256(script.encode()).hexdigest(),engine_calls_during_extraction=0)
f.save('bash-B-parser-retry-command.json',receipt);assert code==0 and not reason and not closure
selected=json.loads((out/'selected-once.json').read_bytes());pub=json.loads((root/f"publications/{selected['publication']}.json").read_bytes());tree=Path((out/'captured-tree.txt').read_text().strip());assert tree==root/'generations'/pub['attempt'];projection=Tree(root).entries(pub['projection']);path=(out/'chosen-source.txt').read_text().strip();spec=projection[path];raw=(out/'copied-source').read_bytes();assert raw==(root/spec['source']).read_bytes() and hashlib.sha256(raw).hexdigest()==spec['digest']
f.save('bash-B-parser-retry-result.json',dict(status='passed',case='ordinary-bash-discovery-search-exact-copy',mode='B',selected_once=selected,captured_tree=str(tree),source_path=path,sha256=spec['digest'],time=f.now(),engine_calls_during_extraction=0,failed_original_preserved=True))
print(json.dumps(receipt,indent=2))
