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

#[test]
fn descriptor_paths_use_the_existing_exact_normalized_repo_relative_grammar() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let condition=ProjectConditionDefinition::load(root,"definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml").unwrap();
    for invalid_path in [
        " .handbook/project/context.yaml",
        ".handbook/./project/context.yaml",
        ".handbook//project/context.yaml",
        "../context.yaml",
        "https://example.invalid/context.yaml",
        "file:context.yaml",
        "data:context.yaml",
        "C:/context.yaml",
    ] {
        let mut values = shipped_root_artifact_instance_values();
        values[1]["canonical_path"] = serde_json::json!(invalid_path);
        assert!(
            ArtifactInstanceRegistry::resolve(&values, &kinds(), &[&condition]).is_err(),
            "{invalid_path}"
        );
    }

    let mut values = shipped_root_artifact_instance_values();
    values[1]["canonical_path"] = serde_json::json!(".handbook/project/contexte-é.yaml");
    ArtifactInstanceRegistry::resolve(&values, &kinds(), &[&condition]).unwrap();
}

#[test]
fn descriptor_public_api_preserves_typed_metadata_without_inventing_a_label_limit() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let condition = ProjectConditionDefinition::load(
        root,
        "definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml",
    )
    .unwrap();
    let mut values = shipped_root_artifact_instance_values();
    values[1]["label"] = serde_json::json!("é".repeat(65));
    values[1]["depends_on"] = serde_json::json!([{
        "target_kind":"instance", "target_ref":"environment_context",
        "target_contract_ref":null, "cardinality":"exactly_one"
    }]);
    let registry = ArtifactInstanceRegistry::resolve(&values, &kinds(), &[&condition]).unwrap();
    let descriptor = registry
        .instance(&SymbolicId::parse("project_context").unwrap())
        .unwrap();

    assert_eq!(descriptor.label().chars().count(), 65);
    assert_eq!(descriptor.requiredness().mode(), RequirednessMode::Always);
    assert_eq!(descriptor.requiredness().condition_ref(), None);
    assert_eq!(descriptor.dependencies().len(), 1);
    assert_eq!(
        descriptor.dependencies()[0].target_kind(),
        DependencyTargetKind::Instance
    );
    assert_eq!(
        descriptor.dependencies()[0].target_ref().as_str(),
        "environment_context"
    );
    assert_eq!(descriptor.dependencies()[0].target_contract_ref(), None);
    assert_eq!(
        descriptor.dependencies()[0].cardinality(),
        DependencyCardinality::ExactlyOne
    );
    assert_eq!(descriptor.lifecycle_policy_ref(), None);
    assert_eq!(descriptor.intake_definition_ref(), None);
    assert!(descriptor.renderer_definition_refs().is_empty());
    assert!(descriptor.projection_definition_refs().is_empty());
    assert!(descriptor.validation_overlay_refs().is_empty());
    assert!(descriptor.extensions().is_empty());
}

#[test]
fn non_charter_descriptor_accepts_only_a_typed_generic_intake_reference() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let condition = ProjectConditionDefinition::load(
        root,
        "definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml",
    )
    .unwrap();
    let mut values = shipped_root_artifact_instance_values();
    values[1]["intake_definition_ref"] = serde_json::json!("example.intake.registry-brief@1.0.0");

    let registry = ArtifactInstanceRegistry::resolve(&values, &kinds(), &[&condition]).unwrap();
    let descriptor = registry
        .instance(&SymbolicId::parse("project_context").unwrap())
        .unwrap();

    assert_eq!(
        descriptor
            .intake_definition_ref()
            .map(ExactDefinitionRef::as_str),
        Some("example.intake.registry-brief@1.0.0")
    );
    assert_eq!(descriptor.lifecycle_policy_ref(), None);
    assert!(descriptor.renderer_definition_refs().is_empty());
    assert!(descriptor.projection_definition_refs().is_empty());
    assert!(descriptor.validation_overlay_refs().is_empty());
}

#[test]
fn every_explicitly_nullable_descriptor_member_must_be_present() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let condition = ProjectConditionDefinition::load(
        root,
        "definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml",
    )
    .unwrap();
    for field in ["role_ref", "lifecycle_policy_ref", "intake_definition_ref"] {
        let mut values = shipped_root_artifact_instance_values();
        values[1].as_object_mut().unwrap().remove(field);
        let failure =
            ArtifactInstanceRegistry::resolve(&values, &kinds(), &[&condition]).unwrap_err();
        assert_eq!(
            failure.kind(),
            RegistryLoadErrorKind::SyntaxError,
            "{field}"
        );
        assert_eq!(failure.location(), None, "{field}");
    }

    let mut requiredness = shipped_root_artifact_instance_values();
    requiredness[1]["requiredness"]
        .as_object_mut()
        .unwrap()
        .remove("condition_ref");
    let failure =
        ArtifactInstanceRegistry::resolve(&requiredness, &kinds(), &[&condition]).unwrap_err();
    assert_eq!(failure.kind(), RegistryLoadErrorKind::SyntaxError);
    assert_eq!(failure.location(), None);

    let mut dependency = shipped_root_artifact_instance_values();
    dependency[1]["depends_on"] = serde_json::json!([{
        "target_kind":"instance", "target_ref":"environment_context",
        "target_contract_ref":null, "cardinality":"exactly_one"
    }]);
    dependency[1]["depends_on"][0]
        .as_object_mut()
        .unwrap()
        .remove("target_contract_ref");
    let failure =
        ArtifactInstanceRegistry::resolve(&dependency, &kinds(), &[&condition]).unwrap_err();
    assert_eq!(failure.kind(), RegistryLoadErrorKind::SyntaxError);
    assert_eq!(failure.location(), None);
}

#[test]
fn descriptor_record_identity_and_later_owned_selection_have_distinct_errors() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let condition = ProjectConditionDefinition::load(
        root,
        "definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml",
    )
    .unwrap();
    for (field, value) in [
        ("schema_id", serde_json::json!("example.wrong-record")),
        ("schema_version", serde_json::json!("2.0")),
    ] {
        let mut values = shipped_root_artifact_instance_values();
        values[1][field] = value;
        let failure =
            ArtifactInstanceRegistry::resolve(&values, &kinds(), &[&condition]).unwrap_err();
        assert_eq!(failure.kind(), RegistryLoadErrorKind::UnsupportedRecord);
        assert_eq!(
            failure.location(),
            Some(format!("artifact_instances/1/{field}").as_str())
        );
    }

    let mut later_owned = shipped_root_artifact_instance_values();
    later_owned[1]["lifecycle_policy_ref"] = serde_json::json!("handbook.lifecycle.example@1.0.0");
    let failure =
        ArtifactInstanceRegistry::resolve(&later_owned, &kinds(), &[&condition]).unwrap_err();
    assert_eq!(failure.kind(), RegistryLoadErrorKind::UnsupportedDependency);
    assert_eq!(
        failure.location(),
        Some("artifact_instances/1/lifecycle_policy_ref")
    );
}

#[test]
fn dependency_namespace_duplicates_cardinality_and_cycles_fail_closed() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let condition = ProjectConditionDefinition::load(
        root,
        "definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml",
    )
    .unwrap();
    let mut one_way = shipped_root_artifact_instance_values();
    one_way[1]["depends_on"] = serde_json::json!([{
        "target_kind":"instance", "target_ref":"environment_context",
        "target_contract_ref":null, "cardinality":"exactly_one"
    }]);
    let with_dependency =
        ArtifactInstanceRegistry::resolve(&one_way, &kinds(), &[&condition]).unwrap();
    let without_dependency = ArtifactInstanceRegistry::resolve(
        &shipped_root_artifact_instance_values(),
        &kinds(),
        &[&condition],
    )
    .unwrap();
    assert_ne!(
        with_dependency.fingerprint(),
        without_dependency.fingerprint()
    );

    let mut cycle = one_way.clone();
    cycle[2]["depends_on"] = serde_json::json!([{
        "target_kind":"instance", "target_ref":"project_context",
        "target_contract_ref":null, "cardinality":"exactly_one"
    }]);
    assert!(ArtifactInstanceRegistry::resolve(&cycle, &kinds(), &[&condition]).is_err());

    let mut duplicate = one_way;
    let dependency = duplicate[1]["depends_on"][0].clone();
    duplicate[1]["depends_on"] = serde_json::json!([dependency.clone(), dependency]);
    assert!(ArtifactInstanceRegistry::resolve(&duplicate, &kinds(), &[&condition]).is_err());

    let mut wrong_cardinality = shipped_root_artifact_instance_values();
    wrong_cardinality[1]["depends_on"] = serde_json::json!([{
        "target_kind":"instance", "target_ref":"environment_context",
        "target_contract_ref":null, "cardinality":"at_least_one"
    }]);
    assert!(
        ArtifactInstanceRegistry::resolve(&wrong_cardinality, &kinds(), &[&condition]).is_err()
    );
}

#[test]
fn descriptor_requiredness_dependency_and_root_errors_have_exact_kinds_and_locations() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let condition = ProjectConditionDefinition::load(
        root,
        "definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml",
    )
    .unwrap();
    let error = |values: Vec<serde_json::Value>| {
        ArtifactInstanceRegistry::resolve(&values, &kinds(), &[&condition]).unwrap_err()
    };

    let mut requiredness = shipped_root_artifact_instance_values();
    requiredness[1]["requiredness"]["condition_ref"] =
        serde_json::json!("handbook.condition.project.managed-operational-surface@1.0.0");
    let failure = error(requiredness);
    assert_eq!(failure.kind(), RegistryLoadErrorKind::InvalidRequiredness);
    assert_eq!(
        failure.location(),
        Some("artifact_instances/1/requiredness")
    );

    let mut zero_root = shipped_root_artifact_instance_values();
    zero_root[0]["capability_refs"] = serde_json::json!([]);
    let failure = error(zero_root);
    assert_eq!(
        failure.kind(),
        RegistryLoadErrorKind::InvalidConstitutionalRoot
    );
    assert_eq!(failure.location(), Some("artifact_instances"));

    for (field, value, kind, location) in [
        (
            "target_kind",
            serde_json::json!("ambient"),
            RegistryLoadErrorKind::InvalidDependencyNamespace,
            "artifact_instances/1/depends_on/0/target_kind",
        ),
        (
            "target_ref",
            serde_json::json!("bad-target"),
            RegistryLoadErrorKind::InvalidDependencyTarget,
            "artifact_instances/1/depends_on/0/target_ref",
        ),
        (
            "cardinality",
            serde_json::json!("ambient"),
            RegistryLoadErrorKind::InvalidDependencyCardinality,
            "artifact_instances/1/depends_on/0/cardinality",
        ),
    ] {
        let mut values = shipped_root_artifact_instance_values();
        values[1]["depends_on"] = serde_json::json!([{
            "target_kind":"instance", "target_ref":"environment_context",
            "target_contract_ref":null, "cardinality":"exactly_one"
        }]);
        values[1]["depends_on"][0][field] = value;
        let failure = error(values);
        assert_eq!(failure.kind(), kind, "{field}");
        assert_eq!(failure.location(), Some(location), "{field}");
    }

    let mut contract = shipped_root_artifact_instance_values();
    contract[1]["depends_on"] = serde_json::json!([{
        "target_kind":"instance", "target_ref":"environment_context",
        "target_contract_ref":"handbook.capabilities.constitutional-root@1.0.0",
        "cardinality":"exactly_one"
    }]);
    let failure = error(contract);
    assert_eq!(
        failure.kind(),
        RegistryLoadErrorKind::InvalidDependencyContract
    );
    assert_eq!(
        failure.location(),
        Some("artifact_instances/1/depends_on/0/target_contract_ref")
    );

    let mut cardinality = shipped_root_artifact_instance_values();
    cardinality[1]["depends_on"] = serde_json::json!([{
        "target_kind":"instance", "target_ref":"environment_context",
        "target_contract_ref":null, "cardinality":"at_least_one"
    }]);
    let failure = error(cardinality);
    assert_eq!(
        failure.kind(),
        RegistryLoadErrorKind::InvalidDependencyCardinality
    );
    assert_eq!(
        failure.location(),
        Some("artifact_instances/1/depends_on/0/cardinality")
    );

    let mut provider = shipped_root_artifact_instance_values();
    provider[1]["depends_on"] = serde_json::json!([{
        "target_kind":"capability", "target_ref":"constitutional_root",
        "target_contract_ref":"example.capability.wrong@1.0.0",
        "cardinality":"exactly_one"
    }]);
    let failure = error(provider);
    assert_eq!(
        failure.kind(),
        RegistryLoadErrorKind::InvalidDependencyContract
    );
    assert_eq!(
        failure.location(),
        Some("artifact_instances/1/depends_on/0/target_contract_ref")
    );

    let mut wrong_capability = shipped_root_artifact_instance_values();
    wrong_capability[1]["depends_on"] = serde_json::json!([{
        "target_kind":"capability", "target_ref":"constitutional_authority",
        "target_contract_ref":"handbook.capabilities.constitutional-root@1.0.0",
        "cardinality":"exactly_one"
    }]);
    let failure = error(wrong_capability);
    assert_eq!(
        failure.kind(),
        RegistryLoadErrorKind::InvalidDependencyContract
    );
    assert_eq!(
        failure.location(),
        Some("artifact_instances/1/depends_on/0/target_contract_ref")
    );

    let mut exact_version_mismatch = shipped_root_artifact_instance_values();
    exact_version_mismatch[1]["depends_on"] = serde_json::json!([{
        "target_kind":"capability", "target_ref":"constitutional_root",
        "target_contract_ref":"handbook.capabilities.constitutional-root@1.0.1",
        "cardinality":"exactly_one"
    }]);
    let failure = error(exact_version_mismatch);
    assert_eq!(
        failure.kind(),
        RegistryLoadErrorKind::InvalidDependencyContract
    );
    assert_eq!(
        failure.location(),
        Some("artifact_instances/1/depends_on/0/target_contract_ref")
    );

    let mut cycle = shipped_root_artifact_instance_values();
    cycle[1]["depends_on"] = serde_json::json!([{
        "target_kind":"instance", "target_ref":"environment_context",
        "target_contract_ref":null, "cardinality":"exactly_one"
    }]);
    cycle[2]["depends_on"] = serde_json::json!([{
        "target_kind":"instance", "target_ref":"project_context",
        "target_contract_ref":null, "cardinality":"exactly_one"
    }]);
    let failure = error(cycle);
    assert_eq!(failure.kind(), RegistryLoadErrorKind::DependencyCycle);
    assert_eq!(failure.location(), Some("artifact_instances/depends_on"));
}

#[test]
fn public_resolver_refuses_duplicate_condition_identities() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let condition = ProjectConditionDefinition::load(
        root,
        "definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml",
    )
    .unwrap();
    let values = shipped_root_artifact_instance_values();

    for conditions in [vec![&condition, &condition], vec![&condition, &condition]] {
        let error = ArtifactInstanceRegistry::resolve(&values, &kinds(), &conditions).unwrap_err();
        assert_eq!(error.kind(), RegistryLoadErrorKind::DuplicateIdentity);
    }
}
