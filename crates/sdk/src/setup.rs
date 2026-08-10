use crate::profile_readiness::{
    project_profile_readiness, ArtifactApplicability, ArtifactInspectionStatus, ProfileArtifactRow,
    ProfileCapabilityRow, ProfileConditionRow, RepositoryReadinessStatus,
};
use crate::route_state::{
    apply_runtime_state_reset, plan_runtime_state_reset, RuntimeStateResetPlan,
};
use handbook_engine::{
    inspect_profile_repository, resolve_shipped_profile_decisions,
    RepositoryIdentitySetupErrorKindV1, RepositoryInvocationIdentityServiceV1,
    ResolvedProfileDecisions, ShippedProfileDecisionError,
};
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SetupMode {
    Auto,
    Init,
    Refresh,
}

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SetupArtifactActionKind {
    Preserve,
    AuthorRequired,
    OptionalAbsent,
    ConditionIndeterminate,
    Invalid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SetupRootAction {
    Preserve,
    Create,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupArtifactAction {
    pub artifact: ProfileArtifactRow,
    pub action: SetupArtifactActionKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupPlan {
    pub requested_mode: SetupMode,
    pub resolved_mode: SetupMode,
    pub root_action: SetupRootAction,
    pub profile_ref: String,
    pub profile_fingerprint: String,
    pub stable_role_registry_ref: String,
    pub stable_role_registry_fingerprint: String,
    pub conditions: Vec<ProfileConditionRow>,
    pub capabilities: Vec<ProfileCapabilityRow>,
    pub artifacts: Vec<SetupArtifactAction>,
    pub reset_paths: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupOutcome {
    pub plan: SetupPlan,
    pub status: RepositoryReadinessStatus,
    pub reset_applied: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SetupErrorKind {
    ProfileResolution,
    ProfileDecision,
    AlreadyInitialized,
    MissingCanonicalRoot,
    InvalidCanonicalRoot,
    InvalidRequest,
    MaterializerUnavailable,
    RuntimeStatePlan,
    RuntimeStateApply,
    RepositoryIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SetupErrorReasonCode {
    ShippedProfileUnavailable,
    SelectedProfileDecisionInvalid,
    UnresolvedMode,
    InitRejectsRefreshFlags,
    RootAlreadyInitialized,
    RefreshRootMissing,
    RootNotDirectory,
    RootSymlinkRefused,
    CanonicalRootInspectFailed,
    CanonicalRootCreateFailed,
    RewriteHasNoMaterializer,
    RuntimeStateTargetUnsafe,
    RuntimeStateMutationFailed,
    RepositoryAuthorityRecoveryBlocked,
    RepositoryIdentityUnsafe,
    RepositoryIdentityMismatch,
    RepositoryIdentityEntropyUnavailable,
    RepositoryIdentityPersistenceFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SetupErrorCode {
    ShippedProfileUnavailable,
    SelectedProfileDecisionInvalid,
    UnresolvedMode,
    InitRejectsRefreshFlags,
    RootAlreadyInitialized,
    RefreshRootMissing,
    RootNotDirectory,
    RootSymlinkRefused,
    CanonicalRootInspectFailed,
    CanonicalRootCreateFailed,
    RewriteHasNoMaterializer,
    RuntimeStateTargetUnsafe,
    RuntimeStateMutationFailed,
    RepositoryAuthorityRecoveryBlocked,
    RepositoryIdentityUnsafe,
    RepositoryIdentityMismatch,
    RepositoryIdentityEntropyUnavailable,
    RepositoryIdentityPersistenceFailed,
}

impl SetupErrorCode {
    pub const ALL: [Self; 18] = [
        Self::ShippedProfileUnavailable,
        Self::SelectedProfileDecisionInvalid,
        Self::UnresolvedMode,
        Self::InitRejectsRefreshFlags,
        Self::RootAlreadyInitialized,
        Self::RefreshRootMissing,
        Self::RootNotDirectory,
        Self::RootSymlinkRefused,
        Self::CanonicalRootInspectFailed,
        Self::CanonicalRootCreateFailed,
        Self::RewriteHasNoMaterializer,
        Self::RuntimeStateTargetUnsafe,
        Self::RuntimeStateMutationFailed,
        Self::RepositoryAuthorityRecoveryBlocked,
        Self::RepositoryIdentityUnsafe,
        Self::RepositoryIdentityMismatch,
        Self::RepositoryIdentityEntropyUnavailable,
        Self::RepositoryIdentityPersistenceFailed,
    ];
}

/// A closed setup failure whose code is SDK-owned.
///
/// ```compile_fail,E0451
/// use handbook_sdk::{SetupError, SetupErrorCode};
/// let _ = SetupError { code: SetupErrorCode::UnresolvedMode };
/// ```
///
/// ```compile_fail,E0624
/// use handbook_sdk::{SetupError, SetupErrorCode};
/// let _ = SetupError::from_code(SetupErrorCode::UnresolvedMode);
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupError {
    code: SetupErrorCode,
}

impl SetupError {
    pub fn code(&self) -> SetupErrorCode {
        self.code
    }

    pub fn kind(&self) -> SetupErrorKind {
        match self.code {
            SetupErrorCode::ShippedProfileUnavailable => SetupErrorKind::ProfileResolution,
            SetupErrorCode::SelectedProfileDecisionInvalid => SetupErrorKind::ProfileDecision,
            SetupErrorCode::RootAlreadyInitialized => SetupErrorKind::AlreadyInitialized,
            SetupErrorCode::RefreshRootMissing => SetupErrorKind::MissingCanonicalRoot,
            SetupErrorCode::RootNotDirectory
            | SetupErrorCode::RootSymlinkRefused
            | SetupErrorCode::CanonicalRootInspectFailed
            | SetupErrorCode::CanonicalRootCreateFailed => SetupErrorKind::InvalidCanonicalRoot,
            SetupErrorCode::UnresolvedMode | SetupErrorCode::InitRejectsRefreshFlags => {
                SetupErrorKind::InvalidRequest
            }
            SetupErrorCode::RewriteHasNoMaterializer => SetupErrorKind::MaterializerUnavailable,
            SetupErrorCode::RuntimeStateTargetUnsafe => SetupErrorKind::RuntimeStatePlan,
            SetupErrorCode::RuntimeStateMutationFailed => SetupErrorKind::RuntimeStateApply,
            SetupErrorCode::RepositoryAuthorityRecoveryBlocked
            | SetupErrorCode::RepositoryIdentityUnsafe
            | SetupErrorCode::RepositoryIdentityMismatch
            | SetupErrorCode::RepositoryIdentityEntropyUnavailable
            | SetupErrorCode::RepositoryIdentityPersistenceFailed => {
                SetupErrorKind::RepositoryIdentity
            }
        }
    }

    pub fn reason_code(&self) -> SetupErrorReasonCode {
        match self.code {
            SetupErrorCode::ShippedProfileUnavailable => {
                SetupErrorReasonCode::ShippedProfileUnavailable
            }
            SetupErrorCode::SelectedProfileDecisionInvalid => {
                SetupErrorReasonCode::SelectedProfileDecisionInvalid
            }
            SetupErrorCode::UnresolvedMode => SetupErrorReasonCode::UnresolvedMode,
            SetupErrorCode::InitRejectsRefreshFlags => {
                SetupErrorReasonCode::InitRejectsRefreshFlags
            }
            SetupErrorCode::RootAlreadyInitialized => SetupErrorReasonCode::RootAlreadyInitialized,
            SetupErrorCode::RefreshRootMissing => SetupErrorReasonCode::RefreshRootMissing,
            SetupErrorCode::RootNotDirectory => SetupErrorReasonCode::RootNotDirectory,
            SetupErrorCode::RootSymlinkRefused => SetupErrorReasonCode::RootSymlinkRefused,
            SetupErrorCode::CanonicalRootInspectFailed => {
                SetupErrorReasonCode::CanonicalRootInspectFailed
            }
            SetupErrorCode::CanonicalRootCreateFailed => {
                SetupErrorReasonCode::CanonicalRootCreateFailed
            }
            SetupErrorCode::RewriteHasNoMaterializer => {
                SetupErrorReasonCode::RewriteHasNoMaterializer
            }
            SetupErrorCode::RuntimeStateTargetUnsafe => {
                SetupErrorReasonCode::RuntimeStateTargetUnsafe
            }
            SetupErrorCode::RuntimeStateMutationFailed => {
                SetupErrorReasonCode::RuntimeStateMutationFailed
            }
            SetupErrorCode::RepositoryAuthorityRecoveryBlocked => {
                SetupErrorReasonCode::RepositoryAuthorityRecoveryBlocked
            }
            SetupErrorCode::RepositoryIdentityUnsafe => {
                SetupErrorReasonCode::RepositoryIdentityUnsafe
            }
            SetupErrorCode::RepositoryIdentityMismatch => {
                SetupErrorReasonCode::RepositoryIdentityMismatch
            }
            SetupErrorCode::RepositoryIdentityEntropyUnavailable => {
                SetupErrorReasonCode::RepositoryIdentityEntropyUnavailable
            }
            SetupErrorCode::RepositoryIdentityPersistenceFailed => {
                SetupErrorReasonCode::RepositoryIdentityPersistenceFailed
            }
        }
    }

    pub fn repo_relative_path(&self) -> Option<&'static str> {
        match self.code {
            SetupErrorCode::RootAlreadyInitialized
            | SetupErrorCode::RefreshRootMissing
            | SetupErrorCode::RootNotDirectory
            | SetupErrorCode::RootSymlinkRefused
            | SetupErrorCode::CanonicalRootInspectFailed
            | SetupErrorCode::CanonicalRootCreateFailed => Some(".handbook"),
            SetupErrorCode::RuntimeStateTargetUnsafe
            | SetupErrorCode::RuntimeStateMutationFailed => Some(".handbook/state"),
            SetupErrorCode::RepositoryAuthorityRecoveryBlocked => Some(".handbook/state"),
            SetupErrorCode::RepositoryIdentityUnsafe
            | SetupErrorCode::RepositoryIdentityMismatch
            | SetupErrorCode::RepositoryIdentityEntropyUnavailable
            | SetupErrorCode::RepositoryIdentityPersistenceFailed => {
                Some(handbook_engine::REPOSITORY_IDENTITY_REPO_PATH)
            }
            SetupErrorCode::ShippedProfileUnavailable
            | SetupErrorCode::SelectedProfileDecisionInvalid
            | SetupErrorCode::UnresolvedMode
            | SetupErrorCode::InitRejectsRefreshFlags
            | SetupErrorCode::RewriteHasNoMaterializer => None,
        }
    }

    pub(crate) fn from_code(code: SetupErrorCode) -> Self {
        Self { code }
    }
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "setup refused: {:?}", self.code)
    }
}

impl std::error::Error for SetupError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RootState {
    Missing,
    Directory,
    Symlink,
    NonDirectory,
}

#[derive(Debug)]
struct SetupExecutionPlan {
    plan: SetupPlan,
    status: RepositoryReadinessStatus,
    reset_plan: Option<RuntimeStateResetPlan>,
}

#[cfg(test)]
pub(crate) fn plan_setup(
    repo_root: impl AsRef<Path>,
    request: &SetupRequest,
) -> Result<SetupPlan, SetupError> {
    let repo_root = repo_root.as_ref();
    let root = inspect_root(repo_root)?;
    let (resolved_mode, root_action) = validate_root_and_request(root, request)?;
    let decisions = resolve_shipped_profile_decisions(repo_root).map_err(map_profile_error)?;
    build_setup_execution_plan(repo_root, request, resolved_mode, root_action, &decisions)
        .map(|execution| execution.plan)
}

#[cfg(test)]
pub(crate) fn plan_setup_with_decisions(
    repo_root: impl AsRef<Path>,
    request: &SetupRequest,
    decisions: &ResolvedProfileDecisions,
) -> Result<SetupPlan, SetupError> {
    let repo_root = repo_root.as_ref();
    let root = inspect_root(repo_root)?;
    let (resolved_mode, root_action) = validate_root_and_request(root, request)?;
    build_setup_execution_plan(repo_root, request, resolved_mode, root_action, decisions)
        .map(|execution| execution.plan)
}

pub(crate) fn run_setup(
    repo_root: impl AsRef<Path>,
    request: &SetupRequest,
) -> Result<SetupOutcome, SetupError> {
    let repo_root = repo_root.as_ref();
    let root = inspect_root(repo_root)?;
    let (resolved_mode, root_action) = validate_root_and_request(root, request)?;
    let decisions = resolve_shipped_profile_decisions(repo_root).map_err(map_profile_error)?;
    let execution =
        build_setup_execution_plan(repo_root, request, resolved_mode, root_action, &decisions)?;
    apply_setup_execution(repo_root, execution)
}

#[cfg(test)]
pub(crate) fn run_setup_with_decisions(
    repo_root: impl AsRef<Path>,
    request: &SetupRequest,
    decisions: &ResolvedProfileDecisions,
) -> Result<SetupOutcome, SetupError> {
    let repo_root = repo_root.as_ref();
    let root = inspect_root(repo_root)?;
    let (resolved_mode, root_action) = validate_root_and_request(root, request)?;
    let execution =
        build_setup_execution_plan(repo_root, request, resolved_mode, root_action, decisions)?;
    apply_setup_execution(repo_root, execution)
}

fn inspect_root(repo_root: &Path) -> Result<RootState, SetupError> {
    match fs::symlink_metadata(repo_root.join(".handbook")) {
        Ok(metadata) if metadata.file_type().is_symlink() => Ok(RootState::Symlink),
        Ok(metadata) if metadata.is_dir() => Ok(RootState::Directory),
        Ok(_) => Ok(RootState::NonDirectory),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(RootState::Missing),
        Err(_) => Err(SetupError::from_code(
            SetupErrorCode::CanonicalRootInspectFailed,
        )),
    }
}

fn validate_root_and_request(
    root: RootState,
    request: &SetupRequest,
) -> Result<(SetupMode, SetupRootAction), SetupError> {
    let resolved_mode = match request.mode {
        SetupMode::Auto => match root {
            RootState::Missing | RootState::Symlink | RootState::NonDirectory => SetupMode::Init,
            RootState::Directory => SetupMode::Refresh,
        },
        SetupMode::Init => SetupMode::Init,
        SetupMode::Refresh => SetupMode::Refresh,
    };
    if resolved_mode == SetupMode::Auto {
        return Err(SetupError::from_code(SetupErrorCode::UnresolvedMode));
    }
    if resolved_mode == SetupMode::Init && (request.rewrite || request.reset_state) {
        return Err(SetupError::from_code(
            SetupErrorCode::InitRejectsRefreshFlags,
        ));
    }

    let root_action = match (resolved_mode, root) {
        (SetupMode::Refresh, RootState::Directory) => SetupRootAction::Preserve,
        (SetupMode::Init, RootState::Missing) => SetupRootAction::Create,
        (SetupMode::Init, RootState::Directory) => {
            return Err(SetupError::from_code(
                SetupErrorCode::RootAlreadyInitialized,
            ))
        }
        (SetupMode::Refresh, RootState::Missing) => {
            return Err(SetupError::from_code(SetupErrorCode::RefreshRootMissing))
        }
        (_, RootState::NonDirectory) => {
            return Err(SetupError::from_code(SetupErrorCode::RootNotDirectory))
        }
        (_, RootState::Symlink) => {
            return Err(SetupError::from_code(SetupErrorCode::RootSymlinkRefused))
        }
        (SetupMode::Auto, _) => return Err(SetupError::from_code(SetupErrorCode::UnresolvedMode)),
    };
    if resolved_mode == SetupMode::Refresh && request.rewrite {
        return Err(SetupError::from_code(
            SetupErrorCode::RewriteHasNoMaterializer,
        ));
    }
    Ok((resolved_mode, root_action))
}

fn build_setup_execution_plan(
    repo_root: &Path,
    request: &SetupRequest,
    resolved_mode: SetupMode,
    root_action: SetupRootAction,
    decisions: &ResolvedProfileDecisions,
) -> Result<SetupExecutionPlan, SetupError> {
    let inspection = inspect_profile_repository(repo_root, decisions);
    let projection = project_profile_readiness(decisions, &inspection);
    let artifacts = projection
        .artifacts
        .iter()
        .cloned()
        .map(|artifact| SetupArtifactAction {
            action: setup_action(&artifact),
            artifact,
        })
        .collect();
    let reset_plan = if request.reset_state {
        Some(
            plan_runtime_state_reset(repo_root)
                .map_err(|_| SetupError::from_code(SetupErrorCode::RuntimeStateTargetUnsafe))?,
        )
    } else {
        None
    };
    let reset_paths = reset_plan
        .as_ref()
        .map(|plan| plan.paths().to_vec())
        .unwrap_or_default();
    Ok(SetupExecutionPlan {
        status: projection.status,
        plan: SetupPlan {
            requested_mode: request.mode,
            resolved_mode,
            root_action,
            profile_ref: projection.profile_ref,
            profile_fingerprint: projection.profile_fingerprint,
            stable_role_registry_ref: projection.stable_role_registry_ref,
            stable_role_registry_fingerprint: projection.stable_role_registry_fingerprint,
            conditions: projection.conditions,
            capabilities: projection.capabilities,
            artifacts,
            reset_paths,
        },
        reset_plan,
    })
}

fn setup_action(artifact: &ProfileArtifactRow) -> SetupArtifactActionKind {
    if matches!(
        artifact.inspection_status,
        ArtifactInspectionStatus::StructurallyInvalid
            | ArtifactInspectionStatus::UnsafePath
            | ArtifactInspectionStatus::Unreadable
    ) {
        return SetupArtifactActionKind::Invalid;
    }
    match artifact.applicability {
        ArtifactApplicability::Required => match artifact.inspection_status {
            ArtifactInspectionStatus::StructurallyValid => SetupArtifactActionKind::Preserve,
            ArtifactInspectionStatus::Missing => SetupArtifactActionKind::AuthorRequired,
            _ => SetupArtifactActionKind::Invalid,
        },
        ArtifactApplicability::Optional => match artifact.inspection_status {
            ArtifactInspectionStatus::StructurallyValid => SetupArtifactActionKind::Preserve,
            ArtifactInspectionStatus::Missing => SetupArtifactActionKind::OptionalAbsent,
            _ => SetupArtifactActionKind::Invalid,
        },
        ArtifactApplicability::Indeterminate => SetupArtifactActionKind::ConditionIndeterminate,
    }
}

fn apply_setup_execution(
    repo_root: &Path,
    execution: SetupExecutionPlan,
) -> Result<SetupOutcome, SetupError> {
    let may_reset_runtime_state = matches!(
        execution.status,
        RepositoryReadinessStatus::Ready | RepositoryReadinessStatus::ActionRequired
    );
    let mut reset_applied = false;
    if execution.plan.root_action == SetupRootAction::Create {
        fs::create_dir(repo_root.join(".handbook"))
            .map_err(|_| SetupError::from_code(SetupErrorCode::CanonicalRootCreateFailed))?;
    }
    if may_reset_runtime_state {
        if let Some(reset_plan) = &execution.reset_plan {
            apply_runtime_state_reset(reset_plan)
                .map_err(|_| SetupError::from_code(SetupErrorCode::RuntimeStateMutationFailed))?;
            reset_applied = true;
        }
    }
    RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo_root)
        .map_err(map_repository_identity_error)?;
    Ok(SetupOutcome {
        plan: execution.plan,
        status: execution.status,
        reset_applied,
    })
}

fn map_repository_identity_error(
    error: handbook_engine::RepositoryIdentitySetupErrorV1,
) -> SetupError {
    SetupError::from_code(match error.kind() {
        RepositoryIdentitySetupErrorKindV1::AuthorityRecoveryBlocked => {
            SetupErrorCode::RepositoryAuthorityRecoveryBlocked
        }
        RepositoryIdentitySetupErrorKindV1::UnsafeIdentityState => {
            SetupErrorCode::RepositoryIdentityUnsafe
        }
        RepositoryIdentitySetupErrorKindV1::RegistryIdentityMismatch => {
            SetupErrorCode::RepositoryIdentityMismatch
        }
        RepositoryIdentitySetupErrorKindV1::EntropyUnavailable => {
            SetupErrorCode::RepositoryIdentityEntropyUnavailable
        }
        RepositoryIdentitySetupErrorKindV1::PersistenceFailed => {
            SetupErrorCode::RepositoryIdentityPersistenceFailed
        }
    })
}

fn map_profile_error(error: ShippedProfileDecisionError) -> SetupError {
    SetupError::from_code(match error {
        ShippedProfileDecisionError::Profile(_) => SetupErrorCode::ShippedProfileUnavailable,
        ShippedProfileDecisionError::Decision(_) => SetupErrorCode::SelectedProfileDecisionInvalid,
    })
}

#[cfg(test)]
#[path = "setup_tests.rs"]
mod behavior_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_error_projection_is_exhaustive() {
        let expected = [
            (
                SetupErrorKind::ProfileResolution,
                SetupErrorReasonCode::ShippedProfileUnavailable,
                None,
            ),
            (
                SetupErrorKind::ProfileDecision,
                SetupErrorReasonCode::SelectedProfileDecisionInvalid,
                None,
            ),
            (
                SetupErrorKind::InvalidRequest,
                SetupErrorReasonCode::UnresolvedMode,
                None,
            ),
            (
                SetupErrorKind::InvalidRequest,
                SetupErrorReasonCode::InitRejectsRefreshFlags,
                None,
            ),
            (
                SetupErrorKind::AlreadyInitialized,
                SetupErrorReasonCode::RootAlreadyInitialized,
                Some(".handbook"),
            ),
            (
                SetupErrorKind::MissingCanonicalRoot,
                SetupErrorReasonCode::RefreshRootMissing,
                Some(".handbook"),
            ),
            (
                SetupErrorKind::InvalidCanonicalRoot,
                SetupErrorReasonCode::RootNotDirectory,
                Some(".handbook"),
            ),
            (
                SetupErrorKind::InvalidCanonicalRoot,
                SetupErrorReasonCode::RootSymlinkRefused,
                Some(".handbook"),
            ),
            (
                SetupErrorKind::InvalidCanonicalRoot,
                SetupErrorReasonCode::CanonicalRootInspectFailed,
                Some(".handbook"),
            ),
            (
                SetupErrorKind::InvalidCanonicalRoot,
                SetupErrorReasonCode::CanonicalRootCreateFailed,
                Some(".handbook"),
            ),
            (
                SetupErrorKind::MaterializerUnavailable,
                SetupErrorReasonCode::RewriteHasNoMaterializer,
                None,
            ),
            (
                SetupErrorKind::RuntimeStatePlan,
                SetupErrorReasonCode::RuntimeStateTargetUnsafe,
                Some(".handbook/state"),
            ),
            (
                SetupErrorKind::RuntimeStateApply,
                SetupErrorReasonCode::RuntimeStateMutationFailed,
                Some(".handbook/state"),
            ),
            (
                SetupErrorKind::RepositoryIdentity,
                SetupErrorReasonCode::RepositoryAuthorityRecoveryBlocked,
                Some(".handbook/state"),
            ),
            (
                SetupErrorKind::RepositoryIdentity,
                SetupErrorReasonCode::RepositoryIdentityUnsafe,
                Some(handbook_engine::REPOSITORY_IDENTITY_REPO_PATH),
            ),
            (
                SetupErrorKind::RepositoryIdentity,
                SetupErrorReasonCode::RepositoryIdentityMismatch,
                Some(handbook_engine::REPOSITORY_IDENTITY_REPO_PATH),
            ),
            (
                SetupErrorKind::RepositoryIdentity,
                SetupErrorReasonCode::RepositoryIdentityEntropyUnavailable,
                Some(handbook_engine::REPOSITORY_IDENTITY_REPO_PATH),
            ),
            (
                SetupErrorKind::RepositoryIdentity,
                SetupErrorReasonCode::RepositoryIdentityPersistenceFailed,
                Some(handbook_engine::REPOSITORY_IDENTITY_REPO_PATH),
            ),
        ];
        for (code, (kind, reason, path)) in SetupErrorCode::ALL.into_iter().zip(expected) {
            let error = SetupError::from_code(code);
            assert_eq!(error.code(), code);
            assert_eq!(error.kind(), kind);
            assert_eq!(error.reason_code(), reason);
            assert_eq!(error.repo_relative_path(), path);
        }
    }
}
