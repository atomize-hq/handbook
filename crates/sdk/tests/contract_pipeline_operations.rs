use handbook_sdk::contract_membrane::{
    admit_operation_definition, canonical_json_bytes, operation_inventory, parse_canonical_json,
    AdmissionReason, ContractLimits, ContractValue, DiscoveryState, InvocationResponseV1,
    OperationAdmissionResult, OperationSchemaValidator, OperationTransport,
    PipelineOperationSchemaValidator, TerminalStatus, UseCaseResponseV1,
};
use handbook_sdk::pipeline_route_api::{
    PipelineResolveExecution, PipelineResolveExecutionFailure, PipelineRouteBasisPersistenceRefusal,
};

#[test]
fn exactly_three_pipeline_operations_are_independently_admitted_and_discoverable() {
    let definitions = handbook_sdk::contract_membrane::pipeline_operation_definitions().unwrap();
    assert_eq!(definitions.len(), 3);
    assert_eq!(
        definitions
            .iter()
            .map(|definition| definition.operation().as_str())
            .collect::<Vec<_>>(),
        vec![
            "pipeline.catalog.list@1.0.0",
            "pipeline.catalog.read@1.0.0",
            "pipeline.compile@1.0.0",
        ]
    );
    for definition in &definitions {
        let operation = definition.operation().as_str();
        let fingerprint = definition.definition_fingerprint().as_str();
        let admission = admit_operation_definition(definition);
        assert!(
            matches!(admission, Ok(OperationAdmissionResult::Admitted(_))),
            "{operation} {fingerprint}: {admission:?}"
        );
    }

    for operation in [
        "pipeline.catalog.list",
        "pipeline.catalog.read",
        "pipeline.compile",
    ] {
        let row = operation_inventory()
            .iter()
            .find(|row| row.operation_id() == operation)
            .unwrap();
        assert_eq!(row.admission_reason(), AdmissionReason::Admitted);
        assert_eq!(row.discovery(), DiscoveryState::Discoverable);
        assert_eq!(row.current_transports(), &[OperationTransport::CliJson]);
    }

    let resolve = operation_inventory()
        .iter()
        .find(|row| row.operation_id() == "pipeline.route.resolve")
        .unwrap();
    assert_eq!(
        resolve.admission_reason(),
        AdmissionReason::SemanticMismatch
    );
    assert_eq!(resolve.discovery(), DiscoveryState::Omit);
    assert!(resolve.current_transports().is_empty());

    let _validator = PipelineOperationSchemaValidator;
}

#[test]
fn exact_pipeline_requests_are_closed_and_compile_modes_share_one_operation() {
    let list =
        handbook_sdk::contract_membrane::pipeline_catalog_list_operation_request(None).unwrap();
    assert_eq!(
        list.operation().reference().as_str(),
        "pipeline.catalog.list@1.0.0"
    );

    let read = handbook_sdk::contract_membrane::pipeline_catalog_read_operation_request(
        None,
        "pipeline.foundation_inputs",
    )
    .unwrap();
    assert_eq!(
        read.operation().reference().as_str(),
        "pipeline.catalog.read@1.0.0"
    );

    let compile = handbook_sdk::contract_membrane::pipeline_compile_operation_request(
        None,
        "pipeline.foundation_inputs",
        "stage.10_feature_spec",
    )
    .unwrap();
    assert_eq!(
        compile.operation().reference().as_str(),
        "pipeline.compile@1.0.0"
    );
    assert_eq!(
        compile.body_schema().reference().as_str(),
        "handbook.operation.pipeline-compile-request@1.0.0"
    );

    let limits = ContractLimits::packet0();
    let extra = parse_canonical_json(br#"{"pipeline_selector":"pipeline.foundation_inputs","raw":true,"stage_selector":"stage.10_feature_spec"}"#, &limits).unwrap();
    assert!(PipelineOperationSchemaValidator
        .validate_instance(compile.body_schema(), &extra)
        .is_err());
    assert!(parse_canonical_json::<ContractValue>(br#"{"selector":""}"#, &limits).is_err());
}

#[test]
fn resolve_revision_conflict_preserves_exact_compare_and_write_truth() {
    let limits = ContractLimits::packet0();
    let failure = PipelineResolveExecutionFailure::RouteBasisPersistenceRefused(
        PipelineRouteBasisPersistenceRefusal::RevisionConflict {
            expected_revision: 7,
            actual_revision: 8,
        },
    );
    let response = UseCaseResponseV1::pipeline_route_resolve_and_persist_execution(
        &PipelineResolveExecution {
            result: Err(failure.clone()),
            effects: Vec::new(),
        },
        &limits,
    )
    .unwrap();
    assert_eq!(response.status(), TerminalStatus::Blocked);
    assert!(response.receipts().is_empty());

    let invocation = InvocationResponseV1::use_case(response, &limits).unwrap();
    let bytes = canonical_json_bytes(&invocation, &limits).unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        value["response"]["blockers"][0]["details"]["cause"],
        "route_basis_revision_conflict"
    );
    assert_eq!(
        value["response"]["blockers"][0]["details"]["expected_revision"],
        7
    );
    assert_eq!(
        value["response"]["blockers"][0]["details"]["actual_revision"],
        8
    );

    let admitted = InvocationResponseV1::parse_and_admit(&bytes, None, &limits).unwrap();
    assert_eq!(
        admitted
            .pipeline_route_resolve_and_persist_outcome(&limits)
            .unwrap(),
        Some(Err(failure))
    );
}
