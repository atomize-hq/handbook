use super::{
    canonical_json_bytes, parse_canonical_json, BoundedString, BoundedVec, CapabilityEntry,
    ContractError, ContractLimits, ContractValue, CorrelationRef, CurrentTransportEvidence,
    CurrentTransportEvidenceSealed, ExactBinding, ExactRef, IdempotencyState,
    OperationAdmissionResult, OperationClass, OperationDefinition, OperationIdempotency,
    OperationOutcomeSchemas, OperationOwnerBinding, OperationRequestV1, OperationResponseV1,
    OperationSchemaValidator, OperationTransport, OwnerAdmissionToken, Problem, ProblemCategory,
    RecheckCondition, RequestId, RetryDirective, SemVer, TerminalFields, TerminalStatus,
};
use crate::{
    ArtifactPresence, Blocker, BlockerCategory, BudgetByteDomain, BudgetDisposition,
    BudgetNextSafeAction, BudgetOutcome, BudgetReason, BudgetTarget, DecisionLog, NextSafeAction,
    PacketBodyNote, PacketBodyNoteKind, PacketDecisionSummary, PacketFixtureContext, PacketResult,
    PacketSection, PacketSectionMode, PacketSelection, PacketSelectionStatus, PacketSourceSummary,
    PacketVariant, ReadyPacketNextSafeAction, Refusal, RefusalCategory, SubjectRef,
};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub(crate) const FLOW_OPERATION_REF: &str = "flow.resolve@1.0.0";
pub(crate) const FLOW_REQUEST_SCHEMA_REF: &str = "handbook.operation.flow-resolve-request@1.0.0";
pub(crate) const FLOW_RESULT_SCHEMA_REF: &str = "handbook.operation.flow-resolve-result@1.0.0";
pub(crate) const FLOW_DETAILS_SCHEMA_REF: &str = "handbook.flow-operation-details@1.0.0";
pub(crate) const FLOW_RECHECK_SCHEMA_REF: &str = "handbook.flow-recheck@1.0.0";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FlowPacketIdV1 {
    #[serde(rename = "planning.packet")]
    Planning,
    #[serde(rename = "execution.demo.packet")]
    ExecutionDemo,
    #[serde(rename = "execution.live.packet")]
    ExecutionLive,
}

impl FlowPacketIdV1 {
    fn parse(value: &str) -> Result<Self, ContractError> {
        match value {
            "planning.packet" => Ok(Self::Planning),
            "execution.demo.packet" => Ok(Self::ExecutionDemo),
            "execution.live.packet" => Ok(Self::ExecutionLive),
            _ => Err(ContractError::InvalidInvariant),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Planning => "planning.packet",
            Self::ExecutionDemo => "execution.demo.packet",
            Self::ExecutionLive => "execution.live.packet",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FlowResolveRequestV1 {
    packet_id: FlowPacketIdV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FlowSelectionStatusV1 {
    Selected,
    Blocked,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FlowArtifactPresenceV1 {
    Missing,
    PresentEmpty,
    PresentNonEmpty,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FlowPacketVariantV1 {
    Planning,
    ExecutionDemo,
    ExecutionLive,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FlowPacketBodyNoteKindV1 {
    Omission,
    Budget,
    InheritedDependency,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FlowPacketSectionModeV1 {
    Verbatim,
    Summary,
    Rendered,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FlowReadyActionV1 {
    InspectProof,
    Generate,
    RunDoctor,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FlowBudgetDispositionV1 {
    Keep,
    Summarize,
    Exclude,
    Refuse,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FlowBudgetReasonV1 {
    WithinBudget,
    OptionalArtifactTooLarge,
    TotalBytesExceeded,
    RequiredArtifactTooLarge,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FlowBudgetByteDomainV1 {
    Source,
    RenderedOutput,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FlowRefusalCategoryV1 {
    NonCanonicalInputAttempt,
    SystemRootMissing,
    SystemRootNotDir,
    SystemRootSymlinkNotAllowed,
    RequiredArtifactMissing,
    RequiredArtifactEmpty,
    RequiredArtifactStarterTemplate,
    RequiredArtifactInvalid,
    ArtifactReadError,
    FreshnessInvalid,
    BudgetRefused,
    UnsupportedRequest,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FlowBlockerCategoryV1 {
    SystemRootMissing,
    SystemRootNotDir,
    SystemRootSymlinkNotAllowed,
    RequiredArtifactMissing,
    RequiredArtifactEmpty,
    RequiredArtifactStarterTemplate,
    RequiredArtifactInvalid,
    ArtifactReadError,
    FreshnessInvalid,
    BudgetRefused,
    UnsupportedRequest,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum FlowSubjectRefV1 {
    CanonicalArtifact {
        instance_id: String,
        kind_ref: String,
        label: String,
        canonical_repo_relative_path: String,
    },
    InheritedDependency {
        dependency_id: String,
        version: Option<String>,
    },
    Policy {
        policy_id: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum FlowNextSafeActionV1 {
    RunSetup,
    RunSetupInit,
    RunSetupRefresh,
    RunAuthorCharter,
    RunAuthorProjectContext,
    CreateSystemRoot {
        canonical_repo_relative_path: String,
    },
    EnsureSystemRootIsDirectory {
        canonical_repo_relative_path: String,
    },
    RemoveSystemRootSymlink {
        canonical_repo_relative_path: String,
    },
    CreateCanonicalArtifact {
        canonical_repo_relative_path: String,
    },
    FillCanonicalArtifact {
        canonical_repo_relative_path: String,
    },
    ReduceCanonicalArtifactSize {
        canonical_repo_relative_path: String,
    },
    RunGenerate {
        packet_id: String,
    },
    RunDoctor,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum FlowBudgetNextSafeActionV1 {
    ReduceCanonicalArtifactSize {
        canonical_repo_relative_path: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FlowPacketSourceV1 {
    instance_id: String,
    kind_ref: String,
    label: String,
    canonical_repo_relative_path: String,
    required: bool,
    presence: FlowArtifactPresenceV1,
    byte_len: Option<u64>,
    content_sha256: Option<String>,
    rendered_output_byte_len: Option<u64>,
    rendered_output_sha256: Option<String>,
    rendered_media_type: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FlowPacketBodyNoteV1 {
    kind: FlowPacketBodyNoteKindV1,
    text: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FlowPacketSectionV1 {
    instance_id: String,
    kind_ref: String,
    label: String,
    canonical_repo_relative_path: String,
    title: String,
    mode: FlowPacketSectionModeV1,
    contents: String,
    source_content_sha256: Option<String>,
    rendered_output_sha256: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FlowFixtureContextV1 {
    fixture_set_id: String,
    fixture_basis_root: String,
    fixture_lineage: Vec<FlowPacketSourceV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FlowDecisionSummaryV1 {
    packet_status: FlowSelectionStatusV1,
    budget_disposition: FlowBudgetDispositionV1,
    budget_reason: FlowBudgetReasonV1,
    decision_log_entries: u64,
    summary_line: String,
    ready_next_safe_action: FlowReadyActionV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FlowPacketResultV1 {
    packet_id: String,
    variant: FlowPacketVariantV1,
    fixture_context: Option<FlowFixtureContextV1>,
    included_sources: Vec<FlowPacketSourceV1>,
    notes: Vec<FlowPacketBodyNoteV1>,
    decision_summary: FlowDecisionSummaryV1,
    sections: Vec<FlowPacketSectionV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FlowBudgetTargetV1 {
    canonical_repo_relative_path: String,
    byte_len: u64,
    byte_domain: FlowBudgetByteDomainV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FlowBudgetOutcomeV1 {
    disposition: FlowBudgetDispositionV1,
    reason: FlowBudgetReasonV1,
    targets: Vec<FlowBudgetTargetV1>,
    next_safe_action: Option<FlowBudgetNextSafeActionV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FlowSelectionV1 {
    packet_id: String,
    status: FlowSelectionStatusV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FlowRefusalV1 {
    category: FlowRefusalCategoryV1,
    summary: String,
    broken_subject: FlowSubjectRefV1,
    next_safe_action: FlowNextSafeActionV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FlowBlockerV1 {
    category: FlowBlockerCategoryV1,
    subject: FlowSubjectRefV1,
    summary: String,
    next_safe_action: FlowNextSafeActionV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FlowResolveResultV1 {
    c04_result_version: String,
    c03_schema_version: String,
    c03_manifest_generation_version: u64,
    c03_fingerprint_sha256: String,
    packet_result: FlowPacketResultV1,
    decision_log_entries: Vec<String>,
    budget_outcome: FlowBudgetOutcomeV1,
    selection: FlowSelectionV1,
    refusal: Option<FlowRefusalV1>,
    blockers: Vec<FlowBlockerV1>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FlowProblemCauseV1 {
    NonCanonicalInputAttempt,
    SystemRootMissing,
    SystemRootNotDir,
    SystemRootSymlinkNotAllowed,
    RequiredArtifactMissing,
    RequiredArtifactEmpty,
    RequiredArtifactStarterTemplate,
    RequiredArtifactInvalid,
    ArtifactReadError,
    FreshnessInvalid,
    BudgetRefused,
    UnsupportedRequest,
    OwnerReadFailure,
    OwnerProfileDecision,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum FlowProblemOriginV1 {
    Resolution,
    OwnerIngest,
    OwnerError,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FlowProblemDetailsV1 {
    origin: FlowProblemOriginV1,
    cause: FlowProblemCauseV1,
    detail: String,
    resolution: Option<FlowResolveResultV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FlowRecheckV1 {
    cause: FlowProblemCauseV1,
}

impl FlowResolveResultV1 {
    fn from_resolution(value: crate::flow_api::PacketResolutionView) -> Self {
        Self {
            c04_result_version: value.c04_result_version,
            c03_schema_version: value.c03_schema_version,
            c03_manifest_generation_version: u64::from(value.c03_manifest_generation_version),
            c03_fingerprint_sha256: value.c03_fingerprint_sha256,
            packet_result: FlowPacketResultV1::from_packet(value.packet_result),
            decision_log_entries: value.decision_log.entries,
            budget_outcome: FlowBudgetOutcomeV1::from_budget(value.budget_outcome),
            selection: FlowSelectionV1 {
                packet_id: value.selection.packet_id,
                status: selection_status(value.selection.status),
            },
            refusal: value.refusal.map(FlowRefusalV1::from_refusal),
            blockers: value
                .blockers
                .into_iter()
                .map(FlowBlockerV1::from_blocker)
                .collect(),
        }
    }

    fn into_resolution(self) -> Result<crate::flow_api::PacketResolutionView, ContractError> {
        Ok(crate::flow_api::PacketResolutionView {
            c04_result_version: self.c04_result_version,
            c03_schema_version: self.c03_schema_version,
            c03_manifest_generation_version: self
                .c03_manifest_generation_version
                .try_into()
                .map_err(|_| ContractError::InvalidInvariant)?,
            c03_fingerprint_sha256: self.c03_fingerprint_sha256,
            packet_result: self.packet_result.into_packet()?,
            decision_log: DecisionLog {
                entries: self.decision_log_entries,
            },
            budget_outcome: self.budget_outcome.into_budget(),
            selection: PacketSelection {
                packet_id: self.selection.packet_id,
                status: selection_status_back(self.selection.status),
            },
            refusal: self.refusal.map(FlowRefusalV1::into_refusal).transpose()?,
            blockers: self
                .blockers
                .into_iter()
                .map(FlowBlockerV1::into_blocker)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn validate_semantics(&self) -> Result<(), ContractError> {
        let packet_id = FlowPacketIdV1::parse(&self.packet_result.packet_id)?;
        if self.selection.packet_id != self.packet_result.packet_id
            || packet_id != FlowPacketIdV1::parse(&self.selection.packet_id)?
            || !matches!(
                (packet_id, self.packet_result.variant),
                (FlowPacketIdV1::Planning, FlowPacketVariantV1::Planning)
                    | (
                        FlowPacketIdV1::ExecutionDemo,
                        FlowPacketVariantV1::ExecutionDemo
                    )
                    | (
                        FlowPacketIdV1::ExecutionLive,
                        FlowPacketVariantV1::ExecutionLive
                    )
            )
            || self.packet_result.decision_summary.packet_status != self.selection.status
            || self.packet_result.decision_summary.budget_disposition
                != self.budget_outcome.disposition
            || self.packet_result.decision_summary.budget_reason != self.budget_outcome.reason
            || usize::try_from(self.packet_result.decision_summary.decision_log_entries).ok()
                != Some(self.decision_log_entries.len())
            || self.decision_log_entries.is_empty()
            || self.c04_result_version != crate::flow_api::C04_RESULT_VERSION
        {
            return Err(ContractError::InvalidInvariant);
        }
        let ready = self.selection.status == FlowSelectionStatusV1::Selected;
        if ready != (self.refusal.is_none() && self.blockers.is_empty())
            || (ready && self.packet_result.sections.is_empty())
            || matches!(packet_id, FlowPacketIdV1::ExecutionDemo)
                != self.packet_result.fixture_context.is_some()
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(())
    }

    fn primary_problem_cause(&self) -> Result<FlowProblemCauseV1, ContractError> {
        if self.selection.status == FlowSelectionStatusV1::Selected {
            return Err(ContractError::InvalidInvariant);
        }
        if let Some(refusal) = self.refusal.as_ref() {
            return Ok(resolution_refusal_projection(refusal.category).0);
        }
        self.blockers
            .first()
            .map(|blocker| resolution_blocker_projection(blocker.category).0)
            .ok_or(ContractError::InvalidInvariant)
    }
}

impl FlowProblemDetailsV1 {
    fn validate_semantics(&self) -> Result<(), ContractError> {
        match self.origin {
            FlowProblemOriginV1::Resolution => {
                let resolution = self
                    .resolution
                    .as_ref()
                    .ok_or(ContractError::InvalidInvariant)?;
                resolution.validate_semantics()?;
                if resolution.primary_problem_cause()? != self.cause {
                    return Err(ContractError::InvalidInvariant);
                }
            }
            FlowProblemOriginV1::OwnerIngest => {
                if self.resolution.is_some() || !is_owner_ingest_cause(self.cause) {
                    return Err(ContractError::InvalidInvariant);
                }
            }
            FlowProblemOriginV1::OwnerError => {
                if self.resolution.is_some() || !is_owner_error_cause(self.cause) {
                    return Err(ContractError::InvalidInvariant);
                }
            }
        }
        Ok(())
    }
}

impl FlowPacketResultV1 {
    fn from_packet(value: PacketResult) -> Self {
        Self {
            packet_id: value.packet_id,
            variant: match value.variant {
                PacketVariant::Planning => FlowPacketVariantV1::Planning,
                PacketVariant::ExecutionDemo => FlowPacketVariantV1::ExecutionDemo,
                PacketVariant::ExecutionLive => FlowPacketVariantV1::ExecutionLive,
            },
            fixture_context: value.fixture_context.map(|context| FlowFixtureContextV1 {
                fixture_set_id: context.fixture_set_id,
                fixture_basis_root: context.fixture_basis_root,
                fixture_lineage: context
                    .fixture_lineage
                    .into_iter()
                    .map(FlowPacketSourceV1::from_source)
                    .collect(),
            }),
            included_sources: value
                .included_sources
                .into_iter()
                .map(FlowPacketSourceV1::from_source)
                .collect(),
            notes: value
                .notes
                .into_iter()
                .map(|note| FlowPacketBodyNoteV1 {
                    kind: match note.kind {
                        PacketBodyNoteKind::Omission => FlowPacketBodyNoteKindV1::Omission,
                        PacketBodyNoteKind::Budget => FlowPacketBodyNoteKindV1::Budget,
                        PacketBodyNoteKind::InheritedDependency => {
                            FlowPacketBodyNoteKindV1::InheritedDependency
                        }
                    },
                    text: note.text,
                })
                .collect(),
            decision_summary: FlowDecisionSummaryV1 {
                packet_status: selection_status(value.decision_summary.packet_status),
                budget_disposition: budget_disposition(value.decision_summary.budget_disposition),
                budget_reason: budget_reason(value.decision_summary.budget_reason),
                decision_log_entries: value.decision_summary.decision_log_entries as u64,
                summary_line: value.decision_summary.summary_line,
                ready_next_safe_action: match value.decision_summary.ready_next_safe_action {
                    ReadyPacketNextSafeAction::InspectProof => FlowReadyActionV1::InspectProof,
                    ReadyPacketNextSafeAction::Generate => FlowReadyActionV1::Generate,
                    ReadyPacketNextSafeAction::RunDoctor => FlowReadyActionV1::RunDoctor,
                },
            },
            sections: value
                .sections
                .into_iter()
                .map(|section| FlowPacketSectionV1 {
                    instance_id: section.instance_id,
                    kind_ref: section.kind_ref,
                    label: section.label,
                    canonical_repo_relative_path: section.canonical_repo_relative_path,
                    title: section.title,
                    mode: match section.mode {
                        PacketSectionMode::Verbatim => FlowPacketSectionModeV1::Verbatim,
                        PacketSectionMode::Summary => FlowPacketSectionModeV1::Summary,
                        PacketSectionMode::Rendered => FlowPacketSectionModeV1::Rendered,
                    },
                    contents: section.contents,
                    source_content_sha256: section.source_content_sha256,
                    rendered_output_sha256: section.rendered_output_sha256,
                })
                .collect(),
        }
    }

    fn into_packet(self) -> Result<PacketResult, ContractError> {
        Ok(PacketResult {
            packet_id: self.packet_id,
            variant: match self.variant {
                FlowPacketVariantV1::Planning => PacketVariant::Planning,
                FlowPacketVariantV1::ExecutionDemo => PacketVariant::ExecutionDemo,
                FlowPacketVariantV1::ExecutionLive => PacketVariant::ExecutionLive,
            },
            fixture_context: self.fixture_context.map(|context| PacketFixtureContext {
                fixture_set_id: context.fixture_set_id,
                fixture_basis_root: context.fixture_basis_root,
                fixture_lineage: context
                    .fixture_lineage
                    .into_iter()
                    .map(FlowPacketSourceV1::into_source)
                    .collect(),
            }),
            included_sources: self
                .included_sources
                .into_iter()
                .map(FlowPacketSourceV1::into_source)
                .collect(),
            notes: self
                .notes
                .into_iter()
                .map(|note| PacketBodyNote {
                    kind: match note.kind {
                        FlowPacketBodyNoteKindV1::Omission => PacketBodyNoteKind::Omission,
                        FlowPacketBodyNoteKindV1::Budget => PacketBodyNoteKind::Budget,
                        FlowPacketBodyNoteKindV1::InheritedDependency => {
                            PacketBodyNoteKind::InheritedDependency
                        }
                    },
                    text: note.text,
                })
                .collect(),
            decision_summary: PacketDecisionSummary {
                packet_status: selection_status_back(self.decision_summary.packet_status),
                budget_disposition: budget_disposition_back(
                    self.decision_summary.budget_disposition,
                ),
                budget_reason: budget_reason_back(self.decision_summary.budget_reason),
                decision_log_entries: self
                    .decision_summary
                    .decision_log_entries
                    .try_into()
                    .map_err(|_| ContractError::InvalidInvariant)?,
                summary_line: self.decision_summary.summary_line,
                ready_next_safe_action: match self.decision_summary.ready_next_safe_action {
                    FlowReadyActionV1::InspectProof => ReadyPacketNextSafeAction::InspectProof,
                    FlowReadyActionV1::Generate => ReadyPacketNextSafeAction::Generate,
                    FlowReadyActionV1::RunDoctor => ReadyPacketNextSafeAction::RunDoctor,
                },
            },
            sections: self
                .sections
                .into_iter()
                .map(|section| PacketSection {
                    instance_id: section.instance_id,
                    kind_ref: section.kind_ref,
                    label: section.label,
                    canonical_repo_relative_path: section.canonical_repo_relative_path,
                    title: section.title,
                    mode: match section.mode {
                        FlowPacketSectionModeV1::Verbatim => PacketSectionMode::Verbatim,
                        FlowPacketSectionModeV1::Summary => PacketSectionMode::Summary,
                        FlowPacketSectionModeV1::Rendered => PacketSectionMode::Rendered,
                    },
                    contents: section.contents,
                    source_content_sha256: section.source_content_sha256,
                    rendered_output_sha256: section.rendered_output_sha256,
                })
                .collect(),
        })
    }
}

impl FlowPacketSourceV1 {
    fn from_source(value: PacketSourceSummary) -> Self {
        Self {
            instance_id: value.instance_id,
            kind_ref: value.kind_ref,
            label: value.label,
            canonical_repo_relative_path: value.canonical_repo_relative_path,
            required: value.required,
            presence: match value.presence {
                ArtifactPresence::Missing => FlowArtifactPresenceV1::Missing,
                ArtifactPresence::PresentEmpty => FlowArtifactPresenceV1::PresentEmpty,
                ArtifactPresence::PresentNonEmpty => FlowArtifactPresenceV1::PresentNonEmpty,
            },
            byte_len: value.byte_len,
            content_sha256: value.content_sha256,
            rendered_output_byte_len: value.rendered_output_byte_len,
            rendered_output_sha256: value.rendered_output_sha256,
            rendered_media_type: value.rendered_media_type,
        }
    }

    fn into_source(self) -> PacketSourceSummary {
        PacketSourceSummary {
            instance_id: self.instance_id,
            kind_ref: self.kind_ref,
            label: self.label,
            canonical_repo_relative_path: self.canonical_repo_relative_path,
            required: self.required,
            presence: match self.presence {
                FlowArtifactPresenceV1::Missing => ArtifactPresence::Missing,
                FlowArtifactPresenceV1::PresentEmpty => ArtifactPresence::PresentEmpty,
                FlowArtifactPresenceV1::PresentNonEmpty => ArtifactPresence::PresentNonEmpty,
            },
            byte_len: self.byte_len,
            content_sha256: self.content_sha256,
            rendered_output_byte_len: self.rendered_output_byte_len,
            rendered_output_sha256: self.rendered_output_sha256,
            rendered_media_type: self.rendered_media_type,
        }
    }
}

impl FlowBudgetOutcomeV1 {
    fn from_budget(value: BudgetOutcome) -> Self {
        Self {
            disposition: budget_disposition(value.disposition),
            reason: budget_reason(value.reason),
            targets: value
                .targets
                .into_iter()
                .map(|target| FlowBudgetTargetV1 {
                    canonical_repo_relative_path: target.canonical_repo_relative_path,
                    byte_len: target.byte_len,
                    byte_domain: match target.byte_domain {
                        BudgetByteDomain::Source => FlowBudgetByteDomainV1::Source,
                        BudgetByteDomain::RenderedOutput => FlowBudgetByteDomainV1::RenderedOutput,
                    },
                })
                .collect(),
            next_safe_action: value.next_safe_action.map(|action| match action {
                BudgetNextSafeAction::ReduceCanonicalArtifactSize {
                    canonical_repo_relative_path,
                } => FlowBudgetNextSafeActionV1::ReduceCanonicalArtifactSize {
                    canonical_repo_relative_path,
                },
            }),
        }
    }

    fn into_budget(self) -> BudgetOutcome {
        BudgetOutcome {
            disposition: budget_disposition_back(self.disposition),
            reason: budget_reason_back(self.reason),
            targets: self
                .targets
                .into_iter()
                .map(|target| BudgetTarget {
                    canonical_repo_relative_path: target.canonical_repo_relative_path,
                    byte_len: target.byte_len,
                    byte_domain: match target.byte_domain {
                        FlowBudgetByteDomainV1::Source => BudgetByteDomain::Source,
                        FlowBudgetByteDomainV1::RenderedOutput => BudgetByteDomain::RenderedOutput,
                    },
                })
                .collect(),
            next_safe_action: self.next_safe_action.map(|action| match action {
                FlowBudgetNextSafeActionV1::ReduceCanonicalArtifactSize {
                    canonical_repo_relative_path,
                } => BudgetNextSafeAction::ReduceCanonicalArtifactSize {
                    canonical_repo_relative_path,
                },
            }),
        }
    }
}

impl FlowRefusalV1 {
    fn from_refusal(value: Refusal) -> Self {
        Self {
            category: refusal_category(value.category),
            summary: value.summary,
            broken_subject: subject(value.broken_subject),
            next_safe_action: next_action(value.next_safe_action),
        }
    }

    fn into_refusal(self) -> Result<Refusal, ContractError> {
        Ok(Refusal {
            category: refusal_category_back(self.category),
            summary: self.summary,
            broken_subject: subject_back(self.broken_subject)?,
            next_safe_action: next_action_back(self.next_safe_action)?,
        })
    }
}

impl FlowBlockerV1 {
    fn from_blocker(value: Blocker) -> Self {
        Self {
            category: blocker_category(value.category),
            subject: subject(value.subject),
            summary: value.summary,
            next_safe_action: next_action(value.next_safe_action),
        }
    }

    fn into_blocker(self) -> Result<Blocker, ContractError> {
        Ok(Blocker {
            category: blocker_category_back(self.category),
            subject: subject_back(self.subject)?,
            summary: self.summary,
            next_safe_action: next_action_back(self.next_safe_action)?,
        })
    }
}

fn selection_status(value: PacketSelectionStatus) -> FlowSelectionStatusV1 {
    match value {
        PacketSelectionStatus::Selected => FlowSelectionStatusV1::Selected,
        PacketSelectionStatus::Blocked => FlowSelectionStatusV1::Blocked,
    }
}

fn selection_status_back(value: FlowSelectionStatusV1) -> PacketSelectionStatus {
    match value {
        FlowSelectionStatusV1::Selected => PacketSelectionStatus::Selected,
        FlowSelectionStatusV1::Blocked => PacketSelectionStatus::Blocked,
    }
}

fn budget_disposition(value: BudgetDisposition) -> FlowBudgetDispositionV1 {
    match value {
        BudgetDisposition::Keep => FlowBudgetDispositionV1::Keep,
        BudgetDisposition::Summarize => FlowBudgetDispositionV1::Summarize,
        BudgetDisposition::Exclude => FlowBudgetDispositionV1::Exclude,
        BudgetDisposition::Refuse => FlowBudgetDispositionV1::Refuse,
    }
}

fn budget_disposition_back(value: FlowBudgetDispositionV1) -> BudgetDisposition {
    match value {
        FlowBudgetDispositionV1::Keep => BudgetDisposition::Keep,
        FlowBudgetDispositionV1::Summarize => BudgetDisposition::Summarize,
        FlowBudgetDispositionV1::Exclude => BudgetDisposition::Exclude,
        FlowBudgetDispositionV1::Refuse => BudgetDisposition::Refuse,
    }
}

fn budget_reason(value: BudgetReason) -> FlowBudgetReasonV1 {
    match value {
        BudgetReason::WithinBudget => FlowBudgetReasonV1::WithinBudget,
        BudgetReason::OptionalArtifactTooLarge => FlowBudgetReasonV1::OptionalArtifactTooLarge,
        BudgetReason::TotalBytesExceeded => FlowBudgetReasonV1::TotalBytesExceeded,
        BudgetReason::RequiredArtifactTooLarge => FlowBudgetReasonV1::RequiredArtifactTooLarge,
    }
}

fn budget_reason_back(value: FlowBudgetReasonV1) -> BudgetReason {
    match value {
        FlowBudgetReasonV1::WithinBudget => BudgetReason::WithinBudget,
        FlowBudgetReasonV1::OptionalArtifactTooLarge => BudgetReason::OptionalArtifactTooLarge,
        FlowBudgetReasonV1::TotalBytesExceeded => BudgetReason::TotalBytesExceeded,
        FlowBudgetReasonV1::RequiredArtifactTooLarge => BudgetReason::RequiredArtifactTooLarge,
    }
}

fn refusal_category(value: RefusalCategory) -> FlowRefusalCategoryV1 {
    match value {
        RefusalCategory::NonCanonicalInputAttempt => {
            FlowRefusalCategoryV1::NonCanonicalInputAttempt
        }
        RefusalCategory::SystemRootMissing => FlowRefusalCategoryV1::SystemRootMissing,
        RefusalCategory::SystemRootNotDir => FlowRefusalCategoryV1::SystemRootNotDir,
        RefusalCategory::SystemRootSymlinkNotAllowed => {
            FlowRefusalCategoryV1::SystemRootSymlinkNotAllowed
        }
        RefusalCategory::RequiredArtifactMissing => FlowRefusalCategoryV1::RequiredArtifactMissing,
        RefusalCategory::RequiredArtifactEmpty => FlowRefusalCategoryV1::RequiredArtifactEmpty,
        RefusalCategory::RequiredArtifactStarterTemplate => {
            FlowRefusalCategoryV1::RequiredArtifactStarterTemplate
        }
        RefusalCategory::RequiredArtifactInvalid => FlowRefusalCategoryV1::RequiredArtifactInvalid,
        RefusalCategory::ArtifactReadError => FlowRefusalCategoryV1::ArtifactReadError,
        RefusalCategory::FreshnessInvalid => FlowRefusalCategoryV1::FreshnessInvalid,
        RefusalCategory::BudgetRefused => FlowRefusalCategoryV1::BudgetRefused,
        RefusalCategory::UnsupportedRequest => FlowRefusalCategoryV1::UnsupportedRequest,
    }
}

fn refusal_category_back(value: FlowRefusalCategoryV1) -> RefusalCategory {
    match value {
        FlowRefusalCategoryV1::NonCanonicalInputAttempt => {
            RefusalCategory::NonCanonicalInputAttempt
        }
        FlowRefusalCategoryV1::SystemRootMissing => RefusalCategory::SystemRootMissing,
        FlowRefusalCategoryV1::SystemRootNotDir => RefusalCategory::SystemRootNotDir,
        FlowRefusalCategoryV1::SystemRootSymlinkNotAllowed => {
            RefusalCategory::SystemRootSymlinkNotAllowed
        }
        FlowRefusalCategoryV1::RequiredArtifactMissing => RefusalCategory::RequiredArtifactMissing,
        FlowRefusalCategoryV1::RequiredArtifactEmpty => RefusalCategory::RequiredArtifactEmpty,
        FlowRefusalCategoryV1::RequiredArtifactStarterTemplate => {
            RefusalCategory::RequiredArtifactStarterTemplate
        }
        FlowRefusalCategoryV1::RequiredArtifactInvalid => RefusalCategory::RequiredArtifactInvalid,
        FlowRefusalCategoryV1::ArtifactReadError => RefusalCategory::ArtifactReadError,
        FlowRefusalCategoryV1::FreshnessInvalid => RefusalCategory::FreshnessInvalid,
        FlowRefusalCategoryV1::BudgetRefused => RefusalCategory::BudgetRefused,
        FlowRefusalCategoryV1::UnsupportedRequest => RefusalCategory::UnsupportedRequest,
    }
}

fn blocker_category(value: BlockerCategory) -> FlowBlockerCategoryV1 {
    match value {
        BlockerCategory::SystemRootMissing => FlowBlockerCategoryV1::SystemRootMissing,
        BlockerCategory::SystemRootNotDir => FlowBlockerCategoryV1::SystemRootNotDir,
        BlockerCategory::SystemRootSymlinkNotAllowed => {
            FlowBlockerCategoryV1::SystemRootSymlinkNotAllowed
        }
        BlockerCategory::RequiredArtifactMissing => FlowBlockerCategoryV1::RequiredArtifactMissing,
        BlockerCategory::RequiredArtifactEmpty => FlowBlockerCategoryV1::RequiredArtifactEmpty,
        BlockerCategory::RequiredArtifactStarterTemplate => {
            FlowBlockerCategoryV1::RequiredArtifactStarterTemplate
        }
        BlockerCategory::RequiredArtifactInvalid => FlowBlockerCategoryV1::RequiredArtifactInvalid,
        BlockerCategory::ArtifactReadError => FlowBlockerCategoryV1::ArtifactReadError,
        BlockerCategory::FreshnessInvalid => FlowBlockerCategoryV1::FreshnessInvalid,
        BlockerCategory::BudgetRefused => FlowBlockerCategoryV1::BudgetRefused,
        BlockerCategory::UnsupportedRequest => FlowBlockerCategoryV1::UnsupportedRequest,
    }
}

fn blocker_category_back(value: FlowBlockerCategoryV1) -> BlockerCategory {
    match value {
        FlowBlockerCategoryV1::SystemRootMissing => BlockerCategory::SystemRootMissing,
        FlowBlockerCategoryV1::SystemRootNotDir => BlockerCategory::SystemRootNotDir,
        FlowBlockerCategoryV1::SystemRootSymlinkNotAllowed => {
            BlockerCategory::SystemRootSymlinkNotAllowed
        }
        FlowBlockerCategoryV1::RequiredArtifactMissing => BlockerCategory::RequiredArtifactMissing,
        FlowBlockerCategoryV1::RequiredArtifactEmpty => BlockerCategory::RequiredArtifactEmpty,
        FlowBlockerCategoryV1::RequiredArtifactStarterTemplate => {
            BlockerCategory::RequiredArtifactStarterTemplate
        }
        FlowBlockerCategoryV1::RequiredArtifactInvalid => BlockerCategory::RequiredArtifactInvalid,
        FlowBlockerCategoryV1::ArtifactReadError => BlockerCategory::ArtifactReadError,
        FlowBlockerCategoryV1::FreshnessInvalid => BlockerCategory::FreshnessInvalid,
        FlowBlockerCategoryV1::BudgetRefused => BlockerCategory::BudgetRefused,
        FlowBlockerCategoryV1::UnsupportedRequest => BlockerCategory::UnsupportedRequest,
    }
}

fn subject(value: SubjectRef) -> FlowSubjectRefV1 {
    match value {
        SubjectRef::CanonicalArtifact {
            instance_id,
            kind_ref,
            label,
            canonical_repo_relative_path,
        } => FlowSubjectRefV1::CanonicalArtifact {
            instance_id,
            kind_ref,
            label,
            canonical_repo_relative_path,
        },
        SubjectRef::InheritedDependency {
            dependency_id,
            version,
        } => FlowSubjectRefV1::InheritedDependency {
            dependency_id,
            version,
        },
        SubjectRef::Policy { policy_id } => FlowSubjectRefV1::Policy {
            policy_id: policy_id.to_owned(),
        },
    }
}

fn subject_back(value: FlowSubjectRefV1) -> Result<SubjectRef, ContractError> {
    Ok(match value {
        FlowSubjectRefV1::CanonicalArtifact {
            instance_id,
            kind_ref,
            label,
            canonical_repo_relative_path,
        } => SubjectRef::CanonicalArtifact {
            instance_id,
            kind_ref,
            label,
            canonical_repo_relative_path,
        },
        FlowSubjectRefV1::InheritedDependency {
            dependency_id,
            version,
        } => SubjectRef::InheritedDependency {
            dependency_id,
            version,
        },
        FlowSubjectRefV1::Policy { policy_id } => SubjectRef::Policy {
            policy_id: match policy_id.as_str() {
                "system_root" => "system_root",
                "freshness" => "freshness",
                "budget" => "budget",
                "packet_id" => "packet_id",
                _ => return Err(ContractError::InvalidInvariant),
            },
        },
    })
}

fn next_action(value: NextSafeAction) -> FlowNextSafeActionV1 {
    match value {
        NextSafeAction::RunSetup => FlowNextSafeActionV1::RunSetup,
        NextSafeAction::RunSetupInit => FlowNextSafeActionV1::RunSetupInit,
        NextSafeAction::RunSetupRefresh => FlowNextSafeActionV1::RunSetupRefresh,
        NextSafeAction::RunAuthorCharter => FlowNextSafeActionV1::RunAuthorCharter,
        NextSafeAction::RunAuthorProjectContext => FlowNextSafeActionV1::RunAuthorProjectContext,
        NextSafeAction::CreateSystemRoot {
            canonical_repo_relative_path,
        } => FlowNextSafeActionV1::CreateSystemRoot {
            canonical_repo_relative_path,
        },
        NextSafeAction::EnsureSystemRootIsDirectory {
            canonical_repo_relative_path,
        } => FlowNextSafeActionV1::EnsureSystemRootIsDirectory {
            canonical_repo_relative_path,
        },
        NextSafeAction::RemoveSystemRootSymlink {
            canonical_repo_relative_path,
        } => FlowNextSafeActionV1::RemoveSystemRootSymlink {
            canonical_repo_relative_path,
        },
        NextSafeAction::CreateCanonicalArtifact {
            canonical_repo_relative_path,
        } => FlowNextSafeActionV1::CreateCanonicalArtifact {
            canonical_repo_relative_path,
        },
        NextSafeAction::FillCanonicalArtifact {
            canonical_repo_relative_path,
        } => FlowNextSafeActionV1::FillCanonicalArtifact {
            canonical_repo_relative_path,
        },
        NextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        } => FlowNextSafeActionV1::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        },
        NextSafeAction::RunGenerate { packet_id } => FlowNextSafeActionV1::RunGenerate {
            packet_id: packet_id.to_owned(),
        },
        NextSafeAction::RunDoctor => FlowNextSafeActionV1::RunDoctor,
    }
}

fn next_action_back(value: FlowNextSafeActionV1) -> Result<NextSafeAction, ContractError> {
    Ok(match value {
        FlowNextSafeActionV1::RunSetup => NextSafeAction::RunSetup,
        FlowNextSafeActionV1::RunSetupInit => NextSafeAction::RunSetupInit,
        FlowNextSafeActionV1::RunSetupRefresh => NextSafeAction::RunSetupRefresh,
        FlowNextSafeActionV1::RunAuthorCharter => NextSafeAction::RunAuthorCharter,
        FlowNextSafeActionV1::RunAuthorProjectContext => NextSafeAction::RunAuthorProjectContext,
        FlowNextSafeActionV1::CreateSystemRoot {
            canonical_repo_relative_path,
        } => NextSafeAction::CreateSystemRoot {
            canonical_repo_relative_path,
        },
        FlowNextSafeActionV1::EnsureSystemRootIsDirectory {
            canonical_repo_relative_path,
        } => NextSafeAction::EnsureSystemRootIsDirectory {
            canonical_repo_relative_path,
        },
        FlowNextSafeActionV1::RemoveSystemRootSymlink {
            canonical_repo_relative_path,
        } => NextSafeAction::RemoveSystemRootSymlink {
            canonical_repo_relative_path,
        },
        FlowNextSafeActionV1::CreateCanonicalArtifact {
            canonical_repo_relative_path,
        } => NextSafeAction::CreateCanonicalArtifact {
            canonical_repo_relative_path,
        },
        FlowNextSafeActionV1::FillCanonicalArtifact {
            canonical_repo_relative_path,
        } => NextSafeAction::FillCanonicalArtifact {
            canonical_repo_relative_path,
        },
        FlowNextSafeActionV1::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        } => NextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        },
        FlowNextSafeActionV1::RunGenerate { packet_id } => NextSafeAction::RunGenerate {
            packet_id: match packet_id.as_str() {
                "planning.packet" => "planning.packet",
                "execution.demo.packet" => "execution.demo.packet",
                "execution.live.packet" => "execution.live.packet",
                _ => return Err(ContractError::InvalidInvariant),
            },
        },
        FlowNextSafeActionV1::RunDoctor => NextSafeAction::RunDoctor,
    })
}

pub struct FlowResolveSchemaValidator;

impl OperationSchemaValidator for FlowResolveSchemaValidator {
    fn validate_instance(
        &self,
        schema: &ExactBinding,
        instance: &ContractValue,
    ) -> Result<(), ContractError> {
        let limits = ContractLimits::packet0();
        let bytes = canonical_json_bytes(instance, &limits)?;
        validate_checked_schema(schema, &bytes)?;
        match schema.reference().as_str() {
            FLOW_REQUEST_SCHEMA_REF => {
                parse_canonical_json::<FlowResolveRequestV1>(&bytes, &limits).map(|_| ())
            }
            FLOW_RESULT_SCHEMA_REF => {
                parse_canonical_json::<FlowResolveResultV1>(&bytes, &limits)?.validate_semantics()
            }
            FLOW_DETAILS_SCHEMA_REF => {
                parse_canonical_json::<FlowProblemDetailsV1>(&bytes, &limits)?.validate_semantics()
            }
            FLOW_RECHECK_SCHEMA_REF => {
                let recheck = parse_canonical_json::<FlowRecheckV1>(&bytes, &limits)?;
                if is_prerequisite_cause(recheck.cause) {
                    Ok(())
                } else {
                    Err(ContractError::InvalidInvariant)
                }
            }
            _ => Err(ContractError::InvalidInvariant),
        }
    }

    fn validate_problem(
        &self,
        status: TerminalStatus,
        problem: &Problem,
    ) -> Result<(), ContractError> {
        self.validate_instance(problem.details_schema(), problem.details())?;
        let details: FlowProblemDetailsV1 = from_contract_value(problem.details())?;
        let (expected_status, expected_category, expected_retry) =
            problem_projection(details.cause);
        if status != expected_status
            || problem.category() != expected_category
            || problem.retry() != expected_retry
            || problem.recheck().is_some() != (status == TerminalStatus::Blocked)
            || problem.rule().is_some() != (status == TerminalStatus::Refused)
            || problem.correlation_ref().is_some() != (status == TerminalStatus::Error)
        {
            return Err(ContractError::InvalidInvariant);
        }
        if let Some(recheck) = problem.recheck() {
            self.validate_instance(recheck.condition_schema(), recheck.condition())?;
            let recheck: FlowRecheckV1 = from_contract_value(recheck.condition())?;
            if recheck.cause != details.cause {
                return Err(ContractError::InvalidInvariant);
            }
        }
        Ok(())
    }
}

fn validate_checked_schema(binding: &ExactBinding, instance: &[u8]) -> Result<(), ContractError> {
    let source =
        flow_schema_source(binding.reference().as_str()).ok_or(ContractError::InvalidInvariant)?;
    if binding.fingerprint() != &super::sha256_fingerprint(source.as_bytes()) {
        return Err(ContractError::InvalidInvariant);
    }
    let schema = serde_json::from_str::<serde_json::Value>(source)
        .map_err(|_| ContractError::InvalidJson)?;
    let instance = serde_json::from_slice::<serde_json::Value>(instance)
        .map_err(|_| ContractError::InvalidJson)?;
    let validator = jsonschema::draft202012::options()
        .with_pattern_options(jsonschema::PatternOptions::fancy_regex())
        .build(&schema)
        .map_err(|_| ContractError::InvalidInvariant)?;
    if validator.is_valid(&instance) {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
}

fn flow_schema_source(reference: &str) -> Option<&'static str> {
    Some(match reference {
        FLOW_DETAILS_SCHEMA_REF => {
            include_str!("../../schemas/2020-12/handbook.flow-operation-details/1.0.0.schema.json")
        }
        FLOW_RECHECK_SCHEMA_REF => {
            include_str!("../../schemas/2020-12/handbook.flow-recheck/1.0.0.schema.json")
        }
        FLOW_REQUEST_SCHEMA_REF => include_str!(
            "../../schemas/2020-12/handbook.operation.flow-resolve-request/1.0.0.schema.json"
        ),
        FLOW_RESULT_SCHEMA_REF => include_str!(
            "../../schemas/2020-12/handbook.operation.flow-resolve-result/1.0.0.schema.json"
        ),
        _ => return None,
    })
}

pub fn flow_resolve_operation_definition() -> Result<OperationDefinition, ContractError> {
    let limits = ContractLimits::packet0();
    let owner = handbook_flow::operation_admission::operation_owner_version_evidence();
    OperationDefinition::new(
        schema_binding("handbook.operation-definition@1.0.0")?,
        ExactRef::parse(FLOW_OPERATION_REF)?,
        OperationOwnerBinding::new(
            BoundedString::new(owner.owner_crate.to_owned())?,
            SemVer::parse(owner.owner_version)?,
            ExactBinding::new(
                ExactRef::parse(owner.owner_source_ref)?,
                super::sha256_fingerprint(owner.source_bytes),
            ),
        )?,
        schema_binding(FLOW_REQUEST_SCHEMA_REF)?,
        schema_binding(FLOW_RESULT_SCHEMA_REF)?,
        OperationOutcomeSchemas::new(
            schema_binding("handbook.operation-blocker@1.0.0")?,
            schema_binding("handbook.operation-refusal@1.0.0")?,
            schema_binding("handbook.operation-error@1.0.0")?,
        )?,
        BoundedVec::new(Vec::new())?,
        OperationClass::ReadOnly,
        OperationIdempotency::safe(),
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(vec![
            OperationTransport::CliJson,
            OperationTransport::RustSdk,
            OperationTransport::Tauri,
        ])?,
        None,
        &limits,
    )
}

pub fn flow_resolve_operation_request(
    request_id: Option<&str>,
    packet_id: &str,
) -> Result<OperationRequestV1, ContractError> {
    let definition = flow_resolve_operation_definition()?;
    let body = to_contract_value(&FlowResolveRequestV1 {
        packet_id: FlowPacketIdV1::parse(packet_id)?,
    })?;
    OperationRequestV1::new(
        schema_binding("handbook.operation-request@1.0.0")?,
        request_id.map(RequestId::parse).transpose()?,
        &definition,
        body,
        &FlowResolveSchemaValidator,
        &ContractLimits::packet0(),
    )
}

pub(crate) fn execute_flow_operation(
    repo_root: &Path,
    request: &OperationRequestV1,
) -> Result<OperationResponseV1, ContractError> {
    let definition = flow_resolve_operation_definition()?;
    if request.operation() != &definition.binding()
        || request.body_schema() != definition.request_schema()
    {
        return Err(ContractError::InvalidInvariant);
    }
    match super::admit_operation_definition(&definition)? {
        OperationAdmissionResult::Admitted(owner)
            if owner.operation_ref() == FLOW_OPERATION_REF
                && owner.definition_fingerprint()
                    == definition.definition_fingerprint().as_str() => {}
        _ => return Err(ContractError::InvalidInvariant),
    }
    let body: FlowResolveRequestV1 = from_contract_value(request.body())?;
    let result = match crate::resolver::resolve_packet(
        repo_root,
        crate::resolver::PacketResolveRequest::new(body.packet_id.as_str()),
    ) {
        Ok(result) => result,
        Err(error) => return compiler_error_response(request, &definition, error),
    };
    resolution_response(request, &definition, result)
}

fn resolution_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    result: crate::flow_api::PacketResolutionView,
) -> Result<OperationResponseV1, ContractError> {
    let projection = FlowResolveResultV1::from_resolution(result);
    if projection.selection.status == FlowSelectionStatusV1::Selected
        && projection.refusal.is_none()
        && projection.blockers.is_empty()
    {
        return ok_response(request, definition, &projection);
    }

    let (cause, status, category, retry) = if let Some(refusal) = projection.refusal.as_ref() {
        resolution_refusal_projection(refusal.category)
    } else if let Some(blocker) = projection.blockers.first() {
        resolution_blocker_projection(blocker.category)
    } else {
        return Err(ContractError::InvalidInvariant);
    };
    problem_response(
        request,
        definition,
        cause,
        status,
        category,
        retry,
        Some(projection),
        "Flow resolution did not reach a ready packet",
    )
}

fn resolution_refusal_projection(
    category: FlowRefusalCategoryV1,
) -> (
    FlowProblemCauseV1,
    TerminalStatus,
    ProblemCategory,
    RetryDirective,
) {
    match category {
        FlowRefusalCategoryV1::NonCanonicalInputAttempt => (
            FlowProblemCauseV1::NonCanonicalInputAttempt,
            TerminalStatus::Refused,
            ProblemCategory::Safety,
            RetryDirective::Never,
        ),
        FlowRefusalCategoryV1::UnsupportedRequest => (
            FlowProblemCauseV1::UnsupportedRequest,
            TerminalStatus::Refused,
            ProblemCategory::Capability,
            RetryDirective::Never,
        ),
        FlowRefusalCategoryV1::SystemRootMissing => {
            prerequisite(FlowProblemCauseV1::SystemRootMissing)
        }
        FlowRefusalCategoryV1::SystemRootNotDir => {
            prerequisite(FlowProblemCauseV1::SystemRootNotDir)
        }
        FlowRefusalCategoryV1::SystemRootSymlinkNotAllowed => {
            prerequisite(FlowProblemCauseV1::SystemRootSymlinkNotAllowed)
        }
        FlowRefusalCategoryV1::RequiredArtifactMissing => {
            prerequisite(FlowProblemCauseV1::RequiredArtifactMissing)
        }
        FlowRefusalCategoryV1::RequiredArtifactEmpty => {
            prerequisite(FlowProblemCauseV1::RequiredArtifactEmpty)
        }
        FlowRefusalCategoryV1::RequiredArtifactStarterTemplate => {
            prerequisite(FlowProblemCauseV1::RequiredArtifactStarterTemplate)
        }
        FlowRefusalCategoryV1::RequiredArtifactInvalid => {
            prerequisite(FlowProblemCauseV1::RequiredArtifactInvalid)
        }
        FlowRefusalCategoryV1::ArtifactReadError => {
            prerequisite(FlowProblemCauseV1::ArtifactReadError)
        }
        FlowRefusalCategoryV1::FreshnessInvalid => {
            prerequisite(FlowProblemCauseV1::FreshnessInvalid)
        }
        FlowRefusalCategoryV1::BudgetRefused => prerequisite(FlowProblemCauseV1::BudgetRefused),
    }
}

fn resolution_blocker_projection(
    category: FlowBlockerCategoryV1,
) -> (
    FlowProblemCauseV1,
    TerminalStatus,
    ProblemCategory,
    RetryDirective,
) {
    match category {
        FlowBlockerCategoryV1::UnsupportedRequest => (
            FlowProblemCauseV1::UnsupportedRequest,
            TerminalStatus::Refused,
            ProblemCategory::Capability,
            RetryDirective::Never,
        ),
        FlowBlockerCategoryV1::SystemRootMissing => {
            prerequisite(FlowProblemCauseV1::SystemRootMissing)
        }
        FlowBlockerCategoryV1::SystemRootNotDir => {
            prerequisite(FlowProblemCauseV1::SystemRootNotDir)
        }
        FlowBlockerCategoryV1::SystemRootSymlinkNotAllowed => {
            prerequisite(FlowProblemCauseV1::SystemRootSymlinkNotAllowed)
        }
        FlowBlockerCategoryV1::RequiredArtifactMissing => {
            prerequisite(FlowProblemCauseV1::RequiredArtifactMissing)
        }
        FlowBlockerCategoryV1::RequiredArtifactEmpty => {
            prerequisite(FlowProblemCauseV1::RequiredArtifactEmpty)
        }
        FlowBlockerCategoryV1::RequiredArtifactStarterTemplate => {
            prerequisite(FlowProblemCauseV1::RequiredArtifactStarterTemplate)
        }
        FlowBlockerCategoryV1::RequiredArtifactInvalid => {
            prerequisite(FlowProblemCauseV1::RequiredArtifactInvalid)
        }
        FlowBlockerCategoryV1::ArtifactReadError => {
            prerequisite(FlowProblemCauseV1::ArtifactReadError)
        }
        FlowBlockerCategoryV1::FreshnessInvalid => {
            prerequisite(FlowProblemCauseV1::FreshnessInvalid)
        }
        FlowBlockerCategoryV1::BudgetRefused => prerequisite(FlowProblemCauseV1::BudgetRefused),
    }
}

fn prerequisite(
    cause: FlowProblemCauseV1,
) -> (
    FlowProblemCauseV1,
    TerminalStatus,
    ProblemCategory,
    RetryDirective,
) {
    (
        cause,
        TerminalStatus::Blocked,
        ProblemCategory::Prerequisite,
        RetryDirective::AfterRecheck,
    )
}

fn problem_projection(
    cause: FlowProblemCauseV1,
) -> (TerminalStatus, ProblemCategory, RetryDirective) {
    match cause {
        FlowProblemCauseV1::NonCanonicalInputAttempt => (
            TerminalStatus::Refused,
            ProblemCategory::Safety,
            RetryDirective::Never,
        ),
        FlowProblemCauseV1::UnsupportedRequest => (
            TerminalStatus::Refused,
            ProblemCategory::Capability,
            RetryDirective::Never,
        ),
        FlowProblemCauseV1::OwnerReadFailure => (
            TerminalStatus::Error,
            ProblemCategory::Implementation,
            RetryDirective::Transient,
        ),
        FlowProblemCauseV1::OwnerProfileDecision => (
            TerminalStatus::Error,
            ProblemCategory::Implementation,
            RetryDirective::Never,
        ),
        FlowProblemCauseV1::SystemRootMissing
        | FlowProblemCauseV1::SystemRootNotDir
        | FlowProblemCauseV1::SystemRootSymlinkNotAllowed
        | FlowProblemCauseV1::RequiredArtifactMissing
        | FlowProblemCauseV1::RequiredArtifactEmpty
        | FlowProblemCauseV1::RequiredArtifactStarterTemplate
        | FlowProblemCauseV1::RequiredArtifactInvalid
        | FlowProblemCauseV1::ArtifactReadError
        | FlowProblemCauseV1::FreshnessInvalid
        | FlowProblemCauseV1::BudgetRefused => (
            TerminalStatus::Blocked,
            ProblemCategory::Prerequisite,
            RetryDirective::AfterRecheck,
        ),
    }
}

fn compiler_error_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    error: crate::CompilerError,
) -> Result<OperationResponseV1, ContractError> {
    use handbook_engine::artifact_manifest::ManifestError;
    use handbook_engine::canonical_artifacts::ArtifactIngestError;
    match error {
        crate::CompilerError::Manifest(ManifestError::Ingest(error)) => match error {
            ArtifactIngestError::SystemRootMissing { .. } => prerequisite_error_response(
                request,
                definition,
                FlowProblemCauseV1::SystemRootMissing,
            ),
            ArtifactIngestError::SystemRootNotDir { .. } => prerequisite_error_response(
                request,
                definition,
                FlowProblemCauseV1::SystemRootNotDir,
            ),
            ArtifactIngestError::SystemRootSymlinkNotAllowed { .. } => prerequisite_error_response(
                request,
                definition,
                FlowProblemCauseV1::SystemRootSymlinkNotAllowed,
            ),
            ArtifactIngestError::RequiredArtifactMissing { .. } => prerequisite_error_response(
                request,
                definition,
                FlowProblemCauseV1::RequiredArtifactMissing,
            ),
            ArtifactIngestError::ReadFailure { .. } => problem_response(
                request,
                definition,
                FlowProblemCauseV1::OwnerReadFailure,
                TerminalStatus::Error,
                ProblemCategory::Implementation,
                RetryDirective::Transient,
                None,
                "Flow owner could not read repository truth",
            ),
        },
        crate::CompilerError::Manifest(ManifestError::ProfileDecision(_)) => problem_response(
            request,
            definition,
            FlowProblemCauseV1::OwnerProfileDecision,
            TerminalStatus::Error,
            ProblemCategory::Implementation,
            RetryDirective::Never,
            None,
            "Flow owner could not establish the selected profile decision",
        ),
    }
}

fn prerequisite_error_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    cause: FlowProblemCauseV1,
) -> Result<OperationResponseV1, ContractError> {
    problem_response(
        request,
        definition,
        cause,
        TerminalStatus::Blocked,
        ProblemCategory::Prerequisite,
        RetryDirective::AfterRecheck,
        None,
        "Flow repository prerequisite is unavailable",
    )
}

#[allow(clippy::too_many_arguments)]
fn problem_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    cause: FlowProblemCauseV1,
    status: TerminalStatus,
    category: ProblemCategory,
    retry: RetryDirective,
    resolution: Option<FlowResolveResultV1>,
    detail: &str,
) -> Result<OperationResponseV1, ContractError> {
    let limits = ContractLimits::packet0();
    let cause_name = cause_name(cause);
    let origin = if resolution.is_some() {
        FlowProblemOriginV1::Resolution
    } else if is_owner_error_cause(cause) {
        FlowProblemOriginV1::OwnerError
    } else {
        FlowProblemOriginV1::OwnerIngest
    };
    let details = to_contract_value(&FlowProblemDetailsV1 {
        origin,
        cause,
        detail: detail.to_owned(),
        resolution,
    })?;
    let details_schema = schema_binding(FLOW_DETAILS_SCHEMA_REF)?;
    FlowResolveSchemaValidator.validate_instance(&details_schema, &details)?;
    let recheck = if status == TerminalStatus::Blocked {
        let condition = to_contract_value(&FlowRecheckV1 { cause })?;
        let schema = schema_binding(FLOW_RECHECK_SCHEMA_REF)?;
        FlowResolveSchemaValidator.validate_instance(&schema, &condition)?;
        Some(RecheckCondition::new(schema, condition, &limits)?)
    } else {
        None
    };
    let rule = if status == TerminalStatus::Refused {
        Some(ExactBinding::new(
            ExactRef::parse(&format!(
                "handbook.flow-rule.{}@1.0.0",
                cause_name.replace('_', "-")
            ))?,
            definition.owner().source().fingerprint().clone(),
        ))
    } else {
        None
    };
    let correlation = if status == TerminalStatus::Error {
        let fingerprint = super::sha256_fingerprint(
            format!(
                "handbook.flow-error-correlation-v1\0{}\0{cause_name}",
                request.request_fingerprint().as_str()
            )
            .as_bytes(),
        );
        Some(CorrelationRef::new(BoundedString::new(format!(
            "corr_{}",
            fingerprint.as_str().trim_start_matches("sha256:")
        ))?)?)
    } else {
        None
    };
    let problem = Problem::new(
        BoundedString::new(format!("flow.{cause_name}"))?,
        BoundedString::new(format!("flow.{cause_name}"))?,
        category,
        definition.binding(),
        rule,
        details_schema,
        details,
        BoundedVec::new(Vec::new())?,
        retry,
        recheck,
        correlation,
        &limits,
    )?;
    let terminal = match status {
        TerminalStatus::Blocked => TerminalFields::new(
            status,
            None,
            BoundedVec::new(vec![problem])?,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
        )?,
        TerminalStatus::Refused => TerminalFields::new(
            status,
            None,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(vec![problem])?,
            BoundedVec::new(Vec::new())?,
        )?,
        TerminalStatus::Error => TerminalFields::new(
            status,
            None,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(vec![problem])?,
        )?,
        TerminalStatus::Ok => return Err(ContractError::InvalidInvariant),
    };
    response_from_parts(request, definition, terminal)
}

fn ok_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    data: &FlowResolveResultV1,
) -> Result<OperationResponseV1, ContractError> {
    response_from_parts(
        request,
        definition,
        TerminalFields::new(
            TerminalStatus::Ok,
            Some(to_contract_value(data)?),
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
        )?,
    )
}

fn response_from_parts(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    terminal: TerminalFields<ContractValue>,
) -> Result<OperationResponseV1, ContractError> {
    OperationResponseV1::new(
        schema_binding("handbook.operation-response@1.0.0")?,
        request.request_id().cloned(),
        request.request_fingerprint().clone(),
        definition,
        terminal,
        IdempotencyState::not_applicable()?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        &FlowResolveSchemaValidator,
        &ContractLimits::packet0(),
    )
}

pub fn flow_resolution_from_response(
    response: &OperationResponseV1,
) -> Result<Option<crate::flow_api::PacketResolutionView>, ContractError> {
    if response.operation().reference().as_str() != FLOW_OPERATION_REF {
        return Err(ContractError::InvalidInvariant);
    }
    if let Some(data) = response.terminal().data() {
        return FlowResolveResultV1::into_resolution(from_contract_value(data)?).map(Some);
    }
    let problem = response
        .terminal()
        .blockers()
        .first()
        .or_else(|| response.terminal().refusals().first())
        .or_else(|| response.terminal().errors().first())
        .ok_or(ContractError::InvalidInvariant)?;
    from_contract_value::<FlowProblemDetailsV1>(problem.details())?
        .resolution
        .map(FlowResolveResultV1::into_resolution)
        .transpose()
}

pub fn flow_problem_detail(
    response: &OperationResponseV1,
) -> Result<Option<String>, ContractError> {
    let problem = response
        .terminal()
        .blockers()
        .first()
        .or_else(|| response.terminal().refusals().first())
        .or_else(|| response.terminal().errors().first());
    problem
        .map(|problem| {
            from_contract_value::<FlowProblemDetailsV1>(problem.details())
                .map(|details| details.detail)
        })
        .transpose()
}

pub(crate) fn current_flow_capability() -> Result<CapabilityEntry, ContractError> {
    let definition = flow_resolve_operation_definition()?;
    let owner = match super::admit_operation_definition(&definition)? {
        OperationAdmissionResult::Admitted(owner) => owner,
        OperationAdmissionResult::Omitted(_) => return Err(ContractError::InvalidInvariant),
    };
    CapabilityEntry::new(
        &definition,
        &FlowCliTransportEvidence { owner },
        &ContractLimits::packet0(),
    )
}

struct FlowCliTransportEvidence {
    owner: OwnerAdmissionToken,
}

impl CurrentTransportEvidenceSealed for FlowCliTransportEvidence {}

impl CurrentTransportEvidence for FlowCliTransportEvidence {
    fn owner_admission(&self) -> &OwnerAdmissionToken {
        &self.owner
    }

    fn current_transports(&self) -> &[OperationTransport] {
        &[OperationTransport::CliJson, OperationTransport::Tauri]
    }
}

fn is_prerequisite_cause(cause: FlowProblemCauseV1) -> bool {
    matches!(
        cause,
        FlowProblemCauseV1::SystemRootMissing
            | FlowProblemCauseV1::SystemRootNotDir
            | FlowProblemCauseV1::SystemRootSymlinkNotAllowed
            | FlowProblemCauseV1::RequiredArtifactMissing
            | FlowProblemCauseV1::RequiredArtifactEmpty
            | FlowProblemCauseV1::RequiredArtifactStarterTemplate
            | FlowProblemCauseV1::RequiredArtifactInvalid
            | FlowProblemCauseV1::ArtifactReadError
            | FlowProblemCauseV1::FreshnessInvalid
            | FlowProblemCauseV1::BudgetRefused
    )
}

fn is_owner_ingest_cause(cause: FlowProblemCauseV1) -> bool {
    matches!(
        cause,
        FlowProblemCauseV1::SystemRootMissing
            | FlowProblemCauseV1::SystemRootNotDir
            | FlowProblemCauseV1::SystemRootSymlinkNotAllowed
            | FlowProblemCauseV1::RequiredArtifactMissing
    )
}

fn is_owner_error_cause(cause: FlowProblemCauseV1) -> bool {
    matches!(
        cause,
        FlowProblemCauseV1::OwnerReadFailure | FlowProblemCauseV1::OwnerProfileDecision
    )
}

fn cause_name(cause: FlowProblemCauseV1) -> &'static str {
    match cause {
        FlowProblemCauseV1::NonCanonicalInputAttempt => "non_canonical_input_attempt",
        FlowProblemCauseV1::SystemRootMissing => "system_root_missing",
        FlowProblemCauseV1::SystemRootNotDir => "system_root_not_dir",
        FlowProblemCauseV1::SystemRootSymlinkNotAllowed => "system_root_symlink_not_allowed",
        FlowProblemCauseV1::RequiredArtifactMissing => "required_artifact_missing",
        FlowProblemCauseV1::RequiredArtifactEmpty => "required_artifact_empty",
        FlowProblemCauseV1::RequiredArtifactStarterTemplate => "required_artifact_starter_template",
        FlowProblemCauseV1::RequiredArtifactInvalid => "required_artifact_invalid",
        FlowProblemCauseV1::ArtifactReadError => "artifact_read_error",
        FlowProblemCauseV1::FreshnessInvalid => "freshness_invalid",
        FlowProblemCauseV1::BudgetRefused => "budget_refused",
        FlowProblemCauseV1::UnsupportedRequest => "unsupported_request",
        FlowProblemCauseV1::OwnerReadFailure => "owner_read_failure",
        FlowProblemCauseV1::OwnerProfileDecision => "owner_profile_decision",
    }
}

fn schema_binding(reference: &str) -> Result<ExactBinding, ContractError> {
    super::artifact_operation::schema_binding(reference)
}

fn to_contract_value<T: Serialize>(value: &T) -> Result<ContractValue, ContractError> {
    super::artifact_operation::to_contract_value(value)
}

fn from_contract_value<T: for<'de> Deserialize<'de> + Serialize>(
    value: &ContractValue,
) -> Result<T, ContractError> {
    parse_canonical_json(
        &canonical_json_bytes(value, &ContractLimits::packet0())?,
        &ContractLimits::packet0(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use handbook_engine::artifact_manifest::ManifestError;
    use handbook_engine::canonical_artifacts::ArtifactIngestError;

    fn assert_owner_error_projection(
        error: crate::CompilerError,
        expected_cause: FlowProblemCauseV1,
        expected_retry: RetryDirective,
        private_marker: &str,
    ) {
        let request = flow_resolve_operation_request(None, "planning.packet").unwrap();
        let definition = flow_resolve_operation_definition().unwrap();
        let response = compiler_error_response(&request, &definition, error).unwrap();
        assert_eq!(response.terminal().status(), TerminalStatus::Error);
        assert!(response.receipts().is_empty());
        let problem = &response.terminal().errors()[0];
        assert_eq!(problem.category(), ProblemCategory::Implementation);
        assert_eq!(problem.retry(), expected_retry);
        assert!(problem.recheck().is_none());
        assert!(problem.correlation_ref().is_some());
        let details: FlowProblemDetailsV1 = from_contract_value(problem.details()).unwrap();
        assert_eq!(details.origin, FlowProblemOriginV1::OwnerError);
        assert_eq!(details.cause, expected_cause);
        assert!(details.resolution.is_none());
        let bytes = canonical_json_bytes(&response, &ContractLimits::packet0()).unwrap();
        assert!(!String::from_utf8(bytes).unwrap().contains(private_marker));
    }

    #[test]
    fn owner_errors_are_closed_sanitized_and_never_fabricate_resolution_truth() {
        assert_owner_error_projection(
            crate::CompilerError::Manifest(ManifestError::Ingest(
                ArtifactIngestError::ReadFailure {
                    path: std::path::PathBuf::from("private-host-root/secret.yaml"),
                    source: std::io::Error::new(
                        std::io::ErrorKind::PermissionDenied,
                        "private-read-detail",
                    ),
                },
            )),
            FlowProblemCauseV1::OwnerReadFailure,
            RetryDirective::Transient,
            "private-host-root",
        );
        assert_owner_error_projection(
            crate::CompilerError::Manifest(ManifestError::ProfileDecision(
                "private-profile-detail".to_owned(),
            )),
            FlowProblemCauseV1::OwnerProfileDecision,
            RetryDirective::Never,
            "private-profile-detail",
        );
    }
}
