use crate::canonical_repo_support::{CanonicalWorkspace, RepoRelativeFileAccessError};
use crate::charter_observation::{
    ensure_charter_observation_stable, load_selected_charter_with_limit,
    CanonicalCharterObservation, CanonicalCharterProjection,
};
use crate::definition_identity::{
    parse_definition_yaml, ExactDefinitionRef, RegistryLoadErrorKind, MAX_SOURCE_DOCUMENT_BYTES,
    MAX_TOTAL_SOURCE_BYTES,
};
use crate::instance_profile::SymbolicId;
use crate::profile_decision::{ArtifactApplicability, ResolvedProfileDecisions};
use crate::project_context_artifact::{
    parse_canonical_project_context, project_context_rendered_fingerprint,
    project_context_source_fingerprint, render_project_context_markdown,
    selected_project_context_decision, CanonicalProjectContext, ProjectContextArtifactErrorKind,
};
use crate::DefinitionFingerprint;
use serde::Serialize;
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactInspectionStatus {
    Missing,
    StructurallyValid,
    StructurallyInvalid,
    UnsafePath,
    Unreadable,
    NotInspected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactInspectionReason {
    PresentAndStructurallyValid,
    RequiredPathMissing,
    OptionalPathMissing,
    ConditionalEvidenceUnavailablePathMissing,
    ConditionalEvidenceUnavailablePathPresent,
    YamlSyntaxInvalid,
    DuplicateYamlKey,
    DocumentNotObject,
    StructuralValidationFailed,
    DocumentLimitExceeded,
    AggregateReadLimitExceeded,
    SymlinkRefused,
    NonRegularFileRefused,
    UnsafeRepositoryPath,
    UnsupportedPlatformStrictRead,
    RepositoryReadFailed,
    TypedDecodeFailed,
    RenderedViewRefused,
    ObservationChangedDuringInspection,
}

/// The downstream-safe projection of a validated Project Context observation.
///
/// Raw source bytes, descriptor identity, and the retained observation type are
/// engine-private:
///
/// ```compile_fail
/// use handbook_engine::CanonicalProjectContextObservation;
/// fn retained_observation_is_engine_private(_: &CanonicalProjectContextObservation) {}
/// ```
///
/// ```compile_fail
/// use handbook_engine::CanonicalProjectContextProjection;
/// fn raw_bytes_are_engine_private(value: &CanonicalProjectContextProjection) {
///     let _ = value.source_bytes();
/// }
/// ```
///
/// ```compile_fail
/// use handbook_engine::CanonicalProjectContextProjection;
/// fn file_identity_is_engine_private(value: &CanonicalProjectContextProjection) {
///     let _ = value.file_identity;
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalProjectContextProjection {
    canonical_path: String,
    record: CanonicalProjectContext,
    source_byte_length: usize,
    rendered_bytes: Vec<u8>,
    source_fingerprint: DefinitionFingerprint,
    rendered_output_fingerprint: DefinitionFingerprint,
}

impl CanonicalProjectContextProjection {
    pub fn canonical_path(&self) -> &str {
        &self.canonical_path
    }
    pub fn record(&self) -> &CanonicalProjectContext {
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

struct CanonicalProjectContextObservation {
    projection: CanonicalProjectContextProjection,
    #[cfg(any(unix, windows))]
    source_bytes: Vec<u8>,
    #[cfg(unix)]
    file_identity: crate::canonical_repo_support::TrustedRepoFileIdentity,
    #[cfg(windows)]
    source_file: crate::canonical_repo_support::TrustedRepoFile,
}

impl CanonicalProjectContextObservation {
    fn projection(&self) -> &CanonicalProjectContextProjection {
        &self.projection
    }
}

impl std::fmt::Debug for CanonicalProjectContextObservation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CanonicalProjectContextObservation")
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectedProjectContextLoadError {
    canonical_path: String,
    status: ArtifactInspectionStatus,
    reason: ArtifactInspectionReason,
}

impl SelectedProjectContextLoadError {
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

#[derive(Debug)]
pub struct ArtifactInspection {
    instance_id: SymbolicId,
    canonical_path: String,
    applicability: ArtifactApplicability,
    status: ArtifactInspectionStatus,
    reason: ArtifactInspectionReason,
    charter_observation: Option<CanonicalCharterObservation>,
    project_context_observation: Option<CanonicalProjectContextObservation>,
}

impl ArtifactInspection {
    pub fn instance_id(&self) -> &SymbolicId {
        &self.instance_id
    }
    pub fn canonical_path(&self) -> &str {
        &self.canonical_path
    }
    pub fn applicability(&self) -> ArtifactApplicability {
        self.applicability
    }
    pub fn status(&self) -> ArtifactInspectionStatus {
        self.status
    }
    pub fn reason(&self) -> ArtifactInspectionReason {
        self.reason
    }
    pub fn charter_projection(&self) -> Option<&CanonicalCharterProjection> {
        self.charter_observation
            .as_ref()
            .map(CanonicalCharterObservation::projection)
    }
    pub fn project_context_projection(&self) -> Option<&CanonicalProjectContextProjection> {
        self.project_context_observation
            .as_ref()
            .map(CanonicalProjectContextObservation::projection)
    }
}

#[derive(Debug)]
pub struct ProfileInspectionReport {
    profile_ref: ExactDefinitionRef,
    profile_fingerprint: crate::DefinitionFingerprint,
    artifacts: Vec<ArtifactInspection>,
}

impl ProfileInspectionReport {
    pub fn profile_ref(&self) -> &ExactDefinitionRef {
        &self.profile_ref
    }
    pub fn profile_fingerprint(&self) -> &crate::DefinitionFingerprint {
        &self.profile_fingerprint
    }
    pub fn artifacts(&self) -> &[ArtifactInspection] {
        &self.artifacts
    }
}

pub fn inspect_profile_repository(
    repo_root: impl AsRef<Path>,
    decisions: &ResolvedProfileDecisions,
) -> ProfileInspectionReport {
    inspect_profile_repository_with_stability_hook(repo_root, decisions, || {})
}

/// Runs repository inspection with a test hook immediately before the final
/// retained-observation stability check. The hook receives no observation
/// bytes or file identity.
#[doc(hidden)]
pub fn inspect_profile_repository_with_stability_hook(
    repo_root: impl AsRef<Path>,
    decisions: &ResolvedProfileDecisions,
    before_final_stability: impl FnOnce(),
) -> ProfileInspectionReport {
    let workspace = CanonicalWorkspace::new(repo_root.as_ref());
    let mut consumed_bytes = 0usize;
    let mut aggregate_exhausted = false;
    let mut artifacts = Vec::with_capacity(decisions.artifact_decisions().len());

    for decision in decisions.artifact_decisions() {
        if consumed_bytes >= MAX_TOTAL_SOURCE_BYTES {
            aggregate_exhausted = true;
        }
        let mut charter_observation = None;
        let mut project_context_observation = None;
        let (status, reason) = if aggregate_exhausted {
            aggregate_limit()
        } else if decision.instance_id().as_str() == "project_authority" {
            let remaining = MAX_TOTAL_SOURCE_BYTES.saturating_sub(consumed_bytes);
            let read_limit = remaining.min(MAX_SOURCE_DOCUMENT_BYTES);
            let mut bounded_read = None;
            let load_result = load_selected_charter_with_limit(
                &workspace,
                decisions,
                decision.canonical_path(),
                read_limit,
                |source_byte_length, exceeded| {
                    bounded_read = Some((source_byte_length, exceeded));
                },
            );
            if let Some((source_byte_length, exceeded)) = bounded_read {
                let charged_bytes = if exceeded {
                    read_limit
                } else {
                    source_byte_length
                };
                consumed_bytes = consumed_bytes
                    .saturating_add(charged_bytes)
                    .min(MAX_TOTAL_SOURCE_BYTES);
                if exceeded && remaining < MAX_SOURCE_DOCUMENT_BYTES {
                    aggregate_exhausted = true;
                }
            }
            match load_result {
                Ok(observation) => {
                    charter_observation = Some(observation);
                    structurally_valid(decision.applicability())
                }
                Err(error)
                    if error.reason() == ArtifactInspectionReason::DocumentLimitExceeded
                        && remaining < MAX_SOURCE_DOCUMENT_BYTES =>
                {
                    aggregate_exhausted = true;
                    aggregate_limit()
                }
                Err(error) => (error.status(), error.reason()),
            }
        } else if decision.instance_id().as_str() == "project_context" {
            let remaining = MAX_TOTAL_SOURCE_BYTES.saturating_sub(consumed_bytes);
            let read_limit = remaining.min(MAX_SOURCE_DOCUMENT_BYTES);
            let mut bounded_read = None;
            let load_result = load_selected_project_context_with_limit(
                &workspace,
                decisions,
                decision.canonical_path(),
                read_limit,
                |source_byte_length, exceeded| {
                    bounded_read = Some((source_byte_length, exceeded));
                },
            );
            if let Some((source_byte_length, exceeded)) = bounded_read {
                let charged_bytes = if exceeded {
                    read_limit
                } else {
                    source_byte_length
                };
                consumed_bytes = consumed_bytes
                    .saturating_add(charged_bytes)
                    .min(MAX_TOTAL_SOURCE_BYTES);
                if exceeded && remaining < MAX_SOURCE_DOCUMENT_BYTES {
                    aggregate_exhausted = true;
                }
            }
            match load_result {
                Ok(observation) => {
                    project_context_observation = Some(observation);
                    structurally_valid(decision.applicability())
                }
                Err(error)
                    if error.reason == ArtifactInspectionReason::DocumentLimitExceeded
                        && remaining < MAX_SOURCE_DOCUMENT_BYTES =>
                {
                    aggregate_exhausted = true;
                    aggregate_limit()
                }
                Err(error) => (error.status, error.reason),
            }
        } else {
            let normalized = match workspace.normalize_repo_relative(decision.canonical_path()) {
                Ok(normalized) => normalized,
                Err(_) => {
                    artifacts.push(inspection(
                        decision.instance_id().clone(),
                        decision.canonical_path(),
                        decision.applicability(),
                        ArtifactInspectionStatus::UnsafePath,
                        ArtifactInspectionReason::UnsafeRepositoryPath,
                        None,
                        None,
                    ));
                    continue;
                }
            };
            match workspace.trusted_read_strict(&normalized) {
                Err(error) => map_open_error(error, decision.applicability()),
                Ok(file) => {
                    let remaining = MAX_TOTAL_SOURCE_BYTES.saturating_sub(consumed_bytes);
                    if remaining == 0 {
                        aggregate_exhausted = true;
                        aggregate_limit()
                    } else {
                        let read_limit = remaining.min(MAX_SOURCE_DOCUMENT_BYTES);
                        match file.read_bytes_bounded(read_limit) {
                            Err(_) => repository_read_failed(),
                            Ok((_bytes, exceeded))
                                if exceeded && remaining < MAX_SOURCE_DOCUMENT_BYTES =>
                            {
                                aggregate_exhausted = true;
                                aggregate_limit()
                            }
                            Ok((_bytes, true)) => {
                                consumed_bytes = consumed_bytes
                                    .saturating_add(MAX_SOURCE_DOCUMENT_BYTES)
                                    .min(MAX_TOTAL_SOURCE_BYTES);
                                (
                                    ArtifactInspectionStatus::Unreadable,
                                    ArtifactInspectionReason::DocumentLimitExceeded,
                                )
                            }
                            Ok((bytes, false)) => {
                                consumed_bytes += bytes.len();
                                inspect_bytes(
                                    decisions,
                                    decision.instance_id(),
                                    decision.applicability(),
                                    &bytes,
                                )
                            }
                        }
                    }
                }
            }
        };
        artifacts.push(inspection(
            decision.instance_id().clone(),
            decision.canonical_path(),
            decision.applicability(),
            status,
            reason,
            charter_observation,
            project_context_observation,
        ));
    }

    finalize_project_context_observation(&workspace, &mut artifacts, before_final_stability);

    ProfileInspectionReport {
        profile_ref: decisions.profile_ref().clone(),
        profile_fingerprint: decisions.profile_fingerprint().clone(),
        artifacts,
    }
}

pub fn load_selected_project_context(
    repo_root: impl AsRef<Path>,
    decisions: &ResolvedProfileDecisions,
) -> Result<CanonicalProjectContextProjection, SelectedProjectContextLoadError> {
    let workspace = CanonicalWorkspace::new(repo_root.as_ref());
    let canonical_path = selected_project_context_path(decisions)?;
    let observation = load_selected_project_context_with_limit(
        &workspace,
        decisions,
        canonical_path,
        MAX_SOURCE_DOCUMENT_BYTES,
        |_, _| {},
    )?;
    ensure_project_context_observation_stable(&workspace, &observation)?;
    Ok(observation.projection)
}

fn selected_project_context_path(
    decisions: &ResolvedProfileDecisions,
) -> Result<&str, SelectedProjectContextLoadError> {
    selected_project_context_decision(decisions)
        .map(|decision| decision.canonical_path())
        .map_err(|_| {
            selected_load_error(
                "project_context",
                ArtifactInspectionStatus::StructurallyInvalid,
                ArtifactInspectionReason::StructuralValidationFailed,
            )
        })
}

fn load_selected_project_context_with_limit(
    workspace: &CanonicalWorkspace<'_>,
    decisions: &ResolvedProfileDecisions,
    canonical_path: &str,
    read_limit: usize,
    on_bounded_read: impl FnOnce(usize, bool),
) -> Result<CanonicalProjectContextObservation, SelectedProjectContextLoadError> {
    let selected = selected_project_context_decision(decisions).map_err(|_| {
        selected_load_error(
            canonical_path,
            ArtifactInspectionStatus::StructurallyInvalid,
            ArtifactInspectionReason::StructuralValidationFailed,
        )
    })?;
    if selected.canonical_path() != canonical_path {
        return Err(selected_load_error(
            canonical_path,
            ArtifactInspectionStatus::StructurallyInvalid,
            ArtifactInspectionReason::StructuralValidationFailed,
        ));
    }
    let normalized = workspace
        .normalize_repo_relative(canonical_path)
        .map_err(|_| {
            selected_load_error(
                canonical_path,
                ArtifactInspectionStatus::UnsafePath,
                ArtifactInspectionReason::UnsafeRepositoryPath,
            )
        })?;
    let file = workspace
        .trusted_read_strict(&normalized)
        .map_err(|error| {
            let (status, reason) = map_open_error(error, ArtifactApplicability::Required);
            selected_load_error(canonical_path, status, reason)
        })?;
    #[cfg(unix)]
    let file_identity = file.identity().map_err(|_| {
        selected_load_error(
            canonical_path,
            ArtifactInspectionStatus::Unreadable,
            ArtifactInspectionReason::RepositoryReadFailed,
        )
    })?;
    let (source_bytes, exceeded) = file.read_bytes_bounded(read_limit).map_err(|_| {
        selected_load_error(
            canonical_path,
            ArtifactInspectionStatus::Unreadable,
            ArtifactInspectionReason::RepositoryReadFailed,
        )
    })?;
    on_bounded_read(source_bytes.len(), exceeded);
    if exceeded {
        return Err(selected_load_error(
            canonical_path,
            ArtifactInspectionStatus::Unreadable,
            ArtifactInspectionReason::DocumentLimitExceeded,
        ));
    }
    let record = parse_canonical_project_context(decisions, &source_bytes).map_err(|error| {
        let reason = map_project_context_artifact_error(error.kind());
        selected_load_error(
            canonical_path,
            ArtifactInspectionStatus::StructurallyInvalid,
            reason,
        )
    })?;
    let rendered_bytes = render_project_context_markdown(&record).map_err(|_| {
        selected_load_error(
            canonical_path,
            ArtifactInspectionStatus::StructurallyInvalid,
            ArtifactInspectionReason::RenderedViewRefused,
        )
    })?;
    let observation = CanonicalProjectContextObservation {
        projection: CanonicalProjectContextProjection {
            canonical_path: canonical_path.to_owned(),
            source_byte_length: source_bytes.len(),
            source_fingerprint: project_context_source_fingerprint(&source_bytes),
            rendered_output_fingerprint: project_context_rendered_fingerprint(&rendered_bytes),
            record,
            rendered_bytes,
        },
        #[cfg(any(unix, windows))]
        source_bytes,
        #[cfg(unix)]
        file_identity,
        #[cfg(windows)]
        source_file: file,
    };

    Ok(observation)
}

fn map_project_context_artifact_error(
    kind: ProjectContextArtifactErrorKind,
) -> ArtifactInspectionReason {
    match kind {
        ProjectContextArtifactErrorKind::DuplicateKey => ArtifactInspectionReason::DuplicateYamlKey,
        ProjectContextArtifactErrorKind::SyntaxError => ArtifactInspectionReason::YamlSyntaxInvalid,
        ProjectContextArtifactErrorKind::NonObjectRoot => {
            ArtifactInspectionReason::DocumentNotObject
        }
        ProjectContextArtifactErrorKind::StructuralValidationFailed
        | ProjectContextArtifactErrorKind::SelectedContractMismatch
        | ProjectContextArtifactErrorKind::SelectedDecisionMissing => {
            ArtifactInspectionReason::StructuralValidationFailed
        }
        ProjectContextArtifactErrorKind::SourceLimitExceeded => {
            ArtifactInspectionReason::DocumentLimitExceeded
        }
        ProjectContextArtifactErrorKind::TypedDecodeFailed
        | ProjectContextArtifactErrorKind::SerializationFailed => {
            ArtifactInspectionReason::TypedDecodeFailed
        }
        ProjectContextArtifactErrorKind::RenderedViewRefused => {
            ArtifactInspectionReason::RenderedViewRefused
        }
    }
}

fn finalize_project_context_observation(
    workspace: &CanonicalWorkspace<'_>,
    artifacts: &mut [ArtifactInspection],
    before_final_stability: impl FnOnce(),
) {
    let has_retained_observation = artifacts.iter().any(|artifact| {
        artifact.charter_observation.is_some() || artifact.project_context_observation.is_some()
    });
    if !has_retained_observation {
        return;
    }

    before_final_stability();

    if let Some(charter) = artifacts
        .iter_mut()
        .find(|artifact| artifact.instance_id.as_str() == "project_authority")
    {
        if charter
            .charter_observation
            .as_ref()
            .is_some_and(|observation| {
                ensure_charter_observation_stable(workspace, observation).is_err()
            })
        {
            charter.status = ArtifactInspectionStatus::Unreadable;
            charter.reason = ArtifactInspectionReason::ObservationChangedDuringInspection;
            charter.charter_observation = None;
        }
    }

    let Some(project_context) = artifacts
        .iter_mut()
        .find(|artifact| artifact.instance_id.as_str() == "project_context")
    else {
        return;
    };
    let Some(observation) = project_context.project_context_observation.as_ref() else {
        return;
    };

    if ensure_project_context_observation_stable(workspace, observation).is_err() {
        project_context.status = ArtifactInspectionStatus::Unreadable;
        project_context.reason = ArtifactInspectionReason::ObservationChangedDuringInspection;
        project_context.project_context_observation = None;
    }
}

#[cfg(unix)]
fn ensure_project_context_observation_stable(
    workspace: &CanonicalWorkspace<'_>,
    observation: &CanonicalProjectContextObservation,
) -> Result<(), SelectedProjectContextLoadError> {
    let normalized = workspace
        .normalize_repo_relative(observation.projection.canonical_path())
        .map_err(|_| observation_changed(observation.projection.canonical_path()))?;
    let final_file = workspace
        .trusted_read_strict(&normalized)
        .map_err(|_| observation_changed(observation.projection.canonical_path()))?;
    let final_identity = final_file
        .identity()
        .map_err(|_| observation_changed(observation.projection.canonical_path()))?;
    let (final_bytes, exceeded) = final_file
        .read_bytes_bounded(MAX_SOURCE_DOCUMENT_BYTES)
        .map_err(|_| observation_changed(observation.projection.canonical_path()))?;
    if exceeded
        || final_identity != observation.file_identity
        || final_bytes != observation.source_bytes
    {
        return Err(observation_changed(observation.projection.canonical_path()));
    }
    Ok(())
}

#[cfg(windows)]
fn ensure_project_context_observation_stable(
    workspace: &CanonicalWorkspace<'_>,
    observation: &CanonicalProjectContextObservation,
) -> Result<(), SelectedProjectContextLoadError> {
    let _retained_share_guard = &observation.source_file;
    let normalized = workspace
        .normalize_repo_relative(observation.projection.canonical_path())
        .map_err(|_| observation_changed(observation.projection.canonical_path()))?;
    let final_file = workspace
        .trusted_read_strict(&normalized)
        .map_err(|_| observation_changed(observation.projection.canonical_path()))?;
    let (final_bytes, exceeded) = final_file
        .read_bytes_bounded(MAX_SOURCE_DOCUMENT_BYTES)
        .map_err(|_| observation_changed(observation.projection.canonical_path()))?;
    if exceeded || final_bytes != observation.source_bytes {
        return Err(observation_changed(observation.projection.canonical_path()));
    }
    Ok(())
}

#[cfg(all(not(unix), not(windows)))]
fn ensure_project_context_observation_stable(
    _workspace: &CanonicalWorkspace<'_>,
    _observation: &CanonicalProjectContextObservation,
) -> Result<(), SelectedProjectContextLoadError> {
    Ok(())
}

fn selected_load_error(
    canonical_path: &str,
    status: ArtifactInspectionStatus,
    reason: ArtifactInspectionReason,
) -> SelectedProjectContextLoadError {
    SelectedProjectContextLoadError {
        canonical_path: canonical_path.to_owned(),
        status,
        reason,
    }
}

#[cfg(any(unix, windows))]
fn observation_changed(canonical_path: &str) -> SelectedProjectContextLoadError {
    selected_load_error(
        canonical_path,
        ArtifactInspectionStatus::Unreadable,
        ArtifactInspectionReason::ObservationChangedDuringInspection,
    )
}

fn inspect_bytes(
    decisions: &ResolvedProfileDecisions,
    instance_id: &SymbolicId,
    applicability: ArtifactApplicability,
    bytes: &[u8],
) -> (ArtifactInspectionStatus, ArtifactInspectionReason) {
    let value = match parse_definition_yaml(bytes) {
        Ok(value) => value,
        Err(error) if error.kind() == RegistryLoadErrorKind::DuplicateKey => {
            return (
                ArtifactInspectionStatus::StructurallyInvalid,
                ArtifactInspectionReason::DuplicateYamlKey,
            )
        }
        Err(_) => {
            return (
                ArtifactInspectionStatus::StructurallyInvalid,
                ArtifactInspectionReason::YamlSyntaxInvalid,
            )
        }
    };
    if !value.is_object() {
        return (
            ArtifactInspectionStatus::StructurallyInvalid,
            ArtifactInspectionReason::DocumentNotObject,
        );
    }
    if decisions
        .registry()
        .validate_json(instance_id, &value)
        .is_err()
    {
        return (
            ArtifactInspectionStatus::StructurallyInvalid,
            ArtifactInspectionReason::StructuralValidationFailed,
        );
    }
    structurally_valid(applicability)
}

fn map_open_error(
    error: RepoRelativeFileAccessError,
    applicability: ArtifactApplicability,
) -> (ArtifactInspectionStatus, ArtifactInspectionReason) {
    match error {
        RepoRelativeFileAccessError::Missing(_) => match applicability {
            ArtifactApplicability::Required => (
                ArtifactInspectionStatus::Missing,
                ArtifactInspectionReason::RequiredPathMissing,
            ),
            ArtifactApplicability::Optional => (
                ArtifactInspectionStatus::Missing,
                ArtifactInspectionReason::OptionalPathMissing,
            ),
            ArtifactApplicability::Indeterminate => (
                ArtifactInspectionStatus::NotInspected,
                ArtifactInspectionReason::ConditionalEvidenceUnavailablePathMissing,
            ),
        },
        RepoRelativeFileAccessError::SymlinkNotAllowed(_) => (
            ArtifactInspectionStatus::UnsafePath,
            ArtifactInspectionReason::SymlinkRefused,
        ),
        RepoRelativeFileAccessError::NotRegularFile(_) => (
            ArtifactInspectionStatus::UnsafePath,
            ArtifactInspectionReason::NonRegularFileRefused,
        ),
        RepoRelativeFileAccessError::InvalidPath(_) => {
            #[cfg(any(unix, windows))]
            let reason = ArtifactInspectionReason::UnsafeRepositoryPath;
            #[cfg(all(not(unix), not(windows)))]
            let reason = ArtifactInspectionReason::UnsupportedPlatformStrictRead;
            (ArtifactInspectionStatus::UnsafePath, reason)
        }
        RepoRelativeFileAccessError::ReadFailure { .. } => repository_read_failed(),
    }
}

fn inspection(
    instance_id: SymbolicId,
    canonical_path: &str,
    applicability: ArtifactApplicability,
    status: ArtifactInspectionStatus,
    reason: ArtifactInspectionReason,
    charter_observation: Option<CanonicalCharterObservation>,
    project_context_observation: Option<CanonicalProjectContextObservation>,
) -> ArtifactInspection {
    ArtifactInspection {
        instance_id,
        canonical_path: canonical_path.to_owned(),
        applicability,
        status,
        reason,
        charter_observation,
        project_context_observation,
    }
}

fn structurally_valid(
    applicability: ArtifactApplicability,
) -> (ArtifactInspectionStatus, ArtifactInspectionReason) {
    (
        ArtifactInspectionStatus::StructurallyValid,
        match applicability {
            ArtifactApplicability::Indeterminate => {
                ArtifactInspectionReason::ConditionalEvidenceUnavailablePathPresent
            }
            ArtifactApplicability::Required | ArtifactApplicability::Optional => {
                ArtifactInspectionReason::PresentAndStructurallyValid
            }
        },
    )
}

fn aggregate_limit() -> (ArtifactInspectionStatus, ArtifactInspectionReason) {
    (
        ArtifactInspectionStatus::Unreadable,
        ArtifactInspectionReason::AggregateReadLimitExceeded,
    )
}

fn repository_read_failed() -> (ArtifactInspectionStatus, ArtifactInspectionReason) {
    (
        ArtifactInspectionStatus::Unreadable,
        ArtifactInspectionReason::RepositoryReadFailed,
    )
}

#[cfg(test)]
mod mapping_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn required_missing_open_maps_to_required_path_missing() {
        assert_eq!(
            map_open_error(
                RepoRelativeFileAccessError::Missing(PathBuf::from("missing")),
                ArtifactApplicability::Required,
            ),
            (
                ArtifactInspectionStatus::Missing,
                ArtifactInspectionReason::RequiredPathMissing,
            )
        );
    }

    #[test]
    fn optional_missing_open_maps_to_optional_path_missing() {
        assert_eq!(
            map_open_error(
                RepoRelativeFileAccessError::Missing(PathBuf::from("missing")),
                ArtifactApplicability::Optional,
            ),
            (
                ArtifactInspectionStatus::Missing,
                ArtifactInspectionReason::OptionalPathMissing,
            )
        );
    }

    #[test]
    fn indeterminate_missing_open_maps_to_condition_unavailable() {
        assert_eq!(
            map_open_error(
                RepoRelativeFileAccessError::Missing(PathBuf::from("missing")),
                ArtifactApplicability::Indeterminate,
            ),
            (
                ArtifactInspectionStatus::NotInspected,
                ArtifactInspectionReason::ConditionalEvidenceUnavailablePathMissing,
            )
        );
    }

    #[test]
    fn symlink_open_refusal_maps_to_symlink_refused() {
        assert_eq!(
            map_open_error(
                RepoRelativeFileAccessError::SymlinkNotAllowed(PathBuf::from("linked")),
                ArtifactApplicability::Required,
            ),
            (
                ArtifactInspectionStatus::UnsafePath,
                ArtifactInspectionReason::SymlinkRefused,
            )
        );
    }

    #[test]
    fn non_regular_open_refusal_maps_to_non_regular_file_refused() {
        assert_eq!(
            map_open_error(
                RepoRelativeFileAccessError::NotRegularFile(PathBuf::from("directory")),
                ArtifactApplicability::Required,
            ),
            (
                ArtifactInspectionStatus::UnsafePath,
                ArtifactInspectionReason::NonRegularFileRefused,
            )
        );
    }

    #[test]
    fn invalid_repository_path_maps_to_platform_specific_unsafe_reason() {
        let actual = map_open_error(
            RepoRelativeFileAccessError::InvalidPath("invalid".to_owned()),
            ArtifactApplicability::Required,
        );
        #[cfg(any(unix, windows))]
        assert_eq!(
            actual,
            (
                ArtifactInspectionStatus::UnsafePath,
                ArtifactInspectionReason::UnsafeRepositoryPath,
            )
        );
        #[cfg(all(not(unix), not(windows)))]
        assert_eq!(
            actual,
            (
                ArtifactInspectionStatus::UnsafePath,
                ArtifactInspectionReason::UnsupportedPlatformStrictRead,
            )
        );
    }

    #[test]
    fn read_failure_maps_to_bounded_repository_read_failed() {
        assert_eq!(
            map_open_error(
                RepoRelativeFileAccessError::ReadFailure {
                    path: PathBuf::from("unreadable"),
                    source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "secret"),
                },
                ArtifactApplicability::Required,
            ),
            (
                ArtifactInspectionStatus::Unreadable,
                ArtifactInspectionReason::RepositoryReadFailed,
            )
        );
    }

    #[test]
    fn project_context_closed_decode_and_renderer_failures_have_exact_reasons() {
        assert_eq!(
            map_project_context_artifact_error(ProjectContextArtifactErrorKind::TypedDecodeFailed),
            ArtifactInspectionReason::TypedDecodeFailed
        );
        assert_eq!(
            map_project_context_artifact_error(
                ProjectContextArtifactErrorKind::SerializationFailed
            ),
            ArtifactInspectionReason::TypedDecodeFailed
        );
        assert_eq!(
            map_project_context_artifact_error(
                ProjectContextArtifactErrorKind::RenderedViewRefused
            ),
            ArtifactInspectionReason::RenderedViewRefused
        );
    }
}
