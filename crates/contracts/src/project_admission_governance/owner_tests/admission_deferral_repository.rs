#![cfg(feature = "test-support")]
use handbook_contracts::project_admission_governance::*;
use handbook_engine::{
    RepositoryInvocationIdentityServiceV1, REPOSITORY_IDENTITY_REPO_PATH,
    REPOSITORY_INITIALIZATION_CLAIM_PATH_V1, REPOSITORY_INITIALIZATION_LOCK_PATH_V1,
    REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1,
};
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn input() -> DeferralRequestInputV2 {
    DeferralRequestInputV2::human_approved(
        "initial",
        "Human approves deferring the exact freshness scope.",
    )
}
fn begin(fault: DeferralFaultBoundaryV2) -> (tempfile::TempDir, DeferralRequestLocatorV2) {
    let root = tempfile::tempdir().unwrap();
    let owner = ProjectAdmissionDeferralV2::new().with_fault_for_testing(fault);
    let prepared = owner.prepare(root.path(), &input()).unwrap();
    let locator = prepared.locator().clone();
    assert!(matches!(
        owner.initialize(root.path(), prepared),
        DeferralExecutionV2::Uncertain { .. }
    ));
    (root, locator)
}
fn history(root: &Path, locator: &DeferralRequestLocatorV2, name: &str) -> PathBuf {
    root.join(deferral_request_storage_path_v2(locator))
        .join(name)
}
fn boundaries() -> Vec<DeferralFaultBoundaryV2> {
    [
        DeferralFaultBoundaryV2::Claim,
        DeferralFaultBoundaryV2::Reservation,
        DeferralFaultBoundaryV2::Anchor,
        DeferralFaultBoundaryV2::Identity,
        DeferralFaultBoundaryV2::Registration,
    ]
    .into_iter()
    .chain((0..6).map(DeferralFaultBoundaryV2::Stage))
    .chain([
        DeferralFaultBoundaryV2::Intent,
        DeferralFaultBoundaryV2::Decision,
    ])
    .chain((0..6).map(DeferralFaultBoundaryV2::Installation))
    .chain([
        DeferralFaultBoundaryV2::CurrentRelation,
        DeferralFaultBoundaryV2::Terminal,
        DeferralFaultBoundaryV2::ClaimReceipt,
        DeferralFaultBoundaryV2::Reply,
    ])
    .collect()
}

#[test]
fn every_error_boundary_retains_exact_prefix_and_truthful_resolution() {
    for fault in boundaries() {
        let (root, locator) = begin(fault);
        let owner = ProjectAdmissionDeferralV2::new();
        let resolved = owner.resolve_request(root.path(), &locator);
        let decided = matches!(
            fault,
            DeferralFaultBoundaryV2::Decision
                | DeferralFaultBoundaryV2::Installation(_)
                | DeferralFaultBoundaryV2::CurrentRelation
                | DeferralFaultBoundaryV2::Terminal
                | DeferralFaultBoundaryV2::ClaimReceipt
                | DeferralFaultBoundaryV2::Reply
        );
        if decided {
            assert!(
                matches!(&resolved, DeferralExecutionV2::Committed { outcome, delivery: GovernanceDeliveryV1::Replay } if outcome.locator == locator && outcome.deferral_committed),
                "{fault:?}: {resolved:?}"
            );
            assert!(
                owner
                    .resolve_current(root.path(), &locator.project, &locator.request.scope, None)
                    .unwrap()
                    .never_established
            );
            assert_eq!(
                RepositoryInvocationIdentityServiceV1::new()
                    .initialize_for_setup(root.path())
                    .unwrap()
                    .repository_identity_fingerprint(),
                locator.project.repository_identity_fingerprint
            );
        } else {
            assert!(
                matches!(&resolved, DeferralExecutionV2::NotCommitted { outcome, delivery: GovernanceDeliveryV1::Replay } if outcome.locator == locator && !outcome.deferral_committed && outcome.condition == "verified_predecision_noncommit"),
                "{fault:?}: {resolved:?}"
            );
            assert!(owner
                .resolve_current(root.path(), &locator.project, &locator.request.scope, None)
                .is_err());
            assert!(RepositoryInvocationIdentityServiceV1::new()
                .initialize_for_setup(root.path())
                .is_err());
        }
        assert_eq!(
            owner.resolve_request(root.path(), &locator),
            resolved,
            "{fault:?}"
        );
        assert!(owner.prepare(root.path(), &input()).is_err());
        assert!(ProjectAdmissionGovernanceV1::new()
            .prepare(
                root.path(),
                &GovernanceRequestInputV1::standalone("other", 300, "Establish project")
            )
            .is_err());
        if fault == DeferralFaultBoundaryV2::Claim {
            assert!(!root.path().join(".handbook").exists());
        }
    }
}

#[test]
fn terminal_replay_never_repairs_or_selects_mutable_current() {
    for damage in ["missing", "changed"] {
        let (root, locator) = begin(DeferralFaultBoundaryV2::Reply);
        let owner = ProjectAdmissionDeferralV2::new();
        let original = owner.resolve_request(root.path(), &locator);
        let current = root.path().join(GOVERNANCE_CURRENT_PATH_V2);
        if damage == "missing" {
            fs::remove_file(&current).unwrap();
        } else {
            fs::write(&current, b"{}").unwrap();
        }
        assert_eq!(owner.resolve_request(root.path(), &locator), original);
        assert!(owner
            .resolve_current(root.path(), &locator.project, &locator.request.scope, None)
            .is_err());
        if damage == "missing" {
            assert!(!current.exists());
        } else {
            assert_eq!(fs::read(current).unwrap(), b"{}");
        }
    }
}

#[test]
fn deleted_or_changed_causal_evidence_never_reconstructs_authority() {
    let names = [
        "stage-0.json",
        "stage-5.json",
        "intent.json",
        "decision.json",
        "rule.json",
        "instruction.json",
        "deferral.json",
        "outcome.json",
        "consumed.json",
        "relation.json",
    ];
    for name in names {
        let (root, locator) = begin(DeferralFaultBoundaryV2::Terminal);
        let target = history(root.path(), &locator, name);
        fs::remove_file(&target).unwrap();
        let result = ProjectAdmissionDeferralV2::new().resolve_request(root.path(), &locator);
        assert!(
            matches!(result, DeferralExecutionV2::Uncertain { .. }),
            "{name}: {result:?}"
        );
        assert!(!target.exists(), "Deleted {name} was rebuilt");
    }
    for name in ["registration.json", "decision.json", "terminal.json"] {
        let (root, locator) = begin(DeferralFaultBoundaryV2::Reply);
        let target = if name == "registration.json" {
            root.path().join(GOVERNANCE_STORAGE_PATH_V2).join(name)
        } else {
            history(root.path(), &locator, name)
        };
        fs::write(target, b"{}").unwrap();
        assert!(matches!(
            ProjectAdmissionDeferralV2::new().resolve_request(root.path(), &locator),
            DeferralExecutionV2::Uncertain { .. }
        ));
    }
}

#[test]
fn incomplete_early_prefix_refuses_positive_noncommit() {
    for later in [
        REPOSITORY_IDENTITY_REPO_PATH,
        ".handbook/.repository-identity.v1.tmp",
        ".handbook/unexpected",
    ] {
        let (root, locator) = begin(DeferralFaultBoundaryV2::Reservation);
        fs::write(
            root.path().join(later),
            locator.project.repository_identity_fingerprint.as_bytes(),
        )
        .unwrap();
        assert!(matches!(
            ProjectAdmissionDeferralV2::new().resolve_request(root.path(), &locator),
            DeferralExecutionV2::Uncertain { .. }
        ));
        assert!(!root.path().join(DEFERRAL_NONCOMMIT_PATH_V2).exists());
    }
    let (root, locator) = begin(DeferralFaultBoundaryV2::Stage(2));
    fs::remove_file(history(root.path(), &locator, "stage-1.json")).unwrap();
    assert!(matches!(
        ProjectAdmissionDeferralV2::new().resolve_request(root.path(), &locator),
        DeferralExecutionV2::Uncertain { .. }
    ));
    assert!(!root.path().join(DEFERRAL_NONCOMMIT_PATH_V2).exists());
}

#[test]
fn directory_only_owner_storage_is_not_a_verified_predecision_prefix() {
    for directory in [".handbook/admission-governance", GOVERNANCE_STORAGE_PATH_V2] {
        let (root, locator) = begin(DeferralFaultBoundaryV2::Anchor);
        fs::create_dir_all(root.path().join(directory)).unwrap();
        assert!(!root.path().join(REPOSITORY_IDENTITY_REPO_PATH).exists());
        assert!(!root
            .path()
            .join(".handbook/.repository-identity.v1.tmp")
            .exists());
        let owner = ProjectAdmissionDeferralV2::new();
        let result = owner.resolve_request(root.path(), &locator);
        assert!(
            matches!(&result, DeferralExecutionV2::Uncertain { condition, .. } if condition == "owner_storage_without_identity"),
            "{directory}: {result:?}"
        );
        assert!(!root.path().join(DEFERRAL_NONCOMMIT_PATH_V2).exists());
        assert!(!root
            .path()
            .join(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1)
            .exists());
        assert!(owner
            .resolve_current(root.path(), &locator.project, &locator.request.scope, None)
            .is_err());
        assert!(!root.path().join(REPOSITORY_IDENTITY_REPO_PATH).exists());
    }
}

#[test]
fn temporary_identity_does_not_authorize_later_owner_storage() {
    for directory in [".handbook/admission-governance", GOVERNANCE_STORAGE_PATH_V2] {
        let (root, locator) = begin(DeferralFaultBoundaryV2::Anchor);
        let temporary = root.path().join(".handbook/.repository-identity.v1.tmp");
        fs::write(
            &temporary,
            locator.project.repository_identity_fingerprint.as_bytes(),
        )
        .unwrap();
        fs::create_dir_all(root.path().join(directory)).unwrap();
        let owner = ProjectAdmissionDeferralV2::new();
        let result = owner.resolve_request(root.path(), &locator);
        assert!(
            matches!(&result, DeferralExecutionV2::Uncertain { condition, .. } if condition == "owner_storage_without_identity"),
            "{directory}: {result:?}"
        );
        assert!(!root.path().join(DEFERRAL_NONCOMMIT_PATH_V2).exists());
        assert!(!root
            .path()
            .join(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1)
            .exists());
        assert!(!root.path().join(REPOSITORY_IDENTITY_REPO_PATH).exists());
        assert_eq!(
            fs::read(temporary).unwrap(),
            locator.project.repository_identity_fingerprint.as_bytes()
        );
        assert!(owner
            .resolve_current(root.path(), &locator.project, &locator.request.scope, None)
            .is_err());
    }
}

#[test]
fn exact_temporary_identity_without_owner_storage_remains_predecision_noncommit() {
    let (root, locator) = begin(DeferralFaultBoundaryV2::Anchor);
    let temporary = root.path().join(".handbook/.repository-identity.v1.tmp");
    fs::write(
        &temporary,
        locator.project.repository_identity_fingerprint.as_bytes(),
    )
    .unwrap();
    let result = ProjectAdmissionDeferralV2::new().resolve_request(root.path(), &locator);
    assert!(matches!(result, DeferralExecutionV2::NotCommitted { .. }));
    assert!(!root.path().join(REPOSITORY_IDENTITY_REPO_PATH).exists());
    assert!(!root.path().join(".handbook/admission-governance").exists());
    assert_eq!(
        fs::read(temporary).unwrap(),
        locator.project.repository_identity_fingerprint.as_bytes()
    );
}

#[test]
fn retained_noncommit_witness_freezes_old_tokens_after_claim_and_receipt_loss() {
    for remove_root_lock in [false, true] {
        let root = tempfile::tempdir().unwrap();
        let v1 = ProjectAdmissionGovernanceV1::new();
        let establishment = GovernanceRequestInputV1::standalone(
            "before-deferral",
            300,
            "Human approved these exact initial establishment terms.",
        );
        let old_token = v1.prepare(root.path(), &establishment).unwrap();
        let deferral = ProjectAdmissionDeferralV2::new()
            .with_fault_for_testing(DeferralFaultBoundaryV2::Claim);
        let prepared = deferral.prepare(root.path(), &input()).unwrap();
        let locator = prepared.locator().clone();
        assert!(matches!(
            deferral.initialize(root.path(), prepared),
            DeferralExecutionV2::Uncertain { .. }
        ));
        let deferral = ProjectAdmissionDeferralV2::new();
        assert!(matches!(
            deferral.resolve_request(root.path(), &locator),
            DeferralExecutionV2::NotCommitted { .. }
        ));
        let witness = fs::read(root.path().join(DEFERRAL_NONCOMMIT_PATH_V2)).unwrap();
        fs::remove_file(root.path().join(REPOSITORY_INITIALIZATION_CLAIM_PATH_V1)).unwrap();
        fs::remove_file(root.path().join(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1)).unwrap();
        if remove_root_lock {
            fs::remove_file(root.path().join(REPOSITORY_INITIALIZATION_LOCK_PATH_V1)).unwrap();
        }
        assert!(
            v1.prepare(root.path(), &establishment).is_err(),
            "remove_root_lock={remove_root_lock}"
        );
        assert!(
            matches!(
                v1.initialize(root.path(), old_token),
                GovernanceExecutionV1::Refused { .. }
            ),
            "remove_root_lock={remove_root_lock}"
        );
        assert!(deferral.prepare(root.path(), &input()).is_err());
        assert!(matches!(
            deferral.resolve_request(root.path(), &locator),
            DeferralExecutionV2::Uncertain { .. }
        ));
        assert!(!root
            .path()
            .join(REPOSITORY_INITIALIZATION_CLAIM_PATH_V1)
            .exists());
        assert!(!root
            .path()
            .join(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1)
            .exists());
        assert!(!root.path().join(REPOSITORY_IDENTITY_REPO_PATH).exists());
        assert!(!root.path().join(".handbook").exists());
        assert_eq!(
            fs::read(root.path().join(DEFERRAL_NONCOMMIT_PATH_V2)).unwrap(),
            witness
        );
        assert_eq!(
            root.path()
                .join(REPOSITORY_INITIALIZATION_LOCK_PATH_V1)
                .exists(),
            !remove_root_lock
        );
    }
}

#[test]
fn missing_substituted_anchor_claim_identity_and_lock_fail_closed() {
    for name in [
        GOVERNANCE_ANCHOR_PATH_V2,
        REPOSITORY_IDENTITY_REPO_PATH,
        REPOSITORY_INITIALIZATION_CLAIM_PATH_V1,
        REPOSITORY_INITIALIZATION_LOCK_PATH_V1,
        REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1,
        ".handbook/admission-governance.lock",
    ] {
        for remove in [false, true] {
            let (root, locator) = begin(DeferralFaultBoundaryV2::Reply);
            if remove {
                fs::remove_file(root.path().join(name)).unwrap();
            } else {
                fs::write(root.path().join(name), b"substituted").unwrap();
            }
            let result = ProjectAdmissionDeferralV2::new().resolve_request(root.path(), &locator);
            // Missing receipt alone is deterministic terminal closure; it isn't missing owner history.
            if remove && name == REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1 {
                assert!(matches!(result, DeferralExecutionV2::Committed { .. }));
            } else {
                assert!(
                    matches!(result, DeferralExecutionV2::Uncertain { .. }),
                    "{name} remove={remove}: {result:?}"
                );
            }
        }
    }
}

#[test]
fn replaced_empty_root_and_governance_locks_refuse_after_restart() {
    for name in [
        REPOSITORY_INITIALIZATION_LOCK_PATH_V1,
        ".handbook/admission-governance.lock",
    ] {
        let (root, locator) = begin(DeferralFaultBoundaryV2::Reply);
        let lock = root.path().join(name);
        fs::rename(&lock, lock.with_extension("displaced")).unwrap();
        fs::write(&lock, b"").unwrap();
        let owner = ProjectAdmissionDeferralV2::new();
        assert!(
            matches!(
                owner.resolve_request(root.path(), &locator),
                DeferralExecutionV2::Uncertain { .. }
            ),
            "{name}"
        );
        assert!(owner
            .resolve_current(root.path(), &locator.project, &locator.request.scope, None)
            .is_err());
    }
}

#[test]
fn changed_complete_request_forged_expected_basis_and_other_scope_refuse() {
    let (root, locator) = begin(DeferralFaultBoundaryV2::Reply);
    let owner = ProjectAdmissionDeferralV2::new();
    let original = owner.resolve_request(root.path(), &locator);
    let mut changed = locator.clone();
    changed
        .request
        .instruction
        .text
        .push_str(" Different approval.");
    assert!(matches!(
        owner.resolve_request(root.path(), &changed),
        DeferralExecutionV2::Refused { .. }
    ));
    changed = locator.clone();
    changed.request.request_key = "other".into();
    assert!(matches!(
        owner.resolve_request(root.path(), &changed),
        DeferralExecutionV2::Refused { .. }
    ));
    let basis = owner
        .resolve_current(root.path(), &locator.project, &locator.request.scope, None)
        .unwrap();
    let expected = ExpectedDeferredBasisV2 {
        outcome_fingerprint: basis.relation_fingerprint.clone(),
        relation_fingerprint: basis.outcome_fingerprint.clone(),
    };
    assert!(owner
        .resolve_current(
            root.path(),
            &locator.project,
            &locator.request.scope,
            Some(&expected)
        )
        .is_err());
    assert!(owner
        .resolve_current(root.path(), &locator.project, "contracts.other/root", None)
        .is_err());
    assert_eq!(owner.resolve_request(root.path(), &locator), original);
}

#[test]
fn well_formed_changed_key_or_content_cannot_rebind_original_claim() {
    let (root, locator) = begin(DeferralFaultBoundaryV2::Reply);
    let owner = ProjectAdmissionDeferralV2::new();
    let original = owner.resolve_request(root.path(), &locator);
    for same_key in [true, false] {
        let mut changed = locator.clone();
        if same_key {
            changed
                .request
                .instruction
                .text
                .push_str(" Changed approved content.");
        } else {
            changed.request.request_key = "different-key".into();
        }
        let preimage = serde_json::json!({"domain":"handbook.project-admission-governance.request@2.0.0", "content":{"project":changed.project,"request":changed.request}});
        changed.request_fingerprint = handbook_contracts::FingerprintV1::parse(&format!(
            "sha256:{:x}",
            Sha256::digest(serde_json_canonicalizer::to_vec(&preimage).unwrap())
        ))
        .unwrap();
        let retained = handbook_engine::RetainedRepositoryRootV1::open_exact(root.path()).unwrap();
        changed.initialization_claim = handbook_engine::RepositoryInitializationClaimV1::prepare(
            root.path(),
            &retained,
            DEFERRAL_NATIVE_OWNER_V2,
            changed.request_fingerprint.as_str(),
            &changed.project.repository_identity_fingerprint,
        )
        .unwrap();
        validate_deferral_locator(&changed).unwrap();
        assert_eq!(
            changed.project.registration_identity,
            locator.project.registration_identity
        );
        assert_ne!(changed.request_fingerprint, locator.request_fingerprint);
        let result = owner.resolve_request(root.path(), &changed);
        assert!(
            matches!(&result, DeferralExecutionV2::Refused { refusal } if refusal.condition == "request_conflict"),
            "{result:?}"
        );
        assert_eq!(owner.resolve_request(root.path(), &locator), original);
    }
}

#[test]
fn unsupported_future_transition_and_inventory_cannot_select_deferral() {
    for unknown in ["v3", "request", "record"] {
        let (root, locator) = begin(DeferralFaultBoundaryV2::Reply);
        match unknown {
            "v3" => fs::create_dir(root.path().join(".handbook/admission-governance/v3")).unwrap(),
            "request" => fs::create_dir(
                root.path()
                    .join(GOVERNANCE_STORAGE_PATH_V2)
                    .join("requests")
                    .join("future-transition"),
            )
            .unwrap(),
            _ => fs::write(
                history(root.path(), &locator, "future-transition.json"),
                b"{}",
            )
            .unwrap(),
        }
        let owner = ProjectAdmissionDeferralV2::new();
        assert!(matches!(
            owner.resolve_request(root.path(), &locator),
            DeferralExecutionV2::Uncertain { .. }
        ));
        assert!(owner
            .resolve_current(root.path(), &locator.project, &locator.request.scope, None)
            .is_err());
    }
}

#[test]
fn physical_root_replacement_and_hard_linked_history_are_unavailable() {
    let (root, locator) = begin(DeferralFaultBoundaryV2::Reply);
    let control = tempfile::tempdir().unwrap();
    fs::hard_link(
        history(root.path(), &locator, "outcome.json"),
        control.path().join("linked-outcome"),
    )
    .unwrap();
    assert!(matches!(
        ProjectAdmissionDeferralV2::new().resolve_request(root.path(), &locator),
        DeferralExecutionV2::Uncertain { .. }
    ));
    let base = tempfile::tempdir().unwrap();
    let path = base.path().join("project");
    fs::create_dir(&path).unwrap();
    let owner = ProjectAdmissionDeferralV2::new();
    let prepared = owner.prepare(&path, &input()).unwrap();
    let locator = prepared.locator().clone();
    assert!(matches!(
        owner.initialize(&path, prepared),
        DeferralExecutionV2::Committed { .. }
    ));
    fs::rename(&path, base.path().join("original")).unwrap();
    fs::create_dir(&path).unwrap();
    assert!(matches!(
        owner.resolve_request(&path, &locator),
        DeferralExecutionV2::Uncertain { .. }
    ));
    assert!(!path.join(REPOSITORY_IDENTITY_REPO_PATH).exists());
}

#[test]
fn old_roots_and_established_missing_active_never_become_deferral() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join(".handbook")).unwrap();
    assert!(ProjectAdmissionDeferralV2::new()
        .prepare(root.path(), &input())
        .is_err());
    let established = tempfile::tempdir().unwrap();
    let owner = ProjectAdmissionGovernanceV1::new();
    let prepared = owner
        .prepare(
            established.path(),
            &GovernanceRequestInputV1::standalone(
                "initial",
                300,
                "Establish human-approved governance",
            ),
        )
        .unwrap();
    assert!(matches!(
        owner.initialize(established.path(), prepared),
        GovernanceExecutionV1::Committed { .. }
    ));
    fs::remove_file(
        established
            .path()
            .join(GOVERNANCE_STORAGE_PATH_V1)
            .join("active.json"),
    )
    .unwrap();
    assert!(ProjectAdmissionDeferralV2::new()
        .prepare(established.path(), &input())
        .is_err());
}

#[test]
fn oversized_records_and_bounded_inventory_refuse_before_recovery_effects() {
    let (root, locator) = begin(DeferralFaultBoundaryV2::Decision);
    for index in 0..33 {
        fs::write(
            history(root.path(), &locator, &format!("foreign-{index}.json")),
            b"{}",
        )
        .unwrap();
    }
    assert!(matches!(
        ProjectAdmissionDeferralV2::new().resolve_request(root.path(), &locator),
        DeferralExecutionV2::Uncertain { .. }
    ));
    assert!(!root.path().join(GOVERNANCE_CURRENT_PATH_V2).exists());
    let (root, locator) = begin(DeferralFaultBoundaryV2::Reply);
    fs::write(
        history(root.path(), &locator, "outcome.json"),
        vec![b'x'; MAX_GOVERNANCE_RECORD_BYTES_V1 + 1],
    )
    .unwrap();
    assert!(matches!(
        ProjectAdmissionDeferralV2::new().resolve_request(root.path(), &locator),
        DeferralExecutionV2::Uncertain { .. }
    ));
}

#[cfg(windows)]
#[test]
fn windows_reparse_governance_inventory_is_unavailable() {
    use std::os::windows::process::CommandExt;
    let (root, locator) = begin(DeferralFaultBoundaryV2::Reply);
    let outside = tempfile::tempdir().unwrap();
    let retained = root
        .path()
        .join(".handbook")
        .join("admission-governance")
        .join("v2");
    let moved = outside.path().join("governance");
    fs::rename(&retained, &moved).unwrap();
    let result = std::process::Command::new("cmd")
        .args(["/C", "mklink", "/J"])
        .arg(&retained)
        .arg(&moved)
        .creation_flags(0x08000000)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let owner = ProjectAdmissionDeferralV2::new();
    assert!(matches!(
        owner.resolve_request(root.path(), &locator),
        DeferralExecutionV2::Uncertain { .. }
    ));
    assert!(owner
        .resolve_current(root.path(), &locator.project, &locator.request.scope, None)
        .is_err());
}

#[cfg(unix)]
#[test]
fn unix_symlinked_history_cannot_supply_owner_truth() {
    use std::os::unix::fs::symlink;
    let (root, locator) = begin(DeferralFaultBoundaryV2::Reply);
    fs::remove_file(history(root.path(), &locator, "outcome.json")).unwrap();
    symlink(
        "stage-3.json",
        history(root.path(), &locator, "outcome.json"),
    )
    .unwrap();
    assert!(matches!(
        ProjectAdmissionDeferralV2::new().resolve_request(root.path(), &locator),
        DeferralExecutionV2::Uncertain { .. }
    ));
}
