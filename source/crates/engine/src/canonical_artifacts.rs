use crate::artifact_instance::RequirednessMode;
use crate::artifact_registry::ResolvedArtifactInstance;
use crate::canonical_paths::{
    default_canonical_layout_contract, CanonicalLayout, CanonicalLayoutContract,
};
use crate::canonical_repo_support::{RepoRelativeFileAccessError, RepoRelativeMetadataReadError};
use crate::charter_artifact::{
    parse_canonical_charter, render_canonical_charter_markdown, CharterArtifactErrorKind,
};
use crate::definition_identity::{DefinitionFingerprint, MAX_SOURCE_DOCUMENT_BYTES};
use crate::environment_context_artifact::{
    parse_canonical_environment_context, render_environment_context_markdown,
    EnvironmentContextArtifactErrorKind,
};
use crate::profile_decision::{
    resolve_shipped_profile_decisions, ArtifactApplicability, ArtifactProfileDecision,
    ResolvedProfileDecisions,
};
use crate::project_context_artifact::{
    parse_canonical_project_context, render_project_context_markdown,
    ProjectContextArtifactErrorKind,
};
use crate::repository_invocation_identity::{
    read_repository_identity, REPOSITORY_IDENTITY_REPO_PATH,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalArtifactKind {
    Charter,
    ProjectContext,
    EnvironmentContext,
    FeatureSpec,
}

const CHARTER_TEMPLATE: &str = "\
# Charter
\n\
Describe the durable operating rules for this system.\n\
\n\
## Purpose\n\
\n\
- TODO\n\
\n\
## Constraints\n\
\n\
- TODO\n\
\n\
## Review Cadence\n\
\n\
- TODO\n";

const FEATURE_SPEC_TEMPLATE: &str = "\
# Feature Spec
\n\
Describe the product behavior that trusted project truth should produce.\n\
\n\
## Problem\n\
\n\
- TODO\n\
\n\
## Outcomes\n\
\n\
- TODO\n\
\n\
## Scope\n\
\n\
- TODO\n";

const PROJECT_CONTEXT_TEMPLATE: &str = "\
# Project Context
\n\
Optional: capture surrounding architecture, constraints, and local context that help planning.\n\
\n\
## Current State\n\
\n\
- TODO\n\
\n\
## Constraints\n\
\n\
- TODO\n\
\n\
## Open Questions\n\
\n\
- TODO\n";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalArtifactDescriptor {
    pub kind: CanonicalArtifactKind,
    pub relative_path: &'static str,
    pub namespace_dir: &'static str,
    pub packet_required: bool,
    pub baseline_required: bool,
    pub setup_scaffolded: bool,
    pub setup_starter_template: &'static str,
}

const CANONICAL_ARTIFACT_DESCRIPTORS: [CanonicalArtifactDescriptor; 3] = [
    CanonicalArtifactDescriptor {
        kind: CanonicalArtifactKind::Charter,
        relative_path: crate::canonical_paths::CANONICAL_CHARTER_RELATIVE_PATH,
        namespace_dir: crate::canonical_paths::CANONICAL_CHARTER_NAMESPACE_DIR,
        packet_required: true,
        baseline_required: true,
        setup_scaffolded: true,
        setup_starter_template: CHARTER_TEMPLATE,
    },
    CanonicalArtifactDescriptor {
        kind: CanonicalArtifactKind::ProjectContext,
        relative_path: crate::canonical_paths::CANONICAL_PROJECT_CONTEXT_RELATIVE_PATH,
        namespace_dir: crate::canonical_paths::CANONICAL_PROJECT_CONTEXT_NAMESPACE_DIR,
        packet_required: false,
        baseline_required: true,
        setup_scaffolded: true,
        setup_starter_template: PROJECT_CONTEXT_TEMPLATE,
    },
    CanonicalArtifactDescriptor {
        kind: CanonicalArtifactKind::FeatureSpec,
        relative_path: crate::canonical_paths::CANONICAL_FEATURE_SPEC_RELATIVE_PATH,
        namespace_dir: crate::canonical_paths::CANONICAL_FEATURE_SPEC_NAMESPACE_DIR,
        packet_required: false,
        baseline_required: false,
        setup_scaffolded: false,
        setup_starter_template: FEATURE_SPEC_TEMPLATE,
    },
];

pub fn canonical_artifact_descriptors() -> &'static [CanonicalArtifactDescriptor; 3] {
    &CANONICAL_ARTIFACT_DESCRIPTORS
}

pub fn setup_starter_template(kind: CanonicalArtifactKind) -> &'static str {
    CANONICAL_ARTIFACT_DESCRIPTORS
        .iter()
        .find(|descriptor| descriptor.kind == kind)
        .expect("canonical artifact descriptor should exist")
        .setup_starter_template
}

pub fn setup_starter_template_bytes(kind: CanonicalArtifactKind) -> &'static [u8] {
    setup_starter_template(kind).as_bytes()
}

pub fn matches_setup_starter_template(kind: CanonicalArtifactKind, bytes: &[u8]) -> bool {
    bytes == setup_starter_template_bytes(kind)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SystemRootStatus {
    Ok,
    Missing,
    NotDir,
    SymlinkNotAllowed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactPresence {
    Missing,
    PresentEmpty,
    PresentNonEmpty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalArtifactIdentity {
    pub instance_id: String,
    pub kind_ref: String,
    pub label: String,
    pub relative_path: String,
    pub requiredness_mode: RequirednessMode,
    pub applicability: ArtifactApplicability,
    pub renderer_definition_refs: Vec<String>,
    pub packet_required: bool,
    pub baseline_required: bool,
    pub setup_scaffolded: bool,
    pub presence: ArtifactPresence,
    pub byte_len: Option<u64>,
    pub content_sha256: Option<String>,
    pub matches_setup_starter_template: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalArtifact {
    pub identity: CanonicalArtifactIdentity,
    pub bytes: Option<Vec<u8>>,
    pub rendered_bytes: Option<Vec<u8>>,
    pub rendered_output_sha256: Option<String>,
    pub rendered_media_type: Option<String>,
    pub render_failure: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalArtifacts {
    pub system_root_status: SystemRootStatus,
    pub artifacts: Vec<CanonicalArtifact>,
    pub ingest_issues: Vec<ArtifactIngestIssue>,
}

impl CanonicalArtifacts {
    pub fn load(repo_root: impl AsRef<Path>) -> Result<Self, ArtifactIngestError> {
        Self::load_with_contract(repo_root, *default_canonical_layout_contract())
    }

    pub fn load_with_contract(
        repo_root: impl AsRef<Path>,
        contract: CanonicalLayoutContract,
    ) -> Result<Self, ArtifactIngestError> {
        let repo_root = repo_root.as_ref();
        let decisions = resolve_shipped_profile_decisions(repo_root).map_err(|error| {
            ArtifactIngestError::ReadFailure {
                path: repo_root.to_path_buf(),
                source: std::io::Error::other(format!(
                    "failed to resolve admitted artifact descriptors: {error:?}"
                )),
            }
        })?;
        let layout = if contract == *default_canonical_layout_contract() {
            CanonicalLayout::new(repo_root)
        } else {
            CanonicalLayout::with_contract(repo_root, contract)
        };
        let workspace = layout.workspace();
        let system_root = layout.system_root();

        let system_root_status = match workspace.metadata_no_follow(&system_root) {
            Ok(Some(meta)) => {
                if meta.file_type().is_symlink() {
                    SystemRootStatus::SymlinkNotAllowed
                } else if !meta.is_dir() {
                    SystemRootStatus::NotDir
                } else if canonical_root_scaffold_exists(repo_root, layout, &decisions)? {
                    SystemRootStatus::Ok
                } else {
                    SystemRootStatus::Missing
                }
            }
            Ok(None) => SystemRootStatus::Missing,
            Err(RepoRelativeMetadataReadError { path, source }) => {
                return Err(ArtifactIngestError::ReadFailure { path, source });
            }
        };

        let mut ingest_issues = Vec::new();

        let mut admitted = decisions.artifact_decisions().iter().collect::<Vec<_>>();
        admitted.sort_by(|left, right| {
            let rank = |applicability| match applicability {
                ArtifactApplicability::Required => 0,
                ArtifactApplicability::Optional => 1,
                ArtifactApplicability::Indeterminate => 2,
            };
            (rank(left.applicability()), left.instance_id().as_str())
                .cmp(&(rank(right.applicability()), right.instance_id().as_str()))
        });
        let artifacts = admitted
            .into_iter()
            .map(|decision| {
                let descriptor = decisions
                    .registry()
                    .instance(decision.instance_id())
                    .expect("admitted decision retains its resolved descriptor");
                match system_root_status {
                    SystemRootStatus::Ok => {
                        load_one(layout, &decisions, decision, descriptor, &mut ingest_issues)
                    }
                    SystemRootStatus::Missing
                    | SystemRootStatus::NotDir
                    | SystemRootStatus::SymlinkNotAllowed => missing_one(decision, descriptor),
                }
            })
            .collect();

        Ok(Self {
            system_root_status,
            artifacts,
            ingest_issues,
        })
    }

    pub fn identities(&self) -> Vec<&CanonicalArtifactIdentity> {
        self.artifacts
            .iter()
            .map(|artifact| &artifact.identity)
            .collect()
    }
}

fn canonical_root_scaffold_exists(
    repo_root: &Path,
    layout: CanonicalLayout<'_>,
    decisions: &ResolvedProfileDecisions,
) -> Result<bool, ArtifactIngestError> {
    if layout.contract() == *default_canonical_layout_contract() {
        match read_repository_identity(repo_root) {
            Ok(Some(_)) => return Ok(true),
            Ok(None) => {}
            Err(error) => {
                return Err(ArtifactIngestError::ReadFailure {
                    path: repo_root.join(REPOSITORY_IDENTITY_REPO_PATH),
                    source: std::io::Error::other(error),
                });
            }
        }
    }

    let workspace = layout.workspace();
    for decision in decisions.artifact_decisions() {
        let artifact_path = workspace
            .normalize_repo_relative(decision.canonical_path())
            .expect("admitted artifact paths stay repo-relative");
        match workspace.metadata_no_follow(&artifact_path) {
            Ok(Some(_)) => return Ok(true),
            Ok(None) => {}
            Err(err) => return Err(artifact_ingest_read_failure(err)),
        }

        let namespace_dir = Path::new(decision.canonical_path())
            .parent()
            .and_then(Path::to_str)
            .map(|path| {
                workspace
                    .normalize_repo_relative(path)
                    .expect("admitted artifact namespaces stay repo-relative")
            })
            .expect("admitted artifact paths retain a namespace");
        match workspace.metadata_no_follow(&namespace_dir) {
            Ok(Some(meta)) if meta.is_dir() => return Ok(true),
            Ok(Some(_)) | Ok(None) => {}
            Err(err) => return Err(artifact_ingest_read_failure(err)),
        }
    }

    Ok(false)
}

#[derive(Debug)]
pub enum ArtifactIngestError {
    SystemRootMissing {
        system_root: PathBuf,
    },
    SystemRootNotDir {
        system_root: PathBuf,
    },
    SystemRootSymlinkNotAllowed {
        system_root: PathBuf,
    },
    RequiredArtifactMissing {
        kind: CanonicalArtifactKind,
        path: PathBuf,
    },
    ReadFailure {
        path: PathBuf,
        source: std::io::Error,
    },
}

impl std::fmt::Display for ArtifactIngestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArtifactIngestError::SystemRootMissing { system_root } => {
                write!(
                    f,
                    "missing canonical system root at {}",
                    system_root.display()
                )
            }
            ArtifactIngestError::SystemRootNotDir { system_root } => write!(
                f,
                "canonical system root is not a directory: {}",
                system_root.display()
            ),
            ArtifactIngestError::SystemRootSymlinkNotAllowed { system_root } => write!(
                f,
                "canonical system root must not be a symlink: {}",
                system_root.display()
            ),
            ArtifactIngestError::RequiredArtifactMissing { kind, path } => write!(
                f,
                "missing required canonical artifact {kind:?} at {}",
                path.display()
            ),
            ArtifactIngestError::ReadFailure { path, source } => {
                write!(
                    f,
                    "failed to read canonical artifact at {}: {source}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for ArtifactIngestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ArtifactIngestError::ReadFailure { source, .. } => Some(source),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactIngestIssueKind {
    CanonicalArtifactSymlinkNotAllowed,
    CanonicalArtifactReadError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactIngestIssue {
    pub kind: ArtifactIngestIssueKind,
    pub instance_id: String,
    pub kind_ref: String,
    pub label: String,
    pub canonical_repo_relative_path: String,
    pub packet_required: bool,
}

fn record_ingest_issue(
    issues: &mut Vec<ArtifactIngestIssue>,
    kind: ArtifactIngestIssueKind,
    instance_id: &str,
    kind_ref: &str,
    label: &str,
    canonical_repo_relative_path: &str,
    packet_required: bool,
) {
    issues.push(ArtifactIngestIssue {
        kind,
        instance_id: instance_id.to_owned(),
        kind_ref: kind_ref.to_owned(),
        label: label.to_owned(),
        canonical_repo_relative_path: canonical_repo_relative_path.to_owned(),
        packet_required,
    });
}

fn load_one(
    layout: CanonicalLayout<'_>,
    decisions: &ResolvedProfileDecisions,
    decision: &ArtifactProfileDecision,
    descriptor: &ResolvedArtifactInstance,
    issues: &mut Vec<ArtifactIngestIssue>,
) -> CanonicalArtifact {
    let workspace = layout.workspace();
    let artifact_path = workspace
        .normalize_repo_relative(decision.canonical_path())
        .expect("admitted artifact paths stay repo-relative");
    let packet_required = decision.applicability() == ArtifactApplicability::Required;
    let mut record_selected_issue = |issue_kind| {
        record_ingest_issue(
            issues,
            issue_kind,
            decision.instance_id().as_str(),
            decision.kind_ref().as_str(),
            descriptor.label(),
            decision.canonical_path(),
            packet_required,
        );
    };

    let meta = match workspace.metadata_no_follow(&artifact_path) {
        Ok(meta) => meta,
        Err(_err) => {
            record_selected_issue(ArtifactIngestIssueKind::CanonicalArtifactReadError);
            let mut artifact = missing_one(decision, descriptor);
            artifact.render_failure = Some("repository_read_failed".to_owned());
            return artifact;
        }
    };

    if meta.is_none() {
        return missing_one(decision, descriptor);
    }

    let meta = meta.expect("meta");
    if meta.file_type().is_symlink() {
        record_selected_issue(ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed);
        let mut artifact = missing_one(decision, descriptor);
        artifact.render_failure = Some("symlink_refused".to_owned());
        return artifact;
    }
    if !meta.is_file() {
        record_selected_issue(ArtifactIngestIssueKind::CanonicalArtifactReadError);
        let mut artifact = missing_one(decision, descriptor);
        artifact.render_failure = Some("non_regular_file_refused".to_owned());
        return artifact;
    }

    let trusted_file = match workspace.trusted_read(&artifact_path) {
        Ok(trusted_file) => trusted_file,
        Err(RepoRelativeFileAccessError::SymlinkNotAllowed(_)) => {
            record_selected_issue(ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed);
            let mut artifact = missing_one(decision, descriptor);
            artifact.render_failure = Some("symlink_refused".to_owned());
            return artifact;
        }
        Err(RepoRelativeFileAccessError::NotRegularFile(_)) => {
            record_selected_issue(ArtifactIngestIssueKind::CanonicalArtifactReadError);
            let mut artifact = missing_one(decision, descriptor);
            artifact.render_failure = Some("non_regular_file_refused".to_owned());
            return artifact;
        }
        Err(
            RepoRelativeFileAccessError::Missing(_)
            | RepoRelativeFileAccessError::ReadFailure { .. },
        ) => {
            record_selected_issue(ArtifactIngestIssueKind::CanonicalArtifactReadError);
            let mut artifact = missing_one(decision, descriptor);
            artifact.render_failure = Some("repository_read_failed".to_owned());
            return artifact;
        }
        Err(RepoRelativeFileAccessError::InvalidPath(_)) => {
            unreachable!("canonical artifact paths should stay repo-relative")
        }
    };

    let (bytes, exceeded) = match trusted_file.read_bytes_bounded_stable(MAX_SOURCE_DOCUMENT_BYTES)
    {
        Ok(observed) => observed,
        Err(error) => {
            record_selected_issue(ArtifactIngestIssueKind::CanonicalArtifactReadError);
            let detail = error.to_string();
            let mut artifact = missing_one(decision, descriptor);
            artifact.render_failure = Some(
                if detail.contains("stable regular-file identity")
                    || detail.contains("retained-handle observation")
                {
                    "observation_changed_during_inspection"
                } else {
                    "repository_read_failed"
                }
                .to_owned(),
            );
            return artifact;
        }
    };
    if exceeded {
        record_selected_issue(ArtifactIngestIssueKind::CanonicalArtifactReadError);
        let mut artifact = missing_one(decision, descriptor);
        artifact.render_failure = Some("document_limit_exceeded".to_owned());
        return artifact;
    }

    let byte_len = bytes.len() as u64;
    let presence = if byte_len == 0 {
        ArtifactPresence::PresentEmpty
    } else {
        ArtifactPresence::PresentNonEmpty
    };

    let content_sha256 = Some(sha256_hex(&bytes));
    let renderer = descriptor
        .renderer_definition_refs()
        .first()
        .map(|renderer| renderer.as_str());
    let fixed_renderer_selected = matches!(
        (
            decision.instance_id().as_str(),
            decision.kind_ref().as_str(),
            renderer,
        ),
        (
            "project_authority",
            "handbook.artifact-kind.project-authority@1.1.0",
            Some("handbook.renderer.charter-review-markdown@1.0.0"),
        ) | (
            "project_context",
            "handbook.artifact-kind.project-context@1.1.0",
            Some("handbook.renderer.project-context-review-markdown@1.0.0"),
        ) | (
            "environment_context",
            "handbook.artifact-kind.environment-context@1.1.0",
            Some("handbook.renderer.environment-context-review-markdown@1.0.0"),
        )
    );
    let rendered = match (
        decision.instance_id().as_str(),
        decision.kind_ref().as_str(),
        renderer,
    ) {
        (
            "project_authority",
            "handbook.artifact-kind.project-authority@1.1.0",
            Some("handbook.renderer.charter-review-markdown@1.0.0"),
        ) => parse_canonical_charter(decisions, &bytes)
            .and_then(|record| render_canonical_charter_markdown(&record))
            .map_err(|error| match error.kind() {
                CharterArtifactErrorKind::SourceLimitExceeded => "document_limit_exceeded",
                CharterArtifactErrorKind::DuplicateKey => "duplicate_yaml_key",
                CharterArtifactErrorKind::SyntaxError => "yaml_syntax_invalid",
                CharterArtifactErrorKind::NonObjectRoot => "document_not_object",
                CharterArtifactErrorKind::SelectedDecisionMissing
                | CharterArtifactErrorKind::SelectedContractMismatch
                | CharterArtifactErrorKind::StructuralValidationFailed
                | CharterArtifactErrorKind::SemanticValidationFailed => {
                    "structural_validation_failed"
                }
                CharterArtifactErrorKind::TypedDecodeFailed => "typed_decode_failed",
                CharterArtifactErrorKind::SerializationFailed
                | CharterArtifactErrorKind::RenderedViewRefused => "rendered_view_refused",
            }),
        (
            "project_context",
            "handbook.artifact-kind.project-context@1.1.0",
            Some("handbook.renderer.project-context-review-markdown@1.0.0"),
        ) => parse_canonical_project_context(decisions, &bytes)
            .and_then(|record| render_project_context_markdown(&record))
            .map_err(|error| match error.kind() {
                ProjectContextArtifactErrorKind::SourceLimitExceeded => "document_limit_exceeded",
                ProjectContextArtifactErrorKind::DuplicateKey => "duplicate_yaml_key",
                ProjectContextArtifactErrorKind::SyntaxError => "yaml_syntax_invalid",
                ProjectContextArtifactErrorKind::NonObjectRoot => "document_not_object",
                ProjectContextArtifactErrorKind::SelectedDecisionMissing
                | ProjectContextArtifactErrorKind::SelectedContractMismatch
                | ProjectContextArtifactErrorKind::StructuralValidationFailed => {
                    "structural_validation_failed"
                }
                ProjectContextArtifactErrorKind::TypedDecodeFailed => "typed_decode_failed",
                ProjectContextArtifactErrorKind::SerializationFailed
                | ProjectContextArtifactErrorKind::RenderedViewRefused => "rendered_view_refused",
            }),
        (
            "environment_context",
            "handbook.artifact-kind.environment-context@1.1.0",
            Some("handbook.renderer.environment-context-review-markdown@1.0.0"),
        ) => parse_canonical_environment_context(decisions, &bytes)
            .and_then(|record| render_environment_context_markdown(&record))
            .map_err(|error| match error.kind() {
                EnvironmentContextArtifactErrorKind::SourceLimitExceeded => {
                    "document_limit_exceeded"
                }
                EnvironmentContextArtifactErrorKind::DuplicateKey => "duplicate_yaml_key",
                EnvironmentContextArtifactErrorKind::SyntaxError => "yaml_syntax_invalid",
                EnvironmentContextArtifactErrorKind::NonObjectRoot => "document_not_object",
                EnvironmentContextArtifactErrorKind::SelectedDecisionMissing
                | EnvironmentContextArtifactErrorKind::SelectedContractMismatch
                | EnvironmentContextArtifactErrorKind::StructuralValidationFailed
                | EnvironmentContextArtifactErrorKind::DuplicateEnvironmentId => {
                    "structural_validation_failed"
                }
                EnvironmentContextArtifactErrorKind::TypedDecodeFailed => "typed_decode_failed",
                EnvironmentContextArtifactErrorKind::SerializationFailed
                | EnvironmentContextArtifactErrorKind::RenderedViewRefused => {
                    "rendered_view_refused"
                }
                EnvironmentContextArtifactErrorKind::Missing
                | EnvironmentContextArtifactErrorKind::UnsafePath
                | EnvironmentContextArtifactErrorKind::SourceReadFailed
                | EnvironmentContextArtifactErrorKind::ObservationChanged => {
                    "repository_read_failed"
                }
            }),
        _ => Ok(bytes.clone()),
    };
    let (rendered_bytes, rendered_output_sha256, rendered_media_type, render_failure) =
        match rendered {
            Ok(rendered_bytes) if fixed_renderer_selected => {
                let fingerprint = DefinitionFingerprint::from_bytes(&rendered_bytes).to_string();
                (
                    Some(rendered_bytes),
                    Some(fingerprint),
                    Some("text/markdown".to_owned()),
                    None,
                )
            }
            Ok(_) => (None, None, None, None),
            Err(reason) => (None, None, None, Some(reason.to_owned())),
        };

    CanonicalArtifact {
        identity: CanonicalArtifactIdentity {
            instance_id: decision.instance_id().as_str().to_owned(),
            kind_ref: decision.kind_ref().as_str().to_owned(),
            label: descriptor.label().to_owned(),
            relative_path: decision.canonical_path().to_owned(),
            requiredness_mode: decision.requiredness_mode(),
            applicability: decision.applicability(),
            renderer_definition_refs: descriptor
                .renderer_definition_refs()
                .iter()
                .map(|renderer| renderer.as_str().to_owned())
                .collect(),
            packet_required,
            baseline_required: packet_required,
            setup_scaffolded: false,
            presence,
            byte_len: Some(byte_len),
            content_sha256,
            matches_setup_starter_template: false,
        },
        bytes: Some(bytes),
        rendered_bytes,
        rendered_output_sha256,
        rendered_media_type,
        render_failure,
    }
}

fn missing_one(
    decision: &ArtifactProfileDecision,
    descriptor: &ResolvedArtifactInstance,
) -> CanonicalArtifact {
    let packet_required = decision.applicability() == ArtifactApplicability::Required;
    CanonicalArtifact {
        identity: CanonicalArtifactIdentity {
            instance_id: decision.instance_id().as_str().to_owned(),
            kind_ref: decision.kind_ref().as_str().to_owned(),
            label: descriptor.label().to_owned(),
            relative_path: decision.canonical_path().to_owned(),
            requiredness_mode: decision.requiredness_mode(),
            applicability: decision.applicability(),
            renderer_definition_refs: descriptor
                .renderer_definition_refs()
                .iter()
                .map(|renderer| renderer.as_str().to_owned())
                .collect(),
            packet_required,
            baseline_required: packet_required,
            setup_scaffolded: false,
            presence: ArtifactPresence::Missing,
            byte_len: None,
            content_sha256: None,
            matches_setup_starter_template: false,
        },
        bytes: None,
        rendered_bytes: None,
        rendered_output_sha256: None,
        rendered_media_type: None,
        render_failure: None,
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    bytes_to_lower_hex(&digest)
}

fn bytes_to_lower_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write;
        let _ = write!(out, "{:02x}", b);
    }
    out
}

fn artifact_ingest_read_failure(err: RepoRelativeMetadataReadError) -> ArtifactIngestError {
    ArtifactIngestError::ReadFailure {
        path: err.path,
        source: err.source,
    }
}
