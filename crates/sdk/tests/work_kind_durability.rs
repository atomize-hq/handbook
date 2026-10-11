//! Real native writes and real process death for immutable kinds and their items.
#[allow(dead_code)]
mod work_kind_support;
use handbook_engine::work_state::{NativeWorkStateV1, WorkWriteBoundaryV1 as B};
use handbook_sdk::local_check::LocalCheckLocatorV1;
use handbook_sdk::local_setup::LocalInitialSetupLocatorV1;
use handbook_sdk::work_state::*;
use handbook_sdk::HandbookSdkV1;
use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
    time::Duration,
};
use tempfile::tempdir;
use work_kind_support::*;

#[test]
fn native_initial_revocation_before_check_adoption_blocks_kind_and_work_bootstrap() {
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
    let prepared = sdk
        .prepare_local_check_terms(
            &initial,
            supplied_successor_terms(&initial, &original.outcome_fingerprint),
        )
        .unwrap();
    let check = prepared.locator().clone();
    let revoked = sdk
        .revoke_local_operating_authority(
            &initial,
            handbook_engine::local_authority::LocalRevokeRequestV1 {
                request_key: "revoke-initial-before-successor".into(),
                expected_current_fingerprint: original.outcome_fingerprint,
                principal: initial.terms.terms.revocation_principal.clone(),
                source_ref: "human.revoke-initial".into(),
                instruction: "Revoke the exact current initial authority before successor adoption"
                    .into(),
            },
        )
        .unwrap();
    assert_eq!(
        revoked.disposition,
        handbook_sdk::local_setup::LocalAuthorityDispositionV1::Committed
    );
    assert!(sdk.adopt_local_check_terms(prepared).outcome.is_none());
    let content = definition("project.kind.leaf@1.0.0");
    assert!(sdk
        .prepare_work_kind_establishment(
            &initial,
            &check,
            content.clone(),
            approval(&sdk, &initial, &check, &content, "after-initial-revoke")
        )
        .is_err());
    assert!(sdk
        .prepare_work_registration_terms(
            &initial,
            &check,
            terms(&check, "after-initial-revoke"),
            WorkTermsTransitionV1::Initial
        )
        .is_err());
    assert!(sdk
        .resolve_local_initial_setup_request(&initial)
        .unwrap()
        .terms
        .unwrap()
        .outcome
        .is_some());
    assert!(!dir
        .path()
        .join(".handbook-work-state-bootstrap.v1.json")
        .exists());
    assert!(!dir.path().join(".handbook/work-state").exists());
}

#[test]
fn definition_every_publication_prefix_preserves_original_basis_after_governance_check_revocation_and_reset(
) {
    for at in PREFIXES {
        for transition in 0..4 {
            let dir = tempdir().unwrap();
            let sdk = HandbookSdkV1::open(dir.path());
            let (initial, check) = setup(&sdk);
            // Kind-first also proves recovery does not invent a work grant.
            let content = definition("project.kind.leaf@1.0.0");
            let prepared = sdk
                .prepare_work_kind_establishment(
                    &initial,
                    &check,
                    content.clone(),
                    approval(&sdk, &initial, &check, &content, "original-kind"),
                )
                .unwrap();
            let original = prepared.locator().clone();
            let interrupted = NativeWorkStateV1::establish_kind_with_boundary_for_test(
                dir.path(),
                prepared,
                fault(at),
            );
            assert!(interrupted.outcome.is_none(), "{at:?}: {interrupted:?}");
            let committed = matches!(at, B::Terminal | B::Reply);
            let current = sdk.resolve_work_kind(&original.binding).unwrap();
            assert!(
                match current {
                    WorkCurrentKindV1::Committed(receipt) =>
                        committed && receipt.locator == original,
                    WorkCurrentKindV1::Absent => at == B::BeforeIntent,
                    WorkCurrentKindV1::Pending { original_request } =>
                        !committed && original_request == original,
                },
                "{at:?}"
            );
            match transition {
                0 => {}
                1 => govern(&sdk, &initial),
                2 => revoke_check(&sdk, &initial, &check),
                3 => reset(&sdk),
                _ => unreachable!(),
            }
            let recovered = sdk.recover_work_kind_request(&original);
            let expected = if committed {
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
                recovered.disposition, expected,
                "{at:?}, transition {transition}: {recovered:?}"
            );
            if recovered.outcome.is_some() {
                assert!(matches!(
                    sdk.resolve_work_kind(&original.binding).unwrap(),
                    WorkCurrentKindV1::Committed(_)
                ));
            }
            if expected == WorkDispositionV1::Uncertain {
                assert!(sdk
                    .prepare_work_kind_establishment(
                        &initial,
                        &check,
                        content.clone(),
                        approval(&sdk, &initial, &check, &content, "blind-kind-retry")
                    )
                    .is_err());
            }
            if expected == WorkDispositionV1::NonCommitted && transition == 1 {
                let prepared = sdk
                    .prepare_work_kind_establishment(
                        &initial,
                        &check,
                        content.clone(),
                        approval(
                            &sdk,
                            &initial,
                            &check,
                            &content,
                            "positively-released-version",
                        ),
                    )
                    .unwrap();
                assert_eq!(
                    sdk.establish_work_kind(prepared).disposition,
                    WorkDispositionV1::Committed
                );
                assert_eq!(
                    sdk.resolve_work_kind_request(&original).disposition,
                    WorkDispositionV1::NonCommitted
                );
            }
        }
    }
}

#[test]
fn kind_item_every_publication_prefix_never_borrows_successor_or_transition_authority() {
    for at in PREFIXES {
        for transition in 0..5 {
            let dir = tempdir().unwrap();
            let sdk = HandbookSdkV1::open(dir.path());
            let (initial, check) = setup(&sdk);
            let binding = establish_kind(&sdk, &initial, &check, "kind", "project.kind.leaf@1.0.0");
            let grant = activate(&sdk, &initial, &check, "grant-D", None);
            let prepared = sdk
                .prepare_work_registration(
                    &initial,
                    &grant,
                    input(&grant, &binding, "original-item"),
                )
                .unwrap();
            let original = prepared.locator().clone();
            let execution =
                NativeWorkStateV1::register_with_boundary_for_test(dir.path(), prepared, fault(at));
            assert!(execution.outcome.is_none(), "{at:?}: {execution:?}");
            match transition {
                0 => {}
                1 => govern(&sdk, &initial),
                2 => revoke(&sdk, &grant),
                3 => revoke_check(&sdk, &initial, &check),
                4 => reset(&sdk),
                _ => unreachable!(),
            }
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
            let recovered = sdk.recover_work_registration_request(&original);
            assert_eq!(
                recovered.disposition, expected,
                "{at:?}, transition {transition}: {recovered:?}"
            );
            assert!(matches!(
                sdk.resolve_work_kind(&binding).unwrap(),
                WorkCurrentKindV1::Committed(_)
            ));
            if let Some(receipt) = recovered.outcome {
                assert_eq!(receipt.locator, original);
                assert_eq!(receipt.snapshot.admitted_kind.as_ref(), Some(&binding));
                assert!(matches!(
                    sdk.resolve_current_work_item(&grant, "original-item")
                        .unwrap(),
                    WorkCurrentItemV1::Open(_)
                ));
            }
            if expected == WorkDispositionV1::NonCommitted && transition == 1 {
                let current = activate(&sdk, &initial, &check, "grant-G", Some(&grant));
                let prepared = sdk
                    .prepare_work_registration(
                        &initial,
                        &current,
                        input(&current, &binding, "fresh-G-item"),
                    )
                    .unwrap();
                assert_eq!(
                    sdk.register_work_item(prepared).disposition,
                    WorkDispositionV1::Committed
                );
                assert_eq!(
                    sdk.resolve_work_registration_request(&original).disposition,
                    WorkDispositionV1::NonCommitted
                );
            }
        }
    }
}

#[test]
fn pending_definition_is_local_but_pending_registration_administration_suspends_work() {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let first = establish_kind(&sdk, &initial, &check, "kind-1", "project.kind.leaf@1.0.0");
    let grant = activate(&sdk, &initial, &check, "grant", None);
    let content = definition("project.kind.leaf@2.0.0");
    let prepared = sdk
        .prepare_work_kind_establishment(
            &initial,
            &check,
            content.clone(),
            approval(&sdk, &initial, &check, &content, "pending-kind-2"),
        )
        .unwrap();
    let pending = prepared.locator().clone();
    NativeWorkStateV1::establish_kind_with_boundary_for_test(
        dir.path(),
        prepared,
        fault(B::Snapshot),
    );
    assert!(matches!(
        sdk.resolve_work_kind(&pending.binding).unwrap(),
        WorkCurrentKindV1::Pending { .. }
    ));
    assert!(sdk
        .prepare_work_registration(
            &initial,
            &grant,
            input(&grant, &pending.binding, "candidate-use")
        )
        .is_err());
    let prepared = sdk
        .prepare_work_registration(&initial, &grant, input(&grant, &first, "retained-kind-use"))
        .unwrap();
    assert_eq!(
        sdk.register_work_item(prepared).disposition,
        WorkDispositionV1::Committed
    );
    let mut explicit = input(&grant, &first, "explicit-use");
    explicit.composition = WorkCompositionV1::ordinary_leaf().into();
    assert_eq!(
        sdk.register_work_item(
            sdk.prepare_work_registration(&initial, &grant, explicit)
                .unwrap()
        )
        .disposition,
        WorkDispositionV1::Committed
    );
    let prepared = sdk
        .prepare_work_registration_terms(
            &initial,
            &check,
            terms(&check, "pending-administration"),
            WorkTermsTransitionV1::ScopeChange(&grant),
        )
        .unwrap();
    let administration = prepared.locator().clone();
    NativeWorkStateV1::activate_with_boundary_for_test(dir.path(), prepared, fault(B::Snapshot));
    let bound_suspended = sdk
        .prepare_work_registration(&initial, &grant, input(&grant, &first, "suspended-work"))
        .unwrap();
    assert_eq!(
        sdk.register_work_item(bound_suspended).disposition,
        WorkDispositionV1::Refused
    );
    // Definition establishment is independent even while registration head administration is pending.
    let third = establish_kind(&sdk, &initial, &check, "kind-3", "project.kind.leaf@3.0.0");
    assert!(matches!(
        sdk.resolve_work_kind(&third).unwrap(),
        WorkCurrentKindV1::Committed(_)
    ));
    assert_eq!(
        sdk.recover_work_kind_request(&pending).disposition,
        WorkDispositionV1::Committed
    );
    assert_eq!(
        sdk.recover_work_registration_terms_request(&administration)
            .disposition,
        WorkDispositionV1::Committed
    );
}

#[test]
fn positive_noncommit_releases_only_version_and_never_original_request_content() {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let content = definition("project.kind.leaf@1.0.0");
    let prepared = sdk
        .prepare_work_kind_establishment(
            &initial,
            &check,
            content.clone(),
            approval(&sdk, &initial, &check, &content, "original"),
        )
        .unwrap();
    let original = prepared.locator().clone();
    NativeWorkStateV1::establish_kind_with_boundary_for_test(
        dir.path(),
        prepared,
        fault(B::Snapshot),
    );
    let closed = NativeWorkStateV1::recover_kind_with_boundary_for_test(
        dir.path(),
        &original,
        fault(B::Snapshot),
    );
    assert_eq!(
        closed.disposition,
        WorkDispositionV1::NonCommitted,
        "{closed:?}"
    );
    assert!(matches!(
        sdk.resolve_work_kind(&original.binding).unwrap(),
        WorkCurrentKindV1::Absent
    ));
    let mut changed = content.clone();
    changed.template.label_max_bytes = 64;
    match sdk.prepare_work_kind_establishment(
        &initial,
        &check,
        changed.clone(),
        approval(&sdk, &initial, &check, &changed, "original"),
    ) {
        Err(_) => {}
        Ok(prepared) => assert_eq!(
            sdk.establish_work_kind(prepared).disposition,
            WorkDispositionV1::Refused
        ),
    }
    let prepared = sdk
        .prepare_work_kind_establishment(
            &initial,
            &check,
            changed.clone(),
            approval(&sdk, &initial, &check, &changed, "fresh-approval"),
        )
        .unwrap();
    assert_eq!(
        sdk.establish_work_kind(prepared).disposition,
        WorkDispositionV1::Committed
    );
    assert_eq!(
        sdk.recover_work_kind_request(&original).disposition,
        WorkDispositionV1::NonCommitted
    );
}

#[test]
fn combined_saturation_closes_accepted_definition_and_kind_item_without_fresh_slots() {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let binding = establish_kind(&sdk, &initial, &check, "kind", "project.kind.leaf@1.0.0"); // slot 1
    let grant = activate(&sdk, &initial, &check, "grant", None); // slot 2
    let content = definition("project.kind.leaf@2.0.0");
    let prepared = sdk
        .prepare_work_kind_establishment(
            &initial,
            &check,
            content.clone(),
            approval(&sdk, &initial, &check, &content, "accepted-kind"),
        )
        .unwrap();
    let pending_kind = prepared.locator().clone(); // slot 3
    NativeWorkStateV1::establish_kind_with_boundary_for_test(
        dir.path(),
        prepared,
        fault(B::Snapshot),
    );
    let prepared = sdk
        .prepare_work_registration(&initial, &grant, input(&grant, &binding, "accepted-item"))
        .unwrap();
    let pending_item = prepared.locator().clone(); // slot 4
    NativeWorkStateV1::register_with_boundary_for_test(dir.path(), prepared, fault(B::Snapshot));
    let mut retained = None;
    for index in 0..28 {
        let prepared = sdk
            .prepare_work_registration(
                &initial,
                &grant,
                input(&grant, &binding, &format!("capacity-{index}")),
            )
            .unwrap();
        retained = Some(prepared.locator().clone());
        assert_eq!(
            sdk.register_work_item(prepared).disposition,
            WorkDispositionV1::Committed
        );
    }
    assert_eq!(
        sdk.recover_work_kind_request(&pending_kind).disposition,
        WorkDispositionV1::Committed
    );
    assert_eq!(
        sdk.recover_work_registration_request(&pending_item)
            .disposition,
        WorkDispositionV1::Committed
    );
    assert_eq!(
        sdk.recover_work_kind_request(&pending_kind).disposition,
        WorkDispositionV1::Replayed
    );
    assert_eq!(
        sdk.recover_work_registration_request(retained.as_ref().unwrap())
            .disposition,
        WorkDispositionV1::Replayed
    );
    let fresh_item = sdk
        .prepare_work_registration(&initial, &grant, input(&grant, &binding, "fresh-overflow"))
        .unwrap();
    assert_eq!(
        sdk.register_work_item(fresh_item).disposition,
        WorkDispositionV1::Refused
    );
    let fresh = definition("project.kind.leaf@3.0.0");
    match sdk.prepare_work_kind_establishment(
        &initial,
        &check,
        fresh.clone(),
        approval(&sdk, &initial, &check, &fresh, "fresh-kind-overflow"),
    ) {
        Err(_) => {}
        Ok(prepared) => assert_eq!(
            sdk.establish_work_kind(prepared).disposition,
            WorkDispositionV1::Refused
        ),
    }
    let revocation = sdk.prepare_work_registration_revocation(
        &grant,
        "fresh-revoke-overflow",
        &grant.terms.principal,
        "human.revoke",
        "Fresh revocation at capacity",
    );
    match revocation {
        Err(_) => {}
        Ok(request) => assert_eq!(
            sdk.revoke_work_registration_terms(&request).disposition,
            WorkDispositionV1::Refused
        ),
    }
    match sdk.prepare_work_registration_terms(
        &initial,
        &check,
        terms(&check, "fresh-authority-overflow"),
        WorkTermsTransitionV1::ScopeChange(&grant),
    ) {
        Err(_) => {}
        Ok(prepared) => assert_eq!(
            sdk.activate_work_registration_terms(prepared).disposition,
            WorkDispositionV1::Refused
        ),
    }
    assert!(matches!(
        sdk.resolve_work_kind(&binding).unwrap(),
        WorkCurrentKindV1::Committed(_)
    ));
    assert!(matches!(
        sdk.resolve_current_work_item(&grant, "accepted-item")
            .unwrap(),
        WorkCurrentItemV1::Open(_)
    ));
    assert_eq!(
        fs::read_dir(dir.path().join(".handbook/work-state/v1"))
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                name.len() == 12 && name.starts_with("slot-") && name.ends_with(".json")
            })
            .count(),
        32
    );
}

fn child(repo: &Path, mode: &str, pause: Option<B>) -> std::process::Child {
    Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "native_kind_child", "--ignored", "--nocapture"])
        .env("KIND_REPO", repo)
        .env("KIND_MODE", mode)
        .env(
            "KIND_PAUSE",
            pause.map_or(String::new(), |at| format!("{at:?}")),
        )
        .env("RUST_MIN_STACK", "33554432")
        .stdout(Stdio::null())
        .spawn()
        .unwrap()
}

#[test]
fn real_process_death_preserves_definition_and_kind_item_publication_truth() {
    for kind in [true, false] {
        for (at, expected) in [
            (B::BootstrapIntent, WorkDispositionV1::Uncertain),
            (B::BeforeIntent, WorkDispositionV1::Unaccepted),
            (B::Reservation, WorkDispositionV1::Uncertain),
            (B::AcceptanceCandidate, WorkDispositionV1::Uncertain),
            (B::StagedIntent, WorkDispositionV1::Committed),
            (B::AcceptanceReady, WorkDispositionV1::Committed),
            (B::Intent, WorkDispositionV1::Committed),
            (B::Decision, WorkDispositionV1::Committed),
            (B::Snapshot, WorkDispositionV1::Committed),
            (B::PublicationAttempt, WorkDispositionV1::Uncertain),
            (B::Terminal, WorkDispositionV1::Replayed),
            (B::Reply, WorkDispositionV1::Replayed),
        ] {
            if !kind && at == B::BootstrapIntent {
                continue;
            }
            let dir = tempdir().unwrap();
            let sdk = HandbookSdkV1::open(dir.path());
            let (initial, check) = setup(&sdk);
            save_setup(dir.path(), &initial, &check);
            if !kind {
                let binding =
                    establish_kind(&sdk, &initial, &check, "kind", "project.kind.leaf@1.0.0");
                let grant = activate(&sdk, &initial, &check, "grant", None);
                fs::write(
                    dir.path().join("binding.kind-test.json"),
                    serde_json::to_vec(&binding).unwrap(),
                )
                .unwrap();
                fs::write(
                    dir.path().join("grant.kind-test.json"),
                    grant.canonical_json_bytes().unwrap(),
                )
                .unwrap();
            }
            let mut writer = child(dir.path(), if kind { "kind-a" } else { "item" }, Some(at));
            wait_file(&dir.path().join("ready.kind-test"));
            writer.kill().unwrap();
            writer.wait().unwrap();
            if kind {
                let original = WorkKindLocatorV1::from_json_bytes(
                    &fs::read(dir.path().join("kind.original.json")).unwrap(),
                )
                .unwrap();
                let recovered = sdk.recover_work_kind_request(&original);
                assert_eq!(
                    recovered.disposition, expected,
                    "kind {at:?}: {recovered:?}"
                );
                assert!(match sdk.resolve_work_kind(&original.binding) {
                    Err(_) | Ok(WorkCurrentKindV1::Pending { .. }) =>
                        expected == WorkDispositionV1::Uncertain,
                    Ok(WorkCurrentKindV1::Committed(_)) => matches!(
                        expected,
                        WorkDispositionV1::Committed | WorkDispositionV1::Replayed
                    ),
                    Ok(WorkCurrentKindV1::Absent) => expected == WorkDispositionV1::Unaccepted,
                });
            } else {
                let original = WorkRequestLocatorV1::from_json_bytes(
                    &fs::read(dir.path().join("item.original.json")).unwrap(),
                )
                .unwrap();
                let recovered = sdk.recover_work_registration_request(&original);
                assert_eq!(
                    recovered.disposition, expected,
                    "item {at:?}: {recovered:?}"
                );
                assert!(
                    match sdk.resolve_current_work_item(&original.grant, "child-item") {
                        Err(_) | Ok(WorkCurrentItemV1::Pending { .. }) =>
                            expected == WorkDispositionV1::Uncertain,
                        Ok(WorkCurrentItemV1::Open(_)) => matches!(
                            expected,
                            WorkDispositionV1::Committed | WorkDispositionV1::Replayed
                        ),
                        Ok(WorkCurrentItemV1::Absent) => expected == WorkDispositionV1::Unaccepted,
                    }
                );
            }
        }
    }
}

#[test]
fn real_process_same_version_and_grant_kind_bootstrap_races_have_one_owner_and_exact_version() {
    for first_kind in [true, false] {
        for same_version in [true, false] {
            let dir = tempdir().unwrap();
            let sdk = HandbookSdkV1::open(dir.path());
            let (initial, check) = setup(&sdk);
            save_setup(dir.path(), &initial, &check);
            let first_mode = if same_version && !first_kind {
                "kind-b"
            } else if first_kind || same_version {
                "kind-a"
            } else {
                "grant"
            };
            let second_mode = if same_version {
                if first_kind {
                    "kind-b"
                } else {
                    "kind-a"
                }
            } else if first_kind {
                "grant"
            } else {
                "kind-a"
            };
            let mut first = child(dir.path(), first_mode, Some(B::Snapshot));
            wait_file(&dir.path().join("ready.kind-test"));
            let mut second = child(dir.path(), second_mode, None);
            std::thread::sleep(Duration::from_millis(150));
            assert!(
                second.try_wait().unwrap().is_none(),
                "competing bootstrap/version bypassed held owner guard"
            );
            fs::write(dir.path().join("release.kind-test"), []).unwrap();
            assert!(first.wait().unwrap().success());
            assert!(second.wait().unwrap().success());
            assert_eq!(
                fs::read_to_string(dir.path().join(format!("{first_mode}.result"))).unwrap(),
                "committed"
            );
            assert_eq!(
                fs::read_to_string(dir.path().join(format!("{second_mode}.result"))).unwrap(),
                if same_version { "refused" } else { "committed" }
            );
            let kind_mode = if same_version { first_mode } else { "kind-a" };
            let original = WorkKindLocatorV1::from_json_bytes(
                &fs::read(dir.path().join(format!("{kind_mode}.original.json"))).unwrap(),
            )
            .unwrap();
            assert!(matches!(
                sdk.resolve_work_kind(&original.binding).unwrap(),
                WorkCurrentKindV1::Committed(_)
            ));
            if !same_version {
                let grant = WorkGrantLocatorV1::from_json_bytes(
                    &fs::read(dir.path().join("grant.kind-test.json")).unwrap(),
                )
                .unwrap();
                sdk.resolve_current_work_registration_terms(&initial, &grant)
                    .unwrap();
                let prepared = sdk
                    .prepare_work_registration(
                        &initial,
                        &grant,
                        input(&grant, &original.binding, "after-bootstrap"),
                    )
                    .unwrap();
                assert_eq!(
                    sdk.register_work_item(prepared).disposition,
                    WorkDispositionV1::Committed
                );
            }
        }
    }
}

#[test]
fn real_process_pending_new_version_does_not_suspend_committed_kind_read_or_item_use() {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let binding = establish_kind(&sdk, &initial, &check, "kind", "project.kind.leaf@1.0.0");
    let grant = activate(&sdk, &initial, &check, "grant", None);
    save_setup(dir.path(), &initial, &check);
    fs::write(
        dir.path().join("binding.kind-test.json"),
        serde_json::to_vec(&binding).unwrap(),
    )
    .unwrap();
    fs::write(
        dir.path().join("grant.kind-test.json"),
        grant.canonical_json_bytes().unwrap(),
    )
    .unwrap();
    let mut writer = child(dir.path(), "kind-c", Some(B::Snapshot));
    wait_file(&dir.path().join("ready.kind-test"));
    writer.kill().unwrap();
    writer.wait().unwrap();
    let original = WorkKindLocatorV1::from_json_bytes(
        &fs::read(dir.path().join("kind-c.original.json")).unwrap(),
    )
    .unwrap();
    assert!(matches!(
        sdk.resolve_work_kind(&original.binding).unwrap(),
        WorkCurrentKindV1::Pending { .. }
    ));
    assert!(child(dir.path(), "reader", None).wait().unwrap().success());
    assert!(child(dir.path(), "item", None).wait().unwrap().success());
    assert_eq!(
        fs::read_to_string(dir.path().join("item.result")).unwrap(),
        "committed"
    );
    assert_eq!(
        sdk.resolve_work_kind_request(&original).disposition,
        WorkDispositionV1::Pending
    );
    assert_eq!(
        sdk.recover_work_kind_request(&original).disposition,
        WorkDispositionV1::Committed
    );
}

#[test]
fn real_process_kind_item_serializes_scope_change_revocation_and_governance_in_both_orders() {
    for transition in ["change", "revoke", "govern"] {
        for transition_first in [true, false] {
            let dir = tempdir().unwrap();
            let sdk = HandbookSdkV1::open(dir.path());
            let (initial, check) = setup(&sdk);
            let binding = establish_kind(&sdk, &initial, &check, "kind", "project.kind.leaf@1.0.0");
            let grant = activate(&sdk, &initial, &check, "grant", None);
            save_setup(dir.path(), &initial, &check);
            fs::write(
                dir.path().join("binding.kind-test.json"),
                serde_json::to_vec(&binding).unwrap(),
            )
            .unwrap();
            fs::write(
                dir.path().join("grant.kind-test.json"),
                grant.canonical_json_bytes().unwrap(),
            )
            .unwrap();
            if transition_first && transition == "govern" {
                assert!(child(dir.path(), transition, None)
                    .wait()
                    .unwrap()
                    .success());
                assert!(child(dir.path(), "item", None).wait().unwrap().success());
            } else {
                let (first_mode, second_mode) = if transition_first {
                    (transition, "item")
                } else {
                    ("item", transition)
                };
                let mut first = child(dir.path(), first_mode, Some(B::Snapshot));
                wait_file(&dir.path().join("ready.kind-test"));
                let mut second = child(dir.path(), second_mode, None);
                std::thread::sleep(Duration::from_millis(150));
                assert!(
                    second.try_wait().unwrap().is_none(),
                    "competing authority operation bypassed native held guards"
                );
                fs::write(dir.path().join("release.kind-test"), []).unwrap();
                assert!(first.wait().unwrap().success());
                assert!(second.wait().unwrap().success());
            }
            assert_eq!(
                fs::read_to_string(dir.path().join("item.result")).unwrap(),
                if transition_first {
                    "refused"
                } else {
                    "committed"
                }
            );
            assert!(matches!(
                sdk.resolve_work_kind(&binding).unwrap(),
                WorkCurrentKindV1::Committed(_)
            ));
            assert!(sdk
                .resolve_current_work_registration_terms(&initial, &grant)
                .is_err());
        }
    }
}

#[test]
#[ignore]
fn native_kind_child() {
    let repo_value = std::env::var("KIND_REPO").unwrap();
    let repo = Path::new(&repo_value);
    let sdk = HandbookSdkV1::open(repo);
    let initial = LocalInitialSetupLocatorV1::from_json_bytes(
        &fs::read(repo.join("initial.kind-test.json")).unwrap(),
    )
    .unwrap();
    let check =
        LocalCheckLocatorV1::from_json_bytes(&fs::read(repo.join("check.kind-test.json")).unwrap())
            .unwrap();
    let mode = std::env::var("KIND_MODE").unwrap();
    let pause_at = std::env::var("KIND_PAUSE").unwrap();
    let mut boundary = |at: B| {
        if format!("{at:?}") == pause_at {
            fs::write(repo.join("ready.kind-test"), []).unwrap();
            wait_file(&repo.join("release.kind-test"));
        }
        Ok(())
    };
    let result = if mode.starts_with("kind-") {
        let mut content = definition(if mode == "kind-c" {
            "project.kind.leaf@2.0.0"
        } else {
            "project.kind.leaf@1.0.0"
        });
        if mode == "kind-b" {
            content.template.label_max_bytes = 64;
        }
        match sdk.prepare_work_kind_establishment(
            &initial,
            &check,
            content.clone(),
            approval(&sdk, &initial, &check, &content, &mode),
        ) {
            Err(_) => "refused",
            Ok(prepared) => {
                fs::write(
                    repo.join(format!("{mode}.original.json")),
                    prepared.locator().canonical_json_bytes().unwrap(),
                )
                .unwrap();
                if mode == "kind-a" {
                    fs::write(
                        repo.join("kind.original.json"),
                        prepared.locator().canonical_json_bytes().unwrap(),
                    )
                    .unwrap();
                }
                let execution = NativeWorkStateV1::establish_kind_with_boundary_for_test(
                    repo,
                    prepared,
                    &mut boundary,
                );
                assert!(
                    matches!(
                        execution.disposition,
                        WorkDispositionV1::Committed | WorkDispositionV1::Refused
                    ),
                    "{execution:?}"
                );
                if execution.disposition == WorkDispositionV1::Committed {
                    "committed"
                } else {
                    "refused"
                }
            }
        }
    } else if mode == "grant" {
        let prepared = sdk
            .prepare_work_registration_terms(
                &initial,
                &check,
                terms(&check, "grant"),
                WorkTermsTransitionV1::Initial,
            )
            .unwrap();
        let original = prepared.locator().clone();
        let execution =
            NativeWorkStateV1::activate_with_boundary_for_test(repo, prepared, &mut boundary);
        assert_eq!(
            execution.disposition,
            WorkDispositionV1::Committed,
            "{execution:?}"
        );
        fs::write(
            repo.join("grant.kind-test.json"),
            original.canonical_json_bytes().unwrap(),
        )
        .unwrap();
        "committed"
    } else if mode == "govern" {
        govern(&sdk, &initial);
        "committed"
    } else {
        let grant = WorkGrantLocatorV1::from_json_bytes(
            &fs::read(repo.join("grant.kind-test.json")).unwrap(),
        )
        .unwrap();
        match mode.as_str() {
            "reader" => {
                let binding: WorkKindBindingV1 =
                    serde_json::from_slice(&fs::read(repo.join("binding.kind-test.json")).unwrap())
                        .unwrap();
                assert!(matches!(
                    sdk.resolve_work_kind(&binding).unwrap(),
                    WorkCurrentKindV1::Committed(_)
                ));
                "committed"
            }
            "item" => {
                let binding: WorkKindBindingV1 =
                    serde_json::from_slice(&fs::read(repo.join("binding.kind-test.json")).unwrap())
                        .unwrap();
                let prepared = sdk
                    .prepare_work_registration(
                        &initial,
                        &grant,
                        input(&grant, &binding, "child-item"),
                    )
                    .unwrap();
                fs::write(
                    repo.join("item.original.json"),
                    prepared.locator().canonical_json_bytes().unwrap(),
                )
                .unwrap();
                let execution = NativeWorkStateV1::register_with_boundary_for_test(
                    repo,
                    prepared,
                    &mut boundary,
                );
                match execution.disposition {
                    WorkDispositionV1::Committed => "committed",
                    WorkDispositionV1::Refused => "refused",
                    _ => panic!("{execution:?}"),
                }
            }
            "change" => {
                let mut narrowed = terms(&check, "narrowed");
                narrowed.scope = WorkRegistrationScopeV1::explicit(vec![WorkCompositionClauseV1 {
                    descriptor: WorkDescriptorV1::ordinary_leaf(),
                    label_max_bytes: 1,
                    work_question_max_bytes: 256,
                    allow_omitted_display_level: true,
                    display_levels: vec![],
                    role_permission: WorkRolePermissionV1::Ordinary {},
                }]);
                narrowed.approved_content_fingerprint =
                    narrowed.fingerprint_approved_content().unwrap();
                let prepared = sdk
                    .prepare_work_registration_terms(
                        &initial,
                        &check,
                        narrowed,
                        WorkTermsTransitionV1::ScopeChange(&grant),
                    )
                    .unwrap();
                let execution = NativeWorkStateV1::activate_with_boundary_for_test(
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
            "revoke" => {
                let request = sdk
                    .prepare_work_registration_revocation(
                        &grant,
                        "revoke",
                        &grant.terms.principal,
                        "human.revoke",
                        "Revoke work grant",
                    )
                    .unwrap();
                let execution =
                    NativeWorkStateV1::revoke_with_boundary_for_test(repo, &request, &mut boundary);
                assert_eq!(
                    execution.disposition,
                    WorkDispositionV1::Committed,
                    "{execution:?}"
                );
                "committed"
            }
            _ => panic!("unknown child mode {mode}"),
        }
    };
    fs::write(repo.join(format!("{mode}.result")), result).unwrap();
}
