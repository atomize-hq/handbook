use handbook_dock_json_schema::{
    evaluate_json_schema_v1, ConformanceV1, ExactSchemaBindingV1, JsonInstanceV1,
    JsonSchemaValidationInputV1, LogicalSchemaPathV1, SchemaClosureMemberV1, ValidatorRefusalV1,
    ViolationCodeV1,
};
use serde_json::{json, Value};

const ROOT: &str = "definitions/schemas/example/root/1.0.0.schema.json";
const SHARED: &str = "definitions/schemas/example/root/shared.schema.json";
const DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";

fn input(
    root: Value,
    additional: Vec<(&str, Value)>,
    instance: Value,
) -> JsonSchemaValidationInputV1 {
    input_at(ROOT, root, additional, instance)
}

fn input_at(
    root_path: &str,
    root: Value,
    additional: Vec<(&str, Value)>,
    instance: Value,
) -> JsonSchemaValidationInputV1 {
    let root =
        SchemaClosureMemberV1::new(LogicalSchemaPathV1::parse(root_path).unwrap(), root).unwrap();
    let binding = ExactSchemaBindingV1::parse(
        "handbook.schema.packet-2-test@1.0.0",
        root.document_fingerprint().clone(),
    )
    .unwrap();
    let mut documents = vec![root];
    documents.extend(additional.into_iter().map(|(path, schema)| {
        SchemaClosureMemberV1::new(LogicalSchemaPathV1::parse(path).unwrap(), schema).unwrap()
    }));
    documents.sort_by(|left, right| left.path().cmp(right.path()));
    JsonSchemaValidationInputV1::new(
        binding,
        LogicalSchemaPathV1::parse(root_path).unwrap(),
        documents,
        JsonInstanceV1::new(instance).unwrap(),
    )
    .unwrap()
}

fn violation_code(schema: Value, instance: Value) -> ViolationCodeV1 {
    let fact = evaluate_json_schema_v1(&input(schema, vec![], instance)).unwrap();
    assert_eq!(fact.conformance(), ConformanceV1::Nonconformant);
    assert_eq!(fact.violations().len(), 1, "{fact:?}");
    fact.violations()[0].code()
}

#[test]
fn evaluates_one_instance_against_single_and_multi_document_closures() {
    let conformant = input(
        json!({"$schema": DIALECT, "type": "string", "minLength": 2}),
        vec![],
        json!("ok"),
    );
    let fact = evaluate_json_schema_v1(&conformant).unwrap();
    assert_eq!(fact.conformance(), ConformanceV1::Conformant);
    assert!(fact.violations().is_empty());

    let nonconformant = input(
        json!({
            "$schema": DIALECT,
            "$ref": "shared.schema.json#/$defs/name"
        }),
        vec![(
            SHARED,
            json!({
                "$schema": DIALECT,
                "$defs": {"name": {"type": "string", "minLength": 3}}
            }),
        )],
        json!("x"),
    );
    let fact = evaluate_json_schema_v1(&nonconformant).unwrap();
    assert_eq!(fact.conformance(), ConformanceV1::Nonconformant);
    assert_eq!(fact.violations().len(), 1);
    assert_eq!(fact.violations()[0].code(), ViolationCodeV1::MinLength);
    assert_eq!(fact.violations()[0].schema_document().as_str(), SHARED);
    assert_eq!(
        fact.violations()[0].schema_pointer(),
        "/$defs/name/minLength"
    );
}

#[test]
fn refuses_unsupported_dialect_keyword_identifier_and_remote_or_missing_refs() {
    for schema in [
        json!({"$schema": "http://json-schema.org/draft-07/schema#", "type": "string"}),
        json!({"$schema": DIALECT, "unknownKeyword": true}),
        json!({"$schema": DIALECT, "$id": "https://example.com/root"}),
        json!({"$schema": DIALECT, "type": 7}),
    ] {
        assert_eq!(
            evaluate_json_schema_v1(&input(schema, vec![], json!(null))),
            Err(ValidatorRefusalV1::SchemaUnsupported)
        );
    }

    for reference in [
        "https://example.com/schema.json",
        "file:///tmp/schema.json",
        "//example.com/schema.json",
        "/absolute/schema.json",
        "..\\shared\\schema.json",
        "shared.json?version=1",
        "shared%2Eschema.json",
        "../shared/schema.json",
        "#plain-name",
    ] {
        let schema = json!({"$schema": DIALECT, "$ref": reference});
        assert_eq!(
            evaluate_json_schema_v1(&input(schema, vec![], json!(null))),
            Err(ValidatorRefusalV1::RefRemote),
            "{reference}"
        );
    }

    let missing = input(
        json!({"$schema": DIALECT, "$ref": "missing.schema.json#/type"}),
        vec![],
        json!(null),
    );
    assert_eq!(
        evaluate_json_schema_v1(&missing),
        Err(ValidatorRefusalV1::RefMissing)
    );

    for keyword in [
        "$anchor",
        "$dynamicAnchor",
        "$recursiveAnchor",
        "$recursiveRef",
        "$dynamicRef",
    ] {
        let mut schema = json!({"$schema": DIALECT});
        schema
            .as_object_mut()
            .unwrap()
            .insert(keyword.to_owned(), json!("value"));
        assert_eq!(
            evaluate_json_schema_v1(&input(schema, vec![], json!(null))),
            Err(ValidatorRefusalV1::SchemaUnsupported),
            "{keyword}"
        );
    }
    for schema in [
        json!(true),
        json!({"type": "string"}),
        json!({"$schema": DIALECT, "properties": {"x": {"$schema": DIALECT}}}),
        json!({"$schema": DIALECT, "format": "email"}),
        json!({"$schema": DIALECT, "contentEncoding": "base64"}),
        json!({"$schema": DIALECT, "contentMediaType": "application/json"}),
    ] {
        assert_eq!(
            evaluate_json_schema_v1(&input(schema, vec![], json!(null))),
            Err(ValidatorRefusalV1::SchemaUnsupported)
        );
    }
}

#[test]
fn rfc6901_pointer_escapes_and_utf8_internal_resource_uris_round_trip() {
    let shared = "definitions/schemas/example/root/shared-λ.schema.json";
    let validation = input(
        json!({"$schema": DIALECT, "$ref": "shared-λ.schema.json#/$defs/a~1b~0c"}),
        vec![(
            shared,
            json!({"$schema": DIALECT, "$defs": {"a/b~c": {"type": "string"}}}),
        )],
        json!(1),
    );
    let fact = evaluate_json_schema_v1(&validation).unwrap();
    assert_eq!(fact.violations().len(), 1);
    assert_eq!(fact.violations()[0].schema_document().as_str(), shared);
    assert_eq!(fact.violations()[0].schema_pointer(), "/$defs/a~1b~0c/type");
}

#[test]
fn maps_every_reachable_typed_library_error_to_the_closed_vocabulary() {
    let cases = [
        (
            json!({"$schema": DIALECT, "type": "object", "properties": {"known": {}}, "additionalProperties": false}),
            json!({"known": true, "extra": true}),
            ViolationCodeV1::AdditionalProperties,
        ),
        (
            json!({"$schema": DIALECT, "anyOf": [{"type": "string"}, {"type": "number"}]}),
            json!(false),
            ViolationCodeV1::AnyOf,
        ),
        (
            json!({"$schema": DIALECT, "const": 1}),
            json!(2),
            ViolationCodeV1::Const,
        ),
        (
            json!({"$schema": DIALECT, "contains": {"type": "string"}}),
            json!([1]),
            ViolationCodeV1::Contains,
        ),
        (
            json!({"$schema": DIALECT, "enum": [1, 2]}),
            json!(3),
            ViolationCodeV1::Enum,
        ),
        (
            json!({"$schema": DIALECT, "exclusiveMaximum": 2}),
            json!(2),
            ViolationCodeV1::ExclusiveMaximum,
        ),
        (
            json!({"$schema": DIALECT, "exclusiveMinimum": 2}),
            json!(2),
            ViolationCodeV1::ExclusiveMinimum,
        ),
        (
            json!({"$schema": DIALECT, "properties": {"x": false}}),
            json!({"x": 1}),
            ViolationCodeV1::FalseSchema,
        ),
        (
            json!({"$schema": DIALECT, "maxItems": 1}),
            json!([1, 2]),
            ViolationCodeV1::MaxItems,
        ),
        (
            json!({"$schema": DIALECT, "maximum": 1}),
            json!(2),
            ViolationCodeV1::Maximum,
        ),
        (
            json!({"$schema": DIALECT, "maxLength": 1}),
            json!("ab"),
            ViolationCodeV1::MaxLength,
        ),
        (
            json!({"$schema": DIALECT, "maxProperties": 1}),
            json!({"a": 1, "b": 2}),
            ViolationCodeV1::MaxProperties,
        ),
        (
            json!({"$schema": DIALECT, "minItems": 1}),
            json!([]),
            ViolationCodeV1::MinItems,
        ),
        (
            json!({"$schema": DIALECT, "minimum": 2}),
            json!(1),
            ViolationCodeV1::Minimum,
        ),
        (
            json!({"$schema": DIALECT, "minLength": 2}),
            json!("a"),
            ViolationCodeV1::MinLength,
        ),
        (
            json!({"$schema": DIALECT, "minProperties": 1}),
            json!({}),
            ViolationCodeV1::MinProperties,
        ),
        (
            json!({"$schema": DIALECT, "multipleOf": 2}),
            json!(3),
            ViolationCodeV1::MultipleOf,
        ),
        (
            json!({"$schema": DIALECT, "not": {"type": "string"}}),
            json!("x"),
            ViolationCodeV1::Not,
        ),
        (
            json!({"$schema": DIALECT, "oneOf": [{"type": "number"}, {"minimum": 0}]}),
            json!(1),
            ViolationCodeV1::OneOfMultipleValid,
        ),
        (
            json!({"$schema": DIALECT, "oneOf": [{"type": "string"}, {"type": "number"}]}),
            json!(false),
            ViolationCodeV1::OneOfNotValid,
        ),
        (
            json!({"$schema": DIALECT, "pattern": "^a+$"}),
            json!("b"),
            ViolationCodeV1::Pattern,
        ),
        (
            json!({"$schema": DIALECT, "propertyNames": {"pattern": "^[a-z]+$"}}),
            json!({"UPPER": 1}),
            ViolationCodeV1::PropertyNames,
        ),
        (
            json!({"$schema": DIALECT, "required": ["x"]}),
            json!({}),
            ViolationCodeV1::Required,
        ),
        (
            json!({"$schema": DIALECT, "type": "string"}),
            json!(1),
            ViolationCodeV1::Type,
        ),
        (
            json!({"$schema": DIALECT, "prefixItems": [{}], "unevaluatedItems": false}),
            json!([1, 2]),
            ViolationCodeV1::UnevaluatedItems,
        ),
        (
            json!({"$schema": DIALECT, "properties": {"x": {}}, "unevaluatedProperties": false}),
            json!({"x": 1, "y": 2}),
            ViolationCodeV1::UnevaluatedProperties,
        ),
        (
            json!({"$schema": DIALECT, "uniqueItems": true}),
            json!([1, 1]),
            ViolationCodeV1::UniqueItems,
        ),
    ];
    for (schema, instance, expected) in cases {
        assert_eq!(violation_code(schema, instance), expected);
    }
}

#[test]
fn exact_date_time_and_fancy_regex_branches_are_selected_only_by_root_path() {
    let date_time_root =
        "definitions/schemas/handbook.schemas.lifecycle.trigger-evidence/1.0.0.schema.json";
    let invalid_date = input_at(
        date_time_root,
        json!({"$schema": DIALECT, "type": "string", "format": "date-time"}),
        vec![],
        json!("not-a-date"),
    );
    assert_eq!(
        evaluate_json_schema_v1(&invalid_date).unwrap().violations()[0].code(),
        ViolationCodeV1::Format
    );
    assert_eq!(
        evaluate_json_schema_v1(&input(
            json!({"$schema": DIALECT, "type": "string", "format": "date-time"}),
            vec![],
            json!("not-a-date")
        )),
        Err(ValidatorRefusalV1::SchemaUnsupported)
    );

    let fancy_root =
        "definitions/schemas/handbook.schemas.artifacts.project-authority/1.1.0.schema.json";
    let fancy = input_at(
        fancy_root,
        json!({"$schema": DIALECT, "type": "string", "pattern": "^(?!bad).+$"}),
        vec![],
        json!("bad"),
    );
    assert_eq!(
        evaluate_json_schema_v1(&fancy).unwrap().violations()[0].code(),
        ViolationCodeV1::Pattern
    );
    let regular = input(
        json!({"$schema": DIALECT, "type": "string", "pattern": "^(?!bad).+$"}),
        vec![],
        json!("bad"),
    );
    assert_eq!(
        evaluate_json_schema_v1(&regular),
        Err(ValidatorRefusalV1::ObservationUnavailable)
    );
}

#[test]
fn refs_must_target_admitted_schema_positions_and_be_acyclic_and_at_most_32_edges() {
    for reference in ["#/missing", "#/$schema"] {
        assert_eq!(
            evaluate_json_schema_v1(&input(
                json!({"$schema": DIALECT, "$ref": reference}),
                vec![],
                json!(null)
            )),
            Err(ValidatorRefusalV1::RefMissing),
            "{reference}"
        );
    }
    assert_eq!(
        evaluate_json_schema_v1(&input(
            json!({"$schema": DIALECT, "$ref": "#"}),
            vec![],
            json!(null)
        )),
        Err(ValidatorRefusalV1::RefMissing)
    );

    fn chain(edges: usize) -> JsonSchemaValidationInputV1 {
        let root_path = "definitions/schemas/depth/000.schema.json";
        let root = json!({"$schema": DIALECT, "$ref": "001.schema.json"});
        let mut additional = Vec::new();
        for index in 1..=edges {
            let path = format!("definitions/schemas/depth/{index:03}.schema.json");
            let schema = if index == edges {
                json!({"$schema": DIALECT, "type": "null"})
            } else {
                json!({"$schema": DIALECT, "$ref": format!("{:03}.schema.json", index + 1)})
            };
            additional.push((path, schema));
        }
        let borrowed = additional
            .iter()
            .map(|(path, schema)| (path.as_str(), schema.clone()))
            .collect();
        input_at(root_path, root, borrowed, json!(null))
    }
    assert_eq!(
        evaluate_json_schema_v1(&chain(32)).unwrap().conformance(),
        ConformanceV1::Conformant
    );
    assert_eq!(
        evaluate_json_schema_v1(&chain(33)),
        Err(ValidatorRefusalV1::LimitExceeded)
    );
}

#[test]
fn errors_are_sorted_and_exact_duplicate_tuples_are_coalesced() {
    let schema = json!({
        "$schema": DIALECT,
        "$defs": {"string": {"type": "string"}},
        "allOf": [
            {"$ref": "#/$defs/string"},
            {"$ref": "#/$defs/string"}
        ],
        "required": ["missing"],
        "type": "object"
    });
    let fact = evaluate_json_schema_v1(&input(schema, vec![], json!(1))).unwrap();
    assert_eq!(fact.conformance(), ConformanceV1::Nonconformant);
    assert_eq!(fact.violations().len(), 2);
    assert_eq!(fact.violations()[0].code(), ViolationCodeV1::Type);
    assert_eq!(fact.violations()[0].schema_pointer(), "/$defs/string/type");
    assert_eq!(fact.violations()[1].code(), ViolationCodeV1::Type);
    assert_eq!(fact.violations()[1].schema_pointer(), "/type");
}

#[test]
fn document_pointer_violation_and_payload_bounds_fail_without_partial_fact() {
    let mut extra_documents = Vec::new();
    for index in 1..128 {
        extra_documents.push((
            format!("definitions/schemas/limit/{index:03}.schema.json"),
            json!({"$schema": DIALECT}),
        ));
    }
    let borrowed = extra_documents
        .iter()
        .map(|(path, schema)| (path.as_str(), schema.clone()))
        .collect();
    assert_eq!(
        evaluate_json_schema_v1(&input_at(
            "definitions/schemas/limit/000.schema.json",
            json!({"$schema": DIALECT, "type": "null"}),
            borrowed,
            json!(null)
        ))
        .unwrap()
        .closure_document_count(),
        128
    );

    let long_name = "x".repeat(1_025);
    assert_eq!(
        evaluate_json_schema_v1(&input(
            json!({"$schema": DIALECT, "properties": {long_name.clone(): {"type": "string"}}}),
            vec![],
            json!({long_name: 1})
        )),
        Err(ValidatorRefusalV1::LimitExceeded)
    );

    let mut properties = serde_json::Map::new();
    let mut instance = serde_json::Map::new();
    for index in 0..128 {
        let name = format!("p{index:03}");
        properties.insert(name.clone(), json!({"type": "string"}));
        instance.insert(name, json!(1));
    }
    assert_eq!(
        evaluate_json_schema_v1(&input(
            json!({"$schema": DIALECT, "properties": properties.clone()}),
            vec![],
            Value::Object(instance.clone())
        ))
        .unwrap()
        .violations()
        .len(),
        128
    );
    properties.clear();
    instance.clear();
    for index in 0..129 {
        let name = format!("p{index:03}");
        properties.insert(name.clone(), json!({"type": "string"}));
        instance.insert(name, json!(1));
    }
    assert_eq!(
        evaluate_json_schema_v1(&input(
            json!({"$schema": DIALECT, "properties": properties}),
            vec![],
            Value::Object(instance)
        )),
        Err(ValidatorRefusalV1::LimitExceeded)
    );

    let long_path = format!("definitions/schemas/{}/root.schema.json", "d".repeat(966));
    let mut properties = serde_json::Map::new();
    let mut instance = serde_json::Map::new();
    for index in 0..128 {
        let name = format!("{}-{index:03}", "n".repeat(900));
        properties.insert(name.clone(), json!({"type": "string"}));
        instance.insert(name, json!(1));
    }
    assert_eq!(
        evaluate_json_schema_v1(&input_at(
            &long_path,
            json!({"$schema": DIALECT, "properties": properties}),
            vec![],
            Value::Object(instance)
        )),
        Err(ValidatorRefusalV1::LimitExceeded)
    );
}

#[test]
fn normal_source_and_dependency_surface_has_no_ambient_or_product_back_edge() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = std::fs::read_to_string(crate_root.join("Cargo.toml")).unwrap();
    assert!(manifest.contains("jsonschema = { version = \"=0.47.0\", default-features = false }"));
    let dependencies = manifest.split("[dependencies]").nth(1).unwrap();
    for forbidden in [
        "handbook-contracts",
        "handbook-dock =",
        "handbook-engine",
        "handbook-flow",
        "handbook-pipeline",
        "handbook-sdk",
        "handbook-tauri",
        "python",
    ] {
        assert!(!dependencies.contains(forbidden), "{forbidden}");
    }

    let source = std::fs::read_dir(crate_root.join("src"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
        .map(|path| std::fs::read_to_string(path).unwrap())
        .collect::<String>();
    for forbidden in [
        "std::process",
        "std::fs",
        "std::net",
        "std::env",
        "Command::new",
        "target_os",
        "cfg!(windows)",
        "cfg(windows)",
        "reqwest",
        "with_retriever",
        ".retriever(",
        "libloading",
        "python",
    ] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
}

#[test]
fn fingerprint_substitution_is_rejected_before_evaluation() {
    let admitted = input(
        json!({"$schema": DIALECT, "type": "string"}),
        vec![],
        json!("ok"),
    );
    for mutate in [
        "root_schema",
        "root_document",
        "closure",
        "document",
        "instance",
    ] {
        let mut value = serde_json::to_value(&admitted).unwrap();
        match mutate {
            "root_schema" => {
                value["root_schema"]["fingerprint"] =
                    json!("sha256:0000000000000000000000000000000000000000000000000000000000000000")
            }
            "root_document" => {
                value["root_document_fingerprint"] =
                    json!("sha256:0000000000000000000000000000000000000000000000000000000000000000")
            }
            "closure" => {
                value["closure_fingerprint"] =
                    json!("sha256:0000000000000000000000000000000000000000000000000000000000000000")
            }
            "document" => value["documents"][0]["schema"]["type"] = json!("number"),
            "instance" => value["instance"]["value"] = json!("changed"),
            _ => unreachable!(),
        }
        assert!(serde_json::from_value::<JsonSchemaValidationInputV1>(value).is_err());
    }
}
