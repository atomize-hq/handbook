#![cfg(feature = "contract-schema-generation")]

use handbook_sdk::contract_membrane::schema_generation::derive_hcm_5_4_packet_1_schema_tree_for_test;
use jsonschema::{Draft, Retrieve, Uri, Validator};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::Path;

const VALIDATION_INPUT_REF: &str = "handbook.dock-json-schema-validation-input@1.0.0";
const FACT_REF: &str = "handbook.fact.schema-conformance@1.0.0";

#[derive(Debug)]
struct RejectExternal;

impl Retrieve for RejectExternal {
    fn retrieve(
        &self,
        uri: &Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        Err(format!("external retrieval forbidden: {uri}").into())
    }
}

fn validator(schema: &Value) -> Validator {
    jsonschema::options()
        .with_draft(Draft::Draft202012)
        .with_retriever(RejectExternal)
        .build(schema)
        .unwrap()
}

#[test]
fn test_only_derivation_adds_exactly_two_independently_retrievable_roots() {
    let tree = derive_hcm_5_4_packet_1_schema_tree_for_test().unwrap();
    assert_eq!(tree.len(), 170);
    assert!(tree.contains_key(VALIDATION_INPUT_REF));
    assert!(tree.contains_key(FACT_REF));

    let checked_manifest: Vec<Value> = serde_json::from_slice(
        &std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas/manifest.json"))
            .unwrap(),
    )
    .unwrap();
    assert_eq!(checked_manifest.len(), 168);
    let predecessor = checked_manifest
        .iter()
        .map(|entry| entry["schema"]["ref"].as_str().unwrap().to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(predecessor.len(), 168);
    assert!(predecessor
        .iter()
        .all(|reference| tree.contains_key(reference)));
    let delta = tree
        .keys()
        .filter(|reference| !predecessor.contains(reference.as_str()))
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    assert_eq!(delta, BTreeSet::from([VALIDATION_INPUT_REF, FACT_REF]));
}

#[test]
fn new_roots_are_deterministic_meta_valid_closed_and_external_ref_free() {
    let first = derive_hcm_5_4_packet_1_schema_tree_for_test().unwrap();
    let second = derive_hcm_5_4_packet_1_schema_tree_for_test().unwrap();
    assert_eq!(first, second);

    for reference in [VALIDATION_INPUT_REF, FACT_REF] {
        let schema: Value = serde_json::from_slice(first.get(reference).unwrap()).unwrap();
        jsonschema::draft202012::meta::validate(&schema)
            .unwrap_or_else(|error| panic!("{reference} is not Draft 2020-12 meta-valid: {error}"));
        assert_local_refs_only(&schema);
        validator(&schema);
        assert_eq!(
            schema["$schema"],
            "https://json-schema.org/draft/2020-12/schema"
        );
        assert!(schema["$id"]
            .as_str()
            .unwrap()
            .starts_with("urn:handbook:schema:"));
    }
}

#[test]
fn roots_accept_exact_vectors_and_reject_unknown_fields_enums_and_invariants() {
    let tree = derive_hcm_5_4_packet_1_schema_tree_for_test().unwrap();
    let input_schema: Value =
        serde_json::from_slice(tree.get(VALIDATION_INPUT_REF).unwrap()).unwrap();
    let fact_schema: Value = serde_json::from_slice(tree.get(FACT_REF).unwrap()).unwrap();
    let input_validator = validator(&input_schema);
    let fact_validator = validator(&fact_schema);

    let root_fingerprint =
        "sha256:21f065b99a46c1d795525e2f941ed4b7fe8f69e6593bc79ef81182897d4faa57";
    let closure_fingerprint =
        "sha256:ae82da6c3732e9f5416dbd70192309734483b7836dbc73dba056ba0ef18b2ef6";
    let instance_fingerprint =
        "sha256:5479921ced5eefedf4712d07ce894893dd28d04df84171944608aed2d6735641";
    let input = json!({
        "root_schema": {"ref": "handbook.schema.example@1.0.0", "fingerprint": root_fingerprint},
        "root_document": "definitions/schemas/example/root/1.0.0.schema.json",
        "root_document_fingerprint": root_fingerprint,
        "closure_fingerprint": closure_fingerprint,
        "documents": [
            {
                "path": "definitions/schemas/example/root/1.0.0.schema.json",
                "schema": {"$schema": "https://json-schema.org/draft/2020-12/schema", "properties": {"name": {"type": "string"}}, "required": ["name"], "type": "object"},
                "document_fingerprint": root_fingerprint
            },
            {
                "path": "definitions/schemas/example/shared/1.0.0.schema.json",
                "schema": {"$schema": "https://json-schema.org/draft/2020-12/schema", "$defs": {"name": {"type": "string"}}, "type": "object"},
                "document_fingerprint": "sha256:7d3fd4b2dbd30bb65f4b01ae77d7d6055fa9cd3ae7c98da0fe28620de2fa522b"
            }
        ],
        "instance": {"value": {"name": "Ada", "scores": [1, 2, 3]}, "instance_fingerprint": instance_fingerprint}
    });
    assert!(input_validator.is_valid(&input));
    let mut unknown = input.clone();
    unknown["unexpected"] = json!(true);
    assert!(!input_validator.is_valid(&unknown));
    for invalid_path in [
        "definitions/schemas/../escape.schema.json",
        "definitions\\schemas\\example.schema.json",
        "definitions/schemas/C:/absolute.schema.json",
        "definitions/schemas/control\nname.schema.json",
    ] {
        let mut invalid = input.clone();
        invalid["root_document"] = json!(invalid_path);
        assert!(!input_validator.is_valid(&invalid), "{invalid_path}");
    }

    let conformant = json!({
        "root_schema": {"ref": "handbook.schema.example@1.0.0", "fingerprint": root_fingerprint},
        "root_document": "definitions/schemas/example/root/1.0.0.schema.json",
        "root_document_fingerprint": root_fingerprint,
        "closure_fingerprint": closure_fingerprint,
        "instance_fingerprint": instance_fingerprint,
        "closure_document_count": 2,
        "conformance": "conformant",
        "violations": []
    });
    assert!(fact_validator.is_valid(&conformant));
    let mut invalid = conformant.clone();
    invalid["conformance"] = json!("unknown");
    assert!(!fact_validator.is_valid(&invalid));
    let mut invalid = conformant.clone();
    invalid["violations"] = json!([{
        "code": "type",
        "instance_pointer": "/name",
        "schema_document": "definitions/schemas/example/root/1.0.0.schema.json",
        "schema_pointer": "/properties/name/type"
    }]);
    assert!(!fact_validator.is_valid(&invalid));
}

fn assert_local_refs_only(value: &Value) {
    match value {
        Value::Array(values) => values.iter().for_each(assert_local_refs_only),
        Value::Object(members) => {
            if let Some(reference) = members.get("$ref") {
                assert!(reference.as_str().unwrap().starts_with("#/$defs/"));
            }
            members.values().for_each(assert_local_refs_only);
        }
        _ => {}
    }
}

#[test]
fn packet3_wire_vectors_bind_the_exact_generated_input_and_fact_schema_bytes() {
    let tree = derive_hcm_5_4_packet_1_schema_tree_for_test().unwrap();
    let request: Value = serde_json::from_slice(include_bytes!(
        "../../dock-json-schema/tests/vectors/process-request-v1.json"
    ))
    .unwrap();
    let result: Value = serde_json::from_slice(include_bytes!(
        "../../dock-json-schema/tests/vectors/process-result-v1.json"
    ))
    .unwrap();

    let input_fingerprint = format!(
        "sha256:{:x}",
        Sha256::digest(tree.get(VALIDATION_INPUT_REF).unwrap())
    );
    let fact_fingerprint = format!("sha256:{:x}", Sha256::digest(tree.get(FACT_REF).unwrap()));
    assert_eq!(request["inputs"][0]["schema"]["ref"], VALIDATION_INPUT_REF);
    assert_eq!(
        request["inputs"][0]["schema"]["fingerprint"],
        input_fingerprint
    );
    assert_eq!(result["candidates"][0]["payload_schema"]["ref"], FACT_REF);
    assert_eq!(
        result["candidates"][0]["payload_schema"]["fingerprint"],
        fact_fingerprint
    );
}
