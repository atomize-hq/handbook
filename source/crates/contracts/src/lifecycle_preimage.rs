use crate::lifecycle::{
    ContractAuthorityAdmissionRecordV1, LifecycleReasonV1, LifecycleStateV1,
    LifecycleSupportingEvidenceV1,
};
use crate::{ContractApprovalClassV1, ExactBindingV1, ExactRefV1, FingerprintV1, StableIdV1};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct DefinitionAppendScopePreimage<'a> {
    pub(crate) schema_ref: &'static str,
    pub(crate) contract_ref: &'a ExactRefV1,
    pub(crate) definition_fingerprint: &'a FingerprintV1,
    pub(crate) definition_author_ref: &'a StableIdV1,
    pub(crate) definition_authority: &'a ExactBindingV1,
    pub(crate) lifecycle_policy: &'a ExactBindingV1,
    pub(crate) draft_genesis_fingerprint: &'a FingerprintV1,
}

#[derive(Serialize)]
pub(crate) struct GenesisFingerprintPreimage<'a> {
    pub(crate) schema_ref: &'static str,
    pub(crate) contract_ref: &'a ExactRefV1,
    pub(crate) definition_fingerprint: &'a FingerprintV1,
    pub(crate) lifecycle_policy_ref: &'a ExactRefV1,
    pub(crate) lifecycle_policy_fingerprint: &'a FingerprintV1,
    pub(crate) state: LifecycleStateV1,
    pub(crate) prior_transition_ref: Option<&'a str>,
}

#[derive(Serialize)]
pub(crate) struct TransitionScopePreimage<'a> {
    pub(crate) schema_ref: &'static str,
    pub(crate) contract_ref: &'a ExactRefV1,
    pub(crate) definition_fingerprint: &'a FingerprintV1,
    pub(crate) prior_transition_ref: Option<&'a str>,
    pub(crate) prior_lifecycle_fingerprint: &'a FingerprintV1,
    pub(crate) from_state: LifecycleStateV1,
    pub(crate) to_state: LifecycleStateV1,
    pub(crate) approval_class: ContractApprovalClassV1,
    pub(crate) transition_policy: &'a ExactBindingV1,
    pub(crate) supporting_evidence: &'a [LifecycleSupportingEvidenceV1],
    pub(crate) reason: &'a LifecycleReasonV1,
}

#[derive(Serialize)]
pub(crate) struct TransitionFingerprintPreimage<'a> {
    pub(crate) schema_ref: &'static str,
    pub(crate) contract_ref: &'a ExactRefV1,
    pub(crate) definition_fingerprint: &'a FingerprintV1,
    pub(crate) prior_transition_ref: Option<&'a str>,
    pub(crate) prior_lifecycle_fingerprint: &'a FingerprintV1,
    pub(crate) from_state: LifecycleStateV1,
    pub(crate) to_state: LifecycleStateV1,
    pub(crate) actor_ref: &'a StableIdV1,
    pub(crate) authority_ref: &'a ExactRefV1,
    pub(crate) authority_admission: &'a ContractAuthorityAdmissionRecordV1,
    pub(crate) transition_policy_ref: &'a ExactRefV1,
    pub(crate) transition_policy_fingerprint: &'a FingerprintV1,
    pub(crate) supporting_evidence: &'a [LifecycleSupportingEvidenceV1],
    pub(crate) reason: &'a LifecycleReasonV1,
    pub(crate) decision_time: &'a str,
}

#[derive(Serialize)]
pub(crate) struct ResultFingerprintPreimage<'a> {
    pub(crate) schema_ref: &'static str,
    pub(crate) contract_ref: &'a ExactRefV1,
    pub(crate) definition_fingerprint: &'a FingerprintV1,
    pub(crate) state: LifecycleStateV1,
    pub(crate) prior_transition_ref: Option<&'a str>,
    pub(crate) prior_lifecycle_fingerprint: &'a FingerprintV1,
    pub(crate) transition_ref: &'a str,
    pub(crate) transition_fingerprint: &'a FingerprintV1,
}

#[derive(Serialize)]
pub(crate) struct BasisFingerprintPreimage<'a> {
    pub(crate) schema_ref: &'static str,
    pub(crate) contract_ref: &'a ExactRefV1,
    pub(crate) definition_fingerprint: &'a FingerprintV1,
    pub(crate) lifecycle_policy_ref: &'a ExactRefV1,
    pub(crate) lifecycle_policy_fingerprint: &'a FingerprintV1,
    pub(crate) independent_lock_transition_ref: &'a str,
    pub(crate) independent_lock_transition_fingerprint: &'a FingerprintV1,
    pub(crate) independent_lock_resulting_lifecycle_fingerprint: &'a FingerprintV1,
    pub(crate) active_transition_ref: &'a str,
    pub(crate) active_transition_fingerprint: &'a FingerprintV1,
    pub(crate) active_resulting_lifecycle_fingerprint: &'a FingerprintV1,
}
