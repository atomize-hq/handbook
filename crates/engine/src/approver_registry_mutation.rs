use crate::approver_registry::{ApproverAdminRequestV1, ApproverAdminResultV1};
use crate::approver_registry_observation::{
    ApproverAuthorityPairV1, CommittedApproverCredentialV1,
};
use crate::canonical_repo_support::CanonicalWorkspace;
use crate::charter_artifact::{validate_canonical_charter_semantics, CanonicalCharter};
use crate::charter_authenticator::{
    decode_and_verify_get_assertion_response, decode_make_credential_response,
    derive_authenticator_user_handle, encode_get_assertion_request, encode_make_credential_request,
    AuthenticatorCredentialV1, AuthenticatorErrorV1, GetAssertionResponseV1,
    MakeCredentialResponseV1, NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1,
    AUTHENTICATOR_RP_ID,
};
use crate::charter_authority_workflow::{
    read_registry_journal_file, recover_registry_authority, registry_commit_marker,
    RegistryAuthorityLocks,
};
use crate::charter_lineage_store::{
    create_new_file, create_safe_directories, reject_reparse_or_symlink, sync_directory,
    LineageRecordClassV1, TrustedLineageStoreV1,
};
use crate::{parse_definition_yaml, parse_schema_json, DefinitionFingerprint};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_APPROVAL_MAPPINGS: usize = 64;
const MAX_REGISTRY_CREDENTIALS: usize = 256;
#[cfg(test)]
const MAX_COSE_PUBLIC_KEY_BASE64: usize = 4_096;
#[cfg(test)]
const MAX_NEXT_ACTIONS: usize = 16;
const MAX_RECORD_BYTES: usize = 1024 * 1024;
const REGISTRY_TRANSACTION_ROOT: &str = ".handbook/state/transactions/registry";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MutationKind {
    Add,
    Revoke,
    Update,
}

impl MutationKind {
    fn operation(self) -> &'static str {
        match self {
            Self::Add => "add_credential",
            Self::Revoke => "revoke_credential",
            Self::Update => "update_mapping",
        }
    }
}

#[derive(Clone)]
struct BuiltRecord {
    value: Value,
    bytes: Vec<u8>,
    relative_ref: String,
    fingerprint: String,
}

struct CurrentAuthority {
    state_ref: String,
    state_fingerprint: String,
    state_bytes: Vec<u8>,
    state: Value,
    transition_ref: String,
    transition_fingerprint: String,
    transition_bytes: Vec<u8>,
    credentials: Vec<CommittedApproverCredentialV1>,
    _locks: RegistryAuthorityLocks,
}

struct RegistrationRecords {
    response: BuiltRecord,
    registration: BuiltRecord,
    decoded: MakeCredentialResponseV1,
    initial_use_head: BuiltRecord,
}

struct AssertionRecords {
    response: BuiltRecord,
    assertion: BuiltRecord,
    use_transition: BuiltRecord,
    result_use_head: BuiltRecord,
    prior_use_head_bytes: Vec<u8>,
    decoded: GetAssertionResponseV1,
}

#[derive(Serialize)]
struct RegistryMutationIntent {
    schema_id: &'static str,
    schema_version: &'static str,
    transaction_id: String,
    operation: String,
    operation_id: String,
    repository_identity_fingerprint: String,
    prior_registry_state_ref: String,
    prior_registry_state_fingerprint: String,
    prior_transition_ref: String,
    prior_transition_fingerprint: String,
    response_ref: Option<String>,
    response_fingerprint: Option<String>,
    registration_ref: Option<String>,
    registration_fingerprint: Option<String>,
    assertion_response_ref: String,
    assertion_response_fingerprint: String,
    assertion_ref: String,
    assertion_fingerprint: String,
    result_registry_state_ref: String,
    result_registry_state_fingerprint: String,
    transition_ref: String,
    transition_fingerprint: String,
    use_transition_ref: String,
    use_transition_fingerprint: String,
    prior_use_head_ref: String,
    prior_use_head_fingerprint: String,
    prior_use_head_bytes_fingerprint: String,
    result_use_head_ref: String,
    result_use_head_fingerprint: String,
    result_use_head_bytes_fingerprint: String,
    new_credential_use_head_ref: Option<String>,
    new_credential_use_head_fingerprint: Option<String>,
}

#[derive(Debug)]
struct MutationRefusal {
    code: &'static str,
    message: String,
    retryable: bool,
    next_action: String,
}

pub(crate) fn add_credential<P: NativeAuthenticatorPortV1>(
    repo_root: &Path,
    port: &mut P,
    request: &ApproverAdminRequestV1,
) -> ApproverAdminResultV1 {
    mutate(repo_root, port, request, MutationKind::Add)
}

pub(crate) fn revoke_credential<P: NativeAuthenticatorPortV1>(
    repo_root: &Path,
    port: &mut P,
    request: &ApproverAdminRequestV1,
) -> ApproverAdminResultV1 {
    mutate(repo_root, port, request, MutationKind::Revoke)
}

pub(crate) fn update_mapping<P: NativeAuthenticatorPortV1>(
    repo_root: &Path,
    port: &mut P,
    request: &ApproverAdminRequestV1,
) -> ApproverAdminResultV1 {
    mutate(repo_root, port, request, MutationKind::Update)
}

fn mutate<P: NativeAuthenticatorPortV1>(
    repo_root: &Path,
    port: &mut P,
    request: &ApproverAdminRequestV1,
    kind: MutationKind,
) -> ApproverAdminResultV1 {
    if request.operation() != kind.operation() {
        return refused(
            request,
            "invalid_request",
            "operation does not match endpoint",
            false,
            "invoke the endpoint matching the closed request operation",
        );
    }
    match mutate_inner(repo_root, port, request, kind) {
        Ok(result) => result,
        Err(error) => refused(
            request,
            error.code,
            &error.message,
            error.retryable,
            &error.next_action,
        ),
    }
}

fn mutate_inner<P: NativeAuthenticatorPortV1>(
    repo_root: &Path,
    port: &mut P,
    request: &ApproverAdminRequestV1,
    kind: MutationKind,
) -> Result<ApproverAdminResultV1, MutationRefusal> {
    let authority = observe_current_authority(repo_root)?;
    crate::charter_approval_workflow::recover_approval_authority(repo_root)
        .map_err(|_| durability("pending approval/use-head authority could not be recovered"))?;
    validate_expected_authority(request, &authority)?;
    let required_pairs = required_pairs(repo_root, &authority)?;
    let requested_mappings = request_mappings(request)?;
    let target_hash = request
        .document()
        .get("credential_id_hash")
        .and_then(Value::as_str);

    let provisional_eligible = eligible_administrators(
        &authority,
        kind,
        target_hash,
        &requested_mappings,
        &required_pairs,
    )?;
    if provisional_eligible.is_empty() {
        return Err(lockout_or_authorization(kind));
    }

    let registration = if kind == MutationKind::Add {
        Some(perform_registration(
            port,
            request,
            &authority,
            &requested_mappings,
        )?)
    } else {
        None
    };
    if let Some(registration) = &registration {
        validate_distinct_registration(&authority, &registration.decoded)?;
    }

    let result_state = build_result_state(
        request,
        &authority,
        kind,
        target_hash,
        &requested_mappings,
        registration.as_ref(),
    )?;
    let eligible = eligible_administrators(
        &authority,
        kind,
        target_hash,
        &requested_mappings,
        &required_pairs,
    )?;
    let changed_hash = registration
        .as_ref()
        .map(|record| record.decoded.credential_id_hash.as_str())
        .or(target_hash)
        .ok_or_else(|| invalid("changed credential identity is absent"))?;
    let assertion = perform_assertion(
        repo_root,
        port,
        request,
        &authority,
        &eligible,
        &result_state,
        changed_hash,
    )?;
    let administrator = eligible
        .iter()
        .find(|credential| credential.credential_id == assertion.decoded.credential_id)
        .ok_or_else(|| authorization("assertion selected no eligible retained administrator"))?;
    validate_counter(administrator.sign_count, assertion.decoded.sign_count)?;
    ensure_result_coverage(
        &authority,
        &result_state,
        administrator,
        registration.as_ref(),
        &required_pairs,
    )?;
    ensure_authority_stable(
        repo_root,
        &authority,
        administrator,
        &assertion.prior_use_head_bytes,
    )?;

    let transition = build_registry_transition(
        request,
        &authority,
        kind,
        target_hash,
        registration.as_ref(),
        &assertion,
        &result_state,
        administrator,
    )?;
    commit_mutation(
        repo_root,
        request,
        &authority,
        registration.as_ref(),
        &assertion,
        &result_state,
        &transition,
        administrator,
    )?;
    success_result(
        request,
        &authority,
        registration.as_ref(),
        &assertion,
        &result_state,
        &transition,
    )
}

#[cfg(test)]
fn final_use_add_would_lock_out(
    authorizing_sequence: u16,
    required_pairs: &[(&str, &str)],
    proposed_pairs: &[(&str, &str)],
) -> bool {
    authorizing_sequence == 4_095
        && required_pairs
            .iter()
            .any(|required| !proposed_pairs.iter().any(|proposed| proposed == required))
}

fn refused(
    request: &ApproverAdminRequestV1,
    code: &str,
    message: &str,
    retryable: bool,
    next_action: &str,
) -> ApproverAdminResultV1 {
    ApproverAdminResultV1::from_json_value(json!({
        "assertion_fingerprint": null,
        "assertion_ref": null,
        "changed_paths": [],
        "next_actions": [next_action],
        "operation": request.operation(),
        "operation_id": request.operation_id(),
        "prior_registry_state_fingerprint": null,
        "prior_registry_state_ref": null,
        "refusal": {"code": code, "message": message, "retryable": retryable},
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
    .expect("engine-built registry refusal must satisfy the closed Admin result schema")
}

fn observe_current_authority(repo_root: &Path) -> Result<CurrentAuthority, MutationRefusal> {
    let locks = RegistryAuthorityLocks::acquire(repo_root)
        .map_err(|_| durability("registry authority locks could not be acquired safely"))?;
    recover_registry_authority(repo_root)
        .map_err(|_| durability("pending registry authority could not be recovered"))?;
    let root = repo_root.join(REGISTRY_TRANSACTION_ROOT);
    if !root.exists() {
        return Err(authority_absent(
            "no committed approver registry authority exists",
        ));
    }
    let mut nodes = BTreeMap::<String, (PathBuf, Value, Vec<u8>, Value, Vec<u8>)>::new();
    let mut referenced_priors = BTreeSet::new();
    for entry in
        fs::read_dir(&root).map_err(|_| durability("registry transaction root is unreadable"))?
    {
        let entry = entry.map_err(|_| durability("registry transaction entry is unreadable"))?;
        let name = entry
            .file_name()
            .to_str()
            .ok_or_else(|| durability("registry transaction name is not UTF-8"))?
            .to_owned();
        if name.ends_with(".pending") {
            return Err(durability(
                "an unrecovered registry mutation journal blocks new native authority",
            ));
        }
        if !name.ends_with(".committed") {
            return Err(durability(
                "registry transaction root contains an unknown entry",
            ));
        }
        let directory = entry.path();
        let intent_bytes = read_registry_journal_file(
            repo_root,
            &directory.join("intent.json"),
            64 * 1024,
            "committed registry intent",
        )
        .map_err(|_| durability("committed registry intent is not a safe bounded file"))?;
        let intent = parse_schema_json(&intent_bytes)
            .map_err(|_| durability("committed registry intent is not closed JSON"))?;
        if serde_json_canonicalizer::to_vec(&intent)
            .map_err(|_| durability("committed registry intent is not canonicalizable"))?
            != intent_bytes
        {
            return Err(durability("committed registry intent is not exact JCS"));
        }
        let marker = read_registry_journal_file(
            repo_root,
            &directory.join("committed"),
            72,
            "committed registry marker",
        )
        .map_err(|_| durability("committed registry marker is unsafe"))?;
        if marker.len() != 72 || marker != registry_commit_marker(&intent_bytes) {
            return Err(durability(
                "committed registry marker does not bind its raw intent",
            ));
        }
        let transition_ref = required_string(&intent, "transition_ref")?.to_owned();
        let transition_fingerprint = required_string(&intent, "transition_fingerprint")?.to_owned();
        let lineage_store = TrustedLineageStoreV1::new(repo_root);
        let transition_bytes = lineage_store
            .read_record(
                LineageRecordClassV1::RegistryTransition,
                &transition_ref,
                &transition_fingerprint,
            )
            .map_err(|_| lineage("committed registry transition could not be re-resolved"))?;
        let transition = parse_schema_json(&transition_bytes)
            .map_err(|_| lineage("committed registry transition is not closed JSON"))?;
        if transition
            .get("transition_fingerprint")
            .and_then(Value::as_str)
            != Some(transition_fingerprint.as_str())
            || transition.get("result_registry_state_ref")
                != intent.get("result_registry_state_ref")
            || transition.get("result_registry_state_fingerprint")
                != intent.get("result_registry_state_fingerprint")
            || transition.get("operation") != intent.get("operation")
        {
            return Err(lineage("registry intent and transition bindings disagree"));
        }
        if let Some(prior) = transition
            .get("prior_transition_fingerprint")
            .and_then(Value::as_str)
        {
            referenced_priors.insert(prior.to_owned());
        }
        if nodes
            .insert(
                transition_fingerprint,
                (
                    directory,
                    intent,
                    intent_bytes,
                    transition,
                    transition_bytes,
                ),
            )
            .is_some()
        {
            return Err(lineage("registry transition fingerprint is duplicated"));
        }
    }
    if nodes.is_empty() {
        return Err(authority_absent(
            "no committed approver registry authority exists",
        ));
    }
    let heads = nodes
        .keys()
        .filter(|fingerprint| !referenced_priors.contains(*fingerprint))
        .cloned()
        .collect::<Vec<_>>();
    if heads.len() != 1 {
        return Err(lineage(
            "committed registry transitions do not have one unique head",
        ));
    }
    validate_registry_chain(&nodes, &heads[0])?;
    let (_, intent, _, _transition, transition_bytes) = &nodes[&heads[0]];
    let state_ref = required_string(intent, "result_registry_state_ref")?.to_owned();
    let state_fingerprint =
        required_string(intent, "result_registry_state_fingerprint")?.to_owned();
    let lineage_store = TrustedLineageStoreV1::new(repo_root);
    let state_bytes = lineage_store
        .read_record(
            LineageRecordClassV1::RegistryState,
            &state_ref,
            &state_fingerprint,
        )
        .map_err(|_| lineage("current registry state could not be re-resolved"))?;
    let state = parse_schema_json(&state_bytes)
        .map_err(|_| lineage("current registry state is not closed JSON"))?;
    if state
        .get("registry_state_fingerprint")
        .and_then(Value::as_str)
        != Some(state_fingerprint.as_str())
        || state.get("repository_identity_fingerprint")
            != intent.get("repository_identity_fingerprint")
    {
        return Err(lineage("current registry state identity is inconsistent"));
    }
    let credentials = resolve_credentials(repo_root, &lineage_store, &state)?;
    Ok(CurrentAuthority {
        state_ref,
        state_fingerprint,
        state_bytes,
        state,
        transition_ref: required_string(intent, "transition_ref")?.to_owned(),
        transition_fingerprint: heads[0].clone(),
        transition_bytes: transition_bytes.clone(),
        credentials,
        _locks: locks,
    })
}

#[allow(
    clippy::type_complexity,
    reason = "the retained tuple positions bind exact path, state, and transition bytes during chain validation"
)]
fn validate_registry_chain(
    nodes: &BTreeMap<String, (PathBuf, Value, Vec<u8>, Value, Vec<u8>)>,
    head: &str,
) -> Result<(), MutationRefusal> {
    let mut cursor = head.to_owned();
    let mut visited = BTreeSet::new();
    loop {
        if !visited.insert(cursor.clone()) {
            return Err(lineage(
                "committed registry transition chain contains a cycle",
            ));
        }
        let (_, _, _, transition, _) = nodes
            .get(&cursor)
            .ok_or_else(|| lineage("committed registry transition chain is incomplete"))?;
        match transition.get("prior_transition_fingerprint") {
            Some(Value::Null) => {
                if transition.get("operation").and_then(Value::as_str) != Some("bootstrap") {
                    return Err(lineage("registry transition chain root is not bootstrap"));
                }
                break;
            }
            Some(Value::String(prior)) => {
                let prior_node = nodes
                    .get(prior)
                    .ok_or_else(|| lineage("registry transition predecessor is absent"))?;
                if transition.get("prior_registry_state_fingerprint")
                    != prior_node.3.get("result_registry_state_fingerprint")
                    || transition.get("prior_registry_state_ref")
                        != prior_node.3.get("result_registry_state_ref")
                {
                    return Err(lineage("registry transition predecessor bindings disagree"));
                }
                cursor = prior.clone();
            }
            _ => return Err(lineage("registry transition predecessor shape is invalid")),
        }
    }
    if visited.len() != nodes.len() {
        return Err(lineage(
            "registry transition chain contains an unreachable fork",
        ));
    }
    Ok(())
}

fn resolve_credentials(
    repo_root: &Path,
    lineage_store: &TrustedLineageStoreV1,
    state: &Value,
) -> Result<Vec<CommittedApproverCredentialV1>, MutationRefusal> {
    let rows = state
        .get("credentials")
        .and_then(Value::as_array)
        .ok_or_else(|| lineage("registry credentials are absent"))?;
    if rows.is_empty() || rows.len() > MAX_REGISTRY_CREDENTIALS {
        return Err(lineage("registry credential count is outside 1..256"));
    }
    let mut seen = BTreeSet::new();
    let mut credentials = Vec::with_capacity(rows.len());
    for row in rows {
        let credential_id_hash = required_string(row, "credential_id_hash")?.to_owned();
        if !seen.insert(credential_id_hash.clone()) {
            return Err(lineage("registry contains duplicate credential IDs"));
        }
        let registration_ref = required_string(row, "registration_ref")?.to_owned();
        let registration_fingerprint = required_string(row, "registration_fingerprint")?.to_owned();
        let registration_bytes = lineage_store
            .read_record(
                LineageRecordClassV1::AuthenticatorRegistration,
                &registration_ref,
                &registration_fingerprint,
            )
            .map_err(|_| lineage("credential registration could not be re-resolved"))?;
        let registration = parse_schema_json(&registration_bytes)
            .map_err(|_| lineage("credential registration is not closed JSON"))?;
        let response_ref = required_string(&registration, "decoded_response_ref")?;
        let response_fingerprint = required_string(&registration, "decoded_response_fingerprint")?;
        let response_bytes = lineage_store
            .read_record(
                LineageRecordClassV1::AuthenticatorMakeCredentialResponse,
                response_ref,
                response_fingerprint,
            )
            .map_err(|_| lineage("credential response could not be re-resolved"))?;
        let response = parse_schema_json(&response_bytes)
            .map_err(|_| lineage("credential response is not closed JSON"))?;
        let credential_id = decode_bounded_base64(
            required_string(&response, "credential_id_base64")?,
            1_024,
            "credential ID",
        )?;
        if DefinitionFingerprint::from_bytes(&credential_id).as_str() != credential_id_hash {
            return Err(lineage("credential ID bytes and retained hash disagree"));
        }
        let cose_public_key = decode_bounded_base64(
            required_string(row, "cose_public_key_base64")?,
            3_072,
            "credential COSE key",
        )?;
        let approval_mappings = parse_mappings(row.get("approval_mappings"))?;
        let (use_head_ref, use_head_fingerprint, use_sequence, sign_count) =
            read_use_head(repo_root, &credential_id_hash)?;
        credentials.push(CommittedApproverCredentialV1 {
            credential_id_hash,
            credential_id,
            cose_public_key,
            active: required_string(row, "status")? == "active",
            approval_mappings,
            registration_ref,
            registration_fingerprint,
            use_head_ref,
            use_head_fingerprint,
            use_sequence,
            sign_count,
        });
    }
    credentials.sort_by(|left, right| left.credential_id.cmp(&right.credential_id));
    Ok(credentials)
}

fn read_use_head(
    repo_root: &Path,
    credential_id_hash: &str,
) -> Result<(String, String, u16, u32), MutationRefusal> {
    let hex = credential_id_hash
        .strip_prefix("sha256:")
        .ok_or_else(|| lineage("credential ID hash is not normalized"))?;
    let relative_ref = format!("authenticator-use-heads/{hex}.json");
    let bytes = read_registry_journal_file(
        repo_root,
        &repo_root.join(".handbook/state").join(&relative_ref),
        64 * 1024,
        "authenticator use head",
    )
    .map_err(|_| lineage("authenticator use head is not a safe bounded file"))?;
    let mut value = parse_schema_json(&bytes)
        .map_err(|_| lineage("authenticator use head is not closed JSON"))?;
    if serde_json_canonicalizer::to_vec(&value)
        .map_err(|_| lineage("authenticator use head is not canonicalizable"))?
        != bytes
        || value.get("credential_id_hash").and_then(Value::as_str) != Some(credential_id_hash)
    {
        return Err(lineage("authenticator use head identity is inconsistent"));
    }
    let fingerprint = required_string(&value, "head_fingerprint")?.to_owned();
    value.as_object_mut().unwrap().remove("head_fingerprint");
    if DefinitionFingerprint::from_json_value(&value)
        .map_err(|_| lineage("authenticator use head cannot be fingerprinted"))?
        .as_str()
        != fingerprint
    {
        return Err(lineage(
            "authenticator use head fingerprint does not recompute",
        ));
    }
    let sequence = value
        .get("sequence")
        .and_then(Value::as_u64)
        .and_then(|value| u16::try_from(value).ok())
        .filter(|value| *value <= 4_096)
        .ok_or_else(|| lineage("authenticator use sequence is outside 0..4096"))?;
    let sign_count = value
        .get("sign_count")
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| lineage("authenticator sign counter is invalid"))?;
    Ok((relative_ref, fingerprint, sequence, sign_count))
}

fn validate_expected_authority(
    request: &ApproverAdminRequestV1,
    authority: &CurrentAuthority,
) -> Result<(), MutationRefusal> {
    if authority
        .state
        .get("repository_identity_fingerprint")
        .and_then(Value::as_str)
        != Some(request.repository_identity_fingerprint())
    {
        return Err(stale(
            "request repository identity differs from committed registry authority",
        ));
    }
    if request.document()["expected_registry_state_fingerprint"]
        != Value::String(authority.state_fingerprint.clone())
        || request.document()["expected_transition_fingerprint"]
            != Value::String(authority.transition_fingerprint.clone())
    {
        return Err(stale("expected registry state or transition head is stale"));
    }
    Ok(())
}

fn request_mappings(
    request: &ApproverAdminRequestV1,
) -> Result<Vec<ApproverAuthorityPairV1>, MutationRefusal> {
    match request.document().get("approval_mappings") {
        None => Ok(Vec::new()),
        Some(value) => parse_mappings(Some(value)),
    }
}

fn parse_mappings(value: Option<&Value>) -> Result<Vec<ApproverAuthorityPairV1>, MutationRefusal> {
    let rows = value
        .and_then(Value::as_array)
        .ok_or_else(|| invalid("approval mappings are absent or not an array"))?;
    if rows.is_empty() || rows.len() > MAX_APPROVAL_MAPPINGS {
        return Err(invalid("approval mapping count is outside 1..64"));
    }
    let mut mappings = rows
        .iter()
        .map(|row| {
            Ok(ApproverAuthorityPairV1 {
                approval_class: required_string(row, "approval_class")?.to_owned(),
                authority_ref: required_string(row, "authority_ref")?.to_owned(),
            })
        })
        .collect::<Result<Vec<_>, MutationRefusal>>()?;
    let retained = mappings.clone();
    mappings.sort_by(pair_order);
    mappings.dedup();
    if mappings != retained {
        return Err(invalid(
            "approval mappings must be unique strict lexical pairs",
        ));
    }
    Ok(mappings)
}

fn required_pairs(
    repo_root: &Path,
    authority: &CurrentAuthority,
) -> Result<Vec<ApproverAuthorityPairV1>, MutationRefusal> {
    let mut pairs = vec![ApproverAuthorityPairV1 {
        approval_class: "registry_admin".to_owned(),
        authority_ref: "repository_registry_admin".to_owned(),
    }];
    let quorum = authority
        .state
        .get("initial_charter_quorum")
        .and_then(Value::as_array)
        .ok_or_else(|| lineage("committed initial Charter quorum is absent"))?;
    for row in quorum {
        pairs.push(ApproverAuthorityPairV1 {
            approval_class: required_string(row, "approval_class")?.to_owned(),
            authority_ref: required_string(row, "authority_ref")?.to_owned(),
        });
    }
    let canonical_path = repo_root.join(".handbook/project/charter.yaml");
    if canonical_path.exists() {
        let workspace = CanonicalWorkspace::new(repo_root);
        let relative = workspace
            .normalize_repo_relative(".handbook/project/charter.yaml")
            .map_err(|_| lineage("canonical Charter path is not normalized"))?;
        let file = workspace
            .trusted_read_strict(&relative)
            .map_err(|_| lineage("canonical Charter is not a safe regular file"))?;
        let (bytes, exceeded) = file
            .read_bytes_bounded(8 * 1024 * 1024)
            .map_err(|_| lineage("canonical Charter could not be read"))?;
        if exceeded {
            return Err(lineage("canonical Charter exceeds 8 MiB"));
        }
        let value = parse_definition_yaml(&bytes)
            .map_err(|_| lineage("canonical Charter is invalid YAML"))?;
        let canonical: CanonicalCharter = serde_json::from_value(value)
            .map_err(|_| lineage("canonical Charter is not closed typed authority"))?;
        validate_canonical_charter_semantics(&canonical)
            .map_err(|_| lineage("canonical Charter semantics are invalid"))?;
        for approval_class in &canonical.governance.required_approvals {
            for authority_ref in &canonical.governance.decision_authority {
                pairs.push(ApproverAuthorityPairV1 {
                    approval_class: approval_class.clone(),
                    authority_ref: authority_ref.clone(),
                });
            }
        }
    }
    pairs.sort_by(pair_order);
    pairs.dedup();
    if pairs.is_empty() || pairs.len() > MAX_APPROVAL_MAPPINGS {
        return Err(lineage(
            "required authority coverage is empty or exceeds 64 pairs",
        ));
    }
    Ok(pairs)
}

fn pair_order(
    left: &ApproverAuthorityPairV1,
    right: &ApproverAuthorityPairV1,
) -> std::cmp::Ordering {
    left.approval_class
        .cmp(&right.approval_class)
        .then_with(|| left.authority_ref.cmp(&right.authority_ref))
}

fn covers(credential: &CommittedApproverCredentialV1, pair: &ApproverAuthorityPairV1) -> bool {
    credential
        .approval_mappings
        .iter()
        .any(|mapping| mapping == pair)
}

fn mappings_cover(mappings: &[ApproverAuthorityPairV1], pair: &ApproverAuthorityPairV1) -> bool {
    mappings.iter().any(|mapping| mapping == pair)
}

fn eligible_administrators<'a>(
    authority: &'a CurrentAuthority,
    kind: MutationKind,
    target_hash: Option<&str>,
    requested_mappings: &[ApproverAuthorityPairV1],
    required_pairs: &[ApproverAuthorityPairV1],
) -> Result<Vec<&'a CommittedApproverCredentialV1>, MutationRefusal> {
    let admin_pair = ApproverAuthorityPairV1 {
        approval_class: "registry_admin".to_owned(),
        authority_ref: "repository_registry_admin".to_owned(),
    };
    let mut eligible = authority
        .credentials
        .iter()
        .filter(|credential| {
            credential.active
                && credential.use_sequence < 4_096
                && covers(credential, &admin_pair)
                && result_covered_after_use(
                    authority,
                    credential,
                    kind,
                    target_hash,
                    requested_mappings,
                    required_pairs,
                )
        })
        .collect::<Vec<_>>();
    eligible.sort_by(|left, right| left.credential_id.cmp(&right.credential_id));
    if eligible.len() > 64 {
        return Err(authorization(
            "eligible administrator allow-list exceeds 64 entries",
        ));
    }
    Ok(eligible)
}

fn result_covered_after_use(
    authority: &CurrentAuthority,
    administrator: &CommittedApproverCredentialV1,
    kind: MutationKind,
    target_hash: Option<&str>,
    requested_mappings: &[ApproverAuthorityPairV1],
    required_pairs: &[ApproverAuthorityPairV1],
) -> bool {
    required_pairs.iter().all(|pair| {
        authority.credentials.iter().any(|credential| {
            let target_revoked = kind == MutationKind::Revoke
                && target_hash == Some(credential.credential_id_hash.as_str());
            let usable_after = credential.active
                && !target_revoked
                && (credential.credential_id_hash != administrator.credential_id_hash
                    || credential.use_sequence < 4_095);
            let mapped_after = if kind == MutationKind::Update
                && target_hash == Some(credential.credential_id_hash.as_str())
            {
                mappings_cover(requested_mappings, pair)
            } else {
                covers(credential, pair)
            };
            usable_after && mapped_after
        }) || (kind == MutationKind::Add && mappings_cover(requested_mappings, pair))
    })
}

fn perform_registration<P: NativeAuthenticatorPortV1>(
    port: &mut P,
    request: &ApproverAdminRequestV1,
    authority: &CurrentAuthority,
    mappings: &[ApproverAuthorityPairV1],
) -> Result<RegistrationRecords, MutationRefusal> {
    if authority.credentials.len() >= MAX_REGISTRY_CREDENTIALS {
        return Err(invalid(
            "registry already contains the maximum 256 credentials",
        ));
    }
    let user_handle = derive_authenticator_user_handle(request.repository_identity_fingerprint())
        .map_err(map_authenticator_error)?;
    let nonce = random_nonce()?;
    let mapping_values = mappings
        .iter()
        .map(|pair| json!({"approval_class": pair.approval_class, "authority_ref": pair.authority_ref}))
        .collect::<Vec<_>>();
    let registration_request = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "allowed_algorithms": ["ES256"],
        "approval_mappings": mapping_values,
        "attestation_policy": "none",
        "extensions": {},
        "nonce_base64": BASE64_STANDARD.encode(nonce),
        "operation": "add_credential",
        "operation_id": request.operation_id(),
        "prior_registry_state_fingerprint": authority.state_fingerprint,
        "prior_transition_fingerprint": authority.transition_fingerprint,
        "protocol_id": "fido2-ctap2.1-native",
        "repository_identity_fingerprint": request.repository_identity_fingerprint(),
        "rp": {"id": AUTHENTICATOR_RP_ID, "name": "Handbook repository authority"},
        "schema_id": "handbook.authenticator-registration-request",
        "schema_version": "1.0",
        "user_handle_hash": user_handle.hash,
        "user_verification": "required"
    });
    let request_jcs = serde_json_canonicalizer::to_vec(&registration_request)
        .map_err(|_| durability("registration request canonicalization failed"))?;
    if request_jcs.len() > 16_384 {
        return Err(invalid("registration request exceeds its admitted bound"));
    }
    let client_data_hash: [u8; 32] = Sha256::digest(&request_jcs).into();
    let cbor = encode_make_credential_request(client_data_hash, user_handle.bytes)
        .map_err(map_authenticator_error)?;
    let raw = port.make_credential(&cbor).map_err(map_port_error)?;
    let decoded = decode_make_credential_response(&raw).map_err(map_authenticator_error)?;
    let response = make_credential_response_record(&decoded)?;
    let registration = registration_record(
        &decoded,
        &request_jcs,
        client_data_hash,
        &user_handle.hash,
        &response,
    )?;
    let initial_use_head = initial_use_head_record(&decoded)?;
    Ok(RegistrationRecords {
        response,
        registration,
        decoded,
        initial_use_head,
    })
}

fn validate_distinct_registration(
    authority: &CurrentAuthority,
    decoded: &MakeCredentialResponseV1,
) -> Result<(), MutationRefusal> {
    if authority.credentials.iter().any(|credential| {
        credential.credential_id_hash == decoded.credential_id_hash
            || credential.credential_id == decoded.credential_id
            || credential.cose_public_key == decoded.cose_public_key
    }) {
        return Err(invalid(
            "new credential ID and ES256 public key must both be distinct",
        ));
    }
    Ok(())
}

fn build_result_state(
    request: &ApproverAdminRequestV1,
    authority: &CurrentAuthority,
    kind: MutationKind,
    target_hash: Option<&str>,
    requested_mappings: &[ApproverAuthorityPairV1],
    registration: Option<&RegistrationRecords>,
) -> Result<BuiltRecord, MutationRefusal> {
    let mut value = authority.state.clone();
    value
        .as_object_mut()
        .ok_or_else(|| lineage("registry state root is not an object"))?
        .remove("registry_state_fingerprint");
    let version = value
        .get("registry_version")
        .and_then(Value::as_u64)
        .and_then(|value| value.checked_add(1))
        .ok_or_else(|| lineage("registry version cannot advance"))?;
    value["registry_version"] = json!(version);
    let credentials = value["credentials"]
        .as_array_mut()
        .ok_or_else(|| lineage("registry credentials are absent"))?;
    match kind {
        MutationKind::Add => {
            let registration = registration
                .ok_or_else(|| durability("add mutation has no verified registration"))?;
            let mappings = requested_mappings
                .iter()
                .map(|pair| json!({"approval_class": pair.approval_class, "authority_ref": pair.authority_ref}))
                .collect::<Vec<_>>();
            credentials.push(json!({
                "algorithm": "ES256",
                "approval_mappings": mappings,
                "cose_public_key_base64": BASE64_STANDARD.encode(&registration.decoded.cose_public_key),
                "credential_id_hash": registration.decoded.credential_id_hash,
                "extensions": {},
                "registration_fingerprint": registration.registration.fingerprint,
                "registration_ref": registration.registration.relative_ref,
                "status": "active"
            }));
        }
        MutationKind::Revoke => {
            let target = target_hash.ok_or_else(|| invalid("revoke target is absent"))?;
            let row = credentials
                .iter_mut()
                .find(|row| row["credential_id_hash"] == target)
                .ok_or_else(|| invalid("revoke target is not registered"))?;
            if row["status"] != "active" {
                return Err(invalid("revoke target is not active"));
            }
            row["status"] = json!("revoked");
        }
        MutationKind::Update => {
            let target = target_hash.ok_or_else(|| invalid("mapping target is absent"))?;
            let row = credentials
                .iter_mut()
                .find(|row| row["credential_id_hash"] == target)
                .ok_or_else(|| invalid("mapping target is not registered"))?;
            if row["status"] != "active" {
                return Err(invalid("mapping target is not active"));
            }
            row["approval_mappings"] = Value::Array(
                requested_mappings
                    .iter()
                    .map(|pair| json!({"approval_class": pair.approval_class, "authority_ref": pair.authority_ref}))
                    .collect(),
            );
        }
    }
    credentials.sort_by(|left, right| {
        left["credential_id_hash"]
            .as_str()
            .unwrap_or_default()
            .cmp(right["credential_id_hash"].as_str().unwrap_or_default())
    });
    if credentials.len() > MAX_REGISTRY_CREDENTIALS {
        return Err(invalid("result registry exceeds 256 credentials"));
    }
    let _ = request;
    finalized_record(
        value,
        None,
        "registry_state_fingerprint",
        "registry-state",
        "registry-states",
        &[],
    )
}

fn perform_assertion<P: NativeAuthenticatorPortV1>(
    repo_root: &Path,
    port: &mut P,
    request: &ApproverAdminRequestV1,
    authority: &CurrentAuthority,
    eligible: &[&CommittedApproverCredentialV1],
    result_state: &BuiltRecord,
    changed_hash: &str,
) -> Result<AssertionRecords, MutationRefusal> {
    if eligible.is_empty() {
        return Err(authorization(
            "no eligible registry administrator is retained",
        ));
    }
    let mut prior_heads = BTreeMap::new();
    for credential in eligible {
        prior_heads.insert(
            credential.credential_id_hash.clone(),
            read_state_file(repo_root, &credential.use_head_ref)?,
        );
    }
    let nonce = random_nonce()?;
    let challenge_value = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "changed_credential_id_hash": changed_hash,
        "nonce_base64": BASE64_STANDARD.encode(nonce),
        "operation": request.operation(),
        "operation_id": request.operation_id(),
        "prior_registry_state_fingerprint": authority.state_fingerprint,
        "prior_transition_fingerprint": authority.transition_fingerprint,
        "repository_identity_fingerprint": request.repository_identity_fingerprint(),
        "result_registry_state_fingerprint": result_state.fingerprint,
        "schema_id": "handbook.authenticator-challenge",
        "schema_version": "1.0"
    });
    let challenge_bytes = serde_json_canonicalizer::to_vec(&challenge_value)
        .map_err(|_| durability("registry challenge canonicalization failed"))?;
    if challenge_bytes.len() > 16_384 {
        return Err(invalid("registry challenge exceeds its admitted bound"));
    }
    let challenge_fingerprint = DefinitionFingerprint::from_bytes(&challenge_bytes).to_string();
    let nonce_fingerprint = DefinitionFingerprint::from_bytes(&nonce).to_string();
    let client_data_hash: [u8; 32] = Sha256::digest(&challenge_bytes).into();
    let allow_ids = eligible
        .iter()
        .map(|credential| credential.credential_id.clone())
        .collect::<Vec<_>>();
    let cbor = encode_get_assertion_request(client_data_hash, &allow_ids)
        .map_err(map_authenticator_error)?;
    let raw = port.get_assertion(&cbor).map_err(map_port_error)?;
    let requested = eligible
        .iter()
        .map(|credential| AuthenticatorCredentialV1 {
            credential_id: credential.credential_id.clone(),
            cose_public_key: credential.cose_public_key.clone(),
        })
        .collect::<Vec<_>>();
    let decoded = decode_and_verify_get_assertion_response(&raw, &requested, client_data_hash)
        .map_err(map_authenticator_error)?;
    let credential = eligible
        .iter()
        .find(|credential| credential.credential_id == decoded.credential_id)
        .ok_or_else(|| authorization("assertion descriptor is not an eligible administrator"))?;
    validate_counter(credential.sign_count, decoded.sign_count)?;
    let prior_use_head_bytes = prior_heads
        .remove(&credential.credential_id_hash)
        .ok_or_else(|| lineage("selected administrator use head was not retained"))?;
    build_assertion_records(
        request,
        credential,
        decoded,
        challenge_bytes,
        challenge_fingerprint,
        nonce_fingerprint,
        client_data_hash,
        prior_use_head_bytes,
    )
}

#[allow(clippy::too_many_arguments)]
fn build_assertion_records(
    request: &ApproverAdminRequestV1,
    credential: &CommittedApproverCredentialV1,
    decoded: GetAssertionResponseV1,
    challenge_bytes: Vec<u8>,
    challenge_fingerprint: String,
    nonce_fingerprint: String,
    client_data_hash: [u8; 32],
    prior_use_head_bytes: Vec<u8>,
) -> Result<AssertionRecords, MutationRefusal> {
    let rp_id_hash = DefinitionFingerprint::from_bytes(AUTHENTICATOR_RP_ID.as_bytes()).to_string();
    let response = finalized_record(
        json!({
            "algorithm": "ES256",
            "attested_credential_data_included": decoded.attested_credential_data_included,
            "authenticator_data_base64": BASE64_STANDARD.encode(decoded.authenticator_data),
            "credential_descriptor": {
                "credential_id_base64": BASE64_STANDARD.encode(&decoded.credential_id),
                "credential_id_hash": credential.credential_id_hash,
                "type": "public-key"
            },
            "extensions_included": decoded.extensions_included,
            "flags_byte": decoded.flags_byte,
            "protocol_id": "fido2-ctap2.1-native",
            "raw_response_base64": BASE64_STANDARD.encode(&decoded.raw_response),
            "rp_id_hash": rp_id_hash,
            "schema_id": "handbook.authenticator-get-assertion-response",
            "schema_version": "1.0",
            "sign_count": decoded.sign_count,
            "signature_base64": BASE64_STANDARD.encode(&decoded.signature_der),
            "signature_encoding": "asn1-der-ecdsa",
            "status_byte": 0,
            "user_present": true,
            "user_verified": true
        }),
        Some("response_id"),
        "response_fingerprint",
        "authenticator-get-assertion-response",
        "authenticator-get-assertion-responses",
        &[],
    )?;
    let mut signed_preimage = Vec::with_capacity(69);
    signed_preimage.extend_from_slice(&decoded.authenticator_data);
    signed_preimage.extend_from_slice(&client_data_hash);
    let assertion = finalized_record(
        json!({
            "algorithm": "ES256",
            "attested_credential_data_included": decoded.attested_credential_data_included,
            "authenticator_data_base64": BASE64_STANDARD.encode(decoded.authenticator_data),
            "challenge_fingerprint": challenge_fingerprint,
            "challenge_jcs_base64": BASE64_STANDARD.encode(&challenge_bytes),
            "challenge_schema_ref": "handbook.schemas.security.authenticator-challenge@1.0.0",
            "client_data_hash": fingerprint_digest(client_data_hash),
            "credential_id_hash": credential.credential_id_hash,
            "decoded_response_fingerprint": response.fingerprint,
            "decoded_response_ref": response.relative_ref,
            "decoded_response_schema_ref": "handbook.schemas.security.authenticator-get-assertion-response@1.0.0",
            "extensions_included": decoded.extensions_included,
            "flags_byte": decoded.flags_byte,
            "protocol_id": "fido2-ctap2.1-native",
            "rp_id": AUTHENTICATOR_RP_ID,
            "schema_id": "handbook.authenticator-assertion",
            "schema_version": "1.0",
            "sign_count": decoded.sign_count,
            "signature_base64": BASE64_STANDARD.encode(&decoded.signature_der),
            "signed_preimage_sha256": DefinitionFingerprint::from_bytes(&signed_preimage).to_string(),
            "user_present": true,
            "user_verified": true,
            "verified_at_utc": current_audit_utc()
        }),
        Some("assertion_id"),
        "assertion_fingerprint",
        "authenticator-assertion",
        "authenticator-assertions",
        &["verified_at_utc"],
    )?;
    let next_sequence = credential
        .use_sequence
        .checked_add(1)
        .filter(|value| *value <= 4_096)
        .ok_or_else(|| authorization("authenticator use sequence is exhausted"))?;
    let result_use_head = finalized_record(
        json!({
            "$schema": "handbook.schemas.security.authenticator-use-head@1.0.0",
            "credential_id_hash": credential.credential_id_hash,
            "last_assertion_fingerprint": assertion.fingerprint,
            "last_assertion_ref": assertion.relative_ref,
            "last_challenge_fingerprint": challenge_fingerprint,
            "last_nonce_sha256": nonce_fingerprint,
            "schema_version": "1.0",
            "sequence": next_sequence,
            "sign_count": decoded.sign_count
        }),
        None,
        "head_fingerprint",
        "authenticator-use-head",
        "authenticator-use-heads",
        &[],
    )?;
    let result_use_head = BuiltRecord {
        relative_ref: credential.use_head_ref.clone(),
        ..result_use_head
    };
    let use_transition = finalized_record(
        json!({
            "$schema": "handbook.schemas.security.authenticator-use-transition@1.0.0",
            "assertion_fingerprint": assertion.fingerprint,
            "assertion_ref": assertion.relative_ref,
            "challenge_fingerprint": challenge_fingerprint,
            "consumer_id": request.operation_id(),
            "consumer_kind": "registry_admin",
            "credential_id_hash": credential.credential_id_hash,
            "nonce_sha256": nonce_fingerprint,
            "prior_head_fingerprint": credential.use_head_fingerprint,
            "prior_sign_count": credential.sign_count,
            "result_head_fingerprint": result_use_head.fingerprint,
            "schema_version": "1.0",
            "sequence": next_sequence,
            "sign_count": decoded.sign_count
        }),
        None,
        "transition_fingerprint",
        "authenticator-use-transition",
        "authenticator-use-transitions",
        &[],
    )?;
    Ok(AssertionRecords {
        response,
        assertion,
        use_transition,
        result_use_head,
        prior_use_head_bytes,
        decoded,
    })
}

fn make_credential_response_record(
    decoded: &MakeCredentialResponseV1,
) -> Result<BuiltRecord, MutationRefusal> {
    finalized_record(
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
            "rp_id_hash": DefinitionFingerprint::from_bytes(AUTHENTICATOR_RP_ID.as_bytes()).to_string(),
            "schema_id": "handbook.authenticator-make-credential-response",
            "schema_version": "1.0",
            "sign_count": decoded.sign_count,
            "user_present": true,
            "user_verified": true
        }),
        Some("response_id"),
        "response_fingerprint",
        "authenticator-make-credential-response",
        "authenticator-make-credential-responses",
        &[],
    )
}

fn registration_record(
    decoded: &MakeCredentialResponseV1,
    request_jcs: &[u8],
    client_data_hash: [u8; 32],
    user_handle_hash: &str,
    response: &BuiltRecord,
) -> Result<BuiltRecord, MutationRefusal> {
    finalized_record(
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
            "decoded_response_schema_ref": "handbook.schemas.security.authenticator-make-credential-response@1.0.0",
            "protocol_id": "fido2-ctap2.1-native",
            "registered_at_utc": current_audit_utc(),
            "request_fingerprint": DefinitionFingerprint::from_bytes(request_jcs).to_string(),
            "request_jcs_base64": BASE64_STANDARD.encode(request_jcs),
            "request_schema_ref": "handbook.schemas.security.authenticator-registration-request@1.0.0",
            "rp_id": AUTHENTICATOR_RP_ID,
            "schema_id": "handbook.authenticator-registration",
            "schema_version": "1.0",
            "sign_count": decoded.sign_count,
            "user_handle_hash": user_handle_hash,
            "user_present": true,
            "user_verified": true
        }),
        Some("registration_id"),
        "registration_fingerprint",
        "authenticator-registration",
        "authenticator-registrations",
        &["registered_at_utc"],
    )
}

fn initial_use_head_record(
    decoded: &MakeCredentialResponseV1,
) -> Result<BuiltRecord, MutationRefusal> {
    let hex = decoded
        .credential_id_hash
        .strip_prefix("sha256:")
        .ok_or_else(|| invalid("credential ID hash is not normalized"))?;
    let mut record = finalized_record(
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
        None,
        "head_fingerprint",
        "authenticator-use-head",
        "authenticator-use-heads",
        &[],
    )?;
    record.relative_ref = format!("authenticator-use-heads/{hex}.json");
    Ok(record)
}

fn finalized_record(
    mut value: Value,
    id_field: Option<&str>,
    fingerprint_field: &str,
    id_prefix: &str,
    partition: &str,
    audit_only_fields: &[&str],
) -> Result<BuiltRecord, MutationRefusal> {
    let mut preimage = value.clone();
    let object = preimage
        .as_object_mut()
        .ok_or_else(|| durability("runtime record root is not an object"))?;
    for field in audit_only_fields {
        object.remove(*field);
    }
    let fingerprint = DefinitionFingerprint::from_json_value(&preimage)
        .map_err(|_| durability("runtime record fingerprint preimage is invalid"))?
        .to_string();
    let hex = fingerprint.strip_prefix("sha256:").unwrap();
    let id = format!("{id_prefix}_{hex}");
    let object = value.as_object_mut().unwrap();
    if let Some(field) = id_field {
        object.insert(field.to_owned(), Value::String(id.clone()));
    }
    object.insert(
        fingerprint_field.to_owned(),
        Value::String(fingerprint.clone()),
    );
    let bytes = serde_json_canonicalizer::to_vec(&value)
        .map_err(|_| durability("runtime record canonicalization failed"))?;
    if bytes.len() > MAX_RECORD_BYTES {
        return Err(durability("runtime record exceeds 1 MiB"));
    }
    Ok(BuiltRecord {
        value,
        bytes,
        relative_ref: format!("{partition}/{id}.json"),
        fingerprint,
    })
}

fn ensure_result_coverage(
    authority: &CurrentAuthority,
    result_state: &BuiltRecord,
    administrator: &CommittedApproverCredentialV1,
    registration: Option<&RegistrationRecords>,
    required_pairs: &[ApproverAuthorityPairV1],
) -> Result<(), MutationRefusal> {
    let rows = result_state
        .value
        .get("credentials")
        .and_then(Value::as_array)
        .ok_or_else(|| durability("result registry credentials are absent"))?;
    for pair in required_pairs {
        let covered = rows.iter().any(|row| {
            if row.get("status").and_then(Value::as_str) != Some("active") {
                return false;
            }
            let hash = row
                .get("credential_id_hash")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let sequence = if hash == administrator.credential_id_hash {
                administrator.use_sequence.saturating_add(1)
            } else if registration
                .map(|registration| registration.decoded.credential_id_hash.as_str())
                == Some(hash)
            {
                0
            } else {
                authority
                    .credentials
                    .iter()
                    .find(|credential| credential.credential_id_hash == hash)
                    .map(|credential| credential.use_sequence)
                    .unwrap_or(4_096)
            };
            sequence < 4_096
                && row
                    .get("approval_mappings")
                    .and_then(Value::as_array)
                    .map(|mappings| {
                        mappings.iter().any(|mapping| {
                            mapping.get("approval_class").and_then(Value::as_str)
                                == Some(pair.approval_class.as_str())
                                && mapping.get("authority_ref").and_then(Value::as_str)
                                    == Some(pair.authority_ref.as_str())
                        })
                    })
                    .unwrap_or(false)
        });
        if !covered {
            return Err(MutationRefusal {
                code: "lockout_refused",
                message: "registry mutation would remove the last usable required authority pair"
                    .to_owned(),
                retryable: false,
                next_action: "retain or add another usable credential covering every required pair"
                    .to_owned(),
            });
        }
    }
    Ok(())
}

fn ensure_authority_stable(
    repo_root: &Path,
    authority: &CurrentAuthority,
    administrator: &CommittedApproverCredentialV1,
    prior_head_bytes: &[u8],
) -> Result<(), MutationRefusal> {
    let lineage_store = TrustedLineageStoreV1::new(repo_root);
    if lineage_store
        .read_record(
            LineageRecordClassV1::RegistryState,
            &authority.state_ref,
            &authority.state_fingerprint,
        )
        .map_err(|_| lineage("retained registry state could not be re-resolved"))?
        != authority.state_bytes
        || lineage_store
            .read_record(
                LineageRecordClassV1::RegistryTransition,
                &authority.transition_ref,
                &authority.transition_fingerprint,
            )
            .map_err(|_| lineage("retained registry transition could not be re-resolved"))?
            != authority.transition_bytes
        || read_state_file(repo_root, &administrator.use_head_ref)? != prior_head_bytes
    {
        return Err(stale(
            "retained registry or authenticator-use authority changed",
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn build_registry_transition(
    request: &ApproverAdminRequestV1,
    authority: &CurrentAuthority,
    kind: MutationKind,
    target_hash: Option<&str>,
    registration: Option<&RegistrationRecords>,
    assertion: &AssertionRecords,
    result_state: &BuiltRecord,
    administrator: &CommittedApproverCredentialV1,
) -> Result<BuiltRecord, MutationRefusal> {
    let changed_hash = match kind {
        MutationKind::Add => registration
            .map(|registration| registration.decoded.credential_id_hash.clone())
            .ok_or_else(|| durability("add transition has no registration"))?,
        MutationKind::Revoke | MutationKind::Update => target_hash
            .ok_or_else(|| invalid("mutation target is absent"))?
            .to_owned(),
    };
    finalized_record(
        json!({
            "admin_credential_id_hash": administrator.credential_id_hash,
            "authorization_fingerprint": assertion.assertion.fingerprint,
            "authorization_kind": "assertion",
            "authorization_ref": assertion.assertion.relative_ref,
            "changed_credential_id_hash": changed_hash,
            "occurred_at_utc": current_audit_utc(),
            "operation": request.operation(),
            "prior_registry_state_fingerprint": authority.state_fingerprint,
            "prior_registry_state_ref": authority.state_ref,
            "prior_transition_fingerprint": authority.transition_fingerprint,
            "prior_transition_ref": authority.transition_ref,
            "repository_identity_fingerprint": request.repository_identity_fingerprint(),
            "result_registry_state_fingerprint": result_state.fingerprint,
            "result_registry_state_ref": result_state.relative_ref,
            "schema_id": "handbook.approver-registry-transition",
            "schema_version": "1.0"
        }),
        Some("transition_id"),
        "transition_fingerprint",
        "registry-transition",
        "registry-transitions",
        &["occurred_at_utc"],
    )
}

#[allow(clippy::too_many_arguments)]
fn commit_mutation(
    repo_root: &Path,
    request: &ApproverAdminRequestV1,
    authority: &CurrentAuthority,
    registration: Option<&RegistrationRecords>,
    assertion: &AssertionRecords,
    result_state: &BuiltRecord,
    transition: &BuiltRecord,
    administrator: &CommittedApproverCredentialV1,
) -> Result<(), MutationRefusal> {
    let root = repo_root.join(REGISTRY_TRANSACTION_ROOT);
    create_safe_directories(repo_root, &root)
        .map_err(|_| durability("registry transaction root could not be created safely"))?;
    let request_bytes = serde_json_canonicalizer::to_vec(request.document())
        .map_err(|_| durability("registry request canonicalization failed"))?;
    let transaction_id = format!("registry_{:x}", Sha256::digest(&request_bytes));
    let pending = root.join(format!("{transaction_id}.pending"));
    let committed = root.join(format!("{transaction_id}.committed"));
    if pending.exists() || committed.exists() {
        return Err(MutationRefusal {
            code: "transaction_conflict",
            message: "registry transaction identity already exists".to_owned(),
            retryable: false,
            next_action: "retain the committed result or restart from current authority".to_owned(),
        });
    }
    let intent = RegistryMutationIntent {
        schema_id: "handbook.registry-mutation-transaction-intent",
        schema_version: "1.0",
        transaction_id: transaction_id.clone(),
        operation: request.operation().to_owned(),
        operation_id: request.operation_id().to_owned(),
        repository_identity_fingerprint: request.repository_identity_fingerprint().to_owned(),
        prior_registry_state_ref: authority.state_ref.clone(),
        prior_registry_state_fingerprint: authority.state_fingerprint.clone(),
        prior_transition_ref: authority.transition_ref.clone(),
        prior_transition_fingerprint: authority.transition_fingerprint.clone(),
        response_ref: registration.map(|record| record.response.relative_ref.clone()),
        response_fingerprint: registration.map(|record| record.response.fingerprint.clone()),
        registration_ref: registration.map(|record| record.registration.relative_ref.clone()),
        registration_fingerprint: registration
            .map(|record| record.registration.fingerprint.clone()),
        assertion_response_ref: assertion.response.relative_ref.clone(),
        assertion_response_fingerprint: assertion.response.fingerprint.clone(),
        assertion_ref: assertion.assertion.relative_ref.clone(),
        assertion_fingerprint: assertion.assertion.fingerprint.clone(),
        result_registry_state_ref: result_state.relative_ref.clone(),
        result_registry_state_fingerprint: result_state.fingerprint.clone(),
        transition_ref: transition.relative_ref.clone(),
        transition_fingerprint: transition.fingerprint.clone(),
        use_transition_ref: assertion.use_transition.relative_ref.clone(),
        use_transition_fingerprint: assertion.use_transition.fingerprint.clone(),
        prior_use_head_ref: administrator.use_head_ref.clone(),
        prior_use_head_fingerprint: administrator.use_head_fingerprint.clone(),
        prior_use_head_bytes_fingerprint: DefinitionFingerprint::from_bytes(
            &assertion.prior_use_head_bytes,
        )
        .to_string(),
        result_use_head_ref: assertion.result_use_head.relative_ref.clone(),
        result_use_head_fingerprint: assertion.result_use_head.fingerprint.clone(),
        result_use_head_bytes_fingerprint: DefinitionFingerprint::from_bytes(
            &assertion.result_use_head.bytes,
        )
        .to_string(),
        new_credential_use_head_ref: registration
            .map(|record| record.initial_use_head.relative_ref.clone()),
        new_credential_use_head_fingerprint: registration
            .map(|record| record.initial_use_head.fingerprint.clone()),
    };
    let intent_bytes = serde_json_canonicalizer::to_vec(&intent)
        .map_err(|_| durability("registry mutation intent canonicalization failed"))?;
    fs::create_dir(&pending)
        .map_err(|_| durability("registry mutation pending directory create failed"))?;
    sync_directory(&root).map_err(|_| durability("registry transaction root fsync failed"))?;
    write_new_durable(&pending.join("intent.tmp"), &intent_bytes)?;
    fs::rename(pending.join("intent.tmp"), pending.join("intent.json"))
        .map_err(|_| durability("registry mutation intent rename failed"))?;

    let mut stages = Vec::<(&str, &[u8])>::new();
    if let Some(registration) = registration {
        stages.push(("make-response.new", &registration.response.bytes));
        stages.push(("registration.new", &registration.registration.bytes));
        stages.push((
            "new-credential-use-head.new",
            &registration.initial_use_head.bytes,
        ));
    }
    stages.extend([
        (
            "assertion-response.new",
            assertion.response.bytes.as_slice(),
        ),
        ("assertion.new", assertion.assertion.bytes.as_slice()),
        ("registry-state.new", result_state.bytes.as_slice()),
        ("registry-transition.new", transition.bytes.as_slice()),
        (
            "use-transition.new",
            assertion.use_transition.bytes.as_slice(),
        ),
        (
            "prior-use-head.json",
            assertion.prior_use_head_bytes.as_slice(),
        ),
        ("use-head.new", assertion.result_use_head.bytes.as_slice()),
    ]);
    for (name, bytes) in stages {
        write_new_durable(&pending.join(name), bytes)?;
    }
    sync_directory(&pending).map_err(|_| durability("registry mutation stage fsync failed"))?;

    let lineage_store = TrustedLineageStoreV1::new(repo_root);
    if let Some(registration) = registration {
        lineage_store
            .append_record(
                LineageRecordClassV1::AuthenticatorMakeCredentialResponse,
                &registration.response.bytes,
            )
            .map_err(|_| lineage("make-credential response install failed"))?;
        lineage_store
            .append_record(
                LineageRecordClassV1::AuthenticatorRegistration,
                &registration.registration.bytes,
            )
            .map_err(|_| lineage("registration install failed"))?;
        install_state_file(
            repo_root,
            &registration.initial_use_head.relative_ref,
            &registration.initial_use_head.bytes,
            false,
            None,
        )?;
    }
    for (class, bytes) in [
        (
            LineageRecordClassV1::AuthenticatorGetAssertionResponse,
            assertion.response.bytes.as_slice(),
        ),
        (
            LineageRecordClassV1::AuthenticatorAssertion,
            assertion.assertion.bytes.as_slice(),
        ),
        (
            LineageRecordClassV1::RegistryState,
            result_state.bytes.as_slice(),
        ),
        (
            LineageRecordClassV1::RegistryTransition,
            transition.bytes.as_slice(),
        ),
    ] {
        lineage_store
            .append_record(class, bytes)
            .map_err(|_| lineage("registry mutation immutable record install failed"))?;
    }
    install_state_file(
        repo_root,
        &assertion.use_transition.relative_ref,
        &assertion.use_transition.bytes,
        false,
        None,
    )?;
    install_state_file(
        repo_root,
        &assertion.result_use_head.relative_ref,
        &assertion.result_use_head.bytes,
        true,
        Some(&assertion.prior_use_head_bytes),
    )?;
    write_new_durable(&pending.join("records-installed"), b"installed\n")?;
    let marker = registry_commit_marker(&intent_bytes);
    write_new_durable(&pending.join("committed.tmp"), &marker)?;
    fs::rename(pending.join("committed.tmp"), pending.join("committed"))
        .map_err(|_| durability("registry commit marker rename failed"))?;
    sync_directory(&pending).map_err(|_| durability("registry pending fsync failed"))?;
    fs::rename(&pending, &committed)
        .map_err(|_| durability("registry mutation finalization failed"))?;
    sync_directory(&root).map_err(|_| durability("registry transaction root fsync failed"))?;
    Ok(())
}

fn success_result(
    request: &ApproverAdminRequestV1,
    authority: &CurrentAuthority,
    registration: Option<&RegistrationRecords>,
    assertion: &AssertionRecords,
    result_state: &BuiltRecord,
    transition: &BuiltRecord,
) -> Result<ApproverAdminResultV1, MutationRefusal> {
    let mut changed_paths = Vec::new();
    if let Some(registration) = registration {
        changed_paths.push(registration.response.relative_ref.clone());
        changed_paths.push(registration.registration.relative_ref.clone());
    }
    changed_paths.extend([
        assertion.response.relative_ref.clone(),
        assertion.assertion.relative_ref.clone(),
        result_state.relative_ref.clone(),
        transition.relative_ref.clone(),
    ]);
    ApproverAdminResultV1::from_json_value(json!({
        "assertion_fingerprint": assertion.assertion.fingerprint,
        "assertion_ref": assertion.assertion.relative_ref,
        "changed_paths": changed_paths,
        "next_actions": ["retain the committed transition reference"],
        "operation": request.operation(),
        "operation_id": request.operation_id(),
        "prior_registry_state_fingerprint": authority.state_fingerprint,
        "prior_registry_state_ref": authority.state_ref,
        "refusal": null,
        "registration_fingerprint": registration.map(|record| record.registration.fingerprint.clone()),
        "registration_ref": registration.map(|record| record.registration.relative_ref.clone()),
        "repository_identity_fingerprint": request.repository_identity_fingerprint(),
        "result_registry_state_fingerprint": result_state.fingerprint,
        "result_registry_state_ref": result_state.relative_ref,
        "schema_id": "handbook.approver-admin-result",
        "schema_version": "1.0",
        "status": "succeeded",
        "transition_fingerprint": transition.fingerprint,
        "transition_ref": transition.relative_ref
    }))
    .map_err(|_| durability("engine-built registry mutation result violated its schema"))
}

fn read_state_file(repo_root: &Path, relative_ref: &str) -> Result<Vec<u8>, MutationRefusal> {
    read_registry_journal_file(
        repo_root,
        &repo_root.join(".handbook/state").join(relative_ref),
        MAX_RECORD_BYTES,
        "retained authority state file",
    )
    .map_err(|_| lineage("retained authority state file is unsafe or oversized"))
}

fn install_state_file(
    repo_root: &Path,
    relative_ref: &str,
    bytes: &[u8],
    replace: bool,
    expected_prior: Option<&[u8]>,
) -> Result<(), MutationRefusal> {
    let target = repo_root.join(".handbook/state").join(relative_ref);
    let parent = target
        .parent()
        .ok_or_else(|| durability("authority state target has no parent"))?;
    create_safe_directories(repo_root, parent)
        .map_err(|_| durability("authority state parent could not be created safely"))?;
    if replace {
        let prior = read_state_file(repo_root, relative_ref)?;
        if expected_prior != Some(prior.as_slice()) {
            return Err(MutationRefusal {
                code: "transaction_conflict",
                message: "authenticator use head forked before registry commit".to_owned(),
                retryable: false,
                next_action: "repair or retain the exact committed use chain".to_owned(),
            });
        }
        let temporary = parent.join(format!(
            ".{}.registry-installing",
            target.file_name().unwrap().to_string_lossy()
        ));
        write_new_durable(&temporary, bytes)?;
        fs::rename(&temporary, &target)
            .map_err(|_| durability("authenticator use-head replacement failed"))?;
    } else if target.exists() {
        if read_state_file(repo_root, relative_ref)? != bytes {
            return Err(lineage(
                "immutable authority destination contains foreign bytes",
            ));
        }
    } else {
        write_new_durable(&target, bytes)?;
    }
    sync_directory(parent).map_err(|_| durability("authority state parent fsync failed"))
}

fn write_new_durable(path: &Path, bytes: &[u8]) -> Result<(), MutationRefusal> {
    let mut file = create_new_file(path)
        .map_err(|_| durability("authority create-new write destination failed"))?;
    file.write_all(bytes)
        .map_err(|_| durability("authority durable write failed"))?;
    file.sync_all()
        .map_err(|_| durability("authority durable file flush failed"))
}

#[allow(
    clippy::nonminimal_bool,
    reason = "the explicit zero/zero, zero/nonzero, and monotonic branches mirror the counter protocol"
)]
fn validate_counter(prior: u32, current: u32) -> Result<(), MutationRefusal> {
    if (prior == 0 && current == 0) || (prior == 0 && current > 0) || current > prior {
        Ok(())
    } else {
        Err(authorization(
            "authenticator sign counter is equal, decreasing, or reset after a nonzero value",
        ))
    }
}

fn random_nonce() -> Result<[u8; 32], MutationRefusal> {
    let mut nonce = [0u8; 32];
    getrandom::fill(&mut nonce).map_err(|_| MutationRefusal {
        code: "transaction_conflict",
        message: "operating-system randomness was unavailable".to_owned(),
        retryable: true,
        next_action: "retry the complete operation with fresh randomness".to_owned(),
    })?;
    Ok(nonce)
}

fn decode_bounded_base64(
    text: &str,
    maximum_bytes: usize,
    label: &str,
) -> Result<Vec<u8>, MutationRefusal> {
    let maximum_text = maximum_bytes.saturating_add(2) / 3 * 4;
    if text.is_empty() || text.len() > maximum_text {
        return Err(lineage(format!(
            "{label} base64 is outside its admitted bound"
        )));
    }
    let bytes = BASE64_STANDARD
        .decode(text)
        .map_err(|_| lineage(format!("{label} base64 is invalid")))?;
    if bytes.is_empty() || bytes.len() > maximum_bytes {
        return Err(lineage(format!(
            "{label} decoded size is outside its admitted bound"
        )));
    }
    Ok(bytes)
}

fn required_string<'a>(value: &'a Value, field: &str) -> Result<&'a str, MutationRefusal> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| lineage(format!("required authority field `{field}` is absent")))
}

fn map_port_error(error: NativeAuthenticatorPortErrorV1) -> MutationRefusal {
    match error {
        NativeAuthenticatorPortErrorV1::Unavailable => MutationRefusal {
            code: "AUTHENTICATOR_UNAVAILABLE",
            message: "native CTAP2.1 authenticator API is unavailable".to_owned(),
            retryable: true,
            next_action: "correct the request and retry".to_owned(),
        },
        NativeAuthenticatorPortErrorV1::Transport => MutationRefusal {
            code: "transaction_conflict",
            message: "native authenticator transport failed".to_owned(),
            retryable: true,
            next_action: "retry the complete operation with a fresh challenge".to_owned(),
        },
    }
}

fn map_authenticator_error(error: AuthenticatorErrorV1) -> MutationRefusal {
    match error {
        AuthenticatorErrorV1::Refused(status) => MutationRefusal {
            code: status.code.as_str(),
            message: format!("authenticator status 0x{:02x}", status.status_byte),
            retryable: status.retryable,
            next_action: status.next_action.to_owned(),
        },
        error => authorization(error.to_string()),
    }
}

fn lockout_or_authorization(kind: MutationKind) -> MutationRefusal {
    match kind {
        MutationKind::Add => MutationRefusal {
            code: "lockout_refused",
            message: "final-use add would leave incomplete required authority coverage".to_owned(),
            retryable: false,
            next_action:
                "add a distinct usable covering credential or use another usable administrator"
                    .to_owned(),
        },
        MutationKind::Revoke => MutationRefusal {
            code: "lockout_refused",
            message: "last registry admin cannot be revoked".to_owned(),
            retryable: false,
            next_action: "correct the request and retry".to_owned(),
        },
        MutationKind::Update => authorization("current administrator assertion was refused"),
    }
}

fn invalid(message: impl Into<String>) -> MutationRefusal {
    MutationRefusal {
        code: "invalid_request",
        message: message.into(),
        retryable: false,
        next_action: "correct the request and committed authority before retry".to_owned(),
    }
}

fn authorization(message: impl Into<String>) -> MutationRefusal {
    MutationRefusal {
        code: "authorization_refused",
        message: message.into(),
        retryable: false,
        next_action: "use an eligible authorized credential or change committed authority"
            .to_owned(),
    }
}

fn authority_absent(message: impl Into<String>) -> MutationRefusal {
    MutationRefusal {
        code: "authorization_refused",
        message: message.into(),
        retryable: false,
        next_action: "bootstrap exact committed registry authority before retry".to_owned(),
    }
}

fn stale(message: impl Into<String>) -> MutationRefusal {
    MutationRefusal {
        code: "stale_head",
        message: message.into(),
        retryable: true,
        next_action: "restart from the current registry state and transition head".to_owned(),
    }
}

fn lineage(message: impl Into<String>) -> MutationRefusal {
    MutationRefusal {
        code: "transaction_conflict",
        message: message.into(),
        retryable: false,
        next_action: "repair exact committed registry lineage before retry".to_owned(),
    }
}

fn durability(message: impl Into<String>) -> MutationRefusal {
    MutationRefusal {
        code: "transaction_conflict",
        message: message.into(),
        retryable: false,
        next_action: "repair exact durable registry authority before retry".to_owned(),
    }
}

fn fingerprint_digest(digest: [u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(71);
    output.push_str("sha256:");
    for byte in digest {
        output.push(HEX[usize::from(byte >> 4)] as char);
        output.push(HEX[usize::from(byte & 0x0f)] as char);
    }
    output
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
    let minute = (seconds_of_day % 3_600) / 60;
    let second = seconds_of_day % 60;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

fn civil_date_from_unix_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += if month <= 2 { 1 } else { 0 };
    (year, month, day)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryRecoveryErrorV1 {
    detail: String,
}

impl RegistryRecoveryErrorV1 {
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl fmt::Display for RegistryRecoveryErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for RegistryRecoveryErrorV1 {}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RegistryMutationRecoveryIntentV1 {
    schema_id: String,
    schema_version: String,
    transaction_id: String,
    operation: String,
    operation_id: String,
    repository_identity_fingerprint: String,
    prior_registry_state_ref: String,
    prior_registry_state_fingerprint: String,
    prior_transition_ref: String,
    prior_transition_fingerprint: String,
    response_ref: Option<String>,
    response_fingerprint: Option<String>,
    registration_ref: Option<String>,
    registration_fingerprint: Option<String>,
    assertion_response_ref: String,
    assertion_response_fingerprint: String,
    assertion_ref: String,
    assertion_fingerprint: String,
    result_registry_state_ref: String,
    result_registry_state_fingerprint: String,
    transition_ref: String,
    transition_fingerprint: String,
    use_transition_ref: String,
    use_transition_fingerprint: String,
    prior_use_head_ref: String,
    prior_use_head_fingerprint: String,
    prior_use_head_bytes_fingerprint: String,
    result_use_head_ref: String,
    result_use_head_fingerprint: String,
    result_use_head_bytes_fingerprint: String,
    new_credential_use_head_ref: Option<String>,
    new_credential_use_head_fingerprint: Option<String>,
}

#[derive(Clone, Copy)]
struct RecoveryRecordKind {
    partition: &'static str,
    id_prefix: &'static str,
    id_field: Option<&'static str>,
    fingerprint_field: &'static str,
    audit_only_fields: &'static [&'static str],
}

struct RecoveryTarget {
    stage_name: &'static str,
    relative_ref: String,
    fingerprint: String,
    kind: RecoveryRecordKind,
    replace_use_head: bool,
}

struct RecoveryTargetState {
    target: RecoveryTarget,
    stage: Option<Vec<u8>>,
    final_bytes: Option<Vec<u8>>,
}

const MAKE_RESPONSE_KIND: RecoveryRecordKind = RecoveryRecordKind {
    partition: "authenticator-make-credential-responses",
    id_prefix: "authenticator-make-credential-response",
    id_field: Some("response_id"),
    fingerprint_field: "response_fingerprint",
    audit_only_fields: &[],
};
const REGISTRATION_KIND: RecoveryRecordKind = RecoveryRecordKind {
    partition: "authenticator-registrations",
    id_prefix: "authenticator-registration",
    id_field: Some("registration_id"),
    fingerprint_field: "registration_fingerprint",
    audit_only_fields: &["registered_at_utc"],
};
const ASSERTION_RESPONSE_KIND: RecoveryRecordKind = RecoveryRecordKind {
    partition: "authenticator-get-assertion-responses",
    id_prefix: "authenticator-get-assertion-response",
    id_field: Some("response_id"),
    fingerprint_field: "response_fingerprint",
    audit_only_fields: &[],
};
const ASSERTION_KIND: RecoveryRecordKind = RecoveryRecordKind {
    partition: "authenticator-assertions",
    id_prefix: "authenticator-assertion",
    id_field: Some("assertion_id"),
    fingerprint_field: "assertion_fingerprint",
    audit_only_fields: &["verified_at_utc"],
};
const REGISTRY_STATE_KIND: RecoveryRecordKind = RecoveryRecordKind {
    partition: "registry-states",
    id_prefix: "registry-state",
    id_field: None,
    fingerprint_field: "registry_state_fingerprint",
    audit_only_fields: &[],
};
const REGISTRY_TRANSITION_KIND: RecoveryRecordKind = RecoveryRecordKind {
    partition: "registry-transitions",
    id_prefix: "registry-transition",
    id_field: Some("transition_id"),
    fingerprint_field: "transition_fingerprint",
    audit_only_fields: &["occurred_at_utc"],
};
const USE_TRANSITION_KIND: RecoveryRecordKind = RecoveryRecordKind {
    partition: "authenticator-use-transitions",
    id_prefix: "authenticator-use-transition",
    id_field: None,
    fingerprint_field: "transition_fingerprint",
    audit_only_fields: &[],
};
const USE_HEAD_KIND: RecoveryRecordKind = RecoveryRecordKind {
    partition: "authenticator-use-heads",
    id_prefix: "authenticator-use-head",
    id_field: None,
    fingerprint_field: "head_fingerprint",
    audit_only_fields: &[],
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SharedUseJournalFamilyV1 {
    Registry,
    Approval,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SharedUseJournalObservationV1 {
    pub(crate) family: SharedUseJournalFamilyV1,
    pub(crate) journal_path: String,
    pub(crate) transaction_id: String,
    pub(crate) credential_id_hash: String,
    pub(crate) prior_use_head_ref: String,
    pub(crate) prior_use_head_fingerprint: String,
    pub(crate) prior_use_sequence: u64,
    pub(crate) result_use_head_ref: String,
    pub(crate) result_use_head_fingerprint: String,
    pub(crate) result_use_sequence: u64,
    pub(crate) marker_exact: bool,
    pub(crate) finalized_directory: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SharedUseHeadAnchorV1 {
    pub(crate) credential_id_hash: String,
    pub(crate) use_head_ref: String,
    pub(crate) use_head_fingerprint: String,
    pub(crate) use_sequence: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedRegistryChainHeadV1 {
    pub state_ref: String,
    pub state_fingerprint: String,
    pub transition_ref: String,
    pub transition_fingerprint: String,
}

pub fn observe_committed_registry_chain_head(
    repo_root: &Path,
) -> Result<CommittedRegistryChainHeadV1, RegistryRecoveryErrorV1> {
    recover_registry_mutation_journals(repo_root)?;
    let _ordered_use_journals = discover_shared_use_journal_graph(repo_root)?;
    let (state_fingerprint, transition_fingerprint) = committed_registry_head_pair(repo_root)?;
    let root = repo_root.join(REGISTRY_TRANSACTION_ROOT);
    let mut match_head = None;
    for entry in fs::read_dir(&root)
        .map_err(|_| recovery_error("registry transaction root is unreadable"))?
    {
        let entry = entry.map_err(|_| recovery_error("registry journal entry is unreadable"))?;
        let name = entry
            .file_name()
            .to_str()
            .ok_or_else(|| recovery_error("registry journal name is not UTF-8"))?
            .to_owned();
        if !name.ends_with(".committed") {
            continue;
        }
        let intent_bytes = recovery_read(
            repo_root,
            &entry.path().join("intent.json"),
            64 * 1024,
            "committed registry chain-head intent",
        )?;
        let intent = parse_schema_json(&intent_bytes)
            .map_err(|_| recovery_error("committed chain-head intent is not closed JSON"))?;
        if intent.get("transition_fingerprint").and_then(Value::as_str)
            != Some(transition_fingerprint.as_str())
        {
            continue;
        }
        let observed = CommittedRegistryChainHeadV1 {
            state_ref: recovery_value_string(&intent, "result_registry_state_ref")?.to_owned(),
            state_fingerprint: recovery_value_string(&intent, "result_registry_state_fingerprint")?
                .to_owned(),
            transition_ref: recovery_value_string(&intent, "transition_ref")?.to_owned(),
            transition_fingerprint: recovery_value_string(&intent, "transition_fingerprint")?
                .to_owned(),
        };
        if observed.state_fingerprint != state_fingerprint || match_head.replace(observed).is_some()
        {
            return Err(recovery_error(
                "committed registry chain head is ambiguous or inconsistently bound",
            ));
        }
    }
    match_head.ok_or_else(|| recovery_error("committed registry chain head intent is absent"))
}

pub(crate) fn observe_committed_registry_chain_head_locked(
    repo_root: &Path,
) -> Result<CommittedRegistryChainHeadV1, RegistryRecoveryErrorV1> {
    let _ordered_use_journals = discover_shared_use_journal_graph(repo_root)?;
    let (state_fingerprint, transition_fingerprint) = committed_registry_head_pair(repo_root)?;
    let root = repo_root.join(REGISTRY_TRANSACTION_ROOT);
    let mut match_head = None;
    for entry in fs::read_dir(&root)
        .map_err(|_| recovery_error("registry transaction root is unreadable"))?
    {
        let entry = entry.map_err(|_| recovery_error("registry journal entry is unreadable"))?;
        let name = entry
            .file_name()
            .to_str()
            .ok_or_else(|| recovery_error("registry journal name is not UTF-8"))?
            .to_owned();
        if !name.ends_with(".committed") {
            continue;
        }
        let intent_bytes = recovery_read(
            repo_root,
            &entry.path().join("intent.json"),
            64 * 1024,
            "retained registry chain-head intent",
        )?;
        let intent = parse_schema_json(&intent_bytes)
            .map_err(|_| recovery_error("retained chain-head intent is not closed JSON"))?;
        if intent.get("transition_fingerprint").and_then(Value::as_str)
            != Some(transition_fingerprint.as_str())
        {
            continue;
        }
        let observed = CommittedRegistryChainHeadV1 {
            state_ref: recovery_value_string(&intent, "result_registry_state_ref")?.to_owned(),
            state_fingerprint: recovery_value_string(&intent, "result_registry_state_fingerprint")?
                .to_owned(),
            transition_ref: recovery_value_string(&intent, "transition_ref")?.to_owned(),
            transition_fingerprint: recovery_value_string(&intent, "transition_fingerprint")?
                .to_owned(),
        };
        if observed.state_fingerprint != state_fingerprint || match_head.replace(observed).is_some()
        {
            return Err(recovery_error(
                "retained registry chain head is ambiguous or inconsistently bound",
            ));
        }
    }
    match_head.ok_or_else(|| recovery_error("retained registry chain head intent is absent"))
}

pub(crate) fn observe_committed_registry_chain_head_if_present_locked(
    repo_root: &Path,
) -> Result<Option<CommittedRegistryChainHeadV1>, RegistryRecoveryErrorV1> {
    let _ordered_use_journals = discover_shared_use_journal_graph(repo_root)?;
    let root = repo_root.join(REGISTRY_TRANSACTION_ROOT);
    if !root.exists() {
        return Ok(None);
    }
    reject_reparse_or_symlink(&root, true)
        .map_err(|_| recovery_error("registry transaction root is unsafe"))?;
    let mut has_committed = false;
    let mut has_entry = false;
    for entry in fs::read_dir(&root)
        .map_err(|_| recovery_error("registry transaction root is unreadable"))?
    {
        let entry = entry.map_err(|_| recovery_error("registry journal entry is unreadable"))?;
        has_entry = true;
        let name = entry
            .file_name()
            .to_str()
            .ok_or_else(|| recovery_error("registry journal name is not UTF-8"))?
            .to_owned();
        if name.ends_with(".committed") {
            has_committed = true;
        } else if name.ends_with(".pending") {
            return Err(recovery_error(
                "pending registry journal remains after locked recovery",
            ));
        } else {
            return Err(recovery_error(
                "registry transaction root contains an unknown entry",
            ));
        }
    }
    if !has_entry {
        return Ok(None);
    }
    if !has_committed {
        return Err(recovery_error(
            "registry transaction root is non-empty without committed authority",
        ));
    }
    observe_committed_registry_chain_head_locked(repo_root).map(Some)
}

fn discover_registry_use_head_anchors(
    repo_root: &Path,
) -> Result<Vec<SharedUseHeadAnchorV1>, RegistryRecoveryErrorV1> {
    let root = repo_root.join(REGISTRY_TRANSACTION_ROOT);
    if !root.exists() {
        return Ok(Vec::new());
    }
    reject_reparse_or_symlink(&root, true)
        .map_err(|_| recovery_error("registry transaction root is unsafe"))?;
    let mut committed = fs::read_dir(&root)
        .map_err(|_| recovery_error("registry transaction root is unreadable"))?
        .map(|entry| {
            let entry =
                entry.map_err(|_| recovery_error("registry journal entry is unreadable"))?;
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| recovery_error("registry journal name is not UTF-8"))?
                .to_owned();
            if !name.ends_with(".pending") && !name.ends_with(".committed") {
                return Err(recovery_error(
                    "registry transaction root contains an unknown entry",
                ));
            }
            Ok((name, entry.path()))
        })
        .collect::<Result<Vec<_>, RegistryRecoveryErrorV1>>()?;
    committed.retain(|(name, _)| name.ends_with(".committed"));
    committed.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));

    let mut anchors = Vec::new();
    for (_, directory) in committed {
        reject_reparse_or_symlink(&directory, true)
            .map_err(|_| recovery_error("committed registry journal is unsafe"))?;
        let intent_bytes = recovery_read(
            repo_root,
            &directory.join("intent.json"),
            64 * 1024,
            "registry anchor intent",
        )?;
        let intent_value = parse_schema_json(&intent_bytes)
            .map_err(|_| recovery_error("registry anchor intent is not closed JSON"))?;
        if serde_json_canonicalizer::to_vec(&intent_value)
            .map_err(|_| recovery_error("registry anchor intent is not canonicalizable"))?
            != intent_bytes
            || recovery_read(
                repo_root,
                &directory.join("committed"),
                72,
                "registry anchor marker",
            )? != registry_commit_marker(&intent_bytes)
        {
            return Err(recovery_error(
                "registry anchor journal does not bind its exact raw intent",
            ));
        }

        match intent_value.get("schema_id").and_then(Value::as_str) {
            Some("handbook.registry-transaction-intent") => {
                let use_head_ref = recovery_value_string(&intent_value, "use_head_ref")?;
                let use_head_fingerprint =
                    recovery_value_string(&intent_value, "use_head_fingerprint")?;
                DefinitionFingerprint::parse(use_head_fingerprint).map_err(|_| {
                    recovery_error("registry bootstrap anchor fingerprint is invalid")
                })?;
                let credential_id_hash = credential_hash_from_use_head_ref(use_head_ref)?;
                anchors.push(SharedUseHeadAnchorV1 {
                    credential_id_hash,
                    use_head_ref: use_head_ref.to_owned(),
                    use_head_fingerprint: use_head_fingerprint.to_owned(),
                    use_sequence: 0,
                });
            }
            Some("handbook.registry-mutation-transaction-intent") => {
                let intent: RegistryMutationRecoveryIntentV1 =
                    serde_json::from_slice(&intent_bytes).map_err(|_| {
                        recovery_error("registry mutation anchor intent is not closed")
                    })?;
                if intent.operation != "add_credential" {
                    continue;
                }
                let use_head_ref = intent
                    .new_credential_use_head_ref
                    .as_deref()
                    .ok_or_else(|| recovery_error("registry add anchor use-head ref is absent"))?;
                let use_head_fingerprint = intent
                    .new_credential_use_head_fingerprint
                    .as_deref()
                    .ok_or_else(|| {
                        recovery_error("registry add anchor use-head fingerprint is absent")
                    })?;
                let head_bytes = recovery_read(
                    repo_root,
                    &directory.join("new-credential-use-head.new"),
                    MAX_RECORD_BYTES,
                    "registry add anchor use head",
                )?;
                validate_recovery_record(
                    &head_bytes,
                    use_head_ref,
                    use_head_fingerprint,
                    USE_HEAD_KIND,
                )?;
                let head_value = parse_schema_json(&head_bytes)
                    .map_err(|_| recovery_error("registry add anchor use head is invalid"))?;
                let credential_id_hash =
                    recovery_value_string(&head_value, "credential_id_hash")?.to_owned();
                if recovery_value_u64(&head_value, "sequence")? != 0
                    || credential_hash_from_use_head_ref(use_head_ref)? != credential_id_hash
                {
                    return Err(recovery_error(
                        "registry add anchor use head does not bind sequence zero and credential",
                    ));
                }
                anchors.push(SharedUseHeadAnchorV1 {
                    credential_id_hash,
                    use_head_ref: use_head_ref.to_owned(),
                    use_head_fingerprint: use_head_fingerprint.to_owned(),
                    use_sequence: 0,
                });
            }
            _ => return Err(recovery_error("registry anchor intent schema is unknown")),
        }
    }
    Ok(anchors)
}

fn credential_hash_from_use_head_ref(
    use_head_ref: &str,
) -> Result<String, RegistryRecoveryErrorV1> {
    let Some(hex) = use_head_ref
        .strip_prefix("authenticator-use-heads/")
        .and_then(|value| value.strip_suffix(".json"))
    else {
        return Err(recovery_error(
            "registry anchor use-head ref is not normalized",
        ));
    };
    if hex.len() != 64
        || !hex
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(recovery_error(
            "registry anchor credential hash is not lowercase SHA-256",
        ));
    }
    Ok(format!("sha256:{hex}"))
}

fn normalized_journal_path(
    repo_root: &Path,
    directory: &Path,
) -> Result<String, RegistryRecoveryErrorV1> {
    let relative = directory
        .strip_prefix(repo_root)
        .map_err(|_| recovery_error("registry journal path escapes repository root"))?
        .to_str()
        .ok_or_else(|| recovery_error("registry journal path is not UTF-8"))?
        .replace('\\', "/");
    CanonicalWorkspace::new(repo_root)
        .normalize_repo_relative(&relative)
        .map(|path| path.as_str().to_owned())
        .map_err(|_| recovery_error("registry journal path is not normalized"))
}

pub(crate) fn discover_registry_use_journals(
    repo_root: &Path,
) -> Result<Vec<SharedUseJournalObservationV1>, RegistryRecoveryErrorV1> {
    let root = repo_root.join(REGISTRY_TRANSACTION_ROOT);
    if !root.exists() {
        return Ok(Vec::new());
    }
    reject_reparse_or_symlink(&root, true)
        .map_err(|_| recovery_error("registry transaction root is unsafe"))?;
    let mut directories = fs::read_dir(&root)
        .map_err(|_| recovery_error("registry transaction root is unreadable"))?
        .map(|entry| {
            let entry =
                entry.map_err(|_| recovery_error("registry journal entry is unreadable"))?;
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| recovery_error("registry journal name is not UTF-8"))?
                .to_owned();
            if !name.ends_with(".pending") && !name.ends_with(".committed") {
                return Err(recovery_error(
                    "registry transaction root contains an unknown entry",
                ));
            }
            reject_reparse_or_symlink(&entry.path(), true)
                .map_err(|_| recovery_error("registry journal directory is unsafe"))?;
            Ok(entry.path())
        })
        .collect::<Result<Vec<_>, RegistryRecoveryErrorV1>>()?;
    directories.sort_by(|left, right| left.as_os_str().cmp(right.as_os_str()));

    let mut nodes = Vec::new();
    let mut prior_heads = BTreeSet::new();
    let mut result_sequences = BTreeSet::new();
    for directory in directories {
        let entries = journal_entry_names(&directory)?;
        if !entries.iter().any(|name| name == "intent.json") {
            if entries.is_empty()
                || (entries.len() == 1 && entries.iter().any(|name| name == "intent.tmp"))
            {
                continue;
            }
            return Err(recovery_error(
                "registry journal has state without a durable intent",
            ));
        }
        let intent_bytes = recovery_read(
            repo_root,
            &directory.join("intent.json"),
            64 * 1024,
            "registry discovery intent",
        )?;
        let intent_value = parse_schema_json(&intent_bytes)
            .map_err(|_| recovery_error("registry discovery intent is not closed JSON"))?;
        if serde_json_canonicalizer::to_vec(&intent_value)
            .map_err(|_| recovery_error("registry discovery intent is not canonicalizable"))?
            != intent_bytes
        {
            return Err(recovery_error("registry discovery intent is not exact JCS"));
        }
        if intent_value.get("schema_id").and_then(Value::as_str)
            == Some("handbook.registry-transaction-intent")
        {
            continue;
        }
        let intent: RegistryMutationRecoveryIntentV1 = serde_json::from_slice(&intent_bytes)
            .map_err(|_| recovery_error("registry mutation discovery intent is not closed"))?;
        let finalized_directory = directory
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with(".committed"));
        let synthetic_pending =
            directory.with_file_name(format!("{}.pending", intent.transaction_id));
        validate_recovery_intent(&synthetic_pending, &intent)?;
        let expected_name = format!(
            "{}{}",
            intent.transaction_id,
            if finalized_directory {
                ".committed"
            } else {
                ".pending"
            }
        );
        if directory.file_name().and_then(|name| name.to_str()) != Some(expected_name.as_str())
            || entries
                .iter()
                .any(|name| !recovery_allowed_entries(&intent).contains(name))
        {
            return Err(recovery_error(
                "registry discovery journal path or entry set is invalid",
            ));
        }
        let expected_marker = registry_commit_marker(&intent_bytes);
        let marker = optional_recovery_read(
            repo_root,
            &directory.join("committed"),
            72,
            "registry discovery marker",
        )?;
        let marker_exact = marker.as_deref() == Some(expected_marker.as_slice());
        if (finalized_directory || marker.is_some()) && !marker_exact {
            return Err(recovery_error(
                "registry discovery marker does not exactly bind raw intent",
            ));
        }
        if let Some(marker_tmp) = optional_recovery_read(
            repo_root,
            &directory.join("committed.tmp"),
            72,
            "registry discovery temporary marker",
        )? {
            if marker.is_some() || marker_tmp != expected_marker {
                return Err(recovery_error(
                    "registry discovery temporary marker is conflicting or mismatched",
                ));
            }
        }
        let prior = recovery_read(
            repo_root,
            &directory.join("prior-use-head.json"),
            MAX_RECORD_BYTES,
            "registry discovery prior use head",
        )?;
        let result = recovery_read(
            repo_root,
            &directory.join("use-head.new"),
            MAX_RECORD_BYTES,
            "registry discovery result use head",
        )?;
        validate_recovery_record(
            &prior,
            &intent.prior_use_head_ref,
            &intent.prior_use_head_fingerprint,
            USE_HEAD_KIND,
        )?;
        validate_recovery_record(
            &result,
            &intent.result_use_head_ref,
            &intent.result_use_head_fingerprint,
            USE_HEAD_KIND,
        )?;
        if DefinitionFingerprint::from_bytes(&prior).as_str()
            != intent.prior_use_head_bytes_fingerprint
            || DefinitionFingerprint::from_bytes(&result).as_str()
                != intent.result_use_head_bytes_fingerprint
        {
            return Err(recovery_error(
                "registry discovery use-head staged bytes disagree with intent",
            ));
        }
        let prior_value = parse_schema_json(&prior)
            .map_err(|_| recovery_error("registry discovery prior head is invalid"))?;
        let result_value = parse_schema_json(&result)
            .map_err(|_| recovery_error("registry discovery result head is invalid"))?;
        let credential_id_hash =
            recovery_value_string(&prior_value, "credential_id_hash")?.to_owned();
        let prior_sequence = recovery_value_u64(&prior_value, "sequence")?;
        let result_sequence = recovery_value_u64(&result_value, "sequence")?;
        if result_value.get("credential_id_hash")
            != Some(&Value::String(credential_id_hash.clone()))
            || intent.prior_use_head_ref != intent.result_use_head_ref
            || result_sequence != prior_sequence.saturating_add(1)
            || result_sequence > 4096
        {
            return Err(recovery_error(
                "registry discovery predecessor graph binding is invalid",
            ));
        }
        if !prior_heads.insert((
            credential_id_hash.clone(),
            intent.prior_use_head_fingerprint.clone(),
        )) {
            return Err(recovery_error(
                "registry use journals contain a same-prior fork",
            ));
        }
        if !result_sequences.insert((credential_id_hash.clone(), result_sequence)) {
            return Err(recovery_error(
                "registry use journals contain a duplicate sequence",
            ));
        }
        nodes.push(SharedUseJournalObservationV1 {
            family: SharedUseJournalFamilyV1::Registry,
            journal_path: normalized_journal_path(repo_root, &directory)?,
            transaction_id: intent.transaction_id,
            credential_id_hash,
            prior_use_head_ref: intent.prior_use_head_ref,
            prior_use_head_fingerprint: intent.prior_use_head_fingerprint,
            prior_use_sequence: prior_sequence,
            result_use_head_ref: intent.result_use_head_ref,
            result_use_head_fingerprint: intent.result_use_head_fingerprint,
            result_use_sequence: result_sequence,
            marker_exact,
            finalized_directory,
        });
    }
    nodes.sort_by(|left, right| {
        left.journal_path
            .as_bytes()
            .cmp(right.journal_path.as_bytes())
    });
    Ok(nodes)
}

pub(crate) fn discover_shared_use_journal_graph(
    repo_root: &Path,
) -> Result<Vec<SharedUseJournalObservationV1>, RegistryRecoveryErrorV1> {
    let anchors = discover_registry_use_head_anchors(repo_root)?;
    let registry = discover_registry_use_journals(repo_root)?;
    let approval = crate::charter_approval_workflow::discover_approval_use_journals(repo_root)
        .map_err(|error| {
            recovery_error(format!(
                "approval journal discovery refused: {}",
                error.message
            ))
        })?
        .into_iter()
        .map(|node| SharedUseJournalObservationV1 {
            family: SharedUseJournalFamilyV1::Approval,
            journal_path: node.journal_path,
            transaction_id: node.transaction_id,
            credential_id_hash: node.credential_id_hash,
            prior_use_head_ref: node.prior_use_head_ref,
            prior_use_head_fingerprint: node.prior_use_head_fingerprint,
            prior_use_sequence: node.prior_use_sequence,
            result_use_head_ref: node.result_use_head_ref,
            result_use_head_fingerprint: node.result_use_head_fingerprint,
            result_use_sequence: node.result_use_sequence,
            marker_exact: matches!(
                node.marker_state,
                crate::charter_approval_workflow::ApprovalJournalMarkerStateV1::Exact
            ),
            finalized_directory: node.finalized_directory,
        })
        .collect();
    compose_shared_use_journal_graph(&anchors, registry, approval)
}

pub(crate) fn compose_shared_use_journal_graph(
    anchors: &[SharedUseHeadAnchorV1],
    mut registry: Vec<SharedUseJournalObservationV1>,
    mut approval: Vec<SharedUseJournalObservationV1>,
) -> Result<Vec<SharedUseJournalObservationV1>, RegistryRecoveryErrorV1> {
    registry.sort_by(|left, right| {
        left.journal_path
            .as_bytes()
            .cmp(right.journal_path.as_bytes())
    });
    approval.sort_by(|left, right| {
        left.journal_path
            .as_bytes()
            .cmp(right.journal_path.as_bytes())
    });
    let mut nodes = registry;
    nodes.extend(approval);
    let mut successors = BTreeMap::<(String, String), usize>::new();
    let mut sequences = BTreeSet::new();
    for (index, node) in nodes.iter().enumerate() {
        if node.prior_use_head_ref != node.result_use_head_ref
            || node.result_use_sequence != node.prior_use_sequence.saturating_add(1)
            || node.result_use_sequence > 4096
        {
            return Err(recovery_error(
                "shared use journal contains a non-adjacent replay sequence",
            ));
        }
        if successors
            .insert(
                (
                    node.credential_id_hash.clone(),
                    node.prior_use_head_fingerprint.clone(),
                ),
                index,
            )
            .is_some()
        {
            return Err(recovery_error(
                "shared use journal graph contains a cross-family fork",
            ));
        }
        if !sequences.insert((node.credential_id_hash.clone(), node.result_use_sequence)) {
            return Err(recovery_error(
                "shared use journal graph contains a duplicate sequence",
            ));
        }
    }
    let mut anchors = anchors.to_vec();
    anchors.sort_by(|left, right| left.credential_id_hash.cmp(&right.credential_id_hash));
    let mut anchor_credentials = BTreeSet::new();
    let mut visited = BTreeSet::new();
    let mut ordered = Vec::with_capacity(nodes.len());
    for anchor in anchors {
        if !anchor_credentials.insert(anchor.credential_id_hash.clone()) || anchor.use_sequence != 0
        {
            return Err(recovery_error(
                "shared use journal graph has a duplicate or nonzero anchor",
            ));
        }
        let mut fingerprint = anchor.use_head_fingerprint;
        let mut sequence = anchor.use_sequence;
        loop {
            let Some(index) = successors
                .get(&(anchor.credential_id_hash.clone(), fingerprint.clone()))
                .copied()
            else {
                break;
            };
            if !visited.insert(index) {
                return Err(recovery_error("shared use journal graph contains a cycle"));
            }
            let node = &nodes[index];
            if node.prior_use_sequence != sequence || node.prior_use_head_ref != anchor.use_head_ref
            {
                return Err(recovery_error(
                    "shared use journal graph predecessor sequence/ref disagrees",
                ));
            }
            ordered.push(node.clone());
            fingerprint = node.result_use_head_fingerprint.clone();
            sequence = node.result_use_sequence;
        }
    }
    if visited.len() != nodes.len() {
        return Err(recovery_error(
            "shared use journal graph contains an unreachable prior or family-local optimistic head",
        ));
    }
    Ok(ordered)
}

pub(crate) fn recover_registry_mutation_journals(
    repo_root: &Path,
) -> Result<(), RegistryRecoveryErrorV1> {
    let root = repo_root.join(REGISTRY_TRANSACTION_ROOT);
    if !root.exists() {
        return Ok(());
    }
    reject_reparse_or_symlink(&root, true)
        .map_err(|_| recovery_error("registry transaction root is unsafe"))?;
    let mut pending = Vec::new();
    for entry in fs::read_dir(&root)
        .map_err(|_| recovery_error("registry transaction root is unreadable"))?
    {
        let entry =
            entry.map_err(|_| recovery_error("registry transaction entry is unreadable"))?;
        let name = entry
            .file_name()
            .to_str()
            .ok_or_else(|| recovery_error("registry transaction name is not UTF-8"))?
            .to_owned();
        if name.ends_with(".pending") {
            reject_reparse_or_symlink(&entry.path(), true)
                .map_err(|_| recovery_error("pending registry journal is unsafe"))?;
            pending.push(entry.path());
        } else if !name.ends_with(".committed") {
            return Err(recovery_error(
                "registry transaction root contains an unknown entry",
            ));
        }
    }
    pending.sort_by(|left, right| left.file_name().cmp(&right.file_name()));
    for directory in pending {
        recover_one_registry_mutation(repo_root, &root, &directory)?;
    }
    Ok(())
}

pub(crate) fn recover_complete_registry_approval_authority_locked(
    repo_root: &Path,
) -> Result<(), RegistryRecoveryErrorV1> {
    recover_registry_pre_intent_journals_locked(repo_root)?;
    crate::charter_approval_workflow::recover_approval_pre_intent_authority_locked(repo_root)
        .map_err(|error| {
            recovery_error(format!(
                "approval pre-intent recovery refused: {}",
                error.message
            ))
        })?;

    let ordered = discover_shared_use_journal_graph(repo_root)?;
    for journal in ordered {
        if journal.finalized_directory {
            continue;
        }
        match journal.family {
            SharedUseJournalFamilyV1::Registry => {
                recover_one_registry_mutation_journal_locked(repo_root, &journal.journal_path)?;
            }
            SharedUseJournalFamilyV1::Approval => {
                crate::charter_approval_workflow::recover_one_approval_authority_journal_locked(
                    repo_root,
                    &journal.journal_path,
                )
                .map_err(|error| {
                    recovery_error(format!(
                        "approval predecessor recovery refused: {}",
                        error.message
                    ))
                })?;
            }
        }
    }
    discover_shared_use_journal_graph(repo_root)?;
    Ok(())
}

fn recover_registry_pre_intent_journals_locked(
    repo_root: &Path,
) -> Result<(), RegistryRecoveryErrorV1> {
    let root = repo_root.join(REGISTRY_TRANSACTION_ROOT);
    if !root.exists() {
        return Ok(());
    }
    reject_reparse_or_symlink(&root, true)
        .map_err(|_| recovery_error("registry transaction root is unsafe"))?;
    let mut pending = fs::read_dir(&root)
        .map_err(|_| recovery_error("registry transaction root is unreadable"))?
        .map(|entry| {
            let entry =
                entry.map_err(|_| recovery_error("registry transaction entry is unreadable"))?;
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| recovery_error("registry transaction name is not UTF-8"))?
                .to_owned();
            if name.ends_with(".pending") {
                Ok(Some(entry.path()))
            } else if name.ends_with(".committed") {
                Ok(None)
            } else {
                Err(recovery_error(
                    "registry transaction root contains an unknown entry",
                ))
            }
        })
        .filter_map(|entry| entry.transpose())
        .collect::<Result<Vec<_>, RegistryRecoveryErrorV1>>()?;
    pending.sort_by(|left, right| left.file_name().cmp(&right.file_name()));
    for directory in pending {
        if directory.join("intent.json").exists() {
            continue;
        }
        reject_reparse_or_symlink(&directory, true)
            .map_err(|_| recovery_error("pending registry journal is unsafe"))?;
        let entries = journal_entry_names(&directory)?;
        if entries.iter().any(|name| name != "intent.tmp") {
            return Err(recovery_error(
                "pre-intent registry journal contains unknown owned state",
            ));
        }
        cleanup_owned_journal(&root, &directory, &entries)?;
    }
    Ok(())
}

fn recover_one_registry_mutation_journal_locked(
    repo_root: &Path,
    journal_path: &str,
) -> Result<(), RegistryRecoveryErrorV1> {
    let normalized = CanonicalWorkspace::new(repo_root)
        .normalize_repo_relative(journal_path)
        .map_err(|_| recovery_error("registry recovery journal path is not normalized"))?;
    let pending = repo_root.join(normalized.as_str());
    let root = repo_root.join(REGISTRY_TRANSACTION_ROOT);
    if pending.parent() != Some(root.as_path())
        || !pending
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with(".pending"))
    {
        return Err(recovery_error(
            "registry recovery journal is outside the pending transaction root",
        ));
    }
    reject_reparse_or_symlink(&root, true)
        .map_err(|_| recovery_error("registry transaction root is unsafe"))?;
    reject_reparse_or_symlink(&pending, true)
        .map_err(|_| recovery_error("pending registry journal is unsafe"))?;
    recover_one_registry_mutation(repo_root, &root, &pending)
}

fn recover_one_registry_mutation(
    repo_root: &Path,
    root: &Path,
    pending: &Path,
) -> Result<(), RegistryRecoveryErrorV1> {
    let intent_path = pending.join("intent.json");
    if !intent_path.exists() {
        let entries = journal_entry_names(pending)?;
        if entries.iter().any(|name| name != "intent.tmp") {
            return Err(recovery_error(
                "pre-intent registry journal contains unknown owned state",
            ));
        }
        cleanup_owned_journal(root, pending, &entries)?;
        return Ok(());
    }

    let intent_bytes = recovery_read(repo_root, &intent_path, 64 * 1024, "registry intent")?;
    let intent: RegistryMutationRecoveryIntentV1 = serde_json::from_slice(&intent_bytes)
        .map_err(|_| recovery_error("pending registry mutation intent is not closed JSON"))?;
    let intent_value = parse_schema_json(&intent_bytes)
        .map_err(|_| recovery_error("pending registry mutation intent has duplicate JSON keys"))?;
    if serde_json_canonicalizer::to_vec(&intent_value)
        .map_err(|_| recovery_error("pending registry mutation intent is not canonicalizable"))?
        != intent_bytes
    {
        return Err(recovery_error(
            "pending registry mutation intent is not exact JCS",
        ));
    }
    validate_recovery_intent(pending, &intent)?;
    let allowed_entries = recovery_allowed_entries(&intent);
    let entries = journal_entry_names(pending)?;
    if entries.iter().any(|name| !allowed_entries.contains(name)) {
        return Err(recovery_error(
            "pending registry mutation journal contains an unknown entry",
        ));
    }
    let committed_head = committed_registry_head_pair(repo_root)?;
    let already_committed = committed_head
        == (
            intent.result_registry_state_fingerprint.clone(),
            intent.transition_fingerprint.clone(),
        )
        && same_committed_registry_journal(repo_root, pending, &intent_bytes)?;
    if !already_committed
        && committed_head
            != (
                intent.prior_registry_state_fingerprint.clone(),
                intent.prior_transition_fingerprint.clone(),
            )
    {
        return Err(recovery_error(
            "pending registry mutation prior state/head is not current",
        ));
    }

    let expected_marker = registry_commit_marker(&intent_bytes);
    let marker = optional_recovery_read(repo_root, &pending.join("committed"), 72, "marker")?;
    let marker_tmp = optional_recovery_read(
        repo_root,
        &pending.join("committed.tmp"),
        72,
        "temporary marker",
    )?;
    if marker.is_some() && marker_tmp.is_some() {
        return Err(recovery_error(
            "registry journal has both committed and temporary markers",
        ));
    }
    if marker
        .as_deref()
        .is_some_and(|bytes| bytes != expected_marker)
        || marker_tmp
            .as_deref()
            .is_some_and(|bytes| bytes != expected_marker)
    {
        return Err(recovery_error(
            "registry commit marker does not exactly bind raw intent bytes",
        ));
    }

    let prior_stage = recovery_read(
        repo_root,
        &pending.join("prior-use-head.json"),
        MAX_RECORD_BYTES,
        "retained prior use head",
    )?;
    validate_recovery_record(
        &prior_stage,
        &intent.prior_use_head_ref,
        &intent.prior_use_head_fingerprint,
        USE_HEAD_KIND,
    )?;
    if DefinitionFingerprint::from_bytes(&prior_stage).as_str()
        != intent.prior_use_head_bytes_fingerprint
        || intent.prior_use_head_ref != intent.result_use_head_ref
    {
        return Err(recovery_error(
            "retained prior use head does not match its exact intent binding",
        ));
    }

    let mut targets = recovery_targets(&intent)?
        .into_iter()
        .map(|target| {
            let stage = optional_recovery_read(
                repo_root,
                &pending.join(target.stage_name),
                MAX_RECORD_BYTES,
                target.stage_name,
            )?;
            if let Some(bytes) = &stage {
                validate_recovery_record(
                    bytes,
                    &target.relative_ref,
                    &target.fingerprint,
                    target.kind,
                )?;
                if target.replace_use_head
                    && DefinitionFingerprint::from_bytes(bytes).as_str()
                        != intent.result_use_head_bytes_fingerprint
                {
                    return Err(recovery_error(
                        "replacement use-head staged bytes disagree with intent",
                    ));
                }
            }
            let final_bytes = optional_state_read(repo_root, &target.relative_ref)?;
            let final_bytes = match final_bytes {
                Some(bytes) if target.replace_use_head && bytes == prior_stage => None,
                Some(bytes) => {
                    validate_recovery_record(
                        &bytes,
                        &target.relative_ref,
                        &target.fingerprint,
                        target.kind,
                    )?;
                    Some(bytes)
                }
                None => None,
            };
            Ok(RecoveryTargetState {
                target,
                stage,
                final_bytes,
            })
        })
        .collect::<Result<Vec<_>, RegistryRecoveryErrorV1>>()?;

    let finals = targets
        .iter()
        .filter(|target| target.final_bytes.is_some())
        .count();
    let complete = finals == targets.len();
    let remaining_stages_complete = targets
        .iter()
        .all(|target| target.final_bytes.is_some() || target.stage.is_some());
    if already_committed {
        if !complete {
            return Err(recovery_error(
                "committed registry journal ancestor is missing an exact final",
            ));
        }
        cleanup_owned_journal(root, pending, &entries)?;
        return Ok(());
    }
    if marker.is_some() {
        if !complete {
            return Err(recovery_error(
                "matching marker is present while an operation-required final is missing",
            ));
        }
        finalize_recovered_registry_journal(repo_root, root, pending, &intent_bytes)?;
        return Ok(());
    }

    if !complete && !remaining_stages_complete {
        if targets
            .iter()
            .find(|target| target.target.replace_use_head)
            .and_then(|target| target.final_bytes.as_ref())
            .is_some()
        {
            restore_prior_use_head(repo_root, &intent.prior_use_head_ref, &prior_stage)?;
        }
        cleanup_owned_journal(root, pending, &entries)?;
        return Ok(());
    }

    if finals == 0 || !complete {
        install_recovery_targets(repo_root, &mut targets, &prior_stage)?;
    }
    if !pending.join("records-installed").exists() {
        write_new_durable(&pending.join("records-installed"), b"installed\n")
            .map_err(|_| recovery_error("records-installed recovery write failed"))?;
    } else if recovery_read(
        repo_root,
        &pending.join("records-installed"),
        10,
        "records-installed",
    )? != b"installed\n"
    {
        return Err(recovery_error("records-installed marker is not exact"));
    }
    if marker_tmp.is_none() {
        write_new_durable(&pending.join("committed.tmp"), &expected_marker)
            .map_err(|_| recovery_error("registry recovery marker write failed"))?;
    }
    fs::rename(pending.join("committed.tmp"), pending.join("committed"))
        .map_err(|_| recovery_error("registry recovery marker rename failed"))?;
    sync_directory(pending)
        .map_err(|_| recovery_error("registry recovery journal fsync failed"))?;
    finalize_recovered_registry_journal(repo_root, root, pending, &intent_bytes)
}

fn validate_recovery_intent(
    pending: &Path,
    intent: &RegistryMutationRecoveryIntentV1,
) -> Result<(), RegistryRecoveryErrorV1> {
    let directory_id = pending
        .file_name()
        .and_then(|name| name.to_str())
        .and_then(|name| name.strip_suffix(".pending"))
        .ok_or_else(|| recovery_error("pending registry journal name is invalid"))?;
    if intent.schema_id != "handbook.registry-mutation-transaction-intent"
        || intent.schema_version != "1.0"
        || !matches!(
            intent.operation.as_str(),
            "add_credential" | "revoke_credential" | "update_mapping"
        )
        || intent.transaction_id != directory_id
        || intent.operation_id.is_empty()
    {
        return Err(recovery_error(
            "pending registry mutation intent header is invalid",
        ));
    }
    for fingerprint in [
        &intent.repository_identity_fingerprint,
        &intent.prior_registry_state_fingerprint,
        &intent.prior_transition_fingerprint,
        &intent.assertion_response_fingerprint,
        &intent.assertion_fingerprint,
        &intent.result_registry_state_fingerprint,
        &intent.transition_fingerprint,
        &intent.use_transition_fingerprint,
        &intent.prior_use_head_fingerprint,
        &intent.prior_use_head_bytes_fingerprint,
        &intent.result_use_head_fingerprint,
        &intent.result_use_head_bytes_fingerprint,
    ] {
        DefinitionFingerprint::parse(fingerprint)
            .map_err(|_| recovery_error("registry mutation intent fingerprint is invalid"))?;
    }
    let prior_state_hex = intent
        .prior_registry_state_fingerprint
        .strip_prefix("sha256:")
        .expect("fingerprint was parsed above");
    let prior_transition_hex = intent
        .prior_transition_fingerprint
        .strip_prefix("sha256:")
        .expect("fingerprint was parsed above");
    if intent.prior_registry_state_ref
        != format!("registry-states/registry-state_{prior_state_hex}.json")
        || intent.prior_transition_ref
            != format!("registry-transitions/registry-transition_{prior_transition_hex}.json")
    {
        return Err(recovery_error(
            "registry mutation intent prior state/transition references disagree",
        ));
    }
    let add = intent.operation == "add_credential";
    if add
        != (intent.response_ref.is_some()
            && intent.response_fingerprint.is_some()
            && intent.registration_ref.is_some()
            && intent.registration_fingerprint.is_some()
            && intent.new_credential_use_head_ref.is_some()
            && intent.new_credential_use_head_fingerprint.is_some())
        || (!add
            && (intent.response_ref.is_some()
                || intent.response_fingerprint.is_some()
                || intent.registration_ref.is_some()
                || intent.registration_fingerprint.is_some()
                || intent.new_credential_use_head_ref.is_some()
                || intent.new_credential_use_head_fingerprint.is_some()))
    {
        return Err(recovery_error(
            "registry mutation intent operation-discriminated fields disagree",
        ));
    }
    Ok(())
}

fn committed_registry_head_pair(
    repo_root: &Path,
) -> Result<(String, String), RegistryRecoveryErrorV1> {
    let root = repo_root.join(REGISTRY_TRANSACTION_ROOT);
    let mut nodes = BTreeMap::<String, (Option<String>, String)>::new();
    let mut referenced = BTreeSet::new();
    for entry in fs::read_dir(&root)
        .map_err(|_| recovery_error("registry transaction root is unreadable"))?
    {
        let entry =
            entry.map_err(|_| recovery_error("registry transaction entry is unreadable"))?;
        let name = entry
            .file_name()
            .to_str()
            .ok_or_else(|| recovery_error("registry transaction name is not UTF-8"))?
            .to_owned();
        if !name.ends_with(".committed") {
            continue;
        }
        reject_reparse_or_symlink(&entry.path(), true)
            .map_err(|_| recovery_error("committed registry journal is unsafe"))?;
        let intent_bytes = recovery_read(
            repo_root,
            &entry.path().join("intent.json"),
            64 * 1024,
            "committed registry intent",
        )?;
        let intent = parse_schema_json(&intent_bytes)
            .map_err(|_| recovery_error("committed registry intent is not closed JSON"))?;
        if serde_json_canonicalizer::to_vec(&intent)
            .map_err(|_| recovery_error("committed registry intent is not canonicalizable"))?
            != intent_bytes
            || recovery_read(
                repo_root,
                &entry.path().join("committed"),
                72,
                "committed registry marker",
            )? != registry_commit_marker(&intent_bytes)
        {
            return Err(recovery_error(
                "committed registry journal does not bind its exact raw intent",
            ));
        }
        let transition_ref = recovery_value_string(&intent, "transition_ref")?;
        let transition_fingerprint = recovery_value_string(&intent, "transition_fingerprint")?;
        let state_fingerprint =
            recovery_value_string(&intent, "result_registry_state_fingerprint")?;
        let transition_bytes = recovery_read(
            repo_root,
            &repo_root.join(".handbook/state").join(transition_ref),
            MAX_RECORD_BYTES,
            "committed registry transition",
        )?;
        validate_recovery_record(
            &transition_bytes,
            transition_ref,
            transition_fingerprint,
            REGISTRY_TRANSITION_KIND,
        )?;
        let transition = parse_schema_json(&transition_bytes)
            .map_err(|_| recovery_error("committed registry transition is not closed JSON"))?;
        if transition.get("result_registry_state_fingerprint")
            != Some(&Value::String(state_fingerprint.to_owned()))
        {
            return Err(recovery_error(
                "committed registry transition and journal state bindings disagree",
            ));
        }
        let prior = match transition.get("prior_transition_fingerprint") {
            Some(Value::Null) => None,
            Some(Value::String(value)) => {
                referenced.insert(value.clone());
                Some(value.clone())
            }
            _ => {
                return Err(recovery_error(
                    "committed registry transition predecessor is invalid",
                ))
            }
        };
        if nodes
            .insert(
                transition_fingerprint.to_owned(),
                (prior, state_fingerprint.to_owned()),
            )
            .is_some()
        {
            return Err(recovery_error(
                "committed registry transition fingerprint is duplicated",
            ));
        }
    }
    let heads = nodes
        .keys()
        .filter(|fingerprint| !referenced.contains(*fingerprint))
        .cloned()
        .collect::<Vec<_>>();
    if heads.len() != 1 {
        return Err(recovery_error(
            "committed registry chain does not have one unique head",
        ));
    }
    let mut cursor = heads[0].clone();
    let mut visited = BTreeSet::new();
    loop {
        if !visited.insert(cursor.clone()) {
            return Err(recovery_error("committed registry chain contains a cycle"));
        }
        match nodes
            .get(&cursor)
            .ok_or_else(|| recovery_error("committed registry chain is incomplete"))?
            .0
            .as_deref()
        {
            Some(prior) => cursor = prior.to_owned(),
            None => break,
        }
    }
    if visited.len() != nodes.len() {
        return Err(recovery_error(
            "committed registry chain contains an unreachable fork",
        ));
    }
    Ok((nodes[&heads[0]].1.clone(), heads[0].clone()))
}

fn same_committed_registry_journal(
    repo_root: &Path,
    pending: &Path,
    intent_bytes: &[u8],
) -> Result<bool, RegistryRecoveryErrorV1> {
    let root = repo_root.join(REGISTRY_TRANSACTION_ROOT);
    let name = pending.file_name().unwrap().to_string_lossy();
    let committed = root.join(format!("{}.committed", name.trim_end_matches(".pending")));
    if !committed.exists() {
        return Ok(false);
    }
    Ok(recovery_read(
        repo_root,
        &committed.join("intent.json"),
        64 * 1024,
        "duplicate committed registry intent",
    )? == intent_bytes
        && recovery_read(
            repo_root,
            &committed.join("committed"),
            72,
            "duplicate committed registry marker",
        )? == registry_commit_marker(intent_bytes))
}

fn recovery_value_string<'a>(
    value: &'a Value,
    field: &str,
) -> Result<&'a str, RegistryRecoveryErrorV1> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|text| !text.is_empty())
        .ok_or_else(|| recovery_error(format!("committed registry field {field} is absent")))
}

fn recovery_value_u64(value: &Value, field: &str) -> Result<u64, RegistryRecoveryErrorV1> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| recovery_error(format!("committed registry field {field} is absent")))
}

fn recovery_targets(
    intent: &RegistryMutationRecoveryIntentV1,
) -> Result<Vec<RecoveryTarget>, RegistryRecoveryErrorV1> {
    let mut targets = Vec::new();
    if intent.operation == "add_credential" {
        targets.extend([
            RecoveryTarget {
                stage_name: "make-response.new",
                relative_ref: intent.response_ref.clone().unwrap(),
                fingerprint: intent.response_fingerprint.clone().unwrap(),
                kind: MAKE_RESPONSE_KIND,
                replace_use_head: false,
            },
            RecoveryTarget {
                stage_name: "registration.new",
                relative_ref: intent.registration_ref.clone().unwrap(),
                fingerprint: intent.registration_fingerprint.clone().unwrap(),
                kind: REGISTRATION_KIND,
                replace_use_head: false,
            },
            RecoveryTarget {
                stage_name: "new-credential-use-head.new",
                relative_ref: intent.new_credential_use_head_ref.clone().unwrap(),
                fingerprint: intent.new_credential_use_head_fingerprint.clone().unwrap(),
                kind: USE_HEAD_KIND,
                replace_use_head: false,
            },
        ]);
    }
    targets.extend([
        RecoveryTarget {
            stage_name: "assertion-response.new",
            relative_ref: intent.assertion_response_ref.clone(),
            fingerprint: intent.assertion_response_fingerprint.clone(),
            kind: ASSERTION_RESPONSE_KIND,
            replace_use_head: false,
        },
        RecoveryTarget {
            stage_name: "assertion.new",
            relative_ref: intent.assertion_ref.clone(),
            fingerprint: intent.assertion_fingerprint.clone(),
            kind: ASSERTION_KIND,
            replace_use_head: false,
        },
        RecoveryTarget {
            stage_name: "registry-state.new",
            relative_ref: intent.result_registry_state_ref.clone(),
            fingerprint: intent.result_registry_state_fingerprint.clone(),
            kind: REGISTRY_STATE_KIND,
            replace_use_head: false,
        },
        RecoveryTarget {
            stage_name: "registry-transition.new",
            relative_ref: intent.transition_ref.clone(),
            fingerprint: intent.transition_fingerprint.clone(),
            kind: REGISTRY_TRANSITION_KIND,
            replace_use_head: false,
        },
        RecoveryTarget {
            stage_name: "use-transition.new",
            relative_ref: intent.use_transition_ref.clone(),
            fingerprint: intent.use_transition_fingerprint.clone(),
            kind: USE_TRANSITION_KIND,
            replace_use_head: false,
        },
        RecoveryTarget {
            stage_name: "use-head.new",
            relative_ref: intent.result_use_head_ref.clone(),
            fingerprint: intent.result_use_head_fingerprint.clone(),
            kind: USE_HEAD_KIND,
            replace_use_head: true,
        },
    ]);
    Ok(targets)
}

fn validate_recovery_record(
    bytes: &[u8],
    relative_ref: &str,
    expected_fingerprint: &str,
    kind: RecoveryRecordKind,
) -> Result<(), RegistryRecoveryErrorV1> {
    let mut value = parse_schema_json(bytes)
        .map_err(|_| recovery_error("registry recovery record is not closed JSON"))?;
    if serde_json_canonicalizer::to_vec(&value)
        .map_err(|_| recovery_error("registry recovery record is not canonicalizable"))?
        != bytes
    {
        return Err(recovery_error("registry recovery record is not exact JCS"));
    }
    let object = value
        .as_object_mut()
        .ok_or_else(|| recovery_error("registry recovery record is not an object"))?;
    if object
        .remove(kind.fingerprint_field)
        .and_then(|value| value.as_str().map(str::to_owned))
        .as_deref()
        != Some(expected_fingerprint)
    {
        return Err(recovery_error(
            "registry recovery record fingerprint binding disagrees",
        ));
    }
    let hex = expected_fingerprint
        .strip_prefix("sha256:")
        .ok_or_else(|| recovery_error("registry recovery fingerprint is not normalized"))?;
    let expected_id = format!("{}_{hex}", kind.id_prefix);
    if let Some(id_field) = kind.id_field {
        if object
            .remove(id_field)
            .and_then(|value| value.as_str().map(str::to_owned))
            .as_deref()
            != Some(expected_id.as_str())
        {
            return Err(recovery_error("registry recovery record ID disagrees"));
        }
    }
    for field in kind.audit_only_fields {
        object.remove(*field);
    }
    if DefinitionFingerprint::from_json_value(&value)
        .map_err(|_| recovery_error("registry recovery record fingerprint failed"))?
        .as_str()
        != expected_fingerprint
        || relative_ref != format!("{}/{expected_id}.json", kind.partition)
            && kind.partition != "authenticator-use-heads"
    {
        return Err(recovery_error(
            "registry recovery record content-addressed identity disagrees",
        ));
    }
    if kind.partition == "authenticator-use-heads"
        && (!relative_ref.starts_with("authenticator-use-heads/")
            || !relative_ref.ends_with(".json")
            || relative_ref.contains(['\\', '/'].as_slice())
                && relative_ref.matches('/').count() != 1)
    {
        return Err(recovery_error("registry recovery use-head path is unsafe"));
    }
    Ok(())
}

fn install_recovery_targets(
    repo_root: &Path,
    targets: &mut [RecoveryTargetState],
    prior_use_head: &[u8],
) -> Result<(), RegistryRecoveryErrorV1> {
    for state in targets
        .iter_mut()
        .filter(|state| !state.target.replace_use_head)
    {
        if state.final_bytes.is_none() {
            let bytes = state
                .stage
                .as_ref()
                .ok_or_else(|| recovery_error("roll-forward stage is absent"))?;
            install_state_file(repo_root, &state.target.relative_ref, bytes, false, None)
                .map_err(|_| recovery_error("registry recovery final install failed"))?;
            state.final_bytes = Some(bytes.clone());
        }
    }
    let result_head = targets
        .iter_mut()
        .find(|state| state.target.replace_use_head)
        .ok_or_else(|| recovery_error("registry recovery result use head is absent"))?;
    if result_head.final_bytes.is_none() {
        let bytes = result_head
            .stage
            .as_ref()
            .ok_or_else(|| recovery_error("registry recovery result use-head stage is absent"))?;
        install_state_file(
            repo_root,
            &result_head.target.relative_ref,
            bytes,
            true,
            Some(prior_use_head),
        )
        .map_err(|_| recovery_error("registry recovery use-head replacement failed"))?;
        result_head.final_bytes = Some(bytes.clone());
    }
    Ok(())
}

fn restore_prior_use_head(
    repo_root: &Path,
    relative_ref: &str,
    prior: &[u8],
) -> Result<(), RegistryRecoveryErrorV1> {
    let target = repo_root.join(".handbook/state").join(relative_ref);
    let parent = target
        .parent()
        .ok_or_else(|| recovery_error("prior use-head target has no parent"))?;
    let temporary = parent.join(format!(
        ".{}.registry-recovering",
        target.file_name().unwrap().to_string_lossy()
    ));
    if temporary.exists() {
        return Err(recovery_error(
            "registry recovery temporary use-head path already exists",
        ));
    }
    write_new_durable(&temporary, prior)
        .map_err(|_| recovery_error("prior use-head recovery write failed"))?;
    fs::rename(&temporary, &target)
        .map_err(|_| recovery_error("prior use-head recovery rename failed"))?;
    sync_directory(parent).map_err(|_| recovery_error("prior use-head recovery fsync failed"))
}

fn finalize_recovered_registry_journal(
    repo_root: &Path,
    root: &Path,
    pending: &Path,
    intent_bytes: &[u8],
) -> Result<(), RegistryRecoveryErrorV1> {
    let name = pending.file_name().unwrap().to_string_lossy();
    let committed = root.join(format!("{}.committed", name.trim_end_matches(".pending")));
    if committed.exists() {
        let existing_intent = recovery_read(
            repo_root,
            &committed.join("intent.json"),
            64 * 1024,
            "existing committed intent",
        )?;
        let existing_marker = recovery_read(
            repo_root,
            &committed.join("committed"),
            72,
            "existing committed marker",
        )?;
        if existing_intent != intent_bytes
            || existing_marker != registry_commit_marker(intent_bytes)
        {
            return Err(recovery_error(
                "registry recovery collides with different committed journal bytes",
            ));
        }
        let entries = journal_entry_names(pending)?;
        cleanup_owned_journal(root, pending, &entries)?;
        return Ok(());
    }
    fs::rename(pending, &committed)
        .map_err(|_| recovery_error("registry recovery finalization rename failed"))?;
    sync_directory(root).map_err(|_| recovery_error("registry transaction root fsync failed"))
}

fn recovery_allowed_entries(intent: &RegistryMutationRecoveryIntentV1) -> BTreeSet<String> {
    let mut names = BTreeSet::from([
        "intent.json".to_owned(),
        "assertion-response.new".to_owned(),
        "assertion.new".to_owned(),
        "registry-state.new".to_owned(),
        "registry-transition.new".to_owned(),
        "use-transition.new".to_owned(),
        "prior-use-head.json".to_owned(),
        "use-head.new".to_owned(),
        "records-installed".to_owned(),
        "committed.tmp".to_owned(),
        "committed".to_owned(),
    ]);
    if intent.operation == "add_credential" {
        names.extend([
            "make-response.new".to_owned(),
            "registration.new".to_owned(),
            "new-credential-use-head.new".to_owned(),
        ]);
    }
    names
}

fn journal_entry_names(directory: &Path) -> Result<Vec<String>, RegistryRecoveryErrorV1> {
    let mut names = Vec::new();
    for entry in
        fs::read_dir(directory).map_err(|_| recovery_error("registry journal is unreadable"))?
    {
        let entry = entry.map_err(|_| recovery_error("registry journal entry is unreadable"))?;
        reject_reparse_or_symlink(&entry.path(), false)
            .map_err(|_| recovery_error("registry journal entry is unsafe"))?;
        names.push(
            entry
                .file_name()
                .to_str()
                .ok_or_else(|| recovery_error("registry journal entry name is not UTF-8"))?
                .to_owned(),
        );
    }
    names.sort();
    Ok(names)
}

fn cleanup_owned_journal(
    root: &Path,
    directory: &Path,
    entries: &[String],
) -> Result<(), RegistryRecoveryErrorV1> {
    for name in entries {
        fs::remove_file(directory.join(name))
            .map_err(|_| recovery_error("owned registry journal cleanup failed"))?;
    }
    fs::remove_dir(directory)
        .map_err(|_| recovery_error("owned registry journal directory cleanup failed"))?;
    sync_directory(root).map_err(|_| recovery_error("registry cleanup root fsync failed"))
}

fn optional_state_read(
    repo_root: &Path,
    relative_ref: &str,
) -> Result<Option<Vec<u8>>, RegistryRecoveryErrorV1> {
    if relative_ref.contains('\\') || relative_ref.starts_with('/') || relative_ref.contains("..") {
        return Err(recovery_error("registry recovery final path is unsafe"));
    }
    optional_recovery_read(
        repo_root,
        &repo_root.join(".handbook/state").join(relative_ref),
        MAX_RECORD_BYTES,
        "registry recovery final",
    )
}

fn optional_recovery_read(
    repo_root: &Path,
    path: &Path,
    limit: usize,
    label: &str,
) -> Result<Option<Vec<u8>>, RegistryRecoveryErrorV1> {
    if !path.exists() {
        return Ok(None);
    }
    recovery_read(repo_root, path, limit, label).map(Some)
}

fn recovery_read(
    repo_root: &Path,
    path: &Path,
    limit: usize,
    label: &str,
) -> Result<Vec<u8>, RegistryRecoveryErrorV1> {
    read_registry_journal_file(repo_root, path, limit, label)
        .map_err(|_| recovery_error(format!("{label} is unsafe, missing, or oversized")))
}

fn recovery_error(detail: impl Into<String>) -> RegistryRecoveryErrorV1 {
    RegistryRecoveryErrorV1 {
        detail: detail.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use p256::ecdsa::{signature::Signer, Signature, SigningKey};
    use serde_json::Value;

    #[test]
    fn registry_counter_protocol_preserves_zero_and_monotonic_boundaries() {
        assert!(validate_counter(0, 0).is_ok());
        assert!(validate_counter(0, u32::MAX).is_ok());
        assert!(validate_counter(1, 2).is_ok());
        assert!(validate_counter(u32::MAX - 1, u32::MAX).is_ok());
        assert!(validate_counter(1, 1).is_err());
        assert!(validate_counter(2, 1).is_err());
        assert!(validate_counter(u32::MAX, 0).is_err());
    }

    const ADMIN_FIXTURE: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/approver-admin-api-vectors-v1.0.json"
    ));
    const FINAL_USE_FIXTURE: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/final-use-add-lockout-vectors-v1.0.json"
    ));
    const SECURITY_BOUNDARY_FIXTURE: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/security-boundary-limit-vectors-v1.0.json"
    ));

    #[test]
    fn sequence_4095_incomplete_add_fixture_refuses_before_native_io() {
        let fixture: Value = serde_json::from_slice(FINAL_USE_FIXTURE).unwrap();
        let vector = &fixture["vectors"][0];
        let required = vector["required_coverage_pairs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|pair| {
                (
                    pair["approval_class"].as_str().unwrap(),
                    pair["authority_ref"].as_str().unwrap(),
                )
            })
            .collect::<Vec<_>>();
        let proposed = vector["proposed_new_credential"]["approval_mappings"]
            .as_array()
            .unwrap()
            .iter()
            .map(|pair| {
                (
                    pair["approval_class"].as_str().unwrap(),
                    pair["authority_ref"].as_str().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        assert!(final_use_add_would_lock_out(4_095, &required, &proposed));
        assert_eq!(vector["expected_native_call_log"], json!([]));
        assert_eq!(vector["expected_result"]["changed_paths"], json!([]));
    }

    #[test]
    fn registry_mutation_limits_equal_the_security_boundary_fixture() {
        let fixture: Value = serde_json::from_slice(SECURITY_BOUNDARY_FIXTURE).unwrap();
        let vectors = fixture["vectors"].as_array().unwrap();
        let limit = |schema: &str, field: &str| {
            vectors
                .iter()
                .find(|row| row["schema_file"] == schema && row["field"] == field)
                .unwrap()["limit"]
                .as_u64()
                .unwrap() as usize
        };

        assert_eq!(
            limit("approver-admin-api-1.0.0.schema.json", "approval_mappings"),
            MAX_APPROVAL_MAPPINGS
        );
        assert_eq!(
            limit("approver-registry-1.0.0.schema.json", "credentials"),
            MAX_REGISTRY_CREDENTIALS
        );
        assert_eq!(
            limit(
                "approver-registry-1.0.0.schema.json",
                "cose_public_key_base64"
            ),
            MAX_COSE_PUBLIC_KEY_BASE64
        );
        assert_eq!(
            limit("approver-admin-api-1.0.0.schema.json", "next_actions"),
            MAX_NEXT_ACTIONS
        );
    }

    #[derive(Default)]
    struct NoNativeCalls {
        calls: usize,
    }

    impl NativeAuthenticatorPortV1 for NoNativeCalls {
        fn make_credential(
            &mut self,
            _request_cbor: &[u8],
        ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
            self.calls += 1;
            panic!("refused preflight must not call makeCredential")
        }

        fn get_assertion(
            &mut self,
            _request_cbor: &[u8],
        ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
            self.calls += 1;
            panic!("refused preflight must not call GetAssertion")
        }
    }

    fn add_request() -> ApproverAdminRequestV1 {
        let fixture: Value = serde_json::from_slice(ADMIN_FIXTURE).unwrap();
        let document = fixture["vectors"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["vector_id"] == "add-request")
            .unwrap()["document"]
            .clone();
        ApproverAdminRequestV1::from_json_value(document).unwrap()
    }

    fn bootstrap_request() -> ApproverAdminRequestV1 {
        let fixture: Value = serde_json::from_slice(ADMIN_FIXTURE).unwrap();
        let document = fixture["vectors"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["vector_id"] == "bootstrap-request")
            .unwrap()["document"]
            .clone();
        ApproverAdminRequestV1::from_json_value(document).unwrap()
    }

    #[test]
    fn missing_committed_registry_refuses_all_null_before_native_io() {
        let temp = tempfile::tempdir().unwrap();
        let mut port = NoNativeCalls::default();
        let result = add_credential(temp.path(), &mut port, &add_request())
            .to_json_value()
            .unwrap();

        assert_eq!(result["status"], "refused");
        assert_eq!(result["changed_paths"], json!([]));
        for field in [
            "assertion_fingerprint",
            "assertion_ref",
            "prior_registry_state_fingerprint",
            "prior_registry_state_ref",
            "registration_fingerprint",
            "registration_ref",
            "result_registry_state_fingerprint",
            "result_registry_state_ref",
            "transition_fingerprint",
            "transition_ref",
        ] {
            assert_eq!(result[field], Value::Null, "{field}");
        }
        assert_eq!(port.calls, 0);
    }

    struct SigningPort {
        admin_key: SigningKey,
        added_key: SigningKey,
        admin_credential: Vec<u8>,
        added_credential: Vec<u8>,
        make_calls: usize,
        assertion_calls: usize,
    }

    impl SigningPort {
        fn new() -> Self {
            Self {
                admin_key: SigningKey::from_slice(&[1_u8; 32]).unwrap(),
                added_key: SigningKey::from_slice(&[2_u8; 32]).unwrap(),
                admin_credential: vec![0x11; 32],
                added_credential: vec![0x22; 32],
                make_calls: 0,
                assertion_calls: 0,
            }
        }
    }

    impl NativeAuthenticatorPortV1 for SigningPort {
        fn make_credential(
            &mut self,
            _request_cbor: &[u8],
        ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
            self.make_calls += 1;
            Ok(if self.make_calls == 1 {
                make_credential_response(&self.admin_key, &self.admin_credential)
            } else {
                make_credential_response(&self.added_key, &self.added_credential)
            })
        }

        fn get_assertion(
            &mut self,
            request_cbor: &[u8],
        ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
            self.assertion_calls += 1;
            let marker = request_cbor
                .windows(3)
                .position(|window| window == [0x02, 0x58, 0x20])
                .expect("GetAssertion client-data hash marker");
            let client_data_hash: [u8; 32] =
                request_cbor[marker + 3..marker + 35].try_into().unwrap();
            Ok(assertion_response(
                &self.admin_key,
                &self.admin_credential,
                client_data_hash,
                self.assertion_calls as u32,
            ))
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

    #[test]
    fn add_revoke_and_update_commit_dynamic_records_and_one_shared_use_chain() {
        let temp = tempfile::tempdir().unwrap();
        let mut service = crate::approver_registry::ApproverRegistryServiceV1::for_repository(
            temp.path(),
            SigningPort::new(),
        );
        let bootstrap = service
            .bootstrap_approver_registry(bootstrap_request())
            .to_json_value()
            .unwrap();
        assert_eq!(bootstrap["status"], "succeeded");
        let mut add = add_request().to_json_value().unwrap();
        add["expected_registry_state_fingerprint"] =
            bootstrap["result_registry_state_fingerprint"].clone();
        add["expected_transition_fingerprint"] = bootstrap["transition_fingerprint"].clone();
        let add = ApproverAdminRequestV1::from_json_value(add).unwrap();

        let result = service
            .add_approver_credential(add)
            .to_json_value()
            .unwrap();
        assert_eq!(result["status"], "succeeded", "{result:#}");
        assert_eq!(result["changed_paths"].as_array().unwrap().len(), 6);
        assert!(result["registration_ref"].is_string());
        assert!(result["assertion_ref"].is_string());

        let fixture: Value = serde_json::from_slice(ADMIN_FIXTURE).unwrap();
        let request_document = |vector_id: &str| {
            fixture["vectors"]
                .as_array()
                .unwrap()
                .iter()
                .find(|row| row["vector_id"] == vector_id)
                .unwrap()["document"]
                .clone()
        };
        let mut revoke = request_document("revoke-request");
        revoke["credential_id_hash"] =
            json!(DefinitionFingerprint::from_bytes(&[0x22; 32]).to_string());
        revoke["expected_registry_state_fingerprint"] =
            result["result_registry_state_fingerprint"].clone();
        revoke["expected_transition_fingerprint"] = result["transition_fingerprint"].clone();
        let revoke = service
            .revoke_approver_credential(ApproverAdminRequestV1::from_json_value(revoke).unwrap())
            .to_json_value()
            .unwrap();
        assert_eq!(revoke["status"], "succeeded", "{revoke:#}");
        assert_eq!(revoke["changed_paths"].as_array().unwrap().len(), 4);
        assert!(revoke["registration_ref"].is_null());

        let mut update = request_document("update-request");
        update["credential_id_hash"] =
            json!(DefinitionFingerprint::from_bytes(&[0x11; 32]).to_string());
        update["approval_mappings"] = json!([
            {"approval_class": "Project owner approval", "authority_ref": "Project owner"},
            {"approval_class": "registry_admin", "authority_ref": "repository_registry_admin"}
        ]);
        update["expected_registry_state_fingerprint"] =
            revoke["result_registry_state_fingerprint"].clone();
        update["expected_transition_fingerprint"] = revoke["transition_fingerprint"].clone();
        let update = service
            .update_approver_mapping(ApproverAdminRequestV1::from_json_value(update).unwrap())
            .to_json_value()
            .unwrap();
        assert_eq!(update["status"], "succeeded", "{update:#}");
        assert_eq!(update["changed_paths"].as_array().unwrap().len(), 4);
        let port = service.into_port();
        assert_eq!(port.make_calls, 2);
        assert_eq!(port.assertion_calls, 3);
    }

    #[test]
    fn frozen_sequence_4095_incomplete_add_refuses_before_either_native_call() {
        let temp = tempfile::tempdir().unwrap();
        let mut service = crate::approver_registry::ApproverRegistryServiceV1::for_repository(
            temp.path(),
            SigningPort::new(),
        );
        let bootstrap = service
            .bootstrap_approver_registry(bootstrap_request())
            .to_json_value()
            .unwrap();
        let use_head_path = temp
            .path()
            .join(".handbook/state/authenticator-use-heads")
            .read_dir()
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        let mut use_head: Value =
            serde_json::from_slice(&fs::read(&use_head_path).unwrap()).unwrap();
        use_head.as_object_mut().unwrap().remove("head_fingerprint");
        use_head["sequence"] = json!(4_095);
        use_head["sign_count"] = json!(4_095);
        use_head["last_assertion_ref"] = json!(
            "authenticator-assertions/authenticator-assertion_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json"
        );
        use_head["last_assertion_fingerprint"] =
            json!("sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        use_head["last_challenge_fingerprint"] =
            json!("sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
        use_head["last_nonce_sha256"] =
            json!("sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc");
        let head_fingerprint = DefinitionFingerprint::from_json_value(&use_head)
            .unwrap()
            .to_string();
        use_head["head_fingerprint"] = json!(head_fingerprint);
        fs::write(
            &use_head_path,
            serde_json_canonicalizer::to_vec(&use_head).unwrap(),
        )
        .unwrap();

        let fixture: Value = serde_json::from_slice(FINAL_USE_FIXTURE).unwrap();
        let vector = &fixture["vectors"][0];
        let mut request = vector["exact_request"].clone();
        request["expected_registry_state_fingerprint"] =
            bootstrap["result_registry_state_fingerprint"].clone();
        request["expected_transition_fingerprint"] = bootstrap["transition_fingerprint"].clone();
        let result = service
            .add_approver_credential(ApproverAdminRequestV1::from_json_value(request).unwrap())
            .to_json_value()
            .unwrap();

        assert_eq!(result, vector["expected_result"]);
        let port = service.into_port();
        assert_eq!(
            port.make_calls, 1,
            "only bootstrap makeCredential is allowed"
        );
        assert_eq!(port.assertion_calls, 0);
    }

    fn dynamic_pending_fixture() -> (tempfile::TempDir, PathBuf, Value) {
        let temp = tempfile::tempdir().unwrap();
        let mut service = crate::approver_registry::ApproverRegistryServiceV1::for_repository(
            temp.path(),
            SigningPort::new(),
        );
        let bootstrap = service
            .bootstrap_approver_registry(bootstrap_request())
            .to_json_value()
            .unwrap();
        let mut add = add_request().to_json_value().unwrap();
        add["expected_registry_state_fingerprint"] =
            bootstrap["result_registry_state_fingerprint"].clone();
        add["expected_transition_fingerprint"] = bootstrap["transition_fingerprint"].clone();
        let result = service
            .add_approver_credential(ApproverAdminRequestV1::from_json_value(add).unwrap())
            .to_json_value()
            .unwrap();
        assert_eq!(result["status"], "succeeded", "{result:#}");

        let root = temp.path().join(".handbook/state/transactions/registry");
        let committed = root
            .read_dir()
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|directory| {
                fs::read(directory.join("intent.json"))
                    .ok()
                    .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok())
                    .and_then(|intent| intent["schema_id"].as_str().map(str::to_owned))
                    .as_deref()
                    == Some("handbook.registry-mutation-transaction-intent")
            })
            .unwrap();
        let intent: Value =
            serde_json::from_slice(&fs::read(committed.join("intent.json")).unwrap()).unwrap();
        let pending = committed.with_file_name(format!(
            "{}.pending",
            committed
                .file_name()
                .unwrap()
                .to_string_lossy()
                .trim_end_matches(".committed")
        ));
        fs::rename(&committed, &pending).unwrap();
        (temp, pending, intent)
    }

    fn intent_ref(intent: &Value, field: &str) -> String {
        intent[field].as_str().unwrap().to_owned()
    }

    fn state_path(repo_root: &Path, intent: &Value, field: &str) -> PathBuf {
        repo_root
            .join(".handbook/state")
            .join(intent_ref(intent, field))
    }

    fn restore_prior_head_from_stage(repo_root: &Path, pending: &Path, intent: &Value) {
        fs::write(
            state_path(repo_root, intent, "prior_use_head_ref"),
            fs::read(pending.join("prior-use-head.json")).unwrap(),
        )
        .unwrap();
    }

    fn remove_dynamic_finals(repo_root: &Path, intent: &Value) {
        for field in [
            "response_ref",
            "registration_ref",
            "assertion_response_ref",
            "assertion_ref",
            "result_registry_state_ref",
            "transition_ref",
            "use_transition_ref",
            "new_credential_use_head_ref",
        ] {
            if let Some(reference) = intent[field].as_str() {
                let path = repo_root.join(".handbook/state").join(reference);
                if path.exists() {
                    fs::remove_file(path).unwrap();
                }
            }
        }
    }

    #[test]
    fn registry_recovery_rolls_forward_a_complete_staged_mutation() {
        let (temp, pending, intent) = dynamic_pending_fixture();
        fs::remove_file(pending.join("committed")).unwrap();
        restore_prior_head_from_stage(temp.path(), &pending, &intent);
        remove_dynamic_finals(temp.path(), &intent);

        recover_registry_mutation_journals(temp.path()).unwrap();

        assert!(!pending.exists());
        let committed = pending.with_file_name(format!(
            "{}.committed",
            pending
                .file_name()
                .unwrap()
                .to_string_lossy()
                .trim_end_matches(".pending")
        ));
        assert_eq!(
            fs::read(committed.join("committed")).unwrap(),
            registry_commit_marker(&fs::read(committed.join("intent.json")).unwrap())
        );
        assert_eq!(
            fs::read(state_path(temp.path(), &intent, "result_use_head_ref")).unwrap(),
            fs::read(committed.join("use-head.new")).unwrap()
        );
        assert_eq!(
            fs::read(state_path(temp.path(), &intent, "transition_ref")).unwrap(),
            fs::read(committed.join("registry-transition.new")).unwrap()
        );
    }

    #[test]
    fn registry_recovery_restores_prior_head_for_partial_final_without_stage() {
        let (temp, pending, intent) = dynamic_pending_fixture();
        fs::remove_file(pending.join("committed")).unwrap();
        let expected_prior = fs::read(pending.join("prior-use-head.json")).unwrap();
        let orphan = state_path(temp.path(), &intent, "assertion_ref");
        let orphan_bytes = fs::read(&orphan).unwrap();
        for field in [
            "response_ref",
            "registration_ref",
            "assertion_response_ref",
            "result_registry_state_ref",
            "transition_ref",
            "use_transition_ref",
            "new_credential_use_head_ref",
        ] {
            if let Some(reference) = intent[field].as_str() {
                let path = temp.path().join(".handbook/state").join(reference);
                if path.exists() {
                    fs::remove_file(path).unwrap();
                }
            }
        }
        fs::remove_file(pending.join("registry-transition.new")).unwrap();

        recover_registry_mutation_journals(temp.path()).unwrap();

        assert!(!pending.exists());
        assert_eq!(fs::read(&orphan).unwrap(), orphan_bytes);
        assert_eq!(
            fs::read(state_path(temp.path(), &intent, "prior_use_head_ref")).unwrap(),
            expected_prior
        );
    }

    #[test]
    fn registry_recovery_finalizes_exact_matching_marker_idempotently() {
        let (temp, pending, _) = dynamic_pending_fixture();

        recover_registry_mutation_journals(temp.path()).unwrap();

        assert!(!pending.exists());
        assert_eq!(
            temp.path()
                .join(".handbook/state/transactions/registry")
                .read_dir()
                .unwrap()
                .filter_map(Result::ok)
                .filter(|entry| entry.file_name().to_string_lossy().ends_with(".committed"))
                .count(),
            2
        );
    }

    #[test]
    fn registry_recovery_preserves_matching_marker_with_missing_final() {
        let (temp, pending, intent) = dynamic_pending_fixture();
        fs::remove_file(state_path(temp.path(), &intent, "transition_ref")).unwrap();

        let error = recover_registry_mutation_journals(temp.path()).unwrap_err();

        assert!(error.detail().contains("matching marker"));
        assert!(pending.exists());
    }

    #[test]
    fn registry_recovery_refuses_unknown_owned_state_without_deleting_evidence() {
        let (temp, pending, _) = dynamic_pending_fixture();
        fs::write(pending.join("foreign.bin"), b"unowned").unwrap();

        let error = recover_registry_mutation_journals(temp.path()).unwrap_err();

        assert!(error.detail().contains("unknown"));
        assert!(pending.join("foreign.bin").exists());
    }

    fn shared_node(
        family: SharedUseJournalFamilyV1,
        path: &str,
        prior: &str,
        prior_sequence: u64,
        result: &str,
        result_sequence: u64,
    ) -> SharedUseJournalObservationV1 {
        SharedUseJournalObservationV1 {
            family,
            journal_path: path.to_owned(),
            transaction_id: path.replace('/', "_"),
            credential_id_hash:
                "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
            prior_use_head_ref: "authenticator-use-heads/credential.json".to_owned(),
            prior_use_head_fingerprint: prior.to_owned(),
            prior_use_sequence: prior_sequence,
            result_use_head_ref: "authenticator-use-heads/credential.json".to_owned(),
            result_use_head_fingerprint: result.to_owned(),
            result_use_sequence: result_sequence,
            marker_exact: false,
            finalized_directory: false,
        }
    }

    #[test]
    fn registry_use_journal_discovery_exposes_exact_dynamic_predecessor_node() {
        let (temp, _, _) = dynamic_pending_fixture();

        let nodes = discover_registry_use_journals(temp.path()).unwrap();

        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].family, SharedUseJournalFamilyV1::Registry);
        assert_eq!(nodes[0].prior_use_sequence, 0);
        assert_eq!(nodes[0].result_use_sequence, 1);
        assert!(nodes[0].marker_exact);
        assert!(!nodes[0].finalized_directory);
    }

    #[test]
    fn shared_use_journal_discovery_orders_a_real_registry_predecessor() {
        let (temp, _, _) = dynamic_pending_fixture();

        let nodes = discover_shared_use_journal_graph(temp.path()).unwrap();

        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].family, SharedUseJournalFamilyV1::Registry);
        assert_eq!(nodes[0].prior_use_sequence, 0);
        assert_eq!(nodes[0].result_use_sequence, 1);
    }

    #[test]
    fn committed_registry_chain_head_recovers_and_observes_dynamic_successor() {
        let (temp, pending, intent) = dynamic_pending_fixture();

        let head = observe_committed_registry_chain_head(temp.path()).unwrap();

        assert!(!pending.exists());
        assert_eq!(
            head.state_ref,
            intent["result_registry_state_ref"].as_str().unwrap()
        );
        assert_eq!(
            head.state_fingerprint,
            intent["result_registry_state_fingerprint"]
                .as_str()
                .unwrap()
        );
        assert_eq!(
            head.transition_ref,
            intent["transition_ref"].as_str().unwrap()
        );
        assert_eq!(
            head.transition_fingerprint,
            intent["transition_fingerprint"].as_str().unwrap()
        );
    }

    #[test]
    fn shared_use_graph_follows_registry_then_approval_predecessor_reachability() {
        let anchor = SharedUseHeadAnchorV1 {
            credential_id_hash:
                "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
            use_head_ref: "authenticator-use-heads/credential.json".to_owned(),
            use_head_fingerprint:
                "sha256:0000000000000000000000000000000000000000000000000000000000000000".to_owned(),
            use_sequence: 0,
        };
        let registry = shared_node(
            SharedUseJournalFamilyV1::Registry,
            "registry/001",
            &anchor.use_head_fingerprint,
            0,
            "sha256:1111111111111111111111111111111111111111111111111111111111111111",
            1,
        );
        let approval = shared_node(
            SharedUseJournalFamilyV1::Approval,
            "approval/001",
            &registry.result_use_head_fingerprint,
            1,
            "sha256:2222222222222222222222222222222222222222222222222222222222222222",
            2,
        );

        let chain =
            compose_shared_use_journal_graph(&[anchor], vec![registry], vec![approval]).unwrap();

        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0].family, SharedUseJournalFamilyV1::Registry);
        assert_eq!(chain[1].family, SharedUseJournalFamilyV1::Approval);
    }

    #[test]
    fn shared_use_graph_rejects_cross_family_fork_and_unreachable_optimism() {
        let anchor = SharedUseHeadAnchorV1 {
            credential_id_hash:
                "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
            use_head_ref: "authenticator-use-heads/credential.json".to_owned(),
            use_head_fingerprint:
                "sha256:0000000000000000000000000000000000000000000000000000000000000000".to_owned(),
            use_sequence: 0,
        };
        let registry = shared_node(
            SharedUseJournalFamilyV1::Registry,
            "registry/001",
            &anchor.use_head_fingerprint,
            0,
            "sha256:1111111111111111111111111111111111111111111111111111111111111111",
            1,
        );
        let fork = shared_node(
            SharedUseJournalFamilyV1::Approval,
            "approval/fork",
            &anchor.use_head_fingerprint,
            0,
            "sha256:2222222222222222222222222222222222222222222222222222222222222222",
            1,
        );
        let error = compose_shared_use_journal_graph(
            std::slice::from_ref(&anchor),
            vec![registry],
            vec![fork],
        )
        .unwrap_err();
        assert!(error.detail().contains("fork"));

        let unreachable = shared_node(
            SharedUseJournalFamilyV1::Approval,
            "approval/unreachable",
            "sha256:3333333333333333333333333333333333333333333333333333333333333333",
            3,
            "sha256:4444444444444444444444444444444444444444444444444444444444444444",
            4,
        );
        let error =
            compose_shared_use_journal_graph(&[anchor], Vec::new(), vec![unreachable]).unwrap_err();
        assert!(error.detail().contains("unreachable"));
    }

    #[test]
    fn shared_use_graph_rejects_duplicate_and_non_adjacent_sequences() {
        let anchor = SharedUseHeadAnchorV1 {
            credential_id_hash:
                "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
            use_head_ref: "authenticator-use-heads/credential.json".to_owned(),
            use_head_fingerprint:
                "sha256:0000000000000000000000000000000000000000000000000000000000000000".to_owned(),
            use_sequence: 0,
        };
        let skipped = shared_node(
            SharedUseJournalFamilyV1::Registry,
            "registry/skipped",
            &anchor.use_head_fingerprint,
            0,
            "sha256:1111111111111111111111111111111111111111111111111111111111111111",
            2,
        );
        let error = compose_shared_use_journal_graph(
            std::slice::from_ref(&anchor),
            vec![skipped],
            Vec::new(),
        )
        .unwrap_err();
        assert!(error.detail().contains("non-adjacent"));

        let first = shared_node(
            SharedUseJournalFamilyV1::Registry,
            "registry/first",
            &anchor.use_head_fingerprint,
            0,
            "sha256:1111111111111111111111111111111111111111111111111111111111111111",
            1,
        );
        let duplicate_sequence = shared_node(
            SharedUseJournalFamilyV1::Approval,
            "approval/duplicate",
            "sha256:2222222222222222222222222222222222222222222222222222222222222222",
            0,
            "sha256:3333333333333333333333333333333333333333333333333333333333333333",
            1,
        );
        let error =
            compose_shared_use_journal_graph(&[anchor], vec![first], vec![duplicate_sequence])
                .unwrap_err();
        assert!(error.detail().contains("duplicate sequence"));
    }

    #[test]
    fn complete_shared_recovery_rolls_forward_a_pending_registry_predecessor() {
        let (temp, pending, intent) = dynamic_pending_fixture();
        fs::remove_file(pending.join("committed")).unwrap();
        restore_prior_head_from_stage(temp.path(), &pending, &intent);
        remove_dynamic_finals(temp.path(), &intent);

        recover_complete_registry_approval_authority_locked(temp.path()).unwrap();

        assert!(!pending.exists());
        assert_eq!(
            fs::read(state_path(temp.path(), &intent, "result_use_head_ref")).unwrap(),
            fs::read(
                pending
                    .with_file_name(format!(
                        "{}.committed",
                        pending
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .trim_end_matches(".pending")
                    ))
                    .join("use-head.new")
            )
            .unwrap()
        );
    }
}
