//! Typed SDK delegation to the Contracts governance owner.
use crate::HandbookSdkV1;
use handbook_contracts::project_admission_governance::*;
pub use handbook_contracts::project_admission_governance::{
    AdmissionConfigurationInputV1, AuthorityModeV1, ExpectedInitialStateV1,
    FreshnessRuleBindingInputV1, GovernanceDeliveryV1, GovernanceRefusalV1,
    GovernanceRuleDescriptorV1, LocalAdvisoryInstructionV1, ReplacementAuthorityBasisV1,
    ADMISSION_GOVERNANCE_SCOPE_V1, JSON_SAFE_INTEGER_MAX, MAX_GOVERNANCE_RECORD_BYTES_V1,
    InitialProjectGovernanceV1, ProjectGovernanceCommittedV1, ProjectGovernanceNotCommittedV1,
    GovernanceProjectSelectionV1, ExpectedCurrentGovernanceV1, CurrentProjectGovernanceV1,
    EstablishedGovernanceOriginV1,
};
pub use handbook_contracts::{ExactBindingV1, ExactRefV1, FingerprintV1};
pub use crate::governance_v1::*;
use handbook_contracts::project_admission_governance as owner;

/// A complete original request identity. Restoring this value authorizes only
/// resolution; it never reconstructs a creation or establishment capability.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(
    tag = "operation",
    content = "original",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ProjectGovernanceLocatorV1 {
    InitializeDeferred(InitialDeferralLocatorV1),
    InitializeEstablished(InitialEstablishmentLocatorV1),
    EstablishDeferred(GovernanceEstablishmentLocatorV1),
}

impl ProjectGovernanceLocatorV1 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, GovernanceRefusalV1> {
        match self {
            Self::InitializeDeferred(value) => {
                let native: DeferralRequestLocatorV2 = crate::governance_v1::translate(value)?;
                native.canonical_json_bytes()?;
            }
            Self::InitializeEstablished(value) => {
                let native: GovernanceRequestLocatorV1 = crate::governance_v1::translate(value)?;
                native.canonical_json_bytes()?;
            }
            Self::EstablishDeferred(value) => {
                let native: DeferredEstablishmentLocatorV3 =
                    crate::governance_v1::translate(value)?;
                native.canonical_json_bytes()?;
            }
        }
        crate::contract_membrane::canonical_json_bytes(
            self,
            &crate::contract_membrane::ContractLimits::packet0(),
        )
        .map_err(|error| GovernanceRefusalV1::new("locator_transport", error.to_string()))
    }

    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, GovernanceRefusalV1> {
        let value: Self = crate::contract_membrane::parse_canonical_json(
            bytes,
            &crate::contract_membrane::ContractLimits::packet0(),
        )
        .map_err(|error| GovernanceRefusalV1::new("locator_transport", error.to_string()))?;
        value.canonical_json_bytes()?;
        Ok(value)
    }
}

#[derive(Debug)]
pub struct PreparedInitialProjectGovernanceV1 {
    locator: ProjectGovernanceLocatorV1,
    prepared: owner::PreparedInitialProjectGovernanceV1,
}
impl PreparedInitialProjectGovernanceV1 {
    pub fn locator(&self) -> &ProjectGovernanceLocatorV1 { &self.locator }
}
#[derive(Debug)]
pub struct PreparedProjectGovernanceEstablishmentV1 {
    locator: ProjectGovernanceLocatorV1,
    prepared: owner::PreparedProjectGovernanceEstablishmentV1,
}
impl PreparedProjectGovernanceEstablishmentV1 {
    pub fn locator(&self) -> &ProjectGovernanceLocatorV1 { &self.locator }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ProjectGovernanceExecutionV1 {
    Committed {
        outcome: ProjectGovernanceCommittedV1,
        delivery: GovernanceDeliveryV1,
    },
    Refused {
        refusal: GovernanceRefusalV1,
    },
    NotCommitted {
        outcome: ProjectGovernanceNotCommittedV1,
        delivery: GovernanceDeliveryV1,
    },
    Uncertain {
        locator: ProjectGovernanceLocatorV1,
        condition: String,
    },
}


fn to_owner(value: &ProjectGovernanceLocatorV1) -> owner::ProjectGovernanceLocatorV1 {
    match value {
        ProjectGovernanceLocatorV1::InitializeDeferred(v) => owner::ProjectGovernanceLocatorV1::InitializeDeferred(v.clone()),
        ProjectGovernanceLocatorV1::InitializeEstablished(v) => owner::ProjectGovernanceLocatorV1::InitializeEstablished(v.clone()),
        ProjectGovernanceLocatorV1::EstablishDeferred(v) => owner::ProjectGovernanceLocatorV1::EstablishDeferred(v.clone()),
    }
}
fn from_owner(value: &owner::ProjectGovernanceLocatorV1) -> ProjectGovernanceLocatorV1 {
    match value {
        owner::ProjectGovernanceLocatorV1::InitializeDeferred(v) => ProjectGovernanceLocatorV1::InitializeDeferred(v.clone()),
        owner::ProjectGovernanceLocatorV1::InitializeEstablished(v) => ProjectGovernanceLocatorV1::InitializeEstablished(v.clone()),
        owner::ProjectGovernanceLocatorV1::EstablishDeferred(v) => ProjectGovernanceLocatorV1::EstablishDeferred(v.clone()),
    }
}
fn convert_execution(value: owner::ProjectGovernanceExecutionV1) -> ProjectGovernanceExecutionV1 {
    match value {
        owner::ProjectGovernanceExecutionV1::Committed { outcome, delivery } => ProjectGovernanceExecutionV1::Committed { outcome, delivery },
        owner::ProjectGovernanceExecutionV1::NotCommitted { outcome, delivery } => ProjectGovernanceExecutionV1::NotCommitted { outcome, delivery },
        owner::ProjectGovernanceExecutionV1::Refused { refusal } => ProjectGovernanceExecutionV1::Refused { refusal },
        owner::ProjectGovernanceExecutionV1::Uncertain { locator, condition } => ProjectGovernanceExecutionV1::Uncertain { locator: from_owner(&locator), condition },
    }
}
impl HandbookSdkV1 {
    pub fn prepare_initial_project_governance(&self, choice: &InitialProjectGovernanceV1) -> Result<PreparedInitialProjectGovernanceV1, GovernanceRefusalV1> {
        let prepared = owner::ProjectGovernanceV1::new().prepare_initial_project_governance(&self.repo_root, choice)?;
        Ok(PreparedInitialProjectGovernanceV1 { locator: from_owner(prepared.locator()), prepared })
    }
    pub fn initialize_project_governance(&self, prepared: PreparedInitialProjectGovernanceV1) -> ProjectGovernanceExecutionV1 {
        convert_execution(owner::ProjectGovernanceV1::new().initialize_project_governance(&self.repo_root, prepared.prepared))
    }
    pub fn prepare_project_governance_establishment(&self, request: &GovernanceEstablishmentRequestV1) -> Result<PreparedProjectGovernanceEstablishmentV1, GovernanceRefusalV1> {
        let prepared = owner::ProjectGovernanceV1::new().prepare_project_governance_establishment(&self.repo_root, request)?;
        Ok(PreparedProjectGovernanceEstablishmentV1 { locator: from_owner(prepared.locator()), prepared })
    }
    pub fn establish_project_governance(&self, prepared: PreparedProjectGovernanceEstablishmentV1) -> ProjectGovernanceExecutionV1 {
        convert_execution(owner::ProjectGovernanceV1::new().establish_project_governance(&self.repo_root, prepared.prepared))
    }
    pub fn resolve_project_governance_request(&self, locator: &ProjectGovernanceLocatorV1) -> ProjectGovernanceExecutionV1 {
        if let Err(refusal) = locator.canonical_json_bytes() { return ProjectGovernanceExecutionV1::Refused { refusal }; }
        convert_execution(owner::ProjectGovernanceV1::new().resolve_project_governance_request(&self.repo_root, &to_owner(locator)))
    }
    pub fn resolve_current_project_governance(&self, project: &GovernanceProjectSelectionV1, scope: &str, expected: Option<&ExpectedCurrentGovernanceV1>) -> Result<CurrentProjectGovernanceV1, GovernanceRefusalV1> {
        owner::ProjectGovernanceV1::new().resolve_current_project_governance(&self.repo_root, project, scope, expected)
    }
}

#[cfg(feature = "governance-test-support")]
pub use owner::GovernanceTestFaultV1;
#[cfg(feature = "governance-test-support")]
impl HandbookSdkV1 {
    pub fn initialize_project_governance_with_fault_for_testing(&self, prepared: PreparedInitialProjectGovernanceV1, fault: GovernanceTestFaultV1) -> ProjectGovernanceExecutionV1 {
        convert_execution(owner::ProjectGovernanceV1::new().initialize_with_fault_for_testing(&self.repo_root, prepared.prepared, fault))
    }
    pub fn resolve_project_governance_request_with_fault_for_testing(
        &self, locator: &ProjectGovernanceLocatorV1, fault: GovernanceTestFaultV1,
    ) -> ProjectGovernanceExecutionV1 {
        if let Err(refusal) = locator.canonical_json_bytes() {
            return ProjectGovernanceExecutionV1::Refused { refusal };
        }
        convert_execution(owner::ProjectGovernanceV1::new()
            .resolve_project_governance_request_with_fault_for_testing(&self.repo_root, &to_owner(locator), fault))
    }
    pub fn establish_project_governance_with_fault_for_testing(&self, prepared: PreparedProjectGovernanceEstablishmentV1, fault: GovernanceTestFaultV1) -> ProjectGovernanceExecutionV1 {
        convert_execution(owner::ProjectGovernanceV1::new().establish_with_fault_for_testing(&self.repo_root, prepared.prepared, fault))
    }
}

#[cfg(all(test, feature = "governance-test-support"))]
mod unified_mapping_tests {
    use super::*;
    use handbook_contracts::project_admission_governance::GovernanceFaultBoundaryV1;

    #[test]
    fn injected_owner_uncertainty_maps_to_uncertainty_with_original_locator() {
        let dir = tempfile::tempdir().unwrap();
        let sdk = HandbookSdkV1::open(dir.path());
        let request = InitialEstablishmentRequestV1::standalone("mapping-uncertain", 7200, "Approve the exact governance.").unwrap();
        let prepared = sdk.prepare_initial_project_governance(&InitialProjectGovernanceV1::Established(request)).unwrap();
        let original = prepared.locator().clone();
        let result = sdk.initialize_project_governance_with_fault_for_testing(prepared, GovernanceTestFaultV1::Initial(GovernanceFaultBoundaryV1::Decision));
        assert!(matches!(result, ProjectGovernanceExecutionV1::Uncertain { locator, .. } if locator == original));
        assert!(matches!(sdk.resolve_project_governance_request(&original), ProjectGovernanceExecutionV1::Committed {
            outcome: ProjectGovernanceCommittedV1::InitialEstablished(_),
            delivery: GovernanceDeliveryV1::Replay,
        }));
    }
}
