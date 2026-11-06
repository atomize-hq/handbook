//! Unified SDK governance data. Native owner versions remain identity backing.
use super::{
    AdmissionConfigurationInputV1, AuthorityModeV1, ExpectedInitialStateV1, GovernanceRefusalV1,
    GovernanceRuleDescriptorV1, LocalAdvisoryInstructionV1, ReplacementAuthorityBasisV1,
};
use crate::FingerprintV1;
use handbook_engine::RepositoryInitializationClaimV1;
use serde::{Deserialize, Serialize};

/// The exact registered project and physical root selected by the owner.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceProjectV1 {
    pub repository_identity_fingerprint: String,
    pub retained_root: String,
    pub physical_root_fingerprint: String,
    pub registration_identity: FingerprintV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InitialDeferralChoiceV1 {
    DeferInitialGovernance,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InitialDeferralInstructionV1 {
    pub mode: AuthorityModeV1,
    pub choice: InitialDeferralChoiceV1,
    pub human_approval_asserted: bool,
    pub text: String,
    pub asserted_principal: Option<String>,
    pub asserted_agent: Option<String>,
    pub asserted_delegation: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InitialDeferralRequestV1 {
    pub scope: String,
    pub instruction: InitialDeferralInstructionV1,
    pub expected_initial_state: ExpectedInitialStateV1,
    pub request_key: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InitialEstablishmentRequestV1 {
    pub scope: String,
    pub configuration: AdmissionConfigurationInputV1,
    pub instruction: LocalAdvisoryInstructionV1,
    pub replacement_basis: ReplacementAuthorityBasisV1,
    pub expected_initial_state: ExpectedInitialStateV1,
    pub request_key: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeferredRevisionV1 {
    pub outcome_fingerprint: FingerprintV1,
    pub relation_fingerprint: FingerprintV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceEstablishmentInstructionV1 {
    pub mode: AuthorityModeV1,
    pub human_approval_asserted: bool,
    pub approved_content_fingerprint: FingerprintV1,
    pub text: String,
    #[serde(deserialize_with = "required_nullable_claim")]
    pub asserted_principal: Option<String>,
    #[serde(deserialize_with = "required_nullable_claim")]
    pub asserted_agent: Option<String>,
    #[serde(deserialize_with = "required_nullable_claim")]
    pub asserted_delegation: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceEstablishmentRequestV1 {
    pub project: GovernanceProjectV1,
    pub scope: String,
    pub expected_deferred: DeferredRevisionV1,
    pub configuration: AdmissionConfigurationInputV1,
    pub replacement_basis: ReplacementAuthorityBasisV1,
    pub instruction: GovernanceEstablishmentInstructionV1,
    pub request_key: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InitialDeferralLocatorV1 {
    pub project: GovernanceProjectV1,
    pub request: InitialDeferralRequestV1,
    pub request_fingerprint: FingerprintV1,
    pub initialization_claim: RepositoryInitializationClaimV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InitialEstablishmentLocatorV1 {
    pub project: GovernanceProjectV1,
    pub request: InitialEstablishmentRequestV1,
    pub request_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceEstablishmentLocatorV1 {
    pub request: GovernanceEstablishmentRequestV1,
    pub request_fingerprint: FingerprintV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatedInitialDeferralV1 {
    pub input: InitialDeferralRequestV1,
    pub instruction_fingerprint: FingerprintV1,
    pub establishment_rule: GovernanceRuleDescriptorV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatedInitialEstablishmentV1 {
    pub input: InitialEstablishmentRequestV1,
    pub configuration_fingerprint: FingerprintV1,
    pub instruction_fingerprint: FingerprintV1,
    pub establishment_rule: GovernanceRuleDescriptorV1,
    pub owner_rule: GovernanceRuleDescriptorV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatedGovernanceEstablishmentV1 {
    pub input: GovernanceEstablishmentRequestV1,
    pub configuration_fingerprint: FingerprintV1,
    pub instruction_fingerprint: FingerprintV1,
    pub establishment_rule: GovernanceRuleDescriptorV1,
    pub owner_rule: GovernanceRuleDescriptorV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InitialDeferralOutcomeV1 {
    pub locator: InitialDeferralLocatorV1,
    pub validated: ValidatedInitialDeferralV1,
    pub validation_confirmed: bool,
    pub advisory_permission_accepted: bool,
    pub deferral_committed: bool,
    pub outcome_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InitialEstablishmentOutcomeV1 {
    pub locator: InitialEstablishmentLocatorV1,
    pub validated: ValidatedInitialEstablishmentV1,
    pub validation_confirmed: bool,
    pub advisory_permission_accepted: bool,
    pub activation_committed: bool,
    pub replacement_basis: ReplacementAuthorityBasisV1,
    pub outcome_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceEstablishmentOutcomeV1 {
    pub locator: GovernanceEstablishmentLocatorV1,
    pub validated: ValidatedGovernanceEstablishmentV1,
    pub validation_confirmed: bool,
    pub advisory_permission_accepted: bool,
    pub activation_committed: bool,
    pub validation_fingerprint: FingerprintV1,
    pub advisory_permission_fingerprint: FingerprintV1,
    pub outcome_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InitialDeferralNoncommitV1 {
    pub locator: InitialDeferralLocatorV1,
    pub condition: String,
    pub deferral_committed: bool,
    pub outcome_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InitialEstablishmentNoncommitV1 {
    pub locator: InitialEstablishmentLocatorV1,
    pub condition: String,
    pub activation_committed: bool,
    pub outcome_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceEstablishmentNoncommitV1 {
    pub locator: GovernanceEstablishmentLocatorV1,
    pub condition: String,
    pub activation_committed: bool,
    pub outcome_fingerprint: FingerprintV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeferredGovernanceV1 {
    pub project: GovernanceProjectV1,
    pub scope: String,
    pub never_established: bool,
    pub registration_identity: FingerprintV1,
    pub deferral_fingerprint: FingerprintV1,
    pub outcome_fingerprint: FingerprintV1,
    pub relation_fingerprint: FingerprintV1,
    pub establishment_rule: GovernanceRuleDescriptorV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InitiallyEstablishedGovernanceV1 {
    pub project: GovernanceProjectV1,
    pub scope: String,
    pub configuration: AdmissionConfigurationInputV1,
    pub configuration_fingerprint: FingerprintV1,
    pub establishment_rule: GovernanceRuleDescriptorV1,
    pub owner_rule: GovernanceRuleDescriptorV1,
    pub outcome_fingerprint: FingerprintV1,
    pub active_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EstablishedDeferredGovernanceV1 {
    pub locator: GovernanceEstablishmentLocatorV1,
    pub project: GovernanceProjectV1,
    pub scope: String,
    pub predecessor: DeferredRevisionV1,
    pub configuration: AdmissionConfigurationInputV1,
    pub configuration_fingerprint: FingerprintV1,
    pub replacement_basis: ReplacementAuthorityBasisV1,
    pub establishment_rule: GovernanceRuleDescriptorV1,
    pub owner_rule: GovernanceRuleDescriptorV1,
    pub request_fingerprint: FingerprintV1,
    pub outcome_fingerprint: FingerprintV1,
    pub relation_fingerprint: FingerprintV1,
}

// Keeps conversion closed and lossless: the owner validates every restored native
// locator before use. These semantic DTO bytes never become owner hash inputs.
pub(crate) fn translate<T: Serialize, U: serde::de::DeserializeOwned>(
    value: &T,
) -> Result<U, GovernanceRefusalV1> {
    serde_json::to_value(value)
        .and_then(serde_json::from_value)
        .map_err(|error| GovernanceRefusalV1::new("governance_projection", error.to_string()))
}

fn required_nullable_claim<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    Option::<String>::deserialize(deserializer)
}

impl InitialDeferralRequestV1 {
    pub fn human_approved(
        key: impl Into<String>,
        text: impl Into<String>,
    ) -> Result<Self, GovernanceRefusalV1> {
        translate(&super::DeferralRequestInputV2::human_approved(key, text))
    }
}
impl InitialEstablishmentRequestV1 {
    pub fn standalone(
        key: impl Into<String>,
        ceiling: u64,
        text: impl Into<String>,
    ) -> Result<Self, GovernanceRefusalV1> {
        translate(
            &super::GovernanceRequestInputV1::standalone(
                key, ceiling, text,
            ),
        )
    }
}
impl GovernanceEstablishmentRequestV1 {
    pub fn human_approved(
        project: GovernanceProjectV1,
        predecessor: DeferredRevisionV1,
        ceiling: u64,
        key: impl Into<String>,
        text: impl Into<String>,
    ) -> Result<Self, GovernanceRefusalV1> {
        let project = translate(&project)?;
        let predecessor = translate(&predecessor)?;
        let request = super::DeferredEstablishmentRequestV3::human_approved(project, predecessor, ceiling, key, text)?;
        translate(&request)
    }
}

impl InitialDeferralLocatorV1 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, GovernanceRefusalV1> {
        let native: super::DeferralRequestLocatorV2 =
            translate(self)?;
        native.canonical_json_bytes()
    }
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, GovernanceRefusalV1> {
        let native = super::DeferralRequestLocatorV2::from_json_bytes(bytes)?;
        translate(&native)
    }
}
impl DeferredGovernanceV1 {
    pub fn native_canonical_bytes(&self) -> Result<Vec<u8>, GovernanceRefusalV1> {
        let native: super::DeferredGovernanceBasisV2 =
            translate(self)?;
        native.canonical_json_bytes()
    }
}
impl EstablishedDeferredGovernanceV1 {
    pub fn native_canonical_bytes(&self) -> Result<Vec<u8>, GovernanceRefusalV1> {
        let native: super::EstablishedFromDeferredBasisV3 = translate(self)?;
        native.canonical_json_bytes()
    }
}

impl From<&GovernanceProjectV1>
    for super::ProjectBindingV2
{
    fn from(project: &GovernanceProjectV1) -> Self {
        Self {
            repository_identity_fingerprint: project.repository_identity_fingerprint.clone(),
            retained_root: project.retained_root.clone(),
            physical_root_fingerprint: project.physical_root_fingerprint.clone(),
            registration_identity: project.registration_identity.clone(),
        }
    }
}
