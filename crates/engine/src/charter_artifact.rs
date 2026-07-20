use crate::{
    parse_definition_yaml, ArtifactProfileDecision, ArtifactRegistryValidationError,
    DefinitionFingerprint, RegistryLoadErrorKind, ResolvedProfileDecisions,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::Write as _;

const CHARTER_INSTANCE_ID: &str = "project_authority";
const CHARTER_KIND_REF: &str = "handbook.artifact-kind.project-authority@1.1.0";
const CHARTER_SCHEMA_REF: &str = "handbook.schemas.artifacts.project-authority@1.1.0";
pub(crate) const SELECTED_CHARTER_CANONICAL_PATH: &str = ".handbook/project/charter.yaml";
const REQUIRED_DIMENSIONS: [&str; 9] = [
    "speed_vs_quality",
    "type_safety_static_analysis",
    "testing_rigor",
    "scalability_performance",
    "reliability_operability",
    "security_privacy",
    "observability",
    "dx_tooling_automation",
    "ux_polish_api_usability",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalCharter {
    pub schema_id: String,
    pub schema_version: String,
    pub record_id: String,
    pub project: CanonicalCharterProject,
    pub posture: CanonicalCharterPosture,
    pub domains: Vec<CanonicalCharterDomain>,
    pub policy: CanonicalCharterPolicy,
    pub governance: CanonicalCharterGovernance,
    pub engineering_posture: CanonicalCharterEngineeringPosture,
    pub debt: CanonicalCharterDebt,
    pub decision_records: CanonicalCharterDecisionRecords,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalCharterProject {
    pub name: String,
    pub classification: String,
    pub team_size: u32,
    pub users: String,
    pub expected_lifetime: String,
    pub surfaces: Vec<String>,
    pub runtime_environments: Vec<String>,
    pub constraints: CanonicalCharterConstraints,
    pub operational_reality: CanonicalCharterOperationalReality,
    pub default_implications: CanonicalCharterDefaultImplications,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalCharterConstraints {
    pub deadline: String,
    pub budget: String,
    pub experience_notes: String,
    pub must_use_tech: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalCharterOperationalReality {
    pub in_production_today: bool,
    pub prod_users_or_data: String,
    pub external_contracts_to_preserve: Vec<String>,
    pub uptime_expectations: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalCharterDefaultImplications {
    pub backward_compatibility: String,
    pub migration_planning: String,
    pub rollout_controls: String,
    pub deprecation_policy: String,
    pub observability_threshold: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalCharterPosture {
    pub rubric_scale: String,
    pub baseline_level: u8,
    pub baseline_rationale: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalCharterDomain {
    pub name: String,
    pub blast_radius: String,
    pub touches: Vec<String>,
    pub constraints: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalCharterPolicy {
    pub revision: String,
    pub authority_statement: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalCharterGovernance {
    pub decision_authority: Vec<String>,
    pub required_approvals: Vec<String>,
    pub exception_policy: String,
    pub exception_process: CanonicalCharterExceptionProcess,
    pub review_triggers: Vec<String>,
    pub reassessment_triggers: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalCharterExceptionProcess {
    pub approvers: Vec<String>,
    pub record_location: String,
    pub minimum_fields: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalCharterEngineeringPosture {
    pub dimensions: Vec<CanonicalCharterDimension>,
    pub red_lines: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalCharterDimension {
    pub dimension_id: String,
    pub level_override: Option<u8>,
    pub default_stance: String,
    pub raise_the_bar_triggers: Vec<String>,
    pub allowed_shortcuts: Vec<String>,
    pub red_lines: Vec<String>,
    pub domain_overrides: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalCharterDebt {
    pub system: String,
    pub labels: Vec<String>,
    pub review_cadence: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalCharterDecisionRecords {
    pub enabled: bool,
    pub path: String,
    pub format: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharterArtifactErrorKind {
    SourceLimitExceeded,
    DuplicateKey,
    SyntaxError,
    NonObjectRoot,
    SelectedDecisionMissing,
    SelectedContractMismatch,
    StructuralValidationFailed,
    TypedDecodeFailed,
    SemanticValidationFailed,
    RenderedViewRefused,
    SerializationFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterArtifactError {
    kind: CharterArtifactErrorKind,
    detail: String,
}

impl CharterArtifactError {
    fn new(kind: CharterArtifactErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> CharterArtifactErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

pub fn parse_canonical_charter(
    decisions: &ResolvedProfileDecisions,
    source_bytes: &[u8],
) -> Result<CanonicalCharter, CharterArtifactError> {
    let value = parse_definition_yaml(source_bytes).map_err(|error| {
        let kind = match error.kind() {
            RegistryLoadErrorKind::SourceLimitExceeded => {
                CharterArtifactErrorKind::SourceLimitExceeded
            }
            RegistryLoadErrorKind::DuplicateKey => CharterArtifactErrorKind::DuplicateKey,
            _ => CharterArtifactErrorKind::SyntaxError,
        };
        CharterArtifactError::new(kind, "canonical Charter YAML was refused")
    })?;
    if !value.is_object() {
        return Err(CharterArtifactError::new(
            CharterArtifactErrorKind::NonObjectRoot,
            "canonical Charter must have an object root",
        ));
    }

    let decision = selected_charter_decision(decisions)?;
    decisions
        .registry()
        .validate_json(decision.instance_id(), &value)
        .map_err(|error| match error {
            ArtifactRegistryValidationError::UnknownArtifactInstance
            | ArtifactRegistryValidationError::Structural(_) => CharterArtifactError::new(
                CharterArtifactErrorKind::StructuralValidationFailed,
                "canonical Charter failed selected-schema validation",
            ),
        })?;

    let record: CanonicalCharter = serde_json::from_value(value.clone()).map_err(|_| {
        CharterArtifactError::new(
            CharterArtifactErrorKind::TypedDecodeFailed,
            "canonical Charter could not be closed-decoded",
        )
    })?;
    let roundtrip = serde_json::to_value(&record).map_err(|_| {
        CharterArtifactError::new(
            CharterArtifactErrorKind::TypedDecodeFailed,
            "canonical Charter could not be closed-encoded",
        )
    })?;
    if roundtrip != value {
        return Err(CharterArtifactError::new(
            CharterArtifactErrorKind::TypedDecodeFailed,
            "canonical Charter typed truth disagrees with validated JSON",
        ));
    }
    validate_canonical_charter_semantics(&record)?;
    Ok(record)
}

pub(crate) fn selected_charter_decision(
    decisions: &ResolvedProfileDecisions,
) -> Result<&ArtifactProfileDecision, CharterArtifactError> {
    let decision = decisions
        .artifact_decisions()
        .iter()
        .find(|decision| decision.instance_id().as_str() == CHARTER_INSTANCE_ID)
        .ok_or_else(|| {
            CharterArtifactError::new(
                CharterArtifactErrorKind::SelectedDecisionMissing,
                "selected Charter decision is missing",
            )
        })?;
    let instance = decisions
        .registry()
        .instance(decision.instance_id())
        .ok_or_else(|| {
            CharterArtifactError::new(
                CharterArtifactErrorKind::SelectedDecisionMissing,
                "selected Charter instance is missing",
            )
        })?;
    let kind = decisions
        .registry()
        .kind(instance.kind_ref())
        .ok_or_else(|| {
            CharterArtifactError::new(
                CharterArtifactErrorKind::SelectedContractMismatch,
                "selected Charter kind is missing",
            )
        })?;
    if decision.kind_ref().as_str() != CHARTER_KIND_REF
        || instance.kind_ref().as_str() != CHARTER_KIND_REF
        || kind.canonical_schema_ref().as_str() != CHARTER_SCHEMA_REF
        || decision.canonical_path() != SELECTED_CHARTER_CANONICAL_PATH
    {
        return Err(CharterArtifactError::new(
            CharterArtifactErrorKind::SelectedContractMismatch,
            "selected Charter kind, schema, or path does not match the fixed contract",
        ));
    }
    Ok(decision)
}

pub fn validate_canonical_charter_semantics(
    record: &CanonicalCharter,
) -> Result<(), CharterArtifactError> {
    let actual_dimensions = record
        .engineering_posture
        .dimensions
        .iter()
        .map(|dimension| dimension.dimension_id.as_str())
        .collect::<Vec<_>>();
    if actual_dimensions != REQUIRED_DIMENSIONS {
        return Err(semantic_refusal(
            "engineering_posture.dimensions must contain the exact ordered nine dimensions",
        ));
    }

    let mut domain_names = BTreeSet::new();
    for domain in &record.domains {
        let identity = normalize_text(&domain.name).to_lowercase();
        if !domain_names.insert(identity) {
            return Err(semantic_refusal("domain names must be unique"));
        }
    }

    let mut required_text = vec![
        record.project.name.as_str(),
        record.project.constraints.experience_notes.as_str(),
        record.policy.revision.as_str(),
        record.policy.authority_statement.as_str(),
        record.governance.exception_policy.as_str(),
        record.governance.exception_process.record_location.as_str(),
        record.debt.system.as_str(),
        record.debt.review_cadence.as_str(),
    ];
    required_text.extend(record.posture.baseline_rationale.iter().map(String::as_str));
    required_text.extend(
        record
            .governance
            .decision_authority
            .iter()
            .map(String::as_str),
    );
    required_text.extend(
        record
            .governance
            .required_approvals
            .iter()
            .map(String::as_str),
    );
    required_text.extend(record.governance.review_triggers.iter().map(String::as_str));
    required_text.extend(
        record
            .governance
            .reassessment_triggers
            .iter()
            .map(String::as_str),
    );
    required_text.extend(
        record
            .engineering_posture
            .red_lines
            .iter()
            .map(String::as_str),
    );
    for domain in &record.domains {
        required_text.push(domain.name.as_str());
        required_text.push(domain.blast_radius.as_str());
    }
    for dimension in &record.engineering_posture.dimensions {
        required_text.push(dimension.default_stance.as_str());
        required_text.extend(dimension.raise_the_bar_triggers.iter().map(String::as_str));
        required_text.extend(dimension.allowed_shortcuts.iter().map(String::as_str));
        required_text.extend(dimension.red_lines.iter().map(String::as_str));
    }
    if required_text.iter().any(|value| is_unusably_vague(value)) {
        return Err(semantic_refusal(
            "canonical Charter contains empty, placeholder, or vague required text",
        ));
    }
    if all_rendered_strings(record)
        .iter()
        .any(|value| !is_render_safe(value))
    {
        return Err(CharterArtifactError::new(
            CharterArtifactErrorKind::RenderedViewRefused,
            "canonical Charter contains Markdown control syntax or forbidden controls",
        ));
    }
    Ok(())
}

fn semantic_refusal(detail: impl Into<String>) -> CharterArtifactError {
    CharterArtifactError::new(CharterArtifactErrorKind::SemanticValidationFailed, detail)
}

fn normalize_text(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_unusably_vague(value: &str) -> bool {
    let normalized = normalize_text(value);
    if normalized.is_empty() {
        return true;
    }
    matches!(
        normalized.to_ascii_lowercase().as_str(),
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
            | "good quality"
            | "good"
            | "quality"
            | "standard"
            | "normal"
            | "stuff"
            | "things"
            | "misc"
            | "various"
            | "whatever"
    )
}

fn is_render_safe(value: &str) -> bool {
    if value
        .chars()
        .any(|character| matches!(character as u32, 0x01..=0x08 | 0x0b..=0x0c | 0x0e..=0x1f | 0x7f))
    {
        return false;
    }
    let normalized = normalize_text(value);
    let trimmed = normalized.trim_start();
    !(trimmed.starts_with('#')
        || trimmed.starts_with('>')
        || trimmed.starts_with("```")
        || trimmed.starts_with("~~~")
        || trimmed.starts_with("<!--")
        || trimmed.starts_with("- ")
        || trimmed.starts_with("* ")
        || normalized.contains("```")
        || normalized.contains("~~~")
        || normalized.contains("<!--")
        || normalized.contains("-->"))
}

fn all_rendered_strings(record: &CanonicalCharter) -> Vec<&str> {
    let mut values = vec![
        record.project.name.as_str(),
        record.project.constraints.deadline.as_str(),
        record.project.constraints.budget.as_str(),
        record.project.constraints.experience_notes.as_str(),
        record
            .project
            .operational_reality
            .prod_users_or_data
            .as_str(),
        record
            .project
            .operational_reality
            .uptime_expectations
            .as_str(),
        record.policy.revision.as_str(),
        record.policy.authority_statement.as_str(),
        record.governance.exception_policy.as_str(),
        record.governance.exception_process.record_location.as_str(),
        record.debt.system.as_str(),
        record.debt.review_cadence.as_str(),
        record.decision_records.path.as_str(),
        record.decision_records.format.as_str(),
    ];
    values.extend(
        record
            .project
            .constraints
            .must_use_tech
            .iter()
            .map(String::as_str),
    );
    values.extend(
        record
            .project
            .operational_reality
            .external_contracts_to_preserve
            .iter()
            .map(String::as_str),
    );
    values.extend(record.posture.baseline_rationale.iter().map(String::as_str));
    for domain in &record.domains {
        values.push(domain.name.as_str());
        values.push(domain.blast_radius.as_str());
        values.extend(domain.touches.iter().map(String::as_str));
        values.extend(domain.constraints.iter().map(String::as_str));
    }
    values.extend(
        record
            .governance
            .decision_authority
            .iter()
            .map(String::as_str),
    );
    values.extend(
        record
            .governance
            .required_approvals
            .iter()
            .map(String::as_str),
    );
    values.extend(record.governance.review_triggers.iter().map(String::as_str));
    values.extend(
        record
            .governance
            .reassessment_triggers
            .iter()
            .map(String::as_str),
    );
    values.extend(
        record
            .governance
            .exception_process
            .approvers
            .iter()
            .map(String::as_str),
    );
    values.extend(
        record
            .governance
            .exception_process
            .minimum_fields
            .iter()
            .map(String::as_str),
    );
    values.extend(
        record
            .engineering_posture
            .red_lines
            .iter()
            .map(String::as_str),
    );
    for dimension in &record.engineering_posture.dimensions {
        values.push(dimension.default_stance.as_str());
        values.extend(dimension.raise_the_bar_triggers.iter().map(String::as_str));
        values.extend(dimension.allowed_shortcuts.iter().map(String::as_str));
        values.extend(dimension.red_lines.iter().map(String::as_str));
        values.extend(dimension.domain_overrides.iter().map(String::as_str));
    }
    values.extend(record.debt.labels.iter().map(String::as_str));
    values
}

pub fn serialize_canonical_charter(
    decisions: &ResolvedProfileDecisions,
    record: &CanonicalCharter,
) -> Result<Vec<u8>, CharterArtifactError> {
    let decision = selected_charter_decision(decisions)?;
    let value = serde_json::to_value(record).map_err(|_| {
        CharterArtifactError::new(
            CharterArtifactErrorKind::SerializationFailed,
            "canonical Charter could not be closed-encoded",
        )
    })?;
    decisions
        .registry()
        .validate_json(decision.instance_id(), &value)
        .map_err(|_| {
            CharterArtifactError::new(
                CharterArtifactErrorKind::StructuralValidationFailed,
                "canonical Charter failed selected-schema validation",
            )
        })?;
    validate_canonical_charter_semantics(record)?;
    let bytes = emit_canonical_charter(record)?;
    let reparsed = parse_canonical_charter(decisions, &bytes)?;
    if reparsed != *record {
        return Err(CharterArtifactError::new(
            CharterArtifactErrorKind::SerializationFailed,
            "canonical Charter emitter changed typed truth",
        ));
    }
    Ok(bytes)
}

pub fn charter_source_fingerprint(source_bytes: &[u8]) -> DefinitionFingerprint {
    DefinitionFingerprint::from_bytes(source_bytes)
}

pub fn charter_rendered_fingerprint(rendered_bytes: &[u8]) -> DefinitionFingerprint {
    DefinitionFingerprint::from_bytes(rendered_bytes)
}

fn emit_canonical_charter(record: &CanonicalCharter) -> Result<Vec<u8>, CharterArtifactError> {
    let mut output = String::new();
    emit_string(&mut output, 0, "schema_id", &record.schema_id)?;
    emit_string(&mut output, 0, "schema_version", &record.schema_version)?;
    emit_string(&mut output, 0, "record_id", &record.record_id)?;
    emit_object_key(&mut output, 0, "project");
    emit_string(&mut output, 2, "name", &record.project.name)?;
    emit_string(
        &mut output,
        2,
        "classification",
        &record.project.classification,
    )?;
    emit_u32(&mut output, 2, "team_size", record.project.team_size);
    emit_string(&mut output, 2, "users", &record.project.users)?;
    emit_string(
        &mut output,
        2,
        "expected_lifetime",
        &record.project.expected_lifetime,
    )?;
    emit_string_sequence(&mut output, 2, "surfaces", &record.project.surfaces)?;
    emit_string_sequence(
        &mut output,
        2,
        "runtime_environments",
        &record.project.runtime_environments,
    )?;
    emit_object_key(&mut output, 2, "constraints");
    emit_string(
        &mut output,
        4,
        "deadline",
        &record.project.constraints.deadline,
    )?;
    emit_string(&mut output, 4, "budget", &record.project.constraints.budget)?;
    emit_string(
        &mut output,
        4,
        "experience_notes",
        &record.project.constraints.experience_notes,
    )?;
    emit_string_sequence(
        &mut output,
        4,
        "must_use_tech",
        &record.project.constraints.must_use_tech,
    )?;
    emit_object_key(&mut output, 2, "operational_reality");
    emit_bool(
        &mut output,
        4,
        "in_production_today",
        record.project.operational_reality.in_production_today,
    );
    emit_string(
        &mut output,
        4,
        "prod_users_or_data",
        &record.project.operational_reality.prod_users_or_data,
    )?;
    emit_string_sequence(
        &mut output,
        4,
        "external_contracts_to_preserve",
        &record
            .project
            .operational_reality
            .external_contracts_to_preserve,
    )?;
    emit_string(
        &mut output,
        4,
        "uptime_expectations",
        &record.project.operational_reality.uptime_expectations,
    )?;
    emit_object_key(&mut output, 2, "default_implications");
    emit_string(
        &mut output,
        4,
        "backward_compatibility",
        &record.project.default_implications.backward_compatibility,
    )?;
    emit_string(
        &mut output,
        4,
        "migration_planning",
        &record.project.default_implications.migration_planning,
    )?;
    emit_string(
        &mut output,
        4,
        "rollout_controls",
        &record.project.default_implications.rollout_controls,
    )?;
    emit_string(
        &mut output,
        4,
        "deprecation_policy",
        &record.project.default_implications.deprecation_policy,
    )?;
    emit_string(
        &mut output,
        4,
        "observability_threshold",
        &record.project.default_implications.observability_threshold,
    )?;

    emit_object_key(&mut output, 0, "posture");
    emit_string(&mut output, 2, "rubric_scale", &record.posture.rubric_scale)?;
    emit_u8(
        &mut output,
        2,
        "baseline_level",
        record.posture.baseline_level,
    );
    emit_string_sequence(
        &mut output,
        2,
        "baseline_rationale",
        &record.posture.baseline_rationale,
    )?;

    if record.domains.is_empty() {
        emit_empty_sequence(&mut output, 0, "domains");
    } else {
        emit_object_sequence_key(&mut output, 0, "domains");
        for domain in &record.domains {
            emit_first_string(&mut output, 2, "name", &domain.name)?;
            emit_string(&mut output, 4, "blast_radius", &domain.blast_radius)?;
            emit_string_sequence(&mut output, 4, "touches", &domain.touches)?;
            emit_string_sequence(&mut output, 4, "constraints", &domain.constraints)?;
        }
    }

    emit_object_key(&mut output, 0, "policy");
    emit_string(&mut output, 2, "revision", &record.policy.revision)?;
    emit_string(
        &mut output,
        2,
        "authority_statement",
        &record.policy.authority_statement,
    )?;

    emit_object_key(&mut output, 0, "governance");
    emit_string_sequence(
        &mut output,
        2,
        "decision_authority",
        &record.governance.decision_authority,
    )?;
    emit_string_sequence(
        &mut output,
        2,
        "required_approvals",
        &record.governance.required_approvals,
    )?;
    emit_string(
        &mut output,
        2,
        "exception_policy",
        &record.governance.exception_policy,
    )?;
    emit_object_key(&mut output, 2, "exception_process");
    emit_string_sequence(
        &mut output,
        4,
        "approvers",
        &record.governance.exception_process.approvers,
    )?;
    emit_string(
        &mut output,
        4,
        "record_location",
        &record.governance.exception_process.record_location,
    )?;
    emit_string_sequence(
        &mut output,
        4,
        "minimum_fields",
        &record.governance.exception_process.minimum_fields,
    )?;
    emit_string_sequence(
        &mut output,
        2,
        "review_triggers",
        &record.governance.review_triggers,
    )?;
    emit_string_sequence(
        &mut output,
        2,
        "reassessment_triggers",
        &record.governance.reassessment_triggers,
    )?;

    emit_object_key(&mut output, 0, "engineering_posture");
    emit_object_sequence_key(&mut output, 2, "dimensions");
    for dimension in &record.engineering_posture.dimensions {
        emit_first_string(&mut output, 4, "dimension_id", &dimension.dimension_id)?;
        emit_optional_u8(&mut output, 6, "level_override", dimension.level_override);
        emit_string(&mut output, 6, "default_stance", &dimension.default_stance)?;
        emit_string_sequence(
            &mut output,
            6,
            "raise_the_bar_triggers",
            &dimension.raise_the_bar_triggers,
        )?;
        emit_string_sequence(
            &mut output,
            6,
            "allowed_shortcuts",
            &dimension.allowed_shortcuts,
        )?;
        emit_string_sequence(&mut output, 6, "red_lines", &dimension.red_lines)?;
        emit_string_sequence(
            &mut output,
            6,
            "domain_overrides",
            &dimension.domain_overrides,
        )?;
    }
    emit_string_sequence(
        &mut output,
        2,
        "red_lines",
        &record.engineering_posture.red_lines,
    )?;

    emit_object_key(&mut output, 0, "debt");
    emit_string(&mut output, 2, "system", &record.debt.system)?;
    emit_string_sequence(&mut output, 2, "labels", &record.debt.labels)?;
    emit_string(
        &mut output,
        2,
        "review_cadence",
        &record.debt.review_cadence,
    )?;

    emit_object_key(&mut output, 0, "decision_records");
    emit_bool(&mut output, 2, "enabled", record.decision_records.enabled);
    emit_string(&mut output, 2, "path", &record.decision_records.path)?;
    emit_string(&mut output, 2, "format", &record.decision_records.format)?;
    Ok(output.into_bytes())
}

fn indentation(output: &mut String, width: usize) {
    for _ in 0..width {
        output.push(' ');
    }
}

fn emit_object_key(output: &mut String, indent: usize, key: &str) {
    indentation(output, indent);
    output.push_str(key);
    output.push_str(":\n");
}

fn emit_object_sequence_key(output: &mut String, indent: usize, key: &str) {
    emit_object_key(output, indent, key);
}

fn emit_empty_sequence(output: &mut String, indent: usize, key: &str) {
    indentation(output, indent);
    output.push_str(key);
    output.push_str(": []\n");
}

fn emit_string(
    output: &mut String,
    indent: usize,
    key: &str,
    value: &str,
) -> Result<(), CharterArtifactError> {
    indentation(output, indent);
    output.push_str(key);
    output.push_str(": ");
    output.push_str(&json_string(value)?);
    output.push('\n');
    Ok(())
}

fn emit_first_string(
    output: &mut String,
    indent: usize,
    key: &str,
    value: &str,
) -> Result<(), CharterArtifactError> {
    indentation(output, indent);
    output.push_str("- ");
    output.push_str(key);
    output.push_str(": ");
    output.push_str(&json_string(value)?);
    output.push('\n');
    Ok(())
}

fn emit_string_sequence(
    output: &mut String,
    indent: usize,
    key: &str,
    values: &[String],
) -> Result<(), CharterArtifactError> {
    if values.is_empty() {
        emit_empty_sequence(output, indent, key);
        return Ok(());
    }
    emit_object_key(output, indent, key);
    for value in values {
        indentation(output, indent + 2);
        output.push_str("- ");
        output.push_str(&json_string(value)?);
        output.push('\n');
    }
    Ok(())
}

fn emit_u8(output: &mut String, indent: usize, key: &str, value: u8) {
    indentation(output, indent);
    writeln!(output, "{key}: {value}").expect("String writes are infallible");
}

fn emit_u32(output: &mut String, indent: usize, key: &str, value: u32) {
    indentation(output, indent);
    writeln!(output, "{key}: {value}").expect("String writes are infallible");
}

fn emit_bool(output: &mut String, indent: usize, key: &str, value: bool) {
    indentation(output, indent);
    writeln!(output, "{key}: {value}").expect("String writes are infallible");
}

fn emit_optional_u8(output: &mut String, indent: usize, key: &str, value: Option<u8>) {
    indentation(output, indent);
    match value {
        Some(value) => writeln!(output, "{key}: {value}"),
        None => writeln!(output, "{key}: null"),
    }
    .expect("String writes are infallible");
}

fn json_string(value: &str) -> Result<String, CharterArtifactError> {
    serde_json::to_string(value).map_err(|_| {
        CharterArtifactError::new(
            CharterArtifactErrorKind::SerializationFailed,
            "canonical Charter string serialization failed",
        )
    })
}

#[derive(Clone, Copy)]
struct RubricLevel {
    level: u8,
    label: &'static str,
    meaning: &'static str,
}

const RUBRIC_LEVELS: [RubricLevel; 5] = [
    RubricLevel {
        level: 1,
        label: "Exploratory",
        meaning: "throwaway ok; optimize learning; minimal gates",
    },
    RubricLevel {
        level: 2,
        label: "Prototype",
        meaning: "demoable/internal use; some structure; still speed-first",
    },
    RubricLevel {
        level: 3,
        label: "Product",
        meaning: "real users; balanced; maintainability matters",
    },
    RubricLevel {
        level: 4,
        label: "Production",
        meaning: "GA/customer-facing; strong quality/reliability/security defaults",
    },
    RubricLevel {
        level: 5,
        label: "Hardened",
        meaning: "critical/regulated/high blast radius; strict gates; defense-in-depth",
    },
];

const DIMENSION_METADATA: [(&str, &str, &str); 9] = [
    (
        "speed_vs_quality",
        "1) Speed vs Quality",
        "Speed vs Quality",
    ),
    (
        "type_safety_static_analysis",
        "2) Type safety / static analysis",
        "Type safety / static analysis",
    ),
    ("testing_rigor", "3) Testing rigor", "Testing rigor"),
    (
        "scalability_performance",
        "4) Scalability & performance",
        "Scalability & performance",
    ),
    (
        "reliability_operability",
        "5) Reliability & operability",
        "Reliability & operability",
    ),
    (
        "security_privacy",
        "6) Security & privacy",
        "Security & privacy",
    ),
    ("observability", "7) Observability", "Observability"),
    (
        "dx_tooling_automation",
        "8) Developer experience (DX)",
        "Developer experience (DX)",
    ),
    (
        "ux_polish_api_usability",
        "9) UX polish / API usability",
        "UX polish / API usability",
    ),
];

pub fn render_canonical_charter_markdown(
    record: &CanonicalCharter,
) -> Result<Vec<u8>, CharterArtifactError> {
    validate_canonical_charter_semantics(record)?;
    if record.schema_id != "handbook.artifact.project-authority"
        || record.schema_version != "1.1"
        || record.posture.rubric_scale != "1-5"
        || !(1..=5).contains(&record.posture.baseline_level)
    {
        return Err(CharterArtifactError::new(
            CharterArtifactErrorKind::StructuralValidationFailed,
            "canonical Charter constants or baseline are invalid",
        ));
    }

    let project_name = normalize_text(&record.project.name);
    let baseline = rubric_level(record.posture.baseline_level)?;
    let surfaces = record
        .project
        .surfaces
        .iter()
        .map(|value| display_token(value))
        .collect::<Vec<_>>()
        .join(", ");
    let runtimes = record
        .project
        .runtime_environments
        .iter()
        .map(|value| display_token(value))
        .collect::<Vec<_>>()
        .join(", ");
    let must_use_tech =
        inline_list_or_default(&record.project.constraints.must_use_tech, "none declared");
    let external_contracts = inline_list_or_default(
        &record
            .project
            .operational_reality
            .external_contracts_to_preserve,
        "none declared",
    );
    let debt_labels = inline_list_or_default(&record.debt.labels, "none");
    let production_state = if record.project.operational_reality.in_production_today {
        "yes"
    } else {
        "no"
    };

    let mut out = String::new();
    writeln!(out, "# Engineering Charter — {project_name}").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## What this is").unwrap();
    writeln!(
        out,
        "This engineering charter is the canonical decision surface for {project_name}. It turns the project's stated baseline posture, domain constraints, and dimension-specific guardrails into the default rules for day-to-day engineering work."
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## How to use this charter").unwrap();
    writeln!(
        out,
        "- Default to the project baseline of level {} ({}) unless a dimension or domain section explicitly raises or lowers the bar.",
        baseline.level, baseline.label
    )
    .unwrap();
    writeln!(out, "- Use the dimension sections below to decide when to raise rigor, what shortcuts remain acceptable, and which red lines are non-negotiable.").unwrap();
    writeln!(
        out,
        "- Record approved exceptions in {} before deviating from these defaults.",
        normalize_text(&record.governance.exception_process.record_location)
    )
    .unwrap();
    writeln!(out, "- Revisit this charter when the project classification, risk profile, or operating environment changes.").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## Rubric: 1–5 rigor levels").unwrap();
    writeln!(out, "| Level | Label | Meaning |").unwrap();
    writeln!(out, "|------:|-------|---------|").unwrap();
    for level in RUBRIC_LEVELS {
        writeln!(
            out,
            "| {} | {} | {} |",
            level.level, level.label, level.meaning
        )
        .unwrap();
    }

    writeln!(out).unwrap();
    writeln!(out, "## Project baseline posture").unwrap();
    writeln!(
        out,
        "- **Baseline level:** {} ({})",
        baseline.level, baseline.label
    )
    .unwrap();
    writeln!(out, "- **Baseline rationale:**").unwrap();
    for rationale in &record.posture.baseline_rationale {
        writeln!(out, "  - {}", normalize_text(rationale)).unwrap();
    }
    writeln!(
        out,
        "- **Project classification:** {}",
        display_token(&record.project.classification)
    )
    .unwrap();
    writeln!(out, "- **Users:** {}", display_token(&record.project.users)).unwrap();
    writeln!(
        out,
        "- **Expected lifetime:** {}",
        display_token(&record.project.expected_lifetime)
    )
    .unwrap();
    writeln!(out, "- **Team size:** {}", record.project.team_size).unwrap();
    writeln!(out, "- **Surfaces:** {surfaces}").unwrap();
    writeln!(out, "- **Runtime environments:** {runtimes}").unwrap();
    writeln!(
        out,
        "- **Deadline:** {}",
        inline_text_or_default(&record.project.constraints.deadline, "none declared")
    )
    .unwrap();
    writeln!(
        out,
        "- **Budget notes:** {}",
        inline_text_or_default(&record.project.constraints.budget, "none declared")
    )
    .unwrap();
    writeln!(
        out,
        "- **Experience notes:** {}",
        normalize_text(&record.project.constraints.experience_notes)
    )
    .unwrap();
    writeln!(out, "- **Required technologies:** {must_use_tech}").unwrap();
    writeln!(out, "- **In production today:** {production_state}").unwrap();
    writeln!(
        out,
        "- **Production users or data:** {}",
        inline_text_or_default(
            &record.project.operational_reality.prod_users_or_data,
            "none declared"
        )
    )
    .unwrap();
    writeln!(
        out,
        "- **External contracts to preserve:** {external_contracts}"
    )
    .unwrap();
    writeln!(
        out,
        "- **Uptime expectations:** {}",
        inline_text_or_default(
            &record.project.operational_reality.uptime_expectations,
            "none declared"
        )
    )
    .unwrap();
    writeln!(
        out,
        "- **Backward compatibility default:** {}",
        display_token(&record.project.default_implications.backward_compatibility)
    )
    .unwrap();
    writeln!(
        out,
        "- **Migration planning default:** {}",
        display_token(&record.project.default_implications.migration_planning)
    )
    .unwrap();
    writeln!(
        out,
        "- **Rollout controls default:** {}",
        display_token(&record.project.default_implications.rollout_controls)
    )
    .unwrap();
    writeln!(
        out,
        "- **Deprecation policy default:** {}",
        display_token(&record.project.default_implications.deprecation_policy)
    )
    .unwrap();
    writeln!(
        out,
        "- **Observability threshold default:** {}",
        display_token(&record.project.default_implications.observability_threshold)
    )
    .unwrap();

    writeln!(out).unwrap();
    writeln!(out, "## Domains / areas (optional overrides)").unwrap();
    if record.domains.is_empty() {
        writeln!(out, "None — baseline applies everywhere.").unwrap();
    } else {
        writeln!(
            out,
            "These domains add context for where the baseline posture needs extra care."
        )
        .unwrap();
        writeln!(out).unwrap();
        for domain in &record.domains {
            writeln!(out, "### {}", normalize_text(&domain.name)).unwrap();
            writeln!(
                out,
                "- **Blast radius:** {}",
                normalize_text(&domain.blast_radius)
            )
            .unwrap();
            writeln!(
                out,
                "- **Touches / trust boundary:** {}",
                inline_list_or_default(&domain.touches, "none declared")
            )
            .unwrap();
            writeln!(
                out,
                "- **Special constraints:** {}",
                inline_list_or_default(&domain.constraints, "none declared")
            )
            .unwrap();
            writeln!(out, "- **Default posture:** baseline applies unless a dimension override below says otherwise.").unwrap();
            writeln!(out).unwrap();
        }
    }

    writeln!(out, "## Posture at a glance (quick scan)").unwrap();
    writeln!(out, "| Dimension | Default level (1–5) | Notes / intent |").unwrap();
    writeln!(out, "|---|---:|---|").unwrap();
    for (dimension, (_, _, table_label)) in record
        .engineering_posture
        .dimensions
        .iter()
        .zip(DIMENSION_METADATA)
    {
        let level = dimension
            .level_override
            .unwrap_or(record.posture.baseline_level);
        writeln!(
            out,
            "| {table_label} | {level} | {} |",
            escape_table_cell(&normalize_text(&dimension.default_stance))
        )
        .unwrap();
    }

    writeln!(out).unwrap();
    writeln!(out, "## Dimensions (details + guardrails)").unwrap();
    writeln!(
        out,
        "Each dimension inherits the project baseline unless an explicit level is set below."
    )
    .unwrap();
    writeln!(out).unwrap();
    for (dimension, (_, title, _)) in record
        .engineering_posture
        .dimensions
        .iter()
        .zip(DIMENSION_METADATA)
    {
        let level = dimension
            .level_override
            .unwrap_or(record.posture.baseline_level);
        let rubric = rubric_level(level)?;
        writeln!(out, "### {title}").unwrap();
        writeln!(
            out,
            "- **Default stance (level):** {} ({})",
            rubric.level, rubric.label
        )
        .unwrap();
        writeln!(
            out,
            "- **Intent:** {}",
            normalize_text(&dimension.default_stance)
        )
        .unwrap();
        writeln!(out, "**Raise the bar when:**").unwrap();
        push_bullets(&mut out, &dimension.raise_the_bar_triggers);
        writeln!(out).unwrap();
        writeln!(out, "**Allowed shortcuts when:**").unwrap();
        push_bullets(&mut out, &dimension.allowed_shortcuts);
        writeln!(out).unwrap();
        writeln!(out, "**Non-negotiables / red lines:**").unwrap();
        push_bullets(&mut out, &dimension.red_lines);
        writeln!(out).unwrap();
        writeln!(out, "**Domain overrides (if any):**").unwrap();
        if dimension.domain_overrides.is_empty() {
            writeln!(out, "- None — baseline applies.").unwrap();
        } else {
            push_bullets(&mut out, &dimension.domain_overrides);
        }
        writeln!(out).unwrap();
    }

    writeln!(out, "## Cross-cutting red lines (global non-negotiables)").unwrap();
    push_bullets(&mut out, &record.engineering_posture.red_lines);
    writeln!(out).unwrap();
    writeln!(out, "## Constitutional policy and governance").unwrap();
    writeln!(
        out,
        "- **Policy revision:** {}",
        normalize_text(&record.policy.revision)
    )
    .unwrap();
    writeln!(
        out,
        "- **Authority statement:** {}",
        normalize_text(&record.policy.authority_statement)
    )
    .unwrap();
    writeln!(
        out,
        "- **Decision authority:** {}",
        inline_list_or_default(&record.governance.decision_authority, "none declared")
    )
    .unwrap();
    writeln!(
        out,
        "- **Required approvals:** {}",
        inline_list_or_default(&record.governance.required_approvals, "none declared")
    )
    .unwrap();
    writeln!(
        out,
        "- **Exception policy:** {}",
        normalize_text(&record.governance.exception_policy)
    )
    .unwrap();
    writeln!(
        out,
        "- **Review triggers:** {}",
        inline_list_or_default(&record.governance.review_triggers, "none declared")
    )
    .unwrap();
    writeln!(
        out,
        "- **Reassessment triggers:** {}",
        inline_list_or_default(&record.governance.reassessment_triggers, "none declared")
    )
    .unwrap();

    writeln!(out).unwrap();
    writeln!(out, "## Exceptions / overrides process").unwrap();
    writeln!(
        out,
        "- **Approvers:** {}",
        inline_list_or_default(
            &record.governance.exception_process.approvers,
            "none declared"
        )
    )
    .unwrap();
    writeln!(
        out,
        "- **Record location:** {}",
        normalize_text(&record.governance.exception_process.record_location)
    )
    .unwrap();
    writeln!(out, "- **Minimum required fields:**").unwrap();
    for field in &record.governance.exception_process.minimum_fields {
        writeln!(out, "  - {}", normalize_text(field)).unwrap();
    }

    writeln!(out).unwrap();
    writeln!(out, "## Debt tracking expectations").unwrap();
    writeln!(
        out,
        "- **Tracking system:** {}",
        normalize_text(&record.debt.system)
    )
    .unwrap();
    writeln!(out, "- **Labels:** {debt_labels}").unwrap();
    writeln!(
        out,
        "- **Review cadence:** {}",
        normalize_text(&record.debt.review_cadence)
    )
    .unwrap();

    writeln!(out).unwrap();
    writeln!(out, "## Decision Records (ADRs): how to use this charter").unwrap();
    if record.decision_records.enabled {
        writeln!(
            out,
            "- Record major design decisions in {} using {} files.",
            normalize_text(&record.decision_records.path),
            normalize_text(&record.decision_records.format)
        )
        .unwrap();
        writeln!(out, "- Use ADRs when a change alters the project baseline, crosses a listed red line, or introduces a lasting domain override.").unwrap();
    } else {
        writeln!(out, "- ADRs are not mandatory by default for this project.").unwrap();
        writeln!(
            out,
            "- Capture any material exception or posture change in {} instead.",
            normalize_text(&record.governance.exception_process.record_location)
        )
        .unwrap();
    }

    writeln!(out).unwrap();
    writeln!(out, "## Review & updates").unwrap();
    writeln!(
        out,
        "- Review this charter on a {} cadence.",
        normalize_text(&record.debt.review_cadence)
    )
    .unwrap();
    writeln!(out, "- Update it when the project classification, domains, runtime environments, or production reality change.").unwrap();
    writeln!(out, "- Re-run impacted plans when any update changes a default level, a red line, or an exception process.").unwrap();
    Ok(out.into_bytes())
}

fn rubric_level(level: u8) -> Result<RubricLevel, CharterArtifactError> {
    RUBRIC_LEVELS
        .get(level.saturating_sub(1) as usize)
        .copied()
        .ok_or_else(|| {
            CharterArtifactError::new(
                CharterArtifactErrorKind::StructuralValidationFailed,
                "canonical Charter rubric level is outside 1-5",
            )
        })
}

fn display_token(value: &str) -> String {
    match value {
        "web_app" => "web app".to_owned(),
        "lib" => "library".to_owned(),
        "infra" => "infrastructure".to_owned(),
        "on_prem" => "on-prem".to_owned(),
        _ => value.replace('_', " "),
    }
}

fn inline_text_or_default(value: &str, default: &str) -> String {
    let normalized = normalize_text(value);
    if normalized.is_empty() {
        default.to_owned()
    } else {
        normalized
    }
}

fn inline_list_or_default(values: &[String], default: &str) -> String {
    if values.is_empty() {
        default.to_owned()
    } else {
        values
            .iter()
            .map(|value| normalize_text(value))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn push_bullets(output: &mut String, values: &[String]) {
    for value in values {
        writeln!(output, "- {}", normalize_text(value)).unwrap();
    }
}

fn escape_table_cell(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('|', "\\|")
        .replace('\r', "\\r")
        .replace('\n', "\\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    const BOUNDARY_YAML: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
    ));
    const BOUNDARY_MARKDOWN: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/charter-review-boundary-v1.0.md"
    ));

    fn boundary() -> CanonicalCharter {
        let value = parse_definition_yaml(BOUNDARY_YAML).expect("boundary YAML");
        serde_json::from_value(value).expect("closed Charter")
    }

    #[test]
    fn production_emitter_and_renderer_reproduce_literal_boundaries_without_registry_fallback() {
        let charter = boundary();
        validate_canonical_charter_semantics(&charter).expect("semantic boundary");
        assert_eq!(emit_canonical_charter(&charter).unwrap(), BOUNDARY_YAML);
        assert_eq!(
            render_canonical_charter_markdown(&charter).unwrap(),
            BOUNDARY_MARKDOWN
        );
    }
}
