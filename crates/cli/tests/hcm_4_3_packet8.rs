#[path = "../src/coverage.rs"]
mod coverage;
#[allow(dead_code)]
#[path = "pipeline_proof_corpus_support.rs"]
mod pipeline_proof_corpus_support;

use coverage::{ResultClass, TerminalSource, TypedExecutionSource};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn handbook() -> Command {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
}

fn run_in(root: &Path, args: &[&str]) -> Output {
    handbook()
        .current_dir(root)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("run `{}`: {error}", args.join(" ")))
}

fn invocation_document(output: &Output) -> serde_json::Value {
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1,
        "machine stdout must contain one document plus one LF: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(output.stdout.ends_with(b"\n"));
    assert!(
        output.stderr.is_empty(),
        "semantic output must not be duplicated on stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let document: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("one JSON document");
    let schema: serde_json::Value = serde_json::from_slice(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../sdk/schemas/2020-12/handbook.invocation-response/1.0.0.schema.json"
    )))
    .expect("checked-in invocation schema");
    let validator = jsonschema::draft202012::options()
        .build(&schema)
        .expect("self-contained Draft 2020-12 invocation schema");
    if !validator.is_valid(&document) {
        let errors = validator
            .iter_errors(&document)
            .map(|error| error.to_string())
            .collect::<Vec<_>>();
        panic!("invocation must validate against the checked-in root: {errors:#?}");
    }
    document
}

fn assert_host_path_absent(output: &Output, root: &Path) {
    assert_output_omits(output, root.to_str().unwrap());
}

fn assert_output_omits(output: &Output, forbidden: &str) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains(forbidden),
        "stdout leaked private value: {stdout}"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains(forbidden),
        "stderr leaked private value: {stderr}"
    );
    if let Ok(document) = serde_json::from_slice::<serde_json::Value>(&output.stdout) {
        assert_json_strings_omit(&document, forbidden);
    }
}

#[cfg(windows)]
#[test]
fn windows_rooted_author_paths_are_reduced_before_any_renderer() {
    let root = tempfile::tempdir().unwrap();
    let rooted_path = r"\Users\packet8-private\missing-charter.yaml";
    for selector in [None, Some("--json"), Some("--legacy-json")] {
        let mut args = vec![
            "author",
            "charter",
            "--mode",
            "guided-adaptive",
            "--from-inputs",
            rooted_path,
        ];
        if let Some(selector) = selector {
            args.push(selector);
        }
        let output = run_in(root.path(), &args);
        assert_eq!(output.status.code(), Some(1));
        assert_output_omits(&output, rooted_path);
    }
}

fn assert_json_strings_omit(value: &serde_json::Value, forbidden: &str) {
    match value {
        serde_json::Value::Array(values) => {
            for value in values {
                assert_json_strings_omit(value, forbidden);
            }
        }
        serde_json::Value::Object(fields) => {
            for value in fields.values() {
                assert_json_strings_omit(value, forbidden);
            }
        }
        serde_json::Value::String(value) => {
            assert!(
                !value.contains(forbidden),
                "JSON string leaked host path: {value}"
            );
        }
        _ => {}
    }
}

#[test]
fn every_authorized_leaf_has_one_exact_result_owner_and_terminal_source() {
    assert_eq!(coverage::validate(), Ok(()));

    let rows = coverage::rows();
    assert_eq!(rows.len(), 36);
    assert_eq!(
        rows.iter()
            .map(|row| row.leaf)
            .collect::<BTreeSet<_>>()
            .len(),
        rows.len(),
        "every leaf/mode must occur exactly once"
    );

    let source_counts = rows.iter().fold(BTreeMap::new(), |mut counts, row| {
        *counts.entry(row.typed_source).or_insert(0usize) += 1;
        counts
    });
    assert_eq!(source_counts[&TypedExecutionSource::EngineOwner], 10);
    assert_eq!(source_counts[&TypedExecutionSource::FlowOwner], 2);
    assert_eq!(source_counts[&TypedExecutionSource::PipelineOwner], 9);
    assert_eq!(source_counts[&TypedExecutionSource::SdkUseCase], 15);

    let terminal_counts = rows.iter().fold(BTreeMap::new(), |mut counts, row| {
        *counts.entry(row.terminal_source).or_insert(0usize) += 1;
        counts
    });
    assert_eq!(terminal_counts[&TerminalSource::OperationResponse], 21);
    assert_eq!(terminal_counts[&TerminalSource::UseCaseResponse], 15);

    for row in rows {
        match row.result_class {
            ResultClass::Operation(operation) => {
                assert_ne!(row.typed_source, TypedExecutionSource::SdkUseCase);
                assert_eq!(row.terminal_source, TerminalSource::OperationResponse);
                assert!(operation.ends_with("@1.0.0"));
            }
            ResultClass::UseCase(use_case) => {
                assert_eq!(row.typed_source, TypedExecutionSource::SdkUseCase);
                assert_eq!(row.terminal_source, TerminalSource::UseCaseResponse);
                assert!(use_case.starts_with("handbook.use-case."));
                assert!(use_case.ends_with("@1.0.0"));
            }
        }
    }
}

#[test]
fn every_command_family_uses_one_schema_valid_machine_document_and_shared_exit_policy() {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let setup = tempfile::tempdir().unwrap();
    fs::create_dir(setup.path().join(".git")).unwrap();
    let author = tempfile::tempdir().unwrap();
    let approver = tempfile::tempdir().unwrap();
    let artifact = tempfile::tempdir().unwrap();
    let pipeline_error = tempfile::tempdir().unwrap();
    let flow = tempfile::tempdir().unwrap();
    let doctor = tempfile::tempdir().unwrap();
    let transport = tempfile::tempdir().unwrap();
    fs::write(
        transport.path().join("context.yaml"),
        "schema_id: [broken\n",
    )
    .unwrap();

    let cases = [
        (
            run_in(setup.path(), &["setup", "--json"]),
            "use_case",
            "blocked",
        ),
        (
            run_in(
                author.path(),
                &["author", "charter", "--validate", "--json"],
            ),
            "use_case",
            "blocked",
        ),
        (
            run_in(
                approver.path(),
                &[
                    "approvers",
                    "bootstrap",
                    "--initial-charter-quorum",
                    "not-a-mapping",
                    "--json",
                ],
            ),
            "use_case",
            "refused",
        ),
        (
            run_in(
                artifact.path(),
                &[
                    "--json",
                    "artifact",
                    "list-kinds",
                    "--repository-root",
                    artifact.path().to_str().unwrap(),
                ],
            ),
            "operation",
            "blocked",
        ),
        (
            run_in(&workspace, &["--json", "pipeline", "list"]),
            "operation",
            "ok",
        ),
        (
            run_in(pipeline_error.path(), &["--json", "pipeline", "list"]),
            "operation",
            "error",
        ),
        (
            run_in(
                flow.path(),
                &["generate", "--json", "--packet", "planning.packet"],
            ),
            "operation",
            "blocked",
        ),
        (
            run_in(doctor.path(), &["doctor", "--json"]),
            "use_case",
            "blocked",
        ),
        (
            run_in(
                transport.path(),
                &[
                    "author",
                    "project-context",
                    "--from-inputs",
                    "context.yaml",
                    "--validate",
                    "--json",
                ],
            ),
            "transport_failure",
            "refused",
        ),
    ];

    let mut observed_statuses = BTreeSet::new();
    let mut observed_kinds = BTreeSet::new();
    for (output, expected_kind, expected_status) in cases {
        assert_eq!(
            output.status.code(),
            Some(if expected_status == "ok" { 0 } else { 1 })
        );
        let document = invocation_document(&output);
        assert_eq!(document["response_kind"], expected_kind);
        assert_eq!(document["response"]["status"], expected_status);
        observed_kinds.insert(expected_kind);
        observed_statuses.insert(expected_status);

        let wire = String::from_utf8(output.stdout).unwrap();
        for forbidden in [
            "raw_idempotency_key",
            "private_journal",
            ".handbook/state/pipeline/idempotency/",
            "debug_value",
        ] {
            assert!(!wire.contains(forbidden), "leaked `{forbidden}`: {wire}");
        }
    }
    assert_eq!(
        observed_kinds,
        BTreeSet::from(["operation", "transport_failure", "use_case"])
    );
    assert_eq!(
        observed_statuses,
        BTreeSet::from(["blocked", "error", "ok", "refused"])
    );

    let help = run_in(&workspace, &["--help"]);
    assert_eq!(help.status.code(), Some(0));
    let version = run_in(&workspace, &["--version"]);
    assert_eq!(version.status.code(), Some(0));
    let syntax = run_in(&workspace, &["not-a-command"]);
    assert_eq!(syntax.status.code(), Some(2));
}

#[test]
fn absolute_author_input_paths_never_cross_machine_human_or_legacy_output() {
    let project_context = tempfile::tempdir().unwrap();
    let context_path = project_context.path().join("private-context.yaml");
    let context_path = context_path.to_str().unwrap();

    for extra in [Vec::new(), vec!["--json"]] {
        let mut args = vec![
            "author",
            "project-context",
            "--from-inputs",
            context_path,
            "--validate",
        ];
        args.extend(extra);
        let output = run_in(project_context.path(), &args);
        assert_eq!(output.status.code(), Some(1));
        assert_host_path_absent(&output, project_context.path());
    }

    let charter = tempfile::tempdir().unwrap();
    let charter_path = charter.path().join("private-charter.yaml");
    let charter_path = charter_path.to_str().unwrap();
    for selector in [None, Some("--json"), Some("--legacy-json")] {
        let mut args = vec![
            "author",
            "charter",
            "--mode",
            "guided-adaptive",
            "--from-inputs",
            charter_path,
        ];
        if let Some(selector) = selector {
            args.push(selector);
        }
        let output = run_in(charter.path(), &args);
        assert_eq!(output.status.code(), Some(1));
        assert_host_path_absent(&output, charter.path());
    }
}

#[test]
fn pipeline_state_human_output_uses_a_portable_repository_root_label() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let output = run_in(
        &root,
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "needs_project_context=true",
        ],
    );
    assert_eq!(output.status.code(), Some(0));
    assert_host_path_absent(&output, &root);
    assert!(String::from_utf8(output.stdout)
        .unwrap()
        .contains("repo_root = ${repo_root}"));
}

#[test]
fn flow_human_acquisition_failures_omit_the_absolute_fixture_path() {
    let root = tempfile::tempdir().unwrap();
    for command in ["generate", "inspect"] {
        let output = run_in(
            root.path(),
            &[
                command,
                "--packet",
                "execution.demo.packet",
                "--fixture-set",
                "definitely_missing_packet8",
            ],
        );
        assert_eq!(output.status.code(), Some(1));
        assert_host_path_absent(&output, root.path());
        assert!(String::from_utf8(output.stdout)
            .unwrap()
            .contains("definitely_missing_packet8"));
    }
}

#[test]
fn pipeline_state_refusal_does_not_echo_an_absolute_reference() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let absolute_ref = root.join("private-charter.md");
    let field = format!("refs.charter_ref={}", absolute_ref.display());
    let output = run_in(
        &root,
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            &field,
        ],
    );
    assert_eq!(output.status.code(), Some(1));
    assert_host_path_absent(&output, &root);
}
