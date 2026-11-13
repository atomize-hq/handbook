use super::{
    BoundedString, BoundedVec, ContractError, Fingerprint, SafeU64, SessionGroundingBoundaryKindV1,
    SessionGroundingBoundaryRequestV1,
};
use handbook_engine::{
    SnapshotBoundaryKindV1, SnapshotCaptureErrorV1, SnapshotCaptureOutcomeV1,
    SnapshotCaptureRequestV1, SnapshotCommittedRecordV1, SnapshotGroundingErrorClassV1,
    SnapshotGroundingErrorKindV1, SnapshotGroundingKernelErrorKindV1,
    SnapshotGroundingKernelErrorV1, SnapshotGroundingMutationEffectV1, SnapshotGroundingOwnerV1,
    SnapshotSourcePortErrorV1, SnapshotSourcePortV1,
};
use handbook_flow::{adopt_session_grounding_outcome, SessionGroundingFlowStatusV1};
use handbook_pipeline::{include_session_grounding, SessionGroundingPipelineInclusionV1};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::Path;

pub(crate) const SESSION_GROUNDING_BOUNDARY_USE_CASE_REF: &str =
    "handbook.use-case.session-grounding-boundary@1.0.0";
pub(crate) const SESSION_GROUNDING_BOUNDARY_RESULT_REF: &str =
    "handbook.use-case.session-grounding-boundary-result@1.0.0";
pub(crate) const SESSION_GROUNDING_BOUNDARY_DETAILS_REF: &str =
    "handbook.use-case.session-grounding-boundary-details@1.0.0";
pub(crate) const SESSION_GROUNDING_BOUNDARY_RULE_REF: &str =
    "handbook.use-case.session-grounding-boundary-rule@1.0.0";
pub(crate) const SESSION_GROUNDING_RECEIPT_RECORD_REF: &str =
    "handbook.snapshot-grounding-owner-receipt@1.0.0";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionGroundingTerminalV1 {
    Started,
    Finalized,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "outcome")]
pub enum SessionGroundingPredecessorV1 {
    NoApplicablePrior,
    ApplicablePrior { boundary_ref: BoundedString<1024> },
    OpenStart { boundary_ref: BoundedString<1024> },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionGroundingRecoveryActionV1 {
    None,
    DeliveryRecoveredByExactReplay,
    RecomposeCommittedOwnerResultOrReplayExactRequest,
    RecheckSameRequest,
    ChangeRequest,
    ContactOperator,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionGroundingRecordV1 {
    exact_ref: BoundedString<1024>,
    fingerprint: Fingerprint,
}

impl SessionGroundingRecordV1 {
    fn from_owner(value: &SnapshotCommittedRecordV1) -> Result<Self, ContractError> {
        Ok(Self {
            exact_ref: BoundedString::new(value.exact_ref().to_owned())?,
            fingerprint: Fingerprint::parse(value.fingerprint())?,
        })
    }

    pub fn exact_ref(&self) -> &str {
        self.exact_ref.as_str()
    }

    pub fn fingerprint(&self) -> &Fingerprint {
        &self.fingerprint
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionGroundingBoundaryResultV1 {
    terminal: SessionGroundingTerminalV1,
    sequence: SafeU64,
    predecessor: SessionGroundingPredecessorV1,
    boundary_record: SessionGroundingRecordV1,
    accounting_record: SessionGroundingRecordV1,
    delta_record: Option<SessionGroundingRecordV1>,
    grounding_record: SessionGroundingRecordV1,
    receipt_record: SessionGroundingRecordV1,
    replayed: bool,
    flow_status: SessionGroundingTerminalV1,
    pipeline_status: SessionGroundingTerminalV1,
    pipeline_namespace: BoundedString<255>,
    handoff_refs: BoundedVec<BoundedString<1024>, 128>,
    recovery_action: SessionGroundingRecoveryActionV1,
}

impl SessionGroundingBoundaryResultV1 {
    pub(crate) fn from_inclusion(
        inclusion: &SessionGroundingPipelineInclusionV1,
    ) -> Result<Self, ContractError> {
        let flow_status = flow_terminal(inclusion.status());
        let owner = inclusion.adoption().owner_outcome();
        let (
            terminal,
            sequence,
            predecessor,
            boundary_record,
            accounting_record,
            delta_record,
            grounding_record,
            receipt_record,
            replayed,
            handoff_refs,
        ) = match owner {
            SnapshotCaptureOutcomeV1::Started(value) => {
                let predecessor = if value.is_first_start() {
                    SessionGroundingPredecessorV1::NoApplicablePrior
                } else {
                    SessionGroundingPredecessorV1::ApplicablePrior {
                        boundary_ref: bounded_required(value.predecessor_boundary_ref())?,
                    }
                };
                (
                    SessionGroundingTerminalV1::Started,
                    value.sequence(),
                    predecessor,
                    value.boundary_record(),
                    value.accounting_record(),
                    value.delta_record(),
                    value.grounding_record(),
                    value.receipt_record(),
                    value.replayed(),
                    Vec::new(),
                )
            }
            SnapshotCaptureOutcomeV1::Finalized(value) => (
                SessionGroundingTerminalV1::Finalized,
                value.sequence(),
                SessionGroundingPredecessorV1::OpenStart {
                    boundary_ref: bounded_required(value.predecessor_boundary_ref())?,
                },
                value.boundary_record(),
                value.accounting_record(),
                value.delta_record(),
                value.grounding_record(),
                value.receipt_record(),
                value.replayed(),
                value.handoff_refs().to_vec(),
            ),
        };
        if terminal != flow_status || terminal != flow_terminal(inclusion.status()) {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Self {
            terminal,
            sequence: SafeU64::new(sequence)?,
            predecessor,
            boundary_record: SessionGroundingRecordV1::from_owner(boundary_record)?,
            accounting_record: SessionGroundingRecordV1::from_owner(accounting_record)?,
            delta_record: delta_record
                .map(SessionGroundingRecordV1::from_owner)
                .transpose()?,
            grounding_record: SessionGroundingRecordV1::from_owner(grounding_record)?,
            receipt_record: SessionGroundingRecordV1::from_owner(receipt_record)?,
            replayed,
            flow_status,
            pipeline_status: flow_terminal(inclusion.status()),
            pipeline_namespace: BoundedString::new(inclusion.namespace().to_owned())?,
            handoff_refs: BoundedVec::new(
                handoff_refs
                    .iter()
                    .map(|value| BoundedString::new(value.clone()))
                    .collect::<Result<Vec<_>, _>>()?,
            )?,
            recovery_action: if replayed {
                SessionGroundingRecoveryActionV1::DeliveryRecoveredByExactReplay
            } else {
                SessionGroundingRecoveryActionV1::None
            },
        })
    }

    pub fn terminal(&self) -> SessionGroundingTerminalV1 {
        self.terminal
    }

    pub fn sequence(&self) -> u64 {
        self.sequence.get()
    }

    pub fn predecessor(&self) -> &SessionGroundingPredecessorV1 {
        &self.predecessor
    }

    pub fn delta_record(&self) -> Option<&SessionGroundingRecordV1> {
        self.delta_record.as_ref()
    }

    pub fn boundary_record(&self) -> &SessionGroundingRecordV1 {
        &self.boundary_record
    }

    pub fn accounting_record(&self) -> &SessionGroundingRecordV1 {
        &self.accounting_record
    }

    pub fn grounding_record(&self) -> &SessionGroundingRecordV1 {
        &self.grounding_record
    }

    pub fn receipt_record(&self) -> &SessionGroundingRecordV1 {
        &self.receipt_record
    }

    pub fn replayed(&self) -> bool {
        self.replayed
    }

    pub fn flow_status(&self) -> SessionGroundingTerminalV1 {
        self.flow_status
    }

    pub fn pipeline_status(&self) -> SessionGroundingTerminalV1 {
        self.pipeline_status
    }

    pub fn handoff_refs(&self) -> &[BoundedString<1024>] {
        self.handoff_refs.as_slice()
    }

    pub fn recovery_action(&self) -> SessionGroundingRecoveryActionV1 {
        self.recovery_action
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionGroundingOwnerFailureV1 {
    error_class: BoundedString<64>,
    error_kind: BoundedString<128>,
    mutation_effect: BoundedString<64>,
    recovery_action: SessionGroundingRecoveryActionV1,
}

impl SessionGroundingOwnerFailureV1 {
    pub fn error_class(&self) -> &str {
        self.error_class.as_str()
    }

    pub fn error_kind(&self) -> &str {
        self.error_kind.as_str()
    }

    pub fn mutation_effect(&self) -> &str {
        self.mutation_effect.as_str()
    }

    pub fn recovery_action(&self) -> SessionGroundingRecoveryActionV1 {
        self.recovery_action
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SessionGroundingCompositionFailureV1 {
    receipt_record: SessionGroundingRecordV1,
    recovery_action: SessionGroundingRecoveryActionV1,
}

impl SessionGroundingCompositionFailureV1 {
    pub fn receipt_record(&self) -> &SessionGroundingRecordV1 {
        &self.receipt_record
    }

    pub fn recovery_action(&self) -> SessionGroundingRecoveryActionV1 {
        self.recovery_action
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SessionGroundingReceiptConditionV1 {
    committed: bool,
    owner_receipt: SessionGroundingRecordV1,
}

impl SessionGroundingReceiptConditionV1 {
    pub(crate) fn committed(owner_receipt: &SessionGroundingRecordV1) -> Self {
        Self {
            committed: true,
            owner_receipt: owner_receipt.clone(),
        }
    }

    pub(crate) fn admits(&self, expected: &SessionGroundingRecordV1) -> bool {
        self.committed && &self.owner_receipt == expected
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "failure")]
pub enum SessionGroundingBoundaryDetailsV1 {
    Owner(SessionGroundingOwnerFailureV1),
    CaptureCommittedCompositionFailed(SessionGroundingCompositionFailureV1),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SessionGroundingBoundaryUseCaseOutcomeV1 {
    Composed(Box<SessionGroundingBoundaryResultV1>),
    OwnerFailed(SessionGroundingOwnerFailureV1),
    CaptureCommittedCompositionFailed(SessionGroundingCompositionFailureV1),
}

pub(crate) enum SessionGroundingBoundaryExecutionV1 {
    Composed(SessionGroundingPipelineInclusionV1),
    OwnerFailed(SessionGroundingOwnerFailureV1),
    CaptureCommittedCompositionFailed {
        committed: SnapshotCaptureOutcomeV1,
        details: SessionGroundingCompositionFailureV1,
    },
}

trait SessionGroundingComposerV1 {
    fn compose(
        &mut self,
        committed: SnapshotCaptureOutcomeV1,
    ) -> Result<SessionGroundingPipelineInclusionV1, Box<SnapshotCaptureOutcomeV1>>;
}

struct FlowPipelineComposerV1;

impl SessionGroundingComposerV1 for FlowPipelineComposerV1 {
    fn compose(
        &mut self,
        committed: SnapshotCaptureOutcomeV1,
    ) -> Result<SessionGroundingPipelineInclusionV1, Box<SnapshotCaptureOutcomeV1>> {
        Ok(include_session_grounding(adopt_session_grounding_outcome(
            committed,
        )))
    }
}

pub(crate) fn execute_session_grounding_boundary<P: SnapshotSourcePortV1>(
    repo_root: &Path,
    request: &SessionGroundingBoundaryRequestV1,
    port: &mut P,
) -> Result<SessionGroundingBoundaryExecutionV1, ContractError> {
    execute_session_grounding_boundary_with_composer(
        repo_root,
        request,
        port,
        &mut FlowPipelineComposerV1,
    )
}

pub(crate) fn execute_session_grounding_boundary_dyn(
    repo_root: &Path,
    request: &SessionGroundingBoundaryRequestV1,
    port: &mut dyn SnapshotSourcePortV1,
) -> Result<SessionGroundingBoundaryExecutionV1, ContractError> {
    let mut forwarding = DynSourcePortV1 { inner: port };
    execute_session_grounding_boundary(repo_root, request, &mut forwarding)
}

pub(crate) fn recompose_session_grounding_boundary(
    committed: SnapshotCaptureOutcomeV1,
) -> Result<SessionGroundingBoundaryExecutionV1, ContractError> {
    compose_committed(committed, &mut FlowPipelineComposerV1)
}

fn execute_session_grounding_boundary_with_composer<
    P: SnapshotSourcePortV1,
    C: SessionGroundingComposerV1,
>(
    repo_root: &Path,
    request: &SessionGroundingBoundaryRequestV1,
    port: &mut P,
    composer: &mut C,
) -> Result<SessionGroundingBoundaryExecutionV1, ContractError> {
    let owner = SnapshotGroundingOwnerV1::new();
    let binding = match owner.bind_repository(
        repo_root,
        request.workspace().reference().as_str(),
        request.workspace().fingerprint().as_str(),
    ) {
        Ok(value) => value,
        Err(error) => {
            return Ok(SessionGroundingBoundaryExecutionV1::OwnerFailed(
                kernel_failure(&error)?,
            ))
        }
    };
    let stream = owner.bind_session_stream(&binding);
    let capture_request = match SnapshotCaptureRequestV1::new(
        request.idempotency_key().as_bytes(),
        request.host_session_ref(),
        match request.boundary() {
            SessionGroundingBoundaryKindV1::Start => SnapshotBoundaryKindV1::Start,
            SessionGroundingBoundaryKindV1::End => SnapshotBoundaryKindV1::End,
        },
    ) {
        Ok(value) => value,
        Err(error) => {
            return Ok(SessionGroundingBoundaryExecutionV1::OwnerFailed(
                capture_failure(&error)?,
            ))
        }
    };
    match owner.capture_boundary(&binding, &stream, &capture_request, port) {
        Ok(committed) => compose_committed(committed, composer),
        Err(error) => Ok(SessionGroundingBoundaryExecutionV1::OwnerFailed(
            capture_failure(&error)?,
        )),
    }
}

fn compose_committed<C: SessionGroundingComposerV1>(
    committed: SnapshotCaptureOutcomeV1,
    composer: &mut C,
) -> Result<SessionGroundingBoundaryExecutionV1, ContractError> {
    match composer.compose(committed) {
        Ok(inclusion) => Ok(SessionGroundingBoundaryExecutionV1::Composed(inclusion)),
        Err(committed) => {
            let committed = *committed;
            let receipt = receipt_record(&committed);
            Ok(
                SessionGroundingBoundaryExecutionV1::CaptureCommittedCompositionFailed {
                    details: SessionGroundingCompositionFailureV1 {
                        receipt_record: SessionGroundingRecordV1::from_owner(receipt)?,
                        recovery_action: SessionGroundingRecoveryActionV1::RecomposeCommittedOwnerResultOrReplayExactRequest,
                    },
                    committed,
                },
            )
        }
    }
}

fn capture_failure(
    error: &SnapshotCaptureErrorV1,
) -> Result<SessionGroundingOwnerFailureV1, ContractError> {
    let class = error.kind().class();
    Ok(SessionGroundingOwnerFailureV1 {
        error_class: BoundedString::new(error_class_name(class).to_owned())?,
        error_kind: BoundedString::new(error_kind_name(error.kind()).to_owned())?,
        mutation_effect: BoundedString::new(
            mutation_effect_name(error.kind().mutation_effect()).to_owned(),
        )?,
        recovery_action: recovery_for_class(class),
    })
}

fn kernel_failure(
    error: &SnapshotGroundingKernelErrorV1,
) -> Result<SessionGroundingOwnerFailureV1, ContractError> {
    Ok(SessionGroundingOwnerFailureV1 {
        error_class: BoundedString::new("refusal".to_owned())?,
        error_kind: BoundedString::new(kernel_error_kind_name(error.kind()).to_owned())?,
        mutation_effect: BoundedString::new("write_nothing".to_owned())?,
        recovery_action: SessionGroundingRecoveryActionV1::ChangeRequest,
    })
}

fn kernel_error_kind_name(kind: SnapshotGroundingKernelErrorKindV1) -> &'static str {
    match kind {
        SnapshotGroundingKernelErrorKindV1::InvalidProductionSource => "invalid_production_source",
        SnapshotGroundingKernelErrorKindV1::InvalidExactBinding => "invalid_exact_binding",
        SnapshotGroundingKernelErrorKindV1::InvalidRepositoryRoot => "invalid_repository_root",
        SnapshotGroundingKernelErrorKindV1::InvalidSessionRef => "invalid_session_ref",
        SnapshotGroundingKernelErrorKindV1::InvalidSequence => "invalid_sequence",
        SnapshotGroundingKernelErrorKindV1::RepositoryMismatch => "repository_mismatch",
    }
}

fn error_class_name(class: SnapshotGroundingErrorClassV1) -> &'static str {
    match class {
        SnapshotGroundingErrorClassV1::Refusal => "refusal",
        SnapshotGroundingErrorClassV1::Conflict => "conflict",
        SnapshotGroundingErrorClassV1::Unavailable => "unavailable",
        SnapshotGroundingErrorClassV1::Error => "error",
        SnapshotGroundingErrorClassV1::PostCommit => "post_commit",
    }
}

fn error_kind_name(kind: SnapshotGroundingErrorKindV1) -> &'static str {
    match kind {
        SnapshotGroundingErrorKindV1::MalformedInput => "malformed_input",
        SnapshotGroundingErrorKindV1::InputLimitExceeded => "input_limit_exceeded",
        SnapshotGroundingErrorKindV1::UnknownReference => "unknown_reference",
        SnapshotGroundingErrorKindV1::SubstitutedReference => "substituted_reference",
        SnapshotGroundingErrorKindV1::RepositoryMismatch => "repository_mismatch",
        SnapshotGroundingErrorKindV1::PolicyMismatch => "policy_mismatch",
        SnapshotGroundingErrorKindV1::UnknownFamily => "unknown_family",
        SnapshotGroundingErrorKindV1::UnknownRule => "unknown_rule",
        SnapshotGroundingErrorKindV1::AdapterMismatch => "adapter_mismatch",
        SnapshotGroundingErrorKindV1::MatcherFailure => "matcher_failure",
        SnapshotGroundingErrorKindV1::SourceDrift => "source_drift",
        SnapshotGroundingErrorKindV1::IncompatiblePredecessor => "incompatible_predecessor",
        SnapshotGroundingErrorKindV1::SequenceConflict => "sequence_conflict",
        SnapshotGroundingErrorKindV1::IncompleteAccounting => "incomplete_accounting",
        SnapshotGroundingErrorKindV1::InsufficientResolution => "insufficient_resolution",
        SnapshotGroundingErrorKindV1::RedactionUncertain => "redaction_uncertain",
        SnapshotGroundingErrorKindV1::RetentionConflict => "retention_conflict",
        SnapshotGroundingErrorKindV1::CorruptStore => "corrupt_store",
        SnapshotGroundingErrorKindV1::IdempotencyConflict => "idempotency_conflict",
        SnapshotGroundingErrorKindV1::RequiredSourceUnavailable => "required_source_unavailable",
        SnapshotGroundingErrorKindV1::LockUnavailable => "lock_unavailable",
        SnapshotGroundingErrorKindV1::RecoveryFailed => "recovery_failed",
        SnapshotGroundingErrorKindV1::DurabilityUncertain => "durability_uncertain",
        SnapshotGroundingErrorKindV1::CompositionFailedAfterCommit => {
            "composition_failed_after_commit"
        }
        SnapshotGroundingErrorKindV1::DeliveryUncertainAfterCommit => {
            "delivery_uncertain_after_commit"
        }
    }
}

fn mutation_effect_name(effect: SnapshotGroundingMutationEffectV1) -> &'static str {
    match effect {
        SnapshotGroundingMutationEffectV1::WriteNothing => "write_nothing",
        SnapshotGroundingMutationEffectV1::CommittedReceiptPreserved => {
            "committed_receipt_preserved"
        }
    }
}

fn recovery_for_class(class: SnapshotGroundingErrorClassV1) -> SessionGroundingRecoveryActionV1 {
    match class {
        SnapshotGroundingErrorClassV1::Refusal | SnapshotGroundingErrorClassV1::Conflict => {
            SessionGroundingRecoveryActionV1::ChangeRequest
        }
        SnapshotGroundingErrorClassV1::Unavailable => {
            SessionGroundingRecoveryActionV1::RecheckSameRequest
        }
        SnapshotGroundingErrorClassV1::Error | SnapshotGroundingErrorClassV1::PostCommit => {
            SessionGroundingRecoveryActionV1::ContactOperator
        }
    }
}

fn flow_terminal(value: SessionGroundingFlowStatusV1) -> SessionGroundingTerminalV1 {
    match value {
        SessionGroundingFlowStatusV1::Started => SessionGroundingTerminalV1::Started,
        SessionGroundingFlowStatusV1::Finalized => SessionGroundingTerminalV1::Finalized,
    }
}

fn receipt_record(value: &SnapshotCaptureOutcomeV1) -> &SnapshotCommittedRecordV1 {
    match value {
        SnapshotCaptureOutcomeV1::Started(value) => value.receipt_record(),
        SnapshotCaptureOutcomeV1::Finalized(value) => value.receipt_record(),
    }
}

fn bounded_required(value: Option<&str>) -> Result<BoundedString<1024>, ContractError> {
    BoundedString::new(value.ok_or(ContractError::InvalidInvariant)?.to_owned())
}

struct DynSourcePortV1<'a> {
    inner: &'a mut dyn SnapshotSourcePortV1,
}

impl SnapshotSourcePortV1 for DynSourcePortV1<'_> {
    fn observe_slot(
        &mut self,
        family: handbook_engine::SnapshotSourceFamilyV1,
        slot: &'static str,
    ) -> Result<handbook_engine::SnapshotSlotObservationV1, SnapshotSourcePortErrorV1> {
        self.inner.observe_slot(family, slot)
    }

    fn read_bounded_slot(
        &mut self,
        family: handbook_engine::SnapshotSourceFamilyV1,
        slot: &'static str,
    ) -> Result<Value, SnapshotSourcePortErrorV1> {
        self.inner.read_bounded_slot(family, slot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract_membrane::{ContractLimits, UseCaseResponseV1};
    use handbook_engine::{
        DefinitionFingerprint, SnapshotGroundingOwnerV1, SnapshotSlotObservationV1,
        SnapshotSourceFamilyV1,
    };
    use serde_json::json;
    use std::collections::BTreeMap;
    use tempfile::tempdir;

    struct FailingComposer;

    impl SessionGroundingComposerV1 for FailingComposer {
        fn compose(
            &mut self,
            committed: SnapshotCaptureOutcomeV1,
        ) -> Result<SessionGroundingPipelineInclusionV1, Box<SnapshotCaptureOutcomeV1>> {
            Err(Box::new(committed))
        }
    }

    #[test]
    fn committed_composition_failure_preserves_receipt_and_recomposes_without_recapture() {
        let repo = tempdir().unwrap();
        let request = SessionGroundingBoundaryRequestV1::new(
            super::super::ExactBinding::new(
                super::super::ExactRef::parse("handbook.workspace.composition-test@1.0.0").unwrap(),
                super::super::Fingerprint::parse(
                    "sha256:4444444444444444444444444444444444444444444444444444444444444444",
                )
                .unwrap(),
            ),
            "composition-test-session",
            SessionGroundingBoundaryKindV1::Start,
            "composition-test-key",
        )
        .unwrap();
        let mut port = CompletePort::new();
        let failed = execute_session_grounding_boundary_with_composer(
            repo.path(),
            &request,
            &mut port,
            &mut FailingComposer,
        )
        .unwrap();
        let observed_after_commit = port.observed;
        let SessionGroundingBoundaryExecutionV1::CaptureCommittedCompositionFailed {
            committed,
            details,
        } = failed
        else {
            panic!("forced downstream composition must fail after the owner commit")
        };
        assert_eq!(
            details.recovery_action(),
            SessionGroundingRecoveryActionV1::RecomposeCommittedOwnerResultOrReplayExactRequest
        );
        assert_eq!(
            details.receipt_record().exact_ref(),
            receipt_record(&committed).exact_ref()
        );
        assert_eq!(
            details.receipt_record().fingerprint().as_str(),
            receipt_record(&committed).fingerprint()
        );

        let failure_response = UseCaseResponseV1::session_grounding_boundary_execution(
            SessionGroundingBoundaryExecutionV1::CaptureCommittedCompositionFailed {
                committed: committed.clone(),
                details: details.clone(),
            },
            &ContractLimits::packet0(),
        )
        .unwrap();
        assert_eq!(
            failure_response
                .session_grounding_boundary_outcome(&ContractLimits::packet0())
                .unwrap(),
            Some(
                SessionGroundingBoundaryUseCaseOutcomeV1::CaptureCommittedCompositionFailed(
                    details
                )
            )
        );

        let recomposed = recompose_session_grounding_boundary(committed).unwrap();
        assert!(matches!(
            recomposed,
            SessionGroundingBoundaryExecutionV1::Composed(_)
        ));
        assert_eq!(port.observed, observed_after_commit);
    }

    struct CompletePort {
        observations: BTreeMap<(SnapshotSourceFamilyV1, &'static str), SnapshotSlotObservationV1>,
        values: BTreeMap<(SnapshotSourceFamilyV1, &'static str), Value>,
        observed: usize,
    }

    impl CompletePort {
        fn new() -> Self {
            let owner = SnapshotGroundingOwnerV1::new();
            let mut observations = BTreeMap::new();
            let mut values = BTreeMap::new();
            for contract in owner.source_contracts() {
                for &slot in contract.source_slots() {
                    let reference = format!(
                        "handbook.snapshot-observation.{}.{}@1.0.0",
                        contract.family().as_str(),
                        slot.replace('_', "-")
                    );
                    let value = json!({"ref": reference});
                    observations.insert(
                        (contract.family(), slot),
                        SnapshotSlotObservationV1::retained(
                            &reference,
                            &DefinitionFingerprint::from_json_value(&value)
                                .unwrap()
                                .to_string(),
                        )
                        .unwrap(),
                    );
                    values.insert((contract.family(), slot), value);
                }
            }
            observations.insert(
                (SnapshotSourceFamilyV1::Git, "worktree"),
                SnapshotSlotObservationV1::redacted(
                    "handbook.snapshot-observation.git.worktree@1.0.0",
                    &DefinitionFingerprint::from_json_value(&json!({"redacted": true}))
                        .unwrap()
                        .to_string(),
                )
                .unwrap(),
            );
            observations.insert(
                (SnapshotSourceFamilyV1::Evidence, "dock_execution_candidate"),
                SnapshotSlotObservationV1::omitted("absent_or_inapplicable"),
            );
            observations.insert(
                (SnapshotSourceFamilyV1::Work, "handoff"),
                SnapshotSlotObservationV1::inapplicable("policy_inapplicable"),
            );
            let decision = values
                .get(&(SnapshotSourceFamilyV1::Evidence, "contracts_decision"))
                .unwrap()
                .clone();
            observations.insert(
                (SnapshotSourceFamilyV1::Evidence, "contracts_decision"),
                SnapshotSlotObservationV1::derived(
                    "handbook.snapshot-observation.evidence.contracts-decision@1.0.0",
                    &DefinitionFingerprint::from_json_value(&decision)
                        .unwrap()
                        .to_string(),
                )
                .unwrap(),
            );
            let resolution = json!({
                "resolution": {
                    "stack_fingerprint": "sha256:4444444444444444444444444444444444444444444444444444444444444444",
                    "stack_ref": "handbook.context-resolution-stack.composition-test@1.0.0",
                    "values": ["task", "repository", "session", "bounded", "execution", "verified"]
                }
            });
            observations.insert(
                (
                    SnapshotSourceFamilyV1::Handbook,
                    "schema_artifact_resolution",
                ),
                SnapshotSlotObservationV1::retained(
                    "handbook.snapshot-observation.handbook.schema-artifact-resolution@1.0.0",
                    &DefinitionFingerprint::from_json_value(&resolution)
                        .unwrap()
                        .to_string(),
                )
                .unwrap(),
            );
            values.insert(
                (
                    SnapshotSourceFamilyV1::Handbook,
                    "schema_artifact_resolution",
                ),
                resolution,
            );
            Self {
                observations,
                values,
                observed: 0,
            }
        }
    }

    impl SnapshotSourcePortV1 for CompletePort {
        fn observe_slot(
            &mut self,
            family: SnapshotSourceFamilyV1,
            slot: &'static str,
        ) -> Result<SnapshotSlotObservationV1, SnapshotSourcePortErrorV1> {
            self.observed += 1;
            self.observations
                .get(&(family, slot))
                .cloned()
                .ok_or_else(SnapshotSourcePortErrorV1::missing)
        }

        fn read_bounded_slot(
            &mut self,
            family: SnapshotSourceFamilyV1,
            slot: &'static str,
        ) -> Result<Value, SnapshotSourcePortErrorV1> {
            self.values
                .get(&(family, slot))
                .cloned()
                .ok_or_else(SnapshotSourcePortErrorV1::missing)
        }
    }
}
