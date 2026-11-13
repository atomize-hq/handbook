use crate::approver_registry_mutation::{
    observe_committed_registry_chain_head_for_repository_preflight_locked,
    observe_committed_registry_chain_head_if_present_locked,
    observe_committed_registry_chain_head_locked, CommittedRegistryChainHeadV1,
};
use crate::charter_authority_workflow::{read_registry_journal_file, RegistryAuthorityLocks};
use crate::charter_lineage_store::{LineageRecordClassV1, TrustedLineageStoreV1};
use crate::repository_durability::RetainedRepositoryTreeGuardV1;
use crate::{parse_schema_json, DefinitionFingerprint, OwnerCausalDispositionV1};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fmt;
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApproverAuthorityPairV1 {
    pub approval_class: String,
    pub authority_ref: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedApproverCredentialV1 {
    pub credential_id_hash: String,
    pub credential_id: Vec<u8>,
    pub cose_public_key: Vec<u8>,
    pub active: bool,
    pub approval_mappings: Vec<ApproverAuthorityPairV1>,
    pub registration_ref: String,
    pub registration_fingerprint: String,
    pub use_head_ref: String,
    pub use_head_fingerprint: String,
    pub use_sequence: u16,
    pub sign_count: u32,
}

pub struct CommittedApproverRegistryObservationV1 {
    pub state_ref: String,
    pub state_fingerprint: String,
    pub state_bytes: Vec<u8>,
    pub state: Value,
    pub head_transition_ref: String,
    pub head_transition_fingerprint: String,
    pub head_transition_bytes: Vec<u8>,
    pub head_transition: Value,
    pub credentials: Vec<CommittedApproverCredentialV1>,
    _locks: RegistryAuthorityLocks,
}

pub(crate) struct RetainedApproverRegistryObservationV1 {
    pub(crate) state_ref: String,
    pub(crate) state_fingerprint: String,
    pub(crate) state_bytes: Vec<u8>,
    pub(crate) state: Value,
    pub(crate) head_transition_ref: String,
    pub(crate) head_transition_fingerprint: String,
    pub(crate) head_transition_bytes: Vec<u8>,
    pub(crate) credentials: Vec<CommittedApproverCredentialV1>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ApproverRegistryObservationErrorKindV1 {
    AuthorityAbsent,
    DurabilityViolation,
    UnsafeFilesystem,
    LineageViolation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApproverRegistryObservationErrorV1 {
    kind: ApproverRegistryObservationErrorKindV1,
    detail: String,
    causal_disposition: OwnerCausalDispositionV1,
}

impl ApproverRegistryObservationErrorV1 {
    pub fn kind(&self) -> ApproverRegistryObservationErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn causal_disposition(&self) -> OwnerCausalDispositionV1 {
        self.causal_disposition
    }
}

impl fmt::Display for ApproverRegistryObservationErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for ApproverRegistryObservationErrorV1 {}

pub fn observe_committed_approver_registry(
    repo_root: impl AsRef<Path>,
) -> Result<CommittedApproverRegistryObservationV1, ApproverRegistryObservationErrorV1> {
    let repo_root = repo_root.as_ref();
    let locks = RegistryAuthorityLocks::acquire(repo_root).map_err(|error| {
        unsafe_error(
            "approver authority locks could not be acquired safely",
            error.causal_disposition(),
        )
    })?;
    let retained = observe_committed_approver_registry_locked(repo_root)?;
    let head_transition = parse_schema_json(&retained.head_transition_bytes)
        .map_err(|_| lineage_error("retained registry head is not closed JSON"))?;
    Ok(CommittedApproverRegistryObservationV1 {
        state_ref: retained.state_ref,
        state_fingerprint: retained.state_fingerprint,
        state_bytes: retained.state_bytes,
        state: retained.state,
        head_transition_ref: retained.head_transition_ref,
        head_transition_fingerprint: retained.head_transition_fingerprint,
        head_transition_bytes: retained.head_transition_bytes,
        head_transition,
        credentials: retained.credentials,
        _locks: locks,
    })
}

pub(crate) fn observe_committed_approver_registry_locked(
    repo_root: &Path,
) -> Result<RetainedApproverRegistryObservationV1, ApproverRegistryObservationErrorV1> {
    let head = observe_committed_registry_chain_head_locked(repo_root)
        .map_err(|error| durability_with_cause(error.detail(), error.causal_disposition()))?;
    resolve_retained_registry_head(repo_root, head)
}

pub(crate) fn observe_committed_approver_registry_retained_locked(
    repository_tree: &RetainedRepositoryTreeGuardV1<'_>,
) -> Result<RetainedApproverRegistryObservationV1, ApproverRegistryObservationErrorV1> {
    repository_tree
        .with_stable_path(observe_committed_approver_registry_locked)
        .map_err(|_| lineage_error("retained approver authority changed before recheck"))?
}

pub(crate) fn observe_committed_approver_registry_if_present_locked(
    repo_root: &Path,
) -> Result<Option<RetainedApproverRegistryObservationV1>, ApproverRegistryObservationErrorV1> {
    observe_committed_registry_chain_head_if_present_locked(repo_root)
        .map_err(|error| durability_with_cause(error.detail(), error.causal_disposition()))?
        .map(|head| resolve_retained_registry_head(repo_root, head))
        .transpose()
}

pub(crate) fn observe_committed_approver_registry_if_present_retained_locked(
    repository_tree: &RetainedRepositoryTreeGuardV1<'_>,
) -> Result<Option<RetainedApproverRegistryObservationV1>, ApproverRegistryObservationErrorV1> {
    repository_tree
        .with_stable_path(observe_committed_approver_registry_if_present_locked)
        .map_err(|_| {
            unsafe_error(
                "retained repository path changed during approver observation",
                OwnerCausalDispositionV1::ImplementationFailure,
            )
        })?
}

pub(crate) fn observe_committed_approver_registry_for_repository_preflight_locked(
    repo_root: &Path,
) -> Result<Option<RetainedApproverRegistryObservationV1>, ApproverRegistryObservationErrorV1> {
    observe_committed_registry_chain_head_for_repository_preflight_locked(repo_root)
        .map_err(|error| durability_with_cause(error.detail(), error.causal_disposition()))?
        .map(|head| resolve_retained_registry_head(repo_root, head))
        .transpose()
}

fn resolve_retained_registry_head(
    repo_root: &Path,
    head: CommittedRegistryChainHeadV1,
) -> Result<RetainedApproverRegistryObservationV1, ApproverRegistryObservationErrorV1> {
    let lineage = TrustedLineageStoreV1::new(repo_root);
    let state_bytes = lineage
        .read_record(
            LineageRecordClassV1::RegistryState,
            &head.state_ref,
            &head.state_fingerprint,
        )
        .map_err(|failure| {
            lineage_error_with_cause(
                "retained registry state could not be re-resolved",
                failure.causal_disposition(),
            )
        })?;
    let state = parse_schema_json(&state_bytes)
        .map_err(|_| lineage_error("retained registry state is not closed JSON"))?;
    let head_transition_bytes = lineage
        .read_record(
            LineageRecordClassV1::RegistryTransition,
            &head.transition_ref,
            &head.transition_fingerprint,
        )
        .map_err(|failure| {
            lineage_error_with_cause(
                "retained registry head could not be re-resolved",
                failure.causal_disposition(),
            )
        })?;
    let transition = parse_schema_json(&head_transition_bytes)
        .map_err(|_| lineage_error("retained registry head is not closed JSON"))?;
    if state
        .get("registry_state_fingerprint")
        .and_then(Value::as_str)
        != Some(head.state_fingerprint.as_str())
        || transition
            .get("transition_fingerprint")
            .and_then(Value::as_str)
            != Some(head.transition_fingerprint.as_str())
        || transition
            .get("result_registry_state_ref")
            .and_then(Value::as_str)
            != Some(head.state_ref.as_str())
        || transition
            .get("result_registry_state_fingerprint")
            .and_then(Value::as_str)
            != Some(head.state_fingerprint.as_str())
        || state.get("repository_identity_fingerprint")
            != transition.get("repository_identity_fingerprint")
    {
        return Err(lineage_error(
            "retained registry state/head exact bindings disagree",
        ));
    }
    let credentials = resolve_credentials(repo_root, &lineage, &state)?;
    if credentials.is_empty() {
        return Err(lineage_error(
            "retained registry authority has no credential material",
        ));
    }
    Ok(RetainedApproverRegistryObservationV1 {
        state_ref: head.state_ref,
        state_fingerprint: head.state_fingerprint,
        state_bytes,
        state,
        head_transition_ref: head.transition_ref,
        head_transition_fingerprint: head.transition_fingerprint,
        head_transition_bytes,
        credentials,
    })
}

fn resolve_credentials(
    repo_root: &Path,
    lineage: &TrustedLineageStoreV1,
    state: &Value,
) -> Result<Vec<CommittedApproverCredentialV1>, ApproverRegistryObservationErrorV1> {
    let rows = state
        .get("credentials")
        .and_then(Value::as_array)
        .ok_or_else(|| lineage_error("registry credentials are absent"))?;
    if rows.is_empty() || rows.len() > 256 {
        return Err(lineage_error(
            "registry credential count is outside the admitted bound",
        ));
    }
    let mut seen = BTreeSet::new();
    let mut credentials = Vec::with_capacity(rows.len());
    for row in rows {
        let credential_id_hash = string(row, "credential_id_hash")?.to_owned();
        if !seen.insert(credential_id_hash.clone()) {
            return Err(lineage_error("registry contains duplicate credential IDs"));
        }
        let registration_ref = string(row, "registration_ref")?.to_owned();
        let registration_fingerprint = string(row, "registration_fingerprint")?.to_owned();
        let registration_bytes = lineage
            .read_record(
                LineageRecordClassV1::AuthenticatorRegistration,
                &registration_ref,
                &registration_fingerprint,
            )
            .map_err(|failure| {
                lineage_error_with_cause(
                    "credential registration could not be re-resolved",
                    failure.causal_disposition(),
                )
            })?;
        let registration = parse_schema_json(&registration_bytes)
            .map_err(|_| lineage_error("credential registration is not closed JSON"))?;
        let response_ref = string(&registration, "decoded_response_ref")?;
        let response_fingerprint = string(&registration, "decoded_response_fingerprint")?;
        let response_bytes = lineage
            .read_record(
                LineageRecordClassV1::AuthenticatorMakeCredentialResponse,
                response_ref,
                response_fingerprint,
            )
            .map_err(|failure| {
                lineage_error_with_cause(
                    "make-credential response could not be re-resolved",
                    failure.causal_disposition(),
                )
            })?;
        let response = parse_schema_json(&response_bytes)
            .map_err(|_| lineage_error("make-credential response is not closed JSON"))?;
        let credential_id = decode_bounded_base64(
            string(&response, "credential_id_base64")?,
            1_024,
            "credential ID",
        )?;
        if DefinitionFingerprint::from_bytes(&credential_id).as_str() != credential_id_hash
            || string(&response, "credential_id_hash")? != credential_id_hash
        {
            return Err(lineage_error(
                "credential ID bytes and retained hash disagree",
            ));
        }
        let cose_public_key = decode_bounded_base64(
            string(row, "cose_public_key_base64")?,
            4_096,
            "credential COSE public key",
        )?;
        if string(&response, "cose_public_key_base64")? != string(row, "cose_public_key_base64")? {
            return Err(lineage_error(
                "registry and registration COSE public keys disagree",
            ));
        }
        let approval_mappings = mappings(row)?;
        let (use_head_ref, use_head_fingerprint, use_sequence, sign_count) =
            read_use_head(repo_root, &credential_id_hash)?;
        credentials.push(CommittedApproverCredentialV1 {
            credential_id_hash,
            credential_id,
            cose_public_key,
            active: string(row, "status")? == "active",
            approval_mappings,
            registration_ref,
            registration_fingerprint,
            use_head_ref,
            use_head_fingerprint,
            use_sequence,
            sign_count,
        });
    }
    credentials.sort_by(|left, right| left.credential_id_hash.cmp(&right.credential_id_hash));
    Ok(credentials)
}

fn read_use_head(
    repo_root: &Path,
    credential_id_hash: &str,
) -> Result<(String, String, u16, u32), ApproverRegistryObservationErrorV1> {
    let hex = credential_id_hash
        .strip_prefix("sha256:")
        .ok_or_else(|| lineage_error("credential ID hash is not normalized"))?;
    let relative_ref = format!("authenticator-use-heads/{hex}.json");
    let path = repo_root.join(".handbook/state").join(&relative_ref);
    let bytes = read_registry_journal_file(repo_root, &path, 64 * 1024, "authenticator use head")
        .map_err(|error| {
        unsafe_error(
            "authenticator use head is not a safe bounded file",
            error.causal_disposition(),
        )
    })?;
    let mut value = parse_schema_json(&bytes)
        .map_err(|_| lineage_error("authenticator use head is not closed JSON"))?;
    let canonical = serde_json_canonicalizer::to_vec(&value)
        .map_err(|_| lineage_error("authenticator use head cannot be canonicalized"))?;
    if canonical != bytes {
        return Err(lineage_error("authenticator use head is not exact JCS"));
    }
    let keys = value
        .as_object()
        .ok_or_else(|| lineage_error("authenticator use head is not an object"))?
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let expected = BTreeSet::from([
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
    ]);
    if keys != expected
        || string(&value, "$schema")? != "handbook.schemas.security.authenticator-use-head@1.0.0"
        || string(&value, "schema_version")? != "1.0"
        || string(&value, "credential_id_hash")? != credential_id_hash
    {
        return Err(lineage_error(
            "authenticator use head violates its exact closed shape",
        ));
    }
    let fingerprint = string(&value, "head_fingerprint")?.to_owned();
    value
        .as_object_mut()
        .expect("validated object")
        .remove("head_fingerprint");
    let computed = DefinitionFingerprint::from_json_value(&value)
        .map_err(|_| lineage_error("authenticator use head fingerprint failed"))?
        .to_string();
    if computed != fingerprint {
        return Err(lineage_error(
            "authenticator use head fingerprint does not recompute",
        ));
    }
    let sequence = value
        .get("sequence")
        .and_then(Value::as_u64)
        .and_then(|number| u16::try_from(number).ok())
        .filter(|number| *number <= 4_096)
        .ok_or_else(|| lineage_error("authenticator use sequence is outside 0..4096"))?;
    let sign_count = value
        .get("sign_count")
        .and_then(Value::as_u64)
        .and_then(|number| u32::try_from(number).ok())
        .ok_or_else(|| lineage_error("authenticator sign count is invalid"))?;
    if sequence == 0
        && [
            "last_assertion_fingerprint",
            "last_assertion_ref",
            "last_challenge_fingerprint",
            "last_nonce_sha256",
        ]
        .iter()
        .any(|field| value.get(*field) != Some(&Value::Null))
    {
        return Err(lineage_error(
            "sequence-zero authenticator use head has non-null retained use identity",
        ));
    }
    Ok((relative_ref, fingerprint, sequence, sign_count))
}

fn mappings(
    credential: &Value,
) -> Result<Vec<ApproverAuthorityPairV1>, ApproverRegistryObservationErrorV1> {
    let rows = credential
        .get("approval_mappings")
        .and_then(Value::as_array)
        .ok_or_else(|| lineage_error("credential approval mappings are absent"))?;
    if rows.is_empty() || rows.len() > 64 {
        return Err(lineage_error(
            "credential approval mapping count is outside the admitted bound",
        ));
    }
    let mut pairs = rows
        .iter()
        .map(|row| {
            Ok(ApproverAuthorityPairV1 {
                approval_class: string(row, "approval_class")?.to_owned(),
                authority_ref: string(row, "authority_ref")?.to_owned(),
            })
        })
        .collect::<Result<Vec<_>, ApproverRegistryObservationErrorV1>>()?;
    let retained = pairs.clone();
    pairs.sort_by(|left, right| {
        left.approval_class
            .cmp(&right.approval_class)
            .then_with(|| left.authority_ref.cmp(&right.authority_ref))
    });
    pairs.dedup();
    if pairs != retained {
        return Err(lineage_error(
            "credential approval mappings are unsorted or duplicated",
        ));
    }
    Ok(pairs)
}

fn decode_bounded_base64(
    text: &str,
    maximum_bytes: usize,
    label: &str,
) -> Result<Vec<u8>, ApproverRegistryObservationErrorV1> {
    let maximum_text = maximum_bytes.saturating_add(2) / 3 * 4;
    if text.is_empty() || text.len() > maximum_text {
        return Err(lineage_error(format!(
            "{label} base64 is outside its admitted bound"
        )));
    }
    let bytes = BASE64_STANDARD
        .decode(text)
        .map_err(|_| lineage_error(format!("{label} base64 is invalid")))?;
    if bytes.is_empty() || bytes.len() > maximum_bytes {
        return Err(lineage_error(format!(
            "{label} decoded bytes are outside their admitted bound"
        )));
    }
    Ok(bytes)
}

fn string<'a>(
    value: &'a Value,
    field: &str,
) -> Result<&'a str, ApproverRegistryObservationErrorV1> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|text| !text.is_empty())
        .ok_or_else(|| lineage_error(format!("required registry field {field} is absent")))
}

fn durability_with_cause(
    detail: impl Into<String>,
    causal_disposition: OwnerCausalDispositionV1,
) -> ApproverRegistryObservationErrorV1 {
    ApproverRegistryObservationErrorV1 {
        kind: ApproverRegistryObservationErrorKindV1::DurabilityViolation,
        detail: detail.into(),
        causal_disposition,
    }
}

fn unsafe_error(
    detail: impl Into<String>,
    causal_disposition: OwnerCausalDispositionV1,
) -> ApproverRegistryObservationErrorV1 {
    ApproverRegistryObservationErrorV1 {
        kind: ApproverRegistryObservationErrorKindV1::UnsafeFilesystem,
        detail: detail.into(),
        causal_disposition,
    }
}

fn lineage_error(detail: impl Into<String>) -> ApproverRegistryObservationErrorV1 {
    lineage_error_with_cause(detail, OwnerCausalDispositionV1::CallerRefusal)
}

fn lineage_error_with_cause(
    detail: impl Into<String>,
    causal_disposition: OwnerCausalDispositionV1,
) -> ApproverRegistryObservationErrorV1 {
    ApproverRegistryObservationErrorV1 {
        kind: ApproverRegistryObservationErrorKindV1::LineageViolation,
        detail: detail.into(),
        causal_disposition,
    }
}

#[cfg(test)]
mod optional_retained_registry_tests {
    use super::*;
    use std::fs;

    #[test]
    fn optional_locked_registry_observation_distinguishes_absence_from_unsafe_state() {
        let repo = tempfile::tempdir().unwrap();
        assert!(
            observe_committed_approver_registry_if_present_locked(repo.path())
                .unwrap()
                .is_none()
        );

        let transactions = repo.path().join(".handbook/state/transactions");
        fs::create_dir_all(&transactions).unwrap();
        fs::write(transactions.join("registry"), b"not a directory").unwrap();
        let failure = match observe_committed_approver_registry_if_present_locked(repo.path()) {
            Err(failure) => failure,
            Ok(_) => panic!("unsafe registry state must not collapse to bootstrap absence"),
        };
        assert_ne!(
            failure.kind(),
            ApproverRegistryObservationErrorKindV1::AuthorityAbsent
        );
    }

    #[test]
    fn unsafe_observation_preserves_caller_and_io_causes() {
        let caller = unsafe_error(
            "unsafe repository state",
            OwnerCausalDispositionV1::CallerRefusal,
        );
        let io = unsafe_error(
            "repository inspection failed",
            OwnerCausalDispositionV1::ImplementationFailure,
        );
        assert_eq!(
            caller.causal_disposition(),
            OwnerCausalDispositionV1::CallerRefusal
        );
        assert_eq!(
            io.causal_disposition(),
            OwnerCausalDispositionV1::ImplementationFailure
        );

        let lineage_conflict = lineage_error_with_cause(
            "retained lineage conflict",
            OwnerCausalDispositionV1::CallerRefusal,
        );
        let lineage_io = lineage_error_with_cause(
            "retained lineage read failed",
            OwnerCausalDispositionV1::ImplementationFailure,
        );
        assert_eq!(
            lineage_conflict.causal_disposition(),
            OwnerCausalDispositionV1::CallerRefusal
        );
        assert_eq!(
            lineage_io.causal_disposition(),
            OwnerCausalDispositionV1::ImplementationFailure
        );
    }
}
