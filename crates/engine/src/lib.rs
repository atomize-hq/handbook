#![forbid(unsafe_code)]

pub mod approver_registry;
pub mod approver_registry_mutation;
pub mod approver_registry_observation;
pub mod artifact_instance;
pub mod artifact_kind_registry;
pub mod artifact_manifest;
pub mod artifact_registry;
pub mod author;
pub mod baseline_validation;
pub mod canonical_artifacts;
mod canonical_paths;
mod canonical_repo_support;
pub mod charter_approval_workflow;
pub mod charter_artifact;
pub mod charter_authenticator;
mod charter_authority_transaction;
pub mod charter_authority_workflow;
pub mod charter_definition_registry;
pub mod charter_intake;
pub mod charter_lifecycle;
pub mod charter_lifecycle_store;
pub mod charter_lineage_store;
pub mod charter_observation;
pub mod charter_promotion_workflow;
pub mod charter_runtime_vectors;
pub mod context_resolution_registry;
pub mod definition_identity;
pub mod freshness;
pub mod instance_profile;
mod profile_builtins;
pub mod profile_decision;
pub mod profile_inspection;
pub mod profile_selection;
pub mod project_condition_registry;
pub mod project_context_artifact;
pub mod repository_invocation_identity;
pub mod schema_registry;
pub mod semantic_capability_registry;
pub mod stable_role_registry;
pub mod vocabulary_registry;

pub use approver_registry::{
    ApproverAdminJsonErrorV1, ApproverAdminRequestV1, ApproverAdminResultV1,
    ApproverRegistryServiceV1,
};
pub use approver_registry_observation::{
    observe_committed_approver_registry, ApproverAuthorityPairV1,
    ApproverRegistryObservationErrorKindV1, ApproverRegistryObservationErrorV1,
    CommittedApproverCredentialV1, CommittedApproverRegistryObservationV1,
};
pub use artifact_instance::{
    shipped_root_artifact_instance_values, ArtifactDependency, ArtifactInstanceDescriptor,
    ArtifactInstanceRegistry, ArtifactRequiredness, DependencyCardinality, DependencyTargetKind,
    RequirednessMode,
};
pub use artifact_kind_registry::{
    load_artifact_kind_registry, ArtifactKindCapability, ArtifactKindDefinition,
    ArtifactKindRegistry, ArtifactKindRegistryLoadRequest,
};
pub use artifact_manifest::{
    ArtifactManifest, ManifestError, ManifestInputs, ManifestVersion, SchemaVersion,
};
pub use artifact_registry::{
    ArtifactRegistryValidationError, ResolvedArtifactCapability, ResolvedArtifactDependency,
    ResolvedArtifactInstance, ResolvedArtifactKind, ResolvedArtifactRegistry,
};
pub use author::{
    parse_charter_structured_input_yaml, parse_environment_inventory_structured_input_yaml,
    render_charter_markdown, render_environment_inventory_markdown, validate_charter_markdown,
    validate_charter_structured_input, validate_environment_inventory_markdown,
    validate_environment_inventory_structured_input, CharterAudience, CharterBackwardCompatibility,
    CharterCoreError, CharterCoreErrorKind, CharterDebtTrackingInput, CharterDecisionRecordsInput,
    CharterDefaultImplicationsInput, CharterDeprecationPolicy, CharterDimensionInput,
    CharterDimensionName, CharterDomainInput, CharterExceptionsInput, CharterExpectedLifetime,
    CharterObservabilityThreshold, CharterOperationalRealityInput, CharterPostureInput,
    CharterProjectClassification, CharterProjectConstraintsInput, CharterProjectInput,
    CharterRequiredness, CharterRolloutControls, CharterRuntimeEnvironment, CharterStructuredInput,
    CharterSurface, EnvironmentCiInput, EnvironmentExternalServiceInput,
    EnvironmentInventoryCoreError, EnvironmentInventoryCoreErrorKind,
    EnvironmentInventoryStructuredInput, EnvironmentKnownUnknownInput,
    EnvironmentLocalDevelopmentInput, EnvironmentProductionInput,
    EnvironmentRuntimeAssumptionsInput, EnvironmentSecretHandlingInput, EnvironmentToolingInput,
    EnvironmentUpdateContractInput, EnvironmentVariableInput, DEFAULT_EXCEPTION_RECORD_LOCATION,
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
pub use charter_approval_workflow::{
    CharterApprovalRefusalCodeV1, CharterApprovalRefusalV1, CharterApprovalRequestV1,
    CharterApprovalResultV1, CharterApprovalServiceV1, CharterApprovalSuccessV1,
};
pub use charter_artifact::{
    charter_rendered_fingerprint, charter_source_fingerprint, parse_canonical_charter,
    render_canonical_charter_markdown, serialize_canonical_charter,
    validate_canonical_charter_semantics, CanonicalCharter, CanonicalCharterConstraints,
    CanonicalCharterDebt, CanonicalCharterDecisionRecords, CanonicalCharterDefaultImplications,
    CanonicalCharterDimension, CanonicalCharterDomain, CanonicalCharterEngineeringPosture,
    CanonicalCharterExceptionProcess, CanonicalCharterGovernance,
    CanonicalCharterOperationalReality, CanonicalCharterPolicy, CanonicalCharterPosture,
    CanonicalCharterProject, CharterArtifactError, CharterArtifactErrorKind,
};
pub use charter_authenticator::{
    decode_and_verify_get_assertion_response, decode_make_credential_response,
    derive_authenticator_user_handle, encode_get_assertion_request, encode_make_credential_request,
    map_ctap_status, select_eligible_credentials, verify_es256_signature,
    AuthenticatorCredentialV1, AuthenticatorErrorV1, AuthenticatorUserHandleV1,
    CredentialSelectionCandidateV1, CtapRefusalCodeV1, CtapRefusalV1, CtapStatusOutcomeV1,
    GetAssertionResponseV1, MakeCredentialResponseV1, NativeAuthenticatorPortErrorV1,
    NativeAuthenticatorPortV1, AUTHENTICATOR_RP_ID, CTAP_GET_ASSERTION_COMMAND,
    CTAP_MAKE_CREDENTIAL_COMMAND, MAX_ALLOW_LIST_ITEMS, MAX_CREDENTIAL_ID_BYTES,
};
pub use charter_authority_transaction::{
    CharterAuthorityTransactionServiceV1, CharterPromotionCommitV1, CharterPromotionErrorKindV1,
    CharterPromotionErrorV1, CommittedCharterAuthorityV1,
};
pub use charter_authority_workflow::{
    CharterAuthorPersistenceErrorKindV1, CharterAuthorPersistenceErrorV1,
    CharterAuthorPersistenceResultV1, CharterAuthorPersistenceServiceV1,
};
pub use charter_definition_registry::{
    load_shipped_charter_definition_registry, CharterDefinitionRecord, CharterDefinitionRegistry,
};
pub use charter_intake::{
    evaluate_charter_intake, CharterAcquisitionMode, CharterCandidateBundle, CharterCandidateV11,
    CharterCoverageResult, CharterCoverageSubmission, CharterFieldSource, CharterIntakeConsumer,
    CharterIntakeEnvelope, CharterIntakeError, CharterIntakeErrorKind, CharterIntakeRecordV11,
    CharterIntakeSourceKind,
};
pub use charter_lifecycle::{
    address_charter_lifecycle, apply_charter_lifecycle_events, CharterLifecycleError,
    CharterLifecycleErrorKind, CharterLifecycleEvent, CharterLifecycleEventKind,
    CharterLifecycleObservation, CharterLifecycleState, CharterLifecycleTransition,
};
pub use charter_lifecycle_store::{
    charter_lifecycle_event_commit_marker, charter_lifecycle_state_fingerprint,
    classify_lifecycle_recovery, CharterLifecycleAuthorityV1, CharterLifecycleEventCommitV1,
    CharterLifecycleEventDispositionV1, CharterLifecycleEventIntentV1,
    CharterLifecycleFaultPointV1, CharterLifecycleRecoveryActionV1,
    CharterLifecycleRecoveryInputsV1, CharterLifecycleStoreErrorKindV1,
    CharterLifecycleStoreErrorV1, CharterLifecycleStoreV1,
};
pub use charter_lineage_store::{
    AppendDispositionV1, CandidateBundlePersistenceV1, LineageAppendResultV1, LineageRecordClassV1,
    LineageStoreErrorKindV1, LineageStoreErrorV1, TrustedLineageStoreV1,
    MAX_CANDIDATE_CONTENT_BYTES, MAX_LINEAGE_RECORD_BYTES, MAX_LINEAGE_REFERENCE_BYTES,
};
pub use charter_observation::{
    load_selected_charter, CanonicalCharterProjection, SelectedCharterLoadError,
};
pub use charter_promotion_workflow::{
    normalize_required_approval_pairs, CharterPromotionIntentV1, CharterPromotionWorkflowCommitV1,
    CharterPromotionWorkflowErrorKindV1, CharterPromotionWorkflowErrorV1,
    CharterPromotionWorkflowServiceV1,
};
pub use charter_runtime_vectors::{
    validate_runtime_record_fingerprint_vectors, RuntimeRecordVectorError,
    RuntimeRecordVectorReport,
};
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
    layer_profile_sources, parse_profile_source, AuthoredProfileSource, DefinitionSource,
    DefinitionSourceBinding, InstanceProfileDefinition, LayerDisposition, LayeredProfile,
    ProfileField, ProfileLayerDecision, ProfileLoadError, ProfileLoadErrorKind, ProfileScope,
    ProfileSelectionRequest, SymbolicId,
};
pub use profile_decision::{
    resolve_shipped_profile_decisions, ArtifactApplicability, ArtifactProfileDecision,
    ProfileCapabilityTruth, ProfileDecisionError, ProjectConditionDecisionReason,
    ProjectConditionEvaluation, ProjectConditionOutcome, ResolvedProfileDecisions,
    ShippedProfileDecisionError,
};
pub use profile_inspection::{
    inspect_profile_repository, inspect_profile_repository_with_stability_hook,
    load_selected_project_context, ArtifactInspection, ArtifactInspectionReason,
    ArtifactInspectionStatus, CanonicalProjectContextProjection, ProfileInspectionReport,
    SelectedProjectContextLoadError,
};
pub use profile_selection::{resolve_profile_selection, ResolvedInstanceProfile};
pub use project_condition_registry::{ProjectConditionDefinition, ProjectConditionRegistry};
pub use project_context_artifact::{
    parse_canonical_project_context, project_context_rendered_fingerprint,
    project_context_source_fingerprint, render_project_context_markdown,
    serialize_canonical_project_context, CanonicalProjectContext, ProjectContextArtifactError,
    ProjectContextArtifactErrorKind,
};
pub use repository_invocation_identity::{
    RepositoryIdentityAllocationErrorV1, RepositoryIdentityAllocationV1,
    RepositoryIdentityInitializationDispositionV1, RepositoryIdentityInitializationV1,
    RepositoryIdentitySetupErrorKindV1, RepositoryIdentitySetupErrorV1,
    RepositoryInvocationContractErrorV1, RepositoryInvocationIdentityServiceV1,
    RepositoryInvocationIdentityV1, RepositoryInvocationOperationV1,
    RepositoryInvocationPreflightResultV1, RepositoryInvocationPreparationFailureV1,
    RepositoryInvocationRecoveryRefusalV1, RepositoryInvocationRefusalV1,
    REPOSITORY_IDENTITY_REPO_PATH,
};
pub use schema_registry::{
    ResolvedSchema, SchemaRegistry, SchemaRegistryEntry, StructuralValidationError,
};
pub use semantic_capability_registry::{
    AllowedInstanceCardinality, BindingCardinality, BindingEmptyPolicy, BindingJsonType,
    SemanticBindingRule, SemanticCapabilityContract, SemanticCapabilityDefinition,
    SemanticCapabilityRegistry, SemanticValidationProfileDefinition,
};
pub use stable_role_registry::{StableRoleCategory, StableRoleDefinition, StableRoleRegistry};
pub use vocabulary_registry::VocabularyDefinition;

pub fn workspace_contract_version() -> &'static str {
    "C-02"
}

pub fn engine_contract_version() -> &'static str {
    workspace_contract_version()
}
