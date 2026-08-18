use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};

const INVOCATION_RESPONSE_SCHEMA_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../sdk/schemas/2020-12/handbook.invocation-response/1.0.0.schema.json"
));

#[derive(Clone, Debug)]
struct DiscoveredCapability {
    operation_ref: String,
    definition_fingerprint: String,
    entry_fingerprint: String,
    request_schema_ref: String,
    request_schema_fingerprint: String,
}

fn run_in(root: &Path, args: &[String], stdin: Option<&[u8]>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_handbook"));
    command.current_dir(root).args(args);
    if let Some(input) = stdin {
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.as_mut().unwrap().write_all(input).unwrap();
        child.wait_with_output().unwrap()
    } else {
        command.output().unwrap()
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn machine_document(output: &Output) -> serde_json::Value {
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1,
        "machine stdout must contain one value plus one LF: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(output.stdout.ends_with(b"\n"));
    assert!(
        output.stderr.is_empty(),
        "typed outcomes belong only on stdout: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("one JSON document")
}

fn discover(root: &Path, operation_ref: &str) -> DiscoveredCapability {
    let output = run_in(
        root,
        &strings(&[
            "capability",
            "catalog-read",
            "--catalog-kind",
            "capabilities",
            "--page-size",
            "128",
        ]),
        None,
    );
    assert!(
        output.status.success(),
        "invoke failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let document = machine_document(&output);
    let entry = document["response"]["entries"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["operation"]["ref"] == operation_ref)
        .unwrap_or_else(|| panic!("{operation_ref} was absent from live discovery"));
    DiscoveredCapability {
        operation_ref: operation_ref.to_owned(),
        definition_fingerprint: entry["operation"]["fingerprint"]
            .as_str()
            .unwrap()
            .to_owned(),
        entry_fingerprint: entry["entry_fingerprint"].as_str().unwrap().to_owned(),
        request_schema_ref: entry["request_schema"]["ref"].as_str().unwrap().to_owned(),
        request_schema_fingerprint: entry["request_schema"]["fingerprint"]
            .as_str()
            .unwrap()
            .to_owned(),
    }
}

fn invoke_args(
    capability: &DiscoveredCapability,
    from_input: &str,
    correlation: Option<&str>,
) -> Vec<String> {
    let mut args = strings(&[
        "capability",
        "invoke",
        "--operation-ref",
        &capability.operation_ref,
        "--definition-fingerprint",
        &capability.definition_fingerprint,
        "--capability-entry-fingerprint",
        &capability.entry_fingerprint,
        "--from-input",
        from_input,
    ]);
    if let Some(correlation) = correlation {
        args.extend(["--correlation".to_owned(), correlation.to_owned()]);
    }
    args
}

fn assert_adapter_failure(output: &Output, expected: &str) {
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, expected.as_bytes());
    assert_eq!(
        output.stderr.iter().filter(|byte| **byte == b'\n').count(),
        1
    );
}

fn seed_pipeline_catalog(root: &Path) {
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../core/pipelines");
    let destination = root.join("core/pipelines");
    std::fs::create_dir_all(&destination).unwrap();
    for entry in std::fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        std::fs::copy(entry.path(), destination.join(entry.file_name())).unwrap();
    }
}

fn file_snapshot(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn visit(root: &Path, current: &Path, snapshot: &mut BTreeMap<String, Vec<u8>>) {
        for entry in std::fs::read_dir(current).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, snapshot);
            } else {
                snapshot.insert(
                    path.strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/"),
                    std::fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut snapshot = BTreeMap::new();
    visit(root, root, &mut snapshot);
    snapshot
}

#[test]
fn live_discovery_schema_and_file_input_close_one_read_only_invocation() {
    let repo = tempfile::tempdir().unwrap();
    let root = repo.path();
    seed_pipeline_catalog(root);
    let capability = discover(root, "pipeline.catalog.list@1.0.0");

    let schema = run_in(
        root,
        &strings(&[
            "capability",
            "schema-read",
            "--schema-ref",
            &capability.request_schema_ref,
            "--manifest-fingerprint",
            &capability.request_schema_fingerprint,
        ]),
        None,
    );
    assert!(schema.status.success());
    assert!(schema.stderr.is_empty());
    let request_schema: serde_json::Value = serde_json::from_slice(&schema.stdout).unwrap();
    let request_validator = jsonschema::draft202012::options()
        .build(&request_schema)
        .unwrap();
    assert!(request_validator.is_valid(&serde_json::json!({})));

    let input_path = root.join("request.json");
    std::fs::write(&input_path, b"{}").unwrap();
    let files_before = file_snapshot(root);
    let output = run_in(
        root,
        &invoke_args(
            &capability,
            input_path.to_str().unwrap(),
            Some("packet3-read-only"),
        ),
        None,
    );
    assert!(
        output.status.success(),
        "invoke failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let response = machine_document(&output);
    assert_eq!(response["response_kind"], "operation");
    assert_eq!(
        response["response"]["operation"]["ref"],
        capability.operation_ref
    );
    assert_eq!(response["response"]["request_id"], "packet3-read-only");
    assert_eq!(response["response"]["status"], "ok");
    let response_schema: serde_json::Value =
        serde_json::from_slice(INVOCATION_RESPONSE_SCHEMA_BYTES).unwrap();
    let response_validator = jsonschema::draft202012::options()
        .build(&response_schema)
        .unwrap();
    assert!(response_validator.is_valid(&response));
    assert_eq!(
        file_snapshot(root),
        files_before,
        "read-only owner wrote state"
    );
}

#[test]
fn stdin_is_the_only_alternative_input_mode_and_duplicate_acquisition_is_rejected() {
    let repo = tempfile::tempdir().unwrap();
    seed_pipeline_catalog(repo.path());
    let capability = discover(repo.path(), "pipeline.catalog.list@1.0.0");
    let output = run_in(
        repo.path(),
        &invoke_args(&capability, "-", None),
        Some(b"{}"),
    );
    assert!(
        output.status.success(),
        "stdin invoke failed: stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(machine_document(&output)["response"]["status"], "ok");

    let mut duplicate = invoke_args(&capability, "-", None);
    duplicate.extend(["--from-input".to_owned(), "second.json".to_owned()]);
    let duplicate = run_in(repo.path(), &duplicate, Some(b"{}"));
    assert_eq!(duplicate.status.code(), Some(2));
    assert!(duplicate.stdout.is_empty());
    assert!(!duplicate.stderr.is_empty());
    assert!(duplicate.stderr.len() < 2_048);
}

#[test]
fn typed_owner_non_success_remains_typed_stdout_and_drives_failure_exit() {
    let repo = tempfile::tempdir().unwrap();
    let capability = discover(repo.path(), "artifact.read@1.0.0");
    let body = br#"{"instance_id":"registry_brief","kind_ref":"example.artifact-kind.registry-brief@1.0.0"}"#;
    let input_path = repo.path().join("read.json");
    std::fs::write(&input_path, body).unwrap();
    let output = run_in(
        repo.path(),
        &invoke_args(&capability, input_path.to_str().unwrap(), None),
        None,
    );
    assert_eq!(output.status.code(), Some(1));
    let response = machine_document(&output);
    assert_eq!(response["response_kind"], "operation");
    assert!(matches!(
        response["response"]["status"].as_str(),
        Some("blocked" | "refused" | "error")
    ));
}

#[test]
fn identity_transport_class_and_command_substitution_fail_before_owner_execution() {
    let repo = tempfile::tempdir().unwrap();
    let root = repo.path();
    let read_only = discover(root, "artifact.kind.list@1.0.0");
    let mutation = discover(root, "intake.record.append@1.0.0");
    let posture = discover(root, "posture.transition.apply@1.0.0");
    let input_path = root.join("request.json");
    std::fs::write(&input_path, b"{}").unwrap();
    let input = input_path.to_str().unwrap();
    let files_before = file_snapshot(root);

    let stale = format!("sha256:{}", "0".repeat(64));
    let mut stale_definition = read_only.clone();
    stale_definition.definition_fingerprint = stale.clone();
    let mut stale_entry = read_only.clone();
    stale_entry.entry_fingerprint = stale.clone();

    for (capability, expected) in [
        (
            DiscoveredCapability {
                operation_ref: "handbook.use-case.repository-doctor@1.0.0".to_owned(),
                ..read_only.clone()
            },
            "handbook capability: operation unavailable\n",
        ),
        (
            DiscoveredCapability {
                operation_ref: "example.unknown@1.0.0".to_owned(),
                ..read_only.clone()
            },
            "handbook capability: operation unavailable\n",
        ),
        (
            stale_definition,
            "handbook capability: operation selection stale\n",
        ),
        (
            stale_entry,
            "handbook capability: operation selection stale\n",
        ),
        (
            mutation,
            "handbook capability: mutation authorization required\n",
        ),
        (posture, "handbook capability: transport unavailable\n"),
    ] {
        assert_adapter_failure(
            &run_in(root, &invoke_args(&capability, input, None), None),
            expected,
        );
    }

    let mut command_name = invoke_args(&read_only, input, None);
    let position = command_name
        .iter()
        .position(|value| value == &read_only.operation_ref)
        .unwrap();
    command_name[position] = "artifact list-kinds".to_owned();
    assert_adapter_failure(
        &run_in(root, &command_name, None),
        "handbook capability: operation binding rejected\n",
    );
    assert_eq!(
        file_snapshot(root),
        files_before,
        "rejected invocation wrote state"
    );
}

#[test]
fn malformed_noncanonical_duplicate_and_unknown_body_fail_without_echoing_input() {
    let repo = tempfile::tempdir().unwrap();
    let capability = discover(repo.path(), "artifact.kind.list@1.0.0");
    for (name, body) in [
        ("malformed", b"{".as_slice()),
        ("noncanonical", br#"{"z":0,"a":0}"#.as_slice()),
        ("duplicate", br#"{"a":0,"a":1}"#.as_slice()),
        (
            "unknown",
            br#"{"private_secret":"must-not-echo"}"#.as_slice(),
        ),
    ] {
        let input_path = repo.path().join(format!("{name}.json"));
        std::fs::write(&input_path, body).unwrap();
        let output = run_in(
            repo.path(),
            &invoke_args(&capability, input_path.to_str().unwrap(), None),
            None,
        );
        assert_adapter_failure(&output, "handbook capability: input rejected\n");
        assert!(!String::from_utf8_lossy(&output.stderr).contains("must-not-echo"));
    }
}

#[test]
fn unavailable_input_and_invalid_correlation_are_bounded_adapter_failures() {
    let repo = tempfile::tempdir().unwrap();
    let capability = discover(repo.path(), "artifact.kind.list@1.0.0");
    assert_adapter_failure(
        &run_in(
            repo.path(),
            &invoke_args(&capability, "missing.json", None),
            None,
        ),
        "handbook capability: input unavailable\n",
    );
    let input_path = repo.path().join("request.json");
    std::fs::write(&input_path, b"{}").unwrap();
    assert_adapter_failure(
        &run_in(
            repo.path(),
            &invoke_args(&capability, input_path.to_str().unwrap(), Some("not valid")),
            None,
        ),
        "handbook capability: correlation rejected\n",
    );
}
