use crate::repo_file_access::{CompilerWorkspace, RepoRelativeFileAccessError};
use std::borrow::Cow;
use std::fmt;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateLibraryRequest {
    CharterAuthoring,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateLibraryResolveRequest {
    selection: TemplateLibraryRequest,
    override_request: Option<TemplateLibraryOverrideRequest>,
}

impl TemplateLibraryResolveRequest {
    pub fn new(selection: TemplateLibraryRequest) -> Self {
        Self {
            selection,
            override_request: None,
        }
    }

    pub fn with_override(mut self, override_request: TemplateLibraryOverrideRequest) -> Self {
        self.override_request = Some(override_request);
        self
    }

    pub const fn selection(&self) -> TemplateLibraryRequest {
        self.selection
    }

    pub fn override_request(&self) -> Option<&TemplateLibraryOverrideRequest> {
        self.override_request.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateLibraryOverrideRequest {
    Charter(CharterTemplateLibraryOverride),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CharterTemplateLibraryOverride {
    authoring_method_repo_relative_path: Option<String>,
    synthesize_directive_repo_relative_path: Option<String>,
    template_repo_relative_path: Option<String>,
}

impl CharterTemplateLibraryOverride {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_authoring_method_repo_relative_path(mut self, path: impl Into<String>) -> Self {
        self.authoring_method_repo_relative_path = Some(path.into());
        self
    }

    pub fn with_synthesize_directive_repo_relative_path(mut self, path: impl Into<String>) -> Self {
        self.synthesize_directive_repo_relative_path = Some(path.into());
        self
    }

    pub fn with_template_repo_relative_path(mut self, path: impl Into<String>) -> Self {
        self.template_repo_relative_path = Some(path.into());
        self
    }

    pub fn authoring_method_repo_relative_path(&self) -> Option<&str> {
        self.authoring_method_repo_relative_path.as_deref()
    }

    pub fn synthesize_directive_repo_relative_path(&self) -> Option<&str> {
        self.synthesize_directive_repo_relative_path.as_deref()
    }

    pub fn template_repo_relative_path(&self) -> Option<&str> {
        self.template_repo_relative_path.as_deref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateLibraryAsset {
    CharterAuthoringMethod,
    CharterSynthesizeDirective,
    CharterTemplate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateLibraryDocument {
    asset: TemplateLibraryAsset,
    repo_relative_path: String,
    contents: Cow<'static, str>,
}

impl TemplateLibraryDocument {
    fn shipped(descriptor: TemplateLibraryDocumentDescriptor) -> Self {
        Self {
            asset: descriptor.asset,
            repo_relative_path: descriptor.repo_relative_path.to_string(),
            contents: Cow::Borrowed(descriptor.contents),
        }
    }

    fn override_document(
        asset: TemplateLibraryAsset,
        repo_relative_path: String,
        contents: String,
    ) -> Self {
        Self {
            asset,
            repo_relative_path,
            contents: Cow::Owned(contents),
        }
    }

    pub fn asset(&self) -> TemplateLibraryAsset {
        self.asset
    }

    pub fn repo_relative_path(&self) -> &str {
        self.repo_relative_path.as_str()
    }

    pub fn contents(&self) -> &str {
        self.contents.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharterTemplateLibrarySelection {
    authoring_method: TemplateLibraryDocument,
    synthesize_directive: TemplateLibraryDocument,
    template: TemplateLibraryDocument,
}

impl CharterTemplateLibrarySelection {
    pub fn authoring_method(&self) -> &TemplateLibraryDocument {
        &self.authoring_method
    }

    pub fn synthesize_directive(&self) -> &TemplateLibraryDocument {
        &self.synthesize_directive
    }

    pub fn template(&self) -> &TemplateLibraryDocument {
        &self.template
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateLibrarySelection {
    Charter(CharterTemplateLibrarySelection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateLibraryResolveErrorKind {
    OverrideFamilyMismatch,
    InvalidOverridePath,
    MissingOverride,
    AssetKindMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateLibraryResolveError {
    pub kind: TemplateLibraryResolveErrorKind,
    pub summary: String,
    pub asset: Option<TemplateLibraryAsset>,
    pub repo_relative_path: Option<String>,
}

impl fmt::Display for TemplateLibraryResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary)
    }
}

impl std::error::Error for TemplateLibraryResolveError {}

pub fn resolve_template_library(
    repo_root: impl AsRef<Path>,
    request: &TemplateLibraryResolveRequest,
) -> Result<TemplateLibrarySelection, TemplateLibraryResolveError> {
    let workspace = CompilerWorkspace::new(repo_root.as_ref());
    match (request.selection(), request.override_request()) {
        (TemplateLibraryRequest::CharterAuthoring, None) => Ok(resolve_shipped_template_library(
            TemplateLibraryRequest::CharterAuthoring,
        )),
        (
            TemplateLibraryRequest::CharterAuthoring,
            Some(TemplateLibraryOverrideRequest::Charter(overrides)),
        ) => Ok(TemplateLibrarySelection::Charter(
            resolve_charter_selection(&workspace, overrides)?,
        )),
    }
}

pub fn resolve_shipped_template_library(
    request: TemplateLibraryRequest,
) -> TemplateLibrarySelection {
    match request {
        TemplateLibraryRequest::CharterAuthoring => {
            TemplateLibrarySelection::Charter(CharterTemplateLibrarySelection {
                authoring_method: TemplateLibraryDocument::shipped(CHARTER_AUTHORING_METHOD),
                synthesize_directive: TemplateLibraryDocument::shipped(
                    CHARTER_SYNTHESIZE_DIRECTIVE,
                ),
                template: TemplateLibraryDocument::shipped(CHARTER_TEMPLATE),
            })
        }
    }
}

fn resolve_charter_selection(
    workspace: &CompilerWorkspace<'_>,
    overrides: &CharterTemplateLibraryOverride,
) -> Result<CharterTemplateLibrarySelection, TemplateLibraryResolveError> {
    let TemplateLibrarySelection::Charter(mut selection) =
        resolve_shipped_template_library(TemplateLibraryRequest::CharterAuthoring);

    if let Some(path) = overrides.authoring_method_repo_relative_path() {
        selection.authoring_method = load_override_document(
            workspace,
            TemplateLibraryAsset::CharterAuthoringMethod,
            path,
        )?;
    }
    if let Some(path) = overrides.synthesize_directive_repo_relative_path() {
        selection.synthesize_directive = load_override_document(
            workspace,
            TemplateLibraryAsset::CharterSynthesizeDirective,
            path,
        )?;
    }
    if let Some(path) = overrides.template_repo_relative_path() {
        selection.template =
            load_override_document(workspace, TemplateLibraryAsset::CharterTemplate, path)?;
    }

    Ok(selection)
}

fn load_override_document(
    workspace: &CompilerWorkspace<'_>,
    asset: TemplateLibraryAsset,
    repo_relative_path: &str,
) -> Result<TemplateLibraryDocument, TemplateLibraryResolveError> {
    let normalized = workspace
        .normalize_repo_relative(repo_relative_path)
        .map_err(|reason| invalid_override_path(asset, repo_relative_path, reason))?;
    let normalized_path = normalized.as_str().to_string();
    ensure_override_matches_asset_kind(asset, &normalized_path)?;
    let contents = workspace
        .read_string(&normalized)
        .map_err(|err| classify_override_read_error(asset, &normalized_path, err))?;
    Ok(TemplateLibraryDocument::override_document(
        asset,
        normalized_path,
        contents,
    ))
}

fn ensure_override_matches_asset_kind(
    asset: TemplateLibraryAsset,
    repo_relative_path: &str,
) -> Result<(), TemplateLibraryResolveError> {
    let rule = override_rule(asset);
    let in_family = rule
        .allowed_prefixes
        .iter()
        .any(|prefix| repo_relative_path.starts_with(prefix));
    let has_expected_extension = repo_relative_path.ends_with(rule.required_extension);
    if in_family && has_expected_extension {
        return Ok(());
    }

    let expected_prefixes = rule.allowed_prefixes.join(" or ");
    Err(TemplateLibraryResolveError {
        kind: TemplateLibraryResolveErrorKind::AssetKindMismatch,
        summary: format!(
            "override for {} must stay under {} and end with `{}`",
            asset_label(asset),
            expected_prefixes,
            rule.required_extension
        ),
        asset: Some(asset),
        repo_relative_path: Some(repo_relative_path.to_string()),
    })
}

fn classify_override_read_error(
    asset: TemplateLibraryAsset,
    repo_relative_path: &str,
    err: RepoRelativeFileAccessError,
) -> TemplateLibraryResolveError {
    match err {
        RepoRelativeFileAccessError::Missing(path) => TemplateLibraryResolveError {
            kind: TemplateLibraryResolveErrorKind::MissingOverride,
            summary: format!(
                "override for {} is missing at `{}` ({})",
                asset_label(asset),
                repo_relative_path,
                path.display()
            ),
            asset: Some(asset),
            repo_relative_path: Some(repo_relative_path.to_string()),
        },
        RepoRelativeFileAccessError::SymlinkNotAllowed(path) => TemplateLibraryResolveError {
            kind: TemplateLibraryResolveErrorKind::InvalidOverridePath,
            summary: format!(
                "override for {} must resolve to a regular repo-owned file; symlinks are not allowed ({})",
                asset_label(asset),
                path.display()
            ),
            asset: Some(asset),
            repo_relative_path: Some(repo_relative_path.to_string()),
        },
        RepoRelativeFileAccessError::NotRegularFile(path) => TemplateLibraryResolveError {
            kind: TemplateLibraryResolveErrorKind::InvalidOverridePath,
            summary: format!(
                "override for {} must resolve to a regular repo-owned file ({})",
                asset_label(asset),
                path.display()
            ),
            asset: Some(asset),
            repo_relative_path: Some(repo_relative_path.to_string()),
        },
        RepoRelativeFileAccessError::ReadFailure { path, source } => TemplateLibraryResolveError {
            kind: TemplateLibraryResolveErrorKind::InvalidOverridePath,
            summary: format!(
                "failed to read override for {} at {}: {}",
                asset_label(asset),
                path.display(),
                source
            ),
            asset: Some(asset),
            repo_relative_path: Some(repo_relative_path.to_string()),
        },
    }
}

fn invalid_override_path(
    asset: TemplateLibraryAsset,
    repo_relative_path: &str,
    reason: impl Into<String>,
) -> TemplateLibraryResolveError {
    TemplateLibraryResolveError {
        kind: TemplateLibraryResolveErrorKind::InvalidOverridePath,
        summary: format!(
            "override for {} must be a bounded repo-relative path: {}",
            asset_label(asset),
            reason.into()
        ),
        asset: Some(asset),
        repo_relative_path: Some(repo_relative_path.to_string()),
    }
}

fn asset_label(asset: TemplateLibraryAsset) -> &'static str {
    match asset {
        TemplateLibraryAsset::CharterAuthoringMethod => "charter authoring method",
        TemplateLibraryAsset::CharterSynthesizeDirective => "charter synthesize directive",
        TemplateLibraryAsset::CharterTemplate => "charter template",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TemplateLibraryOverrideRule {
    allowed_prefixes: &'static [&'static str],
    required_extension: &'static str,
}

fn override_rule(asset: TemplateLibraryAsset) -> TemplateLibraryOverrideRule {
    match asset {
        TemplateLibraryAsset::CharterAuthoringMethod => TemplateLibraryOverrideRule {
            allowed_prefixes: &["core/library/authoring/"],
            required_extension: ".md",
        },
        TemplateLibraryAsset::CharterSynthesizeDirective => TemplateLibraryOverrideRule {
            allowed_prefixes: &["core/library/charter/"],
            required_extension: ".md",
        },
        TemplateLibraryAsset::CharterTemplate => TemplateLibraryOverrideRule {
            allowed_prefixes: &["core/library/charter/"],
            required_extension: ".tmpl",
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TemplateLibraryDocumentDescriptor {
    asset: TemplateLibraryAsset,
    repo_relative_path: &'static str,
    contents: &'static str,
}

const CHARTER_AUTHORING_METHOD: TemplateLibraryDocumentDescriptor =
    TemplateLibraryDocumentDescriptor {
        asset: TemplateLibraryAsset::CharterAuthoringMethod,
        repo_relative_path: "core/library/authoring/charter_authoring_method.md",
        contents: include_str!("../../../core/library/authoring/charter_authoring_method.md"),
    };

const CHARTER_SYNTHESIZE_DIRECTIVE: TemplateLibraryDocumentDescriptor =
    TemplateLibraryDocumentDescriptor {
        asset: TemplateLibraryAsset::CharterSynthesizeDirective,
        repo_relative_path: "core/library/charter/charter_synthesize_directive.md",
        contents: include_str!("../../../core/library/charter/charter_synthesize_directive.md"),
    };

const CHARTER_TEMPLATE: TemplateLibraryDocumentDescriptor = TemplateLibraryDocumentDescriptor {
    asset: TemplateLibraryAsset::CharterTemplate,
    repo_relative_path: "core/library/charter/charter.md.tmpl",
    contents: include_str!("../../../core/library/charter/charter.md.tmpl"),
};
