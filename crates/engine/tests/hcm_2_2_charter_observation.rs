use handbook_engine::{
    charter_rendered_fingerprint, charter_source_fingerprint, inspect_profile_repository,
    inspect_profile_repository_with_stability_hook, load_selected_charter,
    resolve_shipped_profile_decisions, ArtifactInspectionReason, ArtifactInspectionStatus,
};
use std::fs;
use std::path::Path;

const CHARTER: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));
const MARKDOWN: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/charter-review-boundary-v1.0.md"
));

fn write_charter(repo: &Path, bytes: &[u8]) {
    let project = repo.join(".handbook/project");
    fs::create_dir_all(&project).unwrap();
    fs::write(project.join("charter.yaml"), bytes).unwrap();
}

#[test]
fn selected_charter_projection_retains_exact_source_and_render_domains() {
    let repo = tempfile::tempdir().unwrap();
    write_charter(repo.path(), CHARTER);
    let decisions = resolve_shipped_profile_decisions(repo.path()).unwrap();

    let projection = load_selected_charter(repo.path(), &decisions).unwrap();

    assert_eq!(
        projection.canonical_path(),
        ".handbook/project/charter.yaml"
    );
    assert_eq!(projection.source_byte_length(), CHARTER.len());
    assert_eq!(projection.rendered_bytes(), MARKDOWN);
    assert_eq!(
        projection.source_fingerprint(),
        &charter_source_fingerprint(CHARTER)
    );
    assert_eq!(
        projection.rendered_output_fingerprint(),
        &charter_rendered_fingerprint(MARKDOWN)
    );
}

#[test]
fn profile_inspection_projects_selected_charter_and_rejects_post_read_change() {
    let repo = tempfile::tempdir().unwrap();
    write_charter(repo.path(), CHARTER);
    let decisions = resolve_shipped_profile_decisions(repo.path()).unwrap();
    let report = inspect_profile_repository(repo.path(), &decisions);
    let charter = report
        .artifacts()
        .iter()
        .find(|artifact| artifact.instance_id().as_str() == "project_authority")
        .unwrap();
    assert_eq!(
        charter.status(),
        ArtifactInspectionStatus::StructurallyValid
    );
    assert_eq!(
        charter.reason(),
        ArtifactInspectionReason::PresentAndStructurallyValid
    );
    assert!(charter.charter_projection().is_some());
    drop(report);

    write_charter(repo.path(), CHARTER);
    #[cfg(unix)]
    let changed = inspect_profile_repository_with_stability_hook(repo.path(), &decisions, || {
        fs::write(
            repo.path().join(".handbook/project/charter.yaml"),
            b"changed\n",
        )
        .unwrap();
    });
    #[cfg(windows)]
    let changed = inspect_profile_repository_with_stability_hook(repo.path(), &decisions, || {
        assert!(fs::write(
            repo.path().join(".handbook/project/charter.yaml"),
            b"changed\n"
        )
        .is_err());
    });
    let charter = changed
        .artifacts()
        .iter()
        .find(|artifact| artifact.instance_id().as_str() == "project_authority")
        .unwrap();
    #[cfg(unix)]
    {
        assert_eq!(charter.status(), ArtifactInspectionStatus::Unreadable);
        assert_eq!(
            charter.reason(),
            ArtifactInspectionReason::ObservationChangedDuringInspection
        );
        assert!(charter.charter_projection().is_none());
    }
    #[cfg(windows)]
    {
        assert_eq!(
            charter.status(),
            ArtifactInspectionStatus::StructurallyValid
        );
        assert!(charter.charter_projection().is_some());
    }
}
