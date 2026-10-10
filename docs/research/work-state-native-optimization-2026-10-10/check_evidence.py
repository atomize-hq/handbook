"""Check completion, cohort coverage and deterministic summary of this experiment."""
import collections, hashlib, json, pathlib, subprocess, sys
here=pathlib.Path(sys.argv[1])
runs={m:json.loads((here/f'{m}.run.json').read_text()) for m in ('probes','reads','effects','pending')}
assert all(r['exit']==0 for r in runs.values()), 'incomplete native mode'
assert len({r['exe_sha256'] for r in runs.values()})==1, 'mixed measured binaries'
raw={m:[json.loads(line) for line in (here/f'{m}.jsonl').read_text().splitlines()] for m in runs}
rows={m:[r for r in records if 'wall_ms' in r] for m,records in raw.items()}
assert {m:len(rs) for m,rs in rows.items()}=={'probes':0,'reads':564,'effects':75,'pending':2}
assert any(r.get('event')=='direct_read_departure' for r in raw['probes'])
assert sum(r.get('event')=='cohort_complete' for r in raw['reads'])==2
assert any(r.get('event')=='effects_complete' for r in raw['effects'])
assert sum(r.get('event')=='saturated_revocation_refusal' for r in raw['effects'])==5
assert any(r.get('event')=='pending_complete' for r in raw['pending'])
summary_path=here/'summary.json'; previous=summary_path.read_bytes()
subprocess.run([sys.executable,str(here/'summarize.py'),str(here)],check=True,capture_output=True)
assert summary_path.read_bytes()==previous, 'summary differs from raw observations'
summary=json.loads(previous); assert summary['observations']==641
for r in summary['matched_effects']:
    assert r['n']==6 and sorted(r['entry_history'])==list(range(25,31))
counts=collections.Counter((r['cohort'],r['history_slots_at_entry'],r['variant'],r['instrumented'])
    for r in rows['reads'] if r['operation']=='current_item')
assert len(counts)==84 and set(counts.values())=={4}
source=json.loads((here/'source-provenance.json').read_text())
assert source['patch_sha256']==hashlib.sha256((here/'experiment.patch').read_bytes()).hexdigest()
print(json.dumps({'status':'passed','observations':summary['observations'],
    'groups':len(summary['groups']),'matched_effect_variants':len(summary['matched_effects']),
    'current_read_control_groups':len(counts),'binary_sha256':next(iter(runs.values()))['exe_sha256'],
    'patch_sha256':source['patch_sha256']},indent=2))
