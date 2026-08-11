use handbook_sdk::contract_membrane::{
    canonical_fingerprint, canonical_json_bytes, parse_canonical_json, sha256_fingerprint,
    ArtifactLocator, ArtifactLocatorRef, ArtifactRef, BoundedString, BoundedVec, CatalogCursor,
    CatalogPage, CatalogRoot, ContractError, ContractLimits, ContractMember, ContractValue,
    ContractValueRef, ContractWitness, CorrelationRef, DefinitionPinnedKeyPointer, Diagnostic,
    DiagnosticSeverity, DigestAlgorithm, DraftVersion, ExactBinding, ExactRef, Fingerprint,
    IdempotencyStage, IdempotencyState, IdempotencyStateRef, NextAction, NextActionKind, Omission,
    OmissionReason, Problem, ProblemBinding, ProblemCategory, ProofEffect, RawIdempotencyKey,
    RecheckCondition, RetryDirective, SafeI64, SafeU64, SchemaManifestEntry, SchemaMediaType,
    SemVer, Sensitivity, SourceBinding, TerminalFields, TerminalStatus, UtcTimestamp,
    VerificationError, VerifyContract, WriteReceipt,
};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Serialize, Serializer};

#[test]
fn bounded_primitives_enforce_hard_and_selected_limits() {
    assert_eq!(
        BoundedString::<65_537>::new("x".to_owned()),
        Err(ContractError::LimitOutOfRange)
    );
    assert_eq!(
        BoundedString::<8>::new(String::new()),
        Err(ContractError::Empty)
    );
    assert!(BoundedString::<4>::new("four".to_owned()).is_ok());
    assert_eq!(
        BoundedString::<4>::new("five!".to_owned()),
        Err(ContractError::BoundExceeded)
    );

    assert_eq!(
        BoundedVec::<u8, 129>::new(Vec::new()),
        Err(ContractError::LimitOutOfRange)
    );
    assert!(BoundedVec::<u8, 2>::new(vec![1, 2]).is_ok());
    assert_eq!(
        BoundedVec::<u8, 2>::new(vec![1, 2, 3]),
        Err(ContractError::BoundExceeded)
    );

    let packet0 = ContractLimits::packet0();
    assert_eq!(packet0.document_bytes(), 1_048_576);
    assert_eq!(packet0.string_bytes(), 65_536);
    assert_eq!(packet0.array_items(), 128);
    assert_eq!(packet0.object_members(), 128);
    assert_eq!(packet0.nesting_depth(), 32);
    assert_eq!(
        ContractLimits::new(0, 1, 1, 1, 1),
        Err(ContractError::LimitOutOfRange)
    );
    assert_eq!(
        ContractLimits::new(1_048_577, 1, 1, 1, 1),
        Err(ContractError::LimitOutOfRange)
    );

    let bounded = BoundedString::<4>::new("four".to_owned()).unwrap();
    assert_eq!(bounded.clone().into_string(), "four");
    assert!(serde_json::from_str::<BoundedString<4>>(r#""five!""#).is_err());
    let vector = BoundedVec::<u8, 2>::new(vec![1, 2]).unwrap();
    assert_eq!(vector.clone().into_vec(), vec![1, 2]);
    assert!(serde_json::from_str::<BoundedVec<u8, 2>>("[1,2,3]").is_err());
}

#[test]
fn safe_integers_and_utc_timestamps_are_lexically_strict() {
    assert_eq!(
        SafeU64::new(9_007_199_254_740_991).unwrap().get(),
        9_007_199_254_740_991
    );
    assert_eq!(
        SafeU64::new(9_007_199_254_740_992),
        Err(ContractError::InvalidNumber)
    );
    assert_eq!(
        SafeI64::new(-9_007_199_254_740_991).unwrap().get(),
        -9_007_199_254_740_991
    );
    assert_eq!(
        SafeI64::new(-9_007_199_254_740_992),
        Err(ContractError::InvalidNumber)
    );

    let limits = ContractLimits::packet0();
    assert_eq!(
        parse_canonical_json::<SafeI64>(b"-0", &limits),
        Err(ContractError::InvalidNumber)
    );
    assert_eq!(
        parse_canonical_json::<SafeI64>(b"1e0", &limits),
        Err(ContractError::InvalidNumber)
    );
    assert_eq!(
        parse_canonical_json::<SafeU64>(b"9007199254740992", &limits),
        Err(ContractError::InvalidNumber)
    );

    let leap_day = UtcTimestamp::parse("2024-02-29T23:59:59Z").unwrap();
    assert_eq!(leap_day.as_str(), "2024-02-29T23:59:59Z");
    for invalid in [
        "2023-02-29T00:00:00Z",
        "2024-01-01T00:00:00.0Z",
        "2024-01-01T00:00:00+00:00",
        "0000-01-01T00:00:00Z",
        "2024-01-01T00:00:60Z",
    ] {
        assert_eq!(
            UtcTimestamp::parse(invalid),
            Err(ContractError::InvalidTimestamp)
        );
    }
}

#[test]
fn contract_values_are_opaque_bounded_and_utf16_ordered() {
    let supplementary = ContractMember::new(
        BoundedString::new("\u{10000}".to_owned()).unwrap(),
        ContractValue::null(),
    );
    let private_use = ContractMember::new(
        BoundedString::new("\u{e000}".to_owned()).unwrap(),
        ContractValue::boolean(true),
    );
    let value = ContractValue::object(vec![supplementary.clone(), private_use.clone()]).unwrap();
    assert!(matches!(value.as_ref(), ContractValueRef::Object(members) if members.len() == 2));
    assert_eq!(
        ContractValue::object(vec![private_use, supplementary]),
        Err(ContractError::NonCanonicalOrder)
    );

    let duplicate = ContractMember::new(
        BoundedString::new("same".to_owned()).unwrap(),
        ContractValue::null(),
    );
    assert_eq!(
        ContractValue::object(vec![duplicate.clone(), duplicate]),
        Err(ContractError::DuplicateMember)
    );

    let mut nested = ContractValue::null();
    for _ in 0..32 {
        nested = ContractValue::array(vec![nested]).unwrap();
    }
    assert_eq!(
        ContractValue::array(vec![nested]),
        Err(ContractError::BoundExceeded)
    );
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct CanonicalFixture {
    z: SafeI64,
    a: String,
}

#[test]
fn canonical_json_is_bounded_sorted_lf_free_and_lexeme_preserving() {
    let limits = ContractLimits::packet0();
    let fixture = CanonicalFixture {
        z: SafeI64::new(7).unwrap(),
        a: "line\nend".to_owned(),
    };
    let bytes = canonical_json_bytes(&fixture, &limits).unwrap();
    assert_eq!(bytes, br#"{"a":"line\nend","z":7}"#);
    assert!(!bytes.contains(&b'\n'));
    assert_eq!(
        parse_canonical_json::<CanonicalFixture>(&bytes, &limits).unwrap(),
        fixture
    );

    assert_eq!(
        parse_canonical_json::<ContractValue>(br#"{"z":null,"a":true}"#, &limits),
        Err(ContractError::NonCanonicalJson)
    );
    assert_eq!(
        parse_canonical_json::<ContractValue>(br#"{"a":"\u0061"}"#, &limits),
        Err(ContractError::NonCanonicalJson)
    );
    assert_eq!(
        parse_canonical_json::<ContractValue>(br#"{"a":null,"a":true}"#, &limits),
        Err(ContractError::InvalidJson)
    );
    assert_eq!(
        parse_canonical_json::<ContractValue>(&[0xff], &limits),
        Err(ContractError::InvalidJson)
    );
    assert_eq!(
        parse_canonical_json::<ContractValue>(b"", &limits),
        Err(ContractError::Empty)
    );

    let tiny = ContractLimits::new(8, 8, 8, 8, 8).unwrap();
    assert_eq!(
        canonical_json_bytes(&fixture, &tiny),
        Err(ContractError::BoundExceeded)
    );
    assert_eq!(
        canonical_json_bytes(&1.0_f64, &limits),
        Err(ContractError::InvalidNumber)
    );
    assert_eq!(
        canonical_json_bytes(&f64::NAN, &limits),
        Err(ContractError::InvalidNumber)
    );
    assert_eq!(
        canonical_json_bytes(&9_007_199_254_740_992_u64, &limits),
        Err(ContractError::InvalidNumber)
    );

    struct DuplicateObject;

    impl Serialize for DuplicateObject {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = serializer.serialize_map(Some(2))?;
            map.serialize_entry("same", &1_u8)?;
            map.serialize_entry("same", &2_u8)?;
            map.end()
        }
    }

    assert_eq!(
        canonical_json_bytes(&DuplicateObject, &limits),
        Err(ContractError::InvalidJson)
    );

    let selected = ContractLimits::new(256, 4, 2, 2, 2).unwrap();
    assert_eq!(
        parse_canonical_json::<String>(br#""12345""#, &selected),
        Err(ContractError::BoundExceeded)
    );
    assert_eq!(
        parse_canonical_json::<ContractValue>(b"[[[null]]]", &selected),
        Err(ContractError::BoundExceeded)
    );
}

#[test]
fn semver_exact_refs_fingerprints_and_bindings_are_exact() {
    let version = SemVer::parse("1.2.3-alpha.1+build.05").unwrap();
    assert_eq!(version.major(), 1);
    assert_eq!(version.minor(), 2);
    assert_eq!(version.patch(), 3);
    assert_eq!(version.prerelease(), Some("alpha.1"));
    assert_eq!(version.build(), Some("build.05"));
    assert_eq!(version.as_str(), "1.2.3-alpha.1+build.05");
    assert_eq!(version.to_string(), "1.2.3-alpha.1+build.05");
    assert!(SemVer::parse("1.2.3-alpha-beta+build-x").is_ok());
    for invalid in ["1.2", "01.2.3", "1.2.3-01", "1.2.3-", "1.2.3+"] {
        assert_eq!(SemVer::parse(invalid), Err(ContractError::InvalidSemVer));
    }

    let reference = ExactRef::parse("handbook.problem@1.2.3-alpha.1+build.05").unwrap();
    assert_eq!(reference.identity(), "handbook.problem");
    assert_eq!(reference.version(), &version);
    assert_eq!(
        reference.as_str(),
        "handbook.problem@1.2.3-alpha.1+build.05"
    );
    assert_eq!(
        ExactRef::parse("Handbook.problem@1.2.3"),
        Err(ContractError::InvalidExactRef)
    );

    let fingerprint = Fingerprint::parse(
        "sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
    )
    .unwrap();
    assert_eq!(sha256_fingerprint(b"abc"), fingerprint);
    assert_eq!(
        Fingerprint::parse(
            "sha256:BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD"
        ),
        Err(ContractError::InvalidFingerprint)
    );

    let binding = ExactBinding::new(reference, fingerprint.clone());
    assert_eq!(binding.reference().identity(), "handbook.problem");
    assert_eq!(binding.fingerprint(), &fingerprint);
    assert_eq!(fingerprint.as_str(), binding.fingerprint().as_str());
    let limits = ContractLimits::packet0();
    assert_eq!(
        canonical_json_bytes(&binding, &limits).unwrap(),
        br#"{"fingerprint":"sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","ref":"handbook.problem@1.2.3-alpha.1+build.05"}"#
    );
    assert_eq!(
        canonical_fingerprint(&"abc", &limits).unwrap(),
        sha256_fingerprint(br#""abc""#)
    );

    let exact = canonical_json_bytes(&binding, &limits).unwrap();
    assert_eq!(
        parse_canonical_json::<ExactBinding>(&exact, &limits).unwrap(),
        binding
    );
    assert_eq!(
        parse_canonical_json::<ExactBinding>(
            br#"{"extra":null,"fingerprint":"sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","ref":"handbook.problem@1.2.3-alpha.1+build.05"}"#,
            &limits,
        ),
        Err(ContractError::InvalidJson)
    );
    assert_eq!(
        parse_canonical_json::<ExactBinding>(
            br#"{"fingerprint":null,"ref":"handbook.problem@1.2.3"}"#,
            &limits,
        ),
        Err(ContractError::InvalidJson)
    );
}

#[test]
fn contract_error_surface_is_closed_and_redacted() {
    let errors = [
        ContractError::Empty,
        ContractError::LimitOutOfRange,
        ContractError::BoundExceeded,
        ContractError::InvalidString,
        ContractError::InvalidNumber,
        ContractError::InvalidTimestamp,
        ContractError::InvalidSemVer,
        ContractError::InvalidExactRef,
        ContractError::InvalidFingerprint,
        ContractError::InvalidJson,
        ContractError::NonCanonicalJson,
        ContractError::DuplicateMember,
        ContractError::NonCanonicalOrder,
        ContractError::InvalidInvariant,
        ContractError::InvalidArtifactLocator,
        ContractError::FingerprintMismatch,
    ];
    for error in errors {
        assert!(!error.to_string().contains("secret-input"));
    }
}

#[test]
fn bounded_parser_preserves_every_closed_error_classification() {
    let limits = ContractLimits::packet0();
    assert_eq!(
        parse_canonical_json::<BoundedString<8>>(br#""""#, &limits),
        Err(ContractError::Empty)
    );
    assert_eq!(
        parse_canonical_json::<BoundedString<65_537>>(br#""x""#, &limits),
        Err(ContractError::LimitOutOfRange)
    );
    assert_eq!(
        parse_canonical_json::<BoundedString<1>>(br#""xx""#, &limits),
        Err(ContractError::BoundExceeded)
    );
    assert_eq!(
        parse_canonical_json::<SafeU64>(b"-1", &limits),
        Err(ContractError::InvalidNumber)
    );
    assert_eq!(
        parse_canonical_json::<UtcTimestamp>(br#""2024-02-30T00:00:00Z""#, &limits),
        Err(ContractError::InvalidTimestamp)
    );
    assert_eq!(
        parse_canonical_json::<SemVer>(br#""01.2.3""#, &limits),
        Err(ContractError::InvalidSemVer)
    );
    assert_eq!(
        parse_canonical_json::<ExactRef>(br#""UPPER@1.2.3""#, &limits),
        Err(ContractError::InvalidExactRef)
    );
    assert_eq!(
        parse_canonical_json::<Fingerprint>(br#""sha256:invalid""#, &limits),
        Err(ContractError::InvalidFingerprint)
    );

    macro_rules! rejecting_type {
        ($name:ident, $error:expr) => {
            #[derive(Debug)]
            struct $name;

            impl<'de> Deserialize<'de> for $name {
                fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    Err(<D::Error as serde::de::Error>::custom($error))
                }
            }

            impl Serialize for $name {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    serializer.serialize_unit()
                }
            }
        };
    }

    rejecting_type!(RejectInvalidString, ContractError::InvalidString);
    rejecting_type!(RejectInvalidJson, ContractError::InvalidJson);
    rejecting_type!(RejectNonCanonicalJson, ContractError::NonCanonicalJson);
    rejecting_type!(RejectDuplicateMember, ContractError::DuplicateMember);
    rejecting_type!(RejectNonCanonicalOrder, ContractError::NonCanonicalOrder);
    rejecting_type!(RejectInvalidInvariant, ContractError::InvalidInvariant);
    rejecting_type!(
        RejectInvalidArtifactLocator,
        ContractError::InvalidArtifactLocator
    );
    rejecting_type!(
        RejectFingerprintMismatch,
        ContractError::FingerprintMismatch
    );

    assert_eq!(
        parse_canonical_json::<RejectInvalidString>(b"null", &limits).unwrap_err(),
        ContractError::InvalidString
    );
    assert_eq!(
        parse_canonical_json::<RejectInvalidJson>(b"null", &limits).unwrap_err(),
        ContractError::InvalidJson
    );
    assert_eq!(
        parse_canonical_json::<RejectNonCanonicalJson>(b"null", &limits).unwrap_err(),
        ContractError::NonCanonicalJson
    );
    assert_eq!(
        parse_canonical_json::<RejectDuplicateMember>(b"null", &limits).unwrap_err(),
        ContractError::DuplicateMember
    );
    assert_eq!(
        parse_canonical_json::<RejectNonCanonicalOrder>(b"null", &limits).unwrap_err(),
        ContractError::NonCanonicalOrder
    );
    assert_eq!(
        parse_canonical_json::<RejectInvalidInvariant>(b"null", &limits).unwrap_err(),
        ContractError::InvalidInvariant
    );
    assert_eq!(
        parse_canonical_json::<RejectInvalidArtifactLocator>(b"null", &limits).unwrap_err(),
        ContractError::InvalidArtifactLocator
    );
    assert_eq!(
        parse_canonical_json::<RejectFingerprintMismatch>(b"null", &limits).unwrap_err(),
        ContractError::FingerprintMismatch
    );
}

#[test]
fn owned_deserializers_return_only_constant_redacted_errors() {
    let sentinel = "secret-sentinel-value";

    let bounded_error = serde_json::from_str::<BoundedString<1>>(&format!(r#""{sentinel}""#))
        .unwrap_err()
        .to_string();
    assert!(bounded_error.starts_with(&ContractError::BoundExceeded.to_string()));
    assert!(!bounded_error.contains(sentinel));

    let semver_error = serde_json::from_str::<SemVer>(&format!(r#""{sentinel}""#))
        .unwrap_err()
        .to_string();
    assert!(semver_error.starts_with(&ContractError::InvalidSemVer.to_string()));
    assert!(!semver_error.contains(sentinel));

    let binding_json = format!(
        r#"{{"fingerprint":"sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad","ref":"handbook.problem@1.2.3","{sentinel}":null}}"#
    );
    let binding_error = serde_json::from_str::<ExactBinding>(&binding_json)
        .unwrap_err()
        .to_string();
    assert!(binding_error.starts_with(&ContractError::InvalidJson.to_string()));
    assert!(!binding_error.contains(sentinel));

    let oversized_key = format!("{}-{sentinel}", "x".repeat(255));
    let value_error =
        serde_json::from_str::<ContractValue>(&format!(r#"{{"{oversized_key}":null}}"#))
            .unwrap_err()
            .to_string();
    assert!(value_error.starts_with(&ContractError::BoundExceeded.to_string()));
    assert!(!value_error.contains(sentinel));
}

#[test]
fn canonical_capture_rejects_hostile_serializers_before_allocation_or_n_plus_one() {
    struct InflatedSequenceHint;
    impl Serialize for InflatedSequenceHint {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_seq(Some(usize::MAX))?.end()
        }
    }

    struct InflatedMapHint;
    impl Serialize for InflatedMapHint {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_map(Some(usize::MAX))?.end()
        }
    }

    struct PanicIfSerialized;
    impl Serialize for PanicIfSerialized {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            panic!("N+1 value must not be serialized")
        }
    }

    struct NPlusOneSequence;
    impl Serialize for NPlusOneSequence {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut sequence = serializer.serialize_seq(None)?;
            sequence.serialize_element(&0_u8)?;
            sequence.serialize_element(&PanicIfSerialized)?;
            sequence.end()
        }
    }

    struct NPlusOneMap;
    impl Serialize for NPlusOneMap {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = serializer.serialize_map(None)?;
            map.serialize_entry("first", &0_u8)?;
            map.serialize_entry(&PanicIfSerialized, &0_u8)?;
            map.end()
        }
    }

    let limits = ContractLimits::new(64, 16, 1, 1, 4).unwrap();
    for result in [
        std::panic::catch_unwind(|| canonical_json_bytes(&InflatedSequenceHint, &limits)),
        std::panic::catch_unwind(|| canonical_json_bytes(&InflatedMapHint, &limits)),
        std::panic::catch_unwind(|| canonical_json_bytes(&NPlusOneSequence, &limits)),
        std::panic::catch_unwind(|| canonical_json_bytes(&NPlusOneMap, &limits)),
    ] {
        assert!(matches!(result, Ok(Err(ContractError::BoundExceeded))));
    }

    let oversized = "x".repeat(17);
    assert_eq!(
        canonical_json_bytes(&oversized, &limits),
        Err(ContractError::BoundExceeded)
    );
    let document_limited = ContractLimits::new(2, 16, 1, 1, 4).unwrap();
    assert_eq!(
        canonical_json_bytes(&"x", &document_limited),
        Err(ContractError::BoundExceeded)
    );
}

fn bounded<const N: usize>(value: &str) -> BoundedString<N> {
    BoundedString::new(value.to_owned()).unwrap()
}

fn binding(identity: &str) -> ExactBinding {
    ExactBinding::new(
        ExactRef::parse(&format!("{identity}@1.0.0")).unwrap(),
        Fingerprint::parse(
            "sha256:0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap(),
    )
}

fn artifact(identity: &str, path: &str) -> ArtifactRef {
    ArtifactRef::new(
        binding(identity),
        bounded("application/json"),
        SafeU64::new(7).unwrap(),
        Sensitivity::Internal,
        ArtifactLocator::repo_relative(sha256_fingerprint(b"repository"), bounded(path)).unwrap(),
    )
    .unwrap()
}

#[test]
fn artifact_locators_enforce_the_portable_path_and_digest_contract() {
    let repository = sha256_fingerprint(b"repository");
    let locator =
        ArtifactLocator::repo_relative(repository.clone(), bounded("evidence/run-1/result.json"))
            .unwrap();
    assert!(matches!(
        locator.as_ref(),
        ArtifactLocatorRef::RepoRelative {
            repository_identity_fingerprint,
            path,
            no_follow: true,
        } if repository_identity_fingerprint == &repository
            && path.as_str() == "evidence/run-1/result.json"
    ));
    assert!(ArtifactLocator::repo_relative(repository.clone(), bounded(&"a".repeat(4096))).is_ok());

    for path in [
        "/absolute",
        "trailing/",
        "double//slash",
        ".",
        "..",
        "a/../b",
        "back\\slash",
        "C:drive",
        "percent%2e%2e",
        "nonascii-é",
        "segment.",
        "CON",
        "con.txt",
        "AUX.log",
        "COM1",
        "lPt9.bin",
    ] {
        assert_eq!(
            ArtifactLocator::repo_relative(repository.clone(), bounded(path)),
            Err(ContractError::InvalidArtifactLocator),
            "path should be rejected: {path}"
        );
    }
    assert!(ArtifactLocator::repo_relative(repository, bounded("COM0/file")).is_ok());

    let content =
        ArtifactLocator::content_addressed(binding("store.main"), sha256_fingerprint(b"artifact"));
    assert!(matches!(
        content.as_ref(),
        ArtifactLocatorRef::ContentAddressed { algorithm, .. }
            if algorithm == DigestAlgorithm::Sha256
    ));
    assert_eq!(
        ArtifactRef::new(
            binding("artifact.one"),
            bounded("application/json"),
            SafeU64::new(1).unwrap(),
            Sensitivity::Public,
            content,
        ),
        Err(ContractError::FingerprintMismatch)
    );
    assert_eq!(
        ArtifactRef::new(
            binding("artifact.one"),
            bounded("Application/JSON"),
            SafeU64::new(1).unwrap(),
            Sensitivity::Public,
            ArtifactLocator::repo_relative(sha256_fingerprint(b"repo"), bounded("safe/path"))
                .unwrap(),
        ),
        Err(ContractError::InvalidString)
    );

    let zero = binding("artifact.one").fingerprint().as_str().to_owned();
    let false_no_follow = format!(
        r#"{{"kind":"repo_relative","no_follow":false,"path":"safe/path","repository_identity_fingerprint":"{zero}"}}"#
    );
    assert!(serde_json::from_str::<ArtifactLocator>(&false_no_follow).is_err());
    let extra = format!(
        r#"{{"extra":null,"kind":"repo_relative","no_follow":true,"path":"safe/path","repository_identity_fingerprint":"{zero}"}}"#
    );
    assert!(serde_json::from_str::<ArtifactLocator>(&extra).is_err());
}

#[test]
fn dto_fingerprints_use_the_exact_closed_preimages() {
    let limits = ContractLimits::packet0();
    let zero = binding("details.schema").fingerprint().as_str().to_owned();
    let details = ContractValue::object(vec![ContractMember::new(
        bounded("answer"),
        ContractValue::integer(SafeI64::new(7).unwrap()),
    )])
    .unwrap();
    let diagnostic = Diagnostic::new(
        bounded("diagnostic.ready"),
        DiagnosticSeverity::Information,
        binding("subject.item"),
        binding("details.schema"),
        details.clone(),
        None,
        &limits,
    )
    .unwrap();
    let details_preimage = format!(
        r#"{{"details":{{"answer":7}},"details_schema":{{"fingerprint":"{zero}","ref":"details.schema@1.0.0"}}}}"#
    );
    assert_eq!(
        diagnostic.details_fingerprint(),
        &sha256_fingerprint(details_preimage.as_bytes())
    );

    let recheck = RecheckCondition::new(
        binding("condition.schema"),
        ContractValue::boolean(true),
        &limits,
    )
    .unwrap();
    let condition_preimage = format!(
        r#"{{"condition":true,"condition_schema":{{"fingerprint":"{zero}","ref":"condition.schema@1.0.0"}}}}"#
    );
    assert_eq!(
        recheck.condition_fingerprint(),
        &sha256_fingerprint(condition_preimage.as_bytes())
    );

    let problem = Problem::new(
        bounded("problem.one"),
        bounded("precondition.wait"),
        ProblemCategory::Prerequisite,
        binding("subject.item"),
        None,
        binding("details.schema"),
        details,
        BoundedVec::new(Vec::new()).unwrap(),
        RetryDirective::AfterRecheck,
        Some(recheck.clone()),
        None,
        &limits,
    )
    .unwrap();
    let problem_preimage = format!(
        concat!(
            r#"{{"category":"prerequisite","code":"precondition.wait","correlation_ref":null,"#,
            r#""details":{{"answer":7}},"details_fingerprint":"{}","#,
            r#""details_schema":{{"fingerprint":"{}","ref":"details.schema@1.0.0"}},"evidence":[],"#,
            r#""problem_id":"problem.one","recheck":{{"condition":true,"condition_fingerprint":"{}","#,
            r#""condition_schema":{{"fingerprint":"{}","ref":"condition.schema@1.0.0"}}}},"#,
            r#""retry":"after_recheck","rule":null,"subject":{{"fingerprint":"{}","ref":"subject.item@1.0.0"}}}}"#
        ),
        diagnostic.details_fingerprint().as_str(),
        zero,
        recheck.condition_fingerprint().as_str(),
        binding("condition.schema").fingerprint().as_str(),
        binding("subject.item").fingerprint().as_str(),
    );
    assert_eq!(
        problem.problem_fingerprint(),
        &sha256_fingerprint(problem_preimage.as_bytes())
    );

    let receipt = WriteReceipt::new(
        bounded("record.kind"),
        binding("record.one"),
        bounded("authority.primary"),
        ContractValue::boolean(true),
        bounded("atomic.one"),
        &limits,
    )
    .unwrap();
    let receipt_preimage = format!(
        r#"{{"atomic_group":"atomic.one","authority_class":"authority.primary","condition":true,"record":{{"fingerprint":"{zero}","ref":"record.one@1.0.0"}},"record_kind":"record.kind"}}"#
    );
    assert_eq!(
        receipt.receipt_fingerprint(),
        &sha256_fingerprint(receipt_preimage.as_bytes())
    );
}

#[test]
fn dto_null_omission_closed_enum_and_fingerprint_rules_fail_closed() {
    let limits = ContractLimits::packet0();
    let diagnostic = Diagnostic::new(
        bounded("diagnostic.ready"),
        DiagnosticSeverity::Warning,
        binding("subject.item"),
        binding("details.schema"),
        ContractValue::null(),
        None,
        &limits,
    )
    .unwrap();
    let diagnostic_bytes = canonical_json_bytes(&diagnostic, &limits).unwrap();
    assert!(!String::from_utf8_lossy(&diagnostic_bytes).contains("display_message"));
    assert_eq!(
        parse_canonical_json::<Diagnostic>(&diagnostic_bytes, &limits).unwrap(),
        diagnostic
    );

    let tampered = String::from_utf8(diagnostic_bytes).unwrap().replace(
        diagnostic.details_fingerprint().as_str(),
        "sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    );
    assert_eq!(
        parse_canonical_json::<Diagnostic>(tampered.as_bytes(), &limits),
        Err(ContractError::FingerprintMismatch)
    );
    let unknown_severity = tampered.replace("warning", "fatal");
    assert_eq!(
        parse_canonical_json::<Diagnostic>(unknown_severity.as_bytes(), &limits),
        Err(ContractError::InvalidJson)
    );

    let recheck = RecheckCondition::new(
        binding("condition.schema"),
        ContractValue::boolean(true),
        &limits,
    )
    .unwrap();
    let problem = Problem::new(
        bounded("problem.one"),
        bounded("prerequisite.wait"),
        ProblemCategory::Prerequisite,
        binding("subject.item"),
        None,
        binding("details.schema"),
        ContractValue::null(),
        BoundedVec::new(Vec::new()).unwrap(),
        RetryDirective::AfterRecheck,
        Some(recheck),
        None,
        &limits,
    )
    .unwrap();
    let problem_json = String::from_utf8(canonical_json_bytes(&problem, &limits).unwrap()).unwrap();
    assert!(problem_json.contains(r#""correlation_ref":null"#));
    assert!(problem_json.contains(r#""rule":null"#));
    let missing_rule = problem_json.replace(r#""rule":null,"#, "");
    assert!(!missing_rule.contains(r#""rule""#));
    assert!(parse_canonical_json::<ContractValue>(missing_rule.as_bytes(), &limits).is_ok());
    assert_eq!(
        parse_canonical_json::<Problem>(missing_rule.as_bytes(), &limits),
        Err(ContractError::InvalidJson)
    );
    assert_eq!(
        Problem::new(
            bounded("problem.two"),
            bounded("schema.invalid"),
            ProblemCategory::Schema,
            binding("subject.item"),
            None,
            binding("details.schema"),
            ContractValue::null(),
            BoundedVec::new(Vec::new()).unwrap(),
            RetryDirective::AfterRequestChange,
            None,
            None,
            &limits,
        ),
        Err(ContractError::InvalidInvariant)
    );
    assert_eq!(
        ProblemBinding::new(bounded("Uppercase"), sha256_fingerprint(b"problem")),
        Err(ContractError::InvalidString)
    );
}

#[test]
fn dto_accessors_and_collection_order_cover_the_remaining_shared_types() {
    let limits = ContractLimits::packet0();
    let source = SourceBinding::new(
        binding("source.one"),
        binding("revision.one"),
        binding("adapter.one"),
    )
    .unwrap();
    let omission = Omission::new(
        binding("item.one"),
        OmissionReason::Unavailable,
        ProofEffect::Partial,
        None,
        SafeU64::new(3).unwrap(),
    )
    .unwrap();
    assert!(omission.source().is_none());
    assert!(
        String::from_utf8(canonical_json_bytes(&omission, &limits).unwrap())
            .unwrap()
            .contains(r#""source":null"#)
    );
    assert_eq!(source.adapter().reference().identity(), "adapter.one");

    let manifest = SchemaManifestEntry::new(
        binding("schema.one"),
        DraftVersion::Draft202012,
        SafeU64::new(123).unwrap(),
        SchemaMediaType::ApplicationSchemaJson,
    )
    .unwrap();
    assert_eq!(manifest.draft(), DraftVersion::Draft202012);
    assert_eq!(
        serde_json::to_string(&manifest.media_type()).unwrap(),
        r#""application/schema+json""#
    );

    let action = NextAction::new(
        bounded("action.recheck"),
        NextActionKind::Recheck,
        binding("subject.item"),
        binding("parameter.schema"),
        ContractValue::null(),
        None,
        &limits,
    )
    .unwrap();
    assert!(action.display_label().is_none());
    assert!(
        !String::from_utf8(canonical_json_bytes(&action, &limits).unwrap())
            .unwrap()
            .contains("display_label")
    );

    let correlation = CorrelationRef::new(bounded("corr_abcdefghijklmnop")).unwrap();
    assert_eq!(correlation.value().as_str(), "corr_abcdefghijklmnop");
    assert_eq!(
        CorrelationRef::new(bounded("corr_too-short")),
        Err(ContractError::InvalidString)
    );

    let first = artifact("artifact.a", "a");
    let second = artifact("artifact.b", "b");
    let error_problem = |evidence| {
        Problem::new(
            bounded("problem.error"),
            bounded("adapter.failed"),
            ProblemCategory::Adapter,
            binding("subject.item"),
            None,
            binding("details.schema"),
            ContractValue::null(),
            evidence,
            RetryDirective::Transient,
            None,
            Some(correlation.clone()),
            &limits,
        )
    };
    assert!(error_problem(BoundedVec::new(vec![first.clone(), second.clone()]).unwrap()).is_ok());
    assert_eq!(
        error_problem(BoundedVec::new(vec![second.clone(), first.clone()]).unwrap()),
        Err(ContractError::NonCanonicalOrder)
    );
    assert_eq!(
        error_problem(BoundedVec::new(vec![first.clone(), first]).unwrap()),
        Err(ContractError::DuplicateMember)
    );
}

#[test]
fn catalog_values_enforce_local_closure_and_exact_fingerprints() {
    let limits = ContractLimits::packet0();
    let root = CatalogRoot::new(
        binding("catalog.root"),
        binding("entry.schema"),
        SafeU64::new(2).unwrap(),
        binding("sort.definition"),
        UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
    )
    .unwrap();
    assert_eq!(root.total_entries().get(), 2);
    assert_eq!(root.entry_schema().reference().identity(), "entry.schema");
    assert_eq!(
        root.canonical_sort().reference().identity(),
        "sort.definition"
    );
    assert_eq!(root.retention_until_utc().as_str(), "2030-01-01T00:00:00Z");

    let cursor = CatalogCursor::new(
        root.catalog().clone(),
        ContractValue::string(bounded("next")),
        &limits,
    )
    .unwrap();
    let cursor_preimage = format!(
        r#"{{"catalog":{{"fingerprint":"{}","ref":"catalog.root@1.0.0"}},"next_sort_key":"next"}}"#,
        root.catalog().fingerprint().as_str(),
    );
    assert_eq!(
        cursor.cursor_fingerprint(),
        &sha256_fingerprint(cursor_preimage.as_bytes())
    );

    let entries = BoundedVec::new(vec![
        ContractValue::integer(SafeI64::new(1).unwrap()),
        ContractValue::integer(SafeI64::new(2).unwrap()),
    ])
    .unwrap();
    let page = CatalogPage::new(root.clone(), entries, Some(cursor.clone()), &limits).unwrap();
    assert_eq!(page.root(), &root);
    assert_eq!(page.entries().as_slice().len(), 2);
    assert_eq!(page.next_cursor(), Some(&cursor));

    let page_json = canonical_json_bytes(&page, &limits).unwrap();
    let page_preimage = String::from_utf8(page_json.clone()).unwrap().replace(
        &format!(
            r#","page_fingerprint":"{}""#,
            page.page_fingerprint().as_str()
        ),
        "",
    );
    assert_eq!(
        page.page_fingerprint(),
        &sha256_fingerprint(page_preimage.as_bytes())
    );
    assert_eq!(
        parse_canonical_json::<CatalogPage>(&page_json, &limits).unwrap(),
        page
    );

    let other_cursor =
        CatalogCursor::new(binding("catalog.other"), ContractValue::null(), &limits).unwrap();
    assert_eq!(
        CatalogPage::new(
            root.clone(),
            BoundedVec::new(Vec::new()).unwrap(),
            Some(other_cursor),
            &limits,
        ),
        Err(ContractError::InvalidInvariant)
    );
    assert_eq!(
        CatalogPage::new(
            root,
            BoundedVec::new(vec![ContractValue::null(), ContractValue::null()]).unwrap(),
            None,
            &limits,
        ),
        Err(ContractError::DuplicateMember)
    );
}

fn terminal_problem(
    problem_id: &str,
    code: &str,
    category: ProblemCategory,
    limits: &ContractLimits,
) -> Problem {
    let (rule, retry, recheck, correlation_ref) = match category {
        ProblemCategory::Prerequisite => (
            None,
            RetryDirective::AfterRecheck,
            Some(
                RecheckCondition::new(
                    binding("condition.schema"),
                    ContractValue::boolean(true),
                    limits,
                )
                .unwrap(),
            ),
            None,
        ),
        ProblemCategory::Implementation | ProblemCategory::Adapter => (
            None,
            RetryDirective::Transient,
            None,
            Some(CorrelationRef::new(bounded("corr_abcdefghijklmnop")).unwrap()),
        ),
        _ => (Some(binding("rule.one")), RetryDirective::Never, None, None),
    };
    Problem::new(
        bounded(problem_id),
        bounded(code),
        category,
        binding("subject.item"),
        rule,
        binding("details.schema"),
        ContractValue::null(),
        BoundedVec::new(Vec::new()).unwrap(),
        retry,
        recheck,
        correlation_ref,
        limits,
    )
    .unwrap()
}

#[test]
fn terminal_fields_enforce_the_four_way_truth_table_and_problem_order() {
    let limits = ContractLimits::packet0();
    let empty = || BoundedVec::new(Vec::new()).unwrap();
    let ok = TerminalFields::new(
        TerminalStatus::Ok,
        Some(ContractValue::boolean(true)),
        empty(),
        empty(),
        empty(),
    )
    .unwrap();
    assert_eq!(ok.status(), TerminalStatus::Ok);
    assert!(ok.data().is_some());
    assert!(ok.blockers().is_empty());

    let blocker = terminal_problem(
        "problem.blocked",
        "prerequisite.wait",
        ProblemCategory::Prerequisite,
        &limits,
    );
    let blocked = TerminalFields::<ContractValue>::new(
        TerminalStatus::Blocked,
        None,
        BoundedVec::new(vec![blocker.clone()]).unwrap(),
        empty(),
        empty(),
    )
    .unwrap();
    let blocked_json = canonical_json_bytes(&blocked, &limits).unwrap();
    let blocked_text = String::from_utf8(blocked_json.clone()).unwrap();
    assert!(blocked_text.contains(r#""data":null"#));
    assert!(blocked_text.contains(r#""errors":[]"#));
    assert_eq!(
        parse_canonical_json::<TerminalFields<ContractValue>>(&blocked_json, &limits).unwrap(),
        blocked
    );

    assert_eq!(
        TerminalFields::new(
            TerminalStatus::Ok,
            None::<ContractValue>,
            empty(),
            empty(),
            empty(),
        ),
        Err(ContractError::InvalidInvariant)
    );
    assert_eq!(
        TerminalFields::<ContractValue>::new(
            TerminalStatus::Refused,
            None,
            BoundedVec::new(vec![blocker]).unwrap(),
            empty(),
            empty(),
        ),
        Err(ContractError::InvalidInvariant)
    );

    let first = terminal_problem("problem.a", "a.code", ProblemCategory::Schema, &limits);
    let second = terminal_problem("problem.b", "b.code", ProblemCategory::Schema, &limits);
    assert_eq!(
        TerminalFields::<ContractValue>::new(
            TerminalStatus::Refused,
            None,
            empty(),
            BoundedVec::new(vec![second.clone(), first.clone()]).unwrap(),
            empty(),
        ),
        Err(ContractError::NonCanonicalOrder)
    );
    assert_eq!(
        TerminalFields::<ContractValue>::new(
            TerminalStatus::Refused,
            None,
            empty(),
            BoundedVec::new(vec![first.clone(), first]).unwrap(),
            empty(),
        ),
        Err(ContractError::DuplicateMember)
    );
    assert!(TerminalFields::<ContractValue>::new(
        TerminalStatus::Refused,
        None,
        empty(),
        BoundedVec::new(vec![second]).unwrap(),
        empty(),
    )
    .is_ok());
}

#[test]
fn idempotency_state_wire_forms_are_closed_and_never_carry_raw_keys() {
    let limits = ContractLimits::packet0();
    let not_applicable = IdempotencyState::not_applicable().unwrap();
    assert_eq!(
        canonical_json_bytes(&not_applicable, &limits).unwrap(),
        br#"{"state":"not_applicable"}"#
    );

    let problem = terminal_problem(
        "problem.schema",
        "schema.invalid",
        ProblemCategory::Schema,
        &limits,
    );
    let problem_binding = ProblemBinding::new(
        problem.problem_id().clone(),
        problem.problem_fingerprint().clone(),
    )
    .unwrap();
    let not_established = IdempotencyState::not_established(
        IdempotencyStage::RequestValidation,
        problem_binding.clone(),
    )
    .unwrap();
    assert!(matches!(
        not_established.as_ref(),
        IdempotencyStateRef::NotEstablished {
            stage: IdempotencyStage::RequestValidation,
            terminal_problem,
        } if terminal_problem == &problem_binding
    ));

    let key_fingerprint = sha256_fingerprint(b"scoped-key");
    let request_fingerprint = sha256_fingerprint(b"request");
    let original_fingerprint = sha256_fingerprint(b"original-result");
    let retention = UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap();
    assert_eq!(
        IdempotencyState::established(
            key_fingerprint.clone(),
            request_fingerprint.clone(),
            false,
            Some(original_fingerprint.clone()),
            retention.clone(),
        ),
        Err(ContractError::InvalidInvariant)
    );
    let replay = IdempotencyState::established(
        key_fingerprint,
        request_fingerprint,
        true,
        Some(original_fingerprint.clone()),
        retention,
    )
    .unwrap();
    assert!(matches!(
        replay.as_ref(),
        IdempotencyStateRef::Established {
            replayed: true,
            original_result_fingerprint: Some(value),
            ..
        } if value == &original_fingerprint
    ));
    let replay_bytes = canonical_json_bytes(&replay, &limits).unwrap();
    let replay_text = String::from_utf8(replay_bytes.clone()).unwrap();
    assert!(!replay_text.contains("owner-secret"));
    assert_eq!(
        parse_canonical_json::<IdempotencyState>(&replay_bytes, &limits).unwrap(),
        replay
    );
    assert_eq!(
        parse_canonical_json::<IdempotencyState>(
            br#"{"extra":null,"state":"not_applicable"}"#,
            &limits,
        ),
        Err(ContractError::InvalidJson)
    );
}

#[test]
fn raw_idempotency_keys_are_bounded_redacted_and_accessor_free() {
    assert_eq!(
        RawIdempotencyKey::new(b"key".to_vec(), 0)
            .unwrap_err()
            .to_string(),
        ContractError::LimitOutOfRange.to_string()
    );
    assert!(matches!(
        RawIdempotencyKey::new(Vec::new(), 8),
        Err(ContractError::Empty)
    ));
    assert!(matches!(
        RawIdempotencyKey::new(b"ninebytes".to_vec(), 8),
        Err(ContractError::BoundExceeded)
    ));
    assert!(matches!(
        RawIdempotencyKey::new(vec![0xff], 8),
        Err(ContractError::InvalidString)
    ));
    assert!(matches!(
        RawIdempotencyKey::new(b"line\nbreak".to_vec(), 32),
        Err(ContractError::InvalidString)
    ));

    let key = RawIdempotencyKey::new(b"owner-secret".to_vec(), 64).unwrap();
    assert_eq!(format!("{key:?}"), "[REDACTED raw idempotency key]");
    assert_eq!(format!("{key}"), "[REDACTED raw idempotency key]");
    assert!(!format!("{key:?}").contains("owner-secret"));
    assert_eq!(
        DefinitionPinnedKeyPointer::BodyIdempotencyKey.to_string(),
        "/body/idempotency_key"
    );
}

#[test]
fn p0c_public_accessors_and_error_surfaces_have_the_frozen_types() {
    let _: fn(&CatalogPage) -> Option<&CatalogCursor> = CatalogPage::next_cursor;
    let _: fn(&TerminalFields<ContractValue>) -> Option<&ContractValue> =
        TerminalFields::<ContractValue>::data;
    let _: fn(&Problem) -> Option<&ExactBinding> = Problem::rule;
    let _: fn(&Problem) -> Option<&RecheckCondition> = Problem::recheck;
    let _: fn(&Problem) -> Option<&CorrelationRef> = Problem::correlation_ref;
    let _: fn(&Diagnostic) -> Option<&BoundedString<1024>> = Diagnostic::display_message;
    let _: fn(&NextAction) -> Option<&BoundedString<1024>> = NextAction::display_label;
    let _: fn(&Omission) -> Option<&SourceBinding> = Omission::source;
    let _: fn(&dyn ContractWitness) = |_| {};
    let _: fn(ExactBinding, &dyn ContractWitness, &ContractLimits) -> _ =
        handbook_sdk::contract_membrane::verify::<ExactBinding>;

    fn assert_verify_contract<T: VerifyContract>() {}
    assert_verify_contract::<Problem>();
    assert_verify_contract::<CatalogPage>();
    assert_verify_contract::<TerminalFields<ContractValue>>();
    assert_verify_contract::<IdempotencyState>();

    let errors = [
        VerificationError::WitnessUnavailable,
        VerificationError::BindingRejected,
        VerificationError::SchemaRejected,
        VerificationError::ArtifactRejected,
        VerificationError::CatalogRejected,
        VerificationError::DerivedFingerprintMismatch,
        VerificationError::ContractInvariant,
    ];
    for error in errors {
        assert!(!error.to_string().contains("secret-input"));
    }
}
