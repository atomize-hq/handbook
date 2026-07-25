use handbook_engine::*;
use serde_json::{json, Value};
use std::path::Path;

const DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";
const ROLE_REGISTRY_REF: &str = "handbook.roles.core@1.1.0";
const ROLE_REGISTRY_FINGERPRINT: &str =
    "sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029";

fn exact(value: &str) -> ExactDefinitionRef {
    ExactDefinitionRef::parse(value).expect("exact ref")
}

fn sym(value: &str) -> SymbolicId {
    SymbolicId::parse(value).expect("symbolic id")
}

fn builtin(value: &str) -> DefinitionSourceBinding {
    let reference = exact(value);
    DefinitionSourceBinding {
        definition_ref: reference.clone(),
        source: DefinitionSource::BuiltIn(reference),
    }
}

#[test]
fn custom_kind_and_instance_are_selected_profile_data_not_enum_variants() {
    let profile = custom_profile(false, false);
    let registry = ResolvedArtifactRegistry::from_profile(&profile).expect("custom registry");

    assert_eq!(
        registry.profile_ref().as_str(),
        "example.profile.registry-root@1.0.0"
    );
    assert_eq!(
        registry
            .kind_refs()
            .into_iter()
            .map(ExactDefinitionRef::as_str)
            .collect::<Vec<_>>(),
        [
            "example.artifact-kind.registry-brief@1.0.0",
            "handbook.artifact-kind.project-authority@1.0.0",
        ]
    );
    assert_eq!(
        registry
            .instance_ids()
            .into_iter()
            .map(SymbolicId::as_str)
            .collect::<Vec<_>>(),
        ["project_authority", "registry_brief"]
    );

    let custom_kind = registry
        .kind(&exact("example.artifact-kind.registry-brief@1.0.0"))
        .expect("custom kind");
    assert_eq!(
        custom_kind.canonical_schema_ref().as_str(),
        "example.schemas.registry-brief@1.0.0"
    );
    assert_eq!(custom_kind.supported_role_ids(), ["project_context"]);
    assert!(custom_kind.capabilities().is_empty());

    let custom = registry
        .instance(&sym("registry_brief"))
        .expect("custom instance");
    assert_eq!(
        custom.kind_ref().as_str(),
        "example.artifact-kind.registry-brief@1.0.0"
    );
    assert_eq!(custom.role().expect("role").role_id(), "project_context");
    assert_eq!(custom.label(), "Registry Brief");
    assert_eq!(
        custom.canonical_path(),
        ".handbook/project/registry-brief.yaml"
    );
    assert_eq!(custom.requiredness_mode(), RequirednessMode::Always);
    assert!(custom.capabilities().is_empty());
}

#[test]
fn dependencies_bind_provider_sets_and_provider_before_consumer_order() {
    let profile = custom_profile(true, false);
    let registry = ResolvedArtifactRegistry::from_profile(&profile).expect("dependency registry");

    assert_eq!(
        registry
            .instance_ids()
            .into_iter()
            .map(SymbolicId::as_str)
            .collect::<Vec<_>>(),
        [
            "alpha_note",
            "bravo_note",
            "project_authority",
            "registry_brief",
        ]
    );
    let registry_brief = registry
        .instance(&sym("registry_brief"))
        .expect("registry brief");
    assert_eq!(registry_brief.dependencies().len(), 2);
    let constitutional = &registry_brief.dependencies()[0];
    assert_eq!(
        constitutional.target_kind(),
        DependencyTargetKind::Capability
    );
    assert_eq!(constitutional.target_ref().as_str(), "constitutional_root");
    assert_eq!(
        constitutional
            .target_contract_ref()
            .expect("contract")
            .as_str(),
        "handbook.capabilities.constitutional-root@1.0.0"
    );
    assert_eq!(
        constitutional
            .provider_ids()
            .iter()
            .map(SymbolicId::as_str)
            .collect::<Vec<_>>(),
        ["project_authority"]
    );
    let instance_dependency = &registry_brief.dependencies()[1];
    assert_eq!(
        instance_dependency.target_kind(),
        DependencyTargetKind::Instance
    );
    assert_eq!(instance_dependency.target_ref().as_str(), "bravo_note");
    assert_eq!(
        instance_dependency
            .provider_ids()
            .iter()
            .map(SymbolicId::as_str)
            .collect::<Vec<_>>(),
        ["bravo_note"]
    );
    assert_eq!(
        registry
            .instance(&sym("bravo_note"))
            .expect("bravo")
            .dependencies()[0]
            .provider_ids()
            .iter()
            .map(SymbolicId::as_str)
            .collect::<Vec<_>>(),
        ["alpha_note"]
    );
    assert_eq!(
        registry
            .dependency_order()
            .iter()
            .map(SymbolicId::as_str)
            .collect::<Vec<_>>(),
        [
            "alpha_note",
            "bravo_note",
            "project_authority",
            "registry_brief",
        ]
    );
}

#[test]
fn validation_routes_by_instance_bound_kind_and_uses_typed_unknown_error() {
    let profile = custom_profile(false, false);
    let registry = ResolvedArtifactRegistry::from_profile(&profile).expect("registry");

    registry
        .validate_json(
            &sym("registry_brief"),
            &json!({"title": "Registry Brief", "summary": "Descriptor owned"}),
        )
        .expect("custom schema accepts valid content");

    let unknown = registry
        .validate_json(
            &sym("missing_instance"),
            &json!({"title": "Registry Brief"}),
        )
        .unwrap_err();
    assert_eq!(
        unknown,
        ArtifactRegistryValidationError::UnknownArtifactInstance
    );

    let malformed = registry
        .validate_json(&sym("registry_brief"), &json!({"title": "no"}))
        .unwrap_err();
    match malformed {
        ArtifactRegistryValidationError::Structural(errors) => {
            assert!(!errors.is_empty(), "short title must be structural");
        }
        other => panic!("expected structural validation error, got {other:?}"),
    }

    let wrong_kind_shape = registry
        .validate_json(
            &sym("registry_brief"),
            &json!({
                "schema_id": "handbook.artifact.project-authority",
                "schema_version": "1.0",
                "record_id": "authority.example",
                "policy": {"revision": "r1", "authority_statement": "text"},
                "governance": {
                    "decision_authority": ["team"],
                    "required_approvals": ["team"],
                    "exception_policy": "none",
                    "review_triggers": ["change"],
                    "reassessment_triggers": ["change"]
                },
                "engineering_posture": {
                    "dimensions": ["quality"],
                    "red_lines": ["none"]
                }
            }),
        )
        .unwrap_err();
    assert!(
        matches!(
            wrong_kind_shape,
            ArtifactRegistryValidationError::Structural(_)
        ),
        "custom instance must route only to its custom kind schema"
    );
}

#[test]
fn request_source_permutations_have_identical_registry_projection() {
    let forward = custom_profile(true, false);
    let reversed = custom_profile(true, true);
    let forward = ResolvedArtifactRegistry::from_profile(&forward).expect("forward registry");
    let reversed = ResolvedArtifactRegistry::from_profile(&reversed).expect("reversed registry");
    assert_eq!(
        registry_projection(&forward),
        registry_projection(&reversed)
    );
}

fn shipped_request(reverse: bool) -> ProfileSelectionRequest {
    let names = [
        "project-authority",
        "project-context",
        "environment-context",
        "work-specification",
        "decision-record",
        "risk-record",
    ];
    let mut schema_entry_sources = names
        .map(|name| builtin(&format!("handbook.schemas.artifacts.{name}@1.0.0")))
        .to_vec();
    let mut artifact_kind_sources = names
        .map(|name| builtin(&format!("handbook.artifact-kind.{name}@1.0.0")))
        .to_vec();
    if reverse {
        schema_entry_sources.reverse();
        artifact_kind_sources.reverse();
    }
    ProfileSelectionRequest {
        selected_profile_ref: exact("handbook.profile.shipped-root@1.0.0"),
        profile_sources: vec![builtin("handbook.profile.shipped-root@1.0.0")],
        stable_role_registry_sources: vec![builtin("handbook.roles.core@1.1.0")],
        schema_entry_sources,
        artifact_kind_sources,
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
        allowed_schema_roots: vec!["definitions/schemas".to_string()],
    }
}

fn shipped_profile() -> ResolvedInstanceProfile {
    resolve_profile_selection(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        shipped_request(false),
    )
    .expect("shipped profile")
}

fn assert_public_type<T>() {}

fn repo_source(definition_ref: &str, source_path: &str) -> DefinitionSourceBinding {
    DefinitionSourceBinding {
        definition_ref: exact(definition_ref),
        source: DefinitionSource::RepositoryPath(source_path.to_string()),
    }
}

fn write(path: &Path, bytes: impl AsRef<[u8]>) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(path, bytes).expect("write fixture");
}

fn copy_crate_file(repo: &Path, relative_path: &str) {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let target = repo.join(relative_path);
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).expect("create copied parent");
    }
    std::fs::copy(crate_root.join(relative_path), target).expect("copy crate fixture");
}

fn fingerprint(value: &Value) -> String {
    DefinitionFingerprint::from_json_value(value)
        .expect("fingerprint")
        .to_string()
}

fn write_custom_schema_entry(repo: &Path) -> (String, String, String) {
    let document_ref = "schemas/registry-brief.schema.json";
    let entry_path = "sources/registry-brief.schema-entry.yaml";
    let schema = json!({
        "$schema": DIALECT,
        "type": "object",
        "properties": {
            "title": {"type": "string", "minLength": 3},
            "summary": {"type": "string", "minLength": 1}
        },
        "required": ["title"],
        "additionalProperties": false,
        "unevaluatedProperties": false
    });
    let schema_bytes = serde_json::to_vec_pretty(&schema).expect("schema bytes");
    write(&repo.join(document_ref), &schema_bytes);
    let document_fingerprint = DefinitionFingerprint::from_bytes(&schema_bytes).to_string();
    let closure_fingerprint = fingerprint(&json!([{
        "document_ref": document_ref,
        "document_fingerprint": document_fingerprint,
    }]));
    let entry = json!({
        "schema_id": "handbook.schema-registry-entry",
        "schema_version": "1.0",
        "content_schema_id": "example.schemas.registry-brief",
        "content_schema_version": "1.0.0",
        "document_ref": document_ref,
        "document_fingerprint": document_fingerprint,
        "closure_fingerprint": closure_fingerprint,
        "meta_schema_ref": DIALECT,
        "media_type": "application/schema+json",
        "compatibility": "exact",
        "extensions": {},
    });
    let entry_fingerprint = fingerprint(&entry);
    let mut authored = entry.as_object().expect("entry object").clone();
    authored.insert(
        "entry_fingerprint".to_string(),
        Value::String(entry_fingerprint.clone()),
    );
    write(
        &repo.join(entry_path),
        serde_yaml_bw::to_string(&authored).expect("schema entry yaml"),
    );
    (
        entry_path.to_string(),
        "example.schemas.registry-brief@1.0.0".to_string(),
        entry_fingerprint,
    )
}

fn write_custom_kind(
    repo: &Path,
    schema_ref: &str,
    schema_entry_fingerprint: &str,
    schema_closure_fingerprint: &str,
) -> (String, String, String) {
    let path = "sources/registry-brief.kind.yaml";
    let mut record = json!({
        "schema_id": "handbook.artifact-kind-definition",
        "schema_version": "1.0",
        "kind_id": "example.artifact-kind.registry-brief",
        "kind_version": "1.0.0",
        "compatibility": "exact",
        "stable_role_registry": {
            "ref": ROLE_REGISTRY_REF,
            "fingerprint": ROLE_REGISTRY_FINGERPRINT,
        },
        "canonical_schema_ref": schema_ref,
        "supported_role_refs": ["project_context"],
        "semantic_capabilities": [],
        "structural_validation_profile_ref": "json-schema.draft-2020-12",
        "semantic_validation_profile_refs": [],
        "renderer_definition_refs": [],
        "projection_definition_refs": [],
        "lifecycle_policy_ref": null,
        "review_triggers": [],
        "required_capabilities": [],
        "extensions": {},
    });
    let definition_fingerprint = fingerprint(&json!({
        "definition": record,
        "stable_role_registry_fingerprint": ROLE_REGISTRY_FINGERPRINT,
        "schema_entry_fingerprint": schema_entry_fingerprint,
        "schema_closure_fingerprint": schema_closure_fingerprint,
    }));
    record.as_object_mut().expect("kind object").insert(
        "definition_fingerprint".to_string(),
        Value::String(definition_fingerprint.clone()),
    );
    write(
        &repo.join(path),
        serde_yaml_bw::to_string(&record).expect("kind yaml"),
    );
    (
        path.to_string(),
        "example.artifact-kind.registry-brief@1.0.0".to_string(),
        definition_fingerprint,
    )
}

fn project_authority_descriptor() -> Value {
    shipped_root_artifact_instance_values()
        .into_iter()
        .find(|value| value["id"] == json!("project_authority"))
        .expect("project authority descriptor")
}

fn custom_descriptor(id: &str, label: &str, path: &str, dependencies: Value) -> Value {
    json!({
        "schema_id": "handbook.artifact-instance-descriptor",
        "schema_version": "1.0",
        "id": id,
        "kind_ref": "example.artifact-kind.registry-brief@1.0.0",
        "role_ref": "project_context",
        "capability_refs": [],
        "label": label,
        "canonical_path": path,
        "requiredness": {
            "mode": "always",
            "condition_ref": null
        },
        "depends_on": dependencies,
        "lifecycle_policy_ref": null,
        "intake_definition_ref": null,
        "renderer_definition_refs": [],
        "projection_definition_refs": [],
        "validation_overlay_refs": [],
        "extensions": {},
    })
}

fn selected_instances(with_dependencies: bool) -> Vec<Value> {
    if with_dependencies {
        vec![
            project_authority_descriptor(),
            custom_descriptor(
                "alpha_note",
                "Alpha Note",
                ".handbook/project/alpha-note.yaml",
                json!([]),
            ),
            custom_descriptor(
                "bravo_note",
                "Bravo Note",
                ".handbook/project/bravo-note.yaml",
                json!([{
                    "target_kind": "instance",
                    "target_ref": "alpha_note",
                    "target_contract_ref": null,
                    "cardinality": "exactly_one"
                }]),
            ),
            custom_descriptor(
                "registry_brief",
                "Registry Brief",
                ".handbook/project/registry-brief.yaml",
                json!([
                    {
                        "target_kind": "capability",
                        "target_ref": "constitutional_root",
                        "target_contract_ref": "handbook.capabilities.constitutional-root@1.0.0",
                        "cardinality": "exactly_one"
                    },
                    {
                        "target_kind": "instance",
                        "target_ref": "bravo_note",
                        "target_contract_ref": null,
                        "cardinality": "exactly_one"
                    }
                ]),
            ),
        ]
    } else {
        vec![
            project_authority_descriptor(),
            custom_descriptor(
                "registry_brief",
                "Registry Brief",
                ".handbook/project/registry-brief.yaml",
                json!([]),
            ),
        ]
    }
}

fn close_registry_profile_fingerprint(
    value: &mut Value,
    baseline: &ResolvedInstanceProfile,
    schema_entry_fingerprint: &str,
    kind_definition_fingerprint: &str,
    instance_registry_fingerprint: &str,
) {
    let authority_schema_ref = exact("handbook.schemas.artifacts.project-authority@1.0.0");
    let authority_kind_ref = exact("handbook.artifact-kind.project-authority@1.0.0");
    let authority_schema = baseline
        .artifact_kind_registry()
        .schema_registry()
        .entry(&authority_schema_ref)
        .expect("authority schema");
    let authority_kind = baseline
        .artifact_kind_registry()
        .kind(&authority_kind_ref)
        .expect("authority kind");
    let mut dependencies = vec![
        json!({
            "definition_class": "stable_role_registry",
            "reference": baseline.stable_role_registry().exact_ref().as_str(),
            "fingerprint": baseline.stable_role_registry().fingerprint().as_str(),
        }),
        json!({
            "definition_class": "schema_entry",
            "reference": authority_schema_ref.as_str(),
            "fingerprint": authority_schema.entry_fingerprint().as_str(),
        }),
        json!({
            "definition_class": "schema_entry",
            "reference": "example.schemas.registry-brief@1.0.0",
            "fingerprint": schema_entry_fingerprint,
        }),
        json!({
            "definition_class": "artifact_kind",
            "reference": authority_kind_ref.as_str(),
            "fingerprint": authority_kind.definition_fingerprint().as_str(),
        }),
        json!({
            "definition_class": "artifact_kind",
            "reference": "example.artifact-kind.registry-brief@1.0.0",
            "fingerprint": kind_definition_fingerprint,
        }),
        json!({
            "definition_class": "artifact_instance_registry",
            "reference": "example.profile.registry-root@1.0.0",
            "fingerprint": instance_registry_fingerprint,
        }),
        json!({
            "definition_class": "vocabulary",
            "reference": baseline.vocabulary().exact_ref().as_str(),
            "fingerprint": baseline.vocabulary().vocabulary_fingerprint().as_str(),
        }),
        json!({
            "definition_class": "context_resolution",
            "reference": baseline.context_resolution().exact_ref().as_str(),
            "fingerprint": baseline.context_resolution().definition_fingerprint().as_str(),
        }),
    ];
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
        .expect("profile object")
        .remove("profile_fingerprint");
    value["profile_fingerprint"] = Value::String(fingerprint(&json!({
        "definition": definition,
        "dependencies": dependencies,
    })));
}

fn write_custom_profile(repo: &Path, with_dependencies: bool) -> ProfileSelectionRequest {
    for relative_path in [
        "definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.entry.yaml",
        "definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.schema.json",
        "definitions/artifact-kinds/handbook.artifact-kind.project-authority/1.0.0.yaml",
        "definitions/semantic-capabilities/handbook.capabilities.constitutional-root/1.0.0.yaml",
        "definitions/semantic-validators/handbook.semantic-validation.constitutional-root/1.0.0.yaml",
    ] {
        copy_crate_file(repo, relative_path);
    }

    let (schema_entry_path, schema_ref, schema_entry_fingerprint) = write_custom_schema_entry(repo);
    let schema_registry = SchemaRegistry::load(
        repo,
        std::slice::from_ref(&schema_entry_path),
        &["schemas".to_string()],
    )
    .expect("custom schema registry");
    let schema_closure_fingerprint = schema_registry
        .entry(&exact(&schema_ref))
        .expect("custom schema entry")
        .closure_fingerprint()
        .to_string();
    let (kind_path, kind_ref, kind_definition_fingerprint) = write_custom_kind(
        repo,
        &schema_ref,
        &schema_entry_fingerprint,
        &schema_closure_fingerprint,
    );

    let baseline = shipped_profile();
    let combined_kinds = load_artifact_kind_registry(
        repo,
        ArtifactKindRegistryLoadRequest::new(
            exact(ROLE_REGISTRY_REF),
            vec![
                "definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.entry.yaml"
                    .to_string(),
                schema_entry_path.clone(),
            ],
            vec!["definitions/schemas".to_string(), "schemas".to_string()],
            vec![
                "definitions/artifact-kinds/handbook.artifact-kind.project-authority/1.0.0.yaml"
                    .to_string(),
                kind_path.clone(),
            ],
        )
        .with_semantic_sources(
            vec![
                "definitions/semantic-capabilities/handbook.capabilities.constitutional-root/1.0.0.yaml"
                    .to_string(),
            ],
            vec![
                "definitions/semantic-validators/handbook.semantic-validation.constitutional-root/1.0.0.yaml"
                    .to_string(),
            ],
        ),
    )
    .expect("combined kind registry");
    let instances = selected_instances(with_dependencies);
    let instance_registry = ArtifactInstanceRegistry::resolve(&instances, &combined_kinds, &[])
        .expect("custom instance registry");
    let mut profile = json!({
        "schema_id": "handbook.instance-profile",
        "schema_version": "1.0",
        "profile_id": "example.profile.registry-root",
        "profile_version": "1.0.0",
        "profile_scope": "shipped",
        "extends_profile_ref": null,
        "stable_role_registry": {
            "ref": ROLE_REGISTRY_REF,
            "fingerprint": ROLE_REGISTRY_FINGERPRINT,
        },
        "schema_registry_sources": [
            "handbook.schemas.artifacts.project-authority@1.0.0",
            schema_ref.as_str()
        ],
        "artifact_kind_sources": [
            "handbook.artifact-kind.project-authority@1.0.0",
            kind_ref.as_str()
        ],
        "artifact_instances": instances,
        "vocabulary_ref": baseline.vocabulary().exact_ref().as_str(),
        "context_resolution_ref": baseline.context_resolution().exact_ref().as_str(),
        "projection_catalog_refs": [],
        "posture_evaluation_policy": null,
        "dock_requirement_refs": [],
        "adapter_overlay_refs": [],
        "extensions": {},
        "profile_fingerprint": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
    });
    close_registry_profile_fingerprint(
        &mut profile,
        &baseline,
        &schema_entry_fingerprint,
        &kind_definition_fingerprint,
        instance_registry.fingerprint().as_str(),
    );
    write(
        &repo.join("sources/registry-root.profile.yaml"),
        serde_yaml_bw::to_string(&profile).expect("profile yaml"),
    );

    ProfileSelectionRequest {
        selected_profile_ref: exact("example.profile.registry-root@1.0.0"),
        profile_sources: vec![repo_source(
            "example.profile.registry-root@1.0.0",
            "sources/registry-root.profile.yaml",
        )],
        stable_role_registry_sources: vec![builtin(ROLE_REGISTRY_REF)],
        schema_entry_sources: vec![
            builtin("handbook.schemas.artifacts.project-authority@1.0.0"),
            repo_source(&schema_ref, &schema_entry_path),
        ],
        artifact_kind_sources: vec![
            builtin("handbook.artifact-kind.project-authority@1.0.0"),
            repo_source(&kind_ref, &kind_path),
        ],
        semantic_capability_sources: vec![builtin(
            "handbook.capabilities.constitutional-root@1.0.0",
        )],
        semantic_validator_sources: vec![builtin(
            "handbook.semantic-validation.constitutional-root@1.0.0",
        )],
        project_condition_sources: vec![],
        vocabulary_sources: vec![builtin("handbook.vocabulary.shipped-root@1.0.0")],
        context_resolution_sources: vec![builtin("handbook.context-resolution.shipped-root@1.0.0")],
        context_resolution_policy_sources: vec![
            builtin("handbook.mutation-matcher.core@1.0.0"),
            builtin("handbook.resolution-escalation.core@1.0.0"),
            builtin("handbook.memory-promotion.core@1.0.0"),
        ],
        intake_definition_sources: vec![],
        allowed_schema_roots: vec!["definitions/schemas".to_string(), "schemas".to_string()],
    }
}

fn custom_profile(with_dependencies: bool, reverse_sources: bool) -> ResolvedInstanceProfile {
    let repo = tempfile::tempdir().expect("temp repo");
    let mut request = write_custom_profile(repo.path(), with_dependencies);
    if reverse_sources {
        request.schema_entry_sources.reverse();
        request.artifact_kind_sources.reverse();
        request.context_resolution_policy_sources.reverse();
    }
    resolve_profile_selection(repo.path(), request).expect("custom profile")
}

fn registry_projection(registry: &ResolvedArtifactRegistry) -> Value {
    json!({
        "profile_ref": registry.profile_ref().as_str(),
        "profile_fingerprint": registry.profile_fingerprint().as_str(),
        "kinds": registry
            .kind_refs()
            .into_iter()
            .map(ExactDefinitionRef::as_str)
            .collect::<Vec<_>>(),
        "instances": registry
            .instance_ids()
            .into_iter()
            .map(SymbolicId::as_str)
            .collect::<Vec<_>>(),
        "dependency_order": registry
            .dependency_order()
            .iter()
            .map(SymbolicId::as_str)
            .collect::<Vec<_>>(),
    })
}

#[test]
fn registry_public_api_shape_and_shipped_membership_are_selected_profile_owned() {
    assert_public_type::<ResolvedArtifactRegistry>();
    assert_public_type::<ResolvedArtifactKind>();
    assert_public_type::<ResolvedArtifactInstance>();
    assert_public_type::<ResolvedArtifactCapability>();
    assert_public_type::<ResolvedArtifactDependency>();
    assert_public_type::<ArtifactRegistryValidationError>();

    let profile = shipped_profile();
    let registry = ResolvedArtifactRegistry::from_profile(&profile).expect("registry");

    assert_eq!(registry.profile_ref(), profile.exact_ref());
    assert_eq!(
        registry.profile_fingerprint(),
        profile.resolved_profile_fingerprint()
    );
    assert_eq!(
        registry.stable_role_registry_ref(),
        profile.stable_role_registry().exact_ref()
    );
    assert_eq!(
        registry.stable_role_registry_fingerprint(),
        profile.stable_role_registry().fingerprint()
    );
    assert_eq!(
        registry
            .kind_refs()
            .into_iter()
            .map(ExactDefinitionRef::as_str)
            .collect::<Vec<_>>(),
        [
            "handbook.artifact-kind.decision-record@1.0.0",
            "handbook.artifact-kind.environment-context@1.0.0",
            "handbook.artifact-kind.project-authority@1.0.0",
            "handbook.artifact-kind.project-context@1.0.0",
            "handbook.artifact-kind.risk-record@1.0.0",
            "handbook.artifact-kind.work-specification@1.0.0",
        ]
    );
    assert_eq!(
        registry
            .instance_ids()
            .into_iter()
            .map(SymbolicId::as_str)
            .collect::<Vec<_>>(),
        [
            "environment_context",
            "project_authority",
            "project_context",
        ]
    );
}

#[test]
fn registry_exposes_shipped_kind_schema_capability_and_instance_closure_data() {
    let profile = shipped_profile();
    let registry = ResolvedArtifactRegistry::from_profile(&profile).expect("registry");

    let authority_kind_ref = exact("handbook.artifact-kind.project-authority@1.0.0");
    let authority_kind = registry.kind(&authority_kind_ref).expect("authority kind");
    let source_kind = profile
        .artifact_kind_registry()
        .kind(&authority_kind_ref)
        .expect("source authority kind");
    let source_schema = profile
        .artifact_kind_registry()
        .schema_registry()
        .entry(source_kind.canonical_schema_ref())
        .expect("source authority schema");

    assert_eq!(authority_kind.exact_ref(), &authority_kind_ref);
    assert_eq!(
        authority_kind.definition_fingerprint().as_str(),
        "sha256:276c9f5a9686f9f8648db829f83d97730245062a5ddc47b6b3eccd087b8ce42b"
    );
    assert_eq!(
        authority_kind.canonical_schema_ref().as_str(),
        "handbook.schemas.artifacts.project-authority@1.0.0"
    );
    assert_eq!(
        authority_kind.schema_entry_fingerprint(),
        source_schema.entry_fingerprint()
    );
    assert_eq!(
        authority_kind.schema_document_fingerprint(),
        source_schema.document_fingerprint()
    );
    assert_eq!(
        authority_kind.schema_closure_fingerprint(),
        source_schema.closure_fingerprint()
    );
    assert_eq!(
        authority_kind.supported_role_ids(),
        ["constitutional_authority"]
    );

    let kind_capability = authority_kind
        .capabilities()
        .iter()
        .find(|capability| capability.capability_id().as_str() == "constitutional_root")
        .expect("constitutional kind capability");
    assert_eq!(
        kind_capability.contract_ref().as_str(),
        "handbook.capabilities.constitutional-root@1.0.0"
    );
    assert_eq!(
        kind_capability.contract_fingerprint().as_str(),
        "sha256:1d4a1c2f85158c14524559e6846bf805eff4e531d8a78ac5c4890e2a4c0b0998"
    );
    assert_eq!(
        kind_capability
            .required_bindings()
            .iter()
            .map(SymbolicId::as_str)
            .collect::<Vec<_>>(),
        [
            "policy_root",
            "policy_revision",
            "decision_authority",
            "required_approvals",
            "exception_policy",
            "engineering_posture_dimensions",
            "red_lines",
            "review_triggers",
            "reassessment_triggers",
        ]
    );
    assert_eq!(
        kind_capability.allowed_instance_cardinality(),
        AllowedInstanceCardinality::ExactlyOne
    );
    assert_eq!(
        kind_capability
            .bindings()
            .get(&sym("policy_root"))
            .expect("policy_root binding"),
        "/policy"
    );
    assert_eq!(kind_capability.semantic_validators().len(), 1);
    assert_eq!(
        kind_capability.semantic_validators()[0]
            .exact_ref()
            .as_str(),
        "handbook.semantic-validation.constitutional-root@1.0.0"
    );
    assert_eq!(
        kind_capability.semantic_validators()[0]
            .profile_fingerprint()
            .as_str(),
        "sha256:be0fb9fd4ee98e9fc1c384b710d61198e103f2bca6ac6ef2bbe14957808c9738"
    );

    let authority = registry
        .instance(&sym("project_authority"))
        .expect("authority instance");
    assert_eq!(authority.id().as_str(), "project_authority");
    assert_eq!(authority.kind_ref(), &authority_kind_ref);
    assert_eq!(
        authority.role().expect("role").role_id(),
        "constitutional_authority"
    );
    assert_eq!(authority.capabilities().len(), 1);
    assert_eq!(
        authority.capabilities()[0].capability_id().as_str(),
        "constitutional_root"
    );
    assert_eq!(authority.label(), "Charter");
    assert_eq!(authority.canonical_path(), ".handbook/project/charter.yaml");
    assert_eq!(authority.requiredness_mode(), RequirednessMode::Always);
    assert_eq!(authority.condition_ref(), None);
    assert!(authority.dependencies().is_empty());
    assert_eq!(authority.lifecycle_policy_ref(), None);
    assert_eq!(authority.intake_definition_ref(), None);
    assert!(authority.renderer_definition_refs().is_empty());
    assert!(authority.projection_definition_refs().is_empty());
    assert!(authority.validation_overlay_refs().is_empty());
    assert!(authority.extensions().is_empty());

    let environment = registry
        .instance(&sym("environment_context"))
        .expect("environment instance");
    assert_eq!(
        environment.kind_ref().as_str(),
        "handbook.artifact-kind.environment-context@1.0.0"
    );
    assert_eq!(
        environment.role().expect("role").role_id(),
        "environment_context"
    );
    assert_eq!(
        environment.canonical_path(),
        ".handbook/project/environment.yaml"
    );
    assert_eq!(
        environment.requiredness_mode(),
        RequirednessMode::Conditional
    );
    assert_eq!(
        environment.condition_ref().expect("condition").as_str(),
        "handbook.condition.project.managed-operational-surface@1.0.0"
    );

    for empty_kind_ref in [
        "handbook.artifact-kind.decision-record@1.0.0",
        "handbook.artifact-kind.environment-context@1.0.0",
        "handbook.artifact-kind.project-context@1.0.0",
        "handbook.artifact-kind.risk-record@1.0.0",
        "handbook.artifact-kind.work-specification@1.0.0",
    ] {
        assert!(
            registry
                .kind(&exact(empty_kind_ref))
                .expect(empty_kind_ref)
                .capabilities()
                .is_empty(),
            "{empty_kind_ref} must not inherit constitutional capability"
        );
    }
}
