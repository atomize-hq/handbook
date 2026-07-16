use handbook_engine::{
    setup_starter_template_bytes, ArtifactIngestError, ArtifactIngestIssueKind, ArtifactPresence,
    CanonicalArtifactKind, CanonicalArtifacts, CanonicalLayoutContract, SystemRootStatus,
};

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
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

#[test]
fn runtime_only_state_does_not_establish_system_root() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/state/pipeline/pipeline.foundation_inputs.yaml"),
        b"pipeline_id: pipeline.foundation_inputs\n",
    );

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(artifacts.system_root_status, SystemRootStatus::Missing);
    assert_eq!(
        artifacts.charter.identity.presence,
        ArtifactPresence::Missing
    );
    assert_eq!(
        artifacts.feature_spec.identity.presence,
        ArtifactPresence::Missing
    );
}

#[test]
fn system_root_error_display_is_contract_neutral() {
    let system_root = std::path::PathBuf::from(".custom_handbook");

    assert_eq!(
        ArtifactIngestError::SystemRootMissing {
            system_root: system_root.clone(),
        }
        .to_string(),
        "missing canonical system root at .custom_handbook"
    );
    assert_eq!(
        ArtifactIngestError::SystemRootNotDir {
            system_root: system_root.clone(),
        }
        .to_string(),
        "canonical system root is not a directory: .custom_handbook"
    );
    assert_eq!(
        ArtifactIngestError::SystemRootSymlinkNotAllowed { system_root }.to_string(),
        "canonical system root must not be a symlink: .custom_handbook"
    );
}

#[test]
fn charter_namespace_directory_establishes_partial_canonical_root() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    std::fs::create_dir_all(repo_root.join(".handbook/charter")).expect("mkdirs");

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(artifacts.system_root_status, SystemRootStatus::Ok);
    assert_eq!(
        artifacts.charter.identity.presence,
        ArtifactPresence::Missing
    );
    assert_eq!(
        artifacts.feature_spec.identity.presence,
        ArtifactPresence::Missing
    );
}

#[test]
fn project_context_namespace_directory_establishes_partial_canonical_root() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    std::fs::create_dir_all(repo_root.join(".handbook/project_context")).expect("mkdirs");

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(artifacts.system_root_status, SystemRootStatus::Ok);
    assert_eq!(
        artifacts.charter.identity.presence,
        ArtifactPresence::Missing
    );
    assert_eq!(
        artifacts.feature_spec.identity.presence,
        ArtifactPresence::Missing
    );
}

#[test]
fn feature_spec_namespace_directory_establishes_partial_canonical_root() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    std::fs::create_dir_all(repo_root.join(".handbook/feature_spec")).expect("mkdirs");

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(artifacts.system_root_status, SystemRootStatus::Ok);
    assert_eq!(
        artifacts.charter.identity.presence,
        ArtifactPresence::Missing
    );
    assert_eq!(
        artifacts.feature_spec.identity.presence,
        ArtifactPresence::Missing
    );
}

#[test]
fn environment_inventory_namespace_directory_establishes_partial_canonical_root() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    std::fs::create_dir_all(repo_root.join(".handbook/environment_inventory")).expect("mkdirs");

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(artifacts.system_root_status, SystemRootStatus::Ok);
    assert_eq!(
        artifacts.charter.identity.presence,
        ArtifactPresence::Missing
    );
    assert_eq!(
        artifacts.environment_inventory.identity.presence,
        ArtifactPresence::Missing
    );
}

#[test]
fn required_artifact_missing_is_reported_as_presence_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    std::fs::create_dir_all(repo_root.join(".handbook/charter")).expect("mkdirs");
    std::fs::create_dir_all(repo_root.join(".handbook/feature_spec")).expect("mkdirs");

    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(artifacts.system_root_status, SystemRootStatus::Ok);
    assert_eq!(
        artifacts.charter.identity.kind,
        CanonicalArtifactKind::Charter
    );
    assert_eq!(
        artifacts.charter.identity.presence,
        ArtifactPresence::Missing
    );
    assert!(artifacts.charter.bytes.is_none());
}

#[test]
fn optional_missing_is_distinct_from_empty() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(&repo_root.join(".handbook/charter/CHARTER.md"), b"charter");
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(
        artifacts.project_context.identity.presence,
        ArtifactPresence::Missing
    );
    assert!(artifacts.project_context.bytes.is_none());
    assert!(artifacts.project_context.identity.content_sha256.is_none());
}

#[test]
fn empty_means_exactly_zero_bytes() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(&repo_root.join(".handbook/charter/CHARTER.md"), b"charter");
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );
    write_file(
        &repo_root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
        b"",
    );

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(
        artifacts.project_context.identity.presence,
        ArtifactPresence::PresentEmpty
    );
    assert_eq!(artifacts.project_context.identity.byte_len, Some(0));
    assert_eq!(
        artifacts.project_context.bytes.as_deref(),
        Some(b"".as_slice())
    );
}

#[test]
fn whitespace_only_counts_as_non_empty() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(&repo_root.join(".handbook/charter/CHARTER.md"), b"charter");
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );
    write_file(
        &repo_root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
        b" \n\t",
    );

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(
        artifacts.project_context.identity.presence,
        ArtifactPresence::PresentNonEmpty
    );
    assert_eq!(artifacts.project_context.identity.byte_len, Some(3));
    assert!(
        !artifacts
            .project_context
            .identity
            .matches_setup_starter_template
    );
}

#[test]
fn required_starter_template_is_detected_exactly() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        setup_starter_template_bytes(CanonicalArtifactKind::Charter),
    );
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"completed feature spec",
    );

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert!(artifacts.charter.identity.matches_setup_starter_template);
    assert!(
        !artifacts
            .feature_spec
            .identity
            .matches_setup_starter_template
    );
}

#[test]
fn required_artifact_directory_is_recorded_as_read_error_and_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    std::fs::create_dir_all(repo_root.join(".handbook/charter/CHARTER.md")).expect("charter dir");
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(artifacts.system_root_status, SystemRootStatus::Ok);
    assert_eq!(
        artifacts.charter.identity.presence,
        ArtifactPresence::Missing
    );
    assert!(artifacts.charter.bytes.is_none());
    assert!(
        artifacts.ingest_issues.iter().any(|issue| {
            issue.kind == ArtifactIngestIssueKind::CanonicalArtifactReadError
                && issue.canonical_repo_relative_path == ".handbook/charter/CHARTER.md"
        }),
        "expected read-error ingest issue, got: {:?}",
        artifacts.ingest_issues
    );
}

#[test]
fn non_default_layout_contract_flows_through_ingest_identities_and_issue_paths() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    std::fs::create_dir_all(repo_root.join(".custom_handbook/charter/CHARTER.md"))
        .expect("charter dir");
    write_file(
        &repo_root.join(".custom_handbook/feature_spec/FEATURE_SPEC.md"),
        b"custom spec",
    );

    let artifacts =
        CanonicalArtifacts::load_with_contract(repo_root, custom_layout_contract()).expect("load");

    assert_eq!(artifacts.system_root_status, SystemRootStatus::Ok);
    assert_eq!(
        artifacts.charter.identity.relative_path,
        ".custom_handbook/charter/CHARTER.md"
    );
    assert_eq!(
        artifacts.charter.identity.presence,
        ArtifactPresence::Missing
    );
    assert_eq!(
        artifacts.project_context.identity.relative_path,
        ".custom_handbook/project_context/PROJECT_CONTEXT.md"
    );
    assert_eq!(
        artifacts.feature_spec.identity.relative_path,
        ".custom_handbook/feature_spec/FEATURE_SPEC.md"
    );
    assert_eq!(
        artifacts.feature_spec.identity.presence,
        ArtifactPresence::PresentNonEmpty
    );
    assert!(
        artifacts.ingest_issues.iter().any(|issue| {
            issue.kind == ArtifactIngestIssueKind::CanonicalArtifactReadError
                && issue.canonical_repo_relative_path == ".custom_handbook/charter/CHARTER.md"
        }),
        "expected custom-layout read-error ingest issue, got: {:?}",
        artifacts.ingest_issues
    );
    assert!(artifacts
        .identities()
        .iter()
        .all(|identity| identity.relative_path.starts_with(".custom_handbook/")));
}

#[cfg(unix)]
#[test]
fn selected_repo_root_symlink_is_trusted_but_relative_artifact_symlinks_are_refused() {
    use std::os::unix::fs::symlink;

    let outer = tempfile::tempdir().expect("outer tempdir");
    let real_root = outer.path().join("real-root");
    write_file(
        &real_root.join(".handbook/charter/CHARTER.md"),
        b"canonical charter",
    );
    write_file(
        &real_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"canonical feature spec",
    );
    let selected_root = outer.path().join("selected-root");
    symlink(&real_root, &selected_root).expect("selected root symlink");

    let through_real_root = CanonicalArtifacts::load(&real_root).expect("real-root load");
    let through_selected_root =
        CanonicalArtifacts::load(&selected_root).expect("symlink-selected-root load");
    assert_eq!(through_selected_root, through_real_root);

    let malicious_root = outer.path().join("malicious-root");
    let outside_charter = outer.path().join("outside-charter");
    write_file(&outside_charter.join("CHARTER.md"), b"outside charter");
    std::fs::create_dir_all(malicious_root.join(".handbook")).expect("system root");
    symlink(&outside_charter, malicious_root.join(".handbook/charter"))
        .expect("relative component symlink");
    write_file(
        &malicious_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"canonical feature spec",
    );

    let artifacts = CanonicalArtifacts::load(&malicious_root).expect("malicious-root load");
    assert_eq!(
        artifacts.charter.identity.presence,
        ArtifactPresence::Missing
    );
    assert!(artifacts.ingest_issues.iter().any(|issue| {
        issue.kind == ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed
            && issue.canonical_repo_relative_path == ".handbook/charter/CHARTER.md"
    }));
}
