use handbook_contracts::{
    ContractIdV1, DefinitionChangeV1, ExactBindingV1, ExactRefV1, FingerprintV1,
    MinimumVersionChangeV1, VersionV1,
};

#[test]
fn exact_identity_requires_lowercase_dot_separated_id_and_full_semver() {
    let id = ContractIdV1::parse("handbook.contract.example").unwrap();
    let version = VersionV1::parse("1.2.3-alpha.1+build.7").unwrap();
    let exact = ExactRefV1::derive(&id, &version);

    assert_eq!(
        exact.as_str(),
        "handbook.contract.example@1.2.3-alpha.1+build.7"
    );
    for invalid in [
        "contract",
        "Handbook.contract",
        "handbook..contract",
        "handbook.contract_legacy",
        "handbook.contract-",
    ] {
        assert!(ContractIdV1::parse(invalid).is_err(), "accepted {invalid}");
    }
    for invalid in ["1", "1.0", "01.0.0", "1.0.0-01", "latest", "^1.0.0"] {
        assert!(VersionV1::parse(invalid).is_err(), "accepted {invalid}");
    }
}

#[test]
fn exact_refs_and_fingerprints_refuse_noncanonical_values() {
    assert!(ExactRefV1::parse("handbook.contract.example@1.0.0").is_ok());
    assert!(ExactRefV1::parse("handbook.contract.example@latest").is_err());
    assert!(ExactRefV1::parse("handbook.contract.example@^1.0.0").is_err());
    assert!(FingerprintV1::parse(&format!("sha256:{}", "a".repeat(64))).is_ok());
    assert!(FingerprintV1::parse(&format!("sha256:{}", "A".repeat(64))).is_err());
    assert!(FingerprintV1::parse(&"a".repeat(64)).is_err());
}

#[test]
fn exact_binding_matches_both_ref_and_fingerprint() {
    let expected = ExactBindingV1::new(
        ExactRefV1::parse("handbook.policy.example@1.0.0").unwrap(),
        FingerprintV1::parse(&format!("sha256:{}", "1".repeat(64))).unwrap(),
    );
    let substituted = ExactBindingV1::new(
        expected.reference().clone(),
        FingerprintV1::parse(&format!("sha256:{}", "2".repeat(64))).unwrap(),
    );

    assert!(expected.matches(&expected));
    assert!(!expected.matches(&substituted));
}

#[test]
fn mutation_matrix_is_descriptive_and_requires_minimum_version_bumps() {
    assert_eq!(
        DefinitionChangeV1::PresentationOnlyAnnotation.minimum_version_change(),
        MinimumVersionChangeV1::Patch
    );
    assert_eq!(
        DefinitionChangeV1::AdvisoryUnweightedClaimAddition.minimum_version_change(),
        MinimumVersionChangeV1::Minor
    );
    assert_eq!(
        DefinitionChangeV1::MatcherOrApplicabilityChange.minimum_version_change(),
        MinimumVersionChangeV1::Major
    );

    let v1 = VersionV1::parse("1.2.3").unwrap();
    assert!(MinimumVersionChangeV1::Patch.is_satisfied_by(&v1, &VersionV1::parse("1.2.4").unwrap()));
    assert!(MinimumVersionChangeV1::Minor.is_satisfied_by(&v1, &VersionV1::parse("1.3.0").unwrap()));
    assert!(
        !MinimumVersionChangeV1::Minor.is_satisfied_by(&v1, &VersionV1::parse("1.2.4").unwrap())
    );
    assert!(MinimumVersionChangeV1::Major.is_satisfied_by(&v1, &VersionV1::parse("2.0.0").unwrap()));
    assert!(
        !MinimumVersionChangeV1::Major.is_satisfied_by(&v1, &VersionV1::parse("1.99.0").unwrap())
    );
}
