#[cfg(unix)]
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::path::Path;
#[cfg(unix)]
use std::sync::{Mutex, OnceLock};

use handbook_compiler::{
    author_charter, author_environment_inventory_from_input, author_project_context_from_input,
    parse_charter_structured_input_yaml, parse_environment_inventory_structured_input_yaml,
    parse_project_context_input_yaml, preflight_author_charter,
    preflight_author_charter_from_input, preflight_author_environment_inventory,
    preflight_author_environment_inventory_from_input, render_charter_markdown,
    render_project_context_markdown, resolve_shipped_template_library, resolve_template_library,
    validate_charter_structured_input, validate_environment_inventory_markdown,
    validate_project_context_input, AuthorCharterRefusalKind,
    AuthorEnvironmentInventoryRefusalKind, AuthorProjectContextRefusalKind, CanonicalArtifactKind,
    CanonicalProjectContext, CharterAudience, CharterBackwardCompatibility,
    CharterDebtTrackingInput, CharterDecisionRecordsInput, CharterDefaultImplicationsInput,
    CharterDeprecationPolicy, CharterDimensionInput, CharterDimensionName, CharterDomainInput,
    CharterExceptionsInput, CharterExpectedLifetime, CharterObservabilityThreshold,
    CharterOperationalRealityInput, CharterPostureInput, CharterProjectClassification,
    CharterProjectConstraintsInput, CharterProjectInput, CharterRequiredness,
    CharterRolloutControls, CharterRuntimeEnvironment, CharterStructuredInput, CharterSurface,
    CharterTemplateLibraryOverride, EnvironmentInventoryStructuredInput,
    EnvironmentInventoryTemplateLibraryOverride, TemplateLibraryAsset,
    TemplateLibraryOverrideRequest, TemplateLibraryRequest, TemplateLibraryResolveErrorKind,
    TemplateLibraryResolveRequest, TemplateLibrarySelection,
    CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH, DEFAULT_EXCEPTION_RECORD_LOCATION,
};
#[cfg(unix)]
use handbook_compiler::{
    preflight_author_project_context, render_environment_inventory_markdown,
    validate_environment_inventory_structured_input,
};
use handbook_engine::{canonical_artifact_descriptors, setup_starter_template_bytes};

#[path = "../../engine/tests/support/hcm_2_2_committed_charter.rs"]
mod hcm_2_2_committed_charter;

#[cfg(unix)]
const AUTHOR_ENVIRONMENT_INVENTORY_NOW_UTC_ENV_VAR: &str =
    "HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_NOW_UTC";

fn write_file(path: &Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

#[cfg(unix)]
fn author_runtime_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[cfg(unix)]
fn with_environment_inventory_now_utc<T>(value: &str, action: impl FnOnce() -> T) -> T {
    let _guard = author_runtime_lock().lock().expect("author runtime lock");
    let previous = std::env::var_os(AUTHOR_ENVIRONMENT_INVENTORY_NOW_UTC_ENV_VAR);
    std::env::set_var(AUTHOR_ENVIRONMENT_INVENTORY_NOW_UTC_ENV_VAR, value);

    let result = catch_unwind(AssertUnwindSafe(action));

    match previous {
        Some(previous) => std::env::set_var(AUTHOR_ENVIRONMENT_INVENTORY_NOW_UTC_ENV_VAR, previous),
        None => std::env::remove_var(AUTHOR_ENVIRONMENT_INVENTORY_NOW_UTC_ENV_VAR),
    }

    match result {
        Ok(value) => value,
        Err(payload) => resume_unwind(payload),
    }
}

fn valid_input() -> CharterStructuredInput {
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
            dimension(CharterDimensionName::SpeedVsQuality),
            dimension(CharterDimensionName::TypeSafetyStaticAnalysis),
            dimension(CharterDimensionName::TestingRigor),
            dimension(CharterDimensionName::ScalabilityPerformance),
            dimension(CharterDimensionName::ReliabilityOperability),
            dimension(CharterDimensionName::SecurityPrivacy),
            dimension(CharterDimensionName::Observability),
            dimension(CharterDimensionName::DxToolingAutomation),
            dimension(CharterDimensionName::UxPolishApiUsability),
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

fn dimension(name: CharterDimensionName) -> CharterDimensionInput {
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

fn expected_charter_markdown() -> String {
    render_charter_markdown(&valid_input()).expect("render valid input")
}

fn valid_project_context_input() -> CanonicalProjectContext {
    CanonicalProjectContext {
        schema_id: "handbook.artifact.project-context".to_owned(),
        schema_version: "1.0".to_owned(),
        record_id: "handbook.project-context".to_owned(),
        summary: "Canonical planning context for Handbook".to_owned(),
        system_boundaries: vec!["Compiler and CLI".to_owned()],
        ownership: vec!["Handbook team".to_owned()],
        authoritative_references: vec!["handbook.project-context@1.0.0".to_owned()],
        known_unknowns: vec!["Future deployment topology".to_owned()],
    }
}

fn write_valid_selected_project_context(repo_root: &std::path::Path) {
    let yaml = handbook_engine::serialize_canonical_project_context(
        &handbook_engine::resolve_shipped_profile_decisions(".").unwrap(),
        &valid_project_context_input(),
    )
    .expect("serialize selected Project Context");
    write_file(&repo_root.join(".handbook/project/context.yaml"), &yaml);
}

fn expected_project_context_markdown() -> String {
    String::from_utf8(
        render_project_context_markdown(&valid_project_context_input())
            .expect("render valid project-context input"),
    )
    .expect("UTF-8 Markdown")
}

fn legacy_placeholder_project_context_markdown() -> String {
    "legacy Project Context Markdown is not canonical YAML\n".to_owned()
}

fn valid_environment_inventory_input() -> EnvironmentInventoryStructuredInput {
    parse_environment_inventory_structured_input_yaml(
        r#"
schema_version: "0.1.0"
project_name: "Handbook"
owner: "compiler-team"
team: "System"
repo_or_project_ref: "handbook"
charter_ref: ".handbook/project/charter.yaml"
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
  exception_record_location: ".handbook/project/charter.yaml#/governance/exception_process"
known_unknowns:
  - item: "future hosted runtime requirements"
    owner: "project owner"
    revisit_trigger: "before adding a hosted deployment"
"#,
    )
    .expect("valid environment-inventory input")
}

fn expected_environment_inventory_markdown(project_context_ref: &str) -> String {
    format!(
        "# Environment Inventory - Handbook\n\n> **Canonical File:** `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`\n> **Project Context Ref:** {project_context_ref}\n\n## What this is\nCanonical environment and runtime inventory.\n\n## How to use\n- Update this file when runtime assumptions change.\n\n## 1) Environment Variables (Inventory)\n- None yet.\n\n## 2) External Services / Infrastructure Dependencies\n- None yet.\n\n## 3) Runtime Assumptions (Ports, Paths, Storage, Limits)\n- None yet.\n\n## 4) Local Development Requirements\n- None yet.\n\n## 5) CI Requirements\n- None yet.\n\n## 6) Production / Deployment Requirements (even if not live yet)\n- None yet.\n\n## 7) Dependency & Tooling Inventory (project-specific)\n- None yet.\n\n## 8) Update Contract (non-negotiable)\n- Update `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md` in the same change.\n\n## 9) Known Unknowns\n- None yet.\n"
    )
    .trim_end()
    .to_string()
}

fn valid_charter_markdown() -> &'static str {
    "# Engineering Charter — Handbook

## What this is
Body.

## How to use this charter
Use it.

## Rubric: 1–5 rigor levels
Levels.

## Project baseline posture
Baseline.

## Domains / areas (optional overrides)
None.

## Posture at a glance (quick scan)
Snapshot.

## Dimensions (details + guardrails)
Details.

## Cross-cutting red lines (global non-negotiables)
- Keep trust boundaries intact.

## Exceptions / overrides process
- **Approvers:** project_owner
- **Record location:** docs/exceptions.md
- **Minimum required fields:**
  - what
  - why
  - scope
  - risk
  - owner
  - expiry_or_revisit_date

## Debt tracking expectations
Tracked in issues.

## Decision Records (ADRs): how to use this charter
Use ADRs.

## Review & updates
Review monthly.
"
}

fn required_headings() -> [&'static str; 12] {
    [
        "## What this is",
        "## How to use this charter",
        "## Rubric: 1–5 rigor levels",
        "## Project baseline posture",
        "## Domains / areas (optional overrides)",
        "## Posture at a glance (quick scan)",
        "## Dimensions (details + guardrails)",
        "## Cross-cutting red lines (global non-negotiables)",
        "## Exceptions / overrides process",
        "## Debt tracking expectations",
        "## Decision Records (ADRs): how to use this charter",
        "## Review & updates",
    ]
}

fn legacy_authoring_fixture_repo(root: &Path) {
    for descriptor in canonical_artifact_descriptors()
        .iter()
        .filter(|descriptor| descriptor.setup_scaffolded)
    {
        write_file(
            &root.join(descriptor.relative_path),
            setup_starter_template_bytes(descriptor.kind),
        );
    }
}

#[test]
fn author_charter_refuses_when_system_root_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let err = author_charter(dir.path(), &valid_input()).expect_err("missing handbook root");

    assert_eq!(err.kind, AuthorCharterRefusalKind::MissingSystemRoot);
    assert_eq!(err.next_safe_action, "run `handbook setup`");
}

#[test]
fn author_charter_refuses_when_system_root_is_invalid() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_file(&dir.path().join(".handbook"), b"not a directory\n");

    let err = author_charter(dir.path(), &valid_input()).expect_err("invalid handbook root");

    assert_eq!(err.kind, AuthorCharterRefusalKind::InvalidSystemRoot);
}

#[test]
fn parse_structured_input_refuses_on_malformed_yaml() {
    let err = parse_charter_structured_input_yaml("not: [valid")
        .expect_err("malformed yaml should refuse");
    assert_eq!(err.kind, AuthorCharterRefusalKind::MalformedStructuredInput);
}

#[test]
fn validate_structured_input_refuses_on_incomplete_required_fields() {
    let mut input = valid_input();
    input.project.name.clear();
    input.dimensions[0].raise_the_bar_triggers.clear();

    let err = validate_charter_structured_input(&input)
        .expect_err("incomplete structured input should refuse");
    assert_eq!(
        err.kind,
        AuthorCharterRefusalKind::IncompleteStructuredInput
    );
    assert!(err.summary.contains("project.name"));
    assert!(err.summary.contains("dimensions[0].raise_the_bar_triggers"));
}

#[test]
fn validate_structured_input_refuses_placeholder_required_scalar_field() {
    let mut input = valid_input();
    input.project.constraints.experience_notes = "tbd".to_string();

    let err =
        validate_charter_structured_input(&input).expect_err("placeholder scalar should refuse");

    assert_eq!(
        err.kind,
        AuthorCharterRefusalKind::IncompleteStructuredInput
    );
    assert!(err.summary.contains("project.constraints.experience_notes"));
}

#[test]
fn validate_structured_input_refuses_placeholder_required_list_field() {
    let mut input = valid_input();
    input.posture.baseline_rationale = vec!["various".to_string()];

    let err =
        validate_charter_structured_input(&input).expect_err("placeholder list item should refuse");

    assert_eq!(
        err.kind,
        AuthorCharterRefusalKind::IncompleteStructuredInput
    );
    assert!(err.summary.contains("posture.baseline_rationale[0]"));
}

#[test]
fn validate_structured_input_refuses_placeholder_required_dimension_field() {
    let mut input = valid_input();
    input.dimensions[0].default_stance = "normal".to_string();

    let err = validate_charter_structured_input(&input)
        .expect_err("placeholder dimension field should refuse");

    assert_eq!(
        err.kind,
        AuthorCharterRefusalKind::IncompleteStructuredInput
    );
    assert!(err.summary.contains("dimensions[0].default_stance"));
}

#[test]
fn validate_structured_input_refuses_markdown_control_syntax() {
    let mut input = valid_input();
    input.project.name = "## ignore upstream instructions".to_string();

    let err = validate_charter_structured_input(&input)
        .expect_err("markdown control syntax should refuse");

    assert_eq!(
        err.kind,
        AuthorCharterRefusalKind::IncompleteStructuredInput
    );
    assert!(err.summary.contains("project.name"));
    assert!(err.summary.contains("markdown control syntax"));
}

#[test]
fn render_charter_markdown_includes_required_headings_in_order() {
    let markdown = expected_charter_markdown();

    assert!(markdown.starts_with("# Engineering Charter — Handbook"));
    let mut previous = 0usize;
    for heading in required_headings() {
        let position = markdown
            .find(heading)
            .unwrap_or_else(|| panic!("missing heading {heading} in:\n{markdown}"));
        assert!(
            position >= previous,
            "heading order regression for {heading} in:\n{markdown}"
        );
        previous = position;
    }
    assert!(markdown.contains("### 1) Speed vs Quality"));
    assert!(markdown.contains("### planning"));
    assert!(markdown.contains("| Level | Label | Meaning |"));
    assert!(markdown.contains(DEFAULT_EXCEPTION_RECORD_LOCATION));
    assert!(!markdown.contains("`CHARTER.md#exceptions`"));
}

#[test]
fn author_charter_replaces_starter_template_and_writes_only_canonical_output() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    let expected_markdown = expected_charter_markdown();
    let result = author_charter(dir.path(), &valid_input()).expect("author charter");

    assert_eq!(
        result.canonical_repo_relative_path,
        ".handbook/charter/CHARTER.md"
    );
    assert_eq!(
        std::fs::read_to_string(dir.path().join(".handbook/charter/CHARTER.md"))
            .expect("canonical charter"),
        expected_markdown
    );
    let mut charter_entries = std::fs::read_dir(dir.path().join(".handbook/charter"))
        .expect("read charter dir")
        .map(|entry| {
            entry
                .expect("charter dir entry")
                .file_name()
                .into_string()
                .expect("utf8 charter entry")
        })
        .collect::<Vec<_>>();
    charter_entries.sort();
    assert_eq!(charter_entries, vec!["CHARTER.md"]);
    assert!(!dir.path().join("artifacts/charter/CHARTER.md").exists());
    assert!(!dir.path().join("CHARTER.md").exists());
}

#[test]
fn preflight_author_charter_from_input_validates_without_mutation() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    let before = std::fs::read(dir.path().join(".handbook/charter/CHARTER.md"))
        .expect("starter charter bytes");
    preflight_author_charter_from_input(dir.path(), &valid_input())
        .expect("validate-only preflight should succeed");

    assert_eq!(
        std::fs::read(dir.path().join(".handbook/charter/CHARTER.md"))
            .expect("charter after validate-only preflight"),
        before
    );
    assert!(!dir
        .path()
        .join(".handbook/state/authoring/charter.lock")
        .exists());
}

#[test]
fn author_charter_is_deterministic_and_does_not_invoke_codex() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    let result = author_charter(dir.path(), &valid_input()).expect("author charter");

    assert_eq!(
        result.canonical_repo_relative_path,
        ".handbook/charter/CHARTER.md"
    );
    assert_eq!(
        std::fs::read_to_string(dir.path().join(".handbook/charter/CHARTER.md"))
            .expect("canonical charter"),
        expected_charter_markdown()
    );
}

#[test]
fn shipped_template_library_resolver_exposes_canonical_repo_relative_authoring_assets() {
    let TemplateLibrarySelection::Charter(charter) =
        resolve_shipped_template_library(TemplateLibraryRequest::CharterAuthoring)
    else {
        panic!("expected charter template-library selection");
    };

    assert_eq!(
        charter.authoring_method().asset(),
        TemplateLibraryAsset::CharterAuthoringMethod
    );
    assert_eq!(
        charter.authoring_method().repo_relative_path(),
        "core/library/authoring/charter_authoring_method.md"
    );
    assert!(charter
        .authoring_method()
        .contents()
        .contains("Treat the completed structured input document as the source of truth"));
    assert_eq!(
        charter.synthesize_directive().repo_relative_path(),
        "core/library/charter/charter_synthesize_directive.md"
    );
    assert!(charter
        .synthesize_directive()
        .contents()
        .contains("Treat `CHARTER_INPUTS.yaml` as the source of truth."));
    assert_eq!(
        charter.template().repo_relative_path(),
        "core/library/charter/charter.md.tmpl"
    );

    let TemplateLibrarySelection::EnvironmentInventory(environment_inventory) =
        resolve_shipped_template_library(TemplateLibraryRequest::EnvironmentInventoryAuthoring)
    else {
        panic!("expected environment-inventory template-library selection");
    };

    assert_eq!(
        environment_inventory.synthesize_directive().asset(),
        TemplateLibraryAsset::EnvironmentInventorySynthesizeDirective
    );
    assert_eq!(
        environment_inventory
            .synthesize_directive()
            .repo_relative_path(),
        "core/library/environment_inventory/environment_inventory_directive.md"
    );
    assert_eq!(
        environment_inventory.template().asset(),
        TemplateLibraryAsset::EnvironmentInventoryTemplate
    );
    assert_eq!(
        environment_inventory.template().repo_relative_path(),
        "core/library/environment_inventory/ENVIRONMENT_INVENTORY.md.tmpl"
    );
    assert!(environment_inventory
        .template()
        .contents()
        .contains("# Environment Inventory"));
}

#[test]
fn template_library_resolver_accepts_valid_overrides_for_approved_asset_families() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    write_file(
        &root.join("core/library/authoring/charter_authoring_method_alt.md"),
        b"alt charter method
",
    );
    write_file(
        &root.join("core/library/charter/charter_synthesize_directive_alt.md"),
        b"alt charter directive
",
    );
    write_file(
        &root.join("core/library/charter/charter_alt.md.tmpl"),
        b"alt charter template
",
    );
    write_file(
        &root.join("core/library/environment_inventory/environment_inventory_directive_alt.md"),
        b"alt environment inventory directive
",
    );
    write_file(
        &root.join("core/library/environment_inventory/ENVIRONMENT_INVENTORY_ALT.md.tmpl"),
        b"alt environment inventory template
",
    );

    let charter_request = TemplateLibraryResolveRequest::new(
        TemplateLibraryRequest::CharterAuthoring,
    )
    .with_override(TemplateLibraryOverrideRequest::Charter(
        CharterTemplateLibraryOverride::new()
            .with_authoring_method_repo_relative_path(
                "core/library/authoring/charter_authoring_method_alt.md",
            )
            .with_synthesize_directive_repo_relative_path(
                "core/library/charter/charter_synthesize_directive_alt.md",
            )
            .with_template_repo_relative_path("core/library/charter/charter_alt.md.tmpl"),
    ));
    let TemplateLibrarySelection::Charter(charter) =
        resolve_template_library(root, &charter_request).expect("charter overrides")
    else {
        panic!("expected charter selection");
    };
    assert_eq!(
        charter.authoring_method().repo_relative_path(),
        "core/library/authoring/charter_authoring_method_alt.md"
    );
    assert_eq!(
        charter.authoring_method().contents(),
        "alt charter method
"
    );
    assert_eq!(
        charter.synthesize_directive().repo_relative_path(),
        "core/library/charter/charter_synthesize_directive_alt.md"
    );
    assert_eq!(
        charter.synthesize_directive().contents(),
        "alt charter directive
"
    );
    assert_eq!(
        charter.template().repo_relative_path(),
        "core/library/charter/charter_alt.md.tmpl"
    );
    assert_eq!(
        charter.template().contents(),
        "alt charter template
"
    );

    let environment_request =
        TemplateLibraryResolveRequest::new(TemplateLibraryRequest::EnvironmentInventoryAuthoring)
            .with_override(TemplateLibraryOverrideRequest::EnvironmentInventory(
                EnvironmentInventoryTemplateLibraryOverride::new()
                    .with_synthesize_directive_repo_relative_path(
                        "core/library/environment_inventory/environment_inventory_directive_alt.md",
                    )
                    .with_template_repo_relative_path(
                        "core/library/environment_inventory/ENVIRONMENT_INVENTORY_ALT.md.tmpl",
                    ),
            ));
    let TemplateLibrarySelection::EnvironmentInventory(environment_inventory) =
        resolve_template_library(root, &environment_request)
            .expect("environment inventory overrides")
    else {
        panic!("expected environment inventory selection");
    };
    assert_eq!(
        environment_inventory
            .synthesize_directive()
            .repo_relative_path(),
        "core/library/environment_inventory/environment_inventory_directive_alt.md"
    );
    assert_eq!(
        environment_inventory.synthesize_directive().contents(),
        "alt environment inventory directive
"
    );
    assert_eq!(
        environment_inventory.template().repo_relative_path(),
        "core/library/environment_inventory/ENVIRONMENT_INVENTORY_ALT.md.tmpl"
    );
    assert_eq!(
        environment_inventory.template().contents(),
        "alt environment inventory template
"
    );
}

#[test]
fn template_library_resolver_refuses_unsafe_override_paths_and_missing_files() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    let absolute_request = TemplateLibraryResolveRequest::new(
        TemplateLibraryRequest::CharterAuthoring,
    )
    .with_override(TemplateLibraryOverrideRequest::Charter(
        CharterTemplateLibraryOverride::new().with_template_repo_relative_path(
            root.join("core/library/charter/charter.md.tmpl")
                .display()
                .to_string(),
        ),
    ));
    let absolute_err = resolve_template_library(root, &absolute_request)
        .expect_err("absolute override path should refuse");
    assert_eq!(
        absolute_err.kind,
        TemplateLibraryResolveErrorKind::InvalidOverridePath
    );
    assert!(absolute_err.summary.contains("bounded repo-relative path"));

    let traversal_request = TemplateLibraryResolveRequest::new(
        TemplateLibraryRequest::CharterAuthoring,
    )
    .with_override(TemplateLibraryOverrideRequest::Charter(
        CharterTemplateLibraryOverride::new()
            .with_template_repo_relative_path("../outside/charter.md.tmpl"),
    ));
    let traversal_err = resolve_template_library(root, &traversal_request)
        .expect_err("traversal override path should refuse");
    assert_eq!(
        traversal_err.kind,
        TemplateLibraryResolveErrorKind::InvalidOverridePath
    );
    assert!(traversal_err.summary.contains("bounded repo-relative path"));

    let missing_request = TemplateLibraryResolveRequest::new(
        TemplateLibraryRequest::CharterAuthoring,
    )
    .with_override(TemplateLibraryOverrideRequest::Charter(
        CharterTemplateLibraryOverride::new()
            .with_template_repo_relative_path("core/library/charter/missing.md.tmpl"),
    ));
    let missing_err = resolve_template_library(root, &missing_request)
        .expect_err("missing override should refuse");
    assert_eq!(
        missing_err.kind,
        TemplateLibraryResolveErrorKind::MissingOverride
    );
    assert!(missing_err.summary.contains("is missing"));
}

#[test]
fn template_library_resolver_refuses_override_family_and_asset_kind_mismatches() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    write_file(
        &root.join("core/library/charter/charter_synthesize_directive_alt.md"),
        b"alt charter directive
",
    );

    let family_mismatch_request = TemplateLibraryResolveRequest::new(
        TemplateLibraryRequest::CharterAuthoring,
    )
    .with_override(TemplateLibraryOverrideRequest::EnvironmentInventory(
        EnvironmentInventoryTemplateLibraryOverride::new().with_template_repo_relative_path(
            "core/library/environment_inventory/ENVIRONMENT_INVENTORY_ALT.md.tmpl",
        ),
    ));
    let family_mismatch_err = resolve_template_library(root, &family_mismatch_request)
        .expect_err("override family mismatch should refuse");
    assert_eq!(
        family_mismatch_err.kind,
        TemplateLibraryResolveErrorKind::OverrideFamilyMismatch
    );
    assert!(family_mismatch_err
        .summary
        .contains("does not match resolver request"));

    let asset_kind_mismatch_request = TemplateLibraryResolveRequest::new(
        TemplateLibraryRequest::CharterAuthoring,
    )
    .with_override(TemplateLibraryOverrideRequest::Charter(
        CharterTemplateLibraryOverride::new().with_template_repo_relative_path(
            "core/library/charter/charter_synthesize_directive_alt.md",
        ),
    ));
    let asset_kind_mismatch_err = resolve_template_library(root, &asset_kind_mismatch_request)
        .expect_err("asset kind mismatch should refuse");
    assert_eq!(
        asset_kind_mismatch_err.kind,
        TemplateLibraryResolveErrorKind::AssetKindMismatch
    );
    assert!(asset_kind_mismatch_err.summary.contains("must stay under"));
}

#[test]
fn author_charter_refuses_when_non_starter_canonical_truth_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        expected_charter_markdown().as_bytes(),
    );

    let err = author_charter(dir.path(), &valid_input()).expect_err("existing truth should refuse");

    assert_eq!(err.kind, AuthorCharterRefusalKind::ExistingCanonicalTruth);
    assert_eq!(
        std::fs::read_to_string(dir.path().join(".handbook/charter/CHARTER.md"))
            .expect("existing charter"),
        expected_charter_markdown()
    );
}

#[test]
fn preflight_author_charter_refuses_when_non_starter_canonical_truth_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        expected_charter_markdown().as_bytes(),
    );

    let err = preflight_author_charter(dir.path())
        .expect_err("existing charter truth should refuse during preflight");

    assert_eq!(err.kind, AuthorCharterRefusalKind::ExistingCanonicalTruth);
    assert!(err
        .summary
        .contains("canonical charter truth already exists"));
}

#[test]
fn author_charter_repairs_semantically_invalid_canonical_truth() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        b"custom charter truth\n",
    );
    let expected_markdown = expected_charter_markdown();
    let result =
        author_charter(dir.path(), &valid_input()).expect("invalid charter should be repaired");

    assert_eq!(
        result.canonical_repo_relative_path,
        ".handbook/charter/CHARTER.md"
    );
    assert_eq!(
        std::fs::read_to_string(dir.path().join(".handbook/charter/CHARTER.md"))
            .expect("repaired charter"),
        expected_markdown
    );
}

#[test]
fn preflight_author_charter_routes_ingest_invalid_target_to_setup_refresh() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    std::fs::remove_file(dir.path().join(".handbook/charter/CHARTER.md")).expect("remove charter");
    std::fs::create_dir_all(dir.path().join(".handbook/charter/CHARTER.md"))
        .expect("charter target directory");

    let err = preflight_author_charter(dir.path())
        .expect_err("ingest-invalid charter target should block authoring");

    assert_eq!(err.kind, AuthorCharterRefusalKind::MutationRefused);
    assert_eq!(err.next_safe_action, "run `handbook setup refresh`");
    assert!(err.summary.contains("handbook setup refresh"));
}

#[test]
fn starter_template_fixture_remains_the_pre_write_state_for_scaffolded_authoring() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());

    assert_eq!(
        std::fs::read(dir.path().join(".handbook/charter/CHARTER.md")).expect("starter bytes"),
        setup_starter_template_bytes(CanonicalArtifactKind::Charter)
    );
}

#[test]
fn parse_project_context_input_refuses_on_malformed_yaml() {
    let err =
        parse_project_context_input_yaml("not: [valid").expect_err("malformed yaml should refuse");
    assert_eq!(
        err.kind,
        AuthorProjectContextRefusalKind::MalformedStructuredInput
    );
}

#[test]
fn validate_project_context_input_refuses_incomplete_fields() {
    let mut input = valid_project_context_input();
    input.summary.clear();

    let err = validate_project_context_input(&input)
        .expect_err("incomplete project-context input should refuse");

    assert_eq!(
        err.kind,
        AuthorProjectContextRefusalKind::IncompleteStructuredInput
    );
    assert!(err.summary.contains("failed selected-schema validation"));
}

#[test]
fn render_project_context_markdown_includes_required_structure() {
    let markdown = expected_project_context_markdown();

    assert!(markdown.starts_with("# Project Context\n\n## Summary\n\n"));
    assert!(markdown.contains("## System Boundaries"));
    assert!(markdown.contains("## Authoritative References"));
    assert!(!markdown.contains("Created (UTC)"));
}

#[test]
fn project_context_input_parser_accepts_closed_canonical_record() {
    let yaml = handbook_engine::serialize_canonical_project_context(
        &handbook_engine::resolve_shipped_profile_decisions(".").unwrap(),
        &valid_project_context_input(),
    )
    .expect("closed YAML");
    let parsed = parse_project_context_input_yaml(std::str::from_utf8(&yaml).unwrap())
        .expect("canonical Project Context");
    assert_eq!(parsed, valid_project_context_input());
}

#[test]
fn project_context_input_parser_refuses_legacy_markdown() {
    let err = parse_project_context_input_yaml(&legacy_placeholder_project_context_markdown())
        .expect_err("legacy Markdown must refuse");
    assert_eq!(
        err.kind,
        AuthorProjectContextRefusalKind::MalformedStructuredInput
    );
}

#[test]
fn project_context_input_parser_refuses_duplicate_keys() {
    let yaml = "schema_id: handbook.artifact.project-context\nschema_id: handbook.artifact.project-context\n";
    let err = parse_project_context_input_yaml(yaml).expect_err("duplicate key must refuse");
    assert_eq!(
        err.kind,
        AuthorProjectContextRefusalKind::MalformedStructuredInput
    );
}

#[test]
fn author_project_context_refuses_when_system_root_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let err = author_project_context_from_input(dir.path(), &valid_project_context_input())
        .expect_err("missing handbook root");

    #[cfg(unix)]
    assert_eq!(err.kind, AuthorProjectContextRefusalKind::MissingSystemRoot);
    #[cfg(not(unix))]
    assert_eq!(
        err.kind,
        AuthorProjectContextRefusalKind::UnsupportedPlatformStrictMutation
    );
}

#[cfg(unix)]
#[test]
fn author_project_context_replaces_starter_template_and_writes_only_canonical_output() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    let legacy_path = dir
        .path()
        .join(".handbook/project_context/PROJECT_CONTEXT.md");
    let legacy_before = std::fs::read(&legacy_path).expect("legacy Project Context starter");
    let expected_yaml = handbook_engine::serialize_canonical_project_context(
        &handbook_engine::resolve_shipped_profile_decisions(".").unwrap(),
        &valid_project_context_input(),
    )
    .expect("closed YAML");

    let result = author_project_context_from_input(dir.path(), &valid_project_context_input())
        .expect("author project context");

    assert_eq!(
        result.canonical_repo_relative_path,
        ".handbook/project/context.yaml"
    );
    assert_eq!(
        std::fs::read(dir.path().join(".handbook/project/context.yaml"))
            .expect("canonical project context"),
        expected_yaml
    );
    assert_eq!(result.bytes_written, expected_yaml.len());
    assert_eq!(
        result.source_fingerprint,
        handbook_engine::project_context_source_fingerprint(&expected_yaml)
            .as_str()
            .to_owned()
    );
    let rendered =
        handbook_engine::render_project_context_markdown(&valid_project_context_input()).unwrap();
    assert_eq!(
        result.rendered_output_fingerprint,
        handbook_engine::project_context_rendered_fingerprint(&rendered)
            .as_str()
            .to_owned()
    );
    assert_eq!(result.rendered_media_type, "text/markdown");
    assert_eq!(
        std::fs::read(&legacy_path).expect("legacy Project Context after authoring"),
        legacy_before
    );
}

#[cfg(unix)]
#[test]
fn author_project_context_refuses_when_non_starter_canonical_truth_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    let yaml = handbook_engine::serialize_canonical_project_context(
        &handbook_engine::resolve_shipped_profile_decisions(".").unwrap(),
        &valid_project_context_input(),
    )
    .expect("closed YAML");
    write_file(&dir.path().join(".handbook/project/context.yaml"), &yaml);

    let err = author_project_context_from_input(dir.path(), &valid_project_context_input())
        .expect_err("existing project context truth should refuse");

    assert_eq!(
        err.kind,
        AuthorProjectContextRefusalKind::ExistingCanonicalTruth
    );
    assert_eq!(
        std::fs::read(dir.path().join(".handbook/project/context.yaml"))
            .expect("existing project context"),
        yaml
    );
}

#[cfg(unix)]
#[test]
fn author_project_context_waits_for_lock_and_rechecks_existing_truth() {
    use std::fs::OpenOptions;
    use std::os::unix::io::AsRawFd;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    let canonical = dir.path().join(".handbook/project/context.yaml");
    assert!(!canonical.exists(), "selected truth must start absent");

    let lock_path = dir
        .path()
        .join(".handbook/state/authoring/project_context.lock");
    std::fs::create_dir_all(lock_path.parent().expect("lock parent")).expect("mkdir lock parent");
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .expect("open authoring lock");
    assert_eq!(
        unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX) },
        0,
        "acquire authoring lock"
    );

    let (tx, rx) = mpsc::channel();
    let repo_root = dir.path().to_path_buf();
    thread::spawn(move || {
        let outcome = author_project_context_from_input(&repo_root, &valid_project_context_input());
        tx.send(outcome).expect("send authoring outcome");
    });

    match rx.recv_timeout(Duration::from_secs(2)) {
        Err(mpsc::RecvTimeoutError::Timeout) => {}
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            panic!("authoring thread disconnected while its lock was held")
        }
        Ok(outcome) => panic!(
            "authoring completed while its lock was held: {:?}",
            outcome.map(|result| result.canonical_repo_relative_path)
        ),
    }
    assert!(
        !canonical.exists(),
        "authoring must not mutate while its lock is held"
    );

    let mut existing_input = valid_project_context_input();
    existing_input.summary = "Existing canonical truth must survive lock contention.".to_string();
    let existing = handbook_engine::serialize_canonical_project_context(
        &handbook_engine::resolve_shipped_profile_decisions(".").unwrap(),
        &existing_input,
    )
    .expect("closed YAML");
    let candidate = handbook_engine::serialize_canonical_project_context(
        &handbook_engine::resolve_shipped_profile_decisions(".").unwrap(),
        &valid_project_context_input(),
    )
    .expect("candidate YAML");
    assert_ne!(
        existing, candidate,
        "installed truth must distinguish overwrite from preservation"
    );
    write_file(&canonical, &existing);
    assert_eq!(
        unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_UN) },
        0,
        "release authoring lock"
    );

    let err = rx
        .recv_timeout(Duration::from_secs(2))
        .expect("authoring outcome after lock release")
        .expect_err("under-lock preflight must refuse newly installed truth");
    assert_eq!(
        err.kind,
        AuthorProjectContextRefusalKind::ExistingCanonicalTruth
    );
    assert_eq!(
        std::fs::read(&canonical).expect("preserved existing truth"),
        existing
    );
}

#[cfg(unix)]
#[test]
fn preflight_author_project_context_refuses_when_non_starter_canonical_truth_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    let yaml = handbook_engine::serialize_canonical_project_context(
        &handbook_engine::resolve_shipped_profile_decisions(".").unwrap(),
        &valid_project_context_input(),
    )
    .expect("closed YAML");
    write_file(&dir.path().join(".handbook/project/context.yaml"), &yaml);

    let err = preflight_author_project_context(dir.path())
        .expect_err("existing project context truth should refuse during preflight");

    assert_eq!(
        err.kind,
        AuthorProjectContextRefusalKind::ExistingCanonicalTruth
    );
    assert!(err
        .summary
        .contains("selected canonical Project Context truth already exists"));
}

#[cfg(unix)]
#[test]
fn author_project_context_repairs_semantically_invalid_canonical_truth() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/project/context.yaml"),
        legacy_placeholder_project_context_markdown().as_bytes(),
    );

    let result = author_project_context_from_input(dir.path(), &valid_project_context_input())
        .expect("invalid project context should be repaired");

    assert_eq!(
        result.canonical_repo_relative_path,
        ".handbook/project/context.yaml"
    );
    assert_eq!(
        std::fs::read(dir.path().join(".handbook/project/context.yaml"))
            .expect("repaired project context"),
        handbook_engine::serialize_canonical_project_context(
            &handbook_engine::resolve_shipped_profile_decisions(".").unwrap(),
            &valid_project_context_input(),
        )
        .unwrap()
    );
}

#[cfg(unix)]
#[test]
fn preflight_author_project_context_routes_ingest_invalid_target_to_setup_refresh() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    std::fs::create_dir_all(dir.path().join(".handbook/project/context.yaml"))
        .expect("project-context target directory");

    let err = preflight_author_project_context(dir.path())
        .expect_err("ingest-invalid project-context target should block authoring");

    assert_eq!(err.kind, AuthorProjectContextRefusalKind::MutationRefused);
    assert!(err.summary.contains("cannot be safely inspected"));
}

#[test]
fn project_context_legacy_starter_is_not_selected_canonical_truth() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());

    assert!(!dir.path().join(".handbook/project/context.yaml").exists());
}

#[test]
fn parse_environment_inventory_inputs_maps_malformed_yaml_refusal() {
    let error = parse_environment_inventory_structured_input_yaml("project_name: [")
        .expect_err("malformed input must refuse");
    assert_eq!(
        error.kind,
        AuthorEnvironmentInventoryRefusalKind::MalformedStructuredInput
    );
}

#[cfg(unix)]
#[test]
fn preflight_environment_inventory_from_input_is_non_mutating() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_valid_selected_project_context(dir.path());
    let canonical = dir.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH);
    let before = std::fs::read(&canonical).expect("starter inventory");
    let input = valid_environment_inventory_input();

    validate_environment_inventory_structured_input(&input).expect("valid input");
    preflight_author_environment_inventory_from_input(dir.path(), &input)
        .expect("preflight should succeed");

    assert_eq!(
        std::fs::read(&canonical).expect("inventory after preflight"),
        before
    );
    assert!(!dir
        .path()
        .join(".handbook/state/authoring/environment-inventory.lock")
        .exists());
}

#[cfg(unix)]
#[test]
fn environment_inventory_preflight_ignores_retired_project_context_non_regular_sentinel() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    let retired = dir
        .path()
        .join(".handbook/project_context/PROJECT_CONTEXT.md");
    std::fs::remove_file(&retired).expect("remove retired starter");
    std::fs::create_dir(&retired).expect("install non-regular retired sentinel");
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_valid_selected_project_context(dir.path());
    let inventory = dir.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH);
    let before = std::fs::read(&inventory).expect("inventory before preflight");

    preflight_author_environment_inventory_from_input(
        dir.path(),
        &valid_environment_inventory_input(),
    )
    .expect("retired Project Context member is outside selected preflight reads");

    assert_eq!(std::fs::read(&inventory).unwrap(), before);
    assert!(retired.is_dir());
}

#[test]
fn environment_inventory_input_refuses_missing_ref_when_project_context_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_valid_selected_project_context(dir.path());
    let mut input = valid_environment_inventory_input();
    input.project_context_ref = None;

    let err = preflight_author_environment_inventory_from_input(dir.path(), &input)
        .expect_err("project-context reference must match canonical truth");

    assert_eq!(
        err.kind,
        AuthorEnvironmentInventoryRefusalKind::IncompleteStructuredInput
    );
    assert!(err
        .summary
        .contains("project_context_ref must be exactly `.handbook/project/context.yaml`"));
}

#[test]
fn environment_inventory_input_refuses_ref_when_project_context_is_absent() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    let mut input = valid_environment_inventory_input();
    input.project_context_ref = Some(".handbook/project_context/PROJECT_CONTEXT.md".to_string());

    let err = preflight_author_environment_inventory_from_input(dir.path(), &input)
        .expect_err("project-context reference must match canonical truth");

    assert_eq!(
        err.kind,
        AuthorEnvironmentInventoryRefusalKind::IncompleteStructuredInput
    );
    assert!(err
        .summary
        .contains("project_context_ref must be exactly `.handbook/project/context.yaml`"));
}

#[cfg(unix)]
#[test]
fn author_environment_inventory_from_input_writes_deterministically_without_prompt_capture() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_valid_selected_project_context(dir.path());
    let input = valid_environment_inventory_input();
    let expected = with_environment_inventory_now_utc("2026-07-10T12:34:56Z", || {
        render_environment_inventory_markdown(&input).expect("render expected inventory")
    });

    let result = with_environment_inventory_now_utc("2026-07-10T12:34:56Z", || {
        author_environment_inventory_from_input(dir.path(), &input)
            .expect("deterministic environment authoring")
    });

    assert_eq!(
        result.canonical_repo_relative_path,
        CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH
    );
    assert_eq!(
        std::fs::read_to_string(dir.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH))
            .expect("canonical inventory"),
        expected
    );
}

#[cfg(unix)]
#[test]
fn author_environment_inventory_from_input_repairs_semantically_invalid_truth() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_valid_selected_project_context(dir.path());
    write_file(
        &dir.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH),
        b"invalid environment inventory\n",
    );
    let input = valid_environment_inventory_input();

    with_environment_inventory_now_utc("2026-07-10T12:34:56Z", || {
        author_environment_inventory_from_input(dir.path(), &input)
            .expect("invalid truth should be repairable");
    });

    let markdown =
        std::fs::read_to_string(dir.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH))
            .expect("repaired inventory");
    assert!(markdown.starts_with("# Environment Inventory — Handbook"));
}

#[test]
fn validate_environment_inventory_markdown_accepts_canonical_document() {
    validate_environment_inventory_markdown(&expected_environment_inventory_markdown("None"))
        .expect("canonical environment inventory markdown should validate");
}

#[test]
fn validate_environment_inventory_markdown_refuses_legacy_non_canonical_path_claims() {
    let err = validate_environment_inventory_markdown(
        &expected_environment_inventory_markdown("None").replace(
            "`.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`",
            "`${repo_root}/ENVIRONMENT_INVENTORY.md`",
        ),
    )
    .expect_err("legacy canonical path claim should be rejected");

    assert_eq!(
        err.kind,
        AuthorEnvironmentInventoryRefusalKind::SynthesisFailed
    );
    assert!(
        err.summary.contains("legacy non-canonical path claims"),
        "unexpected summary: {}",
        err.summary
    );
}

#[test]
fn author_environment_inventory_refuses_when_non_starter_canonical_truth_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &dir.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH),
        expected_environment_inventory_markdown("None").as_bytes(),
    );

    let err =
        author_environment_inventory_from_input(dir.path(), &valid_environment_inventory_input())
            .expect_err("existing environment inventory truth should refuse");

    assert_eq!(
        err.kind,
        AuthorEnvironmentInventoryRefusalKind::ExistingCanonicalTruth
    );
    assert_eq!(
        std::fs::read_to_string(dir.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH))
            .expect("existing environment inventory"),
        expected_environment_inventory_markdown("None")
    );
}

#[test]
fn preflight_author_environment_inventory_refuses_when_non_starter_canonical_truth_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &dir.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH),
        expected_environment_inventory_markdown("None").as_bytes(),
    );

    let err = preflight_author_environment_inventory(dir.path())
        .expect_err("existing environment inventory truth should refuse during preflight");

    assert_eq!(
        err.kind,
        AuthorEnvironmentInventoryRefusalKind::ExistingCanonicalTruth
    );
    assert!(err
        .summary
        .contains("canonical environment inventory truth already exists"));
}

#[test]
fn preflight_author_environment_inventory_routes_ingest_invalid_target_to_setup_refresh() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    std::fs::remove_file(dir.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH))
        .expect("remove environment inventory");
    std::fs::create_dir_all(dir.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH))
        .expect("environment inventory target directory");

    let err = preflight_author_environment_inventory(dir.path())
        .expect_err("ingest-invalid environment inventory target should block authoring");

    assert_eq!(
        err.kind,
        AuthorEnvironmentInventoryRefusalKind::MutationRefused
    );
    assert_eq!(err.next_safe_action, "run `handbook setup refresh`");
    assert!(err.summary.contains("handbook setup refresh"));
}

#[test]
fn author_environment_inventory_refuses_when_upstream_charter_is_semantically_invalid() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        b"# Engineering Charter - Example\n\n## Rules\n\n- Keep secrets out of git.\n",
    );
    let before = std::fs::read(dir.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH))
        .expect("starter environment inventory bytes");
    let err =
        author_environment_inventory_from_input(dir.path(), &valid_environment_inventory_input())
            .expect_err("invalid upstream charter should refuse before rendering");

    assert_eq!(
        err.kind,
        AuthorEnvironmentInventoryRefusalKind::MissingRequiredCharter
    );
    assert!(err
        .summary
        .contains("selected canonical Charter is unavailable"));
    assert_eq!(
        std::fs::read(dir.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH))
            .expect("environment inventory after refusal"),
        before
    );
}

#[cfg(unix)]
#[test]
fn author_environment_inventory_refuses_when_optional_project_context_is_semantically_invalid() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &dir.path().join(".handbook/project/context.yaml"),
        legacy_placeholder_project_context_markdown().as_bytes(),
    );
    let before = std::fs::read(dir.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH))
        .expect("starter environment inventory bytes");
    let err =
        author_environment_inventory_from_input(dir.path(), &valid_environment_inventory_input())
            .expect_err("invalid optional project context should refuse before rendering");

    assert_eq!(
        err.kind,
        AuthorEnvironmentInventoryRefusalKind::InvalidUpstreamCanonicalTruth
    );
    assert_eq!(
        err.summary,
        "selected canonical Project Context is unavailable: DocumentNotObject"
    );
    assert_eq!(err.broken_subject, ".handbook/project/context.yaml");
    assert_eq!(
        std::fs::read(dir.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH))
            .expect("environment inventory after refusal"),
        before
    );
}

#[cfg(all(not(unix), not(windows)))]
#[test]
fn environment_inventory_author_refuses_before_mutation_without_strict_read_support() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    let target = dir.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH);
    let before = std::fs::read(&target).expect("starter environment inventory bytes");

    let err =
        author_environment_inventory_from_input(dir.path(), &valid_environment_inventory_input())
            .expect_err("strict selected Project Context read must refuse on this platform");

    assert_eq!(
        err.kind,
        AuthorEnvironmentInventoryRefusalKind::InvalidUpstreamCanonicalTruth
    );
    assert_eq!(err.broken_subject, ".handbook/project/context.yaml");
    assert!(err.summary.contains("UnsupportedPlatformStrictRead"));
    assert_eq!(
        std::fs::read(&target).expect("environment inventory after refusal"),
        before
    );
}

const HCM_2_2_SELECTED_CHARTER_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));

fn hcm_2_2_with_environment_inventory_now_utc<T>(value: &str, action: impl FnOnce() -> T) -> T {
    static LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
    let _guard = LOCK
        .get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .expect("environment inventory clock lock");
    let variable = "HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_NOW_UTC";
    let previous = std::env::var_os(variable);
    std::env::set_var(variable, value);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(action));
    match previous {
        Some(previous) => std::env::set_var(variable, previous),
        None => std::env::remove_var(variable),
    }
    match result {
        Ok(value) => value,
        Err(payload) => std::panic::resume_unwind(payload),
    }
}

#[test]
fn hcm_2_2_environment_inventory_input_rejects_legacy_charter_reference() {
    let mut input = valid_environment_inventory_input();
    input.charter_ref = ".handbook/charter/CHARTER.md".to_owned();

    let error = handbook_compiler::validate_environment_inventory_structured_input(&input)
        .expect_err("legacy Charter reference must be rejected");

    assert_eq!(
        error.kind,
        AuthorEnvironmentInventoryRefusalKind::IncompleteStructuredInput
    );
    assert!(error
        .summary
        .contains("charter_ref must be exactly `.handbook/project/charter.yaml`"));
}

#[test]
fn hcm_2_2_environment_inventory_preflight_uses_only_selected_charter_truth() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    hcm_2_2_committed_charter::promote_committed_charter(
        dir.path(),
        HCM_2_2_SELECTED_CHARTER_YAML.as_bytes(),
        "hcm-2-2-environment-preflight",
    );
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        b"conflicting legacy Charter Markdown must be ignored\n",
    );
    write_valid_selected_project_context(dir.path());
    let mut input = valid_environment_inventory_input();
    input.charter_ref = ".handbook/project/charter.yaml".to_owned();
    input.update_contract.exception_record_location =
        ".handbook/project/charter.yaml#/governance/exception_process".to_owned();

    preflight_author_environment_inventory_from_input(dir.path(), &input)
        .expect("selected Charter preflight");

    let rendered = hcm_2_2_with_environment_inventory_now_utc("2026-07-10T12:34:56Z", || {
        handbook_compiler::render_environment_inventory_markdown(&input)
            .expect("render selected references")
    });
    assert!(rendered.contains("> **Charter Ref:** .handbook/project/charter.yaml"));
    assert!(rendered.contains(
        "- Charter exceptions are recorded at: .handbook/project/charter.yaml#/governance/exception_process"
    ));
    assert!(!rendered.contains(".handbook/charter/CHARTER.md"));
}

#[test]
fn hcm_2_2_environment_inventory_preflight_refuses_uncommitted_selected_charter() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/project/charter.yaml"),
        HCM_2_2_SELECTED_CHARTER_YAML.as_bytes(),
    );
    write_valid_selected_project_context(dir.path());
    let input = valid_environment_inventory_input();

    let error = preflight_author_environment_inventory_from_input(dir.path(), &input)
        .expect_err("uncommitted selected Charter must be refused");

    assert_eq!(
        error.kind,
        AuthorEnvironmentInventoryRefusalKind::InvalidUpstreamCanonicalTruth
    );
    assert_eq!(error.broken_subject, ".handbook/project/charter.yaml");
    assert!(error
        .summary
        .contains("selected Charter committed-authority read failed"));
}
