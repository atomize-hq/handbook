//! Closed first-establishment inputs for an existing positively deferred project.
//! Human approval is an exact retained advisory assertion, not authentication.

use super::deferral_types::deferral_registration_fingerprint;
use super::types::{decode_governance_json, governance_canonical_bytes, governance_fingerprint};
use super::{
    establishment_rule_descriptor_v2, freshness_admission_rule_descriptor_v1, validate_request,
    AdmissionConfigurationInputV1, AuthorityModeV1, ExpectedDeferredBasisV2,
    ExpectedInitialStateV1, GovernanceRefusalV1, GovernanceRequestInputV1,
    GovernanceRuleDescriptorV1, GovernanceRuleDescriptorV2, LocalAdvisoryInstructionV1,
    ProjectBindingV2, ReplacementAuthorityBasisV1, ESTABLISHMENT_RULE_NAME_V1,
};
use crate::{ExactBindingV1, ExactRefV1, FingerprintV1};
use serde::{Deserialize, Deserializer, Serialize};
use std::path::Path;

pub const GOVERNANCE_RULE_VERSION_V3: &str = "3.0.0";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HumanApprovedEstablishmentInstructionV3 {
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

// A complete new wire shape retains nullable claim keys. Missing keys cannot
// silently become null; this changes no v1/v2 decoder.
fn required_nullable_claim<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeferredEstablishmentRequestV3 {
    pub project: ProjectBindingV2,
    pub scope: String,
    pub expected_deferred: ExpectedDeferredBasisV2,
    pub configuration: AdmissionConfigurationInputV1,
    pub replacement_basis: ReplacementAuthorityBasisV1,
    pub instruction: HumanApprovedEstablishmentInstructionV3,
    pub request_key: String,
}

impl DeferredEstablishmentRequestV3 {
    /// Records a caller's explicit assertion that the human has already approved
    /// this exact project, deferred basis, configuration and replacement terms.
    /// The caller must have that approval before choosing this constructor.
    /// This assertion grants no project eligibility or authenticated authority.
    pub fn human_approved(
        project: ProjectBindingV2,
        expected_deferred: ExpectedDeferredBasisV2,
        max_observation_age_seconds: u64,
        request_key: impl Into<String>,
        instruction_text: impl Into<String>,
    ) -> Result<Self, GovernanceRefusalV1> {
        let fixed = GovernanceRequestInputV1::standalone(
            request_key,
            max_observation_age_seconds,
            instruction_text,
        );
        let mut input = Self {
            project,
            scope: fixed.scope,
            expected_deferred,
            configuration: fixed.configuration,
            replacement_basis: fixed.replacement_basis,
            instruction: HumanApprovedEstablishmentInstructionV3 {
                mode: fixed.instruction.mode,
                human_approval_asserted: true,
                // Replaced before validation; it is never owner authority.
                approved_content_fingerprint: FingerprintV1::parse(&format!(
                    "sha256:{}",
                    "0".repeat(64)
                ))
                .expect("fixed valid fingerprint"),
                text: fixed.instruction.text,
                asserted_principal: None,
                asserted_agent: None,
                asserted_delegation: None,
            },
            request_key: fixed.request_key,
        };
        // Validate fixed vocabulary and numeric bounds before canonicalizing the
        // approval preimage, preserving the existing safe-number contract.
        validated_fixed_content(&input)?;
        validate_transition_project(&input.project)?;
        input.instruction.approved_content_fingerprint =
            approved_deferred_establishment_content_fingerprint(&input)?;
        Ok(validate_deferred_establishment_request(input)?.input)
    }

    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, GovernanceRefusalV1> {
        Ok(validate_deferred_establishment_request(decode_governance_json(bytes)?)?.input)
    }

    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, GovernanceRefusalV1> {
        validate_deferred_establishment_request(self.clone())?;
        governance_canonical_bytes(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeferredEstablishmentLocatorV3 {
    pub request: DeferredEstablishmentRequestV3,
    pub request_fingerprint: FingerprintV1,
}

impl DeferredEstablishmentLocatorV3 {
    /// Restoration grants exact request resolution only. No initialization
    /// claim, identity allocation or executable prepared operation is restored.
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, GovernanceRefusalV1> {
        let locator: Self = decode_governance_json(bytes)?;
        validate_deferred_establishment_locator(&locator)?;
        Ok(locator)
    }

    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, GovernanceRefusalV1> {
        validate_deferred_establishment_locator(self)?;
        governance_canonical_bytes(self)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatedDeferredEstablishmentV3 {
    pub input: DeferredEstablishmentRequestV3,
    pub configuration_fingerprint: FingerprintV1,
    pub instruction_fingerprint: FingerprintV1,
    pub establishment_rule: GovernanceRuleDescriptorV2,
    pub owner_rule: GovernanceRuleDescriptorV1,
}

pub fn validate_deferred_establishment_request(
    input: DeferredEstablishmentRequestV3,
) -> Result<ValidatedDeferredEstablishmentV3, GovernanceRefusalV1> {
    validate_transition_project(&input.project)?;
    let fixed = validated_fixed_content(&input)
        .map_err(|refusal| refusal.with_project(input.project.refusal_project()))?;
    if !input.instruction.human_approval_asserted {
        return Err(GovernanceRefusalV1::new(
            "human_approved_establishment_instruction",
            "New explicit human approval of the exact configuration and terms is required",
        )
        .with_project(input.project.refusal_project()));
    }
    if approved_deferred_establishment_content_fingerprint(&input)?
        != input.instruction.approved_content_fingerprint
    {
        return Err(GovernanceRefusalV1::new(
            "human_approval_identity",
            "The human-approved content must bind this exact project, scope, deferred basis, configuration and terms",
        )
        .with_project(input.project.refusal_project()));
    }
    governance_canonical_bytes(&input)?;
    Ok(ValidatedDeferredEstablishmentV3 {
        configuration_fingerprint: fixed.configuration_fingerprint,
        instruction_fingerprint: governance_fingerprint(
            "handbook.project-admission-governance.local-advisory-instruction@3.0.0",
            &input.instruction,
        )?,
        establishment_rule: establishment_rule_descriptor_v3()?,
        owner_rule: fixed.owner_rule,
        input,
    })
}

pub fn create_deferred_establishment_locator(
    input: DeferredEstablishmentRequestV3,
) -> Result<DeferredEstablishmentLocatorV3, GovernanceRefusalV1> {
    let validated = validate_deferred_establishment_request(input)?;
    let locator = DeferredEstablishmentLocatorV3 {
        request_fingerprint: deferred_establishment_request_fingerprint(&validated.input)?,
        request: validated.input,
    };
    governance_canonical_bytes(&locator)?;
    Ok(locator)
}

pub fn validate_deferred_establishment_locator(
    locator: &DeferredEstablishmentLocatorV3,
) -> Result<ValidatedDeferredEstablishmentV3, GovernanceRefusalV1> {
    let validated = validate_deferred_establishment_request(locator.request.clone())?;
    if deferred_establishment_request_fingerprint(&locator.request)? != locator.request_fingerprint
    {
        return Err(GovernanceRefusalV1::new(
            "request_identity",
            "The original complete transition request must match its exact request fingerprint",
        )
        .with_project(locator.request.project.refusal_project()));
    }
    governance_canonical_bytes(locator)?;
    Ok(validated)
}

pub(crate) fn deferred_establishment_request_fingerprint(
    input: &DeferredEstablishmentRequestV3,
) -> Result<FingerprintV1, GovernanceRefusalV1> {
    #[derive(Serialize)]
    struct Request<'a> {
        request: &'a DeferredEstablishmentRequestV3,
        establishment_rule: ExactBindingV1,
        owner_rule: ExactBindingV1,
    }
    governance_fingerprint(
        "handbook.project-admission-governance.request@3.0.0",
        &Request {
            request: input,
            establishment_rule: establishment_rule_descriptor_v3()?.binding,
            owner_rule: freshness_admission_rule_descriptor_v1()?.binding,
        },
    )
}

pub(crate) fn approved_deferred_establishment_content_fingerprint(
    input: &DeferredEstablishmentRequestV3,
) -> Result<FingerprintV1, GovernanceRefusalV1> {
    #[derive(Serialize)]
    struct ApprovedContent<'a> {
        project: &'a ProjectBindingV2,
        scope: &'a str,
        expected_deferred: &'a ExpectedDeferredBasisV2,
        configuration: &'a AdmissionConfigurationInputV1,
        replacement_basis: &'a ReplacementAuthorityBasisV1,
        establishment_rule: ExactBindingV1,
        owner_rule: ExactBindingV1,
    }
    governance_fingerprint(
        "handbook.project-admission-governance.approved-content@3.0.0",
        &ApprovedContent {
            project: &input.project,
            scope: &input.scope,
            expected_deferred: &input.expected_deferred,
            configuration: &input.configuration,
            replacement_basis: &input.replacement_basis,
            establishment_rule: establishment_rule_descriptor_v3()?.binding,
            owner_rule: freshness_admission_rule_descriptor_v1()?.binding,
        },
    )
}

fn validated_fixed_content(
    input: &DeferredEstablishmentRequestV3,
) -> Result<super::ValidatedGovernanceV1, GovernanceRefusalV1> {
    // Reuse fixed configuration, terms and bounded claims validation only. This
    // temporary value is never registered, prepared, executed or persisted as v1.
    validate_request(GovernanceRequestInputV1 {
        scope: input.scope.clone(),
        configuration: input.configuration.clone(),
        replacement_basis: input.replacement_basis.clone(),
        instruction: LocalAdvisoryInstructionV1 {
            mode: input.instruction.mode,
            text: input.instruction.text.clone(),
            asserted_principal: input.instruction.asserted_principal.clone(),
            asserted_agent: input.instruction.asserted_agent.clone(),
            asserted_delegation: input.instruction.asserted_delegation.clone(),
        },
        expected_initial_state: ExpectedInitialStateV1::FreshRegistrationUnused,
        request_key: input.request_key.clone(),
    })
}

fn validate_transition_project(project: &ProjectBindingV2) -> Result<(), GovernanceRefusalV1> {
    if FingerprintV1::parse(&project.repository_identity_fingerprint).is_err()
        || FingerprintV1::parse(&project.physical_root_fingerprint).is_err()
        || project.retained_root.len() > 4096
        || project.retained_root.contains('\0')
        || !Path::new(&project.retained_root).is_absolute()
        || project.registration_identity
            != deferral_registration_fingerprint(
                &project.repository_identity_fingerprint,
                &project.retained_root,
                &project.physical_root_fingerprint,
            )?
    {
        return Err(GovernanceRefusalV1::new(
            "project_binding",
            "Exact stable v2 project registration and absolute retained root are required",
        ));
    }
    Ok(())
}

pub fn establishment_rule_descriptor_v3() -> Result<GovernanceRuleDescriptorV2, GovernanceRefusalV1>
{
    let original_rule = establishment_rule_descriptor_v2()?.binding;
    let mut constraints: Vec<String> = [
        "positive_exact_current_deferred_never_established_lifecycle",
        "same_physical_project_identity_registration_and_scope",
        "preserve_original_deferred_request_claim_receipt_and_consumed_initial_choice",
        "new_exact_human_approved_configuration_and_immutable_terms",
        "explicit_standalone_local_advisory_instruction",
        "immutable_configuration_content_and_exact_supported_owner_rule",
        "preserve_fixed_owner_conformance_floors",
        "separate_validation_accepted_permission_and_commit",
        "single_durable_decision_exact_predecessor_and_confirmed_cutover",
        "complete_linked_request_history_and_verified_predecision_noncommit",
        "decided_recovery_finishes_only_original_governance",
        "historical_replay_never_restores_or_repairs_current_relation",
        "unsupported_inventory_and_uncertain_history_refuse_fresh_attempts",
        "no_circular_candidate_authority",
        "no_scope_to_qualification_evaluation_or_action_authority",
    ]
    .iter()
    .map(|value| (*value).into())
    .collect();
    constraints.push(format!(
        "original_establishment_rule:{}:{}",
        original_rule.reference(),
        original_rule.fingerprint().as_str()
    ));
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
        original_establishment_rule: &'a ExactBindingV1,
    }
    let content = Content {
        owner: "handbook",
        name: ESTABLISHMENT_RULE_NAME_V1,
        version: GOVERNANCE_RULE_VERSION_V3,
        candidate_kind: None,
        candidate_schema_ref: None,
        required_authored_sources: vec![],
        approval_sources: vec![],
        exception_sources: vec![],
        constraints: &constraints,
        original_establishment_rule: &original_rule,
    };
    let binding = ExactBindingV1::new(
        ExactRefV1::parse("handbook.admission-establishment.standalone@3.0.0")
            .map_err(|_| GovernanceRefusalV1::new("rule", "Invalid shipped rule reference"))?,
        governance_fingerprint(
            "handbook.project-admission-governance.shipped-rule@3.0.0",
            &content,
        )?,
    );
    Ok(GovernanceRuleDescriptorV2 {
        owner: "handbook".into(),
        name: ESTABLISHMENT_RULE_NAME_V1.into(),
        version: GOVERNANCE_RULE_VERSION_V3.into(),
        candidate_kind: None,
        candidate_schema_ref: None,
        required_authored_sources: vec![],
        approval_sources: vec![],
        exception_sources: vec![],
        constraints,
        binding,
    })
}
