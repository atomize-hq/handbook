#![forbid(unsafe_code)]

pub mod artifact_instance;
pub mod artifact_kind_registry;
pub mod artifact_manifest;
pub mod author;
pub mod baseline_validation;
pub mod canonical_artifacts;
mod canonical_paths;
mod canonical_repo_support;
pub mod context_resolution_registry;
pub mod definition_identity;
pub mod freshness;
pub mod instance_profile;
pub mod project_condition_registry;
pub mod schema_registry;
pub mod semantic_capability_registry;
pub mod stable_role_registry;
pub mod vocabulary_registry;

pub use artifact_instance::{
    shipped_root_artifact_instance_values, ArtifactInstanceDescriptor, ArtifactInstanceRegistry,
    RequirednessMode,
};
pub use artifact_kind_registry::{
    load_artifact_kind_registry, ArtifactKindCapability, ArtifactKindDefinition,
    ArtifactKindRegistry, ArtifactKindRegistryLoadRequest,
};
pub use artifact_manifest::{
    ArtifactManifest, ManifestError, ManifestInputs, ManifestVersion, SchemaVersion,
};
pub use author::{
    parse_charter_structured_input_yaml, parse_environment_inventory_structured_input_yaml,
    parse_project_context_structured_input_yaml, render_charter_markdown,
    render_environment_inventory_markdown, render_project_context_markdown,
    validate_charter_markdown, validate_charter_structured_input,
    validate_environment_inventory_markdown, validate_environment_inventory_structured_input,
    validate_project_context_markdown, validate_project_context_structured_input, CharterAudience,
    CharterBackwardCompatibility, CharterCoreError, CharterCoreErrorKind, CharterDebtTrackingInput,
    CharterDecisionRecordsInput, CharterDefaultImplicationsInput, CharterDeprecationPolicy,
    CharterDimensionInput, CharterDimensionName, CharterDomainInput, CharterExceptionsInput,
    CharterExpectedLifetime, CharterObservabilityThreshold, CharterOperationalRealityInput,
    CharterPostureInput, CharterProjectClassification, CharterProjectConstraintsInput,
    CharterProjectInput, CharterRequiredness, CharterRolloutControls, CharterRuntimeEnvironment,
    CharterStructuredInput, CharterSurface, EnvironmentCiInput, EnvironmentExternalServiceInput,
    EnvironmentInventoryCoreError, EnvironmentInventoryCoreErrorKind,
    EnvironmentInventoryStructuredInput, EnvironmentKnownUnknownInput,
    EnvironmentLocalDevelopmentInput, EnvironmentProductionInput,
    EnvironmentRuntimeAssumptionsInput, EnvironmentSecretHandlingInput, EnvironmentToolingInput,
    EnvironmentUpdateContractInput, EnvironmentVariableInput,
    ProjectContextClassificationImplicationsInput, ProjectContextConstraintsInput,
    ProjectContextCoreError, ProjectContextCoreErrorKind, ProjectContextDataRealityInput,
    ProjectContextEnvironmentsAndDeliveryInput, ProjectContextIntegrationInput,
    ProjectContextKnownUnknownInput, ProjectContextOperationalRealityInput,
    ProjectContextRepoCodebaseRealityInput, ProjectContextStructuredInput,
    ProjectContextSummaryInput, ProjectContextSystemBoundariesInput, ProjectContextValidationError,
    DEFAULT_EXCEPTION_RECORD_LOCATION,
};
pub use baseline_validation::{
    baseline_artifact_validation, baseline_artifact_validation_for_path,
    baseline_artifact_validations, BaselineArtifactValidation, BaselineArtifactVerdict,
};
pub use canonical_artifacts::{
    canonical_artifact_descriptors, matches_setup_starter_template, setup_starter_template,
    setup_starter_template_bytes, ArtifactIngestError, ArtifactIngestIssue,
    ArtifactIngestIssueKind, ArtifactPresence, CanonicalArtifact, CanonicalArtifactDescriptor,
    CanonicalArtifactIdentity, CanonicalArtifactKind, CanonicalArtifacts, SystemRootStatus,
};
pub use canonical_paths::{default_canonical_layout_contract, CanonicalLayoutContract};
pub use context_resolution_registry::{
    ContextResolutionPolicyRegistry, ContextResolutionStackDefinition,
};
pub use definition_identity::{
    parse_definition_yaml, parse_schema_json, DefinitionFingerprint, ExactDefinitionRef,
    RegistryLoadError, RegistryLoadErrorKind, SourceByteBudget, MAX_SOURCE_DOCUMENT_BYTES,
    MAX_TOTAL_SOURCE_BYTES,
};
pub use freshness::{
    compute_freshness, FreshnessIssue, FreshnessIssueKind, FreshnessStatus, FreshnessTruth,
    InheritedDependency, OverrideTarget, OverrideWithRationale, C03_SCHEMA_VERSION,
    MANIFEST_GENERATION_VERSION,
};
pub use instance_profile::{
    DefinitionSource, DefinitionSourceBinding, ProfileLoadError, ProfileLoadErrorKind,
    ProfileSelectionRequest, SymbolicId,
};
pub use project_condition_registry::ProjectConditionDefinition;
pub use schema_registry::{
    ResolvedSchema, SchemaRegistry, SchemaRegistryEntry, StructuralValidationError,
};
pub use semantic_capability_registry::{
    AllowedInstanceCardinality, BindingCardinality, BindingEmptyPolicy, BindingJsonType,
    SemanticBindingRule, SemanticCapabilityDefinition, SemanticCapabilityRegistry,
    SemanticValidationProfileDefinition,
};
pub use stable_role_registry::{StableRoleCategory, StableRoleDefinition, StableRoleRegistry};
pub use vocabulary_registry::VocabularyDefinition;

pub fn workspace_contract_version() -> &'static str {
    "C-02"
}

pub fn engine_contract_version() -> &'static str {
    workspace_contract_version()
}
