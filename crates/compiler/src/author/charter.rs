use super::{
    charter_core::{
        compiler_owned_charter_markdown as compiler_owned_charter_markdown_core,
        parse_charter_structured_input_yaml as parse_charter_structured_input_yaml_core,
        render_charter_markdown as render_charter_markdown_core,
        validate_charter_structured_input as validate_charter_structured_input_core,
        CharterCoreError, CharterCoreErrorKind,
    },
    charter_shell::{
        preflight_author_charter as preflight_author_charter_shell, with_charter_authoring_lock,
        write_canonical_charter_markdown,
    },
};
use crate::layout::CANONICAL_CHARTER_RELATIVE_PATH;
use std::path::Path;

pub use super::charter_core::{
    is_unusably_vague_charter_text, normalize_charter_free_text, validate_charter_markdown,
    CharterAudience, CharterBackwardCompatibility, CharterDebtTrackingInput,
    CharterDecisionRecordsInput, CharterDefaultImplicationsInput, CharterDeprecationPolicy,
    CharterDimensionInput, CharterDimensionName, CharterDomainInput, CharterExceptionsInput,
    CharterExpectedLifetime, CharterObservabilityThreshold, CharterOperationalRealityInput,
    CharterPostureInput, CharterProjectClassification, CharterProjectConstraintsInput,
    CharterProjectInput, CharterRequiredness, CharterRolloutControls, CharterRuntimeEnvironment,
    CharterStructuredInput, CharterSurface, DEFAULT_EXCEPTION_RECORD_LOCATION,
};

pub const CANONICAL_CHARTER_REPO_PATH: &str = CANONICAL_CHARTER_RELATIVE_PATH;
// Command paths:
// `handbook author charter --from-inputs <path|->`
// -> normalized `CharterStructuredInput`
// -> compiler-owned deterministic render
// -> guarded write to `.handbook/charter/CHARTER.md`

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorCharterRefusalKind {
    MissingSystemRoot,
    InvalidSystemRoot,
    MalformedStructuredInput,
    IncompleteStructuredInput,
    ExistingCanonicalTruth,
    MutationRefused,
    SynthesisFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorCharterRefusal {
    pub kind: AuthorCharterRefusalKind,
    pub summary: String,
    pub broken_subject: String,
    pub next_safe_action: String,
}

impl std::fmt::Display for AuthorCharterRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary)
    }
}

impl std::error::Error for AuthorCharterRefusal {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorCharterResult {
    pub canonical_repo_relative_path: &'static str,
    pub bytes_written: usize,
}

pub fn parse_charter_structured_input_yaml(
    yaml: &str,
) -> Result<CharterStructuredInput, AuthorCharterRefusal> {
    parse_charter_structured_input_yaml_core(yaml).map_err(map_charter_core_error)
}

pub fn validate_charter_structured_input(
    input: &CharterStructuredInput,
) -> Result<(), AuthorCharterRefusal> {
    validate_charter_structured_input_core(input).map_err(map_charter_core_error)
}

pub fn render_charter_markdown(
    input: &CharterStructuredInput,
) -> Result<String, AuthorCharterRefusal> {
    render_charter_markdown_core(input).map_err(map_charter_core_error)
}

fn compiler_owned_charter_markdown(
    input: &CharterStructuredInput,
) -> Result<String, AuthorCharterRefusal> {
    compiler_owned_charter_markdown_core(input).map_err(map_charter_core_error)
}

pub fn preflight_author_charter_from_input(
    repo_root: impl AsRef<Path>,
    input: &CharterStructuredInput,
) -> Result<(), AuthorCharterRefusal> {
    let repo_root = repo_root.as_ref();
    compiler_owned_charter_markdown(input)?;
    preflight_author_charter(repo_root)?;
    Ok(())
}

fn map_charter_core_error(err: CharterCoreError) -> AuthorCharterRefusal {
    match err.kind {
        CharterCoreErrorKind::MalformedStructuredInput => AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::MalformedStructuredInput,
            summary: err.summary,
            broken_subject: "structured charter input".to_string(),
            next_safe_action:
                "repair the structured charter input and retry `handbook author charter --from-inputs <path|->`"
                    .to_string(),
        },
        CharterCoreErrorKind::IncompleteStructuredInput => AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::IncompleteStructuredInput,
            summary: err.summary,
            broken_subject: "structured charter input".to_string(),
            next_safe_action:
                "repair the structured charter input and retry `handbook author charter --from-inputs <path|->`"
                    .to_string(),
        },
        CharterCoreErrorKind::DeterministicRenderFailed => AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::SynthesisFailed,
            summary: err.summary,
            broken_subject: "final charter render".to_string(),
            next_safe_action:
                "repair the structured charter input or compiler-owned charter render path and retry `handbook author charter --from-inputs <path|->`"
                    .to_string(),
        },
    }
}

pub fn author_charter(
    repo_root: impl AsRef<Path>,
    input: &CharterStructuredInput,
) -> Result<AuthorCharterResult, AuthorCharterRefusal> {
    let repo_root = repo_root.as_ref();
    preflight_author_charter_from_input(repo_root, input)?;
    with_charter_authoring_lock(repo_root, || {
        preflight_author_charter_from_input(repo_root, input)?;
        let markdown = compiler_owned_charter_markdown(input)?;
        write_canonical_charter_markdown(repo_root, &markdown)
    })
}

pub fn preflight_author_charter(repo_root: impl AsRef<Path>) -> Result<(), AuthorCharterRefusal> {
    preflight_author_charter_shell(repo_root.as_ref())
}
