use handbook_sdk::contract_membrane::{
    operation_inventory, sha256_fingerprint, AdmissionReason, DiscoveryState, OperationTransport,
};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const FIXED_USE_CASES: [&str; 6] = [
    "handbook.use-case.repository-setup@1.0.0",
    "handbook.use-case.repository-doctor@1.0.0",
    "handbook.use-case.author-charter@1.0.0",
    "handbook.use-case.author-project-context@1.0.0",
    "handbook.use-case.approver-administration@1.0.0",
    "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
];

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repository root")
}

fn run_handbook(root: &Path, args: &[String]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
        .current_dir(root)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("run `{}`: {error}", args.join(" ")))
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn machine_document(output: &Output) -> serde_json::Value {
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1,
        "machine stdout must contain one value plus LF: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout).expect("one machine document")
}

fn complete_catalog(root: &Path, kind: &str, page_size: usize) -> Vec<serde_json::Value> {
    let mut entries = Vec::new();
    let mut cursor = serde_json::Value::Null;
    let mut expected_start = 0usize;
    loop {
        let mut args = strings(&[
            "capability",
            "catalog-read",
            "--catalog-kind",
            kind,
            "--page-size",
            &page_size.to_string(),
        ]);
        if !cursor.is_null() {
            args.extend([
                "--cursor".to_owned(),
                serde_json::to_string(&cursor).unwrap(),
            ]);
        }
        let output = run_handbook(root, &args);
        assert!(
            output.status.success(),
            "{kind} page failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let document = machine_document(&output);
        assert_eq!(document["catalog_kind"], kind);
        assert_eq!(document["outcome_kind"], "page");
        let page = &document["response"];
        assert_eq!(page["start_index"], expected_start);
        let page_entries = page["entries"].as_array().unwrap();
        assert!(!page_entries.is_empty());
        entries.extend(page_entries.iter().cloned());
        expected_start = entries.len();
        cursor = page["next_cursor"].clone();
        if cursor.is_null() {
            assert_eq!(page["root"]["total_entries"], entries.len());
            break;
        }
        assert_eq!(cursor["next_index"], expected_start);
        assert_eq!(cursor["page_size"], page_size);
    }
    entries
}

fn schema_bytes(root: &Path, binding: &serde_json::Value) -> Vec<u8> {
    let output = run_handbook(
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

fn has_transport(entry: &serde_json::Value, field: &str, transport: &str) -> bool {
    entry[field]
        .as_array()
        .is_some_and(|values| values.iter().any(|value| value == transport))
}

#[test]
fn four_generic_actions_page_the_complete_unchanged_contract_surface() {
    let root = repository_root();
    let help = run_handbook(&root, &strings(&["capability", "--help"]));
    assert!(help.status.success());
    assert!(help.stderr.is_empty());
    let help = String::from_utf8(help.stdout).unwrap();
    let actions = help
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            trimmed
                .split_once(char::is_whitespace)
                .map(|(action, _)| action)
                .filter(|action| {
                    ["bootstrap", "catalog-read", "schema-read", "invoke"].contains(action)
                })
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actions,
        BTreeSet::from(["bootstrap", "catalog-read", "schema-read", "invoke"])
    );

    let bootstrap = run_handbook(
        &root,
        &strings(&[
            "capability",
            "bootstrap",
            "--client-version",
            "1.0.0",
            "--page-size",
            "1",
        ]),
    );
    assert!(bootstrap.status.success());
    let bootstrap = machine_document(&bootstrap);
    assert_eq!(bootstrap["outcome_kind"], "ok");
    assert_eq!(
        bootstrap["response"]["first_capability_page"]["page_size"],
        1
    );
    let manifest = bootstrap["response"]["schema_manifest"].as_array().unwrap();
    assert_eq!(manifest.len(), 120);

    let capabilities = complete_catalog(&root, "capabilities", 2);
    let schemas = complete_catalog(&root, "schemas", 17);
    assert_eq!(capabilities.len(), 19);
    assert_eq!(schemas.len(), 120);
    let manifest_bindings = manifest
        .iter()
        .map(|entry| {
            (
                entry["schema"]["ref"].as_str().unwrap(),
                entry["schema"]["fingerprint"].as_str().unwrap(),
            )
        })
        .collect::<BTreeSet<_>>();
    let schema_bindings = schemas
        .iter()
        .map(|entry| {
            (
                entry["schema"]["ref"].as_str().unwrap(),
                entry["schema"]["fingerprint"].as_str().unwrap(),
            )
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(schema_bindings, manifest_bindings);

    let rows = operation_inventory();
    assert_eq!(rows.len(), 62);
    assert_eq!(
        rows.iter()
            .filter(|row| row.admission_reason() == AdmissionReason::Admitted)
            .count(),
        19
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.admission_reason() != AdmissionReason::Admitted)
            .count(),
        43
    );
    assert!(rows
        .iter()
        .filter(|row| row.admission_reason() != AdmissionReason::Admitted)
        .all(|row| row.discovery() == DiscoveryState::Omit && row.current_transports().is_empty()));

    let vectors = capabilities
        .iter()
        .map(|entry| {
            (
                entry["operation"]["ref"].as_str().unwrap(),
                entry["current_transports"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_str().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        vectors
            .values()
            .filter(|transports| transports.as_slice() == ["cli_json", "tauri"])
            .count(),
        18
    );
    assert_eq!(
        vectors
            .values()
            .filter(|transports| transports.as_slice() == ["rust_sdk", "tauri"])
            .count(),
        1
    );
    assert!(FIXED_USE_CASES
        .iter()
        .all(|reference| !vectors.contains_key(reference)));
    assert_eq!(
        rows.iter()
            .filter(|row| {
                row.current_transports() == [OperationTransport::CliJson, OperationTransport::Tauri]
            })
            .count(),
        18
    );
}

#[test]
fn dynamically_selected_capability_needs_no_specific_skill_or_cli_route() {
    let root = repository_root();
    let capabilities = complete_catalog(&root, "capabilities", 3);
    let selected = capabilities
        .iter()
        .filter(|entry| {
            entry["class"] == "read_only"
                && has_transport(entry, "declared_transports", "cli_json")
                && has_transport(entry, "current_transports", "cli_json")
        })
        .find(|entry| {
            let bytes = schema_bytes(&root, &entry["request_schema"]);
            serde_json::from_slice::<serde_json::Value>(&bytes)
                .ok()
                .and_then(|schema| jsonschema::draft202012::options().build(&schema).ok())
                .is_some_and(|validator| validator.is_valid(&serde_json::json!({})))
        })
        .expect("one discovered read-only cli_json capability must admit an empty body");

    for binding in [
        &selected["request_schema"],
        &selected["result_schema"],
        &selected["outcome_schemas"]["blocker"],
        &selected["outcome_schemas"]["refusal"],
        &selected["outcome_schemas"]["error"],
    ] {
        let bytes = schema_bytes(&root, binding);
        let schema: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        jsonschema::draft202012::options()
            .build(&schema)
            .expect("exact manifest-bound schema");
    }

    let selected_ref = selected["operation"]["ref"].as_str().unwrap();
    let generic_adapter = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/capability.rs"));
    let generic_parser = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/main.rs"));
    let root_skill = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../install/handbook-home/SKILL.md.tmpl"
    ));
    for source in [generic_adapter, generic_parser, root_skill] {
        assert!(!source.contains(selected_ref));
        for entry in &capabilities {
            assert!(!source.contains(entry["operation"]["ref"].as_str().unwrap()));
        }
        for use_case in FIXED_USE_CASES {
            assert!(!source.contains(use_case));
        }
    }
    assert_eq!(generic_adapter.matches("CapabilityCommand::").count(), 4);
    assert_eq!(
        generic_adapter
            .matches("let response = match consumer.invoke(")
            .count(),
        1
    );

    let mut request = tempfile::NamedTempFile::new().unwrap();
    request.write_all(b"{}").unwrap();
    request.flush().unwrap();
    let invocation = run_handbook(
        &root,
        &strings(&[
            "capability",
            "invoke",
            "--operation-ref",
            selected_ref,
            "--definition-fingerprint",
            selected["operation"]["fingerprint"].as_str().unwrap(),
            "--capability-entry-fingerprint",
            selected["entry_fingerprint"].as_str().unwrap(),
            "--from-input",
            request.path().to_str().unwrap(),
        ]),
    );
    assert!(invocation.stderr.is_empty());
    let invocation = machine_document(&invocation);
    assert_eq!(invocation["response_kind"], "operation");
    assert_eq!(invocation["response"]["operation"], selected["operation"]);
    assert!(matches!(
        invocation["response"]["status"].as_str(),
        Some("ok" | "blocked" | "refused" | "error")
    ));
    for field in [
        "data",
        "blockers",
        "refusals",
        "errors",
        "diagnostics",
        "next_actions",
        "idempotency",
        "receipts",
        "result_fingerprint",
        "response_fingerprint",
    ] {
        assert!(
            invocation["response"].get(field).is_some(),
            "missing {field}"
        );
    }
}
