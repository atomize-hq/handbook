//! Supplied human claims sent through public native owners; no fabricated authority.
#[allow(dead_code)]
#[path = "../work_state_support/mod.rs"]
mod original_support;
#[allow(unused_imports)]
pub use original_support::{establish_initial, supplied_successor_terms};

use handbook_engine::local_authority::LocalRevokeRequestV1;
use handbook_engine::work_state::{WorkWriteBoundaryV1 as B, WORK_STATE_FULL_OWNER_REACH_V1};
use handbook_sdk::admission_governance::*;
use handbook_sdk::local_check::*;
use handbook_sdk::local_setup::{LocalAuthorityRefusalV1, LocalInitialSetupLocatorV1};
use handbook_sdk::work_state::*;
use handbook_sdk::{HandbookSdkV1, SetupMode, SetupRequest};
use std::{
    fs,
    path::Path,
    time::{Duration, Instant},
};

pub fn setup(sdk: &HandbookSdkV1) -> (LocalInitialSetupLocatorV1, LocalCheckLocatorV1) {
    let initial = original_support::establish_initial(sdk);
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
            original_support::supplied_successor_terms(&initial, &original.outcome_fingerprint),
        )
        .unwrap();
    let check = prepared.locator().clone();
    assert!(sdk.adopt_local_check_terms(prepared).outcome.is_some());
    (initial, check)
}

pub fn definition(reference: &str) -> WorkKindDefinitionV1 {
    WorkKindDefinitionV1 {
        schema_ref: WORK_KIND_DEFINITION_SCHEMA_V1.into(),
        reference: reference.into(),
        template: WorkCompositionClauseV1 {
            descriptor: WorkDescriptorV1::ordinary_leaf(),
            label_max_bytes: 128,
            work_question_max_bytes: 256,
            allow_omitted_display_level: true,
            display_levels: vec![],
            role_permission: WorkRolePermissionV1::Ordinary {},
        },
    }
}

pub fn approval(
    sdk: &HandbookSdkV1,
    initial: &LocalInitialSetupLocatorV1,
    check: &LocalCheckLocatorV1,
    content: &WorkKindDefinitionV1,
    key: &str,
) -> WorkKindApprovalV1 {
    let mut approval = WorkKindApprovalV1 {
        request_key: key.into(),
        rule_ref: WORK_KIND_ESTABLISHMENT_RULE_V1.into(),
        mode: "local_advisory".into(),
        principal: check.terms.principal.clone(),
        agent: check.terms.agent.clone(),
        instruction: "Establish this exact supplied immutable kind version and content".into(),
        delegation: "bounded same-project exact-version definition establishment".into(),
        approval_source_ref: "human.exact-kind-version".into(),
        human_approval_asserted: true,
        approved_content_fingerprint: String::new(),
        effect: "establish_immutable_work_kind".into(),
        owner_write_reach: WORK_STATE_FULL_OWNER_REACH_V1.into(),
        owner_read_reach: WORK_STATE_FULL_OWNER_REACH_V1.into(),
        retention: "retain_original_history".into(),
        check_request_fingerprint: check.request_fingerprint.clone(),
        // After supported reset this deliberately remains a stale original
        // test claim: preparation must refuse rather than borrowing a basis.
        basis: sdk
            .observe_local_check_basis(initial)
            .unwrap_or_else(|_| check.adoption_basis.clone()),
    };
    approval.approved_content_fingerprint = approval.fingerprint_approved_content(content).unwrap();
    approval
}

pub fn establish_kind(
    sdk: &HandbookSdkV1,
    initial: &LocalInitialSetupLocatorV1,
    check: &LocalCheckLocatorV1,
    key: &str,
    reference: &str,
) -> WorkKindBindingV1 {
    let content = definition(reference);
    let prepared = sdk
        .prepare_work_kind_establishment(
            initial,
            check,
            content.clone(),
            approval(sdk, initial, check, &content, key),
        )
        .unwrap();
    let original = prepared.locator().clone();
    let execution = sdk.establish_work_kind(prepared);
    assert_eq!(
        execution.disposition,
        WorkDispositionV1::Committed,
        "{execution:?}"
    );
    let receipt = execution.outcome.unwrap();
    assert_eq!(receipt.locator, original);
    receipt.binding
}

pub fn terms(check: &LocalCheckLocatorV1, key: &str) -> WorkRegistrationTermsV1 {
    let mut terms = WorkRegistrationTermsV1 {
        request_key: key.into(),
        rule_ref: WORK_REGISTRATION_RULE_V1.into(),
        mode: "local_advisory".into(),
        principal: check.terms.principal.clone(),
        agent: check.terms.agent.clone(),
        instruction: "Register explicit or kind-expanded ordinary leaves".into(),
        delegation: "bounded same-project registration".into(),
        approval_source_ref: "human.work".into(),
        human_approval_asserted: true,
        approved_content_fingerprint: String::new(),
        effect: "register_open_leaf".into(),
        scope: original_support::ordinary_scope(),
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

pub fn activate(
    sdk: &HandbookSdkV1,
    initial: &LocalInitialSetupLocatorV1,
    check: &LocalCheckLocatorV1,
    key: &str,
    prior: Option<&WorkGrantLocatorV1>,
) -> WorkGrantLocatorV1 {
    let prepared = sdk
        .prepare_work_registration_terms(
            initial,
            check,
            terms(check, key),
            prior.map_or(
                WorkTermsTransitionV1::Initial,
                WorkTermsTransitionV1::EstablishFromDeferred,
            ),
        )
        .unwrap();
    let original = prepared.locator().clone();
    let execution = sdk.activate_work_registration_terms(prepared);
    assert_eq!(
        execution.disposition,
        WorkDispositionV1::Committed,
        "{execution:?}"
    );
    original
}

pub fn input(
    grant: &WorkGrantLocatorV1,
    binding: &WorkKindBindingV1,
    key: &str,
) -> WorkRegistrationInputV1 {
    WorkRegistrationInputV1 {
        request_key: key.into(),
        item_key: key.into(),
        label: "Leaf".into(),
        question: "Exact bounded work question".into(),
        actor: grant.terms.agent.clone(),
        expected_state: "absent".into(),
        composition: WorkCompositionInputV1::kind(binding.clone(), None),
        display_level: None,
    }
}

pub fn govern(sdk: &HandbookSdkV1, initial: &LocalInitialSetupLocatorV1) {
    let project = ProjectAdmissionBindingV3::ScopedChoiceV2(initial.deferral.project.clone());
    let ProjectAdmissionScopeBasisV3::DeferredV2(deferred) = sdk
        .resolve_project_admission_scope_v3(&project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
        .unwrap()
    else {
        panic!("current D required")
    };
    let request = DeferredEstablishmentRequestV3::human_approved(
        initial.deferral.project.clone(),
        ExpectedDeferredBasisV2 {
            outcome_fingerprint: deferred.outcome_fingerprint,
            relation_fingerprint: deferred.relation_fingerprint,
        },
        60,
        "kind-G",
        "Human supplied current G test approval",
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

pub fn revoke(sdk: &HandbookSdkV1, grant: &WorkGrantLocatorV1) {
    let request = sdk
        .prepare_work_registration_revocation(
            grant,
            "revoke-work",
            &grant.terms.revocation_principal,
            "human.revoke",
            "Revoke exact work terms",
        )
        .unwrap();
    let execution = sdk.revoke_work_registration_terms(&request);
    assert_eq!(
        execution.disposition,
        WorkDispositionV1::Committed,
        "{execution:?}"
    );
}

pub fn revoke_check(
    sdk: &HandbookSdkV1,
    initial: &LocalInitialSetupLocatorV1,
    check: &LocalCheckLocatorV1,
) {
    let current = sdk
        .resolve_current_local_check_terms(initial, check)
        .unwrap();
    sdk.revoke_local_check_terms(
        check,
        &LocalRevokeRequestV1 {
            request_key: "revoke-check".into(),
            expected_current_fingerprint: current.outcome_fingerprint,
            principal: check.terms.revocation_principal.clone(),
            source_ref: "human.revoke".into(),
            instruction: "Revoke current local check".into(),
        },
    )
    .unwrap();
}

pub fn reset(sdk: &HandbookSdkV1) {
    assert!(
        sdk.run_setup(&SetupRequest {
            mode: SetupMode::Refresh,
            rewrite: false,
            reset_state: true
        })
        .unwrap()
        .reset_applied
    );
}

pub fn interruption() -> LocalAuthorityRefusalV1 {
    LocalAuthorityRefusalV1 {
        code: "test_interruption".into(),
        detail: "Lost native reply after actual durable write".into(),
        unresolved_request: None,
    }
}

pub fn fault(at: B) -> impl FnMut(B) -> Result<(), LocalAuthorityRefusalV1> {
    move |boundary| {
        if boundary == at {
            Err(interruption())
        } else {
            Ok(())
        }
    }
}

pub const PREFIXES: [B; 7] = [
    B::BeforeIntent,
    B::Intent,
    B::Decision,
    B::Snapshot,
    B::PublicationAttempt,
    B::Terminal,
    B::Reply,
];

pub fn wait_file(path: &Path) {
    let start = Instant::now();
    while !path.exists() {
        assert!(
            start.elapsed() < Duration::from_secs(45),
            "child did not reach native checkpoint: {}",
            path.display()
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

pub fn save_setup(repo: &Path, initial: &LocalInitialSetupLocatorV1, check: &LocalCheckLocatorV1) {
    fs::write(
        repo.join("initial.kind-test.json"),
        initial.canonical_json_bytes().unwrap(),
    )
    .unwrap();
    fs::write(
        repo.join("check.kind-test.json"),
        check.canonical_json_bytes().unwrap(),
    )
    .unwrap();
}
