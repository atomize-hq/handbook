use crate::canonical_repo_support::{CanonicalWorkspace, TrustedRepoFile};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
#[cfg(not(unix))]
use std::fs::OpenOptions;
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Component, Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) const GENERIC_ARTIFACT_OWNER_CONTRACT_REF: &str =
    "handbook.hcm-2-3.generic-artifact-owner-contract@1.0.0";
#[cfg(test)]
#[allow(dead_code)]
pub(crate) const GENERIC_ARTIFACT_LOCK_REF: &str =
    ".handbook/state/locks/generic-artifact-operations.lock";
pub(crate) const RETAINED_RESULT_SECONDS: i64 = 30 * 24 * 60 * 60;

const MAX_RECORD_BYTES: usize = 1024 * 1024;
const MAX_TRANSACTION_BYTES: u64 = 16 * 1024 * 1024;
const MAX_TRANSACTION_FILES: usize = 16;
const MAX_PENDING_PER_FAMILY: usize = 1024;
const MAX_COMMITTED_PER_FAMILY: usize = 4096;
const MAX_ALL_PENDING_BYTES: u64 = 64 * 1024 * 1024;
const MAX_ALL_COMMITTED_BYTES: u64 = 256 * 1024 * 1024;
const MAX_SEMANTIC_RECORDS_PER_FAMILY_INSTANCE: usize = 4096;
const MAX_CLOSURE_RECORDS_PER_FAMILY_INSTANCE: usize = 8192;
const MAX_INSTANCE_STORE_BYTES: u64 = 256 * 1024 * 1024;
const MAX_DIRECTORY_DEPTH: usize = 8;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum GenericArtifactOperationV1 {
    IntakeRecordAppend,
    ArtifactCandidateAppend,
    ArtifactCandidatePromote,
}

impl GenericArtifactOperationV1 {
    pub(crate) fn operation_id(self) -> &'static str {
        match self {
            Self::IntakeRecordAppend => "intake.record.append",
            Self::ArtifactCandidateAppend => "artifact.candidate.append",
            Self::ArtifactCandidatePromote => "artifact.candidate.promote",
        }
    }

    pub(crate) fn transaction_token(self) -> &'static str {
        match self {
            Self::IntakeRecordAppend => "intake_record_append",
            Self::ArtifactCandidateAppend => "artifact_candidate_append",
            Self::ArtifactCandidatePromote => "artifact_candidate_promote",
        }
    }

    pub(crate) fn transaction_family(self) -> &'static str {
        match self {
            Self::IntakeRecordAppend => "intake-records",
            Self::ArtifactCandidateAppend => "artifact-candidates",
            Self::ArtifactCandidatePromote => "artifact-promotions",
        }
    }

    fn from_operation_id(value: &str) -> Option<Self> {
        match value {
            "intake.record.append" => Some(Self::IntakeRecordAppend),
            "artifact.candidate.append" => Some(Self::ArtifactCandidateAppend),
            "artifact.candidate.promote" => Some(Self::ArtifactCandidatePromote),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenericAuthorityClassV1 {
    SubordinateClosure,
    SemanticRecord,
    CanonicalTruth,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum GenericInstallModeV1 {
    CreateNew,
    ReplaceIfCurrent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum GenericReceiptClassV1 {
    SemanticRecord,
    CanonicalTruth,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EstablishedRefusalCodeV1 {
    CanonicalSyntaxInvalid,
    StructuralValidationFailed,
    IntakeCoverageBlocked,
    StaleBasis,
    StaleCurrentArtifact,
    OperationIneligible,
    PublicationBasisConflict,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenericRefusalLayerV1 {
    CanonicalSyntax,
    Structural,
    Intake,
    Currentness,
    Eligibility,
    Publication,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EstablishedRefusalV1 {
    pub code: EstablishedRefusalCodeV1,
    pub layer: GenericRefusalLayerV1,
    pub expected_fingerprint: Option<String>,
    pub observed_fingerprint: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GenericMutationRequestV1 {
    pub(crate) repository_identity_fingerprint: String,
    pub(crate) owner_contract_subject_fingerprint: String,
    pub(crate) operation: GenericArtifactOperationV1,
    pub(crate) idempotency_key: String,
    pub(crate) request_subject: Value,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GenericPlannedOutputV1 {
    pub(crate) token: String,
    pub(crate) authority_class: GenericAuthorityClassV1,
    pub(crate) final_ref: String,
    pub(crate) bytes: Vec<u8>,
    pub(crate) output_fingerprint: String,
    pub(crate) install_mode: GenericInstallModeV1,
    pub(crate) receipt_class: Option<GenericReceiptClassV1>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GenericCommitPlanV1 {
    pub(crate) operation_context_fingerprint: String,
    pub(crate) expected_basis_fingerprint: Option<String>,
    pub(crate) sampled_finalized_at_utc: Option<String>,
    pub(crate) outputs: Vec<GenericPlannedOutputV1>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GenericRefusalPlanV1 {
    pub(crate) operation_context_fingerprint: String,
    pub(crate) expected_basis_fingerprint: Option<String>,
    pub(crate) refusal: EstablishedRefusalV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum GenericPlannedOutcomeV1 {
    Commit(GenericCommitPlanV1),
    Refuse(GenericRefusalPlanV1),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GenericEvaluationContextV1 {
    pub(crate) sampled_finalized_at_utc: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenericAuthoritativeOutputV1 {
    pub authority_class: GenericAuthorityClassV1,
    #[serde(rename = "ref")]
    pub relative_ref: String,
    pub fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenericDomainMutationResultV1 {
    pub schema_id: String,
    pub schema_version: String,
    pub operation_id: String,
    pub transaction_id: String,
    pub request_fingerprint: String,
    pub outcome: String,
    pub refusal: Option<EstablishedRefusalV1>,
    pub authoritative_outputs: Vec<GenericAuthoritativeOutputV1>,
    pub internal_transaction_evidence_ref: Option<String>,
    pub internal_transaction_evidence_fingerprint: Option<String>,
    pub result_fingerprint: String,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum GenericExecutionDispositionV1 {
    Committed,
    Refused,
    Replayed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenericMutationExecutionV1 {
    pub result: GenericDomainMutationResultV1,
    pub disposition: GenericExecutionDispositionV1,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GenericLineageStoreErrorKindV1 {
    UnsupportedPlatform,
    InvalidRequest,
    InvalidPlan,
    InvalidEstablishedRefusal,
    UnsafeReference,
    UnsafeFilesystem,
    BoundExceeded,
    IoFailure,
    IdempotencyConflict,
    IdempotencyExpired,
    BasisMismatch,
    InvalidTransactionIntent,
    InvalidCommitMarker,
    IncompleteStaging,
    ConflictingTransactionState,
    ConflictingRefusalState,
    RetainedResultMismatch,
    InjectedFault,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GenericLineageStoreErrorV1 {
    kind: GenericLineageStoreErrorKindV1,
    detail: String,
}

impl GenericLineageStoreErrorV1 {
    pub(crate) fn new(kind: GenericLineageStoreErrorKindV1, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn kind(&self) -> GenericLineageStoreErrorKindV1 {
        self.kind
    }

    pub(crate) fn detail(&self) -> &str {
        &self.detail
    }
}

#[allow(dead_code, clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GenericLineageFaultPointV1 {
    AfterEstablishedIntent,
    AfterPendingIntent,
    AfterVerified,
    AfterCanonicalScratch(usize),
    AfterInstall(usize),
    AfterInstalledVerification,
    AfterMarker,
    AfterEvidence,
    AfterResult,
    AfterLedger,
    AfterCommittedRename,
}

type FaultSelection = Option<GenericLineageFaultPointV1>;

#[cfg(test)]
type NativePathHook = Box<dyn FnOnce(&Path)>;

#[cfg(test)]
thread_local! {
    static AFTER_RETAINED_READ_HOOK: std::cell::RefCell<Option<(PathBuf, NativePathHook)>> =
        const { std::cell::RefCell::new(None) };
    static BEFORE_PUBLICATION_RENAME_HOOK: std::cell::RefCell<Option<(PathBuf, NativePathHook)>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn set_after_retained_read_hook_for_testing(
    target: PathBuf,
    hook: impl FnOnce(&Path) + 'static,
) {
    AFTER_RETAINED_READ_HOOK.with(|slot| *slot.borrow_mut() = Some((target, Box::new(hook))));
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn set_before_publication_rename_hook_for_testing(
    target: PathBuf,
    hook: impl FnOnce(&Path) + 'static,
) {
    BEFORE_PUBLICATION_RENAME_HOOK.with(|slot| *slot.borrow_mut() = Some((target, Box::new(hook))));
}

#[cfg(test)]
fn run_path_hook(
    slot: &'static std::thread::LocalKey<std::cell::RefCell<Option<(PathBuf, NativePathHook)>>>,
    path: &Path,
) {
    slot.with(|slot| {
        let matches = slot
            .borrow()
            .as_ref()
            .is_some_and(|(target, _)| target == path);
        if matches {
            let (_, hook) = slot.borrow_mut().take().expect("matching hook is present");
            hook(path);
        }
    });
}

#[cfg(test)]
fn run_after_retained_read_hook(path: &Path) {
    run_path_hook(&AFTER_RETAINED_READ_HOOK, path);
}

#[cfg(not(test))]
fn run_after_retained_read_hook(_path: &Path) {}

#[cfg(test)]
fn run_before_publication_rename_hook(path: &Path) {
    run_path_hook(&BEFORE_PUBLICATION_RENAME_HOOK, path);
}

#[cfg(not(test))]
fn run_before_publication_rename_hook(_path: &Path) {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FaultBoundary {
    EstablishedIntent,
    PendingIntent,
    Verified,
    CanonicalScratch(usize),
    Install(usize),
    InstalledVerification,
    Marker,
    Evidence,
    Result,
    Ledger,
    CommittedRename,
}

#[derive(Clone, Debug)]
pub(crate) struct GenericArtifactLineageStoreV1 {
    repo_root: PathBuf,
}

#[derive(Clone, Debug)]
struct DerivedRequest {
    repository_identity_fingerprint: String,
    owner_contract_subject_fingerprint: String,
    operation: GenericArtifactOperationV1,
    key_fingerprint: String,
    request_fingerprint: String,
    request_subject: Value,
}

struct RetainedBasisObservation {
    version_token: String,
    _guard: Option<TrustedRepoFile>,
}

struct PreparedReplacement {
    path: PathBuf,
    file: File,
    #[cfg(windows)]
    _directory_guards: Vec<File>,
    identity_token: String,
    version_token: String,
}

struct InstalledOutputGuard {
    path: PathBuf,
    expected_bytes: Vec<u8>,
    expected_identity_token: Option<String>,
    expected_version_token: Option<String>,
    _guard: Option<TrustedRepoFile>,
    replacement_guard: Option<PreparedReplacement>,
}

fn validate_and_derive_request(
    request: &GenericMutationRequestV1,
) -> Result<DerivedRequest, GenericLineageStoreErrorV1> {
    validate_sha256(&request.repository_identity_fingerprint)?;
    validate_sha256(&request.owner_contract_subject_fingerprint)?;
    if !(16..=128).contains(&request.idempotency_key.len())
        || !request
            .idempotency_key
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
    {
        return Err(error(
            GenericLineageStoreErrorKindV1::InvalidRequest,
            "idempotency key must be 16-128 ASCII alphanumeric, underscore, or hyphen bytes",
        ));
    }
    validate_request_subject(request.operation, &request.request_subject)?;
    let key_fingerprint = sha256_prefixed(&canonical_json(&json!({
        "key": request.idempotency_key
    }))?);
    let request_fingerprint = derive_request_fingerprint(
        &request.repository_identity_fingerprint,
        &request.owner_contract_subject_fingerprint,
        &key_fingerprint,
        &request.request_subject,
    )?;
    Ok(DerivedRequest {
        repository_identity_fingerprint: request.repository_identity_fingerprint.clone(),
        owner_contract_subject_fingerprint: request.owner_contract_subject_fingerprint.clone(),
        operation: request.operation,
        key_fingerprint,
        request_fingerprint,
        request_subject: request.request_subject.clone(),
    })
}

fn derive_request_fingerprint(
    repository_identity_fingerprint: &str,
    owner_contract_subject_fingerprint: &str,
    key_fingerprint: &str,
    request_subject: &Value,
) -> Result<String, GenericLineageStoreErrorV1> {
    Ok(sha256_prefixed(&canonical_json(&json!({
        "schema_id": "handbook.generic-domain-mutation-request",
        "schema_version": "1.0",
        "repository_identity_fingerprint": repository_identity_fingerprint,
        "owner_contract_ref": GENERIC_ARTIFACT_OWNER_CONTRACT_REF,
        "owner_contract_subject_fingerprint": owner_contract_subject_fingerprint,
        "domain_mutation_key_fingerprint": key_fingerprint,
        "request_subject": request_subject
    }))?))
}

fn derive_transaction_id(
    repository_identity_fingerprint: &str,
    owner_contract_subject_fingerprint: &str,
    operation: GenericArtifactOperationV1,
    key_fingerprint: &str,
    request_fingerprint: &str,
) -> Result<String, GenericLineageStoreErrorV1> {
    let preimage = json!({
        "repository_identity_fingerprint": repository_identity_fingerprint,
        "owner_contract_ref": GENERIC_ARTIFACT_OWNER_CONTRACT_REF,
        "owner_contract_subject_fingerprint": owner_contract_subject_fingerprint,
        "operation_id": operation.operation_id(),
        "domain_mutation_key_fingerprint": key_fingerprint,
        "request_fingerprint": request_fingerprint
    });
    Ok(format!(
        "{}_{}",
        operation.transaction_token(),
        hex_digest(&canonical_json(&preimage)?)
    ))
}

fn validate_request_subject(
    operation: GenericArtifactOperationV1,
    value: &Value,
) -> Result<(), GenericLineageStoreErrorV1> {
    let object = value.as_object().ok_or_else(|| {
        error(
            GenericLineageStoreErrorKindV1::InvalidRequest,
            "domain request subject must be one closed object",
        )
    })?;
    let expected: &[&str] = match operation {
        GenericArtifactOperationV1::IntakeRecordAppend => &[
            "operation_id",
            "kind_ref",
            "instance_id",
            "acquisition_mode",
            "expected_current_artifact_fingerprint",
            "coverage_input_fingerprint",
            "operation_context_fingerprint",
        ],
        GenericArtifactOperationV1::ArtifactCandidateAppend => &[
            "operation_id",
            "kind_ref",
            "instance_id",
            "intake_record_ref",
            "intake_record_fingerprint",
            "expected_candidate_fingerprint",
            "operation_context_fingerprint",
        ],
        GenericArtifactOperationV1::ArtifactCandidatePromote => &[
            "operation_id",
            "kind_ref",
            "instance_id",
            "candidate_ref",
            "candidate_fingerprint",
            "expected_current_artifact_fingerprint",
            "operation_context_fingerprint",
            "canonical_artifact_ref",
        ],
    };
    if object.len() != expected.len() || expected.iter().any(|key| !object.contains_key(*key)) {
        return Err(error(
            GenericLineageStoreErrorKindV1::InvalidRequest,
            "domain request subject does not have its operation-specific closed field set",
        ));
    }
    if object.get("operation_id").and_then(Value::as_str) != Some(operation.operation_id()) {
        return Err(error(
            GenericLineageStoreErrorKindV1::InvalidRequest,
            "domain request subject operation does not match typed routing",
        ));
    }
    validate_exact_ref(required_string(object, "kind_ref")?)?;
    validate_instance_id(required_string(object, "instance_id")?)?;
    validate_sha256(required_string(object, "operation_context_fingerprint")?)?;
    match operation {
        GenericArtifactOperationV1::IntakeRecordAppend => {
            if !matches!(
                required_string(object, "acquisition_mode")?,
                "guided_adaptive" | "express" | "agent_assisted"
            ) {
                return Err(error(
                    GenericLineageStoreErrorKindV1::InvalidRequest,
                    "acquisition mode is not closed",
                ));
            }
            validate_nullable_sha256(object.get("expected_current_artifact_fingerprint"))?;
            validate_sha256(required_string(object, "coverage_input_fingerprint")?)?;
        }
        GenericArtifactOperationV1::ArtifactCandidateAppend => {
            validate_safe_ref(required_string(object, "intake_record_ref")?)?;
            validate_sha256(required_string(object, "intake_record_fingerprint")?)?;
            validate_sha256(required_string(object, "expected_candidate_fingerprint")?)?;
        }
        GenericArtifactOperationV1::ArtifactCandidatePromote => {
            validate_safe_ref(required_string(object, "candidate_ref")?)?;
            validate_sha256(required_string(object, "candidate_fingerprint")?)?;
            validate_nullable_sha256(object.get("expected_current_artifact_fingerprint"))?;
            validate_safe_ref(required_string(object, "canonical_artifact_ref")?)?;
        }
    }
    Ok(())
}

fn validate_plan(
    request: &DerivedRequest,
    planned: &GenericPlannedOutcomeV1,
    now_utc: &str,
) -> Result<(), GenericLineageStoreErrorV1> {
    let expected_context = required_string(
        request
            .request_subject
            .as_object()
            .ok_or_else(|| invalid_plan("request subject is not an object"))?,
        "operation_context_fingerprint",
    )?;
    match planned {
        GenericPlannedOutcomeV1::Commit(plan) => {
            validate_sha256(&plan.operation_context_fingerprint)?;
            if plan.operation_context_fingerprint != expected_context {
                return Err(invalid_plan(
                    "commit plan context does not equal request authority",
                ));
            }
            if let Some(basis) = &plan.expected_basis_fingerprint {
                validate_sha256(basis)?;
            }
            match request.operation {
                GenericArtifactOperationV1::IntakeRecordAppend => {
                    if plan.sampled_finalized_at_utc.as_deref() != Some(now_utc) {
                        return Err(invalid_plan(
                            "intake append must use the store-sampled establishment timestamp",
                        ));
                    }
                }
                _ if plan.sampled_finalized_at_utc.is_some() => {
                    return Err(invalid_plan(
                        "candidate append and promotion are timestamp-free",
                    ));
                }
                _ => {}
            }
            validate_output_contract(request, &plan.outputs)
        }
        GenericPlannedOutcomeV1::Refuse(plan) => {
            validate_sha256(&plan.operation_context_fingerprint)?;
            if plan.operation_context_fingerprint != expected_context {
                return Err(invalid_plan(
                    "refusal plan context does not equal request authority",
                ));
            }
            if let Some(basis) = &plan.expected_basis_fingerprint {
                validate_sha256(basis)?;
            }
            validate_established_refusal(&plan.refusal)
        }
    }
}

fn validate_output_contract(
    request: &DerivedRequest,
    outputs: &[GenericPlannedOutputV1],
) -> Result<(), GenericLineageStoreErrorV1> {
    let operation = request.operation;
    if operation == GenericArtifactOperationV1::IntakeRecordAppend {
        if !(2..=16).contains(&outputs.len()) {
            return Err(invalid_plan(
                "intake append requires 1..=15 distinct values and one semantic record",
            ));
        }
        let final_ordinal = outputs.len() - 1;
        let mut refs = BTreeSet::new();
        let mut tokens = BTreeSet::new();
        for (ordinal, output) in outputs.iter().enumerate() {
            let is_record = ordinal == final_ordinal;
            let tuple_is_exact = if is_record {
                output.token == "intake-record"
                    && output.authority_class == GenericAuthorityClassV1::SemanticRecord
                    && output.install_mode == GenericInstallModeV1::CreateNew
                    && output.receipt_class == Some(GenericReceiptClassV1::SemanticRecord)
            } else {
                valid_intake_value_token(&output.token)
                    && output.authority_class == GenericAuthorityClassV1::SubordinateClosure
                    && output.install_mode == GenericInstallModeV1::CreateNew
                    && output.receipt_class.is_none()
            };
            if !tuple_is_exact || !tokens.insert(output.token.as_str()) {
                return Err(invalid_plan("intake output tuple is not exact or unique"));
            }
            validate_safe_ref(&output.final_ref)?;
            if !refs.insert(output.final_ref.as_str()) {
                return Err(invalid_plan("operation output refs must be unique"));
            }
            if output.bytes.is_empty() || output.bytes.len() > MAX_RECORD_BYTES {
                return Err(bound_error("planned output is outside 1..=1 MiB"));
            }
            validate_sha256(&output.output_fingerprint)?;
            validate_planned_output(operation, ordinal, final_ordinal, output)?;
            validate_output_ref_binding(
                operation,
                &request.request_subject,
                &output.token,
                &output.final_ref,
                &output.output_fingerprint,
            )?;
            validate_runtime_record_bindings(
                operation,
                ordinal,
                final_ordinal,
                &output.bytes,
                &request.request_subject,
                &plan_context(request)?,
                None,
            )?;
        }
        validate_intake_value_closure(outputs)?;
        return Ok(());
    }
    let expected: &[(
        &str,
        GenericAuthorityClassV1,
        GenericInstallModeV1,
        Option<GenericReceiptClassV1>,
    )] = match operation {
        GenericArtifactOperationV1::IntakeRecordAppend => unreachable!(),
        GenericArtifactOperationV1::ArtifactCandidateAppend => &[
            (
                "normalized-content",
                GenericAuthorityClassV1::SubordinateClosure,
                GenericInstallModeV1::CreateNew,
                None,
            ),
            (
                "validation-result",
                GenericAuthorityClassV1::SubordinateClosure,
                GenericInstallModeV1::CreateNew,
                None,
            ),
            (
                "candidate-record",
                GenericAuthorityClassV1::SemanticRecord,
                GenericInstallModeV1::CreateNew,
                Some(GenericReceiptClassV1::SemanticRecord),
            ),
        ],
        GenericArtifactOperationV1::ArtifactCandidatePromote => &[
            (
                "canonical-artifact",
                GenericAuthorityClassV1::CanonicalTruth,
                GenericInstallModeV1::ReplaceIfCurrent,
                Some(GenericReceiptClassV1::CanonicalTruth),
            ),
            (
                "promotion-record",
                GenericAuthorityClassV1::SemanticRecord,
                GenericInstallModeV1::CreateNew,
                Some(GenericReceiptClassV1::SemanticRecord),
            ),
        ],
    };
    if outputs.len() != expected.len() {
        return Err(invalid_plan("operation output cardinality is not exact"));
    }
    let mut refs = BTreeSet::new();
    for (ordinal, (output, contract)) in outputs.iter().zip(expected).enumerate() {
        if output.token != contract.0
            || output.authority_class != contract.1
            || output.install_mode != contract.2
            || output.receipt_class != contract.3
        {
            return Err(invalid_plan("operation output tuple is not exact"));
        }
        validate_safe_ref(&output.final_ref)?;
        if !refs.insert(output.final_ref.as_str()) {
            return Err(invalid_plan("operation output refs must be unique"));
        }
        if output.bytes.is_empty() || output.bytes.len() > MAX_RECORD_BYTES {
            return Err(bound_error("planned output is outside 1..=1 MiB"));
        }
        validate_sha256(&output.output_fingerprint)?;
        validate_planned_output(operation, ordinal, outputs.len() - 1, output)?;
        validate_output_ref_binding(
            operation,
            &request.request_subject,
            &output.token,
            &output.final_ref,
            &output.output_fingerprint,
        )?;
        validate_runtime_record_bindings(
            operation,
            ordinal,
            outputs.len() - 1,
            &output.bytes,
            &request.request_subject,
            &plan_context(request)?,
            None,
        )?;
    }
    validate_output_cross_bindings(operation, outputs)?;
    Ok(())
}

fn plan_context(request: &DerivedRequest) -> Result<String, GenericLineageStoreErrorV1> {
    Ok(required_string(
        request
            .request_subject
            .as_object()
            .ok_or_else(|| invalid_plan("request subject is not an object"))?,
        "operation_context_fingerprint",
    )?
    .to_owned())
}

fn validate_planned_output(
    operation: GenericArtifactOperationV1,
    ordinal: usize,
    final_ordinal: usize,
    output: &GenericPlannedOutputV1,
) -> Result<(), GenericLineageStoreErrorV1> {
    match (operation, ordinal) {
        (GenericArtifactOperationV1::IntakeRecordAppend, ordinal) if ordinal < final_ordinal => {
            let value = parse_jcs_lf(&output.bytes)?;
            let fingerprint = sha256_prefixed(&canonical_json(&value)?);
            if fingerprint != output.output_fingerprint {
                return Err(invalid_plan(
                    "subordinate JSON value fingerprint does not recompute",
                ));
            }
        }
        (GenericArtifactOperationV1::ArtifactCandidateAppend, 0) => {
            let value = parse_jcs_lf(&output.bytes)?;
            let fingerprint = sha256_prefixed(&canonical_json(&value)?);
            if fingerprint != output.output_fingerprint {
                return Err(invalid_plan(
                    "subordinate JSON value fingerprint does not recompute",
                ));
            }
        }
        (GenericArtifactOperationV1::ArtifactCandidateAppend, 1) => {
            validate_runtime_record(
                &output.bytes,
                "handbook.artifact-validation-result",
                "1.0",
                "validation_result_id",
                "validation_result_fingerprint",
                "validation",
                &output.output_fingerprint,
            )?;
        }
        (GenericArtifactOperationV1::IntakeRecordAppend, ordinal) if ordinal == final_ordinal => {
            validate_runtime_record(
                &output.bytes,
                "handbook.artifact-intake-record",
                "1.2",
                "intake_record_id",
                "record_fingerprint",
                "intake",
                &output.output_fingerprint,
            )?;
        }
        (GenericArtifactOperationV1::ArtifactCandidateAppend, 2) => {
            validate_runtime_record(
                &output.bytes,
                "handbook.artifact-candidate",
                "1.4",
                "candidate_id",
                "candidate_fingerprint",
                "candidate",
                &output.output_fingerprint,
            )?;
        }
        (GenericArtifactOperationV1::ArtifactCandidatePromote, 0) => {
            if sha256_prefixed(&output.bytes) != output.output_fingerprint {
                return Err(invalid_plan(
                    "canonical artifact fingerprint does not match exact bytes",
                ));
            }
        }
        (GenericArtifactOperationV1::ArtifactCandidatePromote, 1) => {
            validate_runtime_record(
                &output.bytes,
                "handbook.artifact-promotion-record",
                "1.2",
                "promotion_id",
                "promotion_fingerprint",
                "promotion",
                &output.output_fingerprint,
            )?;
        }
        _ => return Err(invalid_plan("unknown operation output ordinal")),
    }
    if output.authority_class != GenericAuthorityClassV1::CanonicalTruth
        && !output_ref_matches_fingerprint(&output.final_ref, &output.output_fingerprint)
    {
        return Err(invalid_plan(
            "content-addressed output ref does not bind its semantic fingerprint",
        ));
    }
    Ok(())
}

fn validate_output_ref_binding(
    operation: GenericArtifactOperationV1,
    request_subject: &Value,
    token: &str,
    final_ref: &str,
    fingerprint: &str,
) -> Result<(), GenericLineageStoreErrorV1> {
    let subject = request_subject
        .as_object()
        .ok_or_else(|| invalid_plan("request subject is not an object"))?;
    let instance = required_string(subject, "instance_id")?;
    let hex = fingerprint_hex(fingerprint)?;
    let expected = match (operation, token) {
        (GenericArtifactOperationV1::IntakeRecordAppend, "intake-record") => {
            format!(".handbook/state/artifacts/{instance}/intake-records/intake_{hex}.json")
        }
        (GenericArtifactOperationV1::IntakeRecordAppend, value)
            if valid_intake_value_token(value) =>
        {
            format!(".handbook/evidence/artifacts/{instance}/intake-values/value_{hex}.json")
        }
        (GenericArtifactOperationV1::ArtifactCandidateAppend, "normalized-content") => {
            format!(".handbook/evidence/artifacts/{instance}/content/content_{hex}.json")
        }
        (GenericArtifactOperationV1::ArtifactCandidateAppend, "validation-result") => format!(
            ".handbook/evidence/artifacts/{instance}/validation-results/validation_{hex}.json"
        ),
        (GenericArtifactOperationV1::ArtifactCandidateAppend, "candidate-record") => {
            if required_string(subject, "expected_candidate_fingerprint")? != fingerprint {
                return Err(invalid_plan(
                    "candidate output does not equal the request candidate fingerprint",
                ));
            }
            format!(".handbook/evidence/artifacts/{instance}/candidates/candidate_{hex}.json")
        }
        (GenericArtifactOperationV1::ArtifactCandidatePromote, "canonical-artifact") => {
            required_string(subject, "canonical_artifact_ref")?.to_owned()
        }
        (GenericArtifactOperationV1::ArtifactCandidatePromote, "promotion-record") => {
            format!(".handbook/state/artifacts/{instance}/promotion-records/promotion_{hex}.json")
        }
        _ => return Err(invalid_plan("output token has no closed authority path")),
    };
    if final_ref != expected {
        return Err(invalid_plan(
            "output ref does not derive from operation, instance, and fingerprint",
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn validate_runtime_record_bindings(
    operation: GenericArtifactOperationV1,
    ordinal: usize,
    final_ordinal: usize,
    bytes: &[u8],
    request_subject: &Value,
    operation_context_fingerprint: &str,
    sampled_finalized_at_utc: Option<&str>,
) -> Result<(), GenericLineageStoreErrorV1> {
    let fields: Option<&[&str]> = match (operation, ordinal) {
        (GenericArtifactOperationV1::IntakeRecordAppend, value) if value == final_ordinal => {
            Some(&[
                "schema_id",
                "schema_version",
                "intake_definition_ref",
                "intake_definition_fingerprint",
                "acquisition_mode",
                "target_kind_ref",
                "target_instance_id",
                "target_schema_ref",
                "profile_ref",
                "resolved_profile_fingerprint",
                "operation_context_fingerprint",
                "consumer",
                "coverage_results",
                "basis_artifact_fingerprint",
                "prompt_event_refs",
                "finalized_at_utc",
                "intake_record_id",
                "record_fingerprint",
            ])
        }
        (GenericArtifactOperationV1::ArtifactCandidateAppend, 1) => Some(&[
            "schema_id",
            "schema_version",
            "target_kind_ref",
            "target_instance_id",
            "target_schema_ref",
            "profile_ref",
            "resolved_profile_fingerprint",
            "operation_context_fingerprint",
            "intake_record_ref",
            "intake_record_fingerprint",
            "normalized_content_ref",
            "normalized_content_fingerprint",
            "basis_artifact_fingerprint",
            "layers",
            "outcome",
            "validation_result_id",
            "validation_result_fingerprint",
        ]),
        (GenericArtifactOperationV1::ArtifactCandidateAppend, 2) => Some(&[
            "schema_id",
            "schema_version",
            "intake_record_ref",
            "intake_record_fingerprint",
            "target_kind_ref",
            "target_instance_id",
            "target_schema_ref",
            "profile_ref",
            "resolved_profile_fingerprint",
            "operation_context_fingerprint",
            "normalized_content_ref",
            "normalized_content_fingerprint",
            "field_sources",
            "validation_result_refs",
            "unresolved_coverage_ids",
            "promotion_eligibility",
            "required_approval_policy_ref",
            "basis_artifact_fingerprint",
            "candidate_id",
            "candidate_fingerprint",
        ]),
        (GenericArtifactOperationV1::ArtifactCandidatePromote, 1) => Some(&[
            "schema_id",
            "schema_version",
            "candidate_ref",
            "candidate_fingerprint",
            "target_instance_id",
            "expected_current_artifact_fingerprint",
            "profile_ref",
            "resolved_profile_fingerprint",
            "operation_context_fingerprint",
            "resolved_definitions",
            "approval_refs",
            "validation_result_refs",
            "decision",
            "authorized_by_ref",
            "canonical_artifact_ref",
            "canonical_artifact_fingerprint",
            "promotion_id",
            "promotion_fingerprint",
        ]),
        _ => None,
    };
    let Some(fields) = fields else {
        return Ok(());
    };
    let value = parse_jcs_lf(bytes)?;
    require_exact_fields(&value, fields, GenericLineageStoreErrorKindV1::InvalidPlan)?;
    let subject = request_subject
        .as_object()
        .ok_or_else(|| invalid_plan("request subject is not an object"))?;
    if value.get("target_instance_id").and_then(Value::as_str)
        != Some(required_string(subject, "instance_id")?)
        || value
            .get("operation_context_fingerprint")
            .and_then(Value::as_str)
            != Some(operation_context_fingerprint)
    {
        return Err(invalid_plan(
            "runtime record does not bind the request target and context",
        ));
    }
    if operation != GenericArtifactOperationV1::ArtifactCandidatePromote
        && value.get("target_kind_ref").and_then(Value::as_str)
            != Some(required_string(subject, "kind_ref")?)
    {
        return Err(invalid_plan(
            "runtime record does not bind the request artifact kind",
        ));
    }
    match operation {
        GenericArtifactOperationV1::IntakeRecordAppend => {
            if value.get("acquisition_mode") != request_subject.get("acquisition_mode")
                || value.get("basis_artifact_fingerprint")
                    != request_subject.get("expected_current_artifact_fingerprint")
                || sampled_finalized_at_utc.is_some_and(|expected| {
                    value.get("finalized_at_utc").and_then(Value::as_str) != Some(expected)
                })
                || !value
                    .get("prompt_event_refs")
                    .and_then(Value::as_array)
                    .is_some_and(Vec::is_empty)
            {
                return Err(invalid_plan("intake record request bindings are not exact"));
            }
            validate_intake_runtime_shape(&value)?;
        }
        GenericArtifactOperationV1::ArtifactCandidateAppend => {
            if value.get("intake_record_ref") != request_subject.get("intake_record_ref")
                || value.get("intake_record_fingerprint")
                    != request_subject.get("intake_record_fingerprint")
            {
                return Err(invalid_plan(
                    "candidate runtime record does not bind its requested intake",
                ));
            }
            if ordinal == 1 {
                validate_validation_runtime_shape(&value)?;
            } else {
                if value.get("candidate_fingerprint")
                    != request_subject.get("expected_candidate_fingerprint")
                    || value.get("promotion_eligibility").and_then(Value::as_str)
                        != Some("eligible_without_approval")
                    || !value
                        .get("unresolved_coverage_ids")
                        .and_then(Value::as_array)
                        .is_some_and(Vec::is_empty)
                    || !value
                        .get("required_approval_policy_ref")
                        .is_some_and(Value::is_null)
                {
                    return Err(invalid_plan("candidate record authority is not exact"));
                }
                validate_candidate_runtime_shape(&value)?;
            }
        }
        GenericArtifactOperationV1::ArtifactCandidatePromote => {
            if value.get("candidate_ref") != request_subject.get("candidate_ref")
                || value.get("candidate_fingerprint")
                    != request_subject.get("candidate_fingerprint")
                || value.get("expected_current_artifact_fingerprint")
                    != request_subject.get("expected_current_artifact_fingerprint")
                || value.get("canonical_artifact_ref")
                    != request_subject.get("canonical_artifact_ref")
                || value.get("decision").and_then(Value::as_str) != Some("not_required")
                || !value
                    .get("approval_refs")
                    .and_then(Value::as_array)
                    .is_some_and(Vec::is_empty)
                || !value.get("authorized_by_ref").is_some_and(Value::is_null)
            {
                return Err(invalid_plan("promotion record authority is not exact"));
            }
            validate_promotion_runtime_shape(&value)?;
        }
    }
    Ok(())
}

fn validate_intake_runtime_shape(value: &Value) -> Result<(), GenericLineageStoreErrorV1> {
    validate_runtime_exact_refs(
        value,
        &[
            "intake_definition_ref",
            "target_kind_ref",
            "target_schema_ref",
            "profile_ref",
        ],
    )?;
    validate_runtime_fingerprints(
        value,
        &[
            "intake_definition_fingerprint",
            "resolved_profile_fingerprint",
            "operation_context_fingerprint",
        ],
    )?;
    validate_instance_id(string_field(value, "target_instance_id")?)?;
    validate_nullable_sha256(value.get("basis_artifact_fingerprint"))?;
    validate_utc(string_field(value, "finalized_at_utc")?)?;
    let consumer = value
        .get("consumer")
        .ok_or_else(|| invalid_plan("intake consumer is absent"))?;
    require_exact_fields(
        consumer,
        &["kind", "id", "version"],
        GenericLineageStoreErrorKindV1::InvalidPlan,
    )?;
    if string_field(consumer, "kind")? != "handbook_cli"
        || string_field(consumer, "id")? != "handbook"
        || string_field(consumer, "version")?.is_empty()
        || string_field(consumer, "version")?.len() > 64
    {
        return Err(invalid_plan(
            "intake consumer is not the closed CLI identity",
        ));
    }
    let coverage = value
        .get("coverage_results")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid_plan("intake coverage results are malformed"))?;
    if coverage.is_empty() || coverage.len() > 15 {
        return Err(invalid_plan(
            "intake coverage cardinality is outside 1..=15",
        ));
    }
    for result in coverage {
        require_exact_fields(
            result,
            &[
                "coverage_id",
                "applicability",
                "source_kind",
                "value_ref",
                "value_fingerprint",
                "evidence_refs",
                "confidence",
                "freshness",
                "sensitivity",
                "evaluation",
                "contradiction_refs",
                "waiver_ref",
            ],
            GenericLineageStoreErrorKindV1::InvalidPlan,
        )?;
        validate_sha256(string_field(result, "value_fingerprint")?)?;
        validate_safe_ref(string_field(result, "value_ref")?)?;
        validate_symbolic_id(string_field(result, "coverage_id")?)?;
        if string_field(result, "applicability")? != "applicable"
            || string_field(result, "source_kind")? != "user_declaration"
            || string_field(result, "confidence")? != "high"
            || string_field(result, "sensitivity")? != "internal"
            || string_field(result, "evaluation")? != "satisfied"
            || !result
                .get("evidence_refs")
                .and_then(Value::as_array)
                .is_some_and(Vec::is_empty)
            || !result
                .get("contradiction_refs")
                .and_then(Value::as_array)
                .is_some_and(Vec::is_empty)
            || !result.get("freshness").is_some_and(Value::is_null)
            || !result.get("waiver_ref").is_some_and(Value::is_null)
        {
            return Err(invalid_plan("intake coverage result is not exact"));
        }
    }
    Ok(())
}

fn validate_validation_runtime_shape(value: &Value) -> Result<(), GenericLineageStoreErrorV1> {
    validate_runtime_exact_refs(
        value,
        &["target_kind_ref", "target_schema_ref", "profile_ref"],
    )?;
    validate_runtime_fingerprints(
        value,
        &[
            "resolved_profile_fingerprint",
            "operation_context_fingerprint",
            "intake_record_fingerprint",
            "normalized_content_fingerprint",
        ],
    )?;
    validate_instance_id(string_field(value, "target_instance_id")?)?;
    validate_safe_ref(string_field(value, "intake_record_ref")?)?;
    validate_safe_ref(string_field(value, "normalized_content_ref")?)?;
    validate_nullable_sha256(value.get("basis_artifact_fingerprint"))?;
    if string_field(value, "outcome")? != "valid" {
        return Err(invalid_plan("validation result outcome is not valid"));
    }
    let layers = value
        .get("layers")
        .ok_or_else(|| invalid_plan("validation layers are absent"))?;
    require_exact_fields(
        layers,
        &[
            "source",
            "registration",
            "canonical_syntax",
            "structural",
            "semantic",
            "intake",
            "approval",
            "external_evidence",
        ],
        GenericLineageStoreErrorKindV1::InvalidPlan,
    )?;
    for name in ["source", "registration", "canonical_syntax"] {
        validate_exact_status_layer(layers.get(name), "pass", None)?;
    }
    let structural = layers
        .get("structural")
        .ok_or_else(|| invalid_plan("structural validation layer is absent"))?;
    require_exact_fields(
        structural,
        &["status", "schema_ref", "schema_closure_fingerprint"],
        GenericLineageStoreErrorKindV1::InvalidPlan,
    )?;
    if string_field(structural, "status")? != "pass" {
        return Err(invalid_plan("structural validation layer did not pass"));
    }
    validate_exact_ref(string_field(structural, "schema_ref")?)?;
    validate_sha256(string_field(structural, "schema_closure_fingerprint")?)?;
    validate_exact_status_layer(
        layers.get("semantic"),
        "not_applicable",
        Some("no_semantic_validator"),
    )?;
    let intake = layers
        .get("intake")
        .ok_or_else(|| invalid_plan("intake validation layer is absent"))?;
    require_exact_fields(
        intake,
        &[
            "status",
            "intake_definition_ref",
            "intake_definition_fingerprint",
        ],
        GenericLineageStoreErrorKindV1::InvalidPlan,
    )?;
    if string_field(intake, "status")? != "pass" {
        return Err(invalid_plan("intake validation layer did not pass"));
    }
    validate_exact_ref(string_field(intake, "intake_definition_ref")?)?;
    validate_sha256(string_field(intake, "intake_definition_fingerprint")?)?;
    validate_exact_status_layer(
        layers.get("approval"),
        "not_applicable",
        Some("null_approval_policy"),
    )?;
    validate_exact_status_layer(
        layers.get("external_evidence"),
        "not_applicable",
        Some("not_selected"),
    )?;
    Ok(())
}

fn validate_candidate_runtime_shape(value: &Value) -> Result<(), GenericLineageStoreErrorV1> {
    validate_runtime_exact_refs(
        value,
        &["target_kind_ref", "target_schema_ref", "profile_ref"],
    )?;
    validate_runtime_fingerprints(
        value,
        &[
            "intake_record_fingerprint",
            "resolved_profile_fingerprint",
            "operation_context_fingerprint",
            "normalized_content_fingerprint",
        ],
    )?;
    validate_instance_id(string_field(value, "target_instance_id")?)?;
    validate_safe_ref(string_field(value, "intake_record_ref")?)?;
    validate_safe_ref(string_field(value, "normalized_content_ref")?)?;
    validate_nullable_sha256(value.get("basis_artifact_fingerprint"))?;
    let sources = value
        .get("field_sources")
        .and_then(Value::as_array)
        .filter(|values| !values.is_empty() && values.len() <= 1024)
        .ok_or_else(|| invalid_plan("candidate field sources are outside their bound"))?;
    let mut target_paths = BTreeSet::new();
    let mut coverage_ids = BTreeSet::new();
    for source in sources {
        require_exact_fields(
            source,
            &[
                "target_path",
                "coverage_id",
                "source_kind",
                "value_ref",
                "value_fingerprint",
            ],
            GenericLineageStoreErrorKindV1::InvalidPlan,
        )?;
        let target_path = string_field(source, "target_path")?;
        let coverage_id = string_field(source, "coverage_id")?;
        validate_json_pointer(target_path)?;
        validate_symbolic_id(coverage_id)?;
        validate_safe_ref(string_field(source, "value_ref")?)?;
        validate_sha256(string_field(source, "value_fingerprint")?)?;
        if string_field(source, "source_kind")? != "user_declaration"
            || !target_paths.insert(target_path)
            || !coverage_ids.insert(coverage_id)
        {
            return Err(invalid_plan(
                "candidate field-source authority is not unique and closed",
            ));
        }
    }
    let validation_refs = value
        .get("validation_result_refs")
        .and_then(Value::as_array)
        .filter(|values| values.len() == 1)
        .ok_or_else(|| invalid_plan("candidate validation closure is not singular"))?;
    validate_safe_ref(
        validation_refs[0]
            .as_str()
            .ok_or_else(|| invalid_plan("candidate validation ref is not a string"))?,
    )
}

fn validate_promotion_runtime_shape(value: &Value) -> Result<(), GenericLineageStoreErrorV1> {
    validate_exact_ref(string_field(value, "profile_ref")?)?;
    validate_runtime_fingerprints(
        value,
        &[
            "candidate_fingerprint",
            "resolved_profile_fingerprint",
            "operation_context_fingerprint",
            "canonical_artifact_fingerprint",
        ],
    )?;
    validate_instance_id(string_field(value, "target_instance_id")?)?;
    validate_safe_ref(string_field(value, "candidate_ref")?)?;
    validate_safe_ref(string_field(value, "canonical_artifact_ref")?)?;
    validate_nullable_sha256(value.get("expected_current_artifact_fingerprint"))?;
    let definitions = value
        .get("resolved_definitions")
        .and_then(Value::as_array)
        .filter(|values| (4..=64).contains(&values.len()))
        .ok_or_else(|| invalid_plan("promotion definitions are outside their bound"))?;
    let mut previous = None;
    for definition in definitions {
        require_exact_fields(
            definition,
            &["definition_ref", "definition_fingerprint"],
            GenericLineageStoreErrorKindV1::InvalidPlan,
        )?;
        let reference = string_field(definition, "definition_ref")?;
        validate_exact_ref(reference)?;
        validate_sha256(string_field(definition, "definition_fingerprint")?)?;
        if previous.is_some_and(|value| value >= reference) {
            return Err(invalid_plan(
                "promotion definitions are not unique UTF-8 ref order",
            ));
        }
        previous = Some(reference);
    }
    let validation_refs = value
        .get("validation_result_refs")
        .and_then(Value::as_array)
        .filter(|values| values.len() == 1)
        .ok_or_else(|| invalid_plan("promotion validation closure is not singular"))?;
    validate_safe_ref(
        validation_refs[0]
            .as_str()
            .ok_or_else(|| invalid_plan("promotion validation ref is not a string"))?,
    )
}

fn validate_runtime_exact_refs(
    value: &Value,
    fields: &[&str],
) -> Result<(), GenericLineageStoreErrorV1> {
    for field in fields {
        validate_exact_ref(string_field(value, field)?)?;
    }
    Ok(())
}

fn validate_runtime_fingerprints(
    value: &Value,
    fields: &[&str],
) -> Result<(), GenericLineageStoreErrorV1> {
    for field in fields {
        validate_sha256(string_field(value, field)?)?;
    }
    Ok(())
}

fn validate_exact_status_layer(
    layer: Option<&Value>,
    expected_status: &str,
    expected_reason: Option<&str>,
) -> Result<(), GenericLineageStoreErrorV1> {
    let layer = layer.ok_or_else(|| invalid_plan("validation layer is absent"))?;
    let fields: &[&str] = if expected_reason.is_some() {
        &["status", "reason"]
    } else {
        &["status"]
    };
    require_exact_fields(layer, fields, GenericLineageStoreErrorKindV1::InvalidPlan)?;
    if string_field(layer, "status")? != expected_status
        || expected_reason.is_some_and(|reason| string_field(layer, "reason") != Ok(reason))
    {
        return Err(invalid_plan("validation layer status is not exact"));
    }
    Ok(())
}

fn validate_symbolic_id(value: &str) -> Result<(), GenericLineageStoreErrorV1> {
    if value.is_empty()
        || value.len() > 128
        || value.split(['.', '-']).any(|segment| {
            segment.is_empty()
                || !segment.as_bytes()[0].is_ascii_lowercase()
                || !segment
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        })
    {
        return Err(invalid_plan("symbolic identifier is outside its grammar"));
    }
    Ok(())
}

fn validate_json_pointer(value: &str) -> Result<(), GenericLineageStoreErrorV1> {
    if !value.starts_with('/')
        || value.len() < 2
        || value.split('/').skip(1).any(|segment| {
            segment.is_empty()
                || segment.as_bytes().iter().enumerate().any(|(index, byte)| {
                    *byte == b'~'
                        && !segment
                            .as_bytes()
                            .get(index + 1)
                            .is_some_and(|next| matches!(*next, b'0' | b'1'))
                })
        })
    {
        return Err(invalid_plan("JSON pointer is outside its grammar"));
    }
    Ok(())
}

fn validate_output_cross_bindings(
    operation: GenericArtifactOperationV1,
    outputs: &[GenericPlannedOutputV1],
) -> Result<(), GenericLineageStoreErrorV1> {
    match operation {
        GenericArtifactOperationV1::IntakeRecordAppend => Ok(()),
        GenericArtifactOperationV1::ArtifactCandidateAppend => {
            let validation = parse_jcs_lf(&outputs[1].bytes)?;
            let candidate = parse_jcs_lf(&outputs[2].bytes)?;
            if string_field(&validation, "normalized_content_ref")? != outputs[0].final_ref
                || string_field(&validation, "normalized_content_fingerprint")?
                    != outputs[0].output_fingerprint
                || string_field(&candidate, "normalized_content_ref")? != outputs[0].final_ref
                || string_field(&candidate, "normalized_content_fingerprint")?
                    != outputs[0].output_fingerprint
                || candidate.get("validation_result_refs") != Some(&json!([outputs[1].final_ref]))
            {
                return Err(invalid_plan(
                    "candidate output closure does not bind its exact subordinate outputs",
                ));
            }
            Ok(())
        }
        GenericArtifactOperationV1::ArtifactCandidatePromote => {
            let promotion = parse_jcs_lf(&outputs[1].bytes)?;
            if string_field(&promotion, "canonical_artifact_ref")? != outputs[0].final_ref
                || string_field(&promotion, "canonical_artifact_fingerprint")?
                    != outputs[0].output_fingerprint
            {
                return Err(invalid_plan(
                    "promotion record does not bind the canonical output",
                ));
            }
            Ok(())
        }
    }
}

fn valid_intake_value_token(value: &str) -> bool {
    value.ends_with("-value")
        && value.len() > "-value".len()
        && value.len() <= 96
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && !value.starts_with('-')
        && !value.contains("--")
}

fn validate_intake_value_closure(
    outputs: &[GenericPlannedOutputV1],
) -> Result<(), GenericLineageStoreErrorV1> {
    let record = parse_jcs_lf(&outputs.last().expect("intake outputs are nonempty").bytes)?;
    let coverage = record
        .get("coverage_results")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid_plan("intake record coverage results are absent"))?;
    let cited = coverage
        .iter()
        .map(|row| {
            Ok((
                string_field(row, "value_ref")?.to_owned(),
                string_field(row, "value_fingerprint")?.to_owned(),
            ))
        })
        .collect::<Result<BTreeSet<_>, GenericLineageStoreErrorV1>>()?;
    let planned = outputs[..outputs.len() - 1]
        .iter()
        .map(|output| (output.final_ref.clone(), output.output_fingerprint.clone()))
        .collect::<BTreeSet<_>>();
    if cited != planned {
        return Err(invalid_plan(
            "intake record value closure does not equal the distinct subordinate outputs",
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn validate_runtime_record(
    bytes: &[u8],
    schema_id: &str,
    schema_version: &str,
    id_field: &str,
    fingerprint_field: &str,
    id_prefix: &str,
    expected_fingerprint: &str,
) -> Result<(), GenericLineageStoreErrorV1> {
    let mut value = parse_jcs_lf(bytes)?;
    let object = value
        .as_object_mut()
        .ok_or_else(|| invalid_plan("runtime record must have an object root"))?;
    if object.get("schema_id").and_then(Value::as_str) != Some(schema_id)
        || object.get("schema_version").and_then(Value::as_str) != Some(schema_version)
        || object.get(fingerprint_field).and_then(Value::as_str) != Some(expected_fingerprint)
    {
        return Err(invalid_plan(
            "runtime record family/version/fingerprint mismatch",
        ));
    }
    let expected_id = format!("{id_prefix}_{}", fingerprint_hex(expected_fingerprint)?);
    if object.get(id_field).and_then(Value::as_str) != Some(expected_id.as_str()) {
        return Err(invalid_plan(
            "runtime record ID does not derive from its fingerprint",
        ));
    }
    object.remove(id_field);
    object.remove(fingerprint_field);
    if sha256_prefixed(&canonical_json(&value)?) != expected_fingerprint {
        return Err(invalid_plan(
            "runtime record fingerprint preimage does not recompute",
        ));
    }
    Ok(())
}

fn validate_established_refusal(
    refusal: &EstablishedRefusalV1,
) -> Result<(), GenericLineageStoreErrorV1> {
    use EstablishedRefusalCodeV1 as Code;
    use GenericRefusalLayerV1 as Layer;
    let both_null =
        refusal.expected_fingerprint.is_none() && refusal.observed_fingerprint.is_none();
    let valid = match refusal.code {
        Code::CanonicalSyntaxInvalid => refusal.layer == Layer::CanonicalSyntax && both_null,
        Code::StructuralValidationFailed => refusal.layer == Layer::Structural && both_null,
        Code::IntakeCoverageBlocked => refusal.layer == Layer::Intake && both_null,
        Code::OperationIneligible => refusal.layer == Layer::Eligibility && both_null,
        Code::StaleBasis => {
            refusal.layer == Layer::Currentness
                && matches!(
                    (&refusal.expected_fingerprint, &refusal.observed_fingerprint),
                    (Some(expected), Some(observed)) if expected != observed
                )
        }
        Code::StaleCurrentArtifact => {
            refusal.layer == Layer::Currentness
                && refusal.expected_fingerprint != refusal.observed_fingerprint
        }
        Code::PublicationBasisConflict => {
            refusal.layer == Layer::Publication
                && refusal.expected_fingerprint != refusal.observed_fingerprint
        }
    };
    for fingerprint in [
        refusal.expected_fingerprint.as_deref(),
        refusal.observed_fingerprint.as_deref(),
    ]
    .into_iter()
    .flatten()
    {
        validate_sha256(fingerprint)?;
    }
    if !valid {
        return Err(error(
            GenericLineageStoreErrorKindV1::InvalidEstablishedRefusal,
            "established refusal code, layer, and fingerprint nullability are inconsistent",
        ));
    }
    Ok(())
}

fn build_intent(
    request: &DerivedRequest,
    planned: &GenericPlannedOutcomeV1,
    expected_basis_byte_length: Option<u64>,
    expected_basis_identity_token: Option<&str>,
    expected_basis_version_token: Option<&str>,
) -> Result<Value, GenericLineageStoreErrorV1> {
    if matches!(
        planned,
        GenericPlannedOutcomeV1::Refuse(GenericRefusalPlanV1 {
            refusal: EstablishedRefusalV1 {
                code: EstablishedRefusalCodeV1::PublicationBasisConflict,
                ..
            },
            ..
        })
    ) {
        return Err(invalid_plan(
            "publication basis conflict may be established only at the final native boundary",
        ));
    }
    let (context, basis, sampled, outcome, refusal, outputs) = match planned {
        GenericPlannedOutcomeV1::Commit(plan) => (
            plan.operation_context_fingerprint.clone(),
            plan.expected_basis_fingerprint.clone(),
            plan.sampled_finalized_at_utc.clone(),
            "commit",
            Value::Null,
            plan.outputs
                .iter()
                .enumerate()
                .map(|(ordinal, output)| {
                    json!({
                        "ordinal": ordinal,
                        "token": output.token,
                        "authority_class": output.authority_class,
                        "final_ref": output.final_ref,
                        "bytes_sha256": sha256_prefixed(&output.bytes),
                        "byte_length": output.bytes.len(),
                        "install_mode": output.install_mode,
                        "receipt_class": output.receipt_class
                    })
                })
                .collect::<Vec<_>>(),
        ),
        GenericPlannedOutcomeV1::Refuse(plan) => (
            plan.operation_context_fingerprint.clone(),
            plan.expected_basis_fingerprint.clone(),
            None,
            "refuse",
            serde_json::to_value(&plan.refusal)
                .map_err(|_| invalid_plan("refusal serialization failed"))?,
            Vec::new(),
        ),
    };
    let transaction_id = derive_transaction_id(
        &request.repository_identity_fingerprint,
        &request.owner_contract_subject_fingerprint,
        request.operation,
        &request.key_fingerprint,
        &request.request_fingerprint,
    )?;
    let mut intent = json!({
        "schema_id": "handbook.generic-transaction-intent",
        "schema_version": "1.0",
        "transaction_id": transaction_id,
        "repository_identity_fingerprint": request.repository_identity_fingerprint,
        "owner_contract_ref": GENERIC_ARTIFACT_OWNER_CONTRACT_REF,
        "owner_contract_subject_fingerprint": request.owner_contract_subject_fingerprint,
        "operation_id": request.operation.operation_id(),
        "domain_mutation_key_fingerprint": request.key_fingerprint,
        "request_fingerprint": request.request_fingerprint,
        "request_subject": request.request_subject,
        "operation_context_fingerprint": context,
        "expected_basis_fingerprint": basis,
        "sampled_finalized_at_utc": sampled,
        "planned_outcome": outcome,
        "refusal": refusal,
        "outputs": outputs
    });
    if request.operation == GenericArtifactOperationV1::ArtifactCandidatePromote
        && outcome == "commit"
    {
        let object = intent.as_object_mut().expect("intent object");
        if basis.is_some() {
            let byte_length = expected_basis_byte_length
                .filter(|length| (1..=MAX_RECORD_BYTES as u64).contains(length))
                .ok_or_else(|| invalid_plan("present promotion basis has no exact byte length"))?;
            let identity_token = expected_basis_identity_token
                .ok_or_else(|| invalid_plan("present promotion basis has no native identity"))?;
            let version_token = expected_basis_version_token
                .ok_or_else(|| invalid_plan("present promotion basis has no native version"))?;
            validate_native_token(identity_token, "native-id-v1:")?;
            validate_native_token(version_token, "native-version-v1:")?;
            object.insert("expected_basis_presence".to_owned(), json!("present"));
            object.insert("expected_basis_byte_length".to_owned(), json!(byte_length));
            object.insert(
                "expected_basis_identity_token".to_owned(),
                json!(identity_token),
            );
            object.insert(
                "expected_basis_version_token".to_owned(),
                json!(version_token),
            );
        } else {
            if expected_basis_byte_length.is_some()
                || expected_basis_identity_token.is_some()
                || expected_basis_version_token.is_some()
            {
                return Err(invalid_plan(
                    "absent promotion basis carries native observation values",
                ));
            }
            object.insert("expected_basis_presence".to_owned(), json!("absent"));
            object.insert("expected_basis_byte_length".to_owned(), Value::Null);
            object.insert("expected_basis_identity_token".to_owned(), Value::Null);
            object.insert("expected_basis_version_token".to_owned(), Value::Null);
        }
    } else if expected_basis_byte_length.is_some()
        || expected_basis_identity_token.is_some()
        || expected_basis_version_token.is_some()
    {
        return Err(invalid_plan(
            "only a promotion commit may carry native basis observations",
        ));
    }
    finalize_control(intent, "intent_fingerprint")
}

fn build_native_publication_observation(
    intent: &Value,
    replacement: &PreparedReplacement,
) -> Result<Value, GenericLineageStoreErrorV1> {
    let outputs = intent_outputs(intent)?;
    let canonical = outputs
        .iter()
        .find(|output| string_field(output, "install_mode").ok() == Some("replace_if_current"))
        .ok_or_else(|| invalid_intent("promotion intent has no canonical replacement"))?;
    let transaction_id = string_field(intent, "transaction_id")?;
    let canonical_ref = string_field(canonical, "final_ref")?;
    let (repo_root, candidate_ref) = store_relative_path(&replacement.path)?;
    let canonical_path = repo_root.join(canonical_ref);
    let displaced_ref = candidate_ref
        .strip_suffix(".candidate")
        .map(|prefix| format!("{prefix}.displaced"))
        .ok_or_else(|| invalid_intent("publication candidate does not use the exact suffix"))?;
    let expected_presence = string_field(intent, "expected_basis_presence")?;
    let expected_fingerprint = intent
        .get("expected_basis_fingerprint")
        .cloned()
        .unwrap_or(Value::Null);
    let expected_byte_length = intent
        .get("expected_basis_byte_length")
        .cloned()
        .unwrap_or(Value::Null);
    let expected_identity = intent
        .get("expected_basis_identity_token")
        .cloned()
        .unwrap_or(Value::Null);
    let expected_version = intent
        .get("expected_basis_version_token")
        .cloned()
        .unwrap_or(Value::Null);
    let replacement_metadata = replacement
        .file
        .metadata()
        .map_err(|_| io_error("replacement publication metadata failed"))?;
    let (_, _, replacement_platform) =
        native_metadata_subjects(&replacement_metadata, &replacement.path)?;
    let pending_ref =
        format!(".handbook/state/transactions/artifact-promotions/{transaction_id}.pending");
    let pending = repo_root.join(&pending_ref);
    let (expected_platform, backup_ref, backup_digest, backup_length, backup_metadata_fingerprint) =
        if expected_presence == "present" {
            let (_guard, bytes, identity, version) =
                observe_retained_regular_file(&canonical_path, MAX_RECORD_BYTES)?;
            if expected_fingerprint.as_str() != Some(sha256_prefixed(&bytes).as_str())
                || expected_byte_length.as_u64() != Some(bytes.len() as u64)
                || expected_identity.as_str() != Some(identity.as_str())
                || expected_version.as_str() != Some(version.as_str())
            {
                return Err(error(
                    GenericLineageStoreErrorKindV1::BasisMismatch,
                    "native publication basis no longer equals the complete intent basis",
                ));
            }
            let metadata = fs::symlink_metadata(&canonical_path)
                .map_err(|_| io_error("publication basis metadata failed"))?;
            let (_, _, platform) = native_metadata_subjects(&metadata, &canonical_path)?;
            let backup_ref = format!("{pending_ref}/expected-basis.backup");
            write_new_or_equal(&pending.join("expected-basis.backup"), &bytes)?;
            (
                platform.clone(),
                json!(backup_ref),
                json!(sha256_prefixed(&bytes)),
                json!(bytes.len()),
                json!(sha256_prefixed(&canonical_json(&platform)?)),
            )
        } else if expected_presence == "absent" {
            if canonical_path
                .try_exists()
                .map_err(|_| io_error("publication basis absence lookup failed"))?
            {
                return Err(error(
                    GenericLineageStoreErrorKindV1::BasisMismatch,
                    "native publication expected an absent canonical basis",
                ));
            }
            (
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
                Value::Null,
            )
        } else {
            return Err(invalid_intent("unknown publication basis presence"));
        };
    #[cfg(unix)]
    let (platform_family, platform_primitive) = (
        "unix",
        if expected_presence == "present" {
            "renameat2_claim_publish_retained_dirfds"
        } else {
            "renameat2_noreplace_retained_dirfds"
        },
    );
    #[cfg(windows)]
    let (platform_family, platform_primitive) = (
        "windows",
        if expected_presence == "present" {
            "setfileinformationbyhandle_filerenameinfo_claim_publish"
        } else {
            "setfileinformationbyhandle_filerenameinfo_noreplace"
        },
    );
    #[cfg(not(any(unix, windows)))]
    return Err(error(
        GenericLineageStoreErrorKindV1::UnsupportedPlatform,
        "native publication requires Unix or Windows",
    ));
    finalize_control(
        json!({
            "schema_id": "handbook.generic-native-publication-observation",
            "schema_version": "1.0",
            "transaction_id": transaction_id,
            "intent_fingerprint": string_field(intent, "intent_fingerprint")?,
            "protocol_id": "atomic-displaced-basis-v1",
            "continuity_protocol": "retained-live-handle-v1",
            "platform_family": platform_family,
            "platform_primitive": platform_primitive,
            "expected_basis_presence": expected_presence,
            "expected_basis_fingerprint": expected_fingerprint,
            "expected_basis_byte_length": expected_byte_length,
            "expected_basis_identity_token": expected_identity,
            "expected_basis_version_token": expected_version,
            "expected_basis_platform_observation": expected_platform,
            "expected_basis_backup_ref": backup_ref,
            "expected_basis_backup_bytes_sha256": backup_digest,
            "expected_basis_backup_byte_length": backup_length,
            "expected_basis_backup_metadata_fingerprint": backup_metadata_fingerprint,
            "canonical_ref": canonical_ref,
            "candidate_ref": candidate_ref,
            "displaced_ref": displaced_ref,
            "replacement_bytes_sha256": string_field(canonical, "bytes_sha256")?,
            "replacement_byte_length": canonical.get("byte_length").cloned().unwrap_or(Value::Null),
            "replacement_identity_token": replacement.identity_token,
            "replacement_version_token": replacement.version_token,
            "replacement_platform_observation": replacement_platform
        }),
        "observation_fingerprint",
    )
}

fn read_native_publication_observation(
    transaction: &Path,
    intent: &Value,
    verified: &Value,
) -> Result<Value, GenericLineageStoreErrorV1> {
    let observation = read_control(
        &transaction.join("native-publication.json"),
        "observation_fingerprint",
    )?;
    validate_native_publication_observation(&observation, intent)?;
    if string_field(&observation, "observation_fingerprint")?
        != string_field(verified, "publication_observation_fingerprint")?
    {
        return Err(conflicting(
            "verified stage does not bind the exact native publication observation",
        ));
    }
    Ok(observation)
}

fn validate_native_publication_observation(
    observation: &Value,
    intent: &Value,
) -> Result<(), GenericLineageStoreErrorV1> {
    require_exact_fields(
        observation,
        &[
            "schema_id",
            "schema_version",
            "transaction_id",
            "intent_fingerprint",
            "protocol_id",
            "continuity_protocol",
            "platform_family",
            "platform_primitive",
            "expected_basis_presence",
            "expected_basis_fingerprint",
            "expected_basis_byte_length",
            "expected_basis_identity_token",
            "expected_basis_version_token",
            "expected_basis_platform_observation",
            "expected_basis_backup_ref",
            "expected_basis_backup_bytes_sha256",
            "expected_basis_backup_byte_length",
            "expected_basis_backup_metadata_fingerprint",
            "canonical_ref",
            "candidate_ref",
            "displaced_ref",
            "replacement_bytes_sha256",
            "replacement_byte_length",
            "replacement_identity_token",
            "replacement_version_token",
            "replacement_platform_observation",
            "observation_fingerprint",
        ],
        GenericLineageStoreErrorKindV1::ConflictingTransactionState,
    )?;
    if string_field(observation, "schema_id")? != "handbook.generic-native-publication-observation"
        || string_field(observation, "schema_version")? != "1.0"
        || string_field(observation, "transaction_id")? != string_field(intent, "transaction_id")?
        || string_field(observation, "intent_fingerprint")?
            != string_field(intent, "intent_fingerprint")?
        || string_field(observation, "protocol_id")? != "atomic-displaced-basis-v1"
        || string_field(observation, "continuity_protocol")? != "retained-live-handle-v1"
    {
        return Err(conflicting(
            "native publication observation does not bind the exact intent",
        ));
    }
    for (field, prefix) in [
        ("replacement_identity_token", "native-id-v1:"),
        ("replacement_version_token", "native-version-v1:"),
    ] {
        validate_native_token(string_field(observation, field)?, prefix)?;
    }
    let presence = string_field(observation, "expected_basis_presence")?;
    for field in [
        "expected_basis_fingerprint",
        "expected_basis_byte_length",
        "expected_basis_identity_token",
        "expected_basis_version_token",
        "expected_basis_platform_observation",
        "expected_basis_backup_ref",
        "expected_basis_backup_bytes_sha256",
        "expected_basis_backup_byte_length",
        "expected_basis_backup_metadata_fingerprint",
    ] {
        if observation.get(field) != intent.get(field)
            && !field.starts_with("expected_basis_backup")
            && field != "expected_basis_platform_observation"
        {
            return Err(conflicting(
                "native publication observation basis disagrees with intent",
            ));
        }
    }
    if presence == "present" {
        validate_sha256(string_field(observation, "expected_basis_fingerprint")?)?;
        validate_native_token(
            string_field(observation, "expected_basis_identity_token")?,
            "native-id-v1:",
        )?;
        validate_native_token(
            string_field(observation, "expected_basis_version_token")?,
            "native-version-v1:",
        )?;
    } else if presence != "absent" {
        return Err(conflicting("unknown native publication basis presence"));
    }
    validate_sha256(string_field(observation, "observation_fingerprint")?)?;
    let canonical = intent_outputs(intent)?
        .into_iter()
        .find(|output| string_field(output, "install_mode").ok() == Some("replace_if_current"))
        .ok_or_else(|| invalid_intent("promotion intent has no canonical replacement"))?;
    if string_field(observation, "canonical_ref")? != string_field(&canonical, "final_ref")? {
        return Err(conflicting(
            "native publication observation canonical ref disagrees with intent",
        ));
    }
    if string_field(observation, "replacement_bytes_sha256")?
        != string_field(&canonical, "bytes_sha256")?
        || observation.get("replacement_byte_length") != canonical.get("byte_length")
    {
        return Err(conflicting(
            "native publication replacement disagrees with intent",
        ));
    }
    Ok(())
}

fn build_verified_from_plan(
    intent: &Value,
    plan: &GenericCommitPlanV1,
    basis: &str,
    publication_observation_fingerprint: Option<&str>,
) -> Result<Value, GenericLineageStoreErrorV1> {
    let outputs = plan
        .outputs
        .iter()
        .enumerate()
        .map(|(ordinal, output)| {
            let staged_ref = if basis == "staging" {
                format!("staged/{}", staged_name(ordinal, &output.token))
            } else {
                output.final_ref.clone()
            };
            json!({
                "ordinal": ordinal,
                "staged_ref": staged_ref,
                "bytes_sha256": sha256_prefixed(&output.bytes),
                "byte_length": output.bytes.len()
            })
        })
        .collect::<Vec<_>>();
    let mut verified = json!({
        "schema_id": "handbook.generic-verified-stage",
        "schema_version": "1.0",
        "transaction_id": string_field(intent, "transaction_id")?,
        "intent_fingerprint": string_field(intent, "intent_fingerprint")?,
        "planned_outcome": "commit",
        "refusal": null,
        "verification_basis": basis,
        "staged_outputs": outputs
    });
    attach_publication_observation_fingerprint(
        &mut verified,
        intent,
        publication_observation_fingerprint,
    )?;
    finalize_control(verified, "verified_fingerprint")
}

fn build_verified_from_descriptors(
    intent: &Value,
    outputs: &[Value],
    basis: &str,
    publication_observation_fingerprint: Option<&str>,
) -> Result<Value, GenericLineageStoreErrorV1> {
    let staged = outputs
        .iter()
        .map(|output| {
            json!({
                "ordinal": output.get("ordinal").cloned().unwrap_or(Value::Null),
                "staged_ref": output.get("final_ref").cloned().unwrap_or(Value::Null),
                "bytes_sha256": output.get("bytes_sha256").cloned().unwrap_or(Value::Null),
                "byte_length": output.get("byte_length").cloned().unwrap_or(Value::Null)
            })
        })
        .collect::<Vec<_>>();
    let mut verified = json!({
        "schema_id": "handbook.generic-verified-stage",
        "schema_version": "1.0",
        "transaction_id": string_field(intent, "transaction_id")?,
        "intent_fingerprint": string_field(intent, "intent_fingerprint")?,
        "planned_outcome": "commit",
        "refusal": null,
        "verification_basis": basis,
        "staged_outputs": staged
    });
    attach_publication_observation_fingerprint(
        &mut verified,
        intent,
        publication_observation_fingerprint,
    )?;
    finalize_control(verified, "verified_fingerprint")
}

fn attach_publication_observation_fingerprint(
    verified: &mut Value,
    intent: &Value,
    fingerprint: Option<&str>,
) -> Result<(), GenericLineageStoreErrorV1> {
    let is_promotion_commit = operation_from_intent(intent)?
        == GenericArtifactOperationV1::ArtifactCandidatePromote
        && string_field(intent, "planned_outcome")? == "commit";
    if is_promotion_commit {
        let fingerprint = fingerprint.ok_or_else(|| {
            invalid_intent("promotion verification lacks its publication observation")
        })?;
        validate_sha256(fingerprint)?;
        verified.as_object_mut().expect("verified object").insert(
            "publication_observation_fingerprint".to_owned(),
            json!(fingerprint),
        );
    } else if fingerprint.is_some() {
        return Err(invalid_intent(
            "non-promotion verification carries a publication observation",
        ));
    }
    Ok(())
}

fn build_refusal_verified(intent: &Value) -> Result<Value, GenericLineageStoreErrorV1> {
    finalize_control(
        json!({
            "schema_id": "handbook.generic-verified-stage",
            "schema_version": "1.0",
            "transaction_id": string_field(intent, "transaction_id")?,
            "intent_fingerprint": string_field(intent, "intent_fingerprint")?,
            "planned_outcome": "refuse",
            "refusal": intent.get("refusal").cloned().unwrap_or(Value::Null),
            "verification_basis": "refusal_decision",
            "staged_outputs": []
        }),
        "verified_fingerprint",
    )
}

fn build_marker(intent: &Value, verified: &Value) -> Result<Value, GenericLineageStoreErrorV1> {
    let committed = string_field(intent, "planned_outcome")? == "commit";
    let outputs = intent_outputs(intent)?;
    let mut authoritative = Vec::new();
    let mut subordinate = Vec::new();
    if committed {
        for output in outputs {
            let authority = string_field(&output, "authority_class")?;
            let reference = string_field(&output, "final_ref")?;
            let fingerprint = if authority == "canonical_truth" {
                string_field(&output, "bytes_sha256")?.to_owned()
            } else {
                fingerprint_from_output_ref(reference)?
            };
            if authority == "subordinate_closure" {
                subordinate.push(json!({"ref": reference, "fingerprint": fingerprint}));
            } else {
                authoritative.push(json!({
                    "authority_class": authority,
                    "ref": reference,
                    "fingerprint": fingerprint
                }));
            }
        }
    }
    finalize_control(
        json!({
            "schema_id": "handbook.generic-commit-marker",
            "schema_version": "1.0",
            "transaction_id": string_field(intent, "transaction_id")?,
            "intent_fingerprint": string_field(intent, "intent_fingerprint")?,
            "verified_fingerprint": string_field(verified, "verified_fingerprint")?,
            "outcome": if committed { "committed" } else { "refused" },
            "refusal": intent.get("refusal").cloned().unwrap_or(Value::Null),
            "authoritative_outputs": authoritative,
            "subordinate_outputs": subordinate
        }),
        "marker_fingerprint",
    )
}

fn build_evidence(
    intent: &Value,
    verified: &Value,
    marker: &Value,
) -> Result<Value, GenericLineageStoreErrorV1> {
    let operation = operation_from_intent(intent)?;
    let mut evidence = json!({
        "schema_id": "handbook.generic-internal-transaction-evidence",
        "schema_version": "1.0",
        "transaction_id": string_field(intent, "transaction_id")?,
        "operation_id": operation.operation_id(),
        "request_fingerprint": string_field(intent, "request_fingerprint")?,
        "intent_fingerprint": string_field(intent, "intent_fingerprint")?,
        "verified_fingerprint": string_field(verified, "verified_fingerprint")?,
        "marker_fingerprint": string_field(marker, "marker_fingerprint")?,
        "outcome": string_field(marker, "outcome")?,
        "refusal": marker.get("refusal").cloned().unwrap_or(Value::Null),
        "authoritative_outputs": marker.get("authoritative_outputs").cloned().unwrap_or_else(|| json!([])),
        "subordinate_outputs": marker.get("subordinate_outputs").cloned().unwrap_or_else(|| json!([])),
        "atomic_group": operation.transaction_token()
    });
    if let Some(fingerprint) = marker.get("publication_result_fingerprint") {
        evidence.as_object_mut().expect("evidence object").insert(
            "publication_result_fingerprint".to_owned(),
            fingerprint.clone(),
        );
    }
    finalize_control(evidence, "evidence_fingerprint")
}

fn build_result(
    intent: &Value,
    marker: &Value,
    evidence: &Value,
) -> Result<Value, GenericLineageStoreErrorV1> {
    let operation = operation_from_intent(intent)?;
    let transaction_id = string_field(intent, "transaction_id")?;
    let evidence_ref = format!(
        ".handbook/state/transactions/{}/{}.committed/evidence.json",
        operation.transaction_family(),
        transaction_id
    );
    finalize_control(
        json!({
            "schema_id": "handbook.generic-domain-mutation-result",
            "schema_version": "1.0",
            "operation_id": operation.operation_id(),
            "transaction_id": transaction_id,
            "request_fingerprint": string_field(intent, "request_fingerprint")?,
            "outcome": string_field(marker, "outcome")?,
            "refusal": marker.get("refusal").cloned().unwrap_or(Value::Null),
            "authoritative_outputs": marker.get("authoritative_outputs").cloned().unwrap_or_else(|| json!([])),
            "internal_transaction_evidence_ref": evidence_ref,
            "internal_transaction_evidence_fingerprint": string_field(evidence, "evidence_fingerprint")?
        }),
        "result_fingerprint",
    )
}

fn build_ledger(
    intent: &Value,
    result: &Value,
    retained_until_utc: &str,
) -> Result<Value, GenericLineageStoreErrorV1> {
    let transaction_id = string_field(intent, "transaction_id")?;
    finalize_control(
        json!({
            "schema_id": "handbook.generic-domain-ledger-entry",
            "schema_version": "1.0",
            "repository_identity_fingerprint": string_field(intent, "repository_identity_fingerprint")?,
            "owner_contract_ref": GENERIC_ARTIFACT_OWNER_CONTRACT_REF,
            "owner_contract_subject_fingerprint": string_field(intent, "owner_contract_subject_fingerprint")?,
            "operation_id": string_field(intent, "operation_id")?,
            "domain_mutation_key_fingerprint": string_field(intent, "domain_mutation_key_fingerprint")?,
            "request_fingerprint": string_field(intent, "request_fingerprint")?,
            "state": "retained_result",
            "transaction_id": transaction_id,
            "result_ref": format!(
                ".handbook/state/idempotency/generic-artifact-operations/results/{transaction_id}.json"
            ),
            "result_fingerprint": string_field(result, "result_fingerprint")?,
            "retained_until_utc": retained_until_utc
        }),
        "entry_fingerprint",
    )
}

impl GenericArtifactLineageStoreV1 {
    pub(crate) fn new(repo_root: impl AsRef<Path>) -> Self {
        Self {
            repo_root: repo_root.as_ref().to_path_buf(),
        }
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn repo_root(&self) -> &Path {
        &self.repo_root
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn execute<F>(
        &self,
        request: GenericMutationRequestV1,
        evaluate: F,
    ) -> Result<GenericMutationExecutionV1, GenericLineageStoreErrorV1>
    where
        F: FnOnce(
            GenericEvaluationContextV1,
        ) -> Result<GenericPlannedOutcomeV1, GenericLineageStoreErrorV1>,
    {
        self.execute_inner(
            request,
            None,
            None,
            || Ok(()),
            |_| Ok(()),
            |_, _, _, _| Ok(()),
            evaluate,
            |_, _| Ok(()),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn execute_authorized<A, V, W, F, R>(
        &self,
        request: GenericMutationRequestV1,
        authorize: A,
        validate_persisted_authority: V,
        validate_persisted_output_authority: W,
        evaluate: F,
        revalidate: R,
    ) -> Result<GenericMutationExecutionV1, GenericLineageStoreErrorV1>
    where
        A: FnMut() -> Result<(), GenericLineageStoreErrorV1>,
        V: FnMut(&Value) -> Result<(), GenericLineageStoreErrorV1>,
        W: FnMut(&Value, usize, usize, &[u8]) -> Result<(), GenericLineageStoreErrorV1>,
        F: FnOnce(
            GenericEvaluationContextV1,
        ) -> Result<GenericPlannedOutcomeV1, GenericLineageStoreErrorV1>,
        R: FnOnce(
            &GenericEvaluationContextV1,
            &GenericPlannedOutcomeV1,
        ) -> Result<(), GenericLineageStoreErrorV1>,
    {
        self.execute_inner(
            request,
            None,
            None,
            authorize,
            validate_persisted_authority,
            validate_persisted_output_authority,
            evaluate,
            revalidate,
        )
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn execute_at_for_testing<F>(
        &self,
        request: GenericMutationRequestV1,
        now_utc: &str,
        evaluate: F,
    ) -> Result<GenericMutationExecutionV1, GenericLineageStoreErrorV1>
    where
        F: FnOnce(
            GenericEvaluationContextV1,
        ) -> Result<GenericPlannedOutcomeV1, GenericLineageStoreErrorV1>,
    {
        self.execute_inner(
            request,
            Some(now_utc),
            None,
            || Ok(()),
            |_| Ok(()),
            |_, _, _, _| Ok(()),
            evaluate,
            |_, _| Ok(()),
        )
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn execute_at_with_fault_for_testing<F>(
        &self,
        request: GenericMutationRequestV1,
        now_utc: &str,
        fault: GenericLineageFaultPointV1,
        evaluate: F,
    ) -> Result<GenericMutationExecutionV1, GenericLineageStoreErrorV1>
    where
        F: FnOnce(
            GenericEvaluationContextV1,
        ) -> Result<GenericPlannedOutcomeV1, GenericLineageStoreErrorV1>,
    {
        self.execute_inner(
            request,
            Some(now_utc),
            Some(fault),
            || Ok(()),
            |_| Ok(()),
            |_, _, _, _| Ok(()),
            evaluate,
            |_, _| Ok(()),
        )
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn recover(&self) -> Result<(), GenericLineageStoreErrorV1> {
        ensure_supported_platform()?;
        let _lock = GenericStoreLock::acquire(&self.repo_root)?;
        self.validate_inventory()?;
        self.recover_locked()
    }

    #[allow(dead_code)]
    pub(crate) fn evaluate_committed_read<T, F>(
        &self,
        evaluate: F,
    ) -> Result<T, GenericLineageStoreErrorV1>
    where
        F: FnOnce() -> Result<T, GenericLineageStoreErrorV1>,
    {
        ensure_supported_platform()?;
        let _lock = GenericStoreLock::acquire(&self.repo_root)?;
        self.validate_inventory()?;
        self.recover_locked()?;
        evaluate()
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn evaluate_committed_read_with<T, E, F, M, V, W>(
        &self,
        expected_repository_identity: &str,
        expected_owner_subject: &str,
        map_store_error: M,
        mut validate_persisted_authority: V,
        mut validate_persisted_output_authority: W,
        evaluate: F,
    ) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>,
        M: Fn(GenericLineageStoreErrorV1) -> E,
        V: FnMut(&Value) -> Result<(), GenericLineageStoreErrorV1>,
        W: FnMut(&Value, usize, usize, &[u8]) -> Result<(), GenericLineageStoreErrorV1>,
    {
        ensure_supported_platform().map_err(&map_store_error)?;
        let _lock = GenericStoreLock::acquire(&self.repo_root).map_err(&map_store_error)?;
        self.validate_inventory().map_err(&map_store_error)?;
        self.recover_locked_for_authority(
            expected_repository_identity,
            expected_owner_subject,
            &mut validate_persisted_authority,
            &mut validate_persisted_output_authority,
        )
        .map_err(&map_store_error)?;
        evaluate()
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn read_committed_authoritative(
        &self,
        relative_ref: &str,
        fingerprint: &str,
    ) -> Result<Vec<u8>, GenericLineageStoreErrorV1> {
        validate_safe_ref(relative_ref)?;
        validate_sha256(fingerprint)?;
        let _lock = GenericStoreLock::acquire(&self.repo_root)?;
        self.validate_inventory()?;
        self.recover_locked()?;
        if !self.authoritative_ref_is_committed(relative_ref, fingerprint)? {
            return Err(error(
                GenericLineageStoreErrorKindV1::RetainedResultMismatch,
                "authoritative output is not cited by an exact committed marker",
            ));
        }
        read_bounded_regular(&self.repo_root.join(relative_ref), MAX_RECORD_BYTES)
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn read_committed_closure(
        &self,
        authoritative_ref: &str,
        authoritative_fingerprint: &str,
        closure_ref: &str,
        closure_fingerprint: &str,
    ) -> Result<Vec<u8>, GenericLineageStoreErrorV1> {
        validate_safe_ref(authoritative_ref)?;
        validate_sha256(authoritative_fingerprint)?;
        validate_safe_ref(closure_ref)?;
        validate_sha256(closure_fingerprint)?;
        let _lock = GenericStoreLock::acquire(&self.repo_root)?;
        self.validate_inventory()?;
        self.recover_locked()?;
        if !self.authoritative_ref_is_committed(authoritative_ref, authoritative_fingerprint)? {
            return Err(error(
                GenericLineageStoreErrorKindV1::RetainedResultMismatch,
                "closure authority is not cited by an exact committed marker",
            ));
        }
        let authority_bytes =
            read_bounded_regular(&self.repo_root.join(authoritative_ref), MAX_RECORD_BYTES)?;
        let authority = parse_jcs_lf(&authority_bytes)?;
        if !value_contains_string(&authority, closure_ref)
            || !output_ref_matches_fingerprint(closure_ref, closure_fingerprint)
        {
            return Err(error(
                GenericLineageStoreErrorKindV1::RetainedResultMismatch,
                "committed authority does not cite the requested closure pair",
            ));
        }
        let bytes = read_bounded_regular(&self.repo_root.join(closure_ref), MAX_RECORD_BYTES)?;
        let value = parse_jcs_lf(&bytes)?;
        let computed = runtime_record_subject_fingerprint(&value)?
            .unwrap_or(sha256_prefixed(&canonical_json(&value)?));
        if computed != closure_fingerprint {
            return Err(error(
                GenericLineageStoreErrorKindV1::RetainedResultMismatch,
                "committed closure fingerprint does not recompute",
            ));
        }
        Ok(bytes)
    }

    pub(crate) fn read_committed_authoritative_during_evaluation(
        &self,
        relative_ref: &str,
        fingerprint: &str,
    ) -> Result<Vec<u8>, GenericLineageStoreErrorV1> {
        validate_safe_ref(relative_ref)?;
        validate_sha256(fingerprint)?;
        if !self.authoritative_ref_is_committed(relative_ref, fingerprint)? {
            return Err(error(
                GenericLineageStoreErrorKindV1::RetainedResultMismatch,
                "authoritative output is not cited by an exact committed marker",
            ));
        }
        read_bounded_regular(&self.repo_root.join(relative_ref), MAX_RECORD_BYTES)
    }

    pub(crate) fn read_committed_closure_during_evaluation(
        &self,
        authoritative_ref: &str,
        authoritative_fingerprint: &str,
        closure_ref: &str,
        closure_fingerprint: &str,
    ) -> Result<Vec<u8>, GenericLineageStoreErrorV1> {
        validate_safe_ref(authoritative_ref)?;
        validate_sha256(authoritative_fingerprint)?;
        validate_safe_ref(closure_ref)?;
        validate_sha256(closure_fingerprint)?;
        if !self.authoritative_ref_is_committed(authoritative_ref, authoritative_fingerprint)? {
            return Err(error(
                GenericLineageStoreErrorKindV1::RetainedResultMismatch,
                "closure authority is not cited by an exact committed marker",
            ));
        }
        let authority_bytes =
            read_bounded_regular(&self.repo_root.join(authoritative_ref), MAX_RECORD_BYTES)?;
        let authority = parse_jcs_lf(&authority_bytes)?;
        if !value_contains_string(&authority, closure_ref)
            || !output_ref_matches_fingerprint(closure_ref, closure_fingerprint)
        {
            return Err(error(
                GenericLineageStoreErrorKindV1::RetainedResultMismatch,
                "committed authority does not cite the requested closure pair",
            ));
        }
        let bytes = read_bounded_regular(&self.repo_root.join(closure_ref), MAX_RECORD_BYTES)?;
        let value = parse_jcs_lf(&bytes)?;
        let computed = runtime_record_subject_fingerprint(&value)?
            .unwrap_or(sha256_prefixed(&canonical_json(&value)?));
        if computed != closure_fingerprint {
            return Err(error(
                GenericLineageStoreErrorKindV1::RetainedResultMismatch,
                "committed closure fingerprint does not recompute",
            ));
        }
        Ok(bytes)
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn expire_retained_result_at_for_testing(
        &self,
        request: &GenericMutationRequestV1,
        now_utc: &str,
    ) -> Result<(), GenericLineageStoreErrorV1> {
        validate_utc(now_utc)?;
        let derived = validate_and_derive_request(request)?;
        let _lock = GenericStoreLock::acquire(&self.repo_root)?;
        self.validate_inventory()?;
        self.recover_locked()?;
        let path = self.ledger_path(&derived.key_fingerprint);
        let ledger = read_control(&path, "entry_fingerprint")?;
        require_ledger_scope(&ledger, &derived)?;
        let stored_request = string_field(&ledger, "request_fingerprint")?;
        if stored_request != derived.request_fingerprint {
            return Err(error(
                GenericLineageStoreErrorKindV1::IdempotencyConflict,
                "same domain key is already bound to a different request",
            ));
        }
        if string_field(&ledger, "state")? != "retained_result" {
            return Err(error(
                GenericLineageStoreErrorKindV1::RetainedResultMismatch,
                "only a retained result may become a tombstone",
            ));
        }
        let deadline = string_field(&ledger, "retained_until_utc")?;
        if parse_utc(now_utc)? <= parse_utc(deadline)? {
            return Err(error(
                GenericLineageStoreErrorKindV1::InvalidRequest,
                "retained result has not reached its expiry boundary",
            ));
        }
        let transaction_id = string_field(&ledger, "transaction_id")?;
        let tombstone = finalize_control(
            json!({
                "schema_id": "handbook.generic-domain-ledger-entry",
                "schema_version": "1.0",
                "repository_identity_fingerprint": derived.repository_identity_fingerprint,
                "owner_contract_ref": GENERIC_ARTIFACT_OWNER_CONTRACT_REF,
                "owner_contract_subject_fingerprint": derived.owner_contract_subject_fingerprint,
                "operation_id": derived.operation.operation_id(),
                "domain_mutation_key_fingerprint": derived.key_fingerprint,
                "request_fingerprint": derived.request_fingerprint,
                "state": "tombstone",
                "transaction_id": transaction_id,
                "result_ref": null,
                "result_fingerprint": null,
                "retained_until_utc": null
            }),
            "entry_fingerprint",
        )?;
        replace_durable(&path, &jcs_lf(&tombstone)?)
    }

    #[allow(clippy::too_many_arguments)]
    fn execute_inner<A, V, W, F, R>(
        &self,
        request: GenericMutationRequestV1,
        injected_now_utc: Option<&str>,
        fault: FaultSelection,
        mut authorize: A,
        mut validate_persisted_authority: V,
        mut validate_persisted_output_authority: W,
        evaluate: F,
        revalidate: R,
    ) -> Result<GenericMutationExecutionV1, GenericLineageStoreErrorV1>
    where
        A: FnMut() -> Result<(), GenericLineageStoreErrorV1>,
        V: FnMut(&Value) -> Result<(), GenericLineageStoreErrorV1>,
        W: FnMut(&Value, usize, usize, &[u8]) -> Result<(), GenericLineageStoreErrorV1>,
        F: FnOnce(
            GenericEvaluationContextV1,
        ) -> Result<GenericPlannedOutcomeV1, GenericLineageStoreErrorV1>,
        R: FnOnce(
            &GenericEvaluationContextV1,
            &GenericPlannedOutcomeV1,
        ) -> Result<(), GenericLineageStoreErrorV1>,
    {
        ensure_supported_platform()?;
        let derived = validate_and_derive_request(&request)?;
        let _lock = GenericStoreLock::acquire(&self.repo_root)?;
        self.validate_inventory()?;
        self.recover_locked_for_authority(
            &derived.repository_identity_fingerprint,
            &derived.owner_contract_subject_fingerprint,
            &mut validate_persisted_authority,
            &mut validate_persisted_output_authority,
        )?;
        authorize()?;

        if let Some(existing) = self.lookup_existing(&derived)? {
            return Ok(existing);
        }

        let sampled_now;
        let now_utc = if let Some(injected) = injected_now_utc {
            injected
        } else {
            sampled_now = current_utc()?;
            &sampled_now
        };
        validate_utc(now_utc)?;

        let sampled = (request.operation == GenericArtifactOperationV1::IntakeRecordAppend)
            .then(|| now_utc.to_owned());
        let evaluation_context = GenericEvaluationContextV1 {
            sampled_finalized_at_utc: sampled,
        };
        let planned = evaluate(evaluation_context.clone())?;
        validate_plan(&derived, &planned, now_utc)?;
        let mut basis_observation = if let GenericPlannedOutcomeV1::Commit(plan) = &planned {
            self.verify_compare_and_write_basis(plan, None)?
        } else {
            None
        };
        authorize()?;
        revalidate(&evaluation_context, &planned)?;
        if let (GenericPlannedOutcomeV1::Commit(plan), Some(observation)) =
            (&planned, basis_observation.as_ref())
        {
            basis_observation =
                self.verify_compare_and_write_basis(plan, Some(&observation.version_token))?;
        }
        if let GenericPlannedOutcomeV1::Commit(plan) = &planned {
            require_fresh_replacement_scratch_absent(&self.repo_root, plan)?;
            require_fresh_create_new_outputs_absent(&self.repo_root, plan)?;
        }

        let mut expected_basis_byte_length = None;
        let mut expected_basis_identity_token = None;
        let mut expected_basis_version_token = None;
        if let GenericPlannedOutcomeV1::Commit(plan) = &planned {
            if request.operation == GenericArtifactOperationV1::ArtifactCandidatePromote
                && plan.expected_basis_fingerprint.is_some()
            {
                let observation = basis_observation.as_ref().ok_or_else(|| {
                    invalid_plan("present promotion basis lacks its retained observation")
                })?;
                let guard = observation._guard.as_ref().ok_or_else(|| {
                    invalid_plan("present promotion basis lacks its retained file handle")
                })?;
                let metadata = guard
                    .metadata()
                    .map_err(|_| io_error("retained promotion basis metadata failed"))?;
                let canonical = plan
                    .outputs
                    .iter()
                    .find(|output| output.install_mode == GenericInstallModeV1::ReplaceIfCurrent)
                    .ok_or_else(|| invalid_plan("promotion plan has no canonical output"))?;
                let canonical_path = self.repo_root.join(&canonical.final_ref);
                let (identity_token, version_token) =
                    native_bound_tokens(&metadata, &canonical_path)?;
                if version_token != observation.version_token {
                    return Err(error(
                        GenericLineageStoreErrorKindV1::BasisMismatch,
                        "retained promotion basis version changed before intent establishment",
                    ));
                }
                expected_basis_byte_length = Some(metadata.len());
                expected_basis_identity_token = Some(identity_token);
                expected_basis_version_token = Some(version_token);
            }
        }
        let intent = build_intent(
            &derived,
            &planned,
            expected_basis_byte_length,
            expected_basis_identity_token.as_deref(),
            expected_basis_version_token.as_deref(),
        )?;
        let transaction_id = string_field(&intent, "transaction_id")?.to_owned();
        let established = self.establish_intent(&derived.key_fingerprint, &intent)?;
        maybe_fault(fault, FaultBoundary::EstablishedIntent)?;
        let pending =
            self.move_intent_to_pending(request.operation, &transaction_id, &established)?;
        maybe_fault(fault, FaultBoundary::PendingIntent)?;

        let disposition = match planned {
            GenericPlannedOutcomeV1::Commit(plan) => {
                let commit = self.commit_new_locked(
                    &pending,
                    &intent,
                    &plan,
                    basis_observation,
                    now_utc,
                    fault,
                );
                match commit {
                    Ok(()) => GenericExecutionDispositionV1::Committed,
                    Err(error)
                        if request.operation
                            == GenericArtifactOperationV1::ArtifactCandidatePromote
                            && error.kind == GenericLineageStoreErrorKindV1::BasisMismatch =>
                    {
                        self.recover_pending(&pending)?;
                        GenericExecutionDispositionV1::Refused
                    }
                    Err(error) => return Err(error),
                }
            }
            GenericPlannedOutcomeV1::Refuse(_) => {
                self.refuse_new_locked(&pending, &intent, now_utc, fault)?;
                GenericExecutionDispositionV1::Refused
            }
        };
        let result = self.read_result_for_transaction(request.operation, &transaction_id)?;
        Ok(GenericMutationExecutionV1 {
            result,
            disposition,
        })
    }

    fn lookup_existing(
        &self,
        derived: &DerivedRequest,
    ) -> Result<Option<GenericMutationExecutionV1>, GenericLineageStoreErrorV1> {
        let path = self.ledger_path(&derived.key_fingerprint);
        if !path
            .try_exists()
            .map_err(|_| io_error("ledger lookup failed"))?
        {
            return Ok(None);
        }
        let ledger = read_control(&path, "entry_fingerprint")?;
        self.require_exact_committed_chain(&ledger)?;
        require_ledger_scope(&ledger, derived)?;
        if string_field(&ledger, "request_fingerprint")? != derived.request_fingerprint {
            return Err(error(
                GenericLineageStoreErrorKindV1::IdempotencyConflict,
                "same domain key is already bound to a different request",
            ));
        }
        match string_field(&ledger, "state")? {
            "retained_result" => {
                let result_ref = string_field(&ledger, "result_ref")?;
                let result_fingerprint = string_field(&ledger, "result_fingerprint")?;
                let value = read_control(&self.repo_root.join(result_ref), "result_fingerprint")?;
                if string_field(&value, "result_fingerprint")? != result_fingerprint {
                    return Err(error(
                        GenericLineageStoreErrorKindV1::RetainedResultMismatch,
                        "retained result fingerprint disagrees with its ledger entry",
                    ));
                }
                let result: GenericDomainMutationResultV1 = serde_json::from_value(value)
                    .map_err(|_| retained_error("retained domain result is not exact-closed"))?;
                Ok(Some(GenericMutationExecutionV1 {
                    result,
                    disposition: GenericExecutionDispositionV1::Replayed,
                }))
            }
            "tombstone" => Err(error(
                GenericLineageStoreErrorKindV1::IdempotencyExpired,
                "the exact consumed domain key has expired",
            )),
            "active_hold" => Err(error(
                GenericLineageStoreErrorKindV1::ConflictingTransactionState,
                "active holds are intent projections and may not be persisted separately",
            )),
            _ => Err(retained_error("unknown domain ledger state")),
        }
    }

    fn establish_intent(
        &self,
        key_fingerprint: &str,
        intent: &Value,
    ) -> Result<Vec<u8>, GenericLineageStoreErrorV1> {
        let establishing = self.establishing_root();
        create_safe_directories(&self.repo_root, &establishing)?;
        let key_hex = fingerprint_hex(key_fingerprint)?;
        let writing = establishing.join(format!("{key_hex}.intent.writing"));
        let published = establishing.join(format!("{key_hex}.intent"));
        if writing
            .try_exists()
            .map_err(|_| io_error("scratch lookup failed"))?
            || published
                .try_exists()
                .map_err(|_| io_error("intent lookup failed"))?
        {
            return Err(error(
                GenericLineageStoreErrorKindV1::ConflictingTransactionState,
                "domain key already has establishing state",
            ));
        }
        let bytes = jcs_lf(intent)?;
        write_new_durable(&writing, &bytes)?;
        rename_store_path(&writing, &published)
            .map_err(|_| io_error("intent publication rename failed"))?;
        sync_directory(&establishing)?;
        Ok(bytes)
    }

    fn move_intent_to_pending(
        &self,
        operation: GenericArtifactOperationV1,
        transaction_id: &str,
        established_bytes: &[u8],
    ) -> Result<PathBuf, GenericLineageStoreErrorV1> {
        let family = self.transaction_family_root(operation);
        create_safe_directories(&self.repo_root, &family)?;
        let pending = family.join(format!("{transaction_id}.pending"));
        create_store_directory(&pending)?;
        sync_directory(&family)?;
        let key = read_control_bytes(established_bytes, "intent_fingerprint")?;
        let key_fingerprint = string_field(&key, "domain_mutation_key_fingerprint")?;
        let established = self
            .establishing_root()
            .join(format!("{}.intent", fingerprint_hex(key_fingerprint)?));
        rename_store_path(&established, &pending.join("intent.json"))
            .map_err(|_| io_error("established intent move failed"))?;
        sync_directory(&pending)?;
        sync_directory(&self.establishing_root())?;
        Ok(pending)
    }

    fn commit_new_locked(
        &self,
        pending: &Path,
        intent: &Value,
        plan: &GenericCommitPlanV1,
        basis_observation: Option<RetainedBasisObservation>,
        now_utc: &str,
        fault: FaultSelection,
    ) -> Result<(), GenericLineageStoreErrorV1> {
        let staged_root = pending.join("staged");
        create_store_directory(&staged_root)?;
        sync_directory(pending)?;
        for (ordinal, output) in plan.outputs.iter().enumerate() {
            let staged = staged_root.join(staged_name(ordinal, &output.token));
            write_new_durable(&staged, &output.bytes)?;
        }
        let mut prepared = plan
            .outputs
            .iter()
            .map(|_| None)
            .collect::<Vec<Option<PreparedReplacement>>>();
        let mut publication_observation = None;
        for (ordinal, output) in plan.outputs.iter().enumerate() {
            if output.install_mode != GenericInstallModeV1::ReplaceIfCurrent {
                continue;
            }
            let path = self.repo_root.join(&output.final_ref);
            create_safe_directories(
                &self.repo_root,
                path.parent()
                    .ok_or_else(|| unsafe_ref("output has no parent"))?,
            )?;
            let transaction_id = string_field(intent, "transaction_id")?;
            let candidate = path
                .parent()
                .ok_or_else(|| unsafe_ref("canonical output has no parent"))?
                .join(format!(
                    ".registry-brief.yaml.generic-publish-{transaction_id}.candidate"
                ));
            let (mut file, directory_guards) = create_new_replacement_guard(&candidate)?;
            file.write_all(&output.bytes)
                .map_err(|_| io_error("publication candidate write failed"))?;
            file.sync_all()
                .map_err(|_| io_error("publication candidate flush failed"))?;
            sync_directory(
                candidate
                    .parent()
                    .ok_or_else(|| unsafe_ref("publication candidate has no parent"))?,
            )?;
            let created = observe_prepared_replacement(
                candidate.clone(),
                file,
                directory_guards,
                &output.bytes,
            )?;
            let created_identity = created.identity_token.clone();
            drop(created);
            let (file, directory_guards) = open_replacement_guard(&candidate)?;
            let replacement =
                observe_prepared_replacement(candidate, file, directory_guards, &output.bytes)?;
            if replacement.identity_token != created_identity {
                return Err(conflicting(
                    "publication candidate changed identity after its write handle closed",
                ));
            }
            let publication = build_native_publication_observation(intent, &replacement)?;
            write_new_durable(
                &pending.join("native-publication.json"),
                &jcs_lf(&publication)?,
            )?;
            publication_observation = Some(publication);
            prepared[ordinal] = Some(replacement);
        }
        let verified = build_verified_from_plan(
            intent,
            plan,
            "staging",
            publication_observation
                .as_ref()
                .map(|value| string_field(value, "observation_fingerprint"))
                .transpose()?,
        )?;
        write_new_durable(&pending.join("verified.json"), &jcs_lf(&verified)?)?;
        maybe_fault(fault, FaultBoundary::Verified)?;
        let mut installed_guards = Vec::new();
        let mut basis_observation = basis_observation;
        for (ordinal, output) in plan.outputs.iter().enumerate() {
            installed_guards.push(
                self.install_output(
                    output,
                    plan.expected_basis_fingerprint.as_deref(),
                    intent
                        .get("expected_basis_version_token")
                        .and_then(Value::as_str),
                    prepared[ordinal].take(),
                    basis_observation.take(),
                    fault,
                    ordinal,
                )?,
            );
            maybe_fault(fault, FaultBoundary::Install(ordinal))?;
        }
        require_installed_guards_unchanged(&installed_guards)?;
        maybe_fault(fault, FaultBoundary::InstalledVerification)?;
        let publication_result = if let Some(observation) = publication_observation.as_ref() {
            let absent = || {
                json!({
                    "presence": "absent",
                    "semantic_binding": "absent",
                    "identity_token": null,
                    "version_token": null,
                    "version_transition": "not_applicable",
                    "bytes_sha256": null,
                    "byte_length": null,
                    "link_count": null,
                    "platform_observation": null
                })
            };
            let observed = |path: &Path,
                            semantic_binding: &str,
                            version_transition: &str|
             -> Result<Value, GenericLineageStoreErrorV1> {
                let (guard, bytes, identity_token, version_token) =
                    observe_retained_regular_file(path, MAX_RECORD_BYTES)?;
                let metadata = guard
                    .metadata()
                    .map_err(|_| io_error("native publication result metadata failed"))?;
                let (_, _, platform_observation) = native_metadata_subjects(&metadata, path)?;
                Ok(json!({
                    "presence": "regular",
                    "semantic_binding": semantic_binding,
                    "identity_token": identity_token,
                    "version_token": version_token,
                    "version_transition": version_transition,
                    "bytes_sha256": sha256_prefixed(&bytes),
                    "byte_length": bytes.len(),
                    "link_count": platform_observation["link_count"],
                    "platform_observation": platform_observation
                }))
            };
            let expected_presence = string_field(observation, "expected_basis_presence")?;
            let canonical_path = self
                .repo_root
                .join(string_field(observation, "canonical_ref")?);
            let candidate_path = self
                .repo_root
                .join(string_field(observation, "candidate_ref")?);
            let displaced_path = self
                .repo_root
                .join(string_field(observation, "displaced_ref")?);
            let candidate_before = json!({
                "presence": "regular",
                "semantic_binding": "replacement",
                "identity_token": observation["replacement_identity_token"],
                "version_token": observation["replacement_version_token"],
                "version_transition": "unchanged_before_first_move",
                "bytes_sha256": observation["replacement_bytes_sha256"],
                "byte_length": observation["replacement_byte_length"],
                "link_count": observation["replacement_platform_observation"]["link_count"],
                "platform_observation": observation["replacement_platform_observation"]
            });
            let canonical_before = if expected_presence == "present" {
                json!({
                    "presence": "regular",
                    "semantic_binding": "expected_basis",
                    "identity_token": observation["expected_basis_identity_token"],
                    "version_token": observation["expected_basis_version_token"],
                    "version_transition": "unchanged_before_first_move",
                    "bytes_sha256": observation["expected_basis_fingerprint"],
                    "byte_length": observation["expected_basis_byte_length"],
                    "link_count": observation["expected_basis_platform_observation"]["link_count"],
                    "platform_observation": observation["expected_basis_platform_observation"]
                })
            } else {
                absent()
            };
            let before = json!({
                "canonical": canonical_before,
                "candidate": candidate_before,
                "displaced": absent()
            });
            let canonical_after = observed(
                &canonical_path,
                "replacement",
                "primitive_mutated_same_object",
            )?;
            if canonical_after["identity_token"] != observation["replacement_identity_token"] {
                return Err(conflicting(
                    "native publication result canonical identity is not the replacement",
                ));
            }
            if candidate_path
                .try_exists()
                .map_err(|_| io_error("native result candidate lookup failed"))?
            {
                return Err(conflicting(
                    "native publication result retained a published candidate pathname",
                ));
            }
            let displaced_after = if expected_presence == "present" {
                let value = observed(
                    &displaced_path,
                    "expected_basis",
                    "primitive_mutated_same_object",
                )?;
                if value["identity_token"] != observation["expected_basis_identity_token"] {
                    return Err(conflicting(
                        "native publication result displaced identity is not the expected basis",
                    ));
                }
                value
            } else {
                if displaced_path
                    .try_exists()
                    .map_err(|_| io_error("native result displaced lookup failed"))?
                {
                    return Err(conflicting(
                        "unexpected displaced path exists after absent-basis publication",
                    ));
                }
                absent()
            };
            let after = json!({
                "canonical": canonical_after,
                "candidate": absent(),
                "displaced": displaced_after
            });
            let platform_family = string_field(observation, "platform_family")?;
            let continuity = |child: &str| {
                json!({
                    "protocol": "retained-live-handle-v1",
                    "platform_family": platform_family,
                    "source_handle_retained_through_destination_observation": true,
                    "source_identity_reverified_from_retained_handle": true,
                    "source_parent_handle_retained": true,
                    "destination_parent_handle_retained": true,
                    "destination_opened_from_retained_parent": true,
                    "destination_child_name": child,
                    "continuity_outcome": "destination_is_same_live_source_object",
                    "same_live_object_verified": true,
                    "numeric_identity_only": false,
                    "ancestor_chain_binding": if platform_family == "windows" {
                        "windows_retained_ancestors_no_share_delete_enumerated_child_ids_no_reparse"
                    } else {
                        "unix_openat2_beneath_retained_dirfds_no_symlinks"
                    },
                    "platform_binding": if platform_family == "windows" {
                        "setfileinformationbyhandle_filerenameinfo_rootdirectory_simple_child_replace_false"
                    } else {
                        "renameat2_retained_source_and_parent_dirfds"
                    }
                })
            };
            let canonical_child = Path::new(string_field(observation, "canonical_ref")?)
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| invalid_intent("canonical publication child is not UTF-8"))?;
            let native_call_trace = if expected_presence == "present" {
                let displaced_child = Path::new(string_field(observation, "displaced_ref")?)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .ok_or_else(|| invalid_intent("displaced publication child is not UTF-8"))?;
                let intermediate = json!({
                    "canonical": absent(),
                    "candidate": before["candidate"],
                    "displaced": after["displaced"]
                });
                json!([
                    {
                        "ordinal": 0,
                        "action": "claim_canonical_to_displaced",
                        "outcome": "success",
                        "error_code": null,
                        "before": before,
                        "after": intermediate,
                        "continuity_proof": continuity(displaced_child)
                    },
                    {
                        "ordinal": 1,
                        "action": "publish_replacement_to_canonical",
                        "outcome": "success",
                        "error_code": null,
                        "before": intermediate,
                        "after": after,
                        "continuity_proof": continuity(canonical_child)
                    }
                ])
            } else {
                json!([{
                    "ordinal": 0,
                    "action": "publish_replacement_to_canonical",
                    "outcome": "success",
                    "error_code": null,
                    "before": before,
                    "after": after,
                    "continuity_proof": continuity(canonical_child)
                }])
            };
            let result = finalize_control(
                json!({
                    "schema_id": "handbook.generic-native-publication-result",
                    "schema_version": "1.0",
                    "transaction_id": observation["transaction_id"],
                    "intent_fingerprint": observation["intent_fingerprint"],
                    "observation_fingerprint": observation["observation_fingerprint"],
                    "verified_fingerprint": string_field(&verified, "verified_fingerprint")?,
                    "continuity_protocol": observation["continuity_protocol"],
                    "continuity_completion": "all_native_moves_proved_while_source_handles_live",
                    "platform_family": observation["platform_family"],
                    "platform_primitive": observation["platform_primitive"],
                    "expected_basis_presence": observation["expected_basis_presence"],
                    "expected_basis_fingerprint": observation["expected_basis_fingerprint"],
                    "expected_basis_byte_length": observation["expected_basis_byte_length"],
                    "expected_basis_identity_token": observation["expected_basis_identity_token"],
                    "expected_basis_version_token": observation["expected_basis_version_token"],
                    "expected_basis_platform_observation": observation["expected_basis_platform_observation"],
                    "expected_basis_backup_ref": observation["expected_basis_backup_ref"],
                    "expected_basis_backup_bytes_sha256": observation["expected_basis_backup_bytes_sha256"],
                    "expected_basis_backup_byte_length": observation["expected_basis_backup_byte_length"],
                    "expected_basis_backup_metadata_fingerprint": observation["expected_basis_backup_metadata_fingerprint"],
                    "replacement_bytes_sha256": observation["replacement_bytes_sha256"],
                    "replacement_byte_length": observation["replacement_byte_length"],
                    "replacement_identity_token": observation["replacement_identity_token"],
                    "replacement_version_token": observation["replacement_version_token"],
                    "replacement_platform_observation": observation["replacement_platform_observation"],
                    "native_call_trace": native_call_trace,
                    "canonical_observation": after["canonical"],
                    "candidate_observation": after["candidate"],
                    "displaced_observation": after["displaced"],
                    "basis_disposition": if expected_presence == "present" {
                        "expected_basis_displaced"
                    } else {
                        "expected_absent_confirmed"
                    },
                    "publication_disposition": "authorized",
                    "reader_authority": "withheld_until_commit_marker",
                    "cleanup_authority": "forbidden"
                }),
                "result_fingerprint",
            )?;
            write_new_durable(
                &pending.join("native-publication-result.json"),
                &jcs_lf(&result)?,
            )?;
            Some(result)
        } else {
            None
        };
        let mut marker = build_marker(intent, &verified)?;
        if let Some(result) = publication_result.as_ref() {
            let object = marker.as_object_mut().expect("marker object");
            object.remove("marker_fingerprint");
            object.insert(
                "publication_result_fingerprint".to_owned(),
                json!(string_field(result, "result_fingerprint")?),
            );
            marker = finalize_control(marker, "marker_fingerprint")?;
        }
        let marker_path = pending.join("commit-marker.json");
        write_new_durable(&marker_path, &jcs_lf(&marker)?)?;
        if let Err(error) = require_installed_guards_unchanged(&installed_guards) {
            remove_store_file(&marker_path)?;
            return Err(error);
        }
        maybe_fault(fault, FaultBoundary::Marker)?;
        self.finish_suffix(pending, intent, &verified, &marker, now_utc, fault)
    }

    fn refuse_new_locked(
        &self,
        pending: &Path,
        intent: &Value,
        now_utc: &str,
        fault: FaultSelection,
    ) -> Result<(), GenericLineageStoreErrorV1> {
        let verified = build_refusal_verified(intent)?;
        write_new_durable(&pending.join("verified.json"), &jcs_lf(&verified)?)?;
        maybe_fault(fault, FaultBoundary::Verified)?;
        let marker = build_marker(intent, &verified)?;
        write_new_durable(&pending.join("commit-marker.json"), &jcs_lf(&marker)?)?;
        maybe_fault(fault, FaultBoundary::Marker)?;
        self.finish_suffix(pending, intent, &verified, &marker, now_utc, fault)
    }

    fn finish_suffix(
        &self,
        pending: &Path,
        intent: &Value,
        verified: &Value,
        marker: &Value,
        now_utc: &str,
        fault: FaultSelection,
    ) -> Result<(), GenericLineageStoreErrorV1> {
        let evidence = build_evidence(intent, verified, marker)?;
        let evidence_path = pending.join("evidence.json");
        write_new_or_equal(&evidence_path, &jcs_lf(&evidence)?)?;
        maybe_fault(fault, FaultBoundary::Evidence)?;

        let result = build_result(intent, marker, &evidence)?;
        let operation = operation_from_intent(intent)?;
        let transaction_id = string_field(intent, "transaction_id")?;
        let result_path = self.result_path(transaction_id);
        create_safe_directories(&self.repo_root, result_path.parent().unwrap())?;
        write_new_or_equal(&result_path, &jcs_lf(&result)?)?;
        maybe_fault(fault, FaultBoundary::Result)?;

        let retained_until = format_utc(parse_utc(now_utc)? + RETAINED_RESULT_SECONDS)?;
        let ledger = build_ledger(intent, &result, &retained_until)?;
        let key_fingerprint = string_field(intent, "domain_mutation_key_fingerprint")?;
        let ledger_path = self.ledger_path(key_fingerprint);
        create_safe_directories(&self.repo_root, ledger_path.parent().unwrap())?;
        write_new_or_equal(&ledger_path, &jcs_lf(&ledger)?)?;
        maybe_fault(fault, FaultBoundary::Ledger)?;

        let family = self.transaction_family_root(operation);
        let committed = family.join(format!("{transaction_id}.committed"));
        rename_store_path(pending, &committed)?;
        sync_directory(&family)?;
        maybe_fault(fault, FaultBoundary::CommittedRename)
    }

    #[allow(clippy::too_many_arguments)]
    fn install_output(
        &self,
        output: &GenericPlannedOutputV1,
        expected_basis_fingerprint: Option<&str>,
        expected_basis_version_token: Option<&str>,
        prepared: Option<PreparedReplacement>,
        retained_basis: Option<RetainedBasisObservation>,
        fault: FaultSelection,
        ordinal: usize,
    ) -> Result<InstalledOutputGuard, GenericLineageStoreErrorV1> {
        let path = self.repo_root.join(&output.final_ref);
        let parent = path
            .parent()
            .ok_or_else(|| unsafe_ref("output has no parent"))?;
        create_safe_directories(&self.repo_root, parent)?;
        match output.install_mode {
            GenericInstallModeV1::CreateNew => {
                write_new_durable(&path, &output.bytes)?;
                observe_installed_output(&path, &output.bytes, None)
            }
            GenericInstallModeV1::ReplaceIfCurrent => {
                let replacement = prepared.ok_or_else(|| {
                    conflicting("canonical replacement lacks its retained scratch identity")
                })?;
                require_prepared_replacement_unchanged(&replacement, &output.bytes)?;
                let final_basis = verify_compare_and_write_path(
                    &path,
                    expected_basis_fingerprint,
                    expected_basis_version_token,
                )?;
                maybe_fault(fault, FaultBoundary::CanonicalScratch(ordinal))?;
                drop(retained_basis);
                drop(final_basis);
                publish_replacement(
                    &path,
                    replacement,
                    &output.bytes,
                    expected_basis_fingerprint,
                    expected_basis_version_token,
                )
            }
        }
    }

    fn verify_compare_and_write_basis(
        &self,
        plan: &GenericCommitPlanV1,
        expected_version_token: Option<&str>,
    ) -> Result<Option<RetainedBasisObservation>, GenericLineageStoreErrorV1> {
        let Some(canonical) = plan
            .outputs
            .iter()
            .find(|output| output.install_mode == GenericInstallModeV1::ReplaceIfCurrent)
        else {
            return Ok(None);
        };
        let path = self.repo_root.join(&canonical.final_ref);
        verify_compare_and_write_path(
            &path,
            plan.expected_basis_fingerprint.as_deref(),
            expected_version_token,
        )
        .map(Some)
    }

    fn recover_locked(&self) -> Result<(), GenericLineageStoreErrorV1> {
        self.validate_recoverable_reachability()?;
        self.recover_establishing()?;
        for operation in [
            GenericArtifactOperationV1::IntakeRecordAppend,
            GenericArtifactOperationV1::ArtifactCandidateAppend,
            GenericArtifactOperationV1::ArtifactCandidatePromote,
        ] {
            let family = self.transaction_family_root(operation);
            if !family
                .try_exists()
                .map_err(|_| io_error("journal family lookup failed"))?
            {
                continue;
            }
            let mut pending = read_dir_paths(&family)?
                .into_iter()
                .filter(|path| path.extension().is_some_and(|ext| ext == "pending"))
                .collect::<Vec<_>>();
            pending.sort();
            for path in pending {
                self.recover_pending(&path)?;
            }
            let mut committed = read_dir_paths(&family)?
                .into_iter()
                .filter(|path| path.extension().is_some_and(|ext| ext == "committed"))
                .collect::<Vec<_>>();
            committed.sort();
            for path in committed {
                self.verify_committed(&path)?;
            }
        }
        self.validate_committed_reachability()
    }

    fn recover_locked_for_authority<V, W>(
        &self,
        expected_repository_identity: &str,
        expected_owner_subject: &str,
        validate_persisted_authority: &mut V,
        validate_persisted_output_authority: &mut W,
    ) -> Result<(), GenericLineageStoreErrorV1>
    where
        V: FnMut(&Value) -> Result<(), GenericLineageStoreErrorV1>,
        W: FnMut(&Value, usize, usize, &[u8]) -> Result<(), GenericLineageStoreErrorV1>,
    {
        validate_sha256(expected_repository_identity)?;
        validate_sha256(expected_owner_subject)?;
        self.require_journal_authority(
            expected_repository_identity,
            expected_owner_subject,
            validate_persisted_authority,
            validate_persisted_output_authority,
        )?;
        self.recover_locked()?;
        self.require_journal_authority(
            expected_repository_identity,
            expected_owner_subject,
            validate_persisted_authority,
            validate_persisted_output_authority,
        )
    }

    fn require_journal_authority<V, W>(
        &self,
        expected_repository_identity: &str,
        expected_owner_subject: &str,
        validate_persisted_authority: &mut V,
        validate_persisted_output_authority: &mut W,
    ) -> Result<(), GenericLineageStoreErrorV1>
    where
        V: FnMut(&Value) -> Result<(), GenericLineageStoreErrorV1>,
        W: FnMut(&Value, usize, usize, &[u8]) -> Result<(), GenericLineageStoreErrorV1>,
    {
        let mut intent_paths = Vec::new();
        let establishing = self.establishing_root();
        if establishing.exists() {
            for path in read_dir_paths(&establishing)? {
                if file_name(&path)?.ends_with(".intent") {
                    intent_paths.push(path);
                }
            }
        }
        let transactions = self.repo_root.join(".handbook/state/transactions");
        if transactions.exists() {
            for family in read_dir_paths(&transactions)? {
                if file_name(&family)? == "registry" {
                    continue;
                }
                for transaction in read_dir_paths(&family)? {
                    let intent = transaction.join("intent.json");
                    if intent.exists() {
                        intent_paths.push(intent);
                    }
                }
            }
        }
        intent_paths.sort();
        for path in intent_paths {
            let intent = read_control(&path, "intent_fingerprint")?;
            validate_intent(&intent)?;
            if string_field(&intent, "repository_identity_fingerprint")?
                != expected_repository_identity
                || string_field(&intent, "owner_contract_subject_fingerprint")?
                    != expected_owner_subject
            {
                return Err(error(
                    GenericLineageStoreErrorKindV1::InvalidRequest,
                    "generic journal identity or owner disagrees with current authority",
                ));
            }
            validate_persisted_authority(&intent)?;
            self.require_available_output_authority(
                &path,
                &intent,
                validate_persisted_output_authority,
            )?;
        }
        Ok(())
    }

    fn require_available_output_authority<W>(
        &self,
        intent_path: &Path,
        intent: &Value,
        validate_persisted_output_authority: &mut W,
    ) -> Result<(), GenericLineageStoreErrorV1>
    where
        W: FnMut(&Value, usize, usize, &[u8]) -> Result<(), GenericLineageStoreErrorV1>,
    {
        if file_name(intent_path)? != "intent.json"
            || string_field(intent, "planned_outcome")? != "commit"
        {
            return Ok(());
        }
        let transaction = intent_path
            .parent()
            .ok_or_else(|| invalid_intent("intent has no transaction directory"))?;
        let outputs = intent_outputs(intent)?;
        let final_ordinal = outputs
            .len()
            .checked_sub(1)
            .ok_or_else(|| invalid_intent("commit intent has no outputs"))?;
        for (ordinal, descriptor) in outputs.iter().enumerate() {
            let staged = transaction
                .join("staged")
                .join(staged_name(ordinal, string_field(descriptor, "token")?));
            let installed = self.repo_root.join(string_field(descriptor, "final_ref")?);
            for (path, is_installed) in [(&staged, false), (&installed, true)] {
                if path
                    .try_exists()
                    .map_err(|_| io_error("persisted output authority lookup failed"))?
                {
                    let bytes = read_bounded_regular(path, MAX_RECORD_BYTES)?;
                    if is_installed
                        && string_field(descriptor, "install_mode")? == "replace_if_current"
                        && (string_field(descriptor, "bytes_sha256")? != sha256_prefixed(&bytes)
                            || descriptor.get("byte_length").and_then(Value::as_u64)
                                != Some(bytes.len() as u64))
                    {
                        continue;
                    }
                    validate_persisted_output_bytes(
                        intent,
                        ordinal,
                        final_ordinal,
                        descriptor,
                        &bytes,
                    )?;
                    validate_persisted_output_authority(intent, ordinal, final_ordinal, &bytes)?;
                }
            }
        }
        Ok(())
    }

    fn recover_establishing(&self) -> Result<(), GenericLineageStoreErrorV1> {
        let root = self.establishing_root();
        if !root
            .try_exists()
            .map_err(|_| io_error("establishing lookup failed"))?
        {
            return Ok(());
        }
        let mut entries = read_dir_paths(&root)?;
        entries.sort();
        for path in entries {
            let name = file_name(&path)?;
            if name.ends_with(".intent.writing") {
                remove_store_file(&path)?;
                sync_directory(&root)?;
                continue;
            }
            if !name.ends_with(".intent") {
                return Err(conflicting("unknown establishing entry"));
            }
            let intent = read_control(&path, "intent_fingerprint")?;
            validate_intent(&intent)?;
            let expected_name = format!(
                "{}.intent",
                fingerprint_hex(string_field(&intent, "domain_mutation_key_fingerprint")?)?
            );
            if name != expected_name {
                return Err(invalid_intent(
                    "establishing filename does not bind the domain mutation key",
                ));
            }
            let operation = operation_from_intent(&intent)?;
            let transaction_id = string_field(&intent, "transaction_id")?;
            let family = self.transaction_family_root(operation);
            create_safe_directories(&self.repo_root, &family)?;
            let pending = family.join(format!("{transaction_id}.pending"));
            if pending
                .try_exists()
                .map_err(|_| io_error("pending lookup failed"))?
            {
                if !read_dir_paths(&pending)?.is_empty() {
                    return Err(conflicting(
                        "established intent may resume only into an exact empty pending shell",
                    ));
                }
            } else {
                create_store_directory(&pending)?;
                sync_directory(&family)?;
            }
            rename_store_path(&path, &pending.join("intent.json"))
                .map_err(|_| io_error("recovery intent move failed"))?;
            sync_directory(&pending)?;
            sync_directory(&root)?;
        }
        Ok(())
    }

    fn recover_pending(&self, pending: &Path) -> Result<(), GenericLineageStoreErrorV1> {
        let intent = read_control(&pending.join("intent.json"), "intent_fingerprint")?;
        validate_intent(&intent)?;
        validate_transaction_directory(pending, &intent, "pending")?;
        let planned_outcome = string_field(&intent, "planned_outcome")?;
        let now = current_utc()?;
        if planned_outcome == "refuse" {
            let verified = ensure_refusal_verified(pending, &intent)?;
            let marker = ensure_marker(pending, &intent, &verified)?;
            return self.finish_suffix_recovery(pending, &intent, &verified, &marker, &now);
        }

        let outputs = intent_outputs(&intent)?;
        let marker_path = pending.join("commit-marker.json");
        if marker_path
            .try_exists()
            .map_err(|_| io_error("marker lookup failed"))?
        {
            let verified = read_control(&pending.join("verified.json"), "verified_fingerprint")?;
            let marker = read_control(&marker_path, "marker_fingerprint")?;
            validate_verified(&verified, &intent)?;
            validate_marker(&marker, &intent, &verified)?;
            let guards = self.verify_installed_outputs(pending, &intent, &verified, &outputs)?;
            require_installed_guards_unchanged(&guards)?;
            return self.finish_suffix_recovery(pending, &intent, &verified, &marker, &now);
        }

        let verified_path = pending.join("verified.json");
        let verified = if verified_path
            .try_exists()
            .map_err(|_| io_error("verified lookup failed"))?
        {
            let value = read_control(&verified_path, "verified_fingerprint")?;
            validate_verified(&value, &intent)?;
            self.verify_staging(pending, &intent, &outputs, &value)?;
            value
        } else {
            let publication_fingerprint = if operation_from_intent(&intent)?
                == GenericArtifactOperationV1::ArtifactCandidatePromote
            {
                let observation = read_control(
                    &pending.join("native-publication.json"),
                    "observation_fingerprint",
                )?;
                validate_native_publication_observation(&observation, &intent)?;
                Some(string_field(&observation, "observation_fingerprint")?.to_owned())
            } else {
                None
            };
            let value = build_verified_from_descriptors(
                &intent,
                &outputs,
                "staging",
                publication_fingerprint.as_deref(),
            )?;
            self.verify_staging(pending, &intent, &outputs, &value)?;
            write_new_durable(&verified_path, &jcs_lf(&value)?)?;
            value
        };
        let promotion_commit =
            operation_from_intent(&intent)? == GenericArtifactOperationV1::ArtifactCandidatePromote;
        let installed = if promotion_commit {
            let publication = read_native_publication_observation(pending, &intent, &verified)?;
            let mut installed = BTreeSet::new();
            for (ordinal, descriptor) in outputs.iter().enumerate() {
                let path = self.repo_root.join(string_field(descriptor, "final_ref")?);
                let Some(observed) = path
                    .try_exists()
                    .map_err(|_| io_error("promotion recovery output lookup failed"))?
                    .then(|| read_bounded_regular(&path, MAX_RECORD_BYTES))
                    .transpose()?
                else {
                    continue;
                };
                if string_field(descriptor, "bytes_sha256")? == sha256_prefixed(&observed)
                    && descriptor.get("byte_length").and_then(Value::as_u64)
                        == Some(observed.len() as u64)
                {
                    if string_field(descriptor, "install_mode")? == "replace_if_current" {
                        let (identity, _) = native_path_tokens(&path)?;
                        if identity != string_field(&publication, "replacement_identity_token")? {
                            continue;
                        }
                    }
                    validate_persisted_output_bytes(
                        &intent,
                        ordinal,
                        outputs.len() - 1,
                        descriptor,
                        &observed,
                    )?;
                    installed.insert(ordinal);
                } else if string_field(descriptor, "install_mode")? == "create_new" {
                    return Err(conflicting(
                        "preexisting immutable recovery destination has unequal bytes",
                    ));
                }
            }
            installed
        } else {
            self.recovery_installed_output_set(pending, &intent, &verified, &outputs)?
        };
        let publication_result_preexists = promotion_commit
            && pending
                .join("native-publication-result.json")
                .try_exists()
                .map_err(|_| io_error("native publication result lookup failed"))?;
        if promotion_commit && installed.contains(&0) && !publication_result_preexists {
            return Err(conflicting(
                "markerless installed publication lacks its durable native result",
            ));
        }

        let mut installed_guards = Vec::new();
        for (ordinal, descriptor) in outputs.iter().enumerate() {
            if installed.contains(&ordinal) {
                continue;
            }
            let staged = pending
                .join("staged")
                .join(staged_name(ordinal, string_field(descriptor, "token")?));
            let bytes = read_bounded_regular(&staged, MAX_RECORD_BYTES)?;
            validate_persisted_output_bytes(
                &intent,
                ordinal,
                outputs.len() - 1,
                descriptor,
                &bytes,
            )?;
            match self.install_descriptor(pending, &intent, &verified, descriptor, &bytes) {
                Ok(guard) => installed_guards.push(guard),
                Err(error)
                    if promotion_commit
                        && ordinal == 0
                        && error.kind == GenericLineageStoreErrorKindV1::BasisMismatch =>
                {
                    let publication =
                        read_native_publication_observation(pending, &intent, &verified)?;
                    let canonical_path = self
                        .repo_root
                        .join(string_field(&publication, "canonical_ref")?);
                    let candidate_path = self
                        .repo_root
                        .join(string_field(&publication, "candidate_ref")?);
                    let displaced_path = self
                        .repo_root
                        .join(string_field(&publication, "displaced_ref")?);
                    if string_field(&publication, "expected_basis_presence")? != "absent"
                        || !canonical_path
                            .try_exists()
                            .map_err(|_| io_error("conflict canonical lookup failed"))?
                        || !candidate_path
                            .try_exists()
                            .map_err(|_| io_error("conflict candidate lookup failed"))?
                        || displaced_path
                            .try_exists()
                            .map_err(|_| io_error("conflict displaced lookup failed"))?
                    {
                        return Err(error);
                    }
                    let (_canonical_guard, canonical_bytes, canonical_identity, canonical_version) =
                        observe_retained_regular_file(&canonical_path, MAX_RECORD_BYTES)?;
                    let canonical_metadata = fs::symlink_metadata(&canonical_path)
                        .map_err(|_| io_error("conflict canonical metadata failed"))?;
                    let (_, _, canonical_platform) =
                        native_metadata_subjects(&canonical_metadata, &canonical_path)?;
                    let (candidate_file, candidate_guards) =
                        open_replacement_guard(&candidate_path)?;
                    let candidate = observe_prepared_replacement(
                        candidate_path.clone(),
                        candidate_file,
                        candidate_guards,
                        &bytes,
                    )?;
                    if candidate.identity_token
                        != string_field(&publication, "replacement_identity_token")?
                        || candidate.version_token
                            != string_field(&publication, "replacement_version_token")?
                    {
                        return Err(conflicting(
                            "publication conflict candidate no longer equals its durable binding",
                        ));
                    }
                    let absent = json!({
                        "presence": "absent",
                        "semantic_binding": "absent",
                        "identity_token": null,
                        "version_token": null,
                        "version_transition": "not_applicable",
                        "bytes_sha256": null,
                        "byte_length": null,
                        "link_count": null,
                        "platform_observation": null
                    });
                    let canonical_observation = json!({
                        "presence": "regular",
                        "semantic_binding": "unexpected_basis",
                        "identity_token": canonical_identity,
                        "version_token": canonical_version,
                        "version_transition": "external_object",
                        "bytes_sha256": sha256_prefixed(&canonical_bytes),
                        "byte_length": canonical_bytes.len(),
                        "link_count": canonical_platform["link_count"],
                        "platform_observation": canonical_platform
                    });
                    let candidate_observation = json!({
                        "presence": "regular",
                        "semantic_binding": "replacement",
                        "identity_token": publication["replacement_identity_token"],
                        "version_token": publication["replacement_version_token"],
                        "version_transition": "unchanged_before_first_move",
                        "bytes_sha256": publication["replacement_bytes_sha256"],
                        "byte_length": publication["replacement_byte_length"],
                        "link_count": publication["replacement_platform_observation"]["link_count"],
                        "platform_observation": publication["replacement_platform_observation"]
                    });
                    let native_result = finalize_control(
                        json!({
                            "schema_id": "handbook.generic-native-publication-result",
                            "schema_version": "1.0",
                            "transaction_id": publication["transaction_id"],
                            "intent_fingerprint": publication["intent_fingerprint"],
                            "observation_fingerprint": publication["observation_fingerprint"],
                            "verified_fingerprint": string_field(&verified, "verified_fingerprint")?,
                            "continuity_protocol": publication["continuity_protocol"],
                            "continuity_completion": "no_native_move_invoked",
                            "platform_family": publication["platform_family"],
                            "platform_primitive": publication["platform_primitive"],
                            "expected_basis_presence": publication["expected_basis_presence"],
                            "expected_basis_fingerprint": publication["expected_basis_fingerprint"],
                            "expected_basis_byte_length": publication["expected_basis_byte_length"],
                            "expected_basis_identity_token": publication["expected_basis_identity_token"],
                            "expected_basis_version_token": publication["expected_basis_version_token"],
                            "expected_basis_platform_observation": publication["expected_basis_platform_observation"],
                            "expected_basis_backup_ref": publication["expected_basis_backup_ref"],
                            "expected_basis_backup_bytes_sha256": publication["expected_basis_backup_bytes_sha256"],
                            "expected_basis_backup_byte_length": publication["expected_basis_backup_byte_length"],
                            "expected_basis_backup_metadata_fingerprint": publication["expected_basis_backup_metadata_fingerprint"],
                            "replacement_bytes_sha256": publication["replacement_bytes_sha256"],
                            "replacement_byte_length": publication["replacement_byte_length"],
                            "replacement_identity_token": publication["replacement_identity_token"],
                            "replacement_version_token": publication["replacement_version_token"],
                            "replacement_platform_observation": publication["replacement_platform_observation"],
                            "native_call_trace": [],
                            "canonical_observation": canonical_observation,
                            "candidate_observation": candidate_observation,
                            "displaced_observation": absent,
                            "basis_disposition": "competing_basis_preserved",
                            "publication_disposition": "refused_basis_conflict",
                            "reader_authority": "withheld_until_commit_marker",
                            "cleanup_authority": "forbidden"
                        }),
                        "result_fingerprint",
                    )?;
                    write_new_or_equal(
                        &pending.join("native-publication-result.json"),
                        &jcs_lf(&native_result)?,
                    )?;
                    let refusal = EstablishedRefusalV1 {
                        code: EstablishedRefusalCodeV1::PublicationBasisConflict,
                        layer: GenericRefusalLayerV1::Publication,
                        expected_fingerprint: None,
                        observed_fingerprint: Some(sha256_prefixed(&canonical_bytes)),
                    };
                    validate_established_refusal(&refusal)?;
                    let marker = finalize_control(
                        json!({
                            "schema_id": "handbook.generic-commit-marker",
                            "schema_version": "1.0",
                            "transaction_id": string_field(&intent, "transaction_id")?,
                            "intent_fingerprint": string_field(&intent, "intent_fingerprint")?,
                            "verified_fingerprint": string_field(&verified, "verified_fingerprint")?,
                            "publication_result_fingerprint": string_field(&native_result, "result_fingerprint")?,
                            "outcome": "refused",
                            "refusal": refusal,
                            "authoritative_outputs": [],
                            "subordinate_outputs": []
                        }),
                        "marker_fingerprint",
                    )?;
                    write_new_durable(&pending.join("commit-marker.json"), &jcs_lf(&marker)?)?;
                    return self.finish_suffix_recovery(pending, &intent, &verified, &marker, &now);
                }
                Err(mut error) => {
                    error.detail = format!("promotion recovery install: {}", error.detail);
                    return Err(error);
                }
            }
        }
        installed_guards = self.verify_installed_outputs(pending, &intent, &verified, &outputs)?;
        require_installed_guards_unchanged(&installed_guards)?;
        let publication_result = if promotion_commit {
            let observation = read_native_publication_observation(pending, &intent, &verified)?;
            if publication_result_preexists {
                let result = read_control(
                    &pending.join("native-publication-result.json"),
                    "result_fingerprint",
                )?;
                if string_field(&result, "schema_id")?
                    != "handbook.generic-native-publication-result"
                    || string_field(&result, "transaction_id")?
                        != string_field(&observation, "transaction_id")?
                    || string_field(&result, "intent_fingerprint")?
                        != string_field(&observation, "intent_fingerprint")?
                    || string_field(&result, "observation_fingerprint")?
                        != string_field(&observation, "observation_fingerprint")?
                    || string_field(&result, "verified_fingerprint")?
                        != string_field(&verified, "verified_fingerprint")?
                    || string_field(&result, "publication_disposition")? != "authorized"
                {
                    return Err(conflicting(
                        "durable native publication result disagrees with recovery journal",
                    ));
                }
                Some(result)
            } else {
                let absent = || {
                    json!({
                        "presence": "absent",
                        "semantic_binding": "absent",
                        "identity_token": null,
                        "version_token": null,
                        "version_transition": "not_applicable",
                        "bytes_sha256": null,
                        "byte_length": null,
                        "link_count": null,
                        "platform_observation": null
                    })
                };
                let observed = |path: &Path,
                                semantic_binding: &str,
                                version_transition: &str|
                 -> Result<Value, GenericLineageStoreErrorV1> {
                    let (guard, bytes, identity_token, version_token) =
                        observe_retained_regular_file(path, MAX_RECORD_BYTES)?;
                    let metadata = guard
                        .metadata()
                        .map_err(|_| io_error("native publication result metadata failed"))?;
                    let (_, _, platform_observation) = native_metadata_subjects(&metadata, path)?;
                    Ok(json!({
                        "presence": "regular",
                        "semantic_binding": semantic_binding,
                        "identity_token": identity_token,
                        "version_token": version_token,
                        "version_transition": version_transition,
                        "bytes_sha256": sha256_prefixed(&bytes),
                        "byte_length": bytes.len(),
                        "link_count": platform_observation["link_count"],
                        "platform_observation": platform_observation
                    }))
                };
                let expected_presence = string_field(&observation, "expected_basis_presence")?;
                let canonical_path = self
                    .repo_root
                    .join(string_field(&observation, "canonical_ref")?);
                let candidate_path = self
                    .repo_root
                    .join(string_field(&observation, "candidate_ref")?);
                let displaced_path = self
                    .repo_root
                    .join(string_field(&observation, "displaced_ref")?);
                let candidate_before = json!({
                    "presence": "regular",
                    "semantic_binding": "replacement",
                    "identity_token": observation["replacement_identity_token"],
                    "version_token": observation["replacement_version_token"],
                    "version_transition": "unchanged_before_first_move",
                    "bytes_sha256": observation["replacement_bytes_sha256"],
                    "byte_length": observation["replacement_byte_length"],
                    "link_count": observation["replacement_platform_observation"]["link_count"],
                    "platform_observation": observation["replacement_platform_observation"]
                });
                let canonical_before = if expected_presence == "present" {
                    json!({
                        "presence": "regular",
                        "semantic_binding": "expected_basis",
                        "identity_token": observation["expected_basis_identity_token"],
                        "version_token": observation["expected_basis_version_token"],
                        "version_transition": "unchanged_before_first_move",
                        "bytes_sha256": observation["expected_basis_fingerprint"],
                        "byte_length": observation["expected_basis_byte_length"],
                        "link_count": observation["expected_basis_platform_observation"]["link_count"],
                        "platform_observation": observation["expected_basis_platform_observation"]
                    })
                } else {
                    absent()
                };
                let before = json!({
                    "canonical": canonical_before,
                    "candidate": candidate_before,
                    "displaced": absent()
                });
                let canonical_after = observed(
                    &canonical_path,
                    "replacement",
                    "primitive_mutated_same_object",
                )?;
                if canonical_after["identity_token"] != observation["replacement_identity_token"] {
                    return Err(conflicting(
                        "native publication result canonical identity is not the replacement",
                    ));
                }
                if candidate_path
                    .try_exists()
                    .map_err(|_| io_error("native result candidate lookup failed"))?
                {
                    return Err(conflicting(
                        "native publication result retained a published candidate pathname",
                    ));
                }
                let displaced_after = if expected_presence == "present" {
                    let value = observed(
                        &displaced_path,
                        "expected_basis",
                        "primitive_mutated_same_object",
                    )?;
                    if value["identity_token"] != observation["expected_basis_identity_token"] {
                        return Err(conflicting(
                        "native publication result displaced identity is not the expected basis",
                    ));
                    }
                    value
                } else {
                    if displaced_path
                        .try_exists()
                        .map_err(|_| io_error("native result displaced lookup failed"))?
                    {
                        return Err(conflicting(
                            "unexpected displaced path exists after absent-basis publication",
                        ));
                    }
                    absent()
                };
                let after = json!({
                    "canonical": canonical_after,
                    "candidate": absent(),
                    "displaced": displaced_after
                });
                let platform_family = string_field(&observation, "platform_family")?;
                let continuity = |child: &str| {
                    json!({
                        "protocol": "retained-live-handle-v1",
                        "platform_family": platform_family,
                        "source_handle_retained_through_destination_observation": true,
                        "source_identity_reverified_from_retained_handle": true,
                        "source_parent_handle_retained": true,
                        "destination_parent_handle_retained": true,
                        "destination_opened_from_retained_parent": true,
                        "destination_child_name": child,
                        "continuity_outcome": "destination_is_same_live_source_object",
                        "same_live_object_verified": true,
                        "numeric_identity_only": false,
                        "ancestor_chain_binding": if platform_family == "windows" {
                            "windows_retained_ancestors_no_share_delete_enumerated_child_ids_no_reparse"
                        } else {
                            "unix_openat2_beneath_retained_dirfds_no_symlinks"
                        },
                        "platform_binding": if platform_family == "windows" {
                            "setfileinformationbyhandle_filerenameinfo_rootdirectory_simple_child_replace_false"
                        } else {
                            "renameat2_retained_source_and_parent_dirfds"
                        }
                    })
                };
                let canonical_child = Path::new(string_field(&observation, "canonical_ref")?)
                    .file_name()
                    .and_then(|name| name.to_str())
                    .ok_or_else(|| invalid_intent("canonical publication child is not UTF-8"))?;
                let native_call_trace = if expected_presence == "present" {
                    let displaced_child = Path::new(string_field(&observation, "displaced_ref")?)
                        .file_name()
                        .and_then(|name| name.to_str())
                        .ok_or_else(|| {
                            invalid_intent("displaced publication child is not UTF-8")
                        })?;
                    let intermediate = json!({
                        "canonical": absent(),
                        "candidate": before["candidate"],
                        "displaced": after["displaced"]
                    });
                    json!([
                        {
                            "ordinal": 0,
                            "action": "claim_canonical_to_displaced",
                            "outcome": "success",
                            "error_code": null,
                            "before": before,
                            "after": intermediate,
                            "continuity_proof": continuity(displaced_child)
                        },
                        {
                            "ordinal": 1,
                            "action": "publish_replacement_to_canonical",
                            "outcome": "success",
                            "error_code": null,
                            "before": intermediate,
                            "after": after,
                            "continuity_proof": continuity(canonical_child)
                        }
                    ])
                } else {
                    json!([{
                        "ordinal": 0,
                        "action": "publish_replacement_to_canonical",
                        "outcome": "success",
                        "error_code": null,
                        "before": before,
                        "after": after,
                        "continuity_proof": continuity(canonical_child)
                    }])
                };
                let result = finalize_control(
                    json!({
                        "schema_id": "handbook.generic-native-publication-result",
                        "schema_version": "1.0",
                        "transaction_id": observation["transaction_id"],
                        "intent_fingerprint": observation["intent_fingerprint"],
                        "observation_fingerprint": observation["observation_fingerprint"],
                        "verified_fingerprint": string_field(&verified, "verified_fingerprint")?,
                        "continuity_protocol": observation["continuity_protocol"],
                        "continuity_completion": "all_native_moves_proved_while_source_handles_live",
                        "platform_family": observation["platform_family"],
                        "platform_primitive": observation["platform_primitive"],
                        "expected_basis_presence": observation["expected_basis_presence"],
                        "expected_basis_fingerprint": observation["expected_basis_fingerprint"],
                        "expected_basis_byte_length": observation["expected_basis_byte_length"],
                        "expected_basis_identity_token": observation["expected_basis_identity_token"],
                        "expected_basis_version_token": observation["expected_basis_version_token"],
                        "expected_basis_platform_observation": observation["expected_basis_platform_observation"],
                        "expected_basis_backup_ref": observation["expected_basis_backup_ref"],
                        "expected_basis_backup_bytes_sha256": observation["expected_basis_backup_bytes_sha256"],
                        "expected_basis_backup_byte_length": observation["expected_basis_backup_byte_length"],
                        "expected_basis_backup_metadata_fingerprint": observation["expected_basis_backup_metadata_fingerprint"],
                        "replacement_bytes_sha256": observation["replacement_bytes_sha256"],
                        "replacement_byte_length": observation["replacement_byte_length"],
                        "replacement_identity_token": observation["replacement_identity_token"],
                        "replacement_version_token": observation["replacement_version_token"],
                        "replacement_platform_observation": observation["replacement_platform_observation"],
                        "native_call_trace": native_call_trace,
                        "canonical_observation": after["canonical"],
                        "candidate_observation": after["candidate"],
                        "displaced_observation": after["displaced"],
                        "basis_disposition": if expected_presence == "present" {
                            "expected_basis_displaced"
                        } else {
                            "expected_absent_confirmed"
                        },
                        "publication_disposition": "authorized",
                        "reader_authority": "withheld_until_commit_marker",
                        "cleanup_authority": "forbidden"
                    }),
                    "result_fingerprint",
                )?;
                write_new_durable(
                    &pending.join("native-publication-result.json"),
                    &jcs_lf(&result)?,
                )?;
                Some(result)
            }
        } else {
            None
        };
        let mut marker = build_marker(&intent, &verified)?;
        if let Some(result) = publication_result.as_ref() {
            let object = marker.as_object_mut().expect("marker object");
            object.remove("marker_fingerprint");
            object.insert(
                "publication_result_fingerprint".to_owned(),
                json!(string_field(result, "result_fingerprint")?),
            );
            marker = finalize_control(marker, "marker_fingerprint")?;
        }
        write_new_durable(&marker_path, &jcs_lf(&marker)?)?;
        if let Err(error) = require_installed_guards_unchanged(&installed_guards) {
            remove_store_file(&marker_path)?;
            return Err(error);
        }
        self.finish_suffix_recovery(pending, &intent, &verified, &marker, &now)
    }

    fn finish_suffix_recovery(
        &self,
        pending: &Path,
        intent: &Value,
        verified: &Value,
        marker: &Value,
        now_utc: &str,
    ) -> Result<(), GenericLineageStoreErrorV1> {
        validate_suffix_prefix(self, pending, intent)?;
        let evidence = build_evidence(intent, verified, marker)?;
        write_new_or_equal(&pending.join("evidence.json"), &jcs_lf(&evidence)?)?;
        let result = build_result(intent, marker, &evidence)?;
        let transaction_id = string_field(intent, "transaction_id")?;
        let result_path = self.result_path(transaction_id);
        create_safe_directories(&self.repo_root, result_path.parent().unwrap())?;
        write_new_or_equal(&result_path, &jcs_lf(&result)?)?;
        let ledger_path =
            self.ledger_path(string_field(intent, "domain_mutation_key_fingerprint")?);
        create_safe_directories(&self.repo_root, ledger_path.parent().unwrap())?;
        if ledger_path
            .try_exists()
            .map_err(|_| io_error("recovery ledger lookup failed"))?
        {
            let ledger = read_control(&ledger_path, "entry_fingerprint")?;
            if string_field(&ledger, "state")? != "retained_result"
                || string_field(&ledger, "request_fingerprint")?
                    != string_field(intent, "request_fingerprint")?
                || string_field(&ledger, "result_fingerprint")?
                    != string_field(&result, "result_fingerprint")?
            {
                return Err(retained_error(
                    "installed recovery ledger disagrees with the exact result",
                ));
            }
        } else {
            let retained_until = format_utc(parse_utc(now_utc)? + RETAINED_RESULT_SECONDS)?;
            let ledger = build_ledger(intent, &result, &retained_until)?;
            write_new_durable(&ledger_path, &jcs_lf(&ledger)?)?;
        }
        let operation = operation_from_intent(intent)?;
        let family = self.transaction_family_root(operation);
        let committed = family.join(format!("{transaction_id}.committed"));
        rename_store_path(pending, &committed)?;
        sync_directory(&family)
    }

    fn verify_committed(&self, committed: &Path) -> Result<(), GenericLineageStoreErrorV1> {
        let intent = read_control(&committed.join("intent.json"), "intent_fingerprint")?;
        validate_intent(&intent)?;
        validate_transaction_directory(committed, &intent, "committed")?;
        let verified = read_control(&committed.join("verified.json"), "verified_fingerprint")?;
        validate_verified(&verified, &intent)?;
        let marker = read_control(&committed.join("commit-marker.json"), "marker_fingerprint")?;
        validate_marker(&marker, &intent, &verified)?;
        let terminal_publication_conflict = marker.get("outcome").and_then(Value::as_str)
            == Some("refused")
            && marker
                .get("refusal")
                .and_then(|value| value.get("code"))
                .and_then(Value::as_str)
                == Some("publication_basis_conflict");
        if string_field(&intent, "planned_outcome")? == "commit" && !terminal_publication_conflict {
            let guards = self.verify_installed_outputs(
                committed,
                &intent,
                &verified,
                &intent_outputs(&intent)?,
            )?;
            require_installed_guards_unchanged(&guards)?;
        }
        if operation_from_intent(&intent)? == GenericArtifactOperationV1::ArtifactCandidatePromote
            && string_field(&intent, "planned_outcome")? == "commit"
        {
            let native_result = read_control(
                &committed.join("native-publication-result.json"),
                "result_fingerprint",
            )?;
            if string_field(&native_result, "schema_id")?
                != "handbook.generic-native-publication-result"
                || string_field(&native_result, "transaction_id")?
                    != string_field(&intent, "transaction_id")?
                || string_field(&native_result, "intent_fingerprint")?
                    != string_field(&intent, "intent_fingerprint")?
                || string_field(&native_result, "verified_fingerprint")?
                    != string_field(&verified, "verified_fingerprint")?
                || string_field(&native_result, "result_fingerprint")?
                    != string_field(&marker, "publication_result_fingerprint")?
                || (terminal_publication_conflict
                    && string_field(&native_result, "publication_disposition")?
                        != "refused_basis_conflict")
                || (!terminal_publication_conflict
                    && string_field(&native_result, "publication_disposition")? != "authorized")
            {
                return Err(retained_error(
                    "committed native publication result disagrees with its exact journal chain",
                ));
            }
        }
        let evidence = read_control(&committed.join("evidence.json"), "evidence_fingerprint")?;
        let expected_evidence = build_evidence(&intent, &verified, &marker)?;
        if evidence != expected_evidence {
            return Err(retained_error("committed evidence disagrees with journal"));
        }
        let transaction_id = string_field(&intent, "transaction_id")?;
        let result = read_control(&self.result_path(transaction_id), "result_fingerprint")?;
        let expected_result = build_result(&intent, &marker, &evidence)?;
        if result != expected_result {
            return Err(retained_error("committed result disagrees with journal"));
        }
        let ledger = read_control(
            &self.ledger_path(string_field(&intent, "domain_mutation_key_fingerprint")?),
            "entry_fingerprint",
        )?;
        validate_ledger_record(&ledger)?;
        for field in [
            "repository_identity_fingerprint",
            "owner_contract_subject_fingerprint",
            "operation_id",
            "domain_mutation_key_fingerprint",
            "request_fingerprint",
            "transaction_id",
        ] {
            if string_field(&ledger, field)? != string_field(&intent, field)? {
                return Err(retained_error("committed ledger disagrees with intent"));
            }
        }
        if string_field(&ledger, "state")? == "retained_result" {
            let transaction_id = string_field(&intent, "transaction_id")?;
            let expected_result_ref = format!(
                ".handbook/state/idempotency/generic-artifact-operations/results/{transaction_id}.json"
            );
            if string_field(&ledger, "result_ref")? != expected_result_ref
                || string_field(&ledger, "result_fingerprint")?
                    != string_field(&result, "result_fingerprint")?
            {
                return Err(retained_error("committed ledger disagrees with result"));
            }
        }
        Ok(())
    }

    fn verify_staging(
        &self,
        pending: &Path,
        intent: &Value,
        outputs: &[Value],
        verified: &Value,
    ) -> Result<(), GenericLineageStoreErrorV1> {
        if string_field(verified, "verification_basis")? != "staging" {
            return Ok(());
        }
        for (ordinal, descriptor) in outputs.iter().enumerate() {
            let path = pending
                .join("staged")
                .join(staged_name(ordinal, string_field(descriptor, "token")?));
            let bytes = read_bounded_regular(&path, MAX_RECORD_BYTES)?;
            validate_persisted_output_bytes(
                intent,
                ordinal,
                outputs.len() - 1,
                descriptor,
                &bytes,
            )?;
        }
        Ok(())
    }

    fn recovery_installed_output_set(
        &self,
        transaction: &Path,
        intent: &Value,
        verified: &Value,
        outputs: &[Value],
    ) -> Result<BTreeSet<usize>, GenericLineageStoreErrorV1> {
        let expected_basis = intent_expected_basis(intent)?;
        let expected_basis_version = intent_expected_basis_version_token(intent)?;
        let publication = if operation_from_intent(intent)?
            == GenericArtifactOperationV1::ArtifactCandidatePromote
        {
            Some(read_native_publication_observation(
                transaction,
                intent,
                verified,
            )?)
        } else {
            None
        };
        let mut installed = BTreeSet::new();
        for (ordinal, descriptor) in outputs.iter().enumerate() {
            let mode = string_field(descriptor, "install_mode")?;
            let path = self.repo_root.join(string_field(descriptor, "final_ref")?);
            let observed = match path.try_exists() {
                Ok(false) => None,
                Ok(true) => {
                    let (_guard, bytes, identity_token, version_token) =
                        observe_retained_regular_file(&path, MAX_RECORD_BYTES)?;
                    Some((bytes, identity_token, version_token))
                }
                Err(_) => return Err(io_error("recovery output lookup failed")),
            };
            match observed {
                Some((bytes, identity_token, _version_token))
                    if string_field(descriptor, "bytes_sha256")? == sha256_prefixed(&bytes)
                        && descriptor.get("byte_length").and_then(Value::as_u64)
                            == Some(bytes.len() as u64) =>
                {
                    if mode == "replace_if_current"
                        && publication.as_ref().is_none_or(|observation| {
                            string_field(observation, "replacement_identity_token").ok()
                                != Some(identity_token.as_str())
                        })
                    {
                        return Err(error(
                            GenericLineageStoreErrorKindV1::BasisMismatch,
                            "recovery canonical bytes have an unbound publication identity",
                        ));
                    }
                    validate_persisted_output_bytes(
                        intent,
                        ordinal,
                        outputs.len() - 1,
                        descriptor,
                        &bytes,
                    )?;
                    installed.insert(ordinal);
                }
                Some((bytes, _identity_token, version_token))
                    if mode == "replace_if_current"
                        && Some(sha256_prefixed(&bytes).as_str()) == expected_basis
                        && Some(version_token.as_str()) == expected_basis_version => {}
                None if mode == "replace_if_current" && expected_basis.is_none() => {}
                Some(_) if mode == "replace_if_current" => {
                    return Err(error(
                        GenericLineageStoreErrorKindV1::BasisMismatch,
                        "recovery canonical observation matches neither basis nor installed output",
                    ));
                }
                None if mode == "replace_if_current" => {
                    return Err(error(
                        GenericLineageStoreErrorKindV1::BasisMismatch,
                        "recovery canonical basis disappeared before installation",
                    ));
                }
                Some(_) => {
                    return Err(conflicting(
                        "preexisting immutable recovery destination has unequal bytes",
                    ));
                }
                None if mode == "create_new" => {}
                None => return Err(invalid_intent("unknown output install mode")),
            }
        }
        Ok(installed)
    }

    fn verify_installed_outputs(
        &self,
        transaction: &Path,
        intent: &Value,
        verified: &Value,
        outputs: &[Value],
    ) -> Result<Vec<InstalledOutputGuard>, GenericLineageStoreErrorV1> {
        let publication = if operation_from_intent(intent)?
            == GenericArtifactOperationV1::ArtifactCandidatePromote
        {
            Some(read_native_publication_observation(
                transaction,
                intent,
                verified,
            )?)
        } else {
            None
        };
        let mut guards = Vec::with_capacity(outputs.len());
        let mut bytes = Vec::with_capacity(outputs.len());
        for descriptor in outputs {
            let path = self.repo_root.join(string_field(descriptor, "final_ref")?);
            let expected = read_bounded_regular(&path, MAX_RECORD_BYTES)?;
            require_descriptor_bytes(descriptor, &expected)?;
            let identity = if string_field(descriptor, "install_mode")? == "replace_if_current" {
                Some(string_field(
                    publication.as_ref().ok_or_else(|| {
                        conflicting("canonical publication observation is absent")
                    })?,
                    "replacement_identity_token",
                )?)
            } else {
                None
            };
            guards.push(observe_installed_output(&path, &expected, identity)?);
            bytes.push(expected);
        }
        validate_persisted_cross_bindings(intent, outputs, &bytes)?;
        Ok(guards)
    }

    fn install_descriptor(
        &self,
        transaction: &Path,
        intent: &Value,
        verified: &Value,
        descriptor: &Value,
        bytes: &[u8],
    ) -> Result<InstalledOutputGuard, GenericLineageStoreErrorV1> {
        let mode = string_field(descriptor, "install_mode")?;
        let path = self.repo_root.join(string_field(descriptor, "final_ref")?);
        let parent = path
            .parent()
            .ok_or_else(|| unsafe_ref("output has no parent"))?;
        create_safe_directories(&self.repo_root, parent)?;
        if mode == "create_new" {
            write_new_durable(&path, bytes)?;
            observe_installed_output(&path, bytes, None)
        } else if mode == "replace_if_current" {
            let publication = read_native_publication_observation(transaction, intent, verified)?;
            let candidate_ref = string_field(&publication, "candidate_ref")?;
            validate_safe_ref(candidate_ref)?;
            let candidate = self.repo_root.join(candidate_ref);
            if candidate.parent() != path.parent() {
                return Err(conflicting(
                    "recovery candidate is not in the canonical output parent",
                ));
            }
            let (file, directory_guards) = open_replacement_guard(&candidate)?;
            let scratch = observe_prepared_replacement(candidate, file, directory_guards, bytes)?;
            if scratch.identity_token != string_field(&publication, "replacement_identity_token")?
                || scratch.version_token != string_field(&publication, "replacement_version_token")?
            {
                return Err(conflicting(
                    "recovery candidate identity or version disagrees with verified publication",
                ));
            }
            let basis = verify_compare_and_write_path(
                &path,
                intent_expected_basis(intent)?,
                intent
                    .get("expected_basis_version_token")
                    .and_then(Value::as_str),
            )?;
            drop(basis);
            publish_replacement(
                &path,
                scratch,
                bytes,
                intent_expected_basis(intent)?,
                intent
                    .get("expected_basis_version_token")
                    .and_then(Value::as_str),
            )
        } else {
            Err(error(
                GenericLineageStoreErrorKindV1::InvalidTransactionIntent,
                "unknown output install mode",
            ))
        }
    }

    fn authoritative_ref_is_committed(
        &self,
        relative_ref: &str,
        fingerprint: &str,
    ) -> Result<bool, GenericLineageStoreErrorV1> {
        for operation in [
            GenericArtifactOperationV1::IntakeRecordAppend,
            GenericArtifactOperationV1::ArtifactCandidateAppend,
            GenericArtifactOperationV1::ArtifactCandidatePromote,
        ] {
            let family = self.transaction_family_root(operation);
            if !family
                .try_exists()
                .map_err(|_| io_error("journal family lookup failed"))?
            {
                continue;
            }
            for path in read_dir_paths(&family)? {
                if path.extension().is_none_or(|ext| ext != "committed") {
                    continue;
                }
                let marker = read_control(&path.join("commit-marker.json"), "marker_fingerprint")?;
                let Some(outputs) = marker
                    .get("authoritative_outputs")
                    .and_then(Value::as_array)
                else {
                    return Err(retained_error("committed marker outputs are malformed"));
                };
                if outputs.iter().any(|output| {
                    output.get("ref").and_then(Value::as_str) == Some(relative_ref)
                        && output.get("fingerprint").and_then(Value::as_str) == Some(fingerprint)
                }) {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    fn require_exact_committed_chain(
        &self,
        ledger: &Value,
    ) -> Result<(), GenericLineageStoreErrorV1> {
        validate_ledger_record(ledger)?;
        let operation =
            GenericArtifactOperationV1::from_operation_id(string_field(ledger, "operation_id")?)
                .ok_or_else(|| retained_error("ledger operation is outside the closed set"))?;
        let transaction_id = string_field(ledger, "transaction_id")?;
        let committed = self
            .transaction_family_root(operation)
            .join(format!("{transaction_id}.committed"));
        if !committed
            .try_exists()
            .map_err(|_| io_error("committed chain lookup failed"))?
        {
            return Err(retained_error(
                "domain ledger is not backed by one exact committed chain",
            ));
        }
        self.verify_committed(&committed)
    }

    fn validate_recoverable_reachability(&self) -> Result<(), GenericLineageStoreErrorV1> {
        let mut expected_ledgers = BTreeSet::new();
        let mut expected_results = BTreeSet::new();
        let mut allowed_artifacts = BTreeSet::new();
        for operation in [
            GenericArtifactOperationV1::IntakeRecordAppend,
            GenericArtifactOperationV1::ArtifactCandidateAppend,
            GenericArtifactOperationV1::ArtifactCandidatePromote,
        ] {
            let family = self.transaction_family_root(operation);
            if !family
                .try_exists()
                .map_err(|_| io_error("recoverable reachability family lookup failed"))?
            {
                continue;
            }
            let mut transactions = read_dir_paths(&family)?;
            transactions.sort();
            for transaction in transactions {
                let name = file_name(&transaction)?;
                let intent_path = transaction.join("intent.json");
                if !intent_path
                    .try_exists()
                    .map_err(|_| io_error("recoverable intent reachability lookup failed"))?
                {
                    if name.ends_with(".pending") && read_dir_paths(&transaction)?.is_empty() {
                        continue;
                    }
                    return Err(retained_error("nonempty journal chain has no exact intent"));
                }
                let intent = read_control(&intent_path, "intent_fingerprint")?;
                validate_intent(&intent)?;
                let transaction_id = string_field(&intent, "transaction_id")?;
                if string_field(&intent, "planned_outcome")? == "commit" {
                    for descriptor in intent_outputs(&intent)? {
                        if string_field(&descriptor, "install_mode")? != "create_new" {
                            continue;
                        }
                        if !allowed_artifacts
                            .insert(string_field(&descriptor, "final_ref")?.to_owned())
                        {
                            return Err(retained_error(
                                "multiple journal chains cite one immutable artifact output",
                            ));
                        }
                    }
                }
                let result_ref = format!(
                    ".handbook/state/idempotency/generic-artifact-operations/results/{transaction_id}.json"
                );
                let ledger_ref = format!(
                    ".handbook/state/idempotency/generic-artifact-operations/ledger/{}.json",
                    fingerprint_hex(string_field(&intent, "domain_mutation_key_fingerprint")?)?
                );
                if name.ends_with(".committed") {
                    self.verify_committed(&transaction)?;
                    if !expected_results.insert(result_ref) || !expected_ledgers.insert(ledger_ref)
                    {
                        return Err(retained_error(
                            "multiple committed chains cite one idempotency record",
                        ));
                    }
                    continue;
                }
                validate_suffix_prefix(self, &transaction, &intent)?;
                let evidence_path = transaction.join("evidence.json");
                let result_path = self.result_path(transaction_id);
                let ledger_path =
                    self.ledger_path(string_field(&intent, "domain_mutation_key_fingerprint")?);
                let evidence_exists = evidence_path
                    .try_exists()
                    .map_err(|_| io_error("pending evidence reachability lookup failed"))?;
                let result_exists = result_path
                    .try_exists()
                    .map_err(|_| io_error("pending result reachability lookup failed"))?;
                let ledger_exists = ledger_path
                    .try_exists()
                    .map_err(|_| io_error("pending ledger reachability lookup failed"))?;
                let mut result = None;
                if evidence_exists {
                    let verified =
                        read_control(&transaction.join("verified.json"), "verified_fingerprint")?;
                    validate_verified(&verified, &intent)?;
                    let marker = read_control(
                        &transaction.join("commit-marker.json"),
                        "marker_fingerprint",
                    )?;
                    validate_marker(&marker, &intent, &verified)?;
                    let evidence = read_control(&evidence_path, "evidence_fingerprint")?;
                    if evidence != build_evidence(&intent, &verified, &marker)? {
                        return Err(retained_error(
                            "pending evidence disagrees with its exact journal chain",
                        ));
                    }
                    if result_exists {
                        let observed = read_control(&result_path, "result_fingerprint")?;
                        if observed != build_result(&intent, &marker, &evidence)? {
                            return Err(retained_error(
                                "pending result disagrees with its exact journal chain",
                            ));
                        }
                        result = Some(observed);
                    }
                }
                if result_exists && !expected_results.insert(result_ref) {
                    return Err(retained_error(
                        "multiple journal chains cite one pending domain result",
                    ));
                }
                if ledger_exists {
                    let result = result.as_ref().ok_or_else(|| {
                        retained_error("pending ledger has no exact result predecessor")
                    })?;
                    let ledger = read_control(&ledger_path, "entry_fingerprint")?;
                    validate_ledger_record(&ledger)?;
                    if string_field(&ledger, "state")? != "retained_result" {
                        return Err(retained_error(
                            "pending ledger is not an exact retained-result suffix",
                        ));
                    }
                    for field in [
                        "repository_identity_fingerprint",
                        "owner_contract_subject_fingerprint",
                        "operation_id",
                        "domain_mutation_key_fingerprint",
                        "request_fingerprint",
                        "transaction_id",
                    ] {
                        if string_field(&ledger, field)? != string_field(&intent, field)? {
                            return Err(retained_error(
                                "pending ledger disagrees with its exact intent",
                            ));
                        }
                    }
                    if string_field(&ledger, "result_ref")? != store_relative_path(&result_path)?.1
                        || string_field(&ledger, "result_fingerprint")?
                            != string_field(result, "result_fingerprint")?
                    {
                        return Err(retained_error(
                            "pending ledger disagrees with its exact result",
                        ));
                    }
                    if !expected_ledgers.insert(ledger_ref) {
                        return Err(retained_error(
                            "multiple journal chains cite one pending domain ledger",
                        ));
                    }
                }
            }
        }

        let actual_ledgers = self.validate_reachable_ledger_records()?;
        let actual_results = self.validate_reachable_result_records()?;
        let actual_artifacts = self.collect_artifact_store_records()?;
        if actual_ledgers != expected_ledgers || actual_results != expected_results {
            return Err(retained_error(
                "idempotency inventory is not exactly reachable from a journal chain",
            ));
        }
        if !actual_artifacts.is_subset(&allowed_artifacts) {
            return Err(retained_error(
                "artifact inventory contains a record uncited by any journal chain",
            ));
        }
        Ok(())
    }

    fn validate_committed_reachability(&self) -> Result<(), GenericLineageStoreErrorV1> {
        let mut expected_ledgers = BTreeSet::new();
        let mut expected_results = BTreeSet::new();
        let mut expected_artifacts = BTreeSet::new();
        for operation in [
            GenericArtifactOperationV1::IntakeRecordAppend,
            GenericArtifactOperationV1::ArtifactCandidateAppend,
            GenericArtifactOperationV1::ArtifactCandidatePromote,
        ] {
            let family = self.transaction_family_root(operation);
            if !family
                .try_exists()
                .map_err(|_| io_error("committed reachability family lookup failed"))?
            {
                continue;
            }
            let mut transactions = read_dir_paths(&family)?;
            transactions.sort();
            for committed in transactions
                .into_iter()
                .filter(|path| path.extension().is_some_and(|ext| ext == "committed"))
            {
                let intent = read_control(&committed.join("intent.json"), "intent_fingerprint")?;
                validate_intent(&intent)?;
                validate_transaction_directory(&committed, &intent, "committed")?;
                let transaction_id = string_field(&intent, "transaction_id")?;
                let result_ref = format!(
                    ".handbook/state/idempotency/generic-artifact-operations/results/{transaction_id}.json"
                );
                if !expected_results.insert(result_ref) {
                    return Err(retained_error(
                        "multiple committed chains cite one domain result",
                    ));
                }
                let ledger_ref = format!(
                    ".handbook/state/idempotency/generic-artifact-operations/ledger/{}.json",
                    fingerprint_hex(string_field(&intent, "domain_mutation_key_fingerprint",)?)?
                );
                if !expected_ledgers.insert(ledger_ref) {
                    return Err(retained_error(
                        "multiple committed chains cite one domain ledger",
                    ));
                }
                let outputless_publication_conflict = if string_field(&intent, "planned_outcome")?
                    == "commit"
                    && operation == GenericArtifactOperationV1::ArtifactCandidatePromote
                {
                    let verified =
                        read_control(&committed.join("verified.json"), "verified_fingerprint")?;
                    validate_verified(&verified, &intent)?;
                    let marker =
                        read_control(&committed.join("commit-marker.json"), "marker_fingerprint")?;
                    validate_marker(&marker, &intent, &verified)?;
                    let is_conflict = marker.get("outcome").and_then(Value::as_str)
                        == Some("refused")
                        && marker
                            .get("refusal")
                            .and_then(|value| value.get("code"))
                            .and_then(Value::as_str)
                            == Some("publication_basis_conflict");
                    if is_conflict {
                        let native_result = read_control(
                            &committed.join("native-publication-result.json"),
                            "result_fingerprint",
                        )?;
                        if string_field(&native_result, "publication_disposition")?
                            != "refused_basis_conflict"
                            || string_field(&native_result, "result_fingerprint")?
                                != string_field(&marker, "publication_result_fingerprint")?
                        {
                            return Err(retained_error(
                                "outputless publication conflict lacks its exact native result",
                            ));
                        }
                    }
                    is_conflict
                } else {
                    false
                };
                if string_field(&intent, "planned_outcome")? == "commit"
                    && !outputless_publication_conflict
                {
                    for descriptor in intent_outputs(&intent)? {
                        if string_field(&descriptor, "install_mode")? != "create_new" {
                            continue;
                        }
                        let output_ref = string_field(&descriptor, "final_ref")?.to_owned();
                        if !expected_artifacts.insert(output_ref) {
                            return Err(retained_error(
                                "multiple committed chains cite one immutable artifact output",
                            ));
                        }
                    }
                }
            }
        }

        let actual_ledgers = self.validate_reachable_ledger_records()?;
        let actual_results = self.validate_reachable_result_records()?;
        let actual_artifacts = self.collect_artifact_store_records()?;
        if actual_ledgers != expected_ledgers {
            return Err(retained_error(
                "domain ledger inventory is not exactly reachable from committed chains",
            ));
        }
        if actual_results != expected_results {
            return Err(retained_error(
                "domain result inventory is not exactly reachable from committed chains",
            ));
        }
        if actual_artifacts != expected_artifacts {
            return Err(retained_error(
                "artifact record inventory is not exactly reachable from committed chains",
            ));
        }
        Ok(())
    }

    fn validate_reachable_ledger_records(
        &self,
    ) -> Result<BTreeSet<String>, GenericLineageStoreErrorV1> {
        let root = self
            .repo_root
            .join(".handbook/state/idempotency/generic-artifact-operations/ledger");
        if !root
            .try_exists()
            .map_err(|_| io_error("ledger reachability lookup failed"))?
        {
            return Ok(BTreeSet::new());
        }
        let mut refs = BTreeSet::new();
        for path in read_dir_paths(&root)? {
            let ledger = read_control(&path, "entry_fingerprint")?;
            validate_ledger_record(&ledger)?;
            let expected_name = format!(
                "{}.json",
                fingerprint_hex(string_field(&ledger, "domain_mutation_key_fingerprint")?)?
            );
            if file_name(&path)? != expected_name {
                return Err(retained_error(
                    "domain ledger filename does not bind its exact key",
                ));
            }
            refs.insert(store_relative_path(&path)?.1);
        }
        Ok(refs)
    }

    fn validate_reachable_result_records(
        &self,
    ) -> Result<BTreeSet<String>, GenericLineageStoreErrorV1> {
        let root = self
            .repo_root
            .join(".handbook/state/idempotency/generic-artifact-operations/results");
        if !root
            .try_exists()
            .map_err(|_| io_error("result reachability lookup failed"))?
        {
            return Ok(BTreeSet::new());
        }
        let mut refs = BTreeSet::new();
        for path in read_dir_paths(&root)? {
            let value = read_control(&path, "result_fingerprint")?;
            let result = validate_result_record(value)?;
            if file_name(&path)? != format!("{}.json", result.transaction_id) {
                return Err(retained_error(
                    "domain result filename does not bind its exact transaction",
                ));
            }
            refs.insert(store_relative_path(&path)?.1);
        }
        Ok(refs)
    }

    fn collect_artifact_store_records(
        &self,
    ) -> Result<BTreeSet<String>, GenericLineageStoreErrorV1> {
        let mut refs = BTreeSet::new();
        for root in [
            self.repo_root.join(".handbook/state/artifacts"),
            self.repo_root.join(".handbook/evidence/artifacts"),
        ] {
            if !root
                .try_exists()
                .map_err(|_| io_error("artifact reachability lookup failed"))?
            {
                continue;
            }
            for instance in read_dir_paths(&root)? {
                for family in read_dir_paths(&instance)? {
                    for path in read_dir_paths(&family)? {
                        if !refs.insert(store_relative_path(&path)?.1) {
                            return Err(retained_error(
                                "artifact record has an ambiguous filesystem identity",
                            ));
                        }
                    }
                }
            }
        }
        Ok(refs)
    }

    fn read_result_for_transaction(
        &self,
        operation: GenericArtifactOperationV1,
        transaction_id: &str,
    ) -> Result<GenericDomainMutationResultV1, GenericLineageStoreErrorV1> {
        if !transaction_id.starts_with(operation.transaction_token()) {
            return Err(retained_error("transaction operation family mismatch"));
        }
        let value = read_control(&self.result_path(transaction_id), "result_fingerprint")?;
        serde_json::from_value(value)
            .map_err(|_| retained_error("domain result is not exact-closed"))
    }

    fn validate_inventory(&self) -> Result<(), GenericLineageStoreErrorV1> {
        let transactions = self.repo_root.join(".handbook/state/transactions");
        let mut all_pending_bytes = 0u64;
        let mut all_committed_bytes = 0u64;
        if transactions
            .try_exists()
            .map_err(|_| io_error("transaction inventory failed"))?
        {
            reject_link(&transactions, true)?;
            let mut families = read_dir_paths(&transactions)?;
            families.sort();
            for family in families {
                reject_link(&family, true)?;
                let name = file_name(&family)?;
                if name == "registry" {
                    for transaction in read_dir_paths(&family)? {
                        let transaction_name = file_name(&transaction)?;
                        if !(transaction_name.ends_with(".pending")
                            || transaction_name.ends_with(".committed"))
                        {
                            return Err(conflicting(
                                "unknown entry in registry transaction family",
                            ));
                        }
                        reject_link(&transaction, true)?;
                    }
                    continue;
                }
                if ![
                    "intake-records",
                    "artifact-candidates",
                    "artifact-promotions",
                ]
                .contains(&name.as_str())
                {
                    return Err(conflicting("unknown generic transaction family"));
                }
                let operation = match name.as_str() {
                    "intake-records" => GenericArtifactOperationV1::IntakeRecordAppend,
                    "artifact-candidates" => GenericArtifactOperationV1::ArtifactCandidateAppend,
                    "artifact-promotions" => GenericArtifactOperationV1::ArtifactCandidatePromote,
                    _ => unreachable!("family checked above"),
                };
                let mut entries = read_dir_paths(&family)?;
                entries.sort();
                let pending = entries
                    .iter()
                    .filter(|path| path.extension().is_some_and(|ext| ext == "pending"))
                    .count();
                let committed = entries
                    .iter()
                    .filter(|path| path.extension().is_some_and(|ext| ext == "committed"))
                    .count();
                if pending > MAX_PENDING_PER_FAMILY || committed > MAX_COMMITTED_PER_FAMILY {
                    return Err(bound_error("generic journal count exceeds its exact limit"));
                }
                let mut identities = BTreeSet::new();
                for transaction in entries {
                    let name = file_name(&transaction)?;
                    if !(name.ends_with(".pending") || name.ends_with(".committed")) {
                        return Err(conflicting("unknown entry in generic transaction family"));
                    }
                    let identity = name
                        .strip_suffix(".pending")
                        .or_else(|| name.strip_suffix(".committed"))
                        .expect("suffix checked");
                    if !valid_transaction_id(operation, identity) {
                        return Err(conflicting(
                            "generic transaction directory identity is invalid",
                        ));
                    }
                    if !identities.insert(identity.to_owned()) {
                        return Err(conflicting(
                            "duplicate pending and committed transaction identity",
                        ));
                    }
                    reject_link(&transaction, true)?;
                    let transaction_bytes = validate_transaction_inventory(&transaction)?;
                    if name.ends_with(".pending") {
                        all_pending_bytes = all_pending_bytes.saturating_add(transaction_bytes);
                    } else {
                        all_committed_bytes = all_committed_bytes.saturating_add(transaction_bytes);
                    }
                }
            }
        }
        if all_pending_bytes > MAX_ALL_PENDING_BYTES
            || all_committed_bytes > MAX_ALL_COMMITTED_BYTES
        {
            return Err(bound_error(
                "aggregate pending or committed journal bytes exceed the closed limit",
            ));
        }
        validate_establishing_inventory(&self.establishing_root())?;
        validate_idempotency_inventory(
            &self
                .repo_root
                .join(".handbook/state/idempotency/generic-artifact-operations"),
        )?;
        validate_instance_store_inventory(&self.repo_root.join(".handbook/state/artifacts"), true)?;
        validate_instance_store_inventory(
            &self.repo_root.join(".handbook/evidence/artifacts"),
            false,
        )
    }

    fn transaction_family_root(&self, operation: GenericArtifactOperationV1) -> PathBuf {
        self.repo_root
            .join(".handbook/state/transactions")
            .join(operation.transaction_family())
    }

    fn establishing_root(&self) -> PathBuf {
        self.repo_root
            .join(".handbook/state/idempotency/generic-artifact-operations/establishing")
    }

    fn result_path(&self, transaction_id: &str) -> PathBuf {
        self.repo_root
            .join(".handbook/state/idempotency/generic-artifact-operations/results")
            .join(format!("{transaction_id}.json"))
    }

    fn ledger_path(&self, key_fingerprint: &str) -> PathBuf {
        let hex = key_fingerprint.strip_prefix("sha256:").unwrap_or("invalid");
        self.repo_root
            .join(".handbook/state/idempotency/generic-artifact-operations/ledger")
            .join(format!("{hex}.json"))
    }
}

fn ensure_refusal_verified(
    pending: &Path,
    intent: &Value,
) -> Result<Value, GenericLineageStoreErrorV1> {
    let path = pending.join("verified.json");
    let expected = build_refusal_verified(intent)?;
    if path
        .try_exists()
        .map_err(|_| io_error("verified lookup failed"))?
    {
        let observed = read_control(&path, "verified_fingerprint")?;
        if observed != expected {
            return Err(refusal_conflict(
                "refusal verified decision disagrees with intent",
            ));
        }
        Ok(observed)
    } else {
        write_new_durable(&path, &jcs_lf(&expected)?)?;
        Ok(expected)
    }
}

fn ensure_marker(
    pending: &Path,
    intent: &Value,
    verified: &Value,
) -> Result<Value, GenericLineageStoreErrorV1> {
    let path = pending.join("commit-marker.json");
    let expected = build_marker(intent, verified)?;
    if path
        .try_exists()
        .map_err(|_| io_error("marker lookup failed"))?
    {
        let observed = read_control(&path, "marker_fingerprint")?;
        if observed != expected {
            return Err(refusal_conflict(
                "refusal marker disagrees with verified decision",
            ));
        }
        Ok(observed)
    } else {
        write_new_durable(&path, &jcs_lf(&expected)?)?;
        Ok(expected)
    }
}

fn validate_intent(intent: &Value) -> Result<(), GenericLineageStoreErrorV1> {
    let promotion_commit = intent.get("operation_id").and_then(Value::as_str)
        == Some("artifact.candidate.promote")
        && intent.get("planned_outcome").and_then(Value::as_str) == Some("commit");
    let mut fields = vec![
        "schema_id",
        "schema_version",
        "transaction_id",
        "repository_identity_fingerprint",
        "owner_contract_ref",
        "owner_contract_subject_fingerprint",
        "operation_id",
        "domain_mutation_key_fingerprint",
        "request_fingerprint",
        "request_subject",
        "operation_context_fingerprint",
        "expected_basis_fingerprint",
        "sampled_finalized_at_utc",
        "planned_outcome",
        "refusal",
        "outputs",
        "intent_fingerprint",
    ];
    if promotion_commit {
        fields.push("expected_basis_presence");
        fields.push("expected_basis_byte_length");
        fields.push("expected_basis_identity_token");
        fields.push("expected_basis_version_token");
    }
    require_exact_fields(
        intent,
        &fields,
        GenericLineageStoreErrorKindV1::InvalidTransactionIntent,
    )?;
    if string_field(intent, "schema_id")? != "handbook.generic-transaction-intent"
        || string_field(intent, "schema_version")? != "1.0"
        || string_field(intent, "owner_contract_ref")? != GENERIC_ARTIFACT_OWNER_CONTRACT_REF
    {
        return Err(invalid_intent(
            "transaction intent schema or owner mismatch",
        ));
    }
    for field in [
        "repository_identity_fingerprint",
        "owner_contract_subject_fingerprint",
        "domain_mutation_key_fingerprint",
        "request_fingerprint",
        "operation_context_fingerprint",
        "intent_fingerprint",
    ] {
        validate_sha256(string_field(intent, field)?)
            .map_err(|_| invalid_intent("transaction intent contains an invalid fingerprint"))?;
    }
    validate_nullable_sha256(intent.get("expected_basis_fingerprint"))
        .map_err(|_| invalid_intent("intent basis is invalid"))?;
    if promotion_commit {
        let presence = string_field(intent, "expected_basis_presence")?;
        let fingerprint = intent
            .get("expected_basis_fingerprint")
            .and_then(Value::as_str);
        let byte_length = intent
            .get("expected_basis_byte_length")
            .and_then(Value::as_u64);
        let identity_token = intent
            .get("expected_basis_identity_token")
            .and_then(Value::as_str);
        let version_token = intent
            .get("expected_basis_version_token")
            .and_then(Value::as_str);
        match presence {
            "absent"
                if fingerprint.is_none()
                    && byte_length.is_none()
                    && identity_token.is_none()
                    && version_token.is_none() => {}
            "present"
                if fingerprint.is_some()
                    && byte_length
                        .is_some_and(|length| (1..=MAX_RECORD_BYTES as u64).contains(&length))
                    && identity_token.is_some()
                    && version_token.is_some() =>
            {
                validate_native_token(identity_token.unwrap(), "native-id-v1:")
                    .map_err(|_| invalid_intent("intent native basis identity is invalid"))?;
                validate_native_token(version_token.unwrap(), "native-version-v1:")
                    .map_err(|_| invalid_intent("intent native basis version is invalid"))?;
            }
            _ => {
                return Err(invalid_intent(
                    "promotion basis presence and nullable observation fields are inconsistent",
                ));
            }
        }
    }
    let operation = operation_from_intent(intent)?;
    let request_subject = intent
        .get("request_subject")
        .ok_or_else(|| invalid_intent("transaction intent request subject is absent"))?;
    validate_request_subject(operation, request_subject)
        .map_err(|_| invalid_intent("transaction intent request subject is invalid"))?;
    let expected_request_fingerprint = derive_request_fingerprint(
        string_field(intent, "repository_identity_fingerprint")?,
        string_field(intent, "owner_contract_subject_fingerprint")?,
        string_field(intent, "domain_mutation_key_fingerprint")?,
        request_subject,
    )?;
    if string_field(intent, "request_fingerprint")? != expected_request_fingerprint {
        return Err(invalid_intent(
            "transaction intent request fingerprint does not derive exactly",
        ));
    }
    if promotion_commit
        && request_subject
            .get("expected_current_artifact_fingerprint")
            .cloned()
            .unwrap_or(Value::Null)
            != intent
                .get("expected_basis_fingerprint")
                .cloned()
                .unwrap_or(Value::Null)
    {
        return Err(invalid_intent(
            "promotion intent basis does not equal its exact request authority",
        ));
    }
    let transaction_id = string_field(intent, "transaction_id")?;
    let expected_transaction_id = derive_transaction_id(
        string_field(intent, "repository_identity_fingerprint")?,
        string_field(intent, "owner_contract_subject_fingerprint")?,
        operation,
        string_field(intent, "domain_mutation_key_fingerprint")?,
        string_field(intent, "request_fingerprint")?,
    )?;
    if transaction_id != expected_transaction_id {
        return Err(invalid_intent(
            "transaction ID does not derive from the exact request authority",
        ));
    }
    let outcome = string_field(intent, "planned_outcome")?;
    let outputs = intent_outputs(intent)?;
    match outcome {
        "commit" => {
            if !intent.get("refusal").is_some_and(Value::is_null) {
                return Err(invalid_intent("commit intent must have null refusal"));
            }
            if operation == GenericArtifactOperationV1::IntakeRecordAppend {
                validate_utc(string_field(intent, "sampled_finalized_at_utc")?)
                    .map_err(|_| invalid_intent("intake intent timestamp is invalid"))?;
            } else if !intent
                .get("sampled_finalized_at_utc")
                .is_some_and(Value::is_null)
            {
                return Err(invalid_intent("non-intake intent must be timestamp-free"));
            }
            validate_descriptor_contract(operation, &outputs, request_subject)?;
        }
        "refuse" => {
            if !outputs.is_empty()
                || !intent
                    .get("sampled_finalized_at_utc")
                    .is_some_and(Value::is_null)
            {
                return Err(refusal_conflict(
                    "established refusal contains outputs or a sampled timestamp",
                ));
            }
            let refusal: EstablishedRefusalV1 =
                serde_json::from_value(intent.get("refusal").cloned().unwrap_or(Value::Null))
                    .map_err(|_| invalid_intent("intent refusal is not exact-closed"))?;
            if refusal.code == EstablishedRefusalCodeV1::PublicationBasisConflict {
                return Err(invalid_intent(
                    "publication basis conflict cannot be established before publication",
                ));
            }
            validate_established_refusal(&refusal)?;
        }
        _ => return Err(invalid_intent("unknown planned outcome")),
    }
    Ok(())
}

fn validate_descriptor_contract(
    operation: GenericArtifactOperationV1,
    outputs: &[Value],
    request_subject: &Value,
) -> Result<(), GenericLineageStoreErrorV1> {
    if operation == GenericArtifactOperationV1::IntakeRecordAppend {
        if !(2..=16).contains(&outputs.len()) {
            return Err(invalid_intent(
                "intake intent output cardinality is outside the closed limit",
            ));
        }
        let final_ordinal = outputs.len() - 1;
        let mut tokens = BTreeSet::new();
        for (ordinal, descriptor) in outputs.iter().enumerate() {
            require_exact_fields(
                descriptor,
                &[
                    "ordinal",
                    "token",
                    "authority_class",
                    "final_ref",
                    "bytes_sha256",
                    "byte_length",
                    "install_mode",
                    "receipt_class",
                ],
                GenericLineageStoreErrorKindV1::InvalidTransactionIntent,
            )?;
            let token = string_field(descriptor, "token")?;
            let is_record = ordinal == final_ordinal;
            let tuple_is_exact = if is_record {
                token == "intake-record"
                    && string_field(descriptor, "authority_class")? == "semantic_record"
                    && string_field(descriptor, "install_mode")? == "create_new"
                    && descriptor.get("receipt_class").and_then(Value::as_str)
                        == Some("semantic_record")
            } else {
                valid_intake_value_token(token)
                    && string_field(descriptor, "authority_class")? == "subordinate_closure"
                    && string_field(descriptor, "install_mode")? == "create_new"
                    && descriptor.get("receipt_class").is_some_and(Value::is_null)
            };
            if descriptor.get("ordinal").and_then(Value::as_u64) != Some(ordinal as u64)
                || !tuple_is_exact
                || !tokens.insert(token)
            {
                return Err(invalid_intent("intake intent output tuple is not exact"));
            }
            validate_safe_ref(string_field(descriptor, "final_ref")?)?;
            validate_sha256(string_field(descriptor, "bytes_sha256")?)?;
            let fingerprint = fingerprint_from_output_ref(string_field(descriptor, "final_ref")?)?;
            validate_output_ref_binding(
                operation,
                request_subject,
                token,
                string_field(descriptor, "final_ref")?,
                &fingerprint,
            )
            .map_err(|_| invalid_intent("intake intent output ref is not request-owned"))?;
            if descriptor
                .get("byte_length")
                .and_then(Value::as_u64)
                .is_none_or(|length| length == 0 || length > MAX_RECORD_BYTES as u64)
            {
                return Err(invalid_intent("intent output byte length is invalid"));
            }
        }
        return Ok(());
    }
    let expected: &[(&str, &str, &str, Option<&str>)] = match operation {
        GenericArtifactOperationV1::IntakeRecordAppend => unreachable!(),
        GenericArtifactOperationV1::ArtifactCandidateAppend => &[
            (
                "normalized-content",
                "subordinate_closure",
                "create_new",
                None,
            ),
            (
                "validation-result",
                "subordinate_closure",
                "create_new",
                None,
            ),
            (
                "candidate-record",
                "semantic_record",
                "create_new",
                Some("semantic_record"),
            ),
        ],
        GenericArtifactOperationV1::ArtifactCandidatePromote => &[
            (
                "canonical-artifact",
                "canonical_truth",
                "replace_if_current",
                Some("canonical_truth"),
            ),
            (
                "promotion-record",
                "semantic_record",
                "create_new",
                Some("semantic_record"),
            ),
        ],
    };
    if outputs.len() != expected.len() {
        return Err(invalid_intent("intent output cardinality mismatch"));
    }
    for (ordinal, (descriptor, expected)) in outputs.iter().zip(expected).enumerate() {
        require_exact_fields(
            descriptor,
            &[
                "ordinal",
                "token",
                "authority_class",
                "final_ref",
                "bytes_sha256",
                "byte_length",
                "install_mode",
                "receipt_class",
            ],
            GenericLineageStoreErrorKindV1::InvalidTransactionIntent,
        )?;
        if descriptor.get("ordinal").and_then(Value::as_u64) != Some(ordinal as u64)
            || string_field(descriptor, "token")? != expected.0
            || string_field(descriptor, "authority_class")? != expected.1
            || string_field(descriptor, "install_mode")? != expected.2
        {
            return Err(invalid_intent("intent output tuple mismatch"));
        }
        let receipt = descriptor.get("receipt_class");
        match expected.3 {
            Some(value) if receipt.and_then(Value::as_str) != Some(value) => {
                return Err(invalid_intent("intent receipt class mismatch"));
            }
            None if !receipt.is_some_and(Value::is_null) => {
                return Err(invalid_intent(
                    "subordinate output cannot have a receipt class",
                ));
            }
            _ => {}
        }
        validate_safe_ref(string_field(descriptor, "final_ref")?)?;
        validate_sha256(string_field(descriptor, "bytes_sha256")?)?;
        let fingerprint = if string_field(descriptor, "authority_class")? == "canonical_truth" {
            string_field(descriptor, "bytes_sha256")?.to_owned()
        } else {
            fingerprint_from_output_ref(string_field(descriptor, "final_ref")?)?
        };
        validate_output_ref_binding(
            operation,
            request_subject,
            string_field(descriptor, "token")?,
            string_field(descriptor, "final_ref")?,
            &fingerprint,
        )
        .map_err(|_| invalid_intent("intent output ref is not request-owned"))?;
        if !descriptor
            .get("byte_length")
            .and_then(Value::as_u64)
            .is_some_and(|length| (1..=MAX_RECORD_BYTES as u64).contains(&length))
        {
            return Err(invalid_intent("intent output byte length is out of bounds"));
        }
    }
    Ok(())
}

fn validate_verified(verified: &Value, intent: &Value) -> Result<(), GenericLineageStoreErrorV1> {
    let promotion_commit = operation_from_intent(intent)?
        == GenericArtifactOperationV1::ArtifactCandidatePromote
        && string_field(intent, "planned_outcome")? == "commit";
    let mut fields = vec![
        "schema_id",
        "schema_version",
        "transaction_id",
        "intent_fingerprint",
        "planned_outcome",
        "refusal",
        "verification_basis",
        "staged_outputs",
        "verified_fingerprint",
    ];
    if promotion_commit {
        fields.push("publication_observation_fingerprint");
    }
    require_exact_fields(
        verified,
        &fields,
        GenericLineageStoreErrorKindV1::ConflictingTransactionState,
    )?;
    if string_field(verified, "schema_id")? != "handbook.generic-verified-stage"
        || string_field(verified, "schema_version")? != "1.0"
        || string_field(verified, "transaction_id")? != string_field(intent, "transaction_id")?
        || string_field(verified, "intent_fingerprint")?
            != string_field(intent, "intent_fingerprint")?
        || string_field(verified, "planned_outcome")? != string_field(intent, "planned_outcome")?
        || verified.get("refusal") != intent.get("refusal")
    {
        return Err(conflicting("verified stage does not bind the exact intent"));
    }
    if promotion_commit {
        validate_sha256(string_field(
            verified,
            "publication_observation_fingerprint",
        )?)
        .map_err(|_| conflicting("verified publication observation is invalid"))?;
    }
    let basis = string_field(verified, "verification_basis")?;
    let staged = verified
        .get("staged_outputs")
        .and_then(Value::as_array)
        .ok_or_else(|| conflicting("verified stage outputs are malformed"))?;
    if string_field(intent, "planned_outcome")? == "refuse" {
        if basis != "refusal_decision" || !staged.is_empty() {
            return Err(refusal_conflict("refusal verified stage is output-bearing"));
        }
    } else {
        if !matches!(basis, "staging" | "installed_complete") {
            return Err(conflicting("commit verified stage has an invalid basis"));
        }
        let outputs = intent_outputs(intent)?;
        let expected = outputs
            .iter()
            .enumerate()
            .map(|(ordinal, output)| {
                let reference = if basis == "staging" {
                    format!(
                        "staged/{}",
                        staged_name(ordinal, string_field(output, "token")?)
                    )
                } else {
                    string_field(output, "final_ref")?.to_owned()
                };
                Ok(json!({
                    "ordinal": ordinal,
                    "staged_ref": reference,
                    "bytes_sha256": string_field(output, "bytes_sha256")?,
                    "byte_length": output.get("byte_length").cloned().unwrap_or(Value::Null)
                }))
            })
            .collect::<Result<Vec<_>, GenericLineageStoreErrorV1>>()?;
        if staged != &expected {
            return Err(conflicting(
                "verified stage descriptors do not equal the exact intent projection",
            ));
        }
    }
    Ok(())
}

fn validate_marker(
    marker: &Value,
    intent: &Value,
    verified: &Value,
) -> Result<(), GenericLineageStoreErrorV1> {
    let promotion_commit = operation_from_intent(intent)?
        == GenericArtifactOperationV1::ArtifactCandidatePromote
        && string_field(intent, "planned_outcome")? == "commit";
    let terminal_conflict = promotion_commit
        && marker.get("outcome").and_then(Value::as_str) == Some("refused")
        && marker
            .get("refusal")
            .and_then(|value| value.get("code"))
            .and_then(Value::as_str)
            == Some("publication_basis_conflict");
    let expected = if terminal_conflict {
        let refusal: EstablishedRefusalV1 = serde_json::from_value(
            marker.get("refusal").cloned().unwrap_or(Value::Null),
        )
        .map_err(|_| {
            error(
                GenericLineageStoreErrorKindV1::InvalidCommitMarker,
                "publication refusal is not exact-closed",
            )
        })?;
        validate_established_refusal(&refusal)?;
        let publication_fingerprint = string_field(marker, "publication_result_fingerprint")?;
        validate_sha256(publication_fingerprint)?;
        finalize_control(
            json!({
                "schema_id": "handbook.generic-commit-marker",
                "schema_version": "1.0",
                "transaction_id": string_field(intent, "transaction_id")?,
                "intent_fingerprint": string_field(intent, "intent_fingerprint")?,
                "verified_fingerprint": string_field(verified, "verified_fingerprint")?,
                "publication_result_fingerprint": publication_fingerprint,
                "outcome": "refused",
                "refusal": refusal,
                "authoritative_outputs": [],
                "subordinate_outputs": []
            }),
            "marker_fingerprint",
        )?
    } else {
        let mut expected = build_marker(intent, verified)?;
        if promotion_commit {
            let publication_fingerprint = string_field(marker, "publication_result_fingerprint")?;
            validate_sha256(publication_fingerprint)?;
            let object = expected.as_object_mut().expect("marker object");
            object.remove("marker_fingerprint");
            object.insert(
                "publication_result_fingerprint".to_owned(),
                json!(publication_fingerprint),
            );
            expected = finalize_control(expected, "marker_fingerprint")?;
        }
        expected
    };
    if marker != &expected {
        let kind = if string_field(intent, "planned_outcome")? == "refuse" {
            GenericLineageStoreErrorKindV1::ConflictingRefusalState
        } else {
            GenericLineageStoreErrorKindV1::InvalidCommitMarker
        };
        return Err(error(
            kind,
            "commit marker does not derive from intent and verified stage",
        ));
    }
    Ok(())
}

fn validate_suffix_prefix(
    store: &GenericArtifactLineageStoreV1,
    pending: &Path,
    intent: &Value,
) -> Result<(), GenericLineageStoreErrorV1> {
    let evidence = pending.join("evidence.json");
    let result = store.result_path(string_field(intent, "transaction_id")?);
    let ledger = store.ledger_path(string_field(intent, "domain_mutation_key_fingerprint")?);
    let e = evidence
        .try_exists()
        .map_err(|_| io_error("evidence lookup failed"))?;
    let r = result
        .try_exists()
        .map_err(|_| io_error("result lookup failed"))?;
    let l = ledger
        .try_exists()
        .map_err(|_| io_error("ledger lookup failed"))?;
    if (r && !e) || (l && !r) {
        let kind = if string_field(intent, "planned_outcome")? == "refuse" {
            GenericLineageStoreErrorKindV1::ConflictingRefusalState
        } else {
            GenericLineageStoreErrorKindV1::ConflictingTransactionState
        };
        return Err(error(
            kind,
            "post-marker installed state is not an ordered prefix",
        ));
    }
    Ok(())
}

fn require_ledger_scope(
    ledger: &Value,
    request: &DerivedRequest,
) -> Result<(), GenericLineageStoreErrorV1> {
    validate_ledger_record(ledger)?;
    if string_field(ledger, "schema_id")? != "handbook.generic-domain-ledger-entry"
        || string_field(ledger, "schema_version")? != "1.0"
        || string_field(ledger, "repository_identity_fingerprint")?
            != request.repository_identity_fingerprint
        || string_field(ledger, "owner_contract_ref")? != GENERIC_ARTIFACT_OWNER_CONTRACT_REF
        || string_field(ledger, "owner_contract_subject_fingerprint")?
            != request.owner_contract_subject_fingerprint
        || string_field(ledger, "operation_id")? != request.operation.operation_id()
        || string_field(ledger, "domain_mutation_key_fingerprint")? != request.key_fingerprint
    {
        return Err(retained_error(
            "ledger scope does not match the exact domain request",
        ));
    }
    Ok(())
}

fn validate_ledger_record(ledger: &Value) -> Result<(), GenericLineageStoreErrorV1> {
    require_exact_fields(
        ledger,
        &[
            "schema_id",
            "schema_version",
            "repository_identity_fingerprint",
            "owner_contract_ref",
            "owner_contract_subject_fingerprint",
            "operation_id",
            "domain_mutation_key_fingerprint",
            "request_fingerprint",
            "state",
            "transaction_id",
            "result_ref",
            "result_fingerprint",
            "retained_until_utc",
            "entry_fingerprint",
        ],
        GenericLineageStoreErrorKindV1::RetainedResultMismatch,
    )?;
    if string_field(ledger, "schema_id")? != "handbook.generic-domain-ledger-entry"
        || string_field(ledger, "schema_version")? != "1.0"
        || string_field(ledger, "owner_contract_ref")? != GENERIC_ARTIFACT_OWNER_CONTRACT_REF
    {
        return Err(retained_error("ledger record identity is invalid"));
    }
    for field in [
        "repository_identity_fingerprint",
        "owner_contract_subject_fingerprint",
        "domain_mutation_key_fingerprint",
        "request_fingerprint",
    ] {
        validate_sha256(string_field(ledger, field)?)?;
    }
    let operation =
        GenericArtifactOperationV1::from_operation_id(string_field(ledger, "operation_id")?)
            .ok_or_else(|| retained_error("ledger operation is outside the closed set"))?;
    let transaction_id = string_field(ledger, "transaction_id")?;
    if !transaction_id.starts_with(&format!("{}_", operation.transaction_token()))
        || transaction_id.len() != operation.transaction_token().len() + 65
    {
        return Err(retained_error("ledger transaction ID is invalid"));
    }
    match string_field(ledger, "state")? {
        "retained_result" => {
            let expected_ref = format!(
                ".handbook/state/idempotency/generic-artifact-operations/results/{transaction_id}.json"
            );
            if string_field(ledger, "result_ref")? != expected_ref {
                return Err(retained_error("ledger result ref is not transaction-bound"));
            }
            validate_sha256(string_field(ledger, "result_fingerprint")?)?;
            validate_utc(string_field(ledger, "retained_until_utc")?)?;
        }
        "tombstone" => {
            if !ledger.get("result_ref").is_some_and(Value::is_null)
                || !ledger.get("result_fingerprint").is_some_and(Value::is_null)
                || !ledger.get("retained_until_utc").is_some_and(Value::is_null)
            {
                return Err(retained_error("ledger tombstone is not exact-null closed"));
            }
        }
        "active_hold" => {
            return Err(retained_error(
                "active holds cannot coexist with committed journal state",
            ))
        }
        _ => return Err(retained_error("unknown domain ledger state")),
    }
    Ok(())
}

fn validate_result_record(
    value: Value,
) -> Result<GenericDomainMutationResultV1, GenericLineageStoreErrorV1> {
    let result: GenericDomainMutationResultV1 = serde_json::from_value(value)
        .map_err(|_| retained_error("domain result is not exact-closed"))?;
    if result.schema_id != "handbook.generic-domain-mutation-result"
        || result.schema_version != "1.0"
    {
        return Err(retained_error("domain result record identity is invalid"));
    }
    let operation = GenericArtifactOperationV1::from_operation_id(&result.operation_id)
        .ok_or_else(|| retained_error("domain result operation is outside the closed set"))?;
    if !valid_transaction_id(operation, &result.transaction_id) {
        return Err(retained_error("domain result transaction ID is invalid"));
    }
    validate_sha256(&result.request_fingerprint)?;
    validate_sha256(&result.result_fingerprint)?;
    let expected_evidence_ref = format!(
        ".handbook/state/transactions/{}/{}.committed/evidence.json",
        operation.transaction_family(),
        result.transaction_id
    );
    if result.internal_transaction_evidence_ref.as_deref() != Some(&expected_evidence_ref) {
        return Err(retained_error(
            "domain result evidence ref is not transaction-bound",
        ));
    }
    validate_sha256(
        result
            .internal_transaction_evidence_fingerprint
            .as_deref()
            .ok_or_else(|| retained_error("domain result evidence fingerprint is absent"))?,
    )?;
    match result.outcome.as_str() {
        "committed" if result.refusal.is_none() && !result.authoritative_outputs.is_empty() => {}
        "refused" if result.refusal.is_some() && result.authoritative_outputs.is_empty() => {}
        _ => {
            return Err(retained_error(
                "domain result outcome, refusal, and outputs are inconsistent",
            ))
        }
    }
    let mut refs = BTreeSet::new();
    for output in &result.authoritative_outputs {
        validate_safe_ref(&output.relative_ref)?;
        validate_sha256(&output.fingerprint)?;
        if !refs.insert(output.relative_ref.as_str()) {
            return Err(retained_error(
                "domain result contains duplicate authoritative refs",
            ));
        }
    }
    Ok(result)
}

fn operation_from_intent(
    intent: &Value,
) -> Result<GenericArtifactOperationV1, GenericLineageStoreErrorV1> {
    GenericArtifactOperationV1::from_operation_id(string_field(intent, "operation_id")?)
        .ok_or_else(|| invalid_intent("intent operation ID is not closed"))
}

fn validate_transaction_directory(
    path: &Path,
    intent: &Value,
    suffix: &str,
) -> Result<(), GenericLineageStoreErrorV1> {
    let operation = operation_from_intent(intent)?;
    let expected_name = format!("{}.{}", string_field(intent, "transaction_id")?, suffix);
    let expected_family = operation.transaction_family();
    if file_name(path)? != expected_name
        || path
            .parent()
            .and_then(Path::file_name)
            .and_then(|name| name.to_str())
            != Some(expected_family)
    {
        return Err(invalid_intent(
            "transaction directory does not bind the exact operation and transaction ID",
        ));
    }
    Ok(())
}

fn intent_outputs(intent: &Value) -> Result<Vec<Value>, GenericLineageStoreErrorV1> {
    intent
        .get("outputs")
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| invalid_intent("intent outputs are not an array"))
}

fn require_descriptor_bytes(
    descriptor: &Value,
    bytes: &[u8],
) -> Result<(), GenericLineageStoreErrorV1> {
    if string_field(descriptor, "bytes_sha256")? != sha256_prefixed(bytes)
        || descriptor.get("byte_length").and_then(Value::as_u64) != Some(bytes.len() as u64)
    {
        return Err(conflicting(
            "installed or staged output bytes disagree with intent",
        ));
    }
    Ok(())
}

fn validate_persisted_output_bytes(
    intent: &Value,
    ordinal: usize,
    final_ordinal: usize,
    descriptor: &Value,
    bytes: &[u8],
) -> Result<(), GenericLineageStoreErrorV1> {
    require_descriptor_bytes(descriptor, bytes)?;
    let operation = operation_from_intent(intent)?;
    let fingerprint = if string_field(descriptor, "authority_class")? == "canonical_truth" {
        string_field(descriptor, "bytes_sha256")?.to_owned()
    } else {
        fingerprint_from_output_ref(string_field(descriptor, "final_ref")?)?
    };
    match (operation, ordinal) {
        (GenericArtifactOperationV1::IntakeRecordAppend, value) if value < final_ordinal => {
            let value = parse_jcs_lf(bytes)?;
            if sha256_prefixed(&canonical_json(&value)?) != fingerprint {
                return Err(invalid_intent(
                    "persisted intake value fingerprint does not recompute",
                ));
            }
        }
        (GenericArtifactOperationV1::IntakeRecordAppend, value) if value == final_ordinal => {
            validate_runtime_record(
                bytes,
                "handbook.artifact-intake-record",
                "1.2",
                "intake_record_id",
                "record_fingerprint",
                "intake",
                &fingerprint,
            )?;
        }
        (GenericArtifactOperationV1::ArtifactCandidateAppend, 0) => {
            let value = parse_jcs_lf(bytes)?;
            if sha256_prefixed(&canonical_json(&value)?) != fingerprint {
                return Err(invalid_intent(
                    "persisted candidate content fingerprint does not recompute",
                ));
            }
        }
        (GenericArtifactOperationV1::ArtifactCandidateAppend, 1) => validate_runtime_record(
            bytes,
            "handbook.artifact-validation-result",
            "1.0",
            "validation_result_id",
            "validation_result_fingerprint",
            "validation",
            &fingerprint,
        )?,
        (GenericArtifactOperationV1::ArtifactCandidateAppend, 2) => validate_runtime_record(
            bytes,
            "handbook.artifact-candidate",
            "1.4",
            "candidate_id",
            "candidate_fingerprint",
            "candidate",
            &fingerprint,
        )?,
        (GenericArtifactOperationV1::ArtifactCandidatePromote, 0) => {
            if sha256_prefixed(bytes) != fingerprint {
                return Err(invalid_intent(
                    "persisted canonical artifact fingerprint does not recompute",
                ));
            }
        }
        (GenericArtifactOperationV1::ArtifactCandidatePromote, 1) => validate_runtime_record(
            bytes,
            "handbook.artifact-promotion-record",
            "1.2",
            "promotion_id",
            "promotion_fingerprint",
            "promotion",
            &fingerprint,
        )?,
        _ => return Err(invalid_intent("persisted output ordinal is not closed")),
    }
    validate_runtime_record_bindings(
        operation,
        ordinal,
        final_ordinal,
        bytes,
        intent
            .get("request_subject")
            .ok_or_else(|| invalid_intent("intent request subject is absent"))?,
        string_field(intent, "operation_context_fingerprint")?,
        match intent.get("sampled_finalized_at_utc") {
            Some(Value::String(value)) => Some(value.as_str()),
            _ => None,
        },
    )
    .map_err(|_| invalid_intent("persisted runtime output authority is invalid"))
}

fn validate_persisted_cross_bindings(
    intent: &Value,
    outputs: &[Value],
    bytes: &[Vec<u8>],
) -> Result<(), GenericLineageStoreErrorV1> {
    match operation_from_intent(intent)? {
        GenericArtifactOperationV1::IntakeRecordAppend => {
            let record = parse_jcs_lf(bytes.last().expect("intake output is nonempty"))?;
            let cited = record
                .get("coverage_results")
                .and_then(Value::as_array)
                .ok_or_else(|| invalid_intent("intake coverage results are malformed"))?
                .iter()
                .map(|result| {
                    Ok((
                        string_field(result, "value_ref")?.to_owned(),
                        string_field(result, "value_fingerprint")?.to_owned(),
                    ))
                })
                .collect::<Result<BTreeSet<_>, GenericLineageStoreErrorV1>>()?;
            let planned = outputs[..outputs.len() - 1]
                .iter()
                .map(|descriptor| {
                    let reference = string_field(descriptor, "final_ref")?.to_owned();
                    let fingerprint = fingerprint_from_output_ref(&reference)?;
                    Ok((reference, fingerprint))
                })
                .collect::<Result<BTreeSet<_>, GenericLineageStoreErrorV1>>()?;
            if cited != planned {
                return Err(invalid_intent(
                    "persisted intake closure does not equal its output descriptors",
                ));
            }
        }
        GenericArtifactOperationV1::ArtifactCandidateAppend => {
            let validation = parse_jcs_lf(&bytes[1])?;
            let candidate = parse_jcs_lf(&bytes[2])?;
            let content_ref = string_field(&outputs[0], "final_ref")?;
            let content_fingerprint = fingerprint_from_output_ref(content_ref)?;
            let validation_ref = string_field(&outputs[1], "final_ref")?;
            if string_field(&validation, "normalized_content_ref")? != content_ref
                || string_field(&validation, "normalized_content_fingerprint")?
                    != content_fingerprint
                || string_field(&candidate, "normalized_content_ref")? != content_ref
                || string_field(&candidate, "normalized_content_fingerprint")?
                    != content_fingerprint
                || candidate.get("validation_result_refs") != Some(&json!([validation_ref]))
            {
                return Err(invalid_intent(
                    "persisted candidate closure is not exactly cross-bound",
                ));
            }
        }
        GenericArtifactOperationV1::ArtifactCandidatePromote => {
            let promotion = parse_jcs_lf(&bytes[1])?;
            if string_field(&promotion, "canonical_artifact_ref")?
                != string_field(&outputs[0], "final_ref")?
                || string_field(&promotion, "canonical_artifact_fingerprint")?
                    != string_field(&outputs[0], "bytes_sha256")?
            {
                return Err(invalid_intent(
                    "persisted promotion does not bind its canonical output",
                ));
            }
        }
    }
    Ok(())
}

fn intent_expected_basis(intent: &Value) -> Result<Option<&str>, GenericLineageStoreErrorV1> {
    match intent.get("expected_basis_fingerprint") {
        Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => Ok(Some(value)),
        _ => Err(invalid_intent(
            "intent expected basis must be null or a fingerprint string",
        )),
    }
}

fn intent_expected_basis_version_token(
    intent: &Value,
) -> Result<Option<&str>, GenericLineageStoreErrorV1> {
    if operation_from_intent(intent)? != GenericArtifactOperationV1::ArtifactCandidatePromote
        || string_field(intent, "planned_outcome")? != "commit"
    {
        return Ok(None);
    }
    let token = string_field(intent, "expected_basis_version_token")?;
    validate_native_token(token, "native-version-v1:")?;
    Ok(Some(token))
}

fn validate_native_token(token: &str, prefix: &str) -> Result<(), GenericLineageStoreErrorV1> {
    let Some(hex) = token.strip_prefix(prefix) else {
        return Err(invalid_intent("native observation token prefix is invalid"));
    };
    if hex.len() != 64
        || !hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid_intent("native observation token digest is invalid"));
    }
    Ok(())
}

fn verify_compare_and_write_path(
    path: &Path,
    expected_basis_fingerprint: Option<&str>,
    expected_version_token: Option<&str>,
) -> Result<RetainedBasisObservation, GenericLineageStoreErrorV1> {
    let (observed_fingerprint, version_token, guard) = match path.try_exists() {
        Ok(false) => (None, native_absent_version_token()?, None),
        Ok(true) => {
            let (guard, bytes, _identity_token, version_token) =
                observe_retained_regular_file(path, MAX_RECORD_BYTES)?;
            (Some(sha256_prefixed(&bytes)), version_token, Some(guard))
        }
        Err(_) => return Err(io_error("canonical basis lookup failed")),
    };
    if observed_fingerprint.as_deref() != expected_basis_fingerprint
        || expected_version_token.is_some_and(|expected| expected != version_token)
    {
        return Err(error(
            GenericLineageStoreErrorKindV1::BasisMismatch,
            "locked canonical content or native version does not equal the exact expected basis",
        ));
    }
    Ok(RetainedBasisObservation {
        version_token,
        _guard: guard,
    })
}

fn native_absent_version_token() -> Result<String, GenericLineageStoreErrorV1> {
    native_token(
        "native-version-v1:",
        &json!({"platform": std::env::consts::OS, "state": "absent"}),
    )
}

fn native_token(prefix: &str, value: &Value) -> Result<String, GenericLineageStoreErrorV1> {
    Ok(format!(
        "{prefix}{}",
        fingerprint_hex(&sha256_prefixed(&canonical_json(value)?))?
    ))
}

fn observe_retained_regular_file(
    path: &Path,
    limit: usize,
) -> Result<(TrustedRepoFile, Vec<u8>, String, String), GenericLineageStoreErrorV1> {
    let (file, bytes) = observe_retained_bytes(path, limit)?;
    run_after_retained_read_hook(path);
    let metadata = file
        .metadata()
        .map_err(|_| io_error("retained native metadata failed"))?;
    let (identity_token, version_token) = native_bound_tokens(&metadata, path)?;
    Ok((file, bytes, identity_token, version_token))
}

fn observe_retained_bytes(
    path: &Path,
    limit: usize,
) -> Result<(TrustedRepoFile, Vec<u8>), GenericLineageStoreErrorV1> {
    let (repo_root, relative) = store_relative_path(path)?;
    let workspace = CanonicalWorkspace::new(&repo_root);
    let normalized = workspace
        .normalize_repo_relative(&relative)
        .map_err(unsafe_ref)?;
    let file = workspace
        .trusted_read_strict(&normalized)
        .map_err(|_| unsafe_ref("native observation failed strict component admission"))?;
    let (bytes, exceeded) = file
        .read_bytes_bounded_stable(limit)
        .map_err(|_| io_error("native observation stable read failed"))?;
    if exceeded {
        return Err(bound_error("native observation exceeds its byte limit"));
    }
    Ok((file, bytes))
}

fn native_path_tokens(path: &Path) -> Result<(String, String), GenericLineageStoreErrorV1> {
    reject_link(path, false)?;
    let metadata =
        fs::symlink_metadata(path).map_err(|_| io_error("native observation metadata failed"))?;
    let (identity, version, _) = native_metadata_subjects(&metadata, path)?;
    Ok((
        native_token("native-id-v1:", &identity)?,
        native_token("native-version-v1:", &version)?,
    ))
}

fn native_bound_tokens(
    retained_metadata: &fs::Metadata,
    path: &Path,
) -> Result<(String, String), GenericLineageStoreErrorV1> {
    reject_link(path, false)?;
    let path_before =
        fs::symlink_metadata(path).map_err(|_| io_error("native bound metadata failed"))?;
    if !same_native_metadata(retained_metadata, &path_before) {
        return Err(conflicting(
            "retained native handle is no longer bound to the observed path",
        ));
    }
    let (path_identity_token, path_version_token) = native_path_tokens(path)?;
    let path_after =
        fs::symlink_metadata(path).map_err(|_| io_error("native bound metadata failed"))?;
    if !same_native_metadata(retained_metadata, &path_after)
        || !same_native_metadata(&path_before, &path_after)
    {
        return Err(conflicting(
            "native path changed while binding its retained handle",
        ));
    }
    let (handle_identity, handle_version, _) = native_metadata_subjects(retained_metadata, path)?;
    let handle_identity_token = native_token("native-id-v1:", &handle_identity)?;
    let handle_version_token = native_token("native-version-v1:", &handle_version)?;
    if handle_identity_token != path_identity_token || handle_version_token != path_version_token {
        return Err(conflicting(
            "retained native handle subjects disagree with the path observation",
        ));
    }
    Ok((handle_identity_token, handle_version_token))
}

#[cfg_attr(
    windows,
    allow(
        unsafe_code,
        reason = "Windows retained-handle metadata requires direct, locally declared Win32 FFI"
    )
)]
fn native_metadata_subjects(
    metadata: &fs::Metadata,
    path: &Path,
) -> Result<(Value, Value, Value), GenericLineageStoreErrorV1> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let entries = Vec::<Value>::new();
        let entries_digest = sha256_prefixed(&canonical_json(&json!(entries))?);
        let platform_observation = json!({
            "platform_family": "unix",
            "file_type": "regular",
            "device_id": metadata.dev().to_string(),
            "inode": metadata.ino().to_string(),
            "byte_length": metadata.len(),
            "link_count": metadata.nlink(),
            "mode_octal": format!("0{:o}", metadata.mode()),
            "uid": metadata.uid().to_string(),
            "gid": metadata.gid().to_string(),
            "mtime": {
                "seconds": metadata.mtime().to_string(),
                "nanoseconds": metadata.mtime_nsec()
            },
            "ctime": {
                "seconds": metadata.ctime().to_string(),
                "nanoseconds": metadata.ctime_nsec()
            },
            "xattrs": {
                "entries": entries,
                "entries_digest": entries_digest
            }
        });
        let identity = json!({
            "platform_family": "unix",
            "file_type": "regular",
            "device_id": metadata.dev().to_string(),
            "inode": metadata.ino().to_string(),
        });
        return Ok((identity, platform_observation.clone(), platform_observation));
    }
    #[cfg(windows)]
    {
        use std::ffi::c_void;
        use std::os::windows::fs::{MetadataExt, OpenOptionsExt};
        use std::os::windows::io::AsRawHandle;
        use std::ptr;

        #[repr(C)]
        #[derive(Clone, Copy)]
        struct FileId128 {
            identifier: [u8; 16],
        }

        #[repr(C)]
        #[derive(Clone, Copy)]
        struct FileIdInfo {
            volume_serial_number: u64,
            file_id: FileId128,
        }

        #[repr(C)]
        #[derive(Clone, Copy)]
        struct FileBasicInfo {
            creation_time: i64,
            last_access_time: i64,
            last_write_time: i64,
            change_time: i64,
            file_attributes: u32,
        }

        #[repr(C)]
        #[derive(Clone, Copy)]
        struct FileStandardInfo {
            allocation_size: i64,
            end_of_file: i64,
            number_of_links: u32,
            delete_pending: i32,
            directory: i32,
        }

        #[repr(C)]
        #[derive(Clone, Copy)]
        struct FileAttributeTagInfo {
            file_attributes: u32,
            reparse_tag: u32,
        }

        #[repr(C)]
        struct AclSizeInformation {
            ace_count: u32,
            acl_bytes_in_use: u32,
            acl_bytes_free: u32,
        }

        #[link(name = "kernel32")]
        unsafe extern "system" {
            fn GetFileInformationByHandleEx(
                file: *mut c_void,
                class: i32,
                information: *mut c_void,
                information_size: u32,
            ) -> i32;
            fn DeviceIoControl(
                device: *mut c_void,
                control_code: u32,
                input: *mut c_void,
                input_size: u32,
                output: *mut c_void,
                output_size: u32,
                returned: *mut u32,
                overlapped: *mut c_void,
            ) -> i32;
            fn LocalFree(memory: *mut c_void) -> *mut c_void;
        }

        #[link(name = "advapi32")]
        unsafe extern "system" {
            fn GetSecurityInfo(
                handle: *mut c_void,
                object_type: u32,
                security_information: u32,
                owner: *mut *mut c_void,
                group: *mut *mut c_void,
                dacl: *mut *mut c_void,
                sacl: *mut *mut c_void,
                security_descriptor: *mut *mut c_void,
            ) -> u32;
            fn ConvertSidToStringSidW(sid: *mut c_void, text: *mut *mut u16) -> i32;
            fn GetSecurityDescriptorControl(
                security_descriptor: *mut c_void,
                control: *mut u16,
                revision: *mut u32,
            ) -> i32;
            fn GetAclInformation(
                acl: *mut c_void,
                information: *mut c_void,
                information_size: u32,
                information_class: u32,
            ) -> i32;
        }

        const FILE_SHARE_READ: u32 = 0x0000_0001;
        const FILE_SHARE_WRITE: u32 = 0x0000_0002;
        const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
        const FILE_BASIC_INFO_CLASS: i32 = 0;
        const FILE_STANDARD_INFO_CLASS: i32 = 1;
        const FILE_STREAM_INFO_CLASS: i32 = 7;
        const FILE_ATTRIBUTE_TAG_INFO_CLASS: i32 = 9;
        const FILE_ID_INFO_CLASS: i32 = 18;
        const SE_FILE_OBJECT: u32 = 1;
        const OWNER_SECURITY_INFORMATION: u32 = 0x0000_0001;
        const GROUP_SECURITY_INFORMATION: u32 = 0x0000_0002;
        const DACL_SECURITY_INFORMATION: u32 = 0x0000_0004;
        const ACL_SIZE_INFORMATION_CLASS: u32 = 2;
        const FSCTL_READ_FILE_USN_DATA: u32 = 0x0009_00eb;

        let parent_path = path
            .parent()
            .ok_or_else(|| unsafe_ref("Windows native observation path has no parent"))?;
        let child_name = path
            .file_name()
            .ok_or_else(|| unsafe_ref("Windows native observation path has no child name"))?;
        let parent = OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
            .open(parent_path)
            .map_err(|_| unsafe_ref("Windows native observation parent open failed"))?;
        let retained = open_retained_parent_relative_nt(&parent, child_name, false, false)?;
        let retained_metadata = retained
            .metadata()
            .map_err(|_| io_error("Windows native observation metadata failed"))?;
        if !same_native_metadata(metadata, &retained_metadata) {
            return Err(conflicting(
                "Windows native observation handle disagrees with retained metadata",
            ));
        }
        let handle = retained.as_raw_handle();

        let observe = || -> Result<Value, GenericLineageStoreErrorV1> {
            let mut file_id = FileIdInfo {
                volume_serial_number: 0,
                file_id: FileId128 {
                    identifier: [0; 16],
                },
            };
            let mut basic = FileBasicInfo {
                creation_time: 0,
                last_access_time: 0,
                last_write_time: 0,
                change_time: 0,
                file_attributes: 0,
            };
            let mut standard = FileStandardInfo {
                allocation_size: 0,
                end_of_file: 0,
                number_of_links: 0,
                delete_pending: 0,
                directory: 0,
            };
            let mut tag = FileAttributeTagInfo {
                file_attributes: 0,
                reparse_tag: 0,
            };

            // SAFETY: `handle` is owned by `retained` for the whole observation. Each output
            // pointer names an initialized, correctly sized `repr(C)` buffer for the selected
            // information class, and no call retains a pointer after returning.
            let fixed_ok = unsafe {
                GetFileInformationByHandleEx(
                    handle,
                    FILE_ID_INFO_CLASS,
                    ptr::from_mut(&mut file_id).cast(),
                    size_of::<FileIdInfo>() as u32,
                ) != 0
                    && GetFileInformationByHandleEx(
                        handle,
                        FILE_BASIC_INFO_CLASS,
                        ptr::from_mut(&mut basic).cast(),
                        size_of::<FileBasicInfo>() as u32,
                    ) != 0
                    && GetFileInformationByHandleEx(
                        handle,
                        FILE_STANDARD_INFO_CLASS,
                        ptr::from_mut(&mut standard).cast(),
                        size_of::<FileStandardInfo>() as u32,
                    ) != 0
                    && GetFileInformationByHandleEx(
                        handle,
                        FILE_ATTRIBUTE_TAG_INFO_CLASS,
                        ptr::from_mut(&mut tag).cast(),
                        size_of::<FileAttributeTagInfo>() as u32,
                    ) != 0
            };
            if !fixed_ok {
                return Err(io_error("Windows retained-handle information query failed"));
            }
            if standard.directory != 0
                || standard.delete_pending != 0
                || standard.end_of_file <= 0
                || standard.end_of_file as u64 != metadata.file_size()
                || standard.number_of_links != 1
                || tag.file_attributes & FILE_ATTRIBUTE_REPARSE_POINT != 0
                || tag.reparse_tag != 0
                || basic.file_attributes != tag.file_attributes
            {
                return Err(conflicting(
                    "Windows retained object is not one stable single-link regular file",
                ));
            }
            if file_id.file_id.identifier == [0; 16] || file_id.file_id.identifier == [u8::MAX; 16]
            {
                return Err(conflicting("native Windows file identity is invalid"));
            }

            query_object_id_absence_nt(&retained)?;

            let mut owner = ptr::null_mut();
            let mut group = ptr::null_mut();
            let mut dacl = ptr::null_mut();
            let mut descriptor = ptr::null_mut();
            // SAFETY: every out-pointer is valid for the duration of the call. The returned
            // descriptor is released exactly once with `LocalFree`; SACL is intentionally null.
            let security_status = unsafe {
                GetSecurityInfo(
                    handle,
                    SE_FILE_OBJECT,
                    OWNER_SECURITY_INFORMATION
                        | GROUP_SECURITY_INFORMATION
                        | DACL_SECURITY_INFORMATION,
                    &mut owner,
                    &mut group,
                    &mut dacl,
                    ptr::null_mut(),
                    &mut descriptor,
                )
            };
            if security_status != 0 || descriptor.is_null() || owner.is_null() || group.is_null() {
                if !descriptor.is_null() {
                    // SAFETY: `descriptor` came from `GetSecurityInfo` and has not been freed.
                    unsafe {
                        LocalFree(descriptor);
                    }
                }
                return Err(io_error(
                    "Windows retained-handle security observation failed",
                ));
            }
            let security_result = (|| -> Result<Value, GenericLineageStoreErrorV1> {
                let sid_text = |sid: *mut c_void| -> Result<String, GenericLineageStoreErrorV1> {
                    let mut wide = ptr::null_mut();
                    // SAFETY: `sid` is a valid SID pointer returned with the live descriptor;
                    // `wide` is an initialized out-pointer and is freed after the terminated
                    // string has been copied.
                    if unsafe { ConvertSidToStringSidW(sid, &mut wide) } == 0 || wide.is_null() {
                        return Err(io_error("Windows SID conversion failed"));
                    }
                    let mut length = 0usize;
                    // SAFETY: `ConvertSidToStringSidW` returns a NUL-terminated allocation.
                    unsafe {
                        while *wide.add(length) != 0 {
                            length += 1;
                        }
                    }
                    // SAFETY: the preceding scan found the allocation's terminator.
                    let text =
                        String::from_utf16(unsafe { std::slice::from_raw_parts(wide, length) })
                            .map_err(|_| io_error("Windows SID was not valid UTF-16"));
                    // SAFETY: `wide` was allocated by `ConvertSidToStringSidW` and is freed once.
                    unsafe {
                        LocalFree(wide.cast());
                    }
                    text
                };
                let owner_sid = sid_text(owner)?;
                let group_sid = sid_text(group)?;
                let mut control = 0u16;
                let mut revision = 0u32;
                // SAFETY: the descriptor remains live and both scalar out-pointers are valid.
                if unsafe { GetSecurityDescriptorControl(descriptor, &mut control, &mut revision) }
                    == 0
                {
                    return Err(io_error("Windows security descriptor control query failed"));
                }
                let (dacl_present, dacl_byte_length, dacl_sha256) = if dacl.is_null() {
                    (false, Value::Null, Value::Null)
                } else {
                    let mut size = AclSizeInformation {
                        ace_count: 0,
                        acl_bytes_in_use: 0,
                        acl_bytes_free: 0,
                    };
                    // SAFETY: `dacl` belongs to the live descriptor and `size` is a correctly
                    // sized initialized output buffer for `AclSizeInformation`.
                    if unsafe {
                        GetAclInformation(
                            dacl,
                            ptr::from_mut(&mut size).cast(),
                            size_of::<AclSizeInformation>() as u32,
                            ACL_SIZE_INFORMATION_CLASS,
                        )
                    } == 0
                    {
                        return Err(io_error("Windows DACL size query failed"));
                    }
                    // SAFETY: `AclBytesInUse` is reported by the API for this live ACL.
                    let bytes = unsafe {
                        std::slice::from_raw_parts(
                            dacl.cast::<u8>(),
                            size.acl_bytes_in_use as usize,
                        )
                    };
                    (
                        true,
                        json!(size.acl_bytes_in_use),
                        json!(sha256_prefixed(bytes)),
                    )
                };
                let mut security = json!({
                    "owner_sid": owner_sid,
                    "group_sid": group_sid,
                    "security_descriptor_control_hex": format!("0x{control:04X}"),
                    "dacl_present": dacl_present,
                    "dacl_byte_length": dacl_byte_length,
                    "dacl_sha256": dacl_sha256
                });
                let digest = sha256_prefixed(&canonical_json(&security)?);
                security
                    .as_object_mut()
                    .expect("security object")
                    .insert("descriptor_digest".to_owned(), json!(digest));
                Ok(security)
            })();
            // SAFETY: `descriptor` came from `GetSecurityInfo` and is released exactly once.
            unsafe {
                LocalFree(descriptor);
            }
            let security = security_result?;

            let mut stream_buffer = vec![0u8; MAX_RECORD_BYTES];
            // SAFETY: the vector exposes a writable buffer of the exact advertised length and
            // the synchronous call completes before the buffer is inspected.
            if unsafe {
                GetFileInformationByHandleEx(
                    handle,
                    FILE_STREAM_INFO_CLASS,
                    stream_buffer.as_mut_ptr().cast(),
                    stream_buffer.len() as u32,
                )
            } == 0
            {
                return Err(io_error("Windows retained-handle stream inventory failed"));
            }
            let mut stream_entries = Vec::new();
            let mut offset = 0usize;
            loop {
                if offset
                    .checked_add(24)
                    .is_none_or(|end| end > stream_buffer.len())
                {
                    return Err(conflicting("Windows stream inventory is malformed"));
                }
                let next = u32::from_le_bytes(
                    stream_buffer[offset..offset + 4]
                        .try_into()
                        .expect("four-byte stream offset"),
                ) as usize;
                let name_bytes = u32::from_le_bytes(
                    stream_buffer[offset + 4..offset + 8]
                        .try_into()
                        .expect("four-byte stream name length"),
                ) as usize;
                let size = i64::from_le_bytes(
                    stream_buffer[offset + 8..offset + 16]
                        .try_into()
                        .expect("eight-byte stream size"),
                );
                let allocation = i64::from_le_bytes(
                    stream_buffer[offset + 16..offset + 24]
                        .try_into()
                        .expect("eight-byte stream allocation"),
                );
                if name_bytes == 0
                    || name_bytes % 2 != 0
                    || size < 0
                    || allocation < 0
                    || offset
                        .checked_add(24 + name_bytes)
                        .is_none_or(|end| end > stream_buffer.len())
                {
                    return Err(conflicting("Windows stream inventory is malformed"));
                }
                let name = stream_buffer[offset + 24..offset + 24 + name_bytes]
                    .chunks_exact(2)
                    .map(|unit| u16::from_le_bytes([unit[0], unit[1]]))
                    .collect::<Vec<_>>();
                let name = String::from_utf16(&name)
                    .map_err(|_| conflicting("Windows stream name is not valid UTF-16"))?;
                stream_entries.push(json!({
                    "name": name,
                    "byte_length": size.to_string(),
                    "allocation_byte_length": allocation.to_string()
                }));
                if next == 0 {
                    break;
                }
                if next < 24 + name_bytes
                    || next % 8 != 0
                    || offset
                        .checked_add(next)
                        .is_none_or(|next_offset| next_offset >= stream_buffer.len())
                {
                    return Err(conflicting("Windows stream chain is malformed"));
                }
                offset += next;
            }
            stream_entries.sort_by(|left, right| {
                left["name"]
                    .as_str()
                    .unwrap_or_default()
                    .encode_utf16()
                    .cmp(right["name"].as_str().unwrap_or_default().encode_utf16())
            });
            if stream_entries.len() != 1
                || stream_entries[0]["name"] != "::$DATA"
                || stream_entries[0]["byte_length"]
                    .as_str()
                    .and_then(|value| value.parse::<i64>().ok())
                    != Some(standard.end_of_file)
            {
                return Err(conflicting(
                    "Windows publication file has a non-default or inconsistent stream",
                ));
            }
            let streams = json!({
                "entries": stream_entries,
                "non_default_stream_count": 0,
                "entries_digest": sha256_prefixed(&canonical_json(&json!(stream_entries))?)
            });

            let mut usn_buffer = [0u8; 4096];
            let mut usn_bytes = 0u32;
            // SAFETY: the retained synchronous handle is live; input and overlapped pointers
            // are null by contract, and the output slice plus byte-count pointer are valid.
            if unsafe {
                DeviceIoControl(
                    handle,
                    FSCTL_READ_FILE_USN_DATA,
                    ptr::null_mut(),
                    0,
                    usn_buffer.as_mut_ptr().cast(),
                    usn_buffer.len() as u32,
                    &mut usn_bytes,
                    ptr::null_mut(),
                )
            } == 0
                || usn_bytes < 32
            {
                return Err(io_error("Windows retained-handle USN observation failed"));
            }
            let record_length =
                u32::from_le_bytes(usn_buffer[0..4].try_into().expect("USN record length"))
                    as usize;
            let usn = i64::from_le_bytes(usn_buffer[24..32].try_into().expect("USN value"));
            if record_length < 32 || record_length > usn_bytes as usize || usn < 0 {
                return Err(conflicting(
                    "Windows retained-handle USN record is malformed",
                ));
            }

            let file_id_hex = format!(
                "0x{}",
                file_id
                    .file_id
                    .identifier
                    .iter()
                    .map(|byte| format!("{byte:02X}"))
                    .collect::<String>()
            );
            Ok(json!({
                "platform_family": "windows",
                "file_type": "regular",
                "reparse_point": false,
                "reparse_tag_hex": null,
                "volume_serial_number_hex": format!("0x{:016X}", file_id.volume_serial_number),
                "file_id_hex": file_id_hex,
                "object_id_query": "absent_status_objectid_not_found",
                "byte_length": standard.end_of_file as u64,
                "link_count": standard.number_of_links,
                "creation_time_100ns": basic.creation_time.to_string(),
                "last_write_time_100ns": basic.last_write_time.to_string(),
                "change_time_100ns": basic.change_time.to_string(),
                "file_attributes_hex": format!("0x{:08X}", basic.file_attributes),
                "security": security,
                "sacl_policy": "excluded_not_observed_not_trusted",
                "streams": streams,
                "usn": usn.to_string()
            }))
        };

        let first = observe()?;
        let second = observe()?;
        if first != second {
            return Err(conflicting(
                "Windows retained-handle metadata changed between complete observations",
            ));
        }
        let identity = json!({
            "platform_family": "windows",
            "file_type": "regular",
            "volume_serial_number_hex": first["volume_serial_number_hex"],
            "file_id_hex": first["file_id_hex"]
        });
        return Ok((identity, first.clone(), first));
    }
    #[allow(unreachable_code)]
    Err(error(
        GenericLineageStoreErrorKindV1::UnsupportedPlatform,
        "retained native metadata requires Unix or Windows",
    ))
}

#[cfg(windows)]
#[allow(
    unsafe_code,
    reason = "the frozen retained-parent contract requires the exact local NtCreateFile ABI"
)]
fn open_retained_parent_relative_nt(
    parent: &File,
    child: &std::ffi::OsStr,
    delete_access: bool,
    directory: bool,
) -> Result<File, GenericLineageStoreErrorV1> {
    use std::ffi::c_void;
    use std::os::windows::ffi::OsStrExt;
    use std::os::windows::io::{AsRawHandle, FromRawHandle};
    use std::ptr;

    #[repr(C)]
    struct UnicodeString {
        length: u16,
        maximum_length: u16,
        buffer: *mut u16,
    }

    #[repr(C)]
    struct ObjectAttributes {
        length: u32,
        root_directory: *mut c_void,
        object_name: *mut UnicodeString,
        attributes: u32,
        security_descriptor: *mut c_void,
        security_quality_of_service: *mut c_void,
    }

    #[repr(C)]
    union IoStatusValue {
        status: i32,
        pointer: *mut c_void,
    }

    #[repr(C)]
    struct IoStatusBlock {
        value: IoStatusValue,
        information: usize,
    }

    #[link(name = "ntdll")]
    unsafe extern "system" {
        fn NtCreateFile(
            file_handle: *mut *mut c_void,
            desired_access: u32,
            object_attributes: *mut ObjectAttributes,
            io_status_block: *mut IoStatusBlock,
            allocation_size: *mut i64,
            file_attributes: u32,
            share_access: u32,
            create_disposition: u32,
            create_options: u32,
            ea_buffer: *mut c_void,
            ea_length: u32,
        ) -> i32;
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn CloseHandle(handle: *mut c_void) -> i32;
    }

    const STATUS_UNSUCCESSFUL: i32 = 0xc000_0001u32 as i32;
    const STATUS_SUCCESS: i32 = 0;
    const FILE_GENERIC_READ: u32 = 0x0012_0089;
    const DELETE: u32 = 0x0001_0000;
    const FILE_SHARE_READ: u32 = 0x0000_0001;
    const FILE_SHARE_WRITE: u32 = 0x0000_0002;
    const FILE_SHARE_DELETE: u32 = 0x0000_0004;
    const FILE_OPEN: u32 = 1;
    const FILE_OPENED: usize = 1;
    const FILE_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 0x0000_0020;
    const FILE_NON_DIRECTORY_FILE: u32 = 0x0000_0040;
    const OBJ_CASE_INSENSITIVE: u32 = 0x0000_0040;
    const OBJ_DONT_REPARSE: u32 = 0x0000_1000;

    let mut name = child.encode_wide().collect::<Vec<_>>();
    if name.is_empty()
        || name.len() > (u16::MAX as usize / 2)
        || name
            .iter()
            .any(|unit| *unit == 0 || *unit == b'/' as u16 || *unit == b'\\' as u16)
        || child == std::ffi::OsStr::new(".")
        || child == std::ffi::OsStr::new("..")
    {
        return Err(unsafe_ref(
            "NtCreateFile child must be one exact nonempty unterminated component",
        ));
    }
    let name_bytes = u16::try_from(name.len() * 2)
        .map_err(|_| unsafe_ref("NtCreateFile child name exceeds the counted limit"))?;
    let mut unicode = UnicodeString {
        length: name_bytes,
        maximum_length: name_bytes,
        buffer: name.as_mut_ptr(),
    };
    let parent_handle = parent.as_raw_handle();
    if parent_handle.is_null() || parent_handle as isize == -1 {
        return Err(unsafe_ref("NtCreateFile retained parent handle is invalid"));
    }
    let mut attributes = ObjectAttributes {
        length: size_of::<ObjectAttributes>() as u32,
        root_directory: parent_handle,
        object_name: &mut unicode,
        attributes: OBJ_CASE_INSENSITIVE | OBJ_DONT_REPARSE,
        security_descriptor: ptr::null_mut(),
        security_quality_of_service: ptr::null_mut(),
    };
    let mut io_status = IoStatusBlock {
        value: IoStatusValue {
            status: STATUS_UNSUCCESSFUL,
        },
        information: usize::MAX,
    };
    let mut handle = ptr::null_mut();
    let desired_access = FILE_GENERIC_READ | if delete_access { DELETE } else { 0 };
    let create_options = FILE_OPEN_REPARSE_POINT
        | FILE_SYNCHRONOUS_IO_NONALERT
        | if directory {
            0
        } else {
            FILE_NON_DIRECTORY_FILE
        };
    // SAFETY: the retained parent remains live; `name` backs the exact counted,
    // unterminated UTF-16 component for the synchronous call; every pointer is either
    // null by contract or references a correctly sized initialized local, and the
    // returned handle is transferred to exactly one `File` on success.
    let (raw_status, final_status) = unsafe {
        let raw = NtCreateFile(
            &mut handle,
            desired_access,
            &mut attributes,
            &mut io_status,
            ptr::null_mut(),
            0,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            FILE_OPEN,
            create_options,
            ptr::null_mut(),
            0,
        );
        (raw, io_status.value.status)
    };
    if raw_status != STATUS_SUCCESS
        || final_status != STATUS_SUCCESS
        || io_status.information != FILE_OPENED
        || handle.is_null()
        || handle as isize == -1
    {
        if !handle.is_null() && handle as isize != -1 {
            // SAFETY: a non-null failure handle, if supplied, has not been transferred.
            unsafe {
                CloseHandle(handle);
            }
        }
        return Err(unsafe_ref(format!(
            "NtCreateFile retained child open failed (raw=0x{:08X}, final=0x{:08X}, information={})",
            raw_status as u32, final_status as u32, io_status.information
        )));
    }
    // SAFETY: successful `NtCreateFile` returned one owned kernel handle that is not
    // represented by any other Rust owner.
    Ok(unsafe { File::from_raw_handle(handle) })
}

#[cfg(windows)]
#[allow(
    unsafe_code,
    reason = "the frozen object-ID contract requires the raw synchronous NtFsControlFile ABI"
)]
fn query_object_id_absence_nt(file: &File) -> Result<(), GenericLineageStoreErrorV1> {
    use std::ffi::c_void;
    use std::os::windows::io::AsRawHandle;
    use std::ptr;

    #[repr(C)]
    union IoStatusValue {
        status: i32,
        pointer: *mut c_void,
    }

    #[repr(C)]
    struct IoStatusBlock {
        value: IoStatusValue,
        information: usize,
    }

    #[repr(C)]
    struct FileObjectIdBuffer {
        object_id: [u8; 16],
        extended_info: [u8; 48],
    }

    #[link(name = "ntdll")]
    unsafe extern "system" {
        fn NtFsControlFile(
            file_handle: *mut c_void,
            event: *mut c_void,
            apc_routine: *mut c_void,
            apc_context: *mut c_void,
            io_status_block: *mut IoStatusBlock,
            fs_control_code: u32,
            input_buffer: *mut c_void,
            input_buffer_length: u32,
            output_buffer: *mut c_void,
            output_buffer_length: u32,
        ) -> i32;
    }

    const STATUS_UNSUCCESSFUL: i32 = 0xc000_0001u32 as i32;
    const STATUS_OBJECTID_NOT_FOUND: i32 = 0xc000_02f0u32 as i32;
    const STATUS_PENDING: i32 = 0x0000_0103;
    const FSCTL_GET_OBJECT_ID: u32 = 0x0009_009c;

    let mut io_status = IoStatusBlock {
        value: IoStatusValue {
            status: STATUS_UNSUCCESSFUL,
        },
        information: usize::MAX,
    };
    let mut output = FileObjectIdBuffer {
        object_id: [0; 16],
        extended_info: [0; 48],
    };
    // SAFETY: `file` owns a live synchronous handle for the whole call. All optional
    // asynchronous/input pointers are null, while the initialized IO-status and exact
    // 64-byte output buffers are uniquely borrowed and correctly sized.
    let (raw_status, final_status) = unsafe {
        let raw = NtFsControlFile(
            file.as_raw_handle(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            &mut io_status,
            FSCTL_GET_OBJECT_ID,
            ptr::null_mut(),
            0,
            ptr::from_mut(&mut output).cast(),
            size_of::<FileObjectIdBuffer>() as u32,
        );
        (raw, io_status.value.status)
    };
    let immediate_absence = raw_status == STATUS_OBJECTID_NOT_FOUND
        && final_status == STATUS_UNSUCCESSFUL
        && io_status.information == usize::MAX;
    let completed_absence = matches!(raw_status, STATUS_OBJECTID_NOT_FOUND | STATUS_PENDING)
        && final_status == STATUS_OBJECTID_NOT_FOUND
        && io_status.information == 0;
    if !immediate_absence && !completed_absence {
        return Err(conflicting(
            format!(
                "NtFsControlFile did not prove exact object-ID absence (raw=0x{:08X}, final=0x{:08X}, information={})",
                raw_status as u32, final_status as u32, io_status.information
            ),
        ));
    }
    Ok(())
}

fn require_exact_fields(
    value: &Value,
    fields: &[&str],
    kind: GenericLineageStoreErrorKindV1,
) -> Result<(), GenericLineageStoreErrorV1> {
    let object = value
        .as_object()
        .ok_or_else(|| error(kind, "control record must have an object root"))?;
    if object.len() != fields.len() || fields.iter().any(|field| !object.contains_key(*field)) {
        return Err(error(
            kind,
            "control record does not have its exact closed field set",
        ));
    }
    Ok(())
}

fn finalize_control(
    mut value: Value,
    fingerprint_field: &str,
) -> Result<Value, GenericLineageStoreErrorV1> {
    let object = value
        .as_object_mut()
        .ok_or_else(|| invalid_plan("control record must have an object root"))?;
    if object.contains_key(fingerprint_field) {
        return Err(invalid_plan("control fingerprint field already exists"));
    }
    let fingerprint = sha256_prefixed(&canonical_json(&value)?);
    value
        .as_object_mut()
        .expect("object retained")
        .insert(fingerprint_field.to_owned(), Value::String(fingerprint));
    Ok(value)
}

fn read_control(path: &Path, fingerprint_field: &str) -> Result<Value, GenericLineageStoreErrorV1> {
    let bytes = read_bounded_regular(path, MAX_RECORD_BYTES)?;
    read_control_bytes(&bytes, fingerprint_field)
}

fn read_control_bytes(
    bytes: &[u8],
    fingerprint_field: &str,
) -> Result<Value, GenericLineageStoreErrorV1> {
    let value = parse_jcs_lf(bytes)?;
    let mut preimage = value.clone();
    let fingerprint = preimage
        .as_object_mut()
        .and_then(|object| object.remove(fingerprint_field))
        .and_then(|entry| entry.as_str().map(str::to_owned))
        .ok_or_else(|| retained_error("control record terminal fingerprint is missing"))?;
    validate_sha256(&fingerprint)?;
    if sha256_prefixed(&canonical_json(&preimage)?) != fingerprint {
        return Err(retained_error(
            "control record fingerprint does not recompute",
        ));
    }
    Ok(value)
}

fn parse_jcs_lf(bytes: &[u8]) -> Result<Value, GenericLineageStoreErrorV1> {
    if bytes.is_empty() || bytes.len() > MAX_RECORD_BYTES || bytes.last() != Some(&b'\n') {
        return Err(invalid_plan(
            "persisted JSON must be bounded JCS plus one LF",
        ));
    }
    let value: Value = serde_json::from_slice(&bytes[..bytes.len() - 1])
        .map_err(|_| invalid_plan("persisted JSON is malformed"))?;
    if jcs_lf(&value)? != bytes {
        return Err(invalid_plan("persisted JSON is not exact JCS plus one LF"));
    }
    Ok(value)
}

fn canonical_json(value: &Value) -> Result<Vec<u8>, GenericLineageStoreErrorV1> {
    serde_json_canonicalizer::to_vec(value)
        .map_err(|_| invalid_plan("RFC 8785 canonicalization failed"))
}

fn runtime_record_subject_fingerprint(
    value: &Value,
) -> Result<Option<String>, GenericLineageStoreErrorV1> {
    let Some(object) = value.as_object() else {
        return Ok(None);
    };
    let fields = match object.get("schema_id").and_then(Value::as_str) {
        Some("handbook.artifact-intake-record") => ("intake_record_id", "record_fingerprint"),
        Some("handbook.artifact-validation-result") => {
            ("validation_result_id", "validation_result_fingerprint")
        }
        Some("handbook.artifact-candidate") => ("candidate_id", "candidate_fingerprint"),
        Some("handbook.artifact-promotion-record") => ("promotion_id", "promotion_fingerprint"),
        _ => return Ok(None),
    };
    let mut subject = value.clone();
    let subject_object = subject
        .as_object_mut()
        .expect("cloned runtime record remains an object");
    subject_object.remove(fields.0);
    subject_object.remove(fields.1);
    Ok(Some(sha256_prefixed(&canonical_json(&subject)?)))
}

fn value_contains_string(value: &Value, expected: &str) -> bool {
    match value {
        Value::String(value) => value == expected,
        Value::Array(values) => values
            .iter()
            .any(|value| value_contains_string(value, expected)),
        Value::Object(values) => values
            .values()
            .any(|value| value_contains_string(value, expected)),
        _ => false,
    }
}

fn jcs_lf(value: &Value) -> Result<Vec<u8>, GenericLineageStoreErrorV1> {
    let mut bytes = canonical_json(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn validate_sha256(value: &str) -> Result<(), GenericLineageStoreErrorV1> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err(error(
            GenericLineageStoreErrorKindV1::InvalidRequest,
            "fingerprint is not lowercase SHA-256",
        ));
    };
    if hex.len() != 64
        || !hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(error(
            GenericLineageStoreErrorKindV1::InvalidRequest,
            "fingerprint is not lowercase SHA-256",
        ));
    }
    Ok(())
}

fn validate_nullable_sha256(value: Option<&Value>) -> Result<(), GenericLineageStoreErrorV1> {
    match value {
        Some(Value::Null) => Ok(()),
        Some(Value::String(value)) => validate_sha256(value),
        _ => Err(error(
            GenericLineageStoreErrorKindV1::InvalidRequest,
            "nullable fingerprint is neither null nor lowercase SHA-256",
        )),
    }
}

fn validate_instance_id(value: &str) -> Result<(), GenericLineageStoreErrorV1> {
    if value.is_empty()
        || value.len() > 64
        || !value.bytes().enumerate().all(|(index, byte)| {
            if index == 0 {
                byte.is_ascii_lowercase()
            } else {
                byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_'
            }
        })
    {
        return Err(error(
            GenericLineageStoreErrorKindV1::InvalidRequest,
            "instance ID is not a bounded symbolic identifier",
        ));
    }
    Ok(())
}

fn validate_exact_ref(value: &str) -> Result<(), GenericLineageStoreErrorV1> {
    if value.len() < 7 || value.len() > 512 || value.bytes().any(|byte| byte.is_ascii_whitespace())
    {
        return Err(error(
            GenericLineageStoreErrorKindV1::InvalidRequest,
            "exact definition ref is outside its grammar",
        ));
    }
    let Some((identity, version)) = value.split_once('@') else {
        return Err(error(
            GenericLineageStoreErrorKindV1::InvalidRequest,
            "exact definition ref is missing its single version separator",
        ));
    };
    if value.matches('@').count() != 1
        || identity.len() > 255
        || identity.split('.').count() < 2
        || identity.split('.').any(|segment| {
            segment.is_empty()
                || segment.len() > 63
                || !segment.as_bytes()[0].is_ascii_lowercase()
                || segment.starts_with('-')
                || segment.ends_with('-')
                || segment.contains("--")
                || !segment
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        })
        || semver::Version::parse(version)
            .ok()
            .is_none_or(|parsed| parsed.to_string() != version)
    {
        return Err(error(
            GenericLineageStoreErrorKindV1::InvalidRequest,
            "exact definition ref is outside its canonical identity grammar",
        ));
    }
    Ok(())
}

fn validate_safe_ref(value: &str) -> Result<(), GenericLineageStoreErrorV1> {
    if value.is_empty()
        || value.len() > 1024
        || value.starts_with('/')
        || value.ends_with('/')
        || value.contains("//")
        || value.contains('\\')
        || value.contains(':')
        || !value.is_ascii()
        || value.bytes().any(|byte| byte.is_ascii_whitespace())
    {
        return Err(unsafe_ref(
            "reference is not a normalized bounded relative path",
        ));
    }
    let parts = value.split('/').collect::<Vec<_>>();
    if parts.len() > 64
        || parts.iter().any(|part| {
            matches!(*part, "" | "." | "..")
                || !part
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || b"._-".contains(&byte))
        })
    {
        return Err(unsafe_ref("reference contains an unsafe path component"));
    }
    Ok(())
}

fn output_ref_matches_fingerprint(reference: &str, fingerprint: &str) -> bool {
    fingerprint_from_output_ref(reference).as_deref() == Ok(fingerprint)
}

fn fingerprint_from_output_ref(reference: &str) -> Result<String, GenericLineageStoreErrorV1> {
    let basename = reference
        .rsplit('/')
        .next()
        .and_then(|name| name.strip_suffix(".json"))
        .ok_or_else(|| invalid_intent("content-addressed output ref has an invalid basename"))?;
    let (_, hex) = basename
        .rsplit_once('_')
        .ok_or_else(|| invalid_intent("content-addressed output ref has no fingerprint"))?;
    let fingerprint = format!("sha256:{hex}");
    validate_sha256(&fingerprint)?;
    Ok(fingerprint)
}

fn fingerprint_hex(value: &str) -> Result<&str, GenericLineageStoreErrorV1> {
    validate_sha256(value)?;
    Ok(value.strip_prefix("sha256:").expect("validated prefix"))
}

fn sha256_prefixed(bytes: &[u8]) -> String {
    format!("sha256:{}", hex_digest(bytes))
}

fn hex_digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn string_field<'a>(value: &'a Value, field: &str) -> Result<&'a str, GenericLineageStoreErrorV1> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| retained_error("control record requires a string field"))
}

fn required_string<'a>(
    value: &'a Map<String, Value>,
    field: &str,
) -> Result<&'a str, GenericLineageStoreErrorV1> {
    value.get(field).and_then(Value::as_str).ok_or_else(|| {
        error(
            GenericLineageStoreErrorKindV1::InvalidRequest,
            format!("request field `{field}` must be a string"),
        )
    })
}

fn staged_name(ordinal: usize, token: &str) -> String {
    format!("{ordinal}-{token}.bin")
}

fn maybe_fault(
    selected: FaultSelection,
    boundary: FaultBoundary,
) -> Result<(), GenericLineageStoreErrorV1> {
    #[cfg(test)]
    if selected.is_some_and(|selected| match (selected, boundary) {
        (GenericLineageFaultPointV1::AfterEstablishedIntent, FaultBoundary::EstablishedIntent)
        | (GenericLineageFaultPointV1::AfterPendingIntent, FaultBoundary::PendingIntent)
        | (GenericLineageFaultPointV1::AfterVerified, FaultBoundary::Verified)
        | (
            GenericLineageFaultPointV1::AfterInstalledVerification,
            FaultBoundary::InstalledVerification,
        )
        | (GenericLineageFaultPointV1::AfterMarker, FaultBoundary::Marker)
        | (GenericLineageFaultPointV1::AfterEvidence, FaultBoundary::Evidence)
        | (GenericLineageFaultPointV1::AfterResult, FaultBoundary::Result)
        | (GenericLineageFaultPointV1::AfterLedger, FaultBoundary::Ledger)
        | (GenericLineageFaultPointV1::AfterCommittedRename, FaultBoundary::CommittedRename) => {
            true
        }
        (GenericLineageFaultPointV1::AfterInstall(left), FaultBoundary::Install(right)) => {
            left == right
        }
        (
            GenericLineageFaultPointV1::AfterCanonicalScratch(left),
            FaultBoundary::CanonicalScratch(right),
        ) => left == right,
        _ => false,
    }) {
        return Err(error(
            GenericLineageStoreErrorKindV1::InjectedFault,
            "test-private fault injected at a durability boundary",
        ));
    }
    #[cfg(not(test))]
    let _ = (selected, boundary);
    Ok(())
}

fn error(
    kind: GenericLineageStoreErrorKindV1,
    detail: impl Into<String>,
) -> GenericLineageStoreErrorV1 {
    GenericLineageStoreErrorV1::new(kind, detail)
}

fn invalid_plan(detail: impl Into<String>) -> GenericLineageStoreErrorV1 {
    error(GenericLineageStoreErrorKindV1::InvalidPlan, detail)
}

fn invalid_intent(detail: impl Into<String>) -> GenericLineageStoreErrorV1 {
    error(
        GenericLineageStoreErrorKindV1::InvalidTransactionIntent,
        detail,
    )
}

fn conflicting(detail: impl Into<String>) -> GenericLineageStoreErrorV1 {
    error(
        GenericLineageStoreErrorKindV1::ConflictingTransactionState,
        detail,
    )
}

fn refusal_conflict(detail: impl Into<String>) -> GenericLineageStoreErrorV1 {
    error(
        GenericLineageStoreErrorKindV1::ConflictingRefusalState,
        detail,
    )
}

fn retained_error(detail: impl Into<String>) -> GenericLineageStoreErrorV1 {
    error(
        GenericLineageStoreErrorKindV1::RetainedResultMismatch,
        detail,
    )
}

fn unsafe_ref(detail: impl Into<String>) -> GenericLineageStoreErrorV1 {
    error(GenericLineageStoreErrorKindV1::UnsafeReference, detail)
}

fn bound_error(detail: impl Into<String>) -> GenericLineageStoreErrorV1 {
    error(GenericLineageStoreErrorKindV1::BoundExceeded, detail)
}

fn io_error(detail: impl Into<String>) -> GenericLineageStoreErrorV1 {
    error(GenericLineageStoreErrorKindV1::IoFailure, detail)
}

struct GenericStoreLock {
    file: File,
    _directory_guards: Vec<File>,
}

impl GenericStoreLock {
    fn acquire(repo_root: &Path) -> Result<Self, GenericLineageStoreErrorV1> {
        let root = repo_root.join(".handbook/state/locks");
        create_safe_directories(repo_root, &root)?;
        let path = root.join("generic-artifact-operations.lock");
        #[cfg(windows)]
        let directory_guards = pin_store_parent_chain(&path)?;
        #[cfg(not(windows))]
        let directory_guards = Vec::new();
        let file = open_lock_file(&path)?;
        file.lock()
            .map_err(|_| io_error("generic artifact lock acquisition failed"))?;
        Ok(Self {
            file,
            _directory_guards: directory_guards,
        })
    }
}

impl Drop for GenericStoreLock {
    fn drop(&mut self) {
        let _ = self.file.unlock();
    }
}

fn ensure_supported_platform() -> Result<(), GenericLineageStoreErrorV1> {
    if cfg!(any(unix, windows)) {
        Ok(())
    } else {
        Err(error(
            GenericLineageStoreErrorKindV1::UnsupportedPlatform,
            "generic authority mutation requires native locks, no-follow opens, rename, and durable directory flush",
        ))
    }
}

fn create_safe_directories(
    trusted_root: &Path,
    target: &Path,
) -> Result<(), GenericLineageStoreErrorV1> {
    let relative = target
        .strip_prefix(trusted_root)
        .map_err(|_| unsafe_ref("state directory escapes retained repository root"))?;
    #[cfg(unix)]
    {
        use rustix::fs::{mkdirat, open, openat, Mode, OFlags};
        use rustix::io::Errno;

        let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::DIRECTORY | OFlags::NOFOLLOW;
        let mut current = open(trusted_root, flags, Mode::empty()).map_err(|_| {
            unsafe_ref("repository root could not be opened without following links")
        })?;
        for component in relative.components() {
            let Component::Normal(part) = component else {
                return Err(unsafe_ref("state directory is not lexically normalized"));
            };
            let created = match mkdirat(
                &current,
                Path::new(part),
                Mode::RUSR | Mode::WUSR | Mode::XUSR,
            ) {
                Ok(()) => true,
                Err(Errno::EXIST) => false,
                Err(_) => return Err(io_error("descriptor-relative directory creation failed")),
            };
            let next = openat(&current, Path::new(part), flags, Mode::empty()).map_err(|_| {
                unsafe_ref("state directory component failed no-follow descriptor admission")
            })?;
            if created {
                rustix::fs::fsync(&current)
                    .map_err(|_| io_error("directory parent durability flush failed"))?;
            }
            current = next;
        }
        Ok(())
    }

    #[cfg(not(unix))]
    {
        #[cfg(windows)]
        let mut guards = Vec::new();
        let mut current = trusted_root.to_path_buf();
        reject_link(&current, true)?;
        #[cfg(windows)]
        guards.push(open_pinned_directory(&current)?);
        for component in relative.components() {
            let Component::Normal(part) = component else {
                return Err(unsafe_ref("state directory is not lexically normalized"));
            };
            let parent = current.clone();
            current.push(part);
            match fs::create_dir(&current) {
                Ok(()) => sync_directory(&parent)?,
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(_) => return Err(io_error("safe directory creation failed")),
            }
            reject_link(&current, true)?;
            #[cfg(windows)]
            guards.push(open_pinned_directory(&current)?);
        }
        Ok(())
    }
}

#[cfg(windows)]
fn open_pinned_directory(path: &Path) -> Result<File, GenericLineageStoreErrorV1> {
    use std::os::windows::fs::OpenOptionsExt;
    const FILE_SHARE_READ: u32 = 0x0000_0001;
    const FILE_SHARE_WRITE: u32 = 0x0000_0002;
    const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    let file = OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
        .map_err(|_| unsafe_ref("directory component could not be pinned"))?;
    if !file
        .metadata()
        .map_err(|_| io_error("pinned directory metadata failed"))?
        .is_dir()
    {
        return Err(unsafe_ref("pinned path component is not a directory"));
    }
    Ok(file)
}

fn reject_link(path: &Path, require_directory: bool) -> Result<(), GenericLineageStoreErrorV1> {
    let metadata = fs::symlink_metadata(path).map_err(|_| {
        error(
            GenericLineageStoreErrorKindV1::UnsafeFilesystem,
            "filesystem metadata admission failed",
        )
    })?;
    if metadata.file_type().is_symlink() {
        return Err(error(
            GenericLineageStoreErrorKindV1::UnsafeFilesystem,
            "generic store path contains a symlink",
        ));
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
        if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Err(error(
                GenericLineageStoreErrorKindV1::UnsafeFilesystem,
                "generic store path contains a reparse point",
            ));
        }
    }
    if require_directory && !metadata.is_dir() {
        return Err(error(
            GenericLineageStoreErrorKindV1::UnsafeFilesystem,
            "generic store directory component is not a directory",
        ));
    }
    if !require_directory && !metadata.is_file() {
        return Err(error(
            GenericLineageStoreErrorKindV1::UnsafeFilesystem,
            "generic store file is not regular",
        ));
    }
    #[cfg(unix)]
    if !require_directory {
        use std::os::unix::fs::MetadataExt;
        if metadata.nlink() != 1 {
            return Err(error(
                GenericLineageStoreErrorKindV1::UnsafeFilesystem,
                "generic store file has a hard-link alias",
            ));
        }
    }
    #[cfg(windows)]
    if !require_directory && !crate::canonical_repo_support::path_has_single_link(path) {
        return Err(error(
            GenericLineageStoreErrorKindV1::UnsafeFilesystem,
            "generic store file has a hard-link alias or unreadable link identity",
        ));
    }
    Ok(())
}

fn create_new_file(path: &Path) -> Result<File, std::io::Error> {
    #[cfg(unix)]
    {
        use rustix::fs::{open, openat, Mode, OFlags};

        let (repo_root, relative) = store_relative_path(path)
            .map_err(|error| std::io::Error::other(error.detail().to_owned()))?;
        let relative_path = Path::new(&relative);
        let parent = relative_path
            .parent()
            .ok_or_else(|| std::io::Error::other("create-new path has no parent"))?;
        let name = relative_path
            .file_name()
            .ok_or_else(|| std::io::Error::other("create-new path has no name"))?;
        let directory_flags =
            OFlags::RDONLY | OFlags::CLOEXEC | OFlags::DIRECTORY | OFlags::NOFOLLOW;
        let mut directory =
            open(&repo_root, directory_flags, Mode::empty()).map_err(std::io::Error::from)?;
        for component in parent.components() {
            let Component::Normal(part) = component else {
                return Err(std::io::Error::other("create-new parent is not normalized"));
            };
            directory = openat(&directory, Path::new(part), directory_flags, Mode::empty())
                .map_err(std::io::Error::from)?;
        }
        let file = openat(
            &directory,
            Path::new(name),
            OFlags::WRONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::CREATE | OFlags::EXCL,
            Mode::RUSR | Mode::WUSR,
        )
        .map_err(std::io::Error::from)?;
        Ok(File::from(file))
    }

    #[cfg(not(unix))]
    {
        let mut options = OpenOptions::new();
        options.create_new(true).write(true);
        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt;
            const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
            const FILE_FLAG_WRITE_THROUGH: u32 = 0x8000_0000;
            options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_WRITE_THROUGH);
        }
        options.open(path)
    }
}

fn open_lock_file(path: &Path) -> Result<File, GenericLineageStoreErrorV1> {
    match create_new_file(path) {
        Ok(file) => return Ok(file),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(_) => return Err(io_error("generic lock file create failed")),
    }
    #[cfg(unix)]
    {
        use rustix::fs::{open, openat, Mode, OFlags};
        let (repo_root, relative) = store_relative_path(path)?;
        let relative = Path::new(&relative);
        let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::DIRECTORY | OFlags::NOFOLLOW;
        let mut directory = open(&repo_root, flags, Mode::empty())
            .map_err(|_| unsafe_ref("lock repository root open failed"))?;
        for component in relative.parent().unwrap_or(Path::new("")).components() {
            let Component::Normal(part) = component else {
                return Err(unsafe_ref("lock parent is not normalized"));
            };
            directory = openat(&directory, Path::new(part), flags, Mode::empty())
                .map_err(|_| unsafe_ref("lock parent component changed"))?;
        }
        let name = relative
            .file_name()
            .ok_or_else(|| unsafe_ref("lock path has no name"))?;
        let file = openat(
            &directory,
            Path::new(name),
            OFlags::RDWR | OFlags::CLOEXEC | OFlags::NOFOLLOW,
            Mode::empty(),
        )
        .map_err(|_| io_error("descriptor-relative generic lock open failed"))?;
        let file = File::from(file);
        if !file
            .metadata()
            .map_err(|_| io_error("generic lock metadata failed"))?
            .is_file()
        {
            return Err(unsafe_ref("generic lock is not a regular file"));
        }
        Ok(file)
    }
    #[cfg(not(unix))]
    {
        reject_link(path, false)?;
        let mut options = OpenOptions::new();
        options.read(true).write(true);
        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt;
            const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
            options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
        }
        let file = options
            .open(path)
            .map_err(|_| io_error("generic lock file open failed"))?;
        if !file
            .metadata()
            .map_err(|_| io_error("generic lock metadata failed"))?
            .is_file()
        {
            return Err(error(
                GenericLineageStoreErrorKindV1::UnsafeFilesystem,
                "generic lock is not a regular file",
            ));
        }
        Ok(file)
    }
}

fn read_bounded_regular(path: &Path, limit: usize) -> Result<Vec<u8>, GenericLineageStoreErrorV1> {
    let (repo_root, relative) = store_relative_path(path)?;
    let workspace = CanonicalWorkspace::new(&repo_root);
    let normalized = workspace
        .normalize_repo_relative(&relative)
        .map_err(unsafe_ref)?;
    let file = workspace
        .trusted_read_strict(&normalized)
        .map_err(|_| unsafe_ref("bounded read path failed strict component admission"))?;
    let (bytes, exceeded) = file
        .read_bytes_bounded_stable(limit)
        .map_err(|_| io_error("bounded read failed"))?;
    if exceeded {
        return Err(bound_error("generic store object exceeds its byte limit"));
    }
    Ok(bytes)
}

fn store_relative_path(path: &Path) -> Result<(PathBuf, String), GenericLineageStoreErrorV1> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|_| io_error("current directory is unavailable"))?
            .join(path)
    };
    let handbook_root = absolute
        .ancestors()
        .find(|ancestor| ancestor.file_name().is_some_and(|name| name == ".handbook"))
        .ok_or_else(|| unsafe_ref("generic store path is outside the .handbook authority root"))?;
    let repo_root = handbook_root
        .parent()
        .ok_or_else(|| unsafe_ref("generic store path has no repository parent"))?
        .to_path_buf();
    let relative = absolute
        .strip_prefix(&repo_root)
        .map_err(|_| unsafe_ref("generic store path escapes repository root"))?
        .components()
        .map(|component| match component {
            Component::Normal(value) => value
                .to_str()
                .map(str::to_owned)
                .ok_or_else(|| unsafe_ref("generic store path is not UTF-8")),
            _ => Err(unsafe_ref("generic store path is not normalized")),
        })
        .collect::<Result<Vec<_>, _>>()?
        .join("/");
    Ok((repo_root, relative))
}

fn write_new_durable(path: &Path, bytes: &[u8]) -> Result<(), GenericLineageStoreErrorV1> {
    #[cfg(windows)]
    let _directory_guards = pin_store_parent_chain(path)?;
    let mut file = create_new_file(path).map_err(|_| io_error("durable create-new failed"))?;
    file.write_all(bytes)
        .map_err(|_| io_error("durable write failed"))?;
    file.sync_all()
        .map_err(|_| io_error("durable file flush failed"))?;
    sync_directory(
        path.parent()
            .ok_or_else(|| unsafe_ref("file has no parent"))?,
    )
}

fn write_new_or_equal(path: &Path, bytes: &[u8]) -> Result<(), GenericLineageStoreErrorV1> {
    match path.try_exists() {
        Ok(false) => write_new_durable(path, bytes),
        Ok(true) => require_equal_file(path, bytes),
        Err(_) => Err(io_error("durable destination lookup failed")),
    }
}

fn require_equal_file(path: &Path, bytes: &[u8]) -> Result<(), GenericLineageStoreErrorV1> {
    if read_bounded_regular(path, MAX_RECORD_BYTES)? == bytes {
        Ok(())
    } else {
        Err(conflicting(
            "preexisting immutable destination has unequal bytes",
        ))
    }
}

#[cfg(test)]
fn replace_durable(path: &Path, bytes: &[u8]) -> Result<(), GenericLineageStoreErrorV1> {
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|_| io_error("locked ledger replacement open failed"))?;
    file.write_all(bytes)
        .map_err(|_| io_error("locked ledger replacement write failed"))?;
    file.sync_all()
        .map_err(|_| io_error("locked ledger replacement flush failed"))?;
    drop(file);
    sync_directory(
        path.parent()
            .ok_or_else(|| unsafe_ref("locked ledger replacement has no parent"))?,
    )?;
    if read_bounded_regular(path, MAX_RECORD_BYTES)? != bytes {
        return Err(io_error("locked ledger replacement reread disagrees"));
    }
    Ok(())
}

fn require_fresh_replacement_scratch_absent(
    repo_root: &Path,
    plan: &GenericCommitPlanV1,
) -> Result<(), GenericLineageStoreErrorV1> {
    for output in &plan.outputs {
        if output.install_mode != GenericInstallModeV1::ReplaceIfCurrent {
            continue;
        }
        let scratch = replacement_scratch_path(&repo_root.join(&output.final_ref))?;
        match fs::symlink_metadata(&scratch) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Ok(_) => {
                return Err(conflicting(
                    "fresh replacement scratch must be absent before intent establishment",
                ));
            }
            Err(_) => return Err(io_error("fresh replacement scratch lookup failed")),
        }
    }
    Ok(())
}

fn require_fresh_create_new_outputs_absent(
    repo_root: &Path,
    plan: &GenericCommitPlanV1,
) -> Result<(), GenericLineageStoreErrorV1> {
    for output in &plan.outputs {
        if output.install_mode != GenericInstallModeV1::CreateNew {
            continue;
        }
        let path = repo_root.join(&output.final_ref);
        match fs::symlink_metadata(&path) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Ok(_) => {
                return Err(conflicting(
                    "fresh immutable output must be absent before intent establishment",
                ));
            }
            Err(_) => return Err(io_error("fresh immutable output lookup failed")),
        }
    }
    Ok(())
}

fn observe_prepared_replacement(
    scratch: PathBuf,
    file: File,
    _directory_guards: Vec<File>,
    bytes: &[u8],
) -> Result<PreparedReplacement, GenericLineageStoreErrorV1> {
    let before = file
        .metadata()
        .map_err(|_| io_error("replacement scratch metadata failed"))?;
    let observed = read_retained_file_bounded(&file, MAX_RECORD_BYTES)?;
    let after = file
        .metadata()
        .map_err(|_| io_error("replacement scratch metadata failed"))?;
    if !before.is_file() || observed != bytes || !same_native_metadata(&before, &after) {
        return Err(conflicting(
            "replacement scratch changed during retained observation",
        ));
    }
    let (identity_token, version_token) = native_bound_tokens(&after, &scratch)?;
    Ok(PreparedReplacement {
        path: scratch,
        file,
        #[cfg(windows)]
        _directory_guards,
        identity_token,
        version_token,
    })
}

fn replacement_scratch_path(path: &Path) -> Result<PathBuf, GenericLineageStoreErrorV1> {
    let parent = path
        .parent()
        .ok_or_else(|| unsafe_ref("replacement has no parent"))?;
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| unsafe_ref("replacement basename is not UTF-8"))?;
    Ok(parent.join(format!(".{name}.generic-replace")))
}

#[cfg_attr(
    windows,
    allow(
        unsafe_code,
        reason = "Windows no-replace publication requires local NtSetInformationFile FFI"
    )
)]
fn publish_replacement(
    path: &Path,
    mut scratch: PreparedReplacement,
    bytes: &[u8],
    expected_basis_fingerprint: Option<&str>,
    expected_basis_version_token: Option<&str>,
) -> Result<InstalledOutputGuard, GenericLineageStoreErrorV1> {
    let parent = path
        .parent()
        .ok_or_else(|| unsafe_ref("replacement has no parent"))?;
    require_prepared_replacement_unchanged(&scratch, bytes)?;
    run_before_publication_rename_hook(&scratch.path);
    let final_basis = verify_compare_and_write_path(
        path,
        expected_basis_fingerprint,
        expected_basis_version_token,
    )?;
    require_prepared_replacement_unchanged(&scratch, bytes)?;
    drop(final_basis);

    #[cfg(windows)]
    {
        use std::ffi::c_void;
        use std::os::windows::io::AsRawHandle;
        use std::ptr;

        #[repr(C)]
        #[derive(Clone, Copy, Eq, PartialEq)]
        struct FileId128 {
            identifier: [u8; 16],
        }

        #[repr(C)]
        #[derive(Clone, Copy, Eq, PartialEq)]
        struct FileIdInfo {
            volume_serial_number: u64,
            file_id: FileId128,
        }

        #[repr(C)]
        struct FileRenameInformation {
            replace_if_exists: u8,
            root_directory: *mut c_void,
            file_name_length: u32,
            file_name: [u16; 1],
        }

        #[repr(C)]
        union IoStatusValue {
            status: i32,
            pointer: *mut c_void,
        }

        #[repr(C)]
        struct IoStatusBlock {
            value: IoStatusValue,
            information: usize,
        }

        #[link(name = "kernel32")]
        unsafe extern "system" {
            fn GetFileInformationByHandleEx(
                file: *mut c_void,
                class: i32,
                information: *mut c_void,
                information_size: u32,
            ) -> i32;
        }

        #[link(name = "ntdll")]
        unsafe extern "system" {
            fn NtSetInformationFile(
                file_handle: *mut c_void,
                io_status_block: *mut IoStatusBlock,
                file_information: *mut c_void,
                length: u32,
                file_information_class: i32,
            ) -> i32;
        }

        const STATUS_UNSUCCESSFUL: i32 = 0xc000_0001u32 as i32;
        const STATUS_SUCCESS: i32 = 0;
        const STATUS_OBJECT_NAME_COLLISION: i32 = 0xc000_0035u32 as i32;
        const FILE_RENAME_INFORMATION_CLASS: i32 = 10;
        const FILE_ID_INFO_CLASS: i32 = 18;

        let (repo_root, relative) = store_relative_path(path)?;
        let relative = Path::new(&relative);
        let mut retained_ancestors = vec![open_pinned_directory(&repo_root)?];
        for component in relative.parent().unwrap_or(Path::new("")).components() {
            let Component::Normal(child) = component else {
                return Err(unsafe_ref("Windows publication parent is not normalized"));
            };
            let next = open_retained_parent_relative_nt(
                retained_ancestors
                    .last()
                    .expect("root ancestor is retained"),
                child,
                false,
                true,
            )?;
            let metadata = next
                .metadata()
                .map_err(|_| io_error("Windows retained parent metadata failed"))?;
            use std::os::windows::fs::MetadataExt;
            if !metadata.is_dir() || metadata.file_attributes() & 0x0000_0400 != 0 {
                return Err(unsafe_ref(
                    "Windows retained publication parent is not a plain directory",
                ));
            }
            retained_ancestors.push(next);
        }
        let parent_handle = retained_ancestors
            .pop()
            .expect("publication parent chain includes the parent");
        let parent_raw = parent_handle.as_raw_handle();
        let retained_id = |file: &File| -> Result<FileIdInfo, GenericLineageStoreErrorV1> {
            let mut id = FileIdInfo {
                volume_serial_number: 0,
                file_id: FileId128 {
                    identifier: [0; 16],
                },
            };
            // SAFETY: `file` owns the handle for the call and `id` is one initialized,
            // correctly sized `FILE_ID_INFO` output buffer.
            if unsafe {
                GetFileInformationByHandleEx(
                    file.as_raw_handle(),
                    FILE_ID_INFO_CLASS,
                    ptr::from_mut(&mut id).cast(),
                    size_of::<FileIdInfo>() as u32,
                )
            } == 0
                || id.file_id.identifier == [0; 16]
                || id.file_id.identifier == [u8::MAX; 16]
            {
                return Err(conflicting(
                    "Windows retained publication file ID is unavailable",
                ));
            }
            Ok(id)
        };
        let rename_relative = |source: &File,
                               child: &std::ffi::OsStr|
         -> Result<(), GenericLineageStoreErrorV1> {
            use std::os::windows::ffi::OsStrExt;
            let name = child.encode_wide().collect::<Vec<_>>();
            if name.is_empty()
                || name.len() > (u32::MAX as usize / 2)
                || name
                    .iter()
                    .any(|unit| *unit == 0 || *unit == b'/' as u16 || *unit == b'\\' as u16)
            {
                return Err(unsafe_ref(
                    "Windows publication destination is not one simple child",
                ));
            }
            let name_bytes = name
                .len()
                .checked_mul(2)
                .ok_or_else(|| unsafe_ref("Windows publication name length overflow"))?;
            let information_size = std::mem::offset_of!(FileRenameInformation, file_name)
                .checked_add(name_bytes)
                .ok_or_else(|| unsafe_ref("Windows publication buffer size overflow"))?;
            let information_size = u32::try_from(information_size)
                .map_err(|_| unsafe_ref("Windows publication buffer exceeds ULONG"))?;
            let mut information =
                vec![0usize; (information_size as usize).div_ceil(size_of::<usize>())];
            let rename = information.as_mut_ptr().cast::<FileRenameInformation>();
            let mut io_status = IoStatusBlock {
                value: IoStatusValue {
                    status: STATUS_UNSUCCESSFUL,
                },
                information: usize::MAX,
            };
            // SAFETY: `information` is large enough for the pointer-size-correct
            // `FILE_RENAME_INFORMATION` header and exact counted UTF-16 child. The
            // I/O status block is initialized and correctly sized, the parent and source
            // handles remain live, ReplaceIfExists stays false (zero), and the
            // synchronous native call retains no pointer after returning.
            let (raw_status, final_status) = unsafe {
                (*rename).replace_if_exists = 0;
                (*rename).root_directory = parent_raw;
                (*rename).file_name_length = name_bytes as u32;
                ptr::copy_nonoverlapping(
                    name.as_ptr(),
                    (*rename).file_name.as_mut_ptr(),
                    name.len(),
                );
                let raw = NtSetInformationFile(
                    source.as_raw_handle(),
                    &mut io_status,
                    rename.cast(),
                    information_size,
                    FILE_RENAME_INFORMATION_CLASS,
                );
                (raw, io_status.value.status)
            };
            if raw_status != STATUS_SUCCESS || final_status != STATUS_SUCCESS {
                if raw_status == STATUS_OBJECT_NAME_COLLISION
                    || final_status == STATUS_OBJECT_NAME_COLLISION
                {
                    return Err(error(
                        GenericLineageStoreErrorKindV1::BasisMismatch,
                        "Windows no-replace publication destination already exists",
                    ));
                }
                return Err(io_error(format!(
                        "Windows retained-handle publication failed (raw=0x{:08X}, final=0x{:08X}, information={})",
                        raw_status as u32, final_status as u32, io_status.information
                    )));
            }
            Ok(())
        };
        let open_source = |source_path: &Path| -> Result<File, GenericLineageStoreErrorV1> {
            let child = source_path
                .file_name()
                .ok_or_else(|| unsafe_ref("Windows publication source has no child name"))?;
            open_retained_parent_relative_nt(&parent_handle, child, true, false)
        };
        let prove_rebound = |source: &File,
                             destination_path: &Path|
         -> Result<File, GenericLineageStoreErrorV1> {
            let destination_name = destination_path
                .file_name()
                .ok_or_else(|| unsafe_ref("Windows publication target has no child name"))?;
            let destination =
                open_retained_parent_relative_nt(&parent_handle, destination_name, false, false)?;
            if retained_id(source)? != retained_id(&destination)? {
                return Err(conflicting(
                    "Windows destination rebound is not the retained live source object",
                ));
            }
            Ok(destination)
        };

        if expected_basis_fingerprint.is_some() {
            let displaced_name = scratch
                .path
                .file_name()
                .and_then(|name| name.to_str())
                .and_then(|name| {
                    name.strip_suffix(".candidate")
                        .map(|prefix| format!("{prefix}.displaced"))
                })
                .ok_or_else(|| unsafe_ref("publication candidate suffix is invalid"))?;
            let displaced = parent.join(&displaced_name);
            if displaced
                .try_exists()
                .map_err(|_| io_error("displaced publication lookup failed"))?
            {
                return Err(conflicting(
                    "fresh displaced publication evidence already exists",
                ));
            }
            let basis_source = open_source(path)?;
            rename_relative(
                &basis_source,
                displaced
                    .file_name()
                    .ok_or_else(|| unsafe_ref("displaced publication path has no child name"))?,
            )?;
            let _displaced_rebound = prove_rebound(&basis_source, &displaced)?;
        }

        let replacement_source = open_source(&scratch.path)?;
        if retained_id(&replacement_source)? != retained_id(&scratch.file)? {
            return Err(conflicting(
                "Windows DELETE-capable source is not the retained candidate object",
            ));
        }
        rename_relative(
            &replacement_source,
            path.file_name()
                .ok_or_else(|| unsafe_ref("canonical publication path has no child name"))?,
        )?;
        let canonical_rebound = prove_rebound(&replacement_source, path)?;
        scratch.file = canonical_rebound;
        scratch._directory_guards.extend(retained_ancestors);
        scratch._directory_guards.push(parent_handle);
    }
    #[cfg(unix)]
    {
        rename_store_path(&scratch.path, path)?;
        sync_directory(parent)?;
    }
    scratch.path = path.to_path_buf();
    let before = scratch
        .file
        .metadata()
        .map_err(|_| io_error("published replacement metadata failed"))?;
    let observed = read_retained_file_bounded(&scratch.file, MAX_RECORD_BYTES)?;
    let after = scratch
        .file
        .metadata()
        .map_err(|_| io_error("published replacement metadata failed"))?;
    let (identity_token, version_token) = native_bound_tokens(&after, path)?;
    if observed != bytes
        || !same_native_metadata(&before, &after)
        || identity_token != scratch.identity_token
    {
        return Err(conflicting(
            "published replacement is not the retained scratch identity and bytes",
        ));
    }
    Ok(InstalledOutputGuard {
        path: path.to_path_buf(),
        expected_bytes: bytes.to_vec(),
        expected_identity_token: Some(identity_token),
        expected_version_token: Some(version_token),
        _guard: None,
        replacement_guard: Some(scratch),
    })
}

fn create_new_replacement_guard(
    path: &Path,
) -> Result<(File, Vec<File>), GenericLineageStoreErrorV1> {
    #[cfg(unix)]
    {
        use rustix::fs::{open, openat, Mode, OFlags};
        let (repo_root, relative) = store_relative_path(path)?;
        let relative = Path::new(&relative);
        let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::DIRECTORY | OFlags::NOFOLLOW;
        let mut directory = open(&repo_root, flags, Mode::empty())
            .map_err(|_| unsafe_ref("replacement root open failed"))?;
        for component in relative.parent().unwrap_or(Path::new("")).components() {
            let Component::Normal(part) = component else {
                return Err(unsafe_ref("replacement parent is not normalized"));
            };
            directory = openat(&directory, Path::new(part), flags, Mode::empty())
                .map_err(|_| unsafe_ref("replacement parent component changed"))?;
        }
        let name = relative
            .file_name()
            .ok_or_else(|| unsafe_ref("replacement scratch has no name"))?;
        let file = openat(
            &directory,
            Path::new(name),
            OFlags::RDWR | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::CREATE | OFlags::EXCL,
            Mode::RUSR | Mode::WUSR,
        )
        .map_err(|error| {
            if error == rustix::io::Errno::EXIST {
                conflicting("fresh replacement scratch already exists")
            } else {
                unsafe_ref("replacement scratch create-new failed")
            }
        })?;
        return Ok((File::from(file), Vec::new()));
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        const FILE_SHARE_READ: u32 = 0x0000_0001;
        const FILE_SHARE_DELETE: u32 = 0x0000_0004;
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        const FILE_FLAG_WRITE_THROUGH: u32 = 0x8000_0000;
        let guards = pin_store_parent_chain(path)?;
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_DELETE)
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_WRITE_THROUGH)
            .open(path)
            .map_err(|error| {
                if error.kind() == std::io::ErrorKind::AlreadyExists {
                    conflicting("fresh replacement scratch already exists")
                } else {
                    io_error("replacement scratch create-new failed")
                }
            })?;
        return Ok((file, guards));
    }
    #[allow(unreachable_code)]
    Err(error(
        GenericLineageStoreErrorKindV1::UnsupportedPlatform,
        "replacement create-new requires Unix or Windows",
    ))
}

fn open_replacement_guard(path: &Path) -> Result<(File, Vec<File>), GenericLineageStoreErrorV1> {
    #[cfg(unix)]
    {
        use rustix::fs::{open, openat, Mode, OFlags};
        let (repo_root, relative) = store_relative_path(path)?;
        let relative = Path::new(&relative);
        let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::DIRECTORY | OFlags::NOFOLLOW;
        let mut directory = open(&repo_root, flags, Mode::empty())
            .map_err(|_| unsafe_ref("replacement root open failed"))?;
        for component in relative.parent().unwrap_or(Path::new("")).components() {
            let Component::Normal(part) = component else {
                return Err(unsafe_ref("replacement parent is not normalized"));
            };
            directory = openat(&directory, Path::new(part), flags, Mode::empty())
                .map_err(|_| unsafe_ref("replacement parent component changed"))?;
        }
        let name = relative
            .file_name()
            .ok_or_else(|| unsafe_ref("replacement scratch has no name"))?;
        let file = openat(
            &directory,
            Path::new(name),
            OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW,
            Mode::empty(),
        )
        .map_err(|_| unsafe_ref("replacement scratch no-follow open failed"))?;
        return Ok((File::from(file), Vec::new()));
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        const FILE_SHARE_READ: u32 = 0x0000_0001;
        const FILE_SHARE_DELETE: u32 = 0x0000_0004;
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        let guards = pin_store_parent_chain(path)?;
        let file = OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_DELETE)
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
            .open(path)
            .map_err(|_| unsafe_ref("replacement scratch pinned open failed"))?;
        return Ok((file, guards));
    }
    #[allow(unreachable_code)]
    Err(error(
        GenericLineageStoreErrorKindV1::UnsupportedPlatform,
        "replacement guard requires Unix or Windows",
    ))
}

fn require_prepared_replacement_unchanged(
    replacement: &PreparedReplacement,
    bytes: &[u8],
) -> Result<(), GenericLineageStoreErrorV1> {
    let before = replacement
        .file
        .metadata()
        .map_err(|_| io_error("replacement retained metadata failed"))?;
    let observed = read_retained_file_bounded(&replacement.file, MAX_RECORD_BYTES)?;
    let after = replacement
        .file
        .metadata()
        .map_err(|_| io_error("replacement retained metadata failed"))?;
    if !same_native_metadata(&before, &after) {
        return Err(conflicting(
            "replacement scratch changed during retained reobservation",
        ));
    }
    let (identity_token, version_token) = native_bound_tokens(&after, &replacement.path)?;
    if identity_token != replacement.identity_token
        || version_token != replacement.version_token
        || observed != bytes
    {
        return Err(conflicting(
            "replacement scratch no longer has its retained native identity and bytes",
        ));
    }
    Ok(())
}

fn read_retained_file_bounded(
    file: &File,
    limit: usize,
) -> Result<Vec<u8>, GenericLineageStoreErrorV1> {
    let mut retained = file;
    retained
        .seek(SeekFrom::Start(0))
        .map_err(|_| io_error("replacement retained seek failed"))?;
    let mut observed = Vec::new();
    retained
        .take((limit + 1) as u64)
        .read_to_end(&mut observed)
        .map_err(|_| io_error("replacement retained read failed"))?;
    if observed.len() > limit {
        return Err(bound_error("replacement scratch exceeds its byte limit"));
    }
    Ok(observed)
}

fn observe_installed_output(
    path: &Path,
    bytes: &[u8],
    expected_identity_token: Option<&str>,
) -> Result<InstalledOutputGuard, GenericLineageStoreErrorV1> {
    let (guard, observed, identity_token, version_token) = if expected_identity_token.is_some() {
        let (guard, observed, identity, version) =
            observe_retained_regular_file(path, MAX_RECORD_BYTES)?;
        (guard, observed, Some(identity), Some(version))
    } else {
        let (guard, observed) = observe_retained_bytes(path, MAX_RECORD_BYTES)?;
        (guard, observed, None, None)
    };
    if observed != bytes
        || expected_identity_token
            .is_some_and(|expected| identity_token.as_deref() != Some(expected))
    {
        return Err(conflicting(
            "installed output does not have its expected bytes and publication identity",
        ));
    }
    Ok(InstalledOutputGuard {
        path: path.to_path_buf(),
        expected_bytes: bytes.to_vec(),
        expected_identity_token: expected_identity_token.map(str::to_owned),
        expected_version_token: version_token,
        _guard: Some(guard),
        replacement_guard: None,
    })
}

fn require_installed_guards_unchanged(
    guards: &[InstalledOutputGuard],
) -> Result<(), GenericLineageStoreErrorV1> {
    for guard in guards {
        let (_retained, bytes, identity_token, version_token) =
            if let Some(replacement) = &guard.replacement_guard {
                let before = replacement
                    .file
                    .metadata()
                    .map_err(|_| io_error("installed replacement metadata failed"))?;
                let bytes = read_retained_file_bounded(&replacement.file, MAX_RECORD_BYTES)?;
                let after = replacement
                    .file
                    .metadata()
                    .map_err(|_| io_error("installed replacement metadata failed"))?;
                if !same_native_metadata(&before, &after) {
                    return Err(conflicting(
                        "installed replacement changed during retained reobservation",
                    ));
                }
                let (identity, version) = native_bound_tokens(&after, &guard.path)?;
                (None, bytes, Some(identity), Some(version))
            } else if guard.expected_identity_token.is_some() {
                let (retained, bytes, identity, version) =
                    observe_retained_regular_file(&guard.path, MAX_RECORD_BYTES)?;
                (Some(retained), bytes, Some(identity), Some(version))
            } else {
                let (retained, bytes) = observe_retained_bytes(&guard.path, MAX_RECORD_BYTES)?;
                (Some(retained), bytes, None, None)
            };
        if bytes != guard.expected_bytes
            || guard
                .expected_identity_token
                .as_ref()
                .is_some_and(|expected| identity_token.as_ref() != Some(expected))
            || version_token != guard.expected_version_token
        {
            return Err(conflicting(
                "installed output changed before commit-marker durability",
            ));
        }
    }
    Ok(())
}

fn same_native_metadata(before: &fs::Metadata, after: &fs::Metadata) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        return before.dev() == after.dev()
            && before.ino() == after.ino()
            && before.nlink() == after.nlink()
            && before.len() == after.len()
            && before.mtime() == after.mtime()
            && before.mtime_nsec() == after.mtime_nsec()
            && before.ctime() == after.ctime()
            && before.ctime_nsec() == after.ctime_nsec();
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        return before.file_size() == after.file_size()
            && before.creation_time() == after.creation_time()
            && before.last_write_time() == after.last_write_time()
            && before.file_attributes() == after.file_attributes();
    }
    #[allow(unreachable_code)]
    false
}

fn create_store_directory(path: &Path) -> Result<(), GenericLineageStoreErrorV1> {
    let parent = path
        .parent()
        .ok_or_else(|| unsafe_ref("store directory has no parent"))?;
    let (repo_root, _) = store_relative_path(path)?;
    create_safe_directories(&repo_root, parent)?;
    #[cfg(unix)]
    {
        use rustix::fs::{mkdirat, open, openat, Mode, OFlags};

        let (_, relative) = store_relative_path(path)?;
        let relative = Path::new(&relative);
        let name = relative
            .file_name()
            .ok_or_else(|| unsafe_ref("store directory has no name"))?;
        let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::DIRECTORY | OFlags::NOFOLLOW;
        let mut directory = open(&repo_root, flags, Mode::empty())
            .map_err(|_| unsafe_ref("repository root directory open failed"))?;
        for component in relative.parent().unwrap_or(Path::new("")).components() {
            let Component::Normal(part) = component else {
                return Err(unsafe_ref("store directory parent is not normalized"));
            };
            directory = openat(&directory, Path::new(part), flags, Mode::empty())
                .map_err(|_| unsafe_ref("store directory parent component changed"))?;
        }
        mkdirat(
            &directory,
            Path::new(name),
            Mode::RUSR | Mode::WUSR | Mode::XUSR,
        )
        .map_err(|_| io_error("descriptor-relative store directory create failed"))?;
        rustix::fs::fsync(&directory)
            .map_err(|_| io_error("store directory parent flush failed"))?;
        Ok(())
    }
    #[cfg(windows)]
    {
        let _guards = pin_store_parent_chain(path)?;
        fs::create_dir(path).map_err(|_| io_error("pinned store directory create failed"))?;
        sync_directory(parent)
    }
    #[cfg(all(not(unix), not(windows)))]
    Err(error(
        GenericLineageStoreErrorKindV1::UnsupportedPlatform,
        "store directory creation requires Unix or Windows",
    ))
}

fn rename_store_path(source: &Path, target: &Path) -> Result<(), GenericLineageStoreErrorV1> {
    let (source_root, _source_relative) = store_relative_path(source)?;
    let (target_root, _target_relative) = store_relative_path(target)?;
    if source_root != target_root {
        return Err(unsafe_ref("store rename crosses repository roots"));
    }
    #[cfg(unix)]
    {
        use rustix::fs::{open, openat, renameat, Mode, OFlags};

        fn parent_and_name(relative: &str) -> Result<(&Path, &Path), GenericLineageStoreErrorV1> {
            let path = Path::new(relative);
            Ok((
                path.parent().unwrap_or(Path::new("")),
                Path::new(
                    path.file_name()
                        .ok_or_else(|| unsafe_ref("rename path has no name"))?,
                ),
            ))
        }
        fn open_parent(
            root: &Path,
            parent: &Path,
        ) -> Result<rustix::fd::OwnedFd, GenericLineageStoreErrorV1> {
            let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::DIRECTORY | OFlags::NOFOLLOW;
            let mut directory = open(root, flags, Mode::empty())
                .map_err(|_| unsafe_ref("rename repository root open failed"))?;
            for component in parent.components() {
                let Component::Normal(part) = component else {
                    return Err(unsafe_ref("rename parent is not normalized"));
                };
                directory = openat(&directory, Path::new(part), flags, Mode::empty())
                    .map_err(|_| unsafe_ref("rename parent component changed"))?;
            }
            Ok(directory)
        }
        let (source_parent, source_name) = parent_and_name(&_source_relative)?;
        let (target_parent, target_name) = parent_and_name(&_target_relative)?;
        let source_directory = open_parent(&source_root, source_parent)?;
        let target_directory = open_parent(&target_root, target_parent)?;
        renameat(
            &source_directory,
            source_name,
            &target_directory,
            target_name,
        )
        .map_err(|_| io_error("descriptor-relative store rename failed"))?;
        rustix::fs::fsync(&source_directory)
            .map_err(|_| io_error("rename source parent flush failed"))?;
        rustix::fs::fsync(&target_directory)
            .map_err(|_| io_error("rename target parent flush failed"))?;
        Ok(())
    }
    #[cfg(windows)]
    {
        let _source_guards = pin_store_parent_chain(source)?;
        let _target_guards = pin_store_parent_chain(target)?;
        fs::rename(source, target).map_err(|_| io_error("pinned store rename failed"))?;
        Ok(())
    }
    #[cfg(all(not(unix), not(windows)))]
    Err(error(
        GenericLineageStoreErrorKindV1::UnsupportedPlatform,
        "store rename requires Unix or Windows",
    ))
}

fn remove_store_file(path: &Path) -> Result<(), GenericLineageStoreErrorV1> {
    #[cfg(unix)]
    {
        use rustix::fs::{open, openat, unlinkat, AtFlags, Mode, OFlags};
        let (repo_root, relative) = store_relative_path(path)?;
        let relative = Path::new(&relative);
        let flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::DIRECTORY | OFlags::NOFOLLOW;
        let mut directory = open(&repo_root, flags, Mode::empty())
            .map_err(|_| unsafe_ref("remove repository root open failed"))?;
        for component in relative.parent().unwrap_or(Path::new("")).components() {
            let Component::Normal(part) = component else {
                return Err(unsafe_ref("remove parent is not normalized"));
            };
            directory = openat(&directory, Path::new(part), flags, Mode::empty())
                .map_err(|_| unsafe_ref("remove parent component changed"))?;
        }
        let name = relative
            .file_name()
            .ok_or_else(|| unsafe_ref("remove path has no name"))?;
        unlinkat(&directory, Path::new(name), AtFlags::empty())
            .map_err(|_| io_error("descriptor-relative scratch removal failed"))?;
        rustix::fs::fsync(&directory).map_err(|_| io_error("scratch parent flush failed"))?;
        Ok(())
    }
    #[cfg(windows)]
    {
        let _guards = pin_store_parent_chain(path)?;
        fs::remove_file(path).map_err(|_| io_error("pinned scratch removal failed"))
    }
    #[cfg(all(not(unix), not(windows)))]
    Err(error(
        GenericLineageStoreErrorKindV1::UnsupportedPlatform,
        "store removal requires Unix or Windows",
    ))
}

#[cfg(windows)]
fn pin_store_parent_chain(path: &Path) -> Result<Vec<File>, GenericLineageStoreErrorV1> {
    let (repo_root, relative) = store_relative_path(path)?;
    let mut guards = vec![open_pinned_directory(&repo_root)?];
    let mut current = repo_root;
    let parent = Path::new(&relative).parent().unwrap_or(Path::new(""));
    for component in parent.components() {
        let Component::Normal(part) = component else {
            return Err(unsafe_ref("pinned parent is not normalized"));
        };
        current.push(part);
        guards.push(open_pinned_directory(&current)?);
    }
    Ok(guards)
}

fn sync_directory(path: &Path) -> Result<(), GenericLineageStoreErrorV1> {
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        let directory = OpenOptions::new()
            .write(true)
            .share_mode(0x0000_0001 | 0x0000_0002 | 0x0000_0004)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
            .open(path)
            .map_err(|_| io_error("directory durable-open failed"))?;
        directory
            .sync_all()
            .map_err(|_| io_error("directory durable flush failed"))?;
        return Ok(());
    }
    #[cfg(unix)]
    {
        let directory = File::open(path).map_err(|_| io_error("directory open failed"))?;
        directory
            .sync_all()
            .map_err(|_| io_error("directory fsync failed"))?;
        return Ok(());
    }
    #[allow(unreachable_code)]
    Err(error(
        GenericLineageStoreErrorKindV1::UnsupportedPlatform,
        "directory durability is unavailable",
    ))
}

fn validate_establishing_inventory(root: &Path) -> Result<(), GenericLineageStoreErrorV1> {
    if !root
        .try_exists()
        .map_err(|_| io_error("establishing inventory failed"))?
    {
        return Ok(());
    }
    reject_link(root, true)?;
    let mut bases = BTreeSet::new();
    for path in read_dir_paths(root)? {
        reject_link(&path, false)?;
        let name = file_name(&path)?;
        let base = name
            .strip_suffix(".intent.writing")
            .or_else(|| name.strip_suffix(".intent"))
            .ok_or_else(|| conflicting("unknown establishing inventory entry"))?;
        if base.len() != 64
            || !base
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(conflicting("establishing path does not carry a key hash"));
        }
        if !bases.insert(base.to_owned()) {
            return Err(conflicting("multiple establishing files exist for one key"));
        }
    }
    Ok(())
}

fn validate_transaction_inventory(path: &Path) -> Result<u64, GenericLineageStoreErrorV1> {
    let mut files = 0usize;
    let mut bytes = 0u64;
    let intent_path = path.join("intent.json");
    let expected_staged = if intent_path
        .try_exists()
        .map_err(|_| io_error("transaction intent inventory lookup failed"))?
    {
        let intent = read_control(&intent_path, "intent_fingerprint")?;
        validate_intent(&intent)?;
        let directory_name = file_name(path)?;
        let directory_transaction = directory_name
            .strip_suffix(".pending")
            .or_else(|| directory_name.strip_suffix(".committed"))
            .ok_or_else(|| conflicting("transaction directory suffix is invalid"))?;
        if string_field(&intent, "transaction_id")? != directory_transaction {
            return Err(conflicting(
                "transaction directory does not bind its exact intent identity",
            ));
        }
        intent_outputs(&intent)?
            .iter()
            .enumerate()
            .map(|(ordinal, descriptor)| {
                Ok(staged_name(ordinal, string_field(descriptor, "token")?))
            })
            .collect::<Result<BTreeSet<_>, GenericLineageStoreErrorV1>>()?
    } else {
        BTreeSet::new()
    };
    let mut stack = vec![(path.to_path_buf(), 0usize)];
    while let Some((directory, depth)) = stack.pop() {
        if depth > MAX_DIRECTORY_DEPTH {
            return Err(bound_error("transaction inventory exceeds depth limit"));
        }
        let mut entries = read_dir_paths(&directory)?;
        entries.sort();
        for entry in entries {
            let metadata = fs::symlink_metadata(&entry)
                .map_err(|_| io_error("transaction inventory metadata failed"))?;
            if metadata.file_type().is_symlink() {
                return Err(error(
                    GenericLineageStoreErrorKindV1::UnsafeFilesystem,
                    "transaction inventory contains a symlink",
                ));
            }
            let name = file_name(&entry)?;
            if metadata.is_dir() {
                if depth != 0 || name != "staged" {
                    return Err(conflicting("transaction contains an unknown directory"));
                }
                stack.push((entry, depth + 1));
            } else if metadata.is_file() {
                reject_link(&entry, false)?;
                files += 1;
                bytes = bytes.saturating_add(metadata.len());
                if depth == 0
                    && ![
                        "intent.json",
                        "expected-basis.backup",
                        "native-publication.json",
                        "native-publication-result.json",
                        "verified.json",
                        "commit-marker.json",
                        "evidence.json",
                    ]
                    .contains(&name.as_str())
                {
                    return Err(conflicting(
                        "transaction contains an unknown top-level file",
                    ));
                }
                if depth == 1 && !expected_staged.contains(&name) {
                    return Err(conflicting(
                        "transaction contains an invalid staged filename",
                    ));
                }
            } else {
                return Err(error(
                    GenericLineageStoreErrorKindV1::UnsafeFilesystem,
                    "transaction inventory contains a non-regular entry",
                ));
            }
        }
    }
    if files > MAX_TRANSACTION_FILES || bytes > MAX_TRANSACTION_BYTES {
        return Err(bound_error(
            "transaction inventory exceeds count or byte limits",
        ));
    }
    if !intent_path.exists() && files != 0 {
        return Err(conflicting(
            "intent-free pending shell must be exactly empty",
        ));
    }
    Ok(bytes)
}

fn validate_idempotency_inventory(root: &Path) -> Result<(), GenericLineageStoreErrorV1> {
    if !root
        .try_exists()
        .map_err(|_| io_error("store inventory lookup failed"))?
    {
        return Ok(());
    }
    reject_link(root, true)?;
    let mut entries = read_dir_paths(root)?;
    entries.sort();
    for entry in entries {
        reject_link(&entry, true)?;
        let name = file_name(&entry)?;
        if !["establishing", "results", "ledger"].contains(&name.as_str()) {
            return Err(conflicting("unknown generic idempotency inventory family"));
        }
        if name == "establishing" {
            continue;
        }
        let mut files = read_dir_paths(&entry)?;
        files.sort();
        for file in files {
            reject_link(&file, false)?;
            let metadata = fs::symlink_metadata(&file)
                .map_err(|_| io_error("idempotency inventory metadata failed"))?;
            if !metadata.is_file() || metadata.len() > MAX_RECORD_BYTES as u64 {
                return Err(bound_error("idempotency record is unsafe or oversized"));
            }
            let filename = file_name(&file)?;
            let valid = if name == "ledger" {
                filename
                    .strip_suffix(".json")
                    .is_some_and(valid_lower_hex_64)
            } else {
                filename.strip_suffix(".json").is_some_and(|identity| {
                    [
                        GenericArtifactOperationV1::IntakeRecordAppend,
                        GenericArtifactOperationV1::ArtifactCandidateAppend,
                        GenericArtifactOperationV1::ArtifactCandidatePromote,
                    ]
                    .into_iter()
                    .any(|operation| valid_transaction_id(operation, identity))
                })
            };
            if !valid {
                return Err(conflicting(
                    "idempotency record name is outside exact grammar",
                ));
            }
        }
    }
    Ok(())
}

fn validate_instance_store_inventory(
    root: &Path,
    semantic: bool,
) -> Result<(), GenericLineageStoreErrorV1> {
    if !root
        .try_exists()
        .map_err(|_| io_error("artifact store inventory lookup failed"))?
    {
        return Ok(());
    }
    reject_link(root, true)?;
    let families: &[(&str, &str)] = if semantic {
        &[
            ("intake-records", "intake_"),
            ("promotion-records", "promotion_"),
        ]
    } else {
        &[
            ("intake-values", "value_"),
            ("content", "content_"),
            ("validation-results", "validation_"),
            ("candidates", "candidate_"),
        ]
    };
    let mut instances = read_dir_paths(root)?;
    instances.sort();
    for instance in instances {
        reject_link(&instance, true)?;
        let instance_id = file_name(&instance)?;
        validate_instance_id(&instance_id)?;
        let mut instance_bytes = 0u64;
        let mut family_paths = read_dir_paths(&instance)?;
        family_paths.sort();
        for family in family_paths {
            reject_link(&family, true)?;
            let family_name = file_name(&family)?;
            let prefix = families
                .iter()
                .find_map(|(name, prefix)| (*name == family_name).then_some(*prefix))
                .ok_or_else(|| conflicting("unknown artifact store family"))?;
            let mut files = read_dir_paths(&family)?;
            files.sort();
            let limit = if semantic {
                MAX_SEMANTIC_RECORDS_PER_FAMILY_INSTANCE
            } else {
                MAX_CLOSURE_RECORDS_PER_FAMILY_INSTANCE
            };
            if files.len() > limit {
                return Err(bound_error(
                    "artifact store family count exceeds exact limit",
                ));
            }
            for file in files {
                reject_link(&file, false)?;
                let metadata = fs::symlink_metadata(&file)
                    .map_err(|_| io_error("artifact store inventory metadata failed"))?;
                if !metadata.is_file()
                    || metadata.len() == 0
                    || metadata.len() > MAX_RECORD_BYTES as u64
                {
                    return Err(bound_error("artifact store record is unsafe or oversized"));
                }
                let filename = file_name(&file)?;
                let valid = filename
                    .strip_prefix(prefix)
                    .and_then(|suffix| suffix.strip_suffix(".json"))
                    .is_some_and(valid_lower_hex_64);
                if !valid {
                    return Err(conflicting(
                        "artifact store record name is outside exact grammar",
                    ));
                }
                instance_bytes = instance_bytes.saturating_add(metadata.len());
            }
        }
        if instance_bytes > MAX_INSTANCE_STORE_BYTES {
            return Err(bound_error(
                "artifact instance store exceeds its byte limit",
            ));
        }
    }
    Ok(())
}

fn valid_lower_hex_64(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn valid_transaction_id(operation: GenericArtifactOperationV1, value: &str) -> bool {
    value
        .strip_prefix(&format!("{}_", operation.transaction_token()))
        .is_some_and(valid_lower_hex_64)
}

fn read_dir_paths(path: &Path) -> Result<Vec<PathBuf>, GenericLineageStoreErrorV1> {
    fs::read_dir(path)
        .map_err(|_| io_error("directory inventory read failed"))?
        .map(|entry| {
            entry
                .map(|entry| entry.path())
                .map_err(|_| io_error("directory inventory entry failed"))
        })
        .collect()
}

fn file_name(path: &Path) -> Result<String, GenericLineageStoreErrorV1> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(str::to_owned)
        .ok_or_else(|| {
            error(
                GenericLineageStoreErrorKindV1::UnsafeFilesystem,
                "generic store basename is not UTF-8",
            )
        })
}

fn current_utc() -> Result<String, GenericLineageStoreErrorV1> {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| io_error("system time is before Unix epoch"))?
        .as_secs();
    format_utc(i64::try_from(seconds).map_err(|_| io_error("system time exceeds i64"))?)
}

fn validate_utc(value: &str) -> Result<(), GenericLineageStoreErrorV1> {
    parse_utc(value).map(|_| ())
}

fn parse_utc(value: &str) -> Result<i64, GenericLineageStoreErrorV1> {
    let bytes = value.as_bytes();
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
    {
        return Err(error(
            GenericLineageStoreErrorKindV1::InvalidRequest,
            "timestamp must be exact whole-second UTC",
        ));
    }
    let number = |range: std::ops::Range<usize>| -> Option<i64> {
        std::str::from_utf8(&bytes[range]).ok()?.parse().ok()
    };
    let invalid_number = || {
        error(
            GenericLineageStoreErrorKindV1::InvalidRequest,
            "timestamp contains a non-numeric calendar component",
        )
    };
    let year = number(0..4).ok_or_else(invalid_number)?;
    let month = number(5..7).ok_or_else(invalid_number)?;
    let day = number(8..10).ok_or_else(invalid_number)?;
    let hour = number(11..13).ok_or_else(invalid_number)?;
    let minute = number(14..16).ok_or_else(invalid_number)?;
    let second = number(17..19).ok_or_else(invalid_number)?;
    if !(1..=12).contains(&month)
        || !(1..=days_in_month(year, month)).contains(&day)
        || !(0..=23).contains(&hour)
        || !(0..=59).contains(&minute)
        || !(0..=59).contains(&second)
    {
        return Err(error(
            GenericLineageStoreErrorKindV1::InvalidRequest,
            "timestamp has an invalid calendar component",
        ));
    }
    Ok(days_from_civil(year, month, day) * 86_400 + hour * 3_600 + minute * 60 + second)
}

fn format_utc(seconds: i64) -> Result<String, GenericLineageStoreErrorV1> {
    if seconds < 0 {
        return Err(io_error("negative Unix timestamps are unsupported"));
    }
    let days = seconds.div_euclid(86_400);
    let within = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    if !(0..=9999).contains(&year) {
        return Err(io_error("UTC year exceeds four digits"));
    }
    Ok(format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        within / 3_600,
        (within % 3_600) / 60,
        within % 60
    ))
}

fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        _ => 0,
    }
}

fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = year - i64::from(month <= 2);
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let adjusted_month = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * adjusted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let days = days + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days - era * 146_097;
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

#[cfg(test)]
mod native_link_identity_tests {
    use super::{reject_link, GenericLineageStoreErrorKindV1};

    #[test]
    fn generic_store_files_reject_hard_link_aliases() {
        let repo = tempfile::tempdir().unwrap();
        let target = repo.path().join("target.json");
        let alias = repo.path().join("alias.json");
        std::fs::write(&target, b"{}\n").unwrap();
        std::fs::hard_link(&target, &alias).unwrap();

        let error = reject_link(&alias, false).expect_err("hard-link alias must fail closed");
        assert_eq!(
            error.kind(),
            GenericLineageStoreErrorKindV1::UnsafeFilesystem
        );
    }
}

#[cfg(test)]
mod inventory_boundary_tests {
    use super::*;

    const REPOSITORY_ID: &str =
        "sha256:1111111111111111111111111111111111111111111111111111111111111111";
    const OWNER_SUBJECT: &str =
        "sha256:6666666666666666666666666666666666666666666666666666666666666666";
    const CONTEXT: &str = "sha256:6247735430f2296162010902e7f6e1e20403f67ab6de7a4e4cc7fcdf9bd466c8";

    fn candidate_request(key: String) -> GenericMutationRequestV1 {
        GenericMutationRequestV1 {
            repository_identity_fingerprint: REPOSITORY_ID.to_owned(),
            owner_contract_subject_fingerprint: OWNER_SUBJECT.to_owned(),
            operation: GenericArtifactOperationV1::ArtifactCandidateAppend,
            idempotency_key: key,
            request_subject: json!({
                "operation_id": "artifact.candidate.append",
                "kind_ref": "example.artifact-kind.registry-brief@1.0.0",
                "instance_id": "registry_brief",
                "intake_record_ref": ".handbook/state/artifacts/registry_brief/intake-records/intake_b88c7d71baafeef02f379be8e317df15d9ad77d6b8cc7599ce3b25a6b69d684c.json",
                "intake_record_fingerprint": "sha256:b88c7d71baafeef02f379be8e317df15d9ad77d6b8cc7599ce3b25a6b69d684c",
                "expected_candidate_fingerprint": "sha256:dad8bb3c473f18ea28d6fd006f98a43d8574b5c031c05d0c6f2b14fcdf21d57c",
                "operation_context_fingerprint": CONTEXT
            }),
        }
    }

    fn intake_request(key: String) -> GenericMutationRequestV1 {
        GenericMutationRequestV1 {
            repository_identity_fingerprint: REPOSITORY_ID.to_owned(),
            owner_contract_subject_fingerprint: OWNER_SUBJECT.to_owned(),
            operation: GenericArtifactOperationV1::IntakeRecordAppend,
            idempotency_key: key,
            request_subject: json!({
                "operation_id": "intake.record.append",
                "kind_ref": "example.artifact-kind.registry-brief@1.0.0",
                "instance_id": "registry_brief",
                "acquisition_mode": "express",
                "expected_current_artifact_fingerprint": null,
                "coverage_input_fingerprint": "sha256:8888888888888888888888888888888888888888888888888888888888888888",
                "operation_context_fingerprint": CONTEXT
            }),
        }
    }

    fn refusal_intent(index: usize) -> Value {
        let request = candidate_request(format!("inventory_refusal_key_{index:08}"));
        let derived = validate_and_derive_request(&request).unwrap();
        build_intent(
            &derived,
            &GenericPlannedOutcomeV1::Refuse(GenericRefusalPlanV1 {
                operation_context_fingerprint: CONTEXT.to_owned(),
                expected_basis_fingerprint: None,
                refusal: EstablishedRefusalV1 {
                    code: EstablishedRefusalCodeV1::CanonicalSyntaxInvalid,
                    layer: GenericRefusalLayerV1::CanonicalSyntax,
                    expected_fingerprint: None,
                    observed_fingerprint: None,
                },
            }),
            None,
            None,
            None,
        )
        .unwrap()
    }

    fn maximum_output_intent() -> Value {
        let request = intake_request("inventory_maximum_output_key_01".to_owned());
        let derived = validate_and_derive_request(&request).unwrap();
        let mut outputs = Vec::new();
        for ordinal in 0..15 {
            let bytes = vec![u8::try_from(ordinal + 1).unwrap()];
            let fingerprint = sha256_prefixed(&bytes);
            outputs.push(GenericPlannedOutputV1 {
                token: format!("field-{ordinal}-value"),
                authority_class: GenericAuthorityClassV1::SubordinateClosure,
                final_ref: format!(
                    ".handbook/evidence/artifacts/registry_brief/intake-values/value_{}.json",
                    fingerprint_hex(&fingerprint).unwrap()
                ),
                bytes,
                output_fingerprint: fingerprint,
                install_mode: GenericInstallModeV1::CreateNew,
                receipt_class: None,
            });
        }
        let record_bytes = b"record\n".to_vec();
        let record_fingerprint = sha256_prefixed(&record_bytes);
        outputs.push(GenericPlannedOutputV1 {
            token: "intake-record".to_owned(),
            authority_class: GenericAuthorityClassV1::SemanticRecord,
            final_ref: format!(
                ".handbook/state/artifacts/registry_brief/intake-records/intake_{}.json",
                fingerprint_hex(&record_fingerprint).unwrap()
            ),
            bytes: record_bytes,
            output_fingerprint: record_fingerprint,
            install_mode: GenericInstallModeV1::CreateNew,
            receipt_class: Some(GenericReceiptClassV1::SemanticRecord),
        });
        build_intent(
            &derived,
            &GenericPlannedOutcomeV1::Commit(GenericCommitPlanV1 {
                operation_context_fingerprint: CONTEXT.to_owned(),
                expected_basis_fingerprint: None,
                sampled_finalized_at_utc: Some("2026-07-21T00:00:00Z".to_owned()),
                outputs,
            }),
            None,
            None,
            None,
        )
        .unwrap()
    }

    fn write_sized_transaction(family: &Path, index: usize, suffix: &str, size: u64) {
        let intent = refusal_intent(index);
        let transaction_id = string_field(&intent, "transaction_id").unwrap();
        let transaction = family.join(format!("{transaction_id}.{suffix}"));
        fs::create_dir_all(&transaction).unwrap();
        let intent_bytes = jcs_lf(&intent).unwrap();
        fs::write(transaction.join("intent.json"), &intent_bytes).unwrap();
        if size > intent_bytes.len() as u64 {
            let filler = File::create(transaction.join("verified.json")).unwrap();
            filler.set_len(size - intent_bytes.len() as u64).unwrap();
        }
    }

    fn assert_bound<T>(result: Result<T, GenericLineageStoreErrorV1>) {
        match result {
            Err(error) => assert_eq!(error.kind(), GenericLineageStoreErrorKindV1::BoundExceeded),
            Ok(_) => panic!("inventory must exceed its exact bound"),
        }
    }

    #[test]
    fn transaction_count_byte_and_grammar_boundaries_are_exact() {
        let repo = tempfile::tempdir().unwrap();
        let intent = maximum_output_intent();
        validate_intent(&intent).unwrap();
        let transaction = repo.path().join(format!(
            ".handbook/state/transactions/intake-records/{}.pending",
            string_field(&intent, "transaction_id").unwrap()
        ));
        let staged = transaction.join("staged");
        fs::create_dir_all(&staged).unwrap();
        fs::write(transaction.join("intent.json"), jcs_lf(&intent).unwrap()).unwrap();
        let outputs = intent_outputs(&intent).unwrap();
        for (ordinal, descriptor) in outputs.iter().take(15).enumerate() {
            fs::write(
                staged.join(staged_name(
                    ordinal,
                    string_field(descriptor, "token").unwrap(),
                )),
                b"x",
            )
            .unwrap();
        }
        validate_transaction_inventory(&transaction).unwrap();
        fs::write(
            staged.join(staged_name(
                15,
                string_field(&outputs[15], "token").unwrap(),
            )),
            b"x",
        )
        .unwrap();
        assert_bound(validate_transaction_inventory(&transaction));

        let repo = tempfile::tempdir().unwrap();
        let transaction = repo.path().join(format!(
            ".handbook/state/transactions/intake-records/{}.pending",
            string_field(&intent, "transaction_id").unwrap()
        ));
        let staged = transaction.join("staged");
        fs::create_dir_all(&staged).unwrap();
        let intent_bytes = jcs_lf(&intent).unwrap();
        fs::write(transaction.join("intent.json"), &intent_bytes).unwrap();
        let payload = staged.join(staged_name(0, string_field(&outputs[0], "token").unwrap()));
        File::create(&payload)
            .unwrap()
            .set_len(MAX_TRANSACTION_BYTES - intent_bytes.len() as u64)
            .unwrap();
        assert_eq!(
            validate_transaction_inventory(&transaction).unwrap(),
            MAX_TRANSACTION_BYTES
        );
        File::options()
            .write(true)
            .open(&payload)
            .unwrap()
            .set_len(MAX_TRANSACTION_BYTES - intent_bytes.len() as u64 + 1)
            .unwrap();
        assert_bound(validate_transaction_inventory(&transaction));

        let repo = tempfile::tempdir().unwrap();
        let refusal = refusal_intent(99);
        let transaction = repo.path().join(format!(
            ".handbook/state/transactions/artifact-candidates/{}.pending",
            string_field(&refusal, "transaction_id").unwrap()
        ));
        fs::create_dir_all(transaction.join("staged/nested")).unwrap();
        fs::write(transaction.join("intent.json"), jcs_lf(&refusal).unwrap()).unwrap();
        assert_eq!(
            validate_transaction_inventory(&transaction)
                .unwrap_err()
                .kind(),
            GenericLineageStoreErrorKindV1::ConflictingTransactionState
        );
    }

    #[test]
    fn journal_count_and_aggregate_byte_boundaries_are_exact() {
        let repo = tempfile::tempdir().unwrap();
        let family = repo
            .path()
            .join(".handbook/state/transactions/artifact-candidates");
        fs::create_dir_all(&family).unwrap();
        for index in 0..MAX_PENDING_PER_FAMILY {
            fs::create_dir(family.join(format!("artifact_candidate_append_{index:064x}.pending")))
                .unwrap();
        }
        for index in 0..MAX_COMMITTED_PER_FAMILY {
            fs::create_dir(family.join(format!(
                "artifact_candidate_append_{:064x}.committed",
                index + 10_000
            )))
            .unwrap();
        }
        let store = GenericArtifactLineageStoreV1::new(repo.path());
        store.validate_inventory().unwrap();
        let extra_pending =
            family.join(format!("artifact_candidate_append_{:064x}.pending", 20_000));
        fs::create_dir(&extra_pending).unwrap();
        assert_bound(store.validate_inventory());
        fs::remove_dir(&extra_pending).unwrap();
        let extra_committed = family.join(format!(
            "artifact_candidate_append_{:064x}.committed",
            20_001
        ));
        fs::create_dir(&extra_committed).unwrap();
        assert_bound(store.validate_inventory());

        let repo = tempfile::tempdir().unwrap();
        let family = repo
            .path()
            .join(".handbook/state/transactions/artifact-candidates");
        fs::create_dir_all(&family).unwrap();
        for index in 0..4 {
            write_sized_transaction(&family, index, "pending", MAX_TRANSACTION_BYTES);
        }
        let store = GenericArtifactLineageStoreV1::new(repo.path());
        store.validate_inventory().unwrap();
        write_sized_transaction(&family, 4, "pending", 1);
        assert_bound(store.validate_inventory());

        let repo = tempfile::tempdir().unwrap();
        let family = repo
            .path()
            .join(".handbook/state/transactions/artifact-candidates");
        fs::create_dir_all(&family).unwrap();
        for index in 0..16 {
            write_sized_transaction(&family, index, "committed", MAX_TRANSACTION_BYTES);
        }
        let store = GenericArtifactLineageStoreV1::new(repo.path());
        store.validate_inventory().unwrap();
        write_sized_transaction(&family, 16, "committed", 1);
        assert_bound(store.validate_inventory());
    }

    #[test]
    fn idempotency_and_instance_store_boundaries_are_exact() {
        let repo = tempfile::tempdir().unwrap();
        let results = repo.path().join("results");
        fs::create_dir(&results).unwrap();
        let result = results.join(format!("artifact_candidate_append_{}.json", "a".repeat(64)));
        File::create(&result)
            .unwrap()
            .set_len(MAX_RECORD_BYTES as u64)
            .unwrap();
        validate_idempotency_inventory(repo.path()).unwrap();
        File::options()
            .write(true)
            .open(&result)
            .unwrap()
            .set_len(MAX_RECORD_BYTES as u64 + 1)
            .unwrap();
        assert_bound(validate_idempotency_inventory(repo.path()));

        let repo = tempfile::tempdir().unwrap();
        let family = repo.path().join("registry_brief/intake-records");
        fs::create_dir_all(&family).unwrap();
        for index in 0..MAX_SEMANTIC_RECORDS_PER_FAMILY_INSTANCE {
            fs::write(family.join(format!("intake_{index:064x}.json")), b"x").unwrap();
        }
        validate_instance_store_inventory(repo.path(), true).unwrap();
        fs::write(
            family.join(format!(
                "intake_{:064x}.json",
                MAX_SEMANTIC_RECORDS_PER_FAMILY_INSTANCE
            )),
            b"x",
        )
        .unwrap();
        assert_bound(validate_instance_store_inventory(repo.path(), true));

        let repo = tempfile::tempdir().unwrap();
        let family = repo.path().join("registry_brief/content");
        fs::create_dir_all(&family).unwrap();
        for index in 0..MAX_CLOSURE_RECORDS_PER_FAMILY_INSTANCE {
            fs::write(family.join(format!("content_{index:064x}.json")), b"x").unwrap();
        }
        validate_instance_store_inventory(repo.path(), false).unwrap();
        fs::write(
            family.join(format!(
                "content_{:064x}.json",
                MAX_CLOSURE_RECORDS_PER_FAMILY_INSTANCE
            )),
            b"x",
        )
        .unwrap();
        assert_bound(validate_instance_store_inventory(repo.path(), false));

        let repo = tempfile::tempdir().unwrap();
        let family = repo.path().join("registry_brief/promotion-records");
        fs::create_dir_all(&family).unwrap();
        for index in 0..256 {
            File::create(family.join(format!("promotion_{index:064x}.json")))
                .unwrap()
                .set_len(MAX_RECORD_BYTES as u64)
                .unwrap();
        }
        validate_instance_store_inventory(repo.path(), true).unwrap();
        fs::write(family.join(format!("promotion_{:064x}.json", 256)), b"x").unwrap();
        assert_bound(validate_instance_store_inventory(repo.path(), true));
    }
}
