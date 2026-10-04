use handbook_sdk::admission_governance::*;
use handbook_sdk::{HandbookSdkV1, SetupMode, SetupRequest};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn input(key: &str) -> DeferralRequestInputV2 {
    DeferralRequestInputV2::human_approved(key, "The human approved initiating this project and deferring the Contracts freshness admission scope.")
}

fn committed(result: DeferralExecutionV2) -> DeferralCommittedOutcomeV2 {
    match result {
        DeferralExecutionV2::Committed { outcome, .. } => outcome,
        other => panic!("expected exact deferral commit: {other:?}"),
    }
}

fn retained_files(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn collect(root: &Path, path: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
        if path.is_dir() {
            for entry in fs::read_dir(path).unwrap() {
                collect(root, &entry.unwrap().path(), out);
            }
        } else {
            out.insert(
                path.strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/"),
                fs::read(path).unwrap(),
            );
        }
    }
    let mut out = BTreeMap::new();
    for entry in fs::read_dir(root).unwrap() {
        let entry = entry.unwrap();
        if entry
            .file_name()
            .to_string_lossy()
            .starts_with(".handbook-")
        {
            collect(root, &entry.path(), &mut out);
        }
    }
    if root.join(".handbook").exists() {
        for entry in fs::read_dir(root.join(".handbook")).unwrap() {
            let entry = entry.unwrap();
            if entry
                .file_name()
                .to_string_lossy()
                .starts_with("admission-governance")
                || entry.file_name() == "repository-identity.v1"
            {
                collect(root, &entry.path(), &mut out);
            }
        }
    }
    out
}

#[test]
fn approved_deferral_reopens_with_exact_owner_basis_and_original_replay() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let prepared = sdk
        .prepare_project_admission_deferral(&input("defer-once"))
        .unwrap();
    let serialized = prepared.locator().canonical_json_bytes().unwrap();
    let locator = DeferralRequestLocatorV2::from_json_bytes(&serialized).unwrap();
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
    let original = committed(sdk.initialize_project_admission_deferral(prepared));
    assert_eq!(original.locator, locator);
    assert!(
        original.validation_confirmed
            && original.advisory_permission_accepted
            && original.deferral_committed
    );
    let json = serde_json::to_value(&original).unwrap();
    assert!(json.get("configuration").is_none());
    assert!(json.get("qualification").is_none());
    drop(sdk);
    let reopened = HandbookSdkV1::open(root.path());
    let binding = ProjectAdmissionBindingV2::ScopedChoiceV2(locator.project.clone());
    let basis = reopened
        .resolve_project_admission_scope(&binding, ADMISSION_GOVERNANCE_SCOPE_V1, None)
        .unwrap();
    let owner = handbook_contracts::project_admission_governance::ProjectAdmissionDeferralV2::new()
        .resolve_current(
            root.path(),
            &locator.project,
            ADMISSION_GOVERNANCE_SCOPE_V1,
            None,
        )
        .unwrap();
    assert_eq!(basis, ProjectAdmissionScopeBasisV2::Deferred(owner.clone()));
    assert!(owner.never_established);
    assert_eq!(owner.outcome_fingerprint, original.outcome_fingerprint);
    assert_eq!(
        owner.establishment_rule,
        original.validated.establishment_rule
    );
    let before = retained_files(root.path());
    let replay = reopened.resolve_admission_deferral_request(&locator);
    assert!(matches!(
        replay,
        DeferralExecutionV2::Committed {
            delivery: GovernanceDeliveryV1::Replay,
            ..
        }
    ));
    assert_eq!(committed(replay), original);
    assert_eq!(retained_files(root.path()), before);
    assert!(reopened
        .resolve_project_admission_scope(
            &binding,
            "contracts.evidence-freshness-policy/other",
            None
        )
        .is_err());
}

#[test]
fn actual_refresh_reset_retains_claim_locks_history_and_deferral() {
    let root = tempdir().unwrap();
    fs::create_dir(root.path().join(".git")).unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let prepared = sdk
        .prepare_project_admission_deferral(&input("reset-deferred"))
        .unwrap();
    let locator = prepared.locator().clone();
    let original = committed(sdk.initialize_project_admission_deferral(prepared));
    let before = retained_files(root.path());
    let state = root.path().join(".handbook/state/temporary");
    fs::create_dir_all(&state).unwrap();
    fs::write(state.join("runtime.json"), b"{}\n").unwrap();
    let refresh = sdk
        .run_setup(&SetupRequest {
            mode: SetupMode::Refresh,
            rewrite: false,
            reset_state: true,
        })
        .unwrap();
    assert!(refresh.reset_applied);
    assert!(!state.exists());
    assert_eq!(retained_files(root.path()), before);
    let reopened = HandbookSdkV1::open(root.path());
    assert_eq!(
        committed(reopened.resolve_admission_deferral_request(&locator)),
        original
    );
    assert!(matches!(
        reopened
            .resolve_project_admission_scope(
                &ProjectAdmissionBindingV2::ScopedChoiceV2(locator.project),
                ADMISSION_GOVERNANCE_SCOPE_V1,
                None
            )
            .unwrap(),
        ProjectAdmissionScopeBasisV2::Deferred(_)
    ));
}

#[test]
fn lost_consumer_reply_restores_only_original_request_resolution() {
    struct Disconnected;
    impl std::io::Write for Disconnected {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("consumer serialization unavailable"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let prepared = sdk
        .prepare_project_admission_deferral(&input("lost-reply"))
        .unwrap();
    let serialized = prepared.locator().canonical_json_bytes().unwrap();
    let original = sdk.initialize_project_admission_deferral(prepared);
    assert!(matches!(original, DeferralExecutionV2::Committed { .. }));
    assert!(serde_json::to_writer(Disconnected, &original).is_err());
    let outcome = committed(original);
    drop(sdk);
    let restored = DeferralRequestLocatorV2::from_json_bytes(&serialized).unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let before = retained_files(root.path());
    assert_eq!(
        committed(sdk.resolve_admission_deferral_request(&restored)),
        outcome
    );
    assert_eq!(retained_files(root.path()), before);
    assert!(sdk
        .prepare_project_admission_deferral(&input("retry"))
        .is_err());
    assert!(sdk
        .prepare_project_admission_governance(&GovernanceRequestInputV1::standalone_freshness(
            300,
            "new-governance",
            "Old deferral does not approve governance terms"
        ))
        .is_err());
}

#[test]
fn owner_outcomes_refusals_and_advisory_claims_are_preserved_by_sdk() {
    use handbook_contracts::project_admission_governance::{
        DeferralFaultBoundaryV2, ProjectAdmissionDeferralV2,
    };
    for (boundary, expected_commit) in [
        (DeferralFaultBoundaryV2::Claim, false),
        (DeferralFaultBoundaryV2::Decision, true),
    ] {
        let root = tempdir().unwrap();
        let sdk = HandbookSdkV1::open(root.path());
        let mut request = input("recover-original");
        request.instruction.asserted_principal = Some("asserted-human".into());
        request.instruction.asserted_agent = Some("configured-agent".into());
        request.instruction.asserted_delegation = Some("advisory claim only".into());
        let prepared = sdk.prepare_project_admission_deferral(&request).unwrap();
        let locator = prepared.locator().clone();
        let interrupted = ProjectAdmissionDeferralV2::new()
            .with_fault_for_testing(boundary)
            .initialize(root.path(), prepared);
        assert!(matches!(interrupted, DeferralExecutionV2::Uncertain { .. }));
        let wire = serde_json::to_vec(&interrupted).unwrap();
        assert_eq!(
            serde_json::from_slice::<DeferralExecutionV2>(&wire).unwrap(),
            interrupted
        );
        drop(sdk);
        let sdk = HandbookSdkV1::open(root.path());
        let resolved = sdk.resolve_admission_deferral_request(&locator);
        assert_eq!(
            resolved,
            ProjectAdmissionDeferralV2::new().resolve_request(root.path(), &locator)
        );
        if expected_commit {
            assert_eq!(
                committed(resolved).validated.input.instruction,
                request.instruction
            );
        } else {
            assert!(matches!(resolved, DeferralExecutionV2::NotCommitted { .. }));
            assert!(!root.path().join(".handbook").exists());
        }
        let before = retained_files(root.path());
        assert!(sdk
            .run_setup(&SetupRequest {
                mode: SetupMode::Init,
                rewrite: false,
                reset_state: false
            })
            .is_err());
        if !expected_commit {
            assert!(
                handbook_engine::RepositoryInvocationIdentityServiceV1::new()
                    .initialize_for_setup(root.path())
                    .is_err()
            );
        }
        assert_eq!(retained_files(root.path()), before);
        let mut changed = locator;
        changed.request.request_key = "changed-key".into();
        assert_eq!(
            sdk.resolve_admission_deferral_request(&changed),
            ProjectAdmissionDeferralV2::new().resolve_request(root.path(), &changed)
        );
        assert!(matches!(
            sdk.resolve_admission_deferral_request(&changed),
            DeferralExecutionV2::Refused { .. }
        ));
    }
}

#[test]
fn retained_noncommit_witness_freezes_initialization_after_partial_claim_loss() {
    use handbook_contracts::project_admission_governance::{
        DeferralFaultBoundaryV2, ProjectAdmissionDeferralV2,
    };
    use handbook_engine::{
        RepositoryInvocationIdentityServiceV1, REPOSITORY_INITIALIZATION_CLAIM_PATH_V1,
        REPOSITORY_INITIALIZATION_LOCK_PATH_V1, REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1,
    };
    for remove_lock in [false, true] {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join(".git")).unwrap();
        let sdk = HandbookSdkV1::open(root.path());
        let establishment = GovernanceRequestInputV1::standalone_freshness(
            300,
            "competing-establishment",
            "Approved initial establishment terms",
        );
        let old_establishment = sdk
            .prepare_project_admission_governance(&establishment)
            .unwrap();
        let prepared = sdk
            .prepare_project_admission_deferral(&input("retained-noncommit"))
            .unwrap();
        let locator = prepared.locator().clone();
        assert!(matches!(
            ProjectAdmissionDeferralV2::new()
                .with_fault_for_testing(DeferralFaultBoundaryV2::Claim)
                .initialize(root.path(), prepared),
            DeferralExecutionV2::Uncertain { .. }
        ));
        assert!(matches!(
            sdk.resolve_admission_deferral_request(&locator),
            DeferralExecutionV2::NotCommitted { .. }
        ));
        fs::remove_file(root.path().join(REPOSITORY_INITIALIZATION_CLAIM_PATH_V1)).unwrap();
        fs::remove_file(root.path().join(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1)).unwrap();
        if remove_lock {
            fs::remove_file(root.path().join(REPOSITORY_INITIALIZATION_LOCK_PATH_V1)).unwrap();
        }
        let before = retained_files(root.path());
        let reopened = HandbookSdkV1::open(root.path());
        assert!(reopened
            .prepare_project_admission_governance(&establishment)
            .is_err());
        assert!(reopened
            .prepare_project_admission_deferral(&input("fresh-again"))
            .is_err());
        assert!(matches!(
            reopened.initialize_project_admission_governance(old_establishment),
            GovernanceExecutionV1::Refused { .. }
        ));
        assert!(reopened
            .run_setup(&SetupRequest {
                mode: SetupMode::Init,
                rewrite: false,
                reset_state: false,
            })
            .is_err());
        assert!(RepositoryInvocationIdentityServiceV1::new()
            .initialize_for_setup(root.path())
            .is_err());
        assert!(matches!(
            reopened.resolve_admission_deferral_request(&locator),
            DeferralExecutionV2::Uncertain { .. }
        ));
        assert!(!root.path().join(".handbook").exists());
        assert!(!root
            .path()
            .join(REPOSITORY_INITIALIZATION_CLAIM_PATH_V1)
            .exists());
        assert!(!root
            .path()
            .join(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1)
            .exists());
        assert_eq!(retained_files(root.path()), before);
    }
}

#[test]
fn exact_expected_basis_and_explicit_version_tags_cannot_expand_scope() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let prepared = sdk
        .prepare_project_admission_deferral(&input("exact-basis"))
        .unwrap();
    let locator = prepared.locator().clone();
    let outcome = committed(sdk.initialize_project_admission_deferral(prepared));
    let binding = ProjectAdmissionBindingV2::ScopedChoiceV2(locator.project.clone());
    let basis = sdk
        .resolve_project_admission_scope(&binding, &locator.request.scope, None)
        .unwrap();
    let ProjectAdmissionScopeBasisV2::Deferred(deferred) = &basis else {
        panic!("deferred basis")
    };
    let mut expected = ExpectedProjectAdmissionBasisV2::ScopedChoiceV2(ExpectedDeferredBasisV2 {
        outcome_fingerprint: outcome.outcome_fingerprint,
        relation_fingerprint: deferred.relation_fingerprint.clone(),
    });
    assert_eq!(
        sdk.resolve_project_admission_scope(&binding, &locator.request.scope, Some(&expected))
            .unwrap(),
        basis
    );
    if let ExpectedProjectAdmissionBasisV2::ScopedChoiceV2(inner) = &mut expected {
        inner.relation_fingerprint =
            FingerprintV1::parse(&format!("sha256:{}", "0".repeat(64))).unwrap();
    }
    assert!(sdk
        .resolve_project_admission_scope(&binding, &locator.request.scope, Some(&expected))
        .is_err());
    let wrong_tag = ExpectedProjectAdmissionBasisV2::EstablishedV1(ExpectedGovernanceBasisV1 {
        configuration_fingerprint: deferred.deferral_fingerprint.clone(),
        outcome_fingerprint: deferred.outcome_fingerprint.clone(),
    });
    assert!(sdk
        .resolve_project_admission_scope(&binding, &locator.request.scope, Some(&wrong_tag))
        .is_err());
    let wire = binding.canonical_json_bytes().unwrap();
    assert_eq!(
        ProjectAdmissionBindingV2::from_json_bytes(&wire).unwrap(),
        binding
    );
    let mut wrong = serde_json::to_value(binding).unwrap();
    wrong["project_contract"] = serde_json::json!("scoped_choice_v3");
    assert!(
        ProjectAdmissionBindingV2::from_json_bytes(&serde_json::to_vec(&wrong).unwrap()).is_err()
    );
    assert!(sdk
        .resolve_project_admission_scope(
            &ProjectAdmissionBindingV2::ScopedChoiceV2(locator.project),
            "contracts.other-policy/root",
            None
        )
        .is_err());
}

#[test]
fn explicit_v1_resolution_still_returns_owner_establishment() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let prepared = sdk
        .prepare_project_admission_governance(&GovernanceRequestInputV1::standalone_freshness(
            7200,
            "establish-v1",
            "Human approves exact v1 governance",
        ))
        .unwrap();
    let locator = prepared.locator().clone();
    assert!(matches!(
        sdk.initialize_project_admission_governance(prepared),
        GovernanceExecutionV1::Committed { .. }
    ));
    let expected_owner = sdk
        .resolve_project_admission_governance(&locator.project, &locator.request.scope, None)
        .unwrap();
    assert_eq!(
        sdk.resolve_project_admission_scope(
            &ProjectAdmissionBindingV2::EstablishedV1(locator.project),
            &locator.request.scope,
            None
        )
        .unwrap(),
        ProjectAdmissionScopeBasisV2::Established(expected_owner)
    );
}

fn wait_file(path: &Path) {
    let started = std::time::Instant::now();
    while !path.exists() {
        assert!(
            started.elapsed() < std::time::Duration::from_secs(20),
            "timed out waiting for {path:?}"
        );
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

#[test]
fn sdk_deferral_process_worker() {
    let Ok(mode) = std::env::var("HCM_SCOPED_SDK_MODE") else {
        return;
    };
    let root = std::path::PathBuf::from(std::env::var_os("HCM_SCOPED_SDK_ROOT").unwrap());
    let control = std::path::PathBuf::from(std::env::var_os("HCM_SCOPED_SDK_CONTROL").unwrap());
    let sdk = HandbookSdkV1::open(&root);
    let deferral = if matches!(mode.as_str(), "winner" | "defer") {
        Some(
            sdk.prepare_project_admission_deferral(&input(&mode))
                .unwrap(),
        )
    } else {
        None
    };
    let establishment = if mode == "establish" {
        Some(
            sdk.prepare_project_admission_governance(
                &GovernanceRequestInputV1::standalone_freshness(
                    300,
                    "competing-v1",
                    "The human approved establishing these exact terms",
                ),
            )
            .unwrap(),
        )
    } else {
        None
    };
    fs::write(control.join(format!("{mode}.ready")), b"ready").unwrap();
    wait_file(&control.join(format!("{mode}.go")));
    fs::write(control.join(format!("{mode}.attempt")), b"attempt").unwrap();
    let result = match mode.as_str() {
        "winner" => {
            use handbook_contracts::project_admission_governance::{
                DeferralFaultBoundaryV2, ProjectAdmissionDeferralV2,
            };
            let boundary = match std::env::var("HCM_SCOPED_SDK_BOUNDARY").unwrap().as_str() {
                "claim" => DeferralFaultBoundaryV2::Claim,
                "reservation" => DeferralFaultBoundaryV2::Reservation,
                "anchor" => DeferralFaultBoundaryV2::Anchor,
                other => panic!("unsupported test boundary {other}"),
            };
            let prepared = deferral.unwrap();
            fs::write(
                control.join("winner.locator.json"),
                prepared.locator().canonical_json_bytes().unwrap(),
            )
            .unwrap();
            serde_json::to_value(
                ProjectAdmissionDeferralV2::new()
                    .with_pause_for_testing(
                        boundary,
                        control.join("winner.paused"),
                        control.join("winner.release"),
                    )
                    .initialize(&root, prepared),
            )
            .unwrap()
        }
        "defer" => {
            serde_json::to_value(sdk.initialize_project_admission_deferral(deferral.unwrap()))
                .unwrap()
        }
        "establish" => serde_json::to_value(
            sdk.initialize_project_admission_governance(establishment.unwrap()),
        )
        .unwrap(),
        "init" | "refresh" => {
            let result = sdk.run_setup(&SetupRequest {
                mode: if mode == "init" {
                    SetupMode::Init
                } else {
                    SetupMode::Refresh
                },
                rewrite: false,
                reset_state: false,
            });
            serde_json::json!({"ok":result.is_ok()})
        }
        "identity" => {
            let result = handbook_engine::RepositoryInvocationIdentityServiceV1::new()
                .initialize_for_setup(&root);
            match result {
                Ok(identity) => {
                    serde_json::json!({"ok":true,"identity":identity.repository_identity_fingerprint()})
                }
                Err(error) => serde_json::json!({"ok":false,"error":error.to_string()}),
            }
        }
        other => panic!("unsupported test mode {other}"),
    };
    fs::write(
        control.join(format!("{mode}.result.json")),
        serde_json::to_vec(&result).unwrap(),
    )
    .unwrap();
}

fn worker(root: &Path, control: &Path, mode: &str, boundary: &str) -> std::process::Child {
    std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "sdk_deferral_process_worker",
            "--nocapture",
            "--test-threads=1",
        ])
        .env("HCM_SCOPED_SDK_MODE", mode)
        .env("HCM_SCOPED_SDK_ROOT", root)
        .env("HCM_SCOPED_SDK_CONTROL", control)
        .env("HCM_SCOPED_SDK_BOUNDARY", boundary)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap()
}

#[test]
fn native_deferral_serializes_sdk_init_refresh_establishment_and_direct_identity() {
    for boundary in ["claim", "reservation", "anchor"] {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join(".git")).unwrap();
        let control = tempdir().unwrap();
        let modes = [
            "winner",
            "defer",
            "establish",
            "init",
            "refresh",
            "identity",
        ];
        let mut children = modes.map(|mode| worker(root.path(), control.path(), mode, boundary));
        for mode in modes {
            wait_file(&control.path().join(format!("{mode}.ready")));
        }
        fs::write(control.path().join("winner.go"), b"go").unwrap();
        wait_file(&control.path().join("winner.paused"));
        for mode in &modes[1..] {
            fs::write(control.path().join(format!("{mode}.go")), b"go").unwrap();
        }
        for mode in &modes[1..] {
            wait_file(&control.path().join(format!("{mode}.attempt")));
        }
        // Every participant has crossed its caller rendezvous into the guarded
        // operation while the native owner retains the root lock.
        std::thread::sleep(std::time::Duration::from_millis(200));
        for mode in modes {
            assert!(
                !control.path().join(format!("{mode}.result.json")).exists(),
                "{boundary}: {mode} bypassed the held guard"
            );
        }
        fs::write(control.path().join("winner.release"), b"release").unwrap();
        for (mode, child) in modes.iter().zip(&mut children) {
            wait_file(&control.path().join(format!("{mode}.result.json")));
            let started = std::time::Instant::now();
            loop {
                if let Some(status) = child.try_wait().unwrap() {
                    assert!(status.success(), "{mode}: {status}");
                    break;
                }
                assert!(started.elapsed() < std::time::Duration::from_secs(20));
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
        }
        let result = |mode: &str| -> serde_json::Value {
            serde_json::from_slice(
                &fs::read(control.path().join(format!("{mode}.result.json"))).unwrap(),
            )
            .unwrap()
        };
        assert_eq!(result("winner")["status"], "committed");
        for mode in ["defer", "establish"] {
            assert_eq!(result(mode)["status"], "refused");
        }
        assert_eq!(result("init")["ok"], false);
        assert_eq!(result("refresh")["ok"], true);
        assert_eq!(result("identity")["ok"], true);
        let locator = DeferralRequestLocatorV2::from_json_bytes(
            &fs::read(control.path().join("winner.locator.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(
            result("identity")["identity"],
            locator.project.repository_identity_fingerprint
        );
        let sdk = HandbookSdkV1::open(root.path());
        assert_eq!(
            committed(sdk.resolve_admission_deferral_request(&locator)),
            committed(serde_json::from_value(result("winner")).unwrap())
        );
        assert!(matches!(
            sdk.resolve_project_admission_scope(
                &ProjectAdmissionBindingV2::ScopedChoiceV2(locator.project),
                ADMISSION_GOVERNANCE_SCOPE_V1,
                None
            )
            .unwrap(),
            ProjectAdmissionScopeBasisV2::Deferred(_)
        ));
    }
}
