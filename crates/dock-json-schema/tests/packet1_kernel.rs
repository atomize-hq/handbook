use handbook_dock_json_schema::{
    CanonicalFingerprintV1, ConformanceV1, ExactSchemaBindingV1, JsonInstanceV1,
    JsonSchemaValidationInputV1, LogicalSchemaPathV1, NormalizedViolationV1, SchemaClosureMemberV1,
    SchemaConformanceFactV1, ValidatorDiagnosticCodeV1, ValidatorLimitsV1,
    ValidatorRefusalReasonV1, ValidatorRefusalV1, ViolationCodeV1,
    SCHEMA_CONFORMANCE_FACT_SCHEMA_REF_V1, VALIDATION_INPUT_SCHEMA_REF_V1,
};
use serde::ser::Error as _;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};

const ROOT_PATH: &str = "definitions/schemas/example/root/1.0.0.schema.json";
const SHARED_PATH: &str = "definitions/schemas/example/shared/1.0.0.schema.json";
const ROOT_FINGERPRINT: &str =
    "sha256:21f065b99a46c1d795525e2f941ed4b7fe8f69e6593bc79ef81182897d4faa57";
const SHARED_FINGERPRINT: &str =
    "sha256:7d3fd4b2dbd30bb65f4b01ae77d7d6055fa9cd3ae7c98da0fe28620de2fa522b";
const INSTANCE_FINGERPRINT: &str =
    "sha256:5479921ced5eefedf4712d07ce894893dd28d04df84171944608aed2d6735641";
const CLOSURE_FINGERPRINT: &str =
    "sha256:ae82da6c3732e9f5416dbd70192309734483b7836dbc73dba056ba0ef18b2ef6";
const FACT_FINGERPRINT: &str =
    "sha256:0d74b5f4f3fb8739b0e9edd96ac97141036438ead81b513a8e5c6e9475afe60a";

struct FailingSerialize;

impl Serialize for FailingSerialize {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Err(S::Error::custom("deliberate test serializer failure"))
    }
}

fn root_schema() -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "properties": {"name": {"type": "string"}},
        "required": ["name"],
        "type": "object"
    })
}

fn shared_schema() -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$defs": {"name": {"type": "string"}},
        "type": "object"
    })
}

fn canonical_input() -> JsonSchemaValidationInputV1 {
    let root = SchemaClosureMemberV1::new(
        LogicalSchemaPathV1::parse(ROOT_PATH).unwrap(),
        root_schema(),
    )
    .unwrap();
    let shared = SchemaClosureMemberV1::new(
        LogicalSchemaPathV1::parse(SHARED_PATH).unwrap(),
        shared_schema(),
    )
    .unwrap();
    let root_binding = ExactSchemaBindingV1::parse(
        "handbook.schema.example@1.0.0",
        root.document_fingerprint().clone(),
    )
    .unwrap();
    JsonSchemaValidationInputV1::new(
        root_binding,
        LogicalSchemaPathV1::parse(ROOT_PATH).unwrap(),
        vec![root, shared],
        JsonInstanceV1::new(json!({"scores": [1, 2, 3], "name": "Ada"})).unwrap(),
    )
    .unwrap()
}

#[test]
fn exact_identities_limits_and_refusal_mapping_are_closed() {
    assert_eq!(
        VALIDATION_INPUT_SCHEMA_REF_V1,
        "handbook.dock-json-schema-validation-input@1.0.0"
    );
    assert_eq!(
        SCHEMA_CONFORMANCE_FACT_SCHEMA_REF_V1,
        "handbook.fact.schema-conformance@1.0.0"
    );

    let limits = ValidatorLimitsV1::v1();
    assert_eq!(limits.max_documents(), 128);
    assert_eq!(limits.max_path_bytes(), 1_024);
    assert_eq!(limits.max_path_components(), 64);
    assert_eq!(limits.max_ref_depth(), 32);
    assert_eq!(limits.max_violations(), 128);
    assert_eq!(limits.max_pointer_bytes(), 1_024);

    let cases = [
        (
            ValidatorRefusalV1::InputMalformed,
            ValidatorRefusalReasonV1::InputRefused,
            ValidatorDiagnosticCodeV1::InputMalformed,
        ),
        (
            ValidatorRefusalV1::InputSubstituted,
            ValidatorRefusalReasonV1::InputRefused,
            ValidatorDiagnosticCodeV1::InputSubstituted,
        ),
        (
            ValidatorRefusalV1::SchemaUnsupported,
            ValidatorRefusalReasonV1::UnsupportedInput,
            ValidatorDiagnosticCodeV1::SchemaUnsupported,
        ),
        (
            ValidatorRefusalV1::RefRemote,
            ValidatorRefusalReasonV1::UnsupportedInput,
            ValidatorDiagnosticCodeV1::RefRemote,
        ),
        (
            ValidatorRefusalV1::RefMissing,
            ValidatorRefusalReasonV1::UnsupportedInput,
            ValidatorDiagnosticCodeV1::RefMissing,
        ),
        (
            ValidatorRefusalV1::LimitExceeded,
            ValidatorRefusalReasonV1::LimitRefused,
            ValidatorDiagnosticCodeV1::LimitExceeded,
        ),
        (
            ValidatorRefusalV1::ObservationUnavailable,
            ValidatorRefusalReasonV1::ObservationUnavailable,
            ValidatorDiagnosticCodeV1::ObservationUnavailable,
        ),
        (
            ValidatorRefusalV1::InternalRefusal,
            ValidatorRefusalReasonV1::InternalRefusal,
            ValidatorDiagnosticCodeV1::InternalRefusal,
        ),
    ];
    for (refusal, reason, diagnostic) in cases {
        assert_eq!(refusal.reason(), reason);
        assert_eq!(refusal.diagnostic_code(), diagnostic);
    }
    assert_eq!(
        CanonicalFingerprintV1::for_value(&FailingSerialize),
        Err(ValidatorRefusalV1::InternalRefusal)
    );
}

#[test]
fn rfc8785_sha256_vectors_close_document_closure_instance_and_fact() {
    let input = canonical_input();
    assert_eq!(
        input.documents()[0].document_fingerprint().as_str(),
        ROOT_FINGERPRINT
    );
    assert_eq!(
        input.documents()[1].document_fingerprint().as_str(),
        SHARED_FINGERPRINT
    );
    assert_eq!(input.closure_fingerprint().as_str(), CLOSURE_FINGERPRINT);
    assert_eq!(
        input.instance().instance_fingerprint().as_str(),
        INSTANCE_FINGERPRINT
    );
    input.verify_fingerprints().unwrap();

    let fact =
        SchemaConformanceFactV1::from_validation_input(&input, ConformanceV1::Conformant, vec![])
            .unwrap();
    assert_eq!(
        fact.canonical_fingerprint().unwrap().as_str(),
        FACT_FINGERPRINT
    );
    fact.verify_against_input(&input).unwrap();
    fact.verify_canonical_fingerprint(&CanonicalFingerprintV1::parse(FACT_FINGERPRINT).unwrap())
        .unwrap();
}

#[test]
fn paths_document_order_duplicates_counts_and_root_match_fail_closed() {
    for invalid in [
        "",
        "definitions/schemas",
        "/definitions/schemas/a.json",
        "definitions/schemas/a.json/",
        "definitions//schemas/a.json",
        "definitions/schemas/./a.json",
        "definitions/schemas/../a.json",
        "definitions\\schemas\\a.json",
        "C:/definitions/schemas/a.json",
        "other/root.schema.json",
        " definitions/schemas/a.json",
        "definitions/schemas/a.json ",
    ] {
        assert_eq!(
            LogicalSchemaPathV1::parse(invalid),
            Err(ValidatorRefusalV1::InputMalformed),
            "{invalid:?}"
        );
    }
    let overlong = format!("definitions/schemas/{}", "a".repeat(1_006));
    assert_eq!(
        LogicalSchemaPathV1::parse(&overlong),
        Err(ValidatorRefusalV1::LimitExceeded)
    );
    let too_deep = format!("definitions/schemas/{}/x.json", vec!["a"; 62].join("/"));
    assert_eq!(
        LogicalSchemaPathV1::parse(&too_deep),
        Err(ValidatorRefusalV1::LimitExceeded)
    );

    let input = canonical_input();
    let root = input.documents()[0].clone();
    let shared = input.documents()[1].clone();
    let binding = input.root_schema().clone();
    let instance = input.instance().clone();
    assert_eq!(
        JsonSchemaValidationInputV1::new(
            binding.clone(),
            input.root_document().clone(),
            vec![],
            instance.clone(),
        ),
        Err(ValidatorRefusalV1::LimitExceeded)
    );
    assert_eq!(
        JsonSchemaValidationInputV1::new(
            binding.clone(),
            input.root_document().clone(),
            vec![shared.clone(), root.clone()],
            instance.clone(),
        ),
        Err(ValidatorRefusalV1::InputMalformed)
    );
    assert_eq!(
        JsonSchemaValidationInputV1::new(
            binding.clone(),
            input.root_document().clone(),
            vec![root.clone(), root.clone()],
            instance.clone(),
        ),
        Err(ValidatorRefusalV1::InputMalformed)
    );
    assert_eq!(
        JsonSchemaValidationInputV1::new(
            binding.clone(),
            LogicalSchemaPathV1::parse("definitions/schemas/example/missing.json").unwrap(),
            vec![root.clone(), shared.clone()],
            instance.clone(),
        ),
        Err(ValidatorRefusalV1::InputMalformed)
    );

    let mut documents = Vec::new();
    for index in 0..129 {
        documents.push(
            SchemaClosureMemberV1::new(
                LogicalSchemaPathV1::parse(&format!(
                    "definitions/schemas/example/{index:03}.schema.json"
                ))
                .unwrap(),
                json!({"$schema": "https://json-schema.org/draft/2020-12/schema"}),
            )
            .unwrap(),
        );
    }
    assert_eq!(
        JsonSchemaValidationInputV1::new(
            ExactSchemaBindingV1::parse(
                "handbook.schema.example@1.0.0",
                documents[0].document_fingerprint().clone(),
            )
            .unwrap(),
            documents[0].path().clone(),
            documents,
            instance,
        ),
        Err(ValidatorRefusalV1::LimitExceeded)
    );
}

#[test]
fn derived_fingerprints_unknown_fields_and_closed_enums_refuse() {
    let input = canonical_input();
    let mut value = serde_json::to_value(&input).unwrap();
    value["documents"][0]["schema"]["type"] = json!("array");
    assert!(serde_json::from_value::<JsonSchemaValidationInputV1>(value).is_err());

    let mut value = serde_json::to_value(&input).unwrap();
    value["closure_fingerprint"] = json!(ROOT_FINGERPRINT);
    assert!(serde_json::from_value::<JsonSchemaValidationInputV1>(value).is_err());

    let mut value = serde_json::to_value(&input).unwrap();
    value["instance"]["value"]["name"] = json!("Grace");
    assert!(serde_json::from_value::<JsonSchemaValidationInputV1>(value).is_err());

    let mut value = serde_json::to_value(&input).unwrap();
    value["unexpected"] = json!(true);
    assert!(serde_json::from_value::<JsonSchemaValidationInputV1>(value).is_err());

    let mut value = serde_json::to_value(&input).unwrap();
    value["root_schema"]["ref"] = json!("../not-an-exact-schema-ref");
    assert!(serde_json::from_value::<JsonSchemaValidationInputV1>(value).is_err());

    let fact =
        SchemaConformanceFactV1::from_validation_input(&input, ConformanceV1::Conformant, vec![])
            .unwrap();
    let mut value = serde_json::to_value(&fact).unwrap();
    value["conformance"] = json!("unknown");
    assert!(serde_json::from_value::<SchemaConformanceFactV1>(value).is_err());
}

#[test]
fn conformance_violation_pointer_and_deterministic_order_invariants_hold() {
    let input = canonical_input();
    let required = NormalizedViolationV1::new(
        ViolationCodeV1::Required,
        "/profile",
        LogicalSchemaPathV1::parse(ROOT_PATH).unwrap(),
        "/required",
    )
    .unwrap();
    let type_error = NormalizedViolationV1::new(
        ViolationCodeV1::Type,
        "/name",
        LogicalSchemaPathV1::parse(ROOT_PATH).unwrap(),
        "/properties/name/type",
    )
    .unwrap();

    assert_eq!(
        SchemaConformanceFactV1::from_validation_input(
            &input,
            ConformanceV1::Conformant,
            vec![required.clone()],
        ),
        Err(ValidatorRefusalV1::InputMalformed)
    );
    assert_eq!(
        SchemaConformanceFactV1::from_validation_input(
            &input,
            ConformanceV1::Nonconformant,
            vec![],
        ),
        Err(ValidatorRefusalV1::InputMalformed)
    );

    let fact = SchemaConformanceFactV1::from_validation_input(
        &input,
        ConformanceV1::Nonconformant,
        vec![required.clone(), type_error.clone(), required.clone()],
    )
    .unwrap();
    assert_eq!(fact.violations(), &[type_error, required]);

    NormalizedViolationV1::new(
        ViolationCodeV1::Type,
        "/line\nbreak",
        LogicalSchemaPathV1::parse(ROOT_PATH).unwrap(),
        "/properties/control\u{7f}",
    )
    .unwrap();

    assert_eq!(
        NormalizedViolationV1::new(
            ViolationCodeV1::Type,
            &format!("/{}", "a".repeat(1_024)),
            LogicalSchemaPathV1::parse(ROOT_PATH).unwrap(),
            "",
        ),
        Err(ValidatorRefusalV1::LimitExceeded)
    );
    assert_eq!(
        SchemaConformanceFactV1::from_validation_input(
            &input,
            ConformanceV1::Nonconformant,
            vec![NormalizedViolationV1::new(
                ViolationCodeV1::Type,
                "",
                LogicalSchemaPathV1::parse(
                    "definitions/schemas/example/not-in-closure.schema.json"
                )
                .unwrap(),
                "",
            )
            .unwrap()],
        ),
        Err(ValidatorRefusalV1::InputMalformed)
    );

    let mut too_many = Vec::new();
    for index in 0..129 {
        too_many.push(
            NormalizedViolationV1::new(
                ViolationCodeV1::Type,
                &format!("/{index:03}"),
                LogicalSchemaPathV1::parse(ROOT_PATH).unwrap(),
                "/type",
            )
            .unwrap(),
        );
    }
    assert_eq!(
        SchemaConformanceFactV1::from_validation_input(
            &input,
            ConformanceV1::Nonconformant,
            too_many,
        ),
        Err(ValidatorRefusalV1::LimitExceeded)
    );
}

#[test]
fn crate_has_no_product_back_edge_python_process_filesystem_network_or_os_branch() {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = std::fs::read_to_string(crate_root.join("Cargo.toml")).unwrap();
    let dependencies = manifest
        .split("[dependencies]")
        .nth(1)
        .unwrap()
        .split("[dev-dependencies]")
        .next()
        .unwrap();
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
        "target_os",
        "cfg!(windows)",
        "cfg(windows)",
        "python",
        "Command::new",
        "libloading",
    ] {
        assert!(!source.contains(forbidden), "{forbidden}");
    }
}
