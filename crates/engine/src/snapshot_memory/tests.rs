use super::delta::{
    derive_snapshot_delta, load_drift_catalog, validate_delta, DeltaError, DeltaErrorKind,
    DriftCatalog, SnapshotEndpoint,
};
use super::policy::{load_policy, validate_capture_input, PolicyErrorKind};
use super::record::consistency::{
    derive_consistency, validate_prior_end, Admissibility, BoundaryRecord, BoundaryTrigger,
    BoundedEvidence, ConsistencyErrorKind, ConsistencyPolicy, FamilyExclusion, FamilyObservation,
    SnapshotConsistency, SourceSlotObservation, UnstableAction,
};
use super::record::{build_snapshot, RecordErrorKind};
use super::redaction::{
    approve_deletion, classify_pointer_read, create_compaction, deduplicate_payloads,
    evaluate_redaction, load_redaction_retention_fixture, resolve_retention, validate_disposition,
    DeletionGuard, ImmutableRecord, MatcherOutcome, PointerRead, RedactionAction, RedactionError,
    RedactionErrorKind, SurfaceClassification,
};
use crate::DefinitionFingerprint;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

const POLICY_FIXTURE: &[u8] = include_bytes!("fixtures/policy-family.json");
const SNAPSHOT_RECORD_FIXTURE: &[u8] = include_bytes!("fixtures/snapshot-record.json");
const DELTA_CATALOG_FIXTURE: &[u8] = include_bytes!("fixtures/delta-catalog.json");
const REDACTION_RETENTION_FIXTURE: &[u8] = include_bytes!("fixtures/redaction-retention.json");

#[test]
fn canonical_policy_fingerprint_is_independent_of_authored_collection_order() {
    let forward = repaired_policy_fixture();
    let forward_policy = load_policy(&canonical_json(&forward)).expect("valid policy fixture");

    let mut reverse = forward;
    reverse["triggers"].as_array_mut().unwrap().reverse();
    reverse["allowed_memory_horizons"]
        .as_array_mut()
        .unwrap()
        .reverse();
    reverse["state_families"]["work"]["source_slots"]
        .as_array_mut()
        .unwrap()
        .reverse();
    reverse["state_families"]["work"]["windows"]
        .as_array_mut()
        .unwrap()
        .reverse();
    let reverse_policy = load_policy(&canonical_json(&reverse)).expect("reordered policy");

    assert_eq!(forward_policy.fingerprint(), reverse_policy.fingerprint());
    assert_eq!(
        forward_policy.canonical_bytes().expect("canonical bytes"),
        reverse_policy.canonical_bytes().expect("canonical bytes")
    );
}

#[test]
fn declared_live_revisions_and_cursors_change_capture_input_not_policy_identity() {
    let policy = valid_policy();
    let first = validate_capture_input(&policy, &canonical_json(&valid_capture_input(&policy)))
        .expect("valid capture input");

    let mut changed = valid_capture_input(&policy);
    changed["families"][0]["slots"][0]["revision"]["fingerprint"] = json!(fingerprint('a'));
    changed["families"][2]["slots"][0]["cursor"] = json!("cursor-v2");
    let second = validate_capture_input(&policy, &canonical_json(&changed))
        .expect("changed live capture input");

    assert_ne!(
        first.capture_input_fingerprint(),
        second.capture_input_fingerprint()
    );
    assert_eq!(
        policy.fingerprint(),
        load_policy(&canonical_json(&repaired_policy_fixture()))
            .expect("same policy")
            .fingerprint()
    );
}

#[test]
fn policy_refuses_duplicate_unknown_and_malformed_static_closure() {
    let mut duplicate_trigger = repaired_policy_fixture();
    duplicate_trigger["triggers"] = json!(["session_start", "session_start"]);
    assert_error(
        load_policy(&canonical_json(&duplicate_trigger)),
        PolicyErrorKind::DuplicateTrigger,
    );

    let mut duplicate_horizon = repaired_policy_fixture();
    duplicate_horizon["allowed_memory_horizons"] = json!(["execution", "execution"]);
    repair_policy_fingerprint(&mut duplicate_horizon);
    assert_error(
        load_policy(&canonical_json(&duplicate_horizon)),
        PolicyErrorKind::DuplicateHorizon,
    );

    let mut unknown_family = repaired_policy_fixture();
    unknown_family["state_families"]["ambient"] = json!({
        "source_adapter": pair("handbook.snapshot-source.ambient@1.0.0", 'a'),
        "source_slots": [],
        "selected_fields": ["anything"],
        "windows": []
    });
    repair_policy_fingerprint(&mut unknown_family);
    assert_error(
        load_policy(&canonical_json(&unknown_family)),
        PolicyErrorKind::UnknownFamily,
    );

    let mut malformed_reference = repaired_policy_fixture();
    malformed_reference["state_families"]["git"]["source_adapter"]["ref"] = json!("latest");
    repair_policy_fingerprint(&mut malformed_reference);
    assert_error(
        load_policy(&canonical_json(&malformed_reference)),
        PolicyErrorKind::InvalidExactPair,
    );

    let mut mismatched_fingerprint = repaired_policy_fixture();
    mismatched_fingerprint["policy_fingerprint"] = json!(fingerprint('f'));
    assert_error(
        load_policy(&canonical_json(&mismatched_fingerprint)),
        PolicyErrorKind::FingerprintMismatch,
    );
}

#[test]
fn capture_input_refuses_scope_widening_and_undeclared_live_sources() {
    let policy = valid_policy();

    let mut unknown_trigger = valid_capture_input(&policy);
    unknown_trigger["trigger"] = json!("command_run");
    assert_capture_error(&policy, unknown_trigger, PolicyErrorKind::UnknownTrigger);

    let mut invalid_horizon = valid_capture_input(&policy);
    invalid_horizon["memory_horizon"] = json!("durable");
    assert_capture_error(&policy, invalid_horizon, PolicyErrorKind::InvalidHorizon);

    let mut duplicate_family = valid_capture_input(&policy);
    let duplicate = duplicate_family["families"][0].clone();
    duplicate_family["families"]
        .as_array_mut()
        .unwrap()
        .push(duplicate);
    assert_capture_error(&policy, duplicate_family, PolicyErrorKind::DuplicateFamily);

    let mut unknown_family = valid_capture_input(&policy);
    unknown_family["families"][0]["family"] = json!("ambient");
    assert_capture_error(&policy, unknown_family, PolicyErrorKind::UnknownFamily);

    let mut duplicate_slot = valid_capture_input(&policy);
    let duplicate = duplicate_slot["families"][4]["slots"][0].clone();
    duplicate_slot["families"][4]["slots"]
        .as_array_mut()
        .unwrap()
        .push(duplicate);
    assert_capture_error(
        &policy,
        duplicate_slot,
        PolicyErrorKind::DuplicateSourceSlot,
    );

    let mut unknown_slot = valid_capture_input(&policy);
    unknown_slot["families"][4]["slots"][0]["slot"] = json!("ambient_slot");
    assert_capture_error(&policy, unknown_slot, PolicyErrorKind::UnknownSourceSlot);

    let mut duplicate_window = valid_capture_input(&policy);
    let duplicate = duplicate_window["families"][4]["windows"][0].clone();
    duplicate_window["families"][4]["windows"]
        .as_array_mut()
        .unwrap()
        .push(duplicate);
    assert_capture_error(&policy, duplicate_window, PolicyErrorKind::DuplicateWindow);

    let mut unknown_window = valid_capture_input(&policy);
    unknown_window["families"][4]["windows"][0]["window_id"] = json!("ambient_window");
    assert_capture_error(&policy, unknown_window, PolicyErrorKind::UnknownWindow);

    let mut ambient_source = valid_capture_input(&policy);
    ambient_source["ambient_sources"] = json!(["git"]);
    assert_capture_error(
        &policy,
        ambient_source,
        PolicyErrorKind::MalformedCaptureInput,
    );
}

fn valid_policy() -> super::policy::SnapshotCapturePolicy {
    load_policy(&canonical_json(&repaired_policy_fixture())).expect("valid policy")
}

fn valid_capture_input(policy: &super::policy::SnapshotCapturePolicy) -> Value {
    json!({
        "policy": exact_pair(policy.reference(), policy.fingerprint().as_str()),
        "trigger": "session_start",
        "memory_horizon": "execution",
        "families": [
            single_slot_capture("evidence", "evidence", "evidence.revision.1", None),
            single_slot_capture("git", "git", "git.revision.1", None),
            single_slot_capture("handbook", "handbook", "handbook.revision.1", None),
            single_slot_capture("session", "session", "session.revision.1", Some("session-cursor")),
            {
                "family": "work",
                "slots": [
                    {"slot": "active_plan", "revision": pair("work.active-plan.1", 'b'), "cursor": null},
                    {"slot": "work_ledger", "revision": pair("work.ledger.1", 'c'), "cursor": "ledger-cursor"}
                ],
                "windows": [
                    {
                        "window_id": "queued_next",
                        "slot": "active_plan",
                        "revision": pair("work.active-plan.1", 'b'),
                        "cursor": null
                    },
                    {
                        "window_id": "recent_completed",
                        "slot": "work_ledger",
                        "revision": pair("work.ledger.1", 'c'),
                        "cursor": "ledger-cursor"
                    }
                ]
            }
        ]
    })
}

fn single_slot_capture(family: &str, slot: &str, revision: &str, cursor: Option<&str>) -> Value {
    json!({
        "family": family,
        "slots": [{"slot": slot, "revision": pair(revision, 'd'), "cursor": cursor}],
        "windows": []
    })
}

fn repaired_policy_fixture() -> Value {
    let mut value = serde_json::from_slice(POLICY_FIXTURE).expect("fixture JSON");
    repair_policy_fingerprint(&mut value);
    value
}

fn repair_policy_fingerprint(value: &mut Value) {
    let mut preimage = value.clone();
    preimage
        .as_object_mut()
        .expect("policy object")
        .remove("policy_fingerprint");
    let fingerprint = DefinitionFingerprint::from_json_value(&preimage)
        .expect("policy fingerprint")
        .to_string();
    value["policy_fingerprint"] = json!(fingerprint);
}

fn pair(reference: &str, fill: char) -> Value {
    exact_pair(reference, &fingerprint(fill))
}

fn exact_pair(reference: &str, fingerprint: &str) -> Value {
    json!({"ref": reference, "fingerprint": fingerprint})
}

fn fingerprint(fill: char) -> String {
    format!("sha256:{}", fill.to_string().repeat(64))
}

fn canonical_json(value: &Value) -> Vec<u8> {
    serde_json_canonicalizer::to_vec(value).expect("canonical JSON")
}

fn assert_error<T: std::fmt::Debug>(
    result: Result<T, super::policy::PolicyError>,
    kind: PolicyErrorKind,
) {
    assert_eq!(result.expect_err("expected refusal").kind(), kind);
}

fn assert_capture_error(
    policy: &super::policy::SnapshotCapturePolicy,
    input: Value,
    kind: PolicyErrorKind,
) {
    assert_error(
        validate_capture_input(policy, &canonical_json(&input)),
        kind,
    );
}

#[test]
fn normalized_snapshot_replay_is_independent_of_family_path_and_ref_order() {
    let policy = valid_policy();
    let capture = validate_capture_input(&policy, &canonical_json(&valid_capture_input(&policy)))
        .expect("valid capture input");
    let first = build_snapshot(
        &policy,
        &capture,
        &[],
        &canonical_json(&repaired_snapshot_record_fixture()),
    )
    .expect("valid snapshot record");

    let mut reordered = repaired_snapshot_record_fixture();
    reordered["family_observations"]
        .as_array_mut()
        .unwrap()
        .reverse();
    reordered["family_observations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|observation| observation["family"] == "work")
        .expect("work observation")["source_slot_revisions"]
        .as_array_mut()
        .unwrap()
        .reverse();
    reordered["family_observations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|observation| observation["family"] == "work")
        .expect("work observation")["windows"]
        .as_array_mut()
        .unwrap()
        .reverse();
    reordered["family_observations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|observation| observation["family"] == "git")
        .expect("git observation")["payload"]["paths"]
        .as_array_mut()
        .unwrap()
        .reverse();
    reordered["family_observations"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|observation| observation["family"] == "handbook")
        .expect("handbook observation")["payload"]["artifact_refs"]
        .as_array_mut()
        .unwrap()
        .reverse();
    let replay = build_snapshot(&policy, &capture, &[], &canonical_json(&reordered))
        .expect("reordered snapshot record");

    assert_eq!(first.state_fingerprint(), replay.state_fingerprint());
    assert_eq!(first.record_fingerprint(), replay.record_fingerprint());
    assert_eq!(
        first.normalized_bytes().expect("normalized bytes"),
        replay.normalized_bytes().expect("normalized bytes")
    );
}

#[test]
fn boundary_metadata_changes_record_identity_without_changing_state_identity() {
    let policy = valid_policy();
    let start_capture =
        validate_capture_input(&policy, &canonical_json(&valid_capture_input(&policy)))
            .expect("valid start capture input");
    let first = build_snapshot(
        &policy,
        &start_capture,
        &[],
        &canonical_json(&repaired_snapshot_record_fixture()),
    )
    .expect("valid start snapshot");

    let mut end_input = valid_capture_input(&policy);
    end_input["trigger"] = json!("session_end");
    let end_capture = validate_capture_input(&policy, &canonical_json(&end_input))
        .expect("valid end capture input");
    let mut later_boundary = repaired_snapshot_record_fixture();
    later_boundary["snapshot_id"] = json!("snap_session_end_0002");
    later_boundary["capture"]["trigger"] = json!("session_end");
    later_boundary["capture"]["input"]["trigger"] = json!("session_end");
    later_boundary["capture"]["started_at"] = json!("2026-08-06T04:01:00Z");
    later_boundary["capture"]["completed_at"] = json!("2026-08-06T04:01:05Z");
    later_boundary["boundary_stream_ref"] = json!("orchestration.example.end");
    later_boundary["boundary_sequence"] = json!(2);
    let second = build_snapshot(&policy, &end_capture, &[], &canonical_json(&later_boundary))
        .expect("valid end snapshot");

    assert_eq!(first.state_fingerprint(), second.state_fingerprint());
    assert_ne!(first.record_fingerprint(), second.record_fingerprint());
}

#[test]
fn normalized_snapshot_refuses_incomplete_duplicate_extra_and_bare_ref_coverage() {
    let policy = valid_policy();
    let capture = validate_capture_input(&policy, &canonical_json(&valid_capture_input(&policy)))
        .expect("valid capture input");

    let mut missing = repaired_snapshot_record_fixture();
    missing["family_observations"]
        .as_array_mut()
        .unwrap()
        .retain(|observation| observation["family"] != "evidence");
    assert_record_error(
        build_snapshot(&policy, &capture, &[], &canonical_json(&missing)),
        RecordErrorKind::MissingFamily,
    );

    let mut duplicate = repaired_snapshot_record_fixture();
    let duplicate_observation = duplicate["family_observations"][0].clone();
    duplicate["family_observations"]
        .as_array_mut()
        .unwrap()
        .push(duplicate_observation);
    assert_record_error(
        build_snapshot(&policy, &capture, &[], &canonical_json(&duplicate)),
        RecordErrorKind::DuplicateFamily,
    );

    let mut extra = repaired_snapshot_record_fixture();
    let mut extra_observation = extra["family_observations"][0].clone();
    extra_observation["family"] = json!("ambient");
    extra["family_observations"]
        .as_array_mut()
        .unwrap()
        .push(extra_observation);
    assert_record_error(
        build_snapshot(&policy, &capture, &[], &canonical_json(&extra)),
        RecordErrorKind::UnknownFamily,
    );

    let mut bare_ref = repaired_snapshot_record_fixture();
    bare_ref["family_observations"][0]["source_adapter"] =
        json!("handbook.snapshot-source.evidence@1.0.0");
    assert_record_error(
        build_snapshot(&policy, &capture, &[], &canonical_json(&bare_ref)),
        RecordErrorKind::MalformedRecord,
    );
}

#[test]
fn typed_exclusions_are_normalized_as_explicit_incomplete_evidence() {
    let policy = valid_policy();
    let capture = validate_capture_input(&policy, &canonical_json(&valid_capture_input(&policy)))
        .expect("valid capture input");
    let mut excluded = repaired_snapshot_record_fixture();
    excluded["family_observations"]
        .as_array_mut()
        .unwrap()
        .retain(|observation| observation["family"] != "evidence");
    excluded["excluded_families"] = json!([{"family": "evidence", "reason": "unavailable"}]);

    let snapshot = build_snapshot(&policy, &capture, &[], &canonical_json(&excluded))
        .expect("typed exclusion is explicit evidence");

    assert_eq!(
        snapshot.normalized_value()["excluded_families"][0]["family"],
        "evidence"
    );
    assert_eq!(
        snapshot.normalized_value()["excluded_families"][0]["reason"],
        "unavailable"
    );
}

#[test]
fn snapshots_bind_admitted_capture_and_derived_evidence() {
    let policy = valid_policy();
    let capture = validate_capture_input(&policy, &canonical_json(&valid_capture_input(&policy)))
        .expect("valid capture input");
    let snapshot = build_snapshot(
        &policy,
        &capture,
        &[],
        &canonical_json(&repaired_snapshot_record_fixture()),
    )
    .expect("valid snapshot record");

    assert_eq!(
        snapshot.normalized_value()["capture"]["input"]["memory_horizon"],
        "execution"
    );
    assert_eq!(
        snapshot.normalized_value()["capture"]["consistency"],
        "stable"
    );
    assert_eq!(
        snapshot.normalized_value()["admissibility"],
        "grounding_and_evidence"
    );
    assert!(snapshot.normalized_value()["previous_snapshot"].is_null());

    let mut forged_input = repaired_snapshot_record_fixture();
    forged_input["capture"]["input"]["memory_horizon"] = json!("operation");
    assert_record_error(
        build_snapshot(&policy, &capture, &[], &canonical_json(&forged_input)),
        RecordErrorKind::InvalidCaptureProvenance,
    );

    let mut bounded_record = repaired_snapshot_record_fixture();
    bounded_record["family_observations"]
        .as_array_mut()
        .expect("family observations")
        .iter_mut()
        .find(|observation| observation["family"] == "git")
        .expect("git observation")["bound_evaluation"] = json!({
        "rule": pair("handbook.snapshot-bound.exact-revision-per-family@1.0.0", '5'),
        "evaluated_revisions": {
            "family_revision": pair("git.revision.1", 'd'),
            "source_slot_revisions": {}
        },
        "outcome": "within_bound",
        "evidence_fingerprint": fingerprint('e')
    });
    let bounded = build_snapshot(&policy, &capture, &[], &canonical_json(&bounded_record))
        .expect("valid bounded snapshot");
    assert_eq!(
        bounded.normalized_value()["capture"]["consistency"],
        "bounded"
    );

    let mut end_input = valid_capture_input(&policy);
    end_input["trigger"] = json!("session_end");
    let end_capture = validate_capture_input(&policy, &canonical_json(&end_input))
        .expect("valid end capture input");
    let mut end_record = repaired_snapshot_record_fixture();
    end_record["snapshot_id"] = json!("snap_session_end_0002");
    end_record["capture"]["trigger"] = json!("session_end");
    end_record["capture"]["input"]["trigger"] = json!("session_end");
    end_record["boundary_stream_ref"] = json!("orchestration.example");
    end_record["boundary_sequence"] = json!(2);
    let end = build_snapshot(&policy, &end_capture, &[], &canonical_json(&end_record))
        .expect("valid prior end snapshot");

    let mut next_record = repaired_snapshot_record_fixture();
    next_record["snapshot_id"] = json!("snap_session_start_0003");
    next_record["boundary_stream_ref"] = json!("orchestration.example");
    next_record["boundary_sequence"] = json!(3);
    next_record["previous_snapshot"] = json!({
        "ref": "snap_session_end_0002",
        "record_fingerprint": end.record_fingerprint().to_string(),
        "boundary_sequence": 2
    });
    let next = build_snapshot(&policy, &capture, &[&end], &canonical_json(&next_record))
        .expect("validated immediate predecessor");
    assert_eq!(
        next.normalized_value()["previous_snapshot"]["record_fingerprint"],
        end.record_fingerprint().to_string()
    );

    next_record["previous_snapshot"]["record_fingerprint"] = json!(fingerprint('f'));
    assert_record_error(
        build_snapshot(&policy, &capture, &[&end], &canonical_json(&next_record)),
        RecordErrorKind::InvalidPredecessor,
    );
}

fn repaired_snapshot_record_fixture() -> Value {
    let mut value: Value = serde_json::from_slice(SNAPSHOT_RECORD_FIXTURE).expect("fixture JSON");
    for observation in value["family_observations"]
        .as_array_mut()
        .expect("family observations")
    {
        let mut payload = observation["payload"].clone();
        normalize_fixture_payload(&mut payload, None);
        let fingerprint = DefinitionFingerprint::from_json_value(&payload)
            .expect("payload fingerprint")
            .to_string();
        observation["payload_fingerprint"] = json!(fingerprint);
    }
    value
}

fn normalize_fixture_payload(value: &mut Value, field_name: Option<&str>) {
    match value {
        Value::Array(values) => {
            for value in values.iter_mut() {
                normalize_fixture_payload(value, None);
            }
            if field_name == Some("paths")
                || field_name == Some("refs")
                || field_name.is_some_and(|field| field.ends_with("_refs"))
            {
                values.sort_by(|left, right| canonical_json(left).cmp(&canonical_json(right)));
            }
        }
        Value::Object(values) => {
            for (field, value) in values {
                normalize_fixture_payload(value, Some(field));
            }
        }
        _ => {}
    }
}

fn assert_record_error(
    result: Result<super::record::ContextMemorySnapshot, super::record::RecordError>,
    kind: RecordErrorKind,
) {
    assert_eq!(result.expect_err("expected refusal").kind(), kind);
}

#[test]
fn snapshot_consistency_derives_stable_and_bounded_from_complete_family_aggregation() {
    let stable = derive_consistency(
        &consistency_policy(UnstableAction::PersistNonPromotable),
        fixture_observations(),
        Vec::new(),
    )
    .expect("all stable families are admissible");
    assert_eq!(stable.consistency, SnapshotConsistency::Stable);
    assert_eq!(stable.admissibility, Admissibility::GroundingAndEvidence);
    assert!(stable.bounded_evidence.is_empty());

    let mut bounded_observations = fixture_observations();
    let work = bounded_observations
        .iter_mut()
        .find(|observation| observation.family == "work")
        .expect("work observation");
    work.bound_evaluation = Some(BoundedEvidence {
        rule: consistency_policy(UnstableAction::PersistNonPromotable).bounded_rule,
        family_revision: work.captured_revision.clone(),
        source_slot_revisions: captured_slot_revisions(work),
        evidence_fingerprint: DefinitionFingerprint::parse(&fingerprint('e'))
            .expect("evidence fingerprint"),
        within_bound: true,
    });
    let bounded = derive_consistency(
        &consistency_policy(UnstableAction::PersistNonPromotable),
        bounded_observations,
        Vec::new(),
    )
    .expect("exact bounded evidence is admissible");
    assert_eq!(bounded.consistency, SnapshotConsistency::Bounded);
    assert_eq!(bounded.admissibility, Admissibility::GroundingAndEvidence);
    assert_eq!(
        bounded.bounded_evidence["work"].rule,
        consistency_policy(UnstableAction::PersistNonPromotable).bounded_rule
    );
}

#[test]
fn snapshot_consistency_refuses_or_downgrades_changed_excluded_and_invalid_bounded_inputs() {
    let policy = consistency_policy(UnstableAction::PersistNonPromotable);

    let mut active_plan_drift = fixture_observations();
    active_plan_drift
        .iter_mut()
        .find(|observation| observation.family == "work")
        .expect("work observation")
        .source_slot_revisions
        .iter_mut()
        .find(|slot| slot.source_slot == "active_plan")
        .expect("active-plan slot")
        .post_revision = revision("work.active-plan.2", '9');
    assert_unstable(&policy, active_plan_drift, Vec::new());

    let mut changed_source = fixture_observations();
    changed_source
        .iter_mut()
        .find(|observation| observation.family == "git")
        .expect("git observation")
        .post_revision = revision("git.revision.2", '8');
    assert_unstable(&policy, changed_source, Vec::new());

    let mut out_of_bound = fixture_observations();
    let git = out_of_bound
        .iter_mut()
        .find(|observation| observation.family == "git")
        .expect("git observation");
    git.post_revision = revision("git.revision.2", '8');
    git.bound_evaluation = Some(BoundedEvidence {
        rule: policy.bounded_rule.clone(),
        family_revision: git.captured_revision.clone(),
        source_slot_revisions: captured_slot_revisions(git),
        evidence_fingerprint: DefinitionFingerprint::parse(&fingerprint('e'))
            .expect("evidence fingerprint"),
        within_bound: false,
    });
    assert_unstable(&policy, out_of_bound, Vec::new());

    let mut excluded = fixture_observations();
    excluded.retain(|observation| observation.family != "evidence");
    assert_unstable(
        &policy,
        excluded,
        vec![FamilyExclusion::unavailable("evidence")],
    );

    let mut substituted_rule = fixture_observations();
    let git = substituted_rule
        .iter_mut()
        .find(|observation| observation.family == "git")
        .expect("git observation");
    git.bound_evaluation = Some(BoundedEvidence {
        rule: revision("handbook.snapshot-bound.substituted@1.0.0", '7'),
        family_revision: git.captured_revision.clone(),
        source_slot_revisions: captured_slot_revisions(git),
        evidence_fingerprint: DefinitionFingerprint::parse(&fingerprint('e'))
            .expect("evidence fingerprint"),
        within_bound: true,
    });
    assert_consistency_error(
        derive_consistency(&policy, substituted_rule, Vec::new()),
        ConsistencyErrorKind::InvalidBoundedEvidence,
    );

    let mut refused = fixture_observations();
    refused
        .iter_mut()
        .find(|observation| observation.family == "git")
        .expect("git observation")
        .post_revision = revision("git.revision.2", '8');
    assert_consistency_error(
        derive_consistency(
            &consistency_policy(UnstableAction::Refuse),
            refused,
            Vec::new(),
        ),
        ConsistencyErrorKind::UnstableRefused,
    );
}

#[test]
fn prior_end_new_start_selects_the_immediate_eligible_same_stream_predecessor() {
    let current = boundary("start-3", 3, BoundaryTrigger::SessionStart, Some("end-2"));
    let predecessor = validate_prior_end(
        &[
            boundary("end-1", 1, BoundaryTrigger::SessionEnd, None),
            boundary("end-2", 2, BoundaryTrigger::SessionEnd, None),
        ],
        &current,
    )
    .expect("current start follows the immediate prior end")
    .expect("prior end");
    assert_eq!(predecessor.snapshot_id, "end-2");
    assert_eq!(predecessor.boundary_sequence, 2);
}

#[test]
fn prior_end_new_start_refuses_invalid_predecessor_topology() {
    let self_link = boundary("start-2", 2, BoundaryTrigger::SessionStart, Some("start-2"));
    assert_topology_error(
        validate_prior_end(&[], &self_link),
        ConsistencyErrorKind::SelfPredecessor,
    );

    let future = boundary("start-2", 2, BoundaryTrigger::SessionStart, Some("end-3"));
    assert_topology_error(
        validate_prior_end(
            &[boundary("end-3", 3, BoundaryTrigger::SessionEnd, None)],
            &future,
        ),
        ConsistencyErrorKind::FuturePredecessor,
    );

    let cyclic = boundary("start-3", 3, BoundaryTrigger::SessionStart, Some("end-2"));
    assert_topology_error(
        validate_prior_end(
            &[boundary(
                "end-2",
                2,
                BoundaryTrigger::SessionEnd,
                Some("start-3"),
            )],
            &cyclic,
        ),
        ConsistencyErrorKind::CyclicPredecessor,
    );

    let skipped = boundary("start-3", 3, BoundaryTrigger::SessionStart, Some("end-1"));
    assert_topology_error(
        validate_prior_end(
            &[
                boundary("end-1", 1, BoundaryTrigger::SessionEnd, None),
                boundary("end-2", 2, BoundaryTrigger::SessionEnd, None),
            ],
            &skipped,
        ),
        ConsistencyErrorKind::SkippedPredecessor,
    );

    let wrong_stream = boundary(
        "start-3",
        3,
        BoundaryTrigger::SessionStart,
        Some("end-other"),
    );
    let mut other_stream = boundary("end-other", 2, BoundaryTrigger::SessionEnd, None);
    other_stream.boundary_stream_ref = "other-stream".to_owned();
    assert_topology_error(
        validate_prior_end(&[other_stream], &wrong_stream),
        ConsistencyErrorKind::WrongPredecessorStream,
    );

    let wrong_trigger = boundary("start-3", 3, BoundaryTrigger::SessionStart, Some("start-2"));
    assert_topology_error(
        validate_prior_end(
            &[boundary("start-2", 2, BoundaryTrigger::SessionStart, None)],
            &wrong_trigger,
        ),
        ConsistencyErrorKind::WrongPredecessorTrigger,
    );
}

fn consistency_policy(unstable_action: UnstableAction) -> ConsistencyPolicy {
    ConsistencyPolicy {
        selected_families: BTreeSet::from([
            "evidence".to_owned(),
            "git".to_owned(),
            "handbook".to_owned(),
            "session".to_owned(),
            "work".to_owned(),
        ]),
        bounded_rule: revision(
            "handbook.snapshot-bound.exact-revision-per-family@1.0.0",
            '5',
        ),
        unstable_action,
    }
}

fn fixture_observations() -> Vec<FamilyObservation> {
    repaired_snapshot_record_fixture()["family_observations"]
        .as_array()
        .expect("family observations")
        .iter()
        .map(|observation| FamilyObservation {
            family: observation["family"].as_str().expect("family").to_owned(),
            pre_revision: revision_from_value(&observation["pre_revision"]),
            captured_revision: revision_from_value(&observation["captured_revision"]),
            post_revision: revision_from_value(&observation["post_revision"]),
            source_slot_revisions: observation["source_slot_revisions"]
                .as_array()
                .expect("source slot revisions")
                .iter()
                .map(|slot| SourceSlotObservation {
                    source_slot: slot["source_slot"]
                        .as_str()
                        .expect("source slot")
                        .to_owned(),
                    pre_revision: revision_from_value(&slot["pre_revision"]),
                    captured_revision: revision_from_value(&slot["captured_revision"]),
                    post_revision: revision_from_value(&slot["post_revision"]),
                })
                .collect(),
            bound_evaluation: None,
        })
        .collect()
}

fn revision_from_value(value: &Value) -> super::record::consistency::Revision {
    super::record::consistency::Revision::new(
        value["ref"].as_str().expect("revision ref"),
        value["fingerprint"].as_str().expect("revision fingerprint"),
    )
    .expect("valid revision")
}

fn revision(reference: &str, fill: char) -> super::record::consistency::Revision {
    super::record::consistency::Revision::new(reference, &fingerprint(fill))
        .expect("valid revision")
}

fn captured_slot_revisions(
    observation: &FamilyObservation,
) -> BTreeMap<String, super::record::consistency::Revision> {
    observation
        .source_slot_revisions
        .iter()
        .map(|slot| (slot.source_slot.clone(), slot.captured_revision.clone()))
        .collect()
}

fn assert_unstable(
    policy: &ConsistencyPolicy,
    observations: Vec<FamilyObservation>,
    exclusions: Vec<FamilyExclusion>,
) {
    let derived = derive_consistency(policy, observations, exclusions)
        .expect("unstable records remain descriptive");
    assert_eq!(derived.consistency, SnapshotConsistency::Unstable);
    assert_eq!(derived.admissibility, Admissibility::DiagnosticOnly);
}

fn assert_consistency_error<T: std::fmt::Debug>(
    result: Result<T, super::record::consistency::ConsistencyError>,
    kind: ConsistencyErrorKind,
) {
    assert_eq!(result.expect_err("expected refusal").kind(), kind);
}

fn boundary(
    snapshot_id: &str,
    boundary_sequence: u64,
    trigger: BoundaryTrigger,
    previous_snapshot_id: Option<&str>,
) -> BoundaryRecord {
    BoundaryRecord {
        snapshot_id: snapshot_id.to_owned(),
        repository_id: "repo.example".to_owned(),
        workspace_id: "workspace.example".to_owned(),
        boundary_stream_ref: "orchestration.example".to_owned(),
        boundary_sequence,
        trigger,
        previous_snapshot_id: previous_snapshot_id.map(str::to_owned),
    }
}

fn assert_topology_error<T: std::fmt::Debug>(
    result: Result<T, super::record::consistency::ConsistencyError>,
    kind: ConsistencyErrorKind,
) {
    assert_consistency_error(result, kind);
}

#[test]
fn snapshot_delta_replays_equal_state_and_orders_changes_and_catalog_signals() {
    let policy = valid_policy();
    let catalog = valid_delta_catalog(&policy);
    let (equal_from, equal_to) = delta_snapshots(&policy, &[]);
    let equal = derive_snapshot_delta(
        SnapshotEndpoint::new(&equal_from),
        SnapshotEndpoint::new(&equal_to),
        &catalog,
        &BTreeMap::new(),
    )
    .expect("equal compatible endpoints derive an empty delta");
    assert!(equal.normalized_value()["changes"]
        .as_array()
        .expect("changes")
        .is_empty());
    assert!(equal.normalized_value()["signals"]
        .as_array()
        .expect("signals")
        .is_empty());
    assert!(equal.normalized_value()["rule_evaluations"]
        .as_array()
        .expect("rule evaluations")
        .iter()
        .all(|evaluation| evaluation["outcome"] == "not_matched"));

    let (from, to) = delta_snapshots(&policy, &["evidence", "git", "handbook", "session", "work"]);
    let delta = derive_snapshot_delta(
        SnapshotEndpoint::new(&from),
        SnapshotEndpoint::new(&to),
        &catalog,
        &delta_justifications(),
    )
    .expect("catalog-complete delta");

    let families = delta.normalized_value()["changes"]
        .as_array()
        .expect("changes")
        .iter()
        .map(|change| change["family"].as_str().expect("family"))
        .collect::<Vec<_>>();
    assert_eq!(families, ["evidence", "git", "handbook", "session", "work"]);
    let kinds = delta.normalized_value()["signals"]
        .as_array()
        .expect("signals")
        .iter()
        .map(|signal| signal["kind"].as_str().expect("signal kind"))
        .collect::<Vec<_>>();
    assert_eq!(
        kinds,
        [
            "expected_progress",
            "justified_divergence",
            "unexplained_drift",
            "scope_expansion",
            "execution_inefficiency_signal",
            "planning_inaccuracy_signal",
            "proof_drift",
            "semantic_drift",
            "stale_handoff",
        ]
    );
    assert_eq!(
        delta.normalized_value()["signals"][1]["justification_refs"],
        json!(["decision.hcm_3_4_delta"])
    );
}

#[test]
fn snapshot_delta_distinguishes_reversed_endpoints_and_refuses_invalid_inputs() {
    let policy = valid_policy();
    let catalog = valid_delta_catalog(&policy);
    let (from, to) = delta_snapshots(&policy, &["work"]);
    let forward = derive_snapshot_delta(
        SnapshotEndpoint::new(&from),
        SnapshotEndpoint::new(&to),
        &catalog,
        &BTreeMap::new(),
    )
    .expect("forward delta");
    let reverse = derive_snapshot_delta(
        SnapshotEndpoint::new(&to),
        SnapshotEndpoint::new(&from),
        &catalog,
        &BTreeMap::new(),
    )
    .expect("reverse delta");
    assert_ne!(forward.fingerprint(), reverse.fingerprint());
    assert_ne!(
        forward.normalized_value()["changes"][0]["before_fingerprint"],
        reverse.normalized_value()["changes"][0]["before_fingerprint"]
    );

    let mut unstable_record = repaired_snapshot_record_fixture();
    unstable_record["family_observations"]
        .as_array_mut()
        .expect("family observations")
        .iter_mut()
        .find(|observation| observation["family"] == "git")
        .expect("git observation")["post_revision"] = pair("git.revision.2", '8');
    let unstable = delta_snapshot(&policy, unstable_record);
    assert_delta_error(
        derive_snapshot_delta(
            SnapshotEndpoint::new(&unstable),
            SnapshotEndpoint::new(&to),
            &catalog,
            &BTreeMap::new(),
        ),
        DeltaErrorKind::UnstableEndpoint,
    );

    let mut other_repository = repaired_snapshot_record_fixture();
    other_repository["repository_identity"]["repository_id"] = json!("repo.other");
    let other_repository = delta_snapshot(&policy, other_repository);
    assert_delta_error(
        derive_snapshot_delta(
            SnapshotEndpoint::new(&from),
            SnapshotEndpoint::new(&other_repository),
            &catalog,
            &BTreeMap::new(),
        ),
        DeltaErrorKind::IncompatibleRepository,
    );

    let mut other_workspace = repaired_snapshot_record_fixture();
    other_workspace["repository_identity"]["workspace_id"] = json!("workspace.other");
    let other_workspace = delta_snapshot(&policy, other_workspace);
    assert_delta_error(
        derive_snapshot_delta(
            SnapshotEndpoint::new(&from),
            SnapshotEndpoint::new(&other_workspace),
            &catalog,
            &BTreeMap::new(),
        ),
        DeltaErrorKind::IncompatibleWorkspace,
    );

    let mut policy_variant = repaired_policy_fixture();
    policy_variant["policy_version"] = json!("1.0.1");
    repair_policy_fingerprint(&mut policy_variant);
    let policy_variant = load_policy(&canonical_json(&policy_variant)).expect("variant policy");
    let (_, variant_to) = delta_snapshots(&policy_variant, &["work"]);
    assert_delta_error(
        derive_snapshot_delta(
            SnapshotEndpoint::new(&from),
            SnapshotEndpoint::new(&variant_to),
            &catalog,
            &BTreeMap::new(),
        ),
        DeltaErrorKind::IncompatiblePolicy,
    );

    let mut excluded = repaired_snapshot_record_fixture();
    excluded["family_observations"]
        .as_array_mut()
        .expect("family observations")
        .retain(|observation| observation["family"] != "evidence");
    excluded["excluded_families"] = json!([{"family": "evidence", "reason": "unavailable"}]);
    let excluded = delta_snapshot(&policy, excluded);
    assert_delta_error(
        derive_snapshot_delta(
            SnapshotEndpoint::new(&from),
            SnapshotEndpoint::new(&excluded),
            &catalog,
            &BTreeMap::new(),
        ),
        DeltaErrorKind::UnstableEndpoint,
    );
}

#[test]
fn snapshot_delta_refuses_stale_incomplete_duplicate_and_uncataloged_catalogs() {
    let policy = valid_policy();

    let mut stale = authored_delta_catalog(&policy);
    stale["catalog_fingerprint"] = json!(fingerprint('f'));
    assert_delta_error(
        load_drift_catalog(&canonical_json(&stale)),
        DeltaErrorKind::StaleCatalog,
    );

    let mut missing = authored_delta_catalog(&policy);
    missing["rules"].as_array_mut().expect("rules").pop();
    repair_delta_catalog_fingerprint(&mut missing);
    assert_delta_error(
        load_drift_catalog(&canonical_json(&missing)),
        DeltaErrorKind::MissingCatalogRule,
    );

    let mut duplicate = authored_delta_catalog(&policy);
    let duplicate_rule = duplicate["rules"][0].clone();
    duplicate["rules"]
        .as_array_mut()
        .expect("rules")
        .push(duplicate_rule);
    repair_delta_catalog_fingerprint(&mut duplicate);
    assert_delta_error(
        load_drift_catalog(&canonical_json(&duplicate)),
        DeltaErrorKind::DuplicateCatalogRule,
    );

    let mut uncataloged = authored_delta_catalog(&policy);
    uncataloged["rules"][0]["signal_kind"] = json!("model_reclassification");
    repair_delta_catalog_fingerprint(&mut uncataloged);
    assert_delta_error(
        load_drift_catalog(&canonical_json(&uncataloged)),
        DeltaErrorKind::InvalidCatalogRule,
    );
}

#[test]
fn snapshot_delta_refuses_incomplete_contradictory_and_unjustified_evaluations_and_signals() {
    let policy = valid_policy();
    let catalog = valid_delta_catalog(&policy);
    let (from, to) = delta_snapshots(&policy, &["evidence", "git", "handbook", "session", "work"]);
    assert_delta_error(
        derive_snapshot_delta(
            SnapshotEndpoint::new(&from),
            SnapshotEndpoint::new(&to),
            &catalog,
            &BTreeMap::new(),
        ),
        DeltaErrorKind::MissingJustification,
    );

    let delta = derive_snapshot_delta(
        SnapshotEndpoint::new(&from),
        SnapshotEndpoint::new(&to),
        &catalog,
        &delta_justifications(),
    )
    .expect("complete delta");

    let mut incomplete = delta.normalized_value().clone();
    incomplete["rule_evaluations"]
        .as_array_mut()
        .expect("rule evaluations")
        .remove(0);
    assert_delta_error(
        validate_delta(&catalog, &incomplete),
        DeltaErrorKind::IncompleteEvaluation,
    );

    let mut duplicate = delta.normalized_value().clone();
    let duplicate_evaluation = duplicate["rule_evaluations"][0].clone();
    duplicate["rule_evaluations"]
        .as_array_mut()
        .expect("rule evaluations")
        .push(duplicate_evaluation);
    assert_delta_error(
        validate_delta(&catalog, &duplicate),
        DeltaErrorKind::DuplicateEvaluation,
    );

    let (equal_from, equal_to) = delta_snapshots(&policy, &[]);
    let equal = derive_snapshot_delta(
        SnapshotEndpoint::new(&equal_from),
        SnapshotEndpoint::new(&equal_to),
        &catalog,
        &BTreeMap::new(),
    )
    .expect("equal delta");
    let mut contradictory = equal.normalized_value().clone();
    contradictory["rule_evaluations"][0]["signal_id"] = json!("signal.expected_progress");
    assert_delta_error(
        validate_delta(&catalog, &contradictory),
        DeltaErrorKind::ContradictoryEvaluation,
    );

    let mut uncataloged = delta.normalized_value().clone();
    uncataloged["signals"][0]["kind"] = json!("model_reclassification");
    assert_delta_error(
        validate_delta(&catalog, &uncataloged),
        DeltaErrorKind::UncatalogedSignal,
    );

    let mut unjustified = delta.normalized_value().clone();
    unjustified["signals"]
        .as_array_mut()
        .expect("signals")
        .iter_mut()
        .find(|signal| signal["kind"] == "justified_divergence")
        .expect("justified signal")["justification_refs"] = json!([]);
    assert_delta_error(
        validate_delta(&catalog, &unjustified),
        DeltaErrorKind::MissingJustification,
    );
}

fn valid_delta_catalog(policy: &super::policy::SnapshotCapturePolicy) -> DriftCatalog {
    load_drift_catalog(&canonical_json(&authored_delta_catalog(policy))).expect("valid catalog")
}

fn authored_delta_catalog(policy: &super::policy::SnapshotCapturePolicy) -> Value {
    let mut value: Value =
        serde_json::from_slice(DELTA_CATALOG_FIXTURE).expect("delta catalog fixture");
    value["admitted_policies"] = json!([exact_pair(
        policy.reference(),
        policy.fingerprint().as_str()
    )]);
    repair_delta_catalog_fingerprint(&mut value);
    value
}

fn repair_delta_catalog_fingerprint(value: &mut Value) {
    let mut preimage = value.clone();
    preimage
        .as_object_mut()
        .expect("catalog object")
        .remove("catalog_fingerprint");
    value["catalog_fingerprint"] = json!(DefinitionFingerprint::from_json_value(&preimage)
        .expect("catalog fingerprint")
        .to_string());
}

fn delta_snapshots(
    policy: &super::policy::SnapshotCapturePolicy,
    changed_families: &[&str],
) -> (
    super::record::ContextMemorySnapshot,
    super::record::ContextMemorySnapshot,
) {
    let from = delta_snapshot(policy, repaired_snapshot_record_fixture());
    let mut to = repaired_snapshot_record_fixture();
    to["snapshot_id"] = json!("snap_session_end_0002");
    to["capture"]["trigger"] = json!("session_end");
    to["capture"]["started_at"] = json!("2026-08-06T04:01:00Z");
    to["capture"]["completed_at"] = json!("2026-08-06T04:01:05Z");
    to["boundary_stream_ref"] = json!("orchestration.example.end");
    to["boundary_sequence"] = json!(2);
    for family in changed_families {
        let observation = to["family_observations"]
            .as_array_mut()
            .expect("family observations")
            .iter_mut()
            .find(|observation| observation["family"] == *family)
            .expect("selected family");
        observation["payload"]["delta_marker"] = json!(format!("{family}_changed"));
    }
    repair_snapshot_payload_fingerprints(&mut to);
    (from, delta_snapshot(policy, to))
}

fn delta_snapshot(
    policy: &super::policy::SnapshotCapturePolicy,
    mut record: Value,
) -> super::record::ContextMemorySnapshot {
    let mut capture_input = valid_capture_input(policy);
    capture_input["trigger"] = record["capture"]["trigger"].clone();
    record["capture"]["input"]["policy"] =
        exact_pair(policy.reference(), policy.fingerprint().as_str());
    record["capture"]["input"]["trigger"] = record["capture"]["trigger"].clone();
    let capture = validate_capture_input(policy, &canonical_json(&capture_input))
        .expect("valid capture input");
    build_snapshot(policy, &capture, &[], &canonical_json(&record)).expect("valid delta snapshot")
}

fn repair_snapshot_payload_fingerprints(value: &mut Value) {
    for observation in value["family_observations"]
        .as_array_mut()
        .expect("family observations")
    {
        let mut payload = observation["payload"].clone();
        normalize_fixture_payload(&mut payload, None);
        observation["payload_fingerprint"] =
            json!(DefinitionFingerprint::from_json_value(&payload)
                .expect("payload fingerprint")
                .to_string());
    }
}

fn delta_justifications() -> BTreeMap<String, Vec<String>> {
    BTreeMap::from([(
        "justified_divergence".to_owned(),
        vec!["decision.hcm_3_4_delta".to_owned()],
    )])
}

fn assert_delta_error<T: std::fmt::Debug>(result: Result<T, DeltaError>, kind: DeltaErrorKind) {
    assert_eq!(result.expect_err("expected delta refusal").kind(), kind);
}

#[test]
fn redaction_fails_closed_for_sensitive_unmatched_and_unknown_surfaces() {
    let fixture =
        load_redaction_retention_fixture(REDACTION_RETENTION_FIXTURE).expect("fail-closed fixture");
    assert_eq!(fixture.redaction.unmatched_action, RedactionAction::Omit);

    for surface in [
        SurfaceClassification::Secret,
        SurfaceClassification::UnrestrictedEnvironment,
        SurfaceClassification::SecretFile,
        SurfaceClassification::RawCommand,
        SurfaceClassification::UnrestrictedDiff,
    ] {
        assert_eq!(
            evaluate_redaction(
                surface,
                MatcherOutcome::Matched(vec![RedactionAction::FingerprintOnly]),
            )
            .expect("sensitive floor resolves"),
            RedactionAction::Omit
        );
        assert_eq!(
            evaluate_redaction(surface, MatcherOutcome::KnownUnmatched)
                .expect("sensitive unmatched surface resolves"),
            RedactionAction::Omit
        );
    }

    assert_eq!(
        evaluate_redaction(SurfaceClassification::Known, MatcherOutcome::KnownUnmatched)
            .expect("known unmatched surface"),
        RedactionAction::Omit
    );
    assert_eq!(
        evaluate_redaction(SurfaceClassification::Known, MatcherOutcome::MatcherFailed)
            .expect("failed matcher"),
        RedactionAction::Omit
    );
    assert_eq!(
        evaluate_redaction(SurfaceClassification::Unknown, MatcherOutcome::Unknown)
            .expect("unclassifiable surface"),
        RedactionAction::Omit
    );
    assert_eq!(
        evaluate_redaction(
            SurfaceClassification::Unknown,
            MatcherOutcome::Matched(vec![RedactionAction::ArtifactRefOnly]),
        )
        .expect("unclassifiable surface cannot retain a pointer"),
        RedactionAction::Omit
    );
}

#[test]
fn redaction_resolves_overlap_and_pointer_coverage_without_hidden_reads() {
    assert_eq!(
        evaluate_redaction(
            SurfaceClassification::Known,
            MatcherOutcome::Matched(vec![
                RedactionAction::FingerprintOnly,
                RedactionAction::FingerprintOnly,
            ]),
        )
        .expect("identical non-omit overlap"),
        RedactionAction::FingerprintOnly
    );
    assert_eq!(
        evaluate_redaction(
            SurfaceClassification::Known,
            MatcherOutcome::Matched(vec![
                RedactionAction::FingerprintOnly,
                RedactionAction::Omit
            ]),
        )
        .expect("omit overlap"),
        RedactionAction::Omit
    );
    assert_redaction_error(
        evaluate_redaction(
            SurfaceClassification::Known,
            MatcherOutcome::Matched(vec![
                RedactionAction::FingerprintOnly,
                RedactionAction::ArtifactRefOnly,
            ]),
        ),
        RedactionErrorKind::IncomparableNonOmitActions,
    );

    assert!(validate_disposition("/payload/secret", RedactionAction::Omit, None).is_ok());
    for action in [
        RedactionAction::FingerprintOnly,
        RedactionAction::ArtifactRefOnly,
        RedactionAction::RedactedSummary,
    ] {
        assert!(validate_disposition("/payload/secret", action, Some("/payload/metadata")).is_ok());
        assert_redaction_error(
            validate_disposition("/payload/secret", action, None),
            RedactionErrorKind::RetainedPointerMismatch,
        );
    }
    assert_redaction_error(
        validate_disposition(
            "/payload/secret",
            RedactionAction::Omit,
            Some("/payload/metadata"),
        ),
        RedactionErrorKind::RetainedPointerMismatch,
    );
    assert_redaction_error(
        validate_disposition(
            "/payload/secret",
            RedactionAction::FingerprintOnly,
            Some("/payload/secret"),
        ),
        RedactionErrorKind::RetainedPointerInsideOriginal,
    );
    assert_redaction_error(
        validate_disposition(
            "/payload/secret",
            RedactionAction::FingerprintOnly,
            Some("/payload/secret/token"),
        ),
        RedactionErrorKind::RetainedPointerInsideOriginal,
    );

    assert_eq!(
        classify_pointer_read("/payload/secret", "/payload/secret/token")
            .expect("covered child pointer"),
        PointerRead::Redacted
    );
    assert_eq!(
        classify_pointer_read("/payload/secret", "/payload/secret_metadata")
            .expect("shared string prefix is independent"),
        PointerRead::IndependentlyClassified
    );
    assert_eq!(
        classify_pointer_read("/payload/secret", "/payload/metadata")
            .expect("retained pointer is independent"),
        PointerRead::IndependentlyClassified
    );
}

#[test]
fn retention_requires_one_rule_and_refuses_held_referenced_or_unexpired_deletion() {
    let fixture = load_redaction_retention_fixture(REDACTION_RETENTION_FIXTURE)
        .expect("valid retention fixture");
    for tuple in &fixture.retention.allowed_tuples {
        let rule = resolve_retention(&fixture.retention.rules, tuple).expect("covered tuple");
        assert!(fixture
            .retention
            .rules
            .iter()
            .any(|candidate| candidate.rule_id == rule.rule_id));
    }

    let tuple = fixture.retention.allowed_tuples[0].clone();
    let mut overlapping = fixture.retention.rules.clone();
    overlapping.push(overlapping[0].clone());
    assert_redaction_error(
        resolve_retention(&overlapping, &tuple),
        RedactionErrorKind::RetentionOverlap,
    );
    let mut uncovered = fixture.retention.rules.clone();
    uncovered.retain(|rule| rule.memory_horizon != tuple.memory_horizon);
    assert_redaction_error(
        resolve_retention(&uncovered, &tuple),
        RedactionErrorKind::RetentionUncovered,
    );

    assert_redaction_error(
        approve_deletion(DeletionGuard {
            held: true,
            referenced: false,
            unexpired_floor: false,
        }),
        RedactionErrorKind::HeldRecord,
    );
    assert_redaction_error(
        approve_deletion(DeletionGuard {
            held: false,
            referenced: true,
            unexpired_floor: false,
        }),
        RedactionErrorKind::ReferencedRecord,
    );
    assert_redaction_error(
        approve_deletion(DeletionGuard {
            held: false,
            referenced: false,
            unexpired_floor: true,
        }),
        RedactionErrorKind::UnexpiredRetentionFloor,
    );
    assert!(approve_deletion(DeletionGuard {
        held: false,
        referenced: false,
        unexpired_floor: false,
    })
    .is_ok());
}

#[test]
fn dedupe_and_reviewed_compaction_preserve_immutable_record_identity_and_bytes() {
    let records = vec![
        ImmutableRecord {
            record_id: "snapshot-002".to_owned(),
            record_bytes: b"record bytes two".to_vec(),
            payload_bytes: b"shared normalized payload".to_vec(),
        },
        ImmutableRecord {
            record_id: "snapshot-001".to_owned(),
            record_bytes: b"record bytes one".to_vec(),
            payload_bytes: b"shared normalized payload".to_vec(),
        },
    ];
    let before = records.clone();
    let bindings = deduplicate_payloads(&records).expect("payload sharing");
    let mut reversed = records.clone();
    reversed.reverse();
    assert_eq!(
        bindings,
        deduplicate_payloads(&reversed).expect("replay sharing")
    );
    assert_eq!(bindings[0].record_id, "snapshot-001");
    assert_eq!(bindings[1].record_id, "snapshot-002");
    assert_eq!(bindings[0].payload_ref, bindings[1].payload_ref);
    assert_eq!(bindings[0].record_bytes, before[1].record_bytes);
    assert_eq!(bindings[1].record_bytes, before[0].record_bytes);

    assert_redaction_error(
        create_compaction(false, &records),
        RedactionErrorKind::CompactionReviewRequired,
    );
    let compaction = create_compaction(true, &records).expect("reviewed additive compaction");
    assert_eq!(records, before);
    assert_eq!(
        compaction.source_record_ids,
        vec!["snapshot-001", "snapshot-002"]
    );
    assert_eq!(compaction.shared_payload_refs.len(), 1);
}

fn assert_redaction_error<T: std::fmt::Debug>(
    result: Result<T, RedactionError>,
    kind: RedactionErrorKind,
) {
    assert_eq!(
        result.expect_err("expected fail-closed refusal").kind(),
        kind
    );
}
