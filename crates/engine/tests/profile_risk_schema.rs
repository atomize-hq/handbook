use handbook_engine::{ExactDefinitionRef, SchemaRegistry};
use serde_json::{json, Value};
use std::path::Path;
const ID: &str = "handbook.schemas.artifacts.risk-record";
const ENTRY: &[u8] = include_bytes!(
    "../definitions/schemas/handbook.schemas.artifacts.risk-record/1.0.0.entry.yaml"
);
const SCHEMA: &[u8] = include_bytes!(
    "../definitions/schemas/handbook.schemas.artifacts.risk-record/1.0.0.schema.json"
);
fn write(p: &Path, b: &[u8]) {
    if let Some(x) = p.parent() {
        std::fs::create_dir_all(x).unwrap();
    }
    std::fs::write(p, b).unwrap();
}
#[test]
fn risk_record_is_exact_closed_bounded_and_not_root_authority() {
    let repo = tempfile::tempdir().unwrap();
    let base = format!("definitions/schemas/{ID}/1.0.0");
    write(&repo.path().join(format!("{base}.entry.yaml")), ENTRY);
    write(&repo.path().join(format!("{base}.schema.json")), SCHEMA);
    let registry = SchemaRegistry::load(
        repo.path(),
        &[format!("{base}.entry.yaml")],
        &["definitions/schemas".into()],
    )
    .unwrap();
    let schema = registry
        .resolved(&ExactDefinitionRef::parse(&format!("{ID}@1.0.0")).unwrap())
        .unwrap();
    let valid = json!({"schema_id":"handbook.artifact.risk-record","schema_version":"1.0","record_id":"example.record.risk","uncertainty":"Delivery may slip","evidence_refs":[],"owner":"Team","treatment":"Track and mitigate","status":"open","review_basis":["example.reference.review"]});
    assert!(schema.validate_json(&valid).is_ok());
    for field in [
        "uncertainty",
        "evidence_refs",
        "owner",
        "treatment",
        "status",
        "review_basis",
    ] {
        let mut v = valid.clone();
        v.as_object_mut().unwrap().remove(field);
        assert!(schema.validate_json(&v).is_err(), "{field}");
    }
    for status in ["open", "monitoring", "mitigated", "accepted", "closed"] {
        let mut v = valid.clone();
        v["status"] = json!(status);
        assert!(schema.validate_json(&v).is_ok());
    }
    let mut empty = valid.clone();
    empty["review_basis"] = json!([]);
    assert!(schema.validate_json(&empty).is_err());
    let mut dup = valid.clone();
    dup["review_basis"] = json!(["example.reference.review", "example.reference.review"]);
    assert!(schema.validate_json(&dup).is_err());
    let mut owner = valid.clone();
    owner["owner"] = json!("x".repeat(256));
    assert!(schema.validate_json(&owner).is_ok());
    owner["owner"] = json!("x".repeat(257));
    assert!(schema.validate_json(&owner).is_err());
    let mut refs = valid.clone();
    refs["evidence_refs"] = Value::Array(
        (0..128)
            .map(|i| json!(format!("example.reference.r{i}")))
            .collect(),
    );
    assert!(schema.validate_json(&refs).is_ok());
    refs["evidence_refs"]
        .as_array_mut()
        .unwrap()
        .push(json!("example.reference.overflow"));
    assert!(schema.validate_json(&refs).is_err());
    let mut root = valid;
    root["selected_by_default"] = json!(true);
    assert!(schema.validate_json(&root).is_err());
}
