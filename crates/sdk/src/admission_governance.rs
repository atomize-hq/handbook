//! Exact native project admission governance for supported registered projects.
//!
//! These typed methods preserve the Contracts owner's outcomes and identities.
//! Establishment confirms validation, advisory permission and activation separately;
//! it does not imply policy admission, evaluation or generic repository setup.
//! After a lost reply, retain and resolve the original request locator.

use handbook_contracts::project_admission_governance::ProjectAdmissionGovernanceV1;
pub use handbook_contracts::project_admission_governance::{
    parse_governance_request_json, ActiveGovernanceBasisV1, AdmissionConfigurationInputV1,
    AuthorityModeV1, ExpectedGovernanceBasisV1, ExpectedInitialStateV1,
    FreshnessRuleBindingInputV1, GovernanceCommittedOutcomeV1, GovernanceDeliveryV1,
    GovernanceExecutionV1, GovernanceNotCommittedOutcomeV1, GovernanceRefusalV1,
    GovernanceRequestInputV1, GovernanceRequestLocatorV1, GovernanceRuleDescriptorV1,
    LocalAdvisoryInstructionV1, PreparedAdmissionGovernanceV1, ProjectBindingV1,
    ReplacementAuthorityBasisV1, ValidatedGovernanceV1, ADMISSION_GOVERNANCE_SCOPE_V1,
    JSON_SAFE_INTEGER_MAX, MAX_GOVERNANCE_RECORD_BYTES_V1,
};
pub use handbook_contracts::{ExactBindingV1, ExactRefV1, FingerprintV1};

pub use handbook_contracts::project_admission_governance::{
    DeferralCommittedOutcomeV2, DeferralExecutionV2, DeferralNotCommittedOutcomeV2,
    DeferralRequestInputV2, DeferralRequestLocatorV2, DeferredGovernanceBasisV2,
    ExpectedDeferredBasisV2, ExpectedInitialStateV2, ExpectedProjectAdmissionBasisV2,
    GovernanceRuleDescriptorV2, HumanApprovedInitialInstructionV2, InitialGovernanceChoiceV2,
    PreparedAdmissionDeferralV2, ProjectAdmissionBindingV2, ProjectAdmissionScopeBasisV2,
    ProjectBindingV2, ValidatedDeferralV2,
};

pub use handbook_contracts::project_admission_governance::{
    DeferredEstablishmentCommittedOutcomeV3, DeferredEstablishmentExecutionV3,
    DeferredEstablishmentLocatorV3, DeferredEstablishmentNotCommittedOutcomeV3,
    DeferredEstablishmentRequestV3, DeferredScopeBasisV3, EstablishedFromDeferredBasisV3,
    ExpectedEstablishedFromDeferredBasisV3, ExpectedProjectAdmissionBasisV3,
    HumanApprovedEstablishmentInstructionV3, PreparedDeferredGovernanceV3,
    ProjectAdmissionBindingV3, ProjectAdmissionScopeBasisV3, ValidatedDeferredEstablishmentV3,
};

use crate::HandbookSdkV1;

impl HandbookSdkV1 {
    /// Prepares exact newly approved governance against this project's current
    /// deferred basis without effects. Serialize the complete locator before
    /// establishment; restoring it permits request resolution only.
    pub fn prepare_deferred_project_admission_governance(
        &self,
        request: &DeferredEstablishmentRequestV3,
    ) -> Result<PreparedDeferredGovernanceV3, GovernanceRefusalV1> {
        handbook_contracts::project_admission_governance::ProjectAdmissionEstablishmentV3::new()
            .prepare(&self.repo_root, request)
    }

    /// Establishes one exact configuration on the same deferred project.
    /// Validation, accepted local advisory permission and committed cutover are
    /// separate owner facts. Refusal, noncommit and uncertainty remain distinct.
    pub fn establish_deferred_project_admission_governance(
        &self,
        prepared: PreparedDeferredGovernanceV3,
    ) -> DeferredEstablishmentExecutionV3 {
        handbook_contracts::project_admission_governance::ProjectAdmissionEstablishmentV3::new()
            .establish(&self.repo_root, prepared)
    }

    /// Resolves the complete original establishment locator after a lost reply
    /// or restart. Recovery finishes only its retained decision; historical replay
    /// does not restore or repair current governance.
    pub fn resolve_deferred_admission_governance_request(
        &self,
        locator: &DeferredEstablishmentLocatorV3,
    ) -> DeferredEstablishmentExecutionV3 {
        handbook_contracts::project_admission_governance::ProjectAdmissionEstablishmentV3::new()
            .resolve_request(&self.repo_root, locator)
    }

    /// Resolves exact current governance with explicit project and basis tags.
    /// Established v1, deferred v2 and established-from-deferred v3 retain distinct
    /// bindings. This scope grants no qualification, evaluation or action authority.
    pub fn resolve_project_admission_scope_v3(
        &self,
        project: &ProjectAdmissionBindingV3,
        scope: &str,
        expected: Option<&ExpectedProjectAdmissionBasisV3>,
    ) -> Result<ProjectAdmissionScopeBasisV3, GovernanceRefusalV1> {
        handbook_contracts::project_admission_governance::resolve_project_admission_scope_v3(
            &self.repo_root,
            project,
            scope,
            expected,
        )
    }

    /// Prepares one exact human-approved advisory deferral without effects.
    /// Serialize the locator before initialization; restoring it permits resolution only.
    pub fn prepare_project_admission_deferral(
        &self,
        request: &DeferralRequestInputV2,
    ) -> Result<PreparedAdmissionDeferralV2, GovernanceRefusalV1> {
        handbook_contracts::project_admission_governance::ProjectAdmissionDeferralV2::new()
            .prepare(&self.repo_root, request)
    }

    /// Consumes one sealed proposal. Deferral establishes no configuration or permission to evaluate.
    pub fn initialize_project_admission_deferral(
        &self,
        prepared: PreparedAdmissionDeferralV2,
    ) -> DeferralExecutionV2 {
        handbook_contracts::project_admission_governance::ProjectAdmissionDeferralV2::new()
            .initialize(&self.repo_root, prepared)
    }

    /// Resolves exactly the selected project contract and scope. This says nothing
    /// about another required scope, qualification, operating delegation or action authority.
    pub fn resolve_project_admission_scope(
        &self,
        project: &ProjectAdmissionBindingV2,
        scope: &str,
        expected: Option<&ExpectedProjectAdmissionBasisV2>,
    ) -> Result<ProjectAdmissionScopeBasisV2, GovernanceRefusalV1> {
        handbook_contracts::project_admission_governance::resolve_project_admission_scope(
            &self.repo_root,
            project,
            scope,
            expected,
        )
    }

    /// Resolves the original request; terminal replay never restores an old current relation.
    pub fn resolve_admission_deferral_request(
        &self,
        locator: &DeferralRequestLocatorV2,
    ) -> DeferralExecutionV2 {
        handbook_contracts::project_admission_governance::ProjectAdmissionDeferralV2::new()
            .resolve_request(&self.repo_root, locator)
    }

    /// Validates an explicit new-project instruction and prepares its exact identity.
    ///
    /// This method does not mutate the repository or grant initial eligibility.
    /// Retain [`PreparedAdmissionGovernanceV1::locator`] before sending the sealed,
    /// single-use prepared value to initialization. The caller supplies the ceiling;
    /// no freshness ceiling is defaulted.
    pub fn prepare_project_admission_governance(
        &self,
        request: &GovernanceRequestInputV1,
    ) -> Result<PreparedAdmissionGovernanceV1, GovernanceRefusalV1> {
        ProjectAdmissionGovernanceV1::new().prepare(&self.repo_root, request)
    }

    /// Consumes a prepared exact identity for one native registration and activation.
    ///
    /// Committed, refused, positively not committed and uncertain owner outcomes
    /// remain distinct. Resolve uncertainty with the original request locator;
    /// initialization never retries or replaces the proposed identity.
    pub fn initialize_project_admission_governance(
        &self,
        prepared: PreparedAdmissionGovernanceV1,
    ) -> GovernanceExecutionV1 {
        ProjectAdmissionGovernanceV1::new().initialize(&self.repo_root, prepared)
    }

    /// Resolves the owner-verified active basis for the exact project and scope.
    ///
    /// Optional expected bindings demand that exact configuration and outcome.
    /// Missing or conflicting records refuse; resolution never creates authority.
    pub fn resolve_project_admission_governance(
        &self,
        project: &ProjectBindingV1,
        scope: &str,
        expected: Option<&ExpectedGovernanceBasisV1>,
    ) -> Result<ActiveGovernanceBasisV1, GovernanceRefusalV1> {
        ProjectAdmissionGovernanceV1::new().resolve_active(
            &self.repo_root,
            project,
            scope,
            expected,
        )
    }

    /// Resolves the original request after restart or a lost reply.
    ///
    /// Historical replay preserves the original outcome identity and reports its
    /// delivery disposition. Owner recovery may finish an already-decided commit;
    /// it cannot resubmit activation or select current active governance implicitly.
    pub fn resolve_admission_governance_request(
        &self,
        locator: &GovernanceRequestLocatorV1,
    ) -> GovernanceExecutionV1 {
        ProjectAdmissionGovernanceV1::new().resolve_request(&self.repo_root, locator)
    }
}
