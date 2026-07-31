use handbook_engine::{
    baseline_artifact_validation, baseline_artifact_validation_for_path,
    baseline_artifact_validations, BaselineArtifactVerdict, CanonicalArtifactIdentity,
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
    write_file(
        &root.join(".handbook/project/charter.yaml"),
        b"valid charter",
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        b"valid project context",
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
        ".custom_handbook/feature_spec",
        ".custom_handbook/feature_spec/FEATURE_SPEC.md",
    )
}

fn test_validator(identity: &CanonicalArtifactIdentity, markdown: &str) -> Result<(), String> {
    match identity.instance_id.as_str() {
        "project_authority" if markdown.contains("valid charter") => Ok(()),
        "project_context" if markdown.contains("valid project context") => Ok(()),
        _ => Err(format!("unexpected markdown for {}", identity.instance_id)),
    }
}

#[test]
fn baseline_validation_uses_supplied_validator() {
    let dir = make_repo();
    let artifacts = CanonicalArtifacts::load(dir.path()).expect("artifacts");

    let validations = baseline_artifact_validations(&artifacts, test_validator);
    assert_eq!(validations.len(), 2);
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

    let validation =
        baseline_artifact_validation(&artifacts, "project_context", |_identity, _markdown| {
            Err("project context failed semantic validation".to_string())
        })
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

    let found =
        baseline_artifact_validation_for_path(&validations, ".handbook/project/context.yaml")
            .expect("matching validation");

    assert_eq!(found.instance_id, "project_context");
    assert_eq!(found.label, "Project Context");
}

#[test]
fn baseline_validation_uses_loaded_custom_paths_and_custom_ingest_issue_paths() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    std::fs::create_dir_all(repo_root.join(".custom_handbook")).expect("custom root");
    std::fs::create_dir_all(repo_root.join(".handbook/project/charter.yaml")).expect("charter dir");
    write_file(
        &repo_root.join(".handbook/project/context.yaml"),
        b"valid project context",
    );

    let artifacts =
        CanonicalArtifacts::load_with_contract(repo_root, custom_layout_contract()).expect("load");

    let validations = baseline_artifact_validations(&artifacts, test_validator);
    assert_eq!(
        validations
            .iter()
            .map(|validation| validation.canonical_repo_relative_path.as_str())
            .collect::<Vec<_>>(),
        vec![
            ".handbook/project/charter.yaml",
            ".handbook/project/context.yaml",
        ]
    );

    let charter = baseline_artifact_validation(&artifacts, "project_authority", test_validator)
        .expect("charter validation");
    assert_eq!(
        charter.canonical_repo_relative_path,
        ".handbook/project/charter.yaml"
    );
    assert_eq!(charter.verdict, BaselineArtifactVerdict::IngestInvalid);

    let found =
        baseline_artifact_validation_for_path(&validations, ".handbook/project/charter.yaml")
            .expect("matching validation");
    assert_eq!(found.verdict, BaselineArtifactVerdict::IngestInvalid);
}
