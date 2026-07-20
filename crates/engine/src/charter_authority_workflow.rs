use crate::approver_registry::{ApproverAdminRequestV1, ApproverAdminResultV1};
use crate::canonical_repo_support::CanonicalWorkspace;
use crate::charter_authenticator::{
    decode_make_credential_response, derive_authenticator_user_handle,
    encode_make_credential_request, AuthenticatorErrorV1, MakeCredentialResponseV1,
    NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1, AUTHENTICATOR_RP_ID,
};
use crate::charter_intake::{evaluate_charter_intake, CharterIntakeEnvelope, CharterIntakeError};
use crate::charter_lineage_store::{
    create_new_file, create_safe_directories, reject_reparse_or_symlink, sync_directory,
    LineageRecordClassV1, LineageStoreErrorV1, TrustedLineageStoreV1,
};
use crate::{DefinitionFingerprint, ResolvedProfileDecisions};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const REGISTRATION_REQUEST_SCHEMA_REF: &str =
    "handbook.schemas.security.authenticator-registration-request@1.0.0";
const MAKE_CREDENTIAL_RESPONSE_SCHEMA_REF: &str =
    "handbook.schemas.security.authenticator-make-credential-response@1.0.0";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharterAuthorPersistenceErrorKindV1 {
    IntakeRefused,
    PersistenceRefused,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterAuthorPersistenceErrorV1 {
    kind: CharterAuthorPersistenceErrorKindV1,
    detail: String,
}

impl CharterAuthorPersistenceErrorV1 {
    pub fn kind(&self) -> CharterAuthorPersistenceErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl fmt::Display for CharterAuthorPersistenceErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for CharterAuthorPersistenceErrorV1 {}

impl From<CharterIntakeError> for CharterAuthorPersistenceErrorV1 {
    fn from(error: CharterIntakeError) -> Self {
        Self {
            kind: CharterAuthorPersistenceErrorKindV1::IntakeRefused,
            detail: error.detail().to_owned(),
        }
    }
}

impl From<LineageStoreErrorV1> for CharterAuthorPersistenceErrorV1 {
    fn from(error: LineageStoreErrorV1) -> Self {
        Self {
            kind: CharterAuthorPersistenceErrorKindV1::PersistenceRefused,
            detail: error.detail().to_owned(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterAuthorPersistenceResultV1 {
    pub intake_ref: String,
    pub candidate_ref: String,
    pub candidate_fingerprint: String,
    pub normalized_content_ref: String,
}

#[derive(Clone, Debug)]
pub struct CharterAuthorPersistenceServiceV1 {
    lineage: TrustedLineageStoreV1,
}

impl CharterAuthorPersistenceServiceV1 {
    pub fn new(repo_root: impl AsRef<Path>) -> Self {
        Self {
            lineage: TrustedLineageStoreV1::new(repo_root),
        }
    }

    pub fn persist(
        &self,
        decisions: &ResolvedProfileDecisions,
        envelope: CharterIntakeEnvelope,
        observed_current_fingerprint: Option<&DefinitionFingerprint>,
    ) -> Result<CharterAuthorPersistenceResultV1, CharterAuthorPersistenceErrorV1> {
        let bundle = evaluate_charter_intake(decisions, envelope, observed_current_fingerprint)?;
        let candidate_fingerprint = bundle.candidate.candidate_fingerprint.clone();
        let persisted = self.lineage.persist_candidate_bundle(&bundle)?;
        Ok(CharterAuthorPersistenceResultV1 {
            intake_ref: persisted.intake_ref,
            candidate_ref: persisted.candidate_ref,
            candidate_fingerprint,
            normalized_content_ref: persisted.normalized_content_ref,
        })
    }
}

pub(crate) fn bootstrap_registry<P: NativeAuthenticatorPortV1>(
    repo_root: &Path,
    port: &mut P,
    request: &ApproverAdminRequestV1,
) -> ApproverAdminResultV1 {
    match bootstrap_registry_inner(repo_root, port, request) {
        Ok(success) => success,
        Err(refusal) => refused_result(request, refusal),
    }
}

pub(crate) struct RegistryRefusal {
    code: &'static str,
    message: String,
    retryable: bool,
    next_action: String,
}

impl RegistryRefusal {
    fn new(
        code: &'static str,
        message: impl Into<String>,
        retryable: bool,
        next_action: impl Into<String>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            retryable,
            next_action: next_action.into(),
        }
    }
}

fn bootstrap_registry_inner<P: NativeAuthenticatorPortV1>(
    repo_root: &Path,
    port: &mut P,
    request: &ApproverAdminRequestV1,
) -> Result<ApproverAdminResultV1, RegistryRefusal> {
    if request.operation() != "bootstrap" {
        return Err(invalid("bootstrap endpoint requires bootstrap operation"));
    }
    let document = request.document();
    let mut quorum = document["initial_charter_quorum"]
        .as_array()
        .expect("validated bootstrap quorum")
        .clone();
    sort_mappings(&mut quorum);
    if quorum.iter().any(|mapping| {
        mapping["approval_class"] == "registry_admin"
            && mapping["authority_ref"] == "repository_registry_admin"
    }) {
        return Err(invalid(
            "bootstrap quorum duplicates the engine-owned registry administrator mapping",
        ));
    }
    let mut mappings = quorum.clone();
    mappings.push(json!({
        "approval_class": "registry_admin",
        "authority_ref": "repository_registry_admin"
    }));
    sort_mappings(&mut mappings);

    let _locks = RegistryAuthorityLocks::acquire(repo_root)?;
    recover_registry_authority(repo_root)?;
    if registry_authority_exists(repo_root)? {
        return Err(RegistryRefusal::new(
            "transaction_conflict",
            "approver registry is already bootstrapped",
            false,
            "retain the existing committed registry authority",
        ));
    }

    let user_handle = derive_authenticator_user_handle(request.repository_identity_fingerprint())
        .map_err(|_| invalid("repository identity is not admissible"))?;
    let nonce = random_nonce()?;
    let registration_request = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "allowed_algorithms": ["ES256"],
        "approval_mappings": mappings,
        "attestation_policy": "none",
        "extensions": {},
        "initial_charter_quorum": quorum,
        "nonce_base64": BASE64_STANDARD.encode(nonce),
        "operation": "bootstrap",
        "operation_id": request.operation_id(),
        "protocol_id": "fido2-ctap2.1-native",
        "repository_identity_fingerprint": request.repository_identity_fingerprint(),
        "rp": {"id": AUTHENTICATOR_RP_ID, "name": "Handbook repository authority"},
        "schema_id": "handbook.authenticator-registration-request",
        "schema_version": "1.0",
        "user_handle_hash": user_handle.hash,
        "user_verification": "required"
    });
    let request_jcs = serde_json_canonicalizer::to_vec(&registration_request)
        .map_err(|_| invalid("registration request could not be canonicalized"))?;
    let client_data_hash: [u8; 32] = Sha256::digest(&request_jcs).into();
    let request_cbor = encode_make_credential_request(client_data_hash, user_handle.bytes)
        .map_err(|_| invalid("make-credential request could not be encoded"))?;

    let raw_response = port
        .make_credential(&request_cbor)
        .map_err(native_port_refusal)?;
    let decoded = decode_make_credential_response(&raw_response).map_err(registration_refusal)?;
    let audit_utc = current_audit_utc();

    let response_record = make_credential_response_record(&decoded)?;
    let registration_record = registration_record(
        &decoded,
        &request_jcs,
        client_data_hash,
        &user_handle.hash,
        &response_record,
        &audit_utc,
    )?;
    let registry_state = bootstrap_registry_state_record(
        request,
        document["initial_charter_quorum"]
            .as_array()
            .expect("validated quorum"),
        &registration_request["approval_mappings"],
        &decoded,
        &registration_record,
    )?;
    let transition = bootstrap_registry_transition_record(
        request,
        &decoded,
        &registration_record,
        &registry_state,
        &audit_utc,
    )?;
    let use_head = initial_use_head_record(&decoded)?;

    let lineage = TrustedLineageStoreV1::new(repo_root);
    for (class, record) in [
        (
            LineageRecordClassV1::AuthenticatorMakeCredentialResponse,
            &response_record,
        ),
        (
            LineageRecordClassV1::AuthenticatorRegistration,
            &registration_record,
        ),
        (LineageRecordClassV1::RegistryState, &registry_state),
        (LineageRecordClassV1::RegistryTransition, &transition),
    ] {
        lineage
            .preflight_record(class, &record.bytes)
            .map_err(lineage_refusal)?;
    }
    let committed_dir = persist_bootstrap_transaction(
        repo_root,
        request,
        &lineage,
        &response_record,
        &registration_record,
        &registry_state,
        &transition,
        &use_head,
    )?;
    if !committed_dir.join("committed").is_file() {
        return Err(durability(
            "registry commit marker is absent after finalization",
        ));
    }

    ApproverAdminResultV1::from_json_value(json!({
        "assertion_fingerprint": null,
        "assertion_ref": null,
        "changed_paths": [
            response_record.relative_ref,
            registration_record.relative_ref,
            registry_state.relative_ref,
            transition.relative_ref
        ],
        "next_actions": ["retain the committed transition reference"],
        "operation": "bootstrap",
        "operation_id": request.operation_id(),
        "prior_registry_state_fingerprint": null,
        "prior_registry_state_ref": null,
        "refusal": null,
        "registration_fingerprint": registration_record.fingerprint,
        "registration_ref": registration_record.relative_ref,
        "repository_identity_fingerprint": request.repository_identity_fingerprint(),
        "result_registry_state_fingerprint": registry_state.fingerprint,
        "result_registry_state_ref": registry_state.relative_ref,
        "schema_id": "handbook.approver-admin-result",
        "schema_version": "1.0",
        "status": "succeeded",
        "transition_fingerprint": transition.fingerprint,
        "transition_ref": transition.relative_ref
    }))
    .map_err(|_| durability("engine-built bootstrap result violated its closed schema"))
}

#[derive(Clone)]
struct BuiltRecord {
    value: Value,
    bytes: Vec<u8>,
    relative_ref: String,
    fingerprint: String,
}

fn make_credential_response_record(
    decoded: &MakeCredentialResponseV1,
) -> Result<BuiltRecord, RegistryRefusal> {
    let rp_id_hash = DefinitionFingerprint::from_bytes(AUTHENTICATOR_RP_ID.as_bytes()).to_string();
    build_record(
        json!({
            "aaguid_base64": BASE64_STANDARD.encode(decoded.aaguid),
            "algorithm": "ES256",
            "attestation_format": "none",
            "attestation_object_base64": BASE64_STANDARD.encode(&decoded.attestation_object),
            "attestation_statement_cbor_base64": "oA==",
            "attested_credential_data_included": true,
            "authenticator_data_base64": BASE64_STANDARD.encode(&decoded.authenticator_data),
            "cose_public_key_base64": BASE64_STANDARD.encode(&decoded.cose_public_key),
            "credential_id_base64": BASE64_STANDARD.encode(&decoded.credential_id),
            "credential_id_hash": decoded.credential_id_hash,
            "flags_byte": decoded.flags_byte,
            "protocol_id": "fido2-ctap2.1-native",
            "rp_id_hash": rp_id_hash,
            "schema_id": "handbook.authenticator-make-credential-response",
            "schema_version": "1.0",
            "sign_count": decoded.sign_count,
            "user_present": true,
            "user_verified": true
        }),
        "response_fingerprint",
        Some(("response_id", "authenticator-make-credential-response")),
        &[],
        "authenticator-make-credential-responses",
    )
}

fn registration_record(
    decoded: &MakeCredentialResponseV1,
    request_jcs: &[u8],
    client_data_hash: [u8; 32],
    user_handle_hash: &str,
    response: &BuiltRecord,
    audit_utc: &str,
) -> Result<BuiltRecord, RegistryRefusal> {
    build_record(
        json!({
            "algorithm": "ES256",
            "attestation_format": "none",
            "attestation_object_base64": BASE64_STANDARD.encode(&decoded.attestation_object),
            "attestation_trust": "none",
            "authenticator_data_base64": BASE64_STANDARD.encode(&decoded.authenticator_data),
            "client_data_hash": fingerprint_digest(client_data_hash),
            "cose_public_key_base64": BASE64_STANDARD.encode(&decoded.cose_public_key),
            "credential_id_hash": decoded.credential_id_hash,
            "decoded_response_fingerprint": response.fingerprint,
            "decoded_response_ref": response.relative_ref,
            "decoded_response_schema_ref": MAKE_CREDENTIAL_RESPONSE_SCHEMA_REF,
            "protocol_id": "fido2-ctap2.1-native",
            "registered_at_utc": audit_utc,
            "request_fingerprint": DefinitionFingerprint::from_bytes(request_jcs).to_string(),
            "request_jcs_base64": BASE64_STANDARD.encode(request_jcs),
            "request_schema_ref": REGISTRATION_REQUEST_SCHEMA_REF,
            "rp_id": AUTHENTICATOR_RP_ID,
            "schema_id": "handbook.authenticator-registration",
            "schema_version": "1.0",
            "sign_count": decoded.sign_count,
            "user_handle_hash": user_handle_hash,
            "user_present": true,
            "user_verified": true
        }),
        "registration_fingerprint",
        Some(("registration_id", "authenticator-registration")),
        &["registered_at_utc"],
        "authenticator-registrations",
    )
}

fn bootstrap_registry_state_record(
    request: &ApproverAdminRequestV1,
    quorum: &[Value],
    mappings: &Value,
    decoded: &MakeCredentialResponseV1,
    registration: &BuiltRecord,
) -> Result<BuiltRecord, RegistryRefusal> {
    build_record(
        json!({
            "credentials": [{
                "algorithm": "ES256",
                "approval_mappings": mappings,
                "cose_public_key_base64": BASE64_STANDARD.encode(&decoded.cose_public_key),
                "credential_id_hash": decoded.credential_id_hash,
                "extensions": {},
                "registration_fingerprint": registration.fingerprint,
                "registration_ref": registration.relative_ref,
                "status": "active"
            }],
            "extensions": {},
            "initial_charter_quorum": quorum,
            "registry_id": "repository-approvers",
            "registry_version": 1,
            "repository_identity_fingerprint": request.repository_identity_fingerprint(),
            "schema_id": "handbook.approver-registry-state",
            "schema_version": "1.0"
        }),
        "registry_state_fingerprint",
        None,
        &[],
        "registry-states",
    )
}

fn bootstrap_registry_transition_record(
    request: &ApproverAdminRequestV1,
    decoded: &MakeCredentialResponseV1,
    registration: &BuiltRecord,
    registry_state: &BuiltRecord,
    audit_utc: &str,
) -> Result<BuiltRecord, RegistryRefusal> {
    build_record(
        json!({
            "admin_credential_id_hash": null,
            "authorization_fingerprint": registration.fingerprint,
            "authorization_kind": "registration",
            "authorization_ref": registration.relative_ref,
            "changed_credential_id_hash": decoded.credential_id_hash,
            "occurred_at_utc": audit_utc,
            "operation": "bootstrap",
            "prior_registry_state_fingerprint": null,
            "prior_registry_state_ref": null,
            "prior_transition_fingerprint": null,
            "prior_transition_ref": null,
            "repository_identity_fingerprint": request.repository_identity_fingerprint(),
            "result_registry_state_fingerprint": registry_state.fingerprint,
            "result_registry_state_ref": registry_state.relative_ref,
            "schema_id": "handbook.approver-registry-transition",
            "schema_version": "1.0"
        }),
        "transition_fingerprint",
        Some(("transition_id", "registry-transition")),
        &["occurred_at_utc"],
        "registry-transitions",
    )
}

fn initial_use_head_record(
    decoded: &MakeCredentialResponseV1,
) -> Result<BuiltRecord, RegistryRefusal> {
    let hex = decoded
        .credential_id_hash
        .strip_prefix("sha256:")
        .ok_or_else(|| invalid("credential ID hash is not normalized"))?;
    build_record(
        json!({
            "$schema": "handbook.schemas.security.authenticator-use-head@1.0.0",
            "credential_id_hash": decoded.credential_id_hash,
            "last_assertion_fingerprint": null,
            "last_assertion_ref": null,
            "last_challenge_fingerprint": null,
            "last_nonce_sha256": null,
            "schema_version": "1.0",
            "sequence": 0,
            "sign_count": decoded.sign_count
        }),
        "head_fingerprint",
        None,
        &[],
        &format!("authenticator-use-heads/{hex}.json"),
    )
}

fn build_record(
    mut value: Value,
    fingerprint_field: &str,
    id: Option<(&str, &str)>,
    audit_only: &[&str],
    partition: &str,
) -> Result<BuiltRecord, RegistryRefusal> {
    let mut preimage = value.clone();
    let object = preimage
        .as_object_mut()
        .ok_or_else(|| durability("engine-built record did not have an object root"))?;
    object.remove(fingerprint_field);
    if let Some((id_field, _)) = id {
        object.remove(id_field);
    }
    for field in audit_only {
        object.remove(*field);
    }
    let fingerprint = DefinitionFingerprint::from_json_value(&preimage)
        .map_err(|_| durability("engine-built record could not be fingerprinted"))?
        .to_string();
    value[fingerprint_field] = Value::String(fingerprint.clone());
    let hex = fingerprint
        .strip_prefix("sha256:")
        .expect("normalized fingerprint");
    let relative_ref = if let Some((id_field, prefix)) = id {
        let record_id = format!("{prefix}_{hex}");
        value[id_field] = Value::String(record_id.clone());
        format!("{partition}/{record_id}.json")
    } else if partition.ends_with(".json") {
        partition.to_owned()
    } else {
        format!("{partition}/registry-state_{hex}.json")
    };
    let bytes = serde_json_canonicalizer::to_vec(&value)
        .map_err(|_| durability("engine-built record could not be canonicalized"))?;
    Ok(BuiltRecord {
        value,
        bytes,
        relative_ref,
        fingerprint,
    })
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RegistryBootstrapIntentV1 {
    schema_id: String,
    schema_version: String,
    transaction_id: String,
    operation: String,
    operation_id: String,
    repository_identity_fingerprint: String,
    response_ref: String,
    response_fingerprint: String,
    registration_ref: String,
    registration_fingerprint: String,
    result_registry_state_ref: String,
    result_registry_state_fingerprint: String,
    transition_ref: String,
    transition_fingerprint: String,
    use_head_ref: String,
    use_head_fingerprint: String,
}

#[allow(clippy::too_many_arguments)]
fn persist_bootstrap_transaction(
    repo_root: &Path,
    request: &ApproverAdminRequestV1,
    lineage: &TrustedLineageStoreV1,
    response: &BuiltRecord,
    registration: &BuiltRecord,
    registry_state: &BuiltRecord,
    transition: &BuiltRecord,
    use_head: &BuiltRecord,
) -> Result<PathBuf, RegistryRefusal> {
    debug_assert_eq!(
        response.value["schema_id"],
        "handbook.authenticator-make-credential-response"
    );
    let transaction_root = repo_root.join(".handbook/state/transactions/registry");
    create_safe_directories(repo_root, &transaction_root).map_err(lineage_refusal)?;
    let request_bytes = serde_json_canonicalizer::to_vec(request.document())
        .map_err(|_| invalid("bootstrap request could not be canonicalized"))?;
    let transaction_id = format!("registry_{:x}", Sha256::digest(&request_bytes));
    let pending = transaction_root.join(format!("{transaction_id}.pending"));
    let committed = transaction_root.join(format!("{transaction_id}.committed"));
    if pending.exists() || committed.exists() {
        return Err(RegistryRefusal::new(
            "transaction_conflict",
            "bootstrap transaction already exists",
            false,
            "retain the existing committed bootstrap result",
        ));
    }
    fs::create_dir(&pending).map_err(|_| durability("registry pending directory create failed"))?;
    sync_directory(&transaction_root).map_err(lineage_refusal)?;
    let intent = RegistryBootstrapIntentV1 {
        schema_id: "handbook.registry-transaction-intent".to_owned(),
        schema_version: "1.0".to_owned(),
        transaction_id,
        operation: "bootstrap".to_owned(),
        operation_id: request.operation_id().to_owned(),
        repository_identity_fingerprint: request.repository_identity_fingerprint().to_owned(),
        response_ref: response.relative_ref.clone(),
        response_fingerprint: response.fingerprint.clone(),
        registration_ref: registration.relative_ref.clone(),
        registration_fingerprint: registration.fingerprint.clone(),
        result_registry_state_ref: registry_state.relative_ref.clone(),
        result_registry_state_fingerprint: registry_state.fingerprint.clone(),
        transition_ref: transition.relative_ref.clone(),
        transition_fingerprint: transition.fingerprint.clone(),
        use_head_ref: use_head.relative_ref.clone(),
        use_head_fingerprint: use_head.fingerprint.clone(),
    };
    let intent_bytes = serde_json_canonicalizer::to_vec(&intent)
        .map_err(|_| durability("registry intent canonicalization failed"))?;
    write_new_durable(&pending.join("intent.tmp"), &intent_bytes)?;
    fs::rename(pending.join("intent.tmp"), pending.join("intent.json"))
        .map_err(|_| durability("registry intent rename failed"))?;
    sync_directory(&pending).map_err(lineage_refusal)?;

    for (class, record) in [
        (
            LineageRecordClassV1::AuthenticatorMakeCredentialResponse,
            response,
        ),
        (
            LineageRecordClassV1::AuthenticatorRegistration,
            registration,
        ),
        (LineageRecordClassV1::RegistryState, registry_state),
        (LineageRecordClassV1::RegistryTransition, transition),
    ] {
        lineage
            .append_record(class, &record.bytes)
            .map_err(lineage_refusal)?;
    }
    let use_head_path = repo_root
        .join(".handbook/state")
        .join(&use_head.relative_ref);
    let use_head_parent = use_head_path
        .parent()
        .ok_or_else(|| durability("authenticator use head has no parent"))?;
    create_safe_directories(repo_root, use_head_parent).map_err(lineage_refusal)?;
    write_new_durable(&use_head_path, &use_head.bytes)?;
    sync_directory(use_head_parent).map_err(lineage_refusal)?;

    let marker = registry_commit_marker(&intent_bytes);
    write_new_durable(&pending.join("committed.tmp"), &marker)?;
    fs::rename(pending.join("committed.tmp"), pending.join("committed"))
        .map_err(|_| durability("registry commit-marker rename failed"))?;
    sync_directory(&pending).map_err(lineage_refusal)?;
    fs::rename(&pending, &committed)
        .map_err(|_| durability("registry transaction finalization rename failed"))?;
    sync_directory(&transaction_root).map_err(lineage_refusal)?;
    Ok(committed)
}

fn write_new_durable(path: &Path, bytes: &[u8]) -> Result<(), RegistryRefusal> {
    let mut file =
        create_new_file(path).map_err(|_| durability("authority file create-new failed"))?;
    file.write_all(bytes)
        .map_err(|_| durability("authority file write failed"))?;
    file.sync_all()
        .map_err(|_| durability("authority file flush failed"))
}

fn registry_authority_exists(repo_root: &Path) -> Result<bool, RegistryRefusal> {
    let root = repo_root.join(".handbook/state/transactions/registry");
    if !root.exists() {
        return Ok(false);
    }
    reject_reparse_or_symlink(&root, true).map_err(lineage_refusal)?;
    for entry in
        fs::read_dir(&root).map_err(|_| durability("registry transaction directory read failed"))?
    {
        let entry = entry.map_err(|_| durability("registry transaction entry read failed"))?;
        let name = entry
            .file_name()
            .to_str()
            .ok_or_else(|| durability("registry transaction entry name is not UTF-8"))?
            .to_owned();
        if name.ends_with(".committed") {
            reject_reparse_or_symlink(&entry.path(), true).map_err(lineage_refusal)?;
            return Ok(true);
        }
    }
    Ok(false)
}

pub(crate) fn recover_registry_authority(repo_root: &Path) -> Result<(), RegistryRefusal> {
    let root = repo_root.join(".handbook/state/transactions/registry");
    if !root.exists() {
        return Ok(());
    }
    reject_reparse_or_symlink(&root, true).map_err(lineage_refusal)?;
    let mut pending = Vec::new();
    for entry in
        fs::read_dir(&root).map_err(|_| durability("registry transaction directory read failed"))?
    {
        let entry = entry.map_err(|_| durability("registry transaction entry read failed"))?;
        let name = entry
            .file_name()
            .to_str()
            .ok_or_else(|| durability("registry transaction entry name is not UTF-8"))?
            .to_owned();
        if name.ends_with(".pending") {
            reject_reparse_or_symlink(&entry.path(), true).map_err(lineage_refusal)?;
            pending.push(entry);
        }
    }
    pending.sort_by_key(|entry| entry.file_name());
    for entry in pending {
        let directory = entry.path();
        let intent_bytes = read_registry_journal_file(
            repo_root,
            &directory.join("intent.json"),
            64 * 1024,
            "pending registry intent",
        )?;
        let intent: RegistryBootstrapIntentV1 = serde_json::from_slice(&intent_bytes)
            .map_err(|_| durability("pending registry intent is not closed JSON"))?;
        let canonical = serde_json_canonicalizer::to_vec(&intent)
            .map_err(|_| durability("pending registry intent cannot be canonicalized"))?;
        if canonical != intent_bytes {
            return Err(durability("pending registry intent is not exact JCS"));
        }
        let marker_path = directory.join("committed");
        if !marker_path.exists() {
            return Err(durability(
                "incomplete registry transaction requires operator recovery before a new ceremony",
            ));
        }
        let marker = read_registry_journal_file(
            repo_root,
            &marker_path,
            72,
            "pending registry commit marker",
        )?;
        if marker != registry_commit_marker(&intent_bytes) || marker.len() != 72 {
            return Err(durability("pending registry commit marker is not exact"));
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        let committed_name = format!("{}.committed", name.trim_end_matches(".pending"));
        fs::rename(&directory, root.join(committed_name))
            .map_err(|_| durability("committed registry journal finalization failed"))?;
        sync_directory(&root).map_err(lineage_refusal)?;
    }
    Ok(())
}

pub(crate) fn read_registry_journal_file(
    repo_root: &Path,
    path: &Path,
    limit: usize,
    label: &str,
) -> Result<Vec<u8>, RegistryRefusal> {
    let relative = path
        .strip_prefix(repo_root)
        .map_err(|_| durability(format!("{label} escapes the repository root")))?;
    let relative = relative
        .to_str()
        .ok_or_else(|| durability(format!("{label} path is not UTF-8")))?
        .replace('\\', "/");
    let workspace = CanonicalWorkspace::new(repo_root);
    let normalized = workspace
        .normalize_repo_relative(&relative)
        .map_err(|_| durability(format!("{label} path is not normalized")))?;
    let file = workspace
        .trusted_read_strict(&normalized)
        .map_err(|_| durability(format!("{label} is not a safe regular file")))?;
    let (bytes, exceeded) = file
        .read_bytes_bounded(limit)
        .map_err(|_| durability(format!("{label} could not be read")))?;
    if exceeded {
        return Err(durability(format!("{label} exceeds its exact bound")));
    }
    Ok(bytes)
}

pub(crate) struct RegistryAuthorityLocks {
    files: Vec<File>,
}

impl RegistryAuthorityLocks {
    pub(crate) fn acquire(repo_root: &Path) -> Result<Self, RegistryRefusal> {
        let lock_root = repo_root.join(".handbook/state/locks");
        create_safe_directories(repo_root, &lock_root).map_err(lineage_refusal)?;
        let mut files = Vec::new();
        for name in ["promotion.lock", "registry.lock"] {
            let file = open_authority_lock_file(&lock_root.join(name))?;
            file.lock()
                .map_err(|_| durability("authority lock acquisition failed"))?;
            files.push(file);
        }
        Ok(Self { files })
    }
}

fn open_authority_lock_file(path: &Path) -> Result<File, RegistryRefusal> {
    match create_new_file(path) {
        Ok(file) => return Ok(file),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(_) => return Err(durability("authority lock file create failed")),
    }
    reject_reparse_or_symlink(path, false).map_err(lineage_refusal)?;
    let mut options = OpenOptions::new();
    options.read(true).write(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(rustix::fs::OFlags::NOFOLLOW.bits() as i32);
    }
    let file = options
        .open(path)
        .map_err(|_| durability("authority lock file open failed"))?;
    if !file
        .metadata()
        .map_err(|_| durability("authority lock metadata failed"))?
        .is_file()
    {
        return Err(durability("authority lock is not a regular file"));
    }
    Ok(file)
}

impl Drop for RegistryAuthorityLocks {
    fn drop(&mut self) {
        for file in self.files.iter().rev() {
            let _ = file.unlock();
        }
    }
}

pub(crate) fn registry_commit_marker(intent_bytes: &[u8]) -> Vec<u8> {
    format!("sha256:{:x}\n", Sha256::digest(intent_bytes)).into_bytes()
}

fn random_nonce() -> Result<[u8; 32], RegistryRefusal> {
    let mut nonce = [0u8; 32];
    getrandom::fill(&mut nonce).map_err(|_| {
        RegistryRefusal::new(
            "transaction_conflict",
            "operating-system randomness was unavailable",
            true,
            "retry the complete operation with fresh operating-system randomness",
        )
    })?;
    Ok(nonce)
}

fn sort_mappings(mappings: &mut [Value]) {
    mappings.sort_by(|left, right| {
        left["approval_class"]
            .as_str()
            .expect("validated approval class")
            .cmp(
                right["approval_class"]
                    .as_str()
                    .expect("validated approval class"),
            )
            .then_with(|| {
                left["authority_ref"]
                    .as_str()
                    .expect("validated authority ref")
                    .cmp(
                        right["authority_ref"]
                            .as_str()
                            .expect("validated authority ref"),
                    )
            })
    });
}

fn fingerprint_digest(digest: [u8; 32]) -> String {
    format!("sha256:{}", hex_lower(&digest))
}

fn hex_lower(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn current_audit_utc() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0);
    let days = seconds.div_euclid(86_400);
    let seconds_of_day = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_date_from_unix_days(days);
    let hour = seconds_of_day / 3_600;
    let minute = seconds_of_day % 3_600 / 60;
    let second = seconds_of_day % 60;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

fn civil_date_from_unix_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = if shifted >= 0 {
        shifted
    } else {
        shifted - 146_096
    } / 146_097;
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    if month <= 2 {
        year += 1;
    }
    (year, month, day)
}

fn refused_result(
    request: &ApproverAdminRequestV1,
    refusal: RegistryRefusal,
) -> ApproverAdminResultV1 {
    ApproverAdminResultV1::from_json_value(json!({
        "assertion_fingerprint": null,
        "assertion_ref": null,
        "changed_paths": [],
        "next_actions": [refusal.next_action],
        "operation": request.operation(),
        "operation_id": request.operation_id(),
        "prior_registry_state_fingerprint": null,
        "prior_registry_state_ref": null,
        "refusal": {
            "code": refusal.code,
            "message": refusal.message,
            "retryable": refusal.retryable
        },
        "registration_fingerprint": null,
        "registration_ref": null,
        "repository_identity_fingerprint": request.repository_identity_fingerprint(),
        "result_registry_state_fingerprint": null,
        "result_registry_state_ref": null,
        "schema_id": "handbook.approver-admin-result",
        "schema_version": "1.0",
        "status": "refused",
        "transition_fingerprint": null,
        "transition_ref": null
    }))
    .expect("engine-built refusal must satisfy the closed Admin result schema")
}

fn invalid(message: impl Into<String>) -> RegistryRefusal {
    RegistryRefusal::new(
        "invalid_request",
        message,
        false,
        "correct the request before retry",
    )
}

fn durability(message: impl Into<String>) -> RegistryRefusal {
    RegistryRefusal::new(
        "transaction_conflict",
        message,
        true,
        "preserve the authority state for operator diagnosis before retry",
    )
}

fn native_port_refusal(error: NativeAuthenticatorPortErrorV1) -> RegistryRefusal {
    match error {
        NativeAuthenticatorPortErrorV1::Unavailable | NativeAuthenticatorPortErrorV1::Transport => {
            RegistryRefusal::new(
                "AUTHENTICATOR_UNAVAILABLE",
                "native CTAP2.1 authenticator API is unavailable",
                true,
                "retry only when the native authenticator API is available",
            )
        }
    }
}

fn registration_refusal(error: AuthenticatorErrorV1) -> RegistryRefusal {
    match error {
        AuthenticatorErrorV1::Refused(refusal) => RegistryRefusal::new(
            refusal.code.as_str(),
            format!("authenticator refused status 0x{:02x}", refusal.status_byte),
            refusal.retryable,
            refusal.next_action,
        ),
        _ => RegistryRefusal::new(
            "registration_invalid",
            "authenticator response was invalid",
            false,
            "correct or replace the authenticator before retry",
        ),
    }
}

fn lineage_refusal(error: LineageStoreErrorV1) -> RegistryRefusal {
    durability(error.detail().to_owned())
}
