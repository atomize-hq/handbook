#[path = "../../sdk/tests/support/hcm_4_6_snapshot_support.rs"]
mod support;

use handbook_sdk::contract_membrane::{canonical_json_bytes, sha256_fingerprint, ContractLimits};
use handbook_sdk::HandbookSdkV1;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::{Command, Output};
use support::{capture_body, operation_invocation, target_body, CompleteSnapshotSource};

const OPERATIONS: [&str; 4] = [
    "snapshot.capture@1.0.0",
    "snapshot.read@1.0.0",
    "snapshot.delta@1.0.0",
    "snapshot.project@1.0.0",
];

fn run(root: &Path, args: &[String]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
        .current_dir(root)
        .args(args)
        .output()
        .unwrap()
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn document(output: &Output) -> Value {
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1
    );
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout).unwrap()
}

fn capabilities(root: &Path) -> BTreeMap<String, Value> {
    let output = run(
        root,
        &strings(&[
            "capability",
            "catalog-read",
            "--catalog-kind",
            "capabilities",
            "--page-size",
            "128",
        ]),
    );
    assert!(output.status.success());
    let value = document(&output);
    assert_eq!(value["response"]["root"]["total_entries"], 35);
    value["response"]["entries"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            (
                entry["operation"]["ref"].as_str().unwrap().to_owned(),
                entry.clone(),
            )
        })
        .collect()
}

fn invoke(root: &Path, capability: &Value, body: &[u8], authorize: bool) -> Output {
    let files = tempfile::tempdir().unwrap();
    let body_path = files.path().join("body.json");
    std::fs::write(&body_path, body).unwrap();
    let mut args = vec![
        "capability".into(),
        "invoke".into(),
        "--operation-ref".into(),
        capability["operation"]["ref"].as_str().unwrap().into(),
        "--definition-fingerprint".into(),
        capability["operation"]["fingerprint"]
            .as_str()
            .unwrap()
            .into(),
        "--capability-entry-fingerprint".into(),
        capability["entry_fingerprint"].as_str().unwrap().into(),
        "--from-input".into(),
        body_path.to_string_lossy().into_owned(),
    ];
    if authorize {
        let path = files.path().join("authorization.json");
        let authorization = json!({
            "capability_entry_fingerprint": capability["entry_fingerprint"],
            "operation": capability["operation"],
            "prepared_input_bytes_fingerprint": sha256_fingerprint(body),
            "request_schema": capability["request_schema"],
        });
        std::fs::write(
            &path,
            canonical_json_bytes(&authorization, &ContractLimits::packet0()).unwrap(),
        )
        .unwrap();
        args.extend([
            "--mutation-authorization-from".into(),
            path.to_string_lossy().into_owned(),
        ]);
    }
    run(root, &args)
}

#[test]
fn unchanged_generic_cli_discovers_all_four_rows_and_preserves_bounded_snapshot_truth() {
    let repo = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(repo.path());
    let limits = ContractLimits::packet0();
    let mut source = CompleteSnapshotSource::new();
    let start = operation_invocation(
        "snapshot.capture@1.0.0",
        capture_body("cli-packet-6", "start", "cli-seed-start-key"),
    );
    sdk.invoke_with_snapshot_source(&start, &limits, &mut source)
        .unwrap();
    source.advance_handoff("handoff/packet-6-cli");
    let end = operation_invocation(
        "snapshot.capture@1.0.0",
        capture_body("cli-packet-6", "end", "cli-seed-end-key"),
    );
    let finalized = sdk
        .invoke_with_snapshot_source(&end, &limits, &mut source)
        .unwrap();
    let finalized = serde_json::to_value(finalized).unwrap();
    let target = target_body(&finalized);

    let help =
        String::from_utf8(run(repo.path(), &strings(&["capability", "--help"])).stdout).unwrap();
    let actions = help
        .lines()
        .filter_map(|line| {
            let action = line.split_whitespace().next()?;
            ["bootstrap", "catalog-read", "schema-read", "invoke"]
                .contains(&action)
                .then_some(action)
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actions,
        BTreeSet::from(["bootstrap", "catalog-read", "schema-read", "invoke"])
    );

    let capabilities = capabilities(repo.path());
    for operation_ref in OPERATIONS {
        let capability = &capabilities[operation_ref];
        assert_eq!(capability["owner"]["owner_crate"], "handbook-engine");
        assert_eq!(
            capability["current_transports"],
            json!(["cli_json", "rust_sdk", "tauri"])
        );
        let schema = run(
            repo.path(),
            &strings(&[
                "capability",
                "schema-read",
                "--schema-ref",
                capability["request_schema"]["ref"].as_str().unwrap(),
                "--manifest-fingerprint",
                capability["request_schema"]["fingerprint"]
                    .as_str()
                    .unwrap(),
            ]),
        );
        assert!(schema.status.success());
        assert_eq!(
            sha256_fingerprint(&schema.stdout).as_str(),
            capability["request_schema"]["fingerprint"]
                .as_str()
                .unwrap()
        );
    }

    for operation_ref in [
        "snapshot.read@1.0.0",
        "snapshot.delta@1.0.0",
        "snapshot.project@1.0.0",
    ] {
        let body = canonical_json_bytes(&target, &limits).unwrap();
        let output = invoke(repo.path(), &capabilities[operation_ref], &body, false);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        let value = document(&output);
        assert_eq!(value["response_kind"], "operation");
        assert_eq!(value["response"]["status"], "ok");
        assert_eq!(value["response"]["data"]["outcome"], "found");
        if operation_ref == "snapshot.project@1.0.0" {
            assert_eq!(
                value["response"]["data"]["resolution_stack"]["ref"],
                "handbook.context-resolution-stack.packet-6@1.0.0"
            );
            assert_eq!(
                value["response"]["data"]["handoff_refs"],
                json!(["handoff/packet-6-cli"])
            );
        }
        let wire = String::from_utf8(output.stdout).unwrap();
        assert!(!wire.contains(repo.path().to_string_lossy().as_ref()));
        assert!(!wire.contains("handbook.snapshot-observation."));
    }

    let state_before = state_bytes(repo.path());
    let capture = canonical_json_bytes(
        &capture_body(
            "cli-packet-6-next",
            "start",
            "cli-capture-source-unavailable-key",
        ),
        &limits,
    )
    .unwrap();
    let unavailable = invoke(
        repo.path(),
        &capabilities["snapshot.capture@1.0.0"],
        &capture,
        true,
    );
    assert!(!unavailable.status.success());
    assert!(unavailable.stdout.is_empty());
    assert_eq!(
        unavailable.stderr,
        b"handbook capability: contract unavailable\n"
    );
    assert_eq!(state_bytes(repo.path()), state_before);
    assert!(!String::from_utf8(unavailable.stderr)
        .unwrap()
        .contains("cli-capture-source-unavailable-key"));

    let adapters = [
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/capability.rs")),
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/main.rs")),
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../tauri/src/lib.rs")),
    ];
    for source in adapters {
        for operation in OPERATIONS {
            assert!(!source.contains(operation));
        }
    }
}

fn state_bytes(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn visit(root: &Path, current: &Path, files: &mut BTreeMap<String, Vec<u8>>) {
        if !current.exists() {
            return;
        }
        for entry in std::fs::read_dir(current).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                visit(root, &path, files);
            } else {
                files.insert(
                    path.strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/"),
                    std::fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut files = BTreeMap::new();
    visit(
        root,
        &root.join(".handbook/state/snapshot-memory/v1"),
        &mut files,
    );
    files
}
