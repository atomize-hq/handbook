use crate::budget::{
    evaluate_budget_with_effective_bytes, BudgetByteDomain, BudgetDisposition,
    BudgetEffectiveBytes, BudgetOutcome, BudgetPolicy, NextSafeAction as BudgetNextSafeAction,
};
use crate::packet_result::{
    PacketBodyNote, PacketBodyNoteKind, PacketDecisionSummary, PacketFixtureContext, PacketResult,
    PacketSection, PacketSectionMode, PacketSourceSummary, PacketVariant,
    ReadyPacketNextSafeAction,
};
use handbook_engine::{
    baseline_artifact_validation_for_path, default_canonical_layout_contract,
    ArtifactIngestIssueKind, ArtifactManifest, ArtifactPresence, BaselineArtifactValidation,
    BaselineArtifactVerdict, CanonicalArtifact, CanonicalArtifacts, CanonicalLayoutContract,
    CharterAuthorityTransactionServiceV1, FreshnessIssueKind, FreshnessStatus, ManifestError,
    ManifestInputs, SystemRootStatus,
};
use std::cmp::Ordering;
use std::path::Path;

pub const C04_RESULT_VERSION: &str = "reduced-v1-m8.3";

#[derive(Debug)]
struct CharterAuthorityEvidence {
    promotion_ref: String,
    lifecycle_transition_ref: String,
}

#[derive(Debug, Clone)]
struct RenderedArtifactProjection {
    source_fingerprint: String,
    rendered_bytes: Vec<u8>,
    rendered_output_fingerprint: String,
}

impl RenderedArtifactProjection {
    fn source_fingerprint(&self) -> &str {
        self.source_fingerprint.as_str()
    }

    fn rendered_bytes(&self) -> &[u8] {
        self.rendered_bytes.as_slice()
    }

    fn rendered_byte_length(&self) -> usize {
        self.rendered_bytes.len()
    }

    fn rendered_output_fingerprint(&self) -> &str {
        self.rendered_output_fingerprint.as_str()
    }
}

fn baseline_artifact_validations(
    repo_root: &Path,
    artifacts: &CanonicalArtifacts,
) -> (
    Vec<BaselineArtifactValidation>,
    Vec<Option<RenderedArtifactProjection>>,
    Option<CharterAuthorityEvidence>,
) {
    let mut validations = Vec::with_capacity(artifacts.artifacts.len());
    let mut projections = Vec::with_capacity(artifacts.artifacts.len());
    let mut charter_authority = None;

    for artifact in &artifacts.artifacts {
        let projection =
            artifact
                .rendered_bytes
                .as_ref()
                .map(|rendered_bytes| RenderedArtifactProjection {
                    source_fingerprint: format!(
                        "sha256:{}",
                        artifact
                            .identity
                            .content_sha256
                            .as_deref()
                            .expect("rendered selected artifacts retain a source fingerprint")
                    ),
                    rendered_bytes: rendered_bytes.clone(),
                    rendered_output_fingerprint: artifact
                        .rendered_output_sha256
                        .clone()
                        .expect("rendered selected artifacts retain a rendered fingerprint"),
                });
        let ingest_issue = artifacts
            .ingest_issues
            .iter()
            .find(|issue| issue.canonical_repo_relative_path == artifact.identity.relative_path);
        let verdict = if let Some(issue) = ingest_issue {
            BaselineArtifactVerdict::SemanticallyInvalid {
                summary: match issue.kind {
                    ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed => {
                        "symlink_refused".to_owned()
                    }
                    ArtifactIngestIssueKind::CanonicalArtifactReadError => artifact
                        .render_failure
                        .clone()
                        .unwrap_or_else(|| "repository_read_failed".to_owned()),
                },
            }
        } else {
            match artifact.identity.presence {
                ArtifactPresence::Missing if artifact.identity.packet_required => {
                    BaselineArtifactVerdict::SemanticallyInvalid {
                        summary: "required_path_missing".to_owned(),
                    }
                }
                ArtifactPresence::Missing => BaselineArtifactVerdict::Missing,
                ArtifactPresence::PresentEmpty if artifact.identity.packet_required => {
                    BaselineArtifactVerdict::SemanticallyInvalid {
                        summary: artifact
                            .render_failure
                            .clone()
                            .unwrap_or_else(|| "yaml_syntax_invalid".to_owned()),
                    }
                }
                ArtifactPresence::PresentEmpty => BaselineArtifactVerdict::Empty,
                ArtifactPresence::PresentNonEmpty => {
                    if let Some(summary) = artifact.render_failure.as_ref() {
                        BaselineArtifactVerdict::SemanticallyInvalid {
                            summary: summary.clone(),
                        }
                    } else if artifact.identity.instance_id == "project_authority"
                        && artifact.identity.kind_ref
                            == "handbook.artifact-kind.project-authority@1.1.0"
                    {
                        let selected_fingerprint = projection
                            .as_ref()
                            .map(|projection| projection.source_fingerprint().to_owned());
                        match (
                            selected_fingerprint,
                            CharterAuthorityTransactionServiceV1::new(repo_root)
                                .read_committed_charter(),
                        ) {
                            (Some(selected), Ok(Some(committed)))
                                if committed.canonical_fingerprint == selected =>
                            {
                                charter_authority = Some(CharterAuthorityEvidence {
                                    promotion_ref: committed.promotion_ref,
                                    lifecycle_transition_ref: committed.lifecycle_transition_ref,
                                });
                                BaselineArtifactVerdict::ValidCanonicalTruth {
                                    markdown: String::from_utf8(
                                        projection
                                            .as_ref()
                                            .expect("selected Charter renderer is retained")
                                            .rendered_bytes()
                                            .to_vec(),
                                    )
                                    .expect("fixed Charter renderer emits UTF-8"),
                                }
                            }
                            (Some(selected), Ok(Some(committed))) => {
                                BaselineArtifactVerdict::SemanticallyInvalid {
                                    summary: format!(
                                        "selected_charter_authority_fingerprint_mismatch selected={selected} committed={}",
                                        committed.canonical_fingerprint
                                    ),
                                }
                            }
                            (_, Ok(None)) => BaselineArtifactVerdict::SemanticallyInvalid {
                                summary: "committed_charter_authority_missing".to_owned(),
                            },
                            (_, Err(error)) => BaselineArtifactVerdict::SemanticallyInvalid {
                                summary: format!(
                                    "committed_charter_authority_read_{:?}",
                                    error.kind()
                                )
                                .to_lowercase(),
                            },
                            (None, Ok(Some(_))) => BaselineArtifactVerdict::SemanticallyInvalid {
                                summary: "selected_charter_renderer_unavailable".to_owned(),
                            },
                        }
                    } else if let Some(projection) = projection.as_ref() {
                        BaselineArtifactVerdict::ValidCanonicalTruth {
                            markdown: String::from_utf8(projection.rendered_bytes().to_vec())
                                .expect("fixed selected renderers emit UTF-8"),
                        }
                    } else {
                        match artifact
                            .bytes
                            .as_ref()
                            .map(|bytes| String::from_utf8(bytes.clone()))
                        {
                            Some(Ok(contents)) => {
                                BaselineArtifactVerdict::ValidCanonicalTruth { markdown: contents }
                            }
                            _ => BaselineArtifactVerdict::SemanticallyInvalid {
                                summary: "canonical artifact bytes are not valid UTF-8".to_owned(),
                            },
                        }
                    }
                }
            }
        };

        validations.push(BaselineArtifactValidation {
            instance_id: artifact.identity.instance_id.clone(),
            kind_ref: artifact.identity.kind_ref.clone(),
            label: artifact.identity.label.clone(),
            canonical_repo_relative_path: artifact.identity.relative_path.clone(),
            packet_required: artifact.identity.packet_required,
            verdict,
        });
        projections.push(projection);
    }

    (validations, projections, charter_authority)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolverRefusalCategory {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolverSubjectRef {
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
        policy_id: &'static str,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolverNextSafeAction {
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
        packet_id: &'static str,
    },
    RunDoctor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolverRefusal {
    pub category: ResolverRefusalCategory,
    pub summary: String,
    pub broken_subject: ResolverSubjectRef,
    pub next_safe_action: ResolverNextSafeAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolverBlockerCategory {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolverBlocker {
    pub category: ResolverBlockerCategory,
    pub subject: ResolverSubjectRef,
    pub summary: String,
    pub next_safe_action: ResolverNextSafeAction,
}

fn blocker_category_priority(category: ResolverBlockerCategory) -> u8 {
    match category {
        ResolverBlockerCategory::SystemRootMissing => 0,
        ResolverBlockerCategory::SystemRootSymlinkNotAllowed => 1,
        ResolverBlockerCategory::SystemRootNotDir => 2,
        ResolverBlockerCategory::RequiredArtifactMissing => 3,
        ResolverBlockerCategory::RequiredArtifactEmpty => 4,
        ResolverBlockerCategory::RequiredArtifactStarterTemplate => 5,
        ResolverBlockerCategory::RequiredArtifactInvalid => 6,
        ResolverBlockerCategory::ArtifactReadError => 7,
        ResolverBlockerCategory::FreshnessInvalid => 8,
        ResolverBlockerCategory::BudgetRefused => 9,
        ResolverBlockerCategory::UnsupportedRequest => 10,
    }
}

fn author_or_fill_next_safe_action(
    instance_id: &str,
    kind_ref: &str,
    canonical_repo_relative_path: &str,
) -> ResolverNextSafeAction {
    match (instance_id, kind_ref) {
        ("project_authority", "handbook.artifact-kind.project-authority@1.1.0") => {
            ResolverNextSafeAction::RunAuthorCharter
        }
        ("project_context", "handbook.artifact-kind.project-context@1.1.0") => {
            ResolverNextSafeAction::RunAuthorProjectContext
        }
        ("environment_context", "handbook.artifact-kind.environment-context@1.1.0") => {
            ResolverNextSafeAction::CreateCanonicalArtifact {
                canonical_repo_relative_path: canonical_repo_relative_path.to_owned(),
            }
        }
        _ => ResolverNextSafeAction::FillCanonicalArtifact {
            canonical_repo_relative_path: canonical_repo_relative_path.to_owned(),
        },
    }
}

fn required_artifact_blocker(
    category: ResolverBlockerCategory,
    summary: String,
    instance_id: &str,
    kind_ref: &str,
    label: &str,
    canonical_repo_relative_path: &str,
    next_safe_action: ResolverNextSafeAction,
) -> ResolverBlocker {
    ResolverBlocker {
        category,
        subject: ResolverSubjectRef::CanonicalArtifact {
            instance_id: instance_id.to_owned(),
            kind_ref: kind_ref.to_owned(),
            label: label.to_owned(),
            canonical_repo_relative_path: canonical_repo_relative_path.to_owned(),
        },
        summary,
        next_safe_action,
    }
}

fn ingest_issue_for_path(
    manifest: &ArtifactManifest,
    canonical_repo_relative_path: &str,
) -> Option<ArtifactIngestIssueKind> {
    manifest
        .ingest_issues
        .iter()
        .find(|issue| issue.canonical_repo_relative_path == canonical_repo_relative_path)
        .map(|issue| issue.kind)
}

fn build_required_baseline_blockers(
    manifest: &ArtifactManifest,
    baseline_validations: &[BaselineArtifactValidation],
    contract: CanonicalLayoutContract,
) -> Vec<ResolverBlocker> {
    build_baseline_blockers(
        manifest,
        baseline_validations,
        BaselineBlockerScope::RequiredOnly,
        contract,
    )
}

fn build_baseline_blockers(
    manifest: &ArtifactManifest,
    baseline_validations: &[BaselineArtifactValidation],
    scope: BaselineBlockerScope,
    contract: CanonicalLayoutContract,
) -> Vec<ResolverBlocker> {
    let mut blockers = Vec::new();

    for issue in &manifest.ingest_issues {
        if manifest.artifacts.iter().any(|artifact| {
            artifact.instance_id == "environment_context"
                && artifact.relative_path == issue.canonical_repo_relative_path
                && !artifact.packet_required
        }) {
            continue;
        }
        let detail = baseline_artifact_validation_for_path(
            baseline_validations,
            issue.canonical_repo_relative_path.as_str(),
        )
        .and_then(|validation| match &validation.verdict {
            BaselineArtifactVerdict::SemanticallyInvalid { summary } => Some(summary.as_str()),
            _ => None,
        });
        blockers.push(ResolverBlocker {
            category: ResolverBlockerCategory::ArtifactReadError,
            subject: ResolverSubjectRef::CanonicalArtifact {
                instance_id: issue.instance_id.clone(),
                kind_ref: issue.kind_ref.clone(),
                label: issue.label.clone(),
                canonical_repo_relative_path: issue.canonical_repo_relative_path.clone(),
            },
            summary: match issue.kind {
                ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed => {
                    "canonical artifact path must not be a symlink".to_string()
                }
                ArtifactIngestIssueKind::CanonicalArtifactReadError => {
                    format!(
                        "failed to read canonical artifact: {}",
                        detail.unwrap_or("repository_read_failed")
                    )
                }
            },
            next_safe_action: ResolverNextSafeAction::RunSetupRefresh,
        });
    }

    match manifest.system_root_status {
        SystemRootStatus::Ok => {}
        SystemRootStatus::Missing => blockers.push(ResolverBlocker {
            category: ResolverBlockerCategory::SystemRootMissing,
            subject: ResolverSubjectRef::Policy {
                policy_id: "system_root",
            },
            summary: system_root_summary(contract, SystemRootStatus::Missing),
            next_safe_action: ResolverNextSafeAction::RunSetup,
        }),
        SystemRootStatus::NotDir => blockers.push(ResolverBlocker {
            category: ResolverBlockerCategory::SystemRootNotDir,
            subject: ResolverSubjectRef::Policy {
                policy_id: "system_root",
            },
            summary: system_root_summary(contract, SystemRootStatus::NotDir),
            next_safe_action: ResolverNextSafeAction::RunSetup,
        }),
        SystemRootStatus::SymlinkNotAllowed => blockers.push(ResolverBlocker {
            category: ResolverBlockerCategory::SystemRootSymlinkNotAllowed,
            subject: ResolverSubjectRef::Policy {
                policy_id: "system_root",
            },
            summary: system_root_summary(contract, SystemRootStatus::SymlinkNotAllowed),
            next_safe_action: ResolverNextSafeAction::RunSetup,
        }),
    }

    if blockers.is_empty() {
        push_baseline_truth_blockers(&mut blockers, baseline_validations, scope);
    }

    sort_blockers(&mut blockers);
    blockers
}

#[cfg(test)]
mod tests {
    use super::*;

    fn missing_validation(
        instance_id: &str,
        kind_ref: &str,
        canonical_repo_relative_path: &str,
    ) -> BaselineArtifactValidation {
        BaselineArtifactValidation {
            instance_id: instance_id.to_owned(),
            kind_ref: kind_ref.to_owned(),
            label: instance_id.to_owned(),
            canonical_repo_relative_path: canonical_repo_relative_path.to_owned(),
            packet_required: true,
            verdict: BaselineArtifactVerdict::Missing,
        }
    }

    #[test]
    fn missing_baseline_truth_uses_instance_aware_author_actions() {
        let validations = vec![
            missing_validation(
                "project_authority",
                "handbook.artifact-kind.project-authority@1.1.0",
                ".handbook/project/charter.yaml",
            ),
            missing_validation(
                "project_context",
                "handbook.artifact-kind.project-context@1.1.0",
                ".handbook/project/context.yaml",
            ),
        ];
        let mut blockers = Vec::new();

        push_baseline_truth_blockers(
            &mut blockers,
            &validations,
            BaselineBlockerScope::AllBaseline,
        );
        let refusal = refusal_for_required_baseline_truth(&validations).expect("refusal");

        assert_eq!(blockers.len(), 2);
        assert_eq!(
            blockers[0].next_safe_action,
            ResolverNextSafeAction::RunAuthorCharter
        );
        assert_eq!(
            blockers[1].next_safe_action,
            ResolverNextSafeAction::RunAuthorProjectContext
        );
        assert_eq!(
            refusal.next_safe_action,
            ResolverNextSafeAction::RunAuthorCharter
        );
    }
}

fn push_baseline_truth_blockers(
    blockers: &mut Vec<ResolverBlocker>,
    baseline_validations: &[BaselineArtifactValidation],
    scope: BaselineBlockerScope,
) {
    for validation in baseline_validations.iter().filter(|validation| {
        matches!(scope, BaselineBlockerScope::AllBaseline) || validation.packet_required
    }) {
        let blocker = match &validation.verdict {
            BaselineArtifactVerdict::Missing => Some(required_artifact_blocker(
                ResolverBlockerCategory::RequiredArtifactMissing,
                "missing required canonical artifact".to_string(),
                validation.instance_id.as_str(),
                validation.kind_ref.as_str(),
                validation.label.as_str(),
                validation.canonical_repo_relative_path.as_str(),
                author_or_fill_next_safe_action(
                    validation.instance_id.as_str(),
                    validation.kind_ref.as_str(),
                    validation.canonical_repo_relative_path.as_str(),
                ),
            )),
            BaselineArtifactVerdict::Empty => Some(required_artifact_blocker(
                ResolverBlockerCategory::RequiredArtifactEmpty,
                "required canonical artifact is empty".to_string(),
                validation.instance_id.as_str(),
                validation.kind_ref.as_str(),
                validation.label.as_str(),
                validation.canonical_repo_relative_path.as_str(),
                author_or_fill_next_safe_action(
                    validation.instance_id.as_str(),
                    validation.kind_ref.as_str(),
                    validation.canonical_repo_relative_path.as_str(),
                ),
            )),
            BaselineArtifactVerdict::StarterOwned => Some(required_artifact_blocker(
                ResolverBlockerCategory::RequiredArtifactStarterTemplate,
                "required canonical artifact still contains the shipped starter template"
                    .to_string(),
                validation.instance_id.as_str(),
                validation.kind_ref.as_str(),
                validation.label.as_str(),
                validation.canonical_repo_relative_path.as_str(),
                author_or_fill_next_safe_action(
                    validation.instance_id.as_str(),
                    validation.kind_ref.as_str(),
                    validation.canonical_repo_relative_path.as_str(),
                ),
            )),
            BaselineArtifactVerdict::SemanticallyInvalid { summary } => {
                Some(required_artifact_blocker(
                    ResolverBlockerCategory::RequiredArtifactInvalid,
                    format!("required canonical artifact is invalid: {summary}"),
                    validation.instance_id.as_str(),
                    validation.kind_ref.as_str(),
                    validation.label.as_str(),
                    validation.canonical_repo_relative_path.as_str(),
                    author_or_fill_next_safe_action(
                        validation.instance_id.as_str(),
                        validation.kind_ref.as_str(),
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

fn sort_blockers(blockers: &mut [ResolverBlocker]) {
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

fn cmp_subject(a: &ResolverSubjectRef, b: &ResolverSubjectRef) -> Ordering {
    let kind_a = subject_kind_priority(a);
    let kind_b = subject_kind_priority(b);
    let kind_cmp = kind_a.cmp(&kind_b);
    if kind_cmp != Ordering::Equal {
        return kind_cmp;
    }

    match (a, b) {
        (
            ResolverSubjectRef::CanonicalArtifact {
                canonical_repo_relative_path: path_a,
                ..
            },
            ResolverSubjectRef::CanonicalArtifact {
                canonical_repo_relative_path: path_b,
                ..
            },
        ) => path_a.cmp(path_b),
        (
            ResolverSubjectRef::InheritedDependency {
                dependency_id: id_a,
                version: ver_a,
            },
            ResolverSubjectRef::InheritedDependency {
                dependency_id: id_b,
                version: ver_b,
            },
        ) => (id_a, ver_a).cmp(&(id_b, ver_b)),
        (
            ResolverSubjectRef::Policy { policy_id: id_a },
            ResolverSubjectRef::Policy { policy_id: id_b },
        ) => id_a.cmp(id_b),
        _ => Ordering::Equal,
    }
}

fn subject_kind_priority(subject: &ResolverSubjectRef) -> u8 {
    match subject {
        ResolverSubjectRef::CanonicalArtifact { .. } => 0,
        ResolverSubjectRef::InheritedDependency { .. } => 1,
        ResolverSubjectRef::Policy { .. } => 2,
    }
}

const DEFAULT_PACKET_ID: &str = "planning.packet";
const DEMO_EXECUTION_PACKET_ID: &str = "execution.demo.packet";
const LIVE_EXECUTION_PACKET_ID: &str = "execution.live.packet";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolveRequest<'a> {
    pub budget_policy: BudgetPolicy,
    pub packet_id: &'a str,
}

impl Default for ResolveRequest<'_> {
    fn default() -> Self {
        Self {
            budget_policy: BudgetPolicy::default(),
            packet_id: DEFAULT_PACKET_ID,
        }
    }
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolverResult {
    pub c04_result_version: String,
    pub c03_schema_version: String,
    pub c03_manifest_generation_version: u32,
    pub c03_fingerprint_sha256: String,
    pub packet_result: PacketResult,
    pub decision_log_entries: Vec<String>,
    pub budget_outcome: BudgetOutcome,
    pub selection: PacketSelection,
    pub refusal: Option<ResolverRefusal>,
    pub blockers: Vec<ResolverBlocker>,
}

pub fn resolve(
    repo_root: impl AsRef<Path>,
    request: ResolveRequest<'_>,
) -> Result<ResolverResult, ManifestError> {
    resolve_with_contract(repo_root, request, *default_canonical_layout_contract())
}

pub fn resolve_with_contract(
    repo_root: impl AsRef<Path>,
    request: ResolveRequest<'_>,
    contract: CanonicalLayoutContract,
) -> Result<ResolverResult, ManifestError> {
    let repo_root = repo_root.as_ref();
    let canonical_artifacts = CanonicalArtifacts::load_with_contract(repo_root, contract)
        .map_err(ManifestError::Ingest)?;
    let (baseline_validations, rendered_projections, charter_authority) =
        baseline_artifact_validations(repo_root, &canonical_artifacts);
    let packet_artifact_inputs = PacketArtifactInputs {
        canonical_artifacts: &canonical_artifacts,
        rendered_projections: &rendered_projections,
    };

    let manifest =
        ArtifactManifest::from_canonical_artifacts(&canonical_artifacts, ManifestInputs::default());

    let mut decision_log_entries = Vec::new();

    decision_log_entries.push(format!(
        "c03.handbook_root status={:?}",
        manifest.system_root_status
    ));

    decision_log_entries.push(format!(
        "c03.provenance schema_version={} manifest_generation_version={} fingerprint_sha256={}",
        manifest.version.schema.version,
        manifest.version.generation,
        manifest.freshness.fingerprint_sha256
    ));

    for (artifact, projection) in manifest.artifacts.iter().zip(&rendered_projections) {
        decision_log_entries.push(format!(
            "c03.artifact kind={} required={} presence={:?} byte_len={:?} sha256={:?} path={}",
            artifact.label,
            artifact.packet_required,
            artifact.presence,
            artifact.byte_len,
            artifact.content_sha256.as_deref(),
            artifact.relative_path
        ));
        if let Some(projection) = projection {
            decision_log_entries.push(format!(
                "hcm2.selected instance_id={} advisory={} rendered_byte_len={} rendered_sha256={} media_type=text/markdown",
                artifact.instance_id,
                artifact.instance_id == "environment_context"
                    && artifact.kind_ref
                        == "handbook.artifact-kind.environment-context@1.1.0",
                projection.rendered_byte_length(),
                projection.rendered_output_fingerprint()
            ));
        }
    }
    if let Some(authority) = charter_authority.as_ref() {
        decision_log_entries.push(format!(
            "hcm2.charter.authority promotion_ref={} lifecycle_transition_ref={}",
            authority.promotion_ref, authority.lifecycle_transition_ref
        ));
    }

    for issue in &manifest.ingest_issues {
        decision_log_entries.push(format!(
            "c03.ingest.issue kind={:?} required={} path={}",
            issue.kind, issue.packet_required, issue.canonical_repo_relative_path
        ));
    }
    for validation in &baseline_validations {
        decision_log_entries.push(format!(
            "c04.baseline.validation kind={} required={} verdict={} path={} detail={}",
            validation.label,
            validation.packet_required,
            baseline_verdict_label(&validation.verdict),
            validation.canonical_repo_relative_path,
            baseline_verdict_detail(&validation.verdict)
        ));
    }

    decision_log_entries.push(format!(
        "c03.freshness status={:?} issue_count={}",
        manifest.freshness.status,
        manifest.freshness.issues.len()
    ));
    for issue in &manifest.freshness.issues {
        decision_log_entries.push(format!(
            "c03.freshness.issue kind={:?} detail={}",
            issue.kind, issue.detail
        ));
    }

    let budget_effective_bytes = canonical_artifacts
        .artifacts
        .iter()
        .zip(&rendered_projections)
        .zip(&baseline_validations)
        .filter_map(|((artifact, projection), validation)| {
            (matches!(
                validation.verdict,
                BaselineArtifactVerdict::ValidCanonicalTruth { .. }
            ))
            .then_some(projection.as_ref())
            .flatten()
            .map(|projection| BudgetEffectiveBytes {
                canonical_repo_relative_path: artifact.identity.relative_path.clone(),
                byte_len: projection.rendered_byte_length() as u64,
                byte_domain: BudgetByteDomain::RenderedOutput,
            })
        })
        .collect::<Vec<_>>();
    let budget_outcome = evaluate_budget_with_effective_bytes(
        &manifest.artifacts,
        &budget_effective_bytes,
        request.budget_policy,
    );
    let packet_artifact_plans = packet_artifact_plans_for(
        &manifest,
        &baseline_validations,
        &budget_outcome,
        packet_artifact_inputs,
    );
    decision_log_entries.push(format!(
        "budget disposition={:?} reason={:?} targets={} next_safe_action={}",
        budget_outcome.disposition,
        budget_outcome.reason,
        budget_outcome.targets.len(),
        budget_outcome.next_safe_action.is_some()
    ));
    for plan in &packet_artifact_plans {
        if !plan.artifact.identity.packet_required
            && matches!(plan.disposition, PacketArtifactDisposition::OmittedInvalid)
        {
            decision_log_entries.push(format!(
                "packet.optional.invalid_omitted path={} detail=invalid canonical truth was not ingested",
                plan.artifact.identity.relative_path
            ));
        }
    }

    let refusal = compute_refusal(
        &manifest,
        &baseline_validations,
        &budget_outcome,
        &request,
        contract,
    );
    if let Some(refusal) = &refusal {
        decision_log_entries.push(format!(
            "refusal category={:?} broken_subject={:?} next_safe_action={:?}",
            refusal.category, refusal.broken_subject, refusal.next_safe_action
        ));
    }

    let blockers = compute_blockers(
        &manifest,
        &baseline_validations,
        &budget_outcome,
        &request,
        contract,
    );
    for blocker in &blockers {
        decision_log_entries.push(format!(
            "blocker category={:?} subject={:?} next_safe_action={:?}",
            blocker.category, blocker.subject, blocker.next_safe_action
        ));
    }

    let selection_status = if manifest.freshness.status == FreshnessStatus::Ok
        && budget_outcome.disposition != BudgetDisposition::Refuse
        && refusal.is_none()
        && blockers.is_empty()
    {
        PacketSelectionStatus::Selected
    } else {
        PacketSelectionStatus::Blocked
    };

    decision_log_entries.push(format!(
        "selection packet_id={} status={:?}",
        request.packet_id, selection_status
    ));

    let packet_result = build_packet_result(BuildPacketResultInput {
        repo_root,
        contract,
        request: &request,
        packet_artifact_inputs,
        manifest: &manifest,
        packet_artifact_plans: &packet_artifact_plans,
        baseline_validations: &baseline_validations,
        budget_outcome: &budget_outcome,
        selection_status,
        refusal: refusal.as_ref(),
        blockers: &blockers,
        decision_log_entries: decision_log_entries.len(),
    });

    Ok(ResolverResult {
        c04_result_version: C04_RESULT_VERSION.to_string(),
        c03_schema_version: manifest.version.schema.version.to_string(),
        c03_manifest_generation_version: manifest.version.generation,
        c03_fingerprint_sha256: manifest.freshness.fingerprint_sha256.clone(),
        packet_result,
        decision_log_entries,
        budget_outcome,
        selection: PacketSelection {
            packet_id: request.packet_id.to_string(),
            status: selection_status,
        },
        refusal,
        blockers,
    })
}

struct BuildPacketResultInput<'a, 'request> {
    repo_root: &'a Path,
    contract: CanonicalLayoutContract,
    request: &'a ResolveRequest<'request>,
    packet_artifact_inputs: PacketArtifactInputs<'a>,
    manifest: &'a ArtifactManifest,
    packet_artifact_plans: &'a [PacketArtifactPlan<'a>],
    baseline_validations: &'a [BaselineArtifactValidation],
    budget_outcome: &'a BudgetOutcome,
    selection_status: PacketSelectionStatus,
    refusal: Option<&'a ResolverRefusal>,
    blockers: &'a [ResolverBlocker],
    decision_log_entries: usize,
}

fn build_packet_result(input: BuildPacketResultInput<'_, '_>) -> PacketResult {
    let BuildPacketResultInput {
        repo_root,
        contract,
        request,
        packet_artifact_inputs,
        manifest,
        packet_artifact_plans,
        baseline_validations,
        budget_outcome,
        selection_status,
        refusal,
        blockers,
        decision_log_entries,
    } = input;

    let variant = packet_variant_for(request.packet_id);
    let included_sources = included_sources_for(packet_artifact_plans);
    let packet_body_ready = selection_status == PacketSelectionStatus::Selected
        && refusal.is_none()
        && blockers.is_empty();
    let notes = packet_notes_for(
        manifest,
        budget_outcome,
        packet_artifact_plans,
        packet_body_ready,
    );
    let sections = if packet_body_ready {
        packet_sections_for(packet_artifact_plans)
    } else {
        Vec::new()
    };
    let fixture_context = fixture_context_for(
        repo_root,
        request.packet_id,
        packet_artifact_inputs,
        baseline_validations,
        contract,
    );

    let summary_line = if selection_status == PacketSelectionStatus::Selected {
        let fixture_suffix = fixture_context
            .as_ref()
            .map(|context| format!(" (fixture set {})", context.fixture_set_id))
            .unwrap_or_default();
        format!(
            "READY {}{}: {} included sources, budget {:?}/{:?}, {} decision log entries",
            request.packet_id,
            fixture_suffix,
            included_sources.len(),
            budget_outcome.disposition,
            budget_outcome.reason,
            decision_log_entries
        )
    } else if let Some(refusal) = refusal {
        format!(
            "REFUSED {}: category={:?}, blockers={}, budget {:?}/{:?}, {} decision log entries",
            request.packet_id,
            refusal.category,
            blockers.len(),
            budget_outcome.disposition,
            budget_outcome.reason,
            decision_log_entries
        )
    } else {
        format!(
            "BLOCKED {}: blockers={}, budget {:?}/{:?}, {} decision log entries",
            request.packet_id,
            blockers.len(),
            budget_outcome.disposition,
            budget_outcome.reason,
            decision_log_entries
        )
    };

    let ready_next_safe_action = next_safe_action_for_ready_packet(variant);

    PacketResult {
        packet_id: request.packet_id.to_string(),
        variant,
        fixture_context,
        included_sources,
        notes,
        decision_summary: PacketDecisionSummary {
            packet_status: selection_status,
            budget_disposition: budget_outcome.disposition,
            budget_reason: budget_outcome.reason.clone(),
            decision_log_entries,
            summary_line,
            ready_next_safe_action,
        },
        sections,
    }
}

fn packet_variant_for(packet_id: &str) -> PacketVariant {
    if packet_id == DEMO_EXECUTION_PACKET_ID {
        PacketVariant::ExecutionDemo
    } else if packet_id == LIVE_EXECUTION_PACKET_ID {
        PacketVariant::ExecutionLive
    } else {
        PacketVariant::Planning
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PacketArtifactDisposition {
    BlockedIngest,
    OmittedMissing,
    OmittedEmpty,
    OmittedStarterTemplate,
    OmittedInvalid,
    IncludedVerbatim,
    IncludedSummary,
    ExcludedDueToBudget,
}

#[derive(Debug, Clone)]
struct PacketArtifactPlan<'a> {
    artifact: &'a CanonicalArtifact,
    title: String,
    disposition: PacketArtifactDisposition,
    rendered_projection: Option<&'a RenderedArtifactProjection>,
}

#[derive(Clone, Copy)]
struct PacketArtifactInputs<'a> {
    canonical_artifacts: &'a CanonicalArtifacts,
    rendered_projections: &'a [Option<RenderedArtifactProjection>],
}

fn packet_artifact_plans_for<'a>(
    manifest: &ArtifactManifest,
    baseline_validations: &[BaselineArtifactValidation],
    budget_outcome: &BudgetOutcome,
    inputs: PacketArtifactInputs<'a>,
) -> Vec<PacketArtifactPlan<'a>> {
    inputs
        .canonical_artifacts
        .artifacts
        .iter()
        .zip(inputs.rendered_projections)
        .map(|(artifact, rendered_projection)| {
            let disposition = packet_artifact_disposition_for(
                manifest,
                baseline_validations,
                artifact,
                budget_outcome,
            );
            let rendered_projection = (disposition == PacketArtifactDisposition::IncludedVerbatim)
                .then_some(rendered_projection.as_ref())
                .flatten();
            PacketArtifactPlan {
                artifact,
                title: artifact
                    .identity
                    .label
                    .to_ascii_uppercase()
                    .replace(' ', "_"),
                disposition,
                rendered_projection,
            }
        })
        .collect()
}

fn packet_artifact_disposition_for(
    manifest: &ArtifactManifest,
    baseline_validations: &[BaselineArtifactValidation],
    artifact: &CanonicalArtifact,
    budget_outcome: &BudgetOutcome,
) -> PacketArtifactDisposition {
    if let Some(validation) = baseline_artifact_validation_for_path(
        baseline_validations,
        artifact.identity.relative_path.as_str(),
    ) {
        match &validation.verdict {
            BaselineArtifactVerdict::IngestInvalid => {
                return PacketArtifactDisposition::BlockedIngest;
            }
            BaselineArtifactVerdict::SemanticallyInvalid { .. } => {
                return PacketArtifactDisposition::OmittedInvalid;
            }
            BaselineArtifactVerdict::Missing
            | BaselineArtifactVerdict::Empty
            | BaselineArtifactVerdict::StarterOwned
            | BaselineArtifactVerdict::ValidCanonicalTruth { .. } => {}
        }
    }

    if ingest_issue_for_path(manifest, artifact.identity.relative_path.as_str()).is_some() {
        return PacketArtifactDisposition::BlockedIngest;
    }

    match artifact.identity.presence {
        ArtifactPresence::Missing => PacketArtifactDisposition::OmittedMissing,
        ArtifactPresence::PresentEmpty => PacketArtifactDisposition::OmittedEmpty,
        ArtifactPresence::PresentNonEmpty if artifact.identity.matches_setup_starter_template => {
            PacketArtifactDisposition::OmittedStarterTemplate
        }
        ArtifactPresence::PresentNonEmpty => {
            if should_exclude_artifact(artifact.identity.relative_path.as_str(), budget_outcome) {
                PacketArtifactDisposition::ExcludedDueToBudget
            } else if should_summarize_artifact(
                artifact.identity.relative_path.as_str(),
                budget_outcome,
            ) {
                PacketArtifactDisposition::IncludedSummary
            } else {
                PacketArtifactDisposition::IncludedVerbatim
            }
        }
    }
}

fn included_sources_for(plans: &[PacketArtifactPlan<'_>]) -> Vec<PacketSourceSummary> {
    plans
        .iter()
        .filter_map(|plan| {
            if !matches!(
                plan.disposition,
                PacketArtifactDisposition::IncludedVerbatim
                    | PacketArtifactDisposition::IncludedSummary
            ) {
                return None;
            }

            let projection = plan.rendered_projection;
            Some(PacketSourceSummary {
                instance_id: plan.artifact.identity.instance_id.clone(),
                kind_ref: plan.artifact.identity.kind_ref.clone(),
                label: plan.artifact.identity.label.clone(),
                canonical_repo_relative_path: plan.artifact.identity.relative_path.clone(),
                required: plan.artifact.identity.packet_required,
                presence: plan.artifact.identity.presence,
                byte_len: plan.artifact.identity.byte_len,
                content_sha256: plan.artifact.identity.content_sha256.clone(),
                rendered_output_byte_len: projection
                    .map(|projection| projection.rendered_byte_length() as u64),
                rendered_output_sha256: projection
                    .map(|projection| projection.rendered_output_fingerprint().to_owned()),
                rendered_media_type: projection.map(|_| "text/markdown".to_owned()),
            })
        })
        .collect()
}

fn present_fixture_sources_for(
    inputs: PacketArtifactInputs<'_>,
    baseline_validations: &[BaselineArtifactValidation],
) -> Vec<PacketSourceSummary> {
    inputs
        .canonical_artifacts
        .artifacts
        .iter()
        .zip(inputs.rendered_projections)
        .filter_map(|(artifact, projection)| {
            let identity = &artifact.identity;
            if let Some(validation) = baseline_artifact_validation_for_path(
                baseline_validations,
                identity.relative_path.as_str(),
            ) {
                if matches!(
                    validation.verdict,
                    BaselineArtifactVerdict::IngestInvalid
                        | BaselineArtifactVerdict::SemanticallyInvalid { .. }
                ) {
                    return None;
                }
            }

            match identity.presence {
                ArtifactPresence::Missing => return None,
                ArtifactPresence::PresentEmpty if !identity.packet_required => return None,
                ArtifactPresence::PresentNonEmpty
                    if !identity.packet_required && identity.matches_setup_starter_template =>
                {
                    return None;
                }
                ArtifactPresence::PresentEmpty | ArtifactPresence::PresentNonEmpty => {}
            }

            Some(PacketSourceSummary {
                instance_id: identity.instance_id.clone(),
                kind_ref: identity.kind_ref.clone(),
                label: identity.label.clone(),
                canonical_repo_relative_path: identity.relative_path.clone(),
                required: identity.packet_required,
                presence: identity.presence,
                byte_len: identity.byte_len,
                content_sha256: identity.content_sha256.clone(),
                rendered_output_byte_len: projection
                    .as_ref()
                    .map(|projection| projection.rendered_byte_length() as u64),
                rendered_output_sha256: projection
                    .as_ref()
                    .map(|projection| projection.rendered_output_fingerprint().to_owned()),
                rendered_media_type: projection.as_ref().map(|_| "text/markdown".to_owned()),
            })
        })
        .collect()
}

fn packet_notes_for(
    manifest: &ArtifactManifest,
    budget_outcome: &BudgetOutcome,
    plans: &[PacketArtifactPlan<'_>],
    packet_body_ready: bool,
) -> Vec<PacketBodyNote> {
    let mut notes = Vec::new();
    push_packet_artifact_notes(&mut notes, plans, budget_outcome);

    if !packet_body_ready {
        notes.push(PacketBodyNote {
            kind: PacketBodyNoteKind::Omission,
            text: "packet body omitted because request is not ready".to_string(),
        });
    }

    let budget_text = format!(
        "budget: {:?}/{:?} across {} tracked canonical artifacts",
        budget_outcome.disposition,
        budget_outcome.reason,
        manifest.artifacts.len()
    );
    notes.push(PacketBodyNote {
        kind: PacketBodyNoteKind::Budget,
        text: budget_text,
    });

    notes
}

fn push_packet_artifact_notes(
    notes: &mut Vec<PacketBodyNote>,
    plans: &[PacketArtifactPlan<'_>],
    budget_outcome: &BudgetOutcome,
) {
    for plan in plans {
        if plan.artifact.identity.packet_required {
            continue;
        }

        let text = match plan.disposition {
            PacketArtifactDisposition::OmittedMissing => Some(format!(
                "optional source omitted: {}",
                plan.artifact.identity.relative_path
            )),
            PacketArtifactDisposition::OmittedEmpty => Some(format!(
                "optional source omitted: {} (empty)",
                plan.artifact.identity.relative_path
            )),
            PacketArtifactDisposition::OmittedStarterTemplate => Some(format!(
                "optional source omitted: {} (shipped starter template)",
                plan.artifact.identity.relative_path
            )),
            PacketArtifactDisposition::OmittedInvalid => Some(format!(
                "optional source omitted: {} (invalid canonical truth)",
                plan.artifact.identity.relative_path
            )),
            PacketArtifactDisposition::IncludedSummary => Some(format!(
                "optional source summarized due to budget: {}{}",
                plan.artifact.identity.relative_path,
                budget_target_note_suffix(
                    plan.artifact.identity.relative_path.as_str(),
                    budget_outcome,
                )
            )),
            PacketArtifactDisposition::ExcludedDueToBudget => Some(format!(
                "optional source excluded due to budget: {}{}",
                plan.artifact.identity.relative_path,
                budget_target_note_suffix(
                    plan.artifact.identity.relative_path.as_str(),
                    budget_outcome,
                )
            )),
            PacketArtifactDisposition::BlockedIngest
            | PacketArtifactDisposition::IncludedVerbatim => None,
        };

        if let Some(text) = text {
            notes.push(PacketBodyNote {
                kind: match plan.disposition {
                    PacketArtifactDisposition::IncludedSummary => PacketBodyNoteKind::Budget,
                    _ => PacketBodyNoteKind::Omission,
                },
                text,
            });
        }
    }
}

fn budget_target_note_suffix(path: &str, budget_outcome: &BudgetOutcome) -> String {
    budget_outcome
        .targets
        .iter()
        .find(|target| target.canonical_repo_relative_path == path)
        .map(|target| {
            let domain = match target.byte_domain {
                BudgetByteDomain::Source => "source",
                BudgetByteDomain::RenderedOutput => "rendered_output",
            };
            format!(" ({} bytes [{domain}])", target.byte_len)
        })
        .unwrap_or_default()
}

fn packet_sections_for(plans: &[PacketArtifactPlan<'_>]) -> Vec<PacketSection> {
    plans
        .iter()
        .filter_map(|plan| {
            let (mode, contents) = if let Some(projection) = plan.rendered_projection {
                (
                    PacketSectionMode::Rendered,
                    String::from_utf8(projection.rendered_bytes().to_vec())
                        .expect("fixed selected renderers emit UTF-8"),
                )
            } else {
                match plan.disposition {
                    PacketArtifactDisposition::IncludedVerbatim => {
                        let contents = plan
                            .artifact
                            .bytes
                            .as_ref()
                            .map(|bytes| String::from_utf8_lossy(bytes).into_owned())
                            .unwrap_or_default();
                        (PacketSectionMode::Verbatim, contents)
                    }
                    PacketArtifactDisposition::IncludedSummary => (
                        PacketSectionMode::Summary,
                        summarize_artifact(plan.artifact),
                    ),
                    PacketArtifactDisposition::BlockedIngest
                    | PacketArtifactDisposition::OmittedMissing
                    | PacketArtifactDisposition::OmittedEmpty
                    | PacketArtifactDisposition::OmittedStarterTemplate
                    | PacketArtifactDisposition::OmittedInvalid
                    | PacketArtifactDisposition::ExcludedDueToBudget => return None,
                }
            };

            let source_content_sha256 = plan
                .rendered_projection
                .map(|projection| projection.source_fingerprint().to_owned())
                .or_else(|| {
                    plan.artifact
                        .identity
                        .content_sha256
                        .as_ref()
                        .map(|hash| format!("sha256:{hash}"))
                });
            let rendered_output_sha256 = plan
                .rendered_projection
                .map(|projection| projection.rendered_output_fingerprint().to_owned());

            Some(PacketSection {
                instance_id: plan.artifact.identity.instance_id.clone(),
                kind_ref: plan.artifact.identity.kind_ref.clone(),
                label: plan.artifact.identity.label.clone(),
                canonical_repo_relative_path: plan.artifact.identity.relative_path.clone(),
                title: plan.title.clone(),
                mode,
                contents,
                source_content_sha256,
                rendered_output_sha256,
            })
        })
        .collect()
}

fn should_summarize_artifact(
    canonical_repo_relative_path: &str,
    budget_outcome: &BudgetOutcome,
) -> bool {
    budget_outcome.disposition == BudgetDisposition::Summarize
        && budget_targets_artifact(canonical_repo_relative_path, budget_outcome)
}

fn should_exclude_artifact(
    canonical_repo_relative_path: &str,
    budget_outcome: &BudgetOutcome,
) -> bool {
    budget_outcome.disposition == BudgetDisposition::Exclude
        && budget_targets_artifact(canonical_repo_relative_path, budget_outcome)
}

fn budget_targets_artifact(
    canonical_repo_relative_path: &str,
    budget_outcome: &BudgetOutcome,
) -> bool {
    budget_outcome
        .targets
        .iter()
        .any(|target| target.canonical_repo_relative_path == canonical_repo_relative_path)
}

fn summarize_artifact(artifact: &CanonicalArtifact) -> String {
    let byte_len = artifact.identity.byte_len.unwrap_or(0);
    let sha256 = artifact
        .identity
        .content_sha256
        .as_deref()
        .unwrap_or("unavailable");

    format!(
        "budget summary: full contents omitted for {} ({} bytes, sha256={})",
        artifact.identity.relative_path, byte_len, sha256
    )
}

fn fixture_basis_root_for(contract: CanonicalLayoutContract, fixture_set_id: &str) -> String {
    format!(
        "tests/fixtures/execution_demo/{fixture_set_id}/{}/",
        contract.system_root_relative()
    )
}

fn fixture_context_for(
    repo_root: &Path,
    packet_id: &str,
    inputs: PacketArtifactInputs<'_>,
    baseline_validations: &[BaselineArtifactValidation],
    contract: CanonicalLayoutContract,
) -> Option<PacketFixtureContext> {
    if packet_variant_for(packet_id) != PacketVariant::ExecutionDemo {
        return None;
    }

    let fixture_set_id = repo_root
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())?;

    let parent = repo_root.parent()?;
    let grandparent = parent.parent()?;
    let great_grandparent = grandparent.parent()?;
    if parent.file_name().and_then(|name| name.to_str())? != "execution_demo"
        || grandparent.file_name().and_then(|name| name.to_str())? != "fixtures"
        || great_grandparent
            .file_name()
            .and_then(|name| name.to_str())?
            != "tests"
    {
        return None;
    }

    Some(PacketFixtureContext {
        fixture_basis_root: fixture_basis_root_for(contract, fixture_set_id.as_str()),
        fixture_set_id,
        fixture_lineage: present_fixture_sources_for(inputs, baseline_validations),
    })
}

fn next_safe_action_for_ready_packet(variant: PacketVariant) -> ReadyPacketNextSafeAction {
    match variant {
        PacketVariant::Planning | PacketVariant::ExecutionDemo => {
            ReadyPacketNextSafeAction::InspectProof
        }
        PacketVariant::ExecutionLive => ReadyPacketNextSafeAction::RunDoctor,
    }
}

fn system_root_summary(contract: CanonicalLayoutContract, status: SystemRootStatus) -> String {
    let system_root = contract.system_root_relative();
    match status {
        SystemRootStatus::Ok => "canonical root is available".to_string(),
        SystemRootStatus::Missing => format!("missing canonical root `{system_root}`"),
        SystemRootStatus::NotDir => {
            format!("canonical root `{system_root}` is not a directory")
        }
        SystemRootStatus::SymlinkNotAllowed => {
            format!("canonical root `{system_root}` must not be a symlink")
        }
    }
}

fn baseline_verdict_label(verdict: &BaselineArtifactVerdict) -> &'static str {
    match verdict {
        BaselineArtifactVerdict::Missing => "missing",
        BaselineArtifactVerdict::Empty => "empty",
        BaselineArtifactVerdict::StarterOwned => "starter_owned",
        BaselineArtifactVerdict::IngestInvalid => "ingest_invalid",
        BaselineArtifactVerdict::SemanticallyInvalid { .. } => "semantically_invalid",
        BaselineArtifactVerdict::ValidCanonicalTruth { .. } => "valid_canonical_truth",
    }
}

fn baseline_verdict_detail(verdict: &BaselineArtifactVerdict) -> String {
    match verdict {
        BaselineArtifactVerdict::SemanticallyInvalid { summary } => summary.clone(),
        BaselineArtifactVerdict::ValidCanonicalTruth { .. } => "<none>".to_string(),
        BaselineArtifactVerdict::Missing
        | BaselineArtifactVerdict::Empty
        | BaselineArtifactVerdict::StarterOwned
        | BaselineArtifactVerdict::IngestInvalid => "<none>".to_string(),
    }
}

fn required_artifact_refusal(
    category: ResolverRefusalCategory,
    summary: String,
    instance_id: &str,
    kind_ref: &str,
    label: &str,
    canonical_repo_relative_path: &str,
    next_safe_action: ResolverNextSafeAction,
) -> ResolverRefusal {
    ResolverRefusal {
        category,
        summary,
        broken_subject: ResolverSubjectRef::CanonicalArtifact {
            instance_id: instance_id.to_owned(),
            kind_ref: kind_ref.to_owned(),
            label: label.to_owned(),
            canonical_repo_relative_path: canonical_repo_relative_path.to_owned(),
        },
        next_safe_action,
    }
}

fn refusal_for_required_baseline_truth(
    baseline_validations: &[BaselineArtifactValidation],
) -> Option<ResolverRefusal> {
    baseline_validations
        .iter()
        .filter(|validation| validation.packet_required)
        .find_map(|validation| match &validation.verdict {
            BaselineArtifactVerdict::Missing => Some(required_artifact_refusal(
                ResolverRefusalCategory::RequiredArtifactMissing,
                "missing required canonical artifact".to_string(),
                validation.instance_id.as_str(),
                validation.kind_ref.as_str(),
                validation.label.as_str(),
                validation.canonical_repo_relative_path.as_str(),
                author_or_fill_next_safe_action(
                    validation.instance_id.as_str(),
                    validation.kind_ref.as_str(),
                    validation.canonical_repo_relative_path.as_str(),
                ),
            )),
            BaselineArtifactVerdict::Empty => Some(required_artifact_refusal(
                ResolverRefusalCategory::RequiredArtifactEmpty,
                "required canonical artifact is empty".to_string(),
                validation.instance_id.as_str(),
                validation.kind_ref.as_str(),
                validation.label.as_str(),
                validation.canonical_repo_relative_path.as_str(),
                author_or_fill_next_safe_action(
                    validation.instance_id.as_str(),
                    validation.kind_ref.as_str(),
                    validation.canonical_repo_relative_path.as_str(),
                ),
            )),
            BaselineArtifactVerdict::StarterOwned => Some(required_artifact_refusal(
                ResolverRefusalCategory::RequiredArtifactStarterTemplate,
                "required canonical artifact still contains the shipped starter template"
                    .to_string(),
                validation.instance_id.as_str(),
                validation.kind_ref.as_str(),
                validation.label.as_str(),
                validation.canonical_repo_relative_path.as_str(),
                author_or_fill_next_safe_action(
                    validation.instance_id.as_str(),
                    validation.kind_ref.as_str(),
                    validation.canonical_repo_relative_path.as_str(),
                ),
            )),
            BaselineArtifactVerdict::SemanticallyInvalid { summary } => {
                Some(required_artifact_refusal(
                    ResolverRefusalCategory::RequiredArtifactInvalid,
                    format!("required canonical artifact is invalid: {summary}"),
                    validation.instance_id.as_str(),
                    validation.kind_ref.as_str(),
                    validation.label.as_str(),
                    validation.canonical_repo_relative_path.as_str(),
                    author_or_fill_next_safe_action(
                        validation.instance_id.as_str(),
                        validation.kind_ref.as_str(),
                        validation.canonical_repo_relative_path.as_str(),
                    ),
                ))
            }
            BaselineArtifactVerdict::IngestInvalid
            | BaselineArtifactVerdict::ValidCanonicalTruth { .. } => None,
        })
}

fn compute_refusal(
    manifest: &ArtifactManifest,
    baseline_validations: &[BaselineArtifactValidation],
    budget_outcome: &BudgetOutcome,
    request: &ResolveRequest<'_>,
    contract: CanonicalLayoutContract,
) -> Option<ResolverRefusal> {
    match manifest.system_root_status {
        SystemRootStatus::Ok => {}
        SystemRootStatus::Missing => {
            return Some(ResolverRefusal {
                category: ResolverRefusalCategory::SystemRootMissing,
                summary: system_root_summary(contract, SystemRootStatus::Missing),
                broken_subject: ResolverSubjectRef::Policy {
                    policy_id: "system_root",
                },
                next_safe_action: ResolverNextSafeAction::RunSetup,
            });
        }
        SystemRootStatus::NotDir => {
            return Some(ResolverRefusal {
                category: ResolverRefusalCategory::SystemRootNotDir,
                summary: system_root_summary(contract, SystemRootStatus::NotDir),
                broken_subject: ResolverSubjectRef::Policy {
                    policy_id: "system_root",
                },
                next_safe_action: ResolverNextSafeAction::RunSetup,
            });
        }
        SystemRootStatus::SymlinkNotAllowed => {
            return Some(ResolverRefusal {
                category: ResolverRefusalCategory::SystemRootSymlinkNotAllowed,
                summary: system_root_summary(contract, SystemRootStatus::SymlinkNotAllowed),
                broken_subject: ResolverSubjectRef::Policy {
                    policy_id: "system_root",
                },
                next_safe_action: ResolverNextSafeAction::RunSetup,
            });
        }
    }

    if let Some(refusal) = refusal_for_ingest_issues(manifest, baseline_validations) {
        return Some(refusal);
    }

    if let Some(refusal) = refusal_for_required_baseline_truth(baseline_validations) {
        return Some(refusal);
    }

    for artifact in &manifest.artifacts {
        if !artifact.packet_required
            || baseline_artifact_validation_for_path(
                baseline_validations,
                artifact.relative_path.as_str(),
            )
            .is_some()
        {
            continue;
        }

        if ingest_issue_for_path(manifest, artifact.relative_path.as_str()).is_some() {
            continue;
        }

        match artifact.presence {
            ArtifactPresence::Missing => {
                return Some(required_artifact_refusal(
                    ResolverRefusalCategory::RequiredArtifactMissing,
                    "missing required canonical artifact".to_string(),
                    artifact.instance_id.as_str(),
                    artifact.kind_ref.as_str(),
                    artifact.label.as_str(),
                    artifact.relative_path.as_str(),
                    author_or_fill_next_safe_action(
                        artifact.instance_id.as_str(),
                        artifact.kind_ref.as_str(),
                        artifact.relative_path.as_str(),
                    ),
                ));
            }
            ArtifactPresence::PresentEmpty => {
                return Some(required_artifact_refusal(
                    ResolverRefusalCategory::RequiredArtifactEmpty,
                    "required canonical artifact is empty".to_string(),
                    artifact.instance_id.as_str(),
                    artifact.kind_ref.as_str(),
                    artifact.label.as_str(),
                    artifact.relative_path.as_str(),
                    author_or_fill_next_safe_action(
                        artifact.instance_id.as_str(),
                        artifact.kind_ref.as_str(),
                        artifact.relative_path.as_str(),
                    ),
                ));
            }
            ArtifactPresence::PresentNonEmpty if artifact.matches_setup_starter_template => {
                return Some(required_artifact_refusal(
                    ResolverRefusalCategory::RequiredArtifactStarterTemplate,
                    "required canonical artifact still contains the shipped starter template"
                        .to_string(),
                    artifact.instance_id.as_str(),
                    artifact.kind_ref.as_str(),
                    artifact.label.as_str(),
                    artifact.relative_path.as_str(),
                    author_or_fill_next_safe_action(
                        artifact.instance_id.as_str(),
                        artifact.kind_ref.as_str(),
                        artifact.relative_path.as_str(),
                    ),
                ));
            }
            ArtifactPresence::PresentNonEmpty => {}
        }
    }

    if manifest.freshness.status == FreshnessStatus::Invalid {
        let has_forbidden_override = manifest
            .freshness
            .issues
            .iter()
            .any(|issue| matches!(issue.kind, FreshnessIssueKind::ForbiddenOverride));
        return Some(ResolverRefusal {
            category: ResolverRefusalCategory::FreshnessInvalid,
            summary: if has_forbidden_override {
                "freshness truth is invalid (forbidden override)".to_string()
            } else {
                "freshness truth is invalid".to_string()
            },
            broken_subject: ResolverSubjectRef::Policy {
                policy_id: "freshness",
            },
            next_safe_action: ResolverNextSafeAction::RunDoctor,
        });
    }

    if budget_outcome.disposition == BudgetDisposition::Refuse {
        let canonical_repo_relative_path = match budget_outcome.next_safe_action.as_ref() {
            Some(BudgetNextSafeAction::ReduceCanonicalArtifactSize {
                canonical_repo_relative_path,
            }) => canonical_repo_relative_path.clone(),
            None => contract.system_root_relative().to_owned(),
        };

        return Some(ResolverRefusal {
            category: ResolverRefusalCategory::BudgetRefused,
            summary: "budget refused packet generation".to_string(),
            broken_subject: ResolverSubjectRef::Policy {
                policy_id: "budget",
            },
            next_safe_action: ResolverNextSafeAction::ReduceCanonicalArtifactSize {
                canonical_repo_relative_path,
            },
        });
    }

    if request.packet_id == LIVE_EXECUTION_PACKET_ID {
        return Some(ResolverRefusal {
            category: ResolverRefusalCategory::UnsupportedRequest,
            summary:
                "unsupported live slice execution request: reduced v1 supports live planning packets plus fixture-backed execution demos only"
                    .to_string(),
            broken_subject: ResolverSubjectRef::Policy {
                policy_id: "packet_id",
            },
            next_safe_action: ResolverNextSafeAction::RunGenerate {
                packet_id: DEFAULT_PACKET_ID,
            },
        });
    }

    None
}

fn refusal_for_ingest_issues(
    manifest: &ArtifactManifest,
    baseline_validations: &[BaselineArtifactValidation],
) -> Option<ResolverRefusal> {
    let mut first_symlink_issue = None;
    let mut first_required_read_issue = None;

    for issue in &manifest.ingest_issues {
        if manifest.artifacts.iter().any(|artifact| {
            artifact.instance_id == "environment_context"
                && artifact.relative_path == issue.canonical_repo_relative_path
                && !artifact.packet_required
        }) {
            continue;
        }
        match issue.kind {
            ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed => {
                first_symlink_issue.get_or_insert(issue);
            }
            ArtifactIngestIssueKind::CanonicalArtifactReadError => {
                if issue.packet_required {
                    let detail = baseline_artifact_validation_for_path(
                        baseline_validations,
                        issue.canonical_repo_relative_path.as_str(),
                    )
                    .and_then(|validation| match &validation.verdict {
                        BaselineArtifactVerdict::SemanticallyInvalid { summary } => {
                            Some(summary.as_str())
                        }
                        _ => None,
                    });
                    first_required_read_issue.get_or_insert((issue, detail));
                }
            }
        }
    }

    if let Some(issue) = first_symlink_issue {
        let canonical_repo_relative_path = issue.canonical_repo_relative_path.clone();
        return Some(ResolverRefusal {
            category: ResolverRefusalCategory::NonCanonicalInputAttempt,
            summary: "canonical artifact path must not be a symlink".to_string(),
            broken_subject: ResolverSubjectRef::CanonicalArtifact {
                instance_id: issue.instance_id.clone(),
                kind_ref: issue.kind_ref.clone(),
                label: issue.label.clone(),
                canonical_repo_relative_path,
            },
            next_safe_action: ResolverNextSafeAction::RunSetupRefresh,
        });
    }

    if let Some((issue, detail)) = first_required_read_issue {
        let canonical_repo_relative_path = issue.canonical_repo_relative_path.clone();
        return Some(ResolverRefusal {
            category: ResolverRefusalCategory::ArtifactReadError,
            summary: format!(
                "failed to read canonical artifact: {}",
                detail.unwrap_or("repository_read_failed")
            ),
            broken_subject: ResolverSubjectRef::CanonicalArtifact {
                instance_id: issue.instance_id.clone(),
                kind_ref: issue.kind_ref.clone(),
                label: issue.label.clone(),
                canonical_repo_relative_path,
            },
            next_safe_action: ResolverNextSafeAction::RunSetupRefresh,
        });
    }

    None
}

fn compute_blockers(
    manifest: &ArtifactManifest,
    baseline_validations: &[BaselineArtifactValidation],
    budget_outcome: &BudgetOutcome,
    request: &ResolveRequest<'_>,
    contract: CanonicalLayoutContract,
) -> Vec<ResolverBlocker> {
    let mut blockers = build_required_baseline_blockers(manifest, baseline_validations, contract);

    if blockers.is_empty() {
        for artifact in &manifest.artifacts {
            if !artifact.packet_required
                || baseline_artifact_validation_for_path(
                    baseline_validations,
                    artifact.relative_path.as_str(),
                )
                .is_some()
            {
                continue;
            }

            if ingest_issue_for_path(manifest, artifact.relative_path.as_str()).is_some() {
                continue;
            }

            match artifact.presence {
                ArtifactPresence::Missing => blockers.push(required_artifact_blocker(
                    ResolverBlockerCategory::RequiredArtifactMissing,
                    "missing required canonical artifact".to_string(),
                    artifact.instance_id.as_str(),
                    artifact.kind_ref.as_str(),
                    artifact.label.as_str(),
                    artifact.relative_path.as_str(),
                    author_or_fill_next_safe_action(
                        artifact.instance_id.as_str(),
                        artifact.kind_ref.as_str(),
                        artifact.relative_path.as_str(),
                    ),
                )),
                ArtifactPresence::PresentEmpty => blockers.push(required_artifact_blocker(
                    ResolverBlockerCategory::RequiredArtifactEmpty,
                    "required canonical artifact is empty".to_string(),
                    artifact.instance_id.as_str(),
                    artifact.kind_ref.as_str(),
                    artifact.label.as_str(),
                    artifact.relative_path.as_str(),
                    author_or_fill_next_safe_action(
                        artifact.instance_id.as_str(),
                        artifact.kind_ref.as_str(),
                        artifact.relative_path.as_str(),
                    ),
                )),
                ArtifactPresence::PresentNonEmpty if artifact.matches_setup_starter_template => {
                    blockers.push(required_artifact_blocker(
                        ResolverBlockerCategory::RequiredArtifactStarterTemplate,
                        "required canonical artifact still contains the shipped starter template"
                            .to_string(),
                        artifact.instance_id.as_str(),
                        artifact.kind_ref.as_str(),
                        artifact.label.as_str(),
                        artifact.relative_path.as_str(),
                        author_or_fill_next_safe_action(
                            artifact.instance_id.as_str(),
                            artifact.kind_ref.as_str(),
                            artifact.relative_path.as_str(),
                        ),
                    ));
                }
                ArtifactPresence::PresentNonEmpty => {}
            }
        }
    }

    let has_forbidden_override = manifest
        .freshness
        .issues
        .iter()
        .any(|issue| matches!(issue.kind, FreshnessIssueKind::ForbiddenOverride));
    if has_forbidden_override {
        blockers.push(ResolverBlocker {
            category: ResolverBlockerCategory::FreshnessInvalid,
            subject: ResolverSubjectRef::Policy {
                policy_id: "freshness",
            },
            summary: "freshness truth is invalid (forbidden override)".to_string(),
            next_safe_action: ResolverNextSafeAction::RunDoctor,
        });
    }

    if budget_outcome.disposition == BudgetDisposition::Refuse {
        let canonical_repo_relative_path = match budget_outcome.next_safe_action.as_ref() {
            Some(BudgetNextSafeAction::ReduceCanonicalArtifactSize {
                canonical_repo_relative_path,
            }) => canonical_repo_relative_path.clone(),
            None => contract.system_root_relative().to_owned(),
        };

        blockers.push(ResolverBlocker {
            category: ResolverBlockerCategory::BudgetRefused,
            subject: ResolverSubjectRef::Policy {
                policy_id: "budget",
            },
            summary: "budget refused packet generation".to_string(),
            next_safe_action: ResolverNextSafeAction::ReduceCanonicalArtifactSize {
                canonical_repo_relative_path,
            },
        });
    }

    if blockers.is_empty() && request.packet_id == LIVE_EXECUTION_PACKET_ID {
        blockers.push(ResolverBlocker {
            category: ResolverBlockerCategory::UnsupportedRequest,
            subject: ResolverSubjectRef::Policy {
                policy_id: "packet_id",
            },
            summary:
                "unsupported live slice execution request: reduced v1 supports live planning packets plus fixture-backed execution demos only"
                    .to_string(),
            next_safe_action: ResolverNextSafeAction::RunGenerate {
                packet_id: DEFAULT_PACKET_ID,
            },
        });
    }

    sort_blockers(&mut blockers);
    blockers
}
