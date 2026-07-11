use super::{
    environment_inventory_core::{
        parse_environment_inventory_structured_input_yaml as parse_environment_inventory_structured_input_yaml_core,
        validate_environment_inventory_markdown as validate_environment_inventory_markdown_core,
        validate_environment_inventory_structured_input as validate_environment_inventory_structured_input_core,
        EnvironmentInventoryCoreError, EnvironmentInventoryCoreErrorKind,
    },
    environment_inventory_shell::{
        preflight_author_environment_inventory as preflight_author_environment_inventory_shell,
        render_environment_inventory_markdown as render_environment_inventory_markdown_shell,
        with_environment_inventory_authoring_lock, write_canonical_environment_inventory_markdown,
    },
};
use crate::layout::CANONICAL_ENVIRONMENT_INVENTORY_RELATIVE_PATH;
use std::path::Path;

pub use super::environment_inventory_core::{
    EnvironmentCiInput, EnvironmentExternalServiceInput, EnvironmentInventoryStructuredInput,
    EnvironmentKnownUnknownInput, EnvironmentLocalDevelopmentInput, EnvironmentProductionInput,
    EnvironmentRuntimeAssumptionsInput, EnvironmentSecretHandlingInput, EnvironmentToolingInput,
    EnvironmentUpdateContractInput, EnvironmentVariableInput,
};

pub const CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH: &str =
    CANONICAL_ENVIRONMENT_INVENTORY_RELATIVE_PATH;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorEnvironmentInventoryRefusalKind {
    MissingSystemRoot,
    InvalidSystemRoot,
    MalformedStructuredInput,
    IncompleteStructuredInput,
    MissingRequiredCharter,
    InvalidUpstreamCanonicalTruth,
    ExistingCanonicalTruth,
    MutationRefused,
    SynthesisFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorEnvironmentInventoryRefusal {
    pub kind: AuthorEnvironmentInventoryRefusalKind,
    pub summary: String,
    pub broken_subject: String,
    pub next_safe_action: String,
}

impl std::fmt::Display for AuthorEnvironmentInventoryRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary)
    }
}

impl std::error::Error for AuthorEnvironmentInventoryRefusal {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorEnvironmentInventoryResult {
    pub canonical_repo_relative_path: &'static str,
    pub bytes_written: usize,
}

pub fn parse_environment_inventory_structured_input_yaml(
    yaml: &str,
) -> Result<EnvironmentInventoryStructuredInput, AuthorEnvironmentInventoryRefusal> {
    parse_environment_inventory_structured_input_yaml_core(yaml)
        .map_err(map_environment_inventory_core_error)
}

pub fn validate_environment_inventory_structured_input(
    input: &EnvironmentInventoryStructuredInput,
) -> Result<(), AuthorEnvironmentInventoryRefusal> {
    validate_environment_inventory_structured_input_core(input)
        .map_err(map_environment_inventory_core_error)
}

pub fn render_environment_inventory_markdown(
    input: &EnvironmentInventoryStructuredInput,
) -> Result<String, AuthorEnvironmentInventoryRefusal> {
    render_environment_inventory_markdown_shell(input)
}

pub fn preflight_author_environment_inventory(
    repo_root: impl AsRef<Path>,
) -> Result<(), AuthorEnvironmentInventoryRefusal> {
    preflight_author_environment_inventory_shell(repo_root.as_ref()).map(|_| ())
}

pub fn preflight_author_environment_inventory_from_input(
    repo_root: impl AsRef<Path>,
    input: &EnvironmentInventoryStructuredInput,
) -> Result<(), AuthorEnvironmentInventoryRefusal> {
    validate_environment_inventory_structured_input(input)?;
    let has_project_context = preflight_author_environment_inventory_shell(repo_root.as_ref())?;
    validate_project_context_reference(input, has_project_context)
}

fn validate_project_context_reference(
    input: &EnvironmentInventoryStructuredInput,
    has_project_context: bool,
) -> Result<(), AuthorEnvironmentInventoryRefusal> {
    if input.project_context_ref.is_some() == has_project_context {
        return Ok(());
    }

    let summary = if has_project_context {
        "structured environment-inventory input must set `project_context_ref` to `.handbook/project_context/PROJECT_CONTEXT.md` because valid canonical project context truth exists"
    } else {
        "structured environment-inventory input must set `project_context_ref` to null because valid canonical project context truth does not exist"
    };
    Err(AuthorEnvironmentInventoryRefusal {
        kind: AuthorEnvironmentInventoryRefusalKind::IncompleteStructuredInput,
        summary: summary.to_string(),
        broken_subject: "structured environment-inventory input project_context_ref".to_string(),
        next_safe_action:
            "align `project_context_ref` with canonical project-context truth and retry `handbook author environment-inventory --from-inputs <path|->`"
                .to_string(),
    })
}

pub fn author_environment_inventory_from_input(
    repo_root: impl AsRef<Path>,
    input: &EnvironmentInventoryStructuredInput,
) -> Result<AuthorEnvironmentInventoryResult, AuthorEnvironmentInventoryRefusal> {
    let repo_root = repo_root.as_ref();
    preflight_author_environment_inventory_from_input(repo_root, input)?;
    with_environment_inventory_authoring_lock(repo_root, || {
        preflight_author_environment_inventory_from_input(repo_root, input)?;
        let markdown = render_environment_inventory_markdown(input)?;
        write_canonical_environment_inventory_markdown(repo_root, &markdown)
    })
}

pub fn validate_environment_inventory_markdown(
    markdown: &str,
) -> Result<(), AuthorEnvironmentInventoryRefusal> {
    validate_environment_inventory_markdown_core(markdown).map_err(|summary| {
        AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::SynthesisFailed,
            summary,
            broken_subject: "environment inventory markdown".to_string(),
            next_safe_action:
                "repair the environment inventory markdown and rerun deterministic validation"
                    .to_string(),
        }
    })
}

pub(super) fn map_environment_inventory_core_error(
    error: EnvironmentInventoryCoreError,
) -> AuthorEnvironmentInventoryRefusal {
    match error.kind {
        EnvironmentInventoryCoreErrorKind::MalformedStructuredInput => {
            AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::MalformedStructuredInput,
                summary: error.summary,
                broken_subject: "structured environment-inventory input".to_string(),
                next_safe_action:
                    "repair the structured environment-inventory input and retry `handbook author environment-inventory --from-inputs <path|->`"
                        .to_string(),
            }
        }
        EnvironmentInventoryCoreErrorKind::IncompleteStructuredInput => {
            AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::IncompleteStructuredInput,
                summary: error.summary,
                broken_subject: "structured environment-inventory input".to_string(),
                next_safe_action:
                    "repair the structured environment-inventory input and retry `handbook author environment-inventory --from-inputs <path|->`"
                        .to_string(),
            }
        }
        EnvironmentInventoryCoreErrorKind::DeterministicRenderFailed => {
            AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::SynthesisFailed,
                summary: error.summary,
                broken_subject: "final environment inventory render".to_string(),
                next_safe_action:
                    "repair the structured environment-inventory input or deterministic render path and retry `handbook author environment-inventory --from-inputs <path|->`"
                        .to_string(),
            }
        }
    }
}
