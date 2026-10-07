use handbook_contracts::*;
use sha2::{Digest, Sha256};
fn fp(bytes: &[u8]) -> FingerprintV1 {
    FingerprintV1::parse(&format!("sha256:{:x}", Sha256::digest(bytes))).unwrap()
}

#[test]
fn original_request_recovery_is_read_only_and_does_not_allocate_or_initialize_evaluation() {
    let root = tempfile::tempdir().unwrap();
    let owner = ContractOwnerRepositoryV1::open(root.path()).unwrap();
    let original = LocalEvaluationRequestLocatorV3 {
        request_key: "lost-original-formation".into(),
        request_fingerprint: fp(b"original complete request"),
    };
    assert_eq!(
        owner
            .recover_local_evaluation_request_v3(&original)
            .unwrap(),
        None
    );
    assert!(!root.path().join(LOCAL_EVALUATION_OWNER_ROOT_V3).exists());
    assert!(!root.path().join(LOCAL_CONTRACT_OWNER_ROOT_V3).exists());
    assert_eq!(
        std::fs::read_dir(
            root.path()
                .join(".handbook/state/contracts/v1/transactions")
        )
        .unwrap()
        .count(),
        0
    );
    let malformed = LocalEvaluationRequestLocatorV3 {
        request_key: "../new-occurrence".into(),
        request_fingerprint: original.request_fingerprint,
    };
    assert!(owner
        .recover_local_evaluation_request_v3(&malformed)
        .is_err());
    assert!(!root.path().join(LOCAL_EVALUATION_OWNER_ROOT_V3).exists());
}

#[test]
fn existing_full_owner_reads_never_repair_a_missing_original_lock() {
    let root = tempfile::tempdir().unwrap();
    assert!(ContractOwnerRepositoryV1::open_existing_for_local_use_v3(root.path()).is_err());
    assert!(!root.path().join(".handbook").exists());
    let owner = ContractOwnerRepositoryV1::open(root.path()).unwrap();
    std::fs::remove_file(
        root.path()
            .join(".handbook/state/locks/contract-owner.lock"),
    )
    .unwrap();
    let original = LocalEvaluationRequestLocatorV3 {
        request_key: "original-request".into(),
        request_fingerprint: fp(b"original request"),
    };
    assert!(owner
        .recover_local_evaluation_request_v3(&original)
        .is_err());
    assert!(ContractOwnerRepositoryV1::open_existing_for_local_use_v3(root.path()).is_err());
    assert!(!root
        .path()
        .join(".handbook/state/locks/contract-owner.lock")
        .exists());
}

#[test]
fn public_capability_observation_is_data_and_preserves_exact_identity() {
    let manifest = ExactBindingV1::new(
        ExactRefV1::parse("handbook.dock.json-schema@1.0.0").unwrap(),
        fp(b"untrusted manifest"),
    );
    let bundle =
        ContentAddressedBindingV1::new("handbook.dock.fake-package@1.0.0", fp(b"untrusted bundle"))
            .unwrap();
    let record = NativeLocalEvaluationCapabilityV3::new(
        manifest,
        bundle,
        "x86_64-pc-windows-msvc".into(),
        "available".into(),
        serde_json::json!({"availability_reason":"caller_says_ready"}),
        SemanticTimeV1::parse("2026-10-06T00:00:00Z").unwrap(),
        SemanticTimeV1::parse("2026-11-06T00:00:00Z").unwrap(),
    )
    .unwrap();
    // Data can describe any nomination; it cannot construct a resolved owner fact.
    assert_eq!(
        record.semantic_capability,
        native_local_evaluation_capability_binding_v3().unwrap()
    );
    let mut substituted = record.clone();
    substituted.controls["availability_reason"] = serde_json::json!("different");
    assert!(substituted.validate().is_err());
    assert_ne!(
        local_evaluation_gate_binding_v3().unwrap(),
        local_evaluation_rule_binding_v3().unwrap()
    );
    assert_ne!(
        local_evidence_admission_policy_binding_v3().unwrap(),
        local_evidence_admission_policy_binding_v2().unwrap()
    );
}
