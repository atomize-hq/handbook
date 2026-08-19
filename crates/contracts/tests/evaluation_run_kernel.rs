use handbook_contracts::EvaluationRunNonceV1;

#[test]
fn evaluation_run_nonce_is_non_empty_bounded_and_exact() {
    assert!(EvaluationRunNonceV1::parse("").is_err());
    assert!(EvaluationRunNonceV1::parse(&"n".repeat(257)).is_err());

    let nonce = EvaluationRunNonceV1::parse("observation-0001").unwrap();
    assert_eq!(nonce.as_str(), "observation-0001");
    assert_eq!(
        serde_json::from_str::<EvaluationRunNonceV1>("\"observation-0001\"").unwrap(),
        nonce
    );
}
