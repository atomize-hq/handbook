use handbook_sdk::contract_membrane::{
    admit_operation_definition, canonical_json_bytes, flow_resolution_from_response,
    flow_resolve_operation_definition, flow_resolve_operation_request, operation_inventory,
    parse_canonical_json, AdmissionReason, ContractLimits, ContractValue, DiscoveryState,
    FlowResolveSchemaValidator, InvocationResponseV1, OperationAdmissionResult,
    OperationSchemaValidator, OperationTransport, ProductionWitness, TerminalStatus, UtcTimestamp,
};
use handbook_sdk::HandbookSdkV1;

#[test]
fn flow_resolve_definition_is_exact_independently_admitted_and_the_only_new_row() {
    let definition = flow_resolve_operation_definition().unwrap();
    assert_eq!(definition.operation().as_str(), "flow.resolve@1.0.0");
    assert!(matches!(
        admit_operation_definition(&definition).unwrap(),
        OperationAdmissionResult::Admitted(_)
    ));

    let row = operation_inventory()
        .iter()
        .find(|row| row.operation_ref().as_str() == "flow.resolve@1.0.0")
        .unwrap();
    assert_eq!(row.admission_reason(), AdmissionReason::Admitted);
    assert_eq!(row.discovery(), DiscoveryState::Discoverable);
    assert_eq!(
        row.current_transports(),
        &[OperationTransport::CliJson, OperationTransport::Tauri]
    );
    assert_eq!(operation_inventory().len(), 62);
    assert_eq!(
        operation_inventory()
            .iter()
            .filter(|row| row.admission_reason() == AdmissionReason::Admitted)
            .count(),
        31
    );

    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap())
            .unwrap();
    assert_eq!(
        witness
            .capabilities()
            .iter()
            .filter(|entry| entry.operation().reference().as_str() == "flow.resolve@1.0.0")
            .count(),
        1
    );
}

#[test]
fn request_admission_freezes_the_three_packet_ids_and_rejects_unknown_values() {
    for packet_id in [
        "planning.packet",
        "execution.demo.packet",
        "execution.live.packet",
    ] {
        let request = flow_resolve_operation_request(None, packet_id).unwrap();
        assert_eq!(
            request.operation().reference().as_str(),
            "flow.resolve@1.0.0"
        );
        assert_eq!(
            request.body_schema().reference().as_str(),
            "handbook.operation.flow-resolve-request@1.0.0"
        );
    }
    assert!(flow_resolve_operation_request(None, "unknown.packet").is_err());
    assert!(flow_resolve_operation_request(None, "").is_err());
}

#[test]
fn exact_flow_schemas_reject_extra_fields_missing_results_and_non_recheckable_causes() {
    let limits = ContractLimits::packet0();
    let definition = flow_resolve_operation_definition().unwrap();
    for invalid_request in [
        br#"{"extra":true,"packet_id":"planning.packet"}"#.as_slice(),
        br#"{"packet_id":"unknown.packet"}"#.as_slice(),
    ] {
        let value = parse_canonical_json::<ContractValue>(invalid_request, &limits).unwrap();
        assert!(FlowResolveSchemaValidator
            .validate_instance(definition.request_schema(), &value)
            .is_err());
    }

    let missing_result = parse_canonical_json::<ContractValue>(b"{}", &limits).unwrap();
    assert!(FlowResolveSchemaValidator
        .validate_instance(definition.result_schema(), &missing_result)
        .is_err());

    let repo = tempfile::tempdir().unwrap();
    let request = flow_resolve_operation_request(None, "planning.packet").unwrap();
    let response = HandbookSdkV1::open(repo.path())
        .execute_flow_operation(&request)
        .unwrap();
    let blocker = &response.terminal().blockers()[0];
    assert!(FlowResolveSchemaValidator
        .validate_problem(TerminalStatus::Error, blocker)
        .is_err());
    let details_bytes = canonical_json_bytes(blocker.details(), &limits).unwrap();
    let details_json: serde_json::Value = serde_json::from_slice(&details_bytes).unwrap();
    assert_eq!(details_json["origin"], "resolution");
    let resolution_json = details_json["resolution"].clone();
    let as_contract = |value: &serde_json::Value| {
        let bytes = canonical_json_bytes(value, &limits).unwrap();
        parse_canonical_json::<ContractValue>(&bytes, &limits).unwrap()
    };

    let mut contradictory_packet = resolution_json.clone();
    contradictory_packet["selection"]["packet_id"] =
        serde_json::Value::String("execution.live.packet".to_owned());
    assert!(FlowResolveSchemaValidator
        .validate_instance(
            definition.result_schema(),
            &as_contract(&contradictory_packet)
        )
        .is_err());
    let mut contradictory_count = resolution_json.clone();
    contradictory_count["packet_result"]["decision_summary"]["decision_log_entries"] =
        serde_json::Value::from(999_u64);
    assert!(FlowResolveSchemaValidator
        .validate_instance(
            definition.result_schema(),
            &as_contract(&contradictory_count)
        )
        .is_err());

    for tampered_details in [
        {
            let mut value = details_json.clone();
            value["resolution"] = serde_json::Value::Null;
            value
        },
        {
            let mut value = details_json.clone();
            value["cause"] = serde_json::Value::String("unsupported_request".to_owned());
            value
        },
        {
            let mut value = details_json.clone();
            value["origin"] = serde_json::Value::String("owner_error".to_owned());
            value
        },
    ] {
        assert!(FlowResolveSchemaValidator
            .validate_instance(blocker.details_schema(), &as_contract(&tampered_details))
            .is_err());
    }

    let extra_detail = parse_canonical_json::<ContractValue>(
        br#"{"cause":"system_root_missing","detail":"x","origin":"resolution","resolution":null,"unexpected":true}"#,
        &limits,
    )
    .unwrap();
    assert!(FlowResolveSchemaValidator
        .validate_instance(blocker.details_schema(), &extra_detail)
        .is_err());
    let invalid_recheck =
        parse_canonical_json::<ContractValue>(br#"{"cause":"unsupported_request"}"#, &limits)
            .unwrap();
    assert!(FlowResolveSchemaValidator
        .validate_instance(
            blocker.recheck().unwrap().condition_schema(),
            &invalid_recheck,
        )
        .is_err());
}

#[test]
fn missing_repository_truth_is_one_blocked_response_with_the_complete_owner_result() {
    let repo = tempfile::tempdir().unwrap();
    let request = flow_resolve_operation_request(None, "planning.packet").unwrap();
    let response = HandbookSdkV1::open(repo.path())
        .execute_flow_operation(&request)
        .unwrap();

    assert_eq!(response.terminal().status(), TerminalStatus::Blocked);
    assert_eq!(response.terminal().blockers().len(), 1);
    assert!(response.terminal().refusals().is_empty());
    assert!(response.terminal().errors().is_empty());
    assert!(response.receipts().is_empty());

    let resolution = flow_resolution_from_response(&response).unwrap().unwrap();
    assert_eq!(resolution.selection.packet_id, "planning.packet");
    assert!(
        resolution.refusal.is_some(),
        "legacy refusal fact is retained"
    );
    assert!(!resolution.blockers.is_empty());
    assert!(!resolution.decision_log.entries.is_empty());
    assert_eq!(resolution.budget_outcome.targets.len(), 0);

    let definition = flow_resolve_operation_definition().unwrap();
    let invocation =
        InvocationResponseV1::operation_response(response, &ContractLimits::packet0()).unwrap();
    let bytes = canonical_json_bytes(&invocation, &ContractLimits::packet0()).unwrap();
    let admitted = InvocationResponseV1::parse_and_admit(
        &bytes,
        Some((&definition, &FlowResolveSchemaValidator)),
        &ContractLimits::packet0(),
    )
    .unwrap();
    assert_eq!(admitted.terminal_status(), TerminalStatus::Blocked);

    let wire = String::from_utf8(bytes).unwrap();
    assert!(!wire.contains(repo.path().to_string_lossy().as_ref()));
    assert!(!wire.contains("idempotency_key"));
    assert!(wire.contains("fixture_context"));
    assert!(wire.contains("budget_outcome"));
}
