mod artifact;
mod artifact_operation;
mod binding;
mod bootstrap;
mod bootstrap_envelope;
mod bounds;
mod catalog;
mod consumer;
mod contract_evidence_operation;
mod contract_operation;
mod dock_operation;
mod dto;
mod envelope;
mod facade;
mod flow_operation;
mod invocation;
mod invocation_dispatch;
mod operation;
mod owner_admission;
mod pipeline_operation;
mod posture;
mod production_witness;
mod raw_key;
mod response;
#[cfg(feature = "contract-schema-generation")]
#[doc(hidden)]
pub mod schema_generation;
mod serialization;
mod session_grounding;
mod typed_catalog;
mod verification;

pub use artifact::{
    ArtifactLocator, ArtifactLocatorRef, ArtifactRef, DigestAlgorithm, Sensitivity,
};
pub(crate) use artifact_operation::execute_artifact_operation;
pub use artifact_operation::{
    artifact_operation_definition, artifact_operation_definitions,
    artifact_operation_problem_detail, artifact_operation_request,
    render_artifact_operation_legacy, ArtifactOperationInputV1,
};
pub use binding::{ExactBinding, ExactRef, Fingerprint, SemVer};
pub use bootstrap::{
    BootstrapCeilings, BootstrapDescriptorV1, BootstrapRefusalReason, BootstrapRequestV1,
    BootstrapSchemaBindings, CapabilityEntry, CurrentTransportEvidence, OwnerVersionEntry,
    ProfileCatalogEntry,
};
pub use bootstrap_envelope::{
    BootstrapRefusalDetails, BootstrapRefusalV1, BootstrapResponseV1, CapabilityCatalogPage,
    CatalogReadBlockedV1, CatalogRoots, ProfileCatalogPage, SchemaCatalogPage,
};
pub use bounds::{
    BoundedString, BoundedVec, ContractLimits, ContractMember, ContractValue, ContractValueRef,
    SafeI64, SafeU64, UtcTimestamp,
};
pub use catalog::{CatalogCursor, CatalogPage, CatalogRoot};
pub use consumer::{CapabilityConsumer, CapabilityConsumerError};
pub use contract_evidence_operation::{
    contract_evidence_append_operation_request, contract_evidence_list_operation_request,
    contract_evidence_operation_definition, contract_evidence_operation_definitions,
    contract_evidence_read_operation_request, contract_gate_evaluate_operation_request,
    contract_verdict_evaluate_operation_request, execute_contract_evidence_operation,
    execute_contract_evidence_operation_with_context, ContractClaimVerdictDtoV1,
    ContractEvaluationRunDtoV1, ContractEvidenceAdmissionPolicyDtoV1,
    ContractEvidenceAppendReceiptDtoV1, ContractEvidenceAppendRequestV1,
    ContractEvidenceAppendResultV1, ContractEvidenceCandidateDtoV1,
    ContractEvidenceFreshnessPolicyDtoV1, ContractEvidenceListEntryV1,
    ContractEvidenceListRequestV1, ContractEvidenceListResultV1,
    ContractEvidenceOperationDetailsDtoV1, ContractEvidenceOperationSchemaValidator,
    ContractEvidenceProblemCauseV1, ContractEvidenceReadRequestV1, ContractEvidenceReadResultV1,
    ContractEvidenceRecordDtoV1, ContractEvidenceRetentionV1, ContractEvidenceStabilityPolicyDtoV1,
    ContractGateEvaluateRequestV1, ContractGateEvaluateResultV1, ContractGatePolicyDtoV1,
    ContractGateResultDtoV1, ContractLocalCloseoutPolicyDtoV1, ContractLocalCloseoutResultDtoV1,
    ContractRecordBindingDtoV1, ContractVerdictEvaluateRequestV1, ContractVerdictEvaluateResultV1,
    CONTRACT_EVIDENCE_APPEND_REF, CONTRACT_EVIDENCE_LIST_REF, CONTRACT_EVIDENCE_OPERATION_REFS,
    CONTRACT_EVIDENCE_READ_REF, CONTRACT_GATE_EVALUATE_REF, CONTRACT_VERDICT_EVALUATE_REF,
};
pub(crate) use contract_operation::execute_contract_operation;
pub use contract_operation::{
    contract_definition_append_operation_request, contract_definition_list_operation_request,
    contract_definition_read_operation_request, contract_lifecycle_transition_operation_request,
    contract_operation_definition, contract_operation_definitions,
    execute_contract_operation_with_authenticator, ContractApplicabilityOutcomeDtoV1,
    ContractApprovalClassDtoV1, ContractClaimDtoV1, ContractDefinitionAppendRequestV1,
    ContractDefinitionAppendResultV1, ContractDefinitionDtoV1, ContractDefinitionListEntryV1,
    ContractDefinitionListRequestV1, ContractDefinitionListResultV1,
    ContractDefinitionReadRequestV1, ContractDefinitionReadResultV1,
    ContractEvaluationLifecycleBasisDtoV1, ContractLifecycleEvidenceDtoV1,
    ContractLifecycleEvidenceKindDtoV1, ContractLifecycleHeadDtoV1, ContractLifecyclePolicyDtoV1,
    ContractLifecycleReasonDtoV1, ContractLifecycleStateDtoV1, ContractLifecycleTransitionDtoV1,
    ContractLifecycleTransitionInputDtoV1, ContractLifecycleTransitionRequestV1,
    ContractLifecycleTransitionResultV1, ContractListRetentionV1, ContractMutationDispositionV1,
    ContractOperationProblemDetailsV1, ContractOperationSchemaValidator, ContractProblemCauseV1,
    CONTRACT_OPERATION_REFS,
};
pub use dock_operation::*;
pub use dto::{
    CorrelationRef, Diagnostic, DiagnosticSeverity, DraftVersion, NextAction, NextActionKind,
    Omission, OmissionReason, Problem, ProblemBinding, ProblemCategory, ProofEffect,
    RecheckCondition, RetryDirective, SchemaManifestEntry, SchemaMediaType, SourceBinding,
    WriteReceipt,
};
pub use envelope::{
    OperationAdmissionRefusalV1, OperationRequestV1, OperationResponseV1, OperationSchemaValidator,
    RequestId,
};
pub use facade::{
    ApproverAdministrationRequestV1, ApproverMappingRequestV1, AuthorCharterRequestV1,
    AuthorProjectContextRequestV1, BootstrapFacadeOutcomeKindV1, BootstrapFacadeResponseBodyV1,
    BootstrapFacadeResponseV1, CatalogKindV1, CatalogReadOutcomeKindV1, CatalogReadRequestV1,
    CatalogReadResponseBodyV1, CatalogReadResponseV1, CharterAcquisitionModeV1,
    CharterCoverageSubmissionV1, CharterInputEntryV1, CharterInputValueV1, CharterIntakeConsumerV1,
    CharterIntakeEnvelopeV1, CharterIntakeSourceKindV1, InvocationRequestBodyV1,
    InvocationRequestKindV1, InvocationRequestV1, PipelineRouteResolveAndPersistRequestV1,
    ProjectContextInputV1, RepositoryDoctorRequestV1, RepositorySetupRequestV1,
    SessionGroundingBoundaryKindV1, SessionGroundingBoundaryRequestV1, UseCaseRequestBodyV1,
    UseCaseRequestV1,
};
pub(crate) use flow_operation::execute_flow_operation;
pub use flow_operation::{
    flow_problem_detail, flow_resolution_from_response, flow_resolve_operation_definition,
    flow_resolve_operation_request, FlowProblemCauseV1, FlowResolveSchemaValidator,
};
pub(crate) use invocation::measure_repository_setup_invocation;
pub use invocation::{
    ApproverAdministrationDetailsV1, ApproverAdministrationOperationV1,
    ApproverAdministrationResultV1, ArtifactApplicabilityV1, ArtifactInspectionReasonV1,
    ArtifactInspectionStatusV1, AuthorCharterDetailsV1, AuthorCharterOperationV1,
    AuthorCharterResultV1, AuthorInputTransportRouteV1, AuthorProjectContextDetailsV1,
    AuthorProjectContextOperationV1, AuthorProjectContextRefusalKindV1,
    AuthorProjectContextResultV1, AuthorWorkingDirectoryCommandV1,
    DoctorCharterDefinitionClosureStatusV1, DoctorCharterLifecycleStateV1,
    DoctorCharterNextActionV1, InvocationResponseKind, InvocationResponseV1,
    PipelineResolvedStageV1, PipelineRouteBasisWriteConditionV1,
    PipelineRouteResolveAndPersistResultV1, PipelineRouteResolveCauseV1,
    PipelineRouteResolveDetailsV1, PipelineRouteRunV1, PipelineRouteStageReasonV1,
    PipelineRouteStageStatusV1, PipelineRouteStateV1, PipelineRoutingEntryV1,
    ProfileConditionOutcomeV1, ProfileConditionReasonV1, ProfileRequirednessModeV1,
    ReferenceFingerprintV1, RepositoryDoctorArtifactV1, RepositoryDoctorCapabilityV1,
    RepositoryDoctorCharterDefinitionV1, RepositoryDoctorCharterV1, RepositoryDoctorConditionV1,
    RepositoryDoctorDetailsV1, RepositoryDoctorFailureKindV1, RepositoryDoctorFailureReasonV1,
    RepositoryDoctorProjectContextV1, RepositoryDoctorResultV1, RepositoryInvocationCommandV1,
    RepositoryReadinessStatusV1, RepositorySetupArtifactActionKindV1, RepositorySetupArtifactV1,
    RepositorySetupBoundsV1, RepositorySetupDetailsV1, RepositorySetupErrorCodeV1,
    RepositorySetupErrorKindV1, RepositorySetupErrorReasonV1, RepositorySetupModeV1,
    RepositorySetupResultV1, RepositorySetupRootActionV1, RepositorySetupSizeBoundsV1,
    TransportFailureDetailsV1, TransportFailureKind, TransportFailureV1, TransportIoErrorKindV1,
    TransportIoErrorV1, TransportPresentationViewV1, UseCaseCauseV1, UseCaseResponseV1,
};
pub(crate) use invocation_dispatch::{
    dispatch_invocation, dispatch_tauri_invocation, dispatch_tauri_invocation_with_snapshot_source,
    readmit_invocation_response,
};
pub use operation::{
    operation_inventory, validate_complete_definition_set, validate_complete_operation_ref_set,
    AdmissionReason, DeclaredWrite, DefinitionSetError, DiscoveryState, JsonPointer,
    OperationAvailability, OperationClass, OperationDefinition, OperationIdempotency,
    OperationInventoryRow, OperationOutcomeSchemas, OperationOwner, OperationOwnerBinding,
    OperationTransport,
};
pub use owner_admission::{
    admit_operation_definition, OperationAdmissionResult, OwnerAdmissionToken,
};
pub(crate) use pipeline_operation::execute_pipeline_operation;
pub use pipeline_operation::{
    pipeline_capture_apply_from_response, pipeline_capture_apply_operation_request,
    pipeline_capture_direct_operation_request, pipeline_capture_plan_from_response,
    pipeline_capture_plan_operation_request, pipeline_capture_refusal_from_response,
    pipeline_catalog_list_from_response, pipeline_catalog_list_operation_request,
    pipeline_catalog_read_from_response, pipeline_catalog_read_operation_request,
    pipeline_compile_from_response, pipeline_compile_operation_request,
    pipeline_handoff_emit_from_response, pipeline_handoff_emit_operation_request,
    pipeline_handoff_refusal_from_response, pipeline_operation_definitions,
    pipeline_operation_problem_details, pipeline_state_apply_from_response,
    pipeline_state_apply_operation_request, pipeline_state_refusal_from_response,
    PipelineOperationProblemProjection, PipelineOperationSchemaValidator,
    PipelineStateProblemProjection,
};
pub(crate) use posture::apply_posture_transition_operation;
pub use posture::{
    admit_posture_transition_apply_request, posture_transition_apply_definition,
    posture_transition_apply_request, PostureCanonicalBindingDtoV1,
    PostureCanonicalWriteConditionV1, PostureDimensionChangeDtoV1, PostureOwnerVariantDetailsV1,
    PostureOwnerVariantV1, PostureReferenceDtoV1, PostureSemanticWriteConditionV1,
    PostureTransitionApplyRequestDataV1, PostureTransitionApplyResultDataV1,
};
pub use production_witness::{
    BootstrapOutcome, ProductionCatalogRead, ProductionWitness, ProductionWitnessError,
};
pub use raw_key::{
    with_owner_idempotency_key, DefinitionPinnedKeyPointer, IdempotencyKeyConsumer,
    RawIdempotencyKey,
};
pub use response::{
    IdempotencyStage, IdempotencyState, IdempotencyStateRef, TerminalFields, TerminalStatus,
};
pub use serialization::{
    canonical_fingerprint, canonical_json_bytes, parse_canonical_json, sha256_fingerprint,
};
pub(crate) use session_grounding::{
    execute_session_grounding_boundary, execute_session_grounding_boundary_dyn,
    recompose_session_grounding_boundary,
};
pub use session_grounding::{
    SessionGroundingBoundaryDetailsV1, SessionGroundingBoundaryResultV1,
    SessionGroundingBoundaryUseCaseOutcomeV1, SessionGroundingCompositionFailureV1,
    SessionGroundingOwnerFailureV1, SessionGroundingPredecessorV1, SessionGroundingRecordV1,
    SessionGroundingRecoveryActionV1, SessionGroundingTerminalV1,
};
pub use typed_catalog::{
    CatalogPageSize, CatalogReadError, ImmutableCatalog, TypedCatalogCursor, TypedCatalogEntry,
    TypedCatalogPage,
};
pub use verification::{verify, ContractWitness, VerificationError, Verified, VerifyContract};

#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) use bootstrap::CurrentTransportEvidenceSealed;
#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) use raw_key::IdempotencyKeyConsumerSealed;
#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) use verification::ContractWitnessSealed;

use std::fmt;

/// Closed error classification for structurally admitted contract values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContractError {
    Empty,
    LimitOutOfRange,
    BoundExceeded,
    InvalidString,
    InvalidNumber,
    InvalidTimestamp,
    InvalidSemVer,
    InvalidExactRef,
    InvalidFingerprint,
    InvalidJson,
    NonCanonicalJson,
    DuplicateMember,
    NonCanonicalOrder,
    InvalidInvariant,
    InvalidArtifactLocator,
    FingerprintMismatch,
}

impl fmt::Display for ContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Empty => "required value is empty",
            Self::LimitOutOfRange => "configured limit is out of range",
            Self::BoundExceeded => "contract bound was exceeded",
            Self::InvalidString => "contract string is invalid",
            Self::InvalidNumber => "contract number is invalid",
            Self::InvalidTimestamp => "UTC timestamp is invalid",
            Self::InvalidSemVer => "semantic version is invalid",
            Self::InvalidExactRef => "exact reference is invalid",
            Self::InvalidFingerprint => "fingerprint is invalid",
            Self::InvalidJson => "JSON document is invalid",
            Self::NonCanonicalJson => "JSON document is not canonical",
            Self::DuplicateMember => "contract member is duplicated",
            Self::NonCanonicalOrder => "contract members are not canonically ordered",
            Self::InvalidInvariant => "contract invariant is invalid",
            Self::InvalidArtifactLocator => "artifact locator is invalid",
            Self::FingerprintMismatch => "fingerprint does not match",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for ContractError {}
