use crate::declarative_roots::{
    profile_file as profile_repo_file, profile_root, runner_file as runner_repo_file, runner_root,
    PROFILES_ROOT_DISPLAY, RUNNERS_ROOT_DISPLAY,
};
use crate::layout::RepoLayoutRoot;
use crate::pipeline::{
    load_selected_pipeline_definition, supported_route_state_variables, PipelineDefinition,
};
use crate::pipeline_route::{
    resolve_pipeline_route, ResolvedPipelineRoute, RouteStageReason, RouteStageStatus,
    RouteVariables,
};
use crate::repo_file_access::{
    sha256_repo_relative_file, CompilerWorkspace, NormalizedRepoRelativePath,
    RepoRelativeDirectoryAccessError, RepoRelativeFileAccessError, RepoRelativeMetadataReadError,
    TrustedRepoDirectory,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::layout::{
    handbook_product_pipeline_storage_layout_contract, PipelineStorageLayoutContract,
};

pub const ROUTE_STATE_SCHEMA_VERSION: &str = "m2-pipeline-state-v3";
const LEGACY_ROUTE_STATE_SCHEMA_VERSION: &str = "m1-pipeline-state-v2";
pub const ROUTE_BASIS_SCHEMA_VERSION: &str = "m2-route-basis-v1";
pub const ROUTE_BASIS_REPO_ROOT_SENTINEL: &str = "${repo_root}";
pub const ROUTE_STATE_AUDIT_LIMIT: usize = 50;

static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

const FIELD_REFS_CHARTER_REF: &str = "refs.charter_ref";
const FIELD_REFS_PROJECT_CONTEXT_REF: &str = "refs.project_context_ref";
const FIELD_RUN_RUNNER: &str = "run.runner";
const FIELD_RUN_PROFILE: &str = "run.profile";
const PROFILE_PACK_REQUIRED_FILES: [&str; 3] = ["profile.yaml", "commands.yaml", "conventions.md"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteState {
    pub schema_version: String,
    pub pipeline_id: String,
    pub revision: u64,
    pub routing: BTreeMap<String, bool>,
    pub refs: RouteStateRefs,
    pub run: RouteStateRun,
    pub audit: Vec<RouteStateAuditEntry>,
    #[serde(default)]
    pub route_basis: Option<RouteBasis>,
}

impl RouteState {
    pub fn empty(pipeline_id: impl Into<String>) -> Self {
        Self {
            schema_version: ROUTE_STATE_SCHEMA_VERSION.to_string(),
            pipeline_id: pipeline_id.into(),
            revision: 0,
            routing: BTreeMap::new(),
            refs: RouteStateRefs::default(),
            run: RouteStateRun::default(),
            audit: Vec::new(),
            route_basis: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteBasis {
    pub schema_version: String,
    pub pipeline_id: String,
    pub pipeline_file: String,
    pub pipeline_file_sha256: String,
    pub state_revision: u64,
    pub routing: BTreeMap<String, bool>,
    pub refs: RouteStateRefs,
    pub run: RouteStateRun,
    pub route: Vec<RouteBasisResolvedStage>,
    pub runner: RouteBasisRunner,
    pub profile: RouteBasisProfilePack,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteBasisResolvedStage {
    pub stage_id: String,
    pub file: String,
    pub status: RouteBasisStageStatus,
    pub reason: Option<RouteBasisStageReason>,
    pub file_sha256: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteBasisStageStatus {
    Active,
    Skipped,
    Blocked,
    Next,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RouteBasisStageReason {
    SkippedActivationFalse {
        operator: RouteBasisActivationOperator,
        unsatisfied_variables: Vec<String>,
    },
    NextMissingRouteVariables {
        operator: RouteBasisActivationOperator,
        missing_variables: Vec<String>,
    },
    BlockedByUnresolvedStage {
        upstream_stage_id: String,
        upstream_status: RouteBasisStageStatus,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteBasisActivationOperator {
    Any,
    All,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteBasisRunner {
    pub id: String,
    pub file: String,
    pub file_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteBasisProfilePack {
    pub id: String,
    pub profile_yaml_sha256: String,
    pub commands_yaml_sha256: String,
    pub conventions_md_sha256: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteStateRefs {
    pub charter_ref: Option<String>,
    pub project_context_ref: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteStateRun {
    pub runner: Option<String>,
    pub profile: Option<String>,
    pub repo_root: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct RouteStateRunInventory {
    runners: BTreeSet<String>,
    profiles: ProfilePackInventory,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct ProfilePackInventory {
    complete_ids: BTreeSet<String>,
    incomplete_ids: BTreeMap<String, Vec<&'static str>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteStateAuditEntry {
    pub revision: u64,
    pub field_path: String,
    pub value: RouteStateValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RouteStateValue {
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteStateMutation {
    RoutingVariable { variable: String, value: bool },
    RefCharterRef { value: String },
    RefProjectContextRef { value: String },
    RunRunner { value: String },
    RunProfile { value: String },
}

impl RouteStateMutation {
    fn field_path(&self) -> String {
        match self {
            Self::RoutingVariable { variable, .. } => format!("routing.{variable}"),
            Self::RefCharterRef { .. } => FIELD_REFS_CHARTER_REF.to_string(),
            Self::RefProjectContextRef { .. } => FIELD_REFS_PROJECT_CONTEXT_REF.to_string(),
            Self::RunRunner { .. } => FIELD_RUN_RUNNER.to_string(),
            Self::RunProfile { .. } => FIELD_RUN_PROFILE.to_string(),
        }
    }

    fn value(&self) -> RouteStateValue {
        match self {
            Self::RoutingVariable { value, .. } => RouteStateValue::Bool(*value),
            Self::RefCharterRef { value }
            | Self::RefProjectContextRef { value }
            | Self::RunRunner { value }
            | Self::RunProfile { value } => RouteStateValue::String(value.clone()),
        }
    }

    fn apply(&self, state: &mut RouteState) {
        match self {
            Self::RoutingVariable { variable, value } => {
                state.routing.insert(variable.clone(), *value);
            }
            Self::RefCharterRef { value } => {
                state.refs.charter_ref = Some(value.clone());
            }
            Self::RefProjectContextRef { value } => {
                state.refs.project_context_ref = Some(value.clone());
            }
            Self::RunRunner { value } => {
                state.run.runner = Some(value.clone());
            }
            Self::RunProfile { value } => {
                state.run.profile = Some(value.clone());
            }
        }
    }
}

enum RouteStateFieldPath<'a> {
    Routing(&'a str),
    RefsCharterRef,
    RefsProjectContextRef,
    RunRunner,
    RunProfile,
}

#[derive(Debug)]
pub enum RouteStateReadError {
    InvalidPipelineId {
        pipeline_id: String,
        reason: &'static str,
    },
    ReadFailure {
        path: PathBuf,
        source: std::io::Error,
    },
    MalformedState {
        path: PathBuf,
        reason: String,
    },
}

#[derive(Debug)]
pub enum RouteStateStoreError {
    InvalidPipelineId {
        pipeline_id: String,
        reason: &'static str,
    },
    InvalidSupportedVariables {
        reason: String,
    },
    InvalidMutation {
        reason: String,
    },
    ReadFailure {
        path: PathBuf,
        source: std::io::Error,
    },
    LockFailure {
        path: PathBuf,
        source: std::io::Error,
    },
    WriteFailure {
        path: PathBuf,
        source: std::io::Error,
    },
    SerializationFailure {
        path: PathBuf,
        reason: String,
    },
}

#[derive(Debug)]
pub enum RouteBasisBuildError {
    MissingSelectedRunner {
        pipeline_id: String,
    },
    MissingSelectedProfile {
        pipeline_id: String,
    },
    IncompleteSelectedProfilePack {
        pipeline_id: String,
        profile_id: String,
        missing_files: Vec<&'static str>,
    },
    InvalidRouteSnapshot {
        pipeline_id: String,
        stage_id: String,
        detail: String,
    },
    ReadFailure {
        path: PathBuf,
        source: std::io::Error,
    },
    InvalidPath {
        path: String,
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteStateMutationOutcome {
    Applied(Box<RouteState>),
    Refused(RouteStateMutationRefusal),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteStateMutationRefusal {
    MalformedState {
        reason: String,
    },
    UnsupportedVariable {
        variable: String,
    },
    RevisionConflict {
        expected_revision: u64,
        actual_revision: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteBasisPersistOutcome {
    Applied(Box<RouteState>),
    Refused(RouteBasisPersistRefusal),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteBasisPersistRefusal {
    MalformedState {
        reason: String,
    },
    RevisionConflict {
        expected_revision: u64,
        actual_revision: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustedPipelineSession {
    pub pipeline_id: String,
    pub route_state: RouteState,
    pub route_basis: RouteBasis,
}

impl TrustedPipelineSession {
    pub fn require_active_stage(
        &self,
        stage_id: impl AsRef<str>,
    ) -> Result<&RouteBasisResolvedStage, TrustedPipelineSessionRefusal> {
        let stage_id = stage_id.as_ref();
        let Some(stage) = self
            .route_basis
            .route
            .iter()
            .find(|stage| stage.stage_id == stage_id)
        else {
            return Err(TrustedPipelineSessionRefusal::MissingStage {
                pipeline_id: self.pipeline_id.clone(),
                stage_id: stage_id.to_string(),
            });
        };

        if stage.status != RouteBasisStageStatus::Active {
            return Err(TrustedPipelineSessionRefusal::InactiveStage {
                pipeline_id: self.pipeline_id.clone(),
                stage_id: stage_id.to_string(),
                status: stage.status,
                reason: stage.reason.clone(),
            });
        }

        Ok(stage)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrustedPipelineSessionRefusal {
    InvalidPipelineId {
        pipeline_id: String,
        reason: &'static str,
    },
    ReadFailure {
        path: PathBuf,
        reason: String,
    },
    InvalidState {
        path: PathBuf,
        reason: String,
    },
    MissingRouteBasis {
        pipeline_id: String,
    },
    StaleRouteBasis {
        pipeline_id: String,
        reason: String,
    },
    MalformedRouteBasis {
        pipeline_id: String,
        reason: String,
    },
    MissingStage {
        pipeline_id: String,
        stage_id: String,
    },
    InactiveStage {
        pipeline_id: String,
        stage_id: String,
        status: RouteBasisStageStatus,
        reason: Option<RouteBasisStageReason>,
    },
}

#[derive(Debug)]
struct RouteStateInventoryLoadError {
    path: PathBuf,
    source: std::io::Error,
}

pub fn load_route_state(
    repo_root: impl AsRef<Path>,
    pipeline_id: impl AsRef<str>,
) -> Result<RouteState, RouteStateReadError> {
    let pipeline_id = pipeline_id.as_ref();
    let state_path = route_state_path(repo_root.as_ref(), pipeline_id).map_err(|reason| {
        RouteStateReadError::InvalidPipelineId {
            pipeline_id: pipeline_id.to_string(),
            reason,
        }
    })?;
    let run_inventory =
        load_run_inventory(repo_root.as_ref()).map_err(|err| RouteStateReadError::ReadFailure {
            path: err.path,
            source: err.source,
        })?;
    load_route_state_at_path(&state_path, pipeline_id, None, &run_inventory)
}

pub fn load_route_state_with_storage_layout(
    repo_root: impl AsRef<Path>,
    pipeline_id: impl AsRef<str>,
    storage_layout: PipelineStorageLayoutContract,
) -> Result<RouteState, RouteStateReadError> {
    let repo_root = repo_root.as_ref();
    let pipeline_id = pipeline_id.as_ref();
    let state_path = route_state_path_with_storage_layout(repo_root, pipeline_id, storage_layout)
        .map_err(|reason| RouteStateReadError::InvalidPipelineId {
        pipeline_id: pipeline_id.to_string(),
        reason,
    })?;
    let run_inventory =
        load_run_inventory(repo_root).map_err(|err| RouteStateReadError::ReadFailure {
            path: err.path,
            source: err.source,
        })?;
    load_route_state_at_path(&state_path, pipeline_id, None, &run_inventory)
}

pub fn load_route_state_with_supported_variables(
    repo_root: impl AsRef<Path>,
    pipeline_id: impl AsRef<str>,
    supported_variables: &BTreeSet<String>,
) -> Result<RouteState, RouteStateReadError> {
    let pipeline_id = pipeline_id.as_ref();
    let state_path = route_state_path(repo_root.as_ref(), pipeline_id).map_err(|reason| {
        RouteStateReadError::InvalidPipelineId {
            pipeline_id: pipeline_id.to_string(),
            reason,
        }
    })?;
    let run_inventory =
        load_run_inventory(repo_root.as_ref()).map_err(|err| RouteStateReadError::ReadFailure {
            path: err.path,
            source: err.source,
        })?;
    load_route_state_at_path(
        &state_path,
        pipeline_id,
        Some(supported_variables),
        &run_inventory,
    )
}

pub(crate) fn load_route_state_with_supported_variables_and_storage_layout(
    repo_root: impl AsRef<Path>,
    pipeline_id: impl AsRef<str>,
    supported_variables: &BTreeSet<String>,
    storage_layout: PipelineStorageLayoutContract,
) -> Result<RouteState, RouteStateReadError> {
    let repo_root = repo_root.as_ref();
    let pipeline_id = pipeline_id.as_ref();
    let state_path = route_state_path_with_storage_layout(repo_root, pipeline_id, storage_layout)
        .map_err(|reason| RouteStateReadError::InvalidPipelineId {
        pipeline_id: pipeline_id.to_string(),
        reason,
    })?;
    let run_inventory =
        load_run_inventory(repo_root).map_err(|err| RouteStateReadError::ReadFailure {
            path: err.path,
            source: err.source,
        })?;
    load_route_state_at_path(
        &state_path,
        pipeline_id,
        Some(supported_variables),
        &run_inventory,
    )
}

pub fn set_route_state(
    repo_root: impl AsRef<Path>,
    pipeline_id: impl AsRef<str>,
    supported_variables: impl IntoIterator<Item = impl AsRef<str>>,
    mutation: RouteStateMutation,
    expected_revision: u64,
) -> Result<RouteStateMutationOutcome, RouteStateStoreError> {
    let pipeline_id = pipeline_id.as_ref();
    validate_pipeline_id(pipeline_id).map_err(|reason| {
        RouteStateStoreError::InvalidPipelineId {
            pipeline_id: pipeline_id.to_string(),
            reason,
        }
    })?;

    let supported_variables = normalize_supported_variables(supported_variables)?;
    let run_inventory = load_run_inventory(repo_root.as_ref()).map_err(|err| {
        RouteStateStoreError::ReadFailure {
            path: err.path,
            source: err.source,
        }
    })?;
    validate_mutation(&mutation, &supported_variables, &run_inventory)?;

    let state_path = route_state_path(repo_root.as_ref(), pipeline_id).map_err(|reason| {
        RouteStateStoreError::InvalidPipelineId {
            pipeline_id: pipeline_id.to_string(),
            reason,
        }
    })?;
    ensure_state_parent_dir(&state_path).map_err(|source| RouteStateStoreError::ReadFailure {
        path: state_path.clone(),
        source,
    })?;

    let _lock = acquire_advisory_lock(&state_path)?;
    let mut state = match load_route_state_at_path(
        &state_path,
        pipeline_id,
        Some(&supported_variables),
        &run_inventory,
    ) {
        Ok(state) => state,
        Err(RouteStateReadError::MalformedState { reason, .. }) => {
            return Ok(RouteStateMutationOutcome::Refused(
                RouteStateMutationRefusal::MalformedState { reason },
            ));
        }
        Err(RouteStateReadError::InvalidPipelineId {
            pipeline_id,
            reason,
        }) => {
            return Err(RouteStateStoreError::InvalidPipelineId {
                pipeline_id,
                reason,
            });
        }
        Err(RouteStateReadError::ReadFailure { path, source }) => {
            return Err(RouteStateStoreError::ReadFailure { path, source });
        }
    };

    if let RouteStateMutation::RoutingVariable { variable, .. } = &mutation {
        if !supported_variables.contains(variable) {
            return Ok(RouteStateMutationOutcome::Refused(
                RouteStateMutationRefusal::UnsupportedVariable {
                    variable: variable.clone(),
                },
            ));
        }
    }

    if state.revision != expected_revision {
        return Ok(RouteStateMutationOutcome::Refused(
            RouteStateMutationRefusal::RevisionConflict {
                expected_revision,
                actual_revision: state.revision,
            },
        ));
    }

    state.schema_version = ROUTE_STATE_SCHEMA_VERSION.to_string();
    state.revision = state.revision.saturating_add(1);
    mutation.apply(&mut state);
    state.run.repo_root = Some(derived_repo_root(repo_root.as_ref()));
    state.audit.push(RouteStateAuditEntry {
        revision: state.revision,
        field_path: mutation.field_path(),
        value: mutation.value(),
    });
    trim_audit_history(&mut state.audit);

    persist_route_state(&state_path, &state)?;

    Ok(RouteStateMutationOutcome::Applied(Box::new(state)))
}

#[allow(dead_code)]
pub fn set_route_state_with_storage_layout(
    repo_root: impl AsRef<Path>,
    pipeline_id: impl AsRef<str>,
    supported_variables: impl IntoIterator<Item = impl AsRef<str>>,
    mutation: RouteStateMutation,
    expected_revision: u64,
    storage_layout: PipelineStorageLayoutContract,
) -> Result<RouteStateMutationOutcome, RouteStateStoreError> {
    let repo_root = repo_root.as_ref();
    let pipeline_id = pipeline_id.as_ref();
    validate_pipeline_id(pipeline_id).map_err(|reason| {
        RouteStateStoreError::InvalidPipelineId {
            pipeline_id: pipeline_id.to_string(),
            reason,
        }
    })?;

    let supported_variables = normalize_supported_variables(supported_variables)?;
    let run_inventory =
        load_run_inventory(repo_root).map_err(|err| RouteStateStoreError::ReadFailure {
            path: err.path,
            source: err.source,
        })?;
    validate_mutation(&mutation, &supported_variables, &run_inventory)?;

    let state_path = route_state_path_with_storage_layout(repo_root, pipeline_id, storage_layout)
        .map_err(|reason| RouteStateStoreError::InvalidPipelineId {
        pipeline_id: pipeline_id.to_string(),
        reason,
    })?;
    ensure_state_parent_dir(&state_path).map_err(|source| RouteStateStoreError::ReadFailure {
        path: state_path.clone(),
        source,
    })?;

    let _lock = acquire_advisory_lock(&state_path)?;
    let mut state = match load_route_state_at_path(
        &state_path,
        pipeline_id,
        Some(&supported_variables),
        &run_inventory,
    ) {
        Ok(state) => state,
        Err(RouteStateReadError::MalformedState { reason, .. }) => {
            return Ok(RouteStateMutationOutcome::Refused(
                RouteStateMutationRefusal::MalformedState { reason },
            ));
        }
        Err(RouteStateReadError::InvalidPipelineId {
            pipeline_id,
            reason,
        }) => {
            return Err(RouteStateStoreError::InvalidPipelineId {
                pipeline_id,
                reason,
            });
        }
        Err(RouteStateReadError::ReadFailure { path, source }) => {
            return Err(RouteStateStoreError::ReadFailure { path, source });
        }
    };

    if let RouteStateMutation::RoutingVariable { variable, .. } = &mutation {
        if !supported_variables.contains(variable) {
            return Ok(RouteStateMutationOutcome::Refused(
                RouteStateMutationRefusal::UnsupportedVariable {
                    variable: variable.clone(),
                },
            ));
        }
    }

    if state.revision != expected_revision {
        return Ok(RouteStateMutationOutcome::Refused(
            RouteStateMutationRefusal::RevisionConflict {
                expected_revision,
                actual_revision: state.revision,
            },
        ));
    }

    state.schema_version = ROUTE_STATE_SCHEMA_VERSION.to_string();
    state.revision = state.revision.saturating_add(1);
    mutation.apply(&mut state);
    state.run.repo_root = Some(derived_repo_root(repo_root));
    state.audit.push(RouteStateAuditEntry {
        revision: state.revision,
        field_path: mutation.field_path(),
        value: mutation.value(),
    });
    trim_audit_history(&mut state.audit);

    persist_route_state(&state_path, &state)?;

    Ok(RouteStateMutationOutcome::Applied(Box::new(state)))
}

pub fn load_trusted_pipeline_session(
    repo_root: impl AsRef<Path>,
    pipeline: &PipelineDefinition,
) -> Result<TrustedPipelineSession, TrustedPipelineSessionRefusal> {
    load_trusted_pipeline_session_with_storage_layout(
        repo_root,
        pipeline,
        *handbook_product_pipeline_storage_layout_contract(),
    )
}

pub fn load_trusted_pipeline_session_with_storage_layout(
    repo_root: impl AsRef<Path>,
    pipeline: &PipelineDefinition,
    storage_layout: PipelineStorageLayoutContract,
) -> Result<TrustedPipelineSession, TrustedPipelineSessionRefusal> {
    let repo_root = repo_root.as_ref();
    let supported_variables = supported_route_state_variables(pipeline);
    let state = load_route_state_with_supported_variables_and_storage_layout(
        repo_root,
        &pipeline.header.id,
        &supported_variables,
        storage_layout,
    )
    .map_err(|err| classify_trusted_pipeline_session_read_error(&pipeline.header.id, err))?;
    let route_basis = state.route_basis.clone().ok_or_else(|| {
        TrustedPipelineSessionRefusal::MissingRouteBasis {
            pipeline_id: pipeline.header.id.clone(),
        }
    })?;
    let canonical_route_basis = rebuild_canonical_route_basis(repo_root, pipeline, &state)
        .map_err(|reason| TrustedPipelineSessionRefusal::StaleRouteBasis {
            pipeline_id: pipeline.header.id.clone(),
            reason: format!("failed to rebuild canonical route_basis: {reason}"),
        })?;

    if let Some(reason) = trusted_pipeline_session_stale_reason(
        repo_root,
        pipeline,
        &state,
        &route_basis,
        &canonical_route_basis,
    ) {
        return Err(TrustedPipelineSessionRefusal::StaleRouteBasis {
            pipeline_id: pipeline.header.id.clone(),
            reason,
        });
    }

    if let Some(reason) = route_basis_mismatch_reason(&route_basis, &canonical_route_basis) {
        return Err(TrustedPipelineSessionRefusal::MalformedRouteBasis {
            pipeline_id: pipeline.header.id.clone(),
            reason,
        });
    }

    let mut route_state = normalized_state_for_route_basis(&state, repo_root);
    route_state.route_basis = Some(canonical_route_basis.clone());

    Ok(TrustedPipelineSession {
        pipeline_id: pipeline.header.id.clone(),
        route_state,
        route_basis: canonical_route_basis,
    })
}

pub fn build_route_basis(
    repo_root: impl AsRef<Path>,
    pipeline: &PipelineDefinition,
    state: &RouteState,
    route: &ResolvedPipelineRoute,
) -> Result<RouteBasis, RouteBasisBuildError> {
    let repo_root = repo_root.as_ref();
    let normalized_state = normalized_state_for_route_basis(state, repo_root);
    let effective_run = effective_route_basis_run(repo_root, pipeline, state);
    let selected_runner_id = effective_run
        .runner
        .clone()
        .expect("effective route-basis run always resolves runner");
    let selected_profile_id = effective_run
        .profile
        .clone()
        .expect("effective route-basis run always resolves profile");

    if selected_runner_id.trim().is_empty() {
        return Err(RouteBasisBuildError::MissingSelectedRunner {
            pipeline_id: pipeline.header.id.clone(),
        });
    }
    if selected_profile_id.trim().is_empty() {
        return Err(RouteBasisBuildError::MissingSelectedProfile {
            pipeline_id: pipeline.header.id.clone(),
        });
    }
    let profile_inventory =
        load_profile_inventory(repo_root).map_err(|err| RouteBasisBuildError::ReadFailure {
            path: err.path,
            source: err.source,
        })?;
    if let Some(missing_files) = profile_inventory
        .incomplete_ids
        .get(&selected_profile_id)
        .cloned()
    {
        return Err(RouteBasisBuildError::IncompleteSelectedProfilePack {
            pipeline_id: pipeline.header.id.clone(),
            profile_id: selected_profile_id.clone(),
            missing_files,
        });
    }

    let pipeline_file_sha256 =
        fingerprint_repo_relative_file(repo_root, pipeline.source_path.to_string_lossy().as_ref())?;

    let mut route_stages = Vec::with_capacity(route.stages.len());
    for resolved_stage in &route.stages {
        let Some(pipeline_stage) = pipeline
            .declared_stages()
            .iter()
            .find(|stage| stage.id == resolved_stage.stage_id)
        else {
            return Err(RouteBasisBuildError::InvalidRouteSnapshot {
                pipeline_id: pipeline.header.id.clone(),
                stage_id: resolved_stage.stage_id.clone(),
                detail: "resolved route stage is not declared in the selected pipeline".to_string(),
            });
        };

        let file_sha256 = fingerprint_repo_relative_file(repo_root, &pipeline_stage.file)?;
        route_stages.push(RouteBasisResolvedStage {
            stage_id: resolved_stage.stage_id.clone(),
            file: pipeline_stage.file.clone(),
            status: RouteBasisStageStatus::from(resolved_stage.status),
            reason: resolved_stage
                .reason
                .clone()
                .map(RouteBasisStageReason::from),
            file_sha256,
        });
    }

    let runner_file = runner_repo_file(&selected_runner_id);
    let profile_yaml = profile_repo_file(&selected_profile_id, "profile.yaml");
    let commands_yaml = profile_repo_file(&selected_profile_id, "commands.yaml");
    let conventions_md = profile_repo_file(&selected_profile_id, "conventions.md");

    Ok(RouteBasis {
        schema_version: ROUTE_BASIS_SCHEMA_VERSION.to_string(),
        pipeline_id: pipeline.header.id.clone(),
        pipeline_file: pipeline.source_path.to_string_lossy().into_owned(),
        pipeline_file_sha256,
        state_revision: normalized_state.revision,
        routing: normalized_state.routing.clone(),
        refs: normalized_state.refs.clone(),
        run: effective_run.clone(),
        route: route_stages,
        runner: RouteBasisRunner {
            id: selected_runner_id.clone(),
            file: runner_file.clone(),
            file_sha256: fingerprint_repo_relative_file(repo_root, &runner_file)?,
        },
        profile: RouteBasisProfilePack {
            id: selected_profile_id.clone(),
            profile_yaml_sha256: fingerprint_repo_relative_file(repo_root, &profile_yaml)?,
            commands_yaml_sha256: fingerprint_repo_relative_file(repo_root, &commands_yaml)?,
            conventions_md_sha256: fingerprint_repo_relative_file(repo_root, &conventions_md)?,
        },
    })
}

pub fn rebuild_canonical_route_basis(
    repo_root: impl AsRef<Path>,
    pipeline: &PipelineDefinition,
    state: &RouteState,
) -> Result<RouteBasis, String> {
    let repo_root = repo_root.as_ref();
    let route_variables =
        RouteVariables::new(state.routing.clone()).map_err(|err| err.to_string())?;
    let route =
        resolve_pipeline_route(pipeline, &route_variables).map_err(|err| err.to_string())?;
    build_route_basis(repo_root, pipeline, state, &route).map_err(|err| err.to_string())
}

pub fn route_basis_mismatch_reason(
    candidate: &RouteBasis,
    canonical: &RouteBasis,
) -> Option<String> {
    if candidate.schema_version != canonical.schema_version {
        return Some(format!(
            "route_basis schema_version `{}` does not match canonical `{}`",
            candidate.schema_version, canonical.schema_version
        ));
    }
    if candidate.pipeline_id != canonical.pipeline_id {
        return Some(format!(
            "route_basis pipeline_id `{}` does not match canonical `{}`",
            candidate.pipeline_id, canonical.pipeline_id
        ));
    }
    if candidate.pipeline_file != canonical.pipeline_file {
        return Some(format!(
            "route_basis pipeline_file `{}` does not match canonical `{}`",
            candidate.pipeline_file, canonical.pipeline_file
        ));
    }
    if candidate.pipeline_file_sha256 != canonical.pipeline_file_sha256 {
        return Some(
            "route_basis pipeline_file_sha256 does not match canonical fingerprint".to_string(),
        );
    }
    if candidate.state_revision != canonical.state_revision {
        return Some(format!(
            "route_basis state_revision {} does not match canonical {}",
            candidate.state_revision, canonical.state_revision
        ));
    }
    if candidate.routing != canonical.routing {
        return Some("route_basis routing does not match the canonical route snapshot".to_string());
    }
    if candidate.refs != canonical.refs {
        return Some("route_basis refs do not match the canonical route snapshot".to_string());
    }
    if normalize_route_basis_run(&candidate.run) != normalize_route_basis_run(&canonical.run) {
        return Some("route_basis run does not match the canonical route snapshot".to_string());
    }
    if candidate.runner.id != canonical.runner.id {
        return Some(format!(
            "route_basis runner.id `{}` does not match canonical `{}`",
            candidate.runner.id, canonical.runner.id
        ));
    }
    if candidate.runner.file != canonical.runner.file {
        return Some(format!(
            "route_basis runner.file `{}` does not match canonical `{}`",
            candidate.runner.file, canonical.runner.file
        ));
    }
    if candidate.runner.file_sha256 != canonical.runner.file_sha256 {
        return Some(
            "route_basis runner.file_sha256 does not match canonical fingerprint".to_string(),
        );
    }
    if candidate.profile.id != canonical.profile.id {
        return Some(format!(
            "route_basis profile.id `{}` does not match canonical `{}`",
            candidate.profile.id, canonical.profile.id
        ));
    }
    if candidate.profile.profile_yaml_sha256 != canonical.profile.profile_yaml_sha256 {
        return Some(
            "route_basis profile.profile_yaml_sha256 does not match canonical fingerprint"
                .to_string(),
        );
    }
    if candidate.profile.commands_yaml_sha256 != canonical.profile.commands_yaml_sha256 {
        return Some(
            "route_basis profile.commands_yaml_sha256 does not match canonical fingerprint"
                .to_string(),
        );
    }
    if candidate.profile.conventions_md_sha256 != canonical.profile.conventions_md_sha256 {
        return Some(
            "route_basis profile.conventions_md_sha256 does not match canonical fingerprint"
                .to_string(),
        );
    }
    if candidate.route.len() != canonical.route.len() {
        return Some(format!(
            "route_basis route length {} does not match canonical length {}",
            candidate.route.len(),
            canonical.route.len()
        ));
    }

    for (index, (candidate_stage, canonical_stage)) in candidate
        .route
        .iter()
        .zip(canonical.route.iter())
        .enumerate()
    {
        if candidate_stage.stage_id != canonical_stage.stage_id {
            return Some(format!(
                "route_basis route entry {} stage_id `{}` does not match canonical `{}`",
                index + 1,
                candidate_stage.stage_id,
                canonical_stage.stage_id
            ));
        }
        if candidate_stage.file != canonical_stage.file {
            return Some(format!(
                "route_basis stage `{}` file `{}` does not match canonical `{}`",
                canonical_stage.stage_id, candidate_stage.file, canonical_stage.file
            ));
        }
        if candidate_stage.status != canonical_stage.status {
            return Some(format!(
                "route_basis stage `{}` status `{}` does not match canonical `{}`",
                canonical_stage.stage_id,
                route_basis_stage_status_label(candidate_stage.status),
                route_basis_stage_status_label(canonical_stage.status)
            ));
        }
        if candidate_stage.reason != canonical_stage.reason {
            return Some(format!(
                "route_basis stage `{}` reason does not match the canonical route result",
                canonical_stage.stage_id
            ));
        }
        if candidate_stage.file_sha256 != canonical_stage.file_sha256 {
            return Some(format!(
                "route_basis stage `{}` file_sha256 does not match canonical fingerprint",
                canonical_stage.stage_id
            ));
        }
    }

    None
}

fn classify_trusted_pipeline_session_read_error(
    pipeline_id: &str,
    err: RouteStateReadError,
) -> TrustedPipelineSessionRefusal {
    match err {
        RouteStateReadError::InvalidPipelineId {
            pipeline_id,
            reason,
        } => TrustedPipelineSessionRefusal::InvalidPipelineId {
            pipeline_id,
            reason,
        },
        RouteStateReadError::ReadFailure { path, source } => {
            TrustedPipelineSessionRefusal::ReadFailure {
                path,
                reason: source.to_string(),
            }
        }
        RouteStateReadError::MalformedState { path, reason } => {
            if malformed_state_targets_route_basis(&reason) {
                TrustedPipelineSessionRefusal::MalformedRouteBasis {
                    pipeline_id: pipeline_id.to_string(),
                    reason,
                }
            } else {
                TrustedPipelineSessionRefusal::InvalidState { path, reason }
            }
        }
    }
}

fn malformed_state_targets_route_basis(reason: &str) -> bool {
    let trimmed = reason.trim_start();
    trimmed.starts_with("route_basis ")
        || trimmed.starts_with("route_basis.")
        || trimmed.starts_with("route_basis:")
        || trimmed.starts_with("unsupported route_basis ")
}

fn trusted_pipeline_session_stale_reason(
    repo_root: &Path,
    pipeline: &PipelineDefinition,
    state: &RouteState,
    basis: &RouteBasis,
    canonical_basis: &RouteBasis,
) -> Option<String> {
    if state.revision != basis.state_revision {
        return Some(format!(
            "route state revision {} does not match persisted route_basis revision {}",
            state.revision, basis.state_revision
        ));
    }

    let effective_run = effective_route_basis_run(repo_root, pipeline, state);
    if state.routing != basis.routing
        || state.refs != basis.refs
        || effective_run != normalize_route_basis_run(&basis.run)
    {
        return Some(
            "route_state routing/refs/run no longer match the persisted route_basis".to_string(),
        );
    }

    if basis.pipeline_file_sha256 != canonical_basis.pipeline_file_sha256 {
        return Some(
            "pipeline definition fingerprint drifted after the last `pipeline resolve`".to_string(),
        );
    }

    if basis.runner.file_sha256 != canonical_basis.runner.file_sha256 {
        return Some(format!(
            "runner document `{}` changed after the last `pipeline resolve`",
            basis.runner.file
        ));
    }

    if basis.profile.profile_yaml_sha256 != canonical_basis.profile.profile_yaml_sha256
        || basis.profile.commands_yaml_sha256 != canonical_basis.profile.commands_yaml_sha256
        || basis.profile.conventions_md_sha256 != canonical_basis.profile.conventions_md_sha256
    {
        return Some("selected profile pack changed after the last `pipeline resolve`".to_string());
    }

    if basis.route.len() == canonical_basis.route.len() {
        for (basis_stage, canonical_stage) in basis.route.iter().zip(canonical_basis.route.iter()) {
            if basis_stage.stage_id == canonical_stage.stage_id
                && basis_stage.file == canonical_stage.file
                && basis_stage.file_sha256 != canonical_stage.file_sha256
            {
                return Some(format!(
                    "stage file `{}` changed after the last `pipeline resolve`",
                    basis_stage.file
                ));
            }
        }
    }

    None
}

pub fn persist_route_basis(
    repo_root: impl AsRef<Path>,
    pipeline_id: impl AsRef<str>,
    route_basis: RouteBasis,
) -> Result<RouteBasisPersistOutcome, RouteStateStoreError> {
    let repo_root = repo_root.as_ref();
    let pipeline_id = pipeline_id.as_ref();
    validate_pipeline_id(pipeline_id).map_err(|reason| {
        RouteStateStoreError::InvalidPipelineId {
            pipeline_id: pipeline_id.to_string(),
            reason,
        }
    })?;

    let state_path = route_state_path(repo_root, pipeline_id).map_err(|reason| {
        RouteStateStoreError::InvalidPipelineId {
            pipeline_id: pipeline_id.to_string(),
            reason,
        }
    })?;
    ensure_state_parent_dir(&state_path).map_err(|source| RouteStateStoreError::ReadFailure {
        path: state_path.clone(),
        source,
    })?;

    let run_inventory =
        load_run_inventory(repo_root).map_err(|err| RouteStateStoreError::ReadFailure {
            path: err.path,
            source: err.source,
        })?;
    let _lock = acquire_advisory_lock(&state_path)?;
    let mut state = match load_route_state_at_path(&state_path, pipeline_id, None, &run_inventory) {
        Ok(state) => state,
        Err(RouteStateReadError::MalformedState { reason, .. }) => {
            return Ok(RouteBasisPersistOutcome::Refused(
                RouteBasisPersistRefusal::MalformedState { reason },
            ));
        }
        Err(RouteStateReadError::InvalidPipelineId {
            pipeline_id,
            reason,
        }) => {
            return Err(RouteStateStoreError::InvalidPipelineId {
                pipeline_id,
                reason,
            });
        }
        Err(RouteStateReadError::ReadFailure { path, source }) => {
            return Err(RouteStateStoreError::ReadFailure { path, source });
        }
    };

    let pipeline = load_selected_pipeline_definition(repo_root, pipeline_id).map_err(|err| {
        RouteStateStoreError::InvalidMutation {
            reason: format!(
                "failed to load selected pipeline definition for route_basis persistence: {err}"
            ),
        }
    })?;
    let effective_run = effective_route_basis_run(repo_root, &pipeline, &state);
    state = normalized_state_for_persistence(&state, repo_root);
    if state.revision != route_basis.state_revision {
        return Ok(RouteBasisPersistOutcome::Refused(
            RouteBasisPersistRefusal::RevisionConflict {
                expected_revision: route_basis.state_revision,
                actual_revision: state.revision,
            },
        ));
    }
    if state.routing != route_basis.routing
        || state.refs != route_basis.refs
        || effective_run != normalize_route_basis_run(&route_basis.run)
    {
        return Ok(RouteBasisPersistOutcome::Refused(
            RouteBasisPersistRefusal::MalformedState {
                reason: "route_basis snapshot does not match the current route-state surfaces"
                    .to_string(),
            },
        ));
    }

    let canonical_route_basis = rebuild_canonical_route_basis(repo_root, &pipeline, &state)
        .map_err(|reason| RouteStateStoreError::InvalidMutation {
            reason: format!("failed to rebuild canonical route_basis during persistence: {reason}"),
        })?;
    if let Some(reason) = route_basis_mismatch_reason(&route_basis, &canonical_route_basis) {
        return Ok(RouteBasisPersistOutcome::Refused(
            RouteBasisPersistRefusal::MalformedState { reason },
        ));
    }

    state.schema_version = ROUTE_STATE_SCHEMA_VERSION.to_string();
    state.route_basis = Some(canonical_route_basis);
    persist_route_state(&state_path, &state)?;

    Ok(RouteBasisPersistOutcome::Applied(Box::new(state)))
}

pub fn persist_route_basis_with_storage_layout(
    repo_root: impl AsRef<Path>,
    pipeline_id: impl AsRef<str>,
    route_basis: RouteBasis,
    storage_layout: PipelineStorageLayoutContract,
) -> Result<RouteBasisPersistOutcome, RouteStateStoreError> {
    let repo_root = repo_root.as_ref();
    let pipeline_id = pipeline_id.as_ref();
    validate_pipeline_id(pipeline_id).map_err(|reason| {
        RouteStateStoreError::InvalidPipelineId {
            pipeline_id: pipeline_id.to_string(),
            reason,
        }
    })?;

    let state_path = route_state_path_with_storage_layout(repo_root, pipeline_id, storage_layout)
        .map_err(|reason| RouteStateStoreError::InvalidPipelineId {
        pipeline_id: pipeline_id.to_string(),
        reason,
    })?;
    ensure_state_parent_dir(&state_path).map_err(|source| RouteStateStoreError::ReadFailure {
        path: state_path.clone(),
        source,
    })?;

    let run_inventory =
        load_run_inventory(repo_root).map_err(|err| RouteStateStoreError::ReadFailure {
            path: err.path,
            source: err.source,
        })?;
    let _lock = acquire_advisory_lock(&state_path)?;
    let mut state = match load_route_state_at_path(&state_path, pipeline_id, None, &run_inventory) {
        Ok(state) => state,
        Err(RouteStateReadError::MalformedState { reason, .. }) => {
            return Ok(RouteBasisPersistOutcome::Refused(
                RouteBasisPersistRefusal::MalformedState { reason },
            ));
        }
        Err(RouteStateReadError::InvalidPipelineId {
            pipeline_id,
            reason,
        }) => {
            return Err(RouteStateStoreError::InvalidPipelineId {
                pipeline_id,
                reason,
            });
        }
        Err(RouteStateReadError::ReadFailure { path, source }) => {
            return Err(RouteStateStoreError::ReadFailure { path, source });
        }
    };

    let pipeline = load_selected_pipeline_definition(repo_root, pipeline_id).map_err(|err| {
        RouteStateStoreError::InvalidMutation {
            reason: format!(
                "failed to load selected pipeline definition for route_basis persistence: {err}"
            ),
        }
    })?;
    let effective_run = effective_route_basis_run(repo_root, &pipeline, &state);
    state = normalized_state_for_persistence(&state, repo_root);
    if state.revision != route_basis.state_revision {
        return Ok(RouteBasisPersistOutcome::Refused(
            RouteBasisPersistRefusal::RevisionConflict {
                expected_revision: route_basis.state_revision,
                actual_revision: state.revision,
            },
        ));
    }
    if state.routing != route_basis.routing
        || state.refs != route_basis.refs
        || effective_run != normalize_route_basis_run(&route_basis.run)
    {
        return Ok(RouteBasisPersistOutcome::Refused(
            RouteBasisPersistRefusal::MalformedState {
                reason: "route_basis snapshot does not match the current route-state surfaces"
                    .to_string(),
            },
        ));
    }

    let canonical_route_basis = rebuild_canonical_route_basis(repo_root, &pipeline, &state)
        .map_err(|reason| RouteStateStoreError::InvalidMutation {
            reason: format!("failed to rebuild canonical route_basis during persistence: {reason}"),
        })?;
    if let Some(reason) = route_basis_mismatch_reason(&route_basis, &canonical_route_basis) {
        return Ok(RouteBasisPersistOutcome::Refused(
            RouteBasisPersistRefusal::MalformedState { reason },
        ));
    }

    state.schema_version = ROUTE_STATE_SCHEMA_VERSION.to_string();
    state.route_basis = Some(canonical_route_basis);
    persist_route_state(&state_path, &state)?;

    Ok(RouteBasisPersistOutcome::Applied(Box::new(state)))
}

fn load_route_state_at_path(
    state_path: &Path,
    pipeline_id: &str,
    supported_variables: Option<&BTreeSet<String>>,
    run_inventory: &RouteStateRunInventory,
) -> Result<RouteState, RouteStateReadError> {
    let file_meta = match fs::symlink_metadata(state_path) {
        Ok(meta) => Some(meta),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
        Err(source) => {
            return Err(RouteStateReadError::ReadFailure {
                path: state_path.to_path_buf(),
                source,
            });
        }
    };

    if file_meta.is_none() {
        let state = RouteState::empty(pipeline_id);
        return Ok(state);
    }

    let meta = file_meta.expect("file_meta present");
    if !meta.is_file() {
        return Err(RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: "state file must be a regular file".to_string(),
        });
    }

    let contents =
        read_file_no_follow(state_path).map_err(|source| RouteStateReadError::ReadFailure {
            path: state_path.to_path_buf(),
            source,
        })?;

    let state: RouteState = serde_yaml_bw::from_str(&contents).map_err(|source| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: source.to_string(),
        }
    })?;

    validate_loaded_state(
        &state,
        pipeline_id,
        supported_variables,
        run_inventory,
        state_path,
    )?;

    Ok(state)
}

fn validate_loaded_state(
    state: &RouteState,
    expected_pipeline_id: &str,
    supported_variables: Option<&BTreeSet<String>>,
    run_inventory: &RouteStateRunInventory,
    state_path: &Path,
) -> Result<(), RouteStateReadError> {
    if state.schema_version != ROUTE_STATE_SCHEMA_VERSION
        && state.schema_version != LEGACY_ROUTE_STATE_SCHEMA_VERSION
    {
        return Err(RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: format!(
                "unexpected schema_version `{}`; expected `{}` or `{}`",
                state.schema_version, ROUTE_STATE_SCHEMA_VERSION, LEGACY_ROUTE_STATE_SCHEMA_VERSION
            ),
        });
    }

    if state.pipeline_id != expected_pipeline_id {
        return Err(RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: format!(
                "pipeline_id `{}` does not match expected `{}`",
                state.pipeline_id, expected_pipeline_id
            ),
        });
    }

    validate_routing_map(&state.routing).map_err(|reason| RouteStateReadError::MalformedState {
        path: state_path.to_path_buf(),
        reason,
    })?;
    validate_refs(&state.refs).map_err(|reason| RouteStateReadError::MalformedState {
        path: state_path.to_path_buf(),
        reason,
    })?;
    validate_state_run(&state.run, run_inventory).map_err(|reason| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason,
        }
    })?;
    validate_audit_entries(&state.audit, supported_variables, run_inventory).map_err(|reason| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason,
        }
    })?;
    validate_route_basis(state, supported_variables, run_inventory, state_path)?;

    if let Some(supported_variables) = supported_variables {
        for variable in state.routing.keys() {
            if !supported_variables.contains(variable) {
                return Err(RouteStateReadError::MalformedState {
                    path: state_path.to_path_buf(),
                    reason: format!("unsupported routing variable `{variable}` in persisted state"),
                });
            }
        }
    }

    Ok(())
}

fn validate_mutation(
    mutation: &RouteStateMutation,
    supported_variables: &BTreeSet<String>,
    run_inventory: &RouteStateRunInventory,
) -> Result<(), RouteStateStoreError> {
    match mutation {
        RouteStateMutation::RoutingVariable { variable, .. } => {
            validate_variable_name(variable)
                .map_err(|reason| RouteStateStoreError::InvalidMutation { reason })?;

            let values = supported_variables
                .iter()
                .map(|variable| (variable.clone(), false))
                .collect::<BTreeMap<_, _>>();
            RouteVariables::new(values).map_err(|err| {
                RouteStateStoreError::InvalidSupportedVariables {
                    reason: err.to_string(),
                }
            })?;
        }
        RouteStateMutation::RefCharterRef { value }
        | RouteStateMutation::RefProjectContextRef { value } => {
            validate_repo_relative_ref(value)
                .map_err(|reason| RouteStateStoreError::InvalidMutation { reason })?
        }
        RouteStateMutation::RunRunner { value } => validate_inventory_value(
            value,
            FIELD_RUN_RUNNER,
            RUNNERS_ROOT_DISPLAY,
            &run_inventory.runners,
        )
        .map_err(|reason| RouteStateStoreError::InvalidMutation { reason })?,
        RouteStateMutation::RunProfile { value } => {
            validate_profile_inventory_value(value, FIELD_RUN_PROFILE, &run_inventory.profiles)
                .map_err(|reason| RouteStateStoreError::InvalidMutation { reason })?
        }
    }

    Ok(())
}

fn validate_audit_entries(
    audit: &[RouteStateAuditEntry],
    supported_variables: Option<&BTreeSet<String>>,
    run_inventory: &RouteStateRunInventory,
) -> Result<(), String> {
    for entry in audit {
        let field = parse_route_state_field_path(&entry.field_path)?;
        match field {
            RouteStateFieldPath::Routing(variable) => {
                match &entry.value {
                    RouteStateValue::Bool(_) => {}
                    RouteStateValue::String(_) => {
                        return Err(format!(
                            "audit field `{}` must use a boolean value",
                            entry.field_path
                        ));
                    }
                }

                if let Some(supported_variables) = supported_variables {
                    if !supported_variables.contains(variable) {
                        return Err(format!(
                            "unsupported audit routing variable `{variable}` in persisted state"
                        ));
                    }
                }
            }
            RouteStateFieldPath::RefsCharterRef | RouteStateFieldPath::RefsProjectContextRef => {
                match &entry.value {
                    RouteStateValue::String(value) => validate_repo_relative_ref(value)?,
                    RouteStateValue::Bool(_) => {
                        return Err(format!(
                            "audit field `{}` must use a string value",
                            entry.field_path
                        ));
                    }
                }
            }
            RouteStateFieldPath::RunRunner | RouteStateFieldPath::RunProfile => {
                match &entry.value {
                    RouteStateValue::String(value) => match field {
                        RouteStateFieldPath::RunRunner => validate_inventory_value(
                            value,
                            &entry.field_path,
                            RUNNERS_ROOT_DISPLAY,
                            &run_inventory.runners,
                        )?,
                        RouteStateFieldPath::RunProfile => validate_profile_inventory_value(
                            value,
                            &entry.field_path,
                            &run_inventory.profiles,
                        )?,
                        _ => unreachable!("run field match already constrained"),
                    },
                    RouteStateValue::Bool(_) => {
                        return Err(format!(
                            "audit field `{}` must use a string value",
                            entry.field_path
                        ));
                    }
                }
            }
        }
    }

    Ok(())
}

fn validate_routing_map(values: &BTreeMap<String, bool>) -> Result<(), String> {
    let values = values.iter().map(|(name, value)| (name.clone(), *value));
    RouteVariables::new(values.collect()).map_err(|err| err.to_string())?;
    Ok(())
}

fn validate_refs(refs: &RouteStateRefs) -> Result<(), String> {
    if let Some(value) = &refs.charter_ref {
        validate_repo_relative_ref(value)?;
    }
    if let Some(value) = &refs.project_context_ref {
        validate_repo_relative_ref(value)?;
    }
    Ok(())
}

fn validate_state_run(
    run: &RouteStateRun,
    run_inventory: &RouteStateRunInventory,
) -> Result<(), String> {
    if let Some(value) = &run.runner {
        validate_inventory_value(
            value,
            FIELD_RUN_RUNNER,
            RUNNERS_ROOT_DISPLAY,
            &run_inventory.runners,
        )?;
    }
    if let Some(value) = &run.profile {
        validate_profile_inventory_value(value, FIELD_RUN_PROFILE, &run_inventory.profiles)?;
    }
    if let Some(value) = &run.repo_root {
        validate_repo_root(value)?;
    }
    Ok(())
}

fn validate_route_basis_run(
    run: &RouteStateRun,
    run_inventory: &RouteStateRunInventory,
) -> Result<(), String> {
    if let Some(value) = &run.runner {
        validate_inventory_value(
            value,
            FIELD_RUN_RUNNER,
            RUNNERS_ROOT_DISPLAY,
            &run_inventory.runners,
        )?;
    }
    if let Some(value) = &run.profile {
        validate_profile_inventory_value(value, FIELD_RUN_PROFILE, &run_inventory.profiles)?;
    }
    if let Some(value) = &run.repo_root {
        validate_route_basis_repo_root(value)?;
    }
    Ok(())
}

fn validate_route_basis(
    state: &RouteState,
    supported_variables: Option<&BTreeSet<String>>,
    run_inventory: &RouteStateRunInventory,
    state_path: &Path,
) -> Result<(), RouteStateReadError> {
    let Some(route_basis) = &state.route_basis else {
        return Ok(());
    };

    if state.schema_version == LEGACY_ROUTE_STATE_SCHEMA_VERSION {
        return Err(RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: format!(
                "route_basis requires schema_version `{}`",
                ROUTE_STATE_SCHEMA_VERSION
            ),
        });
    }
    if route_basis.schema_version != ROUTE_BASIS_SCHEMA_VERSION {
        return Err(RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: format!(
                "route_basis schema_version `{}` does not match expected `{}`",
                route_basis.schema_version, ROUTE_BASIS_SCHEMA_VERSION
            ),
        });
    }
    if route_basis.pipeline_id != state.pipeline_id {
        return Err(RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: format!(
                "route_basis pipeline_id `{}` does not match persisted pipeline_id `{}`",
                route_basis.pipeline_id, state.pipeline_id
            ),
        });
    }
    validate_repo_relative_ref(&route_basis.pipeline_file).map_err(|reason| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: format!("route_basis pipeline_file is invalid: {reason}"),
        }
    })?;
    validate_sha256(&route_basis.pipeline_file_sha256).map_err(|reason| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: format!("route_basis pipeline_file_sha256 is invalid: {reason}"),
        }
    })?;
    validate_routing_map(&route_basis.routing).map_err(|reason| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: format!("route_basis routing is invalid: {reason}"),
        }
    })?;
    validate_refs(&route_basis.refs).map_err(|reason| RouteStateReadError::MalformedState {
        path: state_path.to_path_buf(),
        reason: format!("route_basis refs are invalid: {reason}"),
    })?;
    validate_route_basis_run(&route_basis.run, run_inventory).map_err(|reason| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: format!("route_basis run is invalid: {reason}"),
        }
    })?;
    if let Some(supported_variables) = supported_variables {
        for variable in route_basis.routing.keys() {
            if !supported_variables.contains(variable) {
                return Err(RouteStateReadError::MalformedState {
                    path: state_path.to_path_buf(),
                    reason: format!(
                        "unsupported route_basis routing variable `{variable}` in persisted state"
                    ),
                });
            }
        }
    }

    validate_inventory_value(
        &route_basis.runner.id,
        "route_basis.runner.id",
        RUNNERS_ROOT_DISPLAY,
        &run_inventory.runners,
    )
    .map_err(|reason| RouteStateReadError::MalformedState {
        path: state_path.to_path_buf(),
        reason,
    })?;
    validate_repo_relative_ref(&route_basis.runner.file).map_err(|reason| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: format!("route_basis runner.file is invalid: {reason}"),
        }
    })?;
    validate_sha256(&route_basis.runner.file_sha256).map_err(|reason| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: format!("route_basis runner.file_sha256 is invalid: {reason}"),
        }
    })?;
    validate_profile_inventory_value(
        &route_basis.profile.id,
        "route_basis.profile.id",
        &run_inventory.profiles,
    )
    .map_err(|reason| RouteStateReadError::MalformedState {
        path: state_path.to_path_buf(),
        reason,
    })?;
    validate_sha256(&route_basis.profile.profile_yaml_sha256).map_err(|reason| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: format!("route_basis profile.profile_yaml_sha256 is invalid: {reason}"),
        }
    })?;
    validate_sha256(&route_basis.profile.commands_yaml_sha256).map_err(|reason| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: format!("route_basis profile.commands_yaml_sha256 is invalid: {reason}"),
        }
    })?;
    validate_sha256(&route_basis.profile.conventions_md_sha256).map_err(|reason| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: format!("route_basis profile.conventions_md_sha256 is invalid: {reason}"),
        }
    })?;

    for stage in &route_basis.route {
        validate_repo_relative_ref(&stage.file).map_err(|reason| {
            RouteStateReadError::MalformedState {
                path: state_path.to_path_buf(),
                reason: format!(
                    "route_basis stage `{}` file is invalid: {reason}",
                    stage.stage_id
                ),
            }
        })?;
        validate_sha256(&stage.file_sha256).map_err(|reason| {
            RouteStateReadError::MalformedState {
                path: state_path.to_path_buf(),
                reason: format!(
                    "route_basis stage `{}` file_sha256 is invalid: {reason}",
                    stage.stage_id
                ),
            }
        })?;
    }

    Ok(())
}

fn validate_variable_name(variable: &str) -> Result<(), String> {
    let mut values = BTreeMap::new();
    values.insert(variable.to_string(), false);
    RouteVariables::new(values)
        .map(|_| ())
        .map_err(|err| err.to_string())
}

fn validate_non_empty_string(value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err("value must not be empty".to_string())
    } else {
        Ok(())
    }
}

fn route_basis_stage_status_label(status: RouteBasisStageStatus) -> &'static str {
    match status {
        RouteBasisStageStatus::Active => "active",
        RouteBasisStageStatus::Skipped => "skipped",
        RouteBasisStageStatus::Blocked => "blocked",
        RouteBasisStageStatus::Next => "next",
    }
}

fn validate_inventory_value(
    value: &str,
    field_path: &str,
    inventory_root: &str,
    allowlisted_ids: &BTreeSet<String>,
) -> Result<(), String> {
    validate_non_empty_string(value)?;
    if allowlisted_ids.contains(value) {
        return Ok(());
    }

    Err(format!(
        "{field_path} `{value}` is not declared under `{inventory_root}`"
    ))
}

fn validate_profile_inventory_value(
    value: &str,
    field_path: &str,
    profiles: &ProfilePackInventory,
) -> Result<(), String> {
    validate_non_empty_string(value)?;
    if profiles.complete_ids.contains(value) {
        return Ok(());
    }

    if let Some(missing_files) = profiles.incomplete_ids.get(value) {
        return Err(format_incomplete_profile_pack_reason(
            field_path,
            value,
            missing_files,
        ));
    }

    Err(format!(
        "{field_path} `{value}` is not declared under `{PROFILES_ROOT_DISPLAY}`"
    ))
}

fn format_incomplete_profile_pack_reason(
    field_path: &str,
    profile_id: &str,
    missing_files: &[&'static str],
) -> String {
    format!(
        "{field_path} `{profile_id}` points to incomplete profile pack `{PROFILES_ROOT_DISPLAY}{profile_id}/`: missing {}",
        missing_files.join(", ")
    )
}

fn format_selected_profile_pack_incomplete_reason(
    profile_id: &str,
    missing_files: &[&'static str],
) -> String {
    format!(
        "selected profile pack `{PROFILES_ROOT_DISPLAY}{profile_id}/` is incomplete: missing {}",
        missing_files.join(", ")
    )
}

fn validate_repo_relative_ref(value: &str) -> Result<(), String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err("repo-relative ref must not be empty".to_string());
    }

    let path = Path::new(trimmed);
    if path.is_absolute() {
        return Err(format!(
            "repo-relative ref `{trimmed}` must not be absolute"
        ));
    }

    let mut saw_normal = false;
    for component in path.components() {
        match component {
            Component::Normal(_) => saw_normal = true,
            Component::ParentDir => {
                return Err(format!(
                    "repo-relative ref `{trimmed}` must not escape the repo root"
                ));
            }
            Component::CurDir | Component::RootDir | Component::Prefix(_) => {
                return Err(format!(
                    "repo-relative ref `{trimmed}` must be a clean repo-relative path"
                ));
            }
        }
    }

    if !saw_normal {
        return Err(format!(
            "repo-relative ref `{trimmed}` must include at least one path component"
        ));
    }

    Ok(())
}

fn validate_repo_root(value: &str) -> Result<(), String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err("run.repo_root must not be empty".to_string());
    }

    let path = Path::new(trimmed);
    if !path.is_absolute() {
        return Err(format!("run.repo_root `{trimmed}` must be absolute"));
    }

    let mut saw_normal = false;
    for component in path.components() {
        match component {
            Component::Normal(_) => saw_normal = true,
            Component::RootDir => {}
            Component::ParentDir | Component::CurDir | Component::Prefix(_) => {
                return Err(format!(
                    "run.repo_root `{trimmed}` must be a clean absolute path"
                ));
            }
        }
    }

    if !saw_normal {
        return Err(format!(
            "run.repo_root `{trimmed}` must include at least one path component"
        ));
    }

    Ok(())
}

fn validate_route_basis_repo_root(value: &str) -> Result<(), String> {
    let trimmed = value.trim();
    if trimmed == ROUTE_BASIS_REPO_ROOT_SENTINEL {
        return Ok(());
    }

    validate_repo_root(value)
}

fn validate_sha256(value: &str) -> Result<(), String> {
    let trimmed = value.trim();
    if trimmed.len() != 64 {
        return Err("fingerprint must be 64 hex characters".to_string());
    }
    if trimmed.chars().all(|ch| ch.is_ascii_hexdigit()) {
        Ok(())
    } else {
        Err("fingerprint must be lowercase/uppercase hexadecimal".to_string())
    }
}

fn derived_repo_root(repo_root: &Path) -> String {
    repo_root.to_string_lossy().into_owned()
}

fn derived_route_basis_repo_root() -> String {
    ROUTE_BASIS_REPO_ROOT_SENTINEL.to_string()
}

pub fn normalize_route_basis_run(run: &RouteStateRun) -> RouteStateRun {
    let mut normalized = run.clone();
    if let Some(value) = normalized.repo_root.clone() {
        normalized.repo_root = if value == ROUTE_BASIS_REPO_ROOT_SENTINEL {
            Some(value)
        } else if validate_repo_root(&value).is_ok() {
            Some(derived_route_basis_repo_root())
        } else {
            Some(value)
        };
    }
    normalized
}

fn parse_route_state_field_path(input: &str) -> Result<RouteStateFieldPath<'_>, String> {
    if let Some(variable) = input.strip_prefix("routing.") {
        validate_variable_name(variable)?;
        return Ok(RouteStateFieldPath::Routing(variable));
    }

    match input {
        FIELD_REFS_CHARTER_REF => Ok(RouteStateFieldPath::RefsCharterRef),
        FIELD_REFS_PROJECT_CONTEXT_REF => Ok(RouteStateFieldPath::RefsProjectContextRef),
        FIELD_RUN_RUNNER => Ok(RouteStateFieldPath::RunRunner),
        FIELD_RUN_PROFILE => Ok(RouteStateFieldPath::RunProfile),
        _ => Err(format!("unsupported route-state field path `{input}`")),
    }
}

fn normalize_supported_variables<I, S>(
    supported_variables: I,
) -> Result<BTreeSet<String>, RouteStateStoreError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut values = BTreeMap::new();
    for variable in supported_variables {
        values.insert(variable.as_ref().to_string(), false);
    }

    let supported_variables = values.keys().cloned().collect::<BTreeSet<_>>();

    RouteVariables::new(values).map_err(|err| RouteStateStoreError::InvalidSupportedVariables {
        reason: err.to_string(),
    })?;

    Ok(supported_variables)
}

fn load_run_inventory(
    repo_root: &Path,
) -> Result<RouteStateRunInventory, RouteStateInventoryLoadError> {
    Ok(RouteStateRunInventory {
        runners: load_runner_inventory(repo_root)?,
        profiles: load_profile_inventory(repo_root)?,
    })
}

fn load_runner_inventory(
    repo_root: &Path,
) -> Result<BTreeSet<String>, RouteStateInventoryLoadError> {
    let workspace = CompilerWorkspace::new(repo_root);
    let inventory_root = inventory_root_path(&workspace, runner_root());
    let inventory_dir = match workspace.trusted_directory(&inventory_root) {
        Ok(directory) => directory,
        Err(RepoRelativeDirectoryAccessError::Missing(_)) => return Ok(BTreeSet::new()),
        Err(err) => return Err(map_inventory_directory_error(err)),
    };
    let entries = read_inventory_dir_entries(&inventory_dir)?;

    let mut ids = BTreeSet::new();
    for entry in entries {
        let entry_path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|source| RouteStateInventoryLoadError {
                path: entry_path.clone(),
                source,
            })?;
        if !file_type.is_file() {
            continue;
        }

        let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        let Some(id) = name.strip_suffix(".md") else {
            continue;
        };
        if !is_inventory_id(id) {
            continue;
        }

        ids.insert(id.to_string());
    }

    Ok(ids)
}

fn load_profile_inventory(
    repo_root: &Path,
) -> Result<ProfilePackInventory, RouteStateInventoryLoadError> {
    let workspace = CompilerWorkspace::new(repo_root);
    let inventory_root = inventory_root_path(&workspace, profile_root());
    let inventory_dir = match workspace.trusted_directory(&inventory_root) {
        Ok(directory) => directory,
        Err(RepoRelativeDirectoryAccessError::Missing(_)) => {
            return Ok(ProfilePackInventory::default());
        }
        Err(err) => return Err(map_inventory_directory_error(err)),
    };
    let entries = read_inventory_dir_entries(&inventory_dir)?;

    let mut profiles = ProfilePackInventory::default();
    for entry in entries {
        let entry_path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|source| RouteStateInventoryLoadError {
                path: entry_path.clone(),
                source,
            })?;
        if !file_type.is_dir() {
            continue;
        }

        let Some(id) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        if !is_inventory_id(&id) {
            continue;
        }

        let profile_dir = inventory_entry_path(&workspace, &inventory_root, &id);
        let profile_yaml = inventory_entry_path(&workspace, &profile_dir, "profile.yaml");
        let metadata = match workspace.metadata_no_follow(&profile_yaml) {
            Ok(Some(metadata)) => metadata,
            Ok(None) => continue,
            Err(err) => return Err(map_inventory_metadata_error(err)),
        };
        if !metadata.is_file() {
            continue;
        }

        let missing_files = missing_profile_pack_files(&workspace, &profile_dir)?;
        if missing_files.is_empty() {
            profiles.complete_ids.insert(id);
        } else {
            profiles.incomplete_ids.insert(id, missing_files);
        }
    }

    Ok(profiles)
}

fn missing_profile_pack_files(
    workspace: &CompilerWorkspace<'_>,
    profile_dir: &NormalizedRepoRelativePath,
) -> Result<Vec<&'static str>, RouteStateInventoryLoadError> {
    let mut missing_files = Vec::new();
    for required_file in PROFILE_PACK_REQUIRED_FILES {
        let required_path = inventory_entry_path(workspace, profile_dir, required_file);
        let metadata = match workspace.metadata_no_follow(&required_path) {
            Ok(Some(metadata)) => metadata,
            Ok(None) => {
                missing_files.push(required_file);
                continue;
            }
            Err(err) => return Err(map_inventory_metadata_error(err)),
        };
        if !metadata.is_file() {
            missing_files.push(required_file);
        }
    }

    Ok(missing_files)
}

fn is_inventory_id(value: &str) -> bool {
    let Some(first) = value.chars().next() else {
        return false;
    };
    if !first.is_ascii_lowercase() && !first.is_ascii_digit() {
        return false;
    }

    value
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '_')
}

fn normalized_state_for_route_basis(state: &RouteState, repo_root: &Path) -> RouteState {
    let mut normalized = state.clone();
    normalized.schema_version = ROUTE_STATE_SCHEMA_VERSION.to_string();
    let _ = repo_root;
    normalized.run.repo_root = Some(derived_route_basis_repo_root());
    normalized
}

pub fn normalized_state_for_persistence(state: &RouteState, repo_root: &Path) -> RouteState {
    let mut normalized = state.clone();
    normalized.schema_version = ROUTE_STATE_SCHEMA_VERSION.to_string();
    normalized.run.repo_root = Some(derived_repo_root(repo_root));
    normalized
}

pub fn effective_route_basis_run(
    repo_root: impl AsRef<Path>,
    pipeline: &PipelineDefinition,
    state: &RouteState,
) -> RouteStateRun {
    let repo_root = repo_root.as_ref();
    let mut run = normalized_state_for_route_basis(state, repo_root).run;
    if run.runner.is_none() {
        run.runner = Some(pipeline.body.defaults.runner.clone());
    }
    if run.profile.is_none() {
        run.profile = Some(pipeline.body.defaults.profile.clone());
    }
    run
}

fn fingerprint_repo_relative_file(
    repo_root: &Path,
    relative_path: &str,
) -> Result<String, RouteBasisBuildError> {
    sha256_repo_relative_file(repo_root, relative_path).map_err(|err| match err {
        RepoRelativeFileAccessError::InvalidPath(reason) => RouteBasisBuildError::InvalidPath {
            path: relative_path.to_string(),
            reason,
        },
        RepoRelativeFileAccessError::Missing(path) => RouteBasisBuildError::ReadFailure {
            path,
            source: std::io::Error::from(std::io::ErrorKind::NotFound),
        },
        RepoRelativeFileAccessError::SymlinkNotAllowed(path)
        | RepoRelativeFileAccessError::NotRegularFile(path) => RouteBasisBuildError::ReadFailure {
            path: path.clone(),
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "repo-relative file is not a regular non-symlink file: {}",
                    path.display()
                ),
            ),
        },
        RepoRelativeFileAccessError::ReadFailure { path, source } => {
            RouteBasisBuildError::ReadFailure { path, source }
        }
    })
}

fn trim_audit_history(audit: &mut Vec<RouteStateAuditEntry>) {
    if audit.len() <= ROUTE_STATE_AUDIT_LIMIT {
        return;
    }

    let overflow = audit.len() - ROUTE_STATE_AUDIT_LIMIT;
    audit.drain(0..overflow);
}

pub fn route_state_path(repo_root: &Path, pipeline_id: &str) -> Result<PathBuf, &'static str> {
    validate_pipeline_id(pipeline_id)?;
    let route_state_relative_path = RepoLayoutRoot::new(repo_root)
        .runtime_state()
        .route_state_relative_path(pipeline_id);
    Ok(repo_root.join(route_state_relative_path.as_str()))
}

pub(crate) fn route_state_path_with_storage_layout(
    repo_root: &Path,
    pipeline_id: &str,
    storage_layout: PipelineStorageLayoutContract,
) -> Result<PathBuf, &'static str> {
    validate_pipeline_id(pipeline_id)?;
    let route_state_relative_path = RepoLayoutRoot::with_contract(repo_root, storage_layout)
        .runtime_state()
        .route_state_relative_path(pipeline_id);
    Ok(repo_root.join(route_state_relative_path.as_str()))
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn preview_runtime_state_reset(repo_root: &Path) -> Result<Vec<String>, String> {
    plan_runtime_state_reset(repo_root).map(|plan| plan.paths)
}

#[allow(dead_code)]
pub(crate) fn preview_runtime_state_reset_with_storage_layout(
    repo_root: &Path,
    storage_layout: PipelineStorageLayoutContract,
) -> Result<Vec<String>, String> {
    plan_runtime_state_reset_with_storage_layout(repo_root, storage_layout).map(|plan| plan.paths)
}

#[cfg_attr(not(test), allow(dead_code))]
pub(crate) fn reset_runtime_state_tree(repo_root: &Path) -> Result<Vec<String>, String> {
    let plan = plan_runtime_state_reset(repo_root)?;
    let reset_paths = plan.paths.clone();
    apply_runtime_state_reset(&plan)?;
    Ok(reset_paths)
}

#[allow(dead_code)]
pub(crate) fn reset_runtime_state_tree_with_storage_layout(
    repo_root: &Path,
    storage_layout: PipelineStorageLayoutContract,
) -> Result<Vec<String>, String> {
    let plan = plan_runtime_state_reset_with_storage_layout(repo_root, storage_layout)?;
    let reset_paths = plan.paths.clone();
    apply_runtime_state_reset(&plan)?;
    Ok(reset_paths)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuntimeStateResetPlan {
    entries: Vec<RuntimeStateResetEntry>,
    paths: Vec<String>,
}

impl RuntimeStateResetPlan {
    #[allow(dead_code)]
    pub(crate) fn paths(&self) -> &[String] {
        &self.paths
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuntimeStateResetEntry {
    path: PathBuf,
    display_path: String,
    kind: RuntimeStateResetEntryKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuntimeStateResetEntryKind {
    File,
    Dir,
}

pub(crate) fn plan_runtime_state_reset(repo_root: &Path) -> Result<RuntimeStateResetPlan, String> {
    let workspace = CompilerWorkspace::new(repo_root);
    let runtime_state = RepoLayoutRoot::new(repo_root).runtime_state();
    let state_root = runtime_state.state_root();
    let state_root_relative = runtime_state.state_root_relative();
    let state_root_dir = match workspace.trusted_directory(&state_root) {
        Ok(directory) => directory,
        Err(RepoRelativeDirectoryAccessError::Missing(_)) => {
            return Ok(RuntimeStateResetPlan {
                entries: Vec::new(),
                paths: Vec::new(),
            });
        }
        Err(RepoRelativeDirectoryAccessError::SymlinkNotAllowed(path)) => {
            return Err(format!(
                "runtime state root `{state_root_relative}` cannot be reset through symlink {}",
                path.display()
            ));
        }
        Err(RepoRelativeDirectoryAccessError::NotDirectory(path)) => {
            return Err(format!(
                "runtime state root `{state_root_relative}` is not a directory at {}",
                path.display()
            ));
        }
        Err(RepoRelativeDirectoryAccessError::ReadFailure { path, source }) => {
            return Err(format!(
                "failed to inspect runtime state root `{state_root_relative}` at {}: {source}",
                path.display()
            ));
        }
    };
    let mut entries = fs::read_dir(state_root_dir.absolute_path())
        .map_err(|source| {
            format!(
                "failed to read runtime state root `{state_root_relative}` at {}: {source}",
                state_root_dir.absolute_path().display()
            )
        })?
        .map(|entry| {
            entry.and_then(|entry| {
                let relative_path = Path::new(state_root.as_str()).join(entry.file_name());
                workspace
                    .normalize_repo_relative_path(&relative_path)
                    .map_err(std::io::Error::other)
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|source| {
            format!(
                "failed to enumerate runtime state root `{state_root_relative}` at {}: {source}",
                state_root_dir.absolute_path().display()
            )
        })?;
    entries.sort_by(|left, right| left.as_str().cmp(right.as_str()));

    let mut reset_entries = Vec::new();
    for entry in entries {
        collect_runtime_state_reset_entry(&workspace, &entry, &mut reset_entries)?;
    }

    let mut reset_paths = reset_entries
        .iter()
        .map(|entry| entry.display_path.clone())
        .collect::<Vec<_>>();
    reset_paths.sort();
    Ok(RuntimeStateResetPlan {
        entries: reset_entries,
        paths: reset_paths,
    })
}

#[allow(dead_code)]
pub(crate) fn plan_runtime_state_reset_with_storage_layout(
    repo_root: &Path,
    storage_layout: PipelineStorageLayoutContract,
) -> Result<RuntimeStateResetPlan, String> {
    let workspace = CompilerWorkspace::new(repo_root);
    let runtime_state = RepoLayoutRoot::with_contract(repo_root, storage_layout).runtime_state();
    let state_root = runtime_state.state_root();
    let state_root_relative = runtime_state.state_root_relative();
    let state_root_dir = match workspace.trusted_directory(&state_root) {
        Ok(directory) => directory,
        Err(RepoRelativeDirectoryAccessError::Missing(_)) => {
            return Ok(RuntimeStateResetPlan {
                entries: Vec::new(),
                paths: Vec::new(),
            });
        }
        Err(RepoRelativeDirectoryAccessError::SymlinkNotAllowed(path)) => {
            return Err(format!(
                "runtime state root `{state_root_relative}` cannot be reset through symlink {}",
                path.display()
            ));
        }
        Err(RepoRelativeDirectoryAccessError::NotDirectory(path)) => {
            return Err(format!(
                "runtime state root `{state_root_relative}` is not a directory at {}",
                path.display()
            ));
        }
        Err(RepoRelativeDirectoryAccessError::ReadFailure { path, source }) => {
            return Err(format!(
                "failed to inspect runtime state root `{state_root_relative}` at {}: {source}",
                path.display()
            ));
        }
    };
    let mut entries = fs::read_dir(state_root_dir.absolute_path())
        .map_err(|source| {
            format!(
                "failed to read runtime state root `{state_root_relative}` at {}: {source}",
                state_root_dir.absolute_path().display()
            )
        })?
        .map(|entry| {
            entry.and_then(|entry| {
                let relative_path = Path::new(state_root.as_str()).join(entry.file_name());
                workspace
                    .normalize_repo_relative_path(&relative_path)
                    .map_err(std::io::Error::other)
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|source| {
            format!(
                "failed to enumerate runtime state root `{state_root_relative}` at {}: {source}",
                state_root_dir.absolute_path().display()
            )
        })?;
    entries.sort_by(|left, right| left.as_str().cmp(right.as_str()));

    let mut reset_entries = Vec::new();
    for entry in entries {
        collect_runtime_state_reset_entry(&workspace, &entry, &mut reset_entries)?;
    }

    let mut reset_paths = reset_entries
        .iter()
        .map(|entry| entry.display_path.clone())
        .collect::<Vec<_>>();
    reset_paths.sort();
    Ok(RuntimeStateResetPlan {
        entries: reset_entries,
        paths: reset_paths,
    })
}

pub(crate) fn apply_runtime_state_reset(plan: &RuntimeStateResetPlan) -> Result<(), String> {
    for entry in &plan.entries {
        match entry.kind {
            RuntimeStateResetEntryKind::File => {
                fs::remove_file(&entry.path).map_err(|source| {
                    format!(
                        "failed to remove runtime state file `{}` at {}: {source}",
                        entry.display_path,
                        entry.path.display()
                    )
                })?;
            }
            RuntimeStateResetEntryKind::Dir => {
                fs::remove_dir(&entry.path).map_err(|source| {
                    format!(
                        "failed to remove runtime state directory `{}` at {}: {source}",
                        entry.display_path,
                        entry.path.display()
                    )
                })?;
            }
        }
    }

    Ok(())
}

fn collect_runtime_state_reset_entry(
    workspace: &CompilerWorkspace<'_>,
    path: &NormalizedRepoRelativePath,
    reset_entries: &mut Vec<RuntimeStateResetEntry>,
) -> Result<(), String> {
    if let Ok(directory) = workspace.trusted_directory(path) {
        let mut children = fs::read_dir(directory.absolute_path())
            .map_err(|source| {
                format!(
                    "failed to read runtime state directory `{}` at {}: {source}",
                    path.as_str(),
                    directory.absolute_path().display()
                )
            })?
            .map(|entry| {
                entry.and_then(|entry| {
                    let relative_path = Path::new(path.as_str()).join(entry.file_name());
                    workspace
                        .normalize_repo_relative_path(&relative_path)
                        .map_err(std::io::Error::other)
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|source| {
                format!(
                    "failed to enumerate runtime state directory `{}` at {}: {source}",
                    path.as_str(),
                    directory.absolute_path().display()
                )
            })?;
        children.sort_by(|left, right| left.as_str().cmp(right.as_str()));

        for child in children {
            collect_runtime_state_reset_entry(workspace, &child, reset_entries)?;
        }

        reset_entries.push(RuntimeStateResetEntry {
            path: directory.absolute_path().to_path_buf(),
            display_path: path.as_str().to_string(),
            kind: RuntimeStateResetEntryKind::Dir,
        });
        return Ok(());
    }

    match workspace.trusted_read(path) {
        Ok(file) => {
            reset_entries.push(RuntimeStateResetEntry {
                path: file.absolute_path().to_path_buf(),
                display_path: path.as_str().to_string(),
                kind: RuntimeStateResetEntryKind::File,
            });
            Ok(())
        }
        Err(RepoRelativeFileAccessError::SymlinkNotAllowed(path_buf)) => Err(format!(
            "runtime state path `{}` cannot be reset through symlink {}",
            path.as_str(),
            path_buf.display()
        )),
        Err(RepoRelativeFileAccessError::ReadFailure {
            path: path_buf,
            source,
        }) => Err(format!(
            "failed to inspect runtime state path `{}` at {}: {source}",
            path.as_str(),
            path_buf.display()
        )),
        Err(RepoRelativeFileAccessError::NotRegularFile(_))
        | Err(RepoRelativeFileAccessError::Missing(_))
        | Err(RepoRelativeFileAccessError::InvalidPath(_)) => Err(format!(
            "runtime state path `{}` is not a regular file or directory",
            path.as_str()
        )),
    }
}

fn inventory_root_path(
    workspace: &CompilerWorkspace<'_>,
    root: &Path,
) -> NormalizedRepoRelativePath {
    workspace
        .normalize_repo_relative_path(root)
        .expect("route-state inventory roots must stay valid repo-relative paths")
}

fn inventory_entry_path(
    workspace: &CompilerWorkspace<'_>,
    parent: &NormalizedRepoRelativePath,
    child: &str,
) -> NormalizedRepoRelativePath {
    let path = Path::new(parent.as_str()).join(child);
    workspace
        .normalize_repo_relative_path(&path)
        .expect("route-state inventory entries must stay valid repo-relative paths")
}

fn read_inventory_dir_entries(
    directory: &TrustedRepoDirectory,
) -> Result<Vec<fs::DirEntry>, RouteStateInventoryLoadError> {
    fs::read_dir(directory.absolute_path())
        .map_err(|source| RouteStateInventoryLoadError {
            path: directory.absolute_path().to_path_buf(),
            source,
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|source| RouteStateInventoryLoadError {
            path: directory.absolute_path().to_path_buf(),
            source,
        })
}

fn map_inventory_directory_error(
    err: RepoRelativeDirectoryAccessError,
) -> RouteStateInventoryLoadError {
    match err {
        RepoRelativeDirectoryAccessError::Missing(path) => RouteStateInventoryLoadError {
            path,
            source: std::io::Error::from(std::io::ErrorKind::NotFound),
        },
        RepoRelativeDirectoryAccessError::SymlinkNotAllowed(path) => RouteStateInventoryLoadError {
            path: path.clone(),
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "repo-relative directory cannot be traversed through symlink {}",
                    path.display()
                ),
            ),
        },
        RepoRelativeDirectoryAccessError::NotDirectory(path) => RouteStateInventoryLoadError {
            path: path.clone(),
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "repo-relative directory is not a directory at {}",
                    path.display()
                ),
            ),
        },
        RepoRelativeDirectoryAccessError::ReadFailure { path, source } => {
            RouteStateInventoryLoadError { path, source }
        }
    }
}

fn map_inventory_metadata_error(
    err: RepoRelativeMetadataReadError,
) -> RouteStateInventoryLoadError {
    RouteStateInventoryLoadError {
        path: err.path,
        source: err.source,
    }
}

pub fn ensure_state_parent_dir(state_path: &Path) -> Result<(), std::io::Error> {
    if let Some(parent) = state_path.parent() {
        fs::create_dir_all(parent)?;
    }

    Ok(())
}

pub fn persist_route_state(
    state_path: &Path,
    state: &RouteState,
) -> Result<(), RouteStateStoreError> {
    let serialized = serde_yaml_bw::to_string(state).map_err(|source| {
        RouteStateStoreError::SerializationFailure {
            path: state_path.to_path_buf(),
            reason: source.to_string(),
        }
    })?;

    let temp_path = temp_route_state_path(state_path);
    let result = (|| -> Result<(), RouteStateStoreError> {
        let mut file = open_new_temp_file(&temp_path)?;
        file.write_all(serialized.as_bytes()).map_err(|source| {
            RouteStateStoreError::WriteFailure {
                path: temp_path.clone(),
                source,
            }
        })?;
        file.sync_all()
            .map_err(|source| RouteStateStoreError::WriteFailure {
                path: temp_path.clone(),
                source,
            })?;
        drop(file);
        fs::rename(&temp_path, state_path).map_err(|source| {
            RouteStateStoreError::WriteFailure {
                path: state_path.to_path_buf(),
                source,
            }
        })?;
        sync_parent_dir(state_path)?;
        Ok(())
    })();

    if result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }

    result
}

pub fn open_new_temp_file(path: &Path) -> Result<File, RouteStateStoreError> {
    let mut options = OpenOptions::new();
    options.create_new(true).write(true).read(true);

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;

        options.mode(0o600);
    }

    options
        .open(path)
        .map_err(|source| RouteStateStoreError::WriteFailure {
            path: path.to_path_buf(),
            source,
        })
}

pub fn sync_parent_dir(path: &Path) -> Result<(), RouteStateStoreError> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };

    let dir = File::open(parent).map_err(|source| RouteStateStoreError::WriteFailure {
        path: parent.to_path_buf(),
        source,
    })?;
    dir.sync_all()
        .map_err(|source| RouteStateStoreError::WriteFailure {
            path: parent.to_path_buf(),
            source,
        })?;
    Ok(())
}

pub fn temp_route_state_path(state_path: &Path) -> PathBuf {
    let counter = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let file_name = state_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("state.yaml");
    state_path.with_file_name(format!("{file_name}.tmp-{pid}-{counter}"))
}

fn read_file_no_follow(path: &Path) -> Result<String, std::io::Error> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;

        let mut file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    #[cfg(not(unix))]
    {
        fs::read_to_string(path)
    }
}

fn validate_pipeline_id(pipeline_id: &str) -> Result<(), &'static str> {
    if pipeline_id.trim().is_empty() {
        return Err("pipeline id must not be empty");
    }

    let mut components = Path::new(pipeline_id).components();
    match (components.next(), components.next()) {
        (Some(Component::Normal(_)), None) => Ok(()),
        (Some(Component::CurDir), None) => {
            Err("pipeline id must be a single repo-relative path component")
        }
        _ => Err("pipeline id must be a single repo-relative path component"),
    }
}

pub fn acquire_advisory_lock(
    state_path: &Path,
) -> Result<RouteStateLockGuard, RouteStateStoreError> {
    let lock_path = state_path.with_extension("lock");
    if let Some(parent) = lock_path.parent() {
        fs::create_dir_all(parent).map_err(|source| RouteStateStoreError::LockFailure {
            path: lock_path.clone(),
            source,
        })?;
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .map_err(|source| RouteStateStoreError::LockFailure {
            path: lock_path.clone(),
            source,
        })?;

    #[cfg(unix)]
    {
        lock_file(&file, libc::LOCK_EX).map_err(|source| RouteStateStoreError::LockFailure {
            path: lock_path.clone(),
            source,
        })?;
    }

    Ok(RouteStateLockGuard { file, lock_path })
}

#[cfg(unix)]
fn lock_file(file: &File, operation: libc::c_int) -> Result<(), std::io::Error> {
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
fn lock_file(_file: &File, _operation: libc::c_int) -> Result<(), std::io::Error> {
    Ok(())
}

pub struct RouteStateLockGuard {
    file: File,
    lock_path: PathBuf,
}

impl From<RouteStageStatus> for RouteBasisStageStatus {
    fn from(value: RouteStageStatus) -> Self {
        match value {
            RouteStageStatus::Active => Self::Active,
            RouteStageStatus::Skipped => Self::Skipped,
            RouteStageStatus::Blocked => Self::Blocked,
            RouteStageStatus::Next => Self::Next,
        }
    }
}

impl From<RouteStageReason> for RouteBasisStageReason {
    fn from(value: RouteStageReason) -> Self {
        match value {
            RouteStageReason::SkippedActivationFalse {
                operator,
                unsatisfied_variables,
            } => Self::SkippedActivationFalse {
                operator: RouteBasisActivationOperator::from(operator),
                unsatisfied_variables,
            },
            RouteStageReason::NextMissingRouteVariables {
                operator,
                missing_variables,
            } => Self::NextMissingRouteVariables {
                operator: RouteBasisActivationOperator::from(operator),
                missing_variables,
            },
            RouteStageReason::BlockedByUnresolvedStage {
                upstream_stage_id,
                upstream_status,
            } => Self::BlockedByUnresolvedStage {
                upstream_stage_id,
                upstream_status: RouteBasisStageStatus::from(upstream_status),
            },
        }
    }
}

impl From<crate::pipeline::ActivationOperator> for RouteBasisActivationOperator {
    fn from(value: crate::pipeline::ActivationOperator) -> Self {
        match value {
            crate::pipeline::ActivationOperator::Any => Self::Any,
            crate::pipeline::ActivationOperator::All => Self::All,
        }
    }
}

impl Drop for RouteStateLockGuard {
    fn drop(&mut self) {
        #[cfg(unix)]
        let _ = lock_file(&self.file, libc::LOCK_UN);
        let _ = &self.lock_path;
    }
}

impl fmt::Display for RouteStateValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteStateValue::Bool(value) => write!(f, "{value}"),
            RouteStateValue::String(value) => write!(f, "{value}"),
        }
    }
}

impl fmt::Display for RouteStateReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteStateReadError::InvalidPipelineId {
                pipeline_id,
                reason,
            } => {
                write!(
                    f,
                    "route state pipeline id `{pipeline_id}` is invalid: {reason}"
                )
            }
            RouteStateReadError::ReadFailure { path, source } => {
                write!(
                    f,
                    "failed to read route state at {}: {source}",
                    path.display()
                )
            }
            RouteStateReadError::MalformedState { path, reason } => {
                write!(f, "malformed route state at {}: {reason}", path.display())
            }
        }
    }
}

impl std::error::Error for RouteStateReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RouteStateReadError::ReadFailure { source, .. } => Some(source),
            RouteStateReadError::InvalidPipelineId { .. }
            | RouteStateReadError::MalformedState { .. } => None,
        }
    }
}

impl fmt::Display for RouteStateStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteStateStoreError::InvalidPipelineId {
                pipeline_id,
                reason,
            } => {
                write!(
                    f,
                    "route state pipeline id `{pipeline_id}` is invalid: {reason}"
                )
            }
            RouteStateStoreError::InvalidSupportedVariables { reason } => {
                write!(f, "supported route-state variables are invalid: {reason}")
            }
            RouteStateStoreError::InvalidMutation { reason } => {
                write!(f, "route state mutation is invalid: {reason}")
            }
            RouteStateStoreError::ReadFailure { path, source } => {
                write!(
                    f,
                    "failed to read route state at {}: {source}",
                    path.display()
                )
            }
            RouteStateStoreError::LockFailure { path, source } => {
                write!(
                    f,
                    "failed to acquire route state lock at {}: {source}",
                    path.display()
                )
            }
            RouteStateStoreError::WriteFailure { path, source } => {
                write!(
                    f,
                    "failed to write route state at {}: {source}",
                    path.display()
                )
            }
            RouteStateStoreError::SerializationFailure { path, reason } => {
                write!(
                    f,
                    "failed to serialize route state for {}: {reason}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for RouteStateStoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RouteStateStoreError::ReadFailure { source, .. }
            | RouteStateStoreError::LockFailure { source, .. }
            | RouteStateStoreError::WriteFailure { source, .. } => Some(source),
            RouteStateStoreError::InvalidPipelineId { .. }
            | RouteStateStoreError::InvalidSupportedVariables { .. }
            | RouteStateStoreError::InvalidMutation { .. }
            | RouteStateStoreError::SerializationFailure { .. } => None,
        }
    }
}

impl fmt::Display for RouteBasisBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteBasisBuildError::MissingSelectedRunner { pipeline_id } => write!(
                f,
                "cannot build route_basis for `{pipeline_id}` without a selected runner"
            ),
            RouteBasisBuildError::MissingSelectedProfile { pipeline_id } => write!(
                f,
                "cannot build route_basis for `{pipeline_id}` without a selected profile"
            ),
            RouteBasisBuildError::IncompleteSelectedProfilePack {
                pipeline_id,
                profile_id,
                missing_files,
            } => write!(
                f,
                "cannot build route_basis for `{pipeline_id}` because {}",
                format_selected_profile_pack_incomplete_reason(profile_id, missing_files)
            ),
            RouteBasisBuildError::InvalidRouteSnapshot {
                pipeline_id,
                stage_id,
                detail,
            } => write!(
                f,
                "cannot build route_basis for `{pipeline_id}` because stage `{stage_id}` is invalid: {detail}"
            ),
            RouteBasisBuildError::ReadFailure { path, source } => write!(
                f,
                "failed to read route_basis input {}: {source}",
                path.display()
            ),
            RouteBasisBuildError::InvalidPath { path, reason } => {
                write!(f, "route_basis path `{path}` is invalid: {reason}")
            }
        }
    }
}

impl std::error::Error for RouteBasisBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RouteBasisBuildError::ReadFailure { source, .. } => Some(source),
            RouteBasisBuildError::MissingSelectedRunner { .. }
            | RouteBasisBuildError::MissingSelectedProfile { .. }
            | RouteBasisBuildError::IncompleteSelectedProfilePack { .. }
            | RouteBasisBuildError::InvalidRouteSnapshot { .. }
            | RouteBasisBuildError::InvalidPath { .. } => None,
        }
    }
}

impl fmt::Display for RouteStateMutationOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteStateMutationOutcome::Applied(state) => {
                write!(f, "applied route state revision {}", state.revision)
            }
            RouteStateMutationOutcome::Refused(refusal) => write!(f, "{refusal}"),
        }
    }
}

impl fmt::Display for RouteStateMutationRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteStateMutationRefusal::MalformedState { reason } => {
                write!(f, "malformed route state: {reason}")
            }
            RouteStateMutationRefusal::UnsupportedVariable { variable } => {
                write!(f, "unsupported route-state variable `{variable}`")
            }
            RouteStateMutationRefusal::RevisionConflict {
                expected_revision,
                actual_revision,
            } => write!(
                f,
                "revision conflict: expected {expected_revision}, found {actual_revision}"
            ),
        }
    }
}

impl fmt::Display for RouteBasisPersistOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteBasisPersistOutcome::Applied(state) => {
                write!(
                    f,
                    "persisted route_basis at route state revision {}",
                    state.revision
                )
            }
            RouteBasisPersistOutcome::Refused(refusal) => write!(f, "{refusal}"),
        }
    }
}

impl fmt::Display for RouteBasisPersistRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteBasisPersistRefusal::MalformedState { reason } => {
                write!(f, "malformed route state: {reason}")
            }
            RouteBasisPersistRefusal::RevisionConflict {
                expected_revision,
                actual_revision,
            } => write!(
                f,
                "revision conflict while persisting route_basis: expected {expected_revision}, found {actual_revision}"
            ),
        }
    }
}

impl fmt::Display for TrustedPipelineSessionRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TrustedPipelineSessionRefusal::InvalidPipelineId {
                pipeline_id,
                reason,
            } => {
                write!(
                    f,
                    "trusted pipeline session pipeline id `{pipeline_id}` is invalid: {reason}"
                )
            }
            TrustedPipelineSessionRefusal::ReadFailure { path, reason } => {
                write!(
                    f,
                    "failed to read trusted pipeline session state at {}: {reason}",
                    path.display()
                )
            }
            TrustedPipelineSessionRefusal::InvalidState { path, reason } => {
                write!(
                    f,
                    "trusted pipeline session state at {} is invalid: {reason}",
                    path.display()
                )
            }
            TrustedPipelineSessionRefusal::MissingRouteBasis { pipeline_id } => {
                write!(
                    f,
                    "persisted route_basis is missing for trusted pipeline `{pipeline_id}`"
                )
            }
            TrustedPipelineSessionRefusal::StaleRouteBasis {
                pipeline_id,
                reason,
            } => write!(
                f,
                "persisted route_basis for trusted pipeline `{pipeline_id}` is stale: {reason}"
            ),
            TrustedPipelineSessionRefusal::MalformedRouteBasis {
                pipeline_id,
                reason,
            } => write!(
                f,
                "persisted route_basis for trusted pipeline `{pipeline_id}` is malformed: {reason}"
            ),
            TrustedPipelineSessionRefusal::MissingStage {
                pipeline_id,
                stage_id,
            } => write!(
                f,
                "trusted pipeline `{pipeline_id}` route_basis does not include stage `{stage_id}`"
            ),
            TrustedPipelineSessionRefusal::InactiveStage {
                pipeline_id,
                stage_id,
                status,
                ..
            } => write!(
                f,
                "trusted pipeline `{pipeline_id}` stage `{stage_id}` is not active (current status: {})",
                route_basis_stage_status_label(*status)
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{preview_runtime_state_reset, reset_runtime_state_tree};
    use std::fs;
    use std::path::Path;

    fn write_file(path: &Path, contents: &[u8]) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create parent dirs");
        }
        fs::write(path, contents).expect("write file");
    }

    #[cfg(unix)]
    #[test]
    fn route_state_reset_refuses_symlink_without_partial_deletion() {
        use std::os::unix::fs::symlink;

        let dir = tempfile::tempdir().expect("tempdir");
        let repo_root = dir.path();

        write_file(&repo_root.join(".handbook/state/a.yaml"), b"a: 1\n");
        let external = tempfile::tempdir().expect("external tempdir");
        symlink(external.path(), repo_root.join(".handbook/state/z_symlink")).expect("symlink");

        let err = reset_runtime_state_tree(repo_root).expect_err("reset should refuse");
        assert!(err.contains("symlink"), "{err}");
        assert!(
            repo_root.join(".handbook/state/a.yaml").is_file(),
            "preflight refusal must not partially delete state"
        );
    }

    #[test]
    fn route_state_preview_matches_reset_paths_for_valid_tree() {
        let preview_root = tempfile::tempdir().expect("preview tempdir");
        write_file(
            &preview_root
                .path()
                .join(".handbook/state/pipeline/pipeline.foundation_inputs.yaml"),
            b"pipeline state\n",
        );
        write_file(
            &preview_root
                .path()
                .join(".handbook/state/pipeline/capture/cache.yaml"),
            b"cache state\n",
        );

        let preview_paths =
            preview_runtime_state_reset(preview_root.path()).expect("preview should succeed");

        let apply_root = tempfile::tempdir().expect("apply tempdir");
        write_file(
            &apply_root
                .path()
                .join(".handbook/state/pipeline/pipeline.foundation_inputs.yaml"),
            b"pipeline state\n",
        );
        write_file(
            &apply_root
                .path()
                .join(".handbook/state/pipeline/capture/cache.yaml"),
            b"cache state\n",
        );

        let reset_paths =
            reset_runtime_state_tree(apply_root.path()).expect("reset should succeed");

        assert_eq!(preview_paths, reset_paths);
        assert!(!apply_root
            .path()
            .join(".handbook/state/pipeline/pipeline.foundation_inputs.yaml")
            .exists());
        assert!(!apply_root
            .path()
            .join(".handbook/state/pipeline/capture/cache.yaml")
            .exists());
    }
}
