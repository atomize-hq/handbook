#[path = "../../sdk/tests/support/hcm_4_6_snapshot_support.rs"]
mod support;

use handbook_sdk::contract_membrane::UtcTimestamp;
use handbook_tauri::{FacadeState, TrustedClock, COMMAND_IDS};
use serde_json::{json, Value};
use support::{capture_body, operation_invocation, target_body, CompleteSnapshotSource};
use tauri::ipc::{CallbackFn, InvokeBody, InvokeResponseBody};
use tauri::test::{
    get_ipc_response, mock_builder, mock_context, noop_assets, MockRuntime, INVOKE_KEY,
};
use tauri::webview::InvokeRequest;

#[derive(Clone)]
struct FixedClock(UtcTimestamp);
impl TrustedClock for FixedClock {
    fn now_utc(&self) -> Option<UtcTimestamp> {
        Some(self.0.clone())
    }
}

fn invoke(webview: &tauri::WebviewWindow<MockRuntime>, request: impl serde::Serialize) -> Value {
    let body = match get_ipc_response(
        webview,
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
            body: InvokeBody::Json(json!({"request": request})),
            headers: Default::default(),
            invoke_key: INVOKE_KEY.to_string(),
        },
    )
    .unwrap()
    {
        InvokeResponseBody::Json(value) => value.into_bytes(),
        InvokeResponseBody::Raw(value) => value,
    };
    serde_json::from_slice(&body).unwrap()
}

#[test]
fn unchanged_generic_tauri_command_preserves_four_operation_results_and_exact_replay() {
    let repo = tempfile::tempdir().unwrap();
    let state = FacadeState::open_with_snapshot_source(
        repo.path(),
        UtcTimestamp::parse("2031-01-01T00:00:00Z").unwrap(),
        FixedClock(UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap()),
        CompleteSnapshotSource::new(),
    )
    .unwrap();
    let app = mock_builder()
        .manage(state)
        .invoke_handler(handbook_tauri::invoke_handler())
        .build(mock_context(noop_assets()))
        .unwrap();
    let webview = tauri::WebviewWindowBuilder::new(&app, "packet-six", Default::default())
        .build()
        .unwrap();

    let start = || {
        operation_invocation(
            "snapshot.capture@1.0.0",
            capture_body("tauri-packet-6", "start", "tauri-packet-6-start-key"),
        )
    };
    let started = invoke(&webview, start());
    assert_eq!(started["response"]["data"]["outcome"], "started");
    assert_eq!(
        started["response"]["data"]["currentness"],
        "owner_derived_current"
    );
    assert_eq!(started["response"]["receipts"].as_array().unwrap().len(), 1);
    let replay = invoke(&webview, start());
    assert_eq!(replay["response"]["data"]["replayed"], true);
    assert_eq!(
        replay["response"]["data"]["receipt"],
        started["response"]["data"]["receipt"]
    );

    let target = target_body(&started);
    for operation in [
        "snapshot.read@1.0.0",
        "snapshot.delta@1.0.0",
        "snapshot.project@1.0.0",
    ] {
        let value = invoke(&webview, operation_invocation(operation, target.clone()));
        assert_eq!(value["response_kind"], "operation");
        assert_eq!(value["response"]["status"], "ok");
        if operation == "snapshot.project@1.0.0" {
            assert_eq!(
                value["response"]["data"]["resolution_stack"]["ref"],
                "handbook.context-resolution-stack.packet-6@1.0.0"
            );
            assert_eq!(
                value["response"]["data"]["omissions"]
                    .as_array()
                    .unwrap()
                    .len(),
                3
            );
        }
        let text = serde_json::to_string(&value).unwrap();
        assert!(!text.contains("tauri-packet-6-start-key"));
        assert!(!text.contains(repo.path().to_string_lossy().as_ref()));
        assert!(!text.contains("handbook.snapshot-observation."));
    }
    assert_eq!(
        COMMAND_IDS,
        [
            "handbook_bootstrap_v1",
            "handbook_catalog_read_v1",
            "handbook_invoke_v1"
        ]
    );
}
