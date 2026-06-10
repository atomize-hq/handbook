use super::{
    project_context_core::{
        parse_project_context_structured_input_yaml as parse_project_context_structured_input_yaml_core,
        validate_project_context_structured_input as validate_project_context_structured_input_core,
        ProjectContextCoreError, ProjectContextCoreErrorKind,
    },
    project_context_shell::{
        preflight_author_project_context as preflight_author_project_context_shell,
        project_context_inputs_required_refusal,
        render_project_context_markdown as render_project_context_markdown_shell,
        with_project_context_authoring_lock, write_canonical_project_context_markdown,
    },
};
use crate::layout::CANONICAL_PROJECT_CONTEXT_RELATIVE_PATH;
use std::path::Path;

pub use super::project_context_core::{
    validate_project_context_markdown, ProjectContextClassificationImplicationsInput,
    ProjectContextConstraintsInput, ProjectContextDataRealityInput,
    ProjectContextEnvironmentsAndDeliveryInput, ProjectContextIntegrationInput,
    ProjectContextKnownUnknownInput, ProjectContextOperationalRealityInput,
    ProjectContextRepoCodebaseRealityInput, ProjectContextStructuredInput,
    ProjectContextSummaryInput, ProjectContextSystemBoundariesInput, ProjectContextValidationError,
};

pub const CANONICAL_PROJECT_CONTEXT_REPO_PATH: &str = CANONICAL_PROJECT_CONTEXT_RELATIVE_PATH;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorProjectContextRefusalKind {
    MissingSystemRoot,
    InvalidSystemRoot,
    MalformedStructuredInput,
    IncompleteStructuredInput,
    ExistingCanonicalTruth,
    MutationRefused,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorProjectContextRefusal {
    pub kind: AuthorProjectContextRefusalKind,
    pub summary: String,
    pub broken_subject: String,
    pub next_safe_action: String,
}

impl std::fmt::Display for AuthorProjectContextRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary)
    }
}

impl std::error::Error for AuthorProjectContextRefusal {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorProjectContextResult {
    pub canonical_repo_relative_path: &'static str,
    pub bytes_written: usize,
}

pub fn parse_project_context_structured_input_yaml(
    yaml: &str,
) -> Result<ProjectContextStructuredInput, AuthorProjectContextRefusal> {
    parse_project_context_structured_input_yaml_core(yaml).map_err(map_project_context_core_error)
}

pub fn validate_project_context_structured_input(
    input: &ProjectContextStructuredInput,
) -> Result<(), AuthorProjectContextRefusal> {
    validate_project_context_structured_input_core(input).map_err(map_project_context_core_error)
}

pub fn render_project_context_markdown(
    input: &ProjectContextStructuredInput,
) -> Result<String, AuthorProjectContextRefusal> {
    render_project_context_markdown_shell(input)
}

pub fn preflight_author_project_context(
    repo_root: impl AsRef<Path>,
) -> Result<(), AuthorProjectContextRefusal> {
    preflight_author_project_context_shell(repo_root.as_ref())
}

pub fn author_project_context(
    repo_root: impl AsRef<Path>,
) -> Result<AuthorProjectContextResult, AuthorProjectContextRefusal> {
    let repo_root = repo_root.as_ref();
    preflight_author_project_context(repo_root)?;
    Err(project_context_inputs_required_refusal())
}

pub fn author_project_context_from_input(
    repo_root: impl AsRef<Path>,
    input: &ProjectContextStructuredInput,
) -> Result<AuthorProjectContextResult, AuthorProjectContextRefusal> {
    let repo_root = repo_root.as_ref();
    validate_project_context_structured_input(input)?;
    preflight_author_project_context(repo_root)?;
    with_project_context_authoring_lock(repo_root, || {
        validate_project_context_structured_input(input)?;
        preflight_author_project_context(repo_root)?;
        let markdown = render_project_context_markdown_shell(input)?;
        write_canonical_project_context_markdown(repo_root, &markdown)
    })
}

pub(super) fn map_project_context_core_error(
    err: ProjectContextCoreError,
) -> AuthorProjectContextRefusal {
    match err.kind {
        ProjectContextCoreErrorKind::MalformedStructuredInput => AuthorProjectContextRefusal {
            kind: AuthorProjectContextRefusalKind::MalformedStructuredInput,
            summary: err.summary,
            broken_subject: "structured project-context input".to_string(),
            next_safe_action:
                "repair the structured project-context input and retry `handbook author project-context --from-inputs <path|->`"
                    .to_string(),
        },
        ProjectContextCoreErrorKind::IncompleteStructuredInput
        | ProjectContextCoreErrorKind::DeterministicRenderFailed => AuthorProjectContextRefusal {
            kind: AuthorProjectContextRefusalKind::IncompleteStructuredInput,
            summary: err.summary,
            broken_subject: "structured project-context input".to_string(),
            next_safe_action:
                "repair the structured project-context input and retry `handbook author project-context --from-inputs <path|->`"
                    .to_string(),
        },
    }
}
