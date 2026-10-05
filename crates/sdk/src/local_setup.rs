//! Fresh defer-first local setup composed from separately attributable native owners.
//!
//! A portable locator records exact requests for recovery and history. It is not
//! permission to create state, reactivate a revoked head, or use an envelope.

use crate::admission_governance::{
    DeferralExecutionV2, DeferralRequestInputV2, DeferralRequestLocatorV2, GovernanceRefusalV1,
    PreparedAdmissionDeferralV2, ProjectAdmissionBindingV3, ProjectAdmissionScopeBasisV3,
    ADMISSION_GOVERNANCE_SCOPE_V1,
};
use crate::contract_membrane::{canonical_json_bytes, ContractLimits};
use crate::HandbookSdkV1;
pub use handbook_engine::local_authority::{
    LocalAuthorityDispositionV1, LocalAuthorityExecutionV1, LocalAuthorityLocatorV1,
    LocalAuthorityOutcomeV1, LocalBindingV1, LocalDecisionClassV1, LocalEnvelopeCeilingV1,
    LocalOperatingTermsV1, LocalOperationClassV1, LocalRevokeRequestV1, LocalUnresolvedRequestV1,
};
use handbook_engine::local_authority::{NativeLocalAuthorityServiceV1, PreparedLocalAuthorityV1};
pub use handbook_engine::local_charter::{
    LocalAuthorityRefusalV1, LocalCharterDispositionV1, LocalCharterExecutionV1,
    LocalCharterHumanApprovalV1, LocalCharterLocatorV1, LocalCharterOutcomeV1,
    LocalProjectBindingV1, NativeLocalCharterRequestV1,
};
use handbook_engine::local_charter::{NativeLocalCharterServiceV1, PreparedLocalCharterV1};
use handbook_engine::{
    ContextResolutionDimensions, ContextResolutionEnvelope, ContextResolutionEnvelopeInput,
    ContextResolutionExactBinding, ContextResolutionMutationEffect, ContextResolutionMutationRule,
};
use serde::{Deserialize, Serialize};

const LOCATOR_SCHEMA: &str = "handbook.sdk.local-initial-setup-locator";
const LOCATOR_VERSION: &str = "1.0.0";

/// Supplied initial choices. Implementation approval supplies none of these values.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalInitialSetupRequestV1 {
    pub deferral: DeferralRequestInputV2,
    pub charter: NativeLocalCharterRequestV1,
    pub terms: LocalOperatingTermsV1,
}

impl LocalInitialSetupRequestV1 {
    /// Closed, duplicate-safe transport decoding; native preparation still
    /// validates every owner's exact inputs and current eligibility.
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalSetupRefusalV1> {
        decode(bytes)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalInitialSetupLocatorV1 {
    pub schema_id: String,
    pub schema_version: String,
    pub deferral: DeferralRequestLocatorV2,
    pub charter: LocalCharterLocatorV1,
    pub terms: LocalAuthorityLocatorV1,
}

impl LocalInitialSetupLocatorV1 {
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalSetupRefusalV1> {
        let locator: Self = decode(bytes)?;
        locator.validate()?;
        Ok(locator)
    }

    /// Canonical SDK transport bytes, separate from each native identity preimage.
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalSetupRefusalV1> {
        self.validate()?;
        canonical_json_bytes(self, &ContractLimits::packet0())
            .map_err(|error| transport_error(error.to_string()))
    }

    fn validate(&self) -> Result<(), LocalSetupRefusalV1> {
        if self.schema_id != LOCATOR_SCHEMA || self.schema_version != LOCATOR_VERSION {
            return Err(transport_error("unsupported setup locator schema"));
        }
        self.deferral
            .canonical_json_bytes()
            .map_err(|error| governance_error(LocalSetupStageV1::Preparation, error))?;
        self.charter
            .canonical_json_bytes()
            .map_err(|error| engine_error(LocalSetupStageV1::Preparation, error))?;
        self.terms
            .canonical_json_bytes()
            .map_err(|error| engine_error(LocalSetupStageV1::Preparation, error))?;
        if self.deferral.initialization_claim != self.charter.binding.initialization_claim
            || self.terms.charter != self.charter
            || self.terms.expected_governance_relation_fingerprint
                != prospective_governance_relation_fingerprint(&self.deferral)?
        {
            return Err(transport_error("setup owner project bindings conflict"));
        }
        Ok(())
    }
}

/// Sealed native proposals, consumed once. Save `locator()` before establishment.
#[derive(Debug)]
pub struct PreparedLocalInitialSetupV1 {
    locator: LocalInitialSetupLocatorV1,
    deferral: PreparedAdmissionDeferralV2,
    charter: PreparedLocalCharterV1,
    terms: PreparedLocalAuthorityV1,
}

impl PreparedLocalInitialSetupV1 {
    pub fn locator(&self) -> &LocalInitialSetupLocatorV1 {
        &self.locator
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalSetupStageV1 {
    Preparation,
    Deferral,
    CurrentApplicability,
    Charter,
    Terms,
    Admission,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalSetupRefusalV1 {
    pub stage: LocalSetupStageV1,
    pub code: String,
    pub detail: String,
    pub observed_governance_basis: Option<ProjectAdmissionScopeBasisV3>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unresolved_request: Option<Box<LocalUnresolvedRequestV1>>,
}

/// `None` means an owner was never attempted, not that its operation succeeded.
#[derive(Debug, Serialize)]
pub struct LocalInitialSetupExecutionV1 {
    pub locator: LocalInitialSetupLocatorV1,
    pub deferral: DeferralExecutionV2,
    pub governance: Option<ProjectAdmissionScopeBasisV3>,
    pub charter: Option<LocalCharterExecutionV1>,
    pub terms: Option<LocalAuthorityExecutionV1>,
    pub stopped: Option<LocalSetupRefusalV1>,
}

#[derive(Debug, Serialize)]
pub struct LocalInitialSetupCurrentV1 {
    pub governance: ProjectAdmissionScopeBasisV3,
    pub charter: LocalCharterOutcomeV1,
    pub terms: LocalAuthorityOutcomeV1,
}

/// Supplied actual envelope candidate. SDK construction grants no authority.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEnvelopeRequestV1 {
    pub envelope_id: String,
    pub objective_ref: String,
    pub resolved_profile: LocalBindingV1,
    pub resolution_stack: LocalBindingV1,
    pub active_level_id: String,
    pub dimensions: [String; 6],
    pub parent: Option<LocalBindingV1>,
    pub constraint_inputs: Vec<LocalBindingV1>,
    pub mutation_rules: Vec<LocalMutationRuleInputV1>,
    pub escalation_triggers: Vec<LocalBindingV1>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalMutationEffectV1 {
    Allow,
    Deny,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalMutationRuleInputV1 {
    pub rule_id: String,
    pub effect: LocalMutationEffectV1,
    pub target_kind: String,
    pub selector: String,
}

impl LocalEnvelopeRequestV1 {
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalSetupRefusalV1> {
        let request: Self = decode(bytes)?;
        request.native_input()?;
        Ok(request)
    }

    pub fn native_input(&self) -> Result<ContextResolutionEnvelopeInput, LocalSetupRefusalV1> {
        let bindings = |values: &[LocalBindingV1]| {
            values
                .iter()
                .map(native_binding)
                .collect::<Result<Vec<_>, _>>()
        };
        let rules = self
            .mutation_rules
            .iter()
            .map(|rule| {
                ContextResolutionMutationRule::new(
                    &rule.rule_id,
                    match rule.effect {
                        LocalMutationEffectV1::Allow => ContextResolutionMutationEffect::Allow,
                        LocalMutationEffectV1::Deny => ContextResolutionMutationEffect::Deny,
                    },
                    &rule.target_kind,
                    &rule.selector,
                )
                .map_err(kernel_error)
            })
            .collect::<Result<Vec<_>, _>>()?;
        ContextResolutionEnvelopeInput::new(
            &self.envelope_id,
            &self.objective_ref,
            native_binding(&self.resolved_profile)?,
            native_binding(&self.resolution_stack)?,
            &self.active_level_id,
            ContextResolutionDimensions::new(self.dimensions.each_ref().map(String::as_str))
                .map_err(kernel_error)?,
            self.parent.as_ref().map(native_binding).transpose()?,
            bindings(&self.constraint_inputs)?,
            rules,
            bindings(&self.escalation_triggers)?,
        )
        .map_err(kernel_error)
    }
}

impl HandbookSdkV1 {
    /// Prepares all owner locators without effects. There is no atomic setup owner.
    pub fn prepare_local_initial_setup(
        &self,
        request: &LocalInitialSetupRequestV1,
    ) -> Result<PreparedLocalInitialSetupV1, LocalSetupRefusalV1> {
        for (claim, charter_claim, terms_claim) in [
            (
                &request.deferral.instruction.asserted_principal,
                &request.charter.human_approval.principal_claim,
                &request.terms.principal,
            ),
            (
                &request.deferral.instruction.asserted_agent,
                &request.charter.human_approval.agent_claim,
                &request.terms.agent,
            ),
        ] {
            if claim
                .as_ref()
                .is_some_and(|claim| claim != charter_claim || claim != terms_claim)
            {
                return Err(LocalSetupRefusalV1 {
                    stage: LocalSetupStageV1::Preparation,
                    code: "project_claim_conflict".into(),
                    detail: "explicit deferral actor claims conflict with supplied Charter and operating terms".into(),
                    observed_governance_basis: None,
                    unresolved_request: None,
                });
            }
        }
        let deferral = self
            .prepare_project_admission_deferral(&request.deferral)
            .map_err(|error| governance_error(LocalSetupStageV1::Preparation, error))?;
        let binding = LocalProjectBindingV1 {
            initialization_claim: deferral.locator().initialization_claim.clone(),
        };
        let charter = NativeLocalCharterServiceV1::prepare(
            &self.repo_root,
            &binding,
            request.charter.clone(),
        )
        .map_err(|error| engine_error(LocalSetupStageV1::Preparation, error))?;
        let terms = NativeLocalAuthorityServiceV1::prepare(
            &self.repo_root,
            charter.locator(),
            request.terms.clone(),
            prospective_governance_relation_fingerprint(deferral.locator())?,
        )
        .map_err(|error| engine_error(LocalSetupStageV1::Preparation, error))?;
        let locator = LocalInitialSetupLocatorV1 {
            schema_id: LOCATOR_SCHEMA.into(),
            schema_version: LOCATOR_VERSION.into(),
            deferral: deferral.locator().clone(),
            charter: charter.locator().clone(),
            terms: terms.locator().clone(),
        };
        locator.canonical_json_bytes()?;
        Ok(PreparedLocalInitialSetupV1 {
            locator,
            deferral,
            charter,
            terms,
        })
    }

    /// Executes native owners sequentially and preserves exact partial outcomes.
    pub fn establish_local_initial_setup(
        &self,
        prepared: PreparedLocalInitialSetupV1,
    ) -> LocalInitialSetupExecutionV1 {
        let PreparedLocalInitialSetupV1 {
            locator,
            deferral,
            charter,
            terms,
        } = prepared;
        let mut execution = new_execution(
            locator,
            self.initialize_project_admission_deferral(deferral),
        );
        if !matches!(execution.deferral, DeferralExecutionV2::Committed { .. }) {
            return execution;
        }
        match self.resolve_local_applicability(&execution.locator) {
            Ok(basis) => execution.governance = Some(basis),
            Err(error) => {
                execution.stopped = Some(error);
                return execution;
            }
        }
        let charter = NativeLocalCharterServiceV1::establish(&self.repo_root, charter);
        let proceed = charter_committed(&charter);
        execution.charter = Some(charter);
        if proceed {
            execution.terms = Some(NativeLocalAuthorityServiceV1::establish(
                &self.repo_root,
                terms,
            ));
        }
        execution
    }

    /// Resolves original owner history; this does not create absent owner intent.
    pub fn resolve_local_initial_setup_request(
        &self,
        locator: &LocalInitialSetupLocatorV1,
    ) -> Result<LocalInitialSetupExecutionV1, LocalSetupRefusalV1> {
        self.recover_local_initial_setup(locator, false)
    }

    /// Completes retained original owner intent only. An unreached stage remains
    /// unresolved and requires explicit native preparation with current eligibility.
    pub fn resume_local_initial_setup_request(
        &self,
        locator: &LocalInitialSetupLocatorV1,
    ) -> Result<LocalInitialSetupExecutionV1, LocalSetupRefusalV1> {
        self.recover_local_initial_setup(locator, true)
    }

    pub fn resolve_current_local_initial_setup(
        &self,
        locator: &LocalInitialSetupLocatorV1,
    ) -> Result<LocalInitialSetupCurrentV1, LocalSetupRefusalV1> {
        locator.validate()?;
        let governance = self.resolve_local_applicability(locator)?;
        let charter =
            NativeLocalCharterServiceV1::resolve_current(&self.repo_root, &locator.charter)
                .map_err(|error| engine_error(LocalSetupStageV1::Charter, error))?;
        let terms = NativeLocalAuthorityServiceV1::resolve_current(&self.repo_root, &locator.terms)
            .map_err(|error| engine_error(LocalSetupStageV1::Terms, error))?;
        Ok(LocalInitialSetupCurrentV1 {
            governance,
            charter,
            terms,
        })
    }

    /// Rechecks v3 applicability, then asks Engine to admit the actual candidate.
    /// The returned sealed envelope performs native currentness on consuming checks.
    pub fn admit_local_envelope(
        &self,
        locator: &LocalInitialSetupLocatorV1,
        input: ContextResolutionEnvelopeInput,
    ) -> Result<ContextResolutionEnvelope, LocalSetupRefusalV1> {
        locator.validate()?;
        self.resolve_local_applicability(locator)
            .map_err(|mut error| {
                error.unresolved_request = Some(Box::new(
                    NativeLocalAuthorityServiceV1::missing_authority_context(
                        &locator.terms,
                        &input,
                        &error.detail,
                    ),
                ));
                error
            })?;
        NativeLocalAuthorityServiceV1::admit_envelope(&self.repo_root, &locator.terms, input)
            .map_err(|error| engine_error(LocalSetupStageV1::Admission, error))
    }

    pub fn admit_local_envelope_request(
        &self,
        locator: &LocalInitialSetupLocatorV1,
        request: &LocalEnvelopeRequestV1,
    ) -> Result<ContextResolutionEnvelope, LocalSetupRefusalV1> {
        self.admit_local_envelope(locator, request.native_input()?)
    }

    pub fn revoke_local_operating_authority(
        &self,
        locator: &LocalInitialSetupLocatorV1,
        request: LocalRevokeRequestV1,
    ) -> Result<LocalAuthorityExecutionV1, LocalSetupRefusalV1> {
        locator.validate()?;
        self.resolve_local_applicability(locator)?;
        Ok(NativeLocalAuthorityServiceV1::revoke(
            &self.repo_root,
            &locator.terms,
            request,
        ))
    }

    fn recover_local_initial_setup(
        &self,
        locator: &LocalInitialSetupLocatorV1,
        resume: bool,
    ) -> Result<LocalInitialSetupExecutionV1, LocalSetupRefusalV1> {
        locator.validate()?;
        let mut execution = new_execution(
            locator.clone(),
            self.resolve_admission_deferral_request(&locator.deferral),
        );
        if !matches!(execution.deferral, DeferralExecutionV2::Committed { .. }) {
            return Ok(execution);
        }
        // Historical delivery remains available after applicability invalidation;
        // resumption may publish effects, so resolve v3 before choosing that path.
        let can_resume = match self.resolve_local_applicability(locator) {
            Ok(basis) => {
                execution.governance = Some(basis);
                resume
            }
            Err(error) => {
                execution.stopped = Some(error);
                false
            }
        };
        let charter = if can_resume {
            NativeLocalCharterServiceV1::resume_request(&self.repo_root, &locator.charter)
        } else {
            NativeLocalCharterServiceV1::resolve_request(&self.repo_root, &locator.charter)
        };
        let proceed = charter_committed(&charter);
        execution.charter = Some(charter);
        if proceed {
            execution.terms = Some(if can_resume {
                NativeLocalAuthorityServiceV1::resume_request(&self.repo_root, &locator.terms)
            } else {
                NativeLocalAuthorityServiceV1::resolve_request(&self.repo_root, &locator.terms)
            });
        }
        Ok(execution)
    }

    fn resolve_local_applicability(
        &self,
        locator: &LocalInitialSetupLocatorV1,
    ) -> Result<ProjectAdmissionScopeBasisV3, LocalSetupRefusalV1> {
        let basis = self
            .resolve_project_admission_scope_v3(
                &ProjectAdmissionBindingV3::ScopedChoiceV2(locator.deferral.project.clone()),
                ADMISSION_GOVERNANCE_SCOPE_V1,
                None,
            )
            .map_err(|error| governance_error(LocalSetupStageV1::CurrentApplicability, error))?;
        match basis {
            ProjectAdmissionScopeBasisV3::DeferredV2(_) => Ok(basis),
            _ => Err(LocalSetupRefusalV1 {
                stage: LocalSetupStageV1::CurrentApplicability,
                code: "unsupported_governance_basis".into(),
                detail: "Packet 1 supports exact current deferral; established dependent use requires its own owner path".into(),
                observed_governance_basis: Some(basis),
                unresolved_request: None,
            }),
        }
    }
}

fn new_execution(
    locator: LocalInitialSetupLocatorV1,
    deferral: DeferralExecutionV2,
) -> LocalInitialSetupExecutionV1 {
    LocalInitialSetupExecutionV1 {
        locator,
        deferral,
        governance: None,
        charter: None,
        terms: None,
        stopped: None,
    }
}

fn charter_committed(execution: &LocalCharterExecutionV1) -> bool {
    execution.outcome.is_some()
        && matches!(
            execution.disposition,
            LocalCharterDispositionV1::Committed | LocalCharterDispositionV1::Replayed
        )
}

fn decode<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, LocalSetupRefusalV1> {
    let value = handbook_engine::parse_schema_json(bytes)
        .map_err(|error| transport_error(error.to_string()))?;
    serde_json::from_value(value).map_err(|error| transport_error(error.to_string()))
}

fn transport_error(detail: impl Into<String>) -> LocalSetupRefusalV1 {
    LocalSetupRefusalV1 {
        stage: LocalSetupStageV1::Preparation,
        code: "setup_transport".into(),
        detail: detail.into(),
        observed_governance_basis: None,
        unresolved_request: None,
    }
}

fn engine_error(stage: LocalSetupStageV1, error: LocalAuthorityRefusalV1) -> LocalSetupRefusalV1 {
    LocalSetupRefusalV1 {
        stage,
        code: error.code,
        detail: error.detail,
        observed_governance_basis: None,
        unresolved_request: error.unresolved_request,
    }
}

fn governance_error(stage: LocalSetupStageV1, error: GovernanceRefusalV1) -> LocalSetupRefusalV1 {
    LocalSetupRefusalV1 {
        stage,
        code: error.condition,
        detail: error.detail,
        observed_governance_basis: None,
        unresolved_request: None,
    }
}

fn prospective_governance_relation_fingerprint(
    locator: &DeferralRequestLocatorV2,
) -> Result<String, LocalSetupRefusalV1> {
    let bytes = locator
        .prospective_deferred_basis()
        .and_then(|basis| basis.canonical_json_bytes())
        .map_err(|error| governance_error(LocalSetupStageV1::Preparation, error))?;
    Ok(handbook_engine::DefinitionFingerprint::from_bytes(&bytes)
        .as_str()
        .to_owned())
}

fn native_binding(
    binding: &LocalBindingV1,
) -> Result<ContextResolutionExactBinding, LocalSetupRefusalV1> {
    ContextResolutionExactBinding::new(&binding.reference, &binding.fingerprint)
        .map_err(kernel_error)
}

fn kernel_error(error: handbook_engine::ContextResolutionKernelError) -> LocalSetupRefusalV1 {
    LocalSetupRefusalV1 {
        stage: LocalSetupStageV1::Admission,
        code: "envelope_candidate".into(),
        detail: error.detail().to_owned(),
        observed_governance_basis: None,
        unresolved_request: None,
    }
}
