use crate::{
    parse_definition_yaml, ArtifactProfileDecision, ArtifactRegistryValidationError,
    DefinitionFingerprint, RegistryLoadErrorKind, ResolvedProfileDecisions,
};
use serde::{Deserialize, Serialize};

const PROJECT_CONTEXT_INSTANCE_ID: &str = "project_context";
const PROJECT_CONTEXT_KIND_REF: &str = "handbook.artifact-kind.project-context@1.0.0";
const PROJECT_CONTEXT_SCHEMA_REF: &str = "handbook.schemas.artifacts.project-context@1.0.0";
pub(crate) const SELECTED_PROJECT_CONTEXT_CANONICAL_PATH: &str = ".handbook/project/context.yaml";

/// The complete selected Project Context `1.0` authoring surface.
///
/// The retired rich `0.1` authoring model is intentionally not exported:
///
/// ```compile_fail
/// use handbook_engine::{
///     parse_project_context_structured_input_yaml, ProjectContextStructuredInput,
/// };
/// ```
///
/// ```compile_fail
/// use handbook_engine::author::project_context_core::ProjectContextStructuredInput;
/// ```
///
/// ```compile_fail
/// use handbook_engine::SELECTED_PROJECT_CONTEXT_CANONICAL_PATH;
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalProjectContext {
    pub schema_id: String,
    pub schema_version: String,
    pub record_id: String,
    pub summary: String,
    pub system_boundaries: Vec<String>,
    pub ownership: Vec<String>,
    pub authoritative_references: Vec<String>,
    pub known_unknowns: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectContextArtifactErrorKind {
    SourceLimitExceeded,
    DuplicateKey,
    SyntaxError,
    NonObjectRoot,
    SelectedDecisionMissing,
    SelectedContractMismatch,
    StructuralValidationFailed,
    TypedDecodeFailed,
    RenderedViewRefused,
    SerializationFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectContextArtifactError {
    kind: ProjectContextArtifactErrorKind,
    detail: &'static str,
}

impl ProjectContextArtifactError {
    fn new(kind: ProjectContextArtifactErrorKind, detail: &'static str) -> Self {
        Self { kind, detail }
    }

    pub fn kind(&self) -> ProjectContextArtifactErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        self.detail
    }
}

pub fn parse_canonical_project_context(
    decisions: &ResolvedProfileDecisions,
    source_bytes: &[u8],
) -> Result<CanonicalProjectContext, ProjectContextArtifactError> {
    let value = parse_definition_yaml(source_bytes).map_err(|error| {
        let kind = match error.kind() {
            RegistryLoadErrorKind::SourceLimitExceeded => {
                ProjectContextArtifactErrorKind::SourceLimitExceeded
            }
            RegistryLoadErrorKind::DuplicateKey => ProjectContextArtifactErrorKind::DuplicateKey,
            _ => ProjectContextArtifactErrorKind::SyntaxError,
        };
        ProjectContextArtifactError::new(kind, "canonical Project Context YAML was refused")
    })?;
    if !value.is_object() {
        return Err(ProjectContextArtifactError::new(
            ProjectContextArtifactErrorKind::NonObjectRoot,
            "canonical Project Context must have an object root",
        ));
    }

    let decision = selected_project_context_decision(decisions)?;

    decisions
        .registry()
        .validate_json(decision.instance_id(), &value)
        .map_err(|error| match error {
            ArtifactRegistryValidationError::UnknownArtifactInstance
            | ArtifactRegistryValidationError::Structural(_) => ProjectContextArtifactError::new(
                ProjectContextArtifactErrorKind::StructuralValidationFailed,
                "canonical Project Context failed selected-schema validation",
            ),
        })?;

    let record: CanonicalProjectContext = serde_json::from_value(value.clone()).map_err(|_| {
        ProjectContextArtifactError::new(
            ProjectContextArtifactErrorKind::TypedDecodeFailed,
            "canonical Project Context could not be closed-decoded",
        )
    })?;
    let roundtrip = serde_json::to_value(&record).map_err(|_| {
        ProjectContextArtifactError::new(
            ProjectContextArtifactErrorKind::TypedDecodeFailed,
            "canonical Project Context could not be closed-encoded",
        )
    })?;
    if roundtrip != value {
        return Err(ProjectContextArtifactError::new(
            ProjectContextArtifactErrorKind::TypedDecodeFailed,
            "canonical Project Context typed truth disagrees with validated JSON",
        ));
    }
    Ok(record)
}

pub(crate) fn selected_project_context_decision(
    decisions: &ResolvedProfileDecisions,
) -> Result<&ArtifactProfileDecision, ProjectContextArtifactError> {
    let decision = decisions
        .artifact_decisions()
        .iter()
        .find(|decision| decision.instance_id().as_str() == PROJECT_CONTEXT_INSTANCE_ID)
        .ok_or_else(|| {
            ProjectContextArtifactError::new(
                ProjectContextArtifactErrorKind::SelectedDecisionMissing,
                "selected Project Context decision is missing",
            )
        })?;
    let instance = decisions
        .registry()
        .instance(decision.instance_id())
        .ok_or_else(|| {
            ProjectContextArtifactError::new(
                ProjectContextArtifactErrorKind::SelectedDecisionMissing,
                "selected Project Context instance is missing",
            )
        })?;
    let kind = decisions
        .registry()
        .kind(instance.kind_ref())
        .ok_or_else(|| {
            ProjectContextArtifactError::new(
                ProjectContextArtifactErrorKind::SelectedContractMismatch,
                "selected Project Context kind is missing",
            )
        })?;
    if !selected_contract_matches(
        decision.kind_ref().as_str(),
        instance.kind_ref().as_str(),
        kind.canonical_schema_ref().as_str(),
        decision.canonical_path(),
    ) {
        return Err(ProjectContextArtifactError::new(
            ProjectContextArtifactErrorKind::SelectedContractMismatch,
            "selected Project Context kind or schema binding does not match the fixed contract",
        ));
    }

    Ok(decision)
}

fn selected_contract_matches(
    decision_kind_ref: &str,
    instance_kind_ref: &str,
    schema_ref: &str,
    canonical_path: &str,
) -> bool {
    decision_kind_ref == instance_kind_ref
        && matches!(
            decision_kind_ref,
            PROJECT_CONTEXT_KIND_REF | "handbook.artifact-kind.project-context@1.1.0"
        )
        && schema_ref == PROJECT_CONTEXT_SCHEMA_REF
        && canonical_path == SELECTED_PROJECT_CONTEXT_CANONICAL_PATH
}

pub fn serialize_canonical_project_context(
    decisions: &ResolvedProfileDecisions,
    record: &CanonicalProjectContext,
) -> Result<Vec<u8>, ProjectContextArtifactError> {
    let decision = selected_project_context_decision(decisions)?;
    let value = serde_json::to_value(record).map_err(|_| {
        ProjectContextArtifactError::new(
            ProjectContextArtifactErrorKind::SerializationFailed,
            "canonical Project Context could not be closed-encoded",
        )
    })?;
    decisions
        .registry()
        .validate_json(decision.instance_id(), &value)
        .map_err(|_| {
            ProjectContextArtifactError::new(
                ProjectContextArtifactErrorKind::StructuralValidationFailed,
                "canonical Project Context failed selected-schema validation",
            )
        })?;

    let bytes = emit_canonical_project_context(record)?;
    let reparsed = parse_canonical_project_context(decisions, &bytes)?;
    if reparsed != *record {
        return Err(ProjectContextArtifactError::new(
            ProjectContextArtifactErrorKind::SerializationFailed,
            "canonical Project Context emitter changed typed truth",
        ));
    }
    Ok(bytes)
}

fn emit_canonical_project_context(
    record: &CanonicalProjectContext,
) -> Result<Vec<u8>, ProjectContextArtifactError> {
    let mut output = String::new();
    emit_scalar(&mut output, "schema_id", &record.schema_id)?;
    emit_scalar(&mut output, "schema_version", &record.schema_version)?;
    emit_scalar(&mut output, "record_id", &record.record_id)?;
    emit_scalar(&mut output, "summary", &record.summary)?;
    emit_sequence(&mut output, "system_boundaries", &record.system_boundaries)?;
    emit_sequence(&mut output, "ownership", &record.ownership)?;
    emit_sequence(
        &mut output,
        "authoritative_references",
        &record.authoritative_references,
    )?;
    emit_sequence(&mut output, "known_unknowns", &record.known_unknowns)?;
    Ok(output.into_bytes())
}

pub fn render_project_context_markdown(
    record: &CanonicalProjectContext,
) -> Result<Vec<u8>, ProjectContextArtifactError> {
    let mut output = String::from("# Project Context\n\n## Summary\n\n");
    output.push_str(&plain_text(&record.summary)?);
    output.push_str("\n\n## System Boundaries\n\n");
    emit_markdown_list(&mut output, &record.system_boundaries, false)?;
    output.push_str("\n## Ownership\n\n");
    emit_markdown_list(&mut output, &record.ownership, false)?;
    output.push_str("\n## Authoritative References\n\n");
    emit_markdown_list(&mut output, &record.authoritative_references, true)?;
    output.push_str("\n## Known Unknowns\n\n");
    emit_markdown_list(&mut output, &record.known_unknowns, true)?;
    Ok(output.into_bytes())
}

pub fn project_context_source_fingerprint(source_bytes: &[u8]) -> DefinitionFingerprint {
    DefinitionFingerprint::from_bytes(source_bytes)
}

pub fn project_context_rendered_fingerprint(rendered_bytes: &[u8]) -> DefinitionFingerprint {
    DefinitionFingerprint::from_bytes(rendered_bytes)
}

fn emit_scalar(
    output: &mut String,
    key: &str,
    value: &str,
) -> Result<(), ProjectContextArtifactError> {
    output.push_str(key);
    output.push_str(": ");
    output.push_str(&json_string(value)?);
    output.push('\n');
    Ok(())
}

fn emit_sequence(
    output: &mut String,
    key: &str,
    values: &[String],
) -> Result<(), ProjectContextArtifactError> {
    output.push_str(key);
    if values.is_empty() {
        output.push_str(": []\n");
        return Ok(());
    }
    output.push_str(":\n");
    for value in values {
        output.push_str("  - ");
        output.push_str(&json_string(value)?);
        output.push('\n');
    }
    Ok(())
}

fn json_string(value: &str) -> Result<String, ProjectContextArtifactError> {
    serde_json::to_string(value).map_err(|_| {
        ProjectContextArtifactError::new(
            ProjectContextArtifactErrorKind::SerializationFailed,
            "canonical Project Context string serialization failed",
        )
    })
}

fn emit_markdown_list(
    output: &mut String,
    values: &[String],
    allow_empty: bool,
) -> Result<(), ProjectContextArtifactError> {
    if values.is_empty() && allow_empty {
        output.push_str("- None recorded.\n");
        return Ok(());
    }
    if values.is_empty() {
        return Err(render_refusal());
    }
    for value in values {
        output.push_str("- ");
        output.push_str(&plain_text(value)?);
        output.push('\n');
    }
    Ok(())
}

fn plain_text(value: &str) -> Result<String, ProjectContextArtifactError> {
    let mut normalized = String::new();
    let mut pending_space = false;
    for character in value.chars() {
        if matches!(character, ' ' | '\r' | '\n' | '\t') {
            if !normalized.is_empty() {
                pending_space = true;
            }
            continue;
        }
        if is_forbidden_control(character) {
            return Err(render_refusal());
        }
        if pending_space {
            normalized.push(' ');
            pending_space = false;
        }
        normalized.push(character);
    }
    if normalized.is_empty() {
        return Err(render_refusal());
    }

    let mut escaped = String::with_capacity(normalized.len());
    for character in normalized.chars() {
        if is_escaped_ascii_punctuation(character) {
            escaped.push('\\');
        }
        escaped.push(character);
    }
    Ok(escaped)
}

fn is_forbidden_control(character: char) -> bool {
    matches!(character as u32, 0x01..=0x08 | 0x0b..=0x0c | 0x0e..=0x1f | 0x7f)
}

fn is_escaped_ascii_punctuation(character: char) -> bool {
    character.is_ascii_punctuation()
}

fn render_refusal() -> ProjectContextArtifactError {
    ProjectContextArtifactError::new(
        ProjectContextArtifactErrorKind::RenderedViewRefused,
        "canonical Project Context value cannot be rendered as controlled Markdown",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_contract_binding_refuses_each_kind_schema_and_path_mismatch() {
        for kind_ref in [
            PROJECT_CONTEXT_KIND_REF,
            "handbook.artifact-kind.project-context@1.1.0",
        ] {
            assert!(selected_contract_matches(
                kind_ref,
                kind_ref,
                PROJECT_CONTEXT_SCHEMA_REF,
                SELECTED_PROJECT_CONTEXT_CANONICAL_PATH,
            ));
        }

        for (decision_kind, instance_kind, schema, path) in [
            (
                PROJECT_CONTEXT_KIND_REF,
                "handbook.artifact-kind.project-context@1.1.0",
                PROJECT_CONTEXT_SCHEMA_REF,
                SELECTED_PROJECT_CONTEXT_CANONICAL_PATH,
            ),
            (
                "handbook.artifact-kind.project-context@1.1.0",
                PROJECT_CONTEXT_KIND_REF,
                PROJECT_CONTEXT_SCHEMA_REF,
                SELECTED_PROJECT_CONTEXT_CANONICAL_PATH,
            ),
            (
                "handbook.artifact-kind.other@1.0.0",
                PROJECT_CONTEXT_KIND_REF,
                PROJECT_CONTEXT_SCHEMA_REF,
                SELECTED_PROJECT_CONTEXT_CANONICAL_PATH,
            ),
            (
                PROJECT_CONTEXT_KIND_REF,
                "handbook.artifact-kind.other@1.0.0",
                PROJECT_CONTEXT_SCHEMA_REF,
                SELECTED_PROJECT_CONTEXT_CANONICAL_PATH,
            ),
            (
                "handbook.artifact-kind.project-context@1.2.0",
                "handbook.artifact-kind.project-context@1.2.0",
                PROJECT_CONTEXT_SCHEMA_REF,
                SELECTED_PROJECT_CONTEXT_CANONICAL_PATH,
            ),
            (
                "handbook.artifact-kind.project-context@1.1",
                "handbook.artifact-kind.project-context@1.1",
                PROJECT_CONTEXT_SCHEMA_REF,
                SELECTED_PROJECT_CONTEXT_CANONICAL_PATH,
            ),
            (
                "handbook.artifact-kind.project-context@>=1.0.0",
                "handbook.artifact-kind.project-context@>=1.0.0",
                PROJECT_CONTEXT_SCHEMA_REF,
                SELECTED_PROJECT_CONTEXT_CANONICAL_PATH,
            ),
            (
                "handbook.artifact-kind.project-context@latest",
                "handbook.artifact-kind.project-context@latest",
                PROJECT_CONTEXT_SCHEMA_REF,
                SELECTED_PROJECT_CONTEXT_CANONICAL_PATH,
            ),
            (
                "handbook.artifact-kind.project-context",
                "handbook.artifact-kind.project-context",
                PROJECT_CONTEXT_SCHEMA_REF,
                SELECTED_PROJECT_CONTEXT_CANONICAL_PATH,
            ),
            (
                PROJECT_CONTEXT_KIND_REF,
                PROJECT_CONTEXT_KIND_REF,
                "handbook.schemas.artifacts.other@1.0.0",
                SELECTED_PROJECT_CONTEXT_CANONICAL_PATH,
            ),
            (
                PROJECT_CONTEXT_KIND_REF,
                PROJECT_CONTEXT_KIND_REF,
                PROJECT_CONTEXT_SCHEMA_REF,
                ".handbook/project_context/PROJECT_CONTEXT.md",
            ),
        ] {
            assert!(!selected_contract_matches(
                decision_kind,
                instance_kind,
                schema,
                path
            ));
        }
    }

    #[test]
    fn closed_emitter_frames_every_empty_and_non_empty_sequence_shape() {
        for mask in 0_u8..16 {
            let sequence = |bit| {
                if mask & bit != 0 {
                    vec![format!("value-{bit}")]
                } else {
                    Vec::new()
                }
            };
            let record = CanonicalProjectContext {
                schema_id: "handbook.artifact.project-context".to_owned(),
                schema_version: "1.0".to_owned(),
                record_id: "handbook.project-context".to_owned(),
                summary: "Emitter framing.".to_owned(),
                system_boundaries: sequence(1),
                ownership: sequence(2),
                authoritative_references: sequence(4),
                known_unknowns: sequence(8),
            };

            let text = String::from_utf8(emit_canonical_project_context(&record).unwrap()).unwrap();
            for (key, values) in [
                ("system_boundaries", &record.system_boundaries),
                ("ownership", &record.ownership),
                ("authoritative_references", &record.authoritative_references),
                ("known_unknowns", &record.known_unknowns),
            ] {
                if values.is_empty() {
                    assert!(text.contains(&format!("{key}: []\n")), "{text}");
                } else {
                    assert!(
                        text.contains(&format!("{key}:\n  - \"{}\"\n", values[0])),
                        "{text}"
                    );
                }
            }
        }
    }
}
