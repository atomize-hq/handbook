#[cfg(unix)]
use super::project_context_shell::{
    with_project_context_authoring_lock, write_canonical_project_context_yaml,
};
use crate::author_route_api::ProjectContextInput;
use handbook_engine::{
    parse_canonical_project_context, render_project_context_markdown as render_engine_view,
    resolve_shipped_profile_decisions, serialize_canonical_project_context,
    CanonicalProjectContext, ProjectContextArtifactError, ProjectContextArtifactErrorKind,
    ResolvedProfileDecisions,
};
#[cfg(unix)]
use handbook_engine::{ArtifactInspectionReason, ArtifactInspectionStatus};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorProjectContextRefusalKind {
    MissingSystemRoot,
    InvalidSystemRoot,
    MalformedStructuredInput,
    IncompleteStructuredInput,
    ExistingCanonicalTruth,
    MutationRefused,
    UnsupportedPlatformStrictMutation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorProjectContextRefusal {
    pub kind: AuthorProjectContextRefusalKind,
    pub summary: String,
    pub broken_subject: String,
    pub next_safe_action: String,
}

impl std::fmt::Display for AuthorProjectContextRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.summary)
    }
}

impl std::error::Error for AuthorProjectContextRefusal {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorProjectContextResult {
    pub canonical_repo_relative_path: String,
    pub bytes_written: usize,
    pub source_fingerprint: String,
    pub rendered_output_fingerprint: String,
    pub rendered_media_type: &'static str,
}

pub(crate) fn validate_project_context_input(
    input: &ProjectContextInput,
) -> Result<(), AuthorProjectContextRefusal> {
    let decisions = bundled_decisions()?;
    let input = canonical_project_context(input);
    let yaml =
        serialize_canonical_project_context(&decisions, &input).map_err(map_artifact_error)?;
    let record = parse_canonical_project_context(&decisions, &yaml).map_err(map_artifact_error)?;
    render_engine_view(&record).map_err(map_artifact_error)?;
    Ok(())
}

pub(crate) fn preflight_author_project_context(
    _repo_root: impl AsRef<Path>,
) -> Result<(), AuthorProjectContextRefusal> {
    #[cfg(not(unix))]
    {
        let decisions = bundled_decisions()?;
        Err(unsupported_platform_refusal(selected_path(&decisions)?))
    }

    #[cfg(unix)]
    {
        preflight_author_project_context_unix(_repo_root.as_ref())
    }
}

pub(crate) fn author_project_context_from_input(
    repo_root: impl AsRef<Path>,
    input: &ProjectContextInput,
) -> Result<AuthorProjectContextResult, AuthorProjectContextRefusal> {
    let repo_root = repo_root.as_ref();
    let decisions = bundled_decisions()?;
    let canonical_path = selected_path(&decisions)?.to_owned();
    let input = canonical_project_context(input);
    let yaml =
        serialize_canonical_project_context(&decisions, &input).map_err(map_artifact_error)?;
    let record = parse_canonical_project_context(&decisions, &yaml).map_err(map_artifact_error)?;
    let _rendered = render_engine_view(&record).map_err(map_artifact_error)?;

    #[cfg(not(unix))]
    {
        let _ = repo_root;
        Err(unsupported_platform_refusal(&canonical_path))
    }

    #[cfg(unix)]
    {
        preflight_author_project_context_unix(repo_root)?;
        with_project_context_authoring_lock(repo_root, || {
            preflight_author_project_context_unix(repo_root)?;
            write_canonical_project_context_yaml(repo_root, &canonical_path, &yaml)?;
            Ok(AuthorProjectContextResult {
                canonical_repo_relative_path: canonical_path,
                bytes_written: yaml.len(),
                source_fingerprint: handbook_engine::project_context_source_fingerprint(&yaml)
                    .as_str()
                    .to_owned(),
                rendered_output_fingerprint: handbook_engine::project_context_rendered_fingerprint(
                    &_rendered,
                )
                .as_str()
                .to_owned(),
                rendered_media_type: "text/markdown",
            })
        })
    }
}

fn canonical_project_context(input: &ProjectContextInput) -> CanonicalProjectContext {
    CanonicalProjectContext {
        schema_id: input.schema_id.clone(),
        schema_version: input.schema_version.clone(),
        record_id: input.record_id.clone(),
        summary: input.summary.clone(),
        system_boundaries: input.system_boundaries.clone(),
        ownership: input.ownership.clone(),
        authoritative_references: input.authoritative_references.clone(),
        known_unknowns: input.known_unknowns.clone(),
    }
}

fn bundled_decisions() -> Result<ResolvedProfileDecisions, AuthorProjectContextRefusal> {
    resolve_shipped_profile_decisions(Path::new(".")).map_err(|_| AuthorProjectContextRefusal {
        kind: AuthorProjectContextRefusalKind::InvalidSystemRoot,
        summary: "failed to resolve the shipped Project Context contract".to_owned(),
        broken_subject: "shipped Project Context profile decision".to_owned(),
        next_safe_action: "repair the installed Handbook definition package and retry".to_owned(),
    })
}

fn selected_path(
    decisions: &ResolvedProfileDecisions,
) -> Result<&str, AuthorProjectContextRefusal> {
    decisions
        .artifact_decisions()
        .iter()
        .find(|decision| decision.instance_id().as_str() == "project_context")
        .map(|decision| decision.canonical_path())
        .ok_or_else(|| AuthorProjectContextRefusal {
            kind: AuthorProjectContextRefusalKind::InvalidSystemRoot,
            summary: "the shipped profile has no selected Project Context decision".to_owned(),
            broken_subject: "shipped Project Context profile decision".to_owned(),
            next_safe_action: "repair the installed Handbook definition package and retry"
                .to_owned(),
        })
}

#[cfg(unix)]
fn preflight_author_project_context_unix(
    repo_root: &Path,
) -> Result<(), AuthorProjectContextRefusal> {
    let system_root = repo_root.join(".handbook");
    let metadata = std::fs::symlink_metadata(&system_root).map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            AuthorProjectContextRefusal {
                kind: AuthorProjectContextRefusalKind::MissingSystemRoot,
                summary: "canonical `.handbook` root is missing; project-context authoring requires setup first".to_owned(),
                broken_subject: "canonical `.handbook` root".to_owned(),
                next_safe_action: "run `handbook setup`".to_owned(),
            }
        } else {
            AuthorProjectContextRefusal {
                kind: AuthorProjectContextRefusalKind::InvalidSystemRoot,
                summary: "failed to inspect the canonical `.handbook` root".to_owned(),
                broken_subject: "canonical `.handbook` root".to_owned(),
                next_safe_action: "repair the canonical `.handbook` root and retry".to_owned(),
            }
        }
    })?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(AuthorProjectContextRefusal {
            kind: AuthorProjectContextRefusalKind::InvalidSystemRoot,
            summary: "canonical `.handbook` root must be a real directory".to_owned(),
            broken_subject: "canonical `.handbook` root".to_owned(),
            next_safe_action: "repair the canonical `.handbook` root and retry".to_owned(),
        });
    }

    let decisions = bundled_decisions()?;
    let canonical_path = selected_path(&decisions)?;
    match handbook_engine::load_selected_project_context(repo_root, &decisions) {
        Ok(_) => Err(AuthorProjectContextRefusal {
            kind: AuthorProjectContextRefusalKind::ExistingCanonicalTruth,
            summary: "selected canonical Project Context truth already exists and is valid; authoring refuses to overwrite it".to_owned(),
            broken_subject: canonical_path.to_owned(),
            next_safe_action: format!("inspect `{canonical_path}` instead of overwriting it"),
        }),
        Err(error) if error.reason() == ArtifactInspectionReason::RequiredPathMissing => Ok(()),
        Err(error) if error.status() == ArtifactInspectionStatus::StructurallyInvalid => Ok(()),
        Err(error) => Err(AuthorProjectContextRefusal {
            kind: AuthorProjectContextRefusalKind::MutationRefused,
            summary: format!(
                "selected canonical Project Context cannot be safely inspected: {:?}",
                error.reason()
            ),
            broken_subject: canonical_path.to_owned(),
            next_safe_action: "repair the selected Project Context path and retry".to_owned(),
        }),
    }
}

fn map_artifact_error(error: ProjectContextArtifactError) -> AuthorProjectContextRefusal {
    let kind = match error.kind() {
        ProjectContextArtifactErrorKind::DuplicateKey
        | ProjectContextArtifactErrorKind::SyntaxError
        | ProjectContextArtifactErrorKind::NonObjectRoot
        | ProjectContextArtifactErrorKind::SourceLimitExceeded => {
            AuthorProjectContextRefusalKind::MalformedStructuredInput
        }
        ProjectContextArtifactErrorKind::SelectedDecisionMissing
        | ProjectContextArtifactErrorKind::SelectedContractMismatch
        | ProjectContextArtifactErrorKind::StructuralValidationFailed
        | ProjectContextArtifactErrorKind::TypedDecodeFailed
        | ProjectContextArtifactErrorKind::RenderedViewRefused
        | ProjectContextArtifactErrorKind::SerializationFailed => {
            AuthorProjectContextRefusalKind::IncompleteStructuredInput
        }
    };
    AuthorProjectContextRefusal {
        kind,
        summary: error.detail().to_owned(),
        broken_subject: "canonical project-context input".to_owned(),
        next_safe_action: "repair the canonical `1.0` Project Context YAML and retry".to_owned(),
    }
}

#[cfg(not(unix))]
fn unsupported_platform_refusal(canonical_path: &str) -> AuthorProjectContextRefusal {
    AuthorProjectContextRefusal {
        kind: AuthorProjectContextRefusalKind::UnsupportedPlatformStrictMutation,
        summary: "unsupported_platform_strict_mutation: Project Context mutation requires descriptor-relative no-follow filesystem operations unavailable on this platform".to_owned(),
        broken_subject: canonical_path.to_owned(),
        next_safe_action: "run the Project Context mutation on a supported Unix host".to_owned(),
    }
}
