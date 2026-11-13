#[cfg(windows)]
use crate::canonical_repo_support::TrustedRepoFile;
#[cfg(unix)]
use crate::canonical_repo_support::TrustedRepoFileIdentity;
use crate::canonical_repo_support::{CanonicalWorkspace, RepoRelativeFileAccessError};
use crate::charter_artifact::{
    charter_rendered_fingerprint, charter_source_fingerprint, parse_canonical_charter,
    render_canonical_charter_markdown, selected_charter_decision, CanonicalCharter,
    CharterArtifactErrorKind,
};
use crate::profile_decision::{ArtifactApplicability, ResolvedProfileDecisions};
use crate::profile_inspection::{ArtifactInspectionReason, ArtifactInspectionStatus};
use crate::{DefinitionFingerprint, MAX_SOURCE_DOCUMENT_BYTES};
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalCharterProjection {
    canonical_path: String,
    record: CanonicalCharter,
    source_byte_length: usize,
    rendered_bytes: Vec<u8>,
    source_fingerprint: DefinitionFingerprint,
    rendered_output_fingerprint: DefinitionFingerprint,
}

impl CanonicalCharterProjection {
    pub fn canonical_path(&self) -> &str {
        &self.canonical_path
    }

    pub fn record(&self) -> &CanonicalCharter {
        &self.record
    }

    pub fn source_byte_length(&self) -> usize {
        self.source_byte_length
    }

    pub fn rendered_bytes(&self) -> &[u8] {
        &self.rendered_bytes
    }

    pub fn rendered_byte_length(&self) -> usize {
        self.rendered_bytes.len()
    }

    pub fn source_fingerprint(&self) -> &DefinitionFingerprint {
        &self.source_fingerprint
    }

    pub fn rendered_output_fingerprint(&self) -> &DefinitionFingerprint {
        &self.rendered_output_fingerprint
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedCharterLoadError {
    canonical_path: String,
    status: ArtifactInspectionStatus,
    reason: ArtifactInspectionReason,
}

impl SelectedCharterLoadError {
    pub fn canonical_path(&self) -> &str {
        &self.canonical_path
    }

    pub fn status(&self) -> ArtifactInspectionStatus {
        self.status
    }

    pub fn reason(&self) -> ArtifactInspectionReason {
        self.reason
    }
}

pub(crate) struct CanonicalCharterObservation {
    projection: CanonicalCharterProjection,
    #[cfg(any(unix, windows))]
    source_bytes: Vec<u8>,
    #[cfg(unix)]
    file_identity: TrustedRepoFileIdentity,
    #[cfg(windows)]
    source_file: TrustedRepoFile,
}

impl CanonicalCharterObservation {
    pub(crate) fn projection(&self) -> &CanonicalCharterProjection {
        &self.projection
    }
}

impl std::fmt::Debug for CanonicalCharterObservation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CanonicalCharterObservation")
            .field("canonical_path", &self.projection.canonical_path)
            .field("source_byte_length", &self.projection.source_byte_length)
            .field(
                "rendered_byte_length",
                &self.projection.rendered_byte_length(),
            )
            .field("source_fingerprint", &self.projection.source_fingerprint)
            .field(
                "rendered_output_fingerprint",
                &self.projection.rendered_output_fingerprint,
            )
            .finish_non_exhaustive()
    }
}

pub fn load_selected_charter(
    repo_root: impl AsRef<Path>,
    decisions: &ResolvedProfileDecisions,
) -> Result<CanonicalCharterProjection, SelectedCharterLoadError> {
    let workspace = CanonicalWorkspace::new(repo_root.as_ref());
    let canonical_path = selected_charter_decision(decisions)
        .map_err(|_| invalid_selected_charter(".handbook/project/charter.yaml"))?
        .canonical_path();
    let observation = load_selected_charter_with_limit(
        &workspace,
        decisions,
        canonical_path,
        MAX_SOURCE_DOCUMENT_BYTES,
        |_, _| {},
    )?;
    ensure_charter_observation_stable(&workspace, &observation)?;
    Ok(observation.projection)
}

pub(crate) fn load_selected_charter_with_limit(
    workspace: &CanonicalWorkspace<'_>,
    decisions: &ResolvedProfileDecisions,
    canonical_path: &str,
    read_limit: usize,
    on_bounded_read: impl FnOnce(usize, bool),
) -> Result<CanonicalCharterObservation, SelectedCharterLoadError> {
    let selected = selected_charter_decision(decisions)
        .map_err(|_| invalid_selected_charter(canonical_path))?;
    if selected.canonical_path() != canonical_path
        || selected.applicability() != ArtifactApplicability::Required
    {
        return Err(invalid_selected_charter(canonical_path));
    }
    let normalized = workspace
        .normalize_repo_relative(canonical_path)
        .map_err(|_| unsafe_path(canonical_path))?;
    let file = workspace
        .trusted_read_strict(&normalized)
        .map_err(|error| map_open_error(canonical_path, error))?;
    #[cfg(unix)]
    let file_identity = file.identity().map_err(|_| {
        unreadable(
            canonical_path,
            ArtifactInspectionReason::RepositoryReadFailed,
        )
    })?;
    let (source_bytes, exceeded) = file.read_bytes_bounded(read_limit).map_err(|_| {
        unreadable(
            canonical_path,
            ArtifactInspectionReason::RepositoryReadFailed,
        )
    })?;
    on_bounded_read(source_bytes.len(), exceeded);
    if exceeded {
        return Err(unreadable(
            canonical_path,
            ArtifactInspectionReason::DocumentLimitExceeded,
        ));
    }
    let record = parse_canonical_charter(decisions, &source_bytes).map_err(|error| {
        SelectedCharterLoadError {
            canonical_path: canonical_path.to_owned(),
            status: ArtifactInspectionStatus::StructurallyInvalid,
            reason: map_artifact_error(error.kind()),
        }
    })?;
    let rendered_bytes =
        render_canonical_charter_markdown(&record).map_err(|_| SelectedCharterLoadError {
            canonical_path: canonical_path.to_owned(),
            status: ArtifactInspectionStatus::StructurallyInvalid,
            reason: ArtifactInspectionReason::RenderedViewRefused,
        })?;
    Ok(CanonicalCharterObservation {
        projection: CanonicalCharterProjection {
            canonical_path: canonical_path.to_owned(),
            source_byte_length: source_bytes.len(),
            source_fingerprint: charter_source_fingerprint(&source_bytes),
            rendered_output_fingerprint: charter_rendered_fingerprint(&rendered_bytes),
            record,
            rendered_bytes,
        },
        #[cfg(any(unix, windows))]
        source_bytes,
        #[cfg(unix)]
        file_identity,
        #[cfg(windows)]
        source_file: file,
    })
}

#[cfg(unix)]
pub(crate) fn ensure_charter_observation_stable(
    workspace: &CanonicalWorkspace<'_>,
    observation: &CanonicalCharterObservation,
) -> Result<(), SelectedCharterLoadError> {
    let path = observation.projection.canonical_path();
    let normalized = workspace
        .normalize_repo_relative(path)
        .map_err(|_| observation_changed(path))?;
    let final_file = workspace
        .trusted_read_strict(&normalized)
        .map_err(|_| observation_changed(path))?;
    let final_identity = final_file
        .identity()
        .map_err(|_| observation_changed(path))?;
    let (final_bytes, exceeded) = final_file
        .read_bytes_bounded(MAX_SOURCE_DOCUMENT_BYTES)
        .map_err(|_| observation_changed(path))?;
    if exceeded
        || final_identity != observation.file_identity
        || final_bytes != observation.source_bytes
    {
        return Err(observation_changed(path));
    }
    Ok(())
}

#[cfg(windows)]
pub(crate) fn ensure_charter_observation_stable(
    workspace: &CanonicalWorkspace<'_>,
    observation: &CanonicalCharterObservation,
) -> Result<(), SelectedCharterLoadError> {
    let _retained_share_guard = &observation.source_file;
    let path = observation.projection.canonical_path();
    let normalized = workspace
        .normalize_repo_relative(path)
        .map_err(|_| observation_changed(path))?;
    let final_file = workspace
        .trusted_read_strict(&normalized)
        .map_err(|_| observation_changed(path))?;
    let (final_bytes, exceeded) = final_file
        .read_bytes_bounded(MAX_SOURCE_DOCUMENT_BYTES)
        .map_err(|_| observation_changed(path))?;
    if exceeded || final_bytes != observation.source_bytes {
        return Err(observation_changed(path));
    }
    Ok(())
}

#[cfg(all(not(unix), not(windows)))]
pub(crate) fn ensure_charter_observation_stable(
    _workspace: &CanonicalWorkspace<'_>,
    _observation: &CanonicalCharterObservation,
) -> Result<(), SelectedCharterLoadError> {
    Err(unreadable(
        ".handbook/project/charter.yaml",
        ArtifactInspectionReason::UnsupportedPlatformStrictRead,
    ))
}

fn map_artifact_error(kind: CharterArtifactErrorKind) -> ArtifactInspectionReason {
    match kind {
        CharterArtifactErrorKind::SourceLimitExceeded => {
            ArtifactInspectionReason::DocumentLimitExceeded
        }
        CharterArtifactErrorKind::DuplicateKey => ArtifactInspectionReason::DuplicateYamlKey,
        CharterArtifactErrorKind::SyntaxError => ArtifactInspectionReason::YamlSyntaxInvalid,
        CharterArtifactErrorKind::NonObjectRoot => ArtifactInspectionReason::DocumentNotObject,
        CharterArtifactErrorKind::SelectedDecisionMissing
        | CharterArtifactErrorKind::SelectedContractMismatch
        | CharterArtifactErrorKind::StructuralValidationFailed
        | CharterArtifactErrorKind::SemanticValidationFailed => {
            ArtifactInspectionReason::StructuralValidationFailed
        }
        CharterArtifactErrorKind::TypedDecodeFailed
        | CharterArtifactErrorKind::SerializationFailed => {
            ArtifactInspectionReason::TypedDecodeFailed
        }
        CharterArtifactErrorKind::RenderedViewRefused => {
            ArtifactInspectionReason::RenderedViewRefused
        }
    }
}

fn map_open_error(
    canonical_path: &str,
    error: RepoRelativeFileAccessError,
) -> SelectedCharterLoadError {
    match error {
        RepoRelativeFileAccessError::Missing(_) => SelectedCharterLoadError {
            canonical_path: canonical_path.to_owned(),
            status: ArtifactInspectionStatus::Missing,
            reason: ArtifactInspectionReason::RequiredPathMissing,
        },
        RepoRelativeFileAccessError::InvalidPath(_) => unsafe_path(canonical_path),
        RepoRelativeFileAccessError::SymlinkNotAllowed(_) => SelectedCharterLoadError {
            canonical_path: canonical_path.to_owned(),
            status: ArtifactInspectionStatus::UnsafePath,
            reason: ArtifactInspectionReason::SymlinkRefused,
        },
        RepoRelativeFileAccessError::NotRegularFile(_) => SelectedCharterLoadError {
            canonical_path: canonical_path.to_owned(),
            status: ArtifactInspectionStatus::UnsafePath,
            reason: ArtifactInspectionReason::NonRegularFileRefused,
        },
        RepoRelativeFileAccessError::ReadFailure { .. } => unreadable(
            canonical_path,
            ArtifactInspectionReason::RepositoryReadFailed,
        ),
    }
}

fn invalid_selected_charter(canonical_path: &str) -> SelectedCharterLoadError {
    SelectedCharterLoadError {
        canonical_path: canonical_path.to_owned(),
        status: ArtifactInspectionStatus::StructurallyInvalid,
        reason: ArtifactInspectionReason::StructuralValidationFailed,
    }
}

fn unsafe_path(canonical_path: &str) -> SelectedCharterLoadError {
    SelectedCharterLoadError {
        canonical_path: canonical_path.to_owned(),
        status: ArtifactInspectionStatus::UnsafePath,
        reason: ArtifactInspectionReason::UnsafeRepositoryPath,
    }
}

fn unreadable(canonical_path: &str, reason: ArtifactInspectionReason) -> SelectedCharterLoadError {
    SelectedCharterLoadError {
        canonical_path: canonical_path.to_owned(),
        status: ArtifactInspectionStatus::Unreadable,
        reason,
    }
}

fn observation_changed(canonical_path: &str) -> SelectedCharterLoadError {
    unreadable(
        canonical_path,
        ArtifactInspectionReason::ObservationChangedDuringInspection,
    )
}
