use handbook_contracts::{
    ContentAddressedBindingV1, ContractIdV1, EvaluationCaseV1, EvaluationSubjectV1, ExactBindingV1,
    ExactRefV1, FingerprintV1, MatcherInputValueV1, ResolutionEnvelopeV1, SemanticTimeV1,
    StableIdV1, VersionV1,
};
use handbook_dock::*;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

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

fn bundle() -> DockImplementationBundleV1 {
    DockImplementationBundleV1::new(DockImplementationBundleInputV1 {
        bundle_ref: ExactRefV1::parse("handbook.dock.bundle.fixture@1.0.0").unwrap(),
        files: vec![
            DockBundleFileV1::new(
                SafeRelativePathV1::parse("bin/dock-helper").unwrap(),
                4096,
                fingerprint('a'),
            )
            .unwrap(),
            DockBundleFileV1::new(
                SafeRelativePathV1::parse("share/schema.json").unwrap(),
                1024,
                fingerprint('b'),
            )
            .unwrap(),
        ],
        entrypoint: SafeRelativePathV1::parse("bin/dock-helper").unwrap(),
        entrypoint_fingerprint: fingerprint('a'),
        fixed_arguments: vec!["--protocol".into(), "process-json-v1".into()],
        target: DockTargetTupleV1::parse("x86_64-pc-windows-msvc").unwrap(),
    })
    .unwrap()
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
    DockManifestV1::new(DockManifestInputV1 {
        manifest_schema: binding(DOCK_MANIFEST_SCHEMA_REF_V1, '1'),
        dock_id: ContractIdV1::parse("handbook.dock.fixture").unwrap(),
        dock_version: VersionV1::parse("1.2.3").unwrap(),
        protocol: binding(DOCK_PROCESS_PROTOCOL_REF_V1, '2'),
        request_schema: binding(DOCK_PROCESS_REQUEST_SCHEMA_REF_V1, '3'),
        result_schema: binding(DOCK_PROCESS_RESULT_SCHEMA_REF_V1, '4'),
        implementation_kind: DockImplementationKindV1::NativeProcess,
        implementation_bundle: bundle(),
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

fn run_binding() -> DockEvaluationRunBindingV1 {
    let subject_binding =
        ContentAddressedBindingV1::new("subject.fixture#revision/exact", fingerprint('6')).unwrap();
    let subject = EvaluationSubjectV1::new(
        subject_binding.clone(),
        vec![MatcherInputValueV1::new(stable_id("subject.kind"), "fixture").unwrap()],
    )
    .unwrap();
    let case = EvaluationCaseV1::new(
        stable_id("case.positive"),
        subject_binding,
        vec![MatcherInputValueV1::new(stable_id("case.kind"), "positive").unwrap()],
    )
    .unwrap();
    DockEvaluationRunBindingV1::new(DockEvaluationRunBindingInputV1 {
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
        subject,
        cases: vec![case],
        requested_resolution: resolution(),
        semantic_time_utc: SemanticTimeV1::parse("2026-08-20T12:00:00Z").unwrap(),
        lifecycle_head_fingerprint: fingerprint('8'),
    })
    .unwrap()
}

fn request() -> DockProcessRequestV1 {
    let manifest = manifest();
    DockProcessRequestV1::new(
        &manifest,
        DockProcessRequestInputV1 {
            run: run_binding(),
            requested_evidence_kinds: vec![stable_id("handbook.evidence.schema-conformance")],
            inputs: vec![DockInlineJsonInputV1::new(
                stable_id("input.instance"),
                "application/json",
                binding("handbook.schema.instance@1.0.0", '5'),
                json!({"z": 1, "a": [true, null]}),
            )
            .unwrap()],
            effective_limits: limits(),
        },
    )
    .unwrap()
}

#[test]
fn exact_owner_protocol_schema_manifest_bundle_and_launch_identities_are_fixed() {
    let manifest = manifest();
    assert_eq!(DOCK_OWNER_SOURCE_REF_V1, "handbook.owner.dock@0.1.0");
    assert_eq!(
        DOCK_PROCESS_PROTOCOL_REF_V1,
        "handbook.dock.process-json@1.0.0"
    );
    assert_eq!(DOCK_MANIFEST_SCHEMA_REF_V1, "handbook.dock-manifest@1.0.0");
    assert_eq!(
        DOCK_PROCESS_REQUEST_SCHEMA_REF_V1,
        "handbook.dock-process-request@1.0.0"
    );
    assert_eq!(
        DOCK_PROCESS_RESULT_SCHEMA_REF_V1,
        "handbook.dock-process-result@1.0.0"
    );
    assert_eq!(
        DOCK_EXECUTION_RECORD_SCHEMA_REF_V1,
        "handbook.dock-execution-record@1.0.0"
    );
    assert_eq!(manifest.dock_ref().as_str(), "handbook.dock.fixture@1.2.3");
    assert_eq!(
        manifest.implementation_bundle().entrypoint().as_str(),
        "bin/dock-helper"
    );
    assert_eq!(
        manifest.implementation_bundle().fixed_arguments(),
        ["--protocol", "process-json-v1"]
    );
    assert_eq!(
        manifest.implementation_bundle().target().as_str(),
        "x86_64-pc-windows-msvc"
    );
}

#[test]
fn canonical_bytes_and_fingerprints_are_stable_and_close_over_substitution() {
    let manifest = manifest();
    let bytes = manifest.canonical_json_bytes().unwrap();
    assert_eq!(bytes, serde_json_canonicalizer::to_vec(&manifest).unwrap());
    assert_eq!(
        DockManifestV1::admit_canonical_json(&bytes).unwrap(),
        manifest
    );

    let mut substituted = serde_json::to_value(&manifest).unwrap();
    substituted["implementation_bundle"]["fixed_arguments"][1] = json!("other");
    assert!(serde_json::from_value::<DockManifestV1>(substituted).is_err());

    let mut unknown = serde_json::to_value(&manifest).unwrap();
    unknown["verdict"] = json!("pass");
    assert!(serde_json::from_value::<DockManifestV1>(unknown).is_err());

    let mut wrong_version = serde_json::to_value(&manifest).unwrap();
    wrong_version["protocol"]["ref"] = json!("handbook.dock.process-json@1.0.1");
    assert!(serde_json::from_value::<DockManifestV1>(wrong_version).is_err());
}

#[test]
fn bundle_inventory_is_portable_safe_relative_ordered_and_bounded() {
    for unsafe_path in [
        "",
        "/root",
        "../escape",
        "a/../b",
        "a//b",
        "a\\b",
        "C:/host",
        ".",
        "a/./b",
        "bin/CON",
        "bin/com1.exe",
        "bin/trailing.",
        "bin/trailing ",
        "bin/control\nname",
        "bin/wild?card",
    ] {
        assert!(
            SafeRelativePathV1::parse(unsafe_path).is_err(),
            "accepted {unsafe_path:?}"
        );
    }
    assert!(SafeRelativePathV1::parse("share/schema.json").is_ok());

    let duplicate = DockImplementationBundleV1::new(DockImplementationBundleInputV1 {
        bundle_ref: ExactRefV1::parse("handbook.dock.bundle.fixture@1.0.0").unwrap(),
        files: vec![
            DockBundleFileV1::new(
                SafeRelativePathV1::parse("bin/tool").unwrap(),
                1,
                fingerprint('a'),
            )
            .unwrap(),
            DockBundleFileV1::new(
                SafeRelativePathV1::parse("bin/tool").unwrap(),
                1,
                fingerprint('a'),
            )
            .unwrap(),
        ],
        entrypoint: SafeRelativePathV1::parse("bin/tool").unwrap(),
        entrypoint_fingerprint: fingerprint('a'),
        fixed_arguments: vec![],
        target: DockTargetTupleV1::parse("x86_64-unknown-linux-gnu").unwrap(),
    });
    assert_eq!(duplicate.unwrap_err(), DockErrorV1::InvalidInvariant);

    let mut substituted_entrypoint = serde_json::to_value(bundle()).unwrap();
    substituted_entrypoint["entrypoint_fingerprint"] = json!(fingerprint('f').as_str());
    assert!(serde_json::from_value::<DockImplementationBundleV1>(substituted_entrypoint).is_err());
}

#[test]
fn protocol_limits_enforce_every_fixed_hard_ceiling() {
    let mut over = DockProtocolLimitInputV1 {
        request_bytes: MAX_PROCESS_JSON_BYTES + 1,
        result_bytes: 1,
        stderr_bytes: 1,
        diagnostic_bytes: 1,
        candidate_bytes: 1,
        input_count: 1,
        candidate_count: 1,
        diagnostic_count: 1,
        wall_timeout_ms: 1,
        cancellation_grace_ms: 1,
    };
    assert_eq!(
        DockProtocolLimitsV1::new(over.clone()).unwrap_err(),
        DockErrorV1::BoundExceeded
    );
    over.request_bytes = 1;
    over.wall_timeout_ms = 0;
    assert_eq!(
        DockProtocolLimitsV1::new(over).unwrap_err(),
        DockErrorV1::BoundExceeded
    );
    assert_eq!(MAX_BUNDLE_FILES, 256);
    assert_eq!(MAX_BUNDLE_FILE_BYTES, 64 * 1024 * 1024);
    assert_eq!(MAX_BUNDLE_BYTES, 256 * 1024 * 1024);
    assert_eq!(MAX_INLINE_INPUTS, 128);
    assert_eq!(MAX_CANDIDATE_OBSERVATIONS, 128);
    assert_eq!(MAX_DIAGNOSTICS, 128);
    assert_eq!(MAX_WALL_TIMEOUT_MS, 300_000);
    assert_eq!(MAX_CANCELLATION_GRACE_MS, 5_000);
}

#[test]
fn request_framing_is_one_canonical_object_and_refuses_unknown_or_substituted_input() {
    let manifest = manifest();
    let request = request();
    let framed = request.framed_json_bytes().unwrap();
    assert_eq!(framed.last(), Some(&b'\n'));
    assert_eq!(framed.iter().filter(|byte| **byte == b'\n').count(), 1);
    assert_eq!(
        DockProcessRequestV1::admit_framed_json(&framed, &manifest).unwrap(),
        request
    );

    let mut value = serde_json::to_value(&request).unwrap();
    value["network"] = json!("allowed");
    let mut changed = serde_json_canonicalizer::to_vec(&value).unwrap();
    changed.push(b'\n');
    assert!(DockProcessRequestV1::admit_framed_json(&changed, &manifest).is_err());

    let mut value = serde_json::to_value(&request).unwrap();
    value["gate"] = json!("pass");
    let mut changed = serde_json_canonicalizer::to_vec(&value).unwrap();
    changed.push(b'\n');
    assert!(DockProcessRequestV1::admit_framed_json(&changed, &manifest).is_err());

    let mut extra = framed.clone();
    extra.push(b'\n');
    assert!(DockProcessRequestV1::admit_framed_json(&extra, &manifest).is_err());
}

#[test]
fn result_status_partition_candidates_and_diagnostics_are_closed_and_bounded() {
    let request = request();
    let partition =
        DockClaimPartitionV1::new(vec![stable_id("claim.schema-valid")], vec![], vec![]).unwrap();
    let candidate = DockCandidateObservationV1::new(
        stable_id("claim.schema-valid"),
        stable_id("case.positive"),
        stable_id("handbook.evidence.schema-conformance"),
        binding("handbook.fact.schema-conformance@1.0.0", 'a'),
        json!({"valid": true}),
    )
    .unwrap();
    let result = DockProcessResultV1::completed(
        &request,
        partition,
        vec![candidate],
        vec![DockDiagnosticV1::new(stable_id("diagnostic.valid"), "valid").unwrap()],
        resolution(),
    )
    .unwrap();
    assert_eq!(result.status(), DockProcessResultStatusV1::Completed);
    assert_eq!(
        DockProcessResultV1::admit_framed_json(&result.framed_json_bytes().unwrap(), &request)
            .unwrap(),
        result
    );

    let refused = DockProcessResultV1::refused(
        &request,
        DockProcessRefusalReasonV1::ObservationUnavailable,
        vec![],
    )
    .unwrap();
    assert_eq!(refused.status(), DockProcessResultStatusV1::Refused);
    assert!(refused.candidates().is_empty());
    assert!(serde_json::from_str::<DockProcessResultStatusV1>("\"timed_out\"").is_err());
    assert!(serde_json::from_str::<DockProcessRefusalReasonV1>("\"verdict_failed\"").is_err());

    let incomplete = DockClaimPartitionV1::new(vec![], vec![], vec![]).unwrap();
    assert_eq!(
        DockProcessResultV1::completed(&request, incomplete, vec![], vec![], resolution())
            .unwrap_err(),
        DockErrorV1::InvalidInvariant
    );
}

#[test]
fn host_outcomes_follow_the_fixed_closed_precedence_and_candidate_rule() {
    let completed = DockHostOutcomeFactsV1 {
        cleanup_confirmed: true,
        durability_confirmed: true,
        deadline_reached: false,
        cancellation_accepted: false,
        spawn_or_process_failed: false,
        framing_or_binding_failed: false,
        child_status: Some(DockProcessResultStatusV1::Completed),
    };
    assert_eq!(
        classify_host_outcome_v1(&completed),
        DockHostOutcomeV1::Completed
    );
    assert_eq!(
        classify_host_outcome_v1(&DockHostOutcomeFactsV1 {
            deadline_reached: true,
            child_status: Some(DockProcessResultStatusV1::Completed),
            ..completed
        }),
        DockHostOutcomeV1::TimedOut
    );
    assert_eq!(
        classify_host_outcome_v1(&DockHostOutcomeFactsV1 {
            cancellation_accepted: true,
            child_status: Some(DockProcessResultStatusV1::Completed),
            ..completed
        }),
        DockHostOutcomeV1::Cancelled
    );
    assert_eq!(
        classify_host_outcome_v1(&DockHostOutcomeFactsV1 {
            framing_or_binding_failed: true,
            child_status: Some(DockProcessResultStatusV1::Completed),
            ..completed
        }),
        DockHostOutcomeV1::ProtocolError
    );
    assert_eq!(
        classify_host_outcome_v1(&DockHostOutcomeFactsV1 {
            child_status: Some(DockProcessResultStatusV1::Refused),
            ..completed
        }),
        DockHostOutcomeV1::Refused
    );
    assert_eq!(
        classify_host_outcome_v1(&DockHostOutcomeFactsV1 {
            cleanup_confirmed: false,
            ..completed
        }),
        DockHostOutcomeV1::Failed
    );
    assert!(serde_json::from_str::<DockHostOutcomeV1>("\"succeeded\"").is_err());
}

#[test]
fn execution_record_binds_request_result_outcome_and_completed_candidates_without_a_cycle() {
    let request = request();
    let result = DockProcessResultV1::completed(
        &request,
        DockClaimPartitionV1::new(vec![stable_id("claim.schema-valid")], vec![], vec![]).unwrap(),
        vec![DockCandidateObservationV1::new(
            stable_id("claim.schema-valid"),
            stable_id("case.positive"),
            stable_id("handbook.evidence.schema-conformance"),
            binding("handbook.fact.schema-conformance@1.0.0", 'a'),
            json!({"valid": true}),
        )
        .unwrap()],
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
        wall_time_ms: 50,
        stdout_bytes: result.framed_json_bytes().unwrap().len() as u64,
        stdout_truncated: false,
        stderr_bytes: 0,
        stderr_truncated: false,
        diagnostics: vec![],
    })
    .unwrap();
    assert_eq!(record.host_outcome(), DockHostOutcomeV1::Completed);
    assert_eq!(record.candidate_observation_count(), 1);
    assert!(!record
        .canonical_json_bytes()
        .unwrap()
        .windows("candidate_ref".len())
        .any(|w| w == b"candidate_ref"));
    assert_eq!(
        serde_json::from_slice::<DockExecutionRecordV1>(&record.canonical_json_bytes().unwrap())
            .unwrap(),
        record
    );
}

#[test]
fn production_registry_is_immutable_empty_and_list_read_are_bounded_exact_use_cases() {
    let registry = DockManifestRegistryV1::production();
    let page = registry
        .list(DockManifestListRequestV1::first(25).unwrap())
        .unwrap();
    assert_eq!(page.entries().len(), 1);
    assert_eq!(
        page.entries()[0].manifest_ref().as_str(),
        "handbook.dock.json-schema@1.0.0"
    );
    assert!(page.next_cursor().is_none());
    assert_eq!(page.retention(), DockManifestRetentionV1::ImmutableExact);
    assert_eq!(registry.production_len(), 1);
    assert_eq!(
        registry
            .read(
                &ExactRefV1::parse("handbook.dock.missing@1.0.0").unwrap(),
                &fingerprint('0'),
            )
            .unwrap(),
        DockManifestReadOutcomeV1::Absent
    );
    assert_eq!(
        DockManifestListRequestV1::first(0).unwrap_err(),
        DockErrorV1::BoundExceeded
    );
}

#[test]
fn dependency_direction_and_semantic_owner_constructor_absence_are_source_gated() {
    let dock_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = dock_root.parent().unwrap().parent().unwrap();
    let dock_manifest = fs::read_to_string(dock_root.join("Cargo.toml")).unwrap();
    assert!(dock_manifest.contains("handbook-contracts ="));
    assert!(!dock_manifest.contains("handbook-sdk ="));
    assert!(!dock_manifest.contains("handbook-flow ="));
    assert!(!dock_manifest.contains("handbook-pipeline ="));

    for owner in ["contracts", "engine", "flow", "pipeline"] {
        let text =
            fs::read_to_string(workspace.join("crates").join(owner).join("Cargo.toml")).unwrap();
        assert!(
            !text.contains("handbook-dock"),
            "{owner} depends outward on dock"
        );
    }

    let mut source = String::new();
    for entry in fs::read_dir(dock_root.join("src")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|value| value.to_str()) == Some("rs") {
            source.push_str(&fs::read_to_string(path).unwrap());
        }
    }
    for forbidden in [
        "EvidenceRecordV1::new",
        "evaluate_claim_verdicts_v1",
        "evaluate_gate_and_local_closeout_v1",
        "prepare_lifecycle_transition",
        "prepare_contract_definition_append",
    ] {
        assert!(
            !source.contains(forbidden),
            "dock gained semantic-owner constructor {forbidden}"
        );
    }
}

#[test]
fn public_protocol_shapes_never_gain_semantic_truth_or_host_path_fields() {
    let request: Value = serde_json::to_value(request()).unwrap();
    let result = DockProcessResultV1::refused(
        &request_fixture(),
        DockProcessRefusalReasonV1::InputRefused,
        vec![],
    )
    .unwrap();
    let result: Value = serde_json::to_value(result).unwrap();
    let serialized = format!("{}{}", request, result);
    for forbidden in [
        "verdict",
        "gate",
        "applicability",
        "local_closeout",
        "promotion",
        "repository_path",
        "home_path",
        "credential",
        "idempotency_key",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "protocol leaked {forbidden}"
        );
    }
}

fn request_fixture() -> DockProcessRequestV1 {
    request()
}

#[test]
fn duplicate_json_members_and_invalid_deserialized_bounds_refuse() {
    let manifest = manifest();
    let request = request();
    let canonical = request.canonical_json_bytes().unwrap();
    let mut duplicated = br#"{"network":"denied","#.to_vec();
    duplicated.extend_from_slice(&canonical[1..]);
    duplicated.push(b'\n');
    assert!(DockProcessRequestV1::admit_framed_json(&duplicated, &manifest).is_err());

    let mut invalid_limits = serde_json::to_value(limits()).unwrap();
    invalid_limits["wall_timeout_ms"] = json!(0);
    assert!(serde_json::from_value::<DockProtocolLimitsV1>(invalid_limits).is_err());

    let duplicate_partition = json!({
        "observed": ["claim.schema-valid", "claim.schema-valid"],
        "unobserved": [],
        "typed_excluded": []
    });
    assert!(serde_json::from_value::<DockClaimPartitionV1>(duplicate_partition).is_err());
}

#[test]
fn execution_record_substitution_refuses_with_the_old_fingerprint() {
    let request = request();
    let refused =
        DockProcessResultV1::refused(&request, DockProcessRefusalReasonV1::InputRefused, vec![])
            .unwrap();
    let record = DockExecutionRecordV1::new(DockExecutionRecordInputV1 {
        request: &request,
        result: Some(&refused),
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
        stdout_bytes: refused.framed_json_bytes().unwrap().len() as u64,
        stdout_truncated: false,
        stderr_bytes: 0,
        stderr_truncated: false,
        diagnostics: vec![],
    })
    .unwrap();
    let mut substituted = serde_json::to_value(record).unwrap();
    substituted["host_outcome"] = json!("completed");
    assert!(serde_json::from_value::<DockExecutionRecordV1>(substituted).is_err());
}

#[test]
fn execution_record_refuses_late_results_and_out_of_bound_resource_facts() {
    let request = request();
    let completed = DockProcessResultV1::completed(
        &request,
        DockClaimPartitionV1::new(vec![stable_id("claim.schema-valid")], vec![], vec![]).unwrap(),
        vec![],
        vec![],
        resolution(),
    )
    .unwrap();
    let late = DockExecutionRecordV1::new(DockExecutionRecordInputV1 {
        request: &request,
        result: Some(&completed),
        facts: DockHostOutcomeFactsV1 {
            cleanup_confirmed: true,
            durability_confirmed: true,
            deadline_reached: true,
            cancellation_accepted: false,
            spawn_or_process_failed: false,
            framing_or_binding_failed: false,
            child_status: Some(DockProcessResultStatusV1::Completed),
        },
        exit_code: Some(0),
        wall_time_ms: 10,
        stdout_bytes: completed.framed_json_bytes().unwrap().len() as u64,
        stdout_truncated: false,
        stderr_bytes: 0,
        stderr_truncated: false,
        diagnostics: vec![],
    });
    assert_eq!(late.unwrap_err(), DockErrorV1::InvalidInvariant);

    let oversized = DockExecutionRecordV1::new(DockExecutionRecordInputV1 {
        request: &request,
        result: None,
        facts: DockHostOutcomeFactsV1 {
            cleanup_confirmed: true,
            durability_confirmed: true,
            deadline_reached: false,
            cancellation_accepted: false,
            spawn_or_process_failed: true,
            framing_or_binding_failed: false,
            child_status: None,
        },
        exit_code: None,
        wall_time_ms: MAX_WALL_TIMEOUT_MS + MAX_CANCELLATION_GRACE_MS + 1,
        stdout_bytes: 0,
        stdout_truncated: false,
        stderr_bytes: 0,
        stderr_truncated: false,
        diagnostics: vec![],
    });
    assert_eq!(oversized.unwrap_err(), DockErrorV1::BoundExceeded);
}

#[test]
fn completed_result_rebinds_requested_evidence_kind_and_exact_resolution() {
    let request = request();
    let partition = || {
        DockClaimPartitionV1::new(vec![stable_id("claim.schema-valid")], vec![], vec![]).unwrap()
    };
    let unrequested = DockCandidateObservationV1::new(
        stable_id("claim.schema-valid"),
        stable_id("case.positive"),
        stable_id("handbook.evidence.unrequested"),
        binding("handbook.fact.schema-conformance@1.0.0", 'a'),
        json!({"valid": true}),
    )
    .unwrap();
    assert_eq!(
        DockProcessResultV1::completed(
            &request,
            partition(),
            vec![unrequested],
            vec![],
            resolution(),
        )
        .unwrap_err(),
        DockErrorV1::InvalidInvariant
    );

    let changed_resolution = ResolutionEnvelopeV1::new(
        binding("handbook.resolution.stack@1.0.0", '9'),
        stable_id("resolution.scope.changed"),
        stable_id("resolution.detail.complete"),
        stable_id("resolution.time.current-operation"),
        stable_id("resolution.authority.read-only"),
        stable_id("resolution.memory.operation"),
        stable_id("resolution.validation.observation-only"),
    );
    assert_eq!(
        DockProcessResultV1::completed(&request, partition(), vec![], vec![], changed_resolution,)
            .unwrap_err(),
        DockErrorV1::InvalidInvariant
    );
}

#[test]
fn candidate_effective_byte_limit_applies_to_payload_not_binding_wrapper() {
    let request = request();
    let payload = json!("x".repeat(limits().candidate_bytes() - 6));
    let payload_bytes = serde_json_canonicalizer::to_vec(&payload).unwrap();
    assert!(payload_bytes.len() <= limits().candidate_bytes());
    let candidate = DockCandidateObservationV1::new(
        stable_id("claim.schema-valid"),
        stable_id("case.positive"),
        stable_id("handbook.evidence.schema-conformance"),
        binding("handbook.fact.schema-conformance@1.0.0", 'a'),
        payload,
    )
    .unwrap();
    assert!(
        serde_json_canonicalizer::to_vec(&candidate).unwrap().len() > limits().candidate_bytes()
    );
    assert!(DockProcessResultV1::completed(
        &request,
        DockClaimPartitionV1::new(vec![stable_id("claim.schema-valid")], vec![], vec![],).unwrap(),
        vec![candidate],
        vec![],
        resolution(),
    )
    .is_ok());
}

#[test]
fn execution_record_enforces_effective_limits_and_truncation_cannot_complete() {
    let request = request();
    let completed = DockProcessResultV1::completed(
        &request,
        DockClaimPartitionV1::new(vec![stable_id("claim.schema-valid")], vec![], vec![]).unwrap(),
        vec![],
        vec![],
        resolution(),
    )
    .unwrap();
    let completed_facts = DockHostOutcomeFactsV1 {
        cleanup_confirmed: true,
        durability_confirmed: true,
        deadline_reached: false,
        cancellation_accepted: false,
        spawn_or_process_failed: false,
        framing_or_binding_failed: false,
        child_status: Some(DockProcessResultStatusV1::Completed),
    };
    let truncated = DockExecutionRecordV1::new(DockExecutionRecordInputV1 {
        request: &request,
        result: Some(&completed),
        facts: completed_facts,
        exit_code: Some(0),
        wall_time_ms: 10,
        stdout_bytes: completed.framed_json_bytes().unwrap().len() as u64,
        stdout_truncated: false,
        stderr_bytes: limits().stderr_bytes() as u64,
        stderr_truncated: true,
        diagnostics: vec![],
    });
    assert_eq!(truncated.unwrap_err(), DockErrorV1::InvalidInvariant);

    let over_stderr = DockExecutionRecordV1::new(DockExecutionRecordInputV1 {
        request: &request,
        result: Some(&completed),
        facts: completed_facts,
        exit_code: Some(0),
        wall_time_ms: 10,
        stdout_bytes: completed.framed_json_bytes().unwrap().len() as u64,
        stdout_truncated: false,
        stderr_bytes: limits().stderr_bytes() as u64 + 1,
        stderr_truncated: true,
        diagnostics: vec![],
    });
    assert_eq!(over_stderr.unwrap_err(), DockErrorV1::BoundExceeded);

    let over_diagnostics = DockExecutionRecordV1::new(DockExecutionRecordInputV1 {
        request: &request,
        result: Some(&completed),
        facts: completed_facts,
        exit_code: Some(0),
        wall_time_ms: 10,
        stdout_bytes: completed.framed_json_bytes().unwrap().len() as u64,
        stdout_truncated: false,
        stderr_bytes: 0,
        stderr_truncated: false,
        diagnostics: (0..=limits().diagnostic_count())
            .map(|index| {
                DockDiagnosticV1::new(stable_id(&format!("diagnostic.item-{index}")), "bounded")
                    .unwrap()
            })
            .collect(),
    });
    assert_eq!(over_diagnostics.unwrap_err(), DockErrorV1::BoundExceeded);
}

#[test]
fn cleanup_uncertain_record_retains_only_exact_untruncated_result_binding() {
    let request = request();
    let completed = DockProcessResultV1::completed(
        &request,
        DockClaimPartitionV1::new(vec![stable_id("claim.schema-valid")], vec![], vec![]).unwrap(),
        vec![],
        vec![],
        resolution(),
    )
    .unwrap();
    let facts = DockHostOutcomeFactsV1 {
        cleanup_confirmed: false,
        durability_confirmed: true,
        deadline_reached: false,
        cancellation_accepted: false,
        spawn_or_process_failed: false,
        framing_or_binding_failed: false,
        child_status: Some(DockProcessResultStatusV1::Completed),
    };
    let wrong_length = DockExecutionRecordV1::new(DockExecutionRecordInputV1 {
        request: &request,
        result: Some(&completed),
        facts,
        exit_code: Some(0),
        wall_time_ms: 10,
        stdout_bytes: 0,
        stdout_truncated: false,
        stderr_bytes: 0,
        stderr_truncated: false,
        diagnostics: vec![],
    });
    assert_eq!(wrong_length.unwrap_err(), DockErrorV1::InvalidInvariant);

    let truncated = DockExecutionRecordV1::new(DockExecutionRecordInputV1 {
        request: &request,
        result: Some(&completed),
        facts,
        exit_code: Some(0),
        wall_time_ms: 10,
        stdout_bytes: completed.framed_json_bytes().unwrap().len() as u64,
        stdout_truncated: true,
        stderr_bytes: 0,
        stderr_truncated: false,
        diagnostics: vec![],
    });
    assert_eq!(truncated.unwrap_err(), DockErrorV1::InvalidInvariant);

    let exact = DockExecutionRecordV1::new(DockExecutionRecordInputV1 {
        request: &request,
        result: Some(&completed),
        facts,
        exit_code: Some(0),
        wall_time_ms: 10,
        stdout_bytes: completed.framed_json_bytes().unwrap().len() as u64,
        stdout_truncated: false,
        stderr_bytes: 0,
        stderr_truncated: false,
        diagnostics: vec![],
    })
    .unwrap();
    assert_eq!(exact.host_outcome(), DockHostOutcomeV1::Failed);
    assert_eq!(exact.candidate_observation_count(), 0);
}
