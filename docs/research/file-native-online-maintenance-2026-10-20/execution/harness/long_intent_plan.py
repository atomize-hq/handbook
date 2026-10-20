"""Bounded READ-ONLY long-history intent mapping; never restores/copies a store."""
import hashlib
import json
import os
from pathlib import Path
from job_guard import Budget, sha_file, write_json
from outcome_oracle import intent, normalized, strict

R = Path('C:/file-native-packing-20261020')
ROOT = Path('C:/file-native-recovery-20261019/history-restored/disposable/store')
OUT = Path('C:/file-native-online-matrix-20261020/receipts/long-intent-read-only-plan.json')
PRIOR = Path('C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook/docs/research')
Budget(os.environ['FILE_NATIVE_BUDGET']).check({'receipts': dict(bytes=2*1024**2, files=5)})
assert ROOT.is_dir()
source_manifest = strict((R/'post-gc-operational-backup/manifest.json').read_bytes())
final_manifest = strict((R/'packed-operational-backup/manifest.json').read_bytes())
frozen = strict((R/'long-matrix/frozen.json').read_bytes())
inventory_manifest = strict((R/'long-matrix/L-inventory.manifest.json').read_bytes())
assert final_manifest['logical_bytes'] == 592458971 and final_manifest['files'] == 5807
assert final_manifest['transport_sha256'] == '2fabd1c2439fc95a37a0411c99025bfb5646dee855b6ed0e879fd190e35e324b'
assert frozen['measurement_H'] == 101007 and final_manifest['selected'] == frozen['selected']
assert strict((ROOT/'SELECTED.json').read_bytes()) == source_manifest['selected'] == inventory_manifest['original_selected']
controls = ['STORE.json', 'SELECTED.json', 'HISTORY-SOURCE.json', 'expected-ledger.jsonl']
before = {name: dict(bytes=(ROOT/name).stat().st_size, sha256=sha_file(ROOT/name)) for name in controls}
assert before['expected-ledger.jsonl']['bytes'] == 246560316
inventory_digest = hashlib.sha256()
roles = {}
ledger_row = None
with (R/'post-gc-operational-backup/files.jsonl').open('rb') as source:
    for line in source:
        assert len(line) <= 4096
        inventory_digest.update(line)
        row = strict(line)
        if row['path'] == 'expected-ledger.jsonl':
            ledger_row = row
        parts = row['path'].split('/')
        if len(parts) == 3 and parts[0] == 'attempts' and parts[2] == 'candidate.json':
            roles['candidate'] = roles.get('candidate', 0)+1
        if len(parts) == 4 and parts[0] == 'attempts' and parts[2] == 'allocations':
            roles['allocation'] = roles.get('allocation', 0)+1
assert inventory_digest.hexdigest() == source_manifest['inventory_sha256'] == inventory_manifest['source_inventory_sha256']
assert ledger_row['bytes'] == before['expected-ledger.jsonl']['bytes'] and ledger_row['sha256'] == before['expected-ledger.jsonl']['sha256']

def checked_publication(selector):
    raw = (ROOT/f"publications/{selector['publication']}.json").read_bytes()
    assert len(raw) <= 8*1024**2 and hashlib.sha256(raw).hexdigest() == selector['digest']
    p = strict(raw)
    assert p['id'] == selector['publication']
    return p

known = {}
proofs = []
selector = source_manifest['selected']
tail_publications = 0
reclaimed_nonhistory = []
while selector is not None:
    p = checked_publication(selector)
    if p['sequence'] <= 100000:
        assert p['sequence'] == 100000 and p['request'] == 'history-100000'
        break
    candidate = ROOT/f"attempts/{p['attempt']}/candidate.json"
    if candidate.exists():
        c = strict(candidate.read_bytes())
        request = c['request']
        assert c['attempt'] == p['attempt'] and c['intent'] == p['intent'] and c['graph'] == p['graph'] and c['projection'] == p['projection']
        basis = dict(kind='retained-exact-candidate-request', path=str(candidate), sha256=sha_file(candidate))
    else:
        assert p['request'] == 'reclamation-distinct-source', 'unexpected-reclaimed-nonhistory-request'
        request = dict(request=p['request'], base=p['predecessor'],
                       ops=[dict(kind='import', id='d000000', text='# Unique reclamation source, October 19 2026\r\n')], packet=False)
        script = PRIOR/'file-native-recovery-reclamation-2026-10-19/long_trial.py'
        assert "# Unique reclamation source, October 19 2026\\r\\n" in script.read_text()
        basis = dict(kind='exact-retained-harness-recipe', script=str(script), sha256=sha_file(script),
                     base_binding='pub helper explicitly supplied selected(); accepted predecessor is that exact expected selector')
        reclaimed_nonhistory.append(p['request'])
    assert normalized(request)['request'] == p['request'] and intent(request) == p['intent']
    known[p['request']] = request
    proofs.append(dict(request=p['request'], selected=selector, intent=p['intent'], basis=basis))
    tail_publications += 1
    selector = p['predecessor']
assert tail_publications == 1005 and reclaimed_nonhistory == ['reclamation-distinct-source']
with (ROOT/'expected-ledger.jsonl').open('rb') as ledger:
    first = strict(ledger.readline(4097))
    first_hundred = [first]+[strict(ledger.readline(4097)) for _ in range(99)]
assert [r['index'] for r in first_hundred] == list(range(1, 101))
first_publication = checked_publication(first['selected'])
genesis_selector = first_publication['predecessor']
genesis = checked_publication(genesis_selector)
initial_request = dict(request='initial', base=None, ops=[], packet=False)
assert genesis['sequence'] == 0 and genesis['predecessor'] is None and intent(initial_request) == genesis['intent']
known['initial'] = initial_request
retired_history = []
for row in first_hundred:
    p = checked_publication(row['selected'])
    assert strict((ROOT/f"retired/{p['id']}.json").read_bytes()) == row['selected']
    assert not (ROOT/f"attempts/{p['attempt']}/candidate.json").exists()
    retired_history.append(p['request'])
origin = strict((ROOT/'HISTORY-SOURCE.json').read_bytes())
original = (ROOT/origin['original']).read_bytes()
assert hashlib.sha256(original).hexdigest() == origin['digest'] and len(original) == origin['bytes'] == 4096
initial_oracle = strict((ROOT/'INITIAL-ORACLE.json').read_bytes())
assert len(initial_oracle['docs']) == 16
prep_events = strict((R/'long-matrix/prep-events/events.json').read_bytes())
navigation_event = next(e for e in prep_events if e['input']['action'] == 'publish' and e['input']['request']['request'] == 'P-navigation-upgrade' and e['exit'] == 0)
nav_request = navigation_event['input']['request']
assert nav_request['base'] == source_manifest['selected'] and nav_request['ops'] == [] and nav_request['packet'] is False
known['P-navigation-upgrade'] = nav_request
assert len(known) == 1007
assert any(row['role'] == 'candidate' for row in inventory_manifest['added_records'])
after = {name: dict(bytes=(ROOT/name).stat().st_size, sha256=sha_file(ROOT/name)) for name in controls}
assert before == after
write_json(OUT, dict(status='bounded-read-only-intent-map-prepared',
    target_actual_H=101007, source_actual_H=101006, source=str(ROOT),
    final_archive=str(R/'packed-operational-backup'), final_archive_logical_bytes=592458971,
    original_test_ledger_bytes=246560316, ledger_fully_included_in_later_restore=True,
    protected_controls_before=before, protected_controls_after=after,
    source_inventory_sha256=inventory_digest.hexdigest(), source_attempt_record_counts=roles,
    original_documents=16, history_recipe_publications=100000,
    history_recipe_compatibility='same API history i%5 recipe over first16IDs; no hardcoded N16 graph-count assertion in new outcome oracle',
    retired_history_requests=retired_history, reclaimed_nonhistory=reclaimed_nonhistory,
    known_nonhistory_requests=known, validated_source_nonhistory_proofs=proofs,
    navigation_input_source=dict(path=str(R/'long-matrix/prep-events/events.json'), sha256=sha_file(R/'long-matrix/prep-events/events.json')),
    navigation_intent_binding='exact input prepared; final selected publication/candidate intent is rechecked when admitted final representation is opened',
    no_engine_calls=True, no_restore=True, no_corpus_copy=True, no_long_mutation_lane_taken=True,
    qualification='mapping/read-only proof, not complete H101007 candidate qualification; final oracle must verify every accepted result/intent/ledger row on the chosen representation'))
print(json.dumps(dict(status='read-only-map-prepared', known_nonhistory_inputs=len(known),
                      source_tail_intents_verified=tail_publications, retired_history_intents_reconstructible=100,
                      reclaimed_nonhistory=reclaimed_nonhistory)))
