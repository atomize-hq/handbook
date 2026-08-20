use handbook_sdk::contract_membrane::{canonical_json_bytes, sha256_fingerprint, ContractLimits};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::{Command, Output};

const OPERATIONS: [(&str, &str, &str); 3] = [
    ("dock.manifest.list@1.0.0", "read_only", "safe"),
    ("dock.manifest.read@1.0.0", "read_only", "safe"),
    ("dock.run@1.0.0", "append_only", "idempotency_key_required"),
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

fn machine_document(output: &Output) -> Value {
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1,
        "{}",
        String::from_utf8_lossy(&output.stdout)
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
    let document = machine_document(&output);
    assert_eq!(document["response"]["root"]["total_entries"], 31);
    document["response"]["entries"]
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

fn schema_bytes(root: &Path, binding: &Value) -> Vec<u8> {
    let output = run(
        root,
        &strings(&[
            "capability",
            "schema-read",
            "--schema-ref",
            binding["ref"].as_str().unwrap(),
            "--manifest-fingerprint",
            binding["fingerprint"].as_str().unwrap(),
        ]),
    );
    assert!(output.status.success());
    assert_eq!(
        sha256_fingerprint(&output.stdout).as_str(),
        binding["fingerprint"].as_str().unwrap()
    );
    output.stdout
}

fn invoke(root: &Path, capability: &Value, body: &[u8], authorize_mutation: bool) -> Output {
    let invocation = tempfile::tempdir().unwrap();
    let body_path = invocation.path().join("request.json");
    std::fs::write(&body_path, body).unwrap();
    let mut args = vec![
        "capability".to_owned(),
        "invoke".to_owned(),
        "--operation-ref".to_owned(),
        capability["operation"]["ref"].as_str().unwrap().to_owned(),
        "--definition-fingerprint".to_owned(),
        capability["operation"]["fingerprint"]
            .as_str()
            .unwrap()
            .to_owned(),
        "--capability-entry-fingerprint".to_owned(),
        capability["entry_fingerprint"].as_str().unwrap().to_owned(),
        "--from-input".to_owned(),
        body_path.to_string_lossy().into_owned(),
    ];
    if authorize_mutation {
        let authorization_path = invocation.path().join("authorization.json");
        let authorization = json!({
            "capability_entry_fingerprint": capability["entry_fingerprint"],
            "operation": capability["operation"],
            "prepared_input_bytes_fingerprint": sha256_fingerprint(body),
            "request_schema": capability["request_schema"],
        });
        std::fs::write(
            &authorization_path,
            canonical_json_bytes(&authorization, &ContractLimits::packet0()).unwrap(),
        )
        .unwrap();
        args.extend([
            "--mutation-authorization-from".to_owned(),
            authorization_path.to_string_lossy().into_owned(),
        ]);
    }
    run(root, &args)
}

fn binding(reference: &str, hex: char) -> Value {
    json!({
        "ref": reference,
        "fingerprint": format!("sha256:{}", hex.to_string().repeat(64)),
    })
}

#[test]
fn generic_cli_discovers_and_invokes_the_three_dock_operations() {
    let repo = tempfile::tempdir().unwrap();
    let capabilities = capabilities(repo.path());
    let raw_key = "packet-five-cli-key-must-not-leak";
    let bodies = BTreeMap::from([
        (
            "dock.manifest.list@1.0.0",
            json!({"cursor": null, "limit": 10, "root": null}),
        ),
        (
            "dock.manifest.read@1.0.0",
            json!({"manifest": binding("handbook.dock.absent@1.0.0", 'a')}),
        ),
        (
            "dock.run@1.0.0",
            json!({
                "manifest": binding("handbook.dock.absent@1.0.0", 'a'),
                "evaluation": {
                    "contract": binding("handbook.contract.absent@1.0.0", 'b'),
                    "expected_lifecycle_fingerprint": format!("sha256:{}", "c".repeat(64)),
                    "subject": {},
                    "cases": [],
                    "requested_resolution": {},
                    "matcher": {},
                    "freshness_policies": [],
                    "stability_policies": [],
                    "gate_policy": binding("handbook.contract-gate-policy.absent@1.0.0", 'd'),
                    "local_closeout_policy": binding("handbook.contract-local-closeout-policy.absent@1.0.0", 'e'),
                    "nonce": "packet-five-cli-run"
                },
                "requested_evidence_kinds": [],
                "inputs": [],
                "limits": {
                    "request_bytes": 4096,
                    "result_bytes": 4096,
                    "stderr_bytes": 1024,
                    "diagnostic_count": 4,
                    "diagnostic_bytes": 1024,
                    "candidate_count": 4,
                    "candidate_bytes": 4096,
                    "input_count": 4,
                    "wall_timeout_ms": 1000,
                    "cancellation_grace_ms": 100
                },
                "idempotency_key": raw_key
            }),
        ),
    ]);

    for (operation_ref, class, idempotency) in OPERATIONS {
        let capability = capabilities.get(operation_ref).unwrap();
        assert_eq!(capability["class"], class);
        assert_eq!(capability["idempotency"]["kind"], idempotency);
        assert_eq!(capability["owner"]["owner_crate"], "handbook-dock");
        assert_eq!(
            capability["current_transports"],
            json!(["cli_json", "rust_sdk", "tauri"])
        );
        for schema_binding in [
            &capability["request_schema"],
            &capability["result_schema"],
            &capability["outcome_schemas"]["blocker"],
            &capability["outcome_schemas"]["refusal"],
            &capability["outcome_schemas"]["error"],
        ] {
            let schema: Value =
                serde_json::from_slice(&schema_bytes(repo.path(), schema_binding)).unwrap();
            jsonschema::draft202012::options().build(&schema).unwrap();
        }

        let body =
            canonical_json_bytes(&bodies[operation_ref], &ContractLimits::packet0()).unwrap();
        let output = invoke(repo.path(), capability, &body, class == "append_only");
        let document = machine_document(&output);
        assert_eq!(document["response_kind"], "operation");
        assert!(document["response"]["receipts"]
            .as_array()
            .unwrap()
            .is_empty());
        match operation_ref {
            "dock.manifest.list@1.0.0" => {
                assert!(output.status.success());
                assert_eq!(document["response"]["data"]["entries"], json!([]));
            }
            "dock.manifest.read@1.0.0" => {
                assert!(output.status.success());
                assert_eq!(document["response"]["data"]["outcome"], "absent");
            }
            "dock.run@1.0.0" => {
                assert_eq!(output.status.code(), Some(1));
                assert_eq!(document["response"]["status"], "refused");
                assert_eq!(
                    document["response"]["refusals"][0]["details"]["cause"],
                    "malformed"
                );
                assert_eq!(
                    document["response"]["idempotency"]["state"],
                    "not_established"
                );
            }
            _ => unreachable!(),
        }
        assert!(!output
            .stdout
            .windows(raw_key.len())
            .any(|window| window == raw_key.as_bytes()));
    }
}

#[test]
fn generic_consumers_have_no_dock_operation_or_semantic_branch() {
    let cli_adapter = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/capability.rs"));
    let cli_parser = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/main.rs"));
    let tauri_facade = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../tauri/src/lib.rs"));
    let installed_skill = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../install/handbook-home/SKILL.md.tmpl"
    ));

    for source in [cli_adapter, cli_parser, tauri_facade, installed_skill] {
        for (operation_ref, _, _) in OPERATIONS {
            assert!(!source.contains(operation_ref));
            assert!(!source.contains(operation_ref.trim_end_matches("@1.0.0")));
        }
        for semantic_branch in [
            "manifest_unavailable",
            "launch_refused",
            "process_failed",
            "cleanup_uncertain",
        ] {
            assert!(!source.contains(semantic_branch));
        }
    }
    assert_eq!(cli_adapter.matches("CapabilityCommand::").count(), 4);
    assert_eq!(
        cli_adapter
            .matches("let response = match consumer.invoke(")
            .count(),
        1
    );
    assert_eq!(tauri_facade.matches("#[tauri::command]").count(), 3);
    assert_eq!(installed_skill.matches("capability invoke").count(), 1);
}
