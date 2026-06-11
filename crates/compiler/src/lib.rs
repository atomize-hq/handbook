pub mod artifact_manifest;
pub mod author;
mod baseline_validation;
pub mod blocker;
pub mod budget;
pub mod canonical_artifacts;
pub mod decision_log;
pub mod declarative_roots;
pub mod doctor;
mod doctor_shell;
pub mod error;
pub mod freshness;
// Keep the initial layout-owner seam compiler-internal until later slices
// prove the reviewed outward API surface we actually want to freeze.
mod layout;
pub mod packet_result;
pub mod pipeline;
pub mod pipeline_capture;
pub mod pipeline_compile;
pub mod pipeline_handoff;
pub mod pipeline_route;
pub mod refusal;
pub mod rendering;
// Keep the workspace seam compiler-internal until a downstream crate proves
// the minimal reviewed API surface we actually want to freeze.
mod repo_file_access;
pub mod resolver;
pub mod route_state;
pub mod setup;
mod setup_shell;
mod stage_10_feature_spec_provenance;

pub use artifact_manifest::{
    ArtifactManifest, ManifestError, ManifestInputs, ManifestVersion, SchemaVersion,
};
pub use author::{
    author_charter, author_charter_guided, author_environment_inventory, author_project_context,
    author_project_context_from_input, is_unusably_vague_charter_text, normalize_charter_free_text,
    parse_charter_structured_input_yaml, parse_project_context_structured_input_yaml,
    preflight_author_charter, preflight_author_charter_from_input,
    preflight_author_environment_inventory, preflight_author_project_context,
    render_charter_markdown, render_project_context_markdown, validate_charter_markdown,
    validate_charter_structured_input, validate_environment_inventory_markdown,
    validate_project_context_markdown, validate_project_context_structured_input,
    AuthorCharterRefusal, AuthorCharterRefusalKind, AuthorCharterResult,
    AuthorEnvironmentInventoryRefusal, AuthorEnvironmentInventoryRefusalKind,
    AuthorEnvironmentInventoryResult, AuthorProjectContextRefusal, AuthorProjectContextRefusalKind,
    AuthorProjectContextResult, CharterAudience, CharterBackwardCompatibility,
    CharterDebtTrackingInput, CharterDecisionRecordsInput, CharterDefaultImplicationsInput,
    CharterDeprecationPolicy, CharterDimensionInput, CharterDimensionName, CharterDomainInput,
    CharterExceptionsInput, CharterExpectedLifetime, CharterObservabilityThreshold,
    CharterOperationalRealityInput, CharterPostureInput, CharterProjectClassification,
    CharterProjectConstraintsInput, CharterProjectInput, CharterRequiredness,
    CharterRolloutControls, CharterRuntimeEnvironment, CharterStructuredInput, CharterSurface,
    ProjectContextClassificationImplicationsInput, ProjectContextConstraintsInput,
    ProjectContextDataRealityInput, ProjectContextEnvironmentsAndDeliveryInput,
    ProjectContextIntegrationInput, ProjectContextKnownUnknownInput,
    ProjectContextOperationalRealityInput, ProjectContextRepoCodebaseRealityInput,
    ProjectContextStructuredInput, ProjectContextSummaryInput, ProjectContextSystemBoundariesInput,
    ProjectContextValidationError, CANONICAL_CHARTER_REPO_PATH,
    CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH, CANONICAL_PROJECT_CONTEXT_REPO_PATH,
    DEFAULT_EXCEPTION_RECORD_LOCATION,
};
pub use blocker::{blocker_category_priority, Blocker, BlockerCategory, C04_RESULT_VERSION};
pub use budget::{
    BudgetDisposition, BudgetOutcome, BudgetPolicy, BudgetReason,
    NextSafeAction as BudgetNextSafeAction,
};
pub use canonical_artifacts::{
    canonical_artifact_descriptors, matches_setup_starter_template, setup_starter_template,
    setup_starter_template_bytes, ArtifactIngestError, ArtifactIngestIssue,
    ArtifactIngestIssueKind, ArtifactPresence, CanonicalArtifact, CanonicalArtifactDescriptor,
    CanonicalArtifactIdentity, CanonicalArtifactKind, CanonicalArtifacts, SystemRootStatus,
};
pub use decision_log::DecisionLog;
pub use doctor::{
    doctor, doctor_from_artifacts, DoctorArtifactStatus, DoctorBaselineStatus, DoctorChecklistItem,
    DoctorReport,
};
pub use error::CompilerError;
pub use freshness::{
    compute_freshness, FreshnessIssue, FreshnessIssueKind, FreshnessStatus, FreshnessTruth,
    InheritedDependency, OverrideTarget, OverrideWithRationale, C03_SCHEMA_VERSION,
    MANIFEST_GENERATION_VERSION,
};
pub use packet_result::PacketResult;
pub use pipeline::{
    load_pipeline_catalog, load_pipeline_catalog_metadata, load_pipeline_definition,
    load_pipeline_selection_metadata, load_stage_compile_definition, render_pipeline_list,
    render_pipeline_show, resolve_pipeline_only_selector, resolve_pipeline_selector,
    supported_route_state_variables, ActivationClause, ActivationConditionSet, ActivationOperator,
    ActivationValidationError, CompileStageDefinition, CompileStageGating, CompileStageInput,
    CompileStageInputs, CompileStageLoadError, CompileStageOutput, CompileStageOutputs,
    CompileStageVariable, PipelineBody, PipelineCatalog, PipelineCatalogEntry,
    PipelineCatalogError, PipelineCatalogStageEntry, PipelineDefaults, PipelineDefinition,
    PipelineHeader, PipelineLoadError, PipelineLookupError, PipelineMetadataSelectionError,
    PipelineSelection, PipelineStage, PipelineValidationError, StageActivation, StageCatalogEntry,
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
pub use refusal::{NextSafeAction, Refusal, RefusalCategory, SubjectRef};
pub use rendering::{
    build_output_model, render_blocker_category, render_inspect, render_json, render_markdown,
    render_next_safe_action_value, render_subject_ref, RenderError, RenderOutputModel,
    RenderSurface,
};
pub use resolver::{
    resolve, PacketSelection, PacketSelectionStatus, ResolveRequest, ResolverResult,
};
pub use route_state::{
    build_route_basis, effective_route_basis_run, load_route_state,
    load_route_state_with_supported_variables, persist_route_basis, set_route_state, RouteBasis,
    RouteBasisActivationOperator, RouteBasisBuildError, RouteBasisPersistOutcome,
    RouteBasisPersistRefusal, RouteBasisProfilePack, RouteBasisResolvedStage, RouteBasisRunner,
    RouteBasisStageReason, RouteBasisStageStatus, RouteState, RouteStateAuditEntry,
    RouteStateMutation, RouteStateMutationOutcome, RouteStateMutationRefusal, RouteStateReadError,
    RouteStateRefs, RouteStateRun, RouteStateStoreError, RouteStateValue,
    ROUTE_BASIS_REPO_ROOT_SENTINEL, ROUTE_BASIS_SCHEMA_VERSION, ROUTE_STATE_AUDIT_LIMIT,
    ROUTE_STATE_SCHEMA_VERSION,
};
pub use setup::{
    plan_setup, run_setup, SetupAction, SetupActionLabel, SetupDisposition, SetupMode,
    SetupOutcome, SetupPlan, SetupRefusal, SetupRefusalKind, SetupRequest,
};

pub fn workspace_contract_version() -> &'static str {
    handbook_engine::workspace_contract_version()
}
