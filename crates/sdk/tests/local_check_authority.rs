mod local_contract_support;

use handbook_engine::local_authority::local_check::*;
use handbook_engine::local_authority::{LocalAuthorityDispositionV1, LocalRevokeRequestV1};
use handbook_engine::RetainedRepositoryRootV1;
use handbook_sdk::local_setup::LocalInitialSetupLocatorV1;
use handbook_sdk::HandbookSdkV1;
use local_contract_support::{adopt_check, establish_initial};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use tempfile::tempdir;

fn action(locator: &LocalCheckLocatorV1) -> LocalCheckActionRequestV1 {
    LocalCheckActionRequestV1 {
        action: LocalCheckActionV1::DefinitionAuthor,
        action_ref: "handbook.local-authority-probe@1.0.0".into(),
        subject_ref: "handbook.local-authority-probe@1.0.0".into(),
        content_fingerprint: format!("sha256:{}", "1".repeat(64)),
        closure_fingerprint: format!("sha256:{}", "2".repeat(64)),
        actor_ref: locator.terms.agent.clone(),
        participant_ref: locator.terms.author_participant_ref.clone(),
        semantic_time_utc: "2026-10-05T12:00:00Z".into(),
        owner_write_paths: vec![".handbook/state/contracts/authority-probe.json".into()],
        input_paths: vec!["inputs/order.json".into()],
    }
}

fn prepared() -> (
    tempfile::TempDir,
    LocalInitialSetupLocatorV1,
    LocalCheckLocatorV1,
) {
    let dir = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let initial = establish_initial(&sdk);
    let (locator, _) = adopt_check(&sdk, dir.path(), &initial);
    fs::create_dir_all(dir.path().join(".handbook/state/contracts")).unwrap();
    (dir, initial, locator)
}

#[test]
fn successor_replay_is_history_and_missing_locks_or_revocation_cannot_restore_permission() {
    let (dir, initial, locator) = prepared();
    let sdk = HandbookSdkV1::open(dir.path());
    let basis = sdk.observe_local_check_basis(&initial).unwrap();
    let outcome = sdk
        .resolve_current_local_check_terms(&initial, &locator)
        .unwrap();
    assert_eq!(
        sdk.resume_local_check_terms_request(&locator).disposition,
        LocalAuthorityDispositionV1::Replayed
    );
    assert!(sdk.resolve_current_local_initial_setup(&initial).is_err());
    let mut wider = action(&locator);
    wider.owner_write_paths = vec!["secrets/stolen.json".into()];
    assert!(NativeLocalCheckAuthorityV1::with_admission(
        dir.path(),
        &locator,
        &basis,
        &wider,
        |_| Ok(())
    )
    .is_err());
    let mut invalid_time = action(&locator);
    invalid_time.semantic_time_utc = "2026-02-30T12:00:00Z".into();
    assert!(NativeLocalCheckAuthorityV1::with_admission(
        dir.path(),
        &locator,
        &basis,
        &invalid_time,
        |_| Ok(())
    )
    .is_err());
    let stack = handbook_engine::local_authority::resolved_local_operating_profile().unwrap();
    NativeLocalCheckAuthorityV1::with_admission(
        dir.path(),
        &locator,
        &basis,
        &action(&locator),
        |admission| {
            let selected = stack.context_resolution();
            admission.require_resolution_ceiling(
                selected.exact_ref().as_str(),
                selected.definition_fingerprint().as_str(),
                locator
                    .terms
                    .ceiling
                    .dimensions
                    .each_ref()
                    .map(String::as_str),
            )?;
            let mut wider = locator
                .terms
                .ceiling
                .dimensions
                .each_ref()
                .map(String::as_str);
            wider[0] = "slice";
            assert!(admission
                .require_resolution_ceiling(
                    selected.exact_ref().as_str(),
                    selected.definition_fingerprint().as_str(),
                    wider
                )
                .is_err());
            Ok(())
        },
    )
    .unwrap();
    let mut other_participant = action(&locator);
    other_participant.action = LocalCheckActionV1::Review;
    assert!(NativeLocalCheckAuthorityV1::with_admission(
        dir.path(),
        &locator,
        &basis,
        &other_participant,
        |_| Ok(())
    )
    .is_err());
    let revoke = LocalRevokeRequestV1 {
        request_key: "revoke-check".into(),
        expected_current_fingerprint: outcome.outcome_fingerprint,
        principal: locator.terms.revocation_principal.clone(),
        source_ref: "human.actual-revoke".into(),
        instruction: "Revoke the supplied local-check authority".into(),
    };
    sdk.revoke_local_check_terms(&locator, &revoke).unwrap();
    sdk.revoke_local_check_terms(&locator, &revoke).unwrap();
    assert!(NativeLocalCheckAuthorityV1::with_admission(
        dir.path(),
        &locator,
        &basis,
        &action(&locator),
        |_| Ok(())
    )
    .is_err());
    assert_eq!(
        sdk.resume_local_check_terms_request(&locator).disposition,
        LocalAuthorityDispositionV1::Replayed
    );
    assert!(sdk
        .resolve_current_local_check_terms(&initial, &locator)
        .is_err());
    let lock = dir
        .path()
        .join(".handbook/state/locks/local-operating.lock");
    fs::remove_file(&lock).unwrap();
    assert!(sdk
        .resolve_local_check_terms_request(&locator)
        .outcome
        .is_none());
    assert!(!lock.exists());
}

#[test]
fn substituted_existing_operating_lock_does_not_restore_current_permission() {
    let (dir, initial, locator) = prepared();
    let sdk = HandbookSdkV1::open(dir.path());
    let lock = dir
        .path()
        .join(".handbook/state/locks/local-operating.lock");
    fs::rename(&lock, lock.with_extension("original")).unwrap();
    fs::write(&lock, []).unwrap();
    assert!(sdk
        .resolve_current_local_check_terms(&initial, &locator)
        .is_err());
    let history = sdk.resume_local_check_terms_request(&locator);
    assert_eq!(history.disposition, LocalAuthorityDispositionV1::Replayed);
    assert!(history.outcome.is_some());
    assert!(sdk
        .resolve_current_local_check_terms(&initial, &locator)
        .is_err());
}

#[test]
fn interrupted_successor_adoption_recovers_only_its_retained_original_intent() {
    use local_contract_support::supplied_successor_terms;
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
    let locator = prepared.locator().clone();
    let interrupted = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        NativeLocalCheckAuthorityV1::adopt_with_interruption_for_test(dir.path(), prepared, || {
            panic!("native adoption interrupted before head publication")
        })
    }));
    assert!(interrupted.is_err());
    assert!(sdk
        .resolve_local_check_terms_request(&locator)
        .outcome
        .is_none());
    assert!(!dir
        .path()
        .join(".handbook/local-operating/local-check-v1/current.json")
        .exists());
    let recovered = sdk.resume_local_check_terms_request(&locator);
    assert_eq!(
        recovered.disposition,
        LocalAuthorityDispositionV1::Committed,
        "{recovered:?}"
    );
    assert!(sdk
        .resolve_current_local_check_terms(&initial, &locator)
        .is_ok());
    let terminal = dir.path().join(format!(
        ".handbook/local-operating/local-check-v1/requests/{}/terminal.json",
        locator.terms.request_key
    ));
    fs::remove_file(&terminal).unwrap();
    assert!(sdk
        .resume_local_check_terms_request(&locator)
        .outcome
        .is_none());
    assert!(!terminal.exists());
}

#[test]
fn current_generations_stay_pinned_through_consumption_and_lost_heads_refuse() {
    for target in [
        ".handbook/project/charter.yaml",
        ".handbook/local-operating/local-check-v1/current.json",
        ".handbook/admission-governance/v2/scope-current.json",
    ] {
        let (dir, initial, locator) = prepared();
        let sdk = HandbookSdkV1::open(dir.path());
        let basis = sdk.observe_local_check_basis(&initial).unwrap();
        let bytes = fs::read(dir.path().join(target)).unwrap();
        let result = NativeLocalCheckAuthorityV1::with_admission(
            dir.path(),
            &locator,
            &basis,
            &action(&locator),
            |admission| {
                let changed = fs::write(dir.path().join(target), b"substituted");
                if changed.is_ok() {
                    assert!(admission.validate_current().is_err());
                } else {
                    admission.validate_current()?;
                }
                Ok(changed.is_ok())
            },
        );
        if result.is_ok() {
            assert_eq!(fs::read(dir.path().join(target)).unwrap(), bytes);
        } else {
            assert!(sdk
                .resolve_current_local_check_terms(&initial, &locator)
                .is_err());
        }
    }
    let (dir, initial, locator) = prepared();
    let sdk = HandbookSdkV1::open(dir.path());
    fs::remove_file(
        dir.path()
            .join(".handbook/local-operating/local-check-v1/current.json"),
    )
    .unwrap();
    assert_eq!(
        sdk.resume_local_check_terms_request(&locator).disposition,
        LocalAuthorityDispositionV1::Replayed
    );
    assert!(sdk
        .resolve_current_local_check_terms(&initial, &locator)
        .is_err());
    assert!(!dir
        .path()
        .join(".handbook/local-operating/local-check-v1/current.json")
        .exists());
}

fn wait_file(path: &Path) {
    let start = Instant::now();
    while !path.exists() {
        assert!(
            start.elapsed() < Duration::from_secs(20),
            "child checkpoint never reached"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn paused_local_consumption_serializes_native_governance_establishment_and_revocation() {
    for mode in ["govern", "revoke"] {
        let (dir, initial, locator) = prepared();
        let messages = tempdir().unwrap();
        let sdk = HandbookSdkV1::open(dir.path());
        let basis = sdk.observe_local_check_basis(&initial).unwrap();
        fs::write(
            messages.path().join("initial.json"),
            initial.canonical_json_bytes().unwrap(),
        )
        .unwrap();
        fs::write(
            messages.path().join("locator.json"),
            locator.canonical_json_bytes().unwrap(),
        )
        .unwrap();
        fs::write(
            messages.path().join("basis.json"),
            serde_json::to_vec(&basis).unwrap(),
        )
        .unwrap();
        let child = |mode| {
            Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "local_authority_child", "--nocapture"])
                .env("HANDBOOK_CHECK_AUTHORITY_MODE", mode)
                .env("HANDBOOK_CHECK_AUTHORITY_ROOT", dir.path())
                .env("HANDBOOK_CHECK_AUTHORITY_MESSAGES", messages.path())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap()
        };
        let holding = child("hold");
        wait_file(&messages.path().join("holding"));
        let mut changing = child(mode);
        wait_file(&messages.path().join("changing"));
        std::thread::sleep(Duration::from_millis(80));
        assert!(
            changing.try_wait().unwrap().is_none(),
            "native change passed live consuming authority guard"
        );
        fs::write(messages.path().join("release"), []).unwrap();
        let held = holding.wait_with_output().unwrap();
        assert!(
            held.status.success(),
            "{}",
            String::from_utf8_lossy(&held.stderr)
        );
        let changed = changing.wait_with_output().unwrap();
        assert!(
            changed.status.success(),
            "{}",
            String::from_utf8_lossy(&changed.stderr)
        );
        assert_eq!(
            fs::read(
                dir.path()
                    .join(".handbook/state/contracts/authority-probe.json")
            )
            .unwrap(),
            b"exact serialized local effect"
        );
        assert!(NativeLocalCheckAuthorityV1::with_admission(
            dir.path(),
            &locator,
            &basis,
            &action(&locator),
            |_| Ok(())
        )
        .is_err());
        assert!(sdk
            .resolve_local_check_terms_request(&locator)
            .outcome
            .is_some());
        if mode == "govern" {
            assert!(sdk
                .resolve_current_local_check_terms(&initial, &locator)
                .is_ok());
        } else {
            assert!(sdk
                .resolve_current_local_check_terms(&initial, &locator)
                .is_err());
        }
    }
}

#[test]
fn local_authority_child() {
    let Ok(mode) = std::env::var("HANDBOOK_CHECK_AUTHORITY_MODE") else {
        return;
    };
    let root = std::path::PathBuf::from(std::env::var_os("HANDBOOK_CHECK_AUTHORITY_ROOT").unwrap());
    let messages =
        std::path::PathBuf::from(std::env::var_os("HANDBOOK_CHECK_AUTHORITY_MESSAGES").unwrap());
    let initial = LocalInitialSetupLocatorV1::from_json_bytes(
        &fs::read(messages.join("initial.json")).unwrap(),
    )
    .unwrap();
    let locator =
        LocalCheckLocatorV1::from_json_bytes(&fs::read(messages.join("locator.json")).unwrap())
            .unwrap();
    let basis: LocalCheckCurrentBasisV1 =
        serde_json::from_slice(&fs::read(messages.join("basis.json")).unwrap()).unwrap();
    let sdk = HandbookSdkV1::open(&root);
    if mode == "hold" {
        NativeLocalCheckAuthorityV1::with_admission(
            &root,
            &locator,
            &basis,
            &action(&locator),
            |admission| {
                fs::write(messages.join("holding"), []).unwrap();
                wait_file(&messages.join("release"));
                admission.validate_current()?;
                let retained = RetainedRepositoryRootV1::open_exact(&root).unwrap();
                retained
                    .write_new_durable(
                        ".handbook/state/contracts/authority-probe.json",
                        b"exact serialized local effect",
                    )
                    .unwrap();
                Ok(())
            },
        )
        .unwrap();
    } else if mode == "govern" {
        use handbook_sdk::admission_governance::*;
        fs::write(messages.join("changing"), []).unwrap();
        let project = initial.deferral.project.clone();
        let current = sdk
            .resolve_project_admission_scope_v3(
                &ProjectAdmissionBindingV3::ScopedChoiceV2(project.clone()),
                ADMISSION_GOVERNANCE_SCOPE_V1,
                None,
            )
            .unwrap();
        let ProjectAdmissionScopeBasisV3::DeferredV2(deferred) = current else {
            panic!("D expected");
        };
        let request = DeferredEstablishmentRequestV3::human_approved(
            project,
            ExpectedDeferredBasisV2 {
                outcome_fingerprint: deferred.outcome_fingerprint,
                relation_fingerprint: deferred.relation_fingerprint,
            },
            60,
            "authority-test-G",
            "Human supplied and approved the exact test G",
        )
        .unwrap();
        let prepared = sdk
            .prepare_deferred_project_admission_governance(&request)
            .unwrap();
        assert!(matches!(
            sdk.establish_deferred_project_admission_governance(prepared),
            DeferredEstablishmentExecutionV3::Committed { .. }
        ));
    } else {
        fs::write(messages.join("changing"), []).unwrap();
        let expected = sdk
            .resolve_local_check_terms_request(&locator)
            .outcome
            .unwrap();
        sdk.revoke_local_check_terms(
            &locator,
            &LocalRevokeRequestV1 {
                request_key: "paused-revoke".into(),
                expected_current_fingerprint: expected.outcome_fingerprint,
                principal: locator.terms.revocation_principal.clone(),
                source_ref: "human.paused-revoke".into(),
                instruction: "Actual test revocation".into(),
            },
        )
        .unwrap();
    }
}
