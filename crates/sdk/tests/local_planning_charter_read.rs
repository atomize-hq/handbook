mod local_contract_support;

use handbook_engine::local_authority::local_check::{
    LocalCheckLocatorV1, FIELD_NOTES_PLANNING_INSTRUCTION_V1,
};
use handbook_engine::local_authority::{LocalAuthorityDispositionV1, LocalRevokeRequestV1};
use handbook_sdk::{local_setup::LocalInitialSetupLocatorV1, HandbookSdkV1};
use local_contract_support::{establish_initial, supplied_successor_terms};
use std::fs;
use tempfile::{tempdir, TempDir};

fn setup(changed_successor: bool) -> (TempDir, LocalInitialSetupLocatorV1, LocalCheckLocatorV1) {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let initial = establish_initial(&sdk);
    let original = sdk
        .resolve_local_initial_setup_request(&initial)
        .unwrap()
        .terms
        .unwrap()
        .outcome
        .unwrap();
    let mut terms = supplied_successor_terms(&initial, &original.outcome_fingerprint);
    terms.instruction = FIELD_NOTES_PLANNING_INSTRUCTION_V1.into();
    terms.allowed_input_paths = vec![".handbook/project/charter.yaml".into(), "index.html".into()];
    if changed_successor {
        terms.delegation = "different approved successor delegation".into();
    }
    terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
    let prepared = sdk.prepare_local_check_terms(&initial, terms).unwrap();
    let locator = prepared.locator().clone();
    let adopted = sdk.adopt_local_check_terms(prepared);
    assert_eq!(
        adopted.disposition,
        LocalAuthorityDispositionV1::Committed,
        "{adopted:?}"
    );
    assert!(sdk
        .resolve_current_local_check_terms(&initial, &locator)
        .is_ok());
    (dir, initial, locator)
}

fn read(
    sdk: &HandbookSdkV1,
    initial: &LocalInitialSetupLocatorV1,
    locator: &LocalCheckLocatorV1,
) -> Result<
    handbook_engine::local_authority::local_check::LocalPlanningCharterReadV1,
    handbook_engine::local_authority::LocalAuthorityRefusalV1,
> {
    sdk.read_local_planning_charter(
        initial,
        locator,
        &locator.terms.agent,
        FIELD_NOTES_PLANNING_INSTRUCTION_V1,
    )
}

#[test]
fn independently_current_projects_copying_instruction_do_not_gain_demo_authority() {
    for changed in [false, true] {
        let (dir, initial, locator) = setup(changed);
        let sdk = HandbookSdkV1::open(dir.path());
        assert_eq!(
            locator.terms.instruction,
            FIELD_NOTES_PLANNING_INSTRUCTION_V1
        );
        assert_ne!(
            locator.terms.approved_content_fingerprint,
            "sha256:6d2f7bae73225ea0fbb3000a2e6857d1fbf80e064d81188f11398b28baa39ec6"
        );
        assert_eq!(
            read(&sdk, &initial, &locator).unwrap_err().code,
            "planning_charter_authority"
        );
        assert!(sdk
            .resolve_local_check_terms_request(&locator)
            .outcome
            .is_some());
    }
}

#[test]
fn selected_successor_cannot_be_replaced_by_the_existing_owner_route() {
    let (dir, initial, locator) = setup(false);
    let sdk = HandbookSdkV1::open(dir.path());
    let mut replacement = locator.terms.clone();
    replacement.delegation = "a changed successor".into();
    replacement.approved_content_fingerprint = replacement.fingerprint_approved_content().unwrap();
    let refusal = match sdk.prepare_local_check_terms(&initial, replacement) {
        Err(e) => e,
        Ok(_) => panic!("an existing selected successor must not be silently replaced"),
    };
    assert_eq!(refusal.code, "already_selected");
    assert!(sdk
        .resolve_current_local_check_terms(&initial, &locator)
        .is_ok());
    assert_eq!(
        read(&sdk, &initial, &locator).unwrap_err().code,
        "planning_charter_authority"
    );
}

#[test]
fn stale_revoked_and_changed_charter_refuse_before_selection_with_history_intact() {
    let (dir, initial, locator) = setup(false);
    let sdk = HandbookSdkV1::open(dir.path());
    let outcome = sdk
        .resolve_current_local_check_terms(&initial, &locator)
        .unwrap();
    sdk.revoke_local_check_terms(
        &locator,
        &LocalRevokeRequestV1 {
            request_key: "planning-read-revoke".into(),
            expected_current_fingerprint: outcome.outcome_fingerprint,
            principal: locator.terms.revocation_principal.clone(),
            source_ref: "test.actual-revocation".into(),
            instruction: "Revoke test authority".into(),
        },
    )
    .unwrap();
    assert!(sdk
        .resolve_local_check_terms_request(&locator)
        .outcome
        .is_some());
    assert_eq!(
        read(&sdk, &initial, &locator).unwrap_err().code,
        "revoked_or_stale_head"
    );

    let (dir, initial, locator) = setup(false);
    let sdk = HandbookSdkV1::open(dir.path());
    fs::remove_file(
        dir.path()
            .join(".handbook/local-operating/local-check-v1/current.json"),
    )
    .unwrap();
    assert!(sdk
        .resolve_local_check_terms_request(&locator)
        .outcome
        .is_some());
    assert_eq!(
        read(&sdk, &initial, &locator).unwrap_err().code,
        "required_state_missing"
    );

    let (dir, initial, locator) = setup(false);
    let sdk = HandbookSdkV1::open(dir.path());
    fs::write(
        dir.path().join(".handbook/project/charter.yaml"),
        b"changed",
    )
    .unwrap();
    assert!(sdk
        .resolve_local_check_terms_request(&locator)
        .outcome
        .is_some());
    assert_eq!(
        read(&sdk, &initial, &locator).unwrap_err().code,
        "required_local_charter_state_missing"
    );
}

#[test]
fn independent_root_and_mismatched_initial_refuse() {
    let (first_dir, first_initial, first_locator) = setup(false);
    let (second_dir, second_initial, _) = setup(false);
    let first = HandbookSdkV1::open(first_dir.path());
    let second = HandbookSdkV1::open(second_dir.path());
    assert!(read(&second, &first_initial, &first_locator).is_err());
    assert_eq!(
        read(&first, &second_initial, &first_locator)
            .unwrap_err()
            .code,
        "project_binding"
    );
}
