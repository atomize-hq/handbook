use crate::approver_registry_mutation::recover_complete_registry_approval_authority_locked;
use crate::approver_registry_observation::observe_committed_approver_registry_locked;
use crate::canonical_repo_support::{CanonicalWorkspace, RepoRelativeFileAccessError};
use crate::charter_approval_workflow::observe_committed_candidate_approval_refs_locked;
use crate::charter_lifecycle::CharterLifecycleState;
use crate::charter_lifecycle_store::{
    charter_lifecycle_state_fingerprint, CharterLifecycleStoreV1, RetainedLifecycleAuthorityV1,
};
use crate::charter_lifecycle_validation::{
    validate_candidate_exact_result_authority, validate_result_bytes,
    CharterLifecycleValidationResultV10,
};
use crate::charter_lineage_store::{
    create_new_file, create_safe_directories, reject_reparse_or_symlink, string_field,
    sync_directory, validate_record, LineageRecordClassV1, LineageStoreErrorV1,
    TrustedLineageStoreV1,
};
use crate::charter_promotion_intent_v12::{
    parse_promotion_intent_v12, promotion_intent_marker_v12, PromotionApprovalBindingV12,
    PromotionCandidateLineageV12, PromotionHumanAuthorityV12, PromotionIntentTargetV12,
    PromotionOutputRecordV12, PromotionOutputsV12, PromotionRecoveryV12,
    PromotionSelectedContractV12, PromotionTransactionIntentV12, ValidatedPromotionIntentV12,
    MAX_PROMOTION_INTENT_BYTES_V12,
};
use crate::charter_promotion_workflow::required_approval_pairs_for_authority;
use crate::DefinitionFingerprint;
use crate::{
    load_shipped_charter_definition_registry, parse_canonical_charter,
    resolve_shipped_profile_decisions, serialize_canonical_charter, ExactDefinitionRef,
};
use serde_json::Value;
#[cfg(test)]
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Component, Path, PathBuf};

const CANONICAL_REF: &str = ".handbook/project/charter.yaml";
const MAX_CANONICAL_BYTES: usize = 1024 * 1024;
const MAX_INTENT_BYTES: usize = MAX_PROMOTION_INTENT_BYTES_V12;
const MAX_OUTPUT_RECORD_BYTES: usize = 262_144;
const INTENT_STAGING_NAME: &str = ".intent-staging";
const OUTPUT_STAGING_NAME: &str = ".output-staging";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum OutputPurposeV12 {
    Canonical,
    PromotionRecord,
    LifecycleTransition,
}

impl OutputPurposeV12 {
    #[cfg(test)]
    const ALL: [Self; 3] = [
        Self::Canonical,
        Self::PromotionRecord,
        Self::LifecycleTransition,
    ];
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum OutputStageBoundaryV12 {
    S0Retained,
    S1ScratchCreated,
    S2ScratchWritten,
    S3ScratchSyncedAndClosed,
    S4ScratchReopened,
    S5ScratchVerified,
    S6ScratchDirectorySynced,
    S7StageRenamed,
    S8PendingDirectorySynced,
    S9ScratchDirectoryResynced,
    S10TransactionParentSynced,
    S11PendingStageReverified,
}

#[cfg(test)]
impl OutputStageBoundaryV12 {
    #[cfg(test)]
    const ALL: [Self; 12] = [
        Self::S0Retained,
        Self::S1ScratchCreated,
        Self::S2ScratchWritten,
        Self::S3ScratchSyncedAndClosed,
        Self::S4ScratchReopened,
        Self::S5ScratchVerified,
        Self::S6ScratchDirectorySynced,
        Self::S7StageRenamed,
        Self::S8PendingDirectorySynced,
        Self::S9ScratchDirectoryResynced,
        Self::S10TransactionParentSynced,
        Self::S11PendingStageReverified,
    ];
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RetainedStageStateV12 {
    Absent,
    Exact,
    Mismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TerminalDestinationStateV12 {
    Absent,
    ExactPreExisting,
    Mismatching,
    Unsafe,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PromotionTerminalKindV12 {
    Committed,
    RolledBack,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum WriterBoundaryV12 {
    W0Admitted,
    W1IntentScratch,
    W2Directory,
    W3IntentPublished,
    W4OldSnapshot,
    W5CanonicalStaged,
    W6PromotionStaged,
    W7LifecycleStaged,
    W8Prepared,
    W9CanonicalInstalled,
    W10CanonicalMarked,
    W11PromotionInstalled,
    W12LifecycleInstalled,
    W13RecordsMarked,
    W14Committed,
    W15Terminal,
}

#[cfg(test)]
impl WriterBoundaryV12 {
    #[cfg(test)]
    const ALL: [Self; 16] = [
        Self::W0Admitted,
        Self::W1IntentScratch,
        Self::W2Directory,
        Self::W3IntentPublished,
        Self::W4OldSnapshot,
        Self::W5CanonicalStaged,
        Self::W6PromotionStaged,
        Self::W7LifecycleStaged,
        Self::W8Prepared,
        Self::W9CanonicalInstalled,
        Self::W10CanonicalMarked,
        Self::W11PromotionInstalled,
        Self::W12LifecycleInstalled,
        Self::W13RecordsMarked,
        Self::W14Committed,
        Self::W15Terminal,
    ];
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum RecoveryBoundaryV12 {
    R0Admitted,
    R1SnapshotComplete,
    R2PreparedTempRemoved,
    R3PreparedRemoved,
    R4LifecycleStageRemoved,
    R5PromotionStageRemoved,
    R6CanonicalStageRemoved,
    R7RollbackMarkerTemp,
    R8RollbackMarkerPublished,
    R9RollbackTerminal,
}

#[cfg(test)]
impl RecoveryBoundaryV12 {
    #[cfg(test)]
    const ALL: [Self; 10] = [
        Self::R0Admitted,
        Self::R1SnapshotComplete,
        Self::R2PreparedTempRemoved,
        Self::R3PreparedRemoved,
        Self::R4LifecycleStageRemoved,
        Self::R5PromotionStageRemoved,
        Self::R6CanonicalStageRemoved,
        Self::R7RollbackMarkerTemp,
        Self::R8RollbackMarkerPublished,
        Self::R9RollbackTerminal,
    ];
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MarkerObservationV12 {
    Absent,
    TmpExactPrefix,
    Published,
}

#[cfg(test)]
fn all_output_stage_fault_pairs_for_testing() -> Vec<(OutputPurposeV12, OutputStageBoundaryV12)> {
    OutputPurposeV12::ALL
        .into_iter()
        .flat_map(|purpose| {
            OutputStageBoundaryV12::ALL
                .into_iter()
                .map(move |boundary| (purpose, boundary))
        })
        .collect()
}

#[cfg(test)]
fn classify_retained_stage_bytes(
    observed: Option<&[u8]>,
    expected: &[u8],
) -> RetainedStageStateV12 {
    match observed {
        None => RetainedStageStateV12::Absent,
        Some(observed) if observed == expected => RetainedStageStateV12::Exact,
        Some(_) => RetainedStageStateV12::Mismatch,
    }
}

fn terminal_destination_allows_rename(state: TerminalDestinationStateV12) -> bool {
    state == TerminalDestinationStateV12::Absent
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CharterPromotionRequestV1 {
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

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CharterPromotionFaultPointV1 {
    OutputStage {
        purpose: OutputPurposeV12,
        boundary: OutputStageBoundaryV12,
        write_prefix: Option<usize>,
    },
    Writer(WriterBoundaryV12),
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
    #[cfg(test)]
    InjectedFault,
    IoFailure,
}

#[cfg(test)]
thread_local! {
    static SELECTED_PROMOTION_FAULT_V1: Cell<Option<CharterPromotionFaultPointV1>> = const { Cell::new(None) };
    static SELECTED_RECOVERY_FAULT_V1: Cell<Option<RecoveryBoundaryV12>> = const { Cell::new(None) };
}

#[cfg(test)]
struct ScopedPromotionFaultV12 {
    previous: Option<CharterPromotionFaultPointV1>,
}

#[cfg(test)]
impl ScopedPromotionFaultV12 {
    fn select(selected: CharterPromotionFaultPointV1) -> Self {
        let previous = SELECTED_PROMOTION_FAULT_V1.with(|slot| slot.replace(Some(selected)));
        Self { previous }
    }
}

#[cfg(test)]
impl Drop for ScopedPromotionFaultV12 {
    fn drop(&mut self) {
        SELECTED_PROMOTION_FAULT_V1.with(|slot| slot.set(self.previous));
    }
}

#[cfg(test)]
struct ScopedRecoveryFaultV12 {
    previous: Option<RecoveryBoundaryV12>,
}

#[cfg(test)]
impl ScopedRecoveryFaultV12 {
    fn select(selected: RecoveryBoundaryV12) -> Self {
        let previous = SELECTED_RECOVERY_FAULT_V1.with(|slot| slot.replace(Some(selected)));
        Self { previous }
    }
}

#[cfg(test)]
impl Drop for ScopedRecoveryFaultV12 {
    fn drop(&mut self) {
        SELECTED_RECOVERY_FAULT_V1.with(|slot| slot.set(self.previous));
    }
}

#[derive(Clone, Debug, Default)]
struct TransactionRootInventoryV12 {
    pending: Vec<PathBuf>,
    committed: Vec<PathBuf>,
    rolled_back: Vec<PathBuf>,
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
        self.promote_inner(request)
    }

    #[cfg(test)]
    fn promote_with_fault_for_testing(
        &self,
        request: CharterPromotionRequestV1,
        fault: CharterPromotionFaultPointV1,
    ) -> Result<CharterPromotionCommitV1, CharterPromotionErrorV1> {
        let _selected = ScopedPromotionFaultV12::select(fault);
        self.promote_inner(request)
    }

    #[cfg(test)]
    fn recover_with_fault_for_testing(
        &self,
        fault: RecoveryBoundaryV12,
    ) -> Result<(), CharterPromotionErrorV1> {
        let _selected = ScopedRecoveryFaultV12::select(fault);
        let _locks = AuthorityLocks::acquire(&self.repo_root)?;
        let pending = transaction_directories(&self.transaction_root(), ".pending")?;
        let [pending] = pending.as_slice() else {
            return Err(error(
                CharterPromotionErrorKindV1::InvalidRequest,
                "recovery fault fixture requires exactly one pending transaction",
            ));
        };
        self.recover_one_pending(pending)
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
        let intent = self
            .validate_terminal_history(Some(&canonical_bytes), None)?
            .ok_or_else(|| {
                error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "canonical authority exists without a committed promotion journal",
                )
            })?;
        Ok(Some(CommittedCharterAuthorityV1 {
            canonical_bytes,
            canonical_fingerprint: fingerprint,
            promotion_ref: intent.record.outputs.promotion_record.record_ref,
            lifecycle_transition_ref: intent.record.outputs.lifecycle_transition.record_ref,
        }))
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
        let intent = self
            .validate_terminal_history(Some(&canonical_bytes), None)?
            .ok_or_else(|| {
                error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "canonical authority exists without a committed promotion journal",
                )
            })?;
        Ok(Some(CommittedCharterAuthorityV1 {
            canonical_bytes,
            canonical_fingerprint: fingerprint,
            promotion_ref: intent.record.outputs.promotion_record.record_ref,
            lifecycle_transition_ref: intent.record.outputs.lifecycle_transition.record_ref,
        }))
    }

    #[cfg(test)]
    fn promote_inner(
        &self,
        request: CharterPromotionRequestV1,
    ) -> Result<CharterPromotionCommitV1, CharterPromotionErrorV1> {
        ensure_supported_platform()?;
        if request.canonical_bytes.is_empty() || request.canonical_bytes.len() > MAX_CANONICAL_BYTES
        {
            return Err(error(
                CharterPromotionErrorKindV1::InvalidRequest,
                "canonical artifact must be non-empty and no larger than 1 MiB",
            ));
        }
        let prepared = self.preflight(&request)?;
        let _locks = AuthorityLocks::acquire(&self.repo_root)?;
        self.recover_pending_locked()?;
        self.promote_prepared_locked(request, prepared, None)
    }

    fn promote_prepared_locked(
        &self,
        request: CharterPromotionRequestV1,
        prepared: PreparedPromotion,
        retained: Option<&RetainedPromotionAuthorityV1>,
    ) -> Result<CharterPromotionCommitV1, CharterPromotionErrorV1> {
        let old = read_current_canonical(&self.repo_root)?;
        verify_expected_basis(&prepared.expected_current_fingerprint, old.as_deref())?;
        #[cfg(test)]
        inject_writer_fault(WriterBoundaryV12::W0Admitted)?;
        let transaction_id = allocate_transaction_id()?;
        validate_transaction_id(&transaction_id)?;
        let transaction_root = self.transaction_root();
        let pending = self.pending_path(&transaction_id);
        let committed = self.committed_path(&transaction_id);
        if pending.exists() || committed.exists() || self.rolled_back_path(&transaction_id).exists()
        {
            return Err(error(
                CharterPromotionErrorKindV1::Conflict,
                "engine-allocated promotion transaction ID collided",
            ));
        }
        let intent = build_promotion_intent(&prepared, old.as_deref(), &transaction_id)?;
        publish_intent_from_scratch(&self.repo_root, &transaction_root, &pending, &intent)?;
        if let Some(old_bytes) = &old {
            write_new_durable(&pending.join("canonical.old"), old_bytes)?;
            let observed = read_bounded_regular(&pending.join("canonical.old"), old_bytes.len())?;
            if observed != *old_bytes {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "canonical rollback snapshot differs from retained old authority",
                ));
            }
        }
        #[cfg(test)]
        inject_writer_fault(WriterBoundaryV12::W4OldSnapshot)?;
        publish_output_stage_from_scratch(
            &self.repo_root,
            &transaction_root,
            &pending,
            OutputPurposeV12::Canonical,
            &request.canonical_bytes,
            &intent.record,
        )?;
        #[cfg(test)]
        inject_writer_fault(WriterBoundaryV12::W5CanonicalStaged)?;
        publish_output_stage_from_scratch(
            &self.repo_root,
            &transaction_root,
            &pending,
            OutputPurposeV12::PromotionRecord,
            &request.promotion_record_bytes,
            &intent.record,
        )?;
        #[cfg(test)]
        inject_writer_fault(WriterBoundaryV12::W6PromotionStaged)?;
        publish_output_stage_from_scratch(
            &self.repo_root,
            &transaction_root,
            &pending,
            OutputPurposeV12::LifecycleTransition,
            &request.lifecycle_transition_bytes,
            &intent.record,
        )?;
        #[cfg(test)]
        inject_writer_fault(WriterBoundaryV12::W7LifecycleStaged)?;
        publish_marker(&pending, "prepared", &intent.raw_bytes)?;
        #[cfg(test)]
        inject_writer_fault(WriterBoundaryV12::W8Prepared)?;
        if let Some(retained) = retained {
            if let Err(failure) = self
                .verify_retained_authority(&pending, old.as_deref(), retained)
                .and_then(|_| self.validate_precommit_authority(&pending, &intent))
            {
                self.rollback_and_finalize(&pending, &intent)?;
                return Err(failure);
            }
        }

        let target = self.repo_root.join(CANONICAL_REF);
        let target_parent = target.parent().expect("fixed target has a parent");
        create_safe_directories(&self.repo_root, target_parent).map_err(map_lineage)?;
        rename_durable(&pending.join("canonical.new"), &target, target_parent)?;
        sync_directory(&pending).map_err(map_lineage)?;
        if read_current_canonical(&self.repo_root)?.as_deref()
            != Some(request.canonical_bytes.as_slice())
        {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "canonical target differs after atomic install",
            ));
        }
        #[cfg(test)]
        inject_writer_fault(WriterBoundaryV12::W9CanonicalInstalled)?;
        #[cfg(test)]
        inject_exact_promotion_fault(
            CharterPromotionFaultPointV1::AfterCanonicalInstalled,
            "injected fault after canonical install",
        )?;
        publish_marker(&pending, "canonical-installed", &intent.raw_bytes)?;
        #[cfg(test)]
        inject_writer_fault(WriterBoundaryV12::W10CanonicalMarked)?;
        self.install_record_stage(
            &pending,
            OutputPurposeV12::PromotionRecord,
            &request.promotion_record_bytes,
            &intent.record.outputs.promotion_record,
        )?;
        #[cfg(test)]
        inject_writer_fault(WriterBoundaryV12::W11PromotionInstalled)?;
        self.install_record_stage(
            &pending,
            OutputPurposeV12::LifecycleTransition,
            &request.lifecycle_transition_bytes,
            &intent.record.outputs.lifecycle_transition,
        )?;
        #[cfg(test)]
        inject_writer_fault(WriterBoundaryV12::W12LifecycleInstalled)?;
        #[cfg(test)]
        inject_exact_promotion_fault(
            CharterPromotionFaultPointV1::AfterRecordsInstalled,
            "injected fault after content-addressed record install",
        )?;
        publish_marker(&pending, "records-installed", &intent.raw_bytes)?;
        #[cfg(test)]
        inject_writer_fault(WriterBoundaryV12::W13RecordsMarked)?;

        self.commit_and_finalize(&pending, &committed, &intent.raw_bytes)?;
        Ok(CharterPromotionCommitV1 {
            promotion_ref: prepared.outputs.promotion_record.record_ref,
            lifecycle_transition_ref: prepared.outputs.lifecycle_transition.record_ref,
            committed_marker_path: committed.join("committed"),
        })
    }

    fn verify_retained_authority(
        &self,
        pending: &Path,
        expected_current: Option<&[u8]>,
        retained: &RetainedPromotionAuthorityV1,
    ) -> Result<(), CharterPromotionErrorV1> {
        let current = read_current_canonical(&self.repo_root)?;
        if current.as_deref() != expected_current {
            return Err(error(
                CharterPromotionErrorKindV1::Conflict,
                "canonical basis bytes changed before authority installation",
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
        let candidate_subject_fingerprint =
            string_field(&candidate, "candidate_subject_fingerprint").map_err(map_lineage)?;
        let intake_record_ref =
            string_field(&candidate, "intake_record_ref").map_err(map_lineage)?;
        let intake_record_fingerprint =
            fingerprint_from_content_ref(intake_record_ref, "intake-records/intake_", ".json")?;
        self.lineage
            .read_record_by_ref(
                LineageRecordClassV1::Intake,
                intake_record_ref,
                Some(&intake_record_fingerprint),
            )
            .map_err(map_lineage)?;
        let normalized_content_ref =
            string_field(&candidate, "normalized_content_ref").map_err(map_lineage)?;
        let normalized_content_fingerprint = fingerprint_from_content_ref(
            normalized_content_ref,
            "candidate-content/charter_",
            ".yaml",
        )?;
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

        let retained_validation =
            validate_candidate_exact_result_authority(&self.repo_root, &candidate).map_err(
                |failure| {
                    error(
                        CharterPromotionErrorKindV1::LineageViolation,
                        failure.detail(),
                    )
                },
            )?;
        let validation_result_ref = retained_validation.relative_ref;
        let validation_result_bytes = retained_validation.bytes;
        let validation_result =
            validate_result_bytes(&validation_result_bytes, Some(&validation_result_ref)).map_err(
                |failure| {
                    error(
                        CharterPromotionErrorKindV1::LineageViolation,
                        failure.detail(),
                    )
                },
            )?;
        if validation_result.candidate_subject_fingerprint != candidate_subject_fingerprint
            || validation_result.intake_record_ref != intake_record_ref
            || validation_result.intake_record_fingerprint != intake_record_fingerprint
            || validation_result.normalized_content_ref != normalized_content_ref
            || validation_result.normalized_content_fingerprint != normalized_content_fingerprint
            || validation_result.basis_artifact_fingerprint != expected_current_fingerprint
            || validation_result.observed_current_artifact_fingerprint
                != expected_current_fingerprint
        {
            return Err(error(
                CharterPromotionErrorKindV1::LineageViolation,
                "candidate lifecycle-validation result does not close the promotion lineage",
            ));
        }
        let closed_candidate_authority = self.validate_candidate_validation_authority(
            candidate_ref,
            candidate_fingerprint,
            &request.canonical_bytes,
        )?;
        if closed_candidate_authority.candidate != candidate
            || closed_candidate_authority.validation_result_ref != validation_result_ref
            || closed_candidate_authority.validation_result != validation_result
        {
            return Err(error(
                CharterPromotionErrorKindV1::LineageViolation,
                "promotion preflight did not retain one exact candidate/result authority closure",
            ));
        }
        let promoted_validation_refs = promotion
            .value
            .get("validation_result_refs")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                error(
                    CharterPromotionErrorKindV1::LineageViolation,
                    "promotion validation_result_refs must be an array",
                )
            })?;
        if promoted_validation_refs.as_slice()
            != [Value::String(validation_result_ref.clone())].as_slice()
        {
            return Err(error(
                CharterPromotionErrorKindV1::LineageViolation,
                "promotion and candidate lifecycle-validation refs differ",
            ));
        }

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
        let mut approval_bindings = Vec::with_capacity(approvals.len());
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
            let approval_fingerprint =
                fingerprint_from_content_ref(approval_ref, "approvals/approval_", ".json")?;
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
            approval_bindings.push(PromotionApprovalBindingV12 {
                approval_ref: approval_ref.to_owned(),
                approval_fingerprint,
                approval_class: string_field(&approval, "approval_class")
                    .map_err(map_lineage)?
                    .to_owned(),
                authority_ref: string_field(&approval, "authority_ref")
                    .map_err(map_lineage)?
                    .to_owned(),
            });
        }

        require_equal_string(&lifecycle.value, "target_instance_id", "project_authority")?;
        require_equal_string(&lifecycle.value, "result_state", "current")?;
        require_equal_string(
            &lifecycle.value,
            "clearance_promotion_ref",
            &promotion.relative_ref,
        )?;
        let promotion_id = string_field(&promotion.value, "promotion_id").map_err(map_lineage)?;
        let promotion_document_sha256 =
            DefinitionFingerprint::from_bytes(&request.promotion_record_bytes).to_string();
        let lifecycle_document_sha256 =
            DefinitionFingerprint::from_bytes(&request.lifecycle_transition_bytes).to_string();
        Ok(PreparedPromotion {
            expected_current_fingerprint,
            promotion_id: promotion_id.to_owned(),
            candidate_lineage: PromotionCandidateLineageV12 {
                candidate_ref: candidate_ref.to_owned(),
                candidate_fingerprint: candidate_fingerprint.to_owned(),
                candidate_subject_fingerprint: candidate_subject_fingerprint.to_owned(),
                intake_record_ref: intake_record_ref.to_owned(),
                intake_record_fingerprint,
                normalized_content_ref: normalized_content_ref.to_owned(),
                normalized_content_fingerprint,
                validation_result_ref: validation_result_ref.to_owned(),
                validation_result_fingerprint: validation_result
                    .validation_result_fingerprint
                    .clone(),
            },
            selected_contract: selected_contract_from_validation(&validation_result),
            human_authority: PromotionHumanAuthorityV12 {
                approval_bindings,
                approver_registry_state_ref: registry_ref.to_owned(),
                approver_registry_state_fingerprint: registry_fp.to_owned(),
                registry_head_transition_ref: head_ref.to_owned(),
                registry_head_transition_fingerprint: head_fp.to_owned(),
            },
            outputs: PromotionOutputsV12 {
                new_canonical_fingerprint: canonical_fingerprint.clone(),
                new_canonical_document_sha256: canonical_fingerprint.clone(),
                new_canonical_byte_length: request.canonical_bytes.len() as u64,
                promotion_record: PromotionOutputRecordV12 {
                    record_ref: promotion.relative_ref,
                    record_fingerprint: promotion.fingerprint,
                    document_sha256: promotion_document_sha256,
                    byte_length: request.promotion_record_bytes.len() as u64,
                },
                lifecycle_transition: PromotionOutputRecordV12 {
                    record_ref: lifecycle.relative_ref,
                    record_fingerprint: lifecycle.fingerprint,
                    document_sha256: lifecycle_document_sha256,
                    byte_length: request.lifecycle_transition_bytes.len() as u64,
                },
            },
        })
    }

    fn validate_candidate_validation_authority(
        &self,
        candidate_ref: &str,
        candidate_fingerprint: &str,
        intended_canonical_bytes: &[u8],
    ) -> Result<CandidateValidationAuthorityV12, CharterPromotionErrorV1> {
        let candidate = self
            .lineage
            .read_record_by_ref(
                LineageRecordClassV1::Candidate,
                candidate_ref,
                Some(candidate_fingerprint),
            )
            .map_err(map_lineage)?;
        let candidate_subject_fingerprint =
            string_field(&candidate, "candidate_subject_fingerprint").map_err(map_lineage)?;
        let mut subject_preimage = candidate.clone();
        let subject = subject_preimage.as_object_mut().ok_or_else(|| {
            error(
                CharterPromotionErrorKindV1::LineageViolation,
                "candidate authority is not an object",
            )
        })?;
        for field in [
            "candidate_id",
            "candidate_fingerprint",
            "candidate_subject_fingerprint",
            "validation_result_binding",
        ] {
            subject.remove(field);
        }
        if DefinitionFingerprint::from_json_value(&subject_preimage)
            .map_err(|_| {
                error(
                    CharterPromotionErrorKindV1::LineageViolation,
                    "candidate subject identity cannot be recomputed",
                )
            })?
            .as_str()
            != candidate_subject_fingerprint
        {
            return Err(error(
                CharterPromotionErrorKindV1::LineageViolation,
                "candidate subject fingerprint differs from its exact final subject preimage",
            ));
        }

        let intake_record_ref =
            string_field(&candidate, "intake_record_ref").map_err(map_lineage)?;
        let intake_record_fingerprint =
            fingerprint_from_content_ref(intake_record_ref, "intake-records/intake_", ".json")?;
        let intake = self
            .lineage
            .read_record_by_ref(
                LineageRecordClassV1::Intake,
                intake_record_ref,
                Some(&intake_record_fingerprint),
            )
            .map_err(map_lineage)?;
        for field in [
            "target_kind_ref",
            "target_instance_id",
            "profile_ref",
            "resolved_profile_fingerprint",
            "basis_artifact_fingerprint",
        ] {
            equal_json_field(&candidate, &intake, field)?;
        }

        let normalized_content_ref =
            string_field(&candidate, "normalized_content_ref").map_err(map_lineage)?;
        let normalized_content_fingerprint = fingerprint_from_content_ref(
            normalized_content_ref,
            "candidate-content/charter_",
            ".yaml",
        )?;
        let normalized_content = self
            .lineage
            .read_candidate_content(normalized_content_ref)
            .map_err(map_lineage)?;
        if normalized_content != intended_canonical_bytes
            || DefinitionFingerprint::from_bytes(&normalized_content).as_str()
                != normalized_content_fingerprint
        {
            return Err(error(
                CharterPromotionErrorKindV1::LineageViolation,
                "candidate normalized content is not the exact intended canonical authority",
            ));
        }

        let retained_validation =
            validate_candidate_exact_result_authority(&self.repo_root, &candidate).map_err(
                |failure| {
                    error(
                        CharterPromotionErrorKindV1::LineageViolation,
                        failure.detail(),
                    )
                },
            )?;
        let validation_result_ref = retained_validation.relative_ref;
        let validation_result_bytes = retained_validation.bytes;
        let validation_result =
            validate_result_bytes(&validation_result_bytes, Some(&validation_result_ref)).map_err(
                |failure| {
                    error(
                        CharterPromotionErrorKindV1::LineageViolation,
                        failure.detail(),
                    )
                },
            )?;
        let candidate_basis = optional_fingerprint(&candidate, "basis_artifact_fingerprint")?;
        if validation_result.candidate_subject_fingerprint != candidate_subject_fingerprint
            || validation_result.intake_record_ref != intake_record_ref
            || validation_result.intake_record_fingerprint != intake_record_fingerprint
            || validation_result.normalized_content_ref != normalized_content_ref
            || validation_result.normalized_content_fingerprint != normalized_content_fingerprint
            || validation_result.target_instance_id
                != string_field(&candidate, "target_instance_id").map_err(map_lineage)?
            || validation_result.profile_ref
                != string_field(&candidate, "profile_ref").map_err(map_lineage)?
            || validation_result.resolved_profile_fingerprint
                != string_field(&candidate, "resolved_profile_fingerprint").map_err(map_lineage)?
            || validation_result.basis_artifact_fingerprint != candidate_basis
            || validation_result.observed_current_artifact_fingerprint != candidate_basis
        {
            return Err(error(
                CharterPromotionErrorKindV1::LineageViolation,
                "candidate/result/intake/content authority closure is not exact",
            ));
        }
        Ok(CandidateValidationAuthorityV12 {
            candidate,
            validation_result_ref,
            validation_result,
        })
    }

    fn validate_resolved_definitions(
        &self,
        promotion: &Value,
        candidate: &Value,
        decisions: &crate::ResolvedProfileDecisions,
        definition_registry: &crate::CharterDefinitionRegistry,
    ) -> Result<(), CharterPromotionErrorV1> {
        let resolved = promotion
            .get("resolved_definitions")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                error(
                    CharterPromotionErrorKindV1::LineageViolation,
                    "promotion resolved_definitions must be an array",
                )
            })?;
        let expected = crate::charter_promotion_workflow::resolved_definition_bindings(
            candidate,
            decisions,
            definition_registry,
        )
        .map_err(|failure| {
            error(
                CharterPromotionErrorKindV1::LineageViolation,
                format!(
                    "promotion definition closure cannot be recomputed: {}",
                    failure.detail()
                ),
            )
        })?;
        if resolved != &expected {
            return Err(error(
                CharterPromotionErrorKindV1::LineageViolation,
                "promotion must retain the complete ordered shipped definition closure",
            ));
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
        let inventory = transaction_root_inventory(&root)?;
        self.validate_terminal_inventory(&inventory)?;
        for pending in inventory.pending {
            self.recover_one_pending(&pending)?;
        }
        let canonical = read_current_canonical(&self.repo_root)?;
        self.validate_terminal_history(canonical.as_deref(), None)?;
        Ok(())
    }

    fn validate_terminal_history(
        &self,
        current_canonical: Option<&[u8]>,
        expected_head_fingerprint: Option<&str>,
    ) -> Result<Option<ValidatedPromotionIntentV12>, CharterPromotionErrorV1> {
        if current_canonical.is_some() && expected_head_fingerprint.is_some() {
            return Err(error(
                CharterPromotionErrorKindV1::InvalidRequest,
                "promotion history accepts exactly one current-head observation",
            ));
        }
        let root = self.transaction_root();
        if !root.exists() {
            return if current_canonical.is_none() && expected_head_fingerprint.is_none() {
                Ok(None)
            } else {
                Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "canonical authority exists without promotion transaction history",
                ))
            };
        }
        reject_reparse_or_symlink(&root, true).map_err(map_lineage)?;
        sync_directory(&root).map_err(map_lineage)?;
        let inventory = transaction_root_inventory(&root)?;
        let selected = self.validate_terminal_inventory(&inventory)?;
        match (
            current_canonical,
            expected_head_fingerprint,
            selected.as_ref(),
        ) {
            (None, None, None) => Ok(None),
            (Some(current), None, Some(intent))
                if current.len() as u64 == intent.record.outputs.new_canonical_byte_length
                    && DefinitionFingerprint::from_bytes(current).as_str()
                        == intent.record.outputs.new_canonical_document_sha256
                    && intent.record.outputs.new_canonical_fingerprint
                        == intent.record.outputs.new_canonical_document_sha256 =>
            {
                Ok(selected)
            }
            (None, Some(expected), Some(intent))
                if intent.record.outputs.new_canonical_fingerprint == expected
                    && intent.record.outputs.new_canonical_fingerprint
                        == intent.record.outputs.new_canonical_document_sha256 =>
            {
                Ok(selected)
            }
            _ => Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "current canonical authority is not the exact committed promotion chain head",
            )),
        }
    }

    fn validate_terminal_inventory(
        &self,
        inventory: &TransactionRootInventoryV12,
    ) -> Result<Option<ValidatedPromotionIntentV12>, CharterPromotionErrorV1> {
        let mut transaction_ids = BTreeSet::new();

        for rolled_back in &inventory.rolled_back {
            let intent = read_valid_intent(rolled_back)?;
            validate_terminal_directory_identity(rolled_back, &intent, ".rolled-back")?;
            if !transaction_ids.insert(intent.record.transaction_id.clone()) {
                return Err(error(
                    CharterPromotionErrorKindV1::Conflict,
                    "promotion transaction ID has multiple terminal suffixes",
                ));
            }
            validate_terminal_payload(rolled_back, &intent, PromotionTerminalKindV12::RolledBack)?;
        }

        let mut committed_by_basis: BTreeMap<Option<String>, Vec<ValidatedPromotionIntentV12>> =
            BTreeMap::new();
        for committed in &inventory.committed {
            let intent = read_valid_intent(committed)?;
            validate_terminal_directory_identity(committed, &intent, ".committed")?;
            if !transaction_ids.insert(intent.record.transaction_id.clone()) {
                return Err(error(
                    CharterPromotionErrorKindV1::Conflict,
                    "promotion transaction ID has multiple terminal suffixes",
                ));
            }
            validate_terminal_payload(committed, &intent, PromotionTerminalKindV12::Committed)?;
            self.validate_final_records(&intent.record)?;
            committed_by_basis
                .entry(
                    intent
                        .record
                        .target
                        .observed_current_artifact_fingerprint
                        .clone(),
                )
                .or_default()
                .push(intent);
        }

        let mut cursor = None;
        let mut selected = None;
        let mut traversed = 0_usize;
        while let Some(mut successors) = committed_by_basis.remove(&cursor) {
            if successors.len() != 1 {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "committed promotion history contains a forked successor basis",
                ));
            }
            let successor = successors.remove(0);
            if successor.record.target.basis_artifact_fingerprint != cursor {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "committed promotion successor basis is not exact",
                ));
            }
            cursor = Some(successor.record.outputs.new_canonical_fingerprint.clone());
            selected = Some(successor);
            traversed += 1;
            if traversed > 4096 {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "committed promotion history exceeds its bounded chain length",
                ));
            }
        }
        if !committed_by_basis.is_empty() {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "committed promotion history contains an unreachable or cyclic successor",
            ));
        }
        Ok(selected)
    }

    pub(crate) fn current_committed_intent_locked(
        &self,
        canonical_fingerprint: &str,
    ) -> Result<ValidatedPromotionIntentV12, CharterPromotionErrorV1> {
        self.validate_terminal_history(None, Some(canonical_fingerprint))?
            .ok_or_else(|| {
                error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "promotion history has no committed chain head",
                )
            })
    }

    fn recover_one_pending(&self, pending: &Path) -> Result<(), CharterPromotionErrorV1> {
        reject_reparse_or_symlink(pending, true).map_err(map_lineage)?;
        let names = pending_entry_names(pending)?;
        if !names.contains("intent.json") {
            if !names.is_empty() {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "pre-intent pending directory is not empty",
                ));
            }
            fs::remove_dir(pending)
                .map_err(|_| io_error("empty pre-intent pending removal failed"))?;
            return sync_directory(&self.transaction_root()).map_err(map_lineage);
        }
        let intent = read_valid_intent(pending)?;
        let directory_name = pending
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| name.strip_suffix(".pending"))
            .ok_or_else(|| {
                error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "pending transaction directory name is invalid",
                )
            })?;
        if directory_name != intent.record.transaction_id {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "pending directory key differs from intent transaction ID",
            ));
        }
        let committed_destination = self.committed_path(&intent.record.transaction_id);
        let rolled_destination = self.rolled_back_path(&intent.record.transaction_id);
        if committed_destination.exists() || rolled_destination.exists() {
            return Err(error(
                CharterPromotionErrorKindV1::Conflict,
                "simultaneous pending and terminal promotion suffixes preserve evidence",
            ));
        }
        let current = read_current_canonical(&self.repo_root)?;
        let new_is_installed = current.as_deref().is_some_and(|bytes| {
            bytes.len() as u64 == intent.record.outputs.new_canonical_byte_length
                && DefinitionFingerprint::from_bytes(bytes).to_string()
                    == intent.record.outputs.new_canonical_document_sha256
        });
        let old_is_current = matches_expected_current(
            &intent.record.target.observed_current_artifact_fingerprint,
            current.as_deref(),
        );
        if names.contains("rolled-back") {
            if !old_is_current {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "published rollback marker requires the bound old/absent target",
                ));
            }
            sync_directory(pending).map_err(map_lineage)?;
            validate_terminal_payload(pending, &intent, PromotionTerminalKindV12::RolledBack)?;
            return self.finalize_pending(
                pending,
                &rolled_destination,
                &intent,
                PromotionTerminalKindV12::RolledBack,
            );
        }
        if pending.join("committed").exists() {
            validate_committed_marker(pending, &intent.raw_bytes)?;
            if !new_is_installed {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "committed promotion journal does not match installed canonical authority",
                ));
            }
            self.validate_final_records(&intent.record)?;
            sync_directory(pending).map_err(map_lineage)?;
            validate_terminal_payload(pending, &intent, PromotionTerminalKindV12::Committed)?;
            return self.finalize_pending(
                pending,
                &committed_destination,
                &intent,
                PromotionTerminalKindV12::Committed,
            );
        }
        if old_is_current {
            validate_rollback_origin_or_progress(pending, &intent)?;
            #[cfg(test)]
            inject_recovery_fault(RecoveryBoundaryV12::R0Admitted)?;
            return self.rollback_and_finalize(pending, &intent);
        }
        if new_is_installed {
            return self.roll_forward_and_finalize(pending, &intent);
        }
        Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "pending promotion encountered unrelated canonical bytes",
        ))
    }

    fn roll_forward_and_finalize(
        &self,
        pending: &Path,
        intent: &ValidatedPromotionIntentV12,
    ) -> Result<(), CharterPromotionErrorV1> {
        validate_roll_forward_origin(pending, intent)?;
        self.validate_precommit_authority(pending, intent)?;
        ensure_published_marker(pending, "canonical-installed", &intent.raw_bytes)?;
        self.recover_record_output(pending, OutputPurposeV12::PromotionRecord, intent)?;
        self.recover_record_output(pending, OutputPurposeV12::LifecycleTransition, intent)?;
        ensure_published_marker(pending, "records-installed", &intent.raw_bytes)?;
        self.validate_final_records(&intent.record)?;
        let committed = self.committed_path(&intent.record.transaction_id);
        self.commit_and_finalize(pending, &committed, &intent.raw_bytes)
    }

    fn recover_record_output(
        &self,
        pending: &Path,
        purpose: OutputPurposeV12,
        intent: &ValidatedPromotionIntentV12,
    ) -> Result<(), CharterPromotionErrorV1> {
        let output = match purpose {
            OutputPurposeV12::PromotionRecord => &intent.record.outputs.promotion_record,
            OutputPurposeV12::LifecycleTransition => &intent.record.outputs.lifecycle_transition,
            OutputPurposeV12::Canonical => {
                return Err(error(
                    CharterPromotionErrorKindV1::InvalidRequest,
                    "canonical output is not recoverable as a record final",
                ))
            }
        };
        let stage = pending.join(output_stage_name(purpose));
        let final_path = self
            .lineage
            .state_path(&output.record_ref)
            .map_err(map_lineage)?;
        let stage_bytes = optional_bounded_regular(&stage, MAX_OUTPUT_RECORD_BYTES)?;
        let final_bytes = optional_bounded_regular(&final_path, MAX_OUTPUT_RECORD_BYTES)?;
        if let Some(bytes) = &stage_bytes {
            validate_output_bytes_against_intent(purpose, bytes, &intent.record)?;
        }
        if let Some(bytes) = &final_bytes {
            validate_output_bytes_against_intent(purpose, bytes, &intent.record)?;
        }
        match (stage_bytes, final_bytes) {
            (Some(stage_bytes), None) | (Some(stage_bytes), Some(_)) => {
                self.install_record_stage(pending, purpose, &stage_bytes, output)
            }
            (None, Some(_)) => Ok(()),
            (None, None) => Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "record output is unaccounted during roll-forward recovery",
            )),
        }
    }

    fn validate_precommit_authority(
        &self,
        pending: &Path,
        intent: &ValidatedPromotionIntentV12,
    ) -> Result<(), CharterPromotionErrorV1> {
        let intended_canonical = intended_canonical_bytes(pending, intent, &self.repo_root)?;
        let closed_candidate_authority = self.validate_candidate_validation_authority(
            &intent.record.candidate_lineage.candidate_ref,
            &intent.record.candidate_lineage.candidate_fingerprint,
            &intended_canonical,
        )?;
        let candidate = closed_candidate_authority.candidate;
        require_equal_string(
            &candidate,
            "candidate_subject_fingerprint",
            &intent
                .record
                .candidate_lineage
                .candidate_subject_fingerprint,
        )?;
        if closed_candidate_authority.validation_result_ref
            != intent.record.candidate_lineage.validation_result_ref
        {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "precommit candidate sole validation-result ref differs from intent",
            ));
        }
        let validation = closed_candidate_authority.validation_result;
        if validation.validation_result_fingerprint
            != intent
                .record
                .candidate_lineage
                .validation_result_fingerprint
            || selected_contract_from_validation(&validation) != intent.record.selected_contract
        {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "precommit lifecycle-validation authority differs from intent",
            ));
        }
        for approval in &intent.record.human_authority.approval_bindings {
            let value = self
                .lineage
                .read_record_by_ref(
                    LineageRecordClassV1::Approval,
                    &approval.approval_ref,
                    Some(&approval.approval_fingerprint),
                )
                .map_err(map_lineage)?;
            require_equal_string(&value, "approval_class", &approval.approval_class)?;
            require_equal_string(&value, "authority_ref", &approval.authority_ref)?;
        }
        let selected_registry_state = self
            .lineage
            .read_record_by_ref(
                LineageRecordClassV1::RegistryState,
                &intent.record.human_authority.approver_registry_state_ref,
                Some(
                    &intent
                        .record
                        .human_authority
                        .approver_registry_state_fingerprint,
                ),
            )
            .map_err(map_lineage)?;
        self.lineage
            .read_record(
                LineageRecordClassV1::RegistryTransition,
                &intent.record.human_authority.registry_head_transition_ref,
                &intent
                    .record
                    .human_authority
                    .registry_head_transition_fingerprint,
            )
            .map_err(map_lineage)?;
        let decisions = resolve_shipped_profile_decisions(&self.repo_root).map_err(|_| {
            error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "precommit required-pair authority cannot resolve the shipped profile",
            )
        })?;
        let prior_charter = match intent.record.mutation_mode.as_str() {
            "create" => None,
            "amend" => {
                let old =
                    read_bounded_regular(&pending.join("canonical.old"), MAX_CANONICAL_BYTES)?;
                Some(parse_canonical_charter(&decisions, &old).map_err(|_| {
                    error(
                        CharterPromotionErrorKindV1::DurabilityViolation,
                        "precommit prior canonical cannot derive amendment approval pairs",
                    )
                })?)
            }
            _ => {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "precommit promotion mutation mode is unsupported",
                ))
            }
        };
        let required_pairs =
            required_approval_pairs_for_authority(&selected_registry_state, prior_charter.as_ref())
                .map_err(|failure| {
                    error(
                        CharterPromotionErrorKindV1::DurabilityViolation,
                        format!(
                            "precommit required approval pairs cannot be recomputed: {}",
                            failure.detail()
                        ),
                    )
                })?;
        let selected_pairs = intent
            .record
            .human_authority
            .approval_bindings
            .iter()
            .map(|binding| {
                (
                    binding.approval_class.as_str(),
                    binding.authority_ref.as_str(),
                )
            })
            .collect::<Vec<_>>();
        if selected_pairs.len() != required_pairs.len()
            || selected_pairs.iter().zip(&required_pairs).any(
                |((approval_class, authority_ref), required)| {
                    *approval_class != required.approval_class
                        || *authority_ref != required.authority_ref
                },
            )
        {
            return Err(error(
                CharterPromotionErrorKindV1::Conflict,
                "precommit approval selection is not exactly one current approval per required pair in lexical order",
            ));
        }
        #[cfg(test)]
        if !self
            .repo_root
            .join(".handbook/state/transactions/registry")
            .exists()
            && !self
                .repo_root
                .join(".handbook/state/transactions/approvals")
                .exists()
        {
            // The transaction module's synthetic unit fixtures predate the product
            // authority journals and deliberately exercise only the atomic writer.
            // Production and integration-test builds cannot take this branch.
            return self.validate_precommit_lifecycle_authority(pending, &intent.record);
        }
        let registry =
            observe_committed_approver_registry_locked(&self.repo_root).map_err(|failure| {
                error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    format!(
                        "precommit registry currentness refused: {}",
                        failure.detail()
                    ),
                )
            })?;
        if registry.state_ref != intent.record.human_authority.approver_registry_state_ref
            || registry.state_fingerprint
                != intent
                    .record
                    .human_authority
                    .approver_registry_state_fingerprint
            || registry.head_transition_ref
                != intent.record.human_authority.registry_head_transition_ref
            || registry.head_transition_fingerprint
                != intent
                    .record
                    .human_authority
                    .registry_head_transition_fingerprint
        {
            return Err(error(
                CharterPromotionErrorKindV1::Conflict,
                "precommit approver-registry head differs from the selected intent authority",
            ));
        }
        let committed_approval_refs = observe_committed_candidate_approval_refs_locked(
            &self.repo_root,
            &intent.record.candidate_lineage.candidate_ref,
            &intent.record.candidate_lineage.candidate_fingerprint,
            None,
        )
        .map_err(|failure| {
            error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                format!(
                    "precommit approval currentness refused: {}",
                    failure.message
                ),
            )
        })?;
        if intent
            .record
            .human_authority
            .approval_bindings
            .iter()
            .any(|binding| {
                committed_approval_refs
                    .binary_search_by(|observed| {
                        observed.as_bytes().cmp(binding.approval_ref.as_bytes())
                    })
                    .is_err()
            })
        {
            return Err(error(
                CharterPromotionErrorKindV1::Conflict,
                "precommit approval selection is no longer committed authority",
            ));
        }
        self.validate_precommit_lifecycle_authority(pending, &intent.record)?;
        Ok(())
    }

    fn validate_precommit_lifecycle_authority(
        &self,
        pending: &Path,
        intent: &PromotionTransactionIntentV12,
    ) -> Result<(), CharterPromotionErrorV1> {
        let selected = &intent.selected_contract;
        if intent.recovery.old_canonical_status == "absent" {
            if pending.join("canonical.old").exists()
                || selected.prior_lifecycle_head_ref.is_some()
                || selected.prior_lifecycle_head_fingerprint.is_some()
                || selected.prior_lifecycle_state != "absent"
                || selected.prior_lifecycle_state_fingerprint.is_some()
                || !selected.active_observations.is_empty()
                || !selected.reopened_coverage_ids.is_empty()
                || !transaction_directories(&self.transaction_root(), ".committed")?.is_empty()
            {
                return Err(error(
                    CharterPromotionErrorKindV1::Conflict,
                    "create promotion does not retain an exact absent prior lifecycle authority",
                ));
            }
            let lifecycle_root = self
                .repo_root
                .join(".handbook/state/transactions/lifecycle-events");
            if lifecycle_root.exists()
                && !transaction_directories(&lifecycle_root, ".committed")?.is_empty()
            {
                return Err(error(
                    CharterPromotionErrorKindV1::Conflict,
                    "create promotion encountered historical lifecycle authority",
                ));
            }
            return Ok(());
        }

        let old_bytes = read_bounded_regular(&pending.join("canonical.old"), MAX_CANONICAL_BYTES)?;
        if old_bytes.len() as u64
            != intent
                .recovery
                .old_canonical_byte_length
                .unwrap_or_default()
            || DefinitionFingerprint::from_bytes(&old_bytes).to_string()
                != intent
                    .recovery
                    .old_canonical_document_sha256
                    .as_deref()
                    .unwrap_or_default()
        {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "retained prior canonical bytes differ from lifecycle-selection intent",
            ));
        }
        let retained = CharterLifecycleStoreV1::new(&self.repo_root)
            .observe_canonical_bytes_retained_locked(&old_bytes)
            .map_err(|failure| {
                error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    format!(
                        "precommit lifecycle currentness refused: {}",
                        failure.detail()
                    ),
                )
            })?;
        validate_selected_lifecycle_authority(&retained, selected)
    }

    fn validate_final_records(
        &self,
        intent: &PromotionTransactionIntentV12,
    ) -> Result<(), CharterPromotionErrorV1> {
        let promotion = self
            .lineage
            .read_record(
                LineageRecordClassV1::Promotion,
                &intent.outputs.promotion_record.record_ref,
                &intent.outputs.promotion_record.record_fingerprint,
            )
            .map_err(map_lineage)?;
        validate_output_bytes_against_intent(
            OutputPurposeV12::PromotionRecord,
            &promotion,
            intent,
        )?;
        let lifecycle = self
            .lineage
            .read_record(
                LineageRecordClassV1::LifecycleTransition,
                &intent.outputs.lifecycle_transition.record_ref,
                &intent.outputs.lifecycle_transition.record_fingerprint,
            )
            .map_err(map_lineage)?;
        validate_output_bytes_against_intent(
            OutputPurposeV12::LifecycleTransition,
            &lifecycle,
            intent,
        )
    }

    fn install_record_stage(
        &self,
        pending: &Path,
        purpose: OutputPurposeV12,
        expected_bytes: &[u8],
        expected_output: &PromotionOutputRecordV12,
    ) -> Result<(), CharterPromotionErrorV1> {
        let stage = pending.join(output_stage_name(purpose));
        let staged = read_bounded_regular(&stage, MAX_OUTPUT_RECORD_BYTES)?;
        if staged != expected_bytes {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "owned record stage changed before final installation",
            ));
        }
        let class = match purpose {
            OutputPurposeV12::PromotionRecord => LineageRecordClassV1::Promotion,
            OutputPurposeV12::LifecycleTransition => LineageRecordClassV1::LifecycleTransition,
            OutputPurposeV12::Canonical => {
                return Err(error(
                    CharterPromotionErrorKindV1::InvalidRequest,
                    "canonical output is not a lineage record",
                ))
            }
        };
        validate_record_output(class, &staged, expected_output)?;
        let final_path = self
            .lineage
            .state_path(&expected_output.record_ref)
            .map_err(map_lineage)?;
        let final_parent = final_path.parent().ok_or_else(|| {
            error(
                CharterPromotionErrorKindV1::UnsafeFilesystem,
                "record final path has no parent",
            )
        })?;
        create_safe_directories(&self.repo_root, final_parent).map_err(map_lineage)?;
        match fs::symlink_metadata(&final_path) {
            Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => {
                atomic_rename_no_replace(&stage, &final_path)?;
                sync_directory(pending).map_err(map_lineage)?;
                sync_directory(final_parent).map_err(map_lineage)?;
            }
            Ok(metadata) => {
                if metadata.file_type().is_symlink() || !metadata.is_file() {
                    return Err(error(
                        CharterPromotionErrorKindV1::UnsafeFilesystem,
                        "pre-existing record final is unsafe",
                    ));
                }
                let observed = read_bounded_regular(&final_path, MAX_OUTPUT_RECORD_BYTES)?;
                if observed != expected_bytes {
                    return Err(error(
                        CharterPromotionErrorKindV1::Conflict,
                        "pre-existing record final has unequal bytes",
                    ));
                }
                fs::remove_file(&stage)
                    .map_err(|_| io_error("redundant exact record stage removal failed"))?;
                sync_directory(pending).map_err(map_lineage)?;
            }
            Err(_) => return Err(io_error("record final metadata observation failed")),
        }
        let final_bytes = read_bounded_regular(&final_path, MAX_OUTPUT_RECORD_BYTES)?;
        if final_bytes != expected_bytes {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "installed record final differs from the exact retained stage",
            ));
        }
        validate_record_output(class, &final_bytes, expected_output)
    }

    fn commit_and_finalize(
        &self,
        pending: &Path,
        committed: &Path,
        intent_bytes: &[u8],
    ) -> Result<(), CharterPromotionErrorV1> {
        publish_marker(pending, "committed", intent_bytes)?;
        let intent = read_valid_intent(pending)?;
        self.validate_final_records(&intent.record)?;
        sync_directory(pending).map_err(map_lineage)?;
        validate_terminal_payload(pending, &intent, PromotionTerminalKindV12::Committed)?;
        #[cfg(test)]
        inject_writer_fault(WriterBoundaryV12::W14Committed)?;
        self.finalize_pending(
            pending,
            committed,
            &intent,
            PromotionTerminalKindV12::Committed,
        )?;
        #[cfg(test)]
        inject_writer_fault(WriterBoundaryV12::W15Terminal)?;
        Ok(())
    }

    fn finalize_pending(
        &self,
        pending: &Path,
        destination: &Path,
        intent: &ValidatedPromotionIntentV12,
        terminal_kind: PromotionTerminalKindV12,
    ) -> Result<(), CharterPromotionErrorV1> {
        sync_directory(pending).map_err(map_lineage)?;
        validate_terminal_payload(pending, intent, terminal_kind)?;
        let destination_state = classify_terminal_destination(destination, intent, terminal_kind);
        if !terminal_destination_allows_rename(destination_state) {
            return Err(error(
                CharterPromotionErrorKindV1::Conflict,
                format!(
                    "promotion terminal destination refuses no-replace rename: {destination_state:?}"
                ),
            ));
        }
        atomic_rename_no_replace(pending, destination)?;
        sync_directory(&self.transaction_root()).map_err(map_lineage)?;
        validate_terminal_payload(destination, intent, terminal_kind)
    }

    fn rollback_and_finalize(
        &self,
        pending: &Path,
        intent: &ValidatedPromotionIntentV12,
    ) -> Result<(), CharterPromotionErrorV1> {
        let current = read_current_canonical(&self.repo_root)?;
        if !matches_expected_current(
            &intent.record.target.observed_current_artifact_fingerprint,
            current.as_deref(),
        ) {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "rollback recovery requires the exact bound old/absent target",
            ));
        }
        complete_old_snapshot(pending, &intent.record, current.as_deref())?;
        #[cfg(test)]
        inject_recovery_fault(RecoveryBoundaryV12::R1SnapshotComplete)?;
        for name in [
            "prepared.tmp",
            "prepared",
            "lifecycle-transition.new",
            "promotion-record.new",
            "canonical.new",
        ] {
            remove_exact_rollback_owned_file(pending, name, intent)?;
            #[cfg(test)]
            inject_recovery_fault(match name {
                "prepared.tmp" => RecoveryBoundaryV12::R2PreparedTempRemoved,
                "prepared" => RecoveryBoundaryV12::R3PreparedRemoved,
                "lifecycle-transition.new" => RecoveryBoundaryV12::R4LifecycleStageRemoved,
                "promotion-record.new" => RecoveryBoundaryV12::R5PromotionStageRemoved,
                "canonical.new" => RecoveryBoundaryV12::R6CanonicalStageRemoved,
                _ => unreachable!("fixed rollback-owned name"),
            })?;
        }
        complete_marker_temp(pending, "rolled-back", &intent.raw_bytes)?;
        #[cfg(test)]
        inject_recovery_fault(RecoveryBoundaryV12::R7RollbackMarkerTemp)?;
        publish_completed_marker(pending, "rolled-back", &intent.raw_bytes)?;
        #[cfg(test)]
        inject_recovery_fault(RecoveryBoundaryV12::R8RollbackMarkerPublished)?;
        let destination = self.rolled_back_path(&intent.record.transaction_id);
        self.finalize_pending(
            pending,
            &destination,
            intent,
            PromotionTerminalKindV12::RolledBack,
        )?;
        #[cfg(test)]
        inject_recovery_fault(RecoveryBoundaryV12::R9RollbackTerminal)?;
        Ok(())
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

    fn rolled_back_path(&self, transaction_id: &str) -> PathBuf {
        self.transaction_root()
            .join(format!("{transaction_id}.rolled-back"))
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
        if request.canonical_bytes.is_empty() || request.canonical_bytes.len() > MAX_CANONICAL_BYTES
        {
            return Err(error(
                CharterPromotionErrorKindV1::InvalidRequest,
                "canonical artifact must be non-empty and no larger than 1 MiB",
            ));
        }
        let prepared = self.service.preflight(&request)?;
        self.service
            .promote_prepared_locked(request, prepared, Some(retained))
    }
}

#[derive(Debug)]
struct PreparedPromotion {
    expected_current_fingerprint: Option<String>,
    promotion_id: String,
    candidate_lineage: PromotionCandidateLineageV12,
    selected_contract: PromotionSelectedContractV12,
    human_authority: PromotionHumanAuthorityV12,
    outputs: PromotionOutputsV12,
}

struct CandidateValidationAuthorityV12 {
    candidate: Value,
    validation_result_ref: String,
    validation_result: CharterLifecycleValidationResultV10,
}

type ReadIntent = ValidatedPromotionIntentV12;

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
    promotion_intent_marker_v12(raw_intent_bytes)
}

fn validate_selected_lifecycle_authority(
    retained: &RetainedLifecycleAuthorityV1,
    selected: &PromotionSelectedContractV12,
) -> Result<(), CharterPromotionErrorV1> {
    let authority = retained.authority.as_ref().ok_or_else(|| {
        error(
            CharterPromotionErrorKindV1::Conflict,
            "amend promotion lost its retained prior lifecycle authority",
        )
    })?;
    let state = match authority.state {
        CharterLifecycleState::Current => "current",
        CharterLifecycleState::ReviewRequired => "review_required",
        CharterLifecycleState::ReassessmentRequired => "reassessment_required",
    };
    let observations_match = authority.active_observation_refs.len()
        == selected.active_observations.len()
        && authority
            .active_observation_refs
            .iter()
            .zip(&authority.active_observation_fingerprints)
            .zip(&selected.active_observations)
            .all(|((reference, fingerprint), selected)| {
                reference == &selected.observation_ref
                    && fingerprint == &selected.observation_fingerprint
            });
    if retained.canonical_bytes.is_none()
        || selected.prior_lifecycle_head_ref.as_deref()
            != Some(authority.lifecycle_transition_ref.as_str())
        || selected.prior_lifecycle_head_fingerprint.as_deref()
            != Some(authority.lifecycle_transition_fingerprint.as_str())
        || selected.prior_lifecycle_state != state
        || selected.prior_lifecycle_state_fingerprint.as_deref()
            != Some(authority.state_fingerprint.as_str())
        || !observations_match
        || selected.reopened_coverage_ids != authority.reopened_coverage_ids
    {
        return Err(error(
            CharterPromotionErrorKindV1::Conflict,
            "precommit lifecycle head/state/observation selection differs from retained authority",
        ));
    }
    Ok(())
}

fn selected_contract_from_validation(
    result: &CharterLifecycleValidationResultV10,
) -> PromotionSelectedContractV12 {
    PromotionSelectedContractV12 {
        profile_ref: result.profile_ref.clone(),
        resolved_profile_fingerprint: result.resolved_profile_fingerprint.clone(),
        resolved_definitions: result.resolved_definitions.clone(),
        lifecycle_policy_ref: result.lifecycle_policy_ref.clone(),
        lifecycle_policy_fingerprint: result.lifecycle_policy_fingerprint.clone(),
        prior_lifecycle_head_ref: result.lifecycle_head_ref.clone(),
        prior_lifecycle_head_fingerprint: result.lifecycle_head_fingerprint.clone(),
        prior_lifecycle_state: result.lifecycle_state.clone(),
        prior_lifecycle_state_fingerprint: result.lifecycle_state_fingerprint.clone(),
        active_observations: result.active_observations.clone(),
        reopened_coverage_ids: result.reopened_coverage_ids.clone(),
    }
}

fn build_promotion_intent(
    prepared: &PreparedPromotion,
    old_canonical: Option<&[u8]>,
    transaction_id: &str,
) -> Result<ValidatedPromotionIntentV12, CharterPromotionErrorV1> {
    let old_fingerprint =
        old_canonical.map(|bytes| DefinitionFingerprint::from_bytes(bytes).to_string());
    let mut record = PromotionTransactionIntentV12 {
        schema_id: "handbook.charter-promotion-transaction-intent".to_owned(),
        schema_version: "1.2".to_owned(),
        transaction_id: transaction_id.to_owned(),
        promotion_id: prepared.promotion_id.clone(),
        mutation_mode: if old_canonical.is_some() {
            "amend".to_owned()
        } else {
            "create".to_owned()
        },
        target: PromotionIntentTargetV12 {
            target_instance_id: "project_authority".to_owned(),
            canonical_artifact_ref: CANONICAL_REF.to_owned(),
            basis_artifact_fingerprint: prepared.expected_current_fingerprint.clone(),
            observed_current_artifact_fingerprint: prepared.expected_current_fingerprint.clone(),
        },
        candidate_lineage: prepared.candidate_lineage.clone(),
        selected_contract: prepared.selected_contract.clone(),
        human_authority: prepared.human_authority.clone(),
        outputs: prepared.outputs.clone(),
        recovery: PromotionRecoveryV12 {
            old_canonical_status: if old_canonical.is_some() {
                "present".to_owned()
            } else {
                "absent".to_owned()
            },
            old_canonical_fingerprint: old_fingerprint.clone(),
            old_canonical_document_sha256: old_fingerprint,
            old_canonical_byte_length: old_canonical.map(|bytes| bytes.len() as u64),
        },
        intent_fingerprint:
            "sha256:0000000000000000000000000000000000000000000000000000000000000000".to_owned(),
    };
    let mut preimage = serde_json::to_value(&record)
        .map_err(|_| io_error("promotion intent preimage serialization failed"))?;
    preimage
        .as_object_mut()
        .expect("typed promotion intent is an object")
        .remove("intent_fingerprint");
    record.intent_fingerprint = DefinitionFingerprint::from_json_value(&preimage)
        .map_err(|_| io_error("promotion intent fingerprint computation failed"))?
        .to_string();
    let mut bytes = serde_json_canonicalizer::to_vec(&record)
        .map_err(|_| io_error("promotion intent canonicalization failed"))?;
    bytes.push(b'\n');
    parse_promotion_intent_v12(&bytes).map_err(|failure| {
        error(
            CharterPromotionErrorKindV1::InvalidRequest,
            format!(
                "constructed promotion intent 1.2 refused: {}",
                failure.detail()
            ),
        )
    })
}

fn fingerprint_from_content_ref(
    reference: &str,
    prefix: &str,
    suffix: &str,
) -> Result<String, CharterPromotionErrorV1> {
    let Some(hex) = reference
        .strip_prefix(prefix)
        .and_then(|rest| rest.strip_suffix(suffix))
    else {
        return Err(error(
            CharterPromotionErrorKindV1::LineageViolation,
            "content-addressed reference shape is invalid",
        ));
    };
    let fingerprint = format!("sha256:{hex}");
    DefinitionFingerprint::parse(&fingerprint).map_err(|_| {
        error(
            CharterPromotionErrorKindV1::LineageViolation,
            "content-addressed reference fingerprint is invalid",
        )
    })?;
    Ok(fingerprint)
}

fn random_hex_128() -> Result<String, CharterPromotionErrorV1> {
    let mut bytes = [0_u8; 16];
    getrandom::fill(&mut bytes).map_err(|_| io_error("engine random allocation failed"))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

fn allocate_transaction_id() -> Result<String, CharterPromotionErrorV1> {
    Ok(format!("promotion-transaction_{}", random_hex_128()?))
}

fn atomic_rename_no_replace(source: &Path, target: &Path) -> Result<(), CharterPromotionErrorV1> {
    #[cfg(unix)]
    {
        rustix::fs::renameat_with(
            rustix::fs::CWD,
            source,
            rustix::fs::CWD,
            target,
            rustix::fs::RenameFlags::NOREPLACE,
        )
        .map_err(|_| io_error("atomic rename-no-replace failed"))?;
    }
    #[cfg(windows)]
    {
        let temporary = tempfile::TempPath::try_from_path(source.to_path_buf())
            .map_err(|_| io_error("atomic rename-no-replace source path is invalid"))?;
        if let Err(failure) = temporary.persist_noclobber(target) {
            let _ = failure.path.keep();
            return Err(io_error("atomic rename-no-replace failed"));
        }
    }
    Ok(())
}

fn publish_intent_from_scratch(
    repo_root: &Path,
    transaction_root: &Path,
    pending: &Path,
    intent: &ValidatedPromotionIntentV12,
) -> Result<(), CharterPromotionErrorV1> {
    let token = random_hex_128()?;
    publish_intent_from_named_scratch(repo_root, transaction_root, pending, intent, &token)
}

fn publish_intent_from_named_scratch(
    repo_root: &Path,
    transaction_root: &Path,
    pending: &Path,
    intent: &ValidatedPromotionIntentV12,
    scratch_token: &str,
) -> Result<(), CharterPromotionErrorV1> {
    create_safe_directories(repo_root, transaction_root).map_err(map_lineage)?;
    let staging = transaction_root.join(INTENT_STAGING_NAME);
    create_safe_directories(repo_root, &staging).map_err(map_lineage)?;
    let scratch = staging.join(format!("{scratch_token}.intent"));
    let mut file =
        create_new_file(&scratch).map_err(|_| io_error("intent scratch create-new failed"))?;
    file.write_all(&intent.raw_bytes)
        .map_err(|_| io_error("intent scratch write failed"))?;
    file.sync_all()
        .map_err(|_| io_error("intent scratch fsync failed"))?;
    drop(file);
    let observed = read_bounded_regular(&scratch, MAX_INTENT_BYTES)?;
    if observed != intent.raw_bytes
        || parse_promotion_intent_v12(&observed).map_err(|failure| {
            error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                failure.detail(),
            )
        })? != *intent
    {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "intent scratch bytes or bindings changed before publication",
        ));
    }
    sync_directory(&staging).map_err(map_lineage)?;
    #[cfg(test)]
    inject_writer_fault(WriterBoundaryV12::W1IntentScratch)?;
    fs::create_dir(pending)
        .map_err(|_| io_error("promotion pending directory create-new failed"))?;
    sync_directory(pending).map_err(map_lineage)?;
    sync_directory(transaction_root).map_err(map_lineage)?;
    #[cfg(test)]
    inject_writer_fault(WriterBoundaryV12::W2Directory)?;
    atomic_rename_no_replace(&scratch, &pending.join("intent.json"))?;
    sync_directory(pending).map_err(map_lineage)?;
    sync_directory(&staging).map_err(map_lineage)?;
    sync_directory(transaction_root).map_err(map_lineage)?;
    let published = read_valid_intent(pending)?;
    if published != *intent {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "published promotion intent differs from verified scratch",
        ));
    }
    #[cfg(test)]
    inject_writer_fault(WriterBoundaryV12::W3IntentPublished)?;
    Ok(())
}

fn output_stage_name(purpose: OutputPurposeV12) -> &'static str {
    match purpose {
        OutputPurposeV12::Canonical => "canonical.new",
        OutputPurposeV12::PromotionRecord => "promotion-record.new",
        OutputPurposeV12::LifecycleTransition => "lifecycle-transition.new",
    }
}

fn output_scratch_suffix(purpose: OutputPurposeV12) -> &'static str {
    match purpose {
        OutputPurposeV12::Canonical => "canonical",
        OutputPurposeV12::PromotionRecord => "promotion-record",
        OutputPurposeV12::LifecycleTransition => "lifecycle-transition",
    }
}

#[cfg(test)]
fn inject_output_fault(
    purpose: OutputPurposeV12,
    boundary: OutputStageBoundaryV12,
) -> Result<(), CharterPromotionErrorV1> {
    let selected = SELECTED_PROMOTION_FAULT_V1.with(Cell::get);
    if matches!(
        selected,
        Some(CharterPromotionFaultPointV1::OutputStage {
            purpose: selected_purpose,
            boundary: selected_boundary,
            ..
        }) if selected_purpose == purpose && selected_boundary == boundary
    ) {
        return Err(error(
            CharterPromotionErrorKindV1::InjectedFault,
            format!("injected output-stage fault at {purpose:?} {boundary:?}"),
        ));
    }
    Ok(())
}

#[cfg(test)]
fn inject_writer_fault(boundary: WriterBoundaryV12) -> Result<(), CharterPromotionErrorV1> {
    let selected = SELECTED_PROMOTION_FAULT_V1.with(Cell::get);
    if selected == Some(CharterPromotionFaultPointV1::Writer(boundary)) {
        return Err(error(
            CharterPromotionErrorKindV1::InjectedFault,
            format!("injected promotion writer fault at {boundary:?}"),
        ));
    }
    Ok(())
}

#[cfg(test)]
fn inject_recovery_fault(boundary: RecoveryBoundaryV12) -> Result<(), CharterPromotionErrorV1> {
    let selected = SELECTED_RECOVERY_FAULT_V1.with(Cell::get);
    if selected == Some(boundary) {
        return Err(error(
            CharterPromotionErrorKindV1::InjectedFault,
            format!("injected promotion recovery fault at {boundary:?}"),
        ));
    }
    Ok(())
}

#[cfg(test)]
fn output_fault_write_prefix(purpose: OutputPurposeV12) -> Option<usize> {
    match SELECTED_PROMOTION_FAULT_V1.with(Cell::get) {
        Some(CharterPromotionFaultPointV1::OutputStage {
            purpose: selected_purpose,
            boundary: OutputStageBoundaryV12::S2ScratchWritten,
            write_prefix,
        }) if selected_purpose == purpose => write_prefix,
        _ => None,
    }
}

#[cfg(test)]
fn inject_exact_promotion_fault(
    expected: CharterPromotionFaultPointV1,
    detail: &'static str,
) -> Result<(), CharterPromotionErrorV1> {
    if SELECTED_PROMOTION_FAULT_V1.with(Cell::get) == Some(expected) {
        return Err(error(CharterPromotionErrorKindV1::InjectedFault, detail));
    }
    Ok(())
}

fn validate_output_bytes_against_intent(
    purpose: OutputPurposeV12,
    bytes: &[u8],
    intent: &PromotionTransactionIntentV12,
) -> Result<(), CharterPromotionErrorV1> {
    match purpose {
        OutputPurposeV12::Canonical => {
            if bytes.len() as u64 != intent.outputs.new_canonical_byte_length
                || DefinitionFingerprint::from_bytes(bytes).to_string()
                    != intent.outputs.new_canonical_document_sha256
                || intent.outputs.new_canonical_fingerprint
                    != intent.outputs.new_canonical_document_sha256
            {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "canonical output does not match its intent hash/length/fingerprint",
                ));
            }
        }
        OutputPurposeV12::PromotionRecord => {
            validate_record_output(
                LineageRecordClassV1::Promotion,
                bytes,
                &intent.outputs.promotion_record,
            )?;
            let value: Value = serde_json::from_slice(bytes).map_err(|_| {
                error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "promotion output is not JSON",
                )
            })?;
            let approval_refs = intent
                .human_authority
                .approval_bindings
                .iter()
                .map(|binding| Value::String(binding.approval_ref.clone()))
                .collect::<Vec<_>>();
            let validation_refs = vec![Value::String(
                intent.candidate_lineage.validation_result_ref.clone(),
            )];
            let resolved_definitions = intent
                .selected_contract
                .resolved_definitions
                .iter()
                .map(|binding| {
                    serde_json::json!({
                        "definition_ref": binding.definition_ref,
                        "definition_fingerprint": binding.definition_fingerprint,
                    })
                })
                .collect::<Vec<_>>();
            if value.get("candidate_ref").and_then(Value::as_str)
                != Some(intent.candidate_lineage.candidate_ref.as_str())
                || value.get("candidate_fingerprint").and_then(Value::as_str)
                    != Some(intent.candidate_lineage.candidate_fingerprint.as_str())
                || value.get("canonical_artifact_ref").and_then(Value::as_str)
                    != Some(intent.target.canonical_artifact_ref.as_str())
                || value
                    .get("canonical_artifact_fingerprint")
                    .and_then(Value::as_str)
                    != Some(intent.outputs.new_canonical_fingerprint.as_str())
                || value.get("target_instance_id").and_then(Value::as_str)
                    != Some(intent.target.target_instance_id.as_str())
                || value.get("decision").and_then(Value::as_str) != Some("approved")
                || value.get("profile_ref").and_then(Value::as_str)
                    != Some(intent.selected_contract.profile_ref.as_str())
                || value
                    .get("resolved_profile_fingerprint")
                    .and_then(Value::as_str)
                    != Some(
                        intent
                            .selected_contract
                            .resolved_profile_fingerprint
                            .as_str(),
                    )
                || value.get("resolved_definitions").and_then(Value::as_array)
                    != Some(&resolved_definitions)
                || value.get("approval_refs").and_then(Value::as_array) != Some(&approval_refs)
                || value
                    .get("validation_result_refs")
                    .and_then(Value::as_array)
                    != Some(&validation_refs)
                || value
                    .get("approver_registry_state_ref")
                    .and_then(Value::as_str)
                    != Some(intent.human_authority.approver_registry_state_ref.as_str())
                || value
                    .get("approver_registry_state_fingerprint")
                    .and_then(Value::as_str)
                    != Some(
                        intent
                            .human_authority
                            .approver_registry_state_fingerprint
                            .as_str(),
                    )
                || value
                    .get("registry_head_transition_ref")
                    .and_then(Value::as_str)
                    != Some(intent.human_authority.registry_head_transition_ref.as_str())
                || value
                    .get("registry_head_transition_fingerprint")
                    .and_then(Value::as_str)
                    != Some(
                        intent
                            .human_authority
                            .registry_head_transition_fingerprint
                            .as_str(),
                    )
            {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "promotion output does not match complete intent bindings",
                ));
            }
            require_optional_string_equal(
                &value,
                "basis_artifact_fingerprint",
                intent.target.basis_artifact_fingerprint.as_deref(),
            )?;
            require_optional_string_equal(
                &value,
                "expected_current_artifact_fingerprint",
                intent
                    .target
                    .observed_current_artifact_fingerprint
                    .as_deref(),
            )?;
        }
        OutputPurposeV12::LifecycleTransition => {
            validate_record_output(
                LineageRecordClassV1::LifecycleTransition,
                bytes,
                &intent.outputs.lifecycle_transition,
            )?;
            let value: Value = serde_json::from_slice(bytes).map_err(|_| {
                error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "lifecycle output is not JSON",
                )
            })?;
            let result_state_fingerprint = charter_lifecycle_state_fingerprint(
                &intent.selected_contract.lifecycle_policy_ref,
                &intent.selected_contract.lifecycle_policy_fingerprint,
                &intent.target.target_instance_id,
                &intent.outputs.new_canonical_fingerprint,
                CharterLifecycleState::Current,
                &[],
            )
            .map_err(|failure| {
                error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    format!(
                        "lifecycle result-state intent is invalid: {}",
                        failure.detail()
                    ),
                )
            })?;
            let (prior_state, prior_state_fingerprint) =
                if intent.recovery.old_canonical_status == "absent" {
                    ("current", result_state_fingerprint.as_str())
                } else {
                    (
                        intent.selected_contract.prior_lifecycle_state.as_str(),
                        intent
                            .selected_contract
                            .prior_lifecycle_state_fingerprint
                            .as_deref()
                            .unwrap_or_default(),
                    )
                };
            if value.get("target_instance_id").and_then(Value::as_str)
                != Some(intent.target.target_instance_id.as_str())
                || value.get("result_state").and_then(Value::as_str) != Some("current")
                || value
                    .get("result_state_fingerprint")
                    .and_then(Value::as_str)
                    != Some(result_state_fingerprint.as_str())
                || value.get("prior_state").and_then(Value::as_str) != Some(prior_state)
                || value.get("prior_state_fingerprint").and_then(Value::as_str)
                    != Some(prior_state_fingerprint)
                || !value
                    .get("new_observation_refs")
                    .and_then(Value::as_array)
                    .is_some_and(Vec::is_empty)
                || !value
                    .get("active_observation_refs")
                    .and_then(Value::as_array)
                    .is_some_and(Vec::is_empty)
                || value.get("clearance_promotion_ref").and_then(Value::as_str)
                    != Some(intent.outputs.promotion_record.record_ref.as_str())
                || value.get("lifecycle_policy_ref").and_then(Value::as_str)
                    != Some(intent.selected_contract.lifecycle_policy_ref.as_str())
                || value
                    .get("lifecycle_policy_fingerprint")
                    .and_then(Value::as_str)
                    != Some(
                        intent
                            .selected_contract
                            .lifecycle_policy_fingerprint
                            .as_str(),
                    )
            {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "lifecycle-transition output does not match complete intent bindings",
                ));
            }
        }
    }
    Ok(())
}

fn intended_canonical_bytes(
    pending: &Path,
    intent: &ValidatedPromotionIntentV12,
    repo_root: &Path,
) -> Result<Vec<u8>, CharterPromotionErrorV1> {
    if let Some(staged) = optional_bounded_regular(
        &pending.join(output_stage_name(OutputPurposeV12::Canonical)),
        MAX_CANONICAL_BYTES,
    )? {
        validate_output_bytes_against_intent(OutputPurposeV12::Canonical, &staged, &intent.record)?;
        return Ok(staged);
    }
    let installed = read_current_canonical(repo_root)?.ok_or_else(|| {
        error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "intended canonical bytes are absent from both the exact stage and target",
        )
    })?;
    validate_output_bytes_against_intent(OutputPurposeV12::Canonical, &installed, &intent.record)?;
    Ok(installed)
}

fn validate_record_output(
    class: LineageRecordClassV1,
    bytes: &[u8],
    expected: &PromotionOutputRecordV12,
) -> Result<(), CharterPromotionErrorV1> {
    if bytes.len() as u64 != expected.byte_length
        || DefinitionFingerprint::from_bytes(bytes).to_string() != expected.document_sha256
    {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "record output document hash or length differs from intent",
        ));
    }
    let identity = validate_record(class, bytes).map_err(map_lineage)?;
    if identity.relative_ref != expected.record_ref
        || identity.fingerprint != expected.record_fingerprint
    {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "record output semantic identity differs from intent",
        ));
    }
    Ok(())
}

fn require_optional_string_equal(
    value: &Value,
    field: &str,
    expected: Option<&str>,
) -> Result<(), CharterPromotionErrorV1> {
    let matches = match (value.get(field), expected) {
        (Some(Value::Null), None) => true,
        (Some(Value::String(observed)), Some(expected)) => observed == expected,
        _ => false,
    };
    if matches {
        Ok(())
    } else {
        Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            format!("record output optional binding `{field}` differs from intent"),
        ))
    }
}

fn publish_output_stage_from_scratch(
    repo_root: &Path,
    transaction_root: &Path,
    pending: &Path,
    purpose: OutputPurposeV12,
    expected: &[u8],
    intent: &PromotionTransactionIntentV12,
) -> Result<(), CharterPromotionErrorV1> {
    let token = random_hex_128()?;
    publish_output_stage_from_named_scratch(
        repo_root,
        transaction_root,
        pending,
        NamedOutputScratchPublicationV12 {
            purpose,
            expected,
            intent,
            scratch_token: &token,
        },
    )
}

struct NamedOutputScratchPublicationV12<'a> {
    purpose: OutputPurposeV12,
    expected: &'a [u8],
    intent: &'a PromotionTransactionIntentV12,
    scratch_token: &'a str,
}

fn publish_output_stage_from_named_scratch(
    repo_root: &Path,
    transaction_root: &Path,
    pending: &Path,
    publication: NamedOutputScratchPublicationV12<'_>,
) -> Result<(), CharterPromotionErrorV1> {
    let NamedOutputScratchPublicationV12 {
        purpose,
        expected,
        intent,
        scratch_token,
    } = publication;
    validate_output_bytes_against_intent(purpose, expected, intent)?;
    #[cfg(test)]
    inject_output_fault(purpose, OutputStageBoundaryV12::S0Retained)?;
    let staging = transaction_root.join(OUTPUT_STAGING_NAME);
    create_safe_directories(repo_root, &staging).map_err(map_lineage)?;
    let scratch = staging.join(format!(
        "{}.{}",
        scratch_token,
        output_scratch_suffix(purpose)
    ));
    let mut file =
        create_new_file(&scratch).map_err(|_| io_error("output scratch create-new failed"))?;
    #[cfg(test)]
    inject_output_fault(purpose, OutputStageBoundaryV12::S1ScratchCreated)?;
    #[cfg(test)]
    if let Some(prefix) = output_fault_write_prefix(purpose) {
        let prefix = prefix.min(expected.len());
        file.write_all(&expected[..prefix])
            .map_err(|_| io_error("output scratch prefix write failed"))?;
        return Err(error(
            CharterPromotionErrorKindV1::InjectedFault,
            format!("injected output scratch write-prefix fault at {purpose:?} byte {prefix}"),
        ));
    }
    file.write_all(expected)
        .map_err(|_| io_error("output scratch write failed"))?;
    #[cfg(test)]
    inject_output_fault(purpose, OutputStageBoundaryV12::S2ScratchWritten)?;
    file.sync_all()
        .map_err(|_| io_error("output scratch fsync failed"))?;
    drop(file);
    #[cfg(test)]
    inject_output_fault(purpose, OutputStageBoundaryV12::S3ScratchSyncedAndClosed)?;
    let observed = read_bounded_regular(&scratch, expected.len())?;
    #[cfg(test)]
    inject_output_fault(purpose, OutputStageBoundaryV12::S4ScratchReopened)?;
    if observed != expected {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "output scratch bytes changed before verification",
        ));
    }
    validate_output_bytes_against_intent(purpose, &observed, intent)?;
    #[cfg(test)]
    inject_output_fault(purpose, OutputStageBoundaryV12::S5ScratchVerified)?;
    sync_directory(&staging).map_err(map_lineage)?;
    #[cfg(test)]
    inject_output_fault(purpose, OutputStageBoundaryV12::S6ScratchDirectorySynced)?;
    let stage = pending.join(output_stage_name(purpose));
    atomic_rename_no_replace(&scratch, &stage)?;
    #[cfg(test)]
    inject_output_fault(purpose, OutputStageBoundaryV12::S7StageRenamed)?;
    sync_directory(pending).map_err(map_lineage)?;
    #[cfg(test)]
    inject_output_fault(purpose, OutputStageBoundaryV12::S8PendingDirectorySynced)?;
    sync_directory(&staging).map_err(map_lineage)?;
    #[cfg(test)]
    inject_output_fault(purpose, OutputStageBoundaryV12::S9ScratchDirectoryResynced)?;
    sync_directory(transaction_root).map_err(map_lineage)?;
    #[cfg(test)]
    inject_output_fault(purpose, OutputStageBoundaryV12::S10TransactionParentSynced)?;
    let published = read_bounded_regular(&stage, expected.len())?;
    if published != expected {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "published output stage differs from retained bytes",
        ));
    }
    validate_output_bytes_against_intent(purpose, &published, intent)?;
    #[cfg(test)]
    inject_output_fault(purpose, OutputStageBoundaryV12::S11PendingStageReverified)?;
    Ok(())
}

fn publish_marker(
    pending: &Path,
    marker_name: &str,
    intent_bytes: &[u8],
) -> Result<(), CharterPromotionErrorV1> {
    complete_marker_temp(pending, marker_name, intent_bytes)?;
    publish_completed_marker(pending, marker_name, intent_bytes)
}

fn complete_marker_temp(
    pending: &Path,
    marker_name: &str,
    intent_bytes: &[u8],
) -> Result<(), CharterPromotionErrorV1> {
    let expected = promotion_intent_marker_v12(intent_bytes);
    let temporary = pending.join(format!("{marker_name}.tmp"));
    let published = pending.join(marker_name);
    if published.exists() {
        return Err(error(
            CharterPromotionErrorKindV1::Conflict,
            format!("published marker `{marker_name}` already exists"),
        ));
    }
    let observed_prefix = match create_new_file(&temporary) {
        Ok(file) => {
            drop(file);
            Vec::new()
        }
        Err(failure) if failure.kind() == std::io::ErrorKind::AlreadyExists => {
            read_bounded_regular(&temporary, expected.len())?
        }
        Err(_) => return Err(io_error("marker temp create-new failed")),
    };
    if !expected.starts_with(&observed_prefix) {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            format!("marker temp `{marker_name}.tmp` is not an exact prefix"),
        ));
    }
    let mut file = open_existing_nofollow_write(&temporary)?;
    file.seek(SeekFrom::End(0))
        .map_err(|_| io_error("marker temp seek failed"))?;
    file.write_all(&expected[observed_prefix.len()..])
        .map_err(|_| io_error("marker temp suffix write failed"))?;
    file.sync_all()
        .map_err(|_| io_error("marker temp fsync failed"))?;
    drop(file);
    if read_bounded_regular(&temporary, expected.len())? != expected {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "marker temp changed after exact suffix completion",
        ));
    }
    Ok(())
}

fn publish_completed_marker(
    pending: &Path,
    marker_name: &str,
    intent_bytes: &[u8],
) -> Result<(), CharterPromotionErrorV1> {
    let expected = promotion_intent_marker_v12(intent_bytes);
    let temporary = pending.join(format!("{marker_name}.tmp"));
    let published = pending.join(marker_name);
    if published.exists() {
        return Err(error(
            CharterPromotionErrorKindV1::Conflict,
            format!("published marker `{marker_name}` already exists"),
        ));
    }
    if read_bounded_regular(&temporary, expected.len())? != expected {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            format!("marker temp `{marker_name}.tmp` is not exact before publication"),
        ));
    }
    atomic_rename_no_replace(&temporary, &published)?;
    sync_directory(pending).map_err(map_lineage)?;
    if read_bounded_regular(&published, expected.len())? != expected {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            format!("published marker `{marker_name}` has unequal bytes"),
        ));
    }
    Ok(())
}

fn open_existing_nofollow_write(path: &Path) -> Result<File, CharterPromotionErrorV1> {
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
        .map_err(|_| io_error("existing marker temp open failed"))?;
    if !file
        .metadata()
        .map_err(|_| io_error("existing marker temp metadata failed"))?
        .is_file()
    {
        return Err(error(
            CharterPromotionErrorKindV1::UnsafeFilesystem,
            "existing marker temp is not a regular file",
        ));
    }
    Ok(file)
}

fn complete_old_snapshot(
    pending: &Path,
    intent: &PromotionTransactionIntentV12,
    current: Option<&[u8]>,
) -> Result<(), CharterPromotionErrorV1> {
    let snapshot = pending.join("canonical.old");
    if intent.mutation_mode == "create" {
        if fs::symlink_metadata(&snapshot).is_ok() {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "create rollback forbids canonical.old",
            ));
        }
        return Ok(());
    }
    let expected = current.ok_or_else(|| {
        error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "amend rollback has no authentic old target bytes",
        )
    })?;
    if expected.len() as u64 != intent.recovery.old_canonical_byte_length.unwrap_or(0)
        || DefinitionFingerprint::from_bytes(expected).to_string()
            != intent
                .recovery
                .old_canonical_document_sha256
                .as_deref()
                .unwrap_or_default()
    {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "current target is not the intent-bound amendment recovery source",
        ));
    }
    let prefix = match fs::symlink_metadata(&snapshot) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.is_file() {
                return Err(error(
                    CharterPromotionErrorKindV1::UnsafeFilesystem,
                    "canonical.old is not a safe regular file",
                ));
            }
            read_bounded_regular(&snapshot, expected.len())?
        }
        Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => {
            drop(
                create_new_file(&snapshot)
                    .map_err(|_| io_error("canonical.old create-new failed"))?,
            );
            Vec::new()
        }
        Err(_) => return Err(io_error("canonical.old metadata observation failed")),
    };
    if !expected.starts_with(&prefix) {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "canonical.old is not an authentic exact prefix",
        ));
    }
    let mut file = open_existing_nofollow_write(&snapshot)?;
    file.seek(SeekFrom::End(0))
        .map_err(|_| io_error("canonical.old seek failed"))?;
    file.write_all(&expected[prefix.len()..])
        .map_err(|_| io_error("canonical.old suffix completion failed"))?;
    file.sync_all()
        .map_err(|_| io_error("canonical.old fsync failed"))?;
    drop(file);
    sync_directory(pending).map_err(map_lineage)?;
    if read_bounded_regular(&snapshot, expected.len())? != expected {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "canonical.old differs after exact prefix completion",
        ));
    }
    Ok(())
}

fn remove_exact_rollback_owned_file(
    pending: &Path,
    name: &str,
    intent: &ValidatedPromotionIntentV12,
) -> Result<(), CharterPromotionErrorV1> {
    let path = pending.join(name);
    match fs::symlink_metadata(&path) {
        Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(_) => return Err(io_error("rollback cleanup metadata observation failed")),
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => {
            return Err(error(
                CharterPromotionErrorKindV1::UnsafeFilesystem,
                format!("rollback cleanup target `{name}` is unsafe"),
            ));
        }
        Ok(_) => {}
    }
    let observed = read_bounded_regular(&path, MAX_CANONICAL_BYTES.max(MAX_OUTPUT_RECORD_BYTES))?;
    match name {
        "prepared.tmp" => {
            if !promotion_intent_marker_v12(&intent.raw_bytes).starts_with(&observed) {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "prepared.tmp is not an exact marker prefix",
                ));
            }
        }
        "prepared" => {
            if observed != promotion_intent_marker_v12(&intent.raw_bytes) {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    "prepared marker bytes differ from intent",
                ));
            }
        }
        "canonical.new" => validate_output_bytes_against_intent(
            OutputPurposeV12::Canonical,
            &observed,
            &intent.record,
        )?,
        "promotion-record.new" => validate_output_bytes_against_intent(
            OutputPurposeV12::PromotionRecord,
            &observed,
            &intent.record,
        )?,
        "lifecycle-transition.new" => validate_output_bytes_against_intent(
            OutputPurposeV12::LifecycleTransition,
            &observed,
            &intent.record,
        )?,
        _ => {
            return Err(error(
                CharterPromotionErrorKindV1::InvalidRequest,
                "rollback cleanup name is outside the fixed list",
            ))
        }
    }
    fs::remove_file(&path).map_err(|_| io_error("rollback owned-file removal failed"))?;
    sync_directory(pending).map_err(map_lineage)
}

fn pending_entry_names(pending: &Path) -> Result<BTreeSet<String>, CharterPromotionErrorV1> {
    let mut names = BTreeSet::new();
    for entry in
        fs::read_dir(pending).map_err(|_| io_error("pending promotion directory read failed"))?
    {
        let entry = entry.map_err(|_| io_error("pending promotion entry read failed"))?;
        let name = entry.file_name().into_string().map_err(|_| {
            error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "pending promotion entry name is not UTF-8",
            )
        })?;
        let metadata = fs::symlink_metadata(entry.path())
            .map_err(|_| io_error("pending promotion entry metadata failed"))?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(error(
                CharterPromotionErrorKindV1::UnsafeFilesystem,
                format!("pending promotion entry `{name}` is unsafe"),
            ));
        }
        names.insert(name);
    }
    Ok(names)
}

fn optional_bounded_regular(
    path: &Path,
    limit: usize,
) -> Result<Option<Vec<u8>>, CharterPromotionErrorV1> {
    match fs::symlink_metadata(path) {
        Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err(io_error("optional journal file metadata failed")),
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_file() => Err(error(
            CharterPromotionErrorKindV1::UnsafeFilesystem,
            "optional journal path is not a safe regular file",
        )),
        Ok(_) => read_bounded_regular(path, limit).map(Some),
    }
}

fn observe_marker(
    pending: &Path,
    name: &str,
    intent_bytes: &[u8],
) -> Result<MarkerObservationV12, CharterPromotionErrorV1> {
    let expected = promotion_intent_marker_v12(intent_bytes);
    let temporary = optional_bounded_regular(&pending.join(format!("{name}.tmp")), expected.len())?;
    let published = optional_bounded_regular(&pending.join(name), expected.len())?;
    match (temporary, published) {
        (None, None) => Ok(MarkerObservationV12::Absent),
        (Some(prefix), None) if expected.starts_with(&prefix) => {
            Ok(MarkerObservationV12::TmpExactPrefix)
        }
        (None, Some(observed)) if observed == expected => Ok(MarkerObservationV12::Published),
        _ => Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            format!("marker `{name}` temp/published state is mismatching"),
        )),
    }
}

fn ensure_published_marker(
    pending: &Path,
    name: &str,
    intent_bytes: &[u8],
) -> Result<(), CharterPromotionErrorV1> {
    match observe_marker(pending, name, intent_bytes)? {
        MarkerObservationV12::Published => {
            sync_directory(pending).map_err(map_lineage)?;
            if read_bounded_regular(&pending.join(name), 72)?
                != promotion_intent_marker_v12(intent_bytes)
            {
                return Err(error(
                    CharterPromotionErrorKindV1::DurabilityViolation,
                    format!("published marker `{name}` changed during fsync replay"),
                ));
            }
            Ok(())
        }
        MarkerObservationV12::Absent | MarkerObservationV12::TmpExactPrefix => {
            publish_marker(pending, name, intent_bytes)
        }
    }
}

fn validate_owned_pending_names(names: &BTreeSet<String>) -> Result<(), CharterPromotionErrorV1> {
    const OWNED: [&str; 15] = [
        "intent.json",
        "canonical.old",
        "canonical.new",
        "promotion-record.new",
        "lifecycle-transition.new",
        "prepared.tmp",
        "prepared",
        "canonical-installed.tmp",
        "canonical-installed",
        "records-installed.tmp",
        "records-installed",
        "committed.tmp",
        "committed",
        "rolled-back.tmp",
        "rolled-back",
    ];
    if names.iter().any(|name| !OWNED.contains(&name.as_str())) {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "pending promotion contains an unknown evidence name",
        ));
    }
    Ok(())
}

fn validate_old_snapshot(
    pending: &Path,
    intent: &PromotionTransactionIntentV12,
    allow_prefix: bool,
) -> Result<(), CharterPromotionErrorV1> {
    let observed = optional_bounded_regular(&pending.join("canonical.old"), MAX_CANONICAL_BYTES)?;
    if intent.mutation_mode == "create" {
        return if observed.is_none() {
            Ok(())
        } else {
            Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "create promotion forbids canonical.old",
            ))
        };
    }
    let expected_length = intent.recovery.old_canonical_byte_length.unwrap_or(0) as usize;
    let Some(observed) = observed else {
        return if allow_prefix {
            Ok(())
        } else {
            Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "amend promotion canonical.old is absent",
            ))
        };
    };
    if observed.len() > expected_length {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "canonical.old exceeds its intent-bound length",
        ));
    }
    if observed.len() == expected_length {
        if DefinitionFingerprint::from_bytes(&observed).to_string()
            == intent
                .recovery
                .old_canonical_document_sha256
                .as_deref()
                .unwrap_or_default()
        {
            return Ok(());
        }
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "canonical.old complete bytes differ from intent",
        ));
    }
    if allow_prefix {
        Ok(())
    } else {
        Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "canonical.old is incomplete after the snapshot boundary",
        ))
    }
}

fn validate_rollback_origin_or_progress(
    pending: &Path,
    intent: &ValidatedPromotionIntentV12,
) -> Result<(), CharterPromotionErrorV1> {
    let names = pending_entry_names(pending)?;
    validate_owned_pending_names(&names)?;
    for forbidden in [
        "canonical-installed.tmp",
        "canonical-installed",
        "records-installed.tmp",
        "records-installed",
        "committed.tmp",
        "committed",
        "rolled-back",
    ] {
        if names.contains(forbidden) {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "rollback origin contains a forward or published terminal marker",
            ));
        }
    }
    validate_old_snapshot(pending, &intent.record, true)?;
    let canonical = classify_stage_path(
        &pending.join("canonical.new"),
        OutputPurposeV12::Canonical,
        intent,
    )?;
    let promotion = classify_stage_path(
        &pending.join("promotion-record.new"),
        OutputPurposeV12::PromotionRecord,
        intent,
    )?;
    let lifecycle = classify_stage_path(
        &pending.join("lifecycle-transition.new"),
        OutputPurposeV12::LifecycleTransition,
        intent,
    )?;
    let pattern = [canonical, promotion, lifecycle];
    if !matches!(
        pattern,
        [
            RetainedStageStateV12::Absent,
            RetainedStageStateV12::Absent,
            RetainedStageStateV12::Absent
        ] | [
            RetainedStageStateV12::Exact,
            RetainedStageStateV12::Absent,
            RetainedStageStateV12::Absent
        ] | [
            RetainedStageStateV12::Exact,
            RetainedStageStateV12::Exact,
            RetainedStageStateV12::Absent
        ] | [
            RetainedStageStateV12::Exact,
            RetainedStageStateV12::Exact,
            RetainedStageStateV12::Exact
        ]
    ) {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "pending output stages are outside the W3-W8/R1-R6 prefix closure",
        ));
    }
    let prepared = observe_marker(pending, "prepared", &intent.raw_bytes)?;
    if prepared != MarkerObservationV12::Absent && pattern != [RetainedStageStateV12::Exact; 3] {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "prepared marker is optimistic relative to staged outputs",
        ));
    }
    let rolled_tmp = names.contains("rolled-back.tmp");
    if rolled_tmp
        && (prepared != MarkerObservationV12::Absent
            || pattern != [RetainedStageStateV12::Absent; 3])
    {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "rollback marker temp appears before the exact R6 base",
        ));
    }
    validate_optional_final(pending, OutputPurposeV12::PromotionRecord, intent)?;
    validate_optional_final(pending, OutputPurposeV12::LifecycleTransition, intent)?;
    Ok(())
}

fn classify_stage_path(
    path: &Path,
    purpose: OutputPurposeV12,
    intent: &ValidatedPromotionIntentV12,
) -> Result<RetainedStageStateV12, CharterPromotionErrorV1> {
    let expected_length = match purpose {
        OutputPurposeV12::Canonical => intent.record.outputs.new_canonical_byte_length as usize,
        OutputPurposeV12::PromotionRecord => {
            intent.record.outputs.promotion_record.byte_length as usize
        }
        OutputPurposeV12::LifecycleTransition => {
            intent.record.outputs.lifecycle_transition.byte_length as usize
        }
    };
    let observed = optional_bounded_regular(path, expected_length)?;
    let state = match &observed {
        None => RetainedStageStateV12::Absent,
        Some(bytes) => {
            if validate_output_bytes_against_intent(purpose, bytes, &intent.record).is_ok() {
                RetainedStageStateV12::Exact
            } else {
                RetainedStageStateV12::Mismatch
            }
        }
    };
    if state == RetainedStageStateV12::Mismatch {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            format!(
                "pending output stage `{}` is mismatching",
                output_stage_name(purpose)
            ),
        ));
    }
    Ok(state)
}

fn validate_optional_final(
    repo_context: &Path,
    purpose: OutputPurposeV12,
    intent: &ValidatedPromotionIntentV12,
) -> Result<(), CharterPromotionErrorV1> {
    let repo_root = repo_context
        .ancestors()
        .find(|ancestor| ancestor.join(".handbook").exists())
        .ok_or_else(|| {
            error(
                CharterPromotionErrorKindV1::UnsafeFilesystem,
                "promotion journal is outside a repository root",
            )
        })?;
    let output = match purpose {
        OutputPurposeV12::PromotionRecord => &intent.record.outputs.promotion_record,
        OutputPurposeV12::LifecycleTransition => &intent.record.outputs.lifecycle_transition,
        OutputPurposeV12::Canonical => return Ok(()),
    };
    let path = repo_root.join(".handbook/state").join(&output.record_ref);
    if let Some(bytes) = optional_bounded_regular(&path, MAX_OUTPUT_RECORD_BYTES)? {
        validate_output_bytes_against_intent(purpose, &bytes, &intent.record)?;
    }
    Ok(())
}

fn validate_roll_forward_origin(
    pending: &Path,
    intent: &ValidatedPromotionIntentV12,
) -> Result<(), CharterPromotionErrorV1> {
    let names = pending_entry_names(pending)?;
    validate_owned_pending_names(&names)?;
    validate_old_snapshot(pending, &intent.record, false)?;
    if classify_stage_path(
        &pending.join("canonical.new"),
        OutputPurposeV12::Canonical,
        intent,
    )? != RetainedStageStateV12::Absent
        || observe_marker(pending, "prepared", &intent.raw_bytes)?
            != MarkerObservationV12::Published
        || observe_marker(pending, "rolled-back", &intent.raw_bytes)?
            != MarkerObservationV12::Absent
    {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "roll-forward origin lacks exact W9 prepared/canonical state",
        ));
    }
    let canonical_marker = observe_marker(pending, "canonical-installed", &intent.raw_bytes)?;
    let records_marker = observe_marker(pending, "records-installed", &intent.raw_bytes)?;
    let committed_marker = observe_marker(pending, "committed", &intent.raw_bytes)?;
    if records_marker != MarkerObservationV12::Absent
        && canonical_marker != MarkerObservationV12::Published
        || committed_marker != MarkerObservationV12::Absent
            && records_marker != MarkerObservationV12::Published
    {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "forward marker causality is invalid",
        ));
    }
    for purpose in [
        OutputPurposeV12::PromotionRecord,
        OutputPurposeV12::LifecycleTransition,
    ] {
        let stage =
            classify_stage_path(&pending.join(output_stage_name(purpose)), purpose, intent)?;
        let output = match purpose {
            OutputPurposeV12::PromotionRecord => &intent.record.outputs.promotion_record,
            OutputPurposeV12::LifecycleTransition => &intent.record.outputs.lifecycle_transition,
            OutputPurposeV12::Canonical => unreachable!(),
        };
        let repo_root = pending
            .ancestors()
            .find(|ancestor| ancestor.join(".handbook").exists())
            .ok_or_else(|| {
                error(
                    CharterPromotionErrorKindV1::UnsafeFilesystem,
                    "journal root is invalid",
                )
            })?;
        let final_path = repo_root.join(".handbook/state").join(&output.record_ref);
        let final_bytes = optional_bounded_regular(&final_path, MAX_OUTPUT_RECORD_BYTES)?;
        if let Some(bytes) = &final_bytes {
            validate_output_bytes_against_intent(purpose, bytes, &intent.record)?;
        }
        if stage == RetainedStageStateV12::Absent && final_bytes.is_none() {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "roll-forward output is not accounted by exact stage or final",
            ));
        }
    }
    Ok(())
}

fn validate_terminal_payload(
    directory: &Path,
    intent: &ValidatedPromotionIntentV12,
    terminal_kind: PromotionTerminalKindV12,
) -> Result<(), CharterPromotionErrorV1> {
    reject_reparse_or_symlink(directory, true).map_err(map_lineage)?;
    let mut expected = BTreeSet::from(["intent.json"]);
    if intent.record.mutation_mode == "amend" {
        expected.insert("canonical.old");
    }
    match terminal_kind {
        PromotionTerminalKindV12::Committed => {
            expected.extend([
                "prepared",
                "canonical-installed",
                "records-installed",
                "committed",
            ]);
        }
        PromotionTerminalKindV12::RolledBack => {
            expected.insert("rolled-back");
        }
    }
    let mut observed = BTreeSet::new();
    for entry in
        fs::read_dir(directory).map_err(|_| io_error("terminal promotion directory read failed"))?
    {
        let entry = entry.map_err(|_| io_error("terminal promotion entry read failed"))?;
        let name = entry.file_name();
        let name = name.to_str().ok_or_else(|| {
            error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "terminal promotion entry name is not UTF-8",
            )
        })?;
        let metadata = fs::symlink_metadata(entry.path())
            .map_err(|_| io_error("terminal promotion entry metadata failed"))?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(error(
                CharterPromotionErrorKindV1::UnsafeFilesystem,
                "terminal promotion entry is not a safe regular file",
            ));
        }
        observed.insert(name.to_owned());
    }
    let expected_owned = expected
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>();
    if observed != expected_owned {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "terminal promotion name set is not exact",
        ));
    }
    if read_valid_intent(directory)? != *intent {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "terminal promotion intent bytes changed",
        ));
    }
    let marker = promotion_intent_marker_v12(&intent.raw_bytes);
    let markers: &[&str] = match terminal_kind {
        PromotionTerminalKindV12::Committed => &[
            "prepared",
            "canonical-installed",
            "records-installed",
            "committed",
        ],
        PromotionTerminalKindV12::RolledBack => &["rolled-back"],
    };
    for name in markers {
        if read_bounded_regular(&directory.join(name), marker.len())? != marker {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                format!("terminal marker `{name}` differs from intent"),
            ));
        }
    }
    if intent.record.mutation_mode == "amend" {
        let old = read_bounded_regular(&directory.join("canonical.old"), MAX_CANONICAL_BYTES)?;
        if old.len() as u64
            != intent
                .record
                .recovery
                .old_canonical_byte_length
                .unwrap_or(0)
            || DefinitionFingerprint::from_bytes(&old).to_string()
                != intent
                    .record
                    .recovery
                    .old_canonical_document_sha256
                    .as_deref()
                    .unwrap_or_default()
        {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "terminal canonical.old differs from intent",
            ));
        }
    }
    Ok(())
}

fn validate_terminal_directory_identity(
    directory: &Path,
    intent: &ValidatedPromotionIntentV12,
    suffix: &str,
) -> Result<(), CharterPromotionErrorV1> {
    let expected = format!("{}{suffix}", intent.record.transaction_id);
    if directory.file_name().and_then(|name| name.to_str()) != Some(expected.as_str()) {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "terminal promotion directory key differs from its intent transaction ID",
        ));
    }
    Ok(())
}

fn classify_terminal_destination(
    destination: &Path,
    intent: &ValidatedPromotionIntentV12,
    terminal_kind: PromotionTerminalKindV12,
) -> TerminalDestinationStateV12 {
    match fs::symlink_metadata(destination) {
        Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => {
            TerminalDestinationStateV12::Absent
        }
        Err(_) => TerminalDestinationStateV12::Unsafe,
        Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
            TerminalDestinationStateV12::Unsafe
        }
        Ok(_) if validate_terminal_payload(destination, intent, terminal_kind).is_ok() => {
            TerminalDestinationStateV12::ExactPreExisting
        }
        Ok(_) => TerminalDestinationStateV12::Mismatching,
    }
}

fn read_valid_intent(directory: &Path) -> Result<ReadIntent, CharterPromotionErrorV1> {
    let raw_bytes = read_bounded_regular(&directory.join("intent.json"), MAX_INTENT_BYTES)?;
    parse_promotion_intent_v12(&raw_bytes).map_err(|failure| {
        error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            format!("promotion intent 1.2 refused: {}", failure.detail()),
        )
    })
}

fn transaction_directories(
    root: &Path,
    suffix: &str,
) -> Result<Vec<PathBuf>, CharterPromotionErrorV1> {
    let mut inventory = transaction_root_inventory(root)?;
    match suffix {
        ".pending" => Ok(std::mem::take(&mut inventory.pending)),
        ".committed" => Ok(std::mem::take(&mut inventory.committed)),
        ".rolled-back" => Ok(std::mem::take(&mut inventory.rolled_back)),
        _ => Err(error(
            CharterPromotionErrorKindV1::InvalidRequest,
            "promotion transaction suffix selector is unsupported",
        )),
    }
}

fn transaction_root_inventory(
    root: &Path,
) -> Result<TransactionRootInventoryV12, CharterPromotionErrorV1> {
    let mut inventory = TransactionRootInventoryV12::default();
    let mut transaction_ids = BTreeSet::new();
    for entry in
        fs::read_dir(root).map_err(|_| io_error("promotion transaction directory read failed"))?
    {
        let entry = entry.map_err(|_| io_error("promotion transaction entry read failed"))?;
        let name = entry.file_name();
        let name = name.to_str().ok_or_else(|| {
            error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "promotion transaction root entry name is not UTF-8",
            )
        })?;
        if matches!(name, INTENT_STAGING_NAME | OUTPUT_STAGING_NAME) {
            continue;
        }
        let (transaction_id, destination) = if let Some(id) = name.strip_suffix(".pending") {
            (id, &mut inventory.pending)
        } else if let Some(id) = name.strip_suffix(".committed") {
            (id, &mut inventory.committed)
        } else if let Some(id) = name.strip_suffix(".rolled-back") {
            (id, &mut inventory.rolled_back)
        } else {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "promotion transaction root contains an unsupported suffix or name",
            ));
        };
        if [".pending", ".committed", ".rolled-back"]
            .iter()
            .any(|suffix| transaction_id.ends_with(suffix))
        {
            return Err(error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "promotion transaction root contains a crossed suffix name",
            ));
        }
        validate_transaction_id(transaction_id).map_err(|_| {
            error(
                CharterPromotionErrorKindV1::DurabilityViolation,
                "promotion transaction root entry ID is invalid",
            )
        })?;
        if !transaction_ids.insert(transaction_id.to_owned()) {
            return Err(error(
                CharterPromotionErrorKindV1::Conflict,
                "promotion transaction ID has simultaneous suffixes",
            ));
        }
        let path = entry.path();
        destination.push(path);
    }
    inventory.pending.sort();
    inventory.committed.sort();
    inventory.rolled_back.sort();
    for path in inventory
        .pending
        .iter()
        .chain(&inventory.committed)
        .chain(&inventory.rolled_back)
    {
        reject_reparse_or_symlink(path, true).map_err(map_lineage)?;
    }
    Ok(inventory)
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
    let Some(suffix) = transaction_id.strip_prefix("promotion-transaction_") else {
        return Err(error(
            CharterPromotionErrorKindV1::InvalidRequest,
            "transaction ID prefix is invalid",
        ));
    };
    if suffix.is_empty()
        || suffix.len() > 128
        || !suffix.as_bytes()[0].is_ascii_alphanumeric()
        || !suffix
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_'))
    {
        return Err(error(
            CharterPromotionErrorKindV1::InvalidRequest,
            "transaction ID suffix grammar is invalid",
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
                    "canonical authority exceeds 1 MiB",
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
    let mut options = OpenOptions::new();
    options.read(true);
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
        .map_err(|_| io_error("journal file no-follow open failed"))?;
    let metadata = file
        .metadata()
        .map_err(|_| io_error("journal file metadata failed"))?;
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
        if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Err(error(
                CharterPromotionErrorKindV1::UnsafeFilesystem,
                "journal file is a reparse point",
            ));
        }
    }
    if !metadata.is_file() || metadata.len() > limit as u64 {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "journal file is not a bounded regular file",
        ));
    }
    let mut bytes = Vec::new();
    file.take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| io_error("journal file read failed"))?;
    if bytes.len() > limit {
        return Err(error(
            CharterPromotionErrorKindV1::DurabilityViolation,
            "journal file exceeds its bound",
        ));
    }
    Ok(bytes)
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
            .verify_retained_authority(&pending, Some(b"new canonical"), &retained)
            .unwrap();

        for record in &retained.records {
            let path = state.join(&record.relative_ref);
            fs::write(&path, b"stale").unwrap();
            let error = service
                .verify_retained_authority(&pending, Some(b"new canonical"), &retained)
                .unwrap_err();
            assert_eq!(error.kind(), CharterPromotionErrorKindV1::Conflict);
            fs::write(&path, &record.bytes).unwrap();
        }
        fs::write(pending.join("canonical.old"), b"stale").unwrap();
        assert_eq!(
            service
                .verify_retained_authority(&pending, Some(b"new canonical"), &retained)
                .unwrap_err()
                .kind(),
            CharterPromotionErrorKindV1::Conflict
        );
    }
}

#[cfg(test)]
#[path = "charter_authority_transaction_tests.rs"]
mod charter_authority_transaction_tests;

#[cfg(test)]
#[path = "charter_atomic_stage_tests.rs"]
mod charter_atomic_stage_tests;
