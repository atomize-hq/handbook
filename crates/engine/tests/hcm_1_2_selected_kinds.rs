use handbook_engine::{
    load_artifact_kind_registry, ArtifactKindRegistryLoadRequest, ExactDefinitionRef,
};
use std::collections::BTreeSet;
use std::path::Path;
const ROLE: &str = "handbook.roles.core@1.1.0";
fn path(class: &str, id: &str, suffix: &str) -> String {
    format!("definitions/{class}/{id}/1.0.0{suffix}")
}
fn request(mut kinds: Vec<String>) -> ArtifactKindRegistryLoadRequest {
    let schemas = [
        "project-authority",
        "project-context",
        "environment-context",
    ]
    .into_iter()
    .map(|n| {
        path(
            "schemas",
            &format!("handbook.schemas.artifacts.{n}"),
            ".entry.yaml",
        )
    })
    .collect();
    ArtifactKindRegistryLoadRequest::new(
        ExactDefinitionRef::parse(ROLE).unwrap(),
        schemas,
        vec!["definitions/schemas".into()],
        {
            kinds.shrink_to_fit();
            kinds
        },
    )
    .with_semantic_sources(
        vec![path(
            "semantic-capabilities",
            "handbook.capabilities.constitutional-root",
            ".yaml",
        )],
        vec![path(
            "semantic-validators",
            "handbook.semantic-validation.constitutional-root",
            ".yaml",
        )],
    )
}
fn kind_paths() -> Vec<String> {
    [
        "project-authority",
        "project-context",
        "environment-context",
    ]
    .into_iter()
    .map(|n| {
        path(
            "artifact-kinds",
            &format!("handbook.artifact-kind.{n}"),
            ".yaml",
        )
    })
    .collect()
}
#[test]
fn selected_kind_set_is_exact_permutation_stable_and_only_authority_has_capability() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let forward = load_artifact_kind_registry(root, request(kind_paths())).unwrap();
    let mut reverse = kind_paths();
    reverse.reverse();
    let reverse = load_artifact_kind_registry(root, request(reverse)).unwrap();
    let expected = BTreeSet::from([
        "handbook.artifact-kind.environment-context@1.0.0".to_string(),
        "handbook.artifact-kind.project-authority@1.0.0".to_string(),
        "handbook.artifact-kind.project-context@1.0.0".to_string(),
    ]);
    let actual = forward
        .kind_refs()
        .into_iter()
        .map(|r| r.as_str().to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
    assert_eq!(forward.fingerprint(), reverse.fingerprint());
    for r in forward.kind_refs() {
        let kind = forward.kind(&r).unwrap();
        if r.as_str().contains("project-authority") {
            let cap = kind
                .semantic_capabilities()
                .keys()
                .map(|x| x.as_str())
                .collect::<Vec<_>>();
            assert_eq!(cap, ["constitutional_root"]);
            assert_eq!(kind.supported_role_refs(), ["constitutional_authority"]);
        } else {
            assert!(kind.semantic_capabilities().is_empty());
        }
    }
}
