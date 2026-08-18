use handbook_contracts::{
    classify_definition_mutation, ApplicabilityRuleV1, ContractClaimInputV1, ContractClaimV1,
    ContractDefinitionInputV1, ContractDefinitionV1, ContractIdV1, ContractKindV1,
    DefinitionMutationClassificationV1, EvidenceCardinalityV1, EvidenceCaseSelectionV1,
    EvidenceRequirementV1, ExactBindingV1, ExactRefV1, FingerprintV1, GateEffectV1,
    MatcherBindingV1, ResolutionMinimumV1, ScoreWeightV1, SelectorCriterionV1, SelectorV1,
    StableIdV1, VersionV1, CONTRACT_OWNER_SOURCE_REF_V1,
};
use serde_json::{json, Value};

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

fn claim(claim_id: &str, gate_effect: GateEffectV1, score: Option<u64>) -> ContractClaimV1 {
    ContractClaimV1::new(ContractClaimInputV1 {
        claim_id: id(claim_id),
        statement: format!("Normative statement for {claim_id}."),
        subject_selector: selector("subject.kind", "artifact"),
        case_selector: selector("case.kind", "positive"),
        applicability_rule: ApplicabilityRuleV1::SubjectAndCaseMatch,
        gate_effect,
        evidence_requirements: vec![EvidenceRequirementV1::new(
            id("handbook.evidence.schema-conformance"),
            EvidenceCardinalityV1::ExactlyOne,
            EvidenceCaseSelectionV1::ClaimCaseSelector,
            binding("handbook.stability.exact@1.0.0", '7'),
        )
        .unwrap()],
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
        score_weight: score.map(|value| ScoreWeightV1::new(value).unwrap()),
    })
    .unwrap()
}

fn definition(version: &str, claims: Vec<ContractClaimV1>) -> ContractDefinitionV1 {
    ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: binding("handbook.contract-definition@1.0.0", '1'),
        contract_id: ContractIdV1::parse("handbook.contract.example").unwrap(),
        contract_version: VersionV1::parse(version).unwrap(),
        contract_kind: ContractKindV1::ClaimSet,
        claims,
        definition_author_ref: id("user.definition-author"),
        definition_authority: binding("handbook.authority.contract-author@1.0.0", '2'),
        lifecycle_policy: binding(
            "handbook.contract-lifecycle-policy.independent-lock@1.0.0",
            '3',
        ),
        matcher_bindings: vec![MatcherBindingV1::exact_fields_v1()],
        gate_policy: binding("handbook.contract-gate-policy.closed-v1@1.0.0", '4'),
        extensions_policy: binding("handbook.contract-extensions.closed-v1@1.0.0", '5'),
    })
    .unwrap()
}

#[test]
fn immutable_construction_derives_exact_ref_fingerprint_and_canonical_bytes() {
    let definition = definition(
        "1.0.0",
        vec![claim(
            "schema.payload.valid",
            GateEffectV1::Required,
            Some(2),
        )],
    );

    assert_eq!(
        CONTRACT_OWNER_SOURCE_REF_V1,
        "handbook.owner.contracts@0.1.0"
    );
    assert_eq!(
        definition.contract_ref().as_str(),
        "handbook.contract.example@1.0.0"
    );
    assert!(definition
        .definition_fingerprint()
        .as_str()
        .starts_with("sha256:"));
    let bytes = definition.canonical_json_bytes().unwrap();
    assert_eq!(
        serde_json::from_slice::<ContractDefinitionV1>(&bytes).unwrap(),
        definition
    );
}

#[test]
fn ordered_claims_are_unique_and_order_changes_the_fingerprint() {
    let first = claim("schema.first.valid", GateEffectV1::Required, None);
    let second = claim("schema.second.valid", GateEffectV1::Advisory, None);
    assert!(ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: binding("handbook.contract-definition@1.0.0", '1'),
        contract_id: ContractIdV1::parse("handbook.contract.example").unwrap(),
        contract_version: VersionV1::parse("1.0.0").unwrap(),
        contract_kind: ContractKindV1::ClaimSet,
        claims: vec![first.clone(), first.clone()],
        definition_author_ref: id("user.definition-author"),
        definition_authority: binding("handbook.authority.contract-author@1.0.0", '2'),
        lifecycle_policy: binding(
            "handbook.contract-lifecycle-policy.independent-lock@1.0.0",
            '3',
        ),
        matcher_bindings: vec![MatcherBindingV1::exact_fields_v1()],
        gate_policy: binding("handbook.contract-gate-policy.closed-v1@1.0.0", '4'),
        extensions_policy: binding("handbook.contract-extensions.closed-v1@1.0.0", '5'),
    })
    .is_err());

    let forward = definition("1.0.0", vec![first.clone(), second.clone()]);
    let reverse = definition("1.0.0", vec![second, first]);
    assert_ne!(
        forward.definition_fingerprint(),
        reverse.definition_fingerprint()
    );
}

#[test]
fn changed_semantics_cannot_retain_an_old_ref_or_fingerprint() {
    let definition = definition(
        "1.0.0",
        vec![claim("schema.payload.valid", GateEffectV1::Required, None)],
    );
    let mut value = serde_json::to_value(&definition).unwrap();
    value["claims"][0]["statement"] = json!("Changed normative semantics.");
    assert!(serde_json::from_value::<ContractDefinitionV1>(value).is_err());

    let mut stale_ref = serde_json::to_value(&definition).unwrap();
    stale_ref["contract_ref"] = json!("handbook.contract.example@1.0.1");
    assert!(serde_json::from_value::<ContractDefinitionV1>(stale_ref).is_err());
}

#[test]
fn fingerprint_closes_over_every_top_level_semantic_family() {
    let definition = definition(
        "1.0.0",
        vec![claim("schema.payload.valid", GateEffectV1::Required, None)],
    );
    let baseline = serde_json::to_value(&definition).unwrap();
    let mutations: Vec<(&str, Value)> = vec![
        (
            "definition_schema",
            json!({"ref":"handbook.contract-definition@1.0.0","fingerprint":format!("sha256:{}", "a".repeat(64))}),
        ),
        ("definition_author_ref", json!("user.other-author")),
        (
            "definition_authority",
            json!({"ref":"handbook.authority.contract-author@1.0.0","fingerprint":format!("sha256:{}", "a".repeat(64))}),
        ),
        (
            "lifecycle_policy",
            json!({"ref":"handbook.contract-lifecycle-policy.independent-lock@1.0.0","fingerprint":format!("sha256:{}", "a".repeat(64))}),
        ),
        (
            "gate_policy",
            json!({"ref":"handbook.contract-gate-policy.closed-v1@1.0.0","fingerprint":format!("sha256:{}", "a".repeat(64))}),
        ),
        (
            "extensions_policy",
            json!({"ref":"handbook.contract-extensions.closed-v1@1.0.0","fingerprint":format!("sha256:{}", "a".repeat(64))}),
        ),
    ];
    for (field, replacement) in mutations {
        let mut changed = baseline.clone();
        changed[field] = replacement;
        assert!(
            serde_json::from_value::<ContractDefinitionV1>(changed).is_err(),
            "old fingerprint survived {field} mutation"
        );
    }

    let mut matcher_changed = baseline;
    matcher_changed["matcher_bindings"][0]["matcher"]["fingerprint"] =
        json!(format!("sha256:{}", "a".repeat(64)));
    assert!(serde_json::from_value::<ContractDefinitionV1>(matcher_changed).is_err());
}

#[test]
fn unknown_fields_values_null_fractional_and_over_limit_weights_refuse() {
    let definition = definition(
        "1.0.0",
        vec![claim("schema.payload.valid", GateEffectV1::Required, None)],
    );
    let baseline = serde_json::to_value(definition).unwrap();

    let mut unknown_field = baseline.clone();
    unknown_field["verdict"] = json!("pass");
    assert!(serde_json::from_value::<ContractDefinitionV1>(unknown_field).is_err());

    let mut unknown_kind = baseline.clone();
    unknown_kind["contract_kind"] = json!("dock_owned");
    assert!(serde_json::from_value::<ContractDefinitionV1>(unknown_kind).is_err());

    for weight in [
        json!(null),
        json!(1.5),
        json!(-1),
        json!(9_007_199_254_740_992_u64),
    ] {
        let mut changed = baseline.clone();
        changed["claims"][0]["score_weight"] = weight;
        assert!(serde_json::from_value::<ContractDefinitionV1>(changed).is_err());
    }
}

#[test]
fn compatibility_classification_never_authorizes_exact_substitution() {
    let previous = definition(
        "1.0.0",
        vec![claim("schema.payload.valid", GateEffectV1::Required, None)],
    );
    let minor = definition(
        "1.1.0",
        vec![
            claim("schema.payload.valid", GateEffectV1::Required, None),
            claim("schema.payload.documented", GateEffectV1::Advisory, None),
        ],
    );
    assert_eq!(
        classify_definition_mutation(&previous, &minor),
        DefinitionMutationClassificationV1::MinorMinimum
    );
    assert!(!previous.matches_exact(minor.contract_ref(), minor.definition_fingerprint()));

    let weighted = definition(
        "2.0.0",
        vec![
            claim("schema.payload.valid", GateEffectV1::Required, None),
            claim("schema.payload.documented", GateEffectV1::Advisory, Some(1)),
        ],
    );
    assert_eq!(
        classify_definition_mutation(&previous, &weighted),
        DefinitionMutationClassificationV1::MajorMinimum
    );
}

#[test]
fn definition_claim_count_is_bounded() {
    let claims = (0..129)
        .map(|index| {
            claim(
                &format!("schema.claim-{index}.valid"),
                GateEffectV1::Advisory,
                None,
            )
        })
        .collect();
    let result = ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: binding("handbook.contract-definition@1.0.0", '1'),
        contract_id: ContractIdV1::parse("handbook.contract.example").unwrap(),
        contract_version: VersionV1::parse("1.0.0").unwrap(),
        contract_kind: ContractKindV1::ClaimSet,
        claims,
        definition_author_ref: id("user.definition-author"),
        definition_authority: binding("handbook.authority.contract-author@1.0.0", '2'),
        lifecycle_policy: binding(
            "handbook.contract-lifecycle-policy.independent-lock@1.0.0",
            '3',
        ),
        matcher_bindings: vec![MatcherBindingV1::exact_fields_v1()],
        gate_policy: binding("handbook.contract-gate-policy.closed-v1@1.0.0", '4'),
        extensions_policy: binding("handbook.contract-extensions.closed-v1@1.0.0", '5'),
    });
    assert!(result.is_err());
}
