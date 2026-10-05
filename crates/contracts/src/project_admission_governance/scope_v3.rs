//! Additive project-contract tags preserve the existing v1/v2 meanings.
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
pub enum ProjectAdmissionBindingV3 {
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
pub enum ProjectAdmissionScopeBasisV3 {
    EstablishedV1(ActiveGovernanceBasisV1),
    DeferredV2(DeferredGovernanceBasisV2),
    EstablishedFromDeferredV3(EstablishedFromDeferredBasisV3),
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "project_contract",
    content = "basis",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ExpectedProjectAdmissionBasisV3 {
    EstablishedV1(ExpectedGovernanceBasisV1),
    DeferredV2(ExpectedDeferredBasisV2),
    EstablishedFromDeferredV3(ExpectedEstablishedFromDeferredBasisV3),
}
impl ProjectAdmissionBindingV3 {
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, GovernanceRefusalV1> {
        decode_governance_json(bytes)
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, GovernanceRefusalV1> {
        governance_canonical_bytes(self)
    }
}
pub fn resolve_project_admission_scope_v3(
    root: impl AsRef<Path>,
    project: &ProjectAdmissionBindingV3,
    scope: &str,
    expected: Option<&ExpectedProjectAdmissionBasisV3>,
) -> Result<ProjectAdmissionScopeBasisV3, GovernanceRefusalV1> {
    match (project, expected) {
        (ProjectAdmissionBindingV3::EstablishedV1(p), None) => ProjectAdmissionGovernanceV1::new()
            .resolve_active(root, p, scope, None)
            .map(ProjectAdmissionScopeBasisV3::EstablishedV1),
        (
            ProjectAdmissionBindingV3::EstablishedV1(p),
            Some(ExpectedProjectAdmissionBasisV3::EstablishedV1(e)),
        ) => ProjectAdmissionGovernanceV1::new()
            .resolve_active(root, p, scope, Some(e))
            .map(ProjectAdmissionScopeBasisV3::EstablishedV1),
        (
            ProjectAdmissionBindingV3::ScopedChoiceV2(p),
            None
            | Some(ExpectedProjectAdmissionBasisV3::DeferredV2(_))
            | Some(ExpectedProjectAdmissionBasisV3::EstablishedFromDeferredV3(_)),
        ) => ProjectAdmissionEstablishmentV3::new()
            .resolve_current(root, p, scope, expected)
            .map(|b| match b {
                DeferredScopeBasisV3::DeferredV2(b) => ProjectAdmissionScopeBasisV3::DeferredV2(b),
                DeferredScopeBasisV3::EstablishedFromDeferredV3(b) => {
                    ProjectAdmissionScopeBasisV3::EstablishedFromDeferredV3(b)
                }
            }),
        _ => Err(refusal(
            "basis_contract_conflict",
            "Expected basis must name the exact project contract",
        )),
    }
}
