use crate::repo_file_access::{RepoRelativeMutationError, RepoRelativeWritePathError};
use crate::setup::{SetupDisposition, SetupMode, SetupRefusal, SetupRefusalKind};

const HANDBOOK_SETUP_COMMAND: &str = "handbook setup";
const HANDBOOK_SETUP_INIT_COMMAND: &str = "handbook setup init";
const HANDBOOK_SETUP_REFRESH_COMMAND: &str = "handbook setup refresh";
const HANDBOOK_DOCTOR_COMMAND: &str = "handbook doctor";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SetupRequestRefusalCopy {
    UnresolvedMode,
    InitWithRefreshFlags,
    AlreadyInitialized,
    MissingCanonicalRoot,
    InvalidCanonicalRoot,
    SymlinkCanonicalRoot,
}

pub(crate) fn request_refusal(copy: SetupRequestRefusalCopy) -> SetupRefusal {
    match copy {
        SetupRequestRefusalCopy::UnresolvedMode => refusal(
            SetupRefusalKind::InvalidRequest,
            "setup mode must resolve to init or refresh",
            "setup request",
            format!("retry `{HANDBOOK_SETUP_COMMAND}`"),
        ),
        SetupRequestRefusalCopy::InitWithRefreshFlags => refusal(
            SetupRefusalKind::InvalidRequest,
            "setup init does not accept refresh-only flags; retry without --rewrite or --reset-state",
            "setup request",
            format!(
                "retry `{HANDBOOK_SETUP_INIT_COMMAND}` without --rewrite or --reset-state"
            ),
        ),
        SetupRequestRefusalCopy::AlreadyInitialized => refusal(
            SetupRefusalKind::AlreadyInitialized,
            format!(
                "canonical .handbook root already exists; use `{HANDBOOK_SETUP_REFRESH_COMMAND}` instead"
            ),
            "canonical `.handbook` root",
            format!("run `{HANDBOOK_SETUP_REFRESH_COMMAND}`"),
        ),
        SetupRequestRefusalCopy::MissingCanonicalRoot => refusal(
            SetupRefusalKind::MissingCanonicalRoot,
            format!(
                "canonical .handbook root is missing; run `{HANDBOOK_SETUP_INIT_COMMAND}` first"
            ),
            "canonical `.handbook` root",
            format!("run `{HANDBOOK_SETUP_COMMAND}`"),
        ),
        SetupRequestRefusalCopy::InvalidCanonicalRoot => refusal(
            SetupRefusalKind::InvalidCanonicalRoot,
            format!(
                "canonical .handbook root is invalid; run `{HANDBOOK_SETUP_COMMAND}` to re-establish it"
            ),
            "canonical `.handbook` root",
            format!("run `{HANDBOOK_SETUP_COMMAND}`"),
        ),
        SetupRequestRefusalCopy::SymlinkCanonicalRoot => refusal(
            SetupRefusalKind::InvalidCanonicalRoot,
            format!(
                "canonical .handbook root must not be a symlink; run `{HANDBOOK_SETUP_COMMAND}` to re-establish it"
            ),
            "canonical `.handbook` root",
            format!("run `{HANDBOOK_SETUP_COMMAND}`"),
        ),
    }
}

pub(crate) fn mutation_refusal(
    mode: SetupMode,
    summary: impl Into<String>,
    broken_subject: impl Into<String>,
) -> SetupRefusal {
    refusal(
        SetupRefusalKind::MutationRefused,
        summary,
        broken_subject,
        format!(
            "repair the blocked target and rerun `{}`",
            rerun_setup_command(mode)
        ),
    )
}

pub(crate) fn next_safe_action(disposition: SetupDisposition) -> String {
    match disposition {
        SetupDisposition::Ready | SetupDisposition::Scaffolded => {
            format!("run `{HANDBOOK_DOCTOR_COMMAND}`")
        }
    }
}

pub(crate) fn format_repo_mutation_error(path: &str, err: RepoRelativeMutationError) -> String {
    match err {
        RepoRelativeMutationError::InvalidPath(reason) => {
            format!("write target `{path}` is invalid: {reason}")
        }
        RepoRelativeMutationError::ParentNotDirectory(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a directory",
                found.display()
            )
        }
        RepoRelativeMutationError::NotRegularFile(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a regular file target",
                found.display()
            )
        }
        RepoRelativeMutationError::SymlinkNotAllowed(found) => {
            format!(
                "write target `{path}` cannot be written through symlink {}",
                found.display()
            )
        }
        RepoRelativeMutationError::ReadFailure {
            path: found,
            source,
        }
        | RepoRelativeMutationError::WriteFailure {
            path: found,
            source,
        } => {
            format!(
                "failed to mutate write target `{path}` at {}: {source}",
                found.display()
            )
        }
    }
}

pub(crate) fn format_repo_write_path_error(path: &str, err: RepoRelativeWritePathError) -> String {
    match err {
        RepoRelativeWritePathError::InvalidPath(reason) => {
            format!("write target `{path}` is invalid: {reason}")
        }
        RepoRelativeWritePathError::ParentNotDirectory(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a directory",
                found.display()
            )
        }
        RepoRelativeWritePathError::NotRegularFile(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a regular file target",
                found.display()
            )
        }
        RepoRelativeWritePathError::SymlinkNotAllowed(found) => {
            format!(
                "write target `{path}` cannot be written through symlink {}",
                found.display()
            )
        }
        RepoRelativeWritePathError::ReadFailure {
            path: found,
            source,
        } => {
            format!(
                "failed to inspect write target `{path}` at {}: {source}",
                found.display()
            )
        }
    }
}

fn refusal(
    kind: SetupRefusalKind,
    summary: impl Into<String>,
    broken_subject: impl Into<String>,
    next_safe_action: impl Into<String>,
) -> SetupRefusal {
    SetupRefusal {
        kind,
        summary: summary.into(),
        broken_subject: broken_subject.into(),
        next_safe_action: next_safe_action.into(),
    }
}

fn rerun_setup_command(mode: SetupMode) -> &'static str {
    match mode {
        SetupMode::Auto | SetupMode::Init => HANDBOOK_SETUP_COMMAND,
        SetupMode::Refresh => HANDBOOK_SETUP_REFRESH_COMMAND,
    }
}
