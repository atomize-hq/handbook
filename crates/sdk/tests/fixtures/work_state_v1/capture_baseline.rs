//! Historical capture harness for the foundation API at 277674e2.
//! Public SDK effects produce truth; the native fault hook captures real prefixes.
mod work_state_support;

use handbook_sdk::admission_governance::*;
use handbook_sdk::local_check::*;
use handbook_sdk::local_setup::{LocalBindingV1, LocalInitialSetupLocatorV1};
use handbook_sdk::work_state::*;
use handbook_sdk::HandbookSdkV1;
use std::fs;
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

fn snapshot_files(
    root: &std::path::Path,
    current: &std::path::Path,
    map: &mut std::collections::BTreeMap<String, Vec<u8>>,
) {
    for entry in fs::read_dir(current).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            snapshot_files(root, &path, map);
        } else {
            map.insert(
                path.strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/"),
                fs::read(path).unwrap(),
            );
        }
    }
}

#[test]
fn capture_retained_v1() {
    use handbook_engine::work_state::{NativeWorkStateV1, WorkWriteBoundaryV1 as B};
    let directory =
        std::path::PathBuf::from(std::env::var_os("HANDBOOK_RETAINED_V1_CAPTURE").unwrap());
    fs::create_dir_all(&directory).unwrap();
    for scenario in [
        "committed-change",
        "committed-governance",
        "committed-revoke",
        "committed-reset",
        "pending-live",
        "pending-change",
        "pending-governance",
        "pending-revoke",
        "pending-reset",
        "attempted-live",
        "attempted-change",
        "attempted-governance",
        "attempted-revoke",
        "attempted-reset",
    ] {
        let case = directory.join(scenario);
        let root = case.join("project");
        fs::create_dir_all(&root).unwrap();
        let sdk = HandbookSdkV1::open(&root);
        let (initial, check) = setup(&sdk);
        let grant = activate(&sdk, &initial, &check, "retained-V1-grant", None);
        let receipt = register(
            &sdk,
            &initial,
            &grant,
            "retained-V1-committed",
            "retained-V1-item",
        );
        let prepared = sdk
            .prepare_work_registration(
                &initial,
                &grant,
                input(&grant, "retained-V1-prefix", "retained-V1-prefix-item"),
            )
            .unwrap();
        let request = prepared.locator().clone();
        if !scenario.starts_with("committed") {
            let at = if scenario.starts_with("attempted") {
                B::PublicationAttempt
            } else {
                B::Snapshot
            };
            let execution =
                NativeWorkStateV1::register_with_boundary_for_test(&root, prepared, |boundary| {
                    if boundary == at {
                        Err(handbook_engine::local_authority::LocalAuthorityRefusalV1 {
                            code: "captured_original_prefix".into(),
                            detail: "Real baseline prefix retained before unified implementation"
                                .into(),
                            unresolved_request: None,
                        })
                    } else {
                        Ok(())
                    }
                });
            assert_eq!(execution.disposition, WorkDispositionV1::Uncertain);
        }
        for (name, bytes) in [
            ("initial.json", initial.canonical_json_bytes().unwrap()),
            ("check.json", check.canonical_json_bytes().unwrap()),
            ("grant.json", grant.canonical_json_bytes().unwrap()),
            ("request.json", request.canonical_json_bytes().unwrap()),
            (
                "committed-request.json",
                receipt.locator.canonical_json_bytes().unwrap(),
            ),
            (
                "receipt.json",
                serde_json_canonicalizer::to_vec(&serde_json::to_value(&receipt).unwrap()).unwrap(),
            ),
            ("descriptor.json", receipt.snapshot.descriptor_bytes.clone()),
        ] {
            fs::write(case.join(name), bytes).unwrap();
        }
        let mut files = std::collections::BTreeMap::new();
        snapshot_files(&root, &root.join(".handbook/work-state"), &mut files);
        for name in [
            ".handbook-work-state.v1.json",
            ".handbook-work-state-bootstrap.v1.json",
        ] {
            files.insert(name.into(), fs::read(root.join(name)).unwrap());
        }
        fs::write(
            case.join("original-work-records.json"),
            serde_json::to_vec(&files).unwrap(),
        )
        .unwrap();
        fs::write(
            case.join("baseline-head"),
            "277674e2f9a6350e35c5544fbeb7857f51a6d45d",
        )
        .unwrap();
    }
}
