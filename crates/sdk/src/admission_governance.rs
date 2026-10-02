//! Exact native project admission governance for one newly registered project.
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

use crate::HandbookSdkV1;

impl HandbookSdkV1 {
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
