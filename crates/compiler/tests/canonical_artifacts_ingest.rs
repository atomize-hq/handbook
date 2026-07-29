use handbook_engine::{
    setup_starter_template_bytes, ArtifactIngestIssueKind, ArtifactPresence, CanonicalArtifactKind,
    CanonicalArtifacts, SystemRootStatus,
};

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
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
fn required_artifact_missing_is_reported_as_presence_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    std::fs::create_dir_all(repo_root.join(".handbook/charter")).expect("mkdirs");
    std::fs::create_dir_all(repo_root.join(".handbook/feature_spec")).expect("mkdirs");

    // Only create FEATURE_SPEC; omit required CHARTER.
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
fn optional_artifact_directory_is_recorded_as_read_error_and_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(&repo_root.join(".handbook/charter/CHARTER.md"), b"charter");
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"spec",
    );
    std::fs::create_dir_all(repo_root.join(".handbook/project_context/PROJECT_CONTEXT.md"))
        .expect("project_context dir");

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(artifacts.system_root_status, SystemRootStatus::Ok);
    assert_eq!(
        artifacts.project_context.identity.presence,
        ArtifactPresence::Missing
    );
    assert!(artifacts.project_context.bytes.is_none());
    assert!(
        artifacts.ingest_issues.iter().any(|issue| {
            issue.kind == ArtifactIngestIssueKind::CanonicalArtifactReadError
                && issue.canonical_repo_relative_path
                    == ".handbook/project_context/PROJECT_CONTEXT.md"
        }),
        "expected read-error ingest issue, got: {:?}",
        artifacts.ingest_issues
    );
}

#[cfg(unix)]
#[test]
fn system_root_symlink_is_not_followed_and_is_reported() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    let real_system = repo_root.join("real_system");
    std::fs::create_dir_all(real_system.join("charter")).expect("mkdirs");
    std::fs::create_dir_all(real_system.join("feature_spec")).expect("mkdirs");
    write_file(&real_system.join("charter/CHARTER.md"), b"charter");
    write_file(&real_system.join("feature_spec/FEATURE_SPEC.md"), b"spec");

    symlink(&real_system, repo_root.join(".handbook")).expect("symlink");

    let artifacts = CanonicalArtifacts::load(repo_root).expect("load");
    assert_eq!(
        artifacts.system_root_status,
        SystemRootStatus::SymlinkNotAllowed
    );
    assert_eq!(
        artifacts.charter.identity.presence,
        ArtifactPresence::Missing
    );
    assert_eq!(
        artifacts.feature_spec.identity.presence,
        ArtifactPresence::Missing
    );
}

#[cfg(unix)]
#[test]
fn canonical_artifact_symlink_is_not_followed_and_is_recorded_as_ingest_issue() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    std::fs::create_dir_all(repo_root.join(".handbook/charter")).expect("mkdirs");
    std::fs::create_dir_all(repo_root.join(".handbook/feature_spec")).expect("mkdirs");

    let real = repo_root.join("real_charter.md");
    write_file(&real, b"charter");
    symlink(&real, repo_root.join(".handbook/charter/CHARTER.md")).expect("symlink charter");
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
    assert!(
        artifacts.ingest_issues.iter().any(|issue| {
            issue.kind == ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed
                && issue.canonical_repo_relative_path == ".handbook/charter/CHARTER.md"
        }),
        "expected symlink ingest issue, got: {:?}",
        artifacts.ingest_issues
    );
}
