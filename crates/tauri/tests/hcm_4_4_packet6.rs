use handbook_sdk::contract_membrane::{
    canonical_json_bytes, operation_inventory, pipeline_catalog_list_operation_request,
    BootstrapFacadeResponseV1, CatalogReadRequestV1, CatalogReadResponseV1, ContractLimits,
    DiscoveryState, InvocationRequestV1, OperationTransport, ProductionWitness, UseCaseRequestV1,
    UtcTimestamp,
};
use handbook_tauri::{HostError, COMMAND_IDS};
use serde_json::{json, Value};
use std::any::type_name;
use std::collections::BTreeSet;
use tauri::ipc::{CallbackFn, InvokeBody};
use tauri::test::{get_ipc_response, mock_builder, mock_context, noop_assets, INVOKE_KEY};
use tauri::webview::InvokeRequest;

const COMMANDS: [&str; 3] = [
    "handbook_bootstrap_v1",
    "handbook_catalog_read_v1",
    "handbook_invoke_v1",
];

const FACADE_ROOTS: [&str; 5] = [
    "handbook.bootstrap-facade-response@1.0.0",
    "handbook.catalog-read-request@1.0.0",
    "handbook.catalog-read-response@1.0.0",
    "handbook.invocation-request@1.0.0",
    "handbook.use-case-request@1.0.0",
];

const OPERATIONS: [&str; 19] = [
    "artifact.kind.list@1.0.0",
    "artifact.instance.list@1.0.0",
    "artifact.read@1.0.0",
    "artifact.validate@1.0.0",
    "intake.definition.read@1.0.0",
    "intake.coverage.evaluate@1.0.0",
    "intake.record.append@1.0.0",
    "artifact.candidate.validate@1.0.0",
    "artifact.candidate.append@1.0.0",
    "artifact.candidate.promote@1.0.0",
    "flow.resolve@1.0.0",
    "pipeline.catalog.list@1.0.0",
    "pipeline.catalog.read@1.0.0",
    "pipeline.compile@1.0.0",
    "pipeline.capture.plan@1.0.0",
    "pipeline.capture.apply@1.0.0",
    "pipeline.handoff.emit@1.0.0",
    "pipeline.state.apply@1.0.0",
    "posture.transition.apply@1.0.0",
];

const USE_CASES: [&str; 6] = [
    "handbook.use-case.repository-setup@1.0.0",
    "handbook.use-case.repository-doctor@1.0.0",
    "handbook.use-case.author-charter@1.0.0",
    "handbook.use-case.author-project-context@1.0.0",
    "handbook.use-case.approver-administration@1.0.0",
    "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
];

#[test]
fn integrated_facade_surface_closes_exact_commands_contracts_routes_and_omissions() {
    assert_eq!(COMMAND_IDS, COMMANDS);

    let public_root_types = [
        type_name::<InvocationRequestV1>(),
        type_name::<UseCaseRequestV1>(),
        type_name::<BootstrapFacadeResponseV1>(),
        type_name::<CatalogReadRequestV1>(),
        type_name::<CatalogReadResponseV1>(),
    ];
    assert_eq!(public_root_types.len(), FACADE_ROOTS.len());

    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap())
            .unwrap();
    let manifest_refs = witness
        .schema_manifest()
        .iter()
        .map(|entry| entry.schema().reference().as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(manifest_refs.len(), 120);
    assert_eq!(
        manifest_refs
            .intersection(&FACADE_ROOTS.into_iter().collect())
            .copied()
            .collect::<BTreeSet<_>>(),
        FACADE_ROOTS.into_iter().collect()
    );

    let expected_operations = OPERATIONS.into_iter().collect::<BTreeSet<_>>();
    let expected_use_cases = USE_CASES.into_iter().collect::<BTreeSet<_>>();
    let rows = operation_inventory();
    assert_eq!(rows.len(), 62);
    let inventory_refs = rows
        .iter()
        .map(|row| row.operation_ref().as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(inventory_refs.len(), 62);
    assert!(expected_use_cases.is_disjoint(&inventory_refs));

    let discoverable = rows
        .iter()
        .filter(|row| row.discovery() == DiscoveryState::Discoverable)
        .map(|row| row.operation_ref().as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(discoverable, expected_operations);

    let capability_refs = witness
        .capabilities()
        .iter()
        .map(|entry| entry.operation().reference().as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(capability_refs, expected_operations);
    assert!(expected_use_cases.is_disjoint(&capability_refs));

    for row in rows {
        let operation_ref = row.operation_ref().as_str();
        if operation_ref == "posture.transition.apply@1.0.0" {
            assert_eq!(
                row.current_transports(),
                &[OperationTransport::RustSdk, OperationTransport::Tauri]
            );
        } else if expected_operations.contains(operation_ref) {
            assert_eq!(
                row.current_transports(),
                &[OperationTransport::CliJson, OperationTransport::Tauri],
                "{operation_ref}"
            );
        } else {
            assert_eq!(row.discovery(), DiscoveryState::Omit, "{operation_ref}");
            assert!(row.current_transports().is_empty(), "{operation_ref}");
        }
    }
    assert_eq!(
        rows.iter()
            .filter(|row| row.current_transports().is_empty())
            .count(),
        43
    );

    let limits = ContractLimits::packet0();
    let request = InvocationRequestV1::operation(
        pipeline_catalog_list_operation_request(Some("packet-6-proof")).unwrap(),
        &limits,
    )
    .unwrap();
    for host_field in [
        "repository_root",
        "trusted_time",
        "retention_until_utc",
        "limits",
        "sdk_state",
    ] {
        let mut host_substitution = serde_json::to_value(&request).unwrap();
        host_substitution[host_field] = json!("frontend-controlled");
        assert!(
            serde_json::from_value::<InvocationRequestV1>(host_substitution).is_err(),
            "IPC admitted host-managed field {host_field}"
        );
    }

    let mut fingerprint_substitution = serde_json::to_value(&request).unwrap();
    fingerprint_substitution["request_fingerprint"] =
        Value::String(format!("sha256:{}", "0".repeat(64)));
    let substituted_bytes = canonical_json_bytes(&fingerprint_substitution, &limits).unwrap();
    assert!(InvocationRequestV1::parse_and_admit(&substituted_bytes, &limits).is_err());

    let host_errors = [
        HostError::RepositoryUnavailable,
        HostError::WitnessUnavailable,
        HostError::ClockUnavailable,
        HostError::ContractUnavailable,
        HostError::RuntimeUnavailable,
    ]
    .map(|error| serde_json::to_value(error).unwrap());
    assert_eq!(
        host_errors,
        [
            json!({"code": "repository_unavailable"}),
            json!({"code": "witness_unavailable"}),
            json!({"code": "clock_unavailable"}),
            json!({"code": "contract_unavailable"}),
            json!({"code": "runtime_unavailable"}),
        ]
    );

    let ipc_secret = "packet-6-secret-must-not-echo";
    let mut secret_bearing_request = serde_json::to_value(&request).unwrap();
    secret_bearing_request["repository_root"] = json!(ipc_secret);
    let app = mock_builder()
        .invoke_handler(handbook_tauri::invoke_handler())
        .build(mock_context(noop_assets()))
        .unwrap();
    let webview = tauri::WebviewWindowBuilder::new(&app, "packet-6", Default::default())
        .build()
        .unwrap();
    let rejection = get_ipc_response(
        &webview,
        InvokeRequest {
            cmd: "handbook_invoke_v1".into(),
            callback: CallbackFn(0),
            error: CallbackFn(1),
            url: if cfg!(any(windows, target_os = "android")) {
                "http://tauri.localhost"
            } else {
                "tauri://localhost"
            }
            .parse()
            .unwrap(),
            body: InvokeBody::Json(json!({"request": secret_bearing_request})),
            headers: Default::default(),
            invoke_key: INVOKE_KEY.to_string(),
        },
    )
    .expect_err("generated Tauri handler admitted a host-managed IPC field");
    let rejection_bytes = serde_json::to_vec(&rejection).unwrap();
    assert!(rejection_bytes.len() <= 512, "unbounded IPC rejection");
    assert!(
        !rejection_bytes
            .windows(ipc_secret.len())
            .any(|window| window == ipc_secret.as_bytes()),
        "IPC rejection echoed secret-bearing input"
    );
}
