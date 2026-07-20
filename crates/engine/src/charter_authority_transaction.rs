use crate::approver_registry_mutation::recover_complete_registry_approval_authority_locked;
use crate::canonical_repo_support::{CanonicalWorkspace, RepoRelativeFileAccessError};
use crate::charter_lifecycle_store::CharterLifecycleStoreV1;
use crate::charter_lineage_store::{
    create_new_file, create_safe_directories, reject_reparse_or_symlink, string_field,
    sync_directory, validate_record, LineageRecordClassV1, LineageStoreErrorV1,
    TrustedLineageStoreV1,
};
use crate::DefinitionFingerprint;
use crate::{
    load_shipped_charter_definition_registry, parse_canonical_charter,
    resolve_shipped_profile_decisions, serialize_canonical_charter, ExactDefinitionRef,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

const CANONICAL_REF: &str = ".handbook/project/charter.yaml";
const MAX_CANONICAL_BYTES: usize = 8 * 1024 * 1024;
const MAX_INTENT_BYTES: usize = 64 * 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CharterPromotionRequestV1 {
    pub(crate) transaction_id: String,
    pub(crate) canonical_bytes: Vec<u8>,
    pub(crate) promotion_record_bytes: Vec<u8>,
    pub(crate) lifecycle_transition_bytes: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterPromotionCommitV1 {
    pub promotion_ref: String,
    pub lifecycle_transition_ref: String,
    pub committed_marker_path: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedCharterAuthorityV1 {
    pub canonical_bytes: Vec<u8>,
    pub canonical_fingerprint: String,
    pub promotion_ref: String,
    pub lifecycle_transition_ref: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CharterPromotionFaultPointV1 {
    AfterCanonicalInstalled,
    AfterRecordsInstalled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharterPromotionErrorKindV1 {
    UnsupportedPlatform,
    InvalidRequest,
    UnsafeFilesystem,
    BasisMismatch,
    LineageViolation,
    DurabilityViolation,
    Conflict,
    InjectedFault,
    IoFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterPromotionErrorV1 {
    kind: CharterPromotionErrorKindV1,
    detail: String,
}

impl CharterPromotionErrorV1 {
    fn new(kind: CharterPromotionErrorKindV1, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> CharterPromotionErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

/// Read-only observation of committed Charter authority for engine consumers.
///
/// Low-level canonical mutation is deliberately not part of the external crate
/// surface; callers must use `CharterPromotionWorkflowServiceV1`.
///
/// ```compile_fail
/// use handbook_engine::{
///     CharterAuthorityTransactionServiceV1, CharterPromotionFaultPointV1,
///     CharterPromotionRequestV1,
/// };
///
/// let service = CharterAuthorityTransactionServiceV1::new(".");
/// let request = CharterPromotionRequestV1 {
///     transaction_id: "forged".to_owned(),
///     canonical_bytes: Vec::new(),
///     promotion_record_bytes: Vec::new(),
///     lifecycle_transition_bytes: Vec::new(),
/// };
/// let _ = service.promote(request.clone());
/// let _ = service.promote_with_fault_for_testing(
///     request,
///     CharterPromotionFaultPointV1::AfterCanonicalInstalled,
/// );
/// ```
#[derive(Clone, Debug)]
pub struct CharterAuthorityTransactionServiceV1 {
    repo_root: PathBuf,
    lineage: TrustedLineageStoreV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RetainedAuthorityRecordV1 {
    pub(crate) label: String,
    pub(crate) relative_ref: String,
    pub(crate) bytes: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RetainedPromotionAuthorityV1 {
    pub(crate) canonical_basis_bytes: Option<Vec<u8>>,
    pub(crate) records: Vec<RetainedAuthorityRecordV1>,
}

pub(crate) struct RetainedAuthorityTransactionV1<'a> {
    service: &'a CharterAuthorityTransactionServiceV1,
    _locks: AuthorityLocks,
}

impl CharterAuthorityTransactionServiceV1 {
    pub fn new(repo_root: impl AsRef<Path>) -> Self {
        let repo_root = repo_root.as_ref().to_path_buf();
        Self {
            lineage: TrustedLineageStoreV1::new(&repo_root),
            repo_root,
        }
    }

    #[cfg(test)]
    fn promote(
        &self,
        request: CharterPromotionRequestV1,
    ) -> Result<CharterPromotionCommitV1, CharterPromotionErrorV1> {
        self.promote_inner(request, None)
    }

    #[cfg(test)]
    fn promote_with_fault_for_testing(
        &self,
        request: CharterPromotionRequestV1,
        fault: CharterPromotionFaultPointV1,
    ) -> Result<CharterPromotionCommitV1, CharterPromotionErrorV1> {
        self.promote_inner(request, Some(fault))
    }

    pub fn read_committed_charter(
        &self,
    ) -> Result<Option<CommittedCharterAuthorityV1>, CharterPromotionErrorV1> {
        ensure_supported_platform()?;
        let _locks = AuthorityLocks::acquire(&self.repo_root)?;
        self.recover_pending_locked()?;
        let Some(canonical_bytes) = read_current_canonical(&self.repo_root)? else {
            return Ok(None);
        };
        let fingerprint = DefinitionFingerprint::from_bytes(&canonical_bytes).to_string();
        let transaction_root = self.transaction_root();
        if !transaction_root.exists() {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "canonical authority exists without a committed promotion journal",
            ));
        }
        reject_reparse_or_symlink(&transaction_root, true).map_err(map_lineage)?;
        let committed = transaction_directories(&transaction_root, ".committed")?;
        for path in committed.into_iter().rev() {
            let intent = read_valid_intent(&path)?;
            if intent.record.canonical_fingerprint != fingerprint {
                continue;
            }
            validate_committed_marker(&path, &intent.raw_bytes)?;
            self.validate_final_records(&intent.record)?;
            return Ok(Some(CommittedCharterAuthorityV1 {
                canonical_bytes,
                canonical_fingerprint: fingerprint,
                promotion_ref: intent.record.promotion_ref,
                lifecycle_transition_ref: intent.record.lifecycle_transition_ref,
            }));
        }
        Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "canonical authority does not match any valid committed promotion journal",
        ))
    }

    pub(crate) fn begin_retained_authority(
        &self,
    ) -> Result<RetainedAuthorityTransactionV1<'_>, CharterPromotionErrorV1> {
        ensure_supported_platform()?;
        let locks = AuthorityLocks::acquire(&self.repo_root)?;
        self.recover_pending_locked()?;
        recover_complete_registry_approval_authority_locked(&self.repo_root).map_err(
            |failure| {
                error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    format!("registry/approval recovery refused: {}", failure.detail()),
                )
            },
        )?;
        CharterLifecycleStoreV1::new(&self.repo_root)
            .recover_retained_locked()
            .map_err(|failure| {
                error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    format!("lifecycle recovery refused: {}", failure.detail()),
                )
            })?;
        Ok(RetainedAuthorityTransactionV1 {
            service: self,
            _locks: locks,
        })
    }

    fn read_committed_charter_locked(
        &self,
    ) -> Result<Option<CommittedCharterAuthorityV1>, CharterPromotionErrorV1> {
        let Some(canonical_bytes) = read_current_canonical(&self.repo_root)? else {
            return Ok(None);
        };
        let fingerprint = DefinitionFingerprint::from_bytes(&canonical_bytes).to_string();
        let transaction_root = self.transaction_root();
        if !transaction_root.exists() {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "canonical authority exists without a committed promotion journal",
            ));
        }
        reject_reparse_or_symlink(&transaction_root, true).map_err(map_lineage)?;
        let committed = transaction_directories(&transaction_root, ".committed")?;
        for path in committed.into_iter().rev() {
            let intent = read_valid_intent(&path)?;
            if intent.record.canonical_fingerprint != fingerprint {
                continue;
            }
            validate_committed_marker(&path, &intent.raw_bytes)?;
            self.validate_final_records(&intent.record)?;
            return Ok(Some(CommittedCharterAuthorityV1 {
                canonical_bytes,
                canonical_fingerprint: fingerprint,
                promotion_ref: intent.record.promotion_ref,
                lifecycle_transition_ref: intent.record.lifecycle_transition_ref,
            }));
        }
        Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "canonical authority does not match any valid committed promotion journal",
        ))
    }

    #[cfg(test)]
    fn promote_inner(
        &self,
        request: CharterPromotionRequestV1,
        fault: Option<CharterPromotionFaultPointV1>,
    ) -> Result<CharterPromotionCommitV1, CharterPromotionErrorV1> {
        ensure_supported_platform()?;
        validate_transaction_id(&request.transaction_id)?;
        if request.canonical_bytes.is_empty() || request.canonical_bytes.len() > MAX_CANONICAL_BYTES
        {
            return Err(error(
                CharterPromotionErrorKindV1::InvalidRequest,
                "canonical artifact must be non-empty and no larger than 8 MiB",
            ));
        }
        let prepared = self.preflight(&request)?;
        let _locks = AuthorityLocks::acquire(&self.repo_root)?;
        self.recover_pending_locked()?;
        self.promote_prepared_locked(request, prepared, fault, None)
    }

    fn promote_prepared_locked(
        &self,
        request: CharterPromotionRequestV1,
        prepared: PreparedPromotion,
        fault: Option<CharterPromotionFaultPointV1>,
        retained: Option<&RetainedPromotionAuthorityV1>,
    ) -> Result<CharterPromotionCommitV1, CharterPromotionErrorV1> {
        let pending = self.pending_path(&request.transaction_id);
        let committed = self.committed_path(&request.transaction_id);
        if pending.exists() || committed.exists() {
            return Err(error(
                CharterPromotionErrorKindV1::Conflict,
                "promotion transaction ID already exists",
            ));
        }
        let old = read_current_canonical(&self.repo_root)?;
        verify_expected_basis(&prepared.expected_current_fingerprint, old.as_deref())?;
        create_safe_directories(&self.repo_root, &self.transaction_root()).map_err(map_lineage)?;
        fs::create_dir(&pending)
            .map_err(|_| io_error("promotion pending directory create failed"))?;
        sync_directory(&self.transaction_root()).map_err(map_lineage)?;

        let intent_record = PromotionIntentRecordV1 {
            schema_id: "handbook.promotion-transaction-intent".to_owned(),
            schema_version: "1.0".to_owned(),
            transaction_id: request.transaction_id.clone(),
            canonical_artifact_ref: CANONICAL_REF.to_owned(),
            canonical_fingerprint: prepared.canonical_fingerprint.clone(),
            expected_current_artifact_fingerprint: prepared.expected_current_fingerprint.clone(),
            had_old_canonical: old.is_some(),
            promotion_ref: prepared.promotion_ref.clone(),
            promotion_fingerprint: prepared.promotion_fingerprint.clone(),
            lifecycle_transition_ref: prepared.lifecycle_transition_ref.clone(),
            lifecycle_transition_fingerprint: prepared.lifecycle_transition_fingerprint.clone(),
        };
        let intent_bytes = serde_json_canonicalizer::to_vec(&intent_record)
            .map_err(|_| io_error("promotion intent canonicalization failed"))?;
        write_new_durable(&pending.join("intent.tmp"), &intent_bytes)?;
        rename_durable(
            &pending.join("intent.tmp"),
            &pending.join("intent.json"),
            &pending,
        )?;
        if let Some(old_bytes) = &old {
            write_new_durable(&pending.join("canonical.old"), old_bytes)?;
        }
        write_new_durable(&pending.join("canonical.new"), &request.canonical_bytes)?;
        write_new_durable(
            &pending.join("promotion-record.new"),
            &request.promotion_record_bytes,
        )?;
        write_new_durable(
            &pending.join("lifecycle-transition.new"),
            &request.lifecycle_transition_bytes,
        )?;
        sync_directory(&pending).map_err(map_lineage)?;

        let target = self.repo_root.join(CANONICAL_REF);
        let target_parent = target.parent().expect("fixed target has a parent");
        create_safe_directories(&self.repo_root, target_parent).map_err(map_lineage)?;
        rename_durable(&pending.join("canonical.new"), &target, target_parent)?;
        if fault == Some(CharterPromotionFaultPointV1::AfterCanonicalInstalled) {
            return Err(error(
                CharterPromotionErrorKindV1::InjectedFault,
                "injected fault after canonical install",
            ));
        }

        self.lineage
            .append_record(
                LineageRecordClassV1::Promotion,
                &request.promotion_record_bytes,
            )
            .map_err(map_lineage)?;
        self.lineage
            .append_record(
                LineageRecordClassV1::LifecycleTransition,
                &request.lifecycle_transition_bytes,
            )
            .map_err(map_lineage)?;
        if fault == Some(CharterPromotionFaultPointV1::AfterRecordsInstalled) {
            return Err(error(
                CharterPromotionErrorKindV1::InjectedFault,
                "injected fault after content-addressed record install",
            ));
        }

        if let Some(retained) = retained {
            if let Err(failure) =
                self.verify_retained_authority(&pending, &request.canonical_bytes, retained)
            {
                self.restore_old_canonical(&pending, &intent_record)?;
                cleanup_owned_pending(&pending)?;
                return Err(failure);
            }
        }
        self.commit_and_finalize(&pending, &committed, &intent_bytes)?;
        Ok(CharterPromotionCommitV1 {
            promotion_ref: prepared.promotion_ref,
            lifecycle_transition_ref: prepared.lifecycle_transition_ref,
            committed_marker_path: committed.join("committed"),
        })
    }

    fn verify_retained_authority(
        &self,
        pending: &Path,
        installed_canonical: &[u8],
        retained: &RetainedPromotionAuthorityV1,
    ) -> Result<(), CharterPromotionErrorV1> {
        let current = read_current_canonical(&self.repo_root)?;
        if current.as_deref() != Some(installed_canonical) {
            return Err(error(
                CharterPromotionErrorKindV1::Conflict,
                "installed canonical bytes changed before the authority commit marker",
            ));
        }
        match &retained.canonical_basis_bytes {
            Some(expected) => {
                if read_bounded_regular(&pending.join("canonical.old"), MAX_CANONICAL_BYTES)?
                    != *expected
                {
                    return Err(error(
                        CharterPromotionErrorKindV1::Conflict,
                        "retained canonical basis bytes changed before commit",
                    ));
                }
            }
            None if pending.join("canonical.old").exists() => {
                return Err(error(
                    CharterPromotionErrorKindV1::Conflict,
                    "create-only promotion unexpectedly retained prior canonical bytes",
                ));
            }
            None => {}
        }
        for record in &retained.records {
            let relative = CanonicalWorkspace::new(&self.repo_root)
                .normalize_repo_relative(&format!(".handbook/state/{}", record.relative_ref))
                .map_err(|_| {
                    error(
                        CharterPromotionErrorKindV1::UnsafeFilesystem,
                        format!("retained {} ref is not normalized", record.label),
                    )
                })?;
            let file = CanonicalWorkspace::new(&self.repo_root)
                .trusted_read_strict(&relative)
                .map_err(|_| {
                    error(
                        CharterPromotionErrorKindV1::Conflict,
                        format!("retained {} could not be re-resolved", record.label),
                    )
                })?;
            let (bytes, exceeded) = file
                .read_bytes_bounded(crate::MAX_LINEAGE_RECORD_BYTES)
                .map_err(|_| {
                    error(
                        CharterPromotionErrorKindV1::Conflict,
                        format!("retained {} could not be byte-compared", record.label),
                    )
                })?;
            if exceeded || bytes != record.bytes {
                return Err(error(
                    CharterPromotionErrorKindV1::Conflict,
                    format!("retained {} bytes changed before commit", record.label),
                ));
            }
        }
        Ok(())
    }

    fn preflight(
        &self,
        request: &CharterPromotionRequestV1,
    ) -> Result<PreparedPromotion, CharterPromotionErrorV1> {
        let promotion = validate_record(
            LineageRecordClassV1::Promotion,
            &request.promotion_record_bytes,
        )
        .map_err(map_lineage)?;
        let lifecycle = validate_record(
            LineageRecordClassV1::LifecycleTransition,
            &request.lifecycle_transition_bytes,
        )
        .map_err(map_lineage)?;
        self.lineage
            .preflight_record(
                LineageRecordClassV1::Promotion,
                &request.promotion_record_bytes,
            )
            .map_err(map_lineage)?;
        self.lineage
            .preflight_record(
                LineageRecordClassV1::LifecycleTransition,
                &request.lifecycle_transition_bytes,
            )
            .map_err(map_lineage)?;
        let canonical_fingerprint =
            DefinitionFingerprint::from_bytes(&request.canonical_bytes).to_string();
        require_equal_string(&promotion.value, "canonical_artifact_ref", CANONICAL_REF)?;
        require_equal_string(
            &promotion.value,
            "canonical_artifact_fingerprint",
            &canonical_fingerprint,
        )?;
        require_equal_string(&promotion.value, "decision", "approved")?;
        require_equal_string(&promotion.value, "target_instance_id", "project_authority")?;
        let expected_current_fingerprint =
            optional_fingerprint(&promotion.value, "expected_current_artifact_fingerprint")?;
        if promotion.value.get("basis_artifact_fingerprint")
            != promotion.value.get("expected_current_artifact_fingerprint")
        {
            return Err(error(
                CharterPromotionErrorKindV1::BasisMismatch,
                "promotion basis and expected current fingerprint differ",
            ));
        }

        let candidate_ref = string_field(&promotion.value, "candidate_ref").map_err(map_lineage)?;
        let candidate_fingerprint =
            string_field(&promotion.value, "candidate_fingerprint").map_err(map_lineage)?;
        let candidate = self
            .lineage
            .read_record_by_ref(
                LineageRecordClassV1::Candidate,
                candidate_ref,
                Some(candidate_fingerprint),
            )
            .map_err(map_lineage)?;
        let normalized_content_ref =
            string_field(&candidate, "normalized_content_ref").map_err(map_lineage)?;
        let retained_candidate_bytes = self
            .lineage
            .read_candidate_content(normalized_content_ref)
            .map_err(map_lineage)?;
        if retained_candidate_bytes != request.canonical_bytes {
            return Err(error(
                CharterPromotionErrorKindV1::LineageViolation,
                "staged canonical bytes differ from the retained normalized candidate bytes",
            ));
        }
        equal_json_field(&promotion.value, &candidate, "basis_artifact_fingerprint")?;
        equal_json_field(&promotion.value, &candidate, "profile_ref")?;
        equal_json_field(&promotion.value, &candidate, "resolved_profile_fingerprint")?;
        equal_json_field(&promotion.value, &candidate, "target_instance_id")?;

        let definition_registry = load_shipped_charter_definition_registry().map_err(|_| {
            error(
                CharterPromotionErrorKindV1::LineageViolation,
                "shipped Charter definition closure could not be reloaded",
            )
        })?;
        let decisions = resolve_shipped_profile_decisions(&self.repo_root).map_err(|_| {
            error(
                CharterPromotionErrorKindV1::LineageViolation,
                "selected shipped profile could not be re-resolved at promotion time",
            )
        })?;
        definition_registry
            .validate_selected_decisions(&decisions)
            .map_err(|_| {
                error(
                    CharterPromotionErrorKindV1::LineageViolation,
                    "selected decisions do not retain the shipped Charter definition closure",
                )
            })?;
        require_equal_string(
            &promotion.value,
            "profile_ref",
            decisions.profile_ref().as_str(),
        )?;
        require_equal_string(
            &promotion.value,
            "resolved_profile_fingerprint",
            decisions.profile_definition_fingerprint().as_str(),
        )?;
        self.validate_resolved_definitions(
            &promotion.value,
            &candidate,
            &decisions,
            &definition_registry,
        )?;
        let parsed_charter = parse_canonical_charter(&decisions, &request.canonical_bytes)
            .map_err(|_| {
                error(
                    CharterPromotionErrorKindV1::LineageViolation,
                    "canonical Charter bytes do not validate under the selected shipped profile",
                )
            })?;
        let reserialized =
            serialize_canonical_charter(&decisions, &parsed_charter).map_err(|_| {
                error(
                    CharterPromotionErrorKindV1::LineageViolation,
                    "canonical Charter could not be deterministically reserialized",
                )
            })?;
        if reserialized != request.canonical_bytes {
            return Err(error(
                CharterPromotionErrorKindV1::LineageViolation,
                "canonical Charter bytes differ from deterministic serializer output",
            ));
        }

        let registry_ref =
            string_field(&promotion.value, "approver_registry_state_ref").map_err(map_lineage)?;
        let registry_fp = string_field(&promotion.value, "approver_registry_state_fingerprint")
            .map_err(map_lineage)?;
        self.lineage
            .read_record_by_ref(
                LineageRecordClassV1::RegistryState,
                registry_ref,
                Some(registry_fp),
            )
            .map_err(map_lineage)?;
        let head_ref =
            string_field(&promotion.value, "registry_head_transition_ref").map_err(map_lineage)?;
        let head_fp = string_field(&promotion.value, "registry_head_transition_fingerprint")
            .map_err(map_lineage)?;
        self.lineage
            .read_record_by_ref(
                LineageRecordClassV1::RegistryTransition,
                head_ref,
                Some(head_fp),
            )
            .map_err(map_lineage)?;

        let approvals = promotion
            .value
            .get("approval_refs")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                error(
                    CharterPromotionErrorKindV1::LineageViolation,
                    "promotion approval_refs must be an array",
                )
            })?;
        if approvals.is_empty() {
            return Err(error(
                CharterPromotionErrorKindV1::LineageViolation,
                "promotion requires at least one retained approval",
            ));
        }
        for approval_ref in approvals {
            let approval_ref = approval_ref.as_str().ok_or_else(|| {
                error(
                    CharterPromotionErrorKindV1::LineageViolation,
                    "approval ref must be a string",
                )
            })?;
            let approval = self
                .lineage
                .read_record_by_ref(LineageRecordClassV1::Approval, approval_ref, None)
                .map_err(map_lineage)?;
            require_equal_string(&approval, "decision", "approved")?;
            require_equal_string(&approval, "candidate_ref", candidate_ref)?;
            require_equal_string(&approval, "candidate_fingerprint", candidate_fingerprint)?;
            equal_json_field(&promotion.value, &approval, "basis_artifact_fingerprint")?;
            equal_json_field(&promotion.value, &approval, "approver_registry_state_ref")?;
            equal_json_field(
                &promotion.value,
                &approval,
                "approver_registry_state_fingerprint",
            )?;
            equal_json_field(&promotion.value, &approval, "registry_head_transition_ref")?;
            equal_json_field(
                &promotion.value,
                &approval,
                "registry_head_transition_fingerprint",
            )?;
            let assertion_ref =
                string_field(&approval, "authenticator_assertion_ref").map_err(map_lineage)?;
            let assertion_fp = string_field(&approval, "authenticator_assertion_fingerprint")
                .map_err(map_lineage)?;
            self.lineage
                .read_record_by_ref(
                    LineageRecordClassV1::AuthenticatorAssertion,
                    assertion_ref,
                    Some(assertion_fp),
                )
                .map_err(map_lineage)?;
        }

        require_equal_string(&lifecycle.value, "target_instance_id", "project_authority")?;
        require_equal_string(&lifecycle.value, "result_state", "current")?;
        require_equal_string(
            &lifecycle.value,
            "clearance_promotion_ref",
            &promotion.relative_ref,
        )?;
        Ok(PreparedPromotion {
            canonical_fingerprint,
            expected_current_fingerprint,
            promotion_ref: promotion.relative_ref,
            promotion_fingerprint: promotion.fingerprint,
            lifecycle_transition_ref: lifecycle.relative_ref,
            lifecycle_transition_fingerprint: lifecycle.fingerprint,
        })
    }

    fn validate_resolved_definitions(
        &self,
        promotion: &Value,
        candidate: &Value,
        decisions: &crate::ResolvedProfileDecisions,
        definition_registry: &crate::CharterDefinitionRegistry,
    ) -> Result<(), CharterPromotionErrorV1> {
        let kind_ref = ExactDefinitionRef::parse(
            string_field(candidate, "target_kind_ref").map_err(map_lineage)?,
        )
        .map_err(|_| {
            error(
                CharterPromotionErrorKindV1::LineageViolation,
                "candidate target kind ref is invalid",
            )
        })?;
        let kind = decisions.registry().kind(&kind_ref).ok_or_else(|| {
            error(
                CharterPromotionErrorKindV1::LineageViolation,
                "candidate target kind is absent from the selected profile",
            )
        })?;
        let resolved = promotion
            .get("resolved_definitions")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                error(
                    CharterPromotionErrorKindV1::LineageViolation,
                    "promotion resolved_definitions must be an array",
                )
            })?;
        let matching_kind = resolved
            .iter()
            .filter(|binding| {
                binding.get("definition_ref").and_then(Value::as_str) == Some(kind_ref.as_str())
                    && binding
                        .get("definition_fingerprint")
                        .and_then(Value::as_str)
                        == Some(kind.definition_fingerprint().as_str())
            })
            .count();
        if matching_kind != 1 {
            return Err(error(
                CharterPromotionErrorKindV1::LineageViolation,
                "promotion must retain exactly one selected artifact-kind definition binding",
            ));
        }
        for binding in resolved {
            let reference = ExactDefinitionRef::parse(
                binding
                    .get("definition_ref")
                    .and_then(Value::as_str)
                    .ok_or_else(|| {
                        error(
                            CharterPromotionErrorKindV1::LineageViolation,
                            "resolved definition ref is absent",
                        )
                    })?,
            )
            .map_err(|_| {
                error(
                    CharterPromotionErrorKindV1::LineageViolation,
                    "resolved definition ref is invalid",
                )
            })?;
            let fingerprint = binding
                .get("definition_fingerprint")
                .and_then(Value::as_str)
                .ok_or_else(|| {
                    error(
                        CharterPromotionErrorKindV1::LineageViolation,
                        "resolved definition fingerprint is absent",
                    )
                })?;
            let valid = decisions
                .registry()
                .kind(&reference)
                .is_some_and(|record| record.definition_fingerprint().as_str() == fingerprint)
                || definition_registry
                    .record(&reference)
                    .is_some_and(|record| record.definition_fingerprint().as_str() == fingerprint);
            if !valid {
                return Err(error(
                    CharterPromotionErrorKindV1::LineageViolation,
                    "promotion resolved definition binding is not current package authority",
                ));
            }
        }
        let approval_policy = ExactDefinitionRef::parse(
            string_field(candidate, "required_approval_policy_ref").map_err(map_lineage)?,
        )
        .map_err(|_| {
            error(
                CharterPromotionErrorKindV1::LineageViolation,
                "candidate approval policy ref is invalid",
            )
        })?;
        if definition_registry.record(&approval_policy).is_none() {
            return Err(error(
                CharterPromotionErrorKindV1::LineageViolation,
                "candidate approval policy is outside the shipped closure",
            ));
        }
        Ok(())
    }

    fn recover_pending_locked(&self) -> Result<(), CharterPromotionErrorV1> {
        let root = self.transaction_root();
        if !root.exists() {
            return Ok(());
        }
        reject_reparse_or_symlink(&root, true).map_err(map_lineage)?;
        for pending in transaction_directories(&root, ".pending")? {
            self.recover_one_pending(&pending)?;
        }
        Ok(())
    }

    fn recover_one_pending(&self, pending: &Path) -> Result<(), CharterPromotionErrorV1> {
        reject_reparse_or_symlink(pending, true).map_err(map_lineage)?;
        if !pending.join("intent.json").exists() {
            return cleanup_owned_pending(pending);
        }
        let intent = read_valid_intent(pending)?;
        let current = read_current_canonical(&self.repo_root)?;
        let current_fingerprint = current
            .as_deref()
            .map(|bytes| DefinitionFingerprint::from_bytes(bytes).to_string());
        let new_is_installed =
            current_fingerprint.as_deref() == Some(intent.record.canonical_fingerprint.as_str());
        let promotion_ok = self
            .lineage
            .read_record(
                LineageRecordClassV1::Promotion,
                &intent.record.promotion_ref,
                &intent.record.promotion_fingerprint,
            )
            .is_ok();
        let lifecycle_ok = self
            .lineage
            .read_record(
                LineageRecordClassV1::LifecycleTransition,
                &intent.record.lifecycle_transition_ref,
                &intent.record.lifecycle_transition_fingerprint,
            )
            .is_ok();
        if pending.join("committed").exists() {
            validate_committed_marker(pending, &intent.raw_bytes)?;
            if !new_is_installed || !promotion_ok || !lifecycle_ok {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "committed promotion journal does not match installed authority",
                ));
            }
            return self.finalize_pending(pending, &intent.record.transaction_id);
        }
        if new_is_installed && promotion_ok && lifecycle_ok {
            self.preflight_staged_or_final(&intent.record, pending)?;
            let committed = self.committed_path(&intent.record.transaction_id);
            return self.commit_and_finalize(pending, &committed, &intent.raw_bytes);
        }
        if new_is_installed {
            self.restore_old_canonical(pending, &intent.record)?;
        } else if !matches_expected_current(
            &intent.record.expected_current_artifact_fingerprint,
            current.as_deref(),
        ) {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "pending promotion encountered unrelated canonical bytes",
            ));
        }
        cleanup_owned_pending(pending)
    }

    fn preflight_staged_or_final(
        &self,
        intent: &PromotionIntentRecordV1,
        pending: &Path,
    ) -> Result<(), CharterPromotionErrorV1> {
        let promotion = read_bounded_regular(
            &pending.join("promotion-record.new"),
            MAX_INTENT_BYTES.max(1024 * 1024),
        )?;
        let lifecycle = read_bounded_regular(
            &pending.join("lifecycle-transition.new"),
            MAX_INTENT_BYTES.max(1024 * 1024),
        )?;
        let promotion_identity =
            validate_record(LineageRecordClassV1::Promotion, &promotion).map_err(map_lineage)?;
        let lifecycle_identity =
            validate_record(LineageRecordClassV1::LifecycleTransition, &lifecycle)
                .map_err(map_lineage)?;
        if promotion_identity.relative_ref != intent.promotion_ref
            || lifecycle_identity.relative_ref != intent.lifecycle_transition_ref
        {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "staged records disagree with durable promotion intent",
            ));
        }
        self.validate_final_records(intent)
    }

    fn validate_final_records(
        &self,
        intent: &PromotionIntentRecordV1,
    ) -> Result<(), CharterPromotionErrorV1> {
        self.lineage
            .read_record(
                LineageRecordClassV1::Promotion,
                &intent.promotion_ref,
                &intent.promotion_fingerprint,
            )
            .map_err(map_lineage)?;
        self.lineage
            .read_record(
                LineageRecordClassV1::LifecycleTransition,
                &intent.lifecycle_transition_ref,
                &intent.lifecycle_transition_fingerprint,
            )
            .map_err(map_lineage)?;
        Ok(())
    }

    fn restore_old_canonical(
        &self,
        pending: &Path,
        intent: &PromotionIntentRecordV1,
    ) -> Result<(), CharterPromotionErrorV1> {
        let target = self.repo_root.join(CANONICAL_REF);
        let parent = target.parent().expect("fixed target has parent");
        if intent.had_old_canonical {
            let old = pending.join("canonical.old");
            if !old.exists() {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "rollback bytes are missing",
                ));
            }
            rename_durable(&old, &target, parent)
        } else {
            reject_reparse_or_symlink(&target, false).map_err(map_lineage)?;
            fs::remove_file(&target)
                .map_err(|_| io_error("uncommitted canonical rollback failed"))?;
            sync_directory(parent).map_err(map_lineage)
        }
    }

    fn commit_and_finalize(
        &self,
        pending: &Path,
        committed: &Path,
        intent_bytes: &[u8],
    ) -> Result<(), CharterPromotionErrorV1> {
        write_new_durable(
            &pending.join("committed.tmp"),
            &charter_promotion_commit_marker(intent_bytes),
        )?;
        rename_durable(
            &pending.join("committed.tmp"),
            &pending.join("committed"),
            pending,
        )?;
        fs::rename(pending, committed)
            .map_err(|_| io_error("promotion journal finalization failed"))?;
        sync_directory(&self.transaction_root()).map_err(map_lineage)
    }

    fn finalize_pending(
        &self,
        pending: &Path,
        transaction_id: &str,
    ) -> Result<(), CharterPromotionErrorV1> {
        let committed = self.committed_path(transaction_id);
        if committed.exists() {
            return Err(error(
                CharterPromotionErrorKindV1::Conflict,
                "committed promotion journal already exists",
            ));
        }
        fs::rename(pending, &committed)
            .map_err(|_| io_error("promotion journal recovery finalization failed"))?;
        sync_directory(&self.transaction_root()).map_err(map_lineage)
    }

    fn transaction_root(&self) -> PathBuf {
        self.repo_root
            .join(".handbook/state/transactions/promotions")
    }

    fn pending_path(&self, transaction_id: &str) -> PathBuf {
        self.transaction_root()
            .join(format!("{transaction_id}.pending"))
    }

    fn committed_path(&self, transaction_id: &str) -> PathBuf {
        self.transaction_root()
            .join(format!("{transaction_id}.committed"))
    }
}

impl RetainedAuthorityTransactionV1<'_> {
    pub(crate) fn read_committed_charter(
        &self,
    ) -> Result<Option<CommittedCharterAuthorityV1>, CharterPromotionErrorV1> {
        self.service.read_committed_charter_locked()
    }

    pub(crate) fn promote_retained(
        self,
        request: CharterPromotionRequestV1,
        retained: &RetainedPromotionAuthorityV1,
    ) -> Result<CharterPromotionCommitV1, CharterPromotionErrorV1> {
        validate_transaction_id(&request.transaction_id)?;
        if request.canonical_bytes.is_empty() || request.canonical_bytes.len() > MAX_CANONICAL_BYTES
        {
            return Err(error(
                CharterPromotionErrorKindV1::InvalidRequest,
                "canonical artifact must be non-empty and no larger than 8 MiB",
            ));
        }
        let prepared = self.service.preflight(&request)?;
        self.service
            .promote_prepared_locked(request, prepared, None, Some(retained))
    }
}

#[derive(Debug)]
struct PreparedPromotion {
    canonical_fingerprint: String,
    expected_current_fingerprint: Option<String>,
    promotion_ref: String,
    promotion_fingerprint: String,
    lifecycle_transition_ref: String,
    lifecycle_transition_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PromotionIntentRecordV1 {
    schema_id: String,
    schema_version: String,
    transaction_id: String,
    canonical_artifact_ref: String,
    canonical_fingerprint: String,
    expected_current_artifact_fingerprint: Option<String>,
    had_old_canonical: bool,
    promotion_ref: String,
    promotion_fingerprint: String,
    lifecycle_transition_ref: String,
    lifecycle_transition_fingerprint: String,
}

struct ReadIntent {
    record: PromotionIntentRecordV1,
    raw_bytes: Vec<u8>,
}

struct AuthorityLocks {
    files: Vec<File>,
}

impl AuthorityLocks {
    fn acquire(repo_root: &Path) -> Result<Self, CharterPromotionErrorV1> {
        let lock_root = repo_root.join(".handbook/state/locks");
        create_safe_directories(repo_root, &lock_root).map_err(map_lineage)?;
        let mut files = Vec::new();
        for name in ["promotion.lock", "registry.lock", "lifecycle.lock"] {
            let file = open_lock_file(&lock_root.join(name))?;
            file.lock()
                .map_err(|_| io_error("authority lock acquisition failed"))?;
            files.push(file);
        }
        Ok(Self { files })
    }
}

impl Drop for AuthorityLocks {
    fn drop(&mut self) {
        for file in self.files.iter().rev() {
            let _ = file.unlock();
        }
    }
}

pub(crate) fn charter_promotion_commit_marker(raw_intent_bytes: &[u8]) -> Vec<u8> {
    let digest = Sha256::digest(raw_intent_bytes);
    format!("sha256:{digest:x}\n").into_bytes()
}

fn read_valid_intent(directory: &Path) -> Result<ReadIntent, CharterPromotionErrorV1> {
    let raw_bytes = read_bounded_regular(&directory.join("intent.json"), MAX_INTENT_BYTES)?;
    let record: PromotionIntentRecordV1 = serde_json::from_slice(&raw_bytes).map_err(|_| {
        error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "promotion intent is not closed JSON",
        )
    })?;
    let canonical = serde_json_canonicalizer::to_vec(&record)
        .map_err(|_| io_error("promotion intent canonicalization failed"))?;
    if canonical != raw_bytes
        || record.schema_id != "handbook.promotion-transaction-intent"
        || record.schema_version != "1.0"
        || record.canonical_artifact_ref != CANONICAL_REF
    {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "promotion intent identity or canonical bytes are invalid",
        ));
    }
    validate_transaction_id(&record.transaction_id)?;
    Ok(ReadIntent { record, raw_bytes })
}

fn transaction_directories(
    root: &Path,
    suffix: &str,
) -> Result<Vec<PathBuf>, CharterPromotionErrorV1> {
    let mut paths = Vec::new();
    for entry in
        fs::read_dir(root).map_err(|_| io_error("promotion transaction directory read failed"))?
    {
        let entry = entry.map_err(|_| io_error("promotion transaction entry read failed"))?;
        if !entry.file_name().to_string_lossy().ends_with(suffix) {
            continue;
        }
        let path = entry.path();
        reject_reparse_or_symlink(&path, true).map_err(map_lineage)?;
        paths.push(path);
    }
    paths.sort();
    Ok(paths)
}

fn validate_committed_marker(
    directory: &Path,
    intent_bytes: &[u8],
) -> Result<(), CharterPromotionErrorV1> {
    let observed = read_bounded_regular(&directory.join("committed"), 72)?;
    if observed != charter_promotion_commit_marker(intent_bytes) {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "promotion commit marker does not bind the exact raw intent bytes",
        ));
    }
    Ok(())
}

fn validate_transaction_id(transaction_id: &str) -> Result<(), CharterPromotionErrorV1> {
    if transaction_id.is_empty()
        || transaction_id.len() > 128
        || !transaction_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(error(
            CharterPromotionErrorKindV1::InvalidRequest,
            "transaction ID must be 1-128 ASCII alphanumeric, dash, or underscore bytes",
        ));
    }
    Ok(())
}

fn optional_fingerprint(
    value: &Value,
    field: &str,
) -> Result<Option<String>, CharterPromotionErrorV1> {
    match value.get(field) {
        Some(Value::Null) => Ok(None),
        Some(Value::String(raw)) => DefinitionFingerprint::parse(raw)
            .map(|fingerprint| Some(fingerprint.to_string()))
            .map_err(|_| {
                error(
                    CharterPromotionErrorKindV1::InvalidRequest,
                    "expected-current fingerprint is invalid",
                )
            }),
        _ => Err(error(
            CharterPromotionErrorKindV1::InvalidRequest,
            "expected-current fingerprint must be a lowercase SHA-256 string or null",
        )),
    }
}

fn require_equal_string(
    value: &Value,
    field: &str,
    expected: &str,
) -> Result<(), CharterPromotionErrorV1> {
    let observed = string_field(value, field).map_err(map_lineage)?;
    if observed != expected {
        return Err(error(CharterPromotionErrorKindV1::LineageViolation, format!("`{field}` value `{observed}` does not equal required transaction binding `{expected}`")));
    }
    Ok(())
}

fn equal_json_field(
    left: &Value,
    right: &Value,
    field: &str,
) -> Result<(), CharterPromotionErrorV1> {
    if left.get(field) != right.get(field) {
        return Err(error(
            CharterPromotionErrorKindV1::LineageViolation,
            format!("retained lineage field `{field}` differs"),
        ));
    }
    Ok(())
}

fn verify_expected_basis(
    expected: &Option<String>,
    current: Option<&[u8]>,
) -> Result<(), CharterPromotionErrorV1> {
    if !matches_expected_current(expected, current) {
        return Err(error(
            CharterPromotionErrorKindV1::BasisMismatch,
            "current canonical authority does not equal expected-current basis",
        ));
    }
    Ok(())
}

fn matches_expected_current(expected: &Option<String>, current: Option<&[u8]>) -> bool {
    match (expected, current) {
        (None, None) => true,
        (Some(expected), Some(bytes)) => {
            DefinitionFingerprint::from_bytes(bytes).to_string() == *expected
        }
        _ => false,
    }
}

fn read_current_canonical(repo_root: &Path) -> Result<Option<Vec<u8>>, CharterPromotionErrorV1> {
    let workspace = CanonicalWorkspace::new(repo_root);
    let relative = workspace
        .normalize_repo_relative(CANONICAL_REF)
        .map_err(|_| {
            error(
                CharterPromotionErrorKindV1::UnsafeFilesystem,
                "canonical path is invalid",
            )
        })?;
    match workspace.trusted_read_strict(&relative) {
        Ok(file) => {
            let (bytes, exceeded) = file
                .read_bytes_bounded(MAX_CANONICAL_BYTES)
                .map_err(|_| io_error("canonical authority read failed"))?;
            if exceeded {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "canonical authority exceeds 8 MiB",
                ));
            }
            Ok(Some(bytes))
        }
        Err(RepoRelativeFileAccessError::Missing(_)) => Ok(None),
        Err(_) => Err(error(
            CharterPromotionErrorKindV1::UnsafeFilesystem,
            "canonical authority is not a safe regular file",
        )),
    }
}

fn write_new_durable(path: &Path, bytes: &[u8]) -> Result<(), CharterPromotionErrorV1> {
    let mut file =
        create_new_file(path).map_err(|_| io_error("durable staged file create-new failed"))?;
    file.write_all(bytes)
        .map_err(|_| io_error("durable staged file write failed"))?;
    file.sync_all()
        .map_err(|_| io_error("durable staged file flush failed"))?;
    let parent = path.parent().ok_or_else(|| {
        error(
            CharterPromotionErrorKindV1::UnsafeFilesystem,
            "staged file has no parent",
        )
    })?;
    sync_directory(parent).map_err(map_lineage)
}

fn rename_durable(
    source: &Path,
    target: &Path,
    parent: &Path,
) -> Result<(), CharterPromotionErrorV1> {
    // Rust maps same-volume replacement to native rename primitives. On Windows this is
    // MoveFileEx/SetFileInformationByHandle; the following directory flush makes the name durable.
    // Source: https://doc.rust-lang.org/std/fs/fn.rename.html
    fs::rename(source, target).map_err(|_| io_error("durable rename failed"))?;
    sync_directory(parent).map_err(map_lineage)
}

fn open_lock_file(path: &Path) -> Result<File, CharterPromotionErrorV1> {
    match create_new_file(path) {
        Ok(file) => return Ok(file),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(_) => return Err(io_error("authority lock file create failed")),
    }
    reject_reparse_or_symlink(path, false).map_err(map_lineage)?;
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
        .map_err(|_| io_error("authority lock file open failed"))?;
    if !file
        .metadata()
        .map_err(|_| io_error("authority lock metadata failed"))?
        .is_file()
    {
        return Err(error(
            CharterPromotionErrorKindV1::UnsafeFilesystem,
            "authority lock is not a regular file",
        ));
    }
    Ok(file)
}

fn read_bounded_regular(path: &Path, limit: usize) -> Result<Vec<u8>, CharterPromotionErrorV1> {
    reject_reparse_or_symlink(path, false).map_err(map_lineage)?;
    let metadata = fs::metadata(path).map_err(|_| io_error("journal file metadata failed"))?;
    if !metadata.is_file() || metadata.len() > limit as u64 {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "journal file is not a bounded regular file",
        ));
    }
    let bytes = fs::read(path).map_err(|_| io_error("journal file read failed"))?;
    if bytes.len() > limit {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "journal file exceeds its bound",
        ));
    }
    Ok(bytes)
}

fn cleanup_owned_pending(pending: &Path) -> Result<(), CharterPromotionErrorV1> {
    let allowed = [
        "intent.tmp",
        "intent.json",
        "canonical.old",
        "canonical.new",
        "promotion-record.new",
        "lifecycle-transition.new",
        "committed.tmp",
    ];
    for entry in
        fs::read_dir(pending).map_err(|_| io_error("pending journal cleanup read failed"))?
    {
        let entry = entry.map_err(|_| io_error("pending journal cleanup entry failed"))?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "pending journal contains a non-UTF-8 entry",
            ));
        };
        if !allowed.contains(&name) {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "pending journal contains an unowned entry",
            ));
        }
        let metadata = fs::symlink_metadata(entry.path())
            .map_err(|_| io_error("pending cleanup metadata failed"))?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(error(
                CharterPromotionErrorKindV1::UnsafeFilesystem,
                "pending cleanup target is not an owned regular file",
            ));
        }
        fs::remove_file(entry.path())
            .map_err(|_| io_error("pending journal owned-file cleanup failed"))?;
    }
    fs::remove_dir(pending).map_err(|_| io_error("pending journal directory cleanup failed"))?;
    let parent = pending.parent().ok_or_else(|| {
        error(
            CharterPromotionErrorKindV1::UnsafeFilesystem,
            "pending journal has no parent",
        )
    })?;
    sync_directory(parent).map_err(map_lineage)
}

fn ensure_supported_platform() -> Result<(), CharterPromotionErrorV1> {
    if cfg!(any(unix, windows)) {
        Ok(())
    } else {
        Err(error(CharterPromotionErrorKindV1::UnsupportedPlatform, "promotion requires native strict reads, file locks, same-volume rename, and durable directory flush"))
    }
}

fn map_lineage(source: LineageStoreErrorV1) -> CharterPromotionErrorV1 {
    error(
        CharterPromotionErrorKindV1::LineageViolation,
        source.detail(),
    )
}

fn io_error(detail: impl Into<String>) -> CharterPromotionErrorV1 {
    error(CharterPromotionErrorKindV1::IoFailure, detail)
}

fn error(kind: CharterPromotionErrorKindV1, detail: impl Into<String>) -> CharterPromotionErrorV1 {
    CharterPromotionErrorV1::new(kind, detail)
}

#[allow(dead_code)]
fn normalized_relative(path: &Path) -> bool {
    path.components()
        .all(|component| matches!(component, Component::Normal(_)))
}

#[cfg(test)]
mod retained_authority_tests {
    use super::*;
    use crate::charter_authority_workflow::RegistryAuthorityLocks;
    use crate::charter_lifecycle_store::CharterLifecycleStoreV1;
    use std::sync::{mpsc, Arc, Barrier};
    use std::time::Duration;

    #[test]
    fn retained_guard_blocks_registry_and_standalone_lifecycle_until_reverse_drop() {
        let repo = tempfile::tempdir().unwrap();
        let service = CharterAuthorityTransactionServiceV1::new(repo.path());
        let guard = service.begin_retained_authority().unwrap();
        let start = Arc::new(Barrier::new(3));
        let (completed_tx, completed_rx) = mpsc::channel();

        let registry_root = repo.path().to_path_buf();
        let registry_start = Arc::clone(&start);
        let registry_completed = completed_tx.clone();
        let registry = std::thread::spawn(move || {
            registry_start.wait();
            let held = RegistryAuthorityLocks::acquire(&registry_root)
                .unwrap_or_else(|_| panic!("registry locks after retained guard release"));
            registry_completed.send("registry").unwrap();
            drop(held);
        });

        let lifecycle_root = repo.path().to_path_buf();
        let lifecycle_start = Arc::clone(&start);
        let lifecycle_completed = completed_tx;
        let lifecycle = std::thread::spawn(move || {
            lifecycle_start.wait();
            CharterLifecycleStoreV1::new(lifecycle_root)
                .observe()
                .unwrap();
            lifecycle_completed.send("lifecycle").unwrap();
        });

        start.wait();
        assert!(completed_rx
            .recv_timeout(Duration::from_millis(100))
            .is_err());
        drop(guard);
        let mut completed = vec![
            completed_rx.recv_timeout(Duration::from_secs(2)).unwrap(),
            completed_rx.recv_timeout(Duration::from_secs(2)).unwrap(),
        ];
        completed.sort_unstable();
        assert_eq!(completed, ["lifecycle", "registry"]);
        registry.join().unwrap();
        lifecycle.join().unwrap();
    }

    #[test]
    fn every_retained_authority_byte_class_refuses_stale_bytes() {
        let repo = tempfile::tempdir().unwrap();
        let state = repo.path().join(".handbook/state");
        fs::create_dir_all(&state).unwrap();
        let mut records = Vec::new();
        for label in [
            "registry state",
            "registry head",
            "approval",
            "lifecycle authority",
        ] {
            let relative_ref = format!("retained/{}.json", label.replace(' ', "-"));
            let path = state.join(&relative_ref);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, label.as_bytes()).unwrap();
            records.push(RetainedAuthorityRecordV1 {
                label: label.to_owned(),
                relative_ref,
                bytes: label.as_bytes().to_vec(),
            });
        }
        let pending = repo.path().join("pending");
        fs::create_dir(&pending).unwrap();
        fs::write(pending.join("canonical.old"), b"old canonical").unwrap();
        fs::create_dir_all(repo.path().join(".handbook/project")).unwrap();
        fs::write(repo.path().join(CANONICAL_REF), b"new canonical").unwrap();
        let retained = RetainedPromotionAuthorityV1 {
            canonical_basis_bytes: Some(b"old canonical".to_vec()),
            records,
        };
        let service = CharterAuthorityTransactionServiceV1::new(repo.path());
        service
            .verify_retained_authority(&pending, b"new canonical", &retained)
            .unwrap();

        for record in &retained.records {
            let path = state.join(&record.relative_ref);
            fs::write(&path, b"stale").unwrap();
            let error = service
                .verify_retained_authority(&pending, b"new canonical", &retained)
                .unwrap_err();
            assert_eq!(error.kind(), CharterPromotionErrorKindV1::Conflict);
            fs::write(&path, &record.bytes).unwrap();
        }
        fs::write(pending.join("canonical.old"), b"stale").unwrap();
        assert_eq!(
            service
                .verify_retained_authority(&pending, b"new canonical", &retained)
                .unwrap_err()
                .kind(),
            CharterPromotionErrorKindV1::Conflict
        );
    }
}

#[cfg(test)]
#[path = "charter_authority_transaction_tests.rs"]
mod charter_authority_transaction_tests;
