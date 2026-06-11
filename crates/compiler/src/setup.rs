use crate::canonical_artifacts::{
    canonical_artifact_descriptors, setup_starter_template_bytes, CanonicalArtifactDescriptor,
    CanonicalArtifacts, SystemRootStatus, CANONICAL_ARTIFACT_ORDER,
};
use crate::layout::CanonicalLayout;
use crate::repo_file_access::{resolve_repo_relative_write_path, write_repo_relative_bytes};
use crate::route_state::{
    apply_runtime_state_reset, plan_runtime_state_reset, RuntimeStateResetPlan,
};
use crate::setup_shell::{self, SetupRequestRefusalCopy};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupMode {
    Auto,
    Init,
    Refresh,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupRequest {
    pub mode: SetupMode,
    pub rewrite: bool,
    pub reset_state: bool,
}

impl Default for SetupRequest {
    fn default() -> Self {
        Self {
            mode: SetupMode::Auto,
            rewrite: false,
            reset_state: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupActionLabel {
    Created,
    Preserved,
    Rewritten,
    Reset,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupAction {
    pub label: SetupActionLabel,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupPlan {
    pub requested_mode: SetupMode,
    pub resolved_mode: SetupMode,
    pub actions: Vec<SetupAction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupOutcome {
    pub plan: SetupPlan,
    pub disposition: SetupDisposition,
    pub next_safe_action: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupDisposition {
    Ready,
    Scaffolded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupRefusalKind {
    AlreadyInitialized,
    MissingCanonicalRoot,
    InvalidCanonicalRoot,
    InvalidRequest,
    MutationRefused,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupRefusal {
    pub kind: SetupRefusalKind,
    pub summary: String,
    pub broken_subject: String,
    pub next_safe_action: String,
}

impl std::fmt::Display for SetupRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary)
    }
}

impl std::error::Error for SetupRefusal {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlannedMutation {
    RepairInvalidSystemRoot,
    Write {
        path: &'static str,
        bytes: &'static [u8],
    },
}

#[derive(Debug, Clone)]
struct SetupExecutionPlan {
    plan: SetupPlan,
    mutations: Vec<PlannedMutation>,
    reset_plan: Option<RuntimeStateResetPlan>,
}

pub fn plan_setup(
    repo_root: impl AsRef<Path>,
    request: &SetupRequest,
) -> Result<SetupPlan, SetupRefusal> {
    build_setup_execution_plan(repo_root.as_ref(), request).map(|planned| planned.plan)
}

pub fn run_setup(
    repo_root: impl AsRef<Path>,
    request: &SetupRequest,
) -> Result<SetupOutcome, SetupRefusal> {
    let repo_root = repo_root.as_ref();
    let planned = build_setup_execution_plan(repo_root, request)?;

    for mutation in planned.mutations {
        apply_mutation(repo_root, planned.plan.resolved_mode, mutation)?;
    }

    if let Some(reset_plan) = &planned.reset_plan {
        apply_runtime_state_reset(reset_plan).map_err(|reason| {
            setup_shell::mutation_refusal(
                request.mode,
                reason,
                "runtime-state target under `.handbook/state/**`",
            )
        })?;
    }

    let post_setup_artifacts = CanonicalArtifacts::load(repo_root).map_err(|err| {
        setup_shell::mutation_refusal(request.mode, err.to_string(), "canonical `.handbook` root")
    })?;
    let disposition = setup_disposition(&post_setup_artifacts);

    Ok(SetupOutcome {
        plan: planned.plan,
        disposition,
        next_safe_action: setup_shell::next_safe_action(disposition),
    })
}

fn build_setup_execution_plan(
    repo_root: &Path,
    request: &SetupRequest,
) -> Result<SetupExecutionPlan, SetupRefusal> {
    let artifacts = CanonicalArtifacts::load(repo_root).map_err(|err| {
        setup_shell::mutation_refusal(request.mode, err.to_string(), "canonical `.handbook` root")
    })?;
    let canonical_layout = CanonicalLayout::new(repo_root);
    let resolved_mode = resolve_mode(request.mode, artifacts.system_root_status);
    validate_request(&artifacts, request, resolved_mode)?;

    let repair_invalid_root = resolved_mode == SetupMode::Init
        && matches!(
            artifacts.system_root_status,
            SystemRootStatus::NotDir | SystemRootStatus::SymlinkNotAllowed
        );
    let ingest_issue_paths = artifacts
        .ingest_issues
        .iter()
        .map(|issue| issue.canonical_repo_relative_path)
        .collect::<BTreeSet<_>>();
    let planned_starter_actions = canonical_artifact_descriptors()
        .iter()
        .filter(|descriptor| descriptor.setup_scaffolded)
        .map(|descriptor| {
            plan_starter_action(
                canonical_layout,
                repo_root,
                &artifacts,
                &ingest_issue_paths,
                descriptor,
                resolved_mode,
                request.rewrite,
                repair_invalid_root,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    let reset_plan = if request.reset_state {
        Some(plan_runtime_state_reset(repo_root).map_err(|reason| {
            setup_shell::mutation_refusal(
                request.mode,
                reason,
                "runtime-state target under `.handbook/state/**`",
            )
        })?)
    } else {
        None
    };

    let mut actions = planned_starter_actions
        .iter()
        .map(|planned| planned.action.clone())
        .collect::<Vec<_>>();
    if let Some(reset_plan) = &reset_plan {
        actions.extend(reset_plan.paths().iter().cloned().map(|path| SetupAction {
            label: SetupActionLabel::Reset,
            path,
        }));
    }
    actions.sort_by(|a, b| {
        artifact_order_index_for_path(&a.path)
            .cmp(&artifact_order_index_for_path(&b.path))
            .then_with(|| action_rank(a).cmp(&action_rank(b)))
            .then_with(|| a.path.cmp(&b.path))
    });

    let mut mutations = Vec::new();
    if repair_invalid_root {
        mutations.push(PlannedMutation::RepairInvalidSystemRoot);
    }
    mutations.extend(
        planned_starter_actions
            .into_iter()
            .filter_map(|planned| planned.mutation),
    );

    Ok(SetupExecutionPlan {
        plan: SetupPlan {
            requested_mode: request.mode,
            resolved_mode,
            actions,
        },
        mutations,
        reset_plan,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlannedStarterAction {
    action: SetupAction,
    mutation: Option<PlannedMutation>,
}

fn resolve_mode(requested: SetupMode, system_root_status: SystemRootStatus) -> SetupMode {
    match requested {
        SetupMode::Auto => match system_root_status {
            SystemRootStatus::Ok => SetupMode::Refresh,
            SystemRootStatus::Missing
            | SystemRootStatus::NotDir
            | SystemRootStatus::SymlinkNotAllowed => SetupMode::Init,
        },
        SetupMode::Init | SetupMode::Refresh => requested,
    }
}

fn validate_request(
    artifacts: &CanonicalArtifacts,
    request: &SetupRequest,
    resolved_mode: SetupMode,
) -> Result<(), SetupRefusal> {
    match resolved_mode {
        SetupMode::Auto => {
            return Err(setup_shell::request_refusal(
                SetupRequestRefusalCopy::UnresolvedMode,
            ));
        }
        SetupMode::Init => {
            if request.rewrite || request.reset_state {
                return Err(setup_shell::request_refusal(
                    SetupRequestRefusalCopy::InitWithRefreshFlags,
                ));
            }
            if artifacts.system_root_status == SystemRootStatus::Ok {
                return Err(setup_shell::request_refusal(
                    SetupRequestRefusalCopy::AlreadyInitialized,
                ));
            }
        }
        SetupMode::Refresh => match artifacts.system_root_status {
            SystemRootStatus::Ok => {}
            SystemRootStatus::Missing => {
                return Err(setup_shell::request_refusal(
                    SetupRequestRefusalCopy::MissingCanonicalRoot,
                ));
            }
            SystemRootStatus::NotDir => {
                return Err(setup_shell::request_refusal(
                    SetupRequestRefusalCopy::InvalidCanonicalRoot,
                ));
            }
            SystemRootStatus::SymlinkNotAllowed => {
                return Err(setup_shell::request_refusal(
                    SetupRequestRefusalCopy::SymlinkCanonicalRoot,
                ));
            }
        },
    }

    Ok(())
}

fn plan_starter_action(
    canonical_layout: CanonicalLayout<'_>,
    repo_root: &Path,
    artifacts: &CanonicalArtifacts,
    ingest_issue_paths: &BTreeSet<&'static str>,
    descriptor: &CanonicalArtifactDescriptor,
    resolved_mode: SetupMode,
    rewrite: bool,
    repair_invalid_root: bool,
) -> Result<PlannedStarterAction, SetupRefusal> {
    let relative_path = canonical_layout.artifact_relative_path(descriptor.kind);
    debug_assert_eq!(relative_path, descriptor.relative_path);
    let artifact = match descriptor.kind {
        crate::CanonicalArtifactKind::Charter => &artifacts.charter,
        crate::CanonicalArtifactKind::ProjectContext => &artifacts.project_context,
        crate::CanonicalArtifactKind::EnvironmentInventory => &artifacts.environment_inventory,
        crate::CanonicalArtifactKind::FeatureSpec => &artifacts.feature_spec,
    };

    if resolved_mode == SetupMode::Refresh
        && !rewrite
        && !matches!(artifact.identity.presence, crate::ArtifactPresence::Missing)
    {
        return Ok(PlannedStarterAction {
            action: SetupAction {
                label: SetupActionLabel::Preserved,
                path: relative_path.to_string(),
            },
            mutation: None,
        });
    }

    if (ingest_issue_paths.contains(relative_path)
        || matches!(artifact.identity.presence, crate::ArtifactPresence::Missing)
        || rewrite
        || resolved_mode == SetupMode::Init)
        && !repair_invalid_root
    {
        validate_write_target(repo_root, relative_path, resolved_mode)?;
    }

    let label = if matches!(artifact.identity.presence, crate::ArtifactPresence::Missing) {
        SetupActionLabel::Created
    } else if rewrite {
        SetupActionLabel::Rewritten
    } else {
        SetupActionLabel::Created
    };

    Ok(PlannedStarterAction {
        action: SetupAction {
            label,
            path: relative_path.to_string(),
        },
        mutation: Some(PlannedMutation::Write {
            path: relative_path,
            bytes: setup_starter_template_bytes(descriptor.kind),
        }),
    })
}

fn validate_write_target(
    repo_root: &Path,
    relative_path: &'static str,
    mode: SetupMode,
) -> Result<(), SetupRefusal> {
    resolve_repo_relative_write_path(repo_root, relative_path)
        .map(|_| ())
        .map_err(|err| {
            setup_shell::mutation_refusal(
                mode,
                setup_shell::format_repo_write_path_error(relative_path, err),
                "setup-owned starter-file write target",
            )
        })
}

fn apply_mutation(
    repo_root: &Path,
    mode: SetupMode,
    mutation: PlannedMutation,
) -> Result<(), SetupRefusal> {
    match mutation {
        PlannedMutation::RepairInvalidSystemRoot => repair_invalid_system_root(repo_root, mode),
        PlannedMutation::Write { path, bytes } => write_repo_relative_bytes(repo_root, path, bytes)
            .map_err(|err| {
                setup_shell::mutation_refusal(
                    mode,
                    setup_shell::format_repo_mutation_error(path, err),
                    "setup-owned starter-file write target",
                )
            }),
    }
}

fn repair_invalid_system_root(repo_root: &Path, mode: SetupMode) -> Result<(), SetupRefusal> {
    let canonical_layout = CanonicalLayout::new(repo_root);
    let system_root = repo_root.join(canonical_layout.system_root_relative());
    let metadata = match fs::symlink_metadata(&system_root) {
        Ok(metadata) => metadata,
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(source) => {
            return Err(setup_shell::mutation_refusal(
                mode,
                format!(
                    "failed to inspect canonical `.handbook` root at {}: {source}",
                    system_root.display()
                ),
                "canonical `.handbook` root",
            ));
        }
    };

    if metadata.is_dir() && !metadata.file_type().is_symlink() {
        return Ok(());
    }

    fs::remove_file(&system_root).map_err(|source| {
        setup_shell::mutation_refusal(
            mode,
            format!(
                "failed to remove invalid canonical `.handbook` root at {}: {source}",
                system_root.display()
            ),
            "canonical `.handbook` root",
        )
    })
}

fn setup_disposition(artifacts: &CanonicalArtifacts) -> SetupDisposition {
    if has_scaffolded_starter_template(artifacts) {
        SetupDisposition::Scaffolded
    } else {
        SetupDisposition::Ready
    }
}

fn has_scaffolded_starter_template(artifacts: &CanonicalArtifacts) -> bool {
    canonical_artifact_descriptors()
        .iter()
        .filter(|descriptor| descriptor.setup_scaffolded)
        .any(|descriptor| {
            artifact_for_kind(artifacts, descriptor.kind)
                .identity
                .matches_setup_starter_template
        })
}

fn artifact_for_kind(
    artifacts: &CanonicalArtifacts,
    kind: crate::CanonicalArtifactKind,
) -> &crate::CanonicalArtifact {
    match kind {
        crate::CanonicalArtifactKind::Charter => &artifacts.charter,
        crate::CanonicalArtifactKind::ProjectContext => &artifacts.project_context,
        crate::CanonicalArtifactKind::EnvironmentInventory => &artifacts.environment_inventory,
        crate::CanonicalArtifactKind::FeatureSpec => &artifacts.feature_spec,
    }
}

fn artifact_order_index_for_path(path: &str) -> usize {
    CANONICAL_ARTIFACT_ORDER
        .iter()
        .position(|kind| kind.relative_path() == path)
        .unwrap_or(usize::MAX)
}

fn action_rank(action: &SetupAction) -> usize {
    match action.label {
        SetupActionLabel::Created => 0,
        SetupActionLabel::Preserved => 1,
        SetupActionLabel::Rewritten => 2,
        SetupActionLabel::Reset => 3,
    }
}
