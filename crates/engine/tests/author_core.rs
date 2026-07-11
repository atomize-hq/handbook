use handbook_engine::{
    parse_environment_inventory_structured_input_yaml, render_charter_markdown,
    render_environment_inventory_markdown, render_project_context_markdown,
    validate_charter_structured_input, validate_environment_inventory_markdown,
    validate_project_context_markdown, validate_project_context_structured_input, CharterAudience,
    CharterBackwardCompatibility, CharterDebtTrackingInput, CharterDecisionRecordsInput,
    CharterDefaultImplicationsInput, CharterDeprecationPolicy, CharterDimensionInput,
    CharterDimensionName, CharterDomainInput, CharterExceptionsInput, CharterExpectedLifetime,
    CharterObservabilityThreshold, CharterOperationalRealityInput, CharterPostureInput,
    CharterProjectClassification, CharterProjectConstraintsInput, CharterProjectInput,
    CharterRequiredness, CharterRolloutControls, CharterRuntimeEnvironment, CharterStructuredInput,
    CharterSurface, EnvironmentInventoryCoreErrorKind,
    ProjectContextClassificationImplicationsInput, ProjectContextConstraintsInput,
    ProjectContextDataRealityInput, ProjectContextEnvironmentsAndDeliveryInput,
    ProjectContextIntegrationInput, ProjectContextKnownUnknownInput,
    ProjectContextOperationalRealityInput, ProjectContextRepoCodebaseRealityInput,
    ProjectContextStructuredInput, ProjectContextSummaryInput, ProjectContextSystemBoundariesInput,
    DEFAULT_EXCEPTION_RECORD_LOCATION,
};

const VALID_ENVIRONMENT_INVENTORY_INPUTS: &str = r#"
schema_version: "0.1.0"
project_name: "Handbook"
owner: "compiler-team"
team: "Handbook"
repo_or_project_ref: "handbook"
charter_ref: ".handbook/charter/CHARTER.md"
project_context_ref: ".handbook/project_context/PROJECT_CONTEXT.md"
environment_variables:
  - name: "HANDBOOK_HOME"
    required: false
    secret: false
    default_or_example: "$HOME/handbook"
    validation_notes: "must point to an installed Handbook home"
    where_used: "install and skill smoke scripts"
    source_of_truth: "operator environment"
  - name: "OPENAI_API_KEY"
    required: false
    secret: true
    default_or_example: "None"
    validation_notes: "not used by deterministic authoring"
    where_used: "legacy paths only until removal"
    source_of_truth: "operator secret store"
secret_handling:
  charter_posture: "never store real credentials in repository artifacts"
  storage_locations:
    - "operator secret store"
  rotation_expectations: "follow the owning provider policy"
external_services:
  - name: "GitHub"
    required: false
    environments:
      - "development"
      - "CI"
    purpose: "source hosting and release automation"
    connection_details: "git and GitHub Actions"
    notes: "not required for offline authoring"
runtime_assumptions:
  listening_ports: "None"
  filesystem_requirements: "write access to the managed repository"
  persistent_storage: "repository-local canonical artifacts"
  network_assumptions: "Unknown for future hosted use; offline authoring requires none"
  performance_budgets: "authoring should complete within normal CLI latency"
local_development:
  prerequisites:
    - "Rust stable toolchain"
  works_on_my_machine_prevention: "run workspace tests and install smoke"
  environment_file_pattern: "None"
ci:
  system: "GitHub Actions"
  required_secret_names:
    - "None"
  services:
    - "None"
  artifacts:
    - "test output"
production:
  exists_today: false
  hosting_model: "Not applicable"
  runtime_environments:
    - "local CLI"
  required_secret_names:
    - "None"
  observability: "command output and CI logs"
  backup_and_disaster_recovery: "git history"
tooling:
  primary_language_runtime: "Rust stable"
  package_manager_build_system: "Cargo"
  lockfiles:
    - "Cargo.lock"
  lint_type_test_tools:
    - "rustfmt"
    - "clippy"
    - "cargo test"
  minimum_versions:
    - "Rust 2021 edition"
update_contract:
  exception_record_location: ".handbook/charter/CHARTER.md#exceptions"
known_unknowns:
  - item: "future hosted runtime requirements"
    owner: "project owner"
    revisit_trigger: "before adding a hosted deployment"
"#;

fn valid_charter_input() -> CharterStructuredInput {
    CharterStructuredInput {
        schema_version: "0.1.0".to_string(),
        project: CharterProjectInput {
            name: "Handbook".to_string(),
            classification: CharterProjectClassification::Greenfield,
            team_size: 2,
            users: CharterAudience::Internal,
            expected_lifetime: CharterExpectedLifetime::Months,
            surfaces: vec![CharterSurface::Cli, CharterSurface::Api],
            runtime_environments: vec![CharterRuntimeEnvironment::Server],
            constraints: CharterProjectConstraintsInput {
                deadline: "".to_string(),
                budget: "".to_string(),
                experience_notes: "small team".to_string(),
                must_use_tech: vec!["rust".to_string()],
            },
            operational_reality: CharterOperationalRealityInput {
                in_production_today: false,
                prod_users_or_data: "".to_string(),
                external_contracts_to_preserve: Vec::new(),
                uptime_expectations: "".to_string(),
            },
            default_implications: CharterDefaultImplicationsInput {
                backward_compatibility: CharterBackwardCompatibility::NotRequired,
                migration_planning: CharterRequiredness::NotRequired,
                rollout_controls: CharterRolloutControls::Lightweight,
                deprecation_policy: CharterDeprecationPolicy::NotRequiredYet,
                observability_threshold: CharterObservabilityThreshold::Standard,
            },
        },
        posture: CharterPostureInput {
            rubric_scale: "1-5".to_string(),
            baseline_level: 3,
            baseline_rationale: vec![
                "internal operators".to_string(),
                "moderate blast radius".to_string(),
            ],
        },
        domains: vec![CharterDomainInput {
            name: "planning".to_string(),
            blast_radius: "medium".to_string(),
            touches: vec!["internal".to_string()],
            constraints: vec!["preserve trust product boundaries".to_string()],
        }],
        dimensions: vec![
            charter_dimension(CharterDimensionName::SpeedVsQuality),
            charter_dimension(CharterDimensionName::TypeSafetyStaticAnalysis),
            charter_dimension(CharterDimensionName::TestingRigor),
            charter_dimension(CharterDimensionName::ScalabilityPerformance),
            charter_dimension(CharterDimensionName::ReliabilityOperability),
            charter_dimension(CharterDimensionName::SecurityPrivacy),
            charter_dimension(CharterDimensionName::Observability),
            charter_dimension(CharterDimensionName::DxToolingAutomation),
            charter_dimension(CharterDimensionName::UxPolishApiUsability),
        ],
        exceptions: CharterExceptionsInput {
            approvers: vec!["project_owner".to_string()],
            record_location: DEFAULT_EXCEPTION_RECORD_LOCATION.to_string(),
            minimum_fields: vec![
                "what".to_string(),
                "why".to_string(),
                "scope".to_string(),
                "risk".to_string(),
                "owner".to_string(),
                "expiry_or_revisit_date".to_string(),
            ],
        },
        debt_tracking: CharterDebtTrackingInput {
            system: "issues".to_string(),
            labels: vec!["debt".to_string()],
            review_cadence: "monthly".to_string(),
        },
        decision_records: CharterDecisionRecordsInput {
            enabled: true,
            path: "docs/decisions".to_string(),
            format: "md".to_string(),
        },
    }
}

fn charter_dimension(name: CharterDimensionName) -> CharterDimensionInput {
    CharterDimensionInput {
        name,
        level: Some(3),
        default_stance: format!("default stance for {:?}", name),
        raise_the_bar_triggers: vec!["production data".to_string()],
        allowed_shortcuts: vec!["throwaway exploration".to_string()],
        red_lines: vec!["ship without review".to_string()],
        domain_overrides: Vec::new(),
    }
}

fn valid_project_context_input() -> ProjectContextStructuredInput {
    ProjectContextStructuredInput {
        schema_version: "0.1.0".to_string(),
        project_name: "Handbook".to_string(),
        owner: "compiler-team".to_string(),
        team: "Handbook".to_string(),
        repo_or_project_ref: "handbook".to_string(),
        charter_ref: ".handbook/charter/CHARTER.md".to_string(),
        project_summary: ProjectContextSummaryInput {
            what_this_project_is:
                "CLI and compiler for canonical planning artifacts and workflow proofs".to_string(),
            primary_surface: "CLI plus compiler library".to_string(),
            primary_users: "internal operators and automation".to_string(),
            key_workflows: vec![
                "scaffold canonical .handbook state".to_string(),
                "author baseline artifacts".to_string(),
                "compile and inspect planning outputs".to_string(),
            ],
            non_goals: "End-user product delivery".to_string(),
        },
        operational_reality: ProjectContextOperationalRealityInput {
            is_live_in_production_today: "no".to_string(),
            users: "internal operators only".to_string(),
            data_in_production: "none".to_string(),
            uptime_expectations: "best effort during active development".to_string(),
            incident_on_call_reality: "no formal on-call rotation today".to_string(),
            primary_risk_flags_present:
                "incorrect planning guidance and canonical write regressions".to_string(),
        },
        classification_implications: ProjectContextClassificationImplicationsInput {
            project_type: "greenfield with an active brownfield codebase".to_string(),
            backward_compatibility_required: "no".to_string(),
            backward_compatibility_notes:
                "no external customers depend on the current compiler API".to_string(),
            migration_planning_required: "not applicable".to_string(),
            migration_planning_notes: "no legacy production data to migrate".to_string(),
            deprecation_policy_exists: "not yet".to_string(),
            deprecation_policy_notes:
                "internal interfaces can change with coordinated release notes".to_string(),
            rollout_controls_required: "lightweight only".to_string(),
            rollout_controls_notes: "feature branches and tests gate changes before merge"
                .to_string(),
        },
        system_boundaries: ProjectContextSystemBoundariesInput {
            owned_areas: vec![
                "compiler and CLI crates in this repository".to_string(),
                "canonical .handbook artifact formats and setup flow".to_string(),
            ],
            external_dependencies: vec![
                "OpenAI Codex runtime used for charter synthesis".to_string(),
                "local filesystem layout and git worktree state".to_string(),
            ],
        },
        integrations: vec![
            ProjectContextIntegrationInput {
                name: "Codex exec".to_string(),
                integration_type: "CLI runtime".to_string(),
                contract_surface: "codex exec --output-last-message -".to_string(),
                authentication_authorization:
                    "inherits local operator credentials and API configuration".to_string(),
                failure_mode_expectations:
                    "auth or process failures must refuse without partial writes".to_string(),
            },
            ProjectContextIntegrationInput {
                name: "Repo-local .handbook tree".to_string(),
                integration_type: "filesystem".to_string(),
                contract_surface: "canonical artifact paths under .handbook/**".to_string(),
                authentication_authorization:
                    "write guards reject symlinks and non-regular targets".to_string(),
                failure_mode_expectations: "invalid paths block authoring until repaired"
                    .to_string(),
            },
        ],
        environments_and_delivery: ProjectContextEnvironmentsAndDeliveryInput {
            environments_that_exist: "local development and CI".to_string(),
            deployment_model: "cargo-driven local execution".to_string(),
            ci_cd_reality: "basic CI with compiler and CLI test coverage".to_string(),
            release_cadence: "repo-driven iterative releases".to_string(),
            config_and_secrets: "standard local environment variables and git config".to_string(),
            observability_stack: "test output and local command stderr".to_string(),
        },
        data_reality: ProjectContextDataRealityInput {
            primary_data_stores: "repo-local markdown, yaml, and route-state files".to_string(),
            data_classification: "source code and internal planning metadata".to_string(),
            retention_requirements: "none beyond repository history".to_string(),
            backups_disaster_recovery: "git history plus local worktree backups".to_string(),
            existing_migrations_history: "none for production data".to_string(),
        },
        repo_codebase_reality: ProjectContextRepoCodebaseRealityInput {
            codebase_exists_today: true,
            current_maturity: "medium-sized active Rust workspace".to_string(),
            key_modules_or_areas: vec![
                "crates/compiler".to_string(),
                "crates/cli".to_string(),
                "core/library".to_string(),
            ],
            known_constraints_from_existing_code:
                "lane ownership and canonical artifact ordering must be preserved".to_string(),
        },
        constraints: ProjectContextConstraintsInput {
            deadline_time_constraints: "must fit the current milestone split".to_string(),
            budget_constraints: "limited to local engineering time".to_string(),
            must_use_or_prohibited_tech: "must stay in Rust and preserve existing canonical paths"
                .to_string(),
            compliance_legal_constraints: "none beyond repository policy".to_string(),
            performance_constraints: "compiler authoring should stay fast and deterministic"
                .to_string(),
            security_constraints: "no writes outside canonical repo-owned targets".to_string(),
        },
        known_unknowns: vec![
            ProjectContextKnownUnknownInput {
                item: "final CLI interview wording for project-context authoring".to_string(),
                owner: "Lane D".to_string(),
                revisit_trigger: "when the CLI subcommand lands".to_string(),
            },
            ProjectContextKnownUnknownInput {
                item: "doctor-side invalid baseline messaging for project context".to_string(),
                owner: "Lane D".to_string(),
                revisit_trigger: "when doctor baseline classification is wired".to_string(),
            },
        ],
    }
}

fn expected_environment_inventory_markdown(project_context_ref: &str) -> String {
    format!(
        "# Environment Inventory - Handbook\n\n> **Canonical File:** `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`\n> **Project Context Ref:** {project_context_ref}\n\n## What this is\nCanonical environment and runtime inventory.\n\n## How to use\n- Update this file when runtime assumptions change.\n\n## 1) Environment Variables (Inventory)\n- None yet.\n\n## 2) External Services / Infrastructure Dependencies\n- None yet.\n\n## 3) Runtime Assumptions (Ports, Paths, Storage, Limits)\n- None yet.\n\n## 4) Local Development Requirements\n- None yet.\n\n## 5) CI Requirements\n- None yet.\n\n## 6) Production / Deployment Requirements (even if not live yet)\n- None yet.\n\n## 7) Dependency & Tooling Inventory (project-specific)\n- None yet.\n\n## 8) Update Contract (non-negotiable)\n- Update `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md` in the same change.\n\n## 9) Known Unknowns\n- None yet.\n"
    )
    .trim_end()
    .to_string()
}

#[test]
fn charter_core_renders_required_sections_from_engine() {
    let input = valid_charter_input();
    validate_charter_structured_input(&input).expect("valid charter input");

    let markdown = render_charter_markdown(&input).expect("render charter markdown");
    assert!(markdown.starts_with("# Engineering Charter — Handbook"));
    assert!(markdown.contains("## Dimensions (details + guardrails)"));
    assert!(markdown.contains("## Review & updates"));
}

#[test]
fn project_context_core_renders_with_explicit_timestamp_from_engine() {
    let input = valid_project_context_input();
    validate_project_context_structured_input(&input).expect("valid project-context input");

    let markdown = render_project_context_markdown(&input, "2026-04-21T12:34:56Z")
        .expect("render project-context markdown");
    assert!(markdown.contains("> **Created (UTC):** 2026-04-21T12:34:56Z"));
    assert!(markdown.contains("## 3) System Boundaries (what we own vs integrate with)"));
    validate_project_context_markdown(&markdown).expect("rendered markdown should validate");
}

#[test]
fn environment_inventory_validation_is_engine_owned() {
    validate_environment_inventory_markdown(&expected_environment_inventory_markdown("None"))
        .expect("canonical environment inventory markdown should validate");
}

#[test]
fn handbook_product_exception_record_default_remains_explicitly_bounded() {
    assert_eq!(
        DEFAULT_EXCEPTION_RECORD_LOCATION,
        ".handbook/charter/CHARTER.md#exceptions"
    );
}

#[allow(deprecated)]
#[test]
fn deprecated_environment_inventory_validation_api_remains_source_compatible() {
    let with_project_context =
        expected_environment_inventory_markdown("`.handbook/project_context/PROJECT_CONTEXT.md`");
    handbook_engine::author::validate_synthesized_environment_inventory_markdown(
        &with_project_context,
        handbook_engine::author::EnvironmentInventoryValidationExpectations::for_optional_project_context(
            true,
        ),
    )
    .expect("compatibility validator should accept the expected project-context reference");

    let error = handbook_engine::author::validate_synthesized_environment_inventory_markdown(
        &with_project_context,
        handbook_engine::author::EnvironmentInventoryValidationExpectations::for_optional_project_context(
            false,
        ),
    )
    .expect_err("compatibility validator should preserve reference-expectation behavior");
    assert!(error.contains("exact project context reference line"));
}

#[test]
fn environment_inventory_core_parses_and_renders_deterministically() {
    let input =
        parse_environment_inventory_structured_input_yaml(VALID_ENVIRONMENT_INVENTORY_INPUTS)
            .expect("valid environment-inventory inputs");

    let first = render_environment_inventory_markdown(&input, "2026-07-10T12:34:56Z")
        .expect("render environment inventory");
    let second = render_environment_inventory_markdown(&input, "2026-07-10T12:34:56Z")
        .expect("render environment inventory again");

    assert_eq!(first, second);
    assert!(first.starts_with("# Environment Inventory — Handbook"));
    assert!(first.contains("> **Created (UTC):** 2026-07-10T12:34:56Z"));
    assert!(
        first.contains("> **Project Context Ref:** `.handbook/project_context/PROJECT_CONTEXT.md`")
    );
    assert!(first.contains("| `HANDBOOK_HOME` | No | No |"));
    assert!(first.contains("| `OPENAI_API_KEY` | No | Yes | None |"));
    assert!(first.contains("Unknown for future hosted use"));
    validate_environment_inventory_markdown(&first).expect("rendered markdown should validate");
}

#[test]
fn environment_inventory_core_refuses_malformed_yaml() {
    let error = parse_environment_inventory_structured_input_yaml("project_name: [")
        .expect_err("malformed YAML must fail");
    assert_eq!(
        error.kind,
        EnvironmentInventoryCoreErrorKind::MalformedStructuredInput
    );
}

#[test]
fn environment_inventory_core_refuses_incomplete_and_placeholder_inputs() {
    let incomplete = VALID_ENVIRONMENT_INVENTORY_INPUTS.replace(
        "project_name: \"Handbook\"",
        "project_name: \"{{PROJECT_NAME}}\"",
    );
    let error = parse_environment_inventory_structured_input_yaml(&incomplete)
        .expect_err("placeholder input must fail");
    assert_eq!(
        error.kind,
        EnvironmentInventoryCoreErrorKind::IncompleteStructuredInput
    );
    assert!(error.summary.contains("project_name"));

    let no_unknowns = VALID_ENVIRONMENT_INVENTORY_INPUTS.replace(
        "known_unknowns:\n  - item: \"future hosted runtime requirements\"\n    owner: \"project owner\"\n    revisit_trigger: \"before adding a hosted deployment\"",
        "known_unknowns: []",
    );
    let error = parse_environment_inventory_structured_input_yaml(&no_unknowns)
        .expect_err("unknowns must be represented explicitly");
    assert!(error.summary.contains("known_unknowns"));
}

#[test]
fn environment_inventory_core_refuses_secret_values() {
    let with_secret_value = VALID_ENVIRONMENT_INVENTORY_INPUTS.replace(
        "default_or_example: \"None\"\n    validation_notes: \"not used by deterministic authoring\"",
        "default_or_example: \"sk-live-secret-value\"\n    validation_notes: \"not used by deterministic authoring\"",
    );
    let error = parse_environment_inventory_structured_input_yaml(&with_secret_value)
        .expect_err("secret values must never enter structured inputs");
    assert_eq!(
        error.kind,
        EnvironmentInventoryCoreErrorKind::IncompleteStructuredInput
    );
    assert!(error.summary.contains("default_or_example"));
}
