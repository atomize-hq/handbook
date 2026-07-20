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

fn valid_environment_inventory_markdown(project_context_ref: &str) -> String {
    format!(
        "# Environment Inventory - Handbook\n\n> **Canonical File:** `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`\n> **Project Context Ref:** {project_context_ref}\n\n## What this is\nCanonical environment and runtime inventory.\n\n## How to use\n- Update this file when runtime assumptions change.\n\n## 1) Environment Variables (Inventory)\n- None yet.\n\n## 2) External Services / Infrastructure Dependencies\n- None yet.\n\n## 3) Runtime Assumptions (Ports, Paths, Storage, Limits)\n- None yet.\n\n## 4) Local Development Requirements\n- None yet.\n\n## 5) CI Requirements\n- None yet.\n\n## 6) Production / Deployment Requirements (even if not live yet)\n- None yet.\n\n## 7) Dependency & Tooling Inventory (project-specific)\n- None yet.\n\n## 8) Update Contract (non-negotiable)\n- Update `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md` in the same change.\n\n## 9) Known Unknowns\n- None yet.\n"
    )
    .trim_end()
    .to_string()
}

#[cfg(not(windows))]
fn valid_environment_inventory_inputs_yaml() -> &'static str {
    r#"schema_version: "0.1.0"
project_name: "Handbook"
owner: "compiler-team"
team: "Handbook"
repo_or_project_ref: "handbook"
charter_ref: ".handbook/charter/CHARTER.md"
project_context_ref: ".handbook/project/context.yaml"
environment_variables: []
secret_handling:
  charter_posture: "never store real credentials in repository artifacts"
  storage_locations: ["operator secret store"]
  rotation_expectations: "follow the owning provider policy"
external_services: []
runtime_assumptions:
  listening_ports: "None"
  filesystem_requirements: "write access to the managed repository"
  persistent_storage: "repository-local canonical artifacts"
  network_assumptions: "Unknown for future hosted use; offline authoring requires none"
  performance_budgets: "normal CLI latency"
local_development:
  prerequisites: ["Rust stable toolchain"]
  works_on_my_machine_prevention: "run workspace tests and install smoke"
  environment_file_pattern: "None"
ci:
  system: "GitHub Actions"
  required_secret_names: ["None"]
  services: ["None"]
  artifacts: ["test output"]
production:
  exists_today: false
  hosting_model: "Not applicable"
  runtime_environments: ["local CLI"]
  required_secret_names: ["None"]
  observability: "command output and CI logs"
  backup_and_disaster_recovery: "git history"
tooling:
  primary_language_runtime: "Rust stable"
  package_manager_build_system: "Cargo"
  lockfiles: ["Cargo.lock"]
  lint_type_test_tools: ["rustfmt", "clippy", "cargo test"]
  minimum_versions: ["Rust 2021 edition"]
update_contract:
  exception_record_location: ".handbook/charter/CHARTER.md#exceptions"
known_unknowns:
  - item: "future hosted runtime requirements"
    owner: "project owner"
    revisit_trigger: "before adding a hosted deployment"
"#
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

#[cfg(unix)]
#[test]
fn environment_inventory_file_inputs_author_deterministically_without_codex() {
    let dir = legacy_authoring_fixture_repo();
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        "# Engineering Charter — Example\n\n## What this is\nExample charter truth for environment inventory authoring.\n\n## How to use this charter\nUse it to validate upstream charter requirements.\n\n## Rubric: 1–5 rigor levels\n- Keep secrets out of git.\n\n## Project baseline posture\nBaseline defined.\n\n## Domains / areas (optional overrides)\nNone.\n\n## Posture at a glance (quick scan)\nStable.\n\n## Dimensions (details + guardrails)\nKeep trust boundaries intact.\n\n## Cross-cutting red lines (global non-negotiables)\n- Do not commit secrets.\n\n## Exceptions / overrides process\n- **Approvers:** engineering\n- **Record location:** docs/exceptions.md\n- **Minimum required fields:**\n  - what\n  - why\n  - scope\n  - risk\n  - owner\n  - expiry_or_revisit_date\n\n## Debt tracking expectations\nTrack follow-up work.\n\n## Decision Records (ADRs): how to use this charter\nNot required.\n\n## Review & updates\nReview when runtime assumptions change.\n",
    );
    write_valid_selected_project_context(dir.path());
    let inputs_path = dir.path().join("environment-inventory-inputs.yaml");
    write_file(&inputs_path, valid_environment_inventory_inputs_yaml());
    let output = run_in(
        dir.path(),
        &[
            "author",
            "environment-inventory",
            "--from-inputs",
            inputs_path.to_str().expect("utf8 inputs path"),
        ],
    );

    assert!(
        output.status.success(),
        "environment inventory authoring should succeed: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: AUTHORED"), "{out}");
    assert!(
        out.contains("OBJECT: author environment-inventory"),
        "{out}"
    );
    assert!(
        out.contains("Wrote canonical environment inventory to .handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        "{out}"
    );
    assert!(out.contains("MODE: structured_inputs_file"), "{out}");
    let markdown = fs::read_to_string(
        dir.path()
            .join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
    )
    .expect("environment inventory");
    assert!(markdown.starts_with("# Environment Inventory — Handbook"));
    assert!(markdown.contains("## 9) Known Unknowns"));
    assert!(!dir.path().join("ENVIRONMENT_INVENTORY.md").exists());
    assert!(!dir
        .path()
        .join("artifacts/foundation/ENVIRONMENT_INVENTORY.md")
        .exists());
}

#[cfg(unix)]
#[test]
fn environment_inventory_stdin_inputs_author_deterministically() {
    let dir = legacy_authoring_fixture_repo();
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        "# Engineering Charter — Example\n\n## What this is\nExample.\n\n## How to use this charter\nUse it.\n\n## Rubric: 1–5 rigor levels\nLevels.\n\n## Project baseline posture\nBaseline.\n\n## Domains / areas (optional overrides)\nNone.\n\n## Posture at a glance (quick scan)\nStable.\n\n## Dimensions (details + guardrails)\nDetails.\n\n## Cross-cutting red lines (global non-negotiables)\n- No secrets.\n\n## Exceptions / overrides process\n- **Approvers:** engineering\n- **Record location:** docs/exceptions.md\n- **Minimum required fields:**\n  - what\n  - why\n  - scope\n  - risk\n  - owner\n  - expiry_or_revisit_date\n\n## Debt tracking expectations\nTrack debt.\n\n## Decision Records (ADRs): how to use this charter\nUse ADRs.\n\n## Review & updates\nReview changes.\n",
    );
    write_valid_selected_project_context(dir.path());

    let output = run_in_with_input(
        dir.path(),
        &["author", "environment-inventory", "--from-inputs", "-"],
        valid_environment_inventory_inputs_yaml(),
    );

    assert!(output.status.success(), "{}", stdout(&output));
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: AUTHORED"), "{out}");
    assert!(out.contains("MODE: structured_inputs_stdin"), "{out}");
}

#[cfg(unix)]
#[test]
fn environment_inventory_validate_is_non_mutating() {
    let dir = legacy_authoring_fixture_repo();
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        "# Engineering Charter — Example\n\n## What this is\nExample.\n\n## How to use this charter\nUse it.\n\n## Rubric: 1–5 rigor levels\nLevels.\n\n## Project baseline posture\nBaseline.\n\n## Domains / areas (optional overrides)\nNone.\n\n## Posture at a glance (quick scan)\nStable.\n\n## Dimensions (details + guardrails)\nDetails.\n\n## Cross-cutting red lines (global non-negotiables)\n- No secrets.\n\n## Exceptions / overrides process\n- **Approvers:** engineering\n- **Record location:** docs/exceptions.md\n- **Minimum required fields:**\n  - what\n  - why\n  - scope\n  - risk\n  - owner\n  - expiry_or_revisit_date\n\n## Debt tracking expectations\nTrack debt.\n\n## Decision Records (ADRs): how to use this charter\nUse ADRs.\n\n## Review & updates\nReview changes.\n",
    );
    write_valid_selected_project_context(dir.path());
    let canonical = dir
        .path()
        .join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md");
    let before = fs::read(&canonical).expect("starter inventory");

    let output = run_in_with_input(
        dir.path(),
        &[
            "author",
            "environment-inventory",
            "--validate",
            "--from-inputs",
            "-",
        ],
        valid_environment_inventory_inputs_yaml(),
    );

    assert!(output.status.success(), "{}", stdout(&output));
    assert!(stdout(&output).contains("OUTCOME: VALIDATED"));
    assert_eq!(
        fs::read(&canonical).expect("inventory after validate"),
        before
    );
}

#[test]
fn bare_environment_inventory_command_requires_structured_inputs_without_codex() {
    let dir = legacy_authoring_fixture_repo();
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        "# Engineering Charter - Example\n\n## Rules\n\n- Keep secrets out of git.\n",
    );
    write_file(
        &dir.path()
            .join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        &valid_environment_inventory_markdown("None"),
    );
    let output = run_in(dir.path(), &["author", "environment-inventory"]);

    assert!(
        !output.status.success(),
        "bare environment inventory command should refuse: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"), "{out}");
    assert!(out.contains("CATEGORY: InvalidRequest"), "{out}");
    assert!(out.contains("requires `--from-inputs <path|->`"), "{out}");
    assert_eq!(
        fs::read_to_string(
            dir.path()
                .join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md")
        )
        .expect("environment inventory"),
        valid_environment_inventory_markdown("None")
    );
}

#[cfg(unix)]
#[test]
fn environment_inventory_file_inputs_repair_semantically_invalid_canonical_truth() {
    let dir = legacy_authoring_fixture_repo();
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        "# Engineering Charter — Example\n\n## What this is\nExample charter truth for environment inventory authoring.\n\n## How to use this charter\nUse it to validate upstream charter requirements.\n\n## Rubric: 1–5 rigor levels\n- Keep secrets out of git.\n\n## Project baseline posture\nBaseline defined.\n\n## Domains / areas (optional overrides)\nNone.\n\n## Posture at a glance (quick scan)\nStable.\n\n## Dimensions (details + guardrails)\nKeep trust boundaries intact.\n\n## Cross-cutting red lines (global non-negotiables)\n- Do not commit secrets.\n\n## Exceptions / overrides process\n- **Approvers:** engineering\n- **Record location:** docs/exceptions.md\n- **Minimum required fields:**\n  - what\n  - why\n  - scope\n  - risk\n  - owner\n  - expiry_or_revisit_date\n\n## Debt tracking expectations\nTrack follow-up work.\n\n## Decision Records (ADRs): how to use this charter\nNot required.\n\n## Review & updates\nReview when runtime assumptions change.\n",
    );
    write_valid_selected_project_context(dir.path());
    write_file(
        &dir.path()
            .join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        "custom environment inventory truth\n",
    );
    let inputs_path = dir.path().join("environment-inventory-inputs.yaml");
    write_file(&inputs_path, valid_environment_inventory_inputs_yaml());
    let output = run_in(
        dir.path(),
        &[
            "author",
            "environment-inventory",
            "--from-inputs",
            inputs_path.to_str().expect("utf8 inputs path"),
        ],
    );

    assert!(
        output.status.success(),
        "repair should succeed: {}",
        stdout(&output)
    );
    let markdown = fs::read_to_string(
        dir.path()
            .join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
    )
    .expect("environment inventory");
    assert!(markdown.starts_with("# Environment Inventory — Handbook"));
}

#[cfg(all(not(unix), not(windows)))]
#[test]
fn environment_inventory_author_refuses_before_mutation_without_strict_read_support() {
    let dir = legacy_authoring_fixture_repo();
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        "# Engineering Charter — Example\n\n## What this is\nExample.\n\n## How to use this charter\nUse it.\n\n## Rubric: 1–5 rigor levels\nLevels.\n\n## Project baseline posture\nBaseline.\n\n## Domains / areas (optional overrides)\nNone.\n\n## Posture at a glance (quick scan)\nStable.\n\n## Dimensions (details + guardrails)\nDetails.\n\n## Cross-cutting red lines (global non-negotiables)\n- No secrets.\n\n## Exceptions / overrides process\n- **Approvers:** engineering\n- **Record location:** docs/exceptions.md\n- **Minimum required fields:**\n  - what\n  - why\n  - scope\n  - risk\n  - owner\n  - expiry_or_revisit_date\n\n## Debt tracking expectations\nTrack debt.\n\n## Decision Records (ADRs): how to use this charter\nUse ADRs.\n\n## Review & updates\nReview changes.\n",
    );
    let target = dir
        .path()
        .join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md");
    let before = fs::read(&target).expect("starter environment inventory bytes");

    let output = run_in_with_input(
        dir.path(),
        &["author", "environment-inventory", "--from-inputs", "-"],
        valid_environment_inventory_inputs_yaml(),
    );

    assert!(!output.status.success());
    let out = stdout(&output);
    assert!(
        out.contains("CATEGORY: InvalidUpstreamCanonicalTruth"),
        "{out}"
    );
    assert!(out.contains("UnsupportedPlatformStrictRead"), "{out}");
    assert!(
        out.contains("BROKEN SUBJECT: .handbook/project/context.yaml"),
        "{out}"
    );
    assert_eq!(
        fs::read(&target).expect("environment inventory after refusal"),
        before
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
