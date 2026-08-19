//! Owner-native contract definition and claim semantics.
//!
//! This crate owns exact contract definitions inward of consumer SDKs and
//! transports. Compatibility classifications are descriptive only: callers
//! must always select definitions by an exact reference and fingerprint.

mod canonical;
mod claim;
mod compatibility;
mod definition;
mod identity;
mod lifecycle;
mod lifecycle_policy;
mod lifecycle_preimage;
mod owner_repository;

pub use claim::{
    evaluate_applicability, ApplicabilityInputSourceV1, ApplicabilityOutcomeV1,
    ApplicabilityRuleV1, ContractClaimInputV1, ContractClaimV1, EvidenceCardinalityV1,
    EvidenceCaseSelectionV1, EvidenceRequirementV1, GateEffectV1, MatcherBindingV1,
    MatcherInputCandidateV1, MatcherValueV1, ResolutionMinimumV1, ScoreWeightV1,
    SelectorCriterionV1, SelectorV1, StableIdV1,
};
pub use compatibility::{DefinitionChangeV1, MinimumVersionChangeV1};
pub use definition::{
    classify_definition_mutation, ContractDefinitionInputV1, ContractDefinitionV1, ContractKindV1,
    DefinitionMutationClassificationV1, CONTRACT_OWNER_SOURCE_REF_V1,
};
pub use identity::{
    ContractError, ContractIdV1, ExactBindingV1, ExactRefV1, FingerprintV1, VersionV1,
};
pub use lifecycle::{
    current_lifecycle_head, evaluation_lifecycle_admission_basis_v1,
    prepare_contract_definition_append, prepare_lifecycle_transition, AdmittedContractDefinitionV1,
    ContractLifecycleErrorV1, ContractLifecycleTransitionInputV1, ContractLifecycleTransitionV1,
    DraftGenesisV1, EvaluationLifecycleAdmissionBasisV1, LifecycleEvidenceKindV1, LifecycleHeadV1,
    LifecycleReasonTextV1, LifecycleReasonV1, LifecycleStateV1, LifecycleSupportingEvidenceV1,
    PreparedContractDefinitionAppendV1, PreparedContractLifecycleTransitionV1,
};
pub use lifecycle_policy::{
    lifecycle_policy_binding_v1, lifecycle_policy_canonical_json_v1, ContractApprovalClassV1,
    LIFECYCLE_POLICY_REF_V1,
};
#[cfg(feature = "test-support")]
#[doc(hidden)]
pub use owner_repository::OwnerRepositoryFaultBoundaryV1;
pub use owner_repository::{
    ContractOwnerRepositoryErrorKindV1, ContractOwnerRepositoryErrorV1, ContractOwnerRepositoryV1,
    DefinitionAppendExecutionV1, DefinitionListEntryV1, DefinitionListPageV1,
    DefinitionListRequestV1, DefinitionListRetentionV1, DefinitionReadOutcomeV1,
    DefinitionRecordV1, LifecycleTransitionExecutionV1, OwnerMutationDispositionV1,
    PreparedOwnerLifecycleTransitionV1, CONTRACT_OWNER_REPOSITORY_RELATIVE_ROOT_V1,
};
