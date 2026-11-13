use crate::doctor::{
    doctor, doctor_with_decisions, DoctorError, DoctorErrorKind, DoctorErrorReasonCode,
};
#[cfg(all(not(unix), not(windows)))]
use crate::profile_readiness::RepositoryReadinessStatus;
use handbook_engine::resolve_shipped_profile_decisions;
use tempfile::tempdir;

#[test]
fn doctor_emits_complete_closed_profile_rows() {
    let repo = tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(repo.path()).unwrap();
    let report = doctor_with_decisions(repo.path(), &decisions).unwrap();

    assert_eq!(report.profile_ref, decisions.profile_ref().as_str());
    assert_eq!(report.artifacts.len(), decisions.artifact_decisions().len());
    assert_eq!(
        report.conditions.len(),
        decisions.condition_evaluations().len()
    );
    assert_eq!(
        report.capabilities.len(),
        decisions.capability_truth().len()
    );
}

#[test]
fn default_and_injected_doctor_reports_are_identical() {
    let repo = tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(repo.path()).unwrap();
    assert_eq!(
        doctor(repo.path()).unwrap(),
        doctor_with_decisions(repo.path(), &decisions).unwrap()
    );
}

#[cfg(unix)]
#[test]
fn doctor_api_projects_the_exact_stable_project_context_row() {
    const PROJECT_CONTEXT: &str = concat!(
        "schema_id: \"handbook.artifact.project-context\"\n",
        "schema_version: \"1.0\"\n",
        "record_id: \"handbook.project-context\"\n",
        "summary: \"Project reality.\"\n",
        "system_boundaries:\n",
        "  - \"Canonical handbook truth\"\n",
        "ownership:\n",
        "  - \"handbook-team\"\n",
        "authoritative_references:\n",
        "  - \"handbook.charter@1.0.0\"\n",
        "known_unknowns:\n",
        "  - \"None\"\n",
    );
    let repo = tempdir().unwrap();
    let selected = repo.path().join(".handbook/project/context.yaml");
    std::fs::create_dir_all(selected.parent().unwrap()).unwrap();
    std::fs::write(&selected, PROJECT_CONTEXT).unwrap();
    let decisions = resolve_shipped_profile_decisions(repo.path()).unwrap();
    let report = doctor_with_decisions(repo.path(), &decisions).unwrap();
    let row = report
        .project_context
        .expect("non-null Project Context row");

    assert_eq!(row.instance_id, "project_context");
    assert_eq!(row.kind_ref, "handbook.artifact-kind.project-context@1.1.0");
    assert_eq!(row.canonical_path, ".handbook/project/context.yaml");
    assert_eq!(
        row.source_fingerprint,
        "sha256:c8646d5821d80fe8e0eeade2713fc63f6e91b2cbb64f6832b5a1f178b810d02f"
    );
    assert_eq!(
        row.rendered_output_fingerprint,
        "sha256:9502d897dc9542a492fdc50ca9ebb2340ac59be25b37a615ea6cb842387641fd"
    );
    assert_eq!(row.rendered_media_type, "text/markdown");
}

#[test]
fn doctor_error_projection_is_exhaustive() {
    let expected = [
        (
            DoctorError::ShippedProfileUnavailable,
            DoctorErrorKind::ProfileResolution,
            DoctorErrorReasonCode::ShippedProfileUnavailable,
        ),
        (
            DoctorError::SelectedProfileDecisionInvalid,
            DoctorErrorKind::ProfileDecision,
            DoctorErrorReasonCode::SelectedProfileDecisionInvalid,
        ),
    ];
    assert_eq!(DoctorError::ALL, expected.map(|row| row.0));
    for (error, kind, reason) in expected {
        assert_eq!(error.kind(), kind);
        assert_eq!(error.reason_code(), reason);
    }
}

#[cfg(all(not(unix), not(windows)))]
#[test]
fn non_unix_doctor_reports_invalid_without_reading_artifacts() {
    let repo = tempdir().unwrap();
    let report = doctor(repo.path()).unwrap();
    assert_eq!(report.status, RepositoryReadinessStatus::Invalid);
    assert!(report.project_context.is_none());
    let project_context = report
        .artifacts
        .iter()
        .find(|artifact| artifact.instance_id == "project_context")
        .expect("Project Context artifact row");
    assert_eq!(
        project_context.canonical_path,
        ".handbook/project/context.yaml"
    );
    assert_eq!(
        project_context.inspection_status,
        crate::profile_readiness::ArtifactInspectionStatus::UnsafePath
    );
    assert_eq!(
        project_context.inspection_reason,
        crate::profile_readiness::ArtifactInspectionReason::UnsupportedPlatformStrictRead
    );
}
