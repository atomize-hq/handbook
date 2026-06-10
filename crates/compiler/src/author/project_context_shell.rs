use super::{
    acquire_authoring_lock, baseline_authoring_eligibility, canonical_artifact_identity,
    format_repo_mutation_error, format_repo_write_path_error,
    project_context::{
        map_project_context_core_error, AuthorProjectContextRefusal,
        AuthorProjectContextRefusalKind, AuthorProjectContextResult,
    },
    project_context_core::{
        render_project_context_markdown as render_project_context_markdown_core,
        ProjectContextStructuredInput,
    },
    validate_canonical_write_target, validate_system_root_for_authoring, AuthoringLockError,
    BaselineAuthoringEligibility, SystemRootAuthoringError,
};
use crate::canonical_artifacts::{CanonicalArtifactKind, CanonicalArtifacts};
use crate::layout::RepoLayoutRoot;
use crate::repo_file_access::write_repo_relative_bytes;
use std::path::Path;
use time::macros::format_description;
use time::OffsetDateTime;

const AUTHOR_PROJECT_CONTEXT_NOW_UTC_ENV_VAR: &str = "HANDBOOK_AUTHOR_PROJECT_CONTEXT_NOW_UTC";
const NOW_UTC_FORMAT: &[time::format_description::FormatItem<'static>] =
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z");

pub(super) fn render_project_context_markdown(
    input: &ProjectContextStructuredInput,
) -> Result<String, AuthorProjectContextRefusal> {
    let now_utc = resolve_project_context_now_utc().map_err(render_timestamp_refusal)?;
    render_project_context_markdown_core(input, &now_utc).map_err(map_project_context_core_error)
}

pub(super) fn preflight_author_project_context(
    repo_root: &Path,
) -> Result<(), AuthorProjectContextRefusal> {
    let artifacts =
        CanonicalArtifacts::load(repo_root).map_err(|err| AuthorProjectContextRefusal {
            kind: AuthorProjectContextRefusalKind::InvalidSystemRoot,
            summary: format!("failed to inspect canonical `.handbook` root: {err}"),
            broken_subject: "canonical `.handbook` root".to_string(),
            next_safe_action: "repair the canonical `.handbook` root and rerun `handbook setup`"
                .to_string(),
        })?;
    validate_authoring_preconditions(repo_root, &artifacts)
}

pub(super) fn with_project_context_authoring_lock<T, F>(
    repo_root: &Path,
    action: F,
) -> Result<T, AuthorProjectContextRefusal>
where
    F: FnOnce() -> Result<T, AuthorProjectContextRefusal>,
{
    let project_context_layout = RepoLayoutRoot::new(repo_root).authoring().project_context();
    let _lock = acquire_authoring_lock(repo_root, project_context_layout.lock_path().as_str())
        .map_err(|err| map_authoring_lock_error(repo_root, err))?;
    action()
}

pub(super) fn write_canonical_project_context_markdown(
    repo_root: &Path,
    markdown: &str,
) -> Result<AuthorProjectContextResult, AuthorProjectContextRefusal> {
    let project_context_layout = RepoLayoutRoot::new(repo_root).authoring().project_context();
    write_repo_relative_bytes(
        repo_root,
        project_context_layout.canonical_target().as_str(),
        markdown.as_bytes(),
    )
    .map_err(|err| {
        mutation_refusal(
            format_repo_mutation_error(project_context_layout.canonical_target_relative(), err),
            "canonical project context write target",
            "repair the blocked canonical project context path and retry `handbook author project-context`",
        )
    })?;

    Ok(AuthorProjectContextResult {
        canonical_repo_relative_path: project_context_layout.canonical_target_relative(),
        bytes_written: markdown.len(),
    })
}

pub(super) fn project_context_inputs_required_refusal() -> AuthorProjectContextRefusal {
    AuthorProjectContextRefusal {
        kind: AuthorProjectContextRefusalKind::IncompleteStructuredInput,
        summary: "project-context authoring requires guided answers or explicit structured inputs; use `handbook author project-context --from-inputs <path|->` or provide guided answers through the CLI".to_string(),
        broken_subject: "structured project-context input".to_string(),
        next_safe_action:
            "repair the structured project-context input and retry `handbook author project-context --from-inputs <path|->`"
                .to_string(),
    }
}

fn validate_authoring_preconditions(
    repo_root: &Path,
    artifacts: &CanonicalArtifacts,
) -> Result<(), AuthorProjectContextRefusal> {
    let project_context_layout = RepoLayoutRoot::new(repo_root).authoring().project_context();
    match validate_system_root_for_authoring(artifacts) {
        Ok(()) => {}
        Err(SystemRootAuthoringError::Missing) => {
            return Err(AuthorProjectContextRefusal {
                kind: AuthorProjectContextRefusalKind::MissingSystemRoot,
                summary:
                    "canonical `.handbook` root is missing; project-context authoring requires setup first"
                        .to_string(),
                broken_subject: "canonical `.handbook` root".to_string(),
                next_safe_action: "run `handbook setup`".to_string(),
            });
        }
        Err(SystemRootAuthoringError::NotDir) => {
            return Err(AuthorProjectContextRefusal {
                kind: AuthorProjectContextRefusalKind::InvalidSystemRoot,
                summary: "canonical `.handbook` root exists but is not a directory".to_string(),
                broken_subject: "canonical `.handbook` root".to_string(),
                next_safe_action:
                    "repair the canonical `.handbook` root and rerun `handbook setup`".to_string(),
            });
        }
        Err(SystemRootAuthoringError::SymlinkNotAllowed) => {
            return Err(AuthorProjectContextRefusal {
                kind: AuthorProjectContextRefusalKind::InvalidSystemRoot,
                summary: "canonical `.handbook` root cannot be a symlink".to_string(),
                broken_subject: "canonical `.handbook` root".to_string(),
                next_safe_action: "remove the `.handbook` symlink and rerun `handbook setup`"
                    .to_string(),
            });
        }
    }

    let project_context =
        canonical_artifact_identity(artifacts, CanonicalArtifactKind::ProjectContext);
    if project_context.kind != CanonicalArtifactKind::ProjectContext {
        return Err(AuthorProjectContextRefusal {
            kind: AuthorProjectContextRefusalKind::ExistingCanonicalTruth,
            summary: "unexpected canonical artifact identity for project-context authoring"
                .to_string(),
            broken_subject: "canonical project context truth".to_string(),
            next_safe_action:
                "inspect canonical artifact metadata and retry `handbook author project-context`"
                    .to_string(),
        });
    }

    match baseline_authoring_eligibility(artifacts, CanonicalArtifactKind::ProjectContext) {
        BaselineAuthoringEligibility::Authorable => {}
        BaselineAuthoringEligibility::ExistingValidCanonicalTruth => {
            return Err(AuthorProjectContextRefusal {
                kind: AuthorProjectContextRefusalKind::ExistingCanonicalTruth,
                summary:
                    "canonical project context truth already exists as valid non-starter truth; `handbook author project-context` refuses to overwrite authored canonical truth"
                        .to_string(),
                broken_subject: project_context_layout.canonical_target_relative().to_string(),
                next_safe_action: format!(
                    "inspect `{}` instead of rerunning `handbook author project-context`",
                    project_context_layout.canonical_target_relative()
                ),
            });
        }
        BaselineAuthoringEligibility::RequiresSetupRefresh => {
            return Err(AuthorProjectContextRefusal {
                kind: AuthorProjectContextRefusalKind::MutationRefused,
                summary:
                    "canonical project context truth is unreadable or path-invalid; repair it with `handbook setup refresh` before rerunning `handbook author project-context`"
                        .to_string(),
                broken_subject: project_context_layout.canonical_target_relative().to_string(),
                next_safe_action: "run `handbook setup refresh`".to_string(),
            });
        }
    }

    validate_canonical_write_target(repo_root, project_context_layout.canonical_target().as_str())
        .map_err(|err| {
            mutation_refusal(
                format_repo_write_path_error(project_context_layout.canonical_target_relative(), err),
                "canonical project context write target",
                "repair the blocked canonical project context path and retry `handbook author project-context`",
            )
        })?;

    Ok(())
}

fn resolve_project_context_now_utc() -> Result<String, String> {
    if let Ok(value) = std::env::var(AUTHOR_PROJECT_CONTEXT_NOW_UTC_ENV_VAR) {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    OffsetDateTime::now_utc()
        .format(NOW_UTC_FORMAT)
        .map_err(|err| format!("failed to derive project-context render timestamp: {err}"))
}

fn render_timestamp_refusal(summary: String) -> AuthorProjectContextRefusal {
    mutation_refusal(
        summary,
        "project-context render timestamp",
        "repair the project-context timestamp runtime and retry `handbook author project-context`",
    )
}

fn map_authoring_lock_error(
    repo_root: &Path,
    err: AuthoringLockError,
) -> AuthorProjectContextRefusal {
    let project_context_layout = RepoLayoutRoot::new(repo_root).authoring().project_context();
    match err {
        AuthoringLockError::WritePath(path_err) => mutation_refusal(
            format_repo_write_path_error(project_context_layout.lock_relative_path(), path_err),
            "project-context authoring lock",
            "repair the blocked project-context authoring lock path and retry `handbook author project-context`",
        ),
        AuthoringLockError::Io { lock_path, source } => mutation_refusal(
            format!(
                "failed to acquire exclusive project-context authoring lock at {}: {source}",
                lock_path.display()
            ),
            "project-context authoring lock",
            "wait for any in-progress `handbook author project-context` run to finish or repair the lock path, then retry `handbook author project-context`",
        ),
    }
}

fn mutation_refusal(
    summary: String,
    broken_subject: &str,
    next_safe_action: &str,
) -> AuthorProjectContextRefusal {
    AuthorProjectContextRefusal {
        kind: AuthorProjectContextRefusalKind::MutationRefused,
        summary,
        broken_subject: broken_subject.to_string(),
        next_safe_action: next_safe_action.to_string(),
    }
}
