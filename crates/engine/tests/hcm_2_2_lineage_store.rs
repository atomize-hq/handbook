use handbook_engine::{
    parse_definition_yaml, resolve_shipped_profile_decisions, AppendDispositionV1,
    CharterAcquisitionMode, CharterAuthorPersistenceErrorKindV1, CharterAuthorPersistenceServiceV1,
    CharterCoverageSubmission, CharterIntakeConsumer, CharterIntakeEnvelope,
    CharterIntakeSourceKind, DefinitionFingerprint, LineageRecordClassV1, LineageStoreErrorKindV1,
    TrustedLineageStoreV1, MAX_LINEAGE_RECORD_BYTES,
};
use serde_json::Value;
use std::path::Path;

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

#[test]
fn frozen_intake_and_candidate_1_3_are_create_new_or_exact_replay() {
    let repo = tempfile::tempdir().unwrap();
    let store = TrustedLineageStoreV1::new(repo.path());
    let intake = canonical(&vector("intake"));
    let candidate = canonical(&vector("candidate"));

    let created = store
        .append_record(LineageRecordClassV1::Intake, &intake)
        .expect("create immutable intake");
    assert_eq!(created.disposition, AppendDispositionV1::Created);
    assert_eq!(
        created.relative_ref,
        "intake-records/intake_680b82e17d097e08e189ce1a985e0200ce21c87700dd89dbbdb375231303204a.json"
    );

    let replay = store
        .append_record(LineageRecordClassV1::Intake, &intake)
        .expect("exact replay is idempotent");
    assert_eq!(replay.disposition, AppendDispositionV1::ReusedEqual);

    let candidate_created = store
        .append_record(LineageRecordClassV1::Candidate, &candidate)
        .expect("candidate 1.3 is selected immutable product authority");
    assert_eq!(candidate_created.disposition, AppendDispositionV1::Created);
    assert!(repo
        .path()
        .join(".handbook/evidence/charter")
        .join(&candidate_created.relative_ref)
        .is_file());
    let candidate_replay = store
        .append_record(LineageRecordClassV1::Candidate, &candidate)
        .expect("candidate 1.3 exact replay is idempotent");
    assert_eq!(
        candidate_replay.disposition,
        AppendDispositionV1::ReusedEqual
    );
}

#[test]
fn unsafe_refs_oversize_records_and_changed_bytes_fail_closed() {
    let repo = tempfile::tempdir().unwrap();
    let store = TrustedLineageStoreV1::new(repo.path());
    let intake = canonical(&vector("intake"));
    let appended = store
        .append_record(LineageRecordClassV1::Intake, &intake)
        .unwrap();

    let unsafe_ref = store
        .read_record(
            LineageRecordClassV1::Intake,
            "intake-records/../outside.json",
            &appended.fingerprint,
        )
        .unwrap_err();
    assert_eq!(unsafe_ref.kind(), LineageStoreErrorKindV1::UnsafeReference);

    let oversized = store
        .append_record(
            LineageRecordClassV1::Intake,
            &vec![b' '; MAX_LINEAGE_RECORD_BYTES + 1],
        )
        .unwrap_err();
    assert_eq!(oversized.kind(), LineageStoreErrorKindV1::BoundExceeded);

    let final_path = repo
        .path()
        .join(".handbook/state")
        .join(&appended.relative_ref);
    std::fs::write(&final_path, b"changed").unwrap();
    let changed = store
        .append_record(LineageRecordClassV1::Intake, &intake)
        .unwrap_err();
    assert_eq!(
        changed.kind(),
        LineageStoreErrorKindV1::ExistingBytesMismatch
    );
}

#[test]
fn candidate_basis_must_equal_its_retained_intake_basis() {
    let repo = tempfile::tempdir().unwrap();
    let decisions =
        resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    let persisted = CharterAuthorPersistenceServiceV1::new(repo.path())
        .persist(&decisions, author_envelope(), None)
        .unwrap();
    let store = TrustedLineageStoreV1::new(repo.path());
    let mut candidate: Value = serde_json::from_slice(
        &std::fs::read(
            repo.path()
                .join(".handbook/evidence/charter")
                .join(persisted.candidate_ref),
        )
        .unwrap(),
    )
    .unwrap();
    candidate["basis_artifact_fingerprint"] = Value::String(format!("sha256:{}", "1".repeat(64)));
    let mut subject_preimage = candidate.clone();
    let subject = subject_preimage.as_object_mut().unwrap();
    for field in [
        "candidate_id",
        "candidate_fingerprint",
        "candidate_subject_fingerprint",
        "validation_result_binding",
    ] {
        subject.remove(field);
    }
    candidate["candidate_subject_fingerprint"] = Value::String(
        DefinitionFingerprint::from_json_value(&subject_preimage)
            .unwrap()
            .to_string(),
    );
    resign(
        &mut candidate,
        "candidate_id",
        "candidate_fingerprint",
        "candidate",
    );
    let error = store
        .append_record(LineageRecordClassV1::Candidate, &canonical(&candidate))
        .unwrap_err();

    assert_eq!(error.kind(), LineageStoreErrorKindV1::BasisMismatch);
}

#[test]
fn author_bundle_persists_content_then_intake_then_candidate_idempotently() {
    let repo = tempfile::tempdir().unwrap();
    let decisions =
        resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    let service = CharterAuthorPersistenceServiceV1::new(repo.path());
    let created = service
        .persist(&decisions, author_envelope(), None)
        .unwrap();
    assert_eq!(
        std::fs::read(
            repo.path()
                .join(".handbook/state")
                .join(&created.normalized_content_ref)
        )
        .unwrap(),
        CANONICAL_CHARTER
    );
    assert_eq!(
        service
            .persist(&decisions, author_envelope(), None)
            .unwrap(),
        created
    );

    std::fs::write(
        repo.path()
            .join(".handbook/state")
            .join(&created.normalized_content_ref),
        b"unequal",
    )
    .unwrap();
    assert_eq!(
        service
            .persist(&decisions, author_envelope(), None)
            .unwrap_err()
            .kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused
    );
}

fn author_envelope() -> CharterIntakeEnvelope {
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
            contradiction_refs: vec![],
            waiver_ref: None,
        });
    }
    for coverage_id in normative {
        coverage.push(CharterCoverageSubmission {
            coverage_id: coverage_id.to_owned(),
            source_kind: CharterIntakeSourceKind::UserDeclaration,
            value_ref: format!("intake-values/{coverage_id}.json"),
            evidence_refs: vec![],
            confidence: "high".to_owned(),
            freshness: None,
            sensitivity: "internal".to_owned(),
            contradiction_refs: vec![],
            waiver_ref: None,
        });
    }
    CharterIntakeEnvelope {
        mode: CharterAcquisitionMode::Express,
        content: parse_definition_yaml(CANONICAL_CHARTER).unwrap(),
        coverage,
        consumer: CharterIntakeConsumer {
            kind: "handbook_skill".to_owned(),
            id: "handbook".to_owned(),
            version: "1.1".to_owned(),
        },
        prompt_event_refs: vec![],
        finalized_at_utc: "2026-07-19T16:00:00Z".to_owned(),
        expected_current_fingerprint: None,
    }
}

fn resign(value: &mut Value, id_field: &str, fingerprint_field: &str, prefix: &str) {
    let object = value.as_object_mut().unwrap();
    object.remove(id_field);
    object.remove(fingerprint_field);
    let fingerprint = handbook_engine::DefinitionFingerprint::from_json_value(value)
        .unwrap()
        .to_string();
    let id = format!("{prefix}_{}", fingerprint.strip_prefix("sha256:").unwrap());
    value[id_field] = Value::String(id);
    value[fingerprint_field] = Value::String(fingerprint);
}
