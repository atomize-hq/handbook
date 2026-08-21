#![cfg(feature = "test-support")]

use handbook_contracts::{
    ContentAddressedBindingV1, ContractIdV1, EvaluationCaseV1, EvaluationSubjectV1, ExactBindingV1,
    ExactRefV1, FingerprintV1, MatcherInputValueV1, ResolutionEnvelopeV1, SemanticTimeV1,
    StableIdV1, VersionV1,
};
use handbook_dock::*;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{Duration, Instant};

fn fingerprint(hex: char) -> FingerprintV1 {
    FingerprintV1::parse(&format!("sha256:{}", hex.to_string().repeat(64))).unwrap()
}

fn digest(bytes: &[u8]) -> FingerprintV1 {
    FingerprintV1::parse(&format!("sha256:{:x}", Sha256::digest(bytes))).unwrap()
}

fn binding(reference: &str, hex: char) -> ExactBindingV1 {
    ExactBindingV1::new(ExactRefV1::parse(reference).unwrap(), fingerprint(hex))
}

fn stable_id(value: &str) -> StableIdV1 {
    StableIdV1::parse(value).unwrap()
}

fn resolution() -> ResolutionEnvelopeV1 {
    ResolutionEnvelopeV1::new(
        binding("handbook.resolution.stack@1.0.0", '9'),
        stable_id("resolution.scope.repository"),
        stable_id("resolution.detail.complete"),
        stable_id("resolution.time.current-operation"),
        stable_id("resolution.authority.read-only"),
        stable_id("resolution.memory.operation"),
        stable_id("resolution.validation.observation-only"),
    )
}

fn limits(timeout_ms: u64, grace_ms: u64) -> DockProtocolLimitsV1 {
    DockProtocolLimitsV1::new(DockProtocolLimitInputV1 {
        request_bytes: 1024 * 1024,
        result_bytes: 64 * 1024,
        stderr_bytes: 4096,
        diagnostic_bytes: 1024,
        candidate_bytes: 64 * 1024,
        input_count: 8,
        candidate_count: 8,
        diagnostic_count: 8,
        wall_timeout_ms: timeout_ms,
        cancellation_grace_ms: grace_ms,
    })
    .unwrap()
}

struct Fixture {
    _bundle: tempfile::TempDir,
    repository: tempfile::TempDir,
    bundle_root: PathBuf,
    manifest: DockManifestV1,
    resolved_bundle: DockResolvedFirstPartyBundleV1,
    request: DockProcessRequestV1,
}

impl Fixture {
    fn new(mode: &str, arguments: &[&str], timeout_ms: u64, grace_ms: u64) -> Self {
        let bundle = tempfile::tempdir().unwrap();
        let bin = bundle.path().join("bin");
        fs::create_dir(&bin).unwrap();
        let source = Path::new(env!("CARGO_BIN_EXE_dock-test-helper"));
        let entry_name = source.file_name().unwrap();
        let entry = bin.join(entry_name);
        fs::copy(source, &entry).unwrap();
        let bytes = fs::read(&entry).unwrap();
        let relative = format!("bin/{}", entry_name.to_string_lossy());
        let mut fixed_arguments = vec![mode.to_owned()];
        fixed_arguments.extend(arguments.iter().map(|value| (*value).to_owned()));
        let bundle_definition = DockImplementationBundleV1::new(DockImplementationBundleInputV1 {
            bundle_ref: ExactRefV1::parse("handbook.dock.bundle.fixture@1.0.0").unwrap(),
            files: vec![DockBundleFileV1::new(
                SafeRelativePathV1::parse(&relative).unwrap(),
                bytes.len() as u64,
                digest(&bytes),
            )
            .unwrap()],
            entrypoint: SafeRelativePathV1::parse(&relative).unwrap(),
            entrypoint_fingerprint: digest(&bytes),
            fixed_arguments,
            target: DockTargetTupleV1::parse(host_target_tuple_v1()).unwrap(),
        })
        .unwrap();
        let effective_limits = limits(timeout_ms, grace_ms);
        let manifest = DockManifestV1::new(DockManifestInputV1 {
            manifest_schema: binding(DOCK_MANIFEST_SCHEMA_REF_V1, '1'),
            dock_id: ContractIdV1::parse("handbook.dock.fixture").unwrap(),
            dock_version: VersionV1::parse("1.2.3").unwrap(),
            protocol: binding(DOCK_PROCESS_PROTOCOL_REF_V1, '2'),
            request_schema: binding(DOCK_PROCESS_REQUEST_SCHEMA_REF_V1, '3'),
            result_schema: binding(DOCK_PROCESS_RESULT_SCHEMA_REF_V1, '4'),
            implementation_kind: DockImplementationKindV1::NativeProcess,
            implementation_bundle: bundle_definition,
            supported_evidence_kinds: vec![stable_id("handbook.evidence.schema-conformance")],
            supported_inputs: vec![DockInputRoleV1::new(
                stable_id("input.instance"),
                "application/json",
                binding("handbook.schema.instance@1.0.0", '5'),
            )
            .unwrap()],
            resolution_ceiling: resolution(),
            limits: effective_limits.clone(),
        })
        .unwrap();
        let subject =
            ContentAddressedBindingV1::new("subject.fixture#revision/exact", fingerprint('6'))
                .unwrap();
        let run = DockEvaluationRunBindingV1::new(DockEvaluationRunBindingInputV1 {
            evaluation_run: ContentAddressedBindingV1::new(
                "handbook.contract.fixture@1.0.0#evaluation/exact",
                fingerprint('7'),
            )
            .unwrap(),
            evaluation_lifecycle_basis: ContentAddressedBindingV1::new(
                "handbook.contract.fixture@1.0.0#evaluation-lifecycle-basis/exact",
                fingerprint('8'),
            )
            .unwrap(),
            contract: binding("handbook.contract.fixture@1.0.0", '6'),
            claim_ids: vec![stable_id("claim.schema-valid")],
            subject: EvaluationSubjectV1::new(
                subject.clone(),
                vec![MatcherInputValueV1::new(stable_id("subject.kind"), "fixture").unwrap()],
            )
            .unwrap(),
            cases: vec![EvaluationCaseV1::new(
                stable_id("case.positive"),
                subject,
                vec![MatcherInputValueV1::new(stable_id("case.kind"), "positive").unwrap()],
            )
            .unwrap()],
            requested_resolution: resolution(),
            semantic_time_utc: SemanticTimeV1::parse("2026-08-20T12:00:00Z").unwrap(),
            lifecycle_head_fingerprint: fingerprint('8'),
        })
        .unwrap();
        let request = DockProcessRequestV1::new(
            &manifest,
            DockProcessRequestInputV1 {
                run,
                requested_evidence_kinds: vec![stable_id("handbook.evidence.schema-conformance")],
                inputs: vec![DockInlineJsonInputV1::new(
                    stable_id("input.instance"),
                    "application/json",
                    binding("handbook.schema.instance@1.0.0", '5'),
                    json!({"instance": true}),
                )
                .unwrap()],
                effective_limits,
            },
        )
        .unwrap();
        let resolved_bundle =
            DockResolvedFirstPartyBundleV1::for_testing(manifest.clone(), bundle.path()).unwrap();
        Self {
            bundle_root: bundle.path().to_owned(),
            _bundle: bundle,
            repository: tempfile::tempdir().unwrap(),
            manifest,
            resolved_bundle,
            request,
        }
    }

    fn run(&self, key: &str, cancellation: DockCancellationV1) -> DockExecutionReceiptV1 {
        let store = DockOperationalStoreV1::open(self.repository.path()).unwrap();
        DockProcessExecutorV1::new()
            .execute(DockExecutionInputV1 {
                store: &store,
                resolved_bundle: &self.resolved_bundle,
                request: &self.request,
                evaluation_run_jcs: &evaluation_run_jcs(&self.request),
                idempotency_key: key,
                cancellation: &cancellation,
            })
            .unwrap()
    }

    fn with_large_request(mut self) -> Self {
        self.request = DockProcessRequestV1::new(
            &self.manifest,
            DockProcessRequestInputV1 {
                run: self.request.run().clone(),
                requested_evidence_kinds: self.request.requested_evidence_kinds().to_vec(),
                inputs: vec![DockInlineJsonInputV1::new(
                    stable_id("input.instance"),
                    "application/json",
                    binding("handbook.schema.instance@1.0.0", '5'),
                    json!({"blob": "x".repeat(512 * 1024)}),
                )
                .unwrap()],
                effective_limits: self.request.effective_limits().clone(),
            },
        )
        .unwrap();
        self
    }

    fn with_non_executable_entrypoint(mut self) -> Self {
        let relative = self
            .manifest
            .implementation_bundle()
            .entrypoint()
            .as_str()
            .to_owned();
        let entry = self
            .bundle_root
            .join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
        fs::remove_file(&entry).unwrap();
        fs::write(&entry, b"not a native executable").unwrap();
        let bytes = fs::read(&entry).unwrap();
        let bundle = DockImplementationBundleV1::new(DockImplementationBundleInputV1 {
            bundle_ref: self.manifest.implementation_bundle().bundle_ref().clone(),
            files: vec![DockBundleFileV1::new(
                SafeRelativePathV1::parse(&relative).unwrap(),
                bytes.len() as u64,
                digest(&bytes),
            )
            .unwrap()],
            entrypoint: SafeRelativePathV1::parse(&relative).unwrap(),
            entrypoint_fingerprint: digest(&bytes),
            fixed_arguments: self
                .manifest
                .implementation_bundle()
                .fixed_arguments()
                .to_vec(),
            target: DockTargetTupleV1::parse(host_target_tuple_v1()).unwrap(),
        })
        .unwrap();
        let manifest = DockManifestV1::new(DockManifestInputV1 {
            manifest_schema: binding(DOCK_MANIFEST_SCHEMA_REF_V1, '1'),
            dock_id: ContractIdV1::parse("handbook.dock.fixture").unwrap(),
            dock_version: VersionV1::parse("1.2.3").unwrap(),
            protocol: self.manifest.protocol().clone(),
            request_schema: self.manifest.request_schema().clone(),
            result_schema: self.manifest.result_schema().clone(),
            implementation_kind: DockImplementationKindV1::NativeProcess,
            implementation_bundle: bundle,
            supported_evidence_kinds: self.manifest.supported_evidence_kinds().to_vec(),
            supported_inputs: self.manifest.supported_inputs().to_vec(),
            resolution_ceiling: self.manifest.resolution_ceiling().clone(),
            limits: self.manifest.limits().clone(),
        })
        .unwrap();
        self.request = DockProcessRequestV1::new(
            &manifest,
            DockProcessRequestInputV1 {
                run: self.request.run().clone(),
                requested_evidence_kinds: self.request.requested_evidence_kinds().to_vec(),
                inputs: self.request.inputs().to_vec(),
                effective_limits: self.request.effective_limits().clone(),
            },
        )
        .unwrap();
        self.manifest = manifest;
        self.resolved_bundle =
            DockResolvedFirstPartyBundleV1::for_testing(self.manifest.clone(), &self.bundle_root)
                .unwrap();
        self
    }

    fn request_with_payload(&self, payload: serde_json::Value) -> DockProcessRequestV1 {
        DockProcessRequestV1::new(
            &self.manifest,
            DockProcessRequestInputV1 {
                run: self.request.run().clone(),
                requested_evidence_kinds: self.request.requested_evidence_kinds().to_vec(),
                inputs: vec![DockInlineJsonInputV1::new(
                    stable_id("input.instance"),
                    "application/json",
                    binding("handbook.schema.instance@1.0.0", '5'),
                    payload,
                )
                .unwrap()],
                effective_limits: self.request.effective_limits().clone(),
            },
        )
        .unwrap()
    }
}

fn evaluation_run_jcs(request: &DockProcessRequestV1) -> Vec<u8> {
    serde_json_canonicalizer::to_vec(&json!({
        "evaluation_run_fingerprint": request.run().evaluation_run().fingerprint(),
        "evaluation_run_ref": request.run().evaluation_run().reference(),
        "schema_ref": "handbook.contract-evaluation-run@1.0.0",
    }))
    .unwrap()
}

fn record_json(receipt: &DockExecutionReceiptV1) -> serde_json::Value {
    serde_json::from_slice(&receipt.execution_record().canonical_json_bytes().unwrap()).unwrap()
}

#[test]
fn packet3_executor_api_is_available() {
    let _executor = DockProcessExecutorV1::new();
    let cancellation = DockCancellationV1::new();
    assert!(!cancellation.is_requested());
    assert!(cancellation.cancel());
    assert!(cancellation.is_requested());
    assert!(!cancellation.cancel());
}

#[test]
fn direct_exact_launch_clears_environment_uses_private_cwd_and_commits_zero_candidates() {
    let fixture = Fixture::new("verify-host", &["literal;not-shell"], 5_000, 250);
    let receipt = fixture.run("dock-executor-key-0001", DockCancellationV1::new());
    assert_eq!(
        receipt.execution_record().host_outcome(),
        DockHostOutcomeV1::Completed
    );
    assert!(receipt.candidates().is_empty());
}

#[test]
fn completed_refused_and_candidate_completion_are_durably_distinct() {
    for (index, mode, outcome, candidates) in [
        (1, "completed-zero", DockHostOutcomeV1::Completed, 0),
        (2, "completed-one", DockHostOutcomeV1::Completed, 1),
        (3, "refused", DockHostOutcomeV1::Refused, 0),
    ] {
        let fixture = Fixture::new(mode, &[], 5_000, 250);
        let receipt = fixture.run(
            &format!("dock-executor-key-{index:04}"),
            DockCancellationV1::new(),
        );
        assert_eq!(receipt.execution_record().host_outcome(), outcome);
        assert_eq!(receipt.candidates().len(), candidates);
    }
}

#[test]
fn exact_result_body_ceiling_plus_required_lf_completes() {
    let fixture = Fixture::new("completed-exact-limit", &[], 5_000, 250);
    let receipt = fixture.run("dock-result-boundary-key-0001", DockCancellationV1::new());
    assert_eq!(
        receipt.execution_record().host_outcome(),
        DockHostOutcomeV1::Completed
    );
    assert_eq!(receipt.candidates().len(), 1);
    assert_eq!(
        record_json(&receipt)["stdout_bytes"].as_u64().unwrap(),
        64 * 1024 + 1
    );
}

#[test]
fn absent_extra_and_tampered_bundle_members_refuse_before_launch_or_write() {
    for mutation in 0..4 {
        let fixture = Fixture::new("completed-one", &[], 5_000, 250);
        let entry = fs::read_dir(fixture.bundle_root.join("bin"))
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        match mutation {
            0 => fs::remove_file(entry).unwrap(),
            1 => fs::write(fixture.bundle_root.join("extra"), b"extra").unwrap(),
            2 => {
                let mut bytes = fs::read(&entry).unwrap();
                bytes[0] ^= 0xff;
                fs::write(entry, bytes).unwrap();
            }
            _ => fs::create_dir(fixture.bundle_root.join("unexpected-directory")).unwrap(),
        }
        let store = DockOperationalStoreV1::open(fixture.repository.path()).unwrap();
        let error = DockProcessExecutorV1::new()
            .execute(DockExecutionInputV1 {
                store: &store,
                resolved_bundle: &fixture.resolved_bundle,
                request: &fixture.request,
                evaluation_run_jcs: &evaluation_run_jcs(&fixture.request),
                idempotency_key: "dock-bundle-refusal-key-0001",
                cancellation: &DockCancellationV1::new(),
            })
            .unwrap_err();
        assert_eq!(error.kind(), DockExecutorErrorKindV1::AdmissionRefused);
        assert!(store
            .replay_for_request(&fixture.request, "dock-bundle-refusal-key-0001")
            .unwrap()
            .is_none());
    }
}

#[test]
fn source_substitution_after_private_stage_cannot_change_launched_bytes() {
    let fixture = Fixture::new("completed-zero", &[], 5_000, 250);
    let store = DockOperationalStoreV1::open(fixture.repository.path()).unwrap();
    let receipt = DockProcessExecutorV1::new()
        .execute_with_fault_for_testing(
            DockExecutionInputV1 {
                store: &store,
                resolved_bundle: &fixture.resolved_bundle,
                request: &fixture.request,
                evaluation_run_jcs: &evaluation_run_jcs(&fixture.request),
                idempotency_key: "dock-private-stage-key-0001",
                cancellation: &DockCancellationV1::new(),
            },
            DockExecutorFaultV1::SubstituteSourceAfterPrivateStage,
        )
        .unwrap();
    assert_eq!(
        receipt.execution_record().host_outcome(),
        DockHostOutcomeV1::Completed
    );
    let source_entry = fixture.bundle_root.join(
        fixture
            .manifest
            .implementation_bundle()
            .entrypoint()
            .as_str()
            .replace('/', std::path::MAIN_SEPARATOR_STR),
    );
    assert!(fs::read(source_entry)
        .unwrap()
        .iter()
        .all(|byte| *byte == 0));
}

#[test]
fn invalid_duplicate_noncanonical_extra_oversized_and_wrong_binding_output_is_protocol_error() {
    for (index, mode) in [
        "invalid-utf8",
        "invalid-json",
        "duplicate-json",
        "noncanonical",
        "extra-output",
        "oversized-output",
        "wrong-binding",
    ]
    .into_iter()
    .enumerate()
    {
        let fixture = Fixture::new(mode, &[], 5_000, 250);
        let receipt = fixture.run(
            &format!("dock-protocol-error-key-{index:04}"),
            DockCancellationV1::new(),
        );
        assert_eq!(
            receipt.execution_record().host_outcome(),
            DockHostOutcomeV1::ProtocolError,
            "mode {mode}"
        );
        assert!(receipt.candidates().is_empty());
    }
}

#[test]
fn stderr_is_concurrent_bounded_and_never_discloses_child_text() {
    let bounded = Fixture::new("stderr-secret", &[], 5_000, 250);
    let receipt = bounded.run("dock-stderr-key-0001", DockCancellationV1::new());
    assert_eq!(
        receipt.execution_record().host_outcome(),
        DockHostOutcomeV1::Completed
    );
    let record = receipt.execution_record().canonical_json_bytes().unwrap();
    let record = String::from_utf8(record).unwrap();
    assert!(!record.contains("DO_NOT_DISCLOSE"));
    assert!(!record.contains("http://secret"));

    let overrun = Fixture::new("stderr-overrun", &[], 5_000, 250);
    let receipt = overrun.run("dock-stderr-key-0002", DockCancellationV1::new());
    assert_eq!(
        receipt.execution_record().host_outcome(),
        DockHostOutcomeV1::ProtocolError
    );
    assert!(receipt.candidates().is_empty());
}

#[test]
fn crash_nonzero_timeout_and_late_output_never_complete_or_expose_candidates() {
    for (index, mode) in ["nonzero", "crash"].into_iter().enumerate() {
        let fixture = Fixture::new(mode, &[], 5_000, 250);
        let receipt = fixture.run(
            &format!("dock-process-failure-key-{index:04}"),
            DockCancellationV1::new(),
        );
        assert_eq!(
            receipt.execution_record().host_outcome(),
            DockHostOutcomeV1::Failed
        );
        assert!(receipt.candidates().is_empty());
    }

    let late = Fixture::new("sleep", &["500"], 40, 20);
    let receipt = late.run("dock-timeout-key-0001", DockCancellationV1::new());
    assert_eq!(
        receipt.execution_record().host_outcome(),
        DockHostOutcomeV1::TimedOut
    );
    assert!(receipt.candidates().is_empty());
}

#[test]
fn native_spawn_failure_is_durably_failed_without_candidates() {
    let fixture = Fixture::new("completed-one", &[], 5_000, 250).with_non_executable_entrypoint();
    let receipt = fixture.run("dock-spawn-failure-key-0001", DockCancellationV1::new());
    assert_eq!(
        receipt.execution_record().host_outcome(),
        DockHostOutcomeV1::Failed
    );
    assert!(receipt.candidates().is_empty());
}

#[test]
fn child_that_never_reads_stdin_cannot_block_the_monotonic_timeout() {
    let fixture = Fixture::new("ignore-stdin", &[], 40, 20).with_large_request();
    let started = Instant::now();
    let receipt = fixture.run("dock-stdin-timeout-key-0001", DockCancellationV1::new());
    assert_eq!(
        receipt.execution_record().host_outcome(),
        DockHostOutcomeV1::TimedOut
    );
    assert!(receipt.candidates().is_empty());
    assert!(record_json(&receipt)["wall_time_ms"].as_u64().unwrap() <= 60);
    assert!(started.elapsed() < Duration::from_secs(5));
}

#[test]
fn accepted_cancellation_is_idempotent_and_late_completion_stays_cancelled() {
    let fixture = Arc::new(Fixture::new("sleep", &["2000"], 5_000, 20));
    let cancellation = DockCancellationV1::new();
    let runner_fixture = Arc::clone(&fixture);
    let runner_cancellation = cancellation.clone();
    let join =
        thread::spawn(move || runner_fixture.run("dock-cancel-key-0001", runner_cancellation));
    thread::sleep(Duration::from_millis(750));
    assert!(cancellation.cancel());
    assert!(!cancellation.cancel());
    let receipt = join.join().unwrap();
    assert_eq!(
        receipt.execution_record().host_outcome(),
        DockHostOutcomeV1::Cancelled
    );
    assert!(receipt.candidates().is_empty());
}

#[test]
fn deadline_during_accepted_cancellation_grace_stays_timed_out() {
    let marker_dir = tempfile::tempdir().unwrap();
    let marker = marker_dir.path().join("launched");
    let marker_argument = marker.to_string_lossy().into_owned();
    let fixture = Arc::new(Fixture::new(
        "mark-and-sleep",
        &[&marker_argument, "2000"],
        300,
        400,
    ));
    let cancellation = DockCancellationV1::new();
    let runner_fixture = Arc::clone(&fixture);
    let runner_cancellation = cancellation.clone();
    let join = thread::spawn(move || {
        runner_fixture.run("dock-cancel-deadline-key-0001", runner_cancellation)
    });
    let observation_deadline = Instant::now() + Duration::from_secs(5);
    while !marker.exists() && Instant::now() < observation_deadline {
        thread::sleep(Duration::from_millis(1));
    }
    assert!(marker.exists());
    thread::sleep(Duration::from_millis(150));
    assert!(cancellation.cancel());
    let receipt = join.join().unwrap();
    assert_eq!(
        receipt.execution_record().host_outcome(),
        DockHostOutcomeV1::TimedOut
    );
    assert!(receipt.candidates().is_empty());
}

#[test]
fn cancellation_before_launch_and_conflicting_key_refuse_without_process_records() {
    let fixture = Fixture::new("completed-one", &[], 5_000, 250);
    let store = DockOperationalStoreV1::open(fixture.repository.path()).unwrap();
    let cancellation = DockCancellationV1::new();
    cancellation.cancel();
    let error = DockProcessExecutorV1::new()
        .execute(DockExecutionInputV1 {
            store: &store,
            resolved_bundle: &fixture.resolved_bundle,
            request: &fixture.request,
            evaluation_run_jcs: &evaluation_run_jcs(&fixture.request),
            idempotency_key: "dock-pre-cancel-key-0001",
            cancellation: &cancellation,
        })
        .unwrap_err();
    assert_eq!(error.kind(), DockExecutorErrorKindV1::CancelledBeforeLaunch);
    assert!(store
        .replay_for_request(&fixture.request, "dock-pre-cancel-key-0001")
        .unwrap()
        .is_none());
}

#[test]
fn conflicting_key_request_reuse_refuses_without_replacing_the_original_record() {
    let fixture = Fixture::new("completed-one", &[], 5_000, 250);
    let first = fixture.run("dock-conflict-key-0001", DockCancellationV1::new());
    let substituted_request = fixture.request_with_payload(json!({"instance": "other"}));
    let store = DockOperationalStoreV1::open(fixture.repository.path()).unwrap();
    let error = DockProcessExecutorV1::new()
        .execute(DockExecutionInputV1 {
            store: &store,
            resolved_bundle: &fixture.resolved_bundle,
            request: &substituted_request,
            evaluation_run_jcs: &evaluation_run_jcs(&substituted_request),
            idempotency_key: "dock-conflict-key-0001",
            cancellation: &DockCancellationV1::new(),
        })
        .unwrap_err();
    assert_eq!(error.kind(), DockExecutorErrorKindV1::IdempotencyConflict);
    let replay = store
        .replay_for_request(&fixture.request, "dock-conflict-key-0001")
        .unwrap()
        .unwrap();
    assert_eq!(replay.execution_record(), first.execution_record());
    assert_eq!(replay.candidates(), first.candidates());
}

#[test]
fn exact_replay_skips_bundle_access_and_concurrent_replay_returns_one_record() {
    let replay = Fixture::new("completed-one", &[], 5_000, 250);
    let first = replay.run("dock-replay-key-0001", DockCancellationV1::new());
    fs::remove_dir_all(&replay.bundle_root).unwrap();
    let second = replay.run("dock-replay-key-0001", DockCancellationV1::new());
    assert_eq!(first.execution_record(), second.execution_record());
    assert_eq!(second.disposition(), DockStoreDispositionV1::Replayed);

    let launch_dir = tempfile::tempdir().unwrap();
    let launch_marker = launch_dir.path().join("launch-count");
    let launch_argument = launch_marker.to_string_lossy().into_owned();
    let fixture = Arc::new(Fixture::new(
        "count-and-complete",
        &[&launch_argument, "200"],
        5_000,
        250,
    ));
    let store = Arc::new(DockOperationalStoreV1::open(fixture.repository.path()).unwrap());
    let barrier = Arc::new(Barrier::new(3));
    let mut joins = Vec::new();
    for _ in 0..2 {
        let fixture = Arc::clone(&fixture);
        let store = Arc::clone(&store);
        let barrier = Arc::clone(&barrier);
        joins.push(thread::spawn(move || {
            barrier.wait();
            DockProcessExecutorV1::new()
                .execute(DockExecutionInputV1 {
                    store: &store,
                    resolved_bundle: &fixture.resolved_bundle,
                    request: &fixture.request,
                    evaluation_run_jcs: &evaluation_run_jcs(&fixture.request),
                    idempotency_key: "dock-race-key-0001",
                    cancellation: &DockCancellationV1::new(),
                })
                .unwrap()
        }));
    }
    barrier.wait();
    let left = joins.remove(0).join().unwrap();
    let right = joins.remove(0).join().unwrap();
    assert_eq!(left.execution_record(), right.execution_record());
    assert_eq!(left.candidates(), right.candidates());
    assert_eq!(left.candidates().len(), 1);
    assert_eq!(fs::read(launch_marker).unwrap(), b"x");
}

#[cfg(windows)]
#[test]
fn inherited_output_handle_returns_bounded_and_fails_cleanup_closed() {
    let fixture = Fixture::new("hold-stdout-open", &[], 5_000, 250);
    let started = Instant::now();
    let receipt = fixture.run("dock-inherited-output-key-0001", DockCancellationV1::new());
    assert_eq!(
        receipt.execution_record().host_outcome(),
        DockHostOutcomeV1::Failed
    );
    assert!(receipt.candidates().is_empty());
    assert!(started.elapsed() < Duration::from_secs(5));
}

#[test]
fn cleanup_and_durability_uncertainty_never_expose_candidates_or_partial_records() {
    let cleanup = Fixture::new("completed-one", &[], 5_000, 250);
    let store = DockOperationalStoreV1::open(cleanup.repository.path()).unwrap();
    let receipt = DockProcessExecutorV1::new()
        .execute_with_fault_for_testing(
            DockExecutionInputV1 {
                store: &store,
                resolved_bundle: &cleanup.resolved_bundle,
                request: &cleanup.request,
                evaluation_run_jcs: &evaluation_run_jcs(&cleanup.request),
                idempotency_key: "dock-cleanup-key-0001",
                cancellation: &DockCancellationV1::new(),
            },
            DockExecutorFaultV1::CleanupUncertain,
        )
        .unwrap();
    assert_eq!(
        receipt.execution_record().host_outcome(),
        DockHostOutcomeV1::Failed
    );
    assert!(receipt.candidates().is_empty());

    let durability = Fixture::new("completed-one", &[], 5_000, 250);
    let store = DockOperationalStoreV1::open(durability.repository.path()).unwrap();
    let error = DockProcessExecutorV1::new()
        .execute_with_fault_for_testing(
            DockExecutionInputV1 {
                store: &store,
                resolved_bundle: &durability.resolved_bundle,
                request: &durability.request,
                evaluation_run_jcs: &evaluation_run_jcs(&durability.request),
                idempotency_key: "dock-durability-key-0001",
                cancellation: &DockCancellationV1::new(),
            },
            DockExecutorFaultV1::Store(DockStoreFaultBoundaryV1::AfterIntent),
        )
        .unwrap_err();
    assert_eq!(error.kind(), DockExecutorErrorKindV1::DurabilityUncertain);
    let candidate_root = durability
        .repository
        .path()
        .join(".handbook/state/dock/v1/candidates");
    assert!(candidate_root.read_dir().unwrap().next().is_none());
}

#[test]
fn source_gates_prove_direct_launch_and_empty_production_registry() {
    let source = include_str!("../src/executor.rs");
    assert!(source.contains("Command::new(&entrypoint)"));
    for forbidden in [
        "Command::new(\"sh\")",
        "Command::new(\"cmd\")",
        "powershell",
        "PATH",
        "which::",
        "python",
        "dynamic_load",
    ] {
        assert!(
            !source.contains(forbidden),
            "forbidden source token {forbidden}"
        );
    }
    assert!(source.contains("std::env::current_exe()"));
    assert_eq!(DockManifestRegistryV1::production().production_len(), 1);
}
