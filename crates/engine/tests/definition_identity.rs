use handbook_engine::{DefinitionFingerprint, ExactDefinitionRef, RegistryLoadErrorKind};
use serde_json::json;

#[test]
fn exact_refs_require_the_frozen_identity_and_canonical_semver_grammar() {
    let exact = ExactDefinitionRef::parse("example.schemas.incident-brief@1.2.3-alpha.1+build.7")
        .expect("valid exact ref");
    assert_eq!(exact.identity(), "example.schemas.incident-brief");
    assert_eq!(exact.version().to_string(), "1.2.3-alpha.1+build.7");
    assert_eq!(
        exact.as_str(),
        "example.schemas.incident-brief@1.2.3-alpha.1+build.7"
    );

    for invalid in [
        "ab@1.0.0",
        "example@1.0.0",
        "Example.schemas@1.0.0",
        "example._schemas@1.0.0",
        "example.schemas-@1.0.0",
        "example..schemas@1.0.0",
        "example.schemas@1.0",
        "example.schemas@01.0.0",
        "example.schemas@latest",
        " example.schemas@1.0.0",
        "example.schemas@1.0.0 ",
        "example.schemas@@1.0.0",
        "example.schemas/%2e%2e@1.0.0",
        "example.schémas@1.0.0",
    ] {
        let error = ExactDefinitionRef::parse(invalid).expect_err(invalid);
        assert_eq!(
            error.kind(),
            RegistryLoadErrorKind::InvalidExactDefinitionRef
        );
    }

    let secret_identity = format!("SECRET_IDENTITY.{}", "x".repeat(1_000));
    let error = ExactDefinitionRef::new(&secret_identity, "1.0.0")
        .expect_err("oversized authored identity");
    assert_eq!(
        error.kind(),
        RegistryLoadErrorKind::InvalidExactDefinitionRef
    );
    assert!(!error.detail().contains("SECRET_IDENTITY"));
    assert!(!error.to_string().contains("SECRET_IDENTITY"));
    assert!(error.detail().len() < 256);
}

#[test]
fn exact_identity_segment_and_total_byte_bounds_are_enforced() {
    let segment_63 = format!("a{}", "1".repeat(62));
    ExactDefinitionRef::new(&format!("aa.{segment_63}"), "1.0.0").expect("63-byte segment");

    let segment_64 = format!("a{}", "1".repeat(63));
    assert!(ExactDefinitionRef::new(&format!("aa.{segment_64}"), "1.0.0").is_err());

    let identity_255 = format!(
        "{}.{}.{}.{}",
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(63)
    );
    assert_eq!(identity_255.len(), 255);
    ExactDefinitionRef::new(&identity_255, "1.0.0").expect("255-byte identity");

    let identity_256 = format!("{identity_255}.a");
    assert!(ExactDefinitionRef::new(&identity_256, "1.0.0").is_err());
}

#[test]
fn fingerprints_are_lowercase_sha256_and_recomputed_from_jcs_bytes() {
    let value_a = json!({"z": 1, "a": [3, 2, 1]});
    let value_b = json!({"a": [3, 2, 1], "z": 1});

    let a = DefinitionFingerprint::from_json_value(&value_a).expect("fingerprint");
    let b = DefinitionFingerprint::from_json_value(&value_b).expect("fingerprint");
    assert_eq!(a, b);
    assert!(a.as_str().starts_with("sha256:"));
    assert_eq!(a.as_str().len(), 71);

    for invalid in [
        "",
        "sha256:abc",
        "SHA256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "sha256:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        "sha256:gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg",
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa ",
    ] {
        let error = DefinitionFingerprint::parse(invalid).expect_err(invalid);
        assert_eq!(error.kind(), RegistryLoadErrorKind::InvalidFingerprint);
    }
}

#[test]
fn duplicate_yaml_and_json_mapping_keys_fail_before_typed_decoding() {
    let yaml = b"schema_id: first\nschema_id: second\n";
    let json = br#"{"schema_id":"first","schema_id":"second"}"#;

    assert_eq!(
        handbook_engine::parse_definition_yaml(yaml)
            .expect_err("duplicate YAML")
            .kind(),
        RegistryLoadErrorKind::DuplicateKey
    );
    assert_eq!(
        handbook_engine::parse_schema_json(json)
            .expect_err("duplicate JSON")
            .kind(),
        RegistryLoadErrorKind::DuplicateKey
    );

    let yaml_secret_key = format!("SECRET_DUPLICATE_KEY_{}", "x".repeat(500));
    let json_secret_key = format!("SECRET_DUPLICATE_KEY_{}", "x".repeat(50_000));
    let yaml = format!("{yaml_secret_key}: first\n{yaml_secret_key}: second\n");
    let json = format!(r#"{{"{json_secret_key}":"first","{json_secret_key}":"second"}}"#);
    for error in [
        handbook_engine::parse_definition_yaml(yaml.as_bytes())
            .expect_err("large duplicate YAML key"),
        handbook_engine::parse_schema_json(json.as_bytes()).expect_err("large duplicate JSON key"),
    ] {
        assert_eq!(error.kind(), RegistryLoadErrorKind::DuplicateKey);
        assert!(!error.detail().contains("SECRET_DUPLICATE_KEY"));
        assert!(!error.to_string().contains("SECRET_DUPLICATE_KEY"));
        assert!(error.detail().len() < 256);
    }
}

#[test]
fn source_document_and_aggregate_limits_fail_closed() {
    let over_document = vec![b'x'; handbook_engine::MAX_SOURCE_DOCUMENT_BYTES + 1];
    assert_eq!(
        handbook_engine::parse_definition_yaml(&over_document)
            .expect_err("over document limit")
            .kind(),
        RegistryLoadErrorKind::SourceLimitExceeded
    );

    let mut budget = handbook_engine::SourceByteBudget::default();
    for _ in 0..8 {
        budget
            .admit(handbook_engine::MAX_SOURCE_DOCUMENT_BYTES)
            .expect("document within aggregate limit");
    }
    assert_eq!(
        budget.total_bytes(),
        handbook_engine::MAX_TOTAL_SOURCE_BYTES
    );
    assert_eq!(
        budget.admit(1).expect_err("over aggregate limit").kind(),
        RegistryLoadErrorKind::AggregateLimitExceeded
    );
}
