use handbook_engine::{
    load_artifact_kind_registry, parse_definition_yaml, ArtifactInstanceRegistry,
    ArtifactKindRegistry, ArtifactKindRegistryLoadRequest, ContextResolutionPolicyRegistry,
    ContextResolutionStackDefinition, DefinitionFingerprint, DefinitionSource,
    DefinitionSourceBinding, ExactDefinitionRef, ProfileSelectionRequest,
    ProjectConditionDefinition, SchemaRegistry, VocabularyDefinition,
};
use serde_json::Value;
use std::fs;
use std::path::Path;

const DEFINITION_PATHS: [&str; 16] = [
    "artifact-kinds/handbook.artifact-kind.project-context/1.1.0.yaml",
    "artifact-kinds/handbook.artifact-kind.environment-context/1.1.0.yaml",
    "artifact-kinds/handbook.artifact-kind.work-specification/1.1.0.yaml",
    "artifact-kinds/handbook.artifact-kind.decision-record/1.1.0.yaml",
    "artifact-kinds/handbook.artifact-kind.risk-record/1.1.0.yaml",
    "intakes/handbook.intake.project-context/1.0.0.yaml",
    "intakes/handbook.intake.environment-context/1.0.0.yaml",
    "intakes/handbook.intake.work-specification/1.0.0.yaml",
    "intakes/handbook.intake.decision-record/1.0.0.yaml",
    "intakes/handbook.intake.risk-record/1.0.0.yaml",
    "renderers/handbook.renderer.project-context-review-markdown/1.0.0.yaml",
    "renderers/handbook.renderer.environment-context-review-markdown/1.0.0.yaml",
    "renderers/handbook.renderer.work-specification-review-markdown/1.0.0.yaml",
    "renderers/handbook.renderer.decision-record-review-markdown/1.0.0.yaml",
    "renderers/handbook.renderer.risk-record-review-markdown/1.0.0.yaml",
    "profiles/handbook.profile.shipped-root/1.2.0.yaml",
];
const BUILTIN_SOURCES: [(&str, &str); 16] = [
    (
        "handbook.artifact-kind.project-context@1.1.0",
        "artifact-kinds/handbook.artifact-kind.project-context/1.1.0.yaml",
    ),
    (
        "handbook.artifact-kind.environment-context@1.1.0",
        "artifact-kinds/handbook.artifact-kind.environment-context/1.1.0.yaml",
    ),
    (
        "handbook.artifact-kind.work-specification@1.1.0",
        "artifact-kinds/handbook.artifact-kind.work-specification/1.1.0.yaml",
    ),
    (
        "handbook.artifact-kind.decision-record@1.1.0",
        "artifact-kinds/handbook.artifact-kind.decision-record/1.1.0.yaml",
    ),
    (
        "handbook.artifact-kind.risk-record@1.1.0",
        "artifact-kinds/handbook.artifact-kind.risk-record/1.1.0.yaml",
    ),
    (
        "handbook.intake.project-context@1.0.0",
        "intakes/handbook.intake.project-context/1.0.0.yaml",
    ),
    (
        "handbook.intake.environment-context@1.0.0",
        "intakes/handbook.intake.environment-context/1.0.0.yaml",
    ),
    (
        "handbook.intake.work-specification@1.0.0",
        "intakes/handbook.intake.work-specification/1.0.0.yaml",
    ),
    (
        "handbook.intake.decision-record@1.0.0",
        "intakes/handbook.intake.decision-record/1.0.0.yaml",
    ),
    (
        "handbook.intake.risk-record@1.0.0",
        "intakes/handbook.intake.risk-record/1.0.0.yaml",
    ),
    (
        "handbook.renderer.project-context-review-markdown@1.0.0",
        "renderers/handbook.renderer.project-context-review-markdown/1.0.0.yaml",
    ),
    (
        "handbook.renderer.environment-context-review-markdown@1.0.0",
        "renderers/handbook.renderer.environment-context-review-markdown/1.0.0.yaml",
    ),
    (
        "handbook.renderer.work-specification-review-markdown@1.0.0",
        "renderers/handbook.renderer.work-specification-review-markdown/1.0.0.yaml",
    ),
    (
        "handbook.renderer.decision-record-review-markdown@1.0.0",
        "renderers/handbook.renderer.decision-record-review-markdown/1.0.0.yaml",
    ),
    (
        "handbook.renderer.risk-record-review-markdown@1.0.0",
        "renderers/handbook.renderer.risk-record-review-markdown/1.0.0.yaml",
    ),
    (
        "handbook.profile.shipped-root@1.2.0",
        "profiles/handbook.profile.shipped-root/1.2.0.yaml",
    ),
];
const DEFINITION_VECTOR_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.4/contracts/definition-support-vectors-v1.0.json"
);
const RENDERER_GOLDEN_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.4/contracts/renderer-goldens-v1.0.json"
);
const P1A_PROOF_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P1A-definition-support.md"
);

fn exact_successor_kind_registry() -> ArtifactKindRegistry {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let kind_versions = [
        ("project-authority", "1.1.0"),
        ("project-context", "1.1.0"),
        ("environment-context", "1.1.0"),
        ("work-specification", "1.1.0"),
        ("decision-record", "1.1.0"),
        ("risk-record", "1.1.0"),
    ];
    let schema_versions = [
        ("project-authority", "1.1.0"),
        ("project-context", "1.0.0"),
        ("environment-context", "1.0.0"),
        ("work-specification", "1.0.0"),
        ("decision-record", "1.0.0"),
        ("risk-record", "1.0.0"),
    ];
    let request = ArtifactKindRegistryLoadRequest::new(
        ExactDefinitionRef::parse("handbook.roles.core@1.1.0").expect("role registry ref"),
        schema_versions
            .iter()
            .map(|(slug, version)| {
                format!(
                    "definitions/schemas/handbook.schemas.artifacts.{slug}/{version}.entry.yaml"
                )
            })
            .collect(),
        vec!["definitions/schemas".to_owned()],
        kind_versions
            .iter()
            .map(|(slug, version)| {
                format!(
                    "definitions/artifact-kinds/handbook.artifact-kind.{slug}/{version}.yaml"
                )
            })
            .collect(),
    )
    .with_semantic_sources(
        vec![
            "definitions/semantic-capabilities/handbook.capabilities.constitutional-root/1.0.0.yaml"
                .to_owned(),
        ],
        vec![
            "definitions/semantic-validators/handbook.semantic-validation.constitutional-root/1.0.0.yaml"
                .to_owned(),
            "definitions/semantic-validators/handbook.semantic-validation.constitutional-root/1.1.0.yaml"
                .to_owned(),
        ],
    );
    load_artifact_kind_registry(repository_root, request).expect("load exact successor registry")
}

fn builtin(reference: &str) -> DefinitionSourceBinding {
    let definition_ref = ExactDefinitionRef::parse(reference).expect("built-in ref");
    DefinitionSourceBinding {
        definition_ref: definition_ref.clone(),
        source: DefinitionSource::BuiltIn(definition_ref),
    }
}

fn shipped_profile_1_2_request() -> ProfileSelectionRequest {
    let schema_refs = [
        "handbook.schemas.artifacts.project-authority@1.0.0",
        "handbook.schemas.artifacts.project-authority@1.1.0",
        "handbook.schemas.artifacts.project-context@1.0.0",
        "handbook.schemas.artifacts.environment-context@1.0.0",
        "handbook.schemas.artifacts.work-specification@1.0.0",
        "handbook.schemas.artifacts.decision-record@1.0.0",
        "handbook.schemas.artifacts.risk-record@1.0.0",
        "handbook.schemas.lifecycle.trigger-evidence@1.0.0",
        "handbook.schemas.security.approver-registry@1.0.0",
        "handbook.schemas.security.approver-registry-transition@1.0.0",
        "handbook.schemas.security.authenticator-registration-request@1.0.0",
        "handbook.schemas.security.authenticator-challenge@1.0.0",
        "handbook.schemas.security.authenticator-make-credential-response@1.0.0",
        "handbook.schemas.security.authenticator-registration@1.0.0",
        "handbook.schemas.security.authenticator-get-assertion-response@1.0.0",
        "handbook.schemas.security.authenticator-assertion@1.0.0",
        "handbook.schemas.security.approver-admin-api@1.0.0",
    ];
    let kind_refs = [
        "handbook.artifact-kind.project-authority@1.0.0",
        "handbook.artifact-kind.project-authority@1.1.0",
        "handbook.artifact-kind.project-context@1.0.0",
        "handbook.artifact-kind.project-context@1.1.0",
        "handbook.artifact-kind.environment-context@1.0.0",
        "handbook.artifact-kind.environment-context@1.1.0",
        "handbook.artifact-kind.work-specification@1.0.0",
        "handbook.artifact-kind.work-specification@1.1.0",
        "handbook.artifact-kind.decision-record@1.0.0",
        "handbook.artifact-kind.decision-record@1.1.0",
        "handbook.artifact-kind.risk-record@1.0.0",
        "handbook.artifact-kind.risk-record@1.1.0",
    ];
    ProfileSelectionRequest {
        selected_profile_ref: ExactDefinitionRef::parse("handbook.profile.shipped-root@1.2.0")
            .expect("profile ref"),
        profile_sources: vec![builtin("handbook.profile.shipped-root@1.2.0")],
        stable_role_registry_sources: vec![builtin("handbook.roles.core@1.1.0")],
        schema_entry_sources: schema_refs
            .iter()
            .map(|reference| builtin(reference))
            .collect(),
        artifact_kind_sources: kind_refs
            .iter()
            .map(|reference| builtin(reference))
            .collect(),
        semantic_capability_sources: vec![builtin(
            "handbook.capabilities.constitutional-root@1.0.0",
        )],
        semantic_validator_sources: vec![
            builtin("handbook.semantic-validation.constitutional-root@1.0.0"),
            builtin("handbook.semantic-validation.constitutional-root@1.1.0"),
        ],
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
        allowed_schema_roots: vec!["definitions/schemas".to_owned()],
    }
}

#[test]
fn exact_p1a_definition_set_is_published_and_duplicate_safe() {
    let definition_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("definitions");

    for relative_path in DEFINITION_PATHS {
        let bytes = fs::read(definition_root.join(relative_path))
            .unwrap_or_else(|error| panic!("{relative_path}: {error}"));
        parse_definition_yaml(&bytes).unwrap_or_else(|error| panic!("{relative_path}: {error}"));
    }
}

#[test]
fn exact_p1a_definition_set_is_compile_time_allowlisted() {
    let builtins = include_str!("../src/profile_builtins.rs");
    for (reference, relative_path) in BUILTIN_SOURCES {
        assert!(
            builtins.contains(&format!("\"{reference}\"")),
            "missing built-in exact ref {reference}"
        );
        assert!(
            builtins.contains(&format!("\"{relative_path}\"")),
            "missing built-in package path {relative_path}"
        );
    }
}

#[test]
fn intake_and_renderer_fingerprints_cover_exact_authored_content() {
    let definition_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("definitions");
    let mut mismatches = Vec::new();

    for slug in [
        "project-context",
        "environment-context",
        "work-specification",
        "decision-record",
        "risk-record",
    ] {
        let schema_ref = format!("handbook.schemas.artifacts.{slug}@1.0.0");
        let kind_ref = format!("handbook.artifact-kind.{slug}@1.1.0");
        let schema = parse_definition_yaml(
            &fs::read(definition_root.join(format!(
                "schemas/handbook.schemas.artifacts.{slug}/1.0.0.entry.yaml"
            )))
            .expect("read schema entry"),
        )
        .expect("parse schema entry");
        let kind = parse_definition_yaml(
            &fs::read(definition_root.join(format!(
                "artifact-kinds/handbook.artifact-kind.{slug}/1.1.0.yaml"
            )))
            .expect("read kind"),
        )
        .expect("parse kind");

        let intake_path = format!("intakes/handbook.intake.{slug}/1.0.0.yaml");
        let mut intake = parse_definition_yaml(
            &fs::read(definition_root.join(&intake_path)).expect("read intake"),
        )
        .expect("parse intake");
        let supplied = intake
            .as_object_mut()
            .and_then(|object| object.remove("intake_definition_fingerprint"))
            .and_then(|value| value.as_str().map(ToOwned::to_owned))
            .expect("intake fingerprint");
        let computed = DefinitionFingerprint::from_json_value(&serde_json::json!({
            "definition": intake,
            "resolved_dependencies": [
                {
                    "definition_fingerprint": kind["definition_fingerprint"],
                    "definition_ref": kind_ref,
                    "dependency_role": "artifact_kind",
                },
                {
                    "definition_fingerprint": schema["entry_fingerprint"],
                    "definition_ref": schema_ref,
                    "dependency_role": "candidate_schema",
                },
            ],
        }))
        .expect("compute fingerprint")
        .to_string();
        if supplied != computed {
            mismatches.push(format!("{intake_path}: {computed}"));
        }

        let renderer_path =
            format!("renderers/handbook.renderer.{slug}-review-markdown/1.0.0.yaml");
        let mut renderer = parse_definition_yaml(
            &fs::read(definition_root.join(&renderer_path)).expect("read renderer"),
        )
        .expect("parse renderer");
        let supplied = renderer
            .as_object_mut()
            .and_then(|object| object.remove("renderer_fingerprint"))
            .and_then(|value| value.as_str().map(ToOwned::to_owned))
            .expect("renderer fingerprint");
        let computed = DefinitionFingerprint::from_json_value(&serde_json::json!({
            "definition": renderer,
            "resolved_dependencies": [
                {
                    "definition_fingerprint": schema["entry_fingerprint"],
                    "definition_ref": schema_ref,
                    "dependency_role": "input_schema",
                },
            ],
        }))
        .expect("compute renderer fingerprint")
        .to_string();
        if supplied != computed {
            mismatches.push(format!("{renderer_path}: {computed}"));
        }
    }

    assert!(
        mismatches.is_empty(),
        "uniform intake/renderer fingerprint mismatches:\n{}",
        mismatches.join("\n")
    );
}

#[test]
fn intakes_exhaustively_cover_schema_fields_and_block_unknown_values() {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let expected_definition_fields = std::collections::BTreeSet::from([
        "schema_id",
        "schema_version",
        "intake_id",
        "intake_version",
        "artifact_kind_ref",
        "candidate_schema_ref",
        "supported_modes",
        "coverage",
        "approval_policy_ref",
        "reassessment_triggers",
        "extensions",
        "intake_definition_fingerprint",
    ]);
    for slug in [
        "project-context",
        "environment-context",
        "work-specification",
        "decision-record",
        "risk-record",
    ] {
        let schema: Value = serde_json::from_slice(
            &fs::read(repository_root.join(format!(
                "definitions/schemas/handbook.schemas.artifacts.{slug}/1.0.0.schema.json"
            )))
            .expect("read schema"),
        )
        .expect("parse schema");
        let intake = parse_definition_yaml(
            &fs::read(repository_root.join(format!(
                "definitions/intakes/handbook.intake.{slug}/1.0.0.yaml"
            )))
            .expect("read intake"),
        )
        .expect("parse intake");
        assert_eq!(
            intake
                .as_object()
                .expect("intake object")
                .keys()
                .map(String::as_str)
                .collect::<std::collections::BTreeSet<_>>(),
            expected_definition_fields,
            "{slug}: closed definition fields"
        );
        assert_eq!(
            intake["artifact_kind_ref"],
            format!("handbook.artifact-kind.{slug}@1.1.0")
        );
        assert_eq!(
            intake["candidate_schema_ref"],
            format!("handbook.schemas.artifacts.{slug}@1.0.0")
        );
        assert_eq!(
            intake["supported_modes"],
            serde_json::json!(["guided_adaptive", "express", "agent_assisted"])
        );
        assert!(intake["approval_policy_ref"].is_null());
        assert_eq!(intake["reassessment_triggers"], serde_json::json!([]));
        assert_eq!(intake["extensions"], serde_json::json!({}));

        let expected_paths = schema["properties"]
            .as_object()
            .expect("schema properties")
            .keys()
            .map(|field| format!("/{field}"))
            .collect::<std::collections::BTreeSet<_>>();
        let rows = intake["coverage"].as_array().expect("coverage rows");
        let actual_paths = rows
            .iter()
            .flat_map(|row| {
                row["target_paths"]
                    .as_array()
                    .expect("target paths")
                    .iter()
                    .map(|path| path.as_str().expect("target path").to_owned())
            })
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(actual_paths, expected_paths, "{slug}: exhaustive coverage");
        assert_eq!(
            rows.iter()
                .flat_map(|row| row["target_paths"].as_array().expect("target paths"))
                .count(),
            actual_paths.len(),
            "{slug}: each schema field is covered exactly once"
        );
        for row in rows {
            assert_eq!(row["applicability"], "always", "{slug}");
            assert_eq!(row["authority_class"], "observational", "{slug}");
            assert_eq!(row["acquisition"]["inferable"], false, "{slug}");
            assert_eq!(
                row["acquisition"]["user_declaration_required"], true,
                "{slug}"
            );
            assert_eq!(row["acquisition"]["evidence_kinds"], serde_json::json!([]));
            assert!(row["acquisition"]["freshness"].is_null());
            assert_eq!(row["acquisition"]["sensitivity"], "internal");
            assert!(row["acquisition"]["deterministic_default"].is_null());
            assert_eq!(row["evaluation"]["required"], true);
            assert_eq!(row["evaluation"]["minimum_confidence"], "high");
            assert_eq!(row["evaluation"]["unknown_policy"], "block");
            assert_eq!(row["evaluation"]["contradiction_policy"], "block");
            assert!(row["evaluation"]["waiver_policy_ref"].is_null());
            assert_eq!(row["prompt_guidance_refs"], serde_json::json!([]));
        }
    }

    assert!(
        parse_definition_yaml(br#"{"schema_id":"first","schema_id":"duplicate"}"#).is_err(),
        "duplicate keys must fail before typed definition admission"
    );
}

#[test]
fn successor_kind_fingerprints_cover_their_resolved_schema_closures() {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cases = [
        (
            "project-context",
            "handbook.schemas.artifacts.project-context@1.0.0",
        ),
        (
            "environment-context",
            "handbook.schemas.artifacts.environment-context@1.0.0",
        ),
        (
            "work-specification",
            "handbook.schemas.artifacts.work-specification@1.0.0",
        ),
        (
            "decision-record",
            "handbook.schemas.artifacts.decision-record@1.0.0",
        ),
        (
            "risk-record",
            "handbook.schemas.artifacts.risk-record@1.0.0",
        ),
    ];
    let entry_paths = cases
        .iter()
        .map(|(slug, _)| {
            format!("definitions/schemas/handbook.schemas.artifacts.{slug}/1.0.0.entry.yaml")
        })
        .collect::<Vec<_>>();
    let schemas = SchemaRegistry::load(
        repository_root,
        &entry_paths,
        &["definitions/schemas".to_owned()],
    )
    .expect("load exact schema closure");
    let mut mismatches = Vec::new();

    for (slug, schema_ref) in cases {
        let relative_path =
            format!("definitions/artifact-kinds/handbook.artifact-kind.{slug}/1.1.0.yaml");
        let bytes = fs::read(repository_root.join(&relative_path)).expect("read kind");
        let mut definition = parse_definition_yaml(&bytes).expect("parse kind");
        let supplied = definition
            .as_object_mut()
            .and_then(|object| object.remove("definition_fingerprint"))
            .and_then(|value| value.as_str().map(ToOwned::to_owned))
            .expect("kind fingerprint");
        let schema = schemas
            .entry(&ExactDefinitionRef::parse(schema_ref).expect("schema ref"))
            .expect("schema entry");
        let renderer_ref = format!("handbook.renderer.{slug}-review-markdown@1.0.0");
        let renderer = parse_definition_yaml(
            &fs::read(repository_root.join(format!(
                "definitions/renderers/handbook.renderer.{slug}-review-markdown/1.0.0.yaml"
            )))
            .expect("read renderer"),
        )
        .expect("parse renderer");
        let computed = DefinitionFingerprint::from_json_value(&serde_json::json!({
            "definition": definition,
            "resolved_dependencies": [
                {
                    "definition_fingerprint": schema.entry_fingerprint().as_str(),
                    "definition_ref": schema_ref,
                    "dependency_role": "canonical_schema",
                    "schema_closure_fingerprint": schema.closure_fingerprint().as_str(),
                },
                {
                    "definition_fingerprint": renderer["renderer_fingerprint"],
                    "definition_ref": renderer_ref,
                    "dependency_role": "renderer",
                },
                {
                    "definition_fingerprint":
                        "sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029",
                    "definition_ref": "handbook.roles.core@1.1.0",
                    "dependency_role": "stable_role_registry",
                },
            ],
        }))
        .expect("compute kind closure")
        .to_string();
        if supplied != computed {
            mismatches.push(format!("{relative_path}: {computed}"));
        }
    }

    assert!(
        mismatches.is_empty(),
        "kind closure fingerprint mismatches:\n{}",
        mismatches.join("\n")
    );
}

#[test]
fn exact_successor_kind_renderer_pairs_are_admitted() {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let slugs = [
        "project-context",
        "environment-context",
        "work-specification",
        "decision-record",
        "risk-record",
    ];
    let request = ArtifactKindRegistryLoadRequest::new(
        ExactDefinitionRef::parse("handbook.roles.core@1.1.0").expect("role registry ref"),
        slugs
            .iter()
            .map(|slug| {
                format!("definitions/schemas/handbook.schemas.artifacts.{slug}/1.0.0.entry.yaml")
            })
            .collect(),
        vec!["definitions/schemas".to_owned()],
        slugs
            .iter()
            .map(|slug| {
                format!("definitions/artifact-kinds/handbook.artifact-kind.{slug}/1.1.0.yaml")
            })
            .collect(),
    );

    let registry =
        load_artifact_kind_registry(repository_root, request).expect("load exact successor kinds");
    assert_eq!(registry.kind_refs().len(), slugs.len());
}

#[test]
fn successor_kind_dependency_near_misses_remain_fail_closed() {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cases = [
        (
            "project-context",
            "handbook.renderer.project-context-review-markdown@1.0.0",
        ),
        (
            "environment-context",
            "handbook.renderer.environment-context-review-markdown@1.0.0",
        ),
        (
            "work-specification",
            "handbook.renderer.work-specification-review-markdown@1.0.0",
        ),
        (
            "decision-record",
            "handbook.renderer.decision-record-review-markdown@1.0.0",
        ),
        (
            "risk-record",
            "handbook.renderer.risk-record-review-markdown@1.0.0",
        ),
    ];

    for (slug, expected_renderer) in cases {
        let fixture = tempfile::tempdir().expect("fixture root");
        let schema_directory = format!("definitions/schemas/handbook.schemas.artifacts.{slug}");
        fs::create_dir_all(fixture.path().join(&schema_directory)).expect("schema directory");
        for suffix in ["1.0.0.entry.yaml", "1.0.0.schema.json"] {
            fs::copy(
                repository_root.join(format!("{schema_directory}/{suffix}")),
                fixture.path().join(format!("{schema_directory}/{suffix}")),
            )
            .expect("copy schema fixture");
        }
        let kind_path = format!("definitions/artifact-kinds/{slug}.yaml");
        fs::create_dir_all(fixture.path().join("definitions/artifact-kinds"))
            .expect("kind directory");
        let original = parse_definition_yaml(
            &fs::read(repository_root.join(format!(
                "definitions/artifact-kinds/handbook.artifact-kind.{slug}/1.1.0.yaml"
            )))
            .expect("read successor kind"),
        )
        .expect("parse successor kind");
        let mutations = [
            ("missing renderer", serde_json::json!([])),
            (
                "mismatched renderer",
                serde_json::json!(["example.renderer@1.0.0"]),
            ),
            (
                "extra renderer",
                serde_json::json!([expected_renderer, "example.renderer@1.0.0"]),
            ),
        ];
        for (label, renderer_refs) in mutations {
            let mut mutated = original.clone();
            mutated["renderer_definition_refs"] = renderer_refs;
            fs::write(
                fixture.path().join(&kind_path),
                serde_json::to_vec_pretty(&mutated).expect("encode mutation"),
            )
            .expect("write kind mutation");
            let error = load_artifact_kind_registry(
                fixture.path(),
                ArtifactKindRegistryLoadRequest::new(
                    ExactDefinitionRef::parse("handbook.roles.core@1.1.0").expect("role registry"),
                    vec![format!("{schema_directory}/1.0.0.entry.yaml")],
                    vec!["definitions/schemas".to_owned()],
                    vec![kind_path.clone()],
                ),
            )
            .expect_err(label);
            assert_eq!(
                error.kind(),
                handbook_engine::RegistryLoadErrorKind::UnsupportedDependency,
                "{slug}: {label}"
            );
        }
        for (label, pointer, value) in [
            (
                "projection",
                "/projection_definition_refs",
                serde_json::json!(["example.projection@1.0.0"]),
            ),
            (
                "lifecycle",
                "/lifecycle_policy_ref",
                serde_json::json!("example.lifecycle@1.0.0"),
            ),
            (
                "review trigger",
                "/review_triggers",
                serde_json::json!(["example.trigger@1.0.0"]),
            ),
            (
                "required capability",
                "/required_capabilities",
                serde_json::json!([{"capability_ref": "example.capability@1.0.0"}]),
            ),
            (
                "extension",
                "/extensions",
                serde_json::json!({"unexpected": true}),
            ),
        ] {
            let mut mutated = original.clone();
            *mutated.pointer_mut(pointer).expect("kind mutation pointer") = value;
            fs::write(
                fixture.path().join(&kind_path),
                serde_json::to_vec_pretty(&mutated).expect("encode mutation"),
            )
            .expect("write kind mutation");
            let error = load_artifact_kind_registry(
                fixture.path(),
                ArtifactKindRegistryLoadRequest::new(
                    ExactDefinitionRef::parse("handbook.roles.core@1.1.0").expect("role registry"),
                    vec![format!("{schema_directory}/1.0.0.entry.yaml")],
                    vec!["definitions/schemas".to_owned()],
                    vec![kind_path.clone()],
                ),
            )
            .expect_err(label);
            assert_eq!(
                error.kind(),
                handbook_engine::RegistryLoadErrorKind::UnsupportedDependency,
                "{slug}: {label}"
            );
        }
    }
}

#[test]
fn exact_first_party_descriptor_rows_are_admitted_and_near_misses_refused() {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let profile = parse_definition_yaml(
        &fs::read(
            repository_root.join("definitions/profiles/handbook.profile.shipped-root/1.2.0.yaml"),
        )
        .expect("read profile"),
    )
    .expect("parse profile");
    let charter = profile["artifact_instances"][0].clone();
    let descriptors = vec![
        profile["artifact_instances"][1].clone(),
        profile["artifact_instances"][2].clone(),
        serde_json::json!({
            "schema_id": "handbook.artifact-instance-descriptor",
            "schema_version": "1.0",
            "id": "work_specification",
            "kind_ref": "handbook.artifact-kind.work-specification@1.1.0",
            "role_ref": "delivery_unit",
            "capability_refs": [],
            "label": "Work Specification",
            "canonical_path": "artifacts/work-specification/work-specification.yaml",
            "requiredness": {"mode": "always", "condition_ref": null},
            "depends_on": [],
            "lifecycle_policy_ref": null,
            "intake_definition_ref": "handbook.intake.work-specification@1.0.0",
            "renderer_definition_refs":
                ["handbook.renderer.work-specification-review-markdown@1.0.0"],
            "projection_definition_refs": [],
            "validation_overlay_refs": [],
            "extensions": {}
        }),
        serde_json::json!({
            "schema_id": "handbook.artifact-instance-descriptor",
            "schema_version": "1.0",
            "id": "decision_record",
            "kind_ref": "handbook.artifact-kind.decision-record@1.1.0",
            "role_ref": null,
            "capability_refs": [],
            "label": "Decision Record",
            "canonical_path": ".handbook/records/decision.yaml",
            "requiredness": {"mode": "always", "condition_ref": null},
            "depends_on": [],
            "lifecycle_policy_ref": null,
            "intake_definition_ref": "handbook.intake.decision-record@1.0.0",
            "renderer_definition_refs":
                ["handbook.renderer.decision-record-review-markdown@1.0.0"],
            "projection_definition_refs": [],
            "validation_overlay_refs": [],
            "extensions": {}
        }),
        serde_json::json!({
            "schema_id": "handbook.artifact-instance-descriptor",
            "schema_version": "1.0",
            "id": "risk_record",
            "kind_ref": "handbook.artifact-kind.risk-record@1.1.0",
            "role_ref": null,
            "capability_refs": [],
            "label": "Risk Record",
            "canonical_path": ".handbook/records/risk.yaml",
            "requiredness": {"mode": "always", "condition_ref": null},
            "depends_on": [],
            "lifecycle_policy_ref": null,
            "intake_definition_ref": "handbook.intake.risk-record@1.0.0",
            "renderer_definition_refs":
                ["handbook.renderer.risk-record-review-markdown@1.0.0"],
            "projection_definition_refs": [],
            "validation_overlay_refs": [],
            "extensions": {}
        }),
    ];
    let condition = ProjectConditionDefinition::load(
        repository_root,
        "definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml",
    )
    .expect("load project condition");
    let kinds = exact_successor_kind_registry();

    for descriptor in descriptors {
        ArtifactInstanceRegistry::resolve(
            &[charter.clone(), descriptor.clone()],
            &kinds,
            &[&condition],
        )
        .expect("admit exact first-party descriptor row");

        let renderer = descriptor["renderer_definition_refs"][0].clone();
        let mismatches = [
            ("/schema_id", serde_json::json!("example.descriptor")),
            ("/schema_version", serde_json::json!("1.1")),
            ("/id", serde_json::json!("mismatched_id")),
            (
                "/kind_ref",
                serde_json::json!("handbook.artifact-kind.project-context@1.0.0"),
            ),
            ("/role_ref", serde_json::json!("mismatched_role")),
            (
                "/capability_refs",
                serde_json::json!(["constitutional_root"]),
            ),
            ("/label", serde_json::json!("Mismatched Label")),
            (
                "/canonical_path",
                serde_json::json!(".handbook/mismatched.yaml"),
            ),
            (
                "/requiredness/mode",
                if descriptor["requiredness"]["mode"] == "always" {
                    serde_json::json!("optional")
                } else {
                    serde_json::json!("always")
                },
            ),
            (
                "/requiredness/condition_ref",
                if descriptor["requiredness"]["condition_ref"].is_null() {
                    serde_json::json!(
                        "handbook.condition.project.managed-operational-surface@1.0.0"
                    )
                } else {
                    Value::Null
                },
            ),
            (
                "/depends_on",
                serde_json::json!([{
                    "target_kind": "instance",
                    "target_ref": "project_authority",
                    "target_contract_ref": null,
                    "cardinality": "exactly_one"
                }]),
            ),
            (
                "/lifecycle_policy_ref",
                serde_json::json!("handbook.lifecycle.constitutional-review-lock@1.0.0"),
            ),
            ("/intake_definition_ref", Value::Null),
            (
                "/intake_definition_ref",
                serde_json::json!("example.intake@1.0.0"),
            ),
            ("/renderer_definition_refs", serde_json::json!([])),
            (
                "/projection_definition_refs",
                serde_json::json!(["example.projection@1.0.0"]),
            ),
            (
                "/validation_overlay_refs",
                serde_json::json!(["example.validation-overlay@1.0.0"]),
            ),
            ("/extensions", serde_json::json!({"unexpected": true})),
        ];
        for (pointer, mismatch) in mismatches {
            let mut near_miss = descriptor.clone();
            *near_miss
                .pointer_mut(pointer)
                .expect("descriptor mutation pointer") = mismatch;
            assert!(
                ArtifactInstanceRegistry::resolve(
                    &[charter.clone(), near_miss],
                    &kinds,
                    &[&condition],
                )
                .is_err(),
                "refuse descriptor mismatch at {pointer}"
            );
        }
        for renderers in [
            serde_json::json!(["example.renderer@1.0.0"]),
            serde_json::json!([renderer, "example.renderer@1.0.0"]),
        ] {
            let mut near_miss = descriptor.clone();
            near_miss["renderer_definition_refs"] = renderers;
            assert!(
                ArtifactInstanceRegistry::resolve(
                    &[charter.clone(), near_miss],
                    &kinds,
                    &[&condition],
                )
                .is_err(),
                "refuse mismatched or extra renderer cardinality"
            );
        }
    }
}

#[test]
fn shipped_profile_1_2_resolves_from_exact_builtins_as_the_default() {
    let resolved = handbook_engine::resolve_profile_selection(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        shipped_profile_1_2_request(),
    )
    .expect("resolve shipped profile 1.2");
    assert_eq!(
        resolved.exact_ref().as_str(),
        "handbook.profile.shipped-root@1.2.0"
    );
    assert_eq!(
        handbook_engine::resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
            .expect("existing shipped request")
            .profile_ref()
            .as_str(),
        "handbook.profile.shipped-root@1.2.0"
    );
}

#[test]
fn shipped_profile_1_2_fingerprint_covers_the_complete_typed_closure() {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let profile_path = "definitions/profiles/handbook.profile.shipped-root/1.2.0.yaml";
    let mut profile =
        parse_definition_yaml(&fs::read(repository_root.join(profile_path)).expect("read profile"))
            .expect("parse profile");
    let supplied = profile
        .as_object_mut()
        .and_then(|object| object.remove("profile_fingerprint"))
        .and_then(|value| value.as_str().map(ToOwned::to_owned))
        .expect("profile fingerprint");
    let refs = |field: &str| {
        profile[field]
            .as_array()
            .expect("definition source array")
            .iter()
            .map(|value| value.as_str().expect("definition ref").to_owned())
            .collect::<Vec<_>>()
    };
    let schema_refs = refs("schema_registry_sources");
    let kind_refs = refs("artifact_kind_sources");
    let schema_paths = schema_refs
        .iter()
        .map(|reference| {
            let (identity, version) = reference.split_once('@').expect("exact schema ref");
            format!("definitions/schemas/{identity}/{version}.entry.yaml")
        })
        .collect::<Vec<_>>();
    let kind_paths = kind_refs
        .iter()
        .map(|reference| {
            let (identity, version) = reference.split_once('@').expect("exact kind ref");
            format!("definitions/artifact-kinds/{identity}/{version}.yaml")
        })
        .collect::<Vec<_>>();
    let schemas = SchemaRegistry::load(
        repository_root,
        &schema_paths,
        &["definitions/schemas".to_owned()],
    )
    .expect("load profile schemas");
    let kinds = load_artifact_kind_registry(
        repository_root,
        ArtifactKindRegistryLoadRequest::new(
            ExactDefinitionRef::parse("handbook.roles.core@1.1.0").expect("role ref"),
            schema_paths,
            vec!["definitions/schemas".to_owned()],
            kind_paths,
        )
        .with_semantic_sources(
            vec![
                "definitions/semantic-capabilities/handbook.capabilities.constitutional-root/1.0.0.yaml"
                    .to_owned(),
            ],
            vec![
                "definitions/semantic-validators/handbook.semantic-validation.constitutional-root/1.0.0.yaml"
                    .to_owned(),
                "definitions/semantic-validators/handbook.semantic-validation.constitutional-root/1.1.0.yaml"
                    .to_owned(),
            ],
        ),
    )
    .expect("load profile kinds");
    let condition = ProjectConditionDefinition::load(
        repository_root,
        "definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml",
    )
    .expect("load condition");
    let instances = ArtifactInstanceRegistry::resolve(
        profile["artifact_instances"]
            .as_array()
            .expect("artifact instances"),
        &kinds,
        &[&condition],
    )
    .expect("resolve descriptor closure");
    let vocabulary = VocabularyDefinition::load(
        repository_root,
        "definitions/vocabularies/handbook.vocabulary.shipped-root/1.0.0.yaml",
    )
    .expect("load vocabulary");
    let policies = ContextResolutionPolicyRegistry::load(
        repository_root,
        &[
            "definitions/context-resolution-policies/handbook.mutation-matcher.core/1.0.0.yaml"
                .to_owned(),
            "definitions/context-resolution-policies/handbook.resolution-escalation.core/1.0.0.yaml"
                .to_owned(),
            "definitions/context-resolution-policies/handbook.memory-promotion.core/1.0.0.yaml"
                .to_owned(),
        ],
    )
    .expect("load context policies");
    let context = ContextResolutionStackDefinition::load(
        repository_root,
        "definitions/context-resolution/handbook.context-resolution.shipped-root/1.0.0.yaml",
        &policies,
    )
    .expect("load context stack");

    let mut dependencies = vec![
        serde_json::json!({
            "definition_fingerprint": instances.fingerprint().as_str(),
            "definition_ref": "handbook.profile.shipped-root@1.2.0",
            "dependency_role": "artifact_instance_registry",
        }),
        serde_json::json!({
            "definition_fingerprint": condition.definition_fingerprint().as_str(),
            "definition_ref":
                "handbook.condition.project.managed-operational-surface@1.0.0",
            "dependency_role": "condition",
        }),
        serde_json::json!({
            "definition_fingerprint": context.definition_fingerprint().as_str(),
            "definition_ref": "handbook.context-resolution.shipped-root@1.0.0",
            "dependency_role": "context_resolution",
        }),
        serde_json::json!({
            "definition_fingerprint":
                "sha256:a92229722f25119c7d91137e1feef4ce51b88ae766ce308b585d37f39eb52d1c",
            "definition_ref": "handbook.intake.charter@1.0.0",
            "dependency_role": "intake",
        }),
        serde_json::json!({
            "definition_fingerprint":
                parse_definition_yaml(
                    &fs::read(repository_root.join(
                        "definitions/intakes/handbook.intake.project-context/1.0.0.yaml"
                    ))
                    .expect("read Project Context intake"),
                )
                .expect("parse Project Context intake")["intake_definition_fingerprint"],
            "definition_ref": "handbook.intake.project-context@1.0.0",
            "dependency_role": "intake",
        }),
        serde_json::json!({
            "definition_fingerprint":
                parse_definition_yaml(
                    &fs::read(repository_root.join(
                        "definitions/intakes/handbook.intake.environment-context/1.0.0.yaml"
                    ))
                    .expect("read Environment Context intake"),
                )
                .expect("parse Environment Context intake")["intake_definition_fingerprint"],
            "definition_ref": "handbook.intake.environment-context@1.0.0",
            "dependency_role": "intake",
        }),
        serde_json::json!({
            "definition_fingerprint":
                "sha256:88caafb9caaf137647c42a91cd2762ac0871e0a20e2a1844c2c0076d5fb43cc3",
            "definition_ref": "handbook.lifecycle.constitutional-review-lock@1.0.0",
            "dependency_role": "lifecycle",
        }),
        serde_json::json!({
            "definition_fingerprint":
                "sha256:68f6fceedaab6133364d18a2b474694d1b44b91d73c05d0af65db1aa58e80fa7",
            "definition_ref": "handbook.renderer.charter-review-markdown@1.0.0",
            "dependency_role": "renderer",
        }),
        serde_json::json!({
            "definition_fingerprint":
                parse_definition_yaml(
                    &fs::read(repository_root.join(
                        "definitions/renderers/handbook.renderer.project-context-review-markdown/1.0.0.yaml"
                    ))
                    .expect("read Project Context renderer"),
                )
                .expect("parse Project Context renderer")["renderer_fingerprint"],
            "definition_ref": "handbook.renderer.project-context-review-markdown@1.0.0",
            "dependency_role": "renderer",
        }),
        serde_json::json!({
            "definition_fingerprint":
                parse_definition_yaml(
                    &fs::read(repository_root.join(
                        "definitions/renderers/handbook.renderer.environment-context-review-markdown/1.0.0.yaml"
                    ))
                    .expect("read Environment Context renderer"),
                )
                .expect("parse Environment Context renderer")["renderer_fingerprint"],
            "definition_ref": "handbook.renderer.environment-context-review-markdown@1.0.0",
            "dependency_role": "renderer",
        }),
        serde_json::json!({
            "definition_fingerprint": kinds.stable_role_registry().fingerprint().as_str(),
            "definition_ref": "handbook.roles.core@1.1.0",
            "dependency_role": "stable_role_registry",
        }),
        serde_json::json!({
            "definition_fingerprint": vocabulary.vocabulary_fingerprint().as_str(),
            "definition_ref": "handbook.vocabulary.shipped-root@1.0.0",
            "dependency_role": "vocabulary",
        }),
    ];
    for reference in &schema_refs {
        let exact = ExactDefinitionRef::parse(reference).expect("schema ref");
        dependencies.push(serde_json::json!({
            "definition_fingerprint":
                schemas.entry(&exact).expect("schema entry").entry_fingerprint().as_str(),
            "definition_ref": reference,
            "dependency_role": "schema_source",
        }));
    }
    for reference in &kind_refs {
        let exact = ExactDefinitionRef::parse(reference).expect("kind ref");
        dependencies.push(serde_json::json!({
            "definition_fingerprint":
                kinds.kind(&exact).expect("kind").definition_fingerprint().as_str(),
            "definition_ref": reference,
            "dependency_role": "artifact_kind_source",
        }));
    }
    dependencies.sort_by(|left, right| {
        (
            left["dependency_role"].as_str(),
            left["definition_ref"].as_str(),
        )
            .cmp(&(
                right["dependency_role"].as_str(),
                right["definition_ref"].as_str(),
            ))
    });
    let computed = DefinitionFingerprint::from_json_value(&serde_json::json!({
        "definition": profile,
        "resolved_dependencies": dependencies,
    }))
    .expect("compute profile closure")
    .to_string();

    assert_eq!(
        supplied, computed,
        "profile closure fingerprint must be {}",
        computed
    );
}

#[test]
fn immutable_definition_vector_replays_authored_records_and_released_charter_row() {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let vector: Value =
        serde_json::from_slice(&fs::read(DEFINITION_VECTOR_PATH).expect("definition vector"))
            .expect("parse definition vector");
    let profile_1_1 = parse_definition_yaml(
        &fs::read(
            repository_root.join("definitions/profiles/handbook.profile.shipped-root/1.1.0.yaml"),
        )
        .expect("read profile 1.1"),
    )
    .expect("parse profile 1.1");
    let profile_1_2 = parse_definition_yaml(
        &fs::read(
            repository_root.join("definitions/profiles/handbook.profile.shipped-root/1.2.0.yaml"),
        )
        .expect("read profile 1.2"),
    )
    .expect("parse profile 1.2");
    assert_eq!(
        profile_1_2["profile_fingerprint"],
        vector["profile"]["expected_fingerprint"]
    );
    assert_eq!(
        profile_1_2["artifact_instances"][0], profile_1_1["artifact_instances"][0],
        "Project Authority row must remain field-for-field identical"
    );
    assert_eq!(
        profile_1_2["artifact_instances"][0],
        vector["profile"]["project_authority_descriptor"]
    );
    assert_eq!(
        profile_1_2["artifact_instances"]
            .as_array()
            .expect("instances")
            .iter()
            .map(|instance| instance["id"].clone())
            .collect::<Vec<_>>(),
        vector["profile"]["expected_instance_ids"]
            .as_array()
            .expect("expected IDs")
            .clone()
    );

    for (slug, family) in vector["families"].as_object().expect("definition families") {
        let kind = parse_definition_yaml(
            &fs::read(repository_root.join(format!(
                "definitions/artifact-kinds/handbook.artifact-kind.{slug}/1.1.0.yaml"
            )))
            .expect("read kind"),
        )
        .expect("parse kind");
        let intake = parse_definition_yaml(
            &fs::read(repository_root.join(format!(
                "definitions/intakes/handbook.intake.{slug}/1.0.0.yaml"
            )))
            .expect("read intake"),
        )
        .expect("parse intake");
        let renderer = parse_definition_yaml(
            &fs::read(repository_root.join(format!(
                "definitions/renderers/handbook.renderer.{slug}-review-markdown/1.0.0.yaml"
            )))
            .expect("read renderer"),
        )
        .expect("parse renderer");
        assert_eq!(kind["definition_fingerprint"], family["kind_fingerprint"]);
        assert_eq!(kind["canonical_schema_ref"], family["schema_ref"]);
        assert_eq!(
            kind["renderer_definition_refs"],
            serde_json::json!([family["renderer_ref"].clone()])
        );
        assert_eq!(
            intake["intake_definition_fingerprint"],
            family["intake_fingerprint"]
        );
        assert_eq!(intake["artifact_kind_ref"], family["kind_ref"]);
        assert_eq!(intake["candidate_schema_ref"], family["schema_ref"]);
        let coverage_paths = intake["coverage"]
            .as_array()
            .expect("coverage")
            .iter()
            .flat_map(|row| {
                row["target_paths"]
                    .as_array()
                    .expect("target paths")
                    .clone()
            })
            .collect::<Vec<_>>();
        assert_eq!(
            coverage_paths,
            family["coverage_paths"]
                .as_array()
                .expect("vector coverage paths")
                .clone()
        );
        assert_eq!(
            renderer["renderer_fingerprint"],
            family["renderer_fingerprint"]
        );
        assert_eq!(renderer["input_schema_ref"], family["schema_ref"]);
        assert_eq!(renderer["resolution_input"], Value::Null);
        assert_eq!(renderer["extensions"], serde_json::json!({}));
    }
}

#[test]
fn p1a_proof_freezes_the_exact_profile_vector_fingerprint() {
    let vector: Value =
        serde_json::from_slice(&fs::read(DEFINITION_VECTOR_PATH).expect("definition vector"))
            .expect("parse definition vector");
    let expected = vector["profile"]["expected_fingerprint"]
        .as_str()
        .expect("profile fingerprint");
    let proof = fs::read_to_string(P1A_PROOF_PATH).expect("P1A proof");

    assert!(
        proof.contains(expected),
        "P1A proof must freeze the immutable vector fingerprint {expected}"
    );
}

#[test]
fn renderer_goldens_are_schema_valid_fixed_bytes_and_resolution_free() {
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let vector: Value =
        serde_json::from_slice(&fs::read(RENDERER_GOLDEN_PATH).expect("renderer goldens"))
            .expect("parse renderer goldens");
    assert_eq!(vector["external_inputs"], serde_json::json!([]));
    let schema_paths = [
        "project-context",
        "environment-context",
        "work-specification",
        "decision-record",
        "risk-record",
    ]
    .map(|slug| format!("definitions/schemas/handbook.schemas.artifacts.{slug}/1.0.0.entry.yaml"));
    let schemas = SchemaRegistry::load(
        repository_root,
        &schema_paths,
        &["definitions/schemas".to_owned()],
    )
    .expect("load golden schemas");
    for golden in vector["goldens"].as_array().expect("goldens") {
        let renderer_ref = golden["renderer_ref"].as_str().expect("renderer ref");
        let (renderer_id, renderer_version) =
            renderer_ref.split_once('@').expect("exact renderer ref");
        let slug = renderer_id
            .strip_prefix("handbook.renderer.")
            .and_then(|value| value.strip_suffix("-review-markdown"))
            .expect("renderer slug");
        let renderer = parse_definition_yaml(
            &fs::read(repository_root.join(format!(
                "definitions/renderers/{renderer_id}/{renderer_version}.yaml"
            )))
            .expect("read renderer"),
        )
        .expect("parse renderer");
        assert_eq!(
            renderer["renderer_fingerprint"],
            golden["renderer_fingerprint"]
        );
        assert_eq!(renderer["input_schema_ref"], golden["input_schema_ref"]);
        assert_eq!(renderer["implementation_id"], golden["implementation_id"]);
        assert_eq!(
            renderer["determinism_profile"],
            golden["determinism_profile"]
        );
        assert_eq!(renderer["resolution_input"], Value::Null);
        assert_eq!(renderer["output_media_type"], "text/markdown");
        assert_eq!(renderer["extensions"], serde_json::json!({}));
        schemas
            .resolved(
                &ExactDefinitionRef::parse(
                    golden["input_schema_ref"]
                        .as_str()
                        .expect("input schema ref"),
                )
                .expect("schema ref"),
            )
            .expect("resolved schema")
            .validate_json(&golden["canonical_input"])
            .unwrap_or_else(|errors| panic!("{renderer_ref}: {errors:?}"));
        let markdown = golden["expected_markdown_utf8"]
            .as_str()
            .expect("golden Markdown");
        assert_eq!(
            markdown.as_bytes().len() as u64,
            golden["expected_byte_length"]
                .as_u64()
                .expect("byte length")
        );
        assert_eq!(
            DefinitionFingerprint::from_bytes(markdown.as_bytes()).as_str(),
            golden["expected_sha256"]
                .as_str()
                .expect("golden fingerprint")
        );
        if slug == "project-context" {
            let input: handbook_engine::CanonicalProjectContext =
                serde_json::from_value(golden["canonical_input"].clone())
                    .expect("typed Project Context");
            assert_eq!(
                handbook_engine::render_project_context_markdown(&input)
                    .expect("existing deterministic renderer"),
                markdown.as_bytes()
            );
        }
    }
}
