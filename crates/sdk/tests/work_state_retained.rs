//! Original foundation and Packet-1 bytes, not rewritten historical fixtures.
//! Physical-root recovery uses separately captured baseline stores in place.
use handbook_sdk::admission_governance::*;
use handbook_sdk::local_check::LocalCheckLocatorV1;
use handbook_sdk::local_setup::LocalInitialSetupLocatorV1;
use handbook_sdk::work_state::*;
use handbook_sdk::{HandbookSdkV1, SetupMode, SetupRequest};
use std::{collections::BTreeMap, fs, path::Path};

const GRANT: &[u8] = include_bytes!("fixtures/work_state_v1/grant.json");
const REQUEST: &[u8] = include_bytes!("fixtures/work_state_v1/committed-request.json");
const RECEIPT: &[u8] = include_bytes!("fixtures/work_state_v1/receipt.json");
const DESCRIPTOR: &[u8] = include_bytes!("fixtures/work_state_v1/descriptor.json");

#[test]
fn original_foundation_json_and_identity_domains_round_trip_exactly() {
    let grant = WorkGrantLocatorV1::from_json_bytes(GRANT).unwrap();
    assert_eq!(grant.canonical_json_bytes().unwrap(), GRANT);
    assert!(matches!(
        grant.terms.scope,
        WorkRegistrationScopeV1::RetainedOrdinary(_)
    ));
    assert!(grant.change.is_none());
    let request = WorkRequestLocatorV1::from_json_bytes(REQUEST).unwrap();
    assert_eq!(request.canonical_json_bytes().unwrap(), REQUEST);
    assert_eq!(request.grant, grant);
    assert!(request.input.display_level.is_none());
    assert!(request.input.composition.role().is_none());
    let receipt: WorkRegistrationReceiptV1 = serde_json::from_slice(RECEIPT).unwrap();
    assert_eq!(serde_json_canonicalizer::to_vec(&receipt).unwrap(), RECEIPT);
    assert_eq!(receipt.locator, request);
    assert_eq!(receipt.snapshot.descriptor_bytes, DESCRIPTOR);
    assert_eq!(
        receipt.snapshot.descriptor,
        WorkDescriptorV1::ordinary_leaf()
    );
    assert_eq!(
        WorkDescriptorV1::ordinary_leaf()
            .canonical_json_bytes()
            .unwrap(),
        DESCRIPTOR
    );
    assert!(receipt.snapshot.display_level.is_none());
    assert!(receipt.snapshot.role.is_none());
    assert_eq!(
        request.item_identity,
        work_item_identity_v1(&receipt.snapshot.project_identity, &request.input.item_key).unwrap()
    );
}

#[test]
fn original_receipt_fingerprint_retains_its_exact_snapshot_preimage() {
    let receipt: WorkRegistrationReceiptV1 = serde_json::from_slice(RECEIPT).unwrap();
    let original_preimage = serde_json::json!({
        "domain": "handbook.work-registration-receipt@1.0.0",
        "content": (&receipt.locator, &receipt.snapshot, &receipt.effect, &receipt.assurance)
    });
    assert_eq!(
        receipt.receipt_fingerprint,
        handbook_engine::DefinitionFingerprint::from_bytes(
            &serde_json_canonicalizer::to_vec(&original_preimage).unwrap()
        )
        .to_string()
    );
}

fn scoped_terms(grant: &WorkGrantLocatorV1, key: &str, planning: bool) -> WorkRegistrationTermsV1 {
    let mut terms = grant.terms.clone();
    terms.request_key = key.into();
    let mut clauses = vec![WorkCompositionClauseV1 {
        descriptor: WorkDescriptorV1::ordinary_leaf(),
        label_max_bytes: 4096,
        work_question_max_bytes: 4096,
        allow_omitted_display_level: true,
        display_levels: vec!["task".into()],
        role_permission: WorkRolePermissionV1::Ordinary {},
    }];
    if planning {
        clauses.push(WorkCompositionClauseV1 {
            descriptor: WorkCompositionV1::planning_output("Which exact requirements are needed?")
                .descriptor,
            label_max_bytes: 4096,
            work_question_max_bytes: 4096,
            allow_omitted_display_level: true,
            display_levels: vec!["planning".into()],
            role_permission: WorkRolePermissionV1::PlanningOutput {
                question_max_bytes: 4096,
            },
        });
    }
    terms.scope = WorkRegistrationScopeV1::explicit(clauses);
    terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
    terms
}

fn change_scope(
    sdk: &HandbookSdkV1,
    initial: &LocalInitialSetupLocatorV1,
    check: &LocalCheckLocatorV1,
    grant: &WorkGrantLocatorV1,
    key: &str,
    planning: bool,
) -> WorkGrantLocatorV1 {
    let prepared = sdk
        .prepare_work_registration_terms(
            initial,
            check,
            scoped_terms(grant, key, planning),
            WorkTermsTransitionV1::ScopeChange(grant),
        )
        .unwrap();
    let locator = prepared.locator().clone();
    let execution = sdk.activate_work_registration_terms(prepared);
    assert_eq!(
        execution.disposition,
        WorkDispositionV1::Committed,
        "{execution:?}"
    );
    locator
}

fn govern(sdk: &HandbookSdkV1, initial: &LocalInitialSetupLocatorV1) {
    let ProjectAdmissionScopeBasisV3::DeferredV2(deferred) = sdk
        .resolve_project_admission_scope_v3(
            &ProjectAdmissionBindingV3::ScopedChoiceV2(initial.deferral.project.clone()),
            ADMISSION_GOVERNANCE_SCOPE_V1,
            None,
        )
        .unwrap()
    else {
        panic!("original deferred basis required")
    };
    let request = DeferredEstablishmentRequestV3::human_approved(
        initial.deferral.project.clone(),
        ExpectedDeferredBasisV2 {
            outcome_fingerprint: deferred.outcome_fingerprint,
            relation_fingerprint: deferred.relation_fingerprint,
        },
        60,
        "retained-v1-governance",
        "Human approved this exact fresh retained-fixture governance establishment.",
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

fn assert_original_records(root: &Path, original: &BTreeMap<String, Vec<u8>>) {
    for (path, bytes) in original {
        // Slot empty entries are deliberately claimed by later transactions;
        // publication staging/checkpoint names are consumed by original closure.
        let name = Path::new(path).file_name().unwrap().to_str().unwrap();
        if name.starts_with("slot-")
            && serde_json::from_slice::<serde_json::Value>(bytes).unwrap()["entry"].is_null()
        {
            continue;
        }
        if matches!(name, "not-attempted.json" | "terminal.pending") && !root.join(path).exists() {
            continue;
        }
        assert_eq!(
            fs::read(root.join(path)).unwrap(),
            *bytes,
            "retained {path}"
        );
    }
}

/// Run explicitly with fresh stores from the historical capture executable.
/// Copying them or manufacturing new physical-lock proof would invalidate this test.
#[test]
#[ignore = "requires original physical roots captured before unified product edits"]
fn original_physical_roots_survive_unified_successors_recovery_governance_revocation_and_reset() {
    let directory = std::path::PathBuf::from(
        std::env::var_os("HANDBOOK_RETAINED_V1_ROOTS").expect("historical capture roots required"),
    );
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
        assert_eq!(
            fs::read_to_string(case.join("baseline-head")).unwrap(),
            "277674e2f9a6350e35c5544fbeb7857f51a6d45d"
        );
        let root = case.join("project");
        let sdk = HandbookSdkV1::open(&root);
        let initial = LocalInitialSetupLocatorV1::from_json_bytes(
            &fs::read(case.join("initial.json")).unwrap(),
        )
        .unwrap();
        let check =
            LocalCheckLocatorV1::from_json_bytes(&fs::read(case.join("check.json")).unwrap())
                .unwrap();
        let grant_bytes = fs::read(case.join("grant.json")).unwrap();
        let grant = WorkGrantLocatorV1::from_json_bytes(&grant_bytes).unwrap();
        let request_bytes = fs::read(case.join("request.json")).unwrap();
        let request = WorkRequestLocatorV1::from_json_bytes(&request_bytes).unwrap();
        let receipt_bytes = fs::read(case.join("receipt.json")).unwrap();
        let receipt: WorkRegistrationReceiptV1 = serde_json::from_slice(&receipt_bytes).unwrap();
        let original: BTreeMap<String, Vec<u8>> =
            serde_json::from_slice(&fs::read(case.join("original-work-records.json")).unwrap())
                .unwrap();
        sdk.resolve_current_work_registration_terms(&initial, &grant)
            .unwrap();

        // Recognition of new values cannot widen a retained ordinary singleton.
        let mut forbidden = request.input.clone();
        forbidden.request_key = "retained-token-refusal".into();
        forbidden.item_key = "retained-token-refusal".into();
        forbidden.display_level = Some("task".into());
        assert!(sdk
            .prepare_work_registration(&initial, &grant, forbidden.clone())
            .is_err());
        forbidden.display_level = None;
        forbidden.composition =
            WorkCompositionV1::planning_output("Which requirements are needed?").into();
        assert!(sdk
            .prepare_work_registration(&initial, &grant, forbidden)
            .is_err());
        assert_original_records(&root, &original);

        let mut head = grant.clone();
        if scenario.ends_with("change") || scenario.ends_with("governance") {
            head = change_scope(&sdk, &initial, &check, &head, "retained-scope-wide", true);
            head = change_scope(
                &sdk,
                &initial,
                &check,
                &head,
                "retained-scope-narrow",
                false,
            );
            assert!(sdk
                .resolve_current_work_registration_terms(&initial, &grant)
                .is_err());
            sdk.resolve_current_work_registration_terms(&initial, &head)
                .unwrap();
            let mut collision = request.input.clone();
            collision.request_key = "cross-schema-collision".into();
            collision.item_key = receipt.snapshot.item_key.clone();
            assert!(sdk
                .register_work_item(
                    sdk.prepare_work_registration(&initial, &head, collision)
                        .unwrap()
                )
                .outcome
                .is_none());
        }
        if scenario.ends_with("governance") {
            govern(&sdk, &initial);
            let mut terms = head.terms.clone();
            terms.request_key = "retained-unchanged-G".into();
            terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
            let prepared = sdk
                .prepare_work_registration_terms(
                    &initial,
                    &check,
                    terms,
                    WorkTermsTransitionV1::EstablishFromDeferred(&head),
                )
                .unwrap();
            let next = prepared.locator().clone();
            assert_eq!(next.terms.scope, head.terms.scope);
            assert_eq!(
                sdk.activate_work_registration_terms(prepared).disposition,
                WorkDispositionV1::Committed
            );
            head = next;
        } else if scenario.ends_with("revoke") {
            let revoke = sdk
                .prepare_work_registration_revocation(
                    &head,
                    "retained-revoke",
                    &head.terms.revocation_principal,
                    "human.retained-revoke",
                    "Revoke exact original retained authority",
                )
                .unwrap();
            assert_eq!(
                sdk.revoke_work_registration_terms(&revoke).disposition,
                WorkDispositionV1::Committed
            );
            assert!(sdk
                .prepare_work_registration_terms(
                    &initial,
                    &check,
                    scoped_terms(&head, "no-reinstatement", false),
                    WorkTermsTransitionV1::ScopeChange(&head)
                )
                .is_err());
        } else if scenario.ends_with("reset") {
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
        drop(sdk);
        let reopened = HandbookSdkV1::open(&root);
        assert_eq!(grant.canonical_json_bytes().unwrap(), grant_bytes);
        assert_eq!(request.canonical_json_bytes().unwrap(), request_bytes);
        assert_eq!(
            serde_json_canonicalizer::to_vec(&receipt).unwrap(),
            receipt_bytes
        );
        assert_eq!(
            reopened
                .resolve_current_work_item(&grant, &receipt.snapshot.item_key)
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

        let recovered = reopened.recover_work_registration_request(&request);
        let expected = if scenario.starts_with("committed") {
            WorkDispositionV1::Unaccepted
        } else if scenario.starts_with("attempted") {
            WorkDispositionV1::Uncertain
        } else if scenario == "pending-live" {
            WorkDispositionV1::Committed
        } else {
            WorkDispositionV1::NonCommitted
        };
        assert_eq!(recovered.disposition, expected, "{scenario}: {recovered:?}");
        if expected == WorkDispositionV1::Committed {
            let result = recovered.outcome.unwrap();
            assert_eq!(result.locator, request);
            assert_eq!(
                result.snapshot.descriptor_bytes,
                fs::read(case.join("descriptor.json")).unwrap()
            );
            assert!(result.snapshot.display_level.is_none());
            assert!(result.snapshot.role.is_none());
            assert_eq!(
                reopened
                    .recover_work_registration_request(&request)
                    .disposition,
                WorkDispositionV1::Replayed
            );
        } else if expected == WorkDispositionV1::Uncertain {
            assert!(recovered.outcome.is_none());
            assert!(
                matches!(reopened.resolve_current_work_item(&grant, &request.input.item_key).unwrap(), WorkCurrentItemV1::Pending { original_request } if original_request == request)
            );
            let mut blind = request.input.clone();
            blind.request_key = "blind-replacement-refuses".into();
            assert!(reopened
                .register_work_item(
                    reopened
                        .prepare_work_registration(&initial, &head, blind)
                        .unwrap()
                )
                .outcome
                .is_none());
        }
        assert_original_records(&root, &original);
    }
}
/// The baseline executable must emit new roots at their final path. This test
/// reopens those roots directly; copies would invalidate physical owner proof.
#[test]
#[ignore = "requires physical roots emitted by the untouched Packet-1 writer"]
fn original_packet1_physical_roots_survive_kinds_support_and_original_recovery() {
    let directory = std::path::PathBuf::from(
        std::env::var_os("HANDBOOK_RETAINED_PACKET1_ROOTS")
            .expect("untouched Packet-1 capture roots required"),
    );
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
        assert_eq!(
            fs::read_to_string(case.join("baseline-head")).unwrap(),
            "d7ff969c348929b89f4f4abe1e3536fd55c99e9d"
        );
        let root = case.join("project");
        let sdk = HandbookSdkV1::open(&root);
        let initial = LocalInitialSetupLocatorV1::from_json_bytes(
            &fs::read(case.join("initial.json")).unwrap(),
        )
        .unwrap();
        let check =
            LocalCheckLocatorV1::from_json_bytes(&fs::read(case.join("check.json")).unwrap())
                .unwrap();
        let original_grant_bytes = fs::read(case.join("original-grant.json")).unwrap();
        let original_grant = WorkGrantLocatorV1::from_json_bytes(&original_grant_bytes).unwrap();
        let grant_bytes = fs::read(case.join("grant.json")).unwrap();
        let grant = WorkGrantLocatorV1::from_json_bytes(&grant_bytes).unwrap();
        let request_bytes = fs::read(case.join("request.json")).unwrap();
        let request = WorkRequestLocatorV1::from_json_bytes(&request_bytes).unwrap();
        let receipt_bytes = fs::read(case.join("receipts.json")).unwrap();
        let receipts: Vec<WorkRegistrationReceiptV1> =
            serde_json::from_slice(&receipt_bytes).unwrap();
        let original: BTreeMap<String, Vec<u8>> =
            serde_json::from_slice(&fs::read(case.join("original-work-records.json")).unwrap())
                .unwrap();
        assert_eq!(receipts.len(), 3);
        assert!(sdk
            .resolve_current_work_registration_terms(&initial, &original_grant)
            .is_err());
        sdk.resolve_current_work_registration_terms(&initial, &grant)
            .unwrap();
        assert_original_records(&root, &original);

        let mut head = grant.clone();
        if scenario.ends_with("change") || scenario.ends_with("governance") {
            head = change_scope(&sdk, &initial, &check, &head, "packet1-retained-wide", true);
            head = change_scope(
                &sdk,
                &initial,
                &check,
                &head,
                "packet1-retained-ordinary-only",
                false,
            );
            for receipt in &receipts {
                let mut collision = receipt.locator.input.clone();
                collision.request_key =
                    format!("cross-packet-collision-{}", receipt.snapshot.item_key);
                assert!(sdk
                    .register_work_item(
                        sdk.prepare_work_registration(&initial, &head, collision)
                            .unwrap()
                    )
                    .outcome
                    .is_none());
            }
        }
        if scenario.ends_with("governance") {
            govern(&sdk, &initial);
            let mut terms = head.terms.clone();
            terms.request_key = "packet1-retained-unchanged-G".into();
            terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
            let prepared = sdk
                .prepare_work_registration_terms(
                    &initial,
                    &check,
                    terms,
                    WorkTermsTransitionV1::EstablishFromDeferred(&head),
                )
                .unwrap();
            assert_eq!(prepared.locator().terms.scope, head.terms.scope);
            head = prepared.locator().clone();
            assert_eq!(
                sdk.activate_work_registration_terms(prepared).disposition,
                WorkDispositionV1::Committed
            );
        } else if scenario.ends_with("revoke") {
            let revoke = sdk
                .prepare_work_registration_revocation(
                    &head,
                    "packet1-retained-revoke",
                    &head.terms.revocation_principal,
                    "human.retained-revoke",
                    "Revoke exact original Packet-1 authority",
                )
                .unwrap();
            assert_eq!(
                sdk.revoke_work_registration_terms(&revoke).disposition,
                WorkDispositionV1::Committed
            );
        } else if scenario.ends_with("reset") {
            assert!(
                sdk.run_setup(&SetupRequest {
                    mode: SetupMode::Refresh,
                    rewrite: false,
                    reset_state: true,
                })
                .unwrap()
                .reset_applied
            );
        }
        drop(sdk);
        let reopened = HandbookSdkV1::open(&root);
        assert_eq!(
            original_grant.canonical_json_bytes().unwrap(),
            original_grant_bytes
        );
        assert_eq!(grant.canonical_json_bytes().unwrap(), grant_bytes);
        assert_eq!(request.canonical_json_bytes().unwrap(), request_bytes);
        assert_eq!(
            serde_json_canonicalizer::to_vec(&receipts).unwrap(),
            receipt_bytes
        );
        for receipt in &receipts {
            assert_eq!(
                reopened
                    .resolve_current_work_item(&grant, &receipt.snapshot.item_key)
                    .unwrap(),
                WorkCurrentItemV1::Open(receipt.snapshot.clone())
            );
            assert_eq!(
                reopened
                    .resolve_work_registration_request(&receipt.locator)
                    .outcome
                    .unwrap(),
                *receipt
            );
            assert_eq!(
                reopened
                    .recover_work_registration_request(&receipt.locator)
                    .outcome
                    .unwrap(),
                *receipt
            );
            assert_eq!(
                receipt.snapshot.descriptor.completion_support,
                "unsupported_not_assessed"
            );
        }
        for original in [&original_grant, &grant] {
            assert_eq!(
                reopened
                    .recover_work_registration_terms_request(original)
                    .disposition,
                WorkDispositionV1::Replayed
            );
        }
        let recovered = reopened.recover_work_registration_request(&request);
        let expected = if scenario.starts_with("committed") {
            WorkDispositionV1::Unaccepted
        } else if scenario.starts_with("attempted") {
            WorkDispositionV1::Uncertain
        } else if scenario == "pending-live" {
            WorkDispositionV1::Committed
        } else {
            WorkDispositionV1::NonCommitted
        };
        assert_eq!(recovered.disposition, expected, "{scenario}: {recovered:?}");
        if expected == WorkDispositionV1::Committed {
            let receipt = recovered.outcome.unwrap();
            assert_eq!(receipt.locator, request);
            assert_eq!(
                receipt.snapshot.descriptor,
                request
                    .input
                    .composition
                    .explicit_composition()
                    .unwrap()
                    .descriptor
            );
            assert_eq!(
                receipt.snapshot.role,
                request.input.composition.role().cloned()
            );
            assert_eq!(receipt.snapshot.display_level, request.input.display_level);
            assert_eq!(
                reopened
                    .recover_work_registration_request(&request)
                    .disposition,
                WorkDispositionV1::Replayed
            );
        } else if expected == WorkDispositionV1::Uncertain {
            assert!(recovered.outcome.is_none());
            assert!(matches!(
                reopened.resolve_current_work_item(&grant, &request.input.item_key).unwrap(),
                WorkCurrentItemV1::Pending { original_request } if original_request == request
            ));
            let mut blind = request.input.clone();
            blind.request_key = "packet1-blind-replacement-refuses".into();
            assert!(reopened
                .register_work_item(
                    reopened
                        .prepare_work_registration(&initial, &head, blind)
                        .unwrap()
                )
                .outcome
                .is_none());
        }
        assert_original_records(&root, &original);
    }
}
