#![allow(
    clippy::result_large_err,
    reason = "the exact reviewed refusal DTO layout is serialized across the approval boundary"
)]

use crate::approver_registry_observation::{
    observe_committed_approver_registry_locked, ApproverAuthorityPairV1,
    ApproverRegistryObservationErrorKindV1, ApproverRegistryObservationErrorV1,
    CommittedApproverCredentialV1, RetainedApproverRegistryObservationV1,
};
use crate::canonical_repo_support::{CanonicalWorkspace, RepoRelativeFileAccessError};
use crate::charter_artifact::{parse_canonical_charter, CanonicalCharter};
use crate::charter_authenticator::{
    decode_and_verify_get_assertion_response, encode_get_assertion_request,
    select_eligible_credentials, AuthenticatorCredentialV1, AuthenticatorErrorV1,
    CredentialSelectionCandidateV1, CtapRefusalCodeV1, GetAssertionResponseV1,
    NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1, AUTHENTICATOR_RP_ID,
};
use crate::charter_authority_transaction::CharterAuthorityTransactionServiceV1;
use crate::charter_definition_registry::load_shipped_charter_definition_registry;
use crate::charter_lifecycle_store::CharterLifecycleStoreV1;
use crate::charter_lifecycle_validation::{
    validate_candidate_exact_result_authority, CharterLifecycleValidationServiceV1,
};
use crate::charter_lineage_store::{
    create_new_file, create_safe_directories, reject_reparse_or_symlink, sync_directory,
    validate_record, LineageRecordClassV1, LineageStoreErrorV1, TrustedLineageStoreV1,
};
use crate::{parse_schema_json, DefinitionFingerprint, ResolvedProfileDecisions};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_OPERATION_ID_BYTES: usize = 512;
const MAX_SELECTOR_BYTES: usize = 256;
const MAX_REFERENCE_BYTES: usize = 512;
const MAX_ACCEPTED_WAIVERS: usize = 10;
const MAX_RECORD_BYTES: usize = 1024 * 1024;
const MAX_CANONICAL_BYTES: usize = 8 * 1024 * 1024;
const APPROVAL_POLICY_REF: &str = "handbook.approval.constitutional-candidate@1.0.0";
const CANONICAL_CHARTER_REF: &str = ".handbook/project/charter.yaml";
const APPROVAL_TRANSACTION_ROOT: &str = ".handbook/state/transactions/approvals";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterApprovalRequestV1 {
    pub operation_id: String,
    pub candidate_ref: String,
    pub approval_class: String,
    pub authority_ref: String,
    pub accepted_waiver_refs: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharterApprovalRefusalCodeV1 {
    InvalidRequest,
    AuthorityAbsent,
    StaleBasis,
    AuthorizationRefused,
    LineageViolation,
    TransactionConflict,
    AuthenticatorUserTimeout,
    AuthenticatorResourceExhausted,
    AuthenticatorSecurityBlocked,
    AuthenticatorUserCancelled,
    AuthenticatorUserVerificationRetry,
    AuthenticatorUnknownError,
    AuthenticatorUnavailable,
    DurabilityViolation,
}

impl CharterApprovalRefusalCodeV1 {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidRequest => "invalid_request",
            Self::AuthorityAbsent => "authority_absent",
            Self::StaleBasis => "stale_basis",
            Self::AuthorizationRefused => "authorization_refused",
            Self::LineageViolation => "lineage_violation",
            Self::TransactionConflict => "transaction_conflict",
            Self::AuthenticatorUserTimeout => "authenticator_user_timeout",
            Self::AuthenticatorResourceExhausted => "authenticator_resource_exhausted",
            Self::AuthenticatorSecurityBlocked => "authenticator_security_blocked",
            Self::AuthenticatorUserCancelled => "authenticator_user_cancelled",
            Self::AuthenticatorUserVerificationRetry => "authenticator_user_verification_retry",
            Self::AuthenticatorUnknownError => "authenticator_unknown_error",
            Self::AuthenticatorUnavailable => "AUTHENTICATOR_UNAVAILABLE",
            Self::DurabilityViolation => "durability_violation",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterApprovalRefusalV1 {
    pub operation_id: String,
    pub code: CharterApprovalRefusalCodeV1,
    pub message: String,
    pub retryable: bool,
    pub next_actions: Vec<String>,
    pub response_ref: Option<String>,
    pub response_fingerprint: Option<String>,
    pub assertion_ref: Option<String>,
    pub assertion_fingerprint: Option<String>,
    pub use_transition_ref: Option<String>,
    pub use_transition_fingerprint: Option<String>,
    pub approval_ref: Option<String>,
    pub approval_fingerprint: Option<String>,
    pub result_use_head_ref: Option<String>,
    pub result_use_head_fingerprint: Option<String>,
    pub changed_paths: Vec<String>,
}

impl CharterApprovalRefusalV1 {
    pub fn semantic_identities_are_null(&self) -> bool {
        self.response_ref.is_none()
            && self.response_fingerprint.is_none()
            && self.assertion_ref.is_none()
            && self.assertion_fingerprint.is_none()
            && self.use_transition_ref.is_none()
            && self.use_transition_fingerprint.is_none()
            && self.approval_ref.is_none()
            && self.approval_fingerprint.is_none()
            && self.result_use_head_ref.is_none()
            && self.result_use_head_fingerprint.is_none()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterApprovalSuccessV1 {
    pub operation_id: String,
    pub response_ref: String,
    pub response_fingerprint: String,
    pub assertion_ref: String,
    pub assertion_fingerprint: String,
    pub use_transition_ref: String,
    pub use_transition_fingerprint: String,
    pub approval_ref: String,
    pub approval_fingerprint: String,
    pub result_use_head_ref: String,
    pub result_use_head_fingerprint: String,
    pub changed_paths: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CharterApprovalResultV1 {
    Succeeded(CharterApprovalSuccessV1),
    Refused(CharterApprovalRefusalV1),
}

pub struct CharterApprovalServiceV1<P> {
    repo_root: PathBuf,
    port: P,
}

impl<P: NativeAuthenticatorPortV1> CharterApprovalServiceV1<P> {
    pub fn new(repo_root: impl AsRef<Path>, port: P) -> Self {
        Self {
            repo_root: repo_root.as_ref().to_path_buf(),
            port,
        }
    }

    pub fn into_port(self) -> P {
        self.port
    }

    pub fn approve_candidate(
        &mut self,
        decisions: &ResolvedProfileDecisions,
        request: CharterApprovalRequestV1,
    ) -> CharterApprovalResultV1 {
        let operation_id = request.operation_id.clone();
        if let Err(refusal) = validate_caller_intent(&request) {
            return CharterApprovalResultV1::Refused(refusal);
        }
        match self.approve_inner(decisions, &request) {
            Ok(success) => CharterApprovalResultV1::Succeeded(success),
            Err(mut refusal) => {
                refusal.operation_id = operation_id;
                CharterApprovalResultV1::Refused(refusal)
            }
        }
    }

    fn approve_inner(
        &mut self,
        decisions: &ResolvedProfileDecisions,
        request: &CharterApprovalRequestV1,
    ) -> Result<CharterApprovalSuccessV1, CharterApprovalRefusalV1> {
        load_shipped_charter_definition_registry()
            .and_then(|registry| registry.validate_selected_decisions(decisions))
            .map_err(|_| {
                refused(
                    CharterApprovalRefusalCodeV1::LineageViolation,
                    "selected Charter definitions are not the exact shipped closure",
                    false,
                    "restore the exact shipped Charter definition closure",
                )
            })?;

        let lineage = TrustedLineageStoreV1::new(&self.repo_root);
        let candidate_fingerprint = fingerprint_from_ref(&request.candidate_ref, "candidate")?;
        let candidate_bytes = lineage
            .read_record(
                LineageRecordClassV1::Candidate,
                &request.candidate_ref,
                &candidate_fingerprint,
            )
            .map_err(map_lineage_refusal)?;
        let candidate = parse_schema_json(&candidate_bytes).map_err(|_| {
            refused(
                CharterApprovalRefusalCodeV1::LineageViolation,
                "candidate is not exact closed JSON",
                false,
                "recreate the candidate through Charter intake",
            )
        })?;
        validate_candidate_currentness(decisions, &candidate)?;
        validate_candidate_exact_result_authority(&self.repo_root, &candidate).map_err(
            |failure| {
                refused(
                    CharterApprovalRefusalCodeV1::LineageViolation,
                    format!(
                        "candidate exact lifecycle-validation authority refused: {}",
                        failure.detail()
                    ),
                    false,
                    "preserve the candidate/result evidence and reauthor candidate 1.3",
                )
            },
        )?;
        let canonical = observe_candidate_basis(&self.repo_root, decisions, &candidate)?;

        // Retain promotion, registry, and lifecycle authority through the native
        // assertion and durable approval marker. This also makes candidate-result
        // semantic currentness one atomic observation domain.
        let transaction = CharterAuthorityTransactionServiceV1::new(&self.repo_root);
        let retained = transaction.begin_retained_authority().map_err(|failure| {
            refused(
                CharterApprovalRefusalCodeV1::TransactionConflict,
                format!(
                    "complete retained authority recovery refused: {}",
                    failure.detail()
                ),
                false,
                "preserve the retained journals and repair the complete authority domain",
            )
        })?;
        let authority = observe_committed_approver_registry_locked(&self.repo_root)
            .map_err(map_observation_refusal)?;
        recover_approval_authority(&self.repo_root)?;

        let normalized_content_ref =
            candidate["normalized_content_ref"]
                .as_str()
                .ok_or_else(|| {
                    refused(
                        CharterApprovalRefusalCodeV1::LineageViolation,
                        "candidate normalized-content reference is not a string",
                        false,
                        "recreate the candidate through Charter intake",
                    )
                })?;
        let normalized_content = lineage
            .read_candidate_content(normalized_content_ref)
            .map_err(map_lineage_refusal)?;
        let current = retained.read_committed_charter().map_err(|failure| {
            refused(
                CharterApprovalRefusalCodeV1::TransactionConflict,
                format!(
                    "committed Charter authority observation refused: {}",
                    failure.detail()
                ),
                false,
                "repair retained promotion authority before approval",
            )
        })?;
        let lifecycle = CharterLifecycleStoreV1::new(&self.repo_root)
            .observe_retained_locked()
            .map_err(|failure| {
                refused(
                    CharterApprovalRefusalCodeV1::LineageViolation,
                    format!(
                        "retained lifecycle authority observation refused: {}",
                        failure.detail()
                    ),
                    false,
                    "repair retained lifecycle authority before approval",
                )
            })?;
        CharterLifecycleValidationServiceV1::new(&self.repo_root)
            .validate_current_retained(
                decisions,
                &candidate,
                &normalized_content,
                current.as_ref(),
                &lifecycle,
            )
            .map_err(|failure| {
                refused(
                    CharterApprovalRefusalCodeV1::LineageViolation,
                    format!(
                        "candidate semantic lifecycle-validation authority refused: {}",
                        failure.detail()
                    ),
                    false,
                    "preserve the evidence and reauthor candidate 1.3 against current authority",
                )
            })?;
        let required_pairs = required_approval_pairs(&authority, canonical.record.as_ref())?;
        let requested_pair = ApproverAuthorityPairV1 {
            approval_class: request.approval_class.clone(),
            authority_ref: request.authority_ref.clone(),
        };
        if !required_pairs.contains(&requested_pair) {
            return Err(refused(
                CharterApprovalRefusalCodeV1::AuthorizationRefused,
                "requested approval class and authority are not currently required",
                false,
                "select one exact currently required approval pair",
            ));
        }
        if committed_pair_already_satisfied(
            &self.repo_root,
            &request.candidate_ref,
            &candidate_fingerprint,
            &requested_pair,
        )? {
            return Err(refused(
                CharterApprovalRefusalCodeV1::AuthorizationRefused,
                "the exact candidate authority pair is already satisfied",
                false,
                "retain the committed approval or choose another required pair",
            ));
        }

        let accepted_waivers = resolve_accepted_waivers(
            &lineage,
            &request.accepted_waiver_refs,
            &candidate_fingerprint,
        )?;
        let credentials = eligible_credentials(&authority, &requested_pair, &required_pairs)?;
        if credentials.is_empty() {
            return Err(refused(
                CharterApprovalRefusalCodeV1::AuthorizationRefused,
                "no committed active unexhausted credential covers the exact authority pair",
                false,
                "register or map one usable credential to the required approval pair",
            ));
        }

        let prior_use_heads = credentials
            .iter()
            .map(|credential| {
                Ok((
                    credential.credential_id_hash.clone(),
                    read_retained_use_head(&self.repo_root, credential)?,
                ))
            })
            .collect::<Result<BTreeMap<_, _>, CharterApprovalRefusalV1>>()?;
        let mut nonce = [0u8; 32];
        getrandom::fill(&mut nonce).map_err(|_| {
            refused(
                CharterApprovalRefusalCodeV1::TransactionConflict,
                "operating-system randomness was unavailable",
                true,
                "retry the complete operation with fresh operating-system randomness",
            )
        })?;
        let challenge = build_approval_challenge(
            request,
            &candidate_fingerprint,
            candidate
                .get("basis_artifact_fingerprint")
                .unwrap_or(&Value::Null),
            &authority,
            &accepted_waivers,
            nonce,
        )?;
        let client_data_hash: [u8; 32] = Sha256::digest(&challenge.bytes).into();
        let allow_ids = credentials
            .iter()
            .map(|credential| credential.credential_id.clone())
            .collect::<Vec<_>>();
        let request_cbor = encode_get_assertion_request(client_data_hash, &allow_ids)
            .map_err(map_authenticator_refusal)?;
        let raw_response = self
            .port
            .get_assertion(&request_cbor)
            .map_err(map_port_refusal)?;
        let requested_credentials = credentials
            .iter()
            .map(|credential| AuthenticatorCredentialV1 {
                credential_id: credential.credential_id.clone(),
                cose_public_key: credential.cose_public_key.clone(),
            })
            .collect::<Vec<_>>();
        let decoded = decode_and_verify_get_assertion_response(
            &raw_response,
            &requested_credentials,
            client_data_hash,
        )
        .map_err(map_authenticator_refusal)?;
        let credential = credentials
            .iter()
            .find(|credential| credential.credential_id == decoded.credential_id)
            .ok_or_else(|| {
                refused(
                    CharterApprovalRefusalCodeV1::AuthorizationRefused,
                    "assertion selected no retained eligible credential",
                    false,
                    "perform a new assertion with an eligible credential",
                )
            })?;
        let prior_use_head_bytes = prior_use_heads
            .get(&credential.credential_id_hash)
            .cloned()
            .ok_or_else(|| {
                refused(
                    CharterApprovalRefusalCodeV1::LineageViolation,
                    "selected credential has no retained use-head observation",
                    false,
                    "repair exact append-only Charter lineage before retry",
                )
            })?;
        validate_counter(credential.sign_count, decoded.sign_count)?;
        validate_use_chain(
            &self.repo_root,
            credential,
            &challenge.fingerprint,
            &challenge.nonce_fingerprint,
        )?;

        ensure_retained_authority_stable(
            &self.repo_root,
            &lineage,
            &authority,
            credential,
            &candidate_bytes,
            &request.candidate_ref,
            &candidate_fingerprint,
            &canonical,
            &prior_use_head_bytes,
        )?;
        let records = build_approval_records(
            request,
            &candidate,
            &candidate_fingerprint,
            &authority,
            credential,
            &accepted_waivers,
            &challenge,
            &decoded,
            client_data_hash,
            prior_use_head_bytes,
        )?;
        let success = commit_approval_transaction(
            &self.repo_root,
            &lineage,
            &authority,
            credential,
            &candidate_bytes,
            &canonical,
            records,
        )?;
        drop(retained);
        Ok(success)
    }
}

struct CanonicalBasisObservation {
    bytes: Option<Vec<u8>>,
    _fingerprint: Option<String>,
    record: Option<CanonicalCharter>,
}

#[derive(Clone)]
struct AcceptedWaiver {
    waiver_ref: String,
    waiver_fingerprint: String,
}

struct ApprovalChallenge {
    bytes: Vec<u8>,
    fingerprint: String,
    nonce_fingerprint: String,
}

#[derive(Clone)]
struct BuiltRecord {
    relative_ref: String,
    fingerprint: String,
    bytes: Vec<u8>,
}

struct ApprovalRecords {
    operation_id: String,
    response: BuiltRecord,
    assertion: BuiltRecord,
    use_transition: BuiltRecord,
    approval: BuiltRecord,
    prior_use_head_ref: String,
    prior_use_head_fingerprint: String,
    prior_use_head_bytes: Vec<u8>,
    result_use_head: BuiltRecord,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ApprovalIntentV1 {
    schema_id: String,
    schema_version: String,
    transaction_id: String,
    operation_id: String,
    candidate_ref: String,
    candidate_fingerprint: String,
    approval_class: String,
    authority_ref: String,
    approver_registry_state_ref: String,
    approver_registry_state_fingerprint: String,
    registry_head_transition_ref: String,
    registry_head_transition_fingerprint: String,
    credential_id_hash: String,
    response_ref: String,
    response_fingerprint: String,
    response_bytes_fingerprint: String,
    assertion_ref: String,
    assertion_fingerprint: String,
    assertion_bytes_fingerprint: String,
    use_transition_ref: String,
    use_transition_fingerprint: String,
    use_transition_bytes_fingerprint: String,
    approval_ref: String,
    approval_fingerprint: String,
    approval_bytes_fingerprint: String,
    prior_use_head_ref: String,
    prior_use_head_fingerprint: String,
    prior_use_sequence: u64,
    prior_use_head_bytes_fingerprint: String,
    result_use_head_ref: String,
    result_use_head_fingerprint: String,
    result_use_sequence: u64,
    result_use_head_bytes_fingerprint: String,
}

impl ApprovalIntentV1 {
    fn from_records(
        authority: &RetainedApproverRegistryObservationV1,
        credential: &CommittedApproverCredentialV1,
        records: &ApprovalRecords,
    ) -> Result<Self, CharterApprovalRefusalV1> {
        let approval: Value = serde_json::from_slice(&records.approval.bytes)
            .map_err(|_| lineage("built approval record is not JSON"))?;
        let transaction_id = records
            .approval
            .relative_ref
            .strip_prefix("approvals/")
            .and_then(|value| value.strip_suffix(".json"))
            .ok_or_else(|| lineage("built approval ref cannot identify its transaction"))?
            .to_owned();
        let prior_use_head: Value = serde_json::from_slice(&records.prior_use_head_bytes)
            .map_err(|_| lineage("prior use-head is not JSON while building approval intent"))?;
        let prior_use_sequence = prior_use_head
            .get("sequence")
            .and_then(Value::as_u64)
            .ok_or_else(|| lineage("prior use-head sequence is absent"))?;
        let result_use_head: Value = serde_json::from_slice(&records.result_use_head.bytes)
            .map_err(|_| lineage("result use-head is not JSON while building approval intent"))?;
        let result_use_sequence = result_use_head
            .get("sequence")
            .and_then(Value::as_u64)
            .ok_or_else(|| lineage("result use-head sequence is absent"))?;
        Ok(Self {
            schema_id: "handbook.approval-transaction-intent".to_owned(),
            schema_version: "1.0".to_owned(),
            transaction_id,
            operation_id: records.operation_id.clone(),
            candidate_ref: required_string(&approval, "candidate_ref")?.to_owned(),
            candidate_fingerprint: required_string(&approval, "candidate_fingerprint")?.to_owned(),
            approval_class: required_string(&approval, "approval_class")?.to_owned(),
            authority_ref: required_string(&approval, "authority_ref")?.to_owned(),
            approver_registry_state_ref: authority.state_ref.clone(),
            approver_registry_state_fingerprint: authority.state_fingerprint.clone(),
            registry_head_transition_ref: authority.head_transition_ref.clone(),
            registry_head_transition_fingerprint: authority.head_transition_fingerprint.clone(),
            credential_id_hash: credential.credential_id_hash.clone(),
            response_ref: records.response.relative_ref.clone(),
            response_fingerprint: records.response.fingerprint.clone(),
            response_bytes_fingerprint: DefinitionFingerprint::from_bytes(&records.response.bytes)
                .to_string(),
            assertion_ref: records.assertion.relative_ref.clone(),
            assertion_fingerprint: records.assertion.fingerprint.clone(),
            assertion_bytes_fingerprint: DefinitionFingerprint::from_bytes(
                &records.assertion.bytes,
            )
            .to_string(),
            use_transition_ref: records.use_transition.relative_ref.clone(),
            use_transition_fingerprint: records.use_transition.fingerprint.clone(),
            use_transition_bytes_fingerprint: DefinitionFingerprint::from_bytes(
                &records.use_transition.bytes,
            )
            .to_string(),
            approval_ref: records.approval.relative_ref.clone(),
            approval_fingerprint: records.approval.fingerprint.clone(),
            approval_bytes_fingerprint: DefinitionFingerprint::from_bytes(&records.approval.bytes)
                .to_string(),
            prior_use_head_ref: records.prior_use_head_ref.clone(),
            prior_use_head_fingerprint: records.prior_use_head_fingerprint.clone(),
            prior_use_sequence,
            prior_use_head_bytes_fingerprint: DefinitionFingerprint::from_bytes(
                &records.prior_use_head_bytes,
            )
            .to_string(),
            result_use_head_ref: records.result_use_head.relative_ref.clone(),
            result_use_head_fingerprint: records.result_use_head.fingerprint.clone(),
            result_use_sequence,
            result_use_head_bytes_fingerprint: DefinitionFingerprint::from_bytes(
                &records.result_use_head.bytes,
            )
            .to_string(),
        })
    }
}

pub(crate) fn recover_approval_authority(repo_root: &Path) -> Result<(), CharterApprovalRefusalV1> {
    let root = repo_root.join(APPROVAL_TRANSACTION_ROOT);
    if !root.exists() {
        return Ok(());
    }
    reject_reparse_or_symlink(&root, true).map_err(map_lineage_refusal)?;
    let mut pending = safe_transaction_directories(&root, ".pending")?;
    pending.sort();
    for directory in pending {
        if !directory.join("intent.json").exists() {
            rollback_owned_pre_intent_journal(repo_root, &root, &directory)?;
            continue;
        }
        let intent_bytes = read_safe_file(repo_root, &directory.join("intent.json"), 64 * 1024)?;
        let intent = parse_exact_intent(&intent_bytes)?;
        let marker_path = directory.join("committed");
        if APPROVAL_STAGE_NAMES
            .iter()
            .any(|name| !directory.join(name).exists())
        {
            if marker_path.exists() {
                return Err(durability(
                    "committed approval journal is missing operation-required staged bytes",
                ));
            }
            rollback_incomplete_staged_journal(repo_root, &root, &directory, &intent)?;
            continue;
        }
        let stages = read_staged_records(repo_root, &directory)?;
        validate_stages_against_intent(&intent, &stages)?;
        install_staged_approval(repo_root, &intent, &stages)?;
        let marker = approval_commit_marker(&intent_bytes);
        if marker_path.exists() {
            if read_safe_file(repo_root, &marker_path, 72)? != marker {
                return Err(durability("pending approval commit marker is not exact"));
            }
        } else {
            write_new_durable(&directory.join("committed.tmp"), &marker)?;
            fs::rename(directory.join("committed.tmp"), &marker_path)
                .map_err(|_| durability("approval recovery marker rename failed"))?;
            sync_directory(&directory).map_err(map_lineage_refusal)?;
        }
        finalize_pending_directory(&root, &directory)?;
    }
    Ok(())
}

pub(crate) fn recover_approval_pre_intent_authority_locked(
    repo_root: &Path,
) -> Result<(), CharterApprovalRefusalV1> {
    let root = repo_root.join(APPROVAL_TRANSACTION_ROOT);
    if !root.exists() {
        return Ok(());
    }
    reject_reparse_or_symlink(&root, true).map_err(map_lineage_refusal)?;
    let mut pending = safe_transaction_directories(&root, ".pending")?;
    pending.sort();
    for directory in pending {
        if !directory.join("intent.json").exists() {
            rollback_owned_pre_intent_journal(repo_root, &root, &directory)?;
        }
    }
    Ok(())
}

pub(crate) fn recover_one_approval_authority_journal_locked(
    repo_root: &Path,
    journal_path: &str,
) -> Result<(), CharterApprovalRefusalV1> {
    let normalized = CanonicalWorkspace::new(repo_root)
        .normalize_repo_relative(journal_path)
        .map_err(|_| durability("approval recovery journal path is not normalized"))?;
    let directory = repo_root.join(normalized.as_str());
    let root = repo_root.join(APPROVAL_TRANSACTION_ROOT);
    if directory
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".committed"))
    {
        return Ok(());
    }
    if directory.parent() != Some(root.as_path()) {
        return Err(durability(
            "approval recovery journal is outside the approval transaction root",
        ));
    }
    reject_reparse_or_symlink(&root, true).map_err(map_lineage_refusal)?;
    reject_reparse_or_symlink(&directory, true).map_err(map_lineage_refusal)?;
    if !directory.join("intent.json").exists() {
        return rollback_owned_pre_intent_journal(repo_root, &root, &directory);
    }
    let intent_bytes = read_safe_file(repo_root, &directory.join("intent.json"), 64 * 1024)?;
    let intent = parse_exact_intent(&intent_bytes)?;
    let marker_path = directory.join("committed");
    if APPROVAL_STAGE_NAMES
        .iter()
        .any(|name| !directory.join(name).exists())
    {
        if marker_path.exists() {
            return Err(durability(
                "committed approval journal is missing operation-required staged bytes",
            ));
        }
        return rollback_incomplete_staged_journal(repo_root, &root, &directory, &intent);
    }
    let stages = read_staged_records(repo_root, &directory)?;
    validate_stages_against_intent(&intent, &stages)?;
    install_staged_approval(repo_root, &intent, &stages)?;
    let marker = approval_commit_marker(&intent_bytes);
    if marker_path.exists() {
        if read_safe_file(repo_root, &marker_path, 72)? != marker {
            return Err(durability("pending approval commit marker is not exact"));
        }
    } else {
        write_new_durable(&directory.join("committed.tmp"), &marker)?;
        fs::rename(directory.join("committed.tmp"), &marker_path)
            .map_err(|_| durability("approval recovery marker rename failed"))?;
        sync_directory(&directory).map_err(map_lineage_refusal)?;
    }
    finalize_pending_directory(&root, &directory)
}

const APPROVAL_STAGE_NAMES: [&str; 6] = [
    "response.new",
    "assertion.new",
    "use-transition.new",
    "approval.new",
    "prior-use-head.json",
    "use-head.new",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ApprovalJournalMarkerStateV1 {
    Absent,
    Exact,
}

/// One validated approval-family node for the shared authenticator-use
/// predecessor graph. The cross-family coordinator must append these nodes
/// after lexically ordered registry-family nodes while holding promotion then
/// registry locks; this hook is deliberately read-only and approval-local.
#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ApprovalUseJournalObservationV1 {
    pub(crate) journal_path: String,
    pub(crate) transaction_id: String,
    pub(crate) credential_id_hash: String,
    pub(crate) prior_use_head_ref: String,
    pub(crate) prior_use_head_fingerprint: String,
    pub(crate) prior_use_sequence: u64,
    pub(crate) result_use_head_ref: String,
    pub(crate) result_use_head_fingerprint: String,
    pub(crate) result_use_sequence: u64,
    pub(crate) marker_state: ApprovalJournalMarkerStateV1,
    pub(crate) finalized_directory: bool,
}

/// Discovers approval-family use journals without mutating them. Results are
/// returned in normalized repository-relative lexical byte order. Empty and
/// `intent.tmp`-only pre-intent directories are safe but have no graph node.
#[allow(dead_code)]
pub(crate) fn discover_approval_use_journals(
    repo_root: &Path,
) -> Result<Vec<ApprovalUseJournalObservationV1>, CharterApprovalRefusalV1> {
    let root = repo_root.join(APPROVAL_TRANSACTION_ROOT);
    if !root.exists() {
        return Ok(Vec::new());
    }
    reject_reparse_or_symlink(&root, true).map_err(map_lineage_refusal)?;
    let mut directories = safe_transaction_directories(&root, ".pending")?;
    directories.extend(safe_transaction_directories(&root, ".committed")?);
    directories.sort_by(|left, right| {
        left.as_os_str()
            .as_encoded_bytes()
            .cmp(right.as_os_str().as_encoded_bytes())
    });

    let mut observations = Vec::with_capacity(directories.len());
    let mut prior_heads = BTreeSet::new();
    let mut result_sequences = BTreeSet::new();
    for directory in directories {
        let entry_names = validate_discovered_approval_journal_entries(repo_root, &directory)?;
        if !entry_names.contains("intent.json") {
            if entry_names.is_empty()
                || (entry_names.len() == 1 && entry_names.contains("intent.tmp"))
            {
                continue;
            }
            return Err(durability(
                "approval journal has owned or ambiguous state without a durable intent",
            ));
        }
        let intent_bytes = read_safe_file(repo_root, &directory.join("intent.json"), 64 * 1024)?;
        let intent = parse_exact_intent(&intent_bytes)?;
        let finalized_directory = directory
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with(".committed"));
        validate_discovered_approval_intent(repo_root, &directory, &intent, finalized_directory)?;

        let marker_state = if entry_names.contains("committed") {
            let marker = read_safe_file(repo_root, &directory.join("committed"), 72)?;
            if marker.len() != 72 || marker != approval_commit_marker(&intent_bytes) {
                return Err(durability(
                    "approval journal commit marker does not match its raw intent",
                ));
            }
            ApprovalJournalMarkerStateV1::Exact
        } else {
            if finalized_directory {
                return Err(durability(
                    "finalized approval journal has no exact commit marker",
                ));
            }
            ApprovalJournalMarkerStateV1::Absent
        };

        let prior_key = (
            intent.credential_id_hash.clone(),
            intent.prior_use_head_fingerprint.clone(),
        );
        if !prior_heads.insert(prior_key) {
            return Err(durability(
                "approval use journals contain two successors for one prior head",
            ));
        }
        if !result_sequences.insert((
            intent.credential_id_hash.clone(),
            intent.result_use_sequence,
        )) {
            return Err(durability(
                "approval use journals contain a duplicate result sequence",
            ));
        }
        let relative = directory
            .strip_prefix(repo_root)
            .map_err(|_| durability("approval journal path escapes repository root"))?
            .to_str()
            .ok_or_else(|| durability("approval journal path is not UTF-8"))?
            .replace('\\', "/");
        let normalized = CanonicalWorkspace::new(repo_root)
            .normalize_repo_relative(&relative)
            .map_err(|_| durability("approval journal path is not normalized"))?;
        observations.push(ApprovalUseJournalObservationV1 {
            journal_path: normalized.as_str().to_owned(),
            transaction_id: intent.transaction_id,
            credential_id_hash: intent.credential_id_hash,
            prior_use_head_ref: intent.prior_use_head_ref,
            prior_use_head_fingerprint: intent.prior_use_head_fingerprint,
            prior_use_sequence: intent.prior_use_sequence,
            result_use_head_ref: intent.result_use_head_ref,
            result_use_head_fingerprint: intent.result_use_head_fingerprint,
            result_use_sequence: intent.result_use_sequence,
            marker_state,
            finalized_directory,
        });
    }
    observations.sort_by(|left, right| {
        left.journal_path
            .as_bytes()
            .cmp(right.journal_path.as_bytes())
    });
    Ok(observations)
}

fn validate_discovered_approval_journal_entries(
    repo_root: &Path,
    directory: &Path,
) -> Result<BTreeSet<String>, CharterApprovalRefusalV1> {
    let allowed = [
        "intent.json",
        "intent.tmp",
        "response.new",
        "assertion.new",
        "use-transition.new",
        "approval.new",
        "prior-use-head.json",
        "use-head.new",
        "committed.tmp",
        "committed",
    ];
    let mut names = BTreeSet::new();
    for entry in
        fs::read_dir(directory).map_err(|_| durability("approval journal discovery read failed"))?
    {
        let entry = entry.map_err(|_| durability("approval journal discovery entry failed"))?;
        let name = entry
            .file_name()
            .to_str()
            .ok_or_else(|| durability("approval journal entry name is not UTF-8"))?
            .to_owned();
        if !allowed.contains(&name.as_str()) || !names.insert(name) {
            return Err(durability(
                "approval journal contains an unowned or duplicate entry",
            ));
        }
        reject_reparse_or_symlink(&entry.path(), false).map_err(map_lineage_refusal)?;
        if !entry
            .file_type()
            .map_err(|_| durability("approval journal entry type read failed"))?
            .is_file()
        {
            return Err(durability(
                "approval journal entry is not a safe regular file",
            ));
        }
        let relative = entry
            .path()
            .strip_prefix(repo_root)
            .map_err(|_| durability("approval journal entry escapes repository root"))?
            .to_str()
            .ok_or_else(|| durability("approval journal entry path is not UTF-8"))?
            .replace('\\', "/");
        CanonicalWorkspace::new(repo_root)
            .normalize_repo_relative(&relative)
            .map_err(|_| durability("approval journal entry path is not normalized"))?;
    }
    Ok(names)
}

#[allow(
    clippy::manual_saturating_arithmetic,
    reason = "the protocol text spells out checked successor overflow as the maximum sequence"
)]
fn validate_discovered_approval_intent(
    repo_root: &Path,
    directory: &Path,
    intent: &ApprovalIntentV1,
    finalized_directory: bool,
) -> Result<(), CharterApprovalRefusalV1> {
    let suffix = if finalized_directory {
        ".committed"
    } else {
        ".pending"
    };
    let expected_name = format!("{}{}", intent.transaction_id, suffix);
    let approval_hex = intent
        .approval_ref
        .strip_prefix("approvals/approval_")
        .and_then(|value| value.strip_suffix(".json"));
    if directory.file_name().and_then(|name| name.to_str()) != Some(expected_name.as_str())
        || intent.approval_ref != format!("approvals/{}.json", intent.transaction_id)
        || !is_exact_approval_ref(&intent.approval_ref)
        || approval_hex.is_none_or(|hex| {
            intent.transaction_id != format!("approval_{hex}")
                || intent.approval_fingerprint != format!("sha256:{hex}")
        })
        || !bounded_nonempty(&intent.operation_id, MAX_OPERATION_ID_BYTES)
        || !bounded_nonempty(&intent.approval_class, MAX_SELECTOR_BYTES)
        || !bounded_nonempty(&intent.authority_ref, MAX_SELECTOR_BYTES)
        || DefinitionFingerprint::parse(&intent.credential_id_hash).is_err()
        || DefinitionFingerprint::parse(&intent.prior_use_head_fingerprint).is_err()
        || DefinitionFingerprint::parse(&intent.result_use_head_fingerprint).is_err()
        || DefinitionFingerprint::parse(&intent.prior_use_head_bytes_fingerprint).is_err()
        || DefinitionFingerprint::parse(&intent.result_use_head_bytes_fingerprint).is_err()
        || intent.prior_use_head_ref != intent.result_use_head_ref
        || intent.result_use_sequence
            != intent.prior_use_sequence.checked_add(1).unwrap_or(u64::MAX)
        || intent.result_use_sequence > 4096
    {
        return Err(lineage(
            "approval journal intent has invalid predecessor graph bindings",
        ));
    }
    for fingerprint in [
        &intent.candidate_fingerprint,
        &intent.approver_registry_state_fingerprint,
        &intent.registry_head_transition_fingerprint,
        &intent.response_fingerprint,
        &intent.response_bytes_fingerprint,
        &intent.assertion_fingerprint,
        &intent.assertion_bytes_fingerprint,
        &intent.use_transition_fingerprint,
        &intent.use_transition_bytes_fingerprint,
        &intent.approval_fingerprint,
        &intent.approval_bytes_fingerprint,
    ] {
        if DefinitionFingerprint::parse(fingerprint).is_err() {
            return Err(lineage(
                "approval journal intent contains an invalid fingerprint",
            ));
        }
    }
    for reference in [&intent.prior_use_head_ref, &intent.result_use_head_ref] {
        if !bounded_nonempty(reference, MAX_REFERENCE_BYTES) {
            return Err(lineage("approval use-head reference is not bounded"));
        }
        CanonicalWorkspace::new(repo_root)
            .normalize_repo_relative(reference)
            .map_err(|_| lineage("approval use-head reference is not normalized"))?;
    }
    Ok(())
}

fn rollback_incomplete_staged_journal(
    repo_root: &Path,
    root: &Path,
    directory: &Path,
    intent: &ApprovalIntentV1,
) -> Result<(), CharterApprovalRefusalV1> {
    let allowed = [
        "intent.json",
        "intent.tmp",
        "response.new",
        "assertion.new",
        "use-transition.new",
        "approval.new",
        "prior-use-head.json",
        "use-head.new",
        "committed.tmp",
    ];
    let mut entries = fs::read_dir(directory)
        .map_err(|_| durability("incomplete staged approval journal read failed"))?
        .map(|entry| {
            entry.map_err(|_| durability("incomplete staged approval journal entry read failed"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in &entries {
        let name = entry
            .file_name()
            .to_str()
            .ok_or_else(|| durability("incomplete staged approval entry name is not UTF-8"))?
            .to_owned();
        if !allowed.contains(&name.as_str()) {
            return Err(durability(
                "incomplete staged approval journal contains unowned state",
            ));
        }
        reject_reparse_or_symlink(&entry.path(), false).map_err(map_lineage_refusal)?;
        if APPROVAL_STAGE_NAMES.contains(&name.as_str()) {
            let bytes = read_safe_file(repo_root, &entry.path(), MAX_RECORD_BYTES)?;
            validate_single_stage_against_intent(intent, &name, &bytes)?;
        }
    }

    let prior_path = directory.join("prior-use-head.json");
    let result_path = directory.join("use-head.new");
    if !prior_path.exists() || !result_path.exists() {
        return Err(durability(
            "incomplete approval journal cannot safely restore its mutable use head",
        ));
    }
    if intent.prior_use_head_ref != intent.result_use_head_ref {
        return Err(lineage(
            "approval transaction changes the mutable use-head reference",
        ));
    }
    let prior = read_safe_file(repo_root, &prior_path, MAX_RECORD_BYTES)?;
    let result = read_safe_file(repo_root, &result_path, MAX_RECORD_BYTES)?;
    let target = repo_root
        .join(".handbook/state")
        .join(&intent.prior_use_head_ref);
    let current = read_safe_file(repo_root, &target, MAX_RECORD_BYTES)?;
    if current == result {
        let parent = target
            .parent()
            .ok_or_else(|| durability("credential use head has no parent during rollback"))?;
        let temporary = parent.join(format!(".{}.approval-rollback", intent.transaction_id));
        if temporary.exists() {
            if read_safe_file(repo_root, &temporary, MAX_RECORD_BYTES)? != prior {
                return Err(durability(
                    "approval use-head rollback stage has foreign bytes",
                ));
            }
        } else {
            write_new_durable(&temporary, &prior)?;
        }
        fs::rename(&temporary, &target)
            .map_err(|_| durability("approval use-head rollback replacement failed"))?;
        sync_directory(parent).map_err(map_lineage_refusal)?;
    } else if current != prior {
        return Err(refused(
            CharterApprovalRefusalCodeV1::TransactionConflict,
            "credential use head forked before incomplete approval rollback",
            false,
            "retain the journal and repair the exact credential use chain",
        ));
    }

    for entry in entries {
        fs::remove_file(entry.path())
            .map_err(|_| durability("incomplete approval journal file rollback failed"))?;
    }
    fs::remove_dir(directory)
        .map_err(|_| durability("incomplete approval journal rollback failed"))?;
    sync_directory(root).map_err(map_lineage_refusal)?;
    Ok(())
}

fn validate_single_stage_against_intent(
    intent: &ApprovalIntentV1,
    name: &str,
    bytes: &[u8],
) -> Result<(), CharterApprovalRefusalV1> {
    match name {
        "response.new" | "assertion.new" | "approval.new" => {
            let (class, expected_ref, expected_fingerprint, expected_bytes_fingerprint) = match name
            {
                "response.new" => (
                    LineageRecordClassV1::AuthenticatorGetAssertionResponse,
                    intent.response_ref.as_str(),
                    intent.response_fingerprint.as_str(),
                    intent.response_bytes_fingerprint.as_str(),
                ),
                "assertion.new" => (
                    LineageRecordClassV1::AuthenticatorAssertion,
                    intent.assertion_ref.as_str(),
                    intent.assertion_fingerprint.as_str(),
                    intent.assertion_bytes_fingerprint.as_str(),
                ),
                _ => (
                    LineageRecordClassV1::Approval,
                    intent.approval_ref.as_str(),
                    intent.approval_fingerprint.as_str(),
                    intent.approval_bytes_fingerprint.as_str(),
                ),
            };
            let identity = validate_record(class, bytes).map_err(map_lineage_refusal)?;
            if identity.relative_ref != expected_ref
                || identity.fingerprint != expected_fingerprint
                || DefinitionFingerprint::from_bytes(bytes).as_str() != expected_bytes_fingerprint
            {
                return Err(lineage(
                    "approval transaction stage identity disagrees with intent",
                ));
            }
        }
        "use-transition.new" => {
            parse_exact_runtime_record(
                bytes,
                "transition_fingerprint",
                &intent.use_transition_fingerprint,
            )?;
            if DefinitionFingerprint::from_bytes(bytes).as_str()
                != intent.use_transition_bytes_fingerprint
            {
                return Err(lineage(
                    "approval use-transition staged bytes disagree with intent",
                ));
            }
        }
        "prior-use-head.json" => {
            parse_exact_runtime_record(
                bytes,
                "head_fingerprint",
                &intent.prior_use_head_fingerprint,
            )?;
            if DefinitionFingerprint::from_bytes(bytes).as_str()
                != intent.prior_use_head_bytes_fingerprint
            {
                return Err(lineage(
                    "approval prior use-head stage bytes disagree with intent",
                ));
            }
        }
        "use-head.new" => {
            parse_exact_runtime_record(
                bytes,
                "head_fingerprint",
                &intent.result_use_head_fingerprint,
            )?;
            if DefinitionFingerprint::from_bytes(bytes).as_str()
                != intent.result_use_head_bytes_fingerprint
            {
                return Err(lineage(
                    "approval result use-head stage bytes disagree with intent",
                ));
            }
        }
        _ => {}
    }
    Ok(())
}

fn rollback_owned_pre_intent_journal(
    repo_root: &Path,
    root: &Path,
    directory: &Path,
) -> Result<(), CharterApprovalRefusalV1> {
    let mut entries = fs::read_dir(directory)
        .map_err(|_| durability("incomplete approval journal read failed"))?
        .map(|entry| entry.map_err(|_| durability("incomplete approval journal entry read failed")))
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());
    if entries.is_empty() {
        fs::remove_dir(directory)
            .map_err(|_| durability("empty approval journal rollback failed"))?;
        sync_directory(root).map_err(map_lineage_refusal)?;
        return Ok(());
    }
    if entries.len() != 1 || entries[0].file_name() != "intent.tmp" {
        return Err(durability(
            "incomplete approval journal contains unowned or ambiguous state",
        ));
    }
    let intent_tmp = entries[0].path();
    reject_reparse_or_symlink(&intent_tmp, false).map_err(map_lineage_refusal)?;
    let relative = intent_tmp
        .strip_prefix(repo_root)
        .map_err(|_| durability("incomplete approval journal escapes repository root"))?;
    CanonicalWorkspace::new(repo_root)
        .normalize_repo_relative(
            relative
                .to_str()
                .ok_or_else(|| durability("incomplete approval journal path is not UTF-8"))?,
        )
        .map_err(|_| durability("incomplete approval journal path is not normalized"))?;
    fs::remove_file(&intent_tmp)
        .map_err(|_| durability("approval intent temporary rollback failed"))?;
    fs::remove_dir(directory)
        .map_err(|_| durability("incomplete approval journal rollback failed"))?;
    sync_directory(root).map_err(map_lineage_refusal)?;
    Ok(())
}

pub(crate) fn observe_committed_candidate_approval_refs_locked(
    repo_root: &Path,
    candidate_ref: &str,
    candidate_fingerprint: &str,
    required_anchor_approval_ref: Option<&str>,
) -> Result<Vec<String>, CharterApprovalRefusalV1> {
    if !bounded_nonempty(candidate_ref, MAX_REFERENCE_BYTES)
        || DefinitionFingerprint::parse(candidate_fingerprint).is_err()
        || required_anchor_approval_ref.is_some_and(|reference| {
            !bounded_nonempty(reference, MAX_REFERENCE_BYTES) || !is_exact_approval_ref(reference)
        })
    {
        return Err(refused(
            CharterApprovalRefusalCodeV1::InvalidRequest,
            "retained approval observation selectors are not exact and bounded",
            false,
            "supply one exact candidate identity and committed approval anchor",
        ));
    }
    let mut approval_refs = read_committed_approval_intents(repo_root)?
        .into_iter()
        .filter(|intent| {
            intent.candidate_ref == candidate_ref
                && intent.candidate_fingerprint == candidate_fingerprint
        })
        .map(|intent| intent.approval_ref)
        .collect::<Vec<_>>();
    approval_refs.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    if approval_refs.len() > 64
        || approval_refs.windows(2).any(|pair| pair[0] >= pair[1])
        || approval_refs
            .iter()
            .any(|reference| !is_exact_approval_ref(reference))
    {
        return Err(lineage(
            "retained candidate approval refs are duplicated, malformed, or out of bounds",
        ));
    }
    if let Some(required_anchor) = required_anchor_approval_ref {
        if approval_refs
            .binary_search_by(|observed| observed.as_bytes().cmp(required_anchor.as_bytes()))
            .is_err()
        {
            return Err(refused(
                CharterApprovalRefusalCodeV1::AuthorizationRefused,
                "approval anchor has no committed journal authority for the candidate",
                false,
                "select one exact committed approval ref for the candidate",
            ));
        }
    }
    Ok(approval_refs)
}

fn is_exact_approval_ref(reference: &str) -> bool {
    reference
        .strip_prefix("approvals/approval_")
        .and_then(|value| value.strip_suffix(".json"))
        .is_some_and(|hex| {
            hex.len() == 64
                && hex
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
        })
}

fn committed_pair_already_satisfied(
    repo_root: &Path,
    candidate_ref: &str,
    candidate_fingerprint: &str,
    pair: &ApproverAuthorityPairV1,
) -> Result<bool, CharterApprovalRefusalV1> {
    Ok(read_committed_approval_intents(repo_root)?
        .into_iter()
        .any(|intent| {
            intent.candidate_ref == candidate_ref
                && intent.candidate_fingerprint == candidate_fingerprint
                && intent.approval_class == pair.approval_class
                && intent.authority_ref == pair.authority_ref
        }))
}

fn validate_use_chain(
    repo_root: &Path,
    credential: &CommittedApproverCredentialV1,
    challenge_fingerprint: &str,
    nonce_fingerprint: &str,
) -> Result<(), CharterApprovalRefusalV1> {
    let intents = read_committed_approval_intents(repo_root)?;
    let mut transitions = BTreeMap::<String, Value>::new();
    let mut sequences = BTreeSet::new();
    for intent in intents
        .iter()
        .filter(|intent| intent.credential_id_hash == credential.credential_id_hash)
    {
        let bytes = read_state_ref(repo_root, &intent.use_transition_ref, MAX_RECORD_BYTES)?;
        let transition = parse_exact_runtime_record(
            &bytes,
            "transition_fingerprint",
            &intent.use_transition_fingerprint,
        )?;
        if transition.get("credential_id_hash").and_then(Value::as_str)
            != Some(credential.credential_id_hash.as_str())
        {
            return Err(lineage("committed use transition names another credential"));
        }
        if transition
            .get("challenge_fingerprint")
            .and_then(Value::as_str)
            == Some(challenge_fingerprint)
            || transition.get("nonce_sha256").and_then(Value::as_str) == Some(nonce_fingerprint)
        {
            return Err(refused(
                CharterApprovalRefusalCodeV1::AuthorizationRefused,
                "authenticator challenge or nonce has already been committed",
                false,
                "retry with a fresh operating-system nonce and challenge",
            ));
        }
        let sequence = transition
            .get("sequence")
            .and_then(Value::as_u64)
            .ok_or_else(|| lineage("committed use transition sequence is absent"))?;
        if !sequences.insert(sequence) {
            return Err(lineage("committed credential use sequence is duplicated"));
        }
        let result = required_string(&transition, "result_head_fingerprint")?.to_owned();
        if transitions.insert(result, transition).is_some() {
            return Err(lineage("committed use chain has duplicate result heads"));
        }
    }
    let mut cursor = credential.use_head_fingerprint.clone();
    let mut expected = u64::from(credential.use_sequence);
    let mut visited = BTreeSet::new();
    while expected > 0 {
        if !visited.insert(cursor.clone()) {
            return Err(lineage(
                "committed authenticator use chain contains a cycle",
            ));
        }
        let Some(transition) = transitions.get(&cursor) else {
            return Err(lineage("committed authenticator use chain is incomplete"));
        };
        if transition.get("sequence").and_then(Value::as_u64) != Some(expected) {
            return Err(lineage(
                "committed authenticator use transition is out of sequence",
            ));
        }
        cursor = required_string(transition, "prior_head_fingerprint")?.to_owned();
        expected -= 1;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn ensure_retained_authority_stable(
    repo_root: &Path,
    lineage_store: &TrustedLineageStoreV1,
    authority: &RetainedApproverRegistryObservationV1,
    credential: &CommittedApproverCredentialV1,
    candidate_bytes: &[u8],
    candidate_ref: &str,
    candidate_fingerprint: &str,
    canonical: &CanonicalBasisObservation,
    prior_use_head_bytes: &[u8],
) -> Result<(), CharterApprovalRefusalV1> {
    let state = lineage_store
        .read_record(
            LineageRecordClassV1::RegistryState,
            &authority.state_ref,
            &authority.state_fingerprint,
        )
        .map_err(map_lineage_refusal)?;
    let head = lineage_store
        .read_record(
            LineageRecordClassV1::RegistryTransition,
            &authority.head_transition_ref,
            &authority.head_transition_fingerprint,
        )
        .map_err(map_lineage_refusal)?;
    let candidate = lineage_store
        .read_record(
            LineageRecordClassV1::Candidate,
            candidate_ref,
            candidate_fingerprint,
        )
        .map_err(map_lineage_refusal)?;
    if state != authority.state_bytes
        || head != authority.head_transition_bytes
        || candidate != candidate_bytes
        || read_retained_use_head(repo_root, credential)? != prior_use_head_bytes
        || read_canonical_bytes(repo_root)? != canonical.bytes
    {
        return Err(refused(
            CharterApprovalRefusalCodeV1::StaleBasis,
            "retained candidate, registry, canonical basis, or credential use head changed",
            true,
            "restart approval from a fresh retained authority observation",
        ));
    }
    Ok(())
}

fn commit_approval_transaction(
    repo_root: &Path,
    _lineage_store: &TrustedLineageStoreV1,
    authority: &RetainedApproverRegistryObservationV1,
    credential: &CommittedApproverCredentialV1,
    _candidate_bytes: &[u8],
    _canonical: &CanonicalBasisObservation,
    records: ApprovalRecords,
) -> Result<CharterApprovalSuccessV1, CharterApprovalRefusalV1> {
    let root = repo_root.join(APPROVAL_TRANSACTION_ROOT);
    create_safe_directories(repo_root, &root).map_err(map_lineage_refusal)?;
    let intent = ApprovalIntentV1::from_records(authority, credential, &records)?;
    let intent_bytes = serde_json_canonicalizer::to_vec(&intent)
        .map_err(|_| durability("approval intent canonicalization failed"))?;
    let pending = root.join(format!("{}.pending", intent.transaction_id));
    let committed = root.join(format!("{}.committed", intent.transaction_id));
    if pending.exists() || committed.exists() {
        return Err(refused(
            CharterApprovalRefusalCodeV1::TransactionConflict,
            "approval transaction identity already exists",
            false,
            "retain the existing committed authority or restart from current state",
        ));
    }
    fs::create_dir(&pending).map_err(|_| durability("approval pending directory create failed"))?;
    sync_directory(&root).map_err(map_lineage_refusal)?;
    write_new_durable(&pending.join("intent.tmp"), &intent_bytes)?;
    fs::rename(pending.join("intent.tmp"), pending.join("intent.json"))
        .map_err(|_| durability("approval intent rename failed"))?;
    sync_directory(&pending).map_err(map_lineage_refusal)?;
    for (name, bytes) in [
        ("response.new", records.response.bytes.as_slice()),
        ("assertion.new", records.assertion.bytes.as_slice()),
        (
            "use-transition.new",
            records.use_transition.bytes.as_slice(),
        ),
        ("approval.new", records.approval.bytes.as_slice()),
        (
            "prior-use-head.json",
            records.prior_use_head_bytes.as_slice(),
        ),
        ("use-head.new", records.result_use_head.bytes.as_slice()),
    ] {
        write_new_durable(&pending.join(name), bytes)?;
    }
    sync_directory(&pending).map_err(map_lineage_refusal)?;
    let stages = StagedApprovalRecords {
        response: records.response.bytes.clone(),
        assertion: records.assertion.bytes.clone(),
        use_transition: records.use_transition.bytes.clone(),
        approval: records.approval.bytes.clone(),
        prior_use_head: records.prior_use_head_bytes.clone(),
        result_use_head: records.result_use_head.bytes.clone(),
    };
    validate_stages_against_intent(&intent, &stages)?;
    install_staged_approval(repo_root, &intent, &stages)?;
    let marker = approval_commit_marker(&intent_bytes);
    write_new_durable(&pending.join("committed.tmp"), &marker)?;
    fs::rename(pending.join("committed.tmp"), pending.join("committed"))
        .map_err(|_| durability("approval commit-marker rename failed"))?;
    sync_directory(&pending).map_err(map_lineage_refusal)?;
    fs::rename(&pending, &committed)
        .map_err(|_| durability("approval transaction finalization failed"))?;
    sync_directory(&root).map_err(map_lineage_refusal)?;
    Ok(CharterApprovalSuccessV1 {
        operation_id: records.operation_id,
        response_ref: records.response.relative_ref,
        response_fingerprint: records.response.fingerprint,
        assertion_ref: records.assertion.relative_ref,
        assertion_fingerprint: records.assertion.fingerprint,
        use_transition_ref: records.use_transition.relative_ref,
        use_transition_fingerprint: records.use_transition.fingerprint,
        approval_ref: records.approval.relative_ref,
        approval_fingerprint: records.approval.fingerprint,
        result_use_head_ref: records.result_use_head.relative_ref,
        result_use_head_fingerprint: records.result_use_head.fingerprint,
        changed_paths: vec![
            intent.response_ref,
            intent.assertion_ref,
            intent.use_transition_ref,
            intent.approval_ref,
            intent.result_use_head_ref,
        ],
    })
}

struct StagedApprovalRecords {
    response: Vec<u8>,
    assertion: Vec<u8>,
    use_transition: Vec<u8>,
    approval: Vec<u8>,
    prior_use_head: Vec<u8>,
    result_use_head: Vec<u8>,
}

fn read_staged_records(
    repo_root: &Path,
    directory: &Path,
) -> Result<StagedApprovalRecords, CharterApprovalRefusalV1> {
    Ok(StagedApprovalRecords {
        response: read_safe_file(repo_root, &directory.join("response.new"), MAX_RECORD_BYTES)?,
        assertion: read_safe_file(
            repo_root,
            &directory.join("assertion.new"),
            MAX_RECORD_BYTES,
        )?,
        use_transition: read_safe_file(
            repo_root,
            &directory.join("use-transition.new"),
            MAX_RECORD_BYTES,
        )?,
        approval: read_safe_file(repo_root, &directory.join("approval.new"), MAX_RECORD_BYTES)?,
        prior_use_head: read_safe_file(
            repo_root,
            &directory.join("prior-use-head.json"),
            MAX_RECORD_BYTES,
        )?,
        result_use_head: read_safe_file(
            repo_root,
            &directory.join("use-head.new"),
            MAX_RECORD_BYTES,
        )?,
    })
}

fn validate_stages_against_intent(
    intent: &ApprovalIntentV1,
    stages: &StagedApprovalRecords,
) -> Result<(), CharterApprovalRefusalV1> {
    for (bytes, expected) in [
        (
            stages.response.as_slice(),
            intent.response_bytes_fingerprint.as_str(),
        ),
        (
            stages.assertion.as_slice(),
            intent.assertion_bytes_fingerprint.as_str(),
        ),
        (
            stages.use_transition.as_slice(),
            intent.use_transition_bytes_fingerprint.as_str(),
        ),
        (
            stages.approval.as_slice(),
            intent.approval_bytes_fingerprint.as_str(),
        ),
    ] {
        if DefinitionFingerprint::from_bytes(bytes).as_str() != expected {
            return Err(lineage(
                "approval staged-byte fingerprint disagrees with intent",
            ));
        }
    }
    for (class, bytes, expected_ref, expected_fingerprint) in [
        (
            LineageRecordClassV1::AuthenticatorGetAssertionResponse,
            stages.response.as_slice(),
            intent.response_ref.as_str(),
            intent.response_fingerprint.as_str(),
        ),
        (
            LineageRecordClassV1::AuthenticatorAssertion,
            stages.assertion.as_slice(),
            intent.assertion_ref.as_str(),
            intent.assertion_fingerprint.as_str(),
        ),
        (
            LineageRecordClassV1::Approval,
            stages.approval.as_slice(),
            intent.approval_ref.as_str(),
            intent.approval_fingerprint.as_str(),
        ),
    ] {
        let identity = validate_record(class, bytes).map_err(map_lineage_refusal)?;
        if identity.relative_ref != expected_ref || identity.fingerprint != expected_fingerprint {
            return Err(lineage(
                "approval transaction stage identity disagrees with intent",
            ));
        }
    }
    parse_exact_runtime_record(
        &stages.use_transition,
        "transition_fingerprint",
        &intent.use_transition_fingerprint,
    )?;
    parse_exact_runtime_record(
        &stages.prior_use_head,
        "head_fingerprint",
        &intent.prior_use_head_fingerprint,
    )?;
    parse_exact_runtime_record(
        &stages.result_use_head,
        "head_fingerprint",
        &intent.result_use_head_fingerprint,
    )?;
    if DefinitionFingerprint::from_bytes(&stages.prior_use_head).as_str()
        != intent.prior_use_head_bytes_fingerprint
        || DefinitionFingerprint::from_bytes(&stages.result_use_head).as_str()
            != intent.result_use_head_bytes_fingerprint
    {
        return Err(lineage(
            "approval use-head stage bytes disagree with intent",
        ));
    }
    Ok(())
}

fn install_staged_approval(
    repo_root: &Path,
    intent: &ApprovalIntentV1,
    stages: &StagedApprovalRecords,
) -> Result<(), CharterApprovalRefusalV1> {
    let lineage_store = TrustedLineageStoreV1::new(repo_root);
    for (class, bytes) in [
        (
            LineageRecordClassV1::AuthenticatorGetAssertionResponse,
            stages.response.as_slice(),
        ),
        (
            LineageRecordClassV1::AuthenticatorAssertion,
            stages.assertion.as_slice(),
        ),
        (LineageRecordClassV1::Approval, stages.approval.as_slice()),
    ] {
        lineage_store
            .append_record(class, bytes)
            .map_err(map_lineage_refusal)?;
    }
    install_immutable_state_record(
        repo_root,
        &intent.use_transition_ref,
        &stages.use_transition,
    )?;
    let current = read_state_ref(repo_root, &intent.prior_use_head_ref, MAX_RECORD_BYTES)?;
    if current == stages.result_use_head {
        return Ok(());
    }
    if current != stages.prior_use_head {
        return Err(refused(
            CharterApprovalRefusalCodeV1::TransactionConflict,
            "credential use head forked before approval commit",
            false,
            "repair or retain the exact committed credential use chain",
        ));
    }
    let target = repo_root
        .join(".handbook/state")
        .join(&intent.result_use_head_ref);
    let parent = target
        .parent()
        .ok_or_else(|| durability("credential use head has no parent"))?;
    create_safe_directories(repo_root, parent).map_err(map_lineage_refusal)?;
    let temporary = parent.join(format!(".{}.approval-installing", intent.transaction_id));
    if temporary.exists() {
        if read_safe_file(repo_root, &temporary, MAX_RECORD_BYTES)? != stages.result_use_head {
            return Err(durability(
                "approval use-head install stage has foreign bytes",
            ));
        }
    } else {
        write_new_durable(&temporary, &stages.result_use_head)?;
    }
    fs::rename(&temporary, &target)
        .map_err(|_| durability("approval use-head replacement failed"))?;
    sync_directory(parent).map_err(map_lineage_refusal)?;
    Ok(())
}

fn install_immutable_state_record(
    repo_root: &Path,
    relative_ref: &str,
    bytes: &[u8],
) -> Result<(), CharterApprovalRefusalV1> {
    let target = repo_root.join(".handbook/state").join(relative_ref);
    let parent = target
        .parent()
        .ok_or_else(|| durability("immutable approval record has no parent"))?;
    create_safe_directories(repo_root, parent).map_err(map_lineage_refusal)?;
    match create_new_file(&target) {
        Ok(mut file) => {
            file.write_all(bytes)
                .map_err(|_| durability("immutable approval record write failed"))?;
            file.sync_all()
                .map_err(|_| durability("immutable approval record flush failed"))?;
            sync_directory(parent).map_err(map_lineage_refusal)?;
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            if read_safe_file(repo_root, &target, MAX_RECORD_BYTES)? != bytes {
                return Err(lineage(
                    "immutable approval record already has different bytes",
                ));
            }
        }
        Err(_) => return Err(durability("immutable approval record create-new failed")),
    }
    Ok(())
}

fn read_committed_approval_intents(
    repo_root: &Path,
) -> Result<Vec<ApprovalIntentV1>, CharterApprovalRefusalV1> {
    let root = repo_root.join(APPROVAL_TRANSACTION_ROOT);
    if !root.exists() {
        return Ok(Vec::new());
    }
    reject_reparse_or_symlink(&root, true).map_err(map_lineage_refusal)?;
    let mut directories = safe_transaction_directories(&root, ".committed")?;
    directories.sort();
    let mut intents = Vec::with_capacity(directories.len());
    for directory in directories {
        let bytes = read_safe_file(repo_root, &directory.join("intent.json"), 64 * 1024)?;
        let marker = read_safe_file(repo_root, &directory.join("committed"), 72)?;
        if marker != approval_commit_marker(&bytes) || marker.len() != 72 {
            return Err(lineage("committed approval marker is not exact"));
        }
        let intent = parse_exact_intent(&bytes)?;
        let stages = read_staged_records(repo_root, &directory)?;
        validate_stages_against_intent(&intent, &stages)?;
        let lineage_store = TrustedLineageStoreV1::new(repo_root);
        for (class, reference, fingerprint, staged) in [
            (
                LineageRecordClassV1::AuthenticatorGetAssertionResponse,
                intent.response_ref.as_str(),
                intent.response_fingerprint.as_str(),
                stages.response.as_slice(),
            ),
            (
                LineageRecordClassV1::AuthenticatorAssertion,
                intent.assertion_ref.as_str(),
                intent.assertion_fingerprint.as_str(),
                stages.assertion.as_slice(),
            ),
            (
                LineageRecordClassV1::Approval,
                intent.approval_ref.as_str(),
                intent.approval_fingerprint.as_str(),
                stages.approval.as_slice(),
            ),
        ] {
            let installed = lineage_store
                .read_record(class, reference, fingerprint)
                .map_err(map_lineage_refusal)?;
            if installed != staged {
                return Err(lineage(
                    "committed approval journal and immutable record disagree",
                ));
            }
        }
        if read_state_ref(repo_root, &intent.use_transition_ref, MAX_RECORD_BYTES)?
            != stages.use_transition
        {
            return Err(lineage(
                "committed approval journal and use transition disagree",
            ));
        }
        validate_committed_intent_bindings(&intent, &stages)?;
        intents.push(intent);
    }
    Ok(intents)
}

#[allow(
    clippy::manual_saturating_arithmetic,
    reason = "the protocol text spells out checked successor overflow as the maximum sequence"
)]
fn validate_committed_intent_bindings(
    intent: &ApprovalIntentV1,
    stages: &StagedApprovalRecords,
) -> Result<(), CharterApprovalRefusalV1> {
    if intent.prior_use_head_ref != intent.result_use_head_ref {
        return Err(lineage(
            "committed approval changes the mutable use-head reference",
        ));
    }
    let assertion: Value = serde_json::from_slice(&stages.assertion)
        .map_err(|_| lineage("committed assertion stage is not JSON"))?;
    let approval: Value = serde_json::from_slice(&stages.approval)
        .map_err(|_| lineage("committed approval stage is not JSON"))?;
    let transition = parse_exact_runtime_record(
        &stages.use_transition,
        "transition_fingerprint",
        &intent.use_transition_fingerprint,
    )?;
    let prior = parse_exact_runtime_record(
        &stages.prior_use_head,
        "head_fingerprint",
        &intent.prior_use_head_fingerprint,
    )?;
    let result = parse_exact_runtime_record(
        &stages.result_use_head,
        "head_fingerprint",
        &intent.result_use_head_fingerprint,
    )?;

    for (record, field, expected) in [
        (&approval, "candidate_ref", intent.candidate_ref.as_str()),
        (
            &approval,
            "candidate_fingerprint",
            intent.candidate_fingerprint.as_str(),
        ),
        (&approval, "approval_class", intent.approval_class.as_str()),
        (&approval, "authority_ref", intent.authority_ref.as_str()),
        (
            &approval,
            "approver_registry_state_ref",
            intent.approver_registry_state_ref.as_str(),
        ),
        (
            &approval,
            "approver_registry_state_fingerprint",
            intent.approver_registry_state_fingerprint.as_str(),
        ),
        (
            &approval,
            "registry_head_transition_ref",
            intent.registry_head_transition_ref.as_str(),
        ),
        (
            &approval,
            "registry_head_transition_fingerprint",
            intent.registry_head_transition_fingerprint.as_str(),
        ),
        (
            &approval,
            "authenticator_assertion_ref",
            intent.assertion_ref.as_str(),
        ),
        (
            &approval,
            "authenticator_assertion_fingerprint",
            intent.assertion_fingerprint.as_str(),
        ),
        (
            &assertion,
            "decoded_response_ref",
            intent.response_ref.as_str(),
        ),
        (
            &assertion,
            "decoded_response_fingerprint",
            intent.response_fingerprint.as_str(),
        ),
        (
            &assertion,
            "credential_id_hash",
            intent.credential_id_hash.as_str(),
        ),
        (
            &transition,
            "credential_id_hash",
            intent.credential_id_hash.as_str(),
        ),
        (
            &transition,
            "prior_head_fingerprint",
            intent.prior_use_head_fingerprint.as_str(),
        ),
        (
            &transition,
            "result_head_fingerprint",
            intent.result_use_head_fingerprint.as_str(),
        ),
        (&transition, "assertion_ref", intent.assertion_ref.as_str()),
        (
            &transition,
            "assertion_fingerprint",
            intent.assertion_fingerprint.as_str(),
        ),
        (
            &prior,
            "credential_id_hash",
            intent.credential_id_hash.as_str(),
        ),
        (
            &result,
            "credential_id_hash",
            intent.credential_id_hash.as_str(),
        ),
        (&result, "last_assertion_ref", intent.assertion_ref.as_str()),
        (
            &result,
            "last_assertion_fingerprint",
            intent.assertion_fingerprint.as_str(),
        ),
    ] {
        if record.get(field).and_then(Value::as_str) != Some(expected) {
            return Err(lineage(
                "committed approval intent and staged record bindings disagree",
            ));
        }
    }
    if approval.get("actor_ref").and_then(Value::as_str)
        != Some(format!("credential:{}", intent.credential_id_hash).as_str())
    {
        return Err(lineage(
            "committed approval actor and credential identity disagree",
        ));
    }
    let approval_id = intent
        .approval_ref
        .strip_prefix("approvals/")
        .and_then(|value| value.strip_suffix(".json"))
        .ok_or_else(|| lineage("committed approval ref is malformed"))?;
    if transition.get("consumer_kind").and_then(Value::as_str) != Some("approval")
        || transition.get("consumer_id").and_then(Value::as_str) != Some(approval_id)
    {
        return Err(lineage(
            "committed use transition names another approval consumer",
        ));
    }
    let prior_sequence = prior
        .get("sequence")
        .and_then(Value::as_u64)
        .ok_or_else(|| lineage("committed prior use-head sequence is absent"))?;
    if prior_sequence != intent.prior_use_sequence
        || intent.result_use_sequence != prior_sequence.checked_add(1).unwrap_or(u64::MAX)
        || transition.get("sequence").and_then(Value::as_u64) != Some(intent.result_use_sequence)
        || result.get("sequence").and_then(Value::as_u64) != Some(intent.result_use_sequence)
    {
        return Err(lineage(
            "committed approval use sequence is not one exact successor",
        ));
    }
    Ok(())
}

fn safe_transaction_directories(
    root: &Path,
    suffix: &str,
) -> Result<Vec<PathBuf>, CharterApprovalRefusalV1> {
    let mut result = Vec::new();
    for entry in fs::read_dir(root).map_err(|_| durability("approval journal read failed"))? {
        let entry = entry.map_err(|_| durability("approval journal entry read failed"))?;
        let name = entry
            .file_name()
            .to_str()
            .ok_or_else(|| durability("approval journal entry name is not UTF-8"))?
            .to_owned();
        if name.ends_with(suffix) {
            reject_reparse_or_symlink(&entry.path(), true).map_err(map_lineage_refusal)?;
            result.push(entry.path());
        }
    }
    Ok(result)
}

fn parse_exact_intent(bytes: &[u8]) -> Result<ApprovalIntentV1, CharterApprovalRefusalV1> {
    let intent: ApprovalIntentV1 =
        serde_json::from_slice(bytes).map_err(|_| lineage("approval intent is not closed JSON"))?;
    let canonical = serde_json_canonicalizer::to_vec(&intent)
        .map_err(|_| lineage("approval intent cannot be canonicalized"))?;
    if canonical != bytes
        || intent.schema_id != "handbook.approval-transaction-intent"
        || intent.schema_version != "1.0"
    {
        return Err(lineage("approval intent is not exact canonical authority"));
    }
    Ok(intent)
}

fn parse_exact_runtime_record(
    bytes: &[u8],
    fingerprint_field: &str,
    expected_fingerprint: &str,
) -> Result<Value, CharterApprovalRefusalV1> {
    let value: Value = serde_json::from_slice(bytes)
        .map_err(|_| lineage("runtime authority record is not JSON"))?;
    if serde_json_canonicalizer::to_vec(&value)
        .map_err(|_| lineage("runtime authority record cannot be canonicalized"))?
        != bytes
    {
        return Err(lineage("runtime authority record is not exact JCS"));
    }
    if value.get(fingerprint_field).and_then(Value::as_str) != Some(expected_fingerprint) {
        return Err(lineage(
            "runtime authority record declares a different fingerprint",
        ));
    }
    let mut preimage = value.clone();
    preimage
        .as_object_mut()
        .ok_or_else(|| lineage("runtime authority record root is not an object"))?
        .remove(fingerprint_field);
    let observed = DefinitionFingerprint::from_json_value(&preimage)
        .map_err(|_| lineage("runtime authority fingerprint preimage is invalid"))?
        .to_string();
    if observed != expected_fingerprint {
        return Err(lineage(
            "runtime authority fingerprint does not match exact bytes",
        ));
    }
    Ok(value)
}

fn read_retained_use_head(
    repo_root: &Path,
    credential: &CommittedApproverCredentialV1,
) -> Result<Vec<u8>, CharterApprovalRefusalV1> {
    let bytes = read_state_ref(repo_root, &credential.use_head_ref, MAX_RECORD_BYTES)?;
    let value =
        parse_exact_runtime_record(&bytes, "head_fingerprint", &credential.use_head_fingerprint)?;
    if value.get("credential_id_hash").and_then(Value::as_str)
        != Some(credential.credential_id_hash.as_str())
        || value.get("sequence").and_then(Value::as_u64) != Some(u64::from(credential.use_sequence))
        || value.get("sign_count").and_then(Value::as_u64) != Some(u64::from(credential.sign_count))
    {
        return Err(lineage(
            "retained credential use head fields disagree with registry observation",
        ));
    }
    Ok(bytes)
}

fn read_state_ref(
    repo_root: &Path,
    relative_ref: &str,
    limit: usize,
) -> Result<Vec<u8>, CharterApprovalRefusalV1> {
    read_safe_file(
        repo_root,
        &repo_root.join(".handbook/state").join(relative_ref),
        limit,
    )
}

fn read_safe_file(
    repo_root: &Path,
    path: &Path,
    limit: usize,
) -> Result<Vec<u8>, CharterApprovalRefusalV1> {
    let relative = path
        .strip_prefix(repo_root)
        .map_err(|_| durability("approval authority path escapes repository root"))?;
    let relative = relative
        .to_str()
        .ok_or_else(|| durability("approval authority path is not UTF-8"))?
        .replace('\\', "/");
    let workspace = CanonicalWorkspace::new(repo_root);
    let normalized = workspace
        .normalize_repo_relative(&relative)
        .map_err(|_| durability("approval authority path is not normalized"))?;
    let file = workspace
        .trusted_read_strict(&normalized)
        .map_err(|_| durability("approval authority is not a safe regular file"))?;
    let (bytes, exceeded) = file
        .read_bytes_bounded(limit)
        .map_err(|_| durability("approval authority read failed"))?;
    if exceeded {
        return Err(durability("approval authority exceeds its admitted bound"));
    }
    Ok(bytes)
}

fn read_canonical_bytes(repo_root: &Path) -> Result<Option<Vec<u8>>, CharterApprovalRefusalV1> {
    let workspace = CanonicalWorkspace::new(repo_root);
    let relative = workspace
        .normalize_repo_relative(CANONICAL_CHARTER_REF)
        .map_err(|_| durability("canonical Charter path is not normalized"))?;
    match workspace.trusted_read_strict(&relative) {
        Ok(file) => {
            let (bytes, exceeded) = file
                .read_bytes_bounded(MAX_CANONICAL_BYTES)
                .map_err(|_| durability("canonical Charter read failed"))?;
            if exceeded {
                return Err(durability("canonical Charter exceeds its admitted bound"));
            }
            Ok(Some(bytes))
        }
        Err(RepoRelativeFileAccessError::Missing(_)) => Ok(None),
        Err(_) => Err(durability("canonical Charter is not a safe regular file")),
    }
}

fn write_new_durable(path: &Path, bytes: &[u8]) -> Result<(), CharterApprovalRefusalV1> {
    let mut file = create_new_file(path)
        .map_err(|_| durability("approval authority file create-new failed"))?;
    file.write_all(bytes)
        .map_err(|_| durability("approval authority file write failed"))?;
    file.sync_all()
        .map_err(|_| durability("approval authority file flush failed"))
}

fn approval_commit_marker(intent_bytes: &[u8]) -> Vec<u8> {
    format!("sha256:{:x}\n", Sha256::digest(intent_bytes)).into_bytes()
}

fn finalize_pending_directory(root: &Path, pending: &Path) -> Result<(), CharterApprovalRefusalV1> {
    let name = pending
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| durability("approval pending directory name is invalid"))?;
    let committed = root.join(format!("{}.committed", name.trim_end_matches(".pending")));
    if committed.exists() {
        return Err(durability(
            "approval committed journal already exists during recovery",
        ));
    }
    fs::rename(pending, committed)
        .map_err(|_| durability("approval journal finalization failed"))?;
    sync_directory(root).map_err(map_lineage_refusal)
}

fn durability(message: impl Into<String>) -> CharterApprovalRefusalV1 {
    refused(
        CharterApprovalRefusalCodeV1::DurabilityViolation,
        message,
        false,
        "repair exact approval transaction authority before retry",
    )
}

fn validate_candidate_currentness(
    decisions: &ResolvedProfileDecisions,
    candidate: &Value,
) -> Result<(), CharterApprovalRefusalV1> {
    let exact = candidate.get("schema_id").and_then(Value::as_str)
        == Some("handbook.artifact-candidate")
        && candidate.get("schema_version").and_then(Value::as_str) == Some("1.3")
        && candidate.get("target_instance_id").and_then(Value::as_str) == Some("project_authority")
        && candidate.get("target_kind_ref").and_then(Value::as_str)
            == Some("handbook.artifact-kind.project-authority@1.1.0")
        && candidate.get("target_schema_ref").and_then(Value::as_str)
            == Some("handbook.schemas.artifacts.project-authority@1.1.0")
        && candidate
            .get("required_approval_policy_ref")
            .and_then(Value::as_str)
            == Some(APPROVAL_POLICY_REF)
        && candidate.get("profile_ref").and_then(Value::as_str)
            == Some(decisions.profile_ref().as_str())
        && candidate
            .get("resolved_profile_fingerprint")
            .and_then(Value::as_str)
            == Some(decisions.profile_definition_fingerprint().as_str());
    if !exact {
        return Err(refused(
            CharterApprovalRefusalCodeV1::LineageViolation,
            "candidate does not bind the current exact Charter profile, kind, schema, and policy",
            false,
            "re-finalize the Charter candidate against the current shipped profile",
        ));
    }
    Ok(())
}

fn observe_candidate_basis(
    repo_root: &Path,
    decisions: &ResolvedProfileDecisions,
    candidate: &Value,
) -> Result<CanonicalBasisObservation, CharterApprovalRefusalV1> {
    let workspace = CanonicalWorkspace::new(repo_root);
    let relative = workspace
        .normalize_repo_relative(CANONICAL_CHARTER_REF)
        .map_err(|_| unsafe_filesystem("selected Charter path is not normalized"))?;
    let current = match workspace.trusted_read_strict(&relative) {
        Ok(file) => {
            let (bytes, exceeded) = file.read_bytes_bounded(MAX_CANONICAL_BYTES).map_err(|_| {
                unsafe_filesystem("selected Charter authority could not be read safely")
            })?;
            if exceeded {
                return Err(refused(
                    CharterApprovalRefusalCodeV1::DurabilityViolation,
                    "selected Charter authority exceeds the admitted bound",
                    false,
                    "repair the selected Charter authority before approval",
                ));
            }
            Some(bytes)
        }
        Err(RepoRelativeFileAccessError::Missing(_)) => None,
        Err(_) => {
            return Err(unsafe_filesystem(
                "selected Charter authority is not a safe regular file",
            ))
        }
    };
    let basis = candidate
        .get("basis_artifact_fingerprint")
        .ok_or_else(|| lineage("candidate basis field is absent"))?;
    match (basis, current) {
        (Value::Null, None) => Ok(CanonicalBasisObservation {
            bytes: None,
            _fingerprint: None,
            record: None,
        }),
        (Value::String(expected), Some(bytes)) => {
            if DefinitionFingerprint::parse(expected).is_err() {
                return Err(lineage("candidate basis fingerprint is invalid"));
            }
            let fingerprint = DefinitionFingerprint::from_bytes(&bytes).to_string();
            if &fingerprint != expected {
                return Err(refused(
                    CharterApprovalRefusalCodeV1::StaleBasis,
                    "candidate basis differs from current canonical Charter authority",
                    true,
                    "re-run Charter intake from the current canonical fingerprint",
                ));
            }
            let record = parse_canonical_charter(decisions, &bytes).map_err(|_| {
                refused(
                    CharterApprovalRefusalCodeV1::LineageViolation,
                    "current canonical Charter authority is invalid",
                    false,
                    "repair canonical Charter authority before amendment approval",
                )
            })?;
            Ok(CanonicalBasisObservation {
                bytes: Some(bytes),
                _fingerprint: Some(fingerprint),
                record: Some(record),
            })
        }
        (Value::Null, Some(_)) | (Value::String(_), None) => Err(refused(
            CharterApprovalRefusalCodeV1::StaleBasis,
            "candidate create/amendment basis does not match current canonical presence",
            true,
            "re-finalize the candidate against current canonical authority",
        )),
        _ => Err(lineage(
            "candidate basis must be null or an exact lowercase SHA-256 fingerprint",
        )),
    }
}

fn required_approval_pairs(
    authority: &RetainedApproverRegistryObservationV1,
    current: Option<&CanonicalCharter>,
) -> Result<Vec<ApproverAuthorityPairV1>, CharterApprovalRefusalV1> {
    let mut pairs = Vec::new();
    if let Some(current) = current {
        for approval_class in &current.governance.required_approvals {
            for authority_ref in &current.governance.decision_authority {
                pairs.push(ApproverAuthorityPairV1 {
                    approval_class: approval_class.clone(),
                    authority_ref: authority_ref.clone(),
                });
            }
        }
    } else {
        let rows = authority
            .state
            .get("initial_charter_quorum")
            .and_then(Value::as_array)
            .ok_or_else(|| lineage("committed registry initial Charter quorum is absent"))?;
        for row in rows {
            pairs.push(ApproverAuthorityPairV1 {
                approval_class: required_string(row, "approval_class")?.to_owned(),
                authority_ref: required_string(row, "authority_ref")?.to_owned(),
            });
        }
    }
    pairs.sort_by(|left, right| {
        left.approval_class
            .cmp(&right.approval_class)
            .then_with(|| left.authority_ref.cmp(&right.authority_ref))
    });
    let before = pairs.len();
    pairs.dedup();
    if pairs.is_empty() || pairs.len() != before || pairs.len() > 64 {
        return Err(lineage(
            "required approval authority pairs are empty, duplicated, or out of bounds",
        ));
    }
    Ok(pairs)
}

fn resolve_accepted_waivers(
    lineage_store: &TrustedLineageStoreV1,
    waiver_refs: &[String],
    candidate_fingerprint: &str,
) -> Result<Vec<AcceptedWaiver>, CharterApprovalRefusalV1> {
    let mut refs = BTreeSet::new();
    let mut fingerprints = BTreeSet::new();
    let mut accepted = Vec::with_capacity(waiver_refs.len());
    for waiver_ref in waiver_refs {
        if !refs.insert(waiver_ref.clone()) {
            return Err(refused(
                CharterApprovalRefusalCodeV1::InvalidRequest,
                "duplicate waiver ref is forbidden",
                false,
                "supply each accepted waiver ref exactly once",
            ));
        }
        let fingerprint = fingerprint_from_ref(waiver_ref, "waiver")?;
        if !fingerprints.insert(fingerprint.clone()) {
            return Err(refused(
                CharterApprovalRefusalCodeV1::InvalidRequest,
                "duplicate waiver fingerprint is forbidden",
                false,
                "supply one unique ref for each accepted waiver fingerprint",
            ));
        }
        let bytes = lineage_store
            .read_record(LineageRecordClassV1::Waiver, waiver_ref, &fingerprint)
            .map_err(map_lineage_refusal)?;
        let waiver = parse_schema_json(&bytes).map_err(|_| lineage("waiver is not closed JSON"))?;
        if waiver.get("candidate_fingerprint").and_then(Value::as_str)
            != Some(candidate_fingerprint)
        {
            return Err(refused(
                CharterApprovalRefusalCodeV1::AuthorizationRefused,
                "accepted waiver does not bind the exact candidate",
                false,
                "select only waivers issued for the exact candidate fingerprint",
            ));
        }
        accepted.push(AcceptedWaiver {
            waiver_ref: waiver_ref.clone(),
            waiver_fingerprint: fingerprint,
        });
    }
    accepted.sort_by(|left, right| {
        left.waiver_ref
            .as_bytes()
            .cmp(right.waiver_ref.as_bytes())
            .then_with(|| {
                left.waiver_fingerprint
                    .as_bytes()
                    .cmp(right.waiver_fingerprint.as_bytes())
            })
    });
    Ok(accepted)
}

fn eligible_credentials<'a>(
    authority: &'a RetainedApproverRegistryObservationV1,
    requested_pair: &ApproverAuthorityPairV1,
    current_required_pairs: &[ApproverAuthorityPairV1],
) -> Result<Vec<&'a CommittedApproverCredentialV1>, CharterApprovalRefusalV1> {
    let mut coverage_pairs = current_required_pairs.to_vec();
    let initial = required_approval_pairs(authority, None)?;
    coverage_pairs.extend(initial);
    coverage_pairs.push(ApproverAuthorityPairV1 {
        approval_class: "registry_admin".to_owned(),
        authority_ref: "repository_registry_admin".to_owned(),
    });
    coverage_pairs.sort_by(|left, right| {
        left.approval_class
            .cmp(&right.approval_class)
            .then_with(|| left.authority_ref.cmp(&right.authority_ref))
    });
    coverage_pairs.dedup();

    let candidates = authority
        .credentials
        .iter()
        .map(|credential| CredentialSelectionCandidateV1 {
            credential_id: credential.credential_id.clone(),
            active: credential.active,
            sequence: credential.use_sequence,
            algorithm_es256: true,
            covers_required_pair: covers(credential, requested_pair)
                && final_use_preserves_coverage(
                    credential,
                    &authority.credentials,
                    &coverage_pairs,
                ),
        })
        .collect::<Vec<_>>();
    let selected = select_eligible_credentials(&candidates).map_err(map_authenticator_refusal)?;
    let mut credentials = Vec::with_capacity(selected.len());
    for credential_id in selected {
        let credential = authority
            .credentials
            .iter()
            .find(|credential| credential.credential_id == credential_id)
            .ok_or_else(|| lineage("selected credential is absent from retained registry"))?;
        credentials.push(credential);
    }
    Ok(credentials)
}

fn final_use_preserves_coverage(
    credential: &CommittedApproverCredentialV1,
    credentials: &[CommittedApproverCredentialV1],
    required_pairs: &[ApproverAuthorityPairV1],
) -> bool {
    if credential.use_sequence != 4095 {
        return true;
    }
    required_pairs.iter().all(|pair| {
        credentials.iter().any(|other| {
            other.credential_id_hash != credential.credential_id_hash
                && other.active
                && other.use_sequence < 4096
                && covers(other, pair)
        })
    })
}

fn covers(credential: &CommittedApproverCredentialV1, pair: &ApproverAuthorityPairV1) -> bool {
    credential.approval_mappings.iter().any(|mapping| {
        mapping.approval_class == pair.approval_class && mapping.authority_ref == pair.authority_ref
    })
}

fn build_approval_challenge(
    request: &CharterApprovalRequestV1,
    candidate_fingerprint: &str,
    basis: &Value,
    authority: &RetainedApproverRegistryObservationV1,
    accepted_waivers: &[AcceptedWaiver],
    nonce: [u8; 32],
) -> Result<ApprovalChallenge, CharterApprovalRefusalV1> {
    let operation = match basis {
        Value::Null => "initial_charter_approval",
        Value::String(value) if DefinitionFingerprint::parse(value).is_ok() => "amendment_approval",
        _ => {
            return Err(lineage(
                "candidate basis cannot select an approval challenge branch",
            ))
        }
    };
    let mut waiver_fingerprints = accepted_waivers
        .iter()
        .map(|waiver| waiver.waiver_fingerprint.clone())
        .collect::<Vec<_>>();
    waiver_fingerprints.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    if waiver_fingerprints
        .windows(2)
        .any(|pair| pair[0] >= pair[1])
    {
        return Err(refused(
            CharterApprovalRefusalCodeV1::InvalidRequest,
            "accepted waiver fingerprints are not unique strict ascending values",
            false,
            "supply each accepted candidate waiver exactly once",
        ));
    }
    let challenge = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "accepted_waiver_fingerprints": waiver_fingerprints,
        "approval_class": request.approval_class,
        "approver_registry_state_fingerprint": authority.state_fingerprint,
        "authority_ref": request.authority_ref,
        "basis_artifact_fingerprint": basis,
        "candidate_fingerprint": candidate_fingerprint,
        "candidate_ref": request.candidate_ref,
        "nonce_base64": BASE64_STANDARD.encode(nonce),
        "operation": operation,
        "operation_id": request.operation_id,
        "registry_head_transition_fingerprint": authority.head_transition_fingerprint,
        "repository_identity_fingerprint": required_string(
            &authority.state,
            "repository_identity_fingerprint"
        )?,
        "schema_id": "handbook.authenticator-challenge",
        "schema_version": "1.0"
    });
    let bytes = serde_json_canonicalizer::to_vec(&challenge).map_err(|_| {
        refused(
            CharterApprovalRefusalCodeV1::TransactionConflict,
            "approval challenge could not be RFC 8785 canonicalized",
            true,
            "retry the complete approval operation",
        )
    })?;
    if bytes.len() > 16_384 {
        return Err(refused(
            CharterApprovalRefusalCodeV1::InvalidRequest,
            "approval challenge exceeds the admitted bound",
            false,
            "shorten bounded caller selectors and operation identity",
        ));
    }
    Ok(ApprovalChallenge {
        fingerprint: DefinitionFingerprint::from_bytes(&bytes).to_string(),
        nonce_fingerprint: DefinitionFingerprint::from_bytes(&nonce).to_string(),
        bytes,
    })
}

#[allow(
    clippy::nonminimal_bool,
    reason = "the explicit zero/zero, zero/nonzero, and monotonic branches mirror the counter protocol"
)]
fn validate_counter(prior: u32, current: u32) -> Result<(), CharterApprovalRefusalV1> {
    if (prior == 0 && current == 0) || (prior == 0 && current > 0) || current > prior {
        Ok(())
    } else {
        Err(refused(
            CharterApprovalRefusalCodeV1::AuthorizationRefused,
            "authenticator sign counter is equal, decreasing, or reset after a nonzero value",
            false,
            "replace or diagnose the authenticator before retry",
        ))
    }
}

fn fingerprint_from_ref(
    relative_ref: &str,
    record_prefix: &str,
) -> Result<String, CharterApprovalRefusalV1> {
    let partition = match record_prefix {
        "candidate" => "candidates",
        "waiver" => "waivers",
        _ => return Err(lineage("unsupported content-addressed reference class")),
    };
    let prefix = format!("{partition}/{record_prefix}_");
    let hex = relative_ref
        .strip_prefix(&prefix)
        .and_then(|value| value.strip_suffix(".json"))
        .filter(|value| {
            value.len() == 64
                && value
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        })
        .ok_or_else(|| {
            refused(
                CharterApprovalRefusalCodeV1::InvalidRequest,
                "content-addressed reference is not exact",
                false,
                "supply the exact repository-relative content-addressed record ref",
            )
        })?;
    Ok(format!("sha256:{hex}"))
}

fn required_string<'a>(value: &'a Value, field: &str) -> Result<&'a str, CharterApprovalRefusalV1> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|text| !text.is_empty())
        .ok_or_else(|| lineage(format!("required authority field `{field}` is absent")))
}

fn map_port_refusal(error: NativeAuthenticatorPortErrorV1) -> CharterApprovalRefusalV1 {
    match error {
        NativeAuthenticatorPortErrorV1::Unavailable => refused(
            CharterApprovalRefusalCodeV1::AuthenticatorUnavailable,
            "native authenticator API is unavailable",
            false,
            "use a supported native FIDO2 CTAP2.1 platform and authenticator",
        ),
        NativeAuthenticatorPortErrorV1::Transport => refused(
            CharterApprovalRefusalCodeV1::TransactionConflict,
            "native authenticator transport failed before a complete status was received",
            true,
            "retry the complete operation with a fresh challenge",
        ),
    }
}

fn map_authenticator_refusal(error: AuthenticatorErrorV1) -> CharterApprovalRefusalV1 {
    if let AuthenticatorErrorV1::Refused(ref status) = error {
        let code = match status.code {
            CtapRefusalCodeV1::InvalidRequest => CharterApprovalRefusalCodeV1::InvalidRequest,
            CtapRefusalCodeV1::TransactionConflict => {
                CharterApprovalRefusalCodeV1::TransactionConflict
            }
            CtapRefusalCodeV1::AuthenticatorUserTimeout => {
                CharterApprovalRefusalCodeV1::AuthenticatorUserTimeout
            }
            CtapRefusalCodeV1::AuthenticatorResourceExhausted => {
                CharterApprovalRefusalCodeV1::AuthenticatorResourceExhausted
            }
            CtapRefusalCodeV1::AuthorizationRefused => {
                CharterApprovalRefusalCodeV1::AuthorizationRefused
            }
            CtapRefusalCodeV1::AuthenticatorSecurityBlocked => {
                CharterApprovalRefusalCodeV1::AuthenticatorSecurityBlocked
            }
            CtapRefusalCodeV1::AuthenticatorUserCancelled => {
                CharterApprovalRefusalCodeV1::AuthenticatorUserCancelled
            }
            CtapRefusalCodeV1::AuthenticatorUserVerificationRetry => {
                CharterApprovalRefusalCodeV1::AuthenticatorUserVerificationRetry
            }
            CtapRefusalCodeV1::AuthenticatorUnknownError => {
                CharterApprovalRefusalCodeV1::AuthenticatorUnknownError
            }
        };
        return refused(
            code,
            error.to_string(),
            status.retryable,
            status.next_action,
        );
    }
    refused(
        CharterApprovalRefusalCodeV1::AuthorizationRefused,
        error.to_string(),
        false,
        "perform a new assertion with exact committed authority",
    )
}

fn map_observation_refusal(error: ApproverRegistryObservationErrorV1) -> CharterApprovalRefusalV1 {
    let code = match error.kind() {
        ApproverRegistryObservationErrorKindV1::AuthorityAbsent => {
            CharterApprovalRefusalCodeV1::AuthorityAbsent
        }
        ApproverRegistryObservationErrorKindV1::DurabilityViolation => {
            CharterApprovalRefusalCodeV1::DurabilityViolation
        }
        ApproverRegistryObservationErrorKindV1::UnsafeFilesystem
        | ApproverRegistryObservationErrorKindV1::LineageViolation => {
            CharterApprovalRefusalCodeV1::LineageViolation
        }
    };
    refused(
        code,
        error.to_string(),
        false,
        "repair or bootstrap exact committed approver authority before retry",
    )
}

fn map_lineage_refusal(error: LineageStoreErrorV1) -> CharterApprovalRefusalV1 {
    refused(
        CharterApprovalRefusalCodeV1::LineageViolation,
        error.detail().to_owned(),
        false,
        "repair exact append-only Charter lineage before retry",
    )
}

fn lineage(message: impl Into<String>) -> CharterApprovalRefusalV1 {
    refused(
        CharterApprovalRefusalCodeV1::LineageViolation,
        message,
        false,
        "repair exact append-only Charter lineage before retry",
    )
}

fn unsafe_filesystem(message: impl Into<String>) -> CharterApprovalRefusalV1 {
    refused(
        CharterApprovalRefusalCodeV1::DurabilityViolation,
        message,
        false,
        "repair the unsafe repository authority path before retry",
    )
}

#[allow(clippy::too_many_arguments)]
fn build_approval_records(
    request: &CharterApprovalRequestV1,
    candidate: &Value,
    candidate_fingerprint: &str,
    authority: &RetainedApproverRegistryObservationV1,
    credential: &CommittedApproverCredentialV1,
    accepted_waivers: &[AcceptedWaiver],
    challenge: &ApprovalChallenge,
    decoded: &GetAssertionResponseV1,
    client_data_hash: [u8; 32],
    prior_use_head_bytes: Vec<u8>,
) -> Result<ApprovalRecords, CharterApprovalRefusalV1> {
    let credential_hash = DefinitionFingerprint::from_bytes(&decoded.credential_id).to_string();
    if credential_hash != credential.credential_id_hash {
        return Err(lineage(
            "decoded credential bytes and retained credential hash disagree",
        ));
    }
    let rp_hash = fingerprint_digest(
        decoded.authenticator_data[..32]
            .try_into()
            .expect("verified authenticator data has an RP hash"),
    );
    let response = finalized_record(
        json!({
            "schema_id": "handbook.authenticator-get-assertion-response",
            "schema_version": "1.0",
            "protocol_id": "fido2-ctap2.1-native",
            "status_byte": 0,
            "credential_descriptor": {
                "type": "public-key",
                "credential_id_base64": BASE64_STANDARD.encode(&decoded.credential_id),
                "credential_id_hash": credential.credential_id_hash
            },
            "rp_id_hash": rp_hash,
            "flags_byte": decoded.flags_byte,
            "user_present": true,
            "user_verified": true,
            "sign_count": decoded.sign_count,
            "authenticator_data_base64": BASE64_STANDARD.encode(decoded.authenticator_data),
            "algorithm": "ES256",
            "signature_encoding": "asn1-der-ecdsa",
            "signature_base64": BASE64_STANDARD.encode(&decoded.signature_der),
            "raw_response_base64": BASE64_STANDARD.encode(&decoded.raw_response),
            "attested_credential_data_included": decoded.attested_credential_data_included,
            "extensions_included": decoded.extensions_included
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
            "schema_id": "handbook.authenticator-assertion",
            "schema_version": "1.0",
            "protocol_id": "fido2-ctap2.1-native",
            "challenge_schema_ref": "handbook.schemas.security.authenticator-challenge@1.0.0",
            "challenge_jcs_base64": BASE64_STANDARD.encode(&challenge.bytes),
            "challenge_fingerprint": challenge.fingerprint,
            "credential_id_hash": credential.credential_id_hash,
            "rp_id": AUTHENTICATOR_RP_ID,
            "client_data_hash": fingerprint_digest(client_data_hash),
            "authenticator_data_base64": BASE64_STANDARD.encode(decoded.authenticator_data),
            "signature_base64": BASE64_STANDARD.encode(&decoded.signature_der),
            "signed_preimage_sha256": DefinitionFingerprint::from_bytes(&signed_preimage).to_string(),
            "algorithm": "ES256",
            "user_present": true,
            "user_verified": true,
            "sign_count": decoded.sign_count,
            "verified_at_utc": current_audit_utc(),
            "decoded_response_schema_ref": "handbook.schemas.security.authenticator-get-assertion-response@1.0.0",
            "decoded_response_ref": response.relative_ref,
            "decoded_response_fingerprint": response.fingerprint,
            "flags_byte": decoded.flags_byte,
            "attested_credential_data_included": decoded.attested_credential_data_included,
            "extensions_included": decoded.extensions_included
        }),
        Some("assertion_id"),
        "assertion_fingerprint",
        "authenticator-assertion",
        "authenticator-assertions",
        &["verified_at_utc"],
    )?;
    let accepted = accepted_waivers
        .iter()
        .map(|waiver| {
            json!({
                "waiver_ref": waiver.waiver_ref,
                "waiver_fingerprint": waiver.waiver_fingerprint
            })
        })
        .collect::<Vec<_>>();
    let approval = finalized_record(
        json!({
            "schema_id": "handbook.artifact-approval-record",
            "schema_version": "1.1",
            "candidate_ref": request.candidate_ref,
            "candidate_fingerprint": candidate_fingerprint,
            "approval_policy_ref": APPROVAL_POLICY_REF,
            "decision": "approved",
            "actor_ref": format!("credential:{}", credential.credential_id_hash),
            "authority_ref": request.authority_ref,
            "conditions": [],
            "decided_at_utc": current_audit_utc(),
            "basis_artifact_fingerprint": candidate
                .get("basis_artifact_fingerprint")
                .cloned()
                .unwrap_or(Value::Null),
            "approval_class": request.approval_class,
            "approver_registry_state_ref": authority.state_ref,
            "approver_registry_state_fingerprint": authority.state_fingerprint,
            "registry_head_transition_ref": authority.head_transition_ref,
            "registry_head_transition_fingerprint": authority.head_transition_fingerprint,
            "authenticator_assertion_ref": assertion.relative_ref,
            "authenticator_assertion_fingerprint": assertion.fingerprint,
            "accepted_waivers": accepted
        }),
        Some("approval_id"),
        "approval_fingerprint",
        "approval",
        "approvals",
        &[],
    )?;
    let next_sequence = credential.use_sequence.checked_add(1).ok_or_else(|| {
        refused(
            CharterApprovalRefusalCodeV1::AuthorizationRefused,
            "authenticator use sequence is exhausted",
            false,
            "replace the exhausted credential",
        )
    })?;
    if next_sequence > 4096 {
        return Err(refused(
            CharterApprovalRefusalCodeV1::AuthorizationRefused,
            "authenticator use sequence is exhausted",
            false,
            "replace the exhausted credential",
        ));
    }
    let result_use_head = finalized_record(
        json!({
            "$schema": "handbook.schemas.security.authenticator-use-head@1.0.0",
            "schema_version": "1.0",
            "credential_id_hash": credential.credential_id_hash,
            "sequence": next_sequence,
            "last_assertion_ref": assertion.relative_ref,
            "last_assertion_fingerprint": assertion.fingerprint,
            "last_challenge_fingerprint": challenge.fingerprint,
            "last_nonce_sha256": challenge.nonce_fingerprint,
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
    let approval_id = approval
        .relative_ref
        .strip_prefix("approvals/")
        .and_then(|value| value.strip_suffix(".json"))
        .ok_or_else(|| lineage("approval record identity is invalid"))?;
    let use_transition = finalized_record(
        json!({
            "$schema": "handbook.schemas.security.authenticator-use-transition@1.0.0",
            "schema_version": "1.0",
            "credential_id_hash": credential.credential_id_hash,
            "sequence": next_sequence,
            "prior_head_fingerprint": credential.use_head_fingerprint,
            "result_head_fingerprint": result_use_head.fingerprint,
            "assertion_ref": assertion.relative_ref,
            "assertion_fingerprint": assertion.fingerprint,
            "challenge_fingerprint": challenge.fingerprint,
            "nonce_sha256": challenge.nonce_fingerprint,
            "prior_sign_count": credential.sign_count,
            "sign_count": decoded.sign_count,
            "consumer_kind": "approval",
            "consumer_id": approval_id
        }),
        None,
        "transition_fingerprint",
        "authenticator-use-transition",
        "authenticator-use-transitions",
        &[],
    )?;
    Ok(ApprovalRecords {
        operation_id: request.operation_id.clone(),
        response,
        assertion,
        use_transition,
        approval,
        prior_use_head_ref: credential.use_head_ref.clone(),
        prior_use_head_fingerprint: credential.use_head_fingerprint.clone(),
        prior_use_head_bytes,
        result_use_head,
    })
}

fn finalized_record(
    mut value: Value,
    id_field: Option<&str>,
    fingerprint_field: &str,
    id_prefix: &str,
    partition: &str,
    audit_only_fields: &[&str],
) -> Result<BuiltRecord, CharterApprovalRefusalV1> {
    let mut fingerprint_preimage = value.clone();
    let object = fingerprint_preimage
        .as_object_mut()
        .ok_or_else(|| lineage("runtime record root is not an object"))?;
    for field in audit_only_fields {
        object.remove(*field);
    }
    let fingerprint = DefinitionFingerprint::from_json_value(&fingerprint_preimage)
        .map_err(|_| lineage("runtime record fingerprint preimage is invalid"))?
        .to_string();
    let hex = fingerprint
        .strip_prefix("sha256:")
        .expect("fingerprint is normalized");
    let id = format!("{id_prefix}_{hex}");
    let object = value
        .as_object_mut()
        .ok_or_else(|| lineage("runtime record root is not an object"))?;
    if let Some(id_field) = id_field {
        object.insert(id_field.to_owned(), Value::String(id.clone()));
    }
    object.insert(
        fingerprint_field.to_owned(),
        Value::String(fingerprint.clone()),
    );
    let bytes = serde_json_canonicalizer::to_vec(&value)
        .map_err(|_| lineage("runtime record could not be RFC 8785 canonicalized"))?;
    if bytes.len() > MAX_RECORD_BYTES {
        return Err(lineage("runtime record exceeds the admitted bound"));
    }
    Ok(BuiltRecord {
        relative_ref: format!("{partition}/{id}.json"),
        fingerprint,
        bytes,
    })
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
        shifted / 146_097
    } else {
        (shifted - 146_096) / 146_097
    };
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    (year, month, day)
}

fn validate_caller_intent(
    request: &CharterApprovalRequestV1,
) -> Result<(), CharterApprovalRefusalV1> {
    let valid = bounded_nonempty(&request.operation_id, MAX_OPERATION_ID_BYTES)
        && bounded_nonempty(&request.candidate_ref, MAX_REFERENCE_BYTES)
        && bounded_nonempty(&request.approval_class, MAX_SELECTOR_BYTES)
        && bounded_nonempty(&request.authority_ref, MAX_SELECTOR_BYTES)
        && request.accepted_waiver_refs.len() <= MAX_ACCEPTED_WAIVERS
        && request
            .accepted_waiver_refs
            .iter()
            .all(|reference| bounded_nonempty(reference, MAX_REFERENCE_BYTES));
    if !valid {
        return Err(refusal(
            request.operation_id.clone(),
            CharterApprovalRefusalCodeV1::InvalidRequest,
            "Charter approval caller intent violates its closed bounds",
            false,
            "supply one bounded operation ID, candidate ref, approval class, and authority ref",
        ));
    }
    Ok(())
}

fn bounded_nonempty(value: &str, maximum: usize) -> bool {
    !value.is_empty() && value.len() <= maximum && !value.contains('\0')
}

fn refused(
    code: CharterApprovalRefusalCodeV1,
    message: impl Into<String>,
    retryable: bool,
    next_action: impl Into<String>,
) -> CharterApprovalRefusalV1 {
    refusal(String::new(), code, message, retryable, next_action)
}

fn fingerprint_digest(digest: [u8; 32]) -> String {
    let mut value = String::with_capacity(71);
    value.push_str("sha256:");
    for byte in digest {
        use std::fmt::Write as _;
        write!(&mut value, "{byte:02x}").expect("writing to a String cannot fail");
    }
    value
}

fn refusal(
    operation_id: String,
    code: CharterApprovalRefusalCodeV1,
    message: impl Into<String>,
    retryable: bool,
    next_action: impl Into<String>,
) -> CharterApprovalRefusalV1 {
    CharterApprovalRefusalV1 {
        operation_id,
        code,
        message: message.into(),
        retryable,
        next_actions: vec![next_action.into()],
        response_ref: None,
        response_fingerprint: None,
        assertion_ref: None,
        assertion_fingerprint: None,
        use_transition_ref: None,
        use_transition_fingerprint: None,
        approval_ref: None,
        approval_fingerprint: None,
        result_use_head_ref: None,
        result_use_head_fingerprint: None,
        changed_paths: Vec::new(),
    }
}

fn _port_error_is_closed(error: NativeAuthenticatorPortErrorV1) -> CharterApprovalRefusalCodeV1 {
    match error {
        NativeAuthenticatorPortErrorV1::Unavailable => {
            CharterApprovalRefusalCodeV1::AuthenticatorUnavailable
        }
        NativeAuthenticatorPortErrorV1::Transport => {
            CharterApprovalRefusalCodeV1::TransactionConflict
        }
    }
}

#[cfg(test)]
mod approval_journal_discovery_tests {
    use super::*;

    #[test]
    fn approval_counter_protocol_preserves_zero_and_monotonic_boundaries() {
        assert!(validate_counter(0, 0).is_ok());
        assert!(validate_counter(0, u32::MAX).is_ok());
        assert!(validate_counter(1, 2).is_ok());
        assert!(validate_counter(u32::MAX - 1, u32::MAX).is_ok());
        assert!(validate_counter(1, 1).is_err());
        assert!(validate_counter(2, 1).is_err());
        assert!(validate_counter(u32::MAX, 0).is_err());
    }

    fn fingerprint(byte: char) -> String {
        format!("sha256:{}", byte.to_string().repeat(64))
    }

    fn write_journal(
        repo_root: &Path,
        transaction_byte: char,
        credential_byte: char,
        suffix: &str,
        marker: bool,
    ) {
        let transaction_id = format!("approval_{}", transaction_byte.to_string().repeat(64));
        let credential_id_hash = fingerprint(credential_byte);
        let use_head_ref = format!(
            "authenticator-use-heads/authenticator-use-head_{}.json",
            credential_byte.to_string().repeat(64)
        );
        let intent = ApprovalIntentV1 {
            schema_id: "handbook.approval-transaction-intent".to_owned(),
            schema_version: "1.0".to_owned(),
            transaction_id: transaction_id.clone(),
            operation_id: format!("discover-{transaction_byte}"),
            candidate_ref: format!(
                "candidates/candidate_{}.json",
                transaction_byte.to_string().repeat(64)
            ),
            candidate_fingerprint: fingerprint(transaction_byte),
            approval_class: "Project owner approval".to_owned(),
            authority_ref: "Project owner".to_owned(),
            approver_registry_state_ref: "registry-states/registry-state_000.json".to_owned(),
            approver_registry_state_fingerprint: fingerprint('1'),
            registry_head_transition_ref: "registry-transitions/registry-transition_000.json"
                .to_owned(),
            registry_head_transition_fingerprint: fingerprint('2'),
            credential_id_hash,
            response_ref: "authenticator-get-assertion-responses/response.json".to_owned(),
            response_fingerprint: fingerprint('3'),
            response_bytes_fingerprint: fingerprint('3'),
            assertion_ref: "authenticator-assertions/assertion.json".to_owned(),
            assertion_fingerprint: fingerprint('4'),
            assertion_bytes_fingerprint: fingerprint('4'),
            use_transition_ref: "authenticator-use-transitions/transition.json".to_owned(),
            use_transition_fingerprint: fingerprint('5'),
            use_transition_bytes_fingerprint: fingerprint('5'),
            approval_ref: format!("approvals/{transaction_id}.json"),
            approval_fingerprint: fingerprint(transaction_byte),
            approval_bytes_fingerprint: fingerprint(transaction_byte),
            prior_use_head_ref: use_head_ref.clone(),
            prior_use_head_fingerprint: fingerprint('6'),
            prior_use_sequence: 0,
            prior_use_head_bytes_fingerprint: fingerprint('7'),
            result_use_head_ref: use_head_ref,
            result_use_head_fingerprint: fingerprint('8'),
            result_use_sequence: 1,
            result_use_head_bytes_fingerprint: fingerprint('9'),
        };
        let intent_bytes = serde_json_canonicalizer::to_vec(&intent).unwrap();
        let directory = repo_root
            .join(APPROVAL_TRANSACTION_ROOT)
            .join(format!("{transaction_id}{suffix}"));
        fs::create_dir_all(&directory).unwrap();
        fs::write(directory.join("intent.json"), &intent_bytes).unwrap();
        if marker {
            fs::write(
                directory.join("committed"),
                approval_commit_marker(&intent_bytes),
            )
            .unwrap();
        }
    }

    #[test]
    fn approval_use_journal_discovery_is_lexical_and_marker_exact() {
        let temp = tempfile::tempdir().unwrap();
        write_journal(temp.path(), 'b', 'b', ".pending", false);
        write_journal(temp.path(), 'a', 'a', ".committed", true);

        let observations = discover_approval_use_journals(temp.path()).unwrap();

        assert_eq!(observations.len(), 2);
        assert!(observations[0].journal_path < observations[1].journal_path);
        assert_eq!(
            observations[0].marker_state,
            ApprovalJournalMarkerStateV1::Exact
        );
        assert!(observations[0].finalized_directory);
        assert_eq!(
            observations[1].marker_state,
            ApprovalJournalMarkerStateV1::Absent
        );
        assert!(!observations[1].finalized_directory);
    }

    #[test]
    fn approval_use_journal_discovery_rejects_duplicate_credential_sequence() {
        let temp = tempfile::tempdir().unwrap();
        write_journal(temp.path(), 'a', 'c', ".pending", false);
        write_journal(temp.path(), 'b', 'c', ".pending", false);

        let error = discover_approval_use_journals(temp.path()).unwrap_err();

        assert_eq!(
            error.code,
            CharterApprovalRefusalCodeV1::DurabilityViolation
        );
    }
}
