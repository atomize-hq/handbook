"""CP1/CP2 source-mapped extensions; each caller supplies an admitted root.

Fresh roots below are shallow A/N10 only. Deep B is created exactly once by
Harness.small. Existing shallow donor roots serve native/legacy/CP3 extensions.
Nothing in this module launches engines when imported.
"""
from focused_protocol import MIB, Unsupported, require, sha, strict, transition


def error(response, expected):
    require(response.get('status') == 'error' and response.get('error') == expected,
            'wrong-refusal:'+repr(response))


def owned_decline(journal):
    """Journal.target is optional and serialized NONE is omitted at CP2."""
    require(type(journal) is dict and journal.get('status') == 'aborted-owned'
            and journal.get('target') is None and isinstance(journal.get('id'), str)
            and bool(journal['id']), 'postconstruction-decline-needs-durable-aborted-owner')
    return journal['id']


def discovery(h, root):
    value = h.state(root)['work'].get('discovery')
    require(type(value) is dict, 'rolling-discovery-not-supported-by-pin')
    return value


def lost_reply(h, root, name, phase='selection-boundary'):
    request = dict(request=name, base=h.selected(root), ops=[], packet=False)
    h.ledgers[root].record(request)
    h.call(root, 'publish', request=request, code=86, env={'EXP_EXIT': phase})
    selected = h.selected(root)
    result = h.call(root, 'resolve', request_id=name)['result']
    h.ledgers[root].accept(request, result, selected)
    return selected, result


def partial_interval(h, root, forecast):
    """cp2-schedules.py partial/cancel/crash/beyond-cutoff, no cursor reset."""
    with h.case('D-partial-fixed-cutoff', root, forecast):
        h.initialize(root, 'A', 10)
        for n in range(5):
            h.publish(root, f'D-interval-{n}')
        h.pack(root, bootstrap=True, discovery_limit=2)
        initial = discovery(h, root)
        require(initial['interval'] is not None and initial['completed'] is None, 'partial-bootstrap-interval')
        target = initial['interval']['target']
        h.publish(root, 'D-beyond-target')
        control = h.a.control(root, 'D-cancel')
        h.a.signal(control, 'cancel')
        selected_store = h.read(root, 'STORE.json', 65536)
        try:
            error(h.pack(root, discovery_limit=2, code=2,
                         env={'EXP_PACK_CANCEL': control['cancel']}), 'cancelled:rolling-discovery')
            require(h.read(root, 'STORE.json', 65536) == selected_store, 'cancel-advanced-physical-selection')
            require(discovery(h, root) == initial, 'cancel-advanced-cursor')
        finally:
            h.a.end_signal(control, 'cancel')  # retain signal bytes/evidence; never edit engine-owned state
        h.pack(root, discovery_limit=2, code=86, env={'EXP_EXIT': 'physical-before-selection'})
        require(discovery(h, root) == initial, 'unselected-crash-advanced-cursor')
        h.verify(root, 'D-before-selection-crash')
        h.call(root, 'packing', operation='resume', cleanup=False)
        for _ in range(8):
            if discovery(h, root)['interval'] is None:
                break
            h.pack(root, discovery_limit=2)
        complete = discovery(h, root)
        require(complete['interval'] is None and complete['completed'] == target,
                'fixed-target-chased-new-head-or-left-hole')
        packed = h.pack(root)
        require(packed['metrics']['rolling_publications_visited'] == 1, 'new-tail-must-remain-outside-partial-target')
        require(discovery(h, root)['completed']['selected'] == h.selected(root), 'new-tail-not-discovered-later')
        h.drain_two_passes(root)
        h.check_outcomes(root)
        h.verify(root, 'D-partial-final')


def reservation_late(h, root, forecast):
    """Genuine selected transfer reservation; no accepted-byte mutation."""
    with h.case('D-reservation-late-eligible', root, forecast):
        h.initialize(root, 'A', 10)
        h.pack(root, bootstrap=True)
        h.cleanup(root, max_batches=160)  # initial bootstrap
        selected, result = lost_reply(h, root, 'D-reserved-transfer')
        h.pack(root)
        holes = discovery(h, root)['deferred']
        require(len(holes) == 1 and holes[0]['roles'] == 24 and holes[0]['reason'] == 'reservation',
                'candidate-allocation-reservation-hole-required')
        require(h.document(root, 'RESERVATIONS.json'), 'genuine-durable-reservation-required')
        h.verify(root, 'D-selected-reservation-lost-reply')
        h.publish(root, 'D-release-transfer')
        packed = h.pack(root)
        require(packed['metrics']['rolling_deferred_visited'] == 1 and not discovery(h, root)['deferred'],
                'late-hole-not-retried-and-covered')
        require(h.call(root, 'resolve', request_id='D-reserved-transfer')['result'] == result,
                'lost-reply-result-changed')
        h.drain_two_passes(root)
        h.verify(root, 'D-reservation-final')


def cutoff_and_stale_cas(h, root, forecast):
    """Selected semantic cutoff independent of full physical binding token."""
    with h.case('D-cutoff-Q-stale-CAS', root, forecast):
        h.initialize(root, 'A', 10)
        h.pack(root, bootstrap=True)
        h.cleanup(root, max_batches=160)  # initial bootstrap
        h.publish(root, 'D-captured-tail')
        h.pack(root, code=86, env={'EXP_EXIT': 'physical-before-selection'})
        cutoff = h.selected(root)
        h.publish(root, 'D-after-cutoff')
        h.call(root, 'packing', operation='resume', cleanup=False)
        require(discovery(h, root)['completed']['selected'] == cutoff, 'semantic-advance-invalidated-physical-CAS')
        h.verify(root, 'D-resume-after-semantic-advance')
        # Leave real cleanup work before a later candidate captures the token.
        h.publish(root, 'D-stale-tail')
        h.pack(root, code=86, env={'EXP_EXIT': 'physical-before-selection'})
        parent = h.read(root, 'STORE.json', 65536)
        h.cleanup(root)
        require(h.read(root, 'STORE.json', 65536) != parent, 'work-metadata-CAS-not-exercised')
        before = h.selected(root)
        error(h.call(root, 'packing', operation='resume', cleanup=False, code=2), 'conflict:physical-catalog')
        require(h.selected(root) == before, 'stale-CAS-changed-semantic-authority')
        h.verify(root, 'D-stale-physical-refusal')
        h.call(root, 'packing', operation='abort')
        h.pack(root)
        h.drain_two_passes(root)
        old = h.read(root, 'STORE.json', 65536)
        h.publish(root, 'Q-reservation-tail')
        error(h.pack(root, reserve_bytes=1, code=2), 'refused:maintenance-reservation-bound')
        require(h.read(root, 'STORE.json', 65536) == old, 'quota-refusal-selected-candidate')
        h.pack(root)
        h.drain_two_passes(root)
        h.check_outcomes(root)
        h.verify(root, 'D-cutoff-stale-quota-final')


def terminal_retirement(h, root, forecast, *, before=130, after=132):
    """F1 regression: >128 real retired publications never fill deferred queue."""
    require(before >= 129 and after >= 130, 'F1-real-retired-history-size')
    with h.case('F1-terminal-retired-bootstrap-delta', root, forecast):
        h.initialize(root, 'A', 10)
        selections = [h.selected(root)]
        for n in range(before):
            selections.append(h.publish(root, f'F1-bootstrap-{n}')['result']['selected'])
        for selected in selections[:-1]:
            h.call(root, 'lifecycle', operation='retire', selected=selected)
        result = h.pack(root, bootstrap=True)
        require(result['metrics']['rolling_publications_visited'] == before+1
                and not discovery(h, root)['deferred'], 'terminal-retired-filled-bootstrap-queue')
        h.cleanup(root, max_batches=160)  # retired-history bootstrap, not small-tail drainage
        error(h.call(root, 'point', selected=selections[0], id='d000000', code=2), 'retired:snapshot')
        h.verify(root, 'F1-retired-bootstrap')
        deltas = []
        for n in range(after):
            deltas.append(h.publish(root, f'F1-delta-{n}')['result']['selected'])
        h.pack(root, discovery_limit=2)
        require(discovery(h, root)['interval'] is not None, 'F1-partial-delta-not-exercised')
        for selected in selections[-1:]+deltas[:-1]:
            h.call(root, 'lifecycle', operation='retire', selected=selected)
        result = h.pack(root)
        require(result['metrics']['rolling_publications_visited'] == after-2
                and discovery(h, root)['interval'] is None and not discovery(h, root)['deferred'],
                'terminal-retired-filled-delta-queue')
        h.drain_two_passes(root)
        error(h.call(root, 'point', selected=deltas[-2], id='d000000', code=2), 'retired:snapshot')
        h.call(root, 'lifecycle', operation='collect')
        h.check_outcomes(root)
        h.verify(root, 'F1-retired-delta-final')


def automatic_policy(h, root, forecast, *, history_ceiling=551):
    """F2: decline then later eligible automatic success, no manual repair/force."""
    require(type(history_ceiling) is int and 151 < history_ceiling <= 551,
            'bounded-source-calibrated-auto-history-ceiling')
    require(forecast.get('automatic_history_ceiling') == history_ceiling,
            'automatic-H551-growth-must-be-forecast-before-init')
    require(forecast.get('automatic_cleanup_max_calls') == 160,
            'bootstrap-auto-cleanup-command-bound-required-before-init')
    with h.case('F2-auto-decline-later-admit', root, forecast):
        h.initialize(root, 'A', 10)
        initial_store = h.read(root, 'STORE.json', 65536)
        initial_selected = h.read(root, 'SELECTED.json', 65536)
        before = h.a.inventory(root)
        error(h.pack(root, bootstrap=True, force=False, code=2), 'refused:automatic-compaction-benefit')
        require(h.read(root, 'STORE.json', 65536) == initial_store
                and h.read(root, 'SELECTED.json', 65536) == initial_selected,
                'early-auto-decline-changed-selection')
        require(not h.a.exists(root, 'PACKING.json'), 'early-auto-decline-created-owner')
        h.note('automatic-preflight-decline-inventories', before=before, after=h.a.inventory(root))
        for n in range(30):
            h.publish(root, f'F2-small-{n}')
        semantic = h.read(root, 'SELECTED.json', 65536)
        error(h.pack(root, bootstrap=True, force=False, code=2), 'refused:automatic-compaction-benefit')
        journal = h.document(root, 'PACKING.json')
        owner = owned_decline(journal)
        require(h.read(root, 'STORE.json', 65536) == initial_store
                and h.read(root, 'SELECTED.json', 65536) == semantic, 'owned-auto-decline-changed-selection')
        owners = [owner]
        h.note('automatic-owned-decline', journal=journal, inventory=h.a.inventory(root))
        h.verify(root, 'F2-owned-decline')
        # Retain the reviewed genuine H151 decline rather than repeating its
        # mistaken success expectation. Final policy may admit earlier; either
        # outcome stays raw. No packing abort/force/manual state edit is used.
        for n in range(120):
            h.publish(root, f'F2-eligible-{n}')
        require(len(h.ledgers[root].accepted) == 151, 'actual-H151-auto-checkpoint')
        selected151 = h.read(root, 'SELECTED.json', 65536)
        admitted = h.pack(root, bootstrap=True, force=False,
                          allowed_errors=('refused:automatic-compaction-benefit',))
        if admitted.get('status') == 'error':
            require(h.read(root, 'STORE.json', 65536) == initial_store
                    and h.read(root, 'SELECTED.json', 65536) == selected151,
                    'H151-auto-decline-changed-selection')
            later = h.document(root, 'PACKING.json')
            owners.append(owned_decline(later))
            h.note('automatic-H151-decline-retained', journal=later, ceiling=history_ceiling)
            h.verify(root, 'F2-H151-decline')
            for n in range(history_ceiling-151):
                h.publish(root, f'F2-later-eligible-{n}')
            require(len(h.ledgers[root].accepted) == history_ceiling, 'actual-auto-history-ceiling')
            ceiling_selected = h.read(root, 'SELECTED.json', 65536)
            admitted = h.pack(root, bootstrap=True, force=False,
                              allowed_errors=('refused:automatic-compaction-benefit',))
            if admitted.get('status') == 'error':
                require(h.read(root, 'STORE.json', 65536) == initial_store
                        and h.read(root, 'SELECTED.json', 65536) == ceiling_selected,
                        'auto-ceiling-decline-changed-selection')
                owned_decline(h.document(root, 'PACKING.json'))
                h.note('automatic-ceiling-decline-unqualified', actual_H=history_ceiling,
                       response=admitted, journal=h.document(root, 'PACKING.json'))
                raise RuntimeError('automatic-policy-still-declined-at-frozen-history-ceiling')
        require(admitted['result']['status'] == 'registered'
                and admitted['result']['forced_correctness'] is False, 'later-auto-not-admitted-normally')
        # This is initial bootstrap/eventual owner reclamation, not the final
        # small-tail TWO-call target. Reviewed CP2 H551 needed45 real batches.
        h.cleanup(root, max_batches=160)
        require(all(not h.a.exists(root, 'packs/'+item) for item in owners),
                'automatic-declined-owners-not-actually-reaped')
        h.check_outcomes(root)
        h.verify(root, 'F2-auto-success-final')


def stable_base(h, root, forecast):
    """Reduced segment pressure must consolidate deltas before stable base."""
    with h.case('D-stable-base-segment-pressure', root, forecast):
        h.initialize(root, 'A', 10)
        h.pack(root, bootstrap=True)
        h.cleanup(root, max_batches=160)  # initial bootstrap
        first = h.state(root)
        base = {name: h.a.file_sha256(root, name) for name in h.containers(first)}
        for cycle in range(3):
            for n in range(16):
                h.publish(root, f'D-stable-{cycle}-{n}')
            packed = h.pack(root, segment_limit=3)
            h.drain_two_passes(root)
            current = h.containers(h.state(root))
            require(set(base) <= set(current), 'stable-base-rewritten-instead-of-deltas')
            require(all(h.a.file_sha256(root, name) == digest for name, digest in base.items()),
                    'stable-base-byte-identity-changed')
            require(packed['metrics']['rolling_collateral_raw_bytes'] <= 64*MIB, 'collateral-rewrite-bound')
            h.verify(root, f'D-stable-cycle-{cycle+1}')
        h.check_outcomes(root)


def persistent_owners(h, root, forecast):
    """Extend an admitted shallow A/B donor; no second deep B init/restore."""
    with h.case('S-persistent-owners', root, forecast):
        require(root in h.ledgers and h.ledgers[root].accepted, 'adopt-donor-exact-inputs-first')
        saved = h.selected(root)
        h.call(root, 'lifecycle', operation='pin', name='focused-saved', selected=saved)
        packet = h.call(root, 'packet', selected=saved)['result']
        candidate_request = dict(request='S-prepared-owner', base=saved, ops=[], packet=True)
        h.ledgers[root].record(candidate_request)
        candidate = h.call(root, 'prepare', request=candidate_request)['result']['candidate']
        candidate_raw = h.read(root, candidate, 4*MIB)
        manifest = packet['path']+'/MANIFEST.json'
        manifest_hash = sha(h.read(root, manifest, 4*MIB))
        h.publish(root, 'S-owner-later')
        h.pack(root)
        h.drain_two_passes(root)
        require(h.read(root, candidate, 4*MIB) == candidate_raw
                and sha(h.read(root, manifest, 4*MIB)) == manifest_hash, 'persistent-owner-content-lost')
        error(h.call(root, 'lifecycle', operation='retire', selected=saved, code=2), 'refused:protected-snapshot')
        h.call(root, 'point', selected=saved, id='d000000')
        error(h.call(root, 'select', candidate=candidate, code=2), 'conflict:expected-publication')
        h.check_outcomes(root)
        h.verify(root, 'S-packet-preparation-snapshot-final')


CRASH_PHASES = (
    'pack-reservation', 'pack-install', 'pack-before-catalog', 'physical-before-selection',
    'physical-after-selection', 'pack-accounting', 'pack-loose-delete', 'pack-old-delete',
    'physical-after-owned-file-delete', 'physical-progress-plan-installed',
)


def crash_constructor_binding(h, root, phase):
    """Reviewable source map; absent native/final-source validation stays pending."""
    capability = 'constructor:'+phase
    if capability not in h.a.capabilities:
        raise Unsupported(capability+' needs reviewed final-source constructor mapping')
    binding = h.a.crash_source_binding(root, phase)
    require(binding.get('status') == 'reviewed-source-map'
            and binding.get('final_pin') == h.final_pin and binding.get('checkpoint') == phase
            and isinstance(binding.get('source_sha256'), str) and len(binding['source_sha256']) == 64,
            'crash-constructor-final-source-binding-required')
    h.note('crash-constructor-source-binding', phase=phase, binding=binding)
    return binding


def prepare_deletion_work(h, root, phase, label, forecast):
    """Construct real retirement/abandoned work using EXISTING engine commands.

    No engine-owned path is edited/reinserted. All constructors remain pending
    until crash_constructor_binding matches the actual final source. Forecast
    includes each extra birth/plan and both candidate cohorts BEFORE writes.
    """
    require(forecast.get('cleanup_crash_max_calls') == 8,
            'cleanup-crash-continuation-peak-and-command-forecast-required')
    require(forecast.get('deletion_setup_max_builds') == 2,
            'two-extra-candidate-cohorts-must-be-forecast-before-writes')
    if phase == 'physical-after-owned-file-delete':
        # An actual installed unselected birth, durably abandoned through the
        # supported abort command; later selected Work inherits its real files.
        h.pack(root, cleanup=False, code=86, env={'EXP_EXIT': 'pack-install'})
        journal = h.document(root, 'PACKING.json')
        require(journal.get('status') == 'building' and isinstance(journal.get('id'), str),
                'actual-pack-install-owned-birth-required')
        owner = journal['id']
        owner_prefix = 'packs/'+owner+'/'
        before = h.a.inventory(root)
        owned = {name for name in before['files'] if name.startswith(owner_prefix)
                 and name.endswith(('.pack', '.idx'))}
        require(bool(owned), 'no-real-installed-owned-files')
        h.call(root, 'packing', operation='abort')
        h.pack(root, cleanup=False)
        state = h.state(root)
        garbage = set(state['work'].get('garbage', []))
        targets = owned & garbage
        require(bool(targets) and targets.isdisjoint(h.containers(state))
                and all(h.a.exists(root, name) for name in targets),
                'abandoned-owner-not-inherited-as-real-unselected-garbage')
        h.note('owned-file-deletion-precondition', owner=owner, targets=sorted(targets),
               existing_owned_files=sorted(owned), selected_generation=state['store']['physical'])
        return targets
    if phase == 'pack-old-delete':
        before = h.state(root)
        old = set(h.containers(before))
        # A fresh accepted tail is already present. Reduced limit1 forces
        # source-supported consolidation. It may retain a stable base when
        # multiple groups exist, but at least one actual old instance must retire.
        h.pack(root, codec='zstd', segment_limit=1, cleanup=False)
        after = h.state(root)
        selected = set(h.containers(after))
        targets = set()
        for item in after['work'].get('retired', []):
            descriptor = item['descriptor']
            for field, done, suffix in (('pack', 'pack_done', 'pack'), ('index', 'index_done', 'idx')):
                prefix = 'packs/'+(descriptor['instance']+'/' if descriptor.get('instance') else '')
                name = prefix+descriptor[field]+'.'+suffix
                if not item.get(done, False) and name in old and name not in selected and h.a.exists(root, name):
                    targets.add(name)
        require(bool(targets), 'consolidation-did-not-create-real-retired-container')
        h.note('old-container-deletion-precondition', targets=sorted(targets),
               before=before['store']['physical'], after=after['store']['physical'])
        return targets
    h.pack(root, cleanup=False)
    state = h.state(root)
    require(any(state['work'].get(name) for name in ('jobs', 'retired', 'garbage')),
            'actual-pending-work-required-before-cleanup-checkpoint')
    h.note('cleanup-work-precondition', phase=phase, work=state['work'])
    return set()


def crash_extension(h, root, forecast, *, phase, label):
    """Extend an existing shallow donor with one authentic checkpoint crash.

    Actual deletion phases require Adapter proof of that exact removed instance;
    checkpoint presence or planned deletion alone never satisfies this case.
    """
    require(phase in CRASH_PHASES, 'explicit-source-checkpoint-map-required')
    require('exit:'+phase in h.a.capabilities, 'missing-source-checkpoint:'+phase)
    cleanup_phases = {'pack-loose-delete', 'pack-old-delete',
                      'physical-after-owned-file-delete', 'physical-progress-plan-installed'}
    if phase in cleanup_phases:
        crash_constructor_binding(h, root, phase)  # before any new publication/build
    with h.case('S-crash-'+label, root, forecast):
        h.publish(root, 'S-new-before-'+label)
        semantic = h.read(root, 'SELECTED.json', 65536)
        targets = set()
        if phase in cleanup_phases:
            targets = prepare_deletion_work(h, root, phase, label, forecast)
            h.verify(root, 'S-pre-crash-real-work-'+label)
            before = h.a.inventory(root)
            h.cleanup_to_exit(root, phase, max_calls=forecast['cleanup_crash_max_calls'])
        else:
            before = h.a.inventory(root)
            h.pack(root, 'raw' if phase != 'pack-old-delete' else 'zstd', cleanup=True,
                   code=86, env={'EXP_EXIT': phase})
        require(h.read(root, 'SELECTED.json', 65536) == semantic, 'crash-altered-semantic-authority')
        if phase in {'pack-loose-delete', 'pack-old-delete', 'physical-after-owned-file-delete'}:
            removal = h.a.removal_proof(root, before, phase)
            require(removal.get('actually_removed') is True and removal.get('removed_paths')
                    and removal.get('no_planned_deletion_credit') is True, 'actual-removal-not-exercised')
            if targets:
                require(bool(targets & set(removal['removed_paths'])), 'constructed-instance-not-actually-removed')
            h.note('actual-deletion-crash', phase=phase, proof=removal)
        h.check_outcomes(root)
        h.verify(root, 'S-critical-crash-'+label)
        journal = h.document(root, 'PACKING.json')
        if journal['status'] == 'building':
            h.call(root, 'packing', operation='abort')
            h.pack(root)
        else:
            h.call(root, 'packing', operation='resume', cleanup=False)
        h.drain_two_passes(root)
        h.check_outcomes(root)
        h.verify(root, 'S-recovered-'+label)


def contenders(h, root, forecast):
    with h.case('S-two-maintainers-semantic-crossing', root, forecast):
        h.publish(root, 'S-contender-tail')
        with h.barrier(root, 'S-contender', 'packing', 'pack-before-catalog',
                       operation='build', codec='zstd', force=True,
                       include_attempts=True, cleanup=False):
            response = h.call(root, 'packing', operation='status', code=2, env={'EXP_WAIT_MS': '100'})
            require(response['error'].startswith('busy:competing-maintenance:'), 'two-maintainers-not-serialized')
            h.publish(root, 'S-semantic-under-build')
        h.drain_two_passes(root)
        h.check_outcomes(root)
        h.verify(root, 'S-contenders-final')


def reader_end(h, root, forecast, *, kind):
    """Error uses existing EXP_ERROR; death/cancel need native identity hook."""
    require(kind in {'error', 'cancel', 'death'}, 'reader-end-kind')
    capability = 'reader-'+kind
    if capability not in h.a.capabilities:
        raise Unsupported(capability+' requires native adapter process/identity implementation')
    with h.case('S-reader-'+kind, root, forecast):
        expected = h.ledgers[root].accepted['initial']['result']
        control = h.a.control(root, 'S-reader-'+kind)
        env = dict(EXP_BARRIER='pack-catalog-captured', EXP_READY=control['ready'], EXP_RELEASE=control['release'])
        if kind == 'error':
            env['EXP_ERROR'] = 'pack-catalog-captured'
        handle = h.a.spawn(lambda: h.call(root, 'resolve', request_id='initial', timeout=60,
                                        code=2 if kind == 'error' else 0, env=env))
        try:
            h.a.await_ready(control, handle, 10)
            h.publish(root, 'S-reader-end-'+kind)
            h.pack(root)
            if kind == 'error':
                h.a.release(control)
                error(h.a.wait(handle, 15), 'injected-io:pack-catalog-captured')
            else:
                # Adapter records exact pid/start/native handle, signal/cancel,
                # final raw stdout/stderr, positive release and process closure.
                proof = h.a.interrupt(handle, kind)
                h.note('native-reader-end', kind=kind, proof=proof)
                require(proof.get('identity_verified') is True and proof.get('closed') is True,
                        'native-reader-end-not-proven')
        finally:
            h.a.release(control)
            h.a.finish_interrupted(handle, kind)
        h.drain_two_passes(root)
        require(h.call(root, 'resolve', request_id='initial')['result'] == expected, 'reader-end-harmed-outcomes')
        h.verify(root, 'S-reader-'+kind+'-final')


def injected_case(h, root, forecast, spec):
    """Source-bound extension for native or final paged schema, not engine API.

    The implementation callback may call h.call with EXISTING engine actions.
    It must disclose its source/checkpoint/schema and complete mandatory checks.
    No success is inferred from callback existence, engine audit or summary text.
    """
    require(set(spec) == {'case_id', 'capability', 'source_sha256', 'schema_pin', 'required_checks'},
            'closed-extension-spec')
    if spec['capability'] not in h.a.capabilities:
        raise Unsupported(spec['case_id']+': '+spec['capability'])
    require(spec['schema_pin'] == h.final_pin and len(spec['source_sha256']) == 64,
            'final-source-bound-extension-required')
    with h.case(spec['case_id'], root, forecast):
        proof = h.a.extend(h, root, spec)
        h.note('adapter-extension-result', spec=spec, proof=proof)
        require(proof.get('status') == 'schedule-complete-unqualified'
                and proof.get('source_sha256') == spec['source_sha256']
                and proof.get('schema_pin') == h.final_pin
                and set(spec['required_checks']) <= set(proof.get('checks', [])),
                'extension-obligation-incomplete')
        h.check_outcomes(root)
        h.verify(root, spec['case_id']+'-post-recovery')
