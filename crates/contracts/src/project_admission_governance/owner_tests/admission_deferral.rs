use handbook_contracts::project_admission_governance::*;

#[test]
fn approved_instruction_is_advisory_and_has_no_configuration_or_ceiling() {
    let input = DeferralRequestInputV2::human_approved(
        "initial",
        "The human approves deferring this scope.",
    );
    let validated = validate_deferral_request(input.clone()).unwrap();
    assert_eq!(validated.input, input);
    assert_eq!(
        validated.input.instruction.mode,
        AuthorityModeV1::StandaloneLocalAdvisory
    );
    let encoded = String::from_utf8(input.canonical_json_bytes().unwrap()).unwrap();
    assert!(encoded.contains("human_approval_asserted\":true"));
    assert!(!encoded.contains("configuration"));
    assert!(!encoded.contains("ceiling"));
    assert!(!encoded.contains("verified_delegation"));
}

#[test]
fn malformed_or_unapproved_input_refuses_before_effects() {
    let original = DeferralRequestInputV2::human_approved("initial", "Human-approved deferral.");
    let changes: &[fn(&mut DeferralRequestInputV2)] = &[
        |request| request.scope.push('/'),
        |request| request.instruction.human_approval_asserted = false,
        |request| request.instruction.text.clear(),
        |request| request.instruction.text = "x".repeat(4097),
        |request| request.instruction.asserted_agent = Some(String::new()),
        |request| request.request_key = "invalid key".into(),
        |request| request.request_key = "x".repeat(129),
    ];
    for change in changes {
        let root = tempfile::tempdir().unwrap();
        let mut input = original.clone();
        change(&mut input);
        assert!(ProjectAdmissionDeferralV2::new()
            .prepare(root.path(), &input)
            .is_err());
        assert_eq!(std::fs::read_dir(root.path()).unwrap().count(), 0);
    }
}

#[test]
fn closed_request_json_rejects_omitted_duplicate_unknown_mode_and_documents() {
    let original = DeferralRequestInputV2::human_approved("initial", "Human-approved deferral.");
    let encoded = String::from_utf8(original.canonical_json_bytes().unwrap()).unwrap();
    let invalid = [
        encoded.replace(
            "\"human_approval_asserted\":true",
            "\"human_approval_asserted\":true,\"human_approval_asserted\":true",
        ),
        encoded.replace(
            "\"request_key\":\"initial\"",
            "\"request_key\":\"initial\",\"request_key\":\"initial\"",
        ),
        encoded.replacen('{', "{\"rule\":\"weakened@2.0.0\",", 1),
        encoded.replace("standalone_local_advisory", "agent_role"),
        encoded.replace(
            "fresh_registration_unused",
            "existing_project_empty_governance",
        ),
        encoded.replace("defer_initial_governance", "establish_governance"),
        format!("{encoded} {{}}"),
    ];
    for bytes in invalid {
        assert!(
            DeferralRequestInputV2::from_json_bytes(bytes.as_bytes()).is_err(),
            "{bytes}"
        );
    }
    let mut omitted = serde_json::to_value(original).unwrap();
    omitted["instruction"]
        .as_object_mut()
        .unwrap()
        .remove("human_approval_asserted");
    assert!(
        DeferralRequestInputV2::from_json_bytes(&serde_json::to_vec(&omitted).unwrap()).is_err()
    );
    assert!(DeferralRequestInputV2::from_json_bytes(&vec![
        b' ';
        MAX_GOVERNANCE_RECORD_BYTES_V1 + 1
    ])
    .is_err());
}

#[test]
fn shipped_v2_rule_is_distinct_and_v1_rules_preserve_exact_fixtures() {
    let rule = establishment_rule_descriptor_v2().unwrap();
    assert_eq!(
        rule.binding.fingerprint().as_str(),
        "sha256:a1053c8ebc8c67cf0dff0e9cb57d292039f4e1c223b0308e0210af2a4157bd08"
    );
    assert_eq!(
        rule.binding.reference().as_str(),
        "handbook.admission-establishment.standalone@2.0.0"
    );
    assert!(rule.candidate_kind.is_none());
    assert!(rule.constraints.iter().any(|value| value == "same_identity_later_first_establishment_requires_exact_current_deferred_predecessor_and_new_human_approved_terms"));
    assert_eq!(
        establishment_rule_descriptor_v1()
            .unwrap()
            .binding
            .fingerprint()
            .as_str(),
        "sha256:52d6f6ed6df9e72a6b4c79709386daa225c581a064201355a17b973ad32fd069"
    );
    assert_eq!(
        freshness_admission_rule_descriptor_v1()
            .unwrap()
            .binding
            .fingerprint()
            .as_str(),
        "sha256:cafbaa0b8b5f81fd1041dcb606cd61d1db909b238dfee6b66729d0ab1a576f51"
    );
}

#[test]
fn request_canonical_bytes_are_an_exact_versioned_fixture() {
    let input = DeferralRequestInputV2::human_approved("initial", "Human-approved deferral.");
    assert_eq!(
        String::from_utf8(input.canonical_json_bytes().unwrap()).unwrap(),
        r#"{"expected_initial_state":"fresh_registration_unused","instruction":{"asserted_agent":null,"asserted_delegation":null,"asserted_principal":null,"choice":"defer_initial_governance","human_approval_asserted":true,"mode":"standalone_local_advisory","text":"Human-approved deferral."},"request_key":"initial","scope":"contracts.evidence-freshness-policy/root"}"#
    );
}

#[test]
fn native_deferral_retains_exact_original_and_separate_current_after_reopen() {
    let root = tempfile::tempdir().unwrap();
    let owner = ProjectAdmissionDeferralV2::new();
    let input =
        DeferralRequestInputV2::human_approved("initial", "Human approves this exact deferral.");
    let prepared = owner.prepare(root.path(), &input).unwrap();
    assert_eq!(std::fs::read_dir(root.path()).unwrap().count(), 0);
    let locator = prepared.locator().clone();
    let retained_locator = locator.canonical_json_bytes().unwrap();
    let result = owner.initialize(root.path(), prepared);
    let DeferralExecutionV2::Committed { outcome, delivery } = result else {
        panic!("{result:?}");
    };
    assert_eq!(delivery, GovernanceDeliveryV1::Original);
    let restored = DeferralRequestLocatorV2::from_json_bytes(&retained_locator).unwrap();
    let replay = ProjectAdmissionDeferralV2::new().resolve_request(root.path(), &restored);
    assert_eq!(
        replay,
        DeferralExecutionV2::Committed {
            outcome: outcome.clone(),
            delivery: GovernanceDeliveryV1::Replay
        }
    );
    let basis = ProjectAdmissionDeferralV2::new()
        .resolve_current(root.path(), &locator.project, &input.scope, None)
        .unwrap();
    assert!(basis.never_established);
    assert_eq!(basis.outcome_fingerprint, outcome.outcome_fingerprint);
    assert_eq!(
        basis.registration_identity,
        locator.project.registration_identity
    );
    assert!(owner.prepare(root.path(), &input).is_err());
}

#[test]
fn exact_outcome_and_basis_wire_are_bounded_closed_and_do_not_restore_creation() {
    let root = tempfile::tempdir().unwrap();
    let owner = ProjectAdmissionDeferralV2::new();
    let prepared = owner
        .prepare(
            root.path(),
            &DeferralRequestInputV2::human_approved("initial", "Human approves exact deferral."),
        )
        .unwrap();
    let locator = prepared.locator().clone();
    let result = owner.initialize(root.path(), prepared);
    let bytes = result.canonical_json_bytes().unwrap();
    assert_eq!(
        DeferralExecutionV2::from_json_bytes(&bytes).unwrap(),
        result
    );
    let encoded = String::from_utf8(bytes).unwrap();
    for invalid in [
        encoded.replace(
            "\"deferral_committed\":true",
            "\"deferral_committed\":true,\"deferral_committed\":true",
        ),
        encoded.replace("\"deferral_committed\":true,", ""),
        encoded.replace(
            "\"deferral_committed\":true",
            "\"deferral_committed\":false",
        ),
        encoded.replacen('{', "{\"permission\":\"all\",", 1),
        format!("{encoded} {{}}"),
    ] {
        assert!(DeferralExecutionV2::from_json_bytes(invalid.as_bytes()).is_err());
    }
    assert!(
        DeferralExecutionV2::from_json_bytes(&vec![b' '; MAX_GOVERNANCE_RECORD_BYTES_V1 + 1])
            .is_err()
    );
    let basis = owner
        .resolve_current(root.path(), &locator.project, &locator.request.scope, None)
        .unwrap();
    assert_eq!(
        DeferredGovernanceBasisV2::from_json_bytes(&basis.canonical_json_bytes().unwrap()).unwrap(),
        basis
    );
    let encoded = String::from_utf8(basis.canonical_json_bytes().unwrap()).unwrap();
    for invalid in [
        encoded.replace(
            "\"never_established\":true",
            "\"never_established\":true,\"never_established\":true",
        ),
        encoded.replace("\"never_established\":true,", ""),
        encoded.replace("\"never_established\":true", "\"never_established\":false"),
        encoded.replace("\"version\":\"2.0.0\"", "\"version\":\"2.1.0\""),
        format!("{encoded} {{}}"),
    ] {
        assert!(DeferredGovernanceBasisV2::from_json_bytes(invalid.as_bytes()).is_err());
    }
    assert!(DeferredGovernanceBasisV2::from_json_bytes(&vec![
        b' ';
        MAX_GOVERNANCE_RECORD_BYTES_V1 + 1
    ])
    .is_err());
    assert!(owner.prepare(root.path(), &locator.request).is_err());
}
