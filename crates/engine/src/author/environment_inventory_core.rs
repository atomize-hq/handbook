use serde::{Deserialize, Serialize};
use std::fmt::Write as _;

// These literals remain handbook-product authoring defaults for engine-owned
// markdown synthesis and validation. They are intentionally code-owned and do
// not describe the reusable import-layout contract.
const CANONICAL_ENVIRONMENT_INVENTORY_PATH: &str =
    ".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md";
const LEGACY_NON_CANONICAL_PATH_CLAIMS: [&str; 3] = [
    "${repo_root}/ENVIRONMENT_INVENTORY.md",
    "artifacts/foundation/ENVIRONMENT_INVENTORY.md",
    "repo/project root",
];
const PROJECT_CONTEXT_REF_PRESENT_LINE: &str =
    "> **Project Context Ref:** `.handbook/project_context/PROJECT_CONTEXT.md`";
const PROJECT_CONTEXT_REF_ABSENT_LINE: &str = "> **Project Context Ref:** None";
const ENVIRONMENT_INVENTORY_INPUTS_SCHEMA_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentInventoryCoreErrorKind {
    MalformedStructuredInput,
    IncompleteStructuredInput,
    DeterministicRenderFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentInventoryCoreError {
    pub kind: EnvironmentInventoryCoreErrorKind,
    pub summary: String,
}

impl std::fmt::Display for EnvironmentInventoryCoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary)
    }
}

impl std::error::Error for EnvironmentInventoryCoreError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentInventoryStructuredInput {
    pub schema_version: String,
    pub project_name: String,
    pub owner: String,
    pub team: String,
    pub repo_or_project_ref: String,
    pub charter_ref: String,
    pub project_context_ref: Option<String>,
    #[serde(default)]
    pub environment_variables: Vec<EnvironmentVariableInput>,
    pub secret_handling: EnvironmentSecretHandlingInput,
    #[serde(default)]
    pub external_services: Vec<EnvironmentExternalServiceInput>,
    pub runtime_assumptions: EnvironmentRuntimeAssumptionsInput,
    pub local_development: EnvironmentLocalDevelopmentInput,
    pub ci: EnvironmentCiInput,
    pub production: EnvironmentProductionInput,
    pub tooling: EnvironmentToolingInput,
    pub update_contract: EnvironmentUpdateContractInput,
    pub known_unknowns: Vec<EnvironmentKnownUnknownInput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentVariableInput {
    pub name: String,
    pub required: bool,
    pub secret: bool,
    pub default_or_example: String,
    pub validation_notes: String,
    pub where_used: String,
    pub source_of_truth: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentSecretHandlingInput {
    pub charter_posture: String,
    pub storage_locations: Vec<String>,
    pub rotation_expectations: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentExternalServiceInput {
    pub name: String,
    pub required: bool,
    pub environments: Vec<String>,
    pub purpose: String,
    pub connection_details: String,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentRuntimeAssumptionsInput {
    pub listening_ports: String,
    pub filesystem_requirements: String,
    pub persistent_storage: String,
    pub network_assumptions: String,
    pub performance_budgets: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentLocalDevelopmentInput {
    pub prerequisites: Vec<String>,
    pub works_on_my_machine_prevention: String,
    pub environment_file_pattern: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentCiInput {
    pub system: String,
    pub required_secret_names: Vec<String>,
    pub services: Vec<String>,
    pub artifacts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentProductionInput {
    pub exists_today: bool,
    pub hosting_model: String,
    pub runtime_environments: Vec<String>,
    pub required_secret_names: Vec<String>,
    pub observability: String,
    pub backup_and_disaster_recovery: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentToolingInput {
    pub primary_language_runtime: String,
    pub package_manager_build_system: String,
    pub lockfiles: Vec<String>,
    pub lint_type_test_tools: Vec<String>,
    pub minimum_versions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentUpdateContractInput {
    pub exception_record_location: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentKnownUnknownInput {
    pub item: String,
    pub owner: String,
    pub revisit_trigger: String,
}

pub const REQUIRED_ENVIRONMENT_INVENTORY_HEADINGS: [&str; 11] = [
    "## What this is",
    "## How to use",
    "## 1) Environment Variables (Inventory)",
    "## 2) External Services / Infrastructure Dependencies",
    "## 3) Runtime Assumptions (Ports, Paths, Storage, Limits)",
    "## 4) Local Development Requirements",
    "## 5) CI Requirements",
    "## 6) Production / Deployment Requirements (even if not live yet)",
    "## 7) Dependency & Tooling Inventory (project-specific)",
    "## 8) Update Contract (non-negotiable)",
    "## 9) Known Unknowns",
];

#[deprecated(
    since = "0.1.1",
    note = "use `validate_environment_inventory_markdown`; deterministic authoring validates project-context references during repository preflight"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnvironmentInventoryValidationExpectations {
    expected_project_context_ref_line: &'static str,
}

#[allow(deprecated)]
impl EnvironmentInventoryValidationExpectations {
    pub fn for_optional_project_context(has_project_context: bool) -> Self {
        Self {
            expected_project_context_ref_line: if has_project_context {
                PROJECT_CONTEXT_REF_PRESENT_LINE
            } else {
                PROJECT_CONTEXT_REF_ABSENT_LINE
            },
        }
    }

    pub fn expected_project_context_ref_line(self) -> &'static str {
        self.expected_project_context_ref_line
    }
}

pub fn parse_environment_inventory_structured_input_yaml(
    yaml: &str,
) -> Result<EnvironmentInventoryStructuredInput, EnvironmentInventoryCoreError> {
    let input =
        serde_yaml_bw::from_str::<EnvironmentInventoryStructuredInput>(yaml).map_err(|error| {
            EnvironmentInventoryCoreError {
                kind: EnvironmentInventoryCoreErrorKind::MalformedStructuredInput,
                summary: format!("structured environment-inventory input is malformed: {error}"),
            }
        })?;
    validate_environment_inventory_structured_input(&input)?;
    Ok(input)
}

pub fn validate_environment_inventory_structured_input(
    input: &EnvironmentInventoryStructuredInput,
) -> Result<(), EnvironmentInventoryCoreError> {
    let mut issues = Vec::new();

    if input.schema_version.trim() != ENVIRONMENT_INVENTORY_INPUTS_SCHEMA_VERSION {
        issues.push(format!(
            "schema_version must be `{ENVIRONMENT_INVENTORY_INPUTS_SCHEMA_VERSION}`"
        ));
    }
    require_environment_text("project_name", &input.project_name, &mut issues);
    require_environment_text("owner", &input.owner, &mut issues);
    require_environment_text("team", &input.team, &mut issues);
    require_environment_text(
        "repo_or_project_ref",
        &input.repo_or_project_ref,
        &mut issues,
    );
    require_environment_text("charter_ref", &input.charter_ref, &mut issues);
    if let Some(project_context_ref) = &input.project_context_ref {
        require_environment_text("project_context_ref", project_context_ref, &mut issues);
        if project_context_ref.trim() != ".handbook/project_context/PROJECT_CONTEXT.md" {
            issues.push(
                "project_context_ref must be `.handbook/project_context/PROJECT_CONTEXT.md` or null"
                    .to_string(),
            );
        }
    }

    for (index, variable) in input.environment_variables.iter().enumerate() {
        let prefix = format!("environment_variables[{index}]");
        require_environment_text(&format!("{prefix}.name"), &variable.name, &mut issues);
        require_environment_text(
            &format!("{prefix}.default_or_example"),
            &variable.default_or_example,
            &mut issues,
        );
        require_environment_text(
            &format!("{prefix}.validation_notes"),
            &variable.validation_notes,
            &mut issues,
        );
        require_environment_text(
            &format!("{prefix}.where_used"),
            &variable.where_used,
            &mut issues,
        );
        require_environment_text(
            &format!("{prefix}.source_of_truth"),
            &variable.source_of_truth,
            &mut issues,
        );
        if variable.secret && !is_safe_secret_example(&variable.default_or_example) {
            issues.push(format!(
                "{prefix}.default_or_example must describe absence or redaction for secret variables; real secret values are forbidden"
            ));
        }
    }

    require_environment_text(
        "secret_handling.charter_posture",
        &input.secret_handling.charter_posture,
        &mut issues,
    );
    require_environment_list(
        "secret_handling.storage_locations",
        &input.secret_handling.storage_locations,
        &mut issues,
    );
    require_environment_text(
        "secret_handling.rotation_expectations",
        &input.secret_handling.rotation_expectations,
        &mut issues,
    );

    for (index, service) in input.external_services.iter().enumerate() {
        let prefix = format!("external_services[{index}]");
        require_environment_text(&format!("{prefix}.name"), &service.name, &mut issues);
        require_environment_list(
            &format!("{prefix}.environments"),
            &service.environments,
            &mut issues,
        );
        require_environment_text(&format!("{prefix}.purpose"), &service.purpose, &mut issues);
        require_environment_text(
            &format!("{prefix}.connection_details"),
            &service.connection_details,
            &mut issues,
        );
        require_environment_text(&format!("{prefix}.notes"), &service.notes, &mut issues);
    }

    let runtime = &input.runtime_assumptions;
    require_environment_text(
        "runtime_assumptions.listening_ports",
        &runtime.listening_ports,
        &mut issues,
    );
    require_environment_text(
        "runtime_assumptions.filesystem_requirements",
        &runtime.filesystem_requirements,
        &mut issues,
    );
    require_environment_text(
        "runtime_assumptions.persistent_storage",
        &runtime.persistent_storage,
        &mut issues,
    );
    require_environment_text(
        "runtime_assumptions.network_assumptions",
        &runtime.network_assumptions,
        &mut issues,
    );
    require_environment_text(
        "runtime_assumptions.performance_budgets",
        &runtime.performance_budgets,
        &mut issues,
    );

    require_environment_list(
        "local_development.prerequisites",
        &input.local_development.prerequisites,
        &mut issues,
    );
    require_environment_text(
        "local_development.works_on_my_machine_prevention",
        &input.local_development.works_on_my_machine_prevention,
        &mut issues,
    );
    require_environment_text(
        "local_development.environment_file_pattern",
        &input.local_development.environment_file_pattern,
        &mut issues,
    );

    require_environment_text("ci.system", &input.ci.system, &mut issues);
    require_environment_list(
        "ci.required_secret_names",
        &input.ci.required_secret_names,
        &mut issues,
    );
    require_environment_list("ci.services", &input.ci.services, &mut issues);
    require_environment_list("ci.artifacts", &input.ci.artifacts, &mut issues);

    require_environment_text(
        "production.hosting_model",
        &input.production.hosting_model,
        &mut issues,
    );
    require_environment_list(
        "production.runtime_environments",
        &input.production.runtime_environments,
        &mut issues,
    );
    require_environment_list(
        "production.required_secret_names",
        &input.production.required_secret_names,
        &mut issues,
    );
    require_environment_text(
        "production.observability",
        &input.production.observability,
        &mut issues,
    );
    require_environment_text(
        "production.backup_and_disaster_recovery",
        &input.production.backup_and_disaster_recovery,
        &mut issues,
    );

    require_environment_text(
        "tooling.primary_language_runtime",
        &input.tooling.primary_language_runtime,
        &mut issues,
    );
    require_environment_text(
        "tooling.package_manager_build_system",
        &input.tooling.package_manager_build_system,
        &mut issues,
    );
    require_environment_list("tooling.lockfiles", &input.tooling.lockfiles, &mut issues);
    require_environment_list(
        "tooling.lint_type_test_tools",
        &input.tooling.lint_type_test_tools,
        &mut issues,
    );
    require_environment_list(
        "tooling.minimum_versions",
        &input.tooling.minimum_versions,
        &mut issues,
    );
    require_environment_text(
        "update_contract.exception_record_location",
        &input.update_contract.exception_record_location,
        &mut issues,
    );

    if input.known_unknowns.is_empty() {
        issues.push(
            "known_unknowns must explicitly track at least one unknown or state that none remain"
                .to_string(),
        );
    }
    for (index, unknown) in input.known_unknowns.iter().enumerate() {
        let prefix = format!("known_unknowns[{index}]");
        require_environment_text(&format!("{prefix}.item"), &unknown.item, &mut issues);
        require_environment_text(&format!("{prefix}.owner"), &unknown.owner, &mut issues);
        require_environment_text(
            &format!("{prefix}.revisit_trigger"),
            &unknown.revisit_trigger,
            &mut issues,
        );
    }

    if issues.is_empty() {
        Ok(())
    } else {
        Err(EnvironmentInventoryCoreError {
            kind: EnvironmentInventoryCoreErrorKind::IncompleteStructuredInput,
            summary: format!(
                "structured environment-inventory input is incomplete: {}",
                issues.join("; ")
            ),
        })
    }
}

pub fn render_environment_inventory_markdown(
    input: &EnvironmentInventoryStructuredInput,
    now_utc: &str,
) -> Result<String, EnvironmentInventoryCoreError> {
    validate_environment_inventory_structured_input(input)?;
    if now_utc.trim().is_empty() {
        return Err(EnvironmentInventoryCoreError {
            kind: EnvironmentInventoryCoreErrorKind::DeterministicRenderFailed,
            summary: "environment-inventory render requires an explicit UTC timestamp".to_string(),
        });
    }

    let mut out = String::new();
    writeln!(
        out,
        "# Environment Inventory — {}",
        input.project_name.trim()
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "> **Canonical File:** `{CANONICAL_ENVIRONMENT_INVENTORY_PATH}`  "
    )
    .unwrap();
    writeln!(out, "> **Created (UTC):** {}  ", now_utc.trim()).unwrap();
    writeln!(out, "> **Owner:** {}  ", input.owner.trim()).unwrap();
    writeln!(out, "> **Team:** {}  ", input.team.trim()).unwrap();
    writeln!(
        out,
        "> **Repo / Project:** {}  ",
        input.repo_or_project_ref.trim()
    )
    .unwrap();
    writeln!(out, "> **Charter Ref:** {}  ", input.charter_ref.trim()).unwrap();
    match &input.project_context_ref {
        Some(reference) => writeln!(out, "> **Project Context Ref:** `{}`", reference.trim()),
        None => writeln!(out, "{PROJECT_CONTEXT_REF_ABSENT_LINE}"),
    }
    .unwrap();

    writeln!(out).unwrap();
    writeln!(out, "## What this is").unwrap();
    writeln!(
        out,
        "The canonical store of record for this project's environment and runtime requirements."
    )
    .unwrap();
    writeln!(out, "It prevents drift by keeping variables, services, ports, and runtime assumptions explicit.").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## How to use").unwrap();
    writeln!(
        out,
        "- Treat `{CANONICAL_ENVIRONMENT_INVENTORY_PATH}` as authoritative environment truth."
    )
    .unwrap();
    writeln!(
        out,
        "- Update this file in the same change as any environment-impacting change."
    )
    .unwrap();
    writeln!(
        out,
        "- Record unknowns explicitly instead of inventing project facts."
    )
    .unwrap();

    writeln!(out).unwrap();
    writeln!(out, "## 1) Environment Variables (Inventory)").unwrap();
    writeln!(out, "| Name | Required | Secret | Default/Example | Validation / Notes | Where used | Source of truth |").unwrap();
    writeln!(out, "|---|---:|---:|---|---|---|---|").unwrap();
    if input.environment_variables.is_empty() {
        writeln!(
            out,
            "| None | No | No | None | No environment variables declared | None | This inventory |"
        )
        .unwrap();
    } else {
        for variable in &input.environment_variables {
            writeln!(
                out,
                "| `{}` | {} | {} | {} | {} | {} | {} |",
                variable.name.trim(),
                yes_no(variable.required),
                yes_no(variable.secret),
                variable.default_or_example.trim(),
                variable.validation_notes.trim(),
                variable.where_used.trim(),
                variable.source_of_truth.trim()
            )
            .unwrap();
        }
    }
    writeln!(out).unwrap();
    writeln!(out, "### 1.1) Secret handling (Charter-aligned)").unwrap();
    writeln!(
        out,
        "- Secret posture (from Charter): {}",
        input.secret_handling.charter_posture.trim()
    )
    .unwrap();
    writeln!(
        out,
        "- Storage location(s): {}",
        join_values(&input.secret_handling.storage_locations)
    )
    .unwrap();
    writeln!(
        out,
        "- Rotation expectations: {}",
        input.secret_handling.rotation_expectations.trim()
    )
    .unwrap();
    writeln!(
        out,
        "- Never allowed in repo: real tokens, private keys, raw credentials"
    )
    .unwrap();

    writeln!(out).unwrap();
    writeln!(out, "## 2) External Services / Infrastructure Dependencies").unwrap();
    writeln!(
        out,
        "| Dependency | Required | Environments | Purpose | Connection details | Notes |"
    )
    .unwrap();
    writeln!(out, "|---|---:|---|---|---|---|").unwrap();
    if input.external_services.is_empty() {
        writeln!(
            out,
            "| None | No | None | No external services declared | None | None |"
        )
        .unwrap();
    } else {
        for service in &input.external_services {
            writeln!(
                out,
                "| {} | {} | {} | {} | {} | {} |",
                service.name.trim(),
                yes_no(service.required),
                join_values(&service.environments),
                service.purpose.trim(),
                service.connection_details.trim(),
                service.notes.trim()
            )
            .unwrap();
        }
    }

    let runtime = &input.runtime_assumptions;
    writeln!(out).unwrap();
    writeln!(
        out,
        "## 3) Runtime Assumptions (Ports, Paths, Storage, Limits)"
    )
    .unwrap();
    writeln!(
        out,
        "- Required listening ports: {}",
        runtime.listening_ports.trim()
    )
    .unwrap();
    writeln!(
        out,
        "- Filesystem needs (paths, permissions): {}",
        runtime.filesystem_requirements.trim()
    )
    .unwrap();
    writeln!(
        out,
        "- Persistent storage requirements: {}",
        runtime.persistent_storage.trim()
    )
    .unwrap();
    writeln!(
        out,
        "- Network assumptions: {}",
        runtime.network_assumptions.trim()
    )
    .unwrap();
    writeln!(
        out,
        "- Performance/time budgets that impact env: {}",
        runtime.performance_budgets.trim()
    )
    .unwrap();

    writeln!(out).unwrap();
    writeln!(out, "## 4) Local Development Requirements").unwrap();
    writeln!(
        out,
        "- Local prerequisites: {}",
        join_values(&input.local_development.prerequisites)
    )
    .unwrap();
    writeln!(
        out,
        "- Works-on-my-machine prevention notes: {}",
        input
            .local_development
            .works_on_my_machine_prevention
            .trim()
    )
    .unwrap();
    writeln!(
        out,
        "- Recommended local env var file pattern: {}",
        input.local_development.environment_file_pattern.trim()
    )
    .unwrap();

    writeln!(out).unwrap();
    writeln!(out, "## 5) CI Requirements").unwrap();
    writeln!(out, "- CI system: {}", input.ci.system.trim()).unwrap();
    writeln!(
        out,
        "- Required CI secret/variable names: {}",
        join_values(&input.ci.required_secret_names)
    )
    .unwrap();
    writeln!(
        out,
        "- Services required during CI: {}",
        join_values(&input.ci.services)
    )
    .unwrap();
    writeln!(
        out,
        "- Artifacts produced/required: {}",
        join_values(&input.ci.artifacts)
    )
    .unwrap();

    writeln!(out).unwrap();
    writeln!(
        out,
        "## 6) Production / Deployment Requirements (even if not live yet)"
    )
    .unwrap();
    writeln!(
        out,
        "- Is there production today? {}",
        yes_no(input.production.exists_today)
    )
    .unwrap();
    writeln!(
        out,
        "- Hosting model: {}",
        input.production.hosting_model.trim()
    )
    .unwrap();
    writeln!(
        out,
        "- Runtime environment(s): {}",
        join_values(&input.production.runtime_environments)
    )
    .unwrap();
    writeln!(
        out,
        "- Required secret names in production: {}",
        join_values(&input.production.required_secret_names)
    )
    .unwrap();
    writeln!(
        out,
        "- Observability endpoints/keys: {}",
        input.production.observability.trim()
    )
    .unwrap();
    writeln!(
        out,
        "- Backup/DR requirements: {}",
        input.production.backup_and_disaster_recovery.trim()
    )
    .unwrap();

    writeln!(out).unwrap();
    writeln!(
        out,
        "## 7) Dependency & Tooling Inventory (project-specific)"
    )
    .unwrap();
    writeln!(
        out,
        "- Primary language/runtime: {}",
        input.tooling.primary_language_runtime.trim()
    )
    .unwrap();
    writeln!(
        out,
        "- Package manager / build system: {}",
        input.tooling.package_manager_build_system.trim()
    )
    .unwrap();
    writeln!(
        out,
        "- Lockfile(s): {}",
        join_values(&input.tooling.lockfiles)
    )
    .unwrap();
    writeln!(
        out,
        "- Lint/type/test tools: {}",
        join_values(&input.tooling.lint_type_test_tools)
    )
    .unwrap();
    writeln!(
        out,
        "- Minimum supported versions: {}",
        join_values(&input.tooling.minimum_versions)
    )
    .unwrap();

    writeln!(out).unwrap();
    writeln!(out, "## 8) Update Contract (non-negotiable)").unwrap();
    writeln!(out, "Any change that impacts environment MUST update `{CANONICAL_ENVIRONMENT_INVENTORY_PATH}` in the same change.").unwrap();
    writeln!(out, "- Adding or changing an environment variable").unwrap();
    writeln!(out, "- Adding or changing an external service dependency").unwrap();
    writeln!(
        out,
        "- Changing ports, secret locations, runtime assumptions, production, or CI requirements"
    )
    .unwrap();
    writeln!(
        out,
        "- Charter exceptions are recorded at: {}",
        input.update_contract.exception_record_location.trim()
    )
    .unwrap();

    writeln!(out).unwrap();
    writeln!(out, "## 9) Known Unknowns").unwrap();
    for unknown in &input.known_unknowns {
        writeln!(
            out,
            "- {} (owner: {}; revisit: {})",
            unknown.item.trim(),
            unknown.owner.trim(),
            unknown.revisit_trigger.trim()
        )
        .unwrap();
    }

    let rendered = out.trim_end().to_string();
    validate_environment_inventory_markdown(&rendered).map_err(|error| {
        EnvironmentInventoryCoreError {
            kind: EnvironmentInventoryCoreErrorKind::DeterministicRenderFailed,
            summary: format!(
                "deterministic environment-inventory render failed validation: {error}"
            ),
        }
    })?;
    Ok(rendered)
}

fn require_environment_list(field: &str, values: &[String], issues: &mut Vec<String>) {
    if values.is_empty() {
        issues.push(format!("{field} must contain at least one explicit value"));
        return;
    }
    for (index, value) in values.iter().enumerate() {
        require_environment_text(&format!("{field}[{index}]"), value, issues);
    }
}

fn require_environment_text(field: &str, value: &str, issues: &mut Vec<String>) {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        issues.push(format!("{field} must not be blank"));
        return;
    }
    let lower = trimmed.to_ascii_lowercase();
    if trimmed.contains("{{")
        || trimmed.contains("}}")
        || matches!(
            lower.as_str(),
            "todo" | "tbd" | "placeholder" | "fill me in"
        )
    {
        issues.push(format!("{field} must not contain a template placeholder"));
    }
    if trimmed.contains(['\n', '\r', '|', '`'])
        || trimmed.starts_with('#')
        || trimmed.starts_with('>')
        || trimmed.starts_with("- ")
        || trimmed.starts_with("* ")
        || trimmed.contains("<!--")
        || trimmed.contains("-->")
    {
        issues.push(format!(
            "{field} must not contain markdown control syntax or table delimiters"
        ));
    }
}

fn is_safe_secret_example(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "none" | "not applicable" | "unknown" | "redacted" | "secret store only"
    )
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "Yes"
    } else {
        "No"
    }
}

fn join_values(values: &[String]) -> String {
    values
        .iter()
        .map(|value| value.trim())
        .collect::<Vec<_>>()
        .join(", ")
}

pub fn validate_environment_inventory_markdown(markdown: &str) -> Result<(), String> {
    let normalized = markdown.trim();
    if normalized.is_empty() {
        return Err("synthesized environment inventory markdown was empty".to_string());
    }
    if !markdown.starts_with("# Environment Inventory") {
        return Err(
            "synthesized environment inventory markdown must start with `# Environment Inventory`"
                .to_string(),
        );
    }
    if normalized.contains("{{") || normalized.contains("}}") {
        return Err(
            "synthesized environment inventory markdown contains unresolved template placeholders"
                .to_string(),
        );
    }
    if LEGACY_NON_CANONICAL_PATH_CLAIMS
        .iter()
        .any(|claim| normalized.contains(claim))
    {
        return Err(
            "synthesized environment inventory markdown still contains legacy non-canonical path claims"
                .to_string(),
        );
    }
    if !normalized.contains(&format!("`{CANONICAL_ENVIRONMENT_INVENTORY_PATH}`")) {
        return Err(format!(
            "synthesized environment inventory markdown must reference `{CANONICAL_ENVIRONMENT_INVENTORY_PATH}` as the canonical file"
        ));
    }
    validate_required_heading_order_result(normalized, &REQUIRED_ENVIRONMENT_INVENTORY_HEADINGS)?;
    Ok(())
}

pub fn validate_required_heading_order_result(
    markdown: &str,
    required_headings: &[&str],
) -> Result<(), String> {
    let heading_lines = markdown
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let trimmed = line.trim_end();
            required_headings
                .contains(&trimmed)
                .then_some((index, trimmed))
        })
        .collect::<Vec<_>>();

    let mut previous = 0usize;
    for heading in required_headings {
        let positions = heading_lines
            .iter()
            .filter_map(|(index, line)| (*line == *heading).then_some(*index))
            .collect::<Vec<_>>();
        if positions.is_empty() {
            return Err(format!("missing required heading `{heading}`"));
        }
        if positions.len() != 1 {
            return Err(format!(
                "required heading `{heading}` must appear exactly once"
            ));
        }
        let position = positions[0];
        if position < previous {
            return Err(format!("required heading `{heading}` is out of order"));
        }
        previous = position;
    }

    Ok(())
}

#[allow(deprecated)]
#[deprecated(
    since = "0.1.1",
    note = "use `validate_environment_inventory_markdown`; deterministic authoring validates project-context references during repository preflight"
)]
pub fn validate_synthesized_environment_inventory_markdown(
    markdown: &str,
    expectations: EnvironmentInventoryValidationExpectations,
) -> Result<(), String> {
    validate_environment_inventory_markdown(markdown)?;

    let expected_project_context_ref_line = expectations.expected_project_context_ref_line();
    if !markdown.contains(expected_project_context_ref_line) {
        return Err(format!(
            "synthesized environment inventory markdown must include the exact project context reference line `{expected_project_context_ref_line}`"
        ));
    }

    Ok(())
}
