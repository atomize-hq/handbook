"""Bounded proof regression on a fresh copy; no engine or measured store mutation."""
import argparse,json,shutil
from pathlib import Path
from verify_store import verify_history,sha

a=argparse.ArgumentParser();a.add_argument('--fixture',required=True);a.add_argument('--destination',required=True);ns=a.parse_args()
source=Path(ns.fixture).resolve();destination=Path(ns.destination).resolve()
assert not destination.exists() and source!=destination and source not in destination.parents
shutil.copytree(source,destination)
baseline=verify_history(destination);candidates=list((destination/'attempts').glob('*/candidate.json'))
assert candidates
saved={p:p.read_bytes() for p in candidates}
for p in candidates:p.unlink()
assert verify_history(destination)==baseline
first=json.loads((destination/'expected-ledger.jsonl').open().readline());publication=json.loads((destination/f"publications/{first['selected']['publication']}.json").read_bytes())
one=destination/f"attempts/{publication['attempt']}/candidate.json";one.write_bytes(saved[one]);candidate=json.loads(saved[one]);candidate['request']['packet']=not candidate['request']['packet'];one.write_text(json.dumps(candidate))
try:verify_history(destination)
except AssertionError:pass
else:raise AssertionError('altered present candidate was accepted')
one.unlink()
origin_path=destination/'HISTORY-SOURCE.json';origin_bytes=origin_path.read_bytes();origin=json.loads(origin_bytes)
original=(destination/origin['original']).read_bytes();changed=original[:-2]+b'!'+original[-1:];path=destination/'proof-negative-source';path.write_bytes(changed)
origin.update(original=path.name,digest=sha(changed));origin_path.write_text(json.dumps(origin))
try:verify_history(destination)
except AssertionError as error:assert str(error)=='independent recipe intent mismatch',str(error)
else:raise AssertionError('altered independent request recipe was accepted')
origin_path.write_bytes(origin_bytes);path.unlink()
for p,b in saved.items():p.write_bytes(b)
assert verify_history(destination)==baseline
print(json.dumps(dict(verification=baseline,candidate_files_removed=len(saved),without_candidates=True,altered_candidate_refused=True,altered_recipe_intent_refused=True,restored=True),indent=2))
