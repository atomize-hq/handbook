use handbook_sdk::artifact::{ArtifactAcquisitionMode, ArtifactInputDocument, ArtifactSdk};
use handbook_sdk::contract_membrane::{
    artifact_operation_request, flow_resolve_operation_request, operation_inventory,
    pipeline_capture_direct_operation_request, pipeline_capture_plan_operation_request,
    pipeline_catalog_list_operation_request, pipeline_catalog_read_operation_request,
    pipeline_compile_operation_request, ArtifactOperationInputV1, ContractLimits, DiscoveryState,
    InvocationRequestV1, InvocationResponseKind, InvocationResponseV1, OperationRequestV1,
    OperationTransport, ProductionWitness, UtcTimestamp,
};
use handbook_sdk::HandbookSdkV1;
use std::collections::BTreeSet;

const KIND_REF: &str = "example.artifact-kind.registry-brief@1.0.0";
const INSTANCE_ID: &str = "registry_brief";
const INTAKE_REF: &str = ".handbook/state/artifacts/registry_brief/intake-records/intake_0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef.json";
const FINGERPRINT: &str = "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn coverage_input() -> ArtifactInputDocument {
    ArtifactInputDocument::new(
        serde_json::to_vec(&serde_json::json!({
            "coverage_submissions": [
                {
                    "coverage_id": "registry_brief.title",
                    "state": "supplied",
                    "source_kind": "user_declaration",
                    "value": "Packet 3 parity",
                    "specificity": "exact",
                    "confidence": "high",
                    "contradiction_refs": []
                },
                {
                    "coverage_id": "registry_brief.summary",
                    "state": "supplied",
                    "source_kind": "user_declaration",
                    "value": "Read-only invocation dispatcher",
                    "specificity": "concrete",
                    "confidence": "high",
                    "contradiction_refs": []
                }
            ]
        }))
        .unwrap(),
    )
    .unwrap()
}

fn read_only_requests() -> Vec<OperationRequestV1> {
    vec![
        artifact_operation_request(None, ArtifactOperationInputV1::KindList).unwrap(),
        artifact_operation_request(None, ArtifactOperationInputV1::InstanceList).unwrap(),
        artifact_operation_request(
            None,
            ArtifactOperationInputV1::Read {
                kind_ref: KIND_REF.to_owned(),
                instance_id: INSTANCE_ID.to_owned(),
            },
        )
        .unwrap(),
        artifact_operation_request(
            None,
            ArtifactOperationInputV1::Validate {
                kind_ref: KIND_REF.to_owned(),
                instance_id: INSTANCE_ID.to_owned(),
            },
        )
        .unwrap(),
        artifact_operation_request(
            None,
            ArtifactOperationInputV1::IntakeDefinitionRead {
                kind_ref: KIND_REF.to_owned(),
                instance_id: INSTANCE_ID.to_owned(),
            },
        )
        .unwrap(),
        artifact_operation_request(
            None,
            ArtifactOperationInputV1::IntakeCoverageEvaluate {
                kind_ref: KIND_REF.to_owned(),
                instance_id: INSTANCE_ID.to_owned(),
                mode: ArtifactAcquisitionMode::Express,
                expected_current_artifact_fingerprint: None,
                input: coverage_input(),
            },
        )
        .unwrap(),
        artifact_operation_request(
            None,
            ArtifactOperationInputV1::CandidateValidate {
                kind_ref: KIND_REF.to_owned(),
                instance_id: INSTANCE_ID.to_owned(),
                intake_record_ref: INTAKE_REF.to_owned(),
                intake_record_fingerprint: FINGERPRINT.to_owned(),
                expected_current_artifact_fingerprint: None,
            },
        )
        .unwrap(),
        flow_resolve_operation_request(None, "planning.packet").unwrap(),
        pipeline_catalog_list_operation_request(None).unwrap(),
        pipeline_catalog_read_operation_request(None, "pipeline.foundation_inputs").unwrap(),
        pipeline_compile_operation_request(
            None,
            "pipeline.foundation_inputs",
            "stage.10_feature_spec",
        )
        .unwrap(),
        pipeline_capture_plan_operation_request(
            None,
            "pipeline.foundation_inputs",
            "stage.10_feature_spec",
            "packet-3 preview\n",
        )
        .unwrap(),
    ]
}

#[test]
fn sdk_dispatcher_matches_each_direct_owner_route_for_exactly_twelve_read_only_operations() {
    let repo = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(repo.path());
    let limits = ContractLimits::packet0();
    let requests = read_only_requests();
    assert_eq!(requests.len(), 12);

    for request in requests {
        let operation_ref = request.operation().reference().as_str().to_owned();
        let direct =
            if operation_ref.starts_with("artifact.") || operation_ref.starts_with("intake.") {
                ArtifactSdk::open(repo.path())
                    .execute_operation(&request)
                    .unwrap()
            } else if operation_ref == "flow.resolve@1.0.0" {
                sdk.execute_flow_operation(&request).unwrap()
            } else {
                sdk.execute_pipeline_operation(&request).unwrap()
            };
        let expected = InvocationResponseV1::operation_response(direct, &limits).unwrap();
        let invocation = InvocationRequestV1::operation(request, &limits).unwrap();

        let actual = sdk.invoke(&invocation, &limits).unwrap();
        assert_eq!(actual, expected, "{operation_ref}");
        assert_eq!(actual.response_kind(), InvocationResponseKind::Operation);
        assert_eq!(
            sdk.readmit_invocation_response(&invocation, &actual, &limits)
                .unwrap(),
            actual,
            "{operation_ref}"
        );
    }
}

#[test]
fn sdk_dispatcher_default_denies_mutation_and_non_target_requests_without_writes() {
    let repo = tempfile::tempdir().unwrap();
    let sentinel = repo.path().join("sentinel.txt");
    std::fs::write(&sentinel, b"unchanged").unwrap();
    let sdk = HandbookSdkV1::open(repo.path());
    let limits = ContractLimits::packet0();

    let mutation = InvocationRequestV1::operation(
        pipeline_capture_direct_operation_request(
            None,
            "pipeline.foundation_inputs",
            "stage.10_feature_spec",
            "must not execute",
            "packet-3-denied-mutation",
        )
        .unwrap(),
        &limits,
    )
    .unwrap();
    let response = sdk.invoke(&mutation, &limits).unwrap();
    assert_eq!(response.response_kind(), InvocationResponseKind::Operation);

    let admitted = InvocationRequestV1::operation(
        pipeline_catalog_list_operation_request(None).unwrap(),
        &limits,
    )
    .unwrap();
    let mut non_target = serde_json::to_value(admitted).unwrap();
    non_target["request"]["operation"]["ref"] = serde_json::json!("pipeline.route.resolve@1.0.0");
    assert!(serde_json::from_value::<InvocationRequestV1>(non_target).is_err());

    assert_eq!(std::fs::read(&sentinel).unwrap(), b"unchanged");
}

#[test]
fn production_discovery_adds_tauri_only_to_the_twelve_proven_read_only_rows() {
    let rows = operation_inventory();
    assert_eq!(rows.len(), 62);

    let expected_tauri = rows
        .iter()
        .filter(|row| row.discovery() == DiscoveryState::Discoverable)
        .map(|row| row.operation_ref().as_str().to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(expected_tauri.len(), 31);
    let actual_tauri = rows
        .iter()
        .filter(|row| {
            row.current_transports()
                .contains(&OperationTransport::Tauri)
        })
        .map(|row| row.operation_ref().as_str().to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_tauri, expected_tauri);

    let cli_rows = rows
        .iter()
        .filter(|row| {
            row.current_transports() == [OperationTransport::CliJson, OperationTransport::Tauri]
        })
        .collect::<Vec<_>>();
    assert_eq!(cli_rows.len(), 18);
    for row in cli_rows {
        assert_eq!(
            row.current_transports(),
            &[OperationTransport::CliJson, OperationTransport::Tauri]
        );
    }

    let posture = rows
        .iter()
        .find(|row| row.operation_ref().as_str() == "posture.transition.apply@1.0.0")
        .unwrap();
    assert_eq!(
        posture.current_transports(),
        &[OperationTransport::RustSdk, OperationTransport::Tauri]
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.discovery() == DiscoveryState::Discoverable)
            .count(),
        31
    );

    let inventory_refs = rows
        .iter()
        .map(|row| row.operation_ref().as_str())
        .collect::<BTreeSet<_>>();
    for use_case in [
        "handbook.use-case.repository-setup@1.0.0",
        "handbook.use-case.repository-doctor@1.0.0",
        "handbook.use-case.author-charter@1.0.0",
        "handbook.use-case.author-project-context@1.0.0",
        "handbook.use-case.approver-administration@1.0.0",
        "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
    ] {
        assert!(!inventory_refs.contains(use_case));
    }

    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap())
            .unwrap();
    let witness_tauri = witness
        .capabilities()
        .iter()
        .filter(|entry| {
            entry
                .current_transports()
                .contains(&OperationTransport::Tauri)
        })
        .map(|entry| entry.operation().reference().as_str().to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(witness_tauri, expected_tauri);
}
