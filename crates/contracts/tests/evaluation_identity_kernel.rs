use handbook_contracts::{
    ContentAddressedBindingV1, ContractError, EvaluationCaseV1, EvaluationSubjectV1, FingerprintV1,
    MatcherInputValueV1, ResolutionEnvelopeV1, SemanticTimeV1, StableIdV1,
    CLAIM_VERDICT_SCHEMA_REF_V1, EVIDENCE_CANDIDATE_SCHEMA_REF_V1,
    EVIDENCE_OPERATION_DETAILS_SCHEMA_REF_V1, EVIDENCE_RECORD_SCHEMA_REF_V1,
    GATE_RESULT_SCHEMA_REF_V1, LOCAL_CLOSEOUT_RESULT_SCHEMA_REF_V1,
};

fn fingerprint(byte: u8) -> FingerprintV1 {
    FingerprintV1::parse(&format!("sha256:{}", format!("{byte:02x}").repeat(32))).unwrap()
}

#[test]
fn exact_root_schema_refs_are_fixed() {
    assert_eq!(
        EVIDENCE_CANDIDATE_SCHEMA_REF_V1,
        "handbook.contract-evidence-candidate@1.0.0"
    );
    assert_eq!(
        EVIDENCE_RECORD_SCHEMA_REF_V1,
        "handbook.contract-evidence-record@1.0.0"
    );
    assert_eq!(
        CLAIM_VERDICT_SCHEMA_REF_V1,
        "handbook.contract-claim-verdict@1.0.0"
    );
    assert_eq!(
        GATE_RESULT_SCHEMA_REF_V1,
        "handbook.contract-gate-result@1.0.0"
    );
    assert_eq!(
        LOCAL_CLOSEOUT_RESULT_SCHEMA_REF_V1,
        "handbook.contract-local-closeout-result@1.0.0"
    );
    assert_eq!(
        EVIDENCE_OPERATION_DETAILS_SCHEMA_REF_V1,
        "handbook.contract-evidence-operation-details@1.0.0"
    );
}

#[test]
fn semantic_time_and_content_addressed_bindings_are_closed() {
    let time = SemanticTimeV1::parse("2026-08-19T12:34:56Z").unwrap();
    assert_eq!(time.as_str(), "2026-08-19T12:34:56Z");
    assert_eq!(
        SemanticTimeV1::parse("2026-02-30T12:34:56Z"),
        Err(ContractError::InvalidInvariant)
    );

    let digest = fingerprint(0x11);
    let binding = ContentAddressedBindingV1::new(
        &format!("producer.example#candidate/{}", &digest.as_str()[7..]),
        digest.clone(),
    )
    .unwrap();
    assert_eq!(binding.fingerprint(), &digest);
    let independently_owned =
        ContentAddressedBindingV1::new("producer.example#observation/exact", digest).unwrap();
    assert_eq!(
        independently_owned.reference(),
        "producer.example#observation/exact"
    );
}

#[test]
fn subjects_cases_and_resolution_normalize_matcher_inputs() {
    let binding = ContentAddressedBindingV1::new(
        &format!("subject.example/{}", &fingerprint(0x22).as_str()[7..]),
        fingerprint(0x22),
    )
    .unwrap();
    let a = MatcherInputValueV1::new(StableIdV1::parse("field.a").unwrap(), "alpha").unwrap();
    let b = MatcherInputValueV1::new(StableIdV1::parse("field.b").unwrap(), "beta").unwrap();
    let subject = EvaluationSubjectV1::new(binding.clone(), vec![b.clone(), a.clone()]).unwrap();
    assert_eq!(subject.matcher_inputs()[0].field().as_str(), "field.a");
    assert!(EvaluationSubjectV1::new(binding.clone(), vec![a.clone(), a]).is_err());

    let case =
        EvaluationCaseV1::new(StableIdV1::parse("case.one").unwrap(), binding, vec![b]).unwrap();
    assert_eq!(case.case_id().as_str(), "case.one");

    let level = || StableIdV1::parse("resolution.local").unwrap();
    let stack_ref =
        handbook_contracts::ExactRefV1::parse("handbook.resolution.stack@1.0.0").unwrap();
    let stack = handbook_contracts::ExactBindingV1::new(stack_ref, fingerprint(0x33));
    let envelope =
        ResolutionEnvelopeV1::new(stack, level(), level(), level(), level(), level(), level());
    assert_eq!(envelope.scope_horizon().as_str(), "resolution.local");
}

#[test]
fn closed_enums_reject_unknown_variants_and_promotion_fields() {
    assert!(
        serde_json::from_str::<handbook_contracts::ClaimVerdictDecisionV1>("\"compatible\"")
            .is_err()
    );
    let candidate_with_promotion = serde_json::json!({
        "parent_promotion_eligible": true
    });
    assert!(
        serde_json::from_value::<handbook_contracts::EvidenceCandidateV1>(candidate_with_promotion)
            .is_err()
    );
}
