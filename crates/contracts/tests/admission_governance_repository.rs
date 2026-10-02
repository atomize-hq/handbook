#![cfg(feature = "test-support")]
use handbook_contracts::project_admission_governance::*;
use std::fs;

fn request(ceiling: u64) -> GovernanceRequestInputV1 {
    GovernanceRequestInputV1::standalone_freshness(
        ceiling,
        "initial",
        "Establish exact governance under local advisory instruction",
    )
}
fn begin(fault: GovernanceFaultBoundaryV1) -> (tempfile::TempDir, GovernanceRequestLocatorV1) {
    let root = tempfile::tempdir().unwrap();
    let owner = ProjectAdmissionGovernanceV1::new().with_fault_for_testing(fault);
    let prepared = owner.prepare(root.path(), &request(7200)).unwrap();
    let locator = prepared.locator().clone();
    assert!(matches!(
        owner.initialize(root.path(), prepared),
        GovernanceExecutionV1::Uncertain { .. }
    ));
    (root, locator)
}
fn storage(root: &std::path::Path, name: &str) -> std::path::PathBuf {
    root.join(GOVERNANCE_STORAGE_PATH_V1).join(name)
}

#[test]
fn every_durable_boundary_recovers_only_the_original_decision() {
    let predecision = [
        GovernanceFaultBoundaryV1::Anchor,
        GovernanceFaultBoundaryV1::Identity,
        GovernanceFaultBoundaryV1::Registration,
        GovernanceFaultBoundaryV1::Stage(0),
        GovernanceFaultBoundaryV1::Stage(1),
        GovernanceFaultBoundaryV1::Stage(2),
        GovernanceFaultBoundaryV1::Stage(3),
        GovernanceFaultBoundaryV1::Stage(4),
        GovernanceFaultBoundaryV1::Intent,
    ];
    let committed = [
        GovernanceFaultBoundaryV1::Decision,
        GovernanceFaultBoundaryV1::Installation(0),
        GovernanceFaultBoundaryV1::Installation(1),
        GovernanceFaultBoundaryV1::Installation(2),
        GovernanceFaultBoundaryV1::Installation(3),
        GovernanceFaultBoundaryV1::Installation(4),
        GovernanceFaultBoundaryV1::Terminal,
        GovernanceFaultBoundaryV1::Reply,
    ];
    for fault in predecision {
        let (root, locator) = begin(fault);
        let owner = ProjectAdmissionGovernanceV1::new();
        let resolved = owner.resolve_request(root.path(), &locator);
        assert!(
            matches!(&resolved,GovernanceExecutionV1::NotCommitted{outcome,..} if !outcome.activation_committed),
            "{fault:?}: {resolved:?}"
        );
        assert_eq!(resolved, owner.resolve_request(root.path(), &locator));
        assert!(owner
            .resolve_active(root.path(), &locator.project, GOVERNANCE_SCOPE_V1, None)
            .is_err());
        assert!(owner.prepare(root.path(), &request(7200)).is_err());
    }
    for fault in committed {
        let (root, locator) = begin(fault);
        let owner = ProjectAdmissionGovernanceV1::new();
        let resolved = owner.resolve_request(root.path(), &locator);
        assert!(
            matches!(
                &resolved,
                GovernanceExecutionV1::Committed {
                    delivery: GovernanceDeliveryV1::Replay,
                    ..
                }
            ),
            "{fault:?}: {resolved:?}"
        );
        assert_eq!(resolved, owner.resolve_request(root.path(), &locator));
        assert_eq!(
            owner
                .resolve_active(root.path(), &locator.project, GOVERNANCE_SCOPE_V1, None)
                .unwrap()
                .configuration
                .bindings[0]
                .max_observation_age_seconds,
            7200
        );
        assert!(owner.prepare(root.path(), &request(7200)).is_err());
    }
    let (root, locator) = begin(GovernanceFaultBoundaryV1::Reservation);
    assert!(matches!(
        ProjectAdmissionGovernanceV1::new().resolve_request(root.path(), &locator),
        GovernanceExecutionV1::Uncertain { .. }
    ));
    assert!(ProjectAdmissionGovernanceV1::new()
        .prepare(root.path(), &request(7200))
        .is_err());
}

#[test]
fn missing_corrupt_and_nonprefix_evidence_never_reconstructs_authority() {
    for name in [
        "stage-0.json",
        "intent.json",
        "decision.json",
        "rules.json",
        "configuration.json",
        "outcome.json",
        "consumed.json",
        "active.json",
    ] {
        let (root, locator) = begin(GovernanceFaultBoundaryV1::Terminal);
        let target = storage(root.path(), name);
        fs::remove_file(&target).unwrap();
        let owner = ProjectAdmissionGovernanceV1::new();
        assert!(
            matches!(
                owner.resolve_request(root.path(), &locator),
                GovernanceExecutionV1::Uncertain { .. }
            ),
            "missing {name}"
        );
        assert!(!target.exists(), "missing {name} was reconstructed");
        assert!(owner
            .resolve_active(root.path(), &locator.project, GOVERNANCE_SCOPE_V1, None)
            .is_err());
    }
    for name in [
        "registration.json",
        "stage-0.json",
        "intent.json",
        "decision.json",
        "terminal.json",
        "configuration.json",
        "active.json",
    ] {
        let (root, locator) = begin(GovernanceFaultBoundaryV1::Terminal);
        fs::write(storage(root.path(), name), b"{}").unwrap();
        assert!(
            matches!(
                ProjectAdmissionGovernanceV1::new().resolve_request(root.path(), &locator),
                GovernanceExecutionV1::Uncertain { .. }
            ),
            "corrupt {name}"
        );
    }
    let (root, locator) = begin(GovernanceFaultBoundaryV1::Decision);
    fs::copy(
        storage(root.path(), "stage-1.json"),
        storage(root.path(), "configuration.json"),
    )
    .unwrap();
    assert!(matches!(
        ProjectAdmissionGovernanceV1::new().resolve_request(root.path(), &locator),
        GovernanceExecutionV1::Uncertain { .. }
    ));
    assert!(!storage(root.path(), "rules.json").exists());
}

#[test]
fn incomplete_predecision_history_cannot_claim_positive_noncommit() {
    let (root, locator) = begin(GovernanceFaultBoundaryV1::Stage(4));
    fs::remove_file(storage(root.path(), "stage-1.json")).unwrap();
    assert!(matches!(
        ProjectAdmissionGovernanceV1::new().resolve_request(root.path(), &locator),
        GovernanceExecutionV1::Uncertain { .. }
    ));
    assert!(!storage(root.path(), "not-committed.json").exists());
    let (root, locator) = begin(GovernanceFaultBoundaryV1::Registration);
    fs::remove_file(root.path().join(".handbook/repository-identity.v1")).unwrap();
    assert!(matches!(
        ProjectAdmissionGovernanceV1::new().resolve_request(root.path(), &locator),
        GovernanceExecutionV1::Uncertain { .. }
    ));
}

#[test]
fn retained_anchor_identity_and_bounds_fail_closed() {
    for target in [
        GOVERNANCE_ANCHOR_PATH_V1,
        ".handbook/repository-identity.v1",
        ".handbook/admission-governance.lock",
    ] {
        let (root, locator) = begin(GovernanceFaultBoundaryV1::Terminal);
        fs::remove_file(root.path().join(target)).unwrap();
        let owner = ProjectAdmissionGovernanceV1::new();
        assert!(
            matches!(
                owner.resolve_request(root.path(), &locator),
                GovernanceExecutionV1::Uncertain { .. }
            ),
            "{target}"
        );
        assert!(owner.prepare(root.path(), &request(7200)).is_err());
        assert!(!root.path().join(target).exists());
    }
    let (root, locator) = begin(GovernanceFaultBoundaryV1::Terminal);
    fs::write(
        storage(root.path(), "configuration.json"),
        vec![b' '; 1024 * 1024 + 1],
    )
    .unwrap();
    assert!(matches!(
        ProjectAdmissionGovernanceV1::new().resolve_request(root.path(), &locator),
        GovernanceExecutionV1::Uncertain { .. }
    ));
    let (root, locator) = begin(GovernanceFaultBoundaryV1::Terminal);
    fs::write(storage(root.path(), "unknown.json"), b"{}").unwrap();
    assert!(matches!(
        ProjectAdmissionGovernanceV1::new().resolve_request(root.path(), &locator),
        GovernanceExecutionV1::Uncertain { .. }
    ));
    let (root, locator) = begin(GovernanceFaultBoundaryV1::Terminal);
    fs::create_dir(storage(root.path(), "configuration-shadow")).unwrap();
    assert!(matches!(
        ProjectAdmissionGovernanceV1::new().resolve_request(root.path(), &locator),
        GovernanceExecutionV1::Uncertain { .. }
    ));
}

#[test]
fn hard_link_alias_and_root_or_identity_substitution_are_unavailable() {
    let (root, locator) = begin(GovernanceFaultBoundaryV1::Terminal);
    let outside = tempfile::tempdir().unwrap();
    fs::hard_link(
        storage(root.path(), "configuration.json"),
        outside.path().join("alias"),
    )
    .unwrap();
    let owner = ProjectAdmissionGovernanceV1::new();
    assert!(matches!(
        owner.resolve_request(root.path(), &locator),
        GovernanceExecutionV1::Uncertain { .. }
    ));
    assert!(owner
        .resolve_active(root.path(), &locator.project, GOVERNANCE_SCOPE_V1, None)
        .is_err());
    assert!(owner
        .resolve_active(outside.path(), &locator.project, GOVERNANCE_SCOPE_V1, None)
        .is_err());
    let (root, mut locator) = begin(GovernanceFaultBoundaryV1::Terminal);
    fs::write(
        root.path().join(".handbook/repository-identity.v1"),
        format!("sha256:{}", "a".repeat(64)),
    )
    .unwrap();
    assert!(matches!(
        owner.resolve_request(root.path(), &locator),
        GovernanceExecutionV1::Uncertain { .. }
    ));
    locator.project.physical_root_fingerprint = format!("sha256:{}", "b".repeat(64));
    assert!(matches!(
        owner.resolve_request(root.path(), &locator),
        GovernanceExecutionV1::Refused { .. }
    ));
}

#[test]
fn changed_content_and_keys_are_refused_without_changing_original_history() {
    let (root, locator) = begin(GovernanceFaultBoundaryV1::Terminal);
    let owner = ProjectAdmissionGovernanceV1::new();
    let original = owner.resolve_request(root.path(), &locator);
    for changed in [request(86400), {
        let mut r = request(7200);
        r.request_key = "other".into();
        r
    }] {
        let conflict = create_request_locator(locator.project.clone(), changed).unwrap();
        assert!(matches!(
            owner.resolve_request(root.path(), &conflict),
            GovernanceExecutionV1::Refused { .. }
        ));
        assert_eq!(original, owner.resolve_request(root.path(), &locator));
    }
}

#[cfg(unix)]
#[test]
fn symlinks_and_special_inventory_entries_are_unavailable() {
    use std::os::unix::fs::symlink;
    let (root, locator) = begin(GovernanceFaultBoundaryV1::Terminal);
    fs::remove_file(storage(root.path(), "active.json")).unwrap();
    symlink("stage-4.json", storage(root.path(), "active.json")).unwrap();
    assert!(matches!(
        ProjectAdmissionGovernanceV1::new().resolve_request(root.path(), &locator),
        GovernanceExecutionV1::Uncertain { .. }
    ));
}

#[cfg(windows)]
#[test]
fn reparse_directory_inventory_is_unavailable() {
    use std::os::windows::process::CommandExt;
    let (root, locator) = begin(GovernanceFaultBoundaryV1::Terminal);
    let outside = tempfile::tempdir().unwrap();
    let retained = root
        .path()
        .join(".handbook")
        .join("admission-governance")
        .join("v1");
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
    let owner = ProjectAdmissionGovernanceV1::new();
    assert!(matches!(
        owner.resolve_request(root.path(), &locator),
        GovernanceExecutionV1::Uncertain { .. }
    ));
    assert!(owner
        .resolve_active(root.path(), &locator.project, GOVERNANCE_SCOPE_V1, None)
        .is_err());
}

#[test]
fn owned_inventory_count_bound_refuses_without_new_effects() {
    let (root, locator) = begin(GovernanceFaultBoundaryV1::Decision);
    for index in 0..33 {
        fs::write(
            storage(root.path(), &format!("foreign-{index}.json")),
            b"{}",
        )
        .unwrap();
    }
    assert!(matches!(
        ProjectAdmissionGovernanceV1::new().resolve_request(root.path(), &locator),
        GovernanceExecutionV1::Uncertain { .. }
    ));
    assert!(!storage(root.path(), "active.json").exists());
    assert!(!storage(root.path(), "terminal.json").exists());
}

#[test]
fn supplied_ceiling_and_original_outcome_survive_reopen() {
    let root = tempfile::tempdir().unwrap();
    let owner = ProjectAdmissionGovernanceV1::new();
    let request =
        GovernanceRequestInputV1::standalone_freshness(7200, "first", "Establish this project");
    let prepared = owner.prepare(root.path(), &request).unwrap();
    let locator = prepared.locator().clone();
    assert!(!root.path().join(".handbook").exists());
    let result = owner.initialize(root.path(), prepared);
    let GovernanceExecutionV1::Committed { outcome, delivery } = result else {
        panic!("{result:?}")
    };
    assert_eq!(delivery, GovernanceDeliveryV1::Original);
    assert!(
        outcome.validation_confirmed
            && outcome.advisory_permission_accepted
            && outcome.activation_committed
    );
    let reopened = ProjectAdmissionGovernanceV1::new();
    let active = reopened
        .resolve_active(root.path(), &locator.project, GOVERNANCE_SCOPE_V1, None)
        .unwrap();
    assert_eq!(
        active.configuration.bindings[0].max_observation_age_seconds,
        7200
    );
    assert_eq!(active.outcome_fingerprint, outcome.outcome_fingerprint);
    assert_eq!(
        reopened.resolve_request(root.path(), &locator),
        GovernanceExecutionV1::Committed {
            outcome,
            delivery: GovernanceDeliveryV1::Replay
        }
    );
}
