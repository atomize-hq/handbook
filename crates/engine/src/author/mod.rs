pub mod charter_core;
pub mod environment_inventory_core;
pub mod project_context_core;

pub use charter_core::{
    compiler_owned_charter_markdown, find_charter_template_scaffold_line,
    is_unusably_vague_charter_text, normalize_charter_free_text,
    normalized_charter_structured_input, parse_charter_structured_input_yaml,
    render_charter_markdown, sanitize_charter_template, validate_charter_markdown,
    validate_charter_structured_input, validate_compiler_owned_charter_markdown,
    validate_required_heading_order_result as validate_charter_heading_order_result,
    CharterAudience, CharterBackwardCompatibility, CharterCoreError, CharterCoreErrorKind,
    CharterDebtTrackingInput, CharterDecisionRecordsInput, CharterDefaultImplicationsInput,
    CharterDeprecationPolicy, CharterDimensionInput, CharterDimensionName, CharterDomainInput,
    CharterExceptionsInput, CharterExpectedLifetime, CharterObservabilityThreshold,
    CharterOperationalRealityInput, CharterPostureInput, CharterProjectClassification,
    CharterProjectConstraintsInput, CharterProjectInput, CharterRequiredness,
    CharterRolloutControls, CharterRuntimeEnvironment, CharterStructuredInput, CharterSurface,
    DEFAULT_EXCEPTION_RECORD_LOCATION,
};
#[allow(deprecated)]
pub use environment_inventory_core::{
    parse_environment_inventory_structured_input_yaml, render_environment_inventory_markdown,
    validate_environment_inventory_markdown, validate_environment_inventory_structured_input,
    validate_required_heading_order_result as validate_environment_inventory_heading_order_result,
    validate_synthesized_environment_inventory_markdown, EnvironmentCiInput,
    EnvironmentExternalServiceInput, EnvironmentInventoryCoreError,
    EnvironmentInventoryCoreErrorKind, EnvironmentInventoryStructuredInput,
    EnvironmentInventoryValidationExpectations, EnvironmentKnownUnknownInput,
    EnvironmentLocalDevelopmentInput, EnvironmentProductionInput,
    EnvironmentRuntimeAssumptionsInput, EnvironmentSecretHandlingInput, EnvironmentToolingInput,
    EnvironmentUpdateContractInput, EnvironmentVariableInput,
    REQUIRED_ENVIRONMENT_INVENTORY_HEADINGS,
};
pub use project_context_core::{
    collect_render_safety_issues, normalize_project_context_text,
    normalized_project_context_structured_input, parse_project_context_structured_input_yaml,
    render_project_context_markdown, validate_known_fake_project_context_markers,
    validate_project_context_markdown, validate_project_context_structured_input,
    ProjectContextClassificationImplicationsInput, ProjectContextConstraintsInput,
    ProjectContextCoreError, ProjectContextCoreErrorKind, ProjectContextDataRealityInput,
    ProjectContextEnvironmentsAndDeliveryInput, ProjectContextIntegrationInput,
    ProjectContextKnownUnknownInput, ProjectContextOperationalRealityInput,
    ProjectContextRepoCodebaseRealityInput, ProjectContextStructuredInput,
    ProjectContextSummaryInput, ProjectContextSystemBoundariesInput, ProjectContextValidationError,
};
