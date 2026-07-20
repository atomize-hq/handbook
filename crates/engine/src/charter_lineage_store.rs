use crate::canonical_repo_support::{CanonicalWorkspace, RepoRelativeFileAccessError};
use crate::charter_intake::CharterCandidateBundle;
use crate::definition_identity::canonical_json_bytes;
use crate::{parse_schema_json, DefinitionFingerprint};
use serde_json::Value;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

pub const MAX_LINEAGE_RECORD_BYTES: usize = 1024 * 1024;
pub const MAX_LINEAGE_REFERENCE_BYTES: usize = 512;
pub const MAX_CANDIDATE_CONTENT_BYTES: usize = 8 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineageRecordClassV1 {
    Intake,
    Candidate,
    Approval,
    Promotion,
    Waiver,
    RegistryState,
    RegistryTransition,
    AuthenticatorRegistration,
    AuthenticatorMakeCredentialResponse,
    AuthenticatorGetAssertionResponse,
    AuthenticatorAssertion,
    LifecycleObservation,
    LifecycleTransition,
    TriggerEvidence,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AppendDispositionV1 {
    Created,
    ReusedEqual,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineageAppendResultV1 {
    pub relative_ref: String,
    pub record_id: String,
    pub fingerprint: String,
    pub disposition: AppendDispositionV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateBundlePersistenceV1 {
    pub normalized_content_ref: String,
    pub intake_ref: String,
    pub candidate_ref: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineageStoreErrorKindV1 {
    UnsupportedPlatform,
    BoundExceeded,
    UnsafeReference,
    InvalidRecord,
    IdentityMismatch,
    BasisMismatch,
    MissingDependency,
    ExistingBytesMismatch,
    UnsafeFilesystem,
    IoFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineageStoreErrorV1 {
    kind: LineageStoreErrorKindV1,
    detail: String,
}

impl LineageStoreErrorV1 {
    fn new(kind: LineageStoreErrorKindV1, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> LineageStoreErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Debug)]
pub struct TrustedLineageStoreV1 {
    repo_root: PathBuf,
}

impl TrustedLineageStoreV1 {
    pub fn new(repo_root: impl AsRef<Path>) -> Self {
        Self {
            repo_root: repo_root.as_ref().to_path_buf(),
        }
    }

    pub fn repo_root(&self) -> &Path {
        &self.repo_root
    }

    pub fn append_record(
        &self,
        class: LineageRecordClassV1,
        bytes: &[u8],
    ) -> Result<LineageAppendResultV1, LineageStoreErrorV1> {
        self.append_record_inner(class, bytes)
    }

    pub fn read_record(
        &self,
        class: LineageRecordClassV1,
        relative_ref: &str,
        declared_fingerprint: &str,
    ) -> Result<Vec<u8>, LineageStoreErrorV1> {
        let identity = validate_reference(class, relative_ref, declared_fingerprint)?;
        let repo_relative = format!(".handbook/state/{relative_ref}");
        let workspace = CanonicalWorkspace::new(&self.repo_root);
        let normalized = workspace
            .normalize_repo_relative(&repo_relative)
            .map_err(|_| {
                LineageStoreErrorV1::new(
                    LineageStoreErrorKindV1::UnsafeReference,
                    "lineage reference is not a safe repository-relative path",
                )
            })?;
        let file = workspace
            .trusted_read_strict(&normalized)
            .map_err(map_read_error)?;
        let (bytes, exceeded) = file
            .read_bytes_bounded(MAX_LINEAGE_RECORD_BYTES)
            .map_err(|_| io_error("lineage record could not be read"))?;
        if exceeded {
            return Err(LineageStoreErrorV1::new(
                LineageStoreErrorKindV1::BoundExceeded,
                "lineage record exceeds the 1 MiB admission bound",
            ));
        }
        let observed = validate_record(class, &bytes)?;
        if observed.record_id != identity.record_id
            || observed.fingerprint != declared_fingerprint
            || observed.relative_ref != relative_ref
        {
            return Err(LineageStoreErrorV1::new(
                LineageStoreErrorKindV1::IdentityMismatch,
                "lineage ref basename, record ID, fingerprint, and exact bytes disagree",
            ));
        }
        Ok(bytes)
    }

    pub fn persist_candidate_bundle(
        &self,
        bundle: &CharterCandidateBundle,
    ) -> Result<CandidateBundlePersistenceV1, LineageStoreErrorV1> {
        ensure_supported_mutation_platform()?;
        if bundle.normalized_content.is_empty()
            || bundle.normalized_content.len() > MAX_CANDIDATE_CONTENT_BYTES
        {
            return Err(LineageStoreErrorV1::new(
                LineageStoreErrorKindV1::BoundExceeded,
                "normalized candidate content must be non-empty and no larger than 8 MiB",
            ));
        }
        let content_fingerprint = DefinitionFingerprint::from_bytes(&bundle.normalized_content);
        let expected_content_ref = format!(
            "candidate-content/charter_{}.yaml",
            content_fingerprint
                .as_str()
                .strip_prefix("sha256:")
                .expect("fingerprint is normalized")
        );
        if bundle.candidate.normalized_content_ref != expected_content_ref {
            return Err(LineageStoreErrorV1::new(
                LineageStoreErrorKindV1::IdentityMismatch,
                "candidate normalized-content ref does not equal the exact content fingerprint",
            ));
        }
        let intake_bytes = serde_json_canonicalizer::to_vec(&bundle.intake)
            .map_err(|_| io_error("intake record canonicalization failed"))?;
        let candidate_bytes = serde_json_canonicalizer::to_vec(&bundle.candidate)
            .map_err(|_| io_error("candidate record canonicalization failed"))?;
        let intake = validate_record(LineageRecordClassV1::Intake, &intake_bytes)?;
        let candidate = validate_record(LineageRecordClassV1::Candidate, &candidate_bytes)?;
        if candidate
            .value
            .get("intake_record_ref")
            .and_then(Value::as_str)
            != Some(intake.relative_ref.as_str())
            || candidate.value.get("basis_artifact_fingerprint")
                != intake.value.get("basis_artifact_fingerprint")
        {
            return Err(LineageStoreErrorV1::new(
                LineageStoreErrorKindV1::BasisMismatch,
                "candidate does not bind the exact intake record and retained basis",
            ));
        }

        // Preflight every destination before the first filesystem delta. Candidate is
        // installed last, so its presence is the bundle's append-only visibility point.
        self.preflight_create_new_or_equal(
            &expected_content_ref,
            &bundle.normalized_content,
            MAX_CANDIDATE_CONTENT_BYTES,
        )?;
        self.preflight_create_new_or_equal(
            &intake.relative_ref,
            &intake_bytes,
            MAX_LINEAGE_RECORD_BYTES,
        )?;
        self.preflight_create_new_or_equal(
            &candidate.relative_ref,
            &candidate_bytes,
            MAX_LINEAGE_RECORD_BYTES,
        )?;
        self.append_exact_blob(&expected_content_ref, &bundle.normalized_content)?;
        self.append_record_inner(LineageRecordClassV1::Intake, &intake_bytes)?;
        self.append_record_inner(LineageRecordClassV1::Candidate, &candidate_bytes)?;
        Ok(CandidateBundlePersistenceV1 {
            normalized_content_ref: expected_content_ref,
            intake_ref: intake.relative_ref,
            candidate_ref: candidate.relative_ref,
        })
    }

    pub fn persist_approval_record(
        &self,
        canonical_approval_bytes: &[u8],
    ) -> Result<LineageAppendResultV1, LineageStoreErrorV1> {
        self.append_record_inner(LineageRecordClassV1::Approval, canonical_approval_bytes)
    }

    pub(crate) fn state_path(&self, relative: &str) -> Result<PathBuf, LineageStoreErrorV1> {
        validate_relative_path(relative)?;
        Ok(self.repo_root.join(".handbook/state").join(relative))
    }

    pub(crate) fn preflight_record(
        &self,
        class: LineageRecordClassV1,
        bytes: &[u8],
    ) -> Result<(), LineageStoreErrorV1> {
        let identity = validate_record(class, bytes)?;
        self.validate_immediate_lineage(class, &identity.value)?;
        self.preflight_create_new_or_equal(&identity.relative_ref, bytes, MAX_LINEAGE_RECORD_BYTES)
    }

    pub(crate) fn read_candidate_content(
        &self,
        relative_ref: &str,
    ) -> Result<Vec<u8>, LineageStoreErrorV1> {
        validate_relative_path(relative_ref)?;
        let repo_relative = format!(".handbook/state/{relative_ref}");
        let workspace = CanonicalWorkspace::new(&self.repo_root);
        let normalized = workspace
            .normalize_repo_relative(&repo_relative)
            .map_err(|_| {
                LineageStoreErrorV1::new(
                    LineageStoreErrorKindV1::UnsafeReference,
                    "candidate-content reference is not normalized",
                )
            })?;
        let file = workspace
            .trusted_read_strict(&normalized)
            .map_err(map_read_error)?;
        let (bytes, exceeded) = file
            .read_bytes_bounded(MAX_CANDIDATE_CONTENT_BYTES)
            .map_err(|_| io_error("candidate content read failed"))?;
        if exceeded {
            return Err(LineageStoreErrorV1::new(
                LineageStoreErrorKindV1::BoundExceeded,
                "candidate content exceeds 8 MiB",
            ));
        }
        Ok(bytes)
    }

    pub(crate) fn append_record_inner(
        &self,
        class: LineageRecordClassV1,
        bytes: &[u8],
    ) -> Result<LineageAppendResultV1, LineageStoreErrorV1> {
        ensure_supported_mutation_platform()?;
        if bytes.len() > MAX_LINEAGE_RECORD_BYTES {
            return Err(LineageStoreErrorV1::new(
                LineageStoreErrorKindV1::BoundExceeded,
                "lineage record exceeds the 1 MiB admission bound",
            ));
        }
        let identity = validate_record(class, bytes)?;
        self.validate_immediate_lineage(class, &identity.value)?;

        let final_path = self.state_path(&identity.relative_ref)?;
        let parent = final_path.parent().ok_or_else(|| {
            LineageStoreErrorV1::new(
                LineageStoreErrorKindV1::UnsafeReference,
                "lineage record has no trusted parent",
            )
        })?;
        create_safe_directories(&self.repo_root, parent)?;

        match create_new_file(&final_path) {
            Ok(mut file) => {
                file.write_all(bytes)
                    .map_err(|_| io_error("lineage record write failed"))?;
                file.sync_all()
                    .map_err(|_| io_error("lineage record fsync failed"))?;
                sync_directory(parent)?;
                Ok(LineageAppendResultV1 {
                    relative_ref: identity.relative_ref,
                    record_id: identity.record_id,
                    fingerprint: identity.fingerprint,
                    disposition: AppendDispositionV1::Created,
                })
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                let repo_relative = format!(".handbook/state/{}", identity.relative_ref);
                let workspace = CanonicalWorkspace::new(&self.repo_root);
                let normalized =
                    workspace
                        .normalize_repo_relative(&repo_relative)
                        .map_err(|_| {
                            LineageStoreErrorV1::new(
                                LineageStoreErrorKindV1::UnsafeReference,
                                "lineage reference is not a safe repository-relative path",
                            )
                        })?;
                let file = workspace
                    .trusted_read_strict(&normalized)
                    .map_err(map_read_error)?;
                let (observed, exceeded) = file
                    .read_bytes_bounded(MAX_LINEAGE_RECORD_BYTES)
                    .map_err(|_| io_error("existing lineage record read failed"))?;
                if exceeded {
                    return Err(LineageStoreErrorV1::new(
                        LineageStoreErrorKindV1::ExistingBytesMismatch,
                        "an existing content-addressed record has unequal bytes",
                    ));
                }
                if observed != bytes {
                    return Err(LineageStoreErrorV1::new(
                        LineageStoreErrorKindV1::ExistingBytesMismatch,
                        "an existing content-addressed record has unequal bytes",
                    ));
                }
                Ok(LineageAppendResultV1 {
                    relative_ref: identity.relative_ref,
                    record_id: identity.record_id,
                    fingerprint: identity.fingerprint,
                    disposition: AppendDispositionV1::ReusedEqual,
                })
            }
            Err(_) => Err(io_error("lineage record create-new failed")),
        }
    }

    fn preflight_create_new_or_equal(
        &self,
        relative_ref: &str,
        expected: &[u8],
        limit: usize,
    ) -> Result<(), LineageStoreErrorV1> {
        let final_path = self.state_path(relative_ref)?;
        match fs::symlink_metadata(&final_path) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink() || !metadata.is_file() {
                    return Err(LineageStoreErrorV1::new(
                        LineageStoreErrorKindV1::UnsafeFilesystem,
                        "preexisting append-only destination is not a safe regular file",
                    ));
                }
                let repo_relative = format!(".handbook/state/{relative_ref}");
                let workspace = CanonicalWorkspace::new(&self.repo_root);
                let normalized =
                    workspace
                        .normalize_repo_relative(&repo_relative)
                        .map_err(|_| {
                            LineageStoreErrorV1::new(
                                LineageStoreErrorKindV1::UnsafeReference,
                                "append-only destination is not normalized",
                            )
                        })?;
                let file = workspace
                    .trusted_read_strict(&normalized)
                    .map_err(map_read_error)?;
                let (observed, exceeded) = file
                    .read_bytes_bounded(limit)
                    .map_err(|_| io_error("append-only preflight read failed"))?;
                if exceeded || observed != expected {
                    return Err(LineageStoreErrorV1::new(
                        LineageStoreErrorKindV1::ExistingBytesMismatch,
                        "preexisting append-only destination has unequal bytes",
                    ));
                }
                Ok(())
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(_) => Err(io_error("append-only destination metadata failed")),
        }
    }

    fn append_exact_blob(
        &self,
        relative_ref: &str,
        bytes: &[u8],
    ) -> Result<AppendDispositionV1, LineageStoreErrorV1> {
        validate_relative_path(relative_ref)?;
        let final_path = self.state_path(relative_ref)?;
        let parent = final_path.parent().ok_or_else(|| {
            LineageStoreErrorV1::new(
                LineageStoreErrorKindV1::UnsafeReference,
                "candidate content has no trusted parent",
            )
        })?;
        create_safe_directories(&self.repo_root, parent)?;
        match create_new_file(&final_path) {
            Ok(mut file) => {
                file.write_all(bytes)
                    .map_err(|_| io_error("candidate content write failed"))?;
                file.sync_all()
                    .map_err(|_| io_error("candidate content flush failed"))?;
                sync_directory(parent)?;
                Ok(AppendDispositionV1::Created)
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                self.preflight_create_new_or_equal(
                    relative_ref,
                    bytes,
                    MAX_CANDIDATE_CONTENT_BYTES,
                )?;
                Ok(AppendDispositionV1::ReusedEqual)
            }
            Err(_) => Err(io_error("candidate content create-new failed")),
        }
    }

    fn validate_immediate_lineage(
        &self,
        class: LineageRecordClassV1,
        record: &Value,
    ) -> Result<(), LineageStoreErrorV1> {
        match class {
            LineageRecordClassV1::Candidate => {
                let intake_ref = string_field(record, "intake_record_ref")?;
                let intake =
                    self.read_record_by_ref(LineageRecordClassV1::Intake, intake_ref, None)?;
                equal_basis(record, &intake)
            }
            LineageRecordClassV1::Approval => {
                let candidate_ref = string_field(record, "candidate_ref")?;
                let fingerprint = string_field(record, "candidate_fingerprint")?;
                let candidate = self.read_record_by_ref(
                    LineageRecordClassV1::Candidate,
                    candidate_ref,
                    Some(fingerprint),
                )?;
                equal_basis(record, &candidate)
            }
            _ => Ok(()),
        }
    }

    pub(crate) fn read_record_by_ref(
        &self,
        class: LineageRecordClassV1,
        relative_ref: &str,
        declared_fingerprint: Option<&str>,
    ) -> Result<Value, LineageStoreErrorV1> {
        let fingerprint = declared_fingerprint
            .map(str::to_owned)
            .or_else(|| fingerprint_from_reference(relative_ref))
            .ok_or_else(|| {
                LineageStoreErrorV1::new(
                    LineageStoreErrorKindV1::UnsafeReference,
                    "lineage reference does not carry a content-addressed identity",
                )
            })?;
        let bytes = self.read_record(class, relative_ref, &fingerprint)?;
        parse_schema_json(&bytes).map_err(|_| {
            LineageStoreErrorV1::new(
                LineageStoreErrorKindV1::InvalidRecord,
                "lineage record is not closed JSON",
            )
        })
    }
}

#[derive(Debug)]
pub(crate) struct ValidatedRecord {
    pub(crate) value: Value,
    pub(crate) relative_ref: String,
    pub(crate) record_id: String,
    pub(crate) fingerprint: String,
}

pub(crate) fn validate_record(
    class: LineageRecordClassV1,
    bytes: &[u8],
) -> Result<ValidatedRecord, LineageStoreErrorV1> {
    if bytes.len() > MAX_LINEAGE_RECORD_BYTES {
        return Err(LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::BoundExceeded,
            "lineage record exceeds the 1 MiB admission bound",
        ));
    }
    let value = parse_schema_json(bytes).map_err(|_| {
        LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::InvalidRecord,
            "lineage record is not bounded duplicate-free JSON",
        )
    })?;
    let canonical = canonical_json_bytes(&value).map_err(|_| {
        LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::InvalidRecord,
            "lineage record cannot be RFC 8785 canonicalized",
        )
    })?;
    if canonical != bytes {
        return Err(LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::InvalidRecord,
            "stored lineage bytes must be exact RFC 8785 JCS",
        ));
    }

    let contract = class.contract();
    if string_field(&value, "schema_id")? != contract.schema_id
        || string_field(&value, "schema_version")? != contract.schema_version
    {
        return Err(LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::InvalidRecord,
            "lineage record schema identity does not match its partition",
        ));
    }
    let fingerprint = string_field(&value, contract.fingerprint_field)?.to_owned();
    DefinitionFingerprint::parse(&fingerprint).map_err(|_| {
        LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::IdentityMismatch,
            "lineage fingerprint is not lowercase SHA-256",
        )
    })?;
    let mut preimage = value.clone();
    let object = preimage.as_object_mut().ok_or_else(|| {
        LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::InvalidRecord,
            "lineage record must have an object root",
        )
    })?;
    object.remove(contract.fingerprint_field);
    if let Some(id_field) = contract.id_field {
        object.remove(id_field);
    }
    for audit_field in contract.audit_only_fields {
        object.remove(*audit_field);
    }
    let computed = DefinitionFingerprint::from_json_value(&preimage)
        .map_err(|_| io_error("lineage fingerprint preimage could not be canonicalized"))?
        .to_string();
    if computed != fingerprint {
        return Err(LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::IdentityMismatch,
            "lineage record fingerprint does not recompute from exact exclusions",
        ));
    }
    let hex = fingerprint
        .strip_prefix("sha256:")
        .expect("validated fingerprint");
    let record_id = format!("{}_{}", contract.id_prefix, hex);
    if let Some(id_field) = contract.id_field {
        if string_field(&value, id_field)? != record_id {
            return Err(LineageStoreErrorV1::new(
                LineageStoreErrorKindV1::IdentityMismatch,
                "lineage record ID does not equal its content-addressed fingerprint",
            ));
        }
    }
    let relative_ref = format!("{}/{}.json", contract.partition, record_id);
    Ok(ValidatedRecord {
        value,
        relative_ref,
        record_id,
        fingerprint,
    })
}

#[derive(Clone, Copy)]
struct RecordContract {
    partition: &'static str,
    id_prefix: &'static str,
    id_field: Option<&'static str>,
    fingerprint_field: &'static str,
    audit_only_fields: &'static [&'static str],
    schema_id: &'static str,
    schema_version: &'static str,
}

impl LineageRecordClassV1 {
    fn contract(self) -> RecordContract {
        match self {
            Self::Intake => contract(
                "intake-records",
                "intake",
                Some("intake_record_id"),
                "record_fingerprint",
                &["finalized_at_utc"],
                "handbook.artifact-intake-record",
                "1.1",
            ),
            Self::Candidate => contract(
                "candidates",
                "candidate",
                Some("candidate_id"),
                "candidate_fingerprint",
                &[],
                "handbook.artifact-candidate",
                "1.1",
            ),
            Self::Approval => contract(
                "approvals",
                "approval",
                Some("approval_id"),
                "approval_fingerprint",
                &[],
                "handbook.artifact-approval-record",
                "1.1",
            ),
            Self::Promotion => contract(
                "promotions",
                "promotion",
                Some("promotion_id"),
                "promotion_fingerprint",
                &[],
                "handbook.artifact-promotion-record",
                "1.1",
            ),
            Self::Waiver => contract(
                "waivers",
                "waiver",
                Some("waiver_id"),
                "waiver_fingerprint",
                &[],
                "handbook.artifact-intake-waiver",
                "1.0",
            ),
            Self::RegistryState => contract(
                "registry-states",
                "registry-state",
                None,
                "registry_state_fingerprint",
                &[],
                "handbook.approver-registry-state",
                "1.0",
            ),
            Self::RegistryTransition => contract(
                "registry-transitions",
                "registry-transition",
                Some("transition_id"),
                "transition_fingerprint",
                &["occurred_at_utc"],
                "handbook.approver-registry-transition",
                "1.0",
            ),
            Self::AuthenticatorRegistration => contract(
                "authenticator-registrations",
                "authenticator-registration",
                Some("registration_id"),
                "registration_fingerprint",
                &["registered_at_utc"],
                "handbook.authenticator-registration",
                "1.0",
            ),
            Self::AuthenticatorMakeCredentialResponse => contract(
                "authenticator-make-credential-responses",
                "authenticator-make-credential-response",
                Some("response_id"),
                "response_fingerprint",
                &[],
                "handbook.authenticator-make-credential-response",
                "1.0",
            ),
            Self::AuthenticatorGetAssertionResponse => contract(
                "authenticator-get-assertion-responses",
                "authenticator-get-assertion-response",
                Some("response_id"),
                "response_fingerprint",
                &[],
                "handbook.authenticator-get-assertion-response",
                "1.0",
            ),
            Self::AuthenticatorAssertion => contract(
                "authenticator-assertions",
                "authenticator-assertion",
                Some("assertion_id"),
                "assertion_fingerprint",
                &["verified_at_utc"],
                "handbook.authenticator-assertion",
                "1.0",
            ),
            Self::LifecycleObservation => contract(
                "lifecycle-observations",
                "lifecycle-observation",
                Some("observation_id"),
                "observation_fingerprint",
                &[],
                "handbook.lifecycle-observation",
                "1.0",
            ),
            Self::LifecycleTransition => contract(
                "lifecycle-transitions",
                "lifecycle-transition",
                Some("transition_id"),
                "transition_fingerprint",
                &["transitioned_at_utc"],
                "handbook.lifecycle-transition",
                "1.0",
            ),
            Self::TriggerEvidence => contract(
                "trigger-evidence",
                "trigger-evidence",
                Some("event_id"),
                "evidence_fingerprint",
                &[],
                "handbook.lifecycle-trigger-evidence",
                "1.0",
            ),
        }
    }
}

const fn contract(
    partition: &'static str,
    id_prefix: &'static str,
    id_field: Option<&'static str>,
    fingerprint_field: &'static str,
    audit_only_fields: &'static [&'static str],
    schema_id: &'static str,
    schema_version: &'static str,
) -> RecordContract {
    RecordContract {
        partition,
        id_prefix,
        id_field,
        fingerprint_field,
        audit_only_fields,
        schema_id,
        schema_version,
    }
}

fn validate_reference(
    class: LineageRecordClassV1,
    relative_ref: &str,
    declared_fingerprint: &str,
) -> Result<ValidatedReference, LineageStoreErrorV1> {
    if relative_ref.len() > MAX_LINEAGE_REFERENCE_BYTES {
        return Err(LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::BoundExceeded,
            "lineage reference exceeds 512 bytes",
        ));
    }
    validate_relative_path(relative_ref)?;
    DefinitionFingerprint::parse(declared_fingerprint).map_err(|_| {
        LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::UnsafeReference,
            "declared lineage fingerprint is invalid",
        )
    })?;
    let contract = class.contract();
    let hex = declared_fingerprint
        .strip_prefix("sha256:")
        .expect("validated fingerprint");
    let record_id = format!("{}_{}", contract.id_prefix, hex);
    let expected = format!("{}/{}.json", contract.partition, record_id);
    if relative_ref != expected {
        return Err(LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::UnsafeReference,
            "lineage reference is not the exact class partition and fingerprint-derived basename",
        ));
    }
    Ok(ValidatedReference { record_id })
}

struct ValidatedReference {
    record_id: String,
}

fn validate_relative_path(path: &str) -> Result<(), LineageStoreErrorV1> {
    if path.is_empty()
        || path.len() > MAX_LINEAGE_REFERENCE_BYTES
        || path.contains('\\')
        || path.chars().any(char::is_control)
        || Path::new(path).is_absolute()
        || Path::new(path)
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::UnsafeReference,
            "lineage path must be a bounded normalized repository-relative path",
        ));
    }
    Ok(())
}

fn fingerprint_from_reference(relative_ref: &str) -> Option<String> {
    let basename = relative_ref.rsplit('/').next()?.strip_suffix(".json")?;
    let (_, hex) = basename.rsplit_once('_')?;
    DefinitionFingerprint::parse(&format!("sha256:{hex}"))
        .ok()
        .map(|value| value.to_string())
}

fn equal_basis(left: &Value, right: &Value) -> Result<(), LineageStoreErrorV1> {
    if left.get("basis_artifact_fingerprint") != right.get("basis_artifact_fingerprint") {
        return Err(LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::BasisMismatch,
            "downstream lineage basis does not equal its retained upstream basis",
        ));
    }
    Ok(())
}

pub(crate) fn string_field<'a>(
    value: &'a Value,
    field: &str,
) -> Result<&'a str, LineageStoreErrorV1> {
    value.get(field).and_then(Value::as_str).ok_or_else(|| {
        LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::InvalidRecord,
            format!("lineage record requires string field `{field}`"),
        )
    })
}

fn ensure_supported_mutation_platform() -> Result<(), LineageStoreErrorV1> {
    if cfg!(any(unix, windows)) {
        Ok(())
    } else {
        Err(LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::UnsupportedPlatform,
            "authority mutation requires native no-follow, lock, rename, and durable-flush support",
        ))
    }
}

pub(crate) fn create_safe_directories(
    trusted_root: &Path,
    target: &Path,
) -> Result<(), LineageStoreErrorV1> {
    let relative = target.strip_prefix(trusted_root).map_err(|_| {
        LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::UnsafeFilesystem,
            "state directory escapes the trusted repository root",
        )
    })?;
    let mut current = trusted_root.to_path_buf();
    reject_reparse_or_symlink(&current, true)?;
    for component in relative.components() {
        let Component::Normal(part) = component else {
            return Err(LineageStoreErrorV1::new(
                LineageStoreErrorKindV1::UnsafeFilesystem,
                "state directory is not normalized",
            ));
        };
        let parent = current.clone();
        current.push(part);
        match fs::create_dir(&current) {
            Ok(()) => sync_directory(&parent)?,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(_) => return Err(io_error("state directory create failed")),
        }
        reject_reparse_or_symlink(&current, true)?;
    }
    Ok(())
}

pub(crate) fn reject_reparse_or_symlink(
    path: &Path,
    require_directory: bool,
) -> Result<(), LineageStoreErrorV1> {
    let metadata =
        fs::symlink_metadata(path).map_err(|_| io_error("trusted state metadata failed"))?;
    if metadata.file_type().is_symlink() {
        return Err(LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::UnsafeFilesystem,
            "trusted state path contains a symlink",
        ));
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
        if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Err(LineageStoreErrorV1::new(
                LineageStoreErrorKindV1::UnsafeFilesystem,
                "trusted state path contains a reparse point",
            ));
        }
    }
    if require_directory && !metadata.is_dir() {
        return Err(LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::UnsafeFilesystem,
            "trusted state path component is not a directory",
        ));
    }
    Ok(())
}

pub(crate) fn create_new_file(path: &Path) -> Result<fs::File, std::io::Error> {
    let mut options = OpenOptions::new();
    options.create_new(true).write(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        // Source: https://learn.microsoft.com/windows/win32/api/fileapi/nf-fileapi-createfilew
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        const FILE_FLAG_WRITE_THROUGH: u32 = 0x8000_0000;
        options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_WRITE_THROUGH);
    }
    options.open(path)
}

pub(crate) fn sync_directory(path: &Path) -> Result<(), LineageStoreErrorV1> {
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        // Sources: https://learn.microsoft.com/windows/win32/api/fileapi/nf-fileapi-flushfilebuffers
        // and https://doc.rust-lang.org/std/os/windows/fs/trait.OpenOptionsExt.html
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
        let directory = fs::File::open(path).map_err(|_| io_error("directory open failed"))?;
        directory
            .sync_all()
            .map_err(|_| io_error("directory fsync failed"))?;
        return Ok(());
    }
    #[allow(unreachable_code)]
    Err(LineageStoreErrorV1::new(
        LineageStoreErrorKindV1::UnsupportedPlatform,
        "directory durability is unavailable on this platform",
    ))
}

fn map_read_error(error: RepoRelativeFileAccessError) -> LineageStoreErrorV1 {
    match error {
        RepoRelativeFileAccessError::Missing(_) => LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::MissingDependency,
            "referenced lineage record is missing",
        ),
        RepoRelativeFileAccessError::InvalidPath(_)
        | RepoRelativeFileAccessError::SymlinkNotAllowed(_)
        | RepoRelativeFileAccessError::NotRegularFile(_) => LineageStoreErrorV1::new(
            LineageStoreErrorKindV1::UnsafeFilesystem,
            "referenced lineage record is not a safe regular file",
        ),
        RepoRelativeFileAccessError::ReadFailure { .. } => {
            io_error("referenced lineage record read failed")
        }
    }
}

fn io_error(detail: &'static str) -> LineageStoreErrorV1 {
    LineageStoreErrorV1::new(LineageStoreErrorKindV1::IoFailure, detail)
}
