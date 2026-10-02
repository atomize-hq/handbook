use handbook_contracts::project_admission_governance::*;

#[test]
fn caller_supplied_ceiling_remains_exact_and_changes_configuration_identity() {
    let first = validate_request(GovernanceRequestInputV1::standalone(
        "request-one",
        86_400,
        "Establish this exact governance for a new project.",
    ))
    .unwrap();
    let alternate = validate_request(GovernanceRequestInputV1::standalone(
        "request-one",
        7_200,
        "Establish this exact governance for a new project.",
    ))
    .unwrap();
    assert_eq!(
        alternate.input.configuration.bindings[0].max_observation_age_seconds,
        7_200
    );
    assert_ne!(
        first.configuration_fingerprint,
        alternate.configuration_fingerprint
    );
    assert_eq!(first.owner_rule, alternate.owner_rule);
    assert_eq!(first.establishment_rule, alternate.establishment_rule);
}

#[test]
fn zero_and_non_json_safe_ceilings_refuse() {
    for ceiling in [0, JSON_SAFE_INTEGER_MAX + 1] {
        let request = GovernanceRequestInputV1::standalone(
            "request",
            ceiling,
            "Initialize exact new project.",
        );
        assert_eq!(
            validate_request(request).unwrap_err().condition,
            "freshness_ceiling"
        );
    }
    assert!(validate_request(GovernanceRequestInputV1::standalone(
        "request",
        JSON_SAFE_INTEGER_MAX,
        "Initialize exact new project.",
    ))
    .is_ok());
}

#[test]
fn serialized_ceiling_is_required_strict_and_has_no_fallback() {
    let input =
        GovernanceRequestInputV1::standalone("request", 7_200, "Initialize exact new project.");
    let value = serde_json::to_value(&input).unwrap();
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
        assert!(parse_governance_request_json(&serde_json::to_vec(&malformed).unwrap()).is_err());
    }
    let mut omitted = value;
    omitted["configuration"]["bindings"][0]
        .as_object_mut()
        .unwrap()
        .remove("max_observation_age_seconds");
    assert!(parse_governance_request_json(&serde_json::to_vec(&omitted).unwrap()).is_err());
    assert_eq!(
        parse_governance_request_json(&serde_json::to_vec(&input).unwrap()).unwrap(),
        input
    );
}

#[test]
fn closed_json_refuses_unknown_duplicate_nested_and_trailing_content() {
    let request =
        GovernanceRequestInputV1::standalone("request", 7_200, "Initialize exact new project.");
    let encoded = String::from_utf8(request.canonical_json_bytes().unwrap()).unwrap();
    let duplicate_ceiling = encoded.replace(
        "\"max_observation_age_seconds\":7200",
        "\"max_observation_age_seconds\":7200,\"max_observation_age_seconds\":7200",
    );
    let duplicate_key = encoded.replace(
        "\"request_key\":\"request\"",
        "\"request_key\":\"request\",\"request_key\":\"request\"",
    );
    let selected_establishment_rule =
        encoded.replacen('{', "{\"establishment_rule\":\"weakened@1.0.0\",", 1);
    let unknown_nested = encoded.replace(
        "\"future_skew_seconds\":0",
        "\"future_skew_seconds\":0,\"waiver\":true",
    );
    for invalid in [
        duplicate_ceiling,
        duplicate_key,
        selected_establishment_rule,
        unknown_nested,
        format!("{encoded} {{}}"),
        format!("{encoded} trailing"),
    ] {
        assert_eq!(
            parse_governance_request_json(invalid.as_bytes())
                .unwrap_err()
                .condition,
            "closed_json"
        );
    }
    assert_eq!(
        parse_governance_request_json(&vec![b' '; MAX_GOVERNANCE_RECORD_BYTES_V1 + 1])
            .unwrap_err()
            .condition,
        "record_bound"
    );
}

#[test]
fn exact_scope_owner_rule_population_and_domain_floor_are_required() {
    let original =
        GovernanceRequestInputV1::standalone("request", 7_200, "Initialize exact new project.");
    let cases: &[(&str, fn(&mut GovernanceRequestInputV1))] = &[
        ("scope", |request| request.scope.push('/')),
        ("configuration_identity", |request| {
            request.configuration.owner = "SDK".into()
        }),
        ("configuration_identity", |request| {
            request.configuration.name.push_str(".other")
        }),
        ("configuration_identity", |request| {
            request.configuration.version = "latest".into()
        }),
        ("binding_population", |request| {
            request.configuration.bindings.clear()
        }),
        ("binding_population", |request| {
            request
                .configuration
                .bindings
                .push(request.configuration.bindings[0].clone())
        }),
        ("owner_rule", |request| {
            request.configuration.bindings[0].owner = "other-owner".into()
        }),
        ("owner_rule", |request| {
            request.configuration.bindings[0].rule_name = "other-rule".into()
        }),
        ("owner_rule", |request| {
            request.configuration.bindings[0].rule_version = "1.0.1".into()
        }),
        ("owner_rule", |request| {
            request.configuration.bindings[0].candidate_kind = "DecisionRecord".into()
        }),
        ("freshness_floor", |request| {
            request.configuration.bindings[0].require_exact_source_revision = false
        }),
        ("freshness_floor", |request| {
            request.configuration.bindings[0].future_skew_seconds = 1
        }),
        ("governance_sources", |request| {
            request.configuration.bindings[0]
                .required_authored_sources
                .push("charter@1.0.0".into())
        }),
        ("governance_sources", |request| {
            request.configuration.bindings[0]
                .approval_sources
                .push("approval".into())
        }),
        ("governance_sources", |request| {
            request.configuration.bindings[0]
                .exception_sources
                .push("exception".into())
        }),
    ];
    for (condition, modify) in cases {
        let mut request = original.clone();
        modify(&mut request);
        let refusal = validate_request(request).unwrap_err();
        assert_eq!(&refusal.condition, condition);
        assert!(refusal.project.is_none());
        assert!(refusal.configuration_fingerprint.is_none());
    }
}

#[test]
fn replacement_terms_cannot_self_authorize_a_weaker_successor() {
    let cases: &[fn(&mut ReplacementAuthorityBasisV1)] = &[
        |terms| terms.explicit_local_instruction_required = false,
        |terms| terms.same_project_and_scope_only = false,
        |terms| terms.preserve_fixed_constraints = false,
        |terms| terms.require_exact_predecessor = false,
        |terms| terms.require_confirmed_cutover = false,
        |terms| terms.terms_immutable = false,
    ];
    for weaken in cases {
        let mut request =
            GovernanceRequestInputV1::standalone("request", 7_200, "Initialize exact new project.");
        weaken(&mut request.replacement_basis);
        assert_eq!(
            validate_request(request).unwrap_err().condition,
            "replacement_basis"
        );
    }
}

#[test]
fn local_instruction_is_explicit_bounded_and_claims_remain_advisory() {
    for text in [
        "".into(),
        "  \n\t".into(),
        "bad\0instruction".into(),
        "x".repeat(4097),
    ] {
        assert_eq!(
            validate_request(GovernanceRequestInputV1::standalone("request", 7_200, text))
                .unwrap_err()
                .condition,
            "explicit_instruction"
        );
    }
    let first = validate_request(GovernanceRequestInputV1::standalone(
        "request",
        7_200,
        "Initialize exact new project.",
    ))
    .unwrap();
    let mut claimed = first.input.clone();
    claimed.instruction.asserted_principal = Some("Local operator".into());
    claimed.instruction.asserted_agent = Some("Configured agent".into());
    claimed.instruction.asserted_delegation = Some("Operator instructed this agent".into());
    let second = validate_request(claimed).unwrap();
    assert_eq!(
        second.input.instruction.mode,
        AuthorityModeV1::StandaloneLocalAdvisory
    );
    assert_ne!(
        first.instruction_fingerprint,
        second.instruction_fingerprint
    );
    assert_eq!(
        first.configuration_fingerprint,
        second.configuration_fingerprint
    );
    let serialized = String::from_utf8(second.input.canonical_json_bytes().unwrap()).unwrap();
    assert!(serialized.contains("standalone_local_advisory"));
    assert!(!serialized.contains("authenticated"));
    assert!(!serialized.contains("verified_delegation"));
}

fn project_binding() -> ProjectBindingV1 {
    ProjectBindingV1 {
        repository_identity_fingerprint: format!("sha256:{}", "1".repeat(64)),
        retained_root: std::env::current_dir().unwrap().to_str().unwrap().into(),
        physical_root_fingerprint: format!("sha256:{}", "2".repeat(64)),
        registration_identity: handbook_contracts::FingerprintV1::parse(&format!(
            "sha256:{}",
            "3".repeat(64)
        ))
        .unwrap(),
    }
}

#[test]
fn request_locator_binds_project_complete_input_and_exact_ceiling() {
    let request =
        GovernanceRequestInputV1::standalone("request", 7_200, "Initialize exact new project.");
    let original = create_request_locator(project_binding(), request.clone()).unwrap();
    let bytes = original.canonical_json_bytes().unwrap();
    assert_eq!(
        GovernanceRequestLocatorV1::from_json_bytes(&bytes).unwrap(),
        original
    );
    let mut other_project = project_binding();
    other_project.repository_identity_fingerprint = format!("sha256:{}", "4".repeat(64));
    assert_ne!(
        create_request_locator(other_project, request.clone())
            .unwrap()
            .request_fingerprint,
        original.request_fingerprint
    );
    let changes: &[fn(&mut GovernanceRequestInputV1)] = &[
        |input| input.configuration.bindings[0].max_observation_age_seconds = 86_400,
        |input| input.request_key = "other-request".into(),
        |input| input.instruction.text.push_str(" Exact authorized terms."),
        |input| input.instruction.asserted_principal = Some("operator".into()),
    ];
    for change in changes {
        let mut changed = request.clone();
        change(&mut changed);
        assert_ne!(
            create_request_locator(project_binding(), changed.clone())
                .unwrap()
                .request_fingerprint,
            original.request_fingerprint
        );
        let substituted = GovernanceRequestLocatorV1 {
            request: changed,
            ..original.clone()
        };
        let refusal = validate_request_locator(&substituted).unwrap_err();
        assert_eq!(refusal.condition, "request_identity");
        assert_eq!(refusal.project.as_ref(), Some(&original.project));
        assert!(GovernanceRequestLocatorV1::from_json_bytes(
            &serde_json::to_vec(&substituted).unwrap()
        )
        .is_err());
    }
}

#[test]
fn malformed_project_locator_refuses_without_guessing_identity() {
    let request =
        GovernanceRequestInputV1::standalone("request", 7_200, "Initialize exact new project.");
    let changes: &[fn(&mut ProjectBindingV1)] = &[
        |project| project.retained_root = "relative/root".into(),
        |project| project.repository_identity_fingerprint = "unknown".into(),
        |project| project.physical_root_fingerprint = "unknown".into(),
    ];
    for change in changes {
        let mut malformed = project_binding();
        change(&mut malformed);
        let refusal = create_request_locator(malformed, request.clone()).unwrap_err();
        assert_eq!(refusal.condition, "project_binding");
        assert!(refusal.project.is_none());
        assert!(refusal.configuration_fingerprint.is_none());
    }
}

#[test]
fn shipped_rules_have_exact_distinct_identities_and_explicit_empty_source_sets() {
    let handbook = establishment_rule_descriptor_v1().unwrap();
    let contracts = freshness_admission_rule_descriptor_v1().unwrap();
    assert_eq!(
        handbook.binding.reference().as_str(),
        "handbook.admission-establishment.standalone@1.0.0"
    );
    assert_eq!(
        contracts.binding.reference().as_str(),
        "handbook.contracts.freshness-policy-admission@1.0.0"
    );
    assert_ne!(
        handbook.binding.fingerprint(),
        contracts.binding.fingerprint()
    );
    assert_eq!(
        contracts.candidate_kind.as_deref(),
        Some("EvidenceFreshnessPolicyV1")
    );
    assert_eq!(
        contracts.candidate_schema_ref.as_deref(),
        Some(handbook_contracts::EVIDENCE_FRESHNESS_POLICY_SCHEMA_REF_V1)
    );
    assert!(contracts.required_authored_sources.is_empty());
    assert!(contracts.approval_sources.is_empty());
    assert!(contracts.exception_sources.is_empty());
    assert_eq!(establishment_rule_descriptor_v1().unwrap(), handbook);
    assert_eq!(freshness_admission_rule_descriptor_v1().unwrap(), contracts);
}

#[test]
fn shipped_rule_fingerprints_match_the_versioned_canonical_fixture() {
    // Fixture independently hashes the ASCII canonical JSON content/domain.
    // Changes to these immutable descriptors require a new selected version.
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
fn serialized_authority_mode_initial_state_and_exact_source_population_are_closed() {
    let request =
        GovernanceRequestInputV1::standalone("request", 7_200, "Initialize exact new project.");
    let original = serde_json::to_value(request).unwrap();
    for mode in ["authenticated", "admin", "agent_role"] {
        let mut changed = original.clone();
        changed["instruction"]["mode"] = serde_json::json!(mode);
        assert!(parse_governance_request_json(&serde_json::to_vec(&changed).unwrap()).is_err());
    }
    let mut changed = original.clone();
    changed["expected_initial_state"] = serde_json::json!("existing_project_empty_governance");
    assert!(parse_governance_request_json(&serde_json::to_vec(&changed).unwrap()).is_err());
    let mut changed = original;
    changed["configuration"]["bindings"][0]
        .as_object_mut()
        .unwrap()
        .remove("required_authored_sources");
    assert!(parse_governance_request_json(&serde_json::to_vec(&changed).unwrap()).is_err());
}
