"""Independent immutable Work2 / metadata-only STORE CAS closure oracle.

Reuses the separately implemented CP1 physical byte decoder, not engine code.
Full qualification is blocked until SCHEMA_PIN names the replacement commit.
Work1 remains a historical oracle in physical_decoder.py; it is refused here.
"""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import re
import sys
import time
import physical_decoder as prior

SCHEMA_PIN = 'f6f907943c54936e49123a136a5f0f87993758ab'
INSPECTED_PARENT = '62ce963085ac054bcc34eb21c6733b94db160975'


def work2(value):
    assert type(value) is dict and type(value.get('version')) is int and value['version'] == 2, 'immutable-work2-required'
    prior.shape(value, prior.WORK|{'transition_parent', 'birth_claims'})
    parent = value['transition_parent']
    assert parent is None or type(parent) is str and prior.HASH.fullmatch(parent)
    claims = value['birth_claims']
    assert type(claims) is list and len(claims) <= 64
    assert all(type(claim) is str and prior.ID.fullmatch(claim) for claim in claims)
    assert len(set(claims)) == len(claims)
    # Only the independently checked version tag changes; the bounded closed
    # data model is unchanged. No source document is converted or rewritten.
    prior.work({**{k: v for k, v in value.items() if k not in ['transition_parent', 'birth_claims']}, 'version': 1, 'garbage': []})
    assert type(value['garbage']) is list and len(value['garbage']) <= 4096
    for name in value['garbage']:
        metadata2(name)


def metadata2(name):
    if type(name) is str and name.startswith('physical-progress/'):
        parts = name.split('/')
        assert len(parts) == 2
        assert any(parts[1].endswith(suffix) and prior.ID.fullmatch(parts[1][:-len(suffix)])
                   for suffix in ['.json', '.work-stage', '.selector-stage'])
    else:
        prior.metadata_path(name)


def config3(c):
    allowed = {'version', 'store_id', 'mode', 'consumer', 'fallback', 'navigation_version', 'catalog', 'physical'}
    assert type(c) is dict and set(c) <= allowed
    assert {'version', 'store_id', 'mode', 'consumer', 'fallback', 'catalog', 'physical'} <= set(c)
    assert type(c['version']) is int and c['version'] == 3 and type(c['store_id']) is str
    assert c['mode'] in ['A', 'B'] and c['consumer'] in ['project', 'research'] and type(c['fallback']) is bool
    if 'navigation_version' in c:
        assert type(c['navigation_version']) is int and c['navigation_version'] in [1, 2, 3, 4]
    prior.shape(c['physical'], {'id', 'ordinal', 'plan'})
    b = c['physical']
    assert type(b['id']) is str and prior.ID.fullmatch(b['id']) and prior.unsigned(b['ordinal']) > 0
    assert type(b['plan']) is str and prior.HASH.fullmatch(b['plan'])
    assert type(c['catalog']) is str and prior.HASH.fullmatch(c['catalog'])


class Decoder(prior.Decoder):
    def __init__(self, root, *, quiescent=False):
        self.candidate_context = False
        super().__init__(root, quiescent=quiescent)
        if self.config['version'] != 3:
            self.required = {'STORE.json'}
            self.physical_metadata = dict(legacy=True, schema_pin=SCHEMA_PIN,
                                          journal=self.journal(), required_paths=sorted(self.required))

    @classmethod
    def candidate(cls, root, c):
        config3(c)
        reader = cls.__new__(cls)
        reader.candidate_context = True
        reader.root = Path(root)
        reader.config = c
        reader.entries = {}
        reader.descriptors = []
        reader.zstd = None
        reader.required = set()
        reader.pin(c['physical'], c['catalog'])
        cat = reader.catalog(c['catalog'])
        reader.descriptors = cat['packs']
        for d in reader.descriptors:
            reader.load_index(d)
        reader.physical_metadata = reader.selected_work(cat)
        return reader

    def selected_work(self, catalog):
        c = self.config
        b = c['physical']
        mutable = self.root/('physical-progress/'+b['id']+'.json')
        assert not mutable.exists(), 'unexpected-mutable-progress'
        w = self.read_metadata('physical-plans/'+b['plan']+'.json', 4*1024**2, b['plan'])
        work2(w)
        g = w['generation']
        assert w['store_id'] == c['store_id'] and (g['id'], g['ordinal'], g['catalog']) == (b['id'], b['ordinal'], c['catalog'])
        assert g['packs'] == catalog['packs']
        if self.candidate_context:
            assert w['transition_parent'] is None, 'physical-build-cannot-claim-metadata-only-parent'
        for old in w['older']:
            self.pin(old, old['catalog'])
            retained = self.catalog(old['catalog'])
            assert retained['packs'] == old['packs']
            old_root = self.read_metadata('physical-plans/'+old['plan']+'.json', 4*1024**2, old['plan'])
            work2(old_root)
            assert old_root['store_id'] == c['store_id']
            assert (old_root['generation']['id'], old_root['generation']['ordinal'], old_root['generation']['catalog']) == (old['id'], old['ordinal'], old['catalog'])
        for job in w['jobs']:
            for ext, size in [('pack', job['descriptor']['pack_bytes']), ('idx', job['descriptor']['index_bytes'])]:
                name = prior.container_path(job['descriptor'], ext)
                self.required.add(name)
                assert (self.root/name).stat().st_size == size
        stage = self.selector_stage(w) if not self.candidate_context else None
        journal = self.journal() if not self.candidate_context else None
        work_stage = 'physical-progress/'+b['id']+'.work-stage'
        owned_work_stage = None
        if (self.root/work_stage).exists():
            self.required.add(work_stage)
            data = prior.legacy.bounded(self.root/work_stage, 4*1024**2)
            owned_work_stage = dict(path=work_stage, bytes=len(data), sha256=prior.digest(data),
                                    accepted_as_immutable_plan=False)
        def present(name):
            path = self.root/name
            return path.exists() or name.startswith('physical-stages/') and path.parent.exists()
        garbage_present = [name for name in w['garbage'] if present(name)]
        return dict(schema_pin=SCHEMA_PIN, work_version=2, selected_generation=b,
                    transition_parent=w['transition_parent'], transition_parent_is_comparison_token_not_retained_chain=True,
                    birth_claims=w['birth_claims'], birth_claim_count=len(w['birth_claims']),
                    retained_generations=len(w['older']), pending_loose_jobs=len(w['jobs']),
                    retired_container_jobs=len(w['retired']), garbage_entries=len(w['garbage']),
                    pending_garbage=len(garbage_present), absent_garbage_tombstones=len(w['garbage'])-len(garbage_present),
                    observed_removed_bytes=w['removed_bytes'], observed_removed_files=w['removed_files'],
                    observed_reconciled_absent=w['reconciled_absent'], counters_are_not_admission_credit=True,
                    metadata_work_root_selection_is_not_physical_activation=True,
                    mutable_progress_accepted=False, pending_selector_stage=stage, journal=journal,
                    owned_work_stage=owned_work_stage,
                    required_paths=sorted(self.required), quiescent_native_lock_state_unmeasured=True)

    def selector_stage(self, w):
        c = self.config
        b = c['physical']
        name = 'physical-progress/'+b['id']+'.selector-stage'
        path = self.root/name
        if not path.exists():
            return None
        self.required.add(name)
        raw = prior.legacy.bounded(path, 4096)
        try:
            target = prior.legacy.strict(raw)
            config3(target)
        except (ValueError, AssertionError, KeyError, TypeError) as error:
            raise AssertionError('unidentified-selector-stage-ownership-must-refuse-and-preserve') from error
        fixed = copy.deepcopy(target)
        next_plan = fixed['physical']['plan']
        fixed['physical']['plan'] = b['plan']
        assert fixed == c, 'foreign-work-selector-stage'
        result = dict(path=name, plan=next_plan, state='owned-candidate-selector', establishes_selection=False)
        if next_plan == b['plan']:
            result['state'] = 'already-selected-stale-stage'
        elif any(old['plan'] == next_plan for old in w['older']):
            result['state'] = 'protected-retained-plan-reference'
        else:
            plan_name = 'physical-plans/'+next_plan+'.json'
            if (self.root/plan_name).exists():
                candidate = self.read_metadata(plan_name, 4*1024**2, next_plan)
                work2(candidate)
                assert candidate['store_id'] == c['store_id']
                assert candidate['generation'] == w['generation']
                assert candidate['transition_parent'] == b['plan'], 'candidate-stage-not-owned-by-exact-selected-root'
                assert set(w['birth_claims']) <= set(candidate['birth_claims']), 'same-generation-birth-claim-loss'
                result['candidate_plan_present_and_authenticated'] = True
            else:
                result['state'] = 'selector-written-before-candidate-plan'
        return result

    def journal(self):
        path = self.root/'PACKING.json'
        if not path.exists():
            return None
        j = self.read_metadata('PACKING.json', 65536)
        assert type(j) is dict and type(j.get('version')) is int
        if j['version'] != 2:
            return dict(version=j['version'], legacy_journal=True, establishes_selection=False,
                        qualification='legacy journal is not a Work2 ownership/activation proof')
        required = {'version', 'id', 'status', 'expected_config', 'catalog', 'old',
                    'reserved_bytes', 'used_bytes', 'deleted_bytes', 'deferred_bytes'}
        assert required <= set(j) <= required|{'target', 'abandoned', 'ownership_transferred'}
        assert type(j['id']) is str and prior.ID.fullmatch(j['id'])
        assert j['status'] in ['building', 'installed', 'registered', 'complete', 'aborted-owned']
        assert type(j['expected_config']) is str and prior.HASH.fullmatch(j['expected_config'])
        assert j['catalog'] is None or type(j['catalog']) is str and prior.HASH.fullmatch(j['catalog'])
        assert type(j['old']) is list and len(j['old']) <= 64
        for d in j['old']:
            prior.descriptor(d)
        for field in ['reserved_bytes', 'used_bytes', 'deleted_bytes', 'deferred_bytes']:
            prior.unsigned(j[field])
        transferred = j.get('ownership_transferred', False)
        assert type(transferred) is bool
        owners = j.get('abandoned', [])
        assert type(owners) is list and len(owners) <= 64
        seen = set()
        for owner in owners:
            prior.shape(owner, {'id', 'catalog', 'plan'})
            assert type(owner['id']) is str and prior.ID.fullmatch(owner['id']) and owner['id'] not in seen
            seen.add(owner['id'])
            assert all(owner[k] is None or type(owner[k]) is str and prior.HASH.fullmatch(owner[k]) for k in ['catalog', 'plan'])
        current_id = self.config.get('physical', {}).get('id')
        selected_owner = current_id == j['id']
        target = j.get('target')
        if target is not None:
            config3(target)
            assert target['store_id'] == self.config['store_id'] and target['physical']['id'] == j['id']
            assert target['catalog'] == j['catalog']
            if not selected_owner and not transferred:
                assert all(target[field] == self.config[field] for field in ['store_id', 'mode', 'consumer', 'fallback']), 'unselected-target-changes-frozen-semantic-config'
                assert target.get('navigation_version', 1) == self.config.get('navigation_version', 1), 'unselected-target-changes-navigation-config'
        refs = list(owners)
        if not transferred and not selected_owner:
            refs.append(dict(id=j['id'], catalog=j['catalog'], plan=target['physical']['plan'] if target else None))
        ownership = []
        for owner in refs:
            row = dict(owner)
            if not transferred and not selected_owner:
                directory = self.root/('packs/'+owner['id'])
                files = list(directory.iterdir()) if directory.exists() else []
                assert len(files) <= 128
                for item in files:
                    assert item.is_file() and not item.is_symlink()
                    prior.metadata_path(item.relative_to(self.root).as_posix())
                    self.required.add(item.relative_to(self.root).as_posix())
                row['owned_container_files'] = len(files)
                if owner['plan']:
                    name = 'physical-plans/'+owner['plan']+'.json'
                    if (self.root/name).exists():
                        plan = self.read_metadata(name, 4*1024**2, owner['plan'])
                        work2(plan)
                        assert plan['store_id'] == self.config['store_id'] and plan['generation']['id'] == owner['id']
                        assert plan['generation']['catalog'] == owner['catalog']
                        row['authenticated_flattened_garbage_paths'] = len(plan['garbage'])
                for name, limit in [('physical-pins/'+owner['id']+'.lock', 1024),
                                    ('physical-progress/'+owner['id']+'.work-stage', 4*1024**2),
                                    ('physical-progress/'+owner['id']+'.selector-stage', 4096),
                                    ('physical-stages/'+owner['id']+'/STORE.json', 4096)]:
                    if (self.root/name).exists():
                        self.required.add(name)
                        prior.legacy.bounded(self.root/name, limit)
            ownership.append(row)
        result = dict(version=2, id=j['id'], status=j['status'], abandoned_owner_count=len(owners),
                      ownership_transferred=transferred, selected_by_STORE=selected_owner,
                      pending_owned_refs=ownership, establishes_acceptance=False)
        if not selected_owner and not transferred and j['status'] == 'installed':
            assert target is not None
            candidate = Decoder.candidate(self.root, target)
            candidate_rows = candidate.verify_packs()
            self.required.update(candidate.required)
            result['complete_unselected_candidate_records'] = len(candidate_rows)
            result['candidate_is_not_semantic_acceptance'] = True
        return result


def verify(root, output, inventory=None, baseline=None, *, quiescent=False, known_requests=None):
    assert re.fullmatch(r'[0-9a-f]{40}', SCHEMA_PIN), 'replacement-source-pin-not-yet-bound'
    assert quiescent, 'explicit-quiescent-verifier-contract-required'
    started = time.monotonic()
    d = Decoder(root, quiescent=True)
    rows = d.verify_packs()
    if inventory:
        assert rows == prior.legacy.expected_inventory(inventory), 'complete-raw-inventory-mismatch'
    sys.path.insert(0, str(prior.PRIOR/'file-native-recovery-reclamation-2026-10-19'))
    import verify_operational
    verify_operational.checked = d.checked
    semantic = verify_operational.verify(root, baseline)
    sys.path.insert(0, str(prior.PRIOR/'file-native-navigation-2026-10-19'))
    import oracle
    oracle.checked = d.checked
    navigation = oracle.verify(root)
    from outcome_oracle import verify as intents
    request_intents = intents(root, decoder=d, known_requests=known_requests)
    inventory_hash = hashlib.sha256()
    for row in rows:
        inventory_hash.update((json.dumps(row, separators=(',', ':'))+'\n').encode())
    result = dict(schema_pin=SCHEMA_PIN, raw_records=len(rows), raw_bytes=sum(r['raw_length'] for r in rows),
                  inventory_digest=inventory_hash.hexdigest(), physical_metadata=d.physical_metadata,
                  semantic=semantic, navigation=navigation, request_intents=request_intents,
                  independent_codec=d.zstd.binding if d.zstd else None, seconds=time.monotonic()-started,
                  qualification='quiescent full selected representation and bounded pending ownership; immutable Work2 only; native liveness/crash schedules separately required')
    Path(output).write_text(json.dumps(result, indent=2)+'\n', encoding='utf-8')
    return result


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--root', required=True)
    parser.add_argument('--output', required=True)
    parser.add_argument('--inventory')
    parser.add_argument('--baseline')
    parser.add_argument('--known-requests')
    parser.add_argument('--quiescent', action='store_true')
    args = parser.parse_args()
    known = json.loads(Path(args.known_requests).read_bytes()) if args.known_requests else None
    answer = verify(args.root, args.output, args.inventory, args.baseline,
                    quiescent=args.quiescent, known_requests=known)
    print(json.dumps(dict(status='passed', actual_H=answer['semantic']['publications'],
                          records=answer['raw_records'], schema_pin=SCHEMA_PIN)))
