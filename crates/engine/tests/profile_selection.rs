use handbook_engine::*;
use std::collections::BTreeSet;
use std::path::Path;

const HCM_3_1_VOCABULARY_BYTES: &[u8] = include_bytes!(
    "fixtures/hcm_3_1_vocabulary_resolution/definitions/vocabularies/example.vocabulary.hcm-3-1/1.0.0.yaml"
);
fn r(value: &str) -> ExactDefinitionRef {
    ExactDefinitionRef::parse(value).unwrap()
}
fn builtin(value: &str) -> DefinitionSourceBinding {
    let x = r(value);
    DefinitionSourceBinding {
        definition_ref: x.clone(),
        source: DefinitionSource::BuiltIn(x),
    }
}
fn request(reverse: bool) -> ProfileSelectionRequest {
    let names = [
        "project-authority",
        "project-context",
        "environment-context",
        "work-specification",
        "decision-record",
        "risk-record",
    ];
    let mut schemas = names
        .map(|n| builtin(&format!("handbook.schemas.artifacts.{n}@1.0.0")))
        .to_vec();
    let mut kinds = names
        .map(|n| builtin(&format!("handbook.artifact-kind.{n}@1.0.0")))
        .to_vec();
    if reverse {
        schemas.reverse();
        kinds.reverse();
    }
    ProfileSelectionRequest {
        selected_profile_ref: r("example.profile.root@1.0.0"),
        profile_sources: vec![DefinitionSourceBinding {
            definition_ref: r("example.profile.root@1.0.0"),
            source: DefinitionSource::RepositoryPath(
                "tests/fixtures/hcm_1_2_repository_profile/root.yaml".into(),
            ),
        }],
        stable_role_registry_sources: vec![builtin("handbook.roles.core@1.1.0")],
        schema_entry_sources: schemas,
        artifact_kind_sources: kinds,
        semantic_capability_sources: vec![builtin(
            "handbook.capabilities.constitutional-root@1.0.0",
        )],
        semantic_validator_sources: vec![builtin(
            "handbook.semantic-validation.constitutional-root@1.0.0",
        )],
        project_condition_sources: vec![builtin(
            "handbook.condition.project.managed-operational-surface@1.0.0",
        )],
        vocabulary_sources: vec![builtin("handbook.vocabulary.shipped-root@1.0.0")],
        context_resolution_sources: vec![builtin("handbook.context-resolution.shipped-root@1.0.0")],
        context_resolution_policy_sources: vec![
            builtin("handbook.mutation-matcher.core@1.0.0"),
            builtin("handbook.resolution-escalation.core@1.0.0"),
            builtin("handbook.memory-promotion.core@1.0.0"),
        ],
        intake_definition_sources: vec![],
        allowed_schema_roots: vec!["definitions/schemas".into()],
    }
}

fn shipped_request(repository: bool, reverse_sources: bool) -> ProfileSelectionRequest {
    let mut value = request(reverse_sources);
    let shipped = builtin("handbook.profile.shipped-root@1.0.0");
    if repository {
        value.selected_profile_ref = r("example.profile.repository@1.0.0");
        value.profile_sources = vec![
            shipped,
            DefinitionSourceBinding {
                definition_ref: r("example.profile.repository@1.0.0"),
                source: DefinitionSource::RepositoryPath(
                    "tests/fixtures/hcm_1_2_repository_profile/repository.yaml".into(),
                ),
            },
        ];
        if reverse_sources {
            value.profile_sources.reverse();
        }
    } else {
        value.selected_profile_ref = r("handbook.profile.shipped-root@1.0.0");
        value.profile_sources = vec![shipped];
    }
    value
}
#[test]
fn exact_selection_recomputes_the_complete_typed_closure() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let forward = resolve_profile_selection(root, request(false)).unwrap();
    let reverse = resolve_profile_selection(root, request(true)).unwrap();
    assert_eq!(forward.exact_ref().as_str(), "example.profile.root@1.0.0");
    assert_eq!(
        forward.resolved_profile_fingerprint(),
        reverse.resolved_profile_fingerprint()
    );
    assert_eq!(forward.layer_decisions().len(), 11);
    assert_eq!(
        forward
            .artifact_kind_registry()
            .kind_refs()
            .into_iter()
            .map(|x| x.as_str().to_owned())
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "handbook.artifact-kind.decision-record@1.0.0".into(),
            "handbook.artifact-kind.environment-context@1.0.0".into(),
            "handbook.artifact-kind.project-authority@1.0.0".into(),
            "handbook.artifact-kind.project-context@1.0.0".into(),
            "handbook.artifact-kind.risk-record@1.0.0".into(),
            "handbook.artifact-kind.work-specification@1.0.0".into()
        ])
    );
    assert_eq!(
        forward.artifact_instances().ids(),
        [
            "environment_context",
            "project_authority",
            "project_context"
        ]
    );
}
#[test]
fn missing_typed_source_and_nonempty_later_owned_values_refuse() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut missing = request(false);
    missing.project_condition_sources.clear();
    assert!(resolve_profile_selection(root, missing).is_err());
    let mut unsupported = request(false);
    unsupported.allowed_schema_roots.clear();
    assert!(resolve_profile_selection(root, unsupported).is_err());

    let mut unrelated = shipped_request(false, false);
    unrelated.profile_sources.push(DefinitionSourceBinding {
        definition_ref: r("example.profile.repository@1.0.0"),
        source: DefinitionSource::RepositoryPath(
            "tests/fixtures/hcm_1_2_repository_profile/repository.yaml".into(),
        ),
    });
    assert!(resolve_profile_selection(root, unrelated).is_err());
}

#[test]
fn shipped_root_and_repository_replace_whole_fixture_are_exact() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let shipped = resolve_profile_selection(root, shipped_request(false, false)).unwrap();
    assert_eq!(
        shipped.exact_ref().as_str(),
        "handbook.profile.shipped-root@1.0.0"
    );
    assert_eq!(
        shipped.artifact_instances().ids(),
        [
            "environment_context",
            "project_authority",
            "project_context"
        ]
    );
    let forward = resolve_profile_selection(root, shipped_request(true, false)).unwrap();
    let reverse = resolve_profile_selection(root, shipped_request(true, true)).unwrap();
    assert_eq!(
        forward.resolved_profile_fingerprint(),
        reverse.resolved_profile_fingerprint()
    );
    for field in [
        ProfileField::SchemaRegistrySources,
        ProfileField::ArtifactKindSources,
        ProfileField::VocabularyRef,
        ProfileField::ContextResolutionRef,
    ] {
        let decision = forward
            .layer_decisions()
            .iter()
            .find(|decision| decision.field() == field)
            .unwrap();
        assert_eq!(decision.disposition(), LayerDisposition::Replaced);
        assert_eq!(
            decision.source_profile_ref().as_str(),
            "example.profile.repository@1.0.0"
        );
    }
}

#[test]
fn built_in_selection_uses_immutable_package_bytes_and_ignores_repo_shadows() {
    let repo = tempfile::tempdir().unwrap();
    let before = resolve_profile_selection(repo.path(), shipped_request(false, false)).unwrap();

    for path in [
        "definitions/profiles/handbook.profile.shipped-root/1.0.0.yaml",
        "definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.entry.yaml",
        "definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.schema.json",
        "definitions/artifact-kinds/handbook.artifact-kind.project-authority/1.0.0.yaml",
    ] {
        let path = repo.path().join(path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, b"repo shadow must never be opened\n").unwrap();
    }

    let after = resolve_profile_selection(repo.path(), shipped_request(false, true)).unwrap();
    assert_eq!(before.exact_ref(), after.exact_ref());
    assert_eq!(
        before.resolved_profile_fingerprint(),
        after.resolved_profile_fingerprint()
    );
}

#[test]
fn package_owned_refs_and_every_shipped_profile_drift_refuse_repository_bytes() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let shipped_bytes = std::fs::read(
        crate_root.join("definitions/profiles/handbook.profile.shipped-root/1.0.0.yaml"),
    )
    .unwrap();
    for mutation in [
        "catalog_addition",
        "catalog_omission",
        "catalog_substitution",
        "instance_addition",
        "instance_omission",
        "instance_substitution",
        "path_drift",
        "label_drift",
        "requiredness_drift",
    ] {
        let repo = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(repo.path().join("sources")).unwrap();
        let mut profile: serde_json::Value = serde_yaml_bw::from_slice(&shipped_bytes).unwrap();
        match mutation {
            "catalog_addition" => profile["artifact_kind_sources"]
                .as_array_mut()
                .unwrap()
                .push(serde_json::json!("example.kind.extra@1.0.0")),
            "catalog_omission" => {
                profile["artifact_kind_sources"]
                    .as_array_mut()
                    .unwrap()
                    .pop();
            }
            "catalog_substitution" => {
                profile["artifact_kind_sources"][0] =
                    serde_json::json!("example.kind.substitute@1.0.0");
            }
            "instance_addition" => {
                let extra = profile["artifact_instances"][1].clone();
                profile["artifact_instances"]
                    .as_array_mut()
                    .unwrap()
                    .push(extra);
            }
            "instance_omission" => {
                profile["artifact_instances"].as_array_mut().unwrap().pop();
            }
            "instance_substitution" => {
                profile["artifact_instances"][1]["kind_ref"] =
                    serde_json::json!("handbook.artifact-kind.risk-record@1.0.0");
            }
            "path_drift" => {
                profile["artifact_instances"][0]["canonical_path"] =
                    serde_json::json!(".handbook/project/spoof.yaml");
            }
            "label_drift" => {
                profile["artifact_instances"][0]["label"] = serde_json::json!("Spoofed Charter");
            }
            "requiredness_drift" => {
                profile["artifact_instances"][0]["requiredness"]["mode"] =
                    serde_json::json!("optional");
            }
            _ => unreachable!(),
        }
        std::fs::write(
            repo.path().join("sources/spoof.yaml"),
            serde_yaml_bw::to_string(&profile).unwrap(),
        )
        .unwrap();
        let mut selection = shipped_request(false, false);
        selection.profile_sources[0].source =
            DefinitionSource::RepositoryPath("sources/spoof.yaml".into());
        let failure = resolve_profile_selection(repo.path(), selection).unwrap_err();
        assert_eq!(
            failure.kind(),
            ProfileLoadErrorKind::ShippedSetMismatch,
            "{mutation}"
        );
        assert_eq!(failure.location(), Some("definition_source"), "{mutation}");
    }

    let repo = tempfile::tempdir().unwrap();
    let mut schema_spoof = shipped_request(false, false);
    schema_spoof.schema_entry_sources[0].source =
        DefinitionSource::RepositoryPath("sources/schema.entry.yaml".into());
    let failure = resolve_profile_selection(repo.path(), schema_spoof).unwrap_err();
    assert_eq!(failure.kind(), ProfileLoadErrorKind::ShippedSetMismatch);
    assert_eq!(failure.location(), Some("definition_source"));

    type PackageBinding =
        for<'a> fn(&'a mut ProfileSelectionRequest) -> &'a mut DefinitionSourceBinding;
    let package_classes: [(&str, PackageBinding); 10] = [
        ("profile", |request| &mut request.profile_sources[0]),
        ("stable_role_registry", |request| {
            &mut request.stable_role_registry_sources[0]
        }),
        ("schema_entry", |request| {
            &mut request.schema_entry_sources[0]
        }),
        ("artifact_kind", |request| {
            &mut request.artifact_kind_sources[0]
        }),
        ("semantic_capability", |request| {
            &mut request.semantic_capability_sources[0]
        }),
        ("semantic_validator", |request| {
            &mut request.semantic_validator_sources[0]
        }),
        ("project_condition", |request| {
            &mut request.project_condition_sources[0]
        }),
        ("vocabulary", |request| &mut request.vocabulary_sources[0]),
        ("context_resolution", |request| {
            &mut request.context_resolution_sources[0]
        }),
        ("context_resolution_policy", |request| {
            &mut request.context_resolution_policy_sources[0]
        }),
    ];
    for (class, mutate) in package_classes {
        let repo = tempfile::tempdir().unwrap();
        let mut selection = shipped_request(false, false);
        mutate(&mut selection).source =
            DefinitionSource::RepositoryPath("sources/package-spoof.yaml".into());
        let failure = resolve_profile_selection(repo.path(), selection).unwrap_err();
        assert_eq!(
            failure.kind(),
            ProfileLoadErrorKind::ShippedSetMismatch,
            "{class}"
        );
        assert_eq!(failure.location(), Some("definition_source"), "{class}");
    }
}

fn repository_root_request(repo: &Path) -> ProfileSelectionRequest {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::create_dir_all(repo.join("sources")).unwrap();
    std::fs::copy(
        crate_root.join("tests/fixtures/hcm_1_2_repository_profile/root.yaml"),
        repo.join("sources/root.yaml"),
    )
    .unwrap();
    let mut value = request(false);
    value.profile_sources[0].source = DefinitionSource::RepositoryPath("sources/root.yaml".into());
    value
}

fn close_root_profile_fingerprint(
    value: &mut serde_json::Value,
    resolved: &ResolvedInstanceProfile,
    instances: &ArtifactInstanceRegistry,
) {
    let mut dependencies = Vec::new();
    let mut dependency = |class: &str, reference: &str, fingerprint: &str| {
        dependencies.push(serde_json::json!({
            "definition_class": class,
            "reference": reference,
            "fingerprint": fingerprint,
        }));
    };
    dependency(
        "stable_role_registry",
        resolved.stable_role_registry().exact_ref().as_str(),
        resolved.stable_role_registry().fingerprint().as_str(),
    );
    for reference in value["schema_registry_sources"].as_array().unwrap() {
        let reference = r(reference.as_str().unwrap());
        let entry = resolved
            .artifact_kind_registry()
            .schema_registry()
            .entry(&reference)
            .unwrap();
        dependency(
            "schema_entry",
            reference.as_str(),
            entry.entry_fingerprint().as_str(),
        );
    }
    for reference in value["artifact_kind_sources"].as_array().unwrap() {
        let reference = r(reference.as_str().unwrap());
        let kind = resolved.artifact_kind_registry().kind(&reference).unwrap();
        dependency(
            "artifact_kind",
            reference.as_str(),
            kind.definition_fingerprint().as_str(),
        );
    }
    dependency(
        "artifact_instance_registry",
        "example.profile.root@1.0.0",
        instances.fingerprint().as_str(),
    );
    dependency(
        "vocabulary",
        resolved.vocabulary().exact_ref().as_str(),
        resolved.vocabulary().vocabulary_fingerprint().as_str(),
    );
    dependency(
        "context_resolution",
        resolved.context_resolution().exact_ref().as_str(),
        resolved
            .context_resolution()
            .definition_fingerprint()
            .as_str(),
    );
    dependencies.sort_by(|left, right| {
        (
            left["definition_class"].as_str(),
            left["reference"].as_str(),
        )
            .cmp(&(
                right["definition_class"].as_str(),
                right["reference"].as_str(),
            ))
    });
    let mut definition = value.clone();
    definition
        .as_object_mut()
        .unwrap()
        .remove("profile_fingerprint");
    value["profile_fingerprint"] = DefinitionFingerprint::from_json_value(&serde_json::json!({
        "definition": definition,
        "dependencies": dependencies,
    }))
    .unwrap()
    .to_string()
    .into();
}

fn subset_child_request(repo: &Path, replace_descriptors: bool) -> ProfileSelectionRequest {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let baseline = resolve_profile_selection(crate_root, request(false)).unwrap();
    std::fs::create_dir_all(repo.join("sources")).unwrap();
    let root_bytes =
        std::fs::read(crate_root.join("tests/fixtures/hcm_1_2_repository_profile/root.yaml"))
            .unwrap();
    std::fs::write(repo.join("sources/root.yaml"), &root_bytes).unwrap();
    let root: serde_json::Value = serde_yaml_bw::from_slice(&root_bytes).unwrap();

    let mut child: serde_json::Value = serde_yaml_bw::from_slice(
        &std::fs::read(
            crate_root.join("tests/fixtures/hcm_1_2_repository_profile/repository.yaml"),
        )
        .unwrap(),
    )
    .unwrap();
    child["profile_id"] = serde_json::json!("example.profile.subset");
    child["extends_profile_ref"] = serde_json::json!("example.profile.root@1.0.0");
    child["schema_registry_sources"] =
        serde_json::json!(["handbook.schemas.artifacts.project-authority@1.0.0"]);
    child["artifact_kind_sources"] =
        serde_json::json!(["handbook.artifact-kind.project-authority@1.0.0"]);

    let mut dependencies = vec![serde_json::json!({
        "definition_class": "profile",
        "reference": "example.profile.root@1.0.0",
        "fingerprint": root["profile_fingerprint"].as_str().unwrap(),
    })];
    let schema_ref = r("handbook.schemas.artifacts.project-authority@1.0.0");
    let schema = baseline
        .artifact_kind_registry()
        .schema_registry()
        .entry(&schema_ref)
        .unwrap();
    dependencies.push(serde_json::json!({
        "definition_class": "schema_entry",
        "reference": schema_ref.as_str(),
        "fingerprint": schema.entry_fingerprint().as_str(),
    }));
    let kind_ref = r("handbook.artifact-kind.project-authority@1.0.0");
    let kind = baseline.artifact_kind_registry().kind(&kind_ref).unwrap();
    dependencies.push(serde_json::json!({
        "definition_class": "artifact_kind",
        "reference": kind_ref.as_str(),
        "fingerprint": kind.definition_fingerprint().as_str(),
    }));
    if replace_descriptors {
        child["artifact_instances"] = serde_json::json!([root["artifact_instances"][0].clone()]);
        let descriptors = ArtifactInstanceRegistry::resolve(
            child["artifact_instances"].as_array().unwrap(),
            baseline.artifact_kind_registry(),
            &[],
        )
        .unwrap();
        dependencies.push(serde_json::json!({
            "definition_class": "artifact_instance_registry",
            "reference": "example.profile.subset@1.0.0",
            "fingerprint": descriptors.fingerprint().as_str(),
        }));
    }
    dependencies.push(serde_json::json!({
        "definition_class": "vocabulary",
        "reference": baseline.vocabulary().exact_ref().as_str(),
        "fingerprint": baseline.vocabulary().vocabulary_fingerprint().as_str(),
    }));
    dependencies.push(serde_json::json!({
        "definition_class": "context_resolution",
        "reference": baseline.context_resolution().exact_ref().as_str(),
        "fingerprint": baseline.context_resolution().definition_fingerprint().as_str(),
    }));
    dependencies.sort_by(|left, right| {
        (
            left["definition_class"].as_str(),
            left["reference"].as_str(),
        )
            .cmp(&(
                right["definition_class"].as_str(),
                right["reference"].as_str(),
            ))
    });
    let mut definition = child.clone();
    definition
        .as_object_mut()
        .unwrap()
        .remove("profile_fingerprint");
    child["profile_fingerprint"] = DefinitionFingerprint::from_json_value(&serde_json::json!({
        "definition": definition,
        "dependencies": dependencies,
    }))
    .unwrap()
    .to_string()
    .into();
    std::fs::write(
        repo.join("sources/child.yaml"),
        serde_yaml_bw::to_string(&child).unwrap(),
    )
    .unwrap();

    let mut selection = request(false);
    selection.selected_profile_ref = r("example.profile.subset@1.0.0");
    selection.profile_sources = vec![
        DefinitionSourceBinding {
            definition_ref: r("example.profile.root@1.0.0"),
            source: DefinitionSource::RepositoryPath("sources/root.yaml".into()),
        },
        DefinitionSourceBinding {
            definition_ref: r("example.profile.subset@1.0.0"),
            source: DefinitionSource::RepositoryPath("sources/child.yaml".into()),
        },
    ];
    selection
}

fn close_subset_profile_fingerprint(
    value: &mut serde_json::Value,
    parent_fingerprint: &str,
    resolved: &ResolvedInstanceProfile,
) {
    let mut dependencies = vec![serde_json::json!({
        "definition_class": "profile",
        "reference": value["extends_profile_ref"].as_str().unwrap(),
        "fingerprint": parent_fingerprint,
    })];
    for reference in value["schema_registry_sources"].as_array().unwrap() {
        let reference = r(reference.as_str().unwrap());
        let entry = resolved
            .artifact_kind_registry()
            .schema_registry()
            .entry(&reference)
            .unwrap();
        dependencies.push(serde_json::json!({
            "definition_class": "schema_entry",
            "reference": reference.as_str(),
            "fingerprint": entry.entry_fingerprint().as_str(),
        }));
    }
    for reference in value["artifact_kind_sources"].as_array().unwrap() {
        let reference = r(reference.as_str().unwrap());
        let kind = resolved.artifact_kind_registry().kind(&reference).unwrap();
        dependencies.push(serde_json::json!({
            "definition_class": "artifact_kind",
            "reference": reference.as_str(),
            "fingerprint": kind.definition_fingerprint().as_str(),
        }));
    }
    if let Some(instances) = value
        .get("artifact_instances")
        .and_then(|value| value.as_array())
    {
        let registry =
            ArtifactInstanceRegistry::resolve(instances, resolved.artifact_kind_registry(), &[])
                .unwrap();
        dependencies.push(serde_json::json!({
            "definition_class": "artifact_instance_registry",
            "reference": format!(
                "{}@{}",
                value["profile_id"].as_str().unwrap(),
                value["profile_version"].as_str().unwrap()
            ),
            "fingerprint": registry.fingerprint().as_str(),
        }));
    }
    let vocabulary_ref = r(value["vocabulary_ref"].as_str().unwrap());
    assert_eq!(vocabulary_ref, *resolved.vocabulary().exact_ref());
    dependencies.push(serde_json::json!({
        "definition_class": "vocabulary",
        "reference": vocabulary_ref.as_str(),
        "fingerprint": resolved.vocabulary().vocabulary_fingerprint().as_str(),
    }));
    let context_ref = r(value["context_resolution_ref"].as_str().unwrap());
    assert_eq!(context_ref, *resolved.context_resolution().exact_ref());
    dependencies.push(serde_json::json!({
        "definition_class": "context_resolution",
        "reference": context_ref.as_str(),
        "fingerprint": resolved
            .context_resolution()
            .definition_fingerprint()
            .as_str(),
    }));
    dependencies.sort_by(|left, right| {
        (
            left["definition_class"].as_str(),
            left["reference"].as_str(),
        )
            .cmp(&(
                right["definition_class"].as_str(),
                right["reference"].as_str(),
            ))
    });
    let mut definition = value.clone();
    definition
        .as_object_mut()
        .unwrap()
        .remove("profile_fingerprint");
    value["profile_fingerprint"] = DefinitionFingerprint::from_json_value(&serde_json::json!({
        "definition": definition,
        "dependencies": dependencies,
    }))
    .unwrap()
    .to_string()
    .into();
}

fn layered_later_owned_request(
    repo: &Path,
    field: &str,
    parent_value: serde_json::Value,
    child_value: serde_json::Value,
    reverse: bool,
) -> ProfileSelectionRequest {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let baseline = resolve_profile_selection(crate_root, request(false)).unwrap();
    let mut selection = subset_child_request(repo, true);
    let root_path = repo.join("sources/root.yaml");
    let child_path = repo.join("sources/child.yaml");
    let mut root: serde_json::Value =
        serde_yaml_bw::from_slice(&std::fs::read(&root_path).unwrap()).unwrap();
    let mut child: serde_json::Value =
        serde_yaml_bw::from_slice(&std::fs::read(&child_path).unwrap()).unwrap();

    root[field] = parent_value;
    let condition = ProjectConditionDefinition::load(
        crate_root,
        "definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml",
    )
    .unwrap();
    let root_instances = ArtifactInstanceRegistry::resolve(
        root["artifact_instances"].as_array().unwrap(),
        baseline.artifact_kind_registry(),
        &[&condition],
    )
    .unwrap();
    close_root_profile_fingerprint(&mut root, &baseline, &root_instances);

    child[field] = child_value;
    close_subset_profile_fingerprint(
        &mut child,
        root["profile_fingerprint"].as_str().unwrap(),
        &baseline,
    );
    std::fs::write(&root_path, serde_yaml_bw::to_string(&root).unwrap()).unwrap();
    std::fs::write(&child_path, serde_yaml_bw::to_string(&child).unwrap()).unwrap();
    if reverse {
        selection.profile_sources.reverse();
    }
    selection
}

#[test]
fn shadowed_later_owned_ancestry_fields_refuse_in_both_source_orders() {
    let cases = [
        (
            "projection_catalog_refs",
            serde_json::json!(["example.projection.forbidden@1.0.0"]),
            serde_json::json!([]),
        ),
        (
            "posture_evaluation_policy",
            serde_json::json!({
                "ref": "example.posture.forbidden@1.0.0",
                "fingerprint": "sha256:0000000000000000000000000000000000000000000000000000000000000000"
            }),
            serde_json::Value::Null,
        ),
        (
            "dock_requirement_refs",
            serde_json::json!(["example.dock.forbidden@1.0.0"]),
            serde_json::json!([]),
        ),
        (
            "adapter_overlay_refs",
            serde_json::json!(["example.adapter.forbidden@1.0.0"]),
            serde_json::json!([]),
        ),
        (
            "extensions",
            serde_json::json!({"forbidden": true}),
            serde_json::json!({}),
        ),
    ];
    for (field, parent, child) in cases {
        for reverse in [false, true] {
            let repo = tempfile::tempdir().unwrap();
            let failure = resolve_profile_selection(
                repo.path(),
                layered_later_owned_request(
                    repo.path(),
                    field,
                    parent.clone(),
                    child.clone(),
                    reverse,
                ),
            )
            .unwrap_err();
            assert_eq!(
                failure.kind(),
                ProfileLoadErrorKind::Registry(RegistryLoadErrorKind::UnsupportedDependency),
                "{field} reverse={reverse}"
            );
        }
    }
}

#[test]
fn empty_and_inherited_later_owned_ancestry_fields_remain_supported() {
    for (field, empty) in [
        ("projection_catalog_refs", serde_json::json!([])),
        ("posture_evaluation_policy", serde_json::Value::Null),
        ("dock_requirement_refs", serde_json::json!([])),
        ("adapter_overlay_refs", serde_json::json!([])),
        ("extensions", serde_json::json!({})),
    ] {
        for reverse in [false, true] {
            let repo = tempfile::tempdir().unwrap();
            let resolved = resolve_profile_selection(
                repo.path(),
                layered_later_owned_request(
                    repo.path(),
                    field,
                    empty.clone(),
                    empty.clone(),
                    reverse,
                ),
            )
            .unwrap();
            assert_eq!(
                resolved.exact_ref().as_str(),
                "example.profile.subset@1.0.0"
            );
        }
    }

    for reverse in [false, true] {
        let repo = tempfile::tempdir().unwrap();
        let mut inherited = subset_child_request(repo.path(), true);
        if reverse {
            inherited.profile_sources.reverse();
        }
        resolve_profile_selection(repo.path(), inherited).unwrap();
    }
}

#[test]
fn replace_whole_schema_and_kind_fields_return_only_the_winning_literal_sets() {
    let repo = tempfile::tempdir().unwrap();
    let resolved =
        resolve_profile_selection(repo.path(), subset_child_request(repo.path(), true)).unwrap();
    assert_eq!(
        resolved.artifact_kind_registry().kind_refs(),
        [r("handbook.artifact-kind.project-authority@1.0.0")]
    );
    assert_eq!(
        resolved
            .artifact_kind_registry()
            .schema_registry()
            .entry_refs(),
        [r("handbook.schemas.artifacts.project-authority@1.0.0")]
    );
    assert_eq!(resolved.artifact_instances().ids(), ["project_authority"]);

    let invalid_repo = tempfile::tempdir().unwrap();
    let failure = resolve_profile_selection(
        invalid_repo.path(),
        subset_child_request(invalid_repo.path(), false),
    )
    .unwrap_err();
    assert_eq!(
        failure.kind(),
        ProfileLoadErrorKind::Registry(RegistryLoadErrorKind::UnsupportedDependency)
    );
}

#[test]
fn typed_source_binding_rejects_swapped_schema_and_kind_paths_at_stage_five() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let schema_repo = tempfile::tempdir().unwrap();
    let mut schemas = repository_root_request(schema_repo.path());
    for (target, source, content_schema_id) in [
        (
            "sources/custom-one.entry.yaml",
            "definitions/schemas/handbook.schemas.artifacts.project-context/1.0.0.entry.yaml",
            "example.schemas.custom-one",
        ),
        (
            "sources/custom-two.entry.yaml",
            "definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.entry.yaml",
            "example.schemas.custom-two",
        ),
    ] {
        let mut entry: serde_json::Value =
            serde_yaml_bw::from_slice(&std::fs::read(crate_root.join(source)).unwrap()).unwrap();
        entry["content_schema_id"] = serde_json::json!(content_schema_id);
        std::fs::write(
            schema_repo.path().join(target),
            serde_yaml_bw::to_string(&entry).unwrap(),
        )
        .unwrap();
    }
    schemas.schema_entry_sources.extend([
        DefinitionSourceBinding {
            definition_ref: r("example.schemas.custom-one@1.0.0"),
            source: DefinitionSource::RepositoryPath("sources/custom-two.entry.yaml".into()),
        },
        DefinitionSourceBinding {
            definition_ref: r("example.schemas.custom-two@1.0.0"),
            source: DefinitionSource::RepositoryPath("sources/custom-one.entry.yaml".into()),
        },
    ]);
    assert_eq!(
        resolve_profile_selection(schema_repo.path(), schemas)
            .unwrap_err()
            .kind(),
        ProfileLoadErrorKind::SourceIdentityMismatch
    );

    let kind_repo = tempfile::tempdir().unwrap();
    let mut kinds = repository_root_request(kind_repo.path());
    for (target, source, kind_id) in [
        (
            "sources/custom-one.kind.yaml",
            "definitions/artifact-kinds/handbook.artifact-kind.project-context/1.0.0.yaml",
            "example.artifact-kind.custom-one",
        ),
        (
            "sources/custom-two.kind.yaml",
            "definitions/artifact-kinds/handbook.artifact-kind.project-authority/1.0.0.yaml",
            "example.artifact-kind.custom-two",
        ),
    ] {
        let mut kind: serde_json::Value =
            serde_yaml_bw::from_slice(&std::fs::read(crate_root.join(source)).unwrap()).unwrap();
        kind["kind_id"] = serde_json::json!(kind_id);
        std::fs::write(
            kind_repo.path().join(target),
            serde_yaml_bw::to_string(&kind).unwrap(),
        )
        .unwrap();
    }
    kinds.artifact_kind_sources.extend([
        DefinitionSourceBinding {
            definition_ref: r("example.artifact-kind.custom-one@1.0.0"),
            source: DefinitionSource::RepositoryPath("sources/custom-two.kind.yaml".into()),
        },
        DefinitionSourceBinding {
            definition_ref: r("example.artifact-kind.custom-two@1.0.0"),
            source: DefinitionSource::RepositoryPath("sources/custom-one.kind.yaml".into()),
        },
    ]);
    assert_eq!(
        resolve_profile_selection(kind_repo.path(), kinds)
            .unwrap_err()
            .kind(),
        ProfileLoadErrorKind::SourceIdentityMismatch
    );
}

#[test]
fn root_and_child_replace_whole_fields_refuse_duplicate_schema_and_kind_refs() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for field in ["schema_registry_sources", "artifact_kind_sources"] {
        for child in [false, true] {
            for reverse in [false, true] {
                let repo = tempfile::tempdir().unwrap();
                std::fs::create_dir_all(repo.path().join("sources")).unwrap();
                let source = if child {
                    crate_root.join("tests/fixtures/hcm_1_2_repository_profile/repository.yaml")
                } else {
                    crate_root.join("tests/fixtures/hcm_1_2_repository_profile/root.yaml")
                };
                let target = repo.path().join("sources/profile.yaml");
                let mut value: serde_json::Value =
                    serde_yaml_bw::from_slice(&std::fs::read(source).unwrap()).unwrap();
                let values = value[field].as_array_mut().unwrap();
                values.push(values[0].clone());
                if reverse {
                    values.reverse();
                }
                std::fs::write(&target, serde_yaml_bw::to_string(&value).unwrap()).unwrap();

                let selection = if child {
                    let mut selection = shipped_request(true, reverse);
                    let binding = selection
                        .profile_sources
                        .iter_mut()
                        .find(|binding| {
                            binding.definition_ref.as_str() == "example.profile.repository@1.0.0"
                        })
                        .unwrap();
                    binding.source =
                        DefinitionSource::RepositoryPath("sources/profile.yaml".into());
                    selection
                } else {
                    let mut selection = request(reverse);
                    selection.profile_sources[0].source =
                        DefinitionSource::RepositoryPath("sources/profile.yaml".into());
                    selection
                };
                let failure = resolve_profile_selection(repo.path(), selection).unwrap_err();
                assert_eq!(
                    failure.kind(),
                    ProfileLoadErrorKind::DuplicateProfileDependency,
                    "{field} child={child} reverse={reverse}"
                );
                assert_eq!(failure.location(), Some(field));
            }
        }
    }
}

#[test]
fn profile_failures_preserve_public_typed_categories() {
    let repo = tempfile::tempdir().unwrap();
    let mut invalid_scope = repository_root_request(repo.path());
    let path = repo.path().join("sources/root.yaml");
    let mut value: serde_json::Value =
        serde_yaml_bw::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    value["profile_scope"] = serde_json::json!("ambient");
    std::fs::write(&path, serde_yaml_bw::to_string(&value).unwrap()).unwrap();
    assert_eq!(
        resolve_profile_selection(repo.path(), invalid_scope.clone())
            .unwrap_err()
            .kind(),
        ProfileLoadErrorKind::InvalidProfileScope
    );

    let unrelated_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut unrelated = shipped_request(false, false);
    unrelated.profile_sources.push(DefinitionSourceBinding {
        definition_ref: r("example.profile.repository@1.0.0"),
        source: DefinitionSource::RepositoryPath(
            "tests/fixtures/hcm_1_2_repository_profile/repository.yaml".into(),
        ),
    });
    assert_eq!(
        resolve_profile_selection(unrelated_root, unrelated)
            .unwrap_err()
            .kind(),
        ProfileLoadErrorKind::UnreferencedSource
    );

    invalid_scope.profile_sources[0].source =
        DefinitionSource::RepositoryPath("sources/root.yaml".into());
    value["profile_scope"] = serde_json::json!("named");
    value["profile_id"] = serde_json::json!("Example.Profile.Root");
    std::fs::write(&path, serde_yaml_bw::to_string(&value).unwrap()).unwrap();
    assert_eq!(
        resolve_profile_selection(repo.path(), invalid_scope.clone())
            .unwrap_err()
            .kind(),
        ProfileLoadErrorKind::InvalidProfileIdentity
    );

    value["profile_id"] = serde_json::json!("example.profile.root");
    value["profile_fingerprint"] = serde_json::json!("sha256:NOT-CANONICAL");
    std::fs::write(&path, serde_yaml_bw::to_string(&value).unwrap()).unwrap();
    assert_eq!(
        resolve_profile_selection(repo.path(), invalid_scope.clone())
            .unwrap_err()
            .kind(),
        ProfileLoadErrorKind::InvalidProfileFingerprint
    );

    value["profile_fingerprint"] = serde_json::json!(
        "sha256:0000000000000000000000000000000000000000000000000000000000000000"
    );
    std::fs::write(&path, serde_yaml_bw::to_string(&value).unwrap()).unwrap();
    assert_eq!(
        resolve_profile_selection(repo.path(), invalid_scope)
            .unwrap_err()
            .kind(),
        ProfileLoadErrorKind::FingerprintMismatch
    );
}

#[test]
fn condition_sources_are_derived_only_from_selected_ancestry_descriptors() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let baseline = resolve_profile_selection(crate_root, request(false)).unwrap();
    let repo = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(repo.path().join("sources")).unwrap();
    let mut value: serde_json::Value = serde_yaml_bw::from_slice(
        &std::fs::read(crate_root.join("tests/fixtures/hcm_1_2_repository_profile/root.yaml"))
            .unwrap(),
    )
    .unwrap();
    value["artifact_instances"][2]["requiredness"] =
        serde_json::json!({"mode":"always", "condition_ref":null});
    let instances = ArtifactInstanceRegistry::resolve(
        value["artifact_instances"].as_array().unwrap(),
        baseline.artifact_kind_registry(),
        &[],
    )
    .unwrap();
    close_root_profile_fingerprint(&mut value, &baseline, &instances);
    std::fs::write(
        repo.path().join("sources/root.yaml"),
        serde_yaml_bw::to_string(&value).unwrap(),
    )
    .unwrap();

    let mut condition_free = request(false);
    condition_free.profile_sources[0].source =
        DefinitionSource::RepositoryPath("sources/root.yaml".into());
    condition_free.project_condition_sources.clear();
    resolve_profile_selection(repo.path(), condition_free.clone()).unwrap();

    condition_free.project_condition_sources.push(builtin(
        "handbook.condition.project.managed-operational-surface@1.0.0",
    ));
    assert_eq!(
        resolve_profile_selection(repo.path(), condition_free)
            .unwrap_err()
            .kind(),
        ProfileLoadErrorKind::UnreferencedSource
    );
}

fn vocabulary_role_mismatch_request(
    repo: &Path,
    include_vocabulary_role_producer: bool,
) -> ProfileSelectionRequest {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let baseline = resolve_profile_selection(crate_root, request(false)).unwrap();
    let roles_1_0_ref = r("handbook.roles.core@1.0.0");
    let roles_1_0 = StableRoleRegistry::load_builtin(&roles_1_0_ref).unwrap();
    std::fs::create_dir_all(repo.join("sources")).unwrap();

    let mut profile: serde_json::Value = serde_yaml_bw::from_slice(
        &std::fs::read(crate_root.join("tests/fixtures/hcm_1_2_repository_profile/root.yaml"))
            .unwrap(),
    )
    .unwrap();
    profile["stable_role_registry"] = serde_json::json!({
        "ref": roles_1_0_ref.as_str(),
        "fingerprint": roles_1_0.fingerprint().as_str(),
    });
    profile["schema_registry_sources"] =
        serde_json::json!(["handbook.schemas.artifacts.project-context@1.0.0"]);
    profile["artifact_kind_sources"] =
        serde_json::json!(["example.artifact-kind.project-context@1.0.0"]);
    profile["artifact_instances"] = serde_json::json!([]);
    std::fs::write(
        repo.join("sources/root.yaml"),
        serde_yaml_bw::to_string(&profile).unwrap(),
    )
    .unwrap();

    let mut kind: serde_json::Value =
        serde_yaml_bw::from_slice(
            &std::fs::read(crate_root.join(
                "definitions/artifact-kinds/handbook.artifact-kind.project-context/1.0.0.yaml",
            ))
            .unwrap(),
        )
        .unwrap();
    kind["stable_role_registry"] = serde_json::json!({
        "ref": roles_1_0_ref.as_str(),
        "fingerprint": roles_1_0.fingerprint().as_str(),
    });
    kind["kind_id"] = serde_json::json!("example.artifact-kind.project-context");
    let mut definition = kind.clone();
    definition
        .as_object_mut()
        .unwrap()
        .remove("definition_fingerprint");
    let schema_ref = r("handbook.schemas.artifacts.project-context@1.0.0");
    let schema = baseline
        .artifact_kind_registry()
        .schema_registry()
        .entry(&schema_ref)
        .unwrap();
    kind["definition_fingerprint"] = DefinitionFingerprint::from_json_value(&serde_json::json!({
        "definition": definition,
        "stable_role_registry_fingerprint": roles_1_0.fingerprint().as_str(),
        "schema_entry_fingerprint": schema.entry_fingerprint().as_str(),
        "schema_closure_fingerprint": schema.closure_fingerprint().as_str(),
    }))
    .unwrap()
    .to_string()
    .into();
    std::fs::write(
        repo.join("sources/project-context.kind.yaml"),
        serde_yaml_bw::to_string(&kind).unwrap(),
    )
    .unwrap();

    let mut selection = request(false);
    selection.profile_sources[0].source =
        DefinitionSource::RepositoryPath("sources/root.yaml".into());
    selection.stable_role_registry_sources = vec![builtin("handbook.roles.core@1.0.0")];
    if include_vocabulary_role_producer {
        selection
            .stable_role_registry_sources
            .push(builtin("handbook.roles.core@1.1.0"));
    }
    selection.schema_entry_sources =
        vec![builtin("handbook.schemas.artifacts.project-context@1.0.0")];
    selection.artifact_kind_sources = vec![DefinitionSourceBinding {
        definition_ref: r("example.artifact-kind.project-context@1.0.0"),
        source: DefinitionSource::RepositoryPath("sources/project-context.kind.yaml".into()),
    }];
    selection.project_condition_sources.clear();
    selection
}

#[test]
fn vocabulary_resolves_and_matches_its_stable_role_registry_producer() {
    let missing_repo = tempfile::tempdir().unwrap();
    let missing = vocabulary_role_mismatch_request(missing_repo.path(), false);
    assert_eq!(
        resolve_profile_selection(missing_repo.path(), missing)
            .unwrap_err()
            .kind(),
        ProfileLoadErrorKind::MissingSource
    );

    let mismatch_repo = tempfile::tempdir().unwrap();
    let mismatch = vocabulary_role_mismatch_request(mismatch_repo.path(), true);
    assert_eq!(
        resolve_profile_selection(mismatch_repo.path(), mismatch)
            .unwrap_err()
            .kind(),
        ProfileLoadErrorKind::Registry(RegistryLoadErrorKind::StableRoleRegistryMismatch)
    );

    let stale_repo = tempfile::tempdir().unwrap();
    let stale = vocabulary_role_mismatch_request(stale_repo.path(), true);
    let path = stale_repo.path().join("sources/root.yaml");
    let mut profile: serde_json::Value =
        serde_yaml_bw::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    profile["stable_role_registry"]["fingerprint"] = serde_json::json!(
        "sha256:0000000000000000000000000000000000000000000000000000000000000000"
    );
    std::fs::write(&path, serde_yaml_bw::to_string(&profile).unwrap()).unwrap();
    assert_eq!(
        resolve_profile_selection(stale_repo.path(), stale)
            .unwrap_err()
            .kind(),
        ProfileLoadErrorKind::FingerprintMismatch
    );
}

#[test]
fn selected_nonempty_vocabulary_is_retained_in_exact_profile_identity_only() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let baseline = resolve_profile_selection(crate_root, request(false)).unwrap();
    let repo = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(repo.path().join("sources")).unwrap();

    let root_bytes =
        std::fs::read(crate_root.join("tests/fixtures/hcm_1_2_repository_profile/root.yaml"))
            .unwrap();
    std::fs::write(repo.path().join("sources/root.yaml"), &root_bytes).unwrap();
    std::fs::write(
        repo.path().join("sources/hcm-3-1.vocabulary.yaml"),
        HCM_3_1_VOCABULARY_BYTES,
    )
    .unwrap();

    let root: serde_json::Value = serde_yaml_bw::from_slice(&root_bytes).unwrap();
    let mut child = serde_json::json!({
        "schema_id": "handbook.instance-profile",
        "schema_version": "1.0",
        "profile_id": "example.profile.hcm-3-1",
        "profile_version": "1.0.0",
        "profile_scope": "repository",
        "extends_profile_ref": "example.profile.root@1.0.0",
        "vocabulary_ref": "example.vocabulary.hcm-3-1@1.0.0"
    });
    let definition = child.clone();
    child["profile_fingerprint"] = DefinitionFingerprint::from_json_value(&serde_json::json!({
        "definition": definition,
        "dependencies": [
            {
                "definition_class": "profile",
                "reference": "example.profile.root@1.0.0",
                "fingerprint": root["profile_fingerprint"].as_str().unwrap(),
            },
            {
                "definition_class": "vocabulary",
                "reference": "example.vocabulary.hcm-3-1@1.0.0",
                "fingerprint": "sha256:0a28353460ce60a0fc53ba5e99ea2ec753abf94638489fe1eebd69b317d90ec4",
            }
        ]
    }))
    .unwrap()
    .to_string()
    .into();
    std::fs::write(
        repo.path().join("sources/hcm-3-1.profile.yaml"),
        serde_yaml_bw::to_string(&child).unwrap(),
    )
    .unwrap();

    let mut selection = request(false);
    selection.selected_profile_ref = r("example.profile.hcm-3-1@1.0.0");
    selection.profile_sources[0].source =
        DefinitionSource::RepositoryPath("sources/root.yaml".into());
    selection.profile_sources.push(DefinitionSourceBinding {
        definition_ref: r("example.profile.hcm-3-1@1.0.0"),
        source: DefinitionSource::RepositoryPath("sources/hcm-3-1.profile.yaml".into()),
    });
    selection.vocabulary_sources.push(DefinitionSourceBinding {
        definition_ref: r("example.vocabulary.hcm-3-1@1.0.0"),
        source: DefinitionSource::RepositoryPath("sources/hcm-3-1.vocabulary.yaml".into()),
    });

    let resolved = resolve_profile_selection(repo.path(), selection).unwrap();
    assert_eq!(
        resolved.vocabulary().exact_ref().as_str(),
        "example.vocabulary.hcm-3-1@1.0.0"
    );
    assert_eq!(
        resolved.vocabulary().vocabulary_fingerprint().as_str(),
        "sha256:0a28353460ce60a0fc53ba5e99ea2ec753abf94638489fe1eebd69b317d90ec4"
    );
    assert_ne!(
        resolved.resolved_profile_fingerprint(),
        baseline.resolved_profile_fingerprint()
    );
    assert_eq!(
        resolved.artifact_instances().fingerprint(),
        baseline.artifact_instances().fingerprint(),
        "vocabulary selection cannot change artifact machine identity"
    );
    assert_eq!(
        resolved
            .vocabulary()
            .resolve_typed_role("delivery_unit")
            .unwrap()
            .role_id(),
        "delivery_unit"
    );
}

#[test]
fn compound_invalid_fixtures_preserve_all_ten_fail_fast_stages() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));

    // Stage 1 is a typed construction boundary: an invalid scalar cannot be
    // placed in the closed request at all, irrespective of a later duplicate.
    assert!(ExactDefinitionRef::parse(" example.profile.root@1.0.0").is_err());

    // Stage 2 wins over Stage 3 in either source order.
    for reverse in [false, true] {
        let mut selection = request(false);
        let mut duplicates = vec![
            DefinitionSourceBinding {
                definition_ref: r("example.profile.duplicate@1.0.0"),
                source: DefinitionSource::RepositoryPath("sources/missing.yaml".into()),
            },
            DefinitionSourceBinding {
                definition_ref: r("example.profile.duplicate@1.0.0"),
                source: DefinitionSource::RepositoryPath("../invalid.yaml".into()),
            },
        ];
        if reverse {
            duplicates.reverse();
        }
        selection.profile_sources.extend(duplicates);
        let failure = resolve_profile_selection(crate_root, selection).unwrap_err();
        assert_eq!(failure.kind(), ProfileLoadErrorKind::DuplicateSourceBinding);
        assert_eq!(failure.location(), None);
    }

    // Stage 3 wins over the Stage 4 missing source in either source order.
    for reverse in [false, true] {
        let mut selection = request(false);
        let mut invalid = vec![
            DefinitionSourceBinding {
                definition_ref: r("example.profile.invalid-path@1.0.0"),
                source: DefinitionSource::RepositoryPath("../invalid.yaml".into()),
            },
            DefinitionSourceBinding {
                definition_ref: r("example.profile.missing@1.0.0"),
                source: DefinitionSource::RepositoryPath("sources/missing.yaml".into()),
            },
        ];
        if reverse {
            invalid.reverse();
        }
        selection.profile_sources.extend(invalid);
        let failure = resolve_profile_selection(crate_root, selection).unwrap_err();
        assert_eq!(failure.kind(), ProfileLoadErrorKind::InvalidSourcePath);
        assert_eq!(failure.location(), Some("repository_path"));
    }

    // Stage 4 reads finish before Stage 5 record decoding begins.
    for reverse in [false, true] {
        let repo = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(repo.path().join("sources")).unwrap();
        std::fs::write(repo.path().join("sources/malformed.yaml"), b"[:\n").unwrap();
        let mut selection = request(false);
        let mut sources = vec![
            DefinitionSourceBinding {
                definition_ref: r("example.profile.missing@1.0.0"),
                source: DefinitionSource::RepositoryPath("sources/missing.yaml".into()),
            },
            DefinitionSourceBinding {
                definition_ref: r("example.profile.malformed@1.0.0"),
                source: DefinitionSource::RepositoryPath("sources/malformed.yaml".into()),
            },
        ];
        if reverse {
            sources.reverse();
        }
        selection.profile_sources = sources;
        let failure = resolve_profile_selection(repo.path(), selection).unwrap_err();
        assert_eq!(failure.kind(), ProfileLoadErrorKind::MissingSource);
        assert_eq!(failure.location(), Some("sources/missing.yaml"));
    }

    // Stage 5 closed-record decoding wins over a Stage 6 self-cycle.
    let stage_five_repo = tempfile::tempdir().unwrap();
    let stage_five = repository_root_request(stage_five_repo.path());
    let stage_five_path = stage_five_repo.path().join("sources/root.yaml");
    let mut stage_five_profile: serde_json::Value =
        serde_yaml_bw::from_slice(&std::fs::read(&stage_five_path).unwrap()).unwrap();
    stage_five_profile["extends_profile_ref"] = serde_json::json!("example.profile.root@1.0.0");
    stage_five_profile["unknown"] = serde_json::json!(true);
    std::fs::write(
        &stage_five_path,
        serde_yaml_bw::to_string(&stage_five_profile).unwrap(),
    )
    .unwrap();
    let failure = resolve_profile_selection(stage_five_repo.path(), stage_five).unwrap_err();
    assert_eq!(failure.kind(), ProfileLoadErrorKind::UnknownProfileField);
    assert_eq!(failure.location(), None);

    // Stage 6 ancestry wins over Stage 7 schema-root refusal.
    let stage_six_repo = tempfile::tempdir().unwrap();
    let mut stage_six = repository_root_request(stage_six_repo.path());
    let stage_six_path = stage_six_repo.path().join("sources/root.yaml");
    let mut stage_six_profile: serde_json::Value =
        serde_yaml_bw::from_slice(&std::fs::read(&stage_six_path).unwrap()).unwrap();
    stage_six_profile["extends_profile_ref"] = serde_json::json!("example.profile.root@1.0.0");
    std::fs::write(
        &stage_six_path,
        serde_yaml_bw::to_string(&stage_six_profile).unwrap(),
    )
    .unwrap();
    stage_six.allowed_schema_roots.clear();
    let failure = resolve_profile_selection(stage_six_repo.path(), stage_six).unwrap_err();
    assert_eq!(failure.kind(), ProfileLoadErrorKind::ProfileAncestryCycle);
    assert_eq!(failure.location(), None);

    // Stage 7 schema closure wins over the Stage 8 authored-profile
    // fingerprint even when both are independently invalid.
    let stage_seven_repo = tempfile::tempdir().unwrap();
    let mut stage_seven = repository_root_request(stage_seven_repo.path());
    let stage_seven_path = stage_seven_repo.path().join("sources/root.yaml");
    let mut stage_seven_profile: serde_json::Value =
        serde_yaml_bw::from_slice(&std::fs::read(&stage_seven_path).unwrap()).unwrap();
    stage_seven_profile["profile_fingerprint"] = serde_json::json!(
        "sha256:0000000000000000000000000000000000000000000000000000000000000000"
    );
    std::fs::write(
        &stage_seven_path,
        serde_yaml_bw::to_string(&stage_seven_profile).unwrap(),
    )
    .unwrap();
    stage_seven.allowed_schema_roots.clear();
    let failure = resolve_profile_selection(stage_seven_repo.path(), stage_seven).unwrap_err();
    assert_eq!(
        failure.kind(),
        ProfileLoadErrorKind::Registry(RegistryLoadErrorKind::LocalReferenceOutsideRoot)
    );
    assert_eq!(failure.location(), None);

    // Stage 8 stale fingerprints win over Stage 9 unused-source accounting.
    for reverse in [false, true] {
        let repo = tempfile::tempdir().unwrap();
        let mut stage_eight = repository_root_request(repo.path());
        let root_path = repo.path().join("sources/root.yaml");
        let mut profile: serde_json::Value =
            serde_yaml_bw::from_slice(&std::fs::read(&root_path).unwrap()).unwrap();
        profile["profile_fingerprint"] = serde_json::json!(
            "sha256:0000000000000000000000000000000000000000000000000000000000000000"
        );
        std::fs::write(&root_path, serde_yaml_bw::to_string(&profile).unwrap()).unwrap();
        std::fs::copy(
            crate_root.join("tests/fixtures/hcm_1_2_repository_profile/repository.yaml"),
            repo.path().join("sources/unrelated.yaml"),
        )
        .unwrap();
        stage_eight.profile_sources.push(DefinitionSourceBinding {
            definition_ref: r("example.profile.repository@1.0.0"),
            source: DefinitionSource::RepositoryPath("sources/unrelated.yaml".into()),
        });
        if reverse {
            stage_eight.profile_sources.reverse();
        }
        let failure = resolve_profile_selection(repo.path(), stage_eight).unwrap_err();
        assert_eq!(failure.kind(), ProfileLoadErrorKind::FingerprintMismatch);
        assert_eq!(failure.location(), None);
    }

    // A valid Stage 8 profile with both an unused source and a forbidden
    // later-owned field proves Stage 9 precedes Stage 10. Removing the unused
    // source exposes the exact Stage 10 error.
    let stage_nine_repo = tempfile::tempdir().unwrap();
    let baseline = resolve_profile_selection(crate_root, request(false)).unwrap();
    let mut stage_nine = repository_root_request(stage_nine_repo.path());
    let stage_nine_path = stage_nine_repo.path().join("sources/root.yaml");
    let mut stage_nine_profile: serde_json::Value =
        serde_yaml_bw::from_slice(&std::fs::read(&stage_nine_path).unwrap()).unwrap();
    stage_nine_profile["projection_catalog_refs"] =
        serde_json::json!(["example.projection.forbidden@1.0.0"]);
    let condition = ProjectConditionDefinition::load(
        crate_root,
        "definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml",
    )
    .unwrap();
    let instances = ArtifactInstanceRegistry::resolve(
        stage_nine_profile["artifact_instances"].as_array().unwrap(),
        baseline.artifact_kind_registry(),
        &[&condition],
    )
    .unwrap();
    close_root_profile_fingerprint(&mut stage_nine_profile, &baseline, &instances);
    std::fs::write(
        &stage_nine_path,
        serde_yaml_bw::to_string(&stage_nine_profile).unwrap(),
    )
    .unwrap();
    std::fs::copy(
        crate_root.join("tests/fixtures/hcm_1_2_repository_profile/repository.yaml"),
        stage_nine_repo.path().join("sources/unrelated.yaml"),
    )
    .unwrap();
    stage_nine.profile_sources.push(DefinitionSourceBinding {
        definition_ref: r("example.profile.repository@1.0.0"),
        source: DefinitionSource::RepositoryPath("sources/unrelated.yaml".into()),
    });
    let mut stage_ten = stage_nine.clone();
    stage_ten.profile_sources.pop();
    let failure = resolve_profile_selection(stage_nine_repo.path(), stage_nine).unwrap_err();
    assert_eq!(failure.kind(), ProfileLoadErrorKind::UnreferencedSource);
    assert_eq!(failure.location(), None);
    let failure = resolve_profile_selection(stage_nine_repo.path(), stage_ten).unwrap_err();
    assert_eq!(
        failure.kind(),
        ProfileLoadErrorKind::Registry(RegistryLoadErrorKind::UnsupportedDependency)
    );
    assert_eq!(failure.location(), None);
}

#[test]
fn stage_five_nested_profile_field_decode_precedes_stage_six_cycle() {
    for (field, malformed) in [
        ("stable_role_registry", serde_json::json!("not-a-binding")),
        ("schema_registry_sources", serde_json::json!("not-a-list")),
        ("artifact_kind_sources", serde_json::json!([1])),
        ("artifact_instances", serde_json::json!([{}])),
        ("vocabulary_ref", serde_json::json!({})),
        ("context_resolution_ref", serde_json::json!(1)),
        (
            "projection_catalog_refs",
            serde_json::json!(["not-an-exact-ref"]),
        ),
        ("posture_evaluation_policy", serde_json::json!([])),
        ("dock_requirement_refs", serde_json::json!([false])),
        ("adapter_overlay_refs", serde_json::json!({})),
        ("extensions", serde_json::json!([])),
    ] {
        let repo = tempfile::tempdir().unwrap();
        let selection = repository_root_request(repo.path());
        let path = repo.path().join("sources/root.yaml");
        let mut profile: serde_json::Value =
            serde_yaml_bw::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        profile[field] = malformed;
        profile["extends_profile_ref"] = serde_json::json!("example.profile.root@1.0.0");
        std::fs::write(&path, serde_yaml_bw::to_string(&profile).unwrap()).unwrap();
        let failure = resolve_profile_selection(repo.path(), selection).unwrap_err();
        assert_eq!(
            failure.kind(),
            ProfileLoadErrorKind::InvalidProfileRecord,
            "{field}"
        );
        assert_eq!(failure.location(), Some(field), "{field}");
    }
}

#[test]
fn non_object_profile_records_refuse_at_stage_five_before_unreferenced_source_checks() {
    for (case, bytes) in [
        ("null", b"null\n".as_slice()),
        ("scalar", b"profile\n".as_slice()),
        ("sequence", b"- profile\n".as_slice()),
    ] {
        for reverse in [false, true] {
            let repo = tempfile::tempdir().unwrap();
            let mut selection = repository_root_request(repo.path());
            std::fs::write(repo.path().join("sources/malformed.yaml"), bytes).unwrap();
            selection.profile_sources.push(DefinitionSourceBinding {
                definition_ref: r("example.profile.aaa-malformed@1.0.0"),
                source: DefinitionSource::RepositoryPath("sources/malformed.yaml".into()),
            });
            if reverse {
                selection.profile_sources.reverse();
            }

            let error = resolve_profile_selection(repo.path(), selection).unwrap_err();
            assert_eq!(
                error.kind(),
                ProfileLoadErrorKind::InvalidProfileRecord,
                "{case} reverse={reverse}"
            );
            assert_eq!(
                error.location(),
                Some("profile_source"),
                "{case} reverse={reverse}"
            );
        }
    }
}

#[test]
fn stage_five_nested_kind_capability_decode_precedes_stage_six_cycle() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    for reverse in [false, true] {
        let repo = tempfile::tempdir().unwrap();
        let mut selection = repository_root_request(repo.path());
        let profile_path = repo.path().join("sources/root.yaml");
        let mut profile: serde_json::Value =
            serde_yaml_bw::from_slice(&std::fs::read(&profile_path).unwrap()).unwrap();
        profile["extends_profile_ref"] = serde_json::json!("example.profile.root@1.0.0");
        std::fs::write(&profile_path, serde_yaml_bw::to_string(&profile).unwrap()).unwrap();

        let mut kind: serde_json::Value = serde_yaml_bw::from_slice(
            &std::fs::read(crate_root.join(
                "definitions/artifact-kinds/handbook.artifact-kind.project-authority/1.0.0.yaml",
            ))
            .unwrap(),
        )
        .unwrap();
        kind["kind_id"] = serde_json::json!("example.artifact-kind.malformed-capability");
        kind["semantic_capabilities"][0]["unexpected"] = serde_json::json!(true);
        let kind_path = repo.path().join("sources/malformed-capability.kind.yaml");
        std::fs::write(&kind_path, serde_yaml_bw::to_string(&kind).unwrap()).unwrap();
        selection
            .artifact_kind_sources
            .push(DefinitionSourceBinding {
                definition_ref: r("example.artifact-kind.malformed-capability@1.0.0"),
                source: DefinitionSource::RepositoryPath(
                    "sources/malformed-capability.kind.yaml".into(),
                ),
            });
        if reverse {
            selection.artifact_kind_sources.reverse();
        }

        let failure = resolve_profile_selection(repo.path(), selection).unwrap_err();
        assert_eq!(
            failure.kind(),
            ProfileLoadErrorKind::Registry(RegistryLoadErrorKind::UnknownField)
        );
        assert_eq!(failure.location(), None);
    }
}

fn custom_stale_schema_request(
    repo: &Path,
    structural_closure_failure: bool,
    stale_stable_selection: bool,
) -> ProfileSelectionRequest {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut selection = repository_root_request(repo);
    let profile_path = repo.join("sources/root.yaml");
    let mut profile: serde_json::Value =
        serde_yaml_bw::from_slice(&std::fs::read(&profile_path).unwrap()).unwrap();
    profile["schema_registry_sources"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!("example.schemas.stale@1.0.0"));
    if stale_stable_selection {
        profile["stable_role_registry"]["fingerprint"] = serde_json::json!(
            "sha256:0000000000000000000000000000000000000000000000000000000000000000"
        );
    }
    std::fs::write(&profile_path, serde_yaml_bw::to_string(&profile).unwrap()).unwrap();

    let mut document: serde_json::Value = serde_json::from_slice(
        &std::fs::read(crate_root.join(
            "definitions/schemas/handbook.schemas.artifacts.project-context/1.0.0.schema.json",
        ))
        .unwrap(),
    )
    .unwrap();
    if structural_closure_failure {
        document["properties"]["missingClosureMember"] =
            serde_json::json!({"$ref":"missing.schema.json"});
    }
    std::fs::write(
        repo.join("sources/custom.schema.json"),
        serde_json::to_vec_pretty(&document).unwrap(),
    )
    .unwrap();

    let mut entry: serde_json::Value = serde_yaml_bw::from_slice(
        &std::fs::read(crate_root.join(
            "definitions/schemas/handbook.schemas.artifacts.project-context/1.0.0.entry.yaml",
        ))
        .unwrap(),
    )
    .unwrap();
    entry["content_schema_id"] = serde_json::json!("example.schemas.stale");
    entry["document_ref"] = serde_json::json!("sources/custom.schema.json");
    for field in [
        "document_fingerprint",
        "closure_fingerprint",
        "entry_fingerprint",
    ] {
        entry[field] = serde_json::json!(
            "sha256:0000000000000000000000000000000000000000000000000000000000000000"
        );
    }
    std::fs::write(
        repo.join("sources/custom.entry.yaml"),
        serde_yaml_bw::to_string(&entry).unwrap(),
    )
    .unwrap();
    selection
        .schema_entry_sources
        .push(DefinitionSourceBinding {
            definition_ref: r("example.schemas.stale@1.0.0"),
            source: DefinitionSource::RepositoryPath("sources/custom.entry.yaml".into()),
        });
    selection.allowed_schema_roots.push("sources".into());
    selection
}

#[test]
fn stage_eight_stable_then_schema_and_structural_closure_precedence_are_exact() {
    let stable_repo = tempfile::tempdir().unwrap();
    let stable_first = custom_stale_schema_request(stable_repo.path(), false, true);
    let failure = resolve_profile_selection(stable_repo.path(), stable_first).unwrap_err();
    assert_eq!(failure.kind(), ProfileLoadErrorKind::FingerprintMismatch);
    assert_eq!(failure.location(), None);
    assert_eq!(
        failure.detail(),
        "stable role registry fingerprint mismatch"
    );

    let schema_repo = tempfile::tempdir().unwrap();
    let schema_second = custom_stale_schema_request(schema_repo.path(), false, false);
    let failure = resolve_profile_selection(schema_repo.path(), schema_second).unwrap_err();
    assert_eq!(
        failure.kind(),
        ProfileLoadErrorKind::Registry(RegistryLoadErrorKind::FingerprintMismatch)
    );
    assert_eq!(failure.location(), Some("document_fingerprint"));

    let closure_repo = tempfile::tempdir().unwrap();
    let closure_first = custom_stale_schema_request(closure_repo.path(), true, false);
    let failure = resolve_profile_selection(closure_repo.path(), closure_first).unwrap_err();
    assert_eq!(
        failure.kind(),
        ProfileLoadErrorKind::Registry(RegistryLoadErrorKind::LocalReferenceMissing)
    );
    assert_eq!(failure.location(), Some("schema_document"));
}

fn missing_condition_before_vocabulary_request(repo: &Path) -> ProfileSelectionRequest {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut selection = vocabulary_role_mismatch_request(repo, true);
    let path = repo.join("sources/root.yaml");
    let mut profile: serde_json::Value =
        serde_yaml_bw::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    let shipped: serde_json::Value = serde_yaml_bw::from_slice(
        &std::fs::read(crate_root.join("tests/fixtures/hcm_1_2_repository_profile/root.yaml"))
            .unwrap(),
    )
    .unwrap();
    let mut descriptor = shipped["artifact_instances"][1].clone();
    descriptor["requiredness"] = serde_json::json!({
        "mode":"conditional",
        "condition_ref":"handbook.condition.project.managed-operational-surface@1.0.0"
    });
    profile["artifact_instances"] = serde_json::json!([descriptor]);
    std::fs::write(&path, serde_yaml_bw::to_string(&profile).unwrap()).unwrap();

    selection.project_condition_sources.clear();
    selection
}

fn missing_stack_request(repo: &Path, duplicate_descriptor: bool) -> ProfileSelectionRequest {
    let mut selection = repository_root_request(repo);
    let path = repo.join("sources/root.yaml");
    let mut profile: serde_json::Value =
        serde_yaml_bw::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    if duplicate_descriptor {
        let duplicate = profile["artifact_instances"][0].clone();
        profile["artifact_instances"]
            .as_array_mut()
            .unwrap()
            .push(duplicate);
    }
    std::fs::write(&path, serde_yaml_bw::to_string(&profile).unwrap()).unwrap();
    selection.context_resolution_sources.clear();
    selection
}

#[test]
fn stage_eight_compound_fixtures_cover_each_remaining_producer_boundary() {
    // Schema fingerprint precedes the missing validator producer.
    let schema_repo = tempfile::tempdir().unwrap();
    let mut schema_first = custom_stale_schema_request(schema_repo.path(), false, false);
    schema_first.semantic_validator_sources.clear();
    let failure = resolve_profile_selection(schema_repo.path(), schema_first).unwrap_err();
    assert_eq!(
        failure.kind(),
        ProfileLoadErrorKind::Registry(RegistryLoadErrorKind::FingerprintMismatch)
    );
    assert_eq!(failure.location(), Some("document_fingerprint"));

    // Validator source presence is checked before capability source presence.
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut validator_first = request(false);
    validator_first.semantic_validator_sources.clear();
    validator_first.semantic_capability_sources.clear();
    let failure = resolve_profile_selection(crate_root, validator_first).unwrap_err();
    assert_eq!(failure.kind(), ProfileLoadErrorKind::MissingSource);
    assert_eq!(failure.detail(), "missing exact typed definition source");

    // Capability source presence precedes the custom kind fingerprint.
    let capability_repo = tempfile::tempdir().unwrap();
    let mut capability_first = vocabulary_role_mismatch_request(capability_repo.path(), true);
    capability_first.semantic_capability_sources.clear();
    let kind_path = capability_repo
        .path()
        .join("sources/project-context.kind.yaml");
    let mut kind: serde_json::Value =
        serde_yaml_bw::from_slice(&std::fs::read(&kind_path).unwrap()).unwrap();
    kind["definition_fingerprint"] = serde_json::json!(
        "sha256:0000000000000000000000000000000000000000000000000000000000000000"
    );
    std::fs::write(&kind_path, serde_yaml_bw::to_string(&kind).unwrap()).unwrap();
    let failure = resolve_profile_selection(capability_repo.path(), capability_first).unwrap_err();
    assert_eq!(failure.kind(), ProfileLoadErrorKind::MissingSource);

    // The custom kind fingerprint precedes both condition and vocabulary.
    let kind_repo = tempfile::tempdir().unwrap();
    let kind_first = missing_condition_before_vocabulary_request(kind_repo.path());
    let kind_path = kind_repo.path().join("sources/project-context.kind.yaml");
    let mut kind: serde_json::Value =
        serde_yaml_bw::from_slice(&std::fs::read(&kind_path).unwrap()).unwrap();
    kind["definition_fingerprint"] = serde_json::json!(
        "sha256:0000000000000000000000000000000000000000000000000000000000000000"
    );
    std::fs::write(&kind_path, serde_yaml_bw::to_string(&kind).unwrap()).unwrap();
    let failure = resolve_profile_selection(kind_repo.path(), kind_first).unwrap_err();
    assert_eq!(
        failure.kind(),
        ProfileLoadErrorKind::Registry(RegistryLoadErrorKind::FingerprintMismatch)
    );
    assert_eq!(
        failure.detail(),
        "artifact-kind definition fingerprint does not match its exact typed closure"
    );

    // With the kind correct, the missing condition producer wins over the known
    // vocabulary/stable-role mismatch in the same compound request.
    let condition_repo = tempfile::tempdir().unwrap();
    let condition_first = missing_condition_before_vocabulary_request(condition_repo.path());
    let failure = resolve_profile_selection(condition_repo.path(), condition_first).unwrap_err();
    assert_eq!(failure.kind(), ProfileLoadErrorKind::MissingSource);
    assert_eq!(failure.detail(), "missing exact typed definition source");

    // Vocabulary mismatch precedes a missing policy producer.
    let vocabulary_repo = tempfile::tempdir().unwrap();
    let mut vocabulary_first = vocabulary_role_mismatch_request(vocabulary_repo.path(), true);
    vocabulary_first.context_resolution_policy_sources.clear();
    let failure = resolve_profile_selection(vocabulary_repo.path(), vocabulary_first).unwrap_err();
    assert_eq!(
        failure.kind(),
        ProfileLoadErrorKind::Registry(RegistryLoadErrorKind::StableRoleRegistryMismatch)
    );

    // A missing policy producer precedes a missing stack producer.
    let policy_repo = tempfile::tempdir().unwrap();
    let mut policy_first = missing_stack_request(policy_repo.path(), true);
    policy_first.context_resolution_policy_sources.clear();
    let failure = resolve_profile_selection(policy_repo.path(), policy_first).unwrap_err();
    assert_eq!(failure.kind(), ProfileLoadErrorKind::MissingSource);

    // The missing stack precedes the invalid descriptor closure.
    let stack_repo = tempfile::tempdir().unwrap();
    let stack_first = missing_stack_request(stack_repo.path(), true);
    let failure = resolve_profile_selection(stack_repo.path(), stack_first).unwrap_err();
    assert_eq!(failure.kind(), ProfileLoadErrorKind::MissingSource);
    assert_eq!(failure.detail(), "missing exact typed definition source");

    // The descriptor closure precedes the authored profile fingerprint.
    let descriptor_repo = tempfile::tempdir().unwrap();
    let descriptor_path = descriptor_repo.path().join("sources/root.yaml");
    let descriptor_first = repository_root_request(descriptor_repo.path());
    let mut profile: serde_json::Value =
        serde_yaml_bw::from_slice(&std::fs::read(&descriptor_path).unwrap()).unwrap();
    let duplicate = profile["artifact_instances"][0].clone();
    profile["artifact_instances"]
        .as_array_mut()
        .unwrap()
        .push(duplicate);
    std::fs::write(
        &descriptor_path,
        serde_yaml_bw::to_string(&profile).unwrap(),
    )
    .unwrap();
    let failure = resolve_profile_selection(descriptor_repo.path(), descriptor_first).unwrap_err();
    assert_eq!(
        failure.kind(),
        ProfileLoadErrorKind::Registry(RegistryLoadErrorKind::DuplicateIdentity)
    );

    // Root-to-leaf authored profile validation is invariant to source order.
    for reverse in [false, true] {
        let repo = tempfile::tempdir().unwrap();
        let mut selection = subset_child_request(repo.path(), true);
        let root_path = repo.path().join("sources/root.yaml");
        let mut root: serde_json::Value =
            serde_yaml_bw::from_slice(&std::fs::read(&root_path).unwrap()).unwrap();
        root["profile_fingerprint"] = serde_json::json!(
            "sha256:0000000000000000000000000000000000000000000000000000000000000000"
        );
        std::fs::write(&root_path, serde_yaml_bw::to_string(&root).unwrap()).unwrap();
        if reverse {
            selection.profile_sources.reverse();
        }
        let failure = resolve_profile_selection(repo.path(), selection).unwrap_err();
        assert_eq!(failure.kind(), ProfileLoadErrorKind::FingerprintMismatch);
        assert_eq!(failure.location(), None);
    }
}
