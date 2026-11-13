//! Closed v2 scoped-choice input and exact identities. No configuration is admitted.
use super::types::{decode_governance_json, governance_canonical_bytes, governance_fingerprint};
use super::{
    AuthorityModeV1, GovernanceRefusalV1, ProjectBindingV1, ADMISSION_GOVERNANCE_SCOPE_V1,
    ESTABLISHMENT_RULE_NAME_V1,
};
use crate::{ExactBindingV1, ExactRefV1, FingerprintV1};
use handbook_engine::RepositoryInitializationClaimV1;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const DEFERRAL_NATIVE_OWNER_V2: &str = "handbook-contracts.admission-deferral@2.0.0";
pub const GOVERNANCE_RULE_VERSION_V2: &str = "2.0.0";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InitialGovernanceChoiceV2 {
    DeferInitialGovernance,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpectedInitialStateV2 {
    FreshRegistrationUnused,
}

/// Human approval is a retained advisory assertion, never authenticated authority.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HumanApprovedInitialInstructionV2 {
    pub mode: AuthorityModeV1,
    pub choice: InitialGovernanceChoiceV2,
    pub human_approval_asserted: bool,
    pub text: String,
    pub asserted_principal: Option<String>,
    pub asserted_agent: Option<String>,
    pub asserted_delegation: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeferralRequestInputV2 {
    pub scope: String,
    pub instruction: HumanApprovedInitialInstructionV2,
    pub expected_initial_state: ExpectedInitialStateV2,
    pub request_key: String,
}
impl DeferralRequestInputV2 {
    pub fn human_approved(request_key: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            scope: ADMISSION_GOVERNANCE_SCOPE_V1.into(),
            request_key: request_key.into(),
            expected_initial_state: ExpectedInitialStateV2::FreshRegistrationUnused,
            instruction: HumanApprovedInitialInstructionV2 {
                mode: AuthorityModeV1::StandaloneLocalAdvisory,
                choice: InitialGovernanceChoiceV2::DeferInitialGovernance,
                human_approval_asserted: true,
                text: text.into(),
                asserted_principal: None,
                asserted_agent: None,
                asserted_delegation: None,
            },
        }
    }
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, GovernanceRefusalV1> {
        Ok(validate_deferral_request(decode_governance_json(bytes)?)?.input)
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, GovernanceRefusalV1> {
        validate_deferral_request(self.clone())?;
        governance_canonical_bytes(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectBindingV2 {
    pub repository_identity_fingerprint: String,
    pub retained_root: String,
    pub physical_root_fingerprint: String,
    pub registration_identity: FingerprintV1,
}
impl ProjectBindingV2 {
    pub(crate) fn refusal_project(&self) -> ProjectBindingV1 {
        ProjectBindingV1 {
            repository_identity_fingerprint: self.repository_identity_fingerprint.clone(),
            retained_root: self.retained_root.clone(),
            physical_root_fingerprint: self.physical_root_fingerprint.clone(),
            registration_identity: self.registration_identity.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeferralRequestLocatorV2 {
    pub project: ProjectBindingV2,
    pub request: DeferralRequestInputV2,
    pub request_fingerprint: FingerprintV1,
    pub initialization_claim: RepositoryInitializationClaimV1,
}
impl DeferralRequestLocatorV2 {
    /// Restoration grants historical resolution only; there is no creation token.
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, GovernanceRefusalV1> {
        let result = decode_governance_json(bytes)?;
        validate_deferral_locator(&result)?;
        Ok(result)
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, GovernanceRefusalV1> {
        validate_deferral_locator(self)?;
        governance_canonical_bytes(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceRuleDescriptorV2 {
    pub owner: String,
    pub name: String,
    pub version: String,
    pub candidate_kind: Option<String>,
    pub candidate_schema_ref: Option<String>,
    pub required_authored_sources: Vec<String>,
    pub approval_sources: Vec<String>,
    pub exception_sources: Vec<String>,
    pub constraints: Vec<String>,
    pub binding: ExactBindingV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatedDeferralV2 {
    pub input: DeferralRequestInputV2,
    pub instruction_fingerprint: FingerprintV1,
    pub establishment_rule: GovernanceRuleDescriptorV2,
}

pub fn validate_deferral_request(
    input: DeferralRequestInputV2,
) -> Result<ValidatedDeferralV2, GovernanceRefusalV1> {
    if input.scope != ADMISSION_GOVERNANCE_SCOPE_V1 {
        return Err(GovernanceRefusalV1::new(
            "scope",
            "Only the exact Contracts root freshness scope is supported",
        ));
    }
    let bounded =
        |text: &str, limit| !text.trim().is_empty() && text.len() <= limit && !text.contains('\0');
    if !input.instruction.human_approval_asserted
        || !bounded(&input.instruction.text, 4096)
        || [
            &input.instruction.asserted_principal,
            &input.instruction.asserted_agent,
            &input.instruction.asserted_delegation,
        ]
        .iter()
        .any(|claim| claim.as_ref().is_some_and(|text| !bounded(text, 4096)))
    {
        return Err(GovernanceRefusalV1::new(
            "human_approved_initial_instruction",
            "Explicit approved initial deferral and bounded advisory content are required",
        ));
    }
    if !bounded(&input.request_key, 128)
        || !input
            .request_key
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err(GovernanceRefusalV1::new(
            "request_key",
            "Request key must be 1..128 ASCII letters, digits, hyphens, underscores or periods",
        ));
    }
    governance_canonical_bytes(&input)?;
    Ok(ValidatedDeferralV2 {
        instruction_fingerprint: governance_fingerprint(
            "handbook.project-admission-governance.local-advisory-instruction@2.0.0",
            &input.instruction,
        )?,
        establishment_rule: establishment_rule_descriptor_v2()?,
        input,
    })
}

pub(crate) fn deferral_registration_fingerprint(
    identity: &str,
    root: &str,
    physical: &str,
) -> Result<FingerprintV1, GovernanceRefusalV1> {
    governance_fingerprint(
        "handbook.admission-registration@2.0.0",
        &(identity, root, physical),
    )
}
pub(crate) fn deferral_request_fingerprint(
    project: &ProjectBindingV2,
    request: &DeferralRequestInputV2,
) -> Result<FingerprintV1, GovernanceRefusalV1> {
    #[derive(Serialize)]
    struct Request<'a> {
        project: &'a ProjectBindingV2,
        request: &'a DeferralRequestInputV2,
    }
    governance_fingerprint(
        "handbook.project-admission-governance.request@2.0.0",
        &Request { project, request },
    )
}
pub fn validate_deferral_locator(
    locator: &DeferralRequestLocatorV2,
) -> Result<ValidatedDeferralV2, GovernanceRefusalV1> {
    let p = &locator.project;
    if FingerprintV1::parse(&p.repository_identity_fingerprint).is_err()
        || FingerprintV1::parse(&p.physical_root_fingerprint).is_err()
        || p.retained_root.len() > 4096
        || !Path::new(&p.retained_root).is_absolute()
        || p.registration_identity
            != deferral_registration_fingerprint(
                &p.repository_identity_fingerprint,
                &p.retained_root,
                &p.physical_root_fingerprint,
            )?
    {
        return Err(GovernanceRefusalV1::new(
            "project_binding",
            "Exact stable v2 project binding is required",
        ));
    }
    let validated = validate_deferral_request(locator.request.clone())?;
    let claim = &locator.initialization_claim;
    if deferral_request_fingerprint(p, &locator.request)? != locator.request_fingerprint
        || claim.native_owner() != DEFERRAL_NATIVE_OWNER_V2
        || claim.request_digest() != locator.request_fingerprint.as_str()
        || claim.proposed_repository_identity() != p.repository_identity_fingerprint
        || claim.physical_root_fingerprint() != p.physical_root_fingerprint
    {
        return Err(GovernanceRefusalV1::new(
            "request_identity",
            "The complete exact request and Engine claim must agree",
        )
        .with_project(p.refusal_project()));
    }
    claim.canonical_bytes().map_err(super::fs_refusal)?;
    governance_canonical_bytes(locator)?;
    Ok(validated)
}

pub fn establishment_rule_descriptor_v2() -> Result<GovernanceRuleDescriptorV2, GovernanceRefusalV1>
{
    let constraints: Vec<String> = [
        "positive_fresh_project_registration", "exact_single_project_and_scope",
        "explicit_human_approved_initial_local_advisory_instruction", "separate_validation_permission_and_commit",
        "deferral_has_no_configuration_or_owner_rule_qualification", "positive_complete_never_established_lifecycle",
        "stable_registration_distinct_from_consumed_initial_choice", "immutable_original_request_instruction_deferral_and_outcome",
        "exact_terminal_closed_current_relation", "historical_replay_never_restores_current_relation",
        "same_identity_later_first_establishment_requires_exact_current_deferred_predecessor_and_new_human_approved_terms",
        "unsupported_transitions_refuse", "no_scope_to_evaluation_or_action_authority",
    ].iter().map(|value| (*value).into()).collect();
    #[derive(Serialize)]
    struct Content<'a> {
        owner: &'a str,
        name: &'a str,
        version: &'a str,
        candidate_kind: Option<&'a str>,
        candidate_schema_ref: Option<&'a str>,
        required_authored_sources: Vec<String>,
        approval_sources: Vec<String>,
        exception_sources: Vec<String>,
        constraints: &'a Vec<String>,
    }
    let content = Content {
        owner: "handbook",
        name: ESTABLISHMENT_RULE_NAME_V1,
        version: GOVERNANCE_RULE_VERSION_V2,
        candidate_kind: None,
        candidate_schema_ref: None,
        required_authored_sources: vec![],
        approval_sources: vec![],
        exception_sources: vec![],
        constraints: &constraints,
    };
    let binding = ExactBindingV1::new(
        ExactRefV1::parse("handbook.admission-establishment.standalone@2.0.0")
            .map_err(|_| GovernanceRefusalV1::new("rule", "Invalid shipped rule reference"))?,
        governance_fingerprint(
            "handbook.project-admission-governance.shipped-rule@2.0.0",
            &content,
        )?,
    );
    Ok(GovernanceRuleDescriptorV2 {
        owner: "handbook".into(),
        name: ESTABLISHMENT_RULE_NAME_V1.into(),
        version: GOVERNANCE_RULE_VERSION_V2.into(),
        candidate_kind: None,
        candidate_schema_ref: None,
        required_authored_sources: vec![],
        approval_sources: vec![],
        exception_sources: vec![],
        constraints,
        binding,
    })
}
