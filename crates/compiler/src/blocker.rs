use crate::baseline_validation::{BaselineArtifactValidation, BaselineArtifactVerdict};
use crate::{
    ArtifactIngestIssueKind, ArtifactManifest, CanonicalArtifactKind, NextSafeAction, SubjectRef,
    SystemRootStatus,
};
use serde::Serialize;
use std::cmp::Ordering;

pub const C04_RESULT_VERSION: &str = "reduced-v1-m8.3";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerCategory {
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Blocker {
    pub category: BlockerCategory,
    pub subject: SubjectRef,
    pub summary: String,
    pub next_safe_action: NextSafeAction,
}

pub fn blocker_category_priority(category: BlockerCategory) -> u8 {
    match category {
        BlockerCategory::SystemRootMissing => 0,
        BlockerCategory::SystemRootSymlinkNotAllowed => 1,
        BlockerCategory::SystemRootNotDir => 2,
        BlockerCategory::RequiredArtifactMissing => 3,
        BlockerCategory::RequiredArtifactEmpty => 4,
        BlockerCategory::RequiredArtifactStarterTemplate => 5,
        BlockerCategory::RequiredArtifactInvalid => 6,
        BlockerCategory::ArtifactReadError => 7,
        BlockerCategory::FreshnessInvalid => 8,
        BlockerCategory::BudgetRefused => 9,
        BlockerCategory::UnsupportedRequest => 10,
    }
}

pub(crate) fn author_or_fill_next_safe_action(
    kind: CanonicalArtifactKind,
    canonical_repo_relative_path: &str,
) -> NextSafeAction {
    match kind {
        CanonicalArtifactKind::Charter => NextSafeAction::RunAuthorCharter,
        CanonicalArtifactKind::ProjectContext => NextSafeAction::RunAuthorProjectContext,
        CanonicalArtifactKind::EnvironmentContext => NextSafeAction::FillCanonicalArtifact {
            canonical_repo_relative_path: canonical_repo_relative_path.to_owned(),
        },
        CanonicalArtifactKind::FeatureSpec => NextSafeAction::FillCanonicalArtifact {
            canonical_repo_relative_path: canonical_repo_relative_path.to_owned(),
        },
    }
}

pub(crate) fn required_artifact_blocker(
    category: BlockerCategory,
    summary: String,
    kind: CanonicalArtifactKind,
    canonical_repo_relative_path: &str,
    next_safe_action: NextSafeAction,
) -> Blocker {
    Blocker {
        category,
        subject: SubjectRef::CanonicalArtifact {
            kind,
            canonical_repo_relative_path: canonical_repo_relative_path.to_owned(),
        },
        summary,
        next_safe_action,
    }
}

#[allow(dead_code)]
pub(crate) fn ingest_issue_for_path(
    manifest: &ArtifactManifest,
    canonical_repo_relative_path: &str,
) -> Option<ArtifactIngestIssueKind> {
    manifest
        .ingest_issues
        .iter()
        .find(|issue| issue.canonical_repo_relative_path == canonical_repo_relative_path)
        .map(|issue| issue.kind)
}

pub(crate) fn build_doctor_blockers(
    manifest: &ArtifactManifest,
    baseline_validations: &[BaselineArtifactValidation],
) -> Vec<Blocker> {
    build_baseline_blockers(
        manifest,
        baseline_validations,
        BaselineBlockerScope::AllBaseline,
    )
}

#[allow(dead_code)]
pub(crate) fn build_required_baseline_blockers(
    manifest: &ArtifactManifest,
    baseline_validations: &[BaselineArtifactValidation],
) -> Vec<Blocker> {
    build_baseline_blockers(
        manifest,
        baseline_validations,
        BaselineBlockerScope::RequiredOnly,
    )
}

fn build_baseline_blockers(
    manifest: &ArtifactManifest,
    baseline_validations: &[BaselineArtifactValidation],
    scope: BaselineBlockerScope,
) -> Vec<Blocker> {
    let mut blockers = Vec::new();

    for issue in &manifest.ingest_issues {
        blockers.push(Blocker {
            category: BlockerCategory::ArtifactReadError,
            subject: SubjectRef::CanonicalArtifact {
                kind: issue.artifact_kind,
                canonical_repo_relative_path: issue.canonical_repo_relative_path.clone(),
            },
            summary: match issue.kind {
                ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed => {
                    "canonical artifact path must not be a symlink".to_string()
                }
                ArtifactIngestIssueKind::CanonicalArtifactReadError => {
                    "failed to read canonical artifact".to_string()
                }
            },
            next_safe_action: NextSafeAction::RunSetupRefresh,
        });
    }

    match manifest.system_root_status {
        SystemRootStatus::Ok => {}
        SystemRootStatus::Missing => blockers.push(Blocker {
            category: BlockerCategory::SystemRootMissing,
            subject: SubjectRef::Policy {
                policy_id: "system_root",
            },
            summary: "missing canonical .handbook root".to_string(),
            next_safe_action: NextSafeAction::RunSetup,
        }),
        SystemRootStatus::NotDir => blockers.push(Blocker {
            category: BlockerCategory::SystemRootNotDir,
            subject: SubjectRef::Policy {
                policy_id: "system_root",
            },
            summary: "canonical .handbook root is not a directory".to_string(),
            next_safe_action: NextSafeAction::RunSetup,
        }),
        SystemRootStatus::SymlinkNotAllowed => blockers.push(Blocker {
            category: BlockerCategory::SystemRootSymlinkNotAllowed,
            subject: SubjectRef::Policy {
                policy_id: "system_root",
            },
            summary: "canonical .handbook root must not be a symlink".to_string(),
            next_safe_action: NextSafeAction::RunSetup,
        }),
    }

    if blockers.is_empty() {
        push_baseline_truth_blockers(&mut blockers, baseline_validations, scope);
    }

    sort_blockers(&mut blockers);
    blockers
}

fn push_baseline_truth_blockers(
    blockers: &mut Vec<Blocker>,
    baseline_validations: &[BaselineArtifactValidation],
    scope: BaselineBlockerScope,
) {
    for validation in baseline_validations.iter().filter(|validation| {
        matches!(scope, BaselineBlockerScope::AllBaseline) || validation.packet_required
    }) {
        let blocker = match &validation.verdict {
            BaselineArtifactVerdict::Missing => Some(required_artifact_blocker(
                BlockerCategory::RequiredArtifactMissing,
                "missing required canonical artifact".to_string(),
                validation.kind,
                validation.canonical_repo_relative_path.as_str(),
                NextSafeAction::RunSetupRefresh,
            )),
            BaselineArtifactVerdict::Empty => Some(required_artifact_blocker(
                BlockerCategory::RequiredArtifactEmpty,
                "required canonical artifact is empty".to_string(),
                validation.kind,
                validation.canonical_repo_relative_path.as_str(),
                author_or_fill_next_safe_action(
                    validation.kind,
                    validation.canonical_repo_relative_path.as_str(),
                ),
            )),
            BaselineArtifactVerdict::StarterOwned => Some(required_artifact_blocker(
                BlockerCategory::RequiredArtifactStarterTemplate,
                "required canonical artifact still contains the shipped starter template"
                    .to_string(),
                validation.kind,
                validation.canonical_repo_relative_path.as_str(),
                author_or_fill_next_safe_action(
                    validation.kind,
                    validation.canonical_repo_relative_path.as_str(),
                ),
            )),
            BaselineArtifactVerdict::SemanticallyInvalid { summary } => {
                Some(required_artifact_blocker(
                    BlockerCategory::RequiredArtifactInvalid,
                    format!("required canonical artifact is invalid: {summary}"),
                    validation.kind,
                    validation.canonical_repo_relative_path.as_str(),
                    author_or_fill_next_safe_action(
                        validation.kind,
                        validation.canonical_repo_relative_path.as_str(),
                    ),
                ))
            }
            BaselineArtifactVerdict::IngestInvalid
            | BaselineArtifactVerdict::ValidCanonicalTruth { .. } => None,
        };

        if let Some(blocker) = blocker {
            blockers.push(blocker);
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BaselineBlockerScope {
    RequiredOnly,
    AllBaseline,
}

pub(crate) fn sort_blockers(blockers: &mut [Blocker]) {
    blockers.sort_by(|a, b| {
        let cat = blocker_category_priority(a.category).cmp(&blocker_category_priority(b.category));
        if cat != Ordering::Equal {
            return cat;
        }

        let subj = cmp_subject(&a.subject, &b.subject);
        if subj != Ordering::Equal {
            return subj;
        }

        a.summary.cmp(&b.summary)
    });
}

fn cmp_subject(a: &SubjectRef, b: &SubjectRef) -> Ordering {
    let kind_a = subject_kind_priority(a);
    let kind_b = subject_kind_priority(b);
    let kind_cmp = kind_a.cmp(&kind_b);
    if kind_cmp != Ordering::Equal {
        return kind_cmp;
    }

    match (a, b) {
        (
            SubjectRef::CanonicalArtifact {
                canonical_repo_relative_path: path_a,
                ..
            },
            SubjectRef::CanonicalArtifact {
                canonical_repo_relative_path: path_b,
                ..
            },
        ) => path_a.cmp(path_b),
        (
            SubjectRef::InheritedDependency {
                dependency_id: id_a,
                version: ver_a,
            },
            SubjectRef::InheritedDependency {
                dependency_id: id_b,
                version: ver_b,
            },
        ) => (id_a, ver_a).cmp(&(id_b, ver_b)),
        (SubjectRef::Policy { policy_id: id_a }, SubjectRef::Policy { policy_id: id_b }) => {
            id_a.cmp(id_b)
        }
        _ => Ordering::Equal,
    }
}

fn subject_kind_priority(subject: &SubjectRef) -> u8 {
    match subject {
        SubjectRef::CanonicalArtifact { .. } => 0,
        SubjectRef::InheritedDependency { .. } => 1,
        SubjectRef::Policy { .. } => 2,
    }
}
