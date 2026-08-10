//! Closed Flow projections used by the Generate and Inspect SDK operations.
//!
//! Flow remains the semantic owner of packet selection and budgeting. This module
//! maps its private values into the SDK's operation-specific Rust values so a
//! transport client never imports Flow records directly.

use crate::{
    Blocker, BlockerCategory, CompilerError, DecisionLog, Refusal, RefusalCategory, SubjectRef,
};

pub const C04_RESULT_VERSION: &str = "reduced-v1-m8.3";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketSelectionStatus {
    Selected,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketSelection {
    pub packet_id: String,
    pub status: PacketSelectionStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactPresence {
    Missing,
    PresentEmpty,
    PresentNonEmpty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketVariant {
    Planning,
    ExecutionDemo,
    ExecutionLive,
}

impl PacketVariant {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Planning => "planning.packet",
            Self::ExecutionDemo => "execution.demo.packet",
            Self::ExecutionLive => "execution.live.packet",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketSourceSummary {
    pub instance_id: String,
    pub kind_ref: String,
    pub label: String,
    pub canonical_repo_relative_path: String,
    pub required: bool,
    pub presence: ArtifactPresence,
    pub byte_len: Option<u64>,
    pub content_sha256: Option<String>,
    pub rendered_output_byte_len: Option<u64>,
    pub rendered_output_sha256: Option<String>,
    pub rendered_media_type: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketBodyNoteKind {
    Omission,
    Budget,
    InheritedDependency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketSectionMode {
    Verbatim,
    Summary,
    Rendered,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketBodyNote {
    pub kind: PacketBodyNoteKind,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketSection {
    pub instance_id: String,
    pub kind_ref: String,
    pub label: String,
    pub canonical_repo_relative_path: String,
    pub title: String,
    pub mode: PacketSectionMode,
    pub contents: String,
    pub source_content_sha256: Option<String>,
    pub rendered_output_sha256: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketFixtureContext {
    pub fixture_set_id: String,
    pub fixture_basis_root: String,
    pub fixture_lineage: Vec<PacketSourceSummary>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadyPacketNextSafeAction {
    InspectProof,
    Generate,
    RunDoctor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketDecisionSummary {
    pub packet_status: PacketSelectionStatus,
    pub budget_disposition: BudgetDisposition,
    pub budget_reason: BudgetReason,
    pub decision_log_entries: usize,
    pub summary_line: String,
    pub ready_next_safe_action: ReadyPacketNextSafeAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketResult {
    pub packet_id: String,
    pub variant: PacketVariant,
    pub fixture_context: Option<PacketFixtureContext>,
    pub included_sources: Vec<PacketSourceSummary>,
    pub notes: Vec<PacketBodyNote>,
    pub decision_summary: PacketDecisionSummary,
    pub sections: Vec<PacketSection>,
}

impl PacketResult {
    pub fn is_ready(&self) -> bool {
        self.decision_summary.packet_status == PacketSelectionStatus::Selected
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetDisposition {
    Keep,
    Summarize,
    Exclude,
    Refuse,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetReason {
    WithinBudget,
    OptionalArtifactTooLarge,
    TotalBytesExceeded,
    RequiredArtifactTooLarge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BudgetByteDomain {
    Source,
    RenderedOutput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetTarget {
    pub canonical_repo_relative_path: String,
    pub byte_len: u64,
    pub byte_domain: BudgetByteDomain,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetNextSafeAction {
    ReduceCanonicalArtifactSize {
        canonical_repo_relative_path: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BudgetOutcome {
    pub disposition: BudgetDisposition,
    pub reason: BudgetReason,
    pub targets: Vec<BudgetTarget>,
    pub next_safe_action: Option<BudgetNextSafeAction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolverResult {
    pub c04_result_version: String,
    pub c03_schema_version: String,
    pub c03_manifest_generation_version: u32,
    pub c03_fingerprint_sha256: String,
    pub packet_result: PacketResult,
    pub decision_log: DecisionLog,
    pub budget_outcome: BudgetOutcome,
    pub selection: PacketSelection,
    pub refusal: Option<Refusal>,
    pub blockers: Vec<Blocker>,
}

/// A closed request for resolving one ordinary CLI packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketResolveRequest {
    packet_id: String,
}

impl PacketResolveRequest {
    pub fn new(packet_id: impl Into<String>) -> Self {
        Self {
            packet_id: packet_id.into(),
        }
    }
}

/// Resolves a selected packet through the SDK's bounded Flow composition.
pub(crate) fn resolve_packet(
    repo_root: impl AsRef<std::path::Path>,
    request: PacketResolveRequest,
) -> Result<ResolverResult, CompilerError> {
    let result = handbook_flow::resolve(
        repo_root,
        handbook_flow::ResolveRequest {
            packet_id: &request.packet_id,
            ..handbook_flow::ResolveRequest::default()
        },
    )
    .map_err(CompilerError::Manifest)?;

    Ok(ResolverResult {
        c04_result_version: result.c04_result_version,
        c03_schema_version: result.c03_schema_version,
        c03_manifest_generation_version: result.c03_manifest_generation_version,
        c03_fingerprint_sha256: result.c03_fingerprint_sha256,
        packet_result: map_packet_result(result.packet_result),
        decision_log: DecisionLog {
            entries: result.decision_log_entries,
        },
        budget_outcome: map_budget_outcome(result.budget_outcome),
        selection: PacketSelection {
            packet_id: result.selection.packet_id,
            status: map_packet_selection_status(result.selection.status),
        },
        refusal: result.refusal.map(map_refusal),
        blockers: result.blockers.into_iter().map(map_blocker).collect(),
    })
}

fn map_packet_result(value: handbook_flow::PacketResult) -> PacketResult {
    PacketResult {
        packet_id: value.packet_id,
        variant: match value.variant {
            handbook_flow::PacketVariant::Planning => PacketVariant::Planning,
            handbook_flow::PacketVariant::ExecutionDemo => PacketVariant::ExecutionDemo,
            handbook_flow::PacketVariant::ExecutionLive => PacketVariant::ExecutionLive,
        },
        fixture_context: value.fixture_context.map(|context| PacketFixtureContext {
            fixture_set_id: context.fixture_set_id,
            fixture_basis_root: context.fixture_basis_root,
            fixture_lineage: context
                .fixture_lineage
                .into_iter()
                .map(map_source)
                .collect(),
        }),
        included_sources: value.included_sources.into_iter().map(map_source).collect(),
        notes: value
            .notes
            .into_iter()
            .map(|note| PacketBodyNote {
                kind: match note.kind {
                    handbook_flow::PacketBodyNoteKind::Omission => PacketBodyNoteKind::Omission,
                    handbook_flow::PacketBodyNoteKind::Budget => PacketBodyNoteKind::Budget,
                    handbook_flow::PacketBodyNoteKind::InheritedDependency => {
                        PacketBodyNoteKind::InheritedDependency
                    }
                },
                text: note.text,
            })
            .collect(),
        decision_summary: PacketDecisionSummary {
            packet_status: map_packet_selection_status(value.decision_summary.packet_status),
            budget_disposition: map_budget_disposition(value.decision_summary.budget_disposition),
            budget_reason: map_budget_reason(value.decision_summary.budget_reason),
            decision_log_entries: value.decision_summary.decision_log_entries,
            summary_line: value.decision_summary.summary_line,
            ready_next_safe_action: match value.decision_summary.ready_next_safe_action {
                handbook_flow::ReadyPacketNextSafeAction::InspectProof => {
                    ReadyPacketNextSafeAction::InspectProof
                }
                handbook_flow::ReadyPacketNextSafeAction::Generate => {
                    ReadyPacketNextSafeAction::Generate
                }
                handbook_flow::ReadyPacketNextSafeAction::RunDoctor => {
                    ReadyPacketNextSafeAction::RunDoctor
                }
            },
        },
        sections: value
            .sections
            .into_iter()
            .map(|section| PacketSection {
                instance_id: section.instance_id,
                kind_ref: section.kind_ref,
                label: section.label,
                canonical_repo_relative_path: section.canonical_repo_relative_path,
                title: section.title,
                mode: match section.mode {
                    handbook_flow::PacketSectionMode::Verbatim => PacketSectionMode::Verbatim,
                    handbook_flow::PacketSectionMode::Summary => PacketSectionMode::Summary,
                    handbook_flow::PacketSectionMode::Rendered => PacketSectionMode::Rendered,
                },
                contents: section.contents,
                source_content_sha256: section.source_content_sha256,
                rendered_output_sha256: section.rendered_output_sha256,
            })
            .collect(),
    }
}

fn map_source(value: handbook_flow::PacketSourceSummary) -> PacketSourceSummary {
    PacketSourceSummary {
        instance_id: value.instance_id,
        kind_ref: value.kind_ref,
        label: value.label,
        canonical_repo_relative_path: value.canonical_repo_relative_path,
        required: value.required,
        presence: match value.presence {
            handbook_engine::ArtifactPresence::Missing => ArtifactPresence::Missing,
            handbook_engine::ArtifactPresence::PresentEmpty => ArtifactPresence::PresentEmpty,
            handbook_engine::ArtifactPresence::PresentNonEmpty => ArtifactPresence::PresentNonEmpty,
        },
        byte_len: value.byte_len,
        content_sha256: value.content_sha256,
        rendered_output_byte_len: value.rendered_output_byte_len,
        rendered_output_sha256: value.rendered_output_sha256,
        rendered_media_type: value.rendered_media_type,
    }
}

fn map_budget_outcome(value: handbook_flow::BudgetOutcome) -> BudgetOutcome {
    BudgetOutcome {
        disposition: map_budget_disposition(value.disposition),
        reason: map_budget_reason(value.reason),
        targets: value
            .targets
            .into_iter()
            .map(|target| BudgetTarget {
                canonical_repo_relative_path: target.canonical_repo_relative_path,
                byte_len: target.byte_len,
                byte_domain: match target.byte_domain {
                    handbook_flow::BudgetByteDomain::Source => BudgetByteDomain::Source,
                    handbook_flow::BudgetByteDomain::RenderedOutput => {
                        BudgetByteDomain::RenderedOutput
                    }
                },
            })
            .collect(),
        next_safe_action: value.next_safe_action.map(|action| match action {
            handbook_flow::NextSafeAction::ReduceCanonicalArtifactSize {
                canonical_repo_relative_path,
            } => BudgetNextSafeAction::ReduceCanonicalArtifactSize {
                canonical_repo_relative_path,
            },
        }),
    }
}

fn map_packet_selection_status(
    value: handbook_flow::PacketSelectionStatus,
) -> PacketSelectionStatus {
    match value {
        handbook_flow::PacketSelectionStatus::Selected => PacketSelectionStatus::Selected,
        handbook_flow::PacketSelectionStatus::Blocked => PacketSelectionStatus::Blocked,
    }
}

fn map_budget_disposition(value: handbook_flow::BudgetDisposition) -> BudgetDisposition {
    match value {
        handbook_flow::BudgetDisposition::Keep => BudgetDisposition::Keep,
        handbook_flow::BudgetDisposition::Summarize => BudgetDisposition::Summarize,
        handbook_flow::BudgetDisposition::Exclude => BudgetDisposition::Exclude,
        handbook_flow::BudgetDisposition::Refuse => BudgetDisposition::Refuse,
    }
}

fn map_budget_reason(value: handbook_flow::BudgetReason) -> BudgetReason {
    match value {
        handbook_flow::BudgetReason::WithinBudget => BudgetReason::WithinBudget,
        handbook_flow::BudgetReason::OptionalArtifactTooLarge => {
            BudgetReason::OptionalArtifactTooLarge
        }
        handbook_flow::BudgetReason::TotalBytesExceeded => BudgetReason::TotalBytesExceeded,
        handbook_flow::BudgetReason::RequiredArtifactTooLarge => {
            BudgetReason::RequiredArtifactTooLarge
        }
    }
}

fn map_refusal(refusal: handbook_flow::ResolverRefusal) -> Refusal {
    Refusal {
        category: map_refusal_category(refusal.category),
        summary: refusal.summary,
        broken_subject: map_subject_ref(refusal.broken_subject),
        next_safe_action: map_next_safe_action(refusal.next_safe_action),
    }
}

fn map_blocker(blocker: handbook_flow::ResolverBlocker) -> Blocker {
    Blocker {
        category: map_blocker_category(blocker.category),
        subject: map_subject_ref(blocker.subject),
        summary: blocker.summary,
        next_safe_action: map_next_safe_action(blocker.next_safe_action),
    }
}

fn map_refusal_category(category: handbook_flow::ResolverRefusalCategory) -> RefusalCategory {
    match category {
        handbook_flow::ResolverRefusalCategory::NonCanonicalInputAttempt => {
            RefusalCategory::NonCanonicalInputAttempt
        }
        handbook_flow::ResolverRefusalCategory::SystemRootMissing => {
            RefusalCategory::SystemRootMissing
        }
        handbook_flow::ResolverRefusalCategory::SystemRootNotDir => {
            RefusalCategory::SystemRootNotDir
        }
        handbook_flow::ResolverRefusalCategory::SystemRootSymlinkNotAllowed => {
            RefusalCategory::SystemRootSymlinkNotAllowed
        }
        handbook_flow::ResolverRefusalCategory::RequiredArtifactMissing => {
            RefusalCategory::RequiredArtifactMissing
        }
        handbook_flow::ResolverRefusalCategory::RequiredArtifactEmpty => {
            RefusalCategory::RequiredArtifactEmpty
        }
        handbook_flow::ResolverRefusalCategory::RequiredArtifactStarterTemplate => {
            RefusalCategory::RequiredArtifactStarterTemplate
        }
        handbook_flow::ResolverRefusalCategory::RequiredArtifactInvalid => {
            RefusalCategory::RequiredArtifactInvalid
        }
        handbook_flow::ResolverRefusalCategory::ArtifactReadError => {
            RefusalCategory::ArtifactReadError
        }
        handbook_flow::ResolverRefusalCategory::FreshnessInvalid => {
            RefusalCategory::FreshnessInvalid
        }
        handbook_flow::ResolverRefusalCategory::BudgetRefused => RefusalCategory::BudgetRefused,
        handbook_flow::ResolverRefusalCategory::UnsupportedRequest => {
            RefusalCategory::UnsupportedRequest
        }
    }
}

fn map_blocker_category(category: handbook_flow::ResolverBlockerCategory) -> BlockerCategory {
    match category {
        handbook_flow::ResolverBlockerCategory::SystemRootMissing => {
            BlockerCategory::SystemRootMissing
        }
        handbook_flow::ResolverBlockerCategory::SystemRootNotDir => {
            BlockerCategory::SystemRootNotDir
        }
        handbook_flow::ResolverBlockerCategory::SystemRootSymlinkNotAllowed => {
            BlockerCategory::SystemRootSymlinkNotAllowed
        }
        handbook_flow::ResolverBlockerCategory::RequiredArtifactMissing => {
            BlockerCategory::RequiredArtifactMissing
        }
        handbook_flow::ResolverBlockerCategory::RequiredArtifactEmpty => {
            BlockerCategory::RequiredArtifactEmpty
        }
        handbook_flow::ResolverBlockerCategory::RequiredArtifactStarterTemplate => {
            BlockerCategory::RequiredArtifactStarterTemplate
        }
        handbook_flow::ResolverBlockerCategory::RequiredArtifactInvalid => {
            BlockerCategory::RequiredArtifactInvalid
        }
        handbook_flow::ResolverBlockerCategory::ArtifactReadError => {
            BlockerCategory::ArtifactReadError
        }
        handbook_flow::ResolverBlockerCategory::FreshnessInvalid => {
            BlockerCategory::FreshnessInvalid
        }
        handbook_flow::ResolverBlockerCategory::BudgetRefused => BlockerCategory::BudgetRefused,
        handbook_flow::ResolverBlockerCategory::UnsupportedRequest => {
            BlockerCategory::UnsupportedRequest
        }
    }
}

fn map_subject_ref(subject: handbook_flow::ResolverSubjectRef) -> SubjectRef {
    match subject {
        handbook_flow::ResolverSubjectRef::CanonicalArtifact {
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
        handbook_flow::ResolverSubjectRef::InheritedDependency {
            dependency_id,
            version,
        } => SubjectRef::InheritedDependency {
            dependency_id,
            version,
        },
        handbook_flow::ResolverSubjectRef::Policy { policy_id } => SubjectRef::Policy { policy_id },
    }
}

fn map_next_safe_action(action: handbook_flow::ResolverNextSafeAction) -> crate::NextSafeAction {
    match action {
        handbook_flow::ResolverNextSafeAction::RunSetup => crate::NextSafeAction::RunSetup,
        handbook_flow::ResolverNextSafeAction::RunSetupInit => crate::NextSafeAction::RunSetupInit,
        handbook_flow::ResolverNextSafeAction::RunSetupRefresh => {
            crate::NextSafeAction::RunSetupRefresh
        }
        handbook_flow::ResolverNextSafeAction::RunAuthorCharter => {
            crate::NextSafeAction::RunAuthorCharter
        }
        handbook_flow::ResolverNextSafeAction::RunAuthorProjectContext => {
            crate::NextSafeAction::RunAuthorProjectContext
        }
        handbook_flow::ResolverNextSafeAction::CreateSystemRoot {
            canonical_repo_relative_path,
        } => crate::NextSafeAction::CreateSystemRoot {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::EnsureSystemRootIsDirectory {
            canonical_repo_relative_path,
        } => crate::NextSafeAction::EnsureSystemRootIsDirectory {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::RemoveSystemRootSymlink {
            canonical_repo_relative_path,
        } => crate::NextSafeAction::RemoveSystemRootSymlink {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::CreateCanonicalArtifact {
            canonical_repo_relative_path,
        } => crate::NextSafeAction::CreateCanonicalArtifact {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::FillCanonicalArtifact {
            canonical_repo_relative_path,
        } => crate::NextSafeAction::FillCanonicalArtifact {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        } => crate::NextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::RunGenerate { packet_id } => {
            crate::NextSafeAction::RunGenerate { packet_id }
        }
        handbook_flow::ResolverNextSafeAction::RunDoctor => crate::NextSafeAction::RunDoctor,
    }
}
