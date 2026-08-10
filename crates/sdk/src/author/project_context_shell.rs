use super::{
    acquire_authoring_lock, format_repo_mutation_error, format_repo_write_path_error,
    project_context::{AuthorProjectContextRefusal, AuthorProjectContextRefusalKind},
    AuthoringLockError,
};
use crate::repo_file_access::write_repo_relative_bytes;
use std::path::Path;

const PROJECT_CONTEXT_AUTHORING_LOCK_PATH: &str = ".handbook/state/authoring/project_context.lock";

pub(super) fn with_project_context_authoring_lock<T, F>(
    repo_root: &Path,
    action: F,
) -> Result<T, AuthorProjectContextRefusal>
where
    F: FnOnce() -> Result<T, AuthorProjectContextRefusal>,
{
    let _lock = acquire_authoring_lock(repo_root, PROJECT_CONTEXT_AUTHORING_LOCK_PATH)
        .map_err(|error| map_authoring_lock_error(repo_root, error))?;
    action()
}

pub(super) fn write_canonical_project_context_yaml(
    repo_root: &Path,
    canonical_path: &str,
    bytes: &[u8],
) -> Result<(), AuthorProjectContextRefusal> {
    write_repo_relative_bytes(repo_root, canonical_path, bytes).map_err(|error| {
        mutation_refusal(
            format_repo_mutation_error(canonical_path, error),
            canonical_path,
            "repair the selected Project Context path and retry",
        )
    })
}

fn map_authoring_lock_error(
    repo_root: &Path,
    error: AuthoringLockError,
) -> AuthorProjectContextRefusal {
    match error {
        AuthoringLockError::WritePath(path_error) => mutation_refusal(
            format_repo_write_path_error(PROJECT_CONTEXT_AUTHORING_LOCK_PATH, path_error),
            "project-context authoring lock",
            "repair the blocked authoring lock and retry",
        ),
        AuthoringLockError::Io { lock_path, source } => mutation_refusal(
            format!(
                "failed to acquire exclusive project-context authoring lock at {}: {source}",
                lock_path.display()
            ),
            "project-context authoring lock",
            "wait for any in-progress authoring run to finish, then retry",
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
        broken_subject: broken_subject.to_owned(),
        next_safe_action: next_safe_action.to_owned(),
    }
}
