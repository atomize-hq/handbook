use handbook_engine::local_authority::{
    LocalDecisionClassV1, LocalEnvelopeCeilingV1, LocalOperatingTermsV1, LocalOperationClassV1,
    LOCAL_OPERATING_RULE_V1,
};

#[test]
fn approval_preimage_does_not_approve_itself_and_closed_input_refuses_unknown_fields() {
    let mut terms = LocalOperatingTermsV1::from_json_bytes(br#"{}"#).unwrap_err();
    assert!(!terms.code.is_empty());
    terms = LocalOperatingTermsV1::from_json_bytes(br#"{"unexpected":true}"#).unwrap_err();
    assert!(!terms.code.is_empty());
    assert_ne!(
        LocalOperationClassV1::LocalValidation,
        LocalOperationClassV1::OwnerWrite
    );
}

fn approved_terms() -> LocalOperatingTermsV1 {
    let mut terms = LocalOperatingTermsV1 {
        request_key: "initial-terms".into(),
        mode: "local_advisory".into(),
        rule_ref: LOCAL_OPERATING_RULE_V1.into(),
        instruction: "Validate the supplied local inputs within these bounds".into(),
        principal: "human assertion".into(),
        agent: "local agent assertion".into(),
        delegation: "supplied initial instruction".into(),
        approval_source_ref: "conversation/initial".into(),
        human_approval_asserted: true,
        approved_content_fingerprint: String::new(),
        ceiling: LocalEnvelopeCeilingV1 {
            active_level_id: "strategic".into(),
            dimensions: [
                "slice",
                "normal",
                "current_slice",
                "slice_write",
                "coordination",
                "slice_closeout",
            ]
            .map(str::to_owned),
        },
        constraint_inputs: vec![],
        allowed_paths: vec![".handbook/outputs/**".into()],
        allowed_input_paths: vec!["inputs/**".into()],
        operation_classes: vec![
            LocalOperationClassV1::LocalValidation,
            LocalOperationClassV1::OwnerWrite,
        ],
        reserved_decisions: vec![LocalDecisionClassV1::CharterReplacement],
        delegated_decisions: vec![LocalDecisionClassV1::InputCorrection],
        revocation_principal: "human assertion".into(),
    };
    terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
    terms
}

#[test]
fn changed_reach_missing_approval_and_credential_mode_cannot_reuse_local_approval() {
    let terms = approved_terms();
    let bytes = terms.canonical_json_bytes().unwrap();
    assert_eq!(
        LocalOperatingTermsV1::from_json_bytes(&bytes).unwrap(),
        terms
    );
    let mut changed = terms.clone();
    changed.allowed_paths.push("secrets/**".into());
    assert!(changed.canonical_json_bytes().is_err());
    let mut missing = terms.clone();
    missing.human_approval_asserted = false;
    assert!(missing.canonical_json_bytes().is_err());
    let mut credential = terms.clone();
    credential.mode = "credential_required".into();
    credential.approved_content_fingerprint = credential.fingerprint_approved_content().unwrap();
    assert!(credential.canonical_json_bytes().is_err());
    let mut conflicting = terms;
    conflicting
        .delegated_decisions
        .push(LocalDecisionClassV1::CharterReplacement);
    conflicting.approved_content_fingerprint = conflicting.fingerprint_approved_content().unwrap();
    assert!(conflicting.canonical_json_bytes().is_err());
}
