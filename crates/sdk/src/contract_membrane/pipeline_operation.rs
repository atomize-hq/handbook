use super::{
    canonical_json_bytes, parse_canonical_json, BoundedString, BoundedVec, CapabilityEntry,
    ContractError, ContractLimits, ContractValue, CorrelationRef, CurrentTransportEvidence,
    CurrentTransportEvidenceSealed, ExactBinding, ExactRef, IdempotencyState,
    OperationAdmissionResult, OperationClass, OperationDefinition, OperationIdempotency,
    OperationOutcomeSchemas, OperationOwnerBinding, OperationRequestV1, OperationResponseV1,
    OperationSchemaValidator, OperationTransport, OwnerAdmissionToken, Problem, ProblemCategory,
    RecheckCondition, RequestId, RetryDirective, SemVer, TerminalFields, TerminalStatus,
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
        load_pipeline_catalog_metadata, load_pipeline_selection_metadata, PipelineCatalogError,
        PipelineLookupError, PipelineMetadataSelectionError, PipelineSelection,
    },
    pipeline_compile::{
        compile_pipeline_stage, PipelineCompileDocumentKind, PipelineCompileDocumentStatus,
        PipelineCompileOutputKind, PipelineCompileRefusal, PipelineCompileRefusalClassification,
        PipelineCompileResult,
    },
    route_state::{RouteBasisStageReason, RouteBasisStageStatus},
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub(crate) const PIPELINE_LIST_REF: &str = "pipeline.catalog.list@1.0.0";
pub(crate) const PIPELINE_READ_REF: &str = "pipeline.catalog.read@1.0.0";
pub(crate) const PIPELINE_COMPILE_REF: &str = "pipeline.compile@1.0.0";
pub(crate) const PIPELINE_DETAILS_REF: &str = "handbook.pipeline-operation-details@1.0.0";
pub(crate) const PIPELINE_RECHECK_REF: &str = "handbook.pipeline-recheck@1.0.0";

const LIST_REQUEST_REF: &str = "handbook.operation.pipeline-catalog-list-request@1.0.0";
const LIST_RESULT_REF: &str = "handbook.operation.pipeline-catalog-list-result@1.0.0";
const READ_REQUEST_REF: &str = "handbook.operation.pipeline-catalog-read-request@1.0.0";
const READ_RESULT_REF: &str = "handbook.operation.pipeline-catalog-read-result@1.0.0";
const COMPILE_REQUEST_REF: &str = "handbook.operation.pipeline-compile-request@1.0.0";
const COMPILE_RESULT_REF: &str = "handbook.operation.pipeline-compile-result@1.0.0";

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
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PipelineProblemDetailsV1 {
    cause: PipelineProblemCauseV1,
    detail: String,
    selector: Option<String>,
    matches: Vec<String>,
    compile_refusal: Option<PipelineCompileRefusalV1>,
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

impl PipelineProblemDetailsV1 {
    fn validate(&self) -> Result<(), ContractError> {
        let compile_cause = is_compile_cause(self.cause);
        if compile_cause != self.compile_refusal.is_some() {
            return Err(ContractError::InvalidInvariant);
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
    [PIPELINE_LIST_REF, PIPELINE_READ_REF, PIPELINE_COMPILE_REF]
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
        OperationClass::ReadOnly,
        OperationIdempotency::safe(),
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(vec![
            OperationTransport::CliJson,
            OperationTransport::RustSdk,
            OperationTransport::Tauri,
        ])?,
        None,
        &limits,
    )
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
        },
    )
}

fn problem_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    details: PipelineProblemDetailsV1,
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
    let problem = Problem::new(
        BoundedString::new(format!("pipeline.{}", cause_name(details.cause)))?,
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
    response_from_parts(request, definition, terminal)
}

fn response_from_parts(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    terminal: TerminalFields<ContractValue>,
) -> Result<OperationResponseV1, ContractError> {
    OperationResponseV1::new(
        schema_binding("handbook.operation-response@1.0.0")?,
        request.request_id().cloned(),
        request.request_fingerprint().clone(),
        definition,
        terminal,
        IdempotencyState::not_applicable()?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        &PipelineOperationSchemaValidator,
        &ContractLimits::packet0(),
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

pub type PipelineOperationProblemProjection = (
    String,
    Option<PipelineSelectorRefusal>,
    Option<PipelineCompileRefusalView>,
);

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
            CapabilityEntry::new(
                &definition,
                &PipelineCliTransportEvidence { owner },
                &ContractLimits::packet0(),
            )
        })
        .collect()
}

struct PipelineCliTransportEvidence {
    owner: OwnerAdmissionToken,
}
impl CurrentTransportEvidenceSealed for PipelineCliTransportEvidence {}
impl CurrentTransportEvidence for PipelineCliTransportEvidence {
    fn owner_admission(&self) -> &OwnerAdmissionToken {
        &self.owner
    }
    fn current_transports(&self) -> &[OperationTransport] {
        &[OperationTransport::CliJson]
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
        _ => (
            TerminalStatus::Blocked,
            ProblemCategory::Prerequisite,
            RetryDirective::AfterRecheck,
        ),
    }
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
        PIPELINE_DETAILS_REF => include_str!("../../schemas/2020-12/handbook.pipeline-operation-details/1.0.0.schema.json"),
        PIPELINE_RECHECK_REF => include_str!("../../schemas/2020-12/handbook.pipeline-recheck/1.0.0.schema.json"),
        _ => return None,
    })
}
