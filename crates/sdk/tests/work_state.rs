//! Real owner workflow using public production dependency APIs only. This target
//! is also compiled and run from an isolated Cargo package without test-support.
mod work_state_support;

use handbook_sdk::admission_governance::*;
use handbook_sdk::local_check::*;
use handbook_sdk::local_setup::{LocalBindingV1, LocalInitialSetupLocatorV1};
use handbook_sdk::work_state::*;
use handbook_sdk::{HandbookSdkV1, SetupMode, SetupRequest};
use std::fs;
use tempfile::tempdir;
use work_state_support::{establish_initial, supplied_successor_terms};

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
        composition: WorkCompositionV1::ordinary_leaf(),
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
        .prepare_work_registration_terms(initial, check, supplied_terms(check, key), predecessor)
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
            None,
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
            sdk.prepare_work_registration_terms(&initial, &check, terms, None)
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
            None
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
        .prepare_work_registration_terms(&initial, &check, terms, None)
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
}
