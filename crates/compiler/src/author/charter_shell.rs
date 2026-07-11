use super::{
    acquire_authoring_lock, baseline_authoring_eligibility, canonical_artifact_identity,
    charter::{AuthorCharterRefusal, AuthorCharterRefusalKind, AuthorCharterResult},
    format_repo_mutation_error, format_repo_write_path_error, validate_canonical_write_target,
    validate_system_root_for_authoring, AuthoringLockError, BaselineAuthoringEligibility,
    SystemRootAuthoringError,
};
use crate::canonical_artifacts::{CanonicalArtifactKind, CanonicalArtifacts};
use crate::layout::RepoLayoutRoot;
use crate::repo_file_access::write_repo_relative_bytes;
use std::path::Path;

pub(super) fn preflight_author_charter(repo_root: &Path) -> Result<(), AuthorCharterRefusal> {
    let artifacts = CanonicalArtifacts::load(repo_root).map_err(|err| AuthorCharterRefusal {
        kind: AuthorCharterRefusalKind::InvalidSystemRoot,
        summary: format!("failed to inspect canonical `.handbook` root: {err}"),
        broken_subject: "canonical `.handbook` root".to_string(),
        next_safe_action: "repair the canonical `.handbook` root and rerun `handbook setup`"
            .to_string(),
    })?;
    validate_authoring_preconditions(repo_root, &artifacts)?;
    Ok(())
}

pub(super) fn with_charter_authoring_lock<T, F>(
    repo_root: &Path,
    action: F,
) -> Result<T, AuthorCharterRefusal>
where
    F: FnOnce() -> Result<T, AuthorCharterRefusal>,
{
    let charter_layout = RepoLayoutRoot::new(repo_root).authoring().charter();
    let _lock = acquire_authoring_lock(repo_root, charter_layout.lock_path().as_str())
        .map_err(|err| map_authoring_lock_error(repo_root, err))?;
    action()
}

pub(super) fn write_canonical_charter_markdown(
    repo_root: &Path,
    markdown: &str,
) -> Result<AuthorCharterResult, AuthorCharterRefusal> {
    let charter_layout = RepoLayoutRoot::new(repo_root).authoring().charter();
    write_repo_relative_bytes(
        repo_root,
        charter_layout.canonical_target().as_str(),
        markdown.as_bytes(),
    )
    .map_err(|err| AuthorCharterRefusal {
        kind: AuthorCharterRefusalKind::MutationRefused,
        summary: format_repo_mutation_error(charter_layout.canonical_target_relative(), err),
        broken_subject: "canonical charter write target".to_string(),
        next_safe_action:
            "repair the blocked canonical charter path and retry `handbook author charter --from-inputs <path|->`"
                .to_string(),
    })?;

    Ok(AuthorCharterResult {
        canonical_repo_relative_path: charter_layout.canonical_target_relative(),
        bytes_written: markdown.len(),
    })
}

fn validate_authoring_preconditions(
    repo_root: &Path,
    artifacts: &CanonicalArtifacts,
) -> Result<(), AuthorCharterRefusal> {
    let charter_layout = RepoLayoutRoot::new(repo_root).authoring().charter();
    match validate_system_root_for_authoring(artifacts) {
        Ok(()) => {}
        Err(SystemRootAuthoringError::Missing) => {
            return Err(AuthorCharterRefusal {
                kind: AuthorCharterRefusalKind::MissingSystemRoot,
                summary:
                    "canonical `.handbook` root is missing; charter authoring requires setup first"
                        .to_string(),
                broken_subject: "canonical `.handbook` root".to_string(),
                next_safe_action: "run `handbook setup`".to_string(),
            })
        }
        Err(SystemRootAuthoringError::NotDir) => {
            return Err(AuthorCharterRefusal {
                kind: AuthorCharterRefusalKind::InvalidSystemRoot,
                summary: "canonical `.handbook` root exists but is not a directory".to_string(),
                broken_subject: "canonical `.handbook` root".to_string(),
                next_safe_action:
                    "repair the canonical `.handbook` root and rerun `handbook setup`".to_string(),
            })
        }
        Err(SystemRootAuthoringError::SymlinkNotAllowed) => {
            return Err(AuthorCharterRefusal {
                kind: AuthorCharterRefusalKind::InvalidSystemRoot,
                summary: "canonical `.handbook` root cannot be a symlink".to_string(),
                broken_subject: "canonical `.handbook` root".to_string(),
                next_safe_action: "remove the `.handbook` symlink and rerun `handbook setup`"
                    .to_string(),
            })
        }
    }

    let charter = canonical_artifact_identity(artifacts, CanonicalArtifactKind::Charter);
    if charter.kind != CanonicalArtifactKind::Charter {
        return Err(AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::ExistingCanonicalTruth,
            summary: "unexpected canonical artifact identity for charter authoring".to_string(),
            broken_subject: "canonical charter truth".to_string(),
            next_safe_action:
                "inspect canonical artifact metadata and retry `handbook author charter --from-inputs <path|->`"
                    .to_string(),
        });
    }

    match baseline_authoring_eligibility(artifacts, CanonicalArtifactKind::Charter) {
        BaselineAuthoringEligibility::Authorable => {}
        BaselineAuthoringEligibility::ExistingValidCanonicalTruth => {
            return Err(AuthorCharterRefusal {
                kind: AuthorCharterRefusalKind::ExistingCanonicalTruth,
                summary:
                    "canonical charter truth already exists as valid non-starter truth; `handbook author charter --from-inputs <path|->` refuses to overwrite authored canonical truth"
                        .to_string(),
                broken_subject: charter_layout.canonical_target_relative().to_string(),
                next_safe_action: format!(
                    "inspect `{}` instead of rerunning `handbook author charter --from-inputs <path|->`",
                    charter_layout.canonical_target_relative()
                ),
            });
        }
        BaselineAuthoringEligibility::RequiresSetupRefresh => {
            return Err(AuthorCharterRefusal {
                kind: AuthorCharterRefusalKind::MutationRefused,
                summary:
                    "canonical charter truth is unreadable or path-invalid; repair it with `handbook setup refresh` before rerunning `handbook author charter --from-inputs <path|->`"
                        .to_string(),
                broken_subject: charter_layout.canonical_target_relative().to_string(),
                next_safe_action: "run `handbook setup refresh`".to_string(),
            });
        }
    }

    validate_canonical_write_target(repo_root, charter_layout.canonical_target().as_str())
        .map_err(|err| AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::MutationRefused,
            summary: format_repo_write_path_error(charter_layout.canonical_target_relative(), err),
            broken_subject: "canonical charter write target".to_string(),
            next_safe_action:
                "repair the blocked canonical charter path and retry `handbook author charter --from-inputs <path|->`"
                    .to_string(),
        })?;

    Ok(())
}

fn map_authoring_lock_error(repo_root: &Path, err: AuthoringLockError) -> AuthorCharterRefusal {
    let charter_layout = RepoLayoutRoot::new(repo_root).authoring().charter();
    match err {
        AuthoringLockError::WritePath(path_err) => AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::MutationRefused,
            summary: format_repo_write_path_error(charter_layout.lock_relative_path(), path_err),
            broken_subject: "charter authoring lock".to_string(),
            next_safe_action:
                "repair the blocked charter authoring lock path and retry `handbook author charter --from-inputs <path|->`"
                    .to_string(),
        },
        AuthoringLockError::Io { lock_path, source } => AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::MutationRefused,
            summary: format!(
                "failed to acquire exclusive charter authoring lock at {}: {source}",
                lock_path.display()
            ),
            broken_subject: "charter authoring lock".to_string(),
            next_safe_action:
                "wait for any in-progress `handbook author charter --from-inputs <path|->` run to finish or repair the lock path, then retry `handbook author charter --from-inputs <path|->`"
                    .to_string(),
        },
    }
}
