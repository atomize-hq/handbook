use super::{
    charter_promotion_commit_marker, CharterAuthorityTransactionServiceV1,
    CharterPromotionErrorKindV1, CharterPromotionFaultPointV1, CharterPromotionRequestV1,
    RetainedPromotionAuthorityV1,
};
use crate::{
    evaluate_charter_intake, CharterAcquisitionMode, CharterCoverageSubmission,
    CharterIntakeConsumer, CharterIntakeEnvelope, CharterIntakeSourceKind, LineageRecordClassV1,
    TrustedLineageStoreV1,
};
use serde_json::{json, Value};
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

pub(super) fn seed_lineage(repo: &std::path::Path) {
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
    ] {
        store
            .append_record(class, &canonical(&vector(name)))
            .unwrap();
    }
    let content_fingerprint =
        crate::DefinitionFingerprint::from_bytes(CANONICAL_CHARTER).to_string();
    let content_ref = format!(
        "candidate-content/charter_{}.yaml",
        content_fingerprint.strip_prefix("sha256:").unwrap()
    );
    let content_path = repo.join(".handbook/state").join(&content_ref);
    std::fs::create_dir_all(content_path.parent().unwrap()).unwrap();
    std::fs::write(&content_path, CANONICAL_CHARTER).unwrap();

    let evaluation = runtime_candidate_evaluation();
    assert_eq!(evaluation.normalized_content, CANONICAL_CHARTER);
    let intake = serde_json::to_value(&evaluation.intake).unwrap();
    store
        .append_record(LineageRecordClassV1::Intake, &canonical(&intake))
        .unwrap();
    let intake_ref = format!(
        "intake-records/{}.json",
        intake["intake_record_id"].as_str().unwrap()
    );
    let intake_fingerprint = intake["record_fingerprint"].as_str().unwrap();
    let mut candidate = vector("candidate");
    candidate["intake_record_ref"] = Value::String(intake_ref.clone());
    candidate["normalized_content_ref"] = Value::String(content_ref.clone());
    candidate["field_sources"] =
        serde_json::to_value(evaluation.candidate_subject.field_sources).unwrap();
    let mut subject = candidate.clone();
    let subject_object = subject.as_object_mut().unwrap();
    for field in [
        "candidate_id",
        "candidate_fingerprint",
        "candidate_subject_fingerprint",
        "validation_result_binding",
    ] {
        subject_object.remove(field);
    }
    let subject_fingerprint = crate::DefinitionFingerprint::from_json_value(&subject)
        .unwrap()
        .to_string();
    candidate["candidate_subject_fingerprint"] = Value::String(subject_fingerprint.clone());

    let mut validation = json!({
        "schema_id": "handbook.lifecycle-validation-result",
        "schema_version": "1.0",
        "validation_result_id": "pending",
        "candidate_subject_fingerprint": subject_fingerprint,
        "intake_record_ref": intake_ref,
        "intake_record_fingerprint": intake_fingerprint,
        "normalized_content_ref": content_ref,
        "normalized_content_fingerprint": content_fingerprint,
        "target_instance_id": "project_authority",
        "canonical_artifact_ref": ".handbook/project/charter.yaml",
        "basis_artifact_fingerprint": null,
        "observed_current_artifact_fingerprint": null,
        "profile_ref": "handbook.profile.shipped-root@1.1.0",
        "resolved_profile_fingerprint": "sha256:6a7b41befa77b999b9ee20f513636051726a8401a81bf2f369501e8f3dd4fa74",
        "resolved_definitions": crate::charter_lifecycle_validation::definition_bindings(),
        "lifecycle_policy_ref": "handbook.lifecycle.constitutional-review-lock@1.0.0",
        "lifecycle_policy_fingerprint": "sha256:88caafb9caaf137647c42a91cd2762ac0871e0a20e2a1844c2c0076d5fb43cc3",
        "lifecycle_head_ref": null,
        "lifecycle_head_fingerprint": null,
        "lifecycle_state": "absent",
        "lifecycle_state_fingerprint": null,
        "active_observations": [],
        "reopened_coverage_ids": [],
        "validation_status": "passed",
        "validated_at_utc": "2026-07-20T00:00:00Z",
        "validation_result_fingerprint": "pending"
    });
    let mut validation_preimage = validation.clone();
    for field in [
        "validation_result_id",
        "validation_result_fingerprint",
        "validated_at_utc",
    ] {
        validation_preimage.as_object_mut().unwrap().remove(field);
    }
    let validation_fingerprint =
        crate::DefinitionFingerprint::from_json_value(&validation_preimage)
            .unwrap()
            .to_string();
    let validation_id = format!(
        "lifecycle-validation-result_{}",
        validation_fingerprint.strip_prefix("sha256:").unwrap()
    );
    validation["validation_result_id"] = Value::String(validation_id.clone());
    validation["validation_result_fingerprint"] = Value::String(validation_fingerprint);
    let validation_ref = format!("lifecycle-validation-results/{validation_id}.json");
    let validation_path = repo.join(".handbook/state").join(&validation_ref);
    std::fs::create_dir_all(validation_path.parent().unwrap()).unwrap();
    let mut validation_bytes = canonical(&validation);
    validation_bytes.push(b'\n');
    std::fs::write(validation_path, &validation_bytes).unwrap();

    candidate["validation_result_binding"] = json!({
        "validation_result_ref": validation_ref,
        "validation_result_fingerprint": validation["validation_result_fingerprint"],
        "result_document_sha256": crate::DefinitionFingerprint::from_bytes(&validation_bytes).to_string(),
        "result_byte_length": validation_bytes.len(),
    });
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
    approval["candidate_ref"] = Value::String(candidate_result.relative_ref);
    approval["candidate_fingerprint"] = Value::String(candidate_result.fingerprint);
    resign(
        &mut approval,
        "approval_id",
        "approval_fingerprint",
        "approval",
        &[],
    );
    store
        .append_record(LineageRecordClassV1::Approval, &canonical(&approval))
        .unwrap();
}

fn runtime_candidate_evaluation() -> crate::CharterCandidateEvaluationV12 {
    let observational = [
        "project_shape.definition",
        "delivery.constraints",
        "operational_reality.production_state",
        "risk.domains",
        "debt.register",
    ];
    let normative = [
        "delivery.default_implications",
        "engineering_posture.baseline",
        "policy.authority_and_revision",
        "governance.decision_authority",
        "governance.required_approvals",
        "governance.exception_policy",
        "engineering_posture.dimensions",
        "engineering_posture.red_lines",
        "governance.review_triggers",
        "governance.reassessment_triggers",
        "decisions.records",
    ];
    let mut coverage = Vec::new();
    for coverage_id in observational {
        coverage.push(CharterCoverageSubmission {
            coverage_id: coverage_id.to_owned(),
            source_kind: CharterIntakeSourceKind::EvidencedInference,
            value_ref: format!("intake-values/{coverage_id}.json"),
            evidence_refs: vec![format!("evidence.{coverage_id}")],
            confidence: "high".to_owned(),
            freshness: Some("session".to_owned()),
            sensitivity: "internal".to_owned(),
            contradiction_refs: Vec::new(),
            waiver_ref: None,
        });
    }
    for coverage_id in normative {
        coverage.push(CharterCoverageSubmission {
            coverage_id: coverage_id.to_owned(),
            source_kind: CharterIntakeSourceKind::UserDeclaration,
            value_ref: format!("intake-values/{coverage_id}.json"),
            evidence_refs: Vec::new(),
            confidence: "high".to_owned(),
            freshness: None,
            sensitivity: "internal".to_owned(),
            contradiction_refs: Vec::new(),
            waiver_ref: None,
        });
    }
    let decisions =
        crate::resolve_shipped_profile_decisions(std::path::Path::new(env!("CARGO_MANIFEST_DIR")))
            .unwrap();
    evaluate_charter_intake(
        &decisions,
        CharterIntakeEnvelope {
            mode: CharterAcquisitionMode::Express,
            content: crate::parse_definition_yaml(CANONICAL_CHARTER).unwrap(),
            coverage,
            consumer: CharterIntakeConsumer {
                kind: "handbook_skill".to_owned(),
                id: "handbook".to_owned(),
                version: "1.1".to_owned(),
            },
            prompt_event_refs: Vec::new(),
            finalized_at_utc: "2026-07-20T00:00:00Z".to_owned(),
            expected_current_fingerprint: None,
        },
        None,
    )
    .unwrap()
}

fn request_with_canonical(
    repo: &std::path::Path,
    canonical_bytes: Vec<u8>,
) -> CharterPromotionRequestV1 {
    let canonical_fingerprint =
        crate::DefinitionFingerprint::from_bytes(&canonical_bytes).to_string();

    let mut promotion = vector("promotion");
    let candidate_path = std::fs::read_dir(repo.join(".handbook/evidence/charter/candidates"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .next()
        .unwrap();
    let candidate: Value =
        serde_json::from_slice(&std::fs::read(&candidate_path).unwrap()).unwrap();
    let candidate_ref = format!(
        "candidates/{}",
        candidate_path.file_name().unwrap().to_string_lossy()
    );
    let approval_path = std::fs::read_dir(repo.join(".handbook/state/approvals"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .next()
        .unwrap();
    let approval_ref = format!(
        "approvals/{}",
        approval_path.file_name().unwrap().to_string_lossy()
    );
    promotion["canonical_artifact_fingerprint"] = Value::String(canonical_fingerprint.clone());
    promotion["candidate_ref"] = Value::String(candidate_ref);
    promotion["candidate_fingerprint"] = candidate["candidate_fingerprint"].clone();
    let definitions = crate::load_shipped_charter_definition_registry().unwrap();
    let decisions = crate::resolve_shipped_profile_decisions(repo).unwrap();
    promotion["resolved_definitions"] = Value::Array(
        crate::charter_promotion_workflow::resolved_definition_bindings(
            &candidate,
            &decisions,
            &definitions,
        )
        .unwrap(),
    );
    promotion["approval_refs"] = Value::Array(vec![Value::String(approval_ref)]);
    promotion["validation_result_refs"] = Value::Array(vec![candidate
        ["validation_result_binding"]["validation_result_ref"]
        .clone()]);
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
    let current_state_fingerprint =
        crate::charter_lifecycle_store::charter_lifecycle_state_fingerprint(
            lifecycle["lifecycle_policy_ref"].as_str().unwrap(),
            lifecycle["lifecycle_policy_fingerprint"].as_str().unwrap(),
            "project_authority",
            &canonical_fingerprint,
            crate::CharterLifecycleState::Current,
            &[],
        )
        .unwrap();
    lifecycle["prior_state_fingerprint"] = Value::String(current_state_fingerprint.clone());
    lifecycle["result_state_fingerprint"] = Value::String(current_state_fingerprint);
    lifecycle["clearance_promotion_ref"] = Value::String(format!("promotions/{promotion_id}.json"));
    resign(
        &mut lifecycle,
        "transition_id",
        "transition_fingerprint",
        "lifecycle-transition",
        &["transitioned_at_utc"],
    );

    CharterPromotionRequestV1 {
        canonical_bytes,
        promotion_record_bytes: canonical(&promotion),
        lifecycle_transition_bytes: canonical(&lifecycle),
    }
}

pub(super) fn request(repo: &std::path::Path) -> CharterPromotionRequestV1 {
    request_with_canonical(repo, CANONICAL_CHARTER.to_vec())
}

fn request_with_replaced_approval_pair(
    repo: &std::path::Path,
    approval_class: &str,
    authority_ref: &str,
) -> CharterPromotionRequestV1 {
    let mut request = request(repo);
    let mut promotion: Value = serde_json::from_slice(&request.promotion_record_bytes).unwrap();
    let original_ref = promotion["approval_refs"][0].as_str().unwrap();
    let mut approval: Value = serde_json::from_slice(
        &std::fs::read(repo.join(".handbook/state").join(original_ref)).unwrap(),
    )
    .unwrap();
    approval["approval_class"] = Value::String(approval_class.to_owned());
    approval["authority_ref"] = Value::String(authority_ref.to_owned());
    approval["decided_at_utc"] = Value::String("2026-07-21T13:21:00Z".to_owned());
    resign(
        &mut approval,
        "approval_id",
        "approval_fingerprint",
        "approval",
        &[],
    );
    let appended = TrustedLineageStoreV1::new(repo)
        .append_record(LineageRecordClassV1::Approval, &canonical(&approval))
        .unwrap();
    promotion["approval_refs"] = Value::Array(vec![Value::String(appended.relative_ref)]);
    promotion["authorized_by_ref"] = Value::String(authority_ref.to_owned());
    resign(
        &mut promotion,
        "promotion_id",
        "promotion_fingerprint",
        "promotion",
        &[],
    );
    request.promotion_record_bytes = canonical(&promotion);

    let mut lifecycle: Value = serde_json::from_slice(&request.lifecycle_transition_bytes).unwrap();
    lifecycle["clearance_promotion_ref"] = Value::String(format!(
        "promotions/{}.json",
        promotion["promotion_id"].as_str().unwrap()
    ));
    resign(
        &mut lifecycle,
        "transition_id",
        "transition_fingerprint",
        "lifecycle-transition",
        &["transitioned_at_utc"],
    );
    request.lifecycle_transition_bytes = canonical(&lifecycle);
    request
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
    let request = request(repo.path());

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
fn retained_promotion_refuses_a_coherently_rebound_unrequired_approval_pair() {
    let repo = tempfile::tempdir().unwrap();
    seed_lineage(repo.path());
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    let request =
        request_with_replaced_approval_pair(repo.path(), "Security approval", "Security lead");
    let retained = RetainedPromotionAuthorityV1 {
        canonical_basis_bytes: None,
        records: Vec::new(),
    };

    let error = service
        .begin_retained_authority()
        .unwrap()
        .promote_retained(request, &retained)
        .expect_err("retained promotion must recompute and require the exact approval pair set");

    assert!(matches!(
        error.kind(),
        CharterPromotionErrorKindV1::Conflict
            | CharterPromotionErrorKindV1::DurabilityViolation
            | CharterPromotionErrorKindV1::LineageViolation
    ));
    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
}

#[test]
fn target_kind_only_definition_closure_refuses_before_canonical_mutation() {
    let repo = tempfile::tempdir().unwrap();
    seed_lineage(repo.path());
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    let mut request = request(repo.path());
    let mut promotion: Value = serde_json::from_slice(&request.promotion_record_bytes).unwrap();
    let target_kind = promotion["resolved_definitions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|binding| {
            binding["definition_ref"] == "handbook.artifact-kind.project-authority@1.1.0"
        })
        .unwrap()
        .clone();
    promotion["resolved_definitions"] = Value::Array(vec![target_kind]);
    resign(
        &mut promotion,
        "promotion_id",
        "promotion_fingerprint",
        "promotion",
        &[],
    );
    request.promotion_record_bytes = canonical(&promotion);

    let error = service
        .promote(request)
        .expect_err("promotion must retain the complete ordered definition closure");

    assert_eq!(error.kind(), CharterPromotionErrorKindV1::LineageViolation);
    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
}

#[test]
fn reader_rolls_forward_w9_before_observation_and_never_exposes_partial_authority() {
    let repo = tempfile::tempdir().unwrap();
    seed_lineage(repo.path());
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    let request = request(repo.path());

    let error = service
        .promote_with_fault_for_testing(
            request,
            CharterPromotionFaultPointV1::AfterCanonicalInstalled,
        )
        .unwrap_err();
    assert_eq!(error.kind(), CharterPromotionErrorKindV1::InjectedFault);

    assert!(service.read_committed_charter().unwrap().is_some());
    assert!(repo.path().join(".handbook/project/charter.yaml").exists());
}

#[test]
fn complete_records_without_marker_roll_forward_during_recovery() {
    let repo = tempfile::tempdir().unwrap();
    seed_lineage(repo.path());
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    let request = request(repo.path());

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

fn recovery_refuses_coherently_rebound_definition_output(mutate: impl FnOnce(&mut Vec<Value>)) {
    let repo = tempfile::tempdir().unwrap();
    seed_lineage(repo.path());
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    let request = request(repo.path());

    let error = service
        .promote_with_fault_for_testing(
            request,
            CharterPromotionFaultPointV1::AfterCanonicalInstalled,
        )
        .unwrap_err();
    assert_eq!(error.kind(), CharterPromotionErrorKindV1::InjectedFault);
    let pending = repo
        .path()
        .join(".handbook/state/transactions/promotions")
        .read_dir()
        .unwrap()
        .find_map(|entry| {
            let path = entry.unwrap().path();
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .ends_with(".pending")
                .then_some(path)
        })
        .unwrap();
    let mut intent: Value =
        serde_json::from_slice(&std::fs::read(pending.join("intent.json")).unwrap()).unwrap();
    let mut promotion: Value =
        serde_json::from_slice(&std::fs::read(pending.join("promotion-record.new")).unwrap())
            .unwrap();
    mutate(promotion["resolved_definitions"].as_array_mut().unwrap());
    resign(
        &mut promotion,
        "promotion_id",
        "promotion_fingerprint",
        "promotion",
        &[],
    );
    let promotion_bytes = canonical(&promotion);
    let promotion_ref = format!(
        "promotions/{}.json",
        promotion["promotion_id"].as_str().unwrap()
    );

    let mut lifecycle: Value =
        serde_json::from_slice(&std::fs::read(pending.join("lifecycle-transition.new")).unwrap())
            .unwrap();
    lifecycle["clearance_promotion_ref"] = Value::String(promotion_ref.clone());
    resign(
        &mut lifecycle,
        "transition_id",
        "transition_fingerprint",
        "lifecycle-transition",
        &["transitioned_at_utc"],
    );
    let lifecycle_bytes = canonical(&lifecycle);
    let lifecycle_ref = format!(
        "lifecycle-transitions/{}.json",
        lifecycle["transition_id"].as_str().unwrap()
    );

    intent["promotion_id"] = promotion["promotion_id"].clone();
    intent["outputs"]["promotion_record"] = json!({
        "record_ref": promotion_ref,
        "record_fingerprint": promotion["promotion_fingerprint"],
        "document_sha256": crate::DefinitionFingerprint::from_bytes(&promotion_bytes).to_string(),
        "byte_length": promotion_bytes.len(),
    });
    intent["outputs"]["lifecycle_transition"] = json!({
        "record_ref": lifecycle_ref,
        "record_fingerprint": lifecycle["transition_fingerprint"],
        "document_sha256": crate::DefinitionFingerprint::from_bytes(&lifecycle_bytes).to_string(),
        "byte_length": lifecycle_bytes.len(),
    });
    let mut intent_preimage = intent.clone();
    intent_preimage
        .as_object_mut()
        .unwrap()
        .remove("intent_fingerprint");
    intent["intent_fingerprint"] = Value::String(
        crate::DefinitionFingerprint::from_json_value(&intent_preimage)
            .unwrap()
            .to_string(),
    );
    let mut intent_bytes = canonical(&intent);
    intent_bytes.push(b'\n');
    let marker = format!(
        "{}\n",
        crate::DefinitionFingerprint::from_bytes(&intent_bytes)
    );
    std::fs::write(pending.join("intent.json"), &intent_bytes).unwrap();
    std::fs::write(pending.join("prepared"), marker.as_bytes()).unwrap();
    std::fs::write(pending.join("promotion-record.new"), &promotion_bytes).unwrap();
    std::fs::write(pending.join("lifecycle-transition.new"), &lifecycle_bytes).unwrap();

    let refusal = service.read_committed_charter().expect_err(
        "recovery must reject an inexact definition output even when coherently rebound",
    );
    assert_eq!(
        refusal.kind(),
        CharterPromotionErrorKindV1::DurabilityViolation
    );
    assert!(pending.exists());
}

#[test]
fn recovery_refuses_coherently_rebound_subset_definition_output() {
    recovery_refuses_coherently_rebound_definition_output(|definitions| {
        definitions.retain(|binding| {
            binding["definition_ref"] == "handbook.artifact-kind.project-authority@1.1.0"
        });
    });
}

#[test]
fn recovery_refuses_coherently_rebound_reordered_definition_output() {
    recovery_refuses_coherently_rebound_definition_output(|definitions| definitions.reverse());
}

#[test]
fn recovery_refuses_coherently_resigned_candidate_field_source_forgery() {
    type CandidateFieldSourceForgery = (&'static str, Box<dyn Fn(&mut Value)>);

    let cases: Vec<CandidateFieldSourceForgery> = vec![
        (
            "empty",
            Box::new(|candidate| candidate["field_sources"] = Value::Array(Vec::new())),
        ),
        (
            "missing",
            Box::new(|candidate| {
                candidate["field_sources"].as_array_mut().unwrap().remove(0);
            }),
        ),
        (
            "reordered",
            Box::new(|candidate| {
                candidate["field_sources"]
                    .as_array_mut()
                    .unwrap()
                    .swap(0, 1);
            }),
        ),
        (
            "duplicate",
            Box::new(|candidate| {
                let duplicate = candidate["field_sources"][0].clone();
                candidate["field_sources"]
                    .as_array_mut()
                    .unwrap()
                    .insert(1, duplicate);
            }),
        ),
        (
            "unsafe-path",
            Box::new(|candidate| {
                candidate["field_sources"][0]["target_path"] =
                    Value::String("../policy/revision".to_owned());
            }),
        ),
        (
            "wrong-coverage",
            Box::new(|candidate| {
                candidate["field_sources"][0]["coverage_id"] =
                    Value::String("decisions.records".to_owned());
            }),
        ),
        (
            "unsupported-source-kind",
            Box::new(|candidate| {
                candidate["field_sources"][0]["source_kind"] = Value::String("approval".to_owned());
            }),
        ),
        (
            "source-free-leaf",
            Box::new(|candidate| {
                candidate["field_sources"].as_array_mut().unwrap().pop();
            }),
        ),
    ];

    for (case, mutator) in cases {
        let repo = tempfile::tempdir().unwrap();
        seed_lineage(repo.path());
        let service = CharterAuthorityTransactionServiceV1::new(repo.path());
        let request = request(repo.path());
        let failure = service
            .promote_with_fault_for_testing(
                request,
                CharterPromotionFaultPointV1::AfterCanonicalInstalled,
            )
            .unwrap_err();
        assert_eq!(failure.kind(), CharterPromotionErrorKindV1::InjectedFault);
        let pending = repo
            .path()
            .join(".handbook/state/transactions/promotions")
            .read_dir()
            .unwrap()
            .find_map(|entry| {
                let path = entry.unwrap().path();
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .ends_with(".pending")
                    .then_some(path)
            })
            .unwrap();
        let preserved =
            coherently_rebind_pending_candidate_provenance(repo.path(), &pending, mutator);

        let refusal = service
            .read_committed_charter()
            .expect_err("recovery must refuse coherently resigned candidate provenance");
        assert!(
            matches!(
                refusal.kind(),
                CharterPromotionErrorKindV1::LineageViolation
                    | CharterPromotionErrorKindV1::DurabilityViolation
            ),
            "{case}: {}",
            refusal.detail()
        );
        assert!(
            refusal.detail().contains("provenance") || refusal.detail().contains("field_sources"),
            "{case}: {}",
            refusal.detail()
        );
        assert!(pending.exists(), "{case}");
        for (path, bytes) in preserved {
            assert_eq!(std::fs::read(path).unwrap(), bytes, "{case}");
        }
    }
}

fn coherently_rebind_pending_candidate_provenance(
    repo: &std::path::Path,
    pending: &std::path::Path,
    mutator: Box<dyn Fn(&mut Value)>,
) -> Vec<(std::path::PathBuf, Vec<u8>)> {
    let mut intent: Value =
        serde_json::from_slice(&std::fs::read(pending.join("intent.json")).unwrap()).unwrap();
    let old_candidate_ref = intent["candidate_lineage"]["candidate_ref"]
        .as_str()
        .unwrap();
    let old_candidate_path = repo
        .join(".handbook/evidence/charter")
        .join(old_candidate_ref);
    let mut candidate: Value =
        serde_json::from_slice(&std::fs::read(old_candidate_path).unwrap()).unwrap();
    let old_result_ref = candidate["validation_result_binding"]["validation_result_ref"]
        .as_str()
        .unwrap();
    let old_result_path = repo.join(".handbook/state").join(old_result_ref);
    let mut result: Value =
        serde_json::from_slice(&std::fs::read(old_result_path).unwrap()).unwrap();

    mutator(&mut candidate);
    let mut subject = candidate.clone();
    for field in [
        "candidate_id",
        "candidate_fingerprint",
        "candidate_subject_fingerprint",
        "validation_result_binding",
    ] {
        subject.as_object_mut().unwrap().remove(field);
    }
    let subject_fingerprint = crate::DefinitionFingerprint::from_json_value(&subject)
        .unwrap()
        .to_string();
    candidate["candidate_subject_fingerprint"] = Value::String(subject_fingerprint.clone());
    result["candidate_subject_fingerprint"] = Value::String(subject_fingerprint.clone());
    let mut result_preimage = result.clone();
    for field in [
        "validation_result_id",
        "validation_result_fingerprint",
        "validated_at_utc",
    ] {
        result_preimage.as_object_mut().unwrap().remove(field);
    }
    let result_fingerprint = crate::DefinitionFingerprint::from_json_value(&result_preimage)
        .unwrap()
        .to_string();
    let result_id = format!(
        "lifecycle-validation-result_{}",
        result_fingerprint.strip_prefix("sha256:").unwrap()
    );
    result["validation_result_id"] = Value::String(result_id.clone());
    result["validation_result_fingerprint"] = Value::String(result_fingerprint.clone());
    let result_ref = format!("lifecycle-validation-results/{result_id}.json");
    let mut result_bytes = canonical(&result);
    result_bytes.push(b'\n');
    let result_path = repo.join(".handbook/state").join(&result_ref);
    std::fs::write(&result_path, &result_bytes).unwrap();
    candidate["validation_result_binding"] = json!({
        "validation_result_ref": result_ref,
        "validation_result_fingerprint": result_fingerprint,
        "result_document_sha256": crate::DefinitionFingerprint::from_bytes(&result_bytes).to_string(),
        "result_byte_length": result_bytes.len(),
    });
    resign(
        &mut candidate,
        "candidate_id",
        "candidate_fingerprint",
        "candidate",
        &[],
    );
    let candidate_ref = format!(
        "candidates/{}.json",
        candidate["candidate_id"].as_str().unwrap()
    );
    let candidate_path = repo.join(".handbook/evidence/charter").join(&candidate_ref);
    let candidate_bytes = canonical(&candidate);
    std::fs::write(&candidate_path, &candidate_bytes).unwrap();

    let old_approval_ref = intent["human_authority"]["approval_bindings"][0]["approval_ref"]
        .as_str()
        .unwrap();
    let mut approval: Value = serde_json::from_slice(
        &std::fs::read(repo.join(".handbook/state").join(old_approval_ref)).unwrap(),
    )
    .unwrap();
    approval["candidate_ref"] = Value::String(candidate_ref.clone());
    approval["candidate_fingerprint"] = candidate["candidate_fingerprint"].clone();
    resign(
        &mut approval,
        "approval_id",
        "approval_fingerprint",
        "approval",
        &[],
    );
    let approval_ref = format!(
        "approvals/{}.json",
        approval["approval_id"].as_str().unwrap()
    );
    let approval_path = repo.join(".handbook/state").join(&approval_ref);
    let approval_bytes = canonical(&approval);
    std::fs::write(&approval_path, &approval_bytes).unwrap();

    let mut promotion: Value =
        serde_json::from_slice(&std::fs::read(pending.join("promotion-record.new")).unwrap())
            .unwrap();
    promotion["candidate_ref"] = Value::String(candidate_ref.clone());
    promotion["candidate_fingerprint"] = candidate["candidate_fingerprint"].clone();
    promotion["approval_refs"] = json!([approval_ref]);
    promotion["validation_result_refs"] = json!([result_ref]);
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
    let promotion_bytes = canonical(&promotion);

    let mut lifecycle: Value =
        serde_json::from_slice(&std::fs::read(pending.join("lifecycle-transition.new")).unwrap())
            .unwrap();
    lifecycle["clearance_promotion_ref"] = Value::String(promotion_ref.clone());
    resign(
        &mut lifecycle,
        "transition_id",
        "transition_fingerprint",
        "lifecycle-transition",
        &["transitioned_at_utc"],
    );
    let lifecycle_ref = format!(
        "lifecycle-transitions/{}.json",
        lifecycle["transition_id"].as_str().unwrap()
    );
    let lifecycle_bytes = canonical(&lifecycle);

    intent["promotion_id"] = promotion["promotion_id"].clone();
    intent["candidate_lineage"]["candidate_ref"] = Value::String(candidate_ref);
    intent["candidate_lineage"]["candidate_fingerprint"] =
        candidate["candidate_fingerprint"].clone();
    intent["candidate_lineage"]["candidate_subject_fingerprint"] =
        Value::String(subject_fingerprint);
    intent["candidate_lineage"]["validation_result_ref"] = Value::String(result_ref);
    intent["candidate_lineage"]["validation_result_fingerprint"] =
        Value::String(result_fingerprint);
    intent["human_authority"]["approval_bindings"][0]["approval_ref"] = Value::String(approval_ref);
    intent["human_authority"]["approval_bindings"][0]["approval_fingerprint"] =
        approval["approval_fingerprint"].clone();
    intent["outputs"]["promotion_record"] = json!({
        "record_ref": promotion_ref,
        "record_fingerprint": promotion["promotion_fingerprint"],
        "document_sha256": crate::DefinitionFingerprint::from_bytes(&promotion_bytes).to_string(),
        "byte_length": promotion_bytes.len(),
    });
    intent["outputs"]["lifecycle_transition"] = json!({
        "record_ref": lifecycle_ref,
        "record_fingerprint": lifecycle["transition_fingerprint"],
        "document_sha256": crate::DefinitionFingerprint::from_bytes(&lifecycle_bytes).to_string(),
        "byte_length": lifecycle_bytes.len(),
    });
    let mut intent_preimage = intent.clone();
    intent_preimage
        .as_object_mut()
        .unwrap()
        .remove("intent_fingerprint");
    intent["intent_fingerprint"] = Value::String(
        crate::DefinitionFingerprint::from_json_value(&intent_preimage)
            .unwrap()
            .to_string(),
    );
    let mut intent_bytes = canonical(&intent);
    intent_bytes.push(b'\n');
    let marker = format!(
        "{}\n",
        crate::DefinitionFingerprint::from_bytes(&intent_bytes)
    )
    .into_bytes();
    std::fs::write(pending.join("intent.json"), &intent_bytes).unwrap();
    std::fs::write(pending.join("prepared"), &marker).unwrap();
    std::fs::write(pending.join("promotion-record.new"), &promotion_bytes).unwrap();
    std::fs::write(pending.join("lifecycle-transition.new"), &lifecycle_bytes).unwrap();
    vec![
        (candidate_path, candidate_bytes),
        (result_path, result_bytes),
        (approval_path, approval_bytes),
        (pending.join("intent.json"), intent_bytes),
        (pending.join("prepared"), marker),
        (pending.join("promotion-record.new"), promotion_bytes),
        (pending.join("lifecycle-transition.new"), lifecycle_bytes),
    ]
}

#[test]
fn concurrent_create_promotions_publish_only_one_authority() {
    let repo = tempfile::tempdir().unwrap();
    seed_lineage(repo.path());
    let service = Arc::new(CharterAuthorityTransactionServiceV1::new(repo.path()));
    let barrier = Arc::new(Barrier::new(3));
    let mut threads = Vec::new();
    for _ in ["promotion-race-a", "promotion-race-b"] {
        let service = Arc::clone(&service);
        let barrier = Arc::clone(&barrier);
        let repo_root = repo.path().to_path_buf();
        threads.push(std::thread::spawn(move || {
            barrier.wait();
            service.promote(request(&repo_root))
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

    let mut substituted = request(repo.path());
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

    let original = request(repo.path());
    service.promote(original.clone()).unwrap();
    assert_eq!(
        service.promote(original).unwrap_err().kind(),
        CharterPromotionErrorKindV1::BasisMismatch
    );
    assert_eq!(
        service.promote(request(repo.path())).unwrap_err().kind(),
        CharterPromotionErrorKindV1::BasisMismatch
    );
}

#[test]
fn unequal_preexisting_record_refuses_before_canonical_delta() {
    let repo = tempfile::tempdir().unwrap();
    seed_lineage(repo.path());
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    let request = request(repo.path());
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
    let committed = service.promote(request(repo.path())).unwrap();
    std::fs::write(&committed.committed_marker_path, vec![b'0'; 72]).unwrap();

    assert_eq!(
        service.read_committed_charter().unwrap_err().kind(),
        CharterPromotionErrorKindV1::DurabilityViolation
    );
}

#[test]
fn transaction_identity_is_engine_owned_and_distinct_from_promotion_identity() {
    let repo = tempfile::tempdir().unwrap();
    seed_lineage(repo.path());
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    let commit = service.promote(request(repo.path())).unwrap();
    let intent_path = commit
        .committed_marker_path
        .parent()
        .unwrap()
        .join("intent.json");
    let intent: Value = serde_json::from_slice(&std::fs::read(intent_path).unwrap()).unwrap();
    let transaction_id = intent["transaction_id"].as_str().unwrap();
    let promotion_id = intent["promotion_id"].as_str().unwrap();
    assert!(transaction_id.starts_with("promotion-transaction_"));
    assert!(promotion_id.starts_with("promotion_"));
    assert_ne!(transaction_id, promotion_id);
}
