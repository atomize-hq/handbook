use serde::{Deserialize, Serialize};
use std::fmt::Write as _;

const PROJECT_CONTEXT_INPUTS_SCHEMA_VERSION: &str = "0.1.0";
const REQUIRED_PROJECT_CONTEXT_TOP_LEVEL_HEADINGS: [&str; 13] = [
    "## What this is",
    "## How to use this",
    "## 0) Project Summary (factual, 3–6 bullets)",
    "## 1) Operational Reality (the most important section)",
    "## 2) Project Classification Implications (planning guardrails)",
    "## 3) System Boundaries (what we own vs integrate with)",
    "## 4) Integrations & Contracts (top 1–5)",
    "## 5) Environments & Delivery",
    "## 6) Data Reality",
    "## 7) Repo / Codebase Reality (brownfield-friendly, but safe for greenfield)",
    "## 8) Constraints",
    "## 9) Known Unknowns (explicitly tracked)",
    "## 10) Update Triggers",
];
const REQUIRED_PROJECT_CONTEXT_SUBHEADINGS: [&str; 2] = [
    "### What we own",
    "### What we do NOT own (but may depend on)",
];
const REQUIRED_PROJECT_CONTEXT_METADATA_PREFIXES: [&str; 6] = [
    "> **File:** `PROJECT_CONTEXT.md`",
    "> **Created (UTC):**",
    "> **Owner:**",
    "> **Team:**",
    "> **Repo / Project:**",
    "> **Charter Ref:**",
];
const KNOWN_FAKE_PROJECT_CONTEXT_MARKERS: [&str; 10] = [
    "unknown-owner",
    "project-team",
    "unknown from local repo inspection; confirm before planning live changes.",
    "assume canonical project truth may influence live systems; verify before rollout.",
    "no uptime contract inferred locally; confirm service expectations explicitly.",
    "on-call and incident ownership are not established in repo-local evidence.",
    "canonical truth drift, undocumented integrations, and hidden runtime assumptions.",
    "delivery model is repository-specific and should be documented before production-impacting work.",
    "observability details are unknown from local inspection and should be recorded explicitly.",
    "migration history is unknown and should be recorded before migration-heavy work.",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectContextCoreErrorKind {
    MalformedStructuredInput,
    IncompleteStructuredInput,
    DeterministicRenderFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectContextCoreError {
    pub kind: ProjectContextCoreErrorKind,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectContextValidationError {
    pub summary: String,
}

impl std::fmt::Display for ProjectContextValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary)
    }
}

impl std::error::Error for ProjectContextValidationError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectContextStructuredInput {
    pub schema_version: String,
    pub project_name: String,
    pub owner: String,
    pub team: String,
    pub repo_or_project_ref: String,
    pub charter_ref: String,
    pub project_summary: ProjectContextSummaryInput,
    pub operational_reality: ProjectContextOperationalRealityInput,
    pub classification_implications: ProjectContextClassificationImplicationsInput,
    pub system_boundaries: ProjectContextSystemBoundariesInput,
    #[serde(default)]
    pub integrations: Vec<ProjectContextIntegrationInput>,
    pub environments_and_delivery: ProjectContextEnvironmentsAndDeliveryInput,
    pub data_reality: ProjectContextDataRealityInput,
    pub repo_codebase_reality: ProjectContextRepoCodebaseRealityInput,
    pub constraints: ProjectContextConstraintsInput,
    pub known_unknowns: Vec<ProjectContextKnownUnknownInput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectContextSummaryInput {
    pub what_this_project_is: String,
    pub primary_surface: String,
    pub primary_users: String,
    pub key_workflows: Vec<String>,
    pub non_goals: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectContextOperationalRealityInput {
    pub is_live_in_production_today: String,
    pub users: String,
    pub data_in_production: String,
    pub uptime_expectations: String,
    pub incident_on_call_reality: String,
    pub primary_risk_flags_present: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectContextClassificationImplicationsInput {
    pub project_type: String,
    pub backward_compatibility_required: String,
    pub backward_compatibility_notes: String,
    pub migration_planning_required: String,
    pub migration_planning_notes: String,
    pub deprecation_policy_exists: String,
    pub deprecation_policy_notes: String,
    pub rollout_controls_required: String,
    pub rollout_controls_notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectContextSystemBoundariesInput {
    pub owned_areas: Vec<String>,
    pub external_dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectContextIntegrationInput {
    pub name: String,
    pub integration_type: String,
    pub contract_surface: String,
    pub authentication_authorization: String,
    pub failure_mode_expectations: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectContextEnvironmentsAndDeliveryInput {
    pub environments_that_exist: String,
    pub deployment_model: String,
    pub ci_cd_reality: String,
    pub release_cadence: String,
    pub config_and_secrets: String,
    pub observability_stack: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectContextDataRealityInput {
    pub primary_data_stores: String,
    pub data_classification: String,
    pub retention_requirements: String,
    pub backups_disaster_recovery: String,
    pub existing_migrations_history: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectContextRepoCodebaseRealityInput {
    pub codebase_exists_today: bool,
    pub current_maturity: String,
    #[serde(default)]
    pub key_modules_or_areas: Vec<String>,
    pub known_constraints_from_existing_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectContextConstraintsInput {
    pub deadline_time_constraints: String,
    pub budget_constraints: String,
    pub must_use_or_prohibited_tech: String,
    pub compliance_legal_constraints: String,
    pub performance_constraints: String,
    pub security_constraints: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectContextKnownUnknownInput {
    pub item: String,
    pub owner: String,
    pub revisit_trigger: String,
}

pub fn parse_project_context_structured_input_yaml(
    yaml: &str,
) -> Result<ProjectContextStructuredInput, ProjectContextCoreError> {
    let parsed = serde_yaml_bw::from_str::<ProjectContextStructuredInput>(yaml).map_err(|err| {
        malformed_structured_input_error(format!(
            "structured project-context input is malformed: {err}"
        ))
    })?;
    validate_project_context_structured_input(&parsed)?;
    Ok(parsed)
}

pub fn validate_project_context_structured_input(
    input: &ProjectContextStructuredInput,
) -> Result<(), ProjectContextCoreError> {
    let mut issues = Vec::new();

    if input.schema_version.trim() != PROJECT_CONTEXT_INPUTS_SCHEMA_VERSION {
        issues.push(format!(
            "schema_version must be `{PROJECT_CONTEXT_INPUTS_SCHEMA_VERSION}`"
        ));
    }

    require_factual_text("project_name", &input.project_name, &mut issues);
    require_factual_text("owner", &input.owner, &mut issues);
    require_factual_text("team", &input.team, &mut issues);
    require_factual_text(
        "repo_or_project_ref",
        &input.repo_or_project_ref,
        &mut issues,
    );
    require_factual_text("charter_ref", &input.charter_ref, &mut issues);

    require_factual_text(
        "project_summary.what_this_project_is",
        &input.project_summary.what_this_project_is,
        &mut issues,
    );
    require_factual_text(
        "project_summary.primary_surface",
        &input.project_summary.primary_surface,
        &mut issues,
    );
    require_factual_text(
        "project_summary.primary_users",
        &input.project_summary.primary_users,
        &mut issues,
    );
    require_factual_list(
        "project_summary.key_workflows",
        &input.project_summary.key_workflows,
        Some(3),
        &mut issues,
    );
    require_factual_text(
        "project_summary.non_goals",
        &input.project_summary.non_goals,
        &mut issues,
    );

    require_factual_text(
        "operational_reality.is_live_in_production_today",
        &input.operational_reality.is_live_in_production_today,
        &mut issues,
    );
    require_factual_text(
        "operational_reality.users",
        &input.operational_reality.users,
        &mut issues,
    );
    require_factual_text(
        "operational_reality.data_in_production",
        &input.operational_reality.data_in_production,
        &mut issues,
    );
    require_factual_text(
        "operational_reality.uptime_expectations",
        &input.operational_reality.uptime_expectations,
        &mut issues,
    );
    require_factual_text(
        "operational_reality.incident_on_call_reality",
        &input.operational_reality.incident_on_call_reality,
        &mut issues,
    );
    require_factual_text(
        "operational_reality.primary_risk_flags_present",
        &input.operational_reality.primary_risk_flags_present,
        &mut issues,
    );

    require_factual_text(
        "classification_implications.project_type",
        &input.classification_implications.project_type,
        &mut issues,
    );
    require_factual_text(
        "classification_implications.backward_compatibility_required",
        &input
            .classification_implications
            .backward_compatibility_required,
        &mut issues,
    );
    require_factual_text(
        "classification_implications.backward_compatibility_notes",
        &input
            .classification_implications
            .backward_compatibility_notes,
        &mut issues,
    );
    require_factual_text(
        "classification_implications.migration_planning_required",
        &input
            .classification_implications
            .migration_planning_required,
        &mut issues,
    );
    require_factual_text(
        "classification_implications.migration_planning_notes",
        &input.classification_implications.migration_planning_notes,
        &mut issues,
    );
    require_factual_text(
        "classification_implications.deprecation_policy_exists",
        &input.classification_implications.deprecation_policy_exists,
        &mut issues,
    );
    require_factual_text(
        "classification_implications.deprecation_policy_notes",
        &input.classification_implications.deprecation_policy_notes,
        &mut issues,
    );
    require_factual_text(
        "classification_implications.rollout_controls_required",
        &input.classification_implications.rollout_controls_required,
        &mut issues,
    );
    require_factual_text(
        "classification_implications.rollout_controls_notes",
        &input.classification_implications.rollout_controls_notes,
        &mut issues,
    );

    require_factual_list(
        "system_boundaries.owned_areas",
        &input.system_boundaries.owned_areas,
        None,
        &mut issues,
    );
    require_factual_list(
        "system_boundaries.external_dependencies",
        &input.system_boundaries.external_dependencies,
        None,
        &mut issues,
    );

    if input.integrations.len() > 5 {
        issues.push("integrations must include at most 5 entries".to_string());
    }
    for (index, integration) in input.integrations.iter().enumerate() {
        let prefix = format!("integrations[{index}]");
        require_factual_text(&format!("{prefix}.name"), &integration.name, &mut issues);
        require_factual_text(
            &format!("{prefix}.integration_type"),
            &integration.integration_type,
            &mut issues,
        );
        require_factual_text(
            &format!("{prefix}.contract_surface"),
            &integration.contract_surface,
            &mut issues,
        );
        require_factual_text(
            &format!("{prefix}.authentication_authorization"),
            &integration.authentication_authorization,
            &mut issues,
        );
        require_factual_text(
            &format!("{prefix}.failure_mode_expectations"),
            &integration.failure_mode_expectations,
            &mut issues,
        );
    }

    require_factual_text(
        "environments_and_delivery.environments_that_exist",
        &input.environments_and_delivery.environments_that_exist,
        &mut issues,
    );
    require_factual_text(
        "environments_and_delivery.deployment_model",
        &input.environments_and_delivery.deployment_model,
        &mut issues,
    );
    require_factual_text(
        "environments_and_delivery.ci_cd_reality",
        &input.environments_and_delivery.ci_cd_reality,
        &mut issues,
    );
    require_factual_text(
        "environments_and_delivery.release_cadence",
        &input.environments_and_delivery.release_cadence,
        &mut issues,
    );
    require_factual_text(
        "environments_and_delivery.config_and_secrets",
        &input.environments_and_delivery.config_and_secrets,
        &mut issues,
    );
    require_factual_text(
        "environments_and_delivery.observability_stack",
        &input.environments_and_delivery.observability_stack,
        &mut issues,
    );

    require_factual_text(
        "data_reality.primary_data_stores",
        &input.data_reality.primary_data_stores,
        &mut issues,
    );
    require_factual_text(
        "data_reality.data_classification",
        &input.data_reality.data_classification,
        &mut issues,
    );
    require_factual_text(
        "data_reality.retention_requirements",
        &input.data_reality.retention_requirements,
        &mut issues,
    );
    require_factual_text(
        "data_reality.backups_disaster_recovery",
        &input.data_reality.backups_disaster_recovery,
        &mut issues,
    );
    require_factual_text(
        "data_reality.existing_migrations_history",
        &input.data_reality.existing_migrations_history,
        &mut issues,
    );

    require_factual_text(
        "repo_codebase_reality.current_maturity",
        &input.repo_codebase_reality.current_maturity,
        &mut issues,
    );
    require_factual_list_allow_empty(
        "repo_codebase_reality.key_modules_or_areas",
        &input.repo_codebase_reality.key_modules_or_areas,
        &mut issues,
    );
    require_factual_text(
        "repo_codebase_reality.known_constraints_from_existing_code",
        &input
            .repo_codebase_reality
            .known_constraints_from_existing_code,
        &mut issues,
    );

    require_factual_text(
        "constraints.deadline_time_constraints",
        &input.constraints.deadline_time_constraints,
        &mut issues,
    );
    require_factual_text(
        "constraints.budget_constraints",
        &input.constraints.budget_constraints,
        &mut issues,
    );
    require_factual_text(
        "constraints.must_use_or_prohibited_tech",
        &input.constraints.must_use_or_prohibited_tech,
        &mut issues,
    );
    require_factual_text(
        "constraints.compliance_legal_constraints",
        &input.constraints.compliance_legal_constraints,
        &mut issues,
    );
    require_factual_text(
        "constraints.performance_constraints",
        &input.constraints.performance_constraints,
        &mut issues,
    );
    require_factual_text(
        "constraints.security_constraints",
        &input.constraints.security_constraints,
        &mut issues,
    );

    if input.known_unknowns.is_empty() {
        issues.push("known_unknowns must include at least one tracked uncertainty".to_string());
    }
    for (index, unknown) in input.known_unknowns.iter().enumerate() {
        let prefix = format!("known_unknowns[{index}]");
        require_factual_text(&format!("{prefix}.item"), &unknown.item, &mut issues);
        require_factual_text(&format!("{prefix}.owner"), &unknown.owner, &mut issues);
        require_factual_text(
            &format!("{prefix}.revisit_trigger"),
            &unknown.revisit_trigger,
            &mut issues,
        );
    }

    collect_render_safety_issues(input, &mut issues);

    if issues.is_empty() {
        Ok(())
    } else {
        Err(incomplete_structured_input_error(format!(
            "structured project-context input is incomplete: {}",
            issues.join("; ")
        )))
    }
}

pub fn render_project_context_markdown(
    input: &ProjectContextStructuredInput,
    now_utc: &str,
) -> Result<String, ProjectContextCoreError> {
    validate_project_context_structured_input(input)?;

    let normalized = normalized_project_context_structured_input(input);
    let mut out = String::new();
    writeln!(out, "# Project Context — {}", normalized.project_name).unwrap();
    writeln!(out).unwrap();
    writeln!(out, "> **File:** `PROJECT_CONTEXT.md`  ").unwrap();
    writeln!(out, "> **Created (UTC):** {}  ", now_utc).unwrap();
    writeln!(out, "> **Owner:** {}  ", normalized.owner).unwrap();
    writeln!(out, "> **Team:** {}  ", normalized.team).unwrap();
    writeln!(
        out,
        "> **Repo / Project:** {}  ",
        normalized.repo_or_project_ref
    )
    .unwrap();
    writeln!(out, "> **Charter Ref:** {}", normalized.charter_ref).unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## What this is").unwrap();
    writeln!(
        out,
        "A factual snapshot of project reality used to prevent incorrect planning assumptions."
    )
    .unwrap();
    writeln!(
        out,
        "(\"What exists, what's live, what constraints are true, what we integrate with.\")"
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## How to use this").unwrap();
    writeln!(
        out,
        "- Use this document to ground **feature specs**, **phase plans**, **slice specs**, and **execution** in reality."
    )
    .unwrap();
    writeln!(
        out,
        "- If something is unknown, record it explicitly in **Known Unknowns** and avoid planning around invented constraints."
    )
    .unwrap();
    writeln!(
        out,
        "- This doc should change rarely; update it when reality changes (first prod launch, new integration, major migration, etc.)."
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(out, "---").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "## 0) Project Summary (factual, 3–6 bullets)").unwrap();
    writeln!(
        out,
        "- **What this project is:** {}",
        normalized.project_summary.what_this_project_is
    )
    .unwrap();
    writeln!(
        out,
        "- **Primary surface:** {}",
        normalized.project_summary.primary_surface
    )
    .unwrap();
    writeln!(
        out,
        "- **Primary users:** {}",
        normalized.project_summary.primary_users
    )
    .unwrap();
    writeln!(out, "- **Key workflows (top 1–3):**").unwrap();
    push_bullets(&mut out, &normalized.project_summary.key_workflows);
    writeln!(
        out,
        "- **Non-goals (optional):** {}",
        render_inline_text_or_default(&normalized.project_summary.non_goals, "None")
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(
        out,
        "## 1) Operational Reality (the most important section)"
    )
    .unwrap();
    writeln!(
        out,
        "- **Is anything live in production today?** {}",
        normalized.operational_reality.is_live_in_production_today
    )
    .unwrap();
    writeln!(out, "- **Users:** {}", normalized.operational_reality.users).unwrap();
    writeln!(
        out,
        "- **Data in production:** {}",
        normalized.operational_reality.data_in_production
    )
    .unwrap();
    writeln!(
        out,
        "- **Uptime expectations / SLA:** {}",
        normalized.operational_reality.uptime_expectations
    )
    .unwrap();
    writeln!(
        out,
        "- **Incident/on-call reality:** {}",
        normalized.operational_reality.incident_on_call_reality
    )
    .unwrap();
    writeln!(
        out,
        "- **Primary risk flags present:** {}",
        normalized.operational_reality.primary_risk_flags_present
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(
        out,
        "## 2) Project Classification Implications (planning guardrails)"
    )
    .unwrap();
    writeln!(
        out,
        "> This prevents unnecessary migration/back-compat planning."
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "- **Project type (from Charter):** {}",
        normalized.classification_implications.project_type
    )
    .unwrap();
    writeln!(
        out,
        "- **Backward compatibility required?** {}",
        normalized
            .classification_implications
            .backward_compatibility_required
    )
    .unwrap();
    writeln!(
        out,
        "  - Notes: {}",
        normalized
            .classification_implications
            .backward_compatibility_notes
    )
    .unwrap();
    writeln!(
        out,
        "- **Migration planning required?** {}",
        normalized
            .classification_implications
            .migration_planning_required
    )
    .unwrap();
    writeln!(
        out,
        "  - Notes: {}",
        normalized
            .classification_implications
            .migration_planning_notes
    )
    .unwrap();
    writeln!(
        out,
        "- **Deprecation policy exists?** {}",
        normalized
            .classification_implications
            .deprecation_policy_exists
    )
    .unwrap();
    writeln!(
        out,
        "  - Notes: {}",
        normalized
            .classification_implications
            .deprecation_policy_notes
    )
    .unwrap();
    writeln!(
        out,
        "- **Rollout controls required (flags/canary/staged)?** {}",
        normalized
            .classification_implications
            .rollout_controls_required
    )
    .unwrap();
    writeln!(
        out,
        "  - Notes: {}",
        normalized
            .classification_implications
            .rollout_controls_notes
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(
        out,
        "## 3) System Boundaries (what we own vs integrate with)"
    )
    .unwrap();
    writeln!(out, "### What we own").unwrap();
    push_bullets(&mut out, &normalized.system_boundaries.owned_areas);
    writeln!(out).unwrap();
    writeln!(out, "### What we do NOT own (but may depend on)").unwrap();
    push_bullets(
        &mut out,
        &normalized.system_boundaries.external_dependencies,
    );
    writeln!(out).unwrap();

    writeln!(out, "## 4) Integrations & Contracts (top 1–5)").unwrap();
    writeln!(
        out,
        "> Enumerate only what matters. If none, write \"None.\"."
    )
    .unwrap();
    writeln!(out).unwrap();
    if normalized.integrations.is_empty() {
        writeln!(out, "None.").unwrap();
    } else {
        for (index, integration) in normalized.integrations.iter().enumerate() {
            let label = index + 1;
            writeln!(out, "- **Integration {label}:** {}", integration.name).unwrap();
            writeln!(out, "  - Type: {}", integration.integration_type).unwrap();
            writeln!(
                out,
                "  - Contract surface: {}",
                integration.contract_surface
            )
            .unwrap();
            writeln!(
                out,
                "  - Authentication/authorization: {}",
                integration.authentication_authorization
            )
            .unwrap();
            writeln!(
                out,
                "  - Failure mode expectations: {}",
                integration.failure_mode_expectations
            )
            .unwrap();
        }
    }
    writeln!(out).unwrap();

    writeln!(out, "## 5) Environments & Delivery").unwrap();
    writeln!(
        out,
        "- **Environments that exist:** {}",
        normalized.environments_and_delivery.environments_that_exist
    )
    .unwrap();
    writeln!(
        out,
        "- **Deployment model:** {}",
        normalized.environments_and_delivery.deployment_model
    )
    .unwrap();
    writeln!(
        out,
        "- **CI/CD reality:** {}",
        normalized.environments_and_delivery.ci_cd_reality
    )
    .unwrap();
    writeln!(
        out,
        "- **Release cadence:** {}",
        normalized.environments_and_delivery.release_cadence
    )
    .unwrap();
    writeln!(
        out,
        "- **Config & secrets:** {}",
        normalized.environments_and_delivery.config_and_secrets
    )
    .unwrap();
    writeln!(
        out,
        "- **Observability stack (if any):** {}",
        normalized.environments_and_delivery.observability_stack
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(out, "## 6) Data Reality").unwrap();
    writeln!(out, "> Keep this high level; just enough for planning.").unwrap();
    writeln!(out).unwrap();
    writeln!(
        out,
        "- **Primary data stores:** {}",
        normalized.data_reality.primary_data_stores
    )
    .unwrap();
    writeln!(
        out,
        "- **Data classification:** {}",
        normalized.data_reality.data_classification
    )
    .unwrap();
    writeln!(
        out,
        "- **Retention requirements:** {}",
        normalized.data_reality.retention_requirements
    )
    .unwrap();
    writeln!(
        out,
        "- **Backups / DR reality:** {}",
        normalized.data_reality.backups_disaster_recovery
    )
    .unwrap();
    writeln!(
        out,
        "- **Existing migrations/history:** {}",
        normalized.data_reality.existing_migrations_history
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(
        out,
        "## 7) Repo / Codebase Reality (brownfield-friendly, but safe for greenfield)"
    )
    .unwrap();
    writeln!(
        out,
        "- **Codebase exists today?** {}",
        if normalized.repo_codebase_reality.codebase_exists_today {
            "yes"
        } else {
            "no"
        }
    )
    .unwrap();
    writeln!(
        out,
        "- **If yes:** current maturity: {}",
        normalized.repo_codebase_reality.current_maturity
    )
    .unwrap();
    writeln!(out, "- **Key modules/areas to be aware of:**").unwrap();
    if normalized
        .repo_codebase_reality
        .key_modules_or_areas
        .is_empty()
    {
        writeln!(out, "  - None.").unwrap();
    } else {
        push_indented_bullets(
            &mut out,
            &normalized.repo_codebase_reality.key_modules_or_areas,
        );
    }
    writeln!(
        out,
        "- **Known constraints from existing code:** {}",
        normalized
            .repo_codebase_reality
            .known_constraints_from_existing_code
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(out, "## 8) Constraints").unwrap();
    writeln!(
        out,
        "- **Deadline/time constraints:** {}",
        normalized.constraints.deadline_time_constraints
    )
    .unwrap();
    writeln!(
        out,
        "- **Budget constraints:** {}",
        normalized.constraints.budget_constraints
    )
    .unwrap();
    writeln!(
        out,
        "- **Must-use tech / prohibited tech:** {}",
        normalized.constraints.must_use_or_prohibited_tech
    )
    .unwrap();
    writeln!(
        out,
        "- **Compliance/legal constraints:** {}",
        normalized.constraints.compliance_legal_constraints
    )
    .unwrap();
    writeln!(
        out,
        "- **Performance constraints:** {}",
        normalized.constraints.performance_constraints
    )
    .unwrap();
    writeln!(
        out,
        "- **Security constraints:** {}",
        normalized.constraints.security_constraints
    )
    .unwrap();
    writeln!(out).unwrap();

    writeln!(out, "## 9) Known Unknowns (explicitly tracked)").unwrap();
    writeln!(
        out,
        "> List anything that's uncertain but would change planning decisions."
    )
    .unwrap();
    writeln!(out).unwrap();
    for unknown in &normalized.known_unknowns {
        writeln!(
            out,
            "- {} (owner: {}, revisit trigger: {})",
            unknown.item, unknown.owner, unknown.revisit_trigger
        )
        .unwrap();
    }
    writeln!(out).unwrap();

    writeln!(out, "## 10) Update Triggers").unwrap();
    writeln!(out, "Update this doc when:").unwrap();
    writeln!(out, "- first production launch").unwrap();
    writeln!(out, "- first external users").unwrap();
    writeln!(out, "- new major integration/contract introduced").unwrap();
    writeln!(out, "- major migration/modernization begins").unwrap();
    writeln!(
        out,
        "- posture changes in Charter require new operational constraints"
    )
    .unwrap();

    let markdown = out.trim_end().to_string();
    validate_project_context_markdown(&markdown).map_err(|err| {
        deterministic_render_error(format!(
            "rendered project-context markdown failed structural validation: {}",
            err.summary
        ))
    })?;
    Ok(markdown)
}

pub fn validate_project_context_markdown(
    markdown: &str,
) -> Result<(), ProjectContextValidationError> {
    let normalized = markdown.trim();
    if normalized.is_empty() {
        return Err(validation_error("project context markdown was empty"));
    }
    if !normalized.starts_with("# Project Context — ") {
        return Err(validation_error(
            "project context markdown must start with `# Project Context — `",
        ));
    }
    if normalized.contains("{{") || normalized.contains("}}") {
        return Err(validation_error(
            "project context markdown contains unresolved template placeholders",
        ));
    }
    if normalized.contains("<!--") || normalized.contains("-->") {
        return Err(validation_error(
            "project context markdown must not contain template comments",
        ));
    }

    for prefix in REQUIRED_PROJECT_CONTEXT_METADATA_PREFIXES {
        if !normalized
            .lines()
            .any(|line| line.trim_end().starts_with(prefix))
        {
            return Err(validation_error(format!(
                "project context markdown is missing required metadata line starting with `{prefix}`"
            )));
        }
    }

    validate_required_heading_order_result(
        normalized,
        &REQUIRED_PROJECT_CONTEXT_TOP_LEVEL_HEADINGS,
    )
    .map_err(validation_error)?;
    validate_required_heading_order_result(normalized, &REQUIRED_PROJECT_CONTEXT_SUBHEADINGS)
        .map_err(validation_error)?;
    validate_known_fake_project_context_markers(normalized).map_err(validation_error)?;

    Ok(())
}

pub fn normalized_project_context_structured_input(
    input: &ProjectContextStructuredInput,
) -> ProjectContextStructuredInput {
    let mut normalized = input.clone();

    normalized.project_name = normalize_project_context_text(&normalized.project_name);
    normalized.owner = normalize_project_context_text(&normalized.owner);
    normalized.team = normalize_project_context_text(&normalized.team);
    normalized.repo_or_project_ref =
        normalize_project_context_text(&normalized.repo_or_project_ref);
    normalized.charter_ref = normalize_project_context_text(&normalized.charter_ref);

    normalized.project_summary.what_this_project_is =
        normalize_project_context_text(&normalized.project_summary.what_this_project_is);
    normalized.project_summary.primary_surface =
        normalize_project_context_text(&normalized.project_summary.primary_surface);
    normalized.project_summary.primary_users =
        normalize_project_context_text(&normalized.project_summary.primary_users);
    normalize_string_list(&mut normalized.project_summary.key_workflows);
    normalized.project_summary.non_goals =
        normalize_project_context_text(&normalized.project_summary.non_goals);

    normalized.operational_reality.is_live_in_production_today =
        normalize_project_context_text(&normalized.operational_reality.is_live_in_production_today);
    normalized.operational_reality.users =
        normalize_project_context_text(&normalized.operational_reality.users);
    normalized.operational_reality.data_in_production =
        normalize_project_context_text(&normalized.operational_reality.data_in_production);
    normalized.operational_reality.uptime_expectations =
        normalize_project_context_text(&normalized.operational_reality.uptime_expectations);
    normalized.operational_reality.incident_on_call_reality =
        normalize_project_context_text(&normalized.operational_reality.incident_on_call_reality);
    normalized.operational_reality.primary_risk_flags_present =
        normalize_project_context_text(&normalized.operational_reality.primary_risk_flags_present);

    normalized.classification_implications.project_type =
        normalize_project_context_text(&normalized.classification_implications.project_type);
    normalized
        .classification_implications
        .backward_compatibility_required = normalize_project_context_text(
        &normalized
            .classification_implications
            .backward_compatibility_required,
    );
    normalized
        .classification_implications
        .backward_compatibility_notes = normalize_project_context_text(
        &normalized
            .classification_implications
            .backward_compatibility_notes,
    );
    normalized
        .classification_implications
        .migration_planning_required = normalize_project_context_text(
        &normalized
            .classification_implications
            .migration_planning_required,
    );
    normalized
        .classification_implications
        .migration_planning_notes = normalize_project_context_text(
        &normalized
            .classification_implications
            .migration_planning_notes,
    );
    normalized
        .classification_implications
        .deprecation_policy_exists = normalize_project_context_text(
        &normalized
            .classification_implications
            .deprecation_policy_exists,
    );
    normalized
        .classification_implications
        .deprecation_policy_notes = normalize_project_context_text(
        &normalized
            .classification_implications
            .deprecation_policy_notes,
    );
    normalized
        .classification_implications
        .rollout_controls_required = normalize_project_context_text(
        &normalized
            .classification_implications
            .rollout_controls_required,
    );
    normalized
        .classification_implications
        .rollout_controls_notes = normalize_project_context_text(
        &normalized
            .classification_implications
            .rollout_controls_notes,
    );

    normalize_string_list(&mut normalized.system_boundaries.owned_areas);
    normalize_string_list(&mut normalized.system_boundaries.external_dependencies);

    for integration in &mut normalized.integrations {
        integration.name = normalize_project_context_text(&integration.name);
        integration.integration_type =
            normalize_project_context_text(&integration.integration_type);
        integration.contract_surface =
            normalize_project_context_text(&integration.contract_surface);
        integration.authentication_authorization =
            normalize_project_context_text(&integration.authentication_authorization);
        integration.failure_mode_expectations =
            normalize_project_context_text(&integration.failure_mode_expectations);
    }

    normalized.environments_and_delivery.environments_that_exist = normalize_project_context_text(
        &normalized.environments_and_delivery.environments_that_exist,
    );
    normalized.environments_and_delivery.deployment_model =
        normalize_project_context_text(&normalized.environments_and_delivery.deployment_model);
    normalized.environments_and_delivery.ci_cd_reality =
        normalize_project_context_text(&normalized.environments_and_delivery.ci_cd_reality);
    normalized.environments_and_delivery.release_cadence =
        normalize_project_context_text(&normalized.environments_and_delivery.release_cadence);
    normalized.environments_and_delivery.config_and_secrets =
        normalize_project_context_text(&normalized.environments_and_delivery.config_and_secrets);
    normalized.environments_and_delivery.observability_stack =
        normalize_project_context_text(&normalized.environments_and_delivery.observability_stack);

    normalized.data_reality.primary_data_stores =
        normalize_project_context_text(&normalized.data_reality.primary_data_stores);
    normalized.data_reality.data_classification =
        normalize_project_context_text(&normalized.data_reality.data_classification);
    normalized.data_reality.retention_requirements =
        normalize_project_context_text(&normalized.data_reality.retention_requirements);
    normalized.data_reality.backups_disaster_recovery =
        normalize_project_context_text(&normalized.data_reality.backups_disaster_recovery);
    normalized.data_reality.existing_migrations_history =
        normalize_project_context_text(&normalized.data_reality.existing_migrations_history);

    normalized.repo_codebase_reality.current_maturity =
        normalize_project_context_text(&normalized.repo_codebase_reality.current_maturity);
    normalize_string_list(&mut normalized.repo_codebase_reality.key_modules_or_areas);
    normalized
        .repo_codebase_reality
        .known_constraints_from_existing_code = normalize_project_context_text(
        &normalized
            .repo_codebase_reality
            .known_constraints_from_existing_code,
    );

    normalized.constraints.deadline_time_constraints =
        normalize_project_context_text(&normalized.constraints.deadline_time_constraints);
    normalized.constraints.budget_constraints =
        normalize_project_context_text(&normalized.constraints.budget_constraints);
    normalized.constraints.must_use_or_prohibited_tech =
        normalize_project_context_text(&normalized.constraints.must_use_or_prohibited_tech);
    normalized.constraints.compliance_legal_constraints =
        normalize_project_context_text(&normalized.constraints.compliance_legal_constraints);
    normalized.constraints.performance_constraints =
        normalize_project_context_text(&normalized.constraints.performance_constraints);
    normalized.constraints.security_constraints =
        normalize_project_context_text(&normalized.constraints.security_constraints);

    for unknown in &mut normalized.known_unknowns {
        unknown.item = normalize_project_context_text(&unknown.item);
        unknown.owner = normalize_project_context_text(&unknown.owner);
        unknown.revisit_trigger = normalize_project_context_text(&unknown.revisit_trigger);
    }

    normalized
}

pub fn normalize_project_context_text(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn collect_render_safety_issues(
    input: &ProjectContextStructuredInput,
    issues: &mut Vec<String>,
) {
    require_render_safe_text("project_name", &input.project_name, issues);
    require_render_safe_text("owner", &input.owner, issues);
    require_render_safe_text("team", &input.team, issues);
    require_render_safe_text("repo_or_project_ref", &input.repo_or_project_ref, issues);
    require_render_safe_text("charter_ref", &input.charter_ref, issues);

    require_render_safe_text(
        "project_summary.what_this_project_is",
        &input.project_summary.what_this_project_is,
        issues,
    );
    require_render_safe_text(
        "project_summary.primary_surface",
        &input.project_summary.primary_surface,
        issues,
    );
    require_render_safe_text(
        "project_summary.primary_users",
        &input.project_summary.primary_users,
        issues,
    );
    require_render_safe_list(
        "project_summary.key_workflows",
        &input.project_summary.key_workflows,
        issues,
    );
    require_render_safe_text(
        "project_summary.non_goals",
        &input.project_summary.non_goals,
        issues,
    );

    require_render_safe_text(
        "operational_reality.is_live_in_production_today",
        &input.operational_reality.is_live_in_production_today,
        issues,
    );
    require_render_safe_text(
        "operational_reality.users",
        &input.operational_reality.users,
        issues,
    );
    require_render_safe_text(
        "operational_reality.data_in_production",
        &input.operational_reality.data_in_production,
        issues,
    );
    require_render_safe_text(
        "operational_reality.uptime_expectations",
        &input.operational_reality.uptime_expectations,
        issues,
    );
    require_render_safe_text(
        "operational_reality.incident_on_call_reality",
        &input.operational_reality.incident_on_call_reality,
        issues,
    );
    require_render_safe_text(
        "operational_reality.primary_risk_flags_present",
        &input.operational_reality.primary_risk_flags_present,
        issues,
    );

    require_render_safe_text(
        "classification_implications.project_type",
        &input.classification_implications.project_type,
        issues,
    );
    require_render_safe_text(
        "classification_implications.backward_compatibility_required",
        &input
            .classification_implications
            .backward_compatibility_required,
        issues,
    );
    require_render_safe_text(
        "classification_implications.backward_compatibility_notes",
        &input
            .classification_implications
            .backward_compatibility_notes,
        issues,
    );
    require_render_safe_text(
        "classification_implications.migration_planning_required",
        &input
            .classification_implications
            .migration_planning_required,
        issues,
    );
    require_render_safe_text(
        "classification_implications.migration_planning_notes",
        &input.classification_implications.migration_planning_notes,
        issues,
    );
    require_render_safe_text(
        "classification_implications.deprecation_policy_exists",
        &input.classification_implications.deprecation_policy_exists,
        issues,
    );
    require_render_safe_text(
        "classification_implications.deprecation_policy_notes",
        &input.classification_implications.deprecation_policy_notes,
        issues,
    );
    require_render_safe_text(
        "classification_implications.rollout_controls_required",
        &input.classification_implications.rollout_controls_required,
        issues,
    );
    require_render_safe_text(
        "classification_implications.rollout_controls_notes",
        &input.classification_implications.rollout_controls_notes,
        issues,
    );

    require_render_safe_list(
        "system_boundaries.owned_areas",
        &input.system_boundaries.owned_areas,
        issues,
    );
    require_render_safe_list(
        "system_boundaries.external_dependencies",
        &input.system_boundaries.external_dependencies,
        issues,
    );

    for (index, integration) in input.integrations.iter().enumerate() {
        let prefix = format!("integrations[{index}]");
        require_render_safe_text(&format!("{prefix}.name"), &integration.name, issues);
        require_render_safe_text(
            &format!("{prefix}.integration_type"),
            &integration.integration_type,
            issues,
        );
        require_render_safe_text(
            &format!("{prefix}.contract_surface"),
            &integration.contract_surface,
            issues,
        );
        require_render_safe_text(
            &format!("{prefix}.authentication_authorization"),
            &integration.authentication_authorization,
            issues,
        );
        require_render_safe_text(
            &format!("{prefix}.failure_mode_expectations"),
            &integration.failure_mode_expectations,
            issues,
        );
    }

    require_render_safe_text(
        "environments_and_delivery.environments_that_exist",
        &input.environments_and_delivery.environments_that_exist,
        issues,
    );
    require_render_safe_text(
        "environments_and_delivery.deployment_model",
        &input.environments_and_delivery.deployment_model,
        issues,
    );
    require_render_safe_text(
        "environments_and_delivery.ci_cd_reality",
        &input.environments_and_delivery.ci_cd_reality,
        issues,
    );
    require_render_safe_text(
        "environments_and_delivery.release_cadence",
        &input.environments_and_delivery.release_cadence,
        issues,
    );
    require_render_safe_text(
        "environments_and_delivery.config_and_secrets",
        &input.environments_and_delivery.config_and_secrets,
        issues,
    );
    require_render_safe_text(
        "environments_and_delivery.observability_stack",
        &input.environments_and_delivery.observability_stack,
        issues,
    );

    require_render_safe_text(
        "data_reality.primary_data_stores",
        &input.data_reality.primary_data_stores,
        issues,
    );
    require_render_safe_text(
        "data_reality.data_classification",
        &input.data_reality.data_classification,
        issues,
    );
    require_render_safe_text(
        "data_reality.retention_requirements",
        &input.data_reality.retention_requirements,
        issues,
    );
    require_render_safe_text(
        "data_reality.backups_disaster_recovery",
        &input.data_reality.backups_disaster_recovery,
        issues,
    );
    require_render_safe_text(
        "data_reality.existing_migrations_history",
        &input.data_reality.existing_migrations_history,
        issues,
    );

    require_render_safe_text(
        "repo_codebase_reality.current_maturity",
        &input.repo_codebase_reality.current_maturity,
        issues,
    );
    require_render_safe_list(
        "repo_codebase_reality.key_modules_or_areas",
        &input.repo_codebase_reality.key_modules_or_areas,
        issues,
    );
    require_render_safe_text(
        "repo_codebase_reality.known_constraints_from_existing_code",
        &input
            .repo_codebase_reality
            .known_constraints_from_existing_code,
        issues,
    );

    require_render_safe_text(
        "constraints.deadline_time_constraints",
        &input.constraints.deadline_time_constraints,
        issues,
    );
    require_render_safe_text(
        "constraints.budget_constraints",
        &input.constraints.budget_constraints,
        issues,
    );
    require_render_safe_text(
        "constraints.must_use_or_prohibited_tech",
        &input.constraints.must_use_or_prohibited_tech,
        issues,
    );
    require_render_safe_text(
        "constraints.compliance_legal_constraints",
        &input.constraints.compliance_legal_constraints,
        issues,
    );
    require_render_safe_text(
        "constraints.performance_constraints",
        &input.constraints.performance_constraints,
        issues,
    );
    require_render_safe_text(
        "constraints.security_constraints",
        &input.constraints.security_constraints,
        issues,
    );

    for (index, unknown) in input.known_unknowns.iter().enumerate() {
        let prefix = format!("known_unknowns[{index}]");
        require_render_safe_text(&format!("{prefix}.item"), &unknown.item, issues);
        require_render_safe_text(&format!("{prefix}.owner"), &unknown.owner, issues);
        require_render_safe_text(
            &format!("{prefix}.revisit_trigger"),
            &unknown.revisit_trigger,
            issues,
        );
    }
}

pub fn validate_known_fake_project_context_markers(markdown: &str) -> Result<(), String> {
    let lower = markdown.to_ascii_lowercase();
    for marker in KNOWN_FAKE_PROJECT_CONTEXT_MARKERS {
        if lower.contains(marker) {
            return Err(format!(
                "project context markdown contains known fabricated placeholder text: `{marker}`"
            ));
        }
    }
    Ok(())
}

fn malformed_structured_input_error(summary: String) -> ProjectContextCoreError {
    ProjectContextCoreError {
        kind: ProjectContextCoreErrorKind::MalformedStructuredInput,
        summary,
    }
}

fn incomplete_structured_input_error(summary: String) -> ProjectContextCoreError {
    ProjectContextCoreError {
        kind: ProjectContextCoreErrorKind::IncompleteStructuredInput,
        summary,
    }
}

fn deterministic_render_error(summary: String) -> ProjectContextCoreError {
    ProjectContextCoreError {
        kind: ProjectContextCoreErrorKind::DeterministicRenderFailed,
        summary,
    }
}

fn validation_error(summary: impl Into<String>) -> ProjectContextValidationError {
    ProjectContextValidationError {
        summary: summary.into(),
    }
}

fn normalize_string_list(values: &mut Vec<String>) {
    for value in values {
        *value = normalize_project_context_text(value);
    }
}

fn require_factual_text(field: &str, value: &str, issues: &mut Vec<String>) {
    let normalized = normalize_project_context_text(value);
    if normalized.is_empty() {
        issues.push(format!("{field} must not be empty"));
        return;
    }
    if is_unusably_vague_project_context_text(&normalized) {
        issues.push(format!("{field} must be explicit, not a placeholder"));
    }
}

fn require_factual_list(
    field: &str,
    values: &[String],
    max_len: Option<usize>,
    issues: &mut Vec<String>,
) {
    if values.is_empty() {
        issues.push(format!("{field} must include at least one non-empty value"));
        return;
    }

    if let Some(limit) = max_len {
        if values.len() > limit {
            issues.push(format!("{field} must include at most {limit} entries"));
        }
    }

    let mut saw_value = false;
    for (index, value) in values.iter().enumerate() {
        let normalized = normalize_project_context_text(value);
        if normalized.is_empty() {
            issues.push(format!("{field}[{index}] must not be empty"));
            continue;
        }
        if is_unusably_vague_project_context_text(&normalized) {
            issues.push(format!(
                "{field}[{index}] must be explicit, not a placeholder"
            ));
            continue;
        }
        saw_value = true;
    }

    if !saw_value {
        issues.push(format!("{field} must include at least one non-empty value"));
    }
}

fn require_factual_list_allow_empty(field: &str, values: &[String], issues: &mut Vec<String>) {
    for (index, value) in values.iter().enumerate() {
        let normalized = normalize_project_context_text(value);
        if normalized.is_empty() {
            issues.push(format!("{field}[{index}] must not be empty"));
            continue;
        }
        if is_unusably_vague_project_context_text(&normalized) {
            issues.push(format!(
                "{field}[{index}] must be explicit, not a placeholder"
            ));
        }
    }
}

fn is_unusably_vague_project_context_text(value: &str) -> bool {
    let lower = value.trim().to_ascii_lowercase();
    if lower.is_empty() {
        return true;
    }

    if is_allowed_explicit_default(&lower) {
        return false;
    }

    matches!(
        lower.as_str(),
        "idk"
            | "i don't know"
            | "dont know"
            | "unknown"
            | "n/a"
            | "na"
            | "tbd"
            | "todo"
            | "unsure"
            | "not sure"
            | "various"
            | "stuff"
            | "things"
            | "misc"
            | "whatever"
    )
}

fn is_allowed_explicit_default(lower: &str) -> bool {
    lower == "none"
        || lower == "none."
        || lower == "none declared"
        || lower.starts_with("not applicable")
        || (lower.starts_with("unknown") && lower.contains("known unknown"))
}

fn require_render_safe_list(field: &str, values: &[String], issues: &mut Vec<String>) {
    for (index, value) in values.iter().enumerate() {
        require_render_safe_text(&format!("{field}[{index}]"), value, issues);
    }
}

fn require_render_safe_text(field: &str, value: &str, issues: &mut Vec<String>) {
    if value.trim().is_empty() {
        return;
    }

    let normalized = normalize_project_context_text(value);
    let trimmed = normalized.trim_start();
    let has_unsafe_prefix = trimmed.starts_with('#')
        || trimmed.starts_with('>')
        || trimmed.starts_with("```")
        || trimmed.starts_with("~~~")
        || trimmed.starts_with("<!--")
        || trimmed.starts_with("- ")
        || trimmed.starts_with("* ");
    let has_unsafe_token = normalized.contains("```")
        || normalized.contains("~~~")
        || normalized.contains("<!--")
        || normalized.contains("-->");

    if has_unsafe_prefix || has_unsafe_token {
        issues.push(format!(
            "{field} must not include markdown control syntax such as headings, blockquotes, fences, comments, or list markers"
        ));
    }
}

fn render_inline_text_or_default(value: &str, default: &str) -> String {
    let normalized = normalize_project_context_text(value);
    if normalized.is_empty() {
        default.to_string()
    } else {
        normalized
    }
}

fn push_bullets(out: &mut String, values: &[String]) {
    for value in values {
        writeln!(out, "- {}", value).unwrap();
    }
}

fn push_indented_bullets(out: &mut String, values: &[String]) {
    for value in values {
        writeln!(out, "  - {}", value).unwrap();
    }
}

fn validate_required_heading_order_result(
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
