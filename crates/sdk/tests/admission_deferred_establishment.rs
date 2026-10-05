use handbook_contracts::project_admission_governance::{
    EstablishmentFaultBoundaryV3, ProjectAdmissionEstablishmentV3, GOVERNANCE_CURRENT_PATH_V2,
};
use handbook_sdk::admission_governance::*;
use handbook_sdk::{HandbookSdkV1, SetupMode, SetupRequest};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn defer(root: &Path) -> (DeferralCommittedOutcomeV2, DeferredGovernanceBasisV2) {
    let sdk = HandbookSdkV1::open(root);
    let prepared = sdk
        .prepare_project_admission_deferral(&DeferralRequestInputV2::human_approved(
            "original-deferral",
            "The human approved creating this project and deferring freshness governance.",
        ))
        .unwrap();
    let project = prepared.locator().project.clone();
    let original = committed_deferral(sdk.initialize_project_admission_deferral(prepared));
    let deferred =
        handbook_contracts::project_admission_governance::ProjectAdmissionDeferralV2::new()
            .resolve_current(root, &project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
            .unwrap();
    (original, deferred)
}

fn request(
    basis: &DeferredGovernanceBasisV2,
    ceiling: u64,
    key: &str,
) -> DeferredEstablishmentRequestV3 {
    DeferredEstablishmentRequestV3::human_approved(
        basis.project.clone(),
        ExpectedDeferredBasisV2 {
            outcome_fingerprint: basis.outcome_fingerprint.clone(),
            relation_fingerprint: basis.relation_fingerprint.clone(),
        },
        ceiling,
        key,
        "The human approved this exact configuration and standalone replacement terms.",
    )
    .unwrap()
}

fn committed(result: DeferredEstablishmentExecutionV3) -> DeferredEstablishmentCommittedOutcomeV3 {
    match result {
        DeferredEstablishmentExecutionV3::Committed { outcome, .. } => outcome,
        other => panic!("expected exact establishment commit: {other:?}"),
    }
}

fn committed_deferral(result: DeferralExecutionV2) -> DeferralCommittedOutcomeV2 {
    match result {
        DeferralExecutionV2::Committed { outcome, .. } => outcome,
        other => panic!("expected exact deferral commit: {other:?}"),
    }
}

fn current(sdk: &HandbookSdkV1, project: &ProjectBindingV2) -> EstablishedFromDeferredBasisV3 {
    match sdk
        .resolve_project_admission_scope_v3(
            &ProjectAdmissionBindingV3::ScopedChoiceV2(project.clone()),
            ADMISSION_GOVERNANCE_SCOPE_V1,
            None,
        )
        .unwrap()
    {
        ProjectAdmissionScopeBasisV3::EstablishedFromDeferredV3(basis) => basis,
        other => panic!("expected same-project established basis: {other:?}"),
    }
}

fn retained_files(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn collect(root: &Path, path: &Path, files: &mut BTreeMap<String, Vec<u8>>) {
        if path.is_dir() {
            for entry in fs::read_dir(path).unwrap() {
                collect(root, &entry.unwrap().path(), files);
            }
        } else {
            let relative = path
                .strip_prefix(root)
                .unwrap()
                .to_str()
                .unwrap()
                .replace('\\', "/");
            let bytes = if relative == handbook_engine::REPOSITORY_INITIALIZATION_LOCK_PATH_V1
                || relative == ".handbook/admission-governance.lock"
            {
                // Native byte-range locks cannot be raw-read during contention
                // on Windows. Regular-file metadata proves their exact empty
                // bytes; owner current/replay also verifies their retained
                // physical generation against the original claim/anchor.
                let metadata = fs::symlink_metadata(path).unwrap();
                assert!(metadata.is_file() && !metadata.file_type().is_symlink());
                assert_eq!(metadata.len(), 0);
                Vec::new()
            } else {
                fs::read(path).unwrap()
            };
            files.insert(relative, bytes);
        }
    }
    let mut files = BTreeMap::new();
    for entry in fs::read_dir(root).unwrap() {
        let entry = entry.unwrap();
        if entry
            .file_name()
            .to_str()
            .unwrap()
            .starts_with(".handbook-")
        {
            collect(root, &entry.path(), &mut files);
        }
    }
    for entry in fs::read_dir(root.join(".handbook")).unwrap() {
        let entry = entry.unwrap();
        if entry
            .file_name()
            .to_str()
            .unwrap()
            .starts_with("admission-governance")
            || entry.file_name() == "repository-identity.v1"
        {
            collect(root, &entry.path(), &mut files);
        }
    }
    files
}

#[test]
fn sdk_prepares_and_establishes_exact_governance_on_the_deferred_project() {
    for ceiling in [7_200, 86_400] {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join(".git")).unwrap();
        let (original_d, deferred) = defer(root.path());
        let sdk = HandbookSdkV1::open(root.path());
        let input = request(&deferred, ceiling, "later-establishment");
        let original_files = retained_files(root.path());
        let prepared = sdk
            .prepare_deferred_project_admission_governance(&input)
            .unwrap();
        let encoded = prepared.locator().canonical_json_bytes().unwrap();
        let locator = DeferredEstablishmentLocatorV3::from_json_bytes(&encoded).unwrap();
        assert_eq!(*prepared.locator(), locator);
        assert_eq!(retained_files(root.path()), original_files);
        let original_g = committed(sdk.establish_deferred_project_admission_governance(prepared));
        assert!(
            original_g.validation_confirmed
                && original_g.advisory_permission_accepted
                && original_g.activation_committed
        );
        assert_eq!(original_g.locator, locator);
        assert_eq!(original_g.validated.input, input);
        for (path, bytes) in &original_files {
            if path != GOVERNANCE_CURRENT_PATH_V2 {
                assert_eq!(
                    fs::read(root.path().join(path)).unwrap(),
                    *bytes,
                    "original {path}"
                );
            }
        }
        let before_reset = retained_files(root.path());
        assert!(before_reset.len() > original_files.len());
        let state = root.path().join(".handbook/state/temporary/runtime.json");
        fs::create_dir_all(state.parent().unwrap()).unwrap();
        fs::write(&state, b"runtime sentinel").unwrap();
        let reset = sdk
            .run_setup(&SetupRequest {
                mode: SetupMode::Refresh,
                rewrite: false,
                reset_state: true,
            })
            .unwrap();
        assert!(reset.reset_applied);
        assert!(!state.exists());
        assert_eq!(retained_files(root.path()), before_reset);
        drop(sdk);
        let reopened = HandbookSdkV1::open(root.path());
        let basis = current(&reopened, &deferred.project);
        assert_eq!(basis.project, deferred.project);
        assert_eq!(basis.predecessor, input.expected_deferred);
        assert_eq!(
            basis.configuration.bindings[0].max_observation_age_seconds,
            ceiling
        );
        assert_eq!(basis.outcome_fingerprint, original_g.outcome_fingerprint);
        assert_eq!(basis.replacement_basis, input.replacement_basis);
        assert_eq!(basis.establishment_rule.version, "3.0.0");
        let native = ProjectAdmissionEstablishmentV3::new();
        assert_eq!(
            native
                .resolve_current(
                    root.path(),
                    &deferred.project,
                    ADMISSION_GOVERNANCE_SCOPE_V1,
                    None
                )
                .unwrap(),
            DeferredScopeBasisV3::EstablishedFromDeferredV3(basis)
        );
        let replay = reopened.resolve_deferred_admission_governance_request(&locator);
        assert!(matches!(
            &replay,
            DeferredEstablishmentExecutionV3::Committed {
                delivery: GovernanceDeliveryV1::Replay,
                ..
            }
        ));
        assert_eq!(replay, native.resolve_request(root.path(), &locator));
        assert_eq!(committed(replay), original_g);
        assert_eq!(
            committed_deferral(reopened.resolve_admission_deferral_request(&original_d.locator)),
            original_d
        );
        assert_eq!(retained_files(root.path()), before_reset);
    }
}

#[test]
fn complete_pre_effect_locator_resolves_after_first_consumer_serialization_failure() {
    struct DisconnectedConsumer;
    impl std::io::Write for DisconnectedConsumer {
        fn write(&mut self, _bytes: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other(
                "consumer disconnected before serialization",
            ))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let root = tempdir().unwrap();
    let (_, deferred) = defer(root.path());
    let sdk = HandbookSdkV1::open(root.path());
    let input = request(&deferred, 7_200, "lost-sdk-reply");
    let before = retained_files(root.path());
    let prepared = sdk
        .prepare_deferred_project_admission_governance(&input)
        .unwrap();
    let encoded = prepared.locator().canonical_json_bytes().unwrap();
    assert_eq!(retained_files(root.path()), before);
    let execution = sdk.establish_deferred_project_admission_governance(prepared);
    assert!(matches!(
        &execution,
        DeferredEstablishmentExecutionV3::Committed {
            delivery: GovernanceDeliveryV1::Original,
            ..
        }
    ));
    assert!(serde_json::to_writer(DisconnectedConsumer, &execution).is_err());
    let original = committed(execution);
    drop(sdk);
    let locator = DeferredEstablishmentLocatorV3::from_json_bytes(&encoded).unwrap();
    assert_eq!(locator.request, input);
    let reopened = HandbookSdkV1::open(root.path());
    let before_replay = retained_files(root.path());
    let replay = reopened.resolve_deferred_admission_governance_request(&locator);
    assert_eq!(
        replay,
        ProjectAdmissionEstablishmentV3::new().resolve_request(root.path(), &locator)
    );
    assert_eq!(committed(replay), original);
    assert_eq!(retained_files(root.path()), before_replay);
}

#[test]
fn historical_sdk_replays_never_restore_or_repair_damaged_current() {
    let root = tempdir().unwrap();
    let (original_d, deferred) = defer(root.path());
    let sdk = HandbookSdkV1::open(root.path());
    let input = request(&deferred, 86_400, "historical-replay");
    let prepared = sdk
        .prepare_deferred_project_admission_governance(&input)
        .unwrap();
    let locator = prepared.locator().clone();
    let original_g = committed(sdk.establish_deferred_project_admission_governance(prepared));
    let encoded = DeferredEstablishmentExecutionV3::Committed {
        outcome: original_g.clone(),
        delivery: GovernanceDeliveryV1::Original,
    }
    .canonical_json_bytes()
    .unwrap();
    fs::remove_file(root.path().join(GOVERNANCE_CURRENT_PATH_V2)).unwrap();
    let damaged = retained_files(root.path());
    // A decoded wire result conveys no live owner authority.
    assert_eq!(
        committed(DeferredEstablishmentExecutionV3::from_json_bytes(&encoded).unwrap()),
        original_g
    );
    assert_eq!(
        committed(sdk.resolve_deferred_admission_governance_request(&locator)),
        original_g
    );
    assert_eq!(
        committed_deferral(sdk.resolve_admission_deferral_request(&original_d.locator)),
        original_d
    );
    assert!(sdk
        .resolve_project_admission_scope_v3(
            &ProjectAdmissionBindingV3::ScopedChoiceV2(deferred.project),
            ADMISSION_GOVERNANCE_SCOPE_V1,
            None
        )
        .is_err());
    assert_eq!(retained_files(root.path()), damaged);
}

#[test]
fn tagged_scope_and_exact_expected_basis_preserve_owner_limits() {
    let root = tempdir().unwrap();
    let (_, deferred) = defer(root.path());
    let sdk = HandbookSdkV1::open(root.path());
    let project = ProjectAdmissionBindingV3::ScopedChoiceV2(deferred.project.clone());
    let expected_d = ExpectedProjectAdmissionBasisV3::DeferredV2(ExpectedDeferredBasisV2 {
        outcome_fingerprint: deferred.outcome_fingerprint.clone(),
        relation_fingerprint: deferred.relation_fingerprint.clone(),
    });
    assert_eq!(
        sdk.resolve_project_admission_scope_v3(
            &project,
            ADMISSION_GOVERNANCE_SCOPE_V1,
            Some(&expected_d)
        )
        .unwrap(),
        ProjectAdmissionScopeBasisV3::DeferredV2(deferred.clone())
    );
    let wrong_tag = ExpectedProjectAdmissionBasisV3::EstablishedV1(ExpectedGovernanceBasisV1 {
        configuration_fingerprint: deferred.outcome_fingerprint.clone(),
        outcome_fingerprint: deferred.outcome_fingerprint.clone(),
    });
    assert!(sdk
        .resolve_project_admission_scope_v3(
            &project,
            ADMISSION_GOVERNANCE_SCOPE_V1,
            Some(&wrong_tag)
        )
        .is_err());
    let mut encoded = serde_json::to_value(&project).unwrap();
    encoded["project_contract"] = serde_json::json!("established_v1");
    let wrong_project_tag =
        ProjectAdmissionBindingV3::from_json_bytes(&serde_json::to_vec(&encoded).unwrap()).unwrap();
    assert!(sdk
        .resolve_project_admission_scope_v3(&wrong_project_tag, ADMISSION_GOVERNANCE_SCOPE_V1, None)
        .is_err());
    assert!(sdk
        .resolve_project_admission_scope_v3(
            &project,
            "contracts.evidence-freshness-policy/other",
            None
        )
        .is_err());
    let input = request(&deferred, 7_200, "expected-exact");
    let prepared = sdk
        .prepare_deferred_project_admission_governance(&input)
        .unwrap();
    committed(sdk.establish_deferred_project_admission_governance(prepared));
    let g = current(&sdk, &deferred.project);
    let expected_g = ExpectedProjectAdmissionBasisV3::EstablishedFromDeferredV3(
        ExpectedEstablishedFromDeferredBasisV3 {
            configuration_fingerprint: g.configuration_fingerprint.clone(),
            outcome_fingerprint: g.outcome_fingerprint.clone(),
            relation_fingerprint: g.relation_fingerprint.clone(),
        },
    );
    let before = retained_files(root.path());
    assert!(sdk
        .resolve_project_admission_scope_v3(
            &project,
            ADMISSION_GOVERNANCE_SCOPE_V1,
            Some(&expected_d)
        )
        .is_err());
    assert_eq!(
        sdk.resolve_project_admission_scope_v3(
            &project,
            ADMISSION_GOVERNANCE_SCOPE_V1,
            Some(&expected_g)
        )
        .unwrap(),
        ProjectAdmissionScopeBasisV3::EstablishedFromDeferredV3(g)
    );
    let mut wrong_g = expected_g;
    if let ExpectedProjectAdmissionBasisV3::EstablishedFromDeferredV3(ref mut basis) = wrong_g {
        basis.relation_fingerprint = deferred.relation_fingerprint.clone();
    }
    assert!(sdk
        .resolve_project_admission_scope_v3(&project, ADMISSION_GOVERNANCE_SCOPE_V1, Some(&wrong_g))
        .is_err());
    assert!(sdk
        .prepare_deferred_project_admission_governance(&input)
        .is_err());
    assert!(sdk
        .resolve_project_admission_scope(
            &ProjectAdmissionBindingV2::ScopedChoiceV2(deferred.project),
            ADMISSION_GOVERNANCE_SCOPE_V1,
            None
        )
        .is_err());
    assert_eq!(retained_files(root.path()), before);
}

#[test]
fn missing_claim_and_future_inventory_cannot_supply_sdk_eligibility() {
    for missing_claim in [true, false] {
        let root = tempdir().unwrap();
        let (_, deferred) = defer(root.path());
        let sdk = HandbookSdkV1::open(root.path());
        let input = request(&deferred, 7_200, "damaged-predecessor");
        let prepared = sdk
            .prepare_deferred_project_admission_governance(&input)
            .unwrap();
        if missing_claim {
            fs::remove_file(
                root.path()
                    .join(handbook_engine::REPOSITORY_INITIALIZATION_CLAIM_PATH_V1),
            )
            .unwrap();
        } else {
            fs::create_dir(root.path().join(".handbook/admission-governance/v4")).unwrap();
        }
        let before = retained_files(root.path());
        let sdk_error = sdk
            .prepare_deferred_project_admission_governance(&input)
            .unwrap_err();
        assert_eq!(
            sdk_error,
            ProjectAdmissionEstablishmentV3::new()
                .prepare(root.path(), &input)
                .unwrap_err()
        );
        assert!(matches!(
            sdk.establish_deferred_project_admission_governance(prepared),
            DeferredEstablishmentExecutionV3::Refused { .. }
        ));
        assert!(sdk
            .resolve_project_admission_scope_v3(
                &ProjectAdmissionBindingV3::ScopedChoiceV2(deferred.project),
                ADMISSION_GOVERNANCE_SCOPE_V1,
                None
            )
            .is_err());
        assert_eq!(retained_files(root.path()), before);
    }
}

#[test]
fn sdk_preserves_noncommit_uncertainty_and_exact_native_recovery() {
    for boundary in [
        EstablishmentFaultBoundaryV3::Stage(2),
        EstablishmentFaultBoundaryV3::Decision,
        EstablishmentFaultBoundaryV3::CurrentDisplaced,
    ] {
        let root = tempdir().unwrap();
        let (_, deferred) = defer(root.path());
        let sdk = HandbookSdkV1::open(root.path());
        let input = request(&deferred, 7_200, "interrupted-sdk-operation");
        let prepared = sdk
            .prepare_deferred_project_admission_governance(&input)
            .unwrap();
        let locator = prepared.locator().clone();
        let interrupted = ProjectAdmissionEstablishmentV3::new()
            .with_fault_for_testing(boundary)
            .establish(root.path(), prepared);
        assert!(matches!(
            interrupted,
            DeferredEstablishmentExecutionV3::Uncertain { .. }
        ));
        let replay = sdk.resolve_deferred_admission_governance_request(&locator);
        assert_eq!(
            replay,
            ProjectAdmissionEstablishmentV3::new().resolve_request(root.path(), &locator)
        );
        if boundary == EstablishmentFaultBoundaryV3::Stage(2) {
            assert!(matches!(
                replay,
                DeferredEstablishmentExecutionV3::NotCommitted { .. }
            ));
            assert_eq!(
                sdk.resolve_project_admission_scope_v3(
                    &ProjectAdmissionBindingV3::ScopedChoiceV2(deferred.project.clone()),
                    ADMISSION_GOVERNANCE_SCOPE_V1,
                    None
                )
                .unwrap(),
                ProjectAdmissionScopeBasisV3::DeferredV2(deferred.clone())
            );
            let next = request(&deferred, 86_400, "new-approved-attempt");
            let prepared = sdk
                .prepare_deferred_project_admission_governance(&next)
                .unwrap();
            committed(sdk.establish_deferred_project_admission_governance(prepared));
            assert_eq!(
                current(&sdk, &deferred.project).configuration.bindings[0]
                    .max_observation_age_seconds,
                86_400
            );
            assert_eq!(
                sdk.resolve_deferred_admission_governance_request(&locator),
                replay
            );
        } else {
            assert!(matches!(
                replay,
                DeferredEstablishmentExecutionV3::Committed { .. }
            ));
            assert_eq!(
                current(&sdk, &deferred.project).configuration.bindings[0]
                    .max_observation_age_seconds,
                7_200
            );
        }
    }
}

#[test]
fn additive_sdk_methods_leave_all_generic_catalogs_unchanged() {
    use handbook_sdk::contract_membrane::{operation_inventory, ProductionWitness, UtcTimestamp};
    let rows = operation_inventory();
    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("2031-01-01T00:00:00Z").unwrap())
            .unwrap();
    assert_eq!(rows.len(), 62);
    assert_eq!(witness.capabilities().len(), 35);
    assert_eq!(witness.schema_manifest().len(), 180);
    let use_cases = witness
        .schema_manifest()
        .iter()
        .map(|entry| entry.schema().reference().as_str())
        .filter(|name| name.starts_with("handbook.use-case.") && name.ends_with("-result@1.0.0"))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(use_cases.len(), 7);
    assert!(rows.iter().all(|entry| !entry
        .operation_ref()
        .as_str()
        .contains("admission-governance")));
    assert!(witness.schema_manifest().iter().all(|entry| !entry
        .schema()
        .reference()
        .as_str()
        .contains("admission-governance")));
}

#[test]
fn v3_tagged_resolution_preserves_the_existing_v1_project_contract() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let prepared = sdk
        .prepare_project_admission_governance(&GovernanceRequestInputV1::standalone_freshness(
            7_200,
            "existing-v1",
            "The human approved this exact fresh-project governance.",
        ))
        .unwrap();
    let locator = prepared.locator().clone();
    assert!(matches!(
        sdk.initialize_project_admission_governance(prepared),
        GovernanceExecutionV1::Committed { .. }
    ));
    let active = sdk
        .resolve_project_admission_governance(&locator.project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
        .unwrap();
    let expected = ExpectedProjectAdmissionBasisV3::EstablishedV1(ExpectedGovernanceBasisV1 {
        configuration_fingerprint: active.configuration_fingerprint.clone(),
        outcome_fingerprint: active.outcome_fingerprint.clone(),
    });
    assert_eq!(
        sdk.resolve_project_admission_scope_v3(
            &ProjectAdmissionBindingV3::EstablishedV1(locator.project),
            ADMISSION_GOVERNANCE_SCOPE_V1,
            Some(&expected)
        )
        .unwrap(),
        ProjectAdmissionScopeBasisV3::EstablishedV1(active)
    );
}

fn wait_file(path: &Path) {
    let started = std::time::Instant::now();
    while !path.exists() {
        assert!(
            started.elapsed() < std::time::Duration::from_secs(60),
            "timed out waiting for {path:?}"
        );
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

#[test]
fn deferred_establishment_sdk_process_worker() {
    let Ok(mode) = std::env::var("HCM_D2G_SDK_MODE") else {
        return;
    };
    let root = std::path::PathBuf::from(std::env::var_os("HCM_D2G_SDK_ROOT").unwrap());
    let control = std::path::PathBuf::from(std::env::var_os("HCM_D2G_SDK_CONTROL").unwrap());
    let sdk = HandbookSdkV1::open(&root);
    let locator = DeferredEstablishmentLocatorV3::from_json_bytes(
        &fs::read(control.join("locator.json")).unwrap(),
    )
    .unwrap();
    let original_d = DeferralRequestLocatorV2::from_json_bytes(
        &fs::read(control.join("deferral.json")).unwrap(),
    )
    .unwrap();
    let prepared = if matches!(mode.as_str(), "winner" | "same" | "different") {
        let input = if mode == "different" {
            DeferredEstablishmentRequestV3::human_approved(
                locator.request.project.clone(),
                locator.request.expected_deferred.clone(),
                86_400,
                "competing-request",
                "The human approved this other exact configuration and terms.",
            )
            .unwrap()
        } else {
            locator.request.clone()
        };
        Some(
            sdk.prepare_deferred_project_admission_governance(&input)
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
            let boundary = match std::env::var("HCM_D2G_SDK_BOUNDARY").unwrap().as_str() {
                "decision" => EstablishmentFaultBoundaryV3::Decision,
                "displacement" => EstablishmentFaultBoundaryV3::CurrentDisplaced,
                other => panic!("unsupported boundary {other}"),
            };
            let execution = ProjectAdmissionEstablishmentV3::new()
                .with_pause_for_testing(
                    boundary,
                    control.join("winner.paused"),
                    control.join("winner.release"),
                )
                .establish(&root, prepared.unwrap());
            serde_json::json!({"execution":execution,"selected":retained_files(&root)})
        }
        "same" | "different" => {
            serde_json::json!({"execution":sdk.establish_deferred_project_admission_governance(prepared.unwrap())})
        }
        "refresh" => {
            let outcome = sdk
                .run_setup(&SetupRequest {
                    mode: SetupMode::Refresh,
                    rewrite: false,
                    reset_state: true,
                })
                .unwrap();
            serde_json::json!({"ok":true,"reset_applied":outcome.reset_applied,"selected":retained_files(&root)})
        }
        "identity" => {
            let identity = handbook_engine::RepositoryInvocationIdentityServiceV1::new()
                .initialize_for_setup(&root)
                .unwrap();
            serde_json::json!({"ok":true,"identity":identity.repository_identity_fingerprint()})
        }
        "current" => {
            let basis = sdk
                .resolve_project_admission_scope_v3(
                    &ProjectAdmissionBindingV3::ScopedChoiceV2(locator.request.project.clone()),
                    ADMISSION_GOVERNANCE_SCOPE_V1,
                    None,
                )
                .unwrap();
            serde_json::json!({"ok":true,"basis":basis})
        }
        "recover" => {
            serde_json::json!({"execution":sdk.resolve_deferred_admission_governance_request(&locator)})
        }
        "original_d" => {
            serde_json::json!({"execution":sdk.resolve_admission_deferral_request(&original_d)})
        }
        other => panic!("unsupported SDK worker mode {other}"),
    };
    fs::write(
        control.join(format!("{mode}.result.json")),
        serde_json::to_vec(&result).unwrap(),
    )
    .unwrap();
    fs::write(control.join(format!("{mode}.completed")), b"complete").unwrap();
}

struct Workers(Vec<std::process::Child>);
impl Drop for Workers {
    fn drop(&mut self) {
        for child in &mut self.0 {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn wait_result(child: &mut std::process::Child, path: &Path) {
    let started = std::time::Instant::now();
    while !path.exists() {
        if let Some(status) = child.try_wait().unwrap() {
            let mut stderr = String::new();
            if let Some(mut pipe) = child.stderr.take() {
                std::io::Read::read_to_string(&mut pipe, &mut stderr).unwrap();
            }
            panic!("worker exited {status} before {path:?}: {stderr}");
        }
        assert!(
            started.elapsed() < std::time::Duration::from_secs(60),
            "timed out waiting for {path:?}"
        );
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn worker(root: &Path, control: &Path, mode: &str, boundary: &str) -> std::process::Child {
    std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "deferred_establishment_sdk_process_worker",
            "--nocapture",
            "--test-threads=1",
        ])
        .env("HCM_D2G_SDK_MODE", mode)
        .env("HCM_D2G_SDK_ROOT", root)
        .env("HCM_D2G_SDK_CONTROL", control)
        .env("HCM_D2G_SDK_BOUNDARY", boundary)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap()
}

#[test]
fn paused_native_transition_serializes_sdk_reset_readers_recovery_and_identity() {
    for boundary in ["decision", "displacement"] {
        let root = tempdir().unwrap();
        fs::create_dir(root.path().join(".git")).unwrap();
        let (original_d, deferred) = defer(root.path());
        let sdk = HandbookSdkV1::open(root.path());
        let input = request(&deferred, 7_200, "paused-exact-winner");
        let locator = sdk
            .prepare_deferred_project_admission_governance(&input)
            .unwrap()
            .locator()
            .clone();
        let original_files = retained_files(root.path());
        let state = root.path().join(".handbook/state/temporary/runtime.json");
        fs::create_dir_all(state.parent().unwrap()).unwrap();
        fs::write(&state, b"runtime sentinel").unwrap();
        let control = tempdir().unwrap();
        fs::write(
            control.path().join("locator.json"),
            locator.canonical_json_bytes().unwrap(),
        )
        .unwrap();
        fs::write(
            control.path().join("deferral.json"),
            original_d.locator.canonical_json_bytes().unwrap(),
        )
        .unwrap();
        let modes = [
            "winner",
            "same",
            "different",
            "refresh",
            "identity",
            "current",
            "recover",
            "original_d",
        ];
        let mut children = Workers(
            modes
                .iter()
                .map(|mode| worker(root.path(), control.path(), mode, boundary))
                .collect(),
        );
        for mode in modes {
            wait_file(&control.path().join(format!("{mode}.ready")));
        }
        fs::write(control.path().join("winner.go"), b"go").unwrap();
        wait_file(&control.path().join("winner.paused"));
        let old = original_files.get(GOVERNANCE_CURRENT_PATH_V2).unwrap();
        if boundary == "displacement" {
            assert!(!root.path().join(GOVERNANCE_CURRENT_PATH_V2).exists());
            let history = handbook_contracts::project_admission_governance::transition_request_storage_path_v3(&locator);
            assert_eq!(
                fs::read(root.path().join(&history).join("current-displaced.json")).unwrap(),
                *old
            );
            assert!(root
                .path()
                .join(history)
                .join("current-prepared.json")
                .exists());
        } else {
            assert_eq!(
                fs::read(root.path().join(GOVERNANCE_CURRENT_PATH_V2)).unwrap(),
                *old
            );
        }
        for mode in &modes[1..] {
            fs::write(control.path().join(format!("{mode}.go")), b"go").unwrap();
        }
        for mode in &modes[1..] {
            wait_file(&control.path().join(format!("{mode}.attempt")));
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
        for mode in modes {
            assert!(
                !control.path().join(format!("{mode}.result.json")).exists(),
                "{boundary}: {mode} bypassed held root/governance locks"
            );
        }
        fs::write(control.path().join("winner.release"), b"release").unwrap();
        for (mode, child) in modes.iter().zip(&mut children.0) {
            wait_result(child, &control.path().join(format!("{mode}.completed")));
            let status = child.wait().unwrap();
            assert!(status.success(), "{boundary}: {mode} exited {status}");
        }
        let result = |mode: &str| -> serde_json::Value {
            serde_json::from_slice(
                &fs::read(control.path().join(format!("{mode}.result.json"))).unwrap(),
            )
            .unwrap()
        };
        let original_g =
            committed(serde_json::from_value(result("winner")["execution"].clone()).unwrap());
        for mode in ["same", "recover"] {
            let replay: DeferredEstablishmentExecutionV3 =
                serde_json::from_value(result(mode)["execution"].clone()).unwrap();
            assert!(matches!(
                &replay,
                DeferredEstablishmentExecutionV3::Committed {
                    delivery: GovernanceDeliveryV1::Replay,
                    ..
                }
            ));
            assert_eq!(committed(replay), original_g);
        }
        assert_eq!(result("different")["execution"]["status"], "refused");
        assert_eq!(result("refresh")["reset_applied"], true);
        assert!(!state.exists());
        assert_eq!(
            result("identity")["identity"],
            deferred.project.repository_identity_fingerprint
        );
        let basis = current(&sdk, &deferred.project);
        assert_eq!(
            result("current")["basis"],
            serde_json::to_value(ProjectAdmissionScopeBasisV3::EstablishedFromDeferredV3(
                basis
            ))
            .unwrap()
        );
        assert_eq!(
            committed_deferral(
                serde_json::from_value(result("original_d")["execution"].clone()).unwrap()
            ),
            original_d
        );
        assert_eq!(result("winner")["selected"], result("refresh")["selected"]);
        assert_eq!(
            result("winner")["selected"],
            serde_json::to_value(retained_files(root.path())).unwrap()
        );
        for (path, bytes) in original_files {
            if path != GOVERNANCE_CURRENT_PATH_V2 {
                assert_eq!(fs::read(root.path().join(path)).unwrap(), bytes);
            }
        }
    }
}
