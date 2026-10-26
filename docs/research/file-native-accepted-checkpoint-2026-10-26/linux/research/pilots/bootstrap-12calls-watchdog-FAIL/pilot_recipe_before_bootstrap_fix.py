"""Task-owned native wiring of the retained real pilot; original preserved."""
import copy, hashlib, time
PIN="0628322e4b48e0efad19cdc32238a622d4aaa7a8"
CASE="readiness-real-online-A-H3"
def need(value,reason):
    if not value:raise RuntimeError(reason)
def digest(raw):return hashlib.sha256(raw).hexdigest()
def environment(control,phase):return dict(EXP_BARRIER=phase,EXP_READY=control['ready'],EXP_RELEASE=control['release'])
def run(h,root,forecast):
    with h.case(CASE, root, forecast):
        h.note('readiness-pilot-native-start', forecast=copy.deepcopy(forecast))
        h.initialize(root, 'A', count=64)
        h.pack(root)                 # actual base physical representation
        h.cleanup(root, max_batches=8)
        h.publish(root, 'readiness-tail', retries=1)  # H2, after completed cutoff
        before = h.state(root)
        key = 'publications/'+before['selected']['publication']+'.json'
        raw = h.read(root, key)
        unmapped = h.a.mapping(root, key)
        need(unmapped.get('mapped') is False and unmapped.get('independent') is True
             and unmapped.get('receipt_sha256'), 'real-unmapped-tail')
        expected_old = h.ledgers[root].accepted['readiness-tail']['result']
        calls, controls, roles, starts = [], [], [], {}
        primary = None
        target = candidate = plan_raw = None

        def observe(role):
            def seen(event):
                need(role not in starts and type(event) is dict
                     and type(event.get('pid')) is int and event['pid'] > 0
                     and type(event.get('positive_rss')) is int
                     and 0 < event['positive_rss'] <= (268435456 if role == 'old' else 2147483648)
                     and event.get('before_stdin_delivery') is True
                     and type(event.get('observed_ns')) is int and event['observed_ns'] > 0,
                     'positive-start:'+role)
                starts[role] = copy.deepcopy(event)
            return seen

        def start(role, action, phase, **fields):
            control = h.a.control(root, 'readiness-'+role)
            controls.append(control)
            handle = h.a.spawn(lambda: h.call(root, action, timeout=60,
                env=environment(control, phase), on_started=observe(role), **fields))
            calls.append(handle)
            roles.append(role)
            h.a.await_ready(control, handle, 10)
            need(role in starts and h.a.alive(handle), 'real-held-process:'+role)
            return handle, control

        try:
            old, old_control = start('old', 'resolve', 'pack-catalog-captured',
                                     request_id='readiness-tail')
            old_ready_ns = time.perf_counter_ns()
            held_since = h.a.monotonic()
            maintenance, maintenance_control = start('maintenance', 'packing',
                'physical-before-selection', operation='build', codec='zstd',
                include_attempts=True, cleanup=False, force=True)
            journal = h.document(root, 'PACKING.json')
            target = journal.get('target') or {}
            binding = target.get('physical') or {}
            old_binding = before['store']['physical']
            need(journal.get('status') == 'installed'
                 and journal.get('id') == binding.get('id')
                 and binding.get('id') != old_binding['id']
                 and binding.get('ordinal') == old_binding['ordinal']+1
                 and target.get('catalog') != before['store']['catalog']
                 and journal.get('expected_config') == digest(h.read(root, 'STORE.json', 4096)),
                 'prepared-exact-parent-new-physical-target')
            plan_raw = h.read(root, 'physical-plans/'+binding['plan']+'.json')
            need(digest(plan_raw) == binding['plan'], 'candidate-plan-hash')
            # Existing strict duplicate-key/nonfinite parser, bound to Harness.
            candidate = h.document(root, 'physical-plans/'+binding['plan']+'.json')
            generation = candidate.get('generation') or {}
            discovery = candidate.get('discovery') or {}
            need(candidate.get('physical_parent') == journal['expected_config']
                 and generation.get('id') == binding['id']
                 and generation.get('ordinal') == binding['ordinal']
                 and generation.get('catalog') == target['catalog']
                 and (discovery.get('completed') or {}).get('selected') == before['selected']
                 and discovery.get('interval') is None and discovery.get('deferred') == [],
                 'complete-captured-H2-candidate')
            request = dict(request='readiness-after-capture', base=before['selected'],
                           ops=[], packet=False)
            h.ledgers[root].record(request)
            publisher, publisher_control = start('publisher', 'publish', 'candidate-ready',
                                                   request=request)
            need(len({starts[r]['pid'] for r in starts}) == 3, 'three-native-processes')
            need(h.a.alive(old) and h.a.alive(maintenance), 'old-and-maintenance-still-held')
            # Do NOT call join_callbacks here: it releases every barrier.
            h.a.release(publisher_control)
            published = h.a.wait(publisher, 15)
            value = published['result']
            h.ledgers[root].accept(request, value['result'], value['selected'])
            need(h.selected(root) == value['selected']
                 and value['selected'] != before['selected']
                 and h.document(root, 'STORE.json', 4096) == before['store']
                 and h.a.alive(maintenance) and h.a.alive(old),
                 'accepted-H3-selected-before-maintenance-release')
            accepted_ns = time.perf_counter_ns()
            h.note('readiness-accepted-before-activation', selected=value['selected'],
                   result=value['result'], candidate_plan_sha256=digest(plan_raw),
                   native_starts=starts, old_ready_ns=old_ready_ns, accepted_ns=accepted_ns)
            h.a.release(maintenance_control)
            maintenance_result = h.a.wait(maintenance, 15)
            activated_ns = time.perf_counter_ns()
            after = h.state(root)
            need(after['store'] == target and after['work'] == candidate
                 and after['selected'] == value['selected'], 'exact-activation-preserves-H3')
            need(after['store']['physical']['id'] != old_binding['id']
                 and after['store']['physical']['ordinal'] == old_binding['ordinal']+1
                 and after['catalog'] != before['catalog']
                 and any(row not in before['catalog']['packs'] for row in after['catalog']['packs']),
                 'actual-mapping-activation-no-op-forbidden')
            metric = maintenance_result.get('metrics') or {}
            need(len(metric.get('physical_activation_holds_ns', [])) == 1
                 and metric.get('pack_eligible_records', 0) > 0
                 and metric.get('rolling_publications_visited', 0) > 0,
                 'real-physical-activation-metrics')
            need(h.a.alive(old) and h.read(root, key) == raw, 'old-pin-loose-fallback-preserved')
            need(h.call(root, 'resolve', request_id=request['request'])['result'] == value['result'],
                 'fresh-current-reader-H3')
            h.cleanup(root, max_batches=1, require_complete=False)
            need(h.a.alive(old) and h.read(root, key) == raw, 'cleanup-preserves-old-pin')
            h.a.sleep(max(0.0, 16.1-(h.a.monotonic()-held_since)))
            need(h.a.alive(old) and h.a.monotonic()-held_since > 15, 'old-pin-more-than-15s')
            h.a.release(old_control)
            old_result = h.a.wait(old, 15)
            need(old_result['result'] == expected_old, 'old-protected-reader-exact-result')
            h.note('readiness-real-online-joined', accepted_ns=accepted_ns,
                   activation_joined_ns=activated_ns,
                   exact_selected=value['selected'], native_starts=starts)
        except BaseException as error:
            primary = error
        finally:
            # Releases and observes ALL callbacks even if a spawn/ready/join,
            # publication check, mapping check or evidence append failed.
            h.join_callbacks(root, 'readiness-finally', calls, controls, primary=primary)
        # Linux's complete decoder requires quiescent native calls. No mapping
        # helper is invoked while an old reader or maintenance callback is held.
        mapped = h.a.mapping(root, key)
        need(mapped.get('mapped') is True and mapped.get('raw_sha256') == digest(raw)
             and mapped.get('independent') is True and mapped.get('receipt_sha256'),
             'independent-complete-mapping-binding')
        held_inventory = h.a.inventory(root)
        h.drain_two_passes(root)  # exactly two final maintenance invocations
        backlog = h.a.backlog(root)
        need(type(backlog.get('unprotected')) is int and backlog['unprotected'] == 0,
             'actual-unprotected-backlog-drained')
        need(not h.a.exists(root, key), 'actual-A-loose-tail-removal-after-pin-release')
        final_inventory = h.a.inventory(root)
        h.note('readiness-actual-cleanup-inventories', before=held_inventory,
               after=final_inventory, removed_tail=key, no_planned_deletion_credit=True)
        h.check_outcomes(root)
        proof = h.verify(root, 'readiness-H3-after-release-and-cleanup', mapped_keys=[key])
        need(proof.get('actual_H') == 3, 'complete-H3-oracle')
        h.note('readiness-pilot-semantic-complete', actual_H=3, proof=proof,
               qualification='awaiting-host-native-EOF-descendants-locks-registry-accounting')
    return dict(status="semantic-pilot-complete-awaiting-native-finalizer",case_id=CASE,baseline_pin=PIN,full_matrix_released=False)
