"""Preserve the already-started N10k A prefix under the parent's narrowed scope."""
import json,time,subprocess
import supervise as s
rows=json.loads((s.BASE/'matrix-cells.json').read_bytes());assert len(rows)==4
for cell in ['n128-A-p4','n128-B-p4','n1000-A-p4','n1000-B-p4']:assert any(r['cell']==cell and r['summary']['status']=='success' for r in rows)
assert not (s.BASE/'n10000-B-p4-admission.json').exists()
row=dict(status='scope-cancelled-unqualified-prefix',epoch=time.time(),reason='Parent explicitly narrowed native qualification to N128 and small >512-record geometry (N1k); do not launch a native N10k performance matrix. N10k A had already started initialization when steering arrived.',stopped_process_groups=[254757,253254],kill='SIGKILL after stopping the owning matrix controller',n10000_A_summary_present=(s.BASE/'matrix/n10000-A-p4/summary.json').exists(),n10000_B='unperformed; never launched',earlier_four_cells='success; raw receipts preserved',owned_after=s.census(s.BASE),process_snapshot=subprocess.check_output(['ps','-eo','pid,ppid,pgid,rss,args']).decode())
(s.BASE/'matrix-failed.json').write_text(json.dumps(row,indent=2))
print(json.dumps({k:v for k,v in row.items() if k!='process_snapshot'},indent=2))
