"""Fresh bounded proof fixture: missing retired content differs from lost outcomes."""
import argparse,json,shutil
from pathlib import Path
from verify_store import verify_history,Tree

a=argparse.ArgumentParser();a.add_argument('--fixture',required=True);a.add_argument('--destination',required=True);ns=a.parse_args()
source=Path(ns.fixture).resolve();root=Path(ns.destination).resolve()
assert not root.exists() and source!=root and source not in root.parents
shutil.copytree(source,root);baseline=verify_history(root)
first=json.loads((root/'expected-ledger.jsonl').open().readline());selector=first['selected']
pub=json.loads((root/f"publications/{selector['publication']}.json").read_bytes())
all_graphs=[];s=json.loads((root/'SELECTED.json').read_bytes())
while s:
 p=json.loads((root/f"publications/{s['publication']}.json").read_bytes());all_graphs.append(p);s=p['predecessor']
other=set()
for p in all_graphs:
 if p['id']==pub['id']:continue
 t=Tree(root);t.entries(p['graph']['tree']);other.update(t.seen)
t=Tree(root);t.entries(pub['graph']['tree']);unique=t.seen-other;assert unique
marker=root/f"retired/{pub['id']}.json";marker.parent.mkdir(exist_ok=True);marker.write_text(json.dumps(selector))
candidate=root/f"attempts/{pub['attempt']}/candidate.json";candidate.unlink()
removed=next(iter(unique));(root/'objects'/removed[:2]/removed).unlink()
try:verify_history(root)
except FileNotFoundError:pass
else:raise AssertionError('strict transition proof did not require reclaimed graph')
verified=verify_history(root,allow_retired_content=True)
assert verified['retired_publications']==1 and verified['retired_header_transition_checks_unavailable']==[1,2]
assert {k:verified[k] for k in baseline}==baseline
result=root/f"results/{pub['result_hash']}.json";saved=result.read_bytes();result.write_bytes(b'bad')
try:verify_history(root,allow_retired_content=True)
except AssertionError:pass
else:raise AssertionError('retired accepted outcome corruption was ignored')
result.write_bytes(saved);assert verify_history(root,allow_retired_content=True)==verified
print(json.dumps(dict(verification=verified,strict_missing_retired_graph_refused=True,retired_outcome_corruption_refused=True,no_engine_calls=True),indent=2))
