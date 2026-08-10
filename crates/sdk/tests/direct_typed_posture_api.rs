use handbook_engine::{
    parse_definition_yaml, resolve_shipped_profile_decisions, ApproverAdminRequestV1,
    ApproverRegistryServiceV1, CharterAcquisitionMode, CharterApprovalRequestV1,
    CharterApprovalResultV1, CharterApprovalServiceV1, CharterAuthorPersistenceServiceV1,
    CharterCoverageSubmission, CharterIntakeConsumer, CharterIntakeEnvelope,
    CharterIntakeSourceKind, CharterPromotionIntentV1, CharterPromotionWorkflowServiceV1,
    DefinitionFingerprint, NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1,
    PostureTransitionFaultInjectionGuardV1, RepositoryInvocationIdentityServiceV1,
    AUTHENTICATOR_RP_ID,
};
use handbook_sdk::{
    HandbookSdkV1, PostureCanonicalBindingV1, PostureDimensionChangeRequestV1, PostureReferenceV1,
    PostureTransitionApplyRequestV1, PostureTransitionApplyResultV1,
    PostureTransitionRefusalCodeV1,
};
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const ADMIN_VECTORS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/approver-admin-api-vectors-v1.0.json"
));
const BOUNDARY_YAML: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));

struct DynamicApprovalPort {
    key: SigningKey,
    credential_id: Vec<u8>,
    get_calls: usize,
}

impl DynamicApprovalPort {
    fn new() -> Self {
        Self {
            key: SigningKey::from_slice(&[1_u8; 32]).unwrap(),
            credential_id: vec![0x11; 32],
            get_calls: 0,
        }
    }
}

impl NativeAuthenticatorPortV1 for DynamicApprovalPort {
    fn make_credential(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        Ok(make_credential_response(&self.key, &self.credential_id))
    }

    fn get_assertion(
        &mut self,
        request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        let marker = request_cbor
            .windows(3)
            .position(|window| window == [0x02, 0x58, 0x20])
            .expect("GetAssertion client-data hash marker");
        let client_data_hash: [u8; 32] = request_cbor[marker + 3..marker + 35].try_into().unwrap();
        self.get_calls += 1;
        Ok(assertion_response(
            &self.key,
            &self.credential_id,
            client_data_hash,
            self.get_calls as u32,
        ))
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

fn make_credential_response(key: &SigningKey, credential_id: &[u8]) -> Vec<u8> {
    let point = key.verifying_key().to_encoded_point(false);
    let mut cose = vec![0xa5, 0x01, 0x02, 0x03, 0x26, 0x20, 0x01, 0x21, 0x58, 0x20];
    cose.extend_from_slice(point.x().unwrap());
    cose.extend_from_slice(&[0x22, 0x58, 0x20]);
    cose.extend_from_slice(point.y().unwrap());
    let mut auth_data = Sha256::digest(AUTHENTICATOR_RP_ID.as_bytes()).to_vec();
    auth_data.push(0x45);
    auth_data.extend_from_slice(&0_u32.to_be_bytes());
    auth_data.extend_from_slice(&[0_u8; 16]);
    auth_data.extend_from_slice(&(credential_id.len() as u16).to_be_bytes());
    auth_data.extend_from_slice(credential_id);
    auth_data.extend_from_slice(&cose);
    let mut response = vec![0, 0xa3];
    cbor_text(&mut response, "fmt");
    cbor_text(&mut response, "none");
    cbor_text(&mut response, "attStmt");
    response.push(0xa0);
    cbor_text(&mut response, "authData");
    cbor_bytes(&mut response, &auth_data);
    response
}

fn assertion_response(
    key: &SigningKey,
    credential_id: &[u8],
    client_data_hash: [u8; 32],
    sign_count: u32,
) -> Vec<u8> {
    let mut authenticator_data = Sha256::digest(AUTHENTICATOR_RP_ID.as_bytes()).to_vec();
    authenticator_data.push(0x05);
    authenticator_data.extend_from_slice(&sign_count.to_be_bytes());
    let mut preimage = authenticator_data.clone();
    preimage.extend_from_slice(&client_data_hash);
    let signature: Signature = key.sign(&preimage);
    let der = signature.to_der();
    let mut response = vec![0, 0xa3, 0x01, 0xa2];
    cbor_text(&mut response, "id");
    cbor_bytes(&mut response, credential_id);
    cbor_text(&mut response, "type");
    cbor_text(&mut response, "public-key");
    response.push(0x02);
    cbor_bytes(&mut response, &authenticator_data);
    response.push(0x03);
    cbor_bytes(&mut response, der.as_bytes());
    response
}

fn cbor_text(output: &mut Vec<u8>, value: &str) {
    cbor_length(output, 3, value.len());
    output.extend_from_slice(value.as_bytes());
}

fn cbor_bytes(output: &mut Vec<u8>, value: &[u8]) {
    cbor_length(output, 2, value.len());
    output.extend_from_slice(value);
}

fn cbor_length(output: &mut Vec<u8>, major: u8, length: usize) {
    if length < 24 {
        output.push((major << 5) | length as u8);
    } else if length <= u8::MAX as usize {
        output.extend_from_slice(&[(major << 5) | 24, length as u8]);
    } else {
        output.push((major << 5) | 25);
        output.extend_from_slice(&(length as u16).to_be_bytes());
    }
}

fn intake_envelope(clear_testing_override: bool) -> CharterIntakeEnvelope {
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
    let mut content = parse_definition_yaml(BOUNDARY_YAML).unwrap();
    if clear_testing_override {
        let testing = content["engineering_posture"]["dimensions"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|dimension| dimension["dimension_id"] == "testing_rigor")
            .unwrap();
        testing["level_override"] = Value::Null;
    }
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
        content,
        coverage,
        consumer: CharterIntakeConsumer {
            kind: "handbook_sdk_integration".to_owned(),
            id: "handbook-sdk".to_owned(),
            version: "0.1.0".to_owned(),
        },
        prompt_event_refs: vec![],
        finalized_at_utc: "2026-08-10T16:00:00Z".to_owned(),
        expected_current_fingerprint: None,
    }
}

fn approve(
    approvals: &mut CharterApprovalServiceV1<DynamicApprovalPort>,
    decisions: &handbook_engine::ResolvedProfileDecisions,
    candidate_ref: String,
    operation_id: &str,
) -> handbook_engine::CharterApprovalSuccessV1 {
    let result = approvals.approve_candidate(
        decisions,
        CharterApprovalRequestV1 {
            operation_id: operation_id.to_owned(),
            candidate_ref,
            approval_class: "Project owner approval".to_owned(),
            authority_ref: "Project owner".to_owned(),
            accepted_waiver_refs: vec![],
        },
    );
    let CharterApprovalResultV1::Succeeded(success) = result else {
        panic!("fixture approval must succeed: {result:?}")
    };
    success
}

fn reference() -> PostureReferenceV1 {
    PostureReferenceV1 {
        reference: String::new(),
        fingerprint: String::new(),
    }
}

fn canonical_binding(repo_root: &Path) -> PostureCanonicalBindingV1 {
    let bytes = fs::read(repo_root.join(".handbook/project/charter.yaml")).unwrap();
    let fingerprint = DefinitionFingerprint::from_bytes(&bytes).to_string();
    PostureCanonicalBindingV1 {
        reference: ".handbook/project/charter.yaml".to_owned(),
        fingerprint: fingerprint.clone(),
        document_sha256: fingerprint,
        byte_length: bytes.len() as u64,
    }
}

fn prepared_posture_request() -> (tempfile::TempDir, PostureTransitionApplyRequestV1) {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir(repo.path().join(".handbook")).unwrap();
    RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .unwrap();
    let decisions =
        resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    let initial = CharterAuthorPersistenceServiceV1::new(repo.path())
        .persist(&decisions, intake_envelope(false), None)
        .unwrap();
    let mut registry =
        ApproverRegistryServiceV1::for_repository(repo.path(), DynamicApprovalPort::new());
    assert_eq!(
        registry
            .bootstrap_approver_registry(bootstrap_request())
            .to_json_value()
            .unwrap()["status"],
        "succeeded"
    );
    let mut approvals = CharterApprovalServiceV1::new(repo.path(), registry.into_port());
    let initial_approval = approve(
        &mut approvals,
        &decisions,
        initial.candidate_ref.clone(),
        "sdk-posture-initial-approval",
    );
    let initial_commit = CharterPromotionWorkflowServiceV1::new(repo.path())
        .promote(CharterPromotionIntentV1 {
            candidate_ref: initial.candidate_ref,
            approval_ref: initial_approval.approval_ref,
            expected_current_fingerprint: None,
        })
        .unwrap();
    let mut amended_envelope = intake_envelope(true);
    amended_envelope.expected_current_fingerprint =
        Some(initial_commit.canonical_fingerprint.clone());
    let expected_current =
        DefinitionFingerprint::parse(&initial_commit.canonical_fingerprint).unwrap();
    let recommendation = CharterAuthorPersistenceServiceV1::new(repo.path())
        .persist(&decisions, amended_envelope, Some(&expected_current))
        .unwrap();
    let recommendation_approval = approve(
        &mut approvals,
        &decisions,
        recommendation.candidate_ref.clone(),
        "sdk-posture-recommendation-approval",
    );
    assert!(recommendation_approval
        .approval_ref
        .starts_with("approvals/approval_"));
    assert_eq!(approvals.into_port().get_calls, 2);
    let repository_identity_fingerprint =
        fs::read_to_string(repo.path().join(".handbook/repository-identity.v1")).unwrap();
    let request = PostureTransitionApplyRequestV1 {
        idempotency_key: "sdk-posture-proof-0001".to_owned(),
        repository_identity_fingerprint,
        expected_canonical: canonical_binding(repo.path()),
        recommendation: PostureReferenceV1 {
            reference: recommendation.candidate_ref,
            fingerprint: recommendation.candidate_fingerprint,
        },
        reassessment_coverage_ids: vec!["engineering_posture.dimensions".to_owned()],
        change: PostureDimensionChangeRequestV1 {
            dimension_id: "testing_rigor".to_owned(),
            dimension_index: 2,
            authority_path: "/engineering_posture/dimensions/2/level_override".to_owned(),
            expected_stored_value: Some(4),
            expected_effective_level: 4,
            proposed_effective_level: 3,
        },
        effective_at_utc: "2026-08-10T16:30:00Z".to_owned(),
    };
    (repo, request)
}

#[test]
fn direct_sdk_call_refuses_an_invalid_key_before_any_repository_access() {
    let sdk = HandbookSdkV1::open("this-path-is-never-read-for-an-invalid-key");
    let result = sdk.apply_posture_transition(PostureTransitionApplyRequestV1 {
        idempotency_key: String::new(),
        repository_identity_fingerprint: String::new(),
        expected_canonical: PostureCanonicalBindingV1 {
            reference: String::new(),
            fingerprint: String::new(),
            document_sha256: String::new(),
            byte_length: 0,
        },
        recommendation: reference(),
        reassessment_coverage_ids: Vec::new(),
        change: PostureDimensionChangeRequestV1 {
            dimension_id: String::new(),
            dimension_index: 0,
            authority_path: String::new(),
            expected_stored_value: None,
            expected_effective_level: 0,
            proposed_effective_level: 0,
        },
        effective_at_utc: String::new(),
    });

    assert_eq!(
        result,
        PostureTransitionApplyResultV1::Refused(
            PostureTransitionRefusalCodeV1::InvalidIdempotencyKey
        )
    );
}

#[test]
fn direct_sdk_method_applies_an_unfaulted_first_request_then_replays_exactly() {
    let (repo, request) = prepared_posture_request();
    let sdk = HandbookSdkV1::open(repo.path());
    let before_apply = durable_state_snapshot(repo.path());
    let applied = sdk.apply_posture_transition(request.clone());
    let PostureTransitionApplyResultV1::Applied(receipt) = applied else {
        panic!("unfaulted public SDK call must reach the private writer: {applied:?}")
    };
    assert_eq!(
        receipt.prior_canonical, request.expected_canonical,
        "the receipt must bind exactly the caller's accepted basis"
    );
    assert!(repo
        .path()
        .join(".handbook/state")
        .join(&receipt.posture_transition.reference)
        .is_file());
    assert!(repo
        .path()
        .join(".handbook/state")
        .join(&receipt.lifecycle_transition.reference)
        .is_file());
    let after_apply = durable_state_snapshot(repo.path());
    assert_ne!(after_apply, before_apply);
    assert!(
        transaction_entries(repo.path())
            .iter()
            .all(|entry| !entry.ends_with(".pending")),
        "the successful public call leaves no unresolved posture journal"
    );
    assert_eq!(
        sdk.apply_posture_transition(request),
        PostureTransitionApplyResultV1::Replayed(receipt)
    );
    assert_eq!(durable_state_snapshot(repo.path()), after_apply);
}

#[test]
fn direct_sdk_method_applies_replays_and_refuses_changed_or_stale_requests_without_extra_writes() {
    let (repo, request) = prepared_posture_request();
    let sdk = HandbookSdkV1::open(repo.path());
    let fault = PostureTransitionFaultInjectionGuardV1::after_canonical_install();
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            sdk.apply_posture_transition(request.clone())
        }))
        .is_err(),
        "the test-only fault must interrupt the real write without a typed SDK result"
    );
    drop(fault);

    let restarted_sdk = HandbookSdkV1::open(repo.path());
    let recovered = restarted_sdk.apply_posture_transition(request.clone());
    let PostureTransitionApplyResultV1::Replayed(receipt) = recovered else {
        panic!("restarted SDK must recover the real private writer path: {recovered:?}")
    };
    assert!(repo
        .path()
        .join(".handbook/state")
        .join(&receipt.posture_transition.reference)
        .is_file());
    assert!(repo
        .path()
        .join(".handbook/state")
        .join(&receipt.lifecycle_transition.reference)
        .is_file());

    let after_apply = transaction_entries(repo.path());
    let durable_after_apply = durable_state_snapshot(repo.path());
    assert_eq!(
        restarted_sdk.apply_posture_transition(request.clone()),
        PostureTransitionApplyResultV1::Replayed(receipt.clone())
    );
    assert_eq!(transaction_entries(repo.path()), after_apply);
    assert_eq!(durable_state_snapshot(repo.path()), durable_after_apply);

    let mut changed_key_request = request.clone();
    changed_key_request.effective_at_utc = "2026-08-10T16:31:00Z".to_owned();
    let before_changed_key_refusal = durable_state_snapshot(repo.path());
    assert_eq!(
        restarted_sdk.apply_posture_transition(changed_key_request),
        PostureTransitionApplyResultV1::Refused(PostureTransitionRefusalCodeV1::AuthorityConflict)
    );
    assert_eq!(transaction_entries(repo.path()), after_apply);
    assert_eq!(
        durable_state_snapshot(repo.path()),
        before_changed_key_refusal
    );

    let mut stale_request = request.clone();
    stale_request.idempotency_key = "sdk-posture-proof-0002".to_owned();
    let before_stale_refusal = durable_state_snapshot(repo.path());
    assert_eq!(
        restarted_sdk.apply_posture_transition(stale_request),
        PostureTransitionApplyResultV1::Refused(PostureTransitionRefusalCodeV1::StaleCanonical)
    );
    assert_eq!(transaction_entries(repo.path()), after_apply);
    assert_eq!(durable_state_snapshot(repo.path()), before_stale_refusal);

    let mut caller_identity_mismatch = request.clone();
    caller_identity_mismatch.repository_identity_fingerprint = format!("sha256:{}", "f".repeat(64));
    let before_caller_identity_refusal = durable_state_snapshot(repo.path());
    assert_eq!(
        restarted_sdk.apply_posture_transition(caller_identity_mismatch.clone()),
        PostureTransitionApplyResultV1::Refused(
            PostureTransitionRefusalCodeV1::RepositoryIdentityMismatch
        )
    );
    assert_eq!(
        durable_state_snapshot(repo.path()),
        before_caller_identity_refusal
    );

    fs::write(
        repo.path().join(".handbook/repository-identity.v1"),
        &caller_identity_mismatch.repository_identity_fingerprint,
    )
    .unwrap();
    let before_identity_refusal = durable_state_snapshot(repo.path());
    assert_eq!(
        restarted_sdk.apply_posture_transition(caller_identity_mismatch),
        PostureTransitionApplyResultV1::Refused(PostureTransitionRefusalCodeV1::AuthorityConflict)
    );
    assert_eq!(transaction_entries(repo.path()), after_apply);
    assert_eq!(durable_state_snapshot(repo.path()), before_identity_refusal);
}

fn transaction_entries(repo_root: &Path) -> BTreeSet<String> {
    let root = repo_root.join(".handbook/state/transactions/posture-transitions");
    fs::read_dir(root)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect()
}

fn durable_state_snapshot(repo_root: &Path) -> BTreeMap<String, String> {
    let root = repo_root.join(".handbook");
    let mut snapshot = BTreeMap::new();
    collect_durable_files(&root, &root, &mut snapshot);
    snapshot
}

fn collect_durable_files(root: &Path, current: &Path, snapshot: &mut BTreeMap<String, String>) {
    for entry in fs::read_dir(current).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = entry.metadata().unwrap();
        if metadata.is_dir() {
            collect_durable_files(root, &path, snapshot);
        } else if metadata.is_file() {
            let relative = path
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            snapshot.insert(
                relative,
                format!("sha256:{:x}", Sha256::digest(fs::read(path).unwrap())),
            );
        } else {
            panic!("durable fixture contains an unsupported filesystem entry");
        }
    }
}
