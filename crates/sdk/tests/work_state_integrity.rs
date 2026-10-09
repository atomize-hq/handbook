//! Native retained-owner loss and proof closure through live publication.
mod work_state_support;
use handbook_engine::work_state::{
    NativeWorkStateV1, WorkWriteBoundaryV1, WORK_STATE_FULL_OWNER_REACH_V1,
};
use handbook_sdk::work_state::*;
use handbook_sdk::HandbookSdkV1;
use std::fs;
use tempfile::tempdir;

fn prepared_activation() -> (
    tempfile::TempDir,
    HandbookSdkV1,
    handbook_sdk::local_setup::LocalInitialSetupLocatorV1,
    PreparedWorkGrantV1,
) {
    let d = tempdir().unwrap();
    let s = HandbookSdkV1::open(d.path());
    let i = work_state_support::establish_initial(&s);
    let original = s
        .resolve_local_initial_setup_request(&i)
        .unwrap()
        .terms
        .unwrap()
        .outcome
        .unwrap();
    let p = s
        .prepare_local_check_terms(
            &i,
            work_state_support::supplied_successor_terms(&i, &original.outcome_fingerprint),
        )
        .unwrap();
    let c = p.locator().clone();
    s.adopt_local_check_terms(p);
    let mut terms = WorkRegistrationTermsV1 {
        request_key: "work".into(),
        rule_ref: WORK_REGISTRATION_RULE_V1.into(),
        mode: "local_advisory".into(),
        principal: c.terms.principal.clone(),
        agent: c.terms.agent.clone(),
        instruction: "Register explicit leaf".into(),
        delegation: "bounded registration".into(),
        approval_source_ref: "human.work".into(),
        human_approval_asserted: true,
        approved_content_fingerprint: String::new(),
        effect: "register_open_leaf".into(),
        composition: WorkCompositionV1::ordinary_leaf(),
        owner_read_reach: WORK_STATE_FULL_OWNER_REACH_V1.into(),
        owner_write_reach: WORK_STATE_FULL_OWNER_REACH_V1.into(),
        reserved_decisions: vec![
            "composition_change".into(),
            "terms_change".into(),
            "close".into(),
        ],
        revocation_principal: c.terms.principal.clone(),
        retention: "retain_original_history".into(),
    };
    terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
    let p = s
        .prepare_work_registration_terms(&i, &c, terms, None)
        .unwrap();
    (d, s, i, p)
}
fn fixture() -> (
    tempfile::TempDir,
    HandbookSdkV1,
    handbook_sdk::local_setup::LocalInitialSetupLocatorV1,
    WorkGrantLocatorV1,
) {
    let (d, s, i, p) = prepared_activation();
    let g = p.locator().clone();
    assert!(s.activate_work_registration_terms(p).outcome.is_some());
    (d, s, i, g)
}
fn input(g: &WorkGrantLocatorV1) -> WorkRegistrationInputV1 {
    WorkRegistrationInputV1 {
        request_key: "request".into(),
        item_key: "item".into(),
        label: "Leaf".into(),
        question: "Question".into(),
        actor: g.terms.agent.clone(),
        expected_state: "absent".into(),
        composition: WorkCompositionV1::ordinary_leaf(),
    }
}
#[test]
fn positive_owner_establishment_cannot_rebootstrap_lost_anchor_manifest_or_lock() {
    for file in [
        ".handbook-work-state.v1.json",
        ".handbook/work-state/v1/owner.json",
        ".handbook/work-state/work.lock",
        ".handbook-work-state-bootstrap.v1.json",
    ] {
        let (d, s, i, g) = fixture();
        let old_project = g
            .check
            .predecessor
            .charter
            .binding
            .initialization_claim
            .proposed_repository_identity()
            .to_owned();
        let p = s.prepare_work_registration(&i, &g, input(&g)).unwrap();
        let l = p.locator().clone();
        assert!(s.register_work_item(p).outcome.is_some());
        fs::remove_file(d.path().join(file)).unwrap();
        assert!(s.resolve_current_work_item(&g, "item").is_err());
        assert!(s.recover_work_registration_request(&l).outcome.is_none());
        assert_eq!(
            s.recover_work_registration_terms_request(&g).disposition,
            WorkDispositionV1::Uncertain
        );
        assert!(
            !d.path().join(file).exists(),
            "lost owner dependency recreated: {file}"
        );
        assert_eq!(
            old_project,
            g.check
                .predecessor
                .charter
                .binding
                .initialization_claim
                .proposed_repository_identity()
        );
    }
}
#[test]
fn item_history_requires_actual_original_work_activation_terminal_and_strict_record_fields() {
    for mode in ["missing-grant", "future-field"] {
        let (d, s, i, g) = fixture();
        let p = s.prepare_work_registration(&i, &g, input(&g)).unwrap();
        let l = p.locator().clone();
        assert!(s.register_work_item(p).outcome.is_some());
        if mode == "missing-grant" {
            fs::remove_file(
                d.path()
                    .join(".handbook/work-state/v1/grant-work/terminal.json"),
            )
            .unwrap();
        } else {
            let path = d
                .path()
                .join(".handbook/work-state/v1/item-request/snapshot.json");
            let mut v: serde_json::Value =
                serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
            v["future_role"] = serde_json::json!("requirements_input");
            fs::write(path, serde_json::to_vec(&v).unwrap()).unwrap();
        }
        assert!(s.resolve_current_work_item(&g, "item").is_err());
        assert_eq!(
            s.recover_work_registration_request(&l).disposition,
            WorkDispositionV1::Uncertain
        );
    }
}
#[test]
fn native_lock_and_activation_dependency_substitution_at_snapshot_cannot_publish() {
    for file in [
        ".handbook/work-state/work.lock",
        ".handbook/work-state/v1/grant-work/terminal.json",
    ] {
        let (d, s, i, g) = fixture();
        let p = s.prepare_work_registration(&i, &g, input(&g)).unwrap();
        let l = p.locator().clone();
        let mut changed = false;
        let e = NativeWorkStateV1::register_with_boundary_for_test(d.path(), p, |at| {
            if at == WorkWriteBoundaryV1::Snapshot {
                let path = d.path().join(file);
                if fs::rename(&path, path.with_extension("substituted")).is_ok() {
                    fs::write(path, []).unwrap();
                    changed = true;
                }
            }
            Ok(())
        });
        if changed {
            assert!(
                e.outcome.is_none(),
                "substituted proof allowed terminal: {e:?}"
            );
            assert!(!d
                .path()
                .join(".handbook/work-state/v1/item-request/terminal.json")
                .exists());
            assert!(s.recover_work_registration_request(&l).outcome.is_none());
        } else {
            assert!(
                e.outcome.is_some(),
                "native pin denied mutation but publication failed: {e:?}"
            );
        }
    }
}

fn interruption() -> LocalAuthorityRefusalV1 {
    LocalAuthorityRefusalV1 {
        code: "test_interruption".into(),
        detail: "Stop after actual native durable write".into(),
        unresolved_request: None,
    }
}

fn displace_transaction(root: &std::path::Path, relative: &str) -> std::path::PathBuf {
    let retained_root = fs::canonicalize(root).unwrap();
    let source = retained_root.join(relative);
    let destination = retained_root.join("lost-original-work-transaction");
    assert!(fs::canonicalize(&source)
        .unwrap()
        .starts_with(&retained_root));
    assert!(destination.starts_with(&retained_root));
    assert!(fs::symlink_metadata(&source).unwrap().is_dir());
    assert!(!destination.exists());
    fs::rename(&source, &destination).unwrap();
    source
}

#[test]
fn lost_whole_item_transaction_is_unverifiable_never_absent_or_replaceable() {
    for committed in [true, false] {
        let (d, s, i, g) = fixture();
        let prepared = s.prepare_work_registration(&i, &g, input(&g)).unwrap();
        let original = prepared.locator().clone();
        let execution = if committed {
            s.register_work_item(prepared)
        } else {
            NativeWorkStateV1::register_with_boundary_for_test(d.path(), prepared, |at| {
                if at == WorkWriteBoundaryV1::Snapshot {
                    Err(interruption())
                } else {
                    Ok(())
                }
            })
        };
        assert_eq!(
            execution.disposition,
            if committed {
                WorkDispositionV1::Committed
            } else {
                WorkDispositionV1::Uncertain
            }
        );
        let lost = displace_transaction(d.path(), ".handbook/work-state/v1/item-request");
        assert!(
            s.resolve_current_work_item(&g, "item").is_err(),
            "lost committed={committed} returned item absence"
        );
        let recovery = s.recover_work_registration_request(&original);
        assert_eq!(
            recovery.disposition,
            WorkDispositionV1::Uncertain,
            "{recovery:?}"
        );
        assert!(recovery.outcome.is_none());
        let mut replacement = input(&g);
        replacement.request_key = "replacement-request".into();
        assert!(s
            .prepare_work_registration(&i, &g, replacement.clone())
            .is_err());
        replacement.item_key = "unrelated-new-item".into();
        assert!(s.prepare_work_registration(&i, &g, replacement).is_err());
        assert!(!lost.exists(), "lost original transaction was recreated");
        assert!(!d
            .path()
            .join(".handbook/work-state/v1/item-replacement-request")
            .exists());
    }
}

#[test]
fn lost_whole_activation_transaction_never_restores_grant_or_owner_absence() {
    for committed in [true, false] {
        let (d, s, i, prepared) = prepared_activation();
        let original = prepared.locator().clone();
        let execution = if committed {
            s.activate_work_registration_terms(prepared)
        } else {
            NativeWorkStateV1::activate_with_boundary_for_test(d.path(), prepared, |at| {
                if at == WorkWriteBoundaryV1::Snapshot {
                    Err(interruption())
                } else {
                    Ok(())
                }
            })
        };
        assert_eq!(
            execution.disposition,
            if committed {
                WorkDispositionV1::Committed
            } else {
                WorkDispositionV1::Uncertain
            }
        );
        let lost = displace_transaction(d.path(), ".handbook/work-state/v1/grant-work");
        assert!(s
            .resolve_current_work_item(&original, "never-registered")
            .is_err());
        assert!(s
            .resolve_current_work_registration_terms(&i, &original)
            .is_err());
        let recovery = s.recover_work_registration_terms_request(&original);
        assert_eq!(
            recovery.disposition,
            WorkDispositionV1::Uncertain,
            "{recovery:?}"
        );
        assert!(recovery.outcome.is_none());
        assert!(s
            .prepare_work_registration(&i, &original, input(&original))
            .is_err());
        let mut replacement = original.terms.clone();
        replacement.request_key = "replacement-grant".into();
        replacement.approved_content_fingerprint =
            replacement.fingerprint_approved_content().unwrap();
        assert!(s
            .prepare_work_registration_terms(&i, &original.check, replacement, None)
            .is_err());
        assert!(!lost.exists(), "lost activation was reconstructed");
        assert!(!d
            .path()
            .join(".handbook/work-state/v1/grant-replacement-grant")
            .exists());
    }
}

#[test]
fn missing_or_erased_expected_slot_cannot_hide_committed_history() {
    for erased in [false, true] {
        let (d, s, i, g) = fixture();
        let prepared = s.prepare_work_registration(&i, &g, input(&g)).unwrap();
        let original = prepared.locator().clone();
        assert_eq!(
            s.register_work_item(prepared).disposition,
            WorkDispositionV1::Committed
        );
        let slot = d.path().join(".handbook/work-state/v1/slot-00.json");
        if erased {
            let mut value: serde_json::Value =
                serde_json::from_slice(&fs::read(&slot).unwrap()).unwrap();
            value["entry"] = serde_json::Value::Null;
            fs::write(&slot, serde_json::to_vec(&value).unwrap()).unwrap();
        } else {
            fs::remove_file(&slot).unwrap();
        }
        assert!(s.resolve_current_work_item(&g, "item").is_err());
        assert_eq!(
            s.recover_work_registration_request(&original).disposition,
            WorkDispositionV1::Uncertain
        );
        assert_eq!(
            s.recover_work_registration_terms_request(&g).disposition,
            WorkDispositionV1::Uncertain
        );
        let mut replacement = input(&g);
        replacement.request_key = "new-key".into();
        assert!(s.prepare_work_registration(&i, &g, replacement).is_err());
    }
}

#[test]
fn actual_early_bootstrap_interruption_stays_uncertain_without_identity_or_lock_repair() {
    let (d, s, i, prepared) = prepared_activation();
    let original = prepared.locator().clone();
    let project_before = original
        .check
        .predecessor
        .charter
        .binding
        .initialization_claim
        .proposed_repository_identity()
        .to_owned();
    let execution = NativeWorkStateV1::activate_with_boundary_for_test(d.path(), prepared, |at| {
        if at == WorkWriteBoundaryV1::BootstrapIntent {
            Err(interruption())
        } else {
            Ok(())
        }
    });
    assert_eq!(
        execution.disposition,
        WorkDispositionV1::Uncertain,
        "{execution:?}"
    );
    assert!(
        d.path()
            .join(".handbook-work-state-bootstrap.v1.json")
            .exists(),
        "actual bootstrap intent was not retained"
    );
    let bootstrap_before =
        fs::read(d.path().join(".handbook-work-state-bootstrap.v1.json")).unwrap();
    assert!(!d.path().join(".handbook/work-state/work.lock").exists());
    assert!(!d.path().join(".handbook/work-state/v1/owner.json").exists());
    let recovery = s.recover_work_registration_terms_request(&original);
    assert_eq!(
        recovery.disposition,
        WorkDispositionV1::Uncertain,
        "{recovery:?}"
    );
    assert!(recovery.outcome.is_none());
    let mut replacement = original.terms.clone();
    replacement.request_key = "replacement-bootstrap".into();
    replacement.approved_content_fingerprint = replacement.fingerprint_approved_content().unwrap();
    if let Ok(prepared) = s.prepare_work_registration_terms(&i, &original.check, replacement, None)
    {
        let attempted = s.activate_work_registration_terms(prepared);
        assert!(
            matches!(
                attempted.disposition,
                WorkDispositionV1::Refused | WorkDispositionV1::Uncertain
            ),
            "{attempted:?}"
        );
        assert!(attempted.outcome.is_none());
    }
    assert_eq!(
        fs::read(d.path().join(".handbook-work-state-bootstrap.v1.json")).unwrap(),
        bootstrap_before
    );
    assert_eq!(
        project_before,
        original
            .check
            .predecessor
            .charter
            .binding
            .initialization_claim
            .proposed_repository_identity()
    );
    assert!(!d.path().join(".handbook/work-state/work.lock").exists());
    assert!(!d.path().join(".handbook/work-state/v1/owner.json").exists());
}

#[test]
fn actual_early_acceptance_interruptions_keep_positive_reservation_and_truthful_uncertainty() {
    for boundary in [
        WorkWriteBoundaryV1::Reservation,
        WorkWriteBoundaryV1::AcceptanceCandidate,
    ] {
        let (d, s, i, grant) = fixture();
        let prepared = s
            .prepare_work_registration(&i, &grant, input(&grant))
            .unwrap();
        let original = prepared.locator().clone();
        let project_before = grant
            .check
            .predecessor
            .charter
            .binding
            .initialization_claim
            .proposed_repository_identity()
            .to_owned();
        let execution =
            NativeWorkStateV1::register_with_boundary_for_test(d.path(), prepared, |at| {
                if at == boundary {
                    Err(interruption())
                } else {
                    Ok(())
                }
            });
        assert_eq!(
            execution.disposition,
            WorkDispositionV1::Uncertain,
            "{boundary:?}: {execution:?}"
        );
        let slot_path = d.path().join(".handbook/work-state/v1/slot-01.json");
        let slot_before = fs::read(&slot_path).unwrap();
        let slot: serde_json::Value = serde_json::from_slice(&slot_before).unwrap();
        assert_eq!(
            slot["entry"]["name"], "item-request",
            "original occupied slot missing"
        );
        assert!(
            s.resolve_current_work_item(&grant, "item").is_err(),
            "{boundary:?} supplied item absence"
        );
        let recovery = s.recover_work_registration_request(&original);
        assert_eq!(
            recovery.disposition,
            WorkDispositionV1::Uncertain,
            "{boundary:?}: {recovery:?}"
        );
        assert!(recovery.outcome.is_none());
        let mut replacement = input(&grant);
        replacement.request_key = "new-key-after-early-interruption".into();
        assert!(s
            .prepare_work_registration(&i, &grant, replacement)
            .is_err());
        assert_eq!(
            fs::read(&slot_path).unwrap(),
            slot_before,
            "recovery erased or replaced original reservation"
        );
        assert!(!d
            .path()
            .join(".handbook/work-state/v1/item-new-key-after-early-interruption")
            .exists());
        assert_eq!(
            project_before,
            grant
                .check
                .predecessor
                .charter
                .binding
                .initialization_claim
                .proposed_repository_identity()
        );
    }
}

#[test]
fn staged_original_registration_and_activation_retry_report_uncertain_after_acceptance_write() {
    // Registration retry uses another real sealed preparation of the saved
    // original request, rather than the recovery entry point.
    let (d, sdk, initial, grant) = fixture();
    let prepared = sdk
        .prepare_work_registration(&initial, &grant, input(&grant))
        .unwrap();
    let original = prepared.locator().clone();
    let first = NativeWorkStateV1::register_with_boundary_for_test(d.path(), prepared, |at| {
        if at == WorkWriteBoundaryV1::StagedIntent {
            Err(interruption())
        } else {
            Ok(())
        }
    });
    assert_eq!(first.disposition, WorkDispositionV1::Uncertain, "{first:?}");
    assert!(first.outcome.is_none());
    let stage = d.path().join(".handbook/work-state/v1/accept-item-request");
    let intent_before = fs::read(stage.join("intent.json")).unwrap();
    let slot_path = d.path().join(".handbook/work-state/v1/slot-01.json");
    let slot_before = fs::read(&slot_path).unwrap();
    assert!(!stage.join("not-attempted.json").exists());
    assert_eq!(
        sdk.resolve_work_registration_request(&original).disposition,
        WorkDispositionV1::Pending
    );
    assert_eq!(
        sdk.resolve_current_work_item(&grant, "item").unwrap(),
        WorkCurrentItemV1::Pending {
            original_request: original.clone()
        }
    );
    let repeated = sdk
        .prepare_work_registration(&initial, &grant, original.input.clone())
        .unwrap();
    assert_eq!(repeated.locator(), &original);
    let retry = NativeWorkStateV1::register_with_boundary_for_test(d.path(), repeated, |at| {
        if at == WorkWriteBoundaryV1::AcceptanceReady {
            Err(interruption())
        } else {
            Ok(())
        }
    });
    assert_eq!(retry.disposition, WorkDispositionV1::Uncertain, "{retry:?}");
    assert!(retry.outcome.is_none());
    assert!(
        stage.join("not-attempted.json").exists(),
        "actual retry checkpoint write was not retained"
    );
    assert_eq!(fs::read(stage.join("intent.json")).unwrap(), intent_before);
    assert_eq!(fs::read(&slot_path).unwrap(), slot_before);
    assert_eq!(
        sdk.resolve_work_registration_request(&original).disposition,
        WorkDispositionV1::Pending
    );
    assert_eq!(
        sdk.resolve_current_work_item(&grant, "item").unwrap(),
        WorkCurrentItemV1::Pending {
            original_request: original.clone()
        }
    );
    let recovered = sdk.recover_work_registration_request(&original);
    assert_eq!(
        recovered.disposition,
        WorkDispositionV1::Committed,
        "{recovered:?}"
    );
    let receipt = recovered.outcome.unwrap();
    assert_eq!(receipt.locator, original);
    assert_eq!(
        receipt.snapshot.project_identity,
        grant
            .check
            .predecessor
            .charter
            .binding
            .initialization_claim
            .proposed_repository_identity()
    );
    assert_eq!(
        sdk.resolve_current_work_item(&grant, "item").unwrap(),
        WorkCurrentItemV1::Open(receipt.snapshot.clone())
    );
    let replay = sdk.recover_work_registration_request(&original);
    assert_eq!(replay.disposition, WorkDispositionV1::Replayed);
    assert_eq!(replay.outcome.unwrap(), receipt);
    assert_eq!(fs::read(&slot_path).unwrap(), slot_before);
    assert!(!stage.exists());

    // Activation retry is prepared by the same original human establishment
    // route. Its pending candidate never supplies its own permission.
    let (d, sdk, initial, prepared) = prepared_activation();
    let original = prepared.locator().clone();
    let project_before = original
        .check
        .predecessor
        .charter
        .binding
        .initialization_claim
        .proposed_repository_identity()
        .to_owned();
    let first = NativeWorkStateV1::activate_with_boundary_for_test(d.path(), prepared, |at| {
        if at == WorkWriteBoundaryV1::StagedIntent {
            Err(interruption())
        } else {
            Ok(())
        }
    });
    assert_eq!(first.disposition, WorkDispositionV1::Uncertain, "{first:?}");
    assert!(first.outcome.is_none());
    let stage = d.path().join(".handbook/work-state/v1/accept-grant-work");
    let intent_before = fs::read(stage.join("intent.json")).unwrap();
    let slot_path = d.path().join(".handbook/work-state/v1/slot-00.json");
    let slot_before = fs::read(&slot_path).unwrap();
    assert!(!stage.join("not-attempted.json").exists());
    assert_eq!(
        sdk.resolve_work_registration_terms_request(&original)
            .disposition,
        WorkDispositionV1::Pending
    );
    assert!(sdk
        .resolve_current_work_registration_terms(&initial, &original)
        .is_err());
    let repeated = sdk
        .prepare_work_registration_terms(&initial, &original.check, original.terms.clone(), None)
        .unwrap();
    assert_eq!(repeated.locator(), &original);
    let retry = NativeWorkStateV1::activate_with_boundary_for_test(d.path(), repeated, |at| {
        if at == WorkWriteBoundaryV1::AcceptanceReady {
            Err(interruption())
        } else {
            Ok(())
        }
    });
    assert_eq!(retry.disposition, WorkDispositionV1::Uncertain, "{retry:?}");
    assert!(retry.outcome.is_none());
    assert!(
        stage.join("not-attempted.json").exists(),
        "actual activation retry checkpoint write was not retained"
    );
    assert_eq!(fs::read(stage.join("intent.json")).unwrap(), intent_before);
    assert_eq!(fs::read(&slot_path).unwrap(), slot_before);
    assert_eq!(
        sdk.resolve_work_registration_terms_request(&original)
            .disposition,
        WorkDispositionV1::Pending
    );
    assert!(sdk
        .resolve_current_work_registration_terms(&initial, &original)
        .is_err());
    let recovered = sdk.recover_work_registration_terms_request(&original);
    assert_eq!(
        recovered.disposition,
        WorkDispositionV1::Committed,
        "{recovered:?}"
    );
    let grant = recovered.outcome.unwrap();
    assert_eq!(grant.locator, original);
    assert_eq!(
        sdk.resolve_current_work_registration_terms(&initial, &original)
            .unwrap(),
        grant
    );
    let replay = sdk.recover_work_registration_terms_request(&original);
    assert_eq!(replay.disposition, WorkDispositionV1::Replayed);
    assert_eq!(replay.outcome.unwrap(), grant);
    assert_eq!(fs::read(&slot_path).unwrap(), slot_before);
    assert_eq!(
        project_before,
        original
            .check
            .predecessor
            .charter
            .binding
            .initialization_claim
            .proposed_repository_identity()
    );
    assert!(!stage.exists());
}
