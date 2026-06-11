use handbook_engine::{
    ArtifactManifest, CanonicalArtifactKind, CanonicalArtifacts, InheritedDependency,
    ManifestInputs, OverrideTarget, OverrideWithRationale, C03_SCHEMA_VERSION,
    MANIFEST_GENERATION_VERSION,
};

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent dirs");
    }
    std::fs::write(path, contents).expect("write file");
}

fn make_repo_with_required_system_artifacts() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".handbook/charter/CHARTER.md"), b"charter");
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    dir
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};

    format!("{:x}", Sha256::digest(bytes))
}

#[test]
fn manifest_artifacts_are_in_contract_order() {
    let dir = make_repo_with_required_system_artifacts();
    let manifest =
        ArtifactManifest::generate(dir.path(), ManifestInputs::default()).expect("manifest");

    let kinds: Vec<CanonicalArtifactKind> = manifest.artifacts.iter().map(|a| a.kind).collect();
    assert_eq!(
        kinds,
        vec![
            CanonicalArtifactKind::Charter,
            CanonicalArtifactKind::ProjectContext,
            CanonicalArtifactKind::EnvironmentInventory,
            CanonicalArtifactKind::FeatureSpec,
        ]
    );
}

#[test]
fn manifest_versions_match_freshness_versions() {
    let dir = make_repo_with_required_system_artifacts();
    let manifest =
        ArtifactManifest::generate(dir.path(), ManifestInputs::default()).expect("manifest");

    assert_eq!(manifest.version.schema.contract_id, "C-03");
    assert_eq!(manifest.version.schema.version, C03_SCHEMA_VERSION);
    assert_eq!(manifest.version.generation, MANIFEST_GENERATION_VERSION);

    assert_eq!(manifest.freshness.schema_version, C03_SCHEMA_VERSION);
    assert_eq!(
        manifest.freshness.manifest_generation_version,
        MANIFEST_GENERATION_VERSION
    );
}

#[test]
fn dependencies_are_sorted_deterministically_in_freshness_truth() {
    let dir = make_repo_with_required_system_artifacts();

    let inputs = ManifestInputs {
        inherited_dependencies: vec![
            InheritedDependency {
                id: "b".to_string(),
                version: Some("2".to_string()),
                content_sha256: None,
            },
            InheritedDependency {
                id: "a".to_string(),
                version: Some("9".to_string()),
                content_sha256: None,
            },
        ],
        overrides: Vec::new(),
    };

    let manifest = ArtifactManifest::generate(dir.path(), inputs).expect("manifest");

    let ids: Vec<&str> = manifest
        .freshness
        .inherited_dependencies
        .iter()
        .map(|d| d.id.as_str())
        .collect();
    assert_eq!(ids, vec!["a", "b"]);
}

#[test]
fn freshness_issues_have_deterministic_order() {
    let dir = make_repo_with_required_system_artifacts();

    let inputs = ManifestInputs {
        inherited_dependencies: Vec::new(),
        overrides: vec![
            OverrideWithRationale {
                target: OverrideTarget::CanonicalArtifact(CanonicalArtifactKind::FeatureSpec),
                rationale: "b".to_string(),
            },
            OverrideWithRationale {
                target: OverrideTarget::CanonicalArtifact(CanonicalArtifactKind::Charter),
                rationale: "a".to_string(),
            },
        ],
    };

    let manifest = ArtifactManifest::generate(dir.path(), inputs).expect("manifest");
    assert_eq!(manifest.freshness.issues.len(), 2);

    let details: Vec<&str> = manifest
        .freshness
        .issues
        .iter()
        .map(|issue| issue.detail.as_str())
        .collect();
    assert!(details[0].contains("Charter"));
    assert!(details[0].ends_with(": a"));
    assert!(details[1].contains("FeatureSpec"));
    assert!(details[1].ends_with(": b"));
}

#[test]
fn manifest_from_snapshot_keeps_pre_mutation_identity() {
    let dir = make_repo_with_required_system_artifacts();
    let root = dir.path();
    let original_bytes = b"charter".to_vec();

    let artifacts = CanonicalArtifacts::load(root).expect("artifacts");
    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        b"charter changed after snapshot",
    );

    let manifest =
        ArtifactManifest::from_canonical_artifacts(&artifacts, ManifestInputs::default());
    let charter = manifest
        .artifacts
        .iter()
        .find(|artifact| artifact.kind == CanonicalArtifactKind::Charter)
        .expect("charter identity");

    assert_eq!(charter.byte_len, Some(original_bytes.len() as u64));
    assert_eq!(
        charter.content_sha256.as_deref(),
        Some(sha256_hex(&original_bytes).as_str())
    );
}
