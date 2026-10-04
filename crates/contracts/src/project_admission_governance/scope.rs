//! Explicit versioned scope resolution. These tags select an owner contract;
//! neither resolution nor a deferred basis grants qualification or action authority.
use super::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "project_contract",
    content = "binding",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ProjectAdmissionBindingV2 {
    EstablishedV1(ProjectBindingV1),
    ScopedChoiceV2(ProjectBindingV2),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "state",
    content = "basis",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ProjectAdmissionScopeBasisV2 {
    Established(ActiveGovernanceBasisV1),
    Deferred(DeferredGovernanceBasisV2),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "project_contract",
    content = "basis",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ExpectedProjectAdmissionBasisV2 {
    EstablishedV1(ExpectedGovernanceBasisV1),
    ScopedChoiceV2(ExpectedDeferredBasisV2),
}

impl ProjectAdmissionBindingV2 {
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, GovernanceRefusalV1> {
        decode_governance_json(bytes)
    }

    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, GovernanceRefusalV1> {
        governance_canonical_bytes(self)
    }
}

pub fn resolve_project_admission_scope(
    root: impl AsRef<Path>,
    project: &ProjectAdmissionBindingV2,
    scope: &str,
    expected: Option<&ExpectedProjectAdmissionBasisV2>,
) -> Result<ProjectAdmissionScopeBasisV2, GovernanceRefusalV1> {
    match (project, expected) {
        (ProjectAdmissionBindingV2::EstablishedV1(project), None) => {
            ProjectAdmissionGovernanceV1::new()
                .resolve_active(root, project, scope, None)
                .map(ProjectAdmissionScopeBasisV2::Established)
        }
        (
            ProjectAdmissionBindingV2::EstablishedV1(project),
            Some(ExpectedProjectAdmissionBasisV2::EstablishedV1(expected)),
        ) => ProjectAdmissionGovernanceV1::new()
            .resolve_active(root, project, scope, Some(expected))
            .map(ProjectAdmissionScopeBasisV2::Established),
        (ProjectAdmissionBindingV2::ScopedChoiceV2(project), None) => {
            ProjectAdmissionDeferralV2::new()
                .resolve_current(root, project, scope, None)
                .map(ProjectAdmissionScopeBasisV2::Deferred)
        }
        (
            ProjectAdmissionBindingV2::ScopedChoiceV2(project),
            Some(ExpectedProjectAdmissionBasisV2::ScopedChoiceV2(expected)),
        ) => ProjectAdmissionDeferralV2::new()
            .resolve_current(root, project, scope, Some(expected))
            .map(ProjectAdmissionScopeBasisV2::Deferred),
        _ => Err(GovernanceRefusalV1::new(
            "basis_contract_conflict",
            "Expected basis must name the exact project registration contract",
        )),
    }
}
