use handbook_engine::{
    observe_committed_approver_registry, ApproverAdminRequestV1, ApproverRegistryServiceV1,
    CredentialAuthorityAdmissionErrorKindV1, CredentialAuthorityAdmissionRequestV1,
    CredentialAuthorityAdmissionServiceV1, NativeAuthenticatorPortErrorV1,
    NativeAuthenticatorPortV1, RetainedRepositoryRootV1, AUTHENTICATOR_RP_ID,
};
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

const REPOSITORY_IDENTITY: &str =
    "sha256:1111111111111111111111111111111111111111111111111111111111111111";

struct AuthorityPort {
    key: SigningKey,
    credential_id: Vec<u8>,
    make_calls: usize,
    get_calls: usize,
    forge_assertion: bool,
    corrupt_state_path: Option<PathBuf>,
    transport_failure: bool,
}

impl AuthorityPort {
    fn new() -> Self {
        Self {
            key: SigningKey::from_slice(&[7_u8; 32]).unwrap(),
            credential_id: vec![0x32; 32],
            make_calls: 0,
            get_calls: 0,
            forge_assertion: false,
            corrupt_state_path: None,
            transport_failure: false,
        }
    }
}

impl NativeAuthenticatorPortV1 for AuthorityPort {
    fn make_credential(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        self.make_calls += 1;
        Ok(make_credential_response(&self.key, &self.credential_id))
    }

    fn get_assertion(
        &mut self,
        request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        self.get_calls += 1;
        if self.transport_failure {
            return Err(NativeAuthenticatorPortErrorV1::Transport);
        }
        let marker = request_cbor
            .windows(3)
            .position(|window| window == [0x02, 0x58, 0x20])
            .expect("GetAssertion client-data hash marker");
        let client_data_hash = request_cbor[marker + 3..marker + 35].try_into().unwrap();
        let key = if self.forge_assertion {
            SigningKey::from_slice(&[9_u8; 32]).unwrap()
        } else {
            self.key.clone()
        };
        let response = assertion_response(
            &key,
            &self.credential_id,
            client_data_hash,
            self.get_calls as u32,
        );
        if let Some(path) = self.corrupt_state_path.take() {
            std::fs::write(path, b"{}").unwrap();
        }
        Ok(response)
    }
}

fn bootstrap(
    quorum: Vec<serde_json::Value>,
    forge_assertion: bool,
) -> (
    tempfile::TempDir,
    CredentialAuthorityAdmissionServiceV1<AuthorityPort>,
) {
    let repo = tempfile::tempdir().unwrap();
    let mut port = AuthorityPort::new();
    port.forge_assertion = forge_assertion;
    let mut registry = ApproverRegistryServiceV1::for_repository(repo.path(), port);
    let request = ApproverAdminRequestV1::from_json_value(json!({
        "schema_id":"handbook.approver-admin-request",
        "schema_version":"1.0",
        "operation":"bootstrap",
        "operation_id":"hcm-5-1-authority-bootstrap",
        "repository_identity_fingerprint":REPOSITORY_IDENTITY,
        "expected_registry_state_fingerprint":null,
        "expected_transition_fingerprint":null,
        "initial_charter_quorum":quorum
    }))
    .unwrap();
    let result = registry
        .bootstrap_approver_registry(request)
        .to_json_value()
        .unwrap();
    assert_eq!(result["status"], "succeeded", "{result:#}");
    let service = CredentialAuthorityAdmissionServiceV1::new(repo.path(), registry.into_port());
    (repo, service)
}

fn request(approval_class: &str) -> CredentialAuthorityAdmissionRequestV1 {
    CredentialAuthorityAdmissionRequestV1::new(
        "handbook.contract-lifecycle-transition@1.0.0",
        "handbook.contract.example@1.0.0",
        approval_class,
        &format!("sha256:{}", "a".repeat(64)),
    )
    .unwrap()
}

#[test]
fn sealed_admission_binds_authenticated_actor_and_exact_current_authority() {
    let mapping = json!({
        "approval_class":"contract_definition_author",
        "authority_ref":"handbook.authority.contract-definition-author@1.0.0"
    });
    let (repo, mut service) = bootstrap(vec![mapping], false);
    let observation = observe_committed_approver_registry(repo.path()).unwrap();
    let expected_state_ref = observation.state_ref.clone();
    let expected_state_fingerprint = observation.state_fingerprint.clone();
    let expected_head_ref = observation.head_transition_ref.clone();
    let expected_head_fingerprint = observation.head_transition_fingerprint.clone();
    drop(observation);

    let evidence = service
        .admit(request("contract_definition_author"), |admission| {
            (
                admission.actor_ref().to_owned(),
                admission.approval_class().to_owned(),
                admission.authority_ref().to_owned(),
                admission.repository_identity_fingerprint().to_owned(),
                admission.registry_state_ref().to_owned(),
                admission.registry_state_fingerprint().to_owned(),
                admission.registry_head_transition_ref().to_owned(),
                admission.registry_head_transition_fingerprint().to_owned(),
                admission.semantic_scope_fingerprint().to_owned(),
                admission.semantic_decision_time_utc().to_owned(),
            )
        })
        .unwrap();

    assert!(evidence.0.starts_with("credential.sha256-"));
    assert_eq!(evidence.1, "contract_definition_author");
    assert_eq!(
        evidence.2,
        "handbook.authority.contract-definition-author@1.0.0"
    );
    assert_eq!(evidence.3, REPOSITORY_IDENTITY);
    assert_eq!(evidence.4, expected_state_ref);
    assert_eq!(evidence.5, expected_state_fingerprint);
    assert_eq!(evidence.6, expected_head_ref);
    assert_eq!(evidence.7, expected_head_fingerprint);
    assert_eq!(evidence.8, format!("sha256:{}", "a".repeat(64)));
    assert!(evidence.9.ends_with('Z'));
}

#[test]
fn sealed_admission_requires_the_same_physical_repository_root() {
    let mapping = json!({
        "approval_class":"contract_definition_author",
        "authority_ref":"handbook.authority.contract-definition-author@1.0.0"
    });
    let (source, source_service) = bootstrap(vec![mapping], false);
    let clone = tempfile::tempdir().unwrap();
    copy_directory_tree(source.path(), clone.path());
    let mut clone_service =
        CredentialAuthorityAdmissionServiceV1::new(clone.path(), source_service.into_port());
    let source_root = RetainedRepositoryRootV1::open_exact(source.path()).unwrap();
    let clone_root = RetainedRepositoryRootV1::open_exact(clone.path()).unwrap();

    let (same, substituted) = clone_service
        .admit(request("contract_definition_author"), |admission| {
            (
                admission.require_repository_root(&clone_root),
                admission.require_repository_root(&source_root),
            )
        })
        .unwrap();

    assert!(same.is_ok());
    assert_eq!(
        substituted.unwrap_err().kind(),
        CredentialAuthorityAdmissionErrorKindV1::DurabilityViolation
    );
}

#[test]
fn retained_authority_tree_blocks_or_refuses_substitution_during_consume() {
    let mapping = json!({
        "approval_class":"contract_definition_author",
        "authority_ref":"handbook.authority.contract-definition-author@1.0.0"
    });
    let (repo, mut service) = bootstrap(vec![mapping], false);
    let state = repo.path().join(".handbook/state");
    let authority = state.join("transactions/registry");
    let substitute = state.join("substitute-authority");
    let parked = state.join("parked-authority");
    std::fs::create_dir(&substitute).unwrap();

    let result = service.admit(request("contract_definition_author"), |_| {
        #[cfg(windows)]
        {
            std::fs::rename(&authority, &parked).is_err()
        }
        #[cfg(unix)]
        {
            std::fs::rename(&authority, &parked).unwrap();
            std::fs::rename(&substitute, &authority).unwrap();
            std::fs::rename(&authority, &substitute).unwrap();
            std::fs::rename(&parked, &authority).unwrap();
            true
        }
    });

    #[cfg(windows)]
    assert!(result.unwrap());
    #[cfg(unix)]
    assert_eq!(
        result.unwrap_err().kind(),
        CredentialAuthorityAdmissionErrorKindV1::DurabilityViolation
    );
}

#[test]
fn retained_authority_tree_refuses_in_place_state_mutation_during_consume() {
    let mapping = json!({
        "approval_class":"contract_definition_author",
        "authority_ref":"handbook.authority.contract-definition-author@1.0.0"
    });
    let (repo, mut service) = bootstrap(vec![mapping], false);
    let observation = observe_committed_approver_registry(repo.path()).unwrap();
    let state_path = repo
        .path()
        .join(".handbook/state")
        .join(&observation.state_ref);
    drop(observation);
    let mut mutated = std::fs::read(&state_path).unwrap();
    mutated[0] ^= 1;

    let result = service.admit(request("contract_definition_author"), |_| {
        std::fs::write(&state_path, &mutated).unwrap();
        "must-not-be-received"
    });

    assert_eq!(
        result.unwrap_err().kind(),
        CredentialAuthorityAdmissionErrorKindV1::DurabilityViolation
    );
}

#[test]
fn missing_or_multiply_resolved_authority_refuses_before_authentication() {
    let first = json!({
        "approval_class":"contract_definition_author",
        "authority_ref":"handbook.authority.first@1.0.0"
    });
    let second = json!({
        "approval_class":"contract_definition_author",
        "authority_ref":"handbook.authority.second@1.0.0"
    });
    let (_repo, mut service) = bootstrap(vec![first, second], false);

    let ambiguous = service
        .admit(request("contract_definition_author"), |_| ())
        .unwrap_err();
    assert_eq!(
        ambiguous.kind(),
        CredentialAuthorityAdmissionErrorKindV1::AuthorityAmbiguous
    );
    let missing = service
        .admit(request("contract_independent_lock"), |_| ())
        .unwrap_err();
    assert_eq!(
        missing.kind(),
        CredentialAuthorityAdmissionErrorKindV1::AuthorityAbsent
    );
    assert_eq!(service.into_port().get_calls, 0);
}

#[test]
fn forged_credential_assertion_produces_no_admission() {
    let mapping = json!({
        "approval_class":"contract_activation",
        "authority_ref":"handbook.authority.contract-activation@1.0.0"
    });
    let (_repo, mut service) = bootstrap(vec![mapping], true);

    let refusal = service
        .admit(request("contract_activation"), |_| panic!("must not admit"))
        .unwrap_err();
    assert_eq!(
        refusal.kind(),
        CredentialAuthorityAdmissionErrorKindV1::AuthenticationRefused
    );
}

#[test]
fn stale_current_head_after_authentication_produces_no_admission() {
    let mapping = json!({
        "approval_class":"contract_activation",
        "authority_ref":"handbook.authority.contract-activation@1.0.0"
    });
    let (repo, service) = bootstrap(vec![mapping], false);
    let observation = observe_committed_approver_registry(repo.path()).unwrap();
    let state_path = repo
        .path()
        .join(".handbook/state")
        .join(&observation.state_ref);
    drop(observation);
    let mut port = service.into_port();
    port.corrupt_state_path = Some(state_path);
    let mut service = CredentialAuthorityAdmissionServiceV1::new(repo.path(), port);

    let refusal = service
        .admit(request("contract_activation"), |_| panic!("must not admit"))
        .unwrap_err();
    assert_eq!(
        refusal.kind(),
        CredentialAuthorityAdmissionErrorKindV1::StaleAuthority
    );
}

#[test]
fn absent_committed_authority_is_distinct_from_durability_uncertainty() {
    let repo = tempfile::tempdir().unwrap();
    let mut service = CredentialAuthorityAdmissionServiceV1::new(repo.path(), AuthorityPort::new());

    let refusal = service
        .admit(request("contract_activation"), |_| panic!("must not admit"))
        .unwrap_err();
    assert_eq!(
        refusal.kind(),
        CredentialAuthorityAdmissionErrorKindV1::AuthorityAbsent
    );
    assert_eq!(service.into_port().get_calls, 0);
}

#[test]
fn durability_uncertain_authenticator_transport_produces_no_admission() {
    let mapping = json!({
        "approval_class":"contract_activation",
        "authority_ref":"handbook.authority.contract-activation@1.0.0"
    });
    let (repo, service) = bootstrap(vec![mapping], false);
    let mut port = service.into_port();
    port.transport_failure = true;
    let mut service = CredentialAuthorityAdmissionServiceV1::new(repo.path(), port);

    let refusal = service
        .admit(request("contract_activation"), |_| panic!("must not admit"))
        .unwrap_err();
    assert_eq!(
        refusal.kind(),
        CredentialAuthorityAdmissionErrorKindV1::DurabilityViolation
    );
}

#[test]
fn request_values_are_closed_and_cannot_supply_actor_or_authority() {
    assert!(CredentialAuthorityAdmissionRequestV1::new(
        "latest",
        "handbook.contract.example@1.0.0",
        "contract_activation",
        &format!("sha256:{}", "a".repeat(64)),
    )
    .is_err());
    assert!(CredentialAuthorityAdmissionRequestV1::new(
        "handbook.contract-lifecycle-transition@1.0.0",
        "../authority",
        "contract_activation",
        &format!("sha256:{}", "a".repeat(64)),
    )
    .is_err());
    assert!(CredentialAuthorityAdmissionRequestV1::new(
        "handbook.contract-lifecycle-transition@1.0.0",
        "handbook.contract.example@1.0.0",
        "contract_activation",
        &format!("sha256:{}", "A".repeat(64)),
    )
    .is_err());
}

fn copy_directory_tree(source: &std::path::Path, target: &std::path::Path) {
    for entry in std::fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let destination = target.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            std::fs::create_dir(&destination).unwrap();
            copy_directory_tree(&entry.path(), &destination);
        } else {
            std::fs::copy(entry.path(), destination).unwrap();
        }
    }
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
