pub mod charter;
mod charter_core;
mod charter_shell;
pub mod environment_inventory;
pub mod project_context;
#[path = "../template_library.rs"]
pub mod template_library;

pub use charter::{
    author_charter, author_charter_guided, is_unusably_vague_charter_text,
    normalize_charter_free_text, parse_charter_structured_input_yaml, preflight_author_charter,
    preflight_author_charter_from_input, render_charter_markdown, validate_charter_markdown,
    validate_charter_structured_input, AuthorCharterRefusal, AuthorCharterRefusalKind,
    AuthorCharterResult, CharterAudience, CharterBackwardCompatibility, CharterDebtTrackingInput,
    CharterDecisionRecordsInput, CharterDefaultImplicationsInput, CharterDeprecationPolicy,
    CharterDimensionInput, CharterDimensionName, CharterDomainInput, CharterExceptionsInput,
    CharterExpectedLifetime, CharterObservabilityThreshold, CharterOperationalRealityInput,
    CharterPostureInput, CharterProjectClassification, CharterProjectConstraintsInput,
    CharterProjectInput, CharterRequiredness, CharterRolloutControls, CharterRuntimeEnvironment,
    CharterStructuredInput, CharterSurface, CANONICAL_CHARTER_REPO_PATH,
    DEFAULT_EXCEPTION_RECORD_LOCATION,
};
pub use environment_inventory::{
    author_environment_inventory, preflight_author_environment_inventory,
    validate_environment_inventory_markdown, AuthorEnvironmentInventoryRefusal,
    AuthorEnvironmentInventoryRefusalKind, AuthorEnvironmentInventoryResult,
    CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH,
};
pub use project_context::{
    author_project_context, author_project_context_from_input,
    parse_project_context_structured_input_yaml, preflight_author_project_context,
    render_project_context_markdown, validate_project_context_markdown,
    validate_project_context_structured_input, AuthorProjectContextRefusal,
    AuthorProjectContextRefusalKind, AuthorProjectContextResult,
    ProjectContextClassificationImplicationsInput, ProjectContextConstraintsInput,
    ProjectContextDataRealityInput, ProjectContextEnvironmentsAndDeliveryInput,
    ProjectContextIntegrationInput, ProjectContextKnownUnknownInput,
    ProjectContextOperationalRealityInput, ProjectContextRepoCodebaseRealityInput,
    ProjectContextStructuredInput, ProjectContextSummaryInput, ProjectContextSystemBoundariesInput,
    ProjectContextValidationError, CANONICAL_PROJECT_CONTEXT_REPO_PATH,
};

use crate::baseline_validation::{baseline_artifact_validation, BaselineArtifactVerdict};
use crate::canonical_artifacts::{
    CanonicalArtifact, CanonicalArtifactIdentity, CanonicalArtifactKind, CanonicalArtifacts,
    SystemRootStatus,
};
use crate::repo_file_access::{
    resolve_repo_relative_write_path, RepoRelativeMutationError, RepoRelativeWritePathError,
};
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

const PROCESS_SUMMARY_LINE_LIMIT: usize = 3;
const PROCESS_SUMMARY_CHAR_LIMIT: usize = 600;
const PROCESS_SUMMARY_HIGH_SIGNAL_MARKERS: [&str; 5] = [
    "error:",
    "unauthorized",
    "incorrect api key",
    "missing bearer",
    "failed",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SystemRootAuthoringError {
    Missing,
    NotDir,
    SymlinkNotAllowed,
}

#[derive(Debug)]
enum AuthoringLockError {
    WritePath(RepoRelativeWritePathError),
    Io {
        lock_path: PathBuf,
        source: std::io::Error,
    },
}

fn validate_system_root_for_authoring(
    artifacts: &CanonicalArtifacts,
) -> Result<(), SystemRootAuthoringError> {
    match artifacts.system_root_status {
        SystemRootStatus::Ok => Ok(()),
        SystemRootStatus::Missing => Err(SystemRootAuthoringError::Missing),
        SystemRootStatus::NotDir => Err(SystemRootAuthoringError::NotDir),
        SystemRootStatus::SymlinkNotAllowed => Err(SystemRootAuthoringError::SymlinkNotAllowed),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BaselineAuthoringEligibility {
    Authorable,
    ExistingValidCanonicalTruth,
    RequiresSetupRefresh,
}

fn canonical_artifact_identity(
    artifacts: &CanonicalArtifacts,
    kind: CanonicalArtifactKind,
) -> &CanonicalArtifactIdentity {
    &canonical_artifact(artifacts, kind).identity
}

fn canonical_artifact(
    artifacts: &CanonicalArtifacts,
    kind: CanonicalArtifactKind,
) -> &CanonicalArtifact {
    match kind {
        CanonicalArtifactKind::Charter => &artifacts.charter,
        CanonicalArtifactKind::ProjectContext => &artifacts.project_context,
        CanonicalArtifactKind::EnvironmentInventory => &artifacts.environment_inventory,
        CanonicalArtifactKind::FeatureSpec => &artifacts.feature_spec,
    }
}

fn baseline_authoring_eligibility(
    artifacts: &CanonicalArtifacts,
    kind: CanonicalArtifactKind,
) -> BaselineAuthoringEligibility {
    let validation = baseline_artifact_validation(artifacts, kind)
        .expect("baseline authoring eligibility requires a baseline artifact");

    match validation.verdict {
        BaselineArtifactVerdict::Missing
        | BaselineArtifactVerdict::Empty
        | BaselineArtifactVerdict::StarterOwned
        | BaselineArtifactVerdict::SemanticallyInvalid { .. } => {
            BaselineAuthoringEligibility::Authorable
        }
        BaselineArtifactVerdict::ValidCanonicalTruth { .. } => {
            BaselineAuthoringEligibility::ExistingValidCanonicalTruth
        }
        BaselineArtifactVerdict::IngestInvalid => {
            BaselineAuthoringEligibility::RequiresSetupRefresh
        }
    }
}

fn validate_canonical_write_target(
    repo_root: &Path,
    canonical_repo_relative_path: &str,
) -> Result<(), RepoRelativeWritePathError> {
    resolve_repo_relative_write_path(repo_root, canonical_repo_relative_path).map(|_| ())
}

fn acquire_authoring_lock(
    repo_root: &Path,
    lock_repo_relative_path: &str,
) -> Result<AuthoringLockGuard, AuthoringLockError> {
    let lock_path = resolve_repo_relative_write_path(repo_root, lock_repo_relative_path)
        .map_err(AuthoringLockError::WritePath)?;

    if let Some(parent) = lock_path.parent() {
        std::fs::create_dir_all(parent).map_err(|source| AuthoringLockError::Io {
            lock_path: lock_path.clone(),
            source,
        })?;
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .map_err(|source| AuthoringLockError::Io {
            lock_path: lock_path.clone(),
            source,
        })?;

    lock_authoring_file(&file, libc::LOCK_EX).map_err(|source| AuthoringLockError::Io {
        lock_path: lock_path.clone(),
        source,
    })?;

    Ok(AuthoringLockGuard { file, lock_path })
}

#[cfg(unix)]
fn lock_authoring_file(file: &File, operation: libc::c_int) -> Result<(), std::io::Error> {
    use std::os::unix::io::AsRawFd;

    loop {
        let result = unsafe { libc::flock(file.as_raw_fd(), operation) };
        if result == 0 {
            return Ok(());
        }

        let error = std::io::Error::last_os_error();
        if error.kind() == std::io::ErrorKind::Interrupted {
            continue;
        }

        return Err(error);
    }
}

#[cfg(not(unix))]
fn lock_authoring_file(_file: &File, _operation: libc::c_int) -> Result<(), std::io::Error> {
    Ok(())
}

struct AuthoringLockGuard {
    file: File,
    lock_path: PathBuf,
}

impl Drop for AuthoringLockGuard {
    fn drop(&mut self) {
        #[cfg(unix)]
        let _ = lock_authoring_file(&self.file, libc::LOCK_UN);
        let _ = &self.lock_path;
    }
}

fn summarize_process_output(stdout: &[u8], stderr: &[u8]) -> String {
    let stdout = String::from_utf8_lossy(stdout);
    let stderr = String::from_utf8_lossy(stderr);

    let stderr = summarize_stderr_for_refusal(stderr.trim());
    if !stderr.is_empty() {
        return format!("; stderr: {stderr}");
    }

    let stdout = summarize_stream_tail(stdout.trim());
    if stdout.is_empty() {
        String::new()
    } else {
        format!("; stdout: {stdout}")
    }
}

fn truncate_for_summary(value: &str) -> String {
    if value.chars().count() <= PROCESS_SUMMARY_CHAR_LIMIT {
        value.to_string()
    } else {
        let tail = value
            .chars()
            .rev()
            .take(PROCESS_SUMMARY_CHAR_LIMIT)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<String>();
        format!("...{tail}")
    }
}

fn summarize_stderr_for_refusal(stderr: &str) -> String {
    let high_signal = collect_stream_summary_lines(stderr, true);
    if !high_signal.is_empty() {
        return truncate_for_summary(&high_signal.join(" | "));
    }

    summarize_stream_tail(stderr)
}

fn summarize_stream_tail(stream: &str) -> String {
    let lines = collect_stream_summary_lines(stream, false);
    if lines.is_empty() {
        String::new()
    } else {
        truncate_for_summary(&lines.join(" | "))
    }
}

fn collect_stream_summary_lines(stream: &str, prefer_high_signal: bool) -> Vec<String> {
    let mut selected = Vec::new();
    let mut seen = Vec::new();

    for raw_line in stream.lines().rev() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let normalized = normalize_process_summary_line(line);
        if normalized.is_empty() || seen.iter().any(|existing| existing == &normalized) {
            continue;
        }
        if prefer_high_signal && !is_high_signal_process_summary_line(&normalized) {
            continue;
        }

        seen.push(normalized);
        selected.push(line.to_string());
        if selected.len() == PROCESS_SUMMARY_LINE_LIMIT {
            break;
        }
    }

    selected.reverse();
    selected
}

fn normalize_process_summary_line(line: &str) -> String {
    line.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn is_high_signal_process_summary_line(normalized_line: &str) -> bool {
    PROCESS_SUMMARY_HIGH_SIGNAL_MARKERS
        .iter()
        .any(|marker| normalized_line.contains(marker))
}

fn render_exit_status(code: Option<i32>) -> String {
    match code {
        Some(code) => format!("exit code {code}"),
        None => "signal termination".to_string(),
    }
}

fn format_repo_mutation_error(path: &str, err: RepoRelativeMutationError) -> String {
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

fn format_repo_write_path_error(path: &str, err: RepoRelativeWritePathError) -> String {
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
