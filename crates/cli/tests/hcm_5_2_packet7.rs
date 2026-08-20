use handbook_sdk::contract_membrane::{canonical_json_bytes, sha256_fingerprint, ContractLimits};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::{Command, Output};

const OPERATIONS: [(&str, &str, &str); 5] = [
    ("contract.evidence.list@1.0.0", "read_only", "safe"),
    ("contract.evidence.read@1.0.0", "read_only", "safe"),
    (
        "contract.evidence.append@1.0.0",
        "append_only",
        "idempotency_key_required",
    ),
    ("contract.verdict.evaluate@1.0.0", "read_only", "safe"),
    ("contract.gate.evaluate@1.0.0", "read_only", "safe"),
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
        "machine stdout must contain exactly one JSON value plus LF: {}",
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
    assert_eq!(document["response"]["root"]["total_entries"], 28);
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
    assert!(output.stderr.is_empty());
    assert_eq!(
        sha256_fingerprint(&output.stdout).as_str(),
        binding["fingerprint"].as_str().unwrap()
    );
    output.stdout
}

fn invoke(root: &Path, capability: &Value, body: &[u8], authorize_mutation: bool) -> Output {
    let run_dir = tempfile::tempdir().unwrap();
    let body_path = run_dir.path().join("request.json");
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
        let authorization_path = run_dir.path().join("authorization.json");
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
fn generic_cli_discovers_schemas_and_invokes_all_five_evidence_operations() {
    let repo = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(
        repo.path()
            .join(".handbook/state/contracts/v1/candidate-observations"),
    )
    .unwrap();
    let capabilities = capabilities(repo.path());
    let raw_key = "packet-seven-cli-raw-key-must-not-leak";
    let run = json!({
        "binding": binding(
            "handbook.contract.packet-seven@1.0.0#evaluation/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            'a'
        ),
        "document": {}
    });
    let bodies = BTreeMap::from([
        (
            "contract.evidence.list@1.0.0",
            json!({"cursor": null, "limit": 10, "root": null}),
        ),
        (
            "contract.evidence.read@1.0.0",
            json!({"evidence": binding(
                "handbook.contract.packet-seven@1.0.0#evidence/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                'a'
            )}),
        ),
        (
            "contract.evidence.append@1.0.0",
            json!({
                "candidate": binding("handbook.evidence-candidate.packet-seven@1.0.0", 'b'),
                "idempotency_key": raw_key,
            }),
        ),
        (
            "contract.verdict.evaluate@1.0.0",
            json!({"evaluation_run": run.clone()}),
        ),
        (
            "contract.gate.evaluate@1.0.0",
            json!({"evaluation_run": run}),
        ),
    ]);

    for (operation_ref, class, idempotency_kind) in OPERATIONS {
        let capability = capabilities.get(operation_ref).unwrap();
        assert_eq!(capability["class"], class);
        assert_eq!(capability["idempotency"]["kind"], idempotency_kind);
        assert_eq!(capability["owner"]["owner_crate"], "handbook-contracts");
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
            jsonschema::draft202012::options()
                .build(&schema)
                .expect("manifest-bound schema compiles");
        }

        let body =
            canonical_json_bytes(&bodies[operation_ref], &ContractLimits::packet0()).unwrap();
        let output = invoke(repo.path(), capability, &body, class == "append_only");
        let document = machine_document(&output);
        assert_eq!(document["response_kind"], "operation");
        assert_eq!(document["response"]["operation"], capability["operation"]);
        assert!(document["response"]["receipts"]
            .as_array()
            .unwrap()
            .is_empty());
        match operation_ref {
            "contract.evidence.list@1.0.0" => {
                assert!(output.status.success());
                assert_eq!(document["response"]["status"], "ok");
                assert_eq!(document["response"]["data"]["entries"], json!([]));
            }
            "contract.evidence.read@1.0.0" => {
                assert!(output.status.success());
                assert_eq!(document["response"]["status"], "ok");
                assert_eq!(document["response"]["data"]["outcome"], "absent");
            }
            "contract.evidence.append@1.0.0" => {
                assert_eq!(output.status.code(), Some(1));
                assert_eq!(document["response"]["status"], "error");
                assert_eq!(
                    document["response"]["errors"][0]["details"]["cause"],
                    "candidate_source_unavailable"
                );
                assert_eq!(
                    document["response"]["idempotency"]["state"],
                    "not_established"
                );
            }
            "contract.verdict.evaluate@1.0.0" | "contract.gate.evaluate@1.0.0" => {
                assert_eq!(output.status.code(), Some(1));
                assert_eq!(document["response"]["status"], "error");
                assert_eq!(
                    document["response"]["errors"][0]["details"]["cause"],
                    "repository_unavailable"
                );
            }
            _ => unreachable!(),
        }
        assert!(!output
            .stdout
            .windows(raw_key.len())
            .any(|window| window == raw_key.as_bytes()));
        assert!(!String::from_utf8(output.stdout)
            .unwrap()
            .contains(repo.path().to_string_lossy().as_ref()));
    }
    let persisted = walk_bytes(repo.path());
    assert!(!persisted
        .windows(raw_key.len())
        .any(|window| window == raw_key.as_bytes()));
}

#[test]
fn generic_consumers_contain_no_hcm_5_2_operation_or_semantic_branch() {
    let cli_adapter = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/capability.rs"));
    let cli_parser = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/main.rs"));
    let tauri_facade = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../tauri/src/lib.rs"));
    let installed_skill = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../install/handbook-home/SKILL.md.tmpl"
    ));
    let installed_prompt = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../install/handbook-home/agents/openai.yaml"
    ));

    for source in [
        cli_adapter,
        cli_parser,
        tauri_facade,
        installed_skill,
        installed_prompt,
    ] {
        for (operation_ref, _, _) in OPERATIONS {
            assert!(!source.contains(operation_ref), "hardcoded {operation_ref}");
            assert!(!source.contains(operation_ref.trim_end_matches("@1.0.0")));
        }
        for semantic_branch in [
            "candidate_source_unavailable",
            "insufficient_resolution",
            "local-closeout.blocked",
            "parent_promotion",
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

fn walk_bytes(root: &Path) -> Vec<u8> {
    fn visit(path: &Path, output: &mut Vec<u8>) {
        if path.is_dir() {
            for entry in std::fs::read_dir(path).unwrap() {
                visit(&entry.unwrap().path(), output);
            }
        } else {
            output.extend(std::fs::read(path).unwrap());
        }
    }
    let mut output = Vec::new();
    visit(root, &mut output);
    output
}
