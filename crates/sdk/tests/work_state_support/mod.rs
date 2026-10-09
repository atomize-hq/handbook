// Supplied illustrative test choices sent to real public owners. No test authority hooks.
use handbook_engine::{
    CharterAcquisitionMode, CharterCoverageSubmission, CharterIntakeConsumer,
    CharterIntakeEnvelope, CharterIntakeSourceKind,
};
use handbook_sdk::admission_governance::DeferralRequestInputV2;
use handbook_sdk::local_check::*;
use handbook_sdk::local_setup::*;
use handbook_sdk::work_state::*;
use handbook_sdk::HandbookSdkV1;

pub fn ordinary_scope() -> WorkRegistrationScopeV1 {
    WorkRegistrationScopeV1::explicit(vec![WorkCompositionClauseV1 {
        descriptor: WorkDescriptorV1::ordinary_leaf(),
        label_max_bytes: 4096,
        work_question_max_bytes: 4096,
        allow_omitted_display_level: true,
        display_levels: vec![],
        role_permission: WorkRolePermissionV1::Ordinary {},
    }])
}

pub fn menu_scope(definition: WorkRequirementsDefinitionV1) -> WorkRegistrationScopeV1 {
    WorkRegistrationScopeV1::explicit(vec![
        WorkCompositionClauseV1 {
            descriptor: WorkDescriptorV1::ordinary_leaf(),
            label_max_bytes: 128,
            work_question_max_bytes: 256,
            allow_omitted_display_level: true,
            display_levels: vec!["task".into()],
            role_permission: WorkRolePermissionV1::Ordinary {},
        },
        WorkCompositionClauseV1 {
            descriptor: WorkDescriptorV1::planning_output(),
            label_max_bytes: 128,
            work_question_max_bytes: 256,
            allow_omitted_display_level: false,
            display_levels: vec!["planning".into()],
            role_permission: WorkRolePermissionV1::PlanningOutput {
                question_max_bytes: 256,
            },
        },
        WorkCompositionClauseV1 {
            descriptor: WorkDescriptorV1::requirements_input(),
            label_max_bytes: 128,
            work_question_max_bytes: 256,
            allow_omitted_display_level: false,
            display_levels: vec!["implementation".into()],
            role_permission: WorkRolePermissionV1::RequirementsInput {
                definitions: vec![definition],
            },
        },
    ])
}

/// Owner-built exact definition; no lifecycle, source publication or qualification.
pub fn supplied_intrinsic_definition(version: &str) -> handbook_contracts::ContractDefinitionV1 {
    use handbook_contracts::*;
    let binding = |reference: &str| {
        ExactBindingV1::new(
            ExactRefV1::parse(reference).unwrap(),
            FingerprintV1::parse(&format!("sha256:{}", "a".repeat(64))).unwrap(),
        )
    };
    let id = |value: &str| StableIdV1::parse(value).unwrap();
    let selector = |field: &str, expected: &str| {
        SelectorV1::new(
            MatcherBindingV1::exact_fields_v1(),
            vec![SelectorCriterionV1::new(id(field), expected).unwrap()],
        )
        .unwrap()
    };
    let claim = ContractClaimV1::new(ContractClaimInputV1 {
        claim_id: id("session.renewal.valid"),
        statement: "Session renewal satisfies the exact requirement.".into(),
        subject_selector: selector("subject.kind", "session"),
        case_selector: selector("case.kind", "renewal"),
        applicability_rule: ApplicabilityRuleV1::SubjectAndCaseMatch,
        gate_effect: GateEffectV1::Required,
        evidence_requirements: vec![EvidenceRequirementV1::new(
            id("handbook.evidence.schema-conformance"),
            EvidenceCardinalityV1::ExactlyOne,
            EvidenceCaseSelectionV1::ClaimCaseSelector,
            binding("handbook.stability.exact@1.0.0"),
        )
        .unwrap()],
        freshness_policy: binding("handbook.freshness.exact-revision@1.0.0"),
        minimum_resolution: ResolutionMinimumV1::new(
            binding("handbook.context-resolution.shipped-root@1.0.0"),
            id("resolution.scope.local-observation"),
            id("resolution.detail.identifier-only"),
            id("resolution.time.current-operation"),
            id("resolution.authority.read-only"),
            id("resolution.memory.operation"),
            id("resolution.validation.observation-only"),
        ),
        score_weight: None,
    })
    .unwrap();
    ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: binding("handbook.contract-definition@1.0.0"),
        contract_id: ContractIdV1::parse("handbook.contract.session-renewal").unwrap(),
        contract_version: VersionV1::parse(version).unwrap(),
        contract_kind: ContractKindV1::ClaimSet,
        claims: vec![claim],
        definition_author_ref: id("user.definition-author"),
        definition_authority: binding("handbook.authority.contract-author@1.0.0"),
        lifecycle_policy: binding("handbook.contract-lifecycle-policy.independent-lock@1.0.0"),
        matcher_bindings: vec![MatcherBindingV1::exact_fields_v1()],
        gate_policy: binding("handbook.contract-gate-policy.closed-v1@1.0.0"),
        extensions_policy: binding("handbook.contract-extensions.closed-v1@1.0.0"),
    })
    .unwrap()
}
const CHARTER: &[u8] = include_bytes!("../../../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml");
pub fn supplied_initial_request() -> LocalInitialSetupRequestV1 {
    let content = handbook_engine::parse_definition_yaml(CHARTER).unwrap();
    let mut terms = LocalOperatingTermsV1 {
        request_key: "packet2-initial-terms".into(),
        mode: "local_advisory".into(),
        rule_ref: handbook_engine::local_authority::LOCAL_OPERATING_RULE_V1.into(),
        instruction: "Validate the supplied captured order in the bounded local owner reach."
            .into(),
        principal: "packet2.human".into(),
        agent: "packet2.author".into(),
        delegation: "bounded local order schema validation".into(),
        approval_source_ref: "decision.packet2.initial-terms".into(),
        human_approval_asserted: true,
        approved_content_fingerprint: String::new(),
        ceiling: LocalEnvelopeCeilingV1 {
            active_level_id: "execution".into(),
            dimensions: [
                "assigned_unit",
                "normal",
                "immediate",
                "local_write",
                "execution",
                "unit_closeout",
            ]
            .map(str::to_owned),
        },
        constraint_inputs: vec![],
        allowed_input_paths: vec!["inputs/*".into()],
        allowed_paths: vec![
            ".handbook/state/contracts/**".into(),
            ".handbook/state/dock/**".into(),
        ],
        operation_classes: vec![
            LocalOperationClassV1::LocalValidation,
            LocalOperationClassV1::OwnerWrite,
        ],
        reserved_decisions: vec![
            LocalDecisionClassV1::PathExpansion,
            LocalDecisionClassV1::CharterReplacement,
            LocalDecisionClassV1::TermsReplacement,
        ],
        delegated_decisions: vec![LocalDecisionClassV1::InputCorrection],
        revocation_principal: "packet2.human".into(),
    };
    terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
    let coverage = [
        "project_shape.definition",
        "delivery.constraints",
        "delivery.default_implications",
        "operational_reality.production_state",
        "risk.domains",
        "engineering_posture.baseline",
        "policy.authority_and_revision",
        "governance.decision_authority",
        "governance.required_approvals",
        "governance.exception_policy",
        "engineering_posture.dimensions",
        "engineering_posture.red_lines",
        "governance.review_triggers",
        "governance.reassessment_triggers",
        "debt.register",
        "decisions.records",
    ]
    .into_iter()
    .map(|coverage_id| CharterCoverageSubmission {
        coverage_id: coverage_id.into(),
        source_kind: CharterIntakeSourceKind::UserDeclaration,
        value_ref: format!("intake-values/{coverage_id}.json"),
        evidence_refs: vec![],
        confidence: "high".into(),
        freshness: None,
        sensitivity: "internal".into(),
        contradiction_refs: vec![],
        waiver_ref: None,
    })
    .collect();
    LocalInitialSetupRequestV1 {
        deferral: DeferralRequestInputV2::human_approved(
            "packet2-deferral",
            "The supplied test human approved this exact scoped deferral.",
        ),
        charter: NativeLocalCharterRequestV1 {
            request_key: "packet2-charter".into(),
            intake: CharterIntakeEnvelope {
                mode: CharterAcquisitionMode::Express,
                content,
                coverage,
                consumer: CharterIntakeConsumer {
                    kind: "sdk_consumer".into(),
                    id: "packet2-test".into(),
                    version: "1.0.0".into(),
                },
                prompt_event_refs: vec!["decision.packet2.charter".into()],
                finalized_at_utc: "2026-10-05T12:00:00Z".into(),
                expected_current_fingerprint: None,
            },
            human_approval: LocalCharterHumanApprovalV1 {
                accepted: true,
                principal_claim: "packet2.human".into(),
                agent_claim: "packet2.author".into(),
                delegation_claim: "initial local Charter establishment".into(),
                source_ref: "decision.packet2.charter".into(),
                approved_content_fingerprint: handbook_engine::charter_source_fingerprint(CHARTER)
                    .to_string(),
            },
        },
        terms,
    }
}

pub fn establish_initial(sdk: &HandbookSdkV1) -> LocalInitialSetupLocatorV1 {
    let request = supplied_initial_request();
    let prepared = sdk.prepare_local_initial_setup(&request).unwrap();
    let locator = prepared.locator().clone();
    let outcome = sdk.establish_local_initial_setup(prepared);
    assert!(outcome.stopped.is_none(), "{outcome:?}");
    assert_eq!(
        outcome.charter.unwrap().disposition,
        LocalCharterDispositionV1::Committed
    );
    assert_eq!(
        outcome.terms.unwrap().disposition,
        LocalAuthorityDispositionV1::Committed
    );
    locator
}

pub fn supplied_successor_terms(
    initial: &LocalInitialSetupLocatorV1,
    predecessor_fingerprint: &str,
) -> LocalCheckTermsV1 {
    let original = &initial.terms.terms;
    let mut terms = LocalCheckTermsV1 {
        request_key: "packet2-compatible-local-check".into(), mode: "local_advisory".into(),
        rule_ref: LOCAL_CHECK_RULE_V1.into(),
        instruction: "Author this complete exact local order contract, have the selected reviewer review it, and resolve its captured native validator sources.".into(),
        principal: original.principal.clone(), agent: original.agent.clone(),
        delegation: "bounded author, actual review, lifecycle and captured source resolution".into(),
        approval_source_ref: "decision.packet2.compatible-successor-terms".into(),
        human_approval_asserted: true, approved_content_fingerprint: String::new(),
        expected_predecessor_fingerprint: predecessor_fingerprint.into(),
        expected_charter_fingerprint: initial.charter.canonical_fingerprint.clone(),
        ceiling: original.ceiling.clone(), constraint_inputs: original.constraint_inputs.clone(),
        allowed_paths: vec![".handbook/state/contracts/**".into(), ".handbook/state/dock/**".into()],
        allowed_input_paths: vec![".handbook/state/contracts/**".into(), ".handbook/state/dock/**".into(), ".handbook/state/locks/contract-owner.lock".into(), ".handbook/state/locks/dock-operational-store.lock".into(), "inputs/*".into()],
        action_classes: vec![LocalCheckActionV1::DefinitionAuthor, LocalCheckActionV1::Review, LocalCheckActionV1::Lifecycle, LocalCheckActionV1::EvidenceSource],
        author_participant_ref: "packet2.author".into(), review_participant_refs: vec!["packet2.reviewer".into()],
        reserved_decisions: original.reserved_decisions.clone(), delegated_decisions: original.delegated_decisions.clone(),
        revocation_principal: original.revocation_principal.clone(),
    };
    terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
    terms
}
