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
        EnvironmentInventoryUpstreamSelection,
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
    let selected = preflight_author_environment_inventory_shell(repo_root.as_ref())?;
    validate_charter_reference(input, &selected)?;
    validate_project_context_reference(input, &selected.project_context_path)
}

fn validate_charter_reference(
    input: &EnvironmentInventoryStructuredInput,
    selected: &EnvironmentInventoryUpstreamSelection,
) -> Result<(), AuthorEnvironmentInventoryRefusal> {
    let authority_carrier_is_complete = selected.charter_source_fingerprint.starts_with("sha256:")
        && !selected.charter_promotion_ref.is_empty()
        && !selected.charter_lifecycle_transition_ref.is_empty();
    if input.charter_ref == selected.charter_path && authority_carrier_is_complete {
        return Ok(());
    }

    Err(AuthorEnvironmentInventoryRefusal {
        kind: AuthorEnvironmentInventoryRefusalKind::IncompleteStructuredInput,
        summary: format!(
            "structured environment-inventory input must set `charter_ref` exactly to `{}` and the selected Charter must retain committed source-fingerprint authority",
            selected.charter_path
        ),
        broken_subject: "structured environment-inventory input charter_ref".to_string(),
        next_safe_action:
            "set `charter_ref` to the selected canonical Charter YAML path, ensure it is promoted, and retry `handbook author environment-inventory --from-inputs <path|->`"
                .to_string(),
    })
}

fn validate_project_context_reference(
    input: &EnvironmentInventoryStructuredInput,
    selected_project_context_path: &str,
) -> Result<(), AuthorEnvironmentInventoryRefusal> {
    if input.project_context_ref.as_deref() == Some(selected_project_context_path) {
        return Ok(());
    }

    Err(AuthorEnvironmentInventoryRefusal {
        kind: AuthorEnvironmentInventoryRefusalKind::IncompleteStructuredInput,
        summary: format!(
            "structured environment-inventory input must set `project_context_ref` exactly to `{selected_project_context_path}`"
        ),
        broken_subject: "structured environment-inventory input project_context_ref".to_string(),
        next_safe_action:
            "set `project_context_ref` to the selected canonical Project Context YAML path and retry `handbook author environment-inventory --from-inputs <path|->`"
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
