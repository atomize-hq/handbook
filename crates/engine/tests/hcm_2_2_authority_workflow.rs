use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use handbook_engine::{
    observe_committed_approver_registry, parse_definition_yaml, resolve_shipped_profile_decisions,
    ApproverAdminRequestV1, ApproverRegistryServiceV1, CharterAcquisitionMode,
    CharterAuthorPersistenceServiceV1, CharterCoverageSubmission, CharterIntakeConsumer,
    CharterIntakeEnvelope, CharterIntakeSourceKind, NativeAuthenticatorPortErrorV1,
    NativeAuthenticatorPortV1,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::Path;

const ADMIN_VECTORS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/approver-admin-api-vectors-v1.0.json"
));
const AUTHENTICATOR_VECTORS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/authenticator-transcript-vectors-v1.0.json"
));
const BOUNDARY_YAML: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));

#[derive(Default)]
struct BootstrapPort {
    response: Vec<u8>,
    make_calls: Vec<Vec<u8>>,
}

impl NativeAuthenticatorPortV1 for BootstrapPort {
    fn make_credential(
        &mut self,
        request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        self.make_calls.push(request_cbor.to_vec());
        Ok(self.response.clone())
    }

    fn get_assertion(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        panic!("bootstrap must not request an assertion")
    }
}

fn bootstrap_request() -> ApproverAdminRequestV1 {
    let fixture: Value = serde_json::from_slice(ADMIN_VECTORS).unwrap();
    let document = fixture["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["vector_id"] == "bootstrap-request")
        .unwrap()["document"]
        .clone();
    ApproverAdminRequestV1::from_json_value(document).unwrap()
}

fn bootstrap_response() -> Vec<u8> {
    let fixture: Value = serde_json::from_slice(AUTHENTICATOR_VECTORS).unwrap();
    let vector = fixture["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["operation"] == "bootstrap")
        .unwrap();
    let mut response = vec![0];
    response.extend(
        BASE64_STANDARD
            .decode(
                vector["make_credential_response"]["attestation_object_base64"]
                    .as_str()
                    .unwrap(),
            )
            .unwrap(),
    );
    response
}

fn intake_envelope() -> CharterIntakeEnvelope {
    const OBSERVATIONAL: [&str; 5] = [
        "project_shape.definition",
        "delivery.constraints",
        "operational_reality.production_state",
        "risk.domains",
        "debt.register",
    ];
    const NORMATIVE: [&str; 11] = [
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
    let mut coverage = OBSERVATIONAL
        .iter()
        .map(|coverage_id| CharterCoverageSubmission {
            coverage_id: (*coverage_id).to_owned(),
            source_kind: CharterIntakeSourceKind::EvidencedInference,
            value_ref: format!("intake-values/{coverage_id}.json"),
            evidence_refs: vec![format!("evidence.{coverage_id}")],
            confidence: "high".to_owned(),
            freshness: Some("session".to_owned()),
            sensitivity: "internal".to_owned(),
            contradiction_refs: vec![],
            waiver_ref: None,
        })
        .collect::<Vec<_>>();
    coverage.extend(
        NORMATIVE
            .iter()
            .map(|coverage_id| CharterCoverageSubmission {
                coverage_id: (*coverage_id).to_owned(),
                source_kind: CharterIntakeSourceKind::UserDeclaration,
                value_ref: format!("intake-values/{coverage_id}.json"),
                evidence_refs: vec![],
                confidence: "high".to_owned(),
                freshness: None,
                sensitivity: "internal".to_owned(),
                contradiction_refs: vec![],
                waiver_ref: None,
            }),
    );
    CharterIntakeEnvelope {
        mode: CharterAcquisitionMode::Express,
        content: parse_definition_yaml(BOUNDARY_YAML).unwrap(),
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

#[test]
fn author_service_persists_exact_intake_candidate_and_normalized_content() {
    let temp = tempfile::tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped decisions");
    let result = CharterAuthorPersistenceServiceV1::new(temp.path())
        .persist(&decisions, intake_envelope(), None)
        .expect("persist exact Charter author bundle");

    assert!(temp
        .path()
        .join(".handbook/state")
        .join(&result.intake_ref)
        .is_file());
    assert!(temp
        .path()
        .join(".handbook/evidence/charter")
        .join(&result.candidate_ref)
        .is_file());
    assert!(temp
        .path()
        .join(".handbook/state")
        .join(&result.normalized_content_ref)
        .is_file());
    assert!(result
        .candidate_ref
        .contains(&result.candidate_fingerprint[7..]));
}

#[test]
fn bootstrap_commits_native_registration_registry_state_transition_and_use_head() {
    let temp = tempfile::tempdir().unwrap();
    let port = BootstrapPort {
        response: bootstrap_response(),
        ..BootstrapPort::default()
    };
    let mut service = ApproverRegistryServiceV1::for_repository(temp.path(), port);
    let result = service
        .bootstrap_approver_registry(bootstrap_request())
        .to_json_value()
        .unwrap();

    assert_eq!(result["status"], "succeeded");
    assert_eq!(result["changed_paths"].as_array().unwrap().len(), 4);
    for path in result["changed_paths"].as_array().unwrap() {
        assert!(temp
            .path()
            .join(".handbook/state")
            .join(path.as_str().unwrap())
            .is_file());
    }
    let credential_hash = result["changed_paths"][2]
        .as_str()
        .expect("registry state path");
    assert!(credential_hash.contains("registry-state_"));
    let committed = temp
        .path()
        .join(".handbook/state/transactions/registry")
        .read_dir()
        .unwrap()
        .find_map(|entry| {
            let path = entry.unwrap().path();
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .ends_with(".committed")
                .then_some(path)
        })
        .expect("committed registry journal");
    let marker = std::fs::read(committed.join("committed")).unwrap();
    assert_eq!(marker.len(), 72);
    assert_eq!(marker.last(), Some(&b'\n'));
    let use_head_path = temp
        .path()
        .join(".handbook/state/authenticator-use-heads")
        .read_dir()
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let use_head_bytes = std::fs::read(use_head_path).unwrap();
    let mut use_head: Value = serde_json::from_slice(&use_head_bytes).unwrap();
    assert_eq!(
        use_head
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "$schema",
            "credential_id_hash",
            "head_fingerprint",
            "last_assertion_fingerprint",
            "last_assertion_ref",
            "last_challenge_fingerprint",
            "last_nonce_sha256",
            "schema_version",
            "sequence",
            "sign_count",
        ])
    );
    let declared = use_head["head_fingerprint"].as_str().unwrap().to_owned();
    use_head.as_object_mut().unwrap().remove("head_fingerprint");
    let preimage = serde_json_canonicalizer::to_vec(&use_head).unwrap();
    assert_eq!(declared, format!("sha256:{:x}", Sha256::digest(preimage)));
    let observation = observe_committed_approver_registry(temp.path()).unwrap();
    assert_eq!(
        observation.state_ref,
        result["result_registry_state_ref"].as_str().unwrap()
    );
    assert_eq!(
        observation.head_transition_ref,
        result["transition_ref"].as_str().unwrap()
    );
    assert_eq!(observation.credentials.len(), 1);
    assert!(observation.credentials[0].active);
    assert_eq!(observation.credentials[0].use_sequence, 0);
    assert_eq!(observation.credentials[0].approval_mappings.len(), 2);
    drop(observation);
    let second = service
        .bootstrap_approver_registry(bootstrap_request())
        .to_json_value()
        .unwrap();
    assert_eq!(second["status"], "refused");
    assert_eq!(second["changed_paths"], serde_json::json!([]));
    assert_eq!(service.into_port().make_calls.len(), 1);
}

#[test]
fn unsafe_authority_lock_refuses_before_native_bootstrap_call() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(temp.path().join(".handbook/state/locks/promotion.lock")).unwrap();
    let port = BootstrapPort {
        response: bootstrap_response(),
        ..BootstrapPort::default()
    };
    let mut service = ApproverRegistryServiceV1::for_repository(temp.path(), port);
    let result = service
        .bootstrap_approver_registry(bootstrap_request())
        .to_json_value()
        .unwrap();

    assert_eq!(result["status"], "refused");
    assert_eq!(result["changed_paths"], serde_json::json!([]));
    assert!(service.into_port().make_calls.is_empty());
}

#[test]
fn unsafe_pending_journal_child_refuses_before_native_bootstrap_call() {
    let temp = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(
        temp.path()
            .join(".handbook/state/transactions/registry/unsafe.pending/intent.json"),
    )
    .unwrap();
    let port = BootstrapPort {
        response: bootstrap_response(),
        ..BootstrapPort::default()
    };
    let mut service = ApproverRegistryServiceV1::for_repository(temp.path(), port);
    let result = service
        .bootstrap_approver_registry(bootstrap_request())
        .to_json_value()
        .unwrap();

    assert_eq!(result["status"], "refused");
    assert_eq!(result["changed_paths"], serde_json::json!([]));
    assert!(service.into_port().make_calls.is_empty());
}
