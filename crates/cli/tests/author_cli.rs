use std::fs;
use std::io::{Read, Write};
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use portable_pty::{native_pty_system, CommandBuilder, PtySize};

const AUTHOR_CHARTER_CODEX_BIN_ENV_VAR: &str = "HANDBOOK_AUTHOR_CHARTER_CODEX_BIN";
const AUTHOR_CHARTER_CODEX_MODEL_ENV_VAR: &str = "HANDBOOK_AUTHOR_CHARTER_CODEX_MODEL";
const AUTHOR_ENVIRONMENT_INVENTORY_CODEX_BIN_ENV_VAR: &str =
    "HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_BIN";
const AUTHOR_ENVIRONMENT_INVENTORY_CODEX_MODEL_ENV_VAR: &str =
    "HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_MODEL";
const AUTHOR_PROJECT_CONTEXT_NOW_UTC_ENV_VAR: &str = "HANDBOOK_AUTHOR_PROJECT_CONTEXT_NOW_UTC";
const PROMPT_CAPTURE_REPO_PATH: &str = ".handbook/state/authoring/last_prompt.txt";

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

fn author_runtime_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn with_author_runtime_override<T>(
    binary_path: &Path,
    model: Option<&str>,
    action: impl FnOnce() -> T,
) -> T {
    with_runtime_override(
        AUTHOR_CHARTER_CODEX_BIN_ENV_VAR,
        AUTHOR_CHARTER_CODEX_MODEL_ENV_VAR,
        binary_path,
        model,
        action,
    )
}

fn with_environment_inventory_runtime_override<T>(
    binary_path: &Path,
    model: Option<&str>,
    action: impl FnOnce() -> T,
) -> T {
    with_runtime_override(
        AUTHOR_ENVIRONMENT_INVENTORY_CODEX_BIN_ENV_VAR,
        AUTHOR_ENVIRONMENT_INVENTORY_CODEX_MODEL_ENV_VAR,
        binary_path,
        model,
        action,
    )
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

fn with_runtime_override<T>(
    binary_env_var: &str,
    model_env_var: &str,
    binary_path: &Path,
    model: Option<&str>,
    action: impl FnOnce() -> T,
) -> T {
    let _guard = author_runtime_lock().lock().expect("author runtime lock");
    let previous_bin = std::env::var_os(binary_env_var);
    let previous_model = std::env::var_os(model_env_var);
    std::env::set_var(binary_env_var, binary_path);
    match model {
        Some(value) => std::env::set_var(model_env_var, value),
        None => std::env::remove_var(model_env_var),
    }

    let result = catch_unwind(AssertUnwindSafe(action));

    match previous_bin {
        Some(value) => std::env::set_var(binary_env_var, value),
        None => std::env::remove_var(binary_env_var),
    }
    match previous_model {
        Some(value) => std::env::set_var(model_env_var, value),
        None => std::env::remove_var(model_env_var),
    }

    match result {
        Ok(value) => value,
        Err(payload) => resume_unwind(payload),
    }
}

#[cfg(unix)]
fn install_stub_codex(root: &Path, script: &str) -> std::path::PathBuf {
    use std::os::unix::fs::PermissionsExt;

    let path = root.join("stub-codex.sh");
    write_file(&path, script);
    let mut permissions = fs::metadata(&path).expect("stub metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&path, permissions).expect("chmod stub");
    path
}

#[cfg(not(unix))]
fn install_stub_codex(root: &Path, script: &str) -> std::path::PathBuf {
    let path = root.join("stub-codex.bat");
    write_file(&path, script);
    path
}

fn prompt_capture_path(root: &Path) -> std::path::PathBuf {
    root.join(PROMPT_CAPTURE_REPO_PATH)
}

fn successful_stub_script(markdown: &str) -> String {
    format!(
        "#!/bin/sh\nset -eu\n[ \"$1\" = \"exec\" ] || {{ echo \"expected exec, got: $1\" >&2; exit 97; }}\n[ \"$2\" = \"--skip-git-repo-check\" ] || {{ echo \"expected --skip-git-repo-check\" >&2; exit 97; }}\n[ \"$3\" = \"--sandbox\" ] || {{ echo \"expected --sandbox\" >&2; exit 97; }}\n[ \"$4\" = \"read-only\" ] || {{ echo \"expected read-only sandbox\" >&2; exit 97; }}\n[ \"$5\" = \"--color\" ] || {{ echo \"expected --color\" >&2; exit 97; }}\n[ \"$6\" = \"never\" ] || {{ echo \"expected color=never\" >&2; exit 97; }}\nshift 6\nif [ \"$#\" -eq 5 ]; then\n  [ \"$1\" = \"--model\" ] || {{ echo \"expected --model\" >&2; exit 97; }}\n  [ -n \"$2\" ] || {{ echo \"missing model value\" >&2; exit 97; }}\n  shift 2\nelif [ \"$#\" -ne 3 ]; then\n  echo \"unexpected argv count after prefix: $#\" >&2\n  exit 97\nfi\n[ \"$1\" = \"--output-last-message\" ] || {{ echo \"expected --output-last-message\" >&2; exit 97; }}\noutput=\"$2\"\n[ -n \"$output\" ] || {{ echo \"missing output path\" >&2; exit 97; }}\n[ \"$3\" = \"-\" ] || {{ echo \"expected stdin marker '-'\" >&2; exit 97; }}\nmkdir -p .handbook/state/authoring\ncat > {prompt_capture}\n[ -s {prompt_capture} ] || {{ echo \"prompt capture was empty\" >&2; exit 97; }}\ncat <<'EOF' > \"$output\"\n{markdown}\nEOF\n",
        prompt_capture = PROMPT_CAPTURE_REPO_PATH,
        markdown = markdown
    )
}

fn failing_stub_script() -> String {
    "#!/bin/sh\nset -eu\necho \"unexpected codex invocation\" >&2\nexit 23\n".to_string()
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

fn stubbed_authored_markdown() -> String {
    handbook_engine::render_charter_markdown(&guided_expected_input())
        .expect("render stubbed authored markdown")
}

fn deterministic_authored_markdown() -> String {
    let input =
        handbook_engine::parse_charter_structured_input_yaml(valid_structured_inputs_yaml())
            .expect("parse deterministic charter inputs");
    handbook_engine::render_charter_markdown(&input)
        .expect("render deterministic authored markdown")
}

fn guided_expected_input() -> handbook_engine::CharterStructuredInput {
    let baseline_level = 3;
    let project_name = "Handbook".to_string();
    let in_production_today = false;
    let mut dimensions: Vec<_> = all_dimension_names()
        .iter()
        .copied()
        .map(|name| {
            default_dimension_input(name, baseline_level, &project_name, in_production_today)
        })
        .collect();
    dimensions[0] = handbook_engine::CharterDimensionInput {
        name: handbook_engine::CharterDimensionName::SpeedVsQuality,
        level: Some(4),
        default_stance: "favor durable launches over rush delivery".to_string(),
        raise_the_bar_triggers: vec![
            "changes that affect onboarding conversion".to_string(),
            "irreversible rollout steps".to_string(),
        ],
        allowed_shortcuts: vec![
            "time-boxed prototypes behind a feature flag".to_string(),
            "paired operator review for urgent copy changes".to_string(),
        ],
        red_lines: vec![
            "do not skip launch rollback planning".to_string(),
            "do not trade away review on shipped flows".to_string(),
        ],
        domain_overrides: vec![
            "billing changes stay at level 5 until two successful dry runs".to_string(),
        ],
    };

    handbook_engine::CharterStructuredInput {
        schema_version: "0.1.0".to_string(),
        project: handbook_engine::CharterProjectInput {
            name: project_name.clone(),
            classification: handbook_engine::CharterProjectClassification::Greenfield,
            team_size: 2,
            users: handbook_engine::CharterAudience::Internal,
            expected_lifetime: handbook_engine::CharterExpectedLifetime::Months,
            surfaces: vec![
                handbook_engine::CharterSurface::Cli,
                handbook_engine::CharterSurface::Api,
            ],
            runtime_environments: vec![handbook_engine::CharterRuntimeEnvironment::Server],
            constraints: handbook_engine::CharterProjectConstraintsInput {
                deadline: String::new(),
                budget: String::new(),
                experience_notes: "small team".to_string(),
                must_use_tech: vec!["rust".to_string()],
            },
            operational_reality: handbook_engine::CharterOperationalRealityInput {
                in_production_today,
                prod_users_or_data: String::new(),
                external_contracts_to_preserve: Vec::new(),
                uptime_expectations: "best effort".to_string(),
            },
            default_implications: handbook_engine::CharterDefaultImplicationsInput {
                backward_compatibility: handbook_engine::CharterBackwardCompatibility::NotRequired,
                migration_planning: handbook_engine::CharterRequiredness::NotRequired,
                rollout_controls: handbook_engine::CharterRolloutControls::Lightweight,
                deprecation_policy: handbook_engine::CharterDeprecationPolicy::NotRequiredYet,
                observability_threshold: handbook_engine::CharterObservabilityThreshold::Standard,
            },
        },
        posture: handbook_engine::CharterPostureInput {
            rubric_scale: "1-5".to_string(),
            baseline_level,
            baseline_rationale: vec![
                "internal operators".to_string(),
                "moderate blast radius".to_string(),
            ],
        },
        domains: vec![handbook_engine::CharterDomainInput {
            name: "planning".to_string(),
            blast_radius: "medium".to_string(),
            touches: vec!["internal operators".to_string()],
            constraints: vec!["preserve trust boundaries".to_string()],
        }],
        dimensions,
        exceptions: handbook_engine::CharterExceptionsInput {
            approvers: vec!["project_owner".to_string()],
            record_location: handbook_engine::DEFAULT_EXCEPTION_RECORD_LOCATION.to_string(),
            minimum_fields: default_exception_minimum_fields(),
        },
        debt_tracking: handbook_engine::CharterDebtTrackingInput {
            system: "issues".to_string(),
            labels: vec!["debt".to_string()],
            review_cadence: "monthly".to_string(),
        },
        decision_records: handbook_engine::CharterDecisionRecordsInput {
            enabled: true,
            path: "docs/decisions".to_string(),
            format: "md".to_string(),
        },
    }
}

fn all_dimension_names() -> [handbook_engine::CharterDimensionName; 9] {
    [
        handbook_engine::CharterDimensionName::SpeedVsQuality,
        handbook_engine::CharterDimensionName::TypeSafetyStaticAnalysis,
        handbook_engine::CharterDimensionName::TestingRigor,
        handbook_engine::CharterDimensionName::ScalabilityPerformance,
        handbook_engine::CharterDimensionName::ReliabilityOperability,
        handbook_engine::CharterDimensionName::SecurityPrivacy,
        handbook_engine::CharterDimensionName::Observability,
        handbook_engine::CharterDimensionName::DxToolingAutomation,
        handbook_engine::CharterDimensionName::UxPolishApiUsability,
    ]
}

fn default_exception_minimum_fields() -> Vec<String> {
    [
        "what",
        "why",
        "scope",
        "risk",
        "owner",
        "expiry_or_revisit_date",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn default_dimension_input(
    name: handbook_engine::CharterDimensionName,
    baseline_level: u8,
    project_name: &str,
    in_production_today: bool,
) -> handbook_engine::CharterDimensionInput {
    let dimension_label = dimension_label(name);
    let production_trigger = if in_production_today {
        "changes touching live users, data, or uptime"
    } else {
        "changes that create irreversible migration or trust-boundary cost"
    };

    handbook_engine::CharterDimensionInput {
        name,
        level: Some(baseline_level),
        default_stance: format!(
            "{project_name} defaults to level {baseline_level} on {dimension_label}; raise the bar whenever blast radius, trust boundaries, or recovery cost increases."
        ),
        raise_the_bar_triggers: vec![
            production_trigger.to_string(),
            "new external interfaces or contracts".to_string(),
        ],
        allowed_shortcuts: vec![
            "time-boxed exploration before merge".to_string(),
            "fixture-backed or local-only iteration with explicit follow-up".to_string(),
        ],
        red_lines: vec![
            format!("do not waive {dimension_label} expectations on shipped work"),
            "do not hide known risk without recording an exception".to_string(),
        ],
        domain_overrides: Vec::new(),
    }
}

fn dimension_label(name: handbook_engine::CharterDimensionName) -> &'static str {
    match name {
        handbook_engine::CharterDimensionName::SpeedVsQuality => "speed vs quality",
        handbook_engine::CharterDimensionName::TypeSafetyStaticAnalysis => {
            "type safety and static analysis"
        }
        handbook_engine::CharterDimensionName::TestingRigor => "testing rigor",
        handbook_engine::CharterDimensionName::ScalabilityPerformance => {
            "scalability and performance"
        }
        handbook_engine::CharterDimensionName::ReliabilityOperability => {
            "reliability and operability"
        }
        handbook_engine::CharterDimensionName::SecurityPrivacy => "security and privacy",
        handbook_engine::CharterDimensionName::Observability => "observability",
        handbook_engine::CharterDimensionName::DxToolingAutomation => {
            "developer tooling and automation"
        }
        handbook_engine::CharterDimensionName::UxPolishApiUsability => {
            "ux polish and api usability"
        }
    }
}

fn guided_prompt_answers() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Project name:", "Handbook"),
        (
            "Project classification [greenfield|brownfield|integration|modernization|hardening]:",
            "greenfield",
        ),
        ("Team size (> 0):", "2"),
        ("Audience [internal|external|mixed]:", "internal"),
        ("Expected lifetime [days|weeks|months|years]:", "months"),
        ("Surfaces [web_app, api, cli, lib, infra, ml]:", "cli, api"),
        (
            "Runtime environments [browser, server, cloud, on_prem, edge]:",
            "server",
        ),
        ("Deadline or delivery window:", ""),
        ("Budget notes:", ""),
        ("Experience notes:", "small team"),
        ("Must-use tech (comma-separated, optional):", "rust"),
        ("In production today? [yes|no]:", "no"),
        ("Production users or data notes:", ""),
        (
            "External contracts to preserve (comma-separated, optional):",
            "",
        ),
        ("Uptime expectations:", "best effort"),
        ("Baseline rubric level [1-5]:", "3"),
        (
            "Baseline rationale (comma-separated, at least one):",
            "internal operators, moderate blast radius",
        ),
        (
            "Backward compatibility [required|not_required|boundary_only]:",
            "not_required",
        ),
        (
            "Migration planning [required|not_required]:",
            "not_required",
        ),
        (
            "Rollout controls [none|lightweight|required]:",
            "lightweight",
        ),
        (
            "Deprecation policy [required|not_required_yet]:",
            "not_required_yet",
        ),
        (
            "Observability threshold [minimal|standard|high|regulated]:",
            "standard",
        ),
        ("Primary domain name (optional):", "planning"),
        ("Primary domain blast radius:", "medium"),
        (
            "Primary domain touches (comma-separated, optional):",
            "internal operators",
        ),
        (
            "Primary domain constraints (comma-separated, optional):",
            "preserve trust boundaries",
        ),
        ("Keep baseline for speed vs quality? [yes|no] [yes]:", "no"),
        ("speed vs quality level [1-5] [3]:", "4"),
        ("speed vs quality default stance [", "favor durable launches over rush delivery"),
        (
            "speed vs quality raise-the-bar triggers (comma-separated) [",
            "changes that affect onboarding conversion, irreversible rollout steps",
        ),
        (
            "speed vs quality allowed shortcuts (comma-separated) [",
            "time-boxed prototypes behind a feature flag, paired operator review for urgent copy changes",
        ),
        (
            "speed vs quality red lines (comma-separated) [",
            "do not skip launch rollback planning, do not trade away review on shipped flows",
        ),
        (
            "speed vs quality domain overrides (comma-separated, optional) [none]:",
            "billing changes stay at level 5 until two successful dry runs",
        ),
        (
            "Keep baseline for type safety and static analysis? [yes|no] [yes]:",
            "",
        ),
        ("Keep baseline for testing rigor? [yes|no] [yes]:", ""),
        (
            "Keep baseline for scalability and performance? [yes|no] [yes]:",
            "",
        ),
        (
            "Keep baseline for reliability and operability? [yes|no] [yes]:",
            "",
        ),
        ("Keep baseline for security and privacy? [yes|no] [yes]:", ""),
        ("Keep baseline for observability? [yes|no] [yes]:", ""),
        (
            "Keep baseline for developer tooling and automation? [yes|no] [yes]:",
            "",
        ),
        (
            "Keep baseline for ux polish and api usability? [yes|no] [yes]:",
            "",
        ),
        (
            "Exception approvers (comma-separated, at least one):",
            "project_owner",
        ),
        (
            "Exception record location [.handbook/charter/CHARTER.md#exceptions]:",
            "",
        ),
        (
            "Exception minimum fields (comma-separated; press enter for standard fields):",
            "",
        ),
        ("Debt tracking system:", "issues"),
        ("Debt tracking labels (comma-separated, optional):", "debt"),
        ("Debt tracking review cadence:", "monthly"),
        ("Decision records enabled? [yes|no]:", "yes"),
        ("Decision records path:", "docs/decisions"),
        ("Decision records format:", "md"),
    ]
}

fn wait_for_transcript(transcript: &Arc<Mutex<String>>, needle: &str, timeout: Duration) {
    let deadline = Instant::now() + timeout;
    loop {
        let snapshot = transcript.lock().expect("transcript").clone();
        if snapshot.contains(needle) {
            return;
        }
        if Instant::now() >= deadline {
            panic!("timed out waiting for `{needle}` in transcript:\n{snapshot}");
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn run_guided_author_under_pty(dir: &Path) -> (String, portable_pty::ExitStatus) {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 48,
            cols: 120,
            pixel_width: 0,
            pixel_height: 0,
        })
        .expect("open pty");

    let mut command = CommandBuilder::new(env!("CARGO_BIN_EXE_handbook"));
    command.cwd(dir);
    command.arg("author");
    command.arg("charter");

    let mut child = pair
        .slave
        .spawn_command(command)
        .expect("spawn author in pty");
    drop(pair.slave);

    let mut reader = pair.master.try_clone_reader().expect("clone pty reader");
    let mut writer = pair.master.take_writer().expect("take pty writer");

    let transcript = Arc::new(Mutex::new(String::new()));
    let transcript_reader = Arc::clone(&transcript);
    let reader_thread = thread::spawn(move || {
        let mut buffer = [0u8; 4096];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(read) => {
                    let text = String::from_utf8_lossy(&buffer[..read]);
                    transcript_reader
                        .lock()
                        .expect("transcript")
                        .push_str(&text);
                }
                Err(err) if err.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(err) => panic!("read pty output: {err}"),
            }
        }
    });

    wait_for_transcript(
        &transcript,
        "Guided charter interview",
        Duration::from_secs(5),
    );
    for (prompt, answer) in guided_prompt_answers() {
        wait_for_transcript(&transcript, prompt, Duration::from_secs(5));
        writer
            .write_all(answer.as_bytes())
            .unwrap_or_else(|err| panic!("write answer for `{prompt}`: {err}"));
        writer
            .write_all(b"\n")
            .unwrap_or_else(|err| panic!("write newline for `{prompt}`: {err}"));
        writer
            .flush()
            .unwrap_or_else(|err| panic!("flush answer for `{prompt}`: {err}"));
    }
    wait_for_transcript(&transcript, "OUTCOME: AUTHORED", Duration::from_secs(10));
    drop(writer);

    let status = child.wait().expect("wait for author");
    drop(pair.master);
    reader_thread.join().expect("reader thread");
    let output = transcript.lock().expect("transcript").clone();
    (output, status)
}

fn guided_project_context_prompt_answers() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Project name [", ""),
        ("Owner:", "compiler-team"),
        ("Team:", "Handbook"),
        ("Repo / project reference [", ""),
        ("Charter ref [", ""),
        (
            "What this project is:",
            "CLI and compiler for canonical planning artifacts and workflow proofs",
        ),
        ("Primary surface:", "CLI plus compiler library"),
        ("Primary users:", "internal operators and automation"),
        (
            "Key workflows (comma-separated, 1-3):",
            "scaffold canonical .handbook state, author baseline artifacts, compile and inspect planning outputs",
        ),
        ("Non-goals (optional):", "End-user product delivery"),
        ("Is anything live in production today?:", "no"),
        ("Users:", "internal operators only"),
        ("Data in production:", "none"),
        (
            "Uptime expectations / SLA:",
            "best effort during active development",
        ),
        ("Incident / on-call reality:", "no formal on-call rotation today"),
        (
            "Primary risk flags present:",
            "incorrect planning guidance and canonical write regressions",
        ),
        (
            "Project type:",
            "greenfield with an active brownfield codebase",
        ),
        ("Backward compatibility required?:", "no"),
        (
            "Backward compatibility notes:",
            "no external customers depend on the current compiler API",
        ),
        ("Migration planning required?:", "not applicable"),
        (
            "Migration planning notes:",
            "no legacy production data to migrate",
        ),
        ("Deprecation policy exists?:", "not yet"),
        (
            "Deprecation policy notes:",
            "internal interfaces can change with coordinated release notes",
        ),
        ("Rollout controls required?:", "lightweight only"),
        (
            "Rollout controls notes:",
            "feature branches and tests gate changes before merge",
        ),
        (
            "Owned areas (comma-separated):",
            "compiler and CLI crates in this repository, canonical .handbook artifact formats and setup flow",
        ),
        (
            "External dependencies (comma-separated):",
            "OpenAI Codex runtime used for charter synthesis, local filesystem layout and git worktree state",
        ),
        ("Integration count [0-5] [0]:", ""),
        ("Environments that exist:", "local development and CI"),
        ("Deployment model:", "cargo-driven local execution"),
        ("CI/CD reality:", "basic CI with compiler and CLI test coverage"),
        ("Release cadence:", "repo-driven iterative releases"),
        (
            "Config & secrets:",
            "standard local environment variables and git config",
        ),
        ("Observability stack:", "test output and local command stderr"),
        (
            "Primary data stores:",
            "repo-local markdown, yaml, and route-state files",
        ),
        ("Data classification:", "source code and internal planning metadata"),
        ("Retention requirements:", "none beyond repository history"),
        (
            "Backups / DR reality:",
            "git history plus local worktree backups",
        ),
        ("Existing migrations / history:", "none for production data"),
        ("Codebase exists today? [yes|no] [yes]:", ""),
        ("Current maturity:", "medium-sized active Rust workspace"),
        (
            "Key modules / areas to be aware of (comma-separated, optional):",
            "crates/compiler, crates/cli, core/library",
        ),
        (
            "Known constraints from existing code:",
            "lane ownership and canonical artifact ordering must be preserved",
        ),
        ("Deadline / time constraints:", "must fit the current milestone split"),
        ("Budget constraints:", "limited to local engineering time"),
        (
            "Must-use / prohibited tech:",
            "must stay in Rust and preserve existing canonical paths",
        ),
        ("Compliance / legal constraints:", "none beyond repository policy"),
        (
            "Performance constraints:",
            "compiler authoring should stay fast and deterministic",
        ),
        ("Security constraints:", "no writes outside canonical repo-owned targets"),
        ("Known unknown count [1-5] [1]:", ""),
        (
            "Known unknown 1 item:",
            "final CLI interview wording for project-context authoring",
        ),
        ("Known unknown 1 owner:", "Lane D"),
        ("Known unknown 1 revisit trigger:", "when the CLI subcommand lands"),
    ]
}

fn run_guided_project_context_under_pty_with_interaction(
    dir: &Path,
    interaction: impl FnOnce(&Arc<Mutex<String>>, &mut dyn Write),
) -> (String, portable_pty::ExitStatus) {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 48,
            cols: 120,
            pixel_width: 0,
            pixel_height: 0,
        })
        .expect("open pty");

    let mut command = CommandBuilder::new(env!("CARGO_BIN_EXE_handbook"));
    command.cwd(dir);
    command.arg("author");
    command.arg("project-context");

    let mut child = pair
        .slave
        .spawn_command(command)
        .expect("spawn author in pty");
    drop(pair.slave);

    let mut reader = pair.master.try_clone_reader().expect("clone pty reader");
    let mut writer = pair.master.take_writer().expect("take pty writer");

    let transcript = Arc::new(Mutex::new(String::new()));
    let transcript_reader = Arc::clone(&transcript);
    let reader_thread = thread::spawn(move || {
        let mut buffer = [0u8; 4096];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(read) => {
                    let text = String::from_utf8_lossy(&buffer[..read]);
                    transcript_reader
                        .lock()
                        .expect("transcript")
                        .push_str(&text);
                }
                Err(err) if err.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(err) => panic!("read pty output: {err}"),
            }
        }
    });

    wait_for_transcript(
        &transcript,
        "Guided project-context interview",
        Duration::from_secs(5),
    );
    interaction(&transcript, &mut writer);
    drop(writer);

    let status = child.wait().expect("wait for author");
    drop(pair.master);
    reader_thread.join().expect("reader thread");
    let output = transcript.lock().expect("transcript").clone();
    (output, status)
}

fn run_guided_project_context_under_pty(dir: &Path) -> (String, portable_pty::ExitStatus) {
    run_guided_project_context_under_pty_with_interaction(dir, |transcript, writer| {
        for (prompt, answer) in guided_project_context_prompt_answers() {
            wait_for_transcript(transcript, prompt, Duration::from_secs(5));
            writer
                .write_all(answer.as_bytes())
                .unwrap_or_else(|err| panic!("write answer for `{prompt}`: {err}"));
            writer
                .write_all(b"\n")
                .unwrap_or_else(|err| panic!("write newline for `{prompt}`: {err}"));
            writer
                .flush()
                .unwrap_or_else(|err| panic!("flush answer for `{prompt}`: {err}"));
        }
        wait_for_transcript(transcript, "OUTCOME: AUTHORED", Duration::from_secs(10));
    })
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
        &stubbed_authored_markdown(),
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
    let stub = install_stub_codex(dir.path(), &failing_stub_script());

    let output = with_author_runtime_override(&stub, None, || {
        run_in(
            dir.path(),
            &[
                "author",
                "charter",
                "--from-inputs",
                inputs_path.to_str().expect("utf-8 path"),
            ],
        )
    });

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
    assert!(!prompt_capture_path(dir.path()).exists());
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
    let stub = install_stub_codex(dir.path(), &failing_stub_script());

    let output = with_author_runtime_override(&stub, None, || {
        run_in(
            dir.path(),
            &[
                "author",
                "charter",
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
        fs::read_to_string(dir.path().join(".handbook/charter/CHARTER.md")).expect("charter"),
        expected_markdown
    );
}

#[test]
fn file_inputs_author_charter_ignores_runtime_model_override_for_deterministic_path() {
    let dir = scaffold_repo();
    let inputs_path = dir.path().join("charter-inputs-model.yaml");
    write_file(&inputs_path, valid_structured_inputs_yaml());
    let expected_markdown = deterministic_authored_markdown();
    let stub = install_stub_codex(dir.path(), &failing_stub_script());

    let output = with_author_runtime_override(&stub, Some("gpt-5.4-mini"), || {
        run_in(
            dir.path(),
            &[
                "author",
                "charter",
                "--from-inputs",
                inputs_path.to_str().expect("utf-8 path"),
            ],
        )
    });

    assert!(
        output.status.success(),
        "file inputs with model override should succeed: {}",
        stdout(&output)
    );
    assert_eq!(
        fs::read_to_string(dir.path().join(".handbook/charter/CHARTER.md")).expect("charter"),
        expected_markdown
    );
    assert!(!prompt_capture_path(dir.path()).exists());
}

#[test]
fn stdin_inputs_author_charter_successfully_with_deterministic_rendering() {
    let dir = scaffold_repo();
    let expected_markdown = deterministic_authored_markdown();
    let stub = install_stub_codex(dir.path(), &failing_stub_script());

    let output = with_author_runtime_override(&stub, None, || {
        run_in_with_input(
            dir.path(),
            &["author", "charter", "--from-inputs", "-"],
            valid_structured_inputs_yaml(),
        )
    });

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
    assert!(!prompt_capture_path(dir.path()).exists());
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
    let stub = install_stub_codex(dir.path(), &failing_stub_script());

    let output = with_author_runtime_override(&stub, None, || {
        run_in(
            dir.path(),
            &[
                "author",
                "charter",
                "--validate",
                "--from-inputs",
                inputs_path.to_str().expect("utf-8 path"),
            ],
        )
    });

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
    assert!(!prompt_capture_path(dir.path()).exists());
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
#[ignore = "guided charter path removed; PTY harness is deleted in P3.3"]
fn guided_tty_author_charter_succeeds_via_real_binary_path() {
    let dir = scaffold_repo();
    let expected_markdown = stubbed_authored_markdown();
    let stub = install_stub_codex(dir.path(), &successful_stub_script(&expected_markdown));

    let (output, status) =
        with_author_runtime_override(&stub, None, || run_guided_author_under_pty(dir.path()));

    assert!(
        status.success(),
        "guided PTY author should succeed: {output}"
    );
    assert!(output.contains("Guided charter interview"), "{output}");
    assert!(output.contains("Project name:"), "{output}");
    assert!(
        output.contains("Keep baseline for speed vs quality?"),
        "{output}"
    );
    assert!(output.contains("Decision records format:"), "{output}");
    assert!(output.contains("OUTCOME: AUTHORED"), "{output}");
    assert!(output.contains("MODE: guided_interview"), "{output}");
    let charter =
        fs::read_to_string(dir.path().join(".handbook/charter/CHARTER.md")).expect("charter");
    assert_eq!(charter, expected_markdown);
    assert!(charter.contains("favor durable launches over rush delivery"));
    assert!(charter.contains(handbook_engine::DEFAULT_EXCEPTION_RECORD_LOCATION));
    assert!(!charter.contains("`CHARTER.md#exceptions`"));
    assert!(prompt_capture_path(dir.path()).exists());
}

#[test]
#[ignore = "guided charter path removed; PTY harness is deleted in P3.3"]
fn guided_tty_author_charter_unblocks_doctor_and_generate() {
    let dir = scaffold_repo();
    let expected_markdown = stubbed_authored_markdown();
    let stub = install_stub_codex(dir.path(), &successful_stub_script(&expected_markdown));

    let (author_output, status) =
        with_author_runtime_override(&stub, None, || run_guided_author_under_pty(dir.path()));
    assert!(
        status.success(),
        "guided PTY author should succeed: {author_output}"
    );

    let doctor = run_in(dir.path(), &["doctor"]);
    assert!(
        !doctor.status.success(),
        "doctor should remain incomplete after charter-only authoring: {}",
        stdout(&doctor)
    );
    let doctor_stdout = stdout(&doctor);
    assert!(
        doctor_stdout.contains("PARTIAL_BASELINE"),
        "{doctor_stdout}"
    );
    assert!(doctor_stdout.contains("ROOT STATUS: OK"), "{doctor_stdout}");
    assert!(
        doctor_stdout.contains("NEXT SAFE ACTION: run `handbook author project-context`"),
        "{doctor_stdout}"
    );
    assert!(
        doctor_stdout.contains(
            "CHARTER [.handbook/charter/CHARTER.md] STATUS: VALID_CANONICAL_TRUTH ACTION: run `handbook author charter`"
        ),
        "{doctor_stdout}"
    );
    assert!(
        doctor_stdout.contains(
            "PROJECT_CONTEXT [.handbook/project_context/PROJECT_CONTEXT.md] STATUS: STARTER_OWNED ACTION: run `handbook author project-context`"
        ),
        "{doctor_stdout}"
    );
    assert!(
        doctor_stdout.contains(
            "ENVIRONMENT_INVENTORY [.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md] STATUS: STARTER_OWNED ACTION: run `handbook author environment-inventory`"
        ),
        "{doctor_stdout}"
    );

    let generate = run_in(dir.path(), &["generate"]);
    assert!(
        generate.status.success(),
        "generate should succeed after authoring: {}",
        stdout(&generate)
    );
    let generate_stdout = stdout(&generate);
    assert!(
        generate_stdout.contains("OUTCOME: READY"),
        "{generate_stdout}"
    );
    assert!(
        generate_stdout.contains("### CHARTER (.handbook/charter/CHARTER.md)"),
        "{generate_stdout}"
    );
    assert!(
        generate_stdout.contains("# Engineering Charter — Handbook"),
        "{generate_stdout}"
    );
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
#[ignore = "guided project-context path removed; PTY harness is deleted in P3.3"]
fn guided_tty_author_project_context_succeeds() {
    let dir = scaffold_repo();

    let (output, status) = with_project_context_now_utc("2026-04-21T12:34:56Z", || {
        run_guided_project_context_under_pty(dir.path())
    });

    assert!(
        status.success(),
        "guided PTY project-context author should succeed: {output}"
    );
    assert!(
        output.contains("Guided project-context interview"),
        "{output}"
    );
    assert!(output.contains("What this project is:"), "{output}");
    assert!(
        output.contains("Known unknown 1 revisit trigger:"),
        "{output}"
    );
    assert!(output.contains("OUTCOME: AUTHORED"), "{output}");
    assert!(output.contains("MODE: guided_interview"), "{output}");

    let project_context = fs::read_to_string(
        dir.path()
            .join(".handbook/project_context/PROJECT_CONTEXT.md"),
    )
    .expect("project context");
    assert!(project_context.contains("> **Owner:** compiler-team"));
    assert!(project_context.contains("> **Team:** Handbook"));
    assert!(project_context.contains("CLI and compiler for canonical planning artifacts"));
    assert!(project_context.contains("final CLI interview wording for project-context authoring"));
}

#[test]
#[ignore = "guided project-context path removed; PTY harness is deleted in P3.3"]
fn guided_tty_author_project_context_eof_refusal_names_project_context_command() {
    let dir = scaffold_repo();

    let (output, status) = with_project_context_now_utc("2026-04-21T12:34:56Z", || {
        run_guided_project_context_under_pty_with_interaction(dir.path(), |transcript, _writer| {
            wait_for_transcript(transcript, "Project name [", Duration::from_secs(5));
        })
    });

    assert!(
        !status.success(),
        "guided PTY project-context EOF should refuse: {output}"
    );
    assert!(output.contains("OUTCOME: REFUSED"), "{output}");
    assert!(
        output.contains("OBJECT: author project-context"),
        "{output}"
    );
    assert!(output.contains("CATEGORY: InterviewIncomplete"), "{output}");
    assert!(
        output.contains("restart `handbook author project-context` or use `handbook author project-context --from-inputs <path|->`"),
        "{output}"
    );
    assert!(
        output.contains(
            "guided project-context interview ended before all required answers were collected"
        ),
        "{output}"
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
    let stub = install_stub_codex(dir.path(), &failing_stub_script());

    let output = with_environment_inventory_runtime_override(&stub, None, || {
        run_in(
            dir.path(),
            &[
                "author",
                "environment-inventory",
                "--from-inputs",
                inputs_path.to_str().expect("utf8 inputs path"),
            ],
        )
    });

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
    assert!(!prompt_capture_path(dir.path()).exists());
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
    assert!(!prompt_capture_path(dir.path()).exists());
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
    assert!(!prompt_capture_path(dir.path()).exists());
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
    let stub = install_stub_codex(dir.path(), &failing_stub_script());

    let output = with_environment_inventory_runtime_override(&stub, None, || {
        run_in(dir.path(), &["author", "environment-inventory"])
    });

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
    assert!(!prompt_capture_path(dir.path()).exists());
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
    let stub = install_stub_codex(dir.path(), &failing_stub_script());

    let output = with_environment_inventory_runtime_override(&stub, None, || {
        run_in(
            dir.path(),
            &[
                "author",
                "environment-inventory",
                "--from-inputs",
                inputs_path.to_str().expect("utf8 inputs path"),
            ],
        )
    });

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
    assert!(!prompt_capture_path(dir.path()).exists());
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

#[test]
fn structured_inputs_author_charter_succeeds_with_live_codex_transport() {
    if std::env::var("HANDBOOK_RUN_LIVE_AUTHOR_CHARTER_SMOKE")
        .ok()
        .as_deref()
        != Some("1")
    {
        eprintln!("skipping live Codex smoke; set HANDBOOK_RUN_LIVE_AUTHOR_CHARTER_SMOKE=1");
        return;
    }

    let dir = scaffold_repo();
    let inputs_path = dir.path().join("charter-inputs-live.yaml");
    write_file(&inputs_path, valid_structured_inputs_yaml());

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
        "live Codex authoring should succeed: {}",
        stdout(&output)
    );

    let charter =
        fs::read_to_string(dir.path().join(".handbook/charter/CHARTER.md")).expect("charter");
    assert!(charter.starts_with("# Engineering Charter — Handbook"));
    for heading in [
        "## What this is",
        "## Dimensions (details + guardrails)",
        "## Exceptions / overrides process",
        "## Review & updates",
    ] {
        assert!(
            charter.contains(heading),
            "missing heading `{heading}` in:\n{charter}"
        );
    }
    assert!(charter.contains(handbook_engine::DEFAULT_EXCEPTION_RECORD_LOCATION));
    assert!(!charter.contains("`CHARTER.md#exceptions`"));
    assert!(!dir.path().join("artifacts/charter/CHARTER.md").exists());
    assert!(!dir.path().join("CHARTER.md").exists());
}
