use handbook_sdk::contract_membrane::{
    canonical_json_bytes, dock_manifest_list_operation_request,
    dock_manifest_read_operation_request, dock_run_operation_request, parse_canonical_json,
    BootstrapFacadeResponseV1, BootstrapRequestV1, BoundedString, BoundedVec, CatalogKindV1,
    CatalogPageSize, CatalogReadRequestV1, CatalogReadResponseBodyV1, CatalogReadResponseV1,
    ContractLimits, ContractRecordBindingDtoV1, ContractValue, DockEvaluationPreparationDtoV1,
    DockManifestListRequestV1, DockManifestReadRequestV1, DockProtocolLimitsDtoV1,
    DockProtocolLimitsInputV1, DockRunRequestV1, Fingerprint, InvocationRequestV1,
    ProductionWitness, SemVer, UtcTimestamp,
};
use handbook_tauri::{FacadeState, TrustedClock};
use serde_json::json;
use std::collections::BTreeMap;
use tauri::ipc::{CallbackFn, InvokeBody, InvokeResponseBody};
use tauri::test::{
    get_ipc_response, mock_builder, mock_context, noop_assets, MockRuntime, INVOKE_KEY,
};
use tauri::webview::InvokeRequest;

const OPERATIONS: [&str; 3] = [
    "dock.manifest.list@1.0.0",
    "dock.manifest.read@1.0.0",
    "dock.run@1.0.0",
];

#[derive(Clone)]
struct FixedClock(UtcTimestamp);

impl TrustedClock for FixedClock {
    fn now_utc(&self) -> Option<UtcTimestamp> {
        Some(self.0.clone())
    }
}

fn invoke_ipc(
    webview: &tauri::WebviewWindow<MockRuntime>,
    command: &str,
    request: impl serde::Serialize,
) -> Vec<u8> {
    match get_ipc_response(
        webview,
        InvokeRequest {
            cmd: command.into(),
            callback: CallbackFn(0),
            error: CallbackFn(1),
            url: if cfg!(any(windows, target_os = "android")) {
                "http://tauri.localhost"
            } else {
                "tauri://localhost"
            }
            .parse()
            .unwrap(),
            body: InvokeBody::Json(json!({"request": request})),
            headers: Default::default(),
            invoke_key: INVOKE_KEY.to_string(),
        },
    )
    .unwrap_or_else(|error| panic!("{command}: {error:?}"))
    {
        InvokeResponseBody::Json(value) => value.into_bytes(),
        InvokeResponseBody::Raw(value) => value,
    }
}

fn binding(reference: &str, hex: char) -> ContractRecordBindingDtoV1 {
    ContractRecordBindingDtoV1::new(
        reference,
        Fingerprint::parse(&format!("sha256:{}", hex.to_string().repeat(64))).unwrap(),
    )
    .unwrap()
}

fn run_request(raw_key: &str) -> handbook_sdk::contract_membrane::OperationRequestV1 {
    let limits = ContractLimits::packet0();
    let empty: ContractValue = parse_canonical_json(b"{}", &limits).unwrap();
    let evaluation = DockEvaluationPreparationDtoV1::new(
        binding("handbook.contract.absent@1.0.0", 'b'),
        Fingerprint::parse(&format!("sha256:{}", "c".repeat(64))).unwrap(),
        empty.clone(),
        BoundedVec::new(Vec::new()).unwrap(),
        empty.clone(),
        empty,
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        binding("handbook.contract-gate-policy.absent@1.0.0", 'd'),
        binding("handbook.contract-local-closeout-policy.absent@1.0.0", 'e'),
        BoundedString::new("packet-five-tauri-run".to_owned()).unwrap(),
    );
    let body = DockRunRequestV1::new(
        binding("handbook.dock.absent@1.0.0", 'a'),
        evaluation,
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        DockProtocolLimitsDtoV1::new(DockProtocolLimitsInputV1 {
            request_bytes: 4096,
            result_bytes: 4096,
            stderr_bytes: 1024,
            diagnostic_count: 4,
            diagnostic_bytes: 1024,
            candidate_count: 4,
            candidate_bytes: 4096,
            input_count: 4,
            wall_timeout_ms: 1000,
            cancellation_grace_ms: 100,
        })
        .unwrap(),
    );
    dock_run_operation_request(Some("packet-five-tauri-run"), raw_key, body).unwrap()
}

#[test]
fn generic_tauri_facade_invokes_the_three_dock_operations_without_a_new_command() {
    let repo = tempfile::tempdir().unwrap();
    let retention = UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap();
    let now = UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap();
    let witness = ProductionWitness::from_current(retention.clone()).unwrap();
    let state = FacadeState::open(repo.path(), retention, FixedClock(now)).unwrap();
    let app = mock_builder()
        .manage(state)
        .invoke_handler(handbook_tauri::invoke_handler())
        .build(mock_context(noop_assets()))
        .unwrap();
    let webview = tauri::WebviewWindowBuilder::new(&app, "packet-five", Default::default())
        .build()
        .unwrap();
    let limits = ContractLimits::packet0();

    let bootstrap_request = BootstrapRequestV1::new(
        witness.descriptor().schemas().bootstrap_request().clone(),
        None,
        witness.descriptor().binding(),
        SemVer::parse("1.0.0").unwrap(),
        CatalogPageSize::new(128).unwrap(),
        &limits,
    )
    .unwrap();
    let bootstrap: BootstrapFacadeResponseV1 = serde_json::from_slice(&invoke_ipc(
        &webview,
        "handbook_bootstrap_v1",
        bootstrap_request,
    ))
    .unwrap();
    let bootstrap = serde_json::to_value(bootstrap).unwrap();
    assert_eq!(
        bootstrap["response"]["schema_manifest"]
            .as_array()
            .unwrap()
            .len(),
        168
    );

    let catalog_request = CatalogReadRequestV1::new(
        CatalogKindV1::Capabilities,
        CatalogPageSize::new(128).unwrap(),
        None,
        &limits,
    )
    .unwrap();
    let catalog: CatalogReadResponseV1 = serde_json::from_slice(&invoke_ipc(
        &webview,
        "handbook_catalog_read_v1",
        catalog_request,
    ))
    .unwrap();
    let CatalogReadResponseBodyV1::CapabilityPage(page) = catalog.response() else {
        panic!("Tauri capability read returned a non-capability page");
    };
    assert_eq!(page.root().total_entries().get(), 31);
    let capabilities = page
        .entries()
        .iter()
        .filter(|entry| OPERATIONS.contains(&entry.operation().reference().as_str()))
        .map(|entry| (entry.operation().reference().as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(capabilities.len(), 3);
    for operation_ref in OPERATIONS {
        let capability = capabilities.get(operation_ref).unwrap();
        assert_eq!(
            serde_json::to_value(capability.current_transports()).unwrap(),
            json!(["cli_json", "rust_sdk", "tauri"])
        );
    }

    let raw_key = "packet-five-tauri-key-must-not-leak";
    let requests = [
        dock_manifest_list_operation_request(
            Some("packet-five-tauri-list"),
            DockManifestListRequestV1::first(10).unwrap(),
        )
        .unwrap(),
        dock_manifest_read_operation_request(
            Some("packet-five-tauri-read"),
            DockManifestReadRequestV1::new(binding("handbook.dock.absent@1.0.0", 'a')),
        )
        .unwrap(),
        run_request(raw_key),
    ];
    let sdk = handbook_sdk::HandbookSdkV1::open(repo.path());
    for request in requests {
        let operation_ref = request.operation().reference().as_str().to_owned();
        let invocation = InvocationRequestV1::operation(request, &limits).unwrap();
        let expected = sdk.invoke_tauri(&invocation, &limits).unwrap();
        let actual: serde_json::Value =
            serde_json::from_slice(&invoke_ipc(&webview, "handbook_invoke_v1", invocation))
                .unwrap();
        assert_eq!(actual, serde_json::to_value(expected).unwrap());
        assert_eq!(actual["response_kind"], "operation");
        assert!(actual["response"]["receipts"]
            .as_array()
            .unwrap()
            .is_empty());
        match operation_ref.as_str() {
            "dock.manifest.list@1.0.0" => {
                assert_eq!(actual["response"]["status"], "ok");
                assert_eq!(actual["response"]["data"]["entries"], json!([]));
            }
            "dock.manifest.read@1.0.0" => {
                assert_eq!(actual["response"]["status"], "ok");
                assert_eq!(actual["response"]["data"]["outcome"], "absent");
            }
            "dock.run@1.0.0" => {
                assert_eq!(actual["response"]["status"], "refused");
                assert_eq!(
                    actual["response"]["refusals"][0]["details"]["cause"],
                    "malformed"
                );
                assert_eq!(
                    actual["response"]["idempotency"]["state"],
                    "not_established"
                );
            }
            _ => unreachable!(),
        }
        let bytes = canonical_json_bytes(&actual, &limits).unwrap();
        assert!(!bytes
            .windows(raw_key.len())
            .any(|window| window == raw_key.as_bytes()));
    }
}
