//! Typed, transport-free composition for the fixed Pipeline CLI route.
//!
//! Pipeline parsing, storage, and mutation remain owned by `handbook_pipeline`.
//! This module maps those owner values into closed operation outcomes so clients
//! do not compose the pipeline owner directly. Rendering intentionally remains
//! in the client layer.

use std::{collections::BTreeMap, path::PathBuf};

use handbook_pipeline::{
    pipeline::{
        load_pipeline_catalog_metadata, load_pipeline_selection_metadata,
        load_selected_pipeline_definition, supported_route_state_variables, PipelineLookupError,
        PipelineMetadataSelectionError, PipelineSelection, SelectedPipelineLoadError,
    },
    pipeline_capture::{
        apply_pipeline_capture as owner_apply_pipeline_capture,
        capture_pipeline_output as owner_capture_pipeline_output,
        preview_pipeline_capture as owner_preview_pipeline_capture, PipelineCaptureRefusal,
        PipelineCaptureRefusalClassification, PipelineCaptureRequest,
    },
    pipeline_compile::{
        compile_pipeline_stage, PipelineCompileDocumentKind, PipelineCompileDocumentStatus,
        PipelineCompileOutputKind, PipelineCompileRefusal, PipelineCompileRefusalClassification,
    },
    pipeline_handoff::{
        emit_pipeline_handoff_bundle, PipelineHandoffEmitRequest, PipelineHandoffRefusal,
        PipelineHandoffRefusalClassification,
    },
    pipeline_route::{resolve_pipeline_route, RouteStageReason, RouteStageStatus, RouteVariables},
    route_state::{
        build_route_basis, effective_route_basis_run, load_route_state_with_supported_variables,
        persist_route_basis, set_route_state, RouteBasisPersistOutcome, RouteBasisStageReason,
        RouteBasisStageStatus, RouteState, RouteStateMutation, RouteStateMutationOutcome,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCatalogRequest {
    pub repository_root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCatalogOutcome {
    pub pipelines: Vec<PipelineCatalogEntry>,
    pub pipeline_count: usize,
    pub stage_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCatalogEntry {
    pub id: String,
    pub title: String,
    pub source_path: PathBuf,
    pub stage_count: usize,
}

pub fn list_pipeline_catalog(
    request: &PipelineCatalogRequest,
) -> Result<PipelineCatalogOutcome, String> {
    let catalog = load_pipeline_catalog_metadata(&request.repository_root)
        .map_err(|error| error.to_string())?;
    Ok(PipelineCatalogOutcome {
        pipeline_count: catalog.pipeline_count(),
        stage_count: catalog.stage_count(),
        pipelines: catalog
            .pipelines()
            .map(|pipeline| PipelineCatalogEntry {
                id: pipeline.definition.header.id.clone(),
                title: pipeline.definition.header.title.clone(),
                source_path: pipeline.definition.source_path.clone(),
                stage_count: pipeline.stages.len(),
            })
            .collect(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineShowRequest {
    pub repository_root: PathBuf,
    pub selector: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineShowOutcome {
    Pipeline(PipelineDefinitionView),
    Stage(PipelineStageView),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineDefinitionView {
    pub id: String,
    pub title: String,
    pub description: String,
    pub source_path: PathBuf,
    pub default_runner: String,
    pub default_profile: String,
    pub default_enable_complexity: bool,
    pub stages: Vec<PipelineDeclaredStageView>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineDeclaredStageView {
    pub id: String,
    pub title: String,
    pub source_path: PathBuf,
    pub work_level: Option<String>,
    pub sets: Option<Vec<String>>,
    pub activation: Option<PipelineActivationView>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineActivationView {
    pub operator: PipelineActivationOperator,
    pub clauses: Vec<PipelineActivationClause>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineActivationOperator {
    Any,
    All,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineActivationClause {
    pub variable: String,
    pub value: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineStageView {
    pub id: String,
    pub kind: String,
    pub version: String,
    pub title: String,
    pub description: String,
    pub work_level: Option<String>,
    pub source_path: PathBuf,
    pub pipeline_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineShowFailure {
    Catalog(String),
    Selector(PipelineSelectorRefusal),
}

pub fn show_pipeline(
    request: &PipelineShowRequest,
) -> Result<PipelineShowOutcome, PipelineShowFailure> {
    match load_pipeline_selection_metadata(&request.repository_root, &request.selector) {
        Ok(PipelineSelection::Pipeline(pipeline)) => {
            Ok(PipelineShowOutcome::Pipeline(PipelineDefinitionView {
                id: pipeline.definition.header.id.clone(),
                title: pipeline.definition.header.title.clone(),
                description: pipeline.definition.header.description.clone(),
                source_path: pipeline.definition.source_path.clone(),
                default_runner: pipeline.definition.body.defaults.runner.clone(),
                default_profile: pipeline.definition.body.defaults.profile.clone(),
                default_enable_complexity: pipeline.definition.body.defaults.enable_complexity,
                stages: pipeline
                    .stages
                    .iter()
                    .zip(pipeline.definition.declared_stages())
                    .map(|(stage, declared)| PipelineDeclaredStageView {
                        id: stage.stage_id.clone(),
                        title: stage.title.clone(),
                        source_path: stage.source_path.clone(),
                        work_level: stage.work_level.clone(),
                        sets: declared.sets.clone(),
                        activation: declared.activation.as_ref().map(|activation| {
                            PipelineActivationView {
                                operator: match activation.when.operator {
                                    handbook_pipeline::pipeline::ActivationOperator::Any => {
                                        PipelineActivationOperator::Any
                                    }
                                    handbook_pipeline::pipeline::ActivationOperator::All => {
                                        PipelineActivationOperator::All
                                    }
                                },
                                clauses: {
                                    let mut clauses = activation
                                        .when
                                        .clauses
                                        .iter()
                                        .map(|clause| PipelineActivationClause {
                                            variable: clause.variable.clone(),
                                            value: clause.value,
                                        })
                                        .collect::<Vec<_>>();
                                    clauses
                                        .sort_by(|left, right| left.variable.cmp(&right.variable));
                                    clauses
                                },
                            }
                        }),
                    })
                    .collect(),
            }))
        }
        Ok(PipelineSelection::Stage(stage)) => Ok(PipelineShowOutcome::Stage(PipelineStageView {
            id: stage.id,
            kind: stage.kind,
            version: stage.version,
            title: stage.title,
            description: stage.description,
            work_level: stage.work_level,
            source_path: stage.source_path,
            pipeline_ids: stage.pipelines,
        })),
        Err(PipelineMetadataSelectionError::Catalog(error)) => {
            Err(PipelineShowFailure::Catalog(error.to_string()))
        }
        Err(PipelineMetadataSelectionError::Lookup(error)) => {
            Err(PipelineShowFailure::Selector(map_selector_refusal(error)))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineSelectorRefusal {
    Unsupported {
        selector: String,
        reason: String,
    },
    Ambiguous {
        selector: String,
        matches: Vec<String>,
    },
    Unknown {
        selector: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineResolveRequest {
    pub repository_root: PathBuf,
    pub selector: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineResolveOutcome {
    pub pipeline_id: String,
    pub state: PipelineRouteStateView,
    pub effective_run: PipelineRouteRunView,
    pub stages: Vec<PipelineResolvedStageView>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineRouteStateView {
    pub revision: u64,
    pub routing: BTreeMap<String, bool>,
    pub charter_ref: Option<String>,
    pub project_context_ref: Option<String>,
    pub run: PipelineRouteRunView,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PipelineRouteRunView {
    pub runner: Option<String>,
    pub profile: Option<String>,
    pub repository_root: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineResolvedStageView {
    pub stage_id: String,
    pub status: PipelineRouteStageStatus,
    pub reason: Option<PipelineRouteStageReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineRouteStageStatus {
    Active,
    Skipped,
    Blocked,
    Next,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineRouteStageReason {
    SkippedActivationFalse {
        unsatisfied_variables: Vec<String>,
    },
    NextMissingRouteVariables {
        missing_variables: Vec<String>,
    },
    BlockedByUnresolvedStage {
        upstream_stage_id: String,
        upstream_status: PipelineRouteStageStatus,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineResolveFailure {
    Selection(PipelineSelectedDefinitionFailure),
    RouteStateRead(String),
    RouteVariables(String),
    RouteResolution(String),
    RouteBasisBuild(String),
    RouteBasisPersistenceRefused(String),
    RouteBasisPersistence(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineSelectedDefinitionFailure {
    Catalog(String),
    Selector(PipelineSelectorRefusal),
    Definition(String),
}

pub fn resolve_pipeline(
    request: &PipelineResolveRequest,
) -> Result<PipelineResolveOutcome, PipelineResolveFailure> {
    let pipeline = load_selected_pipeline_definition(&request.repository_root, &request.selector)
        .map_err(map_selected_definition_failure)
        .map_err(PipelineResolveFailure::Selection)?;
    let pipeline_id = pipeline.header.id.clone();
    let supported_variables = supported_route_state_variables(&pipeline);
    let state = load_route_state_with_supported_variables(
        &request.repository_root,
        &pipeline_id,
        &supported_variables,
    )
    .map_err(|error| PipelineResolveFailure::RouteStateRead(error.to_string()))?;
    let route_variables = RouteVariables::new(state.routing.clone())
        .map_err(|error| PipelineResolveFailure::RouteVariables(error.to_string()))?;
    let route = resolve_pipeline_route(&pipeline, &route_variables)
        .map_err(|error| PipelineResolveFailure::RouteResolution(error.to_string()))?;
    let route_basis = build_route_basis(&request.repository_root, &pipeline, &state, &route)
        .map_err(|error| PipelineResolveFailure::RouteBasisBuild(error.to_string()))?;

    match persist_route_basis(&request.repository_root, &pipeline_id, route_basis) {
        Ok(RouteBasisPersistOutcome::Applied(_)) => {}
        Ok(RouteBasisPersistOutcome::Refused(refusal)) => {
            return Err(PipelineResolveFailure::RouteBasisPersistenceRefused(
                refusal.to_string(),
            ));
        }
        Err(error) => {
            return Err(PipelineResolveFailure::RouteBasisPersistence(
                error.to_string(),
            ));
        }
    }

    Ok(PipelineResolveOutcome {
        pipeline_id,
        state: map_route_state(&state),
        effective_run: map_route_run(&effective_route_basis_run(
            &request.repository_root,
            &pipeline,
            &state,
        )),
        stages: route
            .stages
            .into_iter()
            .map(|stage| PipelineResolvedStageView {
                stage_id: stage.stage_id,
                status: map_route_status(stage.status),
                reason: stage.reason.map(map_route_reason),
            })
            .collect(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineStateUpdateRequest {
    pub repository_root: PathBuf,
    pub selector: String,
    pub mutation: PipelineStateMutation,
    pub expected_revision: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineStateMutation {
    RoutingVariable { variable: String, value: bool },
    CharterReference { value: String },
    ProjectContextReference { value: String },
    Runner { value: String },
    Profile { value: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineStateUpdateOutcome {
    pub pipeline_id: String,
    pub result: PipelineStateUpdateResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineStateUpdateResult {
    Applied(PipelineRouteStateView),
    Refused(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineStateUpdateFailure {
    Selection(PipelineSelectedDefinitionFailure),
    RouteStateRead(String),
    Mutation(String),
}

pub fn update_pipeline_state(
    request: &PipelineStateUpdateRequest,
) -> Result<PipelineStateUpdateOutcome, PipelineStateUpdateFailure> {
    let pipeline = load_selected_pipeline_definition(&request.repository_root, &request.selector)
        .map_err(map_selected_definition_failure)
        .map_err(PipelineStateUpdateFailure::Selection)?;
    let pipeline_id = pipeline.header.id.clone();
    let supported_variables = supported_route_state_variables(&pipeline);
    let current_state = load_route_state_with_supported_variables(
        &request.repository_root,
        &pipeline_id,
        &supported_variables,
    )
    .map_err(|error| PipelineStateUpdateFailure::RouteStateRead(error.to_string()))?;
    let expected_revision = request.expected_revision.unwrap_or(current_state.revision);
    let mutation = map_state_mutation(&request.mutation);

    let result = match set_route_state(
        &request.repository_root,
        &pipeline_id,
        supported_variables,
        mutation,
        expected_revision,
    )
    .map_err(|error| PipelineStateUpdateFailure::Mutation(error.to_string()))?
    {
        RouteStateMutationOutcome::Applied(state) => {
            PipelineStateUpdateResult::Applied(map_route_state(&state))
        }
        RouteStateMutationOutcome::Refused(refusal) => {
            PipelineStateUpdateResult::Refused(refusal.to_string())
        }
    };

    Ok(PipelineStateUpdateOutcome {
        pipeline_id,
        result,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileRequest {
    pub repository_root: PathBuf,
    pub pipeline_selector: String,
    pub stage_selector: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileOutcome {
    pub target: PipelineCompileTargetView,
    pub basis: PipelineRouteBasisView,
    pub variables: Vec<PipelineCompileVariableView>,
    pub documents: Vec<PipelineCompileDocumentView>,
    pub outputs: Vec<PipelineCompileOutputView>,
    pub gating: PipelineCompileGatingView,
    pub stage_body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileTargetView {
    pub pipeline_id: String,
    pub stage_id: String,
    pub stage_file: String,
    pub title: String,
    pub description: String,
    pub work_level: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineRouteBasisView {
    pub schema_version: String,
    pub state_revision: u64,
    pub pipeline_file: String,
    pub pipeline_file_sha256: String,
    pub routing: BTreeMap<String, bool>,
    pub charter_ref: Option<String>,
    pub project_context_ref: Option<String>,
    pub run: PipelineRouteRunView,
    pub runner: PipelineRouteBasisRunnerView,
    pub profile: PipelineRouteBasisProfileView,
    pub route: Vec<PipelineRouteBasisStageView>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineRouteBasisRunnerView {
    pub id: String,
    pub file: String,
    pub file_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineRouteBasisProfileView {
    pub id: String,
    pub profile_yaml_sha256: String,
    pub commands_yaml_sha256: String,
    pub conventions_md_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineRouteBasisStageView {
    pub stage_id: String,
    pub file: String,
    pub status: PipelineRouteStageStatus,
    pub reason: Option<PipelineRouteStageReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileVariableView {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileDocumentView {
    pub kind: PipelineCompileDocumentKindView,
    pub path: String,
    pub required: bool,
    pub status: PipelineCompileDocumentStatusView,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineCompileDocumentKindView {
    Include,
    Runner,
    Profile,
    Library,
    Artifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineCompileDocumentStatusView {
    Present,
    MissingOptional,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileOutputView {
    pub kind: PipelineCompileOutputKindView,
    pub path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineCompileOutputKindView {
    Artifact,
    RepositoryFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileGatingView {
    pub mode: Option<String>,
    pub fail_on: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileRefusalView {
    pub classification: PipelineCompileRefusalClassificationView,
    pub summary: String,
    pub pipeline_id: Option<String>,
    pub stage_id: Option<String>,
    pub recovery: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineCompileRefusalClassificationView {
    UnsupportedTarget,
    InvalidDefinition,
    InvalidState,
    MissingRouteBasis,
    MalformedRouteBasis,
    StaleRouteBasis,
    InactiveStage,
    MissingRequiredInput,
    EmptyRequiredInput,
}

pub fn compile_pipeline(
    request: &PipelineCompileRequest,
) -> Result<PipelineCompileOutcome, PipelineCompileRefusalView> {
    compile_pipeline_stage(
        &request.repository_root,
        &request.pipeline_selector,
        &request.stage_selector,
    )
    .map(map_compile_outcome)
    .map_err(map_compile_refusal)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCaptureRequestV1 {
    pub repository_root: PathBuf,
    pub pipeline_selector: String,
    pub stage_selector: String,
    pub input: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCaptureApplyRequest {
    pub repository_root: PathBuf,
    pub capture_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCapturePreviewView {
    pub plan: PipelineCapturePlanView,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCaptureApplyView {
    pub plan: PipelineCapturePlanView,
    pub written_files: Vec<String>,
    pub persisted_state_revision: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCapturePlanView {
    pub target: PipelineCaptureTargetView,
    pub route_basis_revision: u64,
    pub artifact_write_paths: Vec<String>,
    pub repository_mirror_write_paths: Vec<String>,
    pub state_updates: Vec<PipelineCaptureStateUpdateView>,
    pub capture_id: String,
    pub post_apply_next_safe_action: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCaptureTargetView {
    pub pipeline_id: String,
    pub stage_id: String,
    pub stage_file: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCaptureStateUpdateView {
    pub field_path: String,
    pub value: PipelineCaptureStateValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineCaptureStateValue {
    Boolean(bool),
    Text(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCaptureRefusalView {
    pub classification: PipelineCaptureRefusalClassificationView,
    pub summary: String,
    pub pipeline_id: Option<String>,
    pub stage_id: Option<String>,
    pub recovery: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineCaptureRefusalClassificationView {
    UnsupportedTarget,
    InvalidDefinition,
    InvalidState,
    MissingRouteBasis,
    MalformedRouteBasis,
    StaleRouteBasis,
    InactiveStage,
    InvalidCaptureInput,
    InvalidWriteTarget,
    MissingCaptureId,
    TamperedCaptureCache,
    RevisionConflict,
    WriteFailure,
    StatePersistenceFailure,
    CacheFailure,
}

pub fn preview_pipeline_capture(
    request: &PipelineCaptureRequestV1,
) -> Result<PipelineCapturePreviewView, PipelineCaptureRefusalView> {
    owner_preview_pipeline_capture(
        &request.repository_root,
        &PipelineCaptureRequest {
            pipeline_selector: request.pipeline_selector.clone(),
            stage_selector: request.stage_selector.clone(),
            input: request.input.clone(),
        },
    )
    .map(|preview| PipelineCapturePreviewView {
        plan: map_capture_plan(preview.plan),
    })
    .map_err(map_capture_refusal)
}

pub fn capture_pipeline_output(
    request: &PipelineCaptureRequestV1,
) -> Result<PipelineCaptureApplyView, PipelineCaptureRefusalView> {
    owner_capture_pipeline_output(
        &request.repository_root,
        &PipelineCaptureRequest {
            pipeline_selector: request.pipeline_selector.clone(),
            stage_selector: request.stage_selector.clone(),
            input: request.input.clone(),
        },
    )
    .map(|result| PipelineCaptureApplyView {
        plan: map_capture_plan(result.plan),
        written_files: result.written_files,
        persisted_state_revision: result.persisted_state_revision,
    })
    .map_err(map_capture_refusal)
}

pub fn apply_pipeline_capture(
    request: &PipelineCaptureApplyRequest,
) -> Result<PipelineCaptureApplyView, PipelineCaptureRefusalView> {
    owner_apply_pipeline_capture(&request.repository_root, &request.capture_id)
        .map(|result| PipelineCaptureApplyView {
            plan: map_capture_plan(result.plan),
            written_files: result.written_files,
            persisted_state_revision: result.persisted_state_revision,
        })
        .map_err(map_capture_refusal)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineHandoffRequest {
    pub repository_root: PathBuf,
    pub pipeline_selector: String,
    pub consumer_selector: String,
    pub producer_command: String,
    pub producer_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineHandoffOutcome {
    pub pipeline_id: String,
    pub consumer_id: String,
    pub feature_id: String,
    pub bundle_root: String,
    pub written_files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineHandoffRefusalView {
    pub classification: PipelineHandoffRefusalClassificationView,
    pub summary: String,
    pub pipeline_id: Option<String>,
    pub consumer_id: Option<String>,
    pub recovery: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineHandoffRefusalClassificationView {
    UnsupportedTarget,
    InvalidState,
    MissingRequiredInput,
    InvalidProvenance,
    WriteFailure,
}

pub fn emit_pipeline_handoff(
    request: &PipelineHandoffRequest,
) -> Result<PipelineHandoffOutcome, PipelineHandoffRefusalView> {
    emit_pipeline_handoff_bundle(
        &request.repository_root,
        &PipelineHandoffEmitRequest {
            pipeline_selector: request.pipeline_selector.clone(),
            consumer_selector: request.consumer_selector.clone(),
            producer_command: request.producer_command.clone(),
            producer_version: request.producer_version.clone(),
        },
    )
    .map(|result| PipelineHandoffOutcome {
        pipeline_id: result.manifest.pipeline_id,
        consumer_id: result.manifest.consumer_id,
        feature_id: result.manifest.feature_id,
        bundle_root: result.bundle_root,
        written_files: result.written_files,
    })
    .map_err(map_handoff_refusal)
}

fn map_selector_refusal(error: PipelineLookupError) -> PipelineSelectorRefusal {
    match error {
        PipelineLookupError::UnsupportedSelector { selector, reason } => {
            PipelineSelectorRefusal::Unsupported {
                selector,
                reason: reason.to_string(),
            }
        }
        PipelineLookupError::AmbiguousSelector { selector, matches } => {
            PipelineSelectorRefusal::Ambiguous { selector, matches }
        }
        PipelineLookupError::UnknownSelector { selector } => {
            PipelineSelectorRefusal::Unknown { selector }
        }
    }
}

fn map_selected_definition_failure(
    error: SelectedPipelineLoadError,
) -> PipelineSelectedDefinitionFailure {
    match error {
        SelectedPipelineLoadError::Catalog(error) => {
            PipelineSelectedDefinitionFailure::Catalog(error.to_string())
        }
        SelectedPipelineLoadError::Lookup(error) => {
            PipelineSelectedDefinitionFailure::Selector(map_selector_refusal(error))
        }
        SelectedPipelineLoadError::Load(error) => {
            PipelineSelectedDefinitionFailure::Definition(error.to_string())
        }
    }
}

fn map_route_state(state: &RouteState) -> PipelineRouteStateView {
    PipelineRouteStateView {
        revision: state.revision,
        routing: state.routing.clone(),
        charter_ref: state.refs.charter_ref.clone(),
        project_context_ref: state.refs.project_context_ref.clone(),
        run: map_route_run(&state.run),
    }
}

fn map_route_run(run: &handbook_pipeline::route_state::RouteStateRun) -> PipelineRouteRunView {
    PipelineRouteRunView {
        runner: run.runner.clone(),
        profile: run.profile.clone(),
        repository_root: run.repo_root.clone(),
    }
}

fn map_route_status(status: RouteStageStatus) -> PipelineRouteStageStatus {
    match status {
        RouteStageStatus::Active => PipelineRouteStageStatus::Active,
        RouteStageStatus::Skipped => PipelineRouteStageStatus::Skipped,
        RouteStageStatus::Blocked => PipelineRouteStageStatus::Blocked,
        RouteStageStatus::Next => PipelineRouteStageStatus::Next,
    }
}

fn map_route_reason(reason: RouteStageReason) -> PipelineRouteStageReason {
    match reason {
        RouteStageReason::SkippedActivationFalse {
            unsatisfied_variables,
            ..
        } => PipelineRouteStageReason::SkippedActivationFalse {
            unsatisfied_variables,
        },
        RouteStageReason::NextMissingRouteVariables {
            missing_variables, ..
        } => PipelineRouteStageReason::NextMissingRouteVariables { missing_variables },
        RouteStageReason::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status,
        } => PipelineRouteStageReason::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status: map_route_status(upstream_status),
        },
    }
}

fn map_state_mutation(mutation: &PipelineStateMutation) -> RouteStateMutation {
    match mutation {
        PipelineStateMutation::RoutingVariable { variable, value } => {
            RouteStateMutation::RoutingVariable {
                variable: variable.clone(),
                value: *value,
            }
        }
        PipelineStateMutation::CharterReference { value } => RouteStateMutation::RefCharterRef {
            value: value.clone(),
        },
        PipelineStateMutation::ProjectContextReference { value } => {
            RouteStateMutation::RefProjectContextRef {
                value: value.clone(),
            }
        }
        PipelineStateMutation::Runner { value } => RouteStateMutation::RunRunner {
            value: value.clone(),
        },
        PipelineStateMutation::Profile { value } => RouteStateMutation::RunProfile {
            value: value.clone(),
        },
    }
}

fn map_compile_outcome(
    result: handbook_pipeline::pipeline_compile::PipelineCompileResult,
) -> PipelineCompileOutcome {
    PipelineCompileOutcome {
        target: PipelineCompileTargetView {
            pipeline_id: result.target.pipeline_id,
            stage_id: result.target.stage_id,
            stage_file: result.target.stage_file,
            title: result.target.title,
            description: result.target.description,
            work_level: result.target.work_level,
            tags: result.target.tags,
        },
        basis: map_route_basis(result.basis),
        variables: result
            .variables
            .into_iter()
            .map(|variable| PipelineCompileVariableView {
                name: variable.name,
                value: variable.value,
            })
            .collect(),
        documents: result
            .documents
            .into_iter()
            .map(|document| PipelineCompileDocumentView {
                kind: match document.kind {
                    PipelineCompileDocumentKind::Include => {
                        PipelineCompileDocumentKindView::Include
                    }
                    PipelineCompileDocumentKind::Runner => PipelineCompileDocumentKindView::Runner,
                    PipelineCompileDocumentKind::Profile => {
                        PipelineCompileDocumentKindView::Profile
                    }
                    PipelineCompileDocumentKind::Library => {
                        PipelineCompileDocumentKindView::Library
                    }
                    PipelineCompileDocumentKind::Artifact => {
                        PipelineCompileDocumentKindView::Artifact
                    }
                },
                path: document.path,
                required: document.required,
                status: match document.status {
                    PipelineCompileDocumentStatus::Present => {
                        PipelineCompileDocumentStatusView::Present
                    }
                    PipelineCompileDocumentStatus::MissingOptional => {
                        PipelineCompileDocumentStatusView::MissingOptional
                    }
                },
                content: document.content,
            })
            .collect(),
        outputs: result
            .outputs
            .into_iter()
            .map(|output| PipelineCompileOutputView {
                kind: match output.kind {
                    PipelineCompileOutputKind::Artifact => PipelineCompileOutputKindView::Artifact,
                    PipelineCompileOutputKind::RepoFile => {
                        PipelineCompileOutputKindView::RepositoryFile
                    }
                },
                path: output.path,
            })
            .collect(),
        gating: PipelineCompileGatingView {
            mode: result.gating.mode,
            fail_on: result.gating.fail_on,
            notes: result.gating.notes,
        },
        stage_body: result.stage_body,
    }
}

fn map_route_basis(basis: handbook_pipeline::route_state::RouteBasis) -> PipelineRouteBasisView {
    PipelineRouteBasisView {
        schema_version: basis.schema_version,
        state_revision: basis.state_revision,
        pipeline_file: basis.pipeline_file,
        pipeline_file_sha256: basis.pipeline_file_sha256,
        routing: basis.routing,
        charter_ref: basis.refs.charter_ref,
        project_context_ref: basis.refs.project_context_ref,
        run: map_route_run(&basis.run),
        runner: PipelineRouteBasisRunnerView {
            id: basis.runner.id,
            file: basis.runner.file,
            file_sha256: basis.runner.file_sha256,
        },
        profile: PipelineRouteBasisProfileView {
            id: basis.profile.id,
            profile_yaml_sha256: basis.profile.profile_yaml_sha256,
            commands_yaml_sha256: basis.profile.commands_yaml_sha256,
            conventions_md_sha256: basis.profile.conventions_md_sha256,
        },
        route: basis
            .route
            .into_iter()
            .map(|stage| PipelineRouteBasisStageView {
                stage_id: stage.stage_id,
                file: stage.file,
                status: match stage.status {
                    RouteBasisStageStatus::Active => PipelineRouteStageStatus::Active,
                    RouteBasisStageStatus::Skipped => PipelineRouteStageStatus::Skipped,
                    RouteBasisStageStatus::Blocked => PipelineRouteStageStatus::Blocked,
                    RouteBasisStageStatus::Next => PipelineRouteStageStatus::Next,
                },
                reason: stage.reason.map(map_route_basis_reason),
            })
            .collect(),
    }
}

fn map_route_basis_reason(reason: RouteBasisStageReason) -> PipelineRouteStageReason {
    match reason {
        RouteBasisStageReason::SkippedActivationFalse {
            unsatisfied_variables,
            ..
        } => PipelineRouteStageReason::SkippedActivationFalse {
            unsatisfied_variables,
        },
        RouteBasisStageReason::NextMissingRouteVariables {
            missing_variables, ..
        } => PipelineRouteStageReason::NextMissingRouteVariables { missing_variables },
        RouteBasisStageReason::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status,
        } => PipelineRouteStageReason::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status: match upstream_status {
                RouteBasisStageStatus::Active => PipelineRouteStageStatus::Active,
                RouteBasisStageStatus::Skipped => PipelineRouteStageStatus::Skipped,
                RouteBasisStageStatus::Blocked => PipelineRouteStageStatus::Blocked,
                RouteBasisStageStatus::Next => PipelineRouteStageStatus::Next,
            },
        },
    }
}

fn map_compile_refusal(refusal: PipelineCompileRefusal) -> PipelineCompileRefusalView {
    PipelineCompileRefusalView {
        classification: match refusal.classification {
            PipelineCompileRefusalClassification::UnsupportedTarget => {
                PipelineCompileRefusalClassificationView::UnsupportedTarget
            }
            PipelineCompileRefusalClassification::InvalidDefinition => {
                PipelineCompileRefusalClassificationView::InvalidDefinition
            }
            PipelineCompileRefusalClassification::InvalidState => {
                PipelineCompileRefusalClassificationView::InvalidState
            }
            PipelineCompileRefusalClassification::MissingRouteBasis => {
                PipelineCompileRefusalClassificationView::MissingRouteBasis
            }
            PipelineCompileRefusalClassification::MalformedRouteBasis => {
                PipelineCompileRefusalClassificationView::MalformedRouteBasis
            }
            PipelineCompileRefusalClassification::StaleRouteBasis => {
                PipelineCompileRefusalClassificationView::StaleRouteBasis
            }
            PipelineCompileRefusalClassification::InactiveStage => {
                PipelineCompileRefusalClassificationView::InactiveStage
            }
            PipelineCompileRefusalClassification::MissingRequiredInput => {
                PipelineCompileRefusalClassificationView::MissingRequiredInput
            }
            PipelineCompileRefusalClassification::EmptyRequiredInput => {
                PipelineCompileRefusalClassificationView::EmptyRequiredInput
            }
        },
        summary: refusal.summary,
        pipeline_id: refusal.pipeline_id,
        stage_id: refusal.stage_id,
        recovery: refusal.recovery,
    }
}

fn map_capture_plan(
    plan: handbook_pipeline::pipeline_capture::PipelineCapturePlan,
) -> PipelineCapturePlanView {
    PipelineCapturePlanView {
        target: PipelineCaptureTargetView {
            pipeline_id: plan.target.pipeline_id,
            stage_id: plan.target.stage_id,
            stage_file: plan.target.stage_file,
            title: plan.target.title,
        },
        route_basis_revision: plan.basis.state_revision,
        artifact_write_paths: plan
            .artifact_writes
            .into_iter()
            .map(|write| write.path)
            .collect(),
        repository_mirror_write_paths: plan
            .repo_mirror_writes
            .into_iter()
            .map(|write| write.path)
            .collect(),
        state_updates: plan
            .state_updates
            .into_iter()
            .map(|update| PipelineCaptureStateUpdateView {
                field_path: update.field_path,
                value: match update.value {
                    handbook_pipeline::pipeline_capture::PipelineCaptureStateValue::Bool(value) => {
                        PipelineCaptureStateValue::Boolean(value)
                    }
                    handbook_pipeline::pipeline_capture::PipelineCaptureStateValue::String(
                        value,
                    ) => PipelineCaptureStateValue::Text(value),
                },
            })
            .collect(),
        capture_id: plan.capture_id,
        post_apply_next_safe_action: plan.post_apply_next_safe_action,
    }
}

fn map_capture_refusal(refusal: PipelineCaptureRefusal) -> PipelineCaptureRefusalView {
    PipelineCaptureRefusalView {
        classification: match refusal.classification {
            PipelineCaptureRefusalClassification::UnsupportedTarget => {
                PipelineCaptureRefusalClassificationView::UnsupportedTarget
            }
            PipelineCaptureRefusalClassification::InvalidDefinition => {
                PipelineCaptureRefusalClassificationView::InvalidDefinition
            }
            PipelineCaptureRefusalClassification::InvalidState => {
                PipelineCaptureRefusalClassificationView::InvalidState
            }
            PipelineCaptureRefusalClassification::MissingRouteBasis => {
                PipelineCaptureRefusalClassificationView::MissingRouteBasis
            }
            PipelineCaptureRefusalClassification::MalformedRouteBasis => {
                PipelineCaptureRefusalClassificationView::MalformedRouteBasis
            }
            PipelineCaptureRefusalClassification::StaleRouteBasis => {
                PipelineCaptureRefusalClassificationView::StaleRouteBasis
            }
            PipelineCaptureRefusalClassification::InactiveStage => {
                PipelineCaptureRefusalClassificationView::InactiveStage
            }
            PipelineCaptureRefusalClassification::InvalidCaptureInput => {
                PipelineCaptureRefusalClassificationView::InvalidCaptureInput
            }
            PipelineCaptureRefusalClassification::InvalidWriteTarget => {
                PipelineCaptureRefusalClassificationView::InvalidWriteTarget
            }
            PipelineCaptureRefusalClassification::MissingCaptureId => {
                PipelineCaptureRefusalClassificationView::MissingCaptureId
            }
            PipelineCaptureRefusalClassification::TamperedCaptureCache => {
                PipelineCaptureRefusalClassificationView::TamperedCaptureCache
            }
            PipelineCaptureRefusalClassification::RevisionConflict => {
                PipelineCaptureRefusalClassificationView::RevisionConflict
            }
            PipelineCaptureRefusalClassification::WriteFailure => {
                PipelineCaptureRefusalClassificationView::WriteFailure
            }
            PipelineCaptureRefusalClassification::StatePersistenceFailure => {
                PipelineCaptureRefusalClassificationView::StatePersistenceFailure
            }
            PipelineCaptureRefusalClassification::CacheFailure => {
                PipelineCaptureRefusalClassificationView::CacheFailure
            }
        },
        summary: refusal.summary,
        pipeline_id: refusal.pipeline_id,
        stage_id: refusal.stage_id,
        recovery: refusal.recovery,
    }
}

fn map_handoff_refusal(refusal: PipelineHandoffRefusal) -> PipelineHandoffRefusalView {
    PipelineHandoffRefusalView {
        classification: match refusal.classification {
            PipelineHandoffRefusalClassification::UnsupportedTarget => {
                PipelineHandoffRefusalClassificationView::UnsupportedTarget
            }
            PipelineHandoffRefusalClassification::InvalidState => {
                PipelineHandoffRefusalClassificationView::InvalidState
            }
            PipelineHandoffRefusalClassification::MissingRequiredInput => {
                PipelineHandoffRefusalClassificationView::MissingRequiredInput
            }
            PipelineHandoffRefusalClassification::InvalidProvenance => {
                PipelineHandoffRefusalClassificationView::InvalidProvenance
            }
            PipelineHandoffRefusalClassification::WriteFailure => {
                PipelineHandoffRefusalClassificationView::WriteFailure
            }
        },
        summary: refusal.summary,
        pipeline_id: refusal.pipeline_id,
        consumer_id: refusal.consumer_id,
        recovery: refusal.recovery,
    }
}
