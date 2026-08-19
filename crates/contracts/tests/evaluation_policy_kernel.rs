use handbook_contracts::{
    evidence_admission_policy_binding_v1, evidence_admission_policy_canonical_json_v1,
    evidence_stability_policy_binding_v1, evidence_stability_policy_canonical_json_v1,
    local_closeout_policy_binding_v1, local_closeout_policy_canonical_json_v1, ContractError,
    ContractIdV1, EvaluationPolicyKindV1, EvaluationPolicyRegistryV1, EvidenceAdmissionPolicyV1,
    EvidenceFreshnessPolicyInputV1, EvidenceFreshnessPolicyV1, EvidenceStabilityPolicyV1,
    ExactRationalThresholdV1, GatePolicyAlgorithmV1, GatePolicyInputV1, GatePolicyV1,
    LocalCloseoutPolicyV1, VersionV1, EVIDENCE_ADMISSION_POLICY_REF_V1,
    EVIDENCE_ADMISSION_POLICY_SCHEMA_REF_V1, EVIDENCE_FRESHNESS_POLICY_SCHEMA_REF_V1,
    EVIDENCE_STABILITY_POLICY_REF_V1, EVIDENCE_STABILITY_POLICY_SCHEMA_REF_V1,
    GATE_POLICY_SCHEMA_REF_V1, LOCAL_CLOSEOUT_POLICY_REF_V1, LOCAL_CLOSEOUT_POLICY_SCHEMA_REF_V1,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

const JSON_SAFE_INTEGER_MAX: u64 = 9_007_199_254_740_991;

#[test]
fn exact_schema_and_fixed_policy_refs_are_stable() {
    assert_eq!(
        EVIDENCE_ADMISSION_POLICY_SCHEMA_REF_V1,
        "handbook.contract-evidence-admission-policy@1.0.0"
    );
    assert_eq!(
        EVIDENCE_FRESHNESS_POLICY_SCHEMA_REF_V1,
        "handbook.contract-evidence-freshness-policy@1.0.0"
    );
    assert_eq!(
        EVIDENCE_STABILITY_POLICY_SCHEMA_REF_V1,
        "handbook.contract-evidence-stability-policy@1.0.0"
    );
    assert_eq!(
        GATE_POLICY_SCHEMA_REF_V1,
        "handbook.contract-gate-policy@1.0.0"
    );
    assert_eq!(
        LOCAL_CLOSEOUT_POLICY_SCHEMA_REF_V1,
        "handbook.contract-local-closeout-policy@1.0.0"
    );
    assert_eq!(
        EVIDENCE_ADMISSION_POLICY_REF_V1,
        "handbook.contract-evidence-admission-policy.closed@1.0.0"
    );
    assert_eq!(
        EVIDENCE_STABILITY_POLICY_REF_V1,
        "handbook.contract-evidence-stability-policy.outcome-consensus@1.0.0"
    );
    assert_eq!(
        LOCAL_CLOSEOUT_POLICY_REF_V1,
        "handbook.contract-local-closeout-policy.gate-passed@1.0.0"
    );
}

#[test]
fn fixed_policy_sources_are_rfc8785_canonical_and_content_addressed() {
    for (source, binding) in [
        (
            evidence_admission_policy_canonical_json_v1(),
            evidence_admission_policy_binding_v1().unwrap(),
        ),
        (
            evidence_stability_policy_canonical_json_v1(),
            evidence_stability_policy_binding_v1().unwrap(),
        ),
        (
            local_closeout_policy_canonical_json_v1(),
            local_closeout_policy_binding_v1().unwrap(),
        ),
    ] {
        assert!(!source.ends_with('\n'));
        let value: Value = serde_json::from_str(source).unwrap();
        assert_eq!(serde_json_canonicalizer::to_string(&value).unwrap(), source);
        assert_eq!(binding.fingerprint().as_str(), fingerprint(source));
        assert_eq!(value["policy_ref"], binding.reference().as_str());
    }
}

#[test]
fn fixed_policy_types_reject_changed_same_ref_semantics() {
    let mut admission: Value =
        serde_json::from_str(evidence_admission_policy_canonical_json_v1()).unwrap();
    admission["require_exact_source_revision"] = Value::Bool(false);
    assert!(serde_json::from_value::<EvidenceAdmissionPolicyV1>(admission).is_err());

    let mut stability: Value =
        serde_json::from_str(evidence_stability_policy_canonical_json_v1()).unwrap();
    stability["conflicting_outcome"] = Value::String("pass".to_owned());
    assert!(serde_json::from_value::<EvidenceStabilityPolicyV1>(stability).is_err());

    let mut closeout: Value =
        serde_json::from_str(local_closeout_policy_canonical_json_v1()).unwrap();
    closeout["require_current_lifecycle_head"] = Value::Bool(false);
    assert!(serde_json::from_value::<LocalCloseoutPolicyV1>(closeout).is_err());
}

#[test]
fn freshness_policy_requires_exact_revision_zero_future_skew_and_json_safe_age() {
    let policy = freshness_policy("1.2.3-alpha.1+build.7", 86_400);
    assert_eq!(policy.policy_version().as_str(), "1.2.3-alpha.1+build.7");
    assert_eq!(
        policy.policy_ref().as_str(),
        "handbook.contract-evidence-freshness-policy.fixture@1.2.3-alpha.1+build.7"
    );
    assert!(policy.require_exact_source_revision());
    assert_eq!(policy.future_skew_seconds(), 0);
    assert_eq!(policy.max_observation_age_seconds(), 86_400);

    assert_eq!(
        freshness_input("1.0.0", 0, true, 0).unwrap_err(),
        ContractError::InvalidInvariant
    );
    assert_eq!(
        freshness_input("1.0.0", JSON_SAFE_INTEGER_MAX + 1, true, 0).unwrap_err(),
        ContractError::BoundExceeded
    );
    assert_eq!(
        freshness_input("1.0.0", 1, false, 0).unwrap_err(),
        ContractError::InvalidInvariant
    );
    assert_eq!(
        freshness_input("1.0.0", 1, true, 1).unwrap_err(),
        ContractError::InvalidInvariant
    );
}

#[test]
fn policy_deserialization_refuses_unknown_fields_and_derived_substitution() {
    let freshness = freshness_policy("1.0.0", 60);
    let bytes = freshness.canonical_json_bytes().unwrap();
    let mut unknown: Value = serde_json::from_slice(&bytes).unwrap();
    unknown["unexpected"] = Value::Bool(true);
    assert!(serde_json::from_value::<EvidenceFreshnessPolicyV1>(unknown).is_err());

    let mut changed_ref: Value = serde_json::from_slice(&bytes).unwrap();
    changed_ref["policy_ref"] =
        Value::String("handbook.contract-evidence-freshness-policy.fixture@1.0.1".to_owned());
    assert!(serde_json::from_value::<EvidenceFreshnessPolicyV1>(changed_ref).is_err());

    let mut changed_fingerprint: Value = serde_json::from_slice(&bytes).unwrap();
    changed_fingerprint["policy_fingerprint"] = Value::String(format!("sha256:{}", "a".repeat(64)));
    assert!(serde_json::from_value::<EvidenceFreshnessPolicyV1>(changed_fingerprint).is_err());
}

#[test]
fn exact_rational_threshold_is_positive_bounded_and_normalized() {
    let threshold = ExactRationalThresholdV1::new(2, 3).unwrap();
    assert_eq!(threshold.numerator(), 2);
    assert_eq!(threshold.denominator(), 3);
    assert_eq!(
        ExactRationalThresholdV1::new(0, 1),
        Err(ContractError::InvalidInvariant)
    );
    assert_eq!(
        ExactRationalThresholdV1::new(2, 1),
        Err(ContractError::InvalidInvariant)
    );
    assert_eq!(
        ExactRationalThresholdV1::new(2, 4),
        Err(ContractError::InvalidInvariant)
    );
    assert_eq!(
        ExactRationalThresholdV1::new(1, JSON_SAFE_INTEGER_MAX + 1),
        Err(ContractError::BoundExceeded)
    );
}

#[test]
fn gate_policy_binds_closed_accounting_threshold_and_exact_local_policy() {
    let local = local_closeout_policy_binding_v1().unwrap();
    let input = GatePolicyInputV1::new(
        ContractIdV1::parse("handbook.contract-gate-policy.fixture").unwrap(),
        VersionV1::parse("2.0.0-rc.1+exact").unwrap(),
        GatePolicyAlgorithmV1::ClosedAccounting,
        Some(ExactRationalThresholdV1::new(2, 3).unwrap()),
        local.clone(),
    )
    .unwrap();
    let policy = GatePolicyV1::new(input).unwrap();
    assert_eq!(policy.algorithm(), GatePolicyAlgorithmV1::ClosedAccounting);
    assert_eq!(policy.score_threshold().unwrap().numerator(), 2);
    assert_eq!(policy.local_closeout_policy(), &local);

    let wrong_local = handbook_contracts::ExactBindingV1::new(
        handbook_contracts::ExactRefV1::parse(LOCAL_CLOSEOUT_POLICY_REF_V1).unwrap(),
        handbook_contracts::FingerprintV1::parse(&format!("sha256:{}", "b".repeat(64))).unwrap(),
    );
    assert_eq!(
        GatePolicyInputV1::new(
            ContractIdV1::parse("handbook.contract-gate-policy.fixture").unwrap(),
            VersionV1::parse("1.0.0").unwrap(),
            GatePolicyAlgorithmV1::ClosedAccounting,
            None,
            wrong_local,
        )
        .unwrap_err(),
        ContractError::InvalidInvariant
    );
}

#[test]
fn explicit_registry_has_no_freshness_or_gate_default_and_resolves_only_exact_bytes() {
    let mut registry = EvaluationPolicyRegistryV1::new().unwrap();
    assert_eq!(registry.len(), 3);

    let freshness = freshness_policy("1.0.0", 60);
    assert!(registry.resolve_exact(freshness.binding()).is_err());
    registry.register_freshness(freshness.clone()).unwrap();
    let registered = registry.resolve_exact(freshness.binding()).unwrap();
    assert_eq!(registered.kind(), EvaluationPolicyKindV1::Freshness);
    assert_eq!(
        registered.canonical_json(),
        freshness.canonical_json_bytes().unwrap()
    );

    let local = local_closeout_policy_binding_v1().unwrap();
    let gate = GatePolicyV1::new(
        GatePolicyInputV1::new(
            ContractIdV1::parse("handbook.contract-gate-policy.fixture").unwrap(),
            VersionV1::parse("1.0.0").unwrap(),
            GatePolicyAlgorithmV1::ClosedAccounting,
            None,
            local,
        )
        .unwrap(),
    )
    .unwrap();
    assert!(registry.resolve_exact(gate.binding()).is_err());
    registry.register_gate(gate.clone()).unwrap();
    assert_eq!(
        registry.resolve_exact(gate.binding()).unwrap().kind(),
        EvaluationPolicyKindV1::Gate
    );

    let substituted = handbook_contracts::ExactBindingV1::new(
        freshness.policy_ref().clone(),
        handbook_contracts::FingerprintV1::parse(&format!("sha256:{}", "c".repeat(64))).unwrap(),
    );
    assert!(registry.resolve_exact(&substituted).is_err());
}

fn freshness_policy(version: &str, max_age: u64) -> EvidenceFreshnessPolicyV1 {
    EvidenceFreshnessPolicyV1::new(freshness_input(version, max_age, true, 0).unwrap()).unwrap()
}

fn freshness_input(
    version: &str,
    max_age: u64,
    require_exact_source_revision: bool,
    future_skew_seconds: u64,
) -> Result<EvidenceFreshnessPolicyInputV1, ContractError> {
    EvidenceFreshnessPolicyInputV1::new(
        ContractIdV1::parse("handbook.contract-evidence-freshness-policy.fixture").unwrap(),
        VersionV1::parse(version).unwrap(),
        max_age,
        require_exact_source_revision,
        future_skew_seconds,
    )
}

fn fingerprint(source: &str) -> String {
    format!("sha256:{:x}", Sha256::digest(source.as_bytes()))
}
