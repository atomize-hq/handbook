use crate::canonical::{canonical_bytes, canonical_fingerprint};
use crate::lifecycle_preimage::{
    BasisFingerprintPreimage, DefinitionAppendScopePreimage, GenesisFingerprintPreimage,
    ResultFingerprintPreimage, TransitionFingerprintPreimage, TransitionScopePreimage,
};
use crate::{
    lifecycle_policy_binding_v1, ContractApprovalClassV1, ContractDefinitionV1, ContractError,
    ExactBindingV1, ExactRefV1, FingerprintV1, StableIdV1,
};
use handbook_engine::{CredentialAuthorityAdmissionRequestV1, CredentialAuthorityAdmissionV1};
use serde::{de::Error as _, Deserialize, Deserializer, Serialize};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

const DEFINITION_APPEND_ACTION_REF: &str = "handbook.contract-definition-append@1.0.0";
const TRANSITION_ACTION_REF: &str = "handbook.contract-lifecycle-transition@1.0.0";
const GENESIS_SCHEMA_REF: &str = "handbook.contract-lifecycle-genesis@1.0.0";
const RESULT_SCHEMA_REF: &str = "handbook.contract-lifecycle-result@1.0.0";
const BASIS_SCHEMA_REF: &str = "handbook.contract-evaluation-lifecycle-basis@1.0.0";
const MAX_REASON_BYTES: usize = 2_048;
const MAX_SUPPORTING_EVIDENCE: usize = 16;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContractLifecycleErrorV1 {
    InvalidPolicy,
    InvalidAdmission,
    InvalidEdge,
    InvalidRole,
    InvalidReason,
    InvalidEvidence,
    StaleHead,
    InvalidChain,
    SelfLock,
    BoundExceeded,
    Canonicalization,
}

impl fmt::Display for ContractLifecycleErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidPolicy => "contract lifecycle policy is invalid",
            Self::InvalidAdmission => "credential authority admission is invalid",
            Self::InvalidEdge => "contract lifecycle edge is not allowed",
            Self::InvalidRole => "credential authority role is not allowed for the edge",
            Self::InvalidReason => "lifecycle reason is not allowed for the edge",
            Self::InvalidEvidence => "supporting authority evidence is invalid for the edge",
            Self::StaleHead => "expected lifecycle head is stale",
            Self::InvalidChain => "ordered lifecycle chain is invalid",
            Self::SelfLock => "definition author cannot independently lock the definition",
            Self::BoundExceeded => "contract lifecycle bound exceeded",
            Self::Canonicalization => "contract lifecycle canonicalization failed",
        })
    }
}

impl Error for ContractLifecycleErrorV1 {}

impl From<ContractError> for ContractLifecycleErrorV1 {
    fn from(error: ContractError) -> Self {
        match error {
            ContractError::BoundExceeded => Self::BoundExceeded,
            ContractError::Canonicalization => Self::Canonicalization,
            _ => Self::InvalidAdmission,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleStateV1 {
    Draft,
    ReviewReady,
    Locked,
    Active,
    Deprecated,
    Closed,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LifecycleReasonTextV1(String);

impl LifecycleReasonTextV1 {
    pub fn parse(value: &str) -> Result<Self, ContractLifecycleErrorV1> {
        if value.is_empty() || value.len() > MAX_REASON_BYTES || value.contains('\0') {
            return Err(ContractLifecycleErrorV1::InvalidReason);
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for LifecycleReasonTextV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", content = "detail", rename_all = "snake_case")]
pub enum LifecycleReasonV1 {
    ReadyForReview,
    Withdrawn,
    IndependentlyLocked,
    Rejected,
    Activated,
    DeprecatedWithReplacement(ExactBindingV1),
    DeprecatedWithoutReplacement(LifecycleReasonTextV1),
    ClosedAfterRetention,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleEvidenceKindV1 {
    DefinitionValidation,
    IndependentReview,
    Rejection,
    ApplicabilityClosure,
    MatcherClosure,
    PolicyClosure,
    CurrentnessClosure,
    Replacement,
    NoReplacementRationale,
    RetentionSafety,
    ReferenceSafety,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LifecycleSupportingEvidenceV1 {
    kind: LifecycleEvidenceKindV1,
    binding: ExactBindingV1,
}

impl LifecycleSupportingEvidenceV1 {
    pub fn new(kind: LifecycleEvidenceKindV1, binding: ExactBindingV1) -> Self {
        Self { kind, binding }
    }

    pub fn kind(&self) -> LifecycleEvidenceKindV1 {
        self.kind
    }

    pub fn binding(&self) -> &ExactBindingV1 {
        &self.binding
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DraftGenesisV1 {
    schema_ref: &'static str,
    contract_ref: ExactRefV1,
    definition_fingerprint: FingerprintV1,
    lifecycle_policy_ref: ExactRefV1,
    lifecycle_policy_fingerprint: FingerprintV1,
    state: LifecycleStateV1,
    prior_transition_ref: Option<String>,
    lifecycle_fingerprint: FingerprintV1,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DraftGenesisWireV1 {
    schema_ref: String,
    contract_ref: ExactRefV1,
    definition_fingerprint: FingerprintV1,
    lifecycle_policy_ref: ExactRefV1,
    lifecycle_policy_fingerprint: FingerprintV1,
    state: LifecycleStateV1,
    prior_transition_ref: Option<String>,
    lifecycle_fingerprint: FingerprintV1,
}

impl<'de> Deserialize<'de> for DraftGenesisV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let wire = DraftGenesisWireV1::deserialize(deserializer)?;
        if wire.schema_ref != GENESIS_SCHEMA_REF {
            return Err(D::Error::custom("draft genesis schema ref is not exact"));
        }
        Ok(Self {
            schema_ref: GENESIS_SCHEMA_REF,
            contract_ref: wire.contract_ref,
            definition_fingerprint: wire.definition_fingerprint,
            lifecycle_policy_ref: wire.lifecycle_policy_ref,
            lifecycle_policy_fingerprint: wire.lifecycle_policy_fingerprint,
            state: wire.state,
            prior_transition_ref: wire.prior_transition_ref,
            lifecycle_fingerprint: wire.lifecycle_fingerprint,
        })
    }
}

impl DraftGenesisV1 {
    pub fn lifecycle_fingerprint(&self) -> &FingerprintV1 {
        &self.lifecycle_fingerprint
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ContractAuthorityAdmissionRecordV1 {
    repository_identity_fingerprint: FingerprintV1,
    registry_state_ref: String,
    registry_state_fingerprint: FingerprintV1,
    registry_head_transition_ref: String,
    registry_head_transition_fingerprint: FingerprintV1,
    credential_registration_ref: String,
    credential_registration_fingerprint: FingerprintV1,
    authenticator_assertion_fingerprint: FingerprintV1,
    semantic_scope_fingerprint: FingerprintV1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AdmittedContractDefinitionV1 {
    definition: ContractDefinitionV1,
    genesis: DraftGenesisV1,
    author_admission: ContractAuthorityAdmissionRecordV1,
}

impl AdmittedContractDefinitionV1 {
    pub fn definition(&self) -> &ContractDefinitionV1 {
        &self.definition
    }

    pub fn genesis(&self) -> &DraftGenesisV1 {
        &self.genesis
    }

    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, ContractLifecycleErrorV1> {
        canonical_bytes(self).map_err(Into::into)
    }

    pub(crate) fn repository_identity_fingerprint(&self) -> &FingerprintV1 {
        &self.author_admission.repository_identity_fingerprint
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AdmittedContractDefinitionWireV1 {
    definition: ContractDefinitionV1,
    genesis: DraftGenesisV1,
    author_admission: ContractAuthorityAdmissionRecordV1,
}

impl<'de> Deserialize<'de> for AdmittedContractDefinitionV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let wire = AdmittedContractDefinitionWireV1::deserialize(deserializer)?;
        let expected_genesis = derive_genesis(&wire.definition).map_err(D::Error::custom)?;
        let expected_scope = canonical_fingerprint(&DefinitionAppendScopePreimage {
            schema_ref: "handbook.contract-definition-append-scope@1.0.0",
            contract_ref: wire.definition.contract_ref(),
            definition_fingerprint: wire.definition.definition_fingerprint(),
            definition_author_ref: wire.definition.definition_author_ref(),
            definition_authority: wire.definition.definition_authority(),
            lifecycle_policy: wire.definition.lifecycle_policy(),
            draft_genesis_fingerprint: expected_genesis.lifecycle_fingerprint(),
        })
        .map_err(D::Error::custom)?;
        if wire.genesis != expected_genesis
            || wire.author_admission.registry_state_fingerprint
                != *wire.definition.definition_authority().fingerprint()
            || wire.author_admission.semantic_scope_fingerprint != expected_scope
            || !valid_authority_record_ref(&wire.author_admission.registry_state_ref)
            || !valid_authority_record_ref(&wire.author_admission.registry_head_transition_ref)
            || !valid_authority_record_ref(&wire.author_admission.credential_registration_ref)
        {
            return Err(D::Error::custom(
                "persisted definition authority closure is invalid",
            ));
        }
        Ok(Self {
            definition: wire.definition,
            genesis: wire.genesis,
            author_admission: wire.author_admission,
        })
    }
}

pub struct PreparedContractDefinitionAppendV1 {
    definition: ContractDefinitionV1,
    genesis: DraftGenesisV1,
    semantic_scope_fingerprint: FingerprintV1,
    authority_request: CredentialAuthorityAdmissionRequestV1,
}

impl PreparedContractDefinitionAppendV1 {
    pub fn authority_request(&self) -> CredentialAuthorityAdmissionRequestV1 {
        self.authority_request.clone()
    }

    pub fn finalize(
        self,
        admission: CredentialAuthorityAdmissionV1<'_>,
    ) -> Result<AdmittedContractDefinitionV1, ContractLifecycleErrorV1> {
        validate_admission_scope(
            &admission,
            DEFINITION_APPEND_ACTION_REF,
            self.definition.contract_ref().as_str(),
            ContractApprovalClassV1::DefinitionAuthor,
            &self.semantic_scope_fingerprint,
        )?;
        if admission.actor_ref() != self.definition.definition_author_ref().as_str()
            || admission.authority_ref()
                != self.definition.definition_authority().reference().as_str()
            || admission.registry_state_fingerprint()
                != self
                    .definition
                    .definition_authority()
                    .fingerprint()
                    .as_str()
        {
            return Err(ContractLifecycleErrorV1::InvalidAdmission);
        }
        let author_admission = admission_record(&admission)?;
        Ok(AdmittedContractDefinitionV1 {
            definition: self.definition,
            genesis: self.genesis,
            author_admission,
        })
    }
}

pub fn prepare_contract_definition_append(
    definition: ContractDefinitionV1,
) -> Result<PreparedContractDefinitionAppendV1, ContractLifecycleErrorV1> {
    require_exact_policy(&definition)?;
    let genesis = derive_genesis(&definition)?;
    let semantic_scope_fingerprint = canonical_fingerprint(&DefinitionAppendScopePreimage {
        schema_ref: "handbook.contract-definition-append-scope@1.0.0",
        contract_ref: definition.contract_ref(),
        definition_fingerprint: definition.definition_fingerprint(),
        definition_author_ref: definition.definition_author_ref(),
        definition_authority: definition.definition_authority(),
        lifecycle_policy: definition.lifecycle_policy(),
        draft_genesis_fingerprint: genesis.lifecycle_fingerprint(),
    })?;
    let authority_request = CredentialAuthorityAdmissionRequestV1::new(
        DEFINITION_APPEND_ACTION_REF,
        definition.contract_ref().as_str(),
        ContractApprovalClassV1::DefinitionAuthor.as_str(),
        semantic_scope_fingerprint.as_str(),
    )
    .map_err(|_| ContractLifecycleErrorV1::InvalidAdmission)?;
    Ok(PreparedContractDefinitionAppendV1 {
        definition,
        genesis,
        semantic_scope_fingerprint,
        authority_request,
    })
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContractLifecycleTransitionInputV1 {
    pub expected_lifecycle_fingerprint: FingerprintV1,
    pub from_state: LifecycleStateV1,
    pub to_state: LifecycleStateV1,
    pub approval_class: ContractApprovalClassV1,
    pub supporting_evidence: Vec<LifecycleSupportingEvidenceV1>,
    pub reason: LifecycleReasonV1,
}

pub struct PreparedContractLifecycleTransitionV1 {
    definition: AdmittedContractDefinitionV1,
    prior_transition_ref: Option<String>,
    input: ContractLifecycleTransitionInputV1,
    semantic_scope_fingerprint: FingerprintV1,
    authority_request: CredentialAuthorityAdmissionRequestV1,
}

impl PreparedContractLifecycleTransitionV1 {
    pub fn authority_request(&self) -> CredentialAuthorityAdmissionRequestV1 {
        self.authority_request.clone()
    }

    pub fn finalize(
        self,
        admission: CredentialAuthorityAdmissionV1<'_>,
    ) -> Result<ContractLifecycleTransitionV1, ContractLifecycleErrorV1> {
        validate_admission_scope(
            &admission,
            TRANSITION_ACTION_REF,
            self.definition.definition.contract_ref().as_str(),
            self.input.approval_class,
            &self.semantic_scope_fingerprint,
        )?;
        validate_actor_rule(
            &self.definition,
            self.input.from_state,
            self.input.to_state,
            self.input.approval_class,
            admission.actor_ref(),
            admission.authority_ref(),
        )?;
        if admission.repository_identity_fingerprint()
            != self.definition.repository_identity_fingerprint().as_str()
        {
            return Err(ContractLifecycleErrorV1::InvalidAdmission);
        }
        let actor_ref = StableIdV1::parse(admission.actor_ref())?;
        let authority_ref = ExactRefV1::parse(admission.authority_ref())?;
        let authority_admission = admission_record(&admission)?;
        let decision_time = admission.semantic_decision_time_utc().to_owned();
        if !valid_decision_time(&decision_time) {
            return Err(ContractLifecycleErrorV1::InvalidAdmission);
        }
        let policy = lifecycle_policy_binding_v1()?;
        let preimage = TransitionFingerprintPreimage {
            schema_ref: TRANSITION_ACTION_REF,
            contract_ref: self.definition.definition.contract_ref(),
            definition_fingerprint: self.definition.definition.definition_fingerprint(),
            prior_transition_ref: self.prior_transition_ref.as_deref(),
            prior_lifecycle_fingerprint: &self.input.expected_lifecycle_fingerprint,
            from_state: self.input.from_state,
            to_state: self.input.to_state,
            actor_ref: &actor_ref,
            authority_ref: &authority_ref,
            authority_admission: &authority_admission,
            transition_policy_ref: policy.reference(),
            transition_policy_fingerprint: policy.fingerprint(),
            supporting_evidence: &self.input.supporting_evidence,
            reason: &self.input.reason,
            decision_time: &decision_time,
        };
        let transition_fingerprint = canonical_fingerprint(&preimage)?;
        let transition_ref = transition_ref(
            self.definition.definition.contract_ref(),
            &transition_fingerprint,
        );
        let resulting_lifecycle_fingerprint = canonical_fingerprint(&ResultFingerprintPreimage {
            schema_ref: RESULT_SCHEMA_REF,
            contract_ref: self.definition.definition.contract_ref(),
            definition_fingerprint: self.definition.definition.definition_fingerprint(),
            state: self.input.to_state,
            prior_transition_ref: self.prior_transition_ref.as_deref(),
            prior_lifecycle_fingerprint: &self.input.expected_lifecycle_fingerprint,
            transition_ref: &transition_ref,
            transition_fingerprint: &transition_fingerprint,
        })?;
        Ok(ContractLifecycleTransitionV1 {
            schema_ref: TRANSITION_ACTION_REF,
            transition_ref,
            transition_fingerprint,
            contract_ref: self.definition.definition.contract_ref().clone(),
            definition_fingerprint: self.definition.definition.definition_fingerprint().clone(),
            prior_transition_ref: self.prior_transition_ref,
            prior_lifecycle_fingerprint: self.input.expected_lifecycle_fingerprint,
            from_state: self.input.from_state,
            to_state: self.input.to_state,
            actor_ref,
            authority_ref,
            approval_class: self.input.approval_class,
            authority_admission,
            transition_policy_ref: policy.reference().clone(),
            transition_policy_fingerprint: policy.fingerprint().clone(),
            supporting_evidence: self.input.supporting_evidence,
            reason: self.input.reason,
            decision_time,
            resulting_lifecycle_fingerprint,
        })
    }
}

pub fn prepare_lifecycle_transition(
    definition: &AdmittedContractDefinitionV1,
    chain: &[ContractLifecycleTransitionV1],
    input: ContractLifecycleTransitionInputV1,
) -> Result<PreparedContractLifecycleTransitionV1, ContractLifecycleErrorV1> {
    let head = current_lifecycle_head(definition, chain)?;
    if input.expected_lifecycle_fingerprint != head.lifecycle_fingerprint
        || input.from_state != head.state
    {
        return Err(ContractLifecycleErrorV1::StaleHead);
    }
    validate_edge_inputs(
        input.from_state,
        input.to_state,
        input.approval_class,
        &input.reason,
        &input.supporting_evidence,
    )?;
    let policy = lifecycle_policy_binding_v1()?;
    let semantic_scope_fingerprint = canonical_fingerprint(&TransitionScopePreimage {
        schema_ref: "handbook.contract-lifecycle-transition-scope@1.0.0",
        contract_ref: definition.definition.contract_ref(),
        definition_fingerprint: definition.definition.definition_fingerprint(),
        prior_transition_ref: head.transition_ref.as_deref(),
        prior_lifecycle_fingerprint: &head.lifecycle_fingerprint,
        from_state: input.from_state,
        to_state: input.to_state,
        approval_class: input.approval_class,
        transition_policy: &policy,
        supporting_evidence: &input.supporting_evidence,
        reason: &input.reason,
    })?;
    let authority_request = CredentialAuthorityAdmissionRequestV1::new(
        TRANSITION_ACTION_REF,
        definition.definition.contract_ref().as_str(),
        input.approval_class.as_str(),
        semantic_scope_fingerprint.as_str(),
    )
    .map_err(|_| ContractLifecycleErrorV1::InvalidAdmission)?;
    Ok(PreparedContractLifecycleTransitionV1 {
        definition: definition.clone(),
        prior_transition_ref: head.transition_ref,
        input,
        semantic_scope_fingerprint,
        authority_request,
    })
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContractLifecycleTransitionV1 {
    schema_ref: &'static str,
    transition_ref: String,
    transition_fingerprint: FingerprintV1,
    contract_ref: ExactRefV1,
    definition_fingerprint: FingerprintV1,
    prior_transition_ref: Option<String>,
    prior_lifecycle_fingerprint: FingerprintV1,
    from_state: LifecycleStateV1,
    to_state: LifecycleStateV1,
    actor_ref: StableIdV1,
    authority_ref: ExactRefV1,
    approval_class: ContractApprovalClassV1,
    authority_admission: ContractAuthorityAdmissionRecordV1,
    transition_policy_ref: ExactRefV1,
    transition_policy_fingerprint: FingerprintV1,
    supporting_evidence: Vec<LifecycleSupportingEvidenceV1>,
    reason: LifecycleReasonV1,
    decision_time: String,
    resulting_lifecycle_fingerprint: FingerprintV1,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ContractLifecycleTransitionWireV1 {
    schema_ref: String,
    transition_ref: String,
    transition_fingerprint: FingerprintV1,
    contract_ref: ExactRefV1,
    definition_fingerprint: FingerprintV1,
    prior_transition_ref: Option<String>,
    prior_lifecycle_fingerprint: FingerprintV1,
    from_state: LifecycleStateV1,
    to_state: LifecycleStateV1,
    actor_ref: StableIdV1,
    authority_ref: ExactRefV1,
    approval_class: ContractApprovalClassV1,
    authority_admission: ContractAuthorityAdmissionRecordV1,
    transition_policy_ref: ExactRefV1,
    transition_policy_fingerprint: FingerprintV1,
    supporting_evidence: Vec<LifecycleSupportingEvidenceV1>,
    reason: LifecycleReasonV1,
    decision_time: String,
    resulting_lifecycle_fingerprint: FingerprintV1,
}

impl<'de> Deserialize<'de> for ContractLifecycleTransitionV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let wire = ContractLifecycleTransitionWireV1::deserialize(deserializer)?;
        if wire.schema_ref != TRANSITION_ACTION_REF || !valid_decision_time(&wire.decision_time) {
            return Err(D::Error::custom(
                "persisted lifecycle transition envelope is invalid",
            ));
        }
        for reference in [
            wire.authority_admission.registry_state_ref.as_str(),
            wire.authority_admission
                .registry_head_transition_ref
                .as_str(),
            wire.authority_admission
                .credential_registration_ref
                .as_str(),
        ] {
            if !valid_authority_record_ref(reference) {
                return Err(D::Error::custom(
                    "persisted lifecycle authority record ref is invalid",
                ));
            }
        }
        Ok(Self {
            schema_ref: TRANSITION_ACTION_REF,
            transition_ref: wire.transition_ref,
            transition_fingerprint: wire.transition_fingerprint,
            contract_ref: wire.contract_ref,
            definition_fingerprint: wire.definition_fingerprint,
            prior_transition_ref: wire.prior_transition_ref,
            prior_lifecycle_fingerprint: wire.prior_lifecycle_fingerprint,
            from_state: wire.from_state,
            to_state: wire.to_state,
            actor_ref: wire.actor_ref,
            authority_ref: wire.authority_ref,
            approval_class: wire.approval_class,
            authority_admission: wire.authority_admission,
            transition_policy_ref: wire.transition_policy_ref,
            transition_policy_fingerprint: wire.transition_policy_fingerprint,
            supporting_evidence: wire.supporting_evidence,
            reason: wire.reason,
            decision_time: wire.decision_time,
            resulting_lifecycle_fingerprint: wire.resulting_lifecycle_fingerprint,
        })
    }
}

impl ContractLifecycleTransitionV1 {
    pub fn contract_ref(&self) -> &ExactRefV1 {
        &self.contract_ref
    }

    pub fn definition_fingerprint(&self) -> &FingerprintV1 {
        &self.definition_fingerprint
    }

    pub fn transition_ref(&self) -> &str {
        &self.transition_ref
    }

    pub fn transition_fingerprint(&self) -> &FingerprintV1 {
        &self.transition_fingerprint
    }

    pub fn prior_transition_ref(&self) -> Option<&str> {
        self.prior_transition_ref.as_deref()
    }

    pub fn prior_lifecycle_fingerprint(&self) -> &FingerprintV1 {
        &self.prior_lifecycle_fingerprint
    }

    pub fn from_state(&self) -> LifecycleStateV1 {
        self.from_state
    }

    pub fn to_state(&self) -> LifecycleStateV1 {
        self.to_state
    }

    pub fn actor_ref(&self) -> &StableIdV1 {
        &self.actor_ref
    }

    pub fn resulting_lifecycle_fingerprint(&self) -> &FingerprintV1 {
        &self.resulting_lifecycle_fingerprint
    }

    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, ContractLifecycleErrorV1> {
        canonical_bytes(self).map_err(Into::into)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecycleHeadV1 {
    state: LifecycleStateV1,
    transition_ref: Option<String>,
    lifecycle_fingerprint: FingerprintV1,
}

impl LifecycleHeadV1 {
    pub fn state(&self) -> LifecycleStateV1 {
        self.state
    }

    pub fn transition_ref(&self) -> Option<&str> {
        self.transition_ref.as_deref()
    }

    pub fn lifecycle_fingerprint(&self) -> &FingerprintV1 {
        &self.lifecycle_fingerprint
    }
}

pub fn current_lifecycle_head(
    definition: &AdmittedContractDefinitionV1,
    chain: &[ContractLifecycleTransitionV1],
) -> Result<LifecycleHeadV1, ContractLifecycleErrorV1> {
    require_exact_policy(&definition.definition)?;
    let expected_genesis = derive_genesis(&definition.definition)?;
    if expected_genesis != definition.genesis {
        return Err(ContractLifecycleErrorV1::InvalidChain);
    }
    let mut head = LifecycleHeadV1 {
        state: LifecycleStateV1::Draft,
        transition_ref: None,
        lifecycle_fingerprint: definition.genesis.lifecycle_fingerprint.clone(),
    };
    for transition in chain {
        validate_chain_transition(definition, &head, transition)?;
        head = LifecycleHeadV1 {
            state: transition.to_state,
            transition_ref: Some(transition.transition_ref.clone()),
            lifecycle_fingerprint: transition.resulting_lifecycle_fingerprint.clone(),
        };
    }
    Ok(head)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EvaluationLifecycleAdmissionBasisV1 {
    schema_ref: &'static str,
    contract_ref: ExactRefV1,
    definition_fingerprint: FingerprintV1,
    lifecycle_policy_ref: ExactRefV1,
    lifecycle_policy_fingerprint: FingerprintV1,
    independent_lock_transition_ref: String,
    independent_lock_transition_fingerprint: FingerprintV1,
    independent_lock_resulting_lifecycle_fingerprint: FingerprintV1,
    active_transition_ref: String,
    active_transition_fingerprint: FingerprintV1,
    active_resulting_lifecycle_fingerprint: FingerprintV1,
    basis_ref: String,
    basis_fingerprint: FingerprintV1,
}

impl EvaluationLifecycleAdmissionBasisV1 {
    pub fn independent_lock_transition_ref(&self) -> &str {
        &self.independent_lock_transition_ref
    }

    pub fn active_transition_ref(&self) -> &str {
        &self.active_transition_ref
    }

    pub fn active_resulting_lifecycle_fingerprint(&self) -> &FingerprintV1 {
        &self.active_resulting_lifecycle_fingerprint
    }

    pub fn basis_ref(&self) -> &str {
        &self.basis_ref
    }

    pub fn basis_fingerprint(&self) -> &FingerprintV1 {
        &self.basis_fingerprint
    }
}

pub fn evaluation_lifecycle_admission_basis_v1(
    definition: &AdmittedContractDefinitionV1,
    chain: &[ContractLifecycleTransitionV1],
) -> Result<EvaluationLifecycleAdmissionBasisV1, ContractLifecycleErrorV1> {
    let head = current_lifecycle_head(definition, chain)?;
    if head.state != LifecycleStateV1::Active || chain.len() < 2 {
        return Err(ContractLifecycleErrorV1::InvalidChain);
    }
    let active = &chain[chain.len() - 1];
    let lock = &chain[chain.len() - 2];
    if (lock.from_state, lock.to_state) != (LifecycleStateV1::ReviewReady, LifecycleStateV1::Locked)
        || (active.from_state, active.to_state)
            != (LifecycleStateV1::Locked, LifecycleStateV1::Active)
        || active.prior_transition_ref.as_deref() != Some(lock.transition_ref.as_str())
        || active.prior_lifecycle_fingerprint != lock.resulting_lifecycle_fingerprint
        || lock.actor_ref == *definition.definition.definition_author_ref()
        || head.transition_ref.as_deref() != Some(active.transition_ref.as_str())
        || head.lifecycle_fingerprint != active.resulting_lifecycle_fingerprint
    {
        return Err(ContractLifecycleErrorV1::InvalidChain);
    }
    let policy = lifecycle_policy_binding_v1()?;
    let preimage = BasisFingerprintPreimage {
        schema_ref: BASIS_SCHEMA_REF,
        contract_ref: definition.definition.contract_ref(),
        definition_fingerprint: definition.definition.definition_fingerprint(),
        lifecycle_policy_ref: policy.reference(),
        lifecycle_policy_fingerprint: policy.fingerprint(),
        independent_lock_transition_ref: &lock.transition_ref,
        independent_lock_transition_fingerprint: &lock.transition_fingerprint,
        independent_lock_resulting_lifecycle_fingerprint: &lock.resulting_lifecycle_fingerprint,
        active_transition_ref: &active.transition_ref,
        active_transition_fingerprint: &active.transition_fingerprint,
        active_resulting_lifecycle_fingerprint: &active.resulting_lifecycle_fingerprint,
    };
    let basis_fingerprint = canonical_fingerprint(&preimage)?;
    let basis_ref = format!(
        "{}#evaluation-lifecycle-basis/{}",
        definition.definition.contract_ref(),
        fingerprint_hex(&basis_fingerprint)
    );
    Ok(EvaluationLifecycleAdmissionBasisV1 {
        schema_ref: BASIS_SCHEMA_REF,
        contract_ref: definition.definition.contract_ref().clone(),
        definition_fingerprint: definition.definition.definition_fingerprint().clone(),
        lifecycle_policy_ref: policy.reference().clone(),
        lifecycle_policy_fingerprint: policy.fingerprint().clone(),
        independent_lock_transition_ref: lock.transition_ref.clone(),
        independent_lock_transition_fingerprint: lock.transition_fingerprint.clone(),
        independent_lock_resulting_lifecycle_fingerprint: lock
            .resulting_lifecycle_fingerprint
            .clone(),
        active_transition_ref: active.transition_ref.clone(),
        active_transition_fingerprint: active.transition_fingerprint.clone(),
        active_resulting_lifecycle_fingerprint: active.resulting_lifecycle_fingerprint.clone(),
        basis_ref,
        basis_fingerprint,
    })
}

fn derive_genesis(
    definition: &ContractDefinitionV1,
) -> Result<DraftGenesisV1, ContractLifecycleErrorV1> {
    require_exact_policy(definition)?;
    let policy = lifecycle_policy_binding_v1()?;
    let preimage = GenesisFingerprintPreimage {
        schema_ref: GENESIS_SCHEMA_REF,
        contract_ref: definition.contract_ref(),
        definition_fingerprint: definition.definition_fingerprint(),
        lifecycle_policy_ref: policy.reference(),
        lifecycle_policy_fingerprint: policy.fingerprint(),
        state: LifecycleStateV1::Draft,
        prior_transition_ref: None,
    };
    let lifecycle_fingerprint = canonical_fingerprint(&preimage)?;
    Ok(DraftGenesisV1 {
        schema_ref: GENESIS_SCHEMA_REF,
        contract_ref: definition.contract_ref().clone(),
        definition_fingerprint: definition.definition_fingerprint().clone(),
        lifecycle_policy_ref: policy.reference().clone(),
        lifecycle_policy_fingerprint: policy.fingerprint().clone(),
        state: LifecycleStateV1::Draft,
        prior_transition_ref: None,
        lifecycle_fingerprint,
    })
}

fn require_exact_policy(definition: &ContractDefinitionV1) -> Result<(), ContractLifecycleErrorV1> {
    if definition.lifecycle_policy() != &lifecycle_policy_binding_v1()? {
        return Err(ContractLifecycleErrorV1::InvalidPolicy);
    }
    Ok(())
}

fn validate_admission_scope(
    admission: &CredentialAuthorityAdmissionV1<'_>,
    action_ref: &str,
    subject_ref: &str,
    approval_class: ContractApprovalClassV1,
    semantic_scope_fingerprint: &FingerprintV1,
) -> Result<(), ContractLifecycleErrorV1> {
    if admission.action_ref() != action_ref
        || admission.subject_ref() != subject_ref
        || admission.approval_class() != approval_class.as_str()
        || admission.semantic_scope_fingerprint() != semantic_scope_fingerprint.as_str()
    {
        return Err(ContractLifecycleErrorV1::InvalidAdmission);
    }
    Ok(())
}

fn admission_record(
    admission: &CredentialAuthorityAdmissionV1<'_>,
) -> Result<ContractAuthorityAdmissionRecordV1, ContractLifecycleErrorV1> {
    for reference in [
        admission.registry_state_ref(),
        admission.registry_head_transition_ref(),
        admission.credential_registration_ref(),
    ] {
        if !valid_authority_record_ref(reference) {
            return Err(ContractLifecycleErrorV1::InvalidAdmission);
        }
    }
    Ok(ContractAuthorityAdmissionRecordV1 {
        repository_identity_fingerprint: FingerprintV1::parse(
            admission.repository_identity_fingerprint(),
        )?,
        registry_state_ref: admission.registry_state_ref().to_owned(),
        registry_state_fingerprint: FingerprintV1::parse(admission.registry_state_fingerprint())?,
        registry_head_transition_ref: admission.registry_head_transition_ref().to_owned(),
        registry_head_transition_fingerprint: FingerprintV1::parse(
            admission.registry_head_transition_fingerprint(),
        )?,
        credential_registration_ref: admission.credential_registration_ref().to_owned(),
        credential_registration_fingerprint: FingerprintV1::parse(
            admission.credential_registration_fingerprint(),
        )?,
        authenticator_assertion_fingerprint: FingerprintV1::parse(
            admission.authenticator_assertion_fingerprint(),
        )?,
        semantic_scope_fingerprint: FingerprintV1::parse(admission.semantic_scope_fingerprint())?,
    })
}

fn validate_actor_rule(
    definition: &AdmittedContractDefinitionV1,
    from: LifecycleStateV1,
    to: LifecycleStateV1,
    class: ContractApprovalClassV1,
    actor_ref: &str,
    authority_ref: &str,
) -> Result<(), ContractLifecycleErrorV1> {
    let author_path = class == ContractApprovalClassV1::DefinitionAuthor;
    if author_path
        && (actor_ref != definition.definition.definition_author_ref().as_str()
            || authority_ref
                != definition
                    .definition
                    .definition_authority()
                    .reference()
                    .as_str())
    {
        return Err(ContractLifecycleErrorV1::InvalidRole);
    }
    if (from, to) == (LifecycleStateV1::ReviewReady, LifecycleStateV1::Locked)
        && actor_ref == definition.definition.definition_author_ref().as_str()
    {
        return Err(ContractLifecycleErrorV1::SelfLock);
    }
    Ok(())
}

fn validate_edge_inputs(
    from: LifecycleStateV1,
    to: LifecycleStateV1,
    class: ContractApprovalClassV1,
    reason: &LifecycleReasonV1,
    evidence: &[LifecycleSupportingEvidenceV1],
) -> Result<(), ContractLifecycleErrorV1> {
    if evidence.len() > MAX_SUPPORTING_EVIDENCE {
        return Err(ContractLifecycleErrorV1::BoundExceeded);
    }
    let mut bindings = BTreeSet::new();
    if evidence.iter().any(|entry| {
        !bindings.insert((
            entry.binding.reference().clone(),
            entry.binding.fingerprint().clone(),
        ))
    }) {
        return Err(ContractLifecycleErrorV1::InvalidEvidence);
    }
    use ContractApprovalClassV1 as Class;
    use LifecycleEvidenceKindV1 as Evidence;
    use LifecycleReasonV1 as Reason;
    use LifecycleStateV1 as State;
    let expected_kinds: &[Evidence] = match (from, to, class, reason) {
        (State::Draft, State::ReviewReady, Class::DefinitionAuthor, Reason::ReadyForReview) => {
            &[Evidence::DefinitionValidation]
        }
        (
            State::Draft,
            State::Closed,
            Class::DefinitionAuthor | Class::Lifecycle,
            Reason::Withdrawn,
        ) => &[],
        (
            State::ReviewReady,
            State::Locked,
            Class::IndependentLock,
            Reason::IndependentlyLocked,
        ) => &[Evidence::IndependentReview],
        (State::ReviewReady, State::Closed, Class::IndependentLock, Reason::Rejected) => {
            &[Evidence::Rejection]
        }
        (State::Locked, State::Active, Class::Activation, Reason::Activated) => &[
            Evidence::ApplicabilityClosure,
            Evidence::MatcherClosure,
            Evidence::PolicyClosure,
            Evidence::CurrentnessClosure,
        ],
        (
            State::Locked | State::Active,
            State::Deprecated,
            Class::Lifecycle,
            Reason::DeprecatedWithReplacement(replacement),
        ) => {
            if evidence.len() != 1 || evidence[0].binding != *replacement {
                return Err(ContractLifecycleErrorV1::InvalidEvidence);
            }
            &[Evidence::Replacement]
        }
        (
            State::Locked | State::Active,
            State::Deprecated,
            Class::Lifecycle,
            Reason::DeprecatedWithoutReplacement(_),
        ) => &[Evidence::NoReplacementRationale],
        (State::Deprecated, State::Closed, Class::Lifecycle, Reason::ClosedAfterRetention) => {
            &[Evidence::RetentionSafety, Evidence::ReferenceSafety]
        }
        _ if legal_edge(from, to) => return Err(ContractLifecycleErrorV1::InvalidRole),
        _ => return Err(ContractLifecycleErrorV1::InvalidEdge),
    };
    if evidence
        .iter()
        .map(|entry| entry.kind)
        .ne(expected_kinds.iter().copied())
    {
        return Err(ContractLifecycleErrorV1::InvalidEvidence);
    }
    Ok(())
}

fn legal_edge(from: LifecycleStateV1, to: LifecycleStateV1) -> bool {
    use LifecycleStateV1 as State;
    matches!(
        (from, to),
        (State::Draft, State::ReviewReady | State::Closed)
            | (State::ReviewReady, State::Locked | State::Closed)
            | (State::Locked, State::Active | State::Deprecated)
            | (State::Active, State::Deprecated)
            | (State::Deprecated, State::Closed)
    )
}

fn validate_chain_transition(
    definition: &AdmittedContractDefinitionV1,
    head: &LifecycleHeadV1,
    transition: &ContractLifecycleTransitionV1,
) -> Result<(), ContractLifecycleErrorV1> {
    let policy = lifecycle_policy_binding_v1()?;
    if transition.schema_ref != TRANSITION_ACTION_REF
        || transition.contract_ref != *definition.definition.contract_ref()
        || transition.definition_fingerprint != *definition.definition.definition_fingerprint()
        || transition.prior_transition_ref != head.transition_ref
        || transition.prior_lifecycle_fingerprint != head.lifecycle_fingerprint
        || transition.from_state != head.state
        || transition.transition_policy_ref != *policy.reference()
        || transition.transition_policy_fingerprint != *policy.fingerprint()
    {
        return Err(ContractLifecycleErrorV1::InvalidChain);
    }
    validate_edge_inputs(
        transition.from_state,
        transition.to_state,
        transition.approval_class,
        &transition.reason,
        &transition.supporting_evidence,
    )?;
    validate_actor_rule(
        definition,
        transition.from_state,
        transition.to_state,
        transition.approval_class,
        transition.actor_ref.as_str(),
        transition.authority_ref.as_str(),
    )?;
    let scope = canonical_fingerprint(&TransitionScopePreimage {
        schema_ref: "handbook.contract-lifecycle-transition-scope@1.0.0",
        contract_ref: &transition.contract_ref,
        definition_fingerprint: &transition.definition_fingerprint,
        prior_transition_ref: transition.prior_transition_ref.as_deref(),
        prior_lifecycle_fingerprint: &transition.prior_lifecycle_fingerprint,
        from_state: transition.from_state,
        to_state: transition.to_state,
        approval_class: transition.approval_class,
        transition_policy: &policy,
        supporting_evidence: &transition.supporting_evidence,
        reason: &transition.reason,
    })?;
    if scope != transition.authority_admission.semantic_scope_fingerprint {
        return Err(ContractLifecycleErrorV1::InvalidChain);
    }
    let fingerprint = canonical_fingerprint(&TransitionFingerprintPreimage {
        schema_ref: TRANSITION_ACTION_REF,
        contract_ref: &transition.contract_ref,
        definition_fingerprint: &transition.definition_fingerprint,
        prior_transition_ref: transition.prior_transition_ref.as_deref(),
        prior_lifecycle_fingerprint: &transition.prior_lifecycle_fingerprint,
        from_state: transition.from_state,
        to_state: transition.to_state,
        actor_ref: &transition.actor_ref,
        authority_ref: &transition.authority_ref,
        authority_admission: &transition.authority_admission,
        transition_policy_ref: &transition.transition_policy_ref,
        transition_policy_fingerprint: &transition.transition_policy_fingerprint,
        supporting_evidence: &transition.supporting_evidence,
        reason: &transition.reason,
        decision_time: &transition.decision_time,
    })?;
    if fingerprint != transition.transition_fingerprint
        || transition.transition_ref != transition_ref(&transition.contract_ref, &fingerprint)
    {
        return Err(ContractLifecycleErrorV1::InvalidChain);
    }
    let result = canonical_fingerprint(&ResultFingerprintPreimage {
        schema_ref: RESULT_SCHEMA_REF,
        contract_ref: &transition.contract_ref,
        definition_fingerprint: &transition.definition_fingerprint,
        state: transition.to_state,
        prior_transition_ref: transition.prior_transition_ref.as_deref(),
        prior_lifecycle_fingerprint: &transition.prior_lifecycle_fingerprint,
        transition_ref: &transition.transition_ref,
        transition_fingerprint: &transition.transition_fingerprint,
    })?;
    if result != transition.resulting_lifecycle_fingerprint {
        return Err(ContractLifecycleErrorV1::InvalidChain);
    }
    Ok(())
}

fn transition_ref(contract_ref: &ExactRefV1, fingerprint: &FingerprintV1) -> String {
    format!("{contract_ref}#transition/{}", fingerprint_hex(fingerprint))
}

fn fingerprint_hex(fingerprint: &FingerprintV1) -> &str {
    fingerprint
        .as_str()
        .strip_prefix("sha256:")
        .expect("fingerprint type is normalized")
}

fn valid_authority_record_ref(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 512
        && value.is_ascii()
        && !value.starts_with('/')
        && !value.contains('\\')
        && !value
            .split('/')
            .any(|segment| segment.is_empty() || segment == "." || segment == "..")
}

fn valid_decision_time(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 20
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'T'
        && bytes[13] == b':'
        && bytes[16] == b':'
        && bytes[19] == b'Z'
        && bytes.iter().enumerate().all(|(index, byte)| {
            matches!(index, 4 | 7 | 10 | 13 | 16 | 19) || byte.is_ascii_digit()
        })
}
