use handbook_contracts::project_admission_governance::*;
use handbook_contracts::FingerprintV1;
use sha2::{Digest, Sha256};

fn fingerprint(domain: &str, content: serde_json::Value) -> FingerprintV1 {
    let bytes = serde_json_canonicalizer::to_vec(&serde_json::json!({
        "domain": domain,
        "content": content,
    }))
    .unwrap();
    FingerprintV1::parse(&format!("sha256:{:x}", Sha256::digest(bytes))).unwrap()
}

fn project() -> ProjectBindingV2 {
    let identity = format!("sha256:{}", "1".repeat(64));
    let physical = format!("sha256:{}", "2".repeat(64));
    let root = if cfg!(windows) {
        r"C:\handbook-fixture"
    } else {
        "/handbook-fixture"
    };
    ProjectBindingV2 {
        registration_identity: fingerprint(
            "handbook.admission-registration@2.0.0",
            serde_json::json!([identity, root, physical]),
        ),
        repository_identity_fingerprint: identity,
        retained_root: root.into(),
        physical_root_fingerprint: physical,
    }
}

fn expected() -> ExpectedDeferredBasisV2 {
    ExpectedDeferredBasisV2 {
        outcome_fingerprint: FingerprintV1::parse(&format!("sha256:{}", "3".repeat(64))).unwrap(),
        relation_fingerprint: FingerprintV1::parse(&format!("sha256:{}", "4".repeat(64))).unwrap(),
    }
}

fn request(ceiling: u64) -> DeferredEstablishmentRequestV3 {
    DeferredEstablishmentRequestV3::human_approved(
        project(),
        expected(),
        ceiling,
        "establish",
        "Human approves this exact configuration and terms.",
    )
    .unwrap()
}

#[test]
fn explicit_approval_retains_two_caller_ceilings_and_original_configuration_domain() {
    let first = validate_deferred_establishment_request(request(7_200)).unwrap();
    let second = validate_deferred_establishment_request(request(86_400)).unwrap();
    assert_eq!(
        first.input.configuration.bindings[0].max_observation_age_seconds,
        7_200
    );
    assert_eq!(
        second.input.configuration.bindings[0].max_observation_age_seconds,
        86_400
    );
    assert_ne!(
        first.configuration_fingerprint,
        second.configuration_fingerprint
    );
    assert_ne!(
        first.input.instruction.approved_content_fingerprint,
        second.input.instruction.approved_content_fingerprint
    );
    assert_eq!(first.owner_rule, second.owner_rule);
    assert_eq!(first.establishment_rule, second.establishment_rule);
    let original = validate_request(GovernanceRequestInputV1::standalone(
        "establish",
        7_200,
        "Human approves this exact configuration and terms.",
    ))
    .unwrap();
    assert_eq!(
        first.configuration_fingerprint,
        original.configuration_fingerprint
    );
    assert_eq!(first.owner_rule, original.owner_rule);
    assert_ne!(
        first.instruction_fingerprint,
        original.instruction_fingerprint
    );
}

#[test]
fn old_deferral_or_recommendation_cannot_approve_exact_establishment() {
    let mut unapproved = request(7_200);
    unapproved.instruction.human_approval_asserted = false;
    unapproved.instruction.text = "Recommend establishing governance.".into();
    assert!(validate_deferred_establishment_request(unapproved).is_err());
    let mut changed = request(7_200);
    changed.configuration.bindings[0].max_observation_age_seconds = 86_400;
    assert_eq!(
        validate_deferred_establishment_request(changed)
            .unwrap_err()
            .condition,
        "human_approval_identity"
    );
    let mut stale = request(7_200);
    stale.expected_deferred.outcome_fingerprint =
        FingerprintV1::parse(&format!("sha256:{}", "5".repeat(64))).unwrap();
    assert_eq!(
        validate_deferred_establishment_request(stale)
            .unwrap_err()
            .condition,
        "human_approval_identity"
    );
}

#[test]
fn new_rule_binds_original_v2_rule_and_preserves_every_shipped_rule_fixture() {
    let rule = establishment_rule_descriptor_v3().unwrap();
    assert_eq!(rule.owner, "handbook");
    assert_eq!(rule.name, ESTABLISHMENT_RULE_NAME_V1);
    assert_eq!(rule.version, "3.0.0");
    assert_eq!(
        rule.binding.fingerprint().as_str(),
        "sha256:366d1c0283def9a585ec4fa9d7c0ed27f64e54f4b418bd21bc5d362014b503b5"
    );
    assert_eq!(
        rule.binding.reference().as_str(),
        "handbook.admission-establishment.standalone@3.0.0"
    );
    assert!(rule.candidate_kind.is_none());
    assert!(rule.candidate_schema_ref.is_none());
    assert!(rule.required_authored_sources.is_empty());
    assert!(rule.approval_sources.is_empty());
    assert!(rule.exception_sources.is_empty());
    assert!(rule.constraints.contains(
        &concat!(
            "original_establishment_rule:handbook.admission-establishment.standalone@2.0.0:",
            "sha256:a1053c8ebc8c67cf0dff0e9cb57d292039f4e1c223b0308e0210af2a4157bd08"
        )
        .to_string()
    ));
    assert_eq!(
        establishment_rule_descriptor_v1()
            .unwrap()
            .binding
            .fingerprint()
            .as_str(),
        "sha256:52d6f6ed6df9e72a6b4c79709386daa225c581a064201355a17b973ad32fd069"
    );
    assert_eq!(
        establishment_rule_descriptor_v2()
            .unwrap()
            .binding
            .fingerprint()
            .as_str(),
        "sha256:a1053c8ebc8c67cf0dff0e9cb57d292039f4e1c223b0308e0210af2a4157bd08"
    );
    assert_eq!(
        freshness_admission_rule_descriptor_v1()
            .unwrap()
            .binding
            .fingerprint()
            .as_str(),
        "sha256:cafbaa0b8b5f81fd1041dcb606cd61d1db909b238dfee6b66729d0ab1a576f51"
    );
    let mut content = serde_json::to_value(&rule).unwrap();
    content.as_object_mut().unwrap().remove("binding");
    content["original_establishment_rule"] =
        serde_json::to_value(establishment_rule_descriptor_v2().unwrap().binding).unwrap();
    assert_eq!(
        rule.binding.fingerprint(),
        &fingerprint(
            "handbook.project-admission-governance.shipped-rule@3.0.0",
            content
        )
    );
}

#[test]
fn locator_binds_complete_request_without_identity_allocation_or_creation_claim() {
    let input = request(7_200);
    let locator = create_deferred_establishment_locator(input.clone()).unwrap();
    let bytes = locator.canonical_json_bytes().unwrap();
    assert_eq!(
        DeferredEstablishmentLocatorV3::from_json_bytes(&bytes).unwrap(),
        locator
    );
    assert_eq!(
        locator.request_fingerprint,
        fingerprint(
            "handbook.project-admission-governance.request@3.0.0",
            serde_json::json!({
                "request": input,
                "establishment_rule": establishment_rule_descriptor_v3().unwrap().binding,
                "owner_rule": freshness_admission_rule_descriptor_v1().unwrap().binding,
            })
        )
    );
    let encoded = String::from_utf8(bytes).unwrap();
    assert!(!encoded.contains("initialization_claim"));
    assert!(!encoded.contains("expected_initial_state"));
    assert!(!encoded.contains("creation_token"));
    let mut changed = locator.clone();
    changed.request.request_key = "other-key".into();
    assert_eq!(
        validate_deferred_establishment_locator(&changed)
            .unwrap_err()
            .condition,
        "request_identity"
    );
    assert!(DeferredEstablishmentLocatorV3::from_json_bytes(
        &serde_json::to_vec(&changed).unwrap()
    )
    .is_err());
    let mut changed = locator;
    changed.request_fingerprint =
        FingerprintV1::parse(&format!("sha256:{}", "6".repeat(64))).unwrap();
    assert_eq!(
        validate_deferred_establishment_locator(&changed)
            .unwrap_err()
            .condition,
        "request_identity"
    );
}

#[test]
fn explicit_approval_and_instruction_domains_bind_exact_content_and_advisory_claims() {
    let input = request(7_200);
    assert_eq!(
        input.instruction.approved_content_fingerprint,
        fingerprint(
            "handbook.project-admission-governance.approved-content@3.0.0",
            serde_json::json!({
                "project": input.project,
                "scope": input.scope,
                "expected_deferred": input.expected_deferred,
                "configuration": input.configuration,
                "replacement_basis": input.replacement_basis,
                "establishment_rule": establishment_rule_descriptor_v3().unwrap().binding,
                "owner_rule": freshness_admission_rule_descriptor_v1().unwrap().binding,
            })
        )
    );
    let original = validate_deferred_establishment_request(input.clone()).unwrap();
    assert_eq!(
        original.instruction_fingerprint,
        fingerprint(
            "handbook.project-admission-governance.local-advisory-instruction@3.0.0",
            serde_json::to_value(&input.instruction).unwrap()
        )
    );
    let mut claimed = input;
    claimed.instruction.asserted_principal = Some("Local operator".into());
    claimed.instruction.asserted_agent = Some("Configured agent".into());
    claimed.instruction.asserted_delegation = Some("Human instructed this exact operation".into());
    let validated = validate_deferred_establishment_request(claimed).unwrap();
    assert_eq!(
        validated.configuration_fingerprint,
        original.configuration_fingerprint
    );
    assert_eq!(
        validated.input.instruction.approved_content_fingerprint,
        original.input.instruction.approved_content_fingerprint
    );
    assert_ne!(
        validated.instruction_fingerprint,
        original.instruction_fingerprint
    );
    assert_eq!(
        validated.input.instruction.mode,
        AuthorityModeV1::StandaloneLocalAdvisory
    );
    let bytes = String::from_utf8(validated.input.canonical_json_bytes().unwrap()).unwrap();
    assert!(!bytes.contains("verified_delegation"));
    assert!(!bytes.contains("authenticated"));
}

#[test]
fn exact_v2_project_binding_is_required_without_v1_registration_cast() {
    let original = request(7_200);
    let changes: &[fn(&mut ProjectBindingV2)] = &[
        |p| p.retained_root = "relative/root".into(),
        |p| p.retained_root.push('\0'),
        |p| p.repository_identity_fingerprint = "unknown".into(),
        |p| p.physical_root_fingerprint = "unknown".into(),
        |p| {
            p.registration_identity =
                FingerprintV1::parse(&format!("sha256:{}", "6".repeat(64))).unwrap()
        },
    ];
    for change in changes {
        let mut malformed = original.clone();
        change(&mut malformed.project);
        assert_eq!(
            validate_deferred_establishment_request(malformed)
                .unwrap_err()
                .condition,
            "project_binding"
        );
    }
    let mut malformed = original;
    malformed.project.registration_identity = fingerprint(
        "handbook.admission-registration@1.0.0",
        serde_json::json!([
            malformed.project.repository_identity_fingerprint,
            malformed.project.retained_root,
            malformed.project.physical_root_fingerprint,
        ]),
    );
    assert_eq!(
        validate_deferred_establishment_request(malformed)
            .unwrap_err()
            .condition,
        "project_binding"
    );
}

#[test]
fn fixed_configuration_identity_population_and_floors_cannot_be_relaxed() {
    let cases: &[(&str, fn(&mut DeferredEstablishmentRequestV3))] = &[
        ("scope", |r| r.scope.push('/')),
        ("configuration_identity", |r| {
            r.configuration.owner = "SDK".into()
        }),
        ("configuration_identity", |r| {
            r.configuration.name.push_str(".other")
        }),
        ("configuration_identity", |r| {
            r.configuration.version = "latest".into()
        }),
        ("binding_population", |r| r.configuration.bindings.clear()),
        ("binding_population", |r| {
            r.configuration
                .bindings
                .push(r.configuration.bindings[0].clone())
        }),
        ("owner_rule", |r| {
            r.configuration.bindings[0].owner = "other-owner".into()
        }),
        ("owner_rule", |r| {
            r.configuration.bindings[0].rule_name = "other-rule".into()
        }),
        ("owner_rule", |r| {
            r.configuration.bindings[0].rule_version = "1.0.1".into()
        }),
        ("owner_rule", |r| {
            r.configuration.bindings[0].candidate_kind = "DecisionRecord".into()
        }),
        ("freshness_ceiling", |r| {
            r.configuration.bindings[0].max_observation_age_seconds = 0
        }),
        ("freshness_ceiling", |r| {
            r.configuration.bindings[0].max_observation_age_seconds = JSON_SAFE_INTEGER_MAX + 1
        }),
        ("freshness_floor", |r| {
            r.configuration.bindings[0].require_exact_source_revision = false
        }),
        ("freshness_floor", |r| {
            r.configuration.bindings[0].future_skew_seconds = 1
        }),
        ("governance_sources", |r| {
            r.configuration.bindings[0]
                .required_authored_sources
                .push("charter@1.0.0".into())
        }),
        ("governance_sources", |r| {
            r.configuration.bindings[0]
                .approval_sources
                .push("approval".into())
        }),
        ("governance_sources", |r| {
            r.configuration.bindings[0]
                .exception_sources
                .push("exception".into())
        }),
    ];
    for (condition, change) in cases {
        let mut malformed = request(7_200);
        change(&mut malformed);
        assert_eq!(
            &validate_deferred_establishment_request(malformed)
                .unwrap_err()
                .condition,
            condition
        );
    }
    assert!(DeferredEstablishmentRequestV3::human_approved(
        project(),
        expected(),
        JSON_SAFE_INTEGER_MAX,
        "max",
        "Human approves these exact terms."
    )
    .is_ok());
    for ceiling in [0, JSON_SAFE_INTEGER_MAX + 1] {
        assert_eq!(
            DeferredEstablishmentRequestV3::human_approved(
                project(),
                expected(),
                ceiling,
                "bad",
                "Human approves these exact terms."
            )
            .unwrap_err()
            .condition,
            "freshness_ceiling"
        );
    }
}

#[test]
fn fixed_replacement_terms_and_bounded_instruction_and_claims_are_required() {
    let terms: &[fn(&mut ReplacementAuthorityBasisV1)] = &[
        |t| t.explicit_local_instruction_required = false,
        |t| t.same_project_and_scope_only = false,
        |t| t.preserve_fixed_constraints = false,
        |t| t.require_exact_predecessor = false,
        |t| t.require_confirmed_cutover = false,
        |t| t.terms_immutable = false,
    ];
    for change in terms {
        let mut input = request(7_200);
        change(&mut input.replacement_basis);
        assert_eq!(
            validate_deferred_establishment_request(input)
                .unwrap_err()
                .condition,
            "replacement_basis"
        );
    }
    for text in [
        String::new(),
        " \n\t".into(),
        "bad\0text".into(),
        "x".repeat(4097),
    ] {
        let mut input = request(7_200);
        input.instruction.text = text.clone();
        assert_eq!(
            validate_deferred_establishment_request(input)
                .unwrap_err()
                .condition,
            "explicit_instruction"
        );
        for claim in 0..3 {
            let mut input = request(7_200);
            match claim {
                0 => input.instruction.asserted_principal = Some(text.clone()),
                1 => input.instruction.asserted_agent = Some(text.clone()),
                _ => input.instruction.asserted_delegation = Some(text.clone()),
            }
            assert_eq!(
                validate_deferred_establishment_request(input)
                    .unwrap_err()
                    .condition,
                "explicit_instruction"
            );
        }
    }
    for key in [
        String::new(),
        "invalid key".into(),
        "x".repeat(129),
        "unicode-é".into(),
    ] {
        let mut input = request(7_200);
        input.request_key = key;
        assert_eq!(
            validate_deferred_establishment_request(input)
                .unwrap_err()
                .condition,
            "request_key"
        );
    }
}

#[test]
fn closed_new_wire_rejects_missing_unknown_duplicate_mode_and_trailing_fields() {
    let input = request(7_200);
    let encoded = String::from_utf8(input.canonical_json_bytes().unwrap()).unwrap();
    for invalid in [
        encoded.replacen('{', "{\"selected_rule\":\"weakened@3.0.0\",", 1),
        encoded.replace(
            "\"future_skew_seconds\":0",
            "\"future_skew_seconds\":0,\"waiver\":true",
        ),
        encoded.replace(
            "\"future_skew_seconds\":0",
            "\"future_skew_seconds\":0,\"future_skew_seconds\":0",
        ),
        encoded.replace(
            "\"request_key\":\"establish\"",
            "\"request_key\":\"establish\",\"request_key\":\"establish\"",
        ),
        encoded.replace(
            "\"human_approval_asserted\":true",
            "\"human_approval_asserted\":true,\"human_approval_asserted\":true",
        ),
        encoded.replace("standalone_local_advisory", "authenticated"),
        format!("{encoded} {{}}"),
        format!("{encoded} trailing"),
    ] {
        assert_eq!(
            DeferredEstablishmentRequestV3::from_json_bytes(invalid.as_bytes())
                .unwrap_err()
                .condition,
            "closed_json",
            "{invalid}"
        );
    }
    let value = serde_json::to_value(&input).unwrap();
    let paths = [
        ("", "project"),
        ("", "scope"),
        ("", "expected_deferred"),
        ("", "configuration"),
        ("", "replacement_basis"),
        ("", "instruction"),
        ("", "request_key"),
        ("/project", "repository_identity_fingerprint"),
        ("/project", "retained_root"),
        ("/project", "physical_root_fingerprint"),
        ("/project", "registration_identity"),
        ("/expected_deferred", "outcome_fingerprint"),
        ("/expected_deferred", "relation_fingerprint"),
        ("/configuration", "owner"),
        ("/configuration", "name"),
        ("/configuration", "version"),
        ("/configuration", "bindings"),
        ("/configuration/bindings/0", "owner"),
        ("/configuration/bindings/0", "rule_name"),
        ("/configuration/bindings/0", "rule_version"),
        ("/configuration/bindings/0", "candidate_kind"),
        ("/configuration/bindings/0", "max_observation_age_seconds"),
        ("/configuration/bindings/0", "require_exact_source_revision"),
        ("/configuration/bindings/0", "future_skew_seconds"),
        ("/configuration/bindings/0", "required_authored_sources"),
        ("/configuration/bindings/0", "approval_sources"),
        ("/configuration/bindings/0", "exception_sources"),
        ("/replacement_basis", "mode"),
        ("/replacement_basis", "explicit_local_instruction_required"),
        ("/replacement_basis", "same_project_and_scope_only"),
        ("/replacement_basis", "preserve_fixed_constraints"),
        ("/replacement_basis", "require_exact_predecessor"),
        ("/replacement_basis", "require_confirmed_cutover"),
        ("/replacement_basis", "terms_immutable"),
        ("/instruction", "mode"),
        ("/instruction", "human_approval_asserted"),
        ("/instruction", "approved_content_fingerprint"),
        ("/instruction", "text"),
        ("/instruction", "asserted_principal"),
        ("/instruction", "asserted_agent"),
        ("/instruction", "asserted_delegation"),
    ];
    for (path, field) in paths {
        let mut omitted = value.clone();
        omitted
            .pointer_mut(path)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .remove(field);
        assert_eq!(
            DeferredEstablishmentRequestV3::from_json_bytes(&serde_json::to_vec(&omitted).unwrap())
                .unwrap_err()
                .condition,
            "closed_json",
            "missing {path}/{field}"
        );
    }
    assert_eq!(
        DeferredEstablishmentRequestV3::from_json_bytes(&vec![
            b' ';
            MAX_GOVERNANCE_RECORD_BYTES_V1 + 1
        ])
        .unwrap_err()
        .condition,
        "record_bound"
    );
}

#[test]
fn ceiling_wire_is_required_integer_and_never_uses_a_fallback() {
    let value = serde_json::to_value(request(7_200)).unwrap();
    for bad in [
        serde_json::Value::Null,
        serde_json::json!(0),
        serde_json::json!(-1),
        serde_json::json!(1.5),
        serde_json::json!("7200"),
        serde_json::json!(JSON_SAFE_INTEGER_MAX + 1),
    ] {
        let mut malformed = value.clone();
        malformed["configuration"]["bindings"][0]["max_observation_age_seconds"] = bad;
        assert!(DeferredEstablishmentRequestV3::from_json_bytes(
            &serde_json::to_vec(&malformed).unwrap()
        )
        .is_err());
    }
}

#[test]
fn new_locator_wire_is_closed_and_bound_with_no_restored_prepared_operation() {
    let locator = create_deferred_establishment_locator(request(7_200)).unwrap();
    let encoded = String::from_utf8(locator.canonical_json_bytes().unwrap()).unwrap();
    for invalid in [
        encoded.replacen('{', "{\"initialization_claim\":{},", 1),
        encoded.replacen('{', "{\"prepared_operation\":{},", 1),
        encoded.replacen("\"request\":", "\"request\":{},\"request\":", 1),
        format!("{encoded} {{}}"),
    ] {
        assert!(DeferredEstablishmentLocatorV3::from_json_bytes(invalid.as_bytes()).is_err());
    }
    for field in ["request", "request_fingerprint"] {
        let mut omitted = serde_json::to_value(&locator).unwrap();
        omitted.as_object_mut().unwrap().remove(field);
        assert!(DeferredEstablishmentLocatorV3::from_json_bytes(
            &serde_json::to_vec(&omitted).unwrap()
        )
        .is_err());
    }
    assert_eq!(
        DeferredEstablishmentLocatorV3::from_json_bytes(&vec![
            b' ';
            MAX_GOVERNANCE_RECORD_BYTES_V1 + 1
        ])
        .unwrap_err()
        .condition,
        "record_bound"
    );
}

#[test]
fn exact_v3_request_approval_and_instruction_match_independent_canonical_fixtures() {
    // Independently calculated with Python's sorted compact JSON and SHA-256.
    // Only the explicitly synthetic absolute root spelling varies by platform.
    let (approval, instruction, request_identity) = if cfg!(windows) {
        (
            "sha256:e8895ed101f7194f00bdcb0b65a140818adf50089f56f28ad2ad067aceeb5990",
            "sha256:03a868fbeda61f453546635337826a04646957e0ec474a6a882f221cb4a510ea",
            "sha256:3fc2b0428157ea64b2c5ccf2035992f9d9cb230b0e6b1795b1b6ee205112b706",
        )
    } else {
        (
            "sha256:b49de578668ac36804f12da00fd0baae32dc77eae17c26e77def63425874a193",
            "sha256:94d5cdcd7b700a6b8ff843b19b575840b28673d8697fed92d1210e55627c3bae",
            "sha256:35b8967da99ebb9e2ebc61f4d0e47014b4e3cd3f832ffedaa05a2c0573be2fda",
        )
    };
    let input = request(7_200);
    let validated = validate_deferred_establishment_request(input.clone()).unwrap();
    let locator = create_deferred_establishment_locator(input).unwrap();
    assert_eq!(
        validated
            .input
            .instruction
            .approved_content_fingerprint
            .as_str(),
        approval
    );
    assert_eq!(validated.instruction_fingerprint.as_str(), instruction);
    assert_eq!(locator.request_fingerprint.as_str(), request_identity);
}

#[test]
fn changed_project_or_relation_requires_new_exact_content_approval() {
    let mut input = request(7_200);
    input.expected_deferred.relation_fingerprint =
        FingerprintV1::parse(&format!("sha256:{}", "6".repeat(64))).unwrap();
    assert_eq!(
        validate_deferred_establishment_request(input)
            .unwrap_err()
            .condition,
        "human_approval_identity"
    );
    let mut input = request(7_200);
    input.project.repository_identity_fingerprint = format!("sha256:{}", "7".repeat(64));
    input.project.registration_identity = fingerprint(
        "handbook.admission-registration@2.0.0",
        serde_json::json!([
            input.project.repository_identity_fingerprint,
            input.project.retained_root,
            input.project.physical_root_fingerprint,
        ]),
    );
    assert_eq!(
        validate_deferred_establishment_request(input)
            .unwrap_err()
            .condition,
        "human_approval_identity"
    );
}
