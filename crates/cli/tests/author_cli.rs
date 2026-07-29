use std::fs;
use std::io::Write;
#[cfg(unix)]
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::path::Path;
use std::process::{Command, Output, Stdio};
#[cfg(unix)]
const AUTHOR_PROJECT_CONTEXT_NOW_UTC_ENV_VAR: &str = "HANDBOOK_AUTHOR_PROJECT_CONTEXT_NOW_UTC";

const CANONICAL_CHARTER_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));

const CHARTER_COVERAGE_IDS: [&str; 16] = [
    "project_shape.definition",
    "delivery.constraints",
    "delivery.default_implications",
    "operational_reality.production_state",
    "risk.domains",
    "engineering_posture.baseline",
    "policy.authority_and_revision",
    "governance.decision_authority",
    "governance.required_approvals",
    "governance.exception_policy",
    "engineering_posture.dimensions",
    "engineering_posture.red_lines",
    "governance.review_triggers",
    "governance.reassessment_triggers",
    "debt.register",
    "decisions.records",
];

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
}

fn binary_in(dir: &Path) -> Command {
    let mut cmd = binary();
    cmd.current_dir(dir);
    cmd
}

fn run_in(dir: &Path, args: &[&str]) -> Output {
    binary_in(dir)
        .args(args)
        .output()
        .unwrap_or_else(|err| panic!("run `{}`: {err}", args.join(" ")))
}

fn run_in_with_input(dir: &Path, args: &[&str], input: &str) -> Output {
    let mut cmd = binary_in(dir);
    cmd.args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd
        .spawn()
        .unwrap_or_else(|err| panic!("spawn `{}`: {err}", args.join(" ")));

    {
        let stdin = child.stdin.as_mut().expect("stdin");
        stdin
            .write_all(input.as_bytes())
            .unwrap_or_else(|err| panic!("write stdin for `{}`: {err}", args.join(" ")));
    }

    child
        .wait_with_output()
        .unwrap_or_else(|err| panic!("wait `{}`: {err}", args.join(" ")))
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout utf-8")
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("mkdirs");
    }
    fs::write(path, contents).expect("write");
}

#[cfg(unix)]
fn with_project_context_now_utc<T>(value: &str, action: impl FnOnce() -> T) -> T {
    let previous = std::env::var_os(AUTHOR_PROJECT_CONTEXT_NOW_UTC_ENV_VAR);
    std::env::set_var(AUTHOR_PROJECT_CONTEXT_NOW_UTC_ENV_VAR, value);

    let result = catch_unwind(AssertUnwindSafe(action));

    match previous {
        Some(value) => std::env::set_var(AUTHOR_PROJECT_CONTEXT_NOW_UTC_ENV_VAR, value),
        None => std::env::remove_var(AUTHOR_PROJECT_CONTEXT_NOW_UTC_ENV_VAR),
    }

    match result {
        Ok(value) => value,
        Err(payload) => resume_unwind(payload),
    }
}

fn legacy_authoring_fixture_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    for descriptor in handbook_engine::canonical_artifact_descriptors()
        .iter()
        .filter(|descriptor| descriptor.setup_scaffolded)
    {
        let path = dir.path().join(descriptor.relative_path);
        fs::create_dir_all(path.parent().expect("legacy artifact parent")).expect("mkdirs");
        fs::write(
            path,
            handbook_engine::setup_starter_template_bytes(descriptor.kind),
        )
        .expect("write legacy authoring fixture");
    }
    dir
}

fn valid_project_context_inputs_yaml() -> &'static str {
    r#"schema_id: "handbook.artifact.project-context"
schema_version: "1.0"
record_id: "handbook.project-context"
summary: "CLI and compiler for canonical planning artifacts and workflow proofs."
system_boundaries:
  - "Compiler and CLI crates in this repository"
ownership:
  - "compiler-team"
authoritative_references:
  - "handbook.charter@1.0.0"
known_unknowns:
  - "Future external integration requirements"
"#
}

#[cfg(unix)]
fn write_valid_selected_project_context(repo_root: &Path) {
    write_file(
        &repo_root.join(".handbook/project/context.yaml"),
        valid_project_context_inputs_yaml(),
    );
}

#[cfg(unix)]
fn expected_project_context_markdown_from_yaml() -> String {
    let input =
        handbook_compiler::parse_project_context_input_yaml(valid_project_context_inputs_yaml())
            .expect("parse project-context yaml");
    String::from_utf8(
        handbook_engine::serialize_canonical_project_context(
            &handbook_engine::resolve_shipped_profile_decisions(".")
                .expect("selected Project Context decisions"),
            &input,
        )
        .expect("serialize canonical Project Context"),
    )
    .expect("canonical Project Context UTF-8")
}

fn valid_structured_inputs_yaml() -> String {
    let mut document = String::from("mode: guided_adaptive\ncontent:\n");
    for line in CANONICAL_CHARTER_YAML.lines() {
        document.push_str("  ");
        document.push_str(line);
        document.push('\n');
    }
    document.push_str("coverage:\n");
    for coverage_id in CHARTER_COVERAGE_IDS {
        document.push_str(&format!(
            "  - coverage_id: {coverage_id:?}\n    source_kind: user_declaration\n    value_ref: \"input://{coverage_id}\"\n    evidence_refs: []\n    confidence: high\n    freshness: null\n    sensitivity: public\n    contradiction_refs: []\n    waiver_ref: null\n"
        ));
    }
    document.push_str(
        "consumer:\n  kind: agent\n  id: handbook-charter-intake\n  version: \"1.0\"\nprompt_event_refs: []\nfinalized_at_utc: \"2026-07-20T00:00:00Z\"\nexpected_current_fingerprint: null\n",
    );
    document
}

#[test]
fn bare_charter_author_requires_structured_inputs() {
    let dir = legacy_authoring_fixture_repo();

    let output = run_in(dir.path(), &["author", "charter"]);

    assert!(!output.status.success(), "non-tty author should refuse");
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"));
    assert!(out.contains("OPERATION: author"));
    assert!(out.contains("CODE: invalid_request"));
    assert!(out.contains("arguments do not select exactly one frozen Charter operation"));
}

#[test]
fn file_inputs_refuse_when_yaml_is_malformed() {
    let dir = legacy_authoring_fixture_repo();
    let inputs_path = dir.path().join("charter-inputs.yaml");
    write_file(&inputs_path, "project: [not valid");

    let output = run_in(
        dir.path(),
        &[
            "author",
            "charter",
            "--mode",
            "guided-adaptive",
            "--from-inputs",
            inputs_path.to_str().expect("utf-8 path"),
        ],
    );

    assert!(
        !output.status.success(),
        "malformed yaml should refuse: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"));
    assert!(out.contains("OPERATION: author"));
    assert!(out.contains("CODE: invalid_intake_envelope"));
}

#[test]
fn stdin_inputs_refuse_when_yaml_is_malformed() {
    let dir = legacy_authoring_fixture_repo();

    let output = run_in_with_input(
        dir.path(),
        &[
            "author",
            "charter",
            "--mode",
            "guided-adaptive",
            "--from-inputs",
            "-",
        ],
        "schema_version: [broken\n",
    );

    assert!(
        !output.status.success(),
        "malformed stdin yaml should refuse: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"));
    assert!(out.contains("OPERATION: author"));
    assert!(out.contains("CODE: invalid_intake_envelope"));
}

#[test]
fn file_inputs_preserve_malformed_yaml_refusal_even_when_truth_exists() {
    let dir = legacy_authoring_fixture_repo();
    write_file(
        &dir.path().join(".handbook/project/charter.yaml"),
        CANONICAL_CHARTER_YAML,
    );
    let inputs_path = dir.path().join("charter-inputs.yaml");
    write_file(&inputs_path, "project: [not valid");

    let output = run_in(
        dir.path(),
        &[
            "author",
            "charter",
            "--mode",
            "guided-adaptive",
            "--from-inputs",
            inputs_path.to_str().expect("utf-8 path"),
        ],
    );

    assert!(
        !output.status.success(),
        "malformed yaml should still refuse: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"));
    assert!(out.contains("OPERATION: author"));
    assert!(out.contains("CODE: invalid_intake_envelope"));
}

#[test]
fn file_inputs_author_charter_successfully_with_deterministic_rendering() {
    let dir = legacy_authoring_fixture_repo();
    let inputs_path = dir.path().join("charter-inputs.yaml");
    write_file(&inputs_path, &valid_structured_inputs_yaml());
    let output = run_in(
        dir.path(),
        &[
            "author",
            "charter",
            "--mode",
            "guided-adaptive",
            "--from-inputs",
            inputs_path.to_str().expect("utf-8 path"),
            "--json",
        ],
    );

    assert!(
        output.status.success(),
        "file inputs should succeed: {}",
        stdout(&output)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).expect("author JSON");
    assert_eq!(value["operation"], "author");
    assert_eq!(value["status"], "succeeded");
    assert!(value["candidate_ref"].as_str().is_some());
    assert!(value["changed_paths"]
        .as_array()
        .is_some_and(|paths| !paths.is_empty()));
    assert!(!dir.path().join(".handbook/project/charter.yaml").exists());
    assert!(!dir.path().join("artifacts/charter/CHARTER.md").exists());
    assert!(!dir.path().join("CHARTER.md").exists());
}

#[test]
fn file_inputs_author_charter_repairs_semantically_invalid_canonical_truth() {
    let dir = legacy_authoring_fixture_repo();
    let selected = dir.path().join(".handbook/project/charter.yaml");
    write_file(&selected, "schema_id: invalid-charter\n");
    let before = fs::read(&selected).expect("invalid selected Charter");
    let inputs_path = dir.path().join("charter-inputs.yaml");
    write_file(&inputs_path, &valid_structured_inputs_yaml());
    let output = run_in(
        dir.path(),
        &[
            "author",
            "charter",
            "--mode",
            "guided-adaptive",
            "--from-inputs",
            inputs_path.to_str().expect("utf-8 path"),
            "--json",
        ],
    );

    assert!(
        !output.status.success(),
        "authoring must refuse while selected truth is invalid: {}",
        stdout(&output)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).expect("author JSON");
    assert_eq!(value["operation"], "author");
    assert_eq!(value["status"], "refused");
    assert_eq!(value["refusal"]["code"], "canonical_charter_invalid");
    assert_eq!(value["changed_paths"], serde_json::json!([]));
    assert_eq!(
        fs::read(&selected).expect("selected Charter after author"),
        before
    );
}

#[test]
fn stdin_inputs_author_charter_successfully_with_deterministic_rendering() {
    let dir = legacy_authoring_fixture_repo();
    let intake = valid_structured_inputs_yaml();
    let output = run_in_with_input(
        dir.path(),
        &[
            "author",
            "charter",
            "--mode",
            "guided-adaptive",
            "--from-inputs",
            "-",
            "--json",
        ],
        &intake,
    );

    assert!(
        output.status.success(),
        "stdin inputs should succeed: {}",
        stdout(&output)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).expect("author JSON");
    assert_eq!(value["operation"], "author");
    assert_eq!(value["status"], "succeeded");
    assert!(value["candidate_ref"].as_str().is_some());
    assert!(!dir.path().join(".handbook/project/charter.yaml").exists());
    assert!(!dir.path().join("artifacts/charter/CHARTER.md").exists());
    assert!(!dir.path().join("CHARTER.md").exists());
}

#[test]
fn validate_from_inputs_succeeds_without_mutation() {
    let dir = legacy_authoring_fixture_repo();
    let selected = dir.path().join(".handbook/project/charter.yaml");
    write_file(&selected, CANONICAL_CHARTER_YAML);
    let before = fs::read(&selected).expect("selected Charter");
    let output = run_in(dir.path(), &["author", "charter", "--validate", "--json"]);

    assert!(
        output.status.success(),
        "validate should succeed: {}",
        stdout(&output)
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).expect("validate JSON");
    assert_eq!(value["operation"], "validate");
    assert_eq!(value["status"], "succeeded");
    assert_eq!(value["changed_paths"], serde_json::json!([]));
    assert_eq!(fs::read(&selected).expect("Charter after validate"), before);
}

#[test]
fn validate_refuses_without_from_inputs() {
    let dir = legacy_authoring_fixture_repo();

    let output = run_in(dir.path(), &["author", "charter", "--validate"]);

    assert!(
        !output.status.success(),
        "validate without selected truth should refuse"
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"));
    assert!(out.contains("OPERATION: validate"));
    assert!(out.contains("CODE: canonical_charter_missing"));
}

#[test]
fn bare_project_context_author_requires_structured_inputs() {
    let dir = legacy_authoring_fixture_repo();

    let output = run_in(dir.path(), &["author", "project-context"]);

    assert!(
        !output.status.success(),
        "bare project-context author should refuse"
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"));
    assert!(out.contains("CATEGORY: InvalidRequest"));
    assert!(out.contains("requires `--from-inputs <path|->`"));
    assert!(out.contains("handbook author project-context --from-inputs <path|->"));
}

#[test]
fn project_context_validate_file_and_stdin_are_non_mutating() {
    for source in ["file", "stdin"] {
        let dir = legacy_authoring_fixture_repo();
        let legacy = dir
            .path()
            .join(".handbook/project_context/PROJECT_CONTEXT.md");
        let selected = dir.path().join(".handbook/project/context.yaml");
        let before = fs::read(&legacy).expect("legacy starter project context");
        assert!(!selected.exists());

        let output = if source == "file" {
            let inputs_path = dir.path().join("project-context-inputs.yaml");
            write_file(&inputs_path, valid_project_context_inputs_yaml());
            run_in(
                dir.path(),
                &[
                    "author",
                    "project-context",
                    "--validate",
                    "--from-inputs",
                    inputs_path.to_str().expect("utf8 inputs path"),
                ],
            )
        } else {
            run_in_with_input(
                dir.path(),
                &[
                    "author",
                    "project-context",
                    "--validate",
                    "--from-inputs",
                    "-",
                ],
                valid_project_context_inputs_yaml(),
            )
        };

        assert!(output.status.success(), "{}", stdout(&output));
        let out = stdout(&output);
        assert!(out.contains("OUTCOME: VALIDATED"), "{out}");
        assert_eq!(
            fs::read(&legacy).expect("legacy project context after validation"),
            before
        );
        assert!(!selected.exists());
    }
}

#[cfg(not(unix))]
#[test]
fn project_context_author_refuses_before_mutation_without_strict_platform_support() {
    let dir = legacy_authoring_fixture_repo();
    let inputs_path = dir.path().join("project-context-inputs.yaml");
    write_file(&inputs_path, valid_project_context_inputs_yaml());
    let legacy_path = dir
        .path()
        .join(".handbook/project_context/PROJECT_CONTEXT.md");
    let legacy_before = fs::read(&legacy_path).expect("legacy starter bytes");
    let selected_path = dir.path().join(".handbook/project/context.yaml");
    assert!(!selected_path.exists());

    let output = run_in(
        dir.path(),
        &[
            "author",
            "project-context",
            "--from-inputs",
            inputs_path.to_str().expect("UTF-8 inputs path"),
        ],
    );

    assert!(!output.status.success());
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"), "{out}");
    assert!(
        out.contains("CATEGORY: UnsupportedPlatformStrictMutation"),
        "{out}"
    );
    assert!(out.contains(".handbook/project/context.yaml"), "{out}");
    assert_eq!(
        fs::read(&legacy_path).expect("legacy bytes after refusal"),
        legacy_before
    );
    assert!(!selected_path.exists());
}

#[test]
fn project_context_file_inputs_refuse_when_yaml_is_malformed() {
    let dir = legacy_authoring_fixture_repo();
    let inputs_path = dir.path().join("project-context-inputs.yaml");
    write_file(&inputs_path, "project_summary: [not valid");

    let output = run_in(
        dir.path(),
        &[
            "author",
            "project-context",
            "--from-inputs",
            inputs_path.to_str().expect("utf-8 path"),
        ],
    );

    assert!(
        !output.status.success(),
        "malformed project-context yaml should refuse: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"));
    assert!(out.contains("CATEGORY: MalformedStructuredInput"));
}

#[test]
fn project_context_stdin_inputs_refuse_when_yaml_is_malformed() {
    let dir = legacy_authoring_fixture_repo();

    let output = run_in_with_input(
        dir.path(),
        &["author", "project-context", "--from-inputs", "-"],
        "schema_version: [broken\n",
    );

    assert!(
        !output.status.success(),
        "malformed stdin project-context yaml should refuse: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"));
    assert!(out.contains("CATEGORY: MalformedStructuredInput"));
    assert!(out.contains("OBJECT: author project-context"));
}

#[cfg(unix)]
#[test]
fn project_context_file_inputs_succeed() {
    let dir = legacy_authoring_fixture_repo();
    let inputs_path = dir.path().join("project-context-inputs.yaml");
    write_file(&inputs_path, valid_project_context_inputs_yaml());

    let output = with_project_context_now_utc("2026-04-21T12:34:56Z", || {
        run_in(
            dir.path(),
            &[
                "author",
                "project-context",
                "--from-inputs",
                inputs_path.to_str().expect("utf-8 path"),
            ],
        )
    });

    assert!(
        output.status.success(),
        "project-context file inputs should succeed: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: AUTHORED"), "{out}");
    assert!(out.contains("MODE: structured_inputs_file"), "{out}");
    assert!(out.contains("SOURCE: "), "{out}");
    let expected_yaml = expected_project_context_markdown_from_yaml();
    let record =
        handbook_compiler::parse_project_context_input_yaml(valid_project_context_inputs_yaml())
            .unwrap();
    let rendered = handbook_engine::render_project_context_markdown(&record).unwrap();
    assert!(
        out.contains(&format!("BYTES WRITTEN: {}", expected_yaml.len())),
        "{out}"
    );
    assert!(
        out.contains(&format!(
            "SOURCE FINGERPRINT: {}",
            handbook_engine::project_context_source_fingerprint(expected_yaml.as_bytes()).as_str()
        )),
        "{out}"
    );
    assert!(
        out.contains(&format!(
            "RENDERED OUTPUT FINGERPRINT: {}",
            handbook_engine::project_context_rendered_fingerprint(&rendered).as_str()
        )),
        "{out}"
    );
    assert!(out.contains("RENDERED MEDIA TYPE: text/markdown"), "{out}");
    assert_eq!(
        fs::read_to_string(dir.path().join(".handbook/project/context.yaml"))
            .expect("project context"),
        expected_yaml
    );
}

#[cfg(unix)]
#[test]
fn project_context_stdin_inputs_succeed() {
    let dir = legacy_authoring_fixture_repo();

    let output = with_project_context_now_utc("2026-04-21T12:34:56Z", || {
        run_in_with_input(
            dir.path(),
            &["author", "project-context", "--from-inputs", "-"],
            valid_project_context_inputs_yaml(),
        )
    });

    assert!(
        output.status.success(),
        "project-context stdin inputs should succeed: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: AUTHORED"), "{out}");
    assert!(out.contains("MODE: structured_inputs_stdin"), "{out}");
    assert!(out.contains("SOURCE: -"), "{out}");
    assert_eq!(
        fs::read_to_string(dir.path().join(".handbook/project/context.yaml"))
            .expect("project context"),
        expected_project_context_markdown_from_yaml()
    );
}

#[cfg(unix)]
#[test]
fn project_context_file_inputs_repair_semantically_invalid_canonical_truth() {
    let dir = legacy_authoring_fixture_repo();
    write_file(
        &dir.path().join(".handbook/project/context.yaml"),
        "custom project context truth\n",
    );
    let inputs_path = dir.path().join("project-context-inputs.yaml");
    write_file(&inputs_path, valid_project_context_inputs_yaml());

    let output = with_project_context_now_utc("2026-04-21T12:34:56Z", || {
        run_in(
            dir.path(),
            &[
                "author",
                "project-context",
                "--from-inputs",
                inputs_path.to_str().expect("utf-8 path"),
            ],
        )
    });

    assert!(
        output.status.success(),
        "repair should succeed: {}",
        stdout(&output)
    );
    assert_eq!(
        fs::read_to_string(dir.path().join(".handbook/project/context.yaml"))
            .expect("project context"),
        expected_project_context_markdown_from_yaml()
    );
}

#[test]
fn charter_input_templates_and_fixtures_use_canonical_exception_record_location() {
    let shipped_template = include_str!("../../../core/library/charter/CHARTER_INPUTS.yaml.tmpl");
    let brownfield_fixture =
        include_str!("../../../tools/fixtures/charter_inputs/brownfield_external_web.yaml");
    let greenfield_fixture =
        include_str!("../../../tools/fixtures/charter_inputs/greenfield_internal_api.yaml");
    let runtime_fixture =
        include_str!("../../../tools/fixtures/charter_inputs/runtime_smoke_valid.yaml");

    for contents in [
        shipped_template,
        brownfield_fixture,
        greenfield_fixture,
        runtime_fixture,
    ] {
        assert!(
            contents.contains(".handbook/project/charter.yaml#/governance/exception_process"),
            "selected Charter exception location missing"
        );
        assert!(!contents.contains(".handbook/charter/CHARTER.md#exceptions"));
    }
}
