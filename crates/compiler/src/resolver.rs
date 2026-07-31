pub use handbook_flow::{PacketSelection, PacketSelectionStatus, ResolveRequest};

use crate::{
    Blocker, BlockerCategory, BudgetOutcome, CompilerError, DecisionLog, PacketResult, Refusal,
    RefusalCategory, SubjectRef,
};

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

pub fn resolve(
    repo_root: impl AsRef<std::path::Path>,
    request: ResolveRequest,
) -> Result<ResolverResult, CompilerError> {
    let result = handbook_flow::resolve(repo_root, request).map_err(CompilerError::Manifest)?;

    Ok(ResolverResult {
        c04_result_version: result.c04_result_version,
        c03_schema_version: result.c03_schema_version,
        c03_manifest_generation_version: result.c03_manifest_generation_version,
        c03_fingerprint_sha256: result.c03_fingerprint_sha256,
        packet_result: result.packet_result,
        decision_log: DecisionLog {
            entries: result.decision_log_entries,
        },
        budget_outcome: result.budget_outcome,
        selection: result.selection,
        refusal: result.refusal.map(map_refusal),
        blockers: result.blockers.into_iter().map(map_blocker).collect(),
    })
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
