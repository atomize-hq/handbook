pub mod declarative_roots;
mod layout;
pub mod pipeline;
pub mod pipeline_capture;
pub mod pipeline_compile;
pub mod pipeline_handoff;
pub mod pipeline_route;
mod repo_file_access;
pub mod route_state;
pub mod setup;
mod stage_10_feature_spec_provenance;

pub use declarative_roots::PipelineDeclarativeRootsContract;
pub use layout::{
    handbook_product_pipeline_storage_layout_contract, PipelineStorageLayoutContract,
};
pub use pipeline::{
    handbook_product_pipeline_declarative_roots, load_pipeline_catalog,
    load_pipeline_catalog_metadata, load_pipeline_definition, load_pipeline_selection_metadata,
    load_stage_compile_definition, render_pipeline_list, render_pipeline_show,
    resolve_pipeline_only_selector, resolve_pipeline_selector, supported_route_state_variables,
    ActivationClause, ActivationConditionSet, ActivationOperator, ActivationValidationError,
    CompileStageDefinition, CompileStageGating, CompileStageInput, CompileStageInputs,
    CompileStageLoadError, CompileStageOutput, CompileStageOutputs, CompileStageVariable,
    PipelineBody, PipelineCatalog, PipelineCatalogEntry, PipelineCatalogError,
    PipelineCatalogStageEntry, PipelineDefaults, PipelineDefinition, PipelineHeader,
    PipelineLoadError, PipelineLookupError, PipelineMetadataSelectionError, PipelineSelection,
    PipelineStage, PipelineValidationError, StageActivation, StageCatalogEntry,
    StageFileValidationError,
};
pub use pipeline_capture::{
    apply_cached_pipeline_capture, apply_pipeline_capture, capture_pipeline_output,
    load_pipeline_capture_cache_entry, preview_pipeline_capture,
    render_pipeline_capture_apply_result, render_pipeline_capture_preview,
    render_pipeline_capture_refusal, PipelineCaptureApplyResult, PipelineCaptureCacheEntry,
    PipelineCapturePlan, PipelineCapturePreview, PipelineCaptureRefusal,
    PipelineCaptureRefusalClassification, PipelineCaptureRequest, PipelineCaptureStateEffect,
    PipelineCaptureStateUpdate, PipelineCaptureStateValue, PipelineCaptureTarget,
    PipelineCaptureWrite, PipelineCaptureWriteIntent, PIPELINE_CAPTURE_CACHE_SCHEMA_VERSION,
};
pub use pipeline_compile::{
    compile_pipeline_stage, compile_pipeline_stage_with_runtime, render_pipeline_compile_explain,
    render_pipeline_compile_payload, PipelineCompileDocument, PipelineCompileDocumentKind,
    PipelineCompileDocumentStatus, PipelineCompileGatingSummary, PipelineCompileOutput,
    PipelineCompileOutputKind, PipelineCompileRefusal, PipelineCompileRefusalClassification,
    PipelineCompileResult, PipelineCompileRuntimeContext, PipelineCompileTarget,
    PipelineCompileVariable, PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
};
pub use pipeline_handoff::{
    emit_pipeline_handoff_bundle, render_pipeline_handoff_emit_result,
    render_pipeline_handoff_refusal, validate_pipeline_handoff_bundle,
    PipelineHandoffCanonicalArtifactFingerprint, PipelineHandoffCanonicalProvenance,
    PipelineHandoffEmitRequest, PipelineHandoffEmitResult, PipelineHandoffFallbackMetadata,
    PipelineHandoffFeatureSpecCompileProvenance, PipelineHandoffInput, PipelineHandoffManifest,
    PipelineHandoffProducer, PipelineHandoffReadAllowlist, PipelineHandoffRefusal,
    PipelineHandoffRefusalClassification, PipelineHandoffRouteBasisProvenance,
    PipelineHandoffTrustClass, PipelineHandoffValidatedBundle, PipelineHandoffValidationFailure,
    PipelineHandoffValidationFailureClassification,
};
pub use pipeline_route::{
    resolve_pipeline_route, ResolvedPipelineRoute, ResolvedPipelineStage, RouteEvaluationError,
    RouteStageReason, RouteStageStatus, RouteVariables,
};
pub use route_state::{
    build_route_basis, effective_route_basis_run, load_route_state,
    load_route_state_with_supported_variables, persist_route_basis, set_route_state, RouteBasis,
    RouteBasisActivationOperator, RouteBasisBuildError, RouteBasisPersistOutcome,
    RouteBasisPersistRefusal, RouteBasisProfilePack, RouteBasisResolvedStage, RouteBasisRunner,
    RouteBasisStageReason, RouteBasisStageStatus, RouteState, RouteStateAuditEntry,
    RouteStateMutation, RouteStateMutationOutcome, RouteStateMutationRefusal, RouteStateReadError,
    RouteStateRefs, RouteStateRun, RouteStateStoreError, RouteStateValue,
    TrustedPipelineSessionRefusal, ROUTE_BASIS_REPO_ROOT_SENTINEL, ROUTE_BASIS_SCHEMA_VERSION,
    ROUTE_STATE_AUDIT_LIMIT, ROUTE_STATE_SCHEMA_VERSION,
};

pub fn pipeline_contract_version() -> &'static str {
    handbook_engine::workspace_contract_version()
}
