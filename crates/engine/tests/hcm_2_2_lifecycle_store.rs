#[path = "support/hcm_2_2_committed_charter.rs"]
mod hcm_2_2_committed_charter;

use handbook_engine::{
    charter_lifecycle_event_commit_marker, charter_lifecycle_state_fingerprint,
    classify_lifecycle_recovery, CharterLifecycleEventDispositionV1, CharterLifecycleEventIntentV1,
    CharterLifecycleRecoveryActionV1, CharterLifecycleRecoveryInputsV1, CharterLifecycleState,
    CharterLifecycleStoreErrorKindV1, CharterLifecycleStoreV1, DefinitionFingerprint,
    LineageRecordClassV1, TrustedLineageStoreV1,
};
use serde_json::Value;

const VECTORS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/runtime-record-fingerprint-vectors-v1.0.json"
));
const CANONICAL_CHARTER: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));

fn fingerprint(character: char) -> String {
    format!("sha256:{}", character.to_string().repeat(64))
}

fn vector(class: &str) -> Value {
    let fixture: Value = serde_json::from_slice(VECTORS).expect("runtime vectors");
    fixture["vectors"]
        .as_array()
        .expect("vector rows")
        .iter()
        .find(|row| row["record_class"] == class)
        .expect("runtime vector class")["record"]
        .clone()
}

fn canonical(value: &Value) -> Vec<u8> {
    serde_json_canonicalizer::to_vec(value).expect("canonical record")
}

fn resign(
    value: &mut Value,
    id_field: &str,
    fingerprint_field: &str,
    prefix: &str,
    audit_only: &[&str],
) {
    let mut preimage = value.clone();
    let object = preimage.as_object_mut().expect("record object");
    object.remove(id_field);
    object.remove(fingerprint_field);
    for field in audit_only {
        object.remove(*field);
    }
    let fingerprint = DefinitionFingerprint::from_json_value(&preimage)
        .expect("record fingerprint")
        .to_string();
    let id = format!(
        "{prefix}_{}",
        fingerprint
            .strip_prefix("sha256:")
            .expect("sha256 fingerprint")
    );
    value[id_field] = Value::String(id);
    value[fingerprint_field] = Value::String(fingerprint);
}

fn seed_trigger_evidence(
    repo: &std::path::Path,
    basis_fingerprint: &str,
    event_kind: &str,
) -> (String, String) {
    let mut evidence = vector("trigger-evidence");
    evidence["basis_canonical_fingerprint"] = Value::String(basis_fingerprint.to_owned());
    evidence["event_kind"] = Value::String(event_kind.to_owned());
    resign(
        &mut evidence,
        "event_id",
        "evidence_fingerprint",
        "trigger-evidence",
        &[],
    );
    let appended = TrustedLineageStoreV1::new(repo)
        .append_record(LineageRecordClassV1::TriggerEvidence, &canonical(&evidence))
        .expect("trigger evidence record");
    (appended.relative_ref, appended.fingerprint)
}

fn committed_repo() -> tempfile::TempDir {
    let repo = tempfile::tempdir().expect("repository");
    hcm_2_2_committed_charter::promote_committed_charter(
        repo.path(),
        CANONICAL_CHARTER,
        "lifecycle-authority-create",
    );
    repo
}

#[test]
fn lifecycle_state_fingerprint_preserves_the_exact_committed_head_order() {
    let forward = charter_lifecycle_state_fingerprint(
        "handbook.lifecycle.constitutional-review-lock@1.0.0",
        &fingerprint('a'),
        "project_authority",
        &fingerprint('b'),
        CharterLifecycleState::ReassessmentRequired,
        &[fingerprint('d'), fingerprint('c')],
    )
    .expect("state fingerprint");
    let reversed = charter_lifecycle_state_fingerprint(
        "handbook.lifecycle.constitutional-review-lock@1.0.0",
        &fingerprint('a'),
        "project_authority",
        &fingerprint('b'),
        CharterLifecycleState::ReassessmentRequired,
        &[fingerprint('c'), fingerprint('d')],
    )
    .expect("state fingerprint");
    assert_ne!(forward, reversed);
}

#[test]
fn lifecycle_commit_marker_is_the_exact_raw_intent_hash_payload() {
    let marker = charter_lifecycle_event_commit_marker(br#"{"event_id":"event-1"}"#);
    assert_eq!(marker.len(), 72);
    assert_eq!(&marker[..7], b"sha256:");
    assert_eq!(marker.last(), Some(&b'\n'));
}

#[test]
fn lifecycle_recovery_precedence_is_total_for_admitted_observations() {
    use CharterLifecycleRecoveryActionV1::{
        Finalize, InstallBothAndCommit, InstallMissingAndCommit, PreserveAndRefuse,
        RollForwardCommit, RollbackOwnedState,
    };
    let rows = [
        (false, false, false, false, false, RollbackOwnedState),
        (false, false, true, true, false, InstallBothAndCommit),
        (false, false, true, false, false, RollbackOwnedState),
        (true, false, false, true, false, InstallMissingAndCommit),
        (true, false, false, false, false, RollbackOwnedState),
        (true, true, false, false, false, RollForwardCommit),
        (true, true, false, false, true, Finalize),
        (true, false, false, false, true, PreserveAndRefuse),
    ];
    for (
        observation_final,
        transition_final,
        observation_staged,
        transition_staged,
        marker,
        expected,
    ) in rows
    {
        assert_eq!(
            classify_lifecycle_recovery(CharterLifecycleRecoveryInputsV1 {
                observation_final,
                transition_final,
                observation_staged,
                transition_staged,
                matching_commit_marker: marker,
            }),
            expected
        );
    }
}

#[test]
fn standalone_trigger_commits_reassessment_and_exact_replay_is_idempotent() {
    let repo = committed_repo();
    let lifecycle = CharterLifecycleStoreV1::new(repo.path());
    let initial = lifecycle.observe().unwrap().unwrap();
    let (evidence_ref, evidence_fingerprint) = seed_trigger_evidence(
        repo.path(),
        &initial.canonical_fingerprint,
        "production_posture_changed",
    );
    let intent = CharterLifecycleEventIntentV1::TriggerEvidence {
        evidence_ref,
        evidence_fingerprint,
    };
    let committed = lifecycle
        .record_event(intent.clone())
        .expect("event commit");
    assert_eq!(
        committed.disposition,
        CharterLifecycleEventDispositionV1::Committed
    );
    assert_eq!(committed.state, CharterLifecycleState::ReassessmentRequired);
    assert_eq!(
        committed.reopened_coverage_ids,
        ["operational_reality.production_state"]
    );
    let replay = lifecycle.record_event(intent).expect("idempotent replay");
    assert_eq!(
        replay.disposition,
        CharterLifecycleEventDispositionV1::Replayed
    );
    assert_eq!(replay.transition_ref, committed.transition_ref);
}

#[test]
fn stale_basis_and_unknown_standalone_trigger_refuse_closed() {
    let repo = committed_repo();
    let lifecycle = CharterLifecycleStoreV1::new(repo.path());
    let initial = lifecycle.observe().unwrap().unwrap();
    let (stale_ref, stale_fingerprint) =
        seed_trigger_evidence(repo.path(), &fingerprint('f'), "production_posture_changed");
    assert_eq!(
        lifecycle
            .record_event(CharterLifecycleEventIntentV1::TriggerEvidence {
                evidence_ref: stale_ref,
                evidence_fingerprint: stale_fingerprint,
            })
            .expect_err("stale basis")
            .kind(),
        CharterLifecycleStoreErrorKindV1::StaleBasis
    );
    let (unknown_ref, unknown_fingerprint) = seed_trigger_evidence(
        repo.path(),
        &initial.canonical_fingerprint,
        "charter_amendment_proposed",
    );
    assert_eq!(
        lifecycle
            .record_event(CharterLifecycleEventIntentV1::TriggerEvidence {
                evidence_ref: unknown_ref,
                evidence_fingerprint: unknown_fingerprint,
            })
            .expect_err("unknown trigger")
            .kind(),
        CharterLifecycleStoreErrorKindV1::UnknownTrigger
    );
}
