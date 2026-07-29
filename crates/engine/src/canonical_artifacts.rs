use crate::canonical_paths::{
    canonical_artifact_relative_path, default_canonical_layout_contract, CanonicalLayout,
    CanonicalLayoutContract,
};
use crate::canonical_repo_support::{RepoRelativeFileAccessError, RepoRelativeMetadataReadError};
use crate::project_context_artifact::SELECTED_PROJECT_CONTEXT_CANONICAL_PATH;
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

impl CanonicalArtifactKind {
    pub(crate) fn relative_path(self) -> &'static str {
        canonical_artifact_relative_path(self)
    }
}

pub const CANONICAL_ARTIFACT_ORDER: [CanonicalArtifactKind; 3] = [
    CanonicalArtifactKind::Charter,
    CanonicalArtifactKind::ProjectContext,
    CanonicalArtifactKind::FeatureSpec,
];

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
    descriptor_for(kind).setup_starter_template
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
    pub kind: CanonicalArtifactKind,
    pub relative_path: String,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalArtifacts {
    pub system_root_status: SystemRootStatus,
    pub charter: CanonicalArtifact,
    pub project_context: CanonicalArtifact,
    pub feature_spec: CanonicalArtifact,
    pub ingest_issues: Vec<ArtifactIngestIssue>,
}

impl CanonicalArtifacts {
    pub fn load(repo_root: impl AsRef<Path>) -> Result<Self, ArtifactIngestError> {
        Self::load_with_contract(repo_root, *default_canonical_layout_contract())
    }

    pub fn load_fixed_siblings(repo_root: impl AsRef<Path>) -> Result<Self, ArtifactIngestError> {
        Self::load_fixed_siblings_with_contract(repo_root, *default_canonical_layout_contract())
    }

    pub fn load_with_contract(
        repo_root: impl AsRef<Path>,
        contract: CanonicalLayoutContract,
    ) -> Result<Self, ArtifactIngestError> {
        Self::load_with_contract_selection(repo_root, contract, true)
    }

    pub fn load_fixed_siblings_with_contract(
        repo_root: impl AsRef<Path>,
        contract: CanonicalLayoutContract,
    ) -> Result<Self, ArtifactIngestError> {
        Self::load_with_contract_selection(repo_root, contract, false)
    }

    fn load_with_contract_selection(
        repo_root: impl AsRef<Path>,
        contract: CanonicalLayoutContract,
        include_legacy_project_context: bool,
    ) -> Result<Self, ArtifactIngestError> {
        let repo_root = repo_root.as_ref();
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
                } else if canonical_root_scaffold_exists(layout, include_legacy_project_context)? {
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

        let (charter, project_context, feature_spec) = match system_root_status {
            SystemRootStatus::Ok => (
                load_one(layout, CanonicalArtifactKind::Charter, &mut ingest_issues),
                if include_legacy_project_context {
                    load_one(
                        layout,
                        CanonicalArtifactKind::ProjectContext,
                        &mut ingest_issues,
                    )
                } else {
                    missing_one(layout, CanonicalArtifactKind::ProjectContext)
                },
                load_one(
                    layout,
                    CanonicalArtifactKind::FeatureSpec,
                    &mut ingest_issues,
                ),
            ),
            SystemRootStatus::Missing
            | SystemRootStatus::NotDir
            | SystemRootStatus::SymlinkNotAllowed => (
                missing_one(layout, CanonicalArtifactKind::Charter),
                missing_one(layout, CanonicalArtifactKind::ProjectContext),
                missing_one(layout, CanonicalArtifactKind::FeatureSpec),
            ),
        };

        Ok(Self {
            system_root_status,
            charter,
            project_context,
            feature_spec,
            ingest_issues,
        })
    }

    pub fn identities(&self) -> [&CanonicalArtifactIdentity; 3] {
        [
            &self.charter.identity,
            &self.project_context.identity,
            &self.feature_spec.identity,
        ]
    }
}

fn canonical_root_scaffold_exists(
    layout: CanonicalLayout<'_>,
    include_legacy_project_context: bool,
) -> Result<bool, ArtifactIngestError> {
    let workspace = layout.workspace();
    for kind in CANONICAL_ARTIFACT_ORDER {
        if kind == CanonicalArtifactKind::ProjectContext && !include_legacy_project_context {
            let selected_namespace_path = Path::new(SELECTED_PROJECT_CONTEXT_CANONICAL_PATH)
                .parent()
                .expect("selected Project Context path has a namespace");
            if selected_namespace_path.starts_with(Path::new(layout.system_root_relative())) {
                let selected_namespace = workspace
                    .normalize_repo_relative(
                        selected_namespace_path
                            .to_str()
                            .expect("selected Project Context namespace is UTF-8"),
                    )
                    .expect("selected Project Context namespace stays repo-relative");
                match workspace.metadata_no_follow(&selected_namespace) {
                    Ok(Some(meta)) if meta.is_dir() => return Ok(true),
                    Ok(Some(_)) | Ok(None) => {}
                    Err(err) => return Err(artifact_ingest_read_failure(err)),
                }
            }
            continue;
        }
        let artifact_path = layout.artifact_path(kind);
        match workspace.metadata_no_follow(&artifact_path) {
            Ok(Some(_)) => return Ok(true),
            Ok(None) => {}
            Err(err) => return Err(artifact_ingest_read_failure(err)),
        }

        let namespace_dir = layout.namespace_dir_path(kind);
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
    pub artifact_kind: CanonicalArtifactKind,
    pub canonical_repo_relative_path: String,
    pub packet_required: bool,
}

fn record_ingest_issue(
    issues: &mut Vec<ArtifactIngestIssue>,
    kind: ArtifactIngestIssueKind,
    artifact_kind: CanonicalArtifactKind,
    canonical_repo_relative_path: &str,
) {
    issues.push(ArtifactIngestIssue {
        kind,
        artifact_kind,
        canonical_repo_relative_path: canonical_repo_relative_path.to_owned(),
        packet_required: descriptor_for(artifact_kind).packet_required,
    });
}

fn load_one(
    layout: CanonicalLayout<'_>,
    kind: CanonicalArtifactKind,
    issues: &mut Vec<ArtifactIngestIssue>,
) -> CanonicalArtifact {
    let workspace = layout.workspace();
    let artifact_path = layout.artifact_path(kind);
    let descriptor = descriptor_for_layout(layout, kind);

    let meta = match workspace.metadata_no_follow(&artifact_path) {
        Ok(meta) => meta,
        Err(_err) => {
            record_ingest_issue(
                issues,
                ArtifactIngestIssueKind::CanonicalArtifactReadError,
                kind,
                descriptor.relative_path,
            );
            return missing_one(layout, kind);
        }
    };

    if meta.is_none() {
        return missing_one(layout, kind);
    }

    let meta = meta.expect("meta");
    if meta.file_type().is_symlink() {
        record_ingest_issue(
            issues,
            ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed,
            kind,
            descriptor.relative_path,
        );
        return missing_one(layout, kind);
    }
    if !meta.is_file() {
        record_ingest_issue(
            issues,
            ArtifactIngestIssueKind::CanonicalArtifactReadError,
            kind,
            descriptor.relative_path,
        );
        return missing_one(layout, kind);
    }

    let trusted_file = match workspace.trusted_read(&artifact_path) {
        Ok(trusted_file) => trusted_file,
        Err(RepoRelativeFileAccessError::SymlinkNotAllowed(_)) => {
            record_ingest_issue(
                issues,
                ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed,
                kind,
                descriptor.relative_path,
            );
            return missing_one(layout, kind);
        }
        Err(
            RepoRelativeFileAccessError::Missing(_)
            | RepoRelativeFileAccessError::NotRegularFile(_)
            | RepoRelativeFileAccessError::ReadFailure { .. },
        ) => {
            record_ingest_issue(
                issues,
                ArtifactIngestIssueKind::CanonicalArtifactReadError,
                kind,
                descriptor.relative_path,
            );
            return missing_one(layout, kind);
        }
        Err(RepoRelativeFileAccessError::InvalidPath(_)) => {
            unreachable!("canonical artifact paths should stay repo-relative")
        }
    };

    let bytes = match trusted_file.read_bytes() {
        Ok(bytes) => bytes,
        Err(_) => {
            record_ingest_issue(
                issues,
                ArtifactIngestIssueKind::CanonicalArtifactReadError,
                kind,
                descriptor.relative_path,
            );
            return missing_one(layout, kind);
        }
    };

    let byte_len = bytes.len() as u64;
    let matches_setup_starter_template = matches_setup_starter_template(kind, &bytes);
    let presence = if byte_len == 0 {
        ArtifactPresence::PresentEmpty
    } else {
        ArtifactPresence::PresentNonEmpty
    };

    let content_sha256 = Some(sha256_hex(&bytes));

    CanonicalArtifact {
        identity: CanonicalArtifactIdentity {
            kind,
            relative_path: descriptor.relative_path.to_owned(),
            packet_required: descriptor.packet_required,
            baseline_required: descriptor.baseline_required,
            setup_scaffolded: descriptor.setup_scaffolded,
            presence,
            byte_len: Some(byte_len),
            content_sha256,
            matches_setup_starter_template,
        },
        bytes: Some(bytes),
    }
}

fn missing_one(layout: CanonicalLayout<'_>, kind: CanonicalArtifactKind) -> CanonicalArtifact {
    let descriptor = descriptor_for_layout(layout, kind);
    CanonicalArtifact {
        identity: CanonicalArtifactIdentity {
            kind,
            relative_path: descriptor.relative_path.to_owned(),
            packet_required: descriptor.packet_required,
            baseline_required: descriptor.baseline_required,
            setup_scaffolded: descriptor.setup_scaffolded,
            presence: ArtifactPresence::Missing,
            byte_len: None,
            content_sha256: None,
            matches_setup_starter_template: false,
        },
        bytes: None,
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

fn descriptor_for_layout(
    layout: CanonicalLayout<'_>,
    kind: CanonicalArtifactKind,
) -> CanonicalArtifactDescriptor {
    let mut descriptor = *descriptor_for(kind);
    if layout.contract() == *default_canonical_layout_contract() {
        descriptor.relative_path = kind.relative_path();
    } else {
        descriptor.relative_path = layout.artifact_relative_path(kind);
        descriptor.namespace_dir = layout.namespace_dir(kind);
    }
    descriptor
}

fn descriptor_for(kind: CanonicalArtifactKind) -> &'static CanonicalArtifactDescriptor {
    CANONICAL_ARTIFACT_DESCRIPTORS
        .iter()
        .find(|descriptor| descriptor.kind == kind)
        .expect("canonical artifact descriptor should exist")
}

fn artifact_ingest_read_failure(err: RepoRelativeMetadataReadError) -> ArtifactIngestError {
    ArtifactIngestError::ReadFailure {
        path: err.path,
        source: err.source,
    }
}
