//! Owner-native contract definition and claim semantics.
//!
//! This crate owns exact contract definitions inward of consumer SDKs and
//! transports. Compatibility classifications are descriptive only: callers
//! must always select definitions by an exact reference and fingerprint.

mod canonical;
mod claim;
mod claim_verdict;
mod compatibility;
mod definition;
mod evaluation_identity;
mod evaluation_policy;
mod evaluation_run;
mod evidence_admission;
mod evidence_candidate_source;
pub mod evidence_operation_admission;
mod gate_composition;
mod identity;
mod lifecycle;
mod lifecycle_policy;
mod lifecycle_preimage;
pub mod operation_admission;
mod operation_claims;
mod owner_repository;
pub mod project_admission_governance;

pub use claim::{
    evaluate_applicability, ApplicabilityInputSourceV1, ApplicabilityOutcomeV1,
    ApplicabilityRuleV1, ContractClaimInputV1, ContractClaimV1, EvidenceCardinalityV1,
    EvidenceCaseSelectionV1, EvidenceRequirementV1, GateEffectV1, MatcherBindingV1,
    MatcherInputCandidateV1, MatcherValueV1, ResolutionMinimumV1, ScoreWeightV1,
    SelectorCriterionV1, SelectorV1, StableIdV1,
};
pub use claim_verdict::{
    evaluate_claim_verdicts_v1, ClaimVerdictEvaluationErrorKindV1, ClaimVerdictEvaluationErrorV1,
    ClaimVerdictEvaluationV1,
};
pub use compatibility::{DefinitionChangeV1, MinimumVersionChangeV1};
pub use definition::{
    classify_definition_mutation, ContractDefinitionInputV1, ContractDefinitionV1, ContractKindV1,
    DefinitionMutationClassificationV1, CONTRACT_OWNER_SOURCE_REF_V1,
};
pub use evaluation_identity::*;
pub use evaluation_policy::*;
pub use evaluation_run::{
    prepare_evaluation_run_v1, ContractEvaluationRunV1, EvaluationRunNonceV1,
    PrepareEvaluationRunInputV1, CONTRACT_EVALUATION_RUN_SCHEMA_REF_V1,
};
pub use evidence_admission::{
    admit_evidence_candidate_v1, EvidenceCandidateAdmissionErrorKindV1,
    EvidenceCandidateAdmissionErrorV1,
};
pub use evidence_candidate_source::{
    resolve_repository_candidate_observation_v1, EvidenceCandidateSourceErrorKindV1,
    EvidenceCandidateSourceErrorV1, EvidenceCandidateSourceV1, RepositoryEvidenceCandidateSourceV1,
    ResolvedEvidenceCandidateObservationInputV1, ResolvedEvidenceCandidateObservationV1,
};
#[cfg(feature = "test-support")]
#[doc(hidden)]
pub use evidence_candidate_source::{
    seed_committed_candidate_observation_for_testing_v1, CandidateObservationFixtureV1,
};
pub use gate_composition::{
    evaluate_gate_and_local_closeout_v1, GateEvaluationErrorKindV1, GateEvaluationErrorV1,
    GateEvaluationV1,
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
    DefinitionRecordV1, EvidenceAppendExecutionV1, EvidenceAppendReceiptV1, EvidenceListEntryV1,
    EvidenceListPageV1, EvidenceListRequestV1, EvidenceListRetentionV1, EvidenceReadOutcomeV1,
    LifecycleTransitionExecutionV1, OwnerMutationDispositionV1, PreparedOwnerLifecycleTransitionV1,
    StoredEvidenceRecordV1, CONTRACT_OWNER_REPOSITORY_RELATIVE_ROOT_V1,
};
