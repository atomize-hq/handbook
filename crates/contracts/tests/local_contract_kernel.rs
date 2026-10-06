use handbook_contracts::{
    ExactRefV1, LocalContractUseIntentV2, LocalExactContentV2, LocalPermittedUseV2,
    LocalReviewDecisionV2, LocalSchemaClosureV2, LocalSchemaDocumentV2, LocalValidatorDependencyV2,
};
use serde_json::json;
use sha2::{Digest, Sha256};

#[test]
fn diagnostic_definition_refuses_decision_seeking_use() {
    assert!(LocalPermittedUseV2::DiagnosticOnly.permits(LocalContractUseIntentV2::Diagnostic));
    assert!(!LocalPermittedUseV2::DiagnosticOnly.permits(LocalContractUseIntentV2::DecisionSeeking));
    assert!(
        LocalPermittedUseV2::DecisionSeekingEligible.permits(LocalContractUseIntentV2::Diagnostic)
    );
    assert!(LocalPermittedUseV2::DecisionSeekingEligible
        .permits(LocalContractUseIntentV2::DecisionSeeking));
}

#[test]
fn review_decision_preserves_rejection_and_actual_selected_outcome() {
    let approved: LocalReviewDecisionV2 = serde_json::from_str("\"approve\"").unwrap();
    let rejected: LocalReviewDecisionV2 = serde_json::from_str("\"reject\"").unwrap();
    assert_ne!(approved, rejected);
    assert!(serde_json::from_str::<LocalReviewDecisionV2>("\"independent\"").is_err());
}

#[test]
fn source_identity_retains_its_exact_declared_newline_domain() {
    let reference = ExactRefV1::parse("handbook.schema.sample@1.0.0").unwrap();
    let value = LocalExactContentV2::new(reference.clone(), json!({"type":"object"})).unwrap();
    let source =
        LocalExactContentV2::from_source_bytes(reference.clone(), b"{\"type\":\"object\"}\n")
            .unwrap();
    assert_ne!(value.binding, source.binding);
    assert!(LocalExactContentV2::from_source_bytes(
        reference.clone(),
        b"{\"type\":\"object\",\"type\":\"object\"}"
    )
    .is_err());
    assert!(
        LocalExactContentV2::from_source_bytes(reference, b"{\"type\":\"object\"}\n\n").is_err()
    );
}

#[test]
fn complete_offline_schema_closure_refuses_missing_substituted_and_remote_members() {
    let reference = ExactRefV1::parse("handbook.schema.sample@1.0.0").unwrap();
    let root = LocalSchemaDocumentV2::new(
        "definitions/schemas/root.json".into(),
        json!({"$ref":"member.json"}),
    )
    .unwrap();
    let binding = handbook_contracts::ExactBindingV1::new(
        reference.clone(),
        root.document_fingerprint.clone(),
    );
    assert!(
        LocalSchemaClosureV2::new(binding.clone(), root.path.clone(), vec![root.clone()]).is_err()
    );
    let member = LocalSchemaDocumentV2::new(
        "definitions/schemas/member.json".into(),
        json!({"type":"string"}),
    )
    .unwrap();
    let closure = LocalSchemaClosureV2::new(
        binding.clone(),
        root.path.clone(),
        vec![member.clone(), root.clone()],
    )
    .unwrap();
    assert_eq!(closure.documents.len(), 2);
    let mut substituted = member;
    substituted.schema = json!({"type":"number"});
    assert!(
        LocalSchemaClosureV2::new(binding, root.path.clone(), vec![substituted, root]).is_err()
    );
    let remote = LocalSchemaDocumentV2::new(
        "definitions/schemas/root.json".into(),
        json!({"$ref":"https://example.com/schema.json"}),
    )
    .unwrap();
    let remote_binding =
        handbook_contracts::ExactBindingV1::new(reference, remote.document_fingerprint.clone());
    assert!(LocalSchemaClosureV2::new(remote_binding, remote.path.clone(), vec![remote]).is_err());
}

#[test]
fn local_schema_representation_matches_native_path_and_reference_grammar() {
    for path in [
        "schema.json",
        "definitions/schemas/../root.json",
        "definitions/schemas/bad|name.json",
        "definitions/schemas/ root.json",
    ] {
        assert!(LocalSchemaDocumentV2::new(path.into(), json!({"type":"object"})).is_err());
    }
    let reference = ExactRefV1::parse("handbook.schema.sample@1.0.0").unwrap();
    for bad_ref in [
        "#/missing",
        "#anchor",
        "#/~wrong",
        "root.json##",
        "./root.json",
        "root%2ejson",
    ] {
        let root = LocalSchemaDocumentV2::new(
            "definitions/schemas/root.json".into(),
            json!({"$ref":bad_ref}),
        )
        .unwrap();
        let binding = handbook_contracts::ExactBindingV1::new(
            reference.clone(),
            root.document_fingerprint.clone(),
        );
        assert!(
            LocalSchemaClosureV2::new(binding, root.path.clone(), vec![root]).is_err(),
            "{bad_ref}"
        );
    }
    let root = LocalSchemaDocumentV2::new(
        "definitions/schemas/root.json".into(),
        json!({"$defs":{"present":{"type":"string"}},"$ref":"#/$defs/present"}),
    )
    .unwrap();
    let binding =
        handbook_contracts::ExactBindingV1::new(reference, root.document_fingerprint.clone());
    assert!(LocalSchemaClosureV2::new(binding, root.path.clone(), vec![root]).is_ok());
}

#[test]
fn validator_dependency_refuses_a_self_consistent_same_ref_descriptor_substitution() {
    let bytes =
        include_bytes!("../../dock/production-manifests/x86_64-pc-windows-msvc/manifest.json")
            .strip_suffix(b"\n")
            .unwrap();
    assert!(LocalValidatorDependencyV2::from_manifest_json(bytes).is_ok());
    let mut manifest: serde_json::Value = serde_json::from_slice(bytes).unwrap();
    manifest["implementation_bundle"]["entrypoint"] = json!("unsupported-validator.exe");
    manifest
        .as_object_mut()
        .unwrap()
        .remove("manifest_fingerprint");
    let preimage = serde_json_canonicalizer::to_vec(&manifest).unwrap();
    manifest["manifest_fingerprint"] = json!(format!("sha256:{:x}", Sha256::digest(&preimage)));
    let forged = serde_json_canonicalizer::to_vec(&manifest).unwrap();
    assert!(LocalValidatorDependencyV2::from_manifest_json(&forged).is_err());
}
