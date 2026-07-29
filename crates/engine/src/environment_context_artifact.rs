use crate::canonical_repo_support::{CanonicalWorkspace, RepoRelativeFileAccessError};
use crate::canonical_yaml::canonical_yaml_bytes;
use crate::{
    parse_definition_yaml, ArtifactProfileDecision, ArtifactRegistryValidationError,
    DefinitionFingerprint, RegistryLoadErrorKind, ResolvedProfileDecisions,
    MAX_SOURCE_DOCUMENT_BYTES,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;

const ENVIRONMENT_CONTEXT_INSTANCE_ID: &str = "environment_context";
const ENVIRONMENT_CONTEXT_KIND_REF: &str = "handbook.artifact-kind.environment-context@1.1.0";
const ENVIRONMENT_CONTEXT_SCHEMA_REF: &str = "handbook.schemas.artifacts.environment-context@1.1.0";
pub const ENVIRONMENT_CONTEXT_CANONICAL_PATH: &str = ".handbook/project/environment.yaml";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalEnvironmentContext {
    pub schema_id: String,
    pub schema_version: String,
    pub record_id: String,
    pub environments: Vec<NamedEnvironment>,
    pub authoritative_references: Vec<String>,
    pub known_unknowns: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NamedEnvironment {
    pub environment_id: String,
    pub description: String,
    pub capabilities: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnvironmentContextArtifactErrorKind {
    Missing,
    UnsafePath,
    SourceReadFailed,
    SourceLimitExceeded,
    DuplicateKey,
    SyntaxError,
    NonObjectRoot,
    SelectedDecisionMissing,
    SelectedContractMismatch,
    StructuralValidationFailed,
    TypedDecodeFailed,
    DuplicateEnvironmentId,
    RenderedViewRefused,
    SerializationFailed,
    ObservationChanged,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnvironmentContextArtifactError {
    kind: EnvironmentContextArtifactErrorKind,
    detail: &'static str,
}

impl EnvironmentContextArtifactError {
    fn new(kind: EnvironmentContextArtifactErrorKind, detail: &'static str) -> Self {
        Self { kind, detail }
    }

    pub fn kind(&self) -> EnvironmentContextArtifactErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &'static str {
        self.detail
    }
}

impl std::fmt::Display for EnvironmentContextArtifactError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.detail)
    }
}

impl std::error::Error for EnvironmentContextArtifactError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalEnvironmentContextProjection {
    canonical_path: String,
    record: CanonicalEnvironmentContext,
    source_byte_length: usize,
    rendered_bytes: Vec<u8>,
    source_fingerprint: DefinitionFingerprint,
    rendered_output_fingerprint: DefinitionFingerprint,
}

impl CanonicalEnvironmentContextProjection {
    pub fn canonical_path(&self) -> &str {
        &self.canonical_path
    }

    pub fn record(&self) -> &CanonicalEnvironmentContext {
        &self.record
    }

    pub fn source_byte_length(&self) -> usize {
        self.source_byte_length
    }

    pub fn rendered_bytes(&self) -> &[u8] {
        &self.rendered_bytes
    }

    pub fn source_fingerprint(&self) -> &DefinitionFingerprint {
        &self.source_fingerprint
    }

    pub fn rendered_output_fingerprint(&self) -> &DefinitionFingerprint {
        &self.rendered_output_fingerprint
    }
}

pub fn parse_canonical_environment_context(
    decisions: &ResolvedProfileDecisions,
    source_bytes: &[u8],
) -> Result<CanonicalEnvironmentContext, EnvironmentContextArtifactError> {
    let value = parse_definition_yaml(source_bytes).map_err(|error| {
        let kind = match error.kind() {
            RegistryLoadErrorKind::SourceLimitExceeded => {
                EnvironmentContextArtifactErrorKind::SourceLimitExceeded
            }
            RegistryLoadErrorKind::DuplicateKey => {
                EnvironmentContextArtifactErrorKind::DuplicateKey
            }
            _ => EnvironmentContextArtifactErrorKind::SyntaxError,
        };
        EnvironmentContextArtifactError::new(kind, "canonical Environment Context YAML refused")
    })?;
    if !value.is_object() {
        return Err(EnvironmentContextArtifactError::new(
            EnvironmentContextArtifactErrorKind::NonObjectRoot,
            "canonical Environment Context must have an object root",
        ));
    }

    let decision = selected_environment_context_decision(decisions)?;
    decisions
        .registry()
        .validate_json(decision.instance_id(), &value)
        .map_err(|error| match error {
            ArtifactRegistryValidationError::UnknownArtifactInstance
            | ArtifactRegistryValidationError::Structural(_) => {
                EnvironmentContextArtifactError::new(
                    EnvironmentContextArtifactErrorKind::StructuralValidationFailed,
                    "canonical Environment Context failed selected-schema validation",
                )
            }
        })?;

    let record: CanonicalEnvironmentContext =
        serde_json::from_value(value.clone()).map_err(|_| {
            EnvironmentContextArtifactError::new(
                EnvironmentContextArtifactErrorKind::TypedDecodeFailed,
                "canonical Environment Context could not be closed-decoded",
            )
        })?;
    let roundtrip = serde_json::to_value(&record).map_err(|_| {
        EnvironmentContextArtifactError::new(
            EnvironmentContextArtifactErrorKind::TypedDecodeFailed,
            "canonical Environment Context could not be closed-encoded",
        )
    })?;
    if roundtrip != value {
        return Err(EnvironmentContextArtifactError::new(
            EnvironmentContextArtifactErrorKind::TypedDecodeFailed,
            "canonical Environment Context typed truth disagrees with validated JSON",
        ));
    }
    validate_environment_id_uniqueness(&record)?;
    Ok(record)
}

pub fn serialize_canonical_environment_context(
    decisions: &ResolvedProfileDecisions,
    record: &CanonicalEnvironmentContext,
) -> Result<Vec<u8>, EnvironmentContextArtifactError> {
    validate_environment_id_uniqueness(record)?;
    let decision = selected_environment_context_decision(decisions)?;
    let value = serde_json::to_value(record).map_err(|_| {
        EnvironmentContextArtifactError::new(
            EnvironmentContextArtifactErrorKind::SerializationFailed,
            "canonical Environment Context could not be closed-encoded",
        )
    })?;
    decisions
        .registry()
        .validate_json(decision.instance_id(), &value)
        .map_err(|_| {
            EnvironmentContextArtifactError::new(
                EnvironmentContextArtifactErrorKind::StructuralValidationFailed,
                "canonical Environment Context failed selected-schema validation",
            )
        })?;
    let bytes = canonical_yaml_bytes(&value).map_err(|_| {
        EnvironmentContextArtifactError::new(
            EnvironmentContextArtifactErrorKind::SerializationFailed,
            "canonical Environment Context YAML serialization failed",
        )
    })?;
    if parse_canonical_environment_context(decisions, &bytes)? != *record {
        return Err(EnvironmentContextArtifactError::new(
            EnvironmentContextArtifactErrorKind::SerializationFailed,
            "canonical Environment Context serialization changed typed truth",
        ));
    }
    Ok(bytes)
}

pub fn render_environment_context_markdown(
    record: &CanonicalEnvironmentContext,
) -> Result<Vec<u8>, EnvironmentContextArtifactError> {
    validate_environment_id_uniqueness(record)?;
    let mut output = String::from("# Environment Context\n\n");
    for environment in &record.environments {
        output.push_str("## ");
        output.push_str(&plain_text(&environment.environment_id)?);
        output.push_str("\n\n");
        output.push_str(&plain_text(&environment.description)?);
        output.push_str("\n\n### Capabilities\n\n");
        for capability in &environment.capabilities {
            output.push_str("- `");
            output.push_str(&code_text(capability)?);
            output.push_str("`\n");
        }
        output.push('\n');
    }
    output.push_str("## Authoritative References\n\n");
    emit_list(&mut output, &record.authoritative_references, true)?;
    output.push_str("\n## Known Unknowns\n\n");
    emit_list(&mut output, &record.known_unknowns, false)?;
    Ok(output.into_bytes())
}

pub fn load_selected_environment_context(
    repo_root: impl AsRef<Path>,
    decisions: &ResolvedProfileDecisions,
) -> Result<CanonicalEnvironmentContextProjection, EnvironmentContextArtifactError> {
    let decision = selected_environment_context_decision(decisions)?;
    let canonical_path = decision.canonical_path();
    let workspace = CanonicalWorkspace::new(repo_root.as_ref());
    let normalized = workspace
        .normalize_repo_relative(canonical_path)
        .map_err(|_| unsafe_path())?;
    let file = workspace
        .trusted_read_strict(&normalized)
        .map_err(map_read_error)?;
    let (source_bytes, exceeded) = file
        .read_bytes_bounded(MAX_SOURCE_DOCUMENT_BYTES)
        .map_err(|_| source_read_failed())?;
    if exceeded {
        return Err(EnvironmentContextArtifactError::new(
            EnvironmentContextArtifactErrorKind::SourceLimitExceeded,
            "canonical Environment Context exceeds the document limit",
        ));
    }

    let record = parse_canonical_environment_context(decisions, &source_bytes)?;
    let rendered_bytes = render_environment_context_markdown(&record)?;

    let final_file = workspace
        .trusted_read_strict(&normalized)
        .map_err(|_| observation_changed())?;
    let (final_bytes, final_exceeded) = final_file
        .read_bytes_bounded(MAX_SOURCE_DOCUMENT_BYTES)
        .map_err(|_| observation_changed())?;
    if final_exceeded || final_bytes != source_bytes {
        return Err(observation_changed());
    }

    Ok(CanonicalEnvironmentContextProjection {
        canonical_path: canonical_path.to_owned(),
        record,
        source_byte_length: source_bytes.len(),
        source_fingerprint: DefinitionFingerprint::from_bytes(&source_bytes),
        rendered_output_fingerprint: DefinitionFingerprint::from_bytes(&rendered_bytes),
        rendered_bytes,
    })
}

fn selected_environment_context_decision(
    decisions: &ResolvedProfileDecisions,
) -> Result<&ArtifactProfileDecision, EnvironmentContextArtifactError> {
    let decision = decisions
        .artifact_decisions()
        .iter()
        .find(|decision| decision.instance_id().as_str() == ENVIRONMENT_CONTEXT_INSTANCE_ID)
        .ok_or_else(|| {
            EnvironmentContextArtifactError::new(
                EnvironmentContextArtifactErrorKind::SelectedDecisionMissing,
                "selected Environment Context decision is missing",
            )
        })?;
    let instance = decisions
        .registry()
        .instance(decision.instance_id())
        .ok_or_else(|| {
            EnvironmentContextArtifactError::new(
                EnvironmentContextArtifactErrorKind::SelectedDecisionMissing,
                "selected Environment Context instance is missing",
            )
        })?;
    let kind = decisions
        .registry()
        .kind(instance.kind_ref())
        .ok_or_else(|| {
            EnvironmentContextArtifactError::new(
                EnvironmentContextArtifactErrorKind::SelectedContractMismatch,
                "selected Environment Context kind is missing",
            )
        })?;
    if decision.kind_ref().as_str() != ENVIRONMENT_CONTEXT_KIND_REF
        || instance.kind_ref().as_str() != ENVIRONMENT_CONTEXT_KIND_REF
        || kind.canonical_schema_ref().as_str() != ENVIRONMENT_CONTEXT_SCHEMA_REF
        || decision.canonical_path() != ENVIRONMENT_CONTEXT_CANONICAL_PATH
    {
        return Err(EnvironmentContextArtifactError::new(
            EnvironmentContextArtifactErrorKind::SelectedContractMismatch,
            "selected Environment Context kind, schema, or path differs from the fixed contract",
        ));
    }
    Ok(decision)
}

fn validate_environment_id_uniqueness(
    record: &CanonicalEnvironmentContext,
) -> Result<(), EnvironmentContextArtifactError> {
    let mut ids = BTreeSet::new();
    if record
        .environments
        .iter()
        .any(|environment| !ids.insert(environment.environment_id.as_str()))
    {
        return Err(EnvironmentContextArtifactError::new(
            EnvironmentContextArtifactErrorKind::DuplicateEnvironmentId,
            "canonical Environment Context environment IDs must be unique",
        ));
    }
    Ok(())
}

fn emit_list(
    output: &mut String,
    values: &[String],
    code: bool,
) -> Result<(), EnvironmentContextArtifactError> {
    if values.is_empty() {
        output.push_str("- None recorded.\n");
        return Ok(());
    }
    for value in values {
        output.push_str("- ");
        if code {
            output.push('`');
            output.push_str(&code_text(value)?);
            output.push('`');
        } else {
            output.push_str(&plain_text(value)?);
        }
        output.push('\n');
    }
    Ok(())
}

fn plain_text(value: &str) -> Result<String, EnvironmentContextArtifactError> {
    let mut normalized = String::new();
    let mut pending_space = false;
    for character in value.chars() {
        if character.is_whitespace() {
            pending_space = !normalized.is_empty();
            continue;
        }
        if character.is_control() {
            return Err(render_refusal());
        }
        if pending_space {
            normalized.push(' ');
            pending_space = false;
        }
        if character.is_ascii_punctuation() {
            normalized.push('\\');
        }
        normalized.push(character);
    }
    if normalized.is_empty() {
        return Err(render_refusal());
    }
    Ok(normalized)
}

fn code_text(value: &str) -> Result<String, EnvironmentContextArtifactError> {
    if value.is_empty()
        || value
            .chars()
            .any(|character| character.is_control() || character == '`')
    {
        return Err(render_refusal());
    }
    Ok(value.to_owned())
}

fn map_read_error(error: RepoRelativeFileAccessError) -> EnvironmentContextArtifactError {
    match error {
        RepoRelativeFileAccessError::Missing(_) => EnvironmentContextArtifactError::new(
            EnvironmentContextArtifactErrorKind::Missing,
            "canonical Environment Context is absent",
        ),
        RepoRelativeFileAccessError::SymlinkNotAllowed(_)
        | RepoRelativeFileAccessError::NotRegularFile(_)
        | RepoRelativeFileAccessError::InvalidPath(_) => unsafe_path(),
        RepoRelativeFileAccessError::ReadFailure { .. } => source_read_failed(),
    }
}

fn unsafe_path() -> EnvironmentContextArtifactError {
    EnvironmentContextArtifactError::new(
        EnvironmentContextArtifactErrorKind::UnsafePath,
        "canonical Environment Context path is unsafe",
    )
}

fn source_read_failed() -> EnvironmentContextArtifactError {
    EnvironmentContextArtifactError::new(
        EnvironmentContextArtifactErrorKind::SourceReadFailed,
        "canonical Environment Context could not be read",
    )
}

fn observation_changed() -> EnvironmentContextArtifactError {
    EnvironmentContextArtifactError::new(
        EnvironmentContextArtifactErrorKind::ObservationChanged,
        "canonical Environment Context changed during observation",
    )
}

fn render_refusal() -> EnvironmentContextArtifactError {
    EnvironmentContextArtifactError::new(
        EnvironmentContextArtifactErrorKind::RenderedViewRefused,
        "canonical Environment Context cannot be rendered as controlled Markdown",
    )
}
