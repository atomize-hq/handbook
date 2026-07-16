use handbook_engine::{
    load_artifact_kind_registry, ArtifactKindRegistryLoadRequest, DefinitionFingerprint,
    ExactDefinitionRef, RegistryLoadErrorKind,
};
use serde_json::{json, Value};
use std::path::Path;

const DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";
const ROLE_REGISTRY_REF: &str = "handbook.roles.core@1.1.0";
const ROLE_REGISTRY_FINGERPRINT: &str =
    "sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029";

fn write(path: &Path, bytes: impl AsRef<[u8]>) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(path, bytes).expect("write fixture");
}

fn fingerprint(value: &Value) -> String {
    DefinitionFingerprint::from_json_value(value)
        .expect("fingerprint")
        .to_string()
}

fn write_schema_entry(repo: &Path, identity: &str) -> (String, String, String) {
    let document_ref = format!("schemas/{identity}.schema.json");
    let entry_path = format!("definitions/{identity}.schema-entry.yaml");
    let schema = json!({
        "$schema": DIALECT,
        "type": "object",
        "properties": {"title": {"type": "string", "minLength": 3}},
        "required": ["title"],
        "additionalProperties": false
    });
    let schema_bytes = serde_json::to_vec_pretty(&schema).unwrap();
    write(&repo.join(&document_ref), &schema_bytes);
    let document_fingerprint = DefinitionFingerprint::from_bytes(&schema_bytes).to_string();
    let closure_fingerprint = fingerprint(&json!([{
        "document_ref": document_ref,
        "document_fingerprint": document_fingerprint,
    }]));
    let content_schema_id = format!("example.schemas.{identity}");
    let entry = json!({
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
    let entry_fingerprint = fingerprint(&entry);
    let mut authored = entry.as_object().unwrap().clone();
    authored.insert(
        "entry_fingerprint".to_string(),
        Value::String(entry_fingerprint.clone()),
    );
    write(
        &repo.join(&entry_path),
        serde_yaml_bw::to_string(&authored).unwrap(),
    );
    (
        entry_path,
        format!("{content_schema_id}@1.0.0"),
        entry_fingerprint,
    )
}

fn kind_record(kind_id: &str, schema_ref: &str) -> Value {
    json!({
        "schema_id": "handbook.artifact-kind-definition",
        "schema_version": "1.0",
        "kind_id": kind_id,
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
    })
}

fn write_kind(
    repo: &Path,
    path: &str,
    mut record: Value,
    schema_entry_fingerprint: &str,
    schema_closure_fingerprint: &str,
) {
    record["supported_role_refs"]
        .as_array_mut()
        .unwrap()
        .sort_by(|left, right| left.as_str().cmp(&right.as_str()));
    let definition_fingerprint = fingerprint(&json!({
        "definition": record,
        "stable_role_registry_fingerprint": ROLE_REGISTRY_FINGERPRINT,
        "schema_entry_fingerprint": schema_entry_fingerprint,
        "schema_closure_fingerprint": schema_closure_fingerprint,
    }));
    record.as_object_mut().unwrap().insert(
        "definition_fingerprint".into(),
        definition_fingerprint.into(),
    );
    write(&repo.join(path), serde_yaml_bw::to_string(&record).unwrap());
}

fn closure_fingerprint(repo: &Path, entry_path: &str, schema_ref: &str) -> String {
    handbook_engine::SchemaRegistry::load(repo, &[entry_path.to_string()], &["schemas".to_string()])
        .unwrap()
        .entry(&ExactDefinitionRef::parse(schema_ref).unwrap())
        .unwrap()
        .closure_fingerprint()
        .to_string()
}

fn request(schema_entries: Vec<String>, kinds: Vec<String>) -> ArtifactKindRegistryLoadRequest {
    ArtifactKindRegistryLoadRequest::new(
        ExactDefinitionRef::parse(ROLE_REGISTRY_REF).unwrap(),
        schema_entries,
        vec!["schemas".to_string()],
        kinds,
    )
}

#[test]
fn capability_free_kind_loads_and_validates_through_its_exact_schema() {
    let repo = tempfile::tempdir().unwrap();
    let (entry_path, schema_ref, entry_fingerprint) = write_schema_entry(repo.path(), "incident");
    let schema_registry = handbook_engine::SchemaRegistry::load(
        repo.path(),
        std::slice::from_ref(&entry_path),
        &["schemas".to_string()],
    )
    .unwrap();
    let schema_exact_ref = ExactDefinitionRef::parse(&schema_ref).unwrap();
    let closure_fingerprint = schema_registry
        .entry(&schema_exact_ref)
        .unwrap()
        .closure_fingerprint()
        .to_string();
    let kind_path = "definitions/incident-kind.yaml".to_string();
    write_kind(
        repo.path(),
        &kind_path,
        kind_record("example.artifact-kind.incident", &schema_ref),
        &entry_fingerprint,
        &closure_fingerprint,
    );

    let registry =
        load_artifact_kind_registry(repo.path(), request(vec![entry_path], vec![kind_path]))
            .expect("kind registry");
    let kind_ref = ExactDefinitionRef::parse("example.artifact-kind.incident@1.0.0").unwrap();
    let definition = registry.kind(&kind_ref).expect("kind");

    assert_eq!(definition.exact_ref(), &kind_ref);
    assert_eq!(definition.canonical_schema_ref(), &schema_exact_ref);
    assert_eq!(definition.supported_role_refs(), ["project_context"]);
    assert!(registry
        .validate_json(&kind_ref, &json!({"title": "Incident"}))
        .is_ok());
    let errors = registry
        .validate_json(&kind_ref, &json!({"title": "x"}))
        .expect_err("invalid instance");
    assert_eq!(errors[0].instance_location(), "/title");
}

#[test]
fn later_owned_and_wrong_record_fields_refuse_before_fingerprinting() {
    let later_owned = [
        (
            "semantic_capabilities",
            json!([{"forged": "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}]),
        ),
        ("required_capabilities", json!([{"capability_id": "x"}])),
        (
            "semantic_validation_profile_refs",
            json!(["example.validator@1.0.0"]),
        ),
        (
            "renderer_definition_refs",
            json!(["example.renderer@1.0.0"]),
        ),
        (
            "projection_definition_refs",
            json!(["example.projection@1.0.0"]),
        ),
        ("lifecycle_policy_ref", json!("example.lifecycle@1.0.0")),
        ("review_triggers", json!(["example.trigger@1.0.0"])),
        ("extensions", json!({"example": true})),
    ];
    for (field, value) in later_owned {
        let repo = tempfile::tempdir().unwrap();
        let identity = field.replace('_', "-");
        let (entry_path, schema_ref, _) = write_schema_entry(repo.path(), &identity);
        let mut record = kind_record(&format!("example.artifact-kind.{identity}"), &schema_ref);
        record.as_object_mut().unwrap().insert(field.into(), value);
        record.as_object_mut().unwrap().insert(
            "definition_fingerprint".into(),
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
        );
        let path = format!("definitions/{field}.kind.yaml");
        write(
            &repo.path().join(&path),
            serde_yaml_bw::to_string(&record).unwrap(),
        );
        let error = load_artifact_kind_registry(repo.path(), request(vec![entry_path], vec![path]))
            .expect_err(field);
        assert_eq!(
            error.kind(),
            RegistryLoadErrorKind::UnsupportedDependency,
            "{field}"
        );
    }

    for field in [
        "path",
        "instance_label",
        "requiredness",
        "setup_state",
        "intake_definition_ref",
        "dependency_producers",
    ] {
        let repo = tempfile::tempdir().unwrap();
        let identity = field.replace('_', "-");
        let (entry_path, schema_ref, _) = write_schema_entry(repo.path(), &identity);
        let mut record = kind_record(&format!("example.artifact-kind.{identity}"), &schema_ref);
        record
            .as_object_mut()
            .unwrap()
            .insert(field.into(), json!(true));
        record.as_object_mut().unwrap().insert(
            "definition_fingerprint".into(),
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into(),
        );
        let path = format!("definitions/{field}.kind.yaml");
        write(
            &repo.path().join(&path),
            serde_yaml_bw::to_string(&record).unwrap(),
        );
        let error = load_artifact_kind_registry(repo.path(), request(vec![entry_path], vec![path]))
            .expect_err(field);
        assert_eq!(error.kind(), RegistryLoadErrorKind::UnknownField, "{field}");
    }
}

#[test]
fn kind_registry_is_order_independent_and_refuses_dependency_mismatches() {
    let repo = tempfile::tempdir().unwrap();
    let (entry_a, schema_a, fp_a) = write_schema_entry(repo.path(), "alpha");
    let (entry_b, schema_b, fp_b) = write_schema_entry(repo.path(), "bravo");
    let schemas = handbook_engine::SchemaRegistry::load(
        repo.path(),
        &[entry_a.clone(), entry_b.clone()],
        &["schemas".into()],
    )
    .unwrap();
    let closure_a = schemas
        .entry(&ExactDefinitionRef::parse(&schema_a).unwrap())
        .unwrap()
        .closure_fingerprint()
        .to_string();
    let closure_b = schemas
        .entry(&ExactDefinitionRef::parse(&schema_b).unwrap())
        .unwrap()
        .closure_fingerprint()
        .to_string();
    let kind_a = "definitions/alpha.kind.yaml".to_string();
    let kind_b = "definitions/bravo.kind.yaml".to_string();
    write_kind(
        repo.path(),
        &kind_a,
        kind_record("example.artifact-kind.alpha", &schema_a),
        &fp_a,
        &closure_a,
    );
    write_kind(
        repo.path(),
        &kind_b,
        kind_record("example.artifact-kind.bravo", &schema_b),
        &fp_b,
        &closure_b,
    );

    let forward = load_artifact_kind_registry(
        repo.path(),
        request(
            vec![entry_a.clone(), entry_b.clone()],
            vec![kind_a.clone(), kind_b.clone()],
        ),
    )
    .unwrap();
    let reverse = load_artifact_kind_registry(
        repo.path(),
        request(vec![entry_b, entry_a], vec![kind_b, kind_a]),
    )
    .unwrap();
    assert_eq!(forward.fingerprint(), reverse.fingerprint());
    assert_eq!(forward.kind_refs(), reverse.kind_refs());
    assert_eq!(
        forward.schema_registry().fingerprint(),
        reverse.schema_registry().fingerprint()
    );

    let mut mismatch = kind_record("example.artifact-kind.mismatch", &schema_a);
    mismatch["stable_role_registry"]["ref"] = json!("handbook.roles.core@1.0.0");
    mismatch["definition_fingerprint"] =
        json!("sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    write(
        &repo.path().join("definitions/mismatch.kind.yaml"),
        serde_yaml_bw::to_string(&mismatch).unwrap(),
    );
    let error = load_artifact_kind_registry(
        repo.path(),
        request(
            vec!["definitions/alpha.schema-entry.yaml".into()],
            vec!["definitions/mismatch.kind.yaml".into()],
        ),
    )
    .expect_err("wrong role registry");
    assert_eq!(
        error.kind(),
        RegistryLoadErrorKind::StableRoleRegistryMismatch
    );
}

#[test]
fn dependency_identity_shape_and_fingerprint_failures_are_typed() {
    enum Mutation {
        Field(&'static str, Value),
        NestedRoleField,
        ChangedBytesAfterFingerprint,
    }

    let cases = [
        (
            Mutation::Field("compatibility", json!("minor")),
            RegistryLoadErrorKind::UnsupportedCompatibility,
        ),
        (
            Mutation::Field("structural_validation_profile_ref", json!("custom")),
            RegistryLoadErrorKind::UnsupportedStructuralValidationProfile,
        ),
        (
            Mutation::Field(
                "stable_role_registry",
                json!({
                    "ref": ROLE_REGISTRY_REF,
                    "fingerprint": "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                }),
            ),
            RegistryLoadErrorKind::StableRoleRegistryMismatch,
        ),
        (
            Mutation::Field("supported_role_refs", json!(["unknown_role"])),
            RegistryLoadErrorKind::UnknownStableRole,
        ),
        (
            Mutation::Field(
                "supported_role_refs",
                json!(["project_context", "project_context"]),
            ),
            RegistryLoadErrorKind::DuplicateIdentity,
        ),
        (
            Mutation::Field(
                "canonical_schema_ref",
                json!("example.artifact-kind.wrong-dependency-type@1.0.0"),
            ),
            RegistryLoadErrorKind::MissingSchema,
        ),
        (
            Mutation::Field("schema_id", json!("handbook.schema-registry-entry")),
            RegistryLoadErrorKind::UnsupportedRecord,
        ),
        (
            Mutation::NestedRoleField,
            RegistryLoadErrorKind::UnknownField,
        ),
        (
            Mutation::ChangedBytesAfterFingerprint,
            RegistryLoadErrorKind::FingerprintMismatch,
        ),
    ];

    for (index, (mutation, expected)) in cases.into_iter().enumerate() {
        let repo = tempfile::tempdir().unwrap();
        let (entry_path, schema_ref, entry_fingerprint) =
            write_schema_entry(repo.path(), &format!("case-{index}"));
        let closure = closure_fingerprint(repo.path(), &entry_path, &schema_ref);
        let mut record = kind_record(&format!("example.artifact-kind.case-{index}"), &schema_ref);
        let path = format!("definitions/case-{index}.kind.yaml");
        match mutation {
            Mutation::Field(field, value) => {
                record.as_object_mut().unwrap().insert(field.into(), value);
                record["definition_fingerprint"] = json!(
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                );
                write(
                    &repo.path().join(&path),
                    serde_yaml_bw::to_string(&record).unwrap(),
                );
            }
            Mutation::NestedRoleField => {
                record["stable_role_registry"]["unknown"] = json!(true);
                record["definition_fingerprint"] = json!(
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                );
                write(
                    &repo.path().join(&path),
                    serde_yaml_bw::to_string(&record).unwrap(),
                );
            }
            Mutation::ChangedBytesAfterFingerprint => {
                write_kind(repo.path(), &path, record, &entry_fingerprint, &closure);
                let bytes = std::fs::read_to_string(repo.path().join(&path)).unwrap();
                write(
                    &repo.path().join(&path),
                    bytes.replace("kind_version: 1.0.0", "kind_version: 1.0.1"),
                );
            }
        }

        let error = load_artifact_kind_registry(repo.path(), request(vec![entry_path], vec![path]))
            .expect_err("mutation must refuse");
        assert_eq!(error.kind(), expected, "case {index}");
    }
}

#[test]
fn duplicate_and_conflicting_kind_identities_refuse_in_both_source_orders() {
    let repo = tempfile::tempdir().unwrap();
    let (entry_path, schema_ref, entry_fingerprint) = write_schema_entry(repo.path(), "identity");
    let closure = closure_fingerprint(repo.path(), &entry_path, &schema_ref);
    let first = "definitions/first.kind.yaml".to_string();
    let second = "definitions/second.kind.yaml".to_string();

    write_kind(
        repo.path(),
        &first,
        kind_record("example.artifact-kind.identity", &schema_ref),
        &entry_fingerprint,
        &closure,
    );
    let duplicate = load_artifact_kind_registry(
        repo.path(),
        request(vec![entry_path.clone()], vec![first.clone(), first.clone()]),
    )
    .expect_err("duplicate");
    assert_eq!(duplicate.kind(), RegistryLoadErrorKind::DuplicateIdentity);

    let mut changed = kind_record("example.artifact-kind.identity", &schema_ref);
    changed["supported_role_refs"] = json!(["environment_context"]);
    write_kind(repo.path(), &second, changed, &entry_fingerprint, &closure);
    for sources in [
        vec![first.clone(), second.clone()],
        vec![second.clone(), first.clone()],
    ] {
        let conflict =
            load_artifact_kind_registry(repo.path(), request(vec![entry_path.clone()], sources))
                .expect_err("conflict");
        assert_eq!(conflict.kind(), RegistryLoadErrorKind::ConflictingIdentity);
    }
}

#[test]
fn kind_sources_reuse_the_repo_relative_no_follow_boundary() {
    let repo = tempfile::tempdir().unwrap();
    let (entry_path, _, _) = write_schema_entry(repo.path(), "source-boundary");
    for (path, expected) in [
        (
            "definitions/missing.kind.yaml",
            RegistryLoadErrorKind::MissingSource,
        ),
        (
            "../escape.kind.yaml",
            RegistryLoadErrorKind::InvalidSourcePath,
        ),
    ] {
        let error = load_artifact_kind_registry(
            repo.path(),
            request(vec![entry_path.clone()], vec![path.into()]),
        )
        .expect_err(path);
        assert_eq!(error.kind(), expected);
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        write(
            &repo.path().join("definitions/target.kind.yaml"),
            b"not: admitted\n",
        );
        symlink(
            "target.kind.yaml",
            repo.path().join("definitions/link.kind.yaml"),
        )
        .unwrap();
        let error = load_artifact_kind_registry(
            repo.path(),
            request(vec![entry_path], vec!["definitions/link.kind.yaml".into()]),
        )
        .expect_err("symlink");
        assert_eq!(error.kind(), RegistryLoadErrorKind::SymlinkSource);
    }
}
