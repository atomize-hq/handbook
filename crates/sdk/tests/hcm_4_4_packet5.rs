use handbook_sdk::artifact::{ArtifactInputDocument, ArtifactSdk};
use handbook_sdk::contract_membrane::{
    artifact_operation_request, canonical_json_bytes, operation_inventory,
    pipeline_capture_direct_operation_request, pipeline_handoff_emit_operation_request,
    pipeline_state_apply_operation_request, posture_transition_apply_request,
    ArtifactOperationInputV1, ContractLimits, DiscoveryState, InvocationRequestV1,
    InvocationResponseV1, OperationRequestV1, OperationTransport, PostureCanonicalBindingDtoV1,
    PostureDimensionChangeDtoV1, PostureReferenceDtoV1, PostureTransitionApplyRequestDataV1,
    ProductionWitness, TransportFailureKind, TransportFailureV1, UtcTimestamp,
};
use handbook_sdk::pipeline_route_api::PipelineStateMutation;
use handbook_sdk::HandbookSdkV1;
use std::collections::BTreeSet;

const HASH: &str = "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn document(value: serde_json::Value) -> ArtifactInputDocument {
    ArtifactInputDocument::new(serde_json::to_vec(&value).unwrap()).unwrap()
}

fn mutation_requests() -> Vec<OperationRequestV1> {
    let artifact_target = || {
        (
            "example.artifact-kind.registry-brief@1.0.0",
            "registry_brief",
        )
    };
    let (kind_ref, instance_id) = artifact_target();
    vec![
        artifact_operation_request(
            Some("packet5-intake"),
            ArtifactOperationInputV1::IntakeRecordAppend {
                kind_ref: kind_ref.to_owned(),
                instance_id: instance_id.to_owned(),
                input: document(serde_json::json!({
                    "idempotency_key": "packet5_intake_key_0001",
                    "acquisition_mode": "express",
                    "expected_current_artifact_fingerprint": HASH,
                    "coverage_submissions": []
                })),
            },
        )
        .unwrap(),
        artifact_operation_request(
            Some("packet5-candidate"),
            ArtifactOperationInputV1::CandidateAppend {
                kind_ref: kind_ref.to_owned(),
                instance_id: instance_id.to_owned(),
                input: document(serde_json::json!({
                    "idempotency_key": "packet5_candidate_key_0001",
                    "intake_record_ref": ".handbook/state/artifacts/registry_brief/intake-records/intake_0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef.json",
                    "intake_record_fingerprint": HASH,
                    "expected_candidate_fingerprint": HASH
                })),
            },
        )
        .unwrap(),
        artifact_operation_request(
            Some("packet5-promote"),
            ArtifactOperationInputV1::CandidatePromote {
                kind_ref: kind_ref.to_owned(),
                instance_id: instance_id.to_owned(),
                input: document(serde_json::json!({
                    "idempotency_key": "packet5_promotion_key_0001",
                    "candidate_ref": ".handbook/state/artifacts/registry_brief/candidates/candidate_0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef.json",
                    "candidate_fingerprint": HASH,
                    "expected_current_artifact_fingerprint": HASH
                })),
            },
        )
        .unwrap(),
        pipeline_capture_direct_operation_request(
            Some("packet5-capture"),
            "pipeline.foundation_inputs",
            "stage.10_feature_spec",
            "packet 5 capture",
            "packet5_capture_key_0001",
        )
        .unwrap(),
        pipeline_handoff_emit_operation_request(
            Some("packet5-handoff"),
            "pipeline.foundation_inputs",
            "consumer.example",
            "handbook",
            "1.0.0",
            "packet5_handoff_key_0001",
        )
        .unwrap(),
        pipeline_state_apply_operation_request(
            Some("packet5-state"),
            "pipeline.foundation_inputs",
            &PipelineStateMutation::RoutingVariable {
                variable: "has_charter".to_owned(),
                value: true,
            },
            0,
            "packet5_state_key_0001",
        )
        .unwrap(),
        posture_transition_apply_request(
            Some("packet5-posture"),
            "packet5_posture_key_0001",
            PostureTransitionApplyRequestDataV1::new(
                HASH.to_owned(),
                PostureCanonicalBindingDtoV1::new(
                    ".handbook/project/charter.yaml".to_owned(),
                    HASH.to_owned(),
                    HASH.to_owned(),
                    1,
                )
                .unwrap(),
                PostureReferenceDtoV1::new("recommendation.packet5".to_owned(), HASH.to_owned())
                    .unwrap(),
                vec!["coverage.packet5".to_owned()],
                PostureDimensionChangeDtoV1::new(
                    "dimension.packet5".to_owned(),
                    0,
                    "/posture/dimensions/0".to_owned(),
                    None,
                    0,
                    1,
                )
                .unwrap(),
                "2030-01-01T00:00:00Z".to_owned(),
            )
            .unwrap(),
        )
        .unwrap(),
    ]
}

#[test]
fn dispatcher_routes_exactly_seven_mutations_to_the_existing_owner_once() {
    let limits = ContractLimits::packet0();
    let requests = mutation_requests();
    assert_eq!(requests.len(), 7);

    for request in requests {
        let direct_repo = tempfile::tempdir().unwrap();
        let dispatched_repo = tempfile::tempdir().unwrap();
        let operation_ref = request.operation().reference().as_str().to_owned();
        let direct =
            if operation_ref.starts_with("artifact.") || operation_ref.starts_with("intake.") {
                ArtifactSdk::open(direct_repo.path())
                    .execute_operation(&request)
                    .unwrap()
            } else if operation_ref.starts_with("pipeline.") {
                HandbookSdkV1::open(direct_repo.path())
                    .execute_pipeline_operation(&request)
                    .unwrap()
            } else {
                HandbookSdkV1::open(direct_repo.path())
                    .apply_posture_transition_operation(&request)
                    .unwrap()
            };
        let expected = InvocationResponseV1::operation_response(direct, &limits).unwrap();
        let invocation = InvocationRequestV1::operation(request, &limits).unwrap();
        let sdk = HandbookSdkV1::open(dispatched_repo.path());
        let actual = sdk.invoke(&invocation, &limits).unwrap();

        assert_eq!(actual, expected, "{operation_ref}");
        let raw_key = match operation_ref.as_str() {
            "intake.record.append@1.0.0" => "packet5_intake_key_0001",
            "artifact.candidate.append@1.0.0" => "packet5_candidate_key_0001",
            "artifact.candidate.promote@1.0.0" => "packet5_promotion_key_0001",
            "pipeline.capture.apply@1.0.0" => "packet5_capture_key_0001",
            "pipeline.handoff.emit@1.0.0" => "packet5_handoff_key_0001",
            "pipeline.state.apply@1.0.0" => "packet5_state_key_0001",
            "posture.transition.apply@1.0.0" => "packet5_posture_key_0001",
            _ => unreachable!(),
        };
        let wire = canonical_json_bytes(&actual, &limits).unwrap();
        assert!(!wire
            .windows(raw_key.len())
            .any(|window| window == raw_key.as_bytes()));
        assert_eq!(
            sdk.readmit_invocation_response(&invocation, &actual, &limits)
                .unwrap(),
            actual,
            "{operation_ref}"
        );
    }
}

#[test]
fn mutation_response_readmission_binds_operation_semantics_and_request_identity() {
    let limits = ContractLimits::packet0();
    let request_a = pipeline_capture_direct_operation_request(
        Some("packet5-binding-a"),
        "pipeline.foundation_inputs",
        "stage.10_feature_spec",
        "packet 5 response binding",
        "packet5_binding_key_0001",
    )
    .unwrap();
    let response_a = HandbookSdkV1::open(tempfile::tempdir().unwrap().path())
        .execute_pipeline_operation(&request_a)
        .unwrap();
    let response_a = InvocationResponseV1::operation_response(response_a, &limits).unwrap();
    let invocation_a = InvocationRequestV1::operation(request_a, &limits).unwrap();
    let sdk = HandbookSdkV1::open(tempfile::tempdir().unwrap().path());
    assert_eq!(
        sdk.readmit_invocation_response(&invocation_a, &response_a, &limits)
            .unwrap(),
        response_a
    );

    let different_request_id = InvocationRequestV1::operation(
        pipeline_capture_direct_operation_request(
            Some("packet5-binding-b"),
            "pipeline.foundation_inputs",
            "stage.10_feature_spec",
            "packet 5 response binding",
            "packet5_binding_key_0001",
        )
        .unwrap(),
        &limits,
    )
    .unwrap();
    assert!(sdk
        .readmit_invocation_response(&different_request_id, &response_a, &limits)
        .is_err());

    let different_semantics = InvocationRequestV1::operation(
        pipeline_capture_direct_operation_request(
            Some("packet5-binding-a"),
            "pipeline.foundation_inputs",
            "stage.10_feature_spec",
            "different capture input",
            "packet5_binding_key_0001",
        )
        .unwrap(),
        &limits,
    )
    .unwrap();
    assert!(sdk
        .readmit_invocation_response(&different_semantics, &response_a, &limits)
        .is_err());

    let substitutable_failure = InvocationResponseV1::transport_failure(
        TransportFailureV1::adapter_error(
            TransportFailureKind::Adapter,
            "transport.invocation_construction_failed",
            &limits,
        )
        .unwrap(),
        &limits,
    )
    .unwrap();
    assert!(sdk
        .readmit_invocation_response(&invocation_a, &substitutable_failure, &limits)
        .is_err());
}

#[test]
fn dispatcher_preserves_pipeline_compare_and_write_receipts_replay_and_key_conflict() {
    let (_repo, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (definition, supported_variables) =
        pipeline_proof_corpus_support::load_foundation_inputs_definition(&root);
    let variable = supported_variables.iter().next().unwrap().clone();
    let before = handbook_pipeline::route_state::load_route_state_with_supported_variables(
        &root,
        &definition.header.id,
        &supported_variables,
    )
    .unwrap();
    let limits = ContractLimits::packet0();
    let request = pipeline_state_apply_operation_request(
        Some("packet5-state-dispatch"),
        &definition.header.id,
        &PipelineStateMutation::RoutingVariable {
            variable: variable.clone(),
            value: true,
        },
        before.revision,
        "packet5_state_dispatch_key_0001",
    )
    .unwrap();
    let invocation = InvocationRequestV1::operation(request, &limits).unwrap();
    let sdk = HandbookSdkV1::open(&root);
    let applied = sdk.invoke(&invocation, &limits).unwrap();
    let applied_value = serde_json::to_value(&applied).unwrap();
    assert_eq!(applied_value["response"]["status"], "ok");
    assert_eq!(
        applied_value["response"]["idempotency"]["state"],
        "established"
    );
    assert_eq!(applied_value["response"]["idempotency"]["replayed"], false);
    assert_eq!(
        applied_value["response"]["receipts"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let after_apply = handbook_pipeline::route_state::load_route_state_with_supported_variables(
        &root,
        &definition.header.id,
        &supported_variables,
    )
    .unwrap();
    assert_eq!(after_apply.revision, before.revision + 1);

    let replayed = sdk.invoke(&invocation, &limits).unwrap();
    let replayed_value = serde_json::to_value(&replayed).unwrap();
    assert_eq!(replayed_value["response"]["idempotency"]["replayed"], true);
    assert_eq!(
        replayed_value["response"]["idempotency"]["original_result_fingerprint"],
        applied_value["response"]["result_fingerprint"]
    );
    assert_eq!(
        replayed_value["response"]["result_fingerprint"],
        applied_value["response"]["result_fingerprint"]
    );
    assert_eq!(
        replayed_value["response"]["data"],
        applied_value["response"]["data"]
    );
    assert_eq!(
        replayed_value["response"]["receipts"],
        applied_value["response"]["receipts"]
    );
    assert_eq!(
        handbook_pipeline::route_state::load_route_state_with_supported_variables(
            &root,
            &definition.header.id,
            &supported_variables,
        )
        .unwrap()
        .revision,
        after_apply.revision
    );

    let conflict = InvocationRequestV1::operation(
        pipeline_state_apply_operation_request(
            Some("packet5-state-conflict"),
            &definition.header.id,
            &PipelineStateMutation::RoutingVariable {
                variable,
                value: false,
            },
            before.revision,
            "packet5_state_dispatch_key_0001",
        )
        .unwrap(),
        &limits,
    )
    .unwrap();
    let conflict = serde_json::to_value(sdk.invoke(&conflict, &limits).unwrap()).unwrap();
    assert_eq!(conflict["response"]["status"], "refused");
    assert!(conflict["response"]["receipts"]
        .as_array()
        .unwrap()
        .is_empty());
    assert_eq!(
        handbook_pipeline::route_state::load_route_state_with_supported_variables(
            &root,
            &definition.header.id,
            &supported_variables,
        )
        .unwrap()
        .revision,
        after_apply.revision
    );
}

#[test]
fn final_inventory_and_witness_have_exactly_nineteen_transport_vectors() {
    let rows = operation_inventory();
    assert_eq!(rows.len(), 62);
    assert_eq!(
        rows.iter()
            .filter(|row| row.discovery() == DiscoveryState::Discoverable)
            .count(),
        23
    );

    let expected_cli_tauri = rows
        .iter()
        .filter(|row| {
            row.current_transports() == [OperationTransport::CliJson, OperationTransport::Tauri]
        })
        .map(|row| row.operation_ref().as_str().to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(expected_cli_tauri.len(), 18);
    for operation_ref in &expected_cli_tauri {
        let row = rows
            .iter()
            .find(|row| row.operation_ref().as_str() == operation_ref)
            .unwrap();
        assert_eq!(
            row.current_transports(),
            &[OperationTransport::CliJson, OperationTransport::Tauri],
            "{operation_ref}"
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
            .filter(|row| !row.current_transports().is_empty())
            .count(),
        23
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.current_transports().is_empty())
            .count(),
        39
    );

    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap())
            .unwrap();
    let witness_vectors = witness
        .capabilities()
        .iter()
        .filter(|entry| !entry.current_transports().is_empty())
        .map(|entry| {
            (
                entry.operation().reference().as_str().to_owned(),
                entry.current_transports().to_vec(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(witness_vectors.len(), 23);
    assert!(witness_vectors.iter().all(|(operation_ref, transports)| {
        if operation_ref.starts_with("contract.") {
            transports
                == &[
                    OperationTransport::CliJson,
                    OperationTransport::RustSdk,
                    OperationTransport::Tauri,
                ]
        } else if operation_ref == "posture.transition.apply@1.0.0" {
            transports == &[OperationTransport::RustSdk, OperationTransport::Tauri]
        } else {
            transports == &[OperationTransport::CliJson, OperationTransport::Tauri]
        }
    }));
}
#[path = "support/pipeline_proof_corpus_support.rs"]
mod pipeline_proof_corpus_support;
