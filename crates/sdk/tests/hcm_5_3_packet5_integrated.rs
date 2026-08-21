#![cfg(feature = "dock-test-support")]

mod packet5_support;

use handbook_sdk::contract_membrane::{
    canonical_json_bytes, operation_inventory, parse_canonical_json, CapabilityConsumer,
    ContractLimits, ContractRecordBindingDtoV1, DockManifestListRequestV1,
    DockManifestReadRequestV1, DockRunResultV1, Fingerprint, IdempotencyStateRef,
    OperationTransport, ProductionWitness, RequestId, TerminalStatus, UtcTimestamp,
    DOCK_MANIFEST_LIST_REF, DOCK_MANIFEST_READ_REF, DOCK_OPERATION_REFS, DOCK_RUN_REF,
};
use serde_json::json;

#[test]
fn integrated_dock_completion_reaches_the_sdk_only_after_current_contracts_preparation() {
    let fixture = packet5_support::IntegratedDockFixture::new();
    let raw_key = "packet-five-raw-key-must-not-leak";
    let request = fixture.request(raw_key, json!({"instance": true}));
    let committed = fixture.execute(&request);

    assert_eq!(committed.terminal().status(), TerminalStatus::Ok);
    assert_eq!(committed.receipts().len(), 1);
    assert!(matches!(
        committed.idempotency().as_ref(),
        IdempotencyStateRef::Established {
            replayed: false,
            original_result_fingerprint: None,
            ..
        }
    ));
    let committed_result: DockRunResultV1 = parse_canonical_json(
        &canonical_json_bytes(
            committed.terminal().data().unwrap(),
            &ContractLimits::packet0(),
        )
        .unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap();
    assert_eq!(committed_result.candidates().len(), 1);
    let record = serde_json::to_value(committed_result.execution_record().document()).unwrap();
    assert_eq!(record["host_outcome"], "completed");
    assert_eq!(record["candidate_observation_count"], 1);

    let replayed = fixture.execute(&fixture.request(raw_key, json!({"instance": true})));
    assert_eq!(
        replayed.terminal().status(),
        TerminalStatus::Ok,
        "{}",
        serde_json::to_string_pretty(&replayed).unwrap()
    );
    assert_eq!(replayed.receipts(), committed.receipts());
    let replayed_result: DockRunResultV1 = parse_canonical_json(
        &canonical_json_bytes(
            replayed.terminal().data().unwrap(),
            &ContractLimits::packet0(),
        )
        .unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap();
    assert_eq!(
        replayed_result.execution_record(),
        committed_result.execution_record()
    );
    assert_eq!(replayed_result.candidates(), committed_result.candidates());
    assert!(matches!(
        replayed.idempotency().as_ref(),
        IdempotencyStateRef::Established {
            replayed: true,
            original_result_fingerprint: Some(original),
            ..
        } if original == committed.result_fingerprint()
    ));

    let conflicting = fixture.execute(&fixture.request(raw_key, json!({"instance": false})));
    assert_eq!(conflicting.terminal().status(), TerminalStatus::Refused);
    assert!(conflicting.receipts().is_empty());
    let conflict = serde_json::to_value(&conflicting).unwrap();
    assert_eq!(conflict["refusals"][0]["details"]["cause"], "conflict");

    for response in [&committed, &replayed, &conflicting] {
        let bytes = canonical_json_bytes(response, &ContractLimits::packet0()).unwrap();
        assert!(!bytes
            .windows(raw_key.len())
            .any(|window| window == raw_key.as_bytes()));
        assert!(!String::from_utf8(bytes)
            .unwrap()
            .contains(fixture.repository_path().to_string_lossy().as_ref()));
    }
}

#[test]
fn process_failure_commits_one_failed_record_without_partial_candidates() {
    let fixture = packet5_support::IntegratedDockFixture::process_failure();
    let raw_key = "packet-five-failure-key-must-not-leak";
    let response = fixture.execute(&fixture.request(raw_key, json!({"instance": true})));

    assert_eq!(response.terminal().status(), TerminalStatus::Ok);
    assert_eq!(response.receipts().len(), 1);
    let result: DockRunResultV1 = parse_canonical_json(
        &canonical_json_bytes(
            response.terminal().data().unwrap(),
            &ContractLimits::packet0(),
        )
        .unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap();
    assert!(result.candidates().is_empty());
    let record = serde_json::to_value(result.execution_record().document()).unwrap();
    assert_eq!(record["host_outcome"], "failed");
    assert_eq!(record["candidate_observation_count"], 0);
    assert_eq!(record["exit_code"], 7);
    let bytes = canonical_json_bytes(&response, &ContractLimits::packet0()).unwrap();
    assert!(!bytes
        .windows(raw_key.len())
        .any(|window| window == raw_key.as_bytes()));
}

#[test]
fn stale_contract_source_is_refused_before_manifest_body_or_spawn() {
    let mut fixture = packet5_support::IntegratedDockFixture::new();
    let raw_key = "packet-five-stale-key-must-not-leak";
    let request = fixture.request(raw_key, json!({"instance": true}));
    fixture.deprecate_contract();
    let before = repository_file_count(fixture.repository_path());
    let response = fixture.execute(&request);

    assert_eq!(response.terminal().status(), TerminalStatus::Refused);
    assert!(response.receipts().is_empty());
    assert_eq!(repository_file_count(fixture.repository_path()), before);
    let value = serde_json::to_value(&response).unwrap();
    assert_eq!(value["refusals"][0]["details"]["cause"], "stale");
    let bytes = canonical_json_bytes(&response, &ContractLimits::packet0()).unwrap();
    assert!(!bytes
        .windows(raw_key.len())
        .any(|window| window == raw_key.as_bytes()));
}

#[test]
fn generic_rust_sdk_consumes_all_three_rows_and_production_run_stops_before_spawn() {
    let fixture = packet5_support::IntegratedDockFixture::new();
    let limits = ContractLimits::packet0();
    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("9999-12-31T23:59:59Z").unwrap())
            .unwrap();
    assert_eq!(operation_inventory().len(), 62);
    assert_eq!(witness.capabilities().len(), 31);
    assert_eq!(witness.schema_manifest().len(), 168);
    assert_eq!(DOCK_OPERATION_REFS.len(), 3);
    assert_eq!(
        witness
            .capabilities()
            .iter()
            .filter(|entry| {
                !DOCK_OPERATION_REFS.contains(&entry.operation().reference().as_str())
            })
            .count(),
        28
    );
    let dock_roots = [
        "handbook.dock-manifest@1.0.0",
        "handbook.dock-process-request@1.0.0",
        "handbook.dock-process-result@1.0.0",
        "handbook.dock-execution-record@1.0.0",
        "handbook.dock-operation-details@1.0.0",
        "handbook.operation.dock-manifest-list-request@1.0.0",
        "handbook.operation.dock-manifest-list-result@1.0.0",
        "handbook.operation.dock-manifest-read-request@1.0.0",
        "handbook.operation.dock-manifest-read-result@1.0.0",
        "handbook.operation.dock-run-request@1.0.0",
        "handbook.operation.dock-run-result@1.0.0",
    ];
    assert_eq!(
        witness
            .schema_manifest()
            .iter()
            .filter(|entry| !dock_roots.contains(&entry.schema().reference().as_str()))
            .count(),
        157
    );
    let fixed_use_cases = [
        "handbook.use-case.repository-setup@1.0.0",
        "handbook.use-case.repository-doctor@1.0.0",
        "handbook.use-case.author-charter@1.0.0",
        "handbook.use-case.author-project-context@1.0.0",
        "handbook.use-case.approver-administration@1.0.0",
        "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
    ];
    let use_case_source = include_str!("../src/contract_membrane/facade/request.rs");
    for use_case in fixed_use_cases {
        assert!(use_case_source.contains(use_case));
        assert!(!operation_inventory()
            .iter()
            .any(|row| row.operation_ref().as_str() == use_case));
        assert!(!witness
            .capabilities()
            .iter()
            .any(|entry| entry.operation().reference().as_str() == use_case));
    }
    let dock_capabilities = witness
        .capabilities()
        .iter()
        .filter(|entry| DOCK_OPERATION_REFS.contains(&entry.operation().reference().as_str()))
        .collect::<Vec<_>>();
    assert_eq!(dock_capabilities.len(), 3);
    for capability in &dock_capabilities {
        assert_eq!(
            capability.current_transports(),
            [
                OperationTransport::CliJson,
                OperationTransport::RustSdk,
                OperationTransport::Tauri,
            ]
        );
    }

    let consumer = CapabilityConsumer::open(fixture.repository_path()).unwrap();
    let invoke = |operation_ref: &str, body: &[u8], request_id: &str| {
        let capability = dock_capabilities
            .iter()
            .find(|entry| entry.operation().reference().as_str() == operation_ref)
            .unwrap();
        consumer
            .invoke(
                capability.operation(),
                capability.entry_fingerprint(),
                OperationTransport::RustSdk,
                Some(RequestId::parse(request_id).unwrap()),
                body,
            )
            .unwrap_or_else(|error| panic!("{operation_ref}: {error:?}"))
    };
    let list_body =
        canonical_json_bytes(&DockManifestListRequestV1::first(10).unwrap(), &limits).unwrap();
    let list = invoke(DOCK_MANIFEST_LIST_REF, &list_body, "packet-five-sdk-list");
    let list = serde_json::to_value(list).unwrap();
    assert_eq!(list["response"]["status"], "ok");
    let entries = list["response"]["data"]["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(
        entries[0]["manifest"]["ref"],
        "handbook.dock.json-schema@1.0.0"
    );

    let absent = ContractRecordBindingDtoV1::new(
        "handbook.dock.absent@1.0.0",
        Fingerprint::parse(&format!("sha256:{}", "a".repeat(64))).unwrap(),
    )
    .unwrap();
    let read_body = canonical_json_bytes(&DockManifestReadRequestV1::new(absent), &limits).unwrap();
    let read = invoke(DOCK_MANIFEST_READ_REF, &read_body, "packet-five-sdk-read");
    let read = serde_json::to_value(read).unwrap();
    assert_eq!(read["response"]["status"], "ok");
    assert_eq!(read["response"]["data"]["outcome"], "absent");

    let raw_key = "packet-five-production-key-must-not-leak";
    let run_request = fixture.request(raw_key, json!({"instance": true}));
    let mut run_body = serde_json::to_value(run_request.body()).unwrap();
    run_body
        .as_object_mut()
        .unwrap()
        .insert("idempotency_key".to_owned(), json!(raw_key));
    let run_body = canonical_json_bytes(&run_body, &limits).unwrap();
    let before = repository_file_count(fixture.repository_path());
    let run = invoke(DOCK_RUN_REF, &run_body, "packet-five-sdk-production-run");
    let run = serde_json::to_value(run).unwrap();
    assert_eq!(run["response"]["status"], "refused");
    assert!(run["response"]["receipts"].as_array().unwrap().is_empty());
    assert_eq!(repository_file_count(fixture.repository_path()), before);
    assert_eq!(
        run["response"]["refusals"][0]["details"]["cause"],
        "malformed"
    );
    let bytes = canonical_json_bytes(&run, &limits).unwrap();
    assert!(!bytes
        .windows(raw_key.len())
        .any(|window| window == raw_key.as_bytes()));
}

fn repository_file_count(root: &std::path::Path) -> usize {
    fn visit(path: &std::path::Path) -> usize {
        if path.is_dir() {
            std::fs::read_dir(path)
                .unwrap()
                .map(|entry| visit(&entry.unwrap().path()))
                .sum()
        } else {
            1
        }
    }
    visit(root)
}
