use std::fs;
use std::io::Write;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::path::Path;
use std::process::{Command, Output, Stdio};
const AUTHOR_PROJECT_CONTEXT_NOW_UTC_ENV_VAR: &str = "HANDBOOK_AUTHOR_PROJECT_CONTEXT_NOW_UTC";

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

fn scaffold_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = run_in(dir.path(), &["setup"]);
    assert!(
        output.status.success(),
        "setup failed: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    dir
}

fn valid_environment_inventory_markdown(project_context_ref: &str) -> String {
    format!(
        "# Environment Inventory - Handbook\n\n> **Canonical File:** `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`\n> **Project Context Ref:** {project_context_ref}\n\n## What this is\nCanonical environment and runtime inventory.\n\n## How to use\n- Update this file when runtime assumptions change.\n\n## 1) Environment Variables (Inventory)\n- None yet.\n\n## 2) External Services / Infrastructure Dependencies\n- None yet.\n\n## 3) Runtime Assumptions (Ports, Paths, Storage, Limits)\n- None yet.\n\n## 4) Local Development Requirements\n- None yet.\n\n## 5) CI Requirements\n- None yet.\n\n## 6) Production / Deployment Requirements (even if not live yet)\n- None yet.\n\n## 7) Dependency & Tooling Inventory (project-specific)\n- None yet.\n\n## 8) Update Contract (non-negotiable)\n- Update `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md` in the same change.\n\n## 9) Known Unknowns\n- None yet.\n"
    )
    .trim_end()
    .to_string()
}

fn valid_environment_inventory_inputs_yaml() -> &'static str {
    r#"schema_version: "0.1.0"
project_name: "Handbook"
owner: "compiler-team"
team: "Handbook"
repo_or_project_ref: "handbook"
charter_ref: ".handbook/charter/CHARTER.md"
project_context_ref: null
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
    r#"schema_version: "0.1.0"
project_name: "Handbook"
owner: "compiler-team"
team: "Handbook"
repo_or_project_ref: "handbook"
charter_ref: ".handbook/charter/CHARTER.md"
project_summary:
  what_this_project_is: "CLI and compiler for canonical planning artifacts and workflow proofs"
  primary_surface: "CLI plus compiler library"
  primary_users: "internal operators and automation"
  key_workflows:
    - "scaffold canonical .handbook state"
    - "author baseline artifacts"
    - "compile and inspect planning outputs"
  non_goals: "End-user product delivery"
operational_reality:
  is_live_in_production_today: "no"
  users: "internal operators only"
  data_in_production: "none"
  uptime_expectations: "best effort during active development"
  incident_on_call_reality: "no formal on-call rotation today"
  primary_risk_flags_present: "incorrect planning guidance and canonical write regressions"
classification_implications:
  project_type: "greenfield with an active brownfield codebase"
  backward_compatibility_required: "no"
  backward_compatibility_notes: "no external customers depend on the current compiler API"
  migration_planning_required: "not applicable"
  migration_planning_notes: "no legacy production data to migrate"
  deprecation_policy_exists: "not yet"
  deprecation_policy_notes: "internal interfaces can change with coordinated release notes"
  rollout_controls_required: "lightweight only"
  rollout_controls_notes: "feature branches and tests gate changes before merge"
system_boundaries:
  owned_areas:
    - "compiler and CLI crates in this repository"
    - "canonical .handbook artifact formats and setup flow"
  external_dependencies:
    - "OpenAI Codex runtime used for charter synthesis"
    - "local filesystem layout and git worktree state"
integrations:
  - name: "Codex exec"
    integration_type: "CLI runtime"
    contract_surface: "codex exec --output-last-message -"
    authentication_authorization: "inherits local operator credentials and API configuration"
    failure_mode_expectations: "auth or process failures must refuse without partial writes"
environments_and_delivery:
  environments_that_exist: "local development and CI"
  deployment_model: "cargo-driven local execution"
  ci_cd_reality: "basic CI with compiler and CLI test coverage"
  release_cadence: "repo-driven iterative releases"
  config_and_secrets: "standard local environment variables and git config"
  observability_stack: "test output and local command stderr"
data_reality:
  primary_data_stores: "repo-local markdown, yaml, and route-state files"
  data_classification: "source code and internal planning metadata"
  retention_requirements: "none beyond repository history"
  backups_disaster_recovery: "git history plus local worktree backups"
  existing_migrations_history: "none for production data"
repo_codebase_reality:
  codebase_exists_today: true
  current_maturity: "medium-sized active Rust workspace"
  key_modules_or_areas:
    - "crates/compiler"
    - "crates/cli"
    - "core/library"
  known_constraints_from_existing_code: "lane ownership and canonical artifact ordering must be preserved"
constraints:
  deadline_time_constraints: "must fit the current milestone split"
  budget_constraints: "limited to local engineering time"
  must_use_or_prohibited_tech: "must stay in Rust and preserve existing canonical paths"
  compliance_legal_constraints: "none beyond repository policy"
  performance_constraints: "compiler authoring should stay fast and deterministic"
  security_constraints: "no writes outside canonical repo-owned targets"
known_unknowns:
  - item: "final CLI interview wording for project-context authoring"
    owner: "Lane D"
    revisit_trigger: "when the CLI subcommand lands"
"#
}

fn expected_project_context_markdown_from_yaml() -> String {
    let input = handbook_engine::parse_project_context_structured_input_yaml(
        valid_project_context_inputs_yaml(),
    )
    .expect("parse project-context yaml");

    with_project_context_now_utc("2026-04-21T12:34:56Z", || {
        handbook_engine::render_project_context_markdown(&input, "2026-04-21T12:34:56Z")
            .expect("render project-context markdown")
    })
}

fn valid_structured_inputs_yaml() -> &'static str {
    r#"schema_version: "0.1.0"
project:
  name: "Handbook"
  classification: greenfield
  team_size: 2
  users: internal
  expected_lifetime: months
  surfaces:
    - cli
    - api
  runtime_environments:
    - server
  constraints:
    deadline: ""
    budget: ""
    experience_notes: "small team"
    must_use_tech:
      - rust
  operational_reality:
    in_production_today: false
    prod_users_or_data: ""
    external_contracts_to_preserve: []
    uptime_expectations: "best effort"
  default_implications:
    backward_compatibility: not_required
    migration_planning: not_required
    rollout_controls: lightweight
    deprecation_policy: not_required_yet
    observability_threshold: standard
posture:
  rubric_scale: "1-5"
  baseline_level: 3
  baseline_rationale:
    - "internal operators"
    - "moderate blast radius"
domains:
  - name: "planning"
    blast_radius: "medium"
    touches:
      - "internal operators"
    constraints:
      - "preserve trust boundaries"
dimensions:
  - name: speed_vs_quality
    level: 3
    default_stance: "optimize for durability over shortcuts"
    raise_the_bar_triggers: ["production data"]
    allowed_shortcuts: ["time-boxed exploration"]
    red_lines: ["ship without review"]
    domain_overrides: []
  - name: type_safety_static_analysis
    level: 3
    default_stance: "type-safe by default"
    raise_the_bar_triggers: ["cross-boundary interfaces"]
    allowed_shortcuts: ["fixture-backed exploration"]
    red_lines: ["unchecked public contracts"]
    domain_overrides: []
  - name: testing_rigor
    level: 3
    default_stance: "test the shipped path"
    raise_the_bar_triggers: ["regression risk"]
    allowed_shortcuts: ["manual validation for throwaway work"]
    red_lines: ["merge without exercising the path"]
    domain_overrides: []
  - name: scalability_performance
    level: 3
    default_stance: "track obvious bottlenecks"
    raise_the_bar_triggers: ["user-visible latency"]
    allowed_shortcuts: ["defer micro-optimizations"]
    red_lines: ["ignore known load cliffs"]
    domain_overrides: []
  - name: reliability_operability
    level: 3
    default_stance: "prefer recoverable changes"
    raise_the_bar_triggers: ["long-lived state changes"]
    allowed_shortcuts: ["local-only iteration"]
    red_lines: ["unrecoverable migrations without a plan"]
    domain_overrides: []
  - name: security_privacy
    level: 3
    default_stance: "protect boundaries by default"
    raise_the_bar_triggers: ["credentials or user data"]
    allowed_shortcuts: ["synthetic data in local dev"]
    red_lines: ["plaintext secrets"]
    domain_overrides: []
  - name: observability
    level: 3
    default_stance: "emit enough proof to debug production issues"
    raise_the_bar_triggers: ["background jobs"]
    allowed_shortcuts: ["manual logs for local-only work"]
    red_lines: ["silent failures"]
    domain_overrides: []
  - name: dx_tooling_automation
    level: 3
    default_stance: "prefer automation that pays for itself"
    raise_the_bar_triggers: ["frequent repeated workflows"]
    allowed_shortcuts: ["temporary local scripts"]
    red_lines: ["manual-only release steps"]
    domain_overrides: []
  - name: ux_polish_api_usability
    level: 3
    default_stance: "clear operator and API ergonomics"
    raise_the_bar_triggers: ["external users"]
    allowed_shortcuts: ["rough internal copy while iterating"]
    red_lines: ["unclear operator failure modes"]
    domain_overrides: []
exceptions:
  approvers:
    - project_owner
  record_location: ".handbook/charter/CHARTER.md#exceptions"
  minimum_fields:
    - what
    - why
    - scope
    - risk
    - owner
    - expiry_or_revisit_date
debt_tracking:
  system: "issues"
  labels:
    - debt
  review_cadence: "monthly"
decision_records:
  enabled: false
  path: ""
  format: ""
"#
}

fn deterministic_authored_markdown() -> String {
    let input =
        handbook_engine::parse_charter_structured_input_yaml(valid_structured_inputs_yaml())
            .expect("parse deterministic charter inputs");
    handbook_engine::render_charter_markdown(&input)
        .expect("render deterministic authored markdown")
}

#[test]
fn bare_charter_author_requires_structured_inputs() {
    let dir = scaffold_repo();

    let output = run_in(dir.path(), &["author", "charter"]);

    assert!(!output.status.success(), "non-tty author should refuse");
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"));
    assert!(out.contains("CATEGORY: InvalidRequest"));
    assert!(out.contains("requires `--from-inputs <path|->`"));
    assert!(out.contains("handbook author charter --from-inputs <path|->"));
}

#[test]
fn file_inputs_refuse_when_yaml_is_malformed() {
    let dir = scaffold_repo();
    let inputs_path = dir.path().join("charter-inputs.yaml");
    write_file(&inputs_path, "project: [not valid");

    let output = run_in(
        dir.path(),
        &[
            "author",
            "charter",
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
    assert!(out.contains("CATEGORY: MalformedStructuredInput"));
}

#[test]
fn stdin_inputs_refuse_when_yaml_is_malformed() {
    let dir = scaffold_repo();

    let output = run_in_with_input(
        dir.path(),
        &["author", "charter", "--from-inputs", "-"],
        "schema_version: [broken\n",
    );

    assert!(
        !output.status.success(),
        "malformed stdin yaml should refuse: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"));
    assert!(out.contains("CATEGORY: MalformedStructuredInput"));
    assert!(out.contains("OBJECT: author charter"));
}

#[test]
fn file_inputs_preserve_malformed_yaml_refusal_even_when_truth_exists() {
    let dir = scaffold_repo();
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        &deterministic_authored_markdown(),
    );
    let inputs_path = dir.path().join("charter-inputs.yaml");
    write_file(&inputs_path, "project: [not valid");

    let output = run_in(
        dir.path(),
        &[
            "author",
            "charter",
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
    assert!(out.contains("CATEGORY: MalformedStructuredInput"));
}

#[test]
fn file_inputs_author_charter_successfully_with_deterministic_rendering() {
    let dir = scaffold_repo();
    let inputs_path = dir.path().join("charter-inputs.yaml");
    write_file(&inputs_path, valid_structured_inputs_yaml());
    let expected_markdown = deterministic_authored_markdown();
    let output = run_in(
        dir.path(),
        &[
            "author",
            "charter",
            "--from-inputs",
            inputs_path.to_str().expect("utf-8 path"),
        ],
    );

    assert!(
        output.status.success(),
        "file inputs should succeed: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: AUTHORED"));
    assert!(out.contains("MODE: structured_inputs_file"));
    assert!(out.contains(&format!("SOURCE: {}", inputs_path.display())));
    assert_eq!(
        fs::read_to_string(dir.path().join(".handbook/charter/CHARTER.md")).expect("charter"),
        expected_markdown
    );
    assert!(!dir.path().join("artifacts/charter/CHARTER.md").exists());
    assert!(!dir.path().join("CHARTER.md").exists());
}

#[test]
fn file_inputs_author_charter_repairs_semantically_invalid_canonical_truth() {
    let dir = scaffold_repo();
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        "# Engineering Charter - Example\n\n## Rules\n\n- Keep secrets out of git.\n",
    );
    let inputs_path = dir.path().join("charter-inputs.yaml");
    write_file(&inputs_path, valid_structured_inputs_yaml());
    let expected_markdown = deterministic_authored_markdown();
    let output = run_in(
        dir.path(),
        &[
            "author",
            "charter",
            "--from-inputs",
            inputs_path.to_str().expect("utf-8 path"),
        ],
    );

    assert!(
        output.status.success(),
        "repair should succeed: {}",
        stdout(&output)
    );
    assert_eq!(
        fs::read_to_string(dir.path().join(".handbook/charter/CHARTER.md")).expect("charter"),
        expected_markdown
    );
}

#[test]
fn stdin_inputs_author_charter_successfully_with_deterministic_rendering() {
    let dir = scaffold_repo();
    let expected_markdown = deterministic_authored_markdown();
    let output = run_in_with_input(
        dir.path(),
        &["author", "charter", "--from-inputs", "-"],
        valid_structured_inputs_yaml(),
    );

    assert!(
        output.status.success(),
        "stdin inputs should succeed: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: AUTHORED"));
    assert!(out.contains("MODE: structured_inputs_stdin"));
    assert!(out.contains("SOURCE: -"));
    assert_eq!(
        fs::read_to_string(dir.path().join(".handbook/charter/CHARTER.md")).expect("charter"),
        expected_markdown
    );
    assert!(!dir.path().join("artifacts/charter/CHARTER.md").exists());
    assert!(!dir.path().join("CHARTER.md").exists());
}

#[test]
fn validate_from_inputs_succeeds_without_mutation() {
    let dir = scaffold_repo();
    let inputs_path = dir.path().join("charter-inputs.yaml");
    write_file(&inputs_path, valid_structured_inputs_yaml());
    let before =
        fs::read(dir.path().join(".handbook/charter/CHARTER.md")).expect("starter charter");
    let output = run_in(
        dir.path(),
        &[
            "author",
            "charter",
            "--validate",
            "--from-inputs",
            inputs_path.to_str().expect("utf-8 path"),
        ],
    );

    assert!(
        output.status.success(),
        "validate should succeed: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: VALIDATED"));
    assert!(out.contains("MODE: structured_inputs_file"));
    assert_eq!(
        fs::read(dir.path().join(".handbook/charter/CHARTER.md")).expect("charter after validate"),
        before
    );
}

#[test]
fn validate_refuses_without_from_inputs() {
    let dir = scaffold_repo();

    let output = run_in(dir.path(), &["author", "charter", "--validate"]);

    assert!(
        !output.status.success(),
        "validate without from-inputs should refuse"
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"));
    assert!(out.contains("CATEGORY: InvalidRequest"));
    assert!(out.contains("--validate"));
    assert!(out.contains("--from-inputs <path|->"));
}

#[test]
fn bare_project_context_author_requires_structured_inputs() {
    let dir = scaffold_repo();

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
        let dir = scaffold_repo();
        let canonical = dir
            .path()
            .join(".handbook/project_context/PROJECT_CONTEXT.md");
        let before = fs::read(&canonical).expect("starter project context");

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
            fs::read(&canonical).expect("project context after validation"),
            before
        );
    }
}

#[test]
fn project_context_file_inputs_refuse_when_yaml_is_malformed() {
    let dir = scaffold_repo();
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
    let dir = scaffold_repo();

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

#[test]
fn project_context_file_inputs_succeed() {
    let dir = scaffold_repo();
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
    assert_eq!(
        fs::read_to_string(
            dir.path()
                .join(".handbook/project_context/PROJECT_CONTEXT.md")
        )
        .expect("project context"),
        expected_project_context_markdown_from_yaml()
    );
}

#[test]
fn project_context_stdin_inputs_succeed() {
    let dir = scaffold_repo();

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
        fs::read_to_string(
            dir.path()
                .join(".handbook/project_context/PROJECT_CONTEXT.md")
        )
        .expect("project context"),
        expected_project_context_markdown_from_yaml()
    );
}

#[test]
fn project_context_file_inputs_repair_semantically_invalid_canonical_truth() {
    let dir = scaffold_repo();
    write_file(
        &dir.path()
            .join(".handbook/project_context/PROJECT_CONTEXT.md"),
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
        fs::read_to_string(
            dir.path()
                .join(".handbook/project_context/PROJECT_CONTEXT.md")
        )
        .expect("project context"),
        expected_project_context_markdown_from_yaml()
    );
}

#[test]
fn environment_inventory_file_inputs_author_deterministically_without_codex() {
    let dir = scaffold_repo();
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        "# Engineering Charter — Example\n\n## What this is\nExample charter truth for environment inventory authoring.\n\n## How to use this charter\nUse it to validate upstream charter requirements.\n\n## Rubric: 1–5 rigor levels\n- Keep secrets out of git.\n\n## Project baseline posture\nBaseline defined.\n\n## Domains / areas (optional overrides)\nNone.\n\n## Posture at a glance (quick scan)\nStable.\n\n## Dimensions (details + guardrails)\nKeep trust boundaries intact.\n\n## Cross-cutting red lines (global non-negotiables)\n- Do not commit secrets.\n\n## Exceptions / overrides process\n- **Approvers:** engineering\n- **Record location:** docs/exceptions.md\n- **Minimum required fields:**\n  - what\n  - why\n  - scope\n  - risk\n  - owner\n  - expiry_or_revisit_date\n\n## Debt tracking expectations\nTrack follow-up work.\n\n## Decision Records (ADRs): how to use this charter\nNot required.\n\n## Review & updates\nReview when runtime assumptions change.\n",
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

#[test]
fn environment_inventory_stdin_inputs_author_deterministically() {
    let dir = scaffold_repo();
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        "# Engineering Charter — Example\n\n## What this is\nExample.\n\n## How to use this charter\nUse it.\n\n## Rubric: 1–5 rigor levels\nLevels.\n\n## Project baseline posture\nBaseline.\n\n## Domains / areas (optional overrides)\nNone.\n\n## Posture at a glance (quick scan)\nStable.\n\n## Dimensions (details + guardrails)\nDetails.\n\n## Cross-cutting red lines (global non-negotiables)\n- No secrets.\n\n## Exceptions / overrides process\n- **Approvers:** engineering\n- **Record location:** docs/exceptions.md\n- **Minimum required fields:**\n  - what\n  - why\n  - scope\n  - risk\n  - owner\n  - expiry_or_revisit_date\n\n## Debt tracking expectations\nTrack debt.\n\n## Decision Records (ADRs): how to use this charter\nUse ADRs.\n\n## Review & updates\nReview changes.\n",
    );

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

#[test]
fn environment_inventory_validate_is_non_mutating() {
    let dir = scaffold_repo();
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        "# Engineering Charter — Example\n\n## What this is\nExample.\n\n## How to use this charter\nUse it.\n\n## Rubric: 1–5 rigor levels\nLevels.\n\n## Project baseline posture\nBaseline.\n\n## Domains / areas (optional overrides)\nNone.\n\n## Posture at a glance (quick scan)\nStable.\n\n## Dimensions (details + guardrails)\nDetails.\n\n## Cross-cutting red lines (global non-negotiables)\n- No secrets.\n\n## Exceptions / overrides process\n- **Approvers:** engineering\n- **Record location:** docs/exceptions.md\n- **Minimum required fields:**\n  - what\n  - why\n  - scope\n  - risk\n  - owner\n  - expiry_or_revisit_date\n\n## Debt tracking expectations\nTrack debt.\n\n## Decision Records (ADRs): how to use this charter\nUse ADRs.\n\n## Review & updates\nReview changes.\n",
    );
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
    let dir = scaffold_repo();
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

#[test]
fn environment_inventory_file_inputs_repair_semantically_invalid_canonical_truth() {
    let dir = scaffold_repo();
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        "# Engineering Charter — Example\n\n## What this is\nExample charter truth for environment inventory authoring.\n\n## How to use this charter\nUse it to validate upstream charter requirements.\n\n## Rubric: 1–5 rigor levels\n- Keep secrets out of git.\n\n## Project baseline posture\nBaseline defined.\n\n## Domains / areas (optional overrides)\nNone.\n\n## Posture at a glance (quick scan)\nStable.\n\n## Dimensions (details + guardrails)\nKeep trust boundaries intact.\n\n## Cross-cutting red lines (global non-negotiables)\n- Do not commit secrets.\n\n## Exceptions / overrides process\n- **Approvers:** engineering\n- **Record location:** docs/exceptions.md\n- **Minimum required fields:**\n  - what\n  - why\n  - scope\n  - risk\n  - owner\n  - expiry_or_revisit_date\n\n## Debt tracking expectations\nTrack follow-up work.\n\n## Decision Records (ADRs): how to use this charter\nNot required.\n\n## Review & updates\nReview when runtime assumptions change.\n",
    );
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

#[test]
fn charter_input_templates_and_fixtures_use_canonical_exception_record_location() {
    let shipped_template = include_str!("../../../core/library/charter/CHARTER_INPUTS.yaml.tmpl");
    let fixture_template = include_str!(
        "../../../tests/fixtures/foundation_flow_demo/repo/core/library/charter/CHARTER_INPUTS.yaml.tmpl"
    );
    let brownfield_fixture =
        include_str!("../../../tools/fixtures/charter_inputs/brownfield_external_web.yaml");
    let greenfield_fixture =
        include_str!("../../../tools/fixtures/charter_inputs/greenfield_internal_api.yaml");

    for contents in [
        valid_structured_inputs_yaml(),
        shipped_template,
        fixture_template,
        brownfield_fixture,
        greenfield_fixture,
    ] {
        assert!(contents.contains(handbook_engine::DEFAULT_EXCEPTION_RECORD_LOCATION));
        assert!(!contents.contains("record_location: \"CHARTER.md#exceptions\""));
    }
}
