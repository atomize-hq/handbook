use handbook_engine::{ExactDefinitionRef, SchemaRegistry};
use serde_json::{json, Value};
use std::path::Path;

fn write(path: &Path, bytes: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    std::fs::write(path, bytes).unwrap();
}

fn load(
    identity: &str,
    slug: &str,
    entry: &[u8],
    schema: &[u8],
) -> handbook_engine::ResolvedSchema {
    let repo = tempfile::tempdir().unwrap();
    let base = format!("definitions/schemas/{slug}/1.0.0");
    write(&repo.path().join(format!("{base}.entry.yaml")), entry);
    write(&repo.path().join(format!("{base}.schema.json")), schema);
    let registry = SchemaRegistry::load(
        repo.path(),
        &[format!("{base}.entry.yaml")],
        &["definitions/schemas".into()],
    )
    .unwrap();
    registry
        .resolved(&ExactDefinitionRef::parse(&format!("{identity}@1.0.0")).unwrap())
        .unwrap()
        .clone()
}

const PROJECT_SLUG: &str = "handbook.schemas.artifacts.project-context";
const ENV_SLUG: &str = "handbook.schemas.artifacts.environment-context";
const PROJECT_ENTRY: &[u8] = include_bytes!(
    "../definitions/schemas/handbook.schemas.artifacts.project-context/1.0.0.entry.yaml"
);
const PROJECT_SCHEMA: &[u8] = include_bytes!(
    "../definitions/schemas/handbook.schemas.artifacts.project-context/1.0.0.schema.json"
);
const ENV_ENTRY: &[u8] = include_bytes!(
    "../definitions/schemas/handbook.schemas.artifacts.environment-context/1.0.0.entry.yaml"
);
const ENV_SCHEMA: &[u8] = include_bytes!(
    "../definitions/schemas/handbook.schemas.artifacts.environment-context/1.0.0.schema.json"
);

fn project() -> Value {
    json!({
        "schema_id":"handbook.artifact.project-context", "schema_version":"1.0", "record_id":"example.record.project-context",
        "summary":"Summary", "system_boundaries":["API","Worker"], "ownership":["Team"],
        "authoritative_references":["example.reference.one","handbook.example.definition@1.0.0"], "known_unknowns":[]
    })
}

fn environment() -> Value {
    json!({
        "schema_id":"handbook.artifact.environment-context", "schema_version":"1.0", "record_id":"example.record.environment-context",
        "applicability_basis":["example.reference.basis"], "operational_surfaces":["Production"],
        "runtime_dependencies":["Database"], "safe_configuration_references":["example.configuration.name"],
        "authoritative_references":[], "known_unknowns":[]
    })
}

#[test]
fn project_context_is_exact_closed_and_set_bounded() {
    let schema = load(PROJECT_SLUG, PROJECT_SLUG, PROJECT_ENTRY, PROJECT_SCHEMA);
    let valid = project();
    assert!(schema.validate_json(&valid).is_ok());
    for pointer in [
        "/summary",
        "/system_boundaries",
        "/ownership",
        "/authoritative_references",
        "/known_unknowns",
    ] {
        let mut changed = valid.clone();
        changed.as_object_mut().unwrap().remove(&pointer[1..]);
        assert!(schema.validate_json(&changed).is_err(), "{pointer}");
    }
    let mut reverse = valid.clone();
    reverse["system_boundaries"] = json!(["Worker", "API"]);
    assert!(schema.validate_json(&reverse).is_ok());
    let mut duplicate = valid.clone();
    duplicate["system_boundaries"] = json!(["API", "API"]);
    assert!(schema.validate_json(&duplicate).is_err());
    let mut extra = valid.clone();
    extra["extra"] = json!(true);
    assert!(schema.validate_json(&extra).is_err());
    let mut short = valid.clone();
    short["ownership"] = json!(["x".repeat(256)]);
    assert!(schema.validate_json(&short).is_ok());
    short["ownership"] = json!(["x".repeat(257)]);
    assert!(schema.validate_json(&short).is_err());
    let mut long = valid.clone();
    long["summary"] = json!("x".repeat(8192));
    assert!(schema.validate_json(&long).is_ok());
    long["summary"] = json!("x".repeat(8193));
    assert!(schema.validate_json(&long).is_err());
    let mut refs = valid;
    refs["authoritative_references"] = Value::Array(
        (0..128)
            .map(|i| json!(format!("example.reference.r{i}")))
            .collect(),
    );
    assert!(schema.validate_json(&refs).is_ok());
    refs["authoritative_references"]
        .as_array_mut()
        .unwrap()
        .push(json!("example.reference.overflow"));
    assert!(schema.validate_json(&refs).is_err());
}

#[test]
fn environment_context_refuses_secrets_and_requires_applicability() {
    let schema = load(ENV_SLUG, ENV_SLUG, ENV_ENTRY, ENV_SCHEMA);
    let valid = environment();
    assert!(schema.validate_json(&valid).is_ok());
    let mut reverse = valid.clone();
    reverse["operational_surfaces"] = json!(["Worker", "API"]);
    assert!(schema.validate_json(&reverse).is_ok());
    let mut empty = valid.clone();
    empty["applicability_basis"] = json!([]);
    assert!(schema.validate_json(&empty).is_err());
    let mut duplicate = valid.clone();
    duplicate["applicability_basis"] =
        json!(["example.reference.basis", "example.reference.basis"]);
    assert!(schema.validate_json(&duplicate).is_err());
    for forbidden in [
        "secret",
        "secret_value",
        "token",
        "password",
        "credential",
        "private_key",
        "environment_values",
    ] {
        let mut changed = valid.clone();
        changed[forbidden] = json!("value");
        assert!(schema.validate_json(&changed).is_err(), "{forbidden}");
    }
    for bad in [
        "/absolute",
        "https://example.com",
        "has whitespace",
        "xy",
        "example.ref@latest",
    ] {
        let mut changed = valid.clone();
        changed["safe_configuration_references"] = json!([bad]);
        assert!(schema.validate_json(&changed).is_err(), "{bad}");
    }
}
