use handbook_contracts::{
    evaluate_applicability, ApplicabilityInputSourceV1, ApplicabilityOutcomeV1,
    ApplicabilityRuleV1, ContractClaimInputV1, ContractClaimV1, ContractError,
    EvidenceCardinalityV1, EvidenceCaseSelectionV1, EvidenceRequirementV1, ExactBindingV1,
    ExactRefV1, FingerprintV1, GateEffectV1, MatcherBindingV1, MatcherInputCandidateV1,
    MatcherValueV1, ResolutionMinimumV1, ScoreWeightV1, SelectorCriterionV1, SelectorV1,
    StableIdV1,
};

fn id(value: &str) -> StableIdV1 {
    StableIdV1::parse(value).unwrap()
}

fn binding(value: &str, hex: char) -> ExactBindingV1 {
    ExactBindingV1::new(
        ExactRefV1::parse(value).unwrap(),
        FingerprintV1::parse(&format!("sha256:{}", hex.to_string().repeat(64))).unwrap(),
    )
}

fn selector(field: &str, expected: &str) -> SelectorV1 {
    SelectorV1::new(
        MatcherBindingV1::exact_fields_v1(),
        vec![SelectorCriterionV1::new(id(field), expected).unwrap()],
    )
    .unwrap()
}

fn requirement(cardinality: EvidenceCardinalityV1) -> EvidenceRequirementV1 {
    EvidenceRequirementV1::new(
        id("handbook.evidence.schema-conformance"),
        cardinality,
        EvidenceCaseSelectionV1::ClaimCaseSelector,
        binding("handbook.stability.exact@1.0.0", '7'),
    )
    .unwrap()
}

fn claim() -> ContractClaimV1 {
    ContractClaimV1::new(ContractClaimInputV1 {
        claim_id: id("schema.payload.valid"),
        statement: "The payload conforms to its exact schema.".to_owned(),
        subject_selector: selector("subject.kind", "artifact"),
        case_selector: selector("case.kind", "positive"),
        applicability_rule: ApplicabilityRuleV1::SubjectAndCaseMatch,
        gate_effect: GateEffectV1::Required,
        evidence_requirements: vec![requirement(EvidenceCardinalityV1::ExactlyOne)],
        freshness_policy: binding("handbook.freshness.exact-revision@1.0.0", '8'),
        minimum_resolution: ResolutionMinimumV1::new(
            binding("handbook.context-resolution.shipped-root@1.0.0", '9'),
            id("resolution.scope.local-observation"),
            id("resolution.detail.identifier-only"),
            id("resolution.time.current-operation"),
            id("resolution.authority.read-only"),
            id("resolution.memory.operation"),
            id("resolution.validation.observation-only"),
        ),
        score_weight: Some(ScoreWeightV1::new(1).unwrap()),
    })
    .unwrap()
}

#[test]
fn claim_kernel_enforces_nonempty_all_of_requirements_and_numeric_bounds() {
    let mut input = ContractClaimInputV1 {
        claim_id: id("schema.payload.valid"),
        statement: "The payload conforms.".to_owned(),
        subject_selector: selector("subject.kind", "artifact"),
        case_selector: selector("case.kind", "positive"),
        applicability_rule: ApplicabilityRuleV1::SubjectAndCaseMatch,
        gate_effect: GateEffectV1::HardFail,
        evidence_requirements: Vec::new(),
        freshness_policy: binding("handbook.freshness.exact-revision@1.0.0", '8'),
        minimum_resolution: ResolutionMinimumV1::new(
            binding("handbook.context-resolution.shipped-root@1.0.0", '9'),
            id("resolution.scope.local-observation"),
            id("resolution.detail.identifier-only"),
            id("resolution.time.current-operation"),
            id("resolution.authority.read-only"),
            id("resolution.memory.operation"),
            id("resolution.validation.observation-only"),
        ),
        score_weight: None,
    };
    assert_eq!(
        ContractClaimV1::new(input.clone()),
        Err(ContractError::Empty)
    );

    input.evidence_requirements = vec![
        requirement(EvidenceCardinalityV1::ExactlyOne),
        requirement(EvidenceCardinalityV1::AtLeastOne),
    ];
    assert_eq!(
        ContractClaimV1::new(input)
            .unwrap()
            .evidence_requirements()
            .len(),
        2
    );
    assert!(ScoreWeightV1::new(0).is_err());
    assert!(ScoreWeightV1::new(9_007_199_254_740_991).is_ok());
    assert!(ScoreWeightV1::new(9_007_199_254_740_992).is_err());
}

#[test]
fn exact_case_sets_are_nonempty_unique_and_required_for_all_declared_cases() {
    assert!(EvidenceCaseSelectionV1::exact_cases(Vec::new()).is_err());
    assert!(EvidenceCaseSelectionV1::exact_cases(vec![id("case.one"), id("case.one")]).is_err());

    assert!(EvidenceRequirementV1::new(
        id("handbook.evidence.schema-conformance"),
        EvidenceCardinalityV1::AllDeclaredCases,
        EvidenceCaseSelectionV1::ClaimCaseSelector,
        binding("handbook.stability.exact@1.0.0", '7'),
    )
    .is_err());
    assert!(EvidenceRequirementV1::new(
        id("handbook.evidence.schema-conformance"),
        EvidenceCardinalityV1::AllDeclaredCases,
        EvidenceCaseSelectionV1::exact_cases(vec![id("case.one"), id("case.two")]).unwrap(),
        binding("handbook.stability.exact@1.0.0", '7'),
    )
    .is_ok());
}

#[test]
fn closed_values_and_substituted_matchers_refuse() {
    assert!(serde_json::from_str::<GateEffectV1>("\"passed\"").is_err());
    assert!(serde_json::from_str::<EvidenceCardinalityV1>("\"newest_wins\"").is_err());

    let registered = MatcherBindingV1::exact_fields_v1();
    let substituted = MatcherBindingV1::new(
        registered.matcher().clone(),
        binding("handbook.matcher-input.exact-fields@1.0.0", 'f'),
    );
    assert_eq!(
        SelectorV1::new(
            substituted,
            vec![SelectorCriterionV1::new(id("subject.kind"), "artifact").unwrap()],
        ),
        Err(ContractError::UnknownMatcher)
    );
}

#[test]
fn selector_criteria_are_nonempty_unique_and_normalized() {
    let matcher = MatcherBindingV1::exact_fields_v1();
    assert_eq!(
        SelectorV1::new(matcher.clone(), Vec::new()),
        Err(ContractError::Empty)
    );
    assert!(SelectorV1::new(
        matcher,
        vec![
            SelectorCriterionV1::new(id("subject.kind"), "artifact").unwrap(),
            SelectorCriterionV1::new(id("subject.kind"), "other").unwrap(),
        ],
    )
    .is_err());
}

#[test]
fn applicability_is_exact_declarative_and_fail_closed() {
    let claim = claim();
    let schema = MatcherBindingV1::exact_fields_v1().input_schema().clone();
    let subject = MatcherInputCandidateV1::current(
        ApplicabilityInputSourceV1::OwnerRegistry,
        schema.clone(),
        vec![MatcherValueV1::new(id("subject.kind"), "artifact").unwrap()],
    )
    .unwrap();
    let case = MatcherInputCandidateV1::current(
        ApplicabilityInputSourceV1::OwnerRegistry,
        schema.clone(),
        vec![MatcherValueV1::new(id("case.kind"), "positive").unwrap()],
    )
    .unwrap();
    assert_eq!(
        evaluate_applicability(&claim, &subject, &case),
        ApplicabilityOutcomeV1::Applicable
    );

    let false_case = MatcherInputCandidateV1::current(
        ApplicabilityInputSourceV1::OwnerRegistry,
        schema.clone(),
        vec![MatcherValueV1::new(id("case.kind"), "negative").unwrap()],
    )
    .unwrap();
    assert_eq!(
        evaluate_applicability(&claim, &subject, &false_case),
        ApplicabilityOutcomeV1::NotApplicable
    );

    for blocked in [
        MatcherInputCandidateV1::stale(ApplicabilityInputSourceV1::OwnerRegistry, schema.clone()),
        MatcherInputCandidateV1::unavailable(
            ApplicabilityInputSourceV1::OwnerRegistry,
            schema.clone(),
        ),
        MatcherInputCandidateV1::indeterminate(
            ApplicabilityInputSourceV1::OwnerRegistry,
            schema.clone(),
        ),
        MatcherInputCandidateV1::current(
            ApplicabilityInputSourceV1::Dock,
            schema.clone(),
            vec![MatcherValueV1::new(id("case.kind"), "positive").unwrap()],
        )
        .unwrap(),
        MatcherInputCandidateV1::current(
            ApplicabilityInputSourceV1::Ambient,
            schema.clone(),
            vec![MatcherValueV1::new(id("case.kind"), "positive").unwrap()],
        )
        .unwrap(),
        MatcherInputCandidateV1::current(
            ApplicabilityInputSourceV1::Executable,
            schema,
            vec![MatcherValueV1::new(id("case.kind"), "positive").unwrap()],
        )
        .unwrap(),
    ] {
        assert_eq!(
            evaluate_applicability(&claim, &subject, &blocked),
            ApplicabilityOutcomeV1::Blocked
        );
    }
}

#[test]
fn blocked_input_dominates_a_proven_false_selector() {
    let claim = claim();
    let schema = MatcherBindingV1::exact_fields_v1().input_schema().clone();
    let false_subject = MatcherInputCandidateV1::current(
        ApplicabilityInputSourceV1::OwnerRegistry,
        schema.clone(),
        vec![MatcherValueV1::new(id("subject.kind"), "other").unwrap()],
    )
    .unwrap();
    let stale_case =
        MatcherInputCandidateV1::stale(ApplicabilityInputSourceV1::OwnerRegistry, schema);

    assert_eq!(
        evaluate_applicability(&claim, &false_subject, &stale_case),
        ApplicabilityOutcomeV1::Blocked
    );
}

#[test]
fn boundary_collections_and_strings_refuse_before_unbounded_growth() {
    let matcher = MatcherBindingV1::exact_fields_v1();
    let criteria = (0..65)
        .map(|index| {
            SelectorCriterionV1::new(id(&format!("subject.field-{index}")), "value").unwrap()
        })
        .collect();
    assert_eq!(
        SelectorV1::new(matcher.clone(), criteria),
        Err(ContractError::BoundExceeded)
    );

    let values = (0..129)
        .map(|index| MatcherValueV1::new(id(&format!("subject.field-{index}")), "value").unwrap())
        .collect();
    assert_eq!(
        MatcherInputCandidateV1::current(
            ApplicabilityInputSourceV1::OwnerRegistry,
            matcher.input_schema().clone(),
            values,
        ),
        Err(ContractError::BoundExceeded)
    );

    assert!(SelectorCriterionV1::new(id("subject.kind"), &"x".repeat(1025)).is_err());
    let oversized = claim();
    let mut value = serde_json::to_value(&oversized).unwrap();
    value["statement"] = serde_json::json!("x".repeat(4097));
    assert!(serde_json::from_value::<ContractClaimV1>(value).is_err());

    // Public errors classify the boundary without reflecting rejected content.
    let hostile = "secret-attacker-controlled-value";
    let error = StableIdV1::parse(hostile).unwrap_err().to_string();
    assert!(!error.contains(hostile));

    // Keep this binding live so the test also proves the two registered
    // identity domains cannot accidentally collapse into one fingerprint.
    assert_ne!(
        matcher.matcher().fingerprint(),
        matcher.input_schema().fingerprint()
    );
    assert_eq!(oversized.claim_id().as_str(), "schema.payload.valid");
}
