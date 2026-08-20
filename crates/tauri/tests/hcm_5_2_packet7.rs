use handbook_sdk::contract_membrane::{
    canonical_json_bytes, contract_evidence_append_operation_request,
    contract_evidence_list_operation_request, contract_evidence_read_operation_request,
    contract_gate_evaluate_operation_request, contract_verdict_evaluate_operation_request,
    parse_canonical_json, BootstrapFacadeResponseV1, BootstrapRequestV1, CatalogKindV1,
    CatalogPageSize, CatalogReadRequestV1, CatalogReadResponseBodyV1, CatalogReadResponseV1,
    ContractEvaluationRunDtoV1, ContractEvidenceAppendRequestV1, ContractEvidenceListRequestV1,
    ContractEvidenceReadRequestV1, ContractGateEvaluateRequestV1, ContractLimits,
    ContractRecordBindingDtoV1, ContractValue, ContractVerdictEvaluateRequestV1, Fingerprint,
    InvocationRequestV1, ProductionWitness, SemVer, UtcTimestamp,
};
use handbook_tauri::{FacadeState, TrustedClock};
use serde_json::json;
use std::collections::BTreeMap;
use tauri::ipc::{CallbackFn, InvokeBody, InvokeResponseBody};
use tauri::test::{
    get_ipc_response, mock_builder, mock_context, noop_assets, MockRuntime, INVOKE_KEY,
};
use tauri::webview::InvokeRequest;

const OPERATIONS: [&str; 5] = [
    "contract.evidence.list@1.0.0",
    "contract.evidence.read@1.0.0",
    "contract.evidence.append@1.0.0",
    "contract.verdict.evaluate@1.0.0",
    "contract.gate.evaluate@1.0.0",
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

#[test]
fn generic_tauri_facade_invokes_all_five_evidence_operations_without_a_new_command() {
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
    let webview = tauri::WebviewWindowBuilder::new(&app, "packet-seven", Default::default())
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
    let bootstrap_value = serde_json::to_value(&bootstrap).unwrap();
    assert_eq!(
        bootstrap_value["response"]["schema_manifest"]
            .as_array()
            .unwrap()
            .len(),
        157
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
    assert_eq!(page.root().total_entries().get(), 28);
    let capabilities = page
        .entries()
        .iter()
        .filter(|entry| OPERATIONS.contains(&entry.operation().reference().as_str()))
        .map(|entry| (entry.operation().reference().as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(capabilities.len(), OPERATIONS.len());
    for operation_ref in OPERATIONS {
        let capability = capabilities.get(operation_ref).unwrap();
        assert_eq!(
            serde_json::to_value(capability.current_transports()).unwrap(),
            json!(["cli_json", "rust_sdk", "tauri"])
        );
        for schema_binding in [
            capability.request_schema(),
            capability.result_schema(),
            capability.outcome_schemas().blocker(),
            capability.outcome_schemas().refusal(),
            capability.outcome_schemas().error(),
        ] {
            assert!(bootstrap_value["response"]["schema_manifest"]
                .as_array()
                .unwrap()
                .iter()
                .any(|entry| {
                    entry["schema"]["ref"] == schema_binding.reference().as_str()
                        && entry["schema"]["fingerprint"] == schema_binding.fingerprint().as_str()
                }));
        }
    }

    let empty_document: ContractValue = parse_canonical_json(b"{}", &limits).unwrap();
    let run = ContractEvaluationRunDtoV1::new(
        binding(
            "handbook.contract.packet-seven@1.0.0#evaluation/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            'a',
        ),
        empty_document,
    );
    let raw_key = "packet-seven-tauri-raw-key-must-not-leak";
    let operation_requests = [
        contract_evidence_list_operation_request(
            Some("packet-seven-tauri-list"),
            ContractEvidenceListRequestV1::first(10).unwrap(),
        )
        .unwrap(),
        contract_evidence_read_operation_request(
            Some("packet-seven-tauri-read"),
            ContractEvidenceReadRequestV1::new(binding(
                "handbook.contract.packet-seven@1.0.0#evidence/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                'a',
            )),
        )
        .unwrap(),
        contract_evidence_append_operation_request(
            Some("packet-seven-tauri-append"),
            raw_key,
            ContractEvidenceAppendRequestV1::new(binding(
                "handbook.evidence-candidate.packet-seven@1.0.0",
                'b',
            )),
        )
        .unwrap(),
        contract_verdict_evaluate_operation_request(
            Some("packet-seven-tauri-verdict"),
            ContractVerdictEvaluateRequestV1::new(run.clone()),
        )
        .unwrap(),
        contract_gate_evaluate_operation_request(
            Some("packet-seven-tauri-gate"),
            ContractGateEvaluateRequestV1::new(run),
        )
        .unwrap(),
    ];
    let sdk = handbook_sdk::HandbookSdkV1::open(repo.path());

    for request in operation_requests {
        let operation_ref = request.operation().reference().as_str().to_owned();
        let invocation = InvocationRequestV1::operation(request, &limits).unwrap();
        let expected = sdk.invoke_tauri(&invocation, &limits).unwrap();
        let actual: serde_json::Value =
            serde_json::from_slice(&invoke_ipc(&webview, "handbook_invoke_v1", invocation))
                .unwrap();
        assert_eq!(actual, serde_json::to_value(&expected).unwrap());
        assert_eq!(actual["response_kind"], "operation");
        assert!(actual["response"]["receipts"]
            .as_array()
            .unwrap()
            .is_empty());
        match operation_ref.as_str() {
            "contract.evidence.list@1.0.0" => {
                assert_eq!(actual["response"]["status"], "ok");
                assert_eq!(actual["response"]["data"]["entries"], json!([]));
            }
            "contract.evidence.read@1.0.0" => {
                assert_eq!(actual["response"]["status"], "ok");
                assert_eq!(actual["response"]["data"]["outcome"], "absent");
            }
            "contract.evidence.append@1.0.0" => {
                assert_eq!(actual["response"]["status"], "error");
                assert_eq!(
                    actual["response"]["errors"][0]["details"]["cause"],
                    "candidate_source_unavailable"
                );
            }
            "contract.verdict.evaluate@1.0.0" | "contract.gate.evaluate@1.0.0" => {
                assert_eq!(actual["response"]["status"], "error");
                assert_eq!(
                    actual["response"]["errors"][0]["details"]["cause"],
                    "repository_unavailable"
                );
            }
            _ => unreachable!(),
        }
        let bytes = canonical_json_bytes(&actual, &limits).unwrap();
        assert!(!bytes
            .windows(raw_key.len())
            .any(|window| window == raw_key.as_bytes()));
        assert!(!String::from_utf8(bytes)
            .unwrap()
            .contains(repo.path().to_string_lossy().as_ref()));
    }
}
