//! Retained kind proof and exact dependency closure at native publication.
#[allow(dead_code)]
mod work_kind_support;
use handbook_engine::work_state::{NativeWorkStateV1, WorkWriteBoundaryV1 as B};
use handbook_sdk::work_state::*;
use handbook_sdk::HandbookSdkV1;
use std::fs;
use tempfile::tempdir;
use work_kind_support::*;

#[test]
fn complete_derived_definition_records_refuse_before_owner_bootstrap() {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let mut content = definition("project.kind.large@1.0.0");
    content.template.display_levels = (0..4300)
        .map(|index| format!("level-{index:05}-{}", "x".repeat(116)))
        .collect();
    let source_bytes = content.canonical_json_bytes().unwrap();
    assert!(source_bytes.len() < 1024 * 1024);
    // The complete receipt retains definition both in its original locator and
    // as committed content, before accounting for its other required fields.
    assert!(2 * source_bytes.len() > 1024 * 1024);
    let supplied = approval(&sdk, &initial, &check, &content, "large-kind");
    let refusal = sdk
        .prepare_work_kind_establishment(&initial, &check, content, supplied)
        .unwrap_err();
    assert_eq!(refusal.code, "work_codec");
    for path in [
        ".handbook-work-state.v1.json",
        ".handbook-work-state-bootstrap.v1.json",
        ".handbook/work-state",
        ".handbook/work-state-bootstrap",
    ] {
        assert!(
            !dir.path().join(path).exists(),
            "derived-record refusal began owner bootstrap: {path}"
        );
    }
}

#[test]
fn complete_derived_item_records_refuse_before_positive_reservation() {
    let directory = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(directory.path());
    let (initial, check) = setup(&sdk);
    let mut content = definition("project.kind.item-bound@1.0.0");
    content.template.display_levels = (0..3750)
        .map(|index| format!("level-{index:05}-{}", "x".repeat(116)))
        .collect();
    let kind = sdk
        .prepare_work_kind_establishment(
            &initial,
            &check,
            content.clone(),
            approval(&sdk, &initial, &check, &content, "large-committed-kind"),
        )
        .unwrap();
    let binding = kind.locator().binding.clone();
    assert_eq!(
        sdk.establish_work_kind(kind).disposition,
        WorkDispositionV1::Committed
    );
    let grant = activate(&sdk, &initial, &check, "grant", None);
    let prepared = sdk
        .prepare_work_registration(
            &initial,
            &grant,
            input(&grant, &binding, "oversized-derived-item"),
        )
        .unwrap();
    let original = prepared.locator().clone();
    let locator_bytes = original.canonical_json_bytes().unwrap();
    assert!(locator_bytes.len() <= 1024 * 1024);
    let store = directory.path().join(".handbook/work-state/v1");
    let manifest_before = fs::read(directory.path().join(".handbook-work-state.v1.json")).unwrap();
    let slots_before: Vec<_> = (0..32)
        .map(|index| fs::read(store.join(format!("slot-{index:02}.json"))).unwrap())
        .collect();
    let execution = sdk.register_work_item(prepared);
    assert_eq!(
        execution.disposition,
        WorkDispositionV1::Refused,
        "{execution:?}"
    );
    assert_eq!(execution.refusal.as_ref().unwrap().code, "work_codec");
    assert_eq!(
        sdk.resolve_work_registration_request(&original).disposition,
        WorkDispositionV1::Unaccepted
    );
    assert!(!store.join("item-oversized-derived-item").exists());
    assert!(!store.join("accept-item-oversized-derived-item").exists());
    assert_eq!(
        fs::read(directory.path().join(".handbook-work-state.v1.json")).unwrap(),
        manifest_before
    );
    for (index, before) in slots_before.iter().enumerate() {
        assert_eq!(
            &fs::read(store.join(format!("slot-{index:02}.json"))).unwrap(),
            before,
            "positive slot {index} changed before complete-record bounds"
        );
    }
}

#[test]
fn kind_first_bootstrap_interruption_never_repairs_owner_or_mints_a_dummy_grant() {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let content = definition("project.kind.leaf@1.0.0");
    let prepared = sdk
        .prepare_work_kind_establishment(
            &initial,
            &check,
            content.clone(),
            approval(&sdk, &initial, &check, &content, "first-kind"),
        )
        .unwrap();
    let original = prepared.locator().clone();
    let execution = NativeWorkStateV1::establish_kind_with_boundary_for_test(
        dir.path(),
        prepared,
        fault(B::BootstrapIntent),
    );
    assert_eq!(
        execution.disposition,
        WorkDispositionV1::Uncertain,
        "{execution:?}"
    );
    let bootstrap = dir.path().join(".handbook-work-state-bootstrap.v1.json");
    let retained = fs::read(&bootstrap).unwrap();
    assert!(!dir.path().join(".handbook/work-state/work.lock").exists());
    assert!(!dir
        .path()
        .join(".handbook/work-state/v1/owner.json")
        .exists());
    assert_eq!(
        sdk.recover_work_kind_request(&original).disposition,
        WorkDispositionV1::Uncertain
    );
    let grant = sdk.prepare_work_registration_terms(
        &initial,
        &check,
        terms(&check, "first-grant"),
        WorkTermsTransitionV1::Initial,
    );
    if let Ok(prepared) = grant {
        let execution = sdk.activate_work_registration_terms(prepared);
        assert!(
            matches!(
                execution.disposition,
                WorkDispositionV1::Refused | WorkDispositionV1::Uncertain
            ),
            "{execution:?}"
        );
        assert!(execution.outcome.is_none());
    }
    assert_eq!(fs::read(bootstrap).unwrap(), retained);
    assert!(!dir
        .path()
        .join(".handbook/work-state/v1/grant-first-grant")
        .exists());
    assert!(!dir.path().join(".handbook/work-state/work.lock").exists());
}

#[test]
fn early_kind_and_kind_item_acceptance_keeps_original_positive_reservations() {
    for kind in [true, false] {
        for at in [
            B::Reservation,
            B::AcceptanceCandidate,
            B::StagedIntent,
            B::AcceptanceReady,
        ] {
            let dir = tempdir().unwrap();
            let sdk = HandbookSdkV1::open(dir.path());
            let (initial, check) = setup(&sdk);
            let grant = activate(&sdk, &initial, &check, "grant", None);
            if kind {
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
                let execution = NativeWorkStateV1::establish_kind_with_boundary_for_test(
                    dir.path(),
                    prepared,
                    fault(at),
                );
                assert_eq!(
                    execution.disposition,
                    WorkDispositionV1::Uncertain,
                    "kind {at:?}: {execution:?}"
                );
                let slot = dir.path().join(".handbook/work-state/v1/slot-01.json");
                let retained = fs::read(&slot).unwrap();
                let recovered = sdk.recover_work_kind_request(&original);
                assert_eq!(
                    recovered.disposition,
                    if matches!(at, B::StagedIntent | B::AcceptanceReady) {
                        WorkDispositionV1::Committed
                    } else {
                        WorkDispositionV1::Uncertain
                    },
                    "kind {at:?}: {recovered:?}"
                );
                assert_eq!(fs::read(slot).unwrap(), retained);
                assert!(sdk
                    .prepare_work_kind_establishment(
                        &initial,
                        &check,
                        content.clone(),
                        approval(&sdk, &initial, &check, &content, "replacement-kind")
                    )
                    .is_err());
            } else {
                let binding =
                    establish_kind(&sdk, &initial, &check, "kind", "project.kind.leaf@1.0.0");
                let prepared = sdk
                    .prepare_work_registration(
                        &initial,
                        &grant,
                        input(&grant, &binding, "original-item"),
                    )
                    .unwrap();
                let original = prepared.locator().clone();
                let execution = NativeWorkStateV1::register_with_boundary_for_test(
                    dir.path(),
                    prepared,
                    fault(at),
                );
                assert_eq!(
                    execution.disposition,
                    WorkDispositionV1::Uncertain,
                    "item {at:?}: {execution:?}"
                );
                let slot = dir.path().join(".handbook/work-state/v1/slot-02.json");
                let retained = fs::read(&slot).unwrap();
                let recovered = sdk.recover_work_registration_request(&original);
                assert_eq!(
                    recovered.disposition,
                    if matches!(at, B::StagedIntent | B::AcceptanceReady) {
                        WorkDispositionV1::Committed
                    } else {
                        WorkDispositionV1::Uncertain
                    },
                    "item {at:?}: {recovered:?}"
                );
                assert_eq!(fs::read(slot).unwrap(), retained);
                let mut replacement = input(&grant, &binding, "replacement-item");
                replacement.item_key = "original-item".into();
                let binding = sdk.prepare_work_registration(&initial, &grant, replacement);
                if matches!(at, B::StagedIntent | B::AcceptanceReady) {
                    assert_eq!(
                        sdk.register_work_item(binding.unwrap()).disposition,
                        WorkDispositionV1::Refused
                    );
                } else {
                    assert!(
                        binding.is_err(),
                        "partial positive inventory must not supply kind provenance"
                    );
                }
            }
        }
    }
}

#[test]
fn lost_kind_owner_dependencies_never_rebootstrap_or_supply_absence() {
    for file in [
        ".handbook-work-state.v1.json",
        ".handbook-work-state-bootstrap.v1.json",
        ".handbook/work-state/v1/owner.json",
        ".handbook/work-state/work.lock",
        ".handbook/work-state/v1/slot-00.json",
    ] {
        let dir = tempdir().unwrap();
        let sdk = HandbookSdkV1::open(dir.path());
        let (initial, check) = setup(&sdk);
        let content = definition("project.kind.leaf@1.0.0");
        let prepared = sdk
            .prepare_work_kind_establishment(
                &initial,
                &check,
                content.clone(),
                approval(&sdk, &initial, &check, &content, "kind"),
            )
            .unwrap();
        let original = prepared.locator().clone();
        let binding = sdk.establish_work_kind(prepared).outcome.unwrap().binding;
        fs::remove_file(dir.path().join(file)).unwrap();
        assert!(
            sdk.resolve_work_kind(&binding).is_err(),
            "lost {file} became committed/absent"
        );
        assert_eq!(
            sdk.recover_work_kind_request(&original).disposition,
            WorkDispositionV1::Uncertain,
            "{file}"
        );
        if let Ok(prepared) = sdk.prepare_work_registration_terms(
            &initial,
            &check,
            terms(&check, "replacement-grant"),
            WorkTermsTransitionV1::Initial,
        ) {
            assert!(sdk
                .activate_work_registration_terms(prepared)
                .outcome
                .is_none());
        }
        assert!(
            !dir.path().join(file).exists(),
            "lost owner dependency recreated: {file}"
        );
    }
}

#[test]
fn lost_whole_kind_transaction_and_corrupt_exact_definition_fail_closed() {
    for corruption in [
        "whole-transaction",
        "intent",
        "definition",
        "terminal",
        "attempt",
        "unknown-field",
    ] {
        let dir = tempdir().unwrap();
        let sdk = HandbookSdkV1::open(dir.path());
        let (initial, check) = setup(&sdk);
        let content = definition("project.kind.leaf@1.0.0");
        let prepared = sdk
            .prepare_work_kind_establishment(
                &initial,
                &check,
                content.clone(),
                approval(&sdk, &initial, &check, &content, "kind"),
            )
            .unwrap();
        let original = prepared.locator().clone();
        let binding = sdk.establish_work_kind(prepared).outcome.unwrap().binding;
        let transaction = dir.path().join(".handbook/work-state/v1/kind-kind");
        match corruption {
            "whole-transaction" => {
                let root = fs::canonicalize(dir.path()).unwrap();
                let source = fs::canonicalize(&transaction).unwrap();
                let destination = root.join("lost-kind-transaction");
                assert!(source.starts_with(&root) && destination.starts_with(&root));
                fs::rename(&source, destination).unwrap();
            }
            "intent" => fs::remove_file(transaction.join("intent.json")).unwrap(),
            "terminal" => fs::remove_file(transaction.join("terminal.json")).unwrap(),
            "attempt" => fs::remove_file(transaction.join("attempted.json")).unwrap(),
            "definition" => fs::write(transaction.join("snapshot.json"), b"{}").unwrap(),
            "unknown-field" => {
                let path = transaction.join("snapshot.json");
                let mut value: serde_json::Value =
                    serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
                value["future_kind_hook"] = serde_json::json!("execute");
                fs::write(path, serde_json::to_vec(&value).unwrap()).unwrap();
            }
            _ => unreachable!(),
        }
        assert!(
            matches!(
                sdk.resolve_work_kind(&binding),
                Err(_) | Ok(WorkCurrentKindV1::Pending { .. })
            ),
            "corrupt {corruption} produced committed or absent kind truth"
        );
        assert_eq!(
            sdk.recover_work_kind_request(&original).disposition,
            WorkDispositionV1::Uncertain,
            "{corruption}"
        );
        assert!(sdk
            .prepare_work_kind_establishment(
                &initial,
                &check,
                content.clone(),
                approval(&sdk, &initial, &check, &content, "replacement")
            )
            .is_err());
    }
}

#[test]
fn repeated_kind_history_reads_reject_in_place_and_substituted_original_definition_records() {
    for (boundary, record, pointer) in [
        (
            B::BeforeIntent,
            "intent.json",
            "/operation/locator/definition/template/label_max_bytes",
        ),
        (B::Snapshot, "intent.json", "/authority/schema_ref"),
        (
            B::Snapshot,
            "snapshot.json",
            "/value/definition/template/label_max_bytes",
        ),
        (B::Terminal, "terminal.json", "/effect"),
    ] {
        for substitute in [false, true] {
            let dir = tempdir().unwrap();
            let sdk = HandbookSdkV1::open(dir.path());
            let (initial, check) = setup(&sdk);
            let binding = establish_kind(&sdk, &initial, &check, "kind", "project.kind.leaf@1.0.0");
            let grant = activate(&sdk, &initial, &check, "grant", None);
            let prepared = sdk
                .prepare_work_registration(&initial, &grant, input(&grant, &binding, "item"))
                .unwrap();
            let original = prepared.locator().clone();
            let mut changed = false;
            let execution = NativeWorkStateV1::register_with_boundary_for_test(
                dir.path(),
                prepared,
                |at| {
                    if at == boundary {
                        let path = dir
                            .path()
                            .join(".handbook/work-state/v1/kind-kind")
                            .join(record);
                        let mut value: serde_json::Value =
                            serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
                        let target = value
                            .pointer_mut(pointer)
                            .unwrap_or_else(|| {
                                panic!(
                                    "missing exact kind field: {boundary:?} {record} {pointer} substitution={substitute}"
                                )
                            });
                        *target = if pointer.ends_with("label_max_bytes") {
                            serde_json::json!(1)
                        } else {
                            serde_json::json!("changed-exact-kind-record")
                        };
                        if substitute {
                            changed = fs::rename(&path, path.with_extension("replaced")).is_ok();
                            if changed {
                                let _ = fs::write(&path, serde_json::to_vec(&value).unwrap());
                            }
                        } else {
                            changed = fs::write(&path, serde_json::to_vec(&value).unwrap()).is_ok();
                        }
                    }
                    Ok(())
                },
            );
            if changed {
                assert!(
                    execution.outcome.is_none(),
                    "{boundary:?} {record} substitution={substitute}: {execution:?}"
                );
                if boundary != B::Terminal {
                    assert!(!dir
                        .path()
                        .join(".handbook/work-state/v1/item-item/terminal.json")
                        .exists());
                }
                assert_eq!(
                    sdk.recover_work_registration_request(&original).disposition,
                    WorkDispositionV1::Uncertain
                );
            } else {
                assert_eq!(
                    execution.disposition,
                    WorkDispositionV1::Committed,
                    "retained handle denied kind mutation: {boundary:?} {record}: {execution:?}"
                );
            }
        }
    }
}

#[test]
fn definition_dependency_substitution_before_kind_item_terminal_cannot_publish() {
    for file in [
        ".handbook/work-state/work.lock",
        ".handbook/work-state/v1/kind-kind/intent.json",
        ".handbook/work-state/v1/kind-kind/decision.json",
        ".handbook/work-state/v1/kind-kind/snapshot.json",
        ".handbook/work-state/v1/kind-kind/terminal.json",
    ] {
        let dir = tempdir().unwrap();
        let sdk = HandbookSdkV1::open(dir.path());
        let (initial, check) = setup(&sdk);
        let binding = establish_kind(&sdk, &initial, &check, "kind", "project.kind.leaf@1.0.0");
        let grant = activate(&sdk, &initial, &check, "grant", None);
        let prepared = sdk
            .prepare_work_registration(&initial, &grant, input(&grant, &binding, "item"))
            .unwrap();
        let original = prepared.locator().clone();
        let mut substituted = false;
        let execution =
            NativeWorkStateV1::register_with_boundary_for_test(dir.path(), prepared, |at| {
                if at == B::Snapshot {
                    let path = dir.path().join(file);
                    if fs::rename(&path, path.with_extension("substituted")).is_ok() {
                        fs::write(path, []).unwrap();
                        substituted = true;
                    }
                }
                Ok(())
            });
        if substituted {
            assert!(
                execution.outcome.is_none(),
                "changed kind proof allowed item terminal: {file}: {execution:?}"
            );
            assert!(!dir
                .path()
                .join(".handbook/work-state/v1/item-item/terminal.json")
                .exists());
            assert_eq!(
                sdk.recover_work_registration_request(&original).disposition,
                WorkDispositionV1::Uncertain
            );
        } else {
            // Windows retained native handles can positively prevent the replacement itself.
            assert_eq!(
                execution.disposition,
                WorkDispositionV1::Committed,
                "native pin denied replacement but failed publication: {file}: {execution:?}"
            );
        }
    }
}

#[test]
fn committed_kind_item_cannot_recover_without_exact_original_definition_proof() {
    for file in ["intent.json", "snapshot.json", "terminal.json"] {
        let dir = tempdir().unwrap();
        let sdk = HandbookSdkV1::open(dir.path());
        let (initial, check) = setup(&sdk);
        let binding = establish_kind(&sdk, &initial, &check, "kind", "project.kind.leaf@1.0.0");
        let grant = activate(&sdk, &initial, &check, "grant", None);
        let prepared = sdk
            .prepare_work_registration(&initial, &grant, input(&grant, &binding, "item"))
            .unwrap();
        let original = prepared.locator().clone();
        let receipt = sdk.register_work_item(prepared).outcome.unwrap();
        assert_eq!(receipt.snapshot.admitted_kind.as_ref(), Some(&binding));
        fs::remove_file(
            dir.path()
                .join(".handbook/work-state/v1/kind-kind")
                .join(file),
        )
        .unwrap();
        assert!(sdk.resolve_current_work_item(&grant, "item").is_err());
        assert_eq!(
            sdk.recover_work_registration_request(&original).disposition,
            WorkDispositionV1::Uncertain
        );
    }
}

#[test]
fn k2_and_scope_revocation_preserve_k1_definition_and_original_kind_item_receipt() {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let binding = establish_kind(&sdk, &initial, &check, "kind-1", "project.kind.leaf@1.0.0");
    let grant = activate(&sdk, &initial, &check, "grant", None);
    let prepared = sdk
        .prepare_work_registration(&initial, &grant, input(&grant, &binding, "item"))
        .unwrap();
    let original = prepared.locator().clone();
    let receipt = sdk.register_work_item(prepared).outcome.unwrap();
    let first_bytes = fs::read(
        dir.path()
            .join(".handbook/work-state/v1/kind-kind-1/snapshot.json"),
    )
    .unwrap();
    let item_bytes = fs::read(
        dir.path()
            .join(".handbook/work-state/v1/item-item/snapshot.json"),
    )
    .unwrap();
    let mut second = definition("project.kind.leaf@2.0.0");
    second.template.label_max_bytes = 4;
    let prepared = sdk
        .prepare_work_kind_establishment(
            &initial,
            &check,
            second.clone(),
            approval(&sdk, &initial, &check, &second, "kind-2"),
        )
        .unwrap();
    assert_eq!(
        sdk.establish_work_kind(prepared).disposition,
        WorkDispositionV1::Committed
    );
    revoke(&sdk, &grant);
    let third = establish_kind(&sdk, &initial, &check, "kind-3", "project.kind.leaf@3.0.0");
    assert!(matches!(
        sdk.resolve_work_kind(&third).unwrap(),
        WorkCurrentKindV1::Committed(_)
    ));
    let bound_revoked = sdk
        .prepare_work_registration(&initial, &grant, input(&grant, &binding, "revoked-use"))
        .unwrap();
    assert_eq!(
        sdk.register_work_item(bound_revoked).disposition,
        WorkDispositionV1::Refused
    );
    reset(&sdk);
    let reopened = HandbookSdkV1::open(dir.path());
    assert!(matches!(
        reopened.resolve_work_kind(&binding).unwrap(),
        WorkCurrentKindV1::Committed(_)
    ));
    assert_eq!(
        reopened
            .recover_work_registration_request(&original)
            .outcome
            .unwrap(),
        receipt
    );
    assert_eq!(
        fs::read(
            dir.path()
                .join(".handbook/work-state/v1/kind-kind-1/snapshot.json")
        )
        .unwrap(),
        first_bytes
    );
    assert_eq!(
        fs::read(
            dir.path()
                .join(".handbook/work-state/v1/item-item/snapshot.json")
        )
        .unwrap(),
        item_bytes
    );
}
