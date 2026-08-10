use handbook_engine::{
    ArtifactIngestError, ArtifactIngestIssueKind, ArtifactPresence, CanonicalArtifact,
    CanonicalArtifacts, CanonicalLayoutContract, SystemRootStatus,
};

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

fn selected_artifact<'a>(
    artifacts: &'a CanonicalArtifacts,
    instance_id: &str,
) -> &'a CanonicalArtifact {
    artifacts
        .artifacts
        .iter()
        .find(|artifact| artifact.identity.instance_id == instance_id)
        .expect("selected artifact")
}

fn write_required_artifacts(repo_root: &std::path::Path) {
    write_file(
        &repo_root.join(".handbook/project/charter.yaml"),
        b"selected charter bytes\n",
    );
    write_file(
        &repo_root.join(".handbook/project/context.yaml"),
        b"selected project context bytes\n",
    );
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

#[test]
fn descriptor_selected_collection_retains_exact_source_bytes() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let charter_bytes = b"charter-source:\r\n  exact: true\r\n";
    let project_context_bytes = b"project-context-source\n";
    let environment_context_bytes = b"environment-context-source\r\n";

    write_file(
        &repo_root.join(".handbook/project/charter.yaml"),
        charter_bytes,
    );
    write_file(
        &repo_root.join(".handbook/project/context.yaml"),
        project_context_bytes,
    );
    write_file(
        &repo_root.join(".handbook/project/environment.yaml"),
        environment_context_bytes,
    );
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"unadmitted fixed sibling",
    );

    let artifacts = CanonicalArtifacts::load(repo_root).expect("descriptor-selected load");
    let expected = [
        (
            "project_authority",
            "handbook.artifact-kind.project-authority@1.1.0",
            "Charter",
            ".handbook/project/charter.yaml",
            charter_bytes.as_slice(),
            "aa7ac428f52178fabe5b5edd4af8f257f82d39c3394d9f8b0edeafcaf145c6ed",
        ),
        (
            "project_context",
            "handbook.artifact-kind.project-context@1.1.0",
            "Project Context",
            ".handbook/project/context.yaml",
            project_context_bytes.as_slice(),
            "237b9009e40b089237d5d3b2761d080c863a6d27ec9b6309fcf2541a26ee8cac",
        ),
        (
            "environment_context",
            "handbook.artifact-kind.environment-context@1.1.0",
            "Environment Context",
            ".handbook/project/environment.yaml",
            environment_context_bytes.as_slice(),
            "69924ebee0eeb1ed678eaeb2ca7ab13e2b435c49431c297d02eb5b521faad957",
        ),
    ];

    assert_eq!(artifacts.artifacts.len(), expected.len());
    for (artifact, (instance_id, kind_ref, label, path, bytes, source_sha256)) in
        artifacts.artifacts.iter().zip(expected)
    {
        assert_eq!(artifact.identity.instance_id, instance_id);
        assert_eq!(artifact.identity.kind_ref, kind_ref);
        assert_eq!(artifact.identity.label, label);
        assert_eq!(artifact.identity.relative_path, path);
        assert_eq!(artifact.identity.byte_len, Some(bytes.len() as u64));
        assert_eq!(artifact.bytes.as_deref(), Some(bytes));
        assert_eq!(
            artifact.identity.content_sha256.as_deref(),
            Some(source_sha256)
        );
    }
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
    assert!(artifacts
        .artifacts
        .iter()
        .all(|artifact| artifact.identity.presence == ArtifactPresence::Missing));
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
fn selected_descriptor_namespace_establishes_partial_canonical_root() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir_all(dir.path().join(".handbook/project")).expect("mkdirs");

    let artifacts = CanonicalArtifacts::load(dir.path()).expect("load");
    assert_eq!(artifacts.system_root_status, SystemRootStatus::Ok);
    assert!(artifacts
        .artifacts
        .iter()
        .all(|artifact| artifact.identity.presence == ArtifactPresence::Missing));
}

#[test]
fn retired_project_context_namespace_does_not_establish_canonical_root() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir_all(dir.path().join(".handbook/project_context")).expect("mkdirs");

    let artifacts = CanonicalArtifacts::load(dir.path()).expect("load");
    assert_eq!(artifacts.system_root_status, SystemRootStatus::Missing);
}

#[test]
fn unadmitted_feature_spec_namespace_does_not_establish_canonical_root() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir_all(dir.path().join(".handbook/feature_spec")).expect("mkdirs");

    let artifacts = CanonicalArtifacts::load(dir.path()).expect("load");
    assert_eq!(artifacts.system_root_status, SystemRootStatus::Missing);
}

#[test]
fn required_artifact_missing_is_reported_as_presence_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_file(
        &dir.path().join(".handbook/project/context.yaml"),
        b"project context",
    );

    let artifacts = CanonicalArtifacts::load(dir.path()).expect("load");
    let charter = selected_artifact(&artifacts, "project_authority");
    assert_eq!(artifacts.system_root_status, SystemRootStatus::Ok);
    assert_eq!(charter.identity.presence, ArtifactPresence::Missing);
    assert!(charter.bytes.is_none());
}

#[test]
fn optional_missing_is_distinct_from_empty() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_required_artifacts(dir.path());

    let artifacts = CanonicalArtifacts::load(dir.path()).expect("load");
    let environment = selected_artifact(&artifacts, "environment_context");
    assert_eq!(environment.identity.presence, ArtifactPresence::Missing);
    assert!(environment.bytes.is_none());
    assert!(environment.identity.content_sha256.is_none());
}

#[test]
fn empty_means_exactly_zero_bytes() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_required_artifacts(dir.path());
    write_file(&dir.path().join(".handbook/project/environment.yaml"), b"");

    let artifacts = CanonicalArtifacts::load(dir.path()).expect("load");
    let environment = selected_artifact(&artifacts, "environment_context");
    assert_eq!(
        environment.identity.presence,
        ArtifactPresence::PresentEmpty
    );
    assert_eq!(environment.identity.byte_len, Some(0));
    assert_eq!(environment.bytes.as_deref(), Some(b"".as_slice()));
}

#[test]
fn whitespace_only_counts_as_non_empty() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_required_artifacts(dir.path());
    write_file(
        &dir.path().join(".handbook/project/environment.yaml"),
        b" \n\t",
    );

    let artifacts = CanonicalArtifacts::load(dir.path()).expect("load");
    let environment = selected_artifact(&artifacts, "environment_context");
    assert_eq!(
        environment.identity.presence,
        ArtifactPresence::PresentNonEmpty
    );
    assert_eq!(environment.identity.byte_len, Some(3));
    assert!(!environment.identity.matches_setup_starter_template);
}

#[test]
fn required_artifact_directory_is_recorded_as_read_error_and_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir_all(dir.path().join(".handbook/project/charter.yaml"))
        .expect("charter dir");
    write_file(
        &dir.path().join(".handbook/project/context.yaml"),
        b"project context",
    );

    let artifacts = CanonicalArtifacts::load(dir.path()).expect("load");
    let charter = selected_artifact(&artifacts, "project_authority");
    assert_eq!(charter.identity.presence, ArtifactPresence::Missing);
    assert!(charter.bytes.is_none());
    assert!(artifacts.ingest_issues.iter().any(|issue| {
        issue.kind == ArtifactIngestIssueKind::CanonicalArtifactReadError
            && issue.instance_id == "project_authority"
            && issue.canonical_repo_relative_path == ".handbook/project/charter.yaml"
            && issue.packet_required
    }));
}

#[test]
fn non_default_layout_contract_cannot_override_descriptor_paths() {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir_all(dir.path().join(".custom_handbook")).expect("custom root");
    std::fs::create_dir_all(dir.path().join(".handbook/project/charter.yaml"))
        .expect("charter dir");
    write_file(
        &dir.path().join(".handbook/project/context.yaml"),
        b"project context",
    );

    let artifacts =
        CanonicalArtifacts::load_with_contract(dir.path(), custom_layout_contract()).expect("load");
    assert_eq!(artifacts.system_root_status, SystemRootStatus::Ok);
    assert_eq!(
        selected_artifact(&artifacts, "project_authority")
            .identity
            .relative_path,
        ".handbook/project/charter.yaml"
    );
    assert!(artifacts
        .identities()
        .iter()
        .all(|identity| identity.relative_path.starts_with(".handbook/project/")));
    assert!(artifacts.ingest_issues.iter().any(|issue| {
        issue.kind == ArtifactIngestIssueKind::CanonicalArtifactReadError
            && issue.canonical_repo_relative_path == ".handbook/project/charter.yaml"
    }));
}

#[cfg(unix)]
#[test]
fn selected_repo_root_symlink_is_trusted_but_relative_artifact_symlinks_are_refused() {
    use std::os::unix::fs::symlink;

    let outer = tempfile::tempdir().expect("outer tempdir");
    let real_root = outer.path().join("real-root");
    write_required_artifacts(&real_root);
    let selected_root = outer.path().join("selected-root");
    symlink(&real_root, &selected_root).expect("selected root symlink");

    let through_real_root = CanonicalArtifacts::load(&real_root).expect("real-root load");
    let through_selected_root =
        CanonicalArtifacts::load(&selected_root).expect("symlink-selected-root load");
    assert_eq!(through_selected_root, through_real_root);

    let malicious_root = outer.path().join("malicious-root");
    let outside_charter = outer.path().join("outside-charter.yaml");
    write_file(&outside_charter, b"outside charter");
    std::fs::create_dir_all(malicious_root.join(".handbook/project")).expect("system root");
    symlink(
        &outside_charter,
        malicious_root.join(".handbook/project/charter.yaml"),
    )
    .expect("artifact symlink");
    write_file(
        &malicious_root.join(".handbook/project/context.yaml"),
        b"project context",
    );

    let artifacts = CanonicalArtifacts::load(&malicious_root).expect("malicious-root load");
    let charter = selected_artifact(&artifacts, "project_authority");
    assert_eq!(charter.identity.presence, ArtifactPresence::Missing);
    assert!(artifacts.ingest_issues.iter().any(|issue| {
        issue.kind == ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed
            && issue.canonical_repo_relative_path == ".handbook/project/charter.yaml"
    }));
}
