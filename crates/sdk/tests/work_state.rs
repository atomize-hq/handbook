//! Real owner workflow using public production dependency APIs only. This target
//! is also compiled and run from an isolated Cargo package without test-support.
#[allow(dead_code)]
mod local_contract_support;
mod work_state_support;

use handbook_sdk::admission_governance::*;
use handbook_sdk::local_check::*;
use handbook_sdk::local_setup::{LocalBindingV1, LocalInitialSetupLocatorV1};
use handbook_sdk::work_state::*;
use handbook_sdk::{HandbookSdkV1, SetupMode, SetupRequest};
use std::fs;
use tempfile::tempdir;
use work_state_support::{establish_initial, menu_scope, ordinary_scope, supplied_successor_terms};

fn approve_scope(
    check: &LocalCheckLocatorV1,
    key: &str,
    scope: WorkRegistrationScopeV1,
) -> WorkRegistrationTermsV1 {
    let mut terms = supplied_terms(check, key);
    terms.scope = scope;
    terms.instruction = "Human approves this exact finite registration scope.".into();
    terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
    terms
}

fn establish_scope(
    sdk: &HandbookSdkV1,
    initial: &LocalInitialSetupLocatorV1,
    check: &LocalCheckLocatorV1,
    terms: WorkRegistrationTermsV1,
    transition: WorkTermsTransitionV1<'_>,
) -> WorkGrantLocatorV1 {
    let prepared = sdk
        .prepare_work_registration_terms(initial, check, terms, transition)
        .unwrap();
    let original =
        WorkGrantLocatorV1::from_json_bytes(&prepared.locator().canonical_json_bytes().unwrap())
            .unwrap();
    let execution = sdk.activate_work_registration_terms(prepared);
    assert_eq!(
        execution.disposition,
        WorkDispositionV1::Committed,
        "{execution:?}"
    );
    assert_eq!(execution.outcome.unwrap().locator, original);
    original
}

fn menu_input(grant: &WorkGrantLocatorV1, role: usize, key: &str) -> WorkRegistrationInputV1 {
    let mut value = input(grant, key, key);
    match role {
        0 => value.display_level = Some("task".into()),
        1 => {
            value.display_level = Some("planning".into());
            value.composition =
                WorkCompositionV1::planning_output("What exact requirements must renewal satisfy?");
        }
        2 => {
            value.display_level = Some("implementation".into());
            value.composition = WorkCompositionV1::requirements_input(
                work_requirements_definition_from_intrinsic_v1(
                    &work_state_support::supplied_intrinsic_definition("1.0.0"),
                ),
            );
        }
        _ => unreachable!(),
    }
    value
}

fn establish_governance(sdk: &HandbookSdkV1, initial: &LocalInitialSetupLocatorV1) {
    let ProjectAdmissionScopeBasisV3::DeferredV2(basis) = sdk
        .resolve_project_admission_scope_v3(
            &ProjectAdmissionBindingV3::ScopedChoiceV2(initial.deferral.project.clone()),
            ADMISSION_GOVERNANCE_SCOPE_V1,
            None,
        )
        .unwrap()
    else {
        panic!("D required")
    };
    let request = DeferredEstablishmentRequestV3::human_approved(
        initial.deferral.project.clone(),
        ExpectedDeferredBasisV2 {
            outcome_fingerprint: basis.outcome_fingerprint,
            relation_fingerprint: basis.relation_fingerprint,
        },
        60,
        "menu-governance",
        "Human approves the supplied exact G basis.",
    )
    .unwrap();
    assert!(matches!(
        sdk.establish_deferred_project_admission_governance(
            sdk.prepare_deferred_project_admission_governance(&request)
                .unwrap(),
        ),
        DeferredEstablishmentExecutionV3::Committed { .. }
    ));
}

#[test]
fn one_sdk_path_first_establishes_all_three_compositions_on_d_and_g() {
    for on_g in [false, true] {
        let dir = tempdir().unwrap();
        let sdk = HandbookSdkV1::open(dir.path());
        let (initial, check) = setup(&sdk);
        if on_g {
            establish_governance(&sdk, &initial);
        }
        let definition = work_requirements_definition_from_intrinsic_v1(
            &work_state_support::supplied_intrinsic_definition("1.0.0"),
        );
        let grant = establish_scope(
            &sdk,
            &initial,
            &check,
            approve_scope(&check, "menu-initial", menu_scope(definition)),
            WorkTermsTransitionV1::Initial,
        );
        let mut receipts = Vec::new();
        for role in 0..3 {
            let value = menu_input(&grant, role, &format!("menu-{role}"));
            let prepared = sdk
                .prepare_work_registration(&initial, &grant, value.clone())
                .unwrap();
            let original = WorkRequestLocatorV1::from_json_bytes(
                &prepared.locator().canonical_json_bytes().unwrap(),
            )
            .unwrap();
            let execution = sdk.register_work_item(prepared);
            assert_eq!(
                execution.disposition,
                WorkDispositionV1::Committed,
                "{execution:?}"
            );
            let receipt = execution.outcome.unwrap();
            assert_eq!(receipt.locator, original);
            assert_eq!(receipt.snapshot.role, value.composition.role);
            assert_eq!(receipt.snapshot.display_level, value.display_level);
            assert_eq!(receipt.snapshot.state, "open");
            assert_eq!(
                receipt.snapshot.descriptor.completion_support,
                "unsupported_not_assessed"
            );
            receipts.push(receipt);
        }
        drop(sdk);
        let reopened = HandbookSdkV1::open(dir.path());
        for receipt in receipts {
            assert_eq!(
                reopened
                    .resolve_current_work_item(&grant, &receipt.locator.input.item_key)
                    .unwrap(),
                WorkCurrentItemV1::Open(receipt.snapshot.clone())
            );
            assert_eq!(
                reopened
                    .resolve_work_registration_request(&receipt.locator)
                    .outcome
                    .unwrap(),
                receipt
            );
            assert_eq!(
                reopened
                    .recover_work_registration_request(&receipt.locator)
                    .outcome
                    .unwrap(),
                receipt
            );
            let completion = reopened
                .inspect_work_completion_support(&receipt.locator)
                .unwrap();
            assert_eq!(
                (
                    completion.completion_support.as_str(),
                    completion.assessment.as_str()
                ),
                ("unsupported", "not_assessed")
            );
        }
    }
}

#[test]
fn exact_scope_successors_refuse_implicit_expansion_and_unapproved_parameters() {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let ordinary = activate(&sdk, &initial, &check, "ordinary", None);
    for role in 0..3 {
        assert!(sdk
            .prepare_work_registration(
                &initial,
                &ordinary,
                menu_input(&ordinary, role, "unapproved")
            )
            .is_err());
    }
    let original = register(
        &sdk,
        &initial,
        &ordinary,
        "ordinary-request",
        "ordinary-item",
    );
    let definition = work_requirements_definition_from_intrinsic_v1(
        &work_state_support::supplied_intrinsic_definition("1.0.0"),
    );
    let scope = menu_scope(definition);
    let prepared = sdk
        .prepare_work_registration_terms(
            &initial,
            &check,
            approve_scope(&check, "menu-successor", scope.clone()),
            WorkTermsTransitionV1::ScopeChange(&ordinary),
        )
        .unwrap();
    let candidate = prepared.locator().clone();
    assert!(sdk
        .prepare_work_registration(&initial, &candidate, menu_input(&candidate, 1, "candidate"))
        .is_err());
    let execution = sdk.activate_work_registration_terms(prepared);
    assert_eq!(
        execution.disposition,
        WorkDispositionV1::Committed,
        "{execution:?}"
    );
    let menu = execution.outcome.unwrap().locator;
    assert!(sdk
        .resolve_current_work_registration_terms(&initial, &ordinary)
        .is_err());
    sdk.resolve_current_work_registration_terms(&initial, &menu)
        .unwrap();
    assert_eq!(
        sdk.recover_work_registration_request(&original.locator)
            .outcome
            .unwrap(),
        original
    );
    for case in 0..10 {
        let mut request = menu_input(
            &menu,
            if case < 5 { 1 } else { 2 },
            &format!("refuse-{case}"),
        );
        match case {
            0 => request.label = "x".repeat(129),
            1 => request.question = "x".repeat(257),
            2 => request.display_level = Some("bounded-but-unapproved".into()),
            3 => request.display_level = None,
            4 => request.composition = WorkCompositionV1::planning_output("x".repeat(257)),
            5 => {
                if let Some(WorkRoleValueV1::RequirementsInput { definition }) =
                    &mut request.composition.role
                {
                    definition.reference = definition.reference.replace("1.0.0", "2.0.0");
                }
            }
            6 => {
                if let Some(WorkRoleValueV1::RequirementsInput { definition }) =
                    &mut request.composition.role
                {
                    definition.fingerprint = format!("sha256:{}", "b".repeat(64));
                }
            }
            7 => {
                if let Some(WorkRoleValueV1::RequirementsInput { definition }) =
                    &mut request.composition.role
                {
                    definition.reference = "handbook.contract.session-renewal@latest".into();
                }
            }
            8 => {
                if let Some(WorkRoleValueV1::RequirementsInput { definition }) =
                    &mut request.composition.role
                {
                    definition.family = WorkDefinitionFamilyV1::FullLocalV3;
                    definition.schema_ref = definition.family.schema_ref().into();
                }
            }
            _ => request.composition.role = None,
        }
        assert!(
            sdk.prepare_work_registration(&initial, &menu, request)
                .is_err(),
            "case {case}"
        );
    }
    let mut fixed = approve_scope(&check, "reassignment", scope.clone());
    fixed.agent = "another.agent".into();
    fixed.approved_content_fingerprint = fixed.fingerprint_approved_content().unwrap();
    assert!(sdk
        .prepare_work_registration_terms(
            &initial,
            &check,
            fixed,
            WorkTermsTransitionV1::ScopeChange(&menu)
        )
        .is_err());
    assert!(sdk
        .prepare_work_registration_terms(
            &initial,
            &check,
            approve_scope(&check, "stale-successor", scope.clone()),
            WorkTermsTransitionV1::ScopeChange(&ordinary)
        )
        .is_err());
    let old_preparation = sdk
        .prepare_work_registration(&initial, &menu, menu_input(&menu, 1, "old-preparation"))
        .unwrap();
    let WorkRegistrationScopeV1::Explicit(mut narrower) = scope else {
        unreachable!()
    };
    narrower
        .clauses
        .retain(|clause| clause.descriptor != WorkDescriptorV1::planning_output());
    narrower.clauses[0].label_max_bytes = 64;
    let current = establish_scope(
        &sdk,
        &initial,
        &check,
        approve_scope(
            &check,
            "narrower",
            WorkRegistrationScopeV1::Explicit(narrower),
        ),
        WorkTermsTransitionV1::ScopeChange(&menu),
    );
    assert_eq!(
        sdk.register_work_item(old_preparation).disposition,
        WorkDispositionV1::Refused
    );
    assert!(sdk
        .prepare_work_registration(
            &initial,
            &current,
            menu_input(&current, 1, "removed-planning")
        )
        .is_err());
    assert_eq!(
        sdk.resolve_work_registration_terms_request(&menu)
            .disposition,
        WorkDispositionV1::Replayed
    );
    assert_eq!(
        sdk.resolve_current_work_item(&ordinary, "ordinary-item")
            .unwrap(),
        WorkCurrentItemV1::Open(original.snapshot)
    );
}

#[test]
fn latest_scope_after_multiple_revisions_is_the_only_unchanged_d_to_g_subject() {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let first = activate(&sdk, &initial, &check, "first", None);
    let definition = work_requirements_definition_from_intrinsic_v1(
        &work_state_support::supplied_intrinsic_definition("1.0.0"),
    );
    let second = establish_scope(
        &sdk,
        &initial,
        &check,
        approve_scope(&check, "second", menu_scope(definition)),
        WorkTermsTransitionV1::ScopeChange(&first),
    );
    let WorkRegistrationScopeV1::Explicit(mut scope) = second.terms.scope.clone() else {
        unreachable!()
    };
    scope.clauses[0].label_max_bytes = 96;
    let third = establish_scope(
        &sdk,
        &initial,
        &check,
        approve_scope(&check, "third", WorkRegistrationScopeV1::Explicit(scope)),
        WorkTermsTransitionV1::ScopeChange(&second),
    );
    let prepared = sdk
        .prepare_work_registration(&initial, &third, menu_input(&third, 2, "old-d-input"))
        .unwrap();
    let original = prepared.locator().clone();
    establish_governance(&sdk, &initial);
    let g_terms = approve_scope(&check, "g", third.terms.scope.clone());
    assert!(sdk
        .prepare_work_registration_terms(
            &initial,
            &check,
            g_terms.clone(),
            WorkTermsTransitionV1::EstablishFromDeferred(&second)
        )
        .is_err());
    let mut changed = g_terms.clone();
    changed.scope = ordinary_scope();
    changed.approved_content_fingerprint = changed.fingerprint_approved_content().unwrap();
    assert!(sdk
        .prepare_work_registration_terms(
            &initial,
            &check,
            changed,
            WorkTermsTransitionV1::EstablishFromDeferred(&third)
        )
        .is_err());
    let g = establish_scope(
        &sdk,
        &initial,
        &check,
        g_terms,
        WorkTermsTransitionV1::EstablishFromDeferred(&third),
    );
    assert_eq!(g.terms.scope, third.terms.scope);
    assert_eq!(
        sdk.register_work_item(prepared).disposition,
        WorkDispositionV1::Refused
    );
    assert_eq!(
        sdk.recover_work_registration_request(&original).disposition,
        WorkDispositionV1::Unaccepted
    );
    let prepared = sdk
        .prepare_work_registration(&initial, &g, menu_input(&g, 2, "g-input"))
        .unwrap();
    assert_eq!(
        sdk.register_work_item(prepared).disposition,
        WorkDispositionV1::Committed
    );
    let revoke = sdk
        .prepare_work_registration_revocation(
            &g,
            "revoke-latest",
            &g.terms.revocation_principal,
            "human.revoke",
            "Revoke the exact latest G grant.",
        )
        .unwrap();
    assert_eq!(
        sdk.revoke_work_registration_terms(&revoke).disposition,
        WorkDispositionV1::Committed
    );
    for transition in [
        WorkTermsTransitionV1::Initial,
        WorkTermsTransitionV1::ScopeChange(&g),
        WorkTermsTransitionV1::EstablishFromDeferred(&g),
    ] {
        assert!(sdk
            .prepare_work_registration_terms(
                &initial,
                &check,
                approve_scope(&check, "restore", g.terms.scope.clone()),
                transition
            )
            .is_err());
    }
}

#[test]
fn requirements_helpers_copy_the_full_owner_binding_for_each_supported_family() {
    use handbook_contracts::*;
    let author = ExactBindingV1::new(
        ExactRefV1::parse("handbook.authority.supplied@1.0.0").unwrap(),
        FingerprintV1::parse(&format!("sha256:{}", "a".repeat(64))).unwrap(),
    );
    let (local, stack) = local_contract_support::supplied_definition(
        LocalPermittedUseV2::DecisionSeekingEligible,
        3600,
        author,
    );
    let full = LocalContractDefinitionV3::new(
        LocalContractDefinitionInputV3 {
            intrinsic_definition: local.clone(),
            dependencies: LocalDefinitionDependenciesV3::new(local.dependencies().clone()).unwrap(),
        },
        &stack,
    )
    .unwrap();
    let intrinsic = work_requirements_definition_from_intrinsic_v1(local.semantic_definition());
    let local_locator = work_requirements_definition_from_local_v2(&local);
    let full_locator = work_requirements_definition_from_full_local_v3(&full);
    assert_eq!(
        intrinsic.fingerprint,
        local
            .semantic_definition()
            .definition_fingerprint()
            .as_str()
    );
    for (locator, binding, family) in [
        (
            local_locator,
            local.binding(),
            WorkDefinitionFamilyV1::CompleteLocalV2,
        ),
        (
            full_locator,
            full.binding(),
            WorkDefinitionFamilyV1::FullLocalV3,
        ),
    ] {
        assert_eq!(locator.owner, "handbook-contracts");
        assert_eq!(locator.family, family);
        assert_eq!(locator.schema_ref, family.schema_ref());
        assert_eq!(locator.reference, binding.reference().as_str());
        assert_eq!(locator.fingerprint, binding.fingerprint().as_str());
        assert_ne!(locator.fingerprint, intrinsic.fingerprint);
    }
}

#[test]
fn closed_scope_and_role_codecs_reject_unselected_fields_and_complete_record_overflow() {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (_, check) = setup(&sdk);
    let definition = work_requirements_definition_from_intrinsic_v1(
        &work_state_support::supplied_intrinsic_definition("1.0.0"),
    );
    let original = approve_scope(&check, "closed-codec", menu_scope(definition));
    for case in 0..10 {
        let mut terms = original.clone();
        let WorkRegistrationScopeV1::Explicit(scope) = &mut terms.scope else {
            unreachable!()
        };
        match case {
            0 => scope.schema_ref = "handbook.work-registration-scope@2.0.0".into(),
            1 => scope.clauses.clear(),
            2 => scope.clauses.push(scope.clauses[0].clone()),
            3 => scope.clauses[0].label_max_bytes = 0,
            4 => scope.clauses[0].work_question_max_bytes = 4097,
            5 => scope.clauses[0].display_levels = vec!["*".into()],
            6 => scope.clauses[0].display_levels = vec!["x".repeat(129)],
            7 => {
                scope.clauses[0].role_permission = WorkRolePermissionV1::PlanningOutput {
                    question_max_bytes: 256,
                }
            }
            8 => {
                scope.clauses[2].role_permission = WorkRolePermissionV1::RequirementsInput {
                    definitions: vec![],
                }
            }
            _ => {
                scope.clauses[1].role_permission = WorkRolePermissionV1::PlanningOutput {
                    question_max_bytes: 0,
                }
            }
        }
        terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
        assert!(
            WorkRegistrationTermsV1::from_json_bytes(&serde_json::to_vec(&terms).unwrap()).is_err(),
            "case {case}"
        );
    }
    let mut value = serde_json::to_value(&original).unwrap();
    value["composition"]["clauses"][0]["future_permission"] = serde_json::json!(true);
    assert!(
        WorkRegistrationTermsV1::from_json_bytes(&serde_json::to_vec(&value).unwrap()).is_err()
    );
    let mut oversized = serde_json::to_value(&original).unwrap();
    oversized["composition"]["clauses"][0]["display_levels"] = serde_json::json!((0..9000)
        .map(|index| format!("l{index:05}{}", "a".repeat(122)))
        .collect::<Vec<_>>());
    let bytes = serde_json::to_vec(&oversized).unwrap();
    assert!(bytes.len() > 1_048_576);
    assert!(WorkRegistrationTermsV1::from_json_bytes(&bytes).is_err());
    let mut request = serde_json::json!({
        "request_key":"too-large", "item_key":"too-large", "label":"x".repeat(1_048_576),
        "question":"Question", "actor":check.terms.agent, "expected_state":"absent",
        "composition":WorkCompositionV1::planning_output("What exact requirements?")
    });
    assert!(
        WorkRegistrationInputV1::from_json_bytes(&serde_json::to_vec(&request).unwrap()).is_err()
    );
    request["label"] = serde_json::json!("Planning leaf");
    request["composition"]["role"]["output_locator"] = serde_json::json!("guessed.future@1.0.0");
    assert!(
        WorkRegistrationInputV1::from_json_bytes(&serde_json::to_vec(&request).unwrap()).is_err()
    );
    request["composition"]["role"] =
        serde_json::json!({"role":"assessment_result","result":"pass"});
    assert!(
        WorkRegistrationInputV1::from_json_bytes(&serde_json::to_vec(&request).unwrap()).is_err()
    );
}

fn setup(sdk: &HandbookSdkV1) -> (LocalInitialSetupLocatorV1, LocalCheckLocatorV1) {
    let initial = establish_initial(sdk);
    let original = sdk
        .resolve_local_initial_setup_request(&initial)
        .unwrap()
        .terms
        .unwrap()
        .outcome
        .unwrap();
    let prepared = sdk
        .prepare_local_check_terms(
            &initial,
            supplied_successor_terms(&initial, &original.outcome_fingerprint),
        )
        .unwrap();
    let check = prepared.locator().clone();
    let execution = sdk.adopt_local_check_terms(prepared);
    assert!(execution.outcome.is_some(), "{execution:?}");
    (initial, check)
}

fn supplied_terms(check: &LocalCheckLocatorV1, key: &str) -> WorkRegistrationTermsV1 {
    let mut terms = WorkRegistrationTermsV1 {
        request_key: key.into(),
        rule_ref: WORK_REGISTRATION_RULE_V1.into(),
        mode: "local_advisory".into(),
        principal: check.terms.principal.clone(),
        agent: check.terms.agent.clone(),
        instruction: "Register bounded ordinary open leaves under this exact work rule.".into(),
        delegation: "same-project ordinary open leaf registration".into(),
        approval_source_ref: "decision.supplied.work-registration".into(),
        human_approval_asserted: true,
        approved_content_fingerprint: String::new(),
        effect: "register_open_leaf".into(),
        scope: ordinary_scope(),
        owner_write_reach: WORK_STATE_FULL_OWNER_REACH_V1.into(),
        owner_read_reach: WORK_STATE_FULL_OWNER_REACH_V1.into(),
        reserved_decisions: vec![
            "composition_change".into(),
            "terms_change".into(),
            "close".into(),
        ],
        revocation_principal: check.terms.principal.clone(),
        retention: "retain_original_history".into(),
    };
    terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
    terms
}

fn activate(
    sdk: &HandbookSdkV1,
    initial: &LocalInitialSetupLocatorV1,
    check: &LocalCheckLocatorV1,
    key: &str,
    predecessor: Option<&WorkGrantLocatorV1>,
) -> WorkGrantLocatorV1 {
    let prepared = sdk
        .prepare_work_registration_terms(
            initial,
            check,
            supplied_terms(check, key),
            predecessor.map_or(
                WorkTermsTransitionV1::Initial,
                WorkTermsTransitionV1::EstablishFromDeferred,
            ),
        )
        .unwrap();
    let locator =
        WorkGrantLocatorV1::from_json_bytes(&prepared.locator().canonical_json_bytes().unwrap())
            .unwrap();
    assert_eq!(locator, *prepared.locator());
    let execution = sdk.activate_work_registration_terms(prepared);
    assert_eq!(
        execution.disposition,
        WorkDispositionV1::Committed,
        "{execution:?}"
    );
    assert_eq!(execution.outcome.unwrap().locator, locator);
    locator
}

fn input(grant: &WorkGrantLocatorV1, request: &str, item: &str) -> WorkRegistrationInputV1 {
    WorkRegistrationInputV1 {
        request_key: request.into(),
        item_key: item.into(),
        label: "Investigate session renewal".into(),
        question: "Why does this local session lose its renewal token?".into(),
        actor: grant.terms.agent.clone(),
        expected_state: "absent".into(),
        composition: WorkCompositionV1::ordinary_leaf(),
        display_level: None,
    }
}

fn register(
    sdk: &HandbookSdkV1,
    initial: &LocalInitialSetupLocatorV1,
    grant: &WorkGrantLocatorV1,
    request: &str,
    item: &str,
) -> WorkRegistrationReceiptV1 {
    let prepared = sdk
        .prepare_work_registration(initial, grant, input(grant, request, item))
        .unwrap();
    let locator =
        WorkRequestLocatorV1::from_json_bytes(&prepared.locator().canonical_json_bytes().unwrap())
            .unwrap();
    assert_eq!(
        sdk.resolve_work_registration_request(&locator).disposition,
        WorkDispositionV1::Unaccepted
    );
    let execution = sdk.register_work_item(prepared);
    assert_eq!(
        execution.disposition,
        WorkDispositionV1::Committed,
        "{execution:?}"
    );
    let receipt = execution.outcome.unwrap();
    assert_eq!(receipt.locator, locator);
    receipt
}

#[test]
fn registration_reopen_receipt_history_and_completion_have_distinct_meaning() {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let grant = activate(&sdk, &initial, &check, "work-terms-D", None);
    sdk.resolve_current_work_registration_terms(&initial, &grant)
        .unwrap();
    let receipt = register(
        &sdk,
        &initial,
        &grant,
        "register-session",
        "inspect-session-renewal",
    );
    assert_eq!(receipt.snapshot.state, "open");
    assert_eq!(
        receipt.snapshot.descriptor,
        WorkDescriptorV1::ordinary_leaf()
    );
    assert_eq!(
        receipt.snapshot.descriptor_bytes,
        WorkDescriptorV1::ordinary_leaf()
            .canonical_json_bytes()
            .unwrap()
    );
    assert_eq!(
        receipt.snapshot.item_identity,
        work_item_identity_v1(
            initial
                .deferral
                .initialization_claim
                .proposed_repository_identity(),
            "inspect-session-renewal",
        )
        .unwrap()
    );
    drop(sdk);
    let reopened = HandbookSdkV1::open(dir.path());
    assert_eq!(
        reopened
            .resolve_current_work_item(&grant, "inspect-session-renewal")
            .unwrap(),
        WorkCurrentItemV1::Open(receipt.snapshot.clone())
    );
    assert_eq!(
        reopened
            .resolve_work_registration_request(&receipt.locator)
            .outcome
            .unwrap(),
        receipt
    );
    assert_eq!(
        reopened
            .recover_work_registration_request(&receipt.locator)
            .outcome
            .unwrap(),
        receipt
    );
    let replay = reopened.register_work_item(
        reopened
            .prepare_work_registration(&initial, &grant, receipt.locator.input.clone())
            .unwrap(),
    );
    assert_eq!(replay.disposition, WorkDispositionV1::Replayed);
    assert_eq!(replay.outcome.unwrap(), receipt);
    let completion = reopened
        .inspect_work_completion_support(&receipt.locator)
        .unwrap();
    assert_eq!(completion.item, receipt.snapshot);
    assert_eq!(completion.completion_support, "unsupported");
    assert_eq!(completion.assessment, "not_assessed");
}

#[test]
fn local_check_alone_unapproved_substitute_and_insufficient_reach_supply_no_work_permission() {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let candidate = sdk
        .prepare_work_registration_terms(
            &initial,
            &check,
            supplied_terms(&check, "not-activated"),
            WorkTermsTransitionV1::Initial,
        )
        .unwrap();
    let grant = candidate.locator().clone();
    assert!(sdk
        .prepare_work_registration(&initial, &grant, input(&grant, "denied", "denied"))
        .is_err());
    for change in ["unapproved", "agent-principal", "wider-reach"] {
        let mut terms = supplied_terms(&check, change);
        match change {
            "unapproved" => terms.human_approval_asserted = false,
            "agent-principal" => terms.principal = terms.agent.clone(),
            _ => terms.owner_write_reach = ".handbook/work-state/v1/items/**".into(),
        }
        terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
        assert!(
            sdk.prepare_work_registration_terms(
                &initial,
                &check,
                terms,
                WorkTermsTransitionV1::Initial
            )
            .is_err(),
            "{change}"
        );
    }
    let active = activate(&sdk, &initial, &check, "active-D", None);
    let mut substitute = active.clone();
    substitute.terms.request_key = "substituted".into();
    assert!(sdk
        .prepare_work_registration(
            &initial,
            &substitute,
            input(&active, "substitution", "substitution")
        )
        .is_err());
    let other = tempdir().unwrap();
    assert!(HandbookSdkV1::open(other.path())
        .resolve_current_work_item(&active, "absent")
        .is_err());
}

#[test]
fn d_to_g_requires_explicit_predecessor_and_never_substitutes_old_request_authority() {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let d = activate(&sdk, &initial, &check, "work-D", None);
    let old = register(&sdk, &initial, &d, "old-committed", "old-item");
    let prepared = sdk
        .prepare_work_registration(&initial, &d, input(&d, "stale-D-request", "stale-D-item"))
        .unwrap();
    let stale = prepared.locator().clone();
    let current = sdk
        .resolve_project_admission_scope_v3(
            &ProjectAdmissionBindingV3::ScopedChoiceV2(initial.deferral.project.clone()),
            ADMISSION_GOVERNANCE_SCOPE_V1,
            None,
        )
        .unwrap();
    let ProjectAdmissionScopeBasisV3::DeferredV2(basis) = current else {
        panic!("D expected");
    };
    let request = DeferredEstablishmentRequestV3::human_approved(
        initial.deferral.project.clone(),
        ExpectedDeferredBasisV2 {
            outcome_fingerprint: basis.outcome_fingerprint,
            relation_fingerprint: basis.relation_fingerprint,
        },
        60,
        "work-G-governance",
        "Human approved exact supplied G applicability for this test.",
    )
    .unwrap();
    assert!(matches!(
        sdk.establish_deferred_project_admission_governance(
            sdk.prepare_deferred_project_admission_governance(&request)
                .unwrap()
        ),
        DeferredEstablishmentExecutionV3::Committed { .. }
    ));
    assert!(sdk
        .resolve_current_work_registration_terms(&initial, &d)
        .is_err());
    assert_eq!(
        sdk.register_work_item(prepared).disposition,
        WorkDispositionV1::Refused
    );
    assert!(sdk
        .prepare_work_registration_terms(
            &initial,
            &check,
            supplied_terms(&check, "missing-prior"),
            WorkTermsTransitionV1::Initial
        )
        .is_err());
    let g = activate(&sdk, &initial, &check, "work-G", Some(&d));
    assert_eq!(
        g.predecessor_grant_fingerprint.as_deref(),
        Some(d.fingerprint.as_str())
    );
    sdk.resolve_current_work_registration_terms(&initial, &g)
        .unwrap();
    register(&sdk, &initial, &g, "new-G-request", "new-G-item");
    assert!(sdk
        .recover_work_registration_request(&stale)
        .outcome
        .is_none());
    assert_eq!(
        sdk.resolve_current_work_item(&d, "old-item").unwrap(),
        WorkCurrentItemV1::Open(old.snapshot.clone())
    );
    assert_eq!(
        sdk.recover_work_registration_request(&old.locator)
            .outcome
            .unwrap(),
        old
    );
    assert_eq!(
        sdk.resolve_work_registration_terms_request(&d).disposition,
        WorkDispositionV1::Replayed
    );
    assert!(sdk
        .prepare_work_registration(&initial, &d, input(&d, "still-D", "still-D"))
        .is_err());
}

#[test]
fn revocation_and_actual_sdk_reset_preserve_exact_work_history_without_permission() {
    for reset in [false, true] {
        let dir = tempdir().unwrap();
        let sdk = HandbookSdkV1::open(dir.path());
        let (initial, check) = setup(&sdk);
        let grant = activate(&sdk, &initial, &check, "work-history", None);
        let receipt = register(&sdk, &initial, &grant, "retained-request", "retained-item");
        if reset {
            let result = sdk
                .run_setup(&SetupRequest {
                    mode: SetupMode::Refresh,
                    rewrite: false,
                    reset_state: true,
                })
                .unwrap();
            assert!(result.reset_applied, "real SDK reset was not applied");
        } else {
            let revoke = sdk
                .prepare_work_registration_revocation(
                    &grant,
                    "revoke-work",
                    &grant.terms.revocation_principal,
                    "decision.supplied.revoke",
                    "Revoke this exact supplied work registration grant.",
                )
                .unwrap();
            let bytes = revoke.canonical_json_bytes().unwrap();
            let original = WorkRevokeLocatorV1::from_json_bytes(&bytes).unwrap();
            assert_eq!(
                sdk.revoke_work_registration_terms(&original).disposition,
                WorkDispositionV1::Committed
            );
            assert_eq!(
                sdk.resolve_work_registration_revocation_request(&original)
                    .disposition,
                WorkDispositionV1::Replayed
            );
            assert_eq!(
                sdk.recover_work_registration_revocation_request(&original)
                    .disposition,
                WorkDispositionV1::Replayed
            );
        }
        let reopened = HandbookSdkV1::open(dir.path());
        assert_eq!(
            reopened
                .resolve_current_work_item(&grant, "retained-item")
                .unwrap(),
            WorkCurrentItemV1::Open(receipt.snapshot.clone())
        );
        assert_eq!(
            reopened
                .resolve_work_registration_request(&receipt.locator)
                .outcome
                .unwrap(),
            receipt
        );
        assert_eq!(
            reopened
                .recover_work_registration_request(&receipt.locator)
                .outcome
                .unwrap(),
            receipt
        );
        assert_eq!(
            reopened
                .recover_work_registration_terms_request(&grant)
                .disposition,
            WorkDispositionV1::Replayed
        );
        assert!(reopened
            .resolve_current_work_registration_terms(&initial, &grant)
            .is_err());
        assert!(reopened
            .prepare_work_registration(&initial, &grant, input(&grant, "new-write", "new-item"))
            .is_err());
    }
}

#[test]
fn future_roles_kinds_overrides_states_and_conflicting_keys_refuse_without_rebinding() {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let grant = activate(&sdk, &initial, &check, "work-compatibility", None);
    let receipt = register(&sdk, &initial, &grant, "stable-request", "stable-item");
    for future in [
        "required_output",
        "requirements_input",
        "parent",
        "state_machine",
        "contract_locator",
    ] {
        let mut value = serde_json::to_value(input(&grant, "future", "future")).unwrap();
        value[future] = serde_json::json!("unsupported");
        assert!(
            WorkRegistrationInputV1::from_json_bytes(&serde_json::to_vec(&value).unwrap()).is_err(),
            "{future}"
        );
    }
    for change in ["kind", "roles", "relations", "version", "completed"] {
        let mut request = input(&grant, change, change);
        match change {
            "kind" => {
                request.composition.kind = Some(LocalBindingV1 {
                    reference: "handbook.work-kind.future@1.0.0".into(),
                    fingerprint: format!("sha256:{}", "a".repeat(64)),
                })
            }
            "roles" => request
                .composition
                .descriptor
                .role_schema
                .push("requirements_input".into()),
            "relations" => request
                .composition
                .descriptor
                .relation_support
                .push("prerequisite".into()),
            "version" => request.composition.descriptor.compatibility_version = "2".into(),
            _ => request.expected_state = "completed".into(),
        }
        assert!(
            sdk.prepare_work_registration(&initial, &grant, request)
                .is_err(),
            "{change}"
        );
    }
    let mut conflicting = receipt.locator.input.clone();
    conflicting.label = "Changed label under same request".into();
    assert!(sdk
        .prepare_work_registration(&initial, &grant, conflicting)
        .is_err());
    assert!(sdk
        .prepare_work_registration(&initial, &grant, input(&grant, "new-key", "stable-item"),)
        .is_err());
    let original_bytes = receipt.locator.canonical_json_bytes().unwrap();
    let restored = WorkRequestLocatorV1::from_json_bytes(&original_bytes).unwrap();
    assert_eq!(
        sdk.recover_work_registration_request(&restored)
            .outcome
            .unwrap(),
        receipt
    );
    assert_eq!(
        sdk.resolve_current_work_item(&grant, "stable-item")
            .unwrap(),
        WorkCurrentItemV1::Open(receipt.snapshot)
    );
}

#[test]
fn export_supplied_ordinary_work_consumer_inputs() {
    let Some(directory) = std::env::var_os("HANDBOOK_WORK_CONSUMER_INPUTS") else {
        return;
    };
    let directory = std::path::PathBuf::from(directory);
    let root = directory.join("project");
    fs::create_dir_all(&root).unwrap();
    let sdk = HandbookSdkV1::open(&root);
    let (initial, check) = setup(&sdk);
    let terms = supplied_terms(&check, "normal-consumer-terms");
    fs::write(
        directory.join("initial.json"),
        initial.canonical_json_bytes().unwrap(),
    )
    .unwrap();
    fs::write(
        directory.join("check.json"),
        check.canonical_json_bytes().unwrap(),
    )
    .unwrap();
    fs::write(
        directory.join("terms.json"),
        terms.canonical_json_bytes().unwrap(),
    )
    .unwrap();
    let prepared = sdk
        .prepare_work_registration_terms(&initial, &check, terms, WorkTermsTransitionV1::Initial)
        .unwrap();
    fs::write(
        directory.join("input.json"),
        input(
            prepared.locator(),
            "normal-consumer-registration",
            "inspect-session-renewal",
        )
        .canonical_json_bytes()
        .unwrap(),
    )
    .unwrap();
    let definition = work_state_support::supplied_intrinsic_definition("1.0.0");
    fs::write(
        directory.join("requirements-definition.json"),
        definition.canonical_json_bytes().unwrap(),
    )
    .unwrap();
    let menu = approve_scope(
        &check,
        "normal-consumer-menu",
        menu_scope(work_requirements_definition_from_intrinsic_v1(&definition)),
    );
    fs::write(
        directory.join("menu-terms.json"),
        menu.canonical_json_bytes().unwrap(),
    )
    .unwrap();
    for (role, file, key) in [
        (1, "planning-input.json", "normal-consumer-planning"),
        (
            2,
            "implementation-input.json",
            "normal-consumer-implementation",
        ),
    ] {
        fs::write(
            directory.join(file),
            menu_input(prepared.locator(), role, key)
                .canonical_json_bytes()
                .unwrap(),
        )
        .unwrap();
    }
    let mut unapproved = menu_input(prepared.locator(), 1, "normal-consumer-unapproved");
    unapproved.display_level = Some("unapproved".into());
    fs::write(
        directory.join("unapproved-input.json"),
        unapproved.canonical_json_bytes().unwrap(),
    )
    .unwrap();
}
