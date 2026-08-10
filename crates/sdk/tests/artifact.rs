use handbook_engine::RepositoryInvocationIdentityServiceV1;
use handbook_sdk::artifact::{
    ArtifactDocument, ArtifactReadRequest, ArtifactSdk, ArtifactSelector,
};

const KIND_REF: &str = "example.artifact-kind.registry-brief@1.0.0";
const INSTANCE_ID: &str = "registry_brief";

fn copy_tree(source: &std::path::Path, target: &std::path::Path) {
    std::fs::create_dir_all(target).expect("create target directory");
    for entry in std::fs::read_dir(source).expect("read source directory") {
        let entry = entry.expect("directory entry");
        let target_path = target.join(entry.file_name());
        if entry.path().is_dir() {
            copy_tree(&entry.path(), &target_path);
        } else {
            std::fs::copy(entry.path(), target_path).expect("copy fixture file");
        }
    }
}

#[test]
fn artifact_sdk_reads_a_closed_document_without_exposing_engine_json() {
    let repo = tempfile::tempdir().expect("temporary repository");
    copy_tree(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../engine/tests/fixtures/hcm_2_3_generic_custom_kind")
            .as_path(),
        repo.path(),
    );
    RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .expect("initialize repository identity");

    let selector = ArtifactSelector::new(KIND_REF, INSTANCE_ID);
    let result = ArtifactSdk::open(repo.path())
        .read(ArtifactReadRequest { selector })
        .expect("read selected artifact");

    assert_eq!(result.operation_id, "artifact.read");
    assert_eq!(
        result.content,
        ArtifactDocument::Object(std::collections::BTreeMap::from([
            (
                "summary".to_owned(),
                ArtifactDocument::String("A reusable registry brief".to_owned()),
            ),
            (
                "title".to_owned(),
                ArtifactDocument::String("Registry Brief Proof".to_owned()),
            ),
        ])),
    );
}

#[test]
fn artifact_public_api_does_not_publish_json_values_or_renderers() {
    let source = include_str!("../src/artifact.rs");
    let public_lines = source
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .collect::<Vec<_>>();

    assert!(
        public_lines
            .iter()
            .all(|line| !line.contains("serde_json::Value") && !line.contains(" Value")),
        "artifact public API exposed a JSON value: {public_lines:#?}"
    );
    assert!(
        public_lines.iter().all(|line| !line.contains("render")),
        "artifact public API exposed a renderer: {public_lines:#?}"
    );
}
