use super::{
    charter_promotion_commit_marker, CharterAuthorityTransactionServiceV1,
    CharterPromotionErrorKindV1, CharterPromotionFaultPointV1, CharterPromotionRequestV1,
};
use crate::{LineageRecordClassV1, TrustedLineageStoreV1};
use serde_json::Value;
use std::sync::{Arc, Barrier};

const VECTORS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/runtime-record-fingerprint-vectors-v1.0.json"
));
const CANONICAL_CHARTER: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));

fn vector(class: &str) -> Value {
    let fixture: Value = serde_json::from_slice(VECTORS).unwrap();
    fixture["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["record_class"] == class)
        .unwrap()["record"]
        .clone()
}

fn canonical(value: &Value) -> Vec<u8> {
    serde_json_canonicalizer::to_vec(value).unwrap()
}

fn resign(
    value: &mut Value,
    id_field: &str,
    fingerprint_field: &str,
    prefix: &str,
    audit_only: &[&str],
) {
    let mut preimage = value.clone();
    let object = preimage.as_object_mut().unwrap();
    object.remove(id_field);
    object.remove(fingerprint_field);
    for field in audit_only {
        object.remove(*field);
    }
    let fingerprint = crate::DefinitionFingerprint::from_json_value(&preimage)
        .unwrap()
        .to_string();
    let id = format!("{prefix}_{}", fingerprint.strip_prefix("sha256:").unwrap());
    value[id_field] = Value::String(id);
    value[fingerprint_field] = Value::String(fingerprint);
}

fn seed_lineage(repo: &std::path::Path) {
    let store = TrustedLineageStoreV1::new(repo);
    for (class, name) in [
        (
            LineageRecordClassV1::AuthenticatorMakeCredentialResponse,
            "authenticator-make-credential-response",
        ),
        (
            LineageRecordClassV1::AuthenticatorRegistration,
            "authenticator-registration",
        ),
        (LineageRecordClassV1::RegistryState, "registry-state"),
        (
            LineageRecordClassV1::RegistryTransition,
            "registry-transition",
        ),
        (
            LineageRecordClassV1::AuthenticatorGetAssertionResponse,
            "authenticator-get-assertion-response",
        ),
        (
            LineageRecordClassV1::AuthenticatorAssertion,
            "authenticator-assertion",
        ),
        (LineageRecordClassV1::Intake, "intake"),
        (LineageRecordClassV1::Candidate, "candidate"),
        (LineageRecordClassV1::Approval, "approval"),
    ] {
        store
            .append_record(class, &canonical(&vector(name)))
            .unwrap();
    }
    std::fs::write(
        repo.join(".handbook/state/candidates/candidate.yaml"),
        CANONICAL_CHARTER,
    )
    .unwrap();
}

fn request(transaction_id: &str) -> CharterPromotionRequestV1 {
    request_with_canonical(transaction_id, CANONICAL_CHARTER.to_vec())
}

fn request_with_canonical(
    transaction_id: &str,
    canonical_bytes: Vec<u8>,
) -> CharterPromotionRequestV1 {
    let canonical_fingerprint =
        crate::DefinitionFingerprint::from_bytes(&canonical_bytes).to_string();

    let mut promotion = vector("promotion");
    promotion["canonical_artifact_fingerprint"] = Value::String(canonical_fingerprint);
    resign(
        &mut promotion,
        "promotion_id",
        "promotion_fingerprint",
        "promotion",
        &[],
    );
    let promotion_id = promotion["promotion_id"].as_str().unwrap().to_owned();

    let mut lifecycle = vector("lifecycle-transition");
    lifecycle["new_observation_refs"] = Value::Array(Vec::new());
    lifecycle["active_observation_refs"] = Value::Array(Vec::new());
    lifecycle["result_state"] = Value::String("current".to_owned());
    lifecycle["clearance_promotion_ref"] = Value::String(format!("promotions/{promotion_id}.json"));
    resign(
        &mut lifecycle,
        "transition_id",
        "transition_fingerprint",
        "lifecycle-transition",
        &["transitioned_at_utc"],
    );

    CharterPromotionRequestV1 {
        transaction_id: transaction_id.to_owned(),
        canonical_bytes,
        promotion_record_bytes: canonical(&promotion),
        lifecycle_transition_bytes: canonical(&lifecycle),
    }
}

#[test]
fn raw_intent_hash_marker_is_exactly_72_ascii_bytes() {
    let marker = charter_promotion_commit_marker(br#"{"transaction_id":"tx-1"}"#);
    assert_eq!(marker.len(), 72);
    assert_eq!(marker[0..7], *b"sha256:");
    assert_eq!(marker[71], b'\n');
    assert!(marker[7..71]
        .iter()
        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte)));
}

#[test]
fn promotion_is_the_single_commit_point_for_canonical_and_lifecycle_authority() {
    let repo = tempfile::tempdir().unwrap();
    seed_lineage(repo.path());
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    let request = request("promotion-create-001");

    let committed = service.promote(request.clone()).expect("promotion commit");
    let observed = service
        .read_committed_charter()
        .expect("committed read")
        .expect("canonical exists");

    assert_eq!(observed.canonical_bytes, request.canonical_bytes);
    assert_eq!(observed.promotion_ref, committed.promotion_ref);
    assert_eq!(
        observed.lifecycle_transition_ref,
        committed.lifecycle_transition_ref
    );
    assert_eq!(
        std::fs::read(committed.committed_marker_path)
            .unwrap()
            .len(),
        72
    );
}

#[test]
fn reader_recovers_before_observation_and_never_exposes_partial_authority() {
    let repo = tempfile::tempdir().unwrap();
    seed_lineage(repo.path());
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    let request = request("promotion-crash-001");

    let error = service
        .promote_with_fault_for_testing(
            request,
            CharterPromotionFaultPointV1::AfterCanonicalInstalled,
        )
        .unwrap_err();
    assert_eq!(error.kind(), CharterPromotionErrorKindV1::InjectedFault);

    assert!(service.read_committed_charter().unwrap().is_none());
    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
}

#[test]
fn complete_records_without_marker_roll_forward_during_recovery() {
    let repo = tempfile::tempdir().unwrap();
    seed_lineage(repo.path());
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    let request = request("promotion-crash-002");

    let error = service
        .promote_with_fault_for_testing(
            request.clone(),
            CharterPromotionFaultPointV1::AfterRecordsInstalled,
        )
        .unwrap_err();
    assert_eq!(error.kind(), CharterPromotionErrorKindV1::InjectedFault);

    let observed = service
        .read_committed_charter()
        .unwrap()
        .expect("recovery rolls forward complete exact records");
    assert_eq!(observed.canonical_bytes, request.canonical_bytes);
}

#[test]
fn concurrent_create_promotions_publish_only_one_authority() {
    let repo = tempfile::tempdir().unwrap();
    seed_lineage(repo.path());
    let service = Arc::new(CharterAuthorityTransactionServiceV1::new(repo.path()));
    let barrier = Arc::new(Barrier::new(3));
    let mut threads = Vec::new();
    for id in ["promotion-race-a", "promotion-race-b"] {
        let service = Arc::clone(&service);
        let barrier = Arc::clone(&barrier);
        threads.push(std::thread::spawn(move || {
            barrier.wait();
            service.promote(request(id))
        }));
    }
    barrier.wait();
    let results = threads
        .into_iter()
        .map(|thread| thread.join().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert!(service.read_committed_charter().unwrap().is_some());
}

#[test]
fn substitution_stale_basis_and_transaction_replay_refuse_closed() {
    let repo = tempfile::tempdir().unwrap();
    seed_lineage(repo.path());
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());

    let mut substituted = request("promotion-substitution");
    let mut promotion: Value = serde_json::from_slice(&substituted.promotion_record_bytes).unwrap();
    promotion["candidate_fingerprint"] = Value::String(format!("sha256:{}", "f".repeat(64)));
    resign(
        &mut promotion,
        "promotion_id",
        "promotion_fingerprint",
        "promotion",
        &[],
    );
    substituted.promotion_record_bytes = canonical(&promotion);
    assert!(service.promote(substituted).is_err());
    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());

    let original = request("promotion-replay");
    service.promote(original.clone()).unwrap();
    assert_eq!(
        service.promote(original).unwrap_err().kind(),
        CharterPromotionErrorKindV1::Conflict
    );
    assert_eq!(
        service
            .promote(request("promotion-stale-basis"))
            .unwrap_err()
            .kind(),
        CharterPromotionErrorKindV1::BasisMismatch
    );
}

#[test]
fn unequal_preexisting_record_refuses_before_canonical_delta() {
    let repo = tempfile::tempdir().unwrap();
    seed_lineage(repo.path());
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    let request = request("promotion-unequal-final");
    let promotion: Value = serde_json::from_slice(&request.promotion_record_bytes).unwrap();
    let promotion_id = promotion["promotion_id"].as_str().unwrap();
    let partition = repo.path().join(".handbook/state/promotions");
    std::fs::create_dir_all(&partition).unwrap();
    std::fs::write(partition.join(format!("{promotion_id}.json")), b"unequal").unwrap();

    assert!(service.promote(request).is_err());
    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
    assert!(!repo
        .path()
        .join(".handbook/state/transactions/promotions")
        .exists());
}

#[test]
fn marker_mismatch_is_never_observed_as_committed_authority() {
    let repo = tempfile::tempdir().unwrap();
    seed_lineage(repo.path());
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    let committed = service.promote(request("promotion-marker-tamper")).unwrap();
    std::fs::write(&committed.committed_marker_path, vec![b'0'; 72]).unwrap();

    assert_eq!(
        service.read_committed_charter().unwrap_err().kind(),
        CharterPromotionErrorKindV1::DurabilityViolation
    );
}

#[test]
fn amendment_requires_and_commits_the_exact_prior_basis_chain() {
    let repo = tempfile::tempdir().unwrap();
    seed_lineage(repo.path());
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    service
        .promote(request("promotion-before-amendment"))
        .unwrap();
    let expected = crate::DefinitionFingerprint::from_bytes(CANONICAL_CHARTER).to_string();

    let decisions = crate::resolve_shipped_profile_decisions(env!("CARGO_MANIFEST_DIR")).unwrap();
    let mut charter = crate::parse_canonical_charter(&decisions, CANONICAL_CHARTER).unwrap();
    charter.project.name = "Boundary Contract Amended".to_owned();
    let amended_bytes = crate::serialize_canonical_charter(&decisions, &charter).unwrap();
    let store = TrustedLineageStoreV1::new(repo.path());

    let mut intake = vector("intake");
    intake["basis_artifact_fingerprint"] = Value::String(expected.clone());
    resign(
        &mut intake,
        "intake_record_id",
        "record_fingerprint",
        "intake",
        &["finalized_at_utc"],
    );
    let intake_result = store
        .append_record(LineageRecordClassV1::Intake, &canonical(&intake))
        .unwrap();

    let content_fingerprint = crate::DefinitionFingerprint::from_bytes(&amended_bytes).to_string();
    let content_ref = format!(
        "candidate-content/charter_{}.yaml",
        content_fingerprint.strip_prefix("sha256:").unwrap()
    );
    let content_path = repo.path().join(".handbook/state").join(&content_ref);
    std::fs::create_dir_all(content_path.parent().unwrap()).unwrap();
    std::fs::write(&content_path, &amended_bytes).unwrap();
    let mut candidate = vector("candidate");
    candidate["basis_artifact_fingerprint"] = Value::String(expected.clone());
    candidate["intake_record_ref"] = Value::String(intake_result.relative_ref);
    candidate["normalized_content_ref"] = Value::String(content_ref);
    resign(
        &mut candidate,
        "candidate_id",
        "candidate_fingerprint",
        "candidate",
        &[],
    );
    let candidate_result = store
        .append_record(LineageRecordClassV1::Candidate, &canonical(&candidate))
        .unwrap();

    let mut approval = vector("approval");
    approval["basis_artifact_fingerprint"] = Value::String(expected.clone());
    approval["candidate_ref"] = Value::String(candidate_result.relative_ref.clone());
    approval["candidate_fingerprint"] = Value::String(candidate_result.fingerprint.clone());
    resign(
        &mut approval,
        "approval_id",
        "approval_fingerprint",
        "approval",
        &[],
    );
    let approval_result = store
        .persist_approval_record(&canonical(&approval))
        .unwrap();

    let mut amendment = request_with_canonical("promotion-amendment", amended_bytes.clone());
    let mut promotion: Value = serde_json::from_slice(&amendment.promotion_record_bytes).unwrap();
    promotion["basis_artifact_fingerprint"] = Value::String(expected.clone());
    promotion["expected_current_artifact_fingerprint"] = Value::String(expected);
    promotion["candidate_ref"] = Value::String(candidate_result.relative_ref);
    promotion["candidate_fingerprint"] = Value::String(candidate_result.fingerprint);
    promotion["approval_refs"] = Value::Array(vec![Value::String(approval_result.relative_ref)]);
    resign(
        &mut promotion,
        "promotion_id",
        "promotion_fingerprint",
        "promotion",
        &[],
    );
    let promotion_ref = format!(
        "promotions/{}.json",
        promotion["promotion_id"].as_str().unwrap()
    );
    amendment.promotion_record_bytes = canonical(&promotion);
    let mut lifecycle: Value =
        serde_json::from_slice(&amendment.lifecycle_transition_bytes).unwrap();
    lifecycle["clearance_promotion_ref"] = Value::String(promotion_ref);
    resign(
        &mut lifecycle,
        "transition_id",
        "transition_fingerprint",
        "lifecycle-transition",
        &["transitioned_at_utc"],
    );
    amendment.lifecycle_transition_bytes = canonical(&lifecycle);

    service.promote(amendment).unwrap();
    assert_eq!(
        service
            .read_committed_charter()
            .unwrap()
            .unwrap()
            .canonical_bytes,
        amended_bytes
    );
}
