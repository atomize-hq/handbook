use handbook_engine::{
    encode_get_assertion_request, resolve_shipped_profile_decisions, ApproverAdminRequestV1,
    ApproverRegistryServiceV1, CharterApprovalRequestV1, CharterApprovalResultV1,
    CharterApprovalServiceV1, LineageRecordClassV1, NativeAuthenticatorPortErrorV1,
    NativeAuthenticatorPortV1, TrustedLineageStoreV1, AUTHENTICATOR_RP_ID,
};
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::Path;

const SECURITY_BOUNDARY_VECTORS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/security-boundary-limit-vectors-v1.0.json"
));
const WAIVER_ORDER_VECTORS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/waiver-acceptance-order-vectors-v1.0.json"
));
const ADMIN_VECTORS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/approver-admin-api-vectors-v1.0.json"
));
const RUNTIME_VECTORS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/runtime-record-fingerprint-vectors-v1.0.json"
));

#[derive(Default)]
struct ApprovalPort {
    get_calls: Vec<Vec<u8>>,
}

impl NativeAuthenticatorPortV1 for ApprovalPort {
    fn make_credential(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        panic!("Charter approval must not create credentials")
    }

    fn get_assertion(
        &mut self,
        request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        self.get_calls.push(request_cbor.to_vec());
        Err(NativeAuthenticatorPortErrorV1::Unavailable)
    }
}

struct DynamicApprovalPort {
    key: SigningKey,
    credential_id: Vec<u8>,
    make_calls: Vec<Vec<u8>>,
    get_calls: Vec<Vec<u8>>,
}

impl DynamicApprovalPort {
    fn new() -> Self {
        Self {
            key: SigningKey::from_slice(&[1_u8; 32]).unwrap(),
            credential_id: vec![0x11; 32],
            make_calls: Vec::new(),
            get_calls: Vec::new(),
        }
    }
}

impl NativeAuthenticatorPortV1 for DynamicApprovalPort {
    fn make_credential(
        &mut self,
        request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        self.make_calls.push(request_cbor.to_vec());
        Ok(make_credential_response(&self.key, &self.credential_id))
    }

    fn get_assertion(
        &mut self,
        request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        self.get_calls.push(request_cbor.to_vec());
        let marker = request_cbor
            .windows(3)
            .position(|window| window == [0x02, 0x58, 0x20])
            .expect("GetAssertion client-data hash marker");
        let client_data_hash: [u8; 32] = request_cbor[marker + 3..marker + 35].try_into().unwrap();
        Ok(assertion_response(
            &self.key,
            &self.credential_id,
            client_data_hash,
            self.get_calls.len() as u32,
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

fn seed_runtime_candidate(repo_root: &Path) -> (String, String) {
    let fixture: Value = serde_json::from_slice(RUNTIME_VECTORS).unwrap();
    let intake = fixture["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["record_class"] == "intake")
        .unwrap()["record"]
        .clone();
    let intake_bytes = serde_json_canonicalizer::to_vec(&intake).unwrap();
    TrustedLineageStoreV1::new(repo_root)
        .append_record(LineageRecordClassV1::Intake, &intake_bytes)
        .unwrap();
    let candidate = fixture["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["record_class"] == "candidate")
        .unwrap()["record"]
        .clone();
    let bytes = serde_json_canonicalizer::to_vec(&candidate).unwrap();
    let persisted = TrustedLineageStoreV1::new(repo_root)
        .append_record(LineageRecordClassV1::Candidate, &bytes)
        .unwrap();
    (persisted.relative_ref, persisted.fingerprint)
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

#[test]
fn dynamic_es256_approval_commits_exact_fake_port_request_and_journal_authority() {
    let temp = tempfile::tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped Charter decisions");
    let registry_port = DynamicApprovalPort::new();
    let mut registry = ApproverRegistryServiceV1::for_repository(temp.path(), registry_port);
    let bootstrap = registry.bootstrap_approver_registry(bootstrap_request());
    assert_eq!(bootstrap.to_json_value().unwrap()["status"], "succeeded");
    let port = registry.into_port();
    let (candidate_ref, candidate_fingerprint) = seed_runtime_candidate(temp.path());
    let mut service = CharterApprovalServiceV1::new(temp.path(), port);

    let result = service.approve_candidate(
        &decisions,
        CharterApprovalRequestV1 {
            operation_id: "dynamic-initial-approval".to_owned(),
            candidate_ref,
            approval_class: "Project owner approval".to_owned(),
            authority_ref: "Project owner".to_owned(),
            accepted_waiver_refs: vec![],
        },
    );

    let CharterApprovalResultV1::Succeeded(success) = result else {
        panic!("dynamic ES256 approval must succeed: {result:?}")
    };
    assert_eq!(success.changed_paths.len(), 5);
    let port = service.into_port();
    assert_eq!(port.make_calls.len(), 1);
    assert_eq!(port.get_calls.len(), 1);
    let assertion_bytes = std::fs::read(
        temp.path()
            .join(".handbook/state")
            .join(&success.assertion_ref),
    )
    .unwrap();
    let assertion: Value = serde_json::from_slice(&assertion_bytes).unwrap();
    let challenge = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        assertion["challenge_jcs_base64"].as_str().unwrap(),
    )
    .unwrap();
    let client_data_hash: [u8; 32] = Sha256::digest(challenge).into();
    assert_eq!(
        port.get_calls[0],
        encode_get_assertion_request(client_data_hash, &[vec![0x11; 32]]).unwrap()
    );
    assert_eq!(
        assertion["client_data_hash"],
        format!(
            "sha256:{}",
            client_data_hash
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        )
    );
    let committed = temp
        .path()
        .join(".handbook/state/transactions/approvals")
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
        .unwrap();
    let intent = std::fs::read(committed.join("intent.json")).unwrap();
    assert_eq!(
        std::fs::read(committed.join("committed")).unwrap(),
        format!("sha256:{:x}\n", Sha256::digest(intent)).into_bytes()
    );
    assert_eq!(candidate_fingerprint.len(), 71);
}

#[test]
fn incomplete_pre_intent_approval_journals_roll_back_before_authority_use() {
    for entry in ["empty.pending", "intent-tmp-only.pending"] {
        let temp = tempfile::tempdir().unwrap();
        let decisions = resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
            .expect("shipped Charter decisions");
        let registry_port = DynamicApprovalPort::new();
        let mut registry = ApproverRegistryServiceV1::for_repository(temp.path(), registry_port);
        let bootstrap = registry.bootstrap_approver_registry(bootstrap_request());
        assert_eq!(bootstrap.to_json_value().unwrap()["status"], "succeeded");
        let port = registry.into_port();
        let (candidate_ref, _) = seed_runtime_candidate(temp.path());
        let pending = temp
            .path()
            .join(".handbook/state/transactions/approvals")
            .join(entry);
        std::fs::create_dir_all(&pending).unwrap();
        if entry == "intent-tmp-only.pending" {
            std::fs::write(pending.join("intent.tmp"), b"partial intent").unwrap();
        }
        let mut service = CharterApprovalServiceV1::new(temp.path(), port);

        let result = service.approve_candidate(
            &decisions,
            CharterApprovalRequestV1 {
                operation_id: format!("recover-{entry}"),
                candidate_ref,
                approval_class: "Unrequired approval".to_owned(),
                authority_ref: "Project owner".to_owned(),
                accepted_waiver_refs: vec![],
            },
        );

        let CharterApprovalResultV1::Refused(refusal) = result else {
            panic!("unrequired pair must refuse after recovery")
        };
        assert_eq!(refusal.code.as_str(), "authorization_refused");
        assert!(!pending.exists(), "owned incomplete journal must roll back");
        assert!(service.into_port().get_calls.is_empty());
    }
}

#[test]
fn incomplete_staged_approval_rolls_back_mutable_head_and_preserves_orphans() {
    let temp = tempfile::tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped Charter decisions");
    let mut registry =
        ApproverRegistryServiceV1::for_repository(temp.path(), DynamicApprovalPort::new());
    let bootstrap = registry.bootstrap_approver_registry(bootstrap_request());
    assert_eq!(bootstrap.to_json_value().unwrap()["status"], "succeeded");
    let (candidate_ref, _) = seed_runtime_candidate(temp.path());
    let mut service = CharterApprovalServiceV1::new(temp.path(), registry.into_port());
    let first = service.approve_candidate(
        &decisions,
        CharterApprovalRequestV1 {
            operation_id: "seed-interrupted-approval".to_owned(),
            candidate_ref: candidate_ref.clone(),
            approval_class: "Project owner approval".to_owned(),
            authority_ref: "Project owner".to_owned(),
            accepted_waiver_refs: vec![],
        },
    );
    let CharterApprovalResultV1::Succeeded(success) = first else {
        panic!("seed approval must succeed: {first:?}")
    };
    let journal_root = temp.path().join(".handbook/state/transactions/approvals");
    let committed = journal_root
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
        .unwrap();
    let intent: Value =
        serde_json::from_slice(&std::fs::read(committed.join("intent.json")).unwrap()).unwrap();
    let prior_head = std::fs::read(committed.join("prior-use-head.json")).unwrap();
    let result_head = std::fs::read(committed.join("use-head.new")).unwrap();
    let use_head = temp
        .path()
        .join(".handbook/state")
        .join(intent["prior_use_head_ref"].as_str().unwrap());
    assert_eq!(std::fs::read(&use_head).unwrap(), result_head);
    let orphan = temp
        .path()
        .join(".handbook/state")
        .join(&success.assertion_ref);
    let orphan_bytes = std::fs::read(&orphan).unwrap();
    std::fs::remove_file(committed.join("committed")).unwrap();
    std::fs::remove_file(committed.join("assertion.new")).unwrap();
    let pending = committed.with_file_name(
        committed
            .file_name()
            .unwrap()
            .to_string_lossy()
            .replace(".committed", ".pending"),
    );
    std::fs::rename(&committed, &pending).unwrap();

    let result = service.approve_candidate(
        &decisions,
        CharterApprovalRequestV1 {
            operation_id: "recover-incomplete-stages".to_owned(),
            candidate_ref,
            approval_class: "Unrequired approval".to_owned(),
            authority_ref: "Project owner".to_owned(),
            accepted_waiver_refs: vec![],
        },
    );

    let CharterApprovalResultV1::Refused(refusal) = result else {
        panic!("unrequired pair must refuse after recovery")
    };
    assert_eq!(refusal.code.as_str(), "authorization_refused");
    assert!(!pending.exists());
    assert_eq!(std::fs::read(use_head).unwrap(), prior_head);
    assert_eq!(std::fs::read(orphan).unwrap(), orphan_bytes);
    assert_eq!(service.into_port().get_calls.len(), 1);
}

#[test]
fn forged_dynamic_es256_assertion_refuses_without_approval_authority() {
    let temp = tempfile::tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped Charter decisions");
    let mut registry =
        ApproverRegistryServiceV1::for_repository(temp.path(), DynamicApprovalPort::new());
    assert_eq!(
        registry
            .bootstrap_approver_registry(bootstrap_request())
            .to_json_value()
            .unwrap()["status"],
        "succeeded"
    );
    let mut port = registry.into_port();
    port.key = SigningKey::from_slice(&[2_u8; 32]).unwrap();
    let (candidate_ref, _) = seed_runtime_candidate(temp.path());
    let mut service = CharterApprovalServiceV1::new(temp.path(), port);

    let result = service.approve_candidate(
        &decisions,
        CharterApprovalRequestV1 {
            operation_id: "forged-assertion".to_owned(),
            candidate_ref,
            approval_class: "Project owner approval".to_owned(),
            authority_ref: "Project owner".to_owned(),
            accepted_waiver_refs: vec![],
        },
    );

    let CharterApprovalResultV1::Refused(refusal) = result else {
        panic!("forged assertion must refuse")
    };
    assert_eq!(refusal.code.as_str(), "authorization_refused");
    assert!(refusal.semantic_identities_are_null());
    assert_eq!(service.into_port().get_calls.len(), 1);
    assert!(!temp
        .path()
        .join(".handbook/state/transactions/approvals")
        .exists());
}

#[test]
fn replay_and_wrong_pair_refuse_without_additional_native_io() {
    let temp = tempfile::tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped Charter decisions");
    let mut registry =
        ApproverRegistryServiceV1::for_repository(temp.path(), DynamicApprovalPort::new());
    assert_eq!(
        registry
            .bootstrap_approver_registry(bootstrap_request())
            .to_json_value()
            .unwrap()["status"],
        "succeeded"
    );
    let (candidate_ref, _) = seed_runtime_candidate(temp.path());
    let mut service = CharterApprovalServiceV1::new(temp.path(), registry.into_port());
    let request = CharterApprovalRequestV1 {
        operation_id: "initial-use".to_owned(),
        candidate_ref: candidate_ref.clone(),
        approval_class: "Project owner approval".to_owned(),
        authority_ref: "Project owner".to_owned(),
        accepted_waiver_refs: vec![],
    };
    assert!(matches!(
        service.approve_candidate(&decisions, request.clone()),
        CharterApprovalResultV1::Succeeded(_)
    ));
    let replay = service.approve_candidate(
        &decisions,
        CharterApprovalRequestV1 {
            operation_id: "replayed-use".to_owned(),
            ..request.clone()
        },
    );
    let wrong_pair = service.approve_candidate(
        &decisions,
        CharterApprovalRequestV1 {
            operation_id: "wrong-pair".to_owned(),
            approval_class: "Unrequired approval".to_owned(),
            ..request
        },
    );
    for refusal in [replay, wrong_pair] {
        let CharterApprovalResultV1::Refused(refusal) = refusal else {
            panic!("replay and wrong pair must refuse")
        };
        assert_eq!(refusal.code.as_str(), "authorization_refused");
        assert!(refusal.semantic_identities_are_null());
    }
    assert_eq!(service.into_port().get_calls.len(), 1);
}

#[test]
fn stale_candidate_presence_refuses_before_native_io() {
    let temp = tempfile::tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped Charter decisions");
    let mut registry =
        ApproverRegistryServiceV1::for_repository(temp.path(), DynamicApprovalPort::new());
    assert_eq!(
        registry
            .bootstrap_approver_registry(bootstrap_request())
            .to_json_value()
            .unwrap()["status"],
        "succeeded"
    );
    let (candidate_ref, _) = seed_runtime_candidate(temp.path());
    let charter = temp.path().join(".handbook/project/charter.yaml");
    std::fs::create_dir_all(charter.parent().unwrap()).unwrap();
    std::fs::write(&charter, b"stale candidate basis").unwrap();
    let mut service = CharterApprovalServiceV1::new(temp.path(), registry.into_port());

    let result = service.approve_candidate(
        &decisions,
        CharterApprovalRequestV1 {
            operation_id: "stale-basis".to_owned(),
            candidate_ref,
            approval_class: "Project owner approval".to_owned(),
            authority_ref: "Project owner".to_owned(),
            accepted_waiver_refs: vec![],
        },
    );

    let CharterApprovalResultV1::Refused(refusal) = result else {
        panic!("stale candidate must refuse")
    };
    assert_eq!(refusal.code.as_str(), "stale_basis");
    assert!(refusal.semantic_identities_are_null());
    assert!(service.into_port().get_calls.is_empty());
}

#[test]
fn committed_journal_missing_required_final_record_is_not_authority() {
    let temp = tempfile::tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped Charter decisions");
    let mut registry =
        ApproverRegistryServiceV1::for_repository(temp.path(), DynamicApprovalPort::new());
    assert_eq!(
        registry
            .bootstrap_approver_registry(bootstrap_request())
            .to_json_value()
            .unwrap()["status"],
        "succeeded"
    );
    let (candidate_ref, _) = seed_runtime_candidate(temp.path());
    let mut service = CharterApprovalServiceV1::new(temp.path(), registry.into_port());
    let request = CharterApprovalRequestV1 {
        operation_id: "seed-committed-journal".to_owned(),
        candidate_ref,
        approval_class: "Project owner approval".to_owned(),
        authority_ref: "Project owner".to_owned(),
        accepted_waiver_refs: vec![],
    };
    let success = service.approve_candidate(&decisions, request.clone());
    let CharterApprovalResultV1::Succeeded(success) = success else {
        panic!("seed approval must succeed: {success:?}")
    };
    std::fs::remove_file(
        temp.path()
            .join(".handbook/state")
            .join(success.assertion_ref),
    )
    .unwrap();

    let observed = service.approve_candidate(
        &decisions,
        CharterApprovalRequestV1 {
            operation_id: "observe-broken-journal".to_owned(),
            ..request
        },
    );

    let CharterApprovalResultV1::Refused(refusal) = observed else {
        panic!("broken committed journal must refuse")
    };
    assert_eq!(refusal.code.as_str(), "lineage_violation");
    assert!(refusal.semantic_identities_are_null());
    assert_eq!(service.into_port().get_calls.len(), 1);
}

#[test]
fn invalid_caller_intent_is_all_null_and_refuses_before_native_io() {
    let temp = tempfile::tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped Charter decisions");
    let mut service = CharterApprovalServiceV1::new(temp.path(), ApprovalPort::default());

    let result = service.approve_candidate(
        &decisions,
        CharterApprovalRequestV1 {
            operation_id: String::new(),
            candidate_ref: "candidate.json".to_owned(),
            approval_class: "Project owner approval".to_owned(),
            authority_ref: "Project owner".to_owned(),
            accepted_waiver_refs: vec![],
        },
    );

    let CharterApprovalResultV1::Refused(refusal) = result else {
        panic!("invalid caller intent must refuse")
    };
    assert_eq!(refusal.code.as_str(), "invalid_request");
    assert!(refusal.changed_paths.is_empty());
    assert!(refusal.semantic_identities_are_null());
    assert!(service.into_port().get_calls.is_empty());
}

#[test]
fn frozen_approval_boundary_vector_rejects_one_over_before_native_io() {
    let vectors: serde_json::Value = serde_json::from_str(SECURITY_BOUNDARY_VECTORS).unwrap();
    let approval_rows = vectors["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|row| row["schema_file"] == "authenticator-challenge-1.0.0.schema.json")
        .collect::<Vec<_>>();
    assert_eq!(approval_rows.len(), 2);
    let waiver_row = approval_rows
        .iter()
        .find(|row| row["field"] == "accepted_waiver_fingerprints")
        .unwrap();
    assert_eq!(waiver_row["accepted_size"], 10);
    assert_eq!(waiver_row["one_over_rejected_size"], 11);
    let nonce_row = approval_rows
        .iter()
        .find(|row| row["field"] == "nonce_base64")
        .unwrap();
    assert_eq!(nonce_row["accepted_size"], 44);
    assert_eq!(nonce_row["one_over_rejected_size"], 45);

    let temp = tempfile::tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped Charter decisions");
    let mut service = CharterApprovalServiceV1::new(temp.path(), ApprovalPort::default());
    let result = service.approve_candidate(
        &decisions,
        CharterApprovalRequestV1 {
            operation_id: "approval-boundary-one-over".to_owned(),
            candidate_ref: format!("candidates/candidate_{}.json", "0".repeat(64)),
            approval_class: "Project owner approval".to_owned(),
            authority_ref: "Project owner".to_owned(),
            accepted_waiver_refs: (0..11)
                .map(|index| format!("waivers/waiver_{index:064x}.json"))
                .collect(),
        },
    );
    let CharterApprovalResultV1::Refused(refusal) = result else {
        panic!("one-over waiver admission bound must refuse")
    };
    assert_eq!(refusal.code.as_str(), "invalid_request");
    assert!(refusal.semantic_identities_are_null());
    assert!(refusal.changed_paths.is_empty());
    assert!(service.into_port().get_calls.is_empty());
}

#[test]
fn frozen_waiver_order_vectors_are_closed_and_self_consistent() {
    let vectors: serde_json::Value = serde_json::from_str(WAIVER_ORDER_VECTORS).unwrap();
    assert_eq!(vectors["schema_version"], "1.0");
    assert_eq!(
        vectors["normalization_vectors"].as_array().unwrap().len(),
        2
    );
    assert_eq!(vectors["negative_vectors"].as_array().unwrap().len(), 4);

    for vector in vectors["normalization_vectors"].as_array().unwrap() {
        let mut entries = vector["input_entries"].as_array().unwrap().clone();
        entries.sort_by(|left, right| {
            left["waiver_ref"]
                .as_str()
                .unwrap()
                .as_bytes()
                .cmp(right["waiver_ref"].as_str().unwrap().as_bytes())
                .then_with(|| {
                    left["waiver_fingerprint"]
                        .as_str()
                        .unwrap()
                        .as_bytes()
                        .cmp(right["waiver_fingerprint"].as_str().unwrap().as_bytes())
                })
        });
        assert_eq!(
            entries.as_slice(),
            vector["expected_approval_entries"].as_array().unwrap()
        );
        let mut challenge = entries
            .iter()
            .map(|entry| entry["waiver_fingerprint"].clone())
            .collect::<Vec<_>>();
        challenge.sort_by(|left, right| {
            left.as_str()
                .unwrap()
                .as_bytes()
                .cmp(right.as_str().unwrap().as_bytes())
        });
        assert_eq!(
            challenge.as_slice(),
            vector["expected_challenge_fingerprints"]
                .as_array()
                .unwrap()
        );
    }

    let negative_ids = vectors["negative_vectors"]
        .as_array()
        .unwrap()
        .iter()
        .map(|vector| vector["vector_id"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        negative_ids,
        std::collections::BTreeSet::from([
            "duplicate-challenge-fingerprint",
            "duplicate-waiver-fingerprint",
            "duplicate-waiver-ref",
            "unsorted-challenge-fingerprints",
        ])
    );
}
