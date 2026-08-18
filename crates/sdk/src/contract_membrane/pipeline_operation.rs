use super::{
    canonical_json_bytes, parse_canonical_json, BoundedString, BoundedVec, CapabilityEntry,
    ContractError, ContractLimits, ContractValue, CorrelationRef, CurrentTransportEvidence,
    CurrentTransportEvidenceSealed, DeclaredWrite, ExactBinding, ExactRef, IdempotencyKeyConsumer,
    IdempotencyKeyConsumerSealed, IdempotencyStage, IdempotencyState, JsonPointer,
    OperationAdmissionResult, OperationClass, OperationDefinition, OperationIdempotency,
    OperationOutcomeSchemas, OperationOwnerBinding, OperationRequestV1, OperationResponseV1,
    OperationSchemaValidator, OperationTransport, OwnerAdmissionToken, Problem, ProblemBinding,
    ProblemCategory, RecheckCondition, RequestId, RetryDirective, SemVer, TerminalFields,
    TerminalStatus, UtcTimestamp, WriteReceipt,
};
use crate::pipeline_route_api::{
    PipelineActivationClause, PipelineActivationOperator, PipelineActivationView,
    PipelineCatalogEntry, PipelineCatalogOutcome, PipelineCompileDocumentKindView,
    PipelineCompileDocumentStatusView, PipelineCompileDocumentView, PipelineCompileGatingView,
    PipelineCompileOutcome, PipelineCompileOutputKindView, PipelineCompileOutputView,
    PipelineCompileRefusalClassificationView, PipelineCompileRefusalView,
    PipelineCompileTargetView, PipelineDeclaredStageView, PipelineDefinitionView,
    PipelineRouteBasisProfileView, PipelineRouteBasisRunnerView, PipelineRouteBasisStageView,
    PipelineRouteBasisView, PipelineRouteRunView, PipelineRouteStageReason,
    PipelineRouteStageStatus, PipelineSelectorRefusal, PipelineShowOutcome, PipelineStageView,
};
use handbook_pipeline::{
    pipeline::{
        load_pipeline_catalog_metadata, load_pipeline_selection_metadata,
        load_selected_pipeline_definition, supported_route_state_variables, PipelineCatalogError,
        PipelineLookupError, PipelineMetadataSelectionError, PipelineSelection,
        SelectedPipelineLoadError,
    },
    pipeline_capture::{
        apply_pipeline_capture_with_effects, capture_pipeline_output_with_effects,
        preview_pipeline_capture, PipelineCaptureApplyExecution, PipelineCaptureApplyFailure,
        PipelineCapturePlan, PipelineCaptureRefusal, PipelineCaptureRefusalClassification,
        PipelineCaptureRequest,
    },
    pipeline_compile::{
        compile_pipeline_stage, PipelineCompileDocumentKind, PipelineCompileDocumentStatus,
        PipelineCompileOutputKind, PipelineCompileRefusal, PipelineCompileRefusalClassification,
        PipelineCompileResult,
    },
    pipeline_handoff::{
        emit_pipeline_handoff_bundle_with_effects, PipelineHandoffEmitExecution,
        PipelineHandoffEmitFailure, PipelineHandoffEmitRequest, PipelineHandoffRefusal,
        PipelineHandoffRefusalClassification,
    },
    pipeline_mutation_effect::PipelineMutationEffect,
    pipeline_mutation_journal::{
        PipelineMutationJournalError, PipelineMutationJournalErrorClassification,
        PipelineMutationJournalLookup, PipelineMutationJournalSession,
    },
    route_state::{
        set_route_state_with_effects, RouteBasisStageReason, RouteBasisStageStatus, RouteState,
        RouteStateMutation, RouteStateMutationExecution, RouteStateMutationFailure,
        RouteStateMutationOutcome, RouteStateMutationRefusal, RouteStateStoreError,
    },
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub(crate) const PIPELINE_LIST_REF: &str = "pipeline.catalog.list@1.0.0";
pub(crate) const PIPELINE_READ_REF: &str = "pipeline.catalog.read@1.0.0";
pub(crate) const PIPELINE_COMPILE_REF: &str = "pipeline.compile@1.0.0";
pub(crate) const PIPELINE_CAPTURE_PLAN_REF: &str = "pipeline.capture.plan@1.0.0";
pub(crate) const PIPELINE_CAPTURE_APPLY_REF: &str = "pipeline.capture.apply@1.0.0";
pub(crate) const PIPELINE_HANDOFF_EMIT_REF: &str = "pipeline.handoff.emit@1.0.0";
pub(crate) const PIPELINE_STATE_APPLY_REF: &str = "pipeline.state.apply@1.0.0";
pub(crate) const PIPELINE_DETAILS_REF: &str = "handbook.pipeline-operation-details@1.0.0";
pub(crate) const PIPELINE_RECHECK_REF: &str = "handbook.pipeline-recheck@1.0.0";

const LIST_REQUEST_REF: &str = "handbook.operation.pipeline-catalog-list-request@1.0.0";
const LIST_RESULT_REF: &str = "handbook.operation.pipeline-catalog-list-result@1.0.0";
const READ_REQUEST_REF: &str = "handbook.operation.pipeline-catalog-read-request@1.0.0";
const READ_RESULT_REF: &str = "handbook.operation.pipeline-catalog-read-result@1.0.0";
const COMPILE_REQUEST_REF: &str = "handbook.operation.pipeline-compile-request@1.0.0";
const COMPILE_RESULT_REF: &str = "handbook.operation.pipeline-compile-result@1.0.0";
const CAPTURE_PLAN_REQUEST_REF: &str = "handbook.operation.pipeline-capture-plan-request@1.0.0";
const CAPTURE_PLAN_RESULT_REF: &str = "handbook.operation.pipeline-capture-plan-result@1.0.0";
const CAPTURE_APPLY_REQUEST_REF: &str = "handbook.operation.pipeline-capture-apply-request@1.0.0";
const CAPTURE_APPLY_RESULT_REF: &str = "handbook.operation.pipeline-capture-apply-result@1.0.0";
const HANDOFF_EMIT_REQUEST_REF: &str = "handbook.operation.pipeline-handoff-emit-request@1.0.0";
const HANDOFF_EMIT_RESULT_REF: &str = "handbook.operation.pipeline-handoff-emit-result@1.0.0";
const STATE_APPLY_REQUEST_REF: &str = "handbook.operation.pipeline-state-apply-request@1.0.0";
const STATE_APPLY_RESULT_REF: &str = "handbook.operation.pipeline-state-apply-result@1.0.0";
pub(crate) const PIPELINE_WRITE_CONDITION_REF: &str = "handbook.pipeline-write-condition@1.0.0";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCapturePlanRequestV1 {
    pipeline_selector: String,
    stage_selector: String,
    input: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PipelineCaptureApplyModeV1 {
    Direct,
    Cached,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCaptureApplyRequestV1 {
    mode: PipelineCaptureApplyModeV1,
    capture_id: Option<String>,
    pipeline_selector: Option<String>,
    stage_selector: Option<String>,
    input: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCaptureApplyRequestSchemaV1 {
    idempotency_key: String,
    mode: PipelineCaptureApplyModeV1,
    capture_id: Option<String>,
    pipeline_selector: Option<String>,
    stage_selector: Option<String>,
    input: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineHandoffEmitRequestV1 {
    pipeline_selector: String,
    consumer_selector: String,
    producer_command: String,
    producer_version: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineHandoffEmitRequestSchemaV1 {
    idempotency_key: String,
    pipeline_selector: String,
    consumer_selector: String,
    producer_command: String,
    producer_version: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "mutation_kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum PipelineStateMutationV1 {
    RoutingVariable { variable: String, value: bool },
    CharterReference { value: String },
    ProjectContextReference { value: String },
    Runner { value: String },
    Profile { value: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineStateApplyRequestV1 {
    selector: String,
    mutation: PipelineStateMutationV1,
    expected_revision: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineStateApplyRequestSchemaV1 {
    idempotency_key: String,
    selector: String,
    mutation: PipelineStateMutationV1,
    expected_revision: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCaptureTargetDtoV1 {
    pipeline_id: String,
    stage_id: String,
    stage_file: String,
    title: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub(crate) enum PipelineCaptureStateValueDtoV1 {
    Boolean(bool),
    Text(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCaptureStateUpdateDtoV1 {
    field_path: String,
    value: PipelineCaptureStateValueDtoV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCapturePlanDtoV1 {
    target: PipelineCaptureTargetDtoV1,
    route_basis_revision: u64,
    artifact_write_paths: Vec<String>,
    repository_mirror_write_paths: Vec<String>,
    state_updates: Vec<PipelineCaptureStateUpdateDtoV1>,
    capture_id: String,
    post_apply_next_safe_action: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCapturePlanResultV1 {
    plan: PipelineCapturePlanDtoV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCaptureApplyResultV1 {
    plan: PipelineCapturePlanDtoV1,
    written_files: Vec<String>,
    persisted_state_revision: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineHandoffEmitResultV1 {
    pipeline_id: String,
    consumer_id: String,
    feature_id: String,
    bundle_root: String,
    written_files: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineStateApplyResultV1 {
    pipeline_id: String,
    state: PipelineRouteStateDtoV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineRouteStateDtoV1 {
    revision: u64,
    routing: Vec<PipelineRoutingEntryV1>,
    charter_ref: Option<String>,
    project_context_ref: Option<String>,
    runner: Option<String>,
    profile: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineWriteEffectV1 {
    effect_kind: String,
    portable_path: String,
    fingerprint: super::Fingerprint,
    revision: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineWriteConditionV1 {
    operation: ExactRef,
    effects: Vec<PipelineWriteEffectV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineEmptyRequestV1 {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCatalogReadRequestV1 {
    selector: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCompileRequestV1 {
    pipeline_selector: String,
    stage_selector: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCatalogEntryV1 {
    id: String,
    title: String,
    source_path: String,
    stage_count: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCatalogListResultV1 {
    pipelines: Vec<PipelineCatalogEntryV1>,
    pipeline_count: u64,
    stage_count: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PipelineActivationOperatorV1 {
    Any,
    All,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineActivationClauseV1 {
    variable: String,
    value: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineActivationV1 {
    operator: PipelineActivationOperatorV1,
    clauses: Vec<PipelineActivationClauseV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineDeclaredStageV1 {
    id: String,
    title: String,
    source_path: String,
    work_level: Option<String>,
    sets: Option<Vec<String>>,
    activation: Option<PipelineActivationV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineDefinitionV1 {
    id: String,
    title: String,
    description: String,
    source_path: String,
    default_runner: String,
    default_profile: String,
    default_enable_complexity: bool,
    stages: Vec<PipelineDeclaredStageV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineStageV1 {
    id: String,
    kind: String,
    version: String,
    title: String,
    description: String,
    work_level: Option<String>,
    source_path: String,
    pipeline_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "selection_kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum PipelineCatalogReadResultV1 {
    Pipeline { pipeline: PipelineDefinitionV1 },
    Stage { stage: PipelineStageV1 },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PipelineRouteStageStatusV1 {
    Active,
    Skipped,
    Blocked,
    Next,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "reason_kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum PipelineRouteStageReasonV1 {
    SkippedActivationFalse {
        unsatisfied_variables: Vec<String>,
    },
    NextMissingRouteVariables {
        missing_variables: Vec<String>,
    },
    BlockedByUnresolvedStage {
        upstream_stage_id: String,
        upstream_status: PipelineRouteStageStatusV1,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineRouteRunV1 {
    runner: Option<String>,
    profile: Option<String>,
    repository_root: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineRouteBasisRunnerV1 {
    id: String,
    file: String,
    file_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineRouteBasisProfileV1 {
    id: String,
    profile_yaml_sha256: String,
    commands_yaml_sha256: String,
    conventions_md_sha256: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineRouteBasisStageV1 {
    stage_id: String,
    file: String,
    status: PipelineRouteStageStatusV1,
    reason: Option<PipelineRouteStageReasonV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineRouteBasisV1 {
    schema_version: String,
    state_revision: u64,
    pipeline_file: String,
    pipeline_file_sha256: String,
    routing: Vec<PipelineRoutingEntryV1>,
    charter_ref: Option<String>,
    project_context_ref: Option<String>,
    run: PipelineRouteRunV1,
    runner: PipelineRouteBasisRunnerV1,
    profile: PipelineRouteBasisProfileV1,
    route: Vec<PipelineRouteBasisStageV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineRoutingEntryV1 {
    variable: String,
    value: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCompileTargetV1 {
    pipeline_id: String,
    stage_id: String,
    stage_file: String,
    title: String,
    description: String,
    work_level: String,
    tags: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCompileVariableV1 {
    name: String,
    value: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PipelineCompileDocumentKindV1 {
    Include,
    Runner,
    Profile,
    Library,
    Artifact,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PipelineCompileDocumentStatusV1 {
    Present,
    MissingOptional,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCompileDocumentV1 {
    kind: PipelineCompileDocumentKindV1,
    path: String,
    required: bool,
    status: PipelineCompileDocumentStatusV1,
    content: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PipelineCompileOutputKindV1 {
    Artifact,
    RepositoryFile,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCompileOutputV1 {
    kind: PipelineCompileOutputKindV1,
    path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCompileGatingV1 {
    mode: Option<String>,
    fail_on: Vec<String>,
    notes: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCompileResultV1 {
    target: PipelineCompileTargetV1,
    basis: PipelineRouteBasisV1,
    variables: Vec<PipelineCompileVariableV1>,
    documents: Vec<PipelineCompileDocumentV1>,
    outputs: Vec<PipelineCompileOutputV1>,
    gating: PipelineCompileGatingV1,
    stage_body: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PipelineCompileRefusalClassificationV1 {
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineCompileRefusalV1 {
    classification: PipelineCompileRefusalClassificationV1,
    summary: String,
    pipeline_id: Option<String>,
    stage_id: Option<String>,
    recovery: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum PipelineProblemCauseV1 {
    CatalogRead,
    CatalogInvalid,
    UnsupportedSelector,
    AmbiguousSelector,
    UnknownSelector,
    UnsupportedCompileTarget,
    InvalidCompileDefinition,
    InvalidRouteState,
    MissingRouteBasis,
    MalformedRouteBasis,
    StaleRouteBasis,
    InactiveStage,
    MissingRequiredInput,
    EmptyRequiredInput,
    CompileResultBoundExceeded,
    CaptureUnsupportedTarget,
    CaptureInvalidDefinition,
    CaptureInvalidState,
    CaptureMissingRouteBasis,
    CaptureMalformedRouteBasis,
    CaptureStaleRouteBasis,
    CaptureInactiveStage,
    CaptureInvalidInput,
    CaptureInvalidWriteTarget,
    CaptureMissingId,
    CaptureTamperedCache,
    CaptureRevisionConflict,
    CaptureWriteFailure,
    CaptureStatePersistenceFailure,
    CaptureCacheFailure,
    HandoffUnsupportedTarget,
    HandoffInvalidState,
    HandoffMissingInput,
    HandoffInvalidProvenance,
    HandoffWriteFailure,
    StateMalformed,
    StateUnsupportedVariable,
    StateRevisionConflict,
    StateInvalidPipelineId,
    StateInvalidSupportedVariables,
    StateInvalidMutation,
    StateReadFailure,
    StateLockFailure,
    StateWriteFailure,
    StateSerializationFailure,
    IdempotencyInvalidOperation,
    IdempotencyLockFailure,
    IdempotencyReadFailure,
    IdempotencyMalformedEntry,
    IdempotencyWriteFailure,
    IdempotencyConflict,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineMutationProblemDetailsV1 {
    pipeline_id: Option<String>,
    stage_id: Option<String>,
    consumer_id: Option<String>,
    recovery: Option<String>,
    expected_revision: Option<u64>,
    actual_revision: Option<u64>,
    established_request_fingerprint: Option<super::Fingerprint>,
    actual_request_fingerprint: Option<super::Fingerprint>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineProblemDetailsV1 {
    cause: PipelineProblemCauseV1,
    detail: String,
    selector: Option<String>,
    matches: Vec<String>,
    compile_refusal: Option<PipelineCompileRefusalV1>,
    mutation: Option<PipelineMutationProblemDetailsV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineRecheckV1 {
    cause: PipelineProblemCauseV1,
}

pub struct PipelineOperationSchemaValidator;

impl OperationSchemaValidator for PipelineOperationSchemaValidator {
    fn validate_instance(
        &self,
        schema: &ExactBinding,
        instance: &ContractValue,
    ) -> Result<(), ContractError> {
        let limits = ContractLimits::packet0();
        let bytes = canonical_json_bytes(instance, &limits)?;
        validate_checked_schema(schema, &bytes)?;
        match schema.reference().as_str() {
            LIST_REQUEST_REF => {
                parse_canonical_json::<PipelineEmptyRequestV1>(&bytes, &limits).map(|_| ())
            }
            LIST_RESULT_REF => {
                parse_canonical_json::<PipelineCatalogListResultV1>(&bytes, &limits).map(|_| ())
            }
            READ_REQUEST_REF => {
                parse_canonical_json::<PipelineCatalogReadRequestV1>(&bytes, &limits)
                    .and_then(|value| non_empty(&value.selector))
            }
            READ_RESULT_REF => {
                parse_canonical_json::<PipelineCatalogReadResultV1>(&bytes, &limits).map(|_| ())
            }
            COMPILE_REQUEST_REF => parse_canonical_json::<PipelineCompileRequestV1>(
                &bytes, &limits,
            )
            .and_then(|value| {
                non_empty(&value.pipeline_selector)?;
                non_empty(&value.stage_selector)
            }),
            COMPILE_RESULT_REF => {
                parse_canonical_json::<PipelineCompileResultV1>(&bytes, &limits).map(|_| ())
            }
            CAPTURE_PLAN_REQUEST_REF => parse_canonical_json::<PipelineCapturePlanRequestV1>(
                &bytes, &limits,
            )
            .and_then(|value| {
                non_empty(&value.pipeline_selector)?;
                non_empty(&value.stage_selector)
            }),
            CAPTURE_PLAN_RESULT_REF => {
                parse_canonical_json::<PipelineCapturePlanResultV1>(&bytes, &limits).map(|_| ())
            }
            CAPTURE_APPLY_REQUEST_REF => {
                parse_canonical_json::<PipelineCaptureApplyRequestSchemaV1>(&bytes, &limits)
                    .and_then(validate_capture_apply_request)
            }
            CAPTURE_APPLY_RESULT_REF => {
                parse_canonical_json::<PipelineCaptureApplyResultV1>(&bytes, &limits).map(|_| ())
            }
            HANDOFF_EMIT_REQUEST_REF => {
                parse_canonical_json::<PipelineHandoffEmitRequestSchemaV1>(&bytes, &limits)
                    .and_then(|value| {
                        non_empty(&value.pipeline_selector)?;
                        non_empty(&value.consumer_selector)?;
                        non_empty(&value.producer_command)?;
                        non_empty(&value.producer_version)
                    })
            }
            HANDOFF_EMIT_RESULT_REF => {
                parse_canonical_json::<PipelineHandoffEmitResultV1>(&bytes, &limits).map(|_| ())
            }
            STATE_APPLY_REQUEST_REF => {
                parse_canonical_json::<PipelineStateApplyRequestSchemaV1>(&bytes, &limits)
                    .and_then(validate_state_apply_request)
            }
            STATE_APPLY_RESULT_REF => {
                parse_canonical_json::<PipelineStateApplyResultV1>(&bytes, &limits).map(|_| ())
            }
            PIPELINE_WRITE_CONDITION_REF => parse_canonical_json::<PipelineWriteConditionV1>(
                &bytes, &limits,
            )
            .and_then(|value| {
                if !matches!(
                    value.operation.as_str(),
                    PIPELINE_CAPTURE_APPLY_REF
                        | PIPELINE_HANDOFF_EMIT_REF
                        | PIPELINE_STATE_APPLY_REF
                ) || value.effects.is_empty()
                {
                    return Err(ContractError::InvalidInvariant);
                }
                for effect in value.effects {
                    non_empty(&effect.effect_kind)?;
                    non_empty(&effect.portable_path)?;
                }
                Ok(())
            }),
            PIPELINE_DETAILS_REF => {
                parse_canonical_json::<PipelineProblemDetailsV1>(&bytes, &limits)
                    .and_then(|details| details.validate())
            }
            PIPELINE_RECHECK_REF => parse_canonical_json::<PipelineRecheckV1>(&bytes, &limits)
                .and_then(|recheck| {
                    if problem_projection(recheck.cause).0 == TerminalStatus::Blocked {
                        Ok(())
                    } else {
                        Err(ContractError::InvalidInvariant)
                    }
                }),
            _ => Err(ContractError::InvalidInvariant),
        }
    }

    fn validate_problem(
        &self,
        status: TerminalStatus,
        problem: &Problem,
    ) -> Result<(), ContractError> {
        self.validate_instance(problem.details_schema(), problem.details())?;
        let details: PipelineProblemDetailsV1 = from_contract_value(problem.details())?;
        let (expected_status, expected_category, expected_retry) =
            problem_projection(details.cause);
        if status != expected_status
            || problem.category() != expected_category
            || problem.retry() != expected_retry
            || !problem.evidence().as_slice().is_empty()
            || problem.recheck().is_some() != (status == TerminalStatus::Blocked)
            || problem.rule().is_some() != (status == TerminalStatus::Refused)
            || problem.correlation_ref().is_some() != (status == TerminalStatus::Error)
        {
            return Err(ContractError::InvalidInvariant);
        }
        if let Some(recheck) = problem.recheck() {
            self.validate_instance(recheck.condition_schema(), recheck.condition())?;
            let recheck: PipelineRecheckV1 = from_contract_value(recheck.condition())?;
            if recheck.cause != details.cause {
                return Err(ContractError::InvalidInvariant);
            }
        }
        Ok(())
    }
}

fn validate_capture_apply_request(
    value: PipelineCaptureApplyRequestSchemaV1,
) -> Result<(), ContractError> {
    non_empty(&value.idempotency_key)?;
    match value.mode {
        PipelineCaptureApplyModeV1::Direct => {
            if value.capture_id.is_some() {
                return Err(ContractError::InvalidInvariant);
            }
            non_empty(
                value
                    .pipeline_selector
                    .as_deref()
                    .ok_or(ContractError::InvalidInvariant)?,
            )?;
            non_empty(
                value
                    .stage_selector
                    .as_deref()
                    .ok_or(ContractError::InvalidInvariant)?,
            )?;
            value
                .input
                .as_deref()
                .ok_or(ContractError::InvalidInvariant)?;
            Ok(())
        }
        PipelineCaptureApplyModeV1::Cached => {
            if value.pipeline_selector.is_some()
                || value.stage_selector.is_some()
                || value.input.is_some()
            {
                return Err(ContractError::InvalidInvariant);
            }
            non_empty(
                value
                    .capture_id
                    .as_deref()
                    .ok_or(ContractError::InvalidInvariant)?,
            )
        }
    }
}

fn validate_state_apply_request(
    value: PipelineStateApplyRequestSchemaV1,
) -> Result<(), ContractError> {
    non_empty(&value.idempotency_key)?;
    non_empty(&value.selector)?;
    match value.mutation {
        PipelineStateMutationV1::RoutingVariable { variable, .. } => non_empty(&variable),
        PipelineStateMutationV1::CharterReference { value }
        | PipelineStateMutationV1::ProjectContextReference { value }
        | PipelineStateMutationV1::Runner { value }
        | PipelineStateMutationV1::Profile { value } => non_empty(&value),
    }
}

impl PipelineProblemDetailsV1 {
    fn validate(&self) -> Result<(), ContractError> {
        let compile_cause = is_compile_cause(self.cause);
        let mutation_cause = is_mutation_cause(self.cause);
        if compile_cause != self.compile_refusal.is_some()
            || mutation_cause != self.mutation.is_some()
        {
            return Err(ContractError::InvalidInvariant);
        }
        if let Some(mutation) = &self.mutation {
            let revision_conflict = matches!(
                self.cause,
                PipelineProblemCauseV1::CaptureRevisionConflict
                    | PipelineProblemCauseV1::StateRevisionConflict
            );
            if revision_conflict
                != (mutation.expected_revision.is_some() && mutation.actual_revision.is_some())
            {
                return Err(ContractError::InvalidInvariant);
            }
            let idempotency_conflict = self.cause == PipelineProblemCauseV1::IdempotencyConflict;
            if idempotency_conflict
                != (mutation.established_request_fingerprint.is_some()
                    && mutation.actual_request_fingerprint.is_some())
            {
                return Err(ContractError::InvalidInvariant);
            }
        }
        match self.cause {
            PipelineProblemCauseV1::UnsupportedSelector
            | PipelineProblemCauseV1::UnknownSelector => {
                if self.selector.is_none() || !self.matches.is_empty() {
                    return Err(ContractError::InvalidInvariant);
                }
            }
            PipelineProblemCauseV1::AmbiguousSelector => {
                if self.selector.is_none() || self.matches.is_empty() {
                    return Err(ContractError::InvalidInvariant);
                }
            }
            _ if self.selector.is_some() || !self.matches.is_empty() => {
                return Err(ContractError::InvalidInvariant)
            }
            _ => {}
        }
        Ok(())
    }
}

pub fn pipeline_operation_definitions() -> Result<Vec<OperationDefinition>, ContractError> {
    [
        PIPELINE_LIST_REF,
        PIPELINE_READ_REF,
        PIPELINE_COMPILE_REF,
        PIPELINE_CAPTURE_PLAN_REF,
        PIPELINE_CAPTURE_APPLY_REF,
        PIPELINE_HANDOFF_EMIT_REF,
        PIPELINE_STATE_APPLY_REF,
    ]
    .into_iter()
    .map(operation_definition)
    .collect()
}

pub fn pipeline_catalog_list_operation_request(
    request_id: Option<&str>,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(request_id, PIPELINE_LIST_REF, &PipelineEmptyRequestV1 {})
}

pub fn pipeline_catalog_read_operation_request(
    request_id: Option<&str>,
    selector: &str,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        request_id,
        PIPELINE_READ_REF,
        &PipelineCatalogReadRequestV1 {
            selector: selector.to_owned(),
        },
    )
}

pub fn pipeline_compile_operation_request(
    request_id: Option<&str>,
    pipeline_selector: &str,
    stage_selector: &str,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        request_id,
        PIPELINE_COMPILE_REF,
        &PipelineCompileRequestV1 {
            pipeline_selector: pipeline_selector.to_owned(),
            stage_selector: stage_selector.to_owned(),
        },
    )
}

pub fn pipeline_capture_plan_operation_request(
    request_id: Option<&str>,
    pipeline_selector: &str,
    stage_selector: &str,
    input: &str,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        request_id,
        PIPELINE_CAPTURE_PLAN_REF,
        &PipelineCapturePlanRequestV1 {
            pipeline_selector: pipeline_selector.to_owned(),
            stage_selector: stage_selector.to_owned(),
            input: input.to_owned(),
        },
    )
}

pub fn pipeline_capture_direct_operation_request(
    request_id: Option<&str>,
    pipeline_selector: &str,
    stage_selector: &str,
    input: &str,
    idempotency_key: &str,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        request_id,
        PIPELINE_CAPTURE_APPLY_REF,
        &PipelineCaptureApplyRequestSchemaV1 {
            idempotency_key: idempotency_key.to_owned(),
            mode: PipelineCaptureApplyModeV1::Direct,
            capture_id: None,
            pipeline_selector: Some(pipeline_selector.to_owned()),
            stage_selector: Some(stage_selector.to_owned()),
            input: Some(input.to_owned()),
        },
    )
}

pub fn pipeline_capture_apply_operation_request(
    request_id: Option<&str>,
    capture_id: &str,
    idempotency_key: &str,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        request_id,
        PIPELINE_CAPTURE_APPLY_REF,
        &PipelineCaptureApplyRequestSchemaV1 {
            idempotency_key: idempotency_key.to_owned(),
            mode: PipelineCaptureApplyModeV1::Cached,
            capture_id: Some(capture_id.to_owned()),
            pipeline_selector: None,
            stage_selector: None,
            input: None,
        },
    )
}

pub fn pipeline_handoff_emit_operation_request(
    request_id: Option<&str>,
    pipeline_selector: &str,
    consumer_selector: &str,
    producer_command: &str,
    producer_version: &str,
    idempotency_key: &str,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        request_id,
        PIPELINE_HANDOFF_EMIT_REF,
        &PipelineHandoffEmitRequestSchemaV1 {
            idempotency_key: idempotency_key.to_owned(),
            pipeline_selector: pipeline_selector.to_owned(),
            consumer_selector: consumer_selector.to_owned(),
            producer_command: producer_command.to_owned(),
            producer_version: producer_version.to_owned(),
        },
    )
}

pub fn pipeline_state_apply_operation_request(
    request_id: Option<&str>,
    selector: &str,
    mutation: &crate::pipeline_route_api::PipelineStateMutation,
    expected_revision: u64,
    idempotency_key: &str,
) -> Result<OperationRequestV1, ContractError> {
    let mutation = match mutation {
        crate::pipeline_route_api::PipelineStateMutation::RoutingVariable { variable, value } => {
            PipelineStateMutationV1::RoutingVariable {
                variable: variable.clone(),
                value: *value,
            }
        }
        crate::pipeline_route_api::PipelineStateMutation::CharterReference { value } => {
            PipelineStateMutationV1::CharterReference {
                value: value.clone(),
            }
        }
        crate::pipeline_route_api::PipelineStateMutation::ProjectContextReference { value } => {
            PipelineStateMutationV1::ProjectContextReference {
                value: value.clone(),
            }
        }
        crate::pipeline_route_api::PipelineStateMutation::Runner { value } => {
            PipelineStateMutationV1::Runner {
                value: value.clone(),
            }
        }
        crate::pipeline_route_api::PipelineStateMutation::Profile { value } => {
            PipelineStateMutationV1::Profile {
                value: value.clone(),
            }
        }
    };
    operation_request(
        request_id,
        PIPELINE_STATE_APPLY_REF,
        &PipelineStateApplyRequestSchemaV1 {
            idempotency_key: idempotency_key.to_owned(),
            selector: selector.to_owned(),
            mutation,
            expected_revision,
        },
    )
}

fn operation_request<T: Serialize>(
    request_id: Option<&str>,
    operation_ref: &str,
    body: &T,
) -> Result<OperationRequestV1, ContractError> {
    let definition = operation_definition(operation_ref)?;
    OperationRequestV1::new(
        schema_binding("handbook.operation-request@1.0.0")?,
        request_id.map(RequestId::parse).transpose()?,
        &definition,
        to_contract_value(body)?,
        &PipelineOperationSchemaValidator,
        &ContractLimits::packet0(),
    )
}

fn operation_definition(operation_ref: &str) -> Result<OperationDefinition, ContractError> {
    let (request_ref, result_ref) = match operation_ref {
        PIPELINE_LIST_REF => (LIST_REQUEST_REF, LIST_RESULT_REF),
        PIPELINE_READ_REF => (READ_REQUEST_REF, READ_RESULT_REF),
        PIPELINE_COMPILE_REF => (COMPILE_REQUEST_REF, COMPILE_RESULT_REF),
        PIPELINE_CAPTURE_PLAN_REF => (CAPTURE_PLAN_REQUEST_REF, CAPTURE_PLAN_RESULT_REF),
        PIPELINE_CAPTURE_APPLY_REF => (CAPTURE_APPLY_REQUEST_REF, CAPTURE_APPLY_RESULT_REF),
        PIPELINE_HANDOFF_EMIT_REF => (HANDOFF_EMIT_REQUEST_REF, HANDOFF_EMIT_RESULT_REF),
        PIPELINE_STATE_APPLY_REF => (STATE_APPLY_REQUEST_REF, STATE_APPLY_RESULT_REF),
        _ => return Err(ContractError::InvalidInvariant),
    };
    let limits = ContractLimits::packet0();
    let owner = handbook_pipeline::operation_admission::operation_owner_version_evidence();
    OperationDefinition::new(
        schema_binding("handbook.operation-definition@1.0.0")?,
        ExactRef::parse(operation_ref)?,
        OperationOwnerBinding::new(
            BoundedString::new(owner.owner_crate.to_owned())?,
            SemVer::parse(owner.owner_version)?,
            ExactBinding::new(
                ExactRef::parse(owner.owner_source_ref)?,
                super::sha256_fingerprint(owner.source_bytes),
            ),
        )?,
        schema_binding(request_ref)?,
        schema_binding(result_ref)?,
        OperationOutcomeSchemas::new(
            schema_binding("handbook.operation-blocker@1.0.0")?,
            schema_binding("handbook.operation-refusal@1.0.0")?,
            schema_binding("handbook.operation-error@1.0.0")?,
        )?,
        BoundedVec::new(Vec::new())?,
        operation_class(operation_ref),
        operation_idempotency(operation_ref)?,
        operation_write_set(operation_ref)?,
        BoundedVec::new(vec![
            OperationTransport::CliJson,
            OperationTransport::RustSdk,
            OperationTransport::Tauri,
        ])?,
        None,
        &limits,
    )
}

fn operation_class(operation_ref: &str) -> OperationClass {
    match operation_ref {
        PIPELINE_CAPTURE_APPLY_REF | PIPELINE_STATE_APPLY_REF => OperationClass::CompareAndWrite,
        PIPELINE_HANDOFF_EMIT_REF => OperationClass::AppendOnly,
        _ => OperationClass::ReadOnly,
    }
}

fn operation_idempotency(operation_ref: &str) -> Result<OperationIdempotency, ContractError> {
    let key_pointer = || JsonPointer::parse("/body/idempotency_key");
    match operation_ref {
        PIPELINE_CAPTURE_APPLY_REF => OperationIdempotency::compare_and_write(
            key_pointer()?,
            BoundedVec::new(
                [
                    "/body/capture_id",
                    "/body/input",
                    "/body/mode",
                    "/body/pipeline_selector",
                    "/body/stage_selector",
                ]
                .into_iter()
                .map(JsonPointer::parse)
                .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        PIPELINE_HANDOFF_EMIT_REF => OperationIdempotency::key_required(key_pointer()?),
        PIPELINE_STATE_APPLY_REF => OperationIdempotency::compare_and_write(
            key_pointer()?,
            BoundedVec::new(
                [
                    "/body/expected_revision",
                    "/body/mutation",
                    "/body/selector",
                ]
                .into_iter()
                .map(JsonPointer::parse)
                .collect::<Result<Vec<_>, _>>()?,
            )?,
        ),
        _ => Ok(OperationIdempotency::safe()),
    }
}

fn operation_write_set(
    operation_ref: &str,
) -> Result<BoundedVec<DeclaredWrite, 128>, ContractError> {
    let (record_kind, atomic_group) = match operation_ref {
        PIPELINE_CAPTURE_APPLY_REF => ("pipeline_capture", "pipeline_capture_apply"),
        PIPELINE_HANDOFF_EMIT_REF => ("pipeline_handoff", "pipeline_handoff_emit"),
        PIPELINE_STATE_APPLY_REF => ("pipeline_route_state", "pipeline_state_apply"),
        _ => return BoundedVec::new(Vec::new()),
    };
    BoundedVec::new(vec![DeclaredWrite::new(
        BoundedString::new(record_kind.to_owned())?,
        BoundedString::new("pipeline_owner".to_owned())?,
        BoundedString::new(atomic_group.to_owned())?,
        schema_binding(PIPELINE_WRITE_CONDITION_REF)?,
    )?])
}

pub(crate) fn execute_pipeline_operation(
    repo_root: &Path,
    request: &OperationRequestV1,
) -> Result<OperationResponseV1, ContractError> {
    let definition = operation_definition(request.operation().reference().as_str())?;
    if request.operation() != &definition.binding()
        || request.body_schema() != definition.request_schema()
    {
        return Err(ContractError::InvalidInvariant);
    }
    match super::admit_operation_definition(&definition)? {
        OperationAdmissionResult::Admitted(owner)
            if owner.operation_ref() == request.operation().reference().as_str()
                && owner.definition_fingerprint()
                    == definition.definition_fingerprint().as_str() => {}
        _ => return Err(ContractError::InvalidInvariant),
    }
    match request.operation().reference().as_str() {
        PIPELINE_LIST_REF => execute_list(repo_root, request, &definition),
        PIPELINE_READ_REF => execute_read(repo_root, request, &definition),
        PIPELINE_COMPILE_REF => execute_compile(repo_root, request, &definition),
        PIPELINE_CAPTURE_PLAN_REF => execute_capture_plan(repo_root, request, &definition),
        PIPELINE_CAPTURE_APPLY_REF => execute_capture_apply(repo_root, request, &definition),
        PIPELINE_HANDOFF_EMIT_REF => execute_handoff_emit(repo_root, request, &definition),
        PIPELINE_STATE_APPLY_REF => execute_state_apply(repo_root, request, &definition),
        _ => Err(ContractError::InvalidInvariant),
    }
}

fn execute_list(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
) -> Result<OperationResponseV1, ContractError> {
    let _: PipelineEmptyRequestV1 = from_contract_value(request.body())?;
    match load_pipeline_catalog_metadata(repo_root) {
        Ok(catalog) => {
            let data = PipelineCatalogListResultV1 {
                pipeline_count: to_u64(catalog.pipeline_count())?,
                stage_count: to_u64(catalog.stage_count())?,
                pipelines: catalog
                    .pipelines()
                    .map(|pipeline| {
                        Ok(PipelineCatalogEntryV1 {
                            id: pipeline.definition.header.id.clone(),
                            title: pipeline.definition.header.title.clone(),
                            source_path: portable_path(&pipeline.definition.source_path),
                            stage_count: to_u64(pipeline.stages.len())?,
                        })
                    })
                    .collect::<Result<Vec<_>, ContractError>>()?,
            };
            ok_response(request, definition, &data)
        }
        Err(error) => catalog_error_response(repo_root, request, definition, &error),
    }
}

fn execute_read(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
) -> Result<OperationResponseV1, ContractError> {
    let body: PipelineCatalogReadRequestV1 = from_contract_value(request.body())?;
    match load_pipeline_selection_metadata(repo_root, &body.selector) {
        Ok(PipelineSelection::Pipeline(pipeline)) => {
            let data = PipelineCatalogReadResultV1::Pipeline {
                pipeline: PipelineDefinitionV1 {
                    id: pipeline.definition.header.id.clone(),
                    title: pipeline.definition.header.title.clone(),
                    description: pipeline.definition.header.description.clone(),
                    source_path: portable_path(&pipeline.definition.source_path),
                    default_runner: pipeline.definition.body.defaults.runner.clone(),
                    default_profile: pipeline.definition.body.defaults.profile.clone(),
                    default_enable_complexity: pipeline.definition.body.defaults.enable_complexity,
                    stages: pipeline
                        .stages
                        .iter()
                        .zip(pipeline.definition.declared_stages())
                        .map(|(stage, declared)| PipelineDeclaredStageV1 {
                            id: stage.stage_id.clone(),
                            title: stage.title.clone(),
                            source_path: portable_path(&stage.source_path),
                            work_level: stage.work_level.clone(),
                            sets: declared.sets.clone(),
                            activation: declared.activation.as_ref().map(|activation| {
                                PipelineActivationV1 {
                                    operator: match activation.when.operator {
                                        handbook_pipeline::pipeline::ActivationOperator::Any => {
                                            PipelineActivationOperatorV1::Any
                                        }
                                        handbook_pipeline::pipeline::ActivationOperator::All => {
                                            PipelineActivationOperatorV1::All
                                        }
                                    },
                                    clauses: {
                                        let mut clauses = activation
                                            .when
                                            .clauses
                                            .iter()
                                            .map(|clause| PipelineActivationClauseV1 {
                                                variable: clause.variable.clone(),
                                                value: clause.value,
                                            })
                                            .collect::<Vec<_>>();
                                        clauses.sort_by(|a, b| a.variable.cmp(&b.variable));
                                        clauses
                                    },
                                }
                            }),
                        })
                        .collect(),
                },
            };
            ok_response(request, definition, &data)
        }
        Ok(PipelineSelection::Stage(stage)) => ok_response(
            request,
            definition,
            &PipelineCatalogReadResultV1::Stage {
                stage: PipelineStageV1 {
                    id: stage.id,
                    kind: stage.kind,
                    version: stage.version,
                    title: stage.title,
                    description: stage.description,
                    work_level: stage.work_level,
                    source_path: portable_path(&stage.source_path),
                    pipeline_ids: stage.pipelines,
                },
            },
        ),
        Err(PipelineMetadataSelectionError::Catalog(error)) => {
            catalog_error_response(repo_root, request, definition, &error)
        }
        Err(PipelineMetadataSelectionError::Lookup(error)) => {
            selector_error_response(request, definition, error)
        }
    }
}

fn execute_compile(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
) -> Result<OperationResponseV1, ContractError> {
    let body: PipelineCompileRequestV1 = from_contract_value(request.body())?;
    match compile_pipeline_stage(repo_root, &body.pipeline_selector, &body.stage_selector) {
        Ok(result) => {
            let bound_refusal = PipelineCompileRefusal::result_contract_bound_exceeded(
                &result.target.pipeline_id,
                &result.target.stage_id,
            );
            let projected = PipelineCompileResultV1::from_owner(result);
            match to_contract_value(&projected) {
                Ok(projected) => ok_projected_response(request, definition, projected),
                Err(ContractError::BoundExceeded) => compile_problem_response(
                    repo_root,
                    request,
                    definition,
                    bound_refusal,
                    Some(PipelineProblemCauseV1::CompileResultBoundExceeded),
                ),
                Err(error) => Err(error),
            }
        }
        Err(refusal) => compile_problem_response(repo_root, request, definition, refusal, None),
    }
}

enum PipelineJournaledOutcome<T, E> {
    Applied {
        execution: T,
        replayed: bool,
    },
    OwnerFailure(E),
    Conflict {
        established_request_fingerprint: String,
        actual_request_fingerprint: String,
    },
    JournalFailure {
        error: PipelineMutationJournalError,
        effects: Vec<PipelineMutationEffect>,
    },
}

fn execute_journaled<T, E>(
    repo_root: &Path,
    operation_ref: &str,
    raw_key: &[u8],
    request_fingerprint: &str,
    execute: impl FnOnce() -> Result<T, E>,
    effects: impl Fn(&T) -> Vec<PipelineMutationEffect>,
) -> PipelineJournaledOutcome<T, E>
where
    T: serde::de::DeserializeOwned + Serialize,
{
    let journal = match PipelineMutationJournalSession::open(repo_root, operation_ref, raw_key) {
        Ok(journal) => journal,
        Err(error) => {
            return PipelineJournaledOutcome::JournalFailure {
                error,
                effects: Vec::new(),
            }
        }
    };
    match journal.lookup::<T>(request_fingerprint) {
        Ok(PipelineMutationJournalLookup::Replay(execution)) => PipelineJournaledOutcome::Applied {
            execution,
            replayed: true,
        },
        Ok(PipelineMutationJournalLookup::Conflict {
            established_request_fingerprint,
            actual_request_fingerprint,
        }) => PipelineJournaledOutcome::Conflict {
            established_request_fingerprint,
            actual_request_fingerprint,
        },
        Err(error) => PipelineJournaledOutcome::JournalFailure {
            error,
            effects: Vec::new(),
        },
        Ok(PipelineMutationJournalLookup::Miss) => match execute() {
            Err(error) => PipelineJournaledOutcome::OwnerFailure(error),
            Ok(execution) => match journal.persist(request_fingerprint, &execution) {
                Ok(()) => PipelineJournaledOutcome::Applied {
                    execution,
                    replayed: false,
                },
                Err(error) => PipelineJournaledOutcome::JournalFailure {
                    effects: effects(&execution),
                    error,
                },
            },
        },
    }
}

struct CaptureMutationOwnerBridge<'a> {
    repo_root: &'a Path,
    request_fingerprint: &'a str,
    body: &'a PipelineCaptureApplyRequestV1,
    pointer_observed: bool,
    result: Option<
        PipelineJournaledOutcome<PipelineCaptureApplyExecution, PipelineCaptureApplyFailure>,
    >,
}

impl IdempotencyKeyConsumerSealed for CaptureMutationOwnerBridge<'_> {
    fn consume_owner_idempotency_key(&mut self, pointer: &str, bytes: &[u8]) -> super::Fingerprint {
        self.pointer_observed = pointer == "/body/idempotency_key";
        self.result = Some(execute_journaled(
            self.repo_root,
            PIPELINE_CAPTURE_APPLY_REF,
            bytes,
            self.request_fingerprint,
            || match self.body.mode {
                PipelineCaptureApplyModeV1::Direct => capture_pipeline_output_with_effects(
                    self.repo_root,
                    &PipelineCaptureRequest {
                        pipeline_selector: self
                            .body
                            .pipeline_selector
                            .clone()
                            .expect("admitted direct capture selector"),
                        stage_selector: self
                            .body
                            .stage_selector
                            .clone()
                            .expect("admitted direct capture stage"),
                        input: self
                            .body
                            .input
                            .clone()
                            .expect("admitted direct capture input"),
                    },
                ),
                PipelineCaptureApplyModeV1::Cached => apply_pipeline_capture_with_effects(
                    self.repo_root,
                    self.body
                        .capture_id
                        .as_deref()
                        .expect("admitted cached capture id"),
                ),
            },
            |execution| execution.effects.clone(),
        ));
        super::sha256_fingerprint(bytes)
    }
}
impl IdempotencyKeyConsumer for CaptureMutationOwnerBridge<'_> {}

struct HandoffMutationOwnerBridge<'a> {
    repo_root: &'a Path,
    request_fingerprint: &'a str,
    body: &'a PipelineHandoffEmitRequestV1,
    pointer_observed: bool,
    result:
        Option<PipelineJournaledOutcome<PipelineHandoffEmitExecution, PipelineHandoffEmitFailure>>,
}

impl IdempotencyKeyConsumerSealed for HandoffMutationOwnerBridge<'_> {
    fn consume_owner_idempotency_key(&mut self, pointer: &str, bytes: &[u8]) -> super::Fingerprint {
        self.pointer_observed = pointer == "/body/idempotency_key";
        self.result = Some(execute_journaled(
            self.repo_root,
            PIPELINE_HANDOFF_EMIT_REF,
            bytes,
            self.request_fingerprint,
            || {
                emit_pipeline_handoff_bundle_with_effects(
                    self.repo_root,
                    &PipelineHandoffEmitRequest {
                        pipeline_selector: self.body.pipeline_selector.clone(),
                        consumer_selector: self.body.consumer_selector.clone(),
                        producer_command: self.body.producer_command.clone(),
                        producer_version: self.body.producer_version.clone(),
                    },
                )
            },
            |execution| execution.effects.clone(),
        ));
        super::sha256_fingerprint(bytes)
    }
}
impl IdempotencyKeyConsumer for HandoffMutationOwnerBridge<'_> {}

enum StateMutationOwnerFailure {
    Definition(SelectedPipelineLoadError),
    Refusal {
        pipeline_id: String,
        refusal: RouteStateMutationRefusal,
    },
    Error {
        pipeline_id: String,
        failure: RouteStateMutationFailure,
    },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PipelineStateApplyExecution {
    pipeline_id: String,
    execution: RouteStateMutationExecution,
}

struct StateMutationOwnerBridge<'a> {
    repo_root: &'a Path,
    request_fingerprint: &'a str,
    selector: &'a str,
    mutation: &'a RouteStateMutation,
    expected_revision: u64,
    pointer_observed: bool,
    result:
        Option<PipelineJournaledOutcome<PipelineStateApplyExecution, StateMutationOwnerFailure>>,
}

impl IdempotencyKeyConsumerSealed for StateMutationOwnerBridge<'_> {
    fn consume_owner_idempotency_key(&mut self, pointer: &str, bytes: &[u8]) -> super::Fingerprint {
        self.pointer_observed = pointer == "/body/idempotency_key";
        self.result = Some(execute_journaled(
            self.repo_root,
            PIPELINE_STATE_APPLY_REF,
            bytes,
            self.request_fingerprint,
            || {
                let pipeline = load_selected_pipeline_definition(self.repo_root, self.selector)
                    .map_err(StateMutationOwnerFailure::Definition)?;
                let pipeline_id = pipeline.header.id.clone();
                let supported_variables = supported_route_state_variables(&pipeline);
                match set_route_state_with_effects(
                    self.repo_root,
                    &pipeline_id,
                    supported_variables.iter().map(String::as_str),
                    self.mutation.clone(),
                    self.expected_revision,
                ) {
                    Ok(execution) => match &execution.outcome {
                        RouteStateMutationOutcome::Applied(_) => Ok(PipelineStateApplyExecution {
                            pipeline_id,
                            execution,
                        }),
                        RouteStateMutationOutcome::Refused(refusal) => {
                            Err(StateMutationOwnerFailure::Refusal {
                                pipeline_id,
                                refusal: refusal.clone(),
                            })
                        }
                    },
                    Err(failure) => Err(StateMutationOwnerFailure::Error {
                        pipeline_id,
                        failure,
                    }),
                }
            },
            |execution| execution.execution.effects.clone(),
        ));
        super::sha256_fingerprint(bytes)
    }
}
impl IdempotencyKeyConsumer for StateMutationOwnerBridge<'_> {}

fn execute_capture_plan(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
) -> Result<OperationResponseV1, ContractError> {
    let body: PipelineCapturePlanRequestV1 = from_contract_value(request.body())?;
    match preview_pipeline_capture(
        repo_root,
        &PipelineCaptureRequest {
            pipeline_selector: body.pipeline_selector,
            stage_selector: body.stage_selector,
            input: body.input,
        },
    ) {
        Ok(preview) => ok_response(
            request,
            definition,
            &PipelineCapturePlanResultV1 {
                plan: capture_plan_dto(&preview.plan),
            },
        ),
        Err(refusal) => {
            capture_refusal_response(repo_root, request, definition, refusal, &[], None, None)
        }
    }
}

fn execute_capture_apply(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
) -> Result<OperationResponseV1, ContractError> {
    let body: PipelineCaptureApplyRequestV1 = from_contract_value(request.body())?;
    let mut bridge = CaptureMutationOwnerBridge {
        repo_root,
        request_fingerprint: request.request_fingerprint().as_str(),
        body: &body,
        pointer_observed: false,
        result: None,
    };
    request
        .with_owner_idempotency_key(&mut bridge)
        .ok_or(ContractError::InvalidInvariant)?;
    if !bridge.pointer_observed {
        return Err(ContractError::InvalidInvariant);
    }
    match bridge.result.ok_or(ContractError::InvalidInvariant)? {
        PipelineJournaledOutcome::Applied {
            execution,
            replayed,
        } => {
            let data = PipelineCaptureApplyResultV1 {
                plan: capture_plan_dto(&execution.result.plan),
                written_files: execution.result.written_files,
                persisted_state_revision: execution.result.persisted_state_revision,
            };
            let terminal = mutation_ok_terminal(definition, &data)?;
            established_mutation_response(
                request,
                definition,
                terminal,
                &execution.effects,
                replayed,
            )
        }
        PipelineJournaledOutcome::OwnerFailure(failure) => capture_refusal_response(
            repo_root,
            request,
            definition,
            *failure.refusal,
            &failure.effects,
            failure.terminal_facts.expected_revision,
            failure.terminal_facts.actual_revision,
        ),
        PipelineJournaledOutcome::Conflict {
            established_request_fingerprint,
            actual_request_fingerprint,
        } => idempotency_conflict_response(
            request,
            definition,
            established_request_fingerprint,
            actual_request_fingerprint,
        ),
        PipelineJournaledOutcome::JournalFailure { error, effects } => {
            journal_failure_response(request, definition, error, &effects)
        }
    }
}

fn execute_handoff_emit(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
) -> Result<OperationResponseV1, ContractError> {
    let body: PipelineHandoffEmitRequestV1 = from_contract_value(request.body())?;
    let mut bridge = HandoffMutationOwnerBridge {
        repo_root,
        request_fingerprint: request.request_fingerprint().as_str(),
        body: &body,
        pointer_observed: false,
        result: None,
    };
    request
        .with_owner_idempotency_key(&mut bridge)
        .ok_or(ContractError::InvalidInvariant)?;
    if !bridge.pointer_observed {
        return Err(ContractError::InvalidInvariant);
    }
    match bridge.result.ok_or(ContractError::InvalidInvariant)? {
        PipelineJournaledOutcome::Applied {
            execution,
            replayed,
        } => {
            let data = PipelineHandoffEmitResultV1 {
                pipeline_id: execution.result.manifest.pipeline_id,
                consumer_id: execution.result.manifest.consumer_id,
                feature_id: execution.result.manifest.feature_id,
                bundle_root: execution.result.bundle_root,
                written_files: execution.result.written_files,
            };
            let terminal = mutation_ok_terminal(definition, &data)?;
            established_mutation_response(
                request,
                definition,
                terminal,
                &execution.effects,
                replayed,
            )
        }
        PipelineJournaledOutcome::OwnerFailure(failure) => handoff_refusal_response(
            repo_root,
            request,
            definition,
            *failure.refusal,
            &failure.effects,
        ),
        PipelineJournaledOutcome::Conflict {
            established_request_fingerprint,
            actual_request_fingerprint,
        } => idempotency_conflict_response(
            request,
            definition,
            established_request_fingerprint,
            actual_request_fingerprint,
        ),
        PipelineJournaledOutcome::JournalFailure { error, effects } => {
            journal_failure_response(request, definition, error, &effects)
        }
    }
}

fn execute_state_apply(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
) -> Result<OperationResponseV1, ContractError> {
    let body: PipelineStateApplyRequestV1 = from_contract_value(request.body())?;
    let mutation = route_state_mutation(body.mutation);
    let mut bridge = StateMutationOwnerBridge {
        repo_root,
        request_fingerprint: request.request_fingerprint().as_str(),
        selector: &body.selector,
        mutation: &mutation,
        expected_revision: body.expected_revision,
        pointer_observed: false,
        result: None,
    };
    request
        .with_owner_idempotency_key(&mut bridge)
        .ok_or(ContractError::InvalidInvariant)?;
    if !bridge.pointer_observed {
        return Err(ContractError::InvalidInvariant);
    }
    match bridge.result.ok_or(ContractError::InvalidInvariant)? {
        PipelineJournaledOutcome::Applied {
            execution,
            replayed,
        } => {
            let RouteStateMutationOutcome::Applied(state) = execution.execution.outcome else {
                return Err(ContractError::InvalidInvariant);
            };
            let data = PipelineStateApplyResultV1 {
                pipeline_id: execution.pipeline_id,
                state: route_state_dto(&state),
            };
            let terminal = mutation_ok_terminal(definition, &data)?;
            established_mutation_response(
                request,
                definition,
                terminal,
                &execution.execution.effects,
                replayed,
            )
        }
        PipelineJournaledOutcome::OwnerFailure(StateMutationOwnerFailure::Definition(error)) => {
            match error {
                SelectedPipelineLoadError::Catalog(error) => {
                    catalog_error_response(repo_root, request, definition, &error)
                }
                SelectedPipelineLoadError::Lookup(error) => {
                    selector_error_response(request, definition, error)
                }
                SelectedPipelineLoadError::Load(error) => problem_response(
                    request,
                    definition,
                    PipelineProblemDetailsV1 {
                        cause: PipelineProblemCauseV1::CatalogInvalid,
                        detail: sanitize_detail(repo_root, &error.to_string()),
                        selector: None,
                        matches: Vec::new(),
                        compile_refusal: None,
                        mutation: None,
                    },
                ),
            }
        }
        PipelineJournaledOutcome::OwnerFailure(StateMutationOwnerFailure::Refusal {
            pipeline_id,
            refusal,
        }) => state_refusal_response(request, definition, &pipeline_id, refusal),
        PipelineJournaledOutcome::OwnerFailure(StateMutationOwnerFailure::Error {
            pipeline_id,
            failure,
        }) => state_error_response(repo_root, request, definition, &pipeline_id, failure),
        PipelineJournaledOutcome::Conflict {
            established_request_fingerprint,
            actual_request_fingerprint,
        } => idempotency_conflict_response(
            request,
            definition,
            established_request_fingerprint,
            actual_request_fingerprint,
        ),
        PipelineJournaledOutcome::JournalFailure { error, effects } => {
            journal_failure_response(request, definition, error, &effects)
        }
    }
}

fn mutation_ok_terminal<T: Serialize>(
    definition: &OperationDefinition,
    data: &T,
) -> Result<TerminalFields<ContractValue>, ContractError> {
    let projected = to_contract_value(data)?;
    PipelineOperationSchemaValidator.validate_instance(definition.result_schema(), &projected)?;
    TerminalFields::new(
        TerminalStatus::Ok,
        Some(projected),
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
    )
}

fn capture_plan_dto(plan: &PipelineCapturePlan) -> PipelineCapturePlanDtoV1 {
    PipelineCapturePlanDtoV1 {
        target: PipelineCaptureTargetDtoV1 {
            pipeline_id: plan.target.pipeline_id.clone(),
            stage_id: plan.target.stage_id.clone(),
            stage_file: plan.target.stage_file.clone(),
            title: plan.target.title.clone(),
        },
        route_basis_revision: plan.basis.state_revision,
        artifact_write_paths: plan
            .artifact_writes
            .iter()
            .map(|write| write.path.clone())
            .collect(),
        repository_mirror_write_paths: plan
            .repo_mirror_writes
            .iter()
            .map(|write| write.path.clone())
            .collect(),
        state_updates: plan
            .state_updates
            .iter()
            .map(|update| PipelineCaptureStateUpdateDtoV1 {
                field_path: update.field_path.clone(),
                value: match &update.value {
                    handbook_pipeline::pipeline_capture::PipelineCaptureStateValue::Bool(value) => {
                        PipelineCaptureStateValueDtoV1::Boolean(*value)
                    }
                    handbook_pipeline::pipeline_capture::PipelineCaptureStateValue::String(
                        value,
                    ) => PipelineCaptureStateValueDtoV1::Text(value.clone()),
                },
            })
            .collect(),
        capture_id: plan.capture_id.clone(),
        post_apply_next_safe_action: plan.post_apply_next_safe_action.clone(),
    }
}

fn route_state_mutation(mutation: PipelineStateMutationV1) -> RouteStateMutation {
    match mutation {
        PipelineStateMutationV1::RoutingVariable { variable, value } => {
            RouteStateMutation::RoutingVariable { variable, value }
        }
        PipelineStateMutationV1::CharterReference { value } => {
            RouteStateMutation::RefCharterRef { value }
        }
        PipelineStateMutationV1::ProjectContextReference { value } => {
            RouteStateMutation::RefProjectContextRef { value }
        }
        PipelineStateMutationV1::Runner { value } => RouteStateMutation::RunRunner { value },
        PipelineStateMutationV1::Profile { value } => RouteStateMutation::RunProfile { value },
    }
}

fn route_state_dto(state: &RouteState) -> PipelineRouteStateDtoV1 {
    PipelineRouteStateDtoV1 {
        revision: state.revision,
        routing: state
            .routing
            .iter()
            .map(|(variable, value)| PipelineRoutingEntryV1 {
                variable: variable.clone(),
                value: *value,
            })
            .collect(),
        charter_ref: state.refs.charter_ref.clone(),
        project_context_ref: state.refs.project_context_ref.clone(),
        runner: state.run.runner.clone(),
        profile: state.run.profile.clone(),
    }
}

fn mutation_problem_details(
    pipeline_id: Option<String>,
    stage_id: Option<String>,
    consumer_id: Option<String>,
    recovery: Option<String>,
) -> PipelineMutationProblemDetailsV1 {
    PipelineMutationProblemDetailsV1 {
        pipeline_id,
        stage_id,
        consumer_id,
        recovery,
        expected_revision: None,
        actual_revision: None,
        established_request_fingerprint: None,
        actual_request_fingerprint: None,
    }
}

fn capture_refusal_response(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    refusal: PipelineCaptureRefusal,
    effects: &[PipelineMutationEffect],
    expected_revision: Option<u64>,
    actual_revision: Option<u64>,
) -> Result<OperationResponseV1, ContractError> {
    let cause = match refusal.classification {
        PipelineCaptureRefusalClassification::UnsupportedTarget => {
            PipelineProblemCauseV1::CaptureUnsupportedTarget
        }
        PipelineCaptureRefusalClassification::InvalidDefinition => {
            PipelineProblemCauseV1::CaptureInvalidDefinition
        }
        PipelineCaptureRefusalClassification::InvalidState => {
            PipelineProblemCauseV1::CaptureInvalidState
        }
        PipelineCaptureRefusalClassification::MissingRouteBasis => {
            PipelineProblemCauseV1::CaptureMissingRouteBasis
        }
        PipelineCaptureRefusalClassification::MalformedRouteBasis => {
            PipelineProblemCauseV1::CaptureMalformedRouteBasis
        }
        PipelineCaptureRefusalClassification::StaleRouteBasis => {
            PipelineProblemCauseV1::CaptureStaleRouteBasis
        }
        PipelineCaptureRefusalClassification::InactiveStage => {
            PipelineProblemCauseV1::CaptureInactiveStage
        }
        PipelineCaptureRefusalClassification::InvalidCaptureInput => {
            PipelineProblemCauseV1::CaptureInvalidInput
        }
        PipelineCaptureRefusalClassification::InvalidWriteTarget => {
            PipelineProblemCauseV1::CaptureInvalidWriteTarget
        }
        PipelineCaptureRefusalClassification::MissingCaptureId => {
            PipelineProblemCauseV1::CaptureMissingId
        }
        PipelineCaptureRefusalClassification::TamperedCaptureCache => {
            PipelineProblemCauseV1::CaptureTamperedCache
        }
        PipelineCaptureRefusalClassification::RevisionConflict => {
            PipelineProblemCauseV1::CaptureRevisionConflict
        }
        PipelineCaptureRefusalClassification::WriteFailure => {
            PipelineProblemCauseV1::CaptureWriteFailure
        }
        PipelineCaptureRefusalClassification::StatePersistenceFailure => {
            PipelineProblemCauseV1::CaptureStatePersistenceFailure
        }
        PipelineCaptureRefusalClassification::CacheFailure => {
            PipelineProblemCauseV1::CaptureCacheFailure
        }
    };
    let mut mutation = mutation_problem_details(
        refusal.pipeline_id,
        refusal.stage_id,
        None,
        Some(sanitize_detail(repo_root, &refusal.recovery)),
    );
    mutation.expected_revision = expected_revision;
    mutation.actual_revision = actual_revision;
    mutation_problem_response(
        request,
        definition,
        PipelineProblemDetailsV1 {
            cause,
            detail: sanitize_detail(repo_root, &refusal.summary),
            selector: None,
            matches: Vec::new(),
            compile_refusal: None,
            mutation: Some(mutation),
        },
        effects,
        IdempotencyStage::ExecutionStart,
    )
}

fn handoff_refusal_response(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    refusal: PipelineHandoffRefusal,
    effects: &[PipelineMutationEffect],
) -> Result<OperationResponseV1, ContractError> {
    let cause = match refusal.classification {
        PipelineHandoffRefusalClassification::UnsupportedTarget => {
            PipelineProblemCauseV1::HandoffUnsupportedTarget
        }
        PipelineHandoffRefusalClassification::InvalidState => {
            PipelineProblemCauseV1::HandoffInvalidState
        }
        PipelineHandoffRefusalClassification::MissingRequiredInput => {
            PipelineProblemCauseV1::HandoffMissingInput
        }
        PipelineHandoffRefusalClassification::InvalidProvenance => {
            PipelineProblemCauseV1::HandoffInvalidProvenance
        }
        PipelineHandoffRefusalClassification::WriteFailure => {
            PipelineProblemCauseV1::HandoffWriteFailure
        }
    };
    mutation_problem_response(
        request,
        definition,
        PipelineProblemDetailsV1 {
            cause,
            detail: sanitize_detail(repo_root, &refusal.summary),
            selector: None,
            matches: Vec::new(),
            compile_refusal: None,
            mutation: Some(mutation_problem_details(
                refusal.pipeline_id,
                None,
                refusal.consumer_id,
                Some(sanitize_detail(repo_root, &refusal.recovery)),
            )),
        },
        effects,
        IdempotencyStage::ExecutionStart,
    )
}

fn state_refusal_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    pipeline_id: &str,
    refusal: RouteStateMutationRefusal,
) -> Result<OperationResponseV1, ContractError> {
    let detail = refusal.to_string();
    let (cause, detail, expected_revision, actual_revision) = match refusal {
        RouteStateMutationRefusal::MalformedState { .. } => {
            (PipelineProblemCauseV1::StateMalformed, detail, None, None)
        }
        RouteStateMutationRefusal::UnsupportedVariable { .. } => (
            PipelineProblemCauseV1::StateUnsupportedVariable,
            detail,
            None,
            None,
        ),
        RouteStateMutationRefusal::RevisionConflict {
            expected_revision,
            actual_revision,
        } => (
            PipelineProblemCauseV1::StateRevisionConflict,
            detail,
            Some(expected_revision),
            Some(actual_revision),
        ),
    };
    let mut mutation = mutation_problem_details(Some(pipeline_id.to_owned()), None, None, None);
    mutation.expected_revision = expected_revision;
    mutation.actual_revision = actual_revision;
    mutation_problem_response(
        request,
        definition,
        PipelineProblemDetailsV1 {
            cause,
            detail,
            selector: None,
            matches: Vec::new(),
            compile_refusal: None,
            mutation: Some(mutation),
        },
        &[],
        IdempotencyStage::PreconditionValidation,
    )
}

fn state_error_response(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    pipeline_id: &str,
    failure: RouteStateMutationFailure,
) -> Result<OperationResponseV1, ContractError> {
    let cause = match &failure.error {
        RouteStateStoreError::InvalidPipelineId { .. } => {
            PipelineProblemCauseV1::StateInvalidPipelineId
        }
        RouteStateStoreError::InvalidSupportedVariables { .. } => {
            PipelineProblemCauseV1::StateInvalidSupportedVariables
        }
        RouteStateStoreError::InvalidMutation { .. } => {
            PipelineProblemCauseV1::StateInvalidMutation
        }
        RouteStateStoreError::ReadFailure { .. } => PipelineProblemCauseV1::StateReadFailure,
        RouteStateStoreError::LockFailure { .. } => PipelineProblemCauseV1::StateLockFailure,
        RouteStateStoreError::WriteFailure { .. } => PipelineProblemCauseV1::StateWriteFailure,
        RouteStateStoreError::SerializationFailure { .. } => {
            PipelineProblemCauseV1::StateSerializationFailure
        }
    };
    mutation_problem_response(
        request,
        definition,
        PipelineProblemDetailsV1 {
            cause,
            detail: sanitize_detail(repo_root, &failure.error.to_string()),
            selector: None,
            matches: Vec::new(),
            compile_refusal: None,
            mutation: Some(mutation_problem_details(
                Some(pipeline_id.to_owned()),
                None,
                None,
                None,
            )),
        },
        &failure.effects,
        IdempotencyStage::ExecutionStart,
    )
}

fn idempotency_conflict_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    established_request_fingerprint: String,
    actual_request_fingerprint: String,
) -> Result<OperationResponseV1, ContractError> {
    let mut mutation = mutation_problem_details(None, None, None, None);
    mutation.established_request_fingerprint =
        Some(super::Fingerprint::parse(&established_request_fingerprint)?);
    mutation.actual_request_fingerprint =
        Some(super::Fingerprint::parse(&actual_request_fingerprint)?);
    mutation_problem_response(
        request,
        definition,
        PipelineProblemDetailsV1 {
            cause: PipelineProblemCauseV1::IdempotencyConflict,
            detail: "the idempotency key is already bound to a different semantic request"
                .to_string(),
            selector: None,
            matches: Vec::new(),
            compile_refusal: None,
            mutation: Some(mutation),
        },
        &[],
        IdempotencyStage::IdempotencyValidation,
    )
}

fn journal_failure_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    error: PipelineMutationJournalError,
    effects: &[PipelineMutationEffect],
) -> Result<OperationResponseV1, ContractError> {
    let cause = match error.classification {
        PipelineMutationJournalErrorClassification::InvalidOperation => {
            PipelineProblemCauseV1::IdempotencyInvalidOperation
        }
        PipelineMutationJournalErrorClassification::LockFailure => {
            PipelineProblemCauseV1::IdempotencyLockFailure
        }
        PipelineMutationJournalErrorClassification::ReadFailure => {
            PipelineProblemCauseV1::IdempotencyReadFailure
        }
        PipelineMutationJournalErrorClassification::MalformedEntry => {
            PipelineProblemCauseV1::IdempotencyMalformedEntry
        }
        PipelineMutationJournalErrorClassification::WriteFailure => {
            PipelineProblemCauseV1::IdempotencyWriteFailure
        }
    };
    mutation_problem_response(
        request,
        definition,
        PipelineProblemDetailsV1 {
            cause,
            detail: error.summary,
            selector: None,
            matches: Vec::new(),
            compile_refusal: None,
            mutation: Some(mutation_problem_details(None, None, None, None)),
        },
        effects,
        IdempotencyStage::IdempotencyValidation,
    )
}

fn ok_response<T: Serialize>(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    data: &T,
) -> Result<OperationResponseV1, ContractError> {
    let projected = to_contract_value(data)?;
    ok_projected_response(request, definition, projected)
}

fn ok_projected_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    projected: ContractValue,
) -> Result<OperationResponseV1, ContractError> {
    PipelineOperationSchemaValidator.validate_instance(definition.result_schema(), &projected)?;
    response_from_parts(
        request,
        definition,
        TerminalFields::new(
            TerminalStatus::Ok,
            Some(projected),
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
        )?,
    )
}

fn catalog_error_response(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    error: &PipelineCatalogError,
) -> Result<OperationResponseV1, ContractError> {
    let cause = match error {
        PipelineCatalogError::ReadPipelineCatalog { .. }
        | PipelineCatalogError::ReadStageCatalog { .. } => PipelineProblemCauseV1::CatalogRead,
        _ => PipelineProblemCauseV1::CatalogInvalid,
    };
    problem_response(
        request,
        definition,
        PipelineProblemDetailsV1 {
            cause,
            detail: sanitize_detail(repo_root, &error.to_string()),
            selector: None,
            matches: Vec::new(),
            compile_refusal: None,
            mutation: None,
        },
    )
}

fn selector_error_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    error: PipelineLookupError,
) -> Result<OperationResponseV1, ContractError> {
    let (cause, selector, matches, detail) = match error {
        PipelineLookupError::UnsupportedSelector { selector, reason } => (
            PipelineProblemCauseV1::UnsupportedSelector,
            selector.clone(),
            Vec::new(),
            reason.to_string(),
        ),
        PipelineLookupError::AmbiguousSelector { selector, matches } => (
            PipelineProblemCauseV1::AmbiguousSelector,
            selector.clone(),
            matches.clone(),
            format!(
                "ambiguous pipeline selector `{selector}` matches: {}",
                matches.join(", ")
            ),
        ),
        PipelineLookupError::UnknownSelector { selector } => (
            PipelineProblemCauseV1::UnknownSelector,
            selector.clone(),
            Vec::new(),
            format!("unknown pipeline selector `{selector}`"),
        ),
    };
    problem_response(
        request,
        definition,
        PipelineProblemDetailsV1 {
            cause,
            detail,
            selector: Some(selector),
            matches,
            compile_refusal: None,
            mutation: None,
        },
    )
}

fn compile_problem_response(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    refusal: PipelineCompileRefusal,
    cause_override: Option<PipelineProblemCauseV1>,
) -> Result<OperationResponseV1, ContractError> {
    let projection = PipelineCompileRefusalV1::from_owner(refusal, repo_root);
    let classified_cause = match projection.classification {
        PipelineCompileRefusalClassificationV1::UnsupportedTarget => {
            PipelineProblemCauseV1::UnsupportedCompileTarget
        }
        PipelineCompileRefusalClassificationV1::InvalidDefinition => {
            PipelineProblemCauseV1::InvalidCompileDefinition
        }
        PipelineCompileRefusalClassificationV1::InvalidState => {
            PipelineProblemCauseV1::InvalidRouteState
        }
        PipelineCompileRefusalClassificationV1::MissingRouteBasis => {
            PipelineProblemCauseV1::MissingRouteBasis
        }
        PipelineCompileRefusalClassificationV1::MalformedRouteBasis => {
            PipelineProblemCauseV1::MalformedRouteBasis
        }
        PipelineCompileRefusalClassificationV1::StaleRouteBasis => {
            PipelineProblemCauseV1::StaleRouteBasis
        }
        PipelineCompileRefusalClassificationV1::InactiveStage => {
            PipelineProblemCauseV1::InactiveStage
        }
        PipelineCompileRefusalClassificationV1::MissingRequiredInput => {
            PipelineProblemCauseV1::MissingRequiredInput
        }
        PipelineCompileRefusalClassificationV1::EmptyRequiredInput => {
            PipelineProblemCauseV1::EmptyRequiredInput
        }
    };
    let cause = cause_override.unwrap_or(classified_cause);
    problem_response(
        request,
        definition,
        PipelineProblemDetailsV1 {
            cause,
            detail: projection.summary.clone(),
            selector: None,
            matches: Vec::new(),
            compile_refusal: Some(projection),
            mutation: None,
        },
    )
}

fn problem_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    details: PipelineProblemDetailsV1,
) -> Result<OperationResponseV1, ContractError> {
    mutation_problem_response(
        request,
        definition,
        details,
        &[],
        IdempotencyStage::AuthorityValidation,
    )
}

fn mutation_problem_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    details: PipelineProblemDetailsV1,
    effects: &[PipelineMutationEffect],
    idempotency_stage: IdempotencyStage,
) -> Result<OperationResponseV1, ContractError> {
    details.validate()?;
    let limits = ContractLimits::packet0();
    let (status, category, retry) = problem_projection(details.cause);
    let detail_value = to_contract_value(&details)?;
    let details_schema = schema_binding(PIPELINE_DETAILS_REF)?;
    PipelineOperationSchemaValidator.validate_instance(&details_schema, &detail_value)?;
    let recheck = if status == TerminalStatus::Blocked {
        let condition = to_contract_value(&PipelineRecheckV1 {
            cause: details.cause,
        })?;
        let schema = schema_binding(PIPELINE_RECHECK_REF)?;
        PipelineOperationSchemaValidator.validate_instance(&schema, &condition)?;
        Some(RecheckCondition::new(schema, condition, &limits)?)
    } else {
        None
    };
    let rule = if status == TerminalStatus::Refused {
        Some(ExactBinding::new(
            ExactRef::parse(&format!(
                "handbook.pipeline-rule.{}@1.0.0",
                cause_name(details.cause).replace('_', "-")
            ))?,
            definition.owner().source().fingerprint().clone(),
        ))
    } else {
        None
    };
    let correlation = if status == TerminalStatus::Error {
        let fp = super::sha256_fingerprint(
            format!(
                "handbook.pipeline-error-correlation-v1\0{}\0{}",
                request.request_fingerprint().as_str(),
                cause_name(details.cause)
            )
            .as_bytes(),
        );
        Some(CorrelationRef::new(BoundedString::new(format!(
            "corr_{}",
            fp.as_str().trim_start_matches("sha256:")
        ))?)?)
    } else {
        None
    };
    let problem_id = BoundedString::new(format!("pipeline.{}", cause_name(details.cause)))?;
    let problem = Problem::new(
        problem_id.clone(),
        BoundedString::new(format!("pipeline.{}", cause_name(details.cause)))?,
        category,
        definition.binding(),
        rule,
        details_schema,
        detail_value,
        BoundedVec::new(Vec::new())?,
        retry,
        recheck,
        correlation,
        &limits,
    )?;
    let problem_binding = ProblemBinding::new(problem_id, problem.problem_fingerprint().clone())?;
    let terminal = match status {
        TerminalStatus::Blocked => TerminalFields::new(
            status,
            None,
            BoundedVec::new(vec![problem])?,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
        )?,
        TerminalStatus::Refused => TerminalFields::new(
            status,
            None,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(vec![problem])?,
            BoundedVec::new(Vec::new())?,
        )?,
        TerminalStatus::Error => TerminalFields::new(
            status,
            None,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(vec![problem])?,
        )?,
        TerminalStatus::Ok => return Err(ContractError::InvalidInvariant),
    };
    let idempotency = if matches!(definition.idempotency(), OperationIdempotency::Safe) {
        IdempotencyState::not_applicable()?
    } else {
        IdempotencyState::not_established(idempotency_stage, problem_binding)?
    };
    response_from_parts_with_evidence(
        request,
        definition,
        terminal,
        idempotency,
        pipeline_receipts(definition, effects)?,
    )
}

fn response_from_parts(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    terminal: TerminalFields<ContractValue>,
) -> Result<OperationResponseV1, ContractError> {
    response_from_parts_with_evidence(
        request,
        definition,
        terminal,
        IdempotencyState::not_applicable()?,
        BoundedVec::new(Vec::new())?,
    )
}

fn response_from_parts_with_evidence(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    terminal: TerminalFields<ContractValue>,
    idempotency: IdempotencyState,
    receipts: BoundedVec<WriteReceipt, 128>,
) -> Result<OperationResponseV1, ContractError> {
    OperationResponseV1::new(
        schema_binding("handbook.operation-response@1.0.0")?,
        request.request_id().cloned(),
        request.request_fingerprint().clone(),
        definition,
        terminal,
        idempotency,
        receipts,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        &PipelineOperationSchemaValidator,
        &ContractLimits::packet0(),
    )
}

fn pipeline_receipts(
    definition: &OperationDefinition,
    effects: &[PipelineMutationEffect],
) -> Result<BoundedVec<WriteReceipt, 128>, ContractError> {
    if effects.is_empty() {
        return BoundedVec::new(Vec::new());
    }
    let declared = definition
        .write_set()
        .first()
        .ok_or(ContractError::InvalidInvariant)?;
    if definition.write_set().len() != 1 {
        return Err(ContractError::InvalidInvariant);
    }
    let condition = PipelineWriteConditionV1 {
        operation: definition.operation().clone(),
        effects: effects
            .iter()
            .map(|effect| {
                Ok(PipelineWriteEffectV1 {
                    effect_kind: effect.effect_kind.clone(),
                    portable_path: effect.portable_path.clone(),
                    fingerprint: super::Fingerprint::parse(&effect.fingerprint)?,
                    revision: effect.revision,
                })
            })
            .collect::<Result<Vec<_>, ContractError>>()?,
    };
    let condition = to_contract_value(&condition)?;
    PipelineOperationSchemaValidator.validate_instance(declared.condition_schema(), &condition)?;
    let condition_bytes = canonical_json_bytes(&condition, &ContractLimits::packet0())?;
    let record_ref = ExactRef::parse(&format!(
        "handbook.pipeline-effect.{}@1.0.0",
        definition.operation().identity()
    ))?;
    let receipt = WriteReceipt::new(
        BoundedString::new(declared.record_kind().to_owned())?,
        ExactBinding::new(record_ref, super::sha256_fingerprint(&condition_bytes)),
        BoundedString::new(declared.authority_class().to_owned())?,
        condition,
        BoundedString::new(declared.atomic_group().to_owned())?,
        &ContractLimits::packet0(),
    )?;
    BoundedVec::new(vec![receipt])
}

fn established_mutation_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    terminal: TerminalFields<ContractValue>,
    effects: &[PipelineMutationEffect],
    replayed: bool,
) -> Result<OperationResponseV1, ContractError> {
    let scoped_key = request
        .idempotency_key_fingerprint()
        .cloned()
        .ok_or(ContractError::InvalidInvariant)?;
    let receipts = pipeline_receipts(definition, effects)?;
    let retention = UtcTimestamp::parse("9999-12-31T23:59:59Z")?;
    let original = if replayed {
        let original = response_from_parts_with_evidence(
            request,
            definition,
            terminal.clone(),
            IdempotencyState::established(
                scoped_key.clone(),
                request.request_fingerprint().clone(),
                false,
                None,
                retention.clone(),
            )?,
            receipts.clone(),
        )?;
        Some(original.result_fingerprint().clone())
    } else {
        None
    };
    response_from_parts_with_evidence(
        request,
        definition,
        terminal,
        IdempotencyState::established(
            scoped_key,
            request.request_fingerprint().clone(),
            replayed,
            original,
            retention,
        )?,
        receipts,
    )
}

pub fn pipeline_catalog_list_from_response(
    response: &OperationResponseV1,
) -> Result<Option<PipelineCatalogOutcome>, ContractError> {
    require_operation(response, PIPELINE_LIST_REF)?;
    response
        .terminal()
        .data()
        .map(|data| from_contract_value::<PipelineCatalogListResultV1>(data).map(Into::into))
        .transpose()
}

pub fn pipeline_catalog_read_from_response(
    response: &OperationResponseV1,
) -> Result<Option<PipelineShowOutcome>, ContractError> {
    require_operation(response, PIPELINE_READ_REF)?;
    response
        .terminal()
        .data()
        .map(|data| from_contract_value::<PipelineCatalogReadResultV1>(data).map(Into::into))
        .transpose()
}

pub fn pipeline_compile_from_response(
    response: &OperationResponseV1,
) -> Result<Option<PipelineCompileOutcome>, ContractError> {
    require_operation(response, PIPELINE_COMPILE_REF)?;
    response
        .terminal()
        .data()
        .map(|data| from_contract_value::<PipelineCompileResultV1>(data).map(Into::into))
        .transpose()
}

pub fn pipeline_capture_plan_from_response(
    response: &OperationResponseV1,
) -> Result<Option<crate::pipeline_route_api::PipelineCapturePreviewView>, ContractError> {
    require_operation(response, PIPELINE_CAPTURE_PLAN_REF)?;
    response
        .terminal()
        .data()
        .map(|data| {
            let data: PipelineCapturePlanResultV1 = from_contract_value(data)?;
            Ok(crate::pipeline_route_api::PipelineCapturePreviewView {
                plan: capture_plan_view(data.plan),
            })
        })
        .transpose()
}

pub fn pipeline_capture_apply_from_response(
    response: &OperationResponseV1,
) -> Result<Option<crate::pipeline_route_api::PipelineCaptureApplyView>, ContractError> {
    require_operation(response, PIPELINE_CAPTURE_APPLY_REF)?;
    response
        .terminal()
        .data()
        .map(|data| {
            let data: PipelineCaptureApplyResultV1 = from_contract_value(data)?;
            Ok(crate::pipeline_route_api::PipelineCaptureApplyView {
                plan: capture_plan_view(data.plan),
                written_files: data.written_files,
                persisted_state_revision: data.persisted_state_revision,
            })
        })
        .transpose()
}

pub fn pipeline_handoff_emit_from_response(
    response: &OperationResponseV1,
) -> Result<Option<crate::pipeline_route_api::PipelineHandoffOutcome>, ContractError> {
    require_operation(response, PIPELINE_HANDOFF_EMIT_REF)?;
    response
        .terminal()
        .data()
        .map(|data| {
            let data: PipelineHandoffEmitResultV1 = from_contract_value(data)?;
            Ok(crate::pipeline_route_api::PipelineHandoffOutcome {
                pipeline_id: data.pipeline_id,
                consumer_id: data.consumer_id,
                feature_id: data.feature_id,
                bundle_root: data.bundle_root,
                written_files: data.written_files,
            })
        })
        .transpose()
}

pub fn pipeline_state_apply_from_response(
    response: &OperationResponseV1,
) -> Result<Option<crate::pipeline_route_api::PipelineStateUpdateOutcome>, ContractError> {
    require_operation(response, PIPELINE_STATE_APPLY_REF)?;
    response
        .terminal()
        .data()
        .map(|data| {
            let data: PipelineStateApplyResultV1 = from_contract_value(data)?;
            Ok(crate::pipeline_route_api::PipelineStateUpdateOutcome {
                pipeline_id: data.pipeline_id,
                result: crate::pipeline_route_api::PipelineStateUpdateResult::Applied(
                    crate::pipeline_route_api::PipelineRouteStateView {
                        revision: data.state.revision,
                        routing: data
                            .state
                            .routing
                            .into_iter()
                            .map(|entry| (entry.variable, entry.value))
                            .collect(),
                        charter_ref: data.state.charter_ref,
                        project_context_ref: data.state.project_context_ref,
                        run: crate::pipeline_route_api::PipelineRouteRunView {
                            runner: data.state.runner,
                            profile: data.state.profile,
                            repository_root: None,
                        },
                    },
                ),
            })
        })
        .transpose()
}

pub fn pipeline_state_refusal_from_response(
    response: &OperationResponseV1,
) -> Result<Option<PipelineStateProblemProjection>, ContractError> {
    require_operation(response, PIPELINE_STATE_APPLY_REF)?;
    let Some(problem) = response
        .terminal()
        .blockers()
        .first()
        .or_else(|| response.terminal().refusals().first())
        .or_else(|| response.terminal().errors().first())
    else {
        return Ok(None);
    };
    let details: PipelineProblemDetailsV1 = from_contract_value(problem.details())?;
    match details.cause {
        PipelineProblemCauseV1::StateMalformed
        | PipelineProblemCauseV1::StateUnsupportedVariable
        | PipelineProblemCauseV1::StateRevisionConflict => {
            let pipeline_id = details
                .mutation
                .and_then(|mutation| mutation.pipeline_id)
                .ok_or(ContractError::InvalidInvariant)?;
            Ok(Some(PipelineStateProblemProjection::Outcome(
                crate::pipeline_route_api::PipelineStateUpdateOutcome {
                    pipeline_id,
                    result: crate::pipeline_route_api::PipelineStateUpdateResult::Refused(
                        details.detail,
                    ),
                },
            )))
        }
        PipelineProblemCauseV1::StateInvalidPipelineId
        | PipelineProblemCauseV1::StateInvalidSupportedVariables
        | PipelineProblemCauseV1::StateInvalidMutation
        | PipelineProblemCauseV1::StateReadFailure
        | PipelineProblemCauseV1::StateLockFailure
        | PipelineProblemCauseV1::StateWriteFailure
        | PipelineProblemCauseV1::StateSerializationFailure => {
            Ok(Some(PipelineStateProblemProjection::Failure(
                crate::pipeline_route_api::PipelineStateUpdateFailure::Mutation(details.detail),
            )))
        }
        _ => Ok(None),
    }
}

fn capture_plan_view(
    plan: PipelineCapturePlanDtoV1,
) -> crate::pipeline_route_api::PipelineCapturePlanView {
    crate::pipeline_route_api::PipelineCapturePlanView {
        target: crate::pipeline_route_api::PipelineCaptureTargetView {
            pipeline_id: plan.target.pipeline_id,
            stage_id: plan.target.stage_id,
            stage_file: plan.target.stage_file,
            title: plan.target.title,
        },
        route_basis_revision: plan.route_basis_revision,
        artifact_write_paths: plan.artifact_write_paths,
        repository_mirror_write_paths: plan.repository_mirror_write_paths,
        state_updates: plan
            .state_updates
            .into_iter()
            .map(
                |update| crate::pipeline_route_api::PipelineCaptureStateUpdateView {
                    field_path: update.field_path,
                    value: match update.value {
                        PipelineCaptureStateValueDtoV1::Boolean(value) => {
                            crate::pipeline_route_api::PipelineCaptureStateValue::Boolean(value)
                        }
                        PipelineCaptureStateValueDtoV1::Text(value) => {
                            crate::pipeline_route_api::PipelineCaptureStateValue::Text(value)
                        }
                    },
                },
            )
            .collect(),
        capture_id: plan.capture_id,
        post_apply_next_safe_action: plan.post_apply_next_safe_action,
    }
}

pub type PipelineOperationProblemProjection = (
    String,
    Option<PipelineSelectorRefusal>,
    Option<PipelineCompileRefusalView>,
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PipelineStateProblemProjection {
    Outcome(crate::pipeline_route_api::PipelineStateUpdateOutcome),
    Failure(crate::pipeline_route_api::PipelineStateUpdateFailure),
}

pub fn pipeline_operation_problem_details(
    response: &OperationResponseV1,
) -> Result<Option<PipelineOperationProblemProjection>, ContractError> {
    let problem = response
        .terminal()
        .blockers()
        .first()
        .or_else(|| response.terminal().refusals().first())
        .or_else(|| response.terminal().errors().first());
    problem
        .map(|problem| {
            let details: PipelineProblemDetailsV1 = from_contract_value(problem.details())?;
            let selector = match details.cause {
                PipelineProblemCauseV1::UnsupportedSelector => {
                    Some(PipelineSelectorRefusal::Unsupported {
                        selector: details
                            .selector
                            .clone()
                            .ok_or(ContractError::InvalidInvariant)?,
                        reason: details.detail.clone(),
                    })
                }
                PipelineProblemCauseV1::AmbiguousSelector => {
                    Some(PipelineSelectorRefusal::Ambiguous {
                        selector: details
                            .selector
                            .clone()
                            .ok_or(ContractError::InvalidInvariant)?,
                        matches: details.matches.clone(),
                    })
                }
                PipelineProblemCauseV1::UnknownSelector => Some(PipelineSelectorRefusal::Unknown {
                    selector: details
                        .selector
                        .clone()
                        .ok_or(ContractError::InvalidInvariant)?,
                }),
                _ => None,
            };
            Ok((
                details.detail,
                selector,
                details.compile_refusal.map(Into::into),
            ))
        })
        .transpose()
}

pub fn pipeline_capture_refusal_from_response(
    response: &OperationResponseV1,
) -> Result<Option<crate::pipeline_route_api::PipelineCaptureRefusalView>, ContractError> {
    if !matches!(
        response.operation().reference().as_str(),
        PIPELINE_CAPTURE_PLAN_REF | PIPELINE_CAPTURE_APPLY_REF
    ) {
        return Err(ContractError::InvalidInvariant);
    }
    let Some(problem) = response
        .terminal()
        .blockers()
        .first()
        .or_else(|| response.terminal().refusals().first())
        .or_else(|| response.terminal().errors().first())
    else {
        return Ok(None);
    };
    let details: PipelineProblemDetailsV1 = from_contract_value(problem.details())?;
    let classification = match details.cause {
        PipelineProblemCauseV1::CaptureUnsupportedTarget => {
            crate::pipeline_route_api::PipelineCaptureRefusalClassificationView::UnsupportedTarget
        }
        PipelineProblemCauseV1::CaptureInvalidDefinition => {
            crate::pipeline_route_api::PipelineCaptureRefusalClassificationView::InvalidDefinition
        }
        PipelineProblemCauseV1::CaptureInvalidState => {
            crate::pipeline_route_api::PipelineCaptureRefusalClassificationView::InvalidState
        }
        PipelineProblemCauseV1::CaptureMissingRouteBasis => {
            crate::pipeline_route_api::PipelineCaptureRefusalClassificationView::MissingRouteBasis
        }
        PipelineProblemCauseV1::CaptureMalformedRouteBasis => {
            crate::pipeline_route_api::PipelineCaptureRefusalClassificationView::MalformedRouteBasis
        }
        PipelineProblemCauseV1::CaptureStaleRouteBasis => {
            crate::pipeline_route_api::PipelineCaptureRefusalClassificationView::StaleRouteBasis
        }
        PipelineProblemCauseV1::CaptureInactiveStage => {
            crate::pipeline_route_api::PipelineCaptureRefusalClassificationView::InactiveStage
        }
        PipelineProblemCauseV1::CaptureInvalidInput => {
            crate::pipeline_route_api::PipelineCaptureRefusalClassificationView::InvalidCaptureInput
        }
        PipelineProblemCauseV1::CaptureInvalidWriteTarget => {
            crate::pipeline_route_api::PipelineCaptureRefusalClassificationView::InvalidWriteTarget
        }
        PipelineProblemCauseV1::CaptureMissingId => {
            crate::pipeline_route_api::PipelineCaptureRefusalClassificationView::MissingCaptureId
        }
        PipelineProblemCauseV1::CaptureTamperedCache => {
            crate::pipeline_route_api::PipelineCaptureRefusalClassificationView::TamperedCaptureCache
        }
        PipelineProblemCauseV1::CaptureRevisionConflict => {
            crate::pipeline_route_api::PipelineCaptureRefusalClassificationView::RevisionConflict
        }
        PipelineProblemCauseV1::CaptureWriteFailure => {
            crate::pipeline_route_api::PipelineCaptureRefusalClassificationView::WriteFailure
        }
        PipelineProblemCauseV1::CaptureStatePersistenceFailure => {
            crate::pipeline_route_api::PipelineCaptureRefusalClassificationView::StatePersistenceFailure
        }
        PipelineProblemCauseV1::CaptureCacheFailure => {
            crate::pipeline_route_api::PipelineCaptureRefusalClassificationView::CacheFailure
        }
        _ => return Ok(None),
    };
    let mutation = details.mutation.ok_or(ContractError::InvalidInvariant)?;
    Ok(Some(
        crate::pipeline_route_api::PipelineCaptureRefusalView {
            classification,
            summary: details.detail,
            pipeline_id: mutation.pipeline_id,
            stage_id: mutation.stage_id,
            recovery: mutation.recovery.ok_or(ContractError::InvalidInvariant)?,
        },
    ))
}

pub fn pipeline_handoff_refusal_from_response(
    response: &OperationResponseV1,
) -> Result<Option<crate::pipeline_route_api::PipelineHandoffRefusalView>, ContractError> {
    require_operation(response, PIPELINE_HANDOFF_EMIT_REF)?;
    let Some(problem) = response
        .terminal()
        .blockers()
        .first()
        .or_else(|| response.terminal().refusals().first())
        .or_else(|| response.terminal().errors().first())
    else {
        return Ok(None);
    };
    let details: PipelineProblemDetailsV1 = from_contract_value(problem.details())?;
    let classification = match details.cause {
        PipelineProblemCauseV1::HandoffUnsupportedTarget => {
            crate::pipeline_route_api::PipelineHandoffRefusalClassificationView::UnsupportedTarget
        }
        PipelineProblemCauseV1::HandoffInvalidState => {
            crate::pipeline_route_api::PipelineHandoffRefusalClassificationView::InvalidState
        }
        PipelineProblemCauseV1::HandoffMissingInput => {
            crate::pipeline_route_api::PipelineHandoffRefusalClassificationView::MissingRequiredInput
        }
        PipelineProblemCauseV1::HandoffInvalidProvenance => {
            crate::pipeline_route_api::PipelineHandoffRefusalClassificationView::InvalidProvenance
        }
        PipelineProblemCauseV1::HandoffWriteFailure => {
            crate::pipeline_route_api::PipelineHandoffRefusalClassificationView::WriteFailure
        }
        _ => return Ok(None),
    };
    let mutation = details.mutation.ok_or(ContractError::InvalidInvariant)?;
    Ok(Some(
        crate::pipeline_route_api::PipelineHandoffRefusalView {
            classification,
            summary: details.detail,
            pipeline_id: mutation.pipeline_id,
            consumer_id: mutation.consumer_id,
            recovery: mutation.recovery.ok_or(ContractError::InvalidInvariant)?,
        },
    ))
}

fn require_operation(response: &OperationResponseV1, expected: &str) -> Result<(), ContractError> {
    if response.operation().reference().as_str() == expected {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
}

pub(crate) fn current_pipeline_capabilities() -> Result<Vec<CapabilityEntry>, ContractError> {
    pipeline_operation_definitions()?
        .into_iter()
        .map(|definition| {
            let owner = match super::admit_operation_definition(&definition)? {
                OperationAdmissionResult::Admitted(owner) => owner,
                _ => return Err(ContractError::InvalidInvariant),
            };
            let current_transports: &'static [OperationTransport] =
                &[OperationTransport::CliJson, OperationTransport::Tauri];
            CapabilityEntry::new(
                &definition,
                &PipelineCliTransportEvidence {
                    owner,
                    current_transports,
                },
                &ContractLimits::packet0(),
            )
        })
        .collect()
}

struct PipelineCliTransportEvidence {
    owner: OwnerAdmissionToken,
    current_transports: &'static [OperationTransport],
}
impl CurrentTransportEvidenceSealed for PipelineCliTransportEvidence {}
impl CurrentTransportEvidence for PipelineCliTransportEvidence {
    fn owner_admission(&self) -> &OwnerAdmissionToken {
        &self.owner
    }
    fn current_transports(&self) -> &[OperationTransport] {
        self.current_transports
    }
}

impl PipelineCompileResultV1 {
    fn from_owner(result: PipelineCompileResult) -> Self {
        Self {
            target: PipelineCompileTargetV1 {
                pipeline_id: result.target.pipeline_id,
                stage_id: result.target.stage_id,
                stage_file: result.target.stage_file,
                title: result.target.title,
                description: result.target.description,
                work_level: result.target.work_level,
                tags: result.target.tags,
            },
            basis: result.basis.into(),
            variables: result
                .variables
                .into_iter()
                .map(|v| PipelineCompileVariableV1 {
                    name: v.name,
                    value: v.value,
                })
                .collect(),
            documents: result
                .documents
                .into_iter()
                .map(|d| PipelineCompileDocumentV1 {
                    kind: match d.kind {
                        PipelineCompileDocumentKind::Include => {
                            PipelineCompileDocumentKindV1::Include
                        }
                        PipelineCompileDocumentKind::Runner => {
                            PipelineCompileDocumentKindV1::Runner
                        }
                        PipelineCompileDocumentKind::Profile => {
                            PipelineCompileDocumentKindV1::Profile
                        }
                        PipelineCompileDocumentKind::Library => {
                            PipelineCompileDocumentKindV1::Library
                        }
                        PipelineCompileDocumentKind::Artifact => {
                            PipelineCompileDocumentKindV1::Artifact
                        }
                    },
                    path: d.path,
                    required: d.required,
                    status: match d.status {
                        PipelineCompileDocumentStatus::Present => {
                            PipelineCompileDocumentStatusV1::Present
                        }
                        PipelineCompileDocumentStatus::MissingOptional => {
                            PipelineCompileDocumentStatusV1::MissingOptional
                        }
                    },
                    content: d.content,
                })
                .collect(),
            outputs: result
                .outputs
                .into_iter()
                .map(|o| PipelineCompileOutputV1 {
                    kind: match o.kind {
                        PipelineCompileOutputKind::Artifact => {
                            PipelineCompileOutputKindV1::Artifact
                        }
                        PipelineCompileOutputKind::RepoFile => {
                            PipelineCompileOutputKindV1::RepositoryFile
                        }
                    },
                    path: o.path,
                })
                .collect(),
            gating: PipelineCompileGatingV1 {
                mode: result.gating.mode,
                fail_on: result.gating.fail_on,
                notes: result.gating.notes,
            },
            stage_body: result.stage_body,
        }
    }
}

impl PipelineCompileRefusalV1 {
    fn from_owner(value: PipelineCompileRefusal, repo_root: &Path) -> Self {
        Self {
            classification: match value.classification {
                PipelineCompileRefusalClassification::UnsupportedTarget => {
                    PipelineCompileRefusalClassificationV1::UnsupportedTarget
                }
                PipelineCompileRefusalClassification::InvalidDefinition => {
                    PipelineCompileRefusalClassificationV1::InvalidDefinition
                }
                PipelineCompileRefusalClassification::InvalidState => {
                    PipelineCompileRefusalClassificationV1::InvalidState
                }
                PipelineCompileRefusalClassification::MissingRouteBasis => {
                    PipelineCompileRefusalClassificationV1::MissingRouteBasis
                }
                PipelineCompileRefusalClassification::MalformedRouteBasis => {
                    PipelineCompileRefusalClassificationV1::MalformedRouteBasis
                }
                PipelineCompileRefusalClassification::StaleRouteBasis => {
                    PipelineCompileRefusalClassificationV1::StaleRouteBasis
                }
                PipelineCompileRefusalClassification::InactiveStage => {
                    PipelineCompileRefusalClassificationV1::InactiveStage
                }
                PipelineCompileRefusalClassification::MissingRequiredInput => {
                    PipelineCompileRefusalClassificationV1::MissingRequiredInput
                }
                PipelineCompileRefusalClassification::EmptyRequiredInput => {
                    PipelineCompileRefusalClassificationV1::EmptyRequiredInput
                }
            },
            summary: sanitize_detail(repo_root, &value.summary),
            pipeline_id: value.pipeline_id,
            stage_id: value.stage_id,
            recovery: sanitize_detail(repo_root, &value.recovery),
        }
    }
}

impl From<PipelineCatalogListResultV1> for PipelineCatalogOutcome {
    fn from(value: PipelineCatalogListResultV1) -> Self {
        Self {
            pipelines: value
                .pipelines
                .into_iter()
                .map(|p| PipelineCatalogEntry {
                    id: p.id,
                    title: p.title,
                    source_path: PathBuf::from(p.source_path),
                    stage_count: p.stage_count as usize,
                })
                .collect(),
            pipeline_count: value.pipeline_count as usize,
            stage_count: value.stage_count as usize,
        }
    }
}

impl From<PipelineCatalogReadResultV1> for PipelineShowOutcome {
    fn from(value: PipelineCatalogReadResultV1) -> Self {
        match value {
            PipelineCatalogReadResultV1::Pipeline { pipeline } => {
                Self::Pipeline(PipelineDefinitionView {
                    id: pipeline.id,
                    title: pipeline.title,
                    description: pipeline.description,
                    source_path: PathBuf::from(pipeline.source_path),
                    default_runner: pipeline.default_runner,
                    default_profile: pipeline.default_profile,
                    default_enable_complexity: pipeline.default_enable_complexity,
                    stages: pipeline
                        .stages
                        .into_iter()
                        .map(|s| PipelineDeclaredStageView {
                            id: s.id,
                            title: s.title,
                            source_path: PathBuf::from(s.source_path),
                            work_level: s.work_level,
                            sets: s.sets,
                            activation: s.activation.map(|a| PipelineActivationView {
                                operator: match a.operator {
                                    PipelineActivationOperatorV1::Any => {
                                        PipelineActivationOperator::Any
                                    }
                                    PipelineActivationOperatorV1::All => {
                                        PipelineActivationOperator::All
                                    }
                                },
                                clauses: a
                                    .clauses
                                    .into_iter()
                                    .map(|c| PipelineActivationClause {
                                        variable: c.variable,
                                        value: c.value,
                                    })
                                    .collect(),
                            }),
                        })
                        .collect(),
                })
            }
            PipelineCatalogReadResultV1::Stage { stage } => Self::Stage(PipelineStageView {
                id: stage.id,
                kind: stage.kind,
                version: stage.version,
                title: stage.title,
                description: stage.description,
                work_level: stage.work_level,
                source_path: PathBuf::from(stage.source_path),
                pipeline_ids: stage.pipeline_ids,
            }),
        }
    }
}

impl From<PipelineCompileResultV1> for PipelineCompileOutcome {
    fn from(value: PipelineCompileResultV1) -> Self {
        Self {
            target: PipelineCompileTargetView {
                pipeline_id: value.target.pipeline_id,
                stage_id: value.target.stage_id,
                stage_file: value.target.stage_file,
                title: value.target.title,
                description: value.target.description,
                work_level: value.target.work_level,
                tags: value.target.tags,
            },
            basis: value.basis.into(),
            variables: value
                .variables
                .into_iter()
                .map(|v| crate::pipeline_route_api::PipelineCompileVariableView {
                    name: v.name,
                    value: v.value,
                })
                .collect(),
            documents: value
                .documents
                .into_iter()
                .map(|d| PipelineCompileDocumentView {
                    kind: match d.kind {
                        PipelineCompileDocumentKindV1::Include => {
                            PipelineCompileDocumentKindView::Include
                        }
                        PipelineCompileDocumentKindV1::Runner => {
                            PipelineCompileDocumentKindView::Runner
                        }
                        PipelineCompileDocumentKindV1::Profile => {
                            PipelineCompileDocumentKindView::Profile
                        }
                        PipelineCompileDocumentKindV1::Library => {
                            PipelineCompileDocumentKindView::Library
                        }
                        PipelineCompileDocumentKindV1::Artifact => {
                            PipelineCompileDocumentKindView::Artifact
                        }
                    },
                    path: d.path,
                    required: d.required,
                    status: match d.status {
                        PipelineCompileDocumentStatusV1::Present => {
                            PipelineCompileDocumentStatusView::Present
                        }
                        PipelineCompileDocumentStatusV1::MissingOptional => {
                            PipelineCompileDocumentStatusView::MissingOptional
                        }
                    },
                    content: d.content,
                })
                .collect(),
            outputs: value
                .outputs
                .into_iter()
                .map(|o| PipelineCompileOutputView {
                    kind: match o.kind {
                        PipelineCompileOutputKindV1::Artifact => {
                            PipelineCompileOutputKindView::Artifact
                        }
                        PipelineCompileOutputKindV1::RepositoryFile => {
                            PipelineCompileOutputKindView::RepositoryFile
                        }
                    },
                    path: o.path,
                })
                .collect(),
            gating: PipelineCompileGatingView {
                mode: value.gating.mode,
                fail_on: value.gating.fail_on,
                notes: value.gating.notes,
            },
            stage_body: value.stage_body,
        }
    }
}

impl From<PipelineCompileRefusalV1> for PipelineCompileRefusalView {
    fn from(v: PipelineCompileRefusalV1) -> Self {
        Self {
            classification: match v.classification {
                PipelineCompileRefusalClassificationV1::UnsupportedTarget => {
                    PipelineCompileRefusalClassificationView::UnsupportedTarget
                }
                PipelineCompileRefusalClassificationV1::InvalidDefinition => {
                    PipelineCompileRefusalClassificationView::InvalidDefinition
                }
                PipelineCompileRefusalClassificationV1::InvalidState => {
                    PipelineCompileRefusalClassificationView::InvalidState
                }
                PipelineCompileRefusalClassificationV1::MissingRouteBasis => {
                    PipelineCompileRefusalClassificationView::MissingRouteBasis
                }
                PipelineCompileRefusalClassificationV1::MalformedRouteBasis => {
                    PipelineCompileRefusalClassificationView::MalformedRouteBasis
                }
                PipelineCompileRefusalClassificationV1::StaleRouteBasis => {
                    PipelineCompileRefusalClassificationView::StaleRouteBasis
                }
                PipelineCompileRefusalClassificationV1::InactiveStage => {
                    PipelineCompileRefusalClassificationView::InactiveStage
                }
                PipelineCompileRefusalClassificationV1::MissingRequiredInput => {
                    PipelineCompileRefusalClassificationView::MissingRequiredInput
                }
                PipelineCompileRefusalClassificationV1::EmptyRequiredInput => {
                    PipelineCompileRefusalClassificationView::EmptyRequiredInput
                }
            },
            summary: v.summary,
            pipeline_id: v.pipeline_id,
            stage_id: v.stage_id,
            recovery: v.recovery,
        }
    }
}

impl From<handbook_pipeline::route_state::RouteBasis> for PipelineRouteBasisV1 {
    fn from(b: handbook_pipeline::route_state::RouteBasis) -> Self {
        Self {
            schema_version: b.schema_version,
            state_revision: b.state_revision,
            pipeline_file: b.pipeline_file,
            pipeline_file_sha256: b.pipeline_file_sha256,
            routing: b
                .routing
                .into_iter()
                .map(|(variable, value)| PipelineRoutingEntryV1 { variable, value })
                .collect(),
            charter_ref: b.refs.charter_ref,
            project_context_ref: b.refs.project_context_ref,
            run: PipelineRouteRunV1 {
                runner: b.run.runner,
                profile: b.run.profile,
                repository_root: b.run.repo_root,
            },
            runner: PipelineRouteBasisRunnerV1 {
                id: b.runner.id,
                file: b.runner.file,
                file_sha256: b.runner.file_sha256,
            },
            profile: PipelineRouteBasisProfileV1 {
                id: b.profile.id,
                profile_yaml_sha256: b.profile.profile_yaml_sha256,
                commands_yaml_sha256: b.profile.commands_yaml_sha256,
                conventions_md_sha256: b.profile.conventions_md_sha256,
            },
            route: b
                .route
                .into_iter()
                .map(|s| PipelineRouteBasisStageV1 {
                    stage_id: s.stage_id,
                    file: s.file,
                    status: route_status(s.status),
                    reason: s.reason.map(route_reason),
                })
                .collect(),
        }
    }
}

impl From<PipelineRouteBasisV1> for PipelineRouteBasisView {
    fn from(b: PipelineRouteBasisV1) -> Self {
        Self {
            schema_version: b.schema_version,
            state_revision: b.state_revision,
            pipeline_file: b.pipeline_file,
            pipeline_file_sha256: b.pipeline_file_sha256,
            routing: b
                .routing
                .into_iter()
                .map(|e| (e.variable, e.value))
                .collect(),
            charter_ref: b.charter_ref,
            project_context_ref: b.project_context_ref,
            run: PipelineRouteRunView {
                runner: b.run.runner,
                profile: b.run.profile,
                repository_root: b.run.repository_root,
            },
            runner: PipelineRouteBasisRunnerView {
                id: b.runner.id,
                file: b.runner.file,
                file_sha256: b.runner.file_sha256,
            },
            profile: PipelineRouteBasisProfileView {
                id: b.profile.id,
                profile_yaml_sha256: b.profile.profile_yaml_sha256,
                commands_yaml_sha256: b.profile.commands_yaml_sha256,
                conventions_md_sha256: b.profile.conventions_md_sha256,
            },
            route: b
                .route
                .into_iter()
                .map(|s| PipelineRouteBasisStageView {
                    stage_id: s.stage_id,
                    file: s.file,
                    status: view_status(s.status),
                    reason: s.reason.map(view_reason),
                })
                .collect(),
        }
    }
}

fn route_status(s: RouteBasisStageStatus) -> PipelineRouteStageStatusV1 {
    match s {
        RouteBasisStageStatus::Active => PipelineRouteStageStatusV1::Active,
        RouteBasisStageStatus::Skipped => PipelineRouteStageStatusV1::Skipped,
        RouteBasisStageStatus::Blocked => PipelineRouteStageStatusV1::Blocked,
        RouteBasisStageStatus::Next => PipelineRouteStageStatusV1::Next,
    }
}
fn route_reason(r: RouteBasisStageReason) -> PipelineRouteStageReasonV1 {
    match r {
        RouteBasisStageReason::SkippedActivationFalse {
            unsatisfied_variables,
            ..
        } => PipelineRouteStageReasonV1::SkippedActivationFalse {
            unsatisfied_variables,
        },
        RouteBasisStageReason::NextMissingRouteVariables {
            missing_variables, ..
        } => PipelineRouteStageReasonV1::NextMissingRouteVariables { missing_variables },
        RouteBasisStageReason::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status,
        } => PipelineRouteStageReasonV1::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status: route_status(upstream_status),
        },
    }
}
fn view_status(s: PipelineRouteStageStatusV1) -> PipelineRouteStageStatus {
    match s {
        PipelineRouteStageStatusV1::Active => PipelineRouteStageStatus::Active,
        PipelineRouteStageStatusV1::Skipped => PipelineRouteStageStatus::Skipped,
        PipelineRouteStageStatusV1::Blocked => PipelineRouteStageStatus::Blocked,
        PipelineRouteStageStatusV1::Next => PipelineRouteStageStatus::Next,
    }
}
fn view_reason(r: PipelineRouteStageReasonV1) -> PipelineRouteStageReason {
    match r {
        PipelineRouteStageReasonV1::SkippedActivationFalse {
            unsatisfied_variables,
        } => PipelineRouteStageReason::SkippedActivationFalse {
            unsatisfied_variables,
        },
        PipelineRouteStageReasonV1::NextMissingRouteVariables { missing_variables } => {
            PipelineRouteStageReason::NextMissingRouteVariables { missing_variables }
        }
        PipelineRouteStageReasonV1::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status,
        } => PipelineRouteStageReason::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status: view_status(upstream_status),
        },
    }
}

fn problem_projection(
    cause: PipelineProblemCauseV1,
) -> (TerminalStatus, ProblemCategory, RetryDirective) {
    match cause {
        PipelineProblemCauseV1::CatalogRead => (
            TerminalStatus::Error,
            ProblemCategory::Implementation,
            RetryDirective::Transient,
        ),
        PipelineProblemCauseV1::CatalogInvalid => (
            TerminalStatus::Error,
            ProblemCategory::Implementation,
            RetryDirective::Never,
        ),
        PipelineProblemCauseV1::UnsupportedSelector
        | PipelineProblemCauseV1::AmbiguousSelector
        | PipelineProblemCauseV1::UnknownSelector => (
            TerminalStatus::Refused,
            ProblemCategory::Resolution,
            RetryDirective::AfterRequestChange,
        ),
        PipelineProblemCauseV1::UnsupportedCompileTarget => (
            TerminalStatus::Refused,
            ProblemCategory::Capability,
            RetryDirective::Never,
        ),
        PipelineProblemCauseV1::CaptureUnsupportedTarget
        | PipelineProblemCauseV1::HandoffUnsupportedTarget => (
            TerminalStatus::Refused,
            ProblemCategory::Capability,
            RetryDirective::Never,
        ),
        PipelineProblemCauseV1::CaptureInvalidDefinition
        | PipelineProblemCauseV1::CaptureInvalidInput
        | PipelineProblemCauseV1::CaptureInvalidWriteTarget
        | PipelineProblemCauseV1::CaptureMissingId
        | PipelineProblemCauseV1::CaptureTamperedCache
        | PipelineProblemCauseV1::StateUnsupportedVariable
        | PipelineProblemCauseV1::StateInvalidPipelineId
        | PipelineProblemCauseV1::StateInvalidSupportedVariables
        | PipelineProblemCauseV1::StateInvalidMutation => (
            TerminalStatus::Refused,
            ProblemCategory::Precondition,
            RetryDirective::AfterRequestChange,
        ),
        PipelineProblemCauseV1::CaptureRevisionConflict
        | PipelineProblemCauseV1::StateRevisionConflict => (
            TerminalStatus::Refused,
            ProblemCategory::Precondition,
            RetryDirective::AfterRequestChange,
        ),
        PipelineProblemCauseV1::IdempotencyConflict => (
            TerminalStatus::Refused,
            ProblemCategory::Idempotency,
            RetryDirective::AfterRequestChange,
        ),
        PipelineProblemCauseV1::CaptureWriteFailure
        | PipelineProblemCauseV1::CaptureStatePersistenceFailure
        | PipelineProblemCauseV1::CaptureCacheFailure
        | PipelineProblemCauseV1::HandoffWriteFailure
        | PipelineProblemCauseV1::StateReadFailure
        | PipelineProblemCauseV1::StateLockFailure
        | PipelineProblemCauseV1::StateWriteFailure
        | PipelineProblemCauseV1::StateSerializationFailure
        | PipelineProblemCauseV1::IdempotencyInvalidOperation
        | PipelineProblemCauseV1::IdempotencyLockFailure
        | PipelineProblemCauseV1::IdempotencyReadFailure
        | PipelineProblemCauseV1::IdempotencyMalformedEntry
        | PipelineProblemCauseV1::IdempotencyWriteFailure => (
            TerminalStatus::Error,
            ProblemCategory::Implementation,
            RetryDirective::Transient,
        ),
        _ => (
            TerminalStatus::Blocked,
            ProblemCategory::Prerequisite,
            RetryDirective::AfterRecheck,
        ),
    }
}

fn is_mutation_cause(cause: PipelineProblemCauseV1) -> bool {
    !matches!(
        cause,
        PipelineProblemCauseV1::CatalogRead
            | PipelineProblemCauseV1::CatalogInvalid
            | PipelineProblemCauseV1::UnsupportedSelector
            | PipelineProblemCauseV1::AmbiguousSelector
            | PipelineProblemCauseV1::UnknownSelector
            | PipelineProblemCauseV1::UnsupportedCompileTarget
            | PipelineProblemCauseV1::InvalidCompileDefinition
            | PipelineProblemCauseV1::InvalidRouteState
            | PipelineProblemCauseV1::MissingRouteBasis
            | PipelineProblemCauseV1::MalformedRouteBasis
            | PipelineProblemCauseV1::StaleRouteBasis
            | PipelineProblemCauseV1::InactiveStage
            | PipelineProblemCauseV1::MissingRequiredInput
            | PipelineProblemCauseV1::EmptyRequiredInput
            | PipelineProblemCauseV1::CompileResultBoundExceeded
    )
}

fn is_compile_cause(cause: PipelineProblemCauseV1) -> bool {
    matches!(
        cause,
        PipelineProblemCauseV1::UnsupportedCompileTarget
            | PipelineProblemCauseV1::InvalidCompileDefinition
            | PipelineProblemCauseV1::InvalidRouteState
            | PipelineProblemCauseV1::MissingRouteBasis
            | PipelineProblemCauseV1::MalformedRouteBasis
            | PipelineProblemCauseV1::StaleRouteBasis
            | PipelineProblemCauseV1::InactiveStage
            | PipelineProblemCauseV1::MissingRequiredInput
            | PipelineProblemCauseV1::EmptyRequiredInput
            | PipelineProblemCauseV1::CompileResultBoundExceeded
    )
}
fn cause_name(cause: PipelineProblemCauseV1) -> &'static str {
    match cause {
        PipelineProblemCauseV1::CatalogRead => "catalog_read",
        PipelineProblemCauseV1::CatalogInvalid => "catalog_invalid",
        PipelineProblemCauseV1::UnsupportedSelector => "unsupported_selector",
        PipelineProblemCauseV1::AmbiguousSelector => "ambiguous_selector",
        PipelineProblemCauseV1::UnknownSelector => "unknown_selector",
        PipelineProblemCauseV1::UnsupportedCompileTarget => "unsupported_compile_target",
        PipelineProblemCauseV1::InvalidCompileDefinition => "invalid_compile_definition",
        PipelineProblemCauseV1::InvalidRouteState => "invalid_route_state",
        PipelineProblemCauseV1::MissingRouteBasis => "missing_route_basis",
        PipelineProblemCauseV1::MalformedRouteBasis => "malformed_route_basis",
        PipelineProblemCauseV1::StaleRouteBasis => "stale_route_basis",
        PipelineProblemCauseV1::InactiveStage => "inactive_stage",
        PipelineProblemCauseV1::MissingRequiredInput => "missing_required_input",
        PipelineProblemCauseV1::EmptyRequiredInput => "empty_required_input",
        PipelineProblemCauseV1::CompileResultBoundExceeded => "compile_result_bound_exceeded",
        PipelineProblemCauseV1::CaptureUnsupportedTarget => "capture_unsupported_target",
        PipelineProblemCauseV1::CaptureInvalidDefinition => "capture_invalid_definition",
        PipelineProblemCauseV1::CaptureInvalidState => "capture_invalid_state",
        PipelineProblemCauseV1::CaptureMissingRouteBasis => "capture_missing_route_basis",
        PipelineProblemCauseV1::CaptureMalformedRouteBasis => "capture_malformed_route_basis",
        PipelineProblemCauseV1::CaptureStaleRouteBasis => "capture_stale_route_basis",
        PipelineProblemCauseV1::CaptureInactiveStage => "capture_inactive_stage",
        PipelineProblemCauseV1::CaptureInvalidInput => "capture_invalid_input",
        PipelineProblemCauseV1::CaptureInvalidWriteTarget => "capture_invalid_write_target",
        PipelineProblemCauseV1::CaptureMissingId => "capture_missing_id",
        PipelineProblemCauseV1::CaptureTamperedCache => "capture_tampered_cache",
        PipelineProblemCauseV1::CaptureRevisionConflict => "capture_revision_conflict",
        PipelineProblemCauseV1::CaptureWriteFailure => "capture_write_failure",
        PipelineProblemCauseV1::CaptureStatePersistenceFailure => {
            "capture_state_persistence_failure"
        }
        PipelineProblemCauseV1::CaptureCacheFailure => "capture_cache_failure",
        PipelineProblemCauseV1::HandoffUnsupportedTarget => "handoff_unsupported_target",
        PipelineProblemCauseV1::HandoffInvalidState => "handoff_invalid_state",
        PipelineProblemCauseV1::HandoffMissingInput => "handoff_missing_input",
        PipelineProblemCauseV1::HandoffInvalidProvenance => "handoff_invalid_provenance",
        PipelineProblemCauseV1::HandoffWriteFailure => "handoff_write_failure",
        PipelineProblemCauseV1::StateMalformed => "state_malformed",
        PipelineProblemCauseV1::StateUnsupportedVariable => "state_unsupported_variable",
        PipelineProblemCauseV1::StateRevisionConflict => "state_revision_conflict",
        PipelineProblemCauseV1::StateInvalidPipelineId => "state_invalid_pipeline_id",
        PipelineProblemCauseV1::StateInvalidSupportedVariables => {
            "state_invalid_supported_variables"
        }
        PipelineProblemCauseV1::StateInvalidMutation => "state_invalid_mutation",
        PipelineProblemCauseV1::StateReadFailure => "state_read_failure",
        PipelineProblemCauseV1::StateLockFailure => "state_lock_failure",
        PipelineProblemCauseV1::StateWriteFailure => "state_write_failure",
        PipelineProblemCauseV1::StateSerializationFailure => "state_serialization_failure",
        PipelineProblemCauseV1::IdempotencyInvalidOperation => "idempotency_invalid_operation",
        PipelineProblemCauseV1::IdempotencyLockFailure => "idempotency_lock_failure",
        PipelineProblemCauseV1::IdempotencyReadFailure => "idempotency_read_failure",
        PipelineProblemCauseV1::IdempotencyMalformedEntry => "idempotency_malformed_entry",
        PipelineProblemCauseV1::IdempotencyWriteFailure => "idempotency_write_failure",
        PipelineProblemCauseV1::IdempotencyConflict => "idempotency_conflict",
    }
}
fn non_empty(value: &str) -> Result<(), ContractError> {
    if value.trim().is_empty() {
        Err(ContractError::InvalidInvariant)
    } else {
        Ok(())
    }
}
fn to_u64(value: usize) -> Result<u64, ContractError> {
    u64::try_from(value).map_err(|_| ContractError::InvalidNumber)
}
fn portable_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
fn sanitize_detail(repo_root: &Path, detail: &str) -> String {
    detail
        .replace(repo_root.to_string_lossy().as_ref(), "<repository>")
        .replace('\\', "/")
}
fn schema_binding(reference: &str) -> Result<ExactBinding, ContractError> {
    super::artifact_operation::schema_binding(reference)
}
fn to_contract_value<T: Serialize>(value: &T) -> Result<ContractValue, ContractError> {
    super::artifact_operation::to_contract_value(value)
}
fn from_contract_value<T: for<'de> Deserialize<'de> + Serialize>(
    value: &ContractValue,
) -> Result<T, ContractError> {
    parse_canonical_json(
        &canonical_json_bytes(value, &ContractLimits::packet0())?,
        &ContractLimits::packet0(),
    )
}

fn validate_checked_schema(binding: &ExactBinding, instance: &[u8]) -> Result<(), ContractError> {
    let source = pipeline_schema_source(binding.reference().as_str())
        .ok_or(ContractError::InvalidInvariant)?;
    if binding.fingerprint() != &super::sha256_fingerprint(source.as_bytes()) {
        return Err(ContractError::InvalidInvariant);
    }
    let schema = serde_json::from_str::<serde_json::Value>(source)
        .map_err(|_| ContractError::InvalidJson)?;
    let instance = serde_json::from_slice::<serde_json::Value>(instance)
        .map_err(|_| ContractError::InvalidJson)?;
    let validator = jsonschema::draft202012::options()
        .with_pattern_options(jsonschema::PatternOptions::fancy_regex())
        .build(&schema)
        .map_err(|_| ContractError::InvalidInvariant)?;
    if validator.is_valid(&instance) {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
}

fn pipeline_schema_source(reference: &str) -> Option<&'static str> {
    Some(match reference {
        LIST_REQUEST_REF => include_str!("../../schemas/2020-12/handbook.operation.pipeline-catalog-list-request/1.0.0.schema.json"),
        LIST_RESULT_REF => include_str!("../../schemas/2020-12/handbook.operation.pipeline-catalog-list-result/1.0.0.schema.json"),
        READ_REQUEST_REF => include_str!("../../schemas/2020-12/handbook.operation.pipeline-catalog-read-request/1.0.0.schema.json"),
        READ_RESULT_REF => include_str!("../../schemas/2020-12/handbook.operation.pipeline-catalog-read-result/1.0.0.schema.json"),
        COMPILE_REQUEST_REF => include_str!("../../schemas/2020-12/handbook.operation.pipeline-compile-request/1.0.0.schema.json"),
        COMPILE_RESULT_REF => include_str!("../../schemas/2020-12/handbook.operation.pipeline-compile-result/1.0.0.schema.json"),
        CAPTURE_PLAN_REQUEST_REF => include_str!("../../schemas/2020-12/handbook.operation.pipeline-capture-plan-request/1.0.0.schema.json"),
        CAPTURE_PLAN_RESULT_REF => include_str!("../../schemas/2020-12/handbook.operation.pipeline-capture-plan-result/1.0.0.schema.json"),
        CAPTURE_APPLY_REQUEST_REF => include_str!("../../schemas/2020-12/handbook.operation.pipeline-capture-apply-request/1.0.0.schema.json"),
        CAPTURE_APPLY_RESULT_REF => include_str!("../../schemas/2020-12/handbook.operation.pipeline-capture-apply-result/1.0.0.schema.json"),
        HANDOFF_EMIT_REQUEST_REF => include_str!("../../schemas/2020-12/handbook.operation.pipeline-handoff-emit-request/1.0.0.schema.json"),
        HANDOFF_EMIT_RESULT_REF => include_str!("../../schemas/2020-12/handbook.operation.pipeline-handoff-emit-result/1.0.0.schema.json"),
        STATE_APPLY_REQUEST_REF => include_str!("../../schemas/2020-12/handbook.operation.pipeline-state-apply-request/1.0.0.schema.json"),
        STATE_APPLY_RESULT_REF => include_str!("../../schemas/2020-12/handbook.operation.pipeline-state-apply-result/1.0.0.schema.json"),
        PIPELINE_DETAILS_REF => include_str!("../../schemas/2020-12/handbook.pipeline-operation-details/1.0.0.schema.json"),
        PIPELINE_RECHECK_REF => include_str!("../../schemas/2020-12/handbook.pipeline-recheck/1.0.0.schema.json"),
        PIPELINE_WRITE_CONDITION_REF => include_str!("../../schemas/2020-12/handbook.pipeline-write-condition/1.0.0.schema.json"),
        _ => return None,
    })
}
