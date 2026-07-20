use super::{
    acquire_authoring_lock, baseline_authoring_eligibility, canonical_artifact_identity,
    environment_inventory::{
        map_environment_inventory_core_error, AuthorEnvironmentInventoryRefusal,
        AuthorEnvironmentInventoryRefusalKind, AuthorEnvironmentInventoryResult,
    },
    environment_inventory_core::{
        render_environment_inventory_markdown as render_environment_inventory_markdown_core,
        EnvironmentInventoryCoreError, EnvironmentInventoryCoreErrorKind,
        EnvironmentInventoryStructuredInput,
    },
    format_repo_mutation_error, format_repo_write_path_error, validate_canonical_write_target,
    validate_system_root_for_authoring, AuthoringLockError, BaselineAuthoringEligibility,
    SystemRootAuthoringError,
};
use crate::canonical_artifacts::{CanonicalArtifactKind, CanonicalArtifacts};
use crate::layout::RepoLayoutRoot;
use crate::repo_file_access::write_repo_relative_bytes;
use std::path::Path;
use time::macros::format_description;
use time::OffsetDateTime;

const AUTHOR_ENVIRONMENT_INVENTORY_NOW_UTC_ENV_VAR: &str =
    "HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_NOW_UTC";
const NOW_UTC_FORMAT: &[time::format_description::FormatItem<'static>] =
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z");

pub(super) struct EnvironmentInventoryUpstreamSelection {
    pub(super) charter_path: String,
    pub(super) charter_source_fingerprint: String,
    pub(super) charter_promotion_ref: String,
    pub(super) charter_lifecycle_transition_ref: String,
    pub(super) project_context_path: String,
}

pub(super) fn render_environment_inventory_markdown(
    input: &EnvironmentInventoryStructuredInput,
) -> Result<String, AuthorEnvironmentInventoryRefusal> {
    let now_utc = resolve_environment_inventory_now_utc().map_err(|summary| {
        map_environment_inventory_core_error(EnvironmentInventoryCoreError {
            kind: EnvironmentInventoryCoreErrorKind::DeterministicRenderFailed,
            summary,
        })
    })?;
    render_environment_inventory_markdown_core(input, &now_utc)
        .map_err(map_environment_inventory_core_error)
}

pub(super) fn preflight_author_environment_inventory(
    repo_root: &Path,
) -> Result<EnvironmentInventoryUpstreamSelection, AuthorEnvironmentInventoryRefusal> {
    let artifacts = CanonicalArtifacts::load_fixed_siblings(repo_root).map_err(|err| {
        AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::InvalidSystemRoot,
            summary: format!("failed to inspect canonical `.handbook` root: {err}"),
            broken_subject: "canonical `.handbook` root".to_string(),
            next_safe_action: "repair the canonical `.handbook` root and rerun `handbook setup`"
                .to_string(),
        }
    })?;

    validate_environment_inventory_authoring_preconditions(repo_root, &artifacts)?;
    let charter = required_selected_charter(repo_root)?;
    let project_context_path = required_project_context_path(repo_root)?;
    Ok(EnvironmentInventoryUpstreamSelection {
        charter_path: charter.canonical_path().to_owned(),
        charter_source_fingerprint: charter.source_fingerprint().to_string(),
        charter_promotion_ref: charter.promotion_ref,
        charter_lifecycle_transition_ref: charter.lifecycle_transition_ref,
        project_context_path,
    })
}

pub(super) fn with_environment_inventory_authoring_lock<T, F>(
    repo_root: &Path,
    action: F,
) -> Result<T, AuthorEnvironmentInventoryRefusal>
where
    F: FnOnce() -> Result<T, AuthorEnvironmentInventoryRefusal>,
{
    let environment_inventory_layout = RepoLayoutRoot::new(repo_root)
        .authoring()
        .environment_inventory();
    let _lock =
        acquire_authoring_lock(repo_root, environment_inventory_layout.lock_path().as_str())
            .map_err(|err| map_authoring_lock_error(repo_root, err))?;
    action()
}

pub(super) fn write_canonical_environment_inventory_markdown(
    repo_root: &Path,
    markdown: &str,
) -> Result<AuthorEnvironmentInventoryResult, AuthorEnvironmentInventoryRefusal> {
    let environment_inventory_layout = RepoLayoutRoot::new(repo_root)
        .authoring()
        .environment_inventory();
    write_repo_relative_bytes(
        repo_root,
        environment_inventory_layout.canonical_target().as_str(),
        markdown.as_bytes(),
    )
    .map_err(|err| AuthorEnvironmentInventoryRefusal {
        kind: AuthorEnvironmentInventoryRefusalKind::MutationRefused,
        summary: format_repo_mutation_error(
            environment_inventory_layout.canonical_target_relative(),
            err,
        ),
        broken_subject: "canonical environment inventory write target".to_string(),
        next_safe_action:
            "repair the blocked canonical environment inventory path and retry `handbook author environment-inventory --from-inputs <path|->`"
                .to_string(),
    })?;

    Ok(AuthorEnvironmentInventoryResult {
        canonical_repo_relative_path: environment_inventory_layout.canonical_target_relative(),
        bytes_written: markdown.len(),
    })
}

fn validate_environment_inventory_authoring_preconditions(
    repo_root: &Path,
    artifacts: &CanonicalArtifacts,
) -> Result<(), AuthorEnvironmentInventoryRefusal> {
    let environment_inventory_layout = RepoLayoutRoot::new(repo_root)
        .authoring()
        .environment_inventory();
    match validate_system_root_for_authoring(artifacts) {
        Ok(()) => {}
        Err(SystemRootAuthoringError::Missing) => {
            return Err(AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::MissingSystemRoot,
                summary:
                    "canonical `.handbook` root is missing; environment inventory authoring requires setup first"
                        .to_string(),
                broken_subject: "canonical `.handbook` root".to_string(),
                next_safe_action: "run `handbook setup`".to_string(),
            });
        }
        Err(SystemRootAuthoringError::NotDir) => {
            return Err(AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::InvalidSystemRoot,
                summary: "canonical `.handbook` root exists but is not a directory".to_string(),
                broken_subject: "canonical `.handbook` root".to_string(),
                next_safe_action:
                    "repair the canonical `.handbook` root and rerun `handbook setup`".to_string(),
            });
        }
        Err(SystemRootAuthoringError::SymlinkNotAllowed) => {
            return Err(AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::InvalidSystemRoot,
                summary: "canonical `.handbook` root cannot be a symlink".to_string(),
                broken_subject: "canonical `.handbook` root".to_string(),
                next_safe_action: "remove the `.handbook` symlink and rerun `handbook setup`"
                    .to_string(),
            });
        }
    }

    let environment_inventory =
        canonical_artifact_identity(artifacts, CanonicalArtifactKind::EnvironmentInventory);
    if environment_inventory.kind != CanonicalArtifactKind::EnvironmentInventory {
        return Err(AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::ExistingCanonicalTruth,
            summary: "unexpected canonical artifact identity for environment inventory authoring"
                .to_string(),
            broken_subject: "canonical environment inventory truth".to_string(),
            next_safe_action:
                "inspect canonical artifact metadata and retry `handbook author environment-inventory --from-inputs <path|->`"
                    .to_string(),
        });
    }

    match baseline_authoring_eligibility(artifacts, CanonicalArtifactKind::EnvironmentInventory) {
        BaselineAuthoringEligibility::Authorable => {}
        BaselineAuthoringEligibility::ExistingValidCanonicalTruth => {
            return Err(AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::ExistingCanonicalTruth,
                summary:
                    "canonical environment inventory truth already exists as valid non-starter truth; `handbook author environment-inventory --from-inputs <path|->` refuses to overwrite authored canonical truth"
                        .to_string(),
                broken_subject: environment_inventory_layout
                    .canonical_target_relative()
                    .to_string(),
                next_safe_action: format!(
                    "inspect `{}` instead of rerunning `handbook author environment-inventory --from-inputs <path|->`",
                    environment_inventory_layout.canonical_target_relative()
                ),
            });
        }
        BaselineAuthoringEligibility::RequiresSetupRefresh => {
            return Err(AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::MutationRefused,
                summary:
                    "canonical environment inventory truth is unreadable or path-invalid; repair it with `handbook setup refresh` before rerunning `handbook author environment-inventory --from-inputs <path|->`"
                        .to_string(),
                broken_subject: environment_inventory_layout
                    .canonical_target_relative()
                    .to_string(),
                next_safe_action: "run `handbook setup refresh`".to_string(),
            });
        }
    }

    validate_canonical_write_target(repo_root, environment_inventory_layout.canonical_target().as_str())
        .map_err(|err| AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::MutationRefused,
            summary: format_repo_write_path_error(
                environment_inventory_layout.canonical_target_relative(),
                err,
            ),
            broken_subject: "canonical environment inventory write target".to_string(),
            next_safe_action:
                "repair the blocked canonical environment inventory path and retry `handbook author environment-inventory --from-inputs <path|->`"
                    .to_string(),
        })?;

    Ok(())
}

struct SelectedCharterPreflightCarrier {
    projection: handbook_engine::CanonicalCharterProjection,
    promotion_ref: String,
    lifecycle_transition_ref: String,
}

impl SelectedCharterPreflightCarrier {
    fn canonical_path(&self) -> &str {
        self.projection.canonical_path()
    }

    fn source_fingerprint(&self) -> &handbook_engine::DefinitionFingerprint {
        self.projection.source_fingerprint()
    }
}

fn required_selected_charter(
    repo_root: &Path,
) -> Result<SelectedCharterPreflightCarrier, AuthorEnvironmentInventoryRefusal> {
    let decisions =
        handbook_engine::resolve_shipped_profile_decisions(repo_root).map_err(|_| {
            invalid_upstream_canonical_truth_refusal(
                ".handbook/project/charter.yaml",
                "failed to resolve the selected Charter contract".to_owned(),
                "repair the installed Handbook definition package and retry".to_owned(),
            )
        })?;
    let projection =
        handbook_engine::load_selected_charter(repo_root, &decisions).map_err(|error| {
            let kind = if error.status() == handbook_engine::ArtifactInspectionStatus::Missing {
                AuthorEnvironmentInventoryRefusalKind::MissingRequiredCharter
            } else {
                AuthorEnvironmentInventoryRefusalKind::InvalidUpstreamCanonicalTruth
            };
            AuthorEnvironmentInventoryRefusal {
                kind,
                summary: format!(
                    "selected canonical Charter is unavailable: {:?}",
                    error.reason()
                ),
                broken_subject: error.canonical_path().to_owned(),
                next_safe_action:
                    "author, approve, and promote `.handbook/project/charter.yaml`, then retry"
                        .to_owned(),
            }
        })?;
    let committed = handbook_engine::CharterAuthorityTransactionServiceV1::new(repo_root)
        .read_committed_charter()
        .map_err(|error| {
            invalid_upstream_canonical_truth_refusal(
                projection.canonical_path(),
                format!(
                    "selected Charter committed-authority read failed: {:?}",
                    error.kind()
                ),
                "repair the Charter promotion journal and retry".to_owned(),
            )
        })?
        .ok_or_else(|| {
            invalid_upstream_canonical_truth_refusal(
                projection.canonical_path(),
                "selected Charter has no committed promotion authority".to_owned(),
                "approve and promote the selected Charter, then retry".to_owned(),
            )
        })?;
    if committed.canonical_fingerprint != projection.source_fingerprint().as_str() {
        return Err(invalid_upstream_canonical_truth_refusal(
            projection.canonical_path(),
            format!(
                "selected Charter source fingerprint {} does not match committed authority {}",
                projection.source_fingerprint(),
                committed.canonical_fingerprint
            ),
            "repair or repromote selected Charter authority, then retry".to_owned(),
        ));
    }

    Ok(SelectedCharterPreflightCarrier {
        projection,
        promotion_ref: committed.promotion_ref,
        lifecycle_transition_ref: committed.lifecycle_transition_ref,
    })
}

fn required_project_context_path(
    repo_root: &Path,
) -> Result<String, AuthorEnvironmentInventoryRefusal> {
    let decisions =
        handbook_engine::resolve_shipped_profile_decisions(repo_root).map_err(|_| {
            invalid_upstream_canonical_truth_refusal(
                ".handbook/project/context.yaml",
                "failed to resolve the selected Project Context contract".to_owned(),
                "repair the installed Handbook definition package and retry".to_owned(),
            )
        })?;
    handbook_engine::load_selected_project_context(repo_root, &decisions)
        .map(|observation| observation.canonical_path().to_owned())
        .map_err(|error| {
            invalid_upstream_canonical_truth_refusal(
                error.canonical_path(),
                format!(
                    "selected canonical Project Context is unavailable: {:?}",
                    error.reason()
                ),
                "repair or author `.handbook/project/context.yaml`, then retry".to_owned(),
            )
        })
}

fn invalid_upstream_canonical_truth_refusal(
    broken_subject: &str,
    summary: String,
    next_safe_action: String,
) -> AuthorEnvironmentInventoryRefusal {
    AuthorEnvironmentInventoryRefusal {
        kind: AuthorEnvironmentInventoryRefusalKind::InvalidUpstreamCanonicalTruth,
        summary,
        broken_subject: broken_subject.to_string(),
        next_safe_action,
    }
}

fn resolve_environment_inventory_now_utc() -> Result<String, String> {
    if let Ok(value) = std::env::var(AUTHOR_ENVIRONMENT_INVENTORY_NOW_UTC_ENV_VAR) {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_string());
        }
    }

    OffsetDateTime::now_utc()
        .format(NOW_UTC_FORMAT)
        .map_err(|error| {
            format!("failed to derive environment-inventory render timestamp: {error}")
        })
}

fn map_authoring_lock_error(
    repo_root: &Path,
    err: AuthoringLockError,
) -> AuthorEnvironmentInventoryRefusal {
    let environment_inventory_layout = RepoLayoutRoot::new(repo_root)
        .authoring()
        .environment_inventory();
    match err {
        AuthoringLockError::WritePath(path_err) => AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::MutationRefused,
            summary: format_repo_write_path_error(
                environment_inventory_layout.lock_relative_path(),
                path_err,
            ),
            broken_subject: "environment inventory authoring lock".to_string(),
            next_safe_action:
                "repair the blocked environment inventory authoring lock path and retry `handbook author environment-inventory --from-inputs <path|->`"
                    .to_string(),
        },
        AuthoringLockError::Io { lock_path, source } => AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::MutationRefused,
            summary: format!(
                "failed to acquire exclusive environment inventory authoring lock at {}: {source}",
                lock_path.display()
            ),
            broken_subject: "environment inventory authoring lock".to_string(),
            next_safe_action:
                "wait for any in-progress `handbook author environment-inventory --from-inputs <path|->` run to finish or repair the lock path, then retry `handbook author environment-inventory --from-inputs <path|->`"
                    .to_string(),
        },
    }
}
