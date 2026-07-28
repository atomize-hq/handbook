use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use handbook_engine::{parse_definition_yaml, DefinitionFingerprint, ExactDefinitionRef};
use jsonschema::{PatternOptions, Validator};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const FINGERPRINT_PATTERN: &str = "^sha256:[0-9a-f]{64}$";
const IDENTIFIER_PATTERN: &str = "^[a-z0-9](?:[a-z0-9-]{0,62}[a-z0-9])?$";
const UTC_SECOND_PATTERN: &str = "^[0-9]{4}-(?:0[1-9]|1[0-2])-(?:0[1-9]|[12][0-9]|3[01])T(?:[01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]Z$";
const CONDITION_REF: &str = "handbook.condition.project.managed-operational-surface@1.0.0";
const CONDITION_FINGERPRINT: &str =
    "sha256:2ae25788c7860f3062f30659a7674c2ccd8f56b0f8809f1134003e04dea20b61";
const EVALUATOR_REF: &str = "handbook.condition-evaluator.managed-operational-surface@1.0.0";
const EVALUATOR_FINGERPRINT: &str =
    "sha256:ca19395c8ef27aa353dc5918c43b6f3522ee365e204f25ba66f2a22aa793f105";

const SCHEMAS: [(&str, &[u8]); 8] = [
    (
        "source",
        include_bytes!("../definitions/schemas/handbook.schemas.project-condition-admitted-evidence-source/1.0.0.schema.json"),
    ),
    (
        "evidence",
        include_bytes!("../definitions/schemas/handbook.schemas.project-condition-evidence/1.0.0.schema.json"),
    ),
    (
        "head",
        include_bytes!("../definitions/schemas/handbook.schemas.project-condition-evidence-head/1.0.0.schema.json"),
    ),
    (
        "transaction",
        include_bytes!("../definitions/schemas/handbook.schemas.project-condition-evidence-transaction/1.0.0.schema.json"),
    ),
    (
        "challenge",
        include_bytes!("../definitions/schemas/handbook.schemas.project-condition-evidence-challenge/1.0.0.schema.json"),
    ),
    (
        "assertion",
        include_bytes!("../definitions/schemas/handbook.schemas.project-condition-evidence-assertion/1.0.0.schema.json"),
    ),
    (
        "use_transition",
        include_bytes!("../definitions/schemas/handbook.schemas.project-condition-evidence-use-transition/1.0.0.schema.json"),
    ),
    (
        "closure",
        include_bytes!("../definitions/schemas/handbook.schemas.project-condition-evidence-evaluation-closure/1.0.0.schema.json"),
    ),
];

const EVALUATOR_DEFINITION_BYTES: &[u8] = include_bytes!(
    "../definitions/project-condition-evaluators/handbook.condition-evaluator.managed-operational-surface/1.0.0.yaml"
);

fn fingerprint(digit: char) -> String {
    assert!(digit.is_ascii_hexdigit() && !digit.is_ascii_uppercase());
    format!("sha256:{}", digit.to_string().repeat(64))
}

fn schema(name: &str) -> Value {
    let bytes = SCHEMAS
        .iter()
        .find_map(|(candidate, bytes)| (*candidate == name).then_some(*bytes))
        .unwrap_or_else(|| panic!("unknown schema {name}"));
    serde_json::from_slice(bytes).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn validator(name: &str) -> Validator {
    let schema = schema(name);
    jsonschema::draft202012::meta::validate(&schema)
        .unwrap_or_else(|error| panic!("{name} meta-schema: {error}"));
    jsonschema::draft202012::options()
        .with_pattern_options(PatternOptions::regex())
        .should_validate_formats(true)
        .build(&schema)
        .unwrap_or_else(|error| panic!("{name} validator: {error}"))
}

fn assert_valid(name: &str, document: &Value) {
    assert!(
        validator(name).is_valid(document),
        "{name} rejected valid document: {document}"
    );
}

fn assert_invalid(name: &str, document: &Value, case: &str) {
    assert!(
        !validator(name).is_valid(document),
        "{name} admitted {case}: {document}"
    );
}

fn fingerprint_preimage(value: &Value, include: Option<&[&str]>, exclude: &[&str]) -> String {
    let mut preimage = match include {
        Some(fields) => Value::Object(
            fields
                .iter()
                .map(|field| {
                    (
                        (*field).to_owned(),
                        value
                            .get(*field)
                            .unwrap_or_else(|| panic!("missing preimage field {field}"))
                            .clone(),
                    )
                })
                .collect(),
        ),
        None => value.clone(),
    };
    if let Some(object) = preimage.as_object_mut() {
        for field in exclude {
            object.remove(*field);
        }
    }
    DefinitionFingerprint::from_json_value(&preimage)
        .expect("RFC 8785 fingerprint")
        .to_string()
}

fn seal(value: &mut Value, field: &str, include: Option<&[&str]>, exclude: &[&str]) {
    value[field] = Value::String(fingerprint_preimage(value, include, exclude));
}

fn positive_source() -> Value {
    let mut source = json!({
        "$schema": "handbook.schemas.project-condition-admitted-evidence-source@1.0.0",
        "schema_id": "handbook.project-condition-admitted-evidence-source",
        "schema_version": "1.0",
        "source_id": "primary-operator",
        "source_kind": "operator_responsibility_attestation",
        "condition_ref": CONDITION_REF,
        "repository_identity_fingerprint": fingerprint('1'),
        "observed_at_utc": "2026-07-01T00:00:00Z",
        "valid_until_utc": "2026-07-31T00:00:00Z",
        "claim_kind": "qualifying_responsibility_present",
        "surface_kind": "runtime",
        "surface_ref": "service-api",
        "source_fingerprint": fingerprint('0'),
    });
    seal(
        &mut source,
        "source_fingerprint",
        None,
        &["$schema", "source_fingerprint"],
    );
    source
}

fn producer_verification() -> Value {
    let mut verification = json!({
        "verification_kind": "native_approver_registry_assertion_v1",
        "approval_class": "project_condition_evidence",
        "authority_ref": CONDITION_REF,
        "approver_registry_state_ref": format!("registry-states/registry-state_{}.json", "2".repeat(64)),
        "approver_registry_state_fingerprint": fingerprint('2'),
        "registry_head_transition_ref": format!("registry-transitions/registry-transition_{}.json", "3".repeat(64)),
        "registry_head_transition_fingerprint": fingerprint('3'),
        "credential_id_hash": fingerprint('4'),
        "challenge_ref": format!("project-condition-evidence/challenges/challenge_{}.json", "5".repeat(64)),
        "challenge_fingerprint": fingerprint('5'),
        "assertion_ref": format!("project-condition-evidence/assertions/assertion_{}.json", "6".repeat(64)),
        "assertion_fingerprint": fingerprint('6'),
        "authenticator_use_transition_ref": format!("authenticator-use-transitions/authenticator-use-transition_{}.json", "7".repeat(64)),
        "authenticator_use_transition_fingerprint": fingerprint('7'),
        "result_authenticator_use_head_ref": format!("authenticator-use-heads/{}.json", "8".repeat(64)),
        "result_authenticator_use_head_fingerprint": fingerprint('8'),
        "verified_at_utc": "2026-07-01T00:00:01Z",
        "producer_verification_fingerprint": fingerprint('0'),
    });
    seal(
        &mut verification,
        "producer_verification_fingerprint",
        None,
        &["producer_verification_fingerprint"],
    );
    verification
}

const RECORD_SUBJECT_FIELDS: [&str; 9] = [
    "schema_id",
    "schema_version",
    "condition_ref",
    "condition_definition_fingerprint",
    "repository_identity_fingerprint",
    "evidence_sequence",
    "previous_record_ref",
    "previous_record_fingerprint",
    "inputs",
];

fn evidence() -> Value {
    let mut evidence = json!({
        "$schema": "handbook.schemas.project-condition-evidence@1.0.0",
        "schema_id": "handbook.project-condition-evidence",
        "schema_version": "1.0",
        "condition_ref": CONDITION_REF,
        "condition_definition_fingerprint": CONDITION_FINGERPRINT,
        "repository_identity_fingerprint": fingerprint('1'),
        "evidence_sequence": 1,
        "previous_record_ref": null,
        "previous_record_fingerprint": null,
        "inputs": [{
            "input_id": "source-a",
            "input_class": "admitted_evidence_ref",
            "source_ref": format!("sources/admitted-evidence_{}.json", "9".repeat(64)),
            "source_fingerprint": fingerprint('9'),
        }],
        "producer_verification": producer_verification(),
        "record_subject_fingerprint": fingerprint('0'),
        "record_fingerprint": fingerprint('0'),
    });
    seal(
        &mut evidence,
        "record_subject_fingerprint",
        Some(&RECORD_SUBJECT_FIELDS),
        &[],
    );
    seal(
        &mut evidence,
        "record_fingerprint",
        None,
        &["record_fingerprint"],
    );
    evidence
}

fn head() -> Value {
    let mut head = json!({
        "$schema": "handbook.schemas.project-condition-evidence-head@1.0.0",
        "schema_id": "handbook.project-condition-evidence-head",
        "schema_version": "1.0",
        "condition_ref": CONDITION_REF,
        "repository_identity_fingerprint": fingerprint('1'),
        "evidence_sequence": 1,
        "current_record_ref": format!("records/managed-operational-surface_{}.json", "a".repeat(64)),
        "current_record_fingerprint": fingerprint('a'),
        "credential_id_hash": fingerprint('4'),
        "authenticator_use_head_ref": format!("authenticator-use-heads/{}.json", "8".repeat(64)),
        "authenticator_use_head_fingerprint": fingerprint('8'),
        "previous_evidence_head_fingerprint": null,
        "head_fingerprint": fingerprint('0'),
    });
    seal(
        &mut head,
        "head_fingerprint",
        None,
        &["$schema", "head_fingerprint"],
    );
    head
}

fn transaction() -> Value {
    let mut transaction = json!({
        "$schema": "handbook.schemas.project-condition-evidence-transaction@1.0.0",
        "schema_id": "handbook.project-condition-evidence-transaction",
        "schema_version": "1.0",
        "evidence_sequence": 1,
        "previous_transaction_ref": null,
        "previous_transaction_fingerprint": null,
        "record_ref": format!("records/managed-operational-surface_{}.json", "a".repeat(64)),
        "record_fingerprint": fingerprint('a'),
        "evidence_head_fingerprint": fingerprint('b'),
        "credential_id_hash": fingerprint('4'),
        "authenticator_use_head_ref": format!("authenticator-use-heads/{}.json", "8".repeat(64)),
        "authenticator_use_head_fingerprint": fingerprint('8'),
        "producer_verification_fingerprint": fingerprint('c'),
        "committed_at_utc": "2026-07-01T00:00:01Z",
        "transaction_fingerprint": fingerprint('0'),
    });
    seal(
        &mut transaction,
        "transaction_fingerprint",
        None,
        &["$schema", "transaction_fingerprint"],
    );
    transaction
}

fn challenge() -> Value {
    let mut challenge = json!({
        "$schema": "handbook.schemas.project-condition-evidence-challenge@1.0.0",
        "schema_id": "handbook.project-condition-evidence-challenge",
        "schema_version": "1.0",
        "operation_id": "publish-1",
        "operation": "publish_managed_operational_surface_evidence",
        "repository_identity_fingerprint": fingerprint('1'),
        "condition_ref": CONDITION_REF,
        "condition_definition_fingerprint": CONDITION_FINGERPRINT,
        "approval_class": "project_condition_evidence",
        "authority_ref": CONDITION_REF,
        "evidence_sequence": 1,
        "previous_record_fingerprint": null,
        "previous_evidence_head_fingerprint": null,
        "previous_transaction_fingerprint": null,
        "record_subject_fingerprint": fingerprint('d'),
        "approver_registry_state_fingerprint": fingerprint('2'),
        "registry_head_transition_fingerprint": fingerprint('3'),
        "nonce_base64": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
        "challenge_fingerprint": fingerprint('0'),
    });
    seal(
        &mut challenge,
        "challenge_fingerprint",
        None,
        &["$schema", "challenge_fingerprint"],
    );
    challenge
}

fn assertion() -> Value {
    let mut assertion = json!({
        "schema_id": "handbook.project-condition-evidence-assertion",
        "schema_version": "1.0",
        "assertion_id": "assertion-1",
        "protocol_id": "fido2-ctap2.1-native",
        "challenge_schema_ref": "handbook.schemas.project-condition-evidence-challenge@1.0.0",
        "challenge_jcs_base64": "e30=",
        "challenge_fingerprint": fingerprint('5'),
        "decoded_response_schema_ref": "handbook.schemas.security.authenticator-get-assertion-response@1.0.0",
        "decoded_response_ref": format!("authenticator-get-assertion-responses/authenticator-get-assertion-response_{}.json", "e".repeat(64)),
        "decoded_response_fingerprint": fingerprint('e'),
        "rp_id": "handbook.local",
        "client_data_hash": fingerprint('f'),
        "signed_preimage_sha256": fingerprint('1'),
        "credential_id_hash": fingerprint('4'),
        "algorithm": "ES256",
        "signature_base64": "AA==",
        "authenticator_data_base64": format!("{}==", "A".repeat(50)),
        "flags_byte": 5,
        "attested_credential_data_included": false,
        "extensions_included": false,
        "user_present": true,
        "user_verified": true,
        "sign_count": 1,
        "verified_at_utc": "2026-07-01T00:00:01Z",
        "assertion_fingerprint": fingerprint('0'),
    });
    seal(
        &mut assertion,
        "assertion_fingerprint",
        None,
        &["assertion_fingerprint"],
    );
    assertion
}

fn use_transition() -> Value {
    let mut transition = json!({
        "$schema": "handbook.schemas.project-condition-evidence-use-transition@1.0.0",
        "schema_version": "1.0",
        "consumer_kind": "project_condition_evidence",
        "consumer_id": "publish-1",
        "credential_id_hash": fingerprint('4'),
        "sequence": 1,
        "prior_head_fingerprint": fingerprint('5'),
        "challenge_fingerprint": fingerprint('6'),
        "assertion_ref": format!("project-condition-evidence/assertions/assertion_{}.json", "7".repeat(64)),
        "assertion_fingerprint": fingerprint('7'),
        "nonce_sha256": fingerprint('8'),
        "prior_sign_count": 0,
        "sign_count": 1,
        "result_head_fingerprint": fingerprint('9'),
        "transition_fingerprint": fingerprint('0'),
    });
    seal(
        &mut transition,
        "transition_fingerprint",
        None,
        &["$schema", "transition_fingerprint"],
    );
    transition
}

fn store_observation(observation_kind: &str, error_code: Option<&str>) -> Value {
    let retained = observation_kind == "retained_directory";
    json!({
        "dependency_kind": "evidence_transaction_store",
        "logical_id": "managed-operational-surface-transactions",
        "relative_ref": ".handbook/state/project-condition-evidence/transactions",
        "observation_kind": observation_kind,
        "byte_count": retained.then_some(41),
        "observed_bytes_fingerprint": retained.then(|| fingerprint('a')),
        "required_fingerprint": null,
        "claimed_fingerprint": null,
        "computed_fingerprint": null,
        "error_code": error_code,
    })
}

fn closure() -> Value {
    let mut closure = json!({
        "schema_id": "handbook.project-condition-evidence-evaluation-closure",
        "schema_version": "1.0",
        "evaluator_ref": EVALUATOR_REF,
        "evaluator_definition_fingerprint": EVALUATOR_FINGERPRINT,
        "condition_ref": CONDITION_REF,
        "condition_definition_fingerprint": CONDITION_FINGERPRINT,
        "repository_identity_fingerprint": fingerprint('1'),
        "evaluated_at_utc": "2026-07-01T00:00:02Z",
        "dependency_observations": [
            store_observation("absent", Some("transaction_store_missing"))
        ],
        "evidence_head_fingerprint": null,
        "evidence_record_fingerprint": null,
        "producer_verification_fingerprint": null,
        "freshness_basis": [],
        "ordered_inputs": [],
        "outcome": "unresolved",
        "reason": "evidence_record_missing",
        "closure_fingerprint": fingerprint('0'),
    });
    seal(
        &mut closure,
        "closure_fingerprint",
        None,
        &["closure_fingerprint"],
    );
    closure
}

fn fixtures() -> BTreeMap<&'static str, Value> {
    BTreeMap::from([
        ("assertion", assertion()),
        ("challenge", challenge()),
        ("closure", closure()),
        ("evidence", evidence()),
        ("head", head()),
        ("source", positive_source()),
        ("transaction", transaction()),
        ("use_transition", use_transition()),
    ])
}

#[test]
fn every_schema_is_draft_2020_12_closed_required_and_accepts_its_exact_record() {
    for (name, document) in fixtures() {
        let contract = schema(name);
        assert_eq!(
            contract["$schema"],
            "https://json-schema.org/draft/2020-12/schema"
        );
        assert_eq!(contract["type"], "object");
        assert_eq!(contract["additionalProperties"], false);
        assert_eq!(contract["unevaluatedProperties"], false);
        let properties = contract["properties"]
            .as_object()
            .expect("root properties")
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>();
        let required = contract["required"]
            .as_array()
            .expect("root required")
            .iter()
            .map(|field| field.as_str().expect("required string").to_owned())
            .collect::<BTreeSet<_>>();
        assert_eq!(required, properties, "{name} requires every listed key");

        assert_valid(name, &document);

        for field in &required {
            let mut missing = document.clone();
            missing.as_object_mut().unwrap().remove(field);
            assert_invalid(name, &missing, &format!("missing required {field}"));
        }
        let mut extra = document;
        extra["unexpected"] = json!(true);
        assert_invalid(name, &extra, "unknown root field");
    }
}

#[test]
fn constants_patterns_enums_and_scalar_bounds_are_exact() {
    for (name, mut document) in fixtures() {
        let contract = schema(name);
        for (field, property) in contract["properties"].as_object().unwrap() {
            if property.get("const").is_some() {
                document[field] = json!("wrong");
                assert_invalid(name, &document, &format!("wrong constant {field}"));
                document = fixtures().remove(name).unwrap();
            }
        }
    }

    for (name, sequence_field) in [
        ("evidence", "evidence_sequence"),
        ("head", "evidence_sequence"),
        ("transaction", "evidence_sequence"),
        ("challenge", "evidence_sequence"),
        ("use_transition", "sequence"),
    ] {
        let mut document = fixtures().remove(name).unwrap();
        document[sequence_field] = json!(0);
        assert_invalid(name, &document, "sequence zero");
        document[sequence_field] = json!(4097);
        assert_invalid(name, &document, "sequence 4097");
    }

    let mut assertion = assertion();
    assertion["sign_count"] = json!(0);
    assert_invalid("assertion", &assertion, "zero assertion sign count");
    assertion["sign_count"] = json!(4294967296u64);
    assert_invalid("assertion", &assertion, "overflow assertion sign count");

    let mut transition = use_transition();
    transition["prior_sign_count"] = json!(4294967295u64);
    assert_invalid(
        "use_transition",
        &transition,
        "unadvanceable prior sign count",
    );
    transition = use_transition();
    transition["sign_count"] = json!(4294967296u64);
    assert_invalid("use_transition", &transition, "overflow sign count");

    for invalid in [
        "sha256:abc",
        "SHA256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "sha256:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
    ] {
        let mut source = positive_source();
        source["repository_identity_fingerprint"] = json!(invalid);
        assert_invalid("source", &source, "invalid fingerprint grammar");
    }
    for invalid in ["Upper", "-bad", "bad-", &"a".repeat(65)] {
        let mut source = positive_source();
        source["source_id"] = json!(invalid);
        assert_invalid("source", &source, "invalid identifier grammar");
    }
    for invalid in [
        "2026-07-01T00:00:00.000Z",
        "2026-07-01T00:00:00+00:00",
        "2026-13-01T00:00:00Z",
    ] {
        let mut source = positive_source();
        source["observed_at_utc"] = json!(invalid);
        assert_invalid("source", &source, "invalid UtcSecond");
    }

    assert_eq!(
        schema("source")["$defs"]["fingerprint"]["pattern"],
        FINGERPRINT_PATTERN
    );
    assert_eq!(
        schema("source")["$defs"]["identifier"]["pattern"],
        IDENTIFIER_PATTERN
    );
    assert_eq!(
        schema("source")["$defs"]["utcSecond"]["pattern"],
        UTC_SECOND_PATTERN
    );
}

fn collect_semantic_keywords(
    value: &Value,
    pointer: &str,
    rows: &mut Vec<String>,
    patterns: &mut BTreeSet<String>,
    enums: &mut Vec<Value>,
) {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                let child_pointer =
                    format!("{pointer}/{}", key.replace('~', "~0").replace('/', "~1"));
                if [
                    "const",
                    "enum",
                    "pattern",
                    "minimum",
                    "maximum",
                    "minItems",
                    "maxItems",
                    "minLength",
                    "maxLength",
                ]
                .contains(&key.as_str())
                {
                    let canonical = serde_json_canonicalizer::to_string(child).unwrap();
                    rows.push(format!("{child_pointer}\t{canonical}"));
                    if key == "pattern" {
                        patterns.insert(child.as_str().unwrap().to_owned());
                    } else if key == "enum" {
                        enums.push(child.clone());
                    }
                }
                collect_semantic_keywords(child, &child_pointer, rows, patterns, enums);
            }
        }
        Value::Array(array) => {
            for (index, child) in array.iter().enumerate() {
                collect_semantic_keywords(
                    child,
                    &format!("{pointer}/{index}"),
                    rows,
                    patterns,
                    enums,
                );
            }
        }
        _ => {}
    }
}

#[test]
fn every_pattern_enum_constant_and_bound_is_frozen_and_operational() {
    let mut rows = Vec::new();
    let mut patterns = BTreeSet::new();
    let mut enums = Vec::new();
    for (name, _) in SCHEMAS {
        collect_semantic_keywords(
            &schema(name),
            &format!("/{name}"),
            &mut rows,
            &mut patterns,
            &mut enums,
        );
    }
    rows.sort();
    let bytes = rows.join("\n");
    assert_eq!(
        DefinitionFingerprint::from_bytes(bytes.as_bytes()).as_str(),
        "sha256:6231eb48254d0d921d964560ee6cb8e8e57443de90a8b206c60d711c989a57a5"
    );

    let pattern_cases = BTreeMap::from([
        ("^\\.{1,2}$", (".", "service")),
        ("^[A-Za-z]:", ("C:", "service")),
        (
            "^[^/\\\\\\u0000-\\u001F\\u007F\\s]+$",
            ("service-api", "has space"),
        ),
        (
            "^sha256:[0-9a-f]{64}$",
            (
                "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "sha256:AAAA",
            ),
        ),
        (
            "^[a-z0-9](?:[a-z0-9-]{0,62}[a-z0-9])?$",
            ("service-1", "Service"),
        ),
        (
            "^[0-9]{4}-(?:0[1-9]|1[0-2])-(?:0[1-9]|[12][0-9]|3[01])T(?:[01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]Z$",
            ("2026-07-27T23:59:59Z", "2026-07-27T23:59:59.0Z"),
        ),
        (
            "^records/managed-operational-surface_[0-9a-f]{64}[.]json$",
            (
                "records/managed-operational-surface_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json",
                "records/managed-operational-surface_A.json",
            ),
        ),
        (
            "^sources/admitted-evidence_[0-9a-f]{64}[.]json$",
            (
                "sources/admitted-evidence_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json",
                "sources/admitted-evidence_A.json",
            ),
        ),
        (
            "^registry-states/registry-state_[0-9a-f]{64}[.]json$",
            (
                "registry-states/registry-state_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json",
                "registry-states/registry-state_A.json",
            ),
        ),
        (
            "^project-condition-evidence/assertions/assertion_[0-9a-f]{64}[.]json$",
            (
                "project-condition-evidence/assertions/assertion_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json",
                "project-condition-evidence/assertions/assertion_A.json",
            ),
        ),
        (
            "^authenticator-use-transitions/authenticator-use-transition_[0-9a-f]{64}[.]json$",
            (
                "authenticator-use-transitions/authenticator-use-transition_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json",
                "authenticator-use-transitions/authenticator-use-transition_A.json",
            ),
        ),
        (
            "^project-condition-evidence/challenges/challenge_[0-9a-f]{64}[.]json$",
            (
                "project-condition-evidence/challenges/challenge_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json",
                "project-condition-evidence/challenges/challenge_A.json",
            ),
        ),
        (
            "^registry-transitions/registry-transition_[0-9a-f]{64}[.]json$",
            (
                "registry-transitions/registry-transition_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json",
                "registry-transitions/registry-transition_A.json",
            ),
        ),
        (
            "^authenticator-use-heads/[0-9a-f]{64}[.]json$",
            (
                "authenticator-use-heads/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json",
                "authenticator-use-heads/A.json",
            ),
        ),
        (
            "^transactions/transaction_(?:000[1-9]|00[1-9][0-9]|0[1-9][0-9]{2}|[1-3][0-9]{3}|40(?:[0-8][0-9]|9[0-6]))_[0-9a-f]{64}[.]json$",
            (
                "transactions/transaction_4096_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json",
                "transactions/transaction_4097_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json",
            ),
        ),
        (
            "^[A-Za-z0-9+/]{43}=$",
            ("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=", "AAAA"),
        ),
        (
            "^authenticator-get-assertion-responses/authenticator-get-assertion-response_[0-9a-f]{64}[.]json$",
            (
                "authenticator-get-assertion-responses/authenticator-get-assertion-response_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json",
                "authenticator-get-assertion-responses/authenticator-get-assertion-response_A.json",
            ),
        ),
        (
            "^[A-Za-z0-9+/]{50}==$",
            (
                "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==",
                "AAAA",
            ),
        ),
    ]);
    assert_eq!(
        patterns,
        pattern_cases
            .keys()
            .map(|value| (*value).to_owned())
            .collect()
    );
    for (pattern, (matching, not_matching)) in pattern_cases {
        let contract = json!({"type": "string", "pattern": pattern});
        let validator = Validator::options()
            .with_pattern_options(PatternOptions::regex())
            .build(&contract)
            .unwrap();
        assert!(validator.is_valid(&json!(matching)), "{pattern} match");
        assert!(
            !validator.is_valid(&json!(not_matching)),
            "{pattern} negative"
        );
    }
    for admitted in enums {
        let contract = json!({"enum": admitted});
        let validator = Validator::new(&contract).unwrap();
        for value in contract["enum"].as_array().unwrap() {
            assert!(validator.is_valid(value));
        }
        assert!(!validator.is_valid(&json!("__not_admitted__")));
    }
}

#[test]
fn source_claim_surface_and_semantic_ref_branches_are_closed() {
    let mut source = positive_source();
    for kind in [
        "runtime",
        "deployment",
        "operated_automation",
        "operational_integration",
    ] {
        source["surface_kind"] = json!(kind);
        assert_valid("source", &source);
    }
    source["surface_kind"] = json!("build");
    assert_invalid("source", &source, "unadmitted surface kind");

    for claim in ["no_qualifying_responsibility", "unknown"] {
        let mut non_positive = positive_source();
        non_positive["claim_kind"] = json!(claim);
        non_positive["surface_kind"] = Value::Null;
        non_positive["surface_ref"] = Value::Null;
        assert_valid("source", &non_positive);
        non_positive["surface_ref"] = json!("service-api");
        assert_invalid("source", &non_positive, "non-positive surface ref");
    }

    for invalid in [".", "..", "C:drive", "/absolute", "has space", "a\\b"] {
        let mut invalid_ref = positive_source();
        invalid_ref["surface_ref"] = json!(invalid);
        assert_invalid("source", &invalid_ref, "invalid SemanticRef");
    }
    let semantic = &schema("source")["$defs"]["semanticRef"];
    assert_eq!(semantic["x-handbook-normalization"], "NFC");
    assert_eq!(semantic["x-handbook-max-utf8-bytes"], 256);
}

#[test]
fn sequence_one_and_successor_nullability_are_exact() {
    let cases = [
        (
            "evidence",
            "evidence_sequence",
            vec!["previous_record_ref", "previous_record_fingerprint"],
        ),
        (
            "transaction",
            "evidence_sequence",
            vec![
                "previous_transaction_ref",
                "previous_transaction_fingerprint",
            ],
        ),
        (
            "challenge",
            "evidence_sequence",
            vec![
                "previous_record_fingerprint",
                "previous_evidence_head_fingerprint",
                "previous_transaction_fingerprint",
            ],
        ),
    ];
    for (name, sequence, prior_fields) in cases {
        let mut first = fixtures().remove(name).unwrap();
        first[prior_fields[0]] = json!(fingerprint('a'));
        assert_invalid(name, &first, "non-null sequence-one predecessor");

        let mut successor = fixtures().remove(name).unwrap();
        successor[sequence] = json!(2);
        for field in &prior_fields {
            successor[*field] = if field.ends_with("_ref") {
                json!(format!(
                    "transactions/transaction_0001_{}.json",
                    "a".repeat(64)
                ))
            } else {
                json!(fingerprint('a'))
            };
        }
        if name == "evidence" {
            successor["previous_record_ref"] = json!(format!(
                "records/managed-operational-surface_{}.json",
                "a".repeat(64)
            ));
        }
        assert_valid(name, &successor);
        successor[prior_fields[0]] = Value::Null;
        assert_invalid(name, &successor, "null successor predecessor");
    }

    let mut first_head = head();
    first_head["previous_evidence_head_fingerprint"] = json!(fingerprint('a'));
    assert_invalid("head", &first_head, "sequence-one previous head");
    let mut successor_head = head();
    successor_head["evidence_sequence"] = json!(2);
    successor_head["previous_evidence_head_fingerprint"] = json!(fingerprint('a'));
    assert_valid("head", &successor_head);
    successor_head["previous_evidence_head_fingerprint"] = Value::Null;
    assert_invalid("head", &successor_head, "null successor previous head");

    for invalid_sequence in ["0000", "4097", "9999"] {
        let mut transaction = transaction();
        transaction["evidence_sequence"] = json!(2);
        transaction["previous_transaction_ref"] = json!(format!(
            "transactions/transaction_{invalid_sequence}_{}.json",
            "a".repeat(64)
        ));
        transaction["previous_transaction_fingerprint"] = json!(fingerprint('a'));
        assert_invalid(
            "transaction",
            &transaction,
            &format!("out-of-range transaction ref {invalid_sequence}"),
        );
    }

    let predecessor_sequence = |reference: &str| -> u64 {
        reference.strip_prefix("transactions/transaction_").unwrap()[..4]
            .parse()
            .unwrap()
    };
    for (sequence, prior) in [(2, 1), (4096, 4095)] {
        let reference = format!(
            "transactions/transaction_{prior:04}_{}.json",
            "a".repeat(64)
        );
        assert_eq!(predecessor_sequence(&reference) + 1, sequence);
    }
    let mismatched = format!("transactions/transaction_0002_{}.json", "a".repeat(64));
    assert_ne!(predecessor_sequence(&mismatched) + 1, 2);
}

#[test]
fn input_and_nested_record_counts_keys_refs_and_bounds_are_frozen() {
    let mut record = evidence();
    record["inputs"] = json!([]);
    assert_invalid("evidence", &record, "empty inputs");
    record = evidence();
    record["inputs"] = Value::Array(
        (0..16)
            .map(|index| {
                json!({
                    "input_id": format!("source-{index}"),
                    "input_class": "admitted_evidence_ref",
                    "source_ref": format!("sources/admitted-evidence_{:064x}.json", index + 1),
                    "source_fingerprint": format!("sha256:{:064x}", index + 1),
                })
            })
            .collect(),
    );
    assert_valid("evidence", &record);
    record["inputs"].as_array_mut().unwrap().push(json!({
        "input_id": "overflow",
        "input_class": "admitted_evidence_ref",
        "source_ref": format!("sources/admitted-evidence_{}.json", "f".repeat(64)),
        "source_fingerprint": fingerprint('f'),
    }));
    assert_invalid("evidence", &record, "17 inputs");

    for pointer in ["/inputs/0", "/producer_verification"] {
        let mut nested = evidence();
        nested
            .pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("unexpected".to_owned(), json!(true));
        assert_invalid("evidence", &nested, "unknown nested evidence field");
    }

    let mut closure_with_rows = closure();
    closure_with_rows["freshness_basis"] = json!([freshness_row("current")]);
    closure_with_rows["ordered_inputs"] = json!([resolved_input()]);
    for (name, document, pointer) in [
        ("evidence", evidence(), "/inputs/0"),
        ("evidence", evidence(), "/producer_verification"),
        (
            "closure",
            closure_with_rows.clone(),
            "/dependency_observations/0",
        ),
        ("closure", closure_with_rows.clone(), "/freshness_basis/0"),
        ("closure", closure_with_rows.clone(), "/ordered_inputs/0"),
        ("closure", closure_with_rows, "/ordered_inputs/0/source"),
    ] {
        let required = document
            .pointer(pointer)
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        for field in required {
            let mut missing = document.clone();
            missing
                .pointer_mut(pointer)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .remove(&field);
            assert_invalid(
                name,
                &missing,
                &format!("missing nested required {pointer}/{field}"),
            );
        }
    }

    let mut wrong_ref = evidence();
    wrong_ref["inputs"][0]["source_ref"] = json!("sources/other.json");
    assert_invalid("evidence", &wrong_ref, "source ref grammar");
    wrong_ref = evidence();
    wrong_ref["producer_verification"]["challenge_ref"] =
        json!("project-condition-evidence/challenges/wrong.json");
    assert_invalid("evidence", &wrong_ref, "challenge ref grammar");

    assert_eq!(
        schema("source")["x-handbook-max-canonical-json-lf-bytes"],
        16384
    );
    assert_eq!(
        schema("evidence")["x-handbook-max-canonical-json-lf-bytes"],
        131072
    );
    assert_eq!(
        schema("head")["x-handbook-max-canonical-json-lf-bytes"],
        8192
    );
    assert_eq!(
        schema("transaction")["x-handbook-max-canonical-json-lf-bytes"],
        8192
    );
}

#[test]
fn challenge_assertion_transition_constants_refs_base64_and_counts_are_exact() {
    let mut challenge = challenge();
    challenge["nonce_base64"] = json!("A".repeat(44));
    assert_invalid("challenge", &challenge, "nonce without padding");
    challenge["nonce_base64"] = json!("A".repeat(43) + "=");
    assert_valid("challenge", &challenge);

    let mut assertion_record = assertion();
    assertion_record["authenticator_data_base64"] = json!("AA==");
    assert_invalid("assertion", &assertion_record, "short authenticator data");
    assertion_record = assertion();
    assertion_record["decoded_response_ref"] = json!("wrong.json");
    assert_invalid("assertion", &assertion_record, "decoded response ref");
    assertion_record = assertion();
    assertion_record["flags_byte"] = json!(1);
    assert_invalid("assertion", &assertion_record, "flags");
    assertion_record = assertion();
    assertion_record["user_verified"] = json!(false);
    assert_invalid("assertion", &assertion_record, "user verification");
    assertion_record = assertion();
    assertion_record["challenge_jcs_base64"] = json!("x".repeat(16385));
    assert_invalid("assertion", &assertion_record, "challenge byte bound");
    assertion_record = assertion();
    assertion_record["signature_base64"] = json!("x".repeat(4097));
    assert_invalid("assertion", &assertion_record, "signature bound");

    let mut transition = use_transition();
    transition["assertion_ref"] = json!("wrong.json");
    assert_invalid("use_transition", &transition, "assertion ref");
    transition = use_transition();
    transition["consumer_kind"] = json!("registry_admin");
    assert_invalid("use_transition", &transition, "consumer kind");
}

#[test]
fn every_fingerprint_preimage_and_derived_definition_fingerprint_replays() {
    let mut records = vec![
        (
            positive_source(),
            "source_fingerprint",
            None,
            vec!["$schema", "source_fingerprint"],
        ),
        (
            head(),
            "head_fingerprint",
            None,
            vec!["$schema", "head_fingerprint"],
        ),
        (
            transaction(),
            "transaction_fingerprint",
            None,
            vec!["$schema", "transaction_fingerprint"],
        ),
        (
            challenge(),
            "challenge_fingerprint",
            None,
            vec!["$schema", "challenge_fingerprint"],
        ),
        (
            assertion(),
            "assertion_fingerprint",
            None,
            vec!["assertion_fingerprint"],
        ),
        (
            use_transition(),
            "transition_fingerprint",
            None,
            vec!["$schema", "transition_fingerprint"],
        ),
        (
            closure(),
            "closure_fingerprint",
            None,
            vec!["closure_fingerprint"],
        ),
    ];
    for (record, field, include, exclude) in &mut records {
        let actual = record[*field].as_str().unwrap();
        assert_eq!(
            actual,
            fingerprint_preimage(record, *include, exclude),
            "{field}"
        );
        let before = actual.to_owned();
        record.as_object_mut().unwrap().insert(
            "schema_version".to_owned(),
            Value::String("changed".to_owned()),
        );
        assert_ne!(
            before,
            fingerprint_preimage(record, *include, exclude),
            "{field} must bind semantic mutation"
        );
    }

    let evidence = evidence();
    assert_eq!(
        evidence["record_subject_fingerprint"],
        fingerprint_preimage(&evidence, Some(&RECORD_SUBJECT_FIELDS), &[])
    );
    assert_eq!(
        evidence["record_fingerprint"],
        fingerprint_preimage(&evidence, None, &["record_fingerprint"])
    );
    assert_eq!(
        evidence["producer_verification"]["producer_verification_fingerprint"],
        fingerprint_preimage(
            &evidence["producer_verification"],
            None,
            &["producer_verification_fingerprint"],
        )
    );

    let definition = parse_definition_yaml(EVALUATOR_DEFINITION_BYTES).unwrap();
    let supplied = definition["definition_fingerprint"].as_str().unwrap();
    assert_eq!(supplied, EVALUATOR_FINGERPRINT);
    assert_eq!(
        supplied,
        fingerprint_preimage(&definition, None, &["definition_fingerprint"])
    );
}

fn assert_production_tree_omits_refs(path: &Path, forbidden: &[&str]) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            assert_production_tree_omits_refs(&path, forbidden);
        } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
            let text = fs::read_to_string(&path).unwrap();
            for reference in forbidden {
                assert!(
                    !text.contains(reference),
                    "production source {} unexpectedly registers {reference}",
                    path.display()
                );
            }
        }
    }
}

#[test]
fn evaluator_definition_is_exact_closed_and_unregistered() {
    let definition = parse_definition_yaml(EVALUATOR_DEFINITION_BYTES).unwrap();
    let expected_keys = BTreeSet::from([
        "schema_id",
        "schema_version",
        "evaluator_id",
        "evaluator_version",
        "condition_ref",
        "condition_definition_fingerprint",
        "source_schema_ref",
        "evidence_schema_ref",
        "head_schema_ref",
        "transaction_schema_ref",
        "challenge_schema_ref",
        "assertion_schema_ref",
        "use_transition_schema_ref",
        "closure_schema_ref",
        "admitted_input_classes",
        "maximum_validity_seconds",
        "outcome_precedence",
        "owner_module",
        "transport",
        "extensions",
        "definition_fingerprint",
    ]);
    assert_eq!(
        definition
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        expected_keys
    );
    assert_eq!(
        definition["schema_id"],
        "handbook.project-condition-evaluator-definition"
    );
    assert_eq!(
        ExactDefinitionRef::new(
            definition["evaluator_id"].as_str().unwrap(),
            definition["evaluator_version"].as_str().unwrap(),
        )
        .unwrap()
        .as_str(),
        EVALUATOR_REF
    );
    assert_eq!(definition["condition_ref"], CONDITION_REF);
    assert_eq!(
        definition["condition_definition_fingerprint"],
        CONDITION_FINGERPRINT
    );
    assert_eq!(
        definition["admitted_input_classes"],
        json!(["admitted_evidence_ref"])
    );
    assert_eq!(definition["maximum_validity_seconds"], 2592000);
    assert_eq!(
        definition["outcome_precedence"],
        json!(["refused", "unresolved", "stale", "unknown", "false", "true"])
    );
    assert_eq!(
        definition["owner_module"],
        "handbook_engine::project_condition_evidence"
    );
    assert_eq!(
        definition["transport"],
        "bounded_transaction_tip_fixed_head_and_retained_refs"
    );
    assert_eq!(definition["extensions"], json!({}));

    for (field, expected) in [
        (
            "source_schema_ref",
            "handbook.schemas.project-condition-admitted-evidence-source@1.0.0",
        ),
        (
            "evidence_schema_ref",
            "handbook.schemas.project-condition-evidence@1.0.0",
        ),
        (
            "head_schema_ref",
            "handbook.schemas.project-condition-evidence-head@1.0.0",
        ),
        (
            "transaction_schema_ref",
            "handbook.schemas.project-condition-evidence-transaction@1.0.0",
        ),
        (
            "challenge_schema_ref",
            "handbook.schemas.project-condition-evidence-challenge@1.0.0",
        ),
        (
            "assertion_schema_ref",
            "handbook.schemas.project-condition-evidence-assertion@1.0.0",
        ),
        (
            "use_transition_schema_ref",
            "handbook.schemas.project-condition-evidence-use-transition@1.0.0",
        ),
        (
            "closure_schema_ref",
            "handbook.schemas.project-condition-evidence-evaluation-closure@1.0.0",
        ),
    ] {
        assert_eq!(definition[field], expected);
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let crates_dir = manifest_dir.parent().unwrap();
    let forbidden = [
        EVALUATOR_REF,
        "handbook.schemas.project-condition-admitted-evidence-source@1.0.0",
        "handbook.schemas.project-condition-evidence@1.0.0",
        "handbook.schemas.project-condition-evidence-head@1.0.0",
        "handbook.schemas.project-condition-evidence-transaction@1.0.0",
        "handbook.schemas.project-condition-evidence-challenge@1.0.0",
        "handbook.schemas.project-condition-evidence-assertion@1.0.0",
        "handbook.schemas.project-condition-evidence-use-transition@1.0.0",
        "handbook.schemas.project-condition-evidence-evaluation-closure@1.0.0",
    ];
    for crate_entry in fs::read_dir(crates_dir).unwrap() {
        let src = crate_entry.unwrap().path().join("src");
        if src.is_dir() {
            assert_production_tree_omits_refs(&src, &forbidden);
        }
    }
    for manifest in [
        manifest_dir.join("Cargo.toml"),
        crates_dir.parent().unwrap().join("Cargo.toml"),
    ] {
        let text = fs::read_to_string(&manifest).unwrap();
        for reference in forbidden {
            assert!(
                !text.contains(reference),
                "Cargo manifest {} unexpectedly registers {reference}",
                manifest.display()
            );
        }
    }
}

#[test]
fn closure_outcome_reason_and_fingerprint_nullability_branches_are_total() {
    let pairs = [
        ("refused", "evidence_refused"),
        ("unresolved", "evidence_record_missing"),
        ("unresolved", "evidence_dependency_unresolved"),
        ("stale", "evidence_not_current"),
        ("unknown", "insufficient_current_proof"),
        ("false", "affirmative_no_qualifying_responsibility"),
        ("true", "qualifying_continuing_responsibility"),
    ];
    for (outcome, reason) in pairs {
        let mut value = closure();
        value["outcome"] = json!(outcome);
        value["reason"] = json!(reason);
        assert_valid("closure", &value);
        value["reason"] = json!("evidence_refused");
        if (outcome, reason) != ("refused", "evidence_refused") {
            assert_invalid("closure", &value, "mismatched outcome/reason");
        }
    }

    let branches = [
        (None, None, None),
        (Some('a'), None, None),
        (Some('a'), Some('b'), None),
        (Some('a'), Some('b'), Some('c')),
    ];
    for (head, record, verification) in branches {
        let mut value = closure();
        value["evidence_head_fingerprint"] =
            head.map_or(Value::Null, |digit| json!(fingerprint(digit)));
        value["evidence_record_fingerprint"] =
            record.map_or(Value::Null, |digit| json!(fingerprint(digit)));
        value["producer_verification_fingerprint"] =
            verification.map_or(Value::Null, |digit| json!(fingerprint(digit)));
        assert_valid("closure", &value);
    }
    let mut skipped = closure();
    skipped["evidence_record_fingerprint"] = json!(fingerprint('b'));
    assert_invalid("closure", &skipped, "record without head");
    skipped = closure();
    skipped["producer_verification_fingerprint"] = json!(fingerprint('c'));
    assert_invalid("closure", &skipped, "verification without record");
}

#[test]
fn closure_dependency_observation_matrix_is_exact_and_closed() {
    for (kind, error) in [
        ("absent", "transaction_store_missing"),
        ("unsafe", "transaction_store_unsafe"),
        ("read_error", "transaction_store_read_failed"),
        ("directory_limit_exceeded", "transaction_store_over_limit"),
        ("retained_directory", "transaction_entry_invalid"),
        ("retained_directory", "transaction_entry_alias"),
    ] {
        let mut value = closure();
        value["dependency_observations"][0] = store_observation(kind, Some(error));
        assert_valid("closure", &value);
    }
    let mut retained = closure();
    retained["dependency_observations"][0] = store_observation("retained_directory", None);
    assert_valid("closure", &retained);

    let mut wrong_tuple = closure();
    wrong_tuple["dependency_observations"][0] = store_observation(
        "directory_limit_exceeded",
        Some("transaction_store_over_limit"),
    );
    wrong_tuple["dependency_observations"][0]["byte_count"] = json!(1);
    assert_invalid("closure", &wrong_tuple, "over-limit byte leakage");

    let mut wrong_error = closure();
    wrong_error["dependency_observations"][0] =
        store_observation("unsafe", Some("transaction_store_missing"));
    assert_invalid("closure", &wrong_error, "store error mismatch");

    for store_only_error in [
        "transaction_store_missing",
        "transaction_store_unsafe",
        "transaction_store_read_failed",
        "transaction_entry_invalid",
        "transaction_entry_alias",
        "transaction_store_over_limit",
    ] {
        let mut cross_kind = closure();
        cross_kind["dependency_observations"] = json!([
            store_observation("retained_directory", None),
            {
                "dependency_kind": "evidence_head",
                "logical_id": CONDITION_REF,
                "relative_ref": ".handbook/state/project-condition-evidence/managed-operational-surface-head.json",
                "observation_kind": "absent",
                "byte_count": null,
                "observed_bytes_fingerprint": null,
                "required_fingerprint": null,
                "claimed_fingerprint": null,
                "computed_fingerprint": null,
                "error_code": store_only_error,
            }
        ]);
        assert_invalid(
            "closure",
            &cross_kind,
            &format!("store-only error on non-store row: {store_only_error}"),
        );
    }
    let mut non_store_error_on_store = closure();
    non_store_error_on_store["dependency_observations"][0] =
        store_observation("absent", Some("dependency_missing"));
    assert_invalid(
        "closure",
        &non_store_error_on_store,
        "non-store error on store row",
    );

    let accepted_bytes = json!({
        "dependency_kind": "condition_definition",
        "logical_id": CONDITION_REF,
        "relative_ref": "definitions/project-conditions/managed.yaml",
        "observation_kind": "retained_bytes",
        "byte_count": 131072,
        "observed_bytes_fingerprint": fingerprint('1'),
        "required_fingerprint": null,
        "claimed_fingerprint": fingerprint('2'),
        "computed_fingerprint": fingerprint('2'),
        "error_code": null,
    });
    let mut accepted = closure();
    accepted["dependency_observations"] = json!([
        store_observation("retained_directory", None),
        accepted_bytes
    ]);
    assert_valid("closure", &accepted);
    accepted["dependency_observations"][1]["byte_count"] = json!(131073);
    assert_invalid("closure", &accepted, "retained byte ceiling");

    let mut rejected = closure();
    rejected["dependency_observations"] = json!([
        store_observation("retained_directory", None),
        {
            "dependency_kind": "evidence_head",
            "logical_id": CONDITION_REF,
            "relative_ref": ".handbook/state/project-condition-evidence/managed-operational-surface-head.json",
            "observation_kind": "absent",
            "byte_count": null,
            "observed_bytes_fingerprint": null,
            "required_fingerprint": fingerprint('a'),
            "claimed_fingerprint": null,
            "computed_fingerprint": null,
            "error_code": "dependency_missing",
        }
    ]);
    assert_valid("closure", &rejected);
    rejected["dependency_observations"][1]["observed_bytes_fingerprint"] = json!(fingerprint('b'));
    assert_invalid("closure", &rejected, "absent observed bytes");

    for pointer in [
        "/dependency_observations/0",
        "/freshness_basis",
        "/ordered_inputs",
    ] {
        let mut nested = closure();
        if pointer == "/freshness_basis" {
            nested["freshness_basis"] = json!([freshness_row("current")]);
        } else if pointer == "/ordered_inputs" {
            nested["ordered_inputs"] = json!([resolved_input()]);
        }
        let target = if pointer == "/freshness_basis" {
            "/freshness_basis/0"
        } else if pointer == "/ordered_inputs" {
            "/ordered_inputs/0"
        } else {
            pointer
        };
        nested
            .pointer_mut(target)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("unexpected".to_owned(), json!(true));
        assert_invalid("closure", &nested, "unknown nested closure field");
    }
}

fn freshness_row(classification: &str) -> Value {
    json!({
        "input_id": "source-a",
        "source_fingerprint": fingerprint('9'),
        "evaluated_at_utc": "2026-07-01T00:00:02Z",
        "observed_at_utc": "2026-07-01T00:00:00Z",
        "verified_at_utc": "2026-07-01T00:00:01Z",
        "valid_until_utc": "2026-07-31T00:00:00Z",
        "maximum_validity_seconds": 2592000,
        "classification": classification,
    })
}

fn resolved_input() -> Value {
    let source = positive_source();
    json!({
        "input_id": "source-a",
        "input_class": "admitted_evidence_ref",
        "source_ref": format!("sources/admitted-evidence_{}.json", "9".repeat(64)),
        "source_fingerprint": fingerprint('9'),
        "source": source,
    })
}

fn classify(evaluated: &str, verified: &str, observed: &str, valid_until: &str) -> &'static str {
    if evaluated < verified {
        "verification_not_yet_valid"
    } else if evaluated < observed {
        "observation_not_yet_valid"
    } else if evaluated <= valid_until {
        "current"
    } else {
        "expired"
    }
}

fn utc_second(value: &str) -> i64 {
    fn number(bytes: &[u8]) -> i64 {
        bytes.iter().fold(0, |value, byte| {
            value * 10 + i64::from(byte.saturating_sub(b'0'))
        })
    }
    let bytes = value.as_bytes();
    let year = number(&bytes[0..4]);
    let month = number(&bytes[5..7]);
    let day = number(&bytes[8..10]);
    let adjusted_year = year - i64::from(month <= 2);
    let era = adjusted_year.div_euclid(400);
    let year_of_era = adjusted_year - era * 400;
    let adjusted_month = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * adjusted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days = era * 146097 + day_of_era - 719468;
    days * 86400
        + number(&bytes[11..13]) * 3600
        + number(&bytes[14..16]) * 60
        + number(&bytes[17..19])
}

fn source_validity_is_admissible(observed_at: &str, valid_until: &str) -> bool {
    let observed_at = utc_second(observed_at);
    let valid_until = utc_second(valid_until);
    observed_at < valid_until && valid_until - observed_at <= 2_592_000
}

#[test]
fn freshness_rules_cover_lower_upper_and_maximum_validity_boundaries() {
    assert_eq!(
        classify(
            "2026-07-01T00:00:00Z",
            "2026-07-01T00:00:01Z",
            "2026-07-01T00:00:02Z",
            "2026-07-31T00:00:02Z"
        ),
        "verification_not_yet_valid"
    );
    assert_eq!(
        classify(
            "2026-07-01T00:00:01Z",
            "2026-07-01T00:00:01Z",
            "2026-07-01T00:00:02Z",
            "2026-07-31T00:00:02Z"
        ),
        "observation_not_yet_valid"
    );
    assert_eq!(
        classify(
            "2026-07-01T00:00:02Z",
            "2026-07-01T00:00:01Z",
            "2026-07-01T00:00:02Z",
            "2026-07-31T00:00:02Z"
        ),
        "current"
    );
    assert_eq!(
        classify(
            "2026-07-31T00:00:02Z",
            "2026-07-01T00:00:01Z",
            "2026-07-01T00:00:02Z",
            "2026-07-31T00:00:02Z"
        ),
        "current"
    );
    assert_eq!(
        classify(
            "2026-07-31T00:00:03Z",
            "2026-07-01T00:00:01Z",
            "2026-07-01T00:00:02Z",
            "2026-07-31T00:00:02Z"
        ),
        "expired"
    );
    assert_eq!(
        utc_second("2026-07-31T00:00:00Z") - utc_second("2026-07-01T00:00:00Z"),
        2592000
    );
    assert!(utc_second("2026-07-01T00:00:00Z") < utc_second("2026-07-31T00:00:00Z"));
    assert!(source_validity_is_admissible(
        "2026-07-01T00:00:00Z",
        "2026-07-31T00:00:00Z"
    ));
    assert!(!source_validity_is_admissible(
        "2026-07-01T00:00:00Z",
        "2026-07-31T00:00:01Z"
    ));
    assert!(!source_validity_is_admissible(
        "2026-07-01T00:00:00Z",
        "2026-07-01T00:00:00Z"
    ));

    for classification in [
        "verification_not_yet_valid",
        "observation_not_yet_valid",
        "current",
        "expired",
    ] {
        let mut value = closure();
        value["freshness_basis"] = json!([freshness_row(classification)]);
        value["ordered_inputs"] = json!([resolved_input()]);
        assert_valid("closure", &value);
    }
    let mut invalid = closure();
    invalid["freshness_basis"] = json!([freshness_row("stale")]);
    assert_invalid("closure", &invalid, "unknown freshness classification");
}

fn assert_content_addressed_pair(reference: &str, fingerprint: &str) {
    let suffix = fingerprint.strip_prefix("sha256:").unwrap();
    assert!(
        reference.contains(suffix),
        "{reference} must bind fingerprint {fingerprint}"
    );
}

#[test]
fn cross_field_identity_order_and_advance_rules_have_replayable_vectors() {
    let record = evidence();
    assert_content_addressed_pair(
        record["inputs"][0]["source_ref"].as_str().unwrap(),
        record["inputs"][0]["source_fingerprint"].as_str().unwrap(),
    );
    let verification = &record["producer_verification"];
    for (reference, fingerprint) in [
        (
            "approver_registry_state_ref",
            "approver_registry_state_fingerprint",
        ),
        (
            "registry_head_transition_ref",
            "registry_head_transition_fingerprint",
        ),
        ("challenge_ref", "challenge_fingerprint"),
        ("assertion_ref", "assertion_fingerprint"),
        (
            "authenticator_use_transition_ref",
            "authenticator_use_transition_fingerprint",
        ),
    ] {
        assert_content_addressed_pair(
            verification[reference].as_str().unwrap(),
            verification[fingerprint].as_str().unwrap(),
        );
    }

    let mut rows = record["inputs"].as_array().unwrap().clone();
    rows.push(json!({
        "input_id": "source-0",
        "input_class": "admitted_evidence_ref",
        "source_ref": format!("sources/admitted-evidence_{}.json", "a".repeat(64)),
        "source_fingerprint": fingerprint('a'),
    }));
    assert!(
        rows.windows(2)
            .any(|pair| pair[0]["input_id"].as_str() > pair[1]["input_id"].as_str()),
        "unsorted vector must be detectable"
    );
    rows.sort_by(|left, right| left["input_id"].as_str().cmp(&right["input_id"].as_str()));
    assert!(rows
        .windows(2)
        .all(|pair| pair[0]["input_id"].as_str() < pair[1]["input_id"].as_str()));

    let transition = use_transition();
    assert!(
        transition["sign_count"].as_u64().unwrap()
            > transition["prior_sign_count"].as_u64().unwrap()
    );
    assert_eq!(transition["sequence"], 1);
    assert_content_addressed_pair(
        transition["assertion_ref"].as_str().unwrap(),
        transition["assertion_fingerprint"].as_str().unwrap(),
    );
}

fn envelope(raw_names: &[Vec<u8>], filename_encoding: &str) -> Vec<u8> {
    let mut entries = raw_names.to_vec();
    entries.sort();
    let value = json!({
        "filename_encoding": filename_encoding,
        "entries": entries
            .iter()
            .map(|name| BASE64_STANDARD.encode(name))
            .collect::<Vec<_>>(),
    });
    serde_json_canonicalizer::to_vec(&value).unwrap()
}

fn transaction_name(index: usize) -> String {
    format!("transaction_{index:04}_{index:064x}.json", index = index)
}

fn prefixed_name(prefix: &str, index: usize, length: usize) -> Vec<u8> {
    let start = format!("{prefix}{index:04}-");
    assert!(start.len() <= length);
    format!("{start}{}", "a".repeat(length - start.len())).into_bytes()
}

fn platform_bytes(names: impl IntoIterator<Item = String>, windows: bool) -> Vec<Vec<u8>> {
    names
        .into_iter()
        .map(|name| {
            if windows {
                name.encode_utf16()
                    .flat_map(u16::to_le_bytes)
                    .collect::<Vec<_>>()
            } else {
                name.into_bytes()
            }
        })
        .collect()
}

fn assert_envelope(
    names: Vec<Vec<u8>>,
    encoding: &str,
    expected_bytes: usize,
    expected_fingerprint: &str,
) {
    let bytes = envelope(&names, encoding);
    assert_eq!(bytes.len(), expected_bytes);
    assert_eq!(
        DefinitionFingerprint::from_bytes(&bytes).as_str(),
        expected_fingerprint
    );
}

#[derive(Debug, PartialEq)]
struct ProjectionLimit {
    retained_entries: usize,
    candidate_count: usize,
    candidate_envelope_bytes: Option<usize>,
}

fn project_bounded_envelope(
    names: impl IntoIterator<Item = Vec<u8>>,
    encoding: &str,
) -> Result<Vec<u8>, ProjectionLimit> {
    let mut retained = Vec::new();
    let mut projected = match encoding {
        "utf8" => 41usize,
        "windows_utf16le" => 52usize,
        other => panic!("unsupported test encoding {other}"),
    };
    for name in names {
        let candidate_count = retained
            .len()
            .checked_add(1)
            .expect("test count arithmetic");
        if candidate_count > 4096 {
            return Err(ProjectionLimit {
                retained_entries: retained.len(),
                candidate_count,
                candidate_envelope_bytes: None,
            });
        }
        let base64_length = name
            .len()
            .checked_add(2)
            .and_then(|value| value.checked_div(3))
            .and_then(|value| value.checked_mul(4))
            .expect("test base64 arithmetic");
        let candidate_envelope_bytes = projected
            .checked_add(usize::from(!retained.is_empty()))
            .and_then(|value| value.checked_add(2))
            .and_then(|value| value.checked_add(base64_length))
            .expect("test envelope arithmetic");
        if candidate_envelope_bytes > 131072 {
            return Err(ProjectionLimit {
                retained_entries: retained.len(),
                candidate_count,
                candidate_envelope_bytes: Some(candidate_envelope_bytes),
            });
        }
        retained.push(name);
        projected = candidate_envelope_bytes;
    }
    let complete = envelope(&retained, encoding);
    assert_eq!(complete.len(), projected);
    Ok(complete)
}

#[test]
fn transaction_store_count_byte_projection_and_complete_envelope_vectors_replay() {
    let contract = &schema("closure")["x-handbook-transaction-store"];
    assert_eq!(contract["maximum_entries"], 4096);
    assert_eq!(contract["maximum_canonical_envelope_bytes"], 131072);
    assert_eq!(contract["empty_envelope_bytes"]["utf8"], 41);
    assert_eq!(contract["empty_envelope_bytes"]["windows_utf16le"], 52);
    assert_eq!(
        contract["limit_observation"],
        json!({
            "byte_count": null,
            "claimed_fingerprint": null,
            "computed_fingerprint": null,
            "error_code": "transaction_store_over_limit",
            "observation_kind": "directory_limit_exceeded",
            "observed_bytes_fingerprint": null,
            "required_fingerprint": null,
        })
    );

    let count_names = (0..4096)
        .map(|index| format!("c{index:04}"))
        .collect::<Vec<_>>();
    assert_envelope(
        platform_bytes(count_names.clone(), false),
        "utf8",
        45096,
        "sha256:fe286f6ba67aa4f2af1036eb99e75df54fc72957c9c19a2e494c46fab77c8dfc",
    );
    assert_envelope(
        platform_bytes(count_names, true),
        "windows_utf16le",
        77875,
        "sha256:8e1215a1d217e2e8654da5d057e69dca9a481cf9b11270230a2b4e9dae8f697d",
    );
    assert!(project_bounded_envelope(
        (0..4096).map(|index| format!("c{index:04}").into_bytes()),
        "utf8"
    )
    .is_ok());
    assert_eq!(
        project_bounded_envelope(
            (0..4097).map(|index| format!("c{index:04}").into_bytes()),
            "utf8"
        ),
        Err(ProjectionLimit {
            retained_entries: 4096,
            candidate_count: 4097,
            candidate_envelope_bytes: None,
        })
    );

    assert_envelope(
        platform_bytes((1..=1101).map(transaction_name), false),
        "utf8",
        131059,
        "sha256:3c37bbe60138407f496a24f7a61f58f44175c81e4038e7d5de2549c2c1327a0e",
    );
    assert!(
        envelope(
            &platform_bytes((1..=1102).map(transaction_name), false),
            "utf8"
        )
        .len()
            > 131072
    );
    assert_eq!(
        project_bounded_envelope(
            platform_bytes((1..=1102).map(transaction_name), false),
            "utf8"
        ),
        Err(ProjectionLimit {
            retained_entries: 1101,
            candidate_count: 1102,
            candidate_envelope_bytes: Some(131178),
        })
    );
    assert_envelope(
        platform_bytes((1..=557).map(transaction_name), true),
        "windows_utf16le",
        130946,
        "sha256:87881b12bfa5937094c20f030f6a818d5e826c13644f0aeb69b28d98c8e46dea",
    );
    assert!(
        envelope(
            &platform_bytes((1..=558).map(transaction_name), true),
            "windows_utf16le"
        )
        .len()
            > 131072
    );
    assert_eq!(
        project_bounded_envelope(
            platform_bytes((1..=558).map(transaction_name), true),
            "windows_utf16le"
        ),
        Err(ProjectionLimit {
            retained_entries: 557,
            candidate_count: 558,
            candidate_envelope_bytes: Some(131181),
        })
    );

    let unix_exact = (0..86)
        .map(|index| prefixed_name("u84-", index, 84))
        .chain((0..1018).map(|index| prefixed_name("u86-", index, 86)))
        .collect::<Vec<_>>();
    assert_envelope(
        unix_exact.clone(),
        "utf8",
        131072,
        "sha256:8e7ecc78200f2835ed60bb652de75728e3d59e12754c09ff630bad28094c5ea6",
    );
    assert_eq!(
        project_bounded_envelope(unix_exact, "utf8").unwrap().len(),
        131072
    );
    let unix_131073 = (0..56)
        .map(|index| prefixed_name("u84-", index, 84))
        .chain((0..1047).map(|index| prefixed_name("u86-", index, 86)));
    assert_eq!(
        project_bounded_envelope(unix_131073, "utf8"),
        Err(ProjectionLimit {
            retained_entries: 1102,
            candidate_count: 1103,
            candidate_envelope_bytes: Some(131073),
        })
    );

    let windows_exact = (0..86)
        .map(|index| prefixed_name("w85-", index, 85))
        .chain((0..473).map(|index| prefixed_name("w86-", index, 86)))
        .map(|name| {
            String::from_utf8(name)
                .unwrap()
                .encode_utf16()
                .flat_map(u16::to_le_bytes)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert_envelope(
        windows_exact.clone(),
        "windows_utf16le",
        131072,
        "sha256:d0677bb6103e70003e5c7ae1eaa15be84a06034cd6401403bb772ec3cd2dba30",
    );
    assert_eq!(
        project_bounded_envelope(windows_exact, "windows_utf16le")
            .unwrap()
            .len(),
        131072
    );
    let windows_131073 = (0..27)
        .map(|index| prefixed_name("w85-", index, 85))
        .chain((0..531).map(|index| prefixed_name("w86-", index, 86)))
        .map(|name| {
            String::from_utf8(name)
                .unwrap()
                .encode_utf16()
                .flat_map(u16::to_le_bytes)
                .collect::<Vec<_>>()
        });
    assert_eq!(
        project_bounded_envelope(windows_131073, "windows_utf16le"),
        Err(ProjectionLimit {
            retained_entries: 557,
            candidate_count: 558,
            candidate_envelope_bytes: Some(131073),
        })
    );

    let unix_long = platform_bytes((1..=1099).map(transaction_name), false)
        .into_iter()
        .chain(std::iter::once(vec![b'z'; 255]));
    assert_eq!(
        project_bounded_envelope(unix_long, "utf8"),
        Err(ProjectionLimit {
            retained_entries: 1099,
            candidate_count: 1100,
            candidate_envelope_bytes: Some(131164),
        })
    );
    let windows_long = platform_bytes((1..=555).map(transaction_name), true)
        .into_iter()
        .chain(std::iter::once(
            "z".repeat(255)
                .encode_utf16()
                .flat_map(u16::to_le_bytes)
                .collect(),
        ));
    assert_eq!(
        project_bounded_envelope(windows_long, "windows_utf16le"),
        Err(ProjectionLimit {
            retained_entries: 555,
            candidate_count: 556,
            candidate_envelope_bytes: Some(131159),
        })
    );

    let mut unix_ordinal = 0usize;
    let unix_arbitrary_suffix = std::iter::from_fn(move || {
        unix_ordinal += 1;
        match unix_ordinal {
            1..=1102 => Some(transaction_name(unix_ordinal).into_bytes()),
            _ => panic!("Unix arbitrary suffix requested after offender"),
        }
    });
    assert!(project_bounded_envelope(unix_arbitrary_suffix, "utf8").is_err());

    let mut windows_ordinal = 0usize;
    let windows_arbitrary_suffix = std::iter::from_fn(move || {
        windows_ordinal += 1;
        match windows_ordinal {
            1..=558 => Some(
                transaction_name(windows_ordinal)
                    .encode_utf16()
                    .flat_map(u16::to_le_bytes)
                    .collect(),
            ),
            _ => panic!("Windows arbitrary suffix requested after offender"),
        }
    });
    assert!(project_bounded_envelope(windows_arbitrary_suffix, "windows_utf16le").is_err());
}

#[test]
fn closure_count_bounds_and_dependency_order_are_exact() {
    let contract = schema("closure");
    assert_eq!(contract["properties"]["freshness_basis"]["maxItems"], 16);
    assert_eq!(contract["properties"]["ordered_inputs"]["maxItems"], 16);
    assert_eq!(
        contract["x-handbook-dependency-order"],
        json!([
            "evidence_transaction_store",
            "evidence_head",
            "evidence_transaction",
            "evidence_record",
            "admitted_source",
            "condition_definition",
            "evaluator_definition",
            "repository_identity",
            "registry_head_transition",
            "registry_state",
            "credential_registration",
            "authenticator_use_head",
            "challenge",
            "decoded_assertion_response",
            "assertion",
            "evidence_use_transition",
        ])
    );
    let mut value = closure();
    value["freshness_basis"] = Value::Array((0..17).map(|_| freshness_row("current")).collect());
    assert_invalid("closure", &value, "17 freshness rows");
    value = closure();
    value["ordered_inputs"] = Value::Array((0..17).map(|_| resolved_input()).collect());
    assert_invalid("closure", &value, "17 ordered inputs");
}
