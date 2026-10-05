#![cfg(feature = "test-support")]

use handbook_contracts::project_admission_governance::*;
use serde::{de::DeserializeOwned, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const WORKER_MODE: &str = "HCM_DEFERRED_ESTABLISHMENT_MODE";
const WORKER_ROOT: &str = "HCM_DEFERRED_ESTABLISHMENT_ROOT";
const WORKER_CONTROL: &str = "HCM_DEFERRED_ESTABLISHMENT_CONTROL";
const WORKER_ID: &str = "HCM_DEFERRED_ESTABLISHMENT_ID";
const WORKER_KEY: &str = "HCM_DEFERRED_ESTABLISHMENT_KEY";
const WORKER_CEILING: &str = "HCM_DEFERRED_ESTABLISHMENT_CEILING";
const WORKER_BOUNDARY: &str = "HCM_DEFERRED_ESTABLISHMENT_BOUNDARY";
const WORKER_SOURCE: &str = "HCM_DEFERRED_ESTABLISHMENT_SOURCE";
const TIMEOUT: Duration = Duration::from_secs(20);

#[test]
fn admission_deferred_establishment_process_worker() {
    let Ok(mode) = std::env::var(WORKER_MODE) else {
        return;
    };
    let root = PathBuf::from(std::env::var_os(WORKER_ROOT).unwrap());
    let control = PathBuf::from(std::env::var_os(WORKER_CONTROL).unwrap());
    let id = std::env::var(WORKER_ID).unwrap();
    let owner = ProjectAdmissionEstablishmentV3::new();
    match mode.as_str() {
        "establish" | "crash" | "pause" => {
            let basis: DeferredGovernanceBasisV2 = read_json(&control.join("deferred.basis.json"));
            let request = approved_request(
                &basis,
                &std::env::var(WORKER_KEY).unwrap(),
                std::env::var(WORKER_CEILING).unwrap().parse().unwrap(),
            );
            let prepared = owner.prepare(&root, &request).unwrap();
            // Persist the complete caller locator before any operation effect.
            // Restarted resolvers never prepare another executable operation.
            write_bytes(
                &control.join(format!("{id}.locator.json")),
                &prepared.locator().canonical_json_bytes().unwrap(),
            );
            if mode == "crash" {
                let result = owner
                    .with_crash_for_testing(parse_boundary(
                        &std::env::var(WORKER_BOUNDARY).unwrap(),
                    ))
                    .establish(&root, prepared);
                panic!("Crash boundary returned: {}", bounded_debug(&result));
            }
            rendezvous(&control, &id);
            let owner = if mode == "pause" {
                owner.with_pause_for_testing(
                    parse_boundary(&std::env::var(WORKER_BOUNDARY).unwrap()),
                    control.join(format!("{id}.paused")),
                    control.join(format!("{id}.release")),
                )
            } else {
                owner
            };
            write_json(
                &control.join(format!("{id}.result.json")),
                &owner.establish(&root, prepared),
            );
        }
        "resolve" | "resolve-crash" | "current" => {
            let source = std::env::var(WORKER_SOURCE).unwrap();
            let locator = DeferredEstablishmentLocatorV3::from_json_bytes(
                &fs::read(control.join(format!("{source}.locator.json"))).unwrap(),
            )
            .unwrap();
            rendezvous(&control, &id);
            File::create(control.join(format!("{id}.entered"))).unwrap();
            if mode == "current" {
                write_json(
                    &control.join(format!("{id}.result.json")),
                    &owner.resolve_current(
                        &root,
                        &locator.request.project,
                        ADMISSION_GOVERNANCE_SCOPE_V1,
                        None,
                    ),
                );
            } else {
                let owner = if mode == "resolve-crash" {
                    owner.with_crash_for_testing(parse_boundary(
                        &std::env::var(WORKER_BOUNDARY).unwrap(),
                    ))
                } else {
                    owner
                };
                let result = owner.resolve_request(&root, &locator);
                assert_ne!(
                    mode,
                    "resolve-crash",
                    "Recovery crash returned: {}",
                    bounded_debug(&result)
                );
                write_json(&control.join(format!("{id}.result.json")), &result);
            }
        }
        other => panic!("Unknown worker mode: {other}"),
    }
    File::create(control.join(format!("{id}.done"))).unwrap();
}

fn parse_boundary(name: &str) -> EstablishmentFaultBoundaryV3 {
    match name {
        "selection-directory" => EstablishmentFaultBoundaryV3::SelectionDirectory,
        "request-directory" => EstablishmentFaultBoundaryV3::RequestDirectory,
        "request" => EstablishmentFaultBoundaryV3::Request,
        "selection" => EstablishmentFaultBoundaryV3::Selection,
        "intent" => EstablishmentFaultBoundaryV3::Intent,
        "decision" => EstablishmentFaultBoundaryV3::Decision,
        "current-prepared" => EstablishmentFaultBoundaryV3::CurrentPrepared,
        "current-displaced" => EstablishmentFaultBoundaryV3::CurrentDisplaced,
        "current-published" => EstablishmentFaultBoundaryV3::CurrentPublished,
        "terminal" => EstablishmentFaultBoundaryV3::Terminal,
        "closure" => EstablishmentFaultBoundaryV3::Closure,
        "reply" => EstablishmentFaultBoundaryV3::Reply,
        "noncommit-terminal" => EstablishmentFaultBoundaryV3::NonCommitTerminal,
        "noncommit-closure" => EstablishmentFaultBoundaryV3::NonCommitClosure,
        other => {
            if let Some(index) = other.strip_prefix("stage-") {
                EstablishmentFaultBoundaryV3::Stage(index.parse().unwrap())
            } else if let Some(index) = other.strip_prefix("installation-") {
                EstablishmentFaultBoundaryV3::Installation(index.parse().unwrap())
            } else {
                panic!("Unknown boundary: {name}");
            }
        }
    }
}

fn write_json(path: &Path, value: &impl Serialize) {
    write_bytes(path, &serde_json::to_vec(value).unwrap());
}

fn write_bytes(path: &Path, bytes: &[u8]) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    file.write_all(bytes).unwrap();
    file.sync_all().unwrap();
}

fn read_json<T: DeserializeOwned>(path: &Path) -> T {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

fn rendezvous(control: &Path, id: &str) {
    File::create(control.join(format!("{id}.ready"))).unwrap();
    wait_for_file(&control.join("go"));
}

fn wait_for_file(path: &Path) {
    let start = Instant::now();
    while !path.exists() {
        assert!(
            start.elapsed() < TIMEOUT,
            "Worker rendezvous timed out: {path:?}"
        );
        thread::sleep(Duration::from_millis(10));
    }
}

struct Worker(Option<Child>);

impl Worker {
    fn start(
        root: &Path,
        control: &Path,
        id: &str,
        mode: &str,
        key: &str,
        ceiling: u64,
        boundary: &str,
        source: &str,
    ) -> Self {
        Self(Some(
            Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "admission_deferred_establishment_process_worker",
                    "--nocapture",
                    "--test-threads=1",
                ])
                .env(WORKER_MODE, mode)
                .env(WORKER_ROOT, root)
                .env(WORKER_CONTROL, control)
                .env(WORKER_ID, id)
                .env(WORKER_KEY, key)
                .env(WORKER_CEILING, ceiling.to_string())
                .env(WORKER_BOUNDARY, boundary)
                .env(WORKER_SOURCE, source)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap(),
        ))
    }

    fn finish(mut self) -> Output {
        let start = Instant::now();
        while self.0.as_mut().unwrap().try_wait().unwrap().is_none() {
            assert!(start.elapsed() < TIMEOUT, "Subprocess timed out");
            thread::sleep(Duration::from_millis(10));
        }
        self.0.take().unwrap().wait_with_output().unwrap()
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        if let Some(child) = self.0.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "Worker failed: {:?}\nstdout: {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stdout)
            .chars()
            .take(2000)
            .collect::<String>(),
        String::from_utf8_lossy(&output.stderr)
            .chars()
            .take(2000)
            .collect::<String>()
    );
}

fn bounded_debug(value: &impl std::fmt::Debug) -> String {
    format!("{value:?}").chars().take(1200).collect()
}

fn assert_crash(output: &Output, boundary: &str) {
    assert_eq!(
        output.status.code(),
        Some(73),
        "{boundary}: stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout)
            .chars()
            .take(2000)
            .collect::<String>(),
        String::from_utf8_lossy(&output.stderr)
            .chars()
            .take(2000)
            .collect::<String>()
    );
}

fn deferred_project(
    base: &Path,
) -> (
    PathBuf,
    PathBuf,
    DeferredGovernanceBasisV2,
    DeferralCommittedOutcomeV2,
) {
    let root = base.join("repository");
    let control = base.join("caller-retained-locators");
    fs::create_dir(&root).unwrap();
    fs::create_dir(&control).unwrap();
    let owner = ProjectAdmissionDeferralV2::new();
    let prepared = owner
        .prepare(
            &root,
            &DeferralRequestInputV2::human_approved(
                "initial",
                "Human approves deliberate deferral.",
            ),
        )
        .unwrap();
    let locator = prepared.locator().clone();
    let DeferralExecutionV2::Committed { outcome, .. } = owner.initialize(&root, prepared) else {
        panic!("Initial deferral failed");
    };
    let basis = owner
        .resolve_current(&root, &locator.project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
        .unwrap();
    write_json(&control.join("deferred.basis.json"), &basis);
    (root, control, basis, outcome)
}

fn approved_request(
    basis: &DeferredGovernanceBasisV2,
    key: &str,
    ceiling: u64,
) -> DeferredEstablishmentRequestV3 {
    DeferredEstablishmentRequestV3::human_approved(
        basis.project.clone(),
        ExpectedDeferredBasisV2 {
            outcome_fingerprint: basis.outcome_fingerprint.clone(),
            relation_fingerprint: basis.relation_fingerprint.clone(),
        },
        ceiling,
        key,
        "Human approves this exact configuration and supported terms.",
    )
    .unwrap()
}

fn assert_committed_recovery(
    root: &Path,
    locator: &DeferredEstablishmentLocatorV3,
    result: &DeferredEstablishmentExecutionV3,
) {
    let DeferredEstablishmentExecutionV3::Committed { outcome, delivery } = result else {
        panic!("Expected exact committed recovery: {result:?}");
    };
    assert_eq!(*delivery, GovernanceDeliveryV1::Replay);
    assert_eq!(&outcome.locator, locator);
    assert!(
        outcome.validation_confirmed
            && outcome.advisory_permission_accepted
            && outcome.activation_committed
    );
    let owner = ProjectAdmissionEstablishmentV3::new();
    assert_eq!(&owner.resolve_request(root, locator), result);
    let DeferredScopeBasisV3::EstablishedFromDeferredV3(current) = owner
        .resolve_current(
            root,
            &locator.request.project,
            ADMISSION_GOVERNANCE_SCOPE_V1,
            None,
        )
        .unwrap()
    else {
        panic!("Recovered current is not G");
    };
    assert_eq!(current.outcome_fingerprint, outcome.outcome_fingerprint);
    assert_eq!(current.project, locator.request.project);
    assert_eq!(current.configuration, locator.request.configuration);
}

#[test]
fn subprocess_crashes_at_every_durable_boundary_preserve_exact_request_resolution() {
    let boundaries: Vec<String> = [
        "selection-directory",
        "request-directory",
        "request",
        "selection",
    ]
    .into_iter()
    .map(str::to_owned)
    .chain((0..7).map(|i| format!("stage-{i}")))
    .chain(["intent", "decision"].into_iter().map(str::to_owned))
    .chain((0..7).map(|i| format!("installation-{i}")))
    .chain(
        [
            "current-prepared",
            "current-displaced",
            "current-published",
            "terminal",
            "closure",
            "reply",
        ]
        .into_iter()
        .map(str::to_owned),
    )
    .collect();
    for boundary in boundaries {
        let base = tempfile::tempdir().unwrap();
        let (root, control, basis, original) = deferred_project(base.path());
        let identity = fs::read(root.join(handbook_engine::REPOSITORY_IDENTITY_REPO_PATH)).unwrap();
        let creator = Worker::start(
            &root, &control, "original", "crash", "original", 7_200, &boundary, "",
        );
        assert_crash(&creator.finish(), &boundary);
        assert!(!control.join("original.result.json").exists());
        let locator = DeferredEstablishmentLocatorV3::from_json_bytes(
            &fs::read(control.join("original.locator.json")).unwrap(),
        )
        .unwrap();
        if boundary == "current-displaced" {
            assert!(!root.join(GOVERNANCE_CURRENT_PATH_V2).exists());
            assert!(root
                .join(transition_request_storage_path_v3(&locator))
                .join("current-displaced.json")
                .exists());
            assert!(root
                .join(transition_request_storage_path_v3(&locator))
                .join("current-prepared.json")
                .exists());
        }
        File::create(control.join("go")).unwrap();
        let resolver = Worker::start(
            &root,
            &control,
            "restarted",
            "resolve",
            "",
            0,
            "",
            "original",
        );
        assert_success(&resolver.finish());
        let result: DeferredEstablishmentExecutionV3 =
            read_json(&control.join("restarted.result.json"));
        let owner = ProjectAdmissionEstablishmentV3::new();
        match boundary.as_str() {
            "selection-directory" | "request-directory" | "request" => {
                assert!(
                    matches!(&result, DeferredEstablishmentExecutionV3::Uncertain { locator: actual, .. } if actual == &locator),
                    "{boundary}: {result:?}"
                );
                assert!(owner
                    .prepare(&root, &approved_request(&basis, "new", 86_400))
                    .is_err());
                assert!(owner
                    .resolve_current(&root, &basis.project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
                    .is_err());
            }
            "selection" | "intent" => {
                assert_noncommit_then_new_request(&root, &basis, &locator, &result)
            }
            name if name.starts_with("stage-") => {
                assert_noncommit_then_new_request(&root, &basis, &locator, &result)
            }
            _ => assert_committed_recovery(&root, &locator, &result),
        }
        assert_eq!(
            fs::read(root.join(handbook_engine::REPOSITORY_IDENTITY_REPO_PATH)).unwrap(),
            identity,
            "{boundary}"
        );
        if !matches!(
            boundary.as_str(),
            "selection-directory" | "request-directory" | "request"
        ) {
            assert_eq!(
                ProjectAdmissionDeferralV2::new().resolve_request(&root, &original.locator),
                DeferralExecutionV2::Committed {
                    outcome: original,
                    delivery: GovernanceDeliveryV1::Replay
                }
            );
        }
    }
}

fn assert_noncommit_then_new_request(
    root: &Path,
    basis: &DeferredGovernanceBasisV2,
    locator: &DeferredEstablishmentLocatorV3,
    result: &DeferredEstablishmentExecutionV3,
) {
    let DeferredEstablishmentExecutionV3::NotCommitted { outcome, delivery } = result else {
        panic!("Expected verified predecision noncommit: {result:?}");
    };
    assert_eq!(*delivery, GovernanceDeliveryV1::Replay);
    assert_eq!(&outcome.locator, locator);
    assert_eq!(outcome.condition, "verified_predecision_noncommit");
    assert!(!outcome.activation_committed);
    let owner = ProjectAdmissionEstablishmentV3::new();
    assert_eq!(&owner.resolve_request(root, locator), result);
    assert_eq!(
        owner
            .resolve_current(root, &basis.project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
            .unwrap(),
        DeferredScopeBasisV3::DeferredV2(basis.clone())
    );
    let prepared = owner
        .prepare(root, &approved_request(basis, "new", 86_400))
        .unwrap();
    assert!(matches!(
        owner.establish(root, prepared),
        DeferredEstablishmentExecutionV3::Committed { .. }
    ));
    assert_eq!(&owner.resolve_request(root, locator), result);
    assert!(ProjectAdmissionDeferralV2::new()
        .prepare(
            root,
            &DeferralRequestInputV2::human_approved("another-initial", "Human approval")
        )
        .is_err());
}

#[test]
fn process_contenders_commit_one_g_for_same_key_different_key_and_changed_content() {
    for (left_key, right_key, right_ceiling) in [
        ("same", "same", 7_200),
        ("first", "second", 86_400),
        ("same", "same", 86_400),
    ] {
        let base = tempfile::tempdir().unwrap();
        let (root, control, basis, _) = deferred_project(base.path());
        let left = Worker::start(
            &root,
            &control,
            "left",
            "establish",
            left_key,
            7_200,
            "",
            "",
        );
        let right = Worker::start(
            &root,
            &control,
            "right",
            "establish",
            right_key,
            right_ceiling,
            "",
            "",
        );
        wait_for_file(&control.join("left.ready"));
        wait_for_file(&control.join("right.ready"));
        let reader = Worker::start(&root, &control, "reader", "current", "", 0, "", "left");
        wait_for_file(&control.join("reader.ready"));
        File::create(control.join("go")).unwrap();
        assert_success(&left.finish());
        assert_success(&right.finish());
        assert_success(&reader.finish());
        let results: Vec<DeferredEstablishmentExecutionV3> = ["left", "right"]
            .into_iter()
            .map(|id| read_json(&control.join(format!("{id}.result.json"))))
            .collect();
        let originals: Vec<_> = results
            .iter()
            .filter_map(|result| match result {
                DeferredEstablishmentExecutionV3::Committed {
                    outcome,
                    delivery: GovernanceDeliveryV1::Original,
                } => Some(outcome),
                _ => None,
            })
            .collect();
        assert_eq!(originals.len(), 1, "{results:?}");
        let winner = originals[0];
        if left_key == right_key && right_ceiling == 7_200 {
            assert!(results.iter().any(|result| matches!(result, DeferredEstablishmentExecutionV3::Committed { outcome, delivery: GovernanceDeliveryV1::Replay } if outcome == winner)));
        } else {
            assert_eq!(
                results
                    .iter()
                    .filter(|result| matches!(
                        result,
                        DeferredEstablishmentExecutionV3::Refused { .. }
                    ))
                    .count(),
                1,
                "{results:?}"
            );
            if left_key == right_key {
                assert!(results.iter().any(|result| matches!(result, DeferredEstablishmentExecutionV3::Refused { refusal } if refusal.condition == "request_conflict")));
            }
        }
        let current: Result<DeferredScopeBasisV3, GovernanceRefusalV1> =
            read_json(&control.join("reader.result.json"));
        match current {
            Ok(DeferredScopeBasisV3::DeferredV2(actual)) => assert_eq!(actual, basis),
            Ok(DeferredScopeBasisV3::EstablishedFromDeferredV3(actual)) => {
                assert_eq!(actual.outcome_fingerprint, winner.outcome_fingerprint)
            }
            Err(refusal) => assert!(!refusal.condition.is_empty()),
        }
        assert_committed_recovery(
            &root,
            &winner.locator,
            &DeferredEstablishmentExecutionV3::Committed {
                outcome: winner.clone(),
                delivery: GovernanceDeliveryV1::Replay,
            },
        );
        assert_eq!(
            fs::read_dir(root.join(ESTABLISHMENT_SELECTION_PATH_V3))
                .unwrap()
                .count(),
            1
        );
    }
}

#[test]
fn concurrent_process_resolvers_and_current_reader_finish_only_the_displaced_decision() {
    let base = tempfile::tempdir().unwrap();
    let (root, control, _, _) = deferred_project(base.path());
    let creator = Worker::start(
        &root,
        &control,
        "original",
        "crash",
        "original",
        7_200,
        "current-displaced",
        "",
    );
    assert_crash(&creator.finish(), "current-displaced");
    assert!(!root.join(GOVERNANCE_CURRENT_PATH_V2).exists());
    let locator = DeferredEstablishmentLocatorV3::from_json_bytes(
        &fs::read(control.join("original.locator.json")).unwrap(),
    )
    .unwrap();
    let left = Worker::start(&root, &control, "left", "resolve", "", 0, "", "original");
    let right = Worker::start(&root, &control, "right", "resolve", "", 0, "", "original");
    let reader = Worker::start(&root, &control, "reader", "current", "", 0, "", "original");
    for id in ["left", "right", "reader"] {
        wait_for_file(&control.join(format!("{id}.ready")));
    }
    File::create(control.join("go")).unwrap();
    assert_success(&left.finish());
    assert_success(&right.finish());
    assert_success(&reader.finish());
    let left_result: DeferredEstablishmentExecutionV3 =
        read_json(&control.join("left.result.json"));
    let right_result: DeferredEstablishmentExecutionV3 =
        read_json(&control.join("right.result.json"));
    assert_eq!(left_result, right_result);
    assert_committed_recovery(&root, &locator, &left_result);
    let reader_result: Result<DeferredScopeBasisV3, GovernanceRefusalV1> =
        read_json(&control.join("reader.result.json"));
    let DeferredEstablishmentExecutionV3::Committed { outcome, .. } = left_result else {
        unreachable!()
    };
    match reader_result {
        Ok(DeferredScopeBasisV3::EstablishedFromDeferredV3(actual)) => {
            assert_eq!(actual.outcome_fingerprint, outcome.outcome_fingerprint)
        }
        Err(refusal) => assert!(!refusal.condition.is_empty()),
        other => panic!("Reader returned D across unresolved displacement: {other:?}"),
    }
    assert_eq!(
        fs::read_dir(root.join(ESTABLISHMENT_SELECTION_PATH_V3))
            .unwrap()
            .count(),
        1
    );
}

#[test]
fn interrupted_noncommit_terminal_and_closure_survive_concurrent_process_recovery() {
    for boundary in ["noncommit-terminal", "noncommit-closure"] {
        let base = tempfile::tempdir().unwrap();
        let (root, control, basis, _) = deferred_project(base.path());
        let creator = Worker::start(
            &root, &control, "original", "crash", "original", 7_200, "intent", "",
        );
        assert_crash(&creator.finish(), "intent");
        File::create(control.join("go")).unwrap();
        let interrupted = Worker::start(
            &root,
            &control,
            "interrupted",
            "resolve-crash",
            "",
            0,
            boundary,
            "original",
        );
        assert_crash(&interrupted.finish(), boundary);
        let locator = DeferredEstablishmentLocatorV3::from_json_bytes(
            &fs::read(control.join("original.locator.json")).unwrap(),
        )
        .unwrap();
        let left = Worker::start(&root, &control, "left", "resolve", "", 0, "", "original");
        let right = Worker::start(&root, &control, "right", "resolve", "", 0, "", "original");
        let reader = Worker::start(&root, &control, "reader", "current", "", 0, "", "original");
        assert_success(&left.finish());
        assert_success(&right.finish());
        assert_success(&reader.finish());
        let left_result: DeferredEstablishmentExecutionV3 =
            read_json(&control.join("left.result.json"));
        let right_result: DeferredEstablishmentExecutionV3 =
            read_json(&control.join("right.result.json"));
        assert_eq!(left_result, right_result);
        let reader_result: Result<DeferredScopeBasisV3, GovernanceRefusalV1> =
            read_json(&control.join("reader.result.json"));
        match reader_result {
            Ok(DeferredScopeBasisV3::DeferredV2(actual)) => assert_eq!(actual, basis),
            Err(refusal) => assert!(!refusal.condition.is_empty()),
            other => panic!("Predecision noncommit returned G: {other:?}"),
        }
        assert_noncommit_then_new_request(&root, &basis, &locator, &left_result);
    }
}

#[test]
fn paused_process_displacement_serializes_current_reader_and_request_recovery() {
    let base = tempfile::tempdir().unwrap();
    let (root, control, _, _) = deferred_project(base.path());
    let creator = Worker::start(
        &root,
        &control,
        "original",
        "pause",
        "original",
        7_200,
        "current-displaced",
        "",
    );
    wait_for_file(&control.join("original.ready"));
    File::create(control.join("go")).unwrap();
    wait_for_file(&control.join("original.paused"));
    assert!(!root.join(GOVERNANCE_CURRENT_PATH_V2).exists());
    let reader = Worker::start(&root, &control, "reader", "current", "", 0, "", "original");
    let resolver = Worker::start(
        &root, &control, "resolver", "resolve", "", 0, "", "original",
    );
    wait_for_file(&control.join("reader.entered"));
    wait_for_file(&control.join("resolver.entered"));
    // Give processes already at their owner call an opportunity to produce any
    // named unavailability while the real gap and native locks remain in place.
    thread::sleep(Duration::from_millis(50));
    if control.join("reader.done").exists() {
        let current: Result<DeferredScopeBasisV3, GovernanceRefusalV1> =
            read_json(&control.join("reader.result.json"));
        assert!(
            current.is_err(),
            "Reader supplied a usable current while displacement was paused: {current:?}"
        );
    }
    File::create(control.join("original.release")).unwrap();
    assert_success(&creator.finish());
    assert_success(&reader.finish());
    assert_success(&resolver.finish());
    let original: DeferredEstablishmentExecutionV3 =
        read_json(&control.join("original.result.json"));
    let DeferredEstablishmentExecutionV3::Committed {
        outcome,
        delivery: GovernanceDeliveryV1::Original,
    } = original
    else {
        panic!("Paused original did not commit");
    };
    let replay: DeferredEstablishmentExecutionV3 = read_json(&control.join("resolver.result.json"));
    assert_committed_recovery(&root, &outcome.locator, &replay);
    let current: Result<DeferredScopeBasisV3, GovernanceRefusalV1> =
        read_json(&control.join("reader.result.json"));
    match current {
        Ok(DeferredScopeBasisV3::EstablishedFromDeferredV3(actual)) => {
            assert_eq!(actual.outcome_fingerprint, outcome.outcome_fingerprint)
        }
        Err(refusal) => assert!(!refusal.condition.is_empty()),
        other => panic!("Paused gap reader returned wrong owner truth: {other:?}"),
    }
}
