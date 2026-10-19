"""Bound the cancellation return to task evidence; preserve the original locally."""
import hashlib,json
import supervise as s
path=s.BASE/'matrix-failed.json';raw=path.read_bytes();row=json.loads(raw)
snapshot=row.pop('process_snapshot');row['task_process_snapshot']=[line for line in snapshot.splitlines() if str(s.BASE) in line]
row['original_native_receipt_sha256']=hashlib.sha256(raw).hexdigest()
row['original_native_receipt_preserved_at']=str(path)
row['unrelated_host_process_arguments']='omitted from bounded return; original remains at native path'
partial=s.BASE/'matrix/n10000-A-p4/cell/disposable/store'
row['cancelled_N10k_A_prefix_census']=s.census(partial) if partial.exists() else None
row['cancelled_N10k_A_selected_present']=(partial/'SELECTED.json').exists()
(s.BASE/'scope-cancel-report.json').write_text(json.dumps(row,indent=2))
