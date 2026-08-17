mod artifact;
mod binding;
mod bootstrap;
mod bootstrap_envelope;
mod bounds;
mod catalog;
mod dto;
mod envelope;
mod invocation;
mod operation;
mod owner_admission;
mod posture;
mod production_witness;
mod raw_key;
mod response;
#[cfg(feature = "contract-schema-generation")]
#[doc(hidden)]
pub mod schema_generation;
mod serialization;
mod typed_catalog;
mod verification;

pub use artifact::{
    ArtifactLocator, ArtifactLocatorRef, ArtifactRef, DigestAlgorithm, Sensitivity,
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
pub(crate) use invocation::measure_repository_setup_invocation;
pub use invocation::{
    ApproverAdministrationOperationV1, ApproverAdministrationResultV1, ArtifactApplicabilityV1,
    ArtifactInspectionReasonV1, ArtifactInspectionStatusV1, AuthorCharterOperationV1,
    AuthorCharterResultV1, AuthorProjectContextResultV1, DoctorCharterDefinitionClosureStatusV1,
    DoctorCharterLifecycleStateV1, DoctorCharterNextActionV1, InvocationResponseKind,
    InvocationResponseV1, PipelineResolvedStageV1, PipelineRouteResolveAndPersistResultV1,
    PipelineRouteRunV1, PipelineRouteStageReasonV1, PipelineRouteStageStatusV1,
    PipelineRouteStateV1, PipelineRoutingEntryV1, ProfileConditionOutcomeV1,
    ProfileConditionReasonV1, ProfileRequirednessModeV1, ReferenceFingerprintV1,
    RepositoryDoctorArtifactV1, RepositoryDoctorCapabilityV1, RepositoryDoctorCharterDefinitionV1,
    RepositoryDoctorCharterV1, RepositoryDoctorConditionV1, RepositoryDoctorDetailsV1,
    RepositoryDoctorFailureKindV1, RepositoryDoctorFailureReasonV1,
    RepositoryDoctorProjectContextV1, RepositoryDoctorResultV1, RepositoryReadinessStatusV1,
    RepositorySetupArtifactActionKindV1, RepositorySetupArtifactV1, RepositorySetupBoundsV1,
    RepositorySetupDetailsV1, RepositorySetupErrorCodeV1, RepositorySetupErrorKindV1,
    RepositorySetupErrorReasonV1, RepositorySetupModeV1, RepositorySetupResultV1,
    RepositorySetupRootActionV1, RepositorySetupSizeBoundsV1, TransportFailureDetailsV1,
    TransportFailureKind, TransportFailureV1, UseCaseResponseV1,
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
