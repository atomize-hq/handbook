use handbook_engine::{
    baseline_artifact_validation, baseline_artifact_validation_for_path,
    baseline_artifact_validations, BaselineArtifactVerdict, CanonicalArtifactKind,
    CanonicalArtifacts, CanonicalLayoutContract,
};

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

fn make_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    write_file(&root.join(".handbook/charter/CHARTER.md"), b"valid charter");
    write_file(
        &root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
        b"valid project context",
    );
    write_file(
        &root.join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        b"valid environment inventory",
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );
    dir
}

fn custom_layout_contract() -> CanonicalLayoutContract {
    CanonicalLayoutContract::from_paths(
        ".custom_handbook",
        ".custom_handbook/charter",
        ".custom_handbook/charter/CHARTER.md",
        ".custom_handbook/project_context",
        ".custom_handbook/project_context/PROJECT_CONTEXT.md",
        ".custom_handbook/environment_inventory",
        ".custom_handbook/environment_inventory/ENVIRONMENT_INVENTORY.md",
        ".custom_handbook/feature_spec",
        ".custom_handbook/feature_spec/FEATURE_SPEC.md",
    )
}

fn test_validator(kind: CanonicalArtifactKind, markdown: &str) -> Result<(), String> {
    match kind {
        CanonicalArtifactKind::Charter if markdown.contains("valid charter") => Ok(()),
        CanonicalArtifactKind::ProjectContext if markdown.contains("valid project context") => {
            Ok(())
        }
        CanonicalArtifactKind::EnvironmentInventory
            if markdown.contains("valid environment inventory") =>
        {
            Ok(())
        }
        CanonicalArtifactKind::FeatureSpec => {
            Err("feature spec is not part of baseline validation".to_string())
        }
        _ => Err(format!("unexpected markdown for {kind:?}")),
    }
}

#[test]
fn baseline_validation_uses_supplied_validator() {
    let dir = make_repo();
    let artifacts = CanonicalArtifacts::load(dir.path()).expect("artifacts");

    let validations = baseline_artifact_validations(&artifacts, test_validator);
    assert_eq!(validations.len(), 3);
    assert!(validations.iter().all(|validation| {
        matches!(
            validation.verdict,
            BaselineArtifactVerdict::ValidCanonicalTruth { .. }
        )
    }));
}

#[test]
fn baseline_validation_reports_semantic_invalidity_from_validator() {
    let dir = make_repo();
    let artifacts = CanonicalArtifacts::load(dir.path()).expect("artifacts");

    let validation = baseline_artifact_validation(
        &artifacts,
        CanonicalArtifactKind::ProjectContext,
        |_kind, _markdown| Err("project context failed semantic validation".to_string()),
    )
    .expect("validation");

    assert_eq!(
        validation.verdict,
        BaselineArtifactVerdict::SemanticallyInvalid {
            summary: "project context failed semantic validation".to_string(),
        }
    );
}

#[test]
fn baseline_validation_for_path_selects_matching_validation() {
    let dir = make_repo();
    let artifacts = CanonicalArtifacts::load(dir.path()).expect("artifacts");
    let validations = baseline_artifact_validations(&artifacts, test_validator);

    let found = baseline_artifact_validation_for_path(
        &validations,
        ".handbook/project_context/PROJECT_CONTEXT.md",
    )
    .expect("matching validation");

    assert_eq!(found.kind, CanonicalArtifactKind::ProjectContext);
}

#[test]
fn baseline_validation_uses_loaded_custom_paths_and_custom_ingest_issue_paths() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    std::fs::create_dir_all(repo_root.join(".custom_handbook/charter/CHARTER.md"))
        .expect("charter dir");
    write_file(
        &repo_root.join(".custom_handbook/project_context/PROJECT_CONTEXT.md"),
        b"valid project context",
    );
    write_file(
        &repo_root.join(".custom_handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        b"valid environment inventory",
    );

    let artifacts =
        CanonicalArtifacts::load_with_contract(repo_root, custom_layout_contract()).expect("load");

    let validations = baseline_artifact_validations(&artifacts, test_validator);
    assert_eq!(
        validations
            .iter()
            .map(|validation| validation.canonical_repo_relative_path)
            .collect::<Vec<_>>(),
        vec![
            ".custom_handbook/charter/CHARTER.md",
            ".custom_handbook/project_context/PROJECT_CONTEXT.md",
            ".custom_handbook/environment_inventory/ENVIRONMENT_INVENTORY.md",
        ]
    );

    let charter =
        baseline_artifact_validation(&artifacts, CanonicalArtifactKind::Charter, test_validator)
            .expect("charter validation");
    assert_eq!(
        charter.canonical_repo_relative_path,
        ".custom_handbook/charter/CHARTER.md"
    );
    assert_eq!(charter.verdict, BaselineArtifactVerdict::IngestInvalid);

    let found =
        baseline_artifact_validation_for_path(&validations, ".custom_handbook/charter/CHARTER.md")
            .expect("matching validation");
    assert_eq!(found.verdict, BaselineArtifactVerdict::IngestInvalid);
}
