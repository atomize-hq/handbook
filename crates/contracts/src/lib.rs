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
