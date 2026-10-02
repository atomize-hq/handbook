#![cfg(feature = "test-support")]

use handbook_contracts::project_admission_governance::*;
use serde::{de::DeserializeOwned, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const WORKER_MODE: &str = "HCM_NATIVE_GOV_TEST_MODE";
const WORKER_ROOT: &str = "HCM_NATIVE_GOV_TEST_ROOT";
const WORKER_CONTROL: &str = "HCM_NATIVE_GOV_TEST_CONTROL";
const WORKER_ID: &str = "HCM_NATIVE_GOV_TEST_ID";
const WORKER_KEY: &str = "HCM_NATIVE_GOV_TEST_KEY";
const WORKER_BOUNDARY: &str = "HCM_NATIVE_GOV_TEST_BOUNDARY";
const WORKER_SOURCE: &str = "HCM_NATIVE_GOV_TEST_SOURCE";
const TIMEOUT: Duration = Duration::from_secs(20);

/// The subprocess terminates at real owner durability boundaries while the
/// native lock remains held. Process exit, rather than dropping Rust guards,
/// must release it before the restarted resolver can finish the exact request.
#[test]
fn admission_governance_process_worker() {
    let Ok(mode) = std::env::var(WORKER_MODE) else {
        return;
    };
    let root = PathBuf::from(std::env::var_os(WORKER_ROOT).unwrap());
    let control = PathBuf::from(std::env::var_os(WORKER_CONTROL).unwrap());
    let id = std::env::var(WORKER_ID).unwrap();
    let owner = ProjectAdmissionGovernanceV1::new();
    let result = match mode.as_str() {
        "initialize" | "crash" => {
            let key = std::env::var(WORKER_KEY).unwrap();
            let request = GovernanceRequestInputV1::standalone_freshness(
                7200,
                key,
                "Establish this fresh local project",
            );
            let prepared = owner.prepare(&root, &request).unwrap();
            // This caller-controlled locator is flushed before the first owner
            // effect; no fresh token or identity is recreated after a crash.
            write_json(
                &control.join(format!("{id}.locator.json")),
                prepared.locator(),
            );
            if mode == "crash" {
                let boundary = parse_boundary(&std::env::var(WORKER_BOUNDARY).unwrap());
                let result = owner
                    .with_crash_for_testing(boundary)
                    .initialize(&root, prepared);
                panic!("crash boundary returned without process exit: {result:?}");
            }
            rendezvous(&control, &id);
            owner.initialize(&root, prepared)
        }
        "resolve" => {
            let source = std::env::var(WORKER_SOURCE).unwrap();
            let locator: GovernanceRequestLocatorV1 =
                read_json(&control.join(format!("{source}.locator.json")));
            rendezvous(&control, &id);
            owner.resolve_request(&root, &locator)
        }
        other => panic!("unknown worker mode {other}"),
    };
    write_json(&control.join(format!("{id}.result.json")), &result);
}

fn parse_boundary(name: &str) -> GovernanceFaultBoundaryV1 {
    match name {
        "reservation" => GovernanceFaultBoundaryV1::Reservation,
        "anchor" => GovernanceFaultBoundaryV1::Anchor,
        "identity" => GovernanceFaultBoundaryV1::Identity,
        "registration" => GovernanceFaultBoundaryV1::Registration,
        "intent" => GovernanceFaultBoundaryV1::Intent,
        "decision" => GovernanceFaultBoundaryV1::Decision,
        "terminal" => GovernanceFaultBoundaryV1::Terminal,
        "reply" => GovernanceFaultBoundaryV1::Reply,
        other => {
            if let Some(index) = other.strip_prefix("stage-") {
                GovernanceFaultBoundaryV1::Stage(index.parse().unwrap())
            } else if let Some(index) = other.strip_prefix("installation-") {
                GovernanceFaultBoundaryV1::Installation(index.parse().unwrap())
            } else {
                panic!("unknown boundary {name}");
            }
        }
    }
}

fn write_json(path: &Path, value: &impl Serialize) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    file.write_all(&serde_json::to_vec(value).unwrap()).unwrap();
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
            "worker rendezvous timed out: {path:?}"
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
        boundary: &str,
        source: &str,
    ) -> Self {
        Self(Some(
            Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "admission_governance_process_worker",
                    "--nocapture",
                    "--test-threads=1",
                ])
                .env(WORKER_MODE, mode)
                .env(WORKER_ROOT, root)
                .env(WORKER_CONTROL, control)
                .env(WORKER_ID, id)
                .env(WORKER_KEY, key)
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
            assert!(start.elapsed() < TIMEOUT, "subprocess timed out");
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
        "worker failed: {:?}\nstdout: {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn fresh_paths(base: &Path) -> (PathBuf, PathBuf) {
    let root = base.join("repository");
    let control = base.join("caller-retained-locators");
    fs::create_dir(&root).unwrap();
    fs::create_dir(&control).unwrap();
    (root, control)
}

#[test]
fn concurrent_process_creators_commit_once_for_same_and_different_request_keys() {
    for keys in [["same", "same"], ["first", "second"]] {
        let base = tempfile::tempdir().unwrap();
        let (root, control) = fresh_paths(base.path());
        let left = Worker::start(&root, &control, "left", "initialize", keys[0], "", "");
        let right = Worker::start(&root, &control, "right", "initialize", keys[1], "", "");
        wait_for_file(&control.join("left.ready"));
        wait_for_file(&control.join("right.ready"));
        File::create(control.join("go")).unwrap();
        assert_success(&left.finish());
        assert_success(&right.finish());
        let outcomes: Vec<GovernanceExecutionV1> = ["left", "right"]
            .into_iter()
            .map(|id| read_json(&control.join(format!("{id}.result.json"))))
            .collect();
        let committed: Vec<_> = outcomes
            .iter()
            .filter_map(|outcome| match outcome {
                GovernanceExecutionV1::Committed { outcome, delivery } => {
                    assert_eq!(*delivery, GovernanceDeliveryV1::Original);
                    Some(outcome)
                }
                GovernanceExecutionV1::Refused { refusal } => {
                    assert_eq!(refusal.condition, "initial_registration_unavailable");
                    None
                }
                other => panic!("competing creation returned {other:?}"),
            })
            .collect();
        assert_eq!(committed.len(), 1, "{outcomes:?}");
        let winner = committed[0];
        let owner = ProjectAdmissionGovernanceV1::new();
        let active = owner
            .resolve_active(&root, &winner.locator.project, GOVERNANCE_SCOPE_V1, None)
            .unwrap();
        assert_eq!(active.outcome_fingerprint, winner.outcome_fingerprint);
        assert_eq!(
            owner.resolve_request(&root, &winner.locator),
            GovernanceExecutionV1::Committed {
                outcome: winner.clone(),
                delivery: GovernanceDeliveryV1::Replay
            }
        );
    }
}

#[test]
fn concurrent_process_resolvers_replay_one_exact_original_outcome() {
    let base = tempfile::tempdir().unwrap();
    let (root, control) = fresh_paths(base.path());
    let owner = ProjectAdmissionGovernanceV1::new();
    let request =
        GovernanceRequestInputV1::standalone_freshness(7200, "original", "Establish project");
    let prepared = owner.prepare(&root, &request).unwrap();
    let locator = prepared.locator().clone();
    write_json(&control.join("original.locator.json"), &locator);
    let result = owner.initialize(&root, prepared);
    let GovernanceExecutionV1::Committed { outcome, .. } = result else {
        panic!("{result:?}");
    };
    let left = Worker::start(&root, &control, "left", "resolve", "", "", "original");
    let right = Worker::start(&root, &control, "right", "resolve", "", "", "original");
    wait_for_file(&control.join("left.ready"));
    wait_for_file(&control.join("right.ready"));
    File::create(control.join("go")).unwrap();
    assert_success(&left.finish());
    assert_success(&right.finish());
    let expected = GovernanceExecutionV1::Committed {
        outcome,
        delivery: GovernanceDeliveryV1::Replay,
    };
    for id in ["left", "right"] {
        assert_eq!(
            read_json::<GovernanceExecutionV1>(&control.join(format!("{id}.result.json"))),
            expected
        );
    }
}

#[test]
fn crashes_release_held_locks_and_restart_resolves_only_the_original_request() {
    let boundaries: Vec<String> = ["reservation", "anchor", "identity", "registration"]
        .into_iter()
        .map(str::to_owned)
        .chain((0..5).map(|i| format!("stage-{i}")))
        .chain(["intent", "decision"].into_iter().map(str::to_owned))
        .chain((0..5).map(|i| format!("installation-{i}")))
        .chain(["terminal", "reply"].into_iter().map(str::to_owned))
        .collect();
    for boundary in boundaries {
        let base = tempfile::tempdir().unwrap();
        let (root, control) = fresh_paths(base.path());
        let creator = Worker::start(
            &root, &control, "original", "crash", "original", &boundary, "",
        );
        let output = creator.finish();
        assert_eq!(output.status.code(), Some(71), "{boundary}: {output:?}");
        assert!(!control.join("original.result.json").exists());
        let locator: GovernanceRequestLocatorV1 = read_json(&control.join("original.locator.json"));
        File::create(control.join("go")).unwrap();
        let resolver = Worker::start(&root, &control, "restarted", "resolve", "", "", "original");
        assert_success(&resolver.finish());
        let recovered: GovernanceExecutionV1 = read_json(&control.join("restarted.result.json"));
        let owner = ProjectAdmissionGovernanceV1::new();
        match boundary.as_str() {
            "reservation" => {
                assert!(
                    matches!(&recovered, GovernanceExecutionV1::Uncertain { locator: original, .. } if original == &locator),
                    "{boundary}: {recovered:?}"
                );
                assert!(!root.join(GOVERNANCE_ANCHOR_PATH_V1).exists());
            }
            "decision" | "terminal" | "reply" => {
                assert_committed_recovery(&root, &locator, &recovered)
            }
            name if name.starts_with("installation-") => {
                assert_committed_recovery(&root, &locator, &recovered)
            }
            _ => {
                let GovernanceExecutionV1::NotCommitted { outcome, delivery } = &recovered else {
                    panic!("{boundary}: {recovered:?}");
                };
                assert_eq!(*delivery, GovernanceDeliveryV1::Replay);
                assert_eq!(&outcome.locator, &locator);
                assert_eq!(outcome.condition, "verified_predecision_noncommit");
                assert!(!outcome.activation_committed);
                assert_eq!(owner.resolve_request(&root, &locator), recovered);
                assert!(owner
                    .resolve_active(&root, &locator.project, GOVERNANCE_SCOPE_V1, None)
                    .is_err());
            }
        }
        // Every reserved root is consumed for creation, including a reservation
        // without an anchor. A new token cannot become a retry or second project.
        let request = GovernanceRequestInputV1::standalone_freshness(
            7200,
            "replacement",
            "Establish project",
        );
        let refused = owner.prepare(&root, &request).unwrap_err();
        assert_eq!(
            refused.condition, "initial_registration_unavailable",
            "{boundary}"
        );
    }
}

#[test]
fn concurrent_process_resolvers_finish_one_decided_transaction_without_duplicate_activation() {
    let base = tempfile::tempdir().unwrap();
    let (root, control) = fresh_paths(base.path());
    let creator = Worker::start(
        &root, &control, "original", "crash", "original", "decision", "",
    );
    let crashed = creator.finish();
    assert_eq!(crashed.status.code(), Some(71), "{crashed:?}");
    let locator: GovernanceRequestLocatorV1 = read_json(&control.join("original.locator.json"));
    let left = Worker::start(&root, &control, "left", "resolve", "", "", "original");
    let right = Worker::start(&root, &control, "right", "resolve", "", "", "original");
    wait_for_file(&control.join("left.ready"));
    wait_for_file(&control.join("right.ready"));
    File::create(control.join("go")).unwrap();
    assert_success(&left.finish());
    assert_success(&right.finish());
    let left: GovernanceExecutionV1 = read_json(&control.join("left.result.json"));
    let right: GovernanceExecutionV1 = read_json(&control.join("right.result.json"));
    assert_eq!(left, right);
    assert_committed_recovery(&root, &locator, &left);
}

fn assert_committed_recovery(
    root: &Path,
    locator: &GovernanceRequestLocatorV1,
    recovered: &GovernanceExecutionV1,
) {
    let GovernanceExecutionV1::Committed { outcome, delivery } = recovered else {
        panic!("{recovered:?}");
    };
    assert_eq!(*delivery, GovernanceDeliveryV1::Replay);
    assert_eq!(&outcome.locator, locator);
    assert!(
        outcome.validation_confirmed
            && outcome.advisory_permission_accepted
            && outcome.activation_committed
    );
    let owner = ProjectAdmissionGovernanceV1::new();
    assert_eq!(&owner.resolve_request(root, locator), recovered);
    let active = owner
        .resolve_active(root, &locator.project, GOVERNANCE_SCOPE_V1, None)
        .unwrap();
    assert_eq!(active.outcome_fingerprint, outcome.outcome_fingerprint);
    assert_eq!(
        active.configuration.bindings[0].max_observation_age_seconds,
        7200
    );
}

#[test]
fn copied_governance_and_identity_refuse_an_updated_path_locator() {
    let base = tempfile::tempdir().unwrap();
    let (root, _) = fresh_paths(base.path());
    let owner = ProjectAdmissionGovernanceV1::new();
    let request =
        GovernanceRequestInputV1::standalone_freshness(7200, "original", "Establish project");
    let prepared = owner.prepare(&root, &request).unwrap();
    let locator = prepared.locator().clone();
    assert!(matches!(
        owner.initialize(&root, prepared),
        GovernanceExecutionV1::Committed { .. }
    ));
    let copied = base.path().join("copied");
    copy_tree(&root, &copied);
    let mut substituted = locator.project;
    substituted.retained_root = copied.to_str().unwrap().to_owned();
    let refused = owner
        .resolve_active(&copied, &substituted, GOVERNANCE_SCOPE_V1, None)
        .unwrap_err();
    assert_eq!(refused.condition, "physical_root_substituted");
}

fn copy_tree(source: &Path, target: &Path) {
    fs::create_dir(target).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let destination = target.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &destination);
        } else {
            fs::copy(entry.path(), destination).unwrap();
        }
    }
}
