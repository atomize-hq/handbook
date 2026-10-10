//! Public, production-API consumer of immutable composition definitions.
//! Fault/process coverage is separate; this target can run without test-support.
#[allow(dead_code)]
mod work_state_support;

use handbook_sdk::admission_governance::*;
use handbook_sdk::local_check::*;
use handbook_sdk::local_setup::LocalInitialSetupLocatorV1;
use handbook_sdk::work_state::*;
use handbook_sdk::{HandbookSdkV1, SetupMode, SetupRequest};
use std::fs;
use tempfile::tempdir;
use work_state_support::{establish_initial, menu_scope, supplied_successor_terms};

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
    assert!(sdk.adopt_local_check_terms(prepared).outcome.is_some());
    (initial, check)
}

fn requirements(version: &str) -> WorkRequirementsDefinitionV1 {
    work_requirements_definition_from_intrinsic_v1(
        &work_state_support::supplied_intrinsic_definition(version),
    )
}

fn definition(role: usize, version: &str) -> WorkKindDefinitionV1 {
    let scope = menu_scope(requirements("1.0.0"));
    let WorkRegistrationScopeV1::Explicit(scope) = scope else {
        panic!("explicit scope")
    };
    WorkKindDefinitionV1 {
        schema_ref: WORK_KIND_DEFINITION_SCHEMA_V1.into(),
        reference: format!(
            "q.work-kind.{}@{version}",
            ["ordinary", "planning", "requirements"][role]
        ),
        template: scope.clauses[role].clone(),
    }
}

fn approval(
    sdk: &HandbookSdkV1,
    initial: &LocalInitialSetupLocatorV1,
    check: &LocalCheckLocatorV1,
    key: &str,
    definition: &WorkKindDefinitionV1,
) -> WorkKindApprovalV1 {
    let mut approval = WorkKindApprovalV1 {
        request_key: key.into(),
        rule_ref: WORK_KIND_ESTABLISHMENT_RULE_V1.into(),
        mode: "local_advisory".into(),
        principal: check.terms.principal.clone(),
        agent: check.terms.agent.clone(),
        instruction: "The original human approves this complete exact immutable kind version."
            .into(),
        delegation: "establish this exact whole-composition definition".into(),
        approval_source_ref: format!("decision.supplied.{key}"),
        human_approval_asserted: true,
        check_request_fingerprint: check.request_fingerprint.clone(),
        basis: sdk.observe_local_check_basis(initial).unwrap(),
        approved_content_fingerprint: String::new(),
        effect: "establish_immutable_work_kind".into(),
        owner_write_reach: WORK_STATE_FULL_OWNER_REACH_V1.into(),
        owner_read_reach: WORK_STATE_FULL_OWNER_REACH_V1.into(),
        retention: "retain_original_history".into(),
    };
    approval.approved_content_fingerprint =
        approval.fingerprint_approved_content(definition).unwrap();
    approval
}

fn terms(
    check: &LocalCheckLocatorV1,
    key: &str,
    scope: WorkRegistrationScopeV1,
) -> WorkRegistrationTermsV1 {
    let mut terms = WorkRegistrationTermsV1 {
        request_key: key.into(),
        rule_ref: WORK_REGISTRATION_RULE_V1.into(),
        mode: "local_advisory".into(),
        principal: check.terms.principal.clone(),
        agent: check.terms.agent.clone(),
        instruction: "The original human separately approves this exact work-registration scope."
            .into(),
        delegation: "register bounded same-project open leaves".into(),
        approval_source_ref: format!("decision.supplied.{key}"),
        human_approval_asserted: true,
        approved_content_fingerprint: String::new(),
        effect: "register_open_leaf".into(),
        scope,
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
    scope: WorkRegistrationScopeV1,
    transition: WorkTermsTransitionV1<'_>,
) -> WorkGrantLocatorV1 {
    let prepared = sdk
        .prepare_work_registration_terms(initial, check, terms(check, key, scope), transition)
        .unwrap();
    let original = prepared.locator().clone();
    let execution = sdk.activate_work_registration_terms(prepared);
    assert_eq!(
        execution.disposition,
        WorkDispositionV1::Committed,
        "{execution:?}"
    );
    assert_eq!(execution.outcome.unwrap().locator, original);
    original
}

fn establish(
    sdk: &HandbookSdkV1,
    initial: &LocalInitialSetupLocatorV1,
    check: &LocalCheckLocatorV1,
    key: &str,
    definition: WorkKindDefinitionV1,
) -> WorkKindReceiptV1 {
    let prepared = sdk
        .prepare_work_kind_establishment(
            initial,
            check,
            definition.clone(),
            approval(sdk, initial, check, key, &definition),
        )
        .unwrap();
    let original =
        WorkKindLocatorV1::from_json_bytes(&prepared.locator().canonical_json_bytes().unwrap())
            .unwrap();
    let before = sdk.resolve_work_kind_request(&original);
    assert!(before.outcome.is_none(), "{before:?}");
    assert!(
        matches!(
            before.disposition,
            WorkDispositionV1::Unaccepted | WorkDispositionV1::Uncertain
        ),
        "{before:?}"
    );
    let execution = sdk.establish_work_kind(prepared);
    assert_eq!(
        execution.disposition,
        WorkDispositionV1::Committed,
        "{execution:?}"
    );
    let receipt = execution.outcome.unwrap();
    assert_eq!(receipt.locator, original);
    assert_eq!(receipt.definition, definition);
    receipt
}

fn role_value(role: usize) -> Option<WorkRoleValueV1> {
    match role {
        0 => None,
        1 => Some(WorkRoleValueV1::PlanningOutput {
            question: "What exact renewal requirements should be defined?".into(),
        }),
        2 => Some(WorkRoleValueV1::RequirementsInput {
            definition: requirements("1.0.0"),
        }),
        _ => unreachable!(),
    }
}

fn explicit(role: usize) -> WorkCompositionV1 {
    match role {
        0 => WorkCompositionV1::ordinary_leaf(),
        1 => {
            WorkCompositionV1::planning_output("What exact renewal requirements should be defined?")
        }
        2 => WorkCompositionV1::requirements_input(requirements("1.0.0")),
        _ => unreachable!(),
    }
}

fn input(
    grant: &WorkGrantLocatorV1,
    key: &str,
    role: usize,
    kind: Option<&WorkKindBindingV1>,
) -> WorkRegistrationInputV1 {
    WorkRegistrationInputV1 {
        request_key: key.into(),
        item_key: key.into(),
        label: "Plan session renewal".into(),
        question: "What session-renewal behavior should be specified?".into(),
        actor: grant.terms.agent.clone(),
        expected_state: "absent".into(),
        display_level: Some(["task", "planning", "implementation"][role].into()),
        composition: match kind {
            Some(kind) => WorkCompositionInputV1::kind(kind.clone(), role_value(role)),
            None => explicit(role).into(),
        },
    }
}

fn register(
    sdk: &HandbookSdkV1,
    initial: &LocalInitialSetupLocatorV1,
    grant: &WorkGrantLocatorV1,
    value: WorkRegistrationInputV1,
) -> WorkRegistrationReceiptV1 {
    let prepared = sdk
        .prepare_work_registration(initial, grant, value)
        .unwrap();
    let original =
        WorkRequestLocatorV1::from_json_bytes(&prepared.locator().canonical_json_bytes().unwrap())
            .unwrap();
    let execution = sdk.register_work_item(prepared);
    assert_eq!(
        execution.disposition,
        WorkDispositionV1::Committed,
        "{execution:?}"
    );
    let receipt = execution.outcome.unwrap();
    assert_eq!(receipt.locator, original);
    receipt
}

fn governance(sdk: &HandbookSdkV1, initial: &LocalInitialSetupLocatorV1) {
    let ProjectAdmissionScopeBasisV3::DeferredV2(basis) = sdk
        .resolve_project_admission_scope_v3(
            &ProjectAdmissionBindingV3::ScopedChoiceV2(initial.deferral.project.clone()),
            ADMISSION_GOVERNANCE_SCOPE_V1,
            None,
        )
        .unwrap()
    else {
        panic!("current D")
    };
    let request = DeferredEstablishmentRequestV3::human_approved(
        initial.deferral.project.clone(),
        ExpectedDeferredBasisV2 {
            outcome_fingerprint: basis.outcome_fingerprint,
            relation_fingerprint: basis.relation_fingerprint,
        },
        60,
        "kind-governance",
        "The supplied original human approves current G.",
    )
    .unwrap();
    assert!(matches!(
        sdk.establish_deferred_project_admission_governance(
            sdk.prepare_deferred_project_admission_governance(&request)
                .unwrap()
        ),
        DeferredEstablishmentExecutionV3::Committed { .. }
    ));
}

#[test]
fn exact_kind_establishment_is_independent_on_d_and_g_in_both_bootstrap_orders() {
    for on_g in [false, true] {
        for kind_first in [false, true] {
            let directory = tempdir().unwrap();
            let sdk = HandbookSdkV1::open(directory.path());
            let (initial, check) = setup(&sdk);
            if on_g {
                governance(&sdk, &initial);
            }
            let scope = menu_scope(requirements("1.0.0"));
            let (kind, grant) = if kind_first {
                let kind = establish(&sdk, &initial, &check, "kind-first", definition(1, "1.0.0"));
                assert_eq!(
                    sdk.resolve_work_kind(&kind.binding).unwrap(),
                    WorkCurrentKindV1::Committed(kind.clone())
                );
                let prospective = sdk
                    .prepare_work_registration_terms(
                        &initial,
                        &check,
                        terms(&check, "independent-grant", scope.clone()),
                        WorkTermsTransitionV1::Initial,
                    )
                    .unwrap();
                assert!(sdk
                    .prepare_work_registration(
                        &initial,
                        prospective.locator(),
                        input(
                            prospective.locator(),
                            "kind-does-not-permit-work",
                            1,
                            Some(&kind.binding)
                        )
                    )
                    .is_err());
                let grant = activate(
                    &sdk,
                    &initial,
                    &check,
                    "independent-grant",
                    scope,
                    WorkTermsTransitionV1::Initial,
                );
                (kind, grant)
            } else {
                let grant = activate(
                    &sdk,
                    &initial,
                    &check,
                    "grant-first",
                    scope,
                    WorkTermsTransitionV1::Initial,
                );
                let kind = establish(
                    &sdk,
                    &initial,
                    &check,
                    "independent-kind",
                    definition(1, "1.0.0"),
                );
                (kind, grant)
            };
            let receipt = register(
                &sdk,
                &initial,
                &grant,
                input(&grant, "planned-item", 1, Some(&kind.binding)),
            );
            drop(sdk);
            let reopened = HandbookSdkV1::open(directory.path());
            assert_eq!(
                reopened.resolve_work_kind(&kind.binding).unwrap(),
                WorkCurrentKindV1::Committed(kind.clone())
            );
            assert_eq!(
                reopened
                    .resolve_work_kind_request(&kind.locator)
                    .outcome
                    .unwrap(),
                kind
            );
            assert_eq!(
                reopened
                    .recover_work_kind_request(&kind.locator)
                    .outcome
                    .unwrap(),
                kind
            );
            assert_eq!(
                reopened
                    .recover_work_registration_request(&receipt.locator)
                    .outcome
                    .unwrap(),
                receipt
            );
            assert_eq!(receipt.snapshot.admitted_kind, Some(kind.binding));
            assert_eq!(receipt.snapshot.state, "open");
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
fn original_human_exact_content_and_immutable_version_are_not_reusable_permission() {
    let directory = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(directory.path());
    let (initial, check) = setup(&sdk);
    let content = definition(0, "1.0.0");
    let other_directory = tempdir().unwrap();
    let other_sdk = HandbookSdkV1::open(other_directory.path());
    let (other_initial, other_check) = setup(&other_sdk);
    let wrong_project = approval(
        &other_sdk,
        &other_initial,
        &other_check,
        "wrong-project",
        &content,
    );
    assert!(sdk
        .prepare_work_kind_establishment(&initial, &check, content.clone(), wrong_project)
        .is_err());
    assert!(sdk
        .prepare_work_kind_establishment(
            &other_initial,
            &check,
            content.clone(),
            approval(&sdk, &initial, &check, "wrong-initial", &content)
        )
        .is_err());
    for case in 0..8 {
        let mut value = approval(&sdk, &initial, &check, &format!("wrong-{case}"), &content);
        match case {
            0 => value.principal = "other.human".into(),
            1 => value.agent = "other.agent".into(),
            2 => value.human_approval_asserted = false,
            3 => value.rule_ref = WORK_REGISTRATION_RULE_V1.into(),
            4 => value.effect = "register_open_leaf".into(),
            5 => value.owner_write_reach = ".handbook/work-state/**".into(),
            6 => value.retention = "discard_original_history".into(),
            7 => value.instruction.push_str(" changed"),
            _ => unreachable!(),
        }
        if case != 7 {
            value.approved_content_fingerprint =
                value.fingerprint_approved_content(&content).unwrap();
        }
        assert!(
            sdk.prepare_work_kind_establishment(&initial, &check, content.clone(), value)
                .is_err(),
            "case {case}"
        );
    }
    let original = establish(&sdk, &initial, &check, "approved-k1", content.clone());
    assert!(other_sdk.resolve_work_kind(&original.binding).is_err());
    assert_eq!(
        sdk.establish_work_kind(
            sdk.prepare_work_kind_establishment(
                &initial,
                &check,
                content.clone(),
                approval(&sdk, &initial, &check, "approved-k1", &content)
            )
            .unwrap()
        )
        .outcome
        .unwrap(),
        original
    );
    for changed in [false, true] {
        let mut attempted = content.clone();
        if changed {
            attempted.template.label_max_bytes += 1;
        }
        assert!(sdk
            .prepare_work_kind_establishment(
                &initial,
                &check,
                attempted.clone(),
                approval(&sdk, &initial, &check, "another-k1-request", &attempted)
            )
            .is_err());
    }
    let mut changed = content.clone();
    changed.template.label_max_bytes += 1;
    assert!(sdk
        .prepare_work_kind_establishment(
            &initial,
            &check,
            changed,
            approval(
                &sdk,
                &initial,
                &check,
                "same-approval-changed-content",
                &content
            )
        )
        .is_err());
    let mut binding = original.binding.clone();
    binding.fingerprint = format!("sha256:{}", "a".repeat(64));
    assert!(sdk.resolve_work_kind(&binding).is_err());
    binding = original.binding.clone();
    binding.project_identity.push_str("another");
    assert!(sdk.resolve_work_kind(&binding).is_err());
    let k2 = establish(
        &sdk,
        &initial,
        &check,
        "approved-k2",
        definition(0, "2.0.0"),
    );
    assert_ne!(k2.binding, original.binding);
    assert_eq!(
        sdk.resolve_work_kind(&original.binding).unwrap(),
        WorkCurrentKindV1::Committed(original)
    );
}

#[test]
fn ordinary_planning_requirements_meaning_matches_explicit_but_retains_exact_kind_provenance() {
    let directory = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(directory.path());
    let (initial, check) = setup(&sdk);
    let grant = activate(
        &sdk,
        &initial,
        &check,
        "composition-grant",
        menu_scope(requirements("1.0.0")),
        WorkTermsTransitionV1::Initial,
    );
    for role in 0..3 {
        let kind = establish(
            &sdk,
            &initial,
            &check,
            &format!("kind-{role}"),
            definition(role, "1.0.0"),
        );
        let explicit = register(
            &sdk,
            &initial,
            &grant,
            input(&grant, &format!("explicit-{role}"), role, None),
        );
        let expanded = register(
            &sdk,
            &initial,
            &grant,
            input(
                &grant,
                &format!("expanded-{role}"),
                role,
                Some(&kind.binding),
            ),
        );
        assert_eq!(explicit.snapshot.descriptor, expanded.snapshot.descriptor);
        assert_eq!(explicit.snapshot.role, expanded.snapshot.role);
        assert_eq!(explicit.snapshot.label, expanded.snapshot.label);
        assert_eq!(explicit.snapshot.question, expanded.snapshot.question);
        assert_eq!(
            explicit.snapshot.display_level,
            expanded.snapshot.display_level
        );
        assert_eq!(explicit.snapshot.admitted_kind, None);
        assert_eq!(expanded.snapshot.admitted_kind, Some(kind.binding.clone()));
        assert_ne!(explicit.locator.fingerprint, expanded.locator.fingerprint);
        assert_ne!(explicit.receipt_fingerprint, expanded.receipt_fingerprint);
        let k2 = establish(
            &sdk,
            &initial,
            &check,
            &format!("kind-{role}-v2"),
            definition(role, "2.0.0"),
        );
        assert_ne!(k2.binding, kind.binding);
        assert_eq!(
            sdk.recover_work_registration_request(&expanded.locator)
                .outcome
                .unwrap(),
            expanded
        );
        for case in 0..6 {
            let mut invalid = input(
                &grant,
                &format!("invalid-{role}-{case}"),
                role,
                Some(&kind.binding),
            );
            match case {
                0 => invalid.label = "é".repeat(65),
                1 => invalid.question = "x".repeat(257),
                2 => invalid.display_level = Some("unapproved".into()),
                3 if role != 0 => invalid.display_level = None,
                3 => {
                    invalid.composition =
                        WorkCompositionInputV1::kind(kind.binding.clone(), role_value(1))
                }
                4 if role == 1 => {
                    invalid.composition = WorkCompositionInputV1::kind(
                        kind.binding.clone(),
                        Some(WorkRoleValueV1::PlanningOutput {
                            question: "x".repeat(257),
                        }),
                    )
                }
                4 if role == 2 => {
                    invalid.composition = WorkCompositionInputV1::kind(
                        kind.binding.clone(),
                        Some(WorkRoleValueV1::RequirementsInput {
                            definition: requirements("2.0.0"),
                        }),
                    )
                }
                4 => {
                    invalid.composition =
                        WorkCompositionInputV1::kind(kind.binding.clone(), role_value(2))
                }
                5 if role == 2 => {
                    let mut locator = requirements("1.0.0");
                    locator.fingerprint = format!("sha256:{}", "a".repeat(64));
                    invalid.composition = WorkCompositionInputV1::kind(
                        kind.binding.clone(),
                        Some(WorkRoleValueV1::RequirementsInput {
                            definition: locator,
                        }),
                    );
                }
                5 if role == 1 => {
                    invalid.composition = WorkCompositionInputV1::kind(kind.binding.clone(), None)
                }
                _ => invalid.expected_state = "completed".into(),
            }
            assert!(
                sdk.prepare_work_registration(&initial, &grant, invalid)
                    .is_err(),
                "role {role}, case {case}"
            );
        }
        let mut boundary = input(
            &grant,
            &format!("byte-boundary-{role}"),
            role,
            Some(&kind.binding),
        );
        boundary.label = "é".repeat(64);
        boundary.question = "x".repeat(256);
        register(&sdk, &initial, &grant, boundary);
    }
}

#[test]
fn kind_and_current_scope_intersect_and_revocation_does_not_change_definition_truth() {
    let directory = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(directory.path());
    let (initial, check) = setup(&sdk);
    let mut broad = definition(1, "1.0.0");
    broad.template.label_max_bytes = 256;
    let kind = establish(&sdk, &initial, &check, "broad-kind", broad);
    let grant = activate(
        &sdk,
        &initial,
        &check,
        "narrower-grant",
        menu_scope(requirements("1.0.0")),
        WorkTermsTransitionV1::Initial,
    );
    let mut request = input(&grant, "grant-limit", 1, Some(&kind.binding));
    request.label = "x".repeat(129);
    assert!(sdk
        .prepare_work_registration(&initial, &grant, request)
        .is_err());
    let mut narrow = definition(1, "2.0.0");
    narrow.template.label_max_bytes = 16;
    let k2 = establish(&sdk, &initial, &check, "narrow-kind", narrow);
    assert!(sdk
        .prepare_work_registration(
            &initial,
            &grant,
            input(&grant, "kind-limit", 1, Some(&k2.binding))
        )
        .is_err());
    let original = register(
        &sdk,
        &initial,
        &grant,
        input(&grant, "retained-k1-item", 1, Some(&kind.binding)),
    );
    let revocation = sdk
        .prepare_work_registration_revocation(
            &grant,
            "independent-revocation",
            &grant.terms.principal,
            "decision.work-revoke",
            "The original human revokes the work grant.",
        )
        .unwrap();
    assert_eq!(
        sdk.revoke_work_registration_terms(&revocation).disposition,
        WorkDispositionV1::Committed
    );
    assert!(sdk
        .prepare_work_registration(
            &initial,
            &grant,
            input(&grant, "revoked", 1, Some(&kind.binding))
        )
        .is_err());
    establish(
        &sdk,
        &initial,
        &check,
        "kind-after-revocation",
        definition(1, "3.0.0"),
    );
    assert_eq!(
        sdk.resolve_work_kind(&kind.binding).unwrap(),
        WorkCurrentKindV1::Committed(kind)
    );
    assert_eq!(
        sdk.recover_work_registration_request(&original.locator)
            .outcome
            .unwrap(),
        original
    );
}

#[test]
fn current_g_grant_reuses_d_kind_and_runtime_reset_retains_both_originals() {
    let directory = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(directory.path());
    let (initial, check) = setup(&sdk);
    let kind = establish(&sdk, &initial, &check, "kind-on-d", definition(1, "1.0.0"));
    let scope = menu_scope(requirements("1.0.0"));
    let d = activate(
        &sdk,
        &initial,
        &check,
        "grant-on-d",
        scope.clone(),
        WorkTermsTransitionV1::Initial,
    );
    let stale_definition = definition(1, "2.0.0");
    let stale_kind = sdk
        .prepare_work_kind_establishment(
            &initial,
            &check,
            stale_definition.clone(),
            approval(&sdk, &initial, &check, "stale-kind", &stale_definition),
        )
        .unwrap();
    let stale_kind_locator = stale_kind.locator().clone();
    let stale_item = sdk
        .prepare_work_registration(
            &initial,
            &d,
            input(&d, "stale-item", 1, Some(&kind.binding)),
        )
        .unwrap();
    governance(&sdk, &initial);
    assert!(sdk
        .prepare_work_kind_establishment(
            &initial,
            &check,
            stale_definition,
            stale_kind_locator.approval.clone()
        )
        .is_err());
    assert_eq!(
        sdk.establish_work_kind(stale_kind).disposition,
        WorkDispositionV1::Refused
    );
    assert_eq!(
        sdk.register_work_item(stale_item).disposition,
        WorkDispositionV1::Refused
    );
    assert!(sdk
        .recover_work_kind_request(&stale_kind_locator)
        .outcome
        .is_none());
    let g = activate(
        &sdk,
        &initial,
        &check,
        "grant-on-g",
        scope,
        WorkTermsTransitionV1::EstablishFromDeferred(&d),
    );
    let item = register(
        &sdk,
        &initial,
        &g,
        input(&g, "g-using-d-kind", 1, Some(&kind.binding)),
    );
    let reset = sdk
        .run_setup(&SetupRequest {
            mode: SetupMode::Refresh,
            rewrite: false,
            reset_state: true,
        })
        .unwrap();
    assert!(reset.reset_applied);
    drop(sdk);
    let reopened = HandbookSdkV1::open(directory.path());
    assert_eq!(
        reopened.resolve_work_kind(&kind.binding).unwrap(),
        WorkCurrentKindV1::Committed(kind.clone())
    );
    assert_eq!(
        reopened
            .recover_work_kind_request(&kind.locator)
            .outcome
            .unwrap(),
        kind
    );
    assert_eq!(
        reopened
            .recover_work_registration_request(&item.locator)
            .outcome
            .unwrap(),
        item
    );
    assert!(reopened
        .prepare_work_registration(
            &initial,
            &g,
            input(&g, "after-reset", 1, item.snapshot.admitted_kind.as_ref())
        )
        .is_err());
    let fresh = definition(1, "3.0.0");
    let mut original_approval = kind.locator.approval.clone();
    original_approval.request_key = "after-reset-kind".into();
    original_approval.approved_content_fingerprint = original_approval
        .fingerprint_approved_content(&fresh)
        .unwrap();
    assert!(reopened
        .prepare_work_kind_establishment(&initial, &check, fresh, original_approval)
        .is_err());
}

#[test]
fn closed_kind_codecs_refuse_unknown_incomplete_and_overridden_compositions() {
    let original = definition(2, "1.0.0");
    let bytes = original.canonical_json_bytes().unwrap();
    assert_eq!(
        WorkKindDefinitionV1::from_json_bytes(&bytes).unwrap(),
        original
    );
    let json = String::from_utf8(bytes).unwrap();
    assert!(WorkKindDefinitionV1::from_json_bytes(format!("{json} {{}}").as_bytes()).is_err());
    assert!(WorkKindDefinitionV1::from_json_bytes(
        json.replacen("{", "{\"reference\":\"q.work-kind.duplicate@1.0.0\",", 1)
            .as_bytes()
    )
    .is_err());
    for case in 0..13 {
        let mut value = serde_json::to_value(&original).unwrap();
        match case {
            0 => value["schema_ref"] = serde_json::json!("handbook.work-kind-definition@2.0.0"),
            1 => value["reference"] = serde_json::json!("q.work-kind.requirements@latest"),
            2 => value["reference"] = serde_json::json!("q.work-kind.requirements@01.0.0"),
            3 => value["reference"] = serde_json::json!("Q.work-kind.requirements@1.0.0"),
            4 => value["reference"] = serde_json::json!("q..requirements@1.0.0"),
            5 => value["template"]["label_max_bytes"] = serde_json::json!(0),
            6 => value["template"]["work_question_max_bytes"] = serde_json::json!(4097),
            7 => value["template"]["descriptor"]["compatibility_version"] = serde_json::json!("2"),
            8 => {
                value["template"]["descriptor"]["relation_support"] = serde_json::json!(["parent"])
            }
            9 => {
                value["template"]["role_permission"]["definitions"][0]["family"] =
                    serde_json::json!("unknown_family")
            }
            10 => {
                value["template"]["role_permission"]["definitions"][0]["schema_ref"] =
                    serde_json::json!("handbook.contract-definition.local@2.0.0")
            }
            11 => value["template"]["hook"] = serde_json::json!("assessment"),
            _ => {
                value["template"]["display_levels"] =
                    serde_json::json!(["implementation", "implementation"])
            }
        }
        assert!(
            WorkKindDefinitionV1::from_json_bytes(&serde_json::to_vec(&value).unwrap()).is_err(),
            "case {case}"
        );
    }
}

#[test]
fn export_default_feature_kind_consumer_inputs() {
    let Some(directory) = std::env::var_os("HANDBOOK_WORK_KIND_CONSUMER_INPUTS") else {
        return;
    };
    let directory = std::path::PathBuf::from(directory);
    let root = directory.join("project");
    fs::create_dir_all(&root).unwrap();
    let sdk = HandbookSdkV1::open(&root);
    let (initial, check) = setup(&sdk);
    let owner_definition = work_state_support::supplied_intrinsic_definition("1.0.0");
    let content = definition(2, "1.0.0");
    let approved = approval(&sdk, &initial, &check, "normal-consumer-kind", &content);
    let prepared = sdk
        .prepare_work_kind_establishment(&initial, &check, content.clone(), approved.clone())
        .unwrap();
    let grant_terms = terms(
        &check,
        "normal-consumer-grant",
        menu_scope(requirements("1.0.0")),
    );
    let prospective = sdk
        .prepare_work_registration_terms(
            &initial,
            &check,
            grant_terms.clone(),
            WorkTermsTransitionV1::Initial,
        )
        .unwrap();
    let mut refused = input(
        prospective.locator(),
        "normal-refused-item",
        2,
        Some(&prepared.locator().binding),
    );
    refused.label = "x".repeat(129);
    for (name, bytes) in [
        ("initial.json", initial.canonical_json_bytes().unwrap()),
        ("check.json", check.canonical_json_bytes().unwrap()),
        ("definition.json", content.canonical_json_bytes().unwrap()),
        ("approval.json", approved.canonical_json_bytes().unwrap()),
        ("terms.json", grant_terms.canonical_json_bytes().unwrap()),
        (
            "binding.json",
            prepared.locator().binding.canonical_json_bytes().unwrap(),
        ),
        (
            "requirements-definition.json",
            owner_definition.canonical_json_bytes().unwrap(),
        ),
        (
            "kind-input.json",
            input(
                prospective.locator(),
                "normal-kind-item",
                2,
                Some(&prepared.locator().binding),
            )
            .canonical_json_bytes()
            .unwrap(),
        ),
        (
            "explicit-input.json",
            input(prospective.locator(), "normal-explicit-item", 2, None)
                .canonical_json_bytes()
                .unwrap(),
        ),
        (
            "refused-input.json",
            refused.canonical_json_bytes().unwrap(),
        ),
    ] {
        fs::write(directory.join(name), bytes).unwrap();
    }
}
