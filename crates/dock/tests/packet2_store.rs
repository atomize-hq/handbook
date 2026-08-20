#![cfg(feature = "test-support")]

use handbook_contracts::{
    ContentAddressedBindingV1, ContractIdV1, EvaluationCaseV1, EvaluationSubjectV1,
    EvidenceCandidateSourceErrorKindV1, EvidenceCandidateSourceV1, EvidenceObservationV1,
    ExactBindingV1, ExactRefV1, FingerprintV1, MatcherInputValueV1, ResolutionEnvelopeV1,
    SemanticTimeV1, StableIdV1, VersionV1,
};
use handbook_dock::*;
use handbook_engine::{NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1};
use serde_json::json;
use std::fs;
use std::sync::{Arc, Barrier};
use std::thread;

fn fingerprint(hex: char) -> FingerprintV1 {
    FingerprintV1::parse(&format!("sha256:{}", hex.to_string().repeat(64))).unwrap()
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

fn limits() -> DockProtocolLimitsV1 {
    DockProtocolLimitsV1::new(DockProtocolLimitInputV1 {
        request_bytes: 1024 * 1024,
        result_bytes: 1024 * 1024,
        stderr_bytes: 4096,
        diagnostic_bytes: 1024,
        candidate_bytes: 64 * 1024,
        input_count: 8,
        candidate_count: 8,
        diagnostic_count: 8,
        wall_timeout_ms: 10_000,
        cancellation_grace_ms: 1_000,
    })
    .unwrap()
}

fn manifest() -> DockManifestV1 {
    let bundle = DockImplementationBundleV1::new(DockImplementationBundleInputV1 {
        bundle_ref: ExactRefV1::parse("handbook.dock.bundle.fixture@1.0.0").unwrap(),
        files: vec![DockBundleFileV1::new(
            SafeRelativePathV1::parse("bin/dock-helper").unwrap(),
            4096,
            fingerprint('a'),
        )
        .unwrap()],
        entrypoint: SafeRelativePathV1::parse("bin/dock-helper").unwrap(),
        entrypoint_fingerprint: fingerprint('a'),
        fixed_arguments: vec!["--protocol".into(), "process-json-v1".into()],
        target: DockTargetTupleV1::parse("x86_64-pc-windows-msvc").unwrap(),
    })
    .unwrap();
    DockManifestV1::new(DockManifestInputV1 {
        manifest_schema: binding(DOCK_MANIFEST_SCHEMA_REF_V1, '1'),
        dock_id: ContractIdV1::parse("handbook.dock.fixture").unwrap(),
        dock_version: VersionV1::parse("1.2.3").unwrap(),
        protocol: binding(DOCK_PROCESS_PROTOCOL_REF_V1, '2'),
        request_schema: binding(DOCK_PROCESS_REQUEST_SCHEMA_REF_V1, '3'),
        result_schema: binding(DOCK_PROCESS_RESULT_SCHEMA_REF_V1, '4'),
        implementation_kind: DockImplementationKindV1::NativeProcess,
        implementation_bundle: bundle,
        supported_evidence_kinds: vec![stable_id("handbook.evidence.schema-conformance")],
        supported_inputs: vec![DockInputRoleV1::new(
            stable_id("input.instance"),
            "application/json",
            binding("handbook.schema.instance@1.0.0", '5'),
        )
        .unwrap()],
        resolution_ceiling: resolution(),
        limits: limits(),
    })
    .unwrap()
}

fn request_with_payload(payload: serde_json::Value) -> DockProcessRequestV1 {
    let subject_binding =
        ContentAddressedBindingV1::new("subject.fixture#revision/exact", fingerprint('6')).unwrap();
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
            subject_binding.clone(),
            vec![MatcherInputValueV1::new(stable_id("subject.kind"), "fixture").unwrap()],
        )
        .unwrap(),
        cases: vec![EvaluationCaseV1::new(
            stable_id("case.positive"),
            subject_binding,
            vec![MatcherInputValueV1::new(stable_id("case.kind"), "positive").unwrap()],
        )
        .unwrap()],
        requested_resolution: resolution(),
        semantic_time_utc: SemanticTimeV1::parse("2026-08-20T12:00:00Z").unwrap(),
        lifecycle_head_fingerprint: fingerprint('8'),
    })
    .unwrap();
    DockProcessRequestV1::new(
        &manifest(),
        DockProcessRequestInputV1 {
            run,
            requested_evidence_kinds: vec![stable_id("handbook.evidence.schema-conformance")],
            inputs: vec![DockInlineJsonInputV1::new(
                stable_id("input.instance"),
                "application/json",
                binding("handbook.schema.instance@1.0.0", '5'),
                payload,
            )
            .unwrap()],
            effective_limits: limits(),
        },
    )
    .unwrap()
}

fn evaluation_run_jcs(request: &DockProcessRequestV1) -> Vec<u8> {
    serde_json_canonicalizer::to_vec(&json!({
        "evaluation_run_fingerprint": request.run().evaluation_run().fingerprint(),
        "evaluation_run_ref": request.run().evaluation_run().reference(),
        "schema_ref": "handbook.contract-evaluation-run@1.0.0",
    }))
    .unwrap()
}

fn completed(
    payload: serde_json::Value,
    observation: EvidenceObservationV1,
) -> (
    DockProcessRequestV1,
    DockProcessResultV1,
    DockExecutionRecordV1,
) {
    let request = request_with_payload(payload);
    let candidate = DockCandidateObservationV1::new_with_observation(
        stable_id("claim.schema-valid"),
        stable_id("case.positive"),
        stable_id("handbook.evidence.schema-conformance"),
        binding("handbook.fact.schema-conformance@1.0.0", 'b'),
        json!({"valid": observation == EvidenceObservationV1::Satisfied}),
        observation,
    )
    .unwrap();
    let result = DockProcessResultV1::completed(
        &request,
        DockClaimPartitionV1::new(vec![stable_id("claim.schema-valid")], vec![], vec![]).unwrap(),
        vec![candidate],
        vec![],
        resolution(),
    )
    .unwrap();
    let record = DockExecutionRecordV1::new(DockExecutionRecordInputV1 {
        request: &request,
        result: Some(&result),
        facts: DockHostOutcomeFactsV1 {
            cleanup_confirmed: true,
            durability_confirmed: true,
            deadline_reached: false,
            cancellation_accepted: false,
            spawn_or_process_failed: false,
            framing_or_binding_failed: false,
            child_status: Some(DockProcessResultStatusV1::Completed),
        },
        exit_code: Some(0),
        wall_time_ms: 10,
        stdout_bytes: result.framed_json_bytes().unwrap().len() as u64,
        stdout_truncated: false,
        stderr_bytes: 0,
        stderr_truncated: false,
        diagnostics: vec![],
    })
    .unwrap();
    (request, result, record)
}

#[test]
fn completed_commit_is_atomic_replayable_stable_and_noncyclic() {
    let repo = tempfile::tempdir().unwrap();
    let store = DockOperationalStoreV1::open(repo.path()).unwrap();
    let (request, result, execution) = completed(
        json!({"instance": "first"}),
        EvidenceObservationV1::Satisfied,
    );

    let committed = store
        .commit(
            &request,
            Some(&result),
            &execution,
            &evaluation_run_jcs(&request),
            "dock-run-secret-key-0001",
        )
        .unwrap();
    assert_eq!(committed.disposition(), DockStoreDispositionV1::Committed);
    assert_eq!(committed.candidates().len(), 1);
    assert_eq!(committed.execution_record(), &execution);
    assert_ne!(
        committed.candidates()[0].fingerprint(),
        execution.record_fingerprint(),
        "execution closes over observations before final candidate identity"
    );
    let first_root = committed.root().clone();

    let replay = store
        .commit(
            &request,
            Some(&result),
            &execution,
            &evaluation_run_jcs(&request),
            "dock-run-secret-key-0001",
        )
        .unwrap();
    assert_eq!(replay.disposition(), DockStoreDispositionV1::Replayed);
    assert_eq!(replay.root(), &first_root);
    assert_eq!(replay.candidates(), committed.candidates());

    let persisted = all_files(repo.path());
    assert!(!String::from_utf8_lossy(&persisted).contains("dock-run-secret-key-0001"));
    assert!(!String::from_utf8_lossy(&persisted).contains(&repo.path().display().to_string()));
}

#[test]
fn conflicting_key_request_and_concurrent_replay_write_once() {
    let repo = tempfile::tempdir().unwrap();
    let store = Arc::new(DockOperationalStoreV1::open(repo.path()).unwrap());
    let (request, result, execution) = completed(
        json!({"instance": "same"}),
        EvidenceObservationV1::Satisfied,
    );
    let barrier = Arc::new(Barrier::new(8));
    let mut joins = Vec::new();
    for _ in 0..8 {
        let store = Arc::clone(&store);
        let barrier = Arc::clone(&barrier);
        let request = request.clone();
        let result = result.clone();
        let execution = execution.clone();
        joins.push(thread::spawn(move || {
            barrier.wait();
            store.commit(
                &request,
                Some(&result),
                &execution,
                &evaluation_run_jcs(&request),
                "dock-concurrent-key-0001",
            )
        }));
    }
    let receipts: Vec<_> = joins
        .into_iter()
        .map(|join| join.join().unwrap().unwrap())
        .collect();
    assert_eq!(
        receipts
            .iter()
            .filter(|receipt| receipt.disposition() == DockStoreDispositionV1::Committed)
            .count(),
        1
    );
    assert_eq!(
        receipts
            .iter()
            .filter(|receipt| receipt.disposition() == DockStoreDispositionV1::Replayed)
            .count(),
        7
    );
    assert!(receipts
        .windows(2)
        .all(|pair| pair[0].root() == pair[1].root()));

    let (other_request, other_result, other_execution) = completed(
        json!({"instance": "other"}),
        EvidenceObservationV1::Violated,
    );
    let refusal = store
        .commit(
            &other_request,
            Some(&other_result),
            &other_execution,
            &evaluation_run_jcs(&other_request),
            "dock-concurrent-key-0001",
        )
        .unwrap_err();
    assert_eq!(refusal.kind(), DockStoreErrorKindV1::IdempotencyConflict);
}

#[test]
fn interrupted_commit_recovers_to_the_exact_original_receipt() {
    for fault in [
        DockStoreFaultBoundaryV1::AfterIntent,
        DockStoreFaultBoundaryV1::AfterFirstInstall,
        DockStoreFaultBoundaryV1::BeforeCommitMarker,
    ] {
        let repo = tempfile::tempdir().unwrap();
        let store = DockOperationalStoreV1::open(repo.path()).unwrap();
        let (request, result, execution) = completed(
            json!({"fault": format!("{fault:?}")}),
            EvidenceObservationV1::Satisfied,
        );
        assert_eq!(
            store
                .commit_with_fault_for_testing(
                    &request,
                    Some(&result),
                    &execution,
                    &evaluation_run_jcs(&request),
                    "dock-recovery-key-0001",
                    fault,
                )
                .unwrap_err()
                .kind(),
            DockStoreErrorKindV1::DurabilityUncertain
        );

        let recovered = DockOperationalStoreV1::open(repo.path()).unwrap();
        let replay = recovered
            .commit(
                &request,
                Some(&result),
                &execution,
                &evaluation_run_jcs(&request),
                "dock-recovery-key-0001",
            )
            .unwrap();
        assert_eq!(replay.disposition(), DockStoreDispositionV1::Replayed);
        assert_eq!(replay.execution_record(), &execution);
        assert_eq!(replay.candidates().len(), 1);
    }
}

#[test]
fn missing_tampered_partial_surplus_duplicate_and_ambiguous_inventory_refuse() {
    for mutation in 0..8 {
        let repo = tempfile::tempdir().unwrap();
        let store = DockOperationalStoreV1::open(repo.path()).unwrap();
        let (request, result, execution) = completed(
            json!({"mutation": mutation}),
            EvidenceObservationV1::Satisfied,
        );
        store
            .commit(
                &request,
                Some(&result),
                &execution,
                &evaluation_run_jcs(&request),
                "dock-inventory-key-0001",
            )
            .unwrap();
        let root = repo.path().join(DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1);
        let files = regular_files(&root);
        match mutation {
            0 => fs::remove_file(
                files
                    .iter()
                    .find(|path| path.to_string_lossy().contains("executions"))
                    .unwrap(),
            )
            .unwrap(),
            1 => fs::write(
                files
                    .iter()
                    .find(|path| path.to_string_lossy().contains("requests"))
                    .unwrap(),
                b"tampered",
            )
            .unwrap(),
            2 => {
                let transaction = transaction_directory(&root);
                fs::remove_file(transaction.join("committed.json")).unwrap();
                fs::remove_file(
                    files
                        .iter()
                        .find(|path| path.to_string_lossy().contains("results"))
                        .unwrap(),
                )
                .unwrap();
            }
            3 => fs::write(root.join("surplus.json"), b"{}").unwrap(),
            4 => {
                let transaction = transaction_directory(&root);
                fs::copy(
                    transaction.join("intent.json"),
                    transaction.join("duplicate.json"),
                )
                .unwrap();
            }
            5 => fs::create_dir(root.join("transactions/ambiguous")).unwrap(),
            6 => fs::write(
                files
                    .iter()
                    .find(|path| path.to_string_lossy().contains("candidates"))
                    .unwrap(),
                b"tampered-candidate",
            )
            .unwrap(),
            7 => {
                let candidate = files
                    .iter()
                    .find(|path| path.to_string_lossy().contains("candidates"))
                    .unwrap();
                fs::copy(
                    candidate,
                    root.join(format!("candidates/{}.json", "f".repeat(64))),
                )
                .unwrap();
            }
            _ => unreachable!(),
        }
        assert!(
            DockOperationalStoreV1::open(repo.path()).is_err(),
            "mutation {mutation} admitted"
        );
    }
}

#[test]
fn only_clean_completed_durable_records_materialize_candidates_and_zero_is_valid() {
    let repo = tempfile::tempdir().unwrap();
    let store = DockOperationalStoreV1::open(repo.path()).unwrap();
    let request = request_with_payload(json!({"zero": true}));
    let result = DockProcessResultV1::completed(
        &request,
        DockClaimPartitionV1::new(vec![], vec![stable_id("claim.schema-valid")], vec![]).unwrap(),
        vec![],
        vec![],
        resolution(),
    )
    .unwrap();
    let record = DockExecutionRecordV1::new(DockExecutionRecordInputV1 {
        request: &request,
        result: Some(&result),
        facts: DockHostOutcomeFactsV1 {
            cleanup_confirmed: true,
            durability_confirmed: true,
            deadline_reached: false,
            cancellation_accepted: false,
            spawn_or_process_failed: false,
            framing_or_binding_failed: false,
            child_status: Some(DockProcessResultStatusV1::Completed),
        },
        exit_code: Some(0),
        wall_time_ms: 10,
        stdout_bytes: result.framed_json_bytes().unwrap().len() as u64,
        stdout_truncated: false,
        stderr_bytes: 0,
        stderr_truncated: false,
        diagnostics: vec![],
    })
    .unwrap();
    let receipt = store
        .commit(
            &request,
            Some(&result),
            &record,
            &evaluation_run_jcs(&request),
            "dock-zero-key-0001",
        )
        .unwrap();
    assert!(receipt.candidates().is_empty());

    let refused_request = request_with_payload(json!({"refused": true}));
    let refused_result = DockProcessResultV1::refused(
        &refused_request,
        DockProcessRefusalReasonV1::ObservationUnavailable,
        vec![],
    )
    .unwrap();
    let refused_record = DockExecutionRecordV1::new(DockExecutionRecordInputV1 {
        request: &refused_request,
        result: Some(&refused_result),
        facts: DockHostOutcomeFactsV1 {
            cleanup_confirmed: true,
            durability_confirmed: true,
            deadline_reached: false,
            cancellation_accepted: false,
            spawn_or_process_failed: false,
            framing_or_binding_failed: false,
            child_status: Some(DockProcessResultStatusV1::Refused),
        },
        exit_code: Some(0),
        wall_time_ms: 10,
        stdout_bytes: refused_result.framed_json_bytes().unwrap().len() as u64,
        stdout_truncated: false,
        stderr_bytes: 0,
        stderr_truncated: false,
        diagnostics: vec![],
    })
    .unwrap();
    let refused = store
        .commit(
            &refused_request,
            Some(&refused_result),
            &refused_record,
            &evaluation_run_jcs(&refused_request),
            "dock-refused-key-0001",
        )
        .unwrap();
    assert!(refused.candidates().is_empty());
}

struct UnavailablePort;

impl NativeAuthenticatorPortV1 for UnavailablePort {
    fn make_credential(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        Err(NativeAuthenticatorPortErrorV1::Unavailable)
    }

    fn get_assertion(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        Err(NativeAuthenticatorPortErrorV1::Unavailable)
    }
}

#[test]
fn adapter_requires_same_retained_root_and_contracts_source_authority_readmission() {
    let repo = tempfile::tempdir().unwrap();
    let other = tempfile::tempdir().unwrap();
    let store = DockOperationalStoreV1::open(repo.path()).unwrap();
    let (request, result, execution) =
        completed(json!({"source": true}), EvidenceObservationV1::Satisfied);
    let receipt = store
        .commit(
            &request,
            Some(&result),
            &execution,
            &evaluation_run_jcs(&request),
            "dock-source-key-0001",
        )
        .unwrap();
    let candidate = receipt.candidates()[0].clone();

    let mut source = DockEvidenceCandidateSourceV1::open(repo.path(), UnavailablePort).unwrap();
    assert_eq!(
        source.resolve_exact(&candidate, |_| ()).unwrap_err().kind(),
        EvidenceCandidateSourceErrorKindV1::AuthorityAbsent,
        "Dock cannot bypass Contracts source-authority admission"
    );
    let mut wrong_root =
        DockEvidenceCandidateSourceV1::open(other.path(), UnavailablePort).unwrap();
    assert_eq!(
        wrong_root
            .resolve_exact(&candidate, |_| ())
            .unwrap_err()
            .kind(),
        EvidenceCandidateSourceErrorKindV1::Missing
    );

    let source_text = include_str!("../src/store.rs");
    assert!(!source_text.contains("EvidenceRecordV1"));
    assert!(!source_text.contains("ClaimVerdictV1"));
    assert!(!source_text.contains("GateResult"));
}

fn regular_files(root: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut pending = vec![root.to_path_buf()];
    let mut files = Vec::new();
    while let Some(path) = pending.pop() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                pending.push(entry.path());
            } else {
                files.push(entry.path());
            }
        }
    }
    files
}

fn transaction_directory(root: &std::path::Path) -> std::path::PathBuf {
    fs::read_dir(root.join("transactions"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path()
}

fn all_files(root: &std::path::Path) -> Vec<u8> {
    regular_files(root)
        .into_iter()
        .flat_map(|path| fs::read(path).unwrap())
        .collect()
}
