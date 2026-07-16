use handbook_engine::{
    load_artifact_kind_registry, parse_definition_yaml, ArtifactKindRegistryLoadRequest,
    CanonicalArtifactKind, ExactDefinitionRef,
};
use serde_json::json;
use std::path::PathBuf;

const ROLE_REGISTRY_REF: &str = "handbook.roles.core@1.1.0";
const FIXTURE_ROOT: &str = "tests/fixtures/hcm_1_1_custom_kind";

fn request(reverse: bool) -> ArtifactKindRegistryLoadRequest {
    let mut schema_sources = vec![
        format!("{FIXTURE_ROOT}/schema-entry.yaml"),
        format!("{FIXTURE_ROOT}/companion-schema-entry.yaml"),
    ];
    let mut kind_sources = vec![
        format!("{FIXTURE_ROOT}/kind.yaml"),
        format!("{FIXTURE_ROOT}/companion-kind.yaml"),
    ];
    if reverse {
        schema_sources.reverse();
        kind_sources.reverse();
    }
    ArtifactKindRegistryLoadRequest::new(
        ExactDefinitionRef::parse(ROLE_REGISTRY_REF).unwrap(),
        schema_sources,
        vec![FIXTURE_ROOT.to_string()],
        kind_sources,
    )
}

#[test]
fn repository_defined_custom_kind_loads_without_a_product_path_variant() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let forward = load_artifact_kind_registry(&repo_root, request(false)).expect("forward load");
    let reverse = load_artifact_kind_registry(&repo_root, request(true)).expect("reverse load");

    assert_eq!(forward.fingerprint(), reverse.fingerprint());
    assert_eq!(forward.kind_refs(), reverse.kind_refs());
    assert_eq!(
        forward.schema_registry().fingerprint(),
        reverse.schema_registry().fingerprint()
    );
    assert_eq!(
        forward.schema_registry().entry_refs(),
        reverse.schema_registry().entry_refs()
    );
    assert_eq!(forward.kind_refs().len(), 2);
    assert_eq!(forward.schema_registry().entry_refs().len(), 2);

    let primary_kind =
        ExactDefinitionRef::parse("example.artifact-kind.hcm-incident@1.0.0").unwrap();
    let primary_schema = ExactDefinitionRef::parse("example.schemas.hcm-incident@1.0.0").unwrap();
    let companion_kind =
        ExactDefinitionRef::parse("example.artifact-kind.hcm-companion@1.0.0").unwrap();
    let companion_schema =
        ExactDefinitionRef::parse("example.schemas.hcm-companion@1.0.0").unwrap();
    let resolved = forward
        .schema_registry()
        .resolved(&primary_schema)
        .expect("primary schema");
    let reverse_primary = reverse
        .schema_registry()
        .resolved(&primary_schema)
        .expect("reverse primary schema");
    assert_eq!(
        resolved.closure_document_refs(),
        [
            format!("{FIXTURE_ROOT}/fields.schema.json"),
            format!("{FIXTURE_ROOT}/root.schema.json"),
        ]
    );
    assert_eq!(
        resolved.closure_document_refs(),
        reverse_primary.closure_document_refs()
    );
    assert_eq!(
        resolved.entry().closure_fingerprint(),
        reverse_primary.entry().closure_fingerprint()
    );
    assert_eq!(
        resolved.entry().entry_fingerprint(),
        reverse
            .schema_registry()
            .entry(&primary_schema)
            .unwrap()
            .entry_fingerprint()
    );
    assert_eq!(
        forward
            .kind(&primary_kind)
            .unwrap()
            .definition_fingerprint(),
        reverse
            .kind(&primary_kind)
            .unwrap()
            .definition_fingerprint()
    );
    let forward_companion = forward
        .schema_registry()
        .resolved(&companion_schema)
        .expect("forward companion schema");
    let reverse_companion = reverse
        .schema_registry()
        .resolved(&companion_schema)
        .expect("reverse companion schema");
    assert_eq!(
        forward_companion.closure_document_refs(),
        [format!("{FIXTURE_ROOT}/companion.schema.json")]
    );
    assert_eq!(
        forward_companion.closure_document_refs(),
        reverse_companion.closure_document_refs()
    );
    assert_eq!(
        forward_companion.entry().closure_fingerprint(),
        reverse_companion.entry().closure_fingerprint()
    );
    assert_eq!(
        forward
            .schema_registry()
            .entry(&companion_schema)
            .unwrap()
            .entry_fingerprint(),
        reverse
            .schema_registry()
            .entry(&companion_schema)
            .unwrap()
            .entry_fingerprint()
    );
    assert_eq!(
        forward
            .kind(&companion_kind)
            .unwrap()
            .definition_fingerprint(),
        reverse
            .kind(&companion_kind)
            .unwrap()
            .definition_fingerprint()
    );

    let valid_bytes = std::fs::read(repo_root.join(FIXTURE_ROOT).join("valid.yaml")).unwrap();
    let valid = parse_definition_yaml(&valid_bytes).unwrap();
    assert!(forward.validate_json(&primary_kind, &valid).is_ok());
    assert!(reverse.validate_json(&primary_kind, &valid).is_ok());

    let invalid = json!({"title": "x", "severity": "urgent"});
    let forward_errors = forward
        .validate_json(&primary_kind, &invalid)
        .expect_err("invalid fixture");
    let reverse_errors = reverse
        .validate_json(&primary_kind, &invalid)
        .expect_err("invalid fixture");
    assert_eq!(forward_errors, reverse_errors);
    assert_eq!(forward_errors[0].instance_location(), "/severity");
    assert!(forward_errors
        .iter()
        .any(|error| error.instance_location() == "/title"));

    let valid_companion = json!({"enabled": true});
    assert!(forward
        .validate_json(&companion_kind, &valid_companion)
        .is_ok());
    assert!(reverse
        .validate_json(&companion_kind, &valid_companion)
        .is_ok());
    let invalid_companion = json!({"enabled": "SECRET_INVALID"});
    let forward_companion_errors = forward
        .validate_json(&companion_kind, &invalid_companion)
        .expect_err("invalid companion fixture");
    let reverse_companion_errors = reverse
        .validate_json(&companion_kind, &invalid_companion)
        .expect_err("invalid reverse companion fixture");
    assert_eq!(forward_companion_errors, reverse_companion_errors);
    assert_eq!(forward_companion_errors[0].instance_location(), "/enabled");

    let fixed_product_kinds = [
        CanonicalArtifactKind::Charter,
        CanonicalArtifactKind::ProjectContext,
        CanonicalArtifactKind::EnvironmentInventory,
        CanonicalArtifactKind::FeatureSpec,
    ];
    assert_eq!(fixed_product_kinds.len(), 4);
    assert!(!format!("{fixed_product_kinds:?}").contains("HcmIncident"));
}
