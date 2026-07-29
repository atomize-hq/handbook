use handbook_engine::{
    ArtifactApplicability, ArtifactInspectionReason, ArtifactInspectionStatus,
    ProfileInspectionReport, ProjectConditionDecisionReason, ProjectConditionOutcome,
    RequirednessMode, ResolvedProfileDecisions,
};
use serde::Serialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryReadinessStatus {
    Ready,
    ActionRequired,
    Indeterminate,
    Invalid,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProfileConditionRow {
    pub condition_ref: String,
    pub condition_definition_fingerprint: String,
    pub outcome: ProjectConditionOutcome,
    pub reason: ProjectConditionDecisionReason,
    pub evidence_closure_fingerprint: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProfileCapabilityRow {
    pub instance_id: String,
    pub capability_id: String,
    pub contract_ref: String,
    pub contract_fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProfileArtifactRow {
    pub instance_id: String,
    pub kind_ref: String,
    pub role_id: Option<String>,
    pub capability_ids: Vec<String>,
    pub canonical_path: String,
    pub requiredness: RequirednessMode,
    pub condition_ref: Option<String>,
    pub condition_outcome: Option<ProjectConditionOutcome>,
    pub condition_reason: Option<ProjectConditionDecisionReason>,
    pub evidence_closure_fingerprint: Option<String>,
    pub applicability: ArtifactApplicability,
    pub inspection_status: ArtifactInspectionStatus,
    pub inspection_reason: ArtifactInspectionReason,
}

#[derive(Clone, Debug)]
pub(crate) struct ProfileReadinessProjection {
    pub(crate) profile_ref: String,
    pub(crate) profile_fingerprint: String,
    pub(crate) stable_role_registry_ref: String,
    pub(crate) stable_role_registry_fingerprint: String,
    pub(crate) conditions: Vec<ProfileConditionRow>,
    pub(crate) capabilities: Vec<ProfileCapabilityRow>,
    pub(crate) artifacts: Vec<ProfileArtifactRow>,
    pub(crate) status: RepositoryReadinessStatus,
}

pub(crate) fn project_profile_readiness(
    decisions: &ResolvedProfileDecisions,
    inspection: &ProfileInspectionReport,
) -> ProfileReadinessProjection {
    debug_assert_eq!(decisions.profile_ref(), inspection.profile_ref());
    debug_assert_eq!(
        decisions.profile_fingerprint(),
        inspection.profile_fingerprint()
    );

    let conditions = decisions
        .condition_evaluations()
        .iter()
        .map(|evaluation| ProfileConditionRow {
            condition_ref: evaluation.condition_ref().as_str().to_owned(),
            condition_definition_fingerprint: evaluation
                .condition_definition_fingerprint()
                .as_str()
                .to_owned(),
            outcome: evaluation.outcome(),
            reason: evaluation.reason(),
            evidence_closure_fingerprint: evaluation
                .evidence_closure_fingerprint()
                .map(|fingerprint| fingerprint.as_str().to_owned()),
        })
        .collect();
    let capabilities = decisions
        .capability_truth()
        .iter()
        .map(|capability| ProfileCapabilityRow {
            instance_id: capability.instance_id().as_str().to_owned(),
            capability_id: capability.capability_id().as_str().to_owned(),
            contract_ref: capability.contract_ref().as_str().to_owned(),
            contract_fingerprint: capability.contract_fingerprint().as_str().to_owned(),
        })
        .collect();
    let artifacts = decisions
        .artifact_decisions()
        .iter()
        .zip(inspection.artifacts())
        .map(|(decision, inspection)| {
            debug_assert_eq!(decision.instance_id(), inspection.instance_id());
            let mut capability_ids = decision
                .capabilities()
                .iter()
                .map(|capability| capability.capability_id().as_str().to_owned())
                .collect::<Vec<_>>();
            capability_ids.sort();
            ProfileArtifactRow {
                instance_id: decision.instance_id().as_str().to_owned(),
                kind_ref: decision.kind_ref().as_str().to_owned(),
                role_id: decision.role_id().map(str::to_owned),
                capability_ids,
                canonical_path: decision.canonical_path().to_owned(),
                requiredness: decision.requiredness_mode(),
                condition_ref: decision
                    .condition_ref()
                    .map(|reference| reference.as_str().to_owned()),
                condition_outcome: decision.condition_outcome(),
                condition_reason: decision.condition_reason(),
                evidence_closure_fingerprint: decision
                    .evidence_closure_fingerprint()
                    .map(|fingerprint| fingerprint.as_str().to_owned()),
                applicability: decision.applicability(),
                inspection_status: inspection.status(),
                inspection_reason: inspection.reason(),
            }
        })
        .collect::<Vec<_>>();
    let status = classify_readiness(&artifacts);

    ProfileReadinessProjection {
        profile_ref: decisions.profile_ref().as_str().to_owned(),
        profile_fingerprint: decisions.profile_fingerprint().as_str().to_owned(),
        stable_role_registry_ref: decisions.stable_role_registry_ref().as_str().to_owned(),
        stable_role_registry_fingerprint: decisions
            .stable_role_registry_fingerprint()
            .as_str()
            .to_owned(),
        conditions,
        capabilities,
        artifacts,
        status,
    }
}

fn classify_readiness(artifacts: &[ProfileArtifactRow]) -> RepositoryReadinessStatus {
    if artifacts.iter().any(|artifact| {
        !is_unselected_environment_context_advisory(artifact)
            && matches!(
                artifact.inspection_status,
                ArtifactInspectionStatus::StructurallyInvalid
                    | ArtifactInspectionStatus::UnsafePath
                    | ArtifactInspectionStatus::Unreadable
            )
    }) {
        return RepositoryReadinessStatus::Invalid;
    }
    if artifacts
        .iter()
        .any(|artifact| artifact.applicability == ArtifactApplicability::Indeterminate)
    {
        return RepositoryReadinessStatus::Indeterminate;
    }
    if artifacts.iter().any(|artifact| {
        artifact.applicability == ArtifactApplicability::Required
            && artifact.inspection_status == ArtifactInspectionStatus::Missing
    }) {
        return RepositoryReadinessStatus::ActionRequired;
    }
    RepositoryReadinessStatus::Ready
}

fn is_unselected_environment_context_advisory(artifact: &ProfileArtifactRow) -> bool {
    artifact.instance_id == "environment_context"
        && artifact.kind_ref == "handbook.artifact-kind.environment-context@1.1.0"
        && artifact.requiredness == RequirednessMode::Optional
        && artifact.condition_ref.is_none()
        && artifact.applicability == ArtifactApplicability::Optional
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(
        applicability: ArtifactApplicability,
        inspection_status: ArtifactInspectionStatus,
    ) -> ProfileArtifactRow {
        ProfileArtifactRow {
            instance_id: "environment_context".to_owned(),
            kind_ref: "handbook.artifact-kind.environment-context@1.1.0".to_owned(),
            role_id: Some("environment_context".to_owned()),
            capability_ids: Vec::new(),
            canonical_path: ".handbook/project/environment.yaml".to_owned(),
            requiredness: RequirednessMode::Optional,
            condition_ref: None,
            condition_outcome: None,
            condition_reason: None,
            evidence_closure_fingerprint: None,
            applicability,
            inspection_status,
            inspection_reason: ArtifactInspectionReason::StructuralValidationFailed,
        }
    }

    #[test]
    fn invalid_optional_context_is_non_blocking_without_a_selected_gate() {
        let artifacts = [row(
            ArtifactApplicability::Optional,
            ArtifactInspectionStatus::StructurallyInvalid,
        )];

        assert_eq!(
            classify_readiness(&artifacts),
            RepositoryReadinessStatus::Ready
        );
    }

    #[test]
    fn invalid_required_artifact_remains_blocking() {
        let artifacts = [row(
            ArtifactApplicability::Required,
            ArtifactInspectionStatus::StructurallyInvalid,
        )];

        assert_eq!(
            classify_readiness(&artifacts),
            RepositoryReadinessStatus::Invalid
        );
    }

    #[test]
    fn invalid_optional_non_advisory_artifact_remains_blocking() {
        let mut artifact = row(
            ArtifactApplicability::Optional,
            ArtifactInspectionStatus::StructurallyInvalid,
        );
        artifact.instance_id = "optional_release_notes".to_owned();
        artifact.kind_ref = "example.artifact-kind.release-notes@1.0.0".to_owned();
        artifact.role_id = None;
        artifact.canonical_path = ".handbook/project/release-notes.yaml".to_owned();

        assert_eq!(
            classify_readiness(&[artifact]),
            RepositoryReadinessStatus::Invalid
        );
    }
}
