use handbook_engine::{
    DefinitionFingerprint, RegistryLoadErrorKind, SchemaRegistry, StructuralValidationError,
    MAX_SOURCE_DOCUMENT_BYTES,
};
use serde_json::{json, Value};
use std::path::Path;

const DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";

fn write(path: &Path, bytes: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(path, bytes).expect("write fixture");
}

fn canonical_fingerprint(value: &Value) -> String {
    DefinitionFingerprint::from_json_value(value)
        .expect("canonical fingerprint")
        .to_string()
}

fn schema_bytes(value: &Value) -> Vec<u8> {
    serde_json::to_vec_pretty(value).expect("schema JSON")
}

fn write_entry(
    repo: &Path,
    entry_path: &str,
    content_schema_id: &str,
    document_ref: &str,
    closure_documents: &[(&str, &[u8])],
) {
    let root_bytes = closure_documents
        .iter()
        .find_map(|(path, bytes)| (*path == document_ref).then_some(*bytes))
        .expect("root document bytes");
    let document_fingerprint = DefinitionFingerprint::from_bytes(root_bytes).to_string();

    let mut closure = closure_documents
        .iter()
        .map(|(path, bytes)| {
            json!({
                "document_ref": path,
                "document_fingerprint": DefinitionFingerprint::from_bytes(bytes).to_string(),
            })
        })
        .collect::<Vec<_>>();
    closure.sort_by(|left, right| {
        left["document_ref"]
            .as_str()
            .cmp(&right["document_ref"].as_str())
    });
    let closure_fingerprint = canonical_fingerprint(&Value::Array(closure));
    let preimage = json!({
        "schema_id": "handbook.schema-registry-entry",
        "schema_version": "1.0",
        "content_schema_id": content_schema_id,
        "content_schema_version": "1.0.0",
        "document_ref": document_ref,
        "document_fingerprint": document_fingerprint,
        "closure_fingerprint": closure_fingerprint,
        "meta_schema_ref": DIALECT,
        "media_type": "application/schema+json",
        "compatibility": "exact",
        "extensions": {},
    });
    let entry_fingerprint = canonical_fingerprint(&preimage);
    let yaml = format!(
        "schema_id: handbook.schema-registry-entry\n\
         schema_version: \"1.0\"\n\
         content_schema_id: {content_schema_id}\n\
         content_schema_version: \"1.0.0\"\n\
         document_ref: {document_ref}\n\
         document_fingerprint: {document_fingerprint}\n\
         closure_fingerprint: {closure_fingerprint}\n\
         meta_schema_ref: {DIALECT}\n\
         media_type: application/schema+json\n\
         compatibility: exact\n\
         extensions: {{}}\n\
         entry_fingerprint: {entry_fingerprint}\n"
    );
    write(&repo.join(entry_path), yaml.as_bytes());
}

fn valid_schema_repo() -> (tempfile::TempDir, String) {
    let repo = tempfile::tempdir().expect("repo");
    let root = json!({
        "$schema": DIALECT,
        "type": "object",
        "properties": {
            "title": {"$ref": "fields.schema.json#/$defs/title"},
            "metadata": {
                "const": {"unknown_keyword_is_instance_data": true}
            }
        },
        "required": ["title"],
        "additionalProperties": false
    });
    let fields = json!({
        "$schema": DIALECT,
        "$defs": {
            "title": {"type": "string", "minLength": 3}
        }
    });
    let root_bytes = schema_bytes(&root);
    let fields_bytes = schema_bytes(&fields);
    write(&repo.path().join("schemas/root.schema.json"), &root_bytes);
    write(
        &repo.path().join("schemas/fields.schema.json"),
        &fields_bytes,
    );
    let entry_path = "definitions/incident-schema.yaml".to_string();
    write_entry(
        repo.path(),
        &entry_path,
        "example.schemas.incident-brief",
        "schemas/root.schema.json",
        &[
            ("schemas/root.schema.json", root_bytes.as_slice()),
            ("schemas/fields.schema.json", fields_bytes.as_slice()),
        ],
    );
    (repo, entry_path)
}

#[test]
fn safe_local_closure_loads_and_reports_deterministic_structural_locations() {
    let (repo, entry_path) = valid_schema_repo();
    let registry = SchemaRegistry::load(repo.path(), &[entry_path], &["schemas".to_string()])
        .expect("schema registry");
    let schema_ref =
        handbook_engine::ExactDefinitionRef::parse("example.schemas.incident-brief@1.0.0").unwrap();
    let resolved = registry.resolved(&schema_ref).expect("resolved schema");

    assert_eq!(
        resolved.closure_document_refs(),
        ["schemas/fields.schema.json", "schemas/root.schema.json"]
    );
    assert!(resolved
        .validate_json(&json!({"title": "Incident"}))
        .is_ok());

    let errors = resolved
        .validate_json(&json!({"title": "x"}))
        .expect_err("invalid data");
    assert_eq!(errors[0].instance_location(), "/title");
    assert!(errors[0]
        .schema_location()
        .contains("schemas/fields.schema.json"));
}

#[test]
fn structural_locations_bound_long_authored_document_paths() {
    let repo = tempfile::tempdir().expect("repo");
    let sentinel = "SECRET_STRUCTURAL_PATH_SENTINEL";
    let nested = (0..18)
        .map(|index| format!("{sentinel}-{index:02}"))
        .collect::<Vec<_>>()
        .join("/");
    let document_ref = format!("schemas/{nested}/root.schema.json");
    let schema = json!({"$schema": DIALECT, "type": "string"});
    let bytes = schema_bytes(&schema);
    write(&repo.path().join(&document_ref), &bytes);
    write_entry(
        repo.path(),
        "definitions/long-structural.yaml",
        "example.schemas.long-structural",
        &document_ref,
        &[(document_ref.as_str(), bytes.as_slice())],
    );

    let registry = SchemaRegistry::load(
        repo.path(),
        &["definitions/long-structural.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect("long normalized path remains valid");
    let exact_ref =
        handbook_engine::ExactDefinitionRef::parse("example.schemas.long-structural@1.0.0")
            .unwrap();
    let errors = registry
        .resolved(&exact_ref)
        .unwrap()
        .validate_json(&json!(42))
        .expect_err("number must fail string schema");

    assert!(!errors[0].schema_location().contains(sentinel));
    assert!(errors[0].schema_location().len() <= 512);
}

#[test]
fn internal_schema_resources_admit_spaces_and_utf8_without_location_drift() {
    let repo = tempfile::tempdir().expect("repo");
    let root_ref = "schemas/unicode space/root schema.json";
    let child_ref = "schemas/unicode space/café child.json";
    let root = json!({"$schema": DIALECT, "$ref": "café child.json"});
    let child = json!({"$schema": DIALECT, "type": "string"});
    let root_bytes = schema_bytes(&root);
    let child_bytes = schema_bytes(&child);
    write(&repo.path().join(root_ref), &root_bytes);
    write(&repo.path().join(child_ref), &child_bytes);
    write_entry(
        repo.path(),
        "definitions/unicode-space.yaml",
        "example.schemas.unicode-space",
        root_ref,
        &[
            (root_ref, root_bytes.as_slice()),
            (child_ref, child_bytes.as_slice()),
        ],
    );

    let registry = SchemaRegistry::load(
        repo.path(),
        &["definitions/unicode-space.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect("admitted UTF-8 closure");
    let exact_ref =
        handbook_engine::ExactDefinitionRef::parse("example.schemas.unicode-space@1.0.0").unwrap();
    let resolved = registry.resolved(&exact_ref).unwrap();
    assert_eq!(resolved.closure_document_refs(), [child_ref, root_ref]);
    assert!(resolved.validate_json(&json!("valid")).is_ok());
    let errors = resolved
        .validate_json(&json!(42))
        .expect_err("number must fail child schema");
    assert!(errors
        .iter()
        .any(|error| error.schema_location().starts_with(child_ref)));
    assert!(errors
        .iter()
        .all(|error| !error.schema_location().contains('%')));
}

#[test]
fn cross_document_json_pointer_fragments_admit_spaces_and_utf8() {
    for (index, token) in ["space field", "café"].into_iter().enumerate() {
        let repo = tempfile::tempdir().expect("repo");
        let root_ref = "schemas/fragments/root.schema.json";
        let child_ref = "schemas/fragments/child.schema.json";
        let reference = format!("child.schema.json#/$defs/{token}");
        let root = json!({"$schema": DIALECT, "$ref": reference});
        let child = json!({
            "$schema": DIALECT,
            "$defs": {
                (token): {"type": "string"}
            }
        });
        let root_bytes = schema_bytes(&root);
        let child_bytes = schema_bytes(&child);
        write(&repo.path().join(root_ref), &root_bytes);
        write(&repo.path().join(child_ref), &child_bytes);
        let content_schema_id = format!("example.schemas.fragment-case-{index}");
        let entry_path = format!("definitions/fragment-case-{index}.yaml");
        write_entry(
            repo.path(),
            &entry_path,
            &content_schema_id,
            root_ref,
            &[
                (root_ref, root_bytes.as_slice()),
                (child_ref, child_bytes.as_slice()),
            ],
        );

        let registry = SchemaRegistry::load(repo.path(), &[entry_path], &["schemas".to_string()])
            .expect("admitted RFC 6901 fragment");
        let exact_ref =
            handbook_engine::ExactDefinitionRef::parse(&format!("{content_schema_id}@1.0.0"))
                .unwrap();
        let resolved = registry.resolved(&exact_ref).unwrap();
        assert_eq!(resolved.closure_document_refs(), [child_ref, root_ref]);
        assert_eq!(
            resolved.entry().document_fingerprint(),
            &DefinitionFingerprint::from_bytes(&root_bytes)
        );
        assert!(resolved.validate_json(&json!("valid")).is_ok());
        let errors = resolved
            .validate_json(&json!(42))
            .expect_err("number must fail referenced string schema");
        let expected_prefix = format!("{child_ref}#/$defs/{token}");
        assert!(errors
            .iter()
            .any(|error| error.schema_location().starts_with(&expected_prefix)));
        assert!(errors
            .iter()
            .all(|error| !error.schema_location().contains('%')));
    }
}

#[test]
fn same_document_json_pointer_fragments_admit_spaces_and_utf8() {
    for (index, token) in ["space field", "café"].into_iter().enumerate() {
        let repo = tempfile::tempdir().expect("repo");
        let document_ref = "schemas/same-document.schema.json";
        let schema = json!({
            "$schema": DIALECT,
            "$defs": {
                (token): {"type": "string"}
            },
            "$ref": format!("#/$defs/{token}")
        });
        let bytes = schema_bytes(&schema);
        write(&repo.path().join(document_ref), &bytes);
        let content_schema_id = format!("example.schemas.same-fragment-case-{index}");
        let entry_path = format!("definitions/same-fragment-case-{index}.yaml");
        write_entry(
            repo.path(),
            &entry_path,
            &content_schema_id,
            document_ref,
            &[(document_ref, bytes.as_slice())],
        );

        let registry = SchemaRegistry::load(repo.path(), &[entry_path], &["schemas".to_string()])
            .expect("admitted same-document RFC 6901 fragment");
        let exact_ref =
            handbook_engine::ExactDefinitionRef::parse(&format!("{content_schema_id}@1.0.0"))
                .unwrap();
        let resolved = registry.resolved(&exact_ref).unwrap();
        assert_eq!(resolved.closure_document_refs(), [document_ref]);
        assert_eq!(
            resolved.entry().document_fingerprint(),
            &DefinitionFingerprint::from_bytes(&bytes)
        );
        assert!(resolved.validate_json(&json!("valid")).is_ok());
        let errors = resolved
            .validate_json(&json!(42))
            .expect_err("number must fail referenced string schema");
        let expected_prefix = format!("{document_ref}#/$defs/{token}");
        assert!(errors
            .iter()
            .any(|error| error.schema_location().starts_with(&expected_prefix)));
        assert!(errors
            .iter()
            .all(|error| !error.schema_location().contains('%')));
    }
}

#[test]
fn registry_fingerprints_and_lookup_sets_are_source_order_independent() {
    let (repo, first_entry) = valid_schema_repo();
    let companion = json!({"$schema": DIALECT, "type": "boolean"});
    let companion_bytes = schema_bytes(&companion);
    write(
        &repo.path().join("schemas/companion.schema.json"),
        &companion_bytes,
    );
    let second_entry = "definitions/companion-schema.yaml".to_string();
    write_entry(
        repo.path(),
        &second_entry,
        "example.schemas.companion",
        "schemas/companion.schema.json",
        &[("schemas/companion.schema.json", companion_bytes.as_slice())],
    );

    let forward = SchemaRegistry::load(
        repo.path(),
        &[first_entry.clone(), second_entry.clone()],
        &["schemas".to_string()],
    )
    .expect("forward");
    let reverse = SchemaRegistry::load(
        repo.path(),
        &[second_entry, first_entry],
        &["schemas".to_string()],
    )
    .expect("reverse");
    assert_eq!(forward.fingerprint(), reverse.fingerprint());
    assert_eq!(forward.entry_refs(), reverse.entry_refs());
    assert_eq!(forward.entry_refs().len(), 2);
}

#[test]
fn schema_entries_are_closed_and_extensions_must_be_empty() {
    let (repo, entry_path) = valid_schema_repo();
    let path = repo.path().join(&entry_path);
    let base = std::fs::read_to_string(&path).unwrap();

    write(
        &path,
        base.replace(
            "schema_version: \"1.0\"",
            "instance_path: forbidden\nschema_version: \"1.0\"",
        )
        .as_bytes(),
    );
    let error = SchemaRegistry::load(
        repo.path(),
        std::slice::from_ref(&entry_path),
        &["schemas".to_string()],
    )
    .expect_err("wrong-record field");
    assert_eq!(error.kind(), RegistryLoadErrorKind::UnknownField);

    write(
        &path,
        base.replace("extensions: {}", "extensions: {example: true}")
            .as_bytes(),
    );
    let error = SchemaRegistry::load(
        repo.path(),
        std::slice::from_ref(&entry_path),
        &["schemas".to_string()],
    )
    .expect_err("non-empty extensions");
    assert_eq!(error.kind(), RegistryLoadErrorKind::UnsupportedDependency);

    let secret_field = format!("SECRET_SCHEMA_FIELD_{}", "x".repeat(500));
    write(
        &path,
        base.replace(
            "schema_version: \"1.0\"",
            &format!("{secret_field}: true\nschema_version: \"1.0\""),
        )
        .as_bytes(),
    );
    let error = SchemaRegistry::load(
        repo.path(),
        std::slice::from_ref(&entry_path),
        &["schemas".to_string()],
    )
    .expect_err("secret unknown schema-entry field");
    assert_eq!(error.kind(), RegistryLoadErrorKind::UnknownField);
    assert!(!error.detail().contains("SECRET_SCHEMA_FIELD"));
    assert!(!error.to_string().contains("SECRET_SCHEMA_FIELD"));
    assert!(error.detail().len() < 256);

    for (field, replacement, expected) in [
        (
            "media_type",
            "text/plain",
            RegistryLoadErrorKind::UnsupportedMediaType,
        ),
        (
            "compatibility",
            "backward",
            RegistryLoadErrorKind::UnsupportedCompatibility,
        ),
        (
            "document_fingerprint",
            "sha256:0000000000000000000000000000000000000000000000000000000000000000",
            RegistryLoadErrorKind::FingerprintMismatch,
        ),
        (
            "closure_fingerprint",
            "sha256:0000000000000000000000000000000000000000000000000000000000000000",
            RegistryLoadErrorKind::FingerprintMismatch,
        ),
        (
            "entry_fingerprint",
            "sha256:0000000000000000000000000000000000000000000000000000000000000000",
            RegistryLoadErrorKind::FingerprintMismatch,
        ),
    ] {
        let original_line = base
            .lines()
            .find(|line| line.starts_with(&format!("{field}:")))
            .expect("entry field");
        let mutated = base.replace(original_line, &format!("{field}: {replacement}"));
        write(&path, mutated.as_bytes());
        let error = SchemaRegistry::load(
            repo.path(),
            std::slice::from_ref(&entry_path),
            &["schemas".to_string()],
        )
        .expect_err(field);
        assert_eq!(error.kind(), expected, "field: {field}");
    }
}

#[test]
fn long_authored_schema_source_location_is_bounded_and_redacted() {
    let (repo, entry_path) = valid_schema_repo();
    let sentinel = "SECRET_PATH_SENTINEL";
    let relocated_entry = format!("{sentinel}/incident-schema.yaml");
    write(
        &repo.path().join(&relocated_entry),
        &std::fs::read(repo.path().join(entry_path)).expect("read valid entry"),
    );
    let long_source = format!("{}{relocated_entry}", "./".repeat(10_000));

    let error = SchemaRegistry::load(repo.path(), &[long_source], &["schemas".to_string()])
        .expect_err("noncanonical source path must refuse");

    assert_eq!(error.kind(), RegistryLoadErrorKind::InvalidSourcePath);
    assert!(!error.location().unwrap_or_default().contains(sentinel));
    assert!(!error.to_string().contains(sentinel));
    assert!(error.location().unwrap_or_default().len() < 256);
    assert!(error.to_string().len() < 512);
}

#[test]
fn schema_profile_refuses_ambient_or_rebased_resolution_and_unknown_keywords() {
    for (mutation, expected) in [
        (
            json!({"$schema": DIALECT, "$ref": "https://example.com/x"}),
            RegistryLoadErrorKind::RemoteReferenceRefused,
        ),
        (
            json!({"$schema": DIALECT, "$ref": "../outside.schema.json"}),
            RegistryLoadErrorKind::RemoteReferenceRefused,
        ),
        (
            json!({"$schema": DIALECT, "$id": "relative-base", "type": "string"}),
            RegistryLoadErrorKind::UnsupportedSchemaIdentifier,
        ),
        (
            json!({"$schema": DIALECT, "$id": "https://example.com/root", "type": "string"}),
            RegistryLoadErrorKind::UnsupportedSchemaIdentifier,
        ),
        (
            json!({"$schema": DIALECT, "properties": {"x": {"$id": "https://example.com/nested", "type": "string"}}}),
            RegistryLoadErrorKind::UnsupportedSchemaIdentifier,
        ),
        (
            json!({"$schema": DIALECT, "$anchor": "name", "type": "string"}),
            RegistryLoadErrorKind::UnsupportedSchemaIdentifier,
        ),
        (
            json!({"$schema": DIALECT, "$dynamicRef": "#name"}),
            RegistryLoadErrorKind::UnsupportedSchemaIdentifier,
        ),
        (
            json!({"$schema": DIALECT, "unknownAnnotation": true}),
            RegistryLoadErrorKind::UnsupportedSchemaKeyword,
        ),
        (
            json!({"$schema": DIALECT, "format": "email"}),
            RegistryLoadErrorKind::UnsupportedSchemaKeyword,
        ),
        (
            json!({"type": "string"}),
            RegistryLoadErrorKind::UnsupportedDialect,
        ),
        (
            json!({"$schema": "https://json-schema.org/draft/2019-09/schema", "type": "string"}),
            RegistryLoadErrorKind::UnsupportedDialect,
        ),
        (
            json!({"$schema": DIALECT, "properties": {"x": {"$schema": DIALECT, "type": "string"}}}),
            RegistryLoadErrorKind::UnsupportedDialect,
        ),
        (
            json!({"$schema": DIALECT, "type": ["string", "not-a-valid-type"]}),
            RegistryLoadErrorKind::UnsupportedDialect,
        ),
    ] {
        let repo = tempfile::tempdir().unwrap();
        let bytes = schema_bytes(&mutation);
        write(&repo.path().join("schemas/root.schema.json"), &bytes);
        write_entry(
            repo.path(),
            "definitions/entry.yaml",
            "example.schemas.invalid",
            "schemas/root.schema.json",
            &[("schemas/root.schema.json", bytes.as_slice())],
        );
        let error = SchemaRegistry::load(
            repo.path(),
            &["definitions/entry.yaml".to_string()],
            &["schemas".to_string()],
        )
        .expect_err("invalid schema profile");
        assert_eq!(error.kind(), expected, "schema: {mutation}");
    }
}

#[test]
fn missing_cycle_and_conflicting_identity_fail_closed() {
    let repo = tempfile::tempdir().unwrap();
    let first = json!({"$schema": DIALECT, "$ref": "second.schema.json"});
    let second = json!({"$schema": DIALECT, "$ref": "first.schema.json"});
    let first_bytes = schema_bytes(&first);
    let second_bytes = schema_bytes(&second);
    write(&repo.path().join("schemas/first.schema.json"), &first_bytes);
    write(
        &repo.path().join("schemas/second.schema.json"),
        &second_bytes,
    );
    write_entry(
        repo.path(),
        "definitions/cycle.yaml",
        "example.schemas.cycle",
        "schemas/first.schema.json",
        &[
            ("schemas/first.schema.json", first_bytes.as_slice()),
            ("schemas/second.schema.json", second_bytes.as_slice()),
        ],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/cycle.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("cycle");
    assert_eq!(error.kind(), RegistryLoadErrorKind::LocalReferenceCycle);

    let (repo, entry_path) = valid_schema_repo();
    let duplicate = "definitions/duplicate.yaml";
    std::fs::copy(repo.path().join(&entry_path), repo.path().join(duplicate)).unwrap();
    for sources in [
        vec![entry_path.clone(), duplicate.to_string()],
        vec![duplicate.to_string(), entry_path.clone()],
    ] {
        let error = SchemaRegistry::load(repo.path(), &sources, &["schemas".to_string()])
            .expect_err("duplicate identity");
        assert_eq!(error.kind(), RegistryLoadErrorKind::DuplicateIdentity);
    }
}

#[test]
fn duplicate_json_missing_refs_symlinks_and_non_schema_pointer_targets_refuse() {
    let repo = tempfile::tempdir().unwrap();
    let duplicate = br#"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"string","type":"number"}"#;
    write(
        &repo.path().join("schemas/duplicate.schema.json"),
        duplicate,
    );
    write_entry(
        repo.path(),
        "definitions/duplicate-json.yaml",
        "example.schemas.duplicate-json",
        "schemas/duplicate.schema.json",
        &[("schemas/duplicate.schema.json", duplicate.as_slice())],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/duplicate-json.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("duplicate JSON key");
    assert_eq!(error.kind(), RegistryLoadErrorKind::DuplicateKey);

    let root = json!({"$schema": DIALECT, "$ref": "missing.schema.json"});
    let root_bytes = schema_bytes(&root);
    write(
        &repo.path().join("schemas/missing-root.schema.json"),
        &root_bytes,
    );
    write_entry(
        repo.path(),
        "definitions/missing.yaml",
        "example.schemas.missing",
        "schemas/missing-root.schema.json",
        &[("schemas/missing-root.schema.json", root_bytes.as_slice())],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/missing.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("missing local ref");
    assert_eq!(error.kind(), RegistryLoadErrorKind::LocalReferenceMissing);

    let outside = json!({"$schema": DIALECT, "type": "string"});
    let outside_bytes = schema_bytes(&outside);
    write(
        &repo.path().join("outside/root.schema.json"),
        &outside_bytes,
    );
    write_entry(
        repo.path(),
        "definitions/outside-root.yaml",
        "example.schemas.outside-root",
        "outside/root.schema.json",
        &[("outside/root.schema.json", outside_bytes.as_slice())],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/outside-root.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("entry document outside allowed root");
    assert_eq!(
        error.kind(),
        RegistryLoadErrorKind::LocalReferenceOutsideRoot
    );

    let pointer_to_data = json!({
        "$schema": DIALECT,
        "$ref": "#/const/not-a-schema",
        "const": {"not-a-schema": {"type": "string"}}
    });
    let bytes = schema_bytes(&pointer_to_data);
    write(
        &repo.path().join("schemas/data-pointer.schema.json"),
        &bytes,
    );
    write_entry(
        repo.path(),
        "definitions/data-pointer.yaml",
        "example.schemas.data-pointer",
        "schemas/data-pointer.schema.json",
        &[("schemas/data-pointer.schema.json", bytes.as_slice())],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/data-pointer.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("pointer target must be a schema position");
    assert_eq!(error.kind(), RegistryLoadErrorKind::ValidatorTargetMismatch);

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let target = json!({"$schema": DIALECT, "type": "string"});
        let target_bytes = schema_bytes(&target);
        write(
            &repo.path().join("outside/target.schema.json"),
            &target_bytes,
        );
        symlink(
            "../outside/target.schema.json",
            repo.path().join("schemas/link.schema.json"),
        )
        .unwrap();
        let root = json!({"$schema": DIALECT, "$ref": "link.schema.json"});
        let root_bytes = schema_bytes(&root);
        write(
            &repo.path().join("schemas/link-root.schema.json"),
            &root_bytes,
        );
        write_entry(
            repo.path(),
            "definitions/symlink.yaml",
            "example.schemas.symlink",
            "schemas/link-root.schema.json",
            &[
                ("schemas/link-root.schema.json", root_bytes.as_slice()),
                ("schemas/link.schema.json", target_bytes.as_slice()),
            ],
        );
        let error = SchemaRegistry::load(
            repo.path(),
            &["definitions/symlink.yaml".to_string()],
            &["schemas".to_string()],
        )
        .expect_err("symlinked ref");
        assert_eq!(error.kind(), RegistryLoadErrorKind::SymlinkSource);
    }
}

#[test]
fn every_ambient_ref_form_and_invalid_pointer_fragment_refuses() {
    for (reference, expected) in [
        (
            "file:///tmp/schema.json",
            RegistryLoadErrorKind::RemoteReferenceRefused,
        ),
        (
            "data:application/json,{}",
            RegistryLoadErrorKind::RemoteReferenceRefused,
        ),
        (
            "unknown:child",
            RegistryLoadErrorKind::RemoteReferenceRefused,
        ),
        (
            "child.schema.json?revision=1",
            RegistryLoadErrorKind::RemoteReferenceRefused,
        ),
        (
            "child\\schema.json",
            RegistryLoadErrorKind::RemoteReferenceRefused,
        ),
        (
            "%2e%2e/outside.json",
            RegistryLoadErrorKind::RemoteReferenceRefused,
        ),
        ("#plain-anchor", RegistryLoadErrorKind::InvalidJsonPointer),
        ("#/$defs/~2bad", RegistryLoadErrorKind::InvalidJsonPointer),
    ] {
        let repo = tempfile::tempdir().unwrap();
        let root = json!({"$schema": DIALECT, "$ref": reference});
        let bytes = schema_bytes(&root);
        write(&repo.path().join("schemas/root.schema.json"), &bytes);
        write_entry(
            repo.path(),
            "definitions/entry.yaml",
            "example.schemas.bad-ref",
            "schemas/root.schema.json",
            &[("schemas/root.schema.json", bytes.as_slice())],
        );
        let error = SchemaRegistry::load(
            repo.path(),
            &["definitions/entry.yaml".to_string()],
            &["schemas".to_string()],
        )
        .expect_err(reference);
        assert_eq!(error.kind(), expected, "ref: {reference}");
    }
}

#[test]
fn same_document_reference_cycles_and_over_depth_closures_refuse() {
    let repo = tempfile::tempdir().unwrap();
    let cycle = json!({
        "$schema": DIALECT,
        "$defs": {
            "a": {"$ref": "#/$defs/b"},
            "b": {"$ref": "#/$defs/a"}
        },
        "$ref": "#/$defs/a"
    });
    let cycle_bytes = schema_bytes(&cycle);
    write(&repo.path().join("schemas/cycle.schema.json"), &cycle_bytes);
    write_entry(
        repo.path(),
        "definitions/cycle.yaml",
        "example.schemas.same-document-cycle",
        "schemas/cycle.schema.json",
        &[("schemas/cycle.schema.json", cycle_bytes.as_slice())],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/cycle.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("same-document cycle");
    assert_eq!(error.kind(), RegistryLoadErrorKind::LocalReferenceCycle);

    let repo = tempfile::tempdir().unwrap();
    let mut documents = Vec::new();
    for index in 0..34 {
        let value = if index == 33 {
            json!({"$schema": DIALECT, "type": "string"})
        } else {
            json!({"$schema": DIALECT, "$ref": format!("{next}.schema.json", next = index + 1)})
        };
        let bytes = schema_bytes(&value);
        let path = format!("schemas/{index}.schema.json");
        write(&repo.path().join(&path), &bytes);
        documents.push((path, bytes));
    }
    let borrowed = documents
        .iter()
        .map(|(path, bytes)| (path.as_str(), bytes.as_slice()))
        .collect::<Vec<_>>();
    write_entry(
        repo.path(),
        "definitions/deep.yaml",
        "example.schemas.too-deep",
        "schemas/0.schema.json",
        &borrowed,
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/deep.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("over-depth closure");
    assert_eq!(error.kind(), RegistryLoadErrorKind::ReferenceDepthExceeded);
}

#[test]
fn boolean_subschemas_are_valid_targets_but_alias_paths_refuse() {
    let repo = tempfile::tempdir().unwrap();
    let root = json!({
        "$schema": DIALECT,
        "$defs": {"allowed": true},
        "$ref": "#/$defs/allowed"
    });
    let bytes = schema_bytes(&root);
    write(&repo.path().join("schemas/root.schema.json"), &bytes);
    write_entry(
        repo.path(),
        "definitions/boolean.yaml",
        "example.schemas.boolean-target",
        "schemas/root.schema.json",
        &[("schemas/root.schema.json", bytes.as_slice())],
    );
    SchemaRegistry::load(
        repo.path(),
        &["definitions/boolean.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect("boolean subschema target");

    for alias in ["./child.schema.json", "nested//child.schema.json"] {
        let repo = tempfile::tempdir().unwrap();
        let root = json!({"$schema": DIALECT, "$ref": alias});
        let bytes = schema_bytes(&root);
        write(&repo.path().join("schemas/root.schema.json"), &bytes);
        write_entry(
            repo.path(),
            "definitions/alias.yaml",
            "example.schemas.alias",
            "schemas/root.schema.json",
            &[("schemas/root.schema.json", bytes.as_slice())],
        );
        let error = SchemaRegistry::load(
            repo.path(),
            &["definitions/alias.yaml".to_string()],
            &["schemas".to_string()],
        )
        .expect_err(alias);
        assert_eq!(error.kind(), RegistryLoadErrorKind::RemoteReferenceRefused);
    }
}

fn same_document_chain(edge_count: usize) -> Value {
    let mut definitions = serde_json::Map::new();
    for index in 0..edge_count {
        let value = if index + 1 == edge_count {
            json!({"type": "string"})
        } else {
            json!({"$ref": format!("#/$defs/node{}", index + 1)})
        };
        definitions.insert(format!("node{index}"), value);
    }
    json!({
        "$schema": DIALECT,
        "$defs": definitions,
        "$ref": "#/$defs/node0"
    })
}

#[test]
fn every_local_ref_edge_counts_toward_depth_and_nested_cycles_refuse() {
    for (edges, expected) in [
        (32, None),
        (33, Some(RegistryLoadErrorKind::ReferenceDepthExceeded)),
    ] {
        let repo = tempfile::tempdir().unwrap();
        let schema = same_document_chain(edges);
        let bytes = schema_bytes(&schema);
        write(&repo.path().join("schemas/root.schema.json"), &bytes);
        write_entry(
            repo.path(),
            "definitions/entry.yaml",
            &format!("example.schemas.fragment-depth-{edges}"),
            "schemas/root.schema.json",
            &[("schemas/root.schema.json", bytes.as_slice())],
        );
        let result = SchemaRegistry::load(
            repo.path(),
            &["definitions/entry.yaml".to_string()],
            &["schemas".to_string()],
        );
        match expected {
            Some(kind) => assert_eq!(result.expect_err("over depth").kind(), kind),
            None => {
                result.expect("32 local reference edges are admitted");
            }
        }
    }

    let repo = tempfile::tempdir().unwrap();
    let nested_cycle = json!({
        "$schema": DIALECT,
        "$defs": {
            "recursive": {
                "type": "object",
                "properties": {
                    "child": {"$ref": "#/$defs/recursive"}
                }
            }
        },
        "$ref": "#/$defs/recursive"
    });
    let bytes = schema_bytes(&nested_cycle);
    write(&repo.path().join("schemas/root.schema.json"), &bytes);
    write_entry(
        repo.path(),
        "definitions/entry.yaml",
        "example.schemas.nested-cycle",
        "schemas/root.schema.json",
        &[("schemas/root.schema.json", bytes.as_slice())],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/entry.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("nested recursive reference");
    assert_eq!(error.kind(), RegistryLoadErrorKind::LocalReferenceCycle);

    let repo = tempfile::tempdir().unwrap();
    let root = json!({"$schema": DIALECT, "$ref": "child.schema.json#/$defs/node0"});
    let child = same_document_chain(33);
    let root_bytes = schema_bytes(&root);
    let child_bytes = schema_bytes(&child);
    write(&repo.path().join("schemas/root.schema.json"), &root_bytes);
    write(&repo.path().join("schemas/child.schema.json"), &child_bytes);
    write_entry(
        repo.path(),
        "definitions/entry.yaml",
        "example.schemas.mixed-depth",
        "schemas/root.schema.json",
        &[
            ("schemas/root.schema.json", root_bytes.as_slice()),
            ("schemas/child.schema.json", child_bytes.as_slice()),
        ],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/entry.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("mixed cross-document and fragment depth");
    assert_eq!(error.kind(), RegistryLoadErrorKind::ReferenceDepthExceeded);
}

#[test]
fn loader_limits_document_count_and_aggregate_source_bytes() {
    let repo = tempfile::tempdir().unwrap();
    let oversized = vec![b' '; MAX_SOURCE_DOCUMENT_BYTES + 1];
    write(
        &repo.path().join("schemas/oversized.schema.json"),
        &oversized,
    );
    write_entry(
        repo.path(),
        "definitions/oversized.yaml",
        "example.schemas.oversized",
        "schemas/oversized.schema.json",
        &[("schemas/oversized.schema.json", oversized.as_slice())],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/oversized.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("oversized schema source");
    assert_eq!(error.kind(), RegistryLoadErrorKind::SourceLimitExceeded);

    let repo = tempfile::tempdir().unwrap();
    let mut documents = Vec::new();
    for index in 0..129 {
        let value = if index == 0 {
            json!({
                "$schema": DIALECT,
                "allOf": (1..129)
                    .map(|target| json!({"$ref": format!("{target}.schema.json")}))
                    .collect::<Vec<_>>()
            })
        } else {
            json!({"$schema": DIALECT, "type": "string"})
        };
        let bytes = schema_bytes(&value);
        let path = format!("schemas/{index}.schema.json");
        write(&repo.path().join(&path), &bytes);
        documents.push((path, bytes));
    }
    let borrowed = documents
        .iter()
        .map(|(path, bytes)| (path.as_str(), bytes.as_slice()))
        .collect::<Vec<_>>();
    write_entry(
        repo.path(),
        "definitions/entry.yaml",
        "example.schemas.over-count",
        "schemas/0.schema.json",
        &borrowed,
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/entry.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("over document count");
    assert_eq!(error.kind(), RegistryLoadErrorKind::DocumentLimitExceeded);

    let repo = tempfile::tempdir().unwrap();
    let schema = json!({"$schema": DIALECT, "type": "string"});
    let schema_bytes = schema_bytes(&schema);
    write(&repo.path().join("schemas/root.schema.json"), &schema_bytes);
    let mut entries = Vec::new();
    for index in 0..10 {
        let path = format!("definitions/{index}.yaml");
        write_entry(
            repo.path(),
            &path,
            &format!("example.schemas.aggregate-{index}"),
            "schemas/root.schema.json",
            &[("schemas/root.schema.json", schema_bytes.as_slice())],
        );
        let entry_path = repo.path().join(&path);
        let current = std::fs::read(&entry_path).unwrap();
        let padding = MAX_SOURCE_DOCUMENT_BYTES - current.len();
        let mut padded = current;
        padded.extend_from_slice(b"#");
        padded.extend(std::iter::repeat_n(b' ', padding - 2));
        padded.push(b'\n');
        assert_eq!(padded.len(), MAX_SOURCE_DOCUMENT_BYTES);
        write(&entry_path, &padded);
        entries.push(path);
    }
    let error = SchemaRegistry::load(repo.path(), &entries, &["schemas".to_string()])
        .expect_err("aggregate loader limit");
    assert_eq!(error.kind(), RegistryLoadErrorKind::AggregateLimitExceeded);
}

#[test]
fn schema_profile_covers_every_frozen_identifier_keyword_and_data_container() {
    for keyword in ["$dynamicAnchor", "$recursiveAnchor", "$recursiveRef"] {
        let repo = tempfile::tempdir().unwrap();
        let schema = json!({"$schema": DIALECT, keyword: "sentinel"});
        let bytes = schema_bytes(&schema);
        write(&repo.path().join("schemas/root.schema.json"), &bytes);
        write_entry(
            repo.path(),
            "definitions/entry.yaml",
            "example.schemas.identifier-keyword",
            "schemas/root.schema.json",
            &[("schemas/root.schema.json", bytes.as_slice())],
        );
        let error = SchemaRegistry::load(
            repo.path(),
            &["definitions/entry.yaml".to_string()],
            &["schemas".to_string()],
        )
        .expect_err(keyword);
        assert_eq!(
            error.kind(),
            RegistryLoadErrorKind::UnsupportedSchemaIdentifier
        );
    }

    for keyword in [
        "$vocabulary",
        "contentEncoding",
        "contentMediaType",
        "contentSchema",
    ] {
        let repo = tempfile::tempdir().unwrap();
        let schema = json!({"$schema": DIALECT, keyword: {"forbidden": true}});
        let bytes = schema_bytes(&schema);
        write(&repo.path().join("schemas/root.schema.json"), &bytes);
        write_entry(
            repo.path(),
            "definitions/entry.yaml",
            "example.schemas.unknown-keyword",
            "schemas/root.schema.json",
            &[("schemas/root.schema.json", bytes.as_slice())],
        );
        let error = SchemaRegistry::load(
            repo.path(),
            &["definitions/entry.yaml".to_string()],
            &["schemas".to_string()],
        )
        .expect_err(keyword);
        assert_eq!(
            error.kind(),
            RegistryLoadErrorKind::UnsupportedSchemaKeyword
        );
    }

    let repo = tempfile::tempdir().unwrap();
    let schema = json!({
        "$schema": DIALECT,
        "type": "object",
        "properties": {
            "payload": {
                "enum": [{"$id": "data", "unknown": true}],
                "const": {"format": "data"},
                "default": {"$anchor": "data"},
                "examples": [{"contentEncoding": "data"}]
            }
        }
    });
    let bytes = schema_bytes(&schema);
    write(&repo.path().join("schemas/root.schema.json"), &bytes);
    write_entry(
        repo.path(),
        "definitions/entry.yaml",
        "example.schemas.instance-data-containers",
        "schemas/root.schema.json",
        &[("schemas/root.schema.json", bytes.as_slice())],
    );
    SchemaRegistry::load(
        repo.path(),
        &["definitions/entry.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect("object-valued instance data is not schema traversal");
}

#[test]
fn conflicting_schema_identities_refuse_in_both_source_orders() {
    let repo = tempfile::tempdir().unwrap();
    let first = schema_bytes(&json!({"$schema": DIALECT, "type": "string"}));
    let second = schema_bytes(&json!({"$schema": DIALECT, "type": "number"}));
    write(&repo.path().join("schemas/first.schema.json"), &first);
    write(&repo.path().join("schemas/second.schema.json"), &second);
    write_entry(
        repo.path(),
        "definitions/first.yaml",
        "example.schemas.conflict",
        "schemas/first.schema.json",
        &[("schemas/first.schema.json", first.as_slice())],
    );
    write_entry(
        repo.path(),
        "definitions/second.yaml",
        "example.schemas.conflict",
        "schemas/second.schema.json",
        &[("schemas/second.schema.json", second.as_slice())],
    );
    for entries in [
        vec![
            "definitions/first.yaml".into(),
            "definitions/second.yaml".into(),
        ],
        vec![
            "definitions/second.yaml".into(),
            "definitions/first.yaml".into(),
        ],
    ] {
        let error = SchemaRegistry::load(repo.path(), &entries, &["schemas".to_string()])
            .expect_err("conflicting identity");
        assert_eq!(error.kind(), RegistryLoadErrorKind::ConflictingIdentity);
    }
}

#[test]
fn registry_errors_do_not_echo_absolute_paths_or_refused_reference_content() {
    let repo = tempfile::tempdir().unwrap();
    let absolute = "/Users/alice/SECRET_SENTINEL.yaml";
    let error = SchemaRegistry::load(
        repo.path(),
        &[absolute.to_string()],
        &["schemas".to_string()],
    )
    .expect_err("absolute path");
    for exposed in [error.location().unwrap_or_default(), error.detail()] {
        assert!(!exposed.contains("/Users/alice"));
        assert!(!exposed.contains("SECRET_SENTINEL"));
    }
    assert!(!error.to_string().contains("SECRET_SENTINEL"));

    let meta_secret = format!("SECRET_META{}", "x".repeat(100_000));
    let invalid_meta = json!({"$schema": DIALECT, "type": meta_secret});
    let invalid_meta_bytes = schema_bytes(&invalid_meta);
    write(
        &repo.path().join("schemas/invalid-meta.schema.json"),
        &invalid_meta_bytes,
    );
    write_entry(
        repo.path(),
        "definitions/invalid-meta.yaml",
        "example.schemas.invalid-meta",
        "schemas/invalid-meta.schema.json",
        &[(
            "schemas/invalid-meta.schema.json",
            invalid_meta_bytes.as_slice(),
        )],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/invalid-meta.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("invalid meta-shape");
    assert_eq!(error.kind(), RegistryLoadErrorKind::UnsupportedDialect);
    assert!(!error.detail().contains("SECRET_META"));
    assert!(!error.to_string().contains("SECRET_META"));
    assert!(error.detail().len() < 256);

    let pattern_secret = format!("SECRET_PATTERN{}[", "x".repeat(100_000));
    let invalid_pattern = json!({
        "$schema": DIALECT,
        "type": "string",
        "pattern": pattern_secret,
    });
    let invalid_pattern_bytes = schema_bytes(&invalid_pattern);
    write(
        &repo.path().join("schemas/invalid-pattern.schema.json"),
        &invalid_pattern_bytes,
    );
    write_entry(
        repo.path(),
        "definitions/invalid-pattern.yaml",
        "example.schemas.invalid-pattern",
        "schemas/invalid-pattern.schema.json",
        &[(
            "schemas/invalid-pattern.schema.json",
            invalid_pattern_bytes.as_slice(),
        )],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/invalid-pattern.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("validator construction failure");
    assert_eq!(
        error.kind(),
        RegistryLoadErrorKind::StructuralValidationSetup
    );
    assert!(!error.detail().contains("SECRET_PATTERN"));
    assert!(!error.to_string().contains("SECRET_PATTERN"));
    assert!(error.detail().len() < 256);

    let reference = format!("child.schema.json?SECRET_SENTINEL={}", "x".repeat(16_384));
    let schema = json!({"$schema": DIALECT, "$ref": reference});
    let bytes = schema_bytes(&schema);
    write(&repo.path().join("schemas/root.schema.json"), &bytes);
    write_entry(
        repo.path(),
        "definitions/entry.yaml",
        "example.schemas.redacted-ref",
        "schemas/root.schema.json",
        &[("schemas/root.schema.json", bytes.as_slice())],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/entry.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("refused reference");
    assert!(!error.detail().contains("SECRET_SENTINEL"));
    assert!(!error.to_string().contains("SECRET_SENTINEL"));
    assert!(error.detail().len() < 256);

    let fragment = format!("#/SECRET_SENTINEL{}", "x".repeat(16_384));
    let schema = json!({"$schema": DIALECT, "$ref": fragment});
    let bytes = schema_bytes(&schema);
    write(&repo.path().join("schemas/fragment.schema.json"), &bytes);
    write_entry(
        repo.path(),
        "definitions/fragment.yaml",
        "example.schemas.redacted-fragment",
        "schemas/fragment.schema.json",
        &[("schemas/fragment.schema.json", bytes.as_slice())],
    );
    let error = SchemaRegistry::load(
        repo.path(),
        &["definitions/fragment.yaml".to_string()],
        &["schemas".to_string()],
    )
    .expect_err("missing large fragment");
    assert!(!error
        .location()
        .unwrap_or_default()
        .contains("SECRET_SENTINEL"));
    assert!(!error.to_string().contains("SECRET_SENTINEL"));
}

#[allow(dead_code)]
fn assert_structural_error_is_public(_: StructuralValidationError) {}
