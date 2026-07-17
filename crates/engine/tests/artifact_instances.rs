use handbook_engine::*;
use std::collections::BTreeSet;
use std::path::Path;
fn path(c: &str, id: &str, s: &str) -> String {
    format!("definitions/{c}/{id}/1.0.0{s}")
}
fn kinds() -> ArtifactKindRegistry {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let names = [
        "project-authority",
        "project-context",
        "environment-context",
    ];
    let schemas = names
        .map(|n| {
            path(
                "schemas",
                &format!("handbook.schemas.artifacts.{n}"),
                ".entry.yaml",
            )
        })
        .to_vec();
    let ks = names
        .map(|n| {
            path(
                "artifact-kinds",
                &format!("handbook.artifact-kind.{n}"),
                ".yaml",
            )
        })
        .to_vec();
    load_artifact_kind_registry(
        root,
        ArtifactKindRegistryLoadRequest::new(
            ExactDefinitionRef::parse("handbook.roles.core@1.1.0").unwrap(),
            schemas,
            vec!["definitions/schemas".into()],
            ks,
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
        ),
    )
    .unwrap()
}
#[test]
fn exact_three_descriptors_resolve_in_both_orders() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let condition=ProjectConditionDefinition::load(root,"definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml").unwrap();
    let values = shipped_root_artifact_instance_values();
    let forward = ArtifactInstanceRegistry::resolve(&values, &kinds(), &[&condition]).unwrap();
    let mut reversed = values;
    reversed.reverse();
    let reverse = ArtifactInstanceRegistry::resolve(&reversed, &kinds(), &[&condition]).unwrap();
    assert_eq!(forward.fingerprint(), reverse.fingerprint());
    assert_eq!(
        forward.ids().into_iter().collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "environment_context",
            "project_authority",
            "project_context"
        ])
    );
}
#[test]
fn descriptor_requiredness_path_role_and_unique_root_fail_closed() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let condition=ProjectConditionDefinition::load(root,"definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml").unwrap();
    for (pointer, value) in [
        ("/0/requiredness/mode", serde_json::json!("optional")),
        (
            "/1/canonical_path",
            serde_json::json!(".handbook/project/charter.yaml"),
        ),
        ("/1/role_ref", serde_json::json!("constitutional_authority")),
        ("/0/capability_refs", serde_json::json!([])),
    ] {
        let mut values = shipped_root_artifact_instance_values();
        let (index, local) = if let Some(local) = pointer.strip_prefix("/0") {
            (0, local)
        } else {
            (1, pointer.strip_prefix("/1").unwrap())
        };
        *values[index].pointer_mut(local).unwrap() = value;
        assert!(
            ArtifactInstanceRegistry::resolve(&values, &kinds(), &[&condition]).is_err(),
            "{pointer}"
        );
    }
}
