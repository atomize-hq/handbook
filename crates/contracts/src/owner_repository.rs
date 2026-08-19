use crate::canonical::{canonical_bytes, canonical_fingerprint};
use crate::{
    current_lifecycle_head, evaluation_lifecycle_admission_basis_v1,
    prepare_lifecycle_transition as prepare_transition_kernel, AdmittedContractDefinitionV1,
    ContractEvaluationRunV1, ContractLifecycleErrorV1, ContractLifecycleTransitionInputV1,
    ContractLifecycleTransitionV1, EvaluationLifecycleAdmissionBasisV1, EvaluationPolicyKindV1,
    EvaluationPolicyRegistryV1, ExactBindingV1, ExactRefV1, FingerprintV1, GatePolicyV1,
    LifecycleHeadV1, LifecycleStateV1, OwnerCurrentnessObservationV1, PrepareEvaluationRunInputV1,
    PreparedContractDefinitionAppendV1, SemanticTimeV1,
};
use handbook_engine::{
    CredentialAuthorityAdmissionRequestV1, CredentialAuthorityAdmissionV1,
    RepositoryDirectoryEntryKindV1, RepositoryDurabilityErrorKindV1, RepositoryReadV1,
    RetainedRepositoryRootV1,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::path::{Component, Path, PathBuf};
use std::sync::Mutex;

pub const CONTRACT_OWNER_REPOSITORY_RELATIVE_ROOT_V1: &str = ".handbook/state/contracts/v1";
const LOCK_RELATIVE_PATH: &str = ".handbook/state/locks/contract-owner.lock";
const MAX_PAGE_SIZE: usize = 100;
const MAX_RECORDS: usize = 4096;
const MAX_TRANSITIONS_PER_DEFINITION: usize = 1024;
const MAX_RECORD_BYTES: usize = 1024 * 1024;
static OWNER_PROCESS_LOCK: Mutex<()> = Mutex::new(());

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContractOwnerRepositoryErrorKindV1 {
    InvalidRequest,
    UnsafeFilesystem,
    BoundExceeded,
    DurabilityUncertain,
    RecoveryAmbiguous,
    IdempotencyConflict,
    RecordSubstituted,
    LifecycleRefused,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractOwnerRepositoryErrorV1 {
    kind: ContractOwnerRepositoryErrorKindV1,
    detail: &'static str,
}

impl ContractOwnerRepositoryErrorV1 {
    pub fn kind(&self) -> ContractOwnerRepositoryErrorKindV1 {
        self.kind
    }
    pub fn detail(&self) -> &str {
        self.detail
    }
}

impl fmt::Display for ContractOwnerRepositoryErrorV1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.detail)
    }
}
impl Error for ContractOwnerRepositoryErrorV1 {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefinitionListRequestV1 {
    limit: usize,
    root: Option<FingerprintV1>,
    cursor: Option<String>,
}

impl DefinitionListRequestV1 {
    pub fn first(limit: usize) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        if !(1..=MAX_PAGE_SIZE).contains(&limit) {
            return Err(bound("definition page limit is outside the admitted bound"));
        }
        Ok(Self {
            limit,
            root: None,
            cursor: None,
        })
    }

    pub fn continue_from(
        limit: usize,
        root: FingerprintV1,
        cursor: impl Into<String>,
    ) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        if !(1..=MAX_PAGE_SIZE).contains(&limit) {
            return Err(bound("definition page limit is outside the admitted bound"));
        }
        let cursor = cursor.into();
        if cursor.len() > 4096 {
            return Err(bound("definition cursor is outside the admitted bound"));
        }
        Ok(Self {
            limit,
            root: Some(root),
            cursor: Some(cursor),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DefinitionListRetentionV1 {
    CurrentOnly,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DefinitionListEntryV1 {
    contract_ref: ExactRefV1,
    definition_fingerprint: FingerprintV1,
    record_ref: String,
    record_fingerprint: FingerprintV1,
}

impl DefinitionListEntryV1 {
    pub fn contract_ref(&self) -> &ExactRefV1 {
        &self.contract_ref
    }
    pub fn definition_fingerprint(&self) -> &FingerprintV1 {
        &self.definition_fingerprint
    }
    pub fn record_ref(&self) -> &str {
        &self.record_ref
    }
    pub fn record_fingerprint(&self) -> &FingerprintV1 {
        &self.record_fingerprint
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefinitionListPageV1 {
    root: FingerprintV1,
    entries: Vec<DefinitionListEntryV1>,
    next_cursor: Option<String>,
}

impl DefinitionListPageV1 {
    pub fn root(&self) -> &FingerprintV1 {
        &self.root
    }
    pub fn entries(&self) -> &[DefinitionListEntryV1] {
        &self.entries
    }
    pub fn next_cursor(&self) -> Option<&str> {
        self.next_cursor.as_deref()
    }
    pub fn retention(&self) -> DefinitionListRetentionV1 {
        DefinitionListRetentionV1::CurrentOnly
    }
    pub fn is_current(&self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerMutationDispositionV1 {
    Committed,
    Replayed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefinitionAppendExecutionV1 {
    disposition: OwnerMutationDispositionV1,
    request_fingerprint: FingerprintV1,
    contract_ref: ExactRefV1,
    definition_fingerprint: FingerprintV1,
    record_ref: String,
    record_fingerprint: FingerprintV1,
    lifecycle_head: LifecycleHeadV1,
}

impl DefinitionAppendExecutionV1 {
    pub fn disposition(&self) -> OwnerMutationDispositionV1 {
        self.disposition
    }
    pub fn request_fingerprint(&self) -> &FingerprintV1 {
        &self.request_fingerprint
    }
    pub fn contract_ref(&self) -> &ExactRefV1 {
        &self.contract_ref
    }
    pub fn definition_fingerprint(&self) -> &FingerprintV1 {
        &self.definition_fingerprint
    }
    pub fn record_ref(&self) -> &str {
        &self.record_ref
    }
    pub fn record_fingerprint(&self) -> &FingerprintV1 {
        &self.record_fingerprint
    }
    pub fn lifecycle_head(&self) -> &LifecycleHeadV1 {
        &self.lifecycle_head
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedOwnerLifecycleTransitionV1 {
    contract_ref: ExactRefV1,
    definition_fingerprint: FingerprintV1,
    input: ContractLifecycleTransitionInputV1,
    authority_request: CredentialAuthorityAdmissionRequestV1,
}

impl PreparedOwnerLifecycleTransitionV1 {
    pub fn authority_request(&self) -> CredentialAuthorityAdmissionRequestV1 {
        self.authority_request.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecycleTransitionExecutionV1 {
    disposition: OwnerMutationDispositionV1,
    request_fingerprint: FingerprintV1,
    transition_ref: String,
    transition_fingerprint: FingerprintV1,
    lifecycle_head: LifecycleHeadV1,
}

impl LifecycleTransitionExecutionV1 {
    pub fn disposition(&self) -> OwnerMutationDispositionV1 {
        self.disposition
    }
    pub fn request_fingerprint(&self) -> &FingerprintV1 {
        &self.request_fingerprint
    }
    pub fn transition_ref(&self) -> &str {
        &self.transition_ref
    }
    pub fn transition_fingerprint(&self) -> &FingerprintV1 {
        &self.transition_fingerprint
    }
    pub fn lifecycle_head(&self) -> &LifecycleHeadV1 {
        &self.lifecycle_head
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefinitionRecordV1 {
    definition: AdmittedContractDefinitionV1,
    record_ref: String,
    record_fingerprint: FingerprintV1,
    record_bytes: Vec<u8>,
    lifecycle_head: LifecycleHeadV1,
}

impl DefinitionRecordV1 {
    pub fn definition(&self) -> &AdmittedContractDefinitionV1 {
        &self.definition
    }
    pub fn record_ref(&self) -> &str {
        &self.record_ref
    }
    pub fn record_fingerprint(&self) -> &FingerprintV1 {
        &self.record_fingerprint
    }
    pub fn record_bytes(&self) -> &[u8] {
        &self.record_bytes
    }
    pub fn lifecycle_head(&self) -> &LifecycleHeadV1 {
        &self.lifecycle_head
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DefinitionReadOutcomeV1 {
    Found(Box<DefinitionRecordV1>),
    Absent,
}

pub struct ContractOwnerRepositoryV1 {
    repo_root: PathBuf,
    repository_root: RetainedRepositoryRootV1,
    #[cfg(feature = "test-support")]
    fault: Option<OwnerFaultBoundary>,
}

impl fmt::Debug for ContractOwnerRepositoryV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ContractOwnerRepositoryV1")
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OwnerFaultBoundary {
    PendingDirectory,
    StagedPrimary,
    StagedHead,
    StagedLedger,
    Intent,
    Marker,
    PrimaryInstalled,
    HeadInstalled,
    LedgerInstalled,
    CommittedRename,
}

#[cfg(feature = "test-support")]
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OwnerRepositoryFaultBoundaryV1 {
    PendingDirectoryDurable,
    StagedPrimaryDurable,
    StagedHeadDurable,
    StagedLedgerDurable,
    IntentDurable,
    MarkerDurable,
    PrimaryInstalledDurable,
    HeadInstalledDurable,
    LedgerInstalledDurable,
    CommittedRenameDurable,
}

#[cfg(feature = "test-support")]
impl From<OwnerRepositoryFaultBoundaryV1> for OwnerFaultBoundary {
    fn from(value: OwnerRepositoryFaultBoundaryV1) -> Self {
        match value {
            OwnerRepositoryFaultBoundaryV1::PendingDirectoryDurable => Self::PendingDirectory,
            OwnerRepositoryFaultBoundaryV1::StagedPrimaryDurable => Self::StagedPrimary,
            OwnerRepositoryFaultBoundaryV1::StagedHeadDurable => Self::StagedHead,
            OwnerRepositoryFaultBoundaryV1::StagedLedgerDurable => Self::StagedLedger,
            OwnerRepositoryFaultBoundaryV1::IntentDurable => Self::Intent,
            OwnerRepositoryFaultBoundaryV1::MarkerDurable => Self::Marker,
            OwnerRepositoryFaultBoundaryV1::PrimaryInstalledDurable => Self::PrimaryInstalled,
            OwnerRepositoryFaultBoundaryV1::HeadInstalledDurable => Self::HeadInstalled,
            OwnerRepositoryFaultBoundaryV1::LedgerInstalledDurable => Self::LedgerInstalled,
            OwnerRepositoryFaultBoundaryV1::CommittedRenameDurable => Self::CommittedRename,
        }
    }
}

impl ContractOwnerRepositoryV1 {
    /// Lock order is Engine authority -> process owner -> repository owner.
    /// This module never calls Engine and exposes no reentrant callback.
    pub fn open(repo_root: impl AsRef<Path>) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        let root = repo_root.as_ref();
        let repository_root = RetainedRepositoryRootV1::open_exact(root).map_err(map_durability)?;
        let owner = Self {
            repo_root: root.to_path_buf(),
            repository_root,
            #[cfg(feature = "test-support")]
            fault: None,
        };
        owner.with_lock(|owner| {
            owner.ensure_layout()?;
            owner.recover_locked()?;
            owner.validate_inventory()
        })?;
        Ok(owner)
    }

    pub(crate) fn retained_repository_root(&self) -> &RetainedRepositoryRootV1 {
        &self.repository_root
    }

    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn with_fault_for_testing(mut self, fault: OwnerRepositoryFaultBoundaryV1) -> Self {
        self.fault = Some(fault.into());
        self
    }

    #[cfg(feature = "test-support")]
    fn maybe_fault(
        &self,
        boundary: OwnerFaultBoundary,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        if self.fault == Some(boundary) {
            Err(durability("injected owner durability uncertainty"))
        } else {
            Ok(())
        }
    }

    #[cfg(not(feature = "test-support"))]
    fn maybe_fault(
        &self,
        _boundary: OwnerFaultBoundary,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        Ok(())
    }

    pub fn list_definitions(
        &self,
        request: DefinitionListRequestV1,
    ) -> Result<DefinitionListPageV1, ContractOwnerRepositoryErrorV1> {
        self.with_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            let entries = owner.load_entries()?;
            let root = canonical_fingerprint(&ListRootPreimage {
                schema_ref: "handbook.contract-definition-list-root@1.0.0",
                retention: "current_only",
                definitions: &entries,
            })
            .map_err(|_| durability("definition list root could not be derived"))?;
            let offset = match (&request.root, &request.cursor) {
                (None, None) => 0,
                (Some(expected_root), Some(cursor)) => {
                    if expected_root != &root {
                        return Err(substituted("definition list root is no longer current"));
                    }
                    decode_cursor(cursor, expected_root, request.limit)?
                }
                _ => {
                    return Err(repository_error(
                        ContractOwnerRepositoryErrorKindV1::InvalidRequest,
                        "definition list continuation requires both root and cursor",
                    ))
                }
            };
            if offset > entries.len() {
                return Err(substituted(
                    "definition cursor offset is outside the current root",
                ));
            }
            let end = offset.saturating_add(request.limit).min(entries.len());
            let next_cursor = (end < entries.len())
                .then(|| encode_cursor(&root, end, request.limit))
                .transpose()?;
            Ok(DefinitionListPageV1 {
                root,
                entries: entries[offset..end].to_vec(),
                next_cursor,
            })
        })
    }

    pub fn read_definition(
        &self,
        contract_ref: &ExactRefV1,
        fingerprint: &FingerprintV1,
    ) -> Result<DefinitionReadOutcomeV1, ContractOwnerRepositoryErrorV1> {
        self.with_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            let outcome = owner.read_definition_locked(contract_ref, fingerprint)?;
            if matches!(outcome, DefinitionReadOutcomeV1::Absent)
                && owner
                    .load_entries()?
                    .iter()
                    .any(|entry| entry.contract_ref == *contract_ref)
            {
                return Err(substituted(
                    "exact definition ref exists under another fingerprint",
                ));
            }
            Ok(outcome)
        })
    }

    pub fn current_lifecycle_head(
        &self,
        contract_ref: &ExactRefV1,
        definition_fingerprint: &FingerprintV1,
    ) -> Result<LifecycleHeadV1, ContractOwnerRepositoryErrorV1> {
        self.with_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            let (definition, chain, head) =
                owner.load_lifecycle_bundle(contract_ref, definition_fingerprint)?;
            current_lifecycle_head(&definition, &chain).map_err(map_lifecycle)?;
            Ok(head)
        })
    }

    pub fn evaluation_lifecycle_basis(
        &self,
        contract_ref: &ExactRefV1,
        definition_fingerprint: &FingerprintV1,
        expected_lifecycle_fingerprint: &FingerprintV1,
    ) -> Result<EvaluationLifecycleAdmissionBasisV1, ContractOwnerRepositoryErrorV1> {
        self.with_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            let (definition, chain, head) =
                owner.load_lifecycle_bundle(contract_ref, definition_fingerprint)?;
            if head.lifecycle_fingerprint() != expected_lifecycle_fingerprint {
                return Err(map_lifecycle(ContractLifecycleErrorV1::StaleHead));
            }
            evaluation_lifecycle_admission_basis_v1(&definition, &chain).map_err(map_lifecycle)
        })
    }

    /// Atomically prepares one exact read-only evaluation run from the same
    /// recovered definition and current lifecycle snapshot.
    pub fn prepare_evaluation_run_v1(
        &self,
        input: PrepareEvaluationRunInputV1,
        policy_registry: &EvaluationPolicyRegistryV1,
    ) -> Result<ContractEvaluationRunV1, ContractOwnerRepositoryErrorV1> {
        self.with_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            let (definition, chain, persisted_head) = owner.load_lifecycle_bundle(
                input.contract().reference(),
                input.contract().fingerprint(),
            )?;
            let current_head =
                current_lifecycle_head(&definition, &chain).map_err(map_lifecycle)?;
            if current_head != persisted_head
                || current_head.lifecycle_fingerprint() != input.expected_lifecycle_fingerprint()
            {
                return Err(map_lifecycle(ContractLifecycleErrorV1::StaleHead));
            }
            let basis = evaluation_lifecycle_admission_basis_v1(&definition, &chain)
                .map_err(map_lifecycle)?;
            let definition = definition.definition();
            let claims = definition.claims().to_vec();
            let closure = definition.evaluation_closure();
            if input.freshness_policies() != closure.freshness_policies
                || input.stability_policies() != closure.stability_policies
                || closure.matcher_bindings.as_slice() != std::slice::from_ref(input.matcher())
            {
                return Err(invalid("definition evaluation closure was substituted"));
            }
            require_policy_kind(
                policy_registry,
                input.freshness_policies(),
                EvaluationPolicyKindV1::Freshness,
            )?;
            require_policy_kind(
                policy_registry,
                input.stability_policies(),
                EvaluationPolicyKindV1::Stability,
            )?;
            let gate_policy = definition.gate_policy().clone();
            if input.gate_policy() != &gate_policy {
                return Err(invalid("definition gate policy was substituted"));
            }
            let registered_gate = policy_registry
                .resolve_exact(&gate_policy)
                .map_err(map_contract_input)?;
            if registered_gate.kind() != EvaluationPolicyKindV1::Gate {
                return Err(invalid("definition gate policy kind is not admitted"));
            }
            let gate: GatePolicyV1 = serde_json::from_slice(&registered_gate.canonical_json())
                .map_err(|_| invalid("registered gate policy bytes are invalid"))?;
            let local_closeout_policy = gate.local_closeout_policy().clone();
            if input.local_closeout_policy() != &local_closeout_policy {
                return Err(invalid("gate local-closeout policy was substituted"));
            }
            require_policy_kind(
                policy_registry,
                std::slice::from_ref(&local_closeout_policy),
                EvaluationPolicyKindV1::LocalCloseout,
            )?;
            let owner_currentness = OwnerCurrentnessObservationV1::new(
                SemanticTimeV1::owner_now().map_err(map_contract_input)?,
                current_head.lifecycle_fingerprint().clone(),
            );
            ContractEvaluationRunV1::from_owner_snapshot(
                definition.contract_ref().clone(),
                definition.definition_fingerprint().clone(),
                &basis,
                input,
                claims,
                owner_currentness,
            )
            .map_err(map_contract_input)
        })
    }

    /// Independently re-admits a caller/source supplied Packet-1 run against
    /// the current recovered owner definition, lifecycle, and exact policy
    /// closure. It derives no evidence semantics and performs no write.
    pub fn readmit_evaluation_run_v1(
        &self,
        run: &ContractEvaluationRunV1,
        policy_registry: &EvaluationPolicyRegistryV1,
    ) -> Result<LifecycleHeadV1, ContractOwnerRepositoryErrorV1> {
        self.with_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            let contract = run.contract();
            let (definition, chain, persisted_head) =
                owner.load_lifecycle_bundle(contract.reference(), contract.fingerprint())?;
            let current_head =
                current_lifecycle_head(&definition, &chain).map_err(map_lifecycle)?;
            if current_head != persisted_head
                || current_head.lifecycle_fingerprint()
                    != run.owner_currentness().lifecycle_head_fingerprint()
            {
                return Err(map_lifecycle(ContractLifecycleErrorV1::StaleHead));
            }
            let basis = evaluation_lifecycle_admission_basis_v1(&definition, &chain)
                .map_err(map_lifecycle)?;
            if run.evaluation_lifecycle_basis().reference() != basis.basis_ref()
                || run.evaluation_lifecycle_basis().fingerprint() != basis.basis_fingerprint()
            {
                return Err(invalid("evaluation lifecycle basis was substituted"));
            }
            let definition = definition.definition();
            if run.contract().reference() != definition.contract_ref()
                || run.contract().fingerprint() != definition.definition_fingerprint()
                || run.claims() != definition.claims()
            {
                return Err(invalid(
                    "evaluation run definition or claims were substituted",
                ));
            }
            let closure = definition.evaluation_closure();
            if run.freshness_policies() != closure.freshness_policies
                || run.stability_policies() != closure.stability_policies
                || closure.matcher_bindings.as_slice() != std::slice::from_ref(run.matcher())
            {
                return Err(invalid("evaluation run policy closure was substituted"));
            }
            require_policy_kind(
                policy_registry,
                run.freshness_policies(),
                EvaluationPolicyKindV1::Freshness,
            )?;
            require_policy_kind(
                policy_registry,
                run.stability_policies(),
                EvaluationPolicyKindV1::Stability,
            )?;
            if run.gate_policy() != definition.gate_policy() {
                return Err(invalid("evaluation run gate policy was substituted"));
            }
            let registered_gate = policy_registry
                .resolve_exact(run.gate_policy())
                .map_err(map_contract_input)?;
            if registered_gate.kind() != EvaluationPolicyKindV1::Gate {
                return Err(invalid("evaluation run gate policy kind is not admitted"));
            }
            let gate: GatePolicyV1 = serde_json::from_slice(&registered_gate.canonical_json())
                .map_err(|_| invalid("registered gate policy bytes are invalid"))?;
            if run.local_closeout_policy() != gate.local_closeout_policy() {
                return Err(invalid(
                    "evaluation run local-closeout policy was substituted",
                ));
            }
            require_policy_kind(
                policy_registry,
                std::slice::from_ref(run.local_closeout_policy()),
                EvaluationPolicyKindV1::LocalCloseout,
            )?;
            let bytes = run.canonical_json_bytes().map_err(map_contract_input)?;
            let parsed: ContractEvaluationRunV1 = serde_json::from_slice(&bytes)
                .map_err(|_| invalid("evaluation run canonical identity is invalid"))?;
            if &parsed != run {
                return Err(invalid("evaluation run canonical identity was substituted"));
            }
            Ok(current_head)
        })
    }

    pub fn prepare_lifecycle_transition(
        &self,
        contract_ref: &ExactRefV1,
        definition_fingerprint: &FingerprintV1,
        input: ContractLifecycleTransitionInputV1,
    ) -> Result<PreparedOwnerLifecycleTransitionV1, ContractOwnerRepositoryErrorV1> {
        self.with_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            let (definition, chain, _) =
                owner.load_lifecycle_bundle(contract_ref, definition_fingerprint)?;
            let prepared = prepare_transition_kernel(&definition, &chain, input.clone())
                .map_err(map_lifecycle)?;
            Ok(PreparedOwnerLifecycleTransitionV1 {
                contract_ref: contract_ref.clone(),
                definition_fingerprint: definition_fingerprint.clone(),
                input,
                authority_request: prepared.authority_request(),
            })
        })
    }

    pub fn transition_lifecycle(
        &self,
        prepared: PreparedOwnerLifecycleTransitionV1,
        admission: CredentialAuthorityAdmissionV1<'_>,
        key: &str,
    ) -> Result<LifecycleTransitionExecutionV1, ContractOwnerRepositoryErrorV1> {
        validate_key(key)?;
        admission
            .require_repository_root(&self.repository_root)
            .map_err(|_| unsafe_fs("authority admission belongs to another repository root"))?;
        let request_fingerprint = canonical_fingerprint(&TransitionRequestPreimage {
            schema_ref: "handbook.contract-lifecycle-transition-request-fingerprint@1.0.0",
            contract_ref: &prepared.contract_ref,
            definition_fingerprint: &prepared.definition_fingerprint,
            input: &prepared.input,
        })
        .map_err(|_| durability("transition request fingerprint could not be derived"))?;
        let key_fingerprint = canonical_fingerprint(&KeyPreimage {
            schema_ref: "handbook.contract-owner-transition-idempotency-key@1.0.0",
            key,
        })
        .map_err(|_| durability("transition key fingerprint could not be derived"))?;
        self.with_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            if let Some(replay) = owner.replay_transition(&key_fingerprint, &request_fingerprint)? {
                let (definition, chain, _) = owner.load_lifecycle_bundle(
                    &prepared.contract_ref,
                    &prepared.definition_fingerprint,
                )?;
                let replay_index = chain
                    .iter()
                    .position(|transition| transition.transition_ref() == replay.transition_ref())
                    .ok_or_else(|| recovery("transition replay record is absent"))?;
                let replay_prepared = prepare_transition_kernel(
                    &definition,
                    &chain[..replay_index],
                    prepared.input.clone(),
                )
                .map_err(map_lifecycle)?;
                if replay_prepared.authority_request() != prepared.authority_request {
                    return Err(substituted(
                        "transition replay authority request is no longer exact",
                    ));
                }
                replay_prepared.finalize(admission).map_err(map_lifecycle)?;
                return Ok(replay);
            }
            let (definition, chain, _) = owner
                .load_lifecycle_bundle(&prepared.contract_ref, &prepared.definition_fingerprint)?;
            let current_prepared =
                prepare_transition_kernel(&definition, &chain, prepared.input.clone())
                    .map_err(map_lifecycle)?;
            if current_prepared.authority_request() != prepared.authority_request {
                return Err(substituted(
                    "transition authority request is no longer exact",
                ));
            }
            let transition = current_prepared
                .finalize(admission)
                .map_err(map_lifecycle)?;
            owner.commit_transition(
                definition,
                chain,
                transition,
                request_fingerprint,
                key_fingerprint,
            )
        })
    }

    pub fn append_definition(
        &self,
        prepared: PreparedContractDefinitionAppendV1,
        admission: CredentialAuthorityAdmissionV1<'_>,
        key: &str,
    ) -> Result<DefinitionAppendExecutionV1, ContractOwnerRepositoryErrorV1> {
        validate_key(key)?;
        admission
            .require_repository_root(&self.repository_root)
            .map_err(|_| unsafe_fs("authority admission belongs to another repository root"))?;
        let admitted = prepared.finalize(admission).map_err(map_lifecycle)?;
        let definition_bytes = admitted.canonical_json_bytes().map_err(map_lifecycle)?;
        if definition_bytes.len() > MAX_RECORD_BYTES {
            return Err(bound("definition record is too large"));
        }
        let record_fingerprint = FingerprintV1::from_bytes(&definition_bytes);
        let contract_ref = admitted.definition().contract_ref().clone();
        let definition_fingerprint = admitted.definition().definition_fingerprint().clone();
        let request_fingerprint = canonical_fingerprint(&AppendRequestPreimage {
            schema_ref: "handbook.contract-definition-append-request-fingerprint@1.0.0",
            contract_ref: &contract_ref,
            definition_fingerprint: &definition_fingerprint,
        })
        .map_err(|_| durability("append request fingerprint could not be derived"))?;
        let key_fingerprint = canonical_fingerprint(&KeyPreimage {
            schema_ref: "handbook.contract-owner-idempotency-key@1.0.0",
            key,
        })
        .map_err(|_| durability("key fingerprint could not be derived"))?;
        self.with_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            if let Some(replay) = owner.replay(&key_fingerprint, &request_fingerprint)? {
                return Ok(replay);
            }
            if owner
                .load_entries()?
                .iter()
                .any(|entry| entry.contract_ref == contract_ref)
            {
                return Err(substituted("exact definition ref is already immutable"));
            }
            owner.commit_append(
                admitted,
                definition_bytes,
                record_fingerprint,
                request_fingerprint,
                key_fingerprint,
            )
        })
    }

    fn commit_append(
        &self,
        admitted: AdmittedContractDefinitionV1,
        definition_bytes: Vec<u8>,
        record_fingerprint: FingerprintV1,
        request_fingerprint: FingerprintV1,
        key_fingerprint: FingerprintV1,
    ) -> Result<DefinitionAppendExecutionV1, ContractOwnerRepositoryErrorV1> {
        let contract_ref = admitted.definition().contract_ref().clone();
        let definition_fingerprint = admitted.definition().definition_fingerprint().clone();
        let head = current_lifecycle_head(&admitted, &[]).map_err(map_lifecycle)?;
        let head_record = HeadRecord::new(&admitted, &head, 0)?;
        let head_bytes =
            canonical_bytes(&head_record).map_err(|_| durability("head encoding failed"))?;
        let definition_ref = self.definition_ref(&definition_fingerprint);
        let ledger = AppendLedger::new(
            &key_fingerprint,
            &request_fingerprint,
            &contract_ref,
            &definition_fingerprint,
            &definition_ref,
            &record_fingerprint,
            head.lifecycle_fingerprint(),
        )?;
        let ledger_bytes =
            canonical_bytes(&ledger).map_err(|_| durability("ledger encoding failed"))?;
        let transaction_fingerprint = canonical_fingerprint(&TransactionIdPreimage {
            schema_ref: "handbook.contract-owner-transaction-identity@1.0.0",
            operation: "contract.definition.append",
            key_fingerprint: &key_fingerprint,
            request_fingerprint: &request_fingerprint,
        })
        .map_err(|_| durability("transaction identity failed"))?;
        let transaction_id = fingerprint_hex(&transaction_fingerprint);
        let pending = self
            .transactions_root()
            .join(format!("{transaction_id}.pending"));
        let committed = self
            .transactions_root()
            .join(format!("{transaction_id}.committed"));
        if self.exists(&pending)? || self.exists(&committed)? {
            return Err(recovery("transaction identity is occupied"));
        }
        self.create_dir_durable(&pending)?;
        self.maybe_fault(OwnerFaultBoundary::PendingDirectory)?;
        let staged_bytes = [
            definition_bytes.as_slice(),
            head_bytes.as_slice(),
            ledger_bytes.as_slice(),
        ];
        let outputs = vec![
            StagedOutput::new("00-definition.json", &definition_ref, staged_bytes[0])?,
            StagedOutput::new(
                "01-head.json",
                &self.head_ref(&definition_fingerprint),
                staged_bytes[1],
            )?,
            StagedOutput::new(
                "02-ledger.json",
                &self.ledger_ref(&key_fingerprint),
                staged_bytes[2],
            )?,
        ];
        for (index, (output, bytes)) in outputs.iter().zip(staged_bytes).enumerate() {
            self.write_new_durable(&pending.join(&output.staged_name), bytes)?;
            self.maybe_fault(match index {
                0 => OwnerFaultBoundary::StagedPrimary,
                1 => OwnerFaultBoundary::StagedHead,
                _ => OwnerFaultBoundary::StagedLedger,
            })?;
        }
        let intent = TransactionIntent::new(
            transaction_id,
            "contract.definition.append",
            &key_fingerprint,
            &request_fingerprint,
            None,
            outputs,
        )?;
        self.write_new_durable(
            &pending.join("intent.json"),
            &canonical_bytes(&intent).map_err(|_| durability("intent encoding failed"))?,
        )?;
        self.maybe_fault(OwnerFaultBoundary::Intent)?;
        let marker = CommitMarker::new(&intent)?;
        self.write_new_durable(
            &pending.join("commit-marker.json"),
            &canonical_bytes(&marker).map_err(|_| durability("marker encoding failed"))?,
        )?;
        self.maybe_fault(OwnerFaultBoundary::Marker)?;
        self.finish_transaction(&pending, &intent, &marker)?;
        self.rename_dir_durable(&pending, &committed)?;
        self.maybe_fault(OwnerFaultBoundary::CommittedRename)?;
        self.validate_inventory()?;
        Ok(DefinitionAppendExecutionV1 {
            disposition: OwnerMutationDispositionV1::Committed,
            request_fingerprint,
            contract_ref,
            definition_fingerprint,
            record_ref: definition_ref,
            record_fingerprint,
            lifecycle_head: head,
        })
    }

    fn replay(
        &self,
        key: &FingerprintV1,
        request: &FingerprintV1,
    ) -> Result<Option<DefinitionAppendExecutionV1>, ContractOwnerRepositoryErrorV1> {
        let path = self.ledger_path(key);
        if !self.exists(&path)? {
            return Ok(None);
        }
        let ledger: AppendLedger = parse_canonical(&self.read_regular(&path)?)?;
        ledger.validate()?;
        if ledger.key_fingerprint != *key || ledger.request_fingerprint != *request {
            return Err(repository_error(
                ContractOwnerRepositoryErrorKindV1::IdempotencyConflict,
                "idempotency fingerprint is bound to another request",
            ));
        }
        let contract_ref = ExactRefV1::parse(&ledger.contract_ref)
            .map_err(|_| recovery("replay contract ref is invalid"))?;
        let DefinitionReadOutcomeV1::Found(record) =
            self.read_definition_locked(&contract_ref, &ledger.definition_fingerprint)?
        else {
            return Err(recovery("replay definition is absent"));
        };
        let genesis = current_lifecycle_head(&record.definition, &[]).map_err(map_lifecycle)?;
        if record.record_ref != ledger.record_ref
            || record.record_fingerprint != ledger.record_fingerprint
            || genesis.lifecycle_fingerprint() != &ledger.lifecycle_fingerprint
        {
            return Err(recovery("replay closure is substituted"));
        }
        Ok(Some(DefinitionAppendExecutionV1 {
            disposition: OwnerMutationDispositionV1::Replayed,
            request_fingerprint: request.clone(),
            contract_ref,
            definition_fingerprint: ledger.definition_fingerprint,
            record_ref: ledger.record_ref,
            record_fingerprint: ledger.record_fingerprint,
            lifecycle_head: genesis,
        }))
    }

    fn commit_transition(
        &self,
        definition: AdmittedContractDefinitionV1,
        mut chain: Vec<ContractLifecycleTransitionV1>,
        transition: ContractLifecycleTransitionV1,
        request_fingerprint: FingerprintV1,
        key_fingerprint: FingerprintV1,
    ) -> Result<LifecycleTransitionExecutionV1, ContractOwnerRepositoryErrorV1> {
        let transition_bytes = transition.canonical_json_bytes().map_err(map_lifecycle)?;
        if transition_bytes.len() > MAX_RECORD_BYTES {
            return Err(bound("transition record is too large"));
        }
        let transition_fingerprint = transition.transition_fingerprint().clone();
        if FingerprintV1::from_bytes(&transition_bytes) == transition_fingerprint {
            return Err(substituted(
                "transition semantic and record identity domains collided",
            ));
        }
        let transition_record_ref = self.transition_ref(&transition_fingerprint);
        let transition_ref = transition.transition_ref().to_owned();
        let prior_head = current_lifecycle_head(&definition, &chain).map_err(map_lifecycle)?;
        let prior_head_record = HeadRecord::new(&definition, &prior_head, chain.len())?;
        let prior_head_bytes = canonical_bytes(&prior_head_record)
            .map_err(|_| durability("prior transition head encoding failed"))?;
        chain.push(transition.clone());
        if chain.len() > MAX_TRANSITIONS_PER_DEFINITION {
            return Err(bound("lifecycle chain exceeds the admitted bound"));
        }
        let head = current_lifecycle_head(&definition, &chain).map_err(map_lifecycle)?;
        let head_record = HeadRecord::new(&definition, &head, chain.len())?;
        let head_bytes = canonical_bytes(&head_record)
            .map_err(|_| durability("transition head encoding failed"))?;
        let ledger = TransitionLedger::new(
            &key_fingerprint,
            &request_fingerprint,
            definition.definition().contract_ref(),
            definition.definition().definition_fingerprint(),
            &transition_ref,
            &transition_fingerprint,
            head.lifecycle_fingerprint(),
        )?;
        let ledger_bytes = canonical_bytes(&ledger)
            .map_err(|_| durability("transition ledger encoding failed"))?;
        let transaction_fingerprint = canonical_fingerprint(&TransactionIdPreimage {
            schema_ref: "handbook.contract-owner-transaction-identity@1.0.0",
            operation: "contract.lifecycle.transition",
            key_fingerprint: &key_fingerprint,
            request_fingerprint: &request_fingerprint,
        })
        .map_err(|_| durability("transition transaction identity failed"))?;
        let transaction_id = fingerprint_hex(&transaction_fingerprint);
        let pending = self
            .transactions_root()
            .join(format!("{transaction_id}.pending"));
        let committed = self
            .transactions_root()
            .join(format!("{transaction_id}.committed"));
        if self.exists(&pending)? || self.exists(&committed)? {
            return Err(recovery("transition transaction identity is occupied"));
        }
        self.create_dir_durable(&pending)?;
        self.maybe_fault(OwnerFaultBoundary::PendingDirectory)?;
        let staged_bytes = [
            transition_bytes.as_slice(),
            head_bytes.as_slice(),
            ledger_bytes.as_slice(),
        ];
        let outputs = vec![
            StagedOutput::new(
                "00-transition.json",
                &transition_record_ref,
                staged_bytes[0],
            )?,
            StagedOutput::new(
                "01-head.json",
                &self.head_ref(definition.definition().definition_fingerprint()),
                staged_bytes[1],
            )?,
            StagedOutput::new(
                "02-ledger.json",
                &self.ledger_ref(&key_fingerprint),
                staged_bytes[2],
            )?,
        ];
        for (index, (output, bytes)) in outputs.iter().zip(staged_bytes).enumerate() {
            self.write_new_durable(&pending.join(&output.staged_name), bytes)?;
            self.maybe_fault(match index {
                0 => OwnerFaultBoundary::StagedPrimary,
                1 => OwnerFaultBoundary::StagedHead,
                _ => OwnerFaultBoundary::StagedLedger,
            })?;
        }
        let intent = TransactionIntent::new(
            transaction_id,
            "contract.lifecycle.transition",
            &key_fingerprint,
            &request_fingerprint,
            Some(FingerprintV1::from_bytes(&prior_head_bytes)),
            outputs,
        )?;
        self.write_new_durable(
            &pending.join("intent.json"),
            &canonical_bytes(&intent)
                .map_err(|_| durability("transition intent encoding failed"))?,
        )?;
        self.maybe_fault(OwnerFaultBoundary::Intent)?;
        let marker = CommitMarker::new(&intent)?;
        self.write_new_durable(
            &pending.join("commit-marker.json"),
            &canonical_bytes(&marker)
                .map_err(|_| durability("transition marker encoding failed"))?,
        )?;
        self.maybe_fault(OwnerFaultBoundary::Marker)?;
        self.finish_transaction(&pending, &intent, &marker)?;
        self.rename_dir_durable(&pending, &committed)?;
        self.maybe_fault(OwnerFaultBoundary::CommittedRename)?;
        self.validate_inventory()?;
        Ok(LifecycleTransitionExecutionV1 {
            disposition: OwnerMutationDispositionV1::Committed,
            request_fingerprint,
            transition_ref,
            transition_fingerprint,
            lifecycle_head: head,
        })
    }

    fn replay_transition(
        &self,
        key: &FingerprintV1,
        request: &FingerprintV1,
    ) -> Result<Option<LifecycleTransitionExecutionV1>, ContractOwnerRepositoryErrorV1> {
        let path = self.ledger_path(key);
        if !self.exists(&path)? {
            return Ok(None);
        }
        let ledger: TransitionLedger = parse_canonical(&self.read_regular(&path)?)?;
        ledger.validate()?;
        if ledger.key_fingerprint != *key || ledger.request_fingerprint != *request {
            return Err(repository_error(
                ContractOwnerRepositoryErrorKindV1::IdempotencyConflict,
                "transition idempotency fingerprint is bound to another request",
            ));
        }
        let contract_ref = ExactRefV1::parse(&ledger.contract_ref)
            .map_err(|_| recovery("transition replay contract ref is invalid"))?;
        let (definition, chain, _) =
            self.load_lifecycle_bundle(&contract_ref, &ledger.definition_fingerprint)?;
        let (index, transition) = chain
            .iter()
            .enumerate()
            .find(|(_, value)| value.transition_ref() == ledger.transition_ref)
            .ok_or_else(|| recovery("transition replay record is absent"))?;
        let head = current_lifecycle_head(&definition, &chain[..=index]).map_err(map_lifecycle)?;
        if transition.transition_fingerprint() != &ledger.transition_fingerprint
            || head.lifecycle_fingerprint() != &ledger.resulting_lifecycle_fingerprint
        {
            return Err(recovery("transition replay closure is substituted"));
        }
        Ok(Some(LifecycleTransitionExecutionV1 {
            disposition: OwnerMutationDispositionV1::Replayed,
            request_fingerprint: request.clone(),
            transition_ref: ledger.transition_ref,
            transition_fingerprint: ledger.transition_fingerprint,
            lifecycle_head: head,
        }))
    }

    fn read_definition_locked(
        &self,
        contract_ref: &ExactRefV1,
        fingerprint: &FingerprintV1,
    ) -> Result<DefinitionReadOutcomeV1, ContractOwnerRepositoryErrorV1> {
        let path = self.definition_path(fingerprint);
        if !self.exists(&path)? {
            return Ok(DefinitionReadOutcomeV1::Absent);
        }
        let record = self.load_definition(&path)?;
        if record.definition.definition().contract_ref() != contract_ref
            || record.definition.definition().definition_fingerprint() != fingerprint
        {
            return Err(substituted("definition identity is substituted"));
        }
        Ok(DefinitionReadOutcomeV1::Found(Box::new(record)))
    }

    fn load_definition(
        &self,
        path: &Path,
    ) -> Result<DefinitionRecordV1, ContractOwnerRepositoryErrorV1> {
        let bytes = self.read_regular(path)?;
        let definition: AdmittedContractDefinitionV1 = parse_canonical(&bytes)?;
        let fingerprint = definition.definition().definition_fingerprint();
        if file_stem(path)? != fingerprint_hex(fingerprint) {
            return Err(substituted("definition filename is substituted"));
        }
        let (_, lifecycle_head) = self.load_chain_and_head(&definition)?;
        Ok(DefinitionRecordV1 {
            record_ref: self.definition_ref(fingerprint),
            record_fingerprint: FingerprintV1::from_bytes(&bytes),
            record_bytes: bytes,
            definition,
            lifecycle_head,
        })
    }

    fn load_lifecycle_bundle(
        &self,
        contract_ref: &ExactRefV1,
        definition_fingerprint: &FingerprintV1,
    ) -> Result<
        (
            AdmittedContractDefinitionV1,
            Vec<ContractLifecycleTransitionV1>,
            LifecycleHeadV1,
        ),
        ContractOwnerRepositoryErrorV1,
    > {
        let path = self.definition_path(definition_fingerprint);
        if !self.exists(&path)? {
            return Err(substituted("exact lifecycle definition is absent"));
        }
        let bytes = self.read_regular(&path)?;
        let definition: AdmittedContractDefinitionV1 = parse_canonical(&bytes)?;
        if definition.definition().contract_ref() != contract_ref
            || definition.definition().definition_fingerprint() != definition_fingerprint
            || file_stem(&path)? != fingerprint_hex(definition_fingerprint)
        {
            return Err(substituted("exact lifecycle definition is substituted"));
        }
        let (chain, head) = self.load_chain_and_head(&definition)?;
        Ok((definition, chain, head))
    }

    fn load_chain_and_head(
        &self,
        definition: &AdmittedContractDefinitionV1,
    ) -> Result<(Vec<ContractLifecycleTransitionV1>, LifecycleHeadV1), ContractOwnerRepositoryErrorV1>
    {
        let mut remaining = Vec::new();
        for path in self.read_dir_bounded(&self.transitions_root(), MAX_RECORDS)? {
            if path.extension().and_then(|value| value.to_str()) != Some("json") {
                return Err(recovery("transition inventory has an unknown entry"));
            }
            let bytes = self.read_regular(&path)?;
            let transition: ContractLifecycleTransitionV1 = parse_canonical(&bytes)?;
            if file_stem(&path)? != fingerprint_hex(transition.transition_fingerprint()) {
                return Err(substituted("transition filename is substituted"));
            }
            if transition.contract_ref() == definition.definition().contract_ref()
                && transition.definition_fingerprint()
                    == definition.definition().definition_fingerprint()
            {
                remaining.push(transition);
            }
        }
        if remaining.len() > MAX_TRANSITIONS_PER_DEFINITION {
            return Err(bound("lifecycle chain exceeds the recovery bound"));
        }
        let mut chain = Vec::with_capacity(remaining.len());
        let mut head = current_lifecycle_head(definition, &[]).map_err(map_lifecycle)?;
        while !remaining.is_empty() {
            let matching = remaining
                .iter()
                .enumerate()
                .filter(|(_, transition)| {
                    transition.prior_transition_ref() == head.transition_ref()
                        && transition.prior_lifecycle_fingerprint() == head.lifecycle_fingerprint()
                })
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
            match matching.as_slice() {
                [] => {
                    return Err(recovery(
                        "lifecycle transition is orphaned or discontinuous",
                    ))
                }
                [index] => chain.push(remaining.remove(*index)),
                _ => return Err(recovery("lifecycle transition chain forks")),
            }
            head = current_lifecycle_head(definition, &chain).map_err(map_lifecycle)?;
        }

        for (index, transition) in chain.iter().enumerate() {
            let transition_ref = self.transition_ref(transition.transition_fingerprint());
            let mut matched = None;
            for transaction in self.read_dir_bounded(&self.transactions_root(), MAX_RECORDS)? {
                if !file_name(&transaction)?.ends_with(".committed") {
                    continue;
                }
                let intent: TransactionIntent =
                    parse_canonical(&self.read_regular(&transaction.join("intent.json"))?)?;
                if intent.outputs[0].relative_ref == transition_ref
                    && matched.replace((transaction, intent)).is_some()
                {
                    return Err(recovery(
                        "transition is claimed by multiple committed transactions",
                    ));
                }
            }
            let (transaction, intent) =
                matched.ok_or_else(|| recovery("transition has no committed transaction claim"))?;
            let staged_head: HeadRecord =
                parse_canonical(&self.read_regular(&transaction.join("01-head.json"))?)?;
            staged_head.validate(definition, &chain[..=index])?;
            let prior_head =
                current_lifecycle_head(definition, &chain[..index]).map_err(map_lifecycle)?;
            let prior_record = HeadRecord::new(definition, &prior_head, index)?;
            let prior_bytes = canonical_bytes(&prior_record)
                .map_err(|_| recovery("historical prior head cannot be encoded"))?;
            if intent.expected_head_bytes_fingerprint
                != Some(FingerprintV1::from_bytes(&prior_bytes))
            {
                return Err(recovery(
                    "transition transaction expected head basis is substituted",
                ));
            }
        }

        let path = self.head_path(definition.definition().definition_fingerprint());
        let persisted: HeadRecord = parse_canonical(&self.read_regular(&path)?)?;
        if file_stem(&path)? != fingerprint_hex(definition.definition().definition_fingerprint()) {
            return Err(substituted("derived head filename is substituted"));
        }
        persisted.validate(definition, &chain)?;
        Ok((chain, head))
    }

    fn load_entries(&self) -> Result<Vec<DefinitionListEntryV1>, ContractOwnerRepositoryErrorV1> {
        let mut entries = Vec::new();
        for path in self.read_dir_bounded(&self.definitions_root(), MAX_RECORDS)? {
            if path.extension().and_then(|value| value.to_str()) != Some("json") {
                return Err(recovery("definition inventory has an unknown entry"));
            }
            let record = self.load_definition(&path)?;
            entries.push(DefinitionListEntryV1 {
                contract_ref: record.definition.definition().contract_ref().clone(),
                definition_fingerprint: record
                    .definition
                    .definition()
                    .definition_fingerprint()
                    .clone(),
                record_ref: record.record_ref,
                record_fingerprint: record.record_fingerprint,
            });
        }
        entries.sort_by(|a, b| {
            a.contract_ref
                .as_str()
                .cmp(b.contract_ref.as_str())
                .then_with(|| {
                    a.definition_fingerprint
                        .as_str()
                        .cmp(b.definition_fingerprint.as_str())
                })
        });
        if entries
            .windows(2)
            .any(|pair| pair[0].contract_ref == pair[1].contract_ref)
        {
            return Err(recovery("definition inventory repeats an exact ref"));
        }
        Ok(entries)
    }

    fn recover_locked(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let transactions = self.read_dir_bounded(&self.transactions_root(), MAX_RECORDS)?;
        let mut identities = BTreeSet::new();
        let mut pending_count = 0usize;
        for transaction in &transactions {
            self.reject_path(transaction, true)?;
            let name = file_name(transaction)?;
            let (identity, pending) = name
                .strip_suffix(".pending")
                .map(|value| (value, true))
                .or_else(|| name.strip_suffix(".committed").map(|value| (value, false)))
                .ok_or_else(|| recovery("transaction inventory has an unknown entry"))?;
            if !identities.insert(identity.to_owned()) {
                return Err(recovery(
                    "transaction identity appears under more than one terminal suffix",
                ));
            }
            pending_count += usize::from(pending);
        }
        if pending_count > 1 {
            return Err(recovery("more than one pending owner transaction exists"));
        }
        for transaction in &transactions {
            let name = file_name(transaction)?;
            if name.ends_with(".committed") {
                self.validate_transaction(transaction)?;
            } else {
                self.preflight_pending(transaction)?;
            }
        }
        for transaction in transactions {
            let name = file_name(&transaction)?;
            if name.ends_with(".pending") {
                self.recover_pending(&transaction)?;
            }
        }
        Ok(())
    }

    fn preflight_pending(&self, pending: &Path) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let intent_path = pending.join("intent.json");
        let marker_path = pending.join("commit-marker.json");
        if !self.exists(&intent_path)? {
            if self.exists(&marker_path)? {
                return Err(recovery("commit marker exists without an intent"));
            }
            for entry in self.read_dir_bounded(pending, 8)? {
                let name = file_name(&entry)?;
                if !matches!(
                    name.as_str(),
                    "00-definition.json" | "00-transition.json" | "01-head.json" | "02-ledger.json"
                ) {
                    return Err(recovery("unestablished transaction has an unknown entry"));
                }
                self.reject_path(&entry, false)?;
            }
            return Ok(());
        }
        let intent: TransactionIntent = parse_canonical(&self.read_regular(&intent_path)?)?;
        intent.validate()?;
        if file_name(pending)?.strip_suffix(".pending") != Some(intent.transaction_id.as_str()) {
            return Err(recovery(
                "pending transaction directory identity is substituted",
            ));
        }
        let marker_present = self.exists(&marker_path)?;
        self.require_transaction_inventory(pending, &intent, marker_present)?;
        if marker_present {
            let marker: CommitMarker = parse_canonical(&self.read_regular(&marker_path)?)?;
            marker.validate(&intent)?;
        } else {
            self.installed_prefix(&intent)?;
        }
        Ok(())
    }

    fn recover_pending(&self, pending: &Path) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let intent_path = pending.join("intent.json");
        let marker_path = pending.join("commit-marker.json");
        if !self.exists(&intent_path)? {
            if self.exists(&marker_path)? {
                return Err(recovery("commit marker exists without an intent"));
            }
            let mut observed = BTreeSet::new();
            for entry in self.read_dir_bounded(pending, 8)? {
                let name = file_name(&entry)?;
                if !matches!(
                    name.as_str(),
                    "00-definition.json" | "00-transition.json" | "01-head.json" | "02-ledger.json"
                ) {
                    return Err(recovery("unestablished transaction has an unknown entry"));
                }
                self.reject_path(&entry, false)?;
                if !observed.insert(name) {
                    return Err(recovery("unestablished transaction repeats an entry"));
                }
            }
            self.remove_pending(pending)?;
            return Ok(());
        }
        let intent: TransactionIntent = parse_canonical(&self.read_regular(&intent_path)?)?;
        intent.validate()?;
        self.require_transaction_inventory(pending, &intent, self.exists(&marker_path)?)?;
        if file_name(pending)?.strip_suffix(".pending") != Some(intent.transaction_id.as_str()) {
            return Err(recovery(
                "pending transaction directory identity is substituted",
            ));
        }
        let marker = if self.exists(&marker_path)? {
            parse_canonical::<CommitMarker>(&self.read_regular(&marker_path)?)?
        } else {
            let prefix = self.installed_prefix(&intent)?;
            if prefix == 0 {
                self.remove_pending(pending)?;
                return Ok(());
            }
            let marker = CommitMarker::new(&intent)?;
            self.write_new_durable(
                &marker_path,
                &canonical_bytes(&marker)
                    .map_err(|_| durability("recovery marker encoding failed"))?,
            )?;
            marker
        };
        self.finish_transaction(pending, &intent, &marker)?;
        let base = file_name(pending)?
            .strip_suffix(".pending")
            .ok_or_else(|| recovery("pending suffix is invalid"))?
            .to_owned();
        self.rename_dir_durable(
            pending,
            &pending.with_file_name(format!("{base}.committed")),
        )
    }

    fn installed_prefix(
        &self,
        intent: &TransactionIntent,
    ) -> Result<usize, ContractOwnerRepositoryErrorV1> {
        let mut prefix = 0;
        let mut missing = false;
        for output in &intent.outputs {
            let final_path = self.repo_root.join(&output.relative_ref);
            let installed = if self.exists(&final_path)? {
                let observed = FingerprintV1::from_bytes(&self.read_regular(&final_path)?);
                if observed == output.bytes_fingerprint {
                    true
                } else if output.staged_name == "01-head.json"
                    && intent.expected_head_bytes_fingerprint.as_ref() == Some(&observed)
                {
                    false
                } else {
                    return Err(recovery("installed output is unequal"));
                }
            } else if output.staged_name == "01-head.json"
                && intent.expected_head_bytes_fingerprint.is_some()
            {
                return Err(recovery("expected prior head is absent"));
            } else {
                false
            };
            if installed {
                if missing {
                    return Err(recovery("installed outputs are not an exact prefix"));
                }
                prefix += 1;
            } else {
                missing = true;
            }
        }
        Ok(prefix)
    }

    fn finish_transaction(
        &self,
        pending: &Path,
        intent: &TransactionIntent,
        marker: &CommitMarker,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        marker.validate(intent)?;
        for (index, output) in intent.outputs.iter().enumerate() {
            let bytes = self.read_regular(&pending.join(&output.staged_name))?;
            if FingerprintV1::from_bytes(&bytes) != output.bytes_fingerprint {
                return Err(recovery("staged output is substituted"));
            }
            let final_path = self.repo_root.join(&output.relative_ref);
            if output.staged_name == "01-head.json"
                && intent.expected_head_bytes_fingerprint.is_some()
            {
                let expected = match self.exists(&final_path)? {
                    true => self.read_regular(&final_path)?,
                    false => Vec::new(),
                };
                if expected != bytes {
                    self.replace_file_durable(&final_path, &expected, &bytes, intent)?;
                }
                self.maybe_fault(OwnerFaultBoundary::HeadInstalled)?;
                continue;
            }
            if self.exists(&final_path)? {
                let installed = self.read_regular(&final_path)?;
                if installed == bytes {
                    continue;
                }
                if output.staged_name != "01-head.json"
                    || intent.expected_head_bytes_fingerprint.as_ref()
                        != Some(&FingerprintV1::from_bytes(&installed))
                {
                    return Err(recovery("installed output is unequal"));
                }
                return Err(recovery("immutable output is unequal"));
            } else {
                if output.staged_name == "01-head.json"
                    && intent.expected_head_bytes_fingerprint.is_some()
                {
                    return Err(recovery("expected prior head is absent"));
                }
                self.write_new_durable(&final_path, &bytes)?;
            }
            self.maybe_fault(match index {
                0 => OwnerFaultBoundary::PrimaryInstalled,
                1 => OwnerFaultBoundary::HeadInstalled,
                _ => OwnerFaultBoundary::LedgerInstalled,
            })?;
        }
        Ok(())
    }

    fn validate_inventory(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let mut definitions = BTreeSet::new();
        let mut transitions = BTreeSet::new();
        let mut heads = BTreeSet::new();
        let mut ledgers = BTreeSet::new();
        for transaction in self.read_dir_bounded(&self.transactions_root(), MAX_RECORDS)? {
            let name = file_name(&transaction)?;
            if !name.ends_with(".committed") {
                return Err(recovery("pending transaction remains"));
            }
            let intent = self.validate_transaction(&transaction)?;
            for output in intent.outputs {
                if output.relative_ref.contains("/definitions/") {
                    if !definitions.insert(output.relative_ref) {
                        return Err(recovery(
                            "immutable definition is claimed by multiple transactions",
                        ));
                    }
                } else if output.relative_ref.contains("/transitions/") {
                    if !transitions.insert(output.relative_ref) {
                        return Err(recovery(
                            "immutable transition is claimed by multiple transactions",
                        ));
                    }
                } else if output.relative_ref.contains("/heads/") {
                    heads.insert(output.relative_ref);
                } else if output.relative_ref.contains("/idempotency/") {
                    if !ledgers.insert(output.relative_ref) {
                        return Err(recovery(
                            "immutable replay ledger is claimed by multiple transactions",
                        ));
                    }
                } else {
                    return Err(recovery("unknown committed output class"));
                }
            }
        }
        self.require_inventory(&self.definitions_root(), &definitions)?;
        self.require_inventory(&self.transitions_root(), &transitions)?;
        self.require_inventory(&self.heads_root(), &heads)?;
        self.require_inventory(&self.idempotency_root(), &ledgers)?;
        self.validate_semantic_inventory()
    }

    fn validate_semantic_inventory(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let entries = self.load_entries()?;
        let definitions = entries
            .iter()
            .map(|entry| {
                (
                    entry.definition_fingerprint.clone(),
                    entry.contract_ref.clone(),
                )
            })
            .collect::<std::collections::BTreeMap<_, _>>();
        for path in self.read_dir_bounded(&self.transitions_root(), MAX_RECORDS)? {
            let transition: ContractLifecycleTransitionV1 =
                parse_canonical(&self.read_regular(&path)?)?;
            if definitions.get(transition.definition_fingerprint())
                != Some(transition.contract_ref())
            {
                return Err(recovery(
                    "transition is not bound to an exact durable definition",
                ));
            }
        }
        for path in self.read_dir_bounded(&self.heads_root(), MAX_RECORDS)? {
            let head: HeadRecord = parse_canonical(&self.read_regular(&path)?)?;
            if definitions.get(&head.definition_fingerprint) != Some(&head.contract_ref) {
                return Err(recovery("head is not bound to an exact durable definition"));
            }
        }
        for path in self.read_dir_bounded(&self.idempotency_root(), MAX_RECORDS)? {
            let bytes = self.read_regular(&path)?;
            let value: serde_json::Value = parse_canonical(&bytes)?;
            match value.get("operation").and_then(serde_json::Value::as_str) {
                Some("contract.definition.append") => {
                    let ledger: AppendLedger = parse_canonical(&bytes)?;
                    self.replay(&ledger.key_fingerprint, &ledger.request_fingerprint)?
                        .ok_or_else(|| recovery("append replay ledger is not reachable"))?;
                }
                Some("contract.lifecycle.transition") => {
                    let ledger: TransitionLedger = parse_canonical(&bytes)?;
                    self.replay_transition(&ledger.key_fingerprint, &ledger.request_fingerprint)?
                        .ok_or_else(|| recovery("transition replay ledger is not reachable"))?;
                }
                _ => return Err(recovery("replay ledger operation is unknown")),
            }
        }
        Ok(())
    }

    fn validate_transaction(
        &self,
        path: &Path,
    ) -> Result<TransactionIntent, ContractOwnerRepositoryErrorV1> {
        let intent: TransactionIntent =
            parse_canonical(&self.read_regular(&path.join("intent.json"))?)?;
        intent.validate()?;
        self.require_transaction_inventory(path, &intent, true)?;
        if file_name(path)?.strip_suffix(".committed") != Some(intent.transaction_id.as_str()) {
            return Err(recovery(
                "committed transaction directory identity is substituted",
            ));
        }
        let marker: CommitMarker =
            parse_canonical(&self.read_regular(&path.join("commit-marker.json"))?)?;
        marker.validate(&intent)?;
        let primary_bytes = self.read_regular(&path.join(&intent.outputs[0].staged_name))?;
        let historical_head_bytes =
            self.read_regular(&path.join(&intent.outputs[1].staged_name))?;
        let ledger_bytes = self.read_regular(&path.join(&intent.outputs[2].staged_name))?;
        let historical_head: HeadRecord = parse_canonical(&historical_head_bytes)?;
        historical_head.validate_self()?;
        match intent.operation.as_str() {
            "contract.definition.append" => {
                let definition: AdmittedContractDefinitionV1 = parse_canonical(&primary_bytes)?;
                historical_head.validate(&definition, &[])?;
                let ledger: AppendLedger = parse_canonical(&ledger_bytes)?;
                ledger.validate()?;
                if ledger.key_fingerprint != intent.key_fingerprint
                    || ledger.request_fingerprint != intent.request_fingerprint
                    || ledger.definition_fingerprint
                        != *definition.definition().definition_fingerprint()
                    || ledger.lifecycle_fingerprint != historical_head.lifecycle_fingerprint
                {
                    return Err(recovery(
                        "committed append replay ledger does not close its transaction",
                    ));
                }
            }
            "contract.lifecycle.transition" => {
                let transition: ContractLifecycleTransitionV1 = parse_canonical(&primary_bytes)?;
                if historical_head.contract_ref != *transition.contract_ref()
                    || historical_head.definition_fingerprint
                        != *transition.definition_fingerprint()
                    || historical_head.state != transition.to_state()
                    || historical_head.transition_ref.as_deref()
                        != Some(transition.transition_ref())
                    || historical_head.lifecycle_fingerprint
                        != *transition.resulting_lifecycle_fingerprint()
                {
                    return Err(recovery(
                        "committed transition does not derive its staged head",
                    ));
                }
                let ledger: TransitionLedger = parse_canonical(&ledger_bytes)?;
                ledger.validate()?;
                if ledger.key_fingerprint != intent.key_fingerprint
                    || ledger.request_fingerprint != intent.request_fingerprint
                    || ledger.transition_fingerprint != *transition.transition_fingerprint()
                    || ledger.resulting_lifecycle_fingerprint
                        != historical_head.lifecycle_fingerprint
                {
                    return Err(recovery(
                        "committed transition replay ledger does not close its transaction",
                    ));
                }
            }
            _ => return Err(recovery("committed transaction operation is unknown")),
        }
        for output in &intent.outputs {
            let staged = self.read_regular(&path.join(&output.staged_name))?;
            if FingerprintV1::from_bytes(&staged) != output.bytes_fingerprint {
                return Err(recovery("committed staged output is substituted"));
            }
            if output.staged_name == "01-head.json" {
                let head: HeadRecord = parse_canonical(&staged)?;
                if head.definition_fingerprint
                    != definition_fingerprint_from_head_ref(&output.relative_ref)?
                {
                    return Err(recovery("committed staged head ref is substituted"));
                }
                continue;
            }
            let installed = self.read_regular(&self.repo_root.join(&output.relative_ref))?;
            if staged != installed {
                return Err(recovery("committed output is not exactly reachable"));
            }
        }
        Ok(intent)
    }

    fn ensure_layout(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        for relative in [
            ".handbook/state/contracts",
            CONTRACT_OWNER_REPOSITORY_RELATIVE_ROOT_V1,
            ".handbook/state/contracts/v1/definitions",
            ".handbook/state/contracts/v1/transitions",
            ".handbook/state/contracts/v1/heads",
            ".handbook/state/contracts/v1/idempotency",
            ".handbook/state/contracts/v1/transactions",
        ] {
            self.repository_root
                .create_directories(relative)
                .map_err(map_durability)?;
        }
        Ok(())
    }

    fn with_lock<T>(
        &self,
        action: impl FnOnce(&Self) -> Result<T, ContractOwnerRepositoryErrorV1>,
    ) -> Result<T, ContractOwnerRepositoryErrorV1> {
        let _process = OWNER_PROCESS_LOCK
            .lock()
            .map_err(|_| durability("process lock is uncertain"))?;
        self.repository_root
            .create_directories(".handbook/state/locks")
            .map_err(map_durability)?;
        let file = self
            .repository_root
            .acquire_exclusive_lock(LOCK_RELATIVE_PATH)
            .map_err(map_durability)?;
        let result = action(self);
        let release = file.release().map_err(map_durability);
        match (result, release) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(error), _) | (Ok(_), Err(error)) => Err(error),
        }
    }

    fn relative_path(&self, path: &Path) -> Result<String, ContractOwnerRepositoryErrorV1> {
        path.strip_prefix(&self.repo_root)
            .map_err(|_| unsafe_fs("owner path escapes the retained repository root"))?
            .components()
            .map(|component| match component {
                Component::Normal(value) => value
                    .to_str()
                    .map(str::to_owned)
                    .ok_or_else(|| unsafe_fs("owner path is not UTF-8")),
                _ => Err(unsafe_fs("owner path is not normalized")),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|parts| parts.join("/"))
    }

    fn entry_kind(
        &self,
        path: &Path,
    ) -> Result<Option<RepositoryDirectoryEntryKindV1>, ContractOwnerRepositoryErrorV1> {
        if path == self.repo_root {
            return Ok(Some(RepositoryDirectoryEntryKindV1::Directory));
        }
        let relative = self.relative_path(path)?;
        let relative = Path::new(&relative);
        let parent = relative
            .parent()
            .and_then(Path::to_str)
            .ok_or_else(|| unsafe_fs("owner entry parent is not UTF-8"))?;
        let name = relative
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| unsafe_fs("owner entry name is not UTF-8"))?;
        Ok(self
            .repository_root
            .list_directory_bounded(parent, MAX_RECORDS)
            .map_err(map_durability)?
            .into_iter()
            .find(|entry| entry.name() == name)
            .map(|entry| entry.kind()))
    }

    fn exists(&self, path: &Path) -> Result<bool, ContractOwnerRepositoryErrorV1> {
        Ok(self.entry_kind(path)?.is_some())
    }

    fn reject_path(
        &self,
        path: &Path,
        directory: bool,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let expected = if directory {
            RepositoryDirectoryEntryKindV1::Directory
        } else {
            RepositoryDirectoryEntryKindV1::RegularFile
        };
        if self.entry_kind(path)? != Some(expected) {
            return Err(unsafe_fs("owner path has an unsafe or substituted kind"));
        }
        Ok(())
    }

    fn read_regular(&self, path: &Path) -> Result<Vec<u8>, ContractOwnerRepositoryErrorV1> {
        match self
            .repository_root
            .read_regular_bounded(&self.relative_path(path)?, MAX_RECORD_BYTES)
            .map_err(map_durability)?
        {
            RepositoryReadV1::Found(bytes) => Ok(bytes),
            RepositoryReadV1::Absent => Err(recovery("owner record is absent")),
        }
    }

    fn read_dir_bounded(
        &self,
        path: &Path,
        limit: usize,
    ) -> Result<Vec<PathBuf>, ContractOwnerRepositoryErrorV1> {
        self.repository_root
            .list_directory_bounded(&self.relative_path(path)?, limit)
            .map_err(map_durability)?
            .into_iter()
            .map(|entry| Ok(path.join(entry.name())))
            .collect()
    }

    fn write_new_durable(
        &self,
        path: &Path,
        bytes: &[u8],
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        if bytes.len() > MAX_RECORD_BYTES {
            return Err(bound("owner record is too large"));
        }
        self.repository_root
            .write_new_durable(&self.relative_path(path)?, bytes)
            .map_err(map_durability)
    }

    fn create_dir_durable(&self, path: &Path) -> Result<(), ContractOwnerRepositoryErrorV1> {
        self.repository_root
            .create_directory_new_durable(&self.relative_path(path)?)
            .map_err(map_durability)
    }

    fn rename_dir_durable(
        &self,
        source: &Path,
        target: &Path,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        self.repository_root
            .rename_directory_noreplace_durable(
                &self.relative_path(source)?,
                &self.relative_path(target)?,
            )
            .map_err(map_durability)
    }

    fn remove_pending(&self, path: &Path) -> Result<(), ContractOwnerRepositoryErrorV1> {
        for entry in self.read_dir_bounded(path, 8)? {
            self.repository_root
                .remove_regular_durable(&self.relative_path(&entry)?)
                .map_err(map_durability)?;
        }
        self.repository_root
            .remove_empty_directory_durable(&self.relative_path(path)?)
            .map_err(map_durability)
    }

    fn require_transaction_inventory(
        &self,
        path: &Path,
        intent: &TransactionIntent,
        marker_present: bool,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let mut expected = intent
            .outputs
            .iter()
            .map(|output| output.staged_name.clone())
            .collect::<BTreeSet<_>>();
        if expected.len() != intent.outputs.len() {
            return Err(recovery("transaction repeats a staged filename"));
        }
        expected.insert("intent.json".to_owned());
        if marker_present {
            expected.insert("commit-marker.json".to_owned());
        }
        let observed = self
            .read_dir_bounded(path, 8)?
            .into_iter()
            .map(|entry| {
                self.reject_path(&entry, false)?;
                file_name(&entry)
            })
            .collect::<Result<BTreeSet<_>, _>>()?;
        if observed != expected {
            return Err(recovery("transaction directory inventory is not exact"));
        }
        Ok(())
    }

    fn require_inventory(
        &self,
        directory: &Path,
        expected: &BTreeSet<String>,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let observed = self
            .read_dir_bounded(directory, MAX_RECORDS)?
            .into_iter()
            .map(|path| {
                self.reject_path(&path, false)?;
                self.relative_path(&path)
            })
            .collect::<Result<BTreeSet<_>, _>>()?;
        if &observed != expected {
            return Err(recovery("inventory is not committed-reachable"));
        }
        Ok(())
    }

    fn replace_file_durable(
        &self,
        path: &Path,
        expected: &[u8],
        replacement: &[u8],
        intent: &TransactionIntent,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let parent = path
            .parent()
            .ok_or_else(|| unsafe_fs("replace path has no parent"))?;
        let name = path
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| unsafe_fs("replace path has no UTF-8 name"))?;
        let prepared = parent.join(format!(".{name}.{}.owner-prepared", intent.transaction_id));
        let displaced = parent.join(format!(".{name}.{}.owner-displaced", intent.transaction_id));
        let current_bytes = self
            .exists(path)?
            .then(|| self.read_regular(path))
            .transpose()?;
        let prepared_bytes = self
            .exists(&prepared)?
            .then(|| self.read_regular(&prepared))
            .transpose()?;
        let displaced_bytes = self
            .exists(&displaced)?
            .then(|| self.read_regular(&displaced))
            .transpose()?;
        let expected = if expected.is_empty() && current_bytes.is_none() {
            displaced_bytes
                .as_deref()
                .ok_or_else(|| recovery("absent head has no exact displaced basis"))?
        } else {
            expected
        };
        if intent.expected_head_bytes_fingerprint.as_ref()
            != Some(&FingerprintV1::from_bytes(expected))
        {
            return Err(recovery("head replacement expected basis is substituted"));
        }

        match (
            current_bytes.as_deref(),
            prepared_bytes.as_deref(),
            displaced_bytes.as_deref(),
        ) {
            (Some(current), None, None) if current == replacement => return Ok(()),
            (Some(current), None, None) if current == expected => {
                self.write_new_durable(&prepared, replacement)?;
                self.repository_root
                    .replace_exact_durable(
                        &self.relative_path(path)?,
                        expected,
                        &self.relative_path(&prepared)?,
                        &self.relative_path(&displaced)?,
                    )
                    .map_err(map_durability)?;
            }
            (Some(current), Some(candidate), None)
                if current == expected && candidate == replacement =>
            {
                self.repository_root
                    .replace_exact_durable(
                        &self.relative_path(path)?,
                        expected,
                        &self.relative_path(&prepared)?,
                        &self.relative_path(&displaced)?,
                    )
                    .map_err(map_durability)?;
            }
            (None, Some(candidate), Some(old)) if candidate == replacement && old == expected => {
                self.repository_root
                    .rename_noreplace_durable(
                        &self.relative_path(&prepared)?,
                        &self.relative_path(path)?,
                    )
                    .map_err(map_durability)?;
            }
            (Some(current), None, Some(old)) if current == replacement && old == expected => {}
            _ => return Err(recovery("head replacement state is ambiguous")),
        }
        if self.read_regular(path)? != replacement {
            return Err(durability("head replacement reobservation failed"));
        }
        if self.exists(&prepared)? {
            return Err(recovery("published head retained a prepared pathname"));
        }
        if self.exists(&displaced)? {
            if self.read_regular(&displaced)? != expected {
                return Err(recovery("displaced head basis is substituted"));
            }
            self.repository_root
                .remove_regular_durable(&self.relative_path(&displaced)?)
                .map_err(map_durability)?;
        }
        Ok(())
    }

    fn owner_root(&self) -> PathBuf {
        self.repo_root
            .join(CONTRACT_OWNER_REPOSITORY_RELATIVE_ROOT_V1)
    }
    fn definitions_root(&self) -> PathBuf {
        self.owner_root().join("definitions")
    }
    fn transitions_root(&self) -> PathBuf {
        self.owner_root().join("transitions")
    }
    fn heads_root(&self) -> PathBuf {
        self.owner_root().join("heads")
    }
    fn idempotency_root(&self) -> PathBuf {
        self.owner_root().join("idempotency")
    }
    fn transactions_root(&self) -> PathBuf {
        self.owner_root().join("transactions")
    }
    fn definition_ref(&self, fp: &FingerprintV1) -> String {
        format!(
            "{CONTRACT_OWNER_REPOSITORY_RELATIVE_ROOT_V1}/definitions/{}.json",
            fingerprint_hex(fp)
        )
    }
    fn head_ref(&self, fp: &FingerprintV1) -> String {
        format!(
            "{CONTRACT_OWNER_REPOSITORY_RELATIVE_ROOT_V1}/heads/{}.json",
            fingerprint_hex(fp)
        )
    }
    fn transition_ref(&self, fp: &FingerprintV1) -> String {
        format!(
            "{CONTRACT_OWNER_REPOSITORY_RELATIVE_ROOT_V1}/transitions/{}.json",
            fingerprint_hex(fp)
        )
    }
    fn ledger_ref(&self, fp: &FingerprintV1) -> String {
        format!(
            "{CONTRACT_OWNER_REPOSITORY_RELATIVE_ROOT_V1}/idempotency/{}.json",
            fingerprint_hex(fp)
        )
    }
    fn definition_path(&self, fp: &FingerprintV1) -> PathBuf {
        self.repo_root.join(self.definition_ref(fp))
    }
    fn head_path(&self, fp: &FingerprintV1) -> PathBuf {
        self.repo_root.join(self.head_ref(fp))
    }
    fn ledger_path(&self, fp: &FingerprintV1) -> PathBuf {
        self.repo_root.join(self.ledger_ref(fp))
    }
}

#[derive(Serialize)]
struct ListRootPreimage<'a> {
    schema_ref: &'static str,
    retention: &'static str,
    definitions: &'a [DefinitionListEntryV1],
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct DefinitionCursorV1 {
    schema_ref: String,
    root: FingerprintV1,
    offset: usize,
    page_size: usize,
    cursor_fingerprint: FingerprintV1,
}
#[derive(Serialize)]
struct DefinitionCursorPreimage<'a> {
    schema_ref: &'static str,
    domain: &'static str,
    root: &'a FingerprintV1,
    offset: usize,
    page_size: usize,
}
#[derive(Serialize)]
struct AppendRequestPreimage<'a> {
    schema_ref: &'static str,
    contract_ref: &'a ExactRefV1,
    definition_fingerprint: &'a FingerprintV1,
}
#[derive(Serialize)]
struct TransitionRequestPreimage<'a> {
    schema_ref: &'static str,
    contract_ref: &'a ExactRefV1,
    definition_fingerprint: &'a FingerprintV1,
    input: &'a ContractLifecycleTransitionInputV1,
}
#[derive(Serialize)]
struct KeyPreimage<'a> {
    schema_ref: &'static str,
    key: &'a str,
}
#[derive(Serialize)]
struct TransactionIdPreimage<'a> {
    schema_ref: &'static str,
    operation: &'a str,
    key_fingerprint: &'a FingerprintV1,
    request_fingerprint: &'a FingerprintV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct StagedOutput {
    staged_name: String,
    relative_ref: String,
    bytes_fingerprint: FingerprintV1,
}
impl StagedOutput {
    fn new(
        name: &str,
        relative_ref: &str,
        bytes: &[u8],
    ) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        validate_staged(name)?;
        validate_relative(relative_ref)?;
        Ok(Self {
            staged_name: name.to_owned(),
            relative_ref: relative_ref.to_owned(),
            bytes_fingerprint: FingerprintV1::from_bytes(bytes),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct TransactionIntent {
    schema_ref: String,
    transaction_id: String,
    operation: String,
    key_fingerprint: FingerprintV1,
    request_fingerprint: FingerprintV1,
    expected_head_bytes_fingerprint: Option<FingerprintV1>,
    outputs: Vec<StagedOutput>,
    intent_fingerprint: FingerprintV1,
}
impl TransactionIntent {
    fn new(
        id: &str,
        operation: &str,
        key: &FingerprintV1,
        request: &FingerprintV1,
        expected_head_bytes_fingerprint: Option<FingerprintV1>,
        outputs: Vec<StagedOutput>,
    ) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        let fingerprint = canonical_fingerprint(&IntentPreimage {
            schema_ref: "handbook.contract-owner-transaction-intent@1.0.0",
            transaction_id: id,
            operation,
            key_fingerprint: key,
            request_fingerprint: request,
            expected_head_bytes_fingerprint: expected_head_bytes_fingerprint.as_ref(),
            outputs: &outputs,
        })
        .map_err(|_| durability("intent fingerprint failed"))?;
        Ok(Self {
            schema_ref: "handbook.contract-owner-transaction-intent@1.0.0".to_owned(),
            transaction_id: id.to_owned(),
            operation: operation.to_owned(),
            key_fingerprint: key.clone(),
            request_fingerprint: request.clone(),
            expected_head_bytes_fingerprint,
            outputs,
            intent_fingerprint: fingerprint,
        })
    }
    fn validate(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        if self.schema_ref != "handbook.contract-owner-transaction-intent@1.0.0"
            || !matches!(
                self.operation.as_str(),
                "contract.definition.append" | "contract.lifecycle.transition"
            )
            || self.outputs.len() != 3
            || self.transaction_id.len() != 64
            || !self
                .transaction_id
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        {
            return Err(recovery("transaction intent shape is invalid"));
        }
        for output in &self.outputs {
            validate_staged(&output.staged_name)?;
            validate_relative(&output.relative_ref)?;
        }
        let expected_classes: [(&str, &str); 3] = match self.operation.as_str() {
            "contract.definition.append" => [
                ("00-definition.json", "/definitions/"),
                ("01-head.json", "/heads/"),
                ("02-ledger.json", "/idempotency/"),
            ],
            "contract.lifecycle.transition" => [
                ("00-transition.json", "/transitions/"),
                ("01-head.json", "/heads/"),
                ("02-ledger.json", "/idempotency/"),
            ],
            _ => return Err(recovery("transaction operation is invalid")),
        };
        if self
            .outputs
            .iter()
            .zip(expected_classes)
            .any(|(output, (name, class))| {
                let prefix = format!("{CONTRACT_OWNER_REPOSITORY_RELATIVE_ROOT_V1}{class}");
                let basename = output.relative_ref.strip_prefix(&prefix);
                output.staged_name != name
                    || !basename.is_some_and(|value| {
                        value.len() == 69
                            && value.ends_with(".json")
                            && value[..64]
                                .bytes()
                                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
                    })
            })
        {
            return Err(recovery(
                "transaction outputs are outside their exact owner classes",
            ));
        }
        if (self.operation == "contract.definition.append"
            && self.expected_head_bytes_fingerprint.is_some())
            || (self.operation == "contract.lifecycle.transition"
                && self.expected_head_bytes_fingerprint.is_none())
        {
            return Err(recovery(
                "transaction expected head basis does not match its operation",
            ));
        }
        let expected_id = canonical_fingerprint(&TransactionIdPreimage {
            schema_ref: "handbook.contract-owner-transaction-identity@1.0.0",
            operation: &self.operation,
            key_fingerprint: &self.key_fingerprint,
            request_fingerprint: &self.request_fingerprint,
        })
        .map_err(|_| recovery("transaction identity cannot be rederived"))?;
        if self.transaction_id != fingerprint_hex(&expected_id) {
            return Err(recovery("transaction identity is substituted"));
        }
        let expected = Self::new(
            &self.transaction_id,
            &self.operation,
            &self.key_fingerprint,
            &self.request_fingerprint,
            self.expected_head_bytes_fingerprint.clone(),
            self.outputs.clone(),
        )?;
        if expected != *self {
            return Err(recovery("transaction intent is substituted"));
        }
        Ok(())
    }
}
#[derive(Serialize)]
struct IntentPreimage<'a> {
    schema_ref: &'static str,
    transaction_id: &'a str,
    operation: &'a str,
    key_fingerprint: &'a FingerprintV1,
    request_fingerprint: &'a FingerprintV1,
    expected_head_bytes_fingerprint: Option<&'a FingerprintV1>,
    outputs: &'a [StagedOutput],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct CommitMarker {
    schema_ref: String,
    intent_fingerprint: FingerprintV1,
    marker_fingerprint: FingerprintV1,
}
impl CommitMarker {
    fn new(intent: &TransactionIntent) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        let fingerprint = canonical_fingerprint(&MarkerPreimage {
            schema_ref: "handbook.contract-owner-commit-marker@1.0.0",
            intent_fingerprint: &intent.intent_fingerprint,
        })
        .map_err(|_| durability("marker fingerprint failed"))?;
        Ok(Self {
            schema_ref: "handbook.contract-owner-commit-marker@1.0.0".to_owned(),
            intent_fingerprint: intent.intent_fingerprint.clone(),
            marker_fingerprint: fingerprint,
        })
    }
    fn validate(&self, intent: &TransactionIntent) -> Result<(), ContractOwnerRepositoryErrorV1> {
        if self != &Self::new(intent)? {
            return Err(recovery("commit marker is substituted"));
        }
        Ok(())
    }
}
#[derive(Serialize)]
struct MarkerPreimage<'a> {
    schema_ref: &'static str,
    intent_fingerprint: &'a FingerprintV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct HeadRecord {
    schema_ref: String,
    contract_ref: ExactRefV1,
    definition_fingerprint: FingerprintV1,
    state: LifecycleStateV1,
    transition_ref: Option<String>,
    lifecycle_fingerprint: FingerprintV1,
    transition_count: usize,
    head_fingerprint: FingerprintV1,
}
impl HeadRecord {
    fn new(
        definition: &AdmittedContractDefinitionV1,
        head: &LifecycleHeadV1,
        count: usize,
    ) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        let fingerprint = canonical_fingerprint(&HeadPreimage {
            schema_ref: "handbook.contract-lifecycle-head@1.0.0",
            contract_ref: definition.definition().contract_ref(),
            definition_fingerprint: definition.definition().definition_fingerprint(),
            state: head.state(),
            transition_ref: head.transition_ref(),
            lifecycle_fingerprint: head.lifecycle_fingerprint(),
            transition_count: count,
        })
        .map_err(|_| durability("head fingerprint failed"))?;
        Ok(Self {
            schema_ref: "handbook.contract-lifecycle-head@1.0.0".to_owned(),
            contract_ref: definition.definition().contract_ref().clone(),
            definition_fingerprint: definition.definition().definition_fingerprint().clone(),
            state: head.state(),
            transition_ref: head.transition_ref().map(str::to_owned),
            lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
            transition_count: count,
            head_fingerprint: fingerprint,
        })
    }
    fn validate(
        &self,
        definition: &AdmittedContractDefinitionV1,
        chain: &[ContractLifecycleTransitionV1],
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let head = current_lifecycle_head(definition, chain).map_err(map_lifecycle)?;
        if self != &Self::new(definition, &head, chain.len())? {
            return Err(substituted("derived head is substituted"));
        }
        Ok(())
    }

    fn validate_self(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let expected = canonical_fingerprint(&HeadPreimage {
            schema_ref: "handbook.contract-lifecycle-head@1.0.0",
            contract_ref: &self.contract_ref,
            definition_fingerprint: &self.definition_fingerprint,
            state: self.state,
            transition_ref: self.transition_ref.as_deref(),
            lifecycle_fingerprint: &self.lifecycle_fingerprint,
            transition_count: self.transition_count,
        })
        .map_err(|_| recovery("head fingerprint cannot be rederived"))?;
        if self.schema_ref != "handbook.contract-lifecycle-head@1.0.0"
            || self.head_fingerprint != expected
        {
            return Err(substituted("derived head record is substituted"));
        }
        Ok(())
    }
}
#[derive(Serialize)]
struct HeadPreimage<'a> {
    schema_ref: &'static str,
    contract_ref: &'a ExactRefV1,
    definition_fingerprint: &'a FingerprintV1,
    state: LifecycleStateV1,
    transition_ref: Option<&'a str>,
    lifecycle_fingerprint: &'a FingerprintV1,
    transition_count: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct AppendLedger {
    schema_ref: String,
    operation: String,
    key_fingerprint: FingerprintV1,
    request_fingerprint: FingerprintV1,
    contract_ref: String,
    definition_fingerprint: FingerprintV1,
    record_ref: String,
    record_fingerprint: FingerprintV1,
    lifecycle_fingerprint: FingerprintV1,
    ledger_fingerprint: FingerprintV1,
}
impl AppendLedger {
    #[allow(clippy::too_many_arguments)]
    fn new(
        key: &FingerprintV1,
        request: &FingerprintV1,
        contract_ref: &ExactRefV1,
        definition: &FingerprintV1,
        record_ref: &str,
        record: &FingerprintV1,
        lifecycle: &FingerprintV1,
    ) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        let fingerprint = canonical_fingerprint(&LedgerPreimage {
            schema_ref: "handbook.contract-definition-append-replay@1.0.0",
            operation: "contract.definition.append",
            key_fingerprint: key,
            request_fingerprint: request,
            contract_ref: contract_ref.as_str(),
            definition_fingerprint: definition,
            record_ref,
            record_fingerprint: record,
            lifecycle_fingerprint: lifecycle,
        })
        .map_err(|_| durability("ledger fingerprint failed"))?;
        Ok(Self {
            schema_ref: "handbook.contract-definition-append-replay@1.0.0".to_owned(),
            operation: "contract.definition.append".to_owned(),
            key_fingerprint: key.clone(),
            request_fingerprint: request.clone(),
            contract_ref: contract_ref.to_string(),
            definition_fingerprint: definition.clone(),
            record_ref: record_ref.to_owned(),
            record_fingerprint: record.clone(),
            lifecycle_fingerprint: lifecycle.clone(),
            ledger_fingerprint: fingerprint,
        })
    }
    fn validate(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let contract_ref =
            ExactRefV1::parse(&self.contract_ref).map_err(|_| recovery("ledger ref invalid"))?;
        if self
            != &Self::new(
                &self.key_fingerprint,
                &self.request_fingerprint,
                &contract_ref,
                &self.definition_fingerprint,
                &self.record_ref,
                &self.record_fingerprint,
                &self.lifecycle_fingerprint,
            )?
        {
            return Err(recovery("ledger is substituted"));
        }
        Ok(())
    }
}
#[derive(Serialize)]
struct LedgerPreimage<'a> {
    schema_ref: &'static str,
    operation: &'static str,
    key_fingerprint: &'a FingerprintV1,
    request_fingerprint: &'a FingerprintV1,
    contract_ref: &'a str,
    definition_fingerprint: &'a FingerprintV1,
    record_ref: &'a str,
    record_fingerprint: &'a FingerprintV1,
    lifecycle_fingerprint: &'a FingerprintV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct TransitionLedger {
    schema_ref: String,
    operation: String,
    key_fingerprint: FingerprintV1,
    request_fingerprint: FingerprintV1,
    contract_ref: String,
    definition_fingerprint: FingerprintV1,
    transition_ref: String,
    transition_fingerprint: FingerprintV1,
    resulting_lifecycle_fingerprint: FingerprintV1,
    ledger_fingerprint: FingerprintV1,
}

impl TransitionLedger {
    #[allow(clippy::too_many_arguments)]
    fn new(
        key: &FingerprintV1,
        request: &FingerprintV1,
        contract_ref: &ExactRefV1,
        definition_fingerprint: &FingerprintV1,
        transition_ref: &str,
        transition_fingerprint: &FingerprintV1,
        resulting_lifecycle_fingerprint: &FingerprintV1,
    ) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        let ledger_fingerprint = canonical_fingerprint(&TransitionLedgerPreimage {
            schema_ref: "handbook.contract-lifecycle-transition-replay@1.0.0",
            operation: "contract.lifecycle.transition",
            key_fingerprint: key,
            request_fingerprint: request,
            contract_ref: contract_ref.as_str(),
            definition_fingerprint,
            transition_ref,
            transition_fingerprint,
            resulting_lifecycle_fingerprint,
        })
        .map_err(|_| durability("transition ledger fingerprint failed"))?;
        Ok(Self {
            schema_ref: "handbook.contract-lifecycle-transition-replay@1.0.0".to_owned(),
            operation: "contract.lifecycle.transition".to_owned(),
            key_fingerprint: key.clone(),
            request_fingerprint: request.clone(),
            contract_ref: contract_ref.to_string(),
            definition_fingerprint: definition_fingerprint.clone(),
            transition_ref: transition_ref.to_owned(),
            transition_fingerprint: transition_fingerprint.clone(),
            resulting_lifecycle_fingerprint: resulting_lifecycle_fingerprint.clone(),
            ledger_fingerprint,
        })
    }

    fn validate(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let contract_ref = ExactRefV1::parse(&self.contract_ref)
            .map_err(|_| recovery("transition ledger contract ref is invalid"))?;
        let expected = Self::new(
            &self.key_fingerprint,
            &self.request_fingerprint,
            &contract_ref,
            &self.definition_fingerprint,
            &self.transition_ref,
            &self.transition_fingerprint,
            &self.resulting_lifecycle_fingerprint,
        )?;
        if self != &expected {
            return Err(recovery("transition ledger is substituted"));
        }
        Ok(())
    }
}

#[derive(Serialize)]
struct TransitionLedgerPreimage<'a> {
    schema_ref: &'static str,
    operation: &'static str,
    key_fingerprint: &'a FingerprintV1,
    request_fingerprint: &'a FingerprintV1,
    contract_ref: &'a str,
    definition_fingerprint: &'a FingerprintV1,
    transition_ref: &'a str,
    transition_fingerprint: &'a FingerprintV1,
    resulting_lifecycle_fingerprint: &'a FingerprintV1,
}

fn validate_key(key: &str) -> Result<(), ContractOwnerRepositoryErrorV1> {
    if !(16..=128).contains(&key.len())
        || !key.is_ascii()
        || !key
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        return Err(repository_error(
            ContractOwnerRepositoryErrorKindV1::InvalidRequest,
            "idempotency key is outside its admitted syntax and bound",
        ));
    }
    Ok(())
}

fn validate_relative(value: &str) -> Result<(), ContractOwnerRepositoryErrorV1> {
    let path = Path::new(value);
    if value.is_empty()
        || value.len() > 512
        || path.is_absolute()
        || value.contains('\\')
        || !value.starts_with(".handbook/")
        || path
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err(unsafe_fs("owner path is not repository relative"));
    }
    Ok(())
}

fn definition_fingerprint_from_head_ref(
    value: &str,
) -> Result<FingerprintV1, ContractOwnerRepositoryErrorV1> {
    let prefix = format!("{CONTRACT_OWNER_REPOSITORY_RELATIVE_ROOT_V1}/heads/");
    let hex = value
        .strip_prefix(&prefix)
        .and_then(|value| value.strip_suffix(".json"))
        .filter(|value| {
            value.len() == 64
                && value
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        })
        .ok_or_else(|| recovery("head output ref is invalid"))?;
    FingerprintV1::parse(&format!("sha256:{hex}"))
        .map_err(|_| recovery("head output fingerprint is invalid"))
}

fn validate_staged(value: &str) -> Result<(), ContractOwnerRepositoryErrorV1> {
    if value.is_empty()
        || value.len() > 64
        || Path::new(value).components().count() != 1
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.'))
    {
        return Err(recovery("staged name is invalid"));
    }
    Ok(())
}

fn parse_canonical<T: for<'de> Deserialize<'de> + Serialize>(
    bytes: &[u8],
) -> Result<T, ContractOwnerRepositoryErrorV1> {
    let value: T =
        serde_json::from_slice(bytes).map_err(|_| substituted("record type is invalid"))?;
    if canonical_bytes(&value).map_err(|_| substituted("record canonicalization failed"))? != bytes
    {
        return Err(substituted("record bytes are not canonical"));
    }
    Ok(value)
}

fn encode_cursor(
    root: &FingerprintV1,
    offset: usize,
    page_size: usize,
) -> Result<String, ContractOwnerRepositoryErrorV1> {
    let cursor_fingerprint = canonical_fingerprint(&DefinitionCursorPreimage {
        schema_ref: "handbook.contract-definition-list-cursor@1.0.0",
        domain: "contract-definition-list-cursor",
        root,
        offset,
        page_size,
    })
    .map_err(|_| durability("definition cursor fingerprint could not be derived"))?;
    let bytes = canonical_bytes(&DefinitionCursorV1 {
        schema_ref: "handbook.contract-definition-list-cursor@1.0.0".to_owned(),
        root: root.clone(),
        offset,
        page_size,
        cursor_fingerprint,
    })
    .map_err(|_| durability("definition cursor could not be encoded"))?;
    Ok(format!("hcm-owner-v1.{}", hex_encode(&bytes)))
}

fn decode_cursor(
    token: &str,
    root: &FingerprintV1,
    page_size: usize,
) -> Result<usize, ContractOwnerRepositoryErrorV1> {
    let encoded = token
        .strip_prefix("hcm-owner-v1.")
        .ok_or_else(|| substituted("definition cursor envelope is malformed"))?;
    let bytes = hex_decode(encoded)?;
    if bytes.len() > 2048 {
        return Err(bound(
            "definition cursor payload is outside the admitted bound",
        ));
    }
    let cursor: DefinitionCursorV1 =
        parse_canonical(&bytes).map_err(|_| substituted("definition cursor is malformed"))?;
    if cursor.schema_ref != "handbook.contract-definition-list-cursor@1.0.0"
        || cursor.root != *root
        || cursor.page_size != page_size
    {
        return Err(substituted("definition cursor is substituted"));
    }
    let expected = canonical_fingerprint(&DefinitionCursorPreimage {
        schema_ref: "handbook.contract-definition-list-cursor@1.0.0",
        domain: "contract-definition-list-cursor",
        root: &cursor.root,
        offset: cursor.offset,
        page_size: cursor.page_size,
    })
    .map_err(|_| durability("definition cursor fingerprint could not be derived"))?;
    if cursor.cursor_fingerprint != expected {
        return Err(substituted("definition cursor fingerprint is substituted"));
    }
    Ok(cursor.offset)
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn hex_decode(value: &str) -> Result<Vec<u8>, ContractOwnerRepositoryErrorV1> {
    if value.len() % 2 != 0 || value.len() > 4096 || !value.is_ascii() {
        return Err(substituted("definition cursor encoding is malformed"));
    }
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = hex_nibble(pair[0])?;
            let low = hex_nibble(pair[1])?;
            Ok((high << 4) | low)
        })
        .collect()
}

fn hex_nibble(value: u8) -> Result<u8, ContractOwnerRepositoryErrorV1> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        _ => Err(substituted("definition cursor encoding is malformed")),
    }
}

fn file_name(path: &Path) -> Result<String, ContractOwnerRepositoryErrorV1> {
    path.file_name()
        .and_then(|v| v.to_str())
        .map(str::to_owned)
        .ok_or_else(|| unsafe_fs("filename unavailable"))
}
fn file_stem(path: &Path) -> Result<String, ContractOwnerRepositoryErrorV1> {
    path.file_stem()
        .and_then(|v| v.to_str())
        .map(str::to_owned)
        .ok_or_else(|| unsafe_fs("file stem unavailable"))
}
fn fingerprint_hex(value: &FingerprintV1) -> &str {
    value
        .as_str()
        .strip_prefix("sha256:")
        .expect("fingerprint is normalized")
}
fn require_policy_kind(
    registry: &EvaluationPolicyRegistryV1,
    policies: &[ExactBindingV1],
    expected: EvaluationPolicyKindV1,
) -> Result<(), ContractOwnerRepositoryErrorV1> {
    if policies.is_empty() {
        return Err(invalid("evaluation policy closure is empty"));
    }
    for policy in policies {
        let registered = registry.resolve_exact(policy).map_err(map_contract_input)?;
        if registered.kind() != expected {
            return Err(invalid("evaluation policy kind is not admitted"));
        }
    }
    Ok(())
}
fn map_contract_input(_: crate::ContractError) -> ContractOwnerRepositoryErrorV1 {
    invalid("evaluation run input or policy is not exactly admitted")
}
fn map_lifecycle(_: ContractLifecycleErrorV1) -> ContractOwnerRepositoryErrorV1 {
    repository_error(
        ContractOwnerRepositoryErrorKindV1::LifecycleRefused,
        "lifecycle admission or chain refused",
    )
}
fn map_durability(
    error: handbook_engine::RepositoryDurabilityErrorV1,
) -> ContractOwnerRepositoryErrorV1 {
    match error.kind() {
        RepositoryDurabilityErrorKindV1::InvalidRelativePath => repository_error(
            ContractOwnerRepositoryErrorKindV1::InvalidRequest,
            "repository path is not admitted",
        ),
        RepositoryDurabilityErrorKindV1::UnsafeFilesystem => {
            unsafe_fs("retained repository filesystem is unsafe")
        }
        RepositoryDurabilityErrorKindV1::BoundExceeded => {
            bound("retained repository operation exceeded its bound")
        }
        RepositoryDurabilityErrorKindV1::AlreadyExists
        | RepositoryDurabilityErrorKindV1::NotFound
        | RepositoryDurabilityErrorKindV1::CurrentValueMismatch => {
            recovery("retained repository state is ambiguous")
        }
        RepositoryDurabilityErrorKindV1::LockUnavailable
        | RepositoryDurabilityErrorKindV1::DurabilityUncertain
        | RepositoryDurabilityErrorKindV1::UnsupportedPlatform => {
            durability("retained repository durability is uncertain")
        }
    }
}
const fn repository_error(
    kind: ContractOwnerRepositoryErrorKindV1,
    detail: &'static str,
) -> ContractOwnerRepositoryErrorV1 {
    ContractOwnerRepositoryErrorV1 { kind, detail }
}
const fn invalid(detail: &'static str) -> ContractOwnerRepositoryErrorV1 {
    repository_error(ContractOwnerRepositoryErrorKindV1::InvalidRequest, detail)
}
const fn durability(detail: &'static str) -> ContractOwnerRepositoryErrorV1 {
    repository_error(
        ContractOwnerRepositoryErrorKindV1::DurabilityUncertain,
        detail,
    )
}
const fn recovery(detail: &'static str) -> ContractOwnerRepositoryErrorV1 {
    repository_error(
        ContractOwnerRepositoryErrorKindV1::RecoveryAmbiguous,
        detail,
    )
}
const fn substituted(detail: &'static str) -> ContractOwnerRepositoryErrorV1 {
    repository_error(
        ContractOwnerRepositoryErrorKindV1::RecordSubstituted,
        detail,
    )
}
const fn unsafe_fs(detail: &'static str) -> ContractOwnerRepositoryErrorV1 {
    repository_error(ContractOwnerRepositoryErrorKindV1::UnsafeFilesystem, detail)
}
const fn bound(detail: &'static str) -> ContractOwnerRepositoryErrorV1 {
    repository_error(ContractOwnerRepositoryErrorKindV1::BoundExceeded, detail)
}
