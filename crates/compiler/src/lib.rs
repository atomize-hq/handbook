pub mod author;
// HCM-1.4 retires these setup/doctor consumers while sibling compatibility
// surfaces remain until their separately owned migration slices.
#[allow(dead_code)]
mod baseline_validation;
#[allow(dead_code)]
pub mod blocker;
mod canonical_artifacts;
mod charter_product;
pub mod decision_log;
mod doctor;
pub mod error;
// Keep the initial layout-owner seam compiler-internal until later slices
// prove the reviewed outward API surface we actually want to freeze.
#[allow(dead_code)]
mod layout;
mod profile_readiness;
pub mod refusal;
pub mod rendering;
// Keep the workspace seam compiler-internal until a downstream crate proves
// the minimal reviewed API surface we actually want to freeze.
mod repo_file_access;
pub mod resolver;
mod route_state;
mod setup;

// Slice 4.5 retained-seam posture: `handbook-compiler` remains a narrow
// compatibility/support crate for the unresolved CLI-facing seams that still
// span multiple owner crates. Engine-, flow-, and pipeline-owned logic stays in
// those crates, and direct callers should import those owners unless they are
// intentionally consuming the retained compiler support seam. This crate exposes
// only the reviewed CLI-facing support types and adapters at the compiler root
// and keeps no legacy owner-family facade modules alive in the source tree.
pub use author::template_library::{
    resolve_shipped_template_library, resolve_template_library, CharterTemplateLibraryOverride,
    EnvironmentInventoryTemplateLibraryOverride, TemplateLibraryAsset,
    TemplateLibraryOverrideRequest, TemplateLibraryRequest, TemplateLibraryResolveError,
    TemplateLibraryResolveErrorKind, TemplateLibraryResolveRequest, TemplateLibrarySelection,
};
pub use author::{
    author_charter, author_environment_inventory_from_input, author_project_context,
    author_project_context_from_input, is_unusably_vague_charter_text, normalize_charter_free_text,
    parse_charter_structured_input_yaml, parse_environment_inventory_structured_input_yaml,
    parse_project_context_input_yaml, preflight_author_charter,
    preflight_author_charter_from_input, preflight_author_environment_inventory,
    preflight_author_environment_inventory_from_input, preflight_author_project_context,
    render_charter_markdown, render_environment_inventory_markdown,
    render_project_context_markdown, validate_charter_markdown, validate_charter_structured_input,
    validate_environment_inventory_markdown, validate_environment_inventory_structured_input,
    validate_project_context_input, AuthorCharterRefusal, AuthorCharterRefusalKind,
    AuthorCharterResult, AuthorEnvironmentInventoryRefusal, AuthorEnvironmentInventoryRefusalKind,
    AuthorEnvironmentInventoryResult, AuthorProjectContextRefusal, AuthorProjectContextRefusalKind,
    AuthorProjectContextResult, CharterAudience, CharterBackwardCompatibility,
    CharterDebtTrackingInput, CharterDecisionRecordsInput, CharterDefaultImplicationsInput,
    CharterDeprecationPolicy, CharterDimensionInput, CharterDimensionName, CharterDomainInput,
    CharterExceptionsInput, CharterExpectedLifetime, CharterObservabilityThreshold,
    CharterOperationalRealityInput, CharterPostureInput, CharterProjectClassification,
    CharterProjectConstraintsInput, CharterProjectInput, CharterRequiredness,
    CharterRolloutControls, CharterRuntimeEnvironment, CharterStructuredInput, CharterSurface,
    EnvironmentCiInput, EnvironmentExternalServiceInput, EnvironmentInventoryStructuredInput,
    EnvironmentKnownUnknownInput, EnvironmentLocalDevelopmentInput, EnvironmentProductionInput,
    EnvironmentRuntimeAssumptionsInput, EnvironmentSecretHandlingInput, EnvironmentToolingInput,
    EnvironmentUpdateContractInput, EnvironmentVariableInput, CANONICAL_CHARTER_REPO_PATH,
    CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH, DEFAULT_EXCEPTION_RECORD_LOCATION,
};
pub use blocker::{blocker_category_priority, Blocker, BlockerCategory, C04_RESULT_VERSION};
pub(crate) use canonical_artifacts::{ArtifactIngestIssueKind, ArtifactPresence};
pub use charter_product::{
    execute_approver_admin_intent, execute_charter_command, invalid_charter_command_refusal,
    legacy_charter_input_refusal, parse_charter_intake_envelope, AdapterOperationStatus,
    AdapterRefusal, ApproverAdapterResult, ApproverAdminIntent, ApproverAdminOperation,
    CharterCommandIntent, CharterOperation, CharterOperationResult,
    APPROVER_ADAPTER_RESULT_SCHEMA_ID, APPROVER_ADAPTER_RESULT_SCHEMA_VERSION,
    CHARTER_OPERATION_RESULT_SCHEMA_ID, CHARTER_OPERATION_RESULT_SCHEMA_VERSION,
};
pub use decision_log::DecisionLog;
pub use doctor::{
    doctor, doctor_with_decisions, DoctorCharterDefinitionClosureStatus,
    DoctorCharterDefinitionRow, DoctorCharterLifecycleState, DoctorCharterNextAction,
    DoctorCharterRow, DoctorError, DoctorErrorKind, DoctorErrorReasonCode, DoctorProjectContextRow,
    DoctorReport, DOCTOR_REPORT_SCHEMA_ID, DOCTOR_REPORT_SCHEMA_VERSION,
};
pub use error::CompilerError;
pub(crate) use handbook_engine::artifact_manifest::{ArtifactManifest, ManifestError};
pub use handbook_engine::CanonicalProjectContext;
pub use handbook_engine::{ArtifactIngestError, CanonicalArtifactKind, SystemRootStatus};
pub(crate) use handbook_flow::BudgetOutcome;
pub(crate) use handbook_flow::PacketResult;
pub use profile_readiness::{
    ProfileArtifactRow, ProfileCapabilityRow, ProfileConditionRow, RepositoryReadinessStatus,
};
pub use refusal::{NextSafeAction, Refusal, RefusalCategory, SubjectRef};
pub use rendering::{
    build_output_model, render_blocker_category, render_json, render_next_safe_action_value,
    render_subject_ref, RenderError, RenderOutputModel, RenderSurface,
};
pub use resolver::{resolve, ResolverResult};
pub use setup::{
    plan_setup, plan_setup_with_decisions, run_setup, run_setup_with_decisions,
    SetupArtifactAction, SetupArtifactActionKind, SetupError, SetupErrorCode, SetupErrorKind,
    SetupErrorReasonCode, SetupMode, SetupOutcome, SetupPlan, SetupRequest, SetupRootAction,
};

pub fn workspace_contract_version() -> &'static str {
    handbook_engine::workspace_contract_version()
}
