//! Faults surround real retained native writes. Test hooks are absent from the
//! ordinary production consumer; no fabricated grant or state outcome is used.
mod work_state_support;
use handbook_engine::local_authority::LocalRevokeRequestV1;
use handbook_engine::work_state::{
    NativeWorkStateV1, WorkWriteBoundaryV1 as B, WORK_STATE_FULL_OWNER_REACH_V1,
};
use handbook_sdk::admission_governance::*;
use handbook_sdk::local_check::*;
use handbook_sdk::local_setup::{LocalAuthorityRefusalV1, LocalInitialSetupLocatorV1};
use handbook_sdk::work_state::*;
use handbook_sdk::{HandbookSdkV1, SetupMode, SetupRequest};
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
    time::{Duration, Instant},
};
use tempfile::tempdir;
use work_state_support::{establish_initial, menu_scope, ordinary_scope, supplied_successor_terms};

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
fn terms(check: &LocalCheckLocatorV1, key: &str) -> WorkRegistrationTermsV1 {
    let mut t = WorkRegistrationTermsV1 {
        request_key: key.into(),
        rule_ref: WORK_REGISTRATION_RULE_V1.into(),
        mode: "local_advisory".into(),
        principal: check.terms.principal.clone(),
        agent: check.terms.agent.clone(),
        instruction: "Register ordinary leaves".into(),
        delegation: "same-project leaf registration".into(),
        approval_source_ref: "human.work".into(),
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
    t.approved_content_fingerprint = t.fingerprint_approved_content().unwrap();
    t
}
fn activate(
    sdk: &HandbookSdkV1,
    initial: &LocalInitialSetupLocatorV1,
    check: &LocalCheckLocatorV1,
    key: &str,
    prior: Option<&WorkGrantLocatorV1>,
) -> WorkGrantLocatorV1 {
    let p = sdk
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
    let l = p.locator().clone();
    let e = sdk.activate_work_registration_terms(p);
    assert_eq!(e.disposition, WorkDispositionV1::Committed, "{e:?}");
    l
}
fn input(grant: &WorkGrantLocatorV1, key: &str, item: &str) -> WorkRegistrationInputV1 {
    WorkRegistrationInputV1 {
        request_key: key.into(),
        item_key: item.into(),
        label: "Leaf".into(),
        question: "Exact bounded work question".into(),
        actor: grant.terms.agent.clone(),
        expected_state: "absent".into(),
        composition: WorkCompositionV1::ordinary_leaf().into(),
        display_level: None,
    }
}
fn govern(sdk: &HandbookSdkV1, initial: &LocalInitialSetupLocatorV1) {
    let project = ProjectAdmissionBindingV3::ScopedChoiceV2(initial.deferral.project.clone());
    let ProjectAdmissionScopeBasisV3::DeferredV2(d) = sdk
        .resolve_project_admission_scope_v3(&project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
        .unwrap()
    else {
        panic!("D required")
    };
    let request = DeferredEstablishmentRequestV3::human_approved(
        initial.deferral.project.clone(),
        ExpectedDeferredBasisV2 {
            outcome_fingerprint: d.outcome_fingerprint,
            relation_fingerprint: d.relation_fingerprint,
        },
        60,
        "native-work-G",
        "Human selected supplied test G",
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
fn revoke(sdk: &HandbookSdkV1, grant: &WorkGrantLocatorV1) {
    let r = sdk
        .prepare_work_registration_revocation(
            grant,
            "revoke",
            &grant.terms.revocation_principal,
            "human.revoke",
            "Revoke exact work terms",
        )
        .unwrap();
    let e = sdk.revoke_work_registration_terms(&r);
    assert_eq!(e.disposition, WorkDispositionV1::Committed, "{e:?}");
}
fn revoke_check(
    sdk: &HandbookSdkV1,
    initial: &LocalInitialSetupLocatorV1,
    check: &LocalCheckLocatorV1,
) {
    let o = sdk
        .resolve_current_local_check_terms(initial, check)
        .unwrap();
    sdk.revoke_local_check_terms(
        check,
        &LocalRevokeRequestV1 {
            request_key: "revoke-check".into(),
            expected_current_fingerprint: o.outcome_fingerprint,
            principal: check.terms.revocation_principal.clone(),
            source_ref: "human.revoke".into(),
            instruction: "Revoke check".into(),
        },
    )
    .unwrap();
}
fn reset(sdk: &HandbookSdkV1) {
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
fn fault(at: B) -> impl FnMut(B) -> Result<(), LocalAuthorityRefusalV1> {
    move |b| {
        if b == at {
            Err(LocalAuthorityRefusalV1 {
                code: "test_interruption".into(),
                detail: "lost native reply at real durable boundary".into(),
                unresolved_request: None,
            })
        } else {
            Ok(())
        }
    }
}
const PREFIXES: [B; 7] = [
    B::BeforeIntent,
    B::Intent,
    B::Decision,
    B::Snapshot,
    B::PublicationAttempt,
    B::Terminal,
    B::Reply,
];

#[test]
fn registration_every_prefix_crosses_original_successor_governance_revocation_and_reset() {
    for at in PREFIXES {
        for transition in 0..5 {
            let dir = tempdir().unwrap();
            let sdk = HandbookSdkV1::open(dir.path());
            let (initial, check) = setup(&sdk);
            let grant = activate(&sdk, &initial, &check, "grant-D", None);
            let p = sdk
                .prepare_work_registration(&initial, &grant, input(&grant, "original", "item"))
                .unwrap();
            let loc = p.locator().clone();
            let interrupted =
                NativeWorkStateV1::register_with_boundary_for_test(dir.path(), p, fault(at));
            assert!(interrupted.outcome.is_none(), "{at:?} {interrupted:?}");
            let committed = matches!(at, B::Terminal | B::Reply);
            let attempted = at == B::PublicationAttempt;
            let current = sdk.resolve_current_work_item(&grant, "item").unwrap();
            assert!(
                match (&current, at) {
                    (WorkCurrentItemV1::Open(_), B::Terminal | B::Reply) => true,
                    (WorkCurrentItemV1::Absent, B::BeforeIntent) => true,
                    (WorkCurrentItemV1::Pending { .. }, _) => !committed,
                    _ => false,
                },
                "{at:?} {current:?}"
            );
            match transition {
                0 => {}
                1 => {
                    govern(&sdk, &initial);
                    if committed || at == B::BeforeIntent {
                        activate(&sdk, &initial, &check, "grant-G", Some(&grant));
                    }
                }
                2 => govern(&sdk, &initial),
                3 => revoke(&sdk, &grant),
                4 => reset(&sdk),
                _ => unreachable!(),
            }
            let recovered = sdk.recover_work_registration_request(&loc);
            let expected = if committed {
                WorkDispositionV1::Replayed
            } else if attempted {
                WorkDispositionV1::Uncertain
            } else if at == B::BeforeIntent {
                WorkDispositionV1::Unaccepted
            } else if transition == 0 {
                WorkDispositionV1::Committed
            } else {
                WorkDispositionV1::NonCommitted
            };
            assert_eq!(
                recovered.disposition, expected,
                "prefix {at:?}, transition {transition}: {recovered:?}"
            );
            if recovered.outcome.is_some() {
                assert!(matches!(
                    sdk.resolve_current_work_item(&grant, "item").unwrap(),
                    WorkCurrentItemV1::Open(_)
                ));
            }
            if attempted {
                assert!(sdk
                    .prepare_work_registration(
                        &initial,
                        &grant,
                        input(&grant, "blind-retry", "item")
                    )
                    .is_err());
                assert!(matches!(
                    sdk.resolve_current_work_item(&grant, "item").unwrap(),
                    WorkCurrentItemV1::Pending { .. }
                ));
            }
            if expected == WorkDispositionV1::NonCommitted && transition == 2 {
                let g = activate(&sdk, &initial, &check, "grant-G", Some(&grant));
                let p = sdk
                    .prepare_work_registration(&initial, &g, input(&g, "proved-new-key", "item"))
                    .unwrap();
                assert!(sdk.register_work_item(p).outcome.is_some());
                assert_eq!(
                    sdk.resolve_work_registration_request(&loc).disposition,
                    WorkDispositionV1::NonCommitted
                );
            }
        }
    }
}
#[test]
fn activation_and_revocation_have_the_same_terminal_and_original_recovery_boundary() {
    for activation in [true, false] {
        for at in PREFIXES {
            for transition in 0..4 {
                let dir = tempdir().unwrap();
                let sdk = HandbookSdkV1::open(dir.path());
                let (initial, check) = setup(&sdk);
                let (grant, revocation) = if activation {
                    let p = sdk
                        .prepare_work_registration_terms(
                            &initial,
                            &check,
                            terms(&check, "activation"),
                            WorkTermsTransitionV1::Initial,
                        )
                        .unwrap();
                    let grant = p.locator().clone();
                    let e = NativeWorkStateV1::activate_with_boundary_for_test(
                        dir.path(),
                        p,
                        fault(at),
                    );
                    assert!(e.outcome.is_none());
                    (grant, None)
                } else {
                    let grant = activate(&sdk, &initial, &check, "active", None);
                    let r = sdk
                        .prepare_work_registration_revocation(
                            &grant,
                            "revoke",
                            &grant.terms.principal,
                            "human.revoke",
                            "Revoke exact grant",
                        )
                        .unwrap();
                    let e =
                        NativeWorkStateV1::revoke_with_boundary_for_test(dir.path(), &r, fault(at));
                    assert!(e.outcome.is_none());
                    (grant, Some(r))
                };
                match transition {
                    0 => {}
                    1 => govern(&sdk, &initial),
                    2 => revoke_check(&sdk, &initial, &check),
                    3 => reset(&sdk),
                    _ => unreachable!(),
                }
                let disposition = if let Some(r) = &revocation {
                    sdk.recover_work_registration_revocation_request(r)
                        .disposition
                } else {
                    sdk.recover_work_registration_terms_request(&grant)
                        .disposition
                };
                let expected = if matches!(at, B::Terminal | B::Reply) {
                    WorkDispositionV1::Replayed
                } else if at == B::PublicationAttempt {
                    WorkDispositionV1::Uncertain
                } else if at == B::BeforeIntent {
                    WorkDispositionV1::Unaccepted
                } else if transition == 0 {
                    WorkDispositionV1::Committed
                } else {
                    WorkDispositionV1::NonCommitted
                };
                assert_eq!(
                    disposition, expected,
                    "activation {activation}, {at:?}, transition {transition}"
                );
                if activation
                    && expected != WorkDispositionV1::Committed
                    && expected != WorkDispositionV1::Replayed
                {
                    assert!(sdk
                        .prepare_work_registration(
                            &initial,
                            &grant,
                            input(&grant, "self-authorization", "leaf")
                        )
                        .is_err());
                }
            }
        }
    }
}
#[test]
fn missing_attempt_history_corrupt_exact_records_and_substituted_root_lock_never_prove_absence() {
    for damage in [
        "attempted.json",
        "snapshot.json",
        "intent.json",
        "terminal.json",
    ] {
        let dir = tempdir().unwrap();
        let sdk = HandbookSdkV1::open(dir.path());
        let (initial, check) = setup(&sdk);
        let grant = activate(&sdk, &initial, &check, "grant", None);
        let p = sdk
            .prepare_work_registration(&initial, &grant, input(&grant, "damaged", "item"))
            .unwrap();
        let l = p.locator().clone();
        let at = if damage == "attempted.json" {
            B::PublicationAttempt
        } else {
            B::Terminal
        };
        NativeWorkStateV1::register_with_boundary_for_test(dir.path(), p, fault(at));
        fs::remove_file(
            dir.path()
                .join(format!(".handbook/work-state/v1/item-damaged/{damage}")),
        )
        .unwrap();
        assert_eq!(
            sdk.recover_work_registration_request(&l).disposition,
            WorkDispositionV1::Uncertain
        );
        let current = sdk.resolve_current_work_item(&grant, "item");
        if damage == "terminal.json" {
            // Exact accepted intent/attempt proof still identifies the reserved
            // request. Missing terminal cannot prove open or absence; recovery
            // above remains uncertain rather than publishing another terminal.
            assert_eq!(
                current.unwrap(),
                WorkCurrentItemV1::Pending {
                    original_request: l.clone()
                }
            );
        } else {
            assert!(current.is_err());
        }
        assert!(sdk
            .prepare_work_registration(&initial, &grant, input(&grant, "new", "item"))
            .is_err());
    }
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let grant = activate(&sdk, &initial, &check, "grant", None);
    let p = sdk
        .prepare_work_registration(&initial, &grant, input(&grant, "original", "item"))
        .unwrap();
    let l = p.locator().clone();
    assert!(sdk.register_work_item(p).outcome.is_some());
    let other = tempdir().unwrap();
    assert_eq!(
        HandbookSdkV1::open(other.path())
            .recover_work_registration_request(&l)
            .disposition,
        WorkDispositionV1::Refused
    );
    let lock = dir.path().join(".handbook/work-state/work.lock");
    fs::rename(&lock, lock.with_extension("old")).unwrap();
    fs::write(&lock, []).unwrap();
    assert!(sdk.resolve_current_work_item(&grant, "item").is_err());
    assert_eq!(
        sdk.recover_work_registration_request(&l).disposition,
        WorkDispositionV1::Uncertain
    );
}
#[test]
fn bounded_peak_inventory_preserves_old_recovery_and_request_reservations_are_local() {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let grant = activate(&sdk, &initial, &check, "grant", None);
    let p = sdk
        .prepare_work_registration(&initial, &grant, input(&grant, "pending", "reserved"))
        .unwrap();
    let pending = p.locator().clone();
    NativeWorkStateV1::register_with_boundary_for_test(dir.path(), p, fault(B::PublicationAttempt));
    let mut first = None;
    // 32 retained transactions include the grant and unresolved reservation;
    // all candidate/attempt/terminal or noncommit files fit the declared bound.
    for i in 0..30 {
        let k = format!("item-{i}");
        let p = sdk
            .prepare_work_registration(&initial, &grant, input(&grant, &k, &k))
            .unwrap();
        let l = p.locator().clone();
        assert!(sdk.register_work_item(p).outcome.is_some());
        if first.is_none() {
            first = Some(l);
        }
    }
    let overflow = sdk
        .prepare_work_registration(&initial, &grant, input(&grant, "overflow", "overflow"))
        .unwrap();
    assert_eq!(
        sdk.register_work_item(overflow).disposition,
        WorkDispositionV1::Refused
    );
    assert!(sdk
        .resolve_work_registration_request(&first.unwrap())
        .outcome
        .is_some());
    assert_eq!(
        sdk.recover_work_registration_request(&pending).disposition,
        WorkDispositionV1::Uncertain
    );
    fs::write(
        dir.path().join(".handbook/work-state/v1/unknown.json"),
        b"{}",
    )
    .unwrap();
    assert!(sdk.resolve_current_work_item(&grant, "item-0").is_err());
}

fn wait_file(path: &Path) {
    let start = Instant::now();
    while !path.exists() {
        assert!(
            start.elapsed() < Duration::from_secs(30),
            "child did not reach native checkpoint"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}
fn child(repo: &Path, mode: &str) -> std::process::Child {
    Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "native_work_child", "--ignored", "--nocapture"])
        .env("WORK_REPO", repo)
        .env("WORK_MODE", mode)
        .env("RUST_MIN_STACK", "33554432")
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
}
#[test]
fn native_process_serialization_holds_original_guards_through_registration_and_activation_terminal()
{
    for activation in [false, true] {
        for transition in [
            "revoke",
            "govern",
            "work-revoke",
            "duplicate-creator",
            "reader",
        ] {
            if activation && !matches!(transition, "revoke" | "govern") {
                continue;
            }
            let dir = tempdir().unwrap();
            let sdk = HandbookSdkV1::open(dir.path());
            let (initial, check) = setup(&sdk);
            fs::write(
                dir.path().join("initial.test.json"),
                initial.canonical_json_bytes().unwrap(),
            )
            .unwrap();
            fs::write(
                dir.path().join("check.test.json"),
                check.canonical_json_bytes().unwrap(),
            )
            .unwrap();
            if !activation {
                let grant = activate(&sdk, &initial, &check, "grant", None);
                fs::write(
                    dir.path().join("grant.test.json"),
                    grant.canonical_json_bytes().unwrap(),
                )
                .unwrap();
            }
            let mut writer = child(dir.path(), if activation { "activate" } else { "register" });
            wait_file(&dir.path().join("ready.test"));
            let mut competing = child(dir.path(), transition);
            std::thread::sleep(Duration::from_millis(150));
            assert!(
                competing.try_wait().unwrap().is_none(),
                "authority changed before terminal publication"
            );
            fs::write(dir.path().join("release.test"), []).unwrap();
            assert!(writer.wait().unwrap().success());
            assert!(competing.wait().unwrap().success());
            let grant = WorkGrantLocatorV1::from_json_bytes(
                &fs::read(dir.path().join("grant.test.json")).unwrap(),
            )
            .unwrap();
            assert_eq!(
                sdk.resolve_work_registration_terms_request(&grant)
                    .disposition,
                WorkDispositionV1::Replayed
            );
            if matches!(transition, "revoke" | "govern" | "work-revoke") {
                assert!(sdk
                    .resolve_current_work_registration_terms(&initial, &grant)
                    .is_err());
            }
            if !activation {
                assert!(matches!(
                    sdk.resolve_current_work_item(&grant, "child-item").unwrap(),
                    WorkCurrentItemV1::Open(_)
                ));
            }
        }
    }
}
#[test]
#[ignore]
fn native_work_child() {
    let repo = std::env::var("WORK_REPO").unwrap();
    let repo = Path::new(&repo);
    let sdk = HandbookSdkV1::open(repo);
    let initial = LocalInitialSetupLocatorV1::from_json_bytes(
        &fs::read(repo.join("initial.test.json")).unwrap(),
    )
    .unwrap();
    let check =
        LocalCheckLocatorV1::from_json_bytes(&fs::read(repo.join("check.test.json")).unwrap())
            .unwrap();
    let mode = std::env::var("WORK_MODE").unwrap();
    if mode == "govern" {
        govern(&sdk, &initial);
        return;
    }
    if mode == "revoke" {
        revoke_check(&sdk, &initial, &check);
        return;
    }
    if matches!(
        mode.as_str(),
        "work-revoke" | "duplicate-creator" | "reader"
    ) {
        let grant =
            WorkGrantLocatorV1::from_json_bytes(&fs::read(repo.join("grant.test.json")).unwrap())
                .unwrap();
        match mode.as_str() {
            "work-revoke" => revoke(&sdk, &grant),
            "duplicate-creator" => assert!(sdk
                .prepare_work_registration(
                    &initial,
                    &grant,
                    input(&grant, "duplicate-request", "child-item")
                )
                .is_err()),
            _ => assert!(matches!(
                sdk.resolve_current_work_item(&grant, "child-item").unwrap(),
                WorkCurrentItemV1::Open(_)
            )),
        }
        return;
    }
    let pause_at = match mode.as_str() {
        "crash-attempt" => B::PublicationAttempt,
        "crash-terminal" => B::Terminal,
        _ => B::Snapshot,
    };
    let mut pause = |b| {
        if b == pause_at {
            fs::write(repo.join("ready.test"), []).unwrap();
            wait_file(&repo.join("release.test"));
        }
        Ok(())
    };
    if mode == "activate" {
        let p = sdk
            .prepare_work_registration_terms(
                &initial,
                &check,
                terms(&check, "grant"),
                WorkTermsTransitionV1::Initial,
            )
            .unwrap();
        let grant = p.locator().clone();
        let e = NativeWorkStateV1::activate_with_boundary_for_test(repo, p, &mut pause);
        assert_eq!(e.disposition, WorkDispositionV1::Committed, "{e:?}");
        fs::write(
            repo.join("grant.test.json"),
            grant.canonical_json_bytes().unwrap(),
        )
        .unwrap();
    } else {
        let grant =
            WorkGrantLocatorV1::from_json_bytes(&fs::read(repo.join("grant.test.json")).unwrap())
                .unwrap();
        let p = sdk
            .prepare_work_registration(
                &initial,
                &grant,
                input(&grant, "child-request", "child-item"),
            )
            .unwrap();
        fs::write(
            repo.join("request.test.json"),
            p.locator().canonical_json_bytes().unwrap(),
        )
        .unwrap();
        let e = NativeWorkStateV1::register_with_boundary_for_test(repo, p, &mut pause);
        assert_eq!(e.disposition, WorkDispositionV1::Committed, "{e:?}");
    }
}

#[test]
fn real_process_death_preserves_pending_attempt_uncertainty_and_committed_terminal_proof() {
    for (mode, expected) in [
        ("crash-snapshot", WorkDispositionV1::Committed),
        ("crash-attempt", WorkDispositionV1::Uncertain),
        ("crash-terminal", WorkDispositionV1::Replayed),
    ] {
        let dir = tempdir().unwrap();
        let sdk = HandbookSdkV1::open(dir.path());
        let (initial, check) = setup(&sdk);
        let grant = activate(&sdk, &initial, &check, "grant", None);
        fs::write(
            dir.path().join("initial.test.json"),
            initial.canonical_json_bytes().unwrap(),
        )
        .unwrap();
        fs::write(
            dir.path().join("check.test.json"),
            check.canonical_json_bytes().unwrap(),
        )
        .unwrap();
        fs::write(
            dir.path().join("grant.test.json"),
            grant.canonical_json_bytes().unwrap(),
        )
        .unwrap();
        let mut writer = child(dir.path(), mode);
        wait_file(&dir.path().join("ready.test"));
        let original = WorkRequestLocatorV1::from_json_bytes(
            &fs::read(dir.path().join("request.test.json")).unwrap(),
        )
        .unwrap();
        writer.kill().unwrap();
        writer.wait().unwrap();
        let recovered = sdk.recover_work_registration_request(&original);
        assert_eq!(recovered.disposition, expected, "{mode}: {recovered:?}");
        assert!(
            match sdk.resolve_current_work_item(&grant, "child-item").unwrap() {
                WorkCurrentItemV1::Pending { .. } => expected == WorkDispositionV1::Uncertain,
                WorkCurrentItemV1::Open(_) => expected != WorkDispositionV1::Uncertain,
                _ => false,
            }
        );
    }
}
#[test]
fn explicit_g_first_activation_and_interrupted_compatible_g_reestablishment_recover_original_rule()
{
    for prior in [false, true] {
        for at in PREFIXES {
            let dir = tempdir().unwrap();
            let sdk = HandbookSdkV1::open(dir.path());
            let (initial, check) = setup(&sdk);
            let d = if prior {
                Some(activate(&sdk, &initial, &check, "grant-D", None))
            } else {
                None
            };
            govern(&sdk, &initial);
            let p = sdk
                .prepare_work_registration_terms(
                    &initial,
                    &check,
                    terms(&check, "grant-G"),
                    d.as_ref().map_or(
                        WorkTermsTransitionV1::Initial,
                        WorkTermsTransitionV1::EstablishFromDeferred,
                    ),
                )
                .unwrap();
            let original = p.locator().clone();
            NativeWorkStateV1::activate_with_boundary_for_test(dir.path(), p, fault(at));
            let recovered = sdk.recover_work_registration_terms_request(&original);
            let expected = if matches!(at, B::Terminal | B::Reply) {
                WorkDispositionV1::Replayed
            } else if at == B::PublicationAttempt {
                WorkDispositionV1::Uncertain
            } else if at == B::BeforeIntent {
                WorkDispositionV1::Unaccepted
            } else {
                WorkDispositionV1::Committed
            };
            assert_eq!(
                recovered.disposition, expected,
                "prior={prior}, {at:?}: {recovered:?}"
            );
            if recovered.outcome.is_some() {
                sdk.resolve_current_work_registration_terms(&initial, &original)
                    .unwrap();
            }
        }
    }
}

fn menu_terms(check: &LocalCheckLocatorV1, key: &str) -> WorkRegistrationTermsV1 {
    let mut value = terms(check, key);
    value.scope = menu_scope(work_requirements_definition_from_intrinsic_v1(
        &work_state_support::supplied_intrinsic_definition("1.0.0"),
    ));
    value.approved_content_fingerprint = value.fingerprint_approved_content().unwrap();
    value
}

fn activate_menu(
    sdk: &HandbookSdkV1,
    initial: &LocalInitialSetupLocatorV1,
    check: &LocalCheckLocatorV1,
) -> WorkGrantLocatorV1 {
    let prepared = sdk
        .prepare_work_registration_terms(
            initial,
            check,
            menu_terms(check, "menu"),
            WorkTermsTransitionV1::Initial,
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

fn role_input(grant: &WorkGrantLocatorV1, planning: bool, key: &str) -> WorkRegistrationInputV1 {
    let mut value = input(grant, key, key);
    if planning {
        value.display_level = Some("planning".into());
        value.composition =
            WorkCompositionV1::planning_output("What exact requirement should renewal satisfy?")
                .into();
    } else {
        value.display_level = Some("implementation".into());
        value.composition =
            WorkCompositionV1::requirements_input(work_requirements_definition_from_intrinsic_v1(
                &work_state_support::supplied_intrinsic_definition("1.0.0"),
            ))
            .into();
    }
    value
}

#[test]
fn planning_and_requirements_prefixes_never_borrow_successor_or_transition_authority() {
    for planning in [false, true] {
        for at in [B::Intent, B::Snapshot, B::PublicationAttempt, B::Terminal] {
            for transition in 0..4 {
                let dir = tempdir().unwrap();
                let sdk = HandbookSdkV1::open(dir.path());
                let (initial, check) = setup(&sdk);
                let grant = activate_menu(&sdk, &initial, &check);
                let prepared = sdk
                    .prepare_work_registration(
                        &initial,
                        &grant,
                        role_input(&grant, planning, "original-role"),
                    )
                    .unwrap();
                let original = prepared.locator().clone();
                NativeWorkStateV1::register_with_boundary_for_test(dir.path(), prepared, fault(at));
                match transition {
                    0 => {
                        // Exact overlap still cannot supply an old request's authority.
                        let prepared = sdk
                            .prepare_work_registration_terms(
                                &initial,
                                &check,
                                menu_terms(&check, "overlap-successor"),
                                WorkTermsTransitionV1::ScopeChange(&grant),
                            )
                            .unwrap();
                        assert_eq!(
                            sdk.activate_work_registration_terms(prepared).disposition,
                            WorkDispositionV1::Committed
                        );
                    }
                    1 => revoke(&sdk, &grant),
                    2 => govern(&sdk, &initial),
                    _ => reset(&sdk),
                }
                let recovered = sdk.recover_work_registration_request(&original);
                let expected = if at == B::Terminal {
                    WorkDispositionV1::Replayed
                } else if at == B::PublicationAttempt {
                    WorkDispositionV1::Uncertain
                } else {
                    WorkDispositionV1::NonCommitted
                };
                assert_eq!(
                    recovered.disposition, expected,
                    "planning={planning} {at:?} transition={transition}: {recovered:?}"
                );
                if at == B::Terminal {
                    let receipt = recovered.outcome.unwrap();
                    assert_eq!(
                        receipt.snapshot.role,
                        original.input.composition.role().cloned()
                    );
                    assert_eq!(receipt.snapshot.display_level, original.input.display_level);
                }
            }
        }
    }
}

#[test]
fn pending_scope_administration_suspends_use_and_original_noncommit_controls_release() {
    for at in [B::Intent, B::Decision, B::Snapshot, B::PublicationAttempt] {
        for lost_basis in [false, true] {
            let dir = tempdir().unwrap();
            let sdk = HandbookSdkV1::open(dir.path());
            let (initial, check) = setup(&sdk);
            let grant = activate_menu(&sdk, &initial, &check);
            let prepared = sdk
                .prepare_work_registration_terms(
                    &initial,
                    &check,
                    terms(&check, "narrowing"),
                    WorkTermsTransitionV1::ScopeChange(&grant),
                )
                .unwrap();
            let original = prepared.locator().clone();
            NativeWorkStateV1::activate_with_boundary_for_test(dir.path(), prepared, fault(at));
            assert!(sdk
                .resolve_current_work_registration_terms(&initial, &grant)
                .is_err());
            assert!(sdk
                .prepare_work_registration(
                    &initial,
                    &original,
                    role_input(&original, true, "candidate-use")
                )
                .is_err());
            assert!(sdk
                .prepare_work_registration_terms(
                    &initial,
                    &check,
                    terms(&check, "competing"),
                    WorkTermsTransitionV1::ScopeChange(&grant)
                )
                .is_err());
            if lost_basis {
                govern(&sdk, &initial);
            }
            let execution = NativeWorkStateV1::recover_terms_with_boundary_for_test(
                dir.path(),
                &original,
                fault(B::Snapshot),
            );
            let expected = if at == B::PublicationAttempt {
                WorkDispositionV1::Uncertain
            } else {
                WorkDispositionV1::NonCommitted
            };
            assert_eq!(
                execution.disposition, expected,
                "{at:?} lost={lost_basis}: {execution:?}"
            );
            assert_eq!(
                sdk.resolve_current_work_registration_terms(&initial, &grant)
                    .is_ok(),
                !lost_basis && at != B::PublicationAttempt
            );
            assert_eq!(
                sdk.resolve_work_registration_terms_request(&original)
                    .disposition,
                expected
            );
            if !lost_basis && expected == WorkDispositionV1::NonCommitted {
                let changed = menu_terms(&check, "narrowing");
                match sdk.prepare_work_registration_terms(
                    &initial,
                    &check,
                    changed,
                    WorkTermsTransitionV1::ScopeChange(&grant),
                ) {
                    Err(_) => {}
                    Ok(prepared) => assert_eq!(
                        sdk.activate_work_registration_terms(prepared).disposition,
                        WorkDispositionV1::Refused
                    ),
                }
                assert_eq!(
                    sdk.recover_work_registration_terms_request(&original)
                        .disposition,
                    WorkDispositionV1::NonCommitted
                );
            }
        }
    }
}

fn role_child(repo: &Path, mode: &str, planning: bool, pause: bool) -> std::process::Child {
    Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "native_role_child", "--ignored", "--nocapture"])
        .env("ROLE_REPO", repo)
        .env("ROLE_MODE", mode)
        .env("ROLE_PLANNING", if planning { "yes" } else { "no" })
        .env("ROLE_PAUSE", if pause { "yes" } else { "no" })
        .env("RUST_MIN_STACK", "33554432")
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
}

#[test]
fn real_process_scope_change_and_role_registration_serialize_in_both_orders() {
    for planning in [false, true] {
        for change_first in [false, true] {
            let dir = tempdir().unwrap();
            let sdk = HandbookSdkV1::open(dir.path());
            let (initial, check) = setup(&sdk);
            let grant = activate_menu(&sdk, &initial, &check);
            for (name, bytes) in [
                ("initial.role.json", initial.canonical_json_bytes().unwrap()),
                ("check.role.json", check.canonical_json_bytes().unwrap()),
                ("grant.role.json", grant.canonical_json_bytes().unwrap()),
            ] {
                fs::write(dir.path().join(name), bytes).unwrap();
            }
            let (first_mode, second_mode) = if change_first {
                ("change", "register")
            } else {
                ("register", "change")
            };
            let mut first = role_child(dir.path(), first_mode, planning, true);
            wait_file(&dir.path().join("role.ready"));
            let mut second = role_child(dir.path(), second_mode, planning, false);
            std::thread::sleep(Duration::from_millis(150));
            assert!(
                second.try_wait().unwrap().is_none(),
                "competing process bypassed native held authority"
            );
            fs::write(dir.path().join("role.release"), []).unwrap();
            assert!(first.wait().unwrap().success());
            assert!(second.wait().unwrap().success());
            let result = fs::read_to_string(dir.path().join("register.role.result")).unwrap();
            assert_eq!(result, if change_first { "refused" } else { "committed" });
            assert!(sdk
                .resolve_current_work_registration_terms(&initial, &grant)
                .is_err());
            assert!(
                matches!(sdk.resolve_current_work_item(&grant, "serialized-role").unwrap(),
                WorkCurrentItemV1::Open(_) if !change_first)
                    || matches!(
                        sdk.resolve_current_work_item(&grant, "serialized-role")
                            .unwrap(),
                        WorkCurrentItemV1::Absent
                    ) && change_first
            );
        }
    }
}

#[test]
#[ignore]
fn native_role_child() {
    let repo = std::env::var("ROLE_REPO").unwrap();
    let repo = Path::new(&repo);
    let sdk = HandbookSdkV1::open(repo);
    let initial = LocalInitialSetupLocatorV1::from_json_bytes(
        &fs::read(repo.join("initial.role.json")).unwrap(),
    )
    .unwrap();
    let check =
        LocalCheckLocatorV1::from_json_bytes(&fs::read(repo.join("check.role.json")).unwrap())
            .unwrap();
    let grant =
        WorkGrantLocatorV1::from_json_bytes(&fs::read(repo.join("grant.role.json")).unwrap())
            .unwrap();
    let pause = std::env::var("ROLE_PAUSE").unwrap() == "yes";
    let mut boundary = |at| {
        if pause && at == B::Snapshot {
            fs::write(repo.join("role.ready"), []).unwrap();
            wait_file(&repo.join("role.release"));
        }
        Ok(())
    };
    if std::env::var("ROLE_MODE").unwrap() == "change" {
        let prepared = sdk
            .prepare_work_registration_terms(
                &initial,
                &check,
                terms(&check, "serialized-narrowing"),
                WorkTermsTransitionV1::ScopeChange(&grant),
            )
            .unwrap();
        let execution =
            NativeWorkStateV1::activate_with_boundary_for_test(repo, prepared, &mut boundary);
        assert_eq!(
            execution.disposition,
            WorkDispositionV1::Committed,
            "{execution:?}"
        );
    } else {
        let planning = std::env::var("ROLE_PLANNING").unwrap() == "yes";
        let prepared = sdk.prepare_work_registration(
            &initial,
            &grant,
            role_input(&grant, planning, "serialized-role"),
        );
        let result = match prepared {
            Err(_) => "refused",
            Ok(prepared) => {
                let original = prepared.locator().canonical_json_bytes().unwrap();
                fs::write(repo.join("request.role.json"), original).unwrap();
                let execution = NativeWorkStateV1::register_with_boundary_for_test(
                    repo,
                    prepared,
                    &mut boundary,
                );
                assert_eq!(
                    execution.disposition,
                    WorkDispositionV1::Committed,
                    "{execution:?}"
                );
                "committed"
            }
        };
        fs::write(repo.join("register.role.result"), result).unwrap();
    }
}

#[test]
fn zero_free_slots_close_accepted_item_and_authority_without_reusing_history_slots() {
    for authority_last in [false, true] {
        let dir = tempdir().unwrap();
        let sdk = HandbookSdkV1::open(dir.path());
        let (initial, check) = setup(&sdk);
        let first = activate_menu(&sdk, &initial, &check); // slot 1
        let prepared = sdk
            .prepare_work_registration_terms(
                &initial,
                &check,
                menu_terms(&check, "revision"),
                WorkTermsTransitionV1::ScopeChange(&first),
            )
            .unwrap();
        let grant = prepared.locator().clone(); // slot 2
        assert_eq!(
            sdk.activate_work_registration_terms(prepared).disposition,
            WorkDispositionV1::Committed
        );
        let pending_item = if authority_last {
            None
        } else {
            let prepared = sdk
                .prepare_work_registration(
                    &initial,
                    &grant,
                    role_input(&grant, true, "pending-last-item"),
                )
                .unwrap();
            let original = prepared.locator().clone(); // slot 3
            NativeWorkStateV1::register_with_boundary_for_test(
                dir.path(),
                prepared,
                fault(B::Snapshot),
            );
            Some(original)
        };
        let count = if authority_last { 29 } else { 27 };
        let mut retained = None;
        for index in 0..count {
            let key = format!("capacity-{index}");
            let prepared = sdk
                .prepare_work_registration(
                    &initial,
                    &grant,
                    role_input(&grant, index % 2 == 0, &key),
                )
                .unwrap();
            let original = prepared.locator().clone();
            assert_eq!(
                sdk.register_work_item(prepared).disposition,
                WorkDispositionV1::Committed
            );
            retained = Some(original);
        }
        let prepared = sdk
            .prepare_work_registration_terms(
                &initial,
                &check,
                menu_terms(&check, "capacity-administration"),
                WorkTermsTransitionV1::ScopeChange(&grant),
            )
            .unwrap();
        let administration = prepared.locator().clone(); // slot 31 or 32
        NativeWorkStateV1::activate_with_boundary_for_test(
            dir.path(),
            prepared,
            fault(B::Snapshot),
        );
        let current = if authority_last {
            let recovered = sdk.recover_work_registration_terms_request(&administration);
            assert_eq!(
                recovered.disposition,
                WorkDispositionV1::Committed,
                "{recovered:?}"
            );
            administration.clone()
        } else {
            let recovered = NativeWorkStateV1::recover_terms_with_boundary_for_test(
                dir.path(),
                &administration,
                fault(B::Snapshot),
            );
            assert_eq!(
                recovered.disposition,
                WorkDispositionV1::NonCommitted,
                "{recovered:?}"
            );
            let prepared = sdk
                .prepare_work_registration(&initial, &grant, role_input(&grant, false, "last-slot"))
                .unwrap();
            assert_eq!(
                sdk.register_work_item(prepared).disposition,
                WorkDispositionV1::Committed
            ); // slot 32
            let recovered = sdk.recover_work_registration_request(pending_item.as_ref().unwrap());
            assert_eq!(
                recovered.disposition,
                WorkDispositionV1::Committed,
                "{recovered:?}"
            );
            grant.clone()
        };
        let overflow = sdk
            .prepare_work_registration(
                &initial,
                &current,
                role_input(&current, false, "overflow-role"),
            )
            .unwrap();
        assert_eq!(
            sdk.register_work_item(overflow).disposition,
            WorkDispositionV1::Refused
        );
        let original = retained.unwrap();
        assert_eq!(
            sdk.recover_work_registration_request(&original).disposition,
            WorkDispositionV1::Replayed
        );
        assert_eq!(
            sdk.resolve_work_registration_terms_request(&first)
                .disposition,
            WorkDispositionV1::Replayed
        );
        if !authority_last {
            assert_eq!(
                sdk.resolve_work_registration_terms_request(&administration)
                    .disposition,
                WorkDispositionV1::NonCommitted
            );
        }
    }
}
