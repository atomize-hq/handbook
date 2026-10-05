#![cfg(feature = "test-support")]
use handbook_contracts::project_admission_governance::*;
use handbook_contracts::FingerprintV1;
use sha2::{Digest, Sha256};
#[path = "fixtures/admission_reader_v2.rs"]
mod predecessor_reader;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

fn deferred() -> (
    tempfile::TempDir,
    DeferralRequestLocatorV2,
    ExpectedDeferredBasisV2,
) {
    let root = tempfile::tempdir().unwrap();
    let owner = ProjectAdmissionDeferralV2::new();
    let prepared = owner
        .prepare(
            root.path(),
            &DeferralRequestInputV2::human_approved("initial", "Approved initial deferral"),
        )
        .unwrap();
    let locator = prepared.locator().clone();
    assert!(matches!(
        owner.initialize(root.path(), prepared),
        DeferralExecutionV2::Committed { .. }
    ));
    let b = owner
        .resolve_current(
            root.path(),
            &locator.project,
            ADMISSION_GOVERNANCE_SCOPE_V1,
            None,
        )
        .unwrap();
    (
        root,
        locator,
        ExpectedDeferredBasisV2 {
            outcome_fingerprint: b.outcome_fingerprint,
            relation_fingerprint: b.relation_fingerprint,
        },
    )
}
fn request(
    original: &DeferralRequestLocatorV2,
    basis: &ExpectedDeferredBasisV2,
    key: &str,
    ceiling: u64,
) -> DeferredEstablishmentRequestV3 {
    DeferredEstablishmentRequestV3::human_approved(
        original.project.clone(),
        basis.clone(),
        ceiling,
        key,
        "Human approves exact configuration and terms",
    )
    .unwrap()
}
fn files(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn collect(root: &Path, dir: &Path, out: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for e in fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                collect(root, &p, out);
            } else {
                out.insert(
                    p.strip_prefix(root).unwrap().to_owned(),
                    fs::read(p).unwrap(),
                );
            }
        }
    }
    let mut result = BTreeMap::new();
    collect(root, root, &mut result);
    result
}
fn path(root: &Path, locator: &DeferredEstablishmentLocatorV3, name: &str) -> PathBuf {
    root.join(transition_request_storage_path_v3(locator))
        .join(name)
}
fn boundaries() -> Vec<EstablishmentFaultBoundaryV3> {
    use EstablishmentFaultBoundaryV3::*;
    [SelectionDirectory, RequestDirectory, Request, Selection]
        .into_iter()
        .chain((0..7).map(Stage))
        .chain([Intent, Decision])
        .chain((0..7).map(Installation))
        .chain([
            CurrentPrepared,
            CurrentDisplaced,
            CurrentPublished,
            Terminal,
            Closure,
            Reply,
        ])
        .collect()
}

#[test]
fn same_deferred_identity_establishes_exact_configuration_and_keeps_original_replay() {
    let root = tempfile::tempdir().unwrap();
    let d = ProjectAdmissionDeferralV2::new();
    let prepared = d
        .prepare(
            root.path(),
            &DeferralRequestInputV2::human_approved("initial", "Approved deferral"),
        )
        .unwrap();
    let original = prepared.locator().clone();
    assert!(matches!(
        d.initialize(root.path(), prepared),
        DeferralExecutionV2::Committed { .. }
    ));
    let basis = d
        .resolve_current(
            root.path(),
            &original.project,
            ADMISSION_GOVERNANCE_SCOPE_V1,
            None,
        )
        .unwrap();
    let request = DeferredEstablishmentRequestV3::human_approved(
        original.project.clone(),
        ExpectedDeferredBasisV2 {
            outcome_fingerprint: basis.outcome_fingerprint.clone(),
            relation_fingerprint: basis.relation_fingerprint.clone(),
        },
        7200,
        "establish",
        "Human approves exact establishment",
    )
    .unwrap();
    let owner = ProjectAdmissionEstablishmentV3::new();
    let prepared = owner.prepare(root.path(), &request).unwrap();
    let locator = prepared.locator().clone();
    let result = owner.establish(root.path(), prepared);
    assert!(
        matches!(result, DeferredEstablishmentExecutionV3::Committed { .. }),
        "{result:?}"
    );
    let current = owner
        .resolve_current(
            root.path(),
            &original.project,
            ADMISSION_GOVERNANCE_SCOPE_V1,
            None,
        )
        .unwrap();
    assert!(matches!(
        current,
        DeferredScopeBasisV3::EstablishedFromDeferredV3(_)
    ));
    assert!(matches!(
        d.resolve_request(root.path(), &original),
        DeferralExecutionV2::Committed { .. }
    ));
    assert!(matches!(
        owner.resolve_request(root.path(), &locator),
        DeferredEstablishmentExecutionV3::Committed {
            delivery: GovernanceDeliveryV1::Replay,
            ..
        }
    ));
    assert!(d
        .resolve_current(
            root.path(),
            &original.project,
            ADMISSION_GOVERNANCE_SCOPE_V1,
            None
        )
        .is_err());
}

#[test]
fn every_error_boundary_retains_truthful_selection_and_decision_recovery() {
    use EstablishmentFaultBoundaryV3::*;
    for fault in boundaries() {
        let (root, d, b) = deferred();
        let before = files(root.path());
        let owner = ProjectAdmissionEstablishmentV3::new().with_fault_for_testing(fault);
        let p = owner
            .prepare(root.path(), &request(&d, &b, "g", 7200))
            .unwrap();
        let locator = p.locator().clone();
        assert!(
            matches!(
                owner.establish(root.path(), p),
                DeferredEstablishmentExecutionV3::Uncertain { .. }
            ),
            "{fault:?}"
        );
        let owner = ProjectAdmissionEstablishmentV3::new();
        let result = owner.resolve_request(root.path(), &locator);
        if matches!(fault, SelectionDirectory | RequestDirectory | Request) {
            assert!(
                matches!(result, DeferredEstablishmentExecutionV3::Uncertain { .. }),
                "{fault:?}: {result:?}"
            );
            assert!(owner
                .prepare(root.path(), &request(&d, &b, "new", 86400))
                .is_err());
        } else if matches!(fault, Selection | Stage(_) | Intent) {
            assert!(
                matches!(
                    result,
                    DeferredEstablishmentExecutionV3::NotCommitted {
                        delivery: GovernanceDeliveryV1::Replay,
                        ..
                    }
                ),
                "{fault:?}: {result:?}"
            );
            assert!(matches!(
                owner
                    .resolve_current(root.path(), &d.project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
                    .unwrap(),
                DeferredScopeBasisV3::DeferredV2(_)
            ));
            let p = owner
                .prepare(root.path(), &request(&d, &b, "new", 86400))
                .unwrap();
            assert!(matches!(
                owner.establish(root.path(), p),
                DeferredEstablishmentExecutionV3::Committed { .. }
            ));
        } else {
            assert!(
                matches!(
                    result,
                    DeferredEstablishmentExecutionV3::Committed {
                        delivery: GovernanceDeliveryV1::Replay,
                        ..
                    }
                ),
                "{fault:?}: {result:?}"
            );
            assert!(matches!(
                owner
                    .resolve_current(root.path(), &d.project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
                    .unwrap(),
                DeferredScopeBasisV3::EstablishedFromDeferredV3(_)
            ));
        }
        for (name, bytes) in before {
            if name != PathBuf::from(GOVERNANCE_CURRENT_PATH_V2) {
                assert_eq!(
                    fs::read(root.path().join(name)).unwrap(),
                    bytes,
                    "{fault:?}"
                );
            }
        }
        assert_eq!(
            owner.resolve_request(root.path(), &locator),
            result,
            "{fault:?}"
        );
    }
}

#[test]
fn interrupted_noncommit_closes_only_this_attempt_and_links_a_new_request() {
    for fault in [
        EstablishmentFaultBoundaryV3::NonCommitTerminal,
        EstablishmentFaultBoundaryV3::NonCommitClosure,
    ] {
        let (root, d, b) = deferred();
        let owner = ProjectAdmissionEstablishmentV3::new();
        let p = owner
            .prepare(root.path(), &request(&d, &b, "abandoned", 7200))
            .unwrap();
        let locator = p.locator().clone();
        assert!(matches!(
            ProjectAdmissionEstablishmentV3::new()
                .with_fault_for_testing(EstablishmentFaultBoundaryV3::Intent)
                .establish(root.path(), p),
            DeferredEstablishmentExecutionV3::Uncertain { .. }
        ));
        let injected = ProjectAdmissionEstablishmentV3::new()
            .with_fault_for_testing(fault)
            .resolve_request(root.path(), &locator);
        assert!(matches!(
            injected,
            DeferredEstablishmentExecutionV3::Uncertain { .. }
        ));
        let resolved = owner.resolve_request(root.path(), &locator);
        assert!(matches!(
            resolved,
            DeferredEstablishmentExecutionV3::NotCommitted { .. }
        ));
        assert_eq!(owner.resolve_request(root.path(), &locator), resolved);
        assert!(ProjectAdmissionDeferralV2::new()
            .prepare(
                root.path(),
                &DeferralRequestInputV2::human_approved("initial-again", "Approved deferral")
            )
            .is_err());
        let p = owner
            .prepare(root.path(), &request(&d, &b, "next", 86400))
            .unwrap();
        assert!(matches!(
            owner.establish(root.path(), p),
            DeferredEstablishmentExecutionV3::Committed { .. }
        ));
        assert_eq!(owner.resolve_request(root.path(), &locator), resolved);
        assert!(ProjectAdmissionDeferralV2::new()
            .resolve_current(root.path(), &d.project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
            .is_err());
    }
}

#[test]
fn terminal_replays_never_repair_current_and_expected_d_is_stale_after_g() {
    for damage in ["missing", "changed"] {
        let (root, d, b) = deferred();
        let owner = ProjectAdmissionEstablishmentV3::new();
        let p = owner
            .prepare(root.path(), &request(&d, &b, "g", 7200))
            .unwrap();
        let locator = p.locator().clone();
        let result = owner.establish(root.path(), p);
        assert!(matches!(
            result,
            DeferredEstablishmentExecutionV3::Committed { .. }
        ));
        let expected = ExpectedProjectAdmissionBasisV3::DeferredV2(b);
        assert!(owner
            .resolve_current(
                root.path(),
                &d.project,
                ADMISSION_GOVERNANCE_SCOPE_V1,
                Some(&expected)
            )
            .is_err());
        let replay = owner.resolve_request(root.path(), &locator);
        let d_replay = ProjectAdmissionDeferralV2::new().resolve_request(root.path(), &d);
        let current = root.path().join(GOVERNANCE_CURRENT_PATH_V2);
        if damage == "missing" {
            fs::remove_file(&current).unwrap();
        } else {
            fs::write(&current, b"{}").unwrap();
        }
        let before = files(root.path());
        assert_eq!(owner.resolve_request(root.path(), &locator), replay);
        assert_eq!(
            ProjectAdmissionDeferralV2::new().resolve_request(root.path(), &d),
            d_replay
        );
        assert!(owner
            .resolve_current(root.path(), &d.project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
            .is_err());
        assert_eq!(files(root.path()), before);
    }
}

#[test]
fn missing_mixed_deleted_attempt_and_closure_never_become_retry_eligibility() {
    for damage in [
        "request-directory",
        "request.json",
        "stage-0.json",
        "stage-6.json",
        "intent.json",
        "decision.json",
        "configuration.json",
        "terminal.json",
        "closure-substituted",
        "selection-substituted",
    ] {
        let (root, d, b) = deferred();
        let owner = ProjectAdmissionEstablishmentV3::new();
        let p = owner
            .prepare(root.path(), &request(&d, &b, "g", 7200))
            .unwrap();
        let locator = p.locator().clone();
        assert!(matches!(
            owner.establish(root.path(), p),
            DeferredEstablishmentExecutionV3::Committed { .. }
        ));
        if damage == "request-directory" {
            fs::remove_dir_all(
                root.path()
                    .join(transition_request_storage_path_v3(&locator)),
            )
            .unwrap();
        } else if damage == "closure-substituted" || damage == "selection-substituted" {
            let name = if damage.starts_with("closure") {
                "closure.json"
            } else {
                "selection.json"
            };
            fs::write(
                root.path()
                    .join(ESTABLISHMENT_SELECTION_PATH_V3)
                    .join("0001")
                    .join(name),
                b"{}",
            )
            .unwrap();
        } else {
            fs::remove_file(path(root.path(), &locator, damage)).unwrap();
        }
        let before = files(root.path());
        assert!(
            matches!(
                owner.resolve_request(root.path(), &locator),
                DeferredEstablishmentExecutionV3::Uncertain { .. }
            ),
            "{damage}"
        );
        assert!(owner
            .resolve_current(root.path(), &d.project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
            .is_err());
        assert!(owner
            .prepare(root.path(), &request(&d, &b, "new", 86400))
            .is_err());
        assert_eq!(files(root.path()), before, "{damage}");
    }
    // A retained noncommit selection independently binds its deleted terminal.
    let (root, d, b) = deferred();
    let owner = ProjectAdmissionEstablishmentV3::new();
    let p = owner
        .prepare(root.path(), &request(&d, &b, "abandoned", 7200))
        .unwrap();
    let locator = p.locator().clone();
    ProjectAdmissionEstablishmentV3::new()
        .with_fault_for_testing(EstablishmentFaultBoundaryV3::Intent)
        .establish(root.path(), p);
    assert!(matches!(
        owner.resolve_request(root.path(), &locator),
        DeferredEstablishmentExecutionV3::NotCommitted { .. }
    ));
    fs::remove_file(path(root.path(), &locator, "noncommit.json")).unwrap();
    assert!(matches!(
        owner.resolve_request(root.path(), &locator),
        DeferredEstablishmentExecutionV3::Uncertain { .. }
    ));
    assert!(owner
        .prepare(root.path(), &request(&d, &b, "new", 86400))
        .is_err());
}

#[test]
fn corrupted_gap_and_missing_root_lock_claim_identity_fail_closed_without_reconstruction() {
    for damage in [
        "current-prepared.json",
        "current-displaced.json",
        "decision.json",
        handbook_engine::REPOSITORY_INITIALIZATION_LOCK_PATH_V1,
        handbook_engine::REPOSITORY_INITIALIZATION_CLAIM_PATH_V1,
        handbook_engine::REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1,
        handbook_engine::REPOSITORY_IDENTITY_REPO_PATH,
        ".handbook/admission-governance.lock",
    ] {
        let (root, d, b) = deferred();
        let owner = ProjectAdmissionEstablishmentV3::new();
        let p = owner
            .prepare(root.path(), &request(&d, &b, "g", 7200))
            .unwrap();
        let locator = p.locator().clone();
        let injected = ProjectAdmissionEstablishmentV3::new()
            .with_fault_for_testing(EstablishmentFaultBoundaryV3::CurrentDisplaced)
            .establish(root.path(), p);
        assert!(matches!(
            injected,
            DeferredEstablishmentExecutionV3::Uncertain { .. }
        ));
        assert!(!root.path().join(GOVERNANCE_CURRENT_PATH_V2).exists());
        let target = if damage.starts_with(".handbook") {
            root.path().join(damage)
        } else {
            path(root.path(), &locator, damage)
        };
        fs::remove_file(&target).unwrap();
        let before = files(root.path());
        assert!(
            matches!(
                owner.resolve_request(root.path(), &locator),
                DeferredEstablishmentExecutionV3::Uncertain { .. }
            ),
            "{damage}"
        );
        assert!(owner
            .prepare(root.path(), &request(&d, &b, "new", 86400))
            .is_err());
        assert_eq!(files(root.path()), before, "{damage}");
        assert!(!target.exists());
    }
}

#[test]
fn preparation_is_read_only_and_wrong_roots_aliases_and_request_reuse_refuse() {
    let (root, d, b) = deferred();
    let owner = ProjectAdmissionEstablishmentV3::new();
    let before = files(root.path());
    let input = request(&d, &b, "g", 7200);
    let p = owner.prepare(root.path(), &input).unwrap();
    let locator = p.locator().clone();
    assert_eq!(files(root.path()), before);
    let alias = root.path().join(".");
    assert!(owner.prepare(&alias, &input).is_err());
    let other = tempfile::tempdir().unwrap();
    assert!(owner.prepare(other.path(), &input).is_err());
    assert!(matches!(
        owner.establish(root.path(), p),
        DeferredEstablishmentExecutionV3::Committed { .. }
    ));
    let changed = create_deferred_establishment_locator(request(&d, &b, "g", 86400)).unwrap();
    assert!(matches!(
        owner.resolve_request(root.path(), &changed),
        DeferredEstablishmentExecutionV3::Refused { .. }
    ));
    let target = path(root.path(), &locator, "request.json");
    let linked = root.path().join("linked-history");
    fs::hard_link(&target, &linked).unwrap();
    assert!(matches!(
        owner.resolve_request(root.path(), &locator),
        DeferredEstablishmentExecutionV3::Uncertain { .. }
    ));
    assert!(owner
        .prepare(root.path(), &request(&d, &b, "new", 86400))
        .is_err());
}

#[test]
fn unsupported_future_inventory_and_lifetime_count_bound_refuse_before_effects() {
    for future in ["v4", "too-many"] {
        let (root, d, b) = deferred();
        if future == "v4" {
            fs::create_dir(root.path().join(".handbook/admission-governance/v4")).unwrap();
        } else {
            let selection = root.path().join(ESTABLISHMENT_SELECTION_PATH_V3);
            fs::create_dir(&selection).unwrap();
            for i in 1..=MAX_ADMISSION_REQUESTS_V3 {
                fs::create_dir(selection.join(format!("{i:04}"))).unwrap();
            }
        }
        let before = files(root.path());
        let owner = ProjectAdmissionEstablishmentV3::new();
        assert!(owner
            .prepare(root.path(), &request(&d, &b, "g", 7200))
            .is_err());
        assert!(owner
            .resolve_current(root.path(), &d.project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
            .is_err());
        assert_eq!(files(root.path()), before);
    }
}

#[test]
fn frozen_predecessor_reader_refuses_v3_after_noncommit_and_commit() {
    for noncommit in [true, false] {
        let (root, d, b) = deferred();
        let retained = handbook_engine::RetainedRepositoryRootV1::open_exact(root.path()).unwrap();
        predecessor_reader::predecessor_inventory_gate(&retained).unwrap();
        let owner = ProjectAdmissionEstablishmentV3::new();
        let p = owner
            .prepare(root.path(), &request(&d, &b, "g", 7200))
            .unwrap();
        let locator = p.locator().clone();
        if noncommit {
            ProjectAdmissionEstablishmentV3::new()
                .with_fault_for_testing(EstablishmentFaultBoundaryV3::Intent)
                .establish(root.path(), p);
            assert!(matches!(
                owner.resolve_request(root.path(), &locator),
                DeferredEstablishmentExecutionV3::NotCommitted { .. }
            ));
        } else {
            assert!(matches!(
                owner.establish(root.path(), p),
                DeferredEstablishmentExecutionV3::Committed { .. }
            ));
        }
        assert!(predecessor_reader::predecessor_inventory_gate(&retained).is_err());
        assert!(ProjectAdmissionDeferralV2::new()
            .resolve_current(root.path(), &d.project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
            .is_err());
        assert!(matches!(
            ProjectAdmissionDeferralV2::new().resolve_request(root.path(), &d),
            DeferralExecutionV2::Committed { .. }
        ));
    }
}

#[test]
fn versioned_outcome_and_current_wire_are_exact_and_do_not_assert_native_delivery() {
    let (root, d, b) = deferred();
    let owner = ProjectAdmissionEstablishmentV3::new();
    let p = owner
        .prepare(root.path(), &request(&d, &b, "g", 7200))
        .unwrap();
    let outcome = owner.establish(root.path(), p);
    let bytes = outcome.canonical_json_bytes().unwrap();
    assert_eq!(
        DeferredEstablishmentExecutionV3::from_json_bytes(&bytes).unwrap(),
        outcome
    );
    let mut incomplete: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    incomplete["outcome"]["validated"]["establishment_rule"]
        .as_object_mut()
        .unwrap()
        .remove("candidate_kind");
    assert!(DeferredEstablishmentExecutionV3::from_json_bytes(
        &serde_json::to_vec(&incomplete).unwrap()
    )
    .is_err());
    let DeferredScopeBasisV3::EstablishedFromDeferredV3(basis) = owner
        .resolve_current(root.path(), &d.project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
        .unwrap()
    else {
        panic!("G expected")
    };
    assert_eq!(
        EstablishedFromDeferredBasisV3::from_json_bytes(&basis.canonical_json_bytes().unwrap())
            .unwrap(),
        basis
    );
    let mut changed = basis.clone();
    changed.configuration.bindings[0].max_observation_age_seconds = 86400;
    assert!(changed.canonical_json_bytes().is_err());
    let mut changed = basis;
    changed.relation_fingerprint = d.request_fingerprint;
    assert!(changed.canonical_json_bytes().is_err());
    assert!(DeferredEstablishmentExecutionV3::from_json_bytes(b"{\"status\":\"future\"}").is_err());
    assert!(DeferredEstablishmentExecutionV3::from_json_bytes(
        &[bytes.as_slice(), b" {}"].concat()
    )
    .is_err());
}

#[test]
fn ordinary_v1_and_original_deferral_noncommit_never_enroll_as_deferred_projects() {
    for mode in ["ordinary", "v1", "noncommit"] {
        let (_root, _, b) = deferred();
        let owner = ProjectAdmissionEstablishmentV3::new();
        let other = tempfile::tempdir().unwrap();
        let project = if mode == "ordinary" {
            handbook_engine::RepositoryInvocationIdentityServiceV1::new()
                .initialize_for_setup(other.path())
                .unwrap();
            untrusted_binding_for_existing_root(other.path())
        } else if mode == "v1" {
            let v1 = ProjectAdmissionGovernanceV1::new();
            let p = v1
                .prepare(
                    other.path(),
                    &GovernanceRequestInputV1::standalone("v1", 86400, "Establish v1"),
                )
                .unwrap();
            assert!(matches!(
                v1.initialize(other.path(), p),
                GovernanceExecutionV1::Committed { .. }
            ));
            untrusted_binding_for_existing_root(other.path())
        } else {
            let def = ProjectAdmissionDeferralV2::new();
            let p = def
                .prepare(
                    other.path(),
                    &DeferralRequestInputV2::human_approved("no-d", "Approved deferral"),
                )
                .unwrap();
            let l = p.locator().clone();
            ProjectAdmissionDeferralV2::new()
                .with_fault_for_testing(DeferralFaultBoundaryV2::Intent)
                .initialize(other.path(), p);
            assert!(matches!(
                def.resolve_request(other.path(), &l),
                DeferralExecutionV2::NotCommitted { .. }
            ));
            l.project
        };
        let r = DeferredEstablishmentRequestV3::human_approved(
            project,
            b,
            7200,
            "g",
            "Human approves exact configuration and terms",
        )
        .unwrap();
        let before = files(other.path());
        let refusal = owner.prepare(other.path(), &r).unwrap_err();
        assert_eq!(
            refusal.condition,
            if mode == "noncommit" {
                "deferred_predecessor_unavailable"
            } else {
                "anchor_missing"
            },
            "{mode}: {refusal:?}"
        );
        assert_eq!(files(other.path()), before);
    }
}

// An internally consistent caller binding reaches native eligibility checks;
// constructing it cannot turn ordinary setup or a v1 root into a v2 registration.
fn untrusted_binding_for_existing_root(root: &Path) -> ProjectBindingV2 {
    let retained = handbook_engine::RetainedRepositoryRootV1::open_exact(root).unwrap();
    let identity = String::from_utf8(
        fs::read(root.join(handbook_engine::REPOSITORY_IDENTITY_REPO_PATH)).unwrap(),
    )
    .unwrap();
    let physical = retained.governance_physical_root_fingerprint_v1().unwrap();
    let root = root.to_string_lossy().into_owned();
    let bytes = serde_json_canonicalizer::to_vec(&serde_json::json!({
        "domain": "handbook.admission-registration@2.0.0",
        "content": [identity, root, physical],
    }))
    .unwrap();
    ProjectBindingV2 {
        registration_identity: FingerprintV1::parse(&format!("sha256:{:x}", Sha256::digest(bytes)))
            .unwrap(),
        repository_identity_fingerprint: identity,
        retained_root: root,
        physical_root_fingerprint: physical,
    }
}

#[test]
fn substituted_original_lock_anchor_identity_claim_and_physical_root_cannot_establish() {
    for damage in [
        ".handbook/admission-governance.lock",
        handbook_engine::REPOSITORY_INITIALIZATION_LOCK_PATH_V1,
        GOVERNANCE_ANCHOR_PATH_V2,
        handbook_engine::REPOSITORY_IDENTITY_REPO_PATH,
        handbook_engine::REPOSITORY_INITIALIZATION_CLAIM_PATH_V1,
        handbook_engine::REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1,
    ] {
        let (root, d, b) = deferred();
        let owner = ProjectAdmissionEstablishmentV3::new();
        let p = owner
            .prepare(root.path(), &request(&d, &b, "g", 7200))
            .unwrap();
        let target = root.path().join(damage);
        fs::rename(&target, target.with_extension("preserved-original")).unwrap();
        fs::write(
            &target,
            if damage.ends_with(".lock") {
                b"".as_slice()
            } else {
                b"{}".as_slice()
            },
        )
        .unwrap();
        let before = files(root.path());
        assert!(
            matches!(
                owner.establish(root.path(), p),
                DeferredEstablishmentExecutionV3::Refused { .. }
            ),
            "{damage}"
        );
        assert_eq!(files(root.path()), before, "{damage}");
    }
    let base = tempfile::tempdir().unwrap();
    let root = base.path().join("project");
    fs::create_dir(&root).unwrap();
    let def = ProjectAdmissionDeferralV2::new();
    let p = def
        .prepare(
            &root,
            &DeferralRequestInputV2::human_approved("d", "Approved deferral"),
        )
        .unwrap();
    let d = p.locator().clone();
    def.initialize(&root, p);
    let b = def
        .resolve_current(&root, &d.project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
        .unwrap();
    let expected = ExpectedDeferredBasisV2 {
        outcome_fingerprint: b.outcome_fingerprint,
        relation_fingerprint: b.relation_fingerprint,
    };
    let owner = ProjectAdmissionEstablishmentV3::new();
    let p = owner
        .prepare(&root, &request(&d, &expected, "g", 7200))
        .unwrap();
    fs::rename(&root, base.path().join("retained-original")).unwrap();
    fs::create_dir(&root).unwrap();
    assert!(matches!(
        owner.establish(&root, p),
        DeferredEstablishmentExecutionV3::Refused { .. }
    ));
    assert_eq!(fs::read_dir(&root).unwrap().count(), 0);
}

#[cfg(windows)]
#[test]
fn windows_reparse_v3_history_never_becomes_selected_authority() {
    use std::os::windows::process::CommandExt;
    let (root, d, b) = deferred();
    let owner = ProjectAdmissionEstablishmentV3::new();
    let p = owner
        .prepare(root.path(), &request(&d, &b, "g", 7200))
        .unwrap();
    let locator = p.locator().clone();
    owner.establish(root.path(), p);
    let outside = tempfile::tempdir().unwrap();
    let retained = root.path().join(ESTABLISHMENT_SELECTION_PATH_V3);
    let moved = outside.path().join("selected-history");
    fs::rename(&retained, &moved).unwrap();
    let result = std::process::Command::new("cmd")
        .args(["/C", "mklink", "/J"])
        .arg(retained.to_string_lossy().replace('/', "\\"))
        .arg(moved.to_string_lossy().replace('/', "\\"))
        .creation_flags(0x08000000)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(matches!(
        owner.resolve_request(root.path(), &locator),
        DeferredEstablishmentExecutionV3::Uncertain { .. }
    ));
    assert!(owner
        .prepare(root.path(), &request(&d, &b, "next", 86400))
        .is_err());
    fs::remove_dir(&retained).unwrap();
}
